#![allow(dead_code)]
#![allow(unstable)]

extern crate libc;
use std::mem;
use std::marker::Send;

use std::collections::enum_set::CLike;

unsafe impl Send for *mut _GMainLoop {}
unsafe impl Sync for *mut _GMainLoop {}
unsafe impl Send for *mut _GMainContext {}
unsafe impl Sync for *mut _GMainContext {}

/*
GArray * g_array_new() [struct _GArray *]
	(gboolean) zero_terminated [int]
	(gboolean) clear_ [int]
	(guint) element_size [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_array_new(zero_terminated: libc::c_int, clear_: libc::c_int, element_size: libc::c_uint) -> *mut _GArray;
}


/*
GArray * g_array_sized_new() [struct _GArray *]
	(gboolean) zero_terminated [int]
	(gboolean) clear_ [int]
	(guint) element_size [unsigned int]
	(guint) reserved_size [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_array_sized_new(zero_terminated: libc::c_int, clear_: libc::c_int, element_size: libc::c_uint, reserved_size: libc::c_uint) -> *mut _GArray;
}


/*
gchar * g_array_free() [char *]
	(GArray *) array [struct _GArray *]
	(gboolean) free_segment [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_array_free(array: *mut _GArray, free_segment: libc::c_int) -> *mut libc::c_char;
}


/*
GArray * g_array_ref() [struct _GArray *]
	(GArray *) array [struct _GArray *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_array_ref(array: *mut _GArray) -> *mut _GArray;
}


/*
void g_array_unref()
	(GArray *) array [struct _GArray *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_array_unref(array: *mut _GArray);
}


/*
guint g_array_get_element_size() [unsigned int]
	(GArray *) array [struct _GArray *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_array_get_element_size(array: *mut _GArray) -> libc::c_uint;
}


/*
GArray * g_array_append_vals() [struct _GArray *]
	(GArray *) array [struct _GArray *]
	(gconstpointer) data [const void *]
	(guint) len [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_array_append_vals(array: *mut _GArray, data: *const libc::c_void, len: libc::c_uint) -> *mut _GArray;
}


/*
GArray * g_array_prepend_vals() [struct _GArray *]
	(GArray *) array [struct _GArray *]
	(gconstpointer) data [const void *]
	(guint) len [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_array_prepend_vals(array: *mut _GArray, data: *const libc::c_void, len: libc::c_uint) -> *mut _GArray;
}


/*
GArray * g_array_insert_vals() [struct _GArray *]
	(GArray *) array [struct _GArray *]
	(guint) index_ [unsigned int]
	(gconstpointer) data [const void *]
	(guint) len [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_array_insert_vals(array: *mut _GArray, index_: libc::c_uint, data: *const libc::c_void, len: libc::c_uint) -> *mut _GArray;
}


/*
GArray * g_array_set_size() [struct _GArray *]
	(GArray *) array [struct _GArray *]
	(guint) length [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_array_set_size(array: *mut _GArray, length: libc::c_uint) -> *mut _GArray;
}


/*
GArray * g_array_remove_index() [struct _GArray *]
	(GArray *) array [struct _GArray *]
	(guint) index_ [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_array_remove_index(array: *mut _GArray, index_: libc::c_uint) -> *mut _GArray;
}


/*
GArray * g_array_remove_index_fast() [struct _GArray *]
	(GArray *) array [struct _GArray *]
	(guint) index_ [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_array_remove_index_fast(array: *mut _GArray, index_: libc::c_uint) -> *mut _GArray;
}


/*
GArray * g_array_remove_range() [struct _GArray *]
	(GArray *) array [struct _GArray *]
	(guint) index_ [unsigned int]
	(guint) length [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_array_remove_range(array: *mut _GArray, index_: libc::c_uint, length: libc::c_uint) -> *mut _GArray;
}


/*
void g_array_sort()
	(GArray *) array [struct _GArray *]
	(GCompareFunc) compare_func [int (*)(const void *, const void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_array_sort(array: *mut _GArray, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>);
}


/*
void g_array_sort_with_data()
	(GArray *) array [struct _GArray *]
	(GCompareDataFunc) compare_func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_array_sort_with_data(array: *mut _GArray, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
void g_array_set_clear_func()
	(GArray *) array [struct _GArray *]
	(GDestroyNotify) clear_func [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_array_set_clear_func(array: *mut _GArray, clear_func: Option<extern fn(*mut libc::c_void)>);
}


/*
GPtrArray * g_ptr_array_new() [struct _GPtrArray *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ptr_array_new() -> *mut _GPtrArray;
}


/*
GPtrArray * g_ptr_array_new_with_free_func() [struct _GPtrArray *]
	(GDestroyNotify) element_free_func [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ptr_array_new_with_free_func(element_free_func: Option<extern fn(*mut libc::c_void)>) -> *mut _GPtrArray;
}


/*
GPtrArray * g_ptr_array_sized_new() [struct _GPtrArray *]
	(guint) reserved_size [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ptr_array_sized_new(reserved_size: libc::c_uint) -> *mut _GPtrArray;
}


/*
GPtrArray * g_ptr_array_new_full() [struct _GPtrArray *]
	(guint) reserved_size [unsigned int]
	(GDestroyNotify) element_free_func [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ptr_array_new_full(reserved_size: libc::c_uint, element_free_func: Option<extern fn(*mut libc::c_void)>) -> *mut _GPtrArray;
}


/*
gpointer * g_ptr_array_free() [void **]
	(GPtrArray *) array [struct _GPtrArray *]
	(gboolean) free_seg [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ptr_array_free(array: *mut _GPtrArray, free_seg: libc::c_int) -> *mut *mut libc::c_void;
}


/*
GPtrArray * g_ptr_array_ref() [struct _GPtrArray *]
	(GPtrArray *) array [struct _GPtrArray *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ptr_array_ref(array: *mut _GPtrArray) -> *mut _GPtrArray;
}


/*
void g_ptr_array_unref()
	(GPtrArray *) array [struct _GPtrArray *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ptr_array_unref(array: *mut _GPtrArray);
}


/*
void g_ptr_array_set_free_func()
	(GPtrArray *) array [struct _GPtrArray *]
	(GDestroyNotify) element_free_func [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ptr_array_set_free_func(array: *mut _GPtrArray, element_free_func: Option<extern fn(*mut libc::c_void)>);
}


/*
void g_ptr_array_set_size()
	(GPtrArray *) array [struct _GPtrArray *]
	(gint) length [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ptr_array_set_size(array: *mut _GPtrArray, length: libc::c_int);
}


/*
gpointer g_ptr_array_remove_index() [void *]
	(GPtrArray *) array [struct _GPtrArray *]
	(guint) index_ [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ptr_array_remove_index(array: *mut _GPtrArray, index_: libc::c_uint) -> *mut libc::c_void;
}


/*
gpointer g_ptr_array_remove_index_fast() [void *]
	(GPtrArray *) array [struct _GPtrArray *]
	(guint) index_ [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ptr_array_remove_index_fast(array: *mut _GPtrArray, index_: libc::c_uint) -> *mut libc::c_void;
}


/*
gboolean g_ptr_array_remove() [int]
	(GPtrArray *) array [struct _GPtrArray *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ptr_array_remove(array: *mut _GPtrArray, data: *mut libc::c_void) -> libc::c_int;
}


/*
gboolean g_ptr_array_remove_fast() [int]
	(GPtrArray *) array [struct _GPtrArray *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ptr_array_remove_fast(array: *mut _GPtrArray, data: *mut libc::c_void) -> libc::c_int;
}


/*
void g_ptr_array_remove_range()
	(GPtrArray *) array [struct _GPtrArray *]
	(guint) index_ [unsigned int]
	(guint) length [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ptr_array_remove_range(array: *mut _GPtrArray, index_: libc::c_uint, length: libc::c_uint);
}


/*
void g_ptr_array_add()
	(GPtrArray *) array [struct _GPtrArray *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ptr_array_add(array: *mut _GPtrArray, data: *mut libc::c_void);
}


/*
void g_ptr_array_sort()
	(GPtrArray *) array [struct _GPtrArray *]
	(GCompareFunc) compare_func [int (*)(const void *, const void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ptr_array_sort(array: *mut _GPtrArray, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>);
}


/*
void g_ptr_array_sort_with_data()
	(GPtrArray *) array [struct _GPtrArray *]
	(GCompareDataFunc) compare_func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ptr_array_sort_with_data(array: *mut _GPtrArray, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
void g_ptr_array_foreach()
	(GPtrArray *) array [struct _GPtrArray *]
	(GFunc) func [void (*)(void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ptr_array_foreach(array: *mut _GPtrArray, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
GByteArray * g_byte_array_new() [struct _GByteArray *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_byte_array_new() -> *mut _GByteArray;
}


/*
GByteArray * g_byte_array_new_take() [struct _GByteArray *]
	(guint8 *) data [unsigned char *]
	(gsize) len [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_byte_array_new_take(data: *mut libc::c_uchar, len: libc::c_ulong) -> *mut _GByteArray;
}


/*
GByteArray * g_byte_array_sized_new() [struct _GByteArray *]
	(guint) reserved_size [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_byte_array_sized_new(reserved_size: libc::c_uint) -> *mut _GByteArray;
}


/*
guint8 * g_byte_array_free() [unsigned char *]
	(GByteArray *) array [struct _GByteArray *]
	(gboolean) free_segment [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_byte_array_free(array: *mut _GByteArray, free_segment: libc::c_int) -> *mut libc::c_uchar;
}


/*
GBytes * g_byte_array_free_to_bytes() [struct _GBytes *]
	(GByteArray *) array [struct _GByteArray *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_byte_array_free_to_bytes(array: *mut _GByteArray) -> *mut _GBytes;
}


/*
GByteArray * g_byte_array_ref() [struct _GByteArray *]
	(GByteArray *) array [struct _GByteArray *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_byte_array_ref(array: *mut _GByteArray) -> *mut _GByteArray;
}


/*
void g_byte_array_unref()
	(GByteArray *) array [struct _GByteArray *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_byte_array_unref(array: *mut _GByteArray);
}


/*
GByteArray * g_byte_array_append() [struct _GByteArray *]
	(GByteArray *) array [struct _GByteArray *]
	(const guint8 *) data [const unsigned char *]
	(guint) len [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_byte_array_append(array: *mut _GByteArray, data: *const libc::c_uchar, len: libc::c_uint) -> *mut _GByteArray;
}


/*
GByteArray * g_byte_array_prepend() [struct _GByteArray *]
	(GByteArray *) array [struct _GByteArray *]
	(const guint8 *) data [const unsigned char *]
	(guint) len [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_byte_array_prepend(array: *mut _GByteArray, data: *const libc::c_uchar, len: libc::c_uint) -> *mut _GByteArray;
}


/*
GByteArray * g_byte_array_set_size() [struct _GByteArray *]
	(GByteArray *) array [struct _GByteArray *]
	(guint) length [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_byte_array_set_size(array: *mut _GByteArray, length: libc::c_uint) -> *mut _GByteArray;
}


/*
GByteArray * g_byte_array_remove_index() [struct _GByteArray *]
	(GByteArray *) array [struct _GByteArray *]
	(guint) index_ [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_byte_array_remove_index(array: *mut _GByteArray, index_: libc::c_uint) -> *mut _GByteArray;
}


/*
GByteArray * g_byte_array_remove_index_fast() [struct _GByteArray *]
	(GByteArray *) array [struct _GByteArray *]
	(guint) index_ [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_byte_array_remove_index_fast(array: *mut _GByteArray, index_: libc::c_uint) -> *mut _GByteArray;
}


/*
GByteArray * g_byte_array_remove_range() [struct _GByteArray *]
	(GByteArray *) array [struct _GByteArray *]
	(guint) index_ [unsigned int]
	(guint) length [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_byte_array_remove_range(array: *mut _GByteArray, index_: libc::c_uint, length: libc::c_uint) -> *mut _GByteArray;
}


/*
void g_byte_array_sort()
	(GByteArray *) array [struct _GByteArray *]
	(GCompareFunc) compare_func [int (*)(const void *, const void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_byte_array_sort(array: *mut _GByteArray, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>);
}


/*
void g_byte_array_sort_with_data()
	(GByteArray *) array [struct _GByteArray *]
	(GCompareDataFunc) compare_func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_byte_array_sort_with_data(array: *mut _GByteArray, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
gint g_atomic_int_get() [int]
	(volatile gint *) atomic [volatile int *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_atomic_int_get(atomic: *mut libc::c_int) -> libc::c_int;
}


/*
void g_atomic_int_set()
	(volatile gint *) atomic [volatile int *]
	(gint) newval [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_atomic_int_set(atomic: *mut libc::c_int, newval: libc::c_int);
}


/*
void g_atomic_int_inc()
	(volatile gint *) atomic [volatile int *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_atomic_int_inc(atomic: *mut libc::c_int);
}


/*
gboolean g_atomic_int_dec_and_test() [int]
	(volatile gint *) atomic [volatile int *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_atomic_int_dec_and_test(atomic: *mut libc::c_int) -> libc::c_int;
}


/*
gboolean g_atomic_int_compare_and_exchange() [int]
	(volatile gint *) atomic [volatile int *]
	(gint) oldval [int]
	(gint) newval [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_atomic_int_compare_and_exchange(atomic: *mut libc::c_int, oldval: libc::c_int, newval: libc::c_int) -> libc::c_int;
}


/*
gint g_atomic_int_add() [int]
	(volatile gint *) atomic [volatile int *]
	(gint) val [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_atomic_int_add(atomic: *mut libc::c_int, val: libc::c_int) -> libc::c_int;
}


/*
guint g_atomic_int_and() [unsigned int]
	(volatile guint *) atomic [volatile unsigned int *]
	(guint) val [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_atomic_int_and(atomic: *mut libc::c_uint, val: libc::c_uint) -> libc::c_uint;
}


/*
guint g_atomic_int_or() [unsigned int]
	(volatile guint *) atomic [volatile unsigned int *]
	(guint) val [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_atomic_int_or(atomic: *mut libc::c_uint, val: libc::c_uint) -> libc::c_uint;
}


/*
guint g_atomic_int_xor() [unsigned int]
	(volatile guint *) atomic [volatile unsigned int *]
	(guint) val [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_atomic_int_xor(atomic: *mut libc::c_uint, val: libc::c_uint) -> libc::c_uint;
}


/*
gpointer g_atomic_pointer_get() [void *]
	(volatile void *) atomic
*/
#[link(name="nice")]
extern "C" {
	pub fn g_atomic_pointer_get(atomic: *mut libc::c_void) -> *mut libc::c_void;
}


/*
void g_atomic_pointer_set()
	(volatile void *) atomic
	(gpointer) newval [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_atomic_pointer_set(atomic: *mut libc::c_void, newval: *mut libc::c_void);
}


/*
gboolean g_atomic_pointer_compare_and_exchange() [int]
	(volatile void *) atomic
	(gpointer) oldval [void *]
	(gpointer) newval [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_atomic_pointer_compare_and_exchange(atomic: *mut libc::c_void, oldval: *mut libc::c_void, newval: *mut libc::c_void) -> libc::c_int;
}


/*
gssize g_atomic_pointer_add() [long]
	(volatile void *) atomic
	(gssize) val [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_atomic_pointer_add(atomic: *mut libc::c_void, val: libc::c_long) -> libc::c_long;
}


/*
gsize g_atomic_pointer_and() [unsigned long]
	(volatile void *) atomic
	(gsize) val [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_atomic_pointer_and(atomic: *mut libc::c_void, val: libc::c_ulong) -> libc::c_ulong;
}


/*
gsize g_atomic_pointer_or() [unsigned long]
	(volatile void *) atomic
	(gsize) val [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_atomic_pointer_or(atomic: *mut libc::c_void, val: libc::c_ulong) -> libc::c_ulong;
}


/*
gsize g_atomic_pointer_xor() [unsigned long]
	(volatile void *) atomic
	(gsize) val [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_atomic_pointer_xor(atomic: *mut libc::c_void, val: libc::c_ulong) -> libc::c_ulong;
}


/*
gint g_atomic_int_exchange_and_add() [int]
	(volatile gint *) atomic [volatile int *]
	(gint) val [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_atomic_int_exchange_and_add(atomic: *mut libc::c_int, val: libc::c_int) -> libc::c_int;
}


/*
GQuark g_quark_try_string() [unsigned int]
	(const gchar *) string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_quark_try_string(string: *const libc::c_char) -> libc::c_uint;
}


/*
GQuark g_quark_from_static_string() [unsigned int]
	(const gchar *) string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_quark_from_static_string(string: *const libc::c_char) -> libc::c_uint;
}


/*
GQuark g_quark_from_string() [unsigned int]
	(const gchar *) string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_quark_from_string(string: *const libc::c_char) -> libc::c_uint;
}


/*
const gchar * g_quark_to_string() [const char *]
	(GQuark) quark [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_quark_to_string(quark: libc::c_uint) -> *const libc::c_char;
}


/*
const gchar * g_intern_string() [const char *]
	(const gchar *) string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_intern_string(string: *const libc::c_char) -> *const libc::c_char;
}


/*
const gchar * g_intern_static_string() [const char *]
	(const gchar *) string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_intern_static_string(string: *const libc::c_char) -> *const libc::c_char;
}


/*
GError * g_error_new() [struct _GError *]
	(GQuark) domain [unsigned int]
	(gint) code [int]
	(const gchar *) format [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_error_new(domain: libc::c_uint, code: libc::c_int, format: *const libc::c_char) -> *mut _GError;
}


/*
GError * g_error_new_literal() [struct _GError *]
	(GQuark) domain [unsigned int]
	(gint) code [int]
	(const gchar *) message [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_error_new_literal(domain: libc::c_uint, code: libc::c_int, message: *const libc::c_char) -> *mut _GError;
}


/*
GError * g_error_new_valist() [struct _GError *]
	(GQuark) domain [unsigned int]
	(gint) code [int]
	(const gchar *) format [const char *]
	(va_list) args [struct __va_list_tag [1]]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_error_new_valist(domain: libc::c_uint, code: libc::c_int, format: *const libc::c_char, args: [__va_list_tag; 1]) -> *mut _GError;
}


/*
void g_error_free()
	(GError *) error [struct _GError *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_error_free(error: *mut _GError);
}


/*
GError * g_error_copy() [struct _GError *]
	(const GError *) error [const struct _GError *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_error_copy(error: *const _GError) -> *mut _GError;
}


/*
gboolean g_error_matches() [int]
	(const GError *) error [const struct _GError *]
	(GQuark) domain [unsigned int]
	(gint) code [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_error_matches(error: *const _GError, domain: libc::c_uint, code: libc::c_int) -> libc::c_int;
}


/*
void g_set_error()
	(GError **) err [struct _GError **]
	(GQuark) domain [unsigned int]
	(gint) code [int]
	(const gchar *) format [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_set_error(err: *mut *mut _GError, domain: libc::c_uint, code: libc::c_int, format: *const libc::c_char);
}


/*
void g_set_error_literal()
	(GError **) err [struct _GError **]
	(GQuark) domain [unsigned int]
	(gint) code [int]
	(const gchar *) message [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_set_error_literal(err: *mut *mut _GError, domain: libc::c_uint, code: libc::c_int, message: *const libc::c_char);
}


/*
void g_propagate_error()
	(GError **) dest [struct _GError **]
	(GError *) src [struct _GError *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_propagate_error(dest: *mut *mut _GError, src: *mut _GError);
}


/*
void g_clear_error()
	(GError **) err [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_clear_error(err: *mut *mut _GError);
}


/*
void g_prefix_error()
	(GError **) err [struct _GError **]
	(const gchar *) format [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_prefix_error(err: *mut *mut _GError, format: *const libc::c_char);
}


/*
void g_propagate_prefixed_error()
	(GError **) dest [struct _GError **]
	(GError *) src [struct _GError *]
	(const gchar *) format [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_propagate_prefixed_error(dest: *mut *mut _GError, src: *mut _GError, format: *const libc::c_char);
}


/*
GQuark g_thread_error_quark() [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_error_quark() -> libc::c_uint;
}


/*
GThread * g_thread_ref() [struct _GThread *]
	(GThread *) thread [struct _GThread *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_ref(thread: *mut _GThread) -> *mut _GThread;
}


/*
void g_thread_unref()
	(GThread *) thread [struct _GThread *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_unref(thread: *mut _GThread);
}


/*
GThread * g_thread_new() [struct _GThread *]
	(const gchar *) name [const char *]
	(GThreadFunc) func [void *(*)(void *)]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_new(name: *const libc::c_char, func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>, data: *mut libc::c_void) -> *mut _GThread;
}


/*
GThread * g_thread_try_new() [struct _GThread *]
	(const gchar *) name [const char *]
	(GThreadFunc) func [void *(*)(void *)]
	(gpointer) data [void *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_try_new(name: *const libc::c_char, func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>, data: *mut libc::c_void, error: *mut *mut _GError) -> *mut _GThread;
}


/*
GThread * g_thread_self() [struct _GThread *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_self() -> *mut _GThread;
}


/*
void g_thread_exit()
	(gpointer) retval [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_exit(retval: *mut libc::c_void);
}


/*
gpointer g_thread_join() [void *]
	(GThread *) thread [struct _GThread *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_join(thread: *mut _GThread) -> *mut libc::c_void;
}


/*
void g_thread_yield()
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_yield();
}


/*
void g_mutex_init()
	(GMutex *) mutex [union _GMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mutex_init(mutex: *mut union _GMutex);
}


/*
void g_mutex_clear()
	(GMutex *) mutex [union _GMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mutex_clear(mutex: *mut union _GMutex);
}


/*
void g_mutex_lock()
	(GMutex *) mutex [union _GMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mutex_lock(mutex: *mut union _GMutex);
}


/*
gboolean g_mutex_trylock() [int]
	(GMutex *) mutex [union _GMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mutex_trylock(mutex: *mut union _GMutex) -> libc::c_int;
}


/*
void g_mutex_unlock()
	(GMutex *) mutex [union _GMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mutex_unlock(mutex: *mut union _GMutex);
}


/*
void g_rw_lock_init()
	(GRWLock *) rw_lock [struct _GRWLock *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rw_lock_init(rw_lock: *mut _GRWLock);
}


/*
void g_rw_lock_clear()
	(GRWLock *) rw_lock [struct _GRWLock *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rw_lock_clear(rw_lock: *mut _GRWLock);
}


/*
void g_rw_lock_writer_lock()
	(GRWLock *) rw_lock [struct _GRWLock *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rw_lock_writer_lock(rw_lock: *mut _GRWLock);
}


/*
gboolean g_rw_lock_writer_trylock() [int]
	(GRWLock *) rw_lock [struct _GRWLock *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rw_lock_writer_trylock(rw_lock: *mut _GRWLock) -> libc::c_int;
}


/*
void g_rw_lock_writer_unlock()
	(GRWLock *) rw_lock [struct _GRWLock *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rw_lock_writer_unlock(rw_lock: *mut _GRWLock);
}


/*
void g_rw_lock_reader_lock()
	(GRWLock *) rw_lock [struct _GRWLock *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rw_lock_reader_lock(rw_lock: *mut _GRWLock);
}


/*
gboolean g_rw_lock_reader_trylock() [int]
	(GRWLock *) rw_lock [struct _GRWLock *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rw_lock_reader_trylock(rw_lock: *mut _GRWLock) -> libc::c_int;
}


/*
void g_rw_lock_reader_unlock()
	(GRWLock *) rw_lock [struct _GRWLock *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rw_lock_reader_unlock(rw_lock: *mut _GRWLock);
}


/*
void g_rec_mutex_init()
	(GRecMutex *) rec_mutex [struct _GRecMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rec_mutex_init(rec_mutex: *mut _GRecMutex);
}


/*
void g_rec_mutex_clear()
	(GRecMutex *) rec_mutex [struct _GRecMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rec_mutex_clear(rec_mutex: *mut _GRecMutex);
}


/*
void g_rec_mutex_lock()
	(GRecMutex *) rec_mutex [struct _GRecMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rec_mutex_lock(rec_mutex: *mut _GRecMutex);
}


/*
gboolean g_rec_mutex_trylock() [int]
	(GRecMutex *) rec_mutex [struct _GRecMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rec_mutex_trylock(rec_mutex: *mut _GRecMutex) -> libc::c_int;
}


/*
void g_rec_mutex_unlock()
	(GRecMutex *) rec_mutex [struct _GRecMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rec_mutex_unlock(rec_mutex: *mut _GRecMutex);
}


/*
void g_cond_init()
	(GCond *) cond [struct _GCond *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_cond_init(cond: *mut _GCond);
}


/*
void g_cond_clear()
	(GCond *) cond [struct _GCond *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_cond_clear(cond: *mut _GCond);
}


/*
void g_cond_wait()
	(GCond *) cond [struct _GCond *]
	(GMutex *) mutex [union _GMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_cond_wait(cond: *mut _GCond, mutex: *mut union _GMutex);
}


/*
void g_cond_signal()
	(GCond *) cond [struct _GCond *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_cond_signal(cond: *mut _GCond);
}


/*
void g_cond_broadcast()
	(GCond *) cond [struct _GCond *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_cond_broadcast(cond: *mut _GCond);
}


/*
gboolean g_cond_wait_until() [int]
	(GCond *) cond [struct _GCond *]
	(GMutex *) mutex [union _GMutex *]
	(gint64) end_time [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_cond_wait_until(cond: *mut _GCond, mutex: *mut union _GMutex, end_time: libc::c_long) -> libc::c_int;
}


/*
gpointer g_private_get() [void *]
	(GPrivate *) key [struct _GPrivate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_private_get(key: *mut _GPrivate) -> *mut libc::c_void;
}


/*
void g_private_set()
	(GPrivate *) key [struct _GPrivate *]
	(gpointer) value [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_private_set(key: *mut _GPrivate, value: *mut libc::c_void);
}


/*
void g_private_replace()
	(GPrivate *) key [struct _GPrivate *]
	(gpointer) value [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_private_replace(key: *mut _GPrivate, value: *mut libc::c_void);
}


/*
gpointer g_once_impl() [void *]
	(GOnce *) once [struct _GOnce *]
	(GThreadFunc) func [void *(*)(void *)]
	(gpointer) arg [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_once_impl(once: *mut _GOnce, func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>, arg: *mut libc::c_void) -> *mut libc::c_void;
}


/*
gboolean g_once_init_enter() [int]
	(volatile void *) location
*/
#[link(name="nice")]
extern "C" {
	pub fn g_once_init_enter(location: *mut libc::c_void) -> libc::c_int;
}


/*
void g_once_init_leave()
	(volatile void *) location
	(gsize) result [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_once_init_leave(location: *mut libc::c_void, result: libc::c_ulong);
}


/*
GAsyncQueue * g_async_queue_new() [struct _GAsyncQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_new() -> *mut _GAsyncQueue;
}


/*
GAsyncQueue * g_async_queue_new_full() [struct _GAsyncQueue *]
	(GDestroyNotify) item_free_func [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_new_full(item_free_func: Option<extern fn(*mut libc::c_void)>) -> *mut _GAsyncQueue;
}


/*
void g_async_queue_lock()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_lock(queue: *mut _GAsyncQueue);
}


/*
void g_async_queue_unlock()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_unlock(queue: *mut _GAsyncQueue);
}


/*
GAsyncQueue * g_async_queue_ref() [struct _GAsyncQueue *]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_ref(queue: *mut _GAsyncQueue) -> *mut _GAsyncQueue;
}


/*
void g_async_queue_unref()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_unref(queue: *mut _GAsyncQueue);
}


/*
void g_async_queue_ref_unlocked()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_ref_unlocked(queue: *mut _GAsyncQueue);
}


/*
void g_async_queue_unref_and_unlock()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_unref_and_unlock(queue: *mut _GAsyncQueue);
}


/*
void g_async_queue_push()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_push(queue: *mut _GAsyncQueue, data: *mut libc::c_void);
}


/*
void g_async_queue_push_unlocked()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_push_unlocked(queue: *mut _GAsyncQueue, data: *mut libc::c_void);
}


/*
void g_async_queue_push_sorted()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(gpointer) data [void *]
	(GCompareDataFunc) func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_push_sorted(queue: *mut _GAsyncQueue, data: *mut libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
void g_async_queue_push_sorted_unlocked()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(gpointer) data [void *]
	(GCompareDataFunc) func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_push_sorted_unlocked(queue: *mut _GAsyncQueue, data: *mut libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
gpointer g_async_queue_pop() [void *]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_pop(queue: *mut _GAsyncQueue) -> *mut libc::c_void;
}


/*
gpointer g_async_queue_pop_unlocked() [void *]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_pop_unlocked(queue: *mut _GAsyncQueue) -> *mut libc::c_void;
}


/*
gpointer g_async_queue_try_pop() [void *]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_try_pop(queue: *mut _GAsyncQueue) -> *mut libc::c_void;
}


/*
gpointer g_async_queue_try_pop_unlocked() [void *]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_try_pop_unlocked(queue: *mut _GAsyncQueue) -> *mut libc::c_void;
}


/*
gpointer g_async_queue_timeout_pop() [void *]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(guint64) timeout [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_timeout_pop(queue: *mut _GAsyncQueue, timeout: libc::c_ulong) -> *mut libc::c_void;
}


/*
gpointer g_async_queue_timeout_pop_unlocked() [void *]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(guint64) timeout [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_timeout_pop_unlocked(queue: *mut _GAsyncQueue, timeout: libc::c_ulong) -> *mut libc::c_void;
}


/*
gint g_async_queue_length() [int]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_length(queue: *mut _GAsyncQueue) -> libc::c_int;
}


/*
gint g_async_queue_length_unlocked() [int]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_length_unlocked(queue: *mut _GAsyncQueue) -> libc::c_int;
}


/*
void g_async_queue_sort()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(GCompareDataFunc) func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_sort(queue: *mut _GAsyncQueue, func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
void g_async_queue_sort_unlocked()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(GCompareDataFunc) func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_sort_unlocked(queue: *mut _GAsyncQueue, func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
gpointer g_async_queue_timed_pop() [void *]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(GTimeVal *) end_time [struct _GTimeVal *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_timed_pop(queue: *mut _GAsyncQueue, end_time: *mut _GTimeVal) -> *mut libc::c_void;
}


/*
gpointer g_async_queue_timed_pop_unlocked() [void *]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(GTimeVal *) end_time [struct _GTimeVal *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_async_queue_timed_pop_unlocked(queue: *mut _GAsyncQueue, end_time: *mut _GTimeVal) -> *mut libc::c_void;
}


/*
void g_on_error_query()
	(const gchar *) prg_name [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_on_error_query(prg_name: *const libc::c_char);
}


/*
void g_on_error_stack_trace()
	(const gchar *) prg_name [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_on_error_stack_trace(prg_name: *const libc::c_char);
}


/*
gsize g_base64_encode_step() [unsigned long]
	(const guchar *) in [const unsigned char *]
	(gsize) len [unsigned long]
	(gboolean) break_lines [int]
	(gchar *) out [char *]
	(gint *) state [int *]
	(gint *) save [int *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_base64_encode_step(in_: *const libc::c_uchar, len: libc::c_ulong, break_lines: libc::c_int, out: *mut libc::c_char, state: *mut libc::c_int, save: *mut libc::c_int) -> libc::c_ulong;
}


/*
gsize g_base64_encode_close() [unsigned long]
	(gboolean) break_lines [int]
	(gchar *) out [char *]
	(gint *) state [int *]
	(gint *) save [int *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_base64_encode_close(break_lines: libc::c_int, out: *mut libc::c_char, state: *mut libc::c_int, save: *mut libc::c_int) -> libc::c_ulong;
}


/*
gchar * g_base64_encode() [char *]
	(const guchar *) data [const unsigned char *]
	(gsize) len [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_base64_encode(data: *const libc::c_uchar, len: libc::c_ulong) -> *mut libc::c_char;
}


/*
gsize g_base64_decode_step() [unsigned long]
	(const gchar *) in [const char *]
	(gsize) len [unsigned long]
	(guchar *) out [unsigned char *]
	(gint *) state [int *]
	(guint *) save [unsigned int *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_base64_decode_step(in_: *const libc::c_char, len: libc::c_ulong, out: *mut libc::c_uchar, state: *mut libc::c_int, save: *mut libc::c_uint) -> libc::c_ulong;
}


/*
guchar * g_base64_decode() [unsigned char *]
	(const gchar *) text [const char *]
	(gsize *) out_len [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_base64_decode(text: *const libc::c_char, out_len: *mut libc::c_ulong) -> *mut libc::c_uchar;
}


/*
guchar * g_base64_decode_inplace() [unsigned char *]
	(gchar *) text [char *]
	(gsize *) out_len [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_base64_decode_inplace(text: *mut libc::c_char, out_len: *mut libc::c_ulong) -> *mut libc::c_uchar;
}


/*
void g_bit_lock()
	(volatile gint *) address [volatile int *]
	(gint) lock_bit [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bit_lock(address: *mut libc::c_int, lock_bit: libc::c_int);
}


/*
gboolean g_bit_trylock() [int]
	(volatile gint *) address [volatile int *]
	(gint) lock_bit [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bit_trylock(address: *mut libc::c_int, lock_bit: libc::c_int) -> libc::c_int;
}


/*
void g_bit_unlock()
	(volatile gint *) address [volatile int *]
	(gint) lock_bit [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bit_unlock(address: *mut libc::c_int, lock_bit: libc::c_int);
}


/*
void g_pointer_bit_lock()
	(volatile void *) address
	(gint) lock_bit [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_pointer_bit_lock(address: *mut libc::c_void, lock_bit: libc::c_int);
}


/*
gboolean g_pointer_bit_trylock() [int]
	(volatile void *) address
	(gint) lock_bit [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_pointer_bit_trylock(address: *mut libc::c_void, lock_bit: libc::c_int) -> libc::c_int;
}


/*
void g_pointer_bit_unlock()
	(volatile void *) address
	(gint) lock_bit [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_pointer_bit_unlock(address: *mut libc::c_void, lock_bit: libc::c_int);
}


/*
GQuark g_bookmark_file_error_quark() [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_error_quark() -> libc::c_uint;
}


/*
GBookmarkFile * g_bookmark_file_new() [struct _GBookmarkFile *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_new() -> *mut _GBookmarkFile;
}


/*
void g_bookmark_file_free()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_free(bookmark: *mut _GBookmarkFile);
}


/*
gboolean g_bookmark_file_load_from_file() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) filename [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_load_from_file(bookmark: *mut _GBookmarkFile, filename: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_bookmark_file_load_from_data() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) data [const char *]
	(gsize) length [unsigned long]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_load_from_data(bookmark: *mut _GBookmarkFile, data: *const libc::c_char, length: libc::c_ulong, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_bookmark_file_load_from_data_dirs() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) file [const char *]
	(gchar **) full_path [char **]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_load_from_data_dirs(bookmark: *mut _GBookmarkFile, file: *const libc::c_char, full_path: *mut *mut libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
gchar * g_bookmark_file_to_data() [char *]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_to_data(bookmark: *mut _GBookmarkFile, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gboolean g_bookmark_file_to_file() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) filename [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_to_file(bookmark: *mut _GBookmarkFile, filename: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_bookmark_file_set_title()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) title [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_set_title(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, title: *const libc::c_char);
}


/*
gchar * g_bookmark_file_get_title() [char *]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_get_title(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
void g_bookmark_file_set_description()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) description [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_set_description(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, description: *const libc::c_char);
}


/*
gchar * g_bookmark_file_get_description() [char *]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_get_description(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
void g_bookmark_file_set_mime_type()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) mime_type [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_set_mime_type(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, mime_type: *const libc::c_char);
}


/*
gchar * g_bookmark_file_get_mime_type() [char *]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_get_mime_type(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
void g_bookmark_file_set_groups()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar **) groups [const char **]
	(gsize) length [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_set_groups(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, groups: *mut *const libc::c_char, length: libc::c_ulong);
}


/*
void g_bookmark_file_add_group()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) group [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_add_group(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, group: *const libc::c_char);
}


/*
gboolean g_bookmark_file_has_group() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) group [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_has_group(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, group: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
gchar ** g_bookmark_file_get_groups() [char **]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_get_groups(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut *mut libc::c_char;
}


/*
void g_bookmark_file_add_application()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) name [const char *]
	(const gchar *) exec [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_add_application(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, name: *const libc::c_char, exec: *const libc::c_char);
}


/*
gboolean g_bookmark_file_has_application() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) name [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_has_application(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, name: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
gchar ** g_bookmark_file_get_applications() [char **]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_get_applications(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut *mut libc::c_char;
}


/*
gboolean g_bookmark_file_set_app_info() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) name [const char *]
	(const gchar *) exec [const char *]
	(gint) count [int]
	(time_t) stamp [long]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_set_app_info(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, name: *const libc::c_char, exec: *const libc::c_char, count: libc::c_int, stamp: libc::c_long, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_bookmark_file_get_app_info() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) name [const char *]
	(gchar **) exec [char **]
	(guint *) count [unsigned int *]
	(time_t *) stamp [long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_get_app_info(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, name: *const libc::c_char, exec: *mut *mut libc::c_char, count: *mut libc::c_uint, stamp: *mut libc::c_long, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_bookmark_file_set_is_private()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(gboolean) is_private [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_set_is_private(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, is_private: libc::c_int);
}


/*
gboolean g_bookmark_file_get_is_private() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_get_is_private(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_bookmark_file_set_icon()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) href [const char *]
	(const gchar *) mime_type [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_set_icon(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, href: *const libc::c_char, mime_type: *const libc::c_char);
}


/*
gboolean g_bookmark_file_get_icon() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(gchar **) href [char **]
	(gchar **) mime_type [char **]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_get_icon(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, href: *mut *mut libc::c_char, mime_type: *mut *mut libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_bookmark_file_set_added()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(time_t) added [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_set_added(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, added: libc::c_long);
}


/*
time_t g_bookmark_file_get_added() [long]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_get_added(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, error: *mut *mut _GError) -> libc::c_long;
}


/*
void g_bookmark_file_set_modified()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(time_t) modified [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_set_modified(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, modified: libc::c_long);
}


/*
time_t g_bookmark_file_get_modified() [long]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_get_modified(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, error: *mut *mut _GError) -> libc::c_long;
}


/*
void g_bookmark_file_set_visited()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(time_t) visited [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_set_visited(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, visited: libc::c_long);
}


/*
time_t g_bookmark_file_get_visited() [long]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_get_visited(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, error: *mut *mut _GError) -> libc::c_long;
}


/*
gboolean g_bookmark_file_has_item() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_has_item(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char) -> libc::c_int;
}


/*
gint g_bookmark_file_get_size() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_get_size(bookmark: *mut _GBookmarkFile) -> libc::c_int;
}


/*
gchar ** g_bookmark_file_get_uris() [char **]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(gsize *) length [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_get_uris(bookmark: *mut _GBookmarkFile, length: *mut libc::c_ulong) -> *mut *mut libc::c_char;
}


/*
gboolean g_bookmark_file_remove_group() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) group [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_remove_group(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, group: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_bookmark_file_remove_application() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) name [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_remove_application(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, name: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_bookmark_file_remove_item() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_remove_item(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_bookmark_file_move_item() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) old_uri [const char *]
	(const gchar *) new_uri [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bookmark_file_move_item(bookmark: *mut _GBookmarkFile, old_uri: *const libc::c_char, new_uri: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
GBytes * g_bytes_new() [struct _GBytes *]
	(gconstpointer) data [const void *]
	(gsize) size [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bytes_new(data: *const libc::c_void, size: libc::c_ulong) -> *mut _GBytes;
}


/*
GBytes * g_bytes_new_take() [struct _GBytes *]
	(gpointer) data [void *]
	(gsize) size [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bytes_new_take(data: *mut libc::c_void, size: libc::c_ulong) -> *mut _GBytes;
}


/*
GBytes * g_bytes_new_static() [struct _GBytes *]
	(gconstpointer) data [const void *]
	(gsize) size [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bytes_new_static(data: *const libc::c_void, size: libc::c_ulong) -> *mut _GBytes;
}


/*
GBytes * g_bytes_new_with_free_func() [struct _GBytes *]
	(gconstpointer) data [const void *]
	(gsize) size [unsigned long]
	(GDestroyNotify) free_func [void (*)(void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bytes_new_with_free_func(data: *const libc::c_void, size: libc::c_ulong, free_func: Option<extern fn(*mut libc::c_void)>, user_data: *mut libc::c_void) -> *mut _GBytes;
}


/*
GBytes * g_bytes_new_from_bytes() [struct _GBytes *]
	(GBytes *) bytes [struct _GBytes *]
	(gsize) offset [unsigned long]
	(gsize) length [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bytes_new_from_bytes(bytes: *mut _GBytes, offset: libc::c_ulong, length: libc::c_ulong) -> *mut _GBytes;
}


/*
gconstpointer g_bytes_get_data() [const void *]
	(GBytes *) bytes [struct _GBytes *]
	(gsize *) size [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bytes_get_data(bytes: *mut _GBytes, size: *mut libc::c_ulong) -> *const libc::c_void;
}


/*
gsize g_bytes_get_size() [unsigned long]
	(GBytes *) bytes [struct _GBytes *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bytes_get_size(bytes: *mut _GBytes) -> libc::c_ulong;
}


/*
GBytes * g_bytes_ref() [struct _GBytes *]
	(GBytes *) bytes [struct _GBytes *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bytes_ref(bytes: *mut _GBytes) -> *mut _GBytes;
}


/*
void g_bytes_unref()
	(GBytes *) bytes [struct _GBytes *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bytes_unref(bytes: *mut _GBytes);
}


/*
gpointer g_bytes_unref_to_data() [void *]
	(GBytes *) bytes [struct _GBytes *]
	(gsize *) size [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bytes_unref_to_data(bytes: *mut _GBytes, size: *mut libc::c_ulong) -> *mut libc::c_void;
}


/*
GByteArray * g_bytes_unref_to_array() [struct _GByteArray *]
	(GBytes *) bytes [struct _GBytes *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bytes_unref_to_array(bytes: *mut _GBytes) -> *mut _GByteArray;
}


/*
guint g_bytes_hash() [unsigned int]
	(gconstpointer) bytes [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bytes_hash(bytes: *const libc::c_void) -> libc::c_uint;
}


/*
gboolean g_bytes_equal() [int]
	(gconstpointer) bytes1 [const void *]
	(gconstpointer) bytes2 [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bytes_equal(bytes1: *const libc::c_void, bytes2: *const libc::c_void) -> libc::c_int;
}


/*
gint g_bytes_compare() [int]
	(gconstpointer) bytes1 [const void *]
	(gconstpointer) bytes2 [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bytes_compare(bytes1: *const libc::c_void, bytes2: *const libc::c_void) -> libc::c_int;
}


/*
gboolean g_get_charset() [int]
	(const char **) charset
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_charset(charset: *mut *const libc::c_char) -> libc::c_int;
}


/*
gchar * g_get_codeset() [char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_codeset() -> *mut libc::c_char;
}


/*
const gchar *const * g_get_language_names() [const char *const *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_language_names() -> *const *const libc::c_char;
}


/*
gchar ** g_get_locale_variants() [char **]
	(const gchar *) locale [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_locale_variants(locale: *const libc::c_char) -> *mut *mut libc::c_char;
}


/*
gssize g_checksum_type_get_length() [long]
	(GChecksumType) checksum_type [GChecksumType]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_checksum_type_get_length(checksum_type: libc::c_uint) -> libc::c_long;
}


/*
GChecksum * g_checksum_new() [struct _GChecksum *]
	(GChecksumType) checksum_type [GChecksumType]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_checksum_new(checksum_type: libc::c_uint) -> *mut _GChecksum;
}


/*
void g_checksum_reset()
	(GChecksum *) checksum [struct _GChecksum *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_checksum_reset(checksum: *mut _GChecksum);
}


/*
GChecksum * g_checksum_copy() [struct _GChecksum *]
	(const GChecksum *) checksum [const struct _GChecksum *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_checksum_copy(checksum: *const _GChecksum) -> *mut _GChecksum;
}


/*
void g_checksum_free()
	(GChecksum *) checksum [struct _GChecksum *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_checksum_free(checksum: *mut _GChecksum);
}


/*
void g_checksum_update()
	(GChecksum *) checksum [struct _GChecksum *]
	(const guchar *) data [const unsigned char *]
	(gssize) length [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_checksum_update(checksum: *mut _GChecksum, data: *const libc::c_uchar, length: libc::c_long);
}


/*
const gchar * g_checksum_get_string() [const char *]
	(GChecksum *) checksum [struct _GChecksum *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_checksum_get_string(checksum: *mut _GChecksum) -> *const libc::c_char;
}


/*
void g_checksum_get_digest()
	(GChecksum *) checksum [struct _GChecksum *]
	(guint8 *) buffer [unsigned char *]
	(gsize *) digest_len [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_checksum_get_digest(checksum: *mut _GChecksum, buffer: *mut libc::c_uchar, digest_len: *mut libc::c_ulong);
}


/*
gchar * g_compute_checksum_for_data() [char *]
	(GChecksumType) checksum_type [GChecksumType]
	(const guchar *) data [const unsigned char *]
	(gsize) length [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_compute_checksum_for_data(checksum_type: libc::c_uint, data: *const libc::c_uchar, length: libc::c_ulong) -> *mut libc::c_char;
}


/*
gchar * g_compute_checksum_for_string() [char *]
	(GChecksumType) checksum_type [GChecksumType]
	(const gchar *) str [const char *]
	(gssize) length [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_compute_checksum_for_string(checksum_type: libc::c_uint, str: *const libc::c_char, length: libc::c_long) -> *mut libc::c_char;
}


/*
GQuark g_convert_error_quark() [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_convert_error_quark() -> libc::c_uint;
}


/*
GIConv g_iconv_open() [struct _GIConv *]
	(const gchar *) to_codeset [const char *]
	(const gchar *) from_codeset [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_iconv_open(to_codeset: *const libc::c_char, from_codeset: *const libc::c_char) -> *mut _GIConv;
}


/*
gsize g_iconv() [unsigned long]
	(GIConv) converter [struct _GIConv *]
	(gchar **) inbuf [char **]
	(gsize *) inbytes_left [unsigned long *]
	(gchar **) outbuf [char **]
	(gsize *) outbytes_left [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_iconv(converter: *mut _GIConv, inbuf: *mut *mut libc::c_char, inbytes_left: *mut libc::c_ulong, outbuf: *mut *mut libc::c_char, outbytes_left: *mut libc::c_ulong) -> libc::c_ulong;
}


/*
gint g_iconv_close() [int]
	(GIConv) converter [struct _GIConv *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_iconv_close(converter: *mut _GIConv) -> libc::c_int;
}


/*
gchar * g_convert() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
	(const gchar *) to_codeset [const char *]
	(const gchar *) from_codeset [const char *]
	(gsize *) bytes_read [unsigned long *]
	(gsize *) bytes_written [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_convert(str: *const libc::c_char, len: libc::c_long, to_codeset: *const libc::c_char, from_codeset: *const libc::c_char, bytes_read: *mut libc::c_ulong, bytes_written: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gchar * g_convert_with_iconv() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
	(GIConv) converter [struct _GIConv *]
	(gsize *) bytes_read [unsigned long *]
	(gsize *) bytes_written [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_convert_with_iconv(str: *const libc::c_char, len: libc::c_long, converter: *mut _GIConv, bytes_read: *mut libc::c_ulong, bytes_written: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gchar * g_convert_with_fallback() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
	(const gchar *) to_codeset [const char *]
	(const gchar *) from_codeset [const char *]
	(const gchar *) fallback [const char *]
	(gsize *) bytes_read [unsigned long *]
	(gsize *) bytes_written [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_convert_with_fallback(str: *const libc::c_char, len: libc::c_long, to_codeset: *const libc::c_char, from_codeset: *const libc::c_char, fallback: *const libc::c_char, bytes_read: *mut libc::c_ulong, bytes_written: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gchar * g_locale_to_utf8() [char *]
	(const gchar *) opsysstring [const char *]
	(gssize) len [long]
	(gsize *) bytes_read [unsigned long *]
	(gsize *) bytes_written [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_locale_to_utf8(opsysstring: *const libc::c_char, len: libc::c_long, bytes_read: *mut libc::c_ulong, bytes_written: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gchar * g_locale_from_utf8() [char *]
	(const gchar *) utf8string [const char *]
	(gssize) len [long]
	(gsize *) bytes_read [unsigned long *]
	(gsize *) bytes_written [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_locale_from_utf8(utf8string: *const libc::c_char, len: libc::c_long, bytes_read: *mut libc::c_ulong, bytes_written: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gchar * g_filename_to_utf8() [char *]
	(const gchar *) opsysstring [const char *]
	(gssize) len [long]
	(gsize *) bytes_read [unsigned long *]
	(gsize *) bytes_written [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_filename_to_utf8(opsysstring: *const libc::c_char, len: libc::c_long, bytes_read: *mut libc::c_ulong, bytes_written: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gchar * g_filename_from_utf8() [char *]
	(const gchar *) utf8string [const char *]
	(gssize) len [long]
	(gsize *) bytes_read [unsigned long *]
	(gsize *) bytes_written [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_filename_from_utf8(utf8string: *const libc::c_char, len: libc::c_long, bytes_read: *mut libc::c_ulong, bytes_written: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gchar * g_filename_from_uri() [char *]
	(const gchar *) uri [const char *]
	(gchar **) hostname [char **]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_filename_from_uri(uri: *const libc::c_char, hostname: *mut *mut libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gchar * g_filename_to_uri() [char *]
	(const gchar *) filename [const char *]
	(const gchar *) hostname [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_filename_to_uri(filename: *const libc::c_char, hostname: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gchar * g_filename_display_name() [char *]
	(const gchar *) filename [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_filename_display_name(filename: *const libc::c_char) -> *mut libc::c_char;
}


/*
gboolean g_get_filename_charsets() [int]
	(const gchar ***) charsets [const char ***]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_filename_charsets(charsets: *mut *mut *const libc::c_char) -> libc::c_int;
}


/*
gchar * g_filename_display_basename() [char *]
	(const gchar *) filename [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_filename_display_basename(filename: *const libc::c_char) -> *mut libc::c_char;
}


/*
gchar ** g_uri_list_extract_uris() [char **]
	(const gchar *) uri_list [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_uri_list_extract_uris(uri_list: *const libc::c_char) -> *mut *mut libc::c_char;
}


/*
void g_datalist_init()
	(GData **) datalist [struct _GData **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_datalist_init(datalist: *mut *mut _GData);
}


/*
void g_datalist_clear()
	(GData **) datalist [struct _GData **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_datalist_clear(datalist: *mut *mut _GData);
}


/*
gpointer g_datalist_id_get_data() [void *]
	(GData **) datalist [struct _GData **]
	(GQuark) key_id [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_datalist_id_get_data(datalist: *mut *mut _GData, key_id: libc::c_uint) -> *mut libc::c_void;
}


/*
void g_datalist_id_set_data_full()
	(GData **) datalist [struct _GData **]
	(GQuark) key_id [unsigned int]
	(gpointer) data [void *]
	(GDestroyNotify) destroy_func [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_datalist_id_set_data_full(datalist: *mut *mut _GData, key_id: libc::c_uint, data: *mut libc::c_void, destroy_func: Option<extern fn(*mut libc::c_void)>);
}


/*
gpointer g_datalist_id_remove_no_notify() [void *]
	(GData **) datalist [struct _GData **]
	(GQuark) key_id [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_datalist_id_remove_no_notify(datalist: *mut *mut _GData, key_id: libc::c_uint) -> *mut libc::c_void;
}


/*
void g_datalist_foreach()
	(GData **) datalist [struct _GData **]
	(GDataForeachFunc) func [void (*)(unsigned int, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_datalist_foreach(datalist: *mut *mut _GData, func: Option<extern fn(libc::c_uint, *mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
void g_datalist_set_flags()
	(GData **) datalist [struct _GData **]
	(guint) flags [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_datalist_set_flags(datalist: *mut *mut _GData, flags: libc::c_uint);
}


/*
void g_datalist_unset_flags()
	(GData **) datalist [struct _GData **]
	(guint) flags [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_datalist_unset_flags(datalist: *mut *mut _GData, flags: libc::c_uint);
}


/*
guint g_datalist_get_flags() [unsigned int]
	(GData **) datalist [struct _GData **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_datalist_get_flags(datalist: *mut *mut _GData) -> libc::c_uint;
}


/*
void g_dataset_destroy()
	(gconstpointer) dataset_location [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_dataset_destroy(dataset_location: *const libc::c_void);
}


/*
gpointer g_dataset_id_get_data() [void *]
	(gconstpointer) dataset_location [const void *]
	(GQuark) key_id [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_dataset_id_get_data(dataset_location: *const libc::c_void, key_id: libc::c_uint) -> *mut libc::c_void;
}


/*
gpointer g_datalist_get_data() [void *]
	(GData **) datalist [struct _GData **]
	(const gchar *) key [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_datalist_get_data(datalist: *mut *mut _GData, key: *const libc::c_char) -> *mut libc::c_void;
}


/*
void g_dataset_id_set_data_full()
	(gconstpointer) dataset_location [const void *]
	(GQuark) key_id [unsigned int]
	(gpointer) data [void *]
	(GDestroyNotify) destroy_func [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_dataset_id_set_data_full(dataset_location: *const libc::c_void, key_id: libc::c_uint, data: *mut libc::c_void, destroy_func: Option<extern fn(*mut libc::c_void)>);
}


/*
gpointer g_dataset_id_remove_no_notify() [void *]
	(gconstpointer) dataset_location [const void *]
	(GQuark) key_id [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_dataset_id_remove_no_notify(dataset_location: *const libc::c_void, key_id: libc::c_uint) -> *mut libc::c_void;
}


/*
void g_dataset_foreach()
	(gconstpointer) dataset_location [const void *]
	(GDataForeachFunc) func [void (*)(unsigned int, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_dataset_foreach(dataset_location: *const libc::c_void, func: Option<extern fn(libc::c_uint, *mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
GDate * g_date_new() [struct _GDate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_new() -> *mut _GDate;
}


/*
GDate * g_date_new_dmy() [struct _GDate *]
	(GDateDay) day [unsigned char]
	(GDateMonth) month [GDateMonth]
	(GDateYear) year [unsigned short]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_new_dmy(day: libc::c_uchar, month: libc::c_uint, year: libc::c_ushort) -> *mut _GDate;
}


/*
GDate * g_date_new_julian() [struct _GDate *]
	(guint32) julian_day [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_new_julian(julian_day: libc::c_uint) -> *mut _GDate;
}


/*
void g_date_free()
	(GDate *) date [struct _GDate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_free(date: *mut _GDate);
}


/*
gboolean g_date_valid() [int]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_valid(date: *const _GDate) -> libc::c_int;
}


/*
gboolean g_date_valid_day() [int]
	(GDateDay) day [unsigned char]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_valid_day(day: libc::c_uchar) -> libc::c_int;
}


/*
gboolean g_date_valid_month() [int]
	(GDateMonth) month [GDateMonth]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_valid_month(month: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_date_valid_year() [int]
	(GDateYear) year [unsigned short]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_valid_year(year: libc::c_ushort) -> libc::c_int;
}


/*
gboolean g_date_valid_weekday() [int]
	(GDateWeekday) weekday [GDateWeekday]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_valid_weekday(weekday: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_date_valid_julian() [int]
	(guint32) julian_date [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_valid_julian(julian_date: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_date_valid_dmy() [int]
	(GDateDay) day [unsigned char]
	(GDateMonth) month [GDateMonth]
	(GDateYear) year [unsigned short]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_valid_dmy(day: libc::c_uchar, month: libc::c_uint, year: libc::c_ushort) -> libc::c_int;
}


/*
GDateWeekday g_date_get_weekday() [GDateWeekday]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_get_weekday(date: *const _GDate) -> libc::c_uint;
}


/*
GDateMonth g_date_get_month() [GDateMonth]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_get_month(date: *const _GDate) -> libc::c_uint;
}


/*
GDateYear g_date_get_year() [unsigned short]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_get_year(date: *const _GDate) -> libc::c_ushort;
}


/*
GDateDay g_date_get_day() [unsigned char]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_get_day(date: *const _GDate) -> libc::c_uchar;
}


/*
guint32 g_date_get_julian() [unsigned int]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_get_julian(date: *const _GDate) -> libc::c_uint;
}


/*
guint g_date_get_day_of_year() [unsigned int]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_get_day_of_year(date: *const _GDate) -> libc::c_uint;
}


/*
guint g_date_get_monday_week_of_year() [unsigned int]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_get_monday_week_of_year(date: *const _GDate) -> libc::c_uint;
}


/*
guint g_date_get_sunday_week_of_year() [unsigned int]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_get_sunday_week_of_year(date: *const _GDate) -> libc::c_uint;
}


/*
guint g_date_get_iso8601_week_of_year() [unsigned int]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_get_iso8601_week_of_year(date: *const _GDate) -> libc::c_uint;
}


/*
void g_date_clear()
	(GDate *) date [struct _GDate *]
	(guint) n_dates [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_clear(date: *mut _GDate, n_dates: libc::c_uint);
}


/*
void g_date_set_parse()
	(GDate *) date [struct _GDate *]
	(const gchar *) str [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_set_parse(date: *mut _GDate, str: *const libc::c_char);
}


/*
void g_date_set_time_t()
	(GDate *) date [struct _GDate *]
	(time_t) timet [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_set_time_t(date: *mut _GDate, timet: libc::c_long);
}


/*
void g_date_set_time_val()
	(GDate *) date [struct _GDate *]
	(GTimeVal *) timeval [struct _GTimeVal *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_set_time_val(date: *mut _GDate, timeval: *mut _GTimeVal);
}


/*
void g_date_set_time()
	(GDate *) date [struct _GDate *]
	(GTime) time_ [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_set_time(date: *mut _GDate, time_: libc::c_int);
}


/*
void g_date_set_month()
	(GDate *) date [struct _GDate *]
	(GDateMonth) month [GDateMonth]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_set_month(date: *mut _GDate, month: libc::c_uint);
}


/*
void g_date_set_day()
	(GDate *) date [struct _GDate *]
	(GDateDay) day [unsigned char]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_set_day(date: *mut _GDate, day: libc::c_uchar);
}


/*
void g_date_set_year()
	(GDate *) date [struct _GDate *]
	(GDateYear) year [unsigned short]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_set_year(date: *mut _GDate, year: libc::c_ushort);
}


/*
void g_date_set_dmy()
	(GDate *) date [struct _GDate *]
	(GDateDay) day [unsigned char]
	(GDateMonth) month [GDateMonth]
	(GDateYear) y [unsigned short]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_set_dmy(date: *mut _GDate, day: libc::c_uchar, month: libc::c_uint, y: libc::c_ushort);
}


/*
void g_date_set_julian()
	(GDate *) date [struct _GDate *]
	(guint32) julian_date [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_set_julian(date: *mut _GDate, julian_date: libc::c_uint);
}


/*
gboolean g_date_is_first_of_month() [int]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_is_first_of_month(date: *const _GDate) -> libc::c_int;
}


/*
gboolean g_date_is_last_of_month() [int]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_is_last_of_month(date: *const _GDate) -> libc::c_int;
}


/*
void g_date_add_days()
	(GDate *) date [struct _GDate *]
	(guint) n_days [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_add_days(date: *mut _GDate, n_days: libc::c_uint);
}


/*
void g_date_subtract_days()
	(GDate *) date [struct _GDate *]
	(guint) n_days [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_subtract_days(date: *mut _GDate, n_days: libc::c_uint);
}


/*
void g_date_add_months()
	(GDate *) date [struct _GDate *]
	(guint) n_months [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_add_months(date: *mut _GDate, n_months: libc::c_uint);
}


/*
void g_date_subtract_months()
	(GDate *) date [struct _GDate *]
	(guint) n_months [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_subtract_months(date: *mut _GDate, n_months: libc::c_uint);
}


/*
void g_date_add_years()
	(GDate *) date [struct _GDate *]
	(guint) n_years [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_add_years(date: *mut _GDate, n_years: libc::c_uint);
}


/*
void g_date_subtract_years()
	(GDate *) date [struct _GDate *]
	(guint) n_years [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_subtract_years(date: *mut _GDate, n_years: libc::c_uint);
}


/*
gboolean g_date_is_leap_year() [int]
	(GDateYear) year [unsigned short]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_is_leap_year(year: libc::c_ushort) -> libc::c_int;
}


/*
guint8 g_date_get_days_in_month() [unsigned char]
	(GDateMonth) month [GDateMonth]
	(GDateYear) year [unsigned short]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_get_days_in_month(month: libc::c_uint, year: libc::c_ushort) -> libc::c_uchar;
}


/*
guint8 g_date_get_monday_weeks_in_year() [unsigned char]
	(GDateYear) year [unsigned short]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_get_monday_weeks_in_year(year: libc::c_ushort) -> libc::c_uchar;
}


/*
guint8 g_date_get_sunday_weeks_in_year() [unsigned char]
	(GDateYear) year [unsigned short]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_get_sunday_weeks_in_year(year: libc::c_ushort) -> libc::c_uchar;
}


/*
gint g_date_days_between() [int]
	(const GDate *) date1 [const struct _GDate *]
	(const GDate *) date2 [const struct _GDate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_days_between(date1: *const _GDate, date2: *const _GDate) -> libc::c_int;
}


/*
gint g_date_compare() [int]
	(const GDate *) lhs [const struct _GDate *]
	(const GDate *) rhs [const struct _GDate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_compare(lhs: *const _GDate, rhs: *const _GDate) -> libc::c_int;
}


/*
void g_date_to_struct_tm()
	(const GDate *) date [const struct _GDate *]
	(struct tm *) tm [struct tm *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_to_struct_tm(date: *const _GDate, tm: *mut tm);
}


/*
void g_date_clamp()
	(GDate *) date [struct _GDate *]
	(const GDate *) min_date [const struct _GDate *]
	(const GDate *) max_date [const struct _GDate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_clamp(date: *mut _GDate, min_date: *const _GDate, max_date: *const _GDate);
}


/*
void g_date_order()
	(GDate *) date1 [struct _GDate *]
	(GDate *) date2 [struct _GDate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_order(date1: *mut _GDate, date2: *mut _GDate);
}


/*
gsize g_date_strftime() [unsigned long]
	(gchar *) s [char *]
	(gsize) slen [unsigned long]
	(const gchar *) format [const char *]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_strftime(s: *mut libc::c_char, slen: libc::c_ulong, format: *const libc::c_char, date: *const _GDate) -> libc::c_ulong;
}


/*
GTimeZone * g_time_zone_new() [struct _GTimeZone *]
	(const gchar *) identifier [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_time_zone_new(identifier: *const libc::c_char) -> *mut _GTimeZone;
}


/*
GTimeZone * g_time_zone_new_utc() [struct _GTimeZone *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_time_zone_new_utc() -> *mut _GTimeZone;
}


/*
GTimeZone * g_time_zone_new_local() [struct _GTimeZone *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_time_zone_new_local() -> *mut _GTimeZone;
}


/*
GTimeZone * g_time_zone_ref() [struct _GTimeZone *]
	(GTimeZone *) tz [struct _GTimeZone *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_time_zone_ref(tz: *mut _GTimeZone) -> *mut _GTimeZone;
}


/*
void g_time_zone_unref()
	(GTimeZone *) tz [struct _GTimeZone *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_time_zone_unref(tz: *mut _GTimeZone);
}


/*
gint g_time_zone_find_interval() [int]
	(GTimeZone *) tz [struct _GTimeZone *]
	(GTimeType) type [GTimeType]
	(gint64) time_ [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_time_zone_find_interval(tz: *mut _GTimeZone, type_: libc::c_uint, time_: libc::c_long) -> libc::c_int;
}


/*
gint g_time_zone_adjust_time() [int]
	(GTimeZone *) tz [struct _GTimeZone *]
	(GTimeType) type [GTimeType]
	(gint64 *) time_ [long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_time_zone_adjust_time(tz: *mut _GTimeZone, type_: libc::c_uint, time_: *mut libc::c_long) -> libc::c_int;
}


/*
const gchar * g_time_zone_get_abbreviation() [const char *]
	(GTimeZone *) tz [struct _GTimeZone *]
	(gint) interval [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_time_zone_get_abbreviation(tz: *mut _GTimeZone, interval: libc::c_int) -> *const libc::c_char;
}


/*
gint32 g_time_zone_get_offset() [int]
	(GTimeZone *) tz [struct _GTimeZone *]
	(gint) interval [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_time_zone_get_offset(tz: *mut _GTimeZone, interval: libc::c_int) -> libc::c_int;
}


/*
gboolean g_time_zone_is_dst() [int]
	(GTimeZone *) tz [struct _GTimeZone *]
	(gint) interval [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_time_zone_is_dst(tz: *mut _GTimeZone, interval: libc::c_int) -> libc::c_int;
}


/*
void g_date_time_unref()
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_unref(datetime: *mut _GDateTime);
}


/*
GDateTime * g_date_time_ref() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_ref(datetime: *mut _GDateTime) -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_new_now() [struct _GDateTime *]
	(GTimeZone *) tz [struct _GTimeZone *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_new_now(tz: *mut _GTimeZone) -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_new_now_local() [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_new_now_local() -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_new_now_utc() [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_new_now_utc() -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_new_from_unix_local() [struct _GDateTime *]
	(gint64) t [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_new_from_unix_local(t: libc::c_long) -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_new_from_unix_utc() [struct _GDateTime *]
	(gint64) t [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_new_from_unix_utc(t: libc::c_long) -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_new_from_timeval_local() [struct _GDateTime *]
	(const GTimeVal *) tv [const struct _GTimeVal *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_new_from_timeval_local(tv: *const _GTimeVal) -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_new_from_timeval_utc() [struct _GDateTime *]
	(const GTimeVal *) tv [const struct _GTimeVal *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_new_from_timeval_utc(tv: *const _GTimeVal) -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_new() [struct _GDateTime *]
	(GTimeZone *) tz [struct _GTimeZone *]
	(gint) year [int]
	(gint) month [int]
	(gint) day [int]
	(gint) hour [int]
	(gint) minute [int]
	(gdouble) seconds [double]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_new(tz: *mut _GTimeZone, year: libc::c_int, month: libc::c_int, day: libc::c_int, hour: libc::c_int, minute: libc::c_int, seconds: TypeKind.DOUBLE) -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_new_local() [struct _GDateTime *]
	(gint) year [int]
	(gint) month [int]
	(gint) day [int]
	(gint) hour [int]
	(gint) minute [int]
	(gdouble) seconds [double]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_new_local(year: libc::c_int, month: libc::c_int, day: libc::c_int, hour: libc::c_int, minute: libc::c_int, seconds: TypeKind.DOUBLE) -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_new_utc() [struct _GDateTime *]
	(gint) year [int]
	(gint) month [int]
	(gint) day [int]
	(gint) hour [int]
	(gint) minute [int]
	(gdouble) seconds [double]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_new_utc(year: libc::c_int, month: libc::c_int, day: libc::c_int, hour: libc::c_int, minute: libc::c_int, seconds: TypeKind.DOUBLE) -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_add() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
	(GTimeSpan) timespan [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_add(datetime: *mut _GDateTime, timespan: libc::c_long) -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_add_years() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
	(gint) years [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_add_years(datetime: *mut _GDateTime, years: libc::c_int) -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_add_months() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
	(gint) months [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_add_months(datetime: *mut _GDateTime, months: libc::c_int) -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_add_weeks() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
	(gint) weeks [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_add_weeks(datetime: *mut _GDateTime, weeks: libc::c_int) -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_add_days() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
	(gint) days [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_add_days(datetime: *mut _GDateTime, days: libc::c_int) -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_add_hours() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
	(gint) hours [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_add_hours(datetime: *mut _GDateTime, hours: libc::c_int) -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_add_minutes() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
	(gint) minutes [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_add_minutes(datetime: *mut _GDateTime, minutes: libc::c_int) -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_add_seconds() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
	(gdouble) seconds [double]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_add_seconds(datetime: *mut _GDateTime, seconds: TypeKind.DOUBLE) -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_add_full() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
	(gint) years [int]
	(gint) months [int]
	(gint) days [int]
	(gint) hours [int]
	(gint) minutes [int]
	(gdouble) seconds [double]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_add_full(datetime: *mut _GDateTime, years: libc::c_int, months: libc::c_int, days: libc::c_int, hours: libc::c_int, minutes: libc::c_int, seconds: TypeKind.DOUBLE) -> *mut _GDateTime;
}


/*
gint g_date_time_compare() [int]
	(gconstpointer) dt1 [const void *]
	(gconstpointer) dt2 [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_compare(dt1: *const libc::c_void, dt2: *const libc::c_void) -> libc::c_int;
}


/*
GTimeSpan g_date_time_difference() [long]
	(GDateTime *) end [struct _GDateTime *]
	(GDateTime *) begin [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_difference(end: *mut _GDateTime, begin: *mut _GDateTime) -> libc::c_long;
}


/*
guint g_date_time_hash() [unsigned int]
	(gconstpointer) datetime [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_hash(datetime: *const libc::c_void) -> libc::c_uint;
}


/*
gboolean g_date_time_equal() [int]
	(gconstpointer) dt1 [const void *]
	(gconstpointer) dt2 [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_equal(dt1: *const libc::c_void, dt2: *const libc::c_void) -> libc::c_int;
}


/*
void g_date_time_get_ymd()
	(GDateTime *) datetime [struct _GDateTime *]
	(gint *) year [int *]
	(gint *) month [int *]
	(gint *) day [int *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_get_ymd(datetime: *mut _GDateTime, year: *mut libc::c_int, month: *mut libc::c_int, day: *mut libc::c_int);
}


/*
gint g_date_time_get_year() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_get_year(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
gint g_date_time_get_month() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_get_month(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
gint g_date_time_get_day_of_month() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_get_day_of_month(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
gint g_date_time_get_week_numbering_year() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_get_week_numbering_year(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
gint g_date_time_get_week_of_year() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_get_week_of_year(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
gint g_date_time_get_day_of_week() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_get_day_of_week(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
gint g_date_time_get_day_of_year() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_get_day_of_year(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
gint g_date_time_get_hour() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_get_hour(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
gint g_date_time_get_minute() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_get_minute(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
gint g_date_time_get_second() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_get_second(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
gint g_date_time_get_microsecond() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_get_microsecond(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
gdouble g_date_time_get_seconds() [double]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_get_seconds(datetime: *mut _GDateTime) -> TypeKind.DOUBLE;
}


/*
gint64 g_date_time_to_unix() [long]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_to_unix(datetime: *mut _GDateTime) -> libc::c_long;
}


/*
gboolean g_date_time_to_timeval() [int]
	(GDateTime *) datetime [struct _GDateTime *]
	(GTimeVal *) tv [struct _GTimeVal *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_to_timeval(datetime: *mut _GDateTime, tv: *mut _GTimeVal) -> libc::c_int;
}


/*
GTimeSpan g_date_time_get_utc_offset() [long]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_get_utc_offset(datetime: *mut _GDateTime) -> libc::c_long;
}


/*
const gchar * g_date_time_get_timezone_abbreviation() [const char *]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_get_timezone_abbreviation(datetime: *mut _GDateTime) -> *const libc::c_char;
}


/*
gboolean g_date_time_is_daylight_savings() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_is_daylight_savings(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
GDateTime * g_date_time_to_timezone() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
	(GTimeZone *) tz [struct _GTimeZone *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_to_timezone(datetime: *mut _GDateTime, tz: *mut _GTimeZone) -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_to_local() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_to_local(datetime: *mut _GDateTime) -> *mut _GDateTime;
}


/*
GDateTime * g_date_time_to_utc() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_to_utc(datetime: *mut _GDateTime) -> *mut _GDateTime;
}


/*
gchar * g_date_time_format() [char *]
	(GDateTime *) datetime [struct _GDateTime *]
	(const gchar *) format [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_date_time_format(datetime: *mut _GDateTime, format: *const libc::c_char) -> *mut libc::c_char;
}


/*
GDir * g_dir_open() [struct _GDir *]
	(const gchar *) path [const char *]
	(guint) flags [unsigned int]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_dir_open(path: *const libc::c_char, flags: libc::c_uint, error: *mut *mut _GError) -> *mut _GDir;
}


/*
const gchar * g_dir_read_name() [const char *]
	(GDir *) dir [struct _GDir *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_dir_read_name(dir: *mut _GDir) -> *const libc::c_char;
}


/*
void g_dir_rewind()
	(GDir *) dir [struct _GDir *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_dir_rewind(dir: *mut _GDir);
}


/*
void g_dir_close()
	(GDir *) dir [struct _GDir *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_dir_close(dir: *mut _GDir);
}


/*
const gchar * g_getenv() [const char *]
	(const gchar *) variable [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_getenv(variable: *const libc::c_char) -> *const libc::c_char;
}


/*
gboolean g_setenv() [int]
	(const gchar *) variable [const char *]
	(const gchar *) value [const char *]
	(gboolean) overwrite [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_setenv(variable: *const libc::c_char, value: *const libc::c_char, overwrite: libc::c_int) -> libc::c_int;
}


/*
void g_unsetenv()
	(const gchar *) variable [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unsetenv(variable: *const libc::c_char);
}


/*
gchar ** g_listenv() [char **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_listenv() -> *mut *mut libc::c_char;
}


/*
gchar ** g_get_environ() [char **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_environ() -> *mut *mut libc::c_char;
}


/*
const gchar * g_environ_getenv() [const char *]
	(gchar **) envp [char **]
	(const gchar *) variable [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_environ_getenv(envp: *mut *mut libc::c_char, variable: *const libc::c_char) -> *const libc::c_char;
}


/*
gchar ** g_environ_setenv() [char **]
	(gchar **) envp [char **]
	(const gchar *) variable [const char *]
	(const gchar *) value [const char *]
	(gboolean) overwrite [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_environ_setenv(envp: *mut *mut libc::c_char, variable: *const libc::c_char, value: *const libc::c_char, overwrite: libc::c_int) -> *mut *mut libc::c_char;
}


/*
gchar ** g_environ_unsetenv() [char **]
	(gchar **) envp [char **]
	(const gchar *) variable [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_environ_unsetenv(envp: *mut *mut libc::c_char, variable: *const libc::c_char) -> *mut *mut libc::c_char;
}


/*
GQuark g_file_error_quark() [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_file_error_quark() -> libc::c_uint;
}


/*
GFileError g_file_error_from_errno() [GFileError]
	(gint) err_no [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_file_error_from_errno(err_no: libc::c_int) -> libc::c_uint;
}


/*
gboolean g_file_test() [int]
	(const gchar *) filename [const char *]
	(GFileTest) test [GFileTest]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_file_test(filename: *const libc::c_char, test: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_file_get_contents() [int]
	(const gchar *) filename [const char *]
	(gchar **) contents [char **]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_file_get_contents(filename: *const libc::c_char, contents: *mut *mut libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_file_set_contents() [int]
	(const gchar *) filename [const char *]
	(const gchar *) contents [const char *]
	(gssize) length [long]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_file_set_contents(filename: *const libc::c_char, contents: *const libc::c_char, length: libc::c_long, error: *mut *mut _GError) -> libc::c_int;
}


/*
gchar * g_file_read_link() [char *]
	(const gchar *) filename [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_file_read_link(filename: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gchar * g_mkdtemp() [char *]
	(gchar *) tmpl [char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mkdtemp(tmpl: *mut libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_mkdtemp_full() [char *]
	(gchar *) tmpl [char *]
	(gint) mode [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mkdtemp_full(tmpl: *mut libc::c_char, mode: libc::c_int) -> *mut libc::c_char;
}


/*
gint g_mkstemp() [int]
	(gchar *) tmpl [char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mkstemp(tmpl: *mut libc::c_char) -> libc::c_int;
}


/*
gint g_mkstemp_full() [int]
	(gchar *) tmpl [char *]
	(gint) flags [int]
	(gint) mode [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mkstemp_full(tmpl: *mut libc::c_char, flags: libc::c_int, mode: libc::c_int) -> libc::c_int;
}


/*
gint g_file_open_tmp() [int]
	(const gchar *) tmpl [const char *]
	(gchar **) name_used [char **]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_file_open_tmp(tmpl: *const libc::c_char, name_used: *mut *mut libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
gchar * g_dir_make_tmp() [char *]
	(const gchar *) tmpl [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_dir_make_tmp(tmpl: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gchar * g_build_path() [char *]
	(const gchar *) separator [const char *]
	(const gchar *) first_element [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_build_path(separator: *const libc::c_char, first_element: *const libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_build_pathv() [char *]
	(const gchar *) separator [const char *]
	(gchar **) args [char **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_build_pathv(separator: *const libc::c_char, args: *mut *mut libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_build_filename() [char *]
	(const gchar *) first_element [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_build_filename(first_element: *const libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_build_filenamev() [char *]
	(gchar **) args [char **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_build_filenamev(args: *mut *mut libc::c_char) -> *mut libc::c_char;
}


/*
gint g_mkdir_with_parents() [int]
	(const gchar *) pathname [const char *]
	(gint) mode [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mkdir_with_parents(pathname: *const libc::c_char, mode: libc::c_int) -> libc::c_int;
}


/*
gboolean g_path_is_absolute() [int]
	(const gchar *) file_name [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_path_is_absolute(file_name: *const libc::c_char) -> libc::c_int;
}


/*
const gchar * g_path_skip_root() [const char *]
	(const gchar *) file_name [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_path_skip_root(file_name: *const libc::c_char) -> *const libc::c_char;
}


/*
const gchar * g_basename() [const char *]
	(const gchar *) file_name [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_basename(file_name: *const libc::c_char) -> *const libc::c_char;
}


/*
gchar * g_get_current_dir() [char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_current_dir() -> *mut libc::c_char;
}


/*
gchar * g_path_get_basename() [char *]
	(const gchar *) file_name [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_path_get_basename(file_name: *const libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_path_get_dirname() [char *]
	(const gchar *) file_name [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_path_get_dirname(file_name: *const libc::c_char) -> *mut libc::c_char;
}


/*
const gchar * g_strip_context() [const char *]
	(const gchar *) msgid [const char *]
	(const gchar *) msgval [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strip_context(msgid: *const libc::c_char, msgval: *const libc::c_char) -> *const libc::c_char;
}


/*
const gchar * g_dgettext() [const char *]
	(const gchar *) domain [const char *]
	(const gchar *) msgid [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_dgettext(domain: *const libc::c_char, msgid: *const libc::c_char) -> *const libc::c_char;
}


/*
const gchar * g_dcgettext() [const char *]
	(const gchar *) domain [const char *]
	(const gchar *) msgid [const char *]
	(gint) category [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_dcgettext(domain: *const libc::c_char, msgid: *const libc::c_char, category: libc::c_int) -> *const libc::c_char;
}


/*
const gchar * g_dngettext() [const char *]
	(const gchar *) domain [const char *]
	(const gchar *) msgid [const char *]
	(const gchar *) msgid_plural [const char *]
	(gulong) n [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_dngettext(domain: *const libc::c_char, msgid: *const libc::c_char, msgid_plural: *const libc::c_char, n: libc::c_ulong) -> *const libc::c_char;
}


/*
const gchar * g_dpgettext() [const char *]
	(const gchar *) domain [const char *]
	(const gchar *) msgctxtid [const char *]
	(gsize) msgidoffset [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_dpgettext(domain: *const libc::c_char, msgctxtid: *const libc::c_char, msgidoffset: libc::c_ulong) -> *const libc::c_char;
}


/*
const gchar * g_dpgettext2() [const char *]
	(const gchar *) domain [const char *]
	(const gchar *) context [const char *]
	(const gchar *) msgid [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_dpgettext2(domain: *const libc::c_char, context: *const libc::c_char, msgid: *const libc::c_char) -> *const libc::c_char;
}


/*
void g_free()
	(gpointer) mem [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_free(mem: *mut libc::c_void);
}


/*
gpointer g_malloc() [void *]
	(gsize) n_bytes [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_malloc(n_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
gpointer g_malloc0() [void *]
	(gsize) n_bytes [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_malloc0(n_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
gpointer g_realloc() [void *]
	(gpointer) mem [void *]
	(gsize) n_bytes [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_realloc(mem: *mut libc::c_void, n_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
gpointer g_try_malloc() [void *]
	(gsize) n_bytes [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_try_malloc(n_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
gpointer g_try_malloc0() [void *]
	(gsize) n_bytes [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_try_malloc0(n_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
gpointer g_try_realloc() [void *]
	(gpointer) mem [void *]
	(gsize) n_bytes [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_try_realloc(mem: *mut libc::c_void, n_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
gpointer g_malloc_n() [void *]
	(gsize) n_blocks [unsigned long]
	(gsize) n_block_bytes [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_malloc_n(n_blocks: libc::c_ulong, n_block_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
gpointer g_malloc0_n() [void *]
	(gsize) n_blocks [unsigned long]
	(gsize) n_block_bytes [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_malloc0_n(n_blocks: libc::c_ulong, n_block_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
gpointer g_realloc_n() [void *]
	(gpointer) mem [void *]
	(gsize) n_blocks [unsigned long]
	(gsize) n_block_bytes [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_realloc_n(mem: *mut libc::c_void, n_blocks: libc::c_ulong, n_block_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
gpointer g_try_malloc_n() [void *]
	(gsize) n_blocks [unsigned long]
	(gsize) n_block_bytes [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_try_malloc_n(n_blocks: libc::c_ulong, n_block_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
gpointer g_try_malloc0_n() [void *]
	(gsize) n_blocks [unsigned long]
	(gsize) n_block_bytes [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_try_malloc0_n(n_blocks: libc::c_ulong, n_block_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
gpointer g_try_realloc_n() [void *]
	(gpointer) mem [void *]
	(gsize) n_blocks [unsigned long]
	(gsize) n_block_bytes [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_try_realloc_n(mem: *mut libc::c_void, n_blocks: libc::c_ulong, n_block_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
void g_mem_set_vtable()
	(GMemVTable *) vtable [struct _GMemVTable *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mem_set_vtable(vtable: *mut _GMemVTable);
}


/*
gboolean g_mem_is_system_malloc() [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mem_is_system_malloc() -> libc::c_int;
}


/*
void g_mem_profile()
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mem_profile();
}


/*
GList * g_list_alloc() [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_alloc() -> *mut _GList;
}


/*
void g_list_free()
	(GList *) list [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_free(list: *mut _GList);
}


/*
void g_list_free_1()
	(GList *) list [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_free_1(list: *mut _GList);
}


/*
void g_list_free_full()
	(GList *) list [struct _GList *]
	(GDestroyNotify) free_func [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_free_full(list: *mut _GList, free_func: Option<extern fn(*mut libc::c_void)>);
}


/*
GList * g_list_append() [struct _GList *]
	(GList *) list [struct _GList *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_append(list: *mut _GList, data: *mut libc::c_void) -> *mut _GList;
}


/*
GList * g_list_prepend() [struct _GList *]
	(GList *) list [struct _GList *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_prepend(list: *mut _GList, data: *mut libc::c_void) -> *mut _GList;
}


/*
GList * g_list_insert() [struct _GList *]
	(GList *) list [struct _GList *]
	(gpointer) data [void *]
	(gint) position [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_insert(list: *mut _GList, data: *mut libc::c_void, position: libc::c_int) -> *mut _GList;
}


/*
GList * g_list_insert_sorted() [struct _GList *]
	(GList *) list [struct _GList *]
	(gpointer) data [void *]
	(GCompareFunc) func [int (*)(const void *, const void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_insert_sorted(list: *mut _GList, data: *mut libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GList;
}


/*
GList * g_list_insert_sorted_with_data() [struct _GList *]
	(GList *) list [struct _GList *]
	(gpointer) data [void *]
	(GCompareDataFunc) func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_insert_sorted_with_data(list: *mut _GList, data: *mut libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void) -> *mut _GList;
}


/*
GList * g_list_insert_before() [struct _GList *]
	(GList *) list [struct _GList *]
	(GList *) sibling [struct _GList *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_insert_before(list: *mut _GList, sibling: *mut _GList, data: *mut libc::c_void) -> *mut _GList;
}


/*
GList * g_list_concat() [struct _GList *]
	(GList *) list1 [struct _GList *]
	(GList *) list2 [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_concat(list1: *mut _GList, list2: *mut _GList) -> *mut _GList;
}


/*
GList * g_list_remove() [struct _GList *]
	(GList *) list [struct _GList *]
	(gconstpointer) data [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_remove(list: *mut _GList, data: *const libc::c_void) -> *mut _GList;
}


/*
GList * g_list_remove_all() [struct _GList *]
	(GList *) list [struct _GList *]
	(gconstpointer) data [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_remove_all(list: *mut _GList, data: *const libc::c_void) -> *mut _GList;
}


/*
GList * g_list_remove_link() [struct _GList *]
	(GList *) list [struct _GList *]
	(GList *) llink [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_remove_link(list: *mut _GList, llink: *mut _GList) -> *mut _GList;
}


/*
GList * g_list_delete_link() [struct _GList *]
	(GList *) list [struct _GList *]
	(GList *) link_ [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_delete_link(list: *mut _GList, link_: *mut _GList) -> *mut _GList;
}


/*
GList * g_list_reverse() [struct _GList *]
	(GList *) list [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_reverse(list: *mut _GList) -> *mut _GList;
}


/*
GList * g_list_copy() [struct _GList *]
	(GList *) list [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_copy(list: *mut _GList) -> *mut _GList;
}


/*
GList * g_list_nth() [struct _GList *]
	(GList *) list [struct _GList *]
	(guint) n [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_nth(list: *mut _GList, n: libc::c_uint) -> *mut _GList;
}


/*
GList * g_list_nth_prev() [struct _GList *]
	(GList *) list [struct _GList *]
	(guint) n [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_nth_prev(list: *mut _GList, n: libc::c_uint) -> *mut _GList;
}


/*
GList * g_list_find() [struct _GList *]
	(GList *) list [struct _GList *]
	(gconstpointer) data [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_find(list: *mut _GList, data: *const libc::c_void) -> *mut _GList;
}


/*
GList * g_list_find_custom() [struct _GList *]
	(GList *) list [struct _GList *]
	(gconstpointer) data [const void *]
	(GCompareFunc) func [int (*)(const void *, const void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_find_custom(list: *mut _GList, data: *const libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GList;
}


/*
gint g_list_position() [int]
	(GList *) list [struct _GList *]
	(GList *) llink [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_position(list: *mut _GList, llink: *mut _GList) -> libc::c_int;
}


/*
gint g_list_index() [int]
	(GList *) list [struct _GList *]
	(gconstpointer) data [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_index(list: *mut _GList, data: *const libc::c_void) -> libc::c_int;
}


/*
GList * g_list_last() [struct _GList *]
	(GList *) list [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_last(list: *mut _GList) -> *mut _GList;
}


/*
GList * g_list_first() [struct _GList *]
	(GList *) list [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_first(list: *mut _GList) -> *mut _GList;
}


/*
guint g_list_length() [unsigned int]
	(GList *) list [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_length(list: *mut _GList) -> libc::c_uint;
}


/*
void g_list_foreach()
	(GList *) list [struct _GList *]
	(GFunc) func [void (*)(void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_foreach(list: *mut _GList, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
GList * g_list_sort() [struct _GList *]
	(GList *) list [struct _GList *]
	(GCompareFunc) compare_func [int (*)(const void *, const void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_sort(list: *mut _GList, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GList;
}


/*
GList * g_list_sort_with_data() [struct _GList *]
	(GList *) list [struct _GList *]
	(GCompareDataFunc) compare_func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_sort_with_data(list: *mut _GList, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void) -> *mut _GList;
}


/*
gpointer g_list_nth_data() [void *]
	(GList *) list [struct _GList *]
	(guint) n [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_nth_data(list: *mut _GList, n: libc::c_uint) -> *mut libc::c_void;
}


/*
GHashTable * g_hash_table_new() [struct _GHashTable *]
	(GHashFunc) hash_func [unsigned int (*)(const void *)]
	(GEqualFunc) key_equal_func [int (*)(const void *, const void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_new(hash_func: Option<extern fn(*const libc::c_void) -> libc::c_uint>, key_equal_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GHashTable;
}


/*
GHashTable * g_hash_table_new_full() [struct _GHashTable *]
	(GHashFunc) hash_func [unsigned int (*)(const void *)]
	(GEqualFunc) key_equal_func [int (*)(const void *, const void *)]
	(GDestroyNotify) key_destroy_func [void (*)(void *)]
	(GDestroyNotify) value_destroy_func [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_new_full(hash_func: Option<extern fn(*const libc::c_void) -> libc::c_uint>, key_equal_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>, key_destroy_func: Option<extern fn(*mut libc::c_void)>, value_destroy_func: Option<extern fn(*mut libc::c_void)>) -> *mut _GHashTable;
}


/*
void g_hash_table_destroy()
	(GHashTable *) hash_table [struct _GHashTable *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_destroy(hash_table: *mut _GHashTable);
}


/*
void g_hash_table_insert()
	(GHashTable *) hash_table [struct _GHashTable *]
	(gpointer) key [void *]
	(gpointer) value [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_insert(hash_table: *mut _GHashTable, key: *mut libc::c_void, value: *mut libc::c_void);
}


/*
void g_hash_table_replace()
	(GHashTable *) hash_table [struct _GHashTable *]
	(gpointer) key [void *]
	(gpointer) value [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_replace(hash_table: *mut _GHashTable, key: *mut libc::c_void, value: *mut libc::c_void);
}


/*
void g_hash_table_add()
	(GHashTable *) hash_table [struct _GHashTable *]
	(gpointer) key [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_add(hash_table: *mut _GHashTable, key: *mut libc::c_void);
}


/*
gboolean g_hash_table_remove() [int]
	(GHashTable *) hash_table [struct _GHashTable *]
	(gconstpointer) key [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_remove(hash_table: *mut _GHashTable, key: *const libc::c_void) -> libc::c_int;
}


/*
void g_hash_table_remove_all()
	(GHashTable *) hash_table [struct _GHashTable *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_remove_all(hash_table: *mut _GHashTable);
}


/*
gboolean g_hash_table_steal() [int]
	(GHashTable *) hash_table [struct _GHashTable *]
	(gconstpointer) key [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_steal(hash_table: *mut _GHashTable, key: *const libc::c_void) -> libc::c_int;
}


/*
void g_hash_table_steal_all()
	(GHashTable *) hash_table [struct _GHashTable *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_steal_all(hash_table: *mut _GHashTable);
}


/*
gpointer g_hash_table_lookup() [void *]
	(GHashTable *) hash_table [struct _GHashTable *]
	(gconstpointer) key [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_lookup(hash_table: *mut _GHashTable, key: *const libc::c_void) -> *mut libc::c_void;
}


/*
gboolean g_hash_table_contains() [int]
	(GHashTable *) hash_table [struct _GHashTable *]
	(gconstpointer) key [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_contains(hash_table: *mut _GHashTable, key: *const libc::c_void) -> libc::c_int;
}


/*
gboolean g_hash_table_lookup_extended() [int]
	(GHashTable *) hash_table [struct _GHashTable *]
	(gconstpointer) lookup_key [const void *]
	(gpointer *) orig_key [void **]
	(gpointer *) value [void **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_lookup_extended(hash_table: *mut _GHashTable, lookup_key: *const libc::c_void, orig_key: *mut *mut libc::c_void, value: *mut *mut libc::c_void) -> libc::c_int;
}


/*
void g_hash_table_foreach()
	(GHashTable *) hash_table [struct _GHashTable *]
	(GHFunc) func [void (*)(void *, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_foreach(hash_table: *mut _GHashTable, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
gpointer g_hash_table_find() [void *]
	(GHashTable *) hash_table [struct _GHashTable *]
	(GHRFunc) predicate [int (*)(void *, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_find(hash_table: *mut _GHashTable, predicate: Option<extern fn(*mut libc::c_void, *mut libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void) -> *mut libc::c_void;
}


/*
guint g_hash_table_foreach_remove() [unsigned int]
	(GHashTable *) hash_table [struct _GHashTable *]
	(GHRFunc) func [int (*)(void *, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_foreach_remove(hash_table: *mut _GHashTable, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void) -> libc::c_uint;
}


/*
guint g_hash_table_foreach_steal() [unsigned int]
	(GHashTable *) hash_table [struct _GHashTable *]
	(GHRFunc) func [int (*)(void *, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_foreach_steal(hash_table: *mut _GHashTable, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void) -> libc::c_uint;
}


/*
guint g_hash_table_size() [unsigned int]
	(GHashTable *) hash_table [struct _GHashTable *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_size(hash_table: *mut _GHashTable) -> libc::c_uint;
}


/*
GList * g_hash_table_get_keys() [struct _GList *]
	(GHashTable *) hash_table [struct _GHashTable *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_get_keys(hash_table: *mut _GHashTable) -> *mut _GList;
}


/*
GList * g_hash_table_get_values() [struct _GList *]
	(GHashTable *) hash_table [struct _GHashTable *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_get_values(hash_table: *mut _GHashTable) -> *mut _GList;
}


/*
void g_hash_table_iter_init()
	(GHashTableIter *) iter [struct _GHashTableIter *]
	(GHashTable *) hash_table [struct _GHashTable *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_iter_init(iter: *mut _GHashTableIter, hash_table: *mut _GHashTable);
}


/*
gboolean g_hash_table_iter_next() [int]
	(GHashTableIter *) iter [struct _GHashTableIter *]
	(gpointer *) key [void **]
	(gpointer *) value [void **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_iter_next(iter: *mut _GHashTableIter, key: *mut *mut libc::c_void, value: *mut *mut libc::c_void) -> libc::c_int;
}


/*
GHashTable * g_hash_table_iter_get_hash_table() [struct _GHashTable *]
	(GHashTableIter *) iter [struct _GHashTableIter *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_iter_get_hash_table(iter: *mut _GHashTableIter) -> *mut _GHashTable;
}


/*
void g_hash_table_iter_remove()
	(GHashTableIter *) iter [struct _GHashTableIter *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_iter_remove(iter: *mut _GHashTableIter);
}


/*
void g_hash_table_iter_replace()
	(GHashTableIter *) iter [struct _GHashTableIter *]
	(gpointer) value [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_iter_replace(iter: *mut _GHashTableIter, value: *mut libc::c_void);
}


/*
void g_hash_table_iter_steal()
	(GHashTableIter *) iter [struct _GHashTableIter *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_iter_steal(iter: *mut _GHashTableIter);
}


/*
GHashTable * g_hash_table_ref() [struct _GHashTable *]
	(GHashTable *) hash_table [struct _GHashTable *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_ref(hash_table: *mut _GHashTable) -> *mut _GHashTable;
}


/*
void g_hash_table_unref()
	(GHashTable *) hash_table [struct _GHashTable *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hash_table_unref(hash_table: *mut _GHashTable);
}


/*
gboolean g_str_equal() [int]
	(gconstpointer) v1 [const void *]
	(gconstpointer) v2 [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_str_equal(v1: *const libc::c_void, v2: *const libc::c_void) -> libc::c_int;
}


/*
guint g_str_hash() [unsigned int]
	(gconstpointer) v [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_str_hash(v: *const libc::c_void) -> libc::c_uint;
}


/*
gboolean g_int_equal() [int]
	(gconstpointer) v1 [const void *]
	(gconstpointer) v2 [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_int_equal(v1: *const libc::c_void, v2: *const libc::c_void) -> libc::c_int;
}


/*
guint g_int_hash() [unsigned int]
	(gconstpointer) v [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_int_hash(v: *const libc::c_void) -> libc::c_uint;
}


/*
gboolean g_int64_equal() [int]
	(gconstpointer) v1 [const void *]
	(gconstpointer) v2 [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_int64_equal(v1: *const libc::c_void, v2: *const libc::c_void) -> libc::c_int;
}


/*
guint g_int64_hash() [unsigned int]
	(gconstpointer) v [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_int64_hash(v: *const libc::c_void) -> libc::c_uint;
}


/*
gboolean g_double_equal() [int]
	(gconstpointer) v1 [const void *]
	(gconstpointer) v2 [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_double_equal(v1: *const libc::c_void, v2: *const libc::c_void) -> libc::c_int;
}


/*
guint g_double_hash() [unsigned int]
	(gconstpointer) v [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_double_hash(v: *const libc::c_void) -> libc::c_uint;
}


/*
guint g_direct_hash() [unsigned int]
	(gconstpointer) v [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_direct_hash(v: *const libc::c_void) -> libc::c_uint;
}


/*
gboolean g_direct_equal() [int]
	(gconstpointer) v1 [const void *]
	(gconstpointer) v2 [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_direct_equal(v1: *const libc::c_void, v2: *const libc::c_void) -> libc::c_int;
}


/*
GHmac * g_hmac_new() [struct _GHmac *]
	(GChecksumType) digest_type [GChecksumType]
	(const guchar *) key [const unsigned char *]
	(gsize) key_len [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hmac_new(digest_type: libc::c_uint, key: *const libc::c_uchar, key_len: libc::c_ulong) -> *mut _GHmac;
}


/*
GHmac * g_hmac_copy() [struct _GHmac *]
	(const GHmac *) hmac [const struct _GHmac *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hmac_copy(hmac: *const _GHmac) -> *mut _GHmac;
}


/*
GHmac * g_hmac_ref() [struct _GHmac *]
	(GHmac *) hmac [struct _GHmac *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hmac_ref(hmac: *mut _GHmac) -> *mut _GHmac;
}


/*
void g_hmac_unref()
	(GHmac *) hmac [struct _GHmac *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hmac_unref(hmac: *mut _GHmac);
}


/*
void g_hmac_update()
	(GHmac *) hmac [struct _GHmac *]
	(const guchar *) data [const unsigned char *]
	(gssize) length [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hmac_update(hmac: *mut _GHmac, data: *const libc::c_uchar, length: libc::c_long);
}


/*
const gchar * g_hmac_get_string() [const char *]
	(GHmac *) hmac [struct _GHmac *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hmac_get_string(hmac: *mut _GHmac) -> *const libc::c_char;
}


/*
void g_hmac_get_digest()
	(GHmac *) hmac [struct _GHmac *]
	(guint8 *) buffer [unsigned char *]
	(gsize *) digest_len [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hmac_get_digest(hmac: *mut _GHmac, buffer: *mut libc::c_uchar, digest_len: *mut libc::c_ulong);
}


/*
gchar * g_compute_hmac_for_data() [char *]
	(GChecksumType) digest_type [GChecksumType]
	(const guchar *) key [const unsigned char *]
	(gsize) key_len [unsigned long]
	(const guchar *) data [const unsigned char *]
	(gsize) length [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_compute_hmac_for_data(digest_type: libc::c_uint, key: *const libc::c_uchar, key_len: libc::c_ulong, data: *const libc::c_uchar, length: libc::c_ulong) -> *mut libc::c_char;
}


/*
gchar * g_compute_hmac_for_string() [char *]
	(GChecksumType) digest_type [GChecksumType]
	(const guchar *) key [const unsigned char *]
	(gsize) key_len [unsigned long]
	(const gchar *) str [const char *]
	(gssize) length [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_compute_hmac_for_string(digest_type: libc::c_uint, key: *const libc::c_uchar, key_len: libc::c_ulong, str: *const libc::c_char, length: libc::c_long) -> *mut libc::c_char;
}


/*
void g_hook_list_init()
	(GHookList *) hook_list [struct _GHookList *]
	(guint) hook_size [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_list_init(hook_list: *mut _GHookList, hook_size: libc::c_uint);
}


/*
void g_hook_list_clear()
	(GHookList *) hook_list [struct _GHookList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_list_clear(hook_list: *mut _GHookList);
}


/*
GHook * g_hook_alloc() [struct _GHook *]
	(GHookList *) hook_list [struct _GHookList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_alloc(hook_list: *mut _GHookList) -> *mut _GHook;
}


/*
void g_hook_free()
	(GHookList *) hook_list [struct _GHookList *]
	(GHook *) hook [struct _GHook *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_free(hook_list: *mut _GHookList, hook: *mut _GHook);
}


/*
GHook * g_hook_ref() [struct _GHook *]
	(GHookList *) hook_list [struct _GHookList *]
	(GHook *) hook [struct _GHook *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_ref(hook_list: *mut _GHookList, hook: *mut _GHook) -> *mut _GHook;
}


/*
void g_hook_unref()
	(GHookList *) hook_list [struct _GHookList *]
	(GHook *) hook [struct _GHook *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_unref(hook_list: *mut _GHookList, hook: *mut _GHook);
}


/*
gboolean g_hook_destroy() [int]
	(GHookList *) hook_list [struct _GHookList *]
	(gulong) hook_id [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_destroy(hook_list: *mut _GHookList, hook_id: libc::c_ulong) -> libc::c_int;
}


/*
void g_hook_destroy_link()
	(GHookList *) hook_list [struct _GHookList *]
	(GHook *) hook [struct _GHook *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_destroy_link(hook_list: *mut _GHookList, hook: *mut _GHook);
}


/*
void g_hook_prepend()
	(GHookList *) hook_list [struct _GHookList *]
	(GHook *) hook [struct _GHook *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_prepend(hook_list: *mut _GHookList, hook: *mut _GHook);
}


/*
void g_hook_insert_before()
	(GHookList *) hook_list [struct _GHookList *]
	(GHook *) sibling [struct _GHook *]
	(GHook *) hook [struct _GHook *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_insert_before(hook_list: *mut _GHookList, sibling: *mut _GHook, hook: *mut _GHook);
}


/*
void g_hook_insert_sorted()
	(GHookList *) hook_list [struct _GHookList *]
	(GHook *) hook [struct _GHook *]
	(GHookCompareFunc) func [int (*)(struct _GHook *, struct _GHook *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_insert_sorted(hook_list: *mut _GHookList, hook: *mut _GHook, func: Option<extern fn(*mut _GHook, *mut _GHook) -> libc::c_int>);
}


/*
GHook * g_hook_get() [struct _GHook *]
	(GHookList *) hook_list [struct _GHookList *]
	(gulong) hook_id [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_get(hook_list: *mut _GHookList, hook_id: libc::c_ulong) -> *mut _GHook;
}


/*
GHook * g_hook_find() [struct _GHook *]
	(GHookList *) hook_list [struct _GHookList *]
	(gboolean) need_valids [int]
	(GHookFindFunc) func [int (*)(struct _GHook *, void *)]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_find(hook_list: *mut _GHookList, need_valids: libc::c_int, func: Option<extern fn(*mut _GHook, *mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void) -> *mut _GHook;
}


/*
GHook * g_hook_find_data() [struct _GHook *]
	(GHookList *) hook_list [struct _GHookList *]
	(gboolean) need_valids [int]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_find_data(hook_list: *mut _GHookList, need_valids: libc::c_int, data: *mut libc::c_void) -> *mut _GHook;
}


/*
GHook * g_hook_find_func() [struct _GHook *]
	(GHookList *) hook_list [struct _GHookList *]
	(gboolean) need_valids [int]
	(gpointer) func [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_find_func(hook_list: *mut _GHookList, need_valids: libc::c_int, func: *mut libc::c_void) -> *mut _GHook;
}


/*
GHook * g_hook_find_func_data() [struct _GHook *]
	(GHookList *) hook_list [struct _GHookList *]
	(gboolean) need_valids [int]
	(gpointer) func [void *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_find_func_data(hook_list: *mut _GHookList, need_valids: libc::c_int, func: *mut libc::c_void, data: *mut libc::c_void) -> *mut _GHook;
}


/*
GHook * g_hook_first_valid() [struct _GHook *]
	(GHookList *) hook_list [struct _GHookList *]
	(gboolean) may_be_in_call [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_first_valid(hook_list: *mut _GHookList, may_be_in_call: libc::c_int) -> *mut _GHook;
}


/*
GHook * g_hook_next_valid() [struct _GHook *]
	(GHookList *) hook_list [struct _GHookList *]
	(GHook *) hook [struct _GHook *]
	(gboolean) may_be_in_call [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_next_valid(hook_list: *mut _GHookList, hook: *mut _GHook, may_be_in_call: libc::c_int) -> *mut _GHook;
}


/*
gint g_hook_compare_ids() [int]
	(GHook *) new_hook [struct _GHook *]
	(GHook *) sibling [struct _GHook *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_compare_ids(new_hook: *mut _GHook, sibling: *mut _GHook) -> libc::c_int;
}


/*
void g_hook_list_invoke()
	(GHookList *) hook_list [struct _GHookList *]
	(gboolean) may_recurse [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_list_invoke(hook_list: *mut _GHookList, may_recurse: libc::c_int);
}


/*
void g_hook_list_invoke_check()
	(GHookList *) hook_list [struct _GHookList *]
	(gboolean) may_recurse [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_list_invoke_check(hook_list: *mut _GHookList, may_recurse: libc::c_int);
}


/*
void g_hook_list_marshal()
	(GHookList *) hook_list [struct _GHookList *]
	(gboolean) may_recurse [int]
	(GHookMarshaller) marshaller [void (*)(struct _GHook *, void *)]
	(gpointer) marshal_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_list_marshal(hook_list: *mut _GHookList, may_recurse: libc::c_int, marshaller: Option<extern fn(*mut _GHook, *mut libc::c_void)>, marshal_data: *mut libc::c_void);
}


/*
void g_hook_list_marshal_check()
	(GHookList *) hook_list [struct _GHookList *]
	(gboolean) may_recurse [int]
	(GHookCheckMarshaller) marshaller [int (*)(struct _GHook *, void *)]
	(gpointer) marshal_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hook_list_marshal_check(hook_list: *mut _GHookList, may_recurse: libc::c_int, marshaller: Option<extern fn(*mut _GHook, *mut libc::c_void) -> libc::c_int>, marshal_data: *mut libc::c_void);
}


/*
gboolean g_hostname_is_non_ascii() [int]
	(const gchar *) hostname [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hostname_is_non_ascii(hostname: *const libc::c_char) -> libc::c_int;
}


/*
gboolean g_hostname_is_ascii_encoded() [int]
	(const gchar *) hostname [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hostname_is_ascii_encoded(hostname: *const libc::c_char) -> libc::c_int;
}


/*
gboolean g_hostname_is_ip_address() [int]
	(const gchar *) hostname [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hostname_is_ip_address(hostname: *const libc::c_char) -> libc::c_int;
}


/*
gchar * g_hostname_to_ascii() [char *]
	(const gchar *) hostname [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hostname_to_ascii(hostname: *const libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_hostname_to_unicode() [char *]
	(const gchar *) hostname [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_hostname_to_unicode(hostname: *const libc::c_char) -> *mut libc::c_char;
}


/*
gint g_poll() [int]
	(GPollFD *) fds [struct _GPollFD *]
	(guint) nfds [unsigned int]
	(gint) timeout [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_poll(fds: *mut _GPollFD, nfds: libc::c_uint, timeout: libc::c_int) -> libc::c_int;
}


/*
GSList * g_slist_alloc() [struct _GSList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_alloc() -> *mut _GSList;
}


/*
void g_slist_free()
	(GSList *) list [struct _GSList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_free(list: *mut _GSList);
}


/*
void g_slist_free_1()
	(GSList *) list [struct _GSList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_free_1(list: *mut _GSList);
}


/*
void g_slist_free_full()
	(GSList *) list [struct _GSList *]
	(GDestroyNotify) free_func [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_free_full(list: *mut _GSList, free_func: Option<extern fn(*mut libc::c_void)>);
}


/*
GSList * g_slist_append() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_append(list: *mut _GSList, data: *mut libc::c_void) -> *mut _GSList;
}


/*
GSList * g_slist_prepend() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_prepend(list: *mut _GSList, data: *mut libc::c_void) -> *mut _GSList;
}


/*
GSList * g_slist_insert() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(gpointer) data [void *]
	(gint) position [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_insert(list: *mut _GSList, data: *mut libc::c_void, position: libc::c_int) -> *mut _GSList;
}


/*
GSList * g_slist_insert_sorted() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(gpointer) data [void *]
	(GCompareFunc) func [int (*)(const void *, const void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_insert_sorted(list: *mut _GSList, data: *mut libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GSList;
}


/*
GSList * g_slist_insert_sorted_with_data() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(gpointer) data [void *]
	(GCompareDataFunc) func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_insert_sorted_with_data(list: *mut _GSList, data: *mut libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void) -> *mut _GSList;
}


/*
GSList * g_slist_insert_before() [struct _GSList *]
	(GSList *) slist [struct _GSList *]
	(GSList *) sibling [struct _GSList *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_insert_before(slist: *mut _GSList, sibling: *mut _GSList, data: *mut libc::c_void) -> *mut _GSList;
}


/*
GSList * g_slist_concat() [struct _GSList *]
	(GSList *) list1 [struct _GSList *]
	(GSList *) list2 [struct _GSList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_concat(list1: *mut _GSList, list2: *mut _GSList) -> *mut _GSList;
}


/*
GSList * g_slist_remove() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(gconstpointer) data [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_remove(list: *mut _GSList, data: *const libc::c_void) -> *mut _GSList;
}


/*
GSList * g_slist_remove_all() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(gconstpointer) data [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_remove_all(list: *mut _GSList, data: *const libc::c_void) -> *mut _GSList;
}


/*
GSList * g_slist_remove_link() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(GSList *) link_ [struct _GSList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_remove_link(list: *mut _GSList, link_: *mut _GSList) -> *mut _GSList;
}


/*
GSList * g_slist_delete_link() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(GSList *) link_ [struct _GSList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_delete_link(list: *mut _GSList, link_: *mut _GSList) -> *mut _GSList;
}


/*
GSList * g_slist_reverse() [struct _GSList *]
	(GSList *) list [struct _GSList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_reverse(list: *mut _GSList) -> *mut _GSList;
}


/*
GSList * g_slist_copy() [struct _GSList *]
	(GSList *) list [struct _GSList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_copy(list: *mut _GSList) -> *mut _GSList;
}


/*
GSList * g_slist_nth() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(guint) n [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_nth(list: *mut _GSList, n: libc::c_uint) -> *mut _GSList;
}


/*
GSList * g_slist_find() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(gconstpointer) data [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_find(list: *mut _GSList, data: *const libc::c_void) -> *mut _GSList;
}


/*
GSList * g_slist_find_custom() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(gconstpointer) data [const void *]
	(GCompareFunc) func [int (*)(const void *, const void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_find_custom(list: *mut _GSList, data: *const libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GSList;
}


/*
gint g_slist_position() [int]
	(GSList *) list [struct _GSList *]
	(GSList *) llink [struct _GSList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_position(list: *mut _GSList, llink: *mut _GSList) -> libc::c_int;
}


/*
gint g_slist_index() [int]
	(GSList *) list [struct _GSList *]
	(gconstpointer) data [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_index(list: *mut _GSList, data: *const libc::c_void) -> libc::c_int;
}


/*
GSList * g_slist_last() [struct _GSList *]
	(GSList *) list [struct _GSList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_last(list: *mut _GSList) -> *mut _GSList;
}


/*
guint g_slist_length() [unsigned int]
	(GSList *) list [struct _GSList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_length(list: *mut _GSList) -> libc::c_uint;
}


/*
void g_slist_foreach()
	(GSList *) list [struct _GSList *]
	(GFunc) func [void (*)(void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_foreach(list: *mut _GSList, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
GSList * g_slist_sort() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(GCompareFunc) compare_func [int (*)(const void *, const void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_sort(list: *mut _GSList, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GSList;
}


/*
GSList * g_slist_sort_with_data() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(GCompareDataFunc) compare_func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_sort_with_data(list: *mut _GSList, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void) -> *mut _GSList;
}


/*
gpointer g_slist_nth_data() [void *]
	(GSList *) list [struct _GSList *]
	(guint) n [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_nth_data(list: *mut _GSList, n: libc::c_uint) -> *mut libc::c_void;
}


/*
GMainContext * g_main_context_new() [struct _GMainContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_new() -> *mut _GMainContext;
}


/*
GMainContext * g_main_context_ref() [struct _GMainContext *]
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_ref(context: *mut _GMainContext) -> *mut _GMainContext;
}


/*
void g_main_context_unref()
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_unref(context: *mut _GMainContext);
}


/*
GMainContext * g_main_context_default() [struct _GMainContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_default() -> *mut _GMainContext;
}


/*
gboolean g_main_context_iteration() [int]
	(GMainContext *) context [struct _GMainContext *]
	(gboolean) may_block [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_iteration(context: *mut _GMainContext, may_block: libc::c_int) -> libc::c_int;
}


/*
gboolean g_main_context_pending() [int]
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_pending(context: *mut _GMainContext) -> libc::c_int;
}


/*
GSource * g_main_context_find_source_by_id() [struct _GSource *]
	(GMainContext *) context [struct _GMainContext *]
	(guint) source_id [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_find_source_by_id(context: *mut _GMainContext, source_id: libc::c_uint) -> *mut _GSource;
}


/*
GSource * g_main_context_find_source_by_user_data() [struct _GSource *]
	(GMainContext *) context [struct _GMainContext *]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_find_source_by_user_data(context: *mut _GMainContext, user_data: *mut libc::c_void) -> *mut _GSource;
}


/*
GSource * g_main_context_find_source_by_funcs_user_data() [struct _GSource *]
	(GMainContext *) context [struct _GMainContext *]
	(GSourceFuncs *) funcs [struct _GSourceFuncs *]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_find_source_by_funcs_user_data(context: *mut _GMainContext, funcs: *mut _GSourceFuncs, user_data: *mut libc::c_void) -> *mut _GSource;
}


/*
void g_main_context_wakeup()
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_wakeup(context: *mut _GMainContext);
}


/*
gboolean g_main_context_acquire() [int]
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_acquire(context: *mut _GMainContext) -> libc::c_int;
}


/*
void g_main_context_release()
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_release(context: *mut _GMainContext);
}


/*
gboolean g_main_context_is_owner() [int]
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_is_owner(context: *mut _GMainContext) -> libc::c_int;
}


/*
gboolean g_main_context_wait() [int]
	(GMainContext *) context [struct _GMainContext *]
	(GCond *) cond [struct _GCond *]
	(GMutex *) mutex [union _GMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_wait(context: *mut _GMainContext, cond: *mut _GCond, mutex: *mut union _GMutex) -> libc::c_int;
}


/*
gboolean g_main_context_prepare() [int]
	(GMainContext *) context [struct _GMainContext *]
	(gint *) priority [int *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_prepare(context: *mut _GMainContext, priority: *mut libc::c_int) -> libc::c_int;
}


/*
gint g_main_context_query() [int]
	(GMainContext *) context [struct _GMainContext *]
	(gint) max_priority [int]
	(gint *) timeout_ [int *]
	(GPollFD *) fds [struct _GPollFD *]
	(gint) n_fds [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_query(context: *mut _GMainContext, max_priority: libc::c_int, timeout_: *mut libc::c_int, fds: *mut _GPollFD, n_fds: libc::c_int) -> libc::c_int;
}


/*
gint g_main_context_check() [int]
	(GMainContext *) context [struct _GMainContext *]
	(gint) max_priority [int]
	(GPollFD *) fds [struct _GPollFD *]
	(gint) n_fds [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_check(context: *mut _GMainContext, max_priority: libc::c_int, fds: *mut _GPollFD, n_fds: libc::c_int) -> libc::c_int;
}


/*
void g_main_context_dispatch()
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_dispatch(context: *mut _GMainContext);
}


/*
void g_main_context_set_poll_func()
	(GMainContext *) context [struct _GMainContext *]
	(GPollFunc) func [int (*)(struct _GPollFD *, unsigned int, int)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_set_poll_func(context: *mut _GMainContext, func: Option<extern fn(*mut _GPollFD, libc::c_uint, libc::c_int) -> libc::c_int>);
}


/*
GPollFunc g_main_context_get_poll_func() [int (*)(struct _GPollFD *, unsigned int, int)]
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_get_poll_func(context: *mut _GMainContext) -> Option<extern fn(*mut _GPollFD, libc::c_uint, libc::c_int) -> libc::c_int>;
}


/*
void g_main_context_add_poll()
	(GMainContext *) context [struct _GMainContext *]
	(GPollFD *) fd [struct _GPollFD *]
	(gint) priority [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_add_poll(context: *mut _GMainContext, fd: *mut _GPollFD, priority: libc::c_int);
}


/*
void g_main_context_remove_poll()
	(GMainContext *) context [struct _GMainContext *]
	(GPollFD *) fd [struct _GPollFD *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_remove_poll(context: *mut _GMainContext, fd: *mut _GPollFD);
}


/*
gint g_main_depth() [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_depth() -> libc::c_int;
}


/*
GSource * g_main_current_source() [struct _GSource *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_current_source() -> *mut _GSource;
}


/*
void g_main_context_push_thread_default()
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_push_thread_default(context: *mut _GMainContext);
}


/*
void g_main_context_pop_thread_default()
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_pop_thread_default(context: *mut _GMainContext);
}


/*
GMainContext * g_main_context_get_thread_default() [struct _GMainContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_get_thread_default() -> *mut _GMainContext;
}


/*
GMainContext * g_main_context_ref_thread_default() [struct _GMainContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_ref_thread_default() -> *mut _GMainContext;
}


/*
GMainLoop * g_main_loop_new() [struct _GMainLoop *]
	(GMainContext *) context [struct _GMainContext *]
	(gboolean) is_running [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_loop_new(context: *mut _GMainContext, is_running: libc::c_int) -> *mut _GMainLoop;
}


/*
void g_main_loop_run()
	(GMainLoop *) loop [struct _GMainLoop *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_loop_run(loop_: *mut _GMainLoop);
}


/*
void g_main_loop_quit()
	(GMainLoop *) loop [struct _GMainLoop *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_loop_quit(loop_: *mut _GMainLoop);
}


/*
GMainLoop * g_main_loop_ref() [struct _GMainLoop *]
	(GMainLoop *) loop [struct _GMainLoop *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_loop_ref(loop_: *mut _GMainLoop) -> *mut _GMainLoop;
}


/*
void g_main_loop_unref()
	(GMainLoop *) loop [struct _GMainLoop *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_loop_unref(loop_: *mut _GMainLoop);
}


/*
gboolean g_main_loop_is_running() [int]
	(GMainLoop *) loop [struct _GMainLoop *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_loop_is_running(loop_: *mut _GMainLoop) -> libc::c_int;
}


/*
GMainContext * g_main_loop_get_context() [struct _GMainContext *]
	(GMainLoop *) loop [struct _GMainLoop *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_loop_get_context(loop_: *mut _GMainLoop) -> *mut _GMainContext;
}


/*
GSource * g_source_new() [struct _GSource *]
	(GSourceFuncs *) source_funcs [struct _GSourceFuncs *]
	(guint) struct_size [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_new(source_funcs: *mut _GSourceFuncs, struct_size: libc::c_uint) -> *mut _GSource;
}


/*
GSource * g_source_ref() [struct _GSource *]
	(GSource *) source [struct _GSource *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_ref(source: *mut _GSource) -> *mut _GSource;
}


/*
void g_source_unref()
	(GSource *) source [struct _GSource *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_unref(source: *mut _GSource);
}


/*
guint g_source_attach() [unsigned int]
	(GSource *) source [struct _GSource *]
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_attach(source: *mut _GSource, context: *mut _GMainContext) -> libc::c_uint;
}


/*
void g_source_destroy()
	(GSource *) source [struct _GSource *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_destroy(source: *mut _GSource);
}


/*
void g_source_set_priority()
	(GSource *) source [struct _GSource *]
	(gint) priority [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_set_priority(source: *mut _GSource, priority: libc::c_int);
}


/*
gint g_source_get_priority() [int]
	(GSource *) source [struct _GSource *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_get_priority(source: *mut _GSource) -> libc::c_int;
}


/*
void g_source_set_can_recurse()
	(GSource *) source [struct _GSource *]
	(gboolean) can_recurse [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_set_can_recurse(source: *mut _GSource, can_recurse: libc::c_int);
}


/*
gboolean g_source_get_can_recurse() [int]
	(GSource *) source [struct _GSource *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_get_can_recurse(source: *mut _GSource) -> libc::c_int;
}


/*
guint g_source_get_id() [unsigned int]
	(GSource *) source [struct _GSource *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_get_id(source: *mut _GSource) -> libc::c_uint;
}


/*
GMainContext * g_source_get_context() [struct _GMainContext *]
	(GSource *) source [struct _GSource *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_get_context(source: *mut _GSource) -> *mut _GMainContext;
}


/*
void g_source_set_callback()
	(GSource *) source [struct _GSource *]
	(GSourceFunc) func [int (*)(void *)]
	(gpointer) data [void *]
	(GDestroyNotify) notify [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_set_callback(source: *mut _GSource, func: Option<extern fn(*mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void, notify: Option<extern fn(*mut libc::c_void)>);
}


/*
void g_source_set_funcs()
	(GSource *) source [struct _GSource *]
	(GSourceFuncs *) funcs [struct _GSourceFuncs *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_set_funcs(source: *mut _GSource, funcs: *mut _GSourceFuncs);
}


/*
gboolean g_source_is_destroyed() [int]
	(GSource *) source [struct _GSource *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_is_destroyed(source: *mut _GSource) -> libc::c_int;
}


/*
void g_source_set_name()
	(GSource *) source [struct _GSource *]
	(const char *) name
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_set_name(source: *mut _GSource, name: *const libc::c_char);
}


/*
const char * g_source_get_name()
	(GSource *) source [struct _GSource *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_get_name(source: *mut _GSource) -> *const libc::c_char;
}


/*
void g_source_set_name_by_id()
	(guint) tag [unsigned int]
	(const char *) name
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_set_name_by_id(tag: libc::c_uint, name: *const libc::c_char);
}


/*
void g_source_set_callback_indirect()
	(GSource *) source [struct _GSource *]
	(gpointer) callback_data [void *]
	(GSourceCallbackFuncs *) callback_funcs [struct _GSourceCallbackFuncs *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_set_callback_indirect(source: *mut _GSource, callback_data: *mut libc::c_void, callback_funcs: *mut _GSourceCallbackFuncs);
}


/*
void g_source_add_poll()
	(GSource *) source [struct _GSource *]
	(GPollFD *) fd [struct _GPollFD *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_add_poll(source: *mut _GSource, fd: *mut _GPollFD);
}


/*
void g_source_remove_poll()
	(GSource *) source [struct _GSource *]
	(GPollFD *) fd [struct _GPollFD *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_remove_poll(source: *mut _GSource, fd: *mut _GPollFD);
}


/*
void g_source_add_child_source()
	(GSource *) source [struct _GSource *]
	(GSource *) child_source [struct _GSource *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_add_child_source(source: *mut _GSource, child_source: *mut _GSource);
}


/*
void g_source_remove_child_source()
	(GSource *) source [struct _GSource *]
	(GSource *) child_source [struct _GSource *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_remove_child_source(source: *mut _GSource, child_source: *mut _GSource);
}


/*
void g_source_get_current_time()
	(GSource *) source [struct _GSource *]
	(GTimeVal *) timeval [struct _GTimeVal *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_get_current_time(source: *mut _GSource, timeval: *mut _GTimeVal);
}


/*
gint64 g_source_get_time() [long]
	(GSource *) source [struct _GSource *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_get_time(source: *mut _GSource) -> libc::c_long;
}


/*
GSource * g_idle_source_new() [struct _GSource *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_idle_source_new() -> *mut _GSource;
}


/*
GSource * g_child_watch_source_new() [struct _GSource *]
	(GPid) pid [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_child_watch_source_new(pid: libc::c_int) -> *mut _GSource;
}


/*
GSource * g_timeout_source_new() [struct _GSource *]
	(guint) interval [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_timeout_source_new(interval: libc::c_uint) -> *mut _GSource;
}


/*
GSource * g_timeout_source_new_seconds() [struct _GSource *]
	(guint) interval [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_timeout_source_new_seconds(interval: libc::c_uint) -> *mut _GSource;
}


/*
void g_get_current_time()
	(GTimeVal *) result [struct _GTimeVal *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_current_time(result: *mut _GTimeVal);
}


/*
gint64 g_get_monotonic_time() [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_monotonic_time() -> libc::c_long;
}


/*
gint64 g_get_real_time() [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_real_time() -> libc::c_long;
}


/*
gboolean g_source_remove() [int]
	(guint) tag [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_remove(tag: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_source_remove_by_user_data() [int]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_remove_by_user_data(user_data: *mut libc::c_void) -> libc::c_int;
}


/*
gboolean g_source_remove_by_funcs_user_data() [int]
	(GSourceFuncs *) funcs [struct _GSourceFuncs *]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_source_remove_by_funcs_user_data(funcs: *mut _GSourceFuncs, user_data: *mut libc::c_void) -> libc::c_int;
}


/*
guint g_timeout_add_full() [unsigned int]
	(gint) priority [int]
	(guint) interval [unsigned int]
	(GSourceFunc) function [int (*)(void *)]
	(gpointer) data [void *]
	(GDestroyNotify) notify [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_timeout_add_full(priority: libc::c_int, interval: libc::c_uint, function: Option<extern fn(*mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void, notify: Option<extern fn(*mut libc::c_void)>) -> libc::c_uint;
}


/*
guint g_timeout_add() [unsigned int]
	(guint) interval [unsigned int]
	(GSourceFunc) function [int (*)(void *)]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_timeout_add(interval: libc::c_uint, function: Option<extern fn(*mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void) -> libc::c_uint;
}


/*
guint g_timeout_add_seconds_full() [unsigned int]
	(gint) priority [int]
	(guint) interval [unsigned int]
	(GSourceFunc) function [int (*)(void *)]
	(gpointer) data [void *]
	(GDestroyNotify) notify [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_timeout_add_seconds_full(priority: libc::c_int, interval: libc::c_uint, function: Option<extern fn(*mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void, notify: Option<extern fn(*mut libc::c_void)>) -> libc::c_uint;
}


/*
guint g_timeout_add_seconds() [unsigned int]
	(guint) interval [unsigned int]
	(GSourceFunc) function [int (*)(void *)]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_timeout_add_seconds(interval: libc::c_uint, function: Option<extern fn(*mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void) -> libc::c_uint;
}


/*
guint g_child_watch_add_full() [unsigned int]
	(gint) priority [int]
	(GPid) pid [int]
	(GChildWatchFunc) function [void (*)(int, int, void *)]
	(gpointer) data [void *]
	(GDestroyNotify) notify [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_child_watch_add_full(priority: libc::c_int, pid: libc::c_int, function: Option<extern fn(libc::c_int, libc::c_int, *mut libc::c_void)>, data: *mut libc::c_void, notify: Option<extern fn(*mut libc::c_void)>) -> libc::c_uint;
}


/*
guint g_child_watch_add() [unsigned int]
	(GPid) pid [int]
	(GChildWatchFunc) function [void (*)(int, int, void *)]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_child_watch_add(pid: libc::c_int, function: Option<extern fn(libc::c_int, libc::c_int, *mut libc::c_void)>, data: *mut libc::c_void) -> libc::c_uint;
}


/*
guint g_idle_add() [unsigned int]
	(GSourceFunc) function [int (*)(void *)]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_idle_add(function: Option<extern fn(*mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void) -> libc::c_uint;
}


/*
guint g_idle_add_full() [unsigned int]
	(gint) priority [int]
	(GSourceFunc) function [int (*)(void *)]
	(gpointer) data [void *]
	(GDestroyNotify) notify [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_idle_add_full(priority: libc::c_int, function: Option<extern fn(*mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void, notify: Option<extern fn(*mut libc::c_void)>) -> libc::c_uint;
}


/*
gboolean g_idle_remove_by_data() [int]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_idle_remove_by_data(data: *mut libc::c_void) -> libc::c_int;
}


/*
void g_main_context_invoke_full()
	(GMainContext *) context [struct _GMainContext *]
	(gint) priority [int]
	(GSourceFunc) function [int (*)(void *)]
	(gpointer) data [void *]
	(GDestroyNotify) notify [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_invoke_full(context: *mut _GMainContext, priority: libc::c_int, function: Option<extern fn(*mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void, notify: Option<extern fn(*mut libc::c_void)>);
}


/*
void g_main_context_invoke()
	(GMainContext *) context [struct _GMainContext *]
	(GSourceFunc) function [int (*)(void *)]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_main_context_invoke(context: *mut _GMainContext, function: Option<extern fn(*mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void);
}


/*
guint32 g_unicode_script_to_iso15924() [unsigned int]
	(GUnicodeScript) script [GUnicodeScript]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unicode_script_to_iso15924(script: libc::c_uint) -> libc::c_uint;
}


/*
GUnicodeScript g_unicode_script_from_iso15924() [GUnicodeScript]
	(guint32) iso15924 [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unicode_script_from_iso15924(iso15924: libc::c_uint) -> libc::c_uint;
}


/*
gboolean g_unichar_isalnum() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_isalnum(c: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_unichar_isalpha() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_isalpha(c: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_unichar_iscntrl() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_iscntrl(c: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_unichar_isdigit() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_isdigit(c: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_unichar_isgraph() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_isgraph(c: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_unichar_islower() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_islower(c: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_unichar_isprint() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_isprint(c: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_unichar_ispunct() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_ispunct(c: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_unichar_isspace() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_isspace(c: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_unichar_isupper() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_isupper(c: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_unichar_isxdigit() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_isxdigit(c: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_unichar_istitle() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_istitle(c: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_unichar_isdefined() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_isdefined(c: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_unichar_iswide() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_iswide(c: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_unichar_iswide_cjk() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_iswide_cjk(c: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_unichar_iszerowidth() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_iszerowidth(c: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_unichar_ismark() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_ismark(c: libc::c_uint) -> libc::c_int;
}


/*
gunichar g_unichar_toupper() [unsigned int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_toupper(c: libc::c_uint) -> libc::c_uint;
}


/*
gunichar g_unichar_tolower() [unsigned int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_tolower(c: libc::c_uint) -> libc::c_uint;
}


/*
gunichar g_unichar_totitle() [unsigned int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_totitle(c: libc::c_uint) -> libc::c_uint;
}


/*
gint g_unichar_digit_value() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_digit_value(c: libc::c_uint) -> libc::c_int;
}


/*
gint g_unichar_xdigit_value() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_xdigit_value(c: libc::c_uint) -> libc::c_int;
}


/*
GUnicodeType g_unichar_type() [GUnicodeType]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_type(c: libc::c_uint) -> libc::c_uint;
}


/*
GUnicodeBreakType g_unichar_break_type() [GUnicodeBreakType]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_break_type(c: libc::c_uint) -> libc::c_uint;
}


/*
gint g_unichar_combining_class() [int]
	(gunichar) uc [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_combining_class(uc: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_unichar_get_mirror_char() [int]
	(gunichar) ch [unsigned int]
	(gunichar *) mirrored_ch [unsigned int *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_get_mirror_char(ch: libc::c_uint, mirrored_ch: *mut libc::c_uint) -> libc::c_int;
}


/*
GUnicodeScript g_unichar_get_script() [GUnicodeScript]
	(gunichar) ch [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_get_script(ch: libc::c_uint) -> libc::c_uint;
}


/*
gboolean g_unichar_validate() [int]
	(gunichar) ch [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_validate(ch: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_unichar_compose() [int]
	(gunichar) a [unsigned int]
	(gunichar) b [unsigned int]
	(gunichar *) ch [unsigned int *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_compose(a: libc::c_uint, b: libc::c_uint, ch: *mut libc::c_uint) -> libc::c_int;
}


/*
gboolean g_unichar_decompose() [int]
	(gunichar) ch [unsigned int]
	(gunichar *) a [unsigned int *]
	(gunichar *) b [unsigned int *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_decompose(ch: libc::c_uint, a: *mut libc::c_uint, b: *mut libc::c_uint) -> libc::c_int;
}


/*
gsize g_unichar_fully_decompose() [unsigned long]
	(gunichar) ch [unsigned int]
	(gboolean) compat [int]
	(gunichar *) result [unsigned int *]
	(gsize) result_len [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_fully_decompose(ch: libc::c_uint, compat: libc::c_int, result: *mut libc::c_uint, result_len: libc::c_ulong) -> libc::c_ulong;
}


/*
void g_unicode_canonical_ordering()
	(gunichar *) string [unsigned int *]
	(gsize) len [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unicode_canonical_ordering(string: *mut libc::c_uint, len: libc::c_ulong);
}


/*
gunichar * g_unicode_canonical_decomposition() [unsigned int *]
	(gunichar) ch [unsigned int]
	(gsize *) result_len [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unicode_canonical_decomposition(ch: libc::c_uint, result_len: *mut libc::c_ulong) -> *mut libc::c_uint;
}


/*
gunichar g_utf8_get_char() [unsigned int]
	(const gchar *) p [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_get_char(p: *const libc::c_char) -> libc::c_uint;
}


/*
gunichar g_utf8_get_char_validated() [unsigned int]
	(const gchar *) p [const char *]
	(gssize) max_len [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_get_char_validated(p: *const libc::c_char, max_len: libc::c_long) -> libc::c_uint;
}


/*
gchar * g_utf8_offset_to_pointer() [char *]
	(const gchar *) str [const char *]
	(glong) offset [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_offset_to_pointer(str: *const libc::c_char, offset: libc::c_long) -> *mut libc::c_char;
}


/*
glong g_utf8_pointer_to_offset() [long]
	(const gchar *) str [const char *]
	(const gchar *) pos [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_pointer_to_offset(str: *const libc::c_char, pos: *const libc::c_char) -> libc::c_long;
}


/*
gchar * g_utf8_prev_char() [char *]
	(const gchar *) p [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_prev_char(p: *const libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_utf8_find_next_char() [char *]
	(const gchar *) p [const char *]
	(const gchar *) end [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_find_next_char(p: *const libc::c_char, end: *const libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_utf8_find_prev_char() [char *]
	(const gchar *) str [const char *]
	(const gchar *) p [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_find_prev_char(str: *const libc::c_char, p: *const libc::c_char) -> *mut libc::c_char;
}


/*
glong g_utf8_strlen() [long]
	(const gchar *) p [const char *]
	(gssize) max [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_strlen(p: *const libc::c_char, max: libc::c_long) -> libc::c_long;
}


/*
gchar * g_utf8_substring() [char *]
	(const gchar *) str [const char *]
	(glong) start_pos [long]
	(glong) end_pos [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_substring(str: *const libc::c_char, start_pos: libc::c_long, end_pos: libc::c_long) -> *mut libc::c_char;
}


/*
gchar * g_utf8_strncpy() [char *]
	(gchar *) dest [char *]
	(const gchar *) src [const char *]
	(gsize) n [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_strncpy(dest: *mut libc::c_char, src: *const libc::c_char, n: libc::c_ulong) -> *mut libc::c_char;
}


/*
gchar * g_utf8_strchr() [char *]
	(const gchar *) p [const char *]
	(gssize) len [long]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_strchr(p: *const libc::c_char, len: libc::c_long, c: libc::c_uint) -> *mut libc::c_char;
}


/*
gchar * g_utf8_strrchr() [char *]
	(const gchar *) p [const char *]
	(gssize) len [long]
	(gunichar) c [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_strrchr(p: *const libc::c_char, len: libc::c_long, c: libc::c_uint) -> *mut libc::c_char;
}


/*
gchar * g_utf8_strreverse() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_strreverse(str: *const libc::c_char, len: libc::c_long) -> *mut libc::c_char;
}


/*
gunichar2 * g_utf8_to_utf16() [unsigned short *]
	(const gchar *) str [const char *]
	(glong) len [long]
	(glong *) items_read [long *]
	(glong *) items_written [long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_to_utf16(str: *const libc::c_char, len: libc::c_long, items_read: *mut libc::c_long, items_written: *mut libc::c_long, error: *mut *mut _GError) -> *mut libc::c_ushort;
}


/*
gunichar * g_utf8_to_ucs4() [unsigned int *]
	(const gchar *) str [const char *]
	(glong) len [long]
	(glong *) items_read [long *]
	(glong *) items_written [long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_to_ucs4(str: *const libc::c_char, len: libc::c_long, items_read: *mut libc::c_long, items_written: *mut libc::c_long, error: *mut *mut _GError) -> *mut libc::c_uint;
}


/*
gunichar * g_utf8_to_ucs4_fast() [unsigned int *]
	(const gchar *) str [const char *]
	(glong) len [long]
	(glong *) items_written [long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_to_ucs4_fast(str: *const libc::c_char, len: libc::c_long, items_written: *mut libc::c_long) -> *mut libc::c_uint;
}


/*
gunichar * g_utf16_to_ucs4() [unsigned int *]
	(const gunichar2 *) str [const unsigned short *]
	(glong) len [long]
	(glong *) items_read [long *]
	(glong *) items_written [long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf16_to_ucs4(str: *const libc::c_ushort, len: libc::c_long, items_read: *mut libc::c_long, items_written: *mut libc::c_long, error: *mut *mut _GError) -> *mut libc::c_uint;
}


/*
gchar * g_utf16_to_utf8() [char *]
	(const gunichar2 *) str [const unsigned short *]
	(glong) len [long]
	(glong *) items_read [long *]
	(glong *) items_written [long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf16_to_utf8(str: *const libc::c_ushort, len: libc::c_long, items_read: *mut libc::c_long, items_written: *mut libc::c_long, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gunichar2 * g_ucs4_to_utf16() [unsigned short *]
	(const gunichar *) str [const unsigned int *]
	(glong) len [long]
	(glong *) items_read [long *]
	(glong *) items_written [long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ucs4_to_utf16(str: *const libc::c_uint, len: libc::c_long, items_read: *mut libc::c_long, items_written: *mut libc::c_long, error: *mut *mut _GError) -> *mut libc::c_ushort;
}


/*
gchar * g_ucs4_to_utf8() [char *]
	(const gunichar *) str [const unsigned int *]
	(glong) len [long]
	(glong *) items_read [long *]
	(glong *) items_written [long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ucs4_to_utf8(str: *const libc::c_uint, len: libc::c_long, items_read: *mut libc::c_long, items_written: *mut libc::c_long, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gint g_unichar_to_utf8() [int]
	(gunichar) c [unsigned int]
	(gchar *) outbuf [char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_unichar_to_utf8(c: libc::c_uint, outbuf: *mut libc::c_char) -> libc::c_int;
}


/*
gboolean g_utf8_validate() [int]
	(const gchar *) str [const char *]
	(gssize) max_len [long]
	(const gchar **) end [const char **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_validate(str: *const libc::c_char, max_len: libc::c_long, end: *mut *const libc::c_char) -> libc::c_int;
}


/*
gchar * g_utf8_strup() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_strup(str: *const libc::c_char, len: libc::c_long) -> *mut libc::c_char;
}


/*
gchar * g_utf8_strdown() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_strdown(str: *const libc::c_char, len: libc::c_long) -> *mut libc::c_char;
}


/*
gchar * g_utf8_casefold() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_casefold(str: *const libc::c_char, len: libc::c_long) -> *mut libc::c_char;
}


/*
gchar * g_utf8_normalize() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
	(GNormalizeMode) mode [GNormalizeMode]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_normalize(str: *const libc::c_char, len: libc::c_long, mode: libc::c_uint) -> *mut libc::c_char;
}


/*
gint g_utf8_collate() [int]
	(const gchar *) str1 [const char *]
	(const gchar *) str2 [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_collate(str1: *const libc::c_char, str2: *const libc::c_char) -> libc::c_int;
}


/*
gchar * g_utf8_collate_key() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_collate_key(str: *const libc::c_char, len: libc::c_long) -> *mut libc::c_char;
}


/*
gchar * g_utf8_collate_key_for_filename() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_utf8_collate_key_for_filename(str: *const libc::c_char, len: libc::c_long) -> *mut libc::c_char;
}


/*
gchar * _g_utf8_make_valid() [char *]
	(const gchar *) name [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn _g_utf8_make_valid(name: *const libc::c_char) -> *mut libc::c_char;
}


/*
const gchar * g_get_user_name() [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_user_name() -> *const libc::c_char;
}


/*
const gchar * g_get_real_name() [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_real_name() -> *const libc::c_char;
}


/*
const gchar * g_get_home_dir() [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_home_dir() -> *const libc::c_char;
}


/*
const gchar * g_get_tmp_dir() [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_tmp_dir() -> *const libc::c_char;
}


/*
const gchar * g_get_host_name() [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_host_name() -> *const libc::c_char;
}


/*
gchar * g_get_prgname() [char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_prgname() -> *mut libc::c_char;
}


/*
void g_set_prgname()
	(const gchar *) prgname [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_set_prgname(prgname: *const libc::c_char);
}


/*
const gchar * g_get_application_name() [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_application_name() -> *const libc::c_char;
}


/*
void g_set_application_name()
	(const gchar *) application_name [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_set_application_name(application_name: *const libc::c_char);
}


/*
void g_reload_user_special_dirs_cache()
*/
#[link(name="nice")]
extern "C" {
	pub fn g_reload_user_special_dirs_cache();
}


/*
const gchar * g_get_user_data_dir() [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_user_data_dir() -> *const libc::c_char;
}


/*
const gchar * g_get_user_config_dir() [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_user_config_dir() -> *const libc::c_char;
}


/*
const gchar * g_get_user_cache_dir() [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_user_cache_dir() -> *const libc::c_char;
}


/*
const gchar *const * g_get_system_data_dirs() [const char *const *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_system_data_dirs() -> *const *const libc::c_char;
}


/*
const gchar *const * g_get_system_config_dirs() [const char *const *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_system_config_dirs() -> *const *const libc::c_char;
}


/*
const gchar * g_get_user_runtime_dir() [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_user_runtime_dir() -> *const libc::c_char;
}


/*
const gchar * g_get_user_special_dir() [const char *]
	(GUserDirectory) directory [GUserDirectory]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_get_user_special_dir(directory: libc::c_uint) -> *const libc::c_char;
}


/*
guint g_parse_debug_string() [unsigned int]
	(const gchar *) string [const char *]
	(const GDebugKey *) keys [const struct _GDebugKey *]
	(guint) nkeys [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_parse_debug_string(string: *const libc::c_char, keys: *const _GDebugKey, nkeys: libc::c_uint) -> libc::c_uint;
}


/*
gint g_snprintf() [int]
	(gchar *) string [char *]
	(gulong) n [unsigned long]
	(const gchar *) format [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_snprintf(string: *mut libc::c_char, n: libc::c_ulong, format: *const libc::c_char) -> libc::c_int;
}


/*
gint g_vsnprintf() [int]
	(gchar *) string [char *]
	(gulong) n [unsigned long]
	(const gchar *) format [const char *]
	(va_list) args [struct __va_list_tag [1]]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_vsnprintf(string: *mut libc::c_char, n: libc::c_ulong, format: *const libc::c_char, args: [__va_list_tag; 1]) -> libc::c_int;
}


/*
void g_nullify_pointer()
	(gpointer *) nullify_location [void **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_nullify_pointer(nullify_location: *mut *mut libc::c_void);
}


/*
gchar * g_format_size_full() [char *]
	(guint64) size [unsigned long]
	(GFormatSizeFlags) flags [GFormatSizeFlags]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_format_size_full(size: libc::c_ulong, flags: libc::c_uint) -> *mut libc::c_char;
}


/*
gchar * g_format_size() [char *]
	(guint64) size [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_format_size(size: libc::c_ulong) -> *mut libc::c_char;
}


/*
gchar * g_format_size_for_display() [char *]
	(goffset) size [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_format_size_for_display(size: libc::c_long) -> *mut libc::c_char;
}


/*
void g_atexit()
	(GVoidFunc) func [void (*)(void)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_atexit(func: Option<extern fn()>);
}


/*
gchar * g_find_program_in_path() [char *]
	(const gchar *) program [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_find_program_in_path(program: *const libc::c_char) -> *mut libc::c_char;
}


/*
gint g_bit_nth_lsf() [int]
	(gulong) mask [unsigned long]
	(gint) nth_bit [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bit_nth_lsf(mask: libc::c_ulong, nth_bit: libc::c_int) -> libc::c_int;
}


/*
gint g_bit_nth_msf() [int]
	(gulong) mask [unsigned long]
	(gint) nth_bit [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bit_nth_msf(mask: libc::c_ulong, nth_bit: libc::c_int) -> libc::c_int;
}


/*
guint g_bit_storage() [unsigned int]
	(gulong) number [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bit_storage(number: libc::c_ulong) -> libc::c_uint;
}


/*
gint g_bit_nth_lsf() [int]
	(gulong) mask [unsigned long]
	(gint) nth_bit [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bit_nth_lsf(mask: libc::c_ulong, nth_bit: libc::c_int) -> libc::c_int;
}


/*
gint g_bit_nth_msf() [int]
	(gulong) mask [unsigned long]
	(gint) nth_bit [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bit_nth_msf(mask: libc::c_ulong, nth_bit: libc::c_int) -> libc::c_int;
}


/*
guint g_bit_storage() [unsigned int]
	(gulong) number [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_bit_storage(number: libc::c_ulong) -> libc::c_uint;
}


/*
GString * g_string_new() [struct _GString *]
	(const gchar *) init [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_new(init: *const libc::c_char) -> *mut _GString;
}


/*
GString * g_string_new_len() [struct _GString *]
	(const gchar *) init [const char *]
	(gssize) len [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_new_len(init: *const libc::c_char, len: libc::c_long) -> *mut _GString;
}


/*
GString * g_string_sized_new() [struct _GString *]
	(gsize) dfl_size [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_sized_new(dfl_size: libc::c_ulong) -> *mut _GString;
}


/*
gchar * g_string_free() [char *]
	(GString *) string [struct _GString *]
	(gboolean) free_segment [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_free(string: *mut _GString, free_segment: libc::c_int) -> *mut libc::c_char;
}


/*
gboolean g_string_equal() [int]
	(const GString *) v [const struct _GString *]
	(const GString *) v2 [const struct _GString *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_equal(v: *const _GString, v2: *const _GString) -> libc::c_int;
}


/*
guint g_string_hash() [unsigned int]
	(const GString *) str [const struct _GString *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_hash(str: *const _GString) -> libc::c_uint;
}


/*
GString * g_string_assign() [struct _GString *]
	(GString *) string [struct _GString *]
	(const gchar *) rval [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_assign(string: *mut _GString, rval: *const libc::c_char) -> *mut _GString;
}


/*
GString * g_string_truncate() [struct _GString *]
	(GString *) string [struct _GString *]
	(gsize) len [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_truncate(string: *mut _GString, len: libc::c_ulong) -> *mut _GString;
}


/*
GString * g_string_set_size() [struct _GString *]
	(GString *) string [struct _GString *]
	(gsize) len [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_set_size(string: *mut _GString, len: libc::c_ulong) -> *mut _GString;
}


/*
GString * g_string_insert_len() [struct _GString *]
	(GString *) string [struct _GString *]
	(gssize) pos [long]
	(const gchar *) val [const char *]
	(gssize) len [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_insert_len(string: *mut _GString, pos: libc::c_long, val: *const libc::c_char, len: libc::c_long) -> *mut _GString;
}


/*
GString * g_string_append() [struct _GString *]
	(GString *) string [struct _GString *]
	(const gchar *) val [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_append(string: *mut _GString, val: *const libc::c_char) -> *mut _GString;
}


/*
GString * g_string_append_len() [struct _GString *]
	(GString *) string [struct _GString *]
	(const gchar *) val [const char *]
	(gssize) len [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_append_len(string: *mut _GString, val: *const libc::c_char, len: libc::c_long) -> *mut _GString;
}


/*
GString * g_string_append_c() [struct _GString *]
	(GString *) string [struct _GString *]
	(gchar) c [char]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_append_c(string: *mut _GString, c: libc::c_char) -> *mut _GString;
}


/*
GString * g_string_append_unichar() [struct _GString *]
	(GString *) string [struct _GString *]
	(gunichar) wc [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_append_unichar(string: *mut _GString, wc: libc::c_uint) -> *mut _GString;
}


/*
GString * g_string_prepend() [struct _GString *]
	(GString *) string [struct _GString *]
	(const gchar *) val [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_prepend(string: *mut _GString, val: *const libc::c_char) -> *mut _GString;
}


/*
GString * g_string_prepend_c() [struct _GString *]
	(GString *) string [struct _GString *]
	(gchar) c [char]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_prepend_c(string: *mut _GString, c: libc::c_char) -> *mut _GString;
}


/*
GString * g_string_prepend_unichar() [struct _GString *]
	(GString *) string [struct _GString *]
	(gunichar) wc [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_prepend_unichar(string: *mut _GString, wc: libc::c_uint) -> *mut _GString;
}


/*
GString * g_string_prepend_len() [struct _GString *]
	(GString *) string [struct _GString *]
	(const gchar *) val [const char *]
	(gssize) len [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_prepend_len(string: *mut _GString, val: *const libc::c_char, len: libc::c_long) -> *mut _GString;
}


/*
GString * g_string_insert() [struct _GString *]
	(GString *) string [struct _GString *]
	(gssize) pos [long]
	(const gchar *) val [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_insert(string: *mut _GString, pos: libc::c_long, val: *const libc::c_char) -> *mut _GString;
}


/*
GString * g_string_insert_c() [struct _GString *]
	(GString *) string [struct _GString *]
	(gssize) pos [long]
	(gchar) c [char]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_insert_c(string: *mut _GString, pos: libc::c_long, c: libc::c_char) -> *mut _GString;
}


/*
GString * g_string_insert_unichar() [struct _GString *]
	(GString *) string [struct _GString *]
	(gssize) pos [long]
	(gunichar) wc [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_insert_unichar(string: *mut _GString, pos: libc::c_long, wc: libc::c_uint) -> *mut _GString;
}


/*
GString * g_string_overwrite() [struct _GString *]
	(GString *) string [struct _GString *]
	(gsize) pos [unsigned long]
	(const gchar *) val [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_overwrite(string: *mut _GString, pos: libc::c_ulong, val: *const libc::c_char) -> *mut _GString;
}


/*
GString * g_string_overwrite_len() [struct _GString *]
	(GString *) string [struct _GString *]
	(gsize) pos [unsigned long]
	(const gchar *) val [const char *]
	(gssize) len [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_overwrite_len(string: *mut _GString, pos: libc::c_ulong, val: *const libc::c_char, len: libc::c_long) -> *mut _GString;
}


/*
GString * g_string_erase() [struct _GString *]
	(GString *) string [struct _GString *]
	(gssize) pos [long]
	(gssize) len [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_erase(string: *mut _GString, pos: libc::c_long, len: libc::c_long) -> *mut _GString;
}


/*
GString * g_string_ascii_down() [struct _GString *]
	(GString *) string [struct _GString *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_ascii_down(string: *mut _GString) -> *mut _GString;
}


/*
GString * g_string_ascii_up() [struct _GString *]
	(GString *) string [struct _GString *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_ascii_up(string: *mut _GString) -> *mut _GString;
}


/*
void g_string_vprintf()
	(GString *) string [struct _GString *]
	(const gchar *) format [const char *]
	(va_list) args [struct __va_list_tag [1]]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_vprintf(string: *mut _GString, format: *const libc::c_char, args: [__va_list_tag; 1]);
}


/*
void g_string_printf()
	(GString *) string [struct _GString *]
	(const gchar *) format [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_printf(string: *mut _GString, format: *const libc::c_char);
}


/*
void g_string_append_vprintf()
	(GString *) string [struct _GString *]
	(const gchar *) format [const char *]
	(va_list) args [struct __va_list_tag [1]]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_append_vprintf(string: *mut _GString, format: *const libc::c_char, args: [__va_list_tag; 1]);
}


/*
void g_string_append_printf()
	(GString *) string [struct _GString *]
	(const gchar *) format [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_append_printf(string: *mut _GString, format: *const libc::c_char);
}


/*
GString * g_string_append_uri_escaped() [struct _GString *]
	(GString *) string [struct _GString *]
	(const gchar *) unescaped [const char *]
	(const gchar *) reserved_chars_allowed [const char *]
	(gboolean) allow_utf8 [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_append_uri_escaped(string: *mut _GString, unescaped: *const libc::c_char, reserved_chars_allowed: *const libc::c_char, allow_utf8: libc::c_int) -> *mut _GString;
}


/*
GString * g_string_append_c_inline() [struct _GString *]
	(GString *) gstring [struct _GString *]
	(gchar) c [char]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_append_c_inline(gstring: *mut _GString, c: libc::c_char) -> *mut _GString;
}


/*
GString * g_string_down() [struct _GString *]
	(GString *) string [struct _GString *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_down(string: *mut _GString) -> *mut _GString;
}


/*
GString * g_string_up() [struct _GString *]
	(GString *) string [struct _GString *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_up(string: *mut _GString) -> *mut _GString;
}


/*
void g_io_channel_init()
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_init(channel: *mut _GIOChannel);
}


/*
GIOChannel * g_io_channel_ref() [struct _GIOChannel *]
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_ref(channel: *mut _GIOChannel) -> *mut _GIOChannel;
}


/*
void g_io_channel_unref()
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_unref(channel: *mut _GIOChannel);
}


/*
GIOError g_io_channel_read() [GIOError]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gchar *) buf [char *]
	(gsize) count [unsigned long]
	(gsize *) bytes_read [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_read(channel: *mut _GIOChannel, buf: *mut libc::c_char, count: libc::c_ulong, bytes_read: *mut libc::c_ulong) -> libc::c_uint;
}


/*
GIOError g_io_channel_write() [GIOError]
	(GIOChannel *) channel [struct _GIOChannel *]
	(const gchar *) buf [const char *]
	(gsize) count [unsigned long]
	(gsize *) bytes_written [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_write(channel: *mut _GIOChannel, buf: *const libc::c_char, count: libc::c_ulong, bytes_written: *mut libc::c_ulong) -> libc::c_uint;
}


/*
GIOError g_io_channel_seek() [GIOError]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gint64) offset [long]
	(GSeekType) type [GSeekType]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_seek(channel: *mut _GIOChannel, offset: libc::c_long, type_: libc::c_uint) -> libc::c_uint;
}


/*
void g_io_channel_close()
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_close(channel: *mut _GIOChannel);
}


/*
GIOStatus g_io_channel_shutdown() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gboolean) flush [int]
	(GError **) err [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_shutdown(channel: *mut _GIOChannel, flush: libc::c_int, err: *mut *mut _GError) -> libc::c_uint;
}


/*
guint g_io_add_watch_full() [unsigned int]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gint) priority [int]
	(GIOCondition) condition [GIOCondition]
	(GIOFunc) func [int (*)(struct _GIOChannel *, GIOCondition, void *)]
	(gpointer) user_data [void *]
	(GDestroyNotify) notify [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_add_watch_full(channel: *mut _GIOChannel, priority: libc::c_int, condition: libc::c_uint, func: Option<extern fn(*mut _GIOChannel, libc::c_uint, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void, notify: Option<extern fn(*mut libc::c_void)>) -> libc::c_uint;
}


/*
GSource * g_io_create_watch() [struct _GSource *]
	(GIOChannel *) channel [struct _GIOChannel *]
	(GIOCondition) condition [GIOCondition]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_create_watch(channel: *mut _GIOChannel, condition: libc::c_uint) -> *mut _GSource;
}


/*
guint g_io_add_watch() [unsigned int]
	(GIOChannel *) channel [struct _GIOChannel *]
	(GIOCondition) condition [GIOCondition]
	(GIOFunc) func [int (*)(struct _GIOChannel *, GIOCondition, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_add_watch(channel: *mut _GIOChannel, condition: libc::c_uint, func: Option<extern fn(*mut _GIOChannel, libc::c_uint, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void) -> libc::c_uint;
}


/*
void g_io_channel_set_buffer_size()
	(GIOChannel *) channel [struct _GIOChannel *]
	(gsize) size [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_set_buffer_size(channel: *mut _GIOChannel, size: libc::c_ulong);
}


/*
gsize g_io_channel_get_buffer_size() [unsigned long]
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_get_buffer_size(channel: *mut _GIOChannel) -> libc::c_ulong;
}


/*
GIOCondition g_io_channel_get_buffer_condition() [GIOCondition]
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_get_buffer_condition(channel: *mut _GIOChannel) -> libc::c_uint;
}


/*
GIOStatus g_io_channel_set_flags() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(GIOFlags) flags [GIOFlags]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_set_flags(channel: *mut _GIOChannel, flags: libc::c_uint, error: *mut *mut _GError) -> libc::c_uint;
}


/*
GIOFlags g_io_channel_get_flags() [GIOFlags]
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_get_flags(channel: *mut _GIOChannel) -> libc::c_uint;
}


/*
void g_io_channel_set_line_term()
	(GIOChannel *) channel [struct _GIOChannel *]
	(const gchar *) line_term [const char *]
	(gint) length [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_set_line_term(channel: *mut _GIOChannel, line_term: *const libc::c_char, length: libc::c_int);
}


/*
const gchar * g_io_channel_get_line_term() [const char *]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gint *) length [int *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_get_line_term(channel: *mut _GIOChannel, length: *mut libc::c_int) -> *const libc::c_char;
}


/*
void g_io_channel_set_buffered()
	(GIOChannel *) channel [struct _GIOChannel *]
	(gboolean) buffered [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_set_buffered(channel: *mut _GIOChannel, buffered: libc::c_int);
}


/*
gboolean g_io_channel_get_buffered() [int]
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_get_buffered(channel: *mut _GIOChannel) -> libc::c_int;
}


/*
GIOStatus g_io_channel_set_encoding() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(const gchar *) encoding [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_set_encoding(channel: *mut _GIOChannel, encoding: *const libc::c_char, error: *mut *mut _GError) -> libc::c_uint;
}


/*
const gchar * g_io_channel_get_encoding() [const char *]
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_get_encoding(channel: *mut _GIOChannel) -> *const libc::c_char;
}


/*
void g_io_channel_set_close_on_unref()
	(GIOChannel *) channel [struct _GIOChannel *]
	(gboolean) do_close [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_set_close_on_unref(channel: *mut _GIOChannel, do_close: libc::c_int);
}


/*
gboolean g_io_channel_get_close_on_unref() [int]
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_get_close_on_unref(channel: *mut _GIOChannel) -> libc::c_int;
}


/*
GIOStatus g_io_channel_flush() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_flush(channel: *mut _GIOChannel, error: *mut *mut _GError) -> libc::c_uint;
}


/*
GIOStatus g_io_channel_read_line() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gchar **) str_return [char **]
	(gsize *) length [unsigned long *]
	(gsize *) terminator_pos [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_read_line(channel: *mut _GIOChannel, str_return: *mut *mut libc::c_char, length: *mut libc::c_ulong, terminator_pos: *mut libc::c_ulong, error: *mut *mut _GError) -> libc::c_uint;
}


/*
GIOStatus g_io_channel_read_line_string() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(GString *) buffer [struct _GString *]
	(gsize *) terminator_pos [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_read_line_string(channel: *mut _GIOChannel, buffer: *mut _GString, terminator_pos: *mut libc::c_ulong, error: *mut *mut _GError) -> libc::c_uint;
}


/*
GIOStatus g_io_channel_read_to_end() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gchar **) str_return [char **]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_read_to_end(channel: *mut _GIOChannel, str_return: *mut *mut libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> libc::c_uint;
}


/*
GIOStatus g_io_channel_read_chars() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gchar *) buf [char *]
	(gsize) count [unsigned long]
	(gsize *) bytes_read [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_read_chars(channel: *mut _GIOChannel, buf: *mut libc::c_char, count: libc::c_ulong, bytes_read: *mut libc::c_ulong, error: *mut *mut _GError) -> libc::c_uint;
}


/*
GIOStatus g_io_channel_read_unichar() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gunichar *) thechar [unsigned int *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_read_unichar(channel: *mut _GIOChannel, thechar: *mut libc::c_uint, error: *mut *mut _GError) -> libc::c_uint;
}


/*
GIOStatus g_io_channel_write_chars() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(const gchar *) buf [const char *]
	(gssize) count [long]
	(gsize *) bytes_written [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_write_chars(channel: *mut _GIOChannel, buf: *const libc::c_char, count: libc::c_long, bytes_written: *mut libc::c_ulong, error: *mut *mut _GError) -> libc::c_uint;
}


/*
GIOStatus g_io_channel_write_unichar() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gunichar) thechar [unsigned int]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_write_unichar(channel: *mut _GIOChannel, thechar: libc::c_uint, error: *mut *mut _GError) -> libc::c_uint;
}


/*
GIOStatus g_io_channel_seek_position() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gint64) offset [long]
	(GSeekType) type [GSeekType]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_seek_position(channel: *mut _GIOChannel, offset: libc::c_long, type_: libc::c_uint, error: *mut *mut _GError) -> libc::c_uint;
}


/*
GIOChannel * g_io_channel_new_file() [struct _GIOChannel *]
	(const gchar *) filename [const char *]
	(const gchar *) mode [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_new_file(filename: *const libc::c_char, mode: *const libc::c_char, error: *mut *mut _GError) -> *mut _GIOChannel;
}


/*
GQuark g_io_channel_error_quark() [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_error_quark() -> libc::c_uint;
}


/*
GIOChannelError g_io_channel_error_from_errno() [GIOChannelError]
	(gint) en [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_error_from_errno(en: libc::c_int) -> libc::c_uint;
}


/*
GIOChannel * g_io_channel_unix_new() [struct _GIOChannel *]
	(int) fd
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_unix_new(fd: libc::c_int) -> *mut _GIOChannel;
}


/*
gint g_io_channel_unix_get_fd() [int]
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_io_channel_unix_get_fd(channel: *mut _GIOChannel) -> libc::c_int;
}


/*
GQuark g_key_file_error_quark() [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_error_quark() -> libc::c_uint;
}


/*
GKeyFile * g_key_file_new() [struct _GKeyFile *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_new() -> *mut _GKeyFile;
}


/*
GKeyFile * g_key_file_ref() [struct _GKeyFile *]
	(GKeyFile *) key_file [struct _GKeyFile *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_ref(key_file: *mut _GKeyFile) -> *mut _GKeyFile;
}


/*
void g_key_file_unref()
	(GKeyFile *) key_file [struct _GKeyFile *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_unref(key_file: *mut _GKeyFile);
}


/*
void g_key_file_free()
	(GKeyFile *) key_file [struct _GKeyFile *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_free(key_file: *mut _GKeyFile);
}


/*
void g_key_file_set_list_separator()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(gchar) separator [char]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_set_list_separator(key_file: *mut _GKeyFile, separator: libc::c_char);
}


/*
gboolean g_key_file_load_from_file() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) file [const char *]
	(GKeyFileFlags) flags [GKeyFileFlags]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_load_from_file(key_file: *mut _GKeyFile, file: *const libc::c_char, flags: libc::c_uint, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_key_file_load_from_data() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) data [const char *]
	(gsize) length [unsigned long]
	(GKeyFileFlags) flags [GKeyFileFlags]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_load_from_data(key_file: *mut _GKeyFile, data: *const libc::c_char, length: libc::c_ulong, flags: libc::c_uint, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_key_file_load_from_dirs() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) file [const char *]
	(const gchar **) search_dirs [const char **]
	(gchar **) full_path [char **]
	(GKeyFileFlags) flags [GKeyFileFlags]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_load_from_dirs(key_file: *mut _GKeyFile, file: *const libc::c_char, search_dirs: *mut *const libc::c_char, full_path: *mut *mut libc::c_char, flags: libc::c_uint, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_key_file_load_from_data_dirs() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) file [const char *]
	(gchar **) full_path [char **]
	(GKeyFileFlags) flags [GKeyFileFlags]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_load_from_data_dirs(key_file: *mut _GKeyFile, file: *const libc::c_char, full_path: *mut *mut libc::c_char, flags: libc::c_uint, error: *mut *mut _GError) -> libc::c_int;
}


/*
gchar * g_key_file_to_data() [char *]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_to_data(key_file: *mut _GKeyFile, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gchar * g_key_file_get_start_group() [char *]
	(GKeyFile *) key_file [struct _GKeyFile *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_get_start_group(key_file: *mut _GKeyFile) -> *mut libc::c_char;
}


/*
gchar ** g_key_file_get_groups() [char **]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(gsize *) length [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_get_groups(key_file: *mut _GKeyFile, length: *mut libc::c_ulong) -> *mut *mut libc::c_char;
}


/*
gchar ** g_key_file_get_keys() [char **]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_get_keys(key_file: *mut _GKeyFile, group_name: *const libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut *mut libc::c_char;
}


/*
gboolean g_key_file_has_group() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_has_group(key_file: *mut _GKeyFile, group_name: *const libc::c_char) -> libc::c_int;
}


/*
gboolean g_key_file_has_key() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_has_key(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
gchar * g_key_file_get_value() [char *]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_get_value(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
void g_key_file_set_value()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(const gchar *) value [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_set_value(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, value: *const libc::c_char);
}


/*
gchar * g_key_file_get_string() [char *]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_get_string(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
void g_key_file_set_string()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(const gchar *) string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_set_string(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, string: *const libc::c_char);
}


/*
gchar * g_key_file_get_locale_string() [char *]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(const gchar *) locale [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_get_locale_string(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, locale: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
void g_key_file_set_locale_string()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(const gchar *) locale [const char *]
	(const gchar *) string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_set_locale_string(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, locale: *const libc::c_char, string: *const libc::c_char);
}


/*
gboolean g_key_file_get_boolean() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_get_boolean(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_key_file_set_boolean()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gboolean) value [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_set_boolean(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, value: libc::c_int);
}


/*
gint g_key_file_get_integer() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_get_integer(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_key_file_set_integer()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gint) value [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_set_integer(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, value: libc::c_int);
}


/*
gint64 g_key_file_get_int64() [long]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_get_int64(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> libc::c_long;
}


/*
void g_key_file_set_int64()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gint64) value [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_set_int64(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, value: libc::c_long);
}


/*
guint64 g_key_file_get_uint64() [unsigned long]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_get_uint64(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> libc::c_ulong;
}


/*
void g_key_file_set_uint64()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(guint64) value [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_set_uint64(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, value: libc::c_ulong);
}


/*
gdouble g_key_file_get_double() [double]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_get_double(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> TypeKind.DOUBLE;
}


/*
void g_key_file_set_double()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gdouble) value [double]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_set_double(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, value: TypeKind.DOUBLE);
}


/*
gchar ** g_key_file_get_string_list() [char **]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_get_string_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut *mut libc::c_char;
}


/*
void g_key_file_set_string_list()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(const gchar *const []) list [const char *const[]]
	(gsize) length [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_set_string_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, list: *mut *const libc::c_char /* INCOMPLETEARRAY */, length: libc::c_ulong);
}


/*
gchar ** g_key_file_get_locale_string_list() [char **]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(const gchar *) locale [const char *]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_get_locale_string_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, locale: *const libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut *mut libc::c_char;
}


/*
void g_key_file_set_locale_string_list()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(const gchar *) locale [const char *]
	(const gchar *const []) list [const char *const[]]
	(gsize) length [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_set_locale_string_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, locale: *const libc::c_char, list: *mut *const libc::c_char /* INCOMPLETEARRAY */, length: libc::c_ulong);
}


/*
gboolean * g_key_file_get_boolean_list() [int *]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_get_boolean_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_int;
}


/*
void g_key_file_set_boolean_list()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gboolean []) list [int []]
	(gsize) length [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_set_boolean_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, list: *mut libc::c_int /* INCOMPLETEARRAY */, length: libc::c_ulong);
}


/*
gint * g_key_file_get_integer_list() [int *]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_get_integer_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_int;
}


/*
void g_key_file_set_double_list()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gdouble []) list [double []]
	(gsize) length [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_set_double_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, list: *mut TypeKind.DOUBLE /* INCOMPLETEARRAY */, length: libc::c_ulong);
}


/*
gdouble * g_key_file_get_double_list() [double *]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_get_double_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut TypeKind.DOUBLE;
}


/*
void g_key_file_set_integer_list()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gint []) list [int []]
	(gsize) length [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_set_integer_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, list: *mut libc::c_int /* INCOMPLETEARRAY */, length: libc::c_ulong);
}


/*
gboolean g_key_file_set_comment() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(const gchar *) comment [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_set_comment(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, comment: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
gchar * g_key_file_get_comment() [char *]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_get_comment(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gboolean g_key_file_remove_comment() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_remove_comment(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_key_file_remove_key() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_remove_key(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_key_file_remove_group() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_key_file_remove_group(key_file: *mut _GKeyFile, group_name: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
GMappedFile * g_mapped_file_new() [struct _GMappedFile *]
	(const gchar *) filename [const char *]
	(gboolean) writable [int]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mapped_file_new(filename: *const libc::c_char, writable: libc::c_int, error: *mut *mut _GError) -> *mut _GMappedFile;
}


/*
GMappedFile * g_mapped_file_new_from_fd() [struct _GMappedFile *]
	(gint) fd [int]
	(gboolean) writable [int]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mapped_file_new_from_fd(fd: libc::c_int, writable: libc::c_int, error: *mut *mut _GError) -> *mut _GMappedFile;
}


/*
gsize g_mapped_file_get_length() [unsigned long]
	(GMappedFile *) file [struct _GMappedFile *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mapped_file_get_length(file: *mut _GMappedFile) -> libc::c_ulong;
}


/*
gchar * g_mapped_file_get_contents() [char *]
	(GMappedFile *) file [struct _GMappedFile *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mapped_file_get_contents(file: *mut _GMappedFile) -> *mut libc::c_char;
}


/*
GMappedFile * g_mapped_file_ref() [struct _GMappedFile *]
	(GMappedFile *) file [struct _GMappedFile *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mapped_file_ref(file: *mut _GMappedFile) -> *mut _GMappedFile;
}


/*
void g_mapped_file_unref()
	(GMappedFile *) file [struct _GMappedFile *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mapped_file_unref(file: *mut _GMappedFile);
}


/*
void g_mapped_file_free()
	(GMappedFile *) file [struct _GMappedFile *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mapped_file_free(file: *mut _GMappedFile);
}


/*
GQuark g_markup_error_quark() [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_markup_error_quark() -> libc::c_uint;
}


/*
GMarkupParseContext * g_markup_parse_context_new() [struct _GMarkupParseContext *]
	(const GMarkupParser *) parser [const struct _GMarkupParser *]
	(GMarkupParseFlags) flags [GMarkupParseFlags]
	(gpointer) user_data [void *]
	(GDestroyNotify) user_data_dnotify [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_markup_parse_context_new(parser: *const _GMarkupParser, flags: libc::c_uint, user_data: *mut libc::c_void, user_data_dnotify: Option<extern fn(*mut libc::c_void)>) -> *mut _GMarkupParseContext;
}


/*
void g_markup_parse_context_free()
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_markup_parse_context_free(context: *mut _GMarkupParseContext);
}


/*
gboolean g_markup_parse_context_parse() [int]
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
	(const gchar *) text [const char *]
	(gssize) text_len [long]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_markup_parse_context_parse(context: *mut _GMarkupParseContext, text: *const libc::c_char, text_len: libc::c_long, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_markup_parse_context_push()
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
	(const GMarkupParser *) parser [const struct _GMarkupParser *]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_markup_parse_context_push(context: *mut _GMarkupParseContext, parser: *const _GMarkupParser, user_data: *mut libc::c_void);
}


/*
gpointer g_markup_parse_context_pop() [void *]
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_markup_parse_context_pop(context: *mut _GMarkupParseContext) -> *mut libc::c_void;
}


/*
gboolean g_markup_parse_context_end_parse() [int]
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_markup_parse_context_end_parse(context: *mut _GMarkupParseContext, error: *mut *mut _GError) -> libc::c_int;
}


/*
const gchar * g_markup_parse_context_get_element() [const char *]
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_markup_parse_context_get_element(context: *mut _GMarkupParseContext) -> *const libc::c_char;
}


/*
const GSList * g_markup_parse_context_get_element_stack() [const struct _GSList *]
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_markup_parse_context_get_element_stack(context: *mut _GMarkupParseContext) -> *const _GSList;
}


/*
void g_markup_parse_context_get_position()
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
	(gint *) line_number [int *]
	(gint *) char_number [int *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_markup_parse_context_get_position(context: *mut _GMarkupParseContext, line_number: *mut libc::c_int, char_number: *mut libc::c_int);
}


/*
gpointer g_markup_parse_context_get_user_data() [void *]
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_markup_parse_context_get_user_data(context: *mut _GMarkupParseContext) -> *mut libc::c_void;
}


/*
gchar * g_markup_escape_text() [char *]
	(const gchar *) text [const char *]
	(gssize) length [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_markup_escape_text(text: *const libc::c_char, length: libc::c_long) -> *mut libc::c_char;
}


/*
gchar * g_markup_printf_escaped() [char *]
	(const char *) format
*/
#[link(name="nice")]
extern "C" {
	pub fn g_markup_printf_escaped(format: *const libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_markup_vprintf_escaped() [char *]
	(const char *) format
	(va_list) args [struct __va_list_tag [1]]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_markup_vprintf_escaped(format: *const libc::c_char, args: [__va_list_tag; 1]) -> *mut libc::c_char;
}


/*
gboolean g_markup_collect_attributes() [int]
	(const gchar *) element_name [const char *]
	(const gchar **) attribute_names [const char **]
	(const gchar **) attribute_values [const char **]
	(GError **) error [struct _GError **]
	(GMarkupCollectType) first_type [GMarkupCollectType]
	(const gchar *) first_attr [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_markup_collect_attributes(element_name: *const libc::c_char, attribute_names: *mut *const libc::c_char, attribute_values: *mut *const libc::c_char, error: *mut *mut _GError, first_type: libc::c_uint, first_attr: *const libc::c_char) -> libc::c_int;
}


/*
gsize g_printf_string_upper_bound() [unsigned long]
	(const gchar *) format [const char *]
	(va_list) args [struct __va_list_tag [1]]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_printf_string_upper_bound(format: *const libc::c_char, args: [__va_list_tag; 1]) -> libc::c_ulong;
}


/*
guint g_log_set_handler() [unsigned int]
	(const gchar *) log_domain [const char *]
	(GLogLevelFlags) log_levels [GLogLevelFlags]
	(GLogFunc) log_func [void (*)(const char *, GLogLevelFlags, const char *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_log_set_handler(log_domain: *const libc::c_char, log_levels: libc::c_uint, log_func: Option<extern fn(*const libc::c_char, libc::c_uint, *const libc::c_char, *mut libc::c_void)>, user_data: *mut libc::c_void) -> libc::c_uint;
}


/*
void g_log_remove_handler()
	(const gchar *) log_domain [const char *]
	(guint) handler_id [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_log_remove_handler(log_domain: *const libc::c_char, handler_id: libc::c_uint);
}


/*
void g_log_default_handler()
	(const gchar *) log_domain [const char *]
	(GLogLevelFlags) log_level [GLogLevelFlags]
	(const gchar *) message [const char *]
	(gpointer) unused_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_log_default_handler(log_domain: *const libc::c_char, log_level: libc::c_uint, message: *const libc::c_char, unused_data: *mut libc::c_void);
}


/*
GLogFunc g_log_set_default_handler() [void (*)(const char *, GLogLevelFlags, const char *, void *)]
	(GLogFunc) log_func [void (*)(const char *, GLogLevelFlags, const char *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_log_set_default_handler(log_func: Option<extern fn(*const libc::c_char, libc::c_uint, *const libc::c_char, *mut libc::c_void)>, user_data: *mut libc::c_void) -> Option<extern fn(*const libc::c_char, libc::c_uint, *const libc::c_char, *mut libc::c_void)>;
}


/*
void g_log()
	(const gchar *) log_domain [const char *]
	(GLogLevelFlags) log_level [GLogLevelFlags]
	(const gchar *) format [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_log(log_domain: *const libc::c_char, log_level: libc::c_uint, format: *const libc::c_char);
}


/*
void g_logv()
	(const gchar *) log_domain [const char *]
	(GLogLevelFlags) log_level [GLogLevelFlags]
	(const gchar *) format [const char *]
	(va_list) args [struct __va_list_tag [1]]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_logv(log_domain: *const libc::c_char, log_level: libc::c_uint, format: *const libc::c_char, args: [__va_list_tag; 1]);
}


/*
GLogLevelFlags g_log_set_fatal_mask() [GLogLevelFlags]
	(const gchar *) log_domain [const char *]
	(GLogLevelFlags) fatal_mask [GLogLevelFlags]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_log_set_fatal_mask(log_domain: *const libc::c_char, fatal_mask: libc::c_uint) -> libc::c_uint;
}


/*
GLogLevelFlags g_log_set_always_fatal() [GLogLevelFlags]
	(GLogLevelFlags) fatal_mask [GLogLevelFlags]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_log_set_always_fatal(fatal_mask: libc::c_uint) -> libc::c_uint;
}


/*
void _g_log_fallback_handler()
	(const gchar *) log_domain [const char *]
	(GLogLevelFlags) log_level [GLogLevelFlags]
	(const gchar *) message [const char *]
	(gpointer) unused_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn _g_log_fallback_handler(log_domain: *const libc::c_char, log_level: libc::c_uint, message: *const libc::c_char, unused_data: *mut libc::c_void);
}


/*
void g_return_if_fail_warning()
	(const char *) log_domain
	(const char *) pretty_function
	(const char *) expression
*/
#[link(name="nice")]
extern "C" {
	pub fn g_return_if_fail_warning(log_domain: *const libc::c_char, pretty_function: *const libc::c_char, expression: *const libc::c_char);
}


/*
void g_warn_message()
	(const char *) domain
	(const char *) file
	(int) line
	(const char *) func
	(const char *) warnexpr
*/
#[link(name="nice")]
extern "C" {
	pub fn g_warn_message(domain: *const libc::c_char, file: *const libc::c_char, line: libc::c_int, func: *const libc::c_char, warnexpr: *const libc::c_char);
}


/*
void g_assert_warning()
	(const char *) log_domain
	(const char *) file
	(const int) line
	(const char *) pretty_function
	(const char *) expression
*/
#[link(name="nice")]
extern "C" {
	pub fn g_assert_warning(log_domain: *const libc::c_char, file: *const libc::c_char, line: libc::c_int, pretty_function: *const libc::c_char, expression: *const libc::c_char);
}


/*
void g_print()
	(const gchar *) format [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_print(format: *const libc::c_char);
}


/*
GPrintFunc g_set_print_handler() [void (*)(const char *)]
	(GPrintFunc) func [void (*)(const char *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_set_print_handler(func: Option<extern fn(*const libc::c_char)>) -> Option<extern fn(*const libc::c_char)>;
}


/*
void g_printerr()
	(const gchar *) format [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_printerr(format: *const libc::c_char);
}


/*
GPrintFunc g_set_printerr_handler() [void (*)(const char *)]
	(GPrintFunc) func [void (*)(const char *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_set_printerr_handler(func: Option<extern fn(*const libc::c_char)>) -> Option<extern fn(*const libc::c_char)>;
}


/*
GNode * g_node_new() [struct _GNode *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_new(data: *mut libc::c_void) -> *mut _GNode;
}


/*
void g_node_destroy()
	(GNode *) root [struct _GNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_destroy(root: *mut _GNode);
}


/*
void g_node_unlink()
	(GNode *) node [struct _GNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_unlink(node: *mut _GNode);
}


/*
GNode * g_node_copy_deep() [struct _GNode *]
	(GNode *) node [struct _GNode *]
	(GCopyFunc) copy_func [void *(*)(const void *, void *)]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_copy_deep(node: *mut _GNode, copy_func: Option<extern fn(*const libc::c_void, *mut libc::c_void) -> *mut libc::c_void>, data: *mut libc::c_void) -> *mut _GNode;
}


/*
GNode * g_node_copy() [struct _GNode *]
	(GNode *) node [struct _GNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_copy(node: *mut _GNode) -> *mut _GNode;
}


/*
GNode * g_node_insert() [struct _GNode *]
	(GNode *) parent [struct _GNode *]
	(gint) position [int]
	(GNode *) node [struct _GNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_insert(parent: *mut _GNode, position: libc::c_int, node: *mut _GNode) -> *mut _GNode;
}


/*
GNode * g_node_insert_before() [struct _GNode *]
	(GNode *) parent [struct _GNode *]
	(GNode *) sibling [struct _GNode *]
	(GNode *) node [struct _GNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_insert_before(parent: *mut _GNode, sibling: *mut _GNode, node: *mut _GNode) -> *mut _GNode;
}


/*
GNode * g_node_insert_after() [struct _GNode *]
	(GNode *) parent [struct _GNode *]
	(GNode *) sibling [struct _GNode *]
	(GNode *) node [struct _GNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_insert_after(parent: *mut _GNode, sibling: *mut _GNode, node: *mut _GNode) -> *mut _GNode;
}


/*
GNode * g_node_prepend() [struct _GNode *]
	(GNode *) parent [struct _GNode *]
	(GNode *) node [struct _GNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_prepend(parent: *mut _GNode, node: *mut _GNode) -> *mut _GNode;
}


/*
guint g_node_n_nodes() [unsigned int]
	(GNode *) root [struct _GNode *]
	(GTraverseFlags) flags [GTraverseFlags]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_n_nodes(root: *mut _GNode, flags: libc::c_uint) -> libc::c_uint;
}


/*
GNode * g_node_get_root() [struct _GNode *]
	(GNode *) node [struct _GNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_get_root(node: *mut _GNode) -> *mut _GNode;
}


/*
gboolean g_node_is_ancestor() [int]
	(GNode *) node [struct _GNode *]
	(GNode *) descendant [struct _GNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_is_ancestor(node: *mut _GNode, descendant: *mut _GNode) -> libc::c_int;
}


/*
guint g_node_depth() [unsigned int]
	(GNode *) node [struct _GNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_depth(node: *mut _GNode) -> libc::c_uint;
}


/*
GNode * g_node_find() [struct _GNode *]
	(GNode *) root [struct _GNode *]
	(GTraverseType) order [GTraverseType]
	(GTraverseFlags) flags [GTraverseFlags]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_find(root: *mut _GNode, order: libc::c_uint, flags: libc::c_uint, data: *mut libc::c_void) -> *mut _GNode;
}


/*
void g_node_traverse()
	(GNode *) root [struct _GNode *]
	(GTraverseType) order [GTraverseType]
	(GTraverseFlags) flags [GTraverseFlags]
	(gint) max_depth [int]
	(GNodeTraverseFunc) func [int (*)(struct _GNode *, void *)]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_traverse(root: *mut _GNode, order: libc::c_uint, flags: libc::c_uint, max_depth: libc::c_int, func: Option<extern fn(*mut _GNode, *mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void);
}


/*
guint g_node_max_height() [unsigned int]
	(GNode *) root [struct _GNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_max_height(root: *mut _GNode) -> libc::c_uint;
}


/*
void g_node_children_foreach()
	(GNode *) node [struct _GNode *]
	(GTraverseFlags) flags [GTraverseFlags]
	(GNodeForeachFunc) func [void (*)(struct _GNode *, void *)]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_children_foreach(node: *mut _GNode, flags: libc::c_uint, func: Option<extern fn(*mut _GNode, *mut libc::c_void)>, data: *mut libc::c_void);
}


/*
void g_node_reverse_children()
	(GNode *) node [struct _GNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_reverse_children(node: *mut _GNode);
}


/*
guint g_node_n_children() [unsigned int]
	(GNode *) node [struct _GNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_n_children(node: *mut _GNode) -> libc::c_uint;
}


/*
GNode * g_node_nth_child() [struct _GNode *]
	(GNode *) node [struct _GNode *]
	(guint) n [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_nth_child(node: *mut _GNode, n: libc::c_uint) -> *mut _GNode;
}


/*
GNode * g_node_last_child() [struct _GNode *]
	(GNode *) node [struct _GNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_last_child(node: *mut _GNode) -> *mut _GNode;
}


/*
GNode * g_node_find_child() [struct _GNode *]
	(GNode *) node [struct _GNode *]
	(GTraverseFlags) flags [GTraverseFlags]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_find_child(node: *mut _GNode, flags: libc::c_uint, data: *mut libc::c_void) -> *mut _GNode;
}


/*
gint g_node_child_position() [int]
	(GNode *) node [struct _GNode *]
	(GNode *) child [struct _GNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_child_position(node: *mut _GNode, child: *mut _GNode) -> libc::c_int;
}


/*
gint g_node_child_index() [int]
	(GNode *) node [struct _GNode *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_child_index(node: *mut _GNode, data: *mut libc::c_void) -> libc::c_int;
}


/*
GNode * g_node_first_sibling() [struct _GNode *]
	(GNode *) node [struct _GNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_first_sibling(node: *mut _GNode) -> *mut _GNode;
}


/*
GNode * g_node_last_sibling() [struct _GNode *]
	(GNode *) node [struct _GNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_last_sibling(node: *mut _GNode) -> *mut _GNode;
}


/*
GQuark g_option_error_quark() [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_error_quark() -> libc::c_uint;
}


/*
GOptionContext * g_option_context_new() [struct _GOptionContext *]
	(const gchar *) parameter_string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_context_new(parameter_string: *const libc::c_char) -> *mut _GOptionContext;
}


/*
void g_option_context_set_summary()
	(GOptionContext *) context [struct _GOptionContext *]
	(const gchar *) summary [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_context_set_summary(context: *mut _GOptionContext, summary: *const libc::c_char);
}


/*
const gchar * g_option_context_get_summary() [const char *]
	(GOptionContext *) context [struct _GOptionContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_context_get_summary(context: *mut _GOptionContext) -> *const libc::c_char;
}


/*
void g_option_context_set_description()
	(GOptionContext *) context [struct _GOptionContext *]
	(const gchar *) description [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_context_set_description(context: *mut _GOptionContext, description: *const libc::c_char);
}


/*
const gchar * g_option_context_get_description() [const char *]
	(GOptionContext *) context [struct _GOptionContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_context_get_description(context: *mut _GOptionContext) -> *const libc::c_char;
}


/*
void g_option_context_free()
	(GOptionContext *) context [struct _GOptionContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_context_free(context: *mut _GOptionContext);
}


/*
void g_option_context_set_help_enabled()
	(GOptionContext *) context [struct _GOptionContext *]
	(gboolean) help_enabled [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_context_set_help_enabled(context: *mut _GOptionContext, help_enabled: libc::c_int);
}


/*
gboolean g_option_context_get_help_enabled() [int]
	(GOptionContext *) context [struct _GOptionContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_context_get_help_enabled(context: *mut _GOptionContext) -> libc::c_int;
}


/*
void g_option_context_set_ignore_unknown_options()
	(GOptionContext *) context [struct _GOptionContext *]
	(gboolean) ignore_unknown [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_context_set_ignore_unknown_options(context: *mut _GOptionContext, ignore_unknown: libc::c_int);
}


/*
gboolean g_option_context_get_ignore_unknown_options() [int]
	(GOptionContext *) context [struct _GOptionContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_context_get_ignore_unknown_options(context: *mut _GOptionContext) -> libc::c_int;
}


/*
void g_option_context_add_main_entries()
	(GOptionContext *) context [struct _GOptionContext *]
	(const GOptionEntry *) entries [const struct _GOptionEntry *]
	(const gchar *) translation_domain [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_context_add_main_entries(context: *mut _GOptionContext, entries: *const _GOptionEntry, translation_domain: *const libc::c_char);
}


/*
gboolean g_option_context_parse() [int]
	(GOptionContext *) context [struct _GOptionContext *]
	(gint *) argc [int *]
	(gchar ***) argv [char ***]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_context_parse(context: *mut _GOptionContext, argc: *mut libc::c_int, argv: *mut *mut *mut libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_option_context_set_translate_func()
	(GOptionContext *) context [struct _GOptionContext *]
	(GTranslateFunc) func [const char *(*)(const char *, void *)]
	(gpointer) data [void *]
	(GDestroyNotify) destroy_notify [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_context_set_translate_func(context: *mut _GOptionContext, func: Option<extern fn(*const libc::c_char, *mut libc::c_void) -> *const libc::c_char>, data: *mut libc::c_void, destroy_notify: Option<extern fn(*mut libc::c_void)>);
}


/*
void g_option_context_set_translation_domain()
	(GOptionContext *) context [struct _GOptionContext *]
	(const gchar *) domain [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_context_set_translation_domain(context: *mut _GOptionContext, domain: *const libc::c_char);
}


/*
void g_option_context_add_group()
	(GOptionContext *) context [struct _GOptionContext *]
	(GOptionGroup *) group [struct _GOptionGroup *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_context_add_group(context: *mut _GOptionContext, group: *mut _GOptionGroup);
}


/*
void g_option_context_set_main_group()
	(GOptionContext *) context [struct _GOptionContext *]
	(GOptionGroup *) group [struct _GOptionGroup *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_context_set_main_group(context: *mut _GOptionContext, group: *mut _GOptionGroup);
}


/*
GOptionGroup * g_option_context_get_main_group() [struct _GOptionGroup *]
	(GOptionContext *) context [struct _GOptionContext *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_context_get_main_group(context: *mut _GOptionContext) -> *mut _GOptionGroup;
}


/*
gchar * g_option_context_get_help() [char *]
	(GOptionContext *) context [struct _GOptionContext *]
	(gboolean) main_help [int]
	(GOptionGroup *) group [struct _GOptionGroup *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_context_get_help(context: *mut _GOptionContext, main_help: libc::c_int, group: *mut _GOptionGroup) -> *mut libc::c_char;
}


/*
GOptionGroup * g_option_group_new() [struct _GOptionGroup *]
	(const gchar *) name [const char *]
	(const gchar *) description [const char *]
	(const gchar *) help_description [const char *]
	(gpointer) user_data [void *]
	(GDestroyNotify) destroy [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_group_new(name: *const libc::c_char, description: *const libc::c_char, help_description: *const libc::c_char, user_data: *mut libc::c_void, destroy: Option<extern fn(*mut libc::c_void)>) -> *mut _GOptionGroup;
}


/*
void g_option_group_set_parse_hooks()
	(GOptionGroup *) group [struct _GOptionGroup *]
	(GOptionParseFunc) pre_parse_func [int (*)(struct _GOptionContext *, struct _GOptionGroup *, void *, struct _GError **)]
	(GOptionParseFunc) post_parse_func [int (*)(struct _GOptionContext *, struct _GOptionGroup *, void *, struct _GError **)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_group_set_parse_hooks(group: *mut _GOptionGroup, pre_parse_func: Option<extern fn(*mut _GOptionContext, *mut _GOptionGroup, *mut libc::c_void, *mut *mut _GError) -> libc::c_int>, post_parse_func: Option<extern fn(*mut _GOptionContext, *mut _GOptionGroup, *mut libc::c_void, *mut *mut _GError) -> libc::c_int>);
}


/*
void g_option_group_set_error_hook()
	(GOptionGroup *) group [struct _GOptionGroup *]
	(GOptionErrorFunc) error_func [void (*)(struct _GOptionContext *, struct _GOptionGroup *, void *, struct _GError **)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_group_set_error_hook(group: *mut _GOptionGroup, error_func: Option<extern fn(*mut _GOptionContext, *mut _GOptionGroup, *mut libc::c_void, *mut *mut _GError)>);
}


/*
void g_option_group_free()
	(GOptionGroup *) group [struct _GOptionGroup *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_group_free(group: *mut _GOptionGroup);
}


/*
void g_option_group_add_entries()
	(GOptionGroup *) group [struct _GOptionGroup *]
	(const GOptionEntry *) entries [const struct _GOptionEntry *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_group_add_entries(group: *mut _GOptionGroup, entries: *const _GOptionEntry);
}


/*
void g_option_group_set_translate_func()
	(GOptionGroup *) group [struct _GOptionGroup *]
	(GTranslateFunc) func [const char *(*)(const char *, void *)]
	(gpointer) data [void *]
	(GDestroyNotify) destroy_notify [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_group_set_translate_func(group: *mut _GOptionGroup, func: Option<extern fn(*const libc::c_char, *mut libc::c_void) -> *const libc::c_char>, data: *mut libc::c_void, destroy_notify: Option<extern fn(*mut libc::c_void)>);
}


/*
void g_option_group_set_translation_domain()
	(GOptionGroup *) group [struct _GOptionGroup *]
	(const gchar *) domain [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_option_group_set_translation_domain(group: *mut _GOptionGroup, domain: *const libc::c_char);
}


/*
GPatternSpec * g_pattern_spec_new() [struct _GPatternSpec *]
	(const gchar *) pattern [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_pattern_spec_new(pattern: *const libc::c_char) -> *mut _GPatternSpec;
}


/*
void g_pattern_spec_free()
	(GPatternSpec *) pspec [struct _GPatternSpec *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_pattern_spec_free(pspec: *mut _GPatternSpec);
}


/*
gboolean g_pattern_spec_equal() [int]
	(GPatternSpec *) pspec1 [struct _GPatternSpec *]
	(GPatternSpec *) pspec2 [struct _GPatternSpec *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_pattern_spec_equal(pspec1: *mut _GPatternSpec, pspec2: *mut _GPatternSpec) -> libc::c_int;
}


/*
gboolean g_pattern_match() [int]
	(GPatternSpec *) pspec [struct _GPatternSpec *]
	(guint) string_length [unsigned int]
	(const gchar *) string [const char *]
	(const gchar *) string_reversed [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_pattern_match(pspec: *mut _GPatternSpec, string_length: libc::c_uint, string: *const libc::c_char, string_reversed: *const libc::c_char) -> libc::c_int;
}


/*
gboolean g_pattern_match_string() [int]
	(GPatternSpec *) pspec [struct _GPatternSpec *]
	(const gchar *) string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_pattern_match_string(pspec: *mut _GPatternSpec, string: *const libc::c_char) -> libc::c_int;
}


/*
gboolean g_pattern_match_simple() [int]
	(const gchar *) pattern [const char *]
	(const gchar *) string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_pattern_match_simple(pattern: *const libc::c_char, string: *const libc::c_char) -> libc::c_int;
}


/*
guint g_spaced_primes_closest() [unsigned int]
	(guint) num [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_spaced_primes_closest(num: libc::c_uint) -> libc::c_uint;
}


/*
void g_qsort_with_data()
	(gconstpointer) pbase [const void *]
	(gint) total_elems [int]
	(gsize) size [unsigned long]
	(GCompareDataFunc) compare_func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_qsort_with_data(pbase: *const libc::c_void, total_elems: libc::c_int, size: libc::c_ulong, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
GQueue * g_queue_new() [struct _GQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_new() -> *mut _GQueue;
}


/*
void g_queue_free()
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_free(queue: *mut _GQueue);
}


/*
void g_queue_free_full()
	(GQueue *) queue [struct _GQueue *]
	(GDestroyNotify) free_func [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_free_full(queue: *mut _GQueue, free_func: Option<extern fn(*mut libc::c_void)>);
}


/*
void g_queue_init()
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_init(queue: *mut _GQueue);
}


/*
void g_queue_clear()
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_clear(queue: *mut _GQueue);
}


/*
gboolean g_queue_is_empty() [int]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_is_empty(queue: *mut _GQueue) -> libc::c_int;
}


/*
guint g_queue_get_length() [unsigned int]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_get_length(queue: *mut _GQueue) -> libc::c_uint;
}


/*
void g_queue_reverse()
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_reverse(queue: *mut _GQueue);
}


/*
GQueue * g_queue_copy() [struct _GQueue *]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_copy(queue: *mut _GQueue) -> *mut _GQueue;
}


/*
void g_queue_foreach()
	(GQueue *) queue [struct _GQueue *]
	(GFunc) func [void (*)(void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_foreach(queue: *mut _GQueue, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
GList * g_queue_find() [struct _GList *]
	(GQueue *) queue [struct _GQueue *]
	(gconstpointer) data [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_find(queue: *mut _GQueue, data: *const libc::c_void) -> *mut _GList;
}


/*
GList * g_queue_find_custom() [struct _GList *]
	(GQueue *) queue [struct _GQueue *]
	(gconstpointer) data [const void *]
	(GCompareFunc) func [int (*)(const void *, const void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_find_custom(queue: *mut _GQueue, data: *const libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GList;
}


/*
void g_queue_sort()
	(GQueue *) queue [struct _GQueue *]
	(GCompareDataFunc) compare_func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_sort(queue: *mut _GQueue, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
void g_queue_push_head()
	(GQueue *) queue [struct _GQueue *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_push_head(queue: *mut _GQueue, data: *mut libc::c_void);
}


/*
void g_queue_push_tail()
	(GQueue *) queue [struct _GQueue *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_push_tail(queue: *mut _GQueue, data: *mut libc::c_void);
}


/*
void g_queue_push_nth()
	(GQueue *) queue [struct _GQueue *]
	(gpointer) data [void *]
	(gint) n [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_push_nth(queue: *mut _GQueue, data: *mut libc::c_void, n: libc::c_int);
}


/*
gpointer g_queue_pop_head() [void *]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_pop_head(queue: *mut _GQueue) -> *mut libc::c_void;
}


/*
gpointer g_queue_pop_tail() [void *]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_pop_tail(queue: *mut _GQueue) -> *mut libc::c_void;
}


/*
gpointer g_queue_pop_nth() [void *]
	(GQueue *) queue [struct _GQueue *]
	(guint) n [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_pop_nth(queue: *mut _GQueue, n: libc::c_uint) -> *mut libc::c_void;
}


/*
gpointer g_queue_peek_head() [void *]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_peek_head(queue: *mut _GQueue) -> *mut libc::c_void;
}


/*
gpointer g_queue_peek_tail() [void *]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_peek_tail(queue: *mut _GQueue) -> *mut libc::c_void;
}


/*
gpointer g_queue_peek_nth() [void *]
	(GQueue *) queue [struct _GQueue *]
	(guint) n [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_peek_nth(queue: *mut _GQueue, n: libc::c_uint) -> *mut libc::c_void;
}


/*
gint g_queue_index() [int]
	(GQueue *) queue [struct _GQueue *]
	(gconstpointer) data [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_index(queue: *mut _GQueue, data: *const libc::c_void) -> libc::c_int;
}


/*
gboolean g_queue_remove() [int]
	(GQueue *) queue [struct _GQueue *]
	(gconstpointer) data [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_remove(queue: *mut _GQueue, data: *const libc::c_void) -> libc::c_int;
}


/*
guint g_queue_remove_all() [unsigned int]
	(GQueue *) queue [struct _GQueue *]
	(gconstpointer) data [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_remove_all(queue: *mut _GQueue, data: *const libc::c_void) -> libc::c_uint;
}


/*
void g_queue_insert_before()
	(GQueue *) queue [struct _GQueue *]
	(GList *) sibling [struct _GList *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_insert_before(queue: *mut _GQueue, sibling: *mut _GList, data: *mut libc::c_void);
}


/*
void g_queue_insert_after()
	(GQueue *) queue [struct _GQueue *]
	(GList *) sibling [struct _GList *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_insert_after(queue: *mut _GQueue, sibling: *mut _GList, data: *mut libc::c_void);
}


/*
void g_queue_insert_sorted()
	(GQueue *) queue [struct _GQueue *]
	(gpointer) data [void *]
	(GCompareDataFunc) func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_insert_sorted(queue: *mut _GQueue, data: *mut libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
void g_queue_push_head_link()
	(GQueue *) queue [struct _GQueue *]
	(GList *) link_ [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_push_head_link(queue: *mut _GQueue, link_: *mut _GList);
}


/*
void g_queue_push_tail_link()
	(GQueue *) queue [struct _GQueue *]
	(GList *) link_ [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_push_tail_link(queue: *mut _GQueue, link_: *mut _GList);
}


/*
void g_queue_push_nth_link()
	(GQueue *) queue [struct _GQueue *]
	(gint) n [int]
	(GList *) link_ [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_push_nth_link(queue: *mut _GQueue, n: libc::c_int, link_: *mut _GList);
}


/*
GList * g_queue_pop_head_link() [struct _GList *]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_pop_head_link(queue: *mut _GQueue) -> *mut _GList;
}


/*
GList * g_queue_pop_tail_link() [struct _GList *]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_pop_tail_link(queue: *mut _GQueue) -> *mut _GList;
}


/*
GList * g_queue_pop_nth_link() [struct _GList *]
	(GQueue *) queue [struct _GQueue *]
	(guint) n [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_pop_nth_link(queue: *mut _GQueue, n: libc::c_uint) -> *mut _GList;
}


/*
GList * g_queue_peek_head_link() [struct _GList *]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_peek_head_link(queue: *mut _GQueue) -> *mut _GList;
}


/*
GList * g_queue_peek_tail_link() [struct _GList *]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_peek_tail_link(queue: *mut _GQueue) -> *mut _GList;
}


/*
GList * g_queue_peek_nth_link() [struct _GList *]
	(GQueue *) queue [struct _GQueue *]
	(guint) n [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_peek_nth_link(queue: *mut _GQueue, n: libc::c_uint) -> *mut _GList;
}


/*
gint g_queue_link_index() [int]
	(GQueue *) queue [struct _GQueue *]
	(GList *) link_ [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_link_index(queue: *mut _GQueue, link_: *mut _GList) -> libc::c_int;
}


/*
void g_queue_unlink()
	(GQueue *) queue [struct _GQueue *]
	(GList *) link_ [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_unlink(queue: *mut _GQueue, link_: *mut _GList);
}


/*
void g_queue_delete_link()
	(GQueue *) queue [struct _GQueue *]
	(GList *) link_ [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_queue_delete_link(queue: *mut _GQueue, link_: *mut _GList);
}


/*
GRand * g_rand_new_with_seed() [struct _GRand *]
	(guint32) seed [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rand_new_with_seed(seed: libc::c_uint) -> *mut _GRand;
}


/*
GRand * g_rand_new_with_seed_array() [struct _GRand *]
	(const guint32 *) seed [const unsigned int *]
	(guint) seed_length [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rand_new_with_seed_array(seed: *const libc::c_uint, seed_length: libc::c_uint) -> *mut _GRand;
}


/*
GRand * g_rand_new() [struct _GRand *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rand_new() -> *mut _GRand;
}


/*
void g_rand_free()
	(GRand *) rand_ [struct _GRand *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rand_free(rand_: *mut _GRand);
}


/*
GRand * g_rand_copy() [struct _GRand *]
	(GRand *) rand_ [struct _GRand *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rand_copy(rand_: *mut _GRand) -> *mut _GRand;
}


/*
void g_rand_set_seed()
	(GRand *) rand_ [struct _GRand *]
	(guint32) seed [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rand_set_seed(rand_: *mut _GRand, seed: libc::c_uint);
}


/*
void g_rand_set_seed_array()
	(GRand *) rand_ [struct _GRand *]
	(const guint32 *) seed [const unsigned int *]
	(guint) seed_length [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rand_set_seed_array(rand_: *mut _GRand, seed: *const libc::c_uint, seed_length: libc::c_uint);
}


/*
guint32 g_rand_int() [unsigned int]
	(GRand *) rand_ [struct _GRand *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rand_int(rand_: *mut _GRand) -> libc::c_uint;
}


/*
gint32 g_rand_int_range() [int]
	(GRand *) rand_ [struct _GRand *]
	(gint32) begin [int]
	(gint32) end [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rand_int_range(rand_: *mut _GRand, begin: libc::c_int, end: libc::c_int) -> libc::c_int;
}


/*
gdouble g_rand_double() [double]
	(GRand *) rand_ [struct _GRand *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rand_double(rand_: *mut _GRand) -> TypeKind.DOUBLE;
}


/*
gdouble g_rand_double_range() [double]
	(GRand *) rand_ [struct _GRand *]
	(gdouble) begin [double]
	(gdouble) end [double]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_rand_double_range(rand_: *mut _GRand, begin: TypeKind.DOUBLE, end: TypeKind.DOUBLE) -> TypeKind.DOUBLE;
}


/*
void g_random_set_seed()
	(guint32) seed [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_random_set_seed(seed: libc::c_uint);
}


/*
guint32 g_random_int() [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_random_int() -> libc::c_uint;
}


/*
gint32 g_random_int_range() [int]
	(gint32) begin [int]
	(gint32) end [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_random_int_range(begin: libc::c_int, end: libc::c_int) -> libc::c_int;
}


/*
gdouble g_random_double() [double]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_random_double() -> TypeKind.DOUBLE;
}


/*
gdouble g_random_double_range() [double]
	(gdouble) begin [double]
	(gdouble) end [double]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_random_double_range(begin: TypeKind.DOUBLE, end: TypeKind.DOUBLE) -> TypeKind.DOUBLE;
}


/*
GQuark g_regex_error_quark() [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_error_quark() -> libc::c_uint;
}


/*
GRegex * g_regex_new() [struct _GRegex *]
	(const gchar *) pattern [const char *]
	(GRegexCompileFlags) compile_options [GRegexCompileFlags]
	(GRegexMatchFlags) match_options [GRegexMatchFlags]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_new(pattern: *const libc::c_char, compile_options: libc::c_uint, match_options: libc::c_uint, error: *mut *mut _GError) -> *mut _GRegex;
}


/*
GRegex * g_regex_ref() [struct _GRegex *]
	(GRegex *) regex [struct _GRegex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_ref(regex: *mut _GRegex) -> *mut _GRegex;
}


/*
void g_regex_unref()
	(GRegex *) regex [struct _GRegex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_unref(regex: *mut _GRegex);
}


/*
const gchar * g_regex_get_pattern() [const char *]
	(const GRegex *) regex [const struct _GRegex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_get_pattern(regex: *const _GRegex) -> *const libc::c_char;
}


/*
gint g_regex_get_max_backref() [int]
	(const GRegex *) regex [const struct _GRegex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_get_max_backref(regex: *const _GRegex) -> libc::c_int;
}


/*
gint g_regex_get_capture_count() [int]
	(const GRegex *) regex [const struct _GRegex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_get_capture_count(regex: *const _GRegex) -> libc::c_int;
}


/*
gint g_regex_get_string_number() [int]
	(const GRegex *) regex [const struct _GRegex *]
	(const gchar *) name [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_get_string_number(regex: *const _GRegex, name: *const libc::c_char) -> libc::c_int;
}


/*
gchar * g_regex_escape_string() [char *]
	(const gchar *) string [const char *]
	(gint) length [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_escape_string(string: *const libc::c_char, length: libc::c_int) -> *mut libc::c_char;
}


/*
gchar * g_regex_escape_nul() [char *]
	(const gchar *) string [const char *]
	(gint) length [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_escape_nul(string: *const libc::c_char, length: libc::c_int) -> *mut libc::c_char;
}


/*
GRegexCompileFlags g_regex_get_compile_flags() [GRegexCompileFlags]
	(const GRegex *) regex [const struct _GRegex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_get_compile_flags(regex: *const _GRegex) -> libc::c_uint;
}


/*
GRegexMatchFlags g_regex_get_match_flags() [GRegexMatchFlags]
	(const GRegex *) regex [const struct _GRegex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_get_match_flags(regex: *const _GRegex) -> libc::c_uint;
}


/*
gboolean g_regex_match_simple() [int]
	(const gchar *) pattern [const char *]
	(const gchar *) string [const char *]
	(GRegexCompileFlags) compile_options [GRegexCompileFlags]
	(GRegexMatchFlags) match_options [GRegexMatchFlags]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_match_simple(pattern: *const libc::c_char, string: *const libc::c_char, compile_options: libc::c_uint, match_options: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_regex_match() [int]
	(const GRegex *) regex [const struct _GRegex *]
	(const gchar *) string [const char *]
	(GRegexMatchFlags) match_options [GRegexMatchFlags]
	(GMatchInfo **) match_info [struct _GMatchInfo **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_match(regex: *const _GRegex, string: *const libc::c_char, match_options: libc::c_uint, match_info: *mut *mut _GMatchInfo) -> libc::c_int;
}


/*
gboolean g_regex_match_full() [int]
	(const GRegex *) regex [const struct _GRegex *]
	(const gchar *) string [const char *]
	(gssize) string_len [long]
	(gint) start_position [int]
	(GRegexMatchFlags) match_options [GRegexMatchFlags]
	(GMatchInfo **) match_info [struct _GMatchInfo **]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_match_full(regex: *const _GRegex, string: *const libc::c_char, string_len: libc::c_long, start_position: libc::c_int, match_options: libc::c_uint, match_info: *mut *mut _GMatchInfo, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_regex_match_all() [int]
	(const GRegex *) regex [const struct _GRegex *]
	(const gchar *) string [const char *]
	(GRegexMatchFlags) match_options [GRegexMatchFlags]
	(GMatchInfo **) match_info [struct _GMatchInfo **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_match_all(regex: *const _GRegex, string: *const libc::c_char, match_options: libc::c_uint, match_info: *mut *mut _GMatchInfo) -> libc::c_int;
}


/*
gboolean g_regex_match_all_full() [int]
	(const GRegex *) regex [const struct _GRegex *]
	(const gchar *) string [const char *]
	(gssize) string_len [long]
	(gint) start_position [int]
	(GRegexMatchFlags) match_options [GRegexMatchFlags]
	(GMatchInfo **) match_info [struct _GMatchInfo **]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_match_all_full(regex: *const _GRegex, string: *const libc::c_char, string_len: libc::c_long, start_position: libc::c_int, match_options: libc::c_uint, match_info: *mut *mut _GMatchInfo, error: *mut *mut _GError) -> libc::c_int;
}


/*
gchar ** g_regex_split_simple() [char **]
	(const gchar *) pattern [const char *]
	(const gchar *) string [const char *]
	(GRegexCompileFlags) compile_options [GRegexCompileFlags]
	(GRegexMatchFlags) match_options [GRegexMatchFlags]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_split_simple(pattern: *const libc::c_char, string: *const libc::c_char, compile_options: libc::c_uint, match_options: libc::c_uint) -> *mut *mut libc::c_char;
}


/*
gchar ** g_regex_split() [char **]
	(const GRegex *) regex [const struct _GRegex *]
	(const gchar *) string [const char *]
	(GRegexMatchFlags) match_options [GRegexMatchFlags]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_split(regex: *const _GRegex, string: *const libc::c_char, match_options: libc::c_uint) -> *mut *mut libc::c_char;
}


/*
gchar ** g_regex_split_full() [char **]
	(const GRegex *) regex [const struct _GRegex *]
	(const gchar *) string [const char *]
	(gssize) string_len [long]
	(gint) start_position [int]
	(GRegexMatchFlags) match_options [GRegexMatchFlags]
	(gint) max_tokens [int]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_split_full(regex: *const _GRegex, string: *const libc::c_char, string_len: libc::c_long, start_position: libc::c_int, match_options: libc::c_uint, max_tokens: libc::c_int, error: *mut *mut _GError) -> *mut *mut libc::c_char;
}


/*
gchar * g_regex_replace() [char *]
	(const GRegex *) regex [const struct _GRegex *]
	(const gchar *) string [const char *]
	(gssize) string_len [long]
	(gint) start_position [int]
	(const gchar *) replacement [const char *]
	(GRegexMatchFlags) match_options [GRegexMatchFlags]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_replace(regex: *const _GRegex, string: *const libc::c_char, string_len: libc::c_long, start_position: libc::c_int, replacement: *const libc::c_char, match_options: libc::c_uint, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gchar * g_regex_replace_literal() [char *]
	(const GRegex *) regex [const struct _GRegex *]
	(const gchar *) string [const char *]
	(gssize) string_len [long]
	(gint) start_position [int]
	(const gchar *) replacement [const char *]
	(GRegexMatchFlags) match_options [GRegexMatchFlags]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_replace_literal(regex: *const _GRegex, string: *const libc::c_char, string_len: libc::c_long, start_position: libc::c_int, replacement: *const libc::c_char, match_options: libc::c_uint, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gchar * g_regex_replace_eval() [char *]
	(const GRegex *) regex [const struct _GRegex *]
	(const gchar *) string [const char *]
	(gssize) string_len [long]
	(gint) start_position [int]
	(GRegexMatchFlags) match_options [GRegexMatchFlags]
	(GRegexEvalCallback) eval [int (*)(const struct _GMatchInfo *, struct _GString *, void *)]
	(gpointer) user_data [void *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_replace_eval(regex: *const _GRegex, string: *const libc::c_char, string_len: libc::c_long, start_position: libc::c_int, match_options: libc::c_uint, eval: Option<extern fn(*const _GMatchInfo, *mut _GString, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gboolean g_regex_check_replacement() [int]
	(const gchar *) replacement [const char *]
	(gboolean *) has_references [int *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_regex_check_replacement(replacement: *const libc::c_char, has_references: *mut libc::c_int, error: *mut *mut _GError) -> libc::c_int;
}


/*
GRegex * g_match_info_get_regex() [struct _GRegex *]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_match_info_get_regex(match_info: *const _GMatchInfo) -> *mut _GRegex;
}


/*
const gchar * g_match_info_get_string() [const char *]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_match_info_get_string(match_info: *const _GMatchInfo) -> *const libc::c_char;
}


/*
GMatchInfo * g_match_info_ref() [struct _GMatchInfo *]
	(GMatchInfo *) match_info [struct _GMatchInfo *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_match_info_ref(match_info: *mut _GMatchInfo) -> *mut _GMatchInfo;
}


/*
void g_match_info_unref()
	(GMatchInfo *) match_info [struct _GMatchInfo *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_match_info_unref(match_info: *mut _GMatchInfo);
}


/*
void g_match_info_free()
	(GMatchInfo *) match_info [struct _GMatchInfo *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_match_info_free(match_info: *mut _GMatchInfo);
}


/*
gboolean g_match_info_next() [int]
	(GMatchInfo *) match_info [struct _GMatchInfo *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_match_info_next(match_info: *mut _GMatchInfo, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_match_info_matches() [int]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_match_info_matches(match_info: *const _GMatchInfo) -> libc::c_int;
}


/*
gint g_match_info_get_match_count() [int]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_match_info_get_match_count(match_info: *const _GMatchInfo) -> libc::c_int;
}


/*
gboolean g_match_info_is_partial_match() [int]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_match_info_is_partial_match(match_info: *const _GMatchInfo) -> libc::c_int;
}


/*
gchar * g_match_info_expand_references() [char *]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
	(const gchar *) string_to_expand [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_match_info_expand_references(match_info: *const _GMatchInfo, string_to_expand: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gchar * g_match_info_fetch() [char *]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
	(gint) match_num [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_match_info_fetch(match_info: *const _GMatchInfo, match_num: libc::c_int) -> *mut libc::c_char;
}


/*
gboolean g_match_info_fetch_pos() [int]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
	(gint) match_num [int]
	(gint *) start_pos [int *]
	(gint *) end_pos [int *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_match_info_fetch_pos(match_info: *const _GMatchInfo, match_num: libc::c_int, start_pos: *mut libc::c_int, end_pos: *mut libc::c_int) -> libc::c_int;
}


/*
gchar * g_match_info_fetch_named() [char *]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
	(const gchar *) name [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_match_info_fetch_named(match_info: *const _GMatchInfo, name: *const libc::c_char) -> *mut libc::c_char;
}


/*
gboolean g_match_info_fetch_named_pos() [int]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
	(const gchar *) name [const char *]
	(gint *) start_pos [int *]
	(gint *) end_pos [int *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_match_info_fetch_named_pos(match_info: *const _GMatchInfo, name: *const libc::c_char, start_pos: *mut libc::c_int, end_pos: *mut libc::c_int) -> libc::c_int;
}


/*
gchar ** g_match_info_fetch_all() [char **]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_match_info_fetch_all(match_info: *const _GMatchInfo) -> *mut *mut libc::c_char;
}


/*
GScanner * g_scanner_new() [struct _GScanner *]
	(const GScannerConfig *) config_templ [const struct _GScannerConfig *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_new(config_templ: *const _GScannerConfig) -> *mut _GScanner;
}


/*
void g_scanner_destroy()
	(GScanner *) scanner [struct _GScanner *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_destroy(scanner: *mut _GScanner);
}


/*
void g_scanner_input_file()
	(GScanner *) scanner [struct _GScanner *]
	(gint) input_fd [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_input_file(scanner: *mut _GScanner, input_fd: libc::c_int);
}


/*
void g_scanner_sync_file_offset()
	(GScanner *) scanner [struct _GScanner *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_sync_file_offset(scanner: *mut _GScanner);
}


/*
void g_scanner_input_text()
	(GScanner *) scanner [struct _GScanner *]
	(const gchar *) text [const char *]
	(guint) text_len [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_input_text(scanner: *mut _GScanner, text: *const libc::c_char, text_len: libc::c_uint);
}


/*
GTokenType g_scanner_get_next_token() [GTokenType]
	(GScanner *) scanner [struct _GScanner *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_get_next_token(scanner: *mut _GScanner) -> libc::c_uint;
}


/*
GTokenType g_scanner_peek_next_token() [GTokenType]
	(GScanner *) scanner [struct _GScanner *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_peek_next_token(scanner: *mut _GScanner) -> libc::c_uint;
}


/*
GTokenType g_scanner_cur_token() [GTokenType]
	(GScanner *) scanner [struct _GScanner *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_cur_token(scanner: *mut _GScanner) -> libc::c_uint;
}


/*
GTokenValue g_scanner_cur_value() [union _GTokenValue]
	(GScanner *) scanner [struct _GScanner *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_cur_value(scanner: *mut _GScanner) -> union _GTokenValue;
}


/*
guint g_scanner_cur_line() [unsigned int]
	(GScanner *) scanner [struct _GScanner *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_cur_line(scanner: *mut _GScanner) -> libc::c_uint;
}


/*
guint g_scanner_cur_position() [unsigned int]
	(GScanner *) scanner [struct _GScanner *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_cur_position(scanner: *mut _GScanner) -> libc::c_uint;
}


/*
gboolean g_scanner_eof() [int]
	(GScanner *) scanner [struct _GScanner *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_eof(scanner: *mut _GScanner) -> libc::c_int;
}


/*
guint g_scanner_set_scope() [unsigned int]
	(GScanner *) scanner [struct _GScanner *]
	(guint) scope_id [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_set_scope(scanner: *mut _GScanner, scope_id: libc::c_uint) -> libc::c_uint;
}


/*
void g_scanner_scope_add_symbol()
	(GScanner *) scanner [struct _GScanner *]
	(guint) scope_id [unsigned int]
	(const gchar *) symbol [const char *]
	(gpointer) value [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_scope_add_symbol(scanner: *mut _GScanner, scope_id: libc::c_uint, symbol: *const libc::c_char, value: *mut libc::c_void);
}


/*
void g_scanner_scope_remove_symbol()
	(GScanner *) scanner [struct _GScanner *]
	(guint) scope_id [unsigned int]
	(const gchar *) symbol [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_scope_remove_symbol(scanner: *mut _GScanner, scope_id: libc::c_uint, symbol: *const libc::c_char);
}


/*
gpointer g_scanner_scope_lookup_symbol() [void *]
	(GScanner *) scanner [struct _GScanner *]
	(guint) scope_id [unsigned int]
	(const gchar *) symbol [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_scope_lookup_symbol(scanner: *mut _GScanner, scope_id: libc::c_uint, symbol: *const libc::c_char) -> *mut libc::c_void;
}


/*
void g_scanner_scope_foreach_symbol()
	(GScanner *) scanner [struct _GScanner *]
	(guint) scope_id [unsigned int]
	(GHFunc) func [void (*)(void *, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_scope_foreach_symbol(scanner: *mut _GScanner, scope_id: libc::c_uint, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
gpointer g_scanner_lookup_symbol() [void *]
	(GScanner *) scanner [struct _GScanner *]
	(const gchar *) symbol [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_lookup_symbol(scanner: *mut _GScanner, symbol: *const libc::c_char) -> *mut libc::c_void;
}


/*
void g_scanner_unexp_token()
	(GScanner *) scanner [struct _GScanner *]
	(GTokenType) expected_token [GTokenType]
	(const gchar *) identifier_spec [const char *]
	(const gchar *) symbol_spec [const char *]
	(const gchar *) symbol_name [const char *]
	(const gchar *) message [const char *]
	(gint) is_error [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_unexp_token(scanner: *mut _GScanner, expected_token: libc::c_uint, identifier_spec: *const libc::c_char, symbol_spec: *const libc::c_char, symbol_name: *const libc::c_char, message: *const libc::c_char, is_error: libc::c_int);
}


/*
void g_scanner_error()
	(GScanner *) scanner [struct _GScanner *]
	(const gchar *) format [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_error(scanner: *mut _GScanner, format: *const libc::c_char);
}


/*
void g_scanner_warn()
	(GScanner *) scanner [struct _GScanner *]
	(const gchar *) format [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_scanner_warn(scanner: *mut _GScanner, format: *const libc::c_char);
}


/*
GSequence * g_sequence_new() [struct _GSequence *]
	(GDestroyNotify) data_destroy [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_new(data_destroy: Option<extern fn(*mut libc::c_void)>) -> *mut _GSequence;
}


/*
void g_sequence_free()
	(GSequence *) seq [struct _GSequence *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_free(seq: *mut _GSequence);
}


/*
gint g_sequence_get_length() [int]
	(GSequence *) seq [struct _GSequence *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_get_length(seq: *mut _GSequence) -> libc::c_int;
}


/*
void g_sequence_foreach()
	(GSequence *) seq [struct _GSequence *]
	(GFunc) func [void (*)(void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_foreach(seq: *mut _GSequence, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
void g_sequence_foreach_range()
	(GSequenceIter *) begin [struct _GSequenceNode *]
	(GSequenceIter *) end [struct _GSequenceNode *]
	(GFunc) func [void (*)(void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_foreach_range(begin: *mut _GSequenceNode, end: *mut _GSequenceNode, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
void g_sequence_sort()
	(GSequence *) seq [struct _GSequence *]
	(GCompareDataFunc) cmp_func [int (*)(const void *, const void *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_sort(seq: *mut _GSequence, cmp_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void);
}


/*
void g_sequence_sort_iter()
	(GSequence *) seq [struct _GSequence *]
	(GSequenceIterCompareFunc) cmp_func [int (*)(struct _GSequenceNode *, struct _GSequenceNode *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_sort_iter(seq: *mut _GSequence, cmp_func: Option<extern fn(*mut _GSequenceNode, *mut _GSequenceNode, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void);
}


/*
GSequenceIter * g_sequence_get_begin_iter() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_get_begin_iter(seq: *mut _GSequence) -> *mut _GSequenceNode;
}


/*
GSequenceIter * g_sequence_get_end_iter() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_get_end_iter(seq: *mut _GSequence) -> *mut _GSequenceNode;
}


/*
GSequenceIter * g_sequence_get_iter_at_pos() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
	(gint) pos [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_get_iter_at_pos(seq: *mut _GSequence, pos: libc::c_int) -> *mut _GSequenceNode;
}


/*
GSequenceIter * g_sequence_append() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_append(seq: *mut _GSequence, data: *mut libc::c_void) -> *mut _GSequenceNode;
}


/*
GSequenceIter * g_sequence_prepend() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_prepend(seq: *mut _GSequence, data: *mut libc::c_void) -> *mut _GSequenceNode;
}


/*
GSequenceIter * g_sequence_insert_before() [struct _GSequenceNode *]
	(GSequenceIter *) iter [struct _GSequenceNode *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_insert_before(iter: *mut _GSequenceNode, data: *mut libc::c_void) -> *mut _GSequenceNode;
}


/*
void g_sequence_move()
	(GSequenceIter *) src [struct _GSequenceNode *]
	(GSequenceIter *) dest [struct _GSequenceNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_move(src: *mut _GSequenceNode, dest: *mut _GSequenceNode);
}


/*
void g_sequence_swap()
	(GSequenceIter *) a [struct _GSequenceNode *]
	(GSequenceIter *) b [struct _GSequenceNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_swap(a: *mut _GSequenceNode, b: *mut _GSequenceNode);
}


/*
GSequenceIter * g_sequence_insert_sorted() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
	(gpointer) data [void *]
	(GCompareDataFunc) cmp_func [int (*)(const void *, const void *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_insert_sorted(seq: *mut _GSequence, data: *mut libc::c_void, cmp_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void) -> *mut _GSequenceNode;
}


/*
GSequenceIter * g_sequence_insert_sorted_iter() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
	(gpointer) data [void *]
	(GSequenceIterCompareFunc) iter_cmp [int (*)(struct _GSequenceNode *, struct _GSequenceNode *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_insert_sorted_iter(seq: *mut _GSequence, data: *mut libc::c_void, iter_cmp: Option<extern fn(*mut _GSequenceNode, *mut _GSequenceNode, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void) -> *mut _GSequenceNode;
}


/*
void g_sequence_sort_changed()
	(GSequenceIter *) iter [struct _GSequenceNode *]
	(GCompareDataFunc) cmp_func [int (*)(const void *, const void *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_sort_changed(iter: *mut _GSequenceNode, cmp_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void);
}


/*
void g_sequence_sort_changed_iter()
	(GSequenceIter *) iter [struct _GSequenceNode *]
	(GSequenceIterCompareFunc) iter_cmp [int (*)(struct _GSequenceNode *, struct _GSequenceNode *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_sort_changed_iter(iter: *mut _GSequenceNode, iter_cmp: Option<extern fn(*mut _GSequenceNode, *mut _GSequenceNode, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void);
}


/*
void g_sequence_remove()
	(GSequenceIter *) iter [struct _GSequenceNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_remove(iter: *mut _GSequenceNode);
}


/*
void g_sequence_remove_range()
	(GSequenceIter *) begin [struct _GSequenceNode *]
	(GSequenceIter *) end [struct _GSequenceNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_remove_range(begin: *mut _GSequenceNode, end: *mut _GSequenceNode);
}


/*
void g_sequence_move_range()
	(GSequenceIter *) dest [struct _GSequenceNode *]
	(GSequenceIter *) begin [struct _GSequenceNode *]
	(GSequenceIter *) end [struct _GSequenceNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_move_range(dest: *mut _GSequenceNode, begin: *mut _GSequenceNode, end: *mut _GSequenceNode);
}


/*
GSequenceIter * g_sequence_search() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
	(gpointer) data [void *]
	(GCompareDataFunc) cmp_func [int (*)(const void *, const void *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_search(seq: *mut _GSequence, data: *mut libc::c_void, cmp_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void) -> *mut _GSequenceNode;
}


/*
GSequenceIter * g_sequence_search_iter() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
	(gpointer) data [void *]
	(GSequenceIterCompareFunc) iter_cmp [int (*)(struct _GSequenceNode *, struct _GSequenceNode *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_search_iter(seq: *mut _GSequence, data: *mut libc::c_void, iter_cmp: Option<extern fn(*mut _GSequenceNode, *mut _GSequenceNode, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void) -> *mut _GSequenceNode;
}


/*
GSequenceIter * g_sequence_lookup() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
	(gpointer) data [void *]
	(GCompareDataFunc) cmp_func [int (*)(const void *, const void *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_lookup(seq: *mut _GSequence, data: *mut libc::c_void, cmp_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void) -> *mut _GSequenceNode;
}


/*
GSequenceIter * g_sequence_lookup_iter() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
	(gpointer) data [void *]
	(GSequenceIterCompareFunc) iter_cmp [int (*)(struct _GSequenceNode *, struct _GSequenceNode *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_lookup_iter(seq: *mut _GSequence, data: *mut libc::c_void, iter_cmp: Option<extern fn(*mut _GSequenceNode, *mut _GSequenceNode, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void) -> *mut _GSequenceNode;
}


/*
gpointer g_sequence_get() [void *]
	(GSequenceIter *) iter [struct _GSequenceNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_get(iter: *mut _GSequenceNode) -> *mut libc::c_void;
}


/*
void g_sequence_set()
	(GSequenceIter *) iter [struct _GSequenceNode *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_set(iter: *mut _GSequenceNode, data: *mut libc::c_void);
}


/*
gboolean g_sequence_iter_is_begin() [int]
	(GSequenceIter *) iter [struct _GSequenceNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_iter_is_begin(iter: *mut _GSequenceNode) -> libc::c_int;
}


/*
gboolean g_sequence_iter_is_end() [int]
	(GSequenceIter *) iter [struct _GSequenceNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_iter_is_end(iter: *mut _GSequenceNode) -> libc::c_int;
}


/*
GSequenceIter * g_sequence_iter_next() [struct _GSequenceNode *]
	(GSequenceIter *) iter [struct _GSequenceNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_iter_next(iter: *mut _GSequenceNode) -> *mut _GSequenceNode;
}


/*
GSequenceIter * g_sequence_iter_prev() [struct _GSequenceNode *]
	(GSequenceIter *) iter [struct _GSequenceNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_iter_prev(iter: *mut _GSequenceNode) -> *mut _GSequenceNode;
}


/*
gint g_sequence_iter_get_position() [int]
	(GSequenceIter *) iter [struct _GSequenceNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_iter_get_position(iter: *mut _GSequenceNode) -> libc::c_int;
}


/*
GSequenceIter * g_sequence_iter_move() [struct _GSequenceNode *]
	(GSequenceIter *) iter [struct _GSequenceNode *]
	(gint) delta [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_iter_move(iter: *mut _GSequenceNode, delta: libc::c_int) -> *mut _GSequenceNode;
}


/*
GSequence * g_sequence_iter_get_sequence() [struct _GSequence *]
	(GSequenceIter *) iter [struct _GSequenceNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_iter_get_sequence(iter: *mut _GSequenceNode) -> *mut _GSequence;
}


/*
gint g_sequence_iter_compare() [int]
	(GSequenceIter *) a [struct _GSequenceNode *]
	(GSequenceIter *) b [struct _GSequenceNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_iter_compare(a: *mut _GSequenceNode, b: *mut _GSequenceNode) -> libc::c_int;
}


/*
GSequenceIter * g_sequence_range_get_midpoint() [struct _GSequenceNode *]
	(GSequenceIter *) begin [struct _GSequenceNode *]
	(GSequenceIter *) end [struct _GSequenceNode *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_sequence_range_get_midpoint(begin: *mut _GSequenceNode, end: *mut _GSequenceNode) -> *mut _GSequenceNode;
}


/*
GQuark g_shell_error_quark() [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_shell_error_quark() -> libc::c_uint;
}


/*
gchar * g_shell_quote() [char *]
	(const gchar *) unquoted_string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_shell_quote(unquoted_string: *const libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_shell_unquote() [char *]
	(const gchar *) quoted_string [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_shell_unquote(quoted_string: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gboolean g_shell_parse_argv() [int]
	(const gchar *) command_line [const char *]
	(gint *) argcp [int *]
	(gchar ***) argvp [char ***]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_shell_parse_argv(command_line: *const libc::c_char, argcp: *mut libc::c_int, argvp: *mut *mut *mut libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
gpointer g_slice_alloc() [void *]
	(gsize) block_size [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slice_alloc(block_size: libc::c_ulong) -> *mut libc::c_void;
}


/*
gpointer g_slice_alloc0() [void *]
	(gsize) block_size [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slice_alloc0(block_size: libc::c_ulong) -> *mut libc::c_void;
}


/*
gpointer g_slice_copy() [void *]
	(gsize) block_size [unsigned long]
	(gconstpointer) mem_block [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slice_copy(block_size: libc::c_ulong, mem_block: *const libc::c_void) -> *mut libc::c_void;
}


/*
void g_slice_free1()
	(gsize) block_size [unsigned long]
	(gpointer) mem_block [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slice_free1(block_size: libc::c_ulong, mem_block: *mut libc::c_void);
}


/*
void g_slice_free_chain_with_offset()
	(gsize) block_size [unsigned long]
	(gpointer) mem_chain [void *]
	(gsize) next_offset [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slice_free_chain_with_offset(block_size: libc::c_ulong, mem_chain: *mut libc::c_void, next_offset: libc::c_ulong);
}


/*
void g_slice_set_config()
	(GSliceConfig) ckey [GSliceConfig]
	(gint64) value [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slice_set_config(ckey: libc::c_uint, value: libc::c_long);
}


/*
gint64 g_slice_get_config() [long]
	(GSliceConfig) ckey [GSliceConfig]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slice_get_config(ckey: libc::c_uint) -> libc::c_long;
}


/*
gint64 * g_slice_get_config_state() [long *]
	(GSliceConfig) ckey [GSliceConfig]
	(gint64) address [long]
	(guint *) n_values [unsigned int *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slice_get_config_state(ckey: libc::c_uint, address: libc::c_long, n_values: *mut libc::c_uint) -> *mut libc::c_long;
}


/*
GQuark g_spawn_error_quark() [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_spawn_error_quark() -> libc::c_uint;
}


/*
gboolean g_spawn_async() [int]
	(const gchar *) working_directory [const char *]
	(gchar **) argv [char **]
	(gchar **) envp [char **]
	(GSpawnFlags) flags [GSpawnFlags]
	(GSpawnChildSetupFunc) child_setup [void (*)(void *)]
	(gpointer) user_data [void *]
	(GPid *) child_pid [int *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_spawn_async(working_directory: *const libc::c_char, argv: *mut *mut libc::c_char, envp: *mut *mut libc::c_char, flags: libc::c_uint, child_setup: Option<extern fn(*mut libc::c_void)>, user_data: *mut libc::c_void, child_pid: *mut libc::c_int, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_spawn_async_with_pipes() [int]
	(const gchar *) working_directory [const char *]
	(gchar **) argv [char **]
	(gchar **) envp [char **]
	(GSpawnFlags) flags [GSpawnFlags]
	(GSpawnChildSetupFunc) child_setup [void (*)(void *)]
	(gpointer) user_data [void *]
	(GPid *) child_pid [int *]
	(gint *) standard_input [int *]
	(gint *) standard_output [int *]
	(gint *) standard_error [int *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_spawn_async_with_pipes(working_directory: *const libc::c_char, argv: *mut *mut libc::c_char, envp: *mut *mut libc::c_char, flags: libc::c_uint, child_setup: Option<extern fn(*mut libc::c_void)>, user_data: *mut libc::c_void, child_pid: *mut libc::c_int, standard_input: *mut libc::c_int, standard_output: *mut libc::c_int, standard_error: *mut libc::c_int, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_spawn_sync() [int]
	(const gchar *) working_directory [const char *]
	(gchar **) argv [char **]
	(gchar **) envp [char **]
	(GSpawnFlags) flags [GSpawnFlags]
	(GSpawnChildSetupFunc) child_setup [void (*)(void *)]
	(gpointer) user_data [void *]
	(gchar **) standard_output [char **]
	(gchar **) standard_error [char **]
	(gint *) exit_status [int *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_spawn_sync(working_directory: *const libc::c_char, argv: *mut *mut libc::c_char, envp: *mut *mut libc::c_char, flags: libc::c_uint, child_setup: Option<extern fn(*mut libc::c_void)>, user_data: *mut libc::c_void, standard_output: *mut *mut libc::c_char, standard_error: *mut *mut libc::c_char, exit_status: *mut libc::c_int, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_spawn_command_line_sync() [int]
	(const gchar *) command_line [const char *]
	(gchar **) standard_output [char **]
	(gchar **) standard_error [char **]
	(gint *) exit_status [int *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_spawn_command_line_sync(command_line: *const libc::c_char, standard_output: *mut *mut libc::c_char, standard_error: *mut *mut libc::c_char, exit_status: *mut libc::c_int, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_spawn_command_line_async() [int]
	(const gchar *) command_line [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_spawn_command_line_async(command_line: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_spawn_close_pid()
	(GPid) pid [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_spawn_close_pid(pid: libc::c_int);
}


/*
gchar g_ascii_tolower() [char]
	(gchar) c [char]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ascii_tolower(c: libc::c_char) -> libc::c_char;
}


/*
gchar g_ascii_toupper() [char]
	(gchar) c [char]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ascii_toupper(c: libc::c_char) -> libc::c_char;
}


/*
gint g_ascii_digit_value() [int]
	(gchar) c [char]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ascii_digit_value(c: libc::c_char) -> libc::c_int;
}


/*
gint g_ascii_xdigit_value() [int]
	(gchar) c [char]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ascii_xdigit_value(c: libc::c_char) -> libc::c_int;
}


/*
gchar * g_strdelimit() [char *]
	(gchar *) string [char *]
	(const gchar *) delimiters [const char *]
	(gchar) new_delimiter [char]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strdelimit(string: *mut libc::c_char, delimiters: *const libc::c_char, new_delimiter: libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_strcanon() [char *]
	(gchar *) string [char *]
	(const gchar *) valid_chars [const char *]
	(gchar) substitutor [char]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strcanon(string: *mut libc::c_char, valid_chars: *const libc::c_char, substitutor: libc::c_char) -> *mut libc::c_char;
}


/*
const gchar * g_strerror() [const char *]
	(gint) errnum [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strerror(errnum: libc::c_int) -> *const libc::c_char;
}


/*
const gchar * g_strsignal() [const char *]
	(gint) signum [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strsignal(signum: libc::c_int) -> *const libc::c_char;
}


/*
gchar * g_strreverse() [char *]
	(gchar *) string [char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strreverse(string: *mut libc::c_char) -> *mut libc::c_char;
}


/*
gsize g_strlcpy() [unsigned long]
	(gchar *) dest [char *]
	(const gchar *) src [const char *]
	(gsize) dest_size [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strlcpy(dest: *mut libc::c_char, src: *const libc::c_char, dest_size: libc::c_ulong) -> libc::c_ulong;
}


/*
gsize g_strlcat() [unsigned long]
	(gchar *) dest [char *]
	(const gchar *) src [const char *]
	(gsize) dest_size [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strlcat(dest: *mut libc::c_char, src: *const libc::c_char, dest_size: libc::c_ulong) -> libc::c_ulong;
}


/*
gchar * g_strstr_len() [char *]
	(const gchar *) haystack [const char *]
	(gssize) haystack_len [long]
	(const gchar *) needle [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strstr_len(haystack: *const libc::c_char, haystack_len: libc::c_long, needle: *const libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_strrstr() [char *]
	(const gchar *) haystack [const char *]
	(const gchar *) needle [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strrstr(haystack: *const libc::c_char, needle: *const libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_strrstr_len() [char *]
	(const gchar *) haystack [const char *]
	(gssize) haystack_len [long]
	(const gchar *) needle [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strrstr_len(haystack: *const libc::c_char, haystack_len: libc::c_long, needle: *const libc::c_char) -> *mut libc::c_char;
}


/*
gboolean g_str_has_suffix() [int]
	(const gchar *) str [const char *]
	(const gchar *) suffix [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_str_has_suffix(str: *const libc::c_char, suffix: *const libc::c_char) -> libc::c_int;
}


/*
gboolean g_str_has_prefix() [int]
	(const gchar *) str [const char *]
	(const gchar *) prefix [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_str_has_prefix(str: *const libc::c_char, prefix: *const libc::c_char) -> libc::c_int;
}


/*
gdouble g_strtod() [double]
	(const gchar *) nptr [const char *]
	(gchar **) endptr [char **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strtod(nptr: *const libc::c_char, endptr: *mut *mut libc::c_char) -> TypeKind.DOUBLE;
}


/*
gdouble g_ascii_strtod() [double]
	(const gchar *) nptr [const char *]
	(gchar **) endptr [char **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ascii_strtod(nptr: *const libc::c_char, endptr: *mut *mut libc::c_char) -> TypeKind.DOUBLE;
}


/*
guint64 g_ascii_strtoull() [unsigned long]
	(const gchar *) nptr [const char *]
	(gchar **) endptr [char **]
	(guint) base [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ascii_strtoull(nptr: *const libc::c_char, endptr: *mut *mut libc::c_char, base: libc::c_uint) -> libc::c_ulong;
}


/*
gint64 g_ascii_strtoll() [long]
	(const gchar *) nptr [const char *]
	(gchar **) endptr [char **]
	(guint) base [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ascii_strtoll(nptr: *const libc::c_char, endptr: *mut *mut libc::c_char, base: libc::c_uint) -> libc::c_long;
}


/*
gchar * g_ascii_dtostr() [char *]
	(gchar *) buffer [char *]
	(gint) buf_len [int]
	(gdouble) d [double]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ascii_dtostr(buffer: *mut libc::c_char, buf_len: libc::c_int, d: TypeKind.DOUBLE) -> *mut libc::c_char;
}


/*
gchar * g_ascii_formatd() [char *]
	(gchar *) buffer [char *]
	(gint) buf_len [int]
	(const gchar *) format [const char *]
	(gdouble) d [double]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ascii_formatd(buffer: *mut libc::c_char, buf_len: libc::c_int, format: *const libc::c_char, d: TypeKind.DOUBLE) -> *mut libc::c_char;
}


/*
gchar * g_strchug() [char *]
	(gchar *) string [char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strchug(string: *mut libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_strchomp() [char *]
	(gchar *) string [char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strchomp(string: *mut libc::c_char) -> *mut libc::c_char;
}


/*
gint g_ascii_strcasecmp() [int]
	(const gchar *) s1 [const char *]
	(const gchar *) s2 [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ascii_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
}


/*
gint g_ascii_strncasecmp() [int]
	(const gchar *) s1 [const char *]
	(const gchar *) s2 [const char *]
	(gsize) n [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ascii_strncasecmp(s1: *const libc::c_char, s2: *const libc::c_char, n: libc::c_ulong) -> libc::c_int;
}


/*
gchar * g_ascii_strdown() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ascii_strdown(str: *const libc::c_char, len: libc::c_long) -> *mut libc::c_char;
}


/*
gchar * g_ascii_strup() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_ascii_strup(str: *const libc::c_char, len: libc::c_long) -> *mut libc::c_char;
}


/*
gint g_strcasecmp() [int]
	(const gchar *) s1 [const char *]
	(const gchar *) s2 [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
}


/*
gint g_strncasecmp() [int]
	(const gchar *) s1 [const char *]
	(const gchar *) s2 [const char *]
	(guint) n [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strncasecmp(s1: *const libc::c_char, s2: *const libc::c_char, n: libc::c_uint) -> libc::c_int;
}


/*
gchar * g_strdown() [char *]
	(gchar *) string [char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strdown(string: *mut libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_strup() [char *]
	(gchar *) string [char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strup(string: *mut libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_strdup() [char *]
	(const gchar *) str [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strdup(str: *const libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_strdup_printf() [char *]
	(const gchar *) format [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strdup_printf(format: *const libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_strdup_vprintf() [char *]
	(const gchar *) format [const char *]
	(va_list) args [struct __va_list_tag [1]]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strdup_vprintf(format: *const libc::c_char, args: [__va_list_tag; 1]) -> *mut libc::c_char;
}


/*
gchar * g_strndup() [char *]
	(const gchar *) str [const char *]
	(gsize) n [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strndup(str: *const libc::c_char, n: libc::c_ulong) -> *mut libc::c_char;
}


/*
gchar * g_strnfill() [char *]
	(gsize) length [unsigned long]
	(gchar) fill_char [char]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strnfill(length: libc::c_ulong, fill_char: libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_strconcat() [char *]
	(const gchar *) string1 [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strconcat(string1: *const libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_strjoin() [char *]
	(const gchar *) separator [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strjoin(separator: *const libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_strcompress() [char *]
	(const gchar *) source [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strcompress(source: *const libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_strescape() [char *]
	(const gchar *) source [const char *]
	(const gchar *) exceptions [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strescape(source: *const libc::c_char, exceptions: *const libc::c_char) -> *mut libc::c_char;
}


/*
gpointer g_memdup() [void *]
	(gconstpointer) mem [const void *]
	(guint) byte_size [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_memdup(mem: *const libc::c_void, byte_size: libc::c_uint) -> *mut libc::c_void;
}


/*
gchar ** g_strsplit() [char **]
	(const gchar *) string [const char *]
	(const gchar *) delimiter [const char *]
	(gint) max_tokens [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strsplit(string: *const libc::c_char, delimiter: *const libc::c_char, max_tokens: libc::c_int) -> *mut *mut libc::c_char;
}


/*
gchar ** g_strsplit_set() [char **]
	(const gchar *) string [const char *]
	(const gchar *) delimiters [const char *]
	(gint) max_tokens [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strsplit_set(string: *const libc::c_char, delimiters: *const libc::c_char, max_tokens: libc::c_int) -> *mut *mut libc::c_char;
}


/*
gchar * g_strjoinv() [char *]
	(const gchar *) separator [const char *]
	(gchar **) str_array [char **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strjoinv(separator: *const libc::c_char, str_array: *mut *mut libc::c_char) -> *mut libc::c_char;
}


/*
void g_strfreev()
	(gchar **) str_array [char **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strfreev(str_array: *mut *mut libc::c_char);
}


/*
gchar ** g_strdupv() [char **]
	(gchar **) str_array [char **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strdupv(str_array: *mut *mut libc::c_char) -> *mut *mut libc::c_char;
}


/*
guint g_strv_length() [unsigned int]
	(gchar **) str_array [char **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strv_length(str_array: *mut *mut libc::c_char) -> libc::c_uint;
}


/*
gchar * g_stpcpy() [char *]
	(gchar *) dest [char *]
	(const char *) src
*/
#[link(name="nice")]
extern "C" {
	pub fn g_stpcpy(dest: *mut libc::c_char, src: *const libc::c_char) -> *mut libc::c_char;
}


/*
GStringChunk * g_string_chunk_new() [struct _GStringChunk *]
	(gsize) size [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_chunk_new(size: libc::c_ulong) -> *mut _GStringChunk;
}


/*
void g_string_chunk_free()
	(GStringChunk *) chunk [struct _GStringChunk *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_chunk_free(chunk: *mut _GStringChunk);
}


/*
void g_string_chunk_clear()
	(GStringChunk *) chunk [struct _GStringChunk *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_chunk_clear(chunk: *mut _GStringChunk);
}


/*
gchar * g_string_chunk_insert() [char *]
	(GStringChunk *) chunk [struct _GStringChunk *]
	(const gchar *) string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_chunk_insert(chunk: *mut _GStringChunk, string: *const libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_string_chunk_insert_len() [char *]
	(GStringChunk *) chunk [struct _GStringChunk *]
	(const gchar *) string [const char *]
	(gssize) len [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_chunk_insert_len(chunk: *mut _GStringChunk, string: *const libc::c_char, len: libc::c_long) -> *mut libc::c_char;
}


/*
gchar * g_string_chunk_insert_const() [char *]
	(GStringChunk *) chunk [struct _GStringChunk *]
	(const gchar *) string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_string_chunk_insert_const(chunk: *mut _GStringChunk, string: *const libc::c_char) -> *mut libc::c_char;
}


/*
int g_strcmp0()
	(const char *) str1
	(const char *) str2
*/
#[link(name="nice")]
extern "C" {
	pub fn g_strcmp0(str1: *const libc::c_char, str2: *const libc::c_char) -> libc::c_int;
}


/*
void g_test_minimized_result()
	(double) minimized_quantity
	(const char *) format
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_minimized_result(minimized_quantity: TypeKind.DOUBLE, format: *const libc::c_char);
}


/*
void g_test_maximized_result()
	(double) maximized_quantity
	(const char *) format
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_maximized_result(maximized_quantity: TypeKind.DOUBLE, format: *const libc::c_char);
}


/*
void g_test_init()
	(int *) argc
	(char ***) argv
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_init(argc: *mut libc::c_int, argv: *mut *mut *mut libc::c_char);
}


/*
int g_test_run()
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_run() -> libc::c_int;
}


/*
void g_test_add_func()
	(const char *) testpath
	(GTestFunc) test_func [void (*)(void)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_add_func(testpath: *const libc::c_char, test_func: Option<extern fn()>);
}


/*
void g_test_add_data_func()
	(const char *) testpath
	(gconstpointer) test_data [const void *]
	(GTestDataFunc) test_func [void (*)(const void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_add_data_func(testpath: *const libc::c_char, test_data: *const libc::c_void, test_func: Option<extern fn(*const libc::c_void)>);
}


/*
void g_test_fail()
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_fail();
}


/*
void g_test_message()
	(const char *) format
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_message(format: *const libc::c_char);
}


/*
void g_test_bug_base()
	(const char *) uri_pattern
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_bug_base(uri_pattern: *const libc::c_char);
}


/*
void g_test_bug()
	(const char *) bug_uri_snippet
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_bug(bug_uri_snippet: *const libc::c_char);
}


/*
void g_test_timer_start()
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_timer_start();
}


/*
double g_test_timer_elapsed()
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_timer_elapsed() -> TypeKind.DOUBLE;
}


/*
double g_test_timer_last()
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_timer_last() -> TypeKind.DOUBLE;
}


/*
void g_test_queue_free()
	(gpointer) gfree_pointer [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_queue_free(gfree_pointer: *mut libc::c_void);
}


/*
void g_test_queue_destroy()
	(GDestroyNotify) destroy_func [void (*)(void *)]
	(gpointer) destroy_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_queue_destroy(destroy_func: Option<extern fn(*mut libc::c_void)>, destroy_data: *mut libc::c_void);
}


/*
gboolean g_test_trap_fork() [int]
	(guint64) usec_timeout [unsigned long]
	(GTestTrapFlags) test_trap_flags [GTestTrapFlags]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_trap_fork(usec_timeout: libc::c_ulong, test_trap_flags: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_test_trap_has_passed() [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_trap_has_passed() -> libc::c_int;
}


/*
gboolean g_test_trap_reached_timeout() [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_trap_reached_timeout() -> libc::c_int;
}


/*
gint32 g_test_rand_int() [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_rand_int() -> libc::c_int;
}


/*
gint32 g_test_rand_int_range() [int]
	(gint32) begin [int]
	(gint32) end [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_rand_int_range(begin: libc::c_int, end: libc::c_int) -> libc::c_int;
}


/*
double g_test_rand_double()
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_rand_double() -> TypeKind.DOUBLE;
}


/*
double g_test_rand_double_range()
	(double) range_start
	(double) range_end
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_rand_double_range(range_start: TypeKind.DOUBLE, range_end: TypeKind.DOUBLE) -> TypeKind.DOUBLE;
}


/*
GTestCase * g_test_create_case() [struct GTestCase *]
	(const char *) test_name
	(gsize) data_size [unsigned long]
	(gconstpointer) test_data [const void *]
	(GTestFixtureFunc) data_setup [void (*)(void *, const void *)]
	(GTestFixtureFunc) data_test [void (*)(void *, const void *)]
	(GTestFixtureFunc) data_teardown [void (*)(void *, const void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_create_case(test_name: *const libc::c_char, data_size: libc::c_ulong, test_data: *const libc::c_void, data_setup: Option<extern fn(*mut libc::c_void, *const libc::c_void)>, data_test: Option<extern fn(*mut libc::c_void, *const libc::c_void)>, data_teardown: Option<extern fn(*mut libc::c_void, *const libc::c_void)>) -> *mut GTestCase;
}


/*
GTestSuite * g_test_create_suite() [struct GTestSuite *]
	(const char *) suite_name
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_create_suite(suite_name: *const libc::c_char) -> *mut GTestSuite;
}


/*
GTestSuite * g_test_get_root() [struct GTestSuite *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_get_root() -> *mut GTestSuite;
}


/*
void g_test_suite_add()
	(GTestSuite *) suite [struct GTestSuite *]
	(GTestCase *) test_case [struct GTestCase *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_suite_add(suite: *mut GTestSuite, test_case: *mut GTestCase);
}


/*
void g_test_suite_add_suite()
	(GTestSuite *) suite [struct GTestSuite *]
	(GTestSuite *) nestedsuite [struct GTestSuite *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_suite_add_suite(suite: *mut GTestSuite, nestedsuite: *mut GTestSuite);
}


/*
int g_test_run_suite()
	(GTestSuite *) suite [struct GTestSuite *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_run_suite(suite: *mut GTestSuite) -> libc::c_int;
}


/*
void g_test_trap_assertions()
	(const char *) domain
	(const char *) file
	(int) line
	(const char *) func
	(guint64) assertion_flags [unsigned long]
	(const char *) pattern
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_trap_assertions(domain: *const libc::c_char, file: *const libc::c_char, line: libc::c_int, func: *const libc::c_char, assertion_flags: libc::c_ulong, pattern: *const libc::c_char);
}


/*
void g_assertion_message()
	(const char *) domain
	(const char *) file
	(int) line
	(const char *) func
	(const char *) message
*/
#[link(name="nice")]
extern "C" {
	pub fn g_assertion_message(domain: *const libc::c_char, file: *const libc::c_char, line: libc::c_int, func: *const libc::c_char, message: *const libc::c_char);
}


/*
void g_assertion_message_expr()
	(const char *) domain
	(const char *) file
	(int) line
	(const char *) func
	(const char *) expr
*/
#[link(name="nice")]
extern "C" {
	pub fn g_assertion_message_expr(domain: *const libc::c_char, file: *const libc::c_char, line: libc::c_int, func: *const libc::c_char, expr: *const libc::c_char);
}


/*
void g_assertion_message_cmpstr()
	(const char *) domain
	(const char *) file
	(int) line
	(const char *) func
	(const char *) expr
	(const char *) arg1
	(const char *) cmp
	(const char *) arg2
*/
#[link(name="nice")]
extern "C" {
	pub fn g_assertion_message_cmpstr(domain: *const libc::c_char, file: *const libc::c_char, line: libc::c_int, func: *const libc::c_char, expr: *const libc::c_char, arg1: *const libc::c_char, cmp: *const libc::c_char, arg2: *const libc::c_char);
}


/*
void g_assertion_message_cmpnum()
	(const char *) domain
	(const char *) file
	(int) line
	(const char *) func
	(const char *) expr
	(long double) arg1
	(const char *) cmp
	(long double) arg2
	(char) numtype
*/
#[link(name="nice")]
extern "C" {
	pub fn g_assertion_message_cmpnum(domain: *const libc::c_char, file: *const libc::c_char, line: libc::c_int, func: *const libc::c_char, expr: *const libc::c_char, arg1: TypeKind.LONGDOUBLE, cmp: *const libc::c_char, arg2: TypeKind.LONGDOUBLE, numtype: libc::c_char);
}


/*
void g_assertion_message_error()
	(const char *) domain
	(const char *) file
	(int) line
	(const char *) func
	(const char *) expr
	(const GError *) error [const struct _GError *]
	(GQuark) error_domain [unsigned int]
	(int) error_code
*/
#[link(name="nice")]
extern "C" {
	pub fn g_assertion_message_error(domain: *const libc::c_char, file: *const libc::c_char, line: libc::c_int, func: *const libc::c_char, expr: *const libc::c_char, error: *const _GError, error_domain: libc::c_uint, error_code: libc::c_int);
}


/*
void g_test_add_vtable()
	(const char *) testpath
	(gsize) data_size [unsigned long]
	(gconstpointer) test_data [const void *]
	(GTestFixtureFunc) data_setup [void (*)(void *, const void *)]
	(GTestFixtureFunc) data_test [void (*)(void *, const void *)]
	(GTestFixtureFunc) data_teardown [void (*)(void *, const void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_add_vtable(testpath: *const libc::c_char, data_size: libc::c_ulong, test_data: *const libc::c_void, data_setup: Option<extern fn(*mut libc::c_void, *const libc::c_void)>, data_test: Option<extern fn(*mut libc::c_void, *const libc::c_void)>, data_teardown: Option<extern fn(*mut libc::c_void, *const libc::c_void)>);
}


/*
const char * g_test_log_type_name()
	(GTestLogType) log_type [GTestLogType]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_log_type_name(log_type: libc::c_uint) -> *const libc::c_char;
}


/*
GTestLogBuffer * g_test_log_buffer_new() [GTestLogBuffer *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_log_buffer_new() -> *mut GTestLogBuffer;
}


/*
void g_test_log_buffer_free()
	(GTestLogBuffer *) tbuffer [GTestLogBuffer *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_log_buffer_free(tbuffer: *mut GTestLogBuffer);
}


/*
void g_test_log_buffer_push()
	(GTestLogBuffer *) tbuffer [GTestLogBuffer *]
	(guint) n_bytes [unsigned int]
	(const guint8 *) bytes [const unsigned char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_log_buffer_push(tbuffer: *mut GTestLogBuffer, n_bytes: libc::c_uint, bytes: *const libc::c_uchar);
}


/*
GTestLogMsg * g_test_log_buffer_pop() [GTestLogMsg *]
	(GTestLogBuffer *) tbuffer [GTestLogBuffer *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_log_buffer_pop(tbuffer: *mut GTestLogBuffer) -> *mut GTestLogMsg;
}


/*
void g_test_log_msg_free()
	(GTestLogMsg *) tmsg [GTestLogMsg *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_log_msg_free(tmsg: *mut GTestLogMsg);
}


/*
void g_test_log_set_fatal_handler()
	(GTestLogFatalFunc) log_func [int (*)(const char *, GLogLevelFlags, const char *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_test_log_set_fatal_handler(log_func: Option<extern fn(*const libc::c_char, libc::c_uint, *const libc::c_char, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
GThreadPool * g_thread_pool_new() [struct _GThreadPool *]
	(GFunc) func [void (*)(void *, void *)]
	(gpointer) user_data [void *]
	(gint) max_threads [int]
	(gboolean) exclusive [int]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_pool_new(func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void, max_threads: libc::c_int, exclusive: libc::c_int, error: *mut *mut _GError) -> *mut _GThreadPool;
}


/*
void g_thread_pool_free()
	(GThreadPool *) pool [struct _GThreadPool *]
	(gboolean) immediate [int]
	(gboolean) wait_ [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_pool_free(pool: *mut _GThreadPool, immediate: libc::c_int, wait_: libc::c_int);
}


/*
gboolean g_thread_pool_push() [int]
	(GThreadPool *) pool [struct _GThreadPool *]
	(gpointer) data [void *]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_pool_push(pool: *mut _GThreadPool, data: *mut libc::c_void, error: *mut *mut _GError) -> libc::c_int;
}


/*
guint g_thread_pool_unprocessed() [unsigned int]
	(GThreadPool *) pool [struct _GThreadPool *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_pool_unprocessed(pool: *mut _GThreadPool) -> libc::c_uint;
}


/*
void g_thread_pool_set_sort_function()
	(GThreadPool *) pool [struct _GThreadPool *]
	(GCompareDataFunc) func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_pool_set_sort_function(pool: *mut _GThreadPool, func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
gboolean g_thread_pool_set_max_threads() [int]
	(GThreadPool *) pool [struct _GThreadPool *]
	(gint) max_threads [int]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_pool_set_max_threads(pool: *mut _GThreadPool, max_threads: libc::c_int, error: *mut *mut _GError) -> libc::c_int;
}


/*
gint g_thread_pool_get_max_threads() [int]
	(GThreadPool *) pool [struct _GThreadPool *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_pool_get_max_threads(pool: *mut _GThreadPool) -> libc::c_int;
}


/*
guint g_thread_pool_get_num_threads() [unsigned int]
	(GThreadPool *) pool [struct _GThreadPool *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_pool_get_num_threads(pool: *mut _GThreadPool) -> libc::c_uint;
}


/*
void g_thread_pool_set_max_unused_threads()
	(gint) max_threads [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_pool_set_max_unused_threads(max_threads: libc::c_int);
}


/*
gint g_thread_pool_get_max_unused_threads() [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_pool_get_max_unused_threads() -> libc::c_int;
}


/*
guint g_thread_pool_get_num_unused_threads() [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_pool_get_num_unused_threads() -> libc::c_uint;
}


/*
void g_thread_pool_stop_unused_threads()
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_pool_stop_unused_threads();
}


/*
void g_thread_pool_set_max_idle_time()
	(guint) interval [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_pool_set_max_idle_time(interval: libc::c_uint);
}


/*
guint g_thread_pool_get_max_idle_time() [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_pool_get_max_idle_time() -> libc::c_uint;
}


/*
GTimer * g_timer_new() [struct _GTimer *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_timer_new() -> *mut _GTimer;
}


/*
void g_timer_destroy()
	(GTimer *) timer [struct _GTimer *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_timer_destroy(timer: *mut _GTimer);
}


/*
void g_timer_start()
	(GTimer *) timer [struct _GTimer *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_timer_start(timer: *mut _GTimer);
}


/*
void g_timer_stop()
	(GTimer *) timer [struct _GTimer *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_timer_stop(timer: *mut _GTimer);
}


/*
void g_timer_reset()
	(GTimer *) timer [struct _GTimer *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_timer_reset(timer: *mut _GTimer);
}


/*
void g_timer_continue()
	(GTimer *) timer [struct _GTimer *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_timer_continue(timer: *mut _GTimer);
}


/*
gdouble g_timer_elapsed() [double]
	(GTimer *) timer [struct _GTimer *]
	(gulong *) microseconds [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_timer_elapsed(timer: *mut _GTimer, microseconds: *mut libc::c_ulong) -> TypeKind.DOUBLE;
}


/*
void g_usleep()
	(gulong) microseconds [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_usleep(microseconds: libc::c_ulong);
}


/*
void g_time_val_add()
	(GTimeVal *) time_ [struct _GTimeVal *]
	(glong) microseconds [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_time_val_add(time_: *mut _GTimeVal, microseconds: libc::c_long);
}


/*
gboolean g_time_val_from_iso8601() [int]
	(const gchar *) iso_date [const char *]
	(GTimeVal *) time_ [struct _GTimeVal *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_time_val_from_iso8601(iso_date: *const libc::c_char, time_: *mut _GTimeVal) -> libc::c_int;
}


/*
gchar * g_time_val_to_iso8601() [char *]
	(GTimeVal *) time_ [struct _GTimeVal *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_time_val_to_iso8601(time_: *mut _GTimeVal) -> *mut libc::c_char;
}


/*
void g_trash_stack_push()
	(GTrashStack **) stack_p [struct _GTrashStack **]
	(gpointer) data_p [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_trash_stack_push(stack_p: *mut *mut _GTrashStack, data_p: *mut libc::c_void);
}


/*
gpointer g_trash_stack_pop() [void *]
	(GTrashStack **) stack_p [struct _GTrashStack **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_trash_stack_pop(stack_p: *mut *mut _GTrashStack) -> *mut libc::c_void;
}


/*
gpointer g_trash_stack_peek() [void *]
	(GTrashStack **) stack_p [struct _GTrashStack **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_trash_stack_peek(stack_p: *mut *mut _GTrashStack) -> *mut libc::c_void;
}


/*
guint g_trash_stack_height() [unsigned int]
	(GTrashStack **) stack_p [struct _GTrashStack **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_trash_stack_height(stack_p: *mut *mut _GTrashStack) -> libc::c_uint;
}


/*
void g_trash_stack_push()
	(GTrashStack **) stack_p [struct _GTrashStack **]
	(gpointer) data_p [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_trash_stack_push(stack_p: *mut *mut _GTrashStack, data_p: *mut libc::c_void);
}


/*
gpointer g_trash_stack_pop() [void *]
	(GTrashStack **) stack_p [struct _GTrashStack **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_trash_stack_pop(stack_p: *mut *mut _GTrashStack) -> *mut libc::c_void;
}


/*
gpointer g_trash_stack_peek() [void *]
	(GTrashStack **) stack_p [struct _GTrashStack **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_trash_stack_peek(stack_p: *mut *mut _GTrashStack) -> *mut libc::c_void;
}


/*
guint g_trash_stack_height() [unsigned int]
	(GTrashStack **) stack_p [struct _GTrashStack **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_trash_stack_height(stack_p: *mut *mut _GTrashStack) -> libc::c_uint;
}


/*
GTree * g_tree_new() [struct _GTree *]
	(GCompareFunc) key_compare_func [int (*)(const void *, const void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_tree_new(key_compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GTree;
}


/*
GTree * g_tree_new_with_data() [struct _GTree *]
	(GCompareDataFunc) key_compare_func [int (*)(const void *, const void *, void *)]
	(gpointer) key_compare_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_tree_new_with_data(key_compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, key_compare_data: *mut libc::c_void) -> *mut _GTree;
}


/*
GTree * g_tree_new_full() [struct _GTree *]
	(GCompareDataFunc) key_compare_func [int (*)(const void *, const void *, void *)]
	(gpointer) key_compare_data [void *]
	(GDestroyNotify) key_destroy_func [void (*)(void *)]
	(GDestroyNotify) value_destroy_func [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_tree_new_full(key_compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, key_compare_data: *mut libc::c_void, key_destroy_func: Option<extern fn(*mut libc::c_void)>, value_destroy_func: Option<extern fn(*mut libc::c_void)>) -> *mut _GTree;
}


/*
GTree * g_tree_ref() [struct _GTree *]
	(GTree *) tree [struct _GTree *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_tree_ref(tree: *mut _GTree) -> *mut _GTree;
}


/*
void g_tree_unref()
	(GTree *) tree [struct _GTree *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_tree_unref(tree: *mut _GTree);
}


/*
void g_tree_destroy()
	(GTree *) tree [struct _GTree *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_tree_destroy(tree: *mut _GTree);
}


/*
void g_tree_insert()
	(GTree *) tree [struct _GTree *]
	(gpointer) key [void *]
	(gpointer) value [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_tree_insert(tree: *mut _GTree, key: *mut libc::c_void, value: *mut libc::c_void);
}


/*
void g_tree_replace()
	(GTree *) tree [struct _GTree *]
	(gpointer) key [void *]
	(gpointer) value [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_tree_replace(tree: *mut _GTree, key: *mut libc::c_void, value: *mut libc::c_void);
}


/*
gboolean g_tree_remove() [int]
	(GTree *) tree [struct _GTree *]
	(gconstpointer) key [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_tree_remove(tree: *mut _GTree, key: *const libc::c_void) -> libc::c_int;
}


/*
gboolean g_tree_steal() [int]
	(GTree *) tree [struct _GTree *]
	(gconstpointer) key [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_tree_steal(tree: *mut _GTree, key: *const libc::c_void) -> libc::c_int;
}


/*
gpointer g_tree_lookup() [void *]
	(GTree *) tree [struct _GTree *]
	(gconstpointer) key [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_tree_lookup(tree: *mut _GTree, key: *const libc::c_void) -> *mut libc::c_void;
}


/*
gboolean g_tree_lookup_extended() [int]
	(GTree *) tree [struct _GTree *]
	(gconstpointer) lookup_key [const void *]
	(gpointer *) orig_key [void **]
	(gpointer *) value [void **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_tree_lookup_extended(tree: *mut _GTree, lookup_key: *const libc::c_void, orig_key: *mut *mut libc::c_void, value: *mut *mut libc::c_void) -> libc::c_int;
}


/*
void g_tree_foreach()
	(GTree *) tree [struct _GTree *]
	(GTraverseFunc) func [int (*)(void *, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_tree_foreach(tree: *mut _GTree, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
void g_tree_traverse()
	(GTree *) tree [struct _GTree *]
	(GTraverseFunc) traverse_func [int (*)(void *, void *, void *)]
	(GTraverseType) traverse_type [GTraverseType]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_tree_traverse(tree: *mut _GTree, traverse_func: Option<extern fn(*mut libc::c_void, *mut libc::c_void, *mut libc::c_void) -> libc::c_int>, traverse_type: libc::c_uint, user_data: *mut libc::c_void);
}


/*
gpointer g_tree_search() [void *]
	(GTree *) tree [struct _GTree *]
	(GCompareFunc) search_func [int (*)(const void *, const void *)]
	(gconstpointer) user_data [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_tree_search(tree: *mut _GTree, search_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>, user_data: *const libc::c_void) -> *mut libc::c_void;
}


/*
gint g_tree_height() [int]
	(GTree *) tree [struct _GTree *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_tree_height(tree: *mut _GTree) -> libc::c_int;
}


/*
gint g_tree_nnodes() [int]
	(GTree *) tree [struct _GTree *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_tree_nnodes(tree: *mut _GTree) -> libc::c_int;
}


/*
char * g_uri_unescape_string()
	(const char *) escaped_string
	(const char *) illegal_characters
*/
#[link(name="nice")]
extern "C" {
	pub fn g_uri_unescape_string(escaped_string: *const libc::c_char, illegal_characters: *const libc::c_char) -> *mut libc::c_char;
}


/*
char * g_uri_unescape_segment()
	(const char *) escaped_string
	(const char *) escaped_string_end
	(const char *) illegal_characters
*/
#[link(name="nice")]
extern "C" {
	pub fn g_uri_unescape_segment(escaped_string: *const libc::c_char, escaped_string_end: *const libc::c_char, illegal_characters: *const libc::c_char) -> *mut libc::c_char;
}


/*
char * g_uri_parse_scheme()
	(const char *) uri
*/
#[link(name="nice")]
extern "C" {
	pub fn g_uri_parse_scheme(uri: *const libc::c_char) -> *mut libc::c_char;
}


/*
char * g_uri_escape_string()
	(const char *) unescaped
	(const char *) reserved_chars_allowed
	(gboolean) allow_utf8 [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_uri_escape_string(unescaped: *const libc::c_char, reserved_chars_allowed: *const libc::c_char, allow_utf8: libc::c_int) -> *mut libc::c_char;
}


/*
gboolean g_variant_type_string_is_valid() [int]
	(const gchar *) type_string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_string_is_valid(type_string: *const libc::c_char) -> libc::c_int;
}


/*
gboolean g_variant_type_string_scan() [int]
	(const gchar *) string [const char *]
	(const gchar *) limit [const char *]
	(const gchar **) endptr [const char **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_string_scan(string: *const libc::c_char, limit: *const libc::c_char, endptr: *mut *const libc::c_char) -> libc::c_int;
}


/*
void g_variant_type_free()
	(GVariantType *) type [struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_free(type_: *mut _GVariantType);
}


/*
GVariantType * g_variant_type_copy() [struct _GVariantType *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_copy(type_: *const _GVariantType) -> *mut _GVariantType;
}


/*
GVariantType * g_variant_type_new() [struct _GVariantType *]
	(const gchar *) type_string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_new(type_string: *const libc::c_char) -> *mut _GVariantType;
}


/*
gsize g_variant_type_get_string_length() [unsigned long]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_get_string_length(type_: *const _GVariantType) -> libc::c_ulong;
}


/*
const gchar * g_variant_type_peek_string() [const char *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_peek_string(type_: *const _GVariantType) -> *const libc::c_char;
}


/*
gchar * g_variant_type_dup_string() [char *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_dup_string(type_: *const _GVariantType) -> *mut libc::c_char;
}


/*
gboolean g_variant_type_is_definite() [int]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_is_definite(type_: *const _GVariantType) -> libc::c_int;
}


/*
gboolean g_variant_type_is_container() [int]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_is_container(type_: *const _GVariantType) -> libc::c_int;
}


/*
gboolean g_variant_type_is_basic() [int]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_is_basic(type_: *const _GVariantType) -> libc::c_int;
}


/*
gboolean g_variant_type_is_maybe() [int]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_is_maybe(type_: *const _GVariantType) -> libc::c_int;
}


/*
gboolean g_variant_type_is_array() [int]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_is_array(type_: *const _GVariantType) -> libc::c_int;
}


/*
gboolean g_variant_type_is_tuple() [int]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_is_tuple(type_: *const _GVariantType) -> libc::c_int;
}


/*
gboolean g_variant_type_is_dict_entry() [int]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_is_dict_entry(type_: *const _GVariantType) -> libc::c_int;
}


/*
gboolean g_variant_type_is_variant() [int]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_is_variant(type_: *const _GVariantType) -> libc::c_int;
}


/*
guint g_variant_type_hash() [unsigned int]
	(gconstpointer) type [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_hash(type_: *const libc::c_void) -> libc::c_uint;
}


/*
gboolean g_variant_type_equal() [int]
	(gconstpointer) type1 [const void *]
	(gconstpointer) type2 [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_equal(type1: *const libc::c_void, type2: *const libc::c_void) -> libc::c_int;
}


/*
gboolean g_variant_type_is_subtype_of() [int]
	(const GVariantType *) type [const struct _GVariantType *]
	(const GVariantType *) supertype [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_is_subtype_of(type_: *const _GVariantType, supertype: *const _GVariantType) -> libc::c_int;
}


/*
const GVariantType * g_variant_type_element() [const struct _GVariantType *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_element(type_: *const _GVariantType) -> *const _GVariantType;
}


/*
const GVariantType * g_variant_type_first() [const struct _GVariantType *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_first(type_: *const _GVariantType) -> *const _GVariantType;
}


/*
const GVariantType * g_variant_type_next() [const struct _GVariantType *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_next(type_: *const _GVariantType) -> *const _GVariantType;
}


/*
gsize g_variant_type_n_items() [unsigned long]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_n_items(type_: *const _GVariantType) -> libc::c_ulong;
}


/*
const GVariantType * g_variant_type_key() [const struct _GVariantType *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_key(type_: *const _GVariantType) -> *const _GVariantType;
}


/*
const GVariantType * g_variant_type_value() [const struct _GVariantType *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_value(type_: *const _GVariantType) -> *const _GVariantType;
}


/*
GVariantType * g_variant_type_new_array() [struct _GVariantType *]
	(const GVariantType *) element [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_new_array(element: *const _GVariantType) -> *mut _GVariantType;
}


/*
GVariantType * g_variant_type_new_maybe() [struct _GVariantType *]
	(const GVariantType *) element [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_new_maybe(element: *const _GVariantType) -> *mut _GVariantType;
}


/*
GVariantType * g_variant_type_new_tuple() [struct _GVariantType *]
	(const GVariantType *const *) items [const struct _GVariantType *const *]
	(gint) length [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_new_tuple(items: *const *const _GVariantType, length: libc::c_int) -> *mut _GVariantType;
}


/*
GVariantType * g_variant_type_new_dict_entry() [struct _GVariantType *]
	(const GVariantType *) key [const struct _GVariantType *]
	(const GVariantType *) value [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_new_dict_entry(key: *const _GVariantType, value: *const _GVariantType) -> *mut _GVariantType;
}


/*
const GVariantType * g_variant_type_checked_() [const struct _GVariantType *]
	(const gchar *)  [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_type_checked_(: *const libc::c_char) -> *const _GVariantType;
}


/*
void g_variant_unref()
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_unref(value: *mut _GVariant);
}


/*
GVariant * g_variant_ref() [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_ref(value: *mut _GVariant) -> *mut _GVariant;
}


/*
GVariant * g_variant_ref_sink() [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_ref_sink(value: *mut _GVariant) -> *mut _GVariant;
}


/*
gboolean g_variant_is_floating() [int]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_is_floating(value: *mut _GVariant) -> libc::c_int;
}


/*
GVariant * g_variant_take_ref() [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_take_ref(value: *mut _GVariant) -> *mut _GVariant;
}


/*
const GVariantType * g_variant_get_type() [const struct _GVariantType *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_type(value: *mut _GVariant) -> *const _GVariantType;
}


/*
const gchar * g_variant_get_type_string() [const char *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_type_string(value: *mut _GVariant) -> *const libc::c_char;
}


/*
gboolean g_variant_is_of_type() [int]
	(GVariant *) value [struct _GVariant *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_is_of_type(value: *mut _GVariant, type_: *const _GVariantType) -> libc::c_int;
}


/*
gboolean g_variant_is_container() [int]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_is_container(value: *mut _GVariant) -> libc::c_int;
}


/*
GVariantClass g_variant_classify() [GVariantClass]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_classify(value: *mut _GVariant) -> libc::c_uint;
}


/*
GVariant * g_variant_new_boolean() [struct _GVariant *]
	(gboolean) value [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_boolean(value: libc::c_int) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_byte() [struct _GVariant *]
	(guchar) value [unsigned char]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_byte(value: libc::c_uchar) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_int16() [struct _GVariant *]
	(gint16) value [short]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_int16(value: libc::c_short) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_uint16() [struct _GVariant *]
	(guint16) value [unsigned short]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_uint16(value: libc::c_ushort) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_int32() [struct _GVariant *]
	(gint32) value [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_int32(value: libc::c_int) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_uint32() [struct _GVariant *]
	(guint32) value [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_uint32(value: libc::c_uint) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_int64() [struct _GVariant *]
	(gint64) value [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_int64(value: libc::c_long) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_uint64() [struct _GVariant *]
	(guint64) value [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_uint64(value: libc::c_ulong) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_handle() [struct _GVariant *]
	(gint32) value [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_handle(value: libc::c_int) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_double() [struct _GVariant *]
	(gdouble) value [double]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_double(value: TypeKind.DOUBLE) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_string() [struct _GVariant *]
	(const gchar *) string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_string(string: *const libc::c_char) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_object_path() [struct _GVariant *]
	(const gchar *) object_path [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_object_path(object_path: *const libc::c_char) -> *mut _GVariant;
}


/*
gboolean g_variant_is_object_path() [int]
	(const gchar *) string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_is_object_path(string: *const libc::c_char) -> libc::c_int;
}


/*
GVariant * g_variant_new_signature() [struct _GVariant *]
	(const gchar *) signature [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_signature(signature: *const libc::c_char) -> *mut _GVariant;
}


/*
gboolean g_variant_is_signature() [int]
	(const gchar *) string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_is_signature(string: *const libc::c_char) -> libc::c_int;
}


/*
GVariant * g_variant_new_variant() [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_variant(value: *mut _GVariant) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_strv() [struct _GVariant *]
	(const gchar *const *) strv [const char *const *]
	(gssize) length [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_strv(strv: *const *const libc::c_char, length: libc::c_long) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_objv() [struct _GVariant *]
	(const gchar *const *) strv [const char *const *]
	(gssize) length [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_objv(strv: *const *const libc::c_char, length: libc::c_long) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_bytestring() [struct _GVariant *]
	(const gchar *) string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_bytestring(string: *const libc::c_char) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_bytestring_array() [struct _GVariant *]
	(const gchar *const *) strv [const char *const *]
	(gssize) length [long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_bytestring_array(strv: *const *const libc::c_char, length: libc::c_long) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_fixed_array() [struct _GVariant *]
	(const GVariantType *) element_type [const struct _GVariantType *]
	(gconstpointer) elements [const void *]
	(gsize) n_elements [unsigned long]
	(gsize) element_size [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_fixed_array(element_type: *const _GVariantType, elements: *const libc::c_void, n_elements: libc::c_ulong, element_size: libc::c_ulong) -> *mut _GVariant;
}


/*
gboolean g_variant_get_boolean() [int]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_boolean(value: *mut _GVariant) -> libc::c_int;
}


/*
guchar g_variant_get_byte() [unsigned char]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_byte(value: *mut _GVariant) -> libc::c_uchar;
}


/*
gint16 g_variant_get_int16() [short]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_int16(value: *mut _GVariant) -> libc::c_short;
}


/*
guint16 g_variant_get_uint16() [unsigned short]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_uint16(value: *mut _GVariant) -> libc::c_ushort;
}


/*
gint32 g_variant_get_int32() [int]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_int32(value: *mut _GVariant) -> libc::c_int;
}


/*
guint32 g_variant_get_uint32() [unsigned int]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_uint32(value: *mut _GVariant) -> libc::c_uint;
}


/*
gint64 g_variant_get_int64() [long]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_int64(value: *mut _GVariant) -> libc::c_long;
}


/*
guint64 g_variant_get_uint64() [unsigned long]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_uint64(value: *mut _GVariant) -> libc::c_ulong;
}


/*
gint32 g_variant_get_handle() [int]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_handle(value: *mut _GVariant) -> libc::c_int;
}


/*
gdouble g_variant_get_double() [double]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_double(value: *mut _GVariant) -> TypeKind.DOUBLE;
}


/*
GVariant * g_variant_get_variant() [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_variant(value: *mut _GVariant) -> *mut _GVariant;
}


/*
const gchar * g_variant_get_string() [const char *]
	(GVariant *) value [struct _GVariant *]
	(gsize *) length [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_string(value: *mut _GVariant, length: *mut libc::c_ulong) -> *const libc::c_char;
}


/*
gchar * g_variant_dup_string() [char *]
	(GVariant *) value [struct _GVariant *]
	(gsize *) length [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_dup_string(value: *mut _GVariant, length: *mut libc::c_ulong) -> *mut libc::c_char;
}


/*
const gchar ** g_variant_get_strv() [const char **]
	(GVariant *) value [struct _GVariant *]
	(gsize *) length [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_strv(value: *mut _GVariant, length: *mut libc::c_ulong) -> *mut *const libc::c_char;
}


/*
gchar ** g_variant_dup_strv() [char **]
	(GVariant *) value [struct _GVariant *]
	(gsize *) length [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_dup_strv(value: *mut _GVariant, length: *mut libc::c_ulong) -> *mut *mut libc::c_char;
}


/*
const gchar ** g_variant_get_objv() [const char **]
	(GVariant *) value [struct _GVariant *]
	(gsize *) length [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_objv(value: *mut _GVariant, length: *mut libc::c_ulong) -> *mut *const libc::c_char;
}


/*
gchar ** g_variant_dup_objv() [char **]
	(GVariant *) value [struct _GVariant *]
	(gsize *) length [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_dup_objv(value: *mut _GVariant, length: *mut libc::c_ulong) -> *mut *mut libc::c_char;
}


/*
const gchar * g_variant_get_bytestring() [const char *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_bytestring(value: *mut _GVariant) -> *const libc::c_char;
}


/*
gchar * g_variant_dup_bytestring() [char *]
	(GVariant *) value [struct _GVariant *]
	(gsize *) length [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_dup_bytestring(value: *mut _GVariant, length: *mut libc::c_ulong) -> *mut libc::c_char;
}


/*
const gchar ** g_variant_get_bytestring_array() [const char **]
	(GVariant *) value [struct _GVariant *]
	(gsize *) length [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_bytestring_array(value: *mut _GVariant, length: *mut libc::c_ulong) -> *mut *const libc::c_char;
}


/*
gchar ** g_variant_dup_bytestring_array() [char **]
	(GVariant *) value [struct _GVariant *]
	(gsize *) length [unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_dup_bytestring_array(value: *mut _GVariant, length: *mut libc::c_ulong) -> *mut *mut libc::c_char;
}


/*
GVariant * g_variant_new_maybe() [struct _GVariant *]
	(const GVariantType *) child_type [const struct _GVariantType *]
	(GVariant *) child [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_maybe(child_type: *const _GVariantType, child: *mut _GVariant) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_array() [struct _GVariant *]
	(const GVariantType *) child_type [const struct _GVariantType *]
	(GVariant *const *) children [struct _GVariant *const *]
	(gsize) n_children [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_array(child_type: *const _GVariantType, children: *const *mut _GVariant, n_children: libc::c_ulong) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_tuple() [struct _GVariant *]
	(GVariant *const *) children [struct _GVariant *const *]
	(gsize) n_children [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_tuple(children: *const *mut _GVariant, n_children: libc::c_ulong) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_dict_entry() [struct _GVariant *]
	(GVariant *) key [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_dict_entry(key: *mut _GVariant, value: *mut _GVariant) -> *mut _GVariant;
}


/*
GVariant * g_variant_get_maybe() [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_maybe(value: *mut _GVariant) -> *mut _GVariant;
}


/*
gsize g_variant_n_children() [unsigned long]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_n_children(value: *mut _GVariant) -> libc::c_ulong;
}


/*
void g_variant_get_child()
	(GVariant *) value [struct _GVariant *]
	(gsize) index_ [unsigned long]
	(const gchar *) format_string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_child(value: *mut _GVariant, index_: libc::c_ulong, format_string: *const libc::c_char);
}


/*
GVariant * g_variant_get_child_value() [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
	(gsize) index_ [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_child_value(value: *mut _GVariant, index_: libc::c_ulong) -> *mut _GVariant;
}


/*
gboolean g_variant_lookup() [int]
	(GVariant *) dictionary [struct _GVariant *]
	(const gchar *) key [const char *]
	(const gchar *) format_string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_lookup(dictionary: *mut _GVariant, key: *const libc::c_char, format_string: *const libc::c_char) -> libc::c_int;
}


/*
GVariant * g_variant_lookup_value() [struct _GVariant *]
	(GVariant *) dictionary [struct _GVariant *]
	(const gchar *) key [const char *]
	(const GVariantType *) expected_type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_lookup_value(dictionary: *mut _GVariant, key: *const libc::c_char, expected_type: *const _GVariantType) -> *mut _GVariant;
}


/*
gconstpointer g_variant_get_fixed_array() [const void *]
	(GVariant *) value [struct _GVariant *]
	(gsize *) n_elements [unsigned long *]
	(gsize) element_size [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_fixed_array(value: *mut _GVariant, n_elements: *mut libc::c_ulong, element_size: libc::c_ulong) -> *const libc::c_void;
}


/*
gsize g_variant_get_size() [unsigned long]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_size(value: *mut _GVariant) -> libc::c_ulong;
}


/*
gconstpointer g_variant_get_data() [const void *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_data(value: *mut _GVariant) -> *const libc::c_void;
}


/*
void g_variant_store()
	(GVariant *) value [struct _GVariant *]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_store(value: *mut _GVariant, data: *mut libc::c_void);
}


/*
gchar * g_variant_print() [char *]
	(GVariant *) value [struct _GVariant *]
	(gboolean) type_annotate [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_print(value: *mut _GVariant, type_annotate: libc::c_int) -> *mut libc::c_char;
}


/*
GString * g_variant_print_string() [struct _GString *]
	(GVariant *) value [struct _GVariant *]
	(GString *) string [struct _GString *]
	(gboolean) type_annotate [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_print_string(value: *mut _GVariant, string: *mut _GString, type_annotate: libc::c_int) -> *mut _GString;
}


/*
guint g_variant_hash() [unsigned int]
	(gconstpointer) value [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_hash(value: *const libc::c_void) -> libc::c_uint;
}


/*
gboolean g_variant_equal() [int]
	(gconstpointer) one [const void *]
	(gconstpointer) two [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_equal(one: *const libc::c_void, two: *const libc::c_void) -> libc::c_int;
}


/*
GVariant * g_variant_get_normal_form() [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_normal_form(value: *mut _GVariant) -> *mut _GVariant;
}


/*
gboolean g_variant_is_normal_form() [int]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_is_normal_form(value: *mut _GVariant) -> libc::c_int;
}


/*
GVariant * g_variant_byteswap() [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_byteswap(value: *mut _GVariant) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_from_data() [struct _GVariant *]
	(const GVariantType *) type [const struct _GVariantType *]
	(gconstpointer) data [const void *]
	(gsize) size [unsigned long]
	(gboolean) trusted [int]
	(GDestroyNotify) notify [void (*)(void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_from_data(type_: *const _GVariantType, data: *const libc::c_void, size: libc::c_ulong, trusted: libc::c_int, notify: Option<extern fn(*mut libc::c_void)>, user_data: *mut libc::c_void) -> *mut _GVariant;
}


/*
GVariantIter * g_variant_iter_new() [struct _GVariantIter *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_iter_new(value: *mut _GVariant) -> *mut _GVariantIter;
}


/*
gsize g_variant_iter_init() [unsigned long]
	(GVariantIter *) iter [struct _GVariantIter *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_iter_init(iter: *mut _GVariantIter, value: *mut _GVariant) -> libc::c_ulong;
}


/*
GVariantIter * g_variant_iter_copy() [struct _GVariantIter *]
	(GVariantIter *) iter [struct _GVariantIter *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_iter_copy(iter: *mut _GVariantIter) -> *mut _GVariantIter;
}


/*
gsize g_variant_iter_n_children() [unsigned long]
	(GVariantIter *) iter [struct _GVariantIter *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_iter_n_children(iter: *mut _GVariantIter) -> libc::c_ulong;
}


/*
void g_variant_iter_free()
	(GVariantIter *) iter [struct _GVariantIter *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_iter_free(iter: *mut _GVariantIter);
}


/*
GVariant * g_variant_iter_next_value() [struct _GVariant *]
	(GVariantIter *) iter [struct _GVariantIter *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_iter_next_value(iter: *mut _GVariantIter) -> *mut _GVariant;
}


/*
gboolean g_variant_iter_next() [int]
	(GVariantIter *) iter [struct _GVariantIter *]
	(const gchar *) format_string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_iter_next(iter: *mut _GVariantIter, format_string: *const libc::c_char) -> libc::c_int;
}


/*
gboolean g_variant_iter_loop() [int]
	(GVariantIter *) iter [struct _GVariantIter *]
	(const gchar *) format_string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_iter_loop(iter: *mut _GVariantIter, format_string: *const libc::c_char) -> libc::c_int;
}


/*
GQuark g_variant_parser_get_error_quark() [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_parser_get_error_quark() -> libc::c_uint;
}


/*
GVariantBuilder * g_variant_builder_new() [struct _GVariantBuilder *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_builder_new(type_: *const _GVariantType) -> *mut _GVariantBuilder;
}


/*
void g_variant_builder_unref()
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_builder_unref(builder: *mut _GVariantBuilder);
}


/*
GVariantBuilder * g_variant_builder_ref() [struct _GVariantBuilder *]
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_builder_ref(builder: *mut _GVariantBuilder) -> *mut _GVariantBuilder;
}


/*
void g_variant_builder_init()
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_builder_init(builder: *mut _GVariantBuilder, type_: *const _GVariantType);
}


/*
GVariant * g_variant_builder_end() [struct _GVariant *]
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_builder_end(builder: *mut _GVariantBuilder) -> *mut _GVariant;
}


/*
void g_variant_builder_clear()
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_builder_clear(builder: *mut _GVariantBuilder);
}


/*
void g_variant_builder_open()
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_builder_open(builder: *mut _GVariantBuilder, type_: *const _GVariantType);
}


/*
void g_variant_builder_close()
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_builder_close(builder: *mut _GVariantBuilder);
}


/*
void g_variant_builder_add_value()
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_builder_add_value(builder: *mut _GVariantBuilder, value: *mut _GVariant);
}


/*
void g_variant_builder_add()
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
	(const gchar *) format_string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_builder_add(builder: *mut _GVariantBuilder, format_string: *const libc::c_char);
}


/*
void g_variant_builder_add_parsed()
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
	(const gchar *) format [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_builder_add_parsed(builder: *mut _GVariantBuilder, format: *const libc::c_char);
}


/*
GVariant * g_variant_new() [struct _GVariant *]
	(const gchar *) format_string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new(format_string: *const libc::c_char) -> *mut _GVariant;
}


/*
void g_variant_get()
	(GVariant *) value [struct _GVariant *]
	(const gchar *) format_string [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get(value: *mut _GVariant, format_string: *const libc::c_char);
}


/*
GVariant * g_variant_new_va() [struct _GVariant *]
	(const gchar *) format_string [const char *]
	(const gchar **) endptr [const char **]
	(va_list *) app [struct __va_list_tag (*)[1]]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_va(format_string: *const libc::c_char, endptr: *mut *const libc::c_char, app: *mut [__va_list_tag; 1]) -> *mut _GVariant;
}


/*
void g_variant_get_va()
	(GVariant *) value [struct _GVariant *]
	(const gchar *) format_string [const char *]
	(const gchar **) endptr [const char **]
	(va_list *) app [struct __va_list_tag (*)[1]]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_get_va(value: *mut _GVariant, format_string: *const libc::c_char, endptr: *mut *const libc::c_char, app: *mut [__va_list_tag; 1]);
}


/*
GVariant * g_variant_parse() [struct _GVariant *]
	(const GVariantType *) type [const struct _GVariantType *]
	(const gchar *) text [const char *]
	(const gchar *) limit [const char *]
	(const gchar **) endptr [const char **]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_parse(type_: *const _GVariantType, text: *const libc::c_char, limit: *const libc::c_char, endptr: *mut *const libc::c_char, error: *mut *mut _GError) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_parsed() [struct _GVariant *]
	(const gchar *) format [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_parsed(format: *const libc::c_char) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_parsed_va() [struct _GVariant *]
	(const gchar *) format [const char *]
	(va_list *) app [struct __va_list_tag (*)[1]]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_new_parsed_va(format: *const libc::c_char, app: *mut [__va_list_tag; 1]) -> *mut _GVariant;
}


/*
gint g_variant_compare() [int]
	(gconstpointer) one [const void *]
	(gconstpointer) two [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_variant_compare(one: *const libc::c_void, two: *const libc::c_void) -> libc::c_int;
}


/*
const gchar * glib_check_version() [const char *]
	(guint) required_major [unsigned int]
	(guint) required_minor [unsigned int]
	(guint) required_micro [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn glib_check_version(required_major: libc::c_uint, required_minor: libc::c_uint, required_micro: libc::c_uint) -> *const libc::c_char;
}


/*
GMemChunk * g_mem_chunk_new() [struct _GMemChunk *]
	(const gchar *) name [const char *]
	(gint) atom_size [int]
	(gsize) area_size [unsigned long]
	(gint) type [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mem_chunk_new(name: *const libc::c_char, atom_size: libc::c_int, area_size: libc::c_ulong, type_: libc::c_int) -> *mut _GMemChunk;
}


/*
void g_mem_chunk_destroy()
	(GMemChunk *) mem_chunk [struct _GMemChunk *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mem_chunk_destroy(mem_chunk: *mut _GMemChunk);
}


/*
gpointer g_mem_chunk_alloc() [void *]
	(GMemChunk *) mem_chunk [struct _GMemChunk *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mem_chunk_alloc(mem_chunk: *mut _GMemChunk) -> *mut libc::c_void;
}


/*
gpointer g_mem_chunk_alloc0() [void *]
	(GMemChunk *) mem_chunk [struct _GMemChunk *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mem_chunk_alloc0(mem_chunk: *mut _GMemChunk) -> *mut libc::c_void;
}


/*
void g_mem_chunk_free()
	(GMemChunk *) mem_chunk [struct _GMemChunk *]
	(gpointer) mem [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mem_chunk_free(mem_chunk: *mut _GMemChunk, mem: *mut libc::c_void);
}


/*
void g_mem_chunk_clean()
	(GMemChunk *) mem_chunk [struct _GMemChunk *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mem_chunk_clean(mem_chunk: *mut _GMemChunk);
}


/*
void g_mem_chunk_reset()
	(GMemChunk *) mem_chunk [struct _GMemChunk *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mem_chunk_reset(mem_chunk: *mut _GMemChunk);
}


/*
void g_mem_chunk_print()
	(GMemChunk *) mem_chunk [struct _GMemChunk *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mem_chunk_print(mem_chunk: *mut _GMemChunk);
}


/*
void g_mem_chunk_info()
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mem_chunk_info();
}


/*
void g_blow_chunks()
*/
#[link(name="nice")]
extern "C" {
	pub fn g_blow_chunks();
}


/*
GAllocator * g_allocator_new() [struct _GAllocator *]
	(const gchar *) name [const char *]
	(guint) n_preallocs [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_allocator_new(name: *const libc::c_char, n_preallocs: libc::c_uint) -> *mut _GAllocator;
}


/*
void g_allocator_free()
	(GAllocator *) allocator [struct _GAllocator *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_allocator_free(allocator: *mut _GAllocator);
}


/*
void g_list_push_allocator()
	(GAllocator *) allocator [struct _GAllocator *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_push_allocator(allocator: *mut _GAllocator);
}


/*
void g_list_pop_allocator()
*/
#[link(name="nice")]
extern "C" {
	pub fn g_list_pop_allocator();
}


/*
void g_slist_push_allocator()
	(GAllocator *) allocator [struct _GAllocator *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_push_allocator(allocator: *mut _GAllocator);
}


/*
void g_slist_pop_allocator()
*/
#[link(name="nice")]
extern "C" {
	pub fn g_slist_pop_allocator();
}


/*
void g_node_push_allocator()
	(GAllocator *) allocator [struct _GAllocator *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_push_allocator(allocator: *mut _GAllocator);
}


/*
void g_node_pop_allocator()
*/
#[link(name="nice")]
extern "C" {
	pub fn g_node_pop_allocator();
}


/*
GCache * g_cache_new() [struct _GCache *]
	(GCacheNewFunc) value_new_func [void *(*)(void *)]
	(GCacheDestroyFunc) value_destroy_func [void (*)(void *)]
	(GCacheDupFunc) key_dup_func [void *(*)(void *)]
	(GCacheDestroyFunc) key_destroy_func [void (*)(void *)]
	(GHashFunc) hash_key_func [unsigned int (*)(const void *)]
	(GHashFunc) hash_value_func [unsigned int (*)(const void *)]
	(GEqualFunc) key_equal_func [int (*)(const void *, const void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_cache_new(value_new_func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>, value_destroy_func: Option<extern fn(*mut libc::c_void)>, key_dup_func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>, key_destroy_func: Option<extern fn(*mut libc::c_void)>, hash_key_func: Option<extern fn(*const libc::c_void) -> libc::c_uint>, hash_value_func: Option<extern fn(*const libc::c_void) -> libc::c_uint>, key_equal_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GCache;
}


/*
void g_cache_destroy()
	(GCache *) cache [struct _GCache *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_cache_destroy(cache: *mut _GCache);
}


/*
gpointer g_cache_insert() [void *]
	(GCache *) cache [struct _GCache *]
	(gpointer) key [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_cache_insert(cache: *mut _GCache, key: *mut libc::c_void) -> *mut libc::c_void;
}


/*
void g_cache_remove()
	(GCache *) cache [struct _GCache *]
	(gconstpointer) value [const void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_cache_remove(cache: *mut _GCache, value: *const libc::c_void);
}


/*
void g_cache_key_foreach()
	(GCache *) cache [struct _GCache *]
	(GHFunc) func [void (*)(void *, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_cache_key_foreach(cache: *mut _GCache, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
void g_cache_value_foreach()
	(GCache *) cache [struct _GCache *]
	(GHFunc) func [void (*)(void *, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_cache_value_foreach(cache: *mut _GCache, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
GCompletion * g_completion_new() [struct _GCompletion *]
	(GCompletionFunc) func [char *(*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_completion_new(func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_char>) -> *mut _GCompletion;
}


/*
void g_completion_add_items()
	(GCompletion *) cmp [struct _GCompletion *]
	(GList *) items [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_completion_add_items(cmp: *mut _GCompletion, items: *mut _GList);
}


/*
void g_completion_remove_items()
	(GCompletion *) cmp [struct _GCompletion *]
	(GList *) items [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_completion_remove_items(cmp: *mut _GCompletion, items: *mut _GList);
}


/*
void g_completion_clear_items()
	(GCompletion *) cmp [struct _GCompletion *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_completion_clear_items(cmp: *mut _GCompletion);
}


/*
GList * g_completion_complete() [struct _GList *]
	(GCompletion *) cmp [struct _GCompletion *]
	(const gchar *) prefix [const char *]
	(gchar **) new_prefix [char **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_completion_complete(cmp: *mut _GCompletion, prefix: *const libc::c_char, new_prefix: *mut *mut libc::c_char) -> *mut _GList;
}


/*
GList * g_completion_complete_utf8() [struct _GList *]
	(GCompletion *) cmp [struct _GCompletion *]
	(const gchar *) prefix [const char *]
	(gchar **) new_prefix [char **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_completion_complete_utf8(cmp: *mut _GCompletion, prefix: *const libc::c_char, new_prefix: *mut *mut libc::c_char) -> *mut _GList;
}


/*
void g_completion_set_compare()
	(GCompletion *) cmp [struct _GCompletion *]
	(GCompletionStrncmpFunc) strncmp_func [int (*)(const char *, const char *, unsigned long)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_completion_set_compare(cmp: *mut _GCompletion, strncmp_func: Option<extern fn(*const libc::c_char, *const libc::c_char, libc::c_ulong) -> libc::c_int>);
}


/*
void g_completion_free()
	(GCompletion *) cmp [struct _GCompletion *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_completion_free(cmp: *mut _GCompletion);
}


/*
GRelation * g_relation_new() [struct _GRelation *]
	(gint) fields [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_relation_new(fields: libc::c_int) -> *mut _GRelation;
}


/*
void g_relation_destroy()
	(GRelation *) relation [struct _GRelation *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_relation_destroy(relation: *mut _GRelation);
}


/*
void g_relation_index()
	(GRelation *) relation [struct _GRelation *]
	(gint) field [int]
	(GHashFunc) hash_func [unsigned int (*)(const void *)]
	(GEqualFunc) key_equal_func [int (*)(const void *, const void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_relation_index(relation: *mut _GRelation, field: libc::c_int, hash_func: Option<extern fn(*const libc::c_void) -> libc::c_uint>, key_equal_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>);
}


/*
void g_relation_insert()
	(GRelation *) relation [struct _GRelation *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_relation_insert(relation: *mut _GRelation);
}


/*
gint g_relation_delete() [int]
	(GRelation *) relation [struct _GRelation *]
	(gconstpointer) key [const void *]
	(gint) field [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_relation_delete(relation: *mut _GRelation, key: *const libc::c_void, field: libc::c_int) -> libc::c_int;
}


/*
GTuples * g_relation_select() [struct _GTuples *]
	(GRelation *) relation [struct _GRelation *]
	(gconstpointer) key [const void *]
	(gint) field [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_relation_select(relation: *mut _GRelation, key: *const libc::c_void, field: libc::c_int) -> *mut _GTuples;
}


/*
gint g_relation_count() [int]
	(GRelation *) relation [struct _GRelation *]
	(gconstpointer) key [const void *]
	(gint) field [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_relation_count(relation: *mut _GRelation, key: *const libc::c_void, field: libc::c_int) -> libc::c_int;
}


/*
gboolean g_relation_exists() [int]
	(GRelation *) relation [struct _GRelation *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_relation_exists(relation: *mut _GRelation) -> libc::c_int;
}


/*
void g_relation_print()
	(GRelation *) relation [struct _GRelation *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_relation_print(relation: *mut _GRelation);
}


/*
void g_tuples_destroy()
	(GTuples *) tuples [struct _GTuples *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_tuples_destroy(tuples: *mut _GTuples);
}


/*
gpointer g_tuples_index() [void *]
	(GTuples *) tuples [struct _GTuples *]
	(gint) index_ [int]
	(gint) field [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_tuples_index(tuples: *mut _GTuples, index_: libc::c_int, field: libc::c_int) -> *mut libc::c_void;
}


/*
GThread * g_thread_create() [struct _GThread *]
	(GThreadFunc) func [void *(*)(void *)]
	(gpointer) data [void *]
	(gboolean) joinable [int]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_create(func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>, data: *mut libc::c_void, joinable: libc::c_int, error: *mut *mut _GError) -> *mut _GThread;
}


/*
GThread * g_thread_create_full() [struct _GThread *]
	(GThreadFunc) func [void *(*)(void *)]
	(gpointer) data [void *]
	(gulong) stack_size [unsigned long]
	(gboolean) joinable [int]
	(gboolean) bound [int]
	(GThreadPriority) priority [GThreadPriority]
	(GError **) error [struct _GError **]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_create_full(func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>, data: *mut libc::c_void, stack_size: libc::c_ulong, joinable: libc::c_int, bound: libc::c_int, priority: libc::c_uint, error: *mut *mut _GError) -> *mut _GThread;
}


/*
void g_thread_set_priority()
	(GThread *) thread [struct _GThread *]
	(GThreadPriority) priority [GThreadPriority]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_set_priority(thread: *mut _GThread, priority: libc::c_uint);
}


/*
void g_thread_foreach()
	(GFunc) thread_func [void (*)(void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_foreach(thread_func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
void g_static_mutex_init()
	(GStaticMutex *) mutex [GStaticMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_mutex_init(mutex: *mut GStaticMutex);
}


/*
void g_static_mutex_free()
	(GStaticMutex *) mutex [GStaticMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_mutex_free(mutex: *mut GStaticMutex);
}


/*
GMutex * g_static_mutex_get_mutex_impl() [union _GMutex *]
	(GStaticMutex *) mutex [GStaticMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_mutex_get_mutex_impl(mutex: *mut GStaticMutex) -> *mut union _GMutex;
}


/*
void g_static_rec_mutex_init()
	(GStaticRecMutex *) mutex [struct _GStaticRecMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_rec_mutex_init(mutex: *mut _GStaticRecMutex);
}


/*
void g_static_rec_mutex_lock()
	(GStaticRecMutex *) mutex [struct _GStaticRecMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_rec_mutex_lock(mutex: *mut _GStaticRecMutex);
}


/*
gboolean g_static_rec_mutex_trylock() [int]
	(GStaticRecMutex *) mutex [struct _GStaticRecMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_rec_mutex_trylock(mutex: *mut _GStaticRecMutex) -> libc::c_int;
}


/*
void g_static_rec_mutex_unlock()
	(GStaticRecMutex *) mutex [struct _GStaticRecMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_rec_mutex_unlock(mutex: *mut _GStaticRecMutex);
}


/*
void g_static_rec_mutex_lock_full()
	(GStaticRecMutex *) mutex [struct _GStaticRecMutex *]
	(guint) depth [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_rec_mutex_lock_full(mutex: *mut _GStaticRecMutex, depth: libc::c_uint);
}


/*
guint g_static_rec_mutex_unlock_full() [unsigned int]
	(GStaticRecMutex *) mutex [struct _GStaticRecMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_rec_mutex_unlock_full(mutex: *mut _GStaticRecMutex) -> libc::c_uint;
}


/*
void g_static_rec_mutex_free()
	(GStaticRecMutex *) mutex [struct _GStaticRecMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_rec_mutex_free(mutex: *mut _GStaticRecMutex);
}


/*
void g_static_rw_lock_init()
	(GStaticRWLock *) lock [struct _GStaticRWLock *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_rw_lock_init(lock: *mut _GStaticRWLock);
}


/*
void g_static_rw_lock_reader_lock()
	(GStaticRWLock *) lock [struct _GStaticRWLock *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_rw_lock_reader_lock(lock: *mut _GStaticRWLock);
}


/*
gboolean g_static_rw_lock_reader_trylock() [int]
	(GStaticRWLock *) lock [struct _GStaticRWLock *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_rw_lock_reader_trylock(lock: *mut _GStaticRWLock) -> libc::c_int;
}


/*
void g_static_rw_lock_reader_unlock()
	(GStaticRWLock *) lock [struct _GStaticRWLock *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_rw_lock_reader_unlock(lock: *mut _GStaticRWLock);
}


/*
void g_static_rw_lock_writer_lock()
	(GStaticRWLock *) lock [struct _GStaticRWLock *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_rw_lock_writer_lock(lock: *mut _GStaticRWLock);
}


/*
gboolean g_static_rw_lock_writer_trylock() [int]
	(GStaticRWLock *) lock [struct _GStaticRWLock *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_rw_lock_writer_trylock(lock: *mut _GStaticRWLock) -> libc::c_int;
}


/*
void g_static_rw_lock_writer_unlock()
	(GStaticRWLock *) lock [struct _GStaticRWLock *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_rw_lock_writer_unlock(lock: *mut _GStaticRWLock);
}


/*
void g_static_rw_lock_free()
	(GStaticRWLock *) lock [struct _GStaticRWLock *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_rw_lock_free(lock: *mut _GStaticRWLock);
}


/*
GPrivate * g_private_new() [struct _GPrivate *]
	(GDestroyNotify) notify [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_private_new(notify: Option<extern fn(*mut libc::c_void)>) -> *mut _GPrivate;
}


/*
void g_static_private_init()
	(GStaticPrivate *) private_key [struct _GStaticPrivate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_private_init(private_key: *mut _GStaticPrivate);
}


/*
gpointer g_static_private_get() [void *]
	(GStaticPrivate *) private_key [struct _GStaticPrivate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_private_get(private_key: *mut _GStaticPrivate) -> *mut libc::c_void;
}


/*
void g_static_private_set()
	(GStaticPrivate *) private_key [struct _GStaticPrivate *]
	(gpointer) data [void *]
	(GDestroyNotify) notify [void (*)(void *)]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_private_set(private_key: *mut _GStaticPrivate, data: *mut libc::c_void, notify: Option<extern fn(*mut libc::c_void)>);
}


/*
void g_static_private_free()
	(GStaticPrivate *) private_key [struct _GStaticPrivate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_static_private_free(private_key: *mut _GStaticPrivate);
}


/*
gboolean g_once_init_enter_impl() [int]
	(volatile gsize *) location [volatile unsigned long *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_once_init_enter_impl(location: *mut libc::c_ulong) -> libc::c_int;
}


/*
void g_thread_init()
	(gpointer) vtable [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_init(vtable: *mut libc::c_void);
}


/*
void g_thread_init_with_errorcheck_mutexes()
	(gpointer) vtable [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_init_with_errorcheck_mutexes(vtable: *mut libc::c_void);
}


/*
gboolean g_thread_get_initialized() [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_thread_get_initialized() -> libc::c_int;
}


/*
GMutex * g_mutex_new() [union _GMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mutex_new() -> *mut union _GMutex;
}


/*
void g_mutex_free()
	(GMutex *) mutex [union _GMutex *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_mutex_free(mutex: *mut union _GMutex);
}


/*
GCond * g_cond_new() [struct _GCond *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_cond_new() -> *mut _GCond;
}


/*
void g_cond_free()
	(GCond *) cond [struct _GCond *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_cond_free(cond: *mut _GCond);
}


/*
gboolean g_cond_timed_wait() [int]
	(GCond *) cond [struct _GCond *]
	(GMutex *) mutex [union _GMutex *]
	(GTimeVal *) timeval [struct _GTimeVal *]
*/
#[link(name="nice")]
extern "C" {
	pub fn g_cond_timed_wait(cond: *mut _GCond, mutex: *mut union _GMutex, timeval: *mut _GTimeVal) -> libc::c_int;
}


/*
struct 
		(guint) mantissa [unsigned int]
		(guint) biased_exponent [unsigned int]
		(guint) sign [unsigned int]
*/
#[repr(C)]
pub struct  {
	mantissa: libc::c_uint,
	biased_exponent: libc::c_uint,
	sign: libc::c_uint,
}

/*
struct 
		(guint) mantissa_low [unsigned int]
		(guint) mantissa_high [unsigned int]
		(guint) biased_exponent [unsigned int]
		(guint) sign [unsigned int]
*/
#[repr(C)]
pub struct  {
	mantissa_low: libc::c_uint,
	mantissa_high: libc::c_uint,
	biased_exponent: libc::c_uint,
	sign: libc::c_uint,
}

/*
struct _GTimeVal
		(glong) tv_sec [long]
		(glong) tv_usec [long]
*/
#[repr(C)]
pub struct _GTimeVal {
	tv_sec: libc::c_long,
	tv_usec: libc::c_long,
}

/*
struct _GBytes
*/
#[repr(C)]
pub struct _GBytes;

/*
struct _GArray
		(gchar *) data [char *]
		(guint) len [unsigned int]
*/
#[repr(C)]
pub struct _GArray {
	data: *mut libc::c_char,
	len: libc::c_uint,
}

/*
struct _GByteArray
		(guint8 *) data [unsigned char *]
		(guint) len [unsigned int]
*/
#[repr(C)]
pub struct _GByteArray {
	data: *mut libc::c_uchar,
	len: libc::c_uint,
}

/*
struct _GPtrArray
		(gpointer *) pdata [void **]
		(guint) len [unsigned int]
*/
#[repr(C)]
pub struct _GPtrArray {
	pdata: *mut *mut libc::c_void,
	len: libc::c_uint,
}

/*
struct _GError
		(GQuark) domain [unsigned int]
		(gint) code [int]
		(gchar *) message [char *]
*/
#[repr(C)]
pub struct _GError {
	domain: libc::c_uint,
	code: libc::c_int,
	message: *mut libc::c_char,
}

/*
struct _GThread
		(GThreadFunc) func [void *(*)(void *)]
		(gpointer) data [void *]
		(gboolean) joinable [int]
		(GThreadPriority) priority [GThreadPriority]
*/
#[repr(C)]
pub struct _GThread {
	func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>,
	data: *mut libc::c_void,
	joinable: libc::c_int,
	priority: libc::c_uint,
}

/*
struct _GRecMutex
		(gpointer) p [void *]
		(guint [2]) i [unsigned int [2]]
*/
#[repr(C)]
pub struct _GRecMutex {
	p: *mut libc::c_void,
	i: [libc::c_uint; 2],
}

/*
struct _GRWLock
		(gpointer) p [void *]
		(guint [2]) i [unsigned int [2]]
*/
#[repr(C)]
pub struct _GRWLock {
	p: *mut libc::c_void,
	i: [libc::c_uint; 2],
}

/*
struct _GCond
		(gpointer) p [void *]
		(guint [2]) i [unsigned int [2]]
*/
#[repr(C)]
pub struct _GCond {
	p: *mut libc::c_void,
	i: [libc::c_uint; 2],
}

/*
struct _GPrivate
		(gpointer) p [void *]
		(GDestroyNotify) notify [void (*)(void *)]
		(gpointer [2]) future [void *[2]]
*/
#[repr(C)]
pub struct _GPrivate {
	p: *mut libc::c_void,
	notify: Option<extern fn(*mut libc::c_void)>,
	future: [*mut libc::c_void; 2],
}

/*
struct _GOnce
		(volatile GOnceStatus) status [volatile GOnceStatus]
		(volatile gpointer) retval [void *volatile]
*/
#[repr(C)]
pub struct _GOnce {
	status: libc::c_uint,
	retval: *mut libc::c_void,
}

/*
struct _GAsyncQueue
*/
#[repr(C)]
pub struct _GAsyncQueue;

/*
struct _GBookmarkFile
*/
#[repr(C)]
pub struct _GBookmarkFile;

/*
struct _GChecksum
*/
#[repr(C)]
pub struct _GChecksum;

/*
struct _GIConv
*/
#[repr(C)]
pub struct _GIConv;

/*
struct _GData
*/
#[repr(C)]
pub struct _GData;

/*
struct _GDate
		(guint) julian_days [unsigned int]
		(guint) julian [unsigned int]
		(guint) dmy [unsigned int]
		(guint) day [unsigned int]
		(guint) month [unsigned int]
		(guint) year [unsigned int]
*/
#[repr(C)]
pub struct _GDate {
	julian_days: libc::c_uint,
	julian: libc::c_uint,
	dmy: libc::c_uint,
	day: libc::c_uint,
	month: libc::c_uint,
	year: libc::c_uint,
}

/*
struct _GTimeZone
*/
#[repr(C)]
pub struct _GTimeZone;

/*
struct _GDateTime
*/
#[repr(C)]
pub struct _GDateTime;

/*
struct _GDir
*/
#[repr(C)]
pub struct _GDir;

/*
struct _GMemVTable
		(gpointer (*)(gsize)) malloc [void *(*)(unsigned long)]
		(gpointer (*)(gpointer, gsize)) realloc [void *(*)(void *, unsigned long)]
		(void (*)(gpointer)) free [void (*)(void *)]
		(gpointer (*)(gsize, gsize)) calloc [void *(*)(unsigned long, unsigned long)]
		(gpointer (*)(gsize)) try_malloc [void *(*)(unsigned long)]
		(gpointer (*)(gpointer, gsize)) try_realloc [void *(*)(void *, unsigned long)]
*/
#[repr(C)]
pub struct _GMemVTable {
	malloc: Option<extern fn(libc::c_ulong) -> *mut libc::c_void>,
	realloc: Option<extern fn(*mut libc::c_void, libc::c_ulong) -> *mut libc::c_void>,
	free: Option<extern fn(*mut libc::c_void)>,
	calloc: Option<extern fn(libc::c_ulong, libc::c_ulong) -> *mut libc::c_void>,
	try_malloc: Option<extern fn(libc::c_ulong) -> *mut libc::c_void>,
	try_realloc: Option<extern fn(*mut libc::c_void, libc::c_ulong) -> *mut libc::c_void>,
}

/*
struct _GList
		(gpointer) data [void *]
		(GList *) next [struct _GList *]
		(GList *) prev [struct _GList *]
*/
#[repr(C)]
pub struct _GList {
	data: *mut libc::c_void,
	next: *mut _GList,
	prev: *mut _GList,
}

/*
struct _GHashTable
*/
#[repr(C)]
pub struct _GHashTable;

/*
struct _GHashTableIter
		(gpointer) dummy1 [void *]
		(gpointer) dummy2 [void *]
		(gpointer) dummy3 [void *]
		(int) dummy4
		(gboolean) dummy5 [int]
		(gpointer) dummy6 [void *]
*/
#[repr(C)]
pub struct _GHashTableIter {
	dummy1: *mut libc::c_void,
	dummy2: *mut libc::c_void,
	dummy3: *mut libc::c_void,
	dummy4: libc::c_int,
	dummy5: libc::c_int,
	dummy6: *mut libc::c_void,
}

/*
struct _GHmac
*/
#[repr(C)]
pub struct _GHmac;

/*
struct _GHook
		(gpointer) data [void *]
		(GHook *) next [struct _GHook *]
		(GHook *) prev [struct _GHook *]
		(guint) ref_count [unsigned int]
		(gulong) hook_id [unsigned long]
		(guint) flags [unsigned int]
		(gpointer) func [void *]
		(GDestroyNotify) destroy [void (*)(void *)]
*/
#[repr(C)]
pub struct _GHook {
	data: *mut libc::c_void,
	next: *mut _GHook,
	prev: *mut _GHook,
	ref_count: libc::c_uint,
	hook_id: libc::c_ulong,
	flags: libc::c_uint,
	func: *mut libc::c_void,
	destroy: Option<extern fn(*mut libc::c_void)>,
}

/*
struct _GHookList
		(gulong) seq_id [unsigned long]
		(guint) hook_size [unsigned int]
		(guint) is_setup [unsigned int]
		(GHook *) hooks [struct _GHook *]
		(gpointer) dummy3 [void *]
		(GHookFinalizeFunc) finalize_hook [void (*)(struct _GHookList *, struct _GHook *)]
		(gpointer [2]) dummy [void *[2]]
*/
#[repr(C)]
pub struct _GHookList {
	seq_id: libc::c_ulong,
	hook_size: libc::c_uint,
	is_setup: libc::c_uint,
	hooks: *mut _GHook,
	dummy3: *mut libc::c_void,
	finalize_hook: Option<extern fn(*mut _GHookList, *mut _GHook)>,
	dummy: [*mut libc::c_void; 2],
}

/*
struct _GPollFD
		(gint) fd [int]
		(gushort) events [unsigned short]
		(gushort) revents [unsigned short]
*/
#[repr(C)]
pub struct _GPollFD {
	fd: libc::c_int,
	events: libc::c_ushort,
	revents: libc::c_ushort,
}

/*
struct _GSList
		(gpointer) data [void *]
		(GSList *) next [struct _GSList *]
*/
#[repr(C)]
pub struct _GSList {
	data: *mut libc::c_void,
	next: *mut _GSList,
}

/*
struct _GMainContext
*/
#[repr(C)]
pub struct _GMainContext;

/*
struct _GMainLoop
*/
#[repr(C)]
pub struct _GMainLoop;

/*
struct _GSource
		(gpointer) callback_data [void *]
		(GSourceCallbackFuncs *) callback_funcs [struct _GSourceCallbackFuncs *]
		(GSourceFuncs *) source_funcs [struct _GSourceFuncs *]
		(guint) ref_count [unsigned int]
		(GMainContext *) context [struct _GMainContext *]
		(gint) priority [int]
		(guint) flags [unsigned int]
		(guint) source_id [unsigned int]
		(GSList *) poll_fds [struct _GSList *]
		(GSource *) prev [struct _GSource *]
		(GSource *) next [struct _GSource *]
		(char *) name
		(GSourcePrivate *) priv [struct _GSourcePrivate *]
*/
#[repr(C)]
pub struct _GSource {
	callback_data: *mut libc::c_void,
	callback_funcs: *mut _GSourceCallbackFuncs,
	source_funcs: *mut _GSourceFuncs,
	ref_count: libc::c_uint,
	context: *mut _GMainContext,
	priority: libc::c_int,
	flags: libc::c_uint,
	source_id: libc::c_uint,
	poll_fds: *mut _GSList,
	prev: *mut _GSource,
	next: *mut _GSource,
	name: *mut libc::c_char,
	priv_: *mut _GSourcePrivate,
}

/*
struct _GSourcePrivate
*/
#[repr(C)]
pub struct _GSourcePrivate;

/*
struct _GSourceCallbackFuncs
		(void (*)(gpointer)) ref [void (*)(void *)]
		(void (*)(gpointer)) unref [void (*)(void *)]
		(void (*)(gpointer, GSource *, GSourceFunc *, gpointer *)) get [void (*)(void *, struct _GSource *, int (**)(void *), void **)]
*/
#[repr(C)]
pub struct _GSourceCallbackFuncs {
	ref_: Option<extern fn(*mut libc::c_void)>,
	unref: Option<extern fn(*mut libc::c_void)>,
	get: Option<extern fn(*mut libc::c_void, *mut _GSource, *mut Option<extern fn(*mut libc::c_void) -> libc::c_int>, *mut *mut libc::c_void)>,
}

/*
struct _GSourceFuncs
		(gboolean (*)(GSource *, gint *)) prepare [int (*)(struct _GSource *, int *)]
		(gboolean (*)(GSource *)) check [int (*)(struct _GSource *)]
		(gboolean (*)(GSource *, GSourceFunc, gpointer)) dispatch [int (*)(struct _GSource *, int (*)(void *), void *)]
		(void (*)(GSource *)) finalize [void (*)(struct _GSource *)]
		(GSourceFunc) closure_callback [int (*)(void *)]
		(GSourceDummyMarshal) closure_marshal [void (*)(void)]
*/
#[repr(C)]
pub struct _GSourceFuncs {
	prepare: Option<extern fn(*mut _GSource, *mut libc::c_int) -> libc::c_int>,
	check: Option<extern fn(*mut _GSource) -> libc::c_int>,
	dispatch: Option<extern fn(*mut _GSource, Option<extern fn(*mut libc::c_void) -> libc::c_int>, *mut libc::c_void) -> libc::c_int>,
	finalize: Option<extern fn(*mut _GSource)>,
	closure_callback: Option<extern fn(*mut libc::c_void) -> libc::c_int>,
	closure_marshal: Option<extern fn()>,
}

/*
struct _GDebugKey
		(const gchar *) key [const char *]
		(guint) value [unsigned int]
*/
#[repr(C)]
pub struct _GDebugKey {
	key: *const libc::c_char,
	value: libc::c_uint,
}

/*
struct _GString
		(gchar *) str [char *]
		(gsize) len [unsigned long]
		(gsize) allocated_len [unsigned long]
*/
#[repr(C)]
pub struct _GString {
	str: *mut libc::c_char,
	len: libc::c_ulong,
	allocated_len: libc::c_ulong,
}

/*
struct _GIOChannel
		(gint) ref_count [int]
		(GIOFuncs *) funcs [struct _GIOFuncs *]
		(gchar *) encoding [char *]
		(GIConv) read_cd [struct _GIConv *]
		(GIConv) write_cd [struct _GIConv *]
		(gchar *) line_term [char *]
		(guint) line_term_len [unsigned int]
		(gsize) buf_size [unsigned long]
		(GString *) read_buf [struct _GString *]
		(GString *) encoded_read_buf [struct _GString *]
		(GString *) write_buf [struct _GString *]
		(gchar [6]) partial_write_buf [char [6]]
		(guint) use_buffer [unsigned int]
		(guint) do_encode [unsigned int]
		(guint) close_on_unref [unsigned int]
		(guint) is_readable [unsigned int]
		(guint) is_writeable [unsigned int]
		(guint) is_seekable [unsigned int]
		(gpointer) reserved1 [void *]
		(gpointer) reserved2 [void *]
*/
#[repr(C)]
pub struct _GIOChannel {
	ref_count: libc::c_int,
	funcs: *mut _GIOFuncs,
	encoding: *mut libc::c_char,
	read_cd: *mut _GIConv,
	write_cd: *mut _GIConv,
	line_term: *mut libc::c_char,
	line_term_len: libc::c_uint,
	buf_size: libc::c_ulong,
	read_buf: *mut _GString,
	encoded_read_buf: *mut _GString,
	write_buf: *mut _GString,
	partial_write_buf: [libc::c_char; 6],
	use_buffer: libc::c_uint,
	do_encode: libc::c_uint,
	close_on_unref: libc::c_uint,
	is_readable: libc::c_uint,
	is_writeable: libc::c_uint,
	is_seekable: libc::c_uint,
	reserved1: *mut libc::c_void,
	reserved2: *mut libc::c_void,
}

/*
struct _GIOFuncs
		(GIOStatus (*)(GIOChannel *, gchar *, gsize, gsize *, GError **)) io_read [GIOStatus (*)(struct _GIOChannel *, char *, unsigned long, unsigned long *, struct _GError **)]
		(GIOStatus (*)(GIOChannel *, const gchar *, gsize, gsize *, GError **)) io_write [GIOStatus (*)(struct _GIOChannel *, const char *, unsigned long, unsigned long *, struct _GError **)]
		(GIOStatus (*)(GIOChannel *, gint64, GSeekType, GError **)) io_seek [GIOStatus (*)(struct _GIOChannel *, long, GSeekType, struct _GError **)]
		(GIOStatus (*)(GIOChannel *, GError **)) io_close [GIOStatus (*)(struct _GIOChannel *, struct _GError **)]
		(GSource *(*)(GIOChannel *, GIOCondition)) io_create_watch [struct _GSource *(*)(struct _GIOChannel *, GIOCondition)]
		(void (*)(GIOChannel *)) io_free [void (*)(struct _GIOChannel *)]
		(GIOStatus (*)(GIOChannel *, GIOFlags, GError **)) io_set_flags [GIOStatus (*)(struct _GIOChannel *, GIOFlags, struct _GError **)]
		(GIOFlags (*)(GIOChannel *)) io_get_flags [GIOFlags (*)(struct _GIOChannel *)]
*/
#[repr(C)]
pub struct _GIOFuncs {
	io_read: Option<extern fn(*mut _GIOChannel, *mut libc::c_char, libc::c_ulong, *mut libc::c_ulong, *mut *mut _GError) -> libc::c_uint>,
	io_write: Option<extern fn(*mut _GIOChannel, *const libc::c_char, libc::c_ulong, *mut libc::c_ulong, *mut *mut _GError) -> libc::c_uint>,
	io_seek: Option<extern fn(*mut _GIOChannel, libc::c_long, libc::c_uint, *mut *mut _GError) -> libc::c_uint>,
	io_close: Option<extern fn(*mut _GIOChannel, *mut *mut _GError) -> libc::c_uint>,
	io_create_watch: Option<extern fn(*mut _GIOChannel, libc::c_uint) -> *mut _GSource>,
	io_free: Option<extern fn(*mut _GIOChannel)>,
	io_set_flags: Option<extern fn(*mut _GIOChannel, libc::c_uint, *mut *mut _GError) -> libc::c_uint>,
	io_get_flags: Option<extern fn(*mut _GIOChannel) -> libc::c_uint>,
}

/*
struct _GKeyFile
*/
#[repr(C)]
pub struct _GKeyFile;

/*
struct _GMappedFile
*/
#[repr(C)]
pub struct _GMappedFile;

/*
struct _GMarkupParseContext
*/
#[repr(C)]
pub struct _GMarkupParseContext;

/*
struct _GMarkupParser
		(void (*)(GMarkupParseContext *, const gchar *, const gchar **, const gchar **, gpointer, GError **)) start_element [void (*)(struct _GMarkupParseContext *, const char *, const char **, const char **, void *, struct _GError **)]
		(void (*)(GMarkupParseContext *, const gchar *, gpointer, GError **)) end_element [void (*)(struct _GMarkupParseContext *, const char *, void *, struct _GError **)]
		(void (*)(GMarkupParseContext *, const gchar *, gsize, gpointer, GError **)) text [void (*)(struct _GMarkupParseContext *, const char *, unsigned long, void *, struct _GError **)]
		(void (*)(GMarkupParseContext *, const gchar *, gsize, gpointer, GError **)) passthrough [void (*)(struct _GMarkupParseContext *, const char *, unsigned long, void *, struct _GError **)]
		(void (*)(GMarkupParseContext *, GError *, gpointer)) error [void (*)(struct _GMarkupParseContext *, struct _GError *, void *)]
*/
#[repr(C)]
pub struct _GMarkupParser {
	start_element: Option<extern fn(*mut _GMarkupParseContext, *const libc::c_char, *mut *const libc::c_char, *mut *const libc::c_char, *mut libc::c_void, *mut *mut _GError)>,
	end_element: Option<extern fn(*mut _GMarkupParseContext, *const libc::c_char, *mut libc::c_void, *mut *mut _GError)>,
	text: Option<extern fn(*mut _GMarkupParseContext, *const libc::c_char, libc::c_ulong, *mut libc::c_void, *mut *mut _GError)>,
	passthrough: Option<extern fn(*mut _GMarkupParseContext, *const libc::c_char, libc::c_ulong, *mut libc::c_void, *mut *mut _GError)>,
	error: Option<extern fn(*mut _GMarkupParseContext, *mut _GError, *mut libc::c_void)>,
}

/*
struct _GNode
		(gpointer) data [void *]
		(GNode *) next [struct _GNode *]
		(GNode *) prev [struct _GNode *]
		(GNode *) parent [struct _GNode *]
		(GNode *) children [struct _GNode *]
*/
#[repr(C)]
pub struct _GNode {
	data: *mut libc::c_void,
	next: *mut _GNode,
	prev: *mut _GNode,
	parent: *mut _GNode,
	children: *mut _GNode,
}

/*
struct _GOptionContext
*/
#[repr(C)]
pub struct _GOptionContext;

/*
struct _GOptionGroup
*/
#[repr(C)]
pub struct _GOptionGroup;

/*
struct _GOptionEntry
		(const gchar *) long_name [const char *]
		(gchar) short_name [char]
		(gint) flags [int]
		(GOptionArg) arg [GOptionArg]
		(gpointer) arg_data [void *]
		(const gchar *) description [const char *]
		(const gchar *) arg_description [const char *]
*/
#[repr(C)]
pub struct _GOptionEntry {
	long_name: *const libc::c_char,
	short_name: libc::c_char,
	flags: libc::c_int,
	arg: libc::c_uint,
	arg_data: *mut libc::c_void,
	description: *const libc::c_char,
	arg_description: *const libc::c_char,
}

/*
struct _GPatternSpec
*/
#[repr(C)]
pub struct _GPatternSpec;

/*
struct _GQueue
		(GList *) head [struct _GList *]
		(GList *) tail [struct _GList *]
		(guint) length [unsigned int]
*/
#[repr(C)]
pub struct _GQueue {
	head: *mut _GList,
	tail: *mut _GList,
	length: libc::c_uint,
}

/*
struct _GRand
*/
#[repr(C)]
pub struct _GRand;

/*
struct _GRegex
*/
#[repr(C)]
pub struct _GRegex;

/*
struct _GMatchInfo
*/
#[repr(C)]
pub struct _GMatchInfo;

/*
struct _GScanner
		(gpointer) user_data [void *]
		(guint) max_parse_errors [unsigned int]
		(guint) parse_errors [unsigned int]
		(const gchar *) input_name [const char *]
		(GData *) qdata [struct _GData *]
		(GScannerConfig *) config [struct _GScannerConfig *]
		(GTokenType) token [GTokenType]
		(GTokenValue) value [union _GTokenValue]
		(guint) line [unsigned int]
		(guint) position [unsigned int]
		(GTokenType) next_token [GTokenType]
		(GTokenValue) next_value [union _GTokenValue]
		(guint) next_line [unsigned int]
		(guint) next_position [unsigned int]
		(GHashTable *) symbol_table [struct _GHashTable *]
		(gint) input_fd [int]
		(const gchar *) text [const char *]
		(const gchar *) text_end [const char *]
		(gchar *) buffer [char *]
		(guint) scope_id [unsigned int]
		(GScannerMsgFunc) msg_handler [void (*)(struct _GScanner *, char *, int)]
*/
#[repr(C)]
pub struct _GScanner {
	user_data: *mut libc::c_void,
	max_parse_errors: libc::c_uint,
	parse_errors: libc::c_uint,
	input_name: *const libc::c_char,
	qdata: *mut _GData,
	config: *mut _GScannerConfig,
	token: libc::c_uint,
	value: union _GTokenValue,
	line: libc::c_uint,
	position: libc::c_uint,
	next_token: libc::c_uint,
	next_value: union _GTokenValue,
	next_line: libc::c_uint,
	next_position: libc::c_uint,
	symbol_table: *mut _GHashTable,
	input_fd: libc::c_int,
	text: *const libc::c_char,
	text_end: *const libc::c_char,
	buffer: *mut libc::c_char,
	scope_id: libc::c_uint,
	msg_handler: Option<extern fn(*mut _GScanner, *mut libc::c_char, libc::c_int)>,
}

/*
struct _GScannerConfig
		(gchar *) cset_skip_characters [char *]
		(gchar *) cset_identifier_first [char *]
		(gchar *) cset_identifier_nth [char *]
		(gchar *) cpair_comment_single [char *]
		(guint) case_sensitive [unsigned int]
		(guint) skip_comment_multi [unsigned int]
		(guint) skip_comment_single [unsigned int]
		(guint) scan_comment_multi [unsigned int]
		(guint) scan_identifier [unsigned int]
		(guint) scan_identifier_1char [unsigned int]
		(guint) scan_identifier_NULL [unsigned int]
		(guint) scan_symbols [unsigned int]
		(guint) scan_binary [unsigned int]
		(guint) scan_octal [unsigned int]
		(guint) scan_float [unsigned int]
		(guint) scan_hex [unsigned int]
		(guint) scan_hex_dollar [unsigned int]
		(guint) scan_string_sq [unsigned int]
		(guint) scan_string_dq [unsigned int]
		(guint) numbers_2_int [unsigned int]
		(guint) int_2_float [unsigned int]
		(guint) identifier_2_string [unsigned int]
		(guint) char_2_token [unsigned int]
		(guint) symbol_2_token [unsigned int]
		(guint) scope_0_fallback [unsigned int]
		(guint) store_int64 [unsigned int]
		(guint) padding_dummy [unsigned int]
*/
#[repr(C)]
pub struct _GScannerConfig {
	cset_skip_characters: *mut libc::c_char,
	cset_identifier_first: *mut libc::c_char,
	cset_identifier_nth: *mut libc::c_char,
	cpair_comment_single: *mut libc::c_char,
	case_sensitive: libc::c_uint,
	skip_comment_multi: libc::c_uint,
	skip_comment_single: libc::c_uint,
	scan_comment_multi: libc::c_uint,
	scan_identifier: libc::c_uint,
	scan_identifier_1char: libc::c_uint,
	scan_identifier_NULL: libc::c_uint,
	scan_symbols: libc::c_uint,
	scan_binary: libc::c_uint,
	scan_octal: libc::c_uint,
	scan_float: libc::c_uint,
	scan_hex: libc::c_uint,
	scan_hex_dollar: libc::c_uint,
	scan_string_sq: libc::c_uint,
	scan_string_dq: libc::c_uint,
	numbers_2_int: libc::c_uint,
	int_2_float: libc::c_uint,
	identifier_2_string: libc::c_uint,
	char_2_token: libc::c_uint,
	symbol_2_token: libc::c_uint,
	scope_0_fallback: libc::c_uint,
	store_int64: libc::c_uint,
	padding_dummy: libc::c_uint,
}

/*
struct _GSequence
*/
#[repr(C)]
pub struct _GSequence;

/*
struct _GSequenceNode
*/
#[repr(C)]
pub struct _GSequenceNode;

/*
struct _GStringChunk
*/
#[repr(C)]
pub struct _GStringChunk;

/*
struct GTestCase
*/
#[repr(C)]
pub struct GTestCase;

/*
struct GTestSuite
*/
#[repr(C)]
pub struct GTestSuite;

/*
struct 
		(gboolean) test_initialized [int]
		(gboolean) test_quick [int]
		(gboolean) test_perf [int]
		(gboolean) test_verbose [int]
		(gboolean) test_quiet [int]
		(gboolean) test_undefined [int]
*/
#[repr(C)]
pub struct  {
	test_initialized: libc::c_int,
	test_quick: libc::c_int,
	test_perf: libc::c_int,
	test_verbose: libc::c_int,
	test_quiet: libc::c_int,
	test_undefined: libc::c_int,
}

/*
struct 
		(GTestLogType) log_type [GTestLogType]
		(guint) n_strings [unsigned int]
		(gchar **) strings [char **]
		(guint) n_nums [unsigned int]
		(long double *) nums
*/
#[repr(C)]
pub struct  {
	log_type: libc::c_uint,
	n_strings: libc::c_uint,
	strings: *mut *mut libc::c_char,
	n_nums: libc::c_uint,
	nums: *mut TypeKind.LONGDOUBLE,
}

/*
struct 
		(GString *) data [struct _GString *]
		(GSList *) msgs [struct _GSList *]
*/
#[repr(C)]
pub struct  {
	data: *mut _GString,
	msgs: *mut _GSList,
}

/*
struct _GThreadPool
		(GFunc) func [void (*)(void *, void *)]
		(gpointer) user_data [void *]
		(gboolean) exclusive [int]
*/
#[repr(C)]
pub struct _GThreadPool {
	func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>,
	user_data: *mut libc::c_void,
	exclusive: libc::c_int,
}

/*
struct _GTimer
*/
#[repr(C)]
pub struct _GTimer;

/*
struct _GTrashStack
		(GTrashStack *) next [struct _GTrashStack *]
*/
#[repr(C)]
pub struct _GTrashStack {
	next: *mut _GTrashStack,
}

/*
struct _GTree
*/
#[repr(C)]
pub struct _GTree;

/*
struct _GVariantType
*/
#[repr(C)]
pub struct _GVariantType;

/*
struct _GVariant
*/
#[repr(C)]
pub struct _GVariant;

/*
struct _GVariantIter
		(gsize [16]) x [unsigned long [16]]
*/
#[repr(C)]
pub struct _GVariantIter {
	x: [libc::c_ulong; 16],
}

/*
struct _GVariantBuilder
		(gsize [16]) x [unsigned long [16]]
*/
#[repr(C)]
pub struct _GVariantBuilder {
	x: [libc::c_ulong; 16],
}

/*
struct _GAllocator
*/
#[repr(C)]
pub struct _GAllocator;

/*
struct _GMemChunk
*/
#[repr(C)]
pub struct _GMemChunk;

/*
struct _GCache
*/
#[repr(C)]
pub struct _GCache;

/*
struct _GCompletion
		(GList *) items [struct _GList *]
		(GCompletionFunc) func [char *(*)(void *)]
		(gchar *) prefix [char *]
		(GList *) cache [struct _GList *]
		(GCompletionStrncmpFunc) strncmp_func [int (*)(const char *, const char *, unsigned long)]
*/
#[repr(C)]
pub struct _GCompletion {
	items: *mut _GList,
	func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_char>,
	prefix: *mut libc::c_char,
	cache: *mut _GList,
	strncmp_func: Option<extern fn(*const libc::c_char, *const libc::c_char, libc::c_ulong) -> libc::c_int>,
}

/*
struct _GRelation
*/
#[repr(C)]
pub struct _GRelation;

/*
struct _GTuples
		(guint) len [unsigned int]
*/
#[repr(C)]
pub struct _GTuples {
	len: libc::c_uint,
}

/*
struct _GThreadFunctions
		(GMutex *(*)(void)) mutex_new [union _GMutex *(*)(void)]
		(void (*)(GMutex *)) mutex_lock [void (*)(union _GMutex *)]
		(gboolean (*)(GMutex *)) mutex_trylock [int (*)(union _GMutex *)]
		(void (*)(GMutex *)) mutex_unlock [void (*)(union _GMutex *)]
		(void (*)(GMutex *)) mutex_free [void (*)(union _GMutex *)]
		(GCond *(*)(void)) cond_new [struct _GCond *(*)(void)]
		(void (*)(GCond *)) cond_signal [void (*)(struct _GCond *)]
		(void (*)(GCond *)) cond_broadcast [void (*)(struct _GCond *)]
		(void (*)(GCond *, GMutex *)) cond_wait [void (*)(struct _GCond *, union _GMutex *)]
		(gboolean (*)(GCond *, GMutex *, GTimeVal *)) cond_timed_wait [int (*)(struct _GCond *, union _GMutex *, struct _GTimeVal *)]
		(void (*)(GCond *)) cond_free [void (*)(struct _GCond *)]
		(GPrivate *(*)(GDestroyNotify)) private_new [struct _GPrivate *(*)(void (*)(void *))]
		(gpointer (*)(GPrivate *)) private_get [void *(*)(struct _GPrivate *)]
		(void (*)(GPrivate *, gpointer)) private_set [void (*)(struct _GPrivate *, void *)]
		(void (*)(GThreadFunc, gpointer, gulong, gboolean, gboolean, GThreadPriority, gpointer, GError **)) thread_create [void (*)(void *(*)(void *), void *, unsigned long, int, int, GThreadPriority, void *, struct _GError **)]
		(void (*)(void)) thread_yield [void (*)(void)]
		(void (*)(gpointer)) thread_join [void (*)(void *)]
		(void (*)(void)) thread_exit [void (*)(void)]
		(void (*)(gpointer, GThreadPriority)) thread_set_priority [void (*)(void *, GThreadPriority)]
		(void (*)(gpointer)) thread_self [void (*)(void *)]
		(gboolean (*)(gpointer, gpointer)) thread_equal [int (*)(void *, void *)]
*/
#[repr(C)]
pub struct _GThreadFunctions {
	mutex_new: Option<extern fn() -> *mut union _GMutex>,
	mutex_lock: Option<extern fn(*mut union _GMutex)>,
	mutex_trylock: Option<extern fn(*mut union _GMutex) -> libc::c_int>,
	mutex_unlock: Option<extern fn(*mut union _GMutex)>,
	mutex_free: Option<extern fn(*mut union _GMutex)>,
	cond_new: Option<extern fn() -> *mut _GCond>,
	cond_signal: Option<extern fn(*mut _GCond)>,
	cond_broadcast: Option<extern fn(*mut _GCond)>,
	cond_wait: Option<extern fn(*mut _GCond, *mut union _GMutex)>,
	cond_timed_wait: Option<extern fn(*mut _GCond, *mut union _GMutex, *mut _GTimeVal) -> libc::c_int>,
	cond_free: Option<extern fn(*mut _GCond)>,
	private_new: Option<extern fn(Option<extern fn(*mut libc::c_void)>) -> *mut _GPrivate>,
	private_get: Option<extern fn(*mut _GPrivate) -> *mut libc::c_void>,
	private_set: Option<extern fn(*mut _GPrivate, *mut libc::c_void)>,
	thread_create: Option<extern fn(Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>, *mut libc::c_void, libc::c_ulong, libc::c_int, libc::c_int, libc::c_uint, *mut libc::c_void, *mut *mut _GError)>,
	thread_yield: Option<extern fn()>,
	thread_join: Option<extern fn(*mut libc::c_void)>,
	thread_exit: Option<extern fn()>,
	thread_set_priority: Option<extern fn(*mut libc::c_void, libc::c_uint)>,
	thread_self: Option<extern fn(*mut libc::c_void)>,
	thread_equal: Option<extern fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int>,
}

/*
struct 
		(GMutex *) mutex [union _GMutex *]
		(pthread_mutex_t) unused [pthread_mutex_t]
*/
#[repr(C)]
pub struct  {
	mutex: *mut union _GMutex,
	unused: pthread_mutex_t,
}

/*
struct _GStaticRecMutex
		(GStaticMutex) mutex [GStaticMutex]
		(guint) depth [unsigned int]
		(union _GStaticRecMutex::(anonymous at /usr/include/glib-2.0/glib/deprecated/gthread.h:158:3)) 
		(union (anonymous union at /usr/include/glib-2.0/glib/deprecated/gthread.h:158:3)) unused [union _GStaticRecMutex::(anonymous at /usr/include/glib-2.0/glib/deprecated/gthread.h:158:3)]
*/
#[repr(C)]
pub struct _GStaticRecMutex {
	mutex: GStaticMutex,
	depth: libc::c_uint,
	: union _GStaticRecMutex::(anonymous at /usr/include/glib-2.0/glib/deprecated/gthread.h:158:3),
	unused: union _GStaticRecMutex::(anonymous at /usr/include/glib-2.0/glib/deprecated/gthread.h:158:3),
}

/*
struct _GStaticRWLock
		(GStaticMutex) mutex [GStaticMutex]
		(GCond *) read_cond [struct _GCond *]
		(GCond *) write_cond [struct _GCond *]
		(guint) read_counter [unsigned int]
		(gboolean) have_writer [int]
		(guint) want_to_read [unsigned int]
		(guint) want_to_write [unsigned int]
*/
#[repr(C)]
pub struct _GStaticRWLock {
	mutex: GStaticMutex,
	read_cond: *mut _GCond,
	write_cond: *mut _GCond,
	read_counter: libc::c_uint,
	have_writer: libc::c_int,
	want_to_read: libc::c_uint,
	want_to_write: libc::c_uint,
}

/*
struct _GStaticPrivate
		(guint) index [unsigned int]
*/
#[repr(C)]
pub struct _GStaticPrivate {
	index: libc::c_uint,
}

/*
struct GArray
		(gchar *) data [char *]
		(guint) len [unsigned int]
*/
#[repr(C)]
pub struct GArray {
	data: *mut libc::c_char,
	len: libc::c_uint,
}

/*
struct GPtrArray
		(gpointer *) pdata [void **]
		(guint) len [unsigned int]
*/
#[repr(C)]
pub struct GPtrArray {
	pdata: *mut *mut libc::c_void,
	len: libc::c_uint,
}

/*
struct GByteArray
		(guint8 *) data [unsigned char *]
		(guint) len [unsigned int]
*/
#[repr(C)]
pub struct GByteArray {
	data: *mut libc::c_uchar,
	len: libc::c_uint,
}

/*
struct GError
		(GQuark) domain [unsigned int]
		(gint) code [int]
		(gchar *) message [char *]
*/
#[repr(C)]
pub struct GError {
	domain: libc::c_uint,
	code: libc::c_int,
	message: *mut libc::c_char,
}

/*
struct GThread
		(GThreadFunc) func [void *(*)(void *)]
		(gpointer) data [void *]
		(gboolean) joinable [int]
		(GThreadPriority) priority [GThreadPriority]
*/
#[repr(C)]
pub struct GThread {
	func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>,
	data: *mut libc::c_void,
	joinable: libc::c_int,
	priority: libc::c_uint,
}

/*
struct GMutex
		(gpointer) p [void *]
		(guint [2]) i [unsigned int [2]]
*/
#[repr(C)]
pub struct GMutex {
	p: *mut libc::c_void,
	i: [libc::c_uint; 2],
}

/*
struct GRWLock
		(gpointer) p [void *]
		(guint [2]) i [unsigned int [2]]
*/
#[repr(C)]
pub struct GRWLock {
	p: *mut libc::c_void,
	i: [libc::c_uint; 2],
}

/*
struct GRecMutex
		(gpointer) p [void *]
		(guint [2]) i [unsigned int [2]]
*/
#[repr(C)]
pub struct GRecMutex {
	p: *mut libc::c_void,
	i: [libc::c_uint; 2],
}

/*
struct GCond
		(gpointer) p [void *]
		(guint [2]) i [unsigned int [2]]
*/
#[repr(C)]
pub struct GCond {
	p: *mut libc::c_void,
	i: [libc::c_uint; 2],
}

/*
struct GPrivate
		(gpointer) p [void *]
		(GDestroyNotify) notify [void (*)(void *)]
		(gpointer [2]) future [void *[2]]
*/
#[repr(C)]
pub struct GPrivate {
	p: *mut libc::c_void,
	notify: Option<extern fn(*mut libc::c_void)>,
	future: [*mut libc::c_void; 2],
}

/*
struct GOnce
		(volatile GOnceStatus) status [volatile GOnceStatus]
		(volatile gpointer) retval [void *volatile]
*/
#[repr(C)]
pub struct GOnce {
	status: libc::c_uint,
	retval: *mut libc::c_void,
}

/*
struct GAsyncQueue
*/
#[repr(C)]
pub struct GAsyncQueue;

/*
struct GTimeVal
		(glong) tv_sec [long]
		(glong) tv_usec [long]
*/
#[repr(C)]
pub struct GTimeVal {
	tv_sec: libc::c_long,
	tv_usec: libc::c_long,
}

/*
struct GBookmarkFile
*/
#[repr(C)]
pub struct GBookmarkFile;

/*
struct GBytes
*/
#[repr(C)]
pub struct GBytes;

/*
struct GChecksum
*/
#[repr(C)]
pub struct GChecksum;

/*
struct None
*/
#[repr(C)]
pub struct None;

/*
struct GDate
		(guint) julian_days [unsigned int]
		(guint) julian [unsigned int]
		(guint) dmy [unsigned int]
		(guint) day [unsigned int]
		(guint) month [unsigned int]
		(guint) year [unsigned int]
*/
#[repr(C)]
pub struct GDate {
	julian_days: libc::c_uint,
	julian: libc::c_uint,
	dmy: libc::c_uint,
	day: libc::c_uint,
	month: libc::c_uint,
	year: libc::c_uint,
}

/*
struct tm
		(int) tm_sec
		(int) tm_min
		(int) tm_hour
		(int) tm_mday
		(int) tm_mon
		(int) tm_year
		(int) tm_wday
		(int) tm_yday
		(int) tm_isdst
		(long) tm_gmtoff
		(const char *) tm_zone
*/
#[repr(C)]
pub struct tm {
	tm_sec: libc::c_int,
	tm_min: libc::c_int,
	tm_hour: libc::c_int,
	tm_mday: libc::c_int,
	tm_mon: libc::c_int,
	tm_year: libc::c_int,
	tm_wday: libc::c_int,
	tm_yday: libc::c_int,
	tm_isdst: libc::c_int,
	tm_gmtoff: libc::c_long,
	tm_zone: *const libc::c_char,
}

/*
struct GTimeZone
*/
#[repr(C)]
pub struct GTimeZone;

/*
struct GDateTime
*/
#[repr(C)]
pub struct GDateTime;

/*
struct GDir
*/
#[repr(C)]
pub struct GDir;

/*
struct GMemVTable
		(gpointer (*)(gsize)) malloc [void *(*)(unsigned long)]
		(gpointer (*)(gpointer, gsize)) realloc [void *(*)(void *, unsigned long)]
		(void (*)(gpointer)) free [void (*)(void *)]
		(gpointer (*)(gsize, gsize)) calloc [void *(*)(unsigned long, unsigned long)]
		(gpointer (*)(gsize)) try_malloc [void *(*)(unsigned long)]
		(gpointer (*)(gpointer, gsize)) try_realloc [void *(*)(void *, unsigned long)]
*/
#[repr(C)]
pub struct GMemVTable {
	malloc: Option<extern fn(libc::c_ulong) -> *mut libc::c_void>,
	realloc: Option<extern fn(*mut libc::c_void, libc::c_ulong) -> *mut libc::c_void>,
	free: Option<extern fn(*mut libc::c_void)>,
	calloc: Option<extern fn(libc::c_ulong, libc::c_ulong) -> *mut libc::c_void>,
	try_malloc: Option<extern fn(libc::c_ulong) -> *mut libc::c_void>,
	try_realloc: Option<extern fn(*mut libc::c_void, libc::c_ulong) -> *mut libc::c_void>,
}

/*
struct GList
		(gpointer) data [void *]
		(GList *) next [struct _GList *]
		(GList *) prev [struct _GList *]
*/
#[repr(C)]
pub struct GList {
	data: *mut libc::c_void,
	next: *mut _GList,
	prev: *mut _GList,
}

/*
struct GHashTable
*/
#[repr(C)]
pub struct GHashTable;

/*
struct GHashTableIter
		(gpointer) dummy1 [void *]
		(gpointer) dummy2 [void *]
		(gpointer) dummy3 [void *]
		(int) dummy4
		(gboolean) dummy5 [int]
		(gpointer) dummy6 [void *]
*/
#[repr(C)]
pub struct GHashTableIter {
	dummy1: *mut libc::c_void,
	dummy2: *mut libc::c_void,
	dummy3: *mut libc::c_void,
	dummy4: libc::c_int,
	dummy5: libc::c_int,
	dummy6: *mut libc::c_void,
}

/*
struct GHmac
*/
#[repr(C)]
pub struct GHmac;

/*
struct GHookList
		(gulong) seq_id [unsigned long]
		(guint) hook_size [unsigned int]
		(guint) is_setup [unsigned int]
		(GHook *) hooks [struct _GHook *]
		(gpointer) dummy3 [void *]
		(GHookFinalizeFunc) finalize_hook [void (*)(struct _GHookList *, struct _GHook *)]
		(gpointer [2]) dummy [void *[2]]
*/
#[repr(C)]
pub struct GHookList {
	seq_id: libc::c_ulong,
	hook_size: libc::c_uint,
	is_setup: libc::c_uint,
	hooks: *mut _GHook,
	dummy3: *mut libc::c_void,
	finalize_hook: Option<extern fn(*mut _GHookList, *mut _GHook)>,
	dummy: [*mut libc::c_void; 2],
}

/*
struct GHook
		(gpointer) data [void *]
		(GHook *) next [struct _GHook *]
		(GHook *) prev [struct _GHook *]
		(guint) ref_count [unsigned int]
		(gulong) hook_id [unsigned long]
		(guint) flags [unsigned int]
		(gpointer) func [void *]
		(GDestroyNotify) destroy [void (*)(void *)]
*/
#[repr(C)]
pub struct GHook {
	data: *mut libc::c_void,
	next: *mut _GHook,
	prev: *mut _GHook,
	ref_count: libc::c_uint,
	hook_id: libc::c_ulong,
	flags: libc::c_uint,
	func: *mut libc::c_void,
	destroy: Option<extern fn(*mut libc::c_void)>,
}

/*
struct GPollFD
		(gint) fd [int]
		(gushort) events [unsigned short]
		(gushort) revents [unsigned short]
*/
#[repr(C)]
pub struct GPollFD {
	fd: libc::c_int,
	events: libc::c_ushort,
	revents: libc::c_ushort,
}

/*
struct GSList
		(gpointer) data [void *]
		(GSList *) next [struct _GSList *]
*/
#[repr(C)]
pub struct GSList {
	data: *mut libc::c_void,
	next: *mut _GSList,
}

/*
struct GMainContext
*/
#[repr(C)]
pub struct GMainContext;

/*
struct GSourceFuncs
		(gboolean (*)(GSource *, gint *)) prepare [int (*)(struct _GSource *, int *)]
		(gboolean (*)(GSource *)) check [int (*)(struct _GSource *)]
		(gboolean (*)(GSource *, GSourceFunc, gpointer)) dispatch [int (*)(struct _GSource *, int (*)(void *), void *)]
		(void (*)(GSource *)) finalize [void (*)(struct _GSource *)]
		(GSourceFunc) closure_callback [int (*)(void *)]
		(GSourceDummyMarshal) closure_marshal [void (*)(void)]
*/
#[repr(C)]
pub struct GSourceFuncs {
	prepare: Option<extern fn(*mut _GSource, *mut libc::c_int) -> libc::c_int>,
	check: Option<extern fn(*mut _GSource) -> libc::c_int>,
	dispatch: Option<extern fn(*mut _GSource, Option<extern fn(*mut libc::c_void) -> libc::c_int>, *mut libc::c_void) -> libc::c_int>,
	finalize: Option<extern fn(*mut _GSource)>,
	closure_callback: Option<extern fn(*mut libc::c_void) -> libc::c_int>,
	closure_marshal: Option<extern fn()>,
}

/*
struct GMainLoop
*/
#[repr(C)]
pub struct GMainLoop;

/*
struct GSource
		(gpointer) callback_data [void *]
		(GSourceCallbackFuncs *) callback_funcs [struct _GSourceCallbackFuncs *]
		(GSourceFuncs *) source_funcs [struct _GSourceFuncs *]
		(guint) ref_count [unsigned int]
		(GMainContext *) context [struct _GMainContext *]
		(gint) priority [int]
		(guint) flags [unsigned int]
		(guint) source_id [unsigned int]
		(GSList *) poll_fds [struct _GSList *]
		(GSource *) prev [struct _GSource *]
		(GSource *) next [struct _GSource *]
		(char *) name
		(GSourcePrivate *) priv [struct _GSourcePrivate *]
*/
#[repr(C)]
pub struct GSource {
	callback_data: *mut libc::c_void,
	callback_funcs: *mut _GSourceCallbackFuncs,
	source_funcs: *mut _GSourceFuncs,
	ref_count: libc::c_uint,
	context: *mut _GMainContext,
	priority: libc::c_int,
	flags: libc::c_uint,
	source_id: libc::c_uint,
	poll_fds: *mut _GSList,
	prev: *mut _GSource,
	next: *mut _GSource,
	name: *mut libc::c_char,
	priv_: *mut _GSourcePrivate,
}

/*
struct GSourceCallbackFuncs
		(void (*)(gpointer)) ref [void (*)(void *)]
		(void (*)(gpointer)) unref [void (*)(void *)]
		(void (*)(gpointer, GSource *, GSourceFunc *, gpointer *)) get [void (*)(void *, struct _GSource *, int (**)(void *), void **)]
*/
#[repr(C)]
pub struct GSourceCallbackFuncs {
	ref_: Option<extern fn(*mut libc::c_void)>,
	unref: Option<extern fn(*mut libc::c_void)>,
	get: Option<extern fn(*mut libc::c_void, *mut _GSource, *mut Option<extern fn(*mut libc::c_void) -> libc::c_int>, *mut *mut libc::c_void)>,
}

/*
struct GDebugKey
		(const gchar *) key [const char *]
		(guint) value [unsigned int]
*/
#[repr(C)]
pub struct GDebugKey {
	key: *const libc::c_char,
	value: libc::c_uint,
}

/*
struct GString
		(gchar *) str [char *]
		(gsize) len [unsigned long]
		(gsize) allocated_len [unsigned long]
*/
#[repr(C)]
pub struct GString {
	str: *mut libc::c_char,
	len: libc::c_ulong,
	allocated_len: libc::c_ulong,
}

/*
struct GIOChannel
		(gint) ref_count [int]
		(GIOFuncs *) funcs [struct _GIOFuncs *]
		(gchar *) encoding [char *]
		(GIConv) read_cd [struct _GIConv *]
		(GIConv) write_cd [struct _GIConv *]
		(gchar *) line_term [char *]
		(guint) line_term_len [unsigned int]
		(gsize) buf_size [unsigned long]
		(GString *) read_buf [struct _GString *]
		(GString *) encoded_read_buf [struct _GString *]
		(GString *) write_buf [struct _GString *]
		(gchar [6]) partial_write_buf [char [6]]
		(guint) use_buffer [unsigned int]
		(guint) do_encode [unsigned int]
		(guint) close_on_unref [unsigned int]
		(guint) is_readable [unsigned int]
		(guint) is_writeable [unsigned int]
		(guint) is_seekable [unsigned int]
		(gpointer) reserved1 [void *]
		(gpointer) reserved2 [void *]
*/
#[repr(C)]
pub struct GIOChannel {
	ref_count: libc::c_int,
	funcs: *mut _GIOFuncs,
	encoding: *mut libc::c_char,
	read_cd: *mut _GIConv,
	write_cd: *mut _GIConv,
	line_term: *mut libc::c_char,
	line_term_len: libc::c_uint,
	buf_size: libc::c_ulong,
	read_buf: *mut _GString,
	encoded_read_buf: *mut _GString,
	write_buf: *mut _GString,
	partial_write_buf: [libc::c_char; 6],
	use_buffer: libc::c_uint,
	do_encode: libc::c_uint,
	close_on_unref: libc::c_uint,
	is_readable: libc::c_uint,
	is_writeable: libc::c_uint,
	is_seekable: libc::c_uint,
	reserved1: *mut libc::c_void,
	reserved2: *mut libc::c_void,
}

/*
struct GKeyFile
*/
#[repr(C)]
pub struct GKeyFile;

/*
struct GMappedFile
*/
#[repr(C)]
pub struct GMappedFile;

/*
struct GMarkupParser
		(void (*)(GMarkupParseContext *, const gchar *, const gchar **, const gchar **, gpointer, GError **)) start_element [void (*)(struct _GMarkupParseContext *, const char *, const char **, const char **, void *, struct _GError **)]
		(void (*)(GMarkupParseContext *, const gchar *, gpointer, GError **)) end_element [void (*)(struct _GMarkupParseContext *, const char *, void *, struct _GError **)]
		(void (*)(GMarkupParseContext *, const gchar *, gsize, gpointer, GError **)) text [void (*)(struct _GMarkupParseContext *, const char *, unsigned long, void *, struct _GError **)]
		(void (*)(GMarkupParseContext *, const gchar *, gsize, gpointer, GError **)) passthrough [void (*)(struct _GMarkupParseContext *, const char *, unsigned long, void *, struct _GError **)]
		(void (*)(GMarkupParseContext *, GError *, gpointer)) error [void (*)(struct _GMarkupParseContext *, struct _GError *, void *)]
*/
#[repr(C)]
pub struct GMarkupParser {
	start_element: Option<extern fn(*mut _GMarkupParseContext, *const libc::c_char, *mut *const libc::c_char, *mut *const libc::c_char, *mut libc::c_void, *mut *mut _GError)>,
	end_element: Option<extern fn(*mut _GMarkupParseContext, *const libc::c_char, *mut libc::c_void, *mut *mut _GError)>,
	text: Option<extern fn(*mut _GMarkupParseContext, *const libc::c_char, libc::c_ulong, *mut libc::c_void, *mut *mut _GError)>,
	passthrough: Option<extern fn(*mut _GMarkupParseContext, *const libc::c_char, libc::c_ulong, *mut libc::c_void, *mut *mut _GError)>,
	error: Option<extern fn(*mut _GMarkupParseContext, *mut _GError, *mut libc::c_void)>,
}

/*
struct GMarkupParseContext
*/
#[repr(C)]
pub struct GMarkupParseContext;

/*
struct GNode
		(gpointer) data [void *]
		(GNode *) next [struct _GNode *]
		(GNode *) prev [struct _GNode *]
		(GNode *) parent [struct _GNode *]
		(GNode *) children [struct _GNode *]
*/
#[repr(C)]
pub struct GNode {
	data: *mut libc::c_void,
	next: *mut _GNode,
	prev: *mut _GNode,
	parent: *mut _GNode,
	children: *mut _GNode,
}

/*
struct GOptionContext
*/
#[repr(C)]
pub struct GOptionContext;

/*
struct GOptionEntry
		(const gchar *) long_name [const char *]
		(gchar) short_name [char]
		(gint) flags [int]
		(GOptionArg) arg [GOptionArg]
		(gpointer) arg_data [void *]
		(const gchar *) description [const char *]
		(const gchar *) arg_description [const char *]
*/
#[repr(C)]
pub struct GOptionEntry {
	long_name: *const libc::c_char,
	short_name: libc::c_char,
	flags: libc::c_int,
	arg: libc::c_uint,
	arg_data: *mut libc::c_void,
	description: *const libc::c_char,
	arg_description: *const libc::c_char,
}

/*
struct GOptionGroup
*/
#[repr(C)]
pub struct GOptionGroup;

/*
struct GPatternSpec
*/
#[repr(C)]
pub struct GPatternSpec;

/*
struct GQueue
		(GList *) head [struct _GList *]
		(GList *) tail [struct _GList *]
		(guint) length [unsigned int]
*/
#[repr(C)]
pub struct GQueue {
	head: *mut _GList,
	tail: *mut _GList,
	length: libc::c_uint,
}

/*
struct GRand
*/
#[repr(C)]
pub struct GRand;

/*
struct GRegex
*/
#[repr(C)]
pub struct GRegex;

/*
struct GMatchInfo
*/
#[repr(C)]
pub struct GMatchInfo;

/*
struct GScannerConfig
		(gchar *) cset_skip_characters [char *]
		(gchar *) cset_identifier_first [char *]
		(gchar *) cset_identifier_nth [char *]
		(gchar *) cpair_comment_single [char *]
		(guint) case_sensitive [unsigned int]
		(guint) skip_comment_multi [unsigned int]
		(guint) skip_comment_single [unsigned int]
		(guint) scan_comment_multi [unsigned int]
		(guint) scan_identifier [unsigned int]
		(guint) scan_identifier_1char [unsigned int]
		(guint) scan_identifier_NULL [unsigned int]
		(guint) scan_symbols [unsigned int]
		(guint) scan_binary [unsigned int]
		(guint) scan_octal [unsigned int]
		(guint) scan_float [unsigned int]
		(guint) scan_hex [unsigned int]
		(guint) scan_hex_dollar [unsigned int]
		(guint) scan_string_sq [unsigned int]
		(guint) scan_string_dq [unsigned int]
		(guint) numbers_2_int [unsigned int]
		(guint) int_2_float [unsigned int]
		(guint) identifier_2_string [unsigned int]
		(guint) char_2_token [unsigned int]
		(guint) symbol_2_token [unsigned int]
		(guint) scope_0_fallback [unsigned int]
		(guint) store_int64 [unsigned int]
		(guint) padding_dummy [unsigned int]
*/
#[repr(C)]
pub struct GScannerConfig {
	cset_skip_characters: *mut libc::c_char,
	cset_identifier_first: *mut libc::c_char,
	cset_identifier_nth: *mut libc::c_char,
	cpair_comment_single: *mut libc::c_char,
	case_sensitive: libc::c_uint,
	skip_comment_multi: libc::c_uint,
	skip_comment_single: libc::c_uint,
	scan_comment_multi: libc::c_uint,
	scan_identifier: libc::c_uint,
	scan_identifier_1char: libc::c_uint,
	scan_identifier_NULL: libc::c_uint,
	scan_symbols: libc::c_uint,
	scan_binary: libc::c_uint,
	scan_octal: libc::c_uint,
	scan_float: libc::c_uint,
	scan_hex: libc::c_uint,
	scan_hex_dollar: libc::c_uint,
	scan_string_sq: libc::c_uint,
	scan_string_dq: libc::c_uint,
	numbers_2_int: libc::c_uint,
	int_2_float: libc::c_uint,
	identifier_2_string: libc::c_uint,
	char_2_token: libc::c_uint,
	symbol_2_token: libc::c_uint,
	scope_0_fallback: libc::c_uint,
	store_int64: libc::c_uint,
	padding_dummy: libc::c_uint,
}

/*
struct GScanner
		(gpointer) user_data [void *]
		(guint) max_parse_errors [unsigned int]
		(guint) parse_errors [unsigned int]
		(const gchar *) input_name [const char *]
		(GData *) qdata [struct _GData *]
		(GScannerConfig *) config [struct _GScannerConfig *]
		(GTokenType) token [GTokenType]
		(GTokenValue) value [union _GTokenValue]
		(guint) line [unsigned int]
		(guint) position [unsigned int]
		(GTokenType) next_token [GTokenType]
		(GTokenValue) next_value [union _GTokenValue]
		(guint) next_line [unsigned int]
		(guint) next_position [unsigned int]
		(GHashTable *) symbol_table [struct _GHashTable *]
		(gint) input_fd [int]
		(const gchar *) text [const char *]
		(const gchar *) text_end [const char *]
		(gchar *) buffer [char *]
		(guint) scope_id [unsigned int]
		(GScannerMsgFunc) msg_handler [void (*)(struct _GScanner *, char *, int)]
*/
#[repr(C)]
pub struct GScanner {
	user_data: *mut libc::c_void,
	max_parse_errors: libc::c_uint,
	parse_errors: libc::c_uint,
	input_name: *const libc::c_char,
	qdata: *mut _GData,
	config: *mut _GScannerConfig,
	token: libc::c_uint,
	value: union _GTokenValue,
	line: libc::c_uint,
	position: libc::c_uint,
	next_token: libc::c_uint,
	next_value: union _GTokenValue,
	next_line: libc::c_uint,
	next_position: libc::c_uint,
	symbol_table: *mut _GHashTable,
	input_fd: libc::c_int,
	text: *const libc::c_char,
	text_end: *const libc::c_char,
	buffer: *mut libc::c_char,
	scope_id: libc::c_uint,
	msg_handler: Option<extern fn(*mut _GScanner, *mut libc::c_char, libc::c_int)>,
}

/*
struct GSequence
*/
#[repr(C)]
pub struct GSequence;

/*
struct GSequenceIter
*/
#[repr(C)]
pub struct GSequenceIter;

/*
struct GStringChunk
*/
#[repr(C)]
pub struct GStringChunk;

/*
struct GTestSuite
*/
#[repr(C)]
pub struct GTestSuite;

/*
struct GTestCase
*/
#[repr(C)]
pub struct GTestCase;

/*
struct GTestLogBuffer
		(GString *) data [struct _GString *]
		(GSList *) msgs [struct _GSList *]
*/
#[repr(C)]
pub struct GTestLogBuffer {
	data: *mut _GString,
	msgs: *mut _GSList,
}

/*
struct GTestLogMsg
		(GTestLogType) log_type [GTestLogType]
		(guint) n_strings [unsigned int]
		(gchar **) strings [char **]
		(guint) n_nums [unsigned int]
		(long double *) nums
*/
#[repr(C)]
pub struct GTestLogMsg {
	log_type: libc::c_uint,
	n_strings: libc::c_uint,
	strings: *mut *mut libc::c_char,
	n_nums: libc::c_uint,
	nums: *mut TypeKind.LONGDOUBLE,
}

/*
struct GThreadPool
		(GFunc) func [void (*)(void *, void *)]
		(gpointer) user_data [void *]
		(gboolean) exclusive [int]
*/
#[repr(C)]
pub struct GThreadPool {
	func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>,
	user_data: *mut libc::c_void,
	exclusive: libc::c_int,
}

/*
struct GTimer
*/
#[repr(C)]
pub struct GTimer;

/*
struct GTree
*/
#[repr(C)]
pub struct GTree;

/*
struct GVariantType
*/
#[repr(C)]
pub struct GVariantType;

/*
struct GVariant
*/
#[repr(C)]
pub struct GVariant;

/*
struct GVariantIter
		(gsize [16]) x [unsigned long [16]]
*/
#[repr(C)]
pub struct GVariantIter {
	x: [libc::c_ulong; 16],
}

/*
struct GVariantBuilder
		(gsize [16]) x [unsigned long [16]]
*/
#[repr(C)]
pub struct GVariantBuilder {
	x: [libc::c_ulong; 16],
}

/*
struct GMemChunk
*/
#[repr(C)]
pub struct GMemChunk;

/*
struct GAllocator
*/
#[repr(C)]
pub struct GAllocator;

/*
struct GCache
*/
#[repr(C)]
pub struct GCache;

/*
struct GCompletion
		(GList *) items [struct _GList *]
		(GCompletionFunc) func [char *(*)(void *)]
		(gchar *) prefix [char *]
		(GList *) cache [struct _GList *]
		(GCompletionStrncmpFunc) strncmp_func [int (*)(const char *, const char *, unsigned long)]
*/
#[repr(C)]
pub struct GCompletion {
	items: *mut _GList,
	func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_char>,
	prefix: *mut libc::c_char,
	cache: *mut _GList,
	strncmp_func: Option<extern fn(*const libc::c_char, *const libc::c_char, libc::c_ulong) -> libc::c_int>,
}

/*
struct GRelation
*/
#[repr(C)]
pub struct GRelation;

/*
struct GTuples
		(guint) len [unsigned int]
*/
#[repr(C)]
pub struct GTuples {
	len: libc::c_uint,
}

/*
struct GStaticMutex
		(GMutex *) mutex [union _GMutex *]
		(pthread_mutex_t) unused [pthread_mutex_t]
*/
#[repr(C)]
pub struct GStaticMutex {
	mutex: *mut union _GMutex,
	unused: pthread_mutex_t,
}

/*
struct GStaticRecMutex
		(GStaticMutex) mutex [GStaticMutex]
		(guint) depth [unsigned int]
		(union _GStaticRecMutex::(anonymous at /usr/include/glib-2.0/glib/deprecated/gthread.h:158:3)) 
		(union (anonymous union at /usr/include/glib-2.0/glib/deprecated/gthread.h:158:3)) unused [union _GStaticRecMutex::(anonymous at /usr/include/glib-2.0/glib/deprecated/gthread.h:158:3)]
*/
#[repr(C)]
pub struct GStaticRecMutex {
	mutex: GStaticMutex,
	depth: libc::c_uint,
	: union _GStaticRecMutex::(anonymous at /usr/include/glib-2.0/glib/deprecated/gthread.h:158:3),
	unused: union _GStaticRecMutex::(anonymous at /usr/include/glib-2.0/glib/deprecated/gthread.h:158:3),
}

/*
struct GStaticRWLock
		(GStaticMutex) mutex [GStaticMutex]
		(GCond *) read_cond [struct _GCond *]
		(GCond *) write_cond [struct _GCond *]
		(guint) read_counter [unsigned int]
		(gboolean) have_writer [int]
		(guint) want_to_read [unsigned int]
		(guint) want_to_write [unsigned int]
*/
#[repr(C)]
pub struct GStaticRWLock {
	mutex: GStaticMutex,
	read_cond: *mut _GCond,
	write_cond: *mut _GCond,
	read_counter: libc::c_uint,
	have_writer: libc::c_int,
	want_to_read: libc::c_uint,
	want_to_write: libc::c_uint,
}

/*
struct GStaticPrivate
		(guint) index [unsigned int]
*/
#[repr(C)]
pub struct GStaticPrivate {
	index: libc::c_uint,
}

/*
enum  {
	G_THREAD_ERROR_AGAIN =	0x00000000 (0)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GThreadError {
	G_THREAD_ERROR_AGAIN =	0 as u32,
}

impl GThreadError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GThreadError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_THREAD_ERROR_AGAIN =	0x00000000 (0)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GThreadError {
	G_THREAD_ERROR_AGAIN =	0 as u32,
}

impl GThreadError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GThreadError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_ONCE_STATUS_NOTCALLED =	0x00000000 (0)
	G_ONCE_STATUS_PROGRESS =	0x00000001 (1)
	G_ONCE_STATUS_READY =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GOnceStatus {
	G_ONCE_STATUS_NOTCALLED =	0 as u32,
	G_ONCE_STATUS_PROGRESS =	1 as u32,
	G_ONCE_STATUS_READY =	2 as u32,
}

impl GOnceStatus {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GOnceStatus {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_ONCE_STATUS_NOTCALLED =	0x00000000 (0)
	G_ONCE_STATUS_PROGRESS =	0x00000001 (1)
	G_ONCE_STATUS_READY =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GOnceStatus {
	G_ONCE_STATUS_NOTCALLED =	0 as u32,
	G_ONCE_STATUS_PROGRESS =	1 as u32,
	G_ONCE_STATUS_READY =	2 as u32,
}

impl GOnceStatus {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GOnceStatus {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_BOOKMARK_FILE_ERROR_INVALID_URI =	0x00000000 (0)
	G_BOOKMARK_FILE_ERROR_INVALID_VALUE =	0x00000001 (1)
	G_BOOKMARK_FILE_ERROR_APP_NOT_REGISTERED =	0x00000002 (2)
	G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND =	0x00000003 (3)
	G_BOOKMARK_FILE_ERROR_READ =	0x00000004 (4)
	G_BOOKMARK_FILE_ERROR_UNKNOWN_ENCODING =	0x00000005 (5)
	G_BOOKMARK_FILE_ERROR_WRITE =	0x00000006 (6)
	G_BOOKMARK_FILE_ERROR_FILE_NOT_FOUND =	0x00000007 (7)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GBookmarkFileError {
	G_BOOKMARK_FILE_ERROR_INVALID_URI =	0 as u32,
	G_BOOKMARK_FILE_ERROR_INVALID_VALUE =	1 as u32,
	G_BOOKMARK_FILE_ERROR_APP_NOT_REGISTERED =	2 as u32,
	G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND =	3 as u32,
	G_BOOKMARK_FILE_ERROR_READ =	4 as u32,
	G_BOOKMARK_FILE_ERROR_UNKNOWN_ENCODING =	5 as u32,
	G_BOOKMARK_FILE_ERROR_WRITE =	6 as u32,
	G_BOOKMARK_FILE_ERROR_FILE_NOT_FOUND =	7 as u32,
}

impl GBookmarkFileError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GBookmarkFileError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_BOOKMARK_FILE_ERROR_INVALID_URI =	0x00000000 (0)
	G_BOOKMARK_FILE_ERROR_INVALID_VALUE =	0x00000001 (1)
	G_BOOKMARK_FILE_ERROR_APP_NOT_REGISTERED =	0x00000002 (2)
	G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND =	0x00000003 (3)
	G_BOOKMARK_FILE_ERROR_READ =	0x00000004 (4)
	G_BOOKMARK_FILE_ERROR_UNKNOWN_ENCODING =	0x00000005 (5)
	G_BOOKMARK_FILE_ERROR_WRITE =	0x00000006 (6)
	G_BOOKMARK_FILE_ERROR_FILE_NOT_FOUND =	0x00000007 (7)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GBookmarkFileError {
	G_BOOKMARK_FILE_ERROR_INVALID_URI =	0 as u32,
	G_BOOKMARK_FILE_ERROR_INVALID_VALUE =	1 as u32,
	G_BOOKMARK_FILE_ERROR_APP_NOT_REGISTERED =	2 as u32,
	G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND =	3 as u32,
	G_BOOKMARK_FILE_ERROR_READ =	4 as u32,
	G_BOOKMARK_FILE_ERROR_UNKNOWN_ENCODING =	5 as u32,
	G_BOOKMARK_FILE_ERROR_WRITE =	6 as u32,
	G_BOOKMARK_FILE_ERROR_FILE_NOT_FOUND =	7 as u32,
}

impl GBookmarkFileError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GBookmarkFileError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_CHECKSUM_MD5 =	0x00000000 (0)
	G_CHECKSUM_SHA1 =	0x00000001 (1)
	G_CHECKSUM_SHA256 =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GChecksumType {
	G_CHECKSUM_MD5 =	0 as u32,
	G_CHECKSUM_SHA1 =	1 as u32,
	G_CHECKSUM_SHA256 =	2 as u32,
}

impl GChecksumType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GChecksumType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_CHECKSUM_MD5 =	0x00000000 (0)
	G_CHECKSUM_SHA1 =	0x00000001 (1)
	G_CHECKSUM_SHA256 =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GChecksumType {
	G_CHECKSUM_MD5 =	0 as u32,
	G_CHECKSUM_SHA1 =	1 as u32,
	G_CHECKSUM_SHA256 =	2 as u32,
}

impl GChecksumType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GChecksumType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_CONVERT_ERROR_NO_CONVERSION =	0x00000000 (0)
	G_CONVERT_ERROR_ILLEGAL_SEQUENCE =	0x00000001 (1)
	G_CONVERT_ERROR_FAILED =	0x00000002 (2)
	G_CONVERT_ERROR_PARTIAL_INPUT =	0x00000003 (3)
	G_CONVERT_ERROR_BAD_URI =	0x00000004 (4)
	G_CONVERT_ERROR_NOT_ABSOLUTE_PATH =	0x00000005 (5)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GConvertError {
	G_CONVERT_ERROR_NO_CONVERSION =	0 as u32,
	G_CONVERT_ERROR_ILLEGAL_SEQUENCE =	1 as u32,
	G_CONVERT_ERROR_FAILED =	2 as u32,
	G_CONVERT_ERROR_PARTIAL_INPUT =	3 as u32,
	G_CONVERT_ERROR_BAD_URI =	4 as u32,
	G_CONVERT_ERROR_NOT_ABSOLUTE_PATH =	5 as u32,
}

impl GConvertError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GConvertError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_CONVERT_ERROR_NO_CONVERSION =	0x00000000 (0)
	G_CONVERT_ERROR_ILLEGAL_SEQUENCE =	0x00000001 (1)
	G_CONVERT_ERROR_FAILED =	0x00000002 (2)
	G_CONVERT_ERROR_PARTIAL_INPUT =	0x00000003 (3)
	G_CONVERT_ERROR_BAD_URI =	0x00000004 (4)
	G_CONVERT_ERROR_NOT_ABSOLUTE_PATH =	0x00000005 (5)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GConvertError {
	G_CONVERT_ERROR_NO_CONVERSION =	0 as u32,
	G_CONVERT_ERROR_ILLEGAL_SEQUENCE =	1 as u32,
	G_CONVERT_ERROR_FAILED =	2 as u32,
	G_CONVERT_ERROR_PARTIAL_INPUT =	3 as u32,
	G_CONVERT_ERROR_BAD_URI =	4 as u32,
	G_CONVERT_ERROR_NOT_ABSOLUTE_PATH =	5 as u32,
}

impl GConvertError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GConvertError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_DATE_DAY =	0x00000000 (0)
	G_DATE_MONTH =	0x00000001 (1)
	G_DATE_YEAR =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GDateDMY {
	G_DATE_DAY =	0 as u32,
	G_DATE_MONTH =	1 as u32,
	G_DATE_YEAR =	2 as u32,
}

impl GDateDMY {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GDateDMY {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_DATE_DAY =	0x00000000 (0)
	G_DATE_MONTH =	0x00000001 (1)
	G_DATE_YEAR =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GDateDMY {
	G_DATE_DAY =	0 as u32,
	G_DATE_MONTH =	1 as u32,
	G_DATE_YEAR =	2 as u32,
}

impl GDateDMY {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GDateDMY {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_DATE_BAD_WEEKDAY =	0x00000000 (0)
	G_DATE_MONDAY =	0x00000001 (1)
	G_DATE_TUESDAY =	0x00000002 (2)
	G_DATE_WEDNESDAY =	0x00000003 (3)
	G_DATE_THURSDAY =	0x00000004 (4)
	G_DATE_FRIDAY =	0x00000005 (5)
	G_DATE_SATURDAY =	0x00000006 (6)
	G_DATE_SUNDAY =	0x00000007 (7)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GDateWeekday {
	G_DATE_BAD_WEEKDAY =	0 as u32,
	G_DATE_MONDAY =	1 as u32,
	G_DATE_TUESDAY =	2 as u32,
	G_DATE_WEDNESDAY =	3 as u32,
	G_DATE_THURSDAY =	4 as u32,
	G_DATE_FRIDAY =	5 as u32,
	G_DATE_SATURDAY =	6 as u32,
	G_DATE_SUNDAY =	7 as u32,
}

impl GDateWeekday {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GDateWeekday {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_DATE_BAD_WEEKDAY =	0x00000000 (0)
	G_DATE_MONDAY =	0x00000001 (1)
	G_DATE_TUESDAY =	0x00000002 (2)
	G_DATE_WEDNESDAY =	0x00000003 (3)
	G_DATE_THURSDAY =	0x00000004 (4)
	G_DATE_FRIDAY =	0x00000005 (5)
	G_DATE_SATURDAY =	0x00000006 (6)
	G_DATE_SUNDAY =	0x00000007 (7)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GDateWeekday {
	G_DATE_BAD_WEEKDAY =	0 as u32,
	G_DATE_MONDAY =	1 as u32,
	G_DATE_TUESDAY =	2 as u32,
	G_DATE_WEDNESDAY =	3 as u32,
	G_DATE_THURSDAY =	4 as u32,
	G_DATE_FRIDAY =	5 as u32,
	G_DATE_SATURDAY =	6 as u32,
	G_DATE_SUNDAY =	7 as u32,
}

impl GDateWeekday {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GDateWeekday {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_DATE_BAD_MONTH =	0x00000000 (0)
	G_DATE_JANUARY =	0x00000001 (1)
	G_DATE_FEBRUARY =	0x00000002 (2)
	G_DATE_MARCH =	0x00000003 (3)
	G_DATE_APRIL =	0x00000004 (4)
	G_DATE_MAY =	0x00000005 (5)
	G_DATE_JUNE =	0x00000006 (6)
	G_DATE_JULY =	0x00000007 (7)
	G_DATE_AUGUST =	0x00000008 (8)
	G_DATE_SEPTEMBER =	0x00000009 (9)
	G_DATE_OCTOBER =	0x0000000A (10)
	G_DATE_NOVEMBER =	0x0000000B (11)
	G_DATE_DECEMBER =	0x0000000C (12)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GDateMonth {
	G_DATE_BAD_MONTH =	0 as u32,
	G_DATE_JANUARY =	1 as u32,
	G_DATE_FEBRUARY =	2 as u32,
	G_DATE_MARCH =	3 as u32,
	G_DATE_APRIL =	4 as u32,
	G_DATE_MAY =	5 as u32,
	G_DATE_JUNE =	6 as u32,
	G_DATE_JULY =	7 as u32,
	G_DATE_AUGUST =	8 as u32,
	G_DATE_SEPTEMBER =	9 as u32,
	G_DATE_OCTOBER =	10 as u32,
	G_DATE_NOVEMBER =	11 as u32,
	G_DATE_DECEMBER =	12 as u32,
}

impl GDateMonth {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GDateMonth {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_DATE_BAD_MONTH =	0x00000000 (0)
	G_DATE_JANUARY =	0x00000001 (1)
	G_DATE_FEBRUARY =	0x00000002 (2)
	G_DATE_MARCH =	0x00000003 (3)
	G_DATE_APRIL =	0x00000004 (4)
	G_DATE_MAY =	0x00000005 (5)
	G_DATE_JUNE =	0x00000006 (6)
	G_DATE_JULY =	0x00000007 (7)
	G_DATE_AUGUST =	0x00000008 (8)
	G_DATE_SEPTEMBER =	0x00000009 (9)
	G_DATE_OCTOBER =	0x0000000A (10)
	G_DATE_NOVEMBER =	0x0000000B (11)
	G_DATE_DECEMBER =	0x0000000C (12)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GDateMonth {
	G_DATE_BAD_MONTH =	0 as u32,
	G_DATE_JANUARY =	1 as u32,
	G_DATE_FEBRUARY =	2 as u32,
	G_DATE_MARCH =	3 as u32,
	G_DATE_APRIL =	4 as u32,
	G_DATE_MAY =	5 as u32,
	G_DATE_JUNE =	6 as u32,
	G_DATE_JULY =	7 as u32,
	G_DATE_AUGUST =	8 as u32,
	G_DATE_SEPTEMBER =	9 as u32,
	G_DATE_OCTOBER =	10 as u32,
	G_DATE_NOVEMBER =	11 as u32,
	G_DATE_DECEMBER =	12 as u32,
}

impl GDateMonth {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GDateMonth {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_TIME_TYPE_STANDARD =	0x00000000 (0)
	G_TIME_TYPE_DAYLIGHT =	0x00000001 (1)
	G_TIME_TYPE_UNIVERSAL =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GTimeType {
	G_TIME_TYPE_STANDARD =	0 as u32,
	G_TIME_TYPE_DAYLIGHT =	1 as u32,
	G_TIME_TYPE_UNIVERSAL =	2 as u32,
}

impl GTimeType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GTimeType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_TIME_TYPE_STANDARD =	0x00000000 (0)
	G_TIME_TYPE_DAYLIGHT =	0x00000001 (1)
	G_TIME_TYPE_UNIVERSAL =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GTimeType {
	G_TIME_TYPE_STANDARD =	0 as u32,
	G_TIME_TYPE_DAYLIGHT =	1 as u32,
	G_TIME_TYPE_UNIVERSAL =	2 as u32,
}

impl GTimeType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GTimeType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_FILE_ERROR_EXIST =	0x00000000 (0)
	G_FILE_ERROR_ISDIR =	0x00000001 (1)
	G_FILE_ERROR_ACCES =	0x00000002 (2)
	G_FILE_ERROR_NAMETOOLONG =	0x00000003 (3)
	G_FILE_ERROR_NOENT =	0x00000004 (4)
	G_FILE_ERROR_NOTDIR =	0x00000005 (5)
	G_FILE_ERROR_NXIO =	0x00000006 (6)
	G_FILE_ERROR_NODEV =	0x00000007 (7)
	G_FILE_ERROR_ROFS =	0x00000008 (8)
	G_FILE_ERROR_TXTBSY =	0x00000009 (9)
	G_FILE_ERROR_FAULT =	0x0000000A (10)
	G_FILE_ERROR_LOOP =	0x0000000B (11)
	G_FILE_ERROR_NOSPC =	0x0000000C (12)
	G_FILE_ERROR_NOMEM =	0x0000000D (13)
	G_FILE_ERROR_MFILE =	0x0000000E (14)
	G_FILE_ERROR_NFILE =	0x0000000F (15)
	G_FILE_ERROR_BADF =	0x00000010 (16)
	G_FILE_ERROR_INVAL =	0x00000011 (17)
	G_FILE_ERROR_PIPE =	0x00000012 (18)
	G_FILE_ERROR_AGAIN =	0x00000013 (19)
	G_FILE_ERROR_INTR =	0x00000014 (20)
	G_FILE_ERROR_IO =	0x00000015 (21)
	G_FILE_ERROR_PERM =	0x00000016 (22)
	G_FILE_ERROR_NOSYS =	0x00000017 (23)
	G_FILE_ERROR_FAILED =	0x00000018 (24)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GFileError {
	G_FILE_ERROR_EXIST =	0 as u32,
	G_FILE_ERROR_ISDIR =	1 as u32,
	G_FILE_ERROR_ACCES =	2 as u32,
	G_FILE_ERROR_NAMETOOLONG =	3 as u32,
	G_FILE_ERROR_NOENT =	4 as u32,
	G_FILE_ERROR_NOTDIR =	5 as u32,
	G_FILE_ERROR_NXIO =	6 as u32,
	G_FILE_ERROR_NODEV =	7 as u32,
	G_FILE_ERROR_ROFS =	8 as u32,
	G_FILE_ERROR_TXTBSY =	9 as u32,
	G_FILE_ERROR_FAULT =	10 as u32,
	G_FILE_ERROR_LOOP =	11 as u32,
	G_FILE_ERROR_NOSPC =	12 as u32,
	G_FILE_ERROR_NOMEM =	13 as u32,
	G_FILE_ERROR_MFILE =	14 as u32,
	G_FILE_ERROR_NFILE =	15 as u32,
	G_FILE_ERROR_BADF =	16 as u32,
	G_FILE_ERROR_INVAL =	17 as u32,
	G_FILE_ERROR_PIPE =	18 as u32,
	G_FILE_ERROR_AGAIN =	19 as u32,
	G_FILE_ERROR_INTR =	20 as u32,
	G_FILE_ERROR_IO =	21 as u32,
	G_FILE_ERROR_PERM =	22 as u32,
	G_FILE_ERROR_NOSYS =	23 as u32,
	G_FILE_ERROR_FAILED =	24 as u32,
}

impl GFileError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GFileError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_FILE_ERROR_EXIST =	0x00000000 (0)
	G_FILE_ERROR_ISDIR =	0x00000001 (1)
	G_FILE_ERROR_ACCES =	0x00000002 (2)
	G_FILE_ERROR_NAMETOOLONG =	0x00000003 (3)
	G_FILE_ERROR_NOENT =	0x00000004 (4)
	G_FILE_ERROR_NOTDIR =	0x00000005 (5)
	G_FILE_ERROR_NXIO =	0x00000006 (6)
	G_FILE_ERROR_NODEV =	0x00000007 (7)
	G_FILE_ERROR_ROFS =	0x00000008 (8)
	G_FILE_ERROR_TXTBSY =	0x00000009 (9)
	G_FILE_ERROR_FAULT =	0x0000000A (10)
	G_FILE_ERROR_LOOP =	0x0000000B (11)
	G_FILE_ERROR_NOSPC =	0x0000000C (12)
	G_FILE_ERROR_NOMEM =	0x0000000D (13)
	G_FILE_ERROR_MFILE =	0x0000000E (14)
	G_FILE_ERROR_NFILE =	0x0000000F (15)
	G_FILE_ERROR_BADF =	0x00000010 (16)
	G_FILE_ERROR_INVAL =	0x00000011 (17)
	G_FILE_ERROR_PIPE =	0x00000012 (18)
	G_FILE_ERROR_AGAIN =	0x00000013 (19)
	G_FILE_ERROR_INTR =	0x00000014 (20)
	G_FILE_ERROR_IO =	0x00000015 (21)
	G_FILE_ERROR_PERM =	0x00000016 (22)
	G_FILE_ERROR_NOSYS =	0x00000017 (23)
	G_FILE_ERROR_FAILED =	0x00000018 (24)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GFileError {
	G_FILE_ERROR_EXIST =	0 as u32,
	G_FILE_ERROR_ISDIR =	1 as u32,
	G_FILE_ERROR_ACCES =	2 as u32,
	G_FILE_ERROR_NAMETOOLONG =	3 as u32,
	G_FILE_ERROR_NOENT =	4 as u32,
	G_FILE_ERROR_NOTDIR =	5 as u32,
	G_FILE_ERROR_NXIO =	6 as u32,
	G_FILE_ERROR_NODEV =	7 as u32,
	G_FILE_ERROR_ROFS =	8 as u32,
	G_FILE_ERROR_TXTBSY =	9 as u32,
	G_FILE_ERROR_FAULT =	10 as u32,
	G_FILE_ERROR_LOOP =	11 as u32,
	G_FILE_ERROR_NOSPC =	12 as u32,
	G_FILE_ERROR_NOMEM =	13 as u32,
	G_FILE_ERROR_MFILE =	14 as u32,
	G_FILE_ERROR_NFILE =	15 as u32,
	G_FILE_ERROR_BADF =	16 as u32,
	G_FILE_ERROR_INVAL =	17 as u32,
	G_FILE_ERROR_PIPE =	18 as u32,
	G_FILE_ERROR_AGAIN =	19 as u32,
	G_FILE_ERROR_INTR =	20 as u32,
	G_FILE_ERROR_IO =	21 as u32,
	G_FILE_ERROR_PERM =	22 as u32,
	G_FILE_ERROR_NOSYS =	23 as u32,
	G_FILE_ERROR_FAILED =	24 as u32,
}

impl GFileError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GFileError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_FILE_TEST_IS_REGULAR =	0x00000001 (1)
	G_FILE_TEST_IS_SYMLINK =	0x00000002 (2)
	G_FILE_TEST_IS_DIR =	0x00000004 (4)
	G_FILE_TEST_IS_EXECUTABLE =	0x00000008 (8)
	G_FILE_TEST_EXISTS =	0x00000010 (16)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GFileTest {
	G_FILE_TEST_IS_REGULAR =	1 as u32,
	G_FILE_TEST_IS_SYMLINK =	2 as u32,
	G_FILE_TEST_IS_DIR =	4 as u32,
	G_FILE_TEST_IS_EXECUTABLE =	8 as u32,
	G_FILE_TEST_EXISTS =	16 as u32,
}

impl GFileTest {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GFileTest {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_FILE_TEST_IS_REGULAR =	0x00000001 (1)
	G_FILE_TEST_IS_SYMLINK =	0x00000002 (2)
	G_FILE_TEST_IS_DIR =	0x00000004 (4)
	G_FILE_TEST_IS_EXECUTABLE =	0x00000008 (8)
	G_FILE_TEST_EXISTS =	0x00000010 (16)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GFileTest {
	G_FILE_TEST_IS_REGULAR =	1 as u32,
	G_FILE_TEST_IS_SYMLINK =	2 as u32,
	G_FILE_TEST_IS_DIR =	4 as u32,
	G_FILE_TEST_IS_EXECUTABLE =	8 as u32,
	G_FILE_TEST_EXISTS =	16 as u32,
}

impl GFileTest {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GFileTest {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_HOOK_FLAG_ACTIVE =	0x00000001 (1)
	G_HOOK_FLAG_IN_CALL =	0x00000002 (2)
	G_HOOK_FLAG_MASK =	0x0000000F (15)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GHookFlagMask {
	G_HOOK_FLAG_ACTIVE =	1 as u32,
	G_HOOK_FLAG_IN_CALL =	2 as u32,
	G_HOOK_FLAG_MASK =	15 as u32,
}

impl GHookFlagMask {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GHookFlagMask {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_HOOK_FLAG_ACTIVE =	0x00000001 (1)
	G_HOOK_FLAG_IN_CALL =	0x00000002 (2)
	G_HOOK_FLAG_MASK =	0x0000000F (15)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GHookFlagMask {
	G_HOOK_FLAG_ACTIVE =	1 as u32,
	G_HOOK_FLAG_IN_CALL =	2 as u32,
	G_HOOK_FLAG_MASK =	15 as u32,
}

impl GHookFlagMask {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GHookFlagMask {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_UNICODE_CONTROL =	0x00000000 (0)
	G_UNICODE_FORMAT =	0x00000001 (1)
	G_UNICODE_UNASSIGNED =	0x00000002 (2)
	G_UNICODE_PRIVATE_USE =	0x00000003 (3)
	G_UNICODE_SURROGATE =	0x00000004 (4)
	G_UNICODE_LOWERCASE_LETTER =	0x00000005 (5)
	G_UNICODE_MODIFIER_LETTER =	0x00000006 (6)
	G_UNICODE_OTHER_LETTER =	0x00000007 (7)
	G_UNICODE_TITLECASE_LETTER =	0x00000008 (8)
	G_UNICODE_UPPERCASE_LETTER =	0x00000009 (9)
	G_UNICODE_SPACING_MARK =	0x0000000A (10)
	G_UNICODE_ENCLOSING_MARK =	0x0000000B (11)
	G_UNICODE_NON_SPACING_MARK =	0x0000000C (12)
	G_UNICODE_DECIMAL_NUMBER =	0x0000000D (13)
	G_UNICODE_LETTER_NUMBER =	0x0000000E (14)
	G_UNICODE_OTHER_NUMBER =	0x0000000F (15)
	G_UNICODE_CONNECT_PUNCTUATION =	0x00000010 (16)
	G_UNICODE_DASH_PUNCTUATION =	0x00000011 (17)
	G_UNICODE_CLOSE_PUNCTUATION =	0x00000012 (18)
	G_UNICODE_FINAL_PUNCTUATION =	0x00000013 (19)
	G_UNICODE_INITIAL_PUNCTUATION =	0x00000014 (20)
	G_UNICODE_OTHER_PUNCTUATION =	0x00000015 (21)
	G_UNICODE_OPEN_PUNCTUATION =	0x00000016 (22)
	G_UNICODE_CURRENCY_SYMBOL =	0x00000017 (23)
	G_UNICODE_MODIFIER_SYMBOL =	0x00000018 (24)
	G_UNICODE_MATH_SYMBOL =	0x00000019 (25)
	G_UNICODE_OTHER_SYMBOL =	0x0000001A (26)
	G_UNICODE_LINE_SEPARATOR =	0x0000001B (27)
	G_UNICODE_PARAGRAPH_SEPARATOR =	0x0000001C (28)
	G_UNICODE_SPACE_SEPARATOR =	0x0000001D (29)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GUnicodeType {
	G_UNICODE_CONTROL =	0 as u32,
	G_UNICODE_FORMAT =	1 as u32,
	G_UNICODE_UNASSIGNED =	2 as u32,
	G_UNICODE_PRIVATE_USE =	3 as u32,
	G_UNICODE_SURROGATE =	4 as u32,
	G_UNICODE_LOWERCASE_LETTER =	5 as u32,
	G_UNICODE_MODIFIER_LETTER =	6 as u32,
	G_UNICODE_OTHER_LETTER =	7 as u32,
	G_UNICODE_TITLECASE_LETTER =	8 as u32,
	G_UNICODE_UPPERCASE_LETTER =	9 as u32,
	G_UNICODE_SPACING_MARK =	10 as u32,
	G_UNICODE_ENCLOSING_MARK =	11 as u32,
	G_UNICODE_NON_SPACING_MARK =	12 as u32,
	G_UNICODE_DECIMAL_NUMBER =	13 as u32,
	G_UNICODE_LETTER_NUMBER =	14 as u32,
	G_UNICODE_OTHER_NUMBER =	15 as u32,
	G_UNICODE_CONNECT_PUNCTUATION =	16 as u32,
	G_UNICODE_DASH_PUNCTUATION =	17 as u32,
	G_UNICODE_CLOSE_PUNCTUATION =	18 as u32,
	G_UNICODE_FINAL_PUNCTUATION =	19 as u32,
	G_UNICODE_INITIAL_PUNCTUATION =	20 as u32,
	G_UNICODE_OTHER_PUNCTUATION =	21 as u32,
	G_UNICODE_OPEN_PUNCTUATION =	22 as u32,
	G_UNICODE_CURRENCY_SYMBOL =	23 as u32,
	G_UNICODE_MODIFIER_SYMBOL =	24 as u32,
	G_UNICODE_MATH_SYMBOL =	25 as u32,
	G_UNICODE_OTHER_SYMBOL =	26 as u32,
	G_UNICODE_LINE_SEPARATOR =	27 as u32,
	G_UNICODE_PARAGRAPH_SEPARATOR =	28 as u32,
	G_UNICODE_SPACE_SEPARATOR =	29 as u32,
}

impl GUnicodeType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GUnicodeType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_UNICODE_CONTROL =	0x00000000 (0)
	G_UNICODE_FORMAT =	0x00000001 (1)
	G_UNICODE_UNASSIGNED =	0x00000002 (2)
	G_UNICODE_PRIVATE_USE =	0x00000003 (3)
	G_UNICODE_SURROGATE =	0x00000004 (4)
	G_UNICODE_LOWERCASE_LETTER =	0x00000005 (5)
	G_UNICODE_MODIFIER_LETTER =	0x00000006 (6)
	G_UNICODE_OTHER_LETTER =	0x00000007 (7)
	G_UNICODE_TITLECASE_LETTER =	0x00000008 (8)
	G_UNICODE_UPPERCASE_LETTER =	0x00000009 (9)
	G_UNICODE_SPACING_MARK =	0x0000000A (10)
	G_UNICODE_ENCLOSING_MARK =	0x0000000B (11)
	G_UNICODE_NON_SPACING_MARK =	0x0000000C (12)
	G_UNICODE_DECIMAL_NUMBER =	0x0000000D (13)
	G_UNICODE_LETTER_NUMBER =	0x0000000E (14)
	G_UNICODE_OTHER_NUMBER =	0x0000000F (15)
	G_UNICODE_CONNECT_PUNCTUATION =	0x00000010 (16)
	G_UNICODE_DASH_PUNCTUATION =	0x00000011 (17)
	G_UNICODE_CLOSE_PUNCTUATION =	0x00000012 (18)
	G_UNICODE_FINAL_PUNCTUATION =	0x00000013 (19)
	G_UNICODE_INITIAL_PUNCTUATION =	0x00000014 (20)
	G_UNICODE_OTHER_PUNCTUATION =	0x00000015 (21)
	G_UNICODE_OPEN_PUNCTUATION =	0x00000016 (22)
	G_UNICODE_CURRENCY_SYMBOL =	0x00000017 (23)
	G_UNICODE_MODIFIER_SYMBOL =	0x00000018 (24)
	G_UNICODE_MATH_SYMBOL =	0x00000019 (25)
	G_UNICODE_OTHER_SYMBOL =	0x0000001A (26)
	G_UNICODE_LINE_SEPARATOR =	0x0000001B (27)
	G_UNICODE_PARAGRAPH_SEPARATOR =	0x0000001C (28)
	G_UNICODE_SPACE_SEPARATOR =	0x0000001D (29)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GUnicodeType {
	G_UNICODE_CONTROL =	0 as u32,
	G_UNICODE_FORMAT =	1 as u32,
	G_UNICODE_UNASSIGNED =	2 as u32,
	G_UNICODE_PRIVATE_USE =	3 as u32,
	G_UNICODE_SURROGATE =	4 as u32,
	G_UNICODE_LOWERCASE_LETTER =	5 as u32,
	G_UNICODE_MODIFIER_LETTER =	6 as u32,
	G_UNICODE_OTHER_LETTER =	7 as u32,
	G_UNICODE_TITLECASE_LETTER =	8 as u32,
	G_UNICODE_UPPERCASE_LETTER =	9 as u32,
	G_UNICODE_SPACING_MARK =	10 as u32,
	G_UNICODE_ENCLOSING_MARK =	11 as u32,
	G_UNICODE_NON_SPACING_MARK =	12 as u32,
	G_UNICODE_DECIMAL_NUMBER =	13 as u32,
	G_UNICODE_LETTER_NUMBER =	14 as u32,
	G_UNICODE_OTHER_NUMBER =	15 as u32,
	G_UNICODE_CONNECT_PUNCTUATION =	16 as u32,
	G_UNICODE_DASH_PUNCTUATION =	17 as u32,
	G_UNICODE_CLOSE_PUNCTUATION =	18 as u32,
	G_UNICODE_FINAL_PUNCTUATION =	19 as u32,
	G_UNICODE_INITIAL_PUNCTUATION =	20 as u32,
	G_UNICODE_OTHER_PUNCTUATION =	21 as u32,
	G_UNICODE_OPEN_PUNCTUATION =	22 as u32,
	G_UNICODE_CURRENCY_SYMBOL =	23 as u32,
	G_UNICODE_MODIFIER_SYMBOL =	24 as u32,
	G_UNICODE_MATH_SYMBOL =	25 as u32,
	G_UNICODE_OTHER_SYMBOL =	26 as u32,
	G_UNICODE_LINE_SEPARATOR =	27 as u32,
	G_UNICODE_PARAGRAPH_SEPARATOR =	28 as u32,
	G_UNICODE_SPACE_SEPARATOR =	29 as u32,
}

impl GUnicodeType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GUnicodeType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_UNICODE_BREAK_MANDATORY =	0x00000000 (0)
	G_UNICODE_BREAK_CARRIAGE_RETURN =	0x00000001 (1)
	G_UNICODE_BREAK_LINE_FEED =	0x00000002 (2)
	G_UNICODE_BREAK_COMBINING_MARK =	0x00000003 (3)
	G_UNICODE_BREAK_SURROGATE =	0x00000004 (4)
	G_UNICODE_BREAK_ZERO_WIDTH_SPACE =	0x00000005 (5)
	G_UNICODE_BREAK_INSEPARABLE =	0x00000006 (6)
	G_UNICODE_BREAK_NON_BREAKING_GLUE =	0x00000007 (7)
	G_UNICODE_BREAK_CONTINGENT =	0x00000008 (8)
	G_UNICODE_BREAK_SPACE =	0x00000009 (9)
	G_UNICODE_BREAK_AFTER =	0x0000000A (10)
	G_UNICODE_BREAK_BEFORE =	0x0000000B (11)
	G_UNICODE_BREAK_BEFORE_AND_AFTER =	0x0000000C (12)
	G_UNICODE_BREAK_HYPHEN =	0x0000000D (13)
	G_UNICODE_BREAK_NON_STARTER =	0x0000000E (14)
	G_UNICODE_BREAK_OPEN_PUNCTUATION =	0x0000000F (15)
	G_UNICODE_BREAK_CLOSE_PUNCTUATION =	0x00000010 (16)
	G_UNICODE_BREAK_QUOTATION =	0x00000011 (17)
	G_UNICODE_BREAK_EXCLAMATION =	0x00000012 (18)
	G_UNICODE_BREAK_IDEOGRAPHIC =	0x00000013 (19)
	G_UNICODE_BREAK_NUMERIC =	0x00000014 (20)
	G_UNICODE_BREAK_INFIX_SEPARATOR =	0x00000015 (21)
	G_UNICODE_BREAK_SYMBOL =	0x00000016 (22)
	G_UNICODE_BREAK_ALPHABETIC =	0x00000017 (23)
	G_UNICODE_BREAK_PREFIX =	0x00000018 (24)
	G_UNICODE_BREAK_POSTFIX =	0x00000019 (25)
	G_UNICODE_BREAK_COMPLEX_CONTEXT =	0x0000001A (26)
	G_UNICODE_BREAK_AMBIGUOUS =	0x0000001B (27)
	G_UNICODE_BREAK_UNKNOWN =	0x0000001C (28)
	G_UNICODE_BREAK_NEXT_LINE =	0x0000001D (29)
	G_UNICODE_BREAK_WORD_JOINER =	0x0000001E (30)
	G_UNICODE_BREAK_HANGUL_L_JAMO =	0x0000001F (31)
	G_UNICODE_BREAK_HANGUL_V_JAMO =	0x00000020 (32)
	G_UNICODE_BREAK_HANGUL_T_JAMO =	0x00000021 (33)
	G_UNICODE_BREAK_HANGUL_LV_SYLLABLE =	0x00000022 (34)
	G_UNICODE_BREAK_HANGUL_LVT_SYLLABLE =	0x00000023 (35)
	G_UNICODE_BREAK_CLOSE_PARANTHESIS =	0x00000024 (36)
	G_UNICODE_BREAK_CONDITIONAL_JAPANESE_STARTER =	0x00000025 (37)
	G_UNICODE_BREAK_HEBREW_LETTER =	0x00000026 (38)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GUnicodeBreakType {
	G_UNICODE_BREAK_MANDATORY =	0 as u32,
	G_UNICODE_BREAK_CARRIAGE_RETURN =	1 as u32,
	G_UNICODE_BREAK_LINE_FEED =	2 as u32,
	G_UNICODE_BREAK_COMBINING_MARK =	3 as u32,
	G_UNICODE_BREAK_SURROGATE =	4 as u32,
	G_UNICODE_BREAK_ZERO_WIDTH_SPACE =	5 as u32,
	G_UNICODE_BREAK_INSEPARABLE =	6 as u32,
	G_UNICODE_BREAK_NON_BREAKING_GLUE =	7 as u32,
	G_UNICODE_BREAK_CONTINGENT =	8 as u32,
	G_UNICODE_BREAK_SPACE =	9 as u32,
	G_UNICODE_BREAK_AFTER =	10 as u32,
	G_UNICODE_BREAK_BEFORE =	11 as u32,
	G_UNICODE_BREAK_BEFORE_AND_AFTER =	12 as u32,
	G_UNICODE_BREAK_HYPHEN =	13 as u32,
	G_UNICODE_BREAK_NON_STARTER =	14 as u32,
	G_UNICODE_BREAK_OPEN_PUNCTUATION =	15 as u32,
	G_UNICODE_BREAK_CLOSE_PUNCTUATION =	16 as u32,
	G_UNICODE_BREAK_QUOTATION =	17 as u32,
	G_UNICODE_BREAK_EXCLAMATION =	18 as u32,
	G_UNICODE_BREAK_IDEOGRAPHIC =	19 as u32,
	G_UNICODE_BREAK_NUMERIC =	20 as u32,
	G_UNICODE_BREAK_INFIX_SEPARATOR =	21 as u32,
	G_UNICODE_BREAK_SYMBOL =	22 as u32,
	G_UNICODE_BREAK_ALPHABETIC =	23 as u32,
	G_UNICODE_BREAK_PREFIX =	24 as u32,
	G_UNICODE_BREAK_POSTFIX =	25 as u32,
	G_UNICODE_BREAK_COMPLEX_CONTEXT =	26 as u32,
	G_UNICODE_BREAK_AMBIGUOUS =	27 as u32,
	G_UNICODE_BREAK_UNKNOWN =	28 as u32,
	G_UNICODE_BREAK_NEXT_LINE =	29 as u32,
	G_UNICODE_BREAK_WORD_JOINER =	30 as u32,
	G_UNICODE_BREAK_HANGUL_L_JAMO =	31 as u32,
	G_UNICODE_BREAK_HANGUL_V_JAMO =	32 as u32,
	G_UNICODE_BREAK_HANGUL_T_JAMO =	33 as u32,
	G_UNICODE_BREAK_HANGUL_LV_SYLLABLE =	34 as u32,
	G_UNICODE_BREAK_HANGUL_LVT_SYLLABLE =	35 as u32,
	G_UNICODE_BREAK_CLOSE_PARANTHESIS =	36 as u32,
	G_UNICODE_BREAK_CONDITIONAL_JAPANESE_STARTER =	37 as u32,
	G_UNICODE_BREAK_HEBREW_LETTER =	38 as u32,
}

impl GUnicodeBreakType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GUnicodeBreakType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_UNICODE_BREAK_MANDATORY =	0x00000000 (0)
	G_UNICODE_BREAK_CARRIAGE_RETURN =	0x00000001 (1)
	G_UNICODE_BREAK_LINE_FEED =	0x00000002 (2)
	G_UNICODE_BREAK_COMBINING_MARK =	0x00000003 (3)
	G_UNICODE_BREAK_SURROGATE =	0x00000004 (4)
	G_UNICODE_BREAK_ZERO_WIDTH_SPACE =	0x00000005 (5)
	G_UNICODE_BREAK_INSEPARABLE =	0x00000006 (6)
	G_UNICODE_BREAK_NON_BREAKING_GLUE =	0x00000007 (7)
	G_UNICODE_BREAK_CONTINGENT =	0x00000008 (8)
	G_UNICODE_BREAK_SPACE =	0x00000009 (9)
	G_UNICODE_BREAK_AFTER =	0x0000000A (10)
	G_UNICODE_BREAK_BEFORE =	0x0000000B (11)
	G_UNICODE_BREAK_BEFORE_AND_AFTER =	0x0000000C (12)
	G_UNICODE_BREAK_HYPHEN =	0x0000000D (13)
	G_UNICODE_BREAK_NON_STARTER =	0x0000000E (14)
	G_UNICODE_BREAK_OPEN_PUNCTUATION =	0x0000000F (15)
	G_UNICODE_BREAK_CLOSE_PUNCTUATION =	0x00000010 (16)
	G_UNICODE_BREAK_QUOTATION =	0x00000011 (17)
	G_UNICODE_BREAK_EXCLAMATION =	0x00000012 (18)
	G_UNICODE_BREAK_IDEOGRAPHIC =	0x00000013 (19)
	G_UNICODE_BREAK_NUMERIC =	0x00000014 (20)
	G_UNICODE_BREAK_INFIX_SEPARATOR =	0x00000015 (21)
	G_UNICODE_BREAK_SYMBOL =	0x00000016 (22)
	G_UNICODE_BREAK_ALPHABETIC =	0x00000017 (23)
	G_UNICODE_BREAK_PREFIX =	0x00000018 (24)
	G_UNICODE_BREAK_POSTFIX =	0x00000019 (25)
	G_UNICODE_BREAK_COMPLEX_CONTEXT =	0x0000001A (26)
	G_UNICODE_BREAK_AMBIGUOUS =	0x0000001B (27)
	G_UNICODE_BREAK_UNKNOWN =	0x0000001C (28)
	G_UNICODE_BREAK_NEXT_LINE =	0x0000001D (29)
	G_UNICODE_BREAK_WORD_JOINER =	0x0000001E (30)
	G_UNICODE_BREAK_HANGUL_L_JAMO =	0x0000001F (31)
	G_UNICODE_BREAK_HANGUL_V_JAMO =	0x00000020 (32)
	G_UNICODE_BREAK_HANGUL_T_JAMO =	0x00000021 (33)
	G_UNICODE_BREAK_HANGUL_LV_SYLLABLE =	0x00000022 (34)
	G_UNICODE_BREAK_HANGUL_LVT_SYLLABLE =	0x00000023 (35)
	G_UNICODE_BREAK_CLOSE_PARANTHESIS =	0x00000024 (36)
	G_UNICODE_BREAK_CONDITIONAL_JAPANESE_STARTER =	0x00000025 (37)
	G_UNICODE_BREAK_HEBREW_LETTER =	0x00000026 (38)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GUnicodeBreakType {
	G_UNICODE_BREAK_MANDATORY =	0 as u32,
	G_UNICODE_BREAK_CARRIAGE_RETURN =	1 as u32,
	G_UNICODE_BREAK_LINE_FEED =	2 as u32,
	G_UNICODE_BREAK_COMBINING_MARK =	3 as u32,
	G_UNICODE_BREAK_SURROGATE =	4 as u32,
	G_UNICODE_BREAK_ZERO_WIDTH_SPACE =	5 as u32,
	G_UNICODE_BREAK_INSEPARABLE =	6 as u32,
	G_UNICODE_BREAK_NON_BREAKING_GLUE =	7 as u32,
	G_UNICODE_BREAK_CONTINGENT =	8 as u32,
	G_UNICODE_BREAK_SPACE =	9 as u32,
	G_UNICODE_BREAK_AFTER =	10 as u32,
	G_UNICODE_BREAK_BEFORE =	11 as u32,
	G_UNICODE_BREAK_BEFORE_AND_AFTER =	12 as u32,
	G_UNICODE_BREAK_HYPHEN =	13 as u32,
	G_UNICODE_BREAK_NON_STARTER =	14 as u32,
	G_UNICODE_BREAK_OPEN_PUNCTUATION =	15 as u32,
	G_UNICODE_BREAK_CLOSE_PUNCTUATION =	16 as u32,
	G_UNICODE_BREAK_QUOTATION =	17 as u32,
	G_UNICODE_BREAK_EXCLAMATION =	18 as u32,
	G_UNICODE_BREAK_IDEOGRAPHIC =	19 as u32,
	G_UNICODE_BREAK_NUMERIC =	20 as u32,
	G_UNICODE_BREAK_INFIX_SEPARATOR =	21 as u32,
	G_UNICODE_BREAK_SYMBOL =	22 as u32,
	G_UNICODE_BREAK_ALPHABETIC =	23 as u32,
	G_UNICODE_BREAK_PREFIX =	24 as u32,
	G_UNICODE_BREAK_POSTFIX =	25 as u32,
	G_UNICODE_BREAK_COMPLEX_CONTEXT =	26 as u32,
	G_UNICODE_BREAK_AMBIGUOUS =	27 as u32,
	G_UNICODE_BREAK_UNKNOWN =	28 as u32,
	G_UNICODE_BREAK_NEXT_LINE =	29 as u32,
	G_UNICODE_BREAK_WORD_JOINER =	30 as u32,
	G_UNICODE_BREAK_HANGUL_L_JAMO =	31 as u32,
	G_UNICODE_BREAK_HANGUL_V_JAMO =	32 as u32,
	G_UNICODE_BREAK_HANGUL_T_JAMO =	33 as u32,
	G_UNICODE_BREAK_HANGUL_LV_SYLLABLE =	34 as u32,
	G_UNICODE_BREAK_HANGUL_LVT_SYLLABLE =	35 as u32,
	G_UNICODE_BREAK_CLOSE_PARANTHESIS =	36 as u32,
	G_UNICODE_BREAK_CONDITIONAL_JAPANESE_STARTER =	37 as u32,
	G_UNICODE_BREAK_HEBREW_LETTER =	38 as u32,
}

impl GUnicodeBreakType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GUnicodeBreakType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_UNICODE_SCRIPT_INVALID_CODE =	0x-0000001 (-1)
	G_UNICODE_SCRIPT_COMMON =	0x00000000 (0)
	G_UNICODE_SCRIPT_INHERITED =	0x00000001 (1)
	G_UNICODE_SCRIPT_ARABIC =	0x00000002 (2)
	G_UNICODE_SCRIPT_ARMENIAN =	0x00000003 (3)
	G_UNICODE_SCRIPT_BENGALI =	0x00000004 (4)
	G_UNICODE_SCRIPT_BOPOMOFO =	0x00000005 (5)
	G_UNICODE_SCRIPT_CHEROKEE =	0x00000006 (6)
	G_UNICODE_SCRIPT_COPTIC =	0x00000007 (7)
	G_UNICODE_SCRIPT_CYRILLIC =	0x00000008 (8)
	G_UNICODE_SCRIPT_DESERET =	0x00000009 (9)
	G_UNICODE_SCRIPT_DEVANAGARI =	0x0000000A (10)
	G_UNICODE_SCRIPT_ETHIOPIC =	0x0000000B (11)
	G_UNICODE_SCRIPT_GEORGIAN =	0x0000000C (12)
	G_UNICODE_SCRIPT_GOTHIC =	0x0000000D (13)
	G_UNICODE_SCRIPT_GREEK =	0x0000000E (14)
	G_UNICODE_SCRIPT_GUJARATI =	0x0000000F (15)
	G_UNICODE_SCRIPT_GURMUKHI =	0x00000010 (16)
	G_UNICODE_SCRIPT_HAN =	0x00000011 (17)
	G_UNICODE_SCRIPT_HANGUL =	0x00000012 (18)
	G_UNICODE_SCRIPT_HEBREW =	0x00000013 (19)
	G_UNICODE_SCRIPT_HIRAGANA =	0x00000014 (20)
	G_UNICODE_SCRIPT_KANNADA =	0x00000015 (21)
	G_UNICODE_SCRIPT_KATAKANA =	0x00000016 (22)
	G_UNICODE_SCRIPT_KHMER =	0x00000017 (23)
	G_UNICODE_SCRIPT_LAO =	0x00000018 (24)
	G_UNICODE_SCRIPT_LATIN =	0x00000019 (25)
	G_UNICODE_SCRIPT_MALAYALAM =	0x0000001A (26)
	G_UNICODE_SCRIPT_MONGOLIAN =	0x0000001B (27)
	G_UNICODE_SCRIPT_MYANMAR =	0x0000001C (28)
	G_UNICODE_SCRIPT_OGHAM =	0x0000001D (29)
	G_UNICODE_SCRIPT_OLD_ITALIC =	0x0000001E (30)
	G_UNICODE_SCRIPT_ORIYA =	0x0000001F (31)
	G_UNICODE_SCRIPT_RUNIC =	0x00000020 (32)
	G_UNICODE_SCRIPT_SINHALA =	0x00000021 (33)
	G_UNICODE_SCRIPT_SYRIAC =	0x00000022 (34)
	G_UNICODE_SCRIPT_TAMIL =	0x00000023 (35)
	G_UNICODE_SCRIPT_TELUGU =	0x00000024 (36)
	G_UNICODE_SCRIPT_THAANA =	0x00000025 (37)
	G_UNICODE_SCRIPT_THAI =	0x00000026 (38)
	G_UNICODE_SCRIPT_TIBETAN =	0x00000027 (39)
	G_UNICODE_SCRIPT_CANADIAN_ABORIGINAL =	0x00000028 (40)
	G_UNICODE_SCRIPT_YI =	0x00000029 (41)
	G_UNICODE_SCRIPT_TAGALOG =	0x0000002A (42)
	G_UNICODE_SCRIPT_HANUNOO =	0x0000002B (43)
	G_UNICODE_SCRIPT_BUHID =	0x0000002C (44)
	G_UNICODE_SCRIPT_TAGBANWA =	0x0000002D (45)
	G_UNICODE_SCRIPT_BRAILLE =	0x0000002E (46)
	G_UNICODE_SCRIPT_CYPRIOT =	0x0000002F (47)
	G_UNICODE_SCRIPT_LIMBU =	0x00000030 (48)
	G_UNICODE_SCRIPT_OSMANYA =	0x00000031 (49)
	G_UNICODE_SCRIPT_SHAVIAN =	0x00000032 (50)
	G_UNICODE_SCRIPT_LINEAR_B =	0x00000033 (51)
	G_UNICODE_SCRIPT_TAI_LE =	0x00000034 (52)
	G_UNICODE_SCRIPT_UGARITIC =	0x00000035 (53)
	G_UNICODE_SCRIPT_NEW_TAI_LUE =	0x00000036 (54)
	G_UNICODE_SCRIPT_BUGINESE =	0x00000037 (55)
	G_UNICODE_SCRIPT_GLAGOLITIC =	0x00000038 (56)
	G_UNICODE_SCRIPT_TIFINAGH =	0x00000039 (57)
	G_UNICODE_SCRIPT_SYLOTI_NAGRI =	0x0000003A (58)
	G_UNICODE_SCRIPT_OLD_PERSIAN =	0x0000003B (59)
	G_UNICODE_SCRIPT_KHAROSHTHI =	0x0000003C (60)
	G_UNICODE_SCRIPT_UNKNOWN =	0x0000003D (61)
	G_UNICODE_SCRIPT_BALINESE =	0x0000003E (62)
	G_UNICODE_SCRIPT_CUNEIFORM =	0x0000003F (63)
	G_UNICODE_SCRIPT_PHOENICIAN =	0x00000040 (64)
	G_UNICODE_SCRIPT_PHAGS_PA =	0x00000041 (65)
	G_UNICODE_SCRIPT_NKO =	0x00000042 (66)
	G_UNICODE_SCRIPT_KAYAH_LI =	0x00000043 (67)
	G_UNICODE_SCRIPT_LEPCHA =	0x00000044 (68)
	G_UNICODE_SCRIPT_REJANG =	0x00000045 (69)
	G_UNICODE_SCRIPT_SUNDANESE =	0x00000046 (70)
	G_UNICODE_SCRIPT_SAURASHTRA =	0x00000047 (71)
	G_UNICODE_SCRIPT_CHAM =	0x00000048 (72)
	G_UNICODE_SCRIPT_OL_CHIKI =	0x00000049 (73)
	G_UNICODE_SCRIPT_VAI =	0x0000004A (74)
	G_UNICODE_SCRIPT_CARIAN =	0x0000004B (75)
	G_UNICODE_SCRIPT_LYCIAN =	0x0000004C (76)
	G_UNICODE_SCRIPT_LYDIAN =	0x0000004D (77)
	G_UNICODE_SCRIPT_AVESTAN =	0x0000004E (78)
	G_UNICODE_SCRIPT_BAMUM =	0x0000004F (79)
	G_UNICODE_SCRIPT_EGYPTIAN_HIEROGLYPHS =	0x00000050 (80)
	G_UNICODE_SCRIPT_IMPERIAL_ARAMAIC =	0x00000051 (81)
	G_UNICODE_SCRIPT_INSCRIPTIONAL_PAHLAVI =	0x00000052 (82)
	G_UNICODE_SCRIPT_INSCRIPTIONAL_PARTHIAN =	0x00000053 (83)
	G_UNICODE_SCRIPT_JAVANESE =	0x00000054 (84)
	G_UNICODE_SCRIPT_KAITHI =	0x00000055 (85)
	G_UNICODE_SCRIPT_LISU =	0x00000056 (86)
	G_UNICODE_SCRIPT_MEETEI_MAYEK =	0x00000057 (87)
	G_UNICODE_SCRIPT_OLD_SOUTH_ARABIAN =	0x00000058 (88)
	G_UNICODE_SCRIPT_OLD_TURKIC =	0x00000059 (89)
	G_UNICODE_SCRIPT_SAMARITAN =	0x0000005A (90)
	G_UNICODE_SCRIPT_TAI_THAM =	0x0000005B (91)
	G_UNICODE_SCRIPT_TAI_VIET =	0x0000005C (92)
	G_UNICODE_SCRIPT_BATAK =	0x0000005D (93)
	G_UNICODE_SCRIPT_BRAHMI =	0x0000005E (94)
	G_UNICODE_SCRIPT_MANDAIC =	0x0000005F (95)
	G_UNICODE_SCRIPT_CHAKMA =	0x00000060 (96)
	G_UNICODE_SCRIPT_MEROITIC_CURSIVE =	0x00000061 (97)
	G_UNICODE_SCRIPT_MEROITIC_HIEROGLYPHS =	0x00000062 (98)
	G_UNICODE_SCRIPT_MIAO =	0x00000063 (99)
	G_UNICODE_SCRIPT_SHARADA =	0x00000064 (100)
	G_UNICODE_SCRIPT_SORA_SOMPENG =	0x00000065 (101)
	G_UNICODE_SCRIPT_TAKRI =	0x00000066 (102)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(i32)]
pub enum GUnicodeScript {
	G_UNICODE_SCRIPT_INVALID_CODE =	-1 as i32,
	G_UNICODE_SCRIPT_COMMON =	0 as i32,
	G_UNICODE_SCRIPT_INHERITED =	1 as i32,
	G_UNICODE_SCRIPT_ARABIC =	2 as i32,
	G_UNICODE_SCRIPT_ARMENIAN =	3 as i32,
	G_UNICODE_SCRIPT_BENGALI =	4 as i32,
	G_UNICODE_SCRIPT_BOPOMOFO =	5 as i32,
	G_UNICODE_SCRIPT_CHEROKEE =	6 as i32,
	G_UNICODE_SCRIPT_COPTIC =	7 as i32,
	G_UNICODE_SCRIPT_CYRILLIC =	8 as i32,
	G_UNICODE_SCRIPT_DESERET =	9 as i32,
	G_UNICODE_SCRIPT_DEVANAGARI =	10 as i32,
	G_UNICODE_SCRIPT_ETHIOPIC =	11 as i32,
	G_UNICODE_SCRIPT_GEORGIAN =	12 as i32,
	G_UNICODE_SCRIPT_GOTHIC =	13 as i32,
	G_UNICODE_SCRIPT_GREEK =	14 as i32,
	G_UNICODE_SCRIPT_GUJARATI =	15 as i32,
	G_UNICODE_SCRIPT_GURMUKHI =	16 as i32,
	G_UNICODE_SCRIPT_HAN =	17 as i32,
	G_UNICODE_SCRIPT_HANGUL =	18 as i32,
	G_UNICODE_SCRIPT_HEBREW =	19 as i32,
	G_UNICODE_SCRIPT_HIRAGANA =	20 as i32,
	G_UNICODE_SCRIPT_KANNADA =	21 as i32,
	G_UNICODE_SCRIPT_KATAKANA =	22 as i32,
	G_UNICODE_SCRIPT_KHMER =	23 as i32,
	G_UNICODE_SCRIPT_LAO =	24 as i32,
	G_UNICODE_SCRIPT_LATIN =	25 as i32,
	G_UNICODE_SCRIPT_MALAYALAM =	26 as i32,
	G_UNICODE_SCRIPT_MONGOLIAN =	27 as i32,
	G_UNICODE_SCRIPT_MYANMAR =	28 as i32,
	G_UNICODE_SCRIPT_OGHAM =	29 as i32,
	G_UNICODE_SCRIPT_OLD_ITALIC =	30 as i32,
	G_UNICODE_SCRIPT_ORIYA =	31 as i32,
	G_UNICODE_SCRIPT_RUNIC =	32 as i32,
	G_UNICODE_SCRIPT_SINHALA =	33 as i32,
	G_UNICODE_SCRIPT_SYRIAC =	34 as i32,
	G_UNICODE_SCRIPT_TAMIL =	35 as i32,
	G_UNICODE_SCRIPT_TELUGU =	36 as i32,
	G_UNICODE_SCRIPT_THAANA =	37 as i32,
	G_UNICODE_SCRIPT_THAI =	38 as i32,
	G_UNICODE_SCRIPT_TIBETAN =	39 as i32,
	G_UNICODE_SCRIPT_CANADIAN_ABORIGINAL =	40 as i32,
	G_UNICODE_SCRIPT_YI =	41 as i32,
	G_UNICODE_SCRIPT_TAGALOG =	42 as i32,
	G_UNICODE_SCRIPT_HANUNOO =	43 as i32,
	G_UNICODE_SCRIPT_BUHID =	44 as i32,
	G_UNICODE_SCRIPT_TAGBANWA =	45 as i32,
	G_UNICODE_SCRIPT_BRAILLE =	46 as i32,
	G_UNICODE_SCRIPT_CYPRIOT =	47 as i32,
	G_UNICODE_SCRIPT_LIMBU =	48 as i32,
	G_UNICODE_SCRIPT_OSMANYA =	49 as i32,
	G_UNICODE_SCRIPT_SHAVIAN =	50 as i32,
	G_UNICODE_SCRIPT_LINEAR_B =	51 as i32,
	G_UNICODE_SCRIPT_TAI_LE =	52 as i32,
	G_UNICODE_SCRIPT_UGARITIC =	53 as i32,
	G_UNICODE_SCRIPT_NEW_TAI_LUE =	54 as i32,
	G_UNICODE_SCRIPT_BUGINESE =	55 as i32,
	G_UNICODE_SCRIPT_GLAGOLITIC =	56 as i32,
	G_UNICODE_SCRIPT_TIFINAGH =	57 as i32,
	G_UNICODE_SCRIPT_SYLOTI_NAGRI =	58 as i32,
	G_UNICODE_SCRIPT_OLD_PERSIAN =	59 as i32,
	G_UNICODE_SCRIPT_KHAROSHTHI =	60 as i32,
	G_UNICODE_SCRIPT_UNKNOWN =	61 as i32,
	G_UNICODE_SCRIPT_BALINESE =	62 as i32,
	G_UNICODE_SCRIPT_CUNEIFORM =	63 as i32,
	G_UNICODE_SCRIPT_PHOENICIAN =	64 as i32,
	G_UNICODE_SCRIPT_PHAGS_PA =	65 as i32,
	G_UNICODE_SCRIPT_NKO =	66 as i32,
	G_UNICODE_SCRIPT_KAYAH_LI =	67 as i32,
	G_UNICODE_SCRIPT_LEPCHA =	68 as i32,
	G_UNICODE_SCRIPT_REJANG =	69 as i32,
	G_UNICODE_SCRIPT_SUNDANESE =	70 as i32,
	G_UNICODE_SCRIPT_SAURASHTRA =	71 as i32,
	G_UNICODE_SCRIPT_CHAM =	72 as i32,
	G_UNICODE_SCRIPT_OL_CHIKI =	73 as i32,
	G_UNICODE_SCRIPT_VAI =	74 as i32,
	G_UNICODE_SCRIPT_CARIAN =	75 as i32,
	G_UNICODE_SCRIPT_LYCIAN =	76 as i32,
	G_UNICODE_SCRIPT_LYDIAN =	77 as i32,
	G_UNICODE_SCRIPT_AVESTAN =	78 as i32,
	G_UNICODE_SCRIPT_BAMUM =	79 as i32,
	G_UNICODE_SCRIPT_EGYPTIAN_HIEROGLYPHS =	80 as i32,
	G_UNICODE_SCRIPT_IMPERIAL_ARAMAIC =	81 as i32,
	G_UNICODE_SCRIPT_INSCRIPTIONAL_PAHLAVI =	82 as i32,
	G_UNICODE_SCRIPT_INSCRIPTIONAL_PARTHIAN =	83 as i32,
	G_UNICODE_SCRIPT_JAVANESE =	84 as i32,
	G_UNICODE_SCRIPT_KAITHI =	85 as i32,
	G_UNICODE_SCRIPT_LISU =	86 as i32,
	G_UNICODE_SCRIPT_MEETEI_MAYEK =	87 as i32,
	G_UNICODE_SCRIPT_OLD_SOUTH_ARABIAN =	88 as i32,
	G_UNICODE_SCRIPT_OLD_TURKIC =	89 as i32,
	G_UNICODE_SCRIPT_SAMARITAN =	90 as i32,
	G_UNICODE_SCRIPT_TAI_THAM =	91 as i32,
	G_UNICODE_SCRIPT_TAI_VIET =	92 as i32,
	G_UNICODE_SCRIPT_BATAK =	93 as i32,
	G_UNICODE_SCRIPT_BRAHMI =	94 as i32,
	G_UNICODE_SCRIPT_MANDAIC =	95 as i32,
	G_UNICODE_SCRIPT_CHAKMA =	96 as i32,
	G_UNICODE_SCRIPT_MEROITIC_CURSIVE =	97 as i32,
	G_UNICODE_SCRIPT_MEROITIC_HIEROGLYPHS =	98 as i32,
	G_UNICODE_SCRIPT_MIAO =	99 as i32,
	G_UNICODE_SCRIPT_SHARADA =	100 as i32,
	G_UNICODE_SCRIPT_SORA_SOMPENG =	101 as i32,
	G_UNICODE_SCRIPT_TAKRI =	102 as i32,
}

impl GUnicodeScript {
	pub fn to_i32(&self) -> libc::c_int {
		*self as libc::c_int
	}

	pub fn from_i32(v: libc::c_int) -> GUnicodeScript {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_UNICODE_SCRIPT_INVALID_CODE =	0x-0000001 (-1)
	G_UNICODE_SCRIPT_COMMON =	0x00000000 (0)
	G_UNICODE_SCRIPT_INHERITED =	0x00000001 (1)
	G_UNICODE_SCRIPT_ARABIC =	0x00000002 (2)
	G_UNICODE_SCRIPT_ARMENIAN =	0x00000003 (3)
	G_UNICODE_SCRIPT_BENGALI =	0x00000004 (4)
	G_UNICODE_SCRIPT_BOPOMOFO =	0x00000005 (5)
	G_UNICODE_SCRIPT_CHEROKEE =	0x00000006 (6)
	G_UNICODE_SCRIPT_COPTIC =	0x00000007 (7)
	G_UNICODE_SCRIPT_CYRILLIC =	0x00000008 (8)
	G_UNICODE_SCRIPT_DESERET =	0x00000009 (9)
	G_UNICODE_SCRIPT_DEVANAGARI =	0x0000000A (10)
	G_UNICODE_SCRIPT_ETHIOPIC =	0x0000000B (11)
	G_UNICODE_SCRIPT_GEORGIAN =	0x0000000C (12)
	G_UNICODE_SCRIPT_GOTHIC =	0x0000000D (13)
	G_UNICODE_SCRIPT_GREEK =	0x0000000E (14)
	G_UNICODE_SCRIPT_GUJARATI =	0x0000000F (15)
	G_UNICODE_SCRIPT_GURMUKHI =	0x00000010 (16)
	G_UNICODE_SCRIPT_HAN =	0x00000011 (17)
	G_UNICODE_SCRIPT_HANGUL =	0x00000012 (18)
	G_UNICODE_SCRIPT_HEBREW =	0x00000013 (19)
	G_UNICODE_SCRIPT_HIRAGANA =	0x00000014 (20)
	G_UNICODE_SCRIPT_KANNADA =	0x00000015 (21)
	G_UNICODE_SCRIPT_KATAKANA =	0x00000016 (22)
	G_UNICODE_SCRIPT_KHMER =	0x00000017 (23)
	G_UNICODE_SCRIPT_LAO =	0x00000018 (24)
	G_UNICODE_SCRIPT_LATIN =	0x00000019 (25)
	G_UNICODE_SCRIPT_MALAYALAM =	0x0000001A (26)
	G_UNICODE_SCRIPT_MONGOLIAN =	0x0000001B (27)
	G_UNICODE_SCRIPT_MYANMAR =	0x0000001C (28)
	G_UNICODE_SCRIPT_OGHAM =	0x0000001D (29)
	G_UNICODE_SCRIPT_OLD_ITALIC =	0x0000001E (30)
	G_UNICODE_SCRIPT_ORIYA =	0x0000001F (31)
	G_UNICODE_SCRIPT_RUNIC =	0x00000020 (32)
	G_UNICODE_SCRIPT_SINHALA =	0x00000021 (33)
	G_UNICODE_SCRIPT_SYRIAC =	0x00000022 (34)
	G_UNICODE_SCRIPT_TAMIL =	0x00000023 (35)
	G_UNICODE_SCRIPT_TELUGU =	0x00000024 (36)
	G_UNICODE_SCRIPT_THAANA =	0x00000025 (37)
	G_UNICODE_SCRIPT_THAI =	0x00000026 (38)
	G_UNICODE_SCRIPT_TIBETAN =	0x00000027 (39)
	G_UNICODE_SCRIPT_CANADIAN_ABORIGINAL =	0x00000028 (40)
	G_UNICODE_SCRIPT_YI =	0x00000029 (41)
	G_UNICODE_SCRIPT_TAGALOG =	0x0000002A (42)
	G_UNICODE_SCRIPT_HANUNOO =	0x0000002B (43)
	G_UNICODE_SCRIPT_BUHID =	0x0000002C (44)
	G_UNICODE_SCRIPT_TAGBANWA =	0x0000002D (45)
	G_UNICODE_SCRIPT_BRAILLE =	0x0000002E (46)
	G_UNICODE_SCRIPT_CYPRIOT =	0x0000002F (47)
	G_UNICODE_SCRIPT_LIMBU =	0x00000030 (48)
	G_UNICODE_SCRIPT_OSMANYA =	0x00000031 (49)
	G_UNICODE_SCRIPT_SHAVIAN =	0x00000032 (50)
	G_UNICODE_SCRIPT_LINEAR_B =	0x00000033 (51)
	G_UNICODE_SCRIPT_TAI_LE =	0x00000034 (52)
	G_UNICODE_SCRIPT_UGARITIC =	0x00000035 (53)
	G_UNICODE_SCRIPT_NEW_TAI_LUE =	0x00000036 (54)
	G_UNICODE_SCRIPT_BUGINESE =	0x00000037 (55)
	G_UNICODE_SCRIPT_GLAGOLITIC =	0x00000038 (56)
	G_UNICODE_SCRIPT_TIFINAGH =	0x00000039 (57)
	G_UNICODE_SCRIPT_SYLOTI_NAGRI =	0x0000003A (58)
	G_UNICODE_SCRIPT_OLD_PERSIAN =	0x0000003B (59)
	G_UNICODE_SCRIPT_KHAROSHTHI =	0x0000003C (60)
	G_UNICODE_SCRIPT_UNKNOWN =	0x0000003D (61)
	G_UNICODE_SCRIPT_BALINESE =	0x0000003E (62)
	G_UNICODE_SCRIPT_CUNEIFORM =	0x0000003F (63)
	G_UNICODE_SCRIPT_PHOENICIAN =	0x00000040 (64)
	G_UNICODE_SCRIPT_PHAGS_PA =	0x00000041 (65)
	G_UNICODE_SCRIPT_NKO =	0x00000042 (66)
	G_UNICODE_SCRIPT_KAYAH_LI =	0x00000043 (67)
	G_UNICODE_SCRIPT_LEPCHA =	0x00000044 (68)
	G_UNICODE_SCRIPT_REJANG =	0x00000045 (69)
	G_UNICODE_SCRIPT_SUNDANESE =	0x00000046 (70)
	G_UNICODE_SCRIPT_SAURASHTRA =	0x00000047 (71)
	G_UNICODE_SCRIPT_CHAM =	0x00000048 (72)
	G_UNICODE_SCRIPT_OL_CHIKI =	0x00000049 (73)
	G_UNICODE_SCRIPT_VAI =	0x0000004A (74)
	G_UNICODE_SCRIPT_CARIAN =	0x0000004B (75)
	G_UNICODE_SCRIPT_LYCIAN =	0x0000004C (76)
	G_UNICODE_SCRIPT_LYDIAN =	0x0000004D (77)
	G_UNICODE_SCRIPT_AVESTAN =	0x0000004E (78)
	G_UNICODE_SCRIPT_BAMUM =	0x0000004F (79)
	G_UNICODE_SCRIPT_EGYPTIAN_HIEROGLYPHS =	0x00000050 (80)
	G_UNICODE_SCRIPT_IMPERIAL_ARAMAIC =	0x00000051 (81)
	G_UNICODE_SCRIPT_INSCRIPTIONAL_PAHLAVI =	0x00000052 (82)
	G_UNICODE_SCRIPT_INSCRIPTIONAL_PARTHIAN =	0x00000053 (83)
	G_UNICODE_SCRIPT_JAVANESE =	0x00000054 (84)
	G_UNICODE_SCRIPT_KAITHI =	0x00000055 (85)
	G_UNICODE_SCRIPT_LISU =	0x00000056 (86)
	G_UNICODE_SCRIPT_MEETEI_MAYEK =	0x00000057 (87)
	G_UNICODE_SCRIPT_OLD_SOUTH_ARABIAN =	0x00000058 (88)
	G_UNICODE_SCRIPT_OLD_TURKIC =	0x00000059 (89)
	G_UNICODE_SCRIPT_SAMARITAN =	0x0000005A (90)
	G_UNICODE_SCRIPT_TAI_THAM =	0x0000005B (91)
	G_UNICODE_SCRIPT_TAI_VIET =	0x0000005C (92)
	G_UNICODE_SCRIPT_BATAK =	0x0000005D (93)
	G_UNICODE_SCRIPT_BRAHMI =	0x0000005E (94)
	G_UNICODE_SCRIPT_MANDAIC =	0x0000005F (95)
	G_UNICODE_SCRIPT_CHAKMA =	0x00000060 (96)
	G_UNICODE_SCRIPT_MEROITIC_CURSIVE =	0x00000061 (97)
	G_UNICODE_SCRIPT_MEROITIC_HIEROGLYPHS =	0x00000062 (98)
	G_UNICODE_SCRIPT_MIAO =	0x00000063 (99)
	G_UNICODE_SCRIPT_SHARADA =	0x00000064 (100)
	G_UNICODE_SCRIPT_SORA_SOMPENG =	0x00000065 (101)
	G_UNICODE_SCRIPT_TAKRI =	0x00000066 (102)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(i32)]
pub enum GUnicodeScript {
	G_UNICODE_SCRIPT_INVALID_CODE =	-1 as i32,
	G_UNICODE_SCRIPT_COMMON =	0 as i32,
	G_UNICODE_SCRIPT_INHERITED =	1 as i32,
	G_UNICODE_SCRIPT_ARABIC =	2 as i32,
	G_UNICODE_SCRIPT_ARMENIAN =	3 as i32,
	G_UNICODE_SCRIPT_BENGALI =	4 as i32,
	G_UNICODE_SCRIPT_BOPOMOFO =	5 as i32,
	G_UNICODE_SCRIPT_CHEROKEE =	6 as i32,
	G_UNICODE_SCRIPT_COPTIC =	7 as i32,
	G_UNICODE_SCRIPT_CYRILLIC =	8 as i32,
	G_UNICODE_SCRIPT_DESERET =	9 as i32,
	G_UNICODE_SCRIPT_DEVANAGARI =	10 as i32,
	G_UNICODE_SCRIPT_ETHIOPIC =	11 as i32,
	G_UNICODE_SCRIPT_GEORGIAN =	12 as i32,
	G_UNICODE_SCRIPT_GOTHIC =	13 as i32,
	G_UNICODE_SCRIPT_GREEK =	14 as i32,
	G_UNICODE_SCRIPT_GUJARATI =	15 as i32,
	G_UNICODE_SCRIPT_GURMUKHI =	16 as i32,
	G_UNICODE_SCRIPT_HAN =	17 as i32,
	G_UNICODE_SCRIPT_HANGUL =	18 as i32,
	G_UNICODE_SCRIPT_HEBREW =	19 as i32,
	G_UNICODE_SCRIPT_HIRAGANA =	20 as i32,
	G_UNICODE_SCRIPT_KANNADA =	21 as i32,
	G_UNICODE_SCRIPT_KATAKANA =	22 as i32,
	G_UNICODE_SCRIPT_KHMER =	23 as i32,
	G_UNICODE_SCRIPT_LAO =	24 as i32,
	G_UNICODE_SCRIPT_LATIN =	25 as i32,
	G_UNICODE_SCRIPT_MALAYALAM =	26 as i32,
	G_UNICODE_SCRIPT_MONGOLIAN =	27 as i32,
	G_UNICODE_SCRIPT_MYANMAR =	28 as i32,
	G_UNICODE_SCRIPT_OGHAM =	29 as i32,
	G_UNICODE_SCRIPT_OLD_ITALIC =	30 as i32,
	G_UNICODE_SCRIPT_ORIYA =	31 as i32,
	G_UNICODE_SCRIPT_RUNIC =	32 as i32,
	G_UNICODE_SCRIPT_SINHALA =	33 as i32,
	G_UNICODE_SCRIPT_SYRIAC =	34 as i32,
	G_UNICODE_SCRIPT_TAMIL =	35 as i32,
	G_UNICODE_SCRIPT_TELUGU =	36 as i32,
	G_UNICODE_SCRIPT_THAANA =	37 as i32,
	G_UNICODE_SCRIPT_THAI =	38 as i32,
	G_UNICODE_SCRIPT_TIBETAN =	39 as i32,
	G_UNICODE_SCRIPT_CANADIAN_ABORIGINAL =	40 as i32,
	G_UNICODE_SCRIPT_YI =	41 as i32,
	G_UNICODE_SCRIPT_TAGALOG =	42 as i32,
	G_UNICODE_SCRIPT_HANUNOO =	43 as i32,
	G_UNICODE_SCRIPT_BUHID =	44 as i32,
	G_UNICODE_SCRIPT_TAGBANWA =	45 as i32,
	G_UNICODE_SCRIPT_BRAILLE =	46 as i32,
	G_UNICODE_SCRIPT_CYPRIOT =	47 as i32,
	G_UNICODE_SCRIPT_LIMBU =	48 as i32,
	G_UNICODE_SCRIPT_OSMANYA =	49 as i32,
	G_UNICODE_SCRIPT_SHAVIAN =	50 as i32,
	G_UNICODE_SCRIPT_LINEAR_B =	51 as i32,
	G_UNICODE_SCRIPT_TAI_LE =	52 as i32,
	G_UNICODE_SCRIPT_UGARITIC =	53 as i32,
	G_UNICODE_SCRIPT_NEW_TAI_LUE =	54 as i32,
	G_UNICODE_SCRIPT_BUGINESE =	55 as i32,
	G_UNICODE_SCRIPT_GLAGOLITIC =	56 as i32,
	G_UNICODE_SCRIPT_TIFINAGH =	57 as i32,
	G_UNICODE_SCRIPT_SYLOTI_NAGRI =	58 as i32,
	G_UNICODE_SCRIPT_OLD_PERSIAN =	59 as i32,
	G_UNICODE_SCRIPT_KHAROSHTHI =	60 as i32,
	G_UNICODE_SCRIPT_UNKNOWN =	61 as i32,
	G_UNICODE_SCRIPT_BALINESE =	62 as i32,
	G_UNICODE_SCRIPT_CUNEIFORM =	63 as i32,
	G_UNICODE_SCRIPT_PHOENICIAN =	64 as i32,
	G_UNICODE_SCRIPT_PHAGS_PA =	65 as i32,
	G_UNICODE_SCRIPT_NKO =	66 as i32,
	G_UNICODE_SCRIPT_KAYAH_LI =	67 as i32,
	G_UNICODE_SCRIPT_LEPCHA =	68 as i32,
	G_UNICODE_SCRIPT_REJANG =	69 as i32,
	G_UNICODE_SCRIPT_SUNDANESE =	70 as i32,
	G_UNICODE_SCRIPT_SAURASHTRA =	71 as i32,
	G_UNICODE_SCRIPT_CHAM =	72 as i32,
	G_UNICODE_SCRIPT_OL_CHIKI =	73 as i32,
	G_UNICODE_SCRIPT_VAI =	74 as i32,
	G_UNICODE_SCRIPT_CARIAN =	75 as i32,
	G_UNICODE_SCRIPT_LYCIAN =	76 as i32,
	G_UNICODE_SCRIPT_LYDIAN =	77 as i32,
	G_UNICODE_SCRIPT_AVESTAN =	78 as i32,
	G_UNICODE_SCRIPT_BAMUM =	79 as i32,
	G_UNICODE_SCRIPT_EGYPTIAN_HIEROGLYPHS =	80 as i32,
	G_UNICODE_SCRIPT_IMPERIAL_ARAMAIC =	81 as i32,
	G_UNICODE_SCRIPT_INSCRIPTIONAL_PAHLAVI =	82 as i32,
	G_UNICODE_SCRIPT_INSCRIPTIONAL_PARTHIAN =	83 as i32,
	G_UNICODE_SCRIPT_JAVANESE =	84 as i32,
	G_UNICODE_SCRIPT_KAITHI =	85 as i32,
	G_UNICODE_SCRIPT_LISU =	86 as i32,
	G_UNICODE_SCRIPT_MEETEI_MAYEK =	87 as i32,
	G_UNICODE_SCRIPT_OLD_SOUTH_ARABIAN =	88 as i32,
	G_UNICODE_SCRIPT_OLD_TURKIC =	89 as i32,
	G_UNICODE_SCRIPT_SAMARITAN =	90 as i32,
	G_UNICODE_SCRIPT_TAI_THAM =	91 as i32,
	G_UNICODE_SCRIPT_TAI_VIET =	92 as i32,
	G_UNICODE_SCRIPT_BATAK =	93 as i32,
	G_UNICODE_SCRIPT_BRAHMI =	94 as i32,
	G_UNICODE_SCRIPT_MANDAIC =	95 as i32,
	G_UNICODE_SCRIPT_CHAKMA =	96 as i32,
	G_UNICODE_SCRIPT_MEROITIC_CURSIVE =	97 as i32,
	G_UNICODE_SCRIPT_MEROITIC_HIEROGLYPHS =	98 as i32,
	G_UNICODE_SCRIPT_MIAO =	99 as i32,
	G_UNICODE_SCRIPT_SHARADA =	100 as i32,
	G_UNICODE_SCRIPT_SORA_SOMPENG =	101 as i32,
	G_UNICODE_SCRIPT_TAKRI =	102 as i32,
}

impl GUnicodeScript {
	pub fn to_i32(&self) -> libc::c_int {
		*self as libc::c_int
	}

	pub fn from_i32(v: libc::c_int) -> GUnicodeScript {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_NORMALIZE_DEFAULT =	0x00000000 (0)
	G_NORMALIZE_NFD =	0x00000000 (0)
	G_NORMALIZE_DEFAULT_COMPOSE =	0x00000001 (1)
	G_NORMALIZE_NFC =	0x00000001 (1)
	G_NORMALIZE_ALL =	0x00000002 (2)
	G_NORMALIZE_NFKD =	0x00000002 (2)
	G_NORMALIZE_ALL_COMPOSE =	0x00000003 (3)
	G_NORMALIZE_NFKC =	0x00000003 (3)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GNormalizeMode {
	G_NORMALIZE_DEFAULT =	0 as u32,
	G_NORMALIZE_NFD =	0 as u32,
	G_NORMALIZE_DEFAULT_COMPOSE =	1 as u32,
	G_NORMALIZE_NFC =	1 as u32,
	G_NORMALIZE_ALL =	2 as u32,
	G_NORMALIZE_NFKD =	2 as u32,
	G_NORMALIZE_ALL_COMPOSE =	3 as u32,
	G_NORMALIZE_NFKC =	3 as u32,
}

impl GNormalizeMode {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GNormalizeMode {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_NORMALIZE_DEFAULT =	0x00000000 (0)
	G_NORMALIZE_NFD =	0x00000000 (0)
	G_NORMALIZE_DEFAULT_COMPOSE =	0x00000001 (1)
	G_NORMALIZE_NFC =	0x00000001 (1)
	G_NORMALIZE_ALL =	0x00000002 (2)
	G_NORMALIZE_NFKD =	0x00000002 (2)
	G_NORMALIZE_ALL_COMPOSE =	0x00000003 (3)
	G_NORMALIZE_NFKC =	0x00000003 (3)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GNormalizeMode {
	G_NORMALIZE_DEFAULT =	0 as u32,
	G_NORMALIZE_NFD =	0 as u32,
	G_NORMALIZE_DEFAULT_COMPOSE =	1 as u32,
	G_NORMALIZE_NFC =	1 as u32,
	G_NORMALIZE_ALL =	2 as u32,
	G_NORMALIZE_NFKD =	2 as u32,
	G_NORMALIZE_ALL_COMPOSE =	3 as u32,
	G_NORMALIZE_NFKC =	3 as u32,
}

impl GNormalizeMode {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GNormalizeMode {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_USER_DIRECTORY_DESKTOP =	0x00000000 (0)
	G_USER_DIRECTORY_DOCUMENTS =	0x00000001 (1)
	G_USER_DIRECTORY_DOWNLOAD =	0x00000002 (2)
	G_USER_DIRECTORY_MUSIC =	0x00000003 (3)
	G_USER_DIRECTORY_PICTURES =	0x00000004 (4)
	G_USER_DIRECTORY_PUBLIC_SHARE =	0x00000005 (5)
	G_USER_DIRECTORY_TEMPLATES =	0x00000006 (6)
	G_USER_DIRECTORY_VIDEOS =	0x00000007 (7)
	G_USER_N_DIRECTORIES =	0x00000008 (8)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GUserDirectory {
	G_USER_DIRECTORY_DESKTOP =	0 as u32,
	G_USER_DIRECTORY_DOCUMENTS =	1 as u32,
	G_USER_DIRECTORY_DOWNLOAD =	2 as u32,
	G_USER_DIRECTORY_MUSIC =	3 as u32,
	G_USER_DIRECTORY_PICTURES =	4 as u32,
	G_USER_DIRECTORY_PUBLIC_SHARE =	5 as u32,
	G_USER_DIRECTORY_TEMPLATES =	6 as u32,
	G_USER_DIRECTORY_VIDEOS =	7 as u32,
	G_USER_N_DIRECTORIES =	8 as u32,
}

impl GUserDirectory {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GUserDirectory {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_USER_DIRECTORY_DESKTOP =	0x00000000 (0)
	G_USER_DIRECTORY_DOCUMENTS =	0x00000001 (1)
	G_USER_DIRECTORY_DOWNLOAD =	0x00000002 (2)
	G_USER_DIRECTORY_MUSIC =	0x00000003 (3)
	G_USER_DIRECTORY_PICTURES =	0x00000004 (4)
	G_USER_DIRECTORY_PUBLIC_SHARE =	0x00000005 (5)
	G_USER_DIRECTORY_TEMPLATES =	0x00000006 (6)
	G_USER_DIRECTORY_VIDEOS =	0x00000007 (7)
	G_USER_N_DIRECTORIES =	0x00000008 (8)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GUserDirectory {
	G_USER_DIRECTORY_DESKTOP =	0 as u32,
	G_USER_DIRECTORY_DOCUMENTS =	1 as u32,
	G_USER_DIRECTORY_DOWNLOAD =	2 as u32,
	G_USER_DIRECTORY_MUSIC =	3 as u32,
	G_USER_DIRECTORY_PICTURES =	4 as u32,
	G_USER_DIRECTORY_PUBLIC_SHARE =	5 as u32,
	G_USER_DIRECTORY_TEMPLATES =	6 as u32,
	G_USER_DIRECTORY_VIDEOS =	7 as u32,
	G_USER_N_DIRECTORIES =	8 as u32,
}

impl GUserDirectory {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GUserDirectory {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_FORMAT_SIZE_DEFAULT =	0x00000000 (0)
	G_FORMAT_SIZE_LONG_FORMAT =	0x00000001 (1)
	G_FORMAT_SIZE_IEC_UNITS =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GFormatSizeFlags {
	G_FORMAT_SIZE_DEFAULT =	0 as u32,
	G_FORMAT_SIZE_LONG_FORMAT =	1 as u32,
	G_FORMAT_SIZE_IEC_UNITS =	2 as u32,
}

impl GFormatSizeFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GFormatSizeFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_FORMAT_SIZE_DEFAULT =	0x00000000 (0)
	G_FORMAT_SIZE_LONG_FORMAT =	0x00000001 (1)
	G_FORMAT_SIZE_IEC_UNITS =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GFormatSizeFlags {
	G_FORMAT_SIZE_DEFAULT =	0 as u32,
	G_FORMAT_SIZE_LONG_FORMAT =	1 as u32,
	G_FORMAT_SIZE_IEC_UNITS =	2 as u32,
}

impl GFormatSizeFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GFormatSizeFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_IO_ERROR_NONE =	0x00000000 (0)
	G_IO_ERROR_AGAIN =	0x00000001 (1)
	G_IO_ERROR_INVAL =	0x00000002 (2)
	G_IO_ERROR_UNKNOWN =	0x00000003 (3)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GIOError {
	G_IO_ERROR_NONE =	0 as u32,
	G_IO_ERROR_AGAIN =	1 as u32,
	G_IO_ERROR_INVAL =	2 as u32,
	G_IO_ERROR_UNKNOWN =	3 as u32,
}

impl GIOError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GIOError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_IO_ERROR_NONE =	0x00000000 (0)
	G_IO_ERROR_AGAIN =	0x00000001 (1)
	G_IO_ERROR_INVAL =	0x00000002 (2)
	G_IO_ERROR_UNKNOWN =	0x00000003 (3)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GIOError {
	G_IO_ERROR_NONE =	0 as u32,
	G_IO_ERROR_AGAIN =	1 as u32,
	G_IO_ERROR_INVAL =	2 as u32,
	G_IO_ERROR_UNKNOWN =	3 as u32,
}

impl GIOError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GIOError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_IO_CHANNEL_ERROR_FBIG =	0x00000000 (0)
	G_IO_CHANNEL_ERROR_INVAL =	0x00000001 (1)
	G_IO_CHANNEL_ERROR_IO =	0x00000002 (2)
	G_IO_CHANNEL_ERROR_ISDIR =	0x00000003 (3)
	G_IO_CHANNEL_ERROR_NOSPC =	0x00000004 (4)
	G_IO_CHANNEL_ERROR_NXIO =	0x00000005 (5)
	G_IO_CHANNEL_ERROR_OVERFLOW =	0x00000006 (6)
	G_IO_CHANNEL_ERROR_PIPE =	0x00000007 (7)
	G_IO_CHANNEL_ERROR_FAILED =	0x00000008 (8)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GIOChannelError {
	G_IO_CHANNEL_ERROR_FBIG =	0 as u32,
	G_IO_CHANNEL_ERROR_INVAL =	1 as u32,
	G_IO_CHANNEL_ERROR_IO =	2 as u32,
	G_IO_CHANNEL_ERROR_ISDIR =	3 as u32,
	G_IO_CHANNEL_ERROR_NOSPC =	4 as u32,
	G_IO_CHANNEL_ERROR_NXIO =	5 as u32,
	G_IO_CHANNEL_ERROR_OVERFLOW =	6 as u32,
	G_IO_CHANNEL_ERROR_PIPE =	7 as u32,
	G_IO_CHANNEL_ERROR_FAILED =	8 as u32,
}

impl GIOChannelError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GIOChannelError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_IO_CHANNEL_ERROR_FBIG =	0x00000000 (0)
	G_IO_CHANNEL_ERROR_INVAL =	0x00000001 (1)
	G_IO_CHANNEL_ERROR_IO =	0x00000002 (2)
	G_IO_CHANNEL_ERROR_ISDIR =	0x00000003 (3)
	G_IO_CHANNEL_ERROR_NOSPC =	0x00000004 (4)
	G_IO_CHANNEL_ERROR_NXIO =	0x00000005 (5)
	G_IO_CHANNEL_ERROR_OVERFLOW =	0x00000006 (6)
	G_IO_CHANNEL_ERROR_PIPE =	0x00000007 (7)
	G_IO_CHANNEL_ERROR_FAILED =	0x00000008 (8)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GIOChannelError {
	G_IO_CHANNEL_ERROR_FBIG =	0 as u32,
	G_IO_CHANNEL_ERROR_INVAL =	1 as u32,
	G_IO_CHANNEL_ERROR_IO =	2 as u32,
	G_IO_CHANNEL_ERROR_ISDIR =	3 as u32,
	G_IO_CHANNEL_ERROR_NOSPC =	4 as u32,
	G_IO_CHANNEL_ERROR_NXIO =	5 as u32,
	G_IO_CHANNEL_ERROR_OVERFLOW =	6 as u32,
	G_IO_CHANNEL_ERROR_PIPE =	7 as u32,
	G_IO_CHANNEL_ERROR_FAILED =	8 as u32,
}

impl GIOChannelError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GIOChannelError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_IO_STATUS_ERROR =	0x00000000 (0)
	G_IO_STATUS_NORMAL =	0x00000001 (1)
	G_IO_STATUS_EOF =	0x00000002 (2)
	G_IO_STATUS_AGAIN =	0x00000003 (3)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GIOStatus {
	G_IO_STATUS_ERROR =	0 as u32,
	G_IO_STATUS_NORMAL =	1 as u32,
	G_IO_STATUS_EOF =	2 as u32,
	G_IO_STATUS_AGAIN =	3 as u32,
}

impl GIOStatus {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GIOStatus {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_IO_STATUS_ERROR =	0x00000000 (0)
	G_IO_STATUS_NORMAL =	0x00000001 (1)
	G_IO_STATUS_EOF =	0x00000002 (2)
	G_IO_STATUS_AGAIN =	0x00000003 (3)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GIOStatus {
	G_IO_STATUS_ERROR =	0 as u32,
	G_IO_STATUS_NORMAL =	1 as u32,
	G_IO_STATUS_EOF =	2 as u32,
	G_IO_STATUS_AGAIN =	3 as u32,
}

impl GIOStatus {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GIOStatus {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_SEEK_CUR =	0x00000000 (0)
	G_SEEK_SET =	0x00000001 (1)
	G_SEEK_END =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GSeekType {
	G_SEEK_CUR =	0 as u32,
	G_SEEK_SET =	1 as u32,
	G_SEEK_END =	2 as u32,
}

impl GSeekType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GSeekType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_SEEK_CUR =	0x00000000 (0)
	G_SEEK_SET =	0x00000001 (1)
	G_SEEK_END =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GSeekType {
	G_SEEK_CUR =	0 as u32,
	G_SEEK_SET =	1 as u32,
	G_SEEK_END =	2 as u32,
}

impl GSeekType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GSeekType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_IO_IN =	0x00000001 (1)
	G_IO_OUT =	0x00000004 (4)
	G_IO_PRI =	0x00000002 (2)
	G_IO_ERR =	0x00000008 (8)
	G_IO_HUP =	0x00000010 (16)
	G_IO_NVAL =	0x00000020 (32)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GIOCondition {
	G_IO_IN =	1 as u32,
	G_IO_OUT =	4 as u32,
	G_IO_PRI =	2 as u32,
	G_IO_ERR =	8 as u32,
	G_IO_HUP =	16 as u32,
	G_IO_NVAL =	32 as u32,
}

impl GIOCondition {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GIOCondition {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_IO_IN =	0x00000001 (1)
	G_IO_OUT =	0x00000004 (4)
	G_IO_PRI =	0x00000002 (2)
	G_IO_ERR =	0x00000008 (8)
	G_IO_HUP =	0x00000010 (16)
	G_IO_NVAL =	0x00000020 (32)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GIOCondition {
	G_IO_IN =	1 as u32,
	G_IO_OUT =	4 as u32,
	G_IO_PRI =	2 as u32,
	G_IO_ERR =	8 as u32,
	G_IO_HUP =	16 as u32,
	G_IO_NVAL =	32 as u32,
}

impl GIOCondition {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GIOCondition {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_IO_FLAG_APPEND =	0x00000001 (1)
	G_IO_FLAG_NONBLOCK =	0x00000002 (2)
	G_IO_FLAG_IS_READABLE =	0x00000004 (4)
	G_IO_FLAG_IS_WRITABLE =	0x00000008 (8)
	G_IO_FLAG_IS_SEEKABLE =	0x00000010 (16)
	G_IO_FLAG_MASK =	0x0000001F (31)
	G_IO_FLAG_GET_MASK =	0x0000001F (31)
	G_IO_FLAG_SET_MASK =	0x00000003 (3)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GIOFlags {
	G_IO_FLAG_APPEND =	1 as u32,
	G_IO_FLAG_NONBLOCK =	2 as u32,
	G_IO_FLAG_IS_READABLE =	4 as u32,
	G_IO_FLAG_IS_WRITABLE =	8 as u32,
	G_IO_FLAG_IS_SEEKABLE =	16 as u32,
	G_IO_FLAG_MASK =	31 as u32,
	G_IO_FLAG_GET_MASK =	31 as u32,
	G_IO_FLAG_SET_MASK =	3 as u32,
}

impl GIOFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GIOFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_IO_FLAG_APPEND =	0x00000001 (1)
	G_IO_FLAG_NONBLOCK =	0x00000002 (2)
	G_IO_FLAG_IS_READABLE =	0x00000004 (4)
	G_IO_FLAG_IS_WRITABLE =	0x00000008 (8)
	G_IO_FLAG_IS_SEEKABLE =	0x00000010 (16)
	G_IO_FLAG_MASK =	0x0000001F (31)
	G_IO_FLAG_GET_MASK =	0x0000001F (31)
	G_IO_FLAG_SET_MASK =	0x00000003 (3)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GIOFlags {
	G_IO_FLAG_APPEND =	1 as u32,
	G_IO_FLAG_NONBLOCK =	2 as u32,
	G_IO_FLAG_IS_READABLE =	4 as u32,
	G_IO_FLAG_IS_WRITABLE =	8 as u32,
	G_IO_FLAG_IS_SEEKABLE =	16 as u32,
	G_IO_FLAG_MASK =	31 as u32,
	G_IO_FLAG_GET_MASK =	31 as u32,
	G_IO_FLAG_SET_MASK =	3 as u32,
}

impl GIOFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GIOFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_KEY_FILE_ERROR_UNKNOWN_ENCODING =	0x00000000 (0)
	G_KEY_FILE_ERROR_PARSE =	0x00000001 (1)
	G_KEY_FILE_ERROR_NOT_FOUND =	0x00000002 (2)
	G_KEY_FILE_ERROR_KEY_NOT_FOUND =	0x00000003 (3)
	G_KEY_FILE_ERROR_GROUP_NOT_FOUND =	0x00000004 (4)
	G_KEY_FILE_ERROR_INVALID_VALUE =	0x00000005 (5)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GKeyFileError {
	G_KEY_FILE_ERROR_UNKNOWN_ENCODING =	0 as u32,
	G_KEY_FILE_ERROR_PARSE =	1 as u32,
	G_KEY_FILE_ERROR_NOT_FOUND =	2 as u32,
	G_KEY_FILE_ERROR_KEY_NOT_FOUND =	3 as u32,
	G_KEY_FILE_ERROR_GROUP_NOT_FOUND =	4 as u32,
	G_KEY_FILE_ERROR_INVALID_VALUE =	5 as u32,
}

impl GKeyFileError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GKeyFileError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_KEY_FILE_ERROR_UNKNOWN_ENCODING =	0x00000000 (0)
	G_KEY_FILE_ERROR_PARSE =	0x00000001 (1)
	G_KEY_FILE_ERROR_NOT_FOUND =	0x00000002 (2)
	G_KEY_FILE_ERROR_KEY_NOT_FOUND =	0x00000003 (3)
	G_KEY_FILE_ERROR_GROUP_NOT_FOUND =	0x00000004 (4)
	G_KEY_FILE_ERROR_INVALID_VALUE =	0x00000005 (5)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GKeyFileError {
	G_KEY_FILE_ERROR_UNKNOWN_ENCODING =	0 as u32,
	G_KEY_FILE_ERROR_PARSE =	1 as u32,
	G_KEY_FILE_ERROR_NOT_FOUND =	2 as u32,
	G_KEY_FILE_ERROR_KEY_NOT_FOUND =	3 as u32,
	G_KEY_FILE_ERROR_GROUP_NOT_FOUND =	4 as u32,
	G_KEY_FILE_ERROR_INVALID_VALUE =	5 as u32,
}

impl GKeyFileError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GKeyFileError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_KEY_FILE_NONE =	0x00000000 (0)
	G_KEY_FILE_KEEP_COMMENTS =	0x00000001 (1)
	G_KEY_FILE_KEEP_TRANSLATIONS =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GKeyFileFlags {
	G_KEY_FILE_NONE =	0 as u32,
	G_KEY_FILE_KEEP_COMMENTS =	1 as u32,
	G_KEY_FILE_KEEP_TRANSLATIONS =	2 as u32,
}

impl GKeyFileFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GKeyFileFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_KEY_FILE_NONE =	0x00000000 (0)
	G_KEY_FILE_KEEP_COMMENTS =	0x00000001 (1)
	G_KEY_FILE_KEEP_TRANSLATIONS =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GKeyFileFlags {
	G_KEY_FILE_NONE =	0 as u32,
	G_KEY_FILE_KEEP_COMMENTS =	1 as u32,
	G_KEY_FILE_KEEP_TRANSLATIONS =	2 as u32,
}

impl GKeyFileFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GKeyFileFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_MARKUP_ERROR_BAD_UTF8 =	0x00000000 (0)
	G_MARKUP_ERROR_EMPTY =	0x00000001 (1)
	G_MARKUP_ERROR_PARSE =	0x00000002 (2)
	G_MARKUP_ERROR_UNKNOWN_ELEMENT =	0x00000003 (3)
	G_MARKUP_ERROR_UNKNOWN_ATTRIBUTE =	0x00000004 (4)
	G_MARKUP_ERROR_INVALID_CONTENT =	0x00000005 (5)
	G_MARKUP_ERROR_MISSING_ATTRIBUTE =	0x00000006 (6)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GMarkupError {
	G_MARKUP_ERROR_BAD_UTF8 =	0 as u32,
	G_MARKUP_ERROR_EMPTY =	1 as u32,
	G_MARKUP_ERROR_PARSE =	2 as u32,
	G_MARKUP_ERROR_UNKNOWN_ELEMENT =	3 as u32,
	G_MARKUP_ERROR_UNKNOWN_ATTRIBUTE =	4 as u32,
	G_MARKUP_ERROR_INVALID_CONTENT =	5 as u32,
	G_MARKUP_ERROR_MISSING_ATTRIBUTE =	6 as u32,
}

impl GMarkupError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GMarkupError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_MARKUP_ERROR_BAD_UTF8 =	0x00000000 (0)
	G_MARKUP_ERROR_EMPTY =	0x00000001 (1)
	G_MARKUP_ERROR_PARSE =	0x00000002 (2)
	G_MARKUP_ERROR_UNKNOWN_ELEMENT =	0x00000003 (3)
	G_MARKUP_ERROR_UNKNOWN_ATTRIBUTE =	0x00000004 (4)
	G_MARKUP_ERROR_INVALID_CONTENT =	0x00000005 (5)
	G_MARKUP_ERROR_MISSING_ATTRIBUTE =	0x00000006 (6)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GMarkupError {
	G_MARKUP_ERROR_BAD_UTF8 =	0 as u32,
	G_MARKUP_ERROR_EMPTY =	1 as u32,
	G_MARKUP_ERROR_PARSE =	2 as u32,
	G_MARKUP_ERROR_UNKNOWN_ELEMENT =	3 as u32,
	G_MARKUP_ERROR_UNKNOWN_ATTRIBUTE =	4 as u32,
	G_MARKUP_ERROR_INVALID_CONTENT =	5 as u32,
	G_MARKUP_ERROR_MISSING_ATTRIBUTE =	6 as u32,
}

impl GMarkupError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GMarkupError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_MARKUP_DO_NOT_USE_THIS_UNSUPPORTED_FLAG =	0x00000001 (1)
	G_MARKUP_TREAT_CDATA_AS_TEXT =	0x00000002 (2)
	G_MARKUP_PREFIX_ERROR_POSITION =	0x00000004 (4)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GMarkupParseFlags {
	G_MARKUP_DO_NOT_USE_THIS_UNSUPPORTED_FLAG =	1 as u32,
	G_MARKUP_TREAT_CDATA_AS_TEXT =	2 as u32,
	G_MARKUP_PREFIX_ERROR_POSITION =	4 as u32,
}

impl GMarkupParseFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GMarkupParseFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_MARKUP_DO_NOT_USE_THIS_UNSUPPORTED_FLAG =	0x00000001 (1)
	G_MARKUP_TREAT_CDATA_AS_TEXT =	0x00000002 (2)
	G_MARKUP_PREFIX_ERROR_POSITION =	0x00000004 (4)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GMarkupParseFlags {
	G_MARKUP_DO_NOT_USE_THIS_UNSUPPORTED_FLAG =	1 as u32,
	G_MARKUP_TREAT_CDATA_AS_TEXT =	2 as u32,
	G_MARKUP_PREFIX_ERROR_POSITION =	4 as u32,
}

impl GMarkupParseFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GMarkupParseFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_MARKUP_COLLECT_INVALID =	0x00000000 (0)
	G_MARKUP_COLLECT_STRING =	0x00000001 (1)
	G_MARKUP_COLLECT_STRDUP =	0x00000002 (2)
	G_MARKUP_COLLECT_BOOLEAN =	0x00000003 (3)
	G_MARKUP_COLLECT_TRISTATE =	0x00000004 (4)
	G_MARKUP_COLLECT_OPTIONAL =	0x00010000 (65536)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GMarkupCollectType {
	G_MARKUP_COLLECT_INVALID =	0 as u32,
	G_MARKUP_COLLECT_STRING =	1 as u32,
	G_MARKUP_COLLECT_STRDUP =	2 as u32,
	G_MARKUP_COLLECT_BOOLEAN =	3 as u32,
	G_MARKUP_COLLECT_TRISTATE =	4 as u32,
	G_MARKUP_COLLECT_OPTIONAL =	65536 as u32,
}

impl GMarkupCollectType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GMarkupCollectType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_MARKUP_COLLECT_INVALID =	0x00000000 (0)
	G_MARKUP_COLLECT_STRING =	0x00000001 (1)
	G_MARKUP_COLLECT_STRDUP =	0x00000002 (2)
	G_MARKUP_COLLECT_BOOLEAN =	0x00000003 (3)
	G_MARKUP_COLLECT_TRISTATE =	0x00000004 (4)
	G_MARKUP_COLLECT_OPTIONAL =	0x00010000 (65536)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GMarkupCollectType {
	G_MARKUP_COLLECT_INVALID =	0 as u32,
	G_MARKUP_COLLECT_STRING =	1 as u32,
	G_MARKUP_COLLECT_STRDUP =	2 as u32,
	G_MARKUP_COLLECT_BOOLEAN =	3 as u32,
	G_MARKUP_COLLECT_TRISTATE =	4 as u32,
	G_MARKUP_COLLECT_OPTIONAL =	65536 as u32,
}

impl GMarkupCollectType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GMarkupCollectType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_LOG_FLAG_RECURSION =	0x00000001 (1)
	G_LOG_FLAG_FATAL =	0x00000002 (2)
	G_LOG_LEVEL_ERROR =	0x00000004 (4)
	G_LOG_LEVEL_CRITICAL =	0x00000008 (8)
	G_LOG_LEVEL_WARNING =	0x00000010 (16)
	G_LOG_LEVEL_MESSAGE =	0x00000020 (32)
	G_LOG_LEVEL_INFO =	0x00000040 (64)
	G_LOG_LEVEL_DEBUG =	0x00000080 (128)
	G_LOG_LEVEL_MASK =	0x-0000004 (-4)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(i32)]
pub enum GLogLevelFlags {
	G_LOG_FLAG_RECURSION =	1 as i32,
	G_LOG_FLAG_FATAL =	2 as i32,
	G_LOG_LEVEL_ERROR =	4 as i32,
	G_LOG_LEVEL_CRITICAL =	8 as i32,
	G_LOG_LEVEL_WARNING =	16 as i32,
	G_LOG_LEVEL_MESSAGE =	32 as i32,
	G_LOG_LEVEL_INFO =	64 as i32,
	G_LOG_LEVEL_DEBUG =	128 as i32,
	G_LOG_LEVEL_MASK =	-4 as i32,
}

impl GLogLevelFlags {
	pub fn to_i32(&self) -> libc::c_int {
		*self as libc::c_int
	}

	pub fn from_i32(v: libc::c_int) -> GLogLevelFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_LOG_FLAG_RECURSION =	0x00000001 (1)
	G_LOG_FLAG_FATAL =	0x00000002 (2)
	G_LOG_LEVEL_ERROR =	0x00000004 (4)
	G_LOG_LEVEL_CRITICAL =	0x00000008 (8)
	G_LOG_LEVEL_WARNING =	0x00000010 (16)
	G_LOG_LEVEL_MESSAGE =	0x00000020 (32)
	G_LOG_LEVEL_INFO =	0x00000040 (64)
	G_LOG_LEVEL_DEBUG =	0x00000080 (128)
	G_LOG_LEVEL_MASK =	0x-0000004 (-4)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(i32)]
pub enum GLogLevelFlags {
	G_LOG_FLAG_RECURSION =	1 as i32,
	G_LOG_FLAG_FATAL =	2 as i32,
	G_LOG_LEVEL_ERROR =	4 as i32,
	G_LOG_LEVEL_CRITICAL =	8 as i32,
	G_LOG_LEVEL_WARNING =	16 as i32,
	G_LOG_LEVEL_MESSAGE =	32 as i32,
	G_LOG_LEVEL_INFO =	64 as i32,
	G_LOG_LEVEL_DEBUG =	128 as i32,
	G_LOG_LEVEL_MASK =	-4 as i32,
}

impl GLogLevelFlags {
	pub fn to_i32(&self) -> libc::c_int {
		*self as libc::c_int
	}

	pub fn from_i32(v: libc::c_int) -> GLogLevelFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_TRAVERSE_LEAVES =	0x00000001 (1)
	G_TRAVERSE_NON_LEAVES =	0x00000002 (2)
	G_TRAVERSE_ALL =	0x00000003 (3)
	G_TRAVERSE_MASK =	0x00000003 (3)
	G_TRAVERSE_LEAFS =	0x00000001 (1)
	G_TRAVERSE_NON_LEAFS =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GTraverseFlags {
	G_TRAVERSE_LEAVES =	1 as u32,
	G_TRAVERSE_NON_LEAVES =	2 as u32,
	G_TRAVERSE_ALL =	3 as u32,
	G_TRAVERSE_MASK =	3 as u32,
	G_TRAVERSE_LEAFS =	1 as u32,
	G_TRAVERSE_NON_LEAFS =	2 as u32,
}

impl GTraverseFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GTraverseFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_TRAVERSE_LEAVES =	0x00000001 (1)
	G_TRAVERSE_NON_LEAVES =	0x00000002 (2)
	G_TRAVERSE_ALL =	0x00000003 (3)
	G_TRAVERSE_MASK =	0x00000003 (3)
	G_TRAVERSE_LEAFS =	0x00000001 (1)
	G_TRAVERSE_NON_LEAFS =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GTraverseFlags {
	G_TRAVERSE_LEAVES =	1 as u32,
	G_TRAVERSE_NON_LEAVES =	2 as u32,
	G_TRAVERSE_ALL =	3 as u32,
	G_TRAVERSE_MASK =	3 as u32,
	G_TRAVERSE_LEAFS =	1 as u32,
	G_TRAVERSE_NON_LEAFS =	2 as u32,
}

impl GTraverseFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GTraverseFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_IN_ORDER =	0x00000000 (0)
	G_PRE_ORDER =	0x00000001 (1)
	G_POST_ORDER =	0x00000002 (2)
	G_LEVEL_ORDER =	0x00000003 (3)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GTraverseType {
	G_IN_ORDER =	0 as u32,
	G_PRE_ORDER =	1 as u32,
	G_POST_ORDER =	2 as u32,
	G_LEVEL_ORDER =	3 as u32,
}

impl GTraverseType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GTraverseType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_IN_ORDER =	0x00000000 (0)
	G_PRE_ORDER =	0x00000001 (1)
	G_POST_ORDER =	0x00000002 (2)
	G_LEVEL_ORDER =	0x00000003 (3)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GTraverseType {
	G_IN_ORDER =	0 as u32,
	G_PRE_ORDER =	1 as u32,
	G_POST_ORDER =	2 as u32,
	G_LEVEL_ORDER =	3 as u32,
}

impl GTraverseType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GTraverseType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_OPTION_FLAG_HIDDEN =	0x00000001 (1)
	G_OPTION_FLAG_IN_MAIN =	0x00000002 (2)
	G_OPTION_FLAG_REVERSE =	0x00000004 (4)
	G_OPTION_FLAG_NO_ARG =	0x00000008 (8)
	G_OPTION_FLAG_FILENAME =	0x00000010 (16)
	G_OPTION_FLAG_OPTIONAL_ARG =	0x00000020 (32)
	G_OPTION_FLAG_NOALIAS =	0x00000040 (64)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GOptionFlags {
	G_OPTION_FLAG_HIDDEN =	1 as u32,
	G_OPTION_FLAG_IN_MAIN =	2 as u32,
	G_OPTION_FLAG_REVERSE =	4 as u32,
	G_OPTION_FLAG_NO_ARG =	8 as u32,
	G_OPTION_FLAG_FILENAME =	16 as u32,
	G_OPTION_FLAG_OPTIONAL_ARG =	32 as u32,
	G_OPTION_FLAG_NOALIAS =	64 as u32,
}

impl GOptionFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GOptionFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_OPTION_FLAG_HIDDEN =	0x00000001 (1)
	G_OPTION_FLAG_IN_MAIN =	0x00000002 (2)
	G_OPTION_FLAG_REVERSE =	0x00000004 (4)
	G_OPTION_FLAG_NO_ARG =	0x00000008 (8)
	G_OPTION_FLAG_FILENAME =	0x00000010 (16)
	G_OPTION_FLAG_OPTIONAL_ARG =	0x00000020 (32)
	G_OPTION_FLAG_NOALIAS =	0x00000040 (64)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GOptionFlags {
	G_OPTION_FLAG_HIDDEN =	1 as u32,
	G_OPTION_FLAG_IN_MAIN =	2 as u32,
	G_OPTION_FLAG_REVERSE =	4 as u32,
	G_OPTION_FLAG_NO_ARG =	8 as u32,
	G_OPTION_FLAG_FILENAME =	16 as u32,
	G_OPTION_FLAG_OPTIONAL_ARG =	32 as u32,
	G_OPTION_FLAG_NOALIAS =	64 as u32,
}

impl GOptionFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GOptionFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_OPTION_ARG_NONE =	0x00000000 (0)
	G_OPTION_ARG_STRING =	0x00000001 (1)
	G_OPTION_ARG_INT =	0x00000002 (2)
	G_OPTION_ARG_CALLBACK =	0x00000003 (3)
	G_OPTION_ARG_FILENAME =	0x00000004 (4)
	G_OPTION_ARG_STRING_ARRAY =	0x00000005 (5)
	G_OPTION_ARG_FILENAME_ARRAY =	0x00000006 (6)
	G_OPTION_ARG_DOUBLE =	0x00000007 (7)
	G_OPTION_ARG_INT64 =	0x00000008 (8)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GOptionArg {
	G_OPTION_ARG_NONE =	0 as u32,
	G_OPTION_ARG_STRING =	1 as u32,
	G_OPTION_ARG_INT =	2 as u32,
	G_OPTION_ARG_CALLBACK =	3 as u32,
	G_OPTION_ARG_FILENAME =	4 as u32,
	G_OPTION_ARG_STRING_ARRAY =	5 as u32,
	G_OPTION_ARG_FILENAME_ARRAY =	6 as u32,
	G_OPTION_ARG_DOUBLE =	7 as u32,
	G_OPTION_ARG_INT64 =	8 as u32,
}

impl GOptionArg {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GOptionArg {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_OPTION_ARG_NONE =	0x00000000 (0)
	G_OPTION_ARG_STRING =	0x00000001 (1)
	G_OPTION_ARG_INT =	0x00000002 (2)
	G_OPTION_ARG_CALLBACK =	0x00000003 (3)
	G_OPTION_ARG_FILENAME =	0x00000004 (4)
	G_OPTION_ARG_STRING_ARRAY =	0x00000005 (5)
	G_OPTION_ARG_FILENAME_ARRAY =	0x00000006 (6)
	G_OPTION_ARG_DOUBLE =	0x00000007 (7)
	G_OPTION_ARG_INT64 =	0x00000008 (8)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GOptionArg {
	G_OPTION_ARG_NONE =	0 as u32,
	G_OPTION_ARG_STRING =	1 as u32,
	G_OPTION_ARG_INT =	2 as u32,
	G_OPTION_ARG_CALLBACK =	3 as u32,
	G_OPTION_ARG_FILENAME =	4 as u32,
	G_OPTION_ARG_STRING_ARRAY =	5 as u32,
	G_OPTION_ARG_FILENAME_ARRAY =	6 as u32,
	G_OPTION_ARG_DOUBLE =	7 as u32,
	G_OPTION_ARG_INT64 =	8 as u32,
}

impl GOptionArg {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GOptionArg {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_OPTION_ERROR_UNKNOWN_OPTION =	0x00000000 (0)
	G_OPTION_ERROR_BAD_VALUE =	0x00000001 (1)
	G_OPTION_ERROR_FAILED =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GOptionError {
	G_OPTION_ERROR_UNKNOWN_OPTION =	0 as u32,
	G_OPTION_ERROR_BAD_VALUE =	1 as u32,
	G_OPTION_ERROR_FAILED =	2 as u32,
}

impl GOptionError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GOptionError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_OPTION_ERROR_UNKNOWN_OPTION =	0x00000000 (0)
	G_OPTION_ERROR_BAD_VALUE =	0x00000001 (1)
	G_OPTION_ERROR_FAILED =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GOptionError {
	G_OPTION_ERROR_UNKNOWN_OPTION =	0 as u32,
	G_OPTION_ERROR_BAD_VALUE =	1 as u32,
	G_OPTION_ERROR_FAILED =	2 as u32,
}

impl GOptionError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GOptionError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_REGEX_ERROR_COMPILE =	0x00000000 (0)
	G_REGEX_ERROR_OPTIMIZE =	0x00000001 (1)
	G_REGEX_ERROR_REPLACE =	0x00000002 (2)
	G_REGEX_ERROR_MATCH =	0x00000003 (3)
	G_REGEX_ERROR_INTERNAL =	0x00000004 (4)
	G_REGEX_ERROR_STRAY_BACKSLASH =	0x00000065 (101)
	G_REGEX_ERROR_MISSING_CONTROL_CHAR =	0x00000066 (102)
	G_REGEX_ERROR_UNRECOGNIZED_ESCAPE =	0x00000067 (103)
	G_REGEX_ERROR_QUANTIFIERS_OUT_OF_ORDER =	0x00000068 (104)
	G_REGEX_ERROR_QUANTIFIER_TOO_BIG =	0x00000069 (105)
	G_REGEX_ERROR_UNTERMINATED_CHARACTER_CLASS =	0x0000006A (106)
	G_REGEX_ERROR_INVALID_ESCAPE_IN_CHARACTER_CLASS =	0x0000006B (107)
	G_REGEX_ERROR_RANGE_OUT_OF_ORDER =	0x0000006C (108)
	G_REGEX_ERROR_NOTHING_TO_REPEAT =	0x0000006D (109)
	G_REGEX_ERROR_UNRECOGNIZED_CHARACTER =	0x00000070 (112)
	G_REGEX_ERROR_POSIX_NAMED_CLASS_OUTSIDE_CLASS =	0x00000071 (113)
	G_REGEX_ERROR_UNMATCHED_PARENTHESIS =	0x00000072 (114)
	G_REGEX_ERROR_INEXISTENT_SUBPATTERN_REFERENCE =	0x00000073 (115)
	G_REGEX_ERROR_UNTERMINATED_COMMENT =	0x00000076 (118)
	G_REGEX_ERROR_EXPRESSION_TOO_LARGE =	0x00000078 (120)
	G_REGEX_ERROR_MEMORY_ERROR =	0x00000079 (121)
	G_REGEX_ERROR_VARIABLE_LENGTH_LOOKBEHIND =	0x0000007D (125)
	G_REGEX_ERROR_MALFORMED_CONDITION =	0x0000007E (126)
	G_REGEX_ERROR_TOO_MANY_CONDITIONAL_BRANCHES =	0x0000007F (127)
	G_REGEX_ERROR_ASSERTION_EXPECTED =	0x00000080 (128)
	G_REGEX_ERROR_UNKNOWN_POSIX_CLASS_NAME =	0x00000082 (130)
	G_REGEX_ERROR_POSIX_COLLATING_ELEMENTS_NOT_SUPPORTED =	0x00000083 (131)
	G_REGEX_ERROR_HEX_CODE_TOO_LARGE =	0x00000086 (134)
	G_REGEX_ERROR_INVALID_CONDITION =	0x00000087 (135)
	G_REGEX_ERROR_SINGLE_BYTE_MATCH_IN_LOOKBEHIND =	0x00000088 (136)
	G_REGEX_ERROR_INFINITE_LOOP =	0x0000008C (140)
	G_REGEX_ERROR_MISSING_SUBPATTERN_NAME_TERMINATOR =	0x0000008E (142)
	G_REGEX_ERROR_DUPLICATE_SUBPATTERN_NAME =	0x0000008F (143)
	G_REGEX_ERROR_MALFORMED_PROPERTY =	0x00000092 (146)
	G_REGEX_ERROR_UNKNOWN_PROPERTY =	0x00000093 (147)
	G_REGEX_ERROR_SUBPATTERN_NAME_TOO_LONG =	0x00000094 (148)
	G_REGEX_ERROR_TOO_MANY_SUBPATTERNS =	0x00000095 (149)
	G_REGEX_ERROR_INVALID_OCTAL_VALUE =	0x00000097 (151)
	G_REGEX_ERROR_TOO_MANY_BRANCHES_IN_DEFINE =	0x0000009A (154)
	G_REGEX_ERROR_DEFINE_REPETION =	0x0000009B (155)
	G_REGEX_ERROR_INCONSISTENT_NEWLINE_OPTIONS =	0x0000009C (156)
	G_REGEX_ERROR_MISSING_BACK_REFERENCE =	0x0000009D (157)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GRegexError {
	G_REGEX_ERROR_COMPILE =	0 as u32,
	G_REGEX_ERROR_OPTIMIZE =	1 as u32,
	G_REGEX_ERROR_REPLACE =	2 as u32,
	G_REGEX_ERROR_MATCH =	3 as u32,
	G_REGEX_ERROR_INTERNAL =	4 as u32,
	G_REGEX_ERROR_STRAY_BACKSLASH =	101 as u32,
	G_REGEX_ERROR_MISSING_CONTROL_CHAR =	102 as u32,
	G_REGEX_ERROR_UNRECOGNIZED_ESCAPE =	103 as u32,
	G_REGEX_ERROR_QUANTIFIERS_OUT_OF_ORDER =	104 as u32,
	G_REGEX_ERROR_QUANTIFIER_TOO_BIG =	105 as u32,
	G_REGEX_ERROR_UNTERMINATED_CHARACTER_CLASS =	106 as u32,
	G_REGEX_ERROR_INVALID_ESCAPE_IN_CHARACTER_CLASS =	107 as u32,
	G_REGEX_ERROR_RANGE_OUT_OF_ORDER =	108 as u32,
	G_REGEX_ERROR_NOTHING_TO_REPEAT =	109 as u32,
	G_REGEX_ERROR_UNRECOGNIZED_CHARACTER =	112 as u32,
	G_REGEX_ERROR_POSIX_NAMED_CLASS_OUTSIDE_CLASS =	113 as u32,
	G_REGEX_ERROR_UNMATCHED_PARENTHESIS =	114 as u32,
	G_REGEX_ERROR_INEXISTENT_SUBPATTERN_REFERENCE =	115 as u32,
	G_REGEX_ERROR_UNTERMINATED_COMMENT =	118 as u32,
	G_REGEX_ERROR_EXPRESSION_TOO_LARGE =	120 as u32,
	G_REGEX_ERROR_MEMORY_ERROR =	121 as u32,
	G_REGEX_ERROR_VARIABLE_LENGTH_LOOKBEHIND =	125 as u32,
	G_REGEX_ERROR_MALFORMED_CONDITION =	126 as u32,
	G_REGEX_ERROR_TOO_MANY_CONDITIONAL_BRANCHES =	127 as u32,
	G_REGEX_ERROR_ASSERTION_EXPECTED =	128 as u32,
	G_REGEX_ERROR_UNKNOWN_POSIX_CLASS_NAME =	130 as u32,
	G_REGEX_ERROR_POSIX_COLLATING_ELEMENTS_NOT_SUPPORTED =	131 as u32,
	G_REGEX_ERROR_HEX_CODE_TOO_LARGE =	134 as u32,
	G_REGEX_ERROR_INVALID_CONDITION =	135 as u32,
	G_REGEX_ERROR_SINGLE_BYTE_MATCH_IN_LOOKBEHIND =	136 as u32,
	G_REGEX_ERROR_INFINITE_LOOP =	140 as u32,
	G_REGEX_ERROR_MISSING_SUBPATTERN_NAME_TERMINATOR =	142 as u32,
	G_REGEX_ERROR_DUPLICATE_SUBPATTERN_NAME =	143 as u32,
	G_REGEX_ERROR_MALFORMED_PROPERTY =	146 as u32,
	G_REGEX_ERROR_UNKNOWN_PROPERTY =	147 as u32,
	G_REGEX_ERROR_SUBPATTERN_NAME_TOO_LONG =	148 as u32,
	G_REGEX_ERROR_TOO_MANY_SUBPATTERNS =	149 as u32,
	G_REGEX_ERROR_INVALID_OCTAL_VALUE =	151 as u32,
	G_REGEX_ERROR_TOO_MANY_BRANCHES_IN_DEFINE =	154 as u32,
	G_REGEX_ERROR_DEFINE_REPETION =	155 as u32,
	G_REGEX_ERROR_INCONSISTENT_NEWLINE_OPTIONS =	156 as u32,
	G_REGEX_ERROR_MISSING_BACK_REFERENCE =	157 as u32,
}

impl GRegexError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GRegexError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_REGEX_ERROR_COMPILE =	0x00000000 (0)
	G_REGEX_ERROR_OPTIMIZE =	0x00000001 (1)
	G_REGEX_ERROR_REPLACE =	0x00000002 (2)
	G_REGEX_ERROR_MATCH =	0x00000003 (3)
	G_REGEX_ERROR_INTERNAL =	0x00000004 (4)
	G_REGEX_ERROR_STRAY_BACKSLASH =	0x00000065 (101)
	G_REGEX_ERROR_MISSING_CONTROL_CHAR =	0x00000066 (102)
	G_REGEX_ERROR_UNRECOGNIZED_ESCAPE =	0x00000067 (103)
	G_REGEX_ERROR_QUANTIFIERS_OUT_OF_ORDER =	0x00000068 (104)
	G_REGEX_ERROR_QUANTIFIER_TOO_BIG =	0x00000069 (105)
	G_REGEX_ERROR_UNTERMINATED_CHARACTER_CLASS =	0x0000006A (106)
	G_REGEX_ERROR_INVALID_ESCAPE_IN_CHARACTER_CLASS =	0x0000006B (107)
	G_REGEX_ERROR_RANGE_OUT_OF_ORDER =	0x0000006C (108)
	G_REGEX_ERROR_NOTHING_TO_REPEAT =	0x0000006D (109)
	G_REGEX_ERROR_UNRECOGNIZED_CHARACTER =	0x00000070 (112)
	G_REGEX_ERROR_POSIX_NAMED_CLASS_OUTSIDE_CLASS =	0x00000071 (113)
	G_REGEX_ERROR_UNMATCHED_PARENTHESIS =	0x00000072 (114)
	G_REGEX_ERROR_INEXISTENT_SUBPATTERN_REFERENCE =	0x00000073 (115)
	G_REGEX_ERROR_UNTERMINATED_COMMENT =	0x00000076 (118)
	G_REGEX_ERROR_EXPRESSION_TOO_LARGE =	0x00000078 (120)
	G_REGEX_ERROR_MEMORY_ERROR =	0x00000079 (121)
	G_REGEX_ERROR_VARIABLE_LENGTH_LOOKBEHIND =	0x0000007D (125)
	G_REGEX_ERROR_MALFORMED_CONDITION =	0x0000007E (126)
	G_REGEX_ERROR_TOO_MANY_CONDITIONAL_BRANCHES =	0x0000007F (127)
	G_REGEX_ERROR_ASSERTION_EXPECTED =	0x00000080 (128)
	G_REGEX_ERROR_UNKNOWN_POSIX_CLASS_NAME =	0x00000082 (130)
	G_REGEX_ERROR_POSIX_COLLATING_ELEMENTS_NOT_SUPPORTED =	0x00000083 (131)
	G_REGEX_ERROR_HEX_CODE_TOO_LARGE =	0x00000086 (134)
	G_REGEX_ERROR_INVALID_CONDITION =	0x00000087 (135)
	G_REGEX_ERROR_SINGLE_BYTE_MATCH_IN_LOOKBEHIND =	0x00000088 (136)
	G_REGEX_ERROR_INFINITE_LOOP =	0x0000008C (140)
	G_REGEX_ERROR_MISSING_SUBPATTERN_NAME_TERMINATOR =	0x0000008E (142)
	G_REGEX_ERROR_DUPLICATE_SUBPATTERN_NAME =	0x0000008F (143)
	G_REGEX_ERROR_MALFORMED_PROPERTY =	0x00000092 (146)
	G_REGEX_ERROR_UNKNOWN_PROPERTY =	0x00000093 (147)
	G_REGEX_ERROR_SUBPATTERN_NAME_TOO_LONG =	0x00000094 (148)
	G_REGEX_ERROR_TOO_MANY_SUBPATTERNS =	0x00000095 (149)
	G_REGEX_ERROR_INVALID_OCTAL_VALUE =	0x00000097 (151)
	G_REGEX_ERROR_TOO_MANY_BRANCHES_IN_DEFINE =	0x0000009A (154)
	G_REGEX_ERROR_DEFINE_REPETION =	0x0000009B (155)
	G_REGEX_ERROR_INCONSISTENT_NEWLINE_OPTIONS =	0x0000009C (156)
	G_REGEX_ERROR_MISSING_BACK_REFERENCE =	0x0000009D (157)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GRegexError {
	G_REGEX_ERROR_COMPILE =	0 as u32,
	G_REGEX_ERROR_OPTIMIZE =	1 as u32,
	G_REGEX_ERROR_REPLACE =	2 as u32,
	G_REGEX_ERROR_MATCH =	3 as u32,
	G_REGEX_ERROR_INTERNAL =	4 as u32,
	G_REGEX_ERROR_STRAY_BACKSLASH =	101 as u32,
	G_REGEX_ERROR_MISSING_CONTROL_CHAR =	102 as u32,
	G_REGEX_ERROR_UNRECOGNIZED_ESCAPE =	103 as u32,
	G_REGEX_ERROR_QUANTIFIERS_OUT_OF_ORDER =	104 as u32,
	G_REGEX_ERROR_QUANTIFIER_TOO_BIG =	105 as u32,
	G_REGEX_ERROR_UNTERMINATED_CHARACTER_CLASS =	106 as u32,
	G_REGEX_ERROR_INVALID_ESCAPE_IN_CHARACTER_CLASS =	107 as u32,
	G_REGEX_ERROR_RANGE_OUT_OF_ORDER =	108 as u32,
	G_REGEX_ERROR_NOTHING_TO_REPEAT =	109 as u32,
	G_REGEX_ERROR_UNRECOGNIZED_CHARACTER =	112 as u32,
	G_REGEX_ERROR_POSIX_NAMED_CLASS_OUTSIDE_CLASS =	113 as u32,
	G_REGEX_ERROR_UNMATCHED_PARENTHESIS =	114 as u32,
	G_REGEX_ERROR_INEXISTENT_SUBPATTERN_REFERENCE =	115 as u32,
	G_REGEX_ERROR_UNTERMINATED_COMMENT =	118 as u32,
	G_REGEX_ERROR_EXPRESSION_TOO_LARGE =	120 as u32,
	G_REGEX_ERROR_MEMORY_ERROR =	121 as u32,
	G_REGEX_ERROR_VARIABLE_LENGTH_LOOKBEHIND =	125 as u32,
	G_REGEX_ERROR_MALFORMED_CONDITION =	126 as u32,
	G_REGEX_ERROR_TOO_MANY_CONDITIONAL_BRANCHES =	127 as u32,
	G_REGEX_ERROR_ASSERTION_EXPECTED =	128 as u32,
	G_REGEX_ERROR_UNKNOWN_POSIX_CLASS_NAME =	130 as u32,
	G_REGEX_ERROR_POSIX_COLLATING_ELEMENTS_NOT_SUPPORTED =	131 as u32,
	G_REGEX_ERROR_HEX_CODE_TOO_LARGE =	134 as u32,
	G_REGEX_ERROR_INVALID_CONDITION =	135 as u32,
	G_REGEX_ERROR_SINGLE_BYTE_MATCH_IN_LOOKBEHIND =	136 as u32,
	G_REGEX_ERROR_INFINITE_LOOP =	140 as u32,
	G_REGEX_ERROR_MISSING_SUBPATTERN_NAME_TERMINATOR =	142 as u32,
	G_REGEX_ERROR_DUPLICATE_SUBPATTERN_NAME =	143 as u32,
	G_REGEX_ERROR_MALFORMED_PROPERTY =	146 as u32,
	G_REGEX_ERROR_UNKNOWN_PROPERTY =	147 as u32,
	G_REGEX_ERROR_SUBPATTERN_NAME_TOO_LONG =	148 as u32,
	G_REGEX_ERROR_TOO_MANY_SUBPATTERNS =	149 as u32,
	G_REGEX_ERROR_INVALID_OCTAL_VALUE =	151 as u32,
	G_REGEX_ERROR_TOO_MANY_BRANCHES_IN_DEFINE =	154 as u32,
	G_REGEX_ERROR_DEFINE_REPETION =	155 as u32,
	G_REGEX_ERROR_INCONSISTENT_NEWLINE_OPTIONS =	156 as u32,
	G_REGEX_ERROR_MISSING_BACK_REFERENCE =	157 as u32,
}

impl GRegexError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GRegexError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_REGEX_CASELESS =	0x00000001 (1)
	G_REGEX_MULTILINE =	0x00000002 (2)
	G_REGEX_DOTALL =	0x00000004 (4)
	G_REGEX_EXTENDED =	0x00000008 (8)
	G_REGEX_ANCHORED =	0x00000010 (16)
	G_REGEX_DOLLAR_ENDONLY =	0x00000020 (32)
	G_REGEX_UNGREEDY =	0x00000200 (512)
	G_REGEX_RAW =	0x00000800 (2048)
	G_REGEX_NO_AUTO_CAPTURE =	0x00001000 (4096)
	G_REGEX_OPTIMIZE =	0x00002000 (8192)
	G_REGEX_DUPNAMES =	0x00080000 (524288)
	G_REGEX_NEWLINE_CR =	0x00100000 (1048576)
	G_REGEX_NEWLINE_LF =	0x00200000 (2097152)
	G_REGEX_NEWLINE_CRLF =	0x00300000 (3145728)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GRegexCompileFlags {
	G_REGEX_CASELESS =	1 as u32,
	G_REGEX_MULTILINE =	2 as u32,
	G_REGEX_DOTALL =	4 as u32,
	G_REGEX_EXTENDED =	8 as u32,
	G_REGEX_ANCHORED =	16 as u32,
	G_REGEX_DOLLAR_ENDONLY =	32 as u32,
	G_REGEX_UNGREEDY =	512 as u32,
	G_REGEX_RAW =	2048 as u32,
	G_REGEX_NO_AUTO_CAPTURE =	4096 as u32,
	G_REGEX_OPTIMIZE =	8192 as u32,
	G_REGEX_DUPNAMES =	524288 as u32,
	G_REGEX_NEWLINE_CR =	1048576 as u32,
	G_REGEX_NEWLINE_LF =	2097152 as u32,
	G_REGEX_NEWLINE_CRLF =	3145728 as u32,
}

impl GRegexCompileFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GRegexCompileFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_REGEX_CASELESS =	0x00000001 (1)
	G_REGEX_MULTILINE =	0x00000002 (2)
	G_REGEX_DOTALL =	0x00000004 (4)
	G_REGEX_EXTENDED =	0x00000008 (8)
	G_REGEX_ANCHORED =	0x00000010 (16)
	G_REGEX_DOLLAR_ENDONLY =	0x00000020 (32)
	G_REGEX_UNGREEDY =	0x00000200 (512)
	G_REGEX_RAW =	0x00000800 (2048)
	G_REGEX_NO_AUTO_CAPTURE =	0x00001000 (4096)
	G_REGEX_OPTIMIZE =	0x00002000 (8192)
	G_REGEX_DUPNAMES =	0x00080000 (524288)
	G_REGEX_NEWLINE_CR =	0x00100000 (1048576)
	G_REGEX_NEWLINE_LF =	0x00200000 (2097152)
	G_REGEX_NEWLINE_CRLF =	0x00300000 (3145728)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GRegexCompileFlags {
	G_REGEX_CASELESS =	1 as u32,
	G_REGEX_MULTILINE =	2 as u32,
	G_REGEX_DOTALL =	4 as u32,
	G_REGEX_EXTENDED =	8 as u32,
	G_REGEX_ANCHORED =	16 as u32,
	G_REGEX_DOLLAR_ENDONLY =	32 as u32,
	G_REGEX_UNGREEDY =	512 as u32,
	G_REGEX_RAW =	2048 as u32,
	G_REGEX_NO_AUTO_CAPTURE =	4096 as u32,
	G_REGEX_OPTIMIZE =	8192 as u32,
	G_REGEX_DUPNAMES =	524288 as u32,
	G_REGEX_NEWLINE_CR =	1048576 as u32,
	G_REGEX_NEWLINE_LF =	2097152 as u32,
	G_REGEX_NEWLINE_CRLF =	3145728 as u32,
}

impl GRegexCompileFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GRegexCompileFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_REGEX_MATCH_ANCHORED =	0x00000010 (16)
	G_REGEX_MATCH_NOTBOL =	0x00000080 (128)
	G_REGEX_MATCH_NOTEOL =	0x00000100 (256)
	G_REGEX_MATCH_NOTEMPTY =	0x00000400 (1024)
	G_REGEX_MATCH_PARTIAL =	0x00008000 (32768)
	G_REGEX_MATCH_NEWLINE_CR =	0x00100000 (1048576)
	G_REGEX_MATCH_NEWLINE_LF =	0x00200000 (2097152)
	G_REGEX_MATCH_NEWLINE_CRLF =	0x00300000 (3145728)
	G_REGEX_MATCH_NEWLINE_ANY =	0x00400000 (4194304)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GRegexMatchFlags {
	G_REGEX_MATCH_ANCHORED =	16 as u32,
	G_REGEX_MATCH_NOTBOL =	128 as u32,
	G_REGEX_MATCH_NOTEOL =	256 as u32,
	G_REGEX_MATCH_NOTEMPTY =	1024 as u32,
	G_REGEX_MATCH_PARTIAL =	32768 as u32,
	G_REGEX_MATCH_NEWLINE_CR =	1048576 as u32,
	G_REGEX_MATCH_NEWLINE_LF =	2097152 as u32,
	G_REGEX_MATCH_NEWLINE_CRLF =	3145728 as u32,
	G_REGEX_MATCH_NEWLINE_ANY =	4194304 as u32,
}

impl GRegexMatchFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GRegexMatchFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_REGEX_MATCH_ANCHORED =	0x00000010 (16)
	G_REGEX_MATCH_NOTBOL =	0x00000080 (128)
	G_REGEX_MATCH_NOTEOL =	0x00000100 (256)
	G_REGEX_MATCH_NOTEMPTY =	0x00000400 (1024)
	G_REGEX_MATCH_PARTIAL =	0x00008000 (32768)
	G_REGEX_MATCH_NEWLINE_CR =	0x00100000 (1048576)
	G_REGEX_MATCH_NEWLINE_LF =	0x00200000 (2097152)
	G_REGEX_MATCH_NEWLINE_CRLF =	0x00300000 (3145728)
	G_REGEX_MATCH_NEWLINE_ANY =	0x00400000 (4194304)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GRegexMatchFlags {
	G_REGEX_MATCH_ANCHORED =	16 as u32,
	G_REGEX_MATCH_NOTBOL =	128 as u32,
	G_REGEX_MATCH_NOTEOL =	256 as u32,
	G_REGEX_MATCH_NOTEMPTY =	1024 as u32,
	G_REGEX_MATCH_PARTIAL =	32768 as u32,
	G_REGEX_MATCH_NEWLINE_CR =	1048576 as u32,
	G_REGEX_MATCH_NEWLINE_LF =	2097152 as u32,
	G_REGEX_MATCH_NEWLINE_CRLF =	3145728 as u32,
	G_REGEX_MATCH_NEWLINE_ANY =	4194304 as u32,
}

impl GRegexMatchFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GRegexMatchFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_ERR_UNKNOWN =	0x00000000 (0)
	G_ERR_UNEXP_EOF =	0x00000001 (1)
	G_ERR_UNEXP_EOF_IN_STRING =	0x00000002 (2)
	G_ERR_UNEXP_EOF_IN_COMMENT =	0x00000003 (3)
	G_ERR_NON_DIGIT_IN_CONST =	0x00000004 (4)
	G_ERR_DIGIT_RADIX =	0x00000005 (5)
	G_ERR_FLOAT_RADIX =	0x00000006 (6)
	G_ERR_FLOAT_MALFORMED =	0x00000007 (7)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GErrorType {
	G_ERR_UNKNOWN =	0 as u32,
	G_ERR_UNEXP_EOF =	1 as u32,
	G_ERR_UNEXP_EOF_IN_STRING =	2 as u32,
	G_ERR_UNEXP_EOF_IN_COMMENT =	3 as u32,
	G_ERR_NON_DIGIT_IN_CONST =	4 as u32,
	G_ERR_DIGIT_RADIX =	5 as u32,
	G_ERR_FLOAT_RADIX =	6 as u32,
	G_ERR_FLOAT_MALFORMED =	7 as u32,
}

impl GErrorType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GErrorType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_ERR_UNKNOWN =	0x00000000 (0)
	G_ERR_UNEXP_EOF =	0x00000001 (1)
	G_ERR_UNEXP_EOF_IN_STRING =	0x00000002 (2)
	G_ERR_UNEXP_EOF_IN_COMMENT =	0x00000003 (3)
	G_ERR_NON_DIGIT_IN_CONST =	0x00000004 (4)
	G_ERR_DIGIT_RADIX =	0x00000005 (5)
	G_ERR_FLOAT_RADIX =	0x00000006 (6)
	G_ERR_FLOAT_MALFORMED =	0x00000007 (7)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GErrorType {
	G_ERR_UNKNOWN =	0 as u32,
	G_ERR_UNEXP_EOF =	1 as u32,
	G_ERR_UNEXP_EOF_IN_STRING =	2 as u32,
	G_ERR_UNEXP_EOF_IN_COMMENT =	3 as u32,
	G_ERR_NON_DIGIT_IN_CONST =	4 as u32,
	G_ERR_DIGIT_RADIX =	5 as u32,
	G_ERR_FLOAT_RADIX =	6 as u32,
	G_ERR_FLOAT_MALFORMED =	7 as u32,
}

impl GErrorType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GErrorType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_TOKEN_EOF =	0x00000000 (0)
	G_TOKEN_LEFT_PAREN =	0x00000028 (40)
	G_TOKEN_RIGHT_PAREN =	0x00000029 (41)
	G_TOKEN_LEFT_CURLY =	0x0000007B (123)
	G_TOKEN_RIGHT_CURLY =	0x0000007D (125)
	G_TOKEN_LEFT_BRACE =	0x0000005B (91)
	G_TOKEN_RIGHT_BRACE =	0x0000005D (93)
	G_TOKEN_EQUAL_SIGN =	0x0000003D (61)
	G_TOKEN_COMMA =	0x0000002C (44)
	G_TOKEN_NONE =	0x00000100 (256)
	G_TOKEN_ERROR =	0x00000101 (257)
	G_TOKEN_CHAR =	0x00000102 (258)
	G_TOKEN_BINARY =	0x00000103 (259)
	G_TOKEN_OCTAL =	0x00000104 (260)
	G_TOKEN_INT =	0x00000105 (261)
	G_TOKEN_HEX =	0x00000106 (262)
	G_TOKEN_FLOAT =	0x00000107 (263)
	G_TOKEN_STRING =	0x00000108 (264)
	G_TOKEN_SYMBOL =	0x00000109 (265)
	G_TOKEN_IDENTIFIER =	0x0000010A (266)
	G_TOKEN_IDENTIFIER_NULL =	0x0000010B (267)
	G_TOKEN_COMMENT_SINGLE =	0x0000010C (268)
	G_TOKEN_COMMENT_MULTI =	0x0000010D (269)
	G_TOKEN_LAST =	0x0000010E (270)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GTokenType {
	G_TOKEN_EOF =	0 as u32,
	G_TOKEN_LEFT_PAREN =	40 as u32,
	G_TOKEN_RIGHT_PAREN =	41 as u32,
	G_TOKEN_LEFT_CURLY =	123 as u32,
	G_TOKEN_RIGHT_CURLY =	125 as u32,
	G_TOKEN_LEFT_BRACE =	91 as u32,
	G_TOKEN_RIGHT_BRACE =	93 as u32,
	G_TOKEN_EQUAL_SIGN =	61 as u32,
	G_TOKEN_COMMA =	44 as u32,
	G_TOKEN_NONE =	256 as u32,
	G_TOKEN_ERROR =	257 as u32,
	G_TOKEN_CHAR =	258 as u32,
	G_TOKEN_BINARY =	259 as u32,
	G_TOKEN_OCTAL =	260 as u32,
	G_TOKEN_INT =	261 as u32,
	G_TOKEN_HEX =	262 as u32,
	G_TOKEN_FLOAT =	263 as u32,
	G_TOKEN_STRING =	264 as u32,
	G_TOKEN_SYMBOL =	265 as u32,
	G_TOKEN_IDENTIFIER =	266 as u32,
	G_TOKEN_IDENTIFIER_NULL =	267 as u32,
	G_TOKEN_COMMENT_SINGLE =	268 as u32,
	G_TOKEN_COMMENT_MULTI =	269 as u32,
	G_TOKEN_LAST =	270 as u32,
}

impl GTokenType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GTokenType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_TOKEN_EOF =	0x00000000 (0)
	G_TOKEN_LEFT_PAREN =	0x00000028 (40)
	G_TOKEN_RIGHT_PAREN =	0x00000029 (41)
	G_TOKEN_LEFT_CURLY =	0x0000007B (123)
	G_TOKEN_RIGHT_CURLY =	0x0000007D (125)
	G_TOKEN_LEFT_BRACE =	0x0000005B (91)
	G_TOKEN_RIGHT_BRACE =	0x0000005D (93)
	G_TOKEN_EQUAL_SIGN =	0x0000003D (61)
	G_TOKEN_COMMA =	0x0000002C (44)
	G_TOKEN_NONE =	0x00000100 (256)
	G_TOKEN_ERROR =	0x00000101 (257)
	G_TOKEN_CHAR =	0x00000102 (258)
	G_TOKEN_BINARY =	0x00000103 (259)
	G_TOKEN_OCTAL =	0x00000104 (260)
	G_TOKEN_INT =	0x00000105 (261)
	G_TOKEN_HEX =	0x00000106 (262)
	G_TOKEN_FLOAT =	0x00000107 (263)
	G_TOKEN_STRING =	0x00000108 (264)
	G_TOKEN_SYMBOL =	0x00000109 (265)
	G_TOKEN_IDENTIFIER =	0x0000010A (266)
	G_TOKEN_IDENTIFIER_NULL =	0x0000010B (267)
	G_TOKEN_COMMENT_SINGLE =	0x0000010C (268)
	G_TOKEN_COMMENT_MULTI =	0x0000010D (269)
	G_TOKEN_LAST =	0x0000010E (270)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GTokenType {
	G_TOKEN_EOF =	0 as u32,
	G_TOKEN_LEFT_PAREN =	40 as u32,
	G_TOKEN_RIGHT_PAREN =	41 as u32,
	G_TOKEN_LEFT_CURLY =	123 as u32,
	G_TOKEN_RIGHT_CURLY =	125 as u32,
	G_TOKEN_LEFT_BRACE =	91 as u32,
	G_TOKEN_RIGHT_BRACE =	93 as u32,
	G_TOKEN_EQUAL_SIGN =	61 as u32,
	G_TOKEN_COMMA =	44 as u32,
	G_TOKEN_NONE =	256 as u32,
	G_TOKEN_ERROR =	257 as u32,
	G_TOKEN_CHAR =	258 as u32,
	G_TOKEN_BINARY =	259 as u32,
	G_TOKEN_OCTAL =	260 as u32,
	G_TOKEN_INT =	261 as u32,
	G_TOKEN_HEX =	262 as u32,
	G_TOKEN_FLOAT =	263 as u32,
	G_TOKEN_STRING =	264 as u32,
	G_TOKEN_SYMBOL =	265 as u32,
	G_TOKEN_IDENTIFIER =	266 as u32,
	G_TOKEN_IDENTIFIER_NULL =	267 as u32,
	G_TOKEN_COMMENT_SINGLE =	268 as u32,
	G_TOKEN_COMMENT_MULTI =	269 as u32,
	G_TOKEN_LAST =	270 as u32,
}

impl GTokenType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GTokenType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_SHELL_ERROR_BAD_QUOTING =	0x00000000 (0)
	G_SHELL_ERROR_EMPTY_STRING =	0x00000001 (1)
	G_SHELL_ERROR_FAILED =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GShellError {
	G_SHELL_ERROR_BAD_QUOTING =	0 as u32,
	G_SHELL_ERROR_EMPTY_STRING =	1 as u32,
	G_SHELL_ERROR_FAILED =	2 as u32,
}

impl GShellError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GShellError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_SHELL_ERROR_BAD_QUOTING =	0x00000000 (0)
	G_SHELL_ERROR_EMPTY_STRING =	0x00000001 (1)
	G_SHELL_ERROR_FAILED =	0x00000002 (2)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GShellError {
	G_SHELL_ERROR_BAD_QUOTING =	0 as u32,
	G_SHELL_ERROR_EMPTY_STRING =	1 as u32,
	G_SHELL_ERROR_FAILED =	2 as u32,
}

impl GShellError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GShellError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_SLICE_CONFIG_ALWAYS_MALLOC =	0x00000001 (1)
	G_SLICE_CONFIG_BYPASS_MAGAZINES =	0x00000002 (2)
	G_SLICE_CONFIG_WORKING_SET_MSECS =	0x00000003 (3)
	G_SLICE_CONFIG_COLOR_INCREMENT =	0x00000004 (4)
	G_SLICE_CONFIG_CHUNK_SIZES =	0x00000005 (5)
	G_SLICE_CONFIG_CONTENTION_COUNTER =	0x00000006 (6)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GSliceConfig {
	G_SLICE_CONFIG_ALWAYS_MALLOC =	1 as u32,
	G_SLICE_CONFIG_BYPASS_MAGAZINES =	2 as u32,
	G_SLICE_CONFIG_WORKING_SET_MSECS =	3 as u32,
	G_SLICE_CONFIG_COLOR_INCREMENT =	4 as u32,
	G_SLICE_CONFIG_CHUNK_SIZES =	5 as u32,
	G_SLICE_CONFIG_CONTENTION_COUNTER =	6 as u32,
}

impl GSliceConfig {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GSliceConfig {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_SLICE_CONFIG_ALWAYS_MALLOC =	0x00000001 (1)
	G_SLICE_CONFIG_BYPASS_MAGAZINES =	0x00000002 (2)
	G_SLICE_CONFIG_WORKING_SET_MSECS =	0x00000003 (3)
	G_SLICE_CONFIG_COLOR_INCREMENT =	0x00000004 (4)
	G_SLICE_CONFIG_CHUNK_SIZES =	0x00000005 (5)
	G_SLICE_CONFIG_CONTENTION_COUNTER =	0x00000006 (6)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GSliceConfig {
	G_SLICE_CONFIG_ALWAYS_MALLOC =	1 as u32,
	G_SLICE_CONFIG_BYPASS_MAGAZINES =	2 as u32,
	G_SLICE_CONFIG_WORKING_SET_MSECS =	3 as u32,
	G_SLICE_CONFIG_COLOR_INCREMENT =	4 as u32,
	G_SLICE_CONFIG_CHUNK_SIZES =	5 as u32,
	G_SLICE_CONFIG_CONTENTION_COUNTER =	6 as u32,
}

impl GSliceConfig {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GSliceConfig {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_SPAWN_ERROR_FORK =	0x00000000 (0)
	G_SPAWN_ERROR_READ =	0x00000001 (1)
	G_SPAWN_ERROR_CHDIR =	0x00000002 (2)
	G_SPAWN_ERROR_ACCES =	0x00000003 (3)
	G_SPAWN_ERROR_PERM =	0x00000004 (4)
	G_SPAWN_ERROR_TOO_BIG =	0x00000005 (5)
	G_SPAWN_ERROR_2BIG =	0x00000005 (5)
	G_SPAWN_ERROR_NOEXEC =	0x00000006 (6)
	G_SPAWN_ERROR_NAMETOOLONG =	0x00000007 (7)
	G_SPAWN_ERROR_NOENT =	0x00000008 (8)
	G_SPAWN_ERROR_NOMEM =	0x00000009 (9)
	G_SPAWN_ERROR_NOTDIR =	0x0000000A (10)
	G_SPAWN_ERROR_LOOP =	0x0000000B (11)
	G_SPAWN_ERROR_TXTBUSY =	0x0000000C (12)
	G_SPAWN_ERROR_IO =	0x0000000D (13)
	G_SPAWN_ERROR_NFILE =	0x0000000E (14)
	G_SPAWN_ERROR_MFILE =	0x0000000F (15)
	G_SPAWN_ERROR_INVAL =	0x00000010 (16)
	G_SPAWN_ERROR_ISDIR =	0x00000011 (17)
	G_SPAWN_ERROR_LIBBAD =	0x00000012 (18)
	G_SPAWN_ERROR_FAILED =	0x00000013 (19)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GSpawnError {
	G_SPAWN_ERROR_FORK =	0 as u32,
	G_SPAWN_ERROR_READ =	1 as u32,
	G_SPAWN_ERROR_CHDIR =	2 as u32,
	G_SPAWN_ERROR_ACCES =	3 as u32,
	G_SPAWN_ERROR_PERM =	4 as u32,
	G_SPAWN_ERROR_TOO_BIG =	5 as u32,
	G_SPAWN_ERROR_2BIG =	5 as u32,
	G_SPAWN_ERROR_NOEXEC =	6 as u32,
	G_SPAWN_ERROR_NAMETOOLONG =	7 as u32,
	G_SPAWN_ERROR_NOENT =	8 as u32,
	G_SPAWN_ERROR_NOMEM =	9 as u32,
	G_SPAWN_ERROR_NOTDIR =	10 as u32,
	G_SPAWN_ERROR_LOOP =	11 as u32,
	G_SPAWN_ERROR_TXTBUSY =	12 as u32,
	G_SPAWN_ERROR_IO =	13 as u32,
	G_SPAWN_ERROR_NFILE =	14 as u32,
	G_SPAWN_ERROR_MFILE =	15 as u32,
	G_SPAWN_ERROR_INVAL =	16 as u32,
	G_SPAWN_ERROR_ISDIR =	17 as u32,
	G_SPAWN_ERROR_LIBBAD =	18 as u32,
	G_SPAWN_ERROR_FAILED =	19 as u32,
}

impl GSpawnError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GSpawnError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_SPAWN_ERROR_FORK =	0x00000000 (0)
	G_SPAWN_ERROR_READ =	0x00000001 (1)
	G_SPAWN_ERROR_CHDIR =	0x00000002 (2)
	G_SPAWN_ERROR_ACCES =	0x00000003 (3)
	G_SPAWN_ERROR_PERM =	0x00000004 (4)
	G_SPAWN_ERROR_TOO_BIG =	0x00000005 (5)
	G_SPAWN_ERROR_2BIG =	0x00000005 (5)
	G_SPAWN_ERROR_NOEXEC =	0x00000006 (6)
	G_SPAWN_ERROR_NAMETOOLONG =	0x00000007 (7)
	G_SPAWN_ERROR_NOENT =	0x00000008 (8)
	G_SPAWN_ERROR_NOMEM =	0x00000009 (9)
	G_SPAWN_ERROR_NOTDIR =	0x0000000A (10)
	G_SPAWN_ERROR_LOOP =	0x0000000B (11)
	G_SPAWN_ERROR_TXTBUSY =	0x0000000C (12)
	G_SPAWN_ERROR_IO =	0x0000000D (13)
	G_SPAWN_ERROR_NFILE =	0x0000000E (14)
	G_SPAWN_ERROR_MFILE =	0x0000000F (15)
	G_SPAWN_ERROR_INVAL =	0x00000010 (16)
	G_SPAWN_ERROR_ISDIR =	0x00000011 (17)
	G_SPAWN_ERROR_LIBBAD =	0x00000012 (18)
	G_SPAWN_ERROR_FAILED =	0x00000013 (19)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GSpawnError {
	G_SPAWN_ERROR_FORK =	0 as u32,
	G_SPAWN_ERROR_READ =	1 as u32,
	G_SPAWN_ERROR_CHDIR =	2 as u32,
	G_SPAWN_ERROR_ACCES =	3 as u32,
	G_SPAWN_ERROR_PERM =	4 as u32,
	G_SPAWN_ERROR_TOO_BIG =	5 as u32,
	G_SPAWN_ERROR_2BIG =	5 as u32,
	G_SPAWN_ERROR_NOEXEC =	6 as u32,
	G_SPAWN_ERROR_NAMETOOLONG =	7 as u32,
	G_SPAWN_ERROR_NOENT =	8 as u32,
	G_SPAWN_ERROR_NOMEM =	9 as u32,
	G_SPAWN_ERROR_NOTDIR =	10 as u32,
	G_SPAWN_ERROR_LOOP =	11 as u32,
	G_SPAWN_ERROR_TXTBUSY =	12 as u32,
	G_SPAWN_ERROR_IO =	13 as u32,
	G_SPAWN_ERROR_NFILE =	14 as u32,
	G_SPAWN_ERROR_MFILE =	15 as u32,
	G_SPAWN_ERROR_INVAL =	16 as u32,
	G_SPAWN_ERROR_ISDIR =	17 as u32,
	G_SPAWN_ERROR_LIBBAD =	18 as u32,
	G_SPAWN_ERROR_FAILED =	19 as u32,
}

impl GSpawnError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GSpawnError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_SPAWN_LEAVE_DESCRIPTORS_OPEN =	0x00000001 (1)
	G_SPAWN_DO_NOT_REAP_CHILD =	0x00000002 (2)
	G_SPAWN_SEARCH_PATH =	0x00000004 (4)
	G_SPAWN_STDOUT_TO_DEV_NULL =	0x00000008 (8)
	G_SPAWN_STDERR_TO_DEV_NULL =	0x00000010 (16)
	G_SPAWN_CHILD_INHERITS_STDIN =	0x00000020 (32)
	G_SPAWN_FILE_AND_ARGV_ZERO =	0x00000040 (64)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GSpawnFlags {
	G_SPAWN_LEAVE_DESCRIPTORS_OPEN =	1 as u32,
	G_SPAWN_DO_NOT_REAP_CHILD =	2 as u32,
	G_SPAWN_SEARCH_PATH =	4 as u32,
	G_SPAWN_STDOUT_TO_DEV_NULL =	8 as u32,
	G_SPAWN_STDERR_TO_DEV_NULL =	16 as u32,
	G_SPAWN_CHILD_INHERITS_STDIN =	32 as u32,
	G_SPAWN_FILE_AND_ARGV_ZERO =	64 as u32,
}

impl GSpawnFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GSpawnFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_SPAWN_LEAVE_DESCRIPTORS_OPEN =	0x00000001 (1)
	G_SPAWN_DO_NOT_REAP_CHILD =	0x00000002 (2)
	G_SPAWN_SEARCH_PATH =	0x00000004 (4)
	G_SPAWN_STDOUT_TO_DEV_NULL =	0x00000008 (8)
	G_SPAWN_STDERR_TO_DEV_NULL =	0x00000010 (16)
	G_SPAWN_CHILD_INHERITS_STDIN =	0x00000020 (32)
	G_SPAWN_FILE_AND_ARGV_ZERO =	0x00000040 (64)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GSpawnFlags {
	G_SPAWN_LEAVE_DESCRIPTORS_OPEN =	1 as u32,
	G_SPAWN_DO_NOT_REAP_CHILD =	2 as u32,
	G_SPAWN_SEARCH_PATH =	4 as u32,
	G_SPAWN_STDOUT_TO_DEV_NULL =	8 as u32,
	G_SPAWN_STDERR_TO_DEV_NULL =	16 as u32,
	G_SPAWN_CHILD_INHERITS_STDIN =	32 as u32,
	G_SPAWN_FILE_AND_ARGV_ZERO =	64 as u32,
}

impl GSpawnFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GSpawnFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_ASCII_ALNUM =	0x00000001 (1)
	G_ASCII_ALPHA =	0x00000002 (2)
	G_ASCII_CNTRL =	0x00000004 (4)
	G_ASCII_DIGIT =	0x00000008 (8)
	G_ASCII_GRAPH =	0x00000010 (16)
	G_ASCII_LOWER =	0x00000020 (32)
	G_ASCII_PRINT =	0x00000040 (64)
	G_ASCII_PUNCT =	0x00000080 (128)
	G_ASCII_SPACE =	0x00000100 (256)
	G_ASCII_UPPER =	0x00000200 (512)
	G_ASCII_XDIGIT =	0x00000400 (1024)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GAsciiType {
	G_ASCII_ALNUM =	1 as u32,
	G_ASCII_ALPHA =	2 as u32,
	G_ASCII_CNTRL =	4 as u32,
	G_ASCII_DIGIT =	8 as u32,
	G_ASCII_GRAPH =	16 as u32,
	G_ASCII_LOWER =	32 as u32,
	G_ASCII_PRINT =	64 as u32,
	G_ASCII_PUNCT =	128 as u32,
	G_ASCII_SPACE =	256 as u32,
	G_ASCII_UPPER =	512 as u32,
	G_ASCII_XDIGIT =	1024 as u32,
}

impl GAsciiType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GAsciiType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_ASCII_ALNUM =	0x00000001 (1)
	G_ASCII_ALPHA =	0x00000002 (2)
	G_ASCII_CNTRL =	0x00000004 (4)
	G_ASCII_DIGIT =	0x00000008 (8)
	G_ASCII_GRAPH =	0x00000010 (16)
	G_ASCII_LOWER =	0x00000020 (32)
	G_ASCII_PRINT =	0x00000040 (64)
	G_ASCII_PUNCT =	0x00000080 (128)
	G_ASCII_SPACE =	0x00000100 (256)
	G_ASCII_UPPER =	0x00000200 (512)
	G_ASCII_XDIGIT =	0x00000400 (1024)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GAsciiType {
	G_ASCII_ALNUM =	1 as u32,
	G_ASCII_ALPHA =	2 as u32,
	G_ASCII_CNTRL =	4 as u32,
	G_ASCII_DIGIT =	8 as u32,
	G_ASCII_GRAPH =	16 as u32,
	G_ASCII_LOWER =	32 as u32,
	G_ASCII_PRINT =	64 as u32,
	G_ASCII_PUNCT =	128 as u32,
	G_ASCII_SPACE =	256 as u32,
	G_ASCII_UPPER =	512 as u32,
	G_ASCII_XDIGIT =	1024 as u32,
}

impl GAsciiType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GAsciiType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_TEST_TRAP_SILENCE_STDOUT =	0x00000080 (128)
	G_TEST_TRAP_SILENCE_STDERR =	0x00000100 (256)
	G_TEST_TRAP_INHERIT_STDIN =	0x00000200 (512)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GTestTrapFlags {
	G_TEST_TRAP_SILENCE_STDOUT =	128 as u32,
	G_TEST_TRAP_SILENCE_STDERR =	256 as u32,
	G_TEST_TRAP_INHERIT_STDIN =	512 as u32,
}

impl GTestTrapFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GTestTrapFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_TEST_TRAP_SILENCE_STDOUT =	0x00000080 (128)
	G_TEST_TRAP_SILENCE_STDERR =	0x00000100 (256)
	G_TEST_TRAP_INHERIT_STDIN =	0x00000200 (512)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GTestTrapFlags {
	G_TEST_TRAP_SILENCE_STDOUT =	128 as u32,
	G_TEST_TRAP_SILENCE_STDERR =	256 as u32,
	G_TEST_TRAP_INHERIT_STDIN =	512 as u32,
}

impl GTestTrapFlags {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GTestTrapFlags {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_TEST_LOG_NONE =	0x00000000 (0)
	G_TEST_LOG_ERROR =	0x00000001 (1)
	G_TEST_LOG_START_BINARY =	0x00000002 (2)
	G_TEST_LOG_LIST_CASE =	0x00000003 (3)
	G_TEST_LOG_SKIP_CASE =	0x00000004 (4)
	G_TEST_LOG_START_CASE =	0x00000005 (5)
	G_TEST_LOG_STOP_CASE =	0x00000006 (6)
	G_TEST_LOG_MIN_RESULT =	0x00000007 (7)
	G_TEST_LOG_MAX_RESULT =	0x00000008 (8)
	G_TEST_LOG_MESSAGE =	0x00000009 (9)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GTestLogType {
	G_TEST_LOG_NONE =	0 as u32,
	G_TEST_LOG_ERROR =	1 as u32,
	G_TEST_LOG_START_BINARY =	2 as u32,
	G_TEST_LOG_LIST_CASE =	3 as u32,
	G_TEST_LOG_SKIP_CASE =	4 as u32,
	G_TEST_LOG_START_CASE =	5 as u32,
	G_TEST_LOG_STOP_CASE =	6 as u32,
	G_TEST_LOG_MIN_RESULT =	7 as u32,
	G_TEST_LOG_MAX_RESULT =	8 as u32,
	G_TEST_LOG_MESSAGE =	9 as u32,
}

impl GTestLogType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GTestLogType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_TEST_LOG_NONE =	0x00000000 (0)
	G_TEST_LOG_ERROR =	0x00000001 (1)
	G_TEST_LOG_START_BINARY =	0x00000002 (2)
	G_TEST_LOG_LIST_CASE =	0x00000003 (3)
	G_TEST_LOG_SKIP_CASE =	0x00000004 (4)
	G_TEST_LOG_START_CASE =	0x00000005 (5)
	G_TEST_LOG_STOP_CASE =	0x00000006 (6)
	G_TEST_LOG_MIN_RESULT =	0x00000007 (7)
	G_TEST_LOG_MAX_RESULT =	0x00000008 (8)
	G_TEST_LOG_MESSAGE =	0x00000009 (9)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GTestLogType {
	G_TEST_LOG_NONE =	0 as u32,
	G_TEST_LOG_ERROR =	1 as u32,
	G_TEST_LOG_START_BINARY =	2 as u32,
	G_TEST_LOG_LIST_CASE =	3 as u32,
	G_TEST_LOG_SKIP_CASE =	4 as u32,
	G_TEST_LOG_START_CASE =	5 as u32,
	G_TEST_LOG_STOP_CASE =	6 as u32,
	G_TEST_LOG_MIN_RESULT =	7 as u32,
	G_TEST_LOG_MAX_RESULT =	8 as u32,
	G_TEST_LOG_MESSAGE =	9 as u32,
}

impl GTestLogType {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GTestLogType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_VARIANT_CLASS_BOOLEAN =	0x00000062 (98)
	G_VARIANT_CLASS_BYTE =	0x00000079 (121)
	G_VARIANT_CLASS_INT16 =	0x0000006E (110)
	G_VARIANT_CLASS_UINT16 =	0x00000071 (113)
	G_VARIANT_CLASS_INT32 =	0x00000069 (105)
	G_VARIANT_CLASS_UINT32 =	0x00000075 (117)
	G_VARIANT_CLASS_INT64 =	0x00000078 (120)
	G_VARIANT_CLASS_UINT64 =	0x00000074 (116)
	G_VARIANT_CLASS_HANDLE =	0x00000068 (104)
	G_VARIANT_CLASS_DOUBLE =	0x00000064 (100)
	G_VARIANT_CLASS_STRING =	0x00000073 (115)
	G_VARIANT_CLASS_OBJECT_PATH =	0x0000006F (111)
	G_VARIANT_CLASS_SIGNATURE =	0x00000067 (103)
	G_VARIANT_CLASS_VARIANT =	0x00000076 (118)
	G_VARIANT_CLASS_MAYBE =	0x0000006D (109)
	G_VARIANT_CLASS_ARRAY =	0x00000061 (97)
	G_VARIANT_CLASS_TUPLE =	0x00000028 (40)
	G_VARIANT_CLASS_DICT_ENTRY =	0x0000007B (123)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GVariantClass {
	G_VARIANT_CLASS_BOOLEAN =	98 as u32,
	G_VARIANT_CLASS_BYTE =	121 as u32,
	G_VARIANT_CLASS_INT16 =	110 as u32,
	G_VARIANT_CLASS_UINT16 =	113 as u32,
	G_VARIANT_CLASS_INT32 =	105 as u32,
	G_VARIANT_CLASS_UINT32 =	117 as u32,
	G_VARIANT_CLASS_INT64 =	120 as u32,
	G_VARIANT_CLASS_UINT64 =	116 as u32,
	G_VARIANT_CLASS_HANDLE =	104 as u32,
	G_VARIANT_CLASS_DOUBLE =	100 as u32,
	G_VARIANT_CLASS_STRING =	115 as u32,
	G_VARIANT_CLASS_OBJECT_PATH =	111 as u32,
	G_VARIANT_CLASS_SIGNATURE =	103 as u32,
	G_VARIANT_CLASS_VARIANT =	118 as u32,
	G_VARIANT_CLASS_MAYBE =	109 as u32,
	G_VARIANT_CLASS_ARRAY =	97 as u32,
	G_VARIANT_CLASS_TUPLE =	40 as u32,
	G_VARIANT_CLASS_DICT_ENTRY =	123 as u32,
}

impl GVariantClass {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GVariantClass {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_VARIANT_CLASS_BOOLEAN =	0x00000062 (98)
	G_VARIANT_CLASS_BYTE =	0x00000079 (121)
	G_VARIANT_CLASS_INT16 =	0x0000006E (110)
	G_VARIANT_CLASS_UINT16 =	0x00000071 (113)
	G_VARIANT_CLASS_INT32 =	0x00000069 (105)
	G_VARIANT_CLASS_UINT32 =	0x00000075 (117)
	G_VARIANT_CLASS_INT64 =	0x00000078 (120)
	G_VARIANT_CLASS_UINT64 =	0x00000074 (116)
	G_VARIANT_CLASS_HANDLE =	0x00000068 (104)
	G_VARIANT_CLASS_DOUBLE =	0x00000064 (100)
	G_VARIANT_CLASS_STRING =	0x00000073 (115)
	G_VARIANT_CLASS_OBJECT_PATH =	0x0000006F (111)
	G_VARIANT_CLASS_SIGNATURE =	0x00000067 (103)
	G_VARIANT_CLASS_VARIANT =	0x00000076 (118)
	G_VARIANT_CLASS_MAYBE =	0x0000006D (109)
	G_VARIANT_CLASS_ARRAY =	0x00000061 (97)
	G_VARIANT_CLASS_TUPLE =	0x00000028 (40)
	G_VARIANT_CLASS_DICT_ENTRY =	0x0000007B (123)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GVariantClass {
	G_VARIANT_CLASS_BOOLEAN =	98 as u32,
	G_VARIANT_CLASS_BYTE =	121 as u32,
	G_VARIANT_CLASS_INT16 =	110 as u32,
	G_VARIANT_CLASS_UINT16 =	113 as u32,
	G_VARIANT_CLASS_INT32 =	105 as u32,
	G_VARIANT_CLASS_UINT32 =	117 as u32,
	G_VARIANT_CLASS_INT64 =	120 as u32,
	G_VARIANT_CLASS_UINT64 =	116 as u32,
	G_VARIANT_CLASS_HANDLE =	104 as u32,
	G_VARIANT_CLASS_DOUBLE =	100 as u32,
	G_VARIANT_CLASS_STRING =	115 as u32,
	G_VARIANT_CLASS_OBJECT_PATH =	111 as u32,
	G_VARIANT_CLASS_SIGNATURE =	103 as u32,
	G_VARIANT_CLASS_VARIANT =	118 as u32,
	G_VARIANT_CLASS_MAYBE =	109 as u32,
	G_VARIANT_CLASS_ARRAY =	97 as u32,
	G_VARIANT_CLASS_TUPLE =	40 as u32,
	G_VARIANT_CLASS_DICT_ENTRY =	123 as u32,
}

impl GVariantClass {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GVariantClass {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_VARIANT_PARSE_ERROR_FAILED =	0x00000000 (0)
	G_VARIANT_PARSE_ERROR_BASIC_TYPE_EXPECTED =	0x00000001 (1)
	G_VARIANT_PARSE_ERROR_CANNOT_INFER_TYPE =	0x00000002 (2)
	G_VARIANT_PARSE_ERROR_DEFINITE_TYPE_EXPECTED =	0x00000003 (3)
	G_VARIANT_PARSE_ERROR_INPUT_NOT_AT_END =	0x00000004 (4)
	G_VARIANT_PARSE_ERROR_INVALID_CHARACTER =	0x00000005 (5)
	G_VARIANT_PARSE_ERROR_INVALID_FORMAT_STRING =	0x00000006 (6)
	G_VARIANT_PARSE_ERROR_INVALID_OBJECT_PATH =	0x00000007 (7)
	G_VARIANT_PARSE_ERROR_INVALID_SIGNATURE =	0x00000008 (8)
	G_VARIANT_PARSE_ERROR_INVALID_TYPE_STRING =	0x00000009 (9)
	G_VARIANT_PARSE_ERROR_NO_COMMON_TYPE =	0x0000000A (10)
	G_VARIANT_PARSE_ERROR_NUMBER_OUT_OF_RANGE =	0x0000000B (11)
	G_VARIANT_PARSE_ERROR_NUMBER_TOO_BIG =	0x0000000C (12)
	G_VARIANT_PARSE_ERROR_TYPE_ERROR =	0x0000000D (13)
	G_VARIANT_PARSE_ERROR_UNEXPECTED_TOKEN =	0x0000000E (14)
	G_VARIANT_PARSE_ERROR_UNKNOWN_KEYWORD =	0x0000000F (15)
	G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT =	0x00000010 (16)
	G_VARIANT_PARSE_ERROR_VALUE_EXPECTED =	0x00000011 (17)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GVariantParseError {
	G_VARIANT_PARSE_ERROR_FAILED =	0 as u32,
	G_VARIANT_PARSE_ERROR_BASIC_TYPE_EXPECTED =	1 as u32,
	G_VARIANT_PARSE_ERROR_CANNOT_INFER_TYPE =	2 as u32,
	G_VARIANT_PARSE_ERROR_DEFINITE_TYPE_EXPECTED =	3 as u32,
	G_VARIANT_PARSE_ERROR_INPUT_NOT_AT_END =	4 as u32,
	G_VARIANT_PARSE_ERROR_INVALID_CHARACTER =	5 as u32,
	G_VARIANT_PARSE_ERROR_INVALID_FORMAT_STRING =	6 as u32,
	G_VARIANT_PARSE_ERROR_INVALID_OBJECT_PATH =	7 as u32,
	G_VARIANT_PARSE_ERROR_INVALID_SIGNATURE =	8 as u32,
	G_VARIANT_PARSE_ERROR_INVALID_TYPE_STRING =	9 as u32,
	G_VARIANT_PARSE_ERROR_NO_COMMON_TYPE =	10 as u32,
	G_VARIANT_PARSE_ERROR_NUMBER_OUT_OF_RANGE =	11 as u32,
	G_VARIANT_PARSE_ERROR_NUMBER_TOO_BIG =	12 as u32,
	G_VARIANT_PARSE_ERROR_TYPE_ERROR =	13 as u32,
	G_VARIANT_PARSE_ERROR_UNEXPECTED_TOKEN =	14 as u32,
	G_VARIANT_PARSE_ERROR_UNKNOWN_KEYWORD =	15 as u32,
	G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT =	16 as u32,
	G_VARIANT_PARSE_ERROR_VALUE_EXPECTED =	17 as u32,
}

impl GVariantParseError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GVariantParseError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_VARIANT_PARSE_ERROR_FAILED =	0x00000000 (0)
	G_VARIANT_PARSE_ERROR_BASIC_TYPE_EXPECTED =	0x00000001 (1)
	G_VARIANT_PARSE_ERROR_CANNOT_INFER_TYPE =	0x00000002 (2)
	G_VARIANT_PARSE_ERROR_DEFINITE_TYPE_EXPECTED =	0x00000003 (3)
	G_VARIANT_PARSE_ERROR_INPUT_NOT_AT_END =	0x00000004 (4)
	G_VARIANT_PARSE_ERROR_INVALID_CHARACTER =	0x00000005 (5)
	G_VARIANT_PARSE_ERROR_INVALID_FORMAT_STRING =	0x00000006 (6)
	G_VARIANT_PARSE_ERROR_INVALID_OBJECT_PATH =	0x00000007 (7)
	G_VARIANT_PARSE_ERROR_INVALID_SIGNATURE =	0x00000008 (8)
	G_VARIANT_PARSE_ERROR_INVALID_TYPE_STRING =	0x00000009 (9)
	G_VARIANT_PARSE_ERROR_NO_COMMON_TYPE =	0x0000000A (10)
	G_VARIANT_PARSE_ERROR_NUMBER_OUT_OF_RANGE =	0x0000000B (11)
	G_VARIANT_PARSE_ERROR_NUMBER_TOO_BIG =	0x0000000C (12)
	G_VARIANT_PARSE_ERROR_TYPE_ERROR =	0x0000000D (13)
	G_VARIANT_PARSE_ERROR_UNEXPECTED_TOKEN =	0x0000000E (14)
	G_VARIANT_PARSE_ERROR_UNKNOWN_KEYWORD =	0x0000000F (15)
	G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT =	0x00000010 (16)
	G_VARIANT_PARSE_ERROR_VALUE_EXPECTED =	0x00000011 (17)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GVariantParseError {
	G_VARIANT_PARSE_ERROR_FAILED =	0 as u32,
	G_VARIANT_PARSE_ERROR_BASIC_TYPE_EXPECTED =	1 as u32,
	G_VARIANT_PARSE_ERROR_CANNOT_INFER_TYPE =	2 as u32,
	G_VARIANT_PARSE_ERROR_DEFINITE_TYPE_EXPECTED =	3 as u32,
	G_VARIANT_PARSE_ERROR_INPUT_NOT_AT_END =	4 as u32,
	G_VARIANT_PARSE_ERROR_INVALID_CHARACTER =	5 as u32,
	G_VARIANT_PARSE_ERROR_INVALID_FORMAT_STRING =	6 as u32,
	G_VARIANT_PARSE_ERROR_INVALID_OBJECT_PATH =	7 as u32,
	G_VARIANT_PARSE_ERROR_INVALID_SIGNATURE =	8 as u32,
	G_VARIANT_PARSE_ERROR_INVALID_TYPE_STRING =	9 as u32,
	G_VARIANT_PARSE_ERROR_NO_COMMON_TYPE =	10 as u32,
	G_VARIANT_PARSE_ERROR_NUMBER_OUT_OF_RANGE =	11 as u32,
	G_VARIANT_PARSE_ERROR_NUMBER_TOO_BIG =	12 as u32,
	G_VARIANT_PARSE_ERROR_TYPE_ERROR =	13 as u32,
	G_VARIANT_PARSE_ERROR_UNEXPECTED_TOKEN =	14 as u32,
	G_VARIANT_PARSE_ERROR_UNKNOWN_KEYWORD =	15 as u32,
	G_VARIANT_PARSE_ERROR_UNTERMINATED_STRING_CONSTANT =	16 as u32,
	G_VARIANT_PARSE_ERROR_VALUE_EXPECTED =	17 as u32,
}

impl GVariantParseError {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GVariantParseError {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_THREAD_PRIORITY_LOW =	0x00000000 (0)
	G_THREAD_PRIORITY_NORMAL =	0x00000001 (1)
	G_THREAD_PRIORITY_HIGH =	0x00000002 (2)
	G_THREAD_PRIORITY_URGENT =	0x00000003 (3)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GThreadPriority {
	G_THREAD_PRIORITY_LOW =	0 as u32,
	G_THREAD_PRIORITY_NORMAL =	1 as u32,
	G_THREAD_PRIORITY_HIGH =	2 as u32,
	G_THREAD_PRIORITY_URGENT =	3 as u32,
}

impl GThreadPriority {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GThreadPriority {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	G_THREAD_PRIORITY_LOW =	0x00000000 (0)
	G_THREAD_PRIORITY_NORMAL =	0x00000001 (1)
	G_THREAD_PRIORITY_HIGH =	0x00000002 (2)
	G_THREAD_PRIORITY_URGENT =	0x00000003 (3)
}
*/
#[derive(Copy, PartialEq, Show)]
#[repr(u32)]
pub enum GThreadPriority {
	G_THREAD_PRIORITY_LOW =	0 as u32,
	G_THREAD_PRIORITY_NORMAL =	1 as u32,
	G_THREAD_PRIORITY_HIGH =	2 as u32,
	G_THREAD_PRIORITY_URGENT =	3 as u32,
}

impl GThreadPriority {
	pub fn to_u32(&self) -> libc::c_uint {
		*self as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> GThreadPriority {
		unsafe { mem::transmute(v) }
	}
}


/* __G_LIB_H__ # */

/* __GLIB_H_INSIDE__ # */

/* __G_ALLOCA_H__ # */

/* __G_TYPES_H__ # */

/* __G_MACROS_H__ /* We include stddef.h to get the system's definition of NULL
 */ */

/* G_GNUC_EXTENSION __extension__ # */

/* G_GNUC_PURE __attribute__ ( ( __pure__ ) ) # */

/* G_GNUC_MALLOC __attribute__ ( ( __malloc__ ) ) # */

/* G_GNUC_NULL_TERMINATED __attribute__ ( ( __sentinel__ ) ) # */

/* G_GNUC_ALLOC_SIZE ( x ) # */

/* G_GNUC_ALLOC_SIZE2 ( x , y ) # */

/* G_GNUC_PRINTF ( format_idx , arg_idx ) __attribute__ ( ( __format__ ( __printf__ , format_idx , arg_idx ) ) ) # */

/* G_GNUC_SCANF ( format_idx , arg_idx ) __attribute__ ( ( __format__ ( __scanf__ , format_idx , arg_idx ) ) ) # */

/* G_GNUC_FORMAT ( arg_idx ) __attribute__ ( ( __format_arg__ ( arg_idx ) ) ) # */

/* G_GNUC_NORETURN __attribute__ ( ( __noreturn__ ) ) # */

/* G_GNUC_CONST __attribute__ ( ( __const__ ) ) # */

/* G_GNUC_UNUSED __attribute__ ( ( __unused__ ) ) # */

/* G_GNUC_NO_INSTRUMENT __attribute__ ( ( __no_instrument_function__ ) ) # */

/* G_GNUC_DEPRECATED __attribute__ ( ( __deprecated__ ) ) # */

/* G_GNUC_DEPRECATED_FOR ( f ) G_GNUC_DEPRECATED # */

/* G_GNUC_BEGIN_IGNORE_DEPRECATIONS # */

/* G_GNUC_END_IGNORE_DEPRECATIONS # */

/* G_GNUC_MAY_ALIAS __attribute__ ( ( may_alias ) ) # */

/* G_GNUC_WARN_UNUSED_RESULT __attribute__ ( ( warn_unused_result ) ) # */

/* G_GNUC_FUNCTION "" # */

/* G_GNUC_PRETTY_FUNCTION "" # */

/* G_STRINGIFY ( macro_or_string ) G_STRINGIFY_ARG ( macro_or_string ) # */

/* G_STRINGIFY_ARG ( contents ) # contents # */

/* G_PASTE_ARGS ( identifier1 , identifier2 ) identifier1 ## identifier2 # */

/* G_PASTE ( identifier1 , identifier2 ) G_PASTE_ARGS ( identifier1 , identifier2 ) # */

/* G_STATIC_ASSERT ( expr ) typedef char G_PASTE ( _GStaticAssertCompileTimeAssertion_ , __COUNTER__ ) [ ( expr ) ? 1 : - 1 ] # */

/* G_STATIC_ASSERT_EXPR ( expr ) ( ( void ) sizeof ( char [ ( expr ) ? 1 : - 1 ] ) ) # */

/* G_STRLOC __FILE__ ":" G_STRINGIFY ( __LINE__ ) # */

/* G_STRFUNC ( ( const char * ) ( __PRETTY_FUNCTION__ ) ) # */

/* G_BEGIN_DECLS # */

/* G_END_DECLS # */

/* FALSE ( 0 ) # */

/* TRUE ( ! FALSE ) # */

/* MAX ( a , b ) ( ( ( a ) > ( b ) ) ? ( a ) : ( b ) ) # */

/* MIN ( a , b ) ( ( ( a ) < ( b ) ) ? ( a ) : ( b ) ) # */

/* ABS ( a ) ( ( ( a ) < 0 ) ? - ( a ) : ( a ) ) # */

/* CLAMP ( x , low , high ) ( ( ( x ) > ( high ) ) ? ( high ) : ( ( ( x ) < ( low ) ) ? ( low ) : ( x ) ) ) /* Count the number of elements in an array. The array must be defined
 * as such; using this with a dynamically allocated array will give
 * incorrect results.
 */ */

/* G_N_ELEMENTS ( arr ) ( sizeof ( arr ) / sizeof ( ( arr ) [ 0 ] ) ) /* Macros by analogy to GINT_TO_POINTER, GPOINTER_TO_INT
 */ */

/* GPOINTER_TO_SIZE ( p ) ( ( gsize ) ( p ) ) # */

/* GSIZE_TO_POINTER ( s ) ( ( gpointer ) ( gsize ) ( s ) ) /* Provide convenience macros for handling structure
 * fields through their offsets.
 */ */

/* G_STRUCT_OFFSET ( struct_type , member ) ( ( glong ) offsetof ( struct_type , member ) ) # */

/* G_STRUCT_MEMBER_P ( struct_p , struct_offset ) ( ( gpointer ) ( ( guint8 * ) ( struct_p ) + ( glong ) ( struct_offset ) ) ) # */

/* G_STRUCT_MEMBER ( member_type , struct_p , struct_offset ) ( * ( member_type * ) G_STRUCT_MEMBER_P ( ( struct_p ) , ( struct_offset ) ) ) /* Provide simple macro statement wrappers:
 *   G_STMT_START { statements; } G_STMT_END;
 * This can be used as a single statement, like:
 *   if (x) G_STMT_START { ... } G_STMT_END; else ...
 * This intentionally does not use compiler extensions like GCC's '({...})' to
 * avoid portability issue or side effects when compiled with different compilers.
 */ */

/* G_STMT_START do # */

/* G_STMT_END while ( 0 ) # */

/* G_CONST_RETURN const # */

/* G_LIKELY ( expr ) ( expr ) # */

/* G_UNLIKELY ( expr ) ( expr ) # */

/* G_DEPRECATED __attribute__ ( ( __deprecated__ ) ) # */

/* G_DEPRECATED_FOR ( f ) G_DEPRECATED # */

/* G_UNAVAILABLE ( maj , min ) # */

/* GLIB_DEPRECATED G_DEPRECATED # */

/* GLIB_DEPRECATED_FOR ( f ) G_DEPRECATED_FOR ( f ) # */

/* GLIB_UNAVAILABLE ( maj , min ) G_UNAVAILABLE ( maj , min ) # */

/* __G_VERSION_MACROS_H__ /* Version boundaries checks */ */

/* G_ENCODE_VERSION ( major , minor ) ( ( major ) << 16 | ( minor ) << 8 ) /* XXX: Every new stable minor release bump should add a macro here */ */

/* GLIB_VERSION_2_26 ( G_ENCODE_VERSION ( 2 , 26 ) ) /**
 * GLIB_VERSION_2_28:
 *
 * A macro that evaluates to the 2.28 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.32
 */ */

/* GLIB_VERSION_2_28 ( G_ENCODE_VERSION ( 2 , 28 ) ) /**
 * GLIB_VERSION_2_30:
 *
 * A macro that evaluates to the 2.30 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.32
 */ */

/* GLIB_VERSION_2_30 ( G_ENCODE_VERSION ( 2 , 30 ) ) /**
 * GLIB_VERSION_2_32:
 *
 * A macro that evaluates to the 2.32 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.32
 */ */

/* GLIB_VERSION_2_32 ( G_ENCODE_VERSION ( 2 , 32 ) ) /* evaluates to the current stable version; for development cycles,
 * this means the next stable target
 */ */

/* GLIB_VERSION_CUR_STABLE ( G_ENCODE_VERSION ( GLIB_MAJOR_VERSION , GLIB_MINOR_VERSION ) ) # */

/* GLIB_VERSION_PREV_STABLE ( G_ENCODE_VERSION ( GLIB_MAJOR_VERSION , GLIB_MINOR_VERSION - 2 ) ) # */

/* GLIB_VERSION_MIN_REQUIRED ( GLIB_VERSION_PREV_STABLE ) # */

/* GLIB_VERSION_MAX_ALLOWED GLIB_VERSION_CUR_STABLE # */

/* GLIB_DEPRECATED_IN_2_26 GLIB_DEPRECATED # */

/* GLIB_DEPRECATED_IN_2_26_FOR ( f ) GLIB_DEPRECATED_FOR ( f ) # */

/* GLIB_AVAILABLE_IN_2_26 # */

/* GLIB_DEPRECATED_IN_2_28 GLIB_DEPRECATED # */

/* GLIB_DEPRECATED_IN_2_28_FOR ( f ) GLIB_DEPRECATED_FOR ( f ) # */

/* GLIB_AVAILABLE_IN_2_28 # */

/* GLIB_DEPRECATED_IN_2_30 GLIB_DEPRECATED # */

/* GLIB_DEPRECATED_IN_2_30_FOR ( f ) GLIB_DEPRECATED_FOR ( f ) # */

/* GLIB_AVAILABLE_IN_2_30 # */

/* GLIB_DEPRECATED_IN_2_32 # */

/* GLIB_DEPRECATED_IN_2_32_FOR ( f ) # */

/* GLIB_AVAILABLE_IN_2_32 # */

/* G_MININT8 ( ( gint8 ) 0x80 ) # */

/* G_MAXINT8 ( ( gint8 ) 0x7f ) # */

/* G_MAXUINT8 ( ( guint8 ) 0xff ) # */

/* G_MININT16 ( ( gint16 ) 0x8000 ) # */

/* G_MAXINT16 ( ( gint16 ) 0x7fff ) # */

/* G_MAXUINT16 ( ( guint16 ) 0xffff ) # */

/* G_MININT32 ( ( gint32 ) 0x80000000 ) # */

/* G_MAXINT32 ( ( gint32 ) 0x7fffffff ) # */

/* G_MAXUINT32 ( ( guint32 ) 0xffffffff ) # */

/* G_MININT64 ( ( gint64 ) G_GINT64_CONSTANT ( 0x8000000000000000 ) ) # */

/* G_MAXINT64 G_GINT64_CONSTANT ( 0x7fffffffffffffff ) # */

/* G_MAXUINT64 G_GINT64_CONSTANT ( 0xffffffffffffffffU ) typedef */

/* G_E 2.7182818284590452353602874713526624977572470937000 # */

/* G_LN2 0.69314718055994530941723212145817656807550013436026 # */

/* G_LN10 2.3025850929940456840179914546843642076011014886288 # */

/* G_PI 3.1415926535897932384626433832795028841971693993751 # */

/* G_PI_2 1.5707963267948966192313216916397514420985846996876 # */

/* G_PI_4 0.78539816339744830961566084581987572104929234984378 # */

/* G_SQRT2 1.4142135623730950488016887242096980785696718753769 /* Portable endian checks and conversions
 *
 * glibconfig.h defines G_BYTE_ORDER which expands to one of
 * the below macros.
 */ */

/* G_LITTLE_ENDIAN 1234 # */
pub const G_LITTLE_ENDIAN: i32 = 1234;

/* G_BIG_ENDIAN 4321 # */
pub const G_BIG_ENDIAN: i32 = 4321;

/* G_PDP_ENDIAN 3412 /* unused, need specific PDP check */ */
pub const G_PDP_ENDIAN: i32 = 3412;

/* GUINT16_SWAP_LE_BE_CONSTANT ( val ) ( ( guint16 ) ( ( guint16 ) ( ( guint16 ) ( val ) >> 8 ) | ( guint16 ) ( ( guint16 ) ( val ) << 8 ) ) ) # */

/* GUINT32_SWAP_LE_BE_CONSTANT ( val ) ( ( guint32 ) ( ( ( ( guint32 ) ( val ) & ( guint32 ) 0x000000ffU ) << 24 ) | ( ( ( guint32 ) ( val ) & ( guint32 ) 0x0000ff00U ) << 8 ) | ( ( ( guint32 ) ( val ) & ( guint32 ) 0x00ff0000U ) >> 8 ) | ( ( ( guint32 ) ( val ) & ( guint32 ) 0xff000000U ) >> 24 ) ) ) # */

/* GUINT64_SWAP_LE_BE_CONSTANT ( val ) ( ( guint64 ) ( ( ( ( guint64 ) ( val ) & ( guint64 ) G_GINT64_CONSTANT ( 0x00000000000000ffU ) ) << 56 ) | ( ( ( guint64 ) ( val ) & ( guint64 ) G_GINT64_CONSTANT ( 0x000000000000ff00U ) ) << 40 ) | ( ( ( guint64 ) ( val ) & ( guint64 ) G_GINT64_CONSTANT ( 0x0000000000ff0000U ) ) << 24 ) | ( ( ( guint64 ) ( val ) & ( guint64 ) G_GINT64_CONSTANT ( 0x00000000ff000000U ) ) << 8 ) | ( ( ( guint64 ) ( val ) & ( guint64 ) G_GINT64_CONSTANT ( 0x000000ff00000000U ) ) >> 8 ) | ( ( ( guint64 ) ( val ) & ( guint64 ) G_GINT64_CONSTANT ( 0x0000ff0000000000U ) ) >> 24 ) | ( ( ( guint64 ) ( val ) & ( guint64 ) G_GINT64_CONSTANT ( 0x00ff000000000000U ) ) >> 40 ) | ( ( ( guint64 ) ( val ) & ( guint64 ) G_GINT64_CONSTANT ( 0xff00000000000000U ) ) >> 56 ) ) ) /* Arch specific stuff for speed
 */ */

/* GUINT16_SWAP_LE_BE ( val ) ( GUINT16_SWAP_LE_BE_CONSTANT ( val ) ) # */

/* GUINT32_SWAP_LE_BE ( val ) ( GUINT32_SWAP_LE_BE_CONSTANT ( val ) ) # */

/* GUINT64_SWAP_LE_BE ( val ) ( GUINT64_SWAP_LE_BE_CONSTANT ( val ) ) # */

/* GUINT16_SWAP_LE_PDP ( val ) ( ( guint16 ) ( val ) ) # */

/* GUINT16_SWAP_BE_PDP ( val ) ( GUINT16_SWAP_LE_BE ( val ) ) # */

/* GUINT32_SWAP_LE_PDP ( val ) ( ( guint32 ) ( ( ( ( guint32 ) ( val ) & ( guint32 ) 0x0000ffffU ) << 16 ) | ( ( ( guint32 ) ( val ) & ( guint32 ) 0xffff0000U ) >> 16 ) ) ) # */

/* GUINT32_SWAP_BE_PDP ( val ) ( ( guint32 ) ( ( ( ( guint32 ) ( val ) & ( guint32 ) 0x00ff00ffU ) << 8 ) | ( ( ( guint32 ) ( val ) & ( guint32 ) 0xff00ff00U ) >> 8 ) ) ) /* The G*_TO_?E() macros are defined in glibconfig.h.
 * The transformation is symmetric, so the FROM just maps to the TO.
 */ */

/* GINT16_FROM_LE ( val ) ( GINT16_TO_LE ( val ) ) # */

/* GUINT16_FROM_LE ( val ) ( GUINT16_TO_LE ( val ) ) # */

/* GINT16_FROM_BE ( val ) ( GINT16_TO_BE ( val ) ) # */

/* GUINT16_FROM_BE ( val ) ( GUINT16_TO_BE ( val ) ) # */

/* GINT32_FROM_LE ( val ) ( GINT32_TO_LE ( val ) ) # */

/* GUINT32_FROM_LE ( val ) ( GUINT32_TO_LE ( val ) ) # */

/* GINT32_FROM_BE ( val ) ( GINT32_TO_BE ( val ) ) # */

/* GUINT32_FROM_BE ( val ) ( GUINT32_TO_BE ( val ) ) # */

/* GINT64_FROM_LE ( val ) ( GINT64_TO_LE ( val ) ) # */

/* GUINT64_FROM_LE ( val ) ( GUINT64_TO_LE ( val ) ) # */

/* GINT64_FROM_BE ( val ) ( GINT64_TO_BE ( val ) ) # */

/* GUINT64_FROM_BE ( val ) ( GUINT64_TO_BE ( val ) ) # */

/* GLONG_FROM_LE ( val ) ( GLONG_TO_LE ( val ) ) # */

/* GULONG_FROM_LE ( val ) ( GULONG_TO_LE ( val ) ) # */

/* GLONG_FROM_BE ( val ) ( GLONG_TO_BE ( val ) ) # */

/* GULONG_FROM_BE ( val ) ( GULONG_TO_BE ( val ) ) # */

/* GINT_FROM_LE ( val ) ( GINT_TO_LE ( val ) ) # */

/* GUINT_FROM_LE ( val ) ( GUINT_TO_LE ( val ) ) # */

/* GINT_FROM_BE ( val ) ( GINT_TO_BE ( val ) ) # */

/* GUINT_FROM_BE ( val ) ( GUINT_TO_BE ( val ) ) # */

/* GSIZE_FROM_LE ( val ) ( GSIZE_TO_LE ( val ) ) # */

/* GSSIZE_FROM_LE ( val ) ( GSSIZE_TO_LE ( val ) ) # */

/* GSIZE_FROM_BE ( val ) ( GSIZE_TO_BE ( val ) ) # */

/* GSSIZE_FROM_BE ( val ) ( GSSIZE_TO_BE ( val ) ) /* Portable versions of host-network order stuff
 */ */

/* g_ntohl ( val ) ( GUINT32_FROM_BE ( val ) ) # */

/* g_ntohs ( val ) ( GUINT16_FROM_BE ( val ) ) # */

/* g_htonl ( val ) ( GUINT32_TO_BE ( val ) ) # */

/* g_htons ( val ) ( GUINT16_TO_BE ( val ) ) /* IEEE Standard 754 Single Precision Storage Format (gfloat):
 *
 *        31 30           23 22            0
 * +--------+---------------+---------------+
 * | s 1bit | e[30:23] 8bit | f[22:0] 23bit |
 * +--------+---------------+---------------+
 * B0------------------->B1------->B2-->B3-->
 *
 * IEEE Standard 754 Double Precision Storage Format (gdouble):
 *
 *        63 62            52 51            32   31            0
 * +--------+----------------+----------------+ +---------------+
 * | s 1bit | e[62:52] 11bit | f[51:32] 20bit | | f[31:0] 32bit |
 * +--------+----------------+----------------+ +---------------+
 * B0--------------->B1---------->B2--->B3---->  B4->B5->B6->B7->
 */ */

/* G_IEEE754_FLOAT_BIAS ( 127 ) # */

/* G_IEEE754_DOUBLE_BIAS ( 1023 ) /* multiply with base2 exponent to get base10 exponent (normal numbers) */ */

/* G_LOG_2_BASE_10 ( 0.30102999566398119521 ) # */

/* GLIB_VAR extern # */

/* alloca ( size ) __builtin_alloca ( size ) # */

/* g_alloca ( size ) alloca ( size ) /**
 * g_newa:
 * @struct_type: Type of memory chunks to be allocated
 * @n_structs: Number of chunks to be allocated
 * 
 * Wraps g_alloca() in a more typesafe manner.
 * 
 * Returns: Pointer to stack space for @n_structs chunks of type @struct_type
 */ */

/* g_newa ( struct_type , n_structs ) ( ( struct_type * ) g_alloca ( sizeof ( struct_type ) * ( gsize ) ( n_structs ) ) ) # */

/* __G_ARRAY_H__ # */

/* g_array_append_val ( a , v ) g_array_append_vals ( a , & ( v ) , 1 ) # */

/* g_array_prepend_val ( a , v ) g_array_prepend_vals ( a , & ( v ) , 1 ) # */

/* g_array_insert_val ( a , i , v ) g_array_insert_vals ( a , i , & ( v ) , 1 ) # */

/* g_array_index ( a , t , i ) ( ( ( t * ) ( void * ) ( a ) -> data ) [ ( i ) ] ) GArray */

/* g_ptr_array_index ( array , index_ ) ( ( array ) -> pdata ) [ index_ ] GPtrArray */

/* __G_ASYNCQUEUE_H__ # */

/* __G_THREAD_H__ # */

/* __G_ATOMIC_H__ # */

/* g_atomic_int_get ( atomic ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gint ) ) ; ( void ) ( 0 ? * ( atomic ) ^ * ( atomic ) : 0 ) ; __sync_synchronize ( ) ; ( gint ) * ( atomic ) ; } ) ) # */

/* g_atomic_int_set ( atomic , newval ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gint ) ) ; ( void ) ( 0 ? * ( atomic ) ^ ( newval ) : 0 ) ; * ( atomic ) = ( newval ) ; __sync_synchronize ( ) ; } ) ) # */

/* g_atomic_int_inc ( atomic ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gint ) ) ; ( void ) ( 0 ? * ( atomic ) ^ * ( atomic ) : 0 ) ; ( void ) __sync_fetch_and_add ( ( atomic ) , 1 ) ; } ) ) # */

/* g_atomic_int_dec_and_test ( atomic ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gint ) ) ; ( void ) ( 0 ? * ( atomic ) ^ * ( atomic ) : 0 ) ; __sync_fetch_and_sub ( ( atomic ) , 1 ) == 1 ; } ) ) # */

/* g_atomic_int_compare_and_exchange ( atomic , oldval , newval ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gint ) ) ; ( void ) ( 0 ? * ( atomic ) ^ ( newval ) ^ ( oldval ) : 0 ) ; ( gboolean ) __sync_bool_compare_and_swap ( ( atomic ) , ( oldval ) , ( newval ) ) ; } ) ) # */

/* g_atomic_int_add ( atomic , val ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gint ) ) ; ( void ) ( 0 ? * ( atomic ) ^ ( val ) : 0 ) ; ( gint ) __sync_fetch_and_add ( ( atomic ) , ( val ) ) ; } ) ) # */

/* g_atomic_int_and ( atomic , val ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gint ) ) ; ( void ) ( 0 ? * ( atomic ) ^ ( val ) : 0 ) ; ( guint ) __sync_fetch_and_and ( ( atomic ) , ( val ) ) ; } ) ) # */

/* g_atomic_int_or ( atomic , val ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gint ) ) ; ( void ) ( 0 ? * ( atomic ) ^ ( val ) : 0 ) ; ( guint ) __sync_fetch_and_or ( ( atomic ) , ( val ) ) ; } ) ) # */

/* g_atomic_int_xor ( atomic , val ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gint ) ) ; ( void ) ( 0 ? * ( atomic ) ^ ( val ) : 0 ) ; ( guint ) __sync_fetch_and_xor ( ( atomic ) , ( val ) ) ; } ) ) # */

/* g_atomic_pointer_get ( atomic ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gpointer ) ) ; __sync_synchronize ( ) ; ( gpointer ) * ( atomic ) ; } ) ) # */

/* g_atomic_pointer_set ( atomic , newval ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gpointer ) ) ; ( void ) ( 0 ? ( gpointer ) * ( atomic ) : 0 ) ; * ( atomic ) = ( __typeof__ ( * ( atomic ) ) ) ( gsize ) ( newval ) ; __sync_synchronize ( ) ; } ) ) # */

/* g_atomic_pointer_compare_and_exchange ( atomic , oldval , newval ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gpointer ) ) ; ( void ) ( 0 ? ( gpointer ) * ( atomic ) : 0 ) ; ( gboolean ) __sync_bool_compare_and_swap ( ( atomic ) , ( oldval ) , ( newval ) ) ; } ) ) # */

/* g_atomic_pointer_add ( atomic , val ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gpointer ) ) ; ( void ) ( 0 ? ( gpointer ) * ( atomic ) : 0 ) ; ( void ) ( 0 ? ( val ) ^ ( val ) : 0 ) ; ( gssize ) __sync_fetch_and_add ( ( atomic ) , ( val ) ) ; } ) ) # */

/* g_atomic_pointer_and ( atomic , val ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gpointer ) ) ; ( void ) ( 0 ? ( gpointer ) * ( atomic ) : 0 ) ; ( void ) ( 0 ? ( val ) ^ ( val ) : 0 ) ; ( gsize ) __sync_fetch_and_and ( ( atomic ) , ( val ) ) ; } ) ) # */

/* g_atomic_pointer_or ( atomic , val ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gpointer ) ) ; ( void ) ( 0 ? ( gpointer ) * ( atomic ) : 0 ) ; ( void ) ( 0 ? ( val ) ^ ( val ) : 0 ) ; ( gsize ) __sync_fetch_and_or ( ( atomic ) , ( val ) ) ; } ) ) # */

/* g_atomic_pointer_xor ( atomic , val ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gpointer ) ) ; ( void ) ( 0 ? ( gpointer ) * ( atomic ) : 0 ) ; ( void ) ( 0 ? ( val ) ^ ( val ) : 0 ) ; ( gsize ) __sync_fetch_and_xor ( ( atomic ) , ( val ) ) ; } ) ) # */

/* __G_ERROR_H__ # */

/* __G_QUARK_H__ # */

/* G_THREAD_ERROR g_thread_error_quark ( ) GQuark */

/* G_PRIVATE_INIT ( notify ) { NULL , ( notify ) , { NULL , NULL } } struct */

/* G_ONCE_INIT { G_ONCE_STATUS_NOTCALLED , NULL } struct */

/* G_LOCK_NAME ( name ) g__ ## name ## _lock # */

/* G_LOCK_DEFINE_STATIC ( name ) static G_LOCK_DEFINE ( name ) # */

/* G_LOCK_DEFINE ( name ) GMutex G_LOCK_NAME ( name ) # */

/* G_LOCK_EXTERN ( name ) extern GMutex G_LOCK_NAME ( name ) # */

/* G_LOCK ( name ) g_mutex_lock ( & G_LOCK_NAME ( name ) ) # */

/* G_UNLOCK ( name ) g_mutex_unlock ( & G_LOCK_NAME ( name ) ) # */

/* G_TRYLOCK ( name ) g_mutex_trylock ( & G_LOCK_NAME ( name ) ) # */

/* g_once ( once , func , arg ) ( ( ( once ) -> status == G_ONCE_STATUS_READY ) ? ( once ) -> retval : g_once_impl ( ( once ) , ( func ) , ( arg ) ) ) # */

/* g_once_init_enter ( location ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( location ) == sizeof ( gpointer ) ) ; ( void ) ( 0 ? ( gpointer ) * ( location ) : 0 ) ; ( ! g_atomic_pointer_get ( location ) && g_once_init_enter ( location ) ) ; } ) ) # */

/* g_once_init_leave ( location , result ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( location ) == sizeof ( gpointer ) ) ; ( void ) ( 0 ? * ( location ) = ( result ) : 0 ) ; g_once_init_leave ( ( location ) , ( gsize ) ( result ) ) ; } ) ) # */

/* __G_BACKTRACE_H__ # */

/* G_BREAKPOINT ( ) G_STMT_START { __asm__ __volatile__ ( "int $03" ) ; } G_STMT_END # */

/* __G_BASE64_H__ # */

/* __G_BITLOCK_H__ # */

/* g_pointer_bit_lock ( address , lock_bit ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( address ) == sizeof ( gpointer ) ) ; g_pointer_bit_lock ( ( address ) , ( lock_bit ) ) ; } ) ) # */

/* g_pointer_bit_trylock ( address , lock_bit ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( address ) == sizeof ( gpointer ) ) ; g_pointer_bit_trylock ( ( address ) , ( lock_bit ) ) ; } ) ) # */

/* g_pointer_bit_unlock ( address , lock_bit ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( address ) == sizeof ( gpointer ) ) ; g_pointer_bit_unlock ( ( address ) , ( lock_bit ) ) ; } ) ) # */

/* __G_BOOKMARK_FILE_H__ # */

/* G_BOOKMARK_FILE_ERROR ( g_bookmark_file_error_quark ( ) ) /**
 * GBookmarkFileError:
 * @G_BOOKMARK_FILE_ERROR_INVALID_URI: URI was ill-formed
 * @G_BOOKMARK_FILE_ERROR_INVALID_VALUE: a requested field was not found
 * @G_BOOKMARK_FILE_ERROR_APP_NOT_REGISTERED: a requested application did
 *     not register a bookmark
 * @G_BOOKMARK_FILE_ERROR_URI_NOT_FOUND: a requested URI was not found
 * @G_BOOKMARK_FILE_ERROR_READ: document was ill formed
 * @G_BOOKMARK_FILE_ERROR_UNKNOWN_ENCODING: the text being parsed was
 *     in an unknown encoding
 * @G_BOOKMARK_FILE_ERROR_WRITE: an error occurred while writing
 * @G_BOOKMARK_FILE_ERROR_FILE_NOT_FOUND: requested file was not found
 *
 * Error codes returned by bookmark file parsing.
 */ */

/* __G_BYTES_H__ # */

/* __G_CHARSET_H__ # */

/* __G_CHECKSUM_H__ # */

/* __G_CONVERT_H__ # */

/* G_CONVERT_ERROR g_convert_error_quark ( ) GQuark */

/* __G_DATASET_H__ # */

/* G_DATALIST_FLAGS_MASK 0x3 void */
pub const G_DATALIST_FLAGS_MASK: i32 = 3;

/* g_datalist_id_set_data ( dl , q , d ) g_datalist_id_set_data_full ( ( dl ) , ( q ) , ( d ) , NULL ) # */

/* g_datalist_id_remove_data ( dl , q ) g_datalist_id_set_data ( ( dl ) , ( q ) , NULL ) # */

/* g_datalist_set_data_full ( dl , k , d , f ) g_datalist_id_set_data_full ( ( dl ) , g_quark_from_string ( k ) , ( d ) , ( f ) ) # */

/* g_datalist_remove_no_notify ( dl , k ) g_datalist_id_remove_no_notify ( ( dl ) , g_quark_try_string ( k ) ) # */

/* g_datalist_set_data ( dl , k , d ) g_datalist_set_data_full ( ( dl ) , ( k ) , ( d ) , NULL ) # */

/* g_datalist_remove_data ( dl , k ) g_datalist_id_set_data ( ( dl ) , g_quark_try_string ( k ) , NULL ) /* Location Associated Keyed Data
 */ */

/* g_dataset_id_set_data ( l , k , d ) g_dataset_id_set_data_full ( ( l ) , ( k ) , ( d ) , NULL ) # */

/* g_dataset_id_remove_data ( l , k ) g_dataset_id_set_data ( ( l ) , ( k ) , NULL ) # */

/* g_dataset_get_data ( l , k ) ( g_dataset_id_get_data ( ( l ) , g_quark_try_string ( k ) ) ) # */

/* g_dataset_set_data_full ( l , k , d , f ) g_dataset_id_set_data_full ( ( l ) , g_quark_from_string ( k ) , ( d ) , ( f ) ) # */

/* g_dataset_remove_no_notify ( l , k ) g_dataset_id_remove_no_notify ( ( l ) , g_quark_try_string ( k ) ) # */

/* g_dataset_set_data ( l , k , d ) g_dataset_set_data_full ( ( l ) , ( k ) , ( d ) , NULL ) # */

/* g_dataset_remove_data ( l , k ) g_dataset_id_set_data ( ( l ) , g_quark_try_string ( k ) , NULL ) G_END_DECLS */

/* __G_DATE_H__ # */

/* G_DATE_BAD_JULIAN 0U # */

/* G_DATE_BAD_DAY 0U # */

/* G_DATE_BAD_YEAR 0U /* Note: directly manipulating structs is generally a bad idea, but
 * in this case it's an *incredibly* bad idea, because all or part
 * of this struct can be invalid at any given time. Use the functions,
 * or you will get hosed, I promise.
 */ */

/* g_date_weekday g_date_get_weekday # */

/* g_date_month g_date_get_month # */

/* g_date_year g_date_get_year # */

/* g_date_day g_date_get_day # */

/* g_date_julian g_date_get_julian # */

/* g_date_day_of_year g_date_get_day_of_year # */

/* g_date_monday_week_of_year g_date_get_monday_week_of_year # */

/* g_date_sunday_week_of_year g_date_get_sunday_week_of_year # */

/* g_date_days_in_month g_date_get_days_in_month # */

/* g_date_monday_weeks_in_year g_date_get_monday_weeks_in_year # */

/* g_date_sunday_weeks_in_year g_date_get_sunday_weeks_in_year # */

/* __G_DATE_TIME_H__ # */

/* __G_TIME_ZONE_H__ # */

/* G_TIME_SPAN_DAY ( G_GINT64_CONSTANT ( 86400000000 ) ) /**
 * G_TIME_SPAN_HOUR:
 *
 * Evaluates to a time span of one hour.
 *
 * Since: 2.26
 */ */

/* G_TIME_SPAN_HOUR ( G_GINT64_CONSTANT ( 3600000000 ) ) /**
 * G_TIME_SPAN_MINUTE:
 *
 * Evaluates to a time span of one minute.
 *
 * Since: 2.26
 */ */

/* G_TIME_SPAN_MINUTE ( G_GINT64_CONSTANT ( 60000000 ) ) /**
 * G_TIME_SPAN_SECOND:
 *
 * Evaluates to a time span of one second.
 *
 * Since: 2.26
 */ */

/* G_TIME_SPAN_SECOND ( G_GINT64_CONSTANT ( 1000000 ) ) /**
 * G_TIME_SPAN_MILLISECOND:
 *
 * Evaluates to a time span of one millisecond.
 *
 * Since: 2.26
 */ */

/* G_TIME_SPAN_MILLISECOND ( G_GINT64_CONSTANT ( 1000 ) ) /**
 * GTimeSpan:
 *
 * A value representing an interval of time, in microseconds.
 *
 * Since: 2.26
 */ */

/* __G_DIR_H__ # */

/* __G_ENVIRON_H__ # */

/* __G_FILEUTILS_H__ # */

/* G_FILE_ERROR g_file_error_quark ( ) typedef */

/* G_DIR_SEPARATOR '/' # */

/* G_DIR_SEPARATOR_S "/" # */

/* G_IS_DIR_SEPARATOR ( c ) ( ( c ) == G_DIR_SEPARATOR ) # */

/* G_SEARCHPATH_SEPARATOR ':' # */

/* G_SEARCHPATH_SEPARATOR_S ":" # */

/* g_dirname g_path_get_dirname # */

/* __G_GETTEXT_H__ # */

/* __G_HASH_H__ # */

/* __G_LIST_H__ # */

/* __G_MEM_H__ # */

/* G_MEM_ALIGN GLIB_SIZEOF_LONG # */

/* _G_NEW ( struct_type , n_structs , func ) ( ( struct_type * ) g_ ## func ## _n ( ( n_structs ) , sizeof ( struct_type ) ) ) # */

/* _G_RENEW ( struct_type , mem , n_structs , func ) ( ( struct_type * ) g_ ## func ## _n ( mem , ( n_structs ) , sizeof ( struct_type ) ) ) # */

/* g_new ( struct_type , n_structs ) _G_NEW ( struct_type , n_structs , malloc ) /**
 * g_new0:
 * @struct_type: the type of the elements to allocate.
 * @n_structs: the number of elements to allocate.
 * 
 * Allocates @n_structs elements of type @struct_type, initialized to 0's.
 * The returned pointer is cast to a pointer to the given type.
 * If @n_structs is 0 it returns %NULL.
 * Care is taken to avoid overflow when calculating the size of the allocated block.
 * 
 * Since the returned pointer is already casted to the right type,
 * it is normally unnecessary to cast it explicitly, and doing
 * so might hide memory allocation errors.
 * 
 * Returns: a pointer to the allocated memory, cast to a pointer to @struct_type.
 */ */

/* g_new0 ( struct_type , n_structs ) _G_NEW ( struct_type , n_structs , malloc0 ) /**
 * g_renew:
 * @struct_type: the type of the elements to allocate
 * @mem: the currently allocated memory
 * @n_structs: the number of elements to allocate
 * 
 * Reallocates the memory pointed to by @mem, so that it now has space for
 * @n_structs elements of type @struct_type. It returns the new address of
 * the memory, which may have been moved.
 * Care is taken to avoid overflow when calculating the size of the allocated block.
 * 
 * Returns: a pointer to the new allocated memory, cast to a pointer to @struct_type
 */ */

/* g_renew ( struct_type , mem , n_structs ) _G_RENEW ( struct_type , mem , n_structs , realloc ) /**
 * g_try_new:
 * @struct_type: the type of the elements to allocate
 * @n_structs: the number of elements to allocate
 * 
 * Attempts to allocate @n_structs elements of type @struct_type, and returns
 * %NULL on failure. Contrast with g_new(), which aborts the program on failure.
 * The returned pointer is cast to a pointer to the given type.
 * The function returns %NULL when @n_structs is 0 of if an overflow occurs.
 * 
 * Since: 2.8
 * Returns: a pointer to the allocated memory, cast to a pointer to @struct_type
 */ */

/* g_try_new ( struct_type , n_structs ) _G_NEW ( struct_type , n_structs , try_malloc ) /**
 * g_try_new0:
 * @struct_type: the type of the elements to allocate
 * @n_structs: the number of elements to allocate
 * 
 * Attempts to allocate @n_structs elements of type @struct_type, initialized
 * to 0's, and returns %NULL on failure. Contrast with g_new0(), which aborts
 * the program on failure.
 * The returned pointer is cast to a pointer to the given type.
 * The function returns %NULL when @n_structs is 0 of if an overflow occurs.
 * 
 * Since: 2.8
 * Returns: a pointer to the allocated memory, cast to a pointer to @struct_type
 */ */

/* g_try_new0 ( struct_type , n_structs ) _G_NEW ( struct_type , n_structs , try_malloc0 ) /**
 * g_try_renew:
 * @struct_type: the type of the elements to allocate
 * @mem: the currently allocated memory
 * @n_structs: the number of elements to allocate
 * 
 * Attempts to reallocate the memory pointed to by @mem, so that it now has
 * space for @n_structs elements of type @struct_type, and returns %NULL on
 * failure. Contrast with g_renew(), which aborts the program on failure.
 * It returns the new address of the memory, which may have been moved.
 * The function returns %NULL if an overflow occurs.
 * 
 * Since: 2.8
 * Returns: a pointer to the new allocated memory, cast to a pointer to @struct_type
 */ */

/* g_try_renew ( struct_type , mem , n_structs ) _G_RENEW ( struct_type , mem , n_structs , try_realloc ) /* Memory allocation virtualization for debugging purposes
 * g_mem_set_vtable() has to be the very first GLib function called
 * if being used
 */ */

/* g_list_free1 g_list_free_1 void */

/* g_list_previous ( list ) ( ( list ) ? ( ( ( GList * ) ( list ) ) -> prev ) : NULL ) # */

/* g_list_next ( list ) ( ( list ) ? ( ( ( GList * ) ( list ) ) -> next ) : NULL ) G_END_DECLS */

/* g_hash_table_freeze ( hash_table ) ( ( void ) 0 ) # */

/* g_hash_table_thaw ( hash_table ) ( ( void ) 0 ) # */

/* __G_HMAC_H__ # */

/* __G_HOOK_H__ # */

/* G_HOOK_FLAG_USER_SHIFT ( 4 ) /* --- structures --- */ */

/* G_HOOK ( hook ) ( ( GHook * ) ( hook ) ) # */

/* G_HOOK_FLAGS ( hook ) ( G_HOOK ( hook ) -> flags ) # */

/* G_HOOK_ACTIVE ( hook ) ( ( G_HOOK_FLAGS ( hook ) & G_HOOK_FLAG_ACTIVE ) != 0 ) # */

/* G_HOOK_IN_CALL ( hook ) ( ( G_HOOK_FLAGS ( hook ) & G_HOOK_FLAG_IN_CALL ) != 0 ) # */

/* G_HOOK_IS_VALID ( hook ) ( G_HOOK ( hook ) -> hook_id != 0 && ( G_HOOK_FLAGS ( hook ) & G_HOOK_FLAG_ACTIVE ) ) # */

/* G_HOOK_IS_UNLINKED ( hook ) ( G_HOOK ( hook ) -> next == NULL && G_HOOK ( hook ) -> prev == NULL && G_HOOK ( hook ) -> hook_id == 0 && G_HOOK ( hook ) -> ref_count == 0 ) /* --- prototypes --- */ */

/* g_hook_append ( hook_list , hook ) g_hook_insert_before ( ( hook_list ) , NULL , ( hook ) ) /* invoke all valid hooks with the (*GHookFunc) signature.
 */ */

/* __G_HOST_UTILS_H__ # */

/* __G_IOCHANNEL_H__ # */

/* __G_MAIN_H__ # */

/* __G_POLL_H__ # */

/* G_POLLFD_FORMAT "%d" # */

/* __G_SLIST_H__ # */

/* g_slist_free1 g_slist_free_1 void */

/* g_slist_next ( slist ) ( ( slist ) ? ( ( ( GSList * ) ( slist ) ) -> next ) : NULL ) G_END_DECLS */

/* G_PRIORITY_HIGH - 100 /**
 * G_PRIORITY_DEFAULT:
 *
 * Use this for default priority event sources.
 *
 * In GLib this priority is used when adding timeout functions
 * with g_timeout_add(). In GDK this priority is used for events
 * from the X server.
 */ */
pub const G_PRIORITY_HIGH: i32 = -100;

/* G_PRIORITY_DEFAULT 0 /**
 * G_PRIORITY_HIGH_IDLE:
 *
 * Use this for high priority idle functions.
 *
 * GTK+ uses #G_PRIORITY_HIGH_IDLE + 10 for resizing operations,
 * and #G_PRIORITY_HIGH_IDLE + 20 for redrawing operations. (This is
 * done to ensure that any pending resizes are processed before any
 * pending redraws, so that widgets are not redrawn twice unnecessarily.)
 */ */
pub const G_PRIORITY_DEFAULT: i32 = 0;

/* G_PRIORITY_HIGH_IDLE 100 /**
 * G_PRIORITY_DEFAULT_IDLE:
 *
 * Use this for default priority idle functions.
 *
 * In GLib this priority is used when adding idle functions with
 * g_idle_add().
 */ */
pub const G_PRIORITY_HIGH_IDLE: i32 = 100;

/* G_PRIORITY_DEFAULT_IDLE 200 /**
 * G_PRIORITY_LOW:
 *
 * Use this for very low priority background tasks.
 *
 * It is not used within GLib or GTK+.
 */ */
pub const G_PRIORITY_DEFAULT_IDLE: i32 = 200;

/* G_PRIORITY_LOW 300 /**
 * G_SOURCE_REMOVE:
 *
 * Use this macro as the return value of a #GSourceFunc to remove
 * the #GSource from the main loop.
 *
 * Since: 2.32
 */ */
pub const G_PRIORITY_LOW: i32 = 300;

/* G_SOURCE_REMOVE FALSE /**
 * G_SOURCE_CONTINUE:
 *
 * Use this macro as the return value of a #GSourceFunc to leave
 * the #GSource in the main loop.
 *
 * Since: 2.32
 */ */

/* G_SOURCE_CONTINUE TRUE /* GMainContext: */ */

/* __G_STRING_H__ # */

/* __G_UNICODE_H__ # */

/* G_UNICODE_COMBINING_MARK G_UNICODE_SPACING_MARK # */

/* G_UNICHAR_MAX_DECOMPOSITION_LENGTH 18 /* codepoints */ */
pub const G_UNICHAR_MAX_DECOMPOSITION_LENGTH: i32 = 18;

/* g_utf8_next_char ( p ) ( char * ) ( ( p ) + g_utf8_skip [ * ( const guchar * ) ( p ) ] ) gunichar */

/* __G_UTILS_H__ # */

/* G_INLINE_FUNC static __inline __attribute__ ( ( unused ) ) # */

/* ATEXIT ( proc ) g_ATEXIT ( proc ) # */

/* G_WIN32_DLLMAIN_FOR_DLL_NAME ( static , dll_name ) # */

/* g_string_append_c ( gstr , c ) g_string_append_c_inline ( gstr , c ) # */

/* g_string_sprintf g_string_printf # */

/* g_string_sprintfa g_string_append_printf # */

/* G_IO_CHANNEL_ERROR g_io_channel_error_quark ( ) typedef */

/* G_IO_FLAG_IS_WRITEABLE ( G_IO_FLAG_IS_WRITABLE ) struct */

/* __G_KEY_FILE_H__ # */

/* G_KEY_FILE_ERROR g_key_file_error_quark ( ) GQuark */

/* G_KEY_FILE_DESKTOP_GROUP "Desktop Entry" # */

/* G_KEY_FILE_DESKTOP_KEY_TYPE "Type" # */

/* G_KEY_FILE_DESKTOP_KEY_VERSION "Version" # */

/* G_KEY_FILE_DESKTOP_KEY_NAME "Name" # */

/* G_KEY_FILE_DESKTOP_KEY_GENERIC_NAME "GenericName" # */

/* G_KEY_FILE_DESKTOP_KEY_NO_DISPLAY "NoDisplay" # */

/* G_KEY_FILE_DESKTOP_KEY_COMMENT "Comment" # */

/* G_KEY_FILE_DESKTOP_KEY_ICON "Icon" # */

/* G_KEY_FILE_DESKTOP_KEY_HIDDEN "Hidden" # */

/* G_KEY_FILE_DESKTOP_KEY_ONLY_SHOW_IN "OnlyShowIn" # */

/* G_KEY_FILE_DESKTOP_KEY_NOT_SHOW_IN "NotShowIn" # */

/* G_KEY_FILE_DESKTOP_KEY_TRY_EXEC "TryExec" # */

/* G_KEY_FILE_DESKTOP_KEY_EXEC "Exec" # */

/* G_KEY_FILE_DESKTOP_KEY_PATH "Path" # */

/* G_KEY_FILE_DESKTOP_KEY_TERMINAL "Terminal" # */

/* G_KEY_FILE_DESKTOP_KEY_MIME_TYPE "MimeType" # */

/* G_KEY_FILE_DESKTOP_KEY_CATEGORIES "Categories" # */

/* G_KEY_FILE_DESKTOP_KEY_STARTUP_NOTIFY "StartupNotify" # */

/* G_KEY_FILE_DESKTOP_KEY_STARTUP_WM_CLASS "StartupWMClass" # */

/* G_KEY_FILE_DESKTOP_KEY_URL "URL" # */

/* G_KEY_FILE_DESKTOP_KEY_GETTEXT_DOMAIN "X-GNOME-Gettext-Domain" # */

/* G_KEY_FILE_DESKTOP_KEY_FULLNAME "X-GNOME-FullName" # */

/* G_KEY_FILE_DESKTOP_KEY_KEYWORDS "Keywords" # */

/* G_KEY_FILE_DESKTOP_TYPE_APPLICATION "Application" # */

/* G_KEY_FILE_DESKTOP_TYPE_LINK "Link" # */

/* G_KEY_FILE_DESKTOP_TYPE_DIRECTORY "Directory" G_END_DECLS */

/* __G_MAPPED_FILE_H__ # */

/* __G_MARKUP_H__ # */

/* G_MARKUP_ERROR g_markup_error_quark ( ) GQuark */

/* __G_MESSAGES_H__ # */

/* G_LOG_LEVEL_USER_SHIFT ( 8 ) /* Glib log levels and flags.
 */ */

/* G_LOG_FATAL_MASK ( G_LOG_FLAG_RECURSION | G_LOG_LEVEL_ERROR ) typedef */

/* G_LOG_DOMAIN ( ( gchar * ) 0 ) # */

/* g_error ( ... ) G_STMT_START { g_log ( G_LOG_DOMAIN , G_LOG_LEVEL_ERROR , __VA_ARGS__ ) ; for ( ; ; ) ; } G_STMT_END # */

/* g_message ( ... ) g_log ( G_LOG_DOMAIN , G_LOG_LEVEL_MESSAGE , __VA_ARGS__ ) # */

/* g_critical ( ... ) g_log ( G_LOG_DOMAIN , G_LOG_LEVEL_CRITICAL , __VA_ARGS__ ) # */

/* g_warning ( ... ) g_log ( G_LOG_DOMAIN , G_LOG_LEVEL_WARNING , __VA_ARGS__ ) # */

/* g_debug ( ... ) g_log ( G_LOG_DOMAIN , G_LOG_LEVEL_DEBUG , __VA_ARGS__ ) # */

/* g_warn_if_reached ( ) do { g_warn_message ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , NULL ) ; } while ( 0 ) /**
 * g_warn_if_fail:
 * @expr: the expression to check
 *
 * Logs a warning if the expression is not true.
 *
 * Since: 2.16
 */ */

/* g_warn_if_fail ( expr ) do { if G_LIKELY ( expr ) ; else g_warn_message ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , # expr ) ; } while ( 0 ) # */

/* g_return_if_fail ( expr ) G_STMT_START { if G_LIKELY ( expr ) { } else { g_return_if_fail_warning ( G_LOG_DOMAIN , __PRETTY_FUNCTION__ , # expr ) ; return ; } ; } G_STMT_END # */

/* g_return_val_if_fail ( expr , val ) G_STMT_START { if G_LIKELY ( expr ) { } else { g_return_if_fail_warning ( G_LOG_DOMAIN , __PRETTY_FUNCTION__ , # expr ) ; return ( val ) ; } ; } G_STMT_END # */

/* g_return_if_reached ( ) G_STMT_START { g_log ( G_LOG_DOMAIN , G_LOG_LEVEL_CRITICAL , "file %s: line %d (%s): should not be reached" , __FILE__ , __LINE__ , __PRETTY_FUNCTION__ ) ; return ; } G_STMT_END # */

/* g_return_val_if_reached ( val ) G_STMT_START { g_log ( G_LOG_DOMAIN , G_LOG_LEVEL_CRITICAL , "file %s: line %d (%s): should not be reached" , __FILE__ , __LINE__ , __PRETTY_FUNCTION__ ) ; return ( val ) ; } G_STMT_END # */

/* __G_NODE_H__ # */

/* G_NODE_IS_ROOT ( node ) ( ( ( GNode * ) ( node ) ) -> parent == NULL && ( ( GNode * ) ( node ) ) -> prev == NULL && ( ( GNode * ) ( node ) ) -> next == NULL ) /**
 * G_NODE_IS_LEAF:
 * @node: a #GNode
 *
 * Returns %TRUE if a #GNode is a leaf node.
 *
 * Returns: %TRUE if the #GNode is a leaf node 
 *     (i.e. it has no children)
 */ */

/* G_NODE_IS_LEAF ( node ) ( ( ( GNode * ) ( node ) ) -> children == NULL ) GNode */

/* g_node_append ( parent , node ) g_node_insert_before ( ( parent ) , NULL , ( node ) ) /**
 * g_node_insert_data:
 * @parent: the #GNode to place the new #GNode under
 * @position: the position to place the new #GNode at. If position is -1, 
 *     the new #GNode is inserted as the last child of @parent
 * @data: the data for the new #GNode
 *
 * Inserts a new #GNode at the given position.
 *
 * Returns: the new #GNode
 */ */

/* g_node_insert_data ( parent , position , data ) g_node_insert ( ( parent ) , ( position ) , g_node_new ( data ) ) /**
 * g_node_insert_data_after:
 * @parent: the #GNode to place the new #GNode under
 * @sibling: the sibling #GNode to place the new #GNode after
 * @data: the data for the new #GNode
 *
 * Inserts a new #GNode after the given sibling.
 *
 * Returns: the new #GNode
 */ */

/* g_node_insert_data_after ( parent , sibling , data ) g_node_insert_after ( ( parent ) , ( sibling ) , g_node_new ( data ) ) /**
 * g_node_insert_data_before:
 * @parent: the #GNode to place the new #GNode under
 * @sibling: the sibling #GNode to place the new #GNode before
 * @data: the data for the new #GNode
 *
 * Inserts a new #GNode before the given sibling.
 *
 * Returns: the new #GNode
 */ */

/* g_node_insert_data_before ( parent , sibling , data ) g_node_insert_before ( ( parent ) , ( sibling ) , g_node_new ( data ) ) /**
 * g_node_prepend_data:
 * @parent: the #GNode to place the new #GNode under
 * @data: the data for the new #GNode
 *
 * Inserts a new #GNode as the first child of the given parent.
 *
 * Returns: the new #GNode
 */ */

/* g_node_prepend_data ( parent , data ) g_node_prepend ( ( parent ) , g_node_new ( data ) ) /**
 * g_node_append_data:
 * @parent: the #GNode to place the new #GNode under
 * @data: the data for the new #GNode
 *
 * Inserts a new #GNode as the last child of the given parent.
 *
 * Returns: the new #GNode
 */ */

/* g_node_append_data ( parent , data ) g_node_insert_before ( ( parent ) , NULL , g_node_new ( data ) ) /* traversal function, assumes that `node' is root
 * (only traverses `node' and its subtree).
 * this function is just a high level interface to
 * low level traversal functions, optimized for speed.
 */ */

/* g_node_prev_sibling ( node ) ( ( node ) ? ( ( GNode * ) ( node ) ) -> prev : NULL ) /**
 * g_node_next_sibling:
 * @node: a #GNode
 *
 * Gets the next sibling of a #GNode.
 *
 * Returns: the next sibling of @node, or %NULL if @node is the last node
 *     or %NULL
 */ */

/* g_node_next_sibling ( node ) ( ( node ) ? ( ( GNode * ) ( node ) ) -> next : NULL ) /**
 * g_node_first_child:
 * @node: a #GNode
 *
 * Gets the first child of a #GNode.
 *
 * Returns: the first child of @node, or %NULL if @node is %NULL 
 *     or has no children
 */ */

/* g_node_first_child ( node ) ( ( node ) ? ( ( GNode * ) ( node ) ) -> children : NULL ) G_END_DECLS */

/* __G_OPTION_H__ # */

/* G_OPTION_ERROR ( g_option_error_quark ( ) ) /**
 * GOptionError:
 * @G_OPTION_ERROR_UNKNOWN_OPTION: An option was not known to the parser.
 *  This error will only be reported, if the parser hasn't been instructed
 *  to ignore unknown options, see g_option_context_set_ignore_unknown_options().
 * @G_OPTION_ERROR_BAD_VALUE: A value couldn't be parsed.
 * @G_OPTION_ERROR_FAILED: A #GOptionArgFunc callback failed.
 * 
 * Error codes returned by option parsing.
 */ */

/* G_OPTION_REMAINING "" GOptionContext */

/* __G_PATTERN_H__ # */

/* __G_PRIMES_H__ # */

/* __G_QSORT_H__ # */

/* __G_QUEUE_H__ # */

/* G_QUEUE_INIT { NULL , NULL , 0 } /* Queues
 */ */

/* __G_RAND_H__ # */

/* g_rand_boolean ( rand_ ) ( ( g_rand_int ( rand_ ) & ( 1 << 15 ) ) != 0 ) guint32 */

/* g_random_boolean ( ) ( ( g_random_int ( ) & ( 1 << 15 ) ) != 0 ) guint32 */

/* __G_REGEX_H__ # */

/* G_REGEX_ERROR g_regex_error_quark ( ) GQuark */

/* __G_SCANNER_H__ # */

/* G_CSET_A_2_Z "ABCDEFGHIJKLMNOPQRSTUVWXYZ" # */

/* G_CSET_a_2_z "abcdefghijklmnopqrstuvwxyz" # */

/* G_CSET_DIGITS "0123456789" # */

/* G_CSET_LATINC "\300\301\302\303\304\305\306" "\307\310\311\312\313\314\315\316\317\320" "\321\322\323\324\325\326" "\330\331\332\333\334\335\336" # */

/* G_CSET_LATINS "\337\340\341\342\343\344\345\346" "\347\350\351\352\353\354\355\356\357\360" "\361\362\363\364\365\366" "\370\371\372\373\374\375\376\377" /* Error types */ */

/* g_scanner_add_symbol ( scanner , symbol , value ) G_STMT_START { g_scanner_scope_add_symbol ( ( scanner ) , 0 , ( symbol ) , ( value ) ) ; \
} G_STMT_END # */

/* g_scanner_remove_symbol ( scanner , symbol ) G_STMT_START { g_scanner_scope_remove_symbol ( ( scanner ) , 0 , ( symbol ) ) ; \
} G_STMT_END # */

/* g_scanner_foreach_symbol ( scanner , func , data ) G_STMT_START { g_scanner_scope_foreach_symbol ( ( scanner ) , 0 , ( func ) , ( data ) ) ; \
} G_STMT_END /* The following two functions are deprecated and will be removed in
 * the next major release. They do no good. */ */

/* g_scanner_freeze_symbol_table ( scanner ) ( ( void ) 0 ) # */

/* g_scanner_thaw_symbol_table ( scanner ) ( ( void ) 0 ) # */

/* __G_SEQUENCE_H__ # */

/* __G_SHELL_H__ # */

/* G_SHELL_ERROR g_shell_error_quark ( ) typedef */

/* __G_SLICE_H__ # */

/* g_slice_new ( type ) ( ( type * ) g_slice_alloc ( sizeof ( type ) ) ) # */

/* g_slice_new0 ( type ) ( ( type * ) g_slice_alloc0 ( sizeof ( type ) ) ) /* MemoryBlockType *
 *       g_slice_dup                    (MemoryBlockType,
 *	                                 MemoryBlockType *mem_block);
 *       g_slice_free                   (MemoryBlockType,
 *	                                 MemoryBlockType *mem_block);
 *       g_slice_free_chain             (MemoryBlockType,
 *                                       MemoryBlockType *first_chain_block,
 *                                       memory_block_next_field);
 * pseudo prototypes for the macro
 * definitions following below.
 */ */

/* g_slice_dup ( type , mem ) ( 1 ? ( type * ) g_slice_copy ( sizeof ( type ) , ( mem ) ) : ( ( void ) ( ( type * ) 0 == ( mem ) ) , ( type * ) 0 ) ) # */

/* g_slice_free ( type , mem ) do { if ( 1 ) g_slice_free1 ( sizeof ( type ) , ( mem ) ) ; else ( void ) ( ( type * ) 0 == ( mem ) ) ; \
} while ( 0 ) # */

/* g_slice_free_chain ( type , mem_chain , next ) do { if ( 1 ) g_slice_free_chain_with_offset ( sizeof ( type ) , ( mem_chain ) , G_STRUCT_OFFSET ( type , next ) ) ; else ( void ) ( ( type * ) 0 == ( mem_chain ) ) ; \
} while ( 0 ) /* --- internal debugging API --- */ */

/* __G_SPAWN_H__ # */

/* G_SPAWN_ERROR g_spawn_error_quark ( ) /**
 * GSpawnError:
 * @G_SPAWN_ERROR_FORK: Fork failed due to lack of memory.
 * @G_SPAWN_ERROR_READ: Read or select on pipes failed.
 * @G_SPAWN_ERROR_CHDIR: Changing to working directory failed.
 * @G_SPAWN_ERROR_ACCES: execv() returned <literal>EACCES</literal>
 * @G_SPAWN_ERROR_PERM: execv() returned <literal>EPERM</literal>
 * @G_SPAWN_ERROR_TOO_BIG: execv() returned <literal>E2BIG</literal>
 * @G_SPAWN_ERROR_2BIG: deprecated alias for %G_SPAWN_ERROR_TOO_BIG
 * @G_SPAWN_ERROR_NOEXEC: execv() returned <literal>ENOEXEC</literal>
 * @G_SPAWN_ERROR_NAMETOOLONG: execv() returned <literal>ENAMETOOLONG</literal>
 * @G_SPAWN_ERROR_NOENT: execv() returned <literal>ENOENT</literal>
 * @G_SPAWN_ERROR_NOMEM: execv() returned <literal>ENOMEM</literal>
 * @G_SPAWN_ERROR_NOTDIR: execv() returned <literal>ENOTDIR</literal>
 * @G_SPAWN_ERROR_LOOP: execv() returned <literal>ELOOP</literal>
 * @G_SPAWN_ERROR_TXTBUSY: execv() returned <literal>ETXTBUSY</literal>
 * @G_SPAWN_ERROR_IO: execv() returned <literal>EIO</literal>
 * @G_SPAWN_ERROR_NFILE: execv() returned <literal>ENFILE</literal>
 * @G_SPAWN_ERROR_MFILE: execv() returned <literal>EMFILE</literal>
 * @G_SPAWN_ERROR_INVAL: execv() returned <literal>EINVAL</literal>
 * @G_SPAWN_ERROR_ISDIR: execv() returned <literal>EISDIR</literal>
 * @G_SPAWN_ERROR_LIBBAD: execv() returned <literal>ELIBBAD</literal>
 * @G_SPAWN_ERROR_FAILED: Some other fatal failure,
 *   <literal>error-&gt;message</literal> should explain.
 *
 * Error codes returned by spawning processes.
 */ */

/* __G_STRFUNCS_H__ # */

/* g_ascii_isalnum ( c ) ( ( g_ascii_table [ ( guchar ) ( c ) ] & G_ASCII_ALNUM ) != 0 ) # */

/* g_ascii_isalpha ( c ) ( ( g_ascii_table [ ( guchar ) ( c ) ] & G_ASCII_ALPHA ) != 0 ) # */

/* g_ascii_iscntrl ( c ) ( ( g_ascii_table [ ( guchar ) ( c ) ] & G_ASCII_CNTRL ) != 0 ) # */

/* g_ascii_isdigit ( c ) ( ( g_ascii_table [ ( guchar ) ( c ) ] & G_ASCII_DIGIT ) != 0 ) # */

/* g_ascii_isgraph ( c ) ( ( g_ascii_table [ ( guchar ) ( c ) ] & G_ASCII_GRAPH ) != 0 ) # */

/* g_ascii_islower ( c ) ( ( g_ascii_table [ ( guchar ) ( c ) ] & G_ASCII_LOWER ) != 0 ) # */

/* g_ascii_isprint ( c ) ( ( g_ascii_table [ ( guchar ) ( c ) ] & G_ASCII_PRINT ) != 0 ) # */

/* g_ascii_ispunct ( c ) ( ( g_ascii_table [ ( guchar ) ( c ) ] & G_ASCII_PUNCT ) != 0 ) # */

/* g_ascii_isspace ( c ) ( ( g_ascii_table [ ( guchar ) ( c ) ] & G_ASCII_SPACE ) != 0 ) # */

/* g_ascii_isupper ( c ) ( ( g_ascii_table [ ( guchar ) ( c ) ] & G_ASCII_UPPER ) != 0 ) # */

/* g_ascii_isxdigit ( c ) ( ( g_ascii_table [ ( guchar ) ( c ) ] & G_ASCII_XDIGIT ) != 0 ) gchar */

/* G_STR_DELIMITERS "_-|> <." gchar */

/* G_ASCII_DTOSTR_BUF_SIZE ( 29 + 10 ) gchar */

/* g_strstrip ( string ) g_strchomp ( g_strchug ( string ) ) gint */

/* __G_STRINGCHUNK_H__ # */

/* __G_TEST_UTILS_H__ # */

/* g_assert_cmpstr ( s1 , cmp , s2 ) do { const char * __s1 = ( s1 ) , * __s2 = ( s2 ) ; if ( g_strcmp0 ( __s1 , __s2 ) cmp 0 ) ; else g_assertion_message_cmpstr ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , # s1 " " # cmp " " # s2 , __s1 , # cmp , __s2 ) ; } while ( 0 ) # */

/* g_assert_cmpint ( n1 , cmp , n2 ) do { gint64 __n1 = ( n1 ) , __n2 = ( n2 ) ; if ( __n1 cmp __n2 ) ; else g_assertion_message_cmpnum ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , # n1 " " # cmp " " # n2 , __n1 , # cmp , __n2 , 'i' ) ; } while ( 0 ) # */

/* g_assert_cmpuint ( n1 , cmp , n2 ) do { guint64 __n1 = ( n1 ) , __n2 = ( n2 ) ; if ( __n1 cmp __n2 ) ; else g_assertion_message_cmpnum ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , # n1 " " # cmp " " # n2 , __n1 , # cmp , __n2 , 'i' ) ; } while ( 0 ) # */

/* g_assert_cmphex ( n1 , cmp , n2 ) do { guint64 __n1 = ( n1 ) , __n2 = ( n2 ) ; if ( __n1 cmp __n2 ) ; else g_assertion_message_cmpnum ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , # n1 " " # cmp " " # n2 , __n1 , # cmp , __n2 , 'x' ) ; } while ( 0 ) # */

/* g_assert_cmpfloat ( n1 , cmp , n2 ) do { long double __n1 = ( n1 ) , __n2 = ( n2 ) ; if ( __n1 cmp __n2 ) ; else g_assertion_message_cmpnum ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , # n1 " " # cmp " " # n2 , __n1 , # cmp , __n2 , 'f' ) ; } while ( 0 ) # */

/* g_assert_no_error ( err ) do { if ( err ) g_assertion_message_error ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , # err , err , 0 , 0 ) ; } while ( 0 ) # */

/* g_assert_error ( err , dom , c ) do { if ( ! err || ( err ) -> domain != dom || ( err ) -> code != c ) g_assertion_message_error ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , # err , err , dom , c ) ; } while ( 0 ) # */

/* g_assert_not_reached ( ) do { g_assertion_message ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , NULL ) ; } while ( 0 ) # */

/* g_assert ( expr ) do { if G_LIKELY ( expr ) ; else g_assertion_message_expr ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , # expr ) ; } while ( 0 ) # */

/* g_test_quick ( ) ( g_test_config_vars -> test_quick ) # */

/* g_test_slow ( ) ( ! g_test_config_vars -> test_quick ) # */

/* g_test_thorough ( ) ( ! g_test_config_vars -> test_quick ) # */

/* g_test_perf ( ) ( g_test_config_vars -> test_perf ) # */

/* g_test_verbose ( ) ( g_test_config_vars -> test_verbose ) # */

/* g_test_quiet ( ) ( g_test_config_vars -> test_quiet ) # */

/* g_test_undefined ( ) ( g_test_config_vars -> test_undefined ) /* run all tests under toplevel suite (path: /) */ */

/* g_test_add ( testpath , Fixture , tdata , fsetup , ftest , fteardown ) G_STMT_START { void ( * add_vtable ) ( const char * , gsize , gconstpointer , void ( * ) ( Fixture * , gconstpointer ) , void ( * ) ( Fixture * , gconstpointer ) , void ( * ) ( Fixture * , gconstpointer ) ) = ( void ( * ) ( const gchar * , gsize , gconstpointer , void ( * ) ( Fixture * , gconstpointer ) , void ( * ) ( Fixture * , gconstpointer ) , void ( * ) ( Fixture * , gconstpointer ) ) ) g_test_add_vtable ; add_vtable ( testpath , sizeof ( Fixture ) , tdata , fsetup , ftest , fteardown ) ; } G_STMT_END /* add test messages to the test report */ */

/* g_test_queue_unref ( gobject ) g_test_queue_destroy ( g_object_unref , gobject ) /* test traps are guards used around forked tests */ */

/* g_test_trap_assert_passed ( ) g_test_trap_assertions ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , 0 , 0 ) # */

/* g_test_trap_assert_failed ( ) g_test_trap_assertions ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , 1 , 0 ) # */

/* g_test_trap_assert_stdout ( soutpattern ) g_test_trap_assertions ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , 2 , soutpattern ) # */

/* g_test_trap_assert_stdout_unmatched ( soutpattern ) g_test_trap_assertions ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , 3 , soutpattern ) # */

/* g_test_trap_assert_stderr ( serrpattern ) g_test_trap_assertions ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , 4 , serrpattern ) # */

/* g_test_trap_assert_stderr_unmatched ( serrpattern ) g_test_trap_assertions ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , 5 , serrpattern ) /* provide seed-able random numbers for tests */ */

/* g_test_rand_bit ( ) ( 0 != ( g_test_rand_int ( ) & ( 1 << 15 ) ) ) gint32 */

/* __G_THREADPOOL_H__ # */

/* __G_TIMER_H__ # */

/* G_USEC_PER_SEC 1000000 GTimer */
pub const G_USEC_PER_SEC: i32 = 1000000;

/* __G_TRASH_STACK_H__ # */

/* __G_TREE_H__ # */

/* __G_URI_FUNCS_H__ # */

/* G_URI_RESERVED_CHARS_GENERIC_DELIMITERS ":/?#[]@" /**
 * G_URI_RESERVED_CHARS_SUBCOMPONENT_DELIMITERS:
 * 
 * Subcomponent delimiter characters as defined in RFC 3986. Includes "!$&'()*+,;=".
 **/ */

/* G_URI_RESERVED_CHARS_SUBCOMPONENT_DELIMITERS "!$&'()*+,;=" /**
 * G_URI_RESERVED_CHARS_ALLOWED_IN_PATH_ELEMENT:
 * 
 * Allowed characters in path elements. Includes "!$&'()*+,;=:@".
 **/ */

/* G_URI_RESERVED_CHARS_ALLOWED_IN_PATH_ELEMENT G_URI_RESERVED_CHARS_SUBCOMPONENT_DELIMITERS ":@" /**
 * G_URI_RESERVED_CHARS_ALLOWED_IN_PATH:
 * 
 * Allowed characters in a path. Includes "!$&'()*+,;=:@/".
 **/ */

/* G_URI_RESERVED_CHARS_ALLOWED_IN_PATH G_URI_RESERVED_CHARS_ALLOWED_IN_PATH_ELEMENT "/" /**
 * G_URI_RESERVED_CHARS_ALLOWED_IN_USERINFO:
 * 
 * Allowed characters in userinfo as defined in RFC 3986. Includes "!$&'()*+,;=:".
 **/ */

/* G_URI_RESERVED_CHARS_ALLOWED_IN_USERINFO G_URI_RESERVED_CHARS_SUBCOMPONENT_DELIMITERS ":" char */

/* __G_VARIANT_TYPE_H__ # */

/* G_VARIANT_TYPE_BOOLEAN ( ( const GVariantType * ) "b" ) /**
 * G_VARIANT_TYPE_BYTE:
 *
 * The type of an integer value that can range from 0 to 255.
 **/ */

/* G_VARIANT_TYPE_BYTE ( ( const GVariantType * ) "y" ) /**
 * G_VARIANT_TYPE_INT16:
 *
 * The type of an integer value that can range from -32768 to 32767.
 **/ */

/* G_VARIANT_TYPE_INT16 ( ( const GVariantType * ) "n" ) /**
 * G_VARIANT_TYPE_UINT16:
 *
 * The type of an integer value that can range from 0 to 65535.
 * There were about this many people living in Toronto in the 1870s.
 **/ */

/* G_VARIANT_TYPE_UINT16 ( ( const GVariantType * ) "q" ) /**
 * G_VARIANT_TYPE_INT32:
 *
 * The type of an integer value that can range from -2147483648 to
 * 2147483647.
 **/ */

/* G_VARIANT_TYPE_INT32 ( ( const GVariantType * ) "i" ) /**
 * G_VARIANT_TYPE_UINT32:
 *
 * The type of an integer value that can range from 0 to 4294967295.
 * That's one number for everyone who was around in the late 1970s.
 **/ */

/* G_VARIANT_TYPE_UINT32 ( ( const GVariantType * ) "u" ) /**
 * G_VARIANT_TYPE_INT64:
 *
 * The type of an integer value that can range from
 * -9223372036854775808 to 9223372036854775807.
 **/ */

/* G_VARIANT_TYPE_INT64 ( ( const GVariantType * ) "x" ) /**
 * G_VARIANT_TYPE_UINT64:
 *
 * The type of an integer value that can range from 0 to
 * 18446744073709551616.  That's a really big number, but a Rubik's
 * cube can have a bit more than twice as many possible positions.
 **/ */

/* G_VARIANT_TYPE_UINT64 ( ( const GVariantType * ) "t" ) /**
 * G_VARIANT_TYPE_DOUBLE:
 *
 * The type of a double precision IEEE754 floating point number.
 * These guys go up to about 1.80e308 (plus and minus) but miss out on
 * some numbers in between.  In any case, that's far greater than the
 * estimated number of fundamental particles in the observable
 * universe.
 **/ */

/* G_VARIANT_TYPE_DOUBLE ( ( const GVariantType * ) "d" ) /**
 * G_VARIANT_TYPE_STRING:
 *
 * The type of a string.  "" is a string.  %NULL is not a string.
 **/ */

/* G_VARIANT_TYPE_STRING ( ( const GVariantType * ) "s" ) /**
 * G_VARIANT_TYPE_OBJECT_PATH:
 *
 * The type of a D-Bus object reference.  These are strings of a
 * specific format used to identify objects at a given destination on
 * the bus.
 *
 * If you are not interacting with D-Bus, then there is no reason to make
 * use of this type.  If you are, then the D-Bus specification contains a
 * precise description of valid object paths.
 **/ */

/* G_VARIANT_TYPE_OBJECT_PATH ( ( const GVariantType * ) "o" ) /**
 * G_VARIANT_TYPE_SIGNATURE:
 *
 * The type of a D-Bus type signature.  These are strings of a specific
 * format used as type signatures for D-Bus methods and messages.
 *
 * If you are not interacting with D-Bus, then there is no reason to make
 * use of this type.  If you are, then the D-Bus specification contains a
 * precise description of valid signature strings.
 **/ */

/* G_VARIANT_TYPE_SIGNATURE ( ( const GVariantType * ) "g" ) /**
 * G_VARIANT_TYPE_VARIANT:
 *
 * The type of a box that contains any other value (including another
 * variant).
 **/ */

/* G_VARIANT_TYPE_VARIANT ( ( const GVariantType * ) "v" ) /**
 * G_VARIANT_TYPE_HANDLE:
 *
 * The type of a 32bit signed integer value, that by convention, is used
 * as an index into an array of file descriptors that are sent alongside
 * a D-Bus message.
 *
 * If you are not interacting with D-Bus, then there is no reason to make
 * use of this type.
 **/ */

/* G_VARIANT_TYPE_HANDLE ( ( const GVariantType * ) "h" ) /**
 * G_VARIANT_TYPE_UNIT:
 *
 * The empty tuple type.  Has only one instance.  Known also as "triv"
 * or "void".
 **/ */

/* G_VARIANT_TYPE_UNIT ( ( const GVariantType * ) "()" ) /**
 * G_VARIANT_TYPE_ANY:
 *
 * An indefinite type that is a supertype of every type (including
 * itself).
 **/ */

/* G_VARIANT_TYPE_ANY ( ( const GVariantType * ) "*" ) /**
 * G_VARIANT_TYPE_BASIC:
 *
 * An indefinite type that is a supertype of every basic (ie:
 * non-container) type.
 **/ */

/* G_VARIANT_TYPE_BASIC ( ( const GVariantType * ) "?" ) /**
 * G_VARIANT_TYPE_MAYBE:
 *
 * An indefinite type that is a supertype of every maybe type.
 **/ */

/* G_VARIANT_TYPE_MAYBE ( ( const GVariantType * ) "m*" ) /**
 * G_VARIANT_TYPE_ARRAY:
 *
 * An indefinite type that is a supertype of every array type.
 **/ */

/* G_VARIANT_TYPE_ARRAY ( ( const GVariantType * ) "a*" ) /**
 * G_VARIANT_TYPE_TUPLE:
 *
 * An indefinite type that is a supertype of every tuple type,
 * regardless of the number of items in the tuple.
 **/ */

/* G_VARIANT_TYPE_TUPLE ( ( const GVariantType * ) "r" ) /**
 * G_VARIANT_TYPE_DICT_ENTRY:
 *
 * An indefinite type that is a supertype of every dictionary entry
 * type.
 **/ */

/* G_VARIANT_TYPE_DICT_ENTRY ( ( const GVariantType * ) "{?*}" ) /**
 * G_VARIANT_TYPE_DICTIONARY:
 *
 * An indefinite type that is a supertype of every dictionary type --
 * that is, any array type that has an element type equal to any
 * dictionary entry type.
 **/ */

/* G_VARIANT_TYPE_DICTIONARY ( ( const GVariantType * ) "a{?*}" ) /**
 * G_VARIANT_TYPE_STRING_ARRAY:
 *
 * The type of an array of strings.
 **/ */

/* G_VARIANT_TYPE_STRING_ARRAY ( ( const GVariantType * ) "as" ) /**
 * G_VARIANT_TYPE_OBJECT_PATH_ARRAY:
 *
 * The type of an array of object paths.
 **/ */

/* G_VARIANT_TYPE_OBJECT_PATH_ARRAY ( ( const GVariantType * ) "ao" ) /**
 * G_VARIANT_TYPE_BYTESTRING:
 *
 * The type of an array of bytes.  This type is commonly used to pass
 * around strings that may not be valid utf8.  In that case, the
 * convention is that the nul terminator character should be included as
 * the last character in the array.
 **/ */

/* G_VARIANT_TYPE_BYTESTRING ( ( const GVariantType * ) "ay" ) /**
 * G_VARIANT_TYPE_BYTESTRING_ARRAY:
 *
 * The type of an array of byte strings (an array of arrays of bytes).
 **/ */

/* G_VARIANT_TYPE_BYTESTRING_ARRAY ( ( const GVariantType * ) "aay" ) /**
 * G_VARIANT_TYPE_VARDICT:
 *
 * The type of a dictionary mapping strings to variants (the ubiquitous
 * "a{sv}" type).
 *
 * Since: 2.30
 **/ */

/* G_VARIANT_TYPE_VARDICT ( ( const GVariantType * ) "a{sv}" ) /**
 * G_VARIANT_TYPE:
 * @type_string: a well-formed #GVariantType type string
 *
 * Converts a string to a const #GVariantType.  Depending on the
 * current debugging level, this function may perform a runtime check
 * to ensure that @string is a valid GVariant type string.
 *
 * It is always a programmer error to use this macro with an invalid
 * type string. If in doubt, use g_variant_type_string_is_valid() to
 * check if the string is valid.
 *
 * Since 2.24
 **/ */

/* G_VARIANT_TYPE ( type_string ) ( g_variant_type_checked_ ( ( type_string ) ) ) # */

/* __G_VARIANT_H__ # */

/* G_VARIANT_PARSE_ERROR ( g_variant_parser_get_error_quark ( ) ) GQuark */

/* __G_VERSION_H__ # */

/* GLIB_CHECK_VERSION ( major , minor , micro ) ( GLIB_MAJOR_VERSION > ( major ) || ( GLIB_MAJOR_VERSION == ( major ) && GLIB_MINOR_VERSION > ( minor ) ) || ( GLIB_MAJOR_VERSION == ( major ) && GLIB_MINOR_VERSION == ( minor ) && GLIB_MICRO_VERSION >= ( micro ) ) ) G_END_DECLS */

/* __G_ALLOCATOR_H__ # */

/* G_ALLOC_ONLY 1 # */
pub const G_ALLOC_ONLY: i32 = 1;

/* G_ALLOC_AND_FREE 2 # */
pub const G_ALLOC_AND_FREE: i32 = 2;

/* G_ALLOCATOR_LIST 1 # */
pub const G_ALLOCATOR_LIST: i32 = 1;

/* G_ALLOCATOR_SLIST 2 # */
pub const G_ALLOCATOR_SLIST: i32 = 2;

/* G_ALLOCATOR_NODE 3 # */
pub const G_ALLOCATOR_NODE: i32 = 3;

/* g_chunk_new ( type , chunk ) ( ( type * ) g_mem_chunk_alloc ( chunk ) ) # */

/* g_chunk_new0 ( type , chunk ) ( ( type * ) g_mem_chunk_alloc0 ( chunk ) ) # */

/* g_chunk_free ( mem , mem_chunk ) ( g_mem_chunk_free ( mem_chunk , mem ) ) # */

/* g_mem_chunk_create ( type , x , y ) ( g_mem_chunk_new ( NULL , sizeof ( type ) , 0 , 0 ) ) GLIB_DEPRECATED */

/* __G_CACHE_H__ # */

/* __G_COMPLETION_H__ # */

/* __G_DEPRECATED_MAIN_H__ # */

/* g_main_new ( is_running ) g_main_loop_new ( NULL , is_running ) /**
 * g_main_run:
 * @loop: a #GMainLoop
 *
 * Runs a main loop until it stops running.
 *
 * Deprecated: 2.2: Use g_main_loop_run() instead
 */ */

/* g_main_run ( loop ) g_main_loop_run ( loop ) /**
 * g_main_quit:
 * @loop: a #GMainLoop
 *
 * Stops the #GMainLoop.
 * If g_main_run() was called to run the #GMainLoop, it will now return.
 *
 * Deprecated: 2.2: Use g_main_loop_quit() instead
 */ */

/* g_main_quit ( loop ) g_main_loop_quit ( loop ) /**
 * g_main_destroy:
 * @loop: a #GMainLoop
 *
 * Frees the memory allocated for the #GMainLoop.
 *
 * Deprecated: 2.2: Use g_main_loop_unref() instead
 */ */

/* g_main_destroy ( loop ) g_main_loop_unref ( loop ) /**
 * g_main_is_running:
 * @loop: a #GMainLoop
 *
 * Checks if the main loop is running.
 *
 * Returns: %TRUE if the main loop is running
 *
 * Deprecated: 2.2: Use g_main_loop_is_running() instead
 */ */

/* g_main_is_running ( loop ) g_main_loop_is_running ( loop ) /**
 * g_main_iteration:
 * @may_block: set to %TRUE if it should block (i.e. wait) until an event
 *     source becomes ready. It will return after an event source has been
 *     processed. If set to %FALSE it will return immediately if no event
 *     source is ready to be processed.
 *
 * Runs a single iteration for the default #GMainContext.
 *
 * Returns: %TRUE if more events are pending.
 *
 * Deprecated: 2.2: Use g_main_context_iteration() instead.
 */ */

/* g_main_iteration ( may_block ) g_main_context_iteration ( NULL , may_block ) /**
 * g_main_pending:
 *
 * Checks if any events are pending for the default #GMainContext
 * (i.e. ready to be processed).
 *
 * Returns: %TRUE if any events are pending.
 *
 * Deprected: 2.2: Use g_main_context_pending() instead.
 */ */

/* g_main_pending ( ) g_main_context_pending ( NULL ) /**
 * g_main_set_poll_func:
 * @func: the function to call to poll all file descriptors
 *
 * Sets the function to use for the handle polling of file descriptors
 * for the default main context.
 *
 * Deprecated: 2.2: Use g_main_context_set_poll_func() again
 */ */

/* g_main_set_poll_func ( func ) g_main_context_set_poll_func ( NULL , func ) G_END_DECLS */

/* __G_REL_H__ # */

/* __G_DEPRECATED_THREAD_H__ # */

/* g_static_mutex_get_mutex g_static_mutex_get_mutex_impl # */

/* G_STATIC_MUTEX_INIT { NULL } typedef */

/* g_static_mutex_lock ( mutex ) g_mutex_lock ( g_static_mutex_get_mutex ( mutex ) ) # */

/* g_static_mutex_trylock ( mutex ) g_mutex_trylock ( g_static_mutex_get_mutex ( mutex ) ) # */

/* g_static_mutex_unlock ( mutex ) g_mutex_unlock ( g_static_mutex_get_mutex ( mutex ) ) GLIB_DEPRECATED_IN_2_32_FOR */

/* G_STATIC_REC_MUTEX_INIT { G_STATIC_MUTEX_INIT } GLIB_DEPRECATED_IN_2_32_FOR */

/* G_STATIC_RW_LOCK_INIT { G_STATIC_MUTEX_INIT , NULL , NULL , 0 , FALSE , 0 , 0 } GLIB_DEPRECATED_IN_2_32_FOR */

/* G_STATIC_PRIVATE_INIT { 0 } GLIB_DEPRECATED_IN_2_32 */

/* g_thread_supported ( ) ( 1 ) GLIB_DEPRECATED_IN_2_32 */

