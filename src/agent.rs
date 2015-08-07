use std::thread;
use std::any::Any;
use std::sync::{Arc, Mutex, Barrier, MutexGuard, LockResult};

use libc::{c_int, c_uint, c_void};

#[cfg(test)]
use env_logger;
#[cfg(test)]
use std::sync::mpsc::channel;

use condition_variable::{ConditionVariable, Notify};

use bindings_agent as ffi;
use api_agent as api;

use api_gobject::{GMainLoop, GMainContext};
use gobject::GObjectTrait;

use api_agent::NiceComponentState;

use gobject::GCallbackHandle;


pub enum ControllingMode {
	Server,
	Client,
}

impl ControllingMode {
	pub fn to_bool(&self) -> bool {
		match *self {
			ControllingMode::Server => false,
			ControllingMode::Client => true,
		}
	}
}

pub struct Agent {
	agent:     Arc<Mutex<api::Agent>>,
	main_loop: GMainLoop,
	main_ctx:  GMainContext,
}

impl Drop for Agent {
	fn drop(&mut self) {
		self.main_loop.quit();
	}
}

impl Agent {
	pub fn new(mode: ControllingMode) -> Agent {
		let main_loop = GMainLoop::new();
		
		let ctx = main_loop.get_context();
		let compat = ffi::NICE_COMPATIBILITY_RFC5245;

		let agent = Agent {
			agent:     Arc::new(Mutex::new(api::Agent::new(&ctx, compat))),
			main_loop: main_loop.clone(),
			main_ctx:  ctx,
		};
		agent.set_controlling_mode(mode);

		thread::spawn(move || {
			main_loop.run();
		});

		agent
	}

	pub fn set_software(&self, name: &str) {
		let lock = self.agent.lock().unwrap();
		lock.set_software(name);
	}

	pub fn get_context(&self) -> &GMainContext {
		&self.main_ctx
	}

	pub fn add_stream<'a, F:Fn(&[u8])>(
				&'a self, name: &str,
				n_components:   usize,
				callback:       F)
		-> Option<Stream<'a,F>>
	{
		let stream_id = {
			let lock = self.agent.lock().unwrap();

			if let Some(id) = lock.add_stream(n_components) {
				id
			} else {
				return None;
			}
		};

		let stream = Stream::new(&self, stream_id, callback);

		if let Some(s) = stream {
			s.set_name(name);

			if s.gather_candidates() {
				return Some(s);
			}
		}

		None
	}

	pub fn watch_state<'a>(&'a self, stream_id: c_uint, component_id: c_uint)
		-> (Arc<ConditionVariable<NiceComponentState>>,
			GCallbackHandle<'a, api::Agent, Agent>)
	{
		let init_state = NiceComponentState::NICE_COMPONENT_STATE_DISCONNECTED;
		let state = Arc::new(ConditionVariable::new(init_state));

		let state1 = state.clone();
		let state_cb = move |s, c, new_state| {
			if stream_id != s || component_id != c {
				return false; // TODO ???
			}

			debug!("new state: {:?}", new_state);
			state1.set(new_state, Notify::All);

			false // TODO: correct?
		};

		let cb_handle = self.on_component_state_changed(state_cb);

		(state, cb_handle)
	}

	fn set_controlling_mode(&self, mode: ControllingMode) {
		//let lock = self.agent.lock().unwrap();
		//lock.set_property("controlling-mode", mode.to_bool() as *mut c_void);
		let mode = mode.to_bool() as usize as *mut c_void;
		GObjectTrait::set_property(self, "controlling-mode", mode);
	}

	pub fn generate_local_sdp(&self) -> Option<String> {
		let lock = self.agent.lock().unwrap();
		lock.generate_local_sdp()
	}

	/// Stream::set_name() is required before calling this function
	pub fn parse_remote_sdp(&self, sdp: &str) -> Option<usize> {
		let lock = self.agent.lock().unwrap();
		lock.parse_remote_sdp(sdp)
	}

	#[must_use]
	fn on_candidate_gathering_done<'a,'b, F:Any>(&'a self, cb: F)
		-> GCallbackHandle<'a,api::Agent, Self>
		where F: Any + Fn(c_uint) -> bool
	{
		#[allow(unused_variables)]
		extern fn wrapper<G>(_agent:    *mut ffi::_NiceAgent,
		                     stream_id: c_uint,
		                     user_data: *mut c_void)
			-> c_int
				where G: Fn(c_uint) -> bool
		{
			let closure = user_data as *mut G;
			unsafe {
				let res = (*closure)(stream_id);
				return res as c_int;
			}
		}

		let func_ptr: extern fn() = unsafe {
			::std::mem::transmute(wrapper::<F>)
		};

		GObjectTrait::on_signal(self, "candidate-gathering-done", cb, func_ptr)
	}

	#[must_use]
	fn on_component_state_changed<'a, F:Any>(&'a self, cb: F)
		-> GCallbackHandle<'a, api::Agent, Self>
		where F: Any + Fn(c_uint, c_uint, NiceComponentState) -> bool
	{
		extern fn wrapper<G>(_/*agent*/:   *mut ffi::_NiceAgent,
		                     stream_id:    c_uint,
		                     component_id: c_uint,
		                     state:        c_uint,
		                     user_data:    *mut c_void)
			-> c_int
			where G: Fn(c_uint, c_uint, NiceComponentState) -> bool
		{
			let closure = user_data as *mut G;

			unsafe {
				let state = ::std::mem::transmute(state);
				let res = (*closure)(stream_id, component_id, state);
				return res as c_int;
			}
		}

		let func_ptr: extern fn() = unsafe {
			::std::mem::transmute(wrapper::<F>)
		};

		GObjectTrait::on_signal(self, "component-state-changed", cb, func_ptr)
	}

	#[must_use]
	fn gather_candidates(&self, stream_id: c_uint) -> bool {
		let lock = self.agent.lock().unwrap();
		lock.gather_candidates(stream_id)
	}
}

impl GObjectTrait<api::Agent> for Agent {
	fn as_raw(&self) -> LockResult<MutexGuard<api::Agent>> {
		self.agent.lock()
	}
}

// TODO: remove lifetime
pub struct Stream<'a,F:Fn(&[u8])> {
	agent:          &'a Agent,
	stream_id:      c_uint,
	state:          Arc<ConditionVariable<NiceComponentState>>,
	rx_callback:    Box<F>,
	state_callback: GCallbackHandle<'a, api::Agent, Agent>
}

impl<'a,F:Fn(&[u8])> Stream<'a,F> {
	fn new(agent: &'a Agent, stream_id: c_uint, rx_callback: F)
		-> Option<Stream<'a, F>>
	{
		let component_id = 1;

		let (state, state_cb) = agent.watch_state(stream_id, component_id);

		let mut s = Stream {
			agent:          agent,
			stream_id:      stream_id,
			state:          state,
			rx_callback:    Box::new(rx_callback),
			state_callback: state_cb,
		};

		if s.attach_recv(component_id).is_ok() {
			Some(s)
		} else {
			None
		}
	}

	// TODO use return value
	fn gather_candidates(&self) -> bool {
		let barrier1 = Arc::new(Barrier::new(2));
		let barrier2 = barrier1.clone();

		let expected_stream_id = self.stream_id;

		let on_done_cb = move |actual_stream_id:c_uint| {
			assert_eq!(actual_stream_id, expected_stream_id);
			barrier1.wait();

			true // TODO: is this correct, or is it 'false/true'?
		};

		let cb  = self.agent.on_candidate_gathering_done(on_done_cb);
		let res = self.agent.gather_candidates(self.stream_id);

		/*
		 * Attention: We must wait for the callback to be called, then we can
		 * drop the callback (and thus its move'd variables).
		 */
		barrier2.wait();
		drop(cb);

		res
	}

	pub fn get_local_credentials(&self)
		-> Option<(String, String)>
	{
		let lock = self.agent.as_raw().unwrap();
		lock.get_local_credentials(self.stream_id)
	}

	pub fn set_remote_credentials(&self, ufrag: &str, pwd: &str) -> bool {
		let lock = self.agent.as_raw().unwrap();
		lock.set_remote_credentials(self.stream_id, ufrag, pwd)
	}

	pub fn send(&self, component_id: c_uint, buf: &[u8]) -> Option<usize> {
		let lock = self.agent.as_raw().unwrap();
		lock.send(self.stream_id, component_id, buf)
	}

	fn set_name(&self, name: &str) -> bool {
		let lock = self.agent.as_raw().unwrap();
		lock.set_stream_name(self.stream_id, name)
	}

	pub fn get_name(&self) -> Option<String> {
		let lock = self.agent.as_raw().unwrap();
		lock.get_stream_name(self.stream_id)
	}

	pub fn attach_recv(&mut self,
	                   component_id: c_uint)
		-> Result<(), ()>
	{
		let cb   = &mut self.rx_callback;
		let ctx  = self.agent.get_context();
		let lock = self.agent.as_raw().unwrap();

		if !lock.attach_recv(self.stream_id, component_id, ctx, cb) {
			Err(())
		} else {
			Ok(())
		}
	}

	pub fn get_state(&self) -> Arc<ConditionVariable<NiceComponentState>> {
		self.state.clone()
	}
}

impl<'a,F:Fn(&[u8])> Drop for Stream<'a,F> {
	fn drop(&mut self) {
		let ctx  = self.agent.get_context();
		let lock = self.agent.as_raw().unwrap();

		lock.unattach_recv(self.stream_id, 1, ctx);
		lock.remove_stream(self.stream_id);
	}
}

/// Note: you must be connected to a LAN/Internet to pass this test
#[test]
fn test() {
	env_logger::init().unwrap();

	let alice = Agent::new(ControllingMode::Server);
	let bob   = Agent::new(ControllingMode::Client);

	let (alice_tx, alice_rx) = channel();
	let (bob_tx, bob_rx)     = channel();
	let alice_cb = move |buf:&[u8]| alice_tx.send(buf.to_vec()).unwrap();
	let bob_cb   = move |buf:&[u8]| bob_tx.send(buf.to_vec()).unwrap();
	
	let a = alice.add_stream("test", 1, alice_cb).unwrap();
	let b = bob.add_stream("test", 1, bob_cb).unwrap();

	let cred_alice = alice.generate_local_sdp().unwrap();
	let cred_bob   = bob.generate_local_sdp().unwrap();

	let count_a = alice.parse_remote_sdp(&cred_bob[..]);
	let count_b = bob.parse_remote_sdp(&cred_alice[..]);
	assert!(count_a.unwrap() > 0);
	assert!(count_b.unwrap() > 0);

	let state_a = a.get_state();
	let state_b = b.get_state();

	state_a.wait_for(NiceComponentState::NICE_COMPONENT_STATE_READY).unwrap();
	state_b.wait_for(NiceComponentState::NICE_COMPONENT_STATE_READY).unwrap();

	assert_eq!(a.send(1, &[1,2,3]), Some(3));
	assert_eq!(b.send(1, &[6,7,8,9]), Some(4));
	assert_eq!(a.send(1, &[4,5,6]), Some(3));

	assert_eq!(alice_rx.recv().unwrap(), [6,7,8,9]);
	assert_eq!(bob_rx.recv().unwrap(), [1,2,3]);
	assert_eq!(bob_rx.recv().unwrap(), [4,5,6]);
}
