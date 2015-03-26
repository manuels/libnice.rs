extern crate libc;

use bindings_agent as bindings;

use from_pointer::FromUtf8Pointer;
use utils::spawn_thread;

use std;
use std::ptr;
use std::mem;
use std::sync::mpsc::{Sender,Receiver,channel};
use libc::types::common::c95::c_void;

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
	pub ptr: ptr::Unique<bindings::_NiceAgent>,

}

impl NiceAgent {
	pub fn clone(&mut self) -> NiceAgent {
		unsafe {
			g_object_ref(*self.ptr,);
	
			NiceAgent {
				ptr: ptr::Unique::new(*self.ptr,),
			}
		}
	}
}

impl Drop for NiceAgent {
	fn drop(&mut self) {
		unsafe {
			//debug!("NiceAgent::drop() unref({:?})", self.ptr.get());
			g_object_unref(*self.ptr,);
		}
	}
}

const FALSE: i32 = 0;
const TRUE: i32 = 1;

extern "C" fn cb_gathered(_: ptr::Unique<bindings::_NiceAgent>,
		_: u32, //stream
		tx: Box<Sender<()>>)
{
	//debug!("candidates gathered for agent {:?}", agent);

	(*tx).send(()).ok().expect("NiceAgent cb_gathered(): Error while sending!");
}

extern "C" fn cb_receive(_: ptr::Unique<bindings::_NiceAgent>,
		_: libc::c_uint, // stream
		_: libc::c_uint, // component
		len: libc::c_uint,
		buf: *mut libc::c_char,
		tx: *mut Sender<Vec<u8>>)
{
	let res = unsafe {
		let vec = Vec::from_raw_buf(buf as *mut u8, len as usize);
		(*tx).send(vec)
	};

	if res.is_err() {
		warn!("cb_receive failed!");
		let txx: Box<Sender<Vec<u8>>> = unsafe { mem::transmute(tx) };
		drop(txx);
	};
}

extern "C" fn cb_state_changed(_: ptr::Unique<bindings::_NiceAgent>,
		_:     libc::c_uint, // stream
		_:     libc::c_uint, // component
		state: libc::c_uint,
		tx:    *mut Sender<libc::c_uint>)
{
	//debug!("component state changed for agent {:?}: {:?}", agent, bindings::NiceComponentState::from_u32(state));

	let res = unsafe { (*tx).send(state) };
	if res.is_err() {
		warn!("cb_state_changed: send() failed!");
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
	pub fn new(ctx: *mut bindings::GMainContext, controlling_mode: bool)
			-> Result<NiceAgent,()>
	{
		let rfc = bindings::NiceCompatibility::NICE_COMPATIBILITY_RFC5245;
		let ptr = unsafe {
			bindings::nice_agent_new(ctx, rfc.to_u32())
		};

		assert!(!ptr.is_null());
		let mut agent = NiceAgent {
			ptr: unsafe {ptr::Unique::new(ptr)},
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
			ptr: unsafe {ptr::Unique::new(ptr)},
		};
		try!(agent.set_controlling_mode(controlling_mode));

		Ok(agent)
	}

	pub fn set_controlling_mode(&mut self, controlling_mode: bool) -> Result<(),()> {
		let value = if controlling_mode {TRUE} else {FALSE};
		let prop = try!(std::ffi::CString::new("controlling-mode").map_err(|_| ()));

		unsafe {
			g_object_set(*self.ptr, prop.as_ptr(), value, 0);
		};

		Ok(())
	}

	pub fn get_controlling_mode(&mut self) -> Result<bool,()> {
		let prop = try!(std::ffi::CString::new("controlling-mode").map_err(|_| ()));
		let mut value = -1 as libc::c_int;
		unsafe {
			g_object_get(*self.ptr, prop.as_ptr(), &mut value, 0);
		}
		Ok(value != FALSE)
	}

	pub fn add_stream(&mut self, name: Option<&str>)
			-> Result<(u32, Receiver<libc::c_uint>),()>
	{
		let n_components = 1;
		let (tx, state_rx) = channel();
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
			try!(self.set_stream_name(stream, name.unwrap()));
		}

		Ok((stream, state_rx))
	}

	pub fn stream_to_channel(&mut self,
			ctx:      *mut bindings::GMainContext,
			stream:   u32,
			remote_cred: String,
			state_rx: &Receiver<libc::c_uint>)
		-> Result<(Sender<Vec<u8>>, Receiver<Vec<u8>>), ()>
	{
		let is_ready = bindings::NiceComponentState::NICE_COMPONENT_STATE_READY.to_u32();
		let is_failed = bindings::NiceComponentState::NICE_COMPONENT_STATE_FAILED.to_u32();

		let (my_tx, your_rx) = channel();
		let (your_tx, my_rx): (Sender<Vec<u8>>,_) = channel();
		let my_boxed_tx = Box::new(my_tx);

		let res = unsafe {
			let func_ptr = mem::transmute(cb_receive);
			let data_ptr = mem::transmute(my_boxed_tx);
			/* attaching a recv() calback must come BEFORE setting
			 * remote credentials! */
			bindings::nice_agent_attach_recv(*self.ptr, stream, 1, ctx,
				func_ptr, data_ptr)
		};
		if res == FALSE {
			return Err(());
		}

		if try!(self.parse_remote_sdp(remote_cred)) <= 0 {
			return Err(());
		}

		let mut state = state_rx.recv().unwrap();
		while state != is_ready {
			if state == is_failed {
				return Err(());
			}
			state = state_rx.recv().unwrap();
		}


		/*
		 * spawn sender thread 
		 */
		let myself = self.clone();
		spawn_thread("NiceAgent::stream_to_channel::sender", move || {
			for buf in my_rx.iter() {
				let reference:&[u8] = buf.as_ref();
				let buf_ptr = reference.as_ptr() as *const i8;

				let res = unsafe {
					bindings::nice_agent_send(*myself.ptr, stream, 1,
						buf.len() as u32, buf_ptr)
				};
				warn_on!(res < 0, "nice_agent_send() failed!");
			}
		});

		Ok((your_tx, your_rx))
	}

	pub fn remove_stream(&mut self, stream: u32) {
		unsafe {
			bindings::nice_agent_remove_stream(*self.ptr, stream)
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
			bindings::nice_agent_set_relay_info(*self.ptr, stream, component_id,
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
			bindings::nice_agent_gather_candidates(*self.ptr, stream)
		};
		assert!(res == TRUE);

		//debug!("gathering");
		rx.recv().unwrap();
		//debug!("gathered");
	}

	pub fn send(&mut self, stream_id: u32, component_id: u32, buf: &[u8])
		-> Result<usize,()>
	{
		let res = unsafe {
			bindings::nice_agent_send(*self.ptr, stream_id, component_id,
				buf.len() as u32, buf.as_ptr() as *const i8)
		};

		as_result!(res > -1, res as usize, ())
	}

	pub fn reset(&mut self) -> Result<(),()> {
		let res = unsafe {
			bindings::nice_agent_restart(*self.ptr)
		};
		as_result!(res != FALSE, (), ())
	}

	pub fn set_stream_tos(&mut self, stream_id: u32, tos: i32) {
		unsafe {
			bindings::nice_agent_set_stream_tos(*self.ptr, stream_id, tos)
		}
	}

	pub fn set_software(&mut self, software: &str) -> Result<(),()> {
		let sw = try!(std::ffi::CString::new(software).map_err(|_| ()));

		unsafe {
			bindings::nice_agent_set_software(*self.ptr, sw.as_ptr())
		};

		Ok(())
	}

	pub fn set_stream_name(&mut self, stream_id: u32, name: &str) -> Result<(),()> {
		let n = try!(std::ffi::CString::new(name).map_err(|_| ()));

		unsafe {
			bindings::nice_agent_set_stream_name(*self.ptr, stream_id, n.as_ptr());
		}

		Ok(())

	}

	pub fn get_stream_name(&mut self, stream_id: u32) -> String {
		let ptr = unsafe { bindings::nice_agent_get_stream_name(*self.ptr, stream_id) };
		let name = unsafe { FromUtf8Pointer::from_utf8_pointer(ptr as *const i8) };

		unsafe { g_free(ptr as *const libc::c_void) };
		name.unwrap()
	}

	pub fn generate_local_sdp(&mut self) -> String {
		let ptr = unsafe { bindings::nice_agent_generate_local_sdp(*self.ptr,) };
		let sdp = unsafe { FromUtf8Pointer::from_utf8_pointer(ptr as *const i8) };

		unsafe { g_free(ptr as *const libc::c_void) };
		sdp.unwrap()
	}

	/// will fail if no stream with the right name exists!
	pub fn parse_remote_sdp(&mut self, sdp: String) -> Result<usize,()> {
		let s = try!(std::ffi::CString::new(sdp).map_err(|_| ()));

		let count = unsafe {
			bindings::nice_agent_parse_remote_sdp(*self.ptr, s.as_ptr())
		};
		
		as_result!(count > -1, count as usize, ())
	}

	fn on_signal(&mut self, signal: &str, cb: extern fn(), data_ptr: *mut libc::c_void)
	{
		let signal = std::ffi::CString::new(signal).unwrap();

		unsafe {
			let func_ptr = mem::transmute(Some(cb));
			g_signal_connect_data(*self.ptr, signal.as_ptr(),
				func_ptr, data_ptr, None, 0);
		}
	}
}
