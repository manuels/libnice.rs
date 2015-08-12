use std::sync::Arc;
use std::any::Any;

use libc::c_uint;
use condition_variable::{ConditionVariable, Notify};

use agent::Agent;
use gobject::{GObjectTrait, GCallbackHandle};

use api_agent as api;
pub use api_agent::NiceComponentState;


pub struct Stream<'a> {
	agent:             &'a Agent,
	stream_id:         c_uint,
	state:             Arc<ConditionVariable<NiceComponentState>>,
	gathered:          Arc<ConditionVariable<bool>>,
	rx_callback:       Box<Any>,
	state_callback:    GCallbackHandle<'a, api::Agent, Agent>,
	gathered_callback: GCallbackHandle<'a, api::Agent, Agent>,
}

impl<'a> Stream<'a> {
	pub fn new<F:Any + Fn(&[u8])>(agent: &'a Agent, stream_id: c_uint, name: &str, rx_callback: F)
		-> Option<Stream<'a>>
	{
		let component_id = 1;

		let (state, state_cb) = agent.watch_state(stream_id, component_id);

		let gathered = Arc::new(ConditionVariable::new(false));
		let gathered1 = gathered.clone();
		let gathered_cb = agent.on_candidate_gathering_done(move |s| {
			if s == stream_id {
				gathered1.set(true, Notify::All);
				true // TODO: correct?
			} else {
				false // TODO: correct?
			}
		});

		let mut s = Stream {
			agent:             agent,
			stream_id:         stream_id,
			state:             state,
			gathered:          gathered,
			rx_callback:       Box::new(rx_callback),
			state_callback:    state_cb,
			gathered_callback: gathered_cb,
		};

		s.attach_recv::<F>(component_id)
			.and_then(|_| s.set_name(name))
			.and_then(|_| s.gather_candidates())
			.map(|_| s)
	}

	pub fn gather_candidates(&self) -> Option<()> {
		if !self.agent.gather_candidates(self.stream_id) {
			None
		} else {
			Some(())
		}
	}

	pub fn get_local_credentials(&self) -> Option<(String, String)>
	{
		if self.gathered.wait_for(true).is_err() {
			return None;
		}

		let lock = self.agent.as_raw().unwrap();
		lock.get_local_credentials(self.stream_id)
	}

	pub fn set_remote_credentials(&self, ufrag: &str, pwd: &str) -> Option<()> {
		let lock = self.agent.as_raw().unwrap();
		if !lock.set_remote_credentials(self.stream_id, ufrag, pwd) {
			None
		} else {
			Some(())
		}
	}

	pub fn send(&self, component_id: c_uint, buf: &[u8]) -> Option<usize> {
		let lock = self.agent.as_raw().unwrap();
		lock.send(self.stream_id, component_id, buf)
	}

	fn set_name(&self, name: &str) -> Option<()> {
		let lock = self.agent.as_raw().unwrap();
		if lock.set_stream_name(self.stream_id, name) {
			Some(())
		} else {
			None
		}
	}

	pub fn get_name(&self) -> Option<String> {
		let lock = self.agent.as_raw().unwrap();
		lock.get_stream_name(self.stream_id)
	}

	pub fn attach_recv<F:Any+Fn(&[u8])>(&mut self,
	                   component_id: c_uint)
		-> Option<()>
	{
		let cb: &mut Box<F> = unsafe { ::std::mem::transmute(&mut self.rx_callback) };
		let ctx  = self.agent.get_context();
		let lock = self.agent.as_raw().unwrap();

		if !lock.attach_recv(self.stream_id, component_id, ctx, cb) {
			None
		} else {
			Some(())
		}
	}

	pub fn get_state(&self) -> Arc<ConditionVariable<NiceComponentState>> {
		self.state.clone()
	}

	pub fn get_candiates_gathered(&self) -> Arc<ConditionVariable<bool>> {
		self.gathered.clone()
	}
}

impl<'a> Drop for Stream<'a> {
	fn drop(&mut self) {
		let ctx  = self.agent.get_context();
		let lock = self.agent.as_raw().unwrap();

		lock.unattach_recv(self.stream_id, 1, ctx);
		lock.remove_stream(self.stream_id);
	}
}

/// Note: you must be connected to a LAN/Internet to pass this test
#[cfg(test)]
mod tests {
	use std::sync::mpsc::channel;
	
	use env_logger;
	
	use agent::ControllingMode;
	use api_agent::NiceComponentState;

	#[test]
	fn test() {
		env_logger::init().unwrap();

		let alice = ::Agent::new(ControllingMode::Server);
		let bob   = ::Agent::new(ControllingMode::Client);

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
}
