#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate log;
extern crate libc;
#[macro_use]
extern crate bitflags;
extern crate env_logger;

extern crate condition_variable;

use std::ops::Deref;

pub struct Boxing<T: Sized> {
	b: T,
}

impl<T:Sized> Drop for Boxing<T> {
	fn drop(&mut self) {
		debug!("dropping!");
	}
}

impl<T:Sized> Deref for Boxing<T> {
	type Target = T;

    fn deref(&self) -> &Self::Target {
    	&self.b
    }
}

pub mod bindings_agent;
pub mod bindings_gobject;

#[macro_use]
pub mod api_gobject;
pub mod api_agent;

pub mod gobject;
pub mod agent;
