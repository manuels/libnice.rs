#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate log;
extern crate libc;

pub mod bindings_agent;

pub mod agent;
pub mod glib2;

pub use agent::NiceAgent;

#[cfg(test)]
mod tests {
	use std::thread;
	use std::thread::sleep_ms;
	use std::sync::mpsc::{channel,Receiver};
	use std::sync::{Arc,Barrier};
	use libc;

	fn start_agent(controlling_mode: bool) ->
	    (::agent::NiceAgent,
	     u32,
	     Receiver<libc::c_uint>,
	     *mut ::glib2::bindings::GMainContext)
	{
		let mainloop = ::glib2::GMainLoop::new();
		let ctx = mainloop.get_context();
		let mut agent = ::agent::NiceAgent::new(ctx, controlling_mode).unwrap();

		let (stream, state_rx) = agent.add_stream(Some("mystream")).unwrap();

		thread::Builder::new().name("gmainloop test".to_string()).spawn(move || {
			debug!("glib main loop starting...");
			mainloop.run();
			debug!("glib main loop exited.");
		}).unwrap();

		// must come after mainloop.run()
		agent.gather_candidates(stream);

		(agent, stream, state_rx, ctx)
	}

	#[test]
	fn stream_to_channel() {
		unsafe { ::agent::g_type_init() };

		let (ltx_cred, rrx_cred) = channel();
		let (rtx_cred, lrx_cred) = channel();

		let barrier = Arc::new(Barrier::new(3));

		for (control, tx_cred, rx_cred) in vec![(true, ltx_cred, lrx_cred), (false, rtx_cred, rrx_cred)].into_iter() {
			let bar = barrier.clone();
			thread::Builder::new().name("test io".to_string()).spawn(move || {
				let (mut agent, stream, state_rx, ctx) = start_agent(control);

				let cred = agent.generate_local_sdp().unwrap();
				debug!("cred={}", cred);
				tx_cred.send(cred).unwrap();

				let remote_cred = rx_cred.recv().unwrap();

				let (tx, rrx) = channel();
				let (ttx, rx) = channel();
				agent.stream_to_channel(ctx, stream, remote_cred, &state_rx,
					ttx, rrx).unwrap();

				for i in 0..20 {
					tx.send(vec![1u8, 82+i]).unwrap();
					assert_eq!(rx.recv().unwrap(), vec![1u8, 82+i]);
				}
				bar.wait();
			}).unwrap();
		}
		barrier.wait();
	}

	#[test]
	#[should_panic]
	fn does_timeout() {
		unsafe { ::agent::g_type_init() };

		let (mut left, lstream, lstate_rx, lctx) = start_agent(true);
		let (mut right, _, _, _) = start_agent(false);

		let remote_cred = right.generate_local_sdp().unwrap();

		let (tx, rrx) = channel();
		let (ttx, rx) = channel();

		info!("this test might take a sec");
		left.stream_to_channel(lctx,
			lstream, remote_cred, &lstate_rx, ttx, rrx).unwrap();

		drop(tx);
		drop(rx);
	}

	#[test]
	fn retry_works() {
		unsafe { ::agent::g_type_init() };

		let (mut left, lstream, lstate_rx, lctx) = start_agent(true);
		let (mut right, rstream, rstate_rx, rctx) = start_agent(false);

		let mut left_ok = false;
		let mut right_ok = false;
		let mut i = 20;

		while !(left_ok && right_ok) {
			if !left_ok {
				let rcred = right.generate_local_sdp().unwrap();

				let (tx, rrx) = channel();
				let (ttx, rx) = channel();
				let res = left.stream_to_channel(lctx, lstream, rcred, &lstate_rx,
					ttx, rrx);

				left_ok = res.is_ok();
				drop(tx);
				drop(rx);
			}

			sleep_ms(i*1000);

			if !right_ok {
				let lcred = left.generate_local_sdp().unwrap();

				let (tx, rrx) = channel();
				let (ttx, rx) = channel();
				let res = right.stream_to_channel(rctx, rstream, lcred, &rstate_rx,
					ttx, rrx);

				right_ok = res.is_ok();
				drop(tx);
				drop(rx);
			}

			debug!("{}", i);
			i -= 5;
		}
	}
}
