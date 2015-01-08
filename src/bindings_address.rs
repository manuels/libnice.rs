extern crate libc;

/*
void nice_address_init()
	(NiceAddress *) addr [struct _NiceAddress *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_init(addr: *mut _NiceAddress);
}


/*
NiceAddress * nice_address_new() [struct _NiceAddress *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_new() -> *mut _NiceAddress;
}


/*
void nice_address_free()
	(NiceAddress *) addr [struct _NiceAddress *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_free(addr: *mut _NiceAddress);
}


/*
NiceAddress * nice_address_dup() [struct _NiceAddress *]
	(const NiceAddress *) addr [const struct _NiceAddress *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_dup(addr: *const _NiceAddress) -> *mut _NiceAddress;
}


/*
void nice_address_set_ipv4()
	(NiceAddress *) addr [struct _NiceAddress *]
	(int) addr_ipv4
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_set_ipv4(addr: *mut _NiceAddress, addr_ipv4: libc::c_int);
}


/*
void nice_address_set_ipv6()
	(NiceAddress *) addr [struct _NiceAddress *]
	(const int *) addr_ipv6
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_set_ipv6(addr: *mut _NiceAddress, addr_ipv6: *const libc::c_int);
}


/*
void nice_address_set_port()
	(NiceAddress *) addr [struct _NiceAddress *]
	(int) port
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_set_port(addr: *mut _NiceAddress, port: libc::c_int);
}


/*
int nice_address_get_port()
	(const NiceAddress *) addr [const struct _NiceAddress *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_get_port(addr: *const _NiceAddress) -> libc::c_int;
}


/*
int nice_address_set_from_string()
	(NiceAddress *) addr [struct _NiceAddress *]
	(const int *) str
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_set_from_string(addr: *mut _NiceAddress, str: *const libc::c_int) -> libc::c_int;
}


/*
void nice_address_set_from_sockaddr()
	(NiceAddress *) addr [struct _NiceAddress *]
	(const struct sockaddr *) sin [const struct sockaddr *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_set_from_sockaddr(addr: *mut _NiceAddress, sin: *const sockaddr);
}


/*
void nice_address_copy_to_sockaddr()
	(const NiceAddress *) addr [const struct _NiceAddress *]
	(struct sockaddr *) sin [struct sockaddr *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_copy_to_sockaddr(addr: *const _NiceAddress, sin: *mut sockaddr);
}


/*
int nice_address_equal()
	(const NiceAddress *) a [const struct _NiceAddress *]
	(const NiceAddress *) b [const struct _NiceAddress *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_equal(a: *const _NiceAddress, b: *const _NiceAddress) -> libc::c_int;
}


/*
void nice_address_to_string()
	(const NiceAddress *) addr [const struct _NiceAddress *]
	(int *) dst
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_to_string(addr: *const _NiceAddress, dst: *mut libc::c_int);
}


/*
int nice_address_is_private()
	(const NiceAddress *) addr [const struct _NiceAddress *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_is_private(addr: *const _NiceAddress) -> libc::c_int;
}


/*
struct _NiceAddress
*/
#[repr(C)]
pub struct _NiceAddress;

/*
struct NiceAddress
*/
#[repr(C)]
pub struct NiceAddress;

/*
struct sockaddr
		(sa_family_t) sa_family [unsigned short]
		(char [14]) sa_data
*/
#[repr(C)]
pub struct sockaddr;
/*{
	sa_family: libc::c_ushort,
	sa_data: [libc::c_char, ..14],
}*/

/* _ADDRESS_H /**
 * SECTION:address
 * @short_description: IP address convenience library
 * @stability: Stable
 *
 * The #NiceAddress structure will allow you to easily set/get and modify an IPv4
 * or IPv6 address in order to communicate with the #NiceAgent.
 */ */

/* NICE_ADDRESS_STRING_LEN INET6_ADDRSTRLEN typedef */

