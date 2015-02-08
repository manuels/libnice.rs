//[-]fn spawn<F>(f: F) -> Thread where F: FnOnce(), F: Send + 'static
use std::thread;

pub fn spawn_thread<'a, F>(name: &'a str, func: F) -> thread::Thread
		where F: FnOnce(), F: Send + 'static
{
	thread::Builder::new().name(name.to_string()).spawn(func)
}
