#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::java::util::function::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/SecurityManager",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = true,
    source            = "SecurityManager.java",
    inner_classes     = "java/lang/SecurityManager$1:::0;java/lang/SecurityManager$2:::0;java/lang/module/ModuleDescriptor$Opens:java/lang/module/ModuleDescriptor:Opens:25;java/lang/module/ModuleDescriptor$Exports:java/lang/module/ModuleDescriptor:Exports:25;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25",
    all_supertypes    = "java/lang/Object;java/lang/SecurityManager",
)]
#[derive(Clone, Default, PartialEq)]
pub struct SecurityManager {
    #[cfg_attr(any(), java_field(name = "initialized", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
    pub initialized: JField<bool>,
}

impl SecurityManager {
    #[cfg_attr(any(), java_field(name = "rootGroup", descriptor = "Ljava/lang/ThreadGroup;", access = "private", modifiers = "static", is_static = true))]
    // static field: rootGroup:Ljava/lang/ThreadGroup;
    pub fn rootGroup() -> Object {
        panic!("stub: java/lang/SecurityManager.rootGroup:Ljava/lang/ThreadGroup;")
    }

    #[cfg_attr(any(), java_field(name = "packageAccessValid", descriptor = "Z", access = "private", modifiers = "static", is_static = true))]
    // static field: packageAccessValid:Z
    pub fn packageAccessValid() -> bool {
        false
    }

    #[cfg_attr(any(), java_field(name = "packageAccess", descriptor = "[Ljava/lang/String;", access = "private", modifiers = "static", is_static = true))]
    // static field: packageAccess:[Ljava/lang/String;
    pub fn packageAccess() -> Rc<RefCell<Vec<String>>> {
        panic!("stub: java/lang/SecurityManager.packageAccess:[Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_field(name = "packageAccessLock", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "static final", is_static = true))]
    // static field: packageAccessLock:Ljava/lang/Object;
    pub fn packageAccessLock() -> Object {
        panic!("stub: java/lang/SecurityManager.packageAccessLock:Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_field(name = "packageDefinitionValid", descriptor = "Z", access = "private", modifiers = "static", is_static = true))]
    // static field: packageDefinitionValid:Z
    pub fn packageDefinitionValid() -> bool {
        false
    }

    #[cfg_attr(any(), java_field(name = "packageDefinition", descriptor = "[Ljava/lang/String;", access = "private", modifiers = "static", is_static = true))]
    // static field: packageDefinition:[Ljava/lang/String;
    pub fn packageDefinition() -> Rc<RefCell<Vec<String>>> {
        panic!("stub: java/lang/SecurityManager.packageDefinition:[Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_field(name = "packageDefinitionLock", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "static final", is_static = true))]
    // static field: packageDefinitionLock:Ljava/lang/Object;
    pub fn packageDefinitionLock() -> Object {
        panic!("stub: java/lang/SecurityManager.packageDefinitionLock:Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_field(name = "nonExportedPkgs", descriptor = "Ljava/util/Map;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Map<Ljava/lang/String;Ljava/lang/Boolean;>;"))]
    // static field: nonExportedPkgs:Ljava/util/Map;
    pub fn nonExportedPkgs_field() -> Object {
        panic!("stub: java/lang/SecurityManager.nonExportedPkgs:Ljava/util/Map;")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        panic!("stub: java/lang/SecurityManager.<init>:()V")
    }

    #[cfg_attr(any(), java_native(name = "getClassContext", descriptor = "()[Ljava/lang/Class;", access = "protected", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;"))]
    pub fn getClassContext(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("native: java/lang/SecurityManager.getClassContext:()[Ljava/lang/Class;")
    }

    #[cfg_attr(any(), java_method(name = "getSecurityContext", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getSecurityContext(&self) -> Result<Object> {
        panic!("stub: java/lang/SecurityManager.getSecurityContext:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "checkPermission", descriptor = "(Ljava/security/Permission;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: checkPermission(Ljava/security/Permission;)V
    pub fn checkPermission_permis(&self, mut perm: Object) -> Result<()> {
        let this = self;
        AccessController::checkPermission(Clone::clone(&perm))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkPermission", descriptor = "(Ljava/security/Permission;Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkPermission_permis_obj(&self, perm: Object, context: Object) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkPermission:(Ljava/security/Permission;Ljava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_method(name = "checkCreateClassLoader", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkCreateClassLoader(&self) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkCreateClassLoader:()V")
    }

    #[cfg_attr(any(), java_method(name = "getRootGroup", descriptor = "()Ljava/lang/ThreadGroup;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getRootGroup() -> Result<Object> {
        panic!("stub: java/lang/SecurityManager.getRootGroup:()Ljava/lang/ThreadGroup;")
    }

    #[cfg_attr(any(), java_method(name = "checkAccess", descriptor = "(Ljava/lang/Thread;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: checkAccess(Ljava/lang/Thread;)V
    pub fn checkAccess_thread(&self, mut t: Thread) -> Result<()> {
        let this = self;
        if _is_jnull(&t) {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        let _t0 = t.getThreadGroup()?;
        if _t0 == SecurityManager::rootGroup() {
            this.checkPermission_permis(Clone::clone(&SecurityConstants::MODIFY_THREAD_PERMISSION()))?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkAccess", descriptor = "(Ljava/lang/ThreadGroup;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkAccess_thread_1(&self, g: Object) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkAccess:(Ljava/lang/ThreadGroup;)V")
    }

    #[cfg_attr(any(), java_method(name = "checkExit", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkExit(&self, status: i32) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkExit:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "checkExec", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkExec(&self, cmd: String) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkExec:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "checkLink", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkLink(&self, lib: String) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkLink:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "checkRead", descriptor = "(Ljava/io/FileDescriptor;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkRead_filede(&self, fd: Object) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkRead:(Ljava/io/FileDescriptor;)V")
    }

    #[cfg_attr(any(), java_method(name = "checkRead", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkRead_str(&self, file: String) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkRead:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "checkRead", descriptor = "(Ljava/lang/String;Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkRead_str_obj(&self, file: String, context: Object) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkRead:(Ljava/lang/String;Ljava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_method(name = "checkWrite", descriptor = "(Ljava/io/FileDescriptor;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkWrite_filede(&self, fd: Object) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkWrite:(Ljava/io/FileDescriptor;)V")
    }

    #[cfg_attr(any(), java_method(name = "checkWrite", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkWrite_str(&self, file: String) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkWrite:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "checkDelete", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkDelete(&self, file: String) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkDelete:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "checkConnect", descriptor = "(Ljava/lang/String;I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkConnect_str_i(&self, host: String, port: i32) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkConnect:(Ljava/lang/String;I)V")
    }

    #[cfg_attr(any(), java_method(name = "checkConnect", descriptor = "(Ljava/lang/String;ILjava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkConnect_str_i_obj(&self, host: String, port: i32, context: Object) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkConnect:(Ljava/lang/String;ILjava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_method(name = "checkListen", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkListen(&self, port: i32) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkListen:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "checkAccept", descriptor = "(Ljava/lang/String;I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkAccept(&self, host: String, port: i32) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkAccept:(Ljava/lang/String;I)V")
    }

    #[cfg_attr(any(), java_method(name = "checkMulticast", descriptor = "(Ljava/net/InetAddress;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkMulticast_inetad(&self, maddr: Object) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkMulticast:(Ljava/net/InetAddress;)V")
    }

    #[cfg_attr(any(), java_method(name = "checkMulticast", descriptor = "(Ljava/net/InetAddress;B)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true))]
    pub fn checkMulticast_inetad_b(&self, maddr: Object, ttl: i8) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkMulticast:(Ljava/net/InetAddress;B)V")
    }

    #[cfg_attr(any(), java_method(name = "checkPropertiesAccess", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkPropertiesAccess(&self) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkPropertiesAccess:()V")
    }

    #[cfg_attr(any(), java_method(name = "checkPropertyAccess", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkPropertyAccess(&self, key: String) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkPropertyAccess:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "checkPrintJobAccess", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkPrintJobAccess(&self) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkPrintJobAccess:()V")
    }

    #[cfg_attr(any(), java_method(name = "getPackages", descriptor = "(Ljava/lang/String;)[Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getPackages(p: String) -> Result<Rc<RefCell<Vec<String>>>> {
        panic!("stub: java/lang/SecurityManager.getPackages:(Ljava/lang/String;)[Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "addNonExportedPackages", descriptor = "(Ljava/lang/ModuleLayer;)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn addNonExportedPackages(layer: Object) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.addNonExportedPackages:(Ljava/lang/ModuleLayer;)V")
    }

    #[cfg_attr(any(), java_method(name = "invalidatePackageAccessCache", descriptor = "()V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn invalidatePackageAccessCache() -> Result<()> {
        panic!("stub: java/lang/SecurityManager.invalidatePackageAccessCache:()V")
    }

    #[cfg_attr(any(), java_method(name = "nonExportedPkgs", descriptor = "(Ljava/lang/module/ModuleDescriptor;)Ljava/util/Set;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/module/ModuleDescriptor;)Ljava/util/Set<Ljava/lang/String;>;"))]
    pub fn nonExportedPkgs(md: Object) -> Result<Object> {
        panic!("stub: java/lang/SecurityManager.nonExportedPkgs:(Ljava/lang/module/ModuleDescriptor;)Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "checkPackageAccess", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkPackageAccess(&self, pkg: String) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkPackageAccess:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "checkPackageDefinition", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkPackageDefinition(&self, pkg: String) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkPackageDefinition:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "checkSetFactory", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkSetFactory(&self) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkSetFactory:()V")
    }

    #[cfg_attr(any(), java_method(name = "checkSecurityAccess", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkSecurityAccess(&self, target: String) -> Result<()> {
        panic!("stub: java/lang/SecurityManager.checkSecurityAccess:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "getThreadGroup", descriptor = "()Ljava/lang/ThreadGroup;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getThreadGroup(&self) -> Result<Object> {
        panic!("stub: java/lang/SecurityManager.getThreadGroup:()Ljava/lang/ThreadGroup;")
    }
}
