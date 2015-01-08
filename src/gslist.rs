extern crate libc;

use bindings_agent as bindings;

pub struct GSList {
	ptr: *mut bindings::GSList
}

impl GSList {
	pub fn new() -> GSList {
		let ptr = unsafe { bindings::g_slist_alloc() };
		assert!(!ptr.is_null());
		GSList { ptr:ptr }
	}

	pub fn append<T>(&self, element: T) {
		let ptr = unsafe { bindings::g_slist_append(self.ptr, element) };
		assert!(!ptr.is_null());
		self.ptr = ptr;
	}
}

impl Drop for GSList {
	fn drop(&mut self) {
		unsafe { bindings::g_slist_free(self.ptr) }
	}
}
