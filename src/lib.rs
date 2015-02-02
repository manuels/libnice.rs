#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[macro_use] extern crate log;
extern crate libc;

pub mod bindings_agent;

pub mod agent;
pub mod glib2;
mod from_pointer;
mod syscalls;

#[cfg(test)]
mod tests {
	use std::thread::Thread;
	use libc::funcs::bsd43::{send,recv};
	use libc::types::common::c95::c_void;
	use libc::types::os::arch::c95::size_t;
	use libc::types::os::arch::posix88::ssize_t;

	fn start_agent(controlling_mode: bool) -> (::agent::NiceAgent,
			::std::sync::Future<()>, u32,
			*mut ::bindings_agent::GMainContext)
	{
		let mainloop = ::glib2::GMainLoop::new();
		let ctx = mainloop.get_context() as *mut ::bindings_agent::GMainContext;
		let mut agent = ::agent::NiceAgent::new(ctx, controlling_mode);

		let stream = agent.add_stream(Some("mystream")).unwrap();
		let gathered = agent.gather_candidates(stream).unwrap();

		::std::thread::Thread::spawn(move || {
			debug!("glib main loop starting...");
			mainloop.run();
			debug!("glib main loop exited.");
		});
		return (agent, gathered, stream, ctx);
	}

	#[test]
	fn stream_to_channel() {
		unsafe { ::agent::g_type_init() };

		let (mut left,mut lgathered, lstream, lctx) = start_agent(true);
		let (mut right,mut rgathered, rstream, rctx) = start_agent(false);

		lgathered.get();
		right.parse_remote_sdp(left.generate_local_sdp()).unwrap();

		rgathered.get();
		left.parse_remote_sdp(right.generate_local_sdp()).unwrap();

		let (mut lready,lrx) = left.stream_to_channel(lctx, lstream).unwrap();
		let (mut rready,rrx) = right.stream_to_channel(rctx, rstream).unwrap();

		let ltx = lready.get();
		let rtx = rready.get();

		for i in range(0,20) {
			ltx.send(vec![91u8, 82+i]).unwrap();
			rtx.send(vec![19u8, 28-i]).unwrap();
			assert_eq!(lrx.recv().unwrap(), vec![19u8, 28-i]);
			assert_eq!(rrx.recv().unwrap(), vec![91u8, 82+i]);
		}
	}

	#[test]
	fn stream_to_socket() {
		unsafe { ::agent::g_type_init() };

		let (mut left,mut lgathered, lstream, lctx) = start_agent(true);
		let (mut right,mut rgathered, rstream, rctx) = start_agent(false);

		lgathered.get();
		right.parse_remote_sdp(left.generate_local_sdp()).unwrap();

		rgathered.get();
		left.parse_remote_sdp(right.generate_local_sdp()).unwrap();

		let mut lfuture = left.stream_to_socket(lctx, lstream).unwrap();
		let mut rfuture = right.stream_to_socket(rctx, rstream).unwrap();

		Thread::spawn(move || {
			let lfd = lfuture.get();
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

		let rfd = rfuture.get();
		let input = "bar";
		let output = [0 as i8;3];
		unsafe {
			debug!("rfd: send()");
			assert!(send(rfd, input.as_ptr() as *const c_void, input.len() as size_t, 0) == input.len() as ssize_t);
			debug!("rfd: recv()");
			assert!(recv(rfd, output.as_ptr() as *mut c_void, output.len() as size_t, 0) == output.len() as ssize_t);
			debug!("rfd: done");
		};

	}
}
