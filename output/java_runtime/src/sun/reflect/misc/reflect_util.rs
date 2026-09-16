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
use crate::jdk::internal::reflect::Reflection;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "sun/reflect/misc/ReflectUtil"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ReflectUtil.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;sun/reflect/misc/ReflectUtil"]

    pub struct ReflectUtil;

    impl ReflectUtil {
        #[cfg_attr(any(), java_field(name = "PROXY_PACKAGE", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "com.sun.proxy"))]
        // static field: PROXY_PACKAGE:Ljava/lang/String;
        pub fn PROXY_PACKAGE() -> String {
            String::from("com.sun.proxy")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: sun/reflect/misc/ReflectUtil.<init>:()V")
        }

        #[java_method(name = "forName", descriptor = "(Ljava/lang/String;)Ljava/lang/Class;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/ClassNotFoundException", generic_signature = "(Ljava/lang/String;)Ljava/lang/Class<*>;")]
        pub fn forName(name: String) -> Result<Object> {
            panic!("stub: sun/reflect/misc/ReflectUtil.forName:(Ljava/lang/String;)Ljava/lang/Class;")
        }

        #[java_method(name = "ensureMemberAccess", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Object;I)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalAccessException", generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class<*>;Ljava/lang/Object;I)V")]
        pub fn ensureMemberAccess(currentClass: Object, memberClass: Object, target: Object, modifiers: i32) -> Result<()> {
            panic!("stub: sun/reflect/misc/ReflectUtil.ensureMemberAccess:(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Object;I)V")
        }

        #[java_method(name = "conservativeCheckMemberAccess", descriptor = "(Ljava/lang/reflect/Member;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException")]
        pub fn conservativeCheckMemberAccess(m: Object) -> Result<()> {
            panic!("stub: sun/reflect/misc/ReflectUtil.conservativeCheckMemberAccess:(Ljava/lang/reflect/Member;)V")
        }

        #[java_method(name = "checkPackageAccess", descriptor = "(Ljava/lang/Class;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)V")]
        pub fn checkPackageAccess_class(clazz: Object) -> Result<()> {
            panic!("stub: sun/reflect/misc/ReflectUtil.checkPackageAccess:(Ljava/lang/Class;)V")
        }

        #[java_method(name = "privateCheckPackageAccess", descriptor = "(Ljava/lang/SecurityManager;Ljava/lang/Class;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/SecurityManager;Ljava/lang/Class<*>;)V")]
        pub fn privateCheckPackageAccess(s: SecurityManager, clazz: Object) -> Result<()> {
            panic!("stub: sun/reflect/misc/ReflectUtil.privateCheckPackageAccess:(Ljava/lang/SecurityManager;Ljava/lang/Class;)V")
        }

        #[java_method(name = "checkPackageAccess", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkPackageAccess_str(name: String) -> Result<()> {
            panic!("stub: sun/reflect/misc/ReflectUtil.checkPackageAccess:(Ljava/lang/String;)V")
        }

        #[java_method(name = "isPackageAccessible", descriptor = "(Ljava/lang/Class;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z")]
        pub fn isPackageAccessible(clazz: Object) -> Result<bool> {
            panic!("stub: sun/reflect/misc/ReflectUtil.isPackageAccessible:(Ljava/lang/Class;)Z")
        }

        #[java_method(name = "isAncestor", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isAncestor(p: ClassLoader, cl: ClassLoader) -> Result<bool> {
            panic!("stub: sun/reflect/misc/ReflectUtil.isAncestor:(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)Z")
        }

        #[java_method(name = "needsPackageAccessCheck", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn needsPackageAccessCheck(from: ClassLoader, to: ClassLoader) -> Result<bool> {
            panic!("stub: sun/reflect/misc/ReflectUtil.needsPackageAccessCheck:(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)Z")
        }

        #[java_method(name = "checkProxyPackageAccess", descriptor = "(Ljava/lang/Class;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)V")]
        pub fn checkProxyPackageAccess_class(clazz: Object) -> Result<()> {
            panic!("stub: sun/reflect/misc/ReflectUtil.checkProxyPackageAccess:(Ljava/lang/Class;)V")
        }

        #[java_method(name = "privateCheckProxyPackageAccess", descriptor = "(Ljava/lang/SecurityManager;Ljava/lang/Class;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/SecurityManager;Ljava/lang/Class<*>;)V")]
        pub fn privateCheckProxyPackageAccess(s: SecurityManager, clazz: Object) -> Result<()> {
            panic!("stub: sun/reflect/misc/ReflectUtil.privateCheckProxyPackageAccess:(Ljava/lang/SecurityManager;Ljava/lang/Class;)V")
        }

        #[java_method(name = "checkProxyPackageAccess", descriptor = "(Ljava/lang/ClassLoader;[Ljava/lang/Class;)V", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ClassLoader;[Ljava/lang/Class<*>;)V")]
        pub fn checkProxyPackageAccess_classl_arr_cla(ccl: ClassLoader, interfaces: Rc<RefCell<Vec<Object>>>) -> Result<()> {
            panic!("stub: sun/reflect/misc/ReflectUtil.checkProxyPackageAccess:(Ljava/lang/ClassLoader;[Ljava/lang/Class;)V")
        }

        #[java_method(name = "isNonPublicProxyClass", descriptor = "(Ljava/lang/Class;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z")]
        pub fn isNonPublicProxyClass(cls: Object) -> Result<bool> {
            panic!("stub: sun/reflect/misc/ReflectUtil.isNonPublicProxyClass:(Ljava/lang/Class;)Z")
        }

        #[java_method(name = "checkProxyMethod", descriptor = "(Ljava/lang/Object;Ljava/lang/reflect/Method;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkProxyMethod(proxy: Object, method: Method) -> Result<()> {
            panic!("stub: sun/reflect/misc/ReflectUtil.checkProxyMethod:(Ljava/lang/Object;Ljava/lang/reflect/Method;)V")
        }

        #[java_method(name = "isSuperInterface", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class<*>;)Z")]
        pub fn isSuperInterface(c: Object, intf: Object) -> Result<bool> {
            panic!("stub: sun/reflect/misc/ReflectUtil.isSuperInterface:(Ljava/lang/Class;Ljava/lang/Class;)Z")
        }
    }
}
