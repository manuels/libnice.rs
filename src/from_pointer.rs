use std::ffi;
use std::str;

pub trait FromUtf8Pointer {
	unsafe fn from_utf8_pointer(ptr: *const i8) -> Result<Self, ()>;
}

impl FromUtf8Pointer for String {
	unsafe fn from_utf8_pointer(ptr: *const i8) -> Result<Self, ()> {
		assert!(!ptr.is_null());

		let array = ffi::CStr::from_ptr(ptr).to_bytes();
		let utf8 = str::from_utf8(array);

		utf8.map(|s| {s.to_string()}).map_err(|_| ())
	}
}
