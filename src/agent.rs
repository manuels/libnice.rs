extern crate libc;

use bindings_agent as bindings;

use std;
use std::mem;
use std::sync::{Arc,Future};
use std::sync::mpsc::{Sender,Receiver,channel};

pub struct NiceAgent {
	pub ptr: Arc<*mut bindings::_NiceAgent>,

	stream_ready: std::collections::HashMap<u32, Future<()>>,
}

unsafe impl Send for *mut bindings::_NiceAgent {}
unsafe impl Sync for *mut bindings::_NiceAgent {}

impl Drop for NiceAgent {
	fn drop(&mut self) {
		unsafe {
			g_object_unref(*self.ptr);
		}
	}
}

const FALSE: i32 = 0;
const TRUE: i32 = 1;

fn gbool2result(val: i32) -> Result<(),()> {
	if val == FALSE { Err(()) } else { Ok(()) }
}

extern "C" fn cb_gathered(_: *mut bindings::_NiceAgent,
		_: u32, //stream
		tx: Box<Sender<()>>)
{
	(*tx).send(()).ok().expect("NiceAgent cb_gathered(): Error while sending!");
}

extern "C" fn cb_receive(_: *mut bindings::_NiceAgent,
		_: libc::c_uint, // stream
		_: libc::c_uint, // component
		len: libc::c_uint,
		buf: *mut libc::c_char,
		tx: *mut Sender<Vec<u8>>)
{
	debug!("cb_receive: len={}", len);
	let res = unsafe {
		(*tx).send(Vec::from_raw_buf(buf as *mut u8, len as uint))
	};
	if res.is_err() {
		warn!("cb_receive failed!");
		let _: Box<Sender<Vec<u8>>> = unsafe { mem::transmute(tx) }; // drop Sender
	};
}

extern "C" fn cb_state_changed(_: *mut bindings::_NiceAgent,
		_:     libc::c_uint, // stream
		_:     libc::c_uint, // component
		state: libc::c_uint,
		txx:   *mut libc::c_void)
{
	debug!("component state changed: {}", bindings::NiceComponentState::from_u32(state));
	if state == bindings::NiceComponentState::NICE_COMPONENT_STATE_READY.to_u32() {
		// using the mem::transmute() instead of using the correct type in
		// the function declaration prevents drop'ing when state is not 'READY'.
		// TODO: can we circumvent this?
		let tx: Box<Sender<()>> = unsafe { mem::transmute(txx) };
		(*tx).send(()).ok().expect("cb_state_changed(): send failed");
	}
}

#[repr(C)]
struct GClosure;

extern "C" {
	#[link(name="glib-2.0")]
	fn g_free(ptr: *const libc::c_void);

	#[link(name="glib-2.0")]
	pub fn g_type_init();

	#[link(name="glib-2.0")]
	pub fn g_object_unref(ptr: *mut bindings::_NiceAgent);

	#[link(name="glib-2.0")]
	pub fn g_signal_connect_data(instance: *mut bindings::_NiceAgent,
			detailed_signal: *const libc::c_char,
			c_handler:       Option<extern fn()>,
			data:            *mut libc::c_void,
			destroy_data:    Option<extern fn(*mut libc::c_void, *mut GClosure)>,
			connect_flags:   libc::c_uint)
		-> libc::c_ulong;

	#[link(name="glib-2.0")]
	pub fn g_object_set(instance:     *mut bindings::_NiceAgent,
	                   property_name: *const libc::c_char,
	                   value:         libc::c_int,
	                   null:          libc::c_int);

	#[link(name="glib-2.0")]
	pub fn g_object_get(instance:     *mut bindings::_NiceAgent,
	                   property_name: *const libc::c_char,
	                   value:         *mut libc::c_int,
	                   null:          libc::c_int);
}

impl NiceAgent {
	pub fn new(ctx: *mut bindings::GMainContext, controlling_mode: bool) -> NiceAgent {
		//let ctx = 0 as *mut bindings::GMainContext;
		let ptr = unsafe {
			bindings::nice_agent_new(ctx, bindings::NiceCompatibility::NICE_COMPATIBILITY_RFC5245.to_u32())
		};

		assert!(!ptr.is_null());
		let agent = NiceAgent {
			ptr: Arc::new(ptr),
    		stream_ready: std::collections::HashMap::new()
		};
		agent.set_controlling_mode(controlling_mode);

		agent
	}

	pub fn new_reliable(controlling_mode: bool) -> NiceAgent {
		let ctx = 0 as *mut bindings::GMainContext;
		let ptr = unsafe {
			bindings::nice_agent_new_reliable(ctx, bindings::NiceCompatibility::NICE_COMPATIBILITY_RFC5245.to_u32())
		};
		assert!(!ptr.is_null());
		let agent = NiceAgent {
			ptr: Arc::new(ptr),
    		stream_ready: std::collections::HashMap::new()
		};
		agent.set_controlling_mode(controlling_mode);

		agent
	}

	pub fn set_controlling_mode(&self, controlling_mode: bool) {
		let value = if controlling_mode {TRUE} else {FALSE};
		let prop = std::ffi::CString::from_slice("controlling-mode".as_bytes());

		unsafe {
			g_object_set(*self.ptr, prop.as_ptr(), value, 0);
		}
	}

	pub fn get_controlling_mode(&self) -> bool{
		let prop = std::ffi::CString::from_slice("controlling-mode".as_bytes());
		let mut value = -1 as libc::c_int;
		unsafe {
			g_object_get(*self.ptr, prop.as_ptr(), &mut value, 0);
		}
		value != FALSE
	}

	pub fn add_stream(&mut self, name: Option<&str>) -> Result<u32,()> {
		let n_components = 1;// u32
		let stream = unsafe {
			bindings::nice_agent_add_stream(*self.ptr, n_components)
		};
		if stream == 0 {
			return Err(());
		}

		if name.is_some() {
			self.set_stream_name(stream, name.unwrap());
		}

		let (tx, rx): (Sender<()>,Receiver<()>) = channel();
		let boxed_tx = box tx;

		unsafe {
			let ptr = mem::transmute(boxed_tx);
			self.on_signal("component_state_changed", mem::transmute(cb_state_changed), ptr);
		};
		self.stream_ready.insert(stream as u32, Future::from_receiver(rx));

		Ok(stream)
	}

	pub fn stream_to_channel(&mut self, ctx: *mut bindings::GMainContext, stream: u32) ->
			Result<(Future<Sender<Vec<u8>>>, Receiver<Vec<u8>>), ()> {
		//let ctx = 0 as *mut bindings::GMainContext;
		let (my_tx, your_rx): (Sender<Vec<u8>>,Receiver<Vec<u8>>) = channel();
		let (your_tx, my_rx): (Sender<Vec<u8>>,Receiver<Vec<u8>>) = channel();

		let boxed_my_tx = box my_tx;

		let res = unsafe {
			bindings::nice_agent_attach_recv(*self.ptr, stream, 1, ctx,
				mem::transmute(cb_receive), mem::transmute(boxed_my_tx))
		};
		if res == FALSE {
			return Err(());
		}

		/*
		 * spawn sender thread 
		 */
		let self_ptr = self.ptr.clone();
		::std::thread::Thread::spawn(move || {
			loop {
				match my_rx.recv() {
					Err(_) => {
						/*
						unsafe {
							warn!("calling nice_agent_remove_stream() on {}!", *self_ptr);
							bindings::nice_agent_remove_stream(*self_ptr, stream);
						}*/
						break;
					},
					Ok(buf) => {
						let res = unsafe {
							let ptr = buf.as_slice();
							bindings::nice_agent_send(*self_ptr, stream, 1, buf.len() as u32, ptr.as_ptr() as *const i8)
						};
						if res < 0 {
							warn!("nice_agent_send() failed!");
						};
					}
				}
			}
		}).detach();

		/*
		 * wait for the stream to be come ready and then
		 * return the channel in the future
		 */
		let mut is_stream_ready = self.stream_ready.remove(&stream).unwrap();
		let future = Future::spawn(move || {
			is_stream_ready.get();
			(your_tx)
		});
		Ok((future, your_rx))
	}

	pub fn remove_stream(&self, stream: u32) {
		unsafe {
			bindings::nice_agent_remove_stream(*self.ptr, stream)
		}
	}

	pub fn set_relay_info(&self, stream: u32, component_id: u32, server_ip: &str,
			port: u16, username: &str, password: &str, typ: u32) -> Result<(),()>
	{
		let func = bindings::nice_agent_set_relay_info;

		let usr = std::ffi::CString::from_slice(username.as_bytes());
		let pwd = std::ffi::CString::from_slice(password.as_bytes());
		let ip = std::ffi::CString::from_slice(server_ip.as_bytes());

		let res = unsafe {
			func(*self.ptr, stream, component_id, ip.as_ptr(), port as u32,
				usr.as_ptr(), pwd.as_ptr(), typ)
		};

		gbool2result(res)
	}

	pub fn gather_candidates(&self, stream: u32) -> Result<Future<()>,()> {
		let res = unsafe {bindings::nice_agent_gather_candidates(*self.ptr, stream)};
		
		if res == FALSE {
			Err(())
		} else {
			let (tx, rx) = channel();
			let boxed_tx = box tx;

			unsafe {
				let ptr = mem::transmute(boxed_tx);

				self.on_signal("candidate_gathering_done", mem::transmute(cb_gathered), ptr);
			}
			Ok(Future::from_receiver(rx))
		}
	}

	pub fn send(&self, stream_id: u32, component_id: u32, buf: &[u8]) -> Result<uint,()> {
		let res = unsafe { bindings::nice_agent_send(*self.ptr, stream_id, component_id,
			buf.len() as u32, buf.as_ptr() as *const i8) };

		if res > -1 {
			Ok(res as uint)
		} else {
			Err(())
		}
	}

	pub fn reset(&self) -> Result<(),()> {
		let res = unsafe { bindings::nice_agent_restart(*self.ptr) };
		gbool2result(res)
	}

	pub fn set_stream_tos(&self, stream_id: u32, tos: int) {
		unsafe {
			bindings::nice_agent_set_stream_tos(*self.ptr, stream_id, tos as i32)
		}
	}

	pub fn set_software(&self, software: &str) {
		let sw = std::ffi::CString::from_slice(software.as_bytes());

		unsafe {
			bindings::nice_agent_set_software(*self.ptr, sw.as_ptr())
		}
	}

	pub fn set_stream_name(&self, stream_id: u32, name: &str) {
		let n = std::ffi::CString::from_slice(name.as_bytes());

		unsafe {
			bindings::nice_agent_set_stream_name(*self.ptr, stream_id, n.as_ptr());
		}
	}

	pub fn get_stream_name(&self, stream_id: u32) -> String {
		let ptr = unsafe { bindings::nice_agent_get_stream_name(*self.ptr, stream_id) as *const i8 };
		let name = unsafe { std::str::from_utf8(std::ffi::c_str_to_bytes(&(ptr))).unwrap() }.to_string();

		unsafe { g_free(ptr as *const libc::c_void) };
		name
	}

	pub fn generate_local_sdp(&self) -> String {
		let ptr = unsafe { bindings::nice_agent_generate_local_sdp(*self.ptr) };
		let sdp = unsafe { std::str::from_utf8(std::ffi::c_str_to_bytes(&(ptr as *const i8))).unwrap() }.to_string();

		unsafe { g_free(ptr as *const libc::c_void) };
		sdp
	}

	/// will fail if no stream with the right name exists!
	pub fn parse_remote_sdp(&self, sdp: &str) -> Result<uint,()> {
		let s = std::ffi::CString::from_slice(sdp.as_bytes());
		let count = unsafe {
			bindings::nice_agent_parse_remote_sdp(*self.ptr, s.as_ptr())
		};
		if count < 0 {
			Err(())
		} else {
			Ok(count as uint)
		}
	}

	fn on_signal(&self, signal: &str,
		cb: extern fn(stream_id: u32, data: *mut libc::c_void), data: *mut libc::c_void)
	{
		let null:*const libc::c_int = std::ptr::null();
		let signal = std::ffi::CString::from_slice(signal.as_bytes());

		unsafe {
			g_signal_connect_data(*self.ptr, signal.as_ptr(), mem::transmute(Some(cb)),
				data, mem::transmute(null), 0);
		}
	}
}
