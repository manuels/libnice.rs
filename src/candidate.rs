extern crate libc;

use bindings_agent as bindings;

pub struct NiceCandidate {
	ptr: *mut bindings::_NiceCandidate
}

impl NiceCandidate {
	pub fn new(type_: bindings::NiceCompatibility) -> NiceCandidate {
		let ptr = unsafe { bindings::nice_candidate_new(type_ as u32) };
		assert!(!ptr.is_null());
		NiceCandidate { ptr:ptr }
	}
}

impl Clone for NiceCandidate {
	fn clone(&self) -> NiceCandidate {
		let ptr = unsafe { bindings::nice_candidate_copy(self.ptr) };
		assert!(!ptr.is_null());
		NiceCandidate { ptr:ptr }
	}
}

impl Drop for NiceCandidate {
	fn drop(&mut self) {
		unsafe { bindings::nice_candidate_free(self.ptr) }
	}
}
