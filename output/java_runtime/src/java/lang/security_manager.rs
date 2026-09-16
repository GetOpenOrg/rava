#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::math::*;
use crate::java::nio::*;
use crate::java::nio::charset::*;
use crate::java::security::*;
use crate::java::text::*;
use crate::java::text::spi::*;
use crate::java::time::*;
use crate::java::time::chrono::*;
use crate::java::time::temporal::*;
use crate::java::time::zone::*;
use crate::java::util::*;
use crate::java::util::concurrent::*;
use crate::java::util::concurrent::atomic::*;
use crate::java::util::concurrent::locks::*;
use crate::java::util::function::*;
use crate::java::util::regex::*;
use crate::java::util::spi::*;
use crate::java::util::stream::*;
use crate::java::util::zip::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::reflect::misc::*;
use crate::sun::security::action::*;
use crate::sun::security::util::*;
use crate::sun::text::*;
use crate::sun::util::*;
use crate::sun::util::calendar::*;
use crate::sun::util::locale::*;
use crate::sun::util::locale::provider::*;
use crate::sun::util::spi::*;
use crate::java::text::Normalizer;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/SecurityManager"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = true]
    #[source            = "SecurityManager.java"]
    #[inner_classes     = "java/lang/SecurityManager$1:::0;java/lang/SecurityManager$2:::0;java/lang/module/ModuleDescriptor$Opens:java/lang/module/ModuleDescriptor:Opens:25;java/lang/module/ModuleDescriptor$Exports:java/lang/module/ModuleDescriptor:Exports:25;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/lang/SecurityManager"]

    pub struct SecurityManager {
        #[cfg_attr(any(), java_field(name = "initialized", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub initialized: bool,
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

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/SecurityManager.<init>:()V")
        }

        #[native]
        #[java_native(name = "getClassContext", descriptor = "()[Ljava/lang/Class;", access = "protected", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
        pub fn getClassContext(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("native: java/lang/SecurityManager.getClassContext:()[Ljava/lang/Class;")
        }

        #[java_method(name = "getSecurityContext", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSecurityContext(&self) -> Result<Object> {
            panic!("stub: java/lang/SecurityManager.getSecurityContext:()Ljava/lang/Object;")
        }

        #[java_method(name = "checkPermission", descriptor = "(Ljava/security/Permission;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: checkPermission(Ljava/security/Permission;)V
        pub fn checkPermission_permis(&self, mut perm: Permission) -> Result<()> {
            let this = self;
            AccessController::checkPermission(Clone::clone(&perm))?;
            Ok(())
        }

        #[java_method(name = "checkPermission", descriptor = "(Ljava/security/Permission;Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkPermission_permis_obj(&self, perm: Permission, context: Object) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkPermission:(Ljava/security/Permission;Ljava/lang/Object;)V")
        }

        #[java_method(name = "checkCreateClassLoader", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkCreateClassLoader(&self) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkCreateClassLoader:()V")
        }

        #[java_method(name = "getRootGroup", descriptor = "()Ljava/lang/ThreadGroup;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getRootGroup() -> Result<Object> {
            panic!("stub: java/lang/SecurityManager.getRootGroup:()Ljava/lang/ThreadGroup;")
        }

        #[java_method(name = "checkAccess", descriptor = "(Ljava/lang/Thread;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: checkAccess(Ljava/lang/Thread;)V
        pub fn checkAccess_thread(&self, mut t: Thread) -> Result<()> {
            let this = self;
            if _is_jnull(&t) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0 = t.getThreadGroup()?;
            if _t0 == SecurityManager::rootGroup() {
                this.checkPermission_permis(Clone::clone(&SecurityConstants::MODIFY_THREAD_PERMISSION()).into())?;
            }
            Ok(())
        }

        #[java_method(name = "checkAccess", descriptor = "(Ljava/lang/ThreadGroup;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkAccess_thread_1(&self, g: Object) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkAccess:(Ljava/lang/ThreadGroup;)V")
        }

        #[java_method(name = "checkExit", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkExit(&self, status: i32) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkExit:(I)V")
        }

        #[java_method(name = "checkExec", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkExec(&self, cmd: String) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkExec:(Ljava/lang/String;)V")
        }

        #[java_method(name = "checkLink", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkLink(&self, lib: String) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkLink:(Ljava/lang/String;)V")
        }

        #[java_method(name = "checkRead", descriptor = "(Ljava/io/FileDescriptor;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkRead_filede(&self, fd: Object) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkRead:(Ljava/io/FileDescriptor;)V")
        }

        #[java_method(name = "checkRead", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkRead_str(&self, file: String) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkRead:(Ljava/lang/String;)V")
        }

        #[java_method(name = "checkRead", descriptor = "(Ljava/lang/String;Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkRead_str_obj(&self, file: String, context: Object) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkRead:(Ljava/lang/String;Ljava/lang/Object;)V")
        }

        #[java_method(name = "checkWrite", descriptor = "(Ljava/io/FileDescriptor;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkWrite_filede(&self, fd: Object) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkWrite:(Ljava/io/FileDescriptor;)V")
        }

        #[java_method(name = "checkWrite", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkWrite_str(&self, file: String) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkWrite:(Ljava/lang/String;)V")
        }

        #[java_method(name = "checkDelete", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkDelete(&self, file: String) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkDelete:(Ljava/lang/String;)V")
        }

        #[java_method(name = "checkConnect", descriptor = "(Ljava/lang/String;I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkConnect_str_i(&self, host: String, port: i32) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkConnect:(Ljava/lang/String;I)V")
        }

        #[java_method(name = "checkConnect", descriptor = "(Ljava/lang/String;ILjava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkConnect_str_i_obj(&self, host: String, port: i32, context: Object) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkConnect:(Ljava/lang/String;ILjava/lang/Object;)V")
        }

        #[java_method(name = "checkListen", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkListen(&self, port: i32) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkListen:(I)V")
        }

        #[java_method(name = "checkAccept", descriptor = "(Ljava/lang/String;I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkAccept(&self, host: String, port: i32) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkAccept:(Ljava/lang/String;I)V")
        }

        #[java_method(name = "checkMulticast", descriptor = "(Ljava/net/InetAddress;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkMulticast_inetad(&self, maddr: Object) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkMulticast:(Ljava/net/InetAddress;)V")
        }

        #[java_method(name = "checkMulticast", descriptor = "(Ljava/net/InetAddress;B)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn checkMulticast_inetad_b(&self, maddr: Object, ttl: i8) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkMulticast:(Ljava/net/InetAddress;B)V")
        }

        #[java_method(name = "checkPropertiesAccess", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkPropertiesAccess(&self) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkPropertiesAccess:()V")
        }

        #[java_method(name = "checkPropertyAccess", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkPropertyAccess(&self, key: String) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkPropertyAccess:(Ljava/lang/String;)V")
        }

        #[java_method(name = "checkPrintJobAccess", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkPrintJobAccess(&self) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkPrintJobAccess:()V")
        }

        #[java_method(name = "getPackages", descriptor = "(Ljava/lang/String;)[Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPackages(mut p: String) -> Result<Rc<RefCell<Vec<String>>>> {
            let mut packages: Object = Object::default();
            let _t0 = p.isEmpty()?;
            let mut tok = StringTokenizer::new_str_str(Clone::clone(&p), Clone::clone(&String::from(",")))?;
            let _t1 = tok.countTokens()?;
            let mut n: i32 = _t1;
            let mut _arr2: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); n as usize]));
            let mut packages: Rc<RefCell<Vec<String>>> = _arr2;
            let mut i: i32 = 0i32;
            loop {
                let _t3 = tok.hasMoreElements()?;
                if !(_t3) { break; }
                let _t3 = tok.nextToken()?;
                let _t4 = _t3.trim()?;
                let mut s: String = _t4;
                i = i.wrapping_add(1i32);
                packages.borrow_mut()[i as usize] = Clone::clone(&s);
            }
            if _is_jnull(&packages) {
                let mut _arr3: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 0i32 as usize]));
                packages = _arr3;
            }
            Ok(packages)
        }

        #[java_method(name = "addNonExportedPackages", descriptor = "(Ljava/lang/ModuleLayer;)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addNonExportedPackages(layer: Object) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.addNonExportedPackages:(Ljava/lang/ModuleLayer;)V")
        }

        #[java_method(name = "invalidatePackageAccessCache", descriptor = "()V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn invalidatePackageAccessCache() -> Result<()> {
            panic!("stub: java/lang/SecurityManager.invalidatePackageAccessCache:()V")
        }

        #[java_method(name = "nonExportedPkgs", descriptor = "(Ljava/lang/module/ModuleDescriptor;)Ljava/util/Set;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/module/ModuleDescriptor;)Ljava/util/Set<Ljava/lang/String;>;")]
        pub fn nonExportedPkgs(md: Object) -> Result<Object> {
            panic!("stub: java/lang/SecurityManager.nonExportedPkgs:(Ljava/lang/module/ModuleDescriptor;)Ljava/util/Set;")
        }

        #[java_method(name = "checkPackageAccess", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkPackageAccess(&self, mut pkg: String) -> Result<()> {
            let this = self;
            let _t0: Object = Objects::requireNonNull_obj_str(Object::from_any(pkg.clone()), Clone::clone(&String::from("package name can't be null")))?;
            let _vdispatch1: bool = if let Some(_d) = SecurityManager::nonExportedPkgs_field().0.as_any().downcast_ref::<ImmutableCollections_MapN<Object, Object>>() { _d.containsKey(Object::from_any(pkg.clone()))? } else if let Some(_d) = SecurityManager::nonExportedPkgs_field().0.as_any().downcast_ref::<ImmutableCollections_Map1<Object, Object>>() { _d.containsKey(Object::from_any(pkg.clone()))? } else if let Some(_d) = SecurityManager::nonExportedPkgs_field().0.as_any().downcast_ref::<Properties>() { _d.containsKey(Object::from_any(pkg.clone()))? } else if let Some(_d) = SecurityManager::nonExportedPkgs_field().0.as_any().downcast_ref::<ConcurrentHashMap<Object, Object>>() { _d.containsKey(Object::from_any(pkg.clone()))? } else if let Some(_d) = SecurityManager::nonExportedPkgs_field().0.as_any().downcast_ref::<ImmutableCollections_AbstractImmutableMap<Object, Object>>() { _d.containsKey(Object::from_any(pkg.clone()))? } else if let Some(_d) = SecurityManager::nonExportedPkgs_field().0.as_any().downcast_ref::<TreeMap<Object, Object>>() { _d.containsKey(Object::from_any(pkg.clone()))? } else if let Some(_d) = SecurityManager::nonExportedPkgs_field().0.as_any().downcast_ref::<LinkedHashMap<Object, Object>>() { _d.containsKey(Object::from_any(pkg.clone()))? } else if let Some(_d) = SecurityManager::nonExportedPkgs_field().0.as_any().downcast_ref::<Hashtable<Object, Object>>() { _d.containsKey(Object::from_any(pkg.clone()))? } else if let Some(_d) = SecurityManager::nonExportedPkgs_field().0.as_any().downcast_ref::<Object>() { _d.containsKey(Object::from_any(pkg.clone()))? } else if let Some(_d) = SecurityManager::nonExportedPkgs_field().0.as_any().downcast_ref::<AbstractMap<Object, Object>>() { _d.containsKey(Object::from_any(pkg.clone()))? } else if let Some(_d) = SecurityManager::nonExportedPkgs_field().0.as_any().downcast_ref::<HashMap<Object, Object>>() { _d.containsKey(Object::from_any(pkg.clone()))? } else if let Some(_d) = SecurityManager::nonExportedPkgs_field().0.as_any().downcast_ref::<Object>() { _d.containsKey(Object::from_any(pkg.clone()))? } else if let Some(__f) = SecurityManager::nonExportedPkgs_field().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Object::from_any(pkg.clone()))? } else { Default::default() };
            if _vdispatch1 {
                let _t2 = StringBuilder::new()?.append_str(Clone::clone(&String::from("accessClassInPackage.")))?;
                let _t3 = _t2.append_str(Clone::clone(&pkg))?;
                let _t4 = _t3.toString()?;
                this.checkPermission_permis(Clone::clone(&RuntimePermission::new_str(Clone::clone(&_t4))?).into())?;
                return Ok(());
            }
            let mut plen: Object = SecurityManager::packageAccessLock();
            if !(SecurityManager::packageAccessValid()) {
                let _t2: Object = AccessController::doPrivileged_privil(Clone::clone(&SecurityManager_1::new(Clone::clone(this))?).into())?;
                let mut tmpPropertyStr = (_t2).downcast::<String>();
                let _t3: Rc<RefCell<Vec<String>>> = SecurityManager::getPackages(Clone::clone(&tmpPropertyStr))?;
                SecurityManager::set_packageAccess(_t3);
                SecurityManager::set_packageAccessValid(1i32);
            }
            let mut restrictedPkgs: Rc<RefCell<Vec<String>>> = SecurityManager::packageAccess();
            let _t2 = pkg.length()?;
            let mut plen: i32 = _t2;
            let mut tmpPropertyStr: Rc<RefCell<Vec<String>>> = restrictedPkgs;
            let mut local_5 = (tmpPropertyStr.borrow().len() as i32);
            let mut local_6: i32 = 0i32;
            loop {
                if local_6 >= local_5 { break; }
                let mut restrictedPkg = Clone::clone(&tmpPropertyStr.borrow()[local_6 as usize]);
                let _t3 = restrictedPkg.length()?;
                let mut rlast = (_t3).wrapping_sub(1i32);
                let _t4 = pkg.startsWith_str(Clone::clone(&restrictedPkg))?;
                if rlast == plen {
                    let _t5 = restrictedPkg.startsWith_str(Clone::clone(&pkg))?;
                    if _t5 {
                        let _t6 = restrictedPkg.charAt(rlast)?;
                        if (_t6 as i32) == 46i32 {
                            let _t7 = StringBuilder::new()?.append_str(Clone::clone(&String::from("accessClassInPackage.")))?;
                            let _t8 = _t7.append_str(Clone::clone(&pkg))?;
                            let _t9 = _t8.toString()?;
                            this.checkPermission_permis(Clone::clone(&RuntimePermission::new_str(Clone::clone(&_t9))?).into())?;
                            break;
                        }
                    } else {
                        local_6 = local_6.wrapping_add(1i32);
                        continue;
                    }
                } else {
                    local_6 = local_6.wrapping_add(1i32);
                    continue;
                }
            }
            Ok(())
        }

        #[java_method(name = "checkPackageDefinition", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkPackageDefinition(&self, pkg: String) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkPackageDefinition:(Ljava/lang/String;)V")
        }

        #[java_method(name = "checkSetFactory", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkSetFactory(&self) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkSetFactory:()V")
        }

        #[java_method(name = "checkSecurityAccess", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkSecurityAccess(&self, target: String) -> Result<()> {
            panic!("stub: java/lang/SecurityManager.checkSecurityAccess:(Ljava/lang/String;)V")
        }

        #[java_method(name = "getThreadGroup", descriptor = "()Ljava/lang/ThreadGroup;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getThreadGroup(&self) -> Result<Object> {
            panic!("stub: java/lang/SecurityManager.getThreadGroup:()Ljava/lang/ThreadGroup;")
        }
    }
}
