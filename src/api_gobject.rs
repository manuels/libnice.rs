use std::ptr;
use std::sync::Arc;
use std::ffi::CString;
use libc::{c_void,c_ulong};

use bindings_gobject as ffi;

pub struct GMainLoop {
	ptr: Arc<*mut ffi::_GMainLoop>
}

unsafe impl Send for GMainLoop {}

impl GMainLoop {
	pub fn new() -> GMainLoop {
		let ctx = ptr::null_mut();
		let is_running = ffi::TRUE;

		let ptr = unsafe {
			ffi::g_main_loop_new(ctx, is_running)
		};
		assert!(!ptr.is_null());

		GMainLoop {
			ptr: Arc::new(ptr)
		}
	}

	pub fn run(&self) {
		unsafe {
			ffi::g_main_loop_run(*self.ptr);
		}
	}

	fn from_ptr(ptr: *mut ffi::_GMainLoop) -> Self {
		unsafe {
			ffi::g_main_loop_ref(ptr);
		}

		GMainLoop {
			ptr: Arc::new(ptr)
		}
	}

	pub fn as_ptr(&self) -> *mut ffi::_GMainLoop {
		*self.ptr
	}

	pub fn quit(&self) {
		unsafe {
			ffi::g_main_loop_quit(*self.ptr);
		}
	}
	
	pub fn get_context(&self) -> GMainContext {
		let ctx = unsafe {
			ffi::g_main_loop_get_context(*self.ptr)
		};
		assert!(!ctx.is_null());

		GMainContext::from_ptr(ctx)
	}
}

impl Clone for GMainLoop {
	fn clone(&self) -> Self {
		Self::from_ptr(self.as_ptr())
	}
}

impl Drop for GMainLoop {
	fn drop(&mut self) {
		unsafe {
			ffi::g_main_loop_unref(*self.ptr)
		};
	}
}

pub struct GMainContext {
	ptr: Arc<*mut ffi::_GMainContext>
}

impl GMainContext {
	pub fn from_ptr(ptr: *mut ffi::_GMainContext) -> GMainContext {
		assert!(!ptr.is_null());
		unsafe {
			ffi::g_main_context_ref(ptr)
		};

		GMainContext {
			ptr: Arc::new(ptr)
		}
	}

	pub fn as_ptr(&self) -> *mut ffi::_GMainContext {
		*self.ptr
	}
}

impl Drop for GMainContext {
	fn drop(&mut self) {
		unsafe {
			ffi::g_main_context_unref(*self.ptr)
		};
	}
}

pub trait GObjectTrait {
	fn as_ptr<T>(&self) -> *mut T;

	fn set_property(&self, name: &str, value: *mut c_void) {
		let name = CString::new(name).unwrap();

		unsafe {
			ffi::g_object_set(self.as_ptr(), name.as_ptr(), value, ptr::null_mut());
		}
	}

	/// returns the handler id (always greater than 0 for successful connections)
	fn on_signal(&self,
	             signal:    &str,
	             cb:        extern fn(),
	             user_data: *mut c_void)
		-> c_ulong
	{
		let signal = CString::new(signal).unwrap();

		unsafe {
			ffi::g_signal_connect_data(self.as_ptr(), signal.as_ptr(),
			                           Some(cb), user_data, None, 0)
		}
	}

	fn disconnect_handler(&self, id: c_ulong) {
		unsafe {
			ffi::g_signal_handler_disconnect(self.as_ptr(), id);
		}
	}
}
