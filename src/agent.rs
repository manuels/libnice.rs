extern crate libc;

use bindings_agent as bindings;

use std;
use std::slice;
use std::mem;
use std::sync::mpsc::{Sender,Receiver,channel};
use libc::types::common::c95::c_void;
use libc::types::os::arch::c95::c_ulong;
use std::thread;
use std::ffi::CStr;

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
	pub ptr: *mut bindings::_NiceAgent,
}

unsafe impl Send for NiceAgent {}
unsafe impl Sync for NiceAgent {}

impl NiceAgent {
	pub fn clone(&mut self) -> NiceAgent {
		unsafe {
			g_object_ref(self.ptr);
		}
	
		NiceAgent {
			ptr: self.ptr,
		}
	}
}

impl Drop for NiceAgent {
	fn drop(&mut self) {
		unsafe {
			debug!("NiceAgent::drop() unref({:?})", self.ptr);
			g_object_unref(self.ptr);
		}
	}
}

const FALSE: i32 = 0;
const TRUE:  i32 = 1;

extern "C" fn cb_gathered(_: *mut bindings::_NiceAgent,
		_: u32, //stream
		tx: Box<Sender<()>>)
{
	match (*tx).send(()) {
		Ok(_)  => {},
		Err(err) => error!("NiceAgent cb_gathered(): Error while sending ({:?})!", err),
	}
}

extern "C" fn cb_receive(_: *mut bindings::_NiceAgent,
		_: libc::c_uint, // stream
		_: libc::c_uint, // component
		len: libc::c_uint,
		buf: *mut libc::c_char,
		tx: Box<Sender<Vec<u8>>>)
{
	let buf = unsafe {
		slice::from_raw_parts(buf as *mut u8, len as usize)
	};

	match (*tx).send(buf.to_vec()) {
		Ok(_)    => mem::forget(tx), // keep tx alive!
		Err(err) => {
			warn!("cb_receive failed {:?}!", err);
			drop(tx);
		},
	}
}

extern "C" fn cb_state_changed(agent: *mut bindings::_NiceAgent,
		_:     libc::c_uint, // stream
		_:     libc::c_uint, // component
		state: libc::c_uint,
		tx:    Box<Sender<libc::c_uint>>)
{
	debug!("component state changed for agent {:?}: {:?}", agent, bindings::NiceComponentState::from_u32(state));

	match (*tx).send(state) {
		Ok(_)    => mem::forget(tx), // keep tx alive!
		Err(err) => error!("cb_state_changed: send() failed ({:?})!", err),
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
	pub fn g_object_ref(ptr: *mut bindings::_NiceAgent);

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
	pub fn g_object_set(instance:      *mut bindings::_NiceAgent,
	                    property_name: *const libc::c_char,
	                    value:         libc::c_int,
	                    null:          libc::c_int);

	#[link(name="glib-2.0")]
	pub fn g_object_get(instance:      *mut bindings::_NiceAgent,
	                    property_name: *const libc::c_char,
	                    value:         *mut libc::c_int,
	                    null:          libc::c_int);
}

impl NiceAgent {
	pub fn new(ctx: *mut ::glib2::bindings::GMainContext, controlling_mode: bool)
			-> Result<NiceAgent,()>
	{
		let ctx = ctx as *mut bindings::GMainContext;
		let rfc = bindings::NiceCompatibility::NICE_COMPATIBILITY_RFC5245;
		let ptr = unsafe {
			bindings::nice_agent_new(ctx, rfc.to_u32())
		};
		assert!(!ptr.is_null());

		let mut agent = NiceAgent {
			ptr: ptr,
		};

		try!(agent.set_controlling_mode(controlling_mode));
		Ok(agent)
	}

	pub fn new_reliable(controlling_mode: bool) -> Result<NiceAgent,()> {
		let ctx = 0 as *mut bindings::GMainContext;
		let rfc = bindings::NiceCompatibility::NICE_COMPATIBILITY_RFC5245;
		let ptr = unsafe {
			bindings::nice_agent_new_reliable(ctx, rfc.to_u32())
		};
		assert!(!ptr.is_null());

		let mut agent = NiceAgent {
			ptr: ptr,
		};
		try!(agent.set_controlling_mode(controlling_mode));

		Ok(agent)
	}

	pub fn set_controlling_mode(&mut self, controlling_mode: bool) -> Result<(),()> {
		let value = if controlling_mode {TRUE} else {FALSE};
		let prop = try!(std::ffi::CString::new("controlling-mode").map_err(|_| ()));

		unsafe {
			g_object_set(self.ptr, prop.as_ptr(), value, 0);
		};

		Ok(())
	}

	pub fn get_controlling_mode(&mut self) -> Result<bool,()> {
		let prop = try!(std::ffi::CString::new("controlling-mode").map_err(|_| ()));
		let mut value = -1 as libc::c_int;
		unsafe {
			g_object_get(self.ptr, prop.as_ptr(), &mut value, 0);
		}
		Ok(value != FALSE)
	}

	pub fn add_stream(&mut self, name: Option<&str>)
			-> Result<(u32, Receiver<libc::c_uint>),()>
	{
		let n_components = 1;
		let (state_tx, state_rx) = channel();
		let boxed_tx = Box::new(state_tx);

		let stream = unsafe {
			let func_ptr = mem::transmute(cb_state_changed);
			let data_ptr = mem::transmute(boxed_tx);
			self.on_signal("component_state_changed", func_ptr, data_ptr);

			bindings::nice_agent_add_stream(self.ptr, n_components) as u32
		};

		if stream == 0 {
			error!("nice_agent_add_stream() failed!");
			return Err(());
		}

		if name.is_some() {
			try!(self.set_stream_name(stream, name.unwrap()));
		}

		Ok((stream, state_rx))
	}

	pub fn stream_to_channel(&mut self,
			ctx:         *mut ::glib2::bindings::GMainContext,
			stream:      u32,
			remote_cred: String,
			state_rx:    &Receiver<libc::c_uint>,
			my_tx:       Sender<Vec<u8>>,
			my_rx:       Receiver<Vec<u8>>)
		-> Result<(), ()>
	{
		let ctx = ctx as *mut bindings::GMainContext;
		let is_ready  = bindings::NiceComponentState::NICE_COMPONENT_STATE_READY.to_u32();
		let is_failed = bindings::NiceComponentState::NICE_COMPONENT_STATE_FAILED.to_u32();

		let my_boxed_tx = Box::new(my_tx);

		let res = unsafe {
			let func_ptr = mem::transmute(cb_receive);
			let data_ptr = mem::transmute(my_boxed_tx);
			/* attaching a recv() calback must come BEFORE setting
			 * remote credentials! */
			bindings::nice_agent_attach_recv(self.ptr, stream, 1, ctx,
				func_ptr, data_ptr)
		};
		if res == FALSE {
			error!("nice_agent_attach_recv() failed!");
			return Err(());
		}

		if try!(self.parse_remote_sdp(remote_cred)) <= 0 {
			info!("No valid remote credentials!");
			return Err(()); // TODO: better error message!
		}

		loop {
			let state = state_rx.recv().unwrap();

			if state == is_ready {
				break
			}
			if state == is_failed {
				return Err(())
			}
		}

		/*
		 * spawn sender thread 
		 */
		let mut this = self.clone();
		thread::Builder::new().name("NiceAgent::stream_to_channel::sender".to_string()).spawn(move || {
			for buf in my_rx.iter() {
				match this.send(stream, 1, &buf[..]) {
					Err(err) => warn!("nice_agent_send() failed: {:?}!", err),
					Ok(len) if len == buf.len() => {},
					Ok(len) => warn!("nice_agent_send() only sent {} of {} bytes!", len, buf.len()),
				}
			}
		}).unwrap();

		Ok(())
	}

	pub fn remove_stream(&mut self, stream: u32) {
		unsafe {
			bindings::nice_agent_remove_stream(self.ptr, stream)
		}
	}

	pub fn set_relay_info(&mut self,
			stream: u32,
			component_id: u32,
			server_ip: &str,
			port: u16,
			username: &str,
			password: &str,
			typ: u32)
		-> Result<(),()>
	{
		let usr = try!(std::ffi::CString::new(username).map_err(|_| ()));
		let pwd = try!(std::ffi::CString::new(password).map_err(|_| ()));
		let ip = try!(std::ffi::CString::new(server_ip).map_err(|_| ()));

		let res = unsafe {
			bindings::nice_agent_set_relay_info(self.ptr, stream, component_id,
				ip.as_ptr(), port as u32, usr.as_ptr(), pwd.as_ptr(), typ)
		};

		as_result!(res != FALSE, (), ())
	}

	pub fn gather_candidates(&mut self, stream: u32) {
		/* ! A main loop must be running before caling this method !
		 * This method is blocking */
		let (tx, rx): (Sender<()>,_) = channel();
		let boxed_tx = Box::new(tx);

		unsafe {
			let data_ptr = mem::transmute(boxed_tx);
			let func_ptr = mem::transmute(cb_gathered);
			self.on_signal("candidate_gathering_done", func_ptr, data_ptr);
		}

		let res = unsafe {
			bindings::nice_agent_gather_candidates(self.ptr, stream)
		};
		assert!(res != FALSE);

		debug!("gathering");
		rx.recv().unwrap();
		debug!("gathered");
	}

	pub fn send(&mut self, stream_id: u32, component_id: u32, buf: &[u8])
		-> Result<usize,()>
	{
		let res = unsafe {
			bindings::nice_agent_send(self.ptr, stream_id, component_id,
				buf.len() as u32, buf.as_ptr() as *const i8)
		};

		as_result!(res > -1, res as usize, ())
	}

	pub fn reset(&mut self) -> Result<(),()> {
		let res = unsafe {
			bindings::nice_agent_restart(self.ptr)
		};
		as_result!(res != FALSE, (), ())
	}

	pub fn set_stream_tos(&mut self, stream_id: u32, tos: i32) {
		unsafe {
			bindings::nice_agent_set_stream_tos(self.ptr, stream_id, tos)
		}
	}

	pub fn set_software(&mut self, software: &str) -> Result<(),()> {
		let sw = try!(std::ffi::CString::new(software).map_err(|_| ()));

		unsafe {
			bindings::nice_agent_set_software(self.ptr, sw.as_ptr())
		};

		Ok(())
	}

	pub fn set_stream_name(&mut self, stream_id: u32, name: &str) -> Result<(),()> {
		let n = try!(std::ffi::CString::new(name).map_err(|_| ())); // TODO: better error

		unsafe {
			bindings::nice_agent_set_stream_name(self.ptr, stream_id, n.as_ptr());
		}

		Ok(())

	}

	pub fn get_stream_name(&mut self, stream_id: u32) -> String {
		let ptr = unsafe { bindings::nice_agent_get_stream_name(self.ptr, stream_id) };
		let name = unsafe { CStr::from_ptr(ptr as *const i8) };
		let name = std::str::from_utf8(name.to_bytes()).unwrap().to_string();

		unsafe { g_free(ptr as *const libc::c_void) };
		name
	}

	pub fn generate_local_sdp(&mut self) -> Result<String,()> {
		let ptr = unsafe { bindings::nice_agent_generate_local_sdp(self.ptr) };
		let sdp = unsafe { CStr::from_ptr(ptr as *const i8) };
		let sdp = try!(std::str::from_utf8(sdp.to_bytes()).map_err(|_| ()));
		let sdp = sdp.to_string();

		unsafe { g_free(ptr as *const libc::c_void) };
		Ok(sdp)
	}

	/// will fail if no stream with the right name exists!
	pub fn parse_remote_sdp(&mut self, sdp: String) -> Result<usize,()> {
		let s = try!(std::ffi::CString::new(sdp).map_err(|_| ()));

		let count = unsafe {
			bindings::nice_agent_parse_remote_sdp(self.ptr, s.as_ptr())
		};
		
		as_result!(count > -1, count as usize, ())
	}

	fn on_signal(&mut self, signal: &str, cb: extern fn(), data_ptr: *mut libc::c_void)
	{
		let signal = std::ffi::CString::new(signal).unwrap();

		unsafe {
			let func_ptr = mem::transmute(Some(cb));
			g_signal_connect_data(self.ptr, signal.as_ptr(),
				func_ptr, data_ptr, None, 0);
		}
	}
}
