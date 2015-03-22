use from_pointer::FromUtf8Pointer;

use libc::types::os::arch::c95::c_int;
use libc::funcs::c95::string::strerror;
use std::os::unix::io::Fd;
use std::os::errno;

mod syscall {
	use libc::types::os::arch::c95::c_int;

	extern "C" {
		pub fn socketpair(domain: c_int, typ: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
	}
}

pub fn socketpair(domain: c_int, typ: c_int, protocol: c_int)
	-> Result<(Fd, Fd), String>
{
	let mut sv = [-1 as Fd; 2];

	let res = unsafe {
		syscall::socketpair(domain, typ, protocol, sv.as_mut_ptr())
	};

	match res {
		0 => Ok((sv[0], sv[1])),
		_ => unsafe {
			let ptr = strerror(errno() as c_int);
			let msg = FromUtf8Pointer::from_utf8_pointer(ptr as *const i8);
			Err(msg.unwrap())
		}
	}
}
