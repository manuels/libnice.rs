extern crate libc;
use std::ptr;

pub mod bindings {
	extern crate libc;

	#[repr(C)]
	pub struct GMainContext;

	#[repr(C)]
	pub struct GMainLoop;

	/*
	GMainContext * g_main_context_new() [struct _GMainContext *]
	*/
	#[link(name="nice")]
	extern "C" {
		pub fn g_main_context_new() -> *mut GMainContext;
	}

	/*
	GMainLoop * g_main_loop_new() [struct _GMainLoop *]
		(GMainContext *) context [struct _GMainContext *]
		(gboolean) is_running [int]
	*/
	#[link(name="glib-2.0")]
	extern "C" {
		pub fn g_main_loop_new(context: *mut GMainContext, is_running: libc::c_int) -> *mut GMainLoop;
	}


	#[link(name="glib-2.0")]
	extern "C" {
		pub fn g_main_loop_get_context(loop_: *mut GMainLoop) -> *mut GMainContext;
	}

	/*
	void g_main_loop_run()
		(GMainLoop *) loop [struct _GMainLoop *]
	*/
	#[link(name="glib-2.0")]
	extern "C" {
		pub fn g_main_loop_run(loop_: *mut GMainLoop);
	}

	/*
	void g_main_loop_unref()
		(GMainLoop *) loop [struct _GMainLoop *]
	*/
	#[link(name="glib-2.0")]
	extern "C" {
		pub fn g_main_loop_unref(loop_: *mut GMainLoop);
	}
}


pub struct GMainLoop {
	ptr: *mut bindings::GMainLoop
}

impl Drop for GMainLoop {
	fn drop(&mut self) {
		unsafe { bindings::g_main_loop_unref(self.ptr) }
	}
}

unsafe impl Send for GMainLoop {}

impl GMainLoop {
	pub fn new() -> GMainLoop {
		let ctx:*mut bindings::GMainContext = ptr::null_mut();
		let is_running = 0;
		
		unsafe {
			let pointer = bindings::g_main_loop_new(ctx, is_running);
			assert!(!pointer.is_null());
	
			GMainLoop { ptr: pointer }
		}
	}

	pub fn get_context(&self) -> *mut bindings::GMainContext {
		let pointer = unsafe { bindings::g_main_loop_get_context(self.ptr) };
		assert!(!pointer.is_null());
		pointer
	}


	pub fn run(&self) {
		unsafe { bindings::g_main_loop_run(self.ptr) };
	}
}
