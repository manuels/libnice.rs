#![feature(rustc_private)]
#![feature(core)]
#![feature(unique)]
#![feature(libc)]
#![feature(collections)]
#![feature(std_misc)]
#![feature(io_ext)]
#![feature(thread_sleep)]
#![feature(convert)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[macro_use] extern crate log;
extern crate libc;

pub mod bindings_agent;

pub mod agent;
pub mod glib2;
mod from_pointer;
mod syscalls;
mod utils;

#[cfg(test)]
mod tests {
	use std::thread;
	use libc::funcs::bsd43::{send,recv};
	use std::time::duration::Duration;
	use std::thread::sleep;
	use libc::types::common::c95::c_void;
	use libc::types::os::arch::c95::size_t;
	use libc::types::os::arch::posix88::ssize_t;
	use std::sync::mpsc::{channel,Receiver};
	use std::sync::{Arc,Barrier};
	use libc;
	use std::ptr;

	fn start_agent(controlling_mode: bool) ->
			(
				::agent::NiceAgent,
				u32,
				Receiver<libc::c_uint>,
				ptr::Unique<::bindings_agent::GMainContext>)
	{
		let mainloop = ::glib2::GMainLoop::new();
		let ctx = *mainloop.get_context() as *mut ::bindings_agent::GMainContext;
		let mut agent = ::agent::NiceAgent::new(ctx, controlling_mode).unwrap();

		let (stream, state_rx) = agent.add_stream(Some("mystream")).unwrap();

		thread::spawn(move || {
			debug!("glib main loop starting...");
			mainloop.run();
			debug!("glib main loop exited.");
		});

		// must come after mainloop.run()
		agent.gather_candidates(stream);

		unsafe {
			return (agent, stream, state_rx, ptr::Unique::new(ctx));
		}
	}

	#[test]
	fn stream_to_channel() {
		unsafe { ::agent::g_type_init() };

		let (ltx_cred, rrx_cred) = channel();
		let (rtx_cred, lrx_cred) = channel();

		let barrier = Arc::new(Barrier::new(3));

		for (control, tx_cred, rx_cred) in vec![(true, ltx_cred, lrx_cred), (false, rtx_cred, rrx_cred)].into_iter() {
			let bar = barrier.clone();
			thread::spawn(move || {
				let (mut agent, stream, state_rx, ctx) = start_agent(control);

				let cred = agent.generate_local_sdp();
				debug!("cred={}", cred);
				tx_cred.send(cred).unwrap();

				let remote_cred = rx_cred.recv().unwrap();

				let (tx, rx) = agent.stream_to_channel(*ctx, stream, remote_cred, &state_rx).unwrap();

				for i in 0..20 {
					tx.send(vec![1u8, 82+i]).unwrap();
					assert_eq!(rx.recv().unwrap(), vec![1u8, 82+i]);
				}
				bar.wait();
			});
		}
		barrier.wait();
	}

	#[test]
	#[should_panic]
	fn does_timeout() {
		unsafe { ::agent::g_type_init() };

		debug!("1");
		let (mut left, lstream, lstate_rx, lctx) = start_agent(true);
		let (mut right, _, _, _) = start_agent(false);

		let remote_cred = right.generate_local_sdp();

		info!("this test might take a sec");
		left.stream_to_socket(*lctx, lstream, remote_cred, &lstate_rx).unwrap();
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
				let rcred = right.generate_local_sdp();
				let res = left.stream_to_channel(*lctx, lstream, rcred, &lstate_rx);

				left_ok = res.is_ok();
			}

			sleep(Duration::seconds(i));

			if !right_ok {
				let lcred = left.generate_local_sdp();
				let res = right.stream_to_channel(*rctx, rstream, lcred, &rstate_rx);

				right_ok = res.is_ok();
			}

			i -= 5;
		}
	}

	#[test]
	fn stream_to_socket() {
		unsafe { ::agent::g_type_init() };

		debug!("bar1");

		let (ltx,rrx) = channel();
		let (rtx,lrx) = channel();

		let thread = thread::scoped(move || {
			let (mut left, lstream, lstate_rx, lctx) = start_agent(true);
			ltx.send(left.generate_local_sdp()).unwrap();

			let rcred = lrx.recv().unwrap();
			let lfd = left.stream_to_socket(*lctx, lstream, rcred, &lstate_rx).unwrap();

			let input = "foo";
			let output = [0 as i8;3];
			unsafe {
				debug!("lfd: send()");
				assert!(send(lfd, input.as_ptr() as *const c_void, input.len() as size_t, 0) == input.len() as ssize_t);
				debug!("lfd: recv()");
				assert!(recv(lfd, output.as_ptr() as *mut c_void, output.len() as size_t, 0) == output.len() as ssize_t);
				debug!("lfd: done");
			};
		});

		let (mut right, rstream, rstate_rx, rctx) = start_agent(false);
		rtx.send(right.generate_local_sdp()).unwrap();

		let lcred = rrx.recv().unwrap();
		let rfd = right.stream_to_socket(*rctx, rstream, lcred, &rstate_rx).unwrap();

		let input = "bar";
		let output = [0 as i8;3];
		unsafe {
			debug!("rfd: send()");
			assert!(send(rfd, input.as_ptr() as *const c_void, input.len() as size_t, 0) == input.len() as ssize_t);
			debug!("rfd: recv()");
			assert!(recv(rfd, output.as_ptr() as *mut c_void, output.len() as size_t, 0) == output.len() as ssize_t);
			debug!("rfd: done");
		};

		drop(thread);
	}
}
