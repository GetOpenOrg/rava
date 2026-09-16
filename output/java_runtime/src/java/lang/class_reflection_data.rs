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
    #[binary_name       = "java/lang/Class$ReflectionData"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "<T:Ljava/lang/Object;>Ljava/lang/Object;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Class.java"]
    #[inner_classes     = "java/lang/Class$ReflectionData:java/lang/Class:ReflectionData:10"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Class$ReflectionData;java/lang/Object"]

    pub struct Class_ReflectionData<T: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "declaredFields", descriptor = "[Ljava/lang/reflect/Field;", access = "package", modifiers = "volatile", is_static = false))]
        pub declaredFields: Rc<RefCell<Vec<Object>>>,
        #[cfg_attr(any(), java_field(name = "publicFields", descriptor = "[Ljava/lang/reflect/Field;", access = "package", modifiers = "volatile", is_static = false))]
        pub publicFields: Rc<RefCell<Vec<Object>>>,
        #[cfg_attr(any(), java_field(name = "declaredMethods", descriptor = "[Ljava/lang/reflect/Method;", access = "package", modifiers = "volatile", is_static = false))]
        pub declaredMethods: Rc<RefCell<Vec<Method>>>,
        #[cfg_attr(any(), java_field(name = "publicMethods", descriptor = "[Ljava/lang/reflect/Method;", access = "package", modifiers = "volatile", is_static = false))]
        pub publicMethods: Rc<RefCell<Vec<Method>>>,
        #[cfg_attr(any(), java_field(name = "declaredConstructors", descriptor = "[Ljava/lang/reflect/Constructor;", access = "package", modifiers = "volatile", is_static = false, generic_signature = "[Ljava/lang/reflect/Constructor<TT;>;"))]
        pub declaredConstructors: Rc<RefCell<Vec<Object>>>,
        #[cfg_attr(any(), java_field(name = "publicConstructors", descriptor = "[Ljava/lang/reflect/Constructor;", access = "package", modifiers = "volatile", is_static = false, generic_signature = "[Ljava/lang/reflect/Constructor<TT;>;"))]
        pub publicConstructors: Rc<RefCell<Vec<Object>>>,
        #[cfg_attr(any(), java_field(name = "declaredPublicFields", descriptor = "[Ljava/lang/reflect/Field;", access = "package", modifiers = "volatile", is_static = false))]
        pub declaredPublicFields: Rc<RefCell<Vec<Object>>>,
        #[cfg_attr(any(), java_field(name = "declaredPublicMethods", descriptor = "[Ljava/lang/reflect/Method;", access = "package", modifiers = "volatile", is_static = false))]
        pub declaredPublicMethods: Rc<RefCell<Vec<Method>>>,
        #[cfg_attr(any(), java_field(name = "interfaces", descriptor = "[Ljava/lang/Class;", access = "package", modifiers = "volatile", is_static = false, generic_signature = "[Ljava/lang/Class<*>;"))]
        pub interfaces: Rc<RefCell<Vec<Class<Object>>>>,
        #[cfg_attr(any(), java_field(name = "simpleName", descriptor = "Ljava/lang/String;", is_static = false))]
        pub simpleName: String,
        #[cfg_attr(any(), java_field(name = "canonicalName", descriptor = "Ljava/lang/String;", is_static = false))]
        pub canonicalName: String,
        #[cfg_attr(any(), java_field(name = "redefinedCount", descriptor = "I", access = "package", modifiers = "final", is_static = false))]
        pub redefinedCount: i32,
    }

    impl<T> Class_ReflectionData<T> {
        #[cfg_attr(any(), java_field(name = "NULL_SENTINEL", descriptor = "Ljava/lang/String;", access = "package", modifiers = "static final", is_static = true))]
        // static field: NULL_SENTINEL:Ljava/lang/String;
        pub fn NULL_SENTINEL() -> String {
            panic!("stub: java/lang/Class$ReflectionData.NULL_SENTINEL:Ljava/lang/String;")
        }

        #[java_method(name = "<init>", descriptor = "(I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut redefinedCount: i32) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_redefinedCount(redefinedCount);
            Ok(this)
        }
    }
}
