#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "sun/security/util/SecurityConstants",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "final",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "SecurityConstants.java",
)]
#[derive(Clone, Default, PartialEq)]
pub struct SecurityConstants;

impl SecurityConstants {
    #[cfg_attr(any(), java_field(name = "FILE_DELETE_ACTION", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "delete"))]
    // static field: FILE_DELETE_ACTION:Ljava/lang/String;
    pub fn FILE_DELETE_ACTION() -> String {
        String::from("delete")
    }

    #[cfg_attr(any(), java_field(name = "FILE_EXECUTE_ACTION", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "execute"))]
    // static field: FILE_EXECUTE_ACTION:Ljava/lang/String;
    pub fn FILE_EXECUTE_ACTION() -> String {
        String::from("execute")
    }

    #[cfg_attr(any(), java_field(name = "FILE_READ_ACTION", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "read"))]
    // static field: FILE_READ_ACTION:Ljava/lang/String;
    pub fn FILE_READ_ACTION() -> String {
        String::from("read")
    }

    #[cfg_attr(any(), java_field(name = "FILE_WRITE_ACTION", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "write"))]
    // static field: FILE_WRITE_ACTION:Ljava/lang/String;
    pub fn FILE_WRITE_ACTION() -> String {
        String::from("write")
    }

    #[cfg_attr(any(), java_field(name = "FILE_READLINK_ACTION", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "readlink"))]
    // static field: FILE_READLINK_ACTION:Ljava/lang/String;
    pub fn FILE_READLINK_ACTION() -> String {
        String::from("readlink")
    }

    #[cfg_attr(any(), java_field(name = "SOCKET_RESOLVE_ACTION", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "resolve"))]
    // static field: SOCKET_RESOLVE_ACTION:Ljava/lang/String;
    pub fn SOCKET_RESOLVE_ACTION() -> String {
        String::from("resolve")
    }

    #[cfg_attr(any(), java_field(name = "SOCKET_CONNECT_ACTION", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "connect"))]
    // static field: SOCKET_CONNECT_ACTION:Ljava/lang/String;
    pub fn SOCKET_CONNECT_ACTION() -> String {
        String::from("connect")
    }

    #[cfg_attr(any(), java_field(name = "SOCKET_LISTEN_ACTION", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "listen"))]
    // static field: SOCKET_LISTEN_ACTION:Ljava/lang/String;
    pub fn SOCKET_LISTEN_ACTION() -> String {
        String::from("listen")
    }

    #[cfg_attr(any(), java_field(name = "SOCKET_ACCEPT_ACTION", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "accept"))]
    // static field: SOCKET_ACCEPT_ACTION:Ljava/lang/String;
    pub fn SOCKET_ACCEPT_ACTION() -> String {
        String::from("accept")
    }

    #[cfg_attr(any(), java_field(name = "SOCKET_CONNECT_ACCEPT_ACTION", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "connect,accept"))]
    // static field: SOCKET_CONNECT_ACCEPT_ACTION:Ljava/lang/String;
    pub fn SOCKET_CONNECT_ACCEPT_ACTION() -> String {
        String::from("connect,accept")
    }

    #[cfg_attr(any(), java_field(name = "PROPERTY_RW_ACTION", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "read,write"))]
    // static field: PROPERTY_RW_ACTION:Ljava/lang/String;
    pub fn PROPERTY_RW_ACTION() -> String {
        String::from("read,write")
    }

    #[cfg_attr(any(), java_field(name = "PROPERTY_READ_ACTION", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "read"))]
    // static field: PROPERTY_READ_ACTION:Ljava/lang/String;
    pub fn PROPERTY_READ_ACTION() -> String {
        String::from("read")
    }

    #[cfg_attr(any(), java_field(name = "PROPERTY_WRITE_ACTION", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "write"))]
    // static field: PROPERTY_WRITE_ACTION:Ljava/lang/String;
    pub fn PROPERTY_WRITE_ACTION() -> String {
        String::from("write")
    }

    #[cfg_attr(any(), java_field(name = "ALL_PERMISSION", descriptor = "Ljava/security/AllPermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: ALL_PERMISSION:Ljava/security/AllPermission;
    pub fn ALL_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.ALL_PERMISSION:Ljava/security/AllPermission;")
    }

    #[cfg_attr(any(), java_field(name = "SPECIFY_HANDLER_PERMISSION", descriptor = "Ljava/net/NetPermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: SPECIFY_HANDLER_PERMISSION:Ljava/net/NetPermission;
    pub fn SPECIFY_HANDLER_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.SPECIFY_HANDLER_PERMISSION:Ljava/net/NetPermission;")
    }

    #[cfg_attr(any(), java_field(name = "SET_PROXYSELECTOR_PERMISSION", descriptor = "Ljava/net/NetPermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: SET_PROXYSELECTOR_PERMISSION:Ljava/net/NetPermission;
    pub fn SET_PROXYSELECTOR_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.SET_PROXYSELECTOR_PERMISSION:Ljava/net/NetPermission;")
    }

    #[cfg_attr(any(), java_field(name = "GET_PROXYSELECTOR_PERMISSION", descriptor = "Ljava/net/NetPermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: GET_PROXYSELECTOR_PERMISSION:Ljava/net/NetPermission;
    pub fn GET_PROXYSELECTOR_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.GET_PROXYSELECTOR_PERMISSION:Ljava/net/NetPermission;")
    }

    #[cfg_attr(any(), java_field(name = "SET_COOKIEHANDLER_PERMISSION", descriptor = "Ljava/net/NetPermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: SET_COOKIEHANDLER_PERMISSION:Ljava/net/NetPermission;
    pub fn SET_COOKIEHANDLER_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.SET_COOKIEHANDLER_PERMISSION:Ljava/net/NetPermission;")
    }

    #[cfg_attr(any(), java_field(name = "GET_COOKIEHANDLER_PERMISSION", descriptor = "Ljava/net/NetPermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: GET_COOKIEHANDLER_PERMISSION:Ljava/net/NetPermission;
    pub fn GET_COOKIEHANDLER_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.GET_COOKIEHANDLER_PERMISSION:Ljava/net/NetPermission;")
    }

    #[cfg_attr(any(), java_field(name = "SET_RESPONSECACHE_PERMISSION", descriptor = "Ljava/net/NetPermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: SET_RESPONSECACHE_PERMISSION:Ljava/net/NetPermission;
    pub fn SET_RESPONSECACHE_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.SET_RESPONSECACHE_PERMISSION:Ljava/net/NetPermission;")
    }

    #[cfg_attr(any(), java_field(name = "GET_RESPONSECACHE_PERMISSION", descriptor = "Ljava/net/NetPermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: GET_RESPONSECACHE_PERMISSION:Ljava/net/NetPermission;
    pub fn GET_RESPONSECACHE_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.GET_RESPONSECACHE_PERMISSION:Ljava/net/NetPermission;")
    }

    #[cfg_attr(any(), java_field(name = "SET_SOCKETIMPL_PERMISSION", descriptor = "Ljava/net/NetPermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: SET_SOCKETIMPL_PERMISSION:Ljava/net/NetPermission;
    pub fn SET_SOCKETIMPL_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.SET_SOCKETIMPL_PERMISSION:Ljava/net/NetPermission;")
    }

    #[cfg_attr(any(), java_field(name = "CREATE_CLASSLOADER_PERMISSION", descriptor = "Ljava/lang/RuntimePermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: CREATE_CLASSLOADER_PERMISSION:Ljava/lang/RuntimePermission;
    pub fn CREATE_CLASSLOADER_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.CREATE_CLASSLOADER_PERMISSION:Ljava/lang/RuntimePermission;")
    }

    #[cfg_attr(any(), java_field(name = "CHECK_MEMBER_ACCESS_PERMISSION", descriptor = "Ljava/lang/RuntimePermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: CHECK_MEMBER_ACCESS_PERMISSION:Ljava/lang/RuntimePermission;
    pub fn CHECK_MEMBER_ACCESS_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.CHECK_MEMBER_ACCESS_PERMISSION:Ljava/lang/RuntimePermission;")
    }

    #[cfg_attr(any(), java_field(name = "MODIFY_THREAD_PERMISSION", descriptor = "Ljava/lang/RuntimePermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: MODIFY_THREAD_PERMISSION:Ljava/lang/RuntimePermission;
    pub fn MODIFY_THREAD_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.MODIFY_THREAD_PERMISSION:Ljava/lang/RuntimePermission;")
    }

    #[cfg_attr(any(), java_field(name = "MODIFY_THREADGROUP_PERMISSION", descriptor = "Ljava/lang/RuntimePermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: MODIFY_THREADGROUP_PERMISSION:Ljava/lang/RuntimePermission;
    pub fn MODIFY_THREADGROUP_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.MODIFY_THREADGROUP_PERMISSION:Ljava/lang/RuntimePermission;")
    }

    #[cfg_attr(any(), java_field(name = "GET_PD_PERMISSION", descriptor = "Ljava/lang/RuntimePermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: GET_PD_PERMISSION:Ljava/lang/RuntimePermission;
    pub fn GET_PD_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.GET_PD_PERMISSION:Ljava/lang/RuntimePermission;")
    }

    #[cfg_attr(any(), java_field(name = "GET_CLASSLOADER_PERMISSION", descriptor = "Ljava/lang/RuntimePermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: GET_CLASSLOADER_PERMISSION:Ljava/lang/RuntimePermission;
    pub fn GET_CLASSLOADER_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.GET_CLASSLOADER_PERMISSION:Ljava/lang/RuntimePermission;")
    }

    #[cfg_attr(any(), java_field(name = "GET_STACK_TRACE_PERMISSION", descriptor = "Ljava/lang/RuntimePermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: GET_STACK_TRACE_PERMISSION:Ljava/lang/RuntimePermission;
    pub fn GET_STACK_TRACE_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.GET_STACK_TRACE_PERMISSION:Ljava/lang/RuntimePermission;")
    }

    #[cfg_attr(any(), java_field(name = "SUBCLASS_IMPLEMENTATION_PERMISSION", descriptor = "Ljava/lang/RuntimePermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: SUBCLASS_IMPLEMENTATION_PERMISSION:Ljava/lang/RuntimePermission;
    pub fn SUBCLASS_IMPLEMENTATION_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.SUBCLASS_IMPLEMENTATION_PERMISSION:Ljava/lang/RuntimePermission;")
    }

    #[cfg_attr(any(), java_field(name = "CREATE_ACC_PERMISSION", descriptor = "Ljava/security/SecurityPermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: CREATE_ACC_PERMISSION:Ljava/security/SecurityPermission;
    pub fn CREATE_ACC_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.CREATE_ACC_PERMISSION:Ljava/security/SecurityPermission;")
    }

    #[cfg_attr(any(), java_field(name = "GET_COMBINER_PERMISSION", descriptor = "Ljava/security/SecurityPermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: GET_COMBINER_PERMISSION:Ljava/security/SecurityPermission;
    pub fn GET_COMBINER_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.GET_COMBINER_PERMISSION:Ljava/security/SecurityPermission;")
    }

    #[cfg_attr(any(), java_field(name = "GET_POLICY_PERMISSION", descriptor = "Ljava/security/SecurityPermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: GET_POLICY_PERMISSION:Ljava/security/SecurityPermission;
    pub fn GET_POLICY_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.GET_POLICY_PERMISSION:Ljava/security/SecurityPermission;")
    }

    #[cfg_attr(any(), java_field(name = "LOCAL_LISTEN_PERMISSION", descriptor = "Ljava/net/SocketPermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: LOCAL_LISTEN_PERMISSION:Ljava/net/SocketPermission;
    pub fn LOCAL_LISTEN_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.LOCAL_LISTEN_PERMISSION:Ljava/net/SocketPermission;")
    }

    #[cfg_attr(any(), java_field(name = "PROVIDER_VER", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true))]
    // static field: PROVIDER_VER:Ljava/lang/String;
    pub fn PROVIDER_VER() -> String {
        panic!("stub: sun/security/util/SecurityConstants.PROVIDER_VER:Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_field(name = "ACCESS_PERMISSION", descriptor = "Ljava/lang/reflect/ReflectPermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: ACCESS_PERMISSION:Ljava/lang/reflect/ReflectPermission;
    pub fn ACCESS_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.ACCESS_PERMISSION:Ljava/lang/reflect/ReflectPermission;")
    }

    #[cfg_attr(any(), java_field(name = "REFLECTION_FACTORY_ACCESS_PERMISSION", descriptor = "Ljava/lang/RuntimePermission;", access = "public", modifiers = "static final", is_static = true))]
    // static field: REFLECTION_FACTORY_ACCESS_PERMISSION:Ljava/lang/RuntimePermission;
    pub fn REFLECTION_FACTORY_ACCESS_PERMISSION() -> Object {
        panic!("stub: sun/security/util/SecurityConstants.REFLECTION_FACTORY_ACCESS_PERMISSION:Ljava/lang/RuntimePermission;")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        panic!("stub: sun/security/util/SecurityConstants.<init>:()V")
    }
}
