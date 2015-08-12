use std::thread;
use std::any::Any;
use std::sync::{Arc, Mutex, MutexGuard, LockResult};

use libc::{c_int, c_uint, c_void};

use condition_variable::{ConditionVariable, Notify};

use bindings_agent as ffi;

use api_gobject::{GMainLoop, GMainContext};
use api_agent as api;
use api_agent::NiceComponentState;

use gobject::{GObjectTrait, GCallbackHandle};
use stream::Stream;

#[derive(Debug,PartialEq,Clone)]
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

	pub fn add_stream<'a, F:Any+Fn(&[u8])>(
				&'a self, name: &str,
				n_components:   usize,
				callback:       F)
		-> Option<Stream<'a>>
	{
		let stream_id = {
			let lock = self.agent.lock().unwrap();

			if let Some(id) = lock.add_stream(n_components) {
				id
			} else {
				return None;
			}
		};

		Stream::new(&self, stream_id, name, callback)
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
			// We can run into a race condition (why?), so let's check
			// if the stream is already set to READY
			if state1.get().unwrap() != NiceComponentState::NICE_COMPONENT_STATE_READY {
				state1.set(new_state, Notify::All);
			}

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
	pub fn on_candidate_gathering_done<'a,'b, F:Any>(&'a self, cb: F)
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
	pub fn gather_candidates(&self, stream_id: c_uint) -> bool {
		let lock = self.agent.lock().unwrap();
		lock.gather_candidates(stream_id)
	}
}

impl GObjectTrait<api::Agent> for Agent {
	fn as_raw(&self) -> LockResult<MutexGuard<api::Agent>> {
		self.agent.lock()
	}
}
