use std::string::String;
use std::ffi::CString;

use bindings_address as bindings;

pub struct NiceAddress {
	pub ptr: *mut bindings::_NiceAddress
}

impl Drop for NiceAddress {
	fn drop(&mut self) {
		if !self.ptr.is_null() {
			unsafe { bindings::nice_address_free(self.ptr) }
		}
	}
}

impl NiceAddress {
	pub fn new() -> NiceAddress {
		let ptr = unsafe { bindings::nice_address_new() };
		assert!(!ptr.is_null());

		let addr = NiceAddress {ptr: ptr};
		addr.init();

		addr
	}

	pub fn init(&self) {
		unsafe { bindings::nice_address_init(self.ptr) }
	}

	/*
	pub fn set_ip(&self, addr: IpAddr) {
		match addr {
			Ipv4Addr(a,b,c,d) => {
				unsafe { bindings::nice_address_set_ipv4 }
			},
			Ipv6Addr(a,b,c,d,e,f,g,h) => {
				unsafe { bindings::nice_address_set_ipv6 }
			}
		}
	}

	pub fn nice_address_set_from_sockaddr(addr: *mut _NiceAddress, sin: *const sockaddr);
	*/

	pub fn get_port(&self) -> u16 {
		unsafe { bindings::nice_address_get_port(self.ptr) as u16 }
	}

	pub fn set_from_string(&self, string: &str) -> Result<(),()> {
		let s = CString::from_slice(string.as_bytes());
		unsafe {
			if bindings::nice_address_set_from_string(self.ptr, s.as_ptr() as *const i32) != 0 {
				Ok(())
			} else {
				Err(())
			}
		}
	}

	pub fn is_private(&self) -> bool {
		unsafe { bindings::nice_address_is_private(self.ptr) != 0 }
	}
}

impl Clone for NiceAddress {
	fn clone(&self) -> Self {
		let ptr = unsafe { bindings::nice_address_dup(self.ptr) };
		NiceAddress {ptr: ptr}
	}
}

impl PartialEq for NiceAddress {
	fn eq(&self, other: &Self) -> bool {
		unsafe { bindings::nice_address_equal(self.ptr, other.ptr) != 0 }
	}
}

impl ToString for NiceAddress {
    fn to_string(&self) -> String {
    	let mut buf = Vec::with_capacity(1024);
    	unsafe {
    		bindings::nice_address_to_string(self.ptr, buf.as_mut_ptr() as *mut i32);
    		String::from_utf8(buf.to_vec()).unwrap_or(String::from_str(""))
    	}
    }
}
