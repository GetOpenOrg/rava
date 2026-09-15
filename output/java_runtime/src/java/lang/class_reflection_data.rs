#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/Class$ReflectionData",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "package",
    modifiers         = "",
    generic_signature = "<T:Ljava/lang/Object;>Ljava/lang/Object;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Class.java",
    inner_classes     = "java/lang/Class$ReflectionData:java/lang/Class:ReflectionData:10",
    all_supertypes    = "java/lang/Class$ReflectionData;java/lang/Object",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Class_ReflectionData<T: Clone + Default + 'static> {
    #[cfg_attr(any(), java_field(name = "declaredFields", descriptor = "[Ljava/lang/reflect/Field;", access = "package", modifiers = "volatile", is_static = false))]
    pub declaredFields: JField<Rc<RefCell<Vec<Object>>>>,
    #[cfg_attr(any(), java_field(name = "publicFields", descriptor = "[Ljava/lang/reflect/Field;", access = "package", modifiers = "volatile", is_static = false))]
    pub publicFields: JField<Rc<RefCell<Vec<Object>>>>,
    #[cfg_attr(any(), java_field(name = "declaredMethods", descriptor = "[Ljava/lang/reflect/Method;", access = "package", modifiers = "volatile", is_static = false))]
    pub declaredMethods: JField<Rc<RefCell<Vec<Object>>>>,
    #[cfg_attr(any(), java_field(name = "publicMethods", descriptor = "[Ljava/lang/reflect/Method;", access = "package", modifiers = "volatile", is_static = false))]
    pub publicMethods: JField<Rc<RefCell<Vec<Object>>>>,
    #[cfg_attr(any(), java_field(name = "declaredConstructors", descriptor = "[Ljava/lang/reflect/Constructor;", access = "package", modifiers = "volatile", is_static = false, generic_signature = "[Ljava/lang/reflect/Constructor<TT;>;"))]
    pub declaredConstructors: JField<Rc<RefCell<Vec<Object>>>>,
    #[cfg_attr(any(), java_field(name = "publicConstructors", descriptor = "[Ljava/lang/reflect/Constructor;", access = "package", modifiers = "volatile", is_static = false, generic_signature = "[Ljava/lang/reflect/Constructor<TT;>;"))]
    pub publicConstructors: JField<Rc<RefCell<Vec<Object>>>>,
    #[cfg_attr(any(), java_field(name = "declaredPublicFields", descriptor = "[Ljava/lang/reflect/Field;", access = "package", modifiers = "volatile", is_static = false))]
    pub declaredPublicFields: JField<Rc<RefCell<Vec<Object>>>>,
    #[cfg_attr(any(), java_field(name = "declaredPublicMethods", descriptor = "[Ljava/lang/reflect/Method;", access = "package", modifiers = "volatile", is_static = false))]
    pub declaredPublicMethods: JField<Rc<RefCell<Vec<Object>>>>,
    #[cfg_attr(any(), java_field(name = "interfaces", descriptor = "[Ljava/lang/Class;", access = "package", modifiers = "volatile", is_static = false, generic_signature = "[Ljava/lang/Class<*>;"))]
    pub interfaces: JField<Rc<RefCell<Vec<Class<Object>>>>>,
    #[cfg_attr(any(), java_field(name = "simpleName", descriptor = "Ljava/lang/String;", is_static = false))]
    pub simpleName: JField<String>,
    #[cfg_attr(any(), java_field(name = "canonicalName", descriptor = "Ljava/lang/String;", is_static = false))]
    pub canonicalName: JField<String>,
    #[cfg_attr(any(), java_field(name = "redefinedCount", descriptor = "I", access = "package", modifiers = "final", is_static = false))]
    pub redefinedCount: JField<i32>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + Default + 'static> Class_ReflectionData<T> {
    #[cfg_attr(any(), java_field(name = "NULL_SENTINEL", descriptor = "Ljava/lang/String;", access = "package", modifiers = "static final", is_static = true))]
    // static field: NULL_SENTINEL:Ljava/lang/String;
    pub fn NULL_SENTINEL() -> String {
        panic!("stub: java/lang/Class$ReflectionData.NULL_SENTINEL:Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new(mut redefinedCount: i32) -> Result<Self> {
        let mut this = Self { declaredFields: JField::new(Default::default()), publicFields: JField::new(Default::default()), declaredMethods: JField::new(Default::default()), publicMethods: JField::new(Default::default()), declaredConstructors: JField::new(Default::default()), publicConstructors: JField::new(Default::default()), declaredPublicFields: JField::new(Default::default()), declaredPublicMethods: JField::new(Default::default()), interfaces: JField::new(Default::default()), simpleName: JField::new(String::default()), canonicalName: JField::new(String::default()), redefinedCount: JField::new(0), _phantom: std::marker::PhantomData, ..Default::default() };
        /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
        this.redefinedCount.set(redefinedCount);
        Ok(this)
    }
}
