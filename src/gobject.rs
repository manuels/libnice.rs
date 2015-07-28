use std::marker::PhantomData;
use std::any::Any;
use std::sync::{MutexGuard,LockResult};

use libc::{c_ulong};

use api_gobject as api;

pub struct GCallbackHandle<'a, R, I>
	where R: 'a + api::GObjectTrait,
	      I: 'a + GObjectTrait<R>,
{
	id:           c_ulong,
	instance:     &'a I,
	raw_instance: PhantomData<&'a R>,
	func:         Box<Any>,
}


impl<'a, R, I>
GCallbackHandle<'a, R, I>
	where R: 'a + api::GObjectTrait,
	      I: 'a + GObjectTrait<R>,
{
	pub fn new<F:Any>(instance: &'a I, func: Box<F>, id: c_ulong) -> GCallbackHandle<'a, R, I> {
		assert!(id > 0);

		GCallbackHandle {
			id:           id,
			instance:     instance,
			raw_instance: PhantomData,
			func:         func,
		}
	}
}

impl<'a, R, I>
Drop for GCallbackHandle<'a, R, I>
	where R: 'a + api::GObjectTrait,
	      I: 'a + GObjectTrait<R>{
	fn drop(&mut self) {
		let lock = self.instance.as_raw().unwrap();
		api::GObjectTrait::disconnect_handler(&*lock, self.id);
	}
}

pub trait GObjectTrait<R: api::GObjectTrait>: Sized {
	fn as_raw(&self) -> LockResult<MutexGuard<R>>;

	fn set_property<P:Sized>(&self, name: &str, value: *mut P) {
		let lock = self.as_raw().unwrap();
		api::GObjectTrait::set_property(&*lock, name, value as *mut _);
	}

	/// The GCallbackHandle must be kept in scope by you!
	/// The callback will be unregistered automatically as soon
	/// as it drops out of scope.
	#[must_use]
	fn on_signal<'a, F: Any>(&'a self,
	                        signal:    &str,
	                        cb:        F,
	                        wrapper:   extern fn())
		-> GCallbackHandle<'a, R, Self>
	{
		let cb = Box::new(cb);
		let user_data = &*cb as *const F as *mut _;

		let lock = self.as_raw().unwrap();
		let id = api::GObjectTrait::on_signal(&*lock, signal, wrapper, user_data);

		GCallbackHandle::new(self, cb, id)
	}
}
