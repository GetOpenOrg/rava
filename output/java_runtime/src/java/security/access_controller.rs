#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/security/AccessController",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "final",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = true,
    source            = "AccessController.java",
    inner_classes     = "java/security/AccessController$AccHolder:java/security/AccessController:AccHolder:10",
    all_supertypes    = "java/lang/Object;java/security/AccessController",
)]
#[derive(Clone, Default, PartialEq)]
pub struct AccessController;

impl AccessController {
    #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
    // static field: $assertionsDisabled:Z
    pub fn _assertionsDisabled() -> bool {
        false
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        panic!("stub: java/security/AccessController.<init>:()V")
    }

    #[cfg_attr(any(), java_method(name = "doPrivileged", descriptor = "(Ljava/security/PrivilegedAction;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/security/PrivilegedAction<TT;>;)TT;"))]
    pub fn doPrivileged_privil(action: Object) -> Result<Object> {
        panic!("stub: java/security/AccessController.doPrivileged:(Ljava/security/PrivilegedAction;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "doPrivilegedWithCombiner", descriptor = "(Ljava/security/PrivilegedAction;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/security/PrivilegedAction<TT;>;)TT;"))]
    pub fn doPrivilegedWithCombiner_privil(action: Object) -> Result<Object> {
        panic!("stub: java/security/AccessController.doPrivilegedWithCombiner:(Ljava/security/PrivilegedAction;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "doPrivileged", descriptor = "(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/security/PrivilegedAction<TT;>;Ljava/security/AccessControlContext;)TT;"))]
    pub fn doPrivileged_privil_access(action: Object, context: Object) -> Result<Object> {
        panic!("stub: java/security/AccessController.doPrivileged:(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "doPrivileged", descriptor = "(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;[Ljava/security/Permission;)Ljava/lang/Object;", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/security/PrivilegedAction<TT;>;Ljava/security/AccessControlContext;[Ljava/security/Permission;)TT;"))]
    pub fn doPrivileged_privil_access_arr_per(action: Object, context: Object, perms: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
        panic!("stub: java/security/AccessController.doPrivileged:(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;[Ljava/security/Permission;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "doPrivilegedWithCombiner", descriptor = "(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;[Ljava/security/Permission;)Ljava/lang/Object;", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/security/PrivilegedAction<TT;>;Ljava/security/AccessControlContext;[Ljava/security/Permission;)TT;"))]
    pub fn doPrivilegedWithCombiner_privil_access_arr_per(action: Object, context: Object, perms: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
        panic!("stub: java/security/AccessController.doPrivilegedWithCombiner:(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;[Ljava/security/Permission;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "doPrivileged", descriptor = "(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/security/PrivilegedActionException", generic_signature = "<T:Ljava/lang/Object;>(Ljava/security/PrivilegedExceptionAction<TT;>;)TT;"))]
    pub fn doPrivileged_privil_1(action: Object) -> Result<Object> {
        panic!("stub: java/security/AccessController.doPrivileged:(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "doPrivilegedWithCombiner", descriptor = "(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/security/PrivilegedActionException", generic_signature = "<T:Ljava/lang/Object;>(Ljava/security/PrivilegedExceptionAction<TT;>;)TT;"))]
    pub fn doPrivilegedWithCombiner_privil_1(action: Object) -> Result<Object> {
        panic!("stub: java/security/AccessController.doPrivilegedWithCombiner:(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "preserveCombiner", descriptor = "(Ljava/security/DomainCombiner;Ljava/lang/Class;)Ljava/security/AccessControlContext;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/security/DomainCombiner;Ljava/lang/Class<*>;)Ljava/security/AccessControlContext;"))]
    pub fn preserveCombiner(combiner: Object, caller: Object) -> Result<Object> {
        panic!("stub: java/security/AccessController.preserveCombiner:(Ljava/security/DomainCombiner;Ljava/lang/Class;)Ljava/security/AccessControlContext;")
    }

    #[cfg_attr(any(), java_method(name = "createWrapper", descriptor = "(Ljava/security/DomainCombiner;Ljava/lang/Class;Ljava/security/AccessControlContext;Ljava/security/AccessControlContext;[Ljava/security/Permission;)Ljava/security/AccessControlContext;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/security/DomainCombiner;Ljava/lang/Class<*>;Ljava/security/AccessControlContext;Ljava/security/AccessControlContext;[Ljava/security/Permission;)Ljava/security/AccessControlContext;"))]
    pub fn createWrapper(combiner: Object, caller: Object, parent: Object, context: Object, perms: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
        panic!("stub: java/security/AccessController.createWrapper:(Ljava/security/DomainCombiner;Ljava/lang/Class;Ljava/security/AccessControlContext;Ljava/security/AccessControlContext;[Ljava/security/Permission;)Ljava/security/AccessControlContext;")
    }

    #[cfg_attr(any(), java_method(name = "getInnocuousAcc", descriptor = "()Ljava/security/AccessControlContext;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getInnocuousAcc() -> Result<Object> {
        panic!("stub: java/security/AccessController.getInnocuousAcc:()Ljava/security/AccessControlContext;")
    }

    #[cfg_attr(any(), java_native(name = "getProtectionDomain", descriptor = "(Ljava/lang/Class;)Ljava/security/ProtectionDomain;", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Ljava/security/ProtectionDomain;"))]
    pub fn getProtectionDomain(arg0: Object) -> Result<Object> {
        panic!("native: java/security/AccessController.getProtectionDomain:(Ljava/lang/Class;)Ljava/security/ProtectionDomain;")
    }

    #[cfg_attr(any(), java_method(name = "doPrivileged", descriptor = "(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/security/PrivilegedActionException", generic_signature = "<T:Ljava/lang/Object;>(Ljava/security/PrivilegedExceptionAction<TT;>;Ljava/security/AccessControlContext;)TT;"))]
    pub fn doPrivileged_privil_access_1(action: Object, context: Object) -> Result<Object> {
        panic!("stub: java/security/AccessController.doPrivileged:(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "checkContext", descriptor = "(Ljava/security/AccessControlContext;Ljava/lang/Class;)Ljava/security/AccessControlContext;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/security/AccessControlContext;Ljava/lang/Class<*>;)Ljava/security/AccessControlContext;"))]
    pub fn checkContext(context: Object, caller: Object) -> Result<Object> {
        panic!("stub: java/security/AccessController.checkContext:(Ljava/security/AccessControlContext;Ljava/lang/Class;)Ljava/security/AccessControlContext;")
    }

    #[cfg_attr(any(), java_native(name = "ensureMaterializedForStackWalk", descriptor = "(Ljava/lang/Object;)V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn ensureMaterializedForStackWalk(arg0: Object) -> Result<()> {
        panic!("native: java/security/AccessController.ensureMaterializedForStackWalk:(Ljava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_method(name = "isPrivileged", descriptor = "()Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isPrivileged() -> Result<bool> {
        panic!("stub: java/security/AccessController.isPrivileged:()Z")
    }

    #[cfg_attr(any(), java_method(name = "executePrivileged", descriptor = "(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;Ljava/lang/Class;)Ljava/lang/Object;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/security/PrivilegedAction<TT;>;Ljava/security/AccessControlContext;Ljava/lang/Class<*>;)TT;"))]
    pub fn executePrivileged_privil_access_class(action: Object, context: Object, caller: Object) -> Result<Object> {
        panic!("stub: java/security/AccessController.executePrivileged:(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;Ljava/lang/Class;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "executePrivileged", descriptor = "(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;Ljava/lang/Class;)Ljava/lang/Object;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/Exception", generic_signature = "<T:Ljava/lang/Object;>(Ljava/security/PrivilegedExceptionAction<TT;>;Ljava/security/AccessControlContext;Ljava/lang/Class<*>;)TT;"))]
    pub fn executePrivileged_privil_access_class_1(action: Object, context: Object, caller: Object) -> Result<Object> {
        panic!("stub: java/security/AccessController.executePrivileged:(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;Ljava/lang/Class;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "wrapException", descriptor = "(Ljava/lang/Exception;)Ljava/security/PrivilegedActionException;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn wrapException(e: Exception) -> Result<Object> {
        panic!("stub: java/security/AccessController.wrapException:(Ljava/lang/Exception;)Ljava/security/PrivilegedActionException;")
    }

    #[cfg_attr(any(), java_method(name = "doPrivileged", descriptor = "(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;[Ljava/security/Permission;)Ljava/lang/Object;", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/security/PrivilegedActionException", generic_signature = "<T:Ljava/lang/Object;>(Ljava/security/PrivilegedExceptionAction<TT;>;Ljava/security/AccessControlContext;[Ljava/security/Permission;)TT;"))]
    pub fn doPrivileged_privil_access_arr_per_1(action: Object, context: Object, perms: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
        panic!("stub: java/security/AccessController.doPrivileged:(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;[Ljava/security/Permission;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "doPrivilegedWithCombiner", descriptor = "(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;[Ljava/security/Permission;)Ljava/lang/Object;", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/security/PrivilegedActionException", generic_signature = "<T:Ljava/lang/Object;>(Ljava/security/PrivilegedExceptionAction<TT;>;Ljava/security/AccessControlContext;[Ljava/security/Permission;)TT;"))]
    pub fn doPrivilegedWithCombiner_privil_access_arr_per_1(action: Object, context: Object, perms: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
        panic!("stub: java/security/AccessController.doPrivilegedWithCombiner:(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;[Ljava/security/Permission;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_native(name = "getStackAccessControlContext", descriptor = "()Ljava/security/AccessControlContext;", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getStackAccessControlContext() -> Result<Object> {
        panic!("native: java/security/AccessController.getStackAccessControlContext:()Ljava/security/AccessControlContext;")
    }

    #[cfg_attr(any(), java_native(name = "getInheritedAccessControlContext", descriptor = "()Ljava/security/AccessControlContext;", access = "package", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getInheritedAccessControlContext() -> Result<Object> {
        panic!("native: java/security/AccessController.getInheritedAccessControlContext:()Ljava/security/AccessControlContext;")
    }

    #[cfg_attr(any(), java_method(name = "getContext", descriptor = "()Ljava/security/AccessControlContext;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getContext() -> Result<Object> {
        panic!("stub: java/security/AccessController.getContext:()Ljava/security/AccessControlContext;")
    }

    #[cfg_attr(any(), java_method(name = "checkPermission", descriptor = "(Ljava/security/Permission;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/security/AccessControlException"))]
    pub fn checkPermission(perm: Object) -> Result<()> {
        panic!("stub: java/security/AccessController.checkPermission:(Ljava/security/Permission;)V")
    }
}
