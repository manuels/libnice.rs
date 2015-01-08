#![allow(dead_code)]
#![allow(non_camel_case_types)]

#![feature(phase)]
#![feature(plugin)]
#[plugin] #[macro_use] extern crate log;

mod bindings_agent;

mod agent;
mod glib2;


#[cfg(test)]
mod tests {
	fn start_agent(controlling_mode: bool) -> (::agent::NiceAgent,
			::std::sync::Future<()>, u32, ::std::sync::Future<()>,
			*mut ::bindings_agent::GMainContext)
	{
		let mainloop = ::glib2::GMainLoop::new();
		let ctx = mainloop.get_context() as *mut ::bindings_agent::GMainContext;
		let agent = ::agent::NiceAgent::new(ctx, controlling_mode);

		let (stream,is_ready) = agent.add_stream(Some("mystream")).unwrap();

		let gathered = agent.gather_candidates(stream).unwrap();

		::std::thread::Thread::spawn(move || {
			debug!("glib main loop starting...");
			mainloop.run();
			debug!("glib main loop exited.");
		}).detach();
		return (agent, gathered, stream, is_ready, ctx);
	}

	#[test]
	fn it_works() {
		unsafe { ::agent::g_type_init() };

		let (left,mut lgathered, lstream,mut lready,lctx) = start_agent(true);
		let (right,mut rgathered, rstream, mut rready,rctx) = start_agent(false);

		lgathered.get();
		right.parse_remote_sdp(left.generate_local_sdp().as_slice()).unwrap();

		rgathered.get();
		left.parse_remote_sdp(right.generate_local_sdp().as_slice()).unwrap();

		let (ltx,lrx) = left.stream_to_channel(lctx, lstream).unwrap();
		let (rtx,rrx) = right.stream_to_channel(rctx, rstream).unwrap();

		// this MUST come after stream_to_channel() 'cause stream_to_channel() installs the receiver!
		lready.get();
		rready.get();

		for i in range(0,20) {
			ltx.send(vec![91u8, 82+i]).unwrap();
			rtx.send(vec![19u8, 28-i]).unwrap();
			assert_eq!(lrx.recv().unwrap(), vec![19u8, 28-i]);
			assert_eq!(rrx.recv().unwrap(), vec![91u8, 82+i]);

		}
	}
}
