use std::ffi::{CStr, CString};
use std::str;
use std::ops::{Range,DerefMut};
use std::ptr;
use std::slice;
use std::mem;

use libc::{c_int, c_uint, c_char, c_void, c_ulong};

use bindings_agent as ffi;
use bindings_gobject as ffi_gobject;

use api_gobject::{GMainContext,GObjectTrait};

pub use bindings_agent::NiceComponentState;

pub use bindings_agent::NiceCompatibility;

pub struct Agent {
	ptr: *mut ffi::_NiceAgent,
}

pub type RecvFunc = extern fn(*mut ffi::_NiceAgent,  /* agent */
                              c_uint,                /* stream_id */
                              c_uint,                /* component_id */
                              c_uint,                /* len */
                              *mut c_char,           /* buf */
                              *mut c_void);          /* user_data */

pub type CandidateGatheringDone = extern fn(*mut ffi::_NiceAgent,  /* agent */
                                            c_uint,                /* stream_id */
                                            *mut c_void);          /* user_data */

// TODO:
// nice_agent_send_messages_nonblocking()
// nice_agent_recv_messages_nonblocking()
// nice_agent_recv_messages()
// nice_agent_recv_nonblocking()
// nice_agent_get_io_stream()
// nice_agent_get_selected_socket

impl Agent {
	pub fn new(ctx: &GMainContext, compat: NiceCompatibility) -> Agent {
		let ctx = ctx.as_ptr() as *mut ffi::_GMainContext;

		let ptr = unsafe {
			ffi::nice_agent_new(ctx, compat.bits())
		};
		assert!(!ptr.is_null());

		Agent {
			ptr: ptr,
		}
	}

	pub fn new_reliable(ctx: &GMainContext, compat: NiceCompatibility) -> Agent
	{
		let ctx = ctx.as_ptr() as *mut ffi::_GMainContext;

		let ptr = unsafe {
			ffi::nice_agent_new_reliable(ctx, compat.bits())
		};
		assert!(!ptr.is_null());

		Agent {
			ptr: ptr,
		}
	}

	/*
	/// Add a local address from which to derive local host candidates for
	/// candidate gathering.
	pub fn add_local_address<T: ToSocketAddrs>(&self, addr: T) -> bool {
		for addr in Address::new(addr) {
			unsafe {
				ffi::nice_agent_add_local_address(self.ptr, addr.as_ptr());
			}
		}
	}
	*/

	pub fn set_port_range(&self,
	                      stream_id:    c_uint,
	                      component_id: c_uint,
	                      range:        Range<c_uint>)
	{
		unsafe {
			ffi::nice_agent_set_port_range(self.ptr, stream_id, component_id,
				range.start as u32, range.end as u32)
		};
	}

	pub fn add_stream(&self, n_components: usize) -> Option<c_uint> {
		let stream_id = unsafe {
			ffi::nice_agent_add_stream(self.ptr, n_components as c_uint)
		};

		if stream_id == 0 {
			None
		} else {
			Some(stream_id)
		}
	}

	pub fn remove_stream(&self, stream_id: c_uint) {
		unsafe {
			ffi::nice_agent_remove_stream(self.ptr, stream_id)
		};
	}

	/*
	pub fn set_relay_info(&self,
	                      stream_id:    c_uint,
	                      component_id: c_int,
	                      relay:        &RelayInfo)
		-> bool
	{
		let res = unsafe {
			ffi::nice_agent_set_relay_info(self.ptr,
				stream_id,
				component_id,
				CString::new(relay.server_ip).as_ptr(),
				relay.server_port,
				CString::new(relay.username).as_ptr(),
				CString::new(relay.password).as_ptr(),
				relay.relay_type)
		};

		res != ffi::FALSE
	}
	*/

	/*
	pub fn forget_relays(&self, stream_id: c_uint, component_id: c_int) -> bool {
		let res = unsafe {
			ffi::nice_agent_forget_relays(self.ptr, stream_id, component_id);
		};

		res != ffi::FALSE
	}
	*/

	/// Allocate and start listening on local candidate ports and start the
	/// remote candidate gathering process. Once done, "candidate-gathering-done"
	/// is called for the stream. As soon as this function is called,
	/// "new-candidate" signals may be emitted, even before this function returns.
	///
	/// get_local_candidates() will only return non-empty results after calling
	/// this function. 
	pub fn gather_candidates(&self, stream_id: c_uint) -> bool {
		let res = unsafe {
			ffi::nice_agent_gather_candidates(self.ptr, stream_id)
		};

		res != ffi::FALSE
	}


	pub fn set_remote_credentials(&self, stream_id: c_uint, ufrag: &str, pwd: &str)
		-> bool
	{
		let ptr_ufrag = CString::new(ufrag);
		let ptr_pwd   = CString::new(pwd);

		if ptr_ufrag.is_err() || ptr_pwd.is_err() {
			return false;
		}

		let res = unsafe {
			ffi::nice_agent_set_remote_credentials(self.ptr,
			                                       stream_id,
			                                       ptr_ufrag.unwrap().as_ptr(),
			                                       ptr_pwd.unwrap().as_ptr())
		};

		res != ffi::FALSE
	}

	/// returns (ufrag, pwd) pair
	/// `None` will be returned if this is called for a non-existent stream.
	pub fn get_local_credentials(&self, stream_id: c_uint)
		-> Option<(String, String)>
	{
		let mut ptr_ufrag: *mut c_char = ptr::null_mut();
		let mut ptr_pwd:   *mut c_char = ptr::null_mut();

		let res = unsafe {
			ffi::nice_agent_get_local_credentials(self.ptr,
			                                      stream_id,
			                                      &mut ptr_ufrag,
			                                      &mut ptr_pwd)
		};

		if res == ffi::FALSE || ptr_ufrag.is_null() || ptr_pwd.is_null() {
			if !ptr_ufrag.is_null() {
				unsafe {
					ffi::g_free(ptr_ufrag as *mut c_int);
				}
			}
			if !ptr_pwd.is_null() {
				unsafe {
					ffi::g_free(ptr_pwd as *mut c_int);
				}
			}

			return None;
		}

		let buf = unsafe { CStr::from_ptr(ptr_ufrag).to_bytes() };
		let ufrag = str::from_utf8(buf).unwrap().to_owned();

		let buf = unsafe {CStr::from_ptr(ptr_pwd).to_bytes() };
		let pwd = str::from_utf8(buf).unwrap().to_owned();

		unsafe {
			ffi::g_free(ptr_ufrag as *mut c_int);
			ffi::g_free(ptr_pwd as *mut c_int);
		}

		Some((ufrag, pwd))
	}

	/*
	/// returns `None` on error (memory allocation error or invalid component)
	pub fn set_remote_candidates(&self,
	                             stream_id:    c_int,
	                             component_id: c_int,
	                             candidates:   &GSList<Candidate>)
		-> Option<usize>
	{
		let res = unsafe {
			ffi::nice_agent_set_remote_credentials(self.ptr,
			                                       stream_id,
			                                       component_id,
			                                       candidates.as_ptr())
		};

		if res < 0 {
			None
		} else {
			Some(res)
		}
	}
	*/

	/*
	pub fn get_remote_candidates(&self, stream_id: c_uint, component_id: c_int)
		-> Option<GSList<Candidate>>
	{
		let res = unsafe {
			ffi::nice_agent_get_remote_candidates(self.ptr,
			                                      stream_id,
			                                      component_id);
		};

		GSList::from_ptr(res)
	}
	*/

	/*
	pub fn get_local_candidates(&self, stream_id: c_uint, component_id: c_int)
		-> Option<GSList<Candidate>>
	{
		let res = unsafe {
			ffi::nice_agent_get_local_candidates(self.ptr,
			                                     stream_id,
			                                     component_id);
		};

		GSList::from_ptr(res)
	}
	*/

	/*
	/// returns (local_candidate, remote_candidate) pair
	pub fn get_selected_pair(&self, stream_id: c_uint, component_id: c_int)
		-> Option<(Candidate,Candidate)>
	{
		let local_ptr:  *const c_int = ptr::null_mut();
		let remote_ptr: *const c_int = ptr::null_mut();

		let res = unsafe {
			ffi::nice_agent_get_selected_pair(self.ptr, stream_id, component_id,
				&mut local_ptr, &mut remote_ptr)
		};

		if res == ffi::FALSE || local_ptr.is_null() || remote_ptr.is_null() {
			None
		} else {
			let local  = Candidate::from_ptr(local_ptr);
			let remote = Candidate::from_ptr(remote_ptr);

			match (local, remote) {
				(Some(local), Some(remote)) => Some((local, remote)),
				_ => None,
			}
		}
	}
	*/

	pub fn set_selected_pair(&self,
	                         stream_id: c_uint,
	                         component_id: c_uint,
	                         lfoundation: &str,
	                         rfoundation: &str)
		-> bool
	{
		let ptr_left = CString::new(lfoundation);
		let ptr_right = CString::new(rfoundation);

		if ptr_left.is_err() || ptr_right.is_err() {
			return false;
		}

		let res = unsafe {
			ffi::nice_agent_set_selected_pair(self.ptr,
			                                  stream_id,
			                                  component_id,
			                                  ptr_left.unwrap().as_ptr(),
			                                  ptr_right.unwrap().as_ptr())
		};

		res != ffi::FALSE
	}

	/*
	pub fn set_selected_remote_candidate(&self,
	                                     stream_id:    c_int,
	                                     component_id: c_int,
	                                     cand:         Candidate)
		-> bool
	{
		let res = unsafe {
			ffi::nice_agent_set_selected_pair(self.ptr,
				stream_id,
				component_id,
				candidate.as_ptr())
		};

		res != ffi::FALSE
	}
	*/

	pub fn set_stream_tos(&self, stream_id: c_uint, tos: c_int) {
		unsafe {
			ffi::nice_agent_set_stream_tos(self.ptr, stream_id, tos)
		}
	}

	pub fn set_software(&self, software: &str) {
		if let Ok(ptr) = CString::new(software) {
			unsafe {
				ffi::nice_agent_set_software(self.ptr, ptr.as_ptr())
			}
		}
	}

	/// Component state MUST be NICE_COMPONENT_STATE_READY, or as a special
	/// case, in any state if component was in READY state before and was then
	/// restarted.
	///
	/// In reliable mode, the -1 error value means either that you are not yet
	/// connected or that the send buffer is full (equivalent to EWOULDBLOCK).
	/// In both cases, you simply need to wait for the
	/// "reliable-transport-writable" signal to be fired before resending the
	/// data.
	///
	/// In non-reliable mode, it will virtually never happen with UDP sockets,
	/// but it might happen if the active candidate is a TURN-TCP connection that
	/// got disconnected.
	///
	/// In both reliable and non-reliable mode, a -1 error code could also mean
	/// that the stream_id and/or component_id are invalid.
	pub fn send(&self, stream_id: c_uint, component_id: c_uint, buf: &[u8])
		-> Option<usize>
	{
		let len = unsafe {
			let ptr = buf.as_ptr() as *const i8;
			let len = buf.len() as c_uint;
			ffi::nice_agent_send(self.ptr, stream_id, component_id, len, ptr)
		};

		if len < 0 {
			None
		} else {
			Some(len as usize)
		}
	}

	/*
	pub fn recv(&self, stream_id: c_uint, component_id: c_int, buf: &[u8])
		-> Option<usize>
	{
		let len = unsafe {
			let cancellable = ptr::null();
			let error       = ptr::null();

			let ptr = buf.as_ptr();
			let len = buf.len() as c_uint;

			ffi::nice_agent_recv(self.ptr,
			                     stream_id,
			                     component_id,
			                     ptr,
			                     len,
			                     cancellable,
			                     error)
		};

		if len < 0 {
			// TODO: use GError
			None
		} else {
			Some(len as usize)
		}
	}
	*/

	pub fn attach_recv<'a,'b, F:Fn(&'b [u8])>(&'a self,
	                      stream_id:    c_uint,
	                      component_id: c_uint,
	                      ctx:          &'a GMainContext,
//	                      user_data:    &mut Box<Sender<Vec<u8>>>)
	                      callback:     &'a mut Box<F>)
		-> bool
	{
		#[allow(unused_variables)]
		extern fn wrapper<'c, G:Fn(&'c [u8])>(agent:               *mut ffi::_NiceAgent,
                           actual_stream_id:    c_uint,
                           actual_component_id: c_uint,
                           len:                 c_uint,
                           buf:                 *mut c_char,
                           user_data:           *mut c_void)
		{
//			let tx = user_data as *mut Sender<Vec<u8>>;
			let closure = user_data as *mut G;

			unsafe {
				let buf = slice::from_raw_parts(buf as *mut u8, len as usize);

				let res = (*closure)(buf);
//				(*tx).send(buf.to_vec()).unwrap();
			};
		}

		let ctx = ctx.as_ptr() as *mut _;

		let res = unsafe {
			let func_ptr  = mem::transmute(wrapper::<'b, F>);
			//let user_data = (user_data.deref_mut()) as *mut Sender<Vec<u8>> as *mut _;
			let user_data = (callback.deref_mut()) as *mut F as *mut _;

			ffi::nice_agent_attach_recv(self.ptr, stream_id, component_id,
				ctx, func_ptr, user_data)
		};

		res != ffi::FALSE
	}

	pub fn unattach_recv(&self,
	                     stream_id:    c_uint,
	                     component_id: c_uint,
	                     ctx:          &GMainContext)
		-> bool
	{
		let ctx = ctx.as_ptr() as *mut ffi::_GMainContext;

		let res = unsafe {
			ffi::nice_agent_attach_recv(self.ptr, stream_id, component_id,
				ctx, None, ptr::null_mut())
		};

		res != ffi::FALSE
	}

	pub fn restart(&self) -> bool {
		let res = unsafe {
			ffi::nice_agent_restart(self.ptr)
		};

		res != ffi::FALSE
	}

	pub fn restart_stream(&self, stream_id: c_uint) -> bool {
		let res = unsafe {
			ffi::nice_agent_restart_stream(self.ptr, stream_id)
		};

		res != ffi::FALSE
	}

	/*
	pub fn restart_stream(&self, stream_id: c_uint) -> bool {
		let res = unsafe {
			ffi::nice_agent_restart_stream(self.ptr, stream_id)
		};

		res != ffi::FALSE
	}
	*/

	pub fn set_stream_name(&self, stream_id: c_uint, name: &str) -> bool {
		if let Ok(ptr) = CString::new(name) {
			let res = unsafe {
				ffi::nice_agent_set_stream_name(self.ptr, stream_id, ptr.as_ptr())
			};

			res != ffi::FALSE
		} else {
			return false;
		}
	}

	pub fn get_stream_name(&self, stream_id: c_uint) -> Option<String> {
		let ptr = unsafe {
			ffi::nice_agent_get_stream_name(self.ptr, stream_id)
		};

		if ptr.is_null() {
			return None;
		}

		let buf = unsafe { CStr::from_ptr(ptr).to_bytes() };
		let name = str::from_utf8(buf).unwrap().to_owned();

		Some(name)
	}

	/*
	pub fn get_default_local_candidate(&self, stream_id: c_uint, component_id: c_uint)
		-> Option<Candidate>
	{
		let ptr = unsafe {
			ffi::nice_agent_get_default_local_candidate(self.ptr, stream_id, component_id)
		};

		Candidate::from_ptr(ptr)
	}
	*/

	pub fn generate_local_sdp(&self) -> Option<String> {
		let ptr = unsafe {
			ffi::nice_agent_generate_local_sdp(self.ptr)
		};

		if ptr.is_null() {
			return None;
		}

		let buf = unsafe { CStr::from_ptr(ptr).to_bytes() };
		let sdp = str::from_utf8(buf).unwrap().to_owned();

		Some(sdp)
	}

	pub fn generate_local_stream_sdp(&self, stream_id: c_uint, include_non_ice: bool)
		-> Option<String>
	{
		let include_non_ice = if include_non_ice {ffi::TRUE} else {ffi::FALSE};
		let ptr = unsafe {
			ffi::nice_agent_generate_local_stream_sdp(self.ptr, stream_id, include_non_ice)
		};

		if ptr.is_null() {
			return None;
		}

		let buf = unsafe { CStr::from_ptr(ptr).to_bytes() };
		let sdp = str::from_utf8(buf).unwrap().to_owned();

		Some(sdp)
	}

	#[must_use]
	pub fn on_candidate_gathering_done<F>(&self, cb: &F) -> c_ulong
		where F: Fn(c_uint) -> bool
	{
		#[allow(unused_variables)]
		extern fn wrapper<F>(_agent:    *mut ffi::_NiceAgent,
		                     stream_id: c_uint,
		                     user_data: *mut c_void)
			-> c_int
				where F: Fn(c_uint) -> bool
		{
			let closure = user_data as *mut F;
			unsafe {
				let res = (*closure)(stream_id);
				return res as c_int;
			}
		}

		let user_data = cb as *const _ as *mut c_void;
		unsafe {
			let func_ptr = mem::transmute(wrapper::<F>);
			self.on_signal("candidate-gathering-done", func_ptr, user_data)
		}
	}

	#[must_use]
	pub fn on_component_state_changed<F>(&self, cb: &F) -> c_ulong
		where F: Fn(c_uint, c_uint, NiceComponentState) -> bool
	{
		extern fn wrapper<F>(_/*agent*/:   *mut ffi::_NiceAgent,
		                     stream_id:    c_uint,
		                     component_id: c_uint,
		                     state:        c_uint,
		                     user_data:    *mut c_void)
			-> c_int
			where F: Fn(c_uint, c_uint, NiceComponentState) -> bool
		{
			let closure = user_data as *mut F;

			unsafe {
				let state = mem::transmute(state);
				let res = (*closure)(stream_id, component_id, state);
				return res as c_int;
			}
		}

		let user_data = cb as *const _ as *mut c_void;
		unsafe {
			let func_ptr = mem::transmute(wrapper::<F>);
			self.on_signal("component-state-changed", func_ptr, user_data)
		}
	}

	/*
	pub fn generate_local_candidate_sdp(&self, cand: Candidate) -> Option<String>
	{
		let ptr = unsafe {
			ffi::nice_agent_generate_local_candidate_sdp(self.ptr, cand.as_ptr())
		};

		if ptr.is_null() {
			return None;
		}

		let buf = CStr::from_ptr(ptr).to_bytes();
		let sdp = str::from_utf8(buf).unwrap().to_owned();

		Some(sdp)
	}
	*/

	pub fn parse_remote_sdp(&self, sdp: &str) -> Option<usize> {
		if let Ok(ptr) = CString::new(sdp) {
			let res = unsafe {
				ffi::nice_agent_parse_remote_sdp(self.ptr, ptr.as_ptr())
			};

			if res < 0 {
				None
			} else {
				Some(res as usize)
			}
		} else {
			return None
		}
	}

	/*
	/// Returns (candidates, ufrag, pwd) triple
	pub fn parse_remote_stream_sdp(&self,
	                               stream_id: c_uint,
	                               sdp: &str)
		-> Option<(GSList<Candidate>, String, String)>
	{
		let mut ptr_ufrag = ptr::null_mut();
		let mut ptr_pwd   = ptr::null_mut();

		let list_ptr = unsafe {
			nice_agent_parse_remote_stream_sdp(self.ptr,
			                                   stream_id,
			                                   CString::new(sdp).as_ptr(),
			                                   ptr_ufrag,
			                                   ptr_pwd)
		};

		let list = GSList::from_ptr(list_ptr);

		if list.is_none() || ptr_ufrag.is_null() || ptr_pwd.is_null() {
			for ptr in (ptr_ufrag, ptr_pwd) {
				if !ptr.is_null() {
					unsafe {
						ffi::g_free(ptr);
					}
				}
			}
			// list gets free'd automatically

			return None;
		}

		let buf = CStr::from_ptr(ptr_ufrag).to_bytes();
		let ufrag = str::from_utf8(buf).unwrap().to_owned();

		let buf = CStr::from_ptr(ptr_pwd).to_bytes();
		let pwd = str::from_utf8(buf).unwrap().to_owned();

		unsafe {
			ffi::g_free(ptr_ufrag);
			ffi::g_free(ptr_pwd);
		}

		Some((list.unwrap(), ufrag, pwd))
	}
	*/

	/*
	pub fn parse_remote_candidate_sdp(&self,
	                                  stream_id: c_uint,
	                                  sdp: &str)
		-> Option<GSList<Candidate>>
	{
		let list_ptr = unsafe {
			nice_agent_parse_remote_candidate_sdp(self.ptr,
			                                      stream_id,
			                                      CString::new(sdp).as_ptr())
		};

		GSList::from_ptr(list_ptr)
	}
	*/

	pub fn state_to_string(state: NiceComponentState) -> &'static str {
		let ptr = unsafe {
			ffi::nice_component_state_to_string(state as c_uint)
		};

		let buf = unsafe { CStr::from_ptr(ptr).to_bytes() };
		str::from_utf8(buf).unwrap()
	}
}

impl Drop for Agent {
	fn drop(&mut self) {
		unsafe {
			ffi_gobject::g_object_unref(self.ptr as *mut _);
		}
	}
}

impl GObjectTrait for Agent {
	fn as_ptr<T>(&self) -> *mut T {
		self.ptr as *mut T
	}
}
