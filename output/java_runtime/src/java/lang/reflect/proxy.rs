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
use crate::jdk::internal::reflect::Reflection;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/reflect/Proxy"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Proxy.java"]
    #[inner_classes     = "jdk/internal/loader/AbstractClassLoaderValue$Sub:jdk/internal/loader/AbstractClassLoaderValue:Sub:17;java/lang/reflect/Proxy$ProxyBuilder:java/lang/reflect/Proxy:ProxyBuilder:26;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25;java/lang/reflect/Proxy$InvocationException:java/lang/reflect/Proxy:InvocationException:8;java/lang/reflect/Proxy$2:::0;java/lang/reflect/Proxy$1:::0;java/lang/reflect/Proxy$ProxyBuilder$ProxyClassContext:java/lang/reflect/Proxy$ProxyBuilder:ProxyClassContext:26;java/lang/reflect/Proxy$ProxyBuilder$1:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/lang/reflect/Proxy"]

    pub struct Proxy {
        #[cfg_attr(any(), java_field(name = "h", descriptor = "Ljava/lang/reflect/InvocationHandler;", access = "protected", modifiers = "", is_static = false))]
        pub h: Object,
    }

    impl Proxy {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-2222568056686623797"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -2222568056686623797i64
        }

        #[cfg_attr(any(), java_field(name = "constructorParams", descriptor = "[Ljava/lang/Class;", access = "private", modifiers = "static final", is_static = true, generic_signature = "[Ljava/lang/Class<*>;"))]
        // static field: constructorParams:[Ljava/lang/Class;
        pub fn constructorParams() -> Rc<RefCell<Vec<Class<Object>>>> {
            panic!("stub: java/lang/reflect/Proxy.constructorParams:[Ljava/lang/Class;")
        }

        #[cfg_attr(any(), java_field(name = "proxyCache", descriptor = "Ljdk/internal/loader/ClassLoaderValue;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljdk/internal/loader/ClassLoaderValue<Ljava/lang/reflect/Constructor<*>;>;"))]
        // static field: proxyCache:Ljdk/internal/loader/ClassLoaderValue;
        pub fn proxyCache() -> ClassLoaderValue<Object> {
            panic!("stub: java/lang/reflect/Proxy.proxyCache:Ljdk/internal/loader/ClassLoaderValue;")
        }

        #[cfg_attr(any(), java_field(name = "PROXY_PACKAGE_PREFIX", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "com.sun.proxy"))]
        // static field: PROXY_PACKAGE_PREFIX:Ljava/lang/String;
        pub fn PROXY_PACKAGE_PREFIX() -> String {
            String::from("com.sun.proxy")
        }

        #[cfg_attr(any(), java_field(name = "DEFAULT_METHODS_MAP", descriptor = "Ljava/lang/ClassValue;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/lang/ClassValue<Ljava/util/concurrent/ConcurrentHashMap<Ljava/lang/reflect/Method;Ljava/lang/invoke/MethodHandle;>;>;"))]
        // static field: DEFAULT_METHODS_MAP:Ljava/lang/ClassValue;
        pub fn DEFAULT_METHODS_MAP() -> Object {
            panic!("stub: java/lang/reflect/Proxy.DEFAULT_METHODS_MAP:Ljava/lang/ClassValue;")
        }

        #[cfg_attr(any(), java_field(name = "EMPTY_ARGS", descriptor = "[Ljava/lang/Object;", access = "package", modifiers = "static final", is_static = true))]
        // static field: EMPTY_ARGS:[Ljava/lang/Object;
        pub fn EMPTY_ARGS() -> Rc<RefCell<Vec<Object>>> {
            Rc::new(RefCell::new(Vec::new()))
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/reflect/Proxy.<init>:()V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/reflect/InvocationHandler;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_invoca(h: Object) -> Result<Self> {
            panic!("stub: java/lang/reflect/Proxy.<init>:(Ljava/lang/reflect/InvocationHandler;)V")
        }

        #[java_method(name = "getProxyClass", descriptor = "(Ljava/lang/ClassLoader;[Ljava/lang/Class;)Ljava/lang/Class;", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException", generic_signature = "(Ljava/lang/ClassLoader;[Ljava/lang/Class<*>;)Ljava/lang/Class<*>;", is_deprecated = true)]
        pub fn getProxyClass(loader: ClassLoader, interfaces: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
            panic!("stub: java/lang/reflect/Proxy.getProxyClass:(Ljava/lang/ClassLoader;[Ljava/lang/Class;)Ljava/lang/Class;")
        }

        #[java_method(name = "getProxyConstructor", descriptor = "(Ljava/lang/Class;Ljava/lang/ClassLoader;[Ljava/lang/Class;)Ljava/lang/reflect/Constructor;", access = "private", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/ClassLoader;[Ljava/lang/Class<*>;)Ljava/lang/reflect/Constructor<*>;")]
        pub fn getProxyConstructor(caller: Object, loader: ClassLoader, interfaces: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
            panic!("stub: java/lang/reflect/Proxy.getProxyConstructor:(Ljava/lang/Class;Ljava/lang/ClassLoader;[Ljava/lang/Class;)Ljava/lang/reflect/Constructor;")
        }

        #[java_method(name = "checkProxyAccess", descriptor = "(Ljava/lang/Class;Ljava/lang/ClassLoader;[Ljava/lang/Class;)V", access = "private", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/ClassLoader;[Ljava/lang/Class<*>;)V")]
        pub fn checkProxyAccess(caller: Object, loader: ClassLoader, interfaces: Rc<RefCell<Vec<Object>>>) -> Result<()> {
            panic!("stub: java/lang/reflect/Proxy.checkProxyAccess:(Ljava/lang/Class;Ljava/lang/ClassLoader;[Ljava/lang/Class;)V")
        }

        #[java_method(name = "newProxyInstance", descriptor = "(Ljava/lang/ClassLoader;[Ljava/lang/Class;Ljava/lang/reflect/InvocationHandler;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ClassLoader;[Ljava/lang/Class<*>;Ljava/lang/reflect/InvocationHandler;)Ljava/lang/Object;")]
        pub fn newProxyInstance_classl_arr_cla_invoca(loader: ClassLoader, interfaces: Rc<RefCell<Vec<Object>>>, h: Object) -> Result<Object> {
            panic!("stub: java/lang/reflect/Proxy.newProxyInstance:(Ljava/lang/ClassLoader;[Ljava/lang/Class;Ljava/lang/reflect/InvocationHandler;)Ljava/lang/Object;")
        }

        #[java_method(name = "newProxyInstance", descriptor = "(Ljava/lang/Class;Ljava/lang/reflect/Constructor;Ljava/lang/reflect/InvocationHandler;)Ljava/lang/Object;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/reflect/Constructor<*>;Ljava/lang/reflect/InvocationHandler;)Ljava/lang/Object;")]
        pub fn newProxyInstance_class_constr_invoca(caller: Object, cons: Object, h: Object) -> Result<Object> {
            panic!("stub: java/lang/reflect/Proxy.newProxyInstance:(Ljava/lang/Class;Ljava/lang/reflect/Constructor;Ljava/lang/reflect/InvocationHandler;)Ljava/lang/Object;")
        }

        #[java_method(name = "checkNewProxyPermission", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class<*>;)V")]
        pub fn checkNewProxyPermission(caller: Object, proxyClass: Object) -> Result<()> {
            panic!("stub: java/lang/reflect/Proxy.checkNewProxyPermission:(Ljava/lang/Class;Ljava/lang/Class;)V")
        }

        #[java_method(name = "getLoader", descriptor = "(Ljava/lang/Module;)Ljava/lang/ClassLoader;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLoader(m: Object) -> Result<ClassLoader> {
            panic!("stub: java/lang/reflect/Proxy.getLoader:(Ljava/lang/Module;)Ljava/lang/ClassLoader;")
        }

        #[java_method(name = "isProxyClass", descriptor = "(Ljava/lang/Class;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z")]
        pub fn isProxyClass(mut cl: Object) -> Result<bool> {
            let _vdispatch0: bool = if let Some(__f) = Object::default().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Clone::clone(&cl))? } else { Default::default() };
            let mut _merged2: bool;
            if _vdispatch0 {
                let _t1: bool = Proxy_ProxyBuilder::isProxyClass(Clone::clone(&cl))?;
                _merged2 = !(!(_t1));
            } else {
                _merged2 = (0i32 != 0);
            }
            Ok(_merged2)
        }

        #[java_method(name = "getInvocationHandler", descriptor = "(Ljava/lang/Object;)Ljava/lang/reflect/InvocationHandler;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalArgumentException")]
        pub fn getInvocationHandler(proxy: Object) -> Result<Object> {
            panic!("stub: java/lang/reflect/Proxy.getInvocationHandler:(Ljava/lang/Object;)Ljava/lang/reflect/InvocationHandler;")
        }

        #[java_method(name = "defaultMethodMap", descriptor = "(Ljava/lang/Class;)Ljava/util/concurrent/ConcurrentHashMap;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Ljava/util/concurrent/ConcurrentHashMap<Ljava/lang/reflect/Method;Ljava/lang/invoke/MethodHandle;>;")]
        pub fn defaultMethodMap(proxyClass: Object) -> Result<ConcurrentHashMap<Object, Object>> {
            panic!("stub: java/lang/reflect/Proxy.defaultMethodMap:(Ljava/lang/Class;)Ljava/util/concurrent/ConcurrentHashMap;")
        }

        #[java_method(name = "defaultMethodHandle", descriptor = "(Ljava/lang/Class;Ljava/lang/reflect/Method;)Ljava/lang/invoke/MethodHandle;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<+Ljava/lang/reflect/Proxy;>;Ljava/lang/reflect/Method;)Ljava/lang/invoke/MethodHandle;")]
        pub fn defaultMethodHandle(proxyClass: Object, method: Method) -> Result<Object> {
            panic!("stub: java/lang/reflect/Proxy.defaultMethodHandle:(Ljava/lang/Class;Ljava/lang/reflect/Method;)Ljava/lang/invoke/MethodHandle;")
        }

        #[java_method(name = "findProxyInterfaceOrElseThrow", descriptor = "(Ljava/lang/Class;Ljava/lang/reflect/Method;)Ljava/lang/Class;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/reflect/Method;)Ljava/lang/Class<*>;")]
        pub fn findProxyInterfaceOrElseThrow(proxyClass: Object, method: Method) -> Result<Object> {
            panic!("stub: java/lang/reflect/Proxy.findProxyInterfaceOrElseThrow:(Ljava/lang/Class;Ljava/lang/reflect/Method;)Ljava/lang/Class;")
        }

        #[java_method(name = "proxyClassLookup", descriptor = "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandles$Lookup;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/Class<*>;)Ljava/lang/invoke/MethodHandles$Lookup;")]
        pub fn proxyClassLookup(caller: Object, proxyClass: Object) -> Result<Object> {
            panic!("stub: java/lang/reflect/Proxy.proxyClassLookup:(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandles$Lookup;")
        }

        #[java_method(name = "invokeDefault", descriptor = "(Ljava/lang/Object;Ljava/lang/reflect/Method;[Ljava/lang/Object;Ljava/lang/Class;)Ljava/lang/Object;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/Throwable", generic_signature = "(Ljava/lang/Object;Ljava/lang/reflect/Method;[Ljava/lang/Object;Ljava/lang/Class<*>;)Ljava/lang/Object;")]
        pub fn invokeDefault(proxy: Object, method: Method, args: Rc<RefCell<Vec<Object>>>, caller: Object) -> Result<Object> {
            panic!("stub: java/lang/reflect/Proxy.invokeDefault:(Ljava/lang/Object;Ljava/lang/reflect/Method;[Ljava/lang/Object;Ljava/lang/Class;)Ljava/lang/Object;")
        }
    }
}
