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
use crate::jdk::internal::loader::AbstractClassLoaderValue_Sub;
use crate::jdk::internal::loader::ClassLoaderValue;
use crate::jdk::internal::misc::VM;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/reflect/Proxy$ProxyBuilder"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Proxy.java"]
    #[inner_classes     = "java/lang/reflect/Proxy$ProxyBuilder:java/lang/reflect/Proxy:ProxyBuilder:26;java/lang/reflect/Proxy$ProxyBuilder$ProxyClassContext:java/lang/reflect/Proxy$ProxyBuilder:ProxyClassContext:26;jdk/internal/loader/AbstractClassLoaderValue$Sub:jdk/internal/loader/AbstractClassLoaderValue:Sub:17;java/lang/reflect/Proxy$ProxyBuilder$1:::0;java/util/Map$Entry:java/util/Map:Entry:1545;java/lang/module/ModuleDescriptor$Modifier:java/lang/module/ModuleDescriptor:Modifier:16409;java/lang/module/ModuleDescriptor$Builder:java/lang/module/ModuleDescriptor:Builder:25;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/lang/reflect/Proxy$ProxyBuilder"]

    pub struct Proxy_ProxyBuilder {
        #[cfg_attr(any(), java_field(name = "interfaces", descriptor = "Ljava/util/List;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/List<Ljava/lang/Class<*>;>;"))]
        pub interfaces: Object,
        #[cfg_attr(any(), java_field(name = "context", descriptor = "Ljava/lang/reflect/Proxy$ProxyBuilder$ProxyClassContext;", access = "private", modifiers = "final", is_static = false))]
        pub context: Object,
    }

    impl Proxy_ProxyBuilder {
        #[cfg_attr(any(), java_field(name = "JLA", descriptor = "Ljdk/internal/access/JavaLangAccess;", access = "private", modifiers = "static final", is_static = true))]
        // static field: JLA:Ljdk/internal/access/JavaLangAccess;
        pub fn JLA() -> Object {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.JLA:Ljdk/internal/access/JavaLangAccess;")
        }

        #[cfg_attr(any(), java_field(name = "proxyClassNamePrefix", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "$Proxy"))]
        // static field: proxyClassNamePrefix:Ljava/lang/String;
        pub fn proxyClassNamePrefix() -> String {
            String::from("$Proxy")
        }

        #[cfg_attr(any(), java_field(name = "nextUniqueNumber", descriptor = "Ljava/util/concurrent/atomic/AtomicLong;", access = "private", modifiers = "static final", is_static = true))]
        // static field: nextUniqueNumber:Ljava/util/concurrent/atomic/AtomicLong;
        pub fn nextUniqueNumber() -> AtomicLong {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.nextUniqueNumber:Ljava/util/concurrent/atomic/AtomicLong;")
        }

        #[cfg_attr(any(), java_field(name = "reverseProxyCache", descriptor = "Ljdk/internal/loader/ClassLoaderValue;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljdk/internal/loader/ClassLoaderValue<Ljava/lang/Boolean;>;"))]
        // static field: reverseProxyCache:Ljdk/internal/loader/ClassLoaderValue;
        pub fn reverseProxyCache() -> ClassLoaderValue<bool> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.reverseProxyCache:Ljdk/internal/loader/ClassLoaderValue;")
        }

        #[cfg_attr(any(), java_field(name = "DEBUG", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: DEBUG:Ljava/lang/String;
        pub fn DEBUG() -> String {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.DEBUG:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "dynProxyModules", descriptor = "Ljdk/internal/loader/ClassLoaderValue;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljdk/internal/loader/ClassLoaderValue<Ljava/lang/Module;>;"))]
        // static field: dynProxyModules:Ljdk/internal/loader/ClassLoaderValue;
        pub fn dynProxyModules() -> ClassLoaderValue<Object> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.dynProxyModules:Ljdk/internal/loader/ClassLoaderValue;")
        }

        #[cfg_attr(any(), java_field(name = "counter", descriptor = "Ljava/util/concurrent/atomic/AtomicInteger;", access = "private", modifiers = "static final", is_static = true))]
        // static field: counter:Ljava/util/concurrent/atomic/AtomicInteger;
        pub fn counter() -> AtomicInteger {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.counter:Ljava/util/concurrent/atomic/AtomicInteger;")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "defineProxyClass", descriptor = "(Ljava/lang/reflect/Proxy$ProxyBuilder$ProxyClassContext;Ljava/util/List;)Ljava/lang/Class;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/reflect/Proxy$ProxyBuilder$ProxyClassContext;Ljava/util/List<Ljava/lang/Class<*>;>;)Ljava/lang/Class<*>;")]
        pub fn defineProxyClass(context: Object, interfaces: Object) -> Result<Object> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.defineProxyClass:(Ljava/lang/reflect/Proxy$ProxyBuilder$ProxyClassContext;Ljava/util/List;)Ljava/lang/Class;")
        }

        #[java_method(name = "isProxyClass", descriptor = "(Ljava/lang/Class;)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z")]
        pub fn isProxyClass(mut c: Object) -> Result<bool> {
            let _t0 = Proxy_ProxyBuilder::reverseProxyCache().__super().sub(Clone::clone(&c))?;
            let _vdispatch1: ClassLoader = if let Some(__f) = c.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<ClassLoader>>>() { (__f)()? } else { Default::default() };
            let _t2 = _t0.__super().get(Clone::clone(&_vdispatch1))?;
            let _t3: bool = Objects::equals(Clone::clone(&_t2), Boolean::TRUE().into())?;
            Ok(_t3)
        }

        #[java_method(name = "isExportedType", descriptor = "(Ljava/lang/Class;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z")]
        pub fn isExportedType(c: Object) -> Result<bool> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.isExportedType:(Ljava/lang/Class;)Z")
        }

        #[java_method(name = "isPackagePrivateType", descriptor = "(Ljava/lang/Class;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z")]
        pub fn isPackagePrivateType(c: Object) -> Result<bool> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.isPackagePrivateType:(Ljava/lang/Class;)Z")
        }

        #[java_method(name = "toDetails", descriptor = "(Ljava/lang/Class;)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Ljava/lang/String;")]
        pub fn toDetails(c: Object) -> Result<String> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.toDetails:(Ljava/lang/Class;)Ljava/lang/String;")
        }

        #[java_method(name = "trace", descriptor = "(Ljava/lang/String;Ljava/lang/Module;Ljava/lang/ClassLoader;Ljava/util/List;)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;Ljava/lang/Module;Ljava/lang/ClassLoader;Ljava/util/List<Ljava/lang/Class<*>;>;)V")]
        pub fn trace(cn: String, module: Object, loader: ClassLoader, interfaces: Object) -> Result<()> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.trace:(Ljava/lang/String;Ljava/lang/Module;Ljava/lang/ClassLoader;Ljava/util/List;)V")
        }

        #[java_method(name = "isDebug", descriptor = "()Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDebug() -> Result<bool> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.isDebug:()Z")
        }

        #[java_method(name = "isDebug", descriptor = "(Ljava/lang/String;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDebug_str(flag: String) -> Result<bool> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.isDebug:(Ljava/lang/String;)Z")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/ClassLoader;Ljava/util/List;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ClassLoader;Ljava/util/List<Ljava/lang/Class<*>;>;)V")]
        pub fn new_classl_list(loader: ClassLoader, interfaces: Object) -> Result<Self> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.<init>:(Ljava/lang/ClassLoader;Ljava/util/List;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/Class;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ClassLoader;Ljava/lang/Class<*>;)V")]
        pub fn new_classl_class(loader: ClassLoader, intf: Object) -> Result<Self> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.<init>:(Ljava/lang/ClassLoader;Ljava/lang/Class;)V")
        }

        #[java_method(name = "build", descriptor = "()Ljava/lang/reflect/Constructor;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/reflect/Constructor<*>;")]
        pub fn build(&self) -> Result<Object> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.build:()Ljava/lang/reflect/Constructor;")
        }

        #[java_method(name = "validateProxyInterfaces", descriptor = "(Ljava/lang/ClassLoader;Ljava/util/List;Ljava/util/Set;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ClassLoader;Ljava/util/List<Ljava/lang/Class<*>;>;Ljava/util/Set<Ljava/lang/Class<*>;>;)V")]
        pub fn validateProxyInterfaces(loader: ClassLoader, interfaces: Object, refTypes: Object) -> Result<()> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.validateProxyInterfaces:(Ljava/lang/ClassLoader;Ljava/util/List;Ljava/util/Set;)V")
        }

        #[java_method(name = "referencedTypes", descriptor = "(Ljava/lang/ClassLoader;Ljava/util/List;)Ljava/util/Set;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ClassLoader;Ljava/util/List<Ljava/lang/Class<*>;>;)Ljava/util/Set<Ljava/lang/Class<*>;>;")]
        pub fn referencedTypes(loader: ClassLoader, interfaces: Object) -> Result<Object> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.referencedTypes:(Ljava/lang/ClassLoader;Ljava/util/List;)Ljava/util/Set;")
        }

        #[java_method(name = "addElementTypes", descriptor = "(Ljava/util/HashSet;[Ljava/lang/Class;)V", access = "private", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashSet<Ljava/lang/Class<*>;>;[Ljava/lang/Class<*>;)V")]
        pub fn addElementTypes(types: HashSet<Object>, classes: Rc<RefCell<Vec<Object>>>) -> Result<()> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.addElementTypes:(Ljava/util/HashSet;[Ljava/lang/Class;)V")
        }

        #[java_method(name = "addElementType", descriptor = "(Ljava/util/HashSet;Ljava/lang/Class;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/HashSet<Ljava/lang/Class<*>;>;Ljava/lang/Class<*>;)V")]
        pub fn addElementType(types: HashSet<Object>, cls: Object) -> Result<()> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.addElementType:(Ljava/util/HashSet;Ljava/lang/Class;)V")
        }

        #[java_method(name = "proxyClassContext", descriptor = "(Ljava/lang/ClassLoader;Ljava/util/List;Ljava/util/Set;)Ljava/lang/reflect/Proxy$ProxyBuilder$ProxyClassContext;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ClassLoader;Ljava/util/List<Ljava/lang/Class<*>;>;Ljava/util/Set<Ljava/lang/Class<*>;>;)Ljava/lang/reflect/Proxy$ProxyBuilder$ProxyClassContext;")]
        pub fn proxyClassContext(loader: ClassLoader, interfaces: Object, refTypes: Object) -> Result<Object> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.proxyClassContext:(Ljava/lang/ClassLoader;Ljava/util/List;Ljava/util/Set;)Ljava/lang/reflect/Proxy$ProxyBuilder$ProxyClassContext;")
        }

        #[java_method(name = "ensureAccess", descriptor = "(Ljava/lang/Module;Ljava/lang/Class;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Module;Ljava/lang/Class<*>;)V")]
        pub fn ensureAccess(target: Object, c: Object) -> Result<()> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.ensureAccess:(Ljava/lang/Module;Ljava/lang/Class;)V")
        }

        #[java_method(name = "ensureVisible", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/Class;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ClassLoader;Ljava/lang/Class<*>;)V")]
        pub fn ensureVisible(ld: ClassLoader, c: Object) -> Result<()> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.ensureVisible:(Ljava/lang/ClassLoader;Ljava/lang/Class;)V")
        }

        #[java_method(name = "getElementType", descriptor = "(Ljava/lang/Class;)Ljava/lang/Class;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Ljava/lang/Class<*>;")]
        pub fn getElementType(type_: Object) -> Result<Object> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.getElementType:(Ljava/lang/Class;)Ljava/lang/Class;")
        }

        #[java_method(name = "getDynamicModule", descriptor = "(Ljava/lang/ClassLoader;)Ljava/lang/Module;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDynamicModule(loader: ClassLoader) -> Result<Object> {
            panic!("stub: java/lang/reflect/Proxy$ProxyBuilder.getDynamicModule:(Ljava/lang/ClassLoader;)Ljava/lang/Module;")
        }
    }
}
