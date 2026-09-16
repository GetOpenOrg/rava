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
    #[binary_name       = "java/lang/Class$EnclosingMethodInfo"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Class.java"]
    #[inner_classes     = "java/lang/Class$EnclosingMethodInfo:java/lang/Class:EnclosingMethodInfo:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Class$EnclosingMethodInfo;java/lang/Object"]

    pub struct Class_EnclosingMethodInfo {
        #[cfg_attr(any(), java_field(name = "enclosingClass", descriptor = "Ljava/lang/Class;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/lang/Class<*>;"))]
        pub enclosingClass: Class<Object>,
        #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub name: String,
        #[cfg_attr(any(), java_field(name = "descriptor", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub descriptor: String,
    }

    impl Class_EnclosingMethodInfo {
        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "validate", descriptor = "([Ljava/lang/Object;)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn validate(mut enclosingInfo: Rc<RefCell<Vec<Object>>>) -> Result<()> {
            if (enclosingInfo.borrow().len() as i32) != 3i32 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut enclosingClass = Clone::clone(&enclosingInfo.borrow()[0i32 as usize]);
            if _is_jnull(&enclosingClass) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut name = (Clone::clone(&enclosingInfo.borrow()[1i32 as usize])).downcast::<String>();
            let mut descriptor = (Clone::clone(&enclosingInfo.borrow()[2i32 as usize])).downcast::<String>();
            if Object::from_any(name.clone()) != Object::from_any(descriptor.clone()) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "<init>", descriptor = "([Ljava/lang/Object;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut enclosingInfo: Rc<RefCell<Vec<Object>>>) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Class_EnclosingMethodInfo::validate(Clone::clone(&enclosingInfo))?;
            this.__set_enclosingClass(Clone::clone(&enclosingInfo.borrow()[0i32 as usize]));
            this.__set_name((Clone::clone(&enclosingInfo.borrow()[1i32 as usize])).downcast::<String>());
            this.__set_descriptor((Clone::clone(&enclosingInfo.borrow()[2i32 as usize])).downcast::<String>());
            Ok(this)
        }

        #[java_method(name = "isPartial", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isPartial(&self) -> Result<bool> {
            panic!("stub: java/lang/Class$EnclosingMethodInfo.isPartial:()Z")
        }

        #[java_method(name = "isConstructor", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isConstructor(&self) -> Result<bool> {
            panic!("stub: java/lang/Class$EnclosingMethodInfo.isConstructor:()Z")
        }

        #[java_method(name = "isMethod", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isMethod(&self) -> Result<bool> {
            panic!("stub: java/lang/Class$EnclosingMethodInfo.isMethod:()Z")
        }

        #[java_method(name = "getEnclosingClass", descriptor = "()Ljava/lang/Class;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;")]
        pub fn getEnclosingClass(&self) -> Result<Object> {
            let this = self;
            Ok(Object::from_any(this.__get_enclosingClass().clone()))
        }

        #[java_method(name = "getName", descriptor = "()Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getName(&self) -> Result<String> {
            panic!("stub: java/lang/Class$EnclosingMethodInfo.getName:()Ljava/lang/String;")
        }

        #[java_method(name = "getDescriptor", descriptor = "()Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDescriptor(&self) -> Result<String> {
            panic!("stub: java/lang/Class$EnclosingMethodInfo.getDescriptor:()Ljava/lang/String;")
        }
    }
}
