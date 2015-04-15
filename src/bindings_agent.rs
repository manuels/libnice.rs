extern crate libc;
use std::mem;

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
	(guint32) addr_ipv4 [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_set_ipv4(addr: *mut _NiceAddress, addr_ipv4: libc::c_uint);
}


/*
void nice_address_set_ipv6()
	(NiceAddress *) addr [struct _NiceAddress *]
	(const guchar *) addr_ipv6 [const unsigned char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_set_ipv6(addr: *mut _NiceAddress, addr_ipv6: *const libc::c_uchar);
}


/*
void nice_address_set_port()
	(NiceAddress *) addr [struct _NiceAddress *]
	(guint) port [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_set_port(addr: *mut _NiceAddress, port: libc::c_uint);
}


/*
guint nice_address_get_port() [unsigned int]
	(const NiceAddress *) addr [const struct _NiceAddress *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_get_port(addr: *const _NiceAddress) -> libc::c_uint;
}


/*
gboolean nice_address_set_from_string() [int]
	(NiceAddress *) addr [struct _NiceAddress *]
	(const gchar *) str [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_set_from_string(addr: *mut _NiceAddress, str: *const libc::c_char) -> libc::c_int;
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
gboolean nice_address_equal() [int]
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
	(gchar *) dst [char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_to_string(addr: *const _NiceAddress, dst: *mut libc::c_char);
}


/*
gboolean nice_address_is_private() [int]
	(const NiceAddress *) addr [const struct _NiceAddress *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_is_private(addr: *const _NiceAddress) -> libc::c_int;
}


/*
gboolean nice_address_is_valid() [int]
	(const NiceAddress *) addr [const struct _NiceAddress *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_is_valid(addr: *const _NiceAddress) -> libc::c_int;
}


/*
int nice_address_ip_version()
	(const NiceAddress *) addr [const struct _NiceAddress *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_address_ip_version(addr: *const _NiceAddress) -> libc::c_int;
}


/*
NiceCandidate * nice_candidate_new() [struct _NiceCandidate *]
	(NiceCandidateType) type [NiceCandidateType]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_candidate_new(type_: libc::c_uint) -> *mut _NiceCandidate;
}


/*
void nice_candidate_free()
	(NiceCandidate *) candidate [struct _NiceCandidate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_candidate_free(candidate: *mut _NiceCandidate);
}


/*
NiceCandidate * nice_candidate_copy() [struct _NiceCandidate *]
	(const NiceCandidate *) candidate [const struct _NiceCandidate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_candidate_copy(candidate: *const _NiceCandidate) -> *mut _NiceCandidate;
}


/*
guint32 nice_candidate_jingle_priority() [unsigned int]
	(NiceCandidate *) candidate [struct _NiceCandidate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_candidate_jingle_priority(candidate: *mut _NiceCandidate) -> libc::c_uint;
}


/*
guint32 nice_candidate_msn_priority() [unsigned int]
	(NiceCandidate *) candidate [struct _NiceCandidate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_candidate_msn_priority(candidate: *mut _NiceCandidate) -> libc::c_uint;
}


/*
guint32 nice_candidate_ice_priority_full() [unsigned int]
	(guint) type_pref [unsigned int]
	(guint) local_pref [unsigned int]
	(guint) component_id [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_candidate_ice_priority_full(type_pref: libc::c_uint, local_pref: libc::c_uint, component_id: libc::c_uint) -> libc::c_uint;
}


/*
guint32 nice_candidate_ice_priority() [unsigned int]
	(const NiceCandidate *) candidate [const struct _NiceCandidate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_candidate_ice_priority(candidate: *const _NiceCandidate) -> libc::c_uint;
}


/*
guint64 nice_candidate_pair_priority() [unsigned long]
	(guint32) o_prio [unsigned int]
	(guint32) a_prio [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_candidate_pair_priority(o_prio: libc::c_uint, a_prio: libc::c_uint) -> libc::c_ulong;
}


/*
void nice_debug_init()
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_debug_init();
}


/*
void nice_debug_enable()
	(gboolean) with_stun [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_debug_enable(with_stun: libc::c_int);
}


/*
void nice_debug_disable()
	(gboolean) with_stun [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_debug_disable(with_stun: libc::c_int);
}


/*
void nice_debug()
	(const char *) fmt
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_debug(fmt: *const libc::c_char);
}


/*
GType nice_agent_get_type() [unsigned long]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_get_type() -> libc::c_ulong;
}


/*
NiceAgent * nice_agent_new() [struct _NiceAgent *]
	(GMainContext *) ctx [struct GMainContext *]
	(NiceCompatibility) compat [NiceCompatibility]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_new(ctx: *mut GMainContext, compat: libc::c_uint) -> *mut _NiceAgent;
}


/*
NiceAgent * nice_agent_new_reliable() [struct _NiceAgent *]
	(GMainContext *) ctx [struct GMainContext *]
	(NiceCompatibility) compat [NiceCompatibility]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_new_reliable(ctx: *mut GMainContext, compat: libc::c_uint) -> *mut _NiceAgent;
}


/*
gboolean nice_agent_add_local_address() [int]
	(NiceAgent *) agent [struct _NiceAgent *]
	(NiceAddress *) addr [struct _NiceAddress *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_add_local_address(agent: *mut _NiceAgent, addr: *mut _NiceAddress) -> libc::c_int;
}


/*
guint nice_agent_add_stream() [unsigned int]
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) n_components [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_add_stream(agent: *mut _NiceAgent, n_components: libc::c_uint) -> libc::c_uint;
}


/*
void nice_agent_remove_stream()
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_remove_stream(agent: *mut _NiceAgent, stream_id: libc::c_uint);
}


/*
void nice_agent_set_port_range()
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
	(guint) component_id [unsigned int]
	(guint) min_port [unsigned int]
	(guint) max_port [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_set_port_range(agent: *mut _NiceAgent, stream_id: libc::c_uint, component_id: libc::c_uint, min_port: libc::c_uint, max_port: libc::c_uint);
}


/*
gboolean nice_agent_set_relay_info() [int]
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
	(guint) component_id [unsigned int]
	(const gchar *) server_ip [const char *]
	(guint) server_port [unsigned int]
	(const gchar *) username [const char *]
	(const gchar *) password [const char *]
	(NiceRelayType) type [NiceRelayType]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_set_relay_info(agent: *mut _NiceAgent, stream_id: libc::c_uint, component_id: libc::c_uint, server_ip: *const libc::c_char, server_port: libc::c_uint, username: *const libc::c_char, password: *const libc::c_char, type_: libc::c_uint) -> libc::c_int;
}


/*
gboolean nice_agent_gather_candidates() [int]
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_gather_candidates(agent: *mut _NiceAgent, stream_id: libc::c_uint) -> libc::c_int;
}


/*
gboolean nice_agent_set_remote_credentials() [int]
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
	(const gchar *) ufrag [const char *]
	(const gchar *) pwd [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_set_remote_credentials(agent: *mut _NiceAgent, stream_id: libc::c_uint, ufrag: *const libc::c_char, pwd: *const libc::c_char) -> libc::c_int;
}


/*
gboolean nice_agent_get_local_credentials() [int]
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
	(gchar **) ufrag [char **]
	(gchar **) pwd [char **]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_get_local_credentials(agent: *mut _NiceAgent, stream_id: libc::c_uint, ufrag: *mut *mut libc::c_char, pwd: *mut *mut libc::c_char) -> libc::c_int;
}


/*
int nice_agent_set_remote_candidates()
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
	(guint) component_id [unsigned int]
	(const GSList *) candidates [const struct GSList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_set_remote_candidates(agent: *mut _NiceAgent, stream_id: libc::c_uint, component_id: libc::c_uint, candidates: *const GSList) -> libc::c_int;
}


/*
gint nice_agent_send() [int]
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
	(guint) component_id [unsigned int]
	(guint) len [unsigned int]
	(const gchar *) buf [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_send(agent: *mut _NiceAgent, stream_id: libc::c_uint, component_id: libc::c_uint, len: libc::c_uint, buf: *const libc::c_char) -> libc::c_int;
}


/*
GSList * nice_agent_get_local_candidates() [struct GSList *]
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
	(guint) component_id [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_get_local_candidates(agent: *mut _NiceAgent, stream_id: libc::c_uint, component_id: libc::c_uint) -> *mut GSList;
}


/*
GSList * nice_agent_get_remote_candidates() [struct GSList *]
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
	(guint) component_id [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_get_remote_candidates(agent: *mut _NiceAgent, stream_id: libc::c_uint, component_id: libc::c_uint) -> *mut GSList;
}


/*
gboolean nice_agent_restart() [int]
	(NiceAgent *) agent [struct _NiceAgent *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_restart(agent: *mut _NiceAgent) -> libc::c_int;
}


/*
gboolean nice_agent_attach_recv() [int]
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
	(guint) component_id [unsigned int]
	(GMainContext *) ctx [struct GMainContext *]
	(NiceAgentRecvFunc) func [void (*)(struct _NiceAgent *, unsigned int, unsigned int, unsigned int, char *, void *)]
	(gpointer) data [void *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_attach_recv(agent: *mut _NiceAgent, stream_id: libc::c_uint, component_id: libc::c_uint, ctx: *mut GMainContext, func: Option<extern fn(*mut _NiceAgent, libc::c_uint, libc::c_uint, libc::c_uint, *mut libc::c_char, *mut libc::c_void)>, data: *mut libc::c_void) -> libc::c_int;
}


/*
gboolean nice_agent_set_selected_pair() [int]
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
	(guint) component_id [unsigned int]
	(const gchar *) lfoundation [const char *]
	(const gchar *) rfoundation [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_set_selected_pair(agent: *mut _NiceAgent, stream_id: libc::c_uint, component_id: libc::c_uint, lfoundation: *const libc::c_char, rfoundation: *const libc::c_char) -> libc::c_int;
}


/*
gboolean nice_agent_get_selected_pair() [int]
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
	(guint) component_id [unsigned int]
	(NiceCandidate **) local [struct _NiceCandidate **]
	(NiceCandidate **) remote [struct _NiceCandidate **]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_get_selected_pair(agent: *mut _NiceAgent, stream_id: libc::c_uint, component_id: libc::c_uint, local: *mut *mut _NiceCandidate, remote: *mut *mut _NiceCandidate) -> libc::c_int;
}


/*
gboolean nice_agent_set_selected_remote_candidate() [int]
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
	(guint) component_id [unsigned int]
	(NiceCandidate *) candidate [struct _NiceCandidate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_set_selected_remote_candidate(agent: *mut _NiceAgent, stream_id: libc::c_uint, component_id: libc::c_uint, candidate: *mut _NiceCandidate) -> libc::c_int;
}


/*
void nice_agent_set_stream_tos()
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
	(gint) tos [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_set_stream_tos(agent: *mut _NiceAgent, stream_id: libc::c_uint, tos: libc::c_int);
}


/*
void nice_agent_set_software()
	(NiceAgent *) agent [struct _NiceAgent *]
	(const gchar *) software [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_set_software(agent: *mut _NiceAgent, software: *const libc::c_char);
}


/*
gboolean nice_agent_set_stream_name() [int]
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
	(const gchar *) name [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_set_stream_name(agent: *mut _NiceAgent, stream_id: libc::c_uint, name: *const libc::c_char) -> libc::c_int;
}


/*
const gchar * nice_agent_get_stream_name() [const char *]
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_get_stream_name(agent: *mut _NiceAgent, stream_id: libc::c_uint) -> *const libc::c_char;
}


/*
NiceCandidate * nice_agent_get_default_local_candidate() [struct _NiceCandidate *]
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
	(guint) component_id [unsigned int]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_get_default_local_candidate(agent: *mut _NiceAgent, stream_id: libc::c_uint, component_id: libc::c_uint) -> *mut _NiceCandidate;
}


/*
gchar * nice_agent_generate_local_sdp() [char *]
	(NiceAgent *) agent [struct _NiceAgent *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_generate_local_sdp(agent: *mut _NiceAgent) -> *mut libc::c_char;
}


/*
gchar * nice_agent_generate_local_stream_sdp() [char *]
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
	(gboolean) include_non_ice [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_generate_local_stream_sdp(agent: *mut _NiceAgent, stream_id: libc::c_uint, include_non_ice: libc::c_int) -> *mut libc::c_char;
}


/*
gchar * nice_agent_generate_local_candidate_sdp() [char *]
	(NiceAgent *) agent [struct _NiceAgent *]
	(NiceCandidate *) candidate [struct _NiceCandidate *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_generate_local_candidate_sdp(agent: *mut _NiceAgent, candidate: *mut _NiceCandidate) -> *mut libc::c_char;
}


/*
int nice_agent_parse_remote_sdp()
	(NiceAgent *) agent [struct _NiceAgent *]
	(const gchar *) sdp [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_parse_remote_sdp(agent: *mut _NiceAgent, sdp: *const libc::c_char) -> libc::c_int;
}


/*
GSList * nice_agent_parse_remote_stream_sdp() [struct GSList *]
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
	(const gchar *) sdp [const char *]
	(gchar **) ufrag [char **]
	(gchar **) pwd [char **]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_parse_remote_stream_sdp(agent: *mut _NiceAgent, stream_id: libc::c_uint, sdp: *const libc::c_char, ufrag: *mut *mut libc::c_char, pwd: *mut *mut libc::c_char) -> *mut GSList;
}


/*
NiceCandidate * nice_agent_parse_remote_candidate_sdp() [struct _NiceCandidate *]
	(NiceAgent *) agent [struct _NiceAgent *]
	(guint) stream_id [unsigned int]
	(const gchar *) sdp [const char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_agent_parse_remote_candidate_sdp(agent: *mut _NiceAgent, stream_id: libc::c_uint, sdp: *const libc::c_char) -> *mut _NiceCandidate;
}


/*
gchar * nice_interfaces_get_ip_for_interface() [char *]
	(gchar *) interface_name [char *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_interfaces_get_ip_for_interface(interface_name: *mut libc::c_char) -> *mut libc::c_char;
}


/*
GList * nice_interfaces_get_local_ips() [struct _GList *]
	(gboolean) include_loopback [int]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_interfaces_get_local_ips(include_loopback: libc::c_int) -> *mut _GList;
}


/*
GList * nice_interfaces_get_local_interfaces() [struct _GList *]
*/
#[link(name="nice")]
extern "C" {
	pub fn nice_interfaces_get_local_interfaces() -> *mut _GList;
}


/*
struct _NiceAgent
*/
#[repr(C)]
pub struct _NiceAgent;

/*
struct _NiceAddress
		(union _NiceAddress::(anonymous at /usr/include/nice/address.h:78:3)) 
		(union (anonymous union at /usr/include/nice/address.h:78:3)) s [union _NiceAddress::(anonymous at /usr/include/nice/address.h:78:3)]
*/
#[repr(C)]
pub struct _NiceAddress;/* {
	: union _NiceAddress::(anonymous at /usr/include/nice/address.h:78:3),
	s: union _NiceAddress::(anonymous at /usr/include/nice/address.h:78:3),
}*/

/*
struct _NiceCandidate
		(NiceCandidateType) type [NiceCandidateType]
		(NiceCandidateTransport) transport [NiceCandidateTransport]
		(NiceAddress) addr [struct _NiceAddress]
		(NiceAddress) base_addr [struct _NiceAddress]
		(guint32) priority [unsigned int]
		(guint) stream_id [unsigned int]
		(guint) component_id [unsigned int]
		(gchar [33]) foundation [char [33]]
		(gchar *) username [char *]
		(gchar *) password [char *]
		(TurnServer *) turn [struct _TurnServer *]
		(gpointer) sockptr [void *]
*/
#[repr(C)]
pub struct _NiceCandidate {
	type_: libc::c_uint,
	transport: libc::c_uint,
	addr: _NiceAddress,
	base_addr: _NiceAddress,
	priority: libc::c_uint,
	stream_id: libc::c_uint,
	component_id: libc::c_uint,
	foundation: [libc::c_char; 33],
	username: *mut libc::c_char,
	password: *mut libc::c_char,
	turn: *mut _TurnServer,
	sockptr: *mut libc::c_void,
}

/*
struct _TurnServer
		(NiceAddress) server [struct _NiceAddress]
		(gchar *) username [char *]
		(gchar *) password [char *]
		(NiceRelayType) type [NiceRelayType]
*/
#[repr(C)]
pub struct _TurnServer {
	server: _NiceAddress,
	username: *mut libc::c_char,
	password: *mut libc::c_char,
	type_: libc::c_uint,
}

/*
struct _NiceAgentClass
		(GObjectClass) parent_class [struct _GObjectClass]
*/
#[repr(C)]
pub struct _NiceAgentClass;/* {
	parent_class: _GObjectClass,
}*/

/*
struct NiceAddress
		(union _NiceAddress::(anonymous at /usr/include/nice/address.h:78:3)) 
		(union (anonymous union at /usr/include/nice/address.h:78:3)) s [union _NiceAddress::(anonymous at /usr/include/nice/address.h:78:3)]
*/
#[repr(C)]
pub struct NiceAddress;/* {
	: union _NiceAddress::(anonymous at /usr/include/nice/address.h:78:3),
	s: union _NiceAddress::(anonymous at /usr/include/nice/address.h:78:3),
}*/

/*
struct sockaddr
		(sa_family_t) sa_family [unsigned short]
		(char [14]) sa_data
*/
#[repr(C)]
pub struct sockaddr {
	sa_family: libc::c_ushort,
	sa_data: [libc::c_char; 14],
}

/*
struct NiceCandidate
		(NiceCandidateType) type [NiceCandidateType]
		(NiceCandidateTransport) transport [NiceCandidateTransport]
		(NiceAddress) addr [struct _NiceAddress]
		(NiceAddress) base_addr [struct _NiceAddress]
		(guint32) priority [unsigned int]
		(guint) stream_id [unsigned int]
		(guint) component_id [unsigned int]
		(gchar [33]) foundation [char [33]]
		(gchar *) username [char *]
		(gchar *) password [char *]
		(TurnServer *) turn [struct _TurnServer *]
		(gpointer) sockptr [void *]
*/
#[repr(C)]
pub struct NiceCandidate {
	type_: libc::c_uint,
	transport: libc::c_uint,
	addr: _NiceAddress,
	base_addr: _NiceAddress,
	priority: libc::c_uint,
	stream_id: libc::c_uint,
	component_id: libc::c_uint,
	foundation: [libc::c_char; 33],
	username: *mut libc::c_char,
	password: *mut libc::c_char,
	turn: *mut _TurnServer,
	sockptr: *mut libc::c_void,
}

/*
struct GMainContext
*/
#[repr(C)]
pub struct GMainContext;

unsafe impl Send for GMainContext {}
unsafe impl Sync for GMainContext {}

/*
struct NiceAgent
*/
#[repr(C)]
pub struct NiceAgent;

/*
struct GSList
		(gpointer) data [void *]
		(GSList *) next [struct GSList *]
*/
#[repr(C)]
pub struct GSList {
	data: *mut libc::c_void,
	next: *mut GSList,
}

#[repr(C)]
pub struct _GList;

/*
enum  {
	NICE_CANDIDATE_TYPE_HOST =	0x00000000 (0)
	NICE_CANDIDATE_TYPE_SERVER_REFLEXIVE =	0x00000001 (1)
	NICE_CANDIDATE_TYPE_PEER_REFLEXIVE =	0x00000002 (2)
	NICE_CANDIDATE_TYPE_RELAYED =	0x00000003 (3)
}
*/
#[derive(Clone, PartialEq, Debug)]
#[repr(u32)]
pub enum NiceCandidateType {
	NICE_CANDIDATE_TYPE_HOST =	0 as u32,
	NICE_CANDIDATE_TYPE_SERVER_REFLEXIVE =	1 as u32,
	NICE_CANDIDATE_TYPE_PEER_REFLEXIVE =	2 as u32,
	NICE_CANDIDATE_TYPE_RELAYED =	3 as u32,
}

impl NiceCandidateType {
	pub fn to_u32(&self) -> libc::c_uint {
		self.clone() as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> NiceCandidateType {
		unsafe { mem::transmute(v) }
	}
}

/*
enum  {
	NICE_CANDIDATE_TRANSPORT_UDP =	0x00000000 (0)
}
*/
#[derive(Clone, PartialEq, Debug)]
#[repr(u32)]
pub enum NiceCandidateTransport {
	NICE_CANDIDATE_TRANSPORT_UDP =	0 as u32,
	FOO = 1,
}

impl NiceCandidateTransport {
	pub fn to_u32(&self) -> libc::c_uint {
		self.clone() as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> NiceCandidateTransport {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	NICE_RELAY_TYPE_TURN_UDP =	0x00000000 (0)
	NICE_RELAY_TYPE_TURN_TCP =	0x00000001 (1)
	NICE_RELAY_TYPE_TURN_TLS =	0x00000002 (2)
}
*/
#[derive(Clone, PartialEq, Debug)]
#[repr(u32)]
pub enum NiceRelayType {
	NICE_RELAY_TYPE_TURN_UDP =	0 as u32,
	NICE_RELAY_TYPE_TURN_TCP =	1 as u32,
	NICE_RELAY_TYPE_TURN_TLS =	2 as u32,
}

impl NiceRelayType {
	pub fn to_u32(&self) -> libc::c_uint {
		self.clone() as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> NiceRelayType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	NICE_COMPONENT_STATE_DISCONNECTED =	0x00000000 (0)
	NICE_COMPONENT_STATE_GATHERING =	0x00000001 (1)
	NICE_COMPONENT_STATE_CONNECTING =	0x00000002 (2)
	NICE_COMPONENT_STATE_CONNECTED =	0x00000003 (3)
	NICE_COMPONENT_STATE_READY =	0x00000004 (4)
	NICE_COMPONENT_STATE_FAILED =	0x00000005 (5)
	NICE_COMPONENT_STATE_LAST =	0x00000006 (6)
}
*/
#[repr(u32)]
#[derive(Clone, PartialEq, Debug)]
pub enum NiceComponentState {
	NICE_COMPONENT_STATE_DISCONNECTED =	0 as u32,
	NICE_COMPONENT_STATE_GATHERING =	1 as u32,
	NICE_COMPONENT_STATE_CONNECTING =	2 as u32,
	NICE_COMPONENT_STATE_CONNECTED =	3 as u32,
	NICE_COMPONENT_STATE_READY =	4 as u32,
	NICE_COMPONENT_STATE_FAILED =	5 as u32,
}

impl NiceComponentState {
	pub fn to_u32(&self) -> libc::c_uint {
		self.clone() as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> NiceComponentState {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	NICE_COMPONENT_TYPE_RTP =	0x00000001 (1)
	NICE_COMPONENT_TYPE_RTCP =	0x00000002 (2)
}
*/
#[repr(u32)]
#[derive(Clone, PartialEq, Debug)]
pub enum NiceComponentType {
	NICE_COMPONENT_TYPE_RTP =	1 as u32,
	NICE_COMPONENT_TYPE_RTCP =	2 as u32,
}

impl NiceComponentType {
	pub fn to_u32(&self) -> libc::c_uint {
		self.clone() as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> NiceComponentType {
		unsafe { mem::transmute(v) }
	}
}


/*
enum  {
	NICE_COMPATIBILITY_RFC5245 =	0x00000000 (0)
	NICE_COMPATIBILITY_GOOGLE =	0x00000001 (1)
	NICE_COMPATIBILITY_MSN =	0x00000002 (2)
	NICE_COMPATIBILITY_WLM2009 =	0x00000003 (3)
	NICE_COMPATIBILITY_OC2007 =	0x00000004 (4)
	NICE_COMPATIBILITY_OC2007R2 =	0x00000005 (5)
	NICE_COMPATIBILITY_DRAFT19 =	0x00000000 (0)
	NICE_COMPATIBILITY_LAST =	0x00000005 (5)
}
*/
#[derive(Clone, PartialEq, Debug)]
#[repr(u32)]
pub enum NiceCompatibility {
	NICE_COMPATIBILITY_RFC5245 =	0 as u32,
	NICE_COMPATIBILITY_GOOGLE =	1 as u32,
	NICE_COMPATIBILITY_MSN =	2 as u32,
	NICE_COMPATIBILITY_WLM2009 =	3 as u32,
	NICE_COMPATIBILITY_OC2007 =	4 as u32,
	NICE_COMPATIBILITY_OC2007R2 =	5 as u32,
}

impl NiceCompatibility {
	pub fn to_u32(&self) -> libc::c_uint {
		self.clone() as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> NiceCompatibility {
		unsafe { mem::transmute(v) }
	}
}

/*
enum  {
	NICE_PROXY_TYPE_NONE =	0x00000000 (0)
	NICE_PROXY_TYPE_SOCKS5 =	0x00000001 (1)
	NICE_PROXY_TYPE_HTTP =	0x00000002 (2)
	NICE_PROXY_TYPE_LAST =	0x00000002 (2)
}
*/
#[derive(Clone, PartialEq, Debug)]
#[repr(u32)]
pub enum NiceProxyType {
	NICE_PROXY_TYPE_NONE =	0 as u32,
	NICE_PROXY_TYPE_SOCKS5 =	1 as u32,
	NICE_PROXY_TYPE_HTTP =	2 as u32,
}

impl NiceProxyType {
	pub fn to_u32(&self) -> libc::c_uint {
		self.clone() as libc::c_uint
	}

	pub fn from_u32(v: libc::c_uint) -> NiceProxyType {
		unsafe { mem::transmute(v) }
	}
}


/* _NICE_H # */

/* _AGENT_H /**
 * SECTION:agent
 * @short_description:  ICE agent API implementation
 * @see_also: #NiceCandidate
 * @see_also: #NiceAddress
 * @include: agent.h
 * @stability: Stable
 *
 * The #NiceAgent is your main object when using libnice.
 * It is the agent that will take care of everything relating to ICE.
 * It will take care of discovering your local candidates and do
 *  connectivity checks to create a stream of data between you and your peer.
 *
 <example>
   <title>Simple example on how to use libnice</title>
   <programlisting>
   guint stream_id;
   gchar buffer[] = "hello world!";
   GSList *lcands = NULL;

   // Create a nice agent
   NiceAgent *agent = nice_agent_new (NULL, NICE_COMPATIBILITY_RFC5245);

   // Connect the signals
   g_signal_connect (G_OBJECT (agent), "candidate-gathering-done",
                     G_CALLBACK (cb_candidate_gathering_done), NULL);
   g_signal_connect (G_OBJECT (agent), "component-state-changed",
                     G_CALLBACK (cb_component_state_changed), NULL);
   g_signal_connect (G_OBJECT (agent), "new-selected-pair",
                     G_CALLBACK (cb_new_selected_pair), NULL);

   // Create a new stream with one component and start gathering candidates
   stream_id = nice_agent_add_stream (agent, 1);
   nice_agent_gather_candidates (agent, stream_id);

   // Attach to the component to receive the data
   nice_agent_attach_recv (agent, stream_id, 1, NULL,
                          cb_nice_recv, NULL);

   // ... Wait until the signal candidate-gathering-done is fired ...
   lcands = nice_agent_get_local_candidates(agent, stream_id, 1);

   // ... Send local candidates to the peer and set the peer's remote candidates
   nice_agent_set_remote_candidates (agent, stream_id, 1, rcands);

   // ... Wait until the signal new-selected-pair is fired ...
   // Send our message!
   nice_agent_send (agent, stream_id, 1, sizeof(buffer), buffer);

   // Anything received will be received through the cb_nice_recv callback

   // Destroy the object
   g_object_unref(agent);

   </programlisting>
 </example>
 *
 * Refer to the examples in the examples/ subdirectory of the libnice source for
 * complete examples.
 *
 */ */

/* _ADDRESS_H /**
 * SECTION:address
 * @short_description: IP address convenience library
 * @stability: Stable
 *
 * The #NiceAddress structure will allow you to easily set/get and modify an IPv4
 * or IPv6 address in order to communicate with the #NiceAgent.
 */ */

/* NICE_ADDRESS_STRING_LEN INET6_ADDRSTRLEN typedef */

/* _CANDIDATE_H /**
 * SECTION:candidate
 * @short_description: ICE candidate representation
 * @see_also: #NiceAddress
 * @stability: Stable
 *
 * A representation of an ICE candidate. Make sure you read the ICE drafts[1] to
 * understand correctly the concept of ICE candidates.
 *
 * [1] http://tools.ietf.org/wg/mmusic/draft-ietf-mmusic-ice/
 */ */

/* NICE_CANDIDATE_TYPE_PREF_HOST 120 # */
pub const NICE_CANDIDATE_TYPE_PREF_HOST: i32 = 120;

/* NICE_CANDIDATE_TYPE_PREF_PEER_REFLEXIVE 110 # */
pub const NICE_CANDIDATE_TYPE_PREF_PEER_REFLEXIVE: i32 = 110;

/* NICE_CANDIDATE_TYPE_PREF_SERVER_REFLEXIVE 100 # */
pub const NICE_CANDIDATE_TYPE_PREF_SERVER_REFLEXIVE: i32 = 100;

/* NICE_CANDIDATE_TYPE_PREF_RELAYED 60 /* Max foundation size '1*32ice-char' plus terminating NULL, ICE ID-19  */ */
pub const NICE_CANDIDATE_TYPE_PREF_RELAYED: i32 = 60;

/* NICE_CANDIDATE_MAX_FOUNDATION ( 32 + 1 ) /**
 * NiceCandidateType:
 * @NICE_CANDIDATE_TYPE_HOST: A host candidate
 * @NICE_CANDIDATE_TYPE_SERVER_REFLEXIVE: A server reflexive candidate
 * @NICE_CANDIDATE_TYPE_PEER_REFLEXIVE: A peer reflexive candidate
 * @NICE_CANDIDATE_TYPE_RELAYED: A relay candidate
 *
 * An enum represneting the type of a candidate
 */ */

/* _DEBUG_H /**
 * SECTION:debug
 * @short_description: Debug messages utility functions
 * @stability: Unstable
 *
 * <para>Libnice can output a lot of information when debug messages are enabled.
 * This can significantly help track down problems and/or understand what
 * it's doing.</para>
 *
 * <para>You can enable/disable the debug messages by calling nice_debug_enable()
 * or nice_debug_disable() and choosing whether you want only ICE debug messages
 * or also stun debug messages.</para>
 *
 * <para>By default, the debug messages are disabled, unless the environment
 * variable NICE_DEBUG is set, in which case, it must contain a comma separated
 * list of flags specifying which debug to enable.</para>
 * <para> The currently available flags are "nice", "stun", "pseudotcp",
 * "pseudotcp-verbose" or "all" to enable all debug messages.</para>
 * <para> If the 'pseudotcp' flag is enabled, then 'pseudotcp-verbose' gets
 * automatically disabled. This is to allow the use of the 'all' flag without
 * having verbose messages from pseudotcp. You can enable verbose debug messages
 * from the pseudotcp layer by specifying 'pseudotcp-verbose' without the
 * 'pseudotcp' flag.</para>
 *
 *
 * <para>This API is unstable and is subject to change at any time...
 * More flags are to come and a better API to enable/disable each flag
 * should be added.</para>
 */ */

/* NICE_TYPE_AGENT nice_agent_get_type ( ) # */

/* NICE_AGENT ( obj ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( obj ) , NICE_TYPE_AGENT , NiceAgent ) ) # */

/* NICE_AGENT_CLASS ( klass ) ( G_TYPE_CHECK_CLASS_CAST ( ( klass ) , NICE_TYPE_AGENT , NiceAgentClass ) ) # */

/* NICE_IS_AGENT ( obj ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( obj ) , NICE_TYPE_AGENT ) ) # */

/* NICE_IS_AGENT_CLASS ( klass ) ( G_TYPE_CHECK_CLASS_TYPE ( ( klass ) , NICE_TYPE_AGENT ) ) # */

/* NICE_AGENT_GET_CLASS ( obj ) ( G_TYPE_INSTANCE_GET_CLASS ( ( obj ) , NICE_TYPE_AGENT , NiceAgentClass ) ) typedef */

/* NICE_AGENT_MAX_REMOTE_CANDIDATES 25 /**
 * NiceComponentState:
 * @NICE_COMPONENT_STATE_DISCONNECTED: No activity scheduled
 * @NICE_COMPONENT_STATE_GATHERING: Gathering local candidates
 * @NICE_COMPONENT_STATE_CONNECTING: Establishing connectivity
 * @NICE_COMPONENT_STATE_CONNECTED: At least one working candidate pair
 * @NICE_COMPONENT_STATE_READY: ICE concluded, candidate pair selection
 * is now final
 * @NICE_COMPONENT_STATE_FAILED: Connectivity checks have been completed,
 * but connectivity was not established
 * @NICE_COMPONENT_STATE_LAST: Dummy state
 *
 * An enum representing the state of a component.
 * <para> See also: #NiceAgent::component-state-changed </para>
 */ */
pub const NICE_AGENT_MAX_REMOTE_CANDIDATES: i32 = 25;

/* __INTERFACES_H__ /**
 * SECTION:interfaces
 * @short_description: Utility functions to discover local network interfaces
 * @include: interfaces.h
 * @stability: Stable
 *
 * These utility functions allow the discovery of local network interfaces
 * in a portable manner, they also allow finding the local ip addresses or
 * the address allocated to a network interface.
 */ */

