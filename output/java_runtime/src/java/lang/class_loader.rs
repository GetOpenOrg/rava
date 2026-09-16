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
use crate::jdk::internal::misc::Unsafe;
use crate::jdk::internal::misc::VM;
use crate::jdk::internal::reflect::Reflection;
use crate::jdk::internal::util::StaticProperty;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/ClassLoader"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ClassLoader.java"]
    #[inner_classes     = "java/lang/ClassLoader$ParallelLoaders:java/lang/ClassLoader:ParallelLoaders:10;java/lang/ClassLoader$1:::0;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/ClassLoader;java/lang/Object"]

    pub struct ClassLoader {
        #[cfg_attr(any(), java_field(name = "parent", descriptor = "Ljava/lang/ClassLoader;", access = "private", modifiers = "final", is_static = false))]
        pub parent: ClassLoader,
        #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub name: String,
        #[cfg_attr(any(), java_field(name = "unnamedModule", descriptor = "Ljava/lang/Module;", access = "private", modifiers = "final", is_static = false))]
        pub unnamedModule: Object,
        #[cfg_attr(any(), java_field(name = "nameAndId", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub nameAndId: String,
        #[cfg_attr(any(), java_field(name = "parallelLockMap", descriptor = "Ljava/util/concurrent/ConcurrentHashMap;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/concurrent/ConcurrentHashMap<Ljava/lang/String;Ljava/lang/Object;>;"))]
        pub parallelLockMap: ConcurrentHashMap<String, Object>,
        #[cfg_attr(any(), java_field(name = "package2certs", descriptor = "Ljava/util/concurrent/ConcurrentHashMap;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/concurrent/ConcurrentHashMap<Ljava/lang/String;[Ljava/security/cert/Certificate;>;"))]
        pub package2certs: ConcurrentHashMap<Object, Object>,
        #[cfg_attr(any(), java_field(name = "classes", descriptor = "Ljava/util/ArrayList;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/ArrayList<Ljava/lang/Class<*>;>;"))]
        pub classes: ArrayList<Class<Object>>,
        #[cfg_attr(any(), java_field(name = "defaultDomain", descriptor = "Ljava/security/ProtectionDomain;", access = "private", modifiers = "final", is_static = false))]
        pub defaultDomain: Object,
        #[cfg_attr(any(), java_field(name = "packages", descriptor = "Ljava/util/concurrent/ConcurrentHashMap;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/concurrent/ConcurrentHashMap<Ljava/lang/String;Ljava/lang/NamedPackage;>;"))]
        pub packages: ConcurrentHashMap<Object, Object>,
        #[cfg_attr(any(), java_field(name = "libraries", descriptor = "Ljdk/internal/loader/NativeLibraries;", access = "private", modifiers = "final", is_static = false))]
        pub libraries: Object,
        #[cfg_attr(any(), java_field(name = "assertionLock", descriptor = "Ljava/lang/Object;", access = "package", modifiers = "final", is_static = false))]
        pub assertionLock: Object,
        #[cfg_attr(any(), java_field(name = "defaultAssertionStatus", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub defaultAssertionStatus: bool,
        #[cfg_attr(any(), java_field(name = "packageAssertionStatus", descriptor = "Ljava/util/Map;", access = "private", modifiers = "", is_static = false, generic_signature = "Ljava/util/Map<Ljava/lang/String;Ljava/lang/Boolean;>;"))]
        pub packageAssertionStatus: Object,
        #[cfg_attr(any(), java_field(name = "classAssertionStatus", descriptor = "Ljava/util/Map;", is_static = false, generic_signature = "Ljava/util/Map<Ljava/lang/String;Ljava/lang/Boolean;>;"))]
        pub classAssertionStatus: Object,
        #[cfg_attr(any(), java_field(name = "classLoaderValueMap", descriptor = "Ljava/util/concurrent/ConcurrentHashMap;", access = "private", modifiers = "volatile", is_static = false, generic_signature = "Ljava/util/concurrent/ConcurrentHashMap<**>;"))]
        pub classLoaderValueMap: ConcurrentHashMap<Object, Object>,
    }

    impl ClassLoader {
        #[cfg_attr(any(), java_field(name = "nocerts", descriptor = "[Ljava/security/cert/Certificate;", access = "private", modifiers = "static final", is_static = true))]
        // static field: nocerts:[Ljava/security/cert/Certificate;
        pub fn nocerts() -> Rc<RefCell<Vec<Object>>> {
            Rc::new(RefCell::new(Vec::new()))
        }

        #[cfg_attr(any(), java_field(name = "scl", descriptor = "Ljava/lang/ClassLoader;", access = "private", modifiers = "static volatile", is_static = true))]
        // static field: scl:Ljava/lang/ClassLoader;
        pub fn scl() -> ClassLoader {
            panic!("stub: java/lang/ClassLoader.scl:Ljava/lang/ClassLoader;")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[native]
        #[java_native(name = "registerNatives", descriptor = "()V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn registerNatives() -> Result<()> {
            panic!("native: java/lang/ClassLoader.registerNatives:()V")
        }

        #[java_method(name = "addClass", descriptor = "(Ljava/lang/Class;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)V")]
        pub fn addClass(&self, c: Object) -> Result<()> {
            panic!("stub: java/lang/ClassLoader.addClass:(Ljava/lang/Class;)V")
        }

        #[java_method(name = "getNamedPackage", descriptor = "(Ljava/lang/String;Ljava/lang/Module;)Ljava/lang/NamedPackage;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNamedPackage(&self, pn: String, m: Object) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.getNamedPackage:(Ljava/lang/String;Ljava/lang/Module;)Ljava/lang/NamedPackage;")
        }

        #[java_method(name = "checkCreateClassLoader", descriptor = "()Ljava/lang/Void;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkCreateClassLoader() -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.checkCreateClassLoader:()Ljava/lang/Void;")
        }

        #[java_method(name = "checkCreateClassLoader", descriptor = "(Ljava/lang/String;)Ljava/lang/Void;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkCreateClassLoader_str(name: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.checkCreateClassLoader:(Ljava/lang/String;)Ljava/lang/Void;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Void;Ljava/lang/String;Ljava/lang/ClassLoader;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_void_str_classl(unused: Object, name: String, parent: ClassLoader) -> Result<Self> {
            panic!("stub: java/lang/ClassLoader.<init>:(Ljava/lang/Void;Ljava/lang/String;Ljava/lang/ClassLoader;)V")
        }

        #[java_method(name = "nameAndId", descriptor = "(Ljava/lang/ClassLoader;)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nameAndId_classl(ld: ClassLoader) -> Result<String> {
            panic!("stub: java/lang/ClassLoader.nameAndId:(Ljava/lang/ClassLoader;)Ljava/lang/String;")
        }

        #[java_method(name = "nameAndId", descriptor = "()Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nameAndId(&self) -> Result<String> {
            panic!("stub: java/lang/ClassLoader.nameAndId:()Ljava/lang/String;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/ClassLoader;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str_classl(name: String, parent: ClassLoader) -> Result<Self> {
            panic!("stub: java/lang/ClassLoader.<init>:(Ljava/lang/String;Ljava/lang/ClassLoader;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/ClassLoader;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_classl(parent: ClassLoader) -> Result<Self> {
            panic!("stub: java/lang/ClassLoader.<init>:(Ljava/lang/ClassLoader;)V")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/ClassLoader.<init>:()V")
        }

        #[java_method(name = "getName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getName(&self) -> Result<String> {
            panic!("stub: java/lang/ClassLoader.getName:()Ljava/lang/String;")
        }

        #[java_method(name = "name", descriptor = "()Ljava/lang/String;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn name(&self) -> Result<String> {
            panic!("stub: java/lang/ClassLoader.name:()Ljava/lang/String;")
        }

        #[java_method(name = "loadClass", descriptor = "(Ljava/lang/String;)Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/ClassNotFoundException", generic_signature = "(Ljava/lang/String;)Ljava/lang/Class<*>;")]
        pub fn loadClass_str(&self, name: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.loadClass:(Ljava/lang/String;)Ljava/lang/Class;")
        }

        #[java_method(name = "loadClass", descriptor = "(Ljava/lang/String;Z)Ljava/lang/Class;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/ClassNotFoundException", generic_signature = "(Ljava/lang/String;Z)Ljava/lang/Class<*>;")]
        pub fn loadClass_str_z(&self, name: String, resolve: bool) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.loadClass:(Ljava/lang/String;Z)Ljava/lang/Class;")
        }

        #[java_method(name = "loadClass", descriptor = "(Ljava/lang/Module;Ljava/lang/String;)Ljava/lang/Class;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Module;Ljava/lang/String;)Ljava/lang/Class<*>;")]
        pub fn loadClass_module_str(&self, module: Object, name: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.loadClass:(Ljava/lang/Module;Ljava/lang/String;)Ljava/lang/Class;")
        }

        #[java_method(name = "getClassLoadingLock", descriptor = "(Ljava/lang/String;)Ljava/lang/Object;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getClassLoadingLock(&self, className: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.getClassLoadingLock:(Ljava/lang/String;)Ljava/lang/Object;")
        }

        #[java_method(name = "checkPackageAccess", descriptor = "(Ljava/lang/Class;Ljava/security/ProtectionDomain;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/security/ProtectionDomain;)V")]
        pub fn checkPackageAccess(&self, cls: Object, pd: Object) -> Result<()> {
            panic!("stub: java/lang/ClassLoader.checkPackageAccess:(Ljava/lang/Class;Ljava/security/ProtectionDomain;)V")
        }

        #[java_method(name = "findClass", descriptor = "(Ljava/lang/String;)Ljava/lang/Class;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/ClassNotFoundException", generic_signature = "(Ljava/lang/String;)Ljava/lang/Class<*>;")]
        pub fn findClass_str(&self, name: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.findClass:(Ljava/lang/String;)Ljava/lang/Class;")
        }

        #[java_method(name = "findClass", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Class;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Class<*>;")]
        pub fn findClass_str_str(&self, moduleName: String, name: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.findClass:(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Class;")
        }

        #[java_method(name = "defineClass", descriptor = "([BII)Ljava/lang/Class;", access = "protected", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/ClassFormatError", generic_signature = "([BII)Ljava/lang/Class<*>;", is_deprecated = true)]
        pub fn defineClass_arr_b_i_i(&self, b: Rc<RefCell<Vec<i8>>>, off: i32, len: i32) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.defineClass:([BII)Ljava/lang/Class;")
        }

        #[java_method(name = "defineClass", descriptor = "(Ljava/lang/String;[BII)Ljava/lang/Class;", access = "protected", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/ClassFormatError", generic_signature = "(Ljava/lang/String;[BII)Ljava/lang/Class<*>;")]
        pub fn defineClass_str_arr_b_i_i(&self, name: String, b: Rc<RefCell<Vec<i8>>>, off: i32, len: i32) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.defineClass:(Ljava/lang/String;[BII)Ljava/lang/Class;")
        }

        #[java_method(name = "preDefineClass", descriptor = "(Ljava/lang/String;Ljava/security/ProtectionDomain;)Ljava/security/ProtectionDomain;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn preDefineClass(&self, name: String, pd: Object) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.preDefineClass:(Ljava/lang/String;Ljava/security/ProtectionDomain;)Ljava/security/ProtectionDomain;")
        }

        #[java_method(name = "defineClassSourceLocation", descriptor = "(Ljava/security/ProtectionDomain;)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn defineClassSourceLocation(&self, pd: Object) -> Result<String> {
            panic!("stub: java/lang/ClassLoader.defineClassSourceLocation:(Ljava/security/ProtectionDomain;)Ljava/lang/String;")
        }

        #[java_method(name = "postDefineClass", descriptor = "(Ljava/lang/Class;Ljava/security/ProtectionDomain;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/security/ProtectionDomain;)V")]
        pub fn postDefineClass(&self, c: Object, pd: Object) -> Result<()> {
            panic!("stub: java/lang/ClassLoader.postDefineClass:(Ljava/lang/Class;Ljava/security/ProtectionDomain;)V")
        }

        #[java_method(name = "defineClass", descriptor = "(Ljava/lang/String;[BIILjava/security/ProtectionDomain;)Ljava/lang/Class;", access = "protected", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/ClassFormatError", generic_signature = "(Ljava/lang/String;[BIILjava/security/ProtectionDomain;)Ljava/lang/Class<*>;")]
        pub fn defineClass_str_arr_b_i_i_protec(&self, name: String, b: Rc<RefCell<Vec<i8>>>, off: i32, len: i32, protectionDomain: Object) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.defineClass:(Ljava/lang/String;[BIILjava/security/ProtectionDomain;)Ljava/lang/Class;")
        }

        #[java_method(name = "defineClass", descriptor = "(Ljava/lang/String;Ljava/nio/ByteBuffer;Ljava/security/ProtectionDomain;)Ljava/lang/Class;", access = "protected", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/ClassFormatError", generic_signature = "(Ljava/lang/String;Ljava/nio/ByteBuffer;Ljava/security/ProtectionDomain;)Ljava/lang/Class<*>;")]
        pub fn defineClass_str_bytebu_protec(&self, name: String, b: ByteBuffer, protectionDomain: Object) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.defineClass:(Ljava/lang/String;Ljava/nio/ByteBuffer;Ljava/security/ProtectionDomain;)Ljava/lang/Class;")
        }

        #[native]
        #[java_native(name = "defineClass1", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/String;[BIILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;", access = "package", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ClassLoader;Ljava/lang/String;[BIILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class<*>;")]
        pub fn defineClass1(arg0: ClassLoader, arg1: String, arg2: Rc<RefCell<Vec<i8>>>, arg3: i32, arg4: i32, arg5: Object, arg6: String) -> Result<Object> {
            panic!("native: java/lang/ClassLoader.defineClass1:(Ljava/lang/ClassLoader;Ljava/lang/String;[BIILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;")
        }

        #[native]
        #[java_native(name = "defineClass2", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/String;Ljava/nio/ByteBuffer;IILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;", access = "package", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ClassLoader;Ljava/lang/String;Ljava/nio/ByteBuffer;IILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class<*>;")]
        pub fn defineClass2(arg0: ClassLoader, arg1: String, arg2: ByteBuffer, arg3: i32, arg4: i32, arg5: Object, arg6: String) -> Result<Object> {
            panic!("native: java/lang/ClassLoader.defineClass2:(Ljava/lang/ClassLoader;Ljava/lang/String;Ljava/nio/ByteBuffer;IILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;")
        }

        #[native]
        #[java_native(name = "defineClass0", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/Class;Ljava/lang/String;[BIILjava/security/ProtectionDomain;ZILjava/lang/Object;)Ljava/lang/Class;", access = "package", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ClassLoader;Ljava/lang/Class<*>;Ljava/lang/String;[BIILjava/security/ProtectionDomain;ZILjava/lang/Object;)Ljava/lang/Class<*>;")]
        pub fn defineClass0(arg0: ClassLoader, arg1: Object, arg2: String, arg3: Rc<RefCell<Vec<i8>>>, arg4: i32, arg5: i32, arg6: Object, arg7: bool, arg8: i32, arg9: Object) -> Result<Object> {
            panic!("native: java/lang/ClassLoader.defineClass0:(Ljava/lang/ClassLoader;Ljava/lang/Class;Ljava/lang/String;[BIILjava/security/ProtectionDomain;ZILjava/lang/Object;)Ljava/lang/Class;")
        }

        #[java_method(name = "checkName", descriptor = "(Ljava/lang/String;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkName(name: String) -> Result<bool> {
            panic!("stub: java/lang/ClassLoader.checkName:(Ljava/lang/String;)Z")
        }

        #[java_method(name = "checkCerts", descriptor = "(Ljava/lang/String;Ljava/security/CodeSource;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkCerts(&self, name: String, cs: Object) -> Result<()> {
            panic!("stub: java/lang/ClassLoader.checkCerts:(Ljava/lang/String;Ljava/security/CodeSource;)V")
        }

        #[java_method(name = "compareCerts", descriptor = "([Ljava/security/cert/Certificate;[Ljava/security/cert/Certificate;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareCerts(&self, pcerts: Rc<RefCell<Vec<Object>>>, certs: Rc<RefCell<Vec<Object>>>) -> Result<bool> {
            panic!("stub: java/lang/ClassLoader.compareCerts:([Ljava/security/cert/Certificate;[Ljava/security/cert/Certificate;)Z")
        }

        #[java_method(name = "resolveClass", descriptor = "(Ljava/lang/Class;)V", access = "protected", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)V")]
        pub fn resolveClass(&self, c: Object) -> Result<()> {
            panic!("stub: java/lang/ClassLoader.resolveClass:(Ljava/lang/Class;)V")
        }

        #[java_method(name = "findSystemClass", descriptor = "(Ljava/lang/String;)Ljava/lang/Class;", access = "protected", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/ClassNotFoundException", generic_signature = "(Ljava/lang/String;)Ljava/lang/Class<*>;")]
        pub fn findSystemClass(&self, name: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.findSystemClass:(Ljava/lang/String;)Ljava/lang/Class;")
        }

        #[java_method(name = "findBootstrapClassOrNull", descriptor = "(Ljava/lang/String;)Ljava/lang/Class;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;)Ljava/lang/Class<*>;")]
        pub fn findBootstrapClassOrNull(name: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.findBootstrapClassOrNull:(Ljava/lang/String;)Ljava/lang/Class;")
        }

        #[native]
        #[java_native(name = "findBootstrapClass", descriptor = "(Ljava/lang/String;)Ljava/lang/Class;", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;)Ljava/lang/Class<*>;")]
        pub fn findBootstrapClass(arg0: String) -> Result<Object> {
            panic!("native: java/lang/ClassLoader.findBootstrapClass:(Ljava/lang/String;)Ljava/lang/Class;")
        }

        #[java_method(name = "findLoadedClass", descriptor = "(Ljava/lang/String;)Ljava/lang/Class;", access = "protected", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;)Ljava/lang/Class<*>;")]
        pub fn findLoadedClass(&self, name: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.findLoadedClass:(Ljava/lang/String;)Ljava/lang/Class;")
        }

        #[native]
        #[java_native(name = "findLoadedClass0", descriptor = "(Ljava/lang/String;)Ljava/lang/Class;", access = "private", modifiers = "final native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;)Ljava/lang/Class<*>;")]
        pub fn findLoadedClass0(&self, arg0: String) -> Result<Object> {
            panic!("native: java/lang/ClassLoader.findLoadedClass0:(Ljava/lang/String;)Ljava/lang/Class;")
        }

        #[java_method(name = "setSigners", descriptor = "(Ljava/lang/Class;[Ljava/lang/Object;)V", access = "protected", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;[Ljava/lang/Object;)V")]
        pub fn setSigners(&self, c: Object, signers: Rc<RefCell<Vec<Object>>>) -> Result<()> {
            panic!("stub: java/lang/ClassLoader.setSigners:(Ljava/lang/Class;[Ljava/lang/Object;)V")
        }

        #[java_method(name = "findResource", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/net/URL;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn findResource_str_str(&self, moduleName: String, name: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.findResource:(Ljava/lang/String;Ljava/lang/String;)Ljava/net/URL;")
        }

        #[java_method(name = "getResource", descriptor = "(Ljava/lang/String;)Ljava/net/URL;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getResource(&self, name: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.getResource:(Ljava/lang/String;)Ljava/net/URL;")
        }

        #[java_method(name = "getResources", descriptor = "(Ljava/lang/String;)Ljava/util/Enumeration;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException", generic_signature = "(Ljava/lang/String;)Ljava/util/Enumeration<Ljava/net/URL;>;")]
        pub fn getResources(&self, name: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.getResources:(Ljava/lang/String;)Ljava/util/Enumeration;")
        }

        #[java_method(name = "resources", descriptor = "(Ljava/lang/String;)Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;)Ljava/util/stream/Stream<Ljava/net/URL;>;")]
        pub fn resources(&self, name: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.resources:(Ljava/lang/String;)Ljava/util/stream/Stream;")
        }

        #[java_method(name = "findResource", descriptor = "(Ljava/lang/String;)Ljava/net/URL;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn findResource_str(&self, name: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.findResource:(Ljava/lang/String;)Ljava/net/URL;")
        }

        #[java_method(name = "findResources", descriptor = "(Ljava/lang/String;)Ljava/util/Enumeration;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException", generic_signature = "(Ljava/lang/String;)Ljava/util/Enumeration<Ljava/net/URL;>;")]
        pub fn findResources(&self, name: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.findResources:(Ljava/lang/String;)Ljava/util/Enumeration;")
        }

        #[java_method(name = "registerAsParallelCapable", descriptor = "()Z", access = "protected", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn registerAsParallelCapable() -> Result<bool> {
            panic!("stub: java/lang/ClassLoader.registerAsParallelCapable:()Z")
        }

        #[java_method(name = "registerAsParallelCapable", descriptor = "(Ljava/lang/Class;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z")]
        pub fn registerAsParallelCapable_class(caller: Object) -> Result<bool> {
            panic!("stub: java/lang/ClassLoader.registerAsParallelCapable:(Ljava/lang/Class;)Z")
        }

        #[java_method(name = "isRegisteredAsParallelCapable", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isRegisteredAsParallelCapable(&self) -> Result<bool> {
            panic!("stub: java/lang/ClassLoader.isRegisteredAsParallelCapable:()Z")
        }

        #[java_method(name = "getSystemResource", descriptor = "(Ljava/lang/String;)Ljava/net/URL;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSystemResource(name: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.getSystemResource:(Ljava/lang/String;)Ljava/net/URL;")
        }

        #[java_method(name = "getSystemResources", descriptor = "(Ljava/lang/String;)Ljava/util/Enumeration;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException", generic_signature = "(Ljava/lang/String;)Ljava/util/Enumeration<Ljava/net/URL;>;")]
        pub fn getSystemResources(name: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.getSystemResources:(Ljava/lang/String;)Ljava/util/Enumeration;")
        }

        #[java_method(name = "getResourceAsStream", descriptor = "(Ljava/lang/String;)Ljava/io/InputStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getResourceAsStream(&self, name: String) -> Result<InputStream> {
            panic!("stub: java/lang/ClassLoader.getResourceAsStream:(Ljava/lang/String;)Ljava/io/InputStream;")
        }

        #[java_method(name = "getSystemResourceAsStream", descriptor = "(Ljava/lang/String;)Ljava/io/InputStream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSystemResourceAsStream(name: String) -> Result<InputStream> {
            panic!("stub: java/lang/ClassLoader.getSystemResourceAsStream:(Ljava/lang/String;)Ljava/io/InputStream;")
        }

        #[java_method(name = "getParent", descriptor = "()Ljava/lang/ClassLoader;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getParent(&self) -> Result<ClassLoader> {
            panic!("stub: java/lang/ClassLoader.getParent:()Ljava/lang/ClassLoader;")
        }

        #[java_method(name = "getUnnamedModule", descriptor = "()Ljava/lang/Module;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getUnnamedModule(&self) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.getUnnamedModule:()Ljava/lang/Module;")
        }

        #[java_method(name = "getPlatformClassLoader", descriptor = "()Ljava/lang/ClassLoader;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPlatformClassLoader() -> Result<ClassLoader> {
            panic!("stub: java/lang/ClassLoader.getPlatformClassLoader:()Ljava/lang/ClassLoader;")
        }

        #[java_method(name = "getSystemClassLoader", descriptor = "()Ljava/lang/ClassLoader;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSystemClassLoader() -> Result<ClassLoader> {
            panic!("stub: java/lang/ClassLoader.getSystemClassLoader:()Ljava/lang/ClassLoader;")
        }

        #[java_method(name = "getBuiltinPlatformClassLoader", descriptor = "()Ljava/lang/ClassLoader;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBuiltinPlatformClassLoader() -> Result<ClassLoader> {
            panic!("stub: java/lang/ClassLoader.getBuiltinPlatformClassLoader:()Ljava/lang/ClassLoader;")
        }

        #[java_method(name = "getBuiltinAppClassLoader", descriptor = "()Ljava/lang/ClassLoader;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBuiltinAppClassLoader() -> Result<ClassLoader> {
            panic!("stub: java/lang/ClassLoader.getBuiltinAppClassLoader:()Ljava/lang/ClassLoader;")
        }

        #[java_method(name = "initSystemClassLoader", descriptor = "()Ljava/lang/ClassLoader;", access = "package", modifiers = "static synchronized", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn initSystemClassLoader() -> Result<ClassLoader> {
            panic!("stub: java/lang/ClassLoader.initSystemClassLoader:()Ljava/lang/ClassLoader;")
        }

        #[java_method(name = "isAncestor", descriptor = "(Ljava/lang/ClassLoader;)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isAncestor(&self, mut cl: ClassLoader) -> Result<bool> {
            let this = self;
            let mut acl = this.clone();
            loop {
                acl = acl.__get_parent();
                if Object::from_any(cl.clone()) == Object::from_any(acl.clone()) {
                    return Ok((1i32 != 0i32));
                }
                if _is_jnull(&acl) { break; }
            }
            Ok((0i32 != 0i32))
        }

        #[java_method(name = "needsClassLoaderPermissionCheck", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn needsClassLoaderPermissionCheck(mut from: ClassLoader, mut to: ClassLoader) -> Result<bool> {
            if Object::from_any(from.clone()) == Object::from_any(to.clone()) {
                return Ok((0i32 != 0i32));
            }
            if _is_jnull(&from) {
                return Ok((0i32 != 0i32));
            }
            let _t0 = to.isAncestor(Clone::clone(&from))?;
            Ok(!(_t0))
        }

        #[java_method(name = "getClassLoader", descriptor = "(Ljava/lang/Class;)Ljava/lang/ClassLoader;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Ljava/lang/ClassLoader;")]
        pub fn getClassLoader(mut caller: Object) -> Result<ClassLoader> {
            if _is_jnull(&caller) {
                return Ok(Default::default());
            }
            let _vdispatch0: ClassLoader = if let Some(__f) = caller.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<ClassLoader>>>() { (__f)()? } else { Default::default() };
            Ok(_vdispatch0)
        }

        #[java_method(name = "checkClassLoaderPermission", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/Class;)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ClassLoader;Ljava/lang/Class<*>;)V")]
        pub fn checkClassLoaderPermission(mut cl: ClassLoader, mut caller: Object) -> Result<()> {
            let _t0: SecurityManager = System::getSecurityManager()?;
            let mut sm: SecurityManager = _t0;
            let _t1: ClassLoader = ClassLoader::getClassLoader(Clone::clone(&caller))?;
            let mut ccl: ClassLoader = _t1;
            let _t2: bool = ClassLoader::needsClassLoaderPermissionCheck(Clone::clone(&ccl), Clone::clone(&cl))?;
            if _t2 {
                sm.checkPermission_permis(Clone::clone(&SecurityConstants::GET_CLASSLOADER_PERMISSION()).into())?;
            }
            Ok(())
        }

        #[java_method(name = "definePackage", descriptor = "(Ljava/lang/Class;)Ljava/lang/Package;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Ljava/lang/Package;")]
        pub fn definePackage_class(&self, c: Object) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.definePackage:(Ljava/lang/Class;)Ljava/lang/Package;")
        }

        #[java_method(name = "definePackage", descriptor = "(Ljava/lang/String;Ljava/lang/Module;)Ljava/lang/Package;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn definePackage_str_module(&self, name: String, m: Object) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.definePackage:(Ljava/lang/String;Ljava/lang/Module;)Ljava/lang/Package;")
        }

        #[java_method(name = "toPackage", descriptor = "(Ljava/lang/String;Ljava/lang/NamedPackage;Ljava/lang/Module;)Ljava/lang/Package;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toPackage(&self, name: String, p: Object, m: Object) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.toPackage:(Ljava/lang/String;Ljava/lang/NamedPackage;Ljava/lang/Module;)Ljava/lang/Package;")
        }

        #[java_method(name = "definePackage", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/net/URL;)Ljava/lang/Package;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn definePackage_str_str_str_str_str_str_str_url(&self, name: String, specTitle: String, specVersion: String, specVendor: String, implTitle: String, implVersion: String, implVendor: String, sealBase: Object) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.definePackage:(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/net/URL;)Ljava/lang/Package;")
        }

        #[java_method(name = "getDefinedPackage", descriptor = "(Ljava/lang/String;)Ljava/lang/Package;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDefinedPackage(&self, name: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.getDefinedPackage:(Ljava/lang/String;)Ljava/lang/Package;")
        }

        #[java_method(name = "getDefinedPackages", descriptor = "()[Ljava/lang/Package;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDefinedPackages(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/ClassLoader.getDefinedPackages:()[Ljava/lang/Package;")
        }

        #[java_method(name = "getPackage", descriptor = "(Ljava/lang/String;)Ljava/lang/Package;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getPackage(&self, name: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.getPackage:(Ljava/lang/String;)Ljava/lang/Package;")
        }

        #[java_method(name = "getPackages", descriptor = "()[Ljava/lang/Package;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPackages(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/ClassLoader.getPackages:()[Ljava/lang/Package;")
        }

        #[java_method(name = "packages", descriptor = "()Ljava/util/stream/Stream;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<Ljava/lang/Package;>;")]
        pub fn packages(&self) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.packages:()Ljava/util/stream/Stream;")
        }

        #[java_method(name = "findLibrary", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn findLibrary(&self, libname: String) -> Result<String> {
            panic!("stub: java/lang/ClassLoader.findLibrary:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "loadLibrary", descriptor = "(Ljava/lang/Class;Ljava/io/File;)Ljdk/internal/loader/NativeLibrary;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/io/File;)Ljdk/internal/loader/NativeLibrary;")]
        pub fn loadLibrary_class_file(fromClass: Object, file: Object) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.loadLibrary:(Ljava/lang/Class;Ljava/io/File;)Ljdk/internal/loader/NativeLibrary;")
        }

        #[java_method(name = "loadLibrary", descriptor = "(Ljava/lang/Class;Ljava/lang/String;)Ljdk/internal/loader/NativeLibrary;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/String;)Ljdk/internal/loader/NativeLibrary;")]
        pub fn loadLibrary_class_str(fromClass: Object, name: String) -> Result<Object> {
            panic!("stub: java/lang/ClassLoader.loadLibrary:(Ljava/lang/Class;Ljava/lang/String;)Ljdk/internal/loader/NativeLibrary;")
        }

        #[java_method(name = "findNative", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/String;)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn findNative(loader: ClassLoader, entryName: String) -> Result<i64> {
            panic!("stub: java/lang/ClassLoader.findNative:(Ljava/lang/ClassLoader;Ljava/lang/String;)J")
        }

        #[java_method(name = "setDefaultAssertionStatus", descriptor = "(Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDefaultAssertionStatus(&self, enabled: bool) -> Result<()> {
            panic!("stub: java/lang/ClassLoader.setDefaultAssertionStatus:(Z)V")
        }

        #[java_method(name = "setPackageAssertionStatus", descriptor = "(Ljava/lang/String;Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setPackageAssertionStatus(&self, packageName: String, enabled: bool) -> Result<()> {
            panic!("stub: java/lang/ClassLoader.setPackageAssertionStatus:(Ljava/lang/String;Z)V")
        }

        #[java_method(name = "setClassAssertionStatus", descriptor = "(Ljava/lang/String;Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setClassAssertionStatus(&self, className: String, enabled: bool) -> Result<()> {
            panic!("stub: java/lang/ClassLoader.setClassAssertionStatus:(Ljava/lang/String;Z)V")
        }

        #[java_method(name = "clearAssertionStatus", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clearAssertionStatus(&self) -> Result<()> {
            panic!("stub: java/lang/ClassLoader.clearAssertionStatus:()V")
        }

        #[java_method(name = "desiredAssertionStatus", descriptor = "(Ljava/lang/String;)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn desiredAssertionStatus(&self, className: String) -> Result<bool> {
            panic!("stub: java/lang/ClassLoader.desiredAssertionStatus:(Ljava/lang/String;)Z")
        }

        #[java_method(name = "initializeJavaAssertionMaps", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn initializeJavaAssertionMaps(&self) -> Result<()> {
            panic!("stub: java/lang/ClassLoader.initializeJavaAssertionMaps:()V")
        }

        #[native]
        #[java_native(name = "retrieveDirectives", descriptor = "()Ljava/lang/AssertionStatusDirectives;", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn retrieveDirectives() -> Result<Object> {
            panic!("native: java/lang/ClassLoader.retrieveDirectives:()Ljava/lang/AssertionStatusDirectives;")
        }

        #[java_method(name = "createOrGetClassLoaderValueMap", descriptor = "()Ljava/util/concurrent/ConcurrentHashMap;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/concurrent/ConcurrentHashMap<**>;")]
        pub fn createOrGetClassLoaderValueMap(&self) -> Result<ConcurrentHashMap<Object, Object>> {
            panic!("stub: java/lang/ClassLoader.createOrGetClassLoaderValueMap:()Ljava/util/concurrent/ConcurrentHashMap;")
        }

        #[java_method(name = "trySetObjectField", descriptor = "(Ljava/lang/String;Ljava/lang/Object;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn trySetObjectField(&self, name: String, obj: Object) -> Result<bool> {
            panic!("stub: java/lang/ClassLoader.trySetObjectField:(Ljava/lang/String;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "resetArchivedStates", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn resetArchivedStates(&self) -> Result<()> {
            panic!("stub: java/lang/ClassLoader.resetArchivedStates:()V")
        }
    }
}
