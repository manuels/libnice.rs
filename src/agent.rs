extern crate libc;

use bindings_agent as bindings;

use from_pointer::FromUtf8Pointer;

use std;
use std::mem;
use std::sync::{Arc,Future};
use std::sync::mpsc::{Sender,Receiver,channel};

macro_rules! warn_on(
	($cond:expr, $msg:expr) => ({
		if $cond {
			warn!($msg)
		}
	})
);

macro_rules! as_result(
	($cond:expr, $ok:expr, $err:expr) => ({
		if $cond {
			Ok($ok)
		} else {
			Err($err)
		}
	})
);

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
		let vec = Vec::from_raw_buf(buf as *mut u8, len as uint);
		(*tx).send(vec)
	};

	if res.is_err() {
		warn!("cb_receive failed!");
		let txx: Box<Sender<Vec<u8>>> = unsafe { mem::transmute(tx) };
		drop(txx);
	};
}

extern "C" fn cb_state_changed(_: *mut bindings::_NiceAgent,
		_:     libc::c_uint, // stream
		_:     libc::c_uint, // component
		state: libc::c_uint,
		txx:   *mut libc::c_void)
{
	debug!("component state changed: {:?}", bindings::NiceComponentState::from_u32(state));

	let is_ready = bindings::NiceComponentState::NICE_COMPONENT_STATE_READY;
	if state == is_ready.to_u32() {
		// using the mem::transmute() instead of using the correct type in
		// the function declaration prevents drop'ing when state is not 'READY'.
		// TODO: can we circumvent this?
		let tx: Box<Sender<()>> = unsafe { mem::transmute(txx) };
		(*tx).send(()).ok().expect("cb_state_changed(): send failed");
	}
}

#[repr(C)]
pub struct GClosure;

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
	pub fn new(ctx: *mut bindings::GMainContext, controlling_mode: bool)
			-> NiceAgent
	{
		let rfc = bindings::NiceCompatibility::NICE_COMPATIBILITY_RFC5245;
		let ptr = unsafe {
			bindings::nice_agent_new(ctx, rfc.to_u32())
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
		let rfc = bindings::NiceCompatibility::NICE_COMPATIBILITY_RFC5245;
		let ptr = unsafe {
			bindings::nice_agent_new_reliable(ctx, rfc.to_u32())
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

	pub fn get_controlling_mode(&self) -> bool {
		let prop = std::ffi::CString::from_slice("controlling-mode".as_bytes());
		let mut value = -1 as libc::c_int;
		unsafe {
			g_object_get(*self.ptr, prop.as_ptr(), &mut value, 0);
		}
		value != FALSE
	}

	pub fn add_stream(&mut self, name: Option<&str>) -> Result<u32,()> {
		let n_components = 1;
		let (tx, rx) = channel();
		let boxed_tx = Box::new(tx);

		let stream = unsafe {
			let func_ptr = mem::transmute(cb_state_changed);
			let data_ptr = mem::transmute(boxed_tx);
			self.on_signal("component_state_changed", func_ptr, data_ptr);

			bindings::nice_agent_add_stream(*self.ptr, n_components) as u32
		};

		if stream == 0 {
			return Err(());
		}

		if name.is_some() {
			self.set_stream_name(stream, name.unwrap());
		}

		self.stream_ready.insert(stream, Future::from_receiver(rx));
		Ok(stream)
	}

	pub fn stream_to_channel(&mut self,
			ctx: *mut bindings::GMainContext,
			stream: u32)
		-> Result<(Future<Sender<Vec<u8>>>, Receiver<Vec<u8>>), ()>
	{
		let (my_tx, your_rx) = channel();
		let (your_tx, my_rx): (Sender<Vec<u8>>,_) = channel();
		let my_boxed_tx = Box::new(my_tx);

		let res = unsafe {
			let func_ptr = mem::transmute(cb_receive);
			let data_ptr = mem::transmute(my_boxed_tx);
			bindings::nice_agent_attach_recv(*self.ptr, stream, 1, ctx,
				func_ptr, data_ptr)
		};
		if res == FALSE {
			return Err(());
		}

		/*
		 * spawn sender thread 
		 */
		let self_ptr = self.ptr.clone();
		::std::thread::Thread::spawn(move || {
			for buf in my_rx.iter() {
				let buf_ptr = buf.as_slice().as_ptr() as *const i8;

				let res = unsafe {
					bindings::nice_agent_send(*self_ptr, stream, 1,
						buf.len() as u32, buf_ptr)
				};
				warn_on!(res < 0, "nice_agent_send() failed!");
			}
		});

		/*
		 * wait for the stream to be come READY and then
		 * return the Sender in the Future
		 */
		let err_msg = "You requested a channel for a stream that does not exist (anymore?).";
		let mut is_stream_ready = self.stream_ready.remove(&stream).expect(err_msg);

		let future = Future::spawn(move || {
			is_stream_ready.get();
			your_tx
		});
		Ok((future, your_rx))
	}

	pub fn remove_stream(&self, stream: u32) {
		unsafe {
			bindings::nice_agent_remove_stream(*self.ptr, stream)
		}
	}

	pub fn set_relay_info(&self,
			stream: u32,
			component_id: u32,
			server_ip: &str,
			port: u16,
			username: &str,
			password: &str,
			typ: u32)
		-> Result<(),()>
	{
		let usr = std::ffi::CString::from_slice(username.as_bytes()).as_ptr();
		let pwd = std::ffi::CString::from_slice(password.as_bytes()).as_ptr();
		let ip  = std::ffi::CString::from_slice(server_ip.as_bytes()).as_ptr();

		let res = unsafe {
			bindings::nice_agent_set_relay_info(*self.ptr, stream, component_id,
				ip, port as u32, usr, pwd, typ)
		};

		as_result!(res != FALSE, (), ())
	}

	pub fn gather_candidates(&self, stream: u32) -> Result<Future<()>,()> {
		let (tx, rx) = channel();
		let boxed_tx = Box::new(tx);

		unsafe {
			let data_ptr = mem::transmute(boxed_tx);
			let func_ptr = mem::transmute(cb_gathered);
			self.on_signal("candidate_gathering_done", func_ptr, data_ptr);
		}

		let res = unsafe {
			bindings::nice_agent_gather_candidates(*self.ptr, stream)
		};
		as_result!(res != FALSE, Future::from_receiver(rx), ())
	}

	pub fn send(&self, stream_id: u32, component_id: u32, buf: &[u8])
		-> Result<uint,()>
	{
		let res = unsafe {
			bindings::nice_agent_send(*self.ptr, stream_id, component_id,
				buf.len() as u32, buf.as_ptr() as *const i8)
		};

		as_result!(res > -1, res as uint, ())
	}

	pub fn reset(&self) -> Result<(),()> {
		let res = unsafe {
			bindings::nice_agent_restart(*self.ptr)
		};
		as_result!(res != FALSE, (), ())
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
		let ptr = unsafe { bindings::nice_agent_get_stream_name(*self.ptr, stream_id) };
		let name = unsafe { FromUtf8Pointer::from_utf8_pointer(&(ptr as *const i8)) };

		unsafe { g_free(ptr as *const libc::c_void) };
		name.unwrap()
	}

	pub fn generate_local_sdp(&self) -> String {
		let ptr = unsafe { bindings::nice_agent_generate_local_sdp(*self.ptr) };
		let sdp = unsafe { FromUtf8Pointer::from_utf8_pointer(&(ptr as *const i8)) };

		unsafe { g_free(ptr as *const libc::c_void) };
		sdp.unwrap()
	}

	/// will fail if no stream with the right name exists!
	pub fn parse_remote_sdp(&self, sdp: String) -> Result<uint,()> {
		let s = std::ffi::CString::from_slice(sdp.as_bytes());

		let count = unsafe {
			bindings::nice_agent_parse_remote_sdp(*self.ptr, s.as_ptr())
		};
		
		as_result!(count > -1, count as uint, ())
	}

	fn on_signal(&self, signal: &str, cb: extern fn(), data_ptr: *mut libc::c_void)
	{
		let signal = std::ffi::CString::from_slice(signal.as_bytes()).as_ptr();

		unsafe {
			let func_ptr = mem::transmute(Some(cb));
			g_signal_connect_data(*self.ptr, signal,
				func_ptr, data_ptr, None, 0);
		}
	}
}
