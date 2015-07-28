use libc;

pub const FALSE:libc::c_int = 0;
pub const TRUE:libc::c_int = !FALSE;

/*
struct GTree
*/
#[repr(C)]
pub struct GTree;

#[repr(C)]
pub struct _GTypeCValue;

/*
struct _GQueue
		(GList *) head [struct _GList *]
		(GList *) tail [struct _GList *]
		(guint) length [unsigned int]
*/
#[repr(C)]
pub struct _GQueue {
	pub head: *mut _GList,
	pub tail: *mut _GList,
	pub length: libc::c_uint,
}

/*
struct GFlagsClass
		(GTypeClass) g_type_class [struct _GTypeClass]
		(guint) mask [unsigned int]
		(guint) n_values [unsigned int]
		(GFlagsValue *) values [struct _GFlagsValue *]
*/
#[repr(C)]
pub struct GFlagsClass {
	pub g_type_class: _GTypeClass,
	pub mask: libc::c_uint,
	pub n_values: libc::c_uint,
	pub values: *mut _GFlagsValue,
}

/*
struct GTypeInfo
		(guint16) class_size [unsigned short]
		(GBaseInitFunc) base_init [void (*)(void *)]
		(GBaseFinalizeFunc) base_finalize [void (*)(void *)]
		(GClassInitFunc) class_init [void (*)(void *, void *)]
		(GClassFinalizeFunc) class_finalize [void (*)(void *, void *)]
		(gconstpointer) class_data [const void *]
		(guint16) instance_size [unsigned short]
		(guint16) n_preallocs [unsigned short]
		(GInstanceInitFunc) instance_init [void (*)(struct _GTypeInstance *, void *)]
		(const GTypeValueTable *) value_table [const struct _GTypeValueTable *]
*/
#[repr(C)]
pub struct GTypeInfo {
	pub class_size: libc::c_ushort,
	pub base_init: Option<extern fn(*mut libc::c_void)>,
	pub base_finalize: Option<extern fn(*mut libc::c_void)>,
	pub class_init: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>,
	pub class_finalize: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>,
	pub class_data: *const libc::c_void,
	pub instance_size: libc::c_ushort,
	pub n_preallocs: libc::c_ushort,
	pub instance_init: Option<extern fn(*mut _GTypeInstance, *mut libc::c_void)>,
	pub value_table: *const _GTypeValueTable,
}

/*
struct GAllocator
*/
#[repr(C)]
pub struct GAllocator;

/*
struct GRelation
*/
#[repr(C)]
pub struct GRelation;

/*
struct GMatchInfo
*/
#[repr(C)]
pub struct GMatchInfo;

/*
struct _GParamSpecInt
		(GParamSpec) parent_instance [struct _GParamSpec]
		(gint) minimum [int]
		(gint) maximum [int]
		(gint) default_value [int]
*/
#[repr(C)]
pub struct _GParamSpecInt {
	pub parent_instance: _GParamSpec,
	pub minimum: libc::c_int,
	pub maximum: libc::c_int,
	pub default_value: libc::c_int,
}

/*
struct _GBytes
*/
#[repr(C)]
pub struct _GBytes;

/*
struct GEnumClass
		(GTypeClass) g_type_class [struct _GTypeClass]
		(gint) minimum [int]
		(gint) maximum [int]
		(guint) n_values [unsigned int]
		(GEnumValue *) values [struct _GEnumValue *]
*/
#[repr(C)]
pub struct GEnumClass {
	pub g_type_class: _GTypeClass,
	pub minimum: libc::c_int,
	pub maximum: libc::c_int,
	pub n_values: libc::c_uint,
	pub values: *mut _GEnumValue,
}

/*
struct GHashTable
*/
#[repr(C)]
pub struct GHashTable;

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
	pub log_type: libc::c_uint,
	pub n_strings: libc::c_uint,
	pub strings: *mut *mut libc::c_char,
	pub n_nums: libc::c_uint,
	pub nums: *mut libc::c_double,
}

/*
struct _GTrashStack
		(GTrashStack *) next [struct _GTrashStack *]
*/
#[repr(C)]
pub struct _GTrashStack {
	pub next: *mut _GTrashStack,
}

/*
struct GArray
		(gchar *) data [char *]
		(guint) len [unsigned int]
*/
#[repr(C)]
pub struct GArray {
	pub data: *mut libc::c_char,
	pub len: libc::c_uint,
}

/*
struct _GParamSpecULong
		(GParamSpec) parent_instance [struct _GParamSpec]
		(gulong) minimum [unsigned long]
		(gulong) maximum [unsigned long]
		(gulong) default_value [unsigned long]
*/
#[repr(C)]
pub struct _GParamSpecULong {
	pub parent_instance: _GParamSpec,
	pub minimum: libc::c_ulong,
	pub maximum: libc::c_ulong,
	pub default_value: libc::c_ulong,
}

/*
struct _GVariant
*/
#[repr(C)]
pub struct _GVariant;

/*
struct GParameter
		(const gchar *) name [const char *]
		(GValue) value [struct _GValue]
*/
#[repr(C)]
pub struct GParameter {
	pub name: *const libc::c_char,
	pub value: _GValue,
}

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
	pub long_name: *const libc::c_char,
	pub short_name: libc::c_char,
	pub flags: libc::c_int,
	pub arg: libc::c_uint,
	pub arg_data: *mut libc::c_void,
	pub description: *const libc::c_char,
	pub arg_description: *const libc::c_char,
}

/*
struct _GThreadPool
		(GFunc) func [void (*)(void *, void *)]
		(gpointer) user_data [void *]
		(gboolean) exclusive [int]
*/
#[repr(C)]
pub struct _GThreadPool {
	pub func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>,
	pub user_data: *mut libc::c_void,
	pub exclusive: libc::c_int,
}

/*
struct _GParamSpecBoxed
		(GParamSpec) parent_instance [struct _GParamSpec]
*/
#[repr(C)]
pub struct _GParamSpecBoxed {
	pub parent_instance: _GParamSpec,
}

/*
struct GMainLoop
*/
#[repr(C)]
pub struct GMainLoop;

/*
struct _GBinding
*/
#[repr(C)]
pub struct _GBinding;

/*
struct _GMemChunk
*/
#[repr(C)]
pub struct _GMemChunk;

/*
struct _GTypeQuery
		(GType) type [unsigned long]
		(const gchar *) type_name [const char *]
		(guint) class_size [unsigned int]
		(guint) instance_size [unsigned int]
*/
#[repr(C)]
pub struct _GTypeQuery {
	pub type_: libc::c_ulong,
	pub type_name: *const libc::c_char,
	pub class_size: libc::c_uint,
	pub instance_size: libc::c_uint,
}

/*
struct GSequence
*/
#[repr(C)]
pub struct GSequence;

/*
struct GObjectClass
		(GTypeClass) g_type_class [struct _GTypeClass]
		(GSList *) construct_properties [struct _GSList *]
		(GObject *(*)(GType, guint, GObjectConstructParam *)) constructor [struct _GObject *(*)(unsigned long, unsigned int, struct _GObjectConstructParam *)]
		(void (*)(GObject *, guint, const GValue *, GParamSpec *)) set_property [void (*)(struct _GObject *, unsigned int, const struct _GValue *, struct _GParamSpec *)]
		(void (*)(GObject *, guint, GValue *, GParamSpec *)) get_property [void (*)(struct _GObject *, unsigned int, struct _GValue *, struct _GParamSpec *)]
		(void (*)(GObject *)) dispose [void (*)(struct _GObject *)]
		(void (*)(GObject *)) finalize [void (*)(struct _GObject *)]
		(void (*)(GObject *, guint, GParamSpec **)) dispatch_properties_changed [void (*)(struct _GObject *, unsigned int, struct _GParamSpec **)]
		(void (*)(GObject *, GParamSpec *)) notify [void (*)(struct _GObject *, struct _GParamSpec *)]
		(void (*)(GObject *)) constructed [void (*)(struct _GObject *)]
		(gsize) flags [unsigned long]
		(gpointer [6]) pdummy [void *[6]]
*/
#[repr(C)]
pub struct GObjectClass {
	pub g_type_class: _GTypeClass,
	pub construct_properties: *mut _GSList,
	pub constructor: Option<extern fn(libc::c_ulong, libc::c_uint, *mut _GObjectConstructParam) -> *mut _GObject>,
	pub set_property: Option<extern fn(*mut _GObject, libc::c_uint, *const _GValue, *mut _GParamSpec)>,
	pub get_property: Option<extern fn(*mut _GObject, libc::c_uint, *mut _GValue, *mut _GParamSpec)>,
	pub dispose: Option<extern fn(*mut _GObject)>,
	pub finalize: Option<extern fn(*mut _GObject)>,
	pub dispatch_properties_changed: Option<extern fn(*mut _GObject, libc::c_uint, *mut *mut _GParamSpec)>,
	pub notify: Option<extern fn(*mut _GObject, *mut _GParamSpec)>,
	pub constructed: Option<extern fn(*mut _GObject)>,
	pub flags: libc::c_ulong,
	pub pdummy: [*mut libc::c_void; 6],
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
	pub tm_sec: libc::c_int,
	pub tm_min: libc::c_int,
	pub tm_hour: libc::c_int,
	pub tm_mday: libc::c_int,
	pub tm_mon: libc::c_int,
	pub tm_year: libc::c_int,
	pub tm_wday: libc::c_int,
	pub tm_yday: libc::c_int,
	pub tm_isdst: libc::c_int,
	pub tm_gmtoff: libc::c_long,
	pub tm_zone: *const libc::c_char,
}

/*
struct _GClosure
		(volatile guint) ref_count [volatile unsigned int]
		(volatile guint) meta_marshal_nouse [volatile unsigned int]
		(volatile guint) n_guards [volatile unsigned int]
		(volatile guint) n_fnotifiers [volatile unsigned int]
		(volatile guint) n_inotifiers [volatile unsigned int]
		(volatile guint) in_inotify [volatile unsigned int]
		(volatile guint) floating [volatile unsigned int]
		(volatile guint) derivative_flag [volatile unsigned int]
		(volatile guint) in_marshal [volatile unsigned int]
		(volatile guint) is_invalid [volatile unsigned int]
		(void (*)(GClosure *, GValue *, guint, const GValue *, gpointer, gpointer)) marshal [void (*)(struct _GClosure *, struct _GValue *, unsigned int, const struct _GValue *, void *, void *)]
		(gpointer) data [void *]
		(GClosureNotifyData *) notifiers [struct _GClosureNotifyData *]
*/
#[repr(C)]
pub struct _GClosure {
	pub ref_count: libc::c_uint,
	pub meta_marshal_nouse: libc::c_uint,
	pub n_guards: libc::c_uint,
	pub n_fnotifiers: libc::c_uint,
	pub n_inotifiers: libc::c_uint,
	pub in_inotify: libc::c_uint,
	pub floating: libc::c_uint,
	pub derivative_flag: libc::c_uint,
	pub in_marshal: libc::c_uint,
	pub is_invalid: libc::c_uint,
	pub marshal: Option<extern fn(*mut _GClosure, *mut _GValue, libc::c_uint, *const _GValue, *mut libc::c_void, *mut libc::c_void)>,
	pub data: *mut libc::c_void,
	pub notifiers: *mut _GClosureNotifyData,
}

/*
struct _GMainContext
*/
#[repr(C)]
pub struct _GMainContext;

/*
struct GValue
		(GType) g_type [unsigned long]
		(union _GValue::(anonymous at /usr/include/glib-2.0/gobject/gvalue.h:112:3)) 
		(union (anonymous union at /usr/include/glib-2.0/gobject/gvalue.h:112:3) [2]) data [union _GValue::(anonymous at /usr/include/glib-2.0/gobject/gvalue.h:112:3) [2]]
*/
#[repr(C)]
pub struct GValue;/* {
	pub g_type: libc::c_ulong,
	pub _: _GValue::(anonymous at /usr/include/glib-2.0/gobject/gvalue.h:112:3),
	pub data: [_GValue::(anonymous at /usr/include/glib-2.0/gobject/gvalue.h:112:3); 2],
}*/

/*
struct GSignalQuery
		(guint) signal_id [unsigned int]
		(const gchar *) signal_name [const char *]
		(GType) itype [unsigned long]
		(GSignalFlags) signal_flags [GSignalFlags]
		(GType) return_type [unsigned long]
		(guint) n_params [unsigned int]
		(const GType *) param_types [const unsigned long *]
*/
#[repr(C)]
pub struct GSignalQuery {
	pub signal_id: libc::c_uint,
	pub signal_name: *const libc::c_char,
	pub itype: libc::c_ulong,
	pub signal_flags: libc::c_uint,
	pub return_type: libc::c_ulong,
	pub n_params: libc::c_uint,
	pub param_types: *const libc::c_ulong,
}

/*
struct _GArray
		(gchar *) data [char *]
		(guint) len [unsigned int]
*/
#[repr(C)]
pub struct _GArray {
	pub data: *mut libc::c_char,
	pub len: libc::c_uint,
}

/*
struct GPrivate
		(gpointer) p [void *]
		(GDestroyNotify) notify [void (*)(void *)]
		(gpointer [2]) future [void *[2]]
*/
#[repr(C)]
pub struct GPrivate {
	pub p: *mut libc::c_void,
	pub notify: Option<extern fn(*mut libc::c_void)>,
	pub future: [*mut libc::c_void; 2],
}

/*
struct _GTimeZone
*/
#[repr(C)]
pub struct _GTimeZone;

/*
struct GQueue
		(GList *) head [struct _GList *]
		(GList *) tail [struct _GList *]
		(guint) length [unsigned int]
*/
#[repr(C)]
pub struct GQueue {
	pub head: *mut _GList,
	pub tail: *mut _GList,
	pub length: libc::c_uint,
}

/*
struct GTimer
*/
#[repr(C)]
pub struct GTimer;

/*
struct GVariantIter
		(gsize [16]) x [unsigned long [16]]
*/
#[repr(C)]
pub struct GVariantIter {
	pub x: [libc::c_ulong; 16],
}

/*
struct _GMainLoop
*/
#[repr(C)]
pub struct _GMainLoop;

/*
struct _GThread
		(GThreadFunc) func [void *(*)(void *)]
		(gpointer) data [void *]
		(gboolean) joinable [int]
		(GThreadPriority) priority [GThreadPriority]
*/
#[repr(C)]
pub struct _GThread {
	pub func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>,
	pub data: *mut libc::c_void,
	pub joinable: libc::c_int,
	pub priority: libc::c_uint,
}

/*
struct GInterfaceInfo
		(GInterfaceInitFunc) interface_init [void (*)(void *, void *)]
		(GInterfaceFinalizeFunc) interface_finalize [void (*)(void *, void *)]
		(gpointer) interface_data [void *]
*/
#[repr(C)]
pub struct GInterfaceInfo {
	pub interface_init: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>,
	pub interface_finalize: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>,
	pub interface_data: *mut libc::c_void,
}

/*
struct _GRecMutex
		(gpointer) p [void *]
		(guint [2]) i [unsigned int [2]]
*/
#[repr(C)]
pub struct _GRecMutex {
	pub p: *mut libc::c_void,
	pub i: [libc::c_uint; 2],
}

/*
struct GWeakRef
		(union GWeakRef::(anonymous at /usr/include/glib-2.0/gobject/gobject.h:656:5)) 
		(union (anonymous union at /usr/include/glib-2.0/gobject/gobject.h:656:5)) priv [union GWeakRef::(anonymous at /usr/include/glib-2.0/gobject/gobject.h:656:5)]
*/
#[repr(C)]
pub struct GWeakRef;/* {
	pub _: GWeakRef::(anonymous at /usr/include/glib-2.0/gobject/gobject.h:656:5),
	pub priv_: GWeakRef::(anonymous at /usr/include/glib-2.0/gobject/gobject.h:656:5),
}*/

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
	pub malloc: Option<extern fn(libc::c_ulong) -> *mut libc::c_void>,
	pub realloc: Option<extern fn(*mut libc::c_void, libc::c_ulong) -> *mut libc::c_void>,
	pub free: Option<extern fn(*mut libc::c_void)>,
	pub calloc: Option<extern fn(libc::c_ulong, libc::c_ulong) -> *mut libc::c_void>,
	pub try_malloc: Option<extern fn(libc::c_ulong) -> *mut libc::c_void>,
	pub try_realloc: Option<extern fn(*mut libc::c_void, libc::c_ulong) -> *mut libc::c_void>,
}

/*
struct GTypeValueTable
		(void (*)(GValue *)) value_init [void (*)(struct _GValue *)]
		(void (*)(GValue *)) value_free [void (*)(struct _GValue *)]
		(void (*)(const GValue *, GValue *)) value_copy [void (*)(const struct _GValue *, struct _GValue *)]
		(gpointer (*)(const GValue *)) value_peek_pointer [void *(*)(const struct _GValue *)]
		(const gchar *) collect_format [const char *]
		(gchar *(*)(GValue *, guint, GTypeCValue *, guint)) collect_value [char *(*)(struct _GValue *, unsigned int, union _GTypeCValue *, unsigned int)]
		(const gchar *) lcopy_format [const char *]
		(gchar *(*)(const GValue *, guint, GTypeCValue *, guint)) lcopy_value [char *(*)(const struct _GValue *, unsigned int, union _GTypeCValue *, unsigned int)]
*/
#[repr(C)]
pub struct GTypeValueTable {
	pub value_init: Option<extern fn(*mut _GValue)>,
	pub value_free: Option<extern fn(*mut _GValue)>,
	pub value_copy: Option<extern fn(*const _GValue, *mut _GValue)>,
	pub value_peek_pointer: Option<extern fn(*const _GValue) -> *mut libc::c_void>,
	pub collect_format: *const libc::c_char,
	pub collect_value: Option<extern fn(*mut _GValue, libc::c_uint, *mut _GTypeCValue, libc::c_uint) -> *mut libc::c_char>,
	pub lcopy_format: *const libc::c_char,
	pub lcopy_value: Option<extern fn(*const _GValue, libc::c_uint, *mut _GTypeCValue, libc::c_uint) -> *mut libc::c_char>,
}

/*
struct GTimeVal
		(glong) tv_sec [long]
		(glong) tv_usec [long]
*/
#[repr(C)]
pub struct GTimeVal {
	pub tv_sec: libc::c_long,
	pub tv_usec: libc::c_long,
}

/*
struct _GDateTime
*/
#[repr(C)]
pub struct _GDateTime;

/*
struct _GRegex
*/
#[repr(C)]
pub struct _GRegex;

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
	pub seq_id: libc::c_ulong,
	pub hook_size: libc::c_uint,
	pub is_setup: libc::c_uint,
	pub hooks: *mut _GHook,
	pub dummy3: *mut libc::c_void,
	pub finalize_hook: Option<extern fn(*mut _GHookList, *mut _GHook)>,
	pub dummy: [*mut libc::c_void; 2],
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
	pub user_data: *mut libc::c_void,
	pub max_parse_errors: libc::c_uint,
	pub parse_errors: libc::c_uint,
	pub input_name: *const libc::c_char,
	pub qdata: *mut _GData,
	pub config: *mut _GScannerConfig,
	pub token: libc::c_uint,
	pub value: _GTokenValue,
	pub line: libc::c_uint,
	pub position: libc::c_uint,
	pub next_token: libc::c_uint,
	pub next_value: _GTokenValue,
	pub next_line: libc::c_uint,
	pub next_position: libc::c_uint,
	pub symbol_table: *mut _GHashTable,
	pub input_fd: libc::c_int,
	pub text: *const libc::c_char,
	pub text_end: *const libc::c_char,
	pub buffer: *mut libc::c_char,
	pub scope_id: libc::c_uint,
	pub msg_handler: Option<extern fn(*mut _GScanner, *mut libc::c_char, libc::c_int)>,
}

/*
struct _GParamSpecChar
		(GParamSpec) parent_instance [struct _GParamSpec]
		(gint8) minimum [signed char]
		(gint8) maximum [signed char]
		(gint8) default_value [signed char]
*/
#[repr(C)]
pub struct _GParamSpecChar {
	pub parent_instance: _GParamSpec,
	pub minimum: libc::c_char,
	pub maximum: libc::c_char,
	pub default_value: libc::c_char,
}

/*
struct GMainContext
*/
#[repr(C)]
pub struct GMainContext;

/*
struct GOnce
		(volatile GOnceStatus) status [volatile GOnceStatus]
		(volatile gpointer) retval [void *volatile]
*/
#[repr(C)]
pub struct GOnce {
	pub status: libc::c_uint,
	pub retval: *mut libc::c_void,
}

/*
struct _GMatchInfo
*/
#[repr(C)]
pub struct _GMatchInfo;

/*
struct GTypeModule
		(GObject) parent_instance [struct _GObject]
		(guint) use_count [unsigned int]
		(GSList *) type_infos [struct _GSList *]
		(GSList *) interface_infos [struct _GSList *]
		(gchar *) name [char *]
*/
#[repr(C)]
pub struct GTypeModule {
	pub parent_instance: _GObject,
	pub use_count: libc::c_uint,
	pub type_infos: *mut _GSList,
	pub interface_infos: *mut _GSList,
	pub name: *mut libc::c_char,
}

/*
struct _GTypeInstance
		(GTypeClass *) g_class [struct _GTypeClass *]
*/
#[repr(C)]
pub struct _GTypeInstance {
	pub g_class: *mut _GTypeClass,
}

/*
struct _GHmac
*/
#[repr(C)]
pub struct _GHmac;

/*
struct GValueArray
		(guint) n_values [unsigned int]
		(GValue *) values [struct _GValue *]
		(guint) n_prealloced [unsigned int]
*/
#[repr(C)]
pub struct GValueArray {
	pub n_values: libc::c_uint,
	pub values: *mut _GValue,
	pub n_prealloced: libc::c_uint,
}

/*
struct _GParamSpecFloat
		(GParamSpec) parent_instance [struct _GParamSpec]
		(gfloat) minimum [float]
		(gfloat) maximum [float]
		(gfloat) default_value [float]
		(gfloat) epsilon [float]
*/
#[repr(C)]
pub struct _GParamSpecFloat {
	pub parent_instance: _GParamSpec,
	pub minimum: libc::c_float,
	pub maximum: libc::c_float,
	pub default_value: libc::c_float,
	pub epsilon: libc::c_float,
}

/*
struct _GObject
		(GTypeInstance) g_type_instance [struct _GTypeInstance]
		(volatile guint) ref_count [volatile unsigned int]
		(GData *) qdata [struct _GData *]
*/
#[repr(C)]
pub struct _GObject {
	pub g_type_instance: _GTypeInstance,
	pub ref_count: libc::c_uint,
	pub qdata: *mut _GData,
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
	pub ref_count: libc::c_int,
	pub funcs: *mut _GIOFuncs,
	pub encoding: *mut libc::c_char,
	pub read_cd: *mut _GIConv,
	pub write_cd: *mut _GIConv,
	pub line_term: *mut libc::c_char,
	pub line_term_len: libc::c_uint,
	pub buf_size: libc::c_ulong,
	pub read_buf: *mut _GString,
	pub encoded_read_buf: *mut _GString,
	pub write_buf: *mut _GString,
	pub partial_write_buf: [libc::c_char; 6],
	pub use_buffer: libc::c_uint,
	pub do_encode: libc::c_uint,
	pub close_on_unref: libc::c_uint,
	pub is_readable: libc::c_uint,
	pub is_writeable: libc::c_uint,
	pub is_seekable: libc::c_uint,
	pub reserved1: *mut libc::c_void,
	pub reserved2: *mut libc::c_void,
}

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
	pub data: *mut libc::c_void,
	pub next: *mut _GNode,
	pub prev: *mut _GNode,
	pub parent: *mut _GNode,
	pub children: *mut _GNode,
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
	pub data: *mut libc::c_void,
	pub next: *mut _GHook,
	pub prev: *mut _GHook,
	pub ref_count: libc::c_uint,
	pub hook_id: libc::c_ulong,
	pub flags: libc::c_uint,
	pub func: *mut libc::c_void,
	pub destroy: Option<extern fn(*mut libc::c_void)>,
}

/*
struct _GData
*/
#[repr(C)]
pub struct _GData;

/*
struct _GFlagsClass
		(GTypeClass) g_type_class [struct _GTypeClass]
		(guint) mask [unsigned int]
		(guint) n_values [unsigned int]
		(GFlagsValue *) values [struct _GFlagsValue *]
*/
#[repr(C)]
pub struct _GFlagsClass {
	pub g_type_class: _GTypeClass,
	pub mask: libc::c_uint,
	pub n_values: libc::c_uint,
	pub values: *mut _GFlagsValue,
}

/*
struct _GParamSpec
		(GTypeInstance) g_type_instance [struct _GTypeInstance]
		(const gchar *) name [const char *]
		(GParamFlags) flags [GParamFlags]
		(GType) value_type [unsigned long]
		(GType) owner_type [unsigned long]
		(gchar *) _nick [char *]
		(gchar *) _blurb [char *]
		(GData *) qdata [struct _GData *]
		(guint) ref_count [unsigned int]
		(guint) param_id [unsigned int]
*/
#[repr(C)]
pub struct _GParamSpec {
	pub g_type_instance: _GTypeInstance,
	pub name: *const libc::c_char,
	pub flags: libc::c_uint,
	pub value_type: libc::c_ulong,
	pub owner_type: libc::c_ulong,
	pub _nick: *mut libc::c_char,
	pub _blurb: *mut libc::c_char,
	pub qdata: *mut _GData,
	pub ref_count: libc::c_uint,
	pub param_id: libc::c_uint,
}

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
	pub items: *mut _GList,
	pub func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_char>,
	pub prefix: *mut libc::c_char,
	pub cache: *mut _GList,
	pub strncmp_func: Option<extern fn(*const libc::c_char, *const libc::c_char, libc::c_ulong) -> libc::c_int>,
}

/*
struct GTypePlugin
*/
#[repr(C)]
pub struct GTypePlugin;

/*
struct _GParamSpecParam
		(GParamSpec) parent_instance [struct _GParamSpec]
*/
#[repr(C)]
pub struct _GParamSpecParam {
	pub parent_instance: _GParamSpec,
}

/*
struct GVariantType
*/
#[repr(C)]
pub struct GVariantType;

/*
struct _GPrivate
		(gpointer) p [void *]
		(GDestroyNotify) notify [void (*)(void *)]
		(gpointer [2]) future [void *[2]]
*/
#[repr(C)]
pub struct _GPrivate {
	pub p: *mut libc::c_void,
	pub notify: Option<extern fn(*mut libc::c_void)>,
	pub future: [*mut libc::c_void; 2],
}

/*
struct GString
		(gchar *) str [char *]
		(gsize) len [unsigned long]
		(gsize) allocated_len [unsigned long]
*/
#[repr(C)]
pub struct GString {
	pub str: *mut libc::c_char,
	pub len: libc::c_ulong,
	pub allocated_len: libc::c_ulong,
}

/*
struct _GMarkupParseContext
*/
#[repr(C)]
pub struct _GMarkupParseContext;

/*
struct GThread
		(GThreadFunc) func [void *(*)(void *)]
		(gpointer) data [void *]
		(gboolean) joinable [int]
		(GThreadPriority) priority [GThreadPriority]
*/
#[repr(C)]
pub struct GThread {
	pub func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>,
	pub data: *mut libc::c_void,
	pub joinable: libc::c_int,
	pub priority: libc::c_uint,
}

/*
struct GVariantBuilder
		(gsize [16]) x [unsigned long [16]]
*/
#[repr(C)]
pub struct GVariantBuilder {
	pub x: [libc::c_ulong; 16],
}

/*
struct _GVariantBuilder
		(gsize [16]) x [unsigned long [16]]
*/
#[repr(C)]
pub struct _GVariantBuilder {
	pub x: [libc::c_ulong; 16],
}

/*
struct _GParamSpecClass
		(GTypeClass) g_type_class [struct _GTypeClass]
		(GType) value_type [unsigned long]
		(void (*)(GParamSpec *)) finalize [void (*)(struct _GParamSpec *)]
		(void (*)(GParamSpec *, GValue *)) value_set_default [void (*)(struct _GParamSpec *, struct _GValue *)]
		(gboolean (*)(GParamSpec *, GValue *)) value_validate [int (*)(struct _GParamSpec *, struct _GValue *)]
		(gint (*)(GParamSpec *, const GValue *, const GValue *)) values_cmp [int (*)(struct _GParamSpec *, const struct _GValue *, const struct _GValue *)]
		(gpointer [4]) dummy [void *[4]]
*/
#[repr(C)]
pub struct _GParamSpecClass {
	pub g_type_class: _GTypeClass,
	pub value_type: libc::c_ulong,
	pub finalize: Option<extern fn(*mut _GParamSpec)>,
	pub value_set_default: Option<extern fn(*mut _GParamSpec, *mut _GValue)>,
	pub value_validate: Option<extern fn(*mut _GParamSpec, *mut _GValue) -> libc::c_int>,
	pub values_cmp: Option<extern fn(*mut _GParamSpec, *const _GValue, *const _GValue) -> libc::c_int>,
	pub dummy: [*mut libc::c_void; 4],
}

/*
struct _GParamSpecVariant
		(GParamSpec) parent_instance [struct _GParamSpec]
		(GVariantType *) type [struct _GVariantType *]
		(GVariant *) default_value [struct _GVariant *]
		(gpointer [4]) padding [void *[4]]
*/
#[repr(C)]
pub struct _GParamSpecVariant {
	pub parent_instance: _GParamSpec,
	pub type_: *mut _GVariantType,
	pub default_value: *mut _GVariant,
	pub padding: [*mut libc::c_void; 4],
}

/*
struct GTestCase
*/
#[repr(C)]
pub struct GTestCase;

/*
struct _GClosureNotifyData
		(gpointer) data [void *]
		(GClosureNotify) notify [void (*)(void *, struct _GClosure *)]
*/
#[repr(C)]
pub struct _GClosureNotifyData {
	pub data: *mut libc::c_void,
	pub notify: Option<extern fn(*mut libc::c_void, *mut _GClosure)>,
}

/*
struct GRecMutex
		(gpointer) p [void *]
		(guint [2]) i [unsigned int [2]]
*/
#[repr(C)]
pub struct GRecMutex {
	pub p: *mut libc::c_void,
	pub i: [libc::c_uint; 2],
}

/*
struct _GParamSpecBoolean
		(GParamSpec) parent_instance [struct _GParamSpec]
		(gboolean) default_value [int]
*/
#[repr(C)]
pub struct _GParamSpecBoolean {
	pub parent_instance: _GParamSpec,
	pub default_value: libc::c_int,
}

/*
struct _GSignalQuery
		(guint) signal_id [unsigned int]
		(const gchar *) signal_name [const char *]
		(GType) itype [unsigned long]
		(GSignalFlags) signal_flags [GSignalFlags]
		(GType) return_type [unsigned long]
		(guint) n_params [unsigned int]
		(const GType *) param_types [const unsigned long *]
*/
#[repr(C)]
pub struct _GSignalQuery {
	pub signal_id: libc::c_uint,
	pub signal_name: *const libc::c_char,
	pub itype: libc::c_ulong,
	pub signal_flags: libc::c_uint,
	pub return_type: libc::c_ulong,
	pub n_params: libc::c_uint,
	pub param_types: *const libc::c_ulong,
}

/*
struct _GParamSpecObject
		(GParamSpec) parent_instance [struct _GParamSpec]
*/
#[repr(C)]
pub struct _GParamSpecObject {
	pub parent_instance: _GParamSpec,
}

/*
struct _GParamSpecDouble
		(GParamSpec) parent_instance [struct _GParamSpec]
		(gdouble) minimum [double]
		(gdouble) maximum [double]
		(gdouble) default_value [double]
		(gdouble) epsilon [double]
*/
#[repr(C)]
pub struct _GParamSpecDouble {
	pub parent_instance: _GParamSpec,
	pub minimum: libc::c_double,
	pub maximum: libc::c_double,
	pub default_value: libc::c_double,
	pub epsilon: libc::c_double,
}

/*
struct _GStringChunk
*/
#[repr(C)]
pub struct _GStringChunk;

/*
struct _GSource
		(gpointer) callback_data [void *]
		(GSourceCallbackFuncs *) callback_funcs [struct _GSourceCallbackFuncs *]
		(const GSourceFuncs *) source_funcs [const struct _GSourceFuncs *]
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
	pub callback_data: *mut libc::c_void,
	pub callback_funcs: *mut _GSourceCallbackFuncs,
	pub source_funcs: *const _GSourceFuncs,
	pub ref_count: libc::c_uint,
	pub context: *mut _GMainContext,
	pub priority: libc::c_int,
	pub flags: libc::c_uint,
	pub source_id: libc::c_uint,
	pub poll_fds: *mut _GSList,
	pub prev: *mut _GSource,
	pub next: *mut _GSource,
	pub name: *mut libc::c_char,
	pub priv_: *mut _GSourcePrivate,
}

/*
struct _GSequence
*/
#[repr(C)]
pub struct _GSequence;

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
	pub julian_days: libc::c_uint,
	pub julian: libc::c_uint,
	pub dmy: libc::c_uint,
	pub day: libc::c_uint,
	pub month: libc::c_uint,
	pub year: libc::c_uint,
}

/*
struct _GStaticPrivate
		(guint) index [unsigned int]
*/
#[repr(C)]
pub struct _GStaticPrivate {
	pub index: libc::c_uint,
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
	pub mutex: GStaticMutex,
	pub read_cond: *mut _GCond,
	pub write_cond: *mut _GCond,
	pub read_counter: libc::c_uint,
	pub have_writer: libc::c_int,
	pub want_to_read: libc::c_uint,
	pub want_to_write: libc::c_uint,
}

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
	pub items: *mut _GList,
	pub func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_char>,
	pub prefix: *mut libc::c_char,
	pub cache: *mut _GList,
	pub strncmp_func: Option<extern fn(*const libc::c_char, *const libc::c_char, libc::c_ulong) -> libc::c_int>,
}

/*
struct _GTypeModuleClass
		(GObjectClass) parent_class [struct _GObjectClass]
		(gboolean (*)(GTypeModule *)) load [int (*)(struct _GTypeModule *)]
		(void (*)(GTypeModule *)) unload [void (*)(struct _GTypeModule *)]
		(void (*)(void)) reserved1 [void (*)(void)]
		(void (*)(void)) reserved2 [void (*)(void)]
		(void (*)(void)) reserved3 [void (*)(void)]
		(void (*)(void)) reserved4 [void (*)(void)]
*/
#[repr(C)]
pub struct _GTypeModuleClass {
	pub parent_class: _GObjectClass,
	pub load: Option<extern fn(*mut _GTypeModule) -> libc::c_int>,
	pub unload: Option<extern fn(*mut _GTypeModule)>,
	pub reserved1: Option<extern fn()>,
	pub reserved2: Option<extern fn()>,
	pub reserved3: Option<extern fn()>,
	pub reserved4: Option<extern fn()>,
}

/*
struct GTimeZone
*/
#[repr(C)]
pub struct GTimeZone;

/*
struct GTypeClass
		(GType) g_type [unsigned long]
*/
#[repr(C)]
pub struct GTypeClass {
	pub g_type: libc::c_ulong,
}

/*
struct _GBookmarkFile
*/
#[repr(C)]
pub struct _GBookmarkFile;

/*
struct _GVariantType
*/
#[repr(C)]
pub struct _GVariantType;

/*
struct GOptionGroup
*/
#[repr(C)]
pub struct GOptionGroup;

/*
struct GVariantDict
		(gsize [16]) x [unsigned long [16]]
*/
#[repr(C)]
pub struct GVariantDict {
	pub x: [libc::c_ulong; 16],
}

/*
struct _GParamSpecPointer
		(GParamSpec) parent_instance [struct _GParamSpec]
*/
#[repr(C)]
pub struct _GParamSpecPointer {
	pub parent_instance: _GParamSpec,
}

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
	pub user_data: *mut libc::c_void,
	pub max_parse_errors: libc::c_uint,
	pub parse_errors: libc::c_uint,
	pub input_name: *const libc::c_char,
	pub qdata: *mut _GData,
	pub config: *mut _GScannerConfig,
	pub token: libc::c_uint,
	pub value: _GTokenValue,
	pub line: libc::c_uint,
	pub position: libc::c_uint,
	pub next_token: libc::c_uint,
	pub next_value: _GTokenValue,
	pub next_line: libc::c_uint,
	pub next_position: libc::c_uint,
	pub symbol_table: *mut _GHashTable,
	pub input_fd: libc::c_int,
	pub text: *const libc::c_char,
	pub text_end: *const libc::c_char,
	pub buffer: *mut libc::c_char,
	pub scope_id: libc::c_uint,
	pub msg_handler: Option<extern fn(*mut _GScanner, *mut libc::c_char, libc::c_int)>,
}

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
	pub julian_days: libc::c_uint,
	pub julian: libc::c_uint,
	pub dmy: libc::c_uint,
	pub day: libc::c_uint,
	pub month: libc::c_uint,
	pub year: libc::c_uint,
}

/*
struct GFlagsValue
		(guint) value [unsigned int]
		(const gchar *) value_name [const char *]
		(const gchar *) value_nick [const char *]
*/
#[repr(C)]
pub struct GFlagsValue {
	pub value: libc::c_uint,
	pub value_name: *const libc::c_char,
	pub value_nick: *const libc::c_char,
}

/*
struct _GTypeFundamentalInfo
		(GTypeFundamentalFlags) type_flags [GTypeFundamentalFlags]
*/
#[repr(C)]
pub struct _GTypeFundamentalInfo {
	pub type_flags: libc::c_uint,
}

/*
struct _GObjectClass
		(GTypeClass) g_type_class [struct _GTypeClass]
		(GSList *) construct_properties [struct _GSList *]
		(GObject *(*)(GType, guint, GObjectConstructParam *)) constructor [struct _GObject *(*)(unsigned long, unsigned int, struct _GObjectConstructParam *)]
		(void (*)(GObject *, guint, const GValue *, GParamSpec *)) set_property [void (*)(struct _GObject *, unsigned int, const struct _GValue *, struct _GParamSpec *)]
		(void (*)(GObject *, guint, GValue *, GParamSpec *)) get_property [void (*)(struct _GObject *, unsigned int, struct _GValue *, struct _GParamSpec *)]
		(void (*)(GObject *)) dispose [void (*)(struct _GObject *)]
		(void (*)(GObject *)) finalize [void (*)(struct _GObject *)]
		(void (*)(GObject *, guint, GParamSpec **)) dispatch_properties_changed [void (*)(struct _GObject *, unsigned int, struct _GParamSpec **)]
		(void (*)(GObject *, GParamSpec *)) notify [void (*)(struct _GObject *, struct _GParamSpec *)]
		(void (*)(GObject *)) constructed [void (*)(struct _GObject *)]
		(gsize) flags [unsigned long]
		(gpointer [6]) pdummy [void *[6]]
*/
#[repr(C)]
pub struct _GObjectClass {
	pub g_type_class: _GTypeClass,
	pub construct_properties: *mut _GSList,
	pub constructor: Option<extern fn(libc::c_ulong, libc::c_uint, *mut _GObjectConstructParam) -> *mut _GObject>,
	pub set_property: Option<extern fn(*mut _GObject, libc::c_uint, *const _GValue, *mut _GParamSpec)>,
	pub get_property: Option<extern fn(*mut _GObject, libc::c_uint, *mut _GValue, *mut _GParamSpec)>,
	pub dispose: Option<extern fn(*mut _GObject)>,
	pub finalize: Option<extern fn(*mut _GObject)>,
	pub dispatch_properties_changed: Option<extern fn(*mut _GObject, libc::c_uint, *mut *mut _GParamSpec)>,
	pub notify: Option<extern fn(*mut _GObject, *mut _GParamSpec)>,
	pub constructed: Option<extern fn(*mut _GObject)>,
	pub flags: libc::c_ulong,
	pub pdummy: [*mut libc::c_void; 6],
}

/*
struct GBookmarkFile
*/
#[repr(C)]
pub struct GBookmarkFile;

/*
struct _GSourcePrivate
*/
#[repr(C)]
pub struct _GSourcePrivate;

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
	pub dummy1: *mut libc::c_void,
	pub dummy2: *mut libc::c_void,
	pub dummy3: *mut libc::c_void,
	pub dummy4: libc::c_int,
	pub dummy5: libc::c_int,
	pub dummy6: *mut libc::c_void,
}

/*
struct _GRelation
*/
#[repr(C)]
pub struct _GRelation;

/*
struct _GMappedFile
*/
#[repr(C)]
pub struct _GMappedFile;

/*
struct GRegex
*/
#[repr(C)]
pub struct GRegex;

/*
struct GPtrArray
		(gpointer *) pdata [void **]
		(guint) len [unsigned int]
*/
#[repr(C)]
pub struct GPtrArray {
	pub pdata: *mut *mut libc::c_void,
	pub len: libc::c_uint,
}

/*
struct _GCond
		(gpointer) p [void *]
		(guint [2]) i [unsigned int [2]]
*/
#[repr(C)]
pub struct _GCond {
	pub p: *mut libc::c_void,
	pub i: [libc::c_uint; 2],
}

/*
struct GCache
*/
#[repr(C)]
pub struct GCache;

/*
struct _GTuples
		(guint) len [unsigned int]
*/
#[repr(C)]
pub struct _GTuples {
	pub len: libc::c_uint,
}

/*
struct GError
		(GQuark) domain [unsigned int]
		(gint) code [int]
		(gchar *) message [char *]
*/
#[repr(C)]
pub struct GError {
	pub domain: libc::c_uint,
	pub code: libc::c_int,
	pub message: *mut libc::c_char,
}

/*
struct _GAllocator
*/
#[repr(C)]
pub struct _GAllocator;

/*
struct _GEnumValue
		(gint) value [int]
		(const gchar *) value_name [const char *]
		(const gchar *) value_nick [const char *]
*/
#[repr(C)]
pub struct _GEnumValue {
	pub value: libc::c_int,
	pub value_name: *const libc::c_char,
	pub value_nick: *const libc::c_char,
}

/*
struct _GValueArray
		(guint) n_values [unsigned int]
		(GValue *) values [struct _GValue *]
		(guint) n_prealloced [unsigned int]
*/
#[repr(C)]
pub struct _GValueArray {
	pub n_values: libc::c_uint,
	pub values: *mut _GValue,
	pub n_prealloced: libc::c_uint,
}

/*
struct GDateTime
*/
#[repr(C)]
pub struct GDateTime;

/*
struct _GVariantIter
		(gsize [16]) x [unsigned long [16]]
*/
#[repr(C)]
pub struct _GVariantIter {
	pub x: [libc::c_ulong; 16],
}

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
	pub seq_id: libc::c_ulong,
	pub hook_size: libc::c_uint,
	pub is_setup: libc::c_uint,
	pub hooks: *mut _GHook,
	pub dummy3: *mut libc::c_void,
	pub finalize_hook: Option<extern fn(*mut _GHookList, *mut _GHook)>,
	pub dummy: [*mut libc::c_void; 2],
}

/*
struct GSList
		(gpointer) data [void *]
		(GSList *) next [struct _GSList *]
*/
#[repr(C)]
pub struct GSList {
	pub data: *mut libc::c_void,
	pub next: *mut _GSList,
}

/*
struct _GOnce
		(volatile GOnceStatus) status [volatile GOnceStatus]
		(volatile gpointer) retval [void *volatile]
*/
#[repr(C)]
pub struct _GOnce {
	pub status: libc::c_uint,
	pub retval: *mut libc::c_void,
}

/*
struct _GInterfaceInfo
		(GInterfaceInitFunc) interface_init [void (*)(void *, void *)]
		(GInterfaceFinalizeFunc) interface_finalize [void (*)(void *, void *)]
		(gpointer) interface_data [void *]
*/
#[repr(C)]
pub struct _GInterfaceInfo {
	pub interface_init: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>,
	pub interface_finalize: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>,
	pub interface_data: *mut libc::c_void,
}

/*
struct _GStaticRecMutex
		(GStaticMutex) mutex [GStaticMutex]
		(guint) depth [unsigned int]
		(union _GStaticRecMutex::(anonymous at /usr/include/glib-2.0/glib/deprecated/gthread.h:164:3)) 
		(union (anonymous union at /usr/include/glib-2.0/glib/deprecated/gthread.h:164:3)) unused [union _GStaticRecMutex::(anonymous at /usr/include/glib-2.0/glib/deprecated/gthread.h:164:3)]
*/
#[repr(C)]
pub struct _GStaticRecMutex;/* {
	pub mutex: GStaticMutex,
	pub depth: libc::c_uint,
	pub _: _GStaticRecMutex::(anonymous at /usr/include/glib-2.0/glib/deprecated/gthread.h:164:3),
	pub unused: _GStaticRecMutex::(anonymous at /usr/include/glib-2.0/glib/deprecated/gthread.h:164:3),
}*/

/*
struct _GParamSpecUInt
		(GParamSpec) parent_instance [struct _GParamSpec]
		(guint) minimum [unsigned int]
		(guint) maximum [unsigned int]
		(guint) default_value [unsigned int]
*/
#[repr(C)]
pub struct _GParamSpecUInt {
	pub parent_instance: _GParamSpec,
	pub minimum: libc::c_uint,
	pub maximum: libc::c_uint,
	pub default_value: libc::c_uint,
}

/*
struct _GByteArray
		(guint8 *) data [unsigned char *]
		(guint) len [unsigned int]
*/
#[repr(C)]
pub struct _GByteArray {
	pub data: *mut libc::c_uchar,
	pub len: libc::c_uint,
}

/*
struct GCond
		(gpointer) p [void *]
		(guint [2]) i [unsigned int [2]]
*/
#[repr(C)]
pub struct GCond {
	pub p: *mut libc::c_void,
	pub i: [libc::c_uint; 2],
}

/*
struct _GObjectConstructParam
		(GParamSpec *) pspec [struct _GParamSpec *]
		(GValue *) value [struct _GValue *]
*/
#[repr(C)]
pub struct _GObjectConstructParam {
	pub pspec: *mut _GParamSpec,
	pub value: *mut _GValue,
}

/*
struct GSource
		(gpointer) callback_data [void *]
		(GSourceCallbackFuncs *) callback_funcs [struct _GSourceCallbackFuncs *]
		(const GSourceFuncs *) source_funcs [const struct _GSourceFuncs *]
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
	pub callback_data: *mut libc::c_void,
	pub callback_funcs: *mut _GSourceCallbackFuncs,
	pub source_funcs: *const _GSourceFuncs,
	pub ref_count: libc::c_uint,
	pub context: *mut _GMainContext,
	pub priority: libc::c_int,
	pub flags: libc::c_uint,
	pub source_id: libc::c_uint,
	pub poll_fds: *mut _GSList,
	pub prev: *mut _GSource,
	pub next: *mut _GSource,
	pub name: *mut libc::c_char,
	pub priv_: *mut _GSourcePrivate,
}

/*
struct _GChecksum
*/
#[repr(C)]
pub struct _GChecksum;

/*
struct GThreadPool
		(GFunc) func [void (*)(void *, void *)]
		(gpointer) user_data [void *]
		(gboolean) exclusive [int]
*/
#[repr(C)]
pub struct GThreadPool {
	pub func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>,
	pub user_data: *mut libc::c_void,
	pub exclusive: libc::c_int,
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
	pub mutex_new: Option<extern fn() -> *mut _GMutex>,
	pub mutex_lock: Option<extern fn(*mut _GMutex)>,
	pub mutex_trylock: Option<extern fn(*mut _GMutex) -> libc::c_int>,
	pub mutex_unlock: Option<extern fn(*mut _GMutex)>,
	pub mutex_free: Option<extern fn(*mut _GMutex)>,
	pub cond_new: Option<extern fn() -> *mut _GCond>,
	pub cond_signal: Option<extern fn(*mut _GCond)>,
	pub cond_broadcast: Option<extern fn(*mut _GCond)>,
	pub cond_wait: Option<extern fn(*mut _GCond, *mut _GMutex)>,
	pub cond_timed_wait: Option<extern fn(*mut _GCond, *mut _GMutex, *mut _GTimeVal) -> libc::c_int>,
	pub cond_free: Option<extern fn(*mut _GCond)>,
	pub private_new: Option<extern fn(Option<extern fn(*mut libc::c_void)>) -> *mut _GPrivate>,
	pub private_get: Option<extern fn(*mut _GPrivate) -> *mut libc::c_void>,
	pub private_set: Option<extern fn(*mut _GPrivate, *mut libc::c_void)>,
	pub thread_create: Option<extern fn(Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>, *mut libc::c_void, libc::c_ulong, libc::c_int, libc::c_int, libc::c_uint, *mut libc::c_void, *mut *mut _GError)>,
	pub thread_yield: Option<extern fn()>,
	pub thread_join: Option<extern fn(*mut libc::c_void)>,
	pub thread_exit: Option<extern fn()>,
	pub thread_set_priority: Option<extern fn(*mut libc::c_void, libc::c_uint)>,
	pub thread_self: Option<extern fn(*mut libc::c_void)>,
	pub thread_equal: Option<extern fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int>,
}

#[repr(C)]
pub struct _GMutex;

#[repr(C)]
pub struct _GTokenValue;

#[repr(C)]
pub struct _GTokenCValue;

/*
struct _GTypeModule
		(GObject) parent_instance [struct _GObject]
		(guint) use_count [unsigned int]
		(GSList *) type_infos [struct _GSList *]
		(GSList *) interface_infos [struct _GSList *]
		(gchar *) name [char *]
*/
#[repr(C)]
pub struct _GTypeModule {
	pub parent_instance: _GObject,
	pub use_count: libc::c_uint,
	pub type_infos: *mut _GSList,
	pub interface_infos: *mut _GSList,
	pub name: *mut libc::c_char,
}

/*
struct _GPatternSpec
*/
#[repr(C)]
pub struct _GPatternSpec;

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
	pub io_read: Option<extern fn(*mut _GIOChannel, *mut libc::c_char, libc::c_ulong, *mut libc::c_ulong, *mut *mut _GError) -> libc::c_uint>,
	pub io_write: Option<extern fn(*mut _GIOChannel, *const libc::c_char, libc::c_ulong, *mut libc::c_ulong, *mut *mut _GError) -> libc::c_uint>,
	pub io_seek: Option<extern fn(*mut _GIOChannel, libc::c_long, libc::c_uint, *mut *mut _GError) -> libc::c_uint>,
	pub io_close: Option<extern fn(*mut _GIOChannel, *mut *mut _GError) -> libc::c_uint>,
	pub io_create_watch: Option<extern fn(*mut _GIOChannel, libc::c_uint) -> *mut _GSource>,
	pub io_free: Option<extern fn(*mut _GIOChannel)>,
	pub io_set_flags: Option<extern fn(*mut _GIOChannel, libc::c_uint, *mut *mut _GError) -> libc::c_uint>,
	pub io_get_flags: Option<extern fn(*mut _GIOChannel) -> libc::c_uint>,
}

/*
struct GDebugKey
		(const gchar *) key [const char *]
		(guint) value [unsigned int]
*/
#[repr(C)]
pub struct GDebugKey {
	pub key: *const libc::c_char,
	pub value: libc::c_uint,
}

/*
struct GRand
*/
#[repr(C)]
pub struct GRand;

/*
struct _GVariantDict
		(gsize [16]) x [unsigned long [16]]
*/
#[repr(C)]
pub struct _GVariantDict {
	pub x: [libc::c_ulong; 16],
}

/*
struct _GSignalInvocationHint
		(guint) signal_id [unsigned int]
		(GQuark) detail [unsigned int]
		(GSignalFlags) run_type [GSignalFlags]
*/
#[repr(C)]
pub struct _GSignalInvocationHint {
	pub signal_id: libc::c_uint,
	pub detail: libc::c_uint,
	pub run_type: libc::c_uint,
}

/*
struct _GParamSpecUInt64
		(GParamSpec) parent_instance [struct _GParamSpec]
		(guint64) minimum [unsigned long]
		(guint64) maximum [unsigned long]
		(guint64) default_value [unsigned long]
*/
#[repr(C)]
pub struct _GParamSpecUInt64 {
	pub parent_instance: _GParamSpec,
	pub minimum: libc::c_ulong,
	pub maximum: libc::c_ulong,
	pub default_value: libc::c_ulong,
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
	pub mutex: GStaticMutex,
	pub read_cond: *mut _GCond,
	pub write_cond: *mut _GCond,
	pub read_counter: libc::c_uint,
	pub have_writer: libc::c_int,
	pub want_to_read: libc::c_uint,
	pub want_to_write: libc::c_uint,
}

/*
struct _GParamSpecOverride
		(GParamSpec) parent_instance [struct _GParamSpec]
		(GParamSpec *) overridden [struct _GParamSpec *]
*/
#[repr(C)]
pub struct _GParamSpecOverride {
	pub parent_instance: _GParamSpec,
	pub overridden: *mut _GParamSpec,
}

/*
struct GStaticRecMutex
		(GStaticMutex) mutex [GStaticMutex]
		(guint) depth [unsigned int]
		(union _GStaticRecMutex::(anonymous at /usr/include/glib-2.0/glib/deprecated/gthread.h:164:3)) 
		(union (anonymous union at /usr/include/glib-2.0/glib/deprecated/gthread.h:164:3)) unused [union _GStaticRecMutex::(anonymous at /usr/include/glib-2.0/glib/deprecated/gthread.h:164:3)]
*/
#[repr(C)]
pub struct GStaticRecMutex;/* {
	pub mutex: GStaticMutex,
	pub depth: libc::c_uint,
	pub _: _GStaticRecMutex::(anonymous at /usr/include/glib-2.0/glib/deprecated/gthread.h:164:3),
	pub unused: _GStaticRecMutex::(anonymous at /usr/include/glib-2.0/glib/deprecated/gthread.h:164:3),
}*/

/*
struct GSequenceIter
*/
#[repr(C)]
pub struct GSequenceIter;

/*
struct _GParamSpecGType
		(GParamSpec) parent_instance [struct _GParamSpec]
		(GType) is_a_type [unsigned long]
*/
#[repr(C)]
pub struct _GParamSpecGType {
	pub parent_instance: _GParamSpec,
	pub is_a_type: libc::c_ulong,
}

/*
struct _GDir
*/
#[repr(C)]
pub struct _GDir;

/*
struct GObject
		(GTypeInstance) g_type_instance [struct _GTypeInstance]
		(volatile guint) ref_count [volatile unsigned int]
		(GData *) qdata [struct _GData *]
*/
#[repr(C)]
pub struct GObject {
	pub g_type_instance: _GTypeInstance,
	pub ref_count: libc::c_uint,
	pub qdata: *mut _GData,
}

/*
struct GPollFD
		(gint) fd [int]
		(gushort) events [unsigned short]
		(gushort) revents [unsigned short]
*/
#[repr(C)]
pub struct GPollFD {
	pub fd: libc::c_int,
	pub events: libc::c_ushort,
	pub revents: libc::c_ushort,
}

/*
struct GVariant
*/
#[repr(C)]
pub struct GVariant;

/*
struct GParamSpecPool
*/
#[repr(C)]
pub struct GParamSpecPool;

/*
struct GMutex
		(gpointer) p [void *]
		(guint [2]) i [unsigned int [2]]
*/
#[repr(C)]
pub struct GMutex {
	pub p: *mut libc::c_void,
	pub i: [libc::c_uint; 2],
}

/*
struct _GFlagsValue
		(guint) value [unsigned int]
		(const gchar *) value_name [const char *]
		(const gchar *) value_nick [const char *]
*/
#[repr(C)]
pub struct _GFlagsValue {
	pub value: libc::c_uint,
	pub value_name: *const libc::c_char,
	pub value_nick: *const libc::c_char,
}

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
	pub dummy1: *mut libc::c_void,
	pub dummy2: *mut libc::c_void,
	pub dummy3: *mut libc::c_void,
	pub dummy4: libc::c_int,
	pub dummy5: libc::c_int,
	pub dummy6: *mut libc::c_void,
}

/*
struct _GTypeInfo
		(guint16) class_size [unsigned short]
		(GBaseInitFunc) base_init [void (*)(void *)]
		(GBaseFinalizeFunc) base_finalize [void (*)(void *)]
		(GClassInitFunc) class_init [void (*)(void *, void *)]
		(GClassFinalizeFunc) class_finalize [void (*)(void *, void *)]
		(gconstpointer) class_data [const void *]
		(guint16) instance_size [unsigned short]
		(guint16) n_preallocs [unsigned short]
		(GInstanceInitFunc) instance_init [void (*)(struct _GTypeInstance *, void *)]
		(const GTypeValueTable *) value_table [const struct _GTypeValueTable *]
*/
#[repr(C)]
pub struct _GTypeInfo {
	pub class_size: libc::c_ushort,
	pub base_init: Option<extern fn(*mut libc::c_void)>,
	pub base_finalize: Option<extern fn(*mut libc::c_void)>,
	pub class_init: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>,
	pub class_finalize: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>,
	pub class_data: *const libc::c_void,
	pub instance_size: libc::c_ushort,
	pub n_preallocs: libc::c_ushort,
	pub instance_init: Option<extern fn(*mut _GTypeInstance, *mut libc::c_void)>,
	pub value_table: *const _GTypeValueTable,
}

/*
struct _GTypeClass
		(GType) g_type [unsigned long]
*/
#[repr(C)]
pub struct _GTypeClass {
	pub g_type: libc::c_ulong,
}

/*
struct GPatternSpec
*/
#[repr(C)]
pub struct GPatternSpec;

/*
struct _GOptionGroup
*/
#[repr(C)]
pub struct _GOptionGroup;

/*
struct GSignalInvocationHint
		(guint) signal_id [unsigned int]
		(GQuark) detail [unsigned int]
		(GSignalFlags) run_type [GSignalFlags]
*/
#[repr(C)]
pub struct GSignalInvocationHint {
	pub signal_id: libc::c_uint,
	pub detail: libc::c_uint,
	pub run_type: libc::c_uint,
}

/*
struct _GTypeInterface
		(GType) g_type [unsigned long]
		(GType) g_instance_type [unsigned long]
*/
#[repr(C)]
pub struct _GTypeInterface {
	pub g_type: libc::c_ulong,
	pub g_instance_type: libc::c_ulong,
}

/*
struct _GParamSpecFlags
		(GParamSpec) parent_instance [struct _GParamSpec]
		(GFlagsClass *) flags_class [struct _GFlagsClass *]
		(guint) default_value [unsigned int]
*/
#[repr(C)]
pub struct _GParamSpecFlags {
	pub parent_instance: _GParamSpec,
	pub flags_class: *mut _GFlagsClass,
	pub default_value: libc::c_uint,
}

/*
struct GStaticPrivate
		(guint) index [unsigned int]
*/
#[repr(C)]
pub struct GStaticPrivate {
	pub index: libc::c_uint,
}

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
pub struct GScannerConfig;/* {
	pub cset_skip_characters: *mut libc::c_char,
	pub cset_identifier_first: *mut libc::c_char,
	pub cset_identifier_nth: *mut libc::c_char,
	pub cpair_comment_single: *mut libc::c_char,
	pub case_sensitive: libc::c_uint,
	pub skip_comment_multi: libc::c_uint,
	pub skip_comment_single: libc::c_uint,
	pub scan_comment_multi: libc::c_uint,
	pub scan_identifier: libc::c_uint,
	pub scan_identifier_1char: libc::c_uint,
	pub scan_identifier_NULL: libc::c_uint,
	pub scan_symbols: libc::c_uint,
	pub scan_binary: libc::c_uint,
	pub scan_octal: libc::c_uint,
	pub scan_float: libc::c_uint,
	pub scan_hex: libc::c_uint,
	pub scan_hex_dollar: libc::c_uint,
	pub scan_string_sq: libc::c_uint,
	pub scan_string_dq: libc::c_uint,
	pub numbers_2_int: libc::c_uint,
	pub int_2_float: libc::c_uint,
	pub identifier_2_string: libc::c_uint,
	pub char_2_token: libc::c_uint,
	pub symbol_2_token: libc::c_uint,
	pub scope_0_fallback: libc::c_uint,
	pub store_int64: libc::c_uint,
	pub padding_dummy: libc::c_uint,
}*/

/*
struct _GPtrArray
		(gpointer *) pdata [void **]
		(guint) len [unsigned int]
*/
#[repr(C)]
pub struct _GPtrArray {
	pub pdata: *mut *mut libc::c_void,
	pub len: libc::c_uint,
}

/*
struct GParamSpecTypeInfo
		(guint16) instance_size [unsigned short]
		(guint16) n_preallocs [unsigned short]
		(void (*)(GParamSpec *)) instance_init [void (*)(struct _GParamSpec *)]
		(GType) value_type [unsigned long]
		(void (*)(GParamSpec *)) finalize [void (*)(struct _GParamSpec *)]
		(void (*)(GParamSpec *, GValue *)) value_set_default [void (*)(struct _GParamSpec *, struct _GValue *)]
		(gboolean (*)(GParamSpec *, GValue *)) value_validate [int (*)(struct _GParamSpec *, struct _GValue *)]
		(gint (*)(GParamSpec *, const GValue *, const GValue *)) values_cmp [int (*)(struct _GParamSpec *, const struct _GValue *, const struct _GValue *)]
*/
#[repr(C)]
pub struct GParamSpecTypeInfo {
	pub instance_size: libc::c_ushort,
	pub n_preallocs: libc::c_ushort,
	pub instance_init: Option<extern fn(*mut _GParamSpec)>,
	pub value_type: libc::c_ulong,
	pub finalize: Option<extern fn(*mut _GParamSpec)>,
	pub value_set_default: Option<extern fn(*mut _GParamSpec, *mut _GValue)>,
	pub value_validate: Option<extern fn(*mut _GParamSpec, *mut _GValue) -> libc::c_int>,
	pub values_cmp: Option<extern fn(*mut _GParamSpec, *const _GValue, *const _GValue) -> libc::c_int>,
}

/*
struct _GValue
		(GType) g_type [unsigned long]
		(union _GValue::(anonymous at /usr/include/glib-2.0/gobject/gvalue.h:112:3)) 
		(union (anonymous union at /usr/include/glib-2.0/gobject/gvalue.h:112:3) [2]) data [union _GValue::(anonymous at /usr/include/glib-2.0/gobject/gvalue.h:112:3) [2]]
*/
#[repr(C)]
pub struct _GValue;/* {
	pub g_type: libc::c_ulong,
	pub _: _GValue::(anonymous at /usr/include/glib-2.0/gobject/gvalue.h:112:3),
	pub data: [_GValue::(anonymous at /usr/include/glib-2.0/gobject/gvalue.h:112:3); 2],
}*/

/*
struct GOptionContext
*/
#[repr(C)]
pub struct GOptionContext;

/*
struct _GKeyFile
*/
#[repr(C)]
pub struct _GKeyFile;

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
pub struct _GScannerConfig;/* {
	pub cset_skip_characters: *mut libc::c_char,
	pub cset_identifier_first: *mut libc::c_char,
	pub cset_identifier_nth: *mut libc::c_char,
	pub cpair_comment_single: *mut libc::c_char,
	pub case_sensitive: libc::c_uint,
	pub skip_comment_multi: libc::c_uint,
	pub skip_comment_single: libc::c_uint,
	pub scan_comment_multi: libc::c_uint,
	pub scan_identifier: libc::c_uint,
	pub scan_identifier_1char: libc::c_uint,
	pub scan_identifier_NULL: libc::c_uint,
	pub scan_symbols: libc::c_uint,
	pub scan_binary: libc::c_uint,
	pub scan_octal: libc::c_uint,
	pub scan_float: libc::c_uint,
	pub scan_hex: libc::c_uint,
	pub scan_hex_dollar: libc::c_uint,
	pub scan_string_sq: libc::c_uint,
	pub scan_string_dq: libc::c_uint,
	pub numbers_2_int: libc::c_uint,
	pub int_2_float: libc::c_uint,
	pub identifier_2_string: libc::c_uint,
	pub char_2_token: libc::c_uint,
	pub symbol_2_token: libc::c_uint,
	pub scope_0_fallback: libc::c_uint,
	pub store_int64: libc::c_uint,
	pub padding_dummy: libc::c_uint,
}*/

/*
struct _GParamSpecString
		(GParamSpec) parent_instance [struct _GParamSpec]
		(gchar *) default_value [char *]
		(gchar *) cset_first [char *]
		(gchar *) cset_nth [char *]
		(gchar) substitutor [char]
		(guint) null_fold_if_empty [unsigned int]
		(guint) ensure_non_null [unsigned int]
*/
#[repr(C)]
pub struct _GParamSpecString {
	pub parent_instance: _GParamSpec,
	pub default_value: *mut libc::c_char,
	pub cset_first: *mut libc::c_char,
	pub cset_nth: *mut libc::c_char,
	pub substitutor: libc::c_char,
	pub null_fold_if_empty: libc::c_uint,
	pub ensure_non_null: libc::c_uint,
}

/*
struct _GString
		(gchar *) str [char *]
		(gsize) len [unsigned long]
		(gsize) allocated_len [unsigned long]
*/
#[repr(C)]
pub struct _GString {
	pub str: *mut libc::c_char,
	pub len: libc::c_ulong,
	pub allocated_len: libc::c_ulong,
}

/*
struct _GParamSpecTypeInfo
		(guint16) instance_size [unsigned short]
		(guint16) n_preallocs [unsigned short]
		(void (*)(GParamSpec *)) instance_init [void (*)(struct _GParamSpec *)]
		(GType) value_type [unsigned long]
		(void (*)(GParamSpec *)) finalize [void (*)(struct _GParamSpec *)]
		(void (*)(GParamSpec *, GValue *)) value_set_default [void (*)(struct _GParamSpec *, struct _GValue *)]
		(gboolean (*)(GParamSpec *, GValue *)) value_validate [int (*)(struct _GParamSpec *, struct _GValue *)]
		(gint (*)(GParamSpec *, const GValue *, const GValue *)) values_cmp [int (*)(struct _GParamSpec *, const struct _GValue *, const struct _GValue *)]
*/
#[repr(C)]
pub struct _GParamSpecTypeInfo {
	pub instance_size: libc::c_ushort,
	pub n_preallocs: libc::c_ushort,
	pub instance_init: Option<extern fn(*mut _GParamSpec)>,
	pub value_type: libc::c_ulong,
	pub finalize: Option<extern fn(*mut _GParamSpec)>,
	pub value_set_default: Option<extern fn(*mut _GParamSpec, *mut _GValue)>,
	pub value_validate: Option<extern fn(*mut _GParamSpec, *mut _GValue) -> libc::c_int>,
	pub values_cmp: Option<extern fn(*mut _GParamSpec, *const _GValue, *const _GValue) -> libc::c_int>,
}

/*
struct _GList
		(gpointer) data [void *]
		(GList *) next [struct _GList *]
		(GList *) prev [struct _GList *]
*/
#[repr(C)]
pub struct _GList {
	pub data: *mut libc::c_void,
	pub next: *mut _GList,
	pub prev: *mut _GList,
}

/*
struct _GHashTable
*/
#[repr(C)]
pub struct _GHashTable;

/*
struct _GSequenceNode
*/
#[repr(C)]
pub struct _GSequenceNode;

/*
struct GParamSpec
		(GTypeInstance) g_type_instance [struct _GTypeInstance]
		(const gchar *) name [const char *]
		(GParamFlags) flags [GParamFlags]
		(GType) value_type [unsigned long]
		(GType) owner_type [unsigned long]
		(gchar *) _nick [char *]
		(gchar *) _blurb [char *]
		(GData *) qdata [struct _GData *]
		(guint) ref_count [unsigned int]
		(guint) param_id [unsigned int]
*/
#[repr(C)]
pub struct GParamSpec {
	pub g_type_instance: _GTypeInstance,
	pub name: *const libc::c_char,
	pub flags: libc::c_uint,
	pub value_type: libc::c_ulong,
	pub owner_type: libc::c_ulong,
	pub _nick: *mut libc::c_char,
	pub _blurb: *mut libc::c_char,
	pub qdata: *mut _GData,
	pub ref_count: libc::c_uint,
	pub param_id: libc::c_uint,
}

/*
struct GList
		(gpointer) data [void *]
		(GList *) next [struct _GList *]
		(GList *) prev [struct _GList *]
*/
#[repr(C)]
pub struct GList {
	pub data: *mut libc::c_void,
	pub next: *mut _GList,
	pub prev: *mut _GList,
}

/*
struct _GTypePlugin
*/
#[repr(C)]
pub struct _GTypePlugin;

/*
struct _GEnumClass
		(GTypeClass) g_type_class [struct _GTypeClass]
		(gint) minimum [int]
		(gint) maximum [int]
		(guint) n_values [unsigned int]
		(GEnumValue *) values [struct _GEnumValue *]
*/
#[repr(C)]
pub struct _GEnumClass {
	pub g_type_class: _GTypeClass,
	pub minimum: libc::c_int,
	pub maximum: libc::c_int,
	pub n_values: libc::c_uint,
	pub values: *mut _GEnumValue,
}

/*
struct _GTimeVal
		(glong) tv_sec [long]
		(glong) tv_usec [long]
*/
#[repr(C)]
pub struct _GTimeVal {
	pub tv_sec: libc::c_long,
	pub tv_usec: libc::c_long,
}

/*
struct GBinding
*/
#[repr(C)]
pub struct GBinding;

/*
struct GTypeQuery
		(GType) type [unsigned long]
		(const gchar *) type_name [const char *]
		(guint) class_size [unsigned int]
		(guint) instance_size [unsigned int]
*/
#[repr(C)]
pub struct GTypeQuery {
	pub type_: libc::c_ulong,
	pub type_name: *const libc::c_char,
	pub class_size: libc::c_uint,
	pub instance_size: libc::c_uint,
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
	pub prepare: Option<extern fn(*mut _GSource, *mut libc::c_int) -> libc::c_int>,
	pub check: Option<extern fn(*mut _GSource) -> libc::c_int>,
	pub dispatch: Option<extern fn(*mut _GSource, Option<extern fn(*mut libc::c_void) -> libc::c_int>, *mut libc::c_void) -> libc::c_int>,
	pub finalize: Option<extern fn(*mut _GSource)>,
	pub closure_callback: Option<extern fn(*mut libc::c_void) -> libc::c_int>,
	pub closure_marshal: Option<extern fn()>,
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
	pub data: *mut libc::c_void,
	pub next: *mut _GNode,
	pub prev: *mut _GNode,
	pub parent: *mut _GNode,
	pub children: *mut _GNode,
}

/*
struct _GDebugKey
		(const gchar *) key [const char *]
		(guint) value [unsigned int]
*/
#[repr(C)]
pub struct _GDebugKey {
	pub key: *const libc::c_char,
	pub value: libc::c_uint,
}

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
	pub start_element: Option<extern fn(*mut _GMarkupParseContext, *const libc::c_char, *mut *const libc::c_char, *mut *const libc::c_char, *mut libc::c_void, *mut *mut _GError)>,
	pub end_element: Option<extern fn(*mut _GMarkupParseContext, *const libc::c_char, *mut libc::c_void, *mut *mut _GError)>,
	pub text: Option<extern fn(*mut _GMarkupParseContext, *const libc::c_char, libc::c_ulong, *mut libc::c_void, *mut *mut _GError)>,
	pub passthrough: Option<extern fn(*mut _GMarkupParseContext, *const libc::c_char, libc::c_ulong, *mut libc::c_void, *mut *mut _GError)>,
	pub error: Option<extern fn(*mut _GMarkupParseContext, *mut _GError, *mut libc::c_void)>,
}

/*
struct GEnumValue
		(gint) value [int]
		(const gchar *) value_name [const char *]
		(const gchar *) value_nick [const char *]
*/
#[repr(C)]
pub struct GEnumValue {
	pub value: libc::c_int,
	pub value_name: *const libc::c_char,
	pub value_nick: *const libc::c_char,
}

/*
struct GClosure
		(volatile guint) ref_count [volatile unsigned int]
		(volatile guint) meta_marshal_nouse [volatile unsigned int]
		(volatile guint) n_guards [volatile unsigned int]
		(volatile guint) n_fnotifiers [volatile unsigned int]
		(volatile guint) n_inotifiers [volatile unsigned int]
		(volatile guint) in_inotify [volatile unsigned int]
		(volatile guint) floating [volatile unsigned int]
		(volatile guint) derivative_flag [volatile unsigned int]
		(volatile guint) in_marshal [volatile unsigned int]
		(volatile guint) is_invalid [volatile unsigned int]
		(void (*)(GClosure *, GValue *, guint, const GValue *, gpointer, gpointer)) marshal [void (*)(struct _GClosure *, struct _GValue *, unsigned int, const struct _GValue *, void *, void *)]
		(gpointer) data [void *]
		(GClosureNotifyData *) notifiers [struct _GClosureNotifyData *]
*/
#[repr(C)]
pub struct GClosure {
	pub ref_count: libc::c_uint,
	pub meta_marshal_nouse: libc::c_uint,
	pub n_guards: libc::c_uint,
	pub n_fnotifiers: libc::c_uint,
	pub n_inotifiers: libc::c_uint,
	pub in_inotify: libc::c_uint,
	pub floating: libc::c_uint,
	pub derivative_flag: libc::c_uint,
	pub in_marshal: libc::c_uint,
	pub is_invalid: libc::c_uint,
	pub marshal: Option<extern fn(*mut _GClosure, *mut _GValue, libc::c_uint, *const _GValue, *mut libc::c_void, *mut libc::c_void)>,
	pub data: *mut libc::c_void,
	pub notifiers: *mut _GClosureNotifyData,
}

/*
struct _GTypePluginClass
		(GTypeInterface) base_iface [struct _GTypeInterface]
		(GTypePluginUse) use_plugin [void (*)(struct _GTypePlugin *)]
		(GTypePluginUnuse) unuse_plugin [void (*)(struct _GTypePlugin *)]
		(GTypePluginCompleteTypeInfo) complete_type_info [void (*)(struct _GTypePlugin *, unsigned long, struct _GTypeInfo *, struct _GTypeValueTable *)]
		(GTypePluginCompleteInterfaceInfo) complete_interface_info [void (*)(struct _GTypePlugin *, unsigned long, unsigned long, struct _GInterfaceInfo *)]
*/
#[repr(C)]
pub struct _GTypePluginClass {
	pub base_iface: _GTypeInterface,
	pub use_plugin: Option<extern fn(*mut _GTypePlugin)>,
	pub unuse_plugin: Option<extern fn(*mut _GTypePlugin)>,
	pub complete_type_info: Option<extern fn(*mut _GTypePlugin, libc::c_ulong, *mut _GTypeInfo, *mut _GTypeValueTable)>,
	pub complete_interface_info: Option<extern fn(*mut _GTypePlugin, libc::c_ulong, libc::c_ulong, *mut _GInterfaceInfo)>,
}

/*
struct _GSList
		(gpointer) data [void *]
		(GSList *) next [struct _GSList *]
*/
#[repr(C)]
pub struct _GSList {
	pub data: *mut libc::c_void,
	pub next: *mut _GSList,
}

/*
struct GTypeFundamentalInfo
		(GTypeFundamentalFlags) type_flags [GTypeFundamentalFlags]
*/
#[repr(C)]
pub struct GTypeFundamentalInfo {
	pub type_flags: libc::c_uint,
}

/*
struct _GParamSpecUChar
		(GParamSpec) parent_instance [struct _GParamSpec]
		(guint8) minimum [unsigned char]
		(guint8) maximum [unsigned char]
		(guint8) default_value [unsigned char]
*/
#[repr(C)]
pub struct _GParamSpecUChar {
	pub parent_instance: _GParamSpec,
	pub minimum: libc::c_uchar,
	pub maximum: libc::c_uchar,
	pub default_value: libc::c_uchar,
}

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
	pub start_element: Option<extern fn(*mut _GMarkupParseContext, *const libc::c_char, *mut *const libc::c_char, *mut *const libc::c_char, *mut libc::c_void, *mut *mut _GError)>,
	pub end_element: Option<extern fn(*mut _GMarkupParseContext, *const libc::c_char, *mut libc::c_void, *mut *mut _GError)>,
	pub text: Option<extern fn(*mut _GMarkupParseContext, *const libc::c_char, libc::c_ulong, *mut libc::c_void, *mut *mut _GError)>,
	pub passthrough: Option<extern fn(*mut _GMarkupParseContext, *const libc::c_char, libc::c_ulong, *mut libc::c_void, *mut *mut _GError)>,
	pub error: Option<extern fn(*mut _GMarkupParseContext, *mut _GError, *mut libc::c_void)>,
}

/*
struct _GParamSpecUnichar
		(GParamSpec) parent_instance [struct _GParamSpec]
		(gunichar) default_value [unsigned int]
*/
#[repr(C)]
pub struct _GParamSpecUnichar {
	pub parent_instance: _GParamSpec,
	pub default_value: libc::c_uint,
}

/*
struct _GRWLock
		(gpointer) p [void *]
		(guint [2]) i [unsigned int [2]]
*/
#[repr(C)]
pub struct _GRWLock {
	pub p: *mut libc::c_void,
	pub i: [libc::c_uint; 2],
}

/*
struct _GAsyncQueue
*/
#[repr(C)]
pub struct _GAsyncQueue;

/*
struct _GCClosure
		(GClosure) closure [struct _GClosure]
		(gpointer) callback [void *]
*/
#[repr(C)]
pub struct _GCClosure {
	pub closure: _GClosure,
	pub callback: *mut libc::c_void,
}

/*
struct GMarkupParseContext
*/
#[repr(C)]
pub struct GMarkupParseContext;

/*
struct _GRand
*/
#[repr(C)]
pub struct _GRand;

/*
struct GTestLogBuffer
		(GString *) data [struct _GString *]
		(GSList *) msgs [struct _GSList *]
*/
#[repr(C)]
pub struct GTestLogBuffer {
	pub data: *mut _GString,
	pub msgs: *mut _GSList,
}

/*
struct _GParamSpecEnum
		(GParamSpec) parent_instance [struct _GParamSpec]
		(GEnumClass *) enum_class [struct _GEnumClass *]
		(gint) default_value [int]
*/
#[repr(C)]
pub struct _GParamSpecEnum {
	pub parent_instance: _GParamSpec,
	pub enum_class: *mut _GEnumClass,
	pub default_value: libc::c_int,
}

/*
struct _GParamSpecInt64
		(GParamSpec) parent_instance [struct _GParamSpec]
		(gint64) minimum [long]
		(gint64) maximum [long]
		(gint64) default_value [long]
*/
#[repr(C)]
pub struct _GParamSpecInt64 {
	pub parent_instance: _GParamSpec,
	pub minimum: libc::c_long,
	pub maximum: libc::c_long,
	pub default_value: libc::c_long,
}

/*
struct GRWLock
		(gpointer) p [void *]
		(guint [2]) i [unsigned int [2]]
*/
#[repr(C)]
pub struct GRWLock {
	pub p: *mut libc::c_void,
	pub i: [libc::c_uint; 2],
}

/*
struct GMemChunk
*/
#[repr(C)]
pub struct GMemChunk;

/*
struct GStringChunk
*/
#[repr(C)]
pub struct GStringChunk;

/*
struct GStaticMutex
		(GMutex *) mutex [union _GMutex *]
		(pthread_mutex_t) unused [pthread_mutex_t]
*/
#[repr(C)]
pub struct GStaticMutex;
/*
struct _GPollFD
		(gint) fd [int]
		(gushort) events [unsigned short]
		(gushort) revents [unsigned short]
*/
#[repr(C)]
pub struct _GPollFD {
	pub fd: libc::c_int,
	pub events: libc::c_ushort,
	pub revents: libc::c_ushort,
}

/*
struct _GIConv
*/
#[repr(C)]
pub struct _GIConv;

/*
struct GHmac
*/
#[repr(C)]
pub struct GHmac;

/*
struct _GTree
*/
#[repr(C)]
pub struct _GTree;

/*
struct GDir
*/
#[repr(C)]
pub struct GDir;

/*
struct _GCache
*/
#[repr(C)]
pub struct _GCache;

/*
struct _GParameter
		(const gchar *) name [const char *]
		(GValue) value [struct _GValue]
*/
#[repr(C)]
pub struct _GParameter {
	pub name: *const libc::c_char,
	pub value: _GValue,
}

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
	pub malloc: Option<extern fn(libc::c_ulong) -> *mut libc::c_void>,
	pub realloc: Option<extern fn(*mut libc::c_void, libc::c_ulong) -> *mut libc::c_void>,
	pub free: Option<extern fn(*mut libc::c_void)>,
	pub calloc: Option<extern fn(libc::c_ulong, libc::c_ulong) -> *mut libc::c_void>,
	pub try_malloc: Option<extern fn(libc::c_ulong) -> *mut libc::c_void>,
	pub try_realloc: Option<extern fn(*mut libc::c_void, libc::c_ulong) -> *mut libc::c_void>,
}

/*
struct GAsyncQueue
*/
#[repr(C)]
pub struct GAsyncQueue;

/*
struct _GError
		(GQuark) domain [unsigned int]
		(gint) code [int]
		(gchar *) message [char *]
*/
#[repr(C)]
pub struct _GError {
	pub domain: libc::c_uint,
	pub code: libc::c_int,
	pub message: *mut libc::c_char,
}

/*
struct GTypeInstance
		(GTypeClass *) g_class [struct _GTypeClass *]
*/
#[repr(C)]
pub struct GTypeInstance {
	pub g_class: *mut _GTypeClass,
}

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
	pub data: *mut libc::c_void,
	pub next: *mut _GHook,
	pub prev: *mut _GHook,
	pub ref_count: libc::c_uint,
	pub hook_id: libc::c_ulong,
	pub flags: libc::c_uint,
	pub func: *mut libc::c_void,
	pub destroy: Option<extern fn(*mut libc::c_void)>,
}

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
	pub long_name: *const libc::c_char,
	pub short_name: libc::c_char,
	pub flags: libc::c_int,
	pub arg: libc::c_uint,
	pub arg_data: *mut libc::c_void,
	pub description: *const libc::c_char,
	pub arg_description: *const libc::c_char,
}

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
	pub prepare: Option<extern fn(*mut _GSource, *mut libc::c_int) -> libc::c_int>,
	pub check: Option<extern fn(*mut _GSource) -> libc::c_int>,
	pub dispatch: Option<extern fn(*mut _GSource, Option<extern fn(*mut libc::c_void) -> libc::c_int>, *mut libc::c_void) -> libc::c_int>,
	pub finalize: Option<extern fn(*mut _GSource)>,
	pub closure_callback: Option<extern fn(*mut libc::c_void) -> libc::c_int>,
	pub closure_marshal: Option<extern fn()>,
}

/*
struct _GTypeValueTable
		(void (*)(GValue *)) value_init [void (*)(struct _GValue *)]
		(void (*)(GValue *)) value_free [void (*)(struct _GValue *)]
		(void (*)(const GValue *, GValue *)) value_copy [void (*)(const struct _GValue *, struct _GValue *)]
		(gpointer (*)(const GValue *)) value_peek_pointer [void *(*)(const struct _GValue *)]
		(const gchar *) collect_format [const char *]
		(gchar *(*)(GValue *, guint, GTypeCValue *, guint)) collect_value [char *(*)(struct _GValue *, unsigned int, union _GTypeCValue *, unsigned int)]
		(const gchar *) lcopy_format [const char *]
		(gchar *(*)(const GValue *, guint, GTypeCValue *, guint)) lcopy_value [char *(*)(const struct _GValue *, unsigned int, union _GTypeCValue *, unsigned int)]
*/
#[repr(C)]
pub struct _GTypeValueTable {
	pub value_init: Option<extern fn(*mut _GValue)>,
	pub value_free: Option<extern fn(*mut _GValue)>,
	pub value_copy: Option<extern fn(*const _GValue, *mut _GValue)>,
	pub value_peek_pointer: Option<extern fn(*const _GValue) -> *mut libc::c_void>,
	pub collect_format: *const libc::c_char,
	pub collect_value: Option<extern fn(*mut _GValue, libc::c_uint, *mut _GTypeCValue, libc::c_uint) -> *mut libc::c_char>,
	pub lcopy_format: *const libc::c_char,
	pub lcopy_value: Option<extern fn(*const _GValue, libc::c_uint, *mut _GTypeCValue, libc::c_uint) -> *mut libc::c_char>,
}

/*
struct _GSourceCallbackFuncs
		(void (*)(gpointer)) ref [void (*)(void *)]
		(void (*)(gpointer)) unref [void (*)(void *)]
		(void (*)(gpointer, GSource *, GSourceFunc *, gpointer *)) get [void (*)(void *, struct _GSource *, int (**)(void *), void **)]
*/
#[repr(C)]
pub struct _GSourceCallbackFuncs {
	pub ref_: Option<extern fn(*mut libc::c_void)>,
	pub unref: Option<extern fn(*mut libc::c_void)>,
	pub get: Option<extern fn(*mut libc::c_void, *mut _GSource, *mut Option<extern fn(*mut libc::c_void) -> libc::c_int>, *mut *mut libc::c_void)>,
}

/*
struct _GParamSpecLong
		(GParamSpec) parent_instance [struct _GParamSpec]
		(glong) minimum [long]
		(glong) maximum [long]
		(glong) default_value [long]
*/
#[repr(C)]
pub struct _GParamSpecLong {
	pub parent_instance: _GParamSpec,
	pub minimum: libc::c_long,
	pub maximum: libc::c_long,
	pub default_value: libc::c_long,
}

/*
struct GSourceCallbackFuncs
		(void (*)(gpointer)) ref [void (*)(void *)]
		(void (*)(gpointer)) unref [void (*)(void *)]
		(void (*)(gpointer, GSource *, GSourceFunc *, gpointer *)) get [void (*)(void *, struct _GSource *, int (**)(void *), void **)]
*/
#[repr(C)]
pub struct GSourceCallbackFuncs {
	pub ref_: Option<extern fn(*mut libc::c_void)>,
	pub unref: Option<extern fn(*mut libc::c_void)>,
	pub get: Option<extern fn(*mut libc::c_void, *mut _GSource, *mut Option<extern fn(*mut libc::c_void) -> libc::c_int>, *mut *mut libc::c_void)>,
}

/*
struct _GOptionContext
*/
#[repr(C)]
pub struct _GOptionContext;

/*
struct _GTimer
*/
#[repr(C)]
pub struct _GTimer;

/*
struct _GParamSpecValueArray
		(GParamSpec) parent_instance [struct _GParamSpec]
		(GParamSpec *) element_spec [struct _GParamSpec *]
		(guint) fixed_n_elements [unsigned int]
*/
#[repr(C)]
pub struct _GParamSpecValueArray {
	pub parent_instance: _GParamSpec,
	pub element_spec: *mut _GParamSpec,
	pub fixed_n_elements: libc::c_uint,
}

/*
struct GTuples
		(guint) len [unsigned int]
*/
#[repr(C)]
pub struct GTuples {
	pub len: libc::c_uint,
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
	pub ref_count: libc::c_int,
	pub funcs: *mut _GIOFuncs,
	pub encoding: *mut libc::c_char,
	pub read_cd: *mut _GIConv,
	pub write_cd: *mut _GIConv,
	pub line_term: *mut libc::c_char,
	pub line_term_len: libc::c_uint,
	pub buf_size: libc::c_ulong,
	pub read_buf: *mut _GString,
	pub encoded_read_buf: *mut _GString,
	pub write_buf: *mut _GString,
	pub partial_write_buf: [libc::c_char; 6],
	pub use_buffer: libc::c_uint,
	pub do_encode: libc::c_uint,
	pub close_on_unref: libc::c_uint,
	pub is_readable: libc::c_uint,
	pub is_writeable: libc::c_uint,
	pub is_seekable: libc::c_uint,
	pub reserved1: *mut libc::c_void,
	pub reserved2: *mut libc::c_void,
}

/*
struct _GParamSpecPool
*/
#[repr(C)]
pub struct _GParamSpecPool;

/*
struct GByteArray
		(guint8 *) data [unsigned char *]
		(guint) len [unsigned int]
*/
#[repr(C)]
pub struct GByteArray {
	pub data: *mut libc::c_uchar,
	pub len: libc::c_uint,
}

/*
struct GTestSuite
*/
#[repr(C)]
pub struct GTestSuite;

/*
guint g_str_hash() [unsigned int]
	(gconstpointer) v [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_str_hash(v: *const libc::c_void) -> libc::c_uint;
}


/*
gchar * g_shell_unquote() [char *]
	(const gchar *) quoted_string [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_shell_unquote(quoted_string: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
void g_type_remove_class_cache_func()
	(gpointer) cache_data [void *]
	(GTypeClassCacheFunc) cache_func [int (*)(void *, struct _GTypeClass *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_remove_class_cache_func(cache_data: *mut libc::c_void, cache_func: Option<extern fn(*mut libc::c_void, *mut _GTypeClass) -> libc::c_int>);
}


/*
gchar * g_build_pathv() [char *]
	(const gchar *) separator [const char *]
	(gchar **) args [char **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_build_pathv(separator: *const libc::c_char, args: *mut *mut libc::c_char) -> *mut libc::c_char;
}


/*
guint g_thread_pool_get_num_threads() [unsigned int]
	(GThreadPool *) pool [struct _GThreadPool *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_pool_get_num_threads(pool: *mut _GThreadPool) -> libc::c_uint;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_trap_assertions(domain: *const libc::c_char, file: *const libc::c_char, line: libc::c_int, func: *const libc::c_char, assertion_flags: libc::c_ulong, pattern: *const libc::c_char);
}


/*
gchar ** g_bookmark_file_get_applications() [char **]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_get_applications(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut *mut libc::c_char;
}


/*
gpointer g_atomic_pointer_get() [void *]
	(const volatile void *) atomic
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_atomic_pointer_get(atomic: *const libc::c_void) -> *mut libc::c_void;
}


/*
gboolean g_variant_equal() [int]
	(gconstpointer) one [const void *]
	(gconstpointer) two [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_equal(one: *const libc::c_void, two: *const libc::c_void) -> libc::c_int;
}


/*
const gchar * g_intern_string() [const char *]
	(const gchar *) string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_intern_string(string: *const libc::c_char) -> *const libc::c_char;
}


/*
void g_type_add_class_private()
	(GType) class_type [unsigned long]
	(gsize) private_size [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_add_class_private(class_type: libc::c_ulong, private_size: libc::c_ulong);
}


/*
guint32 g_date_get_julian() [unsigned int]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_get_julian(date: *const _GDate) -> libc::c_uint;
}


/*
GClosure * g_cclosure_new() [struct _GClosure *]
	(GCallback) callback_func [void (*)(void)]
	(gpointer) user_data [void *]
	(GClosureNotify) destroy_data [void (*)(void *, struct _GClosure *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_new(callback_func: Option<extern fn()>, user_data: *mut libc::c_void, destroy_data: Option<extern fn(*mut libc::c_void, *mut _GClosure)>) -> *mut _GClosure;
}


/*
GSource * g_source_new() [struct _GSource *]
	(GSourceFuncs *) source_funcs [struct _GSourceFuncs *]
	(guint) struct_size [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_new(source_funcs: *mut _GSourceFuncs, struct_size: libc::c_uint) -> *mut _GSource;
}


/*
void g_param_spec_sink()
	(GParamSpec *) pspec [struct _GParamSpec *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_sink(pspec: *mut _GParamSpec);
}


/*
GTimeZone * g_time_zone_new() [struct _GTimeZone *]
	(const gchar *) identifier [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_time_zone_new(identifier: *const libc::c_char) -> *mut _GTimeZone;
}


/*
gboolean g_type_check_instance_is_fundamentally_a() [int]
	(GTypeInstance *) instance [struct _GTypeInstance *]
	(GType) fundamental_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_check_instance_is_fundamentally_a(instance: *mut _GTypeInstance, fundamental_type: libc::c_ulong) -> libc::c_int;
}


/*
guint g_node_n_children() [unsigned int]
	(GNode *) node [struct _GNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_n_children(node: *mut _GNode) -> libc::c_uint;
}


/*
GStringChunk * g_string_chunk_new() [struct _GStringChunk *]
	(gsize) size [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_chunk_new(size: libc::c_ulong) -> *mut _GStringChunk;
}


/*
gboolean g_match_info_next() [int]
	(GMatchInfo *) match_info [struct _GMatchInfo *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_match_info_next(match_info: *mut _GMatchInfo, error: *mut *mut _GError) -> libc::c_int;
}


/*
gulong g_signal_add_emission_hook() [unsigned long]
	(guint) signal_id [unsigned int]
	(GQuark) detail [unsigned int]
	(GSignalEmissionHook) hook_func [int (*)(struct _GSignalInvocationHint *, unsigned int, const struct _GValue *, void *)]
	(gpointer) hook_data [void *]
	(GDestroyNotify) data_destroy [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_add_emission_hook(signal_id: libc::c_uint, detail: libc::c_uint, hook_func: Option<extern fn(*mut _GSignalInvocationHint, libc::c_uint, *const _GValue, *mut libc::c_void) -> libc::c_int>, hook_data: *mut libc::c_void, data_destroy: Option<extern fn(*mut libc::c_void)>) -> libc::c_ulong;
}


/*
const gchar * g_dir_read_name() [const char *]
	(GDir *) dir [struct _GDir *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_dir_read_name(dir: *mut _GDir) -> *const libc::c_char;
}


/*
void g_main_context_wakeup()
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_wakeup(context: *mut _GMainContext);
}


/*
void g_closure_set_meta_marshal()
	(GClosure *) closure [struct _GClosure *]
	(gpointer) marshal_data [void *]
	(GClosureMarshal) meta_marshal [void (*)(struct _GClosure *, struct _GValue *, unsigned int, const struct _GValue *, void *, void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_closure_set_meta_marshal(closure: *mut _GClosure, marshal_data: *mut libc::c_void, meta_marshal: Option<extern fn(*mut _GClosure, *mut _GValue, libc::c_uint, *const _GValue, *mut libc::c_void, *mut libc::c_void)>);
}


/*
guchar g_variant_get_byte() [unsigned char]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_byte(value: *mut _GVariant) -> libc::c_uchar;
}


/*
const gchar * g_get_tmp_dir() [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_tmp_dir() -> *const libc::c_char;
}


/*
GVariant * g_variant_ref_sink() [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_ref_sink(value: *mut _GVariant) -> *mut _GVariant;
}


/*
const gchar * g_match_info_get_string() [const char *]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_match_info_get_string(match_info: *const _GMatchInfo) -> *const libc::c_char;
}


/*
gint g_sequence_iter_compare() [int]
	(GSequenceIter *) a [struct _GSequenceNode *]
	(GSequenceIter *) b [struct _GSequenceNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_iter_compare(a: *mut _GSequenceNode, b: *mut _GSequenceNode) -> libc::c_int;
}


/*
void g_log_remove_handler()
	(const gchar *) log_domain [const char *]
	(guint) handler_id [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_log_remove_handler(log_domain: *const libc::c_char, handler_id: libc::c_uint);
}


/*
gboolean g_time_zone_is_dst() [int]
	(GTimeZone *) tz [struct _GTimeZone *]
	(gint) interval [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_time_zone_is_dst(tz: *mut _GTimeZone, interval: libc::c_int) -> libc::c_int;
}


/*
guint g_idle_add_full() [unsigned int]
	(gint) priority [int]
	(GSourceFunc) function [int (*)(void *)]
	(gpointer) data [void *]
	(GDestroyNotify) notify [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_idle_add_full(priority: libc::c_int, function: Option<extern fn(*mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void, notify: Option<extern fn(*mut libc::c_void)>) -> libc::c_uint;
}


/*
gboolean g_variant_type_is_definite() [int]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_is_definite(type_: *const _GVariantType) -> libc::c_int;
}


/*
gpointer g_queue_pop_nth() [void *]
	(GQueue *) queue [struct _GQueue *]
	(guint) n [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_pop_nth(queue: *mut _GQueue, n: libc::c_uint) -> *mut libc::c_void;
}


/*
void g_timer_destroy()
	(GTimer *) timer [struct _GTimer *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_timer_destroy(timer: *mut _GTimer);
}


/*
GSList * g_slist_prepend() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_prepend(list: *mut _GSList, data: *mut libc::c_void) -> *mut _GSList;
}


/*
gboolean g_shell_parse_argv() [int]
	(const gchar *) command_line [const char *]
	(gint *) argcp [int *]
	(gchar ***) argvp [char ***]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_shell_parse_argv(command_line: *const libc::c_char, argcp: *mut libc::c_int, argvp: *mut *mut *mut libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
gdouble g_rand_double() [double]
	(GRand *) rand_ [struct _GRand *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rand_double(rand_: *mut _GRand) -> libc::c_double;
}


/*
void g_dataset_foreach()
	(gconstpointer) dataset_location [const void *]
	(GDataForeachFunc) func [void (*)(unsigned int, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_dataset_foreach(dataset_location: *const libc::c_void, func: Option<extern fn(libc::c_uint, *mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
void g_value_set_boxed()
	(GValue *) value [struct _GValue *]
	(gconstpointer) v_boxed [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_boxed(value: *mut _GValue, v_boxed: *const libc::c_void);
}


/*
void g_debug()
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_debug(format: *const libc::c_char);
}


/*
gboolean g_variant_type_is_container() [int]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_is_container(type_: *const _GVariantType) -> libc::c_int;
}


/*
void g_signal_emit_valist()
	(gpointer) instance [void *]
	(guint) signal_id [unsigned int]
	(GQuark) detail [unsigned int]
	(va_list) var_args [struct __va_list_tag [1]]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_signal_emit_valist(instance: *mut libc::c_void, signal_id: libc::c_uint, detail: libc::c_uint, var_args: [__va_list_tag; 1]);
}


/*
GValueArray * g_value_array_new() [struct _GValueArray *]
	(guint) n_prealloced [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_array_new(n_prealloced: libc::c_uint) -> *mut _GValueArray;
}


/*
GPtrArray * g_ptr_array_ref() [struct _GPtrArray *]
	(GPtrArray *) array [struct _GPtrArray *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_ref(array: *mut _GPtrArray) -> *mut _GPtrArray;
}


/*
gboolean g_bookmark_file_load_from_data_dirs() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) file [const char *]
	(gchar **) full_path [char **]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_load_from_data_dirs(bookmark: *mut _GBookmarkFile, file: *const libc::c_char, full_path: *mut *mut libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
GTimeZone * g_time_zone_new_local() [struct _GTimeZone *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_time_zone_new_local() -> *mut _GTimeZone;
}


/*
GOptionGroup * g_option_context_get_main_group() [struct _GOptionGroup *]
	(GOptionContext *) context [struct _GOptionContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_context_get_main_group(context: *mut _GOptionContext) -> *mut _GOptionGroup;
}


/*
gchar * g_ascii_strdown() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ascii_strdown(str: *const libc::c_char, len: libc::c_long) -> *mut libc::c_char;
}


/*
gchar ** g_bookmark_file_get_groups() [char **]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_get_groups(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut *mut libc::c_char;
}


/*
GType g_match_info_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_match_info_get_type() -> libc::c_ulong;
}


/*
gchar * g_compute_hmac_for_data() [char *]
	(GChecksumType) digest_type [GChecksumType]
	(const guchar *) key [const unsigned char *]
	(gsize) key_len [unsigned long]
	(const guchar *) data [const unsigned char *]
	(gsize) length [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_compute_hmac_for_data(digest_type: libc::c_uint, key: *const libc::c_uchar, key_len: libc::c_ulong, data: *const libc::c_uchar, length: libc::c_ulong) -> *mut libc::c_char;
}


/*
gboolean g_option_context_parse_strv() [int]
	(GOptionContext *) context [struct _GOptionContext *]
	(gchar ***) arguments [char ***]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_context_parse_strv(context: *mut _GOptionContext, arguments: *mut *mut *mut libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_static_private_set()
	(GStaticPrivate *) private_key [struct _GStaticPrivate *]
	(gpointer) data [void *]
	(GDestroyNotify) notify [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_private_set(private_key: *mut _GStaticPrivate, data: *mut libc::c_void, notify: Option<extern fn(*mut libc::c_void)>);
}


/*
gchar * g_key_file_get_start_group() [char *]
	(GKeyFile *) key_file [struct _GKeyFile *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_get_start_group(key_file: *mut _GKeyFile) -> *mut libc::c_char;
}


/*
guint g_thread_pool_get_max_idle_time() [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_pool_get_max_idle_time() -> libc::c_uint;
}


/*
GString * g_string_erase() [struct _GString *]
	(GString *) string [struct _GString *]
	(gssize) pos [long]
	(gssize) len [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_erase(string: *mut _GString, pos: libc::c_long, len: libc::c_long) -> *mut _GString;
}


/*
void g_type_set_qdata()
	(GType) type [unsigned long]
	(GQuark) quark [unsigned int]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_set_qdata(type_: libc::c_ulong, quark: libc::c_uint, data: *mut libc::c_void);
}


/*
GSequenceIter * g_sequence_lookup_iter() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
	(gpointer) data [void *]
	(GSequenceIterCompareFunc) iter_cmp [int (*)(struct _GSequenceNode *, struct _GSequenceNode *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_lookup_iter(seq: *mut _GSequence, data: *mut libc::c_void, iter_cmp: Option<extern fn(*mut _GSequenceNode, *mut _GSequenceNode, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void) -> *mut _GSequenceNode;
}


/*
GSList * g_slist_find_custom() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(gconstpointer) data [const void *]
	(GCompareFunc) func [int (*)(const void *, const void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_find_custom(list: *mut _GSList, data: *const libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GSList;
}


/*
const gchar * g_get_user_data_dir() [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_user_data_dir() -> *const libc::c_char;
}


/*
const gchar * g_dgettext() [const char *]
	(const gchar *) domain [const char *]
	(const gchar *) msgid [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_dgettext(domain: *const libc::c_char, msgid: *const libc::c_char) -> *const libc::c_char;
}


/*
gint g_ascii_strncasecmp() [int]
	(const gchar *) s1 [const char *]
	(const gchar *) s2 [const char *]
	(gsize) n [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ascii_strncasecmp(s1: *const libc::c_char, s2: *const libc::c_char, n: libc::c_ulong) -> libc::c_int;
}


/*
gint g_iconv_close() [int]
	(GIConv) converter [struct _GIConv *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_iconv_close(converter: *mut _GIConv) -> libc::c_int;
}


/*
void g_main_loop_run()
	(GMainLoop *) loop [struct _GMainLoop *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_loop_run(loop_: *mut _GMainLoop);
}


/*
void g_queue_push_head_link()
	(GQueue *) queue [struct _GQueue *]
	(GList *) link_ [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_push_head_link(queue: *mut _GQueue, link_: *mut _GList);
}


/*
void g_async_queue_ref_unlocked()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_ref_unlocked(queue: *mut _GAsyncQueue);
}


/*
gchar * g_strdown() [char *]
	(gchar *) string [char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strdown(string: *mut libc::c_char) -> *mut libc::c_char;
}


/*
GList * g_queue_find_custom() [struct _GList *]
	(GQueue *) queue [struct _GQueue *]
	(gconstpointer) data [const void *]
	(GCompareFunc) func [int (*)(const void *, const void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_find_custom(queue: *mut _GQueue, data: *const libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GList;
}


/*
gchar * g_mkdtemp_full() [char *]
	(gchar *) tmpl [char *]
	(gint) mode [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mkdtemp_full(tmpl: *mut libc::c_char, mode: libc::c_int) -> *mut libc::c_char;
}


/*
void g_option_group_set_error_hook()
	(GOptionGroup *) group [struct _GOptionGroup *]
	(GOptionErrorFunc) error_func [void (*)(struct _GOptionContext *, struct _GOptionGroup *, void *, struct _GError **)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_group_set_error_hook(group: *mut _GOptionGroup, error_func: Option<extern fn(*mut _GOptionContext, *mut _GOptionGroup, *mut libc::c_void, *mut *mut _GError)>);
}


/*
GType * g_type_children() [unsigned long *]
	(GType) type [unsigned long]
	(guint *) n_children [unsigned int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_children(type_: libc::c_ulong, n_children: *mut libc::c_uint) -> *mut libc::c_ulong;
}


/*
void g_hash_table_iter_init()
	(GHashTableIter *) iter [struct _GHashTableIter *]
	(GHashTable *) hash_table [struct _GHashTable *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_iter_init(iter: *mut _GHashTableIter, hash_table: *mut _GHashTable);
}


/*
const gchar *const * g_get_language_names() [const char *const *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_language_names() -> *const *const libc::c_char;
}


/*
GFlagsValue * g_flags_get_value_by_nick() [struct _GFlagsValue *]
	(GFlagsClass *) flags_class [struct _GFlagsClass *]
	(const gchar *) nick [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_flags_get_value_by_nick(flags_class: *mut _GFlagsClass, nick: *const libc::c_char) -> *mut _GFlagsValue;
}


/*
void g_log_default_handler()
	(const gchar *) log_domain [const char *]
	(GLogLevelFlags) log_level [GLogLevelFlags]
	(const gchar *) message [const char *]
	(gpointer) unused_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_log_default_handler(log_domain: *const libc::c_char, log_level: libc::c_uint, message: *const libc::c_char, unused_data: *mut libc::c_void);
}


/*
GType g_strv_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strv_get_type() -> libc::c_ulong;
}


/*
gint g_time_zone_find_interval() [int]
	(GTimeZone *) tz [struct _GTimeZone *]
	(GTimeType) type [GTimeType]
	(gint64) time_ [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_time_zone_find_interval(tz: *mut _GTimeZone, type_: libc::c_uint, time_: libc::c_long) -> libc::c_int;
}


/*
void g_type_free_instance()
	(GTypeInstance *) instance [struct _GTypeInstance *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_free_instance(instance: *mut _GTypeInstance);
}


/*
void g_scanner_input_file()
	(GScanner *) scanner [struct _GScanner *]
	(gint) input_fd [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_input_file(scanner: *mut _GScanner, input_fd: libc::c_int);
}


/*
gpointer g_hash_table_lookup() [void *]
	(GHashTable *) hash_table [struct _GHashTable *]
	(gconstpointer) key [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_lookup(hash_table: *mut _GHashTable, key: *const libc::c_void) -> *mut libc::c_void;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_assertion_message_cmpnum(domain: *const libc::c_char, file: *const libc::c_char, line: libc::c_int, func: *const libc::c_char, expr: *const libc::c_char, arg1: libc::c_double, cmp: *const libc::c_char, arg2: libc::c_double, numtype: libc::c_char);
}


/*
gboolean g_variant_dict_lookup() [int]
	(GVariantDict *) dict [struct _GVariantDict *]
	(const gchar *) key [const char *]
	(const gchar *) format_string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_dict_lookup(dict: *mut _GVariantDict, key: *const libc::c_char, format_string: *const libc::c_char) -> libc::c_int;
}


/*
gpointer g_dataset_id_remove_no_notify() [void *]
	(gconstpointer) dataset_location [const void *]
	(GQuark) key_id [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_dataset_id_remove_no_notify(dataset_location: *const libc::c_void, key_id: libc::c_uint) -> *mut libc::c_void;
}


/*
void g_source_set_dummy_callback()
	(GSource *) source [struct _GSource *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_set_dummy_callback(source: *mut _GSource);
}


/*
const gchar * g_date_time_get_timezone_abbreviation() [const char *]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_get_timezone_abbreviation(datetime: *mut _GDateTime) -> *const libc::c_char;
}


/*
gchar * g_base64_encode() [char *]
	(const guchar *) data [const unsigned char *]
	(gsize) len [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_base64_encode(data: *const libc::c_uchar, len: libc::c_ulong) -> *mut libc::c_char;
}


/*
GClosure * g_closure_new_object() [struct _GClosure *]
	(guint) sizeof_closure [unsigned int]
	(GObject *) object [struct _GObject *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_closure_new_object(sizeof_closure: libc::c_uint, object: *mut _GObject) -> *mut _GClosure;
}


/*
void g_hook_insert_sorted()
	(GHookList *) hook_list [struct _GHookList *]
	(GHook *) hook [struct _GHook *]
	(GHookCompareFunc) func [int (*)(struct _GHook *, struct _GHook *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_insert_sorted(hook_list: *mut _GHookList, hook: *mut _GHook, func: Option<extern fn(*mut _GHook, *mut _GHook) -> libc::c_int>);
}


/*
gchar * g_string_chunk_insert() [char *]
	(GStringChunk *) chunk [struct _GStringChunk *]
	(const gchar *) string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_chunk_insert(chunk: *mut _GStringChunk, string: *const libc::c_char) -> *mut libc::c_char;
}


/*
void g_node_destroy()
	(GNode *) root [struct _GNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_destroy(root: *mut _GNode);
}


/*
void g_mem_chunk_clean()
	(GMemChunk *) mem_chunk [struct _GMemChunk *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mem_chunk_clean(mem_chunk: *mut _GMemChunk);
}


/*
gchar * g_find_program_in_path() [char *]
	(const gchar *) program [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_find_program_in_path(program: *const libc::c_char) -> *mut libc::c_char;
}


/*
GClosure * g_cclosure_new_object_swap() [struct _GClosure *]
	(GCallback) callback_func [void (*)(void)]
	(GObject *) object [struct _GObject *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_new_object_swap(callback_func: Option<extern fn()>, object: *mut _GObject) -> *mut _GClosure;
}


/*
gchar * g_markup_printf_escaped() [char *]
	(const char *) format
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_markup_printf_escaped(format: *const libc::c_char) -> *mut libc::c_char;
}


/*
gpointer g_static_private_get() [void *]
	(GStaticPrivate *) private_key [struct _GStaticPrivate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_private_get(private_key: *mut _GStaticPrivate) -> *mut libc::c_void;
}


/*
GParamSpec * g_param_spec_ulong() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(gulong) minimum [unsigned long]
	(gulong) maximum [unsigned long]
	(gulong) default_value [unsigned long]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_ulong(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, minimum: libc::c_ulong, maximum: libc::c_ulong, default_value: libc::c_ulong, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
void g_string_vprintf()
	(GString *) string [struct _GString *]
	(const gchar *) format [const char *]
	(va_list) args [struct __va_list_tag [1]]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_string_vprintf(string: *mut _GString, format: *const libc::c_char, args: [__va_list_tag; 1]);
}


/*
gchar ** g_listenv() [char **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_listenv() -> *mut *mut libc::c_char;
}


/*
void _g_log_fallback_handler()
	(const gchar *) log_domain [const char *]
	(GLogLevelFlags) log_level [GLogLevelFlags]
	(const gchar *) message [const char *]
	(gpointer) unused_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn _g_log_fallback_handler(log_domain: *const libc::c_char, log_level: libc::c_uint, message: *const libc::c_char, unused_data: *mut libc::c_void);
}


/*
void g_sequence_set()
	(GSequenceIter *) iter [struct _GSequenceNode *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_set(iter: *mut _GSequenceNode, data: *mut libc::c_void);
}


/*
GType g_initially_unowned_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_initially_unowned_get_type() -> libc::c_ulong;
}


/*
void g_static_rec_mutex_init()
	(GStaticRecMutex *) mutex [struct _GStaticRecMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_rec_mutex_init(mutex: *mut _GStaticRecMutex);
}


/*
guint32 g_random_int() [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_random_int() -> libc::c_uint;
}


/*
const gchar * g_get_application_name() [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_application_name() -> *const libc::c_char;
}


/*
gint g_sequence_get_length() [int]
	(GSequence *) seq [struct _GSequence *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_get_length(seq: *mut _GSequence) -> libc::c_int;
}


/*
void g_date_set_time()
	(GDate *) date [struct _GDate *]
	(GTime) time_ [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_set_time(date: *mut _GDate, time_: libc::c_int);
}


/*
double g_test_timer_elapsed()
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_timer_elapsed() -> libc::c_double;
}


/*
GSList * g_slist_sort() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(GCompareFunc) compare_func [int (*)(const void *, const void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_sort(list: *mut _GSList, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GSList;
}


/*
gchar * g_markup_vprintf_escaped() [char *]
	(const char *) format
	(va_list) args [struct __va_list_tag [1]]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_markup_vprintf_escaped(format: *const libc::c_char, args: [__va_list_tag; 1]) -> *mut libc::c_char;
}


/*
GParamSpec * g_param_spec_gtype() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(GType) is_a_type [unsigned long]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_gtype(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, is_a_type: libc::c_ulong, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
GSList * g_slist_copy() [struct _GSList *]
	(GSList *) list [struct _GSList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_copy(list: *mut _GSList) -> *mut _GSList;
}


/*
void g_match_info_free()
	(GMatchInfo *) match_info [struct _GMatchInfo *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_match_info_free(match_info: *mut _GMatchInfo);
}


/*
gboolean g_key_file_save_to_file() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) filename [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_save_to_file(key_file: *mut _GKeyFile, filename: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
gchar ** g_variant_dup_bytestring_array() [char **]
	(GVariant *) value [struct _GVariant *]
	(gsize *) length [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_dup_bytestring_array(value: *mut _GVariant, length: *mut libc::c_ulong) -> *mut *mut libc::c_char;
}


/*
GVariantType * g_variant_type_new_tuple() [struct _GVariantType *]
	(const GVariantType *const *) items [const struct _GVariantType *const *]
	(gint) length [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_new_tuple(items: *const *const _GVariantType, length: libc::c_int) -> *mut _GVariantType;
}


/*
gpointer g_queue_peek_head() [void *]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_peek_head(queue: *mut _GQueue) -> *mut libc::c_void;
}


/*
GTimeZone * g_time_zone_ref() [struct _GTimeZone *]
	(GTimeZone *) tz [struct _GTimeZone *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_time_zone_ref(tz: *mut _GTimeZone) -> *mut _GTimeZone;
}


/*
const gchar *const * g_get_system_data_dirs() [const char *const *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_system_data_dirs() -> *const *const libc::c_char;
}


/*
GParamSpec ** g_object_class_list_properties() [struct _GParamSpec **]
	(GObjectClass *) oclass [struct _GObjectClass *]
	(guint *) n_properties [unsigned int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_class_list_properties(oclass: *mut _GObjectClass, n_properties: *mut libc::c_uint) -> *mut *mut _GParamSpec;
}


/*
void g_cclosure_marshal_VOID__BOXED()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_VOID__BOXED(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
GParamSpec * g_param_spec_pool_lookup() [struct _GParamSpec *]
	(GParamSpecPool *) pool [struct _GParamSpecPool *]
	(const gchar *) param_name [const char *]
	(GType) owner_type [unsigned long]
	(gboolean) walk_ancestors [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_pool_lookup(pool: *mut _GParamSpecPool, param_name: *const libc::c_char, owner_type: libc::c_ulong, walk_ancestors: libc::c_int) -> *mut _GParamSpec;
}


/*
void g_date_add_months()
	(GDate *) date [struct _GDate *]
	(guint) n_months [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_add_months(date: *mut _GDate, n_months: libc::c_uint);
}


/*
void g_variant_dict_clear()
	(GVariantDict *) dict [struct _GVariantDict *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_dict_clear(dict: *mut _GVariantDict);
}


/*
gint g_unichar_combining_class() [int]
	(gunichar) uc [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_combining_class(uc: libc::c_uint) -> libc::c_int;
}


/*
void g_signal_query()
	(guint) signal_id [unsigned int]
	(GSignalQuery *) query [struct _GSignalQuery *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_query(signal_id: libc::c_uint, query: *mut _GSignalQuery);
}


/*
void g_queue_push_nth()
	(GQueue *) queue [struct _GQueue *]
	(gpointer) data [void *]
	(gint) n [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_push_nth(queue: *mut _GQueue, data: *mut libc::c_void, n: libc::c_int);
}


/*
void g_queue_insert_after()
	(GQueue *) queue [struct _GQueue *]
	(GList *) sibling [struct _GList *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_insert_after(queue: *mut _GQueue, sibling: *mut _GList, data: *mut libc::c_void);
}


/*
GList * g_list_find() [struct _GList *]
	(GList *) list [struct _GList *]
	(gconstpointer) data [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_find(list: *mut _GList, data: *const libc::c_void) -> *mut _GList;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_base64_encode_step(in_: *const libc::c_uchar, len: libc::c_ulong, break_lines: libc::c_int, out: *mut libc::c_char, state: *mut libc::c_int, save: *mut libc::c_int) -> libc::c_ulong;
}


/*
void g_slist_pop_allocator()
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_pop_allocator();
}


/*
void g_ptr_array_sort()
	(GPtrArray *) array [struct _GPtrArray *]
	(GCompareFunc) compare_func [int (*)(const void *, const void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_sort(array: *mut _GPtrArray, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>);
}


/*
GMappedFile * g_mapped_file_ref() [struct _GMappedFile *]
	(GMappedFile *) file [struct _GMappedFile *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mapped_file_ref(file: *mut _GMappedFile) -> *mut _GMappedFile;
}


/*
GSList * g_slist_sort_with_data() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(GCompareDataFunc) compare_func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_sort_with_data(list: *mut _GSList, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void) -> *mut _GSList;
}


/*
void g_on_error_query()
	(const gchar *) prg_name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_on_error_query(prg_name: *const libc::c_char);
}


/*
gdouble g_strtod() [double]
	(const gchar *) nptr [const char *]
	(gchar **) endptr [char **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strtod(nptr: *const libc::c_char, endptr: *mut *mut libc::c_char) -> libc::c_double;
}


/*
gint64 * g_slice_get_config_state() [long *]
	(GSliceConfig) ckey [GSliceConfig]
	(gint64) address [long]
	(guint *) n_values [unsigned int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slice_get_config_state(ckey: libc::c_uint, address: libc::c_long, n_values: *mut libc::c_uint) -> *mut libc::c_long;
}


/*
GQuark g_type_qname() [unsigned int]
	(GType) type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_qname(type_: libc::c_ulong) -> libc::c_uint;
}


/*
const gchar ** g_variant_get_strv() [const char **]
	(GVariant *) value [struct _GVariant *]
	(gsize *) length [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_strv(value: *mut _GVariant, length: *mut libc::c_ulong) -> *mut *const libc::c_char;
}


/*
guint g_scanner_cur_position() [unsigned int]
	(GScanner *) scanner [struct _GScanner *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_cur_position(scanner: *mut _GScanner) -> libc::c_uint;
}


/*
GParamSpec * g_param_spec_boxed() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(GType) boxed_type [unsigned long]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_boxed(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, boxed_type: libc::c_ulong, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
gchar * g_key_file_get_locale_string() [char *]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(const gchar *) locale [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_get_locale_string(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, locale: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gboolean g_type_check_instance_is_a() [int]
	(GTypeInstance *) instance [struct _GTypeInstance *]
	(GType) iface_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_check_instance_is_a(instance: *mut _GTypeInstance, iface_type: libc::c_ulong) -> libc::c_int;
}


/*
GParamSpec * g_param_spec_pointer() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_pointer(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
guint g_bytes_hash() [unsigned int]
	(gconstpointer) bytes [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bytes_hash(bytes: *const libc::c_void) -> libc::c_uint;
}


/*
gchar * g_strdup_vprintf() [char *]
	(const gchar *) format [const char *]
	(va_list) args [struct __va_list_tag [1]]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_strdup_vprintf(format: *const libc::c_char, args: [__va_list_tag; 1]) -> *mut libc::c_char;
}


/*
gboolean g_bookmark_file_remove_item() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_remove_item(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
GList * g_completion_complete_utf8() [struct _GList *]
	(GCompletion *) cmp [struct _GCompletion *]
	(const gchar *) prefix [const char *]
	(gchar **) new_prefix [char **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_completion_complete_utf8(cmp: *mut _GCompletion, prefix: *const libc::c_char, new_prefix: *mut *mut libc::c_char) -> *mut _GList;
}


/*
GNode * g_node_find() [struct _GNode *]
	(GNode *) root [struct _GNode *]
	(GTraverseType) order [GTraverseType]
	(GTraverseFlags) flags [GTraverseFlags]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_find(root: *mut _GNode, order: libc::c_uint, flags: libc::c_uint, data: *mut libc::c_void) -> *mut _GNode;
}


/*
void g_info()
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_info(format: *const libc::c_char);
}


/*
gint g_date_time_compare() [int]
	(gconstpointer) dt1 [const void *]
	(gconstpointer) dt2 [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_compare(dt1: *const libc::c_void, dt2: *const libc::c_void) -> libc::c_int;
}


/*
void g_thread_foreach()
	(GFunc) thread_func [void (*)(void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_foreach(thread_func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
gboolean g_str_has_prefix() [int]
	(const gchar *) str [const char *]
	(const gchar *) prefix [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_str_has_prefix(str: *const libc::c_char, prefix: *const libc::c_char) -> libc::c_int;
}


/*
void g_queue_push_head()
	(GQueue *) queue [struct _GQueue *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_push_head(queue: *mut _GQueue, data: *mut libc::c_void);
}


/*
gpointer g_memdup() [void *]
	(gconstpointer) mem [const void *]
	(guint) byte_size [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_memdup(mem: *const libc::c_void, byte_size: libc::c_uint) -> *mut libc::c_void;
}


/*
const gchar * g_get_prgname() [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_prgname() -> *const libc::c_char;
}


/*
gboolean g_sequence_iter_is_begin() [int]
	(GSequenceIter *) iter [struct _GSequenceNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_iter_is_begin(iter: *mut _GSequenceNode) -> libc::c_int;
}


/*
void g_bookmark_file_set_description()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) description [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_set_description(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, description: *const libc::c_char);
}


/*
gboolean g_pattern_match() [int]
	(GPatternSpec *) pspec [struct _GPatternSpec *]
	(guint) string_length [unsigned int]
	(const gchar *) string [const char *]
	(const gchar *) string_reversed [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_pattern_match(pspec: *mut _GPatternSpec, string_length: libc::c_uint, string: *const libc::c_char, string_reversed: *const libc::c_char) -> libc::c_int;
}


/*
void g_variant_dict_unref()
	(GVariantDict *) dict [struct _GVariantDict *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_dict_unref(dict: *mut _GVariantDict);
}


/*
gboolean g_date_valid_dmy() [int]
	(GDateDay) day [unsigned char]
	(GDateMonth) month [GDateMonth]
	(GDateYear) year [unsigned short]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_valid_dmy(day: libc::c_uchar, month: libc::c_uint, year: libc::c_ushort) -> libc::c_int;
}


/*
gpointer g_async_queue_timed_pop_unlocked() [void *]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(GTimeVal *) end_time [struct _GTimeVal *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_timed_pop_unlocked(queue: *mut _GAsyncQueue, end_time: *mut _GTimeVal) -> *mut libc::c_void;
}


/*
void g_array_set_clear_func()
	(GArray *) array [struct _GArray *]
	(GDestroyNotify) clear_func [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_array_set_clear_func(array: *mut _GArray, clear_func: Option<extern fn(*mut libc::c_void)>);
}


/*
gchar * g_build_filename() [char *]
	(const gchar *) first_element [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_build_filename(first_element: *const libc::c_char) -> *mut libc::c_char;
}


/*
gpointer g_slice_alloc0() [void *]
	(gsize) block_size [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slice_alloc0(block_size: libc::c_ulong) -> *mut libc::c_void;
}


/*
GList * g_list_first() [struct _GList *]
	(GList *) list [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_first(list: *mut _GList) -> *mut _GList;
}


/*
gint g_relation_delete() [int]
	(GRelation *) relation [struct _GRelation *]
	(gconstpointer) key [const void *]
	(gint) field [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_relation_delete(relation: *mut _GRelation, key: *const libc::c_void, field: libc::c_int) -> libc::c_int;
}


/*
void g_cache_remove()
	(GCache *) cache [struct _GCache *]
	(gconstpointer) value [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cache_remove(cache: *mut _GCache, value: *const libc::c_void);
}


/*
GIOError g_io_channel_write() [GIOError]
	(GIOChannel *) channel [struct _GIOChannel *]
	(const gchar *) buf [const char *]
	(gsize) count [unsigned long]
	(gsize *) bytes_written [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_write(channel: *mut _GIOChannel, buf: *const libc::c_char, count: libc::c_ulong, bytes_written: *mut libc::c_ulong) -> libc::c_uint;
}


/*
gboolean g_key_file_load_from_data_dirs() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) file [const char *]
	(gchar **) full_path [char **]
	(GKeyFileFlags) flags [GKeyFileFlags]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_load_from_data_dirs(key_file: *mut _GKeyFile, file: *const libc::c_char, full_path: *mut *mut libc::c_char, flags: libc::c_uint, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_test_bug()
	(const char *) bug_uri_snippet
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_bug(bug_uri_snippet: *const libc::c_char);
}


/*
guint g_string_hash() [unsigned int]
	(const GString *) str [const struct _GString *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_hash(str: *const _GString) -> libc::c_uint;
}


/*
gpointer g_async_queue_try_pop() [void *]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_try_pop(queue: *mut _GAsyncQueue) -> *mut libc::c_void;
}


/*
void g_value_take_param()
	(GValue *) value [struct _GValue *]
	(GParamSpec *) param [struct _GParamSpec *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_take_param(value: *mut _GValue, param: *mut _GParamSpec);
}


/*
GIOStatus g_io_channel_read_line_string() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(GString *) buffer [struct _GString *]
	(gsize *) terminator_pos [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_read_line_string(channel: *mut _GIOChannel, buffer: *mut _GString, terminator_pos: *mut libc::c_ulong, error: *mut *mut _GError) -> libc::c_uint;
}


/*
void g_thread_init_with_errorcheck_mutexes()
	(gpointer) vtable [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_init_with_errorcheck_mutexes(vtable: *mut libc::c_void);
}


/*
void g_set_error()
	(GError **) err [struct _GError **]
	(GQuark) domain [unsigned int]
	(gint) code [int]
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_set_error(err: *mut *mut _GError, domain: libc::c_uint, code: libc::c_int, format: *const libc::c_char);
}


/*
GList * g_queue_peek_head_link() [struct _GList *]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_peek_head_link(queue: *mut _GQueue) -> *mut _GList;
}


/*
GList * g_list_alloc() [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_alloc() -> *mut _GList;
}


/*
GSList * g_slist_insert_before() [struct _GSList *]
	(GSList *) slist [struct _GSList *]
	(GSList *) sibling [struct _GSList *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_insert_before(slist: *mut _GSList, sibling: *mut _GSList, data: *mut libc::c_void) -> *mut _GSList;
}


/*
void g_mapped_file_free()
	(GMappedFile *) file [struct _GMappedFile *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mapped_file_free(file: *mut _GMappedFile);
}


/*
void g_scanner_scope_add_symbol()
	(GScanner *) scanner [struct _GScanner *]
	(guint) scope_id [unsigned int]
	(const gchar *) symbol [const char *]
	(gpointer) value [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_scope_add_symbol(scanner: *mut _GScanner, scope_id: libc::c_uint, symbol: *const libc::c_char, value: *mut libc::c_void);
}


/*
void g_mem_chunk_destroy()
	(GMemChunk *) mem_chunk [struct _GMemChunk *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mem_chunk_destroy(mem_chunk: *mut _GMemChunk);
}


/*
gint g_date_time_get_second() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_get_second(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
GDateTime * g_date_time_add_hours() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
	(gint) hours [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_add_hours(datetime: *mut _GDateTime, hours: libc::c_int) -> *mut _GDateTime;
}


/*
gsize g_variant_iter_init() [unsigned long]
	(GVariantIter *) iter [struct _GVariantIter *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_iter_init(iter: *mut _GVariantIter, value: *mut _GVariant) -> libc::c_ulong;
}


/*
GCond * g_cond_new() [struct _GCond *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cond_new() -> *mut _GCond;
}


/*
gchar * g_bookmark_file_get_title() [char *]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_get_title(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
void g_option_group_free()
	(GOptionGroup *) group [struct _GOptionGroup *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_group_free(group: *mut _GOptionGroup);
}


/*
void g_mutex_clear()
	(GMutex *) mutex [union _GMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mutex_clear(mutex: *mut _GMutex);
}


/*
void g_hook_list_invoke_check()
	(GHookList *) hook_list [struct _GHookList *]
	(gboolean) may_recurse [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_list_invoke_check(hook_list: *mut _GHookList, may_recurse: libc::c_int);
}


/*
GValueArray * g_value_array_remove() [struct _GValueArray *]
	(GValueArray *) value_array [struct _GValueArray *]
	(guint) index_ [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_array_remove(value_array: *mut _GValueArray, index_: libc::c_uint) -> *mut _GValueArray;
}


/*
void g_value_set_param()
	(GValue *) value [struct _GValue *]
	(GParamSpec *) param [struct _GParamSpec *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_param(value: *mut _GValue, param: *mut _GParamSpec);
}


/*
void g_thread_pool_stop_unused_threads()
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_pool_stop_unused_threads();
}


/*
void g_static_rw_lock_reader_unlock()
	(GStaticRWLock *) lock [struct _GStaticRWLock *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_rw_lock_reader_unlock(lock: *mut _GStaticRWLock);
}


/*
void g_rw_lock_reader_unlock()
	(GRWLock *) rw_lock [struct _GRWLock *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rw_lock_reader_unlock(rw_lock: *mut _GRWLock);
}


/*
gint g_source_get_priority() [int]
	(GSource *) source [struct _GSource *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_get_priority(source: *mut _GSource) -> libc::c_int;
}


/*
guint g_signal_new_valist() [unsigned int]
	(const gchar *) signal_name [const char *]
	(GType) itype [unsigned long]
	(GSignalFlags) signal_flags [GSignalFlags]
	(GClosure *) class_closure [struct _GClosure *]
	(GSignalAccumulator) accumulator [int (*)(struct _GSignalInvocationHint *, struct _GValue *, const struct _GValue *, void *)]
	(gpointer) accu_data [void *]
	(GSignalCMarshaller) c_marshaller [void (*)(struct _GClosure *, struct _GValue *, unsigned int, const struct _GValue *, void *, void *)]
	(GType) return_type [unsigned long]
	(guint) n_params [unsigned int]
	(va_list) args [struct __va_list_tag [1]]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_signal_new_valist(signal_name: *const libc::c_char, itype: libc::c_ulong, signal_flags: libc::c_uint, class_closure: *mut _GClosure, accumulator: Option<extern fn(*mut _GSignalInvocationHint, *mut _GValue, *const _GValue, *mut libc::c_void) -> libc::c_int>, accu_data: *mut libc::c_void, c_marshaller: Option<extern fn(*mut _GClosure, *mut _GValue, libc::c_uint, *const _GValue, *mut libc::c_void, *mut libc::c_void)>, return_type: libc::c_ulong, n_params: libc::c_uint, args: [__va_list_tag; 1]) -> libc::c_uint;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_replace_literal(regex: *const _GRegex, string: *const libc::c_char, string_len: libc::c_long, start_position: libc::c_int, replacement: *const libc::c_char, match_options: libc::c_uint, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
GVariant * g_variant_new_handle() [struct _GVariant *]
	(gint32) value [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_handle(value: libc::c_int) -> *mut _GVariant;
}


/*
void g_hook_list_marshal_check()
	(GHookList *) hook_list [struct _GHookList *]
	(gboolean) may_recurse [int]
	(GHookCheckMarshaller) marshaller [int (*)(struct _GHook *, void *)]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_list_marshal_check(hook_list: *mut _GHookList, may_recurse: libc::c_int, marshaller: Option<extern fn(*mut _GHook, *mut libc::c_void) -> libc::c_int>, marshal_data: *mut libc::c_void);
}


/*
GVariant * g_variant_new_tuple() [struct _GVariant *]
	(GVariant *const *) children [struct _GVariant *const *]
	(gsize) n_children [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_tuple(children: *const *mut _GVariant, n_children: libc::c_ulong) -> *mut _GVariant;
}


/*
GType g_object_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_get_type() -> libc::c_ulong;
}


/*
gsize g_atomic_pointer_or() [unsigned long]
	(volatile void *) atomic
	(gsize) val [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_atomic_pointer_or(atomic: *mut libc::c_void, val: libc::c_ulong) -> libc::c_ulong;
}


/*
gint32 g_variant_get_int32() [int]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_int32(value: *mut _GVariant) -> libc::c_int;
}


/*
GVariant * g_variant_new_bytestring_array() [struct _GVariant *]
	(const gchar *const *) strv [const char *const *]
	(gssize) length [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_bytestring_array(strv: *const *const libc::c_char, length: libc::c_long) -> *mut _GVariant;
}


/*
void g_byte_array_sort_with_data()
	(GByteArray *) array [struct _GByteArray *]
	(GCompareDataFunc) compare_func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_byte_array_sort_with_data(array: *mut _GByteArray, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
GEnumValue * g_enum_get_value_by_name() [struct _GEnumValue *]
	(GEnumClass *) enum_class [struct _GEnumClass *]
	(const gchar *) name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_enum_get_value_by_name(enum_class: *mut _GEnumClass, name: *const libc::c_char) -> *mut _GEnumValue;
}


/*
void g_rw_lock_reader_lock()
	(GRWLock *) rw_lock [struct _GRWLock *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rw_lock_reader_lock(rw_lock: *mut _GRWLock);
}


/*
gunichar * g_utf8_to_ucs4_fast() [unsigned int *]
	(const gchar *) str [const char *]
	(glong) len [long]
	(glong *) items_written [long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_to_ucs4_fast(str: *const libc::c_char, len: libc::c_long, items_written: *mut libc::c_long) -> *mut libc::c_uint;
}


/*
gboolean g_hash_table_replace() [int]
	(GHashTable *) hash_table [struct _GHashTable *]
	(gpointer) key [void *]
	(gpointer) value [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_replace(hash_table: *mut _GHashTable, key: *mut libc::c_void, value: *mut libc::c_void) -> libc::c_int;
}


/*
void g_atomic_int_set()
	(volatile gint *) atomic [volatile int *]
	(gint) newval [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_atomic_int_set(atomic: *mut libc::c_int, newval: libc::c_int);
}


/*
GType g_type_register_static() [unsigned long]
	(GType) parent_type [unsigned long]
	(const gchar *) type_name [const char *]
	(const GTypeInfo *) info [const struct _GTypeInfo *]
	(GTypeFlags) flags [GTypeFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_register_static(parent_type: libc::c_ulong, type_name: *const libc::c_char, info: *const _GTypeInfo, flags: libc::c_uint) -> libc::c_ulong;
}


/*
GError * g_error_new_literal() [struct _GError *]
	(GQuark) domain [unsigned int]
	(gint) code [int]
	(const gchar *) message [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_error_new_literal(domain: libc::c_uint, code: libc::c_int, message: *const libc::c_char) -> *mut _GError;
}


/*
gint g_queue_link_index() [int]
	(GQueue *) queue [struct _GQueue *]
	(GList *) link_ [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_link_index(queue: *mut _GQueue, link_: *mut _GList) -> libc::c_int;
}


/*
void g_mutex_free()
	(GMutex *) mutex [union _GMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mutex_free(mutex: *mut _GMutex);
}


/*
void g_thread_yield()
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_yield();
}


/*
guint64 g_value_get_uint64() [unsigned long]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_uint64(value: *const _GValue) -> libc::c_ulong;
}


/*
double g_test_rand_double()
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_rand_double() -> libc::c_double;
}


/*
gboolean g_int64_equal() [int]
	(gconstpointer) v1 [const void *]
	(gconstpointer) v2 [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_int64_equal(v1: *const libc::c_void, v2: *const libc::c_void) -> libc::c_int;
}


/*
void g_option_group_add_entries()
	(GOptionGroup *) group [struct _GOptionGroup *]
	(const GOptionEntry *) entries [const struct _GOptionEntry *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_group_add_entries(group: *mut _GOptionGroup, entries: *const _GOptionEntry);
}


/*
GByteArray * g_byte_array_remove_index() [struct _GByteArray *]
	(GByteArray *) array [struct _GByteArray *]
	(guint) index_ [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_byte_array_remove_index(array: *mut _GByteArray, index_: libc::c_uint) -> *mut _GByteArray;
}


/*
void g_cclosure_marshal_VOID__UCHARv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_cclosure_marshal_VOID__UCHARv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
gchar * g_filename_to_utf8() [char *]
	(const gchar *) opsysstring [const char *]
	(gssize) len [long]
	(gsize *) bytes_read [unsigned long *]
	(gsize *) bytes_written [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_filename_to_utf8(opsysstring: *const libc::c_char, len: libc::c_long, bytes_read: *mut libc::c_ulong, bytes_written: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gchar * g_strrstr() [char *]
	(const gchar *) haystack [const char *]
	(const gchar *) needle [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strrstr(haystack: *const libc::c_char, needle: *const libc::c_char) -> *mut libc::c_char;
}


/*
gint g_date_days_between() [int]
	(const GDate *) date1 [const struct _GDate *]
	(const GDate *) date2 [const struct _GDate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_days_between(date1: *const _GDate, date2: *const _GDate) -> libc::c_int;
}


/*
GParamSpec * g_param_spec_ref() [struct _GParamSpec *]
	(GParamSpec *) pspec [struct _GParamSpec *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_ref(pspec: *mut _GParamSpec) -> *mut _GParamSpec;
}


/*
const gchar * g_get_host_name() [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_host_name() -> *const libc::c_char;
}


/*
void g_date_set_time_t()
	(GDate *) date [struct _GDate *]
	(time_t) timet [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_set_time_t(date: *mut _GDate, timet: libc::c_long);
}


/*
GType g_variant_type_get_gtype() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_get_gtype() -> libc::c_ulong;
}


/*
GDateWeekday g_date_get_weekday() [GDateWeekday]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_get_weekday(date: *const _GDate) -> libc::c_uint;
}


/*
gboolean g_hostname_is_non_ascii() [int]
	(const gchar *) hostname [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hostname_is_non_ascii(hostname: *const libc::c_char) -> libc::c_int;
}


/*
gsize g_variant_get_size() [unsigned long]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_size(value: *mut _GVariant) -> libc::c_ulong;
}


/*
void g_static_rec_mutex_unlock()
	(GStaticRecMutex *) mutex [struct _GStaticRecMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_rec_mutex_unlock(mutex: *mut _GStaticRecMutex);
}


/*
gboolean g_atomic_pointer_compare_and_exchange() [int]
	(volatile void *) atomic
	(gpointer) oldval [void *]
	(gpointer) newval [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_atomic_pointer_compare_and_exchange(atomic: *mut libc::c_void, oldval: *mut libc::c_void, newval: *mut libc::c_void) -> libc::c_int;
}


/*
void _g_signals_destroy()
	(GType) itype [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn _g_signals_destroy(itype: libc::c_ulong);
}


/*
gchar * g_strndup() [char *]
	(const gchar *) str [const char *]
	(gsize) n [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strndup(str: *const libc::c_char, n: libc::c_ulong) -> *mut libc::c_char;
}


/*
gpointer g_param_spec_steal_qdata() [void *]
	(GParamSpec *) pspec [struct _GParamSpec *]
	(GQuark) quark [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_steal_qdata(pspec: *mut _GParamSpec, quark: libc::c_uint) -> *mut libc::c_void;
}


/*
void g_relation_index()
	(GRelation *) relation [struct _GRelation *]
	(gint) field [int]
	(GHashFunc) hash_func [unsigned int (*)(const void *)]
	(GEqualFunc) key_equal_func [int (*)(const void *, const void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_relation_index(relation: *mut _GRelation, field: libc::c_int, hash_func: Option<extern fn(*const libc::c_void) -> libc::c_uint>, key_equal_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>);
}


/*
void g_test_log_buffer_free()
	(GTestLogBuffer *) tbuffer [GTestLogBuffer *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_log_buffer_free(tbuffer: *mut GTestLogBuffer);
}


/*
gchar * g_key_file_get_string() [char *]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_get_string(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
void g_unsetenv()
	(const gchar *) variable [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unsetenv(variable: *const libc::c_char);
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_match_all_full(regex: *const _GRegex, string: *const libc::c_char, string_len: libc::c_long, start_position: libc::c_int, match_options: libc::c_uint, match_info: *mut *mut _GMatchInfo, error: *mut *mut _GError) -> libc::c_int;
}


/*
const gchar * g_get_real_name() [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_real_name() -> *const libc::c_char;
}


/*
void g_key_file_set_value()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(const gchar *) value [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_set_value(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, value: *const libc::c_char);
}


/*
guint g_node_depth() [unsigned int]
	(GNode *) node [struct _GNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_depth(node: *mut _GNode) -> libc::c_uint;
}


/*
gpointer g_once_impl() [void *]
	(GOnce *) once [struct _GOnce *]
	(GThreadFunc) func [void *(*)(void *)]
	(gpointer) arg [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_once_impl(once: *mut _GOnce, func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>, arg: *mut libc::c_void) -> *mut libc::c_void;
}


/*
const gchar * glib_check_version() [const char *]
	(guint) required_major [unsigned int]
	(guint) required_minor [unsigned int]
	(guint) required_micro [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn glib_check_version(required_major: libc::c_uint, required_minor: libc::c_uint, required_micro: libc::c_uint) -> *const libc::c_char;
}


/*
guint g_variant_hash() [unsigned int]
	(gconstpointer) value [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_hash(value: *const libc::c_void) -> libc::c_uint;
}


/*
void g_async_queue_push_sorted_unlocked()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(gpointer) data [void *]
	(GCompareDataFunc) func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_push_sorted_unlocked(queue: *mut _GAsyncQueue, data: *mut libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
void g_thread_init()
	(gpointer) vtable [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_init(vtable: *mut libc::c_void);
}


/*
gboolean g_value_get_boolean() [int]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_boolean(value: *const _GValue) -> libc::c_int;
}


/*
void g_cclosure_marshal_VOID__VARIANT()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_VOID__VARIANT(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
void g_key_file_free()
	(GKeyFile *) key_file [struct _GKeyFile *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_free(key_file: *mut _GKeyFile);
}


/*
void g_tree_traverse()
	(GTree *) tree [struct _GTree *]
	(GTraverseFunc) traverse_func [int (*)(void *, void *, void *)]
	(GTraverseType) traverse_type [GTraverseType]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_tree_traverse(tree: *mut _GTree, traverse_func: Option<extern fn(*mut libc::c_void, *mut libc::c_void, *mut libc::c_void) -> libc::c_int>, traverse_type: libc::c_uint, user_data: *mut libc::c_void);
}


/*
gboolean g_once_init_enter() [int]
	(volatile void *) location
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_once_init_enter(location: *mut libc::c_void) -> libc::c_int;
}


/*
GValueArray * g_value_array_prepend() [struct _GValueArray *]
	(GValueArray *) value_array [struct _GValueArray *]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_array_prepend(value_array: *mut _GValueArray, value: *const _GValue) -> *mut _GValueArray;
}


/*
GQuark g_spawn_exit_error_quark() [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_spawn_exit_error_quark() -> libc::c_uint;
}


/*
gboolean g_get_filename_charsets() [int]
	(const gchar ***) charsets [const char ***]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_filename_charsets(charsets: *mut *mut *const libc::c_char) -> libc::c_int;
}


/*
GList * g_queue_peek_tail_link() [struct _GList *]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_peek_tail_link(queue: *mut _GQueue) -> *mut _GList;
}


/*
gpointer g_param_spec_get_qdata() [void *]
	(GParamSpec *) pspec [struct _GParamSpec *]
	(GQuark) quark [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_get_qdata(pspec: *mut _GParamSpec, quark: libc::c_uint) -> *mut libc::c_void;
}


/*
GList * g_list_find_custom() [struct _GList *]
	(GList *) list [struct _GList *]
	(gconstpointer) data [const void *]
	(GCompareFunc) func [int (*)(const void *, const void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_find_custom(list: *mut _GList, data: *const libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GList;
}


/*
void g_test_incomplete()
	(const gchar *) msg [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_incomplete(msg: *const libc::c_char);
}


/*
gpointer g_try_malloc0_n() [void *]
	(gsize) n_blocks [unsigned long]
	(gsize) n_block_bytes [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_try_malloc0_n(n_blocks: libc::c_ulong, n_block_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
gboolean g_type_check_value() [int]
	(GValue *) value [struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_check_value(value: *mut _GValue) -> libc::c_int;
}


/*
void g_mem_chunk_reset()
	(GMemChunk *) mem_chunk [struct _GMemChunk *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mem_chunk_reset(mem_chunk: *mut _GMemChunk);
}


/*
void g_weak_ref_init()
	(GWeakRef *) weak_ref [GWeakRef *]
	(gpointer) object [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_weak_ref_init(weak_ref: *mut GWeakRef, object: *mut libc::c_void);
}


/*
gdouble g_value_get_double() [double]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_double(value: *const _GValue) -> libc::c_double;
}


/*
gchar * _g_utf8_make_valid() [char *]
	(const gchar *) name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn _g_utf8_make_valid(name: *const libc::c_char) -> *mut libc::c_char;
}


/*
void g_variant_unref()
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_unref(value: *mut _GVariant);
}


/*
void g_hook_list_clear()
	(GHookList *) hook_list [struct _GHookList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_list_clear(hook_list: *mut _GHookList);
}


/*
guint g_source_get_id() [unsigned int]
	(GSource *) source [struct _GSource *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_get_id(source: *mut _GSource) -> libc::c_uint;
}


/*
GVariant * g_variant_new_variant() [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_variant(value: *mut _GVariant) -> *mut _GVariant;
}


/*
void g_cclosure_marshal_VOID__ENUM()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_VOID__ENUM(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
gboolean g_bookmark_file_load_from_data() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) data [const char *]
	(gsize) length [unsigned long]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_load_from_data(bookmark: *mut _GBookmarkFile, data: *const libc::c_char, length: libc::c_ulong, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_ptr_array_unref()
	(GPtrArray *) array [struct _GPtrArray *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_unref(array: *mut _GPtrArray);
}


/*
gpointer g_try_malloc() [void *]
	(gsize) n_bytes [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_try_malloc(n_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
void g_pointer_bit_unlock()
	(volatile void *) address
	(gint) lock_bit [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_pointer_bit_unlock(address: *mut libc::c_void, lock_bit: libc::c_int);
}


/*
const gchar * g_getenv() [const char *]
	(const gchar *) variable [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_getenv(variable: *const libc::c_char) -> *const libc::c_char;
}


/*
gboolean g_spawn_check_exit_status() [int]
	(gint) exit_status [int]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_spawn_check_exit_status(exit_status: libc::c_int, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_hash_table_iter_replace()
	(GHashTableIter *) iter [struct _GHashTableIter *]
	(gpointer) value [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_iter_replace(iter: *mut _GHashTableIter, value: *mut libc::c_void);
}


/*
GHook * g_hook_alloc() [struct _GHook *]
	(GHookList *) hook_list [struct _GHookList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_alloc(hook_list: *mut _GHookList) -> *mut _GHook;
}


/*
GVariantDict * g_variant_dict_new() [struct _GVariantDict *]
	(GVariant *) from_asv [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_dict_new(from_asv: *mut _GVariant) -> *mut _GVariantDict;
}


/*
void g_param_spec_pool_insert()
	(GParamSpecPool *) pool [struct _GParamSpecPool *]
	(GParamSpec *) pspec [struct _GParamSpec *]
	(GType) owner_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_pool_insert(pool: *mut _GParamSpecPool, pspec: *mut _GParamSpec, owner_type: libc::c_ulong);
}


/*
GType g_type_register_static_simple() [unsigned long]
	(GType) parent_type [unsigned long]
	(const gchar *) type_name [const char *]
	(guint) class_size [unsigned int]
	(GClassInitFunc) class_init [void (*)(void *, void *)]
	(guint) instance_size [unsigned int]
	(GInstanceInitFunc) instance_init [void (*)(struct _GTypeInstance *, void *)]
	(GTypeFlags) flags [GTypeFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_register_static_simple(parent_type: libc::c_ulong, type_name: *const libc::c_char, class_size: libc::c_uint, class_init: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>, instance_size: libc::c_uint, instance_init: Option<extern fn(*mut _GTypeInstance, *mut libc::c_void)>, flags: libc::c_uint) -> libc::c_ulong;
}


/*
gunichar g_unichar_tolower() [unsigned int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_tolower(c: libc::c_uint) -> libc::c_uint;
}


/*
GVariant * g_variant_ref() [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_ref(value: *mut _GVariant) -> *mut _GVariant;
}


/*
void g_signal_handler_disconnect()
	(gpointer) instance [void *]
	(gulong) handler_id [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_handler_disconnect(instance: *mut libc::c_void, handler_id: libc::c_ulong);
}


/*
char * g_uri_unescape_segment()
	(const char *) escaped_string
	(const char *) escaped_string_end
	(const char *) illegal_characters
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_uri_unescape_segment(escaped_string: *const libc::c_char, escaped_string_end: *const libc::c_char, illegal_characters: *const libc::c_char) -> *mut libc::c_char;
}


/*
gboolean g_file_get_contents() [int]
	(const gchar *) filename [const char *]
	(gchar **) contents [char **]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_file_get_contents(filename: *const libc::c_char, contents: *mut *mut libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_key_file_set_uint64()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(guint64) value [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_set_uint64(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, value: libc::c_ulong);
}


/*
void g_time_zone_unref()
	(GTimeZone *) tz [struct _GTimeZone *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_time_zone_unref(tz: *mut _GTimeZone);
}


/*
gint g_bit_nth_lsf() [int]
	(gulong) mask [unsigned long]
	(gint) nth_bit [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bit_nth_lsf(mask: libc::c_ulong, nth_bit: libc::c_int) -> libc::c_int;
}


/*
void g_message()
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_message(format: *const libc::c_char);
}


/*
GType g_thread_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_get_type() -> libc::c_ulong;
}


/*
guint g_list_length() [unsigned int]
	(GList *) list [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_length(list: *mut _GList) -> libc::c_uint;
}


/*
void g_test_add_func()
	(const char *) testpath
	(GTestFunc) test_func [void (*)(void)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_add_func(testpath: *const libc::c_char, test_func: Option<extern fn()>);
}


/*
GList * g_list_remove_link() [struct _GList *]
	(GList *) list [struct _GList *]
	(GList *) llink [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_remove_link(list: *mut _GList, llink: *mut _GList) -> *mut _GList;
}


/*
void g_param_spec_pool_remove()
	(GParamSpecPool *) pool [struct _GParamSpecPool *]
	(GParamSpec *) pspec [struct _GParamSpec *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_pool_remove(pool: *mut _GParamSpecPool, pspec: *mut _GParamSpec);
}


/*
void g_param_spec_unref()
	(GParamSpec *) pspec [struct _GParamSpec *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_unref(pspec: *mut _GParamSpec);
}


/*
gpointer g_malloc() [void *]
	(gsize) n_bytes [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_malloc(n_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
gssize g_atomic_pointer_add() [long]
	(volatile void *) atomic
	(gssize) val [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_atomic_pointer_add(atomic: *mut libc::c_void, val: libc::c_long) -> libc::c_long;
}


/*
guint g_atomic_int_or() [unsigned int]
	(volatile guint *) atomic [volatile unsigned int *]
	(guint) val [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_atomic_int_or(atomic: *mut libc::c_uint, val: libc::c_uint) -> libc::c_uint;
}


/*
gchar * g_strjoin() [char *]
	(const gchar *) separator [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strjoin(separator: *const libc::c_char) -> *mut libc::c_char;
}


/*
gpointer g_trash_stack_pop() [void *]
	(GTrashStack **) stack_p [struct _GTrashStack **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_trash_stack_pop(stack_p: *mut *mut _GTrashStack) -> *mut libc::c_void;
}


/*
void g_value_set_flags()
	(GValue *) value [struct _GValue *]
	(guint) v_flags [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_flags(value: *mut _GValue, v_flags: libc::c_uint);
}


/*
const GValue * g_param_spec_get_default_value() [const struct _GValue *]
	(GParamSpec *) param [struct _GParamSpec *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_get_default_value(param: *mut _GParamSpec) -> *const _GValue;
}


/*
const gchar * g_checksum_get_string() [const char *]
	(GChecksum *) checksum [struct _GChecksum *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_checksum_get_string(checksum: *mut _GChecksum) -> *const libc::c_char;
}


/*
GChecksum * g_checksum_new() [struct _GChecksum *]
	(GChecksumType) checksum_type [GChecksumType]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_checksum_new(checksum_type: libc::c_uint) -> *mut _GChecksum;
}


/*
GVariant * g_variant_new_array() [struct _GVariant *]
	(const GVariantType *) child_type [const struct _GVariantType *]
	(GVariant *const *) children [struct _GVariant *const *]
	(gsize) n_children [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_array(child_type: *const _GVariantType, children: *const *mut _GVariant, n_children: libc::c_ulong) -> *mut _GVariant;
}


/*
void g_object_run_dispose()
	(GObject *) object [struct _GObject *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_run_dispose(object: *mut _GObject);
}


/*
gint g_ascii_xdigit_value() [int]
	(gchar) c [char]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ascii_xdigit_value(c: libc::c_char) -> libc::c_int;
}


/*
void g_dataset_id_set_data_full()
	(gconstpointer) dataset_location [const void *]
	(GQuark) key_id [unsigned int]
	(gpointer) data [void *]
	(GDestroyNotify) destroy_func [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_dataset_id_set_data_full(dataset_location: *const libc::c_void, key_id: libc::c_uint, data: *mut libc::c_void, destroy_func: Option<extern fn(*mut libc::c_void)>);
}


/*
void g_warning()
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_warning(format: *const libc::c_char);
}


/*
gchar * g_strconcat() [char *]
	(const gchar *) string1 [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strconcat(string1: *const libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_compute_hmac_for_string() [char *]
	(GChecksumType) digest_type [GChecksumType]
	(const guchar *) key [const unsigned char *]
	(gsize) key_len [unsigned long]
	(const gchar *) str [const char *]
	(gssize) length [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_compute_hmac_for_string(digest_type: libc::c_uint, key: *const libc::c_uchar, key_len: libc::c_ulong, str: *const libc::c_char, length: libc::c_long) -> *mut libc::c_char;
}


/*
gchar * g_str_to_ascii() [char *]
	(const gchar *) str [const char *]
	(const gchar *) from_locale [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_str_to_ascii(str: *const libc::c_char, from_locale: *const libc::c_char) -> *mut libc::c_char;
}


/*
GString * g_string_prepend_unichar() [struct _GString *]
	(GString *) string [struct _GString *]
	(gunichar) wc [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_prepend_unichar(string: *mut _GString, wc: libc::c_uint) -> *mut _GString;
}


/*
void g_return_if_fail_warning()
	(const char *) log_domain
	(const char *) pretty_function
	(const char *) expression
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_return_if_fail_warning(log_domain: *const libc::c_char, pretty_function: *const libc::c_char, expression: *const libc::c_char);
}


/*
void g_main_context_release()
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_release(context: *mut _GMainContext);
}


/*
void g_cclosure_marshal_STRING__OBJECT_POINTERv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_cclosure_marshal_STRING__OBJECT_POINTERv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
const gchar * g_dngettext() [const char *]
	(const gchar *) domain [const char *]
	(const gchar *) msgid [const char *]
	(const gchar *) msgid_plural [const char *]
	(gulong) n [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_dngettext(domain: *const libc::c_char, msgid: *const libc::c_char, msgid_plural: *const libc::c_char, n: libc::c_ulong) -> *const libc::c_char;
}


/*
gboolean g_hash_table_add() [int]
	(GHashTable *) hash_table [struct _GHashTable *]
	(gpointer) key [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_add(hash_table: *mut _GHashTable, key: *mut libc::c_void) -> libc::c_int;
}


/*
const gchar * g_strsignal() [const char *]
	(gint) signum [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strsignal(signum: libc::c_int) -> *const libc::c_char;
}


/*
gboolean g_regex_match_simple() [int]
	(const gchar *) pattern [const char *]
	(const gchar *) string [const char *]
	(GRegexCompileFlags) compile_options [GRegexCompileFlags]
	(GRegexMatchFlags) match_options [GRegexMatchFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_match_simple(pattern: *const libc::c_char, string: *const libc::c_char, compile_options: libc::c_uint, match_options: libc::c_uint) -> libc::c_int;
}


/*
GList * g_list_insert_sorted() [struct _GList *]
	(GList *) list [struct _GList *]
	(gpointer) data [void *]
	(GCompareFunc) func [int (*)(const void *, const void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_insert_sorted(list: *mut _GList, data: *mut libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GList;
}


/*
GParamSpec * g_param_spec_unichar() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(gunichar) default_value [unsigned int]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_unichar(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, default_value: libc::c_uint, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
void g_bookmark_file_set_mime_type()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) mime_type [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_set_mime_type(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, mime_type: *const libc::c_char);
}


/*
gboolean g_regex_match() [int]
	(const GRegex *) regex [const struct _GRegex *]
	(const gchar *) string [const char *]
	(GRegexMatchFlags) match_options [GRegexMatchFlags]
	(GMatchInfo **) match_info [struct _GMatchInfo **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_match(regex: *const _GRegex, string: *const libc::c_char, match_options: libc::c_uint, match_info: *mut *mut _GMatchInfo) -> libc::c_int;
}


/*
gboolean g_key_file_remove_comment() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_remove_comment(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_unichar_isgraph() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_isgraph(c: libc::c_uint) -> libc::c_int;
}


/*
void g_node_unlink()
	(GNode *) node [struct _GNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_unlink(node: *mut _GNode);
}


/*
gboolean g_unichar_ismark() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_ismark(c: libc::c_uint) -> libc::c_int;
}


/*
GNode * g_node_last_child() [struct _GNode *]
	(GNode *) node [struct _GNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_last_child(node: *mut _GNode) -> *mut _GNode;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_get_app_info(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, name: *const libc::c_char, exec: *mut *mut libc::c_char, count: *mut libc::c_uint, stamp: *mut libc::c_long, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_unichar_ispunct() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_ispunct(c: libc::c_uint) -> libc::c_int;
}


/*
GBinding * g_object_bind_property_with_closures() [struct _GBinding *]
	(gpointer) source [void *]
	(const gchar *) source_property [const char *]
	(gpointer) target [void *]
	(const gchar *) target_property [const char *]
	(GBindingFlags) flags [GBindingFlags]
	(GClosure *) transform_to [struct _GClosure *]
	(GClosure *) transform_from [struct _GClosure *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_bind_property_with_closures(source: *mut libc::c_void, source_property: *const libc::c_char, target: *mut libc::c_void, target_property: *const libc::c_char, flags: libc::c_uint, transform_to: *mut _GClosure, transform_from: *mut _GClosure) -> *mut _GBinding;
}


/*
GValueArray * g_value_array_copy() [struct _GValueArray *]
	(const GValueArray *) value_array [const struct _GValueArray *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_array_copy(value_array: *const _GValueArray) -> *mut _GValueArray;
}


/*
gboolean g_bookmark_file_has_group() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) group [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_has_group(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, group: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_cclosure_marshal_STRING__OBJECT_POINTER()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_STRING__OBJECT_POINTER(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
void g_value_set_param_take_ownership()
	(GValue *) value [struct _GValue *]
	(GParamSpec *) param [struct _GParamSpec *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_param_take_ownership(value: *mut _GValue, param: *mut _GParamSpec);
}


/*
gboolean g_object_replace_data() [int]
	(GObject *) object [struct _GObject *]
	(const gchar *) key [const char *]
	(gpointer) oldval [void *]
	(gpointer) newval [void *]
	(GDestroyNotify) destroy [void (*)(void *)]
	(GDestroyNotify *) old_destroy [void (**)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_replace_data(object: *mut _GObject, key: *const libc::c_char, oldval: *mut libc::c_void, newval: *mut libc::c_void, destroy: Option<extern fn(*mut libc::c_void)>, old_destroy: *mut Option<extern fn(*mut libc::c_void)>) -> libc::c_int;
}


/*
GRand * g_rand_new() [struct _GRand *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rand_new() -> *mut _GRand;
}


/*
void g_static_rw_lock_free()
	(GStaticRWLock *) lock [struct _GStaticRWLock *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_rw_lock_free(lock: *mut _GStaticRWLock);
}


/*
gpointer g_hash_table_find() [void *]
	(GHashTable *) hash_table [struct _GHashTable *]
	(GHRFunc) predicate [int (*)(void *, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_find(hash_table: *mut _GHashTable, predicate: Option<extern fn(*mut libc::c_void, *mut libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void) -> *mut libc::c_void;
}


/*
gpointer g_object_new() [void *]
	(GType) object_type [unsigned long]
	(const gchar *) first_property_name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_new(object_type: libc::c_ulong, first_property_name: *const libc::c_char) -> *mut libc::c_void;
}


/*
GFlagsValue * g_flags_get_value_by_name() [struct _GFlagsValue *]
	(GFlagsClass *) flags_class [struct _GFlagsClass *]
	(const gchar *) name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_flags_get_value_by_name(flags_class: *mut _GFlagsClass, name: *const libc::c_char) -> *mut _GFlagsValue;
}


/*
GClosure * g_cclosure_new_object() [struct _GClosure *]
	(GCallback) callback_func [void (*)(void)]
	(GObject *) object [struct _GObject *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_new_object(callback_func: Option<extern fn()>, object: *mut _GObject) -> *mut _GClosure;
}


/*
void g_static_mutex_free()
	(GStaticMutex *) mutex [GStaticMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_mutex_free(mutex: *mut GStaticMutex);
}


/*
gchar * g_utf8_strchr() [char *]
	(const gchar *) p [const char *]
	(gssize) len [long]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_strchr(p: *const libc::c_char, len: libc::c_long, c: libc::c_uint) -> *mut libc::c_char;
}


/*
void g_rw_lock_clear()
	(GRWLock *) rw_lock [struct _GRWLock *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rw_lock_clear(rw_lock: *mut _GRWLock);
}


/*
GDateTime * g_date_time_to_utc() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_to_utc(datetime: *mut _GDateTime) -> *mut _GDateTime;
}


/*
gchar ** g_str_tokenize_and_fold() [char **]
	(const gchar *) string [const char *]
	(const gchar *) translit_locale [const char *]
	(gchar ***) ascii_alternates [char ***]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_str_tokenize_and_fold(string: *const libc::c_char, translit_locale: *const libc::c_char, ascii_alternates: *mut *mut *mut libc::c_char) -> *mut *mut libc::c_char;
}


/*
gchar ** g_key_file_get_string_list() [char **]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_get_string_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut *mut libc::c_char;
}


/*
void g_regex_unref()
	(GRegex *) regex [struct _GRegex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_unref(regex: *mut _GRegex);
}


/*
gboolean g_object_replace_qdata() [int]
	(GObject *) object [struct _GObject *]
	(GQuark) quark [unsigned int]
	(gpointer) oldval [void *]
	(gpointer) newval [void *]
	(GDestroyNotify) destroy [void (*)(void *)]
	(GDestroyNotify *) old_destroy [void (**)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_replace_qdata(object: *mut _GObject, quark: libc::c_uint, oldval: *mut libc::c_void, newval: *mut libc::c_void, destroy: Option<extern fn(*mut libc::c_void)>, old_destroy: *mut Option<extern fn(*mut libc::c_void)>) -> libc::c_int;
}


/*
gchar * g_value_dup_string() [char *]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_dup_string(value: *const _GValue) -> *mut libc::c_char;
}


/*
const gchar *const * g_get_system_config_dirs() [const char *const *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_system_config_dirs() -> *const *const libc::c_char;
}


/*
guint8 g_date_get_days_in_month() [unsigned char]
	(GDateMonth) month [GDateMonth]
	(GDateYear) year [unsigned short]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_get_days_in_month(month: libc::c_uint, year: libc::c_ushort) -> libc::c_uchar;
}


/*
void g_thread_set_priority()
	(GThread *) thread [struct _GThread *]
	(GThreadPriority) priority [GThreadPriority]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_set_priority(thread: *mut _GThread, priority: libc::c_uint);
}


/*
GHashTable * g_hash_table_ref() [struct _GHashTable *]
	(GHashTable *) hash_table [struct _GHashTable *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_ref(hash_table: *mut _GHashTable) -> *mut _GHashTable;
}


/*
gint g_date_time_get_week_of_year() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_get_week_of_year(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
GList * g_list_sort_with_data() [struct _GList *]
	(GList *) list [struct _GList *]
	(GCompareDataFunc) compare_func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_sort_with_data(list: *mut _GList, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void) -> *mut _GList;
}


/*
GBinding * g_object_bind_property() [struct _GBinding *]
	(gpointer) source [void *]
	(const gchar *) source_property [const char *]
	(gpointer) target [void *]
	(const gchar *) target_property [const char *]
	(GBindingFlags) flags [GBindingFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_bind_property(source: *mut libc::c_void, source_property: *const libc::c_char, target: *mut libc::c_void, target_property: *const libc::c_char, flags: libc::c_uint) -> *mut _GBinding;
}


/*
GTypeInstance * g_type_check_instance_cast() [struct _GTypeInstance *]
	(GTypeInstance *) instance [struct _GTypeInstance *]
	(GType) iface_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_check_instance_cast(instance: *mut _GTypeInstance, iface_type: libc::c_ulong) -> *mut _GTypeInstance;
}


/*
GObject * g_binding_get_source() [struct _GObject *]
	(GBinding *) binding [struct _GBinding *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_binding_get_source(binding: *mut _GBinding) -> *mut _GObject;
}


/*
gpointer g_type_class_get_private() [void *]
	(GTypeClass *) klass [struct _GTypeClass *]
	(GType) private_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_class_get_private(klass: *mut _GTypeClass, private_type: libc::c_ulong) -> *mut libc::c_void;
}


/*
void g_variant_get()
	(GVariant *) value [struct _GVariant *]
	(const gchar *) format_string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get(value: *mut _GVariant, format_string: *const libc::c_char);
}


/*
void g_cclosure_marshal_VOID__INTv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_cclosure_marshal_VOID__INTv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
gchar * g_key_file_to_data() [char *]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_to_data(key_file: *mut _GKeyFile, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gboolean g_match_info_matches() [int]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_match_info_matches(match_info: *const _GMatchInfo) -> libc::c_int;
}


/*
gsize g_unichar_fully_decompose() [unsigned long]
	(gunichar) ch [unsigned int]
	(gboolean) compat [int]
	(gunichar *) result [unsigned int *]
	(gsize) result_len [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_fully_decompose(ch: libc::c_uint, compat: libc::c_int, result: *mut libc::c_uint, result_len: libc::c_ulong) -> libc::c_ulong;
}


/*
gchar ** g_variant_dup_objv() [char **]
	(GVariant *) value [struct _GVariant *]
	(gsize *) length [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_dup_objv(value: *mut _GVariant, length: *mut libc::c_ulong) -> *mut *mut libc::c_char;
}


/*
void g_sequence_swap()
	(GSequenceIter *) a [struct _GSequenceNode *]
	(GSequenceIter *) b [struct _GSequenceNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_swap(a: *mut _GSequenceNode, b: *mut _GSequenceNode);
}


/*
gdouble g_timer_elapsed() [double]
	(GTimer *) timer [struct _GTimer *]
	(gulong *) microseconds [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_timer_elapsed(timer: *mut _GTimer, microseconds: *mut libc::c_ulong) -> libc::c_double;
}


/*
void g_option_group_set_translate_func()
	(GOptionGroup *) group [struct _GOptionGroup *]
	(GTranslateFunc) func [const char *(*)(const char *, void *)]
	(gpointer) data [void *]
	(GDestroyNotify) destroy_notify [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_group_set_translate_func(group: *mut _GOptionGroup, func: Option<extern fn(*const libc::c_char, *mut libc::c_void) -> *const libc::c_char>, data: *mut libc::c_void, destroy_notify: Option<extern fn(*mut libc::c_void)>);
}


/*
gint64 g_slice_get_config() [long]
	(GSliceConfig) ckey [GSliceConfig]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slice_get_config(ckey: libc::c_uint) -> libc::c_long;
}


/*
gint64 g_key_file_get_int64() [long]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_get_int64(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> libc::c_long;
}


/*
void g_thread_pool_free()
	(GThreadPool *) pool [struct _GThreadPool *]
	(gboolean) immediate [int]
	(gboolean) wait_ [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_pool_free(pool: *mut _GThreadPool, immediate: libc::c_int, wait_: libc::c_int);
}


/*
void g_thread_pool_set_sort_function()
	(GThreadPool *) pool [struct _GThreadPool *]
	(GCompareDataFunc) func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_pool_set_sort_function(pool: *mut _GThreadPool, func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
void g_tree_insert()
	(GTree *) tree [struct _GTree *]
	(gpointer) key [void *]
	(gpointer) value [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_tree_insert(tree: *mut _GTree, key: *mut libc::c_void, value: *mut libc::c_void);
}


/*
void g_async_queue_unlock()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_unlock(queue: *mut _GAsyncQueue);
}


/*
void g_type_default_interface_unref()
	(gpointer) g_iface [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_default_interface_unref(g_iface: *mut libc::c_void);
}


/*
GRand * g_rand_new_with_seed() [struct _GRand *]
	(guint32) seed [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rand_new_with_seed(seed: libc::c_uint) -> *mut _GRand;
}


/*
void g_slice_set_config()
	(GSliceConfig) ckey [GSliceConfig]
	(gint64) value [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slice_set_config(ckey: libc::c_uint, value: libc::c_long);
}


/*
gchar * g_locale_to_utf8() [char *]
	(const gchar *) opsysstring [const char *]
	(gssize) len [long]
	(gsize *) bytes_read [unsigned long *]
	(gsize *) bytes_written [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_locale_to_utf8(opsysstring: *const libc::c_char, len: libc::c_long, bytes_read: *mut libc::c_ulong, bytes_written: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
void g_async_queue_push_sorted()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(gpointer) data [void *]
	(GCompareDataFunc) func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_push_sorted(queue: *mut _GAsyncQueue, data: *mut libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
void g_node_pop_allocator()
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_pop_allocator();
}


/*
GString * g_string_overwrite() [struct _GString *]
	(GString *) string [struct _GString *]
	(gsize) pos [unsigned long]
	(const gchar *) val [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_overwrite(string: *mut _GString, pos: libc::c_ulong, val: *const libc::c_char) -> *mut _GString;
}


/*
void g_markup_parse_context_push()
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
	(const GMarkupParser *) parser [const struct _GMarkupParser *]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_markup_parse_context_push(context: *mut _GMarkupParseContext, parser: *const _GMarkupParser, user_data: *mut libc::c_void);
}


/*
void g_date_clamp()
	(GDate *) date [struct _GDate *]
	(const GDate *) min_date [const struct _GDate *]
	(const GDate *) max_date [const struct _GDate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_clamp(date: *mut _GDate, min_date: *const _GDate, max_date: *const _GDate);
}


/*
gchar * g_filename_from_uri() [char *]
	(const gchar *) uri [const char *]
	(gchar **) hostname [char **]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_filename_from_uri(uri: *const libc::c_char, hostname: *mut *mut libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gpointer g_type_default_interface_ref() [void *]
	(GType) g_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_default_interface_ref(g_type: libc::c_ulong) -> *mut libc::c_void;
}


/*
GMainContext * g_source_get_context() [struct _GMainContext *]
	(GSource *) source [struct _GSource *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_get_context(source: *mut _GSource) -> *mut _GMainContext;
}


/*
GHook * g_hook_find_data() [struct _GHook *]
	(GHookList *) hook_list [struct _GHookList *]
	(gboolean) need_valids [int]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_find_data(hook_list: *mut _GHookList, need_valids: libc::c_int, data: *mut libc::c_void) -> *mut _GHook;
}


/*
void g_completion_free()
	(GCompletion *) cmp [struct _GCompletion *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_completion_free(cmp: *mut _GCompletion);
}


/*
void g_byte_array_unref()
	(GByteArray *) array [struct _GByteArray *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_byte_array_unref(array: *mut _GByteArray);
}


/*
GVariant * g_variant_take_ref() [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_take_ref(value: *mut _GVariant) -> *mut _GVariant;
}


/*
glong g_utf8_pointer_to_offset() [long]
	(const gchar *) str [const char *]
	(const gchar *) pos [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_pointer_to_offset(str: *const libc::c_char, pos: *const libc::c_char) -> libc::c_long;
}


/*
char * g_uri_unescape_string()
	(const char *) escaped_string
	(const char *) illegal_characters
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_uri_unescape_string(escaped_string: *const libc::c_char, illegal_characters: *const libc::c_char) -> *mut libc::c_char;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_spawn_sync(working_directory: *const libc::c_char, argv: *mut *mut libc::c_char, envp: *mut *mut libc::c_char, flags: libc::c_uint, child_setup: Option<extern fn(*mut libc::c_void)>, user_data: *mut libc::c_void, standard_output: *mut *mut libc::c_char, standard_error: *mut *mut libc::c_char, exit_status: *mut libc::c_int, error: *mut *mut _GError) -> libc::c_int;
}


/*
gint g_queue_index() [int]
	(GQueue *) queue [struct _GQueue *]
	(gconstpointer) data [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_index(queue: *mut _GQueue, data: *const libc::c_void) -> libc::c_int;
}


/*
void g_type_add_interface_check()
	(gpointer) check_data [void *]
	(GTypeInterfaceCheckFunc) check_func [void (*)(void *, void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_add_interface_check(check_data: *mut libc::c_void, check_func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>);
}


/*
void g_test_message()
	(const char *) format
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_message(format: *const libc::c_char);
}


/*
void g_completion_add_items()
	(GCompletion *) cmp [struct _GCompletion *]
	(GList *) items [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_completion_add_items(cmp: *mut _GCompletion, items: *mut _GList);
}


/*
GEnumValue * g_enum_get_value() [struct _GEnumValue *]
	(GEnumClass *) enum_class [struct _GEnumClass *]
	(gint) value [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_enum_get_value(enum_class: *mut _GEnumClass, value: libc::c_int) -> *mut _GEnumValue;
}


/*
void g_clear_error()
	(GError **) err [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_clear_error(err: *mut *mut _GError);
}


/*
GSequenceIter * g_sequence_get_end_iter() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_get_end_iter(seq: *mut _GSequence) -> *mut _GSequenceNode;
}


/*
gchar g_ascii_tolower() [char]
	(gchar) c [char]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ascii_tolower(c: libc::c_char) -> libc::c_char;
}


/*
const gchar * g_variant_get_string() [const char *]
	(GVariant *) value [struct _GVariant *]
	(gsize *) length [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_string(value: *mut _GVariant, length: *mut libc::c_ulong) -> *const libc::c_char;
}


/*
gchar ** g_get_locale_variants() [char **]
	(const gchar *) locale [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_locale_variants(locale: *const libc::c_char) -> *mut *mut libc::c_char;
}


/*
GQuark g_convert_error_quark() [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_convert_error_quark() -> libc::c_uint;
}


/*
GByteArray * g_byte_array_remove_index_fast() [struct _GByteArray *]
	(GByteArray *) array [struct _GByteArray *]
	(guint) index_ [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_byte_array_remove_index_fast(array: *mut _GByteArray, index_: libc::c_uint) -> *mut _GByteArray;
}


/*
void g_type_remove_interface_check()
	(gpointer) check_data [void *]
	(GTypeInterfaceCheckFunc) check_func [void (*)(void *, void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_remove_interface_check(check_data: *mut libc::c_void, check_func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>);
}


/*
const GVariantType * g_variant_type_next() [const struct _GVariantType *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_next(type_: *const _GVariantType) -> *const _GVariantType;
}


/*
void g_static_rw_lock_init()
	(GStaticRWLock *) lock [struct _GStaticRWLock *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_rw_lock_init(lock: *mut _GStaticRWLock);
}


/*
GValueArray * g_value_array_sort() [struct _GValueArray *]
	(GValueArray *) value_array [struct _GValueArray *]
	(GCompareFunc) compare_func [int (*)(const void *, const void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_array_sort(value_array: *mut _GValueArray, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GValueArray;
}


/*
gboolean g_io_channel_get_buffered() [int]
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_get_buffered(channel: *mut _GIOChannel) -> libc::c_int;
}


/*
void g_object_unref()
	(gpointer) object [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_unref(object: *mut libc::c_void);
}


/*
GDateDay g_date_get_day() [unsigned char]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_get_day(date: *const _GDate) -> libc::c_uchar;
}


/*
void g_value_take_object()
	(GValue *) value [struct _GValue *]
	(gpointer) v_object [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_take_object(value: *mut _GValue, v_object: *mut libc::c_void);
}


/*
gint g_main_depth() [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_depth() -> libc::c_int;
}


/*
gboolean g_signal_accumulator_true_handled() [int]
	(GSignalInvocationHint *) ihint [struct _GSignalInvocationHint *]
	(GValue *) return_accu [struct _GValue *]
	(const GValue *) handler_return [const struct _GValue *]
	(gpointer) dummy [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_accumulator_true_handled(ihint: *mut _GSignalInvocationHint, return_accu: *mut _GValue, handler_return: *const _GValue, dummy: *mut libc::c_void) -> libc::c_int;
}


/*
GType g_type_module_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_module_get_type() -> libc::c_ulong;
}


/*
gsize g_atomic_pointer_xor() [unsigned long]
	(volatile void *) atomic
	(gsize) val [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_atomic_pointer_xor(atomic: *mut libc::c_void, val: libc::c_ulong) -> libc::c_ulong;
}


/*
GString * g_string_sized_new() [struct _GString *]
	(gsize) dfl_size [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_sized_new(dfl_size: libc::c_ulong) -> *mut _GString;
}


/*
gboolean g_variant_is_signature() [int]
	(const gchar *) string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_is_signature(string: *const libc::c_char) -> libc::c_int;
}


/*
gchar * g_filename_display_name() [char *]
	(const gchar *) filename [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_filename_display_name(filename: *const libc::c_char) -> *mut libc::c_char;
}


/*
void g_slist_foreach()
	(GSList *) list [struct _GSList *]
	(GFunc) func [void (*)(void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_foreach(list: *mut _GSList, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
GDir * g_dir_open() [struct _GDir *]
	(const gchar *) path [const char *]
	(guint) flags [unsigned int]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_dir_open(path: *const libc::c_char, flags: libc::c_uint, error: *mut *mut _GError) -> *mut _GDir;
}


/*
void g_hmac_unref()
	(GHmac *) hmac [struct _GHmac *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hmac_unref(hmac: *mut _GHmac);
}


/*
guint g_array_get_element_size() [unsigned int]
	(GArray *) array [struct _GArray *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_array_get_element_size(array: *mut _GArray) -> libc::c_uint;
}


/*
void g_test_bug_base()
	(const char *) uri_pattern
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_bug_base(uri_pattern: *const libc::c_char);
}


/*
void g_date_time_unref()
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_unref(datetime: *mut _GDateTime);
}


/*
const gchar * g_strip_context() [const char *]
	(const gchar *) msgid [const char *]
	(const gchar *) msgval [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strip_context(msgid: *const libc::c_char, msgval: *const libc::c_char) -> *const libc::c_char;
}


/*
void g_object_notify()
	(GObject *) object [struct _GObject *]
	(const gchar *) property_name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_notify(object: *mut _GObject, property_name: *const libc::c_char);
}


/*
gint g_date_time_get_minute() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_get_minute(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
void g_queue_init()
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_init(queue: *mut _GQueue);
}


/*
gchar * g_match_info_expand_references() [char *]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
	(const gchar *) string_to_expand [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_match_info_expand_references(match_info: *const _GMatchInfo, string_to_expand: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
GParamSpec * g_param_spec_char() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(gint8) minimum [signed char]
	(gint8) maximum [signed char]
	(gint8) default_value [signed char]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_char(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, minimum: libc::c_char, maximum: libc::c_char, default_value: libc::c_char, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
GString * g_string_truncate() [struct _GString *]
	(GString *) string [struct _GString *]
	(gsize) len [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_truncate(string: *mut _GString, len: libc::c_ulong) -> *mut _GString;
}


/*
GString * g_string_up() [struct _GString *]
	(GString *) string [struct _GString *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_up(string: *mut _GString) -> *mut _GString;
}


/*
void g_main_context_set_poll_func()
	(GMainContext *) context [struct _GMainContext *]
	(GPollFunc) func [int (*)(struct _GPollFD *, unsigned int, int)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_set_poll_func(context: *mut _GMainContext, func: Option<extern fn(*mut _GPollFD, libc::c_uint, libc::c_int) -> libc::c_int>);
}


/*
gboolean g_error_matches() [int]
	(const GError *) error [const struct _GError *]
	(GQuark) domain [unsigned int]
	(gint) code [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_error_matches(error: *const _GError, domain: libc::c_uint, code: libc::c_int) -> libc::c_int;
}


/*
gpointer g_realloc() [void *]
	(gpointer) mem [void *]
	(gsize) n_bytes [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_realloc(mem: *mut libc::c_void, n_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
void g_source_add_child_source()
	(GSource *) source [struct _GSource *]
	(GSource *) child_source [struct _GSource *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_add_child_source(source: *mut _GSource, child_source: *mut _GSource);
}


/*
gpointer g_queue_pop_tail() [void *]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_pop_tail(queue: *mut _GQueue) -> *mut libc::c_void;
}


/*
GParamSpec * g_value_dup_param() [struct _GParamSpec *]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_dup_param(value: *const _GValue) -> *mut _GParamSpec;
}


/*
GArray * g_array_sized_new() [struct _GArray *]
	(gboolean) zero_terminated [int]
	(gboolean) clear_ [int]
	(guint) element_size [unsigned int]
	(guint) reserved_size [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_array_sized_new(zero_terminated: libc::c_int, clear_: libc::c_int, element_size: libc::c_uint, reserved_size: libc::c_uint) -> *mut _GArray;
}


/*
gchar * g_utf8_strncpy() [char *]
	(gchar *) dest [char *]
	(const gchar *) src [const char *]
	(gsize) n [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_strncpy(dest: *mut libc::c_char, src: *const libc::c_char, n: libc::c_ulong) -> *mut libc::c_char;
}


/*
gdouble g_date_time_get_seconds() [double]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_get_seconds(datetime: *mut _GDateTime) -> libc::c_double;
}


/*
void g_signal_handlers_destroy()
	(gpointer) instance [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_handlers_destroy(instance: *mut libc::c_void);
}


/*
gboolean g_variant_is_of_type() [int]
	(GVariant *) value [struct _GVariant *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_is_of_type(value: *mut _GVariant, type_: *const _GVariantType) -> libc::c_int;
}


/*
GType g_date_time_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_get_type() -> libc::c_ulong;
}


/*
void g_io_channel_init()
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_init(channel: *mut _GIOChannel);
}


/*
gboolean g_unichar_compose() [int]
	(gunichar) a [unsigned int]
	(gunichar) b [unsigned int]
	(gunichar *) ch [unsigned int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_compose(a: libc::c_uint, b: libc::c_uint, ch: *mut libc::c_uint) -> libc::c_int;
}


/*
GValueArray * g_value_array_append() [struct _GValueArray *]
	(GValueArray *) value_array [struct _GValueArray *]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_array_append(value_array: *mut _GValueArray, value: *const _GValue) -> *mut _GValueArray;
}


/*
gint g_io_channel_unix_get_fd() [int]
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_unix_get_fd(channel: *mut _GIOChannel) -> libc::c_int;
}


/*
gchar * g_ascii_strup() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ascii_strup(str: *const libc::c_char, len: libc::c_long) -> *mut libc::c_char;
}


/*
guint g_source_attach() [unsigned int]
	(GSource *) source [struct _GSource *]
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_attach(source: *mut _GSource, context: *mut _GMainContext) -> libc::c_uint;
}


/*
void g_value_set_boolean()
	(GValue *) value [struct _GValue *]
	(gboolean) v_boolean [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_boolean(value: *mut _GValue, v_boolean: libc::c_int);
}


/*
void g_value_set_string_take_ownership()
	(GValue *) value [struct _GValue *]
	(gchar *) v_string [char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_string_take_ownership(value: *mut _GValue, v_string: *mut libc::c_char);
}


/*
GHook * g_hook_find_func_data() [struct _GHook *]
	(GHookList *) hook_list [struct _GHookList *]
	(gboolean) need_valids [int]
	(gpointer) func [void *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_find_func_data(hook_list: *mut _GHookList, need_valids: libc::c_int, func: *mut libc::c_void, data: *mut libc::c_void) -> *mut _GHook;
}


/*
GAsyncQueue * g_async_queue_new() [struct _GAsyncQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_new() -> *mut _GAsyncQueue;
}


/*
void g_type_interface_add_prerequisite()
	(GType) interface_type [unsigned long]
	(GType) prerequisite_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_interface_add_prerequisite(interface_type: libc::c_ulong, prerequisite_type: libc::c_ulong);
}


/*
GQuark g_spawn_error_quark() [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_spawn_error_quark() -> libc::c_uint;
}


/*
gchar ** g_uri_list_extract_uris() [char **]
	(const gchar *) uri_list [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_uri_list_extract_uris(uri_list: *const libc::c_char) -> *mut *mut libc::c_char;
}


/*
GBytes * g_bytes_new_with_free_func() [struct _GBytes *]
	(gconstpointer) data [const void *]
	(gsize) size [unsigned long]
	(GDestroyNotify) free_func [void (*)(void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bytes_new_with_free_func(data: *const libc::c_void, size: libc::c_ulong, free_func: Option<extern fn(*mut libc::c_void)>, user_data: *mut libc::c_void) -> *mut _GBytes;
}


/*
gboolean g_source_remove_by_user_data() [int]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_remove_by_user_data(user_data: *mut libc::c_void) -> libc::c_int;
}


/*
gint g_variant_compare() [int]
	(gconstpointer) one [const void *]
	(gconstpointer) two [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_compare(one: *const libc::c_void, two: *const libc::c_void) -> libc::c_int;
}


/*
GType g_source_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_get_type() -> libc::c_ulong;
}


/*
char * g_uri_escape_string()
	(const char *) unescaped
	(const char *) reserved_chars_allowed
	(gboolean) allow_utf8 [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_uri_escape_string(unescaped: *const libc::c_char, reserved_chars_allowed: *const libc::c_char, allow_utf8: libc::c_int) -> *mut libc::c_char;
}


/*
gchar * g_string_chunk_insert_const() [char *]
	(GStringChunk *) chunk [struct _GStringChunk *]
	(const gchar *) string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_chunk_insert_const(chunk: *mut _GStringChunk, string: *const libc::c_char) -> *mut libc::c_char;
}


/*
GDateTime * g_date_time_add_minutes() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
	(gint) minutes [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_add_minutes(datetime: *mut _GDateTime, minutes: libc::c_int) -> *mut _GDateTime;
}


/*
gboolean g_hostname_is_ascii_encoded() [int]
	(const gchar *) hostname [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hostname_is_ascii_encoded(hostname: *const libc::c_char) -> libc::c_int;
}


/*
gint g_hook_compare_ids() [int]
	(GHook *) new_hook [struct _GHook *]
	(GHook *) sibling [struct _GHook *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_compare_ids(new_hook: *mut _GHook, sibling: *mut _GHook) -> libc::c_int;
}


/*
void g_test_maximized_result()
	(double) maximized_quantity
	(const char *) format
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_maximized_result(maximized_quantity: libc::c_double, format: *const libc::c_char);
}


/*
gunichar * g_unicode_canonical_decomposition() [unsigned int *]
	(gunichar) ch [unsigned int]
	(gsize *) result_len [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unicode_canonical_decomposition(ch: libc::c_uint, result_len: *mut libc::c_ulong) -> *mut libc::c_uint;
}


/*
GNode * g_node_get_root() [struct _GNode *]
	(GNode *) node [struct _GNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_get_root(node: *mut _GNode) -> *mut _GNode;
}


/*
guint32 g_rand_int() [unsigned int]
	(GRand *) rand_ [struct _GRand *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rand_int(rand_: *mut _GRand) -> libc::c_uint;
}


/*
void g_value_copy()
	(const GValue *) src_value [const struct _GValue *]
	(GValue *) dest_value [struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_copy(src_value: *const _GValue, dest_value: *mut _GValue);
}


/*
void g_private_replace()
	(GPrivate *) key [struct _GPrivate *]
	(gpointer) value [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_private_replace(key: *mut _GPrivate, value: *mut libc::c_void);
}


/*
void g_hash_table_foreach()
	(GHashTable *) hash_table [struct _GHashTable *]
	(GHFunc) func [void (*)(void *, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_foreach(hash_table: *mut _GHashTable, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
gsize g_variant_type_n_items() [unsigned long]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_n_items(type_: *const _GVariantType) -> libc::c_ulong;
}


/*
GSequenceIter * g_sequence_append() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_append(seq: *mut _GSequence, data: *mut libc::c_void) -> *mut _GSequenceNode;
}


/*
void g_nullify_pointer()
	(gpointer *) nullify_location [void **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_nullify_pointer(nullify_location: *mut *mut libc::c_void);
}


/*
void g_test_init()
	(int *) argc
	(char ***) argv
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_init(argc: *mut libc::c_int, argv: *mut *mut *mut libc::c_char);
}


/*
void g_key_file_set_locale_string()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(const gchar *) locale [const char *]
	(const gchar *) string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_set_locale_string(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, locale: *const libc::c_char, string: *const libc::c_char);
}


/*
void g_bookmark_file_free()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_free(bookmark: *mut _GBookmarkFile);
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_match_full(regex: *const _GRegex, string: *const libc::c_char, string_len: libc::c_long, start_position: libc::c_int, match_options: libc::c_uint, match_info: *mut *mut _GMatchInfo, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_key_file_set_string_list()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(const gchar *const []) list [const char *const[]]
	(gsize) length [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_set_string_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, list: *mut *const libc::c_char /* INCOMPLETEARRAY */, length: libc::c_ulong);
}


/*
GParamSpec * g_param_spec_override() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(GParamSpec *) overridden [struct _GParamSpec *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_override(name: *const libc::c_char, overridden: *mut _GParamSpec) -> *mut _GParamSpec;
}


/*
GTypeValueTable * g_type_value_table_peek() [struct _GTypeValueTable *]
	(GType) type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_value_table_peek(type_: libc::c_ulong) -> *mut _GTypeValueTable;
}


/*
void g_value_set_uint()
	(GValue *) value [struct _GValue *]
	(guint) v_uint [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_uint(value: *mut _GValue, v_uint: libc::c_uint);
}


/*
GDate * g_date_new_dmy() [struct _GDate *]
	(GDateDay) day [unsigned char]
	(GDateMonth) month [GDateMonth]
	(GDateYear) year [unsigned short]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_new_dmy(day: libc::c_uchar, month: libc::c_uint, year: libc::c_ushort) -> *mut _GDate;
}


/*
void g_signal_emitv()
	(const GValue *) instance_and_params [const struct _GValue *]
	(guint) signal_id [unsigned int]
	(GQuark) detail [unsigned int]
	(GValue *) return_value [struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_emitv(instance_and_params: *const _GValue, signal_id: libc::c_uint, detail: libc::c_uint, return_value: *mut _GValue);
}


/*
guint * g_signal_list_ids() [unsigned int *]
	(GType) itype [unsigned long]
	(guint *) n_ids [unsigned int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_list_ids(itype: libc::c_ulong, n_ids: *mut libc::c_uint) -> *mut libc::c_uint;
}


/*
gpointer g_boxed_copy() [void *]
	(GType) boxed_type [unsigned long]
	(gconstpointer) src_boxed [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_boxed_copy(boxed_type: libc::c_ulong, src_boxed: *const libc::c_void) -> *mut libc::c_void;
}


/*
void g_pattern_spec_free()
	(GPatternSpec *) pspec [struct _GPatternSpec *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_pattern_spec_free(pspec: *mut _GPatternSpec);
}


/*
void g_string_append_printf()
	(GString *) string [struct _GString *]
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_append_printf(string: *mut _GString, format: *const libc::c_char);
}


/*
void g_once_init_leave()
	(volatile void *) location
	(gsize) result [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_once_init_leave(location: *mut libc::c_void, result: libc::c_ulong);
}


/*
GSList * g_slist_delete_link() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(GSList *) link_ [struct _GSList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_delete_link(list: *mut _GSList, link_: *mut _GSList) -> *mut _GSList;
}


/*
void g_value_set_variant()
	(GValue *) value [struct _GValue *]
	(GVariant *) variant [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_variant(value: *mut _GValue, variant: *mut _GVariant);
}


/*
void g_critical()
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_critical(format: *const libc::c_char);
}


/*
gchar ** g_strsplit() [char **]
	(const gchar *) string [const char *]
	(const gchar *) delimiter [const char *]
	(gint) max_tokens [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strsplit(string: *const libc::c_char, delimiter: *const libc::c_char, max_tokens: libc::c_int) -> *mut *mut libc::c_char;
}


/*
gboolean g_variant_iter_next() [int]
	(GVariantIter *) iter [struct _GVariantIter *]
	(const gchar *) format_string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_iter_next(iter: *mut _GVariantIter, format_string: *const libc::c_char) -> libc::c_int;
}


/*
GVariant * g_variant_new_double() [struct _GVariant *]
	(gdouble) value [double]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_double(value: libc::c_double) -> *mut _GVariant;
}


/*
GParamSpec * g_param_spec_param() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(GType) param_type [unsigned long]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_param(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, param_type: libc::c_ulong, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
void g_list_free_1()
	(GList *) list [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_free_1(list: *mut _GList);
}


/*
gboolean g_variant_type_is_tuple() [int]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_is_tuple(type_: *const _GVariantType) -> libc::c_int;
}


/*
gint g_async_queue_length() [int]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_length(queue: *mut _GAsyncQueue) -> libc::c_int;
}


/*
gint64 g_ascii_strtoll() [long]
	(const gchar *) nptr [const char *]
	(gchar **) endptr [char **]
	(guint) base [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ascii_strtoll(nptr: *const libc::c_char, endptr: *mut *mut libc::c_char, base: libc::c_uint) -> libc::c_long;
}


/*
gboolean g_pattern_spec_equal() [int]
	(GPatternSpec *) pspec1 [struct _GPatternSpec *]
	(GPatternSpec *) pspec2 [struct _GPatternSpec *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_pattern_spec_equal(pspec1: *mut _GPatternSpec, pspec2: *mut _GPatternSpec) -> libc::c_int;
}


/*
gchar * g_compute_checksum_for_bytes() [char *]
	(GChecksumType) checksum_type [GChecksumType]
	(GBytes *) data [struct _GBytes *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_compute_checksum_for_bytes(checksum_type: libc::c_uint, data: *mut _GBytes) -> *mut libc::c_char;
}


/*
gpointer g_markup_parse_context_pop() [void *]
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_markup_parse_context_pop(context: *mut _GMarkupParseContext) -> *mut libc::c_void;
}


/*
const gchar * g_option_context_get_summary() [const char *]
	(GOptionContext *) context [struct _GOptionContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_context_get_summary(context: *mut _GOptionContext) -> *const libc::c_char;
}


/*
void g_trash_stack_push()
	(GTrashStack **) stack_p [struct _GTrashStack **]
	(gpointer) data_p [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_trash_stack_push(stack_p: *mut *mut _GTrashStack, data_p: *mut libc::c_void);
}


/*
void g_object_class_override_property()
	(GObjectClass *) oclass [struct _GObjectClass *]
	(guint) property_id [unsigned int]
	(const gchar *) name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_class_override_property(oclass: *mut _GObjectClass, property_id: libc::c_uint, name: *const libc::c_char);
}


/*
gboolean g_unichar_isalpha() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_isalpha(c: libc::c_uint) -> libc::c_int;
}


/*
void g_queue_unlink()
	(GQueue *) queue [struct _GQueue *]
	(GList *) link_ [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_unlink(queue: *mut _GQueue, link_: *mut _GList);
}


/*
GVariant * g_variant_new_strv() [struct _GVariant *]
	(const gchar *const *) strv [const char *const *]
	(gssize) length [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_strv(strv: *const *const libc::c_char, length: libc::c_long) -> *mut _GVariant;
}


/*
void g_source_destroy()
	(GSource *) source [struct _GSource *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_destroy(source: *mut _GSource);
}


/*
gboolean g_bookmark_file_get_is_private() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_get_is_private(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_source_set_closure()
	(GSource *) source [struct _GSource *]
	(GClosure *) closure [struct _GClosure *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_set_closure(source: *mut _GSource, closure: *mut _GClosure);
}


/*
gpointer g_type_interface_peek_parent() [void *]
	(gpointer) g_iface [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_interface_peek_parent(g_iface: *mut libc::c_void) -> *mut libc::c_void;
}


/*
const gchar * g_quark_to_string() [const char *]
	(GQuark) quark [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_quark_to_string(quark: libc::c_uint) -> *const libc::c_char;
}


/*
void g_cclosure_marshal_VOID__STRINGv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_cclosure_marshal_VOID__STRINGv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
const GVariantType * g_variant_type_value() [const struct _GVariantType *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_value(type_: *const _GVariantType) -> *const _GVariantType;
}


/*
GCompletion * g_completion_new() [struct _GCompletion *]
	(GCompletionFunc) func [char *(*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_completion_new(func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_char>) -> *mut _GCompletion;
}


/*
GIOStatus g_io_channel_set_encoding() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(const gchar *) encoding [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_set_encoding(channel: *mut _GIOChannel, encoding: *const libc::c_char, error: *mut *mut _GError) -> libc::c_uint;
}


/*
gchar * g_mapped_file_get_contents() [char *]
	(GMappedFile *) file [struct _GMappedFile *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mapped_file_get_contents(file: *mut _GMappedFile) -> *mut libc::c_char;
}


/*
GTypeInstance * g_type_create_instance() [struct _GTypeInstance *]
	(GType) type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_create_instance(type_: libc::c_ulong) -> *mut _GTypeInstance;
}


/*
void g_qsort_with_data()
	(gconstpointer) pbase [const void *]
	(gint) total_elems [int]
	(gsize) size [unsigned long]
	(GCompareDataFunc) compare_func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_qsort_with_data(pbase: *const libc::c_void, total_elems: libc::c_int, size: libc::c_ulong, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
GVariant * g_variant_builder_end() [struct _GVariant *]
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_builder_end(builder: *mut _GVariantBuilder) -> *mut _GVariant;
}


/*
void g_static_rec_mutex_lock_full()
	(GStaticRecMutex *) mutex [struct _GStaticRecMutex *]
	(guint) depth [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_rec_mutex_lock_full(mutex: *mut _GStaticRecMutex, depth: libc::c_uint);
}


/*
void g_free()
	(gpointer) mem [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_free(mem: *mut libc::c_void);
}


/*
gboolean g_value_type_transformable() [int]
	(GType) src_type [unsigned long]
	(GType) dest_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_type_transformable(src_type: libc::c_ulong, dest_type: libc::c_ulong) -> libc::c_int;
}


/*
GMainContext * g_main_context_ref() [struct _GMainContext *]
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_ref(context: *mut _GMainContext) -> *mut _GMainContext;
}


/*
void g_set_application_name()
	(const gchar *) application_name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_set_application_name(application_name: *const libc::c_char);
}


/*
void g_test_timer_start()
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_timer_start();
}


/*
void g_closure_sink()
	(GClosure *) closure [struct _GClosure *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_closure_sink(closure: *mut _GClosure);
}


/*
void g_dir_rewind()
	(GDir *) dir [struct _GDir *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_dir_rewind(dir: *mut _GDir);
}


/*
guint g_signal_handlers_unblock_matched() [unsigned int]
	(gpointer) instance [void *]
	(GSignalMatchType) mask [GSignalMatchType]
	(guint) signal_id [unsigned int]
	(GQuark) detail [unsigned int]
	(GClosure *) closure [struct _GClosure *]
	(gpointer) func [void *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_handlers_unblock_matched(instance: *mut libc::c_void, mask: libc::c_uint, signal_id: libc::c_uint, detail: libc::c_uint, closure: *mut _GClosure, func: *mut libc::c_void, data: *mut libc::c_void) -> libc::c_uint;
}


/*
gboolean g_key_file_has_key() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_has_key(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_list_free_full()
	(GList *) list [struct _GList *]
	(GDestroyNotify) free_func [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_free_full(list: *mut _GList, free_func: Option<extern fn(*mut libc::c_void)>);
}


/*
GAllocator * g_allocator_new() [struct _GAllocator *]
	(const gchar *) name [const char *]
	(guint) n_preallocs [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_allocator_new(name: *const libc::c_char, n_preallocs: libc::c_uint) -> *mut _GAllocator;
}


/*
gboolean g_markup_parse_context_parse() [int]
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
	(const gchar *) text [const char *]
	(gssize) text_len [long]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_markup_parse_context_parse(context: *mut _GMarkupParseContext, text: *const libc::c_char, text_len: libc::c_long, error: *mut *mut _GError) -> libc::c_int;
}


/*
GNode * g_node_copy_deep() [struct _GNode *]
	(GNode *) node [struct _GNode *]
	(GCopyFunc) copy_func [void *(*)(const void *, void *)]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_copy_deep(node: *mut _GNode, copy_func: Option<extern fn(*const libc::c_void, *mut libc::c_void) -> *mut libc::c_void>, data: *mut libc::c_void) -> *mut _GNode;
}


/*
gchar * g_file_read_link() [char *]
	(const gchar *) filename [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_file_read_link(filename: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
guint g_signal_lookup() [unsigned int]
	(const gchar *) name [const char *]
	(GType) itype [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_lookup(name: *const libc::c_char, itype: libc::c_ulong) -> libc::c_uint;
}


/*
gsize g_strlcat() [unsigned long]
	(gchar *) dest [char *]
	(const gchar *) src [const char *]
	(gsize) dest_size [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strlcat(dest: *mut libc::c_char, src: *const libc::c_char, dest_size: libc::c_ulong) -> libc::c_ulong;
}


/*
void g_async_queue_unref_and_unlock()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_unref_and_unlock(queue: *mut _GAsyncQueue);
}


/*
gboolean g_hash_table_contains() [int]
	(GHashTable *) hash_table [struct _GHashTable *]
	(gconstpointer) key [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_contains(hash_table: *mut _GHashTable, key: *const libc::c_void) -> libc::c_int;
}


/*
GList * g_list_copy() [struct _GList *]
	(GList *) list [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_copy(list: *mut _GList) -> *mut _GList;
}


/*
gchar g_value_get_char() [char]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_char(value: *const _GValue) -> libc::c_char;
}


/*
gboolean g_date_valid_month() [int]
	(GDateMonth) month [GDateMonth]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_valid_month(month: libc::c_uint) -> libc::c_int;
}


/*
gsize g_base64_decode_step() [unsigned long]
	(const gchar *) in [const char *]
	(gsize) len [unsigned long]
	(guchar *) out [unsigned char *]
	(gint *) state [int *]
	(guint *) save [unsigned int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_base64_decode_step(in_: *const libc::c_char, len: libc::c_ulong, out: *mut libc::c_uchar, state: *mut libc::c_int, save: *mut libc::c_uint) -> libc::c_ulong;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_convert(str: *const libc::c_char, len: libc::c_long, to_codeset: *const libc::c_char, from_codeset: *const libc::c_char, bytes_read: *mut libc::c_ulong, bytes_written: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
guint g_scanner_set_scope() [unsigned int]
	(GScanner *) scanner [struct _GScanner *]
	(guint) scope_id [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_set_scope(scanner: *mut _GScanner, scope_id: libc::c_uint) -> libc::c_uint;
}


/*
void g_main_context_add_poll()
	(GMainContext *) context [struct _GMainContext *]
	(GPollFD *) fd [struct _GPollFD *]
	(gint) priority [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_add_poll(context: *mut _GMainContext, fd: *mut _GPollFD, priority: libc::c_int);
}


/*
int g_test_run_suite()
	(GTestSuite *) suite [struct GTestSuite *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_run_suite(suite: *mut GTestSuite) -> libc::c_int;
}


/*
void g_clear_object()
	(volatile GObject **) object_ptr [volatile struct _GObject **]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_clear_object(object_ptr: *mut *mut volatile struct _GObject);
}


/*
gpointer g_tuples_index() [void *]
	(GTuples *) tuples [struct _GTuples *]
	(gint) index_ [int]
	(gint) field [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_tuples_index(tuples: *mut _GTuples, index_: libc::c_int, field: libc::c_int) -> *mut libc::c_void;
}


/*
GQuark g_quark_from_string() [unsigned int]
	(const gchar *) string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_quark_from_string(string: *const libc::c_char) -> libc::c_uint;
}


/*
void g_variant_builder_close()
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_builder_close(builder: *mut _GVariantBuilder);
}


/*
void g_signal_emit_by_name()
	(gpointer) instance [void *]
	(const gchar *) detailed_signal [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_emit_by_name(instance: *mut libc::c_void, detailed_signal: *const libc::c_char);
}


/*
gpointer g_datalist_id_remove_no_notify() [void *]
	(GData **) datalist [struct _GData **]
	(GQuark) key_id [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_datalist_id_remove_no_notify(datalist: *mut *mut _GData, key_id: libc::c_uint) -> *mut libc::c_void;
}


/*
GClosure * g_signal_type_cclosure_new() [struct _GClosure *]
	(GType) itype [unsigned long]
	(guint) struct_offset [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_type_cclosure_new(itype: libc::c_ulong, struct_offset: libc::c_uint) -> *mut _GClosure;
}


/*
gboolean g_str_equal() [int]
	(gconstpointer) v1 [const void *]
	(gconstpointer) v2 [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_str_equal(v1: *const libc::c_void, v2: *const libc::c_void) -> libc::c_int;
}


/*
gpointer g_type_class_ref() [void *]
	(GType) type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_class_ref(type_: libc::c_ulong) -> *mut libc::c_void;
}


/*
gpointer g_bytes_unref_to_data() [void *]
	(GBytes *) bytes [struct _GBytes *]
	(gsize *) size [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bytes_unref_to_data(bytes: *mut _GBytes, size: *mut libc::c_ulong) -> *mut libc::c_void;
}


/*
gulong g_signal_connect_object() [unsigned long]
	(gpointer) instance [void *]
	(const gchar *) detailed_signal [const char *]
	(GCallback) c_handler [void (*)(void)]
	(gpointer) gobject [void *]
	(GConnectFlags) connect_flags [GConnectFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_connect_object(instance: *mut libc::c_void, detailed_signal: *const libc::c_char, c_handler: Option<extern fn()>, gobject: *mut libc::c_void, connect_flags: libc::c_uint) -> libc::c_ulong;
}


/*
gboolean g_bookmark_file_has_item() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_has_item(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char) -> libc::c_int;
}


/*
GParamSpec * g_param_spec_variant() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(const GVariantType *) type [const struct _GVariantType *]
	(GVariant *) default_value [struct _GVariant *]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_variant(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, type_: *const _GVariantType, default_value: *mut _GVariant, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
void g_hook_list_marshal()
	(GHookList *) hook_list [struct _GHookList *]
	(gboolean) may_recurse [int]
	(GHookMarshaller) marshaller [void (*)(struct _GHook *, void *)]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_list_marshal(hook_list: *mut _GHookList, may_recurse: libc::c_int, marshaller: Option<extern fn(*mut _GHook, *mut libc::c_void)>, marshal_data: *mut libc::c_void);
}


/*
void g_relation_insert()
	(GRelation *) relation [struct _GRelation *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_relation_insert(relation: *mut _GRelation);
}


/*
gint g_atomic_int_add() [int]
	(volatile gint *) atomic [volatile int *]
	(gint) val [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_atomic_int_add(atomic: *mut libc::c_int, val: libc::c_int) -> libc::c_int;
}


/*
gpointer g_ptr_array_remove_index() [void *]
	(GPtrArray *) array [struct _GPtrArray *]
	(guint) index_ [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_remove_index(array: *mut _GPtrArray, index_: libc::c_uint) -> *mut libc::c_void;
}


/*
void g_node_push_allocator()
	(GAllocator *) allocator [struct _GAllocator *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_push_allocator(allocator: *mut _GAllocator);
}


/*
gchar * g_utf8_prev_char() [char *]
	(const gchar *) p [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_prev_char(p: *const libc::c_char) -> *mut libc::c_char;
}


/*
GEnumValue * g_enum_get_value_by_nick() [struct _GEnumValue *]
	(GEnumClass *) enum_class [struct _GEnumClass *]
	(const gchar *) nick [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_enum_get_value_by_nick(enum_class: *mut _GEnumClass, nick: *const libc::c_char) -> *mut _GEnumValue;
}


/*
void g_value_set_double()
	(GValue *) value [struct _GValue *]
	(gdouble) v_double [double]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_double(value: *mut _GValue, v_double: libc::c_double);
}


/*
gchar * g_filename_from_utf8() [char *]
	(const gchar *) utf8string [const char *]
	(gssize) len [long]
	(gsize *) bytes_read [unsigned long *]
	(gsize *) bytes_written [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_filename_from_utf8(utf8string: *const libc::c_char, len: libc::c_long, bytes_read: *mut libc::c_ulong, bytes_written: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
GType g_bytes_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bytes_get_type() -> libc::c_ulong;
}


/*
GArray * g_array_remove_range() [struct _GArray *]
	(GArray *) array [struct _GArray *]
	(guint) index_ [unsigned int]
	(guint) length [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_array_remove_range(array: *mut _GArray, index_: libc::c_uint, length: libc::c_uint) -> *mut _GArray;
}


/*
void g_value_set_schar()
	(GValue *) value [struct _GValue *]
	(gint8) v_char [signed char]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_schar(value: *mut _GValue, v_char: libc::c_char);
}


/*
gchar ** g_environ_unsetenv() [char **]
	(gchar **) envp [char **]
	(const gchar *) variable [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_environ_unsetenv(envp: *mut *mut libc::c_char, variable: *const libc::c_char) -> *mut *mut libc::c_char;
}


/*
gboolean g_value_fits_pointer() [int]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_fits_pointer(value: *const _GValue) -> libc::c_int;
}


/*
void g_value_init_from_instance()
	(GValue *) value [struct _GValue *]
	(gpointer) instance [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_init_from_instance(value: *mut _GValue, instance: *mut libc::c_void);
}


/*
GByteArray * g_byte_array_append() [struct _GByteArray *]
	(GByteArray *) array [struct _GByteArray *]
	(const guint8 *) data [const unsigned char *]
	(guint) len [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_byte_array_append(array: *mut _GByteArray, data: *const libc::c_uchar, len: libc::c_uint) -> *mut _GByteArray;
}


/*
void g_static_rw_lock_reader_lock()
	(GStaticRWLock *) lock [struct _GStaticRWLock *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_rw_lock_reader_lock(lock: *mut _GStaticRWLock);
}


/*
void g_list_free()
	(GList *) list [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_free(list: *mut _GList);
}


/*
gint g_async_queue_length_unlocked() [int]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_length_unlocked(queue: *mut _GAsyncQueue) -> libc::c_int;
}


/*
gpointer g_async_queue_timeout_pop_unlocked() [void *]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(guint64) timeout [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_timeout_pop_unlocked(queue: *mut _GAsyncQueue, timeout: libc::c_ulong) -> *mut libc::c_void;
}


/*
GHashTable * g_hash_table_new_full() [struct _GHashTable *]
	(GHashFunc) hash_func [unsigned int (*)(const void *)]
	(GEqualFunc) key_equal_func [int (*)(const void *, const void *)]
	(GDestroyNotify) key_destroy_func [void (*)(void *)]
	(GDestroyNotify) value_destroy_func [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_new_full(hash_func: Option<extern fn(*const libc::c_void) -> libc::c_uint>, key_equal_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>, key_destroy_func: Option<extern fn(*mut libc::c_void)>, value_destroy_func: Option<extern fn(*mut libc::c_void)>) -> *mut _GHashTable;
}


/*
void g_io_channel_set_buffered()
	(GIOChannel *) channel [struct _GIOChannel *]
	(gboolean) buffered [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_set_buffered(channel: *mut _GIOChannel, buffered: libc::c_int);
}


/*
GParamSpec * g_param_spec_float() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(gfloat) minimum [float]
	(gfloat) maximum [float]
	(gfloat) default_value [float]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_float(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, minimum: libc::c_float, maximum: libc::c_float, default_value: libc::c_float, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
GLogFunc g_log_set_default_handler() [void (*)(const char *, GLogLevelFlags, const char *, void *)]
	(GLogFunc) log_func [void (*)(const char *, GLogLevelFlags, const char *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_log_set_default_handler(log_func: Option<extern fn(*const libc::c_char, libc::c_uint, *const libc::c_char, *mut libc::c_void)>, user_data: *mut libc::c_void) -> Option<extern fn(*const libc::c_char, libc::c_uint, *const libc::c_char, *mut libc::c_void)>;
}


/*
gchar * g_utf8_find_prev_char() [char *]
	(const gchar *) str [const char *]
	(const gchar *) p [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_find_prev_char(str: *const libc::c_char, p: *const libc::c_char) -> *mut libc::c_char;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_add_vtable(testpath: *const libc::c_char, data_size: libc::c_ulong, test_data: *const libc::c_void, data_setup: Option<extern fn(*mut libc::c_void, *const libc::c_void)>, data_test: Option<extern fn(*mut libc::c_void, *const libc::c_void)>, data_teardown: Option<extern fn(*mut libc::c_void, *const libc::c_void)>);
}


/*
void g_sequence_move()
	(GSequenceIter *) src [struct _GSequenceNode *]
	(GSequenceIter *) dest [struct _GSequenceNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_move(src: *mut _GSequenceNode, dest: *mut _GSequenceNode);
}


/*
void g_cond_wait()
	(GCond *) cond [struct _GCond *]
	(GMutex *) mutex [union _GMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cond_wait(cond: *mut _GCond, mutex: *mut _GMutex);
}


/*
guint g_date_get_monday_week_of_year() [unsigned int]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_get_monday_week_of_year(date: *const _GDate) -> libc::c_uint;
}


/*
GArray * g_array_append_vals() [struct _GArray *]
	(GArray *) array [struct _GArray *]
	(gconstpointer) data [const void *]
	(guint) len [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_array_append_vals(array: *mut _GArray, data: *const libc::c_void, len: libc::c_uint) -> *mut _GArray;
}


/*
gboolean g_variant_iter_loop() [int]
	(GVariantIter *) iter [struct _GVariantIter *]
	(const gchar *) format_string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_iter_loop(iter: *mut _GVariantIter, format_string: *const libc::c_char) -> libc::c_int;
}


/*
const gchar * g_dpgettext() [const char *]
	(const gchar *) domain [const char *]
	(const gchar *) msgctxtid [const char *]
	(gsize) msgidoffset [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_dpgettext(domain: *const libc::c_char, msgctxtid: *const libc::c_char, msgidoffset: libc::c_ulong) -> *const libc::c_char;
}


/*
void g_bookmark_file_set_added()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(time_t) added [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_set_added(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, added: libc::c_long);
}


/*
gchar * g_strescape() [char *]
	(const gchar *) source [const char *]
	(const gchar *) exceptions [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strescape(source: *const libc::c_char, exceptions: *const libc::c_char) -> *mut libc::c_char;
}


/*
gboolean g_main_context_is_owner() [int]
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_is_owner(context: *mut _GMainContext) -> libc::c_int;
}


/*
gpointer g_datalist_get_data() [void *]
	(GData **) datalist [struct _GData **]
	(const gchar *) key [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_datalist_get_data(datalist: *mut *mut _GData, key: *const libc::c_char) -> *mut libc::c_void;
}


/*
gboolean g_ptr_array_remove() [int]
	(GPtrArray *) array [struct _GPtrArray *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_remove(array: *mut _GPtrArray, data: *mut libc::c_void) -> libc::c_int;
}


/*
void g_datalist_clear()
	(GData **) datalist [struct _GData **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_datalist_clear(datalist: *mut *mut _GData);
}


/*
void g_source_set_callback_indirect()
	(GSource *) source [struct _GSource *]
	(gpointer) callback_data [void *]
	(GSourceCallbackFuncs *) callback_funcs [struct _GSourceCallbackFuncs *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_set_callback_indirect(source: *mut _GSource, callback_data: *mut libc::c_void, callback_funcs: *mut _GSourceCallbackFuncs);
}


/*
void g_allocator_free()
	(GAllocator *) allocator [struct _GAllocator *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_allocator_free(allocator: *mut _GAllocator);
}


/*
gint64 g_variant_get_int64() [long]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_int64(value: *mut _GVariant) -> libc::c_long;
}


/*
gsize g_strlcpy() [unsigned long]
	(gchar *) dest [char *]
	(const gchar *) src [const char *]
	(gsize) dest_size [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strlcpy(dest: *mut libc::c_char, src: *const libc::c_char, dest_size: libc::c_ulong) -> libc::c_ulong;
}


/*
GOptionContext * g_option_context_new() [struct _GOptionContext *]
	(const gchar *) parameter_string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_context_new(parameter_string: *const libc::c_char) -> *mut _GOptionContext;
}


/*
GVariant * g_variant_new_objv() [struct _GVariant *]
	(const gchar *const *) strv [const char *const *]
	(gssize) length [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_objv(strv: *const *const libc::c_char, length: libc::c_long) -> *mut _GVariant;
}


/*
gboolean g_key_file_has_group() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_has_group(key_file: *mut _GKeyFile, group_name: *const libc::c_char) -> libc::c_int;
}


/*
gboolean g_param_value_validate() [int]
	(GParamSpec *) pspec [struct _GParamSpec *]
	(GValue *) value [struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_value_validate(pspec: *mut _GParamSpec, value: *mut _GValue) -> libc::c_int;
}


/*
gulong g_signal_handler_find() [unsigned long]
	(gpointer) instance [void *]
	(GSignalMatchType) mask [GSignalMatchType]
	(guint) signal_id [unsigned int]
	(GQuark) detail [unsigned int]
	(GClosure *) closure [struct _GClosure *]
	(gpointer) func [void *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_handler_find(instance: *mut libc::c_void, mask: libc::c_uint, signal_id: libc::c_uint, detail: libc::c_uint, closure: *mut _GClosure, func: *mut libc::c_void, data: *mut libc::c_void) -> libc::c_ulong;
}


/*
gboolean g_unichar_isalnum() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_isalnum(c: libc::c_uint) -> libc::c_int;
}


/*
GHook * g_hook_find_func() [struct _GHook *]
	(GHookList *) hook_list [struct _GHookList *]
	(gboolean) need_valids [int]
	(gpointer) func [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_find_func(hook_list: *mut _GHookList, need_valids: libc::c_int, func: *mut libc::c_void) -> *mut _GHook;
}


/*
void g_io_channel_unref()
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_unref(channel: *mut _GIOChannel);
}


/*
gboolean g_type_check_instance() [int]
	(GTypeInstance *) instance [struct _GTypeInstance *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_check_instance(instance: *mut _GTypeInstance) -> libc::c_int;
}


/*
GString * g_string_append_len() [struct _GString *]
	(GString *) string [struct _GString *]
	(const gchar *) val [const char *]
	(gssize) len [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_append_len(string: *mut _GString, val: *const libc::c_char, len: libc::c_long) -> *mut _GString;
}


/*
gboolean g_signal_has_handler_pending() [int]
	(gpointer) instance [void *]
	(guint) signal_id [unsigned int]
	(GQuark) detail [unsigned int]
	(gboolean) may_be_blocked [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_has_handler_pending(instance: *mut libc::c_void, signal_id: libc::c_uint, detail: libc::c_uint, may_be_blocked: libc::c_int) -> libc::c_int;
}


/*
GParamSpec * g_param_spec_enum() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(GType) enum_type [unsigned long]
	(gint) default_value [int]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_enum(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, enum_type: libc::c_ulong, default_value: libc::c_int, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
gchar * g_variant_dup_bytestring() [char *]
	(GVariant *) value [struct _GVariant *]
	(gsize *) length [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_dup_bytestring(value: *mut _GVariant, length: *mut libc::c_ulong) -> *mut libc::c_char;
}


/*
void g_value_set_long()
	(GValue *) value [struct _GValue *]
	(glong) v_long [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_long(value: *mut _GValue, v_long: libc::c_long);
}


/*
void g_cclosure_marshal_VOID__DOUBLE()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_VOID__DOUBLE(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
gboolean g_time_val_from_iso8601() [int]
	(const gchar *) iso_date [const char *]
	(GTimeVal *) time_ [struct _GTimeVal *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_time_val_from_iso8601(iso_date: *const libc::c_char, time_: *mut _GTimeVal) -> libc::c_int;
}


/*
GUnicodeScript g_unicode_script_from_iso15924() [GUnicodeScript]
	(guint32) iso15924 [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unicode_script_from_iso15924(iso15924: libc::c_uint) -> libc::c_uint;
}


/*
GType g_date_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_get_type() -> libc::c_ulong;
}


/*
void g_async_queue_push()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_push(queue: *mut _GAsyncQueue, data: *mut libc::c_void);
}


/*
gboolean g_source_remove_by_funcs_user_data() [int]
	(GSourceFuncs *) funcs [struct _GSourceFuncs *]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_remove_by_funcs_user_data(funcs: *mut _GSourceFuncs, user_data: *mut libc::c_void) -> libc::c_int;
}


/*
void g_param_spec_set_qdata()
	(GParamSpec *) pspec [struct _GParamSpec *]
	(GQuark) quark [unsigned int]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_set_qdata(pspec: *mut _GParamSpec, quark: libc::c_uint, data: *mut libc::c_void);
}


/*
GList * g_list_copy_deep() [struct _GList *]
	(GList *) list [struct _GList *]
	(GCopyFunc) func [void *(*)(const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_copy_deep(list: *mut _GList, func: Option<extern fn(*const libc::c_void, *mut libc::c_void) -> *mut libc::c_void>, user_data: *mut libc::c_void) -> *mut _GList;
}


/*
void g_queue_free_full()
	(GQueue *) queue [struct _GQueue *]
	(GDestroyNotify) free_func [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_free_full(queue: *mut _GQueue, free_func: Option<extern fn(*mut libc::c_void)>);
}


/*
void g_thread_exit()
	(gpointer) retval [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_exit(retval: *mut libc::c_void);
}


/*
const gchar * g_dcgettext() [const char *]
	(const gchar *) domain [const char *]
	(const gchar *) msgid [const char *]
	(gint) category [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_dcgettext(domain: *const libc::c_char, msgid: *const libc::c_char, category: libc::c_int) -> *const libc::c_char;
}


/*
gchar * g_strstr_len() [char *]
	(const gchar *) haystack [const char *]
	(gssize) haystack_len [long]
	(const gchar *) needle [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strstr_len(haystack: *const libc::c_char, haystack_len: libc::c_long, needle: *const libc::c_char) -> *mut libc::c_char;
}


/*
void g_rec_mutex_init()
	(GRecMutex *) rec_mutex [struct _GRecMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rec_mutex_init(rec_mutex: *mut _GRecMutex);
}


/*
gboolean g_unichar_isxdigit() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_isxdigit(c: libc::c_uint) -> libc::c_int;
}


/*
void g_string_chunk_clear()
	(GStringChunk *) chunk [struct _GStringChunk *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_chunk_clear(chunk: *mut _GStringChunk);
}


/*
GArray * g_array_ref() [struct _GArray *]
	(GArray *) array [struct _GArray *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_array_ref(array: *mut _GArray) -> *mut _GArray;
}


/*
GString * g_string_append_c_inline() [struct _GString *]
	(GString *) gstring [struct _GString *]
	(gchar) c [char]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_append_c_inline(gstring: *mut _GString, c: libc::c_char) -> *mut _GString;
}


/*
void g_async_queue_unref()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_unref(queue: *mut _GAsyncQueue);
}


/*
GQuark g_option_error_quark() [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_error_quark() -> libc::c_uint;
}


/*
gchar * g_date_time_format() [char *]
	(GDateTime *) datetime [struct _GDateTime *]
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_format(datetime: *mut _GDateTime, format: *const libc::c_char) -> *mut libc::c_char;
}


/*
GType g_byte_array_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_byte_array_get_type() -> libc::c_ulong;
}


/*
GSource * g_timeout_source_new_seconds() [struct _GSource *]
	(guint) interval [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_timeout_source_new_seconds(interval: libc::c_uint) -> *mut _GSource;
}


/*
void g_scanner_scope_remove_symbol()
	(GScanner *) scanner [struct _GScanner *]
	(guint) scope_id [unsigned int]
	(const gchar *) symbol [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_scope_remove_symbol(scanner: *mut _GScanner, scope_id: libc::c_uint, symbol: *const libc::c_char);
}


/*
void g_cclosure_marshal_VOID__UINTv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_cclosure_marshal_VOID__UINTv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
void g_match_info_unref()
	(GMatchInfo *) match_info [struct _GMatchInfo *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_match_info_unref(match_info: *mut _GMatchInfo);
}


/*
void g_cond_init()
	(GCond *) cond [struct _GCond *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cond_init(cond: *mut _GCond);
}


/*
gpointer g_type_interface_peek() [void *]
	(gpointer) instance_class [void *]
	(GType) iface_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_interface_peek(instance_class: *mut libc::c_void, iface_type: libc::c_ulong) -> *mut libc::c_void;
}


/*
gchar * g_ascii_dtostr() [char *]
	(gchar *) buffer [char *]
	(gint) buf_len [int]
	(gdouble) d [double]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ascii_dtostr(buffer: *mut libc::c_char, buf_len: libc::c_int, d: libc::c_double) -> *mut libc::c_char;
}


/*
guint g_signal_handlers_disconnect_matched() [unsigned int]
	(gpointer) instance [void *]
	(GSignalMatchType) mask [GSignalMatchType]
	(guint) signal_id [unsigned int]
	(GQuark) detail [unsigned int]
	(GClosure *) closure [struct _GClosure *]
	(gpointer) func [void *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_handlers_disconnect_matched(instance: *mut libc::c_void, mask: libc::c_uint, signal_id: libc::c_uint, detail: libc::c_uint, closure: *mut _GClosure, func: *mut libc::c_void, data: *mut libc::c_void) -> libc::c_uint;
}


/*
gboolean g_test_trap_fork() [int]
	(guint64) usec_timeout [unsigned long]
	(GTestTrapFlags) test_trap_flags [GTestTrapFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_trap_fork(usec_timeout: libc::c_ulong, test_trap_flags: libc::c_uint) -> libc::c_int;
}


/*
GSource * g_main_context_find_source_by_user_data() [struct _GSource *]
	(GMainContext *) context [struct _GMainContext *]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_find_source_by_user_data(context: *mut _GMainContext, user_data: *mut libc::c_void) -> *mut _GSource;
}


/*
GThread * g_thread_try_new() [struct _GThread *]
	(const gchar *) name [const char *]
	(GThreadFunc) func [void *(*)(void *)]
	(gpointer) data [void *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_try_new(name: *const libc::c_char, func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>, data: *mut libc::c_void, error: *mut *mut _GError) -> *mut _GThread;
}


/*
GParamSpec * g_object_class_find_property() [struct _GParamSpec *]
	(GObjectClass *) oclass [struct _GObjectClass *]
	(const gchar *) property_name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_class_find_property(oclass: *mut _GObjectClass, property_name: *const libc::c_char) -> *mut _GParamSpec;
}


/*
GVariantDict * g_variant_dict_ref() [struct _GVariantDict *]
	(GVariantDict *) dict [struct _GVariantDict *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_dict_ref(dict: *mut _GVariantDict) -> *mut _GVariantDict;
}


/*
GClosure * g_closure_ref() [struct _GClosure *]
	(GClosure *) closure [struct _GClosure *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_closure_ref(closure: *mut _GClosure) -> *mut _GClosure;
}


/*
void g_random_set_seed()
	(guint32) seed [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_random_set_seed(seed: libc::c_uint);
}


/*
GOptionGroup * g_option_group_new() [struct _GOptionGroup *]
	(const gchar *) name [const char *]
	(const gchar *) description [const char *]
	(const gchar *) help_description [const char *]
	(gpointer) user_data [void *]
	(GDestroyNotify) destroy [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_group_new(name: *const libc::c_char, description: *const libc::c_char, help_description: *const libc::c_char, user_data: *mut libc::c_void, destroy: Option<extern fn(*mut libc::c_void)>) -> *mut _GOptionGroup;
}


/*
GObject * g_binding_get_target() [struct _GObject *]
	(GBinding *) binding [struct _GBinding *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_binding_get_target(binding: *mut _GBinding) -> *mut _GObject;
}


/*
gint g_thread_pool_get_max_unused_threads() [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_pool_get_max_unused_threads() -> libc::c_int;
}


/*
GTokenValue g_scanner_cur_value() [union _GTokenValue]
	(GScanner *) scanner [struct _GScanner *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_cur_value(scanner: *mut _GScanner) -> _GTokenValue;
}


/*
gboolean g_int_equal() [int]
	(gconstpointer) v1 [const void *]
	(gconstpointer) v2 [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_int_equal(v1: *const libc::c_void, v2: *const libc::c_void) -> libc::c_int;
}


/*
GVariant * g_variant_new_fixed_array() [struct _GVariant *]
	(const GVariantType *) element_type [const struct _GVariantType *]
	(gconstpointer) elements [const void *]
	(gsize) n_elements [unsigned long]
	(gsize) element_size [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_fixed_array(element_type: *const _GVariantType, elements: *const libc::c_void, n_elements: libc::c_ulong, element_size: libc::c_ulong) -> *mut _GVariant;
}


/*
gint g_time_zone_adjust_time() [int]
	(GTimeZone *) tz [struct _GTimeZone *]
	(GTimeType) type [GTimeType]
	(gint64 *) time_ [long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_time_zone_adjust_time(tz: *mut _GTimeZone, type_: libc::c_uint, time_: *mut libc::c_long) -> libc::c_int;
}


/*
void g_signal_emit()
	(gpointer) instance [void *]
	(guint) signal_id [unsigned int]
	(GQuark) detail [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_emit(instance: *mut libc::c_void, signal_id: libc::c_uint, detail: libc::c_uint);
}


/*
guint8 * g_byte_array_free() [unsigned char *]
	(GByteArray *) array [struct _GByteArray *]
	(gboolean) free_segment [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_byte_array_free(array: *mut _GByteArray, free_segment: libc::c_int) -> *mut libc::c_uchar;
}


/*
void g_async_queue_sort()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(GCompareDataFunc) func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_sort(queue: *mut _GAsyncQueue, func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
gboolean g_path_is_absolute() [int]
	(const gchar *) file_name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_path_is_absolute(file_name: *const libc::c_char) -> libc::c_int;
}


/*
GString * g_string_prepend_len() [struct _GString *]
	(GString *) string [struct _GString *]
	(const gchar *) val [const char *]
	(gssize) len [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_prepend_len(string: *mut _GString, val: *const libc::c_char, len: libc::c_long) -> *mut _GString;
}


/*
GString * g_string_overwrite_len() [struct _GString *]
	(GString *) string [struct _GString *]
	(gsize) pos [unsigned long]
	(const gchar *) val [const char *]
	(gssize) len [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_overwrite_len(string: *mut _GString, pos: libc::c_ulong, val: *const libc::c_char, len: libc::c_long) -> *mut _GString;
}


/*
gboolean g_main_loop_is_running() [int]
	(GMainLoop *) loop [struct _GMainLoop *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_loop_is_running(loop_: *mut _GMainLoop) -> libc::c_int;
}


/*
GString * g_string_new() [struct _GString *]
	(const gchar *) init [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_new(init: *const libc::c_char) -> *mut _GString;
}


/*
gchar * g_filename_to_uri() [char *]
	(const gchar *) filename [const char *]
	(const gchar *) hostname [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_filename_to_uri(filename: *const libc::c_char, hostname: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
void g_assertion_message_expr()
	(const char *) domain
	(const char *) file
	(int) line
	(const char *) func
	(const char *) expr
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_assertion_message_expr(domain: *const libc::c_char, file: *const libc::c_char, line: libc::c_int, func: *const libc::c_char, expr: *const libc::c_char);
}


/*
const GVariantType * g_variant_type_key() [const struct _GVariantType *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_key(type_: *const _GVariantType) -> *const _GVariantType;
}


/*
gpointer g_mem_chunk_alloc0() [void *]
	(GMemChunk *) mem_chunk [struct _GMemChunk *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mem_chunk_alloc0(mem_chunk: *mut _GMemChunk) -> *mut libc::c_void;
}


/*
gboolean g_key_file_remove_key() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_remove_key(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
GDateTime * g_date_time_ref() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_ref(datetime: *mut _GDateTime) -> *mut _GDateTime;
}


/*
GBookmarkFile * g_bookmark_file_new() [struct _GBookmarkFile *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_new() -> *mut _GBookmarkFile;
}


/*
const gchar * g_binding_get_source_property() [const char *]
	(GBinding *) binding [struct _GBinding *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_binding_get_source_property(binding: *mut _GBinding) -> *const libc::c_char;
}


/*
GList * g_queue_pop_nth_link() [struct _GList *]
	(GQueue *) queue [struct _GQueue *]
	(guint) n [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_pop_nth_link(queue: *mut _GQueue, n: libc::c_uint) -> *mut _GList;
}


/*
guint g_atomic_int_xor() [unsigned int]
	(volatile guint *) atomic [volatile unsigned int *]
	(guint) val [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_atomic_int_xor(atomic: *mut libc::c_uint, val: libc::c_uint) -> libc::c_uint;
}


/*
gpointer g_async_queue_pop() [void *]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_pop(queue: *mut _GAsyncQueue) -> *mut libc::c_void;
}


/*
gchar * g_stpcpy() [char *]
	(gchar *) dest [char *]
	(const char *) src
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_stpcpy(dest: *mut libc::c_char, src: *const libc::c_char) -> *mut libc::c_char;
}


/*
GString * g_string_insert_c() [struct _GString *]
	(GString *) string [struct _GString *]
	(gssize) pos [long]
	(gchar) c [char]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_insert_c(string: *mut _GString, pos: libc::c_long, c: libc::c_char) -> *mut _GString;
}


/*
gint g_date_time_get_year() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_get_year(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
GType g_key_file_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_get_type() -> libc::c_ulong;
}


/*
GVariant * g_variant_new_parsed() [struct _GVariant *]
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_parsed(format: *const libc::c_char) -> *mut _GVariant;
}


/*
gint64 g_source_get_ready_time() [long]
	(GSource *) source [struct _GSource *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_get_ready_time(source: *mut _GSource) -> libc::c_long;
}


/*
guint g_thread_pool_unprocessed() [unsigned int]
	(GThreadPool *) pool [struct _GThreadPool *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_pool_unprocessed(pool: *mut _GThreadPool) -> libc::c_uint;
}


/*
void g_printerr()
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_printerr(format: *const libc::c_char);
}


/*
void g_tree_unref()
	(GTree *) tree [struct _GTree *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_tree_unref(tree: *mut _GTree);
}


/*
gulong g_signal_connect_closure() [unsigned long]
	(gpointer) instance [void *]
	(const gchar *) detailed_signal [const char *]
	(GClosure *) closure [struct _GClosure *]
	(gboolean) after [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_connect_closure(instance: *mut libc::c_void, detailed_signal: *const libc::c_char, closure: *mut _GClosure, after: libc::c_int) -> libc::c_ulong;
}


/*
guint g_timeout_add_seconds_full() [unsigned int]
	(gint) priority [int]
	(guint) interval [unsigned int]
	(GSourceFunc) function [int (*)(void *)]
	(gpointer) data [void *]
	(GDestroyNotify) notify [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_timeout_add_seconds_full(priority: libc::c_int, interval: libc::c_uint, function: Option<extern fn(*mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void, notify: Option<extern fn(*mut libc::c_void)>) -> libc::c_uint;
}


/*
gint g_mkstemp_full() [int]
	(gchar *) tmpl [char *]
	(gint) flags [int]
	(gint) mode [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mkstemp_full(tmpl: *mut libc::c_char, flags: libc::c_int, mode: libc::c_int) -> libc::c_int;
}


/*
GString * g_string_insert() [struct _GString *]
	(GString *) string [struct _GString *]
	(gssize) pos [long]
	(const gchar *) val [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_insert(string: *mut _GString, pos: libc::c_long, val: *const libc::c_char) -> *mut _GString;
}


/*
gpointer g_async_queue_pop_unlocked() [void *]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_pop_unlocked(queue: *mut _GAsyncQueue) -> *mut libc::c_void;
}


/*
void g_main_context_push_thread_default()
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_push_thread_default(context: *mut _GMainContext);
}


/*
guint g_bit_storage() [unsigned int]
	(gulong) number [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bit_storage(number: libc::c_ulong) -> libc::c_uint;
}


/*
gboolean g_date_valid_day() [int]
	(GDateDay) day [unsigned char]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_valid_day(day: libc::c_uchar) -> libc::c_int;
}


/*
void g_type_plugin_unuse()
	(GTypePlugin *) plugin [struct _GTypePlugin *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_plugin_unuse(plugin: *mut _GTypePlugin);
}


/*
GSList * g_slist_alloc() [struct _GSList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_alloc() -> *mut _GSList;
}


/*
void g_object_watch_closure()
	(GObject *) object [struct _GObject *]
	(GClosure *) closure [struct _GClosure *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_watch_closure(object: *mut _GObject, closure: *mut _GClosure);
}


/*
gboolean g_rec_mutex_trylock() [int]
	(GRecMutex *) rec_mutex [struct _GRecMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rec_mutex_trylock(rec_mutex: *mut _GRecMutex) -> libc::c_int;
}


/*
gboolean g_once_init_enter_impl() [int]
	(volatile gsize *) location [volatile unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_once_init_enter_impl(location: *mut libc::c_ulong) -> libc::c_int;
}


/*
gboolean g_io_channel_get_close_on_unref() [int]
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_get_close_on_unref(channel: *mut _GIOChannel) -> libc::c_int;
}


/*
GType g_value_get_gtype() [unsigned long]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_gtype(value: *const _GValue) -> libc::c_ulong;
}


/*
char * g_uri_parse_scheme()
	(const char *) uri
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_uri_parse_scheme(uri: *const libc::c_char) -> *mut libc::c_char;
}


/*
GUnicodeType g_unichar_type() [GUnicodeType]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_type(c: libc::c_uint) -> libc::c_uint;
}


/*
GRegex * g_match_info_get_regex() [struct _GRegex *]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_match_info_get_regex(match_info: *const _GMatchInfo) -> *mut _GRegex;
}


/*
gpointer g_value_get_boxed() [void *]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_boxed(value: *const _GValue) -> *mut libc::c_void;
}


/*
GString * g_variant_print_string() [struct _GString *]
	(GVariant *) value [struct _GVariant *]
	(GString *) string [struct _GString *]
	(gboolean) type_annotate [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_print_string(value: *mut _GVariant, string: *mut _GString, type_annotate: libc::c_int) -> *mut _GString;
}


/*
gchar * g_array_free() [char *]
	(GArray *) array [struct _GArray *]
	(gboolean) free_segment [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_array_free(array: *mut _GArray, free_segment: libc::c_int) -> *mut libc::c_char;
}


/*
gboolean g_relation_exists() [int]
	(GRelation *) relation [struct _GRelation *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_relation_exists(relation: *mut _GRelation) -> libc::c_int;
}


/*
const gchar * g_strerror() [const char *]
	(gint) errnum [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strerror(errnum: libc::c_int) -> *const libc::c_char;
}


/*
void g_hmac_update()
	(GHmac *) hmac [struct _GHmac *]
	(const guchar *) data [const unsigned char *]
	(gssize) length [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hmac_update(hmac: *mut _GHmac, data: *const libc::c_uchar, length: libc::c_long);
}


/*
GVariantType * g_variant_type_new_dict_entry() [struct _GVariantType *]
	(const GVariantType *) key [const struct _GVariantType *]
	(const GVariantType *) value [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_new_dict_entry(key: *const _GVariantType, value: *const _GVariantType) -> *mut _GVariantType;
}


/*
void g_date_free()
	(GDate *) date [struct _GDate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_free(date: *mut _GDate);
}


/*
guint g_hash_table_foreach_remove() [unsigned int]
	(GHashTable *) hash_table [struct _GHashTable *]
	(GHRFunc) func [int (*)(void *, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_foreach_remove(hash_table: *mut _GHashTable, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void) -> libc::c_uint;
}


/*
GTestSuite * g_test_get_root() [struct GTestSuite *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_get_root() -> *mut GTestSuite;
}


/*
gchar * g_hostname_to_unicode() [char *]
	(const gchar *) hostname [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hostname_to_unicode(hostname: *const libc::c_char) -> *mut libc::c_char;
}


/*
GPtrArray * g_ptr_array_remove_range() [struct _GPtrArray *]
	(GPtrArray *) array [struct _GPtrArray *]
	(guint) index_ [unsigned int]
	(guint) length [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_remove_range(array: *mut _GPtrArray, index_: libc::c_uint, length: libc::c_uint) -> *mut _GPtrArray;
}


/*
void g_source_set_name_by_id()
	(guint) tag [unsigned int]
	(const char *) name
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_set_name_by_id(tag: libc::c_uint, name: *const libc::c_char);
}


/*
GString * g_string_append_c() [struct _GString *]
	(GString *) string [struct _GString *]
	(gchar) c [char]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_append_c(string: *mut _GString, c: libc::c_char) -> *mut _GString;
}


/*
gboolean g_date_valid_year() [int]
	(GDateYear) year [unsigned short]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_valid_year(year: libc::c_ushort) -> libc::c_int;
}


/*
void g_queue_insert_before()
	(GQueue *) queue [struct _GQueue *]
	(GList *) sibling [struct _GList *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_insert_before(queue: *mut _GQueue, sibling: *mut _GList, data: *mut libc::c_void);
}


/*
void g_key_file_set_integer_list()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gint []) list [int []]
	(gsize) length [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_set_integer_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, list: *mut libc::c_int /* INCOMPLETEARRAY */, length: libc::c_ulong);
}


/*
GDateTime * g_date_time_new_now_utc() [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_new_now_utc() -> *mut _GDateTime;
}


/*
void g_clear_pointer()
	(gpointer *) pp [void **]
	(GDestroyNotify) destroy [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_clear_pointer(pp: *mut *mut libc::c_void, destroy: Option<extern fn(*mut libc::c_void)>);
}


/*
gpointer * g_ptr_array_free() [void **]
	(GPtrArray *) array [struct _GPtrArray *]
	(gboolean) free_seg [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_free(array: *mut _GPtrArray, free_seg: libc::c_int) -> *mut *mut libc::c_void;
}


/*
GVariantBuilder * g_variant_builder_ref() [struct _GVariantBuilder *]
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_builder_ref(builder: *mut _GVariantBuilder) -> *mut _GVariantBuilder;
}


/*
void g_pointer_bit_lock()
	(volatile void *) address
	(gint) lock_bit [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_pointer_bit_lock(address: *mut libc::c_void, lock_bit: libc::c_int);
}


/*
gchar * g_variant_dup_string() [char *]
	(GVariant *) value [struct _GVariant *]
	(gsize *) length [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_dup_string(value: *mut _GVariant, length: *mut libc::c_ulong) -> *mut libc::c_char;
}


/*
gint8 g_value_get_schar() [signed char]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_schar(value: *const _GValue) -> libc::c_char;
}


/*
GDateTime * g_date_time_new_from_timeval_local() [struct _GDateTime *]
	(const GTimeVal *) tv [const struct _GTimeVal *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_new_from_timeval_local(tv: *const _GTimeVal) -> *mut _GDateTime;
}


/*
guint64 g_variant_get_uint64() [unsigned long]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_uint64(value: *mut _GVariant) -> libc::c_ulong;
}


/*
gint32 g_rand_int_range() [int]
	(GRand *) rand_ [struct _GRand *]
	(gint32) begin [int]
	(gint32) end [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rand_int_range(rand_: *mut _GRand, begin: libc::c_int, end: libc::c_int) -> libc::c_int;
}


/*
gboolean g_date_valid_julian() [int]
	(guint32) julian_date [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_valid_julian(julian_date: libc::c_uint) -> libc::c_int;
}


/*
GValueArray * g_value_array_insert() [struct _GValueArray *]
	(GValueArray *) value_array [struct _GValueArray *]
	(guint) index_ [unsigned int]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_array_insert(value_array: *mut _GValueArray, index_: libc::c_uint, value: *const _GValue) -> *mut _GValueArray;
}


/*
void g_source_get_current_time()
	(GSource *) source [struct _GSource *]
	(GTimeVal *) timeval [struct _GTimeVal *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_get_current_time(source: *mut _GSource, timeval: *mut _GTimeVal);
}


/*
void g_async_queue_push_unlocked()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_push_unlocked(queue: *mut _GAsyncQueue, data: *mut libc::c_void);
}


/*
GVariant * g_variant_new_uint32() [struct _GVariant *]
	(guint32) value [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_uint32(value: libc::c_uint) -> *mut _GVariant;
}


/*
void g_type_init()
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_init();
}


/*
GSList * g_slist_find() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(gconstpointer) data [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_find(list: *mut _GSList, data: *const libc::c_void) -> *mut _GSList;
}


/*
gchar * g_build_path() [char *]
	(const gchar *) separator [const char *]
	(const gchar *) first_element [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_build_path(separator: *const libc::c_char, first_element: *const libc::c_char) -> *mut libc::c_char;
}


/*
GType g_param_type_register_static() [unsigned long]
	(const gchar *) name [const char *]
	(const GParamSpecTypeInfo *) pspec_info [const struct _GParamSpecTypeInfo *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_type_register_static(name: *const libc::c_char, pspec_info: *const _GParamSpecTypeInfo) -> libc::c_ulong;
}


/*
gchar * g_bookmark_file_get_mime_type() [char *]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_get_mime_type(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
void g_mem_chunk_info()
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mem_chunk_info();
}


/*
void g_source_set_priority()
	(GSource *) source [struct _GSource *]
	(gint) priority [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_set_priority(source: *mut _GSource, priority: libc::c_int);
}


/*
void g_checksum_get_digest()
	(GChecksum *) checksum [struct _GChecksum *]
	(guint8 *) buffer [unsigned char *]
	(gsize *) digest_len [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_checksum_get_digest(checksum: *mut _GChecksum, buffer: *mut libc::c_uchar, digest_len: *mut libc::c_ulong);
}


/*
guint g_get_num_processors() [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_num_processors() -> libc::c_uint;
}


/*
GBytes * g_bytes_new_from_bytes() [struct _GBytes *]
	(GBytes *) bytes [struct _GBytes *]
	(gsize) offset [unsigned long]
	(gsize) length [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bytes_new_from_bytes(bytes: *mut _GBytes, offset: libc::c_ulong, length: libc::c_ulong) -> *mut _GBytes;
}


/*
gboolean g_variant_type_is_basic() [int]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_is_basic(type_: *const _GVariantType) -> libc::c_int;
}


/*
void g_sequence_foreach()
	(GSequence *) seq [struct _GSequence *]
	(GFunc) func [void (*)(void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_foreach(seq: *mut _GSequence, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
void g_hash_table_steal_all()
	(GHashTable *) hash_table [struct _GHashTable *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_steal_all(hash_table: *mut _GHashTable);
}


/*
void g_hook_free()
	(GHookList *) hook_list [struct _GHookList *]
	(GHook *) hook [struct _GHook *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_free(hook_list: *mut _GHookList, hook: *mut _GHook);
}


/*
GDateTime * g_date_time_new_now() [struct _GDateTime *]
	(GTimeZone *) tz [struct _GTimeZone *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_new_now(tz: *mut _GTimeZone) -> *mut _GDateTime;
}


/*
gconstpointer g_bytes_get_data() [const void *]
	(GBytes *) bytes [struct _GBytes *]
	(gsize *) size [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bytes_get_data(bytes: *mut _GBytes, size: *mut libc::c_ulong) -> *const libc::c_void;
}


/*
guint g_trash_stack_height() [unsigned int]
	(GTrashStack **) stack_p [struct _GTrashStack **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_trash_stack_height(stack_p: *mut *mut _GTrashStack) -> libc::c_uint;
}


/*
GHashTable * g_hash_table_new() [struct _GHashTable *]
	(GHashFunc) hash_func [unsigned int (*)(const void *)]
	(GEqualFunc) key_equal_func [int (*)(const void *, const void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_new(hash_func: Option<extern fn(*const libc::c_void) -> libc::c_uint>, key_equal_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GHashTable;
}


/*
gint g_tree_nnodes() [int]
	(GTree *) tree [struct _GTree *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_tree_nnodes(tree: *mut _GTree) -> libc::c_int;
}


/*
GType g_type_plugin_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_plugin_get_type() -> libc::c_ulong;
}


/*
void g_date_order()
	(GDate *) date1 [struct _GDate *]
	(GDate *) date2 [struct _GDate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_order(date1: *mut _GDate, date2: *mut _GDate);
}


/*
gboolean g_type_test_flags() [int]
	(GType) type [unsigned long]
	(guint) flags [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_test_flags(type_: libc::c_ulong, flags: libc::c_uint) -> libc::c_int;
}


/*
void g_option_context_set_translation_domain()
	(GOptionContext *) context [struct _GOptionContext *]
	(const gchar *) domain [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_context_set_translation_domain(context: *mut _GOptionContext, domain: *const libc::c_char);
}


/*
void g_main_context_invoke()
	(GMainContext *) context [struct _GMainContext *]
	(GSourceFunc) function [int (*)(void *)]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_invoke(context: *mut _GMainContext, function: Option<extern fn(*mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void);
}


/*
void g_object_add_toggle_ref()
	(GObject *) object [struct _GObject *]
	(GToggleNotify) notify [void (*)(void *, struct _GObject *, int)]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_add_toggle_ref(object: *mut _GObject, notify: Option<extern fn(*mut libc::c_void, *mut _GObject, libc::c_int)>, data: *mut libc::c_void);
}


/*
void g_signal_handler_unblock()
	(gpointer) instance [void *]
	(gulong) handler_id [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_handler_unblock(instance: *mut libc::c_void, handler_id: libc::c_ulong);
}


/*
GPrintFunc g_set_print_handler() [void (*)(const char *)]
	(GPrintFunc) func [void (*)(const char *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_set_print_handler(func: Option<extern fn(*const libc::c_char)>) -> Option<extern fn(*const libc::c_char)>;
}


/*
GType g_checksum_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_checksum_get_type() -> libc::c_ulong;
}


/*
gsize g_io_channel_get_buffer_size() [unsigned long]
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_get_buffer_size(channel: *mut _GIOChannel) -> libc::c_ulong;
}


/*
void g_date_set_month()
	(GDate *) date [struct _GDate *]
	(GDateMonth) month [GDateMonth]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_set_month(date: *mut _GDate, month: libc::c_uint);
}


/*
gint g_unichar_digit_value() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_digit_value(c: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_type_check_class_is_a() [int]
	(GTypeClass *) g_class [struct _GTypeClass *]
	(GType) is_a_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_check_class_is_a(g_class: *mut _GTypeClass, is_a_type: libc::c_ulong) -> libc::c_int;
}


/*
gboolean g_bookmark_file_remove_application() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) name [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_remove_application(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, name: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
gint64 g_value_get_int64() [long]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_int64(value: *const _GValue) -> libc::c_long;
}


/*
void g_option_context_set_main_group()
	(GOptionContext *) context [struct _GOptionContext *]
	(GOptionGroup *) group [struct _GOptionGroup *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_context_set_main_group(context: *mut _GOptionContext, group: *mut _GOptionGroup);
}


/*
guint g_thread_pool_get_num_unused_threads() [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_pool_get_num_unused_threads() -> libc::c_uint;
}


/*
gboolean g_test_failed() [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_failed() -> libc::c_int;
}


/*
GList * g_list_concat() [struct _GList *]
	(GList *) list1 [struct _GList *]
	(GList *) list2 [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_concat(list1: *mut _GList, list2: *mut _GList) -> *mut _GList;
}


/*
GTestLogBuffer * g_test_log_buffer_new() [GTestLogBuffer *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_log_buffer_new() -> *mut GTestLogBuffer;
}


/*
GError * g_error_copy() [struct _GError *]
	(const GError *) error [const struct _GError *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_error_copy(error: *const _GError) -> *mut _GError;
}


/*
gchar * g_strdup_printf() [char *]
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strdup_printf(format: *const libc::c_char) -> *mut libc::c_char;
}


/*
GVariant * g_variant_new_from_bytes() [struct _GVariant *]
	(const GVariantType *) type [const struct _GVariantType *]
	(GBytes *) bytes [struct _GBytes *]
	(gboolean) trusted [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_from_bytes(type_: *const _GVariantType, bytes: *mut _GBytes, trusted: libc::c_int) -> *mut _GVariant;
}


/*
void g_test_log_buffer_push()
	(GTestLogBuffer *) tbuffer [GTestLogBuffer *]
	(guint) n_bytes [unsigned int]
	(const guint8 *) bytes [const unsigned char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_log_buffer_push(tbuffer: *mut GTestLogBuffer, n_bytes: libc::c_uint, bytes: *const libc::c_uchar);
}


/*
void g_ptr_array_insert()
	(GPtrArray *) array [struct _GPtrArray *]
	(gint) index_ [int]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_insert(array: *mut _GPtrArray, index_: libc::c_int, data: *mut libc::c_void);
}


/*
gboolean g_unichar_iscntrl() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_iscntrl(c: libc::c_uint) -> libc::c_int;
}


/*
GMappedFile * g_mapped_file_new() [struct _GMappedFile *]
	(const gchar *) filename [const char *]
	(gboolean) writable [int]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mapped_file_new(filename: *const libc::c_char, writable: libc::c_int, error: *mut *mut _GError) -> *mut _GMappedFile;
}


/*
gboolean g_pattern_match_string() [int]
	(GPatternSpec *) pspec [struct _GPatternSpec *]
	(const gchar *) string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_pattern_match_string(pspec: *mut _GPatternSpec, string: *const libc::c_char) -> libc::c_int;
}


/*
GVariantType * g_variant_type_new() [struct _GVariantType *]
	(const gchar *) type_string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_new(type_string: *const libc::c_char) -> *mut _GVariantType;
}


/*
void g_source_set_callback()
	(GSource *) source [struct _GSource *]
	(GSourceFunc) func [int (*)(void *)]
	(gpointer) data [void *]
	(GDestroyNotify) notify [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_set_callback(source: *mut _GSource, func: Option<extern fn(*mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void, notify: Option<extern fn(*mut libc::c_void)>);
}


/*
void g_sequence_sort_iter()
	(GSequence *) seq [struct _GSequence *]
	(GSequenceIterCompareFunc) cmp_func [int (*)(struct _GSequenceNode *, struct _GSequenceNode *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_sort_iter(seq: *mut _GSequence, cmp_func: Option<extern fn(*mut _GSequenceNode, *mut _GSequenceNode, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void);
}


/*
const gchar * g_get_user_config_dir() [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_user_config_dir() -> *const libc::c_char;
}


/*
void g_async_queue_lock()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_lock(queue: *mut _GAsyncQueue);
}


/*
gchar * g_variant_parse_error_print_context() [char *]
	(GError *) error [struct _GError *]
	(const gchar *) source_str [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_parse_error_print_context(error: *mut _GError, source_str: *const libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_utf8_strup() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_strup(str: *const libc::c_char, len: libc::c_long) -> *mut libc::c_char;
}


/*
void g_source_set_can_recurse()
	(GSource *) source [struct _GSource *]
	(gboolean) can_recurse [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_set_can_recurse(source: *mut _GSource, can_recurse: libc::c_int);
}


/*
void g_error()
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_error(format: *const libc::c_char);
}


/*
GType g_type_parent() [unsigned long]
	(GType) type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_parent(type_: libc::c_ulong) -> libc::c_ulong;
}


/*
GQuark g_file_error_quark() [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_file_error_quark() -> libc::c_uint;
}


/*
GIOChannel * g_io_channel_new_file() [struct _GIOChannel *]
	(const gchar *) filename [const char *]
	(const gchar *) mode [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_new_file(filename: *const libc::c_char, mode: *const libc::c_char, error: *mut *mut _GError) -> *mut _GIOChannel;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_assertion_message_error(domain: *const libc::c_char, file: *const libc::c_char, line: libc::c_int, func: *const libc::c_char, expr: *const libc::c_char, error: *const _GError, error_domain: libc::c_uint, error_code: libc::c_int);
}


/*
void g_timer_continue()
	(GTimer *) timer [struct _GTimer *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_timer_continue(timer: *mut _GTimer);
}


/*
GDate * g_date_new() [struct _GDate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_new() -> *mut _GDate;
}


/*
void g_cond_clear()
	(GCond *) cond [struct _GCond *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cond_clear(cond: *mut _GCond);
}


/*
void g_sequence_free()
	(GSequence *) seq [struct _GSequence *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_free(seq: *mut _GSequence);
}


/*
GParamSpec * g_param_spec_flags() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(GType) flags_type [unsigned long]
	(guint) default_value [unsigned int]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_flags(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, flags_type: libc::c_ulong, default_value: libc::c_uint, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
void g_datalist_unset_flags()
	(GData **) datalist [struct _GData **]
	(guint) flags [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_datalist_unset_flags(datalist: *mut *mut _GData, flags: libc::c_uint);
}


/*
gchar * g_utf8_collate_key_for_filename() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_collate_key_for_filename(str: *const libc::c_char, len: libc::c_long) -> *mut libc::c_char;
}


/*
void g_checksum_update()
	(GChecksum *) checksum [struct _GChecksum *]
	(const guchar *) data [const unsigned char *]
	(gssize) length [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_checksum_update(checksum: *mut _GChecksum, data: *const libc::c_uchar, length: libc::c_long);
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_get_locale_string_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, locale: *const libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut *mut libc::c_char;
}


/*
GSource * g_io_create_watch() [struct _GSource *]
	(GIOChannel *) channel [struct _GIOChannel *]
	(GIOCondition) condition [GIOCondition]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_create_watch(channel: *mut _GIOChannel, condition: libc::c_uint) -> *mut _GSource;
}


/*
GMappedFile * g_mapped_file_new_from_fd() [struct _GMappedFile *]
	(gint) fd [int]
	(gboolean) writable [int]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mapped_file_new_from_fd(fd: libc::c_int, writable: libc::c_int, error: *mut *mut _GError) -> *mut _GMappedFile;
}


/*
void g_array_unref()
	(GArray *) array [struct _GArray *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_array_unref(array: *mut _GArray);
}


/*
gint g_date_time_get_week_numbering_year() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_get_week_numbering_year(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
const gchar * g_param_spec_get_blurb() [const char *]
	(GParamSpec *) pspec [struct _GParamSpec *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_get_blurb(pspec: *mut _GParamSpec) -> *const libc::c_char;
}


/*
GSList * g_slist_remove_all() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(gconstpointer) data [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_remove_all(list: *mut _GSList, data: *const libc::c_void) -> *mut _GSList;
}


/*
void g_bit_unlock()
	(volatile gint *) address [volatile int *]
	(gint) lock_bit [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bit_unlock(address: *mut libc::c_int, lock_bit: libc::c_int);
}


/*
void g_queue_delete_link()
	(GQueue *) queue [struct _GQueue *]
	(GList *) link_ [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_delete_link(queue: *mut _GQueue, link_: *mut _GList);
}


/*
const gchar ** g_variant_get_bytestring_array() [const char **]
	(GVariant *) value [struct _GVariant *]
	(gsize *) length [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_bytestring_array(value: *mut _GVariant, length: *mut libc::c_ulong) -> *mut *const libc::c_char;
}


/*
void g_queue_push_nth_link()
	(GQueue *) queue [struct _GQueue *]
	(gint) n [int]
	(GList *) link_ [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_push_nth_link(queue: *mut _GQueue, n: libc::c_int, link_: *mut _GList);
}


/*
guint g_io_add_watch() [unsigned int]
	(GIOChannel *) channel [struct _GIOChannel *]
	(GIOCondition) condition [GIOCondition]
	(GIOFunc) func [int (*)(struct _GIOChannel *, GIOCondition, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_add_watch(channel: *mut _GIOChannel, condition: libc::c_uint, func: Option<extern fn(*mut _GIOChannel, libc::c_uint, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void) -> libc::c_uint;
}


/*
void g_cclosure_marshal_VOID__BOXEDv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_cclosure_marshal_VOID__BOXEDv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
GVariant * g_variant_new_bytestring() [struct _GVariant *]
	(const gchar *) string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_bytestring(string: *const libc::c_char) -> *mut _GVariant;
}


/*
void g_markup_parse_context_unref()
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_markup_parse_context_unref(context: *mut _GMarkupParseContext);
}


/*
void g_list_foreach()
	(GList *) list [struct _GList *]
	(GFunc) func [void (*)(void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_foreach(list: *mut _GList, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
gint g_node_child_index() [int]
	(GNode *) node [struct _GNode *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_child_index(node: *mut _GNode, data: *mut libc::c_void) -> libc::c_int;
}


/*
gboolean g_hostname_is_ip_address() [int]
	(const gchar *) hostname [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hostname_is_ip_address(hostname: *const libc::c_char) -> libc::c_int;
}


/*
void g_completion_clear_items()
	(GCompletion *) cmp [struct _GCompletion *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_completion_clear_items(cmp: *mut _GCompletion);
}


/*
void g_mutex_lock()
	(GMutex *) mutex [union _GMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mutex_lock(mutex: *mut _GMutex);
}


/*
void g_key_file_unref()
	(GKeyFile *) key_file [struct _GKeyFile *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_unref(key_file: *mut _GKeyFile);
}


/*
gboolean g_variant_dict_remove() [int]
	(GVariantDict *) dict [struct _GVariantDict *]
	(const gchar *) key [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_dict_remove(dict: *mut _GVariantDict, key: *const libc::c_char) -> libc::c_int;
}


/*
GType g_pollfd_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_pollfd_get_type() -> libc::c_ulong;
}


/*
gboolean g_str_has_suffix() [int]
	(const gchar *) str [const char *]
	(const gchar *) suffix [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_str_has_suffix(str: *const libc::c_char, suffix: *const libc::c_char) -> libc::c_int;
}


/*
GTuples * g_relation_select() [struct _GTuples *]
	(GRelation *) relation [struct _GRelation *]
	(gconstpointer) key [const void *]
	(gint) field [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_relation_select(relation: *mut _GRelation, key: *const libc::c_void, field: libc::c_int) -> *mut _GTuples;
}


/*
GVariant * g_variant_new_boolean() [struct _GVariant *]
	(gboolean) value [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_boolean(value: libc::c_int) -> *mut _GVariant;
}


/*
gboolean g_unichar_islower() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_islower(c: libc::c_uint) -> libc::c_int;
}


/*
gpointer g_source_add_unix_fd() [void *]
	(GSource *) source [struct _GSource *]
	(gint) fd [int]
	(GIOCondition) events [GIOCondition]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_add_unix_fd(source: *mut _GSource, fd: libc::c_int, events: libc::c_uint) -> *mut libc::c_void;
}


/*
void g_list_pop_allocator()
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_pop_allocator();
}


/*
void g_date_set_year()
	(GDate *) date [struct _GDate *]
	(GDateYear) year [unsigned short]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_set_year(date: *mut _GDate, year: libc::c_ushort);
}


/*
GParamSpecPool * g_param_spec_pool_new() [struct _GParamSpecPool *]
	(gboolean) type_prefixing [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_pool_new(type_prefixing: libc::c_int) -> *mut _GParamSpecPool;
}


/*
GSequenceIter * g_sequence_insert_sorted() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
	(gpointer) data [void *]
	(GCompareDataFunc) cmp_func [int (*)(const void *, const void *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_insert_sorted(seq: *mut _GSequence, data: *mut libc::c_void, cmp_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void) -> *mut _GSequenceNode;
}


/*
void g_value_set_boxed_take_ownership()
	(GValue *) value [struct _GValue *]
	(gconstpointer) v_boxed [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_boxed_take_ownership(value: *mut _GValue, v_boxed: *const libc::c_void);
}


/*
gint16 g_variant_get_int16() [short]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_int16(value: *mut _GVariant) -> libc::c_short;
}


/*
guint8 g_date_get_monday_weeks_in_year() [unsigned char]
	(GDateYear) year [unsigned short]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_get_monday_weeks_in_year(year: libc::c_ushort) -> libc::c_uchar;
}


/*
GSequenceIter * g_sequence_iter_next() [struct _GSequenceNode *]
	(GSequenceIter *) iter [struct _GSequenceNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_iter_next(iter: *mut _GSequenceNode) -> *mut _GSequenceNode;
}


/*
GPtrArray * g_ptr_array_sized_new() [struct _GPtrArray *]
	(guint) reserved_size [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_sized_new(reserved_size: libc::c_uint) -> *mut _GPtrArray;
}


/*
void g_object_remove_toggle_ref()
	(GObject *) object [struct _GObject *]
	(GToggleNotify) notify [void (*)(void *, struct _GObject *, int)]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_remove_toggle_ref(object: *mut _GObject, notify: Option<extern fn(*mut libc::c_void, *mut _GObject, libc::c_int)>, data: *mut libc::c_void);
}


/*
guint g_int_hash() [unsigned int]
	(gconstpointer) v [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_int_hash(v: *const libc::c_void) -> libc::c_uint;
}


/*
gpointer g_type_default_interface_peek() [void *]
	(GType) g_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_default_interface_peek(g_type: libc::c_ulong) -> *mut libc::c_void;
}


/*
void g_source_set_name()
	(GSource *) source [struct _GSource *]
	(const char *) name
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_set_name(source: *mut _GSource, name: *const libc::c_char);
}


/*
gboolean g_variant_check_format_string() [int]
	(GVariant *) value [struct _GVariant *]
	(const gchar *) format_string [const char *]
	(gboolean) copy_only [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_check_format_string(value: *mut _GVariant, format_string: *const libc::c_char, copy_only: libc::c_int) -> libc::c_int;
}


/*
GIOError g_io_channel_seek() [GIOError]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gint64) offset [long]
	(GSeekType) type [GSeekType]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_seek(channel: *mut _GIOChannel, offset: libc::c_long, type_: libc::c_uint) -> libc::c_uint;
}


/*
GSequenceIter * g_sequence_search() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
	(gpointer) data [void *]
	(GCompareDataFunc) cmp_func [int (*)(const void *, const void *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_search(seq: *mut _GSequence, data: *mut libc::c_void, cmp_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void) -> *mut _GSequenceNode;
}


/*
void g_cclosure_marshal_VOID__UINT_POINTER()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_VOID__UINT_POINTER(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
void g_type_class_unref()
	(gpointer) g_class [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_class_unref(g_class: *mut libc::c_void);
}


/*
void g_value_set_int()
	(GValue *) value [struct _GValue *]
	(gint) v_int [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_int(value: *mut _GValue, v_int: libc::c_int);
}


/*
void g_mapped_file_unref()
	(GMappedFile *) file [struct _GMappedFile *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mapped_file_unref(file: *mut _GMappedFile);
}


/*
void g_cclosure_marshal_VOID__ENUMv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_cclosure_marshal_VOID__ENUMv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
gboolean g_match_info_is_partial_match() [int]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_match_info_is_partial_match(match_info: *const _GMatchInfo) -> libc::c_int;
}


/*
GLogLevelFlags g_log_set_always_fatal() [GLogLevelFlags]
	(GLogLevelFlags) fatal_mask [GLogLevelFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_log_set_always_fatal(fatal_mask: libc::c_uint) -> libc::c_uint;
}


/*
void g_date_subtract_days()
	(GDate *) date [struct _GDate *]
	(guint) n_days [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_subtract_days(date: *mut _GDate, n_days: libc::c_uint);
}


/*
void g_timer_stop()
	(GTimer *) timer [struct _GTimer *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_timer_stop(timer: *mut _GTimer);
}


/*
void g_test_log_msg_free()
	(GTestLogMsg *) tmsg [GTestLogMsg *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_log_msg_free(tmsg: *mut GTestLogMsg);
}


/*
GSignalInvocationHint * g_signal_get_invocation_hint() [struct _GSignalInvocationHint *]
	(gpointer) instance [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_get_invocation_hint(instance: *mut libc::c_void) -> *mut _GSignalInvocationHint;
}


/*
GTypePlugin * g_type_get_plugin() [struct _GTypePlugin *]
	(GType) type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_get_plugin(type_: libc::c_ulong) -> *mut _GTypePlugin;
}


/*
gunichar g_utf8_get_char() [unsigned int]
	(const gchar *) p [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_get_char(p: *const libc::c_char) -> libc::c_uint;
}


/*
void g_type_class_adjust_private_offset()
	(gpointer) g_class [void *]
	(gint *) private_size_or_offset [int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_class_adjust_private_offset(g_class: *mut libc::c_void, private_size_or_offset: *mut libc::c_int);
}


/*
gboolean g_variant_is_floating() [int]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_is_floating(value: *mut _GVariant) -> libc::c_int;
}


/*
gchar g_ascii_toupper() [char]
	(gchar) c [char]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ascii_toupper(c: libc::c_char) -> libc::c_char;
}


/*
GType g_flags_register_static() [unsigned long]
	(const gchar *) name [const char *]
	(const GFlagsValue *) const_static_values [const struct _GFlagsValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_flags_register_static(name: *const libc::c_char, const_static_values: *const _GFlagsValue) -> libc::c_ulong;
}


/*
void g_cclosure_marshal_VOID__FLAGS()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_VOID__FLAGS(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
void g_mem_chunk_free()
	(GMemChunk *) mem_chunk [struct _GMemChunk *]
	(gpointer) mem [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mem_chunk_free(mem_chunk: *mut _GMemChunk, mem: *mut libc::c_void);
}


/*
gboolean g_variant_type_is_maybe() [int]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_is_maybe(type_: *const _GVariantType) -> libc::c_int;
}


/*
guchar * g_base64_decode_inplace() [unsigned char *]
	(gchar *) text [char *]
	(gsize *) out_len [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_base64_decode_inplace(text: *mut libc::c_char, out_len: *mut libc::c_ulong) -> *mut libc::c_uchar;
}


/*
void g_type_add_class_cache_func()
	(gpointer) cache_data [void *]
	(GTypeClassCacheFunc) cache_func [int (*)(void *, struct _GTypeClass *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_add_class_cache_func(cache_data: *mut libc::c_void, cache_func: Option<extern fn(*mut libc::c_void, *mut _GTypeClass) -> libc::c_int>);
}


/*
void g_prefix_error()
	(GError **) err [struct _GError **]
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_prefix_error(err: *mut *mut _GError, format: *const libc::c_char);
}


/*
GList * g_hash_table_get_values() [struct _GList *]
	(GHashTable *) hash_table [struct _GHashTable *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_get_values(hash_table: *mut _GHashTable) -> *mut _GList;
}


/*
GParamSpec * g_object_interface_find_property() [struct _GParamSpec *]
	(gpointer) g_iface [void *]
	(const gchar *) property_name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_interface_find_property(g_iface: *mut libc::c_void, property_name: *const libc::c_char) -> *mut _GParamSpec;
}


/*
GMainContext * g_main_context_get_thread_default() [struct _GMainContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_get_thread_default() -> *mut _GMainContext;
}


/*
GSList * g_slist_insert_sorted() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(gpointer) data [void *]
	(GCompareFunc) func [int (*)(const void *, const void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_insert_sorted(list: *mut _GSList, data: *mut libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GSList;
}


/*
guint g_hash_table_foreach_steal() [unsigned int]
	(GHashTable *) hash_table [struct _GHashTable *]
	(GHRFunc) func [int (*)(void *, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_foreach_steal(hash_table: *mut _GHashTable, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void) -> libc::c_uint;
}


/*
void g_cclosure_marshal_generic()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_gvalue [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_generic(closure: *mut _GClosure, return_gvalue: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
const gchar ** g_variant_get_objv() [const char **]
	(GVariant *) value [struct _GVariant *]
	(gsize *) length [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_objv(value: *mut _GVariant, length: *mut libc::c_ulong) -> *mut *const libc::c_char;
}


/*
gsize g_base64_encode_close() [unsigned long]
	(gboolean) break_lines [int]
	(gchar *) out [char *]
	(gint *) state [int *]
	(gint *) save [int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_base64_encode_close(break_lines: libc::c_int, out: *mut libc::c_char, state: *mut libc::c_int, save: *mut libc::c_int) -> libc::c_ulong;
}


/*
void g_mem_set_vtable()
	(GMemVTable *) vtable [struct _GMemVTable *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mem_set_vtable(vtable: *mut _GMemVTable);
}


/*
gchar * g_path_get_dirname() [char *]
	(const gchar *) file_name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_path_get_dirname(file_name: *const libc::c_char) -> *mut libc::c_char;
}


/*
void g_option_context_set_description()
	(GOptionContext *) context [struct _GOptionContext *]
	(const gchar *) description [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_context_set_description(context: *mut _GOptionContext, description: *const libc::c_char);
}


/*
guint16 g_variant_get_uint16() [unsigned short]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_uint16(value: *mut _GVariant) -> libc::c_ushort;
}


/*
gboolean g_pattern_match_simple() [int]
	(const gchar *) pattern [const char *]
	(const gchar *) string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_pattern_match_simple(pattern: *const libc::c_char, string: *const libc::c_char) -> libc::c_int;
}


/*
void g_print()
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_print(format: *const libc::c_char);
}


/*
void g_bookmark_file_set_visited()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(time_t) visited [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_set_visited(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, visited: libc::c_long);
}


/*
GVariant * g_variant_dict_end() [struct _GVariant *]
	(GVariantDict *) dict [struct _GVariantDict *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_dict_end(dict: *mut _GVariantDict) -> *mut _GVariant;
}


/*
gchar * g_strcanon() [char *]
	(gchar *) string [char *]
	(const gchar *) valid_chars [const char *]
	(gchar) substitutor [char]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strcanon(string: *mut libc::c_char, valid_chars: *const libc::c_char, substitutor: libc::c_char) -> *mut libc::c_char;
}


/*
GList * g_list_insert() [struct _GList *]
	(GList *) list [struct _GList *]
	(gpointer) data [void *]
	(gint) position [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_insert(list: *mut _GList, data: *mut libc::c_void, position: libc::c_int) -> *mut _GList;
}


/*
void g_assert_warning()
	(const char *) log_domain
	(const char *) file
	(const int) line
	(const char *) pretty_function
	(const char *) expression
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_assert_warning(log_domain: *const libc::c_char, file: *const libc::c_char, line: libc::c_int, pretty_function: *const libc::c_char, expression: *const libc::c_char);
}


/*
gchar * g_build_filenamev() [char *]
	(gchar **) args [char **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_build_filenamev(args: *mut *mut libc::c_char) -> *mut libc::c_char;
}


/*
const gchar * g_test_get_dir() [const char *]
	(GTestFileType) file_type [GTestFileType]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_get_dir(file_type: libc::c_uint) -> *const libc::c_char;
}


/*
GString * g_string_append_uri_escaped() [struct _GString *]
	(GString *) string [struct _GString *]
	(const gchar *) unescaped [const char *]
	(const gchar *) reserved_chars_allowed [const char *]
	(gboolean) allow_utf8 [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_append_uri_escaped(string: *mut _GString, unescaped: *const libc::c_char, reserved_chars_allowed: *const libc::c_char, allow_utf8: libc::c_int) -> *mut _GString;
}


/*
guint g_queue_get_length() [unsigned int]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_get_length(queue: *mut _GQueue) -> libc::c_uint;
}


/*
gconstpointer g_variant_get_data() [const void *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_data(value: *mut _GVariant) -> *const libc::c_void;
}


/*
void g_variant_dict_insert()
	(GVariantDict *) dict [struct _GVariantDict *]
	(const gchar *) key [const char *]
	(const gchar *) format_string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_dict_insert(dict: *mut _GVariantDict, key: *const libc::c_char, format_string: *const libc::c_char);
}


/*
gchar * g_utf8_strreverse() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_strreverse(str: *const libc::c_char, len: libc::c_long) -> *mut libc::c_char;
}


/*
GTimeSpan g_date_time_get_utc_offset() [long]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_get_utc_offset(datetime: *mut _GDateTime) -> libc::c_long;
}


/*
gboolean g_idle_remove_by_data() [int]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_idle_remove_by_data(data: *mut libc::c_void) -> libc::c_int;
}


/*
gboolean g_mutex_trylock() [int]
	(GMutex *) mutex [union _GMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mutex_trylock(mutex: *mut _GMutex) -> libc::c_int;
}


/*
gboolean g_variant_is_normal_form() [int]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_is_normal_form(value: *mut _GVariant) -> libc::c_int;
}


/*
GFlagsValue * g_flags_get_first_value() [struct _GFlagsValue *]
	(GFlagsClass *) flags_class [struct _GFlagsClass *]
	(guint) value [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_flags_get_first_value(flags_class: *mut _GFlagsClass, value: libc::c_uint) -> *mut _GFlagsValue;
}


/*
gboolean g_static_rw_lock_writer_trylock() [int]
	(GStaticRWLock *) lock [struct _GStaticRWLock *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_rw_lock_writer_trylock(lock: *mut _GStaticRWLock) -> libc::c_int;
}


/*
void g_weak_ref_clear()
	(GWeakRef *) weak_ref [GWeakRef *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_weak_ref_clear(weak_ref: *mut GWeakRef);
}


/*
void g_key_file_set_list_separator()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(gchar) separator [char]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_set_list_separator(key_file: *mut _GKeyFile, separator: libc::c_char);
}


/*
gdouble g_rand_double_range() [double]
	(GRand *) rand_ [struct _GRand *]
	(gdouble) begin [double]
	(gdouble) end [double]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rand_double_range(rand_: *mut _GRand, begin: libc::c_double, end: libc::c_double) -> libc::c_double;
}


/*
gchar * g_test_build_filename() [char *]
	(GTestFileType) file_type [GTestFileType]
	(const gchar *) first_path [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_build_filename(file_type: libc::c_uint, first_path: *const libc::c_char) -> *mut libc::c_char;
}


/*
GType g_variant_builder_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_builder_get_type() -> libc::c_ulong;
}


/*
void g_option_context_add_main_entries()
	(GOptionContext *) context [struct _GOptionContext *]
	(const GOptionEntry *) entries [const struct _GOptionEntry *]
	(const gchar *) translation_domain [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_context_add_main_entries(context: *mut _GOptionContext, entries: *const _GOptionEntry, translation_domain: *const libc::c_char);
}


/*
gint g_list_index() [int]
	(GList *) list [struct _GList *]
	(gconstpointer) data [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_index(list: *mut _GList, data: *const libc::c_void) -> libc::c_int;
}


/*
gboolean g_thread_pool_push() [int]
	(GThreadPool *) pool [struct _GThreadPool *]
	(gpointer) data [void *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_pool_push(pool: *mut _GThreadPool, data: *mut libc::c_void, error: *mut *mut _GError) -> libc::c_int;
}


/*
GString * g_string_prepend_c() [struct _GString *]
	(GString *) string [struct _GString *]
	(gchar) c [char]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_prepend_c(string: *mut _GString, c: libc::c_char) -> *mut _GString;
}


/*
void g_cclosure_marshal_VOID__VOIDv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_cclosure_marshal_VOID__VOIDv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
const gchar * g_get_home_dir() [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_home_dir() -> *const libc::c_char;
}


/*
void g_sequence_move_range()
	(GSequenceIter *) dest [struct _GSequenceNode *]
	(GSequenceIter *) begin [struct _GSequenceNode *]
	(GSequenceIter *) end [struct _GSequenceNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_move_range(dest: *mut _GSequenceNode, begin: *mut _GSequenceNode, end: *mut _GSequenceNode);
}


/*
gboolean g_key_file_load_from_data() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) data [const char *]
	(gsize) length [unsigned long]
	(GKeyFileFlags) flags [GKeyFileFlags]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_load_from_data(key_file: *mut _GKeyFile, data: *const libc::c_char, length: libc::c_ulong, flags: libc::c_uint, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_completion_set_compare()
	(GCompletion *) cmp [struct _GCompletion *]
	(GCompletionStrncmpFunc) strncmp_func [int (*)(const char *, const char *, unsigned long)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_completion_set_compare(cmp: *mut _GCompletion, strncmp_func: Option<extern fn(*const libc::c_char, *const libc::c_char, libc::c_ulong) -> libc::c_int>);
}


/*
GAsyncQueue * g_async_queue_new_full() [struct _GAsyncQueue *]
	(GDestroyNotify) item_free_func [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_new_full(item_free_func: Option<extern fn(*mut libc::c_void)>) -> *mut _GAsyncQueue;
}


/*
GValue * g_value_reset() [struct _GValue *]
	(GValue *) value [struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_reset(value: *mut _GValue) -> *mut _GValue;
}


/*
gboolean g_date_is_leap_year() [int]
	(GDateYear) year [unsigned short]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_is_leap_year(year: libc::c_ushort) -> libc::c_int;
}


/*
GType g_type_module_register_enum() [unsigned long]
	(GTypeModule *) module [struct _GTypeModule *]
	(const gchar *) name [const char *]
	(const GEnumValue *) const_static_values [const struct _GEnumValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_module_register_enum(module: *mut _GTypeModule, name: *const libc::c_char, const_static_values: *const _GEnumValue) -> libc::c_ulong;
}


/*
GVariant * g_variant_byteswap() [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_byteswap(value: *mut _GVariant) -> *mut _GVariant;
}


/*
GBytes * g_byte_array_free_to_bytes() [struct _GBytes *]
	(GByteArray *) array [struct _GByteArray *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_byte_array_free_to_bytes(array: *mut _GByteArray) -> *mut _GBytes;
}


/*
void g_signal_stop_emission_by_name()
	(gpointer) instance [void *]
	(const gchar *) detailed_signal [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_stop_emission_by_name(instance: *mut libc::c_void, detailed_signal: *const libc::c_char);
}


/*
GThread * g_thread_new() [struct _GThread *]
	(const gchar *) name [const char *]
	(GThreadFunc) func [void *(*)(void *)]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_new(name: *const libc::c_char, func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>, data: *mut libc::c_void) -> *mut _GThread;
}


/*
gboolean g_signal_accumulator_first_wins() [int]
	(GSignalInvocationHint *) ihint [struct _GSignalInvocationHint *]
	(GValue *) return_accu [struct _GValue *]
	(const GValue *) handler_return [const struct _GValue *]
	(gpointer) dummy [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_accumulator_first_wins(ihint: *mut _GSignalInvocationHint, return_accu: *mut _GValue, handler_return: *const _GValue, dummy: *mut libc::c_void) -> libc::c_int;
}


/*
void g_date_set_time_val()
	(GDate *) date [struct _GDate *]
	(GTimeVal *) timeval [struct _GTimeVal *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_set_time_val(date: *mut _GDate, timeval: *mut _GTimeVal);
}


/*
gboolean g_variant_type_is_subtype_of() [int]
	(const GVariantType *) type [const struct _GVariantType *]
	(const GVariantType *) supertype [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_is_subtype_of(type_: *const _GVariantType, supertype: *const _GVariantType) -> libc::c_int;
}


/*
void g_relation_destroy()
	(GRelation *) relation [struct _GRelation *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_relation_destroy(relation: *mut _GRelation);
}


/*
gboolean g_regex_check_replacement() [int]
	(const gchar *) replacement [const char *]
	(gboolean *) has_references [int *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_check_replacement(replacement: *const libc::c_char, has_references: *mut libc::c_int, error: *mut *mut _GError) -> libc::c_int;
}


/*
guchar * g_base64_decode() [unsigned char *]
	(const gchar *) text [const char *]
	(gsize *) out_len [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_base64_decode(text: *const libc::c_char, out_len: *mut libc::c_ulong) -> *mut libc::c_uchar;
}


/*
gboolean g_file_test() [int]
	(const gchar *) filename [const char *]
	(GFileTest) test [GFileTest]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_file_test(filename: *const libc::c_char, test: libc::c_uint) -> libc::c_int;
}


/*
gboolean g_thread_pool_set_max_threads() [int]
	(GThreadPool *) pool [struct _GThreadPool *]
	(gint) max_threads [int]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_pool_set_max_threads(pool: *mut _GThreadPool, max_threads: libc::c_int, error: *mut *mut _GError) -> libc::c_int;
}


/*
gchar * g_utf8_find_next_char() [char *]
	(const gchar *) p [const char *]
	(const gchar *) end [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_find_next_char(p: *const libc::c_char, end: *const libc::c_char) -> *mut libc::c_char;
}


/*
gboolean g_unichar_isspace() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_isspace(c: libc::c_uint) -> libc::c_int;
}


/*
gunichar * g_utf8_to_ucs4() [unsigned int *]
	(const gchar *) str [const char *]
	(glong) len [long]
	(glong *) items_read [long *]
	(glong *) items_written [long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_to_ucs4(str: *const libc::c_char, len: libc::c_long, items_read: *mut libc::c_long, items_written: *mut libc::c_long, error: *mut *mut _GError) -> *mut libc::c_uint;
}


/*
GNode * g_node_first_sibling() [struct _GNode *]
	(GNode *) node [struct _GNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_first_sibling(node: *mut _GNode) -> *mut _GNode;
}


/*
void g_hook_list_init()
	(GHookList *) hook_list [struct _GHookList *]
	(guint) hook_size [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_list_init(hook_list: *mut _GHookList, hook_size: libc::c_uint);
}


/*
gpointer g_tree_search() [void *]
	(GTree *) tree [struct _GTree *]
	(GCompareFunc) search_func [int (*)(const void *, const void *)]
	(gconstpointer) user_data [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_tree_search(tree: *mut _GTree, search_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>, user_data: *const libc::c_void) -> *mut libc::c_void;
}


/*
GString * g_string_insert_len() [struct _GString *]
	(GString *) string [struct _GString *]
	(gssize) pos [long]
	(const gchar *) val [const char *]
	(gssize) len [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_insert_len(string: *mut _GString, pos: libc::c_long, val: *const libc::c_char, len: libc::c_long) -> *mut _GString;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cache_new(value_new_func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>, value_destroy_func: Option<extern fn(*mut libc::c_void)>, key_dup_func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>, key_destroy_func: Option<extern fn(*mut libc::c_void)>, hash_key_func: Option<extern fn(*const libc::c_void) -> libc::c_uint>, hash_value_func: Option<extern fn(*const libc::c_void) -> libc::c_uint>, key_equal_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GCache;
}


/*
const gchar * g_value_get_string() [const char *]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_string(value: *const _GValue) -> *const libc::c_char;
}


/*
void g_cclosure_marshal_VOID__FLOATv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_cclosure_marshal_VOID__FLOATv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_set_locale_string_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, locale: *const libc::c_char, list: *mut *const libc::c_char /* INCOMPLETEARRAY */, length: libc::c_ulong);
}


/*
void g_source_set_ready_time()
	(GSource *) source [struct _GSource *]
	(gint64) ready_time [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_set_ready_time(source: *mut _GSource, ready_time: libc::c_long);
}


/*
gint g_utf8_collate() [int]
	(const gchar *) str1 [const char *]
	(const gchar *) str2 [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_collate(str1: *const libc::c_char, str2: *const libc::c_char) -> libc::c_int;
}


/*
GTypeClass * g_type_check_class_cast() [struct _GTypeClass *]
	(GTypeClass *) g_class [struct _GTypeClass *]
	(GType) is_a_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_check_class_cast(g_class: *mut _GTypeClass, is_a_type: libc::c_ulong) -> *mut _GTypeClass;
}


/*
void g_cclosure_marshal_VOID__FLAGSv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_cclosure_marshal_VOID__FLAGSv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
guint g_date_get_iso8601_week_of_year() [unsigned int]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_get_iso8601_week_of_year(date: *const _GDate) -> libc::c_uint;
}


/*
void g_bytes_unref()
	(GBytes *) bytes [struct _GBytes *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bytes_unref(bytes: *mut _GBytes);
}


/*
void g_scanner_scope_foreach_symbol()
	(GScanner *) scanner [struct _GScanner *]
	(guint) scope_id [unsigned int]
	(GHFunc) func [void (*)(void *, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_scope_foreach_symbol(scanner: *mut _GScanner, scope_id: libc::c_uint, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
gint g_atomic_int_get() [int]
	(const volatile gint *) atomic [const volatile int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_atomic_int_get(atomic: *const libc::c_int) -> libc::c_int;
}


/*
gboolean g_main_context_pending() [int]
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_pending(context: *mut _GMainContext) -> libc::c_int;
}


/*
void g_spawn_close_pid()
	(GPid) pid [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_spawn_close_pid(pid: libc::c_int);
}


/*
gchar * g_string_free() [char *]
	(GString *) string [struct _GString *]
	(gboolean) free_segment [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_free(string: *mut _GString, free_segment: libc::c_int) -> *mut libc::c_char;
}


/*
GArray * g_array_new() [struct _GArray *]
	(gboolean) zero_terminated [int]
	(gboolean) clear_ [int]
	(guint) element_size [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_array_new(zero_terminated: libc::c_int, clear_: libc::c_int, element_size: libc::c_uint) -> *mut _GArray;
}


/*
GMainContext * g_main_context_ref_thread_default() [struct _GMainContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_ref_thread_default() -> *mut _GMainContext;
}


/*
GIOFlags g_io_channel_get_flags() [GIOFlags]
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_get_flags(channel: *mut _GIOChannel) -> libc::c_uint;
}


/*
void g_sequence_remove_range()
	(GSequenceIter *) begin [struct _GSequenceNode *]
	(GSequenceIter *) end [struct _GSequenceNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_remove_range(begin: *mut _GSequenceNode, end: *mut _GSequenceNode);
}


/*
GScanner * g_scanner_new() [struct _GScanner *]
	(const GScannerConfig *) config_templ [const struct _GScannerConfig *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_new(config_templ: *const _GScannerConfig) -> *mut _GScanner;
}


/*
void g_value_set_ulong()
	(GValue *) value [struct _GValue *]
	(gulong) v_ulong [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_ulong(value: *mut _GValue, v_ulong: libc::c_ulong);
}


/*
GIOCondition g_source_query_unix_fd() [GIOCondition]
	(GSource *) source [struct _GSource *]
	(gpointer) tag [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_query_unix_fd(source: *mut _GSource, tag: *mut libc::c_void) -> libc::c_uint;
}


/*
void g_test_add_data_func_full()
	(const char *) testpath
	(gpointer) test_data [void *]
	(GTestDataFunc) test_func [void (*)(const void *)]
	(GDestroyNotify) data_free_func [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_add_data_func_full(testpath: *const libc::c_char, test_data: *mut libc::c_void, test_func: Option<extern fn(*const libc::c_void)>, data_free_func: Option<extern fn(*mut libc::c_void)>);
}


/*
GTree * g_tree_new_with_data() [struct _GTree *]
	(GCompareDataFunc) key_compare_func [int (*)(const void *, const void *, void *)]
	(gpointer) key_compare_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_tree_new_with_data(key_compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, key_compare_data: *mut libc::c_void) -> *mut _GTree;
}


/*
gchar ** g_environ_setenv() [char **]
	(gchar **) envp [char **]
	(const gchar *) variable [const char *]
	(const gchar *) value [const char *]
	(gboolean) overwrite [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_environ_setenv(envp: *mut *mut libc::c_char, variable: *const libc::c_char, value: *const libc::c_char, overwrite: libc::c_int) -> *mut *mut libc::c_char;
}


/*
gboolean g_unichar_iswide_cjk() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_iswide_cjk(c: libc::c_uint) -> libc::c_int;
}


/*
GTestSuite * g_test_create_suite() [struct GTestSuite *]
	(const char *) suite_name
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_create_suite(suite_name: *const libc::c_char) -> *mut GTestSuite;
}


/*
gint64 g_source_get_time() [long]
	(GSource *) source [struct _GSource *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_get_time(source: *mut _GSource) -> libc::c_long;
}


/*
void g_variant_builder_open()
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_builder_open(builder: *mut _GVariantBuilder, type_: *const _GVariantType);
}


/*
void g_async_queue_sort_unlocked()
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(GCompareDataFunc) func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_sort_unlocked(queue: *mut _GAsyncQueue, func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
void g_value_set_static_string()
	(GValue *) value [struct _GValue *]
	(const gchar *) v_string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_static_string(value: *mut _GValue, v_string: *const libc::c_char);
}


/*
GDateTime * g_date_time_add_days() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
	(gint) days [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_add_days(datetime: *mut _GDateTime, days: libc::c_int) -> *mut _GDateTime;
}


/*
GSequenceIter * g_sequence_lookup() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
	(gpointer) data [void *]
	(GCompareDataFunc) cmp_func [int (*)(const void *, const void *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_lookup(seq: *mut _GSequence, data: *mut libc::c_void, cmp_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void) -> *mut _GSequenceNode;
}


/*
gchar * g_strdup_value_contents() [char *]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strdup_value_contents(value: *const _GValue) -> *mut libc::c_char;
}


/*
GBytes * g_bytes_ref() [struct _GBytes *]
	(GBytes *) bytes [struct _GBytes *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bytes_ref(bytes: *mut _GBytes) -> *mut _GBytes;
}


/*
gpointer g_try_realloc_n() [void *]
	(gpointer) mem [void *]
	(gsize) n_blocks [unsigned long]
	(gsize) n_block_bytes [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_try_realloc_n(mem: *mut libc::c_void, n_blocks: libc::c_ulong, n_block_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
gchar * g_shell_quote() [char *]
	(const gchar *) unquoted_string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_shell_quote(unquoted_string: *const libc::c_char) -> *mut libc::c_char;
}


/*
gint64 g_date_time_to_unix() [long]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_to_unix(datetime: *mut _GDateTime) -> libc::c_long;
}


/*
guint g_signal_new() [unsigned int]
	(const gchar *) signal_name [const char *]
	(GType) itype [unsigned long]
	(GSignalFlags) signal_flags [GSignalFlags]
	(guint) class_offset [unsigned int]
	(GSignalAccumulator) accumulator [int (*)(struct _GSignalInvocationHint *, struct _GValue *, const struct _GValue *, void *)]
	(gpointer) accu_data [void *]
	(GSignalCMarshaller) c_marshaller [void (*)(struct _GClosure *, struct _GValue *, unsigned int, const struct _GValue *, void *, void *)]
	(GType) return_type [unsigned long]
	(guint) n_params [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_new(signal_name: *const libc::c_char, itype: libc::c_ulong, signal_flags: libc::c_uint, class_offset: libc::c_uint, accumulator: Option<extern fn(*mut _GSignalInvocationHint, *mut _GValue, *const _GValue, *mut libc::c_void) -> libc::c_int>, accu_data: *mut libc::c_void, c_marshaller: Option<extern fn(*mut _GClosure, *mut _GValue, libc::c_uint, *const _GValue, *mut libc::c_void, *mut libc::c_void)>, return_type: libc::c_ulong, n_params: libc::c_uint) -> libc::c_uint;
}


/*
int g_strcmp0()
	(const char *) str1
	(const char *) str2
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strcmp0(str1: *const libc::c_char, str2: *const libc::c_char) -> libc::c_int;
}


/*
void g_value_set_string()
	(GValue *) value [struct _GValue *]
	(const gchar *) v_string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_string(value: *mut _GValue, v_string: *const libc::c_char);
}


/*
gpointer g_try_malloc0() [void *]
	(gsize) n_bytes [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_try_malloc0(n_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
GByteArray * g_byte_array_remove_range() [struct _GByteArray *]
	(GByteArray *) array [struct _GByteArray *]
	(guint) index_ [unsigned int]
	(guint) length [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_byte_array_remove_range(array: *mut _GByteArray, index_: libc::c_uint, length: libc::c_uint) -> *mut _GByteArray;
}


/*
void g_value_set_uint64()
	(GValue *) value [struct _GValue *]
	(guint64) v_uint64 [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_uint64(value: *mut _GValue, v_uint64: libc::c_ulong);
}


/*
gchar * g_key_file_get_value() [char *]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_get_value(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
void g_closure_remove_finalize_notifier()
	(GClosure *) closure [struct _GClosure *]
	(gpointer) notify_data [void *]
	(GClosureNotify) notify_func [void (*)(void *, struct _GClosure *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_closure_remove_finalize_notifier(closure: *mut _GClosure, notify_data: *mut libc::c_void, notify_func: Option<extern fn(*mut libc::c_void, *mut _GClosure)>);
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_new_local(year: libc::c_int, month: libc::c_int, day: libc::c_int, hour: libc::c_int, minute: libc::c_int, seconds: libc::c_double) -> *mut _GDateTime;
}


/*
GPatternSpec * g_pattern_spec_new() [struct _GPatternSpec *]
	(const gchar *) pattern [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_pattern_spec_new(pattern: *const libc::c_char) -> *mut _GPatternSpec;
}


/*
GVariant * g_variant_new_signature() [struct _GVariant *]
	(const gchar *) signature [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_signature(signature: *const libc::c_char) -> *mut _GVariant;
}


/*
void g_queue_reverse()
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_reverse(queue: *mut _GQueue);
}


/*
gpointer g_object_dup_data() [void *]
	(GObject *) object [struct _GObject *]
	(const gchar *) key [const char *]
	(GDuplicateFunc) dup_func [void *(*)(void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_dup_data(object: *mut _GObject, key: *const libc::c_char, dup_func: Option<extern fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void>, user_data: *mut libc::c_void) -> *mut libc::c_void;
}


/*
GList * g_list_last() [struct _GList *]
	(GList *) list [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_last(list: *mut _GList) -> *mut _GList;
}


/*
gchar * g_regex_escape_string() [char *]
	(const gchar *) string [const char *]
	(gint) length [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_escape_string(string: *const libc::c_char, length: libc::c_int) -> *mut libc::c_char;
}


/*
guint g_value_get_flags() [unsigned int]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_flags(value: *const _GValue) -> libc::c_uint;
}


/*
GType g_value_array_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_array_get_type() -> libc::c_ulong;
}


/*
gboolean g_unichar_istitle() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_istitle(c: libc::c_uint) -> libc::c_int;
}


/*
guint g_direct_hash() [unsigned int]
	(gconstpointer) v [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_direct_hash(v: *const libc::c_void) -> libc::c_uint;
}


/*
gboolean g_hook_destroy() [int]
	(GHookList *) hook_list [struct _GHookList *]
	(gulong) hook_id [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_destroy(hook_list: *mut _GHookList, hook_id: libc::c_ulong) -> libc::c_int;
}


/*
gint g_bit_nth_msf() [int]
	(gulong) mask [unsigned long]
	(gint) nth_bit [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bit_nth_msf(mask: libc::c_ulong, nth_bit: libc::c_int) -> libc::c_int;
}


/*
void g_test_queue_free()
	(gpointer) gfree_pointer [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_queue_free(gfree_pointer: *mut libc::c_void);
}


/*
GArray * g_array_remove_index() [struct _GArray *]
	(GArray *) array [struct _GArray *]
	(guint) index_ [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_array_remove_index(array: *mut _GArray, index_: libc::c_uint) -> *mut _GArray;
}


/*
GIOChannel * g_io_channel_ref() [struct _GIOChannel *]
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_ref(channel: *mut _GIOChannel) -> *mut _GIOChannel;
}


/*
gboolean g_unichar_isdigit() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_isdigit(c: libc::c_uint) -> libc::c_int;
}


/*
gulong g_value_get_ulong() [unsigned long]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_ulong(value: *const _GValue) -> libc::c_ulong;
}


/*
GValueArray * g_value_array_sort_with_data() [struct _GValueArray *]
	(GValueArray *) value_array [struct _GValueArray *]
	(GCompareDataFunc) compare_func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_array_sort_with_data(value_array: *mut _GValueArray, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void) -> *mut _GValueArray;
}


/*
void g_cclosure_marshal_VOID__UCHAR()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_VOID__UCHAR(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
GVariant * g_variant_get_child_value() [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
	(gsize) index_ [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_child_value(value: *mut _GVariant, index_: libc::c_ulong) -> *mut _GVariant;
}


/*
void g_scanner_warn()
	(GScanner *) scanner [struct _GScanner *]
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_warn(scanner: *mut _GScanner, format: *const libc::c_char);
}


/*
GSequence * g_sequence_new() [struct _GSequence *]
	(GDestroyNotify) data_destroy [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_new(data_destroy: Option<extern fn(*mut libc::c_void)>) -> *mut _GSequence;
}


/*
void g_io_channel_close()
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_close(channel: *mut _GIOChannel);
}


/*
void g_key_file_set_double()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gdouble) value [double]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_set_double(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, value: libc::c_double);
}


/*
gint g_param_values_cmp() [int]
	(GParamSpec *) pspec [struct _GParamSpec *]
	(const GValue *) value1 [const struct _GValue *]
	(const GValue *) value2 [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_values_cmp(pspec: *mut _GParamSpec, value1: *const _GValue, value2: *const _GValue) -> libc::c_int;
}


/*
GVariant * g_variant_dict_lookup_value() [struct _GVariant *]
	(GVariantDict *) dict [struct _GVariantDict *]
	(const gchar *) key [const char *]
	(const GVariantType *) expected_type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_dict_lookup_value(dict: *mut _GVariantDict, key: *const libc::c_char, expected_type: *const _GVariantType) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_va() [struct _GVariant *]
	(const gchar *) format_string [const char *]
	(const gchar **) endptr [const char **]
	(va_list *) app [struct __va_list_tag (*)[1]]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_variant_new_va(format_string: *const libc::c_char, endptr: *mut *const libc::c_char, app: *mut [__va_list_tag; 1]) -> *mut _GVariant;
}


/*
void g_bit_lock()
	(volatile gint *) address [volatile int *]
	(gint) lock_bit [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bit_lock(address: *mut libc::c_int, lock_bit: libc::c_int);
}


/*
void g_completion_remove_items()
	(GCompletion *) cmp [struct _GCompletion *]
	(GList *) items [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_completion_remove_items(cmp: *mut _GCompletion, items: *mut _GList);
}


/*
GType * g_type_interface_prerequisites() [unsigned long *]
	(GType) interface_type [unsigned long]
	(guint *) n_prerequisites [unsigned int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_interface_prerequisites(interface_type: libc::c_ulong, n_prerequisites: *mut libc::c_uint) -> *mut libc::c_ulong;
}


/*
gulong g_signal_connect_closure_by_id() [unsigned long]
	(gpointer) instance [void *]
	(guint) signal_id [unsigned int]
	(GQuark) detail [unsigned int]
	(GClosure *) closure [struct _GClosure *]
	(gboolean) after [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_connect_closure_by_id(instance: *mut libc::c_void, signal_id: libc::c_uint, detail: libc::c_uint, closure: *mut _GClosure, after: libc::c_int) -> libc::c_ulong;
}


/*
GError * g_error_new_valist() [struct _GError *]
	(GQuark) domain [unsigned int]
	(gint) code [int]
	(const gchar *) format [const char *]
	(va_list) args [struct __va_list_tag [1]]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_error_new_valist(domain: libc::c_uint, code: libc::c_int, format: *const libc::c_char, args: [__va_list_tag; 1]) -> *mut _GError;
}


/*
gint g_snprintf() [int]
	(gchar *) string [char *]
	(gulong) n [unsigned long]
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_snprintf(string: *mut libc::c_char, n: libc::c_ulong, format: *const libc::c_char) -> libc::c_int;
}


/*
GVariant * g_variant_new_uint16() [struct _GVariant *]
	(guint16) value [unsigned short]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_uint16(value: libc::c_ushort) -> *mut _GVariant;
}


/*
void g_value_set_pointer()
	(GValue *) value [struct _GValue *]
	(gpointer) v_pointer [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_pointer(value: *mut _GValue, v_pointer: *mut libc::c_void);
}


/*
void g_value_set_char()
	(GValue *) value [struct _GValue *]
	(gchar) v_char [char]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_char(value: *mut _GValue, v_char: libc::c_char);
}


/*
void g_cache_key_foreach()
	(GCache *) cache [struct _GCache *]
	(GHFunc) func [void (*)(void *, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cache_key_foreach(cache: *mut _GCache, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
const gchar * g_dpgettext2() [const char *]
	(const gchar *) domain [const char *]
	(const gchar *) context [const char *]
	(const gchar *) msgid [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_dpgettext2(domain: *const libc::c_char, context: *const libc::c_char, msgid: *const libc::c_char) -> *const libc::c_char;
}


/*
gboolean g_key_file_get_boolean() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_get_boolean(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
gint g_tree_height() [int]
	(GTree *) tree [struct _GTree *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_tree_height(tree: *mut _GTree) -> libc::c_int;
}


/*
void g_assertion_message()
	(const char *) domain
	(const char *) file
	(int) line
	(const char *) func
	(const char *) message
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_assertion_message(domain: *const libc::c_char, file: *const libc::c_char, line: libc::c_int, func: *const libc::c_char, message: *const libc::c_char);
}


/*
GIOStatus g_io_channel_read_chars() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gchar *) buf [char *]
	(gsize) count [unsigned long]
	(gsize *) bytes_read [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_read_chars(channel: *mut _GIOChannel, buf: *mut libc::c_char, count: libc::c_ulong, bytes_read: *mut libc::c_ulong, error: *mut *mut _GError) -> libc::c_uint;
}


/*
void g_variant_get_va()
	(GVariant *) value [struct _GVariant *]
	(const gchar *) format_string [const char *]
	(const gchar **) endptr [const char **]
	(va_list *) app [struct __va_list_tag (*)[1]]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_variant_get_va(value: *mut _GVariant, format_string: *const libc::c_char, endptr: *mut *const libc::c_char, app: *mut [__va_list_tag; 1]);
}


/*
gboolean g_main_context_iteration() [int]
	(GMainContext *) context [struct _GMainContext *]
	(gboolean) may_block [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_iteration(context: *mut _GMainContext, may_block: libc::c_int) -> libc::c_int;
}


/*
GParamSpec ** g_param_spec_pool_list() [struct _GParamSpec **]
	(GParamSpecPool *) pool [struct _GParamSpecPool *]
	(GType) owner_type [unsigned long]
	(guint *) n_pspecs_p [unsigned int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_pool_list(pool: *mut _GParamSpecPool, owner_type: libc::c_ulong, n_pspecs_p: *mut libc::c_uint) -> *mut *mut _GParamSpec;
}


/*
GType g_variant_get_gtype() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_gtype() -> libc::c_ulong;
}


/*
gint * g_key_file_get_integer_list() [int *]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_get_integer_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_int;
}


/*
GFileError g_file_error_from_errno() [GFileError]
	(gint) err_no [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_file_error_from_errno(err_no: libc::c_int) -> libc::c_uint;
}


/*
GSource * g_main_context_find_source_by_funcs_user_data() [struct _GSource *]
	(GMainContext *) context [struct _GMainContext *]
	(GSourceFuncs *) funcs [struct _GSourceFuncs *]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_find_source_by_funcs_user_data(context: *mut _GMainContext, funcs: *mut _GSourceFuncs, user_data: *mut libc::c_void) -> *mut _GSource;
}


/*
GHook * g_hook_ref() [struct _GHook *]
	(GHookList *) hook_list [struct _GHookList *]
	(GHook *) hook [struct _GHook *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_ref(hook_list: *mut _GHookList, hook: *mut _GHook) -> *mut _GHook;
}


/*
void g_object_get_valist()
	(GObject *) object [struct _GObject *]
	(const gchar *) first_property_name [const char *]
	(va_list) var_args [struct __va_list_tag [1]]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_object_get_valist(object: *mut _GObject, first_property_name: *const libc::c_char, var_args: [__va_list_tag; 1]);
}


/*
gboolean g_cond_timed_wait() [int]
	(GCond *) cond [struct _GCond *]
	(GMutex *) mutex [union _GMutex *]
	(GTimeVal *) timeval [struct _GTimeVal *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cond_timed_wait(cond: *mut _GCond, mutex: *mut _GMutex, timeval: *mut _GTimeVal) -> libc::c_int;
}


/*
GSList * g_slist_nth() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(guint) n [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_nth(list: *mut _GSList, n: libc::c_uint) -> *mut _GSList;
}


/*
void g_unicode_canonical_ordering()
	(gunichar *) string [unsigned int *]
	(gsize) len [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unicode_canonical_ordering(string: *mut libc::c_uint, len: libc::c_ulong);
}


/*
gint g_slist_position() [int]
	(GSList *) list [struct _GSList *]
	(GSList *) llink [struct _GSList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_position(list: *mut _GSList, llink: *mut _GSList) -> libc::c_int;
}


/*
GVariant * g_variant_new_object_path() [struct _GVariant *]
	(const gchar *) object_path [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_object_path(object_path: *const libc::c_char) -> *mut _GVariant;
}


/*
GKeyFile * g_key_file_ref() [struct _GKeyFile *]
	(GKeyFile *) key_file [struct _GKeyFile *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_ref(key_file: *mut _GKeyFile) -> *mut _GKeyFile;
}


/*
void g_queue_foreach()
	(GQueue *) queue [struct _GQueue *]
	(GFunc) func [void (*)(void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_foreach(queue: *mut _GQueue, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
gboolean g_queue_remove() [int]
	(GQueue *) queue [struct _GQueue *]
	(gconstpointer) data [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_remove(queue: *mut _GQueue, data: *const libc::c_void) -> libc::c_int;
}


/*
GSequence * g_sequence_iter_get_sequence() [struct _GSequence *]
	(GSequenceIter *) iter [struct _GSequenceNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_iter_get_sequence(iter: *mut _GSequenceNode) -> *mut _GSequence;
}


/*
GType g_io_channel_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_get_type() -> libc::c_ulong;
}


/*
GType g_binding_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_binding_get_type() -> libc::c_ulong;
}


/*
GHashTable * g_hash_table_iter_get_hash_table() [struct _GHashTable *]
	(GHashTableIter *) iter [struct _GHashTableIter *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_iter_get_hash_table(iter: *mut _GHashTableIter) -> *mut _GHashTable;
}


/*
gdouble g_key_file_get_double() [double]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_get_double(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> libc::c_double;
}


/*
void g_static_private_init()
	(GStaticPrivate *) private_key [struct _GStaticPrivate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_private_init(private_key: *mut _GStaticPrivate);
}


/*
void g_cclosure_marshal_VOID__STRING()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_VOID__STRING(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
gchar ** g_match_info_fetch_all() [char **]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_match_info_fetch_all(match_info: *const _GMatchInfo) -> *mut *mut libc::c_char;
}


/*
GByteArray * g_byte_array_new() [struct _GByteArray *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_byte_array_new() -> *mut _GByteArray;
}


/*
const gchar * g_type_name_from_instance() [const char *]
	(GTypeInstance *) instance [struct _GTypeInstance *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_name_from_instance(instance: *mut _GTypeInstance) -> *const libc::c_char;
}


/*
gboolean g_scanner_eof() [int]
	(GScanner *) scanner [struct _GScanner *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_eof(scanner: *mut _GScanner) -> libc::c_int;
}


/*
gchar * g_variant_type_dup_string() [char *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_dup_string(type_: *const _GVariantType) -> *mut libc::c_char;
}


/*
void g_test_trap_subprocess()
	(const char *) test_path
	(guint64) usec_timeout [unsigned long]
	(GTestSubprocessFlags) test_flags [GTestSubprocessFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_trap_subprocess(test_path: *const libc::c_char, usec_timeout: libc::c_ulong, test_flags: libc::c_uint);
}


/*
void g_value_set_int64()
	(GValue *) value [struct _GValue *]
	(gint64) v_int64 [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_int64(value: *mut _GValue, v_int64: libc::c_long);
}


/*
gboolean g_atomic_int_compare_and_exchange() [int]
	(volatile gint *) atomic [volatile int *]
	(gint) oldval [int]
	(gint) newval [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_atomic_int_compare_and_exchange(atomic: *mut libc::c_int, oldval: libc::c_int, newval: libc::c_int) -> libc::c_int;
}


/*
gboolean g_unichar_validate() [int]
	(gunichar) ch [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_validate(ch: libc::c_uint) -> libc::c_int;
}


/*
gint g_date_time_get_day_of_year() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_get_day_of_year(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
GParamSpec * g_param_spec_int64() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(gint64) minimum [long]
	(gint64) maximum [long]
	(gint64) default_value [long]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_int64(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, minimum: libc::c_long, maximum: libc::c_long, default_value: libc::c_long, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
void g_cclosure_marshal_VOID__OBJECT()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_VOID__OBJECT(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
void g_cclosure_marshal_VOID__CHAR()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_VOID__CHAR(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
gboolean g_spawn_command_line_async() [int]
	(const gchar *) command_line [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_spawn_command_line_async(command_line: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_value_set_instance()
	(GValue *) value [struct _GValue *]
	(gpointer) instance [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_instance(value: *mut _GValue, instance: *mut libc::c_void);
}


/*
void g_type_plugin_complete_interface_info()
	(GTypePlugin *) plugin [struct _GTypePlugin *]
	(GType) instance_type [unsigned long]
	(GType) interface_type [unsigned long]
	(GInterfaceInfo *) info [struct _GInterfaceInfo *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_plugin_complete_interface_info(plugin: *mut _GTypePlugin, instance_type: libc::c_ulong, interface_type: libc::c_ulong, info: *mut _GInterfaceInfo);
}


/*
void g_signal_chain_from_overridden()
	(const GValue *) instance_and_params [const struct _GValue *]
	(GValue *) return_value [struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_chain_from_overridden(instance_and_params: *const _GValue, return_value: *mut _GValue);
}


/*
GVariant * g_variant_new_byte() [struct _GVariant *]
	(guchar) value [unsigned char]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_byte(value: libc::c_uchar) -> *mut _GVariant;
}


/*
GQuark g_shell_error_quark() [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_shell_error_quark() -> libc::c_uint;
}


/*
gboolean g_date_valid_weekday() [int]
	(GDateWeekday) weekday [GDateWeekday]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_valid_weekday(weekday: libc::c_uint) -> libc::c_int;
}


/*
const char * g_source_get_name()
	(GSource *) source [struct _GSource *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_get_name(source: *mut _GSource) -> *const libc::c_char;
}


/*
void g_closure_unref()
	(GClosure *) closure [struct _GClosure *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_closure_unref(closure: *mut _GClosure);
}


/*
GIOStatus g_io_channel_set_flags() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(GIOFlags) flags [GIOFlags]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_set_flags(channel: *mut _GIOChannel, flags: libc::c_uint, error: *mut *mut _GError) -> libc::c_uint;
}


/*
guint g_idle_add() [unsigned int]
	(GSourceFunc) function [int (*)(void *)]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_idle_add(function: Option<extern fn(*mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void) -> libc::c_uint;
}


/*
guint g_variant_type_hash() [unsigned int]
	(gconstpointer) type [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_hash(type_: *const libc::c_void) -> libc::c_uint;
}


/*
GVariant * g_variant_new_int64() [struct _GVariant *]
	(gint64) value [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_int64(value: libc::c_long) -> *mut _GVariant;
}


/*
void g_cclosure_marshal_VOID__PARAM()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_VOID__PARAM(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
gpointer g_value_dup_object() [void *]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_dup_object(value: *const _GValue) -> *mut libc::c_void;
}


/*
guint g_log_set_handler() [unsigned int]
	(const gchar *) log_domain [const char *]
	(GLogLevelFlags) log_levels [GLogLevelFlags]
	(GLogFunc) log_func [void (*)(const char *, GLogLevelFlags, const char *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_log_set_handler(log_domain: *const libc::c_char, log_levels: libc::c_uint, log_func: Option<extern fn(*const libc::c_char, libc::c_uint, *const libc::c_char, *mut libc::c_void)>, user_data: *mut libc::c_void) -> libc::c_uint;
}


/*
GVariantType * g_variant_type_copy() [struct _GVariantType *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_copy(type_: *const _GVariantType) -> *mut _GVariantType;
}


/*
gfloat g_value_get_float() [float]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_float(value: *const _GValue) -> libc::c_float;
}


/*
void g_usleep()
	(gulong) microseconds [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_usleep(microseconds: libc::c_ulong);
}


/*
GKeyFile * g_key_file_new() [struct _GKeyFile *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_new() -> *mut _GKeyFile;
}


/*
GSequenceIter * g_sequence_search_iter() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
	(gpointer) data [void *]
	(GSequenceIterCompareFunc) iter_cmp [int (*)(struct _GSequenceNode *, struct _GSequenceNode *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_search_iter(seq: *mut _GSequence, data: *mut libc::c_void, iter_cmp: Option<extern fn(*mut _GSequenceNode, *mut _GSequenceNode, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void) -> *mut _GSequenceNode;
}


/*
void g_main_context_unref()
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_unref(context: *mut _GMainContext);
}


/*
GSList * g_slist_copy_deep() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(GCopyFunc) func [void *(*)(const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_copy_deep(list: *mut _GSList, func: Option<extern fn(*const libc::c_void, *mut libc::c_void) -> *mut libc::c_void>, user_data: *mut libc::c_void) -> *mut _GSList;
}


/*
GList * g_param_spec_pool_list_owned() [struct _GList *]
	(GParamSpecPool *) pool [struct _GParamSpecPool *]
	(GType) owner_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_pool_list_owned(pool: *mut _GParamSpecPool, owner_type: libc::c_ulong) -> *mut _GList;
}


/*
GSequenceIter * g_sequence_range_get_midpoint() [struct _GSequenceNode *]
	(GSequenceIter *) begin [struct _GSequenceNode *]
	(GSequenceIter *) end [struct _GSequenceNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_range_get_midpoint(begin: *mut _GSequenceNode, end: *mut _GSequenceNode) -> *mut _GSequenceNode;
}


/*
void g_get_current_time()
	(GTimeVal *) result [struct _GTimeVal *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_current_time(result: *mut _GTimeVal);
}


/*
guint g_double_hash() [unsigned int]
	(gconstpointer) v [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_double_hash(v: *const libc::c_void) -> libc::c_uint;
}


/*
GSequenceIter * g_sequence_iter_prev() [struct _GSequenceNode *]
	(GSequenceIter *) iter [struct _GSequenceNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_iter_prev(iter: *mut _GSequenceNode) -> *mut _GSequenceNode;
}


/*
gchar * g_strchug() [char *]
	(gchar *) string [char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strchug(string: *mut libc::c_char) -> *mut libc::c_char;
}


/*
gpointer g_queue_pop_head() [void *]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_pop_head(queue: *mut _GQueue) -> *mut libc::c_void;
}


/*
GType g_mapped_file_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mapped_file_get_type() -> libc::c_ulong;
}


/*
void g_static_private_free()
	(GStaticPrivate *) private_key [struct _GStaticPrivate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_private_free(private_key: *mut _GStaticPrivate);
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_replace(regex: *const _GRegex, string: *const libc::c_char, string_len: libc::c_long, start_position: libc::c_int, replacement: *const libc::c_char, match_options: libc::c_uint, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
GMarkupParseContext * g_markup_parse_context_new() [struct _GMarkupParseContext *]
	(const GMarkupParser *) parser [const struct _GMarkupParser *]
	(GMarkupParseFlags) flags [GMarkupParseFlags]
	(gpointer) user_data [void *]
	(GDestroyNotify) user_data_dnotify [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_markup_parse_context_new(parser: *const _GMarkupParser, flags: libc::c_uint, user_data: *mut libc::c_void, user_data_dnotify: Option<extern fn(*mut libc::c_void)>) -> *mut _GMarkupParseContext;
}


/*
GType g_hash_table_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_get_type() -> libc::c_ulong;
}


/*
GUnicodeScript g_unichar_get_script() [GUnicodeScript]
	(gunichar) ch [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_get_script(ch: libc::c_uint) -> libc::c_uint;
}


/*
gpointer g_realloc_n() [void *]
	(gpointer) mem [void *]
	(gsize) n_blocks [unsigned long]
	(gsize) n_block_bytes [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_realloc_n(mem: *mut libc::c_void, n_blocks: libc::c_ulong, n_block_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
void g_tree_destroy()
	(GTree *) tree [struct _GTree *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_tree_destroy(tree: *mut _GTree);
}


/*
void g_dir_close()
	(GDir *) dir [struct _GDir *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_dir_close(dir: *mut _GDir);
}


/*
gpointer g_malloc0_n() [void *]
	(gsize) n_blocks [unsigned long]
	(gsize) n_block_bytes [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_malloc0_n(n_blocks: libc::c_ulong, n_block_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
void g_closure_invalidate()
	(GClosure *) closure [struct _GClosure *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_closure_invalidate(closure: *mut _GClosure);
}


/*
void g_variant_builder_unref()
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_builder_unref(builder: *mut _GVariantBuilder);
}


/*
gboolean g_date_is_last_of_month() [int]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_is_last_of_month(date: *const _GDate) -> libc::c_int;
}


/*
gchar * g_strcompress() [char *]
	(const gchar *) source [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strcompress(source: *const libc::c_char) -> *mut libc::c_char;
}


/*
gboolean g_variant_lookup() [int]
	(GVariant *) dictionary [struct _GVariant *]
	(const gchar *) key [const char *]
	(const gchar *) format_string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_lookup(dictionary: *mut _GVariant, key: *const libc::c_char, format_string: *const libc::c_char) -> libc::c_int;
}


/*
GHook * g_hook_find() [struct _GHook *]
	(GHookList *) hook_list [struct _GHookList *]
	(gboolean) need_valids [int]
	(GHookFindFunc) func [int (*)(struct _GHook *, void *)]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_find(hook_list: *mut _GHookList, need_valids: libc::c_int, func: Option<extern fn(*mut _GHook, *mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void) -> *mut _GHook;
}


/*
void g_source_remove_unix_fd()
	(GSource *) source [struct _GSource *]
	(gpointer) tag [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_remove_unix_fd(source: *mut _GSource, tag: *mut libc::c_void);
}


/*
GDateTime * g_date_time_to_local() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_to_local(datetime: *mut _GDateTime) -> *mut _GDateTime;
}


/*
GTokenType g_scanner_cur_token() [GTokenType]
	(GScanner *) scanner [struct _GScanner *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_cur_token(scanner: *mut _GScanner) -> libc::c_uint;
}


/*
gboolean g_file_set_contents() [int]
	(const gchar *) filename [const char *]
	(const gchar *) contents [const char *]
	(gssize) length [long]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_file_set_contents(filename: *const libc::c_char, contents: *const libc::c_char, length: libc::c_long, error: *mut *mut _GError) -> libc::c_int;
}


/*
GDateMonth g_date_get_month() [GDateMonth]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_get_month(date: *const _GDate) -> libc::c_uint;
}


/*
void g_rw_lock_writer_unlock()
	(GRWLock *) rw_lock [struct _GRWLock *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rw_lock_writer_unlock(rw_lock: *mut _GRWLock);
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_split_full(regex: *const _GRegex, string: *const libc::c_char, string_len: libc::c_long, start_position: libc::c_int, match_options: libc::c_uint, max_tokens: libc::c_int, error: *mut *mut _GError) -> *mut *mut libc::c_char;
}


/*
gulong g_signal_connect_data() [unsigned long]
	(gpointer) instance [void *]
	(const gchar *) detailed_signal [const char *]
	(GCallback) c_handler [void (*)(void)]
	(gpointer) data [void *]
	(GClosureNotify) destroy_data [void (*)(void *, struct _GClosure *)]
	(GConnectFlags) connect_flags [GConnectFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_connect_data(instance: *mut libc::c_void, detailed_signal: *const libc::c_char, c_handler: Option<extern fn()>, data: *mut libc::c_void, destroy_data: Option<extern fn(*mut libc::c_void, *mut _GClosure)>, connect_flags: libc::c_uint) -> libc::c_ulong;
}


/*
const GVariantType * g_variant_type_checked_() [const struct _GVariantType *]
	(const gchar *)  [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_checked_(_: *const libc::c_char) -> *const _GVariantType;
}


/*
gchar * g_match_info_fetch_named() [char *]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
	(const gchar *) name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_match_info_fetch_named(match_info: *const _GMatchInfo, name: *const libc::c_char) -> *mut libc::c_char;
}


/*
GVariant * g_variant_new_string() [struct _GVariant *]
	(const gchar *) string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_string(string: *const libc::c_char) -> *mut _GVariant;
}


/*
void g_bookmark_file_add_group()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) group [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_add_group(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, group: *const libc::c_char);
}


/*
gboolean g_cond_wait_until() [int]
	(GCond *) cond [struct _GCond *]
	(GMutex *) mutex [union _GMutex *]
	(gint64) end_time [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cond_wait_until(cond: *mut _GCond, mutex: *mut _GMutex, end_time: libc::c_long) -> libc::c_int;
}


/*
GSequenceIter * g_sequence_prepend() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_prepend(seq: *mut _GSequence, data: *mut libc::c_void) -> *mut _GSequenceNode;
}


/*
gunichar2 * g_utf8_to_utf16() [unsigned short *]
	(const gchar *) str [const char *]
	(glong) len [long]
	(glong *) items_read [long *]
	(glong *) items_written [long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_to_utf16(str: *const libc::c_char, len: libc::c_long, items_read: *mut libc::c_long, items_written: *mut libc::c_long, error: *mut *mut _GError) -> *mut libc::c_ushort;
}


/*
void g_object_interface_install_property()
	(gpointer) g_iface [void *]
	(GParamSpec *) pspec [struct _GParamSpec *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_interface_install_property(g_iface: *mut libc::c_void, pspec: *mut _GParamSpec);
}


/*
gchar * g_format_size_for_display() [char *]
	(goffset) size [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_format_size_for_display(size: libc::c_long) -> *mut libc::c_char;
}


/*
GList * g_list_remove() [struct _GList *]
	(GList *) list [struct _GList *]
	(gconstpointer) data [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_remove(list: *mut _GList, data: *const libc::c_void) -> *mut _GList;
}


/*
void g_cclosure_marshal_VOID__POINTER()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_VOID__POINTER(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
guint g_child_watch_add_full() [unsigned int]
	(gint) priority [int]
	(GPid) pid [int]
	(GChildWatchFunc) function [void (*)(int, int, void *)]
	(gpointer) data [void *]
	(GDestroyNotify) notify [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_child_watch_add_full(priority: libc::c_int, pid: libc::c_int, function: Option<extern fn(libc::c_int, libc::c_int, *mut libc::c_void)>, data: *mut libc::c_void, notify: Option<extern fn(*mut libc::c_void)>) -> libc::c_uint;
}


/*
void g_option_context_free()
	(GOptionContext *) context [struct _GOptionContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_context_free(context: *mut _GOptionContext);
}


/*
gint g_bytes_compare() [int]
	(gconstpointer) bytes1 [const void *]
	(gconstpointer) bytes2 [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bytes_compare(bytes1: *const libc::c_void, bytes2: *const libc::c_void) -> libc::c_int;
}


/*
gchar * g_utf8_strdown() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_strdown(str: *const libc::c_char, len: libc::c_long) -> *mut libc::c_char;
}


/*
gint g_match_info_get_match_count() [int]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_match_info_get_match_count(match_info: *const _GMatchInfo) -> libc::c_int;
}


/*
gint g_mkstemp() [int]
	(gchar *) tmpl [char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mkstemp(tmpl: *mut libc::c_char) -> libc::c_int;
}


/*
GDate * g_date_new_julian() [struct _GDate *]
	(guint32) julian_day [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_new_julian(julian_day: libc::c_uint) -> *mut _GDate;
}


/*
gchar * g_filename_display_basename() [char *]
	(const gchar *) filename [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_filename_display_basename(filename: *const libc::c_char) -> *mut libc::c_char;
}


/*
gboolean g_bookmark_file_to_file() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) filename [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_to_file(bookmark: *mut _GBookmarkFile, filename: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_datalist_set_flags()
	(GData **) datalist [struct _GData **]
	(guint) flags [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_datalist_set_flags(datalist: *mut *mut _GData, flags: libc::c_uint);
}


/*
void g_cclosure_marshal_VOID__CHARv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_cclosure_marshal_VOID__CHARv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
void g_param_spec_set_qdata_full()
	(GParamSpec *) pspec [struct _GParamSpec *]
	(GQuark) quark [unsigned int]
	(gpointer) data [void *]
	(GDestroyNotify) destroy [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_set_qdata_full(pspec: *mut _GParamSpec, quark: libc::c_uint, data: *mut libc::c_void, destroy: Option<extern fn(*mut libc::c_void)>);
}


/*
void g_hash_table_unref()
	(GHashTable *) hash_table [struct _GHashTable *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_unref(hash_table: *mut _GHashTable);
}


/*
gchar ** g_variant_dup_strv() [char **]
	(GVariant *) value [struct _GVariant *]
	(gsize *) length [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_dup_strv(value: *mut _GVariant, length: *mut libc::c_ulong) -> *mut *mut libc::c_char;
}


/*
gpointer g_dataset_id_get_data() [void *]
	(gconstpointer) dataset_location [const void *]
	(GQuark) key_id [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_dataset_id_get_data(dataset_location: *const libc::c_void, key_id: libc::c_uint) -> *mut libc::c_void;
}


/*
GQuark g_bookmark_file_error_quark() [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_error_quark() -> libc::c_uint;
}


/*
gchar ** g_strdupv() [char **]
	(gchar **) str_array [char **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strdupv(str_array: *mut *mut libc::c_char) -> *mut *mut libc::c_char;
}


/*
GString * g_string_append() [struct _GString *]
	(GString *) string [struct _GString *]
	(const gchar *) val [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_append(string: *mut _GString, val: *const libc::c_char) -> *mut _GString;
}


/*
gint g_bookmark_file_get_size() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_get_size(bookmark: *mut _GBookmarkFile) -> libc::c_int;
}


/*
gchar ** g_bookmark_file_get_uris() [char **]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(gsize *) length [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_get_uris(bookmark: *mut _GBookmarkFile, length: *mut libc::c_ulong) -> *mut *mut libc::c_char;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_new(tz: *mut _GTimeZone, year: libc::c_int, month: libc::c_int, day: libc::c_int, hour: libc::c_int, minute: libc::c_int, seconds: libc::c_double) -> *mut _GDateTime;
}


/*
const gchar * g_time_zone_get_abbreviation() [const char *]
	(GTimeZone *) tz [struct _GTimeZone *]
	(gint) interval [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_time_zone_get_abbreviation(tz: *mut _GTimeZone, interval: libc::c_int) -> *const libc::c_char;
}


/*
gboolean g_param_value_convert() [int]
	(GParamSpec *) pspec [struct _GParamSpec *]
	(const GValue *) src_value [const struct _GValue *]
	(GValue *) dest_value [struct _GValue *]
	(gboolean) strict_validation [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_value_convert(pspec: *mut _GParamSpec, src_value: *const _GValue, dest_value: *mut _GValue, strict_validation: libc::c_int) -> libc::c_int;
}


/*
GSequenceIter * g_sequence_insert_sorted_iter() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
	(gpointer) data [void *]
	(GSequenceIterCompareFunc) iter_cmp [int (*)(struct _GSequenceNode *, struct _GSequenceNode *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_insert_sorted_iter(seq: *mut _GSequence, data: *mut libc::c_void, iter_cmp: Option<extern fn(*mut _GSequenceNode, *mut _GSequenceNode, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void) -> *mut _GSequenceNode;
}


/*
void g_queue_push_tail_link()
	(GQueue *) queue [struct _GQueue *]
	(GList *) link_ [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_push_tail_link(queue: *mut _GQueue, link_: *mut _GList);
}


/*
void g_type_module_add_interface()
	(GTypeModule *) module [struct _GTypeModule *]
	(GType) instance_type [unsigned long]
	(GType) interface_type [unsigned long]
	(const GInterfaceInfo *) interface_info [const struct _GInterfaceInfo *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_module_add_interface(module: *mut _GTypeModule, instance_type: libc::c_ulong, interface_type: libc::c_ulong, interface_info: *const _GInterfaceInfo);
}


/*
guint32 g_variant_get_uint32() [unsigned int]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_uint32(value: *mut _GVariant) -> libc::c_uint;
}


/*
GSource * g_child_watch_source_new() [struct _GSource *]
	(GPid) pid [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_child_watch_source_new(pid: libc::c_int) -> *mut _GSource;
}


/*
void g_variant_get_child()
	(GVariant *) value [struct _GVariant *]
	(gsize) index_ [unsigned long]
	(const gchar *) format_string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_child(value: *mut _GVariant, index_: libc::c_ulong, format_string: *const libc::c_char);
}


/*
gboolean g_variant_type_string_scan() [int]
	(const gchar *) string [const char *]
	(const gchar *) limit [const char *]
	(const gchar **) endptr [const char **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_string_scan(string: *const libc::c_char, limit: *const libc::c_char, endptr: *mut *const libc::c_char) -> libc::c_int;
}


/*
gsize g_printf_string_upper_bound() [unsigned long]
	(const gchar *) format [const char *]
	(va_list) args [struct __va_list_tag [1]]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_printf_string_upper_bound(format: *const libc::c_char, args: [__va_list_tag; 1]) -> libc::c_ulong;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_create_full(func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>, data: *mut libc::c_void, stack_size: libc::c_ulong, joinable: libc::c_int, bound: libc::c_int, priority: libc::c_uint, error: *mut *mut _GError) -> *mut _GThread;
}


/*
void g_type_ensure()
	(GType) type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_ensure(type_: libc::c_ulong);
}


/*
gint g_date_time_get_day_of_month() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_get_day_of_month(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
GQuark g_variant_parser_get_error_quark() [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_parser_get_error_quark() -> libc::c_uint;
}


/*
void g_test_assert_expected_messages_internal()
	(const char *) domain
	(const char *) file
	(int) line
	(const char *) func
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_assert_expected_messages_internal(domain: *const libc::c_char, file: *const libc::c_char, line: libc::c_int, func: *const libc::c_char);
}


/*
GType g_boxed_type_register_static() [unsigned long]
	(const gchar *) name [const char *]
	(GBoxedCopyFunc) boxed_copy [void *(*)(void *)]
	(GBoxedFreeFunc) boxed_free [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_boxed_type_register_static(name: *const libc::c_char, boxed_copy: Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>, boxed_free: Option<extern fn(*mut libc::c_void)>) -> libc::c_ulong;
}


/*
GSource * g_idle_source_new() [struct _GSource *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_idle_source_new() -> *mut _GSource;
}


/*
void g_object_set_property()
	(GObject *) object [struct _GObject *]
	(const gchar *) property_name [const char *]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_set_property(object: *mut _GObject, property_name: *const libc::c_char, value: *const _GValue);
}


/*
gchar * g_string_chunk_insert_len() [char *]
	(GStringChunk *) chunk [struct _GStringChunk *]
	(const gchar *) string [const char *]
	(gssize) len [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_chunk_insert_len(chunk: *mut _GStringChunk, string: *const libc::c_char, len: libc::c_long) -> *mut libc::c_char;
}


/*
GList * g_list_delete_link() [struct _GList *]
	(GList *) list [struct _GList *]
	(GList *) link_ [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_delete_link(list: *mut _GList, link_: *mut _GList) -> *mut _GList;
}


/*
guchar g_value_get_uchar() [unsigned char]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_uchar(value: *const _GValue) -> libc::c_uchar;
}


/*
void g_bookmark_file_set_is_private()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(gboolean) is_private [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_set_is_private(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, is_private: libc::c_int);
}


/*
guint g_scanner_cur_line() [unsigned int]
	(GScanner *) scanner [struct _GScanner *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_cur_line(scanner: *mut _GScanner) -> libc::c_uint;
}


/*
void g_rec_mutex_lock()
	(GRecMutex *) rec_mutex [struct _GRecMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rec_mutex_lock(rec_mutex: *mut _GRecMutex);
}


/*
GVariant * g_variant_new() [struct _GVariant *]
	(const gchar *) format_string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new(format_string: *const libc::c_char) -> *mut _GVariant;
}


/*
GString * g_string_down() [struct _GString *]
	(GString *) string [struct _GString *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_down(string: *mut _GString) -> *mut _GString;
}


/*
gchar * g_strup() [char *]
	(gchar *) string [char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strup(string: *mut libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_hostname_to_ascii() [char *]
	(const gchar *) hostname [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hostname_to_ascii(hostname: *const libc::c_char) -> *mut libc::c_char;
}


/*
guint g_hash_table_size() [unsigned int]
	(GHashTable *) hash_table [struct _GHashTable *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_size(hash_table: *mut _GHashTable) -> libc::c_uint;
}


/*
gpointer g_type_class_peek() [void *]
	(GType) type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_class_peek(type_: libc::c_ulong) -> *mut libc::c_void;
}


/*
gboolean g_bookmark_file_has_application() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) name [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_has_application(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, name: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
GHmac * g_hmac_ref() [struct _GHmac *]
	(GHmac *) hmac [struct _GHmac *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hmac_ref(hmac: *mut _GHmac) -> *mut _GHmac;
}


/*
gchar ** g_get_environ() [char **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_environ() -> *mut *mut libc::c_char;
}


/*
GQuark g_variant_parse_error_quark() [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_parse_error_quark() -> libc::c_uint;
}


/*
gpointer g_try_realloc() [void *]
	(gpointer) mem [void *]
	(gsize) n_bytes [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_try_realloc(mem: *mut libc::c_void, n_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
void g_cclosure_marshal_BOOLEAN__FLAGS()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_BOOLEAN__FLAGS(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
GPtrArray * g_ptr_array_new_full() [struct _GPtrArray *]
	(guint) reserved_size [unsigned int]
	(GDestroyNotify) element_free_func [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_new_full(reserved_size: libc::c_uint, element_free_func: Option<extern fn(*mut libc::c_void)>) -> *mut _GPtrArray;
}


/*
GVariantType * g_variant_type_new_maybe() [struct _GVariantType *]
	(const GVariantType *) element [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_new_maybe(element: *const _GVariantType) -> *mut _GVariantType;
}


/*
void g_type_query()
	(GType) type [unsigned long]
	(GTypeQuery *) query [struct _GTypeQuery *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_query(type_: libc::c_ulong, query: *mut _GTypeQuery);
}


/*
GBytes * g_bytes_new_static() [struct _GBytes *]
	(gconstpointer) data [const void *]
	(gsize) size [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bytes_new_static(data: *const libc::c_void, size: libc::c_ulong) -> *mut _GBytes;
}


/*
void g_cond_free()
	(GCond *) cond [struct _GCond *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cond_free(cond: *mut _GCond);
}


/*
void g_key_file_set_int64()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gint64) value [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_set_int64(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, value: libc::c_long);
}


/*
guint g_strv_length() [unsigned int]
	(gchar **) str_array [char **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strv_length(str_array: *mut *mut libc::c_char) -> libc::c_uint;
}


/*
void g_thread_pool_set_max_unused_threads()
	(gint) max_threads [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_pool_set_max_unused_threads(max_threads: libc::c_int);
}


/*
void g_cclosure_marshal_VOID__POINTERv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_cclosure_marshal_VOID__POINTERv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
void g_scanner_input_text()
	(GScanner *) scanner [struct _GScanner *]
	(const gchar *) text [const char *]
	(guint) text_len [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_input_text(scanner: *mut _GScanner, text: *const libc::c_char, text_len: libc::c_uint);
}


/*
void g_object_freeze_notify()
	(GObject *) object [struct _GObject *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_freeze_notify(object: *mut _GObject);
}


/*
void g_bookmark_file_set_groups()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar **) groups [const char **]
	(gsize) length [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_set_groups(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, groups: *mut *const libc::c_char, length: libc::c_ulong);
}


/*
guint g_node_n_nodes() [unsigned int]
	(GNode *) root [struct _GNode *]
	(GTraverseFlags) flags [GTraverseFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_n_nodes(root: *mut _GNode, flags: libc::c_uint) -> libc::c_uint;
}


/*
gint g_relation_count() [int]
	(GRelation *) relation [struct _GRelation *]
	(gconstpointer) key [const void *]
	(gint) field [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_relation_count(relation: *mut _GRelation, key: *const libc::c_void, field: libc::c_int) -> libc::c_int;
}


/*
GSource * g_source_ref() [struct _GSource *]
	(GSource *) source [struct _GSource *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_ref(source: *mut _GSource) -> *mut _GSource;
}


/*
gboolean g_main_context_wait() [int]
	(GMainContext *) context [struct _GMainContext *]
	(GCond *) cond [struct _GCond *]
	(GMutex *) mutex [union _GMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_wait(context: *mut _GMainContext, cond: *mut _GCond, mutex: *mut _GMutex) -> libc::c_int;
}


/*
gpointer g_value_peek_pointer() [void *]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_peek_pointer(value: *const _GValue) -> *mut libc::c_void;
}


/*
GQuark g_io_channel_error_quark() [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_error_quark() -> libc::c_uint;
}


/*
void g_cclosure_marshal_VOID__PARAMv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_cclosure_marshal_VOID__PARAMv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
gboolean g_signal_handler_is_connected() [int]
	(gpointer) instance [void *]
	(gulong) handler_id [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_handler_is_connected(instance: *mut libc::c_void, handler_id: libc::c_ulong) -> libc::c_int;
}


/*
const gchar * g_intern_static_string() [const char *]
	(const gchar *) string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_intern_static_string(string: *const libc::c_char) -> *const libc::c_char;
}


/*
void g_test_fail()
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_fail();
}


/*
void g_source_modify_unix_fd()
	(GSource *) source [struct _GSource *]
	(gpointer) tag [void *]
	(GIOCondition) new_events [GIOCondition]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_modify_unix_fd(source: *mut _GSource, tag: *mut libc::c_void, new_events: libc::c_uint);
}


/*
GVariant * g_variant_new_maybe() [struct _GVariant *]
	(const GVariantType *) child_type [const struct _GVariantType *]
	(GVariant *) child [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_maybe(child_type: *const _GVariantType, child: *mut _GVariant) -> *mut _GVariant;
}


/*
gboolean g_unichar_isprint() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_isprint(c: libc::c_uint) -> libc::c_int;
}


/*
gint g_value_get_int() [int]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_int(value: *const _GValue) -> libc::c_int;
}


/*
gchar * g_dir_make_tmp() [char *]
	(const gchar *) tmpl [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_dir_make_tmp(tmpl: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
const GVariantType * g_variant_type_first() [const struct _GVariantType *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_first(type_: *const _GVariantType) -> *const _GVariantType;
}


/*
void g_source_remove_poll()
	(GSource *) source [struct _GSource *]
	(GPollFD *) fd [struct _GPollFD *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_remove_poll(source: *mut _GSource, fd: *mut _GPollFD);
}


/*
void g_date_clear()
	(GDate *) date [struct _GDate *]
	(guint) n_dates [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_clear(date: *mut _GDate, n_dates: libc::c_uint);
}


/*
gdouble g_ascii_strtod() [double]
	(const gchar *) nptr [const char *]
	(gchar **) endptr [char **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ascii_strtod(nptr: *const libc::c_char, endptr: *mut *mut libc::c_char) -> libc::c_double;
}


/*
gboolean g_atomic_int_dec_and_test() [int]
	(volatile gint *) atomic [volatile int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_atomic_int_dec_and_test(atomic: *mut libc::c_int) -> libc::c_int;
}


/*
glong g_utf8_strlen() [long]
	(const gchar *) p [const char *]
	(gssize) max [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_strlen(p: *const libc::c_char, max: libc::c_long) -> libc::c_long;
}


/*
GBytes * g_mapped_file_get_bytes() [struct _GBytes *]
	(GMappedFile *) file [struct _GMappedFile *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mapped_file_get_bytes(file: *mut _GMappedFile) -> *mut _GBytes;
}


/*
void g_tuples_destroy()
	(GTuples *) tuples [struct _GTuples *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_tuples_destroy(tuples: *mut _GTuples);
}


/*
void g_variant_dict_insert_value()
	(GVariantDict *) dict [struct _GVariantDict *]
	(const gchar *) key [const char *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_dict_insert_value(dict: *mut _GVariantDict, key: *const libc::c_char, value: *mut _GVariant);
}


/*
void g_main_context_pop_thread_default()
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_pop_thread_default(context: *mut _GMainContext);
}


/*
gchar * g_path_get_basename() [char *]
	(const gchar *) file_name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_path_get_basename(file_name: *const libc::c_char) -> *mut libc::c_char;
}


/*
GType g_time_zone_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_time_zone_get_type() -> libc::c_ulong;
}


/*
guint g_queue_remove_all() [unsigned int]
	(GQueue *) queue [struct _GQueue *]
	(gconstpointer) data [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_remove_all(queue: *mut _GQueue, data: *const libc::c_void) -> libc::c_uint;
}


/*
GNode * g_node_nth_child() [struct _GNode *]
	(GNode *) node [struct _GNode *]
	(guint) n [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_nth_child(node: *mut _GNode, n: libc::c_uint) -> *mut _GNode;
}


/*
guint g_slist_length() [unsigned int]
	(GSList *) list [struct _GSList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_length(list: *mut _GSList) -> libc::c_uint;
}


/*
gsize g_date_strftime() [unsigned long]
	(gchar *) s [char *]
	(gsize) slen [unsigned long]
	(const gchar *) format [const char *]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_strftime(s: *mut libc::c_char, slen: libc::c_ulong, format: *const libc::c_char, date: *const _GDate) -> libc::c_ulong;
}


/*
gchar * g_mkdtemp() [char *]
	(gchar *) tmpl [char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mkdtemp(tmpl: *mut libc::c_char) -> *mut libc::c_char;
}


/*
void g_array_sort()
	(GArray *) array [struct _GArray *]
	(GCompareFunc) compare_func [int (*)(const void *, const void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_array_sort(array: *mut _GArray, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>);
}


/*
gint g_date_time_get_hour() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_get_hour(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
const gchar * g_regex_get_pattern() [const char *]
	(const GRegex *) regex [const struct _GRegex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_get_pattern(regex: *const _GRegex) -> *const libc::c_char;
}


/*
gint g_regex_get_max_backref() [int]
	(const GRegex *) regex [const struct _GRegex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_get_max_backref(regex: *const _GRegex) -> libc::c_int;
}


/*
void g_date_add_days()
	(GDate *) date [struct _GDate *]
	(guint) n_days [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_add_days(date: *mut _GDate, n_days: libc::c_uint);
}


/*
void g_static_rw_lock_writer_lock()
	(GStaticRWLock *) lock [struct _GStaticRWLock *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_rw_lock_writer_lock(lock: *mut _GStaticRWLock);
}


/*
gboolean g_str_match_string() [int]
	(const gchar *) search_term [const char *]
	(const gchar *) potential_hit [const char *]
	(gboolean) accept_alternates [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_str_match_string(search_term: *const libc::c_char, potential_hit: *const libc::c_char, accept_alternates: libc::c_int) -> libc::c_int;
}


/*
void g_cclosure_marshal_VOID__DOUBLEv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_cclosure_marshal_VOID__DOUBLEv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
void g_date_set_day()
	(GDate *) date [struct _GDate *]
	(GDateDay) day [unsigned char]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_set_day(date: *mut _GDate, day: libc::c_uchar);
}


/*
gchar ** g_regex_split_simple() [char **]
	(const gchar *) pattern [const char *]
	(const gchar *) string [const char *]
	(GRegexCompileFlags) compile_options [GRegexCompileFlags]
	(GRegexMatchFlags) match_options [GRegexMatchFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_split_simple(pattern: *const libc::c_char, string: *const libc::c_char, compile_options: libc::c_uint, match_options: libc::c_uint) -> *mut *mut libc::c_char;
}


/*
void g_test_minimized_result()
	(double) minimized_quantity
	(const char *) format
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_minimized_result(minimized_quantity: libc::c_double, format: *const libc::c_char);
}


/*
GThread * g_thread_create() [struct _GThread *]
	(GThreadFunc) func [void *(*)(void *)]
	(gpointer) data [void *]
	(gboolean) joinable [int]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_create(func: Option<extern fn(*mut libc::c_void) -> *mut libc::c_void>, data: *mut libc::c_void, joinable: libc::c_int, error: *mut *mut _GError) -> *mut _GThread;
}


/*
void g_object_class_install_properties()
	(GObjectClass *) oclass [struct _GObjectClass *]
	(guint) n_pspecs [unsigned int]
	(GParamSpec **) pspecs [struct _GParamSpec **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_class_install_properties(oclass: *mut _GObjectClass, n_pspecs: libc::c_uint, pspecs: *mut *mut _GParamSpec);
}


/*
gboolean g_variant_get_boolean() [int]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_boolean(value: *mut _GVariant) -> libc::c_int;
}


/*
gboolean g_object_is_floating() [int]
	(gpointer) object [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_is_floating(object: *mut libc::c_void) -> libc::c_int;
}


/*
gboolean g_date_valid() [int]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_valid(date: *const _GDate) -> libc::c_int;
}


/*
void g_main_context_remove_poll()
	(GMainContext *) context [struct _GMainContext *]
	(GPollFD *) fd [struct _GPollFD *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_remove_poll(context: *mut _GMainContext, fd: *mut _GPollFD);
}


/*
void g_private_set()
	(GPrivate *) key [struct _GPrivate *]
	(gpointer) value [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_private_set(key: *mut _GPrivate, value: *mut libc::c_void);
}


/*
void g_object_set_data()
	(GObject *) object [struct _GObject *]
	(const gchar *) key [const char *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_set_data(object: *mut _GObject, key: *const libc::c_char, data: *mut libc::c_void);
}


/*
void g_thread_unref()
	(GThread *) thread [struct _GThread *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_unref(thread: *mut _GThread);
}


/*
void g_node_children_foreach()
	(GNode *) node [struct _GNode *]
	(GTraverseFlags) flags [GTraverseFlags]
	(GNodeForeachFunc) func [void (*)(struct _GNode *, void *)]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_children_foreach(node: *mut _GNode, flags: libc::c_uint, func: Option<extern fn(*mut _GNode, *mut libc::c_void)>, data: *mut libc::c_void);
}


/*
void g_slist_free_full()
	(GSList *) list [struct _GSList *]
	(GDestroyNotify) free_func [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_free_full(list: *mut _GSList, free_func: Option<extern fn(*mut libc::c_void)>);
}


/*
GByteArray * g_byte_array_sized_new() [struct _GByteArray *]
	(guint) reserved_size [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_byte_array_sized_new(reserved_size: libc::c_uint) -> *mut _GByteArray;
}


/*
const gchar * g_markup_parse_context_get_element() [const char *]
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_markup_parse_context_get_element(context: *mut _GMarkupParseContext) -> *const libc::c_char;
}


/*
GHook * g_hook_get() [struct _GHook *]
	(GHookList *) hook_list [struct _GHookList *]
	(gulong) hook_id [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_get(hook_list: *mut _GHookList, hook_id: libc::c_ulong) -> *mut _GHook;
}


/*
GRegexCompileFlags g_regex_get_compile_flags() [GRegexCompileFlags]
	(const GRegex *) regex [const struct _GRegex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_get_compile_flags(regex: *const _GRegex) -> libc::c_uint;
}


/*
GQueue * g_queue_copy() [struct _GQueue *]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_copy(queue: *mut _GQueue) -> *mut _GQueue;
}


/*
void g_queue_sort()
	(GQueue *) queue [struct _GQueue *]
	(GCompareDataFunc) compare_func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_sort(queue: *mut _GQueue, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
void g_option_context_add_group()
	(GOptionContext *) context [struct _GOptionContext *]
	(GOptionGroup *) group [struct _GOptionGroup *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_context_add_group(context: *mut _GOptionContext, group: *mut _GOptionGroup);
}


/*
gboolean g_unichar_isupper() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_isupper(c: libc::c_uint) -> libc::c_int;
}


/*
GVariant * g_variant_new_dict_entry() [struct _GVariant *]
	(GVariant *) key [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_dict_entry(key: *mut _GVariant, value: *mut _GVariant) -> *mut _GVariant;
}


/*
void g_cclosure_marshal_VOID__FLOAT()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_VOID__FLOAT(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
gboolean g_type_check_value_holds() [int]
	(GValue *) value [struct _GValue *]
	(GType) type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_check_value_holds(value: *mut _GValue, type_: libc::c_ulong) -> libc::c_int;
}


/*
void g_key_file_set_string()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(const gchar *) string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_set_string(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, string: *const libc::c_char);
}


/*
gboolean * g_key_file_get_boolean_list() [int *]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_get_boolean_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_int;
}


/*
const GVariantType * g_variant_get_type() [const struct _GVariantType *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_type(value: *mut _GVariant) -> *const _GVariantType;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_create_case(test_name: *const libc::c_char, data_size: libc::c_ulong, test_data: *const libc::c_void, data_setup: Option<extern fn(*mut libc::c_void, *const libc::c_void)>, data_test: Option<extern fn(*mut libc::c_void, *const libc::c_void)>, data_teardown: Option<extern fn(*mut libc::c_void, *const libc::c_void)>) -> *mut GTestCase;
}


/*
GDateTime * g_date_time_new_from_unix_utc() [struct _GDateTime *]
	(gint64) t [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_new_from_unix_utc(t: libc::c_long) -> *mut _GDateTime;
}


/*
gpointer g_markup_parse_context_get_user_data() [void *]
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_markup_parse_context_get_user_data(context: *mut _GMarkupParseContext) -> *mut libc::c_void;
}


/*
gboolean g_variant_type_is_array() [int]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_is_array(type_: *const _GVariantType) -> libc::c_int;
}


/*
GValue * g_value_array_get_nth() [struct _GValue *]
	(GValueArray *) value_array [struct _GValueArray *]
	(guint) index_ [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_array_get_nth(value_array: *mut _GValueArray, index_: libc::c_uint) -> *mut _GValue;
}


/*
gboolean g_static_rec_mutex_trylock() [int]
	(GStaticRecMutex *) mutex [struct _GStaticRecMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_rec_mutex_trylock(mutex: *mut _GStaticRecMutex) -> libc::c_int;
}


/*
gboolean g_source_remove() [int]
	(guint) tag [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_remove(tag: libc::c_uint) -> libc::c_int;
}


/*
void g_ptr_array_set_free_func()
	(GPtrArray *) array [struct _GPtrArray *]
	(GDestroyNotify) element_free_func [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_set_free_func(array: *mut _GPtrArray, element_free_func: Option<extern fn(*mut libc::c_void)>);
}


/*
GSList * g_slist_insert_sorted_with_data() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(gpointer) data [void *]
	(GCompareDataFunc) func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_insert_sorted_with_data(list: *mut _GSList, data: *mut libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void) -> *mut _GSList;
}


/*
gchar * g_strjoinv() [char *]
	(const gchar *) separator [const char *]
	(gchar **) str_array [char **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strjoinv(separator: *const libc::c_char, str_array: *mut *mut libc::c_char) -> *mut libc::c_char;
}


/*
GMutex * g_static_mutex_get_mutex_impl() [union _GMutex *]
	(GStaticMutex *) mutex [GStaticMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_mutex_get_mutex_impl(mutex: *mut GStaticMutex) -> *mut _GMutex;
}


/*
gboolean g_key_file_set_comment() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(const gchar *) comment [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_set_comment(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, comment: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_type_class_add_private()
	(gpointer) g_class [void *]
	(gsize) private_size [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_class_add_private(g_class: *mut libc::c_void, private_size: libc::c_ulong);
}


/*
GQuark g_quark_try_string() [unsigned int]
	(const gchar *) string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_quark_try_string(string: *const libc::c_char) -> libc::c_uint;
}


/*
void g_queue_clear()
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_clear(queue: *mut _GQueue);
}


/*
gboolean g_setenv() [int]
	(const gchar *) variable [const char *]
	(const gchar *) value [const char *]
	(gboolean) overwrite [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_setenv(variable: *const libc::c_char, value: *const libc::c_char, overwrite: libc::c_int) -> libc::c_int;
}


/*
GSList * g_slist_insert() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(gpointer) data [void *]
	(gint) position [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_insert(list: *mut _GSList, data: *mut libc::c_void, position: libc::c_int) -> *mut _GSList;
}


/*
gboolean g_node_is_ancestor() [int]
	(GNode *) node [struct _GNode *]
	(GNode *) descendant [struct _GNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_is_ancestor(node: *mut _GNode, descendant: *mut _GNode) -> libc::c_int;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_convert_with_fallback(str: *const libc::c_char, len: libc::c_long, to_codeset: *const libc::c_char, from_codeset: *const libc::c_char, fallback: *const libc::c_char, bytes_read: *mut libc::c_ulong, bytes_written: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
void g_object_add_weak_pointer()
	(GObject *) object [struct _GObject *]
	(gpointer *) weak_pointer_location [void **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_add_weak_pointer(object: *mut _GObject, weak_pointer_location: *mut *mut libc::c_void);
}


/*
void g_bookmark_file_set_modified()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(time_t) modified [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_set_modified(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, modified: libc::c_long);
}


/*
GParamSpec * g_param_spec_value_array() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(GParamSpec *) element_spec [struct _GParamSpec *]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_value_array(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, element_spec: *mut _GParamSpec, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
gint g_type_add_instance_private() [int]
	(GType) class_type [unsigned long]
	(gsize) private_size [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_add_instance_private(class_type: libc::c_ulong, private_size: libc::c_ulong) -> libc::c_int;
}


/*
GList * g_completion_complete() [struct _GList *]
	(GCompletion *) cmp [struct _GCompletion *]
	(const gchar *) prefix [const char *]
	(gchar **) new_prefix [char **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_completion_complete(cmp: *mut _GCompletion, prefix: *const libc::c_char, new_prefix: *mut *mut libc::c_char) -> *mut _GList;
}


/*
GString * g_string_ascii_down() [struct _GString *]
	(GString *) string [struct _GString *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_ascii_down(string: *mut _GString) -> *mut _GString;
}


/*
void g_value_set_enum()
	(GValue *) value [struct _GValue *]
	(gint) v_enum [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_enum(value: *mut _GValue, v_enum: libc::c_int);
}


/*
gboolean g_bookmark_file_get_icon() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(gchar **) href [char **]
	(gchar **) mime_type [char **]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_get_icon(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, href: *mut *mut libc::c_char, mime_type: *mut *mut libc::c_char, error: *mut *mut _GError) -> libc::c_int;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_new_utc(year: libc::c_int, month: libc::c_int, day: libc::c_int, hour: libc::c_int, minute: libc::c_int, seconds: libc::c_double) -> *mut _GDateTime;
}


/*
gchar * g_strdup() [char *]
	(const gchar *) str [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strdup(str: *const libc::c_char) -> *mut libc::c_char;
}


/*
gboolean g_thread_get_initialized() [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_get_initialized() -> libc::c_int;
}


/*
void g_rand_set_seed()
	(GRand *) rand_ [struct _GRand *]
	(guint32) seed [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rand_set_seed(rand_: *mut _GRand, seed: libc::c_uint);
}


/*
void g_value_array_free()
	(GValueArray *) value_array [struct _GValueArray *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_array_free(value_array: *mut _GValueArray);
}


/*
GBinding * g_object_bind_property_full() [struct _GBinding *]
	(gpointer) source [void *]
	(const gchar *) source_property [const char *]
	(gpointer) target [void *]
	(const gchar *) target_property [const char *]
	(GBindingFlags) flags [GBindingFlags]
	(GBindingTransformFunc) transform_to [int (*)(struct _GBinding *, const struct _GValue *, struct _GValue *, void *)]
	(GBindingTransformFunc) transform_from [int (*)(struct _GBinding *, const struct _GValue *, struct _GValue *, void *)]
	(gpointer) user_data [void *]
	(GDestroyNotify) notify [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_bind_property_full(source: *mut libc::c_void, source_property: *const libc::c_char, target: *mut libc::c_void, target_property: *const libc::c_char, flags: libc::c_uint, transform_to: Option<extern fn(*mut _GBinding, *const _GValue, *mut _GValue, *mut libc::c_void) -> libc::c_int>, transform_from: Option<extern fn(*mut _GBinding, *const _GValue, *mut _GValue, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void, notify: Option<extern fn(*mut libc::c_void)>) -> *mut _GBinding;
}


/*
gchar * g_regex_escape_nul() [char *]
	(const gchar *) string [const char *]
	(gint) length [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_escape_nul(string: *const libc::c_char, length: libc::c_int) -> *mut libc::c_char;
}


/*
gpointer g_object_get_data() [void *]
	(GObject *) object [struct _GObject *]
	(const gchar *) key [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_get_data(object: *mut _GObject, key: *const libc::c_char) -> *mut libc::c_void;
}


/*
gchar ** g_key_file_get_keys() [char **]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_get_keys(key_file: *mut _GKeyFile, group_name: *const libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut *mut libc::c_char;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_unexp_token(scanner: *mut _GScanner, expected_token: libc::c_uint, identifier_spec: *const libc::c_char, symbol_spec: *const libc::c_char, symbol_name: *const libc::c_char, message: *const libc::c_char, is_error: libc::c_int);
}


/*
void g_date_subtract_months()
	(GDate *) date [struct _GDate *]
	(guint) n_months [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_subtract_months(date: *mut _GDate, n_months: libc::c_uint);
}


/*
void g_value_set_uchar()
	(GValue *) value [struct _GValue *]
	(guchar) v_uchar [unsigned char]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_uchar(value: *mut _GValue, v_uchar: libc::c_uchar);
}


/*
gboolean g_mem_is_system_malloc() [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mem_is_system_malloc() -> libc::c_int;
}


/*
gsize g_bytes_get_size() [unsigned long]
	(GBytes *) bytes [struct _GBytes *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bytes_get_size(bytes: *mut _GBytes) -> libc::c_ulong;
}


/*
void g_test_expect_message()
	(const gchar *) log_domain [const char *]
	(GLogLevelFlags) log_level [GLogLevelFlags]
	(const gchar *) pattern [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_expect_message(log_domain: *const libc::c_char, log_level: libc::c_uint, pattern: *const libc::c_char);
}


/*
gpointer g_mem_chunk_alloc() [void *]
	(GMemChunk *) mem_chunk [struct _GMemChunk *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mem_chunk_alloc(mem_chunk: *mut _GMemChunk) -> *mut libc::c_void;
}


/*
guint g_parse_debug_string() [unsigned int]
	(const gchar *) string [const char *]
	(const GDebugKey *) keys [const struct _GDebugKey *]
	(guint) nkeys [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_parse_debug_string(string: *const libc::c_char, keys: *const _GDebugKey, nkeys: libc::c_uint) -> libc::c_uint;
}


/*
void g_source_unref()
	(GSource *) source [struct _GSource *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_unref(source: *mut _GSource);
}


/*
void g_static_mutex_init()
	(GStaticMutex *) mutex [GStaticMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_mutex_init(mutex: *mut GStaticMutex);
}


/*
void g_test_skip()
	(const gchar *) msg [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_skip(msg: *const libc::c_char);
}


/*
const gchar * g_param_spec_get_nick() [const char *]
	(GParamSpec *) pspec [struct _GParamSpec *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_get_nick(pspec: *mut _GParamSpec) -> *const libc::c_char;
}


/*
gpointer g_value_dup_boxed() [void *]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_dup_boxed(value: *const _GValue) -> *mut libc::c_void;
}


/*
GType g_pointer_type_register_static() [unsigned long]
	(const gchar *) name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_pointer_type_register_static(name: *const libc::c_char) -> libc::c_ulong;
}


/*
void g_hash_table_iter_remove()
	(GHashTableIter *) iter [struct _GHashTableIter *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_iter_remove(iter: *mut _GHashTableIter);
}


/*
gboolean g_double_equal() [int]
	(gconstpointer) v1 [const void *]
	(gconstpointer) v2 [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_double_equal(v1: *const libc::c_void, v2: *const libc::c_void) -> libc::c_int;
}


/*
void g_closure_add_marshal_guards()
	(GClosure *) closure [struct _GClosure *]
	(gpointer) pre_marshal_data [void *]
	(GClosureNotify) pre_marshal_notify [void (*)(void *, struct _GClosure *)]
	(gpointer) post_marshal_data [void *]
	(GClosureNotify) post_marshal_notify [void (*)(void *, struct _GClosure *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_closure_add_marshal_guards(closure: *mut _GClosure, pre_marshal_data: *mut libc::c_void, pre_marshal_notify: Option<extern fn(*mut libc::c_void, *mut _GClosure)>, post_marshal_data: *mut libc::c_void, post_marshal_notify: Option<extern fn(*mut libc::c_void, *mut _GClosure)>);
}


/*
guint g_value_get_uint() [unsigned int]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_uint(value: *const _GValue) -> libc::c_uint;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_traverse(root: *mut _GNode, order: libc::c_uint, flags: libc::c_uint, max_depth: libc::c_int, func: Option<extern fn(*mut _GNode, *mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void);
}


/*
GBytes * g_string_free_to_bytes() [struct _GBytes *]
	(GString *) string [struct _GString *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_free_to_bytes(string: *mut _GString) -> *mut _GBytes;
}


/*
const gchar * g_get_user_cache_dir() [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_user_cache_dir() -> *const libc::c_char;
}


/*
void g_markup_parse_context_get_position()
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
	(gint *) line_number [int *]
	(gint *) char_number [int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_markup_parse_context_get_position(context: *mut _GMarkupParseContext, line_number: *mut libc::c_int, char_number: *mut libc::c_int);
}


/*
const gchar * g_test_get_filename() [const char *]
	(GTestFileType) file_type [GTestFileType]
	(const gchar *) first_path [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_get_filename(file_type: libc::c_uint, first_path: *const libc::c_char) -> *const libc::c_char;
}


/*
void g_thread_pool_set_max_idle_time()
	(guint) interval [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_pool_set_max_idle_time(interval: libc::c_uint);
}


/*
gchar * g_variant_print() [char *]
	(GVariant *) value [struct _GVariant *]
	(gboolean) type_annotate [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_print(value: *mut _GVariant, type_annotate: libc::c_int) -> *mut libc::c_char;
}


/*
gint32 g_test_rand_int_range() [int]
	(gint32) begin [int]
	(gint32) end [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_rand_int_range(begin: libc::c_int, end: libc::c_int) -> libc::c_int;
}


/*
gboolean g_pointer_bit_trylock() [int]
	(volatile void *) address
	(gint) lock_bit [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_pointer_bit_trylock(address: *mut libc::c_void, lock_bit: libc::c_int) -> libc::c_int;
}


/*
gboolean g_date_time_is_daylight_savings() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_is_daylight_savings(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
void g_string_printf()
	(GString *) string [struct _GString *]
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_printf(string: *mut _GString, format: *const libc::c_char);
}


/*
void g_signal_stop_emission()
	(gpointer) instance [void *]
	(guint) signal_id [unsigned int]
	(GQuark) detail [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_stop_emission(instance: *mut libc::c_void, signal_id: libc::c_uint, detail: libc::c_uint);
}


/*
GParamSpec * g_param_spec_long() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(glong) minimum [long]
	(glong) maximum [long]
	(glong) default_value [long]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_long(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, minimum: libc::c_long, maximum: libc::c_long, default_value: libc::c_long, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
void g_value_set_object_take_ownership()
	(GValue *) value [struct _GValue *]
	(gpointer) v_object [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_object_take_ownership(value: *mut _GValue, v_object: *mut libc::c_void);
}


/*
GList * g_list_insert_before() [struct _GList *]
	(GList *) list [struct _GList *]
	(GList *) sibling [struct _GList *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_insert_before(list: *mut _GList, sibling: *mut _GList, data: *mut libc::c_void) -> *mut _GList;
}


/*
const gchar * g_get_user_name() [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_user_name() -> *const libc::c_char;
}


/*
void g_type_add_interface_static()
	(GType) instance_type [unsigned long]
	(GType) interface_type [unsigned long]
	(const GInterfaceInfo *) info [const struct _GInterfaceInfo *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_add_interface_static(instance_type: libc::c_ulong, interface_type: libc::c_ulong, info: *const _GInterfaceInfo);
}


/*
gboolean g_variant_is_container() [int]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_is_container(value: *mut _GVariant) -> libc::c_int;
}


/*
GList * g_queue_peek_nth_link() [struct _GList *]
	(GQueue *) queue [struct _GQueue *]
	(guint) n [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_peek_nth_link(queue: *mut _GQueue, n: libc::c_uint) -> *mut _GList;
}


/*
void g_main_context_invoke_full()
	(GMainContext *) context [struct _GMainContext *]
	(gint) priority [int]
	(GSourceFunc) function [int (*)(void *)]
	(gpointer) data [void *]
	(GDestroyNotify) notify [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_invoke_full(context: *mut _GMainContext, priority: libc::c_int, function: Option<extern fn(*mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void, notify: Option<extern fn(*mut libc::c_void)>);
}


/*
GIOChannel * g_io_channel_unix_new() [struct _GIOChannel *]
	(int) fd
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_unix_new(fd: libc::c_int) -> *mut _GIOChannel;
}


/*
GParamSpec * g_param_spec_int() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(gint) minimum [int]
	(gint) maximum [int]
	(gint) default_value [int]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_int(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, minimum: libc::c_int, maximum: libc::c_int, default_value: libc::c_int, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
void g_closure_invoke()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_closure_invoke(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void);
}


/*
void g_cclosure_marshal_VOID__ULONG()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_VOID__ULONG(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
GNode * g_node_copy() [struct _GNode *]
	(GNode *) node [struct _GNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_copy(node: *mut _GNode) -> *mut _GNode;
}


/*
GTimeZone * g_time_zone_new_utc() [struct _GTimeZone *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_time_zone_new_utc() -> *mut _GTimeZone;
}


/*
void g_cclosure_marshal_VOID__INT()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_VOID__INT(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
void g_rec_mutex_clear()
	(GRecMutex *) rec_mutex [struct _GRecMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rec_mutex_clear(rec_mutex: *mut _GRecMutex);
}


/*
gpointer g_object_connect() [void *]
	(gpointer) object [void *]
	(const gchar *) signal_spec [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_connect(object: *mut libc::c_void, signal_spec: *const libc::c_char) -> *mut libc::c_void;
}


/*
GVariant * g_value_dup_variant() [struct _GVariant *]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_dup_variant(value: *const _GValue) -> *mut _GVariant;
}


/*
void g_cclosure_marshal_BOOLEAN__BOXED_BOXEDv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_cclosure_marshal_BOOLEAN__BOXED_BOXEDv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
gpointer g_scanner_scope_lookup_symbol() [void *]
	(GScanner *) scanner [struct _GScanner *]
	(guint) scope_id [unsigned int]
	(const gchar *) symbol [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_scope_lookup_symbol(scanner: *mut _GScanner, scope_id: libc::c_uint, symbol: *const libc::c_char) -> *mut libc::c_void;
}


/*
void g_cclosure_marshal_VOID__LONG()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_VOID__LONG(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
GSource * g_main_context_find_source_by_id() [struct _GSource *]
	(GMainContext *) context [struct _GMainContext *]
	(guint) source_id [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_find_source_by_id(context: *mut _GMainContext, source_id: libc::c_uint) -> *mut _GSource;
}


/*
void g_signal_set_va_marshaller()
	(guint) signal_id [unsigned int]
	(GType) instance_type [unsigned long]
	(GSignalCVaMarshaller) va_marshaller [void (*)(struct _GClosure *, struct _GValue *, void *, struct __va_list_tag *, void *, int, unsigned long *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
//	pub fn g_signal_set_va_marshaller(signal_id: libc::c_uint, instance_type: libc::c_ulong, va_marshaller: Option<extern fn(*mut _GClosure, *mut _GValue, *mut libc::c_void, *mut __va_list_tag, *mut libc::c_void, libc::c_int, *mut libc::c_ulong)>);
}


/*
gboolean g_main_context_prepare() [int]
	(GMainContext *) context [struct _GMainContext *]
	(gint *) priority [int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_prepare(context: *mut _GMainContext, priority: *mut libc::c_int) -> libc::c_int;
}


/*
glong g_value_get_long() [long]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_long(value: *const _GValue) -> libc::c_long;
}


/*
GType g_type_fundamental() [unsigned long]
	(GType) type_id [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_fundamental(type_id: libc::c_ulong) -> libc::c_ulong;
}


/*
GNode * g_node_prepend() [struct _GNode *]
	(GNode *) parent [struct _GNode *]
	(GNode *) node [struct _GNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_prepend(parent: *mut _GNode, node: *mut _GNode) -> *mut _GNode;
}


/*
gchar * g_strnfill() [char *]
	(gsize) length [unsigned long]
	(gchar) fill_char [char]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strnfill(length: libc::c_ulong, fill_char: libc::c_char) -> *mut libc::c_char;
}


/*
void g_key_file_set_integer()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gint) value [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_set_integer(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, value: libc::c_int);
}


/*
gchar * g_get_codeset() [char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_codeset() -> *mut libc::c_char;
}


/*
GQuark g_key_file_error_quark() [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_error_quark() -> libc::c_uint;
}


/*
gboolean g_value_transform() [int]
	(const GValue *) src_value [const struct _GValue *]
	(GValue *) dest_value [struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_transform(src_value: *const _GValue, dest_value: *mut _GValue) -> libc::c_int;
}


/*
gchar * g_key_file_get_comment() [char *]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_get_comment(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
GType g_main_loop_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_loop_get_type() -> libc::c_ulong;
}


/*
GRegexMatchFlags g_regex_get_match_flags() [GRegexMatchFlags]
	(const GRegex *) regex [const struct _GRegex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_get_match_flags(regex: *const _GRegex) -> libc::c_uint;
}


/*
gunichar * g_utf16_to_ucs4() [unsigned int *]
	(const gunichar2 *) str [const unsigned short *]
	(glong) len [long]
	(glong *) items_read [long *]
	(glong *) items_written [long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf16_to_ucs4(str: *const libc::c_ushort, len: libc::c_long, items_read: *mut libc::c_long, items_written: *mut libc::c_long, error: *mut *mut _GError) -> *mut libc::c_uint;
}


/*
void g_hash_table_iter_steal()
	(GHashTableIter *) iter [struct _GHashTableIter *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_iter_steal(iter: *mut _GHashTableIter);
}


/*
GQueue * g_queue_new() [struct _GQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_new() -> *mut _GQueue;
}


/*
void g_source_add_poll()
	(GSource *) source [struct _GSource *]
	(GPollFD *) fd [struct _GPollFD *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_add_poll(source: *mut _GSource, fd: *mut _GPollFD);
}


/*
void g_reload_user_special_dirs_cache()
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_reload_user_special_dirs_cache();
}


/*
GList * g_queue_pop_tail_link() [struct _GList *]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_pop_tail_link(queue: *mut _GQueue) -> *mut _GList;
}


/*
const gchar * g_path_skip_root() [const char *]
	(const gchar *) file_name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_path_skip_root(file_name: *const libc::c_char) -> *const libc::c_char;
}


/*
gint g_date_compare() [int]
	(const GDate *) lhs [const struct _GDate *]
	(const GDate *) rhs [const struct _GDate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_compare(lhs: *const _GDate, rhs: *const _GDate) -> libc::c_int;
}


/*
GDateYear g_date_get_year() [unsigned short]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_get_year(date: *const _GDate) -> libc::c_ushort;
}


/*
void g_hook_prepend()
	(GHookList *) hook_list [struct _GHookList *]
	(GHook *) hook [struct _GHook *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_prepend(hook_list: *mut _GHookList, hook: *mut _GHook);
}


/*
gboolean g_option_context_get_help_enabled() [int]
	(GOptionContext *) context [struct _GOptionContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_context_get_help_enabled(context: *mut _GOptionContext) -> libc::c_int;
}


/*
void g_object_set_qdata_full()
	(GObject *) object [struct _GObject *]
	(GQuark) quark [unsigned int]
	(gpointer) data [void *]
	(GDestroyNotify) destroy [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_set_qdata_full(object: *mut _GObject, quark: libc::c_uint, data: *mut libc::c_void, destroy: Option<extern fn(*mut libc::c_void)>);
}


/*
void g_set_prgname()
	(const gchar *) prgname [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_set_prgname(prgname: *const libc::c_char);
}


/*
gpointer g_object_newv() [void *]
	(GType) object_type [unsigned long]
	(guint) n_parameters [unsigned int]
	(GParameter *) parameters [struct _GParameter *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_newv(object_type: libc::c_ulong, n_parameters: libc::c_uint, parameters: *mut _GParameter) -> *mut libc::c_void;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_spawn_async_with_pipes(working_directory: *const libc::c_char, argv: *mut *mut libc::c_char, envp: *mut *mut libc::c_char, flags: libc::c_uint, child_setup: Option<extern fn(*mut libc::c_void)>, user_data: *mut libc::c_void, child_pid: *mut libc::c_int, standard_input: *mut libc::c_int, standard_output: *mut libc::c_int, standard_error: *mut libc::c_int, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_str_is_ascii() [int]
	(const gchar *) str [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_str_is_ascii(str: *const libc::c_char) -> libc::c_int;
}


/*
guint g_static_rec_mutex_unlock_full() [unsigned int]
	(GStaticRecMutex *) mutex [struct _GStaticRecMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_rec_mutex_unlock_full(mutex: *mut _GStaticRecMutex) -> libc::c_uint;
}


/*
gboolean g_date_is_first_of_month() [int]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_is_first_of_month(date: *const _GDate) -> libc::c_int;
}


/*
GRand * g_rand_copy() [struct _GRand *]
	(GRand *) rand_ [struct _GRand *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rand_copy(rand_: *mut _GRand) -> *mut _GRand;
}


/*
void g_object_set_qdata()
	(GObject *) object [struct _GObject *]
	(GQuark) quark [unsigned int]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_set_qdata(object: *mut _GObject, quark: libc::c_uint, data: *mut libc::c_void);
}


/*
gsize g_variant_type_get_string_length() [unsigned long]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_get_string_length(type_: *const _GVariantType) -> libc::c_ulong;
}


/*
gint32 g_random_int_range() [int]
	(gint32) begin [int]
	(gint32) end [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_random_int_range(begin: libc::c_int, end: libc::c_int) -> libc::c_int;
}


/*
guint g_datalist_get_flags() [unsigned int]
	(GData **) datalist [struct _GData **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_datalist_get_flags(datalist: *mut *mut _GData) -> libc::c_uint;
}


/*
void g_checksum_reset()
	(GChecksum *) checksum [struct _GChecksum *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_checksum_reset(checksum: *mut _GChecksum);
}


/*
void g_hmac_get_digest()
	(GHmac *) hmac [struct _GHmac *]
	(guint8 *) buffer [unsigned char *]
	(gsize *) digest_len [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hmac_get_digest(hmac: *mut _GHmac, buffer: *mut libc::c_uchar, digest_len: *mut libc::c_ulong);
}


/*
void g_sequence_remove()
	(GSequenceIter *) iter [struct _GSequenceNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_remove(iter: *mut _GSequenceNode);
}


/*
gpointer g_slice_copy() [void *]
	(gsize) block_size [unsigned long]
	(gconstpointer) mem_block [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slice_copy(block_size: libc::c_ulong, mem_block: *const libc::c_void) -> *mut libc::c_void;
}


/*
void g_slice_free_chain_with_offset()
	(gsize) block_size [unsigned long]
	(gpointer) mem_chain [void *]
	(gsize) next_offset [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slice_free_chain_with_offset(block_size: libc::c_ulong, mem_chain: *mut libc::c_void, next_offset: libc::c_ulong);
}


/*
void g_ptr_array_add()
	(GPtrArray *) array [struct _GPtrArray *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_add(array: *mut _GPtrArray, data: *mut libc::c_void);
}


/*
GList * g_list_insert_sorted_with_data() [struct _GList *]
	(GList *) list [struct _GList *]
	(gpointer) data [void *]
	(GCompareDataFunc) func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_insert_sorted_with_data(list: *mut _GList, data: *mut libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void) -> *mut _GList;
}


/*
void g_hash_table_remove_all()
	(GHashTable *) hash_table [struct _GHashTable *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_remove_all(hash_table: *mut _GHashTable);
}


/*
void g_list_push_allocator()
	(GAllocator *) allocator [struct _GAllocator *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_push_allocator(allocator: *mut _GAllocator);
}


/*
gboolean g_hash_table_remove() [int]
	(GHashTable *) hash_table [struct _GHashTable *]
	(gconstpointer) key [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_remove(hash_table: *mut _GHashTable, key: *const libc::c_void) -> libc::c_int;
}


/*
GVariantClass g_variant_classify() [GVariantClass]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_classify(value: *mut _GVariant) -> libc::c_uint;
}


/*
void g_rw_lock_writer_lock()
	(GRWLock *) rw_lock [struct _GRWLock *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rw_lock_writer_lock(rw_lock: *mut _GRWLock);
}


/*
GThread * g_thread_ref() [struct _GThread *]
	(GThread *) thread [struct _GThread *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_ref(thread: *mut _GThread) -> *mut _GThread;
}


/*
gpointer g_ptr_array_remove_index_fast() [void *]
	(GPtrArray *) array [struct _GPtrArray *]
	(guint) index_ [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_remove_index_fast(array: *mut _GPtrArray, index_: libc::c_uint) -> *mut libc::c_void;
}


/*
gpointer g_object_steal_data() [void *]
	(GObject *) object [struct _GObject *]
	(const gchar *) key [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_steal_data(object: *mut _GObject, key: *const libc::c_char) -> *mut libc::c_void;
}


/*
GQuark g_regex_error_quark() [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_error_quark() -> libc::c_uint;
}


/*
void g_static_rw_lock_writer_unlock()
	(GStaticRWLock *) lock [struct _GStaticRWLock *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_rw_lock_writer_unlock(lock: *mut _GStaticRWLock);
}


/*
gpointer g_object_ref_sink() [void *]
	(gpointer) object [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_ref_sink(object: *mut libc::c_void) -> *mut libc::c_void;
}


/*
void g_static_rec_mutex_free()
	(GStaticRecMutex *) mutex [struct _GStaticRecMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_rec_mutex_free(mutex: *mut _GStaticRecMutex);
}


/*
gint g_main_context_query() [int]
	(GMainContext *) context [struct _GMainContext *]
	(gint) max_priority [int]
	(gint *) timeout_ [int *]
	(GPollFD *) fds [struct _GPollFD *]
	(gint) n_fds [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_query(context: *mut _GMainContext, max_priority: libc::c_int, timeout_: *mut libc::c_int, fds: *mut _GPollFD, n_fds: libc::c_int) -> libc::c_int;
}


/*
GVariant * g_variant_get_normal_form() [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_normal_form(value: *mut _GVariant) -> *mut _GVariant;
}


/*
GQuark g_quark_from_static_string() [unsigned int]
	(const gchar *) string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_quark_from_static_string(string: *const libc::c_char) -> libc::c_uint;
}


/*
GList * g_list_prepend() [struct _GList *]
	(GList *) list [struct _GList *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_prepend(list: *mut _GList, data: *mut libc::c_void) -> *mut _GList;
}


/*
void g_value_set_gtype()
	(GValue *) value [struct _GValue *]
	(GType) v_gtype [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_gtype(value: *mut _GValue, v_gtype: libc::c_ulong);
}


/*
void g_queue_free()
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_free(queue: *mut _GQueue);
}


/*
GType g_type_next_base() [unsigned long]
	(GType) leaf_type [unsigned long]
	(GType) root_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_next_base(leaf_type: libc::c_ulong, root_type: libc::c_ulong) -> libc::c_ulong;
}


/*
void g_timer_start()
	(GTimer *) timer [struct _GTimer *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_timer_start(timer: *mut _GTimer);
}


/*
void g_date_to_struct_tm()
	(const GDate *) date [const struct _GDate *]
	(struct tm *) tm [struct tm *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_to_struct_tm(date: *const _GDate, tm: *mut tm);
}


/*
gboolean g_markup_parse_context_end_parse() [int]
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_markup_parse_context_end_parse(context: *mut _GMarkupParseContext, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_mem_chunk_print()
	(GMemChunk *) mem_chunk [struct _GMemChunk *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mem_chunk_print(mem_chunk: *mut _GMemChunk);
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_assertion_message_cmpstr(domain: *const libc::c_char, file: *const libc::c_char, line: libc::c_int, func: *const libc::c_char, expr: *const libc::c_char, arg1: *const libc::c_char, cmp: *const libc::c_char, arg2: *const libc::c_char);
}


/*
gint g_ascii_digit_value() [int]
	(gchar) c [char]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ascii_digit_value(c: libc::c_char) -> libc::c_int;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_add_watch_full(channel: *mut _GIOChannel, priority: libc::c_int, condition: libc::c_uint, func: Option<extern fn(*mut _GIOChannel, libc::c_uint, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void, notify: Option<extern fn(*mut libc::c_void)>) -> libc::c_uint;
}


/*
gboolean g_variant_type_string_is_valid() [int]
	(const gchar *) type_string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_string_is_valid(type_string: *const libc::c_char) -> libc::c_int;
}


/*
GRegex * g_regex_new() [struct _GRegex *]
	(const gchar *) pattern [const char *]
	(GRegexCompileFlags) compile_options [GRegexCompileFlags]
	(GRegexMatchFlags) match_options [GRegexMatchFlags]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_new(pattern: *const libc::c_char, compile_options: libc::c_uint, match_options: libc::c_uint, error: *mut *mut _GError) -> *mut _GRegex;
}


/*
GList * g_list_nth() [struct _GList *]
	(GList *) list [struct _GList *]
	(guint) n [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_nth(list: *mut _GList, n: libc::c_uint) -> *mut _GList;
}


/*
guint g_date_get_day_of_year() [unsigned int]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_get_day_of_year(date: *const _GDate) -> libc::c_uint;
}


/*
void g_datalist_init()
	(GData **) datalist [struct _GData **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_datalist_init(datalist: *mut *mut _GData);
}


/*
gboolean g_source_is_destroyed() [int]
	(GSource *) source [struct _GSource *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_is_destroyed(source: *mut _GSource) -> libc::c_int;
}


/*
void g_variant_builder_init()
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_builder_init(builder: *mut _GVariantBuilder, type_: *const _GVariantType);
}


/*
GMatchInfo * g_match_info_ref() [struct _GMatchInfo *]
	(GMatchInfo *) match_info [struct _GMatchInfo *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_match_info_ref(match_info: *mut _GMatchInfo) -> *mut _GMatchInfo;
}


/*
void g_test_add_data_func()
	(const char *) testpath
	(gconstpointer) test_data [const void *]
	(GTestDataFunc) test_func [void (*)(const void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_add_data_func(testpath: *const libc::c_char, test_data: *const libc::c_void, test_func: Option<extern fn(*const libc::c_void)>);
}


/*
void g_cclosure_marshal_VOID__UINT()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_VOID__UINT(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
void g_variant_builder_add()
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
	(const gchar *) format_string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_builder_add(builder: *mut _GVariantBuilder, format_string: *const libc::c_char);
}


/*
gchar * g_ascii_formatd() [char *]
	(gchar *) buffer [char *]
	(gint) buf_len [int]
	(const gchar *) format [const char *]
	(gdouble) d [double]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ascii_formatd(buffer: *mut libc::c_char, buf_len: libc::c_int, format: *const libc::c_char, d: libc::c_double) -> *mut libc::c_char;
}


/*
void g_bookmark_file_set_title()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) title [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_set_title(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, title: *const libc::c_char);
}


/*
void g_enum_complete_type_info()
	(GType) g_enum_type [unsigned long]
	(GTypeInfo *) info [struct _GTypeInfo *]
	(const GEnumValue *) const_values [const struct _GEnumValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_enum_complete_type_info(g_enum_type: libc::c_ulong, info: *mut _GTypeInfo, const_values: *const _GEnumValue);
}


/*
void g_propagate_prefixed_error()
	(GError **) dest [struct _GError **]
	(GError *) src [struct _GError *]
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_propagate_prefixed_error(dest: *mut *mut _GError, src: *mut _GError, format: *const libc::c_char);
}


/*
GType g_type_fundamental_next() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_fundamental_next() -> libc::c_ulong;
}


/*
gboolean g_regex_get_has_cr_or_lf() [int]
	(const GRegex *) regex [const struct _GRegex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_get_has_cr_or_lf(regex: *const _GRegex) -> libc::c_int;
}


/*
gsize g_atomic_pointer_and() [unsigned long]
	(volatile void *) atomic
	(gsize) val [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_atomic_pointer_and(atomic: *mut libc::c_void, val: libc::c_ulong) -> libc::c_ulong;
}


/*
GThreadPool * g_thread_pool_new() [struct _GThreadPool *]
	(GFunc) func [void (*)(void *, void *)]
	(gpointer) user_data [void *]
	(gint) max_threads [int]
	(gboolean) exclusive [int]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_pool_new(func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void, max_threads: libc::c_int, exclusive: libc::c_int, error: *mut *mut _GError) -> *mut _GThreadPool;
}


/*
void g_variant_dict_init()
	(GVariantDict *) dict [struct _GVariantDict *]
	(GVariant *) from_asv [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_dict_init(dict: *mut _GVariantDict, from_asv: *mut _GVariant);
}


/*
GType g_markup_parse_context_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_markup_parse_context_get_type() -> libc::c_ulong;
}


/*
GType g_closure_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_closure_get_type() -> libc::c_ulong;
}


/*
gboolean g_tree_remove() [int]
	(GTree *) tree [struct _GTree *]
	(gconstpointer) key [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_tree_remove(tree: *mut _GTree, key: *const libc::c_void) -> libc::c_int;
}


/*
const gchar * g_variant_get_type_string() [const char *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_type_string(value: *mut _GVariant) -> *const libc::c_char;
}


/*
void g_atexit()
	(GVoidFunc) func [void (*)(void)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_atexit(func: Option<extern fn()>);
}


/*
GVariant * g_variant_iter_next_value() [struct _GVariant *]
	(GVariantIter *) iter [struct _GVariantIter *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_iter_next_value(iter: *mut _GVariantIter) -> *mut _GVariant;
}


/*
GVariant * g_variant_new_uint64() [struct _GVariant *]
	(guint64) value [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_uint64(value: libc::c_ulong) -> *mut _GVariant;
}


/*
gboolean g_bytes_equal() [int]
	(gconstpointer) bytes1 [const void *]
	(gconstpointer) bytes2 [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bytes_equal(bytes1: *const libc::c_void, bytes2: *const libc::c_void) -> libc::c_int;
}


/*
gboolean g_tree_steal() [int]
	(GTree *) tree [struct _GTree *]
	(gconstpointer) key [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_tree_steal(tree: *mut _GTree, key: *const libc::c_void) -> libc::c_int;
}


/*
void g_ptr_array_sort_with_data()
	(GPtrArray *) array [struct _GPtrArray *]
	(GCompareDataFunc) compare_func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_sort_with_data(array: *mut _GPtrArray, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
GTree * g_tree_new_full() [struct _GTree *]
	(GCompareDataFunc) key_compare_func [int (*)(const void *, const void *, void *)]
	(gpointer) key_compare_data [void *]
	(GDestroyNotify) key_destroy_func [void (*)(void *)]
	(GDestroyNotify) value_destroy_func [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_tree_new_full(key_compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, key_compare_data: *mut libc::c_void, key_destroy_func: Option<extern fn(*mut libc::c_void)>, value_destroy_func: Option<extern fn(*mut libc::c_void)>) -> *mut _GTree;
}


/*
GVariant * g_variant_get_maybe() [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_maybe(value: *mut _GVariant) -> *mut _GVariant;
}


/*
void g_flags_complete_type_info()
	(GType) g_flags_type [unsigned long]
	(GTypeInfo *) info [struct _GTypeInfo *]
	(const GFlagsValue *) const_values [const struct _GFlagsValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_flags_complete_type_info(g_flags_type: libc::c_ulong, info: *mut _GTypeInfo, const_values: *const _GFlagsValue);
}


/*
guint g_type_get_type_registration_serial() [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_get_type_registration_serial() -> libc::c_uint;
}


/*
GParamSpec * g_param_spec_get_redirect_target() [struct _GParamSpec *]
	(GParamSpec *) pspec [struct _GParamSpec *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_get_redirect_target(pspec: *mut _GParamSpec) -> *mut _GParamSpec;
}


/*
GVariant * g_variant_new_printf() [struct _GVariant *]
	(const gchar *) format_string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_printf(format_string: *const libc::c_char) -> *mut _GVariant;
}


/*
gchar * g_strdelimit() [char *]
	(gchar *) string [char *]
	(const gchar *) delimiters [const char *]
	(gchar) new_delimiter [char]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strdelimit(string: *mut libc::c_char, delimiters: *const libc::c_char, new_delimiter: libc::c_char) -> *mut libc::c_char;
}


/*
gint g_strncasecmp() [int]
	(const gchar *) s1 [const char *]
	(const gchar *) s2 [const char *]
	(guint) n [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strncasecmp(s1: *const libc::c_char, s2: *const libc::c_char, n: libc::c_uint) -> libc::c_int;
}


/*
gpointer g_object_dup_qdata() [void *]
	(GObject *) object [struct _GObject *]
	(GQuark) quark [unsigned int]
	(GDuplicateFunc) dup_func [void *(*)(void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_dup_qdata(object: *mut _GObject, quark: libc::c_uint, dup_func: Option<extern fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void>, user_data: *mut libc::c_void) -> *mut libc::c_void;
}


/*
GList * g_list_nth_prev() [struct _GList *]
	(GList *) list [struct _GList *]
	(guint) n [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_nth_prev(list: *mut _GList, n: libc::c_uint) -> *mut _GList;
}


/*
const gchar * g_type_name_from_class() [const char *]
	(GTypeClass *) g_class [struct _GTypeClass *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_name_from_class(g_class: *mut _GTypeClass) -> *const libc::c_char;
}


/*
gboolean g_key_file_remove_group() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_remove_group(key_file: *mut _GKeyFile, group_name: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_scanner_sync_file_offset()
	(GScanner *) scanner [struct _GScanner *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_sync_file_offset(scanner: *mut _GScanner);
}


/*
GRegex * g_regex_ref() [struct _GRegex *]
	(GRegex *) regex [struct _GRegex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_ref(regex: *mut _GRegex) -> *mut _GRegex;
}


/*
void g_hook_unref()
	(GHookList *) hook_list [struct _GHookList *]
	(GHook *) hook [struct _GHook *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_unref(hook_list: *mut _GHookList, hook: *mut _GHook);
}


/*
void g_mutex_unlock()
	(GMutex *) mutex [union _GMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mutex_unlock(mutex: *mut _GMutex);
}


/*
gpointer g_slist_nth_data() [void *]
	(GSList *) list [struct _GSList *]
	(guint) n [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_nth_data(list: *mut _GSList, n: libc::c_uint) -> *mut libc::c_void;
}


/*
gboolean g_ptr_array_remove_fast() [int]
	(GPtrArray *) array [struct _GPtrArray *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_remove_fast(array: *mut _GPtrArray, data: *mut libc::c_void) -> libc::c_int;
}


/*
GIOStatus g_io_channel_read_line() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gchar **) str_return [char **]
	(gsize *) length [unsigned long *]
	(gsize *) terminator_pos [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_read_line(channel: *mut _GIOChannel, str_return: *mut *mut libc::c_char, length: *mut libc::c_ulong, terminator_pos: *mut libc::c_ulong, error: *mut *mut _GError) -> libc::c_uint;
}


/*
GString * g_string_insert_unichar() [struct _GString *]
	(GString *) string [struct _GString *]
	(gssize) pos [long]
	(gunichar) wc [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_insert_unichar(string: *mut _GString, pos: libc::c_long, wc: libc::c_uint) -> *mut _GString;
}


/*
gboolean g_param_value_defaults() [int]
	(GParamSpec *) pspec [struct _GParamSpec *]
	(GValue *) value [struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_value_defaults(pspec: *mut _GParamSpec, value: *mut _GValue) -> libc::c_int;
}


/*
GType g_binding_flags_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_binding_flags_get_type() -> libc::c_ulong;
}


/*
void g_variant_store()
	(GVariant *) value [struct _GVariant *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_store(value: *mut _GVariant, data: *mut libc::c_void);
}


/*
gchar * g_markup_escape_text() [char *]
	(const gchar *) text [const char *]
	(gssize) length [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_markup_escape_text(text: *const libc::c_char, length: libc::c_long) -> *mut libc::c_char;
}


/*
const gchar * g_variant_type_peek_string() [const char *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_peek_string(type_: *const _GVariantType) -> *const libc::c_char;
}


/*
gboolean g_option_context_parse() [int]
	(GOptionContext *) context [struct _GOptionContext *]
	(gint *) argc [int *]
	(gchar ***) argv [char ***]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_context_parse(context: *mut _GOptionContext, argc: *mut libc::c_int, argv: *mut *mut *mut libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_type_plugin_use()
	(GTypePlugin *) plugin [struct _GTypePlugin *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_plugin_use(plugin: *mut _GTypePlugin);
}


/*
GNode * g_node_new() [struct _GNode *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_new(data: *mut libc::c_void) -> *mut _GNode;
}


/*
GList * g_list_reverse() [struct _GList *]
	(GList *) list [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_reverse(list: *mut _GList) -> *mut _GList;
}


/*
gint g_main_context_check() [int]
	(GMainContext *) context [struct _GMainContext *]
	(gint) max_priority [int]
	(GPollFD *) fds [struct _GPollFD *]
	(gint) n_fds [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_check(context: *mut _GMainContext, max_priority: libc::c_int, fds: *mut _GPollFD, n_fds: libc::c_int) -> libc::c_int;
}


/*
GVariant * g_variant_parse() [struct _GVariant *]
	(const GVariantType *) type [const struct _GVariantType *]
	(const gchar *) text [const char *]
	(const gchar *) limit [const char *]
	(const gchar **) endptr [const char **]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_parse(type_: *const _GVariantType, text: *const libc::c_char, limit: *const libc::c_char, endptr: *mut *const libc::c_char, error: *mut *mut _GError) -> *mut _GVariant;
}


/*
void g_sequence_sort()
	(GSequence *) seq [struct _GSequence *]
	(GCompareDataFunc) cmp_func [int (*)(const void *, const void *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_sort(seq: *mut _GSequence, cmp_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void);
}


/*
void g_option_context_set_summary()
	(GOptionContext *) context [struct _GOptionContext *]
	(const gchar *) summary [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_context_set_summary(context: *mut _GOptionContext, summary: *const libc::c_char);
}


/*
GType g_ptr_array_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_get_type() -> libc::c_ulong;
}


/*
const gchar * g_hmac_get_string() [const char *]
	(GHmac *) hmac [struct _GHmac *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hmac_get_string(hmac: *mut _GHmac) -> *const libc::c_char;
}


/*
void g_cclosure_marshal_BOOLEAN__BOXED_BOXED()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_BOOLEAN__BOXED_BOXED(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
GByteArray * g_byte_array_new_take() [struct _GByteArray *]
	(guint8 *) data [unsigned char *]
	(gsize) len [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_byte_array_new_take(data: *mut libc::c_uchar, len: libc::c_ulong) -> *mut _GByteArray;
}


/*
GParamSpec * g_param_spec_double() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(gdouble) minimum [double]
	(gdouble) maximum [double]
	(gdouble) default_value [double]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_double(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, minimum: libc::c_double, maximum: libc::c_double, default_value: libc::c_double, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
GIOStatus g_io_channel_read_to_end() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gchar **) str_return [char **]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_read_to_end(channel: *mut _GIOChannel, str_return: *mut *mut libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> libc::c_uint;
}


/*
void g_variant_type_free()
	(GVariantType *) type [struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_free(type_: *mut _GVariantType);
}


/*
gdouble g_random_double() [double]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_random_double() -> libc::c_double;
}


/*
void g_binding_unbind()
	(GBinding *) binding [struct _GBinding *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_binding_unbind(binding: *mut _GBinding);
}


/*
void g_closure_add_finalize_notifier()
	(GClosure *) closure [struct _GClosure *]
	(gpointer) notify_data [void *]
	(GClosureNotify) notify_func [void (*)(void *, struct _GClosure *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_closure_add_finalize_notifier(closure: *mut _GClosure, notify_data: *mut libc::c_void, notify_func: Option<extern fn(*mut libc::c_void, *mut _GClosure)>);
}


/*
gint g_ascii_strcasecmp() [int]
	(const gchar *) s1 [const char *]
	(const gchar *) s2 [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ascii_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
}


/*
void g_signal_remove_emission_hook()
	(guint) signal_id [unsigned int]
	(gulong) hook_id [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_remove_emission_hook(signal_id: libc::c_uint, hook_id: libc::c_ulong);
}


/*
gchar * g_compute_checksum_for_data() [char *]
	(GChecksumType) checksum_type [GChecksumType]
	(const guchar *) data [const unsigned char *]
	(gsize) length [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_compute_checksum_for_data(checksum_type: libc::c_uint, data: *const libc::c_uchar, length: libc::c_ulong) -> *mut libc::c_char;
}


/*
gboolean g_hash_table_insert() [int]
	(GHashTable *) hash_table [struct _GHashTable *]
	(gpointer) key [void *]
	(gpointer) value [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_insert(hash_table: *mut _GHashTable, key: *mut libc::c_void, value: *mut libc::c_void) -> libc::c_int;
}


/*
void g_tree_foreach()
	(GTree *) tree [struct _GTree *]
	(GTraverseFunc) func [int (*)(void *, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_tree_foreach(tree: *mut _GTree, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
gint g_value_get_enum() [int]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_enum(value: *const _GValue) -> libc::c_int;
}


/*
void g_closure_set_marshal()
	(GClosure *) closure [struct _GClosure *]
	(GClosureMarshal) marshal [void (*)(struct _GClosure *, struct _GValue *, unsigned int, const struct _GValue *, void *, void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_closure_set_marshal(closure: *mut _GClosure, marshal: Option<extern fn(*mut _GClosure, *mut _GValue, libc::c_uint, *const _GValue, *mut libc::c_void, *mut libc::c_void)>);
}


/*
const gchar * g_get_user_special_dir() [const char *]
	(GUserDirectory) directory [GUserDirectory]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_user_special_dir(directory: libc::c_uint) -> *const libc::c_char;
}


/*
void g_cclosure_marshal_VOID__BOOLEANv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	// pub fn g_cclosure_marshal_VOID__BOOLEANv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
void g_variant_builder_add_parsed()
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_builder_add_parsed(builder: *mut _GVariantBuilder, format: *const libc::c_char);
}


/*
gint g_strcasecmp() [int]
	(const gchar *) s1 [const char *]
	(const gchar *) s2 [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
}


/*
void g_closure_add_invalidate_notifier()
	(GClosure *) closure [struct _GClosure *]
	(gpointer) notify_data [void *]
	(GClosureNotify) notify_func [void (*)(void *, struct _GClosure *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_closure_add_invalidate_notifier(closure: *mut _GClosure, notify_data: *mut libc::c_void, notify_func: Option<extern fn(*mut libc::c_void, *mut _GClosure)>);
}


/*
void g_propagate_error()
	(GError **) dest [struct _GError **]
	(GError *) src [struct _GError *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_propagate_error(dest: *mut *mut _GError, src: *mut _GError);
}


/*
gint32 g_time_zone_get_offset() [int]
	(GTimeZone *) tz [struct _GTimeZone *]
	(gint) interval [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_time_zone_get_offset(tz: *mut _GTimeZone, interval: libc::c_int) -> libc::c_int;
}


/*
void g_date_set_dmy()
	(GDate *) date [struct _GDate *]
	(GDateDay) day [unsigned char]
	(GDateMonth) month [GDateMonth]
	(GDateYear) y [unsigned short]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_set_dmy(date: *mut _GDate, day: libc::c_uchar, month: libc::c_uint, y: libc::c_ushort);
}


/*
GVariant * g_variant_new_int16() [struct _GVariant *]
	(gint16) value [short]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_int16(value: libc::c_short) -> *mut _GVariant;
}


/*
GVariantIter * g_variant_iter_new() [struct _GVariantIter *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_iter_new(value: *mut _GVariant) -> *mut _GVariantIter;
}


/*
gboolean g_string_equal() [int]
	(const GString *) v [const struct _GString *]
	(const GString *) v2 [const struct _GString *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_equal(v: *const _GString, v2: *const _GString) -> libc::c_int;
}


/*
gssize g_checksum_type_get_length() [long]
	(GChecksumType) checksum_type [GChecksumType]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_checksum_type_get_length(checksum_type: libc::c_uint) -> libc::c_long;
}


/*
GSource * g_main_current_source() [struct _GSource *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_current_source() -> *mut _GSource;
}


/*
void g_key_file_set_double_list()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gdouble []) list [double []]
	(gsize) length [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_set_double_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, list: *mut libc::c_double /* INCOMPLETEARRAY */, length: libc::c_ulong);
}


/*
GIOChannelError g_io_channel_error_from_errno() [GIOChannelError]
	(gint) en [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_error_from_errno(en: libc::c_int) -> libc::c_uint;
}


/*
GType g_enum_register_static() [unsigned long]
	(const gchar *) name [const char *]
	(const GEnumValue *) const_static_values [const struct _GEnumValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_enum_register_static(name: *const libc::c_char, const_static_values: *const _GEnumValue) -> libc::c_ulong;
}


/*
gint g_sequence_iter_get_position() [int]
	(GSequenceIter *) iter [struct _GSequenceNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_iter_get_position(iter: *mut _GSequenceNode) -> libc::c_int;
}


/*
gsize g_object_compat_control() [unsigned long]
	(gsize) what [unsigned long]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_compat_control(what: libc::c_ulong, data: *mut libc::c_void) -> libc::c_ulong;
}


/*
gchar * g_utf8_normalize() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
	(GNormalizeMode) mode [GNormalizeMode]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_normalize(str: *const libc::c_char, len: libc::c_long, mode: libc::c_uint) -> *mut libc::c_char;
}


/*
gpointer g_private_get() [void *]
	(GPrivate *) key [struct _GPrivate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_private_get(key: *mut _GPrivate) -> *mut libc::c_void;
}


/*
gchar * g_match_info_fetch() [char *]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
	(gint) match_num [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_match_info_fetch(match_info: *const _GMatchInfo, match_num: libc::c_int) -> *mut libc::c_char;
}


/*
gboolean g_main_context_acquire() [int]
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_acquire(context: *mut _GMainContext) -> libc::c_int;
}


/*
gint g_date_time_get_microsecond() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_get_microsecond(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
guint g_type_depth() [unsigned int]
	(GType) type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_depth(type_: libc::c_ulong) -> libc::c_uint;
}


/*
GList * g_hash_table_get_keys() [struct _GList *]
	(GHashTable *) hash_table [struct _GHashTable *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_get_keys(hash_table: *mut _GHashTable) -> *mut _GList;
}


/*
GByteArray * g_byte_array_ref() [struct _GByteArray *]
	(GByteArray *) array [struct _GByteArray *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_byte_array_ref(array: *mut _GByteArray) -> *mut _GByteArray;
}


/*
GDateTime * g_date_time_new_from_timeval_utc() [struct _GDateTime *]
	(const GTimeVal *) tv [const struct _GTimeVal *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_new_from_timeval_utc(tv: *const _GTimeVal) -> *mut _GDateTime;
}


/*
GBytes * g_bytes_new() [struct _GBytes *]
	(gconstpointer) data [const void *]
	(gsize) size [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bytes_new(data: *const libc::c_void, size: libc::c_ulong) -> *mut _GBytes;
}


/*
void g_key_file_set_boolean_list()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gboolean []) list [int []]
	(gsize) length [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_set_boolean_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, list: *mut libc::c_int /* INCOMPLETEARRAY */, length: libc::c_ulong);
}


/*
void g_object_set_data_full()
	(GObject *) object [struct _GObject *]
	(const gchar *) key [const char *]
	(gpointer) data [void *]
	(GDestroyNotify) destroy [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_set_data_full(object: *mut _GObject, key: *const libc::c_char, data: *mut libc::c_void, destroy: Option<extern fn(*mut libc::c_void)>);
}


/*
GParamSpec * g_value_get_param() [struct _GParamSpec *]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_param(value: *const _GValue) -> *mut _GParamSpec;
}


/*
gunichar g_unichar_totitle() [unsigned int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_totitle(c: libc::c_uint) -> libc::c_uint;
}


/*
void g_main_loop_unref()
	(GMainLoop *) loop [struct _GMainLoop *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_loop_unref(loop_: *mut _GMainLoop);
}


/*
void g_date_set_parse()
	(GDate *) date [struct _GDate *]
	(const gchar *) str [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_set_parse(date: *mut _GDate, str: *const libc::c_char);
}


/*
GValue * g_value_init() [struct _GValue *]
	(GValue *) value [struct _GValue *]
	(GType) g_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_init(value: *mut _GValue, g_type: libc::c_ulong) -> *mut _GValue;
}


/*
time_t g_bookmark_file_get_added() [long]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_get_added(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, error: *mut *mut _GError) -> libc::c_long;
}


/*
void g_option_group_set_translation_domain()
	(GOptionGroup *) group [struct _GOptionGroup *]
	(const gchar *) domain [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_group_set_translation_domain(group: *mut _GOptionGroup, domain: *const libc::c_char);
}


/*
void g_cache_value_foreach()
	(GCache *) cache [struct _GCache *]
	(GHFunc) func [void (*)(void *, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cache_value_foreach(cache: *mut _GCache, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
gboolean g_variant_type_is_variant() [int]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_is_variant(type_: *const _GVariantType) -> libc::c_int;
}


/*
void g_checksum_free()
	(GChecksum *) checksum [struct _GChecksum *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_checksum_free(checksum: *mut _GChecksum);
}


/*
void g_object_set()
	(gpointer) object [void *]
	(const gchar *) first_property_name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_set(object: *mut libc::c_void, first_property_name: *const libc::c_char, value: *mut libc::c_void, end: *mut libc::c_void);
}


/*
GType g_variant_dict_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_dict_get_type() -> libc::c_ulong;
}


/*
guint8 g_date_get_sunday_weeks_in_year() [unsigned char]
	(GDateYear) year [unsigned short]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_get_sunday_weeks_in_year(year: libc::c_ushort) -> libc::c_uchar;
}


/*
gpointer g_tree_lookup() [void *]
	(GTree *) tree [struct _GTree *]
	(gconstpointer) key [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_tree_lookup(tree: *mut _GTree, key: *const libc::c_void) -> *mut libc::c_void;
}


/*
void g_hook_insert_before()
	(GHookList *) hook_list [struct _GHookList *]
	(GHook *) sibling [struct _GHook *]
	(GHook *) hook [struct _GHook *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_insert_before(hook_list: *mut _GHookList, sibling: *mut _GHook, hook: *mut _GHook);
}


/*
gpointer g_trash_stack_peek() [void *]
	(GTrashStack **) stack_p [struct _GTrashStack **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_trash_stack_peek(stack_p: *mut *mut _GTrashStack) -> *mut libc::c_void;
}


/*
void g_hash_table_destroy()
	(GHashTable *) hash_table [struct _GHashTable *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_destroy(hash_table: *mut _GHashTable);
}


/*
void g_datalist_id_set_data_full()
	(GData **) datalist [struct _GData **]
	(GQuark) key_id [unsigned int]
	(gpointer) data [void *]
	(GDestroyNotify) destroy_func [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_datalist_id_set_data_full(datalist: *mut *mut _GData, key_id: libc::c_uint, data: *mut libc::c_void, destroy_func: Option<extern fn(*mut libc::c_void)>);
}


/*
double g_test_timer_last()
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_timer_last() -> libc::c_double;
}


/*
gboolean g_unichar_iswide() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_iswide(c: libc::c_uint) -> libc::c_int;
}


/*
GSList * g_slist_remove_link() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(GSList *) link_ [struct _GSList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_remove_link(list: *mut _GSList, link_: *mut _GSList) -> *mut _GSList;
}


/*
gint g_regex_get_capture_count() [int]
	(const GRegex *) regex [const struct _GRegex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_get_capture_count(regex: *const _GRegex) -> libc::c_int;
}


/*
time_t g_bookmark_file_get_modified() [long]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_get_modified(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, error: *mut *mut _GError) -> libc::c_long;
}


/*
void g_type_module_set_name()
	(GTypeModule *) module [struct _GTypeModule *]
	(const gchar *) name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_module_set_name(module: *mut _GTypeModule, name: *const libc::c_char);
}


/*
GIOStatus g_io_channel_write_unichar() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gunichar) thechar [unsigned int]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_write_unichar(channel: *mut _GIOChannel, thechar: libc::c_uint, error: *mut *mut _GError) -> libc::c_uint;
}


/*
GDateTime * g_date_time_new_now_local() [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_new_now_local() -> *mut _GDateTime;
}


/*
gint g_type_class_get_instance_private_offset() [int]
	(gpointer) g_class [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_class_get_instance_private_offset(g_class: *mut libc::c_void) -> libc::c_int;
}


/*
gdouble g_random_double_range() [double]
	(gdouble) begin [double]
	(gdouble) end [double]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_random_double_range(begin: libc::c_double, end: libc::c_double) -> libc::c_double;
}


/*
void g_cclosure_marshal_BOOLEAN__FLAGSv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	// pub fn g_cclosure_marshal_BOOLEAN__FLAGSv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
gint g_date_time_get_month() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_get_month(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
void g_time_val_add()
	(GTimeVal *) time_ [struct _GTimeVal *]
	(glong) microseconds [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_time_val_add(time_: *mut _GTimeVal, microseconds: libc::c_long);
}


/*
void g_slist_push_allocator()
	(GAllocator *) allocator [struct _GAllocator *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_push_allocator(allocator: *mut _GAllocator);
}


/*
gint g_mkdir_with_parents() [int]
	(const gchar *) pathname [const char *]
	(gint) mode [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mkdir_with_parents(pathname: *const libc::c_char, mode: libc::c_int) -> libc::c_int;
}


/*
void g_slist_free()
	(GSList *) list [struct _GSList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_free(list: *mut _GSList);
}


/*
void g_signal_chain_from_overridden_handler()
	(gpointer) instance [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_chain_from_overridden_handler(instance: *mut libc::c_void);
}


/*
guint g_child_watch_add() [unsigned int]
	(GPid) pid [int]
	(GChildWatchFunc) function [void (*)(int, int, void *)]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_child_watch_add(pid: libc::c_int, function: Option<extern fn(libc::c_int, libc::c_int, *mut libc::c_void)>, data: *mut libc::c_void) -> libc::c_uint;
}


/*
gboolean g_type_module_use() [int]
	(GTypeModule *) module [struct _GTypeModule *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_module_use(module: *mut _GTypeModule) -> libc::c_int;
}


/*
void g_object_class_install_property()
	(GObjectClass *) oclass [struct _GObjectClass *]
	(guint) property_id [unsigned int]
	(GParamSpec *) pspec [struct _GParamSpec *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_class_install_property(oclass: *mut _GObjectClass, property_id: libc::c_uint, pspec: *mut _GParamSpec);
}


/*
GBytes * g_variant_get_data_as_bytes() [struct _GBytes *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_data_as_bytes(value: *mut _GVariant) -> *mut _GBytes;
}


/*
void g_io_channel_set_buffer_size()
	(GIOChannel *) channel [struct _GIOChannel *]
	(gsize) size [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_set_buffer_size(channel: *mut _GIOChannel, size: libc::c_ulong);
}


/*
const gchar * g_signal_name() [const char *]
	(guint) signal_id [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_name(signal_id: libc::c_uint) -> *const libc::c_char;
}


/*
void g_value_take_boxed()
	(GValue *) value [struct _GValue *]
	(gconstpointer) v_boxed [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_take_boxed(value: *mut _GValue, v_boxed: *const libc::c_void);
}


/*
gint64 g_get_real_time() [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_real_time() -> libc::c_long;
}


/*
guint g_int64_hash() [unsigned int]
	(gconstpointer) v [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_int64_hash(v: *const libc::c_void) -> libc::c_uint;
}


/*
void g_error_free()
	(GError *) error [struct _GError *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_error_free(error: *mut _GError);
}


/*
void g_date_add_years()
	(GDate *) date [struct _GDate *]
	(guint) n_years [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_add_years(date: *mut _GDate, n_years: libc::c_uint);
}


/*
void g_cclosure_marshal_VOID__OBJECTv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	// pub fn g_cclosure_marshal_VOID__OBJECTv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
GNode * g_node_insert_before() [struct _GNode *]
	(GNode *) parent [struct _GNode *]
	(GNode *) sibling [struct _GNode *]
	(GNode *) node [struct _GNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_insert_before(parent: *mut _GNode, sibling: *mut _GNode, node: *mut _GNode) -> *mut _GNode;
}


/*
GDateTime * g_date_time_new_from_unix_local() [struct _GDateTime *]
	(gint64) t [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_new_from_unix_local(t: libc::c_long) -> *mut _GDateTime;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_from_data(type_: *const _GVariantType, data: *const libc::c_void, size: libc::c_ulong, trusted: libc::c_int, notify: Option<extern fn(*mut libc::c_void)>, user_data: *mut libc::c_void) -> *mut _GVariant;
}


/*
guint g_timeout_add_seconds() [unsigned int]
	(guint) interval [unsigned int]
	(GSourceFunc) function [int (*)(void *)]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_timeout_add_seconds(interval: libc::c_uint, function: Option<extern fn(*mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void) -> libc::c_uint;
}


/*
void g_type_init_with_debug_flags()
	(GTypeDebugFlags) debug_flags [GTypeDebugFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_init_with_debug_flags(debug_flags: libc::c_uint);
}


/*
GLogLevelFlags g_log_set_fatal_mask() [GLogLevelFlags]
	(const gchar *) log_domain [const char *]
	(GLogLevelFlags) fatal_mask [GLogLevelFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_log_set_fatal_mask(log_domain: *const libc::c_char, fatal_mask: libc::c_uint) -> libc::c_uint;
}


/*
gpointer g_type_get_qdata() [void *]
	(GType) type [unsigned long]
	(GQuark) quark [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_get_qdata(type_: libc::c_ulong, quark: libc::c_uint) -> *mut libc::c_void;
}


/*
gpointer g_thread_join() [void *]
	(GThread *) thread [struct _GThread *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_join(thread: *mut _GThread) -> *mut libc::c_void;
}


/*
GUnicodeBreakType g_unichar_break_type() [GUnicodeBreakType]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_break_type(c: libc::c_uint) -> libc::c_uint;
}


/*
gboolean g_hash_table_steal() [int]
	(GHashTable *) hash_table [struct _GHashTable *]
	(gconstpointer) key [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_steal(hash_table: *mut _GHashTable, key: *const libc::c_void) -> libc::c_int;
}


/*
GByteArray * g_byte_array_set_size() [struct _GByteArray *]
	(GByteArray *) array [struct _GByteArray *]
	(guint) length [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_byte_array_set_size(array: *mut _GByteArray, length: libc::c_uint) -> *mut _GByteArray;
}


/*
GString * g_string_new_len() [struct _GString *]
	(const gchar *) init [const char *]
	(gssize) len [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_new_len(init: *const libc::c_char, len: libc::c_long) -> *mut _GString;
}


/*
guint g_signal_new_class_handler() [unsigned int]
	(const gchar *) signal_name [const char *]
	(GType) itype [unsigned long]
	(GSignalFlags) signal_flags [GSignalFlags]
	(GCallback) class_handler [void (*)(void)]
	(GSignalAccumulator) accumulator [int (*)(struct _GSignalInvocationHint *, struct _GValue *, const struct _GValue *, void *)]
	(gpointer) accu_data [void *]
	(GSignalCMarshaller) c_marshaller [void (*)(struct _GClosure *, struct _GValue *, unsigned int, const struct _GValue *, void *, void *)]
	(GType) return_type [unsigned long]
	(guint) n_params [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_new_class_handler(signal_name: *const libc::c_char, itype: libc::c_ulong, signal_flags: libc::c_uint, class_handler: Option<extern fn()>, accumulator: Option<extern fn(*mut _GSignalInvocationHint, *mut _GValue, *const _GValue, *mut libc::c_void) -> libc::c_int>, accu_data: *mut libc::c_void, c_marshaller: Option<extern fn(*mut _GClosure, *mut _GValue, libc::c_uint, *const _GValue, *mut libc::c_void, *mut libc::c_void)>, return_type: libc::c_ulong, n_params: libc::c_uint) -> libc::c_uint;
}


/*
void g_ptr_array_foreach()
	(GPtrArray *) array [struct _GPtrArray *]
	(GFunc) func [void (*)(void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_foreach(array: *mut _GPtrArray, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
void g_queue_push_tail()
	(GQueue *) queue [struct _GQueue *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_push_tail(queue: *mut _GQueue, data: *mut libc::c_void);
}


/*
GBindingFlags g_binding_get_flags() [GBindingFlags]
	(GBinding *) binding [struct _GBinding *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_binding_get_flags(binding: *mut _GBinding) -> libc::c_uint;
}


/*
GVariant * g_variant_new_parsed_va() [struct _GVariant *]
	(const gchar *) format [const char *]
	(va_list *) app [struct __va_list_tag (*)[1]]
*/
#[link(name="gobject-2.0")]
extern "C" {
	// pub fn g_variant_new_parsed_va(format: *const libc::c_char, app: *mut [__va_list_tag; 1]) -> *mut _GVariant;
}


/*
gboolean g_datalist_id_replace_data() [int]
	(GData **) datalist [struct _GData **]
	(GQuark) key_id [unsigned int]
	(gpointer) oldval [void *]
	(gpointer) newval [void *]
	(GDestroyNotify) destroy [void (*)(void *)]
	(GDestroyNotify *) old_destroy [void (**)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_datalist_id_replace_data(datalist: *mut *mut _GData, key_id: libc::c_uint, oldval: *mut libc::c_void, newval: *mut libc::c_void, destroy: Option<extern fn(*mut libc::c_void)>, old_destroy: *mut Option<extern fn(*mut libc::c_void)>) -> libc::c_int;
}


/*
gboolean g_variant_dict_contains() [int]
	(GVariantDict *) dict [struct _GVariantDict *]
	(const gchar *) key [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_dict_contains(dict: *mut _GVariantDict, key: *const libc::c_char) -> libc::c_int;
}


/*
void g_option_group_set_parse_hooks()
	(GOptionGroup *) group [struct _GOptionGroup *]
	(GOptionParseFunc) pre_parse_func [int (*)(struct _GOptionContext *, struct _GOptionGroup *, void *, struct _GError **)]
	(GOptionParseFunc) post_parse_func [int (*)(struct _GOptionContext *, struct _GOptionGroup *, void *, struct _GError **)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_group_set_parse_hooks(group: *mut _GOptionGroup, pre_parse_func: Option<extern fn(*mut _GOptionContext, *mut _GOptionGroup, *mut libc::c_void, *mut *mut _GError) -> libc::c_int>, post_parse_func: Option<extern fn(*mut _GOptionContext, *mut _GOptionGroup, *mut libc::c_void, *mut *mut _GError) -> libc::c_int>);
}


/*
void g_cclosure_marshal_VOID__VARIANTv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	// pub fn g_cclosure_marshal_VOID__VARIANTv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
void g_bookmark_file_add_application()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) name [const char *]
	(const gchar *) exec [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_add_application(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, name: *const libc::c_char, exec: *const libc::c_char);
}


/*
GSList * g_slist_reverse() [struct _GSList *]
	(GSList *) list [struct _GSList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_reverse(list: *mut _GSList) -> *mut _GSList;
}


/*
gpointer g_type_class_peek_parent() [void *]
	(gpointer) g_class [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_class_peek_parent(g_class: *mut libc::c_void) -> *mut libc::c_void;
}


/*
gchar * g_option_context_get_help() [char *]
	(GOptionContext *) context [struct _GOptionContext *]
	(gboolean) main_help [int]
	(GOptionGroup *) group [struct _GOptionGroup *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_context_get_help(context: *mut _GOptionContext, main_help: libc::c_int, group: *mut _GOptionGroup) -> *mut libc::c_char;
}


/*
GDateTime * g_date_time_add_seconds() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
	(gdouble) seconds [double]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_add_seconds(datetime: *mut _GDateTime, seconds: libc::c_double) -> *mut _GDateTime;
}


/*
GMainContext * g_main_context_new() [struct _GMainContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_new() -> *mut _GMainContext;
}


/*
gint g_vsnprintf() [int]
	(gchar *) string [char *]
	(gulong) n [unsigned long]
	(const gchar *) format [const char *]
	(va_list) args [struct __va_list_tag [1]]
*/
#[link(name="gobject-2.0")]
extern "C" {
	// pub fn g_vsnprintf(string: *mut libc::c_char, n: libc::c_ulong, format: *const libc::c_char, args: [__va_list_tag; 1]) -> libc::c_int;
}


/*
void g_object_set_valist()
	(GObject *) object [struct _GObject *]
	(const gchar *) first_property_name [const char *]
	(va_list) var_args [struct __va_list_tag [1]]
*/
#[link(name="gobject-2.0")]
extern "C" {
	// pub fn g_object_set_valist(object: *mut _GObject, first_property_name: *const libc::c_char, var_args: [__va_list_tag; 1]);
}


/*
void g_set_error_literal()
	(GError **) err [struct _GError **]
	(GQuark) domain [unsigned int]
	(gint) code [int]
	(const gchar *) message [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_set_error_literal(err: *mut *mut _GError, domain: libc::c_uint, code: libc::c_int, message: *const libc::c_char);
}


/*
GArray * g_array_insert_vals() [struct _GArray *]
	(GArray *) array [struct _GArray *]
	(guint) index_ [unsigned int]
	(gconstpointer) data [const void *]
	(guint) len [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_array_insert_vals(array: *mut _GArray, index_: libc::c_uint, data: *const libc::c_void, len: libc::c_uint) -> *mut _GArray;
}


/*
GType g_type_register_fundamental() [unsigned long]
	(GType) type_id [unsigned long]
	(const gchar *) type_name [const char *]
	(const GTypeInfo *) info [const struct _GTypeInfo *]
	(const GTypeFundamentalInfo *) finfo [const struct _GTypeFundamentalInfo *]
	(GTypeFlags) flags [GTypeFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_register_fundamental(type_id: libc::c_ulong, type_name: *const libc::c_char, info: *const _GTypeInfo, finfo: *const _GTypeFundamentalInfo, flags: libc::c_uint) -> libc::c_ulong;
}


/*
GParamSpec * g_param_spec_ref_sink() [struct _GParamSpec *]
	(GParamSpec *) pspec [struct _GParamSpec *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_ref_sink(pspec: *mut _GParamSpec) -> *mut _GParamSpec;
}


/*
GString * g_string_prepend() [struct _GString *]
	(GString *) string [struct _GString *]
	(const gchar *) val [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_prepend(string: *mut _GString, val: *const libc::c_char) -> *mut _GString;
}


/*
GString * g_string_append_unichar() [struct _GString *]
	(GString *) string [struct _GString *]
	(gunichar) wc [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_append_unichar(string: *mut _GString, wc: libc::c_uint) -> *mut _GString;
}


/*
gboolean g_hash_table_iter_next() [int]
	(GHashTableIter *) iter [struct _GHashTableIter *]
	(gpointer *) key [void **]
	(gpointer *) value [void **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_iter_next(iter: *mut _GHashTableIter, key: *mut *mut libc::c_void, value: *mut *mut libc::c_void) -> libc::c_int;
}


/*
GParamSpec * g_param_spec_uint64() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(guint64) minimum [unsigned long]
	(guint64) maximum [unsigned long]
	(guint64) default_value [unsigned long]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_uint64(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, minimum: libc::c_ulong, maximum: libc::c_ulong, default_value: libc::c_ulong, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
gboolean g_option_context_get_ignore_unknown_options() [int]
	(GOptionContext *) context [struct _GOptionContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_context_get_ignore_unknown_options(context: *mut _GOptionContext) -> libc::c_int;
}


/*
void g_object_remove_weak_pointer()
	(GObject *) object [struct _GObject *]
	(gpointer *) weak_pointer_location [void **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_remove_weak_pointer(object: *mut _GObject, weak_pointer_location: *mut *mut libc::c_void);
}


/*
gchar * g_utf8_offset_to_pointer() [char *]
	(const gchar *) str [const char *]
	(glong) offset [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_offset_to_pointer(str: *const libc::c_char, offset: libc::c_long) -> *mut libc::c_char;
}


/*
const gchar * g_io_channel_get_encoding() [const char *]
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_get_encoding(channel: *mut _GIOChannel) -> *const libc::c_char;
}


/*
void g_date_subtract_years()
	(GDate *) date [struct _GDate *]
	(guint) n_years [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_subtract_years(date: *mut _GDate, n_years: libc::c_uint);
}


/*
gboolean g_bookmark_file_load_from_file() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) filename [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_load_from_file(bookmark: *mut _GBookmarkFile, filename: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_key_file_set_boolean()
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gboolean) value [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_set_boolean(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, value: libc::c_int);
}


/*
gchar * g_ucs4_to_utf8() [char *]
	(const gunichar *) str [const unsigned int *]
	(glong) len [long]
	(glong *) items_read [long *]
	(glong *) items_written [long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ucs4_to_utf8(str: *const libc::c_uint, len: libc::c_long, items_read: *mut libc::c_long, items_written: *mut libc::c_long, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gpointer g_async_queue_timeout_pop() [void *]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(guint64) timeout [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_timeout_pop(queue: *mut _GAsyncQueue, timeout: libc::c_ulong) -> *mut libc::c_void;
}


/*
guint g_date_get_sunday_week_of_year() [unsigned int]
	(const GDate *) date [const struct _GDate *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_get_sunday_week_of_year(date: *const _GDate) -> libc::c_uint;
}


/*
GSList * g_slist_remove() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(gconstpointer) data [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_remove(list: *mut _GSList, data: *const libc::c_void) -> *mut _GSList;
}


/*
gpointer g_queue_peek_tail() [void *]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_peek_tail(queue: *mut _GQueue) -> *mut libc::c_void;
}


/*
void g_relation_print()
	(GRelation *) relation [struct _GRelation *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_relation_print(relation: *mut _GRelation);
}


/*
gdouble * g_key_file_get_double_list() [double *]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_get_double_list(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_double;
}


/*
gdouble g_variant_get_double() [double]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_double(value: *mut _GVariant) -> libc::c_double;
}


/*
gchar * g_strchomp() [char *]
	(gchar *) string [char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strchomp(string: *mut libc::c_char) -> *mut libc::c_char;
}


/*
gchar * g_utf8_casefold() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_casefold(str: *const libc::c_char, len: libc::c_long) -> *mut libc::c_char;
}


/*
gboolean g_utf8_validate() [int]
	(const gchar *) str [const char *]
	(gssize) max_len [long]
	(const gchar **) end [const char **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_validate(str: *const libc::c_char, max_len: libc::c_long, end: *mut *const libc::c_char) -> libc::c_int;
}


/*
gboolean g_date_time_equal() [int]
	(gconstpointer) dt1 [const void *]
	(gconstpointer) dt2 [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_equal(dt1: *const libc::c_void, dt2: *const libc::c_void) -> libc::c_int;
}


/*
void g_value_take_variant()
	(GValue *) value [struct _GValue *]
	(GVariant *) variant [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_take_variant(value: *mut _GValue, variant: *mut _GVariant);
}


/*
gint64 g_get_monotonic_time() [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_monotonic_time() -> libc::c_long;
}


/*
gchar * g_utf8_collate_key() [char *]
	(const gchar *) str [const char *]
	(gssize) len [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_collate_key(str: *const libc::c_char, len: libc::c_long) -> *mut libc::c_char;
}


/*
gint g_key_file_get_integer() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_get_integer(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_unichar_get_mirror_char() [int]
	(gunichar) ch [unsigned int]
	(gunichar *) mirrored_ch [unsigned int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_get_mirror_char(ch: libc::c_uint, mirrored_ch: *mut libc::c_uint) -> libc::c_int;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_convert_with_iconv(str: *const libc::c_char, len: libc::c_long, converter: *mut _GIConv, bytes_read: *mut libc::c_ulong, bytes_written: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
GTimeSpan g_date_time_difference() [long]
	(GDateTime *) end [struct _GDateTime *]
	(GDateTime *) begin [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_difference(end: *mut _GDateTime, begin: *mut _GDateTime) -> libc::c_long;
}


/*
gboolean g_variant_type_is_dict_entry() [int]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_is_dict_entry(type_: *const _GVariantType) -> libc::c_int;
}


/*
GType g_type_module_register_type() [unsigned long]
	(GTypeModule *) module [struct _GTypeModule *]
	(GType) parent_type [unsigned long]
	(const gchar *) type_name [const char *]
	(const GTypeInfo *) type_info [const struct _GTypeInfo *]
	(GTypeFlags) flags [GTypeFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_module_register_type(module: *mut _GTypeModule, parent_type: libc::c_ulong, type_name: *const libc::c_char, type_info: *const _GTypeInfo, flags: libc::c_uint) -> libc::c_ulong;
}


/*
GMarkupParseContext * g_markup_parse_context_ref() [struct _GMarkupParseContext *]
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_markup_parse_context_ref(context: *mut _GMarkupParseContext) -> *mut _GMarkupParseContext;
}


/*
gboolean g_variant_is_object_path() [int]
	(const gchar *) string [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_is_object_path(string: *const libc::c_char) -> libc::c_int;
}


/*
gpointer g_value_get_pointer() [void *]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_pointer(value: *const _GValue) -> *mut libc::c_void;
}


/*
GClosure * g_closure_new_simple() [struct _GClosure *]
	(guint) sizeof_closure [unsigned int]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_closure_new_simple(sizeof_closure: libc::c_uint, data: *mut libc::c_void) -> *mut _GClosure;
}


/*
const char * g_test_log_type_name()
	(GTestLogType) log_type [GTestLogType]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_log_type_name(log_type: libc::c_uint) -> *const libc::c_char;
}


/*
GPtrArray * g_ptr_array_new_with_free_func() [struct _GPtrArray *]
	(GDestroyNotify) element_free_func [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_new_with_free_func(element_free_func: Option<extern fn(*mut libc::c_void)>) -> *mut _GPtrArray;
}


/*
GVariantIter * g_variant_iter_copy() [struct _GVariantIter *]
	(GVariantIter *) iter [struct _GVariantIter *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_iter_copy(iter: *mut _GVariantIter) -> *mut _GVariantIter;
}


/*
GType * g_type_interfaces() [unsigned long *]
	(GType) type [unsigned long]
	(guint *) n_interfaces [unsigned int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_interfaces(type_: libc::c_ulong, n_interfaces: *mut libc::c_uint) -> *mut libc::c_ulong;
}


/*
GVariantType * g_variant_type_new_array() [struct _GVariantType *]
	(const GVariantType *) element [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_new_array(element: *const _GVariantType) -> *mut _GVariantType;
}


/*
void g_array_sort_with_data()
	(GArray *) array [struct _GArray *]
	(GCompareDataFunc) compare_func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_array_sort_with_data(array: *mut _GArray, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
void g_mem_profile()
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mem_profile();
}


/*
GIOError g_io_channel_read() [GIOError]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gchar *) buf [char *]
	(gsize) count [unsigned long]
	(gsize *) bytes_read [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_read(channel: *mut _GIOChannel, buf: *mut libc::c_char, count: libc::c_ulong, bytes_read: *mut libc::c_ulong) -> libc::c_uint;
}


/*
void g_cond_signal()
	(GCond *) cond [struct _GCond *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cond_signal(cond: *mut _GCond);
}


/*
void g_sequence_sort_changed_iter()
	(GSequenceIter *) iter [struct _GSequenceNode *]
	(GSequenceIterCompareFunc) iter_cmp [int (*)(struct _GSequenceNode *, struct _GSequenceNode *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_sort_changed_iter(iter: *mut _GSequenceNode, iter_cmp: Option<extern fn(*mut _GSequenceNode, *mut _GSequenceNode, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void);
}


/*
const gchar * g_environ_getenv() [const char *]
	(gchar **) envp [char **]
	(const gchar *) variable [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_environ_getenv(envp: *mut *mut libc::c_char, variable: *const libc::c_char) -> *const libc::c_char;
}


/*
gboolean g_unichar_isdefined() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_isdefined(c: libc::c_uint) -> libc::c_int;
}


/*
guint g_spaced_primes_closest() [unsigned int]
	(guint) num [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_spaced_primes_closest(num: libc::c_uint) -> libc::c_uint;
}


/*
GHmac * g_hmac_new() [struct _GHmac *]
	(GChecksumType) digest_type [GChecksumType]
	(const guchar *) key [const unsigned char *]
	(gsize) key_len [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hmac_new(digest_type: libc::c_uint, key: *const libc::c_uchar, key_len: libc::c_ulong) -> *mut _GHmac;
}


/*
GType g_io_condition_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_condition_get_type() -> libc::c_ulong;
}


/*
void g_signal_handler_block()
	(gpointer) instance [void *]
	(gulong) handler_id [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_handler_block(instance: *mut libc::c_void, handler_id: libc::c_ulong);
}


/*
void g_value_set_float()
	(GValue *) value [struct _GValue *]
	(gfloat) v_float [float]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_float(value: *mut _GValue, v_float: libc::c_float);
}


/*
const GSList * g_markup_parse_context_get_element_stack() [const struct _GSList *]
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_markup_parse_context_get_element_stack(context: *mut _GMarkupParseContext) -> *const _GSList;
}


/*
GParamSpec * g_param_spec_uchar() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(guint8) minimum [unsigned char]
	(guint8) maximum [unsigned char]
	(guint8) default_value [unsigned char]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_uchar(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, minimum: libc::c_uchar, maximum: libc::c_uchar, default_value: libc::c_uchar, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
void g_value_take_string()
	(GValue *) value [struct _GValue *]
	(gchar *) v_string [char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_take_string(value: *mut _GValue, v_string: *mut libc::c_char);
}


/*
GThread * g_thread_self() [struct _GThread *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_self() -> *mut _GThread;
}


/*
guint g_signal_newv() [unsigned int]
	(const gchar *) signal_name [const char *]
	(GType) itype [unsigned long]
	(GSignalFlags) signal_flags [GSignalFlags]
	(GClosure *) class_closure [struct _GClosure *]
	(GSignalAccumulator) accumulator [int (*)(struct _GSignalInvocationHint *, struct _GValue *, const struct _GValue *, void *)]
	(gpointer) accu_data [void *]
	(GSignalCMarshaller) c_marshaller [void (*)(struct _GClosure *, struct _GValue *, unsigned int, const struct _GValue *, void *, void *)]
	(GType) return_type [unsigned long]
	(guint) n_params [unsigned int]
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_newv(signal_name: *const libc::c_char, itype: libc::c_ulong, signal_flags: libc::c_uint, class_closure: *mut _GClosure, accumulator: Option<extern fn(*mut _GSignalInvocationHint, *mut _GValue, *const _GValue, *mut libc::c_void) -> libc::c_int>, accu_data: *mut libc::c_void, c_marshaller: Option<extern fn(*mut _GClosure, *mut _GValue, libc::c_uint, *const _GValue, *mut libc::c_void, *mut libc::c_void)>, return_type: libc::c_ulong, n_params: libc::c_uint, param_types: *mut libc::c_ulong) -> libc::c_uint;
}


/*
void g_date_time_get_ymd()
	(GDateTime *) datetime [struct _GDateTime *]
	(gint *) year [int *]
	(gint *) month [int *]
	(gint *) day [int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_get_ymd(datetime: *mut _GDateTime, year: *mut libc::c_int, month: *mut libc::c_int, day: *mut libc::c_int);
}


/*
GVariant * g_value_get_variant() [struct _GVariant *]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_variant(value: *const _GValue) -> *mut _GVariant;
}


/*
void g_io_channel_set_close_on_unref()
	(GIOChannel *) channel [struct _GIOChannel *]
	(gboolean) do_close [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_set_close_on_unref(channel: *mut _GIOChannel, do_close: libc::c_int);
}


/*
void g_logv()
	(const gchar *) log_domain [const char *]
	(GLogLevelFlags) log_level [GLogLevelFlags]
	(const gchar *) format [const char *]
	(va_list) args [struct __va_list_tag [1]]
*/
#[link(name="gobject-2.0")]
extern "C" {
	// pub fn g_logv(log_domain: *const libc::c_char, log_level: libc::c_uint, format: *const libc::c_char, args: [__va_list_tag; 1]);
}


/*
const gchar * g_option_context_get_description() [const char *]
	(GOptionContext *) context [struct _GOptionContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_context_get_description(context: *mut _GOptionContext) -> *const libc::c_char;
}


/*
void g_warn_message()
	(const char *) domain
	(const char *) file
	(int) line
	(const char *) func
	(const char *) warnexpr
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_warn_message(domain: *const libc::c_char, file: *const libc::c_char, line: libc::c_int, func: *const libc::c_char, warnexpr: *const libc::c_char);
}


/*
GIOStatus g_io_channel_shutdown() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gboolean) flush [int]
	(GError **) err [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_shutdown(channel: *mut _GIOChannel, flush: libc::c_int, err: *mut *mut _GError) -> libc::c_uint;
}


/*
GType g_type_module_register_flags() [unsigned long]
	(GTypeModule *) module [struct _GTypeModule *]
	(const gchar *) name [const char *]
	(const GFlagsValue *) const_static_values [const struct _GFlagsValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_module_register_flags(module: *mut _GTypeModule, name: *const libc::c_char, const_static_values: *const _GFlagsValue) -> libc::c_ulong;
}


/*
gunichar2 * g_ucs4_to_utf16() [unsigned short *]
	(const gunichar *) str [const unsigned int *]
	(glong) len [long]
	(glong *) items_read [long *]
	(glong *) items_written [long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ucs4_to_utf16(str: *const libc::c_uint, len: libc::c_long, items_read: *mut libc::c_long, items_written: *mut libc::c_long, error: *mut *mut _GError) -> *mut libc::c_ushort;
}


/*
gboolean g_static_rw_lock_reader_trylock() [int]
	(GStaticRWLock *) lock [struct _GStaticRWLock *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_rw_lock_reader_trylock(lock: *mut _GStaticRWLock) -> libc::c_int;
}


/*
GNode * g_node_insert_after() [struct _GNode *]
	(GNode *) parent [struct _GNode *]
	(GNode *) sibling [struct _GNode *]
	(GNode *) node [struct _GNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_insert_after(parent: *mut _GNode, sibling: *mut _GNode, node: *mut _GNode) -> *mut _GNode;
}


/*
void g_queue_insert_sorted()
	(GQueue *) queue [struct _GQueue *]
	(gpointer) data [void *]
	(GCompareDataFunc) func [int (*)(const void *, const void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_insert_sorted(queue: *mut _GQueue, data: *mut libc::c_void, func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
gint32 g_test_rand_int() [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_rand_int() -> libc::c_int;
}


/*
gboolean g_type_is_a() [int]
	(GType) type [unsigned long]
	(GType) is_a_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_is_a(type_: libc::c_ulong, is_a_type: libc::c_ulong) -> libc::c_int;
}


/*
gpointer g_object_get_qdata() [void *]
	(GObject *) object [struct _GObject *]
	(GQuark) quark [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_get_qdata(object: *mut _GObject, quark: libc::c_uint) -> *mut libc::c_void;
}


/*
void g_test_suite_add()
	(GTestSuite *) suite [struct GTestSuite *]
	(GTestCase *) test_case [struct GTestCase *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_suite_add(suite: *mut GTestSuite, test_case: *mut GTestCase);
}


/*
void g_object_disconnect()
	(gpointer) object [void *]
	(const gchar *) signal_spec [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_disconnect(object: *mut libc::c_void, signal_spec: *const libc::c_char);
}


/*
GIConv g_iconv_open() [struct _GIConv *]
	(const gchar *) to_codeset [const char *]
	(const gchar *) from_codeset [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_iconv_open(to_codeset: *const libc::c_char, from_codeset: *const libc::c_char) -> *mut _GIConv;
}


/*
void g_object_weak_ref()
	(GObject *) object [struct _GObject *]
	(GWeakNotify) notify [void (*)(void *, struct _GObject *)]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_weak_ref(object: *mut _GObject, notify: Option<extern fn(*mut libc::c_void, *mut _GObject)>, data: *mut libc::c_void);
}


/*
gchar * g_compute_checksum_for_string() [char *]
	(GChecksumType) checksum_type [GChecksumType]
	(const gchar *) str [const char *]
	(gssize) length [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_compute_checksum_for_string(checksum_type: libc::c_uint, str: *const libc::c_char, length: libc::c_long) -> *mut libc::c_char;
}


/*
void g_rand_free()
	(GRand *) rand_ [struct _GRand *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rand_free(rand_: *mut _GRand);
}


/*
guint64 g_key_file_get_uint64() [unsigned long]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) group_name [const char *]
	(const gchar *) key [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_get_uint64(key_file: *mut _GKeyFile, group_name: *const libc::c_char, key: *const libc::c_char, error: *mut *mut _GError) -> libc::c_ulong;
}


/*
gchar ** g_key_file_get_groups() [char **]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(gsize *) length [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_get_groups(key_file: *mut _GKeyFile, length: *mut libc::c_ulong) -> *mut *mut libc::c_char;
}


/*
void g_dataset_destroy()
	(gconstpointer) dataset_location [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_dataset_destroy(dataset_location: *const libc::c_void);
}


/*
GType _g_param_type_register_static_constant() [unsigned long]
	(const gchar *) name [const char *]
	(const GParamSpecTypeInfo *) pspec_info [const struct _GParamSpecTypeInfo *]
	(GType) opt_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn _g_param_type_register_static_constant(name: *const libc::c_char, pspec_info: *const _GParamSpecTypeInfo, opt_type: libc::c_ulong) -> libc::c_ulong;
}


/*
void g_sequence_foreach_range()
	(GSequenceIter *) begin [struct _GSequenceNode *]
	(GSequenceIter *) end [struct _GSequenceNode *]
	(GFunc) func [void (*)(void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_foreach_range(begin: *mut _GSequenceNode, end: *mut _GSequenceNode, func: Option<extern fn(*mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
gpointer g_type_class_peek_static() [void *]
	(GType) type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_class_peek_static(type_: libc::c_ulong) -> *mut libc::c_void;
}


/*
GParamSpec * g_param_spec_boolean() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(gboolean) default_value [int]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_boolean(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, default_value: libc::c_int, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
gchar * g_format_size_full() [char *]
	(guint64) size [unsigned long]
	(GFormatSizeFlags) flags [GFormatSizeFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_format_size_full(size: libc::c_ulong, flags: libc::c_uint) -> *mut libc::c_char;
}


/*
GRand * g_rand_new_with_seed_array() [struct _GRand *]
	(const guint32 *) seed [const unsigned int *]
	(guint) seed_length [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rand_new_with_seed_array(seed: *const libc::c_uint, seed_length: libc::c_uint) -> *mut _GRand;
}


/*
guint g_atomic_int_and() [unsigned int]
	(volatile guint *) atomic [volatile unsigned int *]
	(guint) val [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_atomic_int_and(atomic: *mut libc::c_uint, val: libc::c_uint) -> libc::c_uint;
}


/*
gchar * g_utf8_substring() [char *]
	(const gchar *) str [const char *]
	(glong) start_pos [long]
	(glong) end_pos [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_substring(str: *const libc::c_char, start_pos: libc::c_long, end_pos: libc::c_long) -> *mut libc::c_char;
}


/*
void g_rand_set_seed_array()
	(GRand *) rand_ [struct _GRand *]
	(const guint32 *) seed [const unsigned int *]
	(guint) seed_length [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rand_set_seed_array(rand_: *mut _GRand, seed: *const libc::c_uint, seed_length: libc::c_uint);
}


/*
GList * g_list_sort() [struct _GList *]
	(GList *) list [struct _GList *]
	(GCompareFunc) compare_func [int (*)(const void *, const void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_sort(list: *mut _GList, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GList;
}


/*
void g_strfreev()
	(gchar **) str_array [char **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strfreev(str_array: *mut *mut libc::c_char);
}


/*
void g_object_weak_unref()
	(GObject *) object [struct _GObject *]
	(GWeakNotify) notify [void (*)(void *, struct _GObject *)]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_weak_unref(object: *mut _GObject, notify: Option<extern fn(*mut libc::c_void, *mut _GObject)>, data: *mut libc::c_void);
}


/*
const gchar * g_binding_get_target_property() [const char *]
	(GBinding *) binding [struct _GBinding *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_binding_get_target_property(binding: *mut _GBinding) -> *const libc::c_char;
}


/*
const gchar * g_type_name() [const char *]
	(GType) type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_name(type_: libc::c_ulong) -> *const libc::c_char;
}


/*
gsize g_iconv() [unsigned long]
	(GIConv) converter [struct _GIConv *]
	(gchar **) inbuf [char **]
	(gsize *) inbytes_left [unsigned long *]
	(gchar **) outbuf [char **]
	(gsize *) outbytes_left [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_iconv(converter: *mut _GIConv, inbuf: *mut *mut libc::c_char, inbytes_left: *mut libc::c_ulong, outbuf: *mut *mut libc::c_char, outbytes_left: *mut libc::c_ulong) -> libc::c_ulong;
}


/*
void g_main_context_dispatch()
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_dispatch(context: *mut _GMainContext);
}


/*
GQuark g_markup_error_quark() [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_markup_error_quark() -> libc::c_uint;
}


/*
gint g_node_child_position() [int]
	(GNode *) node [struct _GNode *]
	(GNode *) child [struct _GNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_child_position(node: *mut _GNode, child: *mut _GNode) -> libc::c_int;
}


/*
GHook * g_hook_first_valid() [struct _GHook *]
	(GHookList *) hook_list [struct _GHookList *]
	(gboolean) may_be_in_call [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_first_valid(hook_list: *mut _GHookList, may_be_in_call: libc::c_int) -> *mut _GHook;
}


/*
GDateTime * g_date_time_to_timezone() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
	(GTimeZone *) tz [struct _GTimeZone *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_to_timezone(datetime: *mut _GDateTime, tz: *mut _GTimeZone) -> *mut _GDateTime;
}


/*
void g_option_context_set_help_enabled()
	(GOptionContext *) context [struct _GOptionContext *]
	(gboolean) help_enabled [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_context_set_help_enabled(context: *mut _GOptionContext, help_enabled: libc::c_int);
}


/*
GList * g_list_append() [struct _GList *]
	(GList *) list [struct _GList *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_append(list: *mut _GList, data: *mut libc::c_void) -> *mut _GList;
}


/*
gchar * g_utf8_strrchr() [char *]
	(const gchar *) p [const char *]
	(gssize) len [long]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_strrchr(p: *const libc::c_char, len: libc::c_long, c: libc::c_uint) -> *mut libc::c_char;
}


/*
GVariant * g_variant_get_variant() [struct _GVariant *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_variant(value: *mut _GVariant) -> *mut _GVariant;
}


/*
void g_source_remove_child_source()
	(GSource *) source [struct _GSource *]
	(GSource *) child_source [struct _GSource *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_remove_child_source(source: *mut _GSource, child_source: *mut _GSource);
}


/*
void g_cclosure_marshal_VOID__ULONGv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	// pub fn g_cclosure_marshal_VOID__ULONGv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
guint g_node_max_height() [unsigned int]
	(GNode *) root [struct _GNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_max_height(root: *mut _GNode) -> libc::c_uint;
}


/*
void g_cond_broadcast()
	(GCond *) cond [struct _GCond *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cond_broadcast(cond: *mut _GCond);
}


/*
void g_static_rec_mutex_lock()
	(GStaticRecMutex *) mutex [struct _GStaticRecMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_static_rec_mutex_lock(mutex: *mut _GStaticRecMutex);
}


/*
gchar * g_bookmark_file_get_description() [char *]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_get_description(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gpointer g_list_nth_data() [void *]
	(GList *) list [struct _GList *]
	(guint) n [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_nth_data(list: *mut _GList, n: libc::c_uint) -> *mut libc::c_void;
}


/*
GSequenceIter * g_sequence_insert_before() [struct _GSequenceNode *]
	(GSequenceIter *) iter [struct _GSequenceNode *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_insert_before(iter: *mut _GSequenceNode, data: *mut libc::c_void) -> *mut _GSequenceNode;
}


/*
gint g_slist_index() [int]
	(GSList *) list [struct _GSList *]
	(gconstpointer) data [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_index(list: *mut _GSList, data: *const libc::c_void) -> libc::c_int;
}


/*
GDateTime * g_date_time_add_weeks() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
	(gint) weeks [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_add_weeks(datetime: *mut _GDateTime, weeks: libc::c_int) -> *mut _GDateTime;
}


/*
void g_datalist_foreach()
	(GData **) datalist [struct _GData **]
	(GDataForeachFunc) func [void (*)(unsigned int, void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_datalist_foreach(datalist: *mut *mut _GData, func: Option<extern fn(libc::c_uint, *mut libc::c_void, *mut libc::c_void)>, user_data: *mut libc::c_void);
}


/*
void g_value_set_object()
	(GValue *) value [struct _GValue *]
	(gpointer) v_object [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_object(value: *mut _GValue, v_object: *mut libc::c_void);
}


/*
void g_timer_reset()
	(GTimer *) timer [struct _GTimer *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_timer_reset(timer: *mut _GTimer);
}


/*
gint g_file_open_tmp() [int]
	(const gchar *) tmpl [const char *]
	(gchar **) name_used [char **]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_file_open_tmp(tmpl: *const libc::c_char, name_used: *mut *mut libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_markup_parse_context_free()
	(GMarkupParseContext *) context [struct _GMarkupParseContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_markup_parse_context_free(context: *mut _GMarkupParseContext);
}


/*
gboolean g_direct_equal() [int]
	(gconstpointer) v1 [const void *]
	(gconstpointer) v2 [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_direct_equal(v1: *const libc::c_void, v2: *const libc::c_void) -> libc::c_int;
}


/*
void g_param_value_set_default()
	(GParamSpec *) pspec [struct _GParamSpec *]
	(GValue *) value [struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_value_set_default(pspec: *mut _GParamSpec, value: *mut _GValue);
}


/*
void g_rec_mutex_unlock()
	(GRecMutex *) rec_mutex [struct _GRecMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rec_mutex_unlock(rec_mutex: *mut _GRecMutex);
}


/*
gpointer g_cache_insert() [void *]
	(GCache *) cache [struct _GCache *]
	(gpointer) key [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cache_insert(cache: *mut _GCache, key: *mut libc::c_void) -> *mut libc::c_void;
}


/*
GMemChunk * g_mem_chunk_new() [struct _GMemChunk *]
	(const gchar *) name [const char *]
	(gint) atom_size [int]
	(gsize) area_size [unsigned long]
	(gint) type [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mem_chunk_new(name: *const libc::c_char, atom_size: libc::c_int, area_size: libc::c_ulong, type_: libc::c_int) -> *mut _GMemChunk;
}


/*
gpointer g_try_malloc_n() [void *]
	(gsize) n_blocks [unsigned long]
	(gsize) n_block_bytes [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_try_malloc_n(n_blocks: libc::c_ulong, n_block_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
gboolean g_rw_lock_reader_trylock() [int]
	(GRWLock *) rw_lock [struct _GRWLock *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rw_lock_reader_trylock(rw_lock: *mut _GRWLock) -> libc::c_int;
}


/*
GHmac * g_hmac_copy() [struct _GHmac *]
	(const GHmac *) hmac [const struct _GHmac *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hmac_copy(hmac: *const _GHmac) -> *mut _GHmac;
}


/*
gboolean g_tree_lookup_extended() [int]
	(GTree *) tree [struct _GTree *]
	(gconstpointer) lookup_key [const void *]
	(gpointer *) orig_key [void **]
	(gpointer *) value [void **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_tree_lookup_extended(tree: *mut _GTree, lookup_key: *const libc::c_void, orig_key: *mut *mut libc::c_void, value: *mut *mut libc::c_void) -> libc::c_int;
}


/*
void g_slist_free_1()
	(GSList *) list [struct _GSList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_free_1(list: *mut _GSList);
}


/*
GPrivate * g_private_new() [struct _GPrivate *]
	(GDestroyNotify) notify [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_private_new(notify: Option<extern fn(*mut libc::c_void)>) -> *mut _GPrivate;
}


/*
GArray * g_array_prepend_vals() [struct _GArray *]
	(GArray *) array [struct _GArray *]
	(gconstpointer) data [const void *]
	(guint) len [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_array_prepend_vals(array: *mut _GArray, data: *const libc::c_void, len: libc::c_uint) -> *mut _GArray;
}


/*
GString * g_string_assign() [struct _GString *]
	(GString *) string [struct _GString *]
	(const gchar *) rval [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_assign(string: *mut _GString, rval: *const libc::c_char) -> *mut _GString;
}


/*
gboolean g_unichar_decompose() [int]
	(gunichar) ch [unsigned int]
	(gunichar *) a [unsigned int *]
	(gunichar *) b [unsigned int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_decompose(ch: libc::c_uint, a: *mut libc::c_uint, b: *mut libc::c_uint) -> libc::c_int;
}


/*
GNode * g_node_insert() [struct _GNode *]
	(GNode *) parent [struct _GNode *]
	(gint) position [int]
	(GNode *) node [struct _GNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_insert(parent: *mut _GNode, position: libc::c_int, node: *mut _GNode) -> *mut _GNode;
}


/*
void g_variant_iter_free()
	(GVariantIter *) iter [struct _GVariantIter *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_iter_free(iter: *mut _GVariantIter);
}


/*
gboolean g_match_info_fetch_named_pos() [int]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
	(const gchar *) name [const char *]
	(gint *) start_pos [int *]
	(gint *) end_pos [int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_match_info_fetch_named_pos(match_info: *const _GMatchInfo, name: *const libc::c_char, start_pos: *mut libc::c_int, end_pos: *mut libc::c_int) -> libc::c_int;
}


/*
gboolean g_rw_lock_writer_trylock() [int]
	(GRWLock *) rw_lock [struct _GRWLock *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rw_lock_writer_trylock(rw_lock: *mut _GRWLock) -> libc::c_int;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_add_full(datetime: *mut _GDateTime, years: libc::c_int, months: libc::c_int, days: libc::c_int, hours: libc::c_int, minutes: libc::c_int, seconds: libc::c_double) -> *mut _GDateTime;
}


/*
const gchar * g_variant_get_bytestring() [const char *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_bytestring(value: *mut _GVariant) -> *const libc::c_char;
}


/*
gchar ** g_regex_split() [char **]
	(const GRegex *) regex [const struct _GRegex *]
	(const gchar *) string [const char *]
	(GRegexMatchFlags) match_options [GRegexMatchFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_split(regex: *const _GRegex, string: *const libc::c_char, match_options: libc::c_uint) -> *mut *mut libc::c_char;
}


/*
gconstpointer g_variant_get_fixed_array() [const void *]
	(GVariant *) value [struct _GVariant *]
	(gsize *) n_elements [unsigned long *]
	(gsize) element_size [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_fixed_array(value: *mut _GVariant, n_elements: *mut libc::c_ulong, element_size: libc::c_ulong) -> *const libc::c_void;
}


/*
void g_type_plugin_complete_type_info()
	(GTypePlugin *) plugin [struct _GTypePlugin *]
	(GType) g_type [unsigned long]
	(GTypeInfo *) info [struct _GTypeInfo *]
	(GTypeValueTable *) value_table [struct _GTypeValueTable *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_plugin_complete_type_info(plugin: *mut _GTypePlugin, g_type: libc::c_ulong, info: *mut _GTypeInfo, value_table: *mut _GTypeValueTable);
}


/*
GTokenType g_scanner_peek_next_token() [GTokenType]
	(GScanner *) scanner [struct _GScanner *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_peek_next_token(scanner: *mut _GScanner) -> libc::c_uint;
}


/*
GList * g_queue_find() [struct _GList *]
	(GQueue *) queue [struct _GQueue *]
	(gconstpointer) data [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_find(queue: *mut _GQueue, data: *const libc::c_void) -> *mut _GList;
}


/*
GParamSpec * g_param_spec_uint() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(guint) minimum [unsigned int]
	(guint) maximum [unsigned int]
	(guint) default_value [unsigned int]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_uint(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, minimum: libc::c_uint, maximum: libc::c_uint, default_value: libc::c_uint, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
gchar * g_bookmark_file_to_data() [char *]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(gsize *) length [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_to_data(bookmark: *mut _GBookmarkFile, length: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
GMainContext * g_main_loop_get_context() [struct _GMainContext *]
	(GMainLoop *) loop [struct _GMainLoop *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_loop_get_context(loop_: *mut _GMainLoop) -> *mut _GMainContext;
}


/*
void g_blow_chunks()
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_blow_chunks();
}


/*
GIOStatus g_io_channel_seek_position() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gint64) offset [long]
	(GSeekType) type [GSeekType]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_seek_position(channel: *mut _GIOChannel, offset: libc::c_long, type_: libc::c_uint, error: *mut *mut _GError) -> libc::c_uint;
}


/*
GNode * g_node_last_sibling() [struct _GNode *]
	(GNode *) node [struct _GNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_last_sibling(node: *mut _GNode) -> *mut _GNode;
}


/*
gpointer g_malloc_n() [void *]
	(gsize) n_blocks [unsigned long]
	(gsize) n_block_bytes [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_malloc_n(n_blocks: libc::c_ulong, n_block_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
GVariant * g_variant_new_int32() [struct _GVariant *]
	(gint32) value [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_int32(value: libc::c_int) -> *mut _GVariant;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_replace_eval(regex: *const _GRegex, string: *const libc::c_char, string_len: libc::c_long, start_position: libc::c_int, match_options: libc::c_uint, eval: Option<extern fn(*const _GMatchInfo, *mut _GString, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gboolean g_hash_table_lookup_extended() [int]
	(GHashTable *) hash_table [struct _GHashTable *]
	(gconstpointer) lookup_key [const void *]
	(gpointer *) orig_key [void **]
	(gpointer *) value [void **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_lookup_extended(hash_table: *mut _GHashTable, lookup_key: *const libc::c_void, orig_key: *mut *mut libc::c_void, value: *mut *mut libc::c_void) -> libc::c_int;
}


/*
GType g_value_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_type() -> libc::c_ulong;
}


/*
gboolean g_test_subprocess() [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_subprocess() -> libc::c_int;
}


/*
GDateTime * g_date_time_add_months() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
	(gint) months [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_add_months(datetime: *mut _GDateTime, months: libc::c_int) -> *mut _GDateTime;
}


/*
void g_test_set_nonfatal_assertions()
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_set_nonfatal_assertions();
}


/*
gchar * g_strrstr_len() [char *]
	(const gchar *) haystack [const char *]
	(gssize) haystack_len [long]
	(const gchar *) needle [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strrstr_len(haystack: *const libc::c_char, haystack_len: libc::c_long, needle: *const libc::c_char) -> *mut libc::c_char;
}


/*
const GVariantType * g_variant_type_element() [const struct _GVariantType *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_element(type_: *const _GVariantType) -> *const _GVariantType;
}


/*
void g_type_module_unuse()
	(GTypeModule *) module [struct _GTypeModule *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_module_unuse(module: *mut _GTypeModule);
}


/*
gpointer g_queue_peek_nth() [void *]
	(GQueue *) queue [struct _GQueue *]
	(guint) n [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_peek_nth(queue: *mut _GQueue, n: libc::c_uint) -> *mut libc::c_void;
}


/*
GIOStatus g_io_channel_flush() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_flush(channel: *mut _GIOChannel, error: *mut *mut _GError) -> libc::c_uint;
}


/*
GSList * g_slist_append() [struct _GSList *]
	(GSList *) list [struct _GSList *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_append(list: *mut _GSList, data: *mut libc::c_void) -> *mut _GSList;
}


/*
void g_object_get()
	(gpointer) object [void *]
	(const gchar *) first_property_name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_get(object: *mut libc::c_void, first_property_name: *const libc::c_char);
}


/*
GType g_gstring_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_gstring_get_type() -> libc::c_ulong;
}


/*
GMainLoop * g_main_loop_ref() [struct _GMainLoop *]
	(GMainLoop *) loop [struct _GMainLoop *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_loop_ref(loop_: *mut _GMainLoop) -> *mut _GMainLoop;
}


/*
gboolean g_bookmark_file_move_item() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) old_uri [const char *]
	(const gchar *) new_uri [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_move_item(bookmark: *mut _GBookmarkFile, old_uri: *const libc::c_char, new_uri: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_byte_array_sort()
	(GByteArray *) array [struct _GByteArray *]
	(GCompareFunc) compare_func [int (*)(const void *, const void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_byte_array_sort(array: *mut _GByteArray, compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>);
}


/*
gboolean g_match_info_fetch_pos() [int]
	(const GMatchInfo *) match_info [const struct _GMatchInfo *]
	(gint) match_num [int]
	(gint *) start_pos [int *]
	(gint *) end_pos [int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_match_info_fetch_pos(match_info: *const _GMatchInfo, match_num: libc::c_int, start_pos: *mut libc::c_int, end_pos: *mut libc::c_int) -> libc::c_int;
}


/*
GType g_type_from_name() [unsigned long]
	(const gchar *) name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_from_name(name: *const libc::c_char) -> libc::c_ulong;
}


/*
GList * g_queue_pop_head_link() [struct _GList *]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_pop_head_link(queue: *mut _GQueue) -> *mut _GList;
}


/*
GList * g_list_remove_all() [struct _GList *]
	(GList *) list [struct _GList *]
	(gconstpointer) data [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_remove_all(list: *mut _GList, data: *const libc::c_void) -> *mut _GList;
}


/*
void g_value_unset()
	(GValue *) value [struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_unset(value: *mut _GValue);
}


/*
GBytes * g_bytes_new_take() [struct _GBytes *]
	(gpointer) data [void *]
	(gsize) size [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bytes_new_take(data: *mut libc::c_void, size: libc::c_ulong) -> *mut _GBytes;
}


/*
void g_bookmark_file_set_icon()
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) href [const char *]
	(const gchar *) mime_type [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_set_icon(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, href: *const libc::c_char, mime_type: *const libc::c_char);
}


/*
GVariant * g_variant_new_take_string() [struct _GVariant *]
	(gchar *) string [char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_new_take_string(string: *mut libc::c_char) -> *mut _GVariant;
}


/*
void g_slice_free1()
	(gsize) block_size [unsigned long]
	(gpointer) mem_block [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slice_free1(block_size: libc::c_ulong, mem_block: *mut libc::c_void);
}


/*
GMutex * g_mutex_new() [union _GMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mutex_new() -> *mut _GMutex;
}


/*
void g_type_class_unref_uncached()
	(gpointer) g_class [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_class_unref_uncached(g_class: *mut libc::c_void);
}


/*
void g_log()
	(const gchar *) log_domain [const char *]
	(GLogLevelFlags) log_level [GLogLevelFlags]
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_log(log_domain: *const libc::c_char, log_level: libc::c_uint, format: *const libc::c_char);
}


/*
gpointer g_sequence_get() [void *]
	(GSequenceIter *) iter [struct _GSequenceNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_get(iter: *mut _GSequenceNode) -> *mut libc::c_void;
}


/*
GArray * g_array_remove_index_fast() [struct _GArray *]
	(GArray *) array [struct _GArray *]
	(guint) index_ [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_array_remove_index_fast(array: *mut _GArray, index_: libc::c_uint) -> *mut _GArray;
}


/*
GType g_error_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_error_get_type() -> libc::c_ulong;
}


/*
void g_cclosure_marshal_generic_va()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args_list [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	// pub fn g_cclosure_marshal_generic_va(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args_list: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
void g_object_notify_by_pspec()
	(GObject *) object [struct _GObject *]
	(GParamSpec *) pspec [struct _GParamSpec *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_notify_by_pspec(object: *mut _GObject, pspec: *mut _GParamSpec);
}


/*
GMainLoop * g_main_loop_new() [struct _GMainLoop *]
	(GMainContext *) context [struct _GMainContext *]
	(gboolean) is_running [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_loop_new(context: *mut _GMainContext, is_running: libc::c_int) -> *mut _GMainLoop;
}


/*
void g_hook_destroy_link()
	(GHookList *) hook_list [struct _GHookList *]
	(GHook *) hook [struct _GHook *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_destroy_link(hook_list: *mut _GHookList, hook: *mut _GHook);
}


/*
gboolean g_key_file_load_from_file() [int]
	(GKeyFile *) key_file [struct _GKeyFile *]
	(const gchar *) file [const char *]
	(GKeyFileFlags) flags [GKeyFileFlags]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_load_from_file(key_file: *mut _GKeyFile, file: *const libc::c_char, flags: libc::c_uint, error: *mut *mut _GError) -> libc::c_int;
}


/*
GTestLogMsg * g_test_log_buffer_pop() [GTestLogMsg *]
	(GTestLogBuffer *) tbuffer [GTestLogBuffer *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_log_buffer_pop(tbuffer: *mut GTestLogBuffer) -> *mut GTestLogMsg;
}


/*
GAsyncQueue * g_async_queue_ref() [struct _GAsyncQueue *]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_ref(queue: *mut _GAsyncQueue) -> *mut _GAsyncQueue;
}


/*
void g_object_get_property()
	(GObject *) object [struct _GObject *]
	(const gchar *) property_name [const char *]
	(GValue *) value [struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_get_property(object: *mut _GObject, property_name: *const libc::c_char, value: *mut _GValue);
}


/*
void g_main_loop_quit()
	(GMainLoop *) loop [struct _GMainLoop *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_loop_quit(loop_: *mut _GMainLoop);
}


/*
gint g_unichar_to_utf8() [int]
	(gunichar) c [unsigned int]
	(gchar *) outbuf [char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_to_utf8(c: libc::c_uint, outbuf: *mut libc::c_char) -> libc::c_int;
}


/*
gboolean g_signal_parse_name() [int]
	(const gchar *) detailed_signal [const char *]
	(GType) itype [unsigned long]
	(guint *) signal_id_p [unsigned int *]
	(GQuark *) detail_p [unsigned int *]
	(gboolean) force_detail_quark [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_parse_name(detailed_signal: *const libc::c_char, itype: libc::c_ulong, signal_id_p: *mut libc::c_uint, detail_p: *mut libc::c_uint, force_detail_quark: libc::c_int) -> libc::c_int;
}


/*
GType g_array_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_array_get_type() -> libc::c_ulong;
}


/*
gsize g_variant_iter_n_children() [unsigned long]
	(GVariantIter *) iter [struct _GVariantIter *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_iter_n_children(iter: *mut _GVariantIter) -> libc::c_ulong;
}


/*
gboolean g_value_type_compatible() [int]
	(GType) src_type [unsigned long]
	(GType) dest_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_type_compatible(src_type: libc::c_ulong, dest_type: libc::c_ulong) -> libc::c_int;
}


/*
gpointer * g_hash_table_get_keys_as_array() [void **]
	(GHashTable *) hash_table [struct _GHashTable *]
	(guint *) length [unsigned int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hash_table_get_keys_as_array(hash_table: *mut _GHashTable, length: *mut libc::c_uint) -> *mut *mut libc::c_void;
}


/*
guint32 g_unicode_script_to_iso15924() [unsigned int]
	(GUnicodeScript) script [GUnicodeScript]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unicode_script_to_iso15924(script: libc::c_uint) -> libc::c_uint;
}


/*
GMainContext * g_main_context_default() [struct _GMainContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_default() -> *mut _GMainContext;
}


/*
void g_string_chunk_free()
	(GStringChunk *) chunk [struct _GStringChunk *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_chunk_free(chunk: *mut _GStringChunk);
}


/*
gchar * g_get_current_dir() [char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_current_dir() -> *mut libc::c_char;
}


/*
gpointer g_datalist_id_get_data() [void *]
	(GData **) datalist [struct _GData **]
	(GQuark) key_id [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_datalist_id_get_data(datalist: *mut *mut _GData, key_id: libc::c_uint) -> *mut libc::c_void;
}


/*
GString * g_string_set_size() [struct _GString *]
	(GString *) string [struct _GString *]
	(gsize) len [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_set_size(string: *mut _GString, len: libc::c_ulong) -> *mut _GString;
}


/*
gboolean g_bookmark_file_remove_group() [int]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(const gchar *) group [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_remove_group(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, group: *const libc::c_char, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_date_set_julian()
	(GDate *) date [struct _GDate *]
	(guint32) julian_date [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_set_julian(date: *mut _GDate, julian_date: libc::c_uint);
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_set_app_info(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, name: *const libc::c_char, exec: *const libc::c_char, count: libc::c_int, stamp: libc::c_long, error: *mut *mut _GError) -> libc::c_int;
}


/*
gchar ** g_strsplit_set() [char **]
	(const gchar *) string [const char *]
	(const gchar *) delimiters [const char *]
	(gint) max_tokens [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strsplit_set(string: *const libc::c_char, delimiters: *const libc::c_char, max_tokens: libc::c_int) -> *mut *mut libc::c_char;
}


/*
GClosure * g_cclosure_new_swap() [struct _GClosure *]
	(GCallback) callback_func [void (*)(void)]
	(gpointer) user_data [void *]
	(GClosureNotify) destroy_data [void (*)(void *, struct _GClosure *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_new_swap(callback_func: Option<extern fn()>, user_data: *mut libc::c_void, destroy_data: Option<extern fn(*mut libc::c_void, *mut _GClosure)>) -> *mut _GClosure;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_key_file_load_from_dirs(key_file: *mut _GKeyFile, file: *const libc::c_char, search_dirs: *mut *const libc::c_char, full_path: *mut *mut libc::c_char, flags: libc::c_uint, error: *mut *mut _GError) -> libc::c_int;
}


/*
gboolean g_type_check_is_value_type() [int]
	(GType) type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_check_is_value_type(type_: libc::c_ulong) -> libc::c_int;
}


/*
gpointer g_malloc0() [void *]
	(gsize) n_bytes [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_malloc0(n_bytes: libc::c_ulong) -> *mut libc::c_void;
}


/*
gint32 g_variant_get_handle() [int]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_get_handle(value: *mut _GVariant) -> libc::c_int;
}


/*
GTimer * g_timer_new() [struct _GTimer *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_timer_new() -> *mut _GTimer;
}


/*
void g_atomic_int_inc()
	(volatile gint *) atomic [volatile int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_atomic_int_inc(atomic: *mut libc::c_int);
}


/*
gpointer g_scanner_lookup_symbol() [void *]
	(GScanner *) scanner [struct _GScanner *]
	(const gchar *) symbol [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_lookup_symbol(scanner: *mut _GScanner, symbol: *const libc::c_char) -> *mut libc::c_void;
}


/*
gboolean g_test_trap_has_passed() [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_trap_has_passed() -> libc::c_int;
}


/*
gunichar g_utf8_get_char_validated() [unsigned int]
	(const gchar *) p [const char *]
	(gssize) max_len [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf8_get_char_validated(p: *const libc::c_char, max_len: libc::c_long) -> libc::c_uint;
}


/*
gpointer g_value_get_object() [void *]
	(const GValue *) value [const struct _GValue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_get_object(value: *const _GValue) -> *mut libc::c_void;
}


/*
gboolean g_sequence_iter_is_end() [int]
	(GSequenceIter *) iter [struct _GSequenceNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_iter_is_end(iter: *mut _GSequenceNode) -> libc::c_int;
}


/*
void g_on_error_stack_trace()
	(const gchar *) prg_name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_on_error_stack_trace(prg_name: *const libc::c_char);
}


/*
gint g_unichar_xdigit_value() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_xdigit_value(c: libc::c_uint) -> libc::c_int;
}


/*
void g_string_append_vprintf()
	(GString *) string [struct _GString *]
	(const gchar *) format [const char *]
	(va_list) args [struct __va_list_tag [1]]
*/
#[link(name="gobject-2.0")]
extern "C" {
	// pub fn g_string_append_vprintf(string: *mut _GString, format: *const libc::c_char, args: [__va_list_tag; 1]);
}


/*
void g_test_suite_add_suite()
	(GTestSuite *) suite [struct GTestSuite *]
	(GTestSuite *) nestedsuite [struct GTestSuite *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_suite_add_suite(suite: *mut GTestSuite, nestedsuite: *mut GTestSuite);
}


/*
gpointer g_async_queue_timed_pop() [void *]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
	(GTimeVal *) end_time [struct _GTimeVal *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_timed_pop(queue: *mut _GAsyncQueue, end_time: *mut _GTimeVal) -> *mut libc::c_void;
}


/*
GTypePlugin * g_type_interface_get_plugin() [struct _GTypePlugin *]
	(GType) instance_type [unsigned long]
	(GType) interface_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_interface_get_plugin(instance_type: libc::c_ulong, interface_type: libc::c_ulong) -> *mut _GTypePlugin;
}


/*
GSequenceIter * g_sequence_get_iter_at_pos() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
	(gint) pos [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_get_iter_at_pos(seq: *mut _GSequence, pos: libc::c_int) -> *mut _GSequenceNode;
}


/*
GDateTime * g_date_time_add_years() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
	(gint) years [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_add_years(datetime: *mut _GDateTime, years: libc::c_int) -> *mut _GDateTime;
}


/*
void g_variant_builder_add_value()
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_builder_add_value(builder: *mut _GVariantBuilder, value: *mut _GVariant);
}


/*
gpointer g_object_steal_qdata() [void *]
	(GObject *) object [struct _GObject *]
	(GQuark) quark [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_steal_qdata(object: *mut _GObject, quark: libc::c_uint) -> *mut libc::c_void;
}


/*
gint g_atomic_int_exchange_and_add() [int]
	(volatile gint *) atomic [volatile int *]
	(gint) val [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_atomic_int_exchange_and_add(atomic: *mut libc::c_int, val: libc::c_int) -> libc::c_int;
}


/*
GHook * g_hook_next_valid() [struct _GHook *]
	(GHookList *) hook_list [struct _GHookList *]
	(GHook *) hook [struct _GHook *]
	(gboolean) may_be_in_call [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_next_valid(hook_list: *mut _GHookList, hook: *mut _GHook, may_be_in_call: libc::c_int) -> *mut _GHook;
}


/*
GObject * g_object_new_valist() [struct _GObject *]
	(GType) object_type [unsigned long]
	(const gchar *) first_property_name [const char *]
	(va_list) var_args [struct __va_list_tag [1]]
*/
#[link(name="gobject-2.0")]
extern "C" {
	// pub fn g_object_new_valist(object_type: libc::c_ulong, first_property_name: *const libc::c_char, var_args: [__va_list_tag; 1]) -> *mut _GObject;
}


/*
gchar * g_utf16_to_utf8() [char *]
	(const gunichar2 *) str [const unsigned short *]
	(glong) len [long]
	(glong *) items_read [long *]
	(glong *) items_written [long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_utf16_to_utf8(str: *const libc::c_ushort, len: libc::c_long, items_read: *mut libc::c_long, items_written: *mut libc::c_long, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
void g_option_context_set_ignore_unknown_options()
	(GOptionContext *) context [struct _GOptionContext *]
	(gboolean) ignore_unknown [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_context_set_ignore_unknown_options(context: *mut _GOptionContext, ignore_unknown: libc::c_int);
}


/*
void g_value_register_transform_func()
	(GType) src_type [unsigned long]
	(GType) dest_type [unsigned long]
	(GValueTransform) transform_func [void (*)(const struct _GValue *, struct _GValue *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_register_transform_func(src_type: libc::c_ulong, dest_type: libc::c_ulong, transform_func: Option<extern fn(*const _GValue, *mut _GValue)>);
}


/*
gpointer g_object_ref() [void *]
	(gpointer) object [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_ref(object: *mut libc::c_void) -> *mut libc::c_void;
}


/*
GChecksum * g_checksum_copy() [struct _GChecksum *]
	(const GChecksum *) checksum [const struct _GChecksum *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_checksum_copy(checksum: *const _GChecksum) -> *mut _GChecksum;
}


/*
void g_test_log_set_fatal_handler()
	(GTestLogFatalFunc) log_func [int (*)(const char *, GLogLevelFlags, const char *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_log_set_fatal_handler(log_func: Option<extern fn(*const libc::c_char, libc::c_uint, *const libc::c_char, *mut libc::c_void) -> libc::c_int>, user_data: *mut libc::c_void);
}


/*
GIOStatus g_io_channel_write_chars() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(const gchar *) buf [const char *]
	(gssize) count [long]
	(gsize *) bytes_written [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_write_chars(channel: *mut _GIOChannel, buf: *const libc::c_char, count: libc::c_long, bytes_written: *mut libc::c_ulong, error: *mut *mut _GError) -> libc::c_uint;
}


/*
void g_rw_lock_init()
	(GRWLock *) rw_lock [struct _GRWLock *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_rw_lock_init(rw_lock: *mut _GRWLock);
}


/*
const gchar * g_io_channel_get_line_term() [const char *]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gint *) length [int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_get_line_term(channel: *mut _GIOChannel, length: *mut libc::c_int) -> *const libc::c_char;
}


/*
void g_cclosure_marshal_VOID__BOOLEAN()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_VOID__BOOLEAN(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
const gchar * g_param_spec_get_name() [const char *]
	(GParamSpec *) pspec [struct _GParamSpec *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_get_name(pspec: *mut _GParamSpec) -> *const libc::c_char;
}


/*
void g_cclosure_marshal_VOID__VOID()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(guint) n_param_values [unsigned int]
	(const GValue *) param_values [const struct _GValue *]
	(gpointer) invocation_hint [void *]
	(gpointer) marshal_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cclosure_marshal_VOID__VOID(closure: *mut _GClosure, return_value: *mut _GValue, n_param_values: libc::c_uint, param_values: *const _GValue, invocation_hint: *mut libc::c_void, marshal_data: *mut libc::c_void);
}


/*
const gchar * g_basename() [const char *]
	(const gchar *) file_name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_basename(file_name: *const libc::c_char) -> *const libc::c_char;
}


/*
void g_value_set_static_boxed()
	(GValue *) value [struct _GValue *]
	(gconstpointer) v_boxed [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_value_set_static_boxed(value: *mut _GValue, v_boxed: *const libc::c_void);
}


/*
gchar * g_locale_from_utf8() [char *]
	(const gchar *) utf8string [const char *]
	(gssize) len [long]
	(gsize *) bytes_read [unsigned long *]
	(gsize *) bytes_written [unsigned long *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_locale_from_utf8(utf8string: *const libc::c_char, len: libc::c_long, bytes_read: *mut libc::c_ulong, bytes_written: *mut libc::c_ulong, error: *mut *mut _GError) -> *mut libc::c_char;
}


/*
gpointer g_async_queue_try_pop_unlocked() [void *]
	(GAsyncQueue *) queue [struct _GAsyncQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_async_queue_try_pop_unlocked(queue: *mut _GAsyncQueue) -> *mut libc::c_void;
}


/*
GByteArray * g_bytes_unref_to_array() [struct _GByteArray *]
	(GBytes *) bytes [struct _GBytes *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bytes_unref_to_array(bytes: *mut _GBytes) -> *mut _GByteArray;
}


/*
void g_object_force_floating()
	(GObject *) object [struct _GObject *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_force_floating(object: *mut _GObject);
}


/*
GPrintFunc g_set_printerr_handler() [void (*)(const char *)]
	(GPrintFunc) func [void (*)(const char *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_set_printerr_handler(func: Option<extern fn(*const libc::c_char)>) -> Option<extern fn(*const libc::c_char)>;
}


/*
GType g_main_context_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_get_type() -> libc::c_ulong;
}


/*
void g_node_reverse_children()
	(GNode *) node [struct _GNode *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_reverse_children(node: *mut _GNode);
}


/*
GError * g_error_new() [struct _GError *]
	(GQuark) domain [unsigned int]
	(gint) code [int]
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_error_new(domain: libc::c_uint, code: libc::c_int, format: *const libc::c_char) -> *mut _GError;
}


/*
gint g_poll() [int]
	(GPollFD *) fds [struct _GPollFD *]
	(guint) nfds [unsigned int]
	(gint) timeout [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_poll(fds: *mut _GPollFD, nfds: libc::c_uint, timeout: libc::c_int) -> libc::c_int;
}


/*
GRelation * g_relation_new() [struct _GRelation *]
	(gint) fields [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_relation_new(fields: libc::c_int) -> *mut _GRelation;
}


/*
void g_signal_override_class_handler()
	(const gchar *) signal_name [const char *]
	(GType) instance_type [unsigned long]
	(GCallback) class_handler [void (*)(void)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_override_class_handler(signal_name: *const libc::c_char, instance_type: libc::c_ulong, class_handler: Option<extern fn()>);
}


/*
GVariant * g_variant_lookup_value() [struct _GVariant *]
	(GVariant *) dictionary [struct _GVariant *]
	(const gchar *) key [const char *]
	(const GVariantType *) expected_type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_lookup_value(dictionary: *mut _GVariant, key: *const libc::c_char, expected_type: *const _GVariantType) -> *mut _GVariant;
}


/*
void g_atomic_pointer_set()
	(volatile void *) atomic
	(gpointer) newval [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_atomic_pointer_set(atomic: *mut libc::c_void, newval: *mut libc::c_void);
}


/*
void g_sequence_sort_changed()
	(GSequenceIter *) iter [struct _GSequenceNode *]
	(GCompareDataFunc) cmp_func [int (*)(const void *, const void *, void *)]
	(gpointer) cmp_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_sort_changed(iter: *mut _GSequenceNode, cmp_func: Option<extern fn(*const libc::c_void, *const libc::c_void, *mut libc::c_void) -> libc::c_int>, cmp_data: *mut libc::c_void);
}


/*
GString * g_string_ascii_up() [struct _GString *]
	(GString *) string [struct _GString *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_string_ascii_up(string: *mut _GString) -> *mut _GString;
}


/*
GSList * g_slist_last() [struct _GSList *]
	(GSList *) list [struct _GSList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_last(list: *mut _GSList) -> *mut _GSList;
}


/*
gchar * g_format_size() [char *]
	(guint64) size [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_format_size(size: libc::c_ulong) -> *mut libc::c_char;
}


/*
gint g_thread_pool_get_max_threads() [int]
	(GThreadPool *) pool [struct _GThreadPool *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_pool_get_max_threads(pool: *mut _GThreadPool) -> libc::c_int;
}


/*
GVariantBuilder * g_variant_builder_new() [struct _GVariantBuilder *]
	(const GVariantType *) type [const struct _GVariantType *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_builder_new(type_: *const _GVariantType) -> *mut _GVariantBuilder;
}


/*
gboolean g_test_trap_reached_timeout() [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_trap_reached_timeout() -> libc::c_int;
}


/*
GType g_gtype_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_gtype_get_type() -> libc::c_ulong;
}


/*
void g_cache_destroy()
	(GCache *) cache [struct _GCache *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_cache_destroy(cache: *mut _GCache);
}


/*
GParamSpec ** g_object_interface_list_properties() [struct _GParamSpec **]
	(gpointer) g_iface [void *]
	(guint *) n_properties_p [unsigned int *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_interface_list_properties(g_iface: *mut libc::c_void, n_properties_p: *mut libc::c_uint) -> *mut *mut _GParamSpec;
}


/*
guint g_timeout_add_full() [unsigned int]
	(gint) priority [int]
	(guint) interval [unsigned int]
	(GSourceFunc) function [int (*)(void *)]
	(gpointer) data [void *]
	(GDestroyNotify) notify [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_timeout_add_full(priority: libc::c_int, interval: libc::c_uint, function: Option<extern fn(*mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void, notify: Option<extern fn(*mut libc::c_void)>) -> libc::c_uint;
}


/*
gboolean g_bit_trylock() [int]
	(volatile gint *) address [volatile int *]
	(gint) lock_bit [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bit_trylock(address: *mut libc::c_int, lock_bit: libc::c_int) -> libc::c_int;
}


/*
gboolean g_unichar_iszerowidth() [int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_iszerowidth(c: libc::c_uint) -> libc::c_int;
}


/*
guint g_timeout_add() [unsigned int]
	(guint) interval [unsigned int]
	(GSourceFunc) function [int (*)(void *)]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_timeout_add(interval: libc::c_uint, function: Option<extern fn(*mut libc::c_void) -> libc::c_int>, data: *mut libc::c_void) -> libc::c_uint;
}


/*
gint g_list_position() [int]
	(GList *) list [struct _GList *]
	(GList *) llink [struct _GList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_list_position(list: *mut _GList, llink: *mut _GList) -> libc::c_int;
}


/*
GDateTime * g_date_time_add() [struct _GDateTime *]
	(GDateTime *) datetime [struct _GDateTime *]
	(GTimeSpan) timespan [long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_add(datetime: *mut _GDateTime, timespan: libc::c_long) -> *mut _GDateTime;
}


/*
gint g_date_time_get_day_of_week() [int]
	(GDateTime *) datetime [struct _GDateTime *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_get_day_of_week(datetime: *mut _GDateTime) -> libc::c_int;
}


/*
GPtrArray * g_ptr_array_new() [struct _GPtrArray *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_new() -> *mut _GPtrArray;
}


/*
gchar * g_strreverse() [char *]
	(gchar *) string [char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_strreverse(string: *mut libc::c_char) -> *mut libc::c_char;
}


/*
void g_tree_replace()
	(GTree *) tree [struct _GTree *]
	(gpointer) key [void *]
	(gpointer) value [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_tree_replace(tree: *mut _GTree, key: *mut libc::c_void, value: *mut libc::c_void);
}


/*
GNode * g_node_find_child() [struct _GNode *]
	(GNode *) node [struct _GNode *]
	(GTraverseFlags) flags [GTraverseFlags]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_node_find_child(node: *mut _GNode, flags: libc::c_uint, data: *mut libc::c_void) -> *mut _GNode;
}


/*
void g_closure_remove_invalidate_notifier()
	(GClosure *) closure [struct _GClosure *]
	(gpointer) notify_data [void *]
	(GClosureNotify) notify_func [void (*)(void *, struct _GClosure *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_closure_remove_invalidate_notifier(closure: *mut _GClosure, notify_data: *mut libc::c_void, notify_func: Option<extern fn(*mut libc::c_void, *mut _GClosure)>);
}


/*
GQuark g_thread_error_quark() [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_thread_error_quark() -> libc::c_uint;
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_spawn_async(working_directory: *const libc::c_char, argv: *mut *mut libc::c_char, envp: *mut *mut libc::c_char, flags: libc::c_uint, child_setup: Option<extern fn(*mut libc::c_void)>, user_data: *mut libc::c_void, child_pid: *mut libc::c_int, error: *mut *mut _GError) -> libc::c_int;
}


/*
void g_signal_override_class_closure()
	(guint) signal_id [unsigned int]
	(GType) instance_type [unsigned long]
	(GClosure *) class_closure [struct _GClosure *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_override_class_closure(signal_id: libc::c_uint, instance_type: libc::c_ulong, class_closure: *mut _GClosure);
}


/*
GSList * g_slist_concat() [struct _GSList *]
	(GSList *) list1 [struct _GSList *]
	(GSList *) list2 [struct _GSList *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slist_concat(list1: *mut _GSList, list2: *mut _GSList) -> *mut _GSList;
}


/*
guint64 g_ascii_strtoull() [unsigned long]
	(const gchar *) nptr [const char *]
	(gchar **) endptr [char **]
	(guint) base [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ascii_strtoull(nptr: *const libc::c_char, endptr: *mut *mut libc::c_char, base: libc::c_uint) -> libc::c_ulong;
}


/*
void g_type_add_interface_dynamic()
	(GType) instance_type [unsigned long]
	(GType) interface_type [unsigned long]
	(GTypePlugin *) plugin [struct _GTypePlugin *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_add_interface_dynamic(instance_type: libc::c_ulong, interface_type: libc::c_ulong, plugin: *mut _GTypePlugin);
}


/*
GTree * g_tree_ref() [struct _GTree *]
	(GTree *) tree [struct _GTree *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_tree_ref(tree: *mut _GTree) -> *mut _GTree;
}


/*
gint g_regex_get_max_lookbehind() [int]
	(const GRegex *) regex [const struct _GRegex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_get_max_lookbehind(regex: *const _GRegex) -> libc::c_int;
}


/*
void g_cclosure_marshal_VOID__UINT_POINTERv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	// pub fn g_cclosure_marshal_VOID__UINT_POINTERv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
GSource * g_timeout_source_new() [struct _GSource *]
	(guint) interval [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_timeout_source_new(interval: libc::c_uint) -> *mut _GSource;
}


/*
GPollFunc g_main_context_get_poll_func() [int (*)(struct _GPollFD *, unsigned int, int)]
	(GMainContext *) context [struct _GMainContext *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_main_context_get_poll_func(context: *mut _GMainContext) -> Option<extern fn(*mut _GPollFD, libc::c_uint, libc::c_int) -> libc::c_int>;
}


/*
GArray * g_array_set_size() [struct _GArray *]
	(GArray *) array [struct _GArray *]
	(guint) length [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_array_set_size(array: *mut _GArray, length: libc::c_uint) -> *mut _GArray;
}


/*
void g_scanner_error()
	(GScanner *) scanner [struct _GScanner *]
	(const gchar *) format [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_error(scanner: *mut _GScanner, format: *const libc::c_char);
}


/*
gunichar g_unichar_toupper() [unsigned int]
	(gunichar) c [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_unichar_toupper(c: libc::c_uint) -> libc::c_uint;
}


/*
GByteArray * g_byte_array_prepend() [struct _GByteArray *]
	(GByteArray *) array [struct _GByteArray *]
	(const guint8 *) data [const unsigned char *]
	(guint) len [unsigned int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_byte_array_prepend(array: *mut _GByteArray, data: *const libc::c_uchar, len: libc::c_uint) -> *mut _GByteArray;
}


/*
void g_cclosure_marshal_VOID__LONGv()
	(GClosure *) closure [struct _GClosure *]
	(GValue *) return_value [struct _GValue *]
	(gpointer) instance [void *]
	(va_list) args [struct __va_list_tag [1]]
	(gpointer) marshal_data [void *]
	(int) n_params
	(GType *) param_types [unsigned long *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	// pub fn g_cclosure_marshal_VOID__LONGv(closure: *mut _GClosure, return_value: *mut _GValue, instance: *mut libc::c_void, args: [__va_list_tag; 1], marshal_data: *mut libc::c_void, n_params: libc::c_int, param_types: *mut libc::c_ulong);
}


/*
gpointer g_param_spec_internal() [void *]
	(GType) param_type [unsigned long]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_internal(param_type: libc::c_ulong, name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, flags: libc::c_uint) -> *mut libc::c_void;
}


/*
void g_io_channel_set_line_term()
	(GIOChannel *) channel [struct _GIOChannel *]
	(const gchar *) line_term [const char *]
	(gint) length [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_set_line_term(channel: *mut _GIOChannel, line_term: *const libc::c_char, length: libc::c_int);
}


/*
void g_boxed_free()
	(GType) boxed_type [unsigned long]
	(gpointer) boxed [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_boxed_free(boxed_type: libc::c_ulong, boxed: *mut libc::c_void);
}


/*
GSequenceIter * g_sequence_get_begin_iter() [struct _GSequenceNode *]
	(GSequence *) seq [struct _GSequence *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_get_begin_iter(seq: *mut _GSequence) -> *mut _GSequenceNode;
}


/*
GType g_type_register_dynamic() [unsigned long]
	(GType) parent_type [unsigned long]
	(const gchar *) type_name [const char *]
	(GTypePlugin *) plugin [struct _GTypePlugin *]
	(GTypeFlags) flags [GTypeFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_register_dynamic(parent_type: libc::c_ulong, type_name: *const libc::c_char, plugin: *mut _GTypePlugin, flags: libc::c_uint) -> libc::c_ulong;
}


/*
void g_weak_ref_set()
	(GWeakRef *) weak_ref [GWeakRef *]
	(gpointer) object [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_weak_ref_set(weak_ref: *mut GWeakRef, object: *mut libc::c_void);
}


/*
void g_hook_list_invoke()
	(GHookList *) hook_list [struct _GHookList *]
	(gboolean) may_recurse [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_hook_list_invoke(hook_list: *mut _GHookList, may_recurse: libc::c_int);
}


/*
gpointer g_datalist_id_dup_data() [void *]
	(GData **) datalist [struct _GData **]
	(GQuark) key_id [unsigned int]
	(GDuplicateFunc) dup_func [void *(*)(void *, void *)]
	(gpointer) user_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_datalist_id_dup_data(datalist: *mut *mut _GData, key_id: libc::c_uint, dup_func: Option<extern fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void>, user_data: *mut libc::c_void) -> *mut libc::c_void;
}


/*
void g_option_context_set_translate_func()
	(GOptionContext *) context [struct _GOptionContext *]
	(GTranslateFunc) func [const char *(*)(const char *, void *)]
	(gpointer) data [void *]
	(GDestroyNotify) destroy_notify [void (*)(void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_option_context_set_translate_func(context: *mut _GOptionContext, func: Option<extern fn(*const libc::c_char, *mut libc::c_void) -> *const libc::c_char>, data: *mut libc::c_void, destroy_notify: Option<extern fn(*mut libc::c_void)>);
}


/*
GIOCondition g_io_channel_get_buffer_condition() [GIOCondition]
	(GIOChannel *) channel [struct _GIOChannel *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_get_buffer_condition(channel: *mut _GIOChannel) -> libc::c_uint;
}


/*
gboolean g_variant_type_equal() [int]
	(gconstpointer) type1 [const void *]
	(gconstpointer) type2 [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_type_equal(type1: *const libc::c_void, type2: *const libc::c_void) -> libc::c_int;
}


/*
GSequenceIter * g_sequence_iter_move() [struct _GSequenceNode *]
	(GSequenceIter *) iter [struct _GSequenceNode *]
	(gint) delta [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_sequence_iter_move(iter: *mut _GSequenceNode, delta: libc::c_int) -> *mut _GSequenceNode;
}


/*
gboolean g_get_charset() [int]
	(const char **) charset
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_charset(charset: *mut *const libc::c_char) -> libc::c_int;
}


/*
void g_object_thaw_notify()
	(GObject *) object [struct _GObject *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_object_thaw_notify(object: *mut _GObject);
}


/*
guint g_date_time_hash() [unsigned int]
	(gconstpointer) datetime [const void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_hash(datetime: *const libc::c_void) -> libc::c_uint;
}


/*
gpointer g_slice_alloc() [void *]
	(gsize) block_size [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_slice_alloc(block_size: libc::c_ulong) -> *mut libc::c_void;
}


/*
gboolean g_queue_is_empty() [int]
	(GQueue *) queue [struct _GQueue *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_queue_is_empty(queue: *mut _GQueue) -> libc::c_int;
}


/*
GType g_regex_get_type() [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_get_type() -> libc::c_ulong;
}


/*
GTokenType g_scanner_get_next_token() [GTokenType]
	(GScanner *) scanner [struct _GScanner *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_get_next_token(scanner: *mut _GScanner) -> libc::c_uint;
}


/*
void g_mutex_init()
	(GMutex *) mutex [union _GMutex *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mutex_init(mutex: *mut _GMutex);
}


/*
void g_test_queue_destroy()
	(GDestroyNotify) destroy_func [void (*)(void *)]
	(gpointer) destroy_data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_queue_destroy(destroy_func: Option<extern fn(*mut libc::c_void)>, destroy_data: *mut libc::c_void);
}


/*
void g_scanner_destroy()
	(GScanner *) scanner [struct _GScanner *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_scanner_destroy(scanner: *mut _GScanner);
}


/*
gboolean g_date_time_to_timeval() [int]
	(GDateTime *) datetime [struct _GDateTime *]
	(GTimeVal *) tv [struct _GTimeVal *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_date_time_to_timeval(datetime: *mut _GDateTime, tv: *mut _GTimeVal) -> libc::c_int;
}


/*
gint g_regex_get_string_number() [int]
	(const GRegex *) regex [const struct _GRegex *]
	(const gchar *) name [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_get_string_number(regex: *const _GRegex, name: *const libc::c_char) -> libc::c_int;
}


/*
gboolean g_spawn_command_line_sync() [int]
	(const gchar *) command_line [const char *]
	(gchar **) standard_output [char **]
	(gchar **) standard_error [char **]
	(gint *) exit_status [int *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_spawn_command_line_sync(command_line: *const libc::c_char, standard_output: *mut *mut libc::c_char, standard_error: *mut *mut libc::c_char, exit_status: *mut libc::c_int, error: *mut *mut _GError) -> libc::c_int;
}


/*
gsize g_variant_n_children() [unsigned long]
	(GVariant *) value [struct _GVariant *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_n_children(value: *mut _GVariant) -> libc::c_ulong;
}


/*
void g_source_set_funcs()
	(GSource *) source [struct _GSource *]
	(GSourceFuncs *) funcs [struct _GSourceFuncs *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_set_funcs(source: *mut _GSource, funcs: *mut _GSourceFuncs);
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
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_markup_collect_attributes(element_name: *const libc::c_char, attribute_names: *mut *const libc::c_char, attribute_values: *mut *const libc::c_char, error: *mut *mut _GError, first_type: libc::c_uint, first_attr: *const libc::c_char) -> libc::c_int;
}


/*
gboolean g_source_get_can_recurse() [int]
	(GSource *) source [struct _GSource *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_source_get_can_recurse(source: *mut _GSource) -> libc::c_int;
}


/*
void g_variant_builder_clear()
	(GVariantBuilder *) builder [struct _GVariantBuilder *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_variant_builder_clear(builder: *mut _GVariantBuilder);
}


/*
GTree * g_tree_new() [struct _GTree *]
	(GCompareFunc) key_compare_func [int (*)(const void *, const void *)]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_tree_new(key_compare_func: Option<extern fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>) -> *mut _GTree;
}


/*
GParamSpec * g_param_spec_object() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(GType) object_type [unsigned long]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_object(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, object_type: libc::c_ulong, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
gpointer g_type_instance_get_private() [void *]
	(GTypeInstance *) instance [struct _GTypeInstance *]
	(GType) private_type [unsigned long]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_type_instance_get_private(instance: *mut _GTypeInstance, private_type: libc::c_ulong) -> *mut libc::c_void;
}


/*
int g_test_run()
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_run() -> libc::c_int;
}


/*
gsize g_mapped_file_get_length() [unsigned long]
	(GMappedFile *) file [struct _GMappedFile *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_mapped_file_get_length(file: *mut _GMappedFile) -> libc::c_ulong;
}


/*
const gchar * g_get_user_runtime_dir() [const char *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_get_user_runtime_dir() -> *const libc::c_char;
}


/*
GParamSpec * g_param_spec_string() [struct _GParamSpec *]
	(const gchar *) name [const char *]
	(const gchar *) nick [const char *]
	(const gchar *) blurb [const char *]
	(const gchar *) default_value [const char *]
	(GParamFlags) flags [GParamFlags]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_param_spec_string(name: *const libc::c_char, nick: *const libc::c_char, blurb: *const libc::c_char, default_value: *const libc::c_char, flags: libc::c_uint) -> *mut _GParamSpec;
}


/*
time_t g_bookmark_file_get_visited() [long]
	(GBookmarkFile *) bookmark [struct _GBookmarkFile *]
	(const gchar *) uri [const char *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_bookmark_file_get_visited(bookmark: *mut _GBookmarkFile, uri: *const libc::c_char, error: *mut *mut _GError) -> libc::c_long;
}


/*
double g_test_rand_double_range()
	(double) range_start
	(double) range_end
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_test_rand_double_range(range_start: libc::c_double, range_end: libc::c_double) -> libc::c_double;
}


/*
GIOStatus g_io_channel_read_unichar() [GIOStatus]
	(GIOChannel *) channel [struct _GIOChannel *]
	(gunichar *) thechar [unsigned int *]
	(GError **) error [struct _GError **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_io_channel_read_unichar(channel: *mut _GIOChannel, thechar: *mut libc::c_uint, error: *mut *mut _GError) -> libc::c_uint;
}


/*
gchar * g_time_val_to_iso8601() [char *]
	(GTimeVal *) time_ [struct _GTimeVal *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_time_val_to_iso8601(time_: *mut _GTimeVal) -> *mut libc::c_char;
}


/*
gboolean g_regex_match_all() [int]
	(const GRegex *) regex [const struct _GRegex *]
	(const gchar *) string [const char *]
	(GRegexMatchFlags) match_options [GRegexMatchFlags]
	(GMatchInfo **) match_info [struct _GMatchInfo **]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_regex_match_all(regex: *const _GRegex, string: *const libc::c_char, match_options: libc::c_uint, match_info: *mut *mut _GMatchInfo) -> libc::c_int;
}


/*
gpointer g_weak_ref_get() [void *]
	(GWeakRef *) weak_ref [GWeakRef *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_weak_ref_get(weak_ref: *mut GWeakRef) -> *mut libc::c_void;
}


/*
guint g_signal_handlers_block_matched() [unsigned int]
	(gpointer) instance [void *]
	(GSignalMatchType) mask [GSignalMatchType]
	(guint) signal_id [unsigned int]
	(GQuark) detail [unsigned int]
	(GClosure *) closure [struct _GClosure *]
	(gpointer) func [void *]
	(gpointer) data [void *]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_signal_handlers_block_matched(instance: *mut libc::c_void, mask: libc::c_uint, signal_id: libc::c_uint, detail: libc::c_uint, closure: *mut _GClosure, func: *mut libc::c_void, data: *mut libc::c_void) -> libc::c_uint;
}


/*
void g_ptr_array_set_size()
	(GPtrArray *) array [struct _GPtrArray *]
	(gint) length [int]
*/
#[link(name="gobject-2.0")]
extern "C" {
	pub fn g_ptr_array_set_size(array: *mut _GPtrArray, length: libc::c_int);
}


/* __GLIB_GOBJECT_H__ # */

/* __GLIB_GOBJECT_H_INSIDE__ /* topmost include file for GObject header files */ */

/* __G_BINDING_H__ # */

/* __G_LIB_H__ # */

/* __GLIB_H_INSIDE__ # */

/* __G_ALLOCA_H__ # */

/* __G_TYPES_H__ # */

/* __G_MACROS_H__ # */

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

/* G_GNUC_BEGIN_IGNORE_DEPRECATIONS _Pragma ( "clang diagnostic push" ) _Pragma ( "clang diagnostic ignored \"-Wdeprecated-declarations\"" ) # */

/* G_GNUC_END_IGNORE_DEPRECATIONS _Pragma ( "clang diagnostic pop" ) # */

/* G_GNUC_MAY_ALIAS __attribute__ ( ( may_alias ) ) # */

/* G_GNUC_WARN_UNUSED_RESULT __attribute__ ( ( warn_unused_result ) ) # */

/* G_GNUC_FUNCTION "" # */

/* G_GNUC_PRETTY_FUNCTION "" # */

/* G_ANALYZER_ANALYZING 1 # */
pub const G_ANALYZER_ANALYZING: i32 = 1;

/* G_ANALYZER_NORETURN __attribute__ ( ( analyzer_noreturn ) ) # */

/* G_STRINGIFY ( macro_or_string ) G_STRINGIFY_ARG ( macro_or_string ) # */

/* G_STRINGIFY_ARG ( contents ) # contents # */

/* G_PASTE_ARGS ( identifier1 , identifier2 ) identifier1 ## identifier2 # */

/* G_PASTE ( identifier1 , identifier2 ) G_PASTE_ARGS ( identifier1 , identifier2 ) # */

/* G_STATIC_ASSERT ( expr ) typedef char G_PASTE ( _GStaticAssertCompileTimeAssertion_ , __COUNTER__ ) [ ( expr ) ? 1 : - 1 ] G_GNUC_UNUSED # */

/* G_STATIC_ASSERT_EXPR ( expr ) ( ( void ) sizeof ( char [ ( expr ) ? 1 : - 1 ] ) ) # */

/* G_STRLOC __FILE__ ":" G_STRINGIFY ( __LINE__ ) # */

/* G_STRFUNC ( ( const char * ) ( __func__ ) ) # */

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

/* G_UNAVAILABLE ( maj , min ) G_DEPRECATED # */

/* _GLIB_EXTERN extern # */

/* GLIB_DEPRECATED G_DEPRECATED _GLIB_EXTERN # */

/* GLIB_DEPRECATED_FOR ( f ) G_DEPRECATED_FOR ( f ) _GLIB_EXTERN # */

/* GLIB_UNAVAILABLE ( maj , min ) G_UNAVAILABLE ( maj , min ) _GLIB_EXTERN # */

/* __G_VERSION_MACROS_H__ # */

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

/* GLIB_VERSION_2_32 ( G_ENCODE_VERSION ( 2 , 32 ) ) /**
 * GLIB_VERSION_2_34:
 *
 * A macro that evaluates to the 2.34 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.34
 */ */

/* GLIB_VERSION_2_34 ( G_ENCODE_VERSION ( 2 , 34 ) ) /**
 * GLIB_VERSION_2_36:
 *
 * A macro that evaluates to the 2.36 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.36
 */ */

/* GLIB_VERSION_2_36 ( G_ENCODE_VERSION ( 2 , 36 ) ) /**
 * GLIB_VERSION_2_38:
 *
 * A macro that evaluates to the 2.38 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.38
 */ */

/* GLIB_VERSION_2_38 ( G_ENCODE_VERSION ( 2 , 38 ) ) /**
 * GLIB_VERSION_2_40:
 *
 * A macro that evaluates to the 2.40 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.40
 */ */

/* GLIB_VERSION_2_40 ( G_ENCODE_VERSION ( 2 , 40 ) ) /**
 * GLIB_VERSION_2_42:
 *
 * A macro that evaluates to the 2.42 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.42
 */ */

/* GLIB_VERSION_2_42 ( G_ENCODE_VERSION ( 2 , 42 ) ) /* evaluates to the current stable version; for development cycles,
 * this means the next stable target
 */ */

/* GLIB_VERSION_CUR_STABLE ( G_ENCODE_VERSION ( GLIB_MAJOR_VERSION , GLIB_MINOR_VERSION ) ) # */

/* GLIB_VERSION_PREV_STABLE ( G_ENCODE_VERSION ( GLIB_MAJOR_VERSION , GLIB_MINOR_VERSION - 2 ) ) # */

/* GLIB_VERSION_MIN_REQUIRED ( GLIB_VERSION_CUR_STABLE ) # */

/* GLIB_VERSION_MAX_ALLOWED ( GLIB_VERSION_CUR_STABLE ) # */

/* GLIB_AVAILABLE_IN_ALL _GLIB_EXTERN /* XXX: Every new stable minor release should add a set of macros here */ */

/* GLIB_DEPRECATED_IN_2_26 GLIB_DEPRECATED # */

/* GLIB_DEPRECATED_IN_2_26_FOR ( f ) GLIB_DEPRECATED_FOR ( f ) # */

/* GLIB_AVAILABLE_IN_2_26 _GLIB_EXTERN # */

/* GLIB_DEPRECATED_IN_2_28 GLIB_DEPRECATED # */

/* GLIB_DEPRECATED_IN_2_28_FOR ( f ) GLIB_DEPRECATED_FOR ( f ) # */

/* GLIB_AVAILABLE_IN_2_28 _GLIB_EXTERN # */

/* GLIB_DEPRECATED_IN_2_30 GLIB_DEPRECATED # */

/* GLIB_DEPRECATED_IN_2_30_FOR ( f ) GLIB_DEPRECATED_FOR ( f ) # */

/* GLIB_AVAILABLE_IN_2_30 _GLIB_EXTERN # */

/* GLIB_DEPRECATED_IN_2_32 GLIB_DEPRECATED # */

/* GLIB_DEPRECATED_IN_2_32_FOR ( f ) GLIB_DEPRECATED_FOR ( f ) # */

/* GLIB_AVAILABLE_IN_2_32 _GLIB_EXTERN # */

/* GLIB_DEPRECATED_IN_2_34 GLIB_DEPRECATED # */

/* GLIB_DEPRECATED_IN_2_34_FOR ( f ) GLIB_DEPRECATED_FOR ( f ) # */

/* GLIB_AVAILABLE_IN_2_34 _GLIB_EXTERN # */

/* GLIB_DEPRECATED_IN_2_36 GLIB_DEPRECATED # */

/* GLIB_DEPRECATED_IN_2_36_FOR ( f ) GLIB_DEPRECATED_FOR ( f ) # */

/* GLIB_AVAILABLE_IN_2_36 _GLIB_EXTERN # */

/* GLIB_DEPRECATED_IN_2_38 GLIB_DEPRECATED # */

/* GLIB_DEPRECATED_IN_2_38_FOR ( f ) GLIB_DEPRECATED_FOR ( f ) # */

/* GLIB_AVAILABLE_IN_2_38 _GLIB_EXTERN # */

/* GLIB_DEPRECATED_IN_2_40 GLIB_DEPRECATED # */

/* GLIB_DEPRECATED_IN_2_40_FOR ( f ) GLIB_DEPRECATED_FOR ( f ) # */

/* GLIB_AVAILABLE_IN_2_40 _GLIB_EXTERN # */

/* GLIB_DEPRECATED_IN_2_42 GLIB_DEPRECATED # */

/* GLIB_DEPRECATED_IN_2_42_FOR ( f ) GLIB_DEPRECATED_FOR ( f ) # */

/* GLIB_AVAILABLE_IN_2_42 _GLIB_EXTERN # */

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

/* GLIB_VAR _GLIB_EXTERN # */

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

/* g_array_index ( a , t , i ) ( ( ( t * ) ( void * ) ( a ) -> data ) [ ( i ) ] ) GLIB_AVAILABLE_IN_ALL */

/* g_ptr_array_index ( array , index_ ) ( ( array ) -> pdata ) [ index_ ] GLIB_AVAILABLE_IN_ALL */

/* __G_ASYNCQUEUE_H__ # */

/* __G_THREAD_H__ # */

/* __G_ATOMIC_H__ # */

/* g_atomic_int_get ( atomic ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gint ) ) ; ( void ) ( 0 ? * ( atomic ) ^ * ( atomic ) : 0 ) ; __sync_synchronize ( ) ; ( gint ) * ( atomic ) ; } ) ) # */

/* g_atomic_int_set ( atomic , newval ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gint ) ) ; ( void ) ( 0 ? * ( atomic ) ^ ( newval ) : 0 ) ; * ( atomic ) = ( newval ) ; __sync_synchronize ( ) ; } ) ) # */

/* g_atomic_pointer_get ( atomic ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gpointer ) ) ; __sync_synchronize ( ) ; ( gpointer ) * ( atomic ) ; } ) ) # */

/* g_atomic_pointer_set ( atomic , newval ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gpointer ) ) ; ( void ) ( 0 ? ( gpointer ) * ( atomic ) : 0 ) ; * ( atomic ) = ( __typeof__ ( * ( atomic ) ) ) ( gsize ) ( newval ) ; __sync_synchronize ( ) ; } ) ) # */

/* g_atomic_int_inc ( atomic ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gint ) ) ; ( void ) ( 0 ? * ( atomic ) ^ * ( atomic ) : 0 ) ; ( void ) __sync_fetch_and_add ( ( atomic ) , 1 ) ; } ) ) # */

/* g_atomic_int_dec_and_test ( atomic ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gint ) ) ; ( void ) ( 0 ? * ( atomic ) ^ * ( atomic ) : 0 ) ; __sync_fetch_and_sub ( ( atomic ) , 1 ) == 1 ; } ) ) # */

/* g_atomic_int_compare_and_exchange ( atomic , oldval , newval ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gint ) ) ; ( void ) ( 0 ? * ( atomic ) ^ ( newval ) ^ ( oldval ) : 0 ) ; ( gboolean ) __sync_bool_compare_and_swap ( ( atomic ) , ( oldval ) , ( newval ) ) ; } ) ) # */

/* g_atomic_int_add ( atomic , val ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gint ) ) ; ( void ) ( 0 ? * ( atomic ) ^ ( val ) : 0 ) ; ( gint ) __sync_fetch_and_add ( ( atomic ) , ( val ) ) ; } ) ) # */

/* g_atomic_int_and ( atomic , val ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gint ) ) ; ( void ) ( 0 ? * ( atomic ) ^ ( val ) : 0 ) ; ( guint ) __sync_fetch_and_and ( ( atomic ) , ( val ) ) ; } ) ) # */

/* g_atomic_int_or ( atomic , val ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gint ) ) ; ( void ) ( 0 ? * ( atomic ) ^ ( val ) : 0 ) ; ( guint ) __sync_fetch_and_or ( ( atomic ) , ( val ) ) ; } ) ) # */

/* g_atomic_int_xor ( atomic , val ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gint ) ) ; ( void ) ( 0 ? * ( atomic ) ^ ( val ) : 0 ) ; ( guint ) __sync_fetch_and_xor ( ( atomic ) , ( val ) ) ; } ) ) # */

/* g_atomic_pointer_compare_and_exchange ( atomic , oldval , newval ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gpointer ) ) ; ( void ) ( 0 ? ( gpointer ) * ( atomic ) : 0 ) ; ( gboolean ) __sync_bool_compare_and_swap ( ( atomic ) , ( oldval ) , ( newval ) ) ; } ) ) # */

/* g_atomic_pointer_add ( atomic , val ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gpointer ) ) ; ( void ) ( 0 ? ( gpointer ) * ( atomic ) : 0 ) ; ( void ) ( 0 ? ( val ) ^ ( val ) : 0 ) ; ( gssize ) __sync_fetch_and_add ( ( atomic ) , ( val ) ) ; } ) ) # */

/* g_atomic_pointer_and ( atomic , val ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gpointer ) ) ; ( void ) ( 0 ? ( gpointer ) * ( atomic ) : 0 ) ; ( void ) ( 0 ? ( val ) ^ ( val ) : 0 ) ; ( gsize ) __sync_fetch_and_and ( ( atomic ) , ( val ) ) ; } ) ) # */

/* g_atomic_pointer_or ( atomic , val ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gpointer ) ) ; ( void ) ( 0 ? ( gpointer ) * ( atomic ) : 0 ) ; ( void ) ( 0 ? ( val ) ^ ( val ) : 0 ) ; ( gsize ) __sync_fetch_and_or ( ( atomic ) , ( val ) ) ; } ) ) # */

/* g_atomic_pointer_xor ( atomic , val ) ( G_GNUC_EXTENSION ( { G_STATIC_ASSERT ( sizeof * ( atomic ) == sizeof ( gpointer ) ) ; ( void ) ( 0 ? ( gpointer ) * ( atomic ) : 0 ) ; ( void ) ( 0 ? ( val ) ^ ( val ) : 0 ) ; ( gsize ) __sync_fetch_and_xor ( ( atomic ) , ( val ) ) ; } ) ) # */

/* __G_ERROR_H__ # */

/* __G_QUARK_H__ # */

/* G_DEFINE_QUARK ( QN , q_n ) GQuark q_n ## _quark ( void ) \
{ static GQuark q ; if G_UNLIKELY ( q == 0 ) q = g_quark_from_static_string ( # QN ) ; return q ; \
} GLIB_AVAILABLE_IN_ALL */

/* G_THREAD_ERROR g_thread_error_quark ( ) GLIB_AVAILABLE_IN_ALL */

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

/* G_CONVERT_ERROR g_convert_error_quark ( ) GLIB_AVAILABLE_IN_ALL */

/* __G_DATASET_H__ # */

/* G_DATALIST_FLAGS_MASK 0x3 GLIB_AVAILABLE_IN_ALL */
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

/* g_clear_pointer ( pp , destroy ) G_STMT_START { G_STATIC_ASSERT ( sizeof * ( pp ) == sizeof ( gpointer ) ) ; /* Only one access, please */ gpointer * _pp = ( gpointer * ) ( pp ) ; gpointer _p ; /* This assignment is needed to avoid a gcc warning */ GDestroyNotify _destroy = ( GDestroyNotify ) ( destroy ) ; _p = * _pp ; if ( _p ) { * _pp = NULL ; _destroy ( _p ) ; } } G_STMT_END /* Optimise: avoid the call to the (slower) _n function if we can
 * determine at compile-time that no overflow happens.
 */ */

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

/* G_NODE_IS_LEAF ( node ) ( ( ( GNode * ) ( node ) ) -> children == NULL ) GLIB_AVAILABLE_IN_ALL */

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

/* g_node_append_data ( parent , data ) g_node_insert_before ( ( parent ) , NULL , g_node_new ( data ) ) /* traversal function, assumes that 'node' is root
 * (only traverses 'node' and its subtree).
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

/* g_list_free1 g_list_free_1 GLIB_AVAILABLE_IN_ALL */

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

/* g_slist_free1 g_slist_free_1 GLIB_AVAILABLE_IN_ALL */

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

/* g_utf8_next_char ( p ) ( char * ) ( ( p ) + g_utf8_skip [ * ( const guchar * ) ( p ) ] ) GLIB_AVAILABLE_IN_ALL */

/* __G_UTILS_H__ # */

/* G_INLINE_FUNC static __inline __attribute__ ( ( unused ) ) # */

/* ATEXIT ( proc ) g_ATEXIT ( proc ) GLIB_DEPRECATED */

/* G_WIN32_DLLMAIN_FOR_DLL_NAME ( static , dll_name ) # */

/* g_string_append_c ( gstr , c ) g_string_append_c_inline ( gstr , c ) # */

/* g_string_sprintf g_string_printf # */

/* g_string_sprintfa g_string_append_printf # */

/* G_IO_CHANNEL_ERROR g_io_channel_error_quark ( ) typedef */

/* __G_KEY_FILE_H__ # */

/* G_KEY_FILE_ERROR g_key_file_error_quark ( ) GLIB_AVAILABLE_IN_ALL */

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

/* G_KEY_FILE_DESKTOP_KEY_DBUS_ACTIVATABLE "DBusActivatable" # */

/* G_KEY_FILE_DESKTOP_KEY_ACTIONS "Actions" # */

/* G_KEY_FILE_DESKTOP_KEY_GETTEXT_DOMAIN "X-GNOME-Gettext-Domain" # */

/* G_KEY_FILE_DESKTOP_KEY_FULLNAME "X-GNOME-FullName" # */

/* G_KEY_FILE_DESKTOP_KEY_KEYWORDS "Keywords" # */

/* G_KEY_FILE_DESKTOP_TYPE_APPLICATION "Application" # */

/* G_KEY_FILE_DESKTOP_TYPE_LINK "Link" # */

/* G_KEY_FILE_DESKTOP_TYPE_DIRECTORY "Directory" G_END_DECLS */

/* __G_MAPPED_FILE_H__ # */

/* __G_MARKUP_H__ # */

/* G_MARKUP_ERROR g_markup_error_quark ( ) GLIB_AVAILABLE_IN_ALL */

/* __G_MESSAGES_H__ # */

/* G_LOG_LEVEL_USER_SHIFT ( 8 ) /* Glib log levels and flags.
 */ */

/* G_LOG_FATAL_MASK ( G_LOG_FLAG_RECURSION | G_LOG_LEVEL_ERROR ) typedef */

/* G_LOG_DOMAIN ( ( gchar * ) 0 ) # */

/* g_warn_if_reached ( ) do { g_warn_message ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , NULL ) ; } while ( 0 ) /**
 * g_warn_if_fail:
 * @expr: the expression to check
 *
 * Logs a warning if the expression is not true.
 *
 * Since: 2.16
 */ */

/* g_warn_if_fail ( expr ) do { if G_LIKELY ( expr ) ; else g_warn_message ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , # expr ) ; } while ( 0 ) # */

/* g_return_if_fail ( expr ) G_STMT_START { if G_LIKELY ( expr ) { } else { g_return_if_fail_warning ( G_LOG_DOMAIN , G_STRFUNC , # expr ) ; return ; } ; } G_STMT_END # */

/* g_return_val_if_fail ( expr , val ) G_STMT_START { if G_LIKELY ( expr ) { } else { g_return_if_fail_warning ( G_LOG_DOMAIN , G_STRFUNC , # expr ) ; return ( val ) ; } ; } G_STMT_END # */

/* g_return_if_reached ( ) G_STMT_START { g_log ( G_LOG_DOMAIN , G_LOG_LEVEL_CRITICAL , "file %s: line %d (%s): should not be reached" , __FILE__ , __LINE__ , G_STRFUNC ) ; return ; } G_STMT_END # */

/* g_return_val_if_reached ( val ) G_STMT_START { g_log ( G_LOG_DOMAIN , G_LOG_LEVEL_CRITICAL , "file %s: line %d (%s): should not be reached" , __FILE__ , __LINE__ , G_STRFUNC ) ; return ( val ) ; } G_STMT_END # */

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

/* G_OPTION_REMAINING "" GLIB_AVAILABLE_IN_ALL */

/* __G_PATTERN_H__ # */

/* __G_PRIMES_H__ # */

/* __G_QSORT_H__ # */

/* __G_QUEUE_H__ # */

/* G_QUEUE_INIT { NULL , NULL , 0 } /* Queues
 */ */

/* __G_RAND_H__ # */

/* g_rand_boolean ( rand_ ) ( ( g_rand_int ( rand_ ) & ( 1 << 15 ) ) != 0 ) GLIB_AVAILABLE_IN_ALL */

/* g_random_boolean ( ) ( ( g_random_int ( ) & ( 1 << 15 ) ) != 0 ) GLIB_AVAILABLE_IN_ALL */

/* __G_REGEX_H__ # */

/* G_REGEX_ERROR g_regex_error_quark ( ) GLIB_AVAILABLE_IN_ALL */

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
 * @G_SPAWN_ERROR_ACCES: execv() returned `EACCES`
 * @G_SPAWN_ERROR_PERM: execv() returned `EPERM`
 * @G_SPAWN_ERROR_TOO_BIG: execv() returned `E2BIG`
 * @G_SPAWN_ERROR_2BIG: deprecated alias for %G_SPAWN_ERROR_TOO_BIG
 * @G_SPAWN_ERROR_NOEXEC: execv() returned `ENOEXEC`
 * @G_SPAWN_ERROR_NAMETOOLONG: execv() returned `ENAMETOOLONG`
 * @G_SPAWN_ERROR_NOENT: execv() returned `ENOENT`
 * @G_SPAWN_ERROR_NOMEM: execv() returned `ENOMEM`
 * @G_SPAWN_ERROR_NOTDIR: execv() returned `ENOTDIR`
 * @G_SPAWN_ERROR_LOOP: execv() returned `ELOOP`
 * @G_SPAWN_ERROR_TXTBUSY: execv() returned `ETXTBUSY`
 * @G_SPAWN_ERROR_IO: execv() returned `EIO`
 * @G_SPAWN_ERROR_NFILE: execv() returned `ENFILE`
 * @G_SPAWN_ERROR_MFILE: execv() returned `EMFILE`
 * @G_SPAWN_ERROR_INVAL: execv() returned `EINVAL`
 * @G_SPAWN_ERROR_ISDIR: execv() returned `EISDIR`
 * @G_SPAWN_ERROR_LIBBAD: execv() returned `ELIBBAD`
 * @G_SPAWN_ERROR_FAILED: Some other fatal failure,
 *   `error->message` should explain.
 *
 * Error codes returned by spawning processes.
 */ */

/* G_SPAWN_EXIT_ERROR g_spawn_exit_error_quark ( ) /**
 * GSpawnChildSetupFunc:
 * @user_data: user data to pass to the function.
 *
 * Specifies the type of the setup function passed to g_spawn_async(),
 * g_spawn_sync() and g_spawn_async_with_pipes(), which can, in very
 * limited ways, be used to affect the child's execution.
 *
 * On POSIX platforms, the function is called in the child after GLib
 * has performed all the setup it plans to perform, but before calling
 * exec(). Actions taken in this function will only affect the child,
 * not the parent.
 *
 * On Windows, the function is called in the parent. Its usefulness on
 * Windows is thus questionable. In many cases executing the child setup
 * function in the parent can have ill effects, and you should be very
 * careful when porting software to Windows that uses child setup
 * functions.
 *
 * However, even on POSIX, you are extremely limited in what you can
 * safely do from a #GSpawnChildSetupFunc, because any mutexes that were
 * held by other threads in the parent process at the time of the fork()
 * will still be locked in the child process, and they will never be
 * unlocked (since the threads that held them don't exist in the child).
 * POSIX allows only async-signal-safe functions (see signal(7)) to be
 * called in the child between fork() and exec(), which drastically limits
 * the usefulness of child setup functions.
 *
 * In particular, it is not safe to call any function which may
 * call malloc(), which includes POSIX functions such as setenv().
 * If you need to set up the child environment differently from
 * the parent, you should use g_get_environ(), g_environ_setenv(),
 * and g_environ_unsetenv(), and then pass the complete environment
 * list to the `g_spawn...` function.
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

/* g_ascii_isxdigit ( c ) ( ( g_ascii_table [ ( guchar ) ( c ) ] & G_ASCII_XDIGIT ) != 0 ) GLIB_AVAILABLE_IN_ALL */

/* G_STR_DELIMITERS "_-|> <." GLIB_AVAILABLE_IN_ALL */

/* G_ASCII_DTOSTR_BUF_SIZE ( 29 + 10 ) GLIB_AVAILABLE_IN_ALL */

/* g_strstrip ( string ) g_strchomp ( g_strchug ( string ) ) GLIB_AVAILABLE_IN_ALL */

/* __G_STRINGCHUNK_H__ # */

/* __G_TEST_UTILS_H__ # */

/* g_assert_cmpstr ( s1 , cmp , s2 ) do { const char * __s1 = ( s1 ) , * __s2 = ( s2 ) ; if ( g_strcmp0 ( __s1 , __s2 ) cmp 0 ) ; else g_assertion_message_cmpstr ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , # s1 " " # cmp " " # s2 , __s1 , # cmp , __s2 ) ; } while ( 0 ) # */

/* g_assert_cmpint ( n1 , cmp , n2 ) do { gint64 __n1 = ( n1 ) , __n2 = ( n2 ) ; if ( __n1 cmp __n2 ) ; else g_assertion_message_cmpnum ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , # n1 " " # cmp " " # n2 , __n1 , # cmp , __n2 , 'i' ) ; } while ( 0 ) # */

/* g_assert_cmpuint ( n1 , cmp , n2 ) do { guint64 __n1 = ( n1 ) , __n2 = ( n2 ) ; if ( __n1 cmp __n2 ) ; else g_assertion_message_cmpnum ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , # n1 " " # cmp " " # n2 , __n1 , # cmp , __n2 , 'i' ) ; } while ( 0 ) # */

/* g_assert_cmphex ( n1 , cmp , n2 ) do { guint64 __n1 = ( n1 ) , __n2 = ( n2 ) ; if ( __n1 cmp __n2 ) ; else g_assertion_message_cmpnum ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , # n1 " " # cmp " " # n2 , __n1 , # cmp , __n2 , 'x' ) ; } while ( 0 ) # */

/* g_assert_cmpfloat ( n1 , cmp , n2 ) do { long double __n1 = ( n1 ) , __n2 = ( n2 ) ; if ( __n1 cmp __n2 ) ; else g_assertion_message_cmpnum ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , # n1 " " # cmp " " # n2 , __n1 , # cmp , __n2 , 'f' ) ; } while ( 0 ) # */

/* g_assert_no_error ( err ) do { if ( err ) g_assertion_message_error ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , # err , err , 0 , 0 ) ; } while ( 0 ) # */

/* g_assert_error ( err , dom , c ) do { if ( ! err || ( err ) -> domain != dom || ( err ) -> code != c ) g_assertion_message_error ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , # err , err , dom , c ) ; } while ( 0 ) # */

/* g_assert_true ( expr ) do { if G_LIKELY ( expr ) ; else g_assertion_message ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , "'" # expr "' should be TRUE" ) ; } while ( 0 ) # */

/* g_assert_false ( expr ) do { if G_LIKELY ( ! ( expr ) ) ; else g_assertion_message ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , "'" # expr "' should be FALSE" ) ; } while ( 0 ) # */

/* g_assert_null ( expr ) do { if G_LIKELY ( ( expr ) == NULL ) ; else g_assertion_message ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , "'" # expr "' should be NULL" ) ; } while ( 0 ) # */

/* g_assert_nonnull ( expr ) do { if G_LIKELY ( ( expr ) != NULL ) ; else g_assertion_message ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , "'" # expr "' should not be NULL" ) ; } while ( 0 ) # */

/* g_assert_not_reached ( ) do { g_assertion_message_expr ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , NULL ) ; } while ( 0 ) # */

/* g_assert ( expr ) do { if G_LIKELY ( expr ) ; else g_assertion_message_expr ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , # expr ) ; } while ( 0 ) # */

/* g_test_initialized ( ) ( g_test_config_vars -> test_initialized ) # */

/* g_test_quick ( ) ( g_test_config_vars -> test_quick ) # */

/* g_test_slow ( ) ( ! g_test_config_vars -> test_quick ) # */

/* g_test_thorough ( ) ( ! g_test_config_vars -> test_quick ) # */

/* g_test_perf ( ) ( g_test_config_vars -> test_perf ) # */

/* g_test_verbose ( ) ( g_test_config_vars -> test_verbose ) # */

/* g_test_quiet ( ) ( g_test_config_vars -> test_quiet ) # */

/* g_test_undefined ( ) ( g_test_config_vars -> test_undefined ) GLIB_AVAILABLE_IN_2_38 */

/* g_test_add ( testpath , Fixture , tdata , fsetup , ftest , fteardown ) G_STMT_START { void ( * add_vtable ) ( const char * , gsize , gconstpointer , void ( * ) ( Fixture * , gconstpointer ) , void ( * ) ( Fixture * , gconstpointer ) , void ( * ) ( Fixture * , gconstpointer ) ) = ( void ( * ) ( const gchar * , gsize , gconstpointer , void ( * ) ( Fixture * , gconstpointer ) , void ( * ) ( Fixture * , gconstpointer ) , void ( * ) ( Fixture * , gconstpointer ) ) ) g_test_add_vtable ; add_vtable ( testpath , sizeof ( Fixture ) , tdata , fsetup , ftest , fteardown ) ; } G_STMT_END /* add test messages to the test report */ */

/* g_test_queue_unref ( gobject ) g_test_queue_destroy ( g_object_unref , gobject ) typedef */

/* g_test_trap_assert_passed ( ) g_test_trap_assertions ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , 0 , 0 ) # */

/* g_test_trap_assert_failed ( ) g_test_trap_assertions ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , 1 , 0 ) # */

/* g_test_trap_assert_stdout ( soutpattern ) g_test_trap_assertions ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , 2 , soutpattern ) # */

/* g_test_trap_assert_stdout_unmatched ( soutpattern ) g_test_trap_assertions ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , 3 , soutpattern ) # */

/* g_test_trap_assert_stderr ( serrpattern ) g_test_trap_assertions ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , 4 , serrpattern ) # */

/* g_test_trap_assert_stderr_unmatched ( serrpattern ) g_test_trap_assertions ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC , 5 , serrpattern ) /* provide seed-able random numbers for tests */ */

/* g_test_rand_bit ( ) ( 0 != ( g_test_rand_int ( ) & ( 1 << 15 ) ) ) GLIB_AVAILABLE_IN_ALL */

/* g_test_assert_expected_messages ( ) g_test_assert_expected_messages_internal ( G_LOG_DOMAIN , __FILE__ , __LINE__ , G_STRFUNC ) G_END_DECLS */

/* __G_THREADPOOL_H__ # */

/* __G_TIMER_H__ # */

/* G_USEC_PER_SEC 1000000 GLIB_AVAILABLE_IN_ALL */
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

/* G_URI_RESERVED_CHARS_ALLOWED_IN_USERINFO G_URI_RESERVED_CHARS_SUBCOMPONENT_DELIMITERS ":" GLIB_AVAILABLE_IN_ALL */

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

/* G_VARIANT_PARSE_ERROR ( g_variant_parse_error_quark ( ) ) GLIB_DEPRECATED_IN_2_38_FOR */

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

/* g_main_set_poll_func ( func ) g_main_context_set_poll_func ( NULL , func ) # */

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

/* __G_OBJECT_H__ # */

/* __G_TYPE_H__ # */

/* G_TYPE_FUNDAMENTAL ( type ) ( g_type_fundamental ( type ) ) /**
 * G_TYPE_FUNDAMENTAL_MAX:
 * 
 * An integer constant that represents the number of identifiers reserved
 * for types that are assigned at compile-time.
 */ */

/* G_TYPE_FUNDAMENTAL_MAX ( 255 << G_TYPE_FUNDAMENTAL_SHIFT ) /* Constant fundamental types,
 */ */

/* G_TYPE_INVALID G_TYPE_MAKE_FUNDAMENTAL ( 0 ) /**
 * G_TYPE_NONE:
 * 
 * A fundamental type which is used as a replacement for the C
 * void return type.
 */ */

/* G_TYPE_NONE G_TYPE_MAKE_FUNDAMENTAL ( 1 ) /**
 * G_TYPE_INTERFACE:
 * 
 * The fundamental type from which all interfaces are derived.
 */ */

/* G_TYPE_INTERFACE G_TYPE_MAKE_FUNDAMENTAL ( 2 ) /**
 * G_TYPE_CHAR:
 * 
 * The fundamental type corresponding to #gchar.
 * The type designated by G_TYPE_CHAR is unconditionally an 8-bit signed integer.
 * This may or may not be the same type a the C type "gchar".
 */ */

/* G_TYPE_CHAR G_TYPE_MAKE_FUNDAMENTAL ( 3 ) /**
 * G_TYPE_UCHAR:
 * 
 * The fundamental type corresponding to #guchar.
 */ */

/* G_TYPE_UCHAR G_TYPE_MAKE_FUNDAMENTAL ( 4 ) /**
 * G_TYPE_BOOLEAN:
 * 
 * The fundamental type corresponding to #gboolean.
 */ */

/* G_TYPE_BOOLEAN G_TYPE_MAKE_FUNDAMENTAL ( 5 ) /**
 * G_TYPE_INT:
 * 
 * The fundamental type corresponding to #gint.
 */ */

/* G_TYPE_INT G_TYPE_MAKE_FUNDAMENTAL ( 6 ) /**
 * G_TYPE_UINT:
 * 
 * The fundamental type corresponding to #guint.
 */ */

/* G_TYPE_UINT G_TYPE_MAKE_FUNDAMENTAL ( 7 ) /**
 * G_TYPE_LONG:
 * 
 * The fundamental type corresponding to #glong.
 */ */

/* G_TYPE_LONG G_TYPE_MAKE_FUNDAMENTAL ( 8 ) /**
 * G_TYPE_ULONG:
 * 
 * The fundamental type corresponding to #gulong.
 */ */

/* G_TYPE_ULONG G_TYPE_MAKE_FUNDAMENTAL ( 9 ) /**
 * G_TYPE_INT64:
 * 
 * The fundamental type corresponding to #gint64.
 */ */

/* G_TYPE_INT64 G_TYPE_MAKE_FUNDAMENTAL ( 10 ) /**
 * G_TYPE_UINT64:
 * 
 * The fundamental type corresponding to #guint64.
 */ */

/* G_TYPE_UINT64 G_TYPE_MAKE_FUNDAMENTAL ( 11 ) /**
 * G_TYPE_ENUM:
 * 
 * The fundamental type from which all enumeration types are derived.
 */ */

/* G_TYPE_ENUM G_TYPE_MAKE_FUNDAMENTAL ( 12 ) /**
 * G_TYPE_FLAGS:
 * 
 * The fundamental type from which all flags types are derived.
 */ */

/* G_TYPE_FLAGS G_TYPE_MAKE_FUNDAMENTAL ( 13 ) /**
 * G_TYPE_FLOAT:
 * 
 * The fundamental type corresponding to #gfloat.
 */ */

/* G_TYPE_FLOAT G_TYPE_MAKE_FUNDAMENTAL ( 14 ) /**
 * G_TYPE_DOUBLE:
 * 
 * The fundamental type corresponding to #gdouble.
 */ */

/* G_TYPE_DOUBLE G_TYPE_MAKE_FUNDAMENTAL ( 15 ) /**
 * G_TYPE_STRING:
 * 
 * The fundamental type corresponding to nul-terminated C strings.
 */ */

/* G_TYPE_STRING G_TYPE_MAKE_FUNDAMENTAL ( 16 ) /**
 * G_TYPE_POINTER:
 * 
 * The fundamental type corresponding to #gpointer.
 */ */

/* G_TYPE_POINTER G_TYPE_MAKE_FUNDAMENTAL ( 17 ) /**
 * G_TYPE_BOXED:
 * 
 * The fundamental type from which all boxed types are derived.
 */ */

/* G_TYPE_BOXED G_TYPE_MAKE_FUNDAMENTAL ( 18 ) /**
 * G_TYPE_PARAM:
 * 
 * The fundamental type from which all #GParamSpec types are derived.
 */ */

/* G_TYPE_PARAM G_TYPE_MAKE_FUNDAMENTAL ( 19 ) /**
 * G_TYPE_OBJECT:
 * 
 * The fundamental type for #GObject.
 */ */

/* G_TYPE_OBJECT G_TYPE_MAKE_FUNDAMENTAL ( 20 ) /**
 * G_TYPE_VARIANT:
 *
 * The fundamental type corresponding to #GVariant.
 *
 * All floating #GVariant instances passed through the #GType system are
 * consumed.
 * 
 * Note that callbacks in closures, and signal handlers
 * for signals of return type %G_TYPE_VARIANT, must never return floating
 * variants.
 *
 * Note: GLib 2.24 did include a boxed type with this name. It was replaced
 * with this fundamental type in 2.26.
 *
 * Since: 2.26
 */ */

/* G_TYPE_VARIANT G_TYPE_MAKE_FUNDAMENTAL ( 21 ) /* Reserved fundamental type numbers to create new fundamental
 * type IDs with G_TYPE_MAKE_FUNDAMENTAL().
 * Send email to gtk-devel-list@gnome.org for reservations.
 */ */

/* G_TYPE_FUNDAMENTAL_SHIFT ( 2 ) /**
 * G_TYPE_MAKE_FUNDAMENTAL:
 * @x: the fundamental type number.
 * 
 * Get the type ID for the fundamental type number @x.
 * Use g_type_fundamental_next() instead of this macro to create new fundamental 
 * types.
 *
 * Returns: the GType
 */ */

/* G_TYPE_MAKE_FUNDAMENTAL ( x ) ( ( GType ) ( ( x ) << G_TYPE_FUNDAMENTAL_SHIFT ) ) /**
 * G_TYPE_RESERVED_GLIB_FIRST:
 * 
 * First fundamental type number to create a new fundamental type id with
 * G_TYPE_MAKE_FUNDAMENTAL() reserved for GLib.
 */ */

/* G_TYPE_RESERVED_GLIB_FIRST ( 22 ) /**
 * G_TYPE_RESERVED_GLIB_LAST:
 * 
 * Last fundamental type number reserved for GLib.
 */ */

/* G_TYPE_RESERVED_GLIB_LAST ( 31 ) /**
 * G_TYPE_RESERVED_BSE_FIRST:
 * 
 * First fundamental type number to create a new fundamental type id with
 * G_TYPE_MAKE_FUNDAMENTAL() reserved for BSE.
 */ */

/* G_TYPE_RESERVED_BSE_FIRST ( 32 ) /**
 * G_TYPE_RESERVED_BSE_LAST:
 * 
 * Last fundamental type number reserved for BSE.
 */ */

/* G_TYPE_RESERVED_BSE_LAST ( 48 ) /**
 * G_TYPE_RESERVED_USER_FIRST:
 * 
 * First available fundamental type number to create new fundamental 
 * type id with G_TYPE_MAKE_FUNDAMENTAL().
 */ */

/* G_TYPE_RESERVED_USER_FIRST ( 49 ) /* Type Checking Macros
 */ */

/* G_TYPE_IS_FUNDAMENTAL ( type ) ( ( type ) <= G_TYPE_FUNDAMENTAL_MAX ) /**
 * G_TYPE_IS_DERIVED:
 * @type: A #GType value
 * 
 * Checks if @type is derived (or in object-oriented terminology:
 * inherited) from another type (this holds true for all non-fundamental
 * types).
 *
 * Returns: %TRUE on success
 */ */

/* G_TYPE_IS_DERIVED ( type ) ( ( type ) > G_TYPE_FUNDAMENTAL_MAX ) /**
 * G_TYPE_IS_INTERFACE:
 * @type: A #GType value
 * 
 * Checks if @type is an interface type.
 * An interface type provides a pure API, the implementation
 * of which is provided by another type (which is then said to conform
 * to the interface).  GLib interfaces are somewhat analogous to Java
 * interfaces and C++ classes containing only pure virtual functions, 
 * with the difference that GType interfaces are not derivable (but see
 * g_type_interface_add_prerequisite() for an alternative).
 *
 * Returns: %TRUE on success
 */ */

/* G_TYPE_IS_INTERFACE ( type ) ( G_TYPE_FUNDAMENTAL ( type ) == G_TYPE_INTERFACE ) /**
 * G_TYPE_IS_CLASSED:
 * @type: A #GType value
 * 
 * Checks if @type is a classed type.
 *
 * Returns: %TRUE on success
 */ */

/* G_TYPE_IS_CLASSED ( type ) ( g_type_test_flags ( ( type ) , G_TYPE_FLAG_CLASSED ) ) /**
 * G_TYPE_IS_INSTANTIATABLE:
 * @type: A #GType value
 * 
 * Checks if @type can be instantiated.  Instantiation is the
 * process of creating an instance (object) of this type.
 *
 * Returns: %TRUE on success
 */ */

/* G_TYPE_IS_INSTANTIATABLE ( type ) ( g_type_test_flags ( ( type ) , G_TYPE_FLAG_INSTANTIATABLE ) ) /**
 * G_TYPE_IS_DERIVABLE:
 * @type: A #GType value
 * 
 * Checks if @type is a derivable type.  A derivable type can
 * be used as the base class of a flat (single-level) class hierarchy.
 *
 * Returns: %TRUE on success
 */ */

/* G_TYPE_IS_DERIVABLE ( type ) ( g_type_test_flags ( ( type ) , G_TYPE_FLAG_DERIVABLE ) ) /**
 * G_TYPE_IS_DEEP_DERIVABLE:
 * @type: A #GType value
 * 
 * Checks if @type is a deep derivable type.  A deep derivable type
 * can be used as the base class of a deep (multi-level) class hierarchy.
 *
 * Returns: %TRUE on success
 */ */

/* G_TYPE_IS_DEEP_DERIVABLE ( type ) ( g_type_test_flags ( ( type ) , G_TYPE_FLAG_DEEP_DERIVABLE ) ) /**
 * G_TYPE_IS_ABSTRACT:
 * @type: A #GType value
 * 
 * Checks if @type is an abstract type.  An abstract type cannot be
 * instantiated and is normally used as an abstract base class for
 * derived classes.
 *
 * Returns: %TRUE on success
 */ */

/* G_TYPE_IS_ABSTRACT ( type ) ( g_type_test_flags ( ( type ) , G_TYPE_FLAG_ABSTRACT ) ) /**
 * G_TYPE_IS_VALUE_ABSTRACT:
 * @type: A #GType value
 * 
 * Checks if @type is an abstract value type.  An abstract value type introduces
 * a value table, but can't be used for g_value_init() and is normally used as
 * an abstract base type for derived value types.
 *
 * Returns: %TRUE on success
 */ */

/* G_TYPE_IS_VALUE_ABSTRACT ( type ) ( g_type_test_flags ( ( type ) , G_TYPE_FLAG_VALUE_ABSTRACT ) ) /**
 * G_TYPE_IS_VALUE_TYPE:
 * @type: A #GType value
 * 
 * Checks if @type is a value type and can be used with g_value_init(). 
 *
 * Returns: %TRUE on success
 */ */

/* G_TYPE_IS_VALUE_TYPE ( type ) ( g_type_check_is_value_type ( type ) ) /**
 * G_TYPE_HAS_VALUE_TABLE:
 * @type: A #GType value
 * 
 * Checks if @type has a #GTypeValueTable.
 *
 * Returns: %TRUE on success
 */ */

/* G_TYPE_HAS_VALUE_TABLE ( type ) ( g_type_value_table_peek ( type ) != NULL ) /* Typedefs
 */ */

/* G_TYPE_CHECK_INSTANCE ( instance ) ( _G_TYPE_CHI ( ( GTypeInstance * ) ( instance ) ) ) /**
 * G_TYPE_CHECK_INSTANCE_CAST:
 * @instance: Location of a #GTypeInstance structure
 * @g_type: The type to be returned
 * @c_type: The corresponding C type of @g_type
 * 
 * Checks that @instance is an instance of the type identified by @g_type
 * and issues a warning if this is not the case. Returns @instance casted 
 * to a pointer to @c_type.
 * 
 * This macro should only be used in type implementations.
 */ */

/* G_TYPE_CHECK_INSTANCE_CAST ( instance , g_type , c_type ) ( _G_TYPE_CIC ( ( instance ) , ( g_type ) , c_type ) ) /**
 * G_TYPE_CHECK_INSTANCE_TYPE:
 * @instance: Location of a #GTypeInstance structure.
 * @g_type: The type to be checked
 * 
 * Checks if @instance is an instance of the type identified by @g_type.
 * 
 * This macro should only be used in type implementations.
 *
 * Returns: %TRUE on success
 */ */

/* G_TYPE_CHECK_INSTANCE_TYPE ( instance , g_type ) ( _G_TYPE_CIT ( ( instance ) , ( g_type ) ) ) /**
 * G_TYPE_CHECK_INSTANCE_FUNDAMENTAL_TYPE:
 * @instance: Location of a #GTypeInstance structure.
 * @g_type: The fundamental type to be checked
 *
 * Checks if @instance is an instance of the fundamental type identified by @g_type.
 *
 * This macro should only be used in type implementations.
 *
 * Returns: %TRUE on success
 */ */

/* G_TYPE_CHECK_INSTANCE_FUNDAMENTAL_TYPE ( instance , g_type ) ( _G_TYPE_CIFT ( ( instance ) , ( g_type ) ) ) /**
 * G_TYPE_INSTANCE_GET_CLASS:
 * @instance: Location of the #GTypeInstance structure
 * @g_type: The #GType of the class to be returned
 * @c_type: The C type of the class structure
 * 
 * Get the class structure of a given @instance, casted
 * to a specified ancestor type @g_type of the instance.
 * 
 * Note that while calling a GInstanceInitFunc(), the class pointer
 * gets modified, so it might not always return the expected pointer.
 * 
 * This macro should only be used in type implementations.
 *
 * Returns: a pointer to the class structure
 */ */

/* G_TYPE_INSTANCE_GET_CLASS ( instance , g_type , c_type ) ( _G_TYPE_IGC ( ( instance ) , ( g_type ) , c_type ) ) /**
 * G_TYPE_INSTANCE_GET_INTERFACE:
 * @instance: Location of the #GTypeInstance structure
 * @g_type: The #GType of the interface to be returned
 * @c_type: The C type of the interface structure
 * 
 * Get the interface structure for interface @g_type of a given @instance.
 * 
 * This macro should only be used in type implementations.
 *
 * Returns: a pointer to the interface structure
 */ */

/* G_TYPE_INSTANCE_GET_INTERFACE ( instance , g_type , c_type ) ( _G_TYPE_IGI ( ( instance ) , ( g_type ) , c_type ) ) /**
 * G_TYPE_CHECK_CLASS_CAST:
 * @g_class: Location of a #GTypeClass structure
 * @g_type: The type to be returned
 * @c_type: The corresponding C type of class structure of @g_type
 * 
 * Checks that @g_class is a class structure of the type identified by @g_type
 * and issues a warning if this is not the case. Returns @g_class casted 
 * to a pointer to @c_type.
 * 
 * This macro should only be used in type implementations.
 */ */

/* G_TYPE_CHECK_CLASS_CAST ( g_class , g_type , c_type ) ( _G_TYPE_CCC ( ( g_class ) , ( g_type ) , c_type ) ) /**
 * G_TYPE_CHECK_CLASS_TYPE:
 * @g_class: Location of a #GTypeClass structure
 * @g_type: The type to be checked
 * 
 * Checks if @g_class is a class structure of the type identified by 
 * @g_type.
 * 
 * This macro should only be used in type implementations.
 *
 * Returns: %TRUE on success
 */ */

/* G_TYPE_CHECK_CLASS_TYPE ( g_class , g_type ) ( _G_TYPE_CCT ( ( g_class ) , ( g_type ) ) ) /**
 * G_TYPE_CHECK_VALUE:
 * @value: a #GValue
 * 
 * Checks if @value has been initialized to hold values
 * of a value type.
 * 
 * This macro should only be used in type implementations.
 *
 * Returns: %TRUE on success
 */ */

/* G_TYPE_CHECK_VALUE ( value ) ( _G_TYPE_CHV ( ( value ) ) ) /**
 * G_TYPE_CHECK_VALUE_TYPE:
 * @value: a #GValue
 * @g_type: The type to be checked
 * 
 * Checks if @value has been initialized to hold values
 * of type @g_type. 
 * 
 * This macro should only be used in type implementations.
 *
 * Returns: %TRUE on success
 */ */

/* G_TYPE_CHECK_VALUE_TYPE ( value , g_type ) ( _G_TYPE_CVH ( ( value ) , ( g_type ) ) ) /**
 * G_TYPE_FROM_INSTANCE:
 * @instance: Location of a valid #GTypeInstance structure
 * 
 * Get the type identifier from a given @instance structure. 
 * 
 * This macro should only be used in type implementations.
 *
 * Returns: the #GType
 */ */

/* G_TYPE_FROM_INSTANCE ( instance ) ( G_TYPE_FROM_CLASS ( ( ( GTypeInstance * ) ( instance ) ) -> g_class ) ) /**
 * G_TYPE_FROM_CLASS:
 * @g_class: Location of a valid #GTypeClass structure
 * 
 * Get the type identifier from a given @class structure.
 * 
 * This macro should only be used in type implementations.
 *
 * Returns: the #GType
 */ */

/* G_TYPE_FROM_CLASS ( g_class ) ( ( ( GTypeClass * ) ( g_class ) ) -> g_type ) /**
 * G_TYPE_FROM_INTERFACE:
 * @g_iface: Location of a valid #GTypeInterface structure
 * 
 * Get the type identifier from a given @interface structure.
 * 
 * This macro should only be used in type implementations.
 *
 * Returns: the #GType
 */ */

/* G_TYPE_FROM_INTERFACE ( g_iface ) ( ( ( GTypeInterface * ) ( g_iface ) ) -> g_type ) /**
 * G_TYPE_INSTANCE_GET_PRIVATE:
 * @instance: the instance of a type deriving from @private_type
 * @g_type: the type identifying which private data to retrieve
 * @c_type: The C type for the private structure
 * 
 * Gets the private structure for a particular type.
 * The private structure must have been registered in the
 * class_init function with g_type_class_add_private().
 * 
 * This macro should only be used in type implementations.
 * 
 * Since: 2.4
 * Returns: a pointer to the private data structure
 */ */

/* G_TYPE_INSTANCE_GET_PRIVATE ( instance , g_type , c_type ) ( ( c_type * ) g_type_instance_get_private ( ( GTypeInstance * ) ( instance ) , ( g_type ) ) ) /**
 * G_TYPE_CLASS_GET_PRIVATE:
 * @klass: the class of a type deriving from @private_type
 * @g_type: the type identifying which private data to retrieve
 * @c_type: The C type for the private structure
 * 
 * Gets the private class structure for a particular type.
 * The private structure must have been registered in the
 * get_type() function with g_type_add_class_private().
 * 
 * This macro should only be used in type implementations.
 * 
 * Since: 2.24
 * Returns: a pointer to the private data structure
 */ */

/* G_TYPE_CLASS_GET_PRIVATE ( klass , g_type , c_type ) ( ( c_type * ) g_type_class_get_private ( ( GTypeClass * ) ( klass ) , ( g_type ) ) ) /**
 * GTypeDebugFlags:
 * @G_TYPE_DEBUG_NONE: Print no messages
 * @G_TYPE_DEBUG_OBJECTS: Print messages about object bookkeeping
 * @G_TYPE_DEBUG_SIGNALS: Print messages about signal emissions
 * @G_TYPE_DEBUG_MASK: Mask covering all debug flags
 *
 * These flags used to be passed to g_type_init_with_debug_flags() which
 * is now deprecated.
 *
 * If you need to enable debugging features, use the GOBJECT_DEBUG
 * environment variable.
 *
 * Deprecated: 2.36: g_type_init() is now done automatically
 */ */

/* G_DEFINE_TYPE ( TN , t_n , T_P ) G_DEFINE_TYPE_EXTENDED ( TN , t_n , T_P , 0 , { } ) /**
 * G_DEFINE_TYPE_WITH_CODE:
 * @TN: The name of the new type, in Camel case.
 * @t_n: The name of the new type in lowercase, with words separated by '_'.
 * @T_P: The #GType of the parent type.
 * @_C_: Custom code that gets inserted in the *_get_type() function.
 * 
 * A convenience macro for type implementations.  
 * Similar to G_DEFINE_TYPE(), but allows you to insert custom code into the 
 * *_get_type() function, e.g. interface implementations via G_IMPLEMENT_INTERFACE().
 * See G_DEFINE_TYPE_EXTENDED() for an example.
 * 
 * Since: 2.4
 */ */

/* G_DEFINE_TYPE_WITH_CODE ( TN , t_n , T_P , _C_ ) _G_DEFINE_TYPE_EXTENDED_BEGIN ( TN , t_n , T_P , 0 ) { _C_ ; } _G_DEFINE_TYPE_EXTENDED_END ( ) /**
 * G_DEFINE_TYPE_WITH_PRIVATE:
 * @TN: The name of the new type, in Camel case.
 * @t_n: The name of the new type, in lowercase, with words 
 *  separated by '_'.
 * @T_P: The #GType of the parent type.
 * 
 * A convenience macro for type implementations, which declares a class
 * initialization function, an instance initialization function (see #GTypeInfo
 * for information about these), a static variable named `t_n_parent_class`
 * pointing to the parent class, and adds private instance data to the type.
 * Furthermore, it defines a *_get_type() function. See G_DEFINE_TYPE_EXTENDED()
 * for an example.
 * 
 * Note that private structs added with this macros must have a struct
 * name of the form @TN Private.
 *
 * Since: 2.38
 */ */

/* G_DEFINE_TYPE_WITH_PRIVATE ( TN , t_n , T_P ) G_DEFINE_TYPE_EXTENDED ( TN , t_n , T_P , 0 , G_ADD_PRIVATE ( TN ) ) /**
 * G_DEFINE_ABSTRACT_TYPE:
 * @TN: The name of the new type, in Camel case.
 * @t_n: The name of the new type, in lowercase, with words 
 *  separated by '_'.
 * @T_P: The #GType of the parent type.
 * 
 * A convenience macro for type implementations. 
 * Similar to G_DEFINE_TYPE(), but defines an abstract type. 
 * See G_DEFINE_TYPE_EXTENDED() for an example.
 * 
 * Since: 2.4
 */ */

/* G_DEFINE_ABSTRACT_TYPE ( TN , t_n , T_P ) G_DEFINE_TYPE_EXTENDED ( TN , t_n , T_P , G_TYPE_FLAG_ABSTRACT , { } ) /**
 * G_DEFINE_ABSTRACT_TYPE_WITH_CODE:
 * @TN: The name of the new type, in Camel case.
 * @t_n: The name of the new type, in lowercase, with words 
 *  separated by '_'.
 * @T_P: The #GType of the parent type.
 * @_C_: Custom code that gets inserted in the @type_name_get_type() function.
 * 
 * A convenience macro for type implementations.
 * Similar to G_DEFINE_TYPE_WITH_CODE(), but defines an abstract type and
 * allows you to insert custom code into the *_get_type() function, e.g.
 * interface implementations  via G_IMPLEMENT_INTERFACE().
 * See G_DEFINE_TYPE_EXTENDED() for an example.
 * 
 * Since: 2.4
 */ */

/* G_DEFINE_ABSTRACT_TYPE_WITH_CODE ( TN , t_n , T_P , _C_ ) _G_DEFINE_TYPE_EXTENDED_BEGIN ( TN , t_n , T_P , G_TYPE_FLAG_ABSTRACT ) { _C_ ; } _G_DEFINE_TYPE_EXTENDED_END ( ) /**
 * G_DEFINE_ABSTRACT_TYPE_WITH_PRIVATE:
 * @TN: The name of the new type, in Camel case.
 * @t_n: The name of the new type, in lowercase, with words 
 *  separated by '_'.
 * @T_P: The #GType of the parent type.
 *
 * Similar to G_DEFINE_TYPE_WITH_PRIVATE(), but defines an abstract type. 
 * See G_DEFINE_TYPE_EXTENDED() for an example.
 * 
 * Since: 2.38
 */ */

/* G_DEFINE_ABSTRACT_TYPE_WITH_PRIVATE ( TN , t_n , T_P ) G_DEFINE_TYPE_EXTENDED ( TN , t_n , T_P , G_TYPE_FLAG_ABSTRACT , G_ADD_PRIVATE ( TN ) ) /**
 * G_DEFINE_TYPE_EXTENDED:
 * @TN: The name of the new type, in Camel case.
 * @t_n: The name of the new type, in lowercase, with words
 *    separated by '_'.
 * @T_P: The #GType of the parent type.
 * @_f_: #GTypeFlags to pass to g_type_register_static()
 * @_C_: Custom code that gets inserted in the *_get_type() function.
 *
 * The most general convenience macro for type implementations, on which
 * G_DEFINE_TYPE(), etc are based.
 *
 * |[<!-- language="C" -->
 * G_DEFINE_TYPE_EXTENDED (GtkGadget,
 *                         gtk_gadget,
 *                         GTK_TYPE_WIDGET,
 *                         0,
 *                         G_IMPLEMENT_INTERFACE (TYPE_GIZMO,
 *                                                gtk_gadget_gizmo_init));
 * ]|
 * expands to
 * |[<!-- language="C" -->
 * static void     gtk_gadget_init       (GtkGadget      *self);
 * static void     gtk_gadget_class_init (GtkGadgetClass *klass);
 * static gpointer gtk_gadget_parent_class = NULL;
 * static void     gtk_gadget_class_intern_init (gpointer klass)
 * {
 *   gtk_gadget_parent_class = g_type_class_peek_parent (klass);
 *   gtk_gadget_class_init ((GtkGadgetClass*) klass);
 * }
 *
 * GType
 * gtk_gadget_get_type (void)
 * {
 *   static volatile gsize g_define_type_id__volatile = 0;
 *   if (g_once_init_enter (&g_define_type_id__volatile))
 *     {
 *       GType g_define_type_id =
 *         g_type_register_static_simple (GTK_TYPE_WIDGET,
 *                                        g_intern_static_string ("GtkGadget"),
 *                                        sizeof (GtkGadgetClass),
 *                                        (GClassInitFunc) gtk_gadget_class_intern_init,
 *                                        sizeof (GtkGadget),
 *                                        (GInstanceInitFunc) gtk_gadget_init,
 *                                        0);
 *       {
 *         const GInterfaceInfo g_implement_interface_info = {
 *           (GInterfaceInitFunc) gtk_gadget_gizmo_init
 *         };
 *         g_type_add_interface_static (g_define_type_id, TYPE_GIZMO, &g_implement_interface_info);
 *       }
 *       g_once_init_leave (&g_define_type_id__volatile, g_define_type_id);
 *     }
 *   return g_define_type_id__volatile;
 * }
 * ]|
 * The only pieces which have to be manually provided are the definitions of
 * the instance and class structure and the definitions of the instance and
 * class init functions.
 *
 * Since: 2.4
 */ */

/* G_DEFINE_TYPE_EXTENDED ( TN , t_n , T_P , _f_ , _C_ ) _G_DEFINE_TYPE_EXTENDED_BEGIN ( TN , t_n , T_P , _f_ ) { _C_ ; } _G_DEFINE_TYPE_EXTENDED_END ( ) /**
 * G_DEFINE_INTERFACE:
 * @TN: The name of the new type, in Camel case.
 * @t_n: The name of the new type, in lowercase, with words separated by '_'.
 * @T_P: The #GType of the prerequisite type for the interface, or 0
 * (%G_TYPE_INVALID) for no prerequisite type.
 *
 * A convenience macro for #GTypeInterface definitions, which declares
 * a default vtable initialization function and defines a *_get_type()
 * function.
 *
 * The macro expects the interface initialization function to have the
 * name `t_n ## _default_init`, and the interface structure to have the
 * name `TN ## Interface`.
 *
 * Since: 2.24
 */ */

/* G_DEFINE_INTERFACE ( TN , t_n , T_P ) G_DEFINE_INTERFACE_WITH_CODE ( TN , t_n , T_P , ; ) /**
 * G_DEFINE_INTERFACE_WITH_CODE:
 * @TN: The name of the new type, in Camel case.
 * @t_n: The name of the new type, in lowercase, with words separated by '_'.
 * @T_P: The #GType of the prerequisite type for the interface, or 0
 * (%G_TYPE_INVALID) for no prerequisite type.
 * @_C_: Custom code that gets inserted in the *_get_type() function.
 *
 * A convenience macro for #GTypeInterface definitions. Similar to
 * G_DEFINE_INTERFACE(), but allows you to insert custom code into the
 * *_get_type() function, e.g. additional interface implementations
 * via G_IMPLEMENT_INTERFACE(), or additional prerequisite types. See
 * G_DEFINE_TYPE_EXTENDED() for a similar example using
 * G_DEFINE_TYPE_WITH_CODE().
 *
 * Since: 2.24
 */ */

/* G_DEFINE_INTERFACE_WITH_CODE ( TN , t_n , T_P , _C_ ) _G_DEFINE_INTERFACE_EXTENDED_BEGIN ( TN , t_n , T_P ) { _C_ ; } _G_DEFINE_INTERFACE_EXTENDED_END ( ) /**
 * G_IMPLEMENT_INTERFACE:
 * @TYPE_IFACE: The #GType of the interface to add
 * @iface_init: The interface init function
 *
 * A convenience macro to ease interface addition in the `_C_` section
 * of G_DEFINE_TYPE_WITH_CODE() or G_DEFINE_ABSTRACT_TYPE_WITH_CODE().
 * See G_DEFINE_TYPE_EXTENDED() for an example.
 *
 * Note that this macro can only be used together with the G_DEFINE_TYPE_*
 * macros, since it depends on variable names from those macros.
 *
 * Since: 2.4
 */ */

/* G_IMPLEMENT_INTERFACE ( TYPE_IFACE , iface_init ) { const GInterfaceInfo g_implement_interface_info = { ( GInterfaceInitFunc ) iface_init , NULL , NULL } ; g_type_add_interface_static ( g_define_type_id , TYPE_IFACE , & g_implement_interface_info ) ; \
} /**
 * G_ADD_PRIVATE:
 * @TypeName: the name of the type in CamelCase
 *
 * A convenience macro to ease adding private data to instances of a new type
 * in the @_C_ section of G_DEFINE_TYPE_WITH_CODE() or
 * G_DEFINE_ABSTRACT_TYPE_WITH_CODE().
 *
 * For instance:
 *
 * |[<!-- language="C" -->
 *   typedef struct _MyObject MyObject;
 *   typedef struct _MyObjectClass MyObjectClass;
 *
 *   typedef struct {
 *     gint foo;
 *     gint bar;
 *   } MyObjectPrivate;
 *
 *   G_DEFINE_TYPE_WITH_CODE (MyObject, my_object, G_TYPE_OBJECT,
 *                            G_ADD_PRIVATE (MyObject))
 * ]|
 *
 * Will add MyObjectPrivate as the private data to any instance of the MyObject
 * type.
 *
 * G_DEFINE_TYPE_* macros will automatically create a private function
 * based on the arguments to this macro, which can be used to safely
 * retrieve the private data from an instance of the type; for instance:
 *
 * |[<!-- language="C" -->
 *   gint
 *   my_object_get_foo (MyObject *obj)
 *   {
 *     MyObjectPrivate *priv = my_object_get_instance_private (obj);
 *
 *     return priv->foo;
 *   }
 *
 *   void
 *   my_object_set_bar (MyObject *obj,
 *                      gint      bar)
 *   {
 *     MyObjectPrivate *priv = my_object_get_instance_private (obj);
 *
 *     if (priv->bar != bar)
 *       priv->bar = bar;
 *   }
 * ]|
 *
 * Note that this macro can only be used together with the G_DEFINE_TYPE_*
 * macros, since it depends on variable names from those macros.
 *
 * Also note that private structs added with these macros must have a struct
 * name of the form `TypeNamePrivate`.
 *
 * Since: 2.38
 */ */

/* G_ADD_PRIVATE ( TypeName ) { TypeName ## _private_offset = g_type_add_instance_private ( g_define_type_id , sizeof ( TypeName ## Private ) ) ; \
} /**
 * G_PRIVATE_OFFSET:
 * @TypeName: the name of the type in CamelCase
 * @field: the name of the field in the private data structure
 *
 * Evaluates to the offset of the @field inside the instance private data
 * structure for @TypeName.
 *
 * Note that this macro can only be used together with the G_DEFINE_TYPE_*
 * and G_ADD_PRIVATE() macros, since it depends on variable names from
 * those macros.
 *
 * Since: 2.38
 */ */

/* G_PRIVATE_OFFSET ( TypeName , field ) ( TypeName ## _private_offset + ( G_STRUCT_OFFSET ( TypeName ## Private , field ) ) ) /**
 * G_PRIVATE_FIELD_P:
 * @TypeName: the name of the type in CamelCase
 * @inst: the instance of @TypeName you wish to access
 * @field_name: the name of the field in the private data structure
 *
 * Evaluates to a pointer to the @field_name inside the @inst private data
 * structure for @TypeName.
 *
 * Note that this macro can only be used together with the G_DEFINE_TYPE_*
 * and G_ADD_PRIVATE() macros, since it depends on variable names from
 * those macros.
 *
 * Since: 2.38
 */ */

/* G_PRIVATE_FIELD_P ( TypeName , inst , field_name ) G_STRUCT_MEMBER_P ( inst , G_PRIVATE_OFFSET ( TypeName , field_name ) ) /**
 * G_PRIVATE_FIELD:
 * @TypeName: the name of the type in CamelCase
 * @inst: the instance of @TypeName you wish to access
 * @field_type: the type of the field in the private data structure
 * @field_name: the name of the field in the private data structure
 *
 * Evaluates to the @field_name inside the @inst private data
 * structure for @TypeName.
 *
 * Note that this macro can only be used together with the G_DEFINE_TYPE_*
 * and G_ADD_PRIVATE() macros, since it depends on variable names from
 * those macros.
 *
 * Since: 2.38
 */ */

/* G_PRIVATE_FIELD ( TypeName , inst , field_type , field_name ) G_STRUCT_MEMBER ( field_type , inst , G_PRIVATE_OFFSET ( TypeName , field_name ) ) /* we need to have this macro under conditional expansion, as it references
 * a function that has been added in 2.38. see bug:
 * https://bugzilla.gnome.org/show_bug.cgi?id=703191
 */ */

/* _G_DEFINE_TYPE_EXTENDED_CLASS_INIT ( TypeName , type_name ) static void type_name ## _class_intern_init ( gpointer klass ) \
{ type_name ## _parent_class = g_type_class_peek_parent ( klass ) ; if ( TypeName ## _private_offset != 0 ) g_type_class_adjust_private_offset ( klass , & TypeName ## _private_offset ) ; type_name ## _class_init ( ( TypeName ## Class * ) klass ) ; \
} # */

/* _G_DEFINE_TYPE_EXTENDED_BEGIN ( TypeName , type_name , TYPE_PARENT , flags ) static void type_name ## _init ( TypeName * self ) ; static void type_name ## _class_init ( TypeName ## Class * klass ) ; static gpointer type_name ## _parent_class = NULL ; static gint TypeName ## _private_offset ; _G_DEFINE_TYPE_EXTENDED_CLASS_INIT ( TypeName , type_name ) G_GNUC_UNUSED static inline gpointer type_name ## _get_instance_private ( TypeName * self ) \
{ return ( G_STRUCT_MEMBER_P ( self , TypeName ## _private_offset ) ) ; \
} GType type_name ## _get_type ( void ) \
{ static volatile gsize g_define_type_id__volatile = 0 ; if ( g_once_init_enter ( & g_define_type_id__volatile ) ) { GType g_define_type_id = g_type_register_static_simple ( TYPE_PARENT , g_intern_static_string ( # TypeName ) , sizeof ( TypeName ## Class ) , ( GClassInitFunc ) type_name ## _class_intern_init , sizeof ( TypeName ) , ( GInstanceInitFunc ) type_name ## _init , ( GTypeFlags ) flags ) ; { /* custom code follows */ */

/* _G_DEFINE_TYPE_EXTENDED_END ( ) /* following custom code */ } g_once_init_leave ( & g_define_type_id__volatile , g_define_type_id ) ; } return g_define_type_id__volatile ; \
} /* closes type_name##_get_type() */ */

/* _G_DEFINE_INTERFACE_EXTENDED_BEGIN ( TypeName , type_name , TYPE_PREREQ ) static void type_name ## _default_init ( TypeName ## Interface * klass ) ; GType type_name ## _get_type ( void ) \
{ static volatile gsize g_define_type_id__volatile = 0 ; if ( g_once_init_enter ( & g_define_type_id__volatile ) ) { GType g_define_type_id = g_type_register_static_simple ( G_TYPE_INTERFACE , g_intern_static_string ( # TypeName ) , sizeof ( TypeName ## Interface ) , ( GClassInitFunc ) type_name ## _default_init , 0 , ( GInstanceInitFunc ) NULL , ( GTypeFlags ) 0 ) ; if ( TYPE_PREREQ ) g_type_interface_add_prerequisite ( g_define_type_id , TYPE_PREREQ ) ; { /* custom code follows */ */

/* _G_DEFINE_INTERFACE_EXTENDED_END ( ) /* following custom code */ } g_once_init_leave ( & g_define_type_id__volatile , g_define_type_id ) ; } return g_define_type_id__volatile ; \
} /* closes type_name##_get_type() */ */

/* G_DEFINE_BOXED_TYPE ( TypeName , type_name , copy_func , free_func ) G_DEFINE_BOXED_TYPE_WITH_CODE ( TypeName , type_name , copy_func , free_func , { } ) /**
 * G_DEFINE_BOXED_TYPE_WITH_CODE:
 * @TypeName: The name of the new type, in Camel case
 * @type_name: The name of the new type, in lowercase, with words
 *  separated by '_'
 * @copy_func: the #GBoxedCopyFunc for the new type
 * @free_func: the #GBoxedFreeFunc for the new type
 * @_C_: Custom code that gets inserted in the *_get_type() function
 *
 * A convenience macro for boxed type implementations.
 * Similar to G_DEFINE_BOXED_TYPE(), but allows to insert custom code into the
 * type_name_get_type() function, e.g. to register value transformations with
 * g_value_register_transform_func().
 *
 * Since: 2.26
 */ */

/* G_DEFINE_BOXED_TYPE_WITH_CODE ( TypeName , type_name , copy_func , free_func , _C_ ) _G_DEFINE_BOXED_TYPE_BEGIN ( TypeName , type_name , copy_func , free_func ) { _C_ ; } _G_DEFINE_TYPE_EXTENDED_END ( ) /* Only use this in non-C++ on GCC >= 2.7, except for Darwin/ppc64.
 * See https://bugzilla.gnome.org/show_bug.cgi?id=647145
 */ */

/* _G_DEFINE_BOXED_TYPE_BEGIN ( TypeName , type_name , copy_func , free_func ) GType type_name ## _get_type ( void ) \
{ static volatile gsize g_define_type_id__volatile = 0 ; if ( g_once_init_enter ( & g_define_type_id__volatile ) ) { GType ( * _g_register_boxed ) ( const gchar * , union { TypeName * ( * do_copy_type ) ( TypeName * ) ; TypeName * ( * do_const_copy_type ) ( const TypeName * ) ; GBoxedCopyFunc do_copy_boxed ; } __attribute__ ( ( __transparent_union__ ) ) , union { void ( * do_free_type ) ( TypeName * ) ; GBoxedFreeFunc do_free_boxed ; } __attribute__ ( ( __transparent_union__ ) ) ) = g_boxed_type_register_static ; GType g_define_type_id = _g_register_boxed ( g_intern_static_string ( # TypeName ) , copy_func , free_func ) ; { /* custom code follows */ */

/* G_DEFINE_POINTER_TYPE ( TypeName , type_name ) G_DEFINE_POINTER_TYPE_WITH_CODE ( TypeName , type_name , { } ) /**
 * G_DEFINE_POINTER_TYPE_WITH_CODE:
 * @TypeName: The name of the new type, in Camel case
 * @type_name: The name of the new type, in lowercase, with words
 *  separated by '_'
 * @_C_: Custom code that gets inserted in the *_get_type() function
 *
 * A convenience macro for pointer type implementations.
 * Similar to G_DEFINE_POINTER_TYPE(), but allows to insert
 * custom code into the type_name_get_type() function.
 *
 * Since: 2.26
 */ */

/* G_DEFINE_POINTER_TYPE_WITH_CODE ( TypeName , type_name , _C_ ) _G_DEFINE_POINTER_TYPE_BEGIN ( TypeName , type_name ) { _C_ ; } _G_DEFINE_TYPE_EXTENDED_END ( ) # */

/* _G_DEFINE_POINTER_TYPE_BEGIN ( TypeName , type_name ) GType type_name ## _get_type ( void ) \
{ static volatile gsize g_define_type_id__volatile = 0 ; if ( g_once_init_enter ( & g_define_type_id__volatile ) ) { GType g_define_type_id = g_pointer_type_register_static ( g_intern_static_string ( # TypeName ) ) ; { /* custom code follows */ */

/* _G_TYPE_CIC ( ip , gt , ct ) ( ( ct * ) g_type_check_instance_cast ( ( GTypeInstance * ) ip , gt ) ) # */

/* _G_TYPE_CCC ( cp , gt , ct ) ( ( ct * ) g_type_check_class_cast ( ( GTypeClass * ) cp , gt ) ) # */

/* _G_TYPE_CHI ( ip ) ( g_type_check_instance ( ( GTypeInstance * ) ip ) ) # */

/* _G_TYPE_CHV ( vl ) ( g_type_check_value ( ( GValue * ) vl ) ) # */

/* _G_TYPE_IGC ( ip , gt , ct ) ( ( ct * ) ( ( ( GTypeInstance * ) ip ) -> g_class ) ) # */

/* _G_TYPE_IGI ( ip , gt , ct ) ( ( ct * ) g_type_interface_peek ( ( ( GTypeInstance * ) ip ) -> g_class , gt ) ) # */

/* _G_TYPE_CIFT ( ip , ft ) ( g_type_check_instance_is_fundamentally_a ( ( GTypeInstance * ) ip , ft ) ) # */

/* _G_TYPE_CIT ( ip , gt ) ( G_GNUC_EXTENSION ( { GTypeInstance * __inst = ( GTypeInstance * ) ip ; GType __t = gt ; gboolean __r ; if ( ! __inst ) __r = FALSE ; else if ( __inst -> g_class && __inst -> g_class -> g_type == __t ) __r = TRUE ; else __r = g_type_check_instance_is_a ( __inst , __t ) ; __r ; \
} ) ) # */

/* _G_TYPE_CCT ( cp , gt ) ( G_GNUC_EXTENSION ( { GTypeClass * __class = ( GTypeClass * ) cp ; GType __t = gt ; gboolean __r ; if ( ! __class ) __r = FALSE ; else if ( __class -> g_type == __t ) __r = TRUE ; else __r = g_type_check_class_is_a ( __class , __t ) ; __r ; \
} ) ) # */

/* _G_TYPE_CVH ( vl , gt ) ( G_GNUC_EXTENSION ( { GValue * __val = ( GValue * ) vl ; GType __t = gt ; gboolean __r ; if ( ! __val ) __r = FALSE ; else if ( __val -> g_type == __t ) __r = TRUE ; else __r = g_type_check_value_holds ( __val , __t ) ; __r ; \
} ) ) # */

/* G_TYPE_FLAG_RESERVED_ID_BIT ( ( GType ) ( 1 << 0 ) ) G_END_DECLS */

/* __G_VALUE_H__ # */

/* G_TYPE_IS_VALUE ( type ) ( g_type_check_is_value_type ( type ) ) /**
 * G_IS_VALUE:
 * @value: A #GValue structure.
 * 
 * Checks if @value is a valid and initialized #GValue structure.
 *
 * Returns: %TRUE on success.
 */ */

/* G_IS_VALUE ( value ) ( G_TYPE_CHECK_VALUE ( value ) ) /**
 * G_VALUE_TYPE:
 * @value: A #GValue structure.
 *
 * Get the type identifier of @value.
 *
 * Returns: the #GType.
 */ */

/* G_VALUE_TYPE ( value ) ( ( ( GValue * ) ( value ) ) -> g_type ) /**
 * G_VALUE_TYPE_NAME:
 * @value: A #GValue structure.
 *
 * Gets the type name of @value.
 *
 * Returns: the type name.
 */ */

/* G_VALUE_TYPE_NAME ( value ) ( g_type_name ( G_VALUE_TYPE ( value ) ) ) /**
 * G_VALUE_HOLDS:
 * @value: A #GValue structure.
 * @type: A #GType value.
 *
 * Checks if @value holds (or contains) a value of @type.
 * This macro will also check for @value != %NULL and issue a
 * warning if the check fails.
 *
 * Returns: %TRUE if @value holds the @type.
 */ */

/* G_VALUE_HOLDS ( value , type ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , ( type ) ) ) /* --- typedefs & structures --- */ */

/* G_VALUE_NOCOPY_CONTENTS ( 1 << 27 ) /**
 * G_VALUE_INIT:
 *
 * A #GValue must be initialized before it can be used. This macro can
 * be used as initializer instead of an explicit `{ 0 }` when declaring
 * a variable, but it cannot be assigned to a variable.
 *
 * |[
 *   GValue value = G_VALUE_INIT;
 * ]|
 *
 * Since: 2.30
 */ */

/* G_VALUE_INIT { 0 , { { 0 } } } G_END_DECLS */

/* __G_PARAM_H__ # */

/* G_TYPE_IS_PARAM ( type ) ( G_TYPE_FUNDAMENTAL ( type ) == G_TYPE_PARAM ) /**
 * G_PARAM_SPEC:
 * @pspec: a valid #GParamSpec
 * 
 * Casts a derived #GParamSpec object (e.g. of type #GParamSpecInt) into
 * a #GParamSpec object.
 */ */

/* G_PARAM_SPEC ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM , GParamSpec ) ) /**
 * G_IS_PARAM_SPEC:
 * @pspec: a #GParamSpec
 * 
 * Checks whether @pspec "is a" valid #GParamSpec structure of type %G_TYPE_PARAM
 * or derived.
 */ */

/* G_IS_PARAM_SPEC ( pspec ) ( G_TYPE_CHECK_INSTANCE_FUNDAMENTAL_TYPE ( ( pspec ) , G_TYPE_PARAM ) ) # */

/* G_PARAM_SPEC_CLASS ( pclass ) ( G_TYPE_CHECK_CLASS_CAST ( ( pclass ) , G_TYPE_PARAM , GParamSpecClass ) ) /**
 * G_IS_PARAM_SPEC_CLASS:
 * @pclass: a #GParamSpecClass
 * 
 * Checks whether @pclass "is a" valid #GParamSpecClass structure of type 
 * %G_TYPE_PARAM or derived.
 */ */

/* G_IS_PARAM_SPEC_CLASS ( pclass ) ( G_TYPE_CHECK_CLASS_TYPE ( ( pclass ) , G_TYPE_PARAM ) ) /**
 * G_PARAM_SPEC_GET_CLASS:
 * @pspec: a valid #GParamSpec
 * 
 * Retrieves the #GParamSpecClass of a #GParamSpec.
 */ */

/* G_PARAM_SPEC_GET_CLASS ( pspec ) ( G_TYPE_INSTANCE_GET_CLASS ( ( pspec ) , G_TYPE_PARAM , GParamSpecClass ) ) /* --- convenience macros --- */ */

/* G_PARAM_SPEC_TYPE ( pspec ) ( G_TYPE_FROM_INSTANCE ( pspec ) ) /**
 * G_PARAM_SPEC_TYPE_NAME:
 * @pspec: a valid #GParamSpec
 * 
 * Retrieves the #GType name of this @pspec.
 */ */

/* G_PARAM_SPEC_TYPE_NAME ( pspec ) ( g_type_name ( G_PARAM_SPEC_TYPE ( pspec ) ) ) /**
 * G_PARAM_SPEC_VALUE_TYPE:
 * @pspec: a valid #GParamSpec
 * 
 * Retrieves the #GType to initialize a #GValue for this parameter.
 */ */

/* G_PARAM_SPEC_VALUE_TYPE ( pspec ) ( G_PARAM_SPEC ( pspec ) -> value_type ) /**
 * G_VALUE_HOLDS_PARAM:
 * @value: a valid #GValue structure
 * 
 * Checks whether the given #GValue can hold values derived from type %G_TYPE_PARAM.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_VALUE_HOLDS_PARAM ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_PARAM ) ) /* --- flags --- */ */

/* G_PARAM_STATIC_STRINGS ( G_PARAM_STATIC_NAME | G_PARAM_STATIC_NICK | G_PARAM_STATIC_BLURB ) /* bits in the range 0xffffff00 are reserved for 3rd party usage */ */

/* G_PARAM_MASK ( 0x000000ff ) /**
 * G_PARAM_USER_SHIFT:
 * 
 * Minimum shift count to be used for user defined flags, to be stored in
 * #GParamSpec.flags. The maximum allowed is 10.
 */ */

/* G_PARAM_USER_SHIFT ( 8 ) /* --- typedefs & structures --- */ */

/* __G_CLOSURE_H__ # */

/* G_CLOSURE_NEEDS_MARSHAL ( closure ) ( ( ( GClosure * ) ( closure ) ) -> marshal == NULL ) /**
 * G_CLOSURE_N_NOTIFIERS:
 * @cl: a #GClosure
 * 
 * Get the total number of notifiers connected with the closure @cl. 
 * The count includes the meta marshaller, the finalize and invalidate notifiers 
 * and the marshal guards. Note that each guard counts as two notifiers. 
 * See g_closure_set_meta_marshal(), g_closure_add_finalize_notifier(),
 * g_closure_add_invalidate_notifier() and g_closure_add_marshal_guards().
 *
 * Returns: number of notifiers
 */ */

/* G_CLOSURE_N_NOTIFIERS ( cl ) ( ( ( cl ) -> n_guards << 1L ) + ( cl ) -> n_fnotifiers + ( cl ) -> n_inotifiers ) /**
 * G_CCLOSURE_SWAP_DATA:
 * @cclosure: a #GCClosure
 * 
 * Checks whether the user data of the #GCClosure should be passed as the
 * first parameter to the callback. See g_cclosure_new_swap().
 *
 * Returns: %TRUE if data has to be swapped.
 */ */

/* G_CCLOSURE_SWAP_DATA ( cclosure ) ( ( ( GClosure * ) ( cclosure ) ) -> derivative_flag ) /**
 * G_CALLBACK:
 * @f: a function pointer.
 * 
 * Cast a function pointer to a #GCallback.
 */ */

/* G_CALLBACK ( f ) ( ( GCallback ) ( f ) ) /* -- typedefs --- */ */

/* __G_SIGNAL_H__ # */

/* __G_MARSHAL_H__ G_BEGIN_DECLS */

/* g_cclosure_marshal_BOOL__FLAGS g_cclosure_marshal_BOOLEAN__FLAGS /* STRING:OBJECT,POINTER (./gmarshal.list:28) */ */

/* g_cclosure_marshal_BOOL__BOXED_BOXED g_cclosure_marshal_BOOLEAN__BOXED_BOXED G_END_DECLS */

/* G_SIGNAL_FLAGS_MASK 0x1ff /**
 * GConnectFlags:
 * @G_CONNECT_AFTER: whether the handler should be called before or after the 
 *  default handler of the signal.
 * @G_CONNECT_SWAPPED: whether the instance and data should be swapped when
 *  calling the handler; see g_signal_connect_swapped() for an example.
 * 
 * The connection flags are used to specify the behaviour of a signal's 
 * connection.
 */ */
pub const G_SIGNAL_FLAGS_MASK: i32 = 511;

/* G_SIGNAL_MATCH_MASK 0x3f /**
 * G_SIGNAL_TYPE_STATIC_SCOPE:
 * 
 * This macro flags signal argument types for which the signal system may 
 * assume that instances thereof remain persistent across all signal emissions
 * they are used in. This is only useful for non ref-counted, value-copy types.
 * 
 * To flag a signal argument in this way, add `| G_SIGNAL_TYPE_STATIC_SCOPE`
 * to the corresponding argument of g_signal_new().
 * |[
 * g_signal_new ("size_request",
 *   G_TYPE_FROM_CLASS (gobject_class),
 * 	 G_SIGNAL_RUN_FIRST,
 * 	 G_STRUCT_OFFSET (GtkWidgetClass, size_request),
 * 	 NULL, NULL,
 * 	 _gtk_marshal_VOID__BOXED,
 * 	 G_TYPE_NONE, 1,
 * 	 GTK_TYPE_REQUISITION | G_SIGNAL_TYPE_STATIC_SCOPE);
 * ]|
 */ */
pub const G_SIGNAL_MATCH_MASK: i32 = 63;

/* G_SIGNAL_TYPE_STATIC_SCOPE ( G_TYPE_FLAG_RESERVED_ID_BIT ) /* --- signal information --- */ */

/* g_signal_connect ( instance , detailed_signal , c_handler , data ) g_signal_connect_data ( ( instance ) , ( detailed_signal ) , ( c_handler ) , ( data ) , NULL , ( GConnectFlags ) 0 ) /**
 * g_signal_connect_after:
 * @instance: the instance to connect to.
 * @detailed_signal: a string of the form "signal-name::detail".
 * @c_handler: the #GCallback to connect.
 * @data: data to pass to @c_handler calls.
 * 
 * Connects a #GCallback function to a signal for a particular object.
 * 
 * The handler will be called after the default handler of the signal.
 * 
 * Returns: the handler id (always greater than 0 for successful connections)
 */ */

/* g_signal_connect_after ( instance , detailed_signal , c_handler , data ) g_signal_connect_data ( ( instance ) , ( detailed_signal ) , ( c_handler ) , ( data ) , NULL , G_CONNECT_AFTER ) /**
 * g_signal_connect_swapped:
 * @instance: the instance to connect to.
 * @detailed_signal: a string of the form "signal-name::detail".
 * @c_handler: the #GCallback to connect.
 * @data: data to pass to @c_handler calls.
 * 
 * Connects a #GCallback function to a signal for a particular object.
 * 
 * The instance on which the signal is emitted and @data will be swapped when 
 * calling the handler. This is useful when calling pre-existing functions to
 * operate purely on the @data, rather than the @instance: swapping the
 * parameters avoids the need to write a wrapper function.
 *
 * For example, this allows the shorter code:
 * |[<!-- language="C" -->
 * g_signal_connect_swapped (button, "clicked",
 *                           (GCallback) gtk_widget_hide, other_widget);
 * ]|
 *
 * Rather than the cumbersome:
 * |[<!-- language="C" -->
 * static void
 * button_clicked_cb (GtkButton *button, GtkWidget *other_widget)
 * {
 *     gtk_widget_hide (other_widget);
 * }
 *
 * …
 *
 * g_signal_connect (button, "clicked",
 *                   (GCallback) button_clicked_cb, other_widget);
 * ]|
 * 
 * Returns: the handler ID (always greater than 0 for successful connections)
 */ */

/* g_signal_connect_swapped ( instance , detailed_signal , c_handler , data ) g_signal_connect_data ( ( instance ) , ( detailed_signal ) , ( c_handler ) , ( data ) , NULL , G_CONNECT_SWAPPED ) /**
 * g_signal_handlers_disconnect_by_func:
 * @instance: The instance to remove handlers from.
 * @func: The C closure callback of the handlers (useless for non-C closures).
 * @data: The closure data of the handlers' closures.
 * 
 * Disconnects all handlers on an instance that match @func and @data.
 * 
 * Returns: The number of handlers that matched.
 */ */

/* g_signal_handlers_disconnect_by_func ( instance , func , data ) g_signal_handlers_disconnect_matched ( ( instance ) , ( GSignalMatchType ) ( G_SIGNAL_MATCH_FUNC | G_SIGNAL_MATCH_DATA ) , 0 , 0 , NULL , ( func ) , ( data ) ) /**
 * g_signal_handlers_disconnect_by_data:
 * @instance: The instance to remove handlers from
 * @data: the closure data of the handlers' closures
 *
 * Disconnects all handlers on an instance that match @data.
 *
 * Returns: The number of handlers that matched.
 *
 * Since: 2.32
 */ */

/* g_signal_handlers_disconnect_by_data ( instance , data ) g_signal_handlers_disconnect_matched ( ( instance ) , G_SIGNAL_MATCH_DATA , 0 , 0 , NULL , NULL , ( data ) ) /**
 * g_signal_handlers_block_by_func:
 * @instance: The instance to block handlers from.
 * @func: The C closure callback of the handlers (useless for non-C closures).
 * @data: The closure data of the handlers' closures.
 * 
 * Blocks all handlers on an instance that match @func and @data.
 * 
 * Returns: The number of handlers that matched.
 */ */

/* g_signal_handlers_block_by_func ( instance , func , data ) g_signal_handlers_block_matched ( ( instance ) , ( GSignalMatchType ) ( G_SIGNAL_MATCH_FUNC | G_SIGNAL_MATCH_DATA ) , 0 , 0 , NULL , ( func ) , ( data ) ) /**
 * g_signal_handlers_unblock_by_func:
 * @instance: The instance to unblock handlers from.
 * @func: The C closure callback of the handlers (useless for non-C closures).
 * @data: The closure data of the handlers' closures.
 * 
 * Unblocks all handlers on an instance that match @func and @data.
 * 
 * Returns: The number of handlers that matched.
 */ */

/* g_signal_handlers_unblock_by_func ( instance , func , data ) g_signal_handlers_unblock_matched ( ( instance ) , ( GSignalMatchType ) ( G_SIGNAL_MATCH_FUNC | G_SIGNAL_MATCH_DATA ) , 0 , 0 , NULL , ( func ) , ( data ) ) GLIB_AVAILABLE_IN_ALL */

/* __G_BOXED_H__ # */

/* __GLIB_TYPES_H__ # */

/* G_TYPE_DATE ( g_date_get_type ( ) ) /**
 * G_TYPE_STRV:
 *
 * The #GType for a boxed type holding a %NULL-terminated array of strings.
 *
 * The code fragments in the following example show the use of a property of
 * type #G_TYPE_STRV with g_object_class_install_property(), g_object_set()
 * and g_object_get().
 *
 * |[
 * g_object_class_install_property (object_class,
 *                                  PROP_AUTHORS,
 *                                  g_param_spec_boxed ("authors",
 *                                                      _("Authors"),
 *                                                      _("List of authors"),
 *                                                      G_TYPE_STRV,
 *                                                      G_PARAM_READWRITE));
 *
 * gchar *authors[] = { "Owen", "Tim", NULL };
 * g_object_set (obj, "authors", authors, NULL);
 *
 * gchar *writers[];
 * g_object_get (obj, "authors", &writers, NULL);
 * /&ast; do something with writers &ast;/
 * g_strfreev (writers);
 * ]|
 *
 * Since: 2.4
 */ */

/* G_TYPE_STRV ( g_strv_get_type ( ) ) /**
 * G_TYPE_GSTRING:
 *
 * The #GType for #GString.
 */ */

/* G_TYPE_GSTRING ( g_gstring_get_type ( ) ) /**
 * G_TYPE_HASH_TABLE:
 *
 * The #GType for a boxed type holding a #GHashTable reference.
 *
 * Since: 2.10
 */ */

/* G_TYPE_HASH_TABLE ( g_hash_table_get_type ( ) ) /**
 * G_TYPE_REGEX:
 *
 * The #GType for a boxed type holding a #GRegex reference.
 *
 * Since: 2.14
 */ */

/* G_TYPE_REGEX ( g_regex_get_type ( ) ) /**
 * G_TYPE_MATCH_INFO:
 *
 * The #GType for a boxed type holding a #GMatchInfo reference.
 *
 * Since: 2.30
 */ */

/* G_TYPE_MATCH_INFO ( g_match_info_get_type ( ) ) /**
 * G_TYPE_ARRAY:
 *
 * The #GType for a boxed type holding a #GArray reference.
 *
 * Since: 2.22
 */ */

/* G_TYPE_ARRAY ( g_array_get_type ( ) ) /**
 * G_TYPE_BYTE_ARRAY:
 *
 * The #GType for a boxed type holding a #GByteArray reference.
 *
 * Since: 2.22
 */ */

/* G_TYPE_BYTE_ARRAY ( g_byte_array_get_type ( ) ) /**
 * G_TYPE_PTR_ARRAY:
 *
 * The #GType for a boxed type holding a #GPtrArray reference.
 *
 * Since: 2.22
 */ */

/* G_TYPE_PTR_ARRAY ( g_ptr_array_get_type ( ) ) /**
 * G_TYPE_BYTES:
 *
 * The #GType for #GBytes.
 *
 * Since: 2.32
 */ */

/* G_TYPE_BYTES ( g_bytes_get_type ( ) ) /**
 * G_TYPE_VARIANT_TYPE:
 *
 * The #GType for a boxed type holding a #GVariantType.
 *
 * Since: 2.24
 */ */

/* G_TYPE_VARIANT_TYPE ( g_variant_type_get_gtype ( ) ) /**
 * G_TYPE_ERROR:
 *
 * The #GType for a boxed type holding a #GError.
 *
 * Since: 2.26
 */ */

/* G_TYPE_ERROR ( g_error_get_type ( ) ) /**
 * G_TYPE_DATE_TIME:
 *
 * The #GType for a boxed type holding a #GDateTime.
 *
 * Since: 2.26
 */ */

/* G_TYPE_DATE_TIME ( g_date_time_get_type ( ) ) /**
 * G_TYPE_TIME_ZONE:
 *
 * The #GType for a boxed type holding a #GTimeZone.
 *
 * Since: 2.34
 */ */

/* G_TYPE_TIME_ZONE ( g_time_zone_get_type ( ) ) /**
 * G_TYPE_IO_CHANNEL:
 *
 * The #GType for #GIOChannel.
 */ */

/* G_TYPE_IO_CHANNEL ( g_io_channel_get_type ( ) ) /**
 * G_TYPE_IO_CONDITION:
 *
 * The #GType for #GIOCondition.
 */ */

/* G_TYPE_IO_CONDITION ( g_io_condition_get_type ( ) ) /**
 * G_TYPE_VARIANT_BUILDER:
 *
 * The #GType for a boxed type holding a #GVariantBuilder.
 *
 * Since: 2.30
 */ */

/* G_TYPE_VARIANT_BUILDER ( g_variant_builder_get_type ( ) ) /**
 * G_TYPE_VARIANT_DICT:
 *
 * The #GType for a boxed type holding a #GVariantDict.
 *
 * Since: 2.40
 */ */

/* G_TYPE_VARIANT_DICT ( g_variant_dict_get_type ( ) ) /**
 * G_TYPE_MAIN_LOOP:
 *
 * The #GType for a boxed type holding a #GMainLoop.
 *
 * Since: 2.30
 */ */

/* G_TYPE_MAIN_LOOP ( g_main_loop_get_type ( ) ) /**
 * G_TYPE_MAIN_CONTEXT:
 *
 * The #GType for a boxed type holding a #GMainContext.
 *
 * Since: 2.30
 */ */

/* G_TYPE_MAIN_CONTEXT ( g_main_context_get_type ( ) ) /**
 * G_TYPE_SOURCE:
 *
 * The #GType for a boxed type holding a #GSource.
 *
 * Since: 2.30
 */ */

/* G_TYPE_SOURCE ( g_source_get_type ( ) ) /**
 * G_TYPE_POLLFD:
 *
 * The #GType for a boxed type holding a #GPollFD.
 *
 * Since: 2.36
 */ */

/* G_TYPE_POLLFD ( g_pollfd_get_type ( ) ) /**
 * G_TYPE_MARKUP_PARSE_CONTEXT:
 *
 * The #GType for a boxed type holding a #GMarkupParseContext.
 *
 * Since: 2.36
 */ */

/* G_TYPE_MARKUP_PARSE_CONTEXT ( g_markup_parse_context_get_type ( ) ) /**
 * G_TYPE_KEY_FILE:
 *
 * The #GType for a boxed type holding a #GKeyFile.
 *
 * Since: 2.32
 */ */

/* G_TYPE_KEY_FILE ( g_key_file_get_type ( ) ) /**
 * G_TYPE_MAPPED_FILE:
 *
 * The #GType for a boxed type holding a #GMappedFile.
 *
 * Since: 2.40
 */ */

/* G_TYPE_MAPPED_FILE ( g_mapped_file_get_type ( ) ) /**
 * G_TYPE_THREAD:
 *
 * The #GType for a boxed type holding a #GThread.
 *
 * Since: 2.36
 */ */

/* G_TYPE_THREAD ( g_thread_get_type ( ) ) /**
 * G_TYPE_CHECKSUM:
 *
 * The #GType for a boxed type holding a #GChecksum.
 *
 * Since: 2.36
 */ */

/* G_TYPE_CHECKSUM ( g_checksum_get_type ( ) ) GLIB_AVAILABLE_IN_ALL */

/* G_TYPE_IS_BOXED ( type ) ( G_TYPE_FUNDAMENTAL ( type ) == G_TYPE_BOXED ) /**
 * G_VALUE_HOLDS_BOXED:
 * @value: a valid #GValue structure
 *
 * Checks whether the given #GValue can hold values derived
 * from type %G_TYPE_BOXED.
 *
 * Returns: %TRUE on success.
 */ */

/* G_VALUE_HOLDS_BOXED ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_BOXED ) ) /* --- typedefs --- */ */

/* G_TYPE_CLOSURE ( g_closure_get_type ( ) ) /**
 * G_TYPE_VALUE:
 *
 * The type ID of the "GValue" type which is a boxed type,
 * used to pass around pointers to GValues.
 */ */

/* G_TYPE_VALUE ( g_value_get_type ( ) ) GLIB_AVAILABLE_IN_ALL */

/* G_TYPE_IS_OBJECT ( type ) ( G_TYPE_FUNDAMENTAL ( type ) == G_TYPE_OBJECT ) /**
 * G_OBJECT:
 * @object: Object which is subject to casting.
 * 
 * Casts a #GObject or derived pointer into a (GObject*) pointer.
 * Depending on the current debugging level, this function may invoke
 * certain runtime checks to identify invalid casts.
 */ */

/* G_OBJECT ( object ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( object ) , G_TYPE_OBJECT , GObject ) ) /**
 * G_OBJECT_CLASS:
 * @class: a valid #GObjectClass
 * 
 * Casts a derived #GObjectClass structure into a #GObjectClass structure.
 */ */

/* G_OBJECT_CLASS ( class ) ( G_TYPE_CHECK_CLASS_CAST ( ( class ) , G_TYPE_OBJECT , GObjectClass ) ) /**
 * G_IS_OBJECT:
 * @object: Instance to check for being a %G_TYPE_OBJECT.
 * 
 * Checks whether a valid #GTypeInstance pointer is of type %G_TYPE_OBJECT.
 */ */

/* G_IS_OBJECT ( object ) ( G_TYPE_CHECK_INSTANCE_FUNDAMENTAL_TYPE ( ( object ) , G_TYPE_OBJECT ) ) # */

/* G_IS_OBJECT_CLASS ( class ) ( G_TYPE_CHECK_CLASS_TYPE ( ( class ) , G_TYPE_OBJECT ) ) /**
 * G_OBJECT_GET_CLASS:
 * @object: a #GObject instance.
 * 
 * Get the class structure associated to a #GObject instance.
 *
 * Returns: pointer to object class structure.
 */ */

/* G_OBJECT_GET_CLASS ( object ) ( G_TYPE_INSTANCE_GET_CLASS ( ( object ) , G_TYPE_OBJECT , GObjectClass ) ) /**
 * G_OBJECT_TYPE:
 * @object: Object to return the type id for.
 * 
 * Get the type id of an object.
 * 
 * Returns: Type id of @object.
 */ */

/* G_OBJECT_TYPE ( object ) ( G_TYPE_FROM_INSTANCE ( object ) ) /**
 * G_OBJECT_TYPE_NAME:
 * @object: Object to return the type name for.
 * 
 * Get the name of an object's type.
 * 
 * Returns: Type name of @object. The string is owned by the type system and 
 *  should not be freed.
 */ */

/* G_OBJECT_TYPE_NAME ( object ) ( g_type_name ( G_OBJECT_TYPE ( object ) ) ) /**
 * G_OBJECT_CLASS_TYPE:
 * @class: a valid #GObjectClass
 * 
 * Get the type id of a class structure.
 * 
 * Returns: Type id of @class.
 */ */

/* G_OBJECT_CLASS_TYPE ( class ) ( G_TYPE_FROM_CLASS ( class ) ) /**
 * G_OBJECT_CLASS_NAME:
 * @class: a valid #GObjectClass
 * 
 * Return the name of a class structure's type.
 * 
 * Returns: Type name of @class. The string is owned by the type system and 
 *  should not be freed.
 */ */

/* G_OBJECT_CLASS_NAME ( class ) ( g_type_name ( G_OBJECT_CLASS_TYPE ( class ) ) ) /**
 * G_VALUE_HOLDS_OBJECT:
 * @value: a valid #GValue structure
 * 
 * Checks whether the given #GValue can hold values derived from type %G_TYPE_OBJECT.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_VALUE_HOLDS_OBJECT ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_OBJECT ) ) /* --- type macros --- */ */

/* G_TYPE_INITIALLY_UNOWNED ( g_initially_unowned_get_type ( ) ) /**
 * G_INITIALLY_UNOWNED:
 * @object: Object which is subject to casting.
 * 
 * Casts a #GInitiallyUnowned or derived pointer into a (GInitiallyUnowned*) 
 * pointer. Depending on the current debugging level, this function may invoke
 * certain runtime checks to identify invalid casts.
 */ */

/* G_INITIALLY_UNOWNED ( object ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( object ) , G_TYPE_INITIALLY_UNOWNED , GInitiallyUnowned ) ) /**
 * G_INITIALLY_UNOWNED_CLASS:
 * @class: a valid #GInitiallyUnownedClass
 * 
 * Casts a derived #GInitiallyUnownedClass structure into a
 * #GInitiallyUnownedClass structure.
 */ */

/* G_INITIALLY_UNOWNED_CLASS ( class ) ( G_TYPE_CHECK_CLASS_CAST ( ( class ) , G_TYPE_INITIALLY_UNOWNED , GInitiallyUnownedClass ) ) /**
 * G_IS_INITIALLY_UNOWNED:
 * @object: Instance to check for being a %G_TYPE_INITIALLY_UNOWNED.
 * 
 * Checks whether a valid #GTypeInstance pointer is of type %G_TYPE_INITIALLY_UNOWNED.
 */ */

/* G_IS_INITIALLY_UNOWNED ( object ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( object ) , G_TYPE_INITIALLY_UNOWNED ) ) /**
 * G_IS_INITIALLY_UNOWNED_CLASS:
 * @class: a #GInitiallyUnownedClass
 * 
 * Checks whether @class "is a" valid #GInitiallyUnownedClass structure of type
 * %G_TYPE_INITIALLY_UNOWNED or derived.
 */ */

/* G_IS_INITIALLY_UNOWNED_CLASS ( class ) ( G_TYPE_CHECK_CLASS_TYPE ( ( class ) , G_TYPE_INITIALLY_UNOWNED ) ) /**
 * G_INITIALLY_UNOWNED_GET_CLASS:
 * @object: a #GInitiallyUnowned instance.
 * 
 * Get the class structure associated to a #GInitiallyUnowned instance.
 *
 * Returns: pointer to object class structure.
 */ */

/* G_INITIALLY_UNOWNED_GET_CLASS ( object ) ( G_TYPE_INSTANCE_GET_CLASS ( ( object ) , G_TYPE_INITIALLY_UNOWNED , GInitiallyUnownedClass ) ) /* GInitiallyUnowned ia a GObject with initially floating reference count */ */

/* G_OBJECT_WARN_INVALID_PSPEC ( object , pname , property_id , pspec ) G_STMT_START { GObject * _glib__object = ( GObject * ) ( object ) ; GParamSpec * _glib__pspec = ( GParamSpec * ) ( pspec ) ; guint _glib__property_id = ( property_id ) ; g_warning ( "%s: invalid %s id %u for \"%s\" of type '%s' in '%s'" , G_STRLOC , ( pname ) , _glib__property_id , _glib__pspec -> name , g_type_name ( G_PARAM_SPEC_TYPE ( _glib__pspec ) ) , G_OBJECT_TYPE_NAME ( _glib__object ) ) ; \
} G_STMT_END /**
 * G_OBJECT_WARN_INVALID_PROPERTY_ID:
 * @object: the #GObject on which set_property() or get_property() was called
 * @property_id: the numeric id of the property
 * @pspec: the #GParamSpec of the property
 * 
 * This macro should be used to emit a standard warning about unexpected 
 * properties in set_property() and get_property() implementations.
 */ */

/* G_OBJECT_WARN_INVALID_PROPERTY_ID ( object , property_id , pspec ) G_OBJECT_WARN_INVALID_PSPEC ( ( object ) , "property" , ( property_id ) , ( pspec ) ) GLIB_AVAILABLE_IN_ALL */

/* g_clear_object ( object_ptr ) g_clear_pointer ( ( object_ptr ) , g_object_unref ) typedef */

/* G_TYPE_BINDING_FLAGS ( g_binding_flags_get_type ( ) ) # */

/* G_TYPE_BINDING ( g_binding_get_type ( ) ) # */

/* G_BINDING ( obj ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( obj ) , G_TYPE_BINDING , GBinding ) ) # */

/* G_IS_BINDING ( obj ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( obj ) , G_TYPE_BINDING ) ) /**
 * GBinding:
 *
 * GBinding is an opaque structure whose members
 * cannot be accessed directly.
 *
 * Since: 2.26
 */ */

/* __G_ENUMS_H__ # */

/* G_TYPE_IS_ENUM ( type ) ( G_TYPE_FUNDAMENTAL ( type ) == G_TYPE_ENUM ) /**
 * G_ENUM_CLASS:
 * @class: a valid #GEnumClass
 * 
 * Casts a derived #GEnumClass structure into a #GEnumClass structure.
 */ */

/* G_ENUM_CLASS ( class ) ( G_TYPE_CHECK_CLASS_CAST ( ( class ) , G_TYPE_ENUM , GEnumClass ) ) /**
 * G_IS_ENUM_CLASS:
 * @class: a #GEnumClass
 * 
 * Checks whether @class "is a" valid #GEnumClass structure of type %G_TYPE_ENUM
 * or derived.
 */ */

/* G_IS_ENUM_CLASS ( class ) ( G_TYPE_CHECK_CLASS_TYPE ( ( class ) , G_TYPE_ENUM ) ) /**
 * G_ENUM_CLASS_TYPE:
 * @class: a #GEnumClass
 * 
 * Get the type identifier from a given #GEnumClass structure.
 *
 * Returns: the #GType
 */ */

/* G_ENUM_CLASS_TYPE ( class ) ( G_TYPE_FROM_CLASS ( class ) ) /**
 * G_ENUM_CLASS_TYPE_NAME:
 * @class: a #GEnumClass
 * 
 * Get the static type name from a given #GEnumClass structure.
 *
 * Returns: the type name.
 */ */

/* G_ENUM_CLASS_TYPE_NAME ( class ) ( g_type_name ( G_ENUM_CLASS_TYPE ( class ) ) ) /**
 * G_TYPE_IS_FLAGS:
 * @type: a #GType ID.
 *
 * Checks whether @type "is a" %G_TYPE_FLAGS. 
 *
 * Returns: %TRUE if @type "is a" %G_TYPE_FLAGS.
 */ */

/* G_TYPE_IS_FLAGS ( type ) ( G_TYPE_FUNDAMENTAL ( type ) == G_TYPE_FLAGS ) /**
 * G_FLAGS_CLASS:
 * @class: a valid #GFlagsClass
 * 
 * Casts a derived #GFlagsClass structure into a #GFlagsClass structure.
 */ */

/* G_FLAGS_CLASS ( class ) ( G_TYPE_CHECK_CLASS_CAST ( ( class ) , G_TYPE_FLAGS , GFlagsClass ) ) /**
 * G_IS_FLAGS_CLASS:
 * @class: a #GFlagsClass
 * 
 * Checks whether @class "is a" valid #GFlagsClass structure of type %G_TYPE_FLAGS
 * or derived.
 */ */

/* G_IS_FLAGS_CLASS ( class ) ( G_TYPE_CHECK_CLASS_TYPE ( ( class ) , G_TYPE_FLAGS ) ) /**
 * G_FLAGS_CLASS_TYPE:
 * @class: a #GFlagsClass
 * 
 * Get the type identifier from a given #GFlagsClass structure.
 *
 * Returns: the #GType
 */ */

/* G_FLAGS_CLASS_TYPE ( class ) ( G_TYPE_FROM_CLASS ( class ) ) /**
 * G_FLAGS_CLASS_TYPE_NAME:
 * @class: a #GFlagsClass
 * 
 * Get the static type name from a given #GFlagsClass structure.
 *
 * Returns: the type name.
 */ */

/* G_FLAGS_CLASS_TYPE_NAME ( class ) ( g_type_name ( G_FLAGS_CLASS_TYPE ( class ) ) ) /**
 * G_VALUE_HOLDS_ENUM:
 * @value: a valid #GValue structure
 * 
 * Checks whether the given #GValue can hold values derived from type %G_TYPE_ENUM.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_VALUE_HOLDS_ENUM ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_ENUM ) ) /**
 * G_VALUE_HOLDS_FLAGS:
 * @value: a valid #GValue structure
 * 
 * Checks whether the given #GValue can hold values derived from type %G_TYPE_FLAGS.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_VALUE_HOLDS_FLAGS ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_FLAGS ) ) /* --- enum/flag values & classes --- */ */

/* __G_PARAMSPECS_H__ # */

/* G_TYPE_PARAM_CHAR ( g_param_spec_types [ 0 ] ) /**
 * G_IS_PARAM_SPEC_CHAR:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_CHAR.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_CHAR ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_CHAR ) ) /**
 * G_PARAM_SPEC_CHAR:
 * @pspec: a valid #GParamSpec instance
 * 
 * Cast a #GParamSpec instance into a #GParamSpecChar.
 */ */

/* G_PARAM_SPEC_CHAR ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_CHAR , GParamSpecChar ) ) /**
 * G_TYPE_PARAM_UCHAR:
 * 
 * The #GType of #GParamSpecUChar.
 */ */

/* G_TYPE_PARAM_UCHAR ( g_param_spec_types [ 1 ] ) /**
 * G_IS_PARAM_SPEC_UCHAR:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_UCHAR.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_UCHAR ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_UCHAR ) ) /**
 * G_PARAM_SPEC_UCHAR:
 * @pspec: a valid #GParamSpec instance
 * 
 * Cast a #GParamSpec instance into a #GParamSpecUChar.
 */ */

/* G_PARAM_SPEC_UCHAR ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_UCHAR , GParamSpecUChar ) ) /**
 * G_TYPE_PARAM_BOOLEAN:
 * 
 * The #GType of #GParamSpecBoolean.
 */ */

/* G_TYPE_PARAM_BOOLEAN ( g_param_spec_types [ 2 ] ) /**
 * G_IS_PARAM_SPEC_BOOLEAN:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_BOOLEAN.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_BOOLEAN ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_BOOLEAN ) ) /**
 * G_PARAM_SPEC_BOOLEAN:
 * @pspec: a valid #GParamSpec instance
 * 
 * Cast a #GParamSpec instance into a #GParamSpecBoolean.
 */ */

/* G_PARAM_SPEC_BOOLEAN ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_BOOLEAN , GParamSpecBoolean ) ) /**
 * G_TYPE_PARAM_INT:
 * 
 * The #GType of #GParamSpecInt.
 */ */

/* G_TYPE_PARAM_INT ( g_param_spec_types [ 3 ] ) /**
 * G_IS_PARAM_SPEC_INT:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_INT.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_INT ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_INT ) ) /**
 * G_PARAM_SPEC_INT:
 * @pspec: a valid #GParamSpec instance
 * 
 * Cast a #GParamSpec instance into a #GParamSpecInt.
 */ */

/* G_PARAM_SPEC_INT ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_INT , GParamSpecInt ) ) /**
 * G_TYPE_PARAM_UINT:
 * 
 * The #GType of #GParamSpecUInt.
 */ */

/* G_TYPE_PARAM_UINT ( g_param_spec_types [ 4 ] ) /**
 * G_IS_PARAM_SPEC_UINT:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_UINT.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_UINT ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_UINT ) ) /**
 * G_PARAM_SPEC_UINT:
 * @pspec: a valid #GParamSpec instance
 * 
 * Cast a #GParamSpec instance into a #GParamSpecUInt.
 */ */

/* G_PARAM_SPEC_UINT ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_UINT , GParamSpecUInt ) ) /**
 * G_TYPE_PARAM_LONG:
 * 
 * The #GType of #GParamSpecLong.
 */ */

/* G_TYPE_PARAM_LONG ( g_param_spec_types [ 5 ] ) /**
 * G_IS_PARAM_SPEC_LONG:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_LONG.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_LONG ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_LONG ) ) /**
 * G_PARAM_SPEC_LONG:
 * @pspec: a valid #GParamSpec instance
 * 
 * Cast a #GParamSpec instance into a #GParamSpecLong.
 */ */

/* G_PARAM_SPEC_LONG ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_LONG , GParamSpecLong ) ) /**
 * G_TYPE_PARAM_ULONG:
 * 
 * The #GType of #GParamSpecULong.
 */ */

/* G_TYPE_PARAM_ULONG ( g_param_spec_types [ 6 ] ) /**
 * G_IS_PARAM_SPEC_ULONG:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_ULONG.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_ULONG ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_ULONG ) ) /**
 * G_PARAM_SPEC_ULONG:
 * @pspec: a valid #GParamSpec instance
 * 
 * Cast a #GParamSpec instance into a #GParamSpecULong.
 */ */

/* G_PARAM_SPEC_ULONG ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_ULONG , GParamSpecULong ) ) /**
 * G_TYPE_PARAM_INT64:
 * 
 * The #GType of #GParamSpecInt64.
 */ */

/* G_TYPE_PARAM_INT64 ( g_param_spec_types [ 7 ] ) /**
 * G_IS_PARAM_SPEC_INT64:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_INT64.
 *
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_INT64 ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_INT64 ) ) /**
 * G_PARAM_SPEC_INT64:
 * @pspec: a valid #GParamSpec instance
 * 
 * Cast a #GParamSpec instance into a #GParamSpecInt64.
 */ */

/* G_PARAM_SPEC_INT64 ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_INT64 , GParamSpecInt64 ) ) /**
 * G_TYPE_PARAM_UINT64:
 * 
 * The #GType of #GParamSpecUInt64.
 */ */

/* G_TYPE_PARAM_UINT64 ( g_param_spec_types [ 8 ] ) /**
 * G_IS_PARAM_SPEC_UINT64:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_UINT64.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_UINT64 ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_UINT64 ) ) /**
 * G_PARAM_SPEC_UINT64:
 * @pspec: a valid #GParamSpec instance
 * 
 * Cast a #GParamSpec instance into a #GParamSpecUInt64.
 */ */

/* G_PARAM_SPEC_UINT64 ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_UINT64 , GParamSpecUInt64 ) ) /**
 * G_TYPE_PARAM_UNICHAR:
 * 
 * The #GType of #GParamSpecUnichar.
 */ */

/* G_TYPE_PARAM_UNICHAR ( g_param_spec_types [ 9 ] ) /**
 * G_PARAM_SPEC_UNICHAR:
 * @pspec: a valid #GParamSpec instance
 * 
 * Cast a #GParamSpec instance into a #GParamSpecUnichar.
 */ */

/* G_PARAM_SPEC_UNICHAR ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_UNICHAR , GParamSpecUnichar ) ) /**
 * G_IS_PARAM_SPEC_UNICHAR:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_UNICHAR.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_UNICHAR ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_UNICHAR ) ) /**
 * G_TYPE_PARAM_ENUM:
 * 
 * The #GType of #GParamSpecEnum.
 */ */

/* G_TYPE_PARAM_ENUM ( g_param_spec_types [ 10 ] ) /**
 * G_IS_PARAM_SPEC_ENUM:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_ENUM.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_ENUM ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_ENUM ) ) /**
 * G_PARAM_SPEC_ENUM:
 * @pspec: a valid #GParamSpec instance
 * 
 * Cast a #GParamSpec instance into a #GParamSpecEnum.
 */ */

/* G_PARAM_SPEC_ENUM ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_ENUM , GParamSpecEnum ) ) /**
 * G_TYPE_PARAM_FLAGS:
 * 
 * The #GType of #GParamSpecFlags.
 */ */

/* G_TYPE_PARAM_FLAGS ( g_param_spec_types [ 11 ] ) /**
 * G_IS_PARAM_SPEC_FLAGS:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_FLAGS.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_FLAGS ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_FLAGS ) ) /**
 * G_PARAM_SPEC_FLAGS:
 * @pspec: a valid #GParamSpec instance
 * 
 * Cast a #GParamSpec instance into a #GParamSpecFlags.
 */ */

/* G_PARAM_SPEC_FLAGS ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_FLAGS , GParamSpecFlags ) ) /**
 * G_TYPE_PARAM_FLOAT:
 * 
 * The #GType of #GParamSpecFloat.
 */ */

/* G_TYPE_PARAM_FLOAT ( g_param_spec_types [ 12 ] ) /**
 * G_IS_PARAM_SPEC_FLOAT:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_FLOAT.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_FLOAT ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_FLOAT ) ) /**
 * G_PARAM_SPEC_FLOAT:
 * @pspec: a valid #GParamSpec instance
 * 
 * Cast a #GParamSpec instance into a #GParamSpecFloat.
 */ */

/* G_PARAM_SPEC_FLOAT ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_FLOAT , GParamSpecFloat ) ) /**
 * G_TYPE_PARAM_DOUBLE:
 * 
 * The #GType of #GParamSpecDouble.
 */ */

/* G_TYPE_PARAM_DOUBLE ( g_param_spec_types [ 13 ] ) /**
 * G_IS_PARAM_SPEC_DOUBLE:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_DOUBLE.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_DOUBLE ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_DOUBLE ) ) /**
 * G_PARAM_SPEC_DOUBLE:
 * @pspec: a valid #GParamSpec instance
 * 
 * Cast a #GParamSpec instance into a #GParamSpecDouble.
 */ */

/* G_PARAM_SPEC_DOUBLE ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_DOUBLE , GParamSpecDouble ) ) /**
 * G_TYPE_PARAM_STRING:
 * 
 * The #GType of #GParamSpecString.
 */ */

/* G_TYPE_PARAM_STRING ( g_param_spec_types [ 14 ] ) /**
 * G_IS_PARAM_SPEC_STRING:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_STRING.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_STRING ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_STRING ) ) /**
 * G_PARAM_SPEC_STRING:
 * @pspec: a valid #GParamSpec instance
 * 
 * Casts a #GParamSpec instance into a #GParamSpecString.
 */ */

/* G_PARAM_SPEC_STRING ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_STRING , GParamSpecString ) ) /**
 * G_TYPE_PARAM_PARAM:
 * 
 * The #GType of #GParamSpecParam.
 */ */

/* G_TYPE_PARAM_PARAM ( g_param_spec_types [ 15 ] ) /**
 * G_IS_PARAM_SPEC_PARAM:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_PARAM.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_PARAM ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_PARAM ) ) /**
 * G_PARAM_SPEC_PARAM:
 * @pspec: a valid #GParamSpec instance
 * 
 * Casts a #GParamSpec instance into a #GParamSpecParam.
 */ */

/* G_PARAM_SPEC_PARAM ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_PARAM , GParamSpecParam ) ) /**
 * G_TYPE_PARAM_BOXED:
 * 
 * The #GType of #GParamSpecBoxed.
 */ */

/* G_TYPE_PARAM_BOXED ( g_param_spec_types [ 16 ] ) /**
 * G_IS_PARAM_SPEC_BOXED:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_BOXED.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_BOXED ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_BOXED ) ) /**
 * G_PARAM_SPEC_BOXED:
 * @pspec: a valid #GParamSpec instance
 * 
 * Cast a #GParamSpec instance into a #GParamSpecBoxed.
 */ */

/* G_PARAM_SPEC_BOXED ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_BOXED , GParamSpecBoxed ) ) /**
 * G_TYPE_PARAM_POINTER:
 * 
 * The #GType of #GParamSpecPointer.
 */ */

/* G_TYPE_PARAM_POINTER ( g_param_spec_types [ 17 ] ) /**
 * G_IS_PARAM_SPEC_POINTER:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_POINTER.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_POINTER ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_POINTER ) ) /**
 * G_PARAM_SPEC_POINTER:
 * @pspec: a valid #GParamSpec instance
 * 
 * Casts a #GParamSpec instance into a #GParamSpecPointer.
 */ */

/* G_PARAM_SPEC_POINTER ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_POINTER , GParamSpecPointer ) ) /**
 * G_TYPE_PARAM_VALUE_ARRAY:
 * 
 * The #GType of #GParamSpecValueArray.
 *
 * Deprecated: 2.32: Use #GArray instead of #GValueArray
 */ */

/* G_TYPE_PARAM_VALUE_ARRAY ( g_param_spec_types [ 18 ] ) /**
 * G_IS_PARAM_SPEC_VALUE_ARRAY:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_VALUE_ARRAY.
 * 
 * Returns: %TRUE on success.
 *
 * Deprecated: 2.32: Use #GArray instead of #GValueArray
 */ */

/* G_IS_PARAM_SPEC_VALUE_ARRAY ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_VALUE_ARRAY ) ) /**
 * G_PARAM_SPEC_VALUE_ARRAY:
 * @pspec: a valid #GParamSpec instance
 * 
 * Cast a #GParamSpec instance into a #GParamSpecValueArray.
 *
 * Deprecated: 2.32: Use #GArray instead of #GValueArray
 */ */

/* G_PARAM_SPEC_VALUE_ARRAY ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_VALUE_ARRAY , GParamSpecValueArray ) ) /**
 * G_TYPE_PARAM_OBJECT:
 * 
 * The #GType of #GParamSpecObject.
 */ */

/* G_TYPE_PARAM_OBJECT ( g_param_spec_types [ 19 ] ) /**
 * G_IS_PARAM_SPEC_OBJECT:
 * @pspec: a valid #GParamSpec instance
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_OBJECT.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_OBJECT ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_OBJECT ) ) /**
 * G_PARAM_SPEC_OBJECT:
 * @pspec: a valid #GParamSpec instance
 * 
 * Casts a #GParamSpec instance into a #GParamSpecObject.
 */ */

/* G_PARAM_SPEC_OBJECT ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_OBJECT , GParamSpecObject ) ) /**
 * G_TYPE_PARAM_OVERRIDE:
 * 
 * The #GType of #GParamSpecOverride.
 * 
 * Since: 2.4
 */ */

/* G_TYPE_PARAM_OVERRIDE ( g_param_spec_types [ 20 ] ) /**
 * G_IS_PARAM_SPEC_OVERRIDE:
 * @pspec: a #GParamSpec
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_OVERRIDE.
 * 
 * Since: 2.4
 * Returns: %TRUE on success.
 */ */

/* G_IS_PARAM_SPEC_OVERRIDE ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_OVERRIDE ) ) /**
 * G_PARAM_SPEC_OVERRIDE:
 * @pspec: a #GParamSpec
 * 
 * Casts a #GParamSpec into a #GParamSpecOverride.
 * 
 * Since: 2.4
 */ */

/* G_PARAM_SPEC_OVERRIDE ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_OVERRIDE , GParamSpecOverride ) ) /**
 * G_TYPE_PARAM_GTYPE:
 * 
 * The #GType of #GParamSpecGType.
 * 
 * Since: 2.10
 */ */

/* G_TYPE_PARAM_GTYPE ( g_param_spec_types [ 21 ] ) /**
 * G_IS_PARAM_SPEC_GTYPE:
 * @pspec: a #GParamSpec
 * 
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_GTYPE.
 * 
 * Since: 2.10
 * Returns: %TRUE on success. 
 */ */

/* G_IS_PARAM_SPEC_GTYPE ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_GTYPE ) ) /**
 * G_PARAM_SPEC_GTYPE:
 * @pspec: a #GParamSpec
 * 
 * Casts a #GParamSpec into a #GParamSpecGType.
 * 
 * Since: 2.10
 */ */

/* G_PARAM_SPEC_GTYPE ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_GTYPE , GParamSpecGType ) ) /**
 * G_TYPE_PARAM_VARIANT:
 *
 * The #GType of #GParamSpecVariant.
 *
 * Since: 2.26
 */ */

/* G_TYPE_PARAM_VARIANT ( g_param_spec_types [ 22 ] ) /**
 * G_IS_PARAM_SPEC_VARIANT:
 * @pspec: a #GParamSpec
 *
 * Checks whether the given #GParamSpec is of type %G_TYPE_PARAM_VARIANT.
 *
 * Returns: %TRUE on success
 *
 * Since: 2.26
 */ */

/* G_IS_PARAM_SPEC_VARIANT ( pspec ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( pspec ) , G_TYPE_PARAM_VARIANT ) ) /**
 * G_PARAM_SPEC_VARIANT:
 * @pspec: a #GParamSpec
 *
 * Casts a #GParamSpec into a #GParamSpecVariant.
 *
 * Since: 2.26
 */ */

/* G_PARAM_SPEC_VARIANT ( pspec ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( pspec ) , G_TYPE_PARAM_VARIANT , GParamSpecVariant ) ) /* --- typedefs & structures --- */ */

/* GOBJECT_VAR _GLIB_EXTERN # */

/* __G_SOURCECLOSURE_H__ # */

/* __G_TYPE_MODULE_H__ # */

/* G_TYPE_TYPE_MODULE ( g_type_module_get_type ( ) ) # */

/* G_TYPE_MODULE ( module ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( module ) , G_TYPE_TYPE_MODULE , GTypeModule ) ) # */

/* G_TYPE_MODULE_CLASS ( class ) ( G_TYPE_CHECK_CLASS_CAST ( ( class ) , G_TYPE_TYPE_MODULE , GTypeModuleClass ) ) # */

/* G_IS_TYPE_MODULE ( module ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( module ) , G_TYPE_TYPE_MODULE ) ) # */

/* G_IS_TYPE_MODULE_CLASS ( class ) ( G_TYPE_CHECK_CLASS_TYPE ( ( class ) , G_TYPE_TYPE_MODULE ) ) # */

/* G_TYPE_MODULE_GET_CLASS ( module ) ( G_TYPE_INSTANCE_GET_CLASS ( ( module ) , G_TYPE_TYPE_MODULE , GTypeModuleClass ) ) /**
 * GTypeModule:
 * @name: the name of the module
 * 
 * The members of the GTypeModule structure should not 
 * be accessed directly, except for the @name field.
 */ */

/* G_DEFINE_DYNAMIC_TYPE ( TN , t_n , T_P ) G_DEFINE_DYNAMIC_TYPE_EXTENDED ( TN , t_n , T_P , 0 , { } ) /**
 * G_DEFINE_DYNAMIC_TYPE_EXTENDED:
 * @TypeName: The name of the new type, in Camel case.
 * @type_name: The name of the new type, in lowercase, with words
 *  separated by '_'.
 * @TYPE_PARENT: The #GType of the parent type.
 * @flags: #GTypeFlags to pass to g_type_module_register_type()
 * @CODE: Custom code that gets inserted in the *_get_type() function.
 * 
 * A more general version of G_DEFINE_DYNAMIC_TYPE() which
 * allows to specify #GTypeFlags and custom code.
 * 
 * |[
 * G_DEFINE_DYNAMIC_TYPE_EXTENDED (GtkGadget,
 *                                 gtk_gadget,
 *                                 GTK_TYPE_THING,
 *                                 0,
 *                                 G_IMPLEMENT_INTERFACE_DYNAMIC (TYPE_GIZMO,
 *                                                                gtk_gadget_gizmo_init));
 * ]|
 * expands to
 * |[
 * static void     gtk_gadget_init              (GtkGadget      *self);
 * static void     gtk_gadget_class_init        (GtkGadgetClass *klass);
 * static void     gtk_gadget_class_finalize    (GtkGadgetClass *klass);
 * 
 * static gpointer gtk_gadget_parent_class = NULL;
 * static GType    gtk_gadget_type_id = 0;
 * 
 * static void     gtk_gadget_class_intern_init (gpointer klass)
 * {
 *   gtk_gadget_parent_class = g_type_class_peek_parent (klass); 
 *   gtk_gadget_class_init ((GtkGadgetClass*) klass); 
 * }
 * 
 * GType
 * gtk_gadget_get_type (void)
 * {
 *   return gtk_gadget_type_id;
 * }
 * 
 * static void
 * gtk_gadget_register_type (GTypeModule *type_module)
 * {
 *   const GTypeInfo g_define_type_info = {
 *     sizeof (GtkGadgetClass),
 *     (GBaseInitFunc) NULL,
 *     (GBaseFinalizeFunc) NULL,
 *     (GClassInitFunc) gtk_gadget_class_intern_init,
 *     (GClassFinalizeFunc) gtk_gadget_class_finalize,
 *     NULL,   // class_data
 *     sizeof (GtkGadget),
 *     0,      // n_preallocs
 *     (GInstanceInitFunc) gtk_gadget_init, 
 *     NULL    // value_table
 *   };
 *   gtk_gadget_type_id = g_type_module_register_type (type_module,
 *                                                     GTK_TYPE_THING,
 *                                                     "GtkGadget",
 *                                                     &g_define_type_info,
 *                                                     (GTypeFlags) flags);
 *   {
 *     const GInterfaceInfo g_implement_interface_info = {
 *       (GInterfaceInitFunc) gtk_gadget_gizmo_init
 *     };
 *     g_type_module_add_interface (type_module, g_define_type_id, TYPE_GIZMO, &g_implement_interface_info);
 *   }
 * }
 * ]|
 * 
 * Since: 2.14
 */ */

/* G_DEFINE_DYNAMIC_TYPE_EXTENDED ( TypeName , type_name , TYPE_PARENT , flags , CODE ) static void type_name ## _init ( TypeName * self ) ; static void type_name ## _class_init ( TypeName ## Class * klass ) ; static void type_name ## _class_finalize ( TypeName ## Class * klass ) ; static gpointer type_name ## _parent_class = NULL ; static GType type_name ## _type_id = 0 ; static gint TypeName ## _private_offset ; _G_DEFINE_TYPE_EXTENDED_CLASS_INIT ( TypeName , type_name ) static inline gpointer type_name ## _get_instance_private ( TypeName * self ) \
{ return ( G_STRUCT_MEMBER_P ( self , TypeName ## _private_offset ) ) ; \
} GType type_name ## _get_type ( void ) \
{ return type_name ## _type_id ; \
} static void type_name ## _register_type ( GTypeModule * type_module ) \
{ GType g_define_type_id G_GNUC_UNUSED ; const GTypeInfo g_define_type_info = { sizeof ( TypeName ## Class ) , ( GBaseInitFunc ) NULL , ( GBaseFinalizeFunc ) NULL , ( GClassInitFunc ) type_name ## _class_intern_init , ( GClassFinalizeFunc ) type_name ## _class_finalize , NULL , /* class_data */ sizeof ( TypeName ) , 0 , /* n_preallocs */ ( GInstanceInitFunc ) type_name ## _init , NULL /* value_table */ } ; type_name ## _type_id = g_type_module_register_type ( type_module , TYPE_PARENT , # TypeName , & g_define_type_info , ( GTypeFlags ) flags ) ; g_define_type_id = type_name ## _type_id ; { CODE ; } \
} /**
 * G_IMPLEMENT_INTERFACE_DYNAMIC:
 * @TYPE_IFACE: The #GType of the interface to add
 * @iface_init: The interface init function
 *
 * A convenience macro to ease interface addition in the @_C_ section
 * of G_DEFINE_DYNAMIC_TYPE_EXTENDED(). See G_DEFINE_DYNAMIC_TYPE_EXTENDED()
 * for an example.
 *
 * Note that this macro can only be used together with the
 * G_DEFINE_DYNAMIC_TYPE_EXTENDED macros, since it depends on variable
 * names from that macro.
 *
 * Since: 2.24
 */ */

/* G_IMPLEMENT_INTERFACE_DYNAMIC ( TYPE_IFACE , iface_init ) { const GInterfaceInfo g_implement_interface_info = { ( GInterfaceInitFunc ) iface_init , NULL , NULL } ; g_type_module_add_interface ( type_module , g_define_type_id , TYPE_IFACE , & g_implement_interface_info ) ; \
} # */

/* G_ADD_PRIVATE_DYNAMIC ( TypeName ) { TypeName ## _private_offset = sizeof ( TypeName ## Private ) ; \
} GLIB_AVAILABLE_IN_ALL */

/* __G_TYPE_PLUGIN_H__ # */

/* G_TYPE_TYPE_PLUGIN ( g_type_plugin_get_type ( ) ) # */

/* G_TYPE_PLUGIN ( inst ) ( G_TYPE_CHECK_INSTANCE_CAST ( ( inst ) , G_TYPE_TYPE_PLUGIN , GTypePlugin ) ) # */

/* G_TYPE_PLUGIN_CLASS ( vtable ) ( G_TYPE_CHECK_CLASS_CAST ( ( vtable ) , G_TYPE_TYPE_PLUGIN , GTypePluginClass ) ) # */

/* G_IS_TYPE_PLUGIN ( inst ) ( G_TYPE_CHECK_INSTANCE_TYPE ( ( inst ) , G_TYPE_TYPE_PLUGIN ) ) # */

/* G_IS_TYPE_PLUGIN_CLASS ( vtable ) ( G_TYPE_CHECK_CLASS_TYPE ( ( vtable ) , G_TYPE_TYPE_PLUGIN ) ) # */

/* G_TYPE_PLUGIN_GET_CLASS ( inst ) ( G_TYPE_INSTANCE_GET_INTERFACE ( ( inst ) , G_TYPE_TYPE_PLUGIN , GTypePluginClass ) ) /* --- typedefs & structures --- */ */

/* __G_VALUE_ARRAY_H__ # */

/* G_TYPE_VALUE_ARRAY ( g_value_array_get_type ( ) ) /* --- typedefs & structs --- */ */

/* __G_VALUETYPES_H__ # */

/* G_VALUE_HOLDS_CHAR ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_CHAR ) ) /**
 * G_VALUE_HOLDS_UCHAR:
 * @value: a valid #GValue structure
 * 
 * Checks whether the given #GValue can hold values of type %G_TYPE_UCHAR.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_VALUE_HOLDS_UCHAR ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_UCHAR ) ) /**
 * G_VALUE_HOLDS_BOOLEAN:
 * @value: a valid #GValue structure
 * 
 * Checks whether the given #GValue can hold values of type %G_TYPE_BOOLEAN.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_VALUE_HOLDS_BOOLEAN ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_BOOLEAN ) ) /**
 * G_VALUE_HOLDS_INT:
 * @value: a valid #GValue structure
 * 
 * Checks whether the given #GValue can hold values of type %G_TYPE_INT.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_VALUE_HOLDS_INT ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_INT ) ) /**
 * G_VALUE_HOLDS_UINT:
 * @value: a valid #GValue structure
 * 
 * Checks whether the given #GValue can hold values of type %G_TYPE_UINT.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_VALUE_HOLDS_UINT ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_UINT ) ) /**
 * G_VALUE_HOLDS_LONG:
 * @value: a valid #GValue structure
 * 
 * Checks whether the given #GValue can hold values of type %G_TYPE_LONG.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_VALUE_HOLDS_LONG ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_LONG ) ) /**
 * G_VALUE_HOLDS_ULONG:
 * @value: a valid #GValue structure
 * 
 * Checks whether the given #GValue can hold values of type %G_TYPE_ULONG.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_VALUE_HOLDS_ULONG ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_ULONG ) ) /**
 * G_VALUE_HOLDS_INT64:
 * @value: a valid #GValue structure
 * 
 * Checks whether the given #GValue can hold values of type %G_TYPE_INT64.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_VALUE_HOLDS_INT64 ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_INT64 ) ) /**
 * G_VALUE_HOLDS_UINT64:
 * @value: a valid #GValue structure
 * 
 * Checks whether the given #GValue can hold values of type %G_TYPE_UINT64.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_VALUE_HOLDS_UINT64 ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_UINT64 ) ) /**
 * G_VALUE_HOLDS_FLOAT:
 * @value: a valid #GValue structure
 * 
 * Checks whether the given #GValue can hold values of type %G_TYPE_FLOAT.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_VALUE_HOLDS_FLOAT ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_FLOAT ) ) /**
 * G_VALUE_HOLDS_DOUBLE:
 * @value: a valid #GValue structure
 * 
 * Checks whether the given #GValue can hold values of type %G_TYPE_DOUBLE.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_VALUE_HOLDS_DOUBLE ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_DOUBLE ) ) /**
 * G_VALUE_HOLDS_STRING:
 * @value: a valid #GValue structure
 * 
 * Checks whether the given #GValue can hold values of type %G_TYPE_STRING.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_VALUE_HOLDS_STRING ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_STRING ) ) /**
 * G_VALUE_HOLDS_POINTER:
 * @value: a valid #GValue structure
 * 
 * Checks whether the given #GValue can hold values of type %G_TYPE_POINTER.
 * 
 * Returns: %TRUE on success.
 */ */

/* G_VALUE_HOLDS_POINTER ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_POINTER ) ) /**
 * G_TYPE_GTYPE:
 * 
 * The type for #GType.
 */ */

/* G_TYPE_GTYPE ( g_gtype_get_type ( ) ) /**
 * G_VALUE_HOLDS_GTYPE:
 * @value: a valid #GValue structure
 * 
 * Checks whether the given #GValue can hold values of type %G_TYPE_GTYPE.
 * 
 * Since: 2.12
 * Returns: %TRUE on success.
 */ */

/* G_VALUE_HOLDS_GTYPE ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_GTYPE ) ) /**
 * G_VALUE_HOLDS_VARIANT:
 * @value: a valid #GValue structure
 *
 * Checks whether the given #GValue can hold values of type %G_TYPE_VARIANT.
 *
 * Returns: %TRUE on success.
 *
 * Since: 2.26
 */ */

/* G_VALUE_HOLDS_VARIANT ( value ) ( G_TYPE_CHECK_VALUE_TYPE ( ( value ) , G_TYPE_VARIANT ) ) /* --- prototypes --- */ */

