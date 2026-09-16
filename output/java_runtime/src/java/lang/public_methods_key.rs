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
use crate::jdk::internal::reflect::ReflectionFactory;
use crate::jdk::internal::reflect::ReflectionFactory_GetReflectionFactoryAction;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/PublicMethods$Key"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "PublicMethods.java"]
    #[inner_classes     = "java/lang/PublicMethods$Key:java/lang/PublicMethods:Key:26;jdk/internal/reflect/ReflectionFactory$GetReflectionFactoryAction:jdk/internal/reflect/ReflectionFactory:GetReflectionFactoryAction:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/lang/PublicMethods$Key"]
    #[has_hash_code_method = true]

    pub struct PublicMethods_Key {
        #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub name: String,
        #[cfg_attr(any(), java_field(name = "ptypes", descriptor = "[Ljava/lang/Class;", access = "private", modifiers = "final", is_static = false, generic_signature = "[Ljava/lang/Class<*>;"))]
        pub ptypes: Rc<RefCell<Vec<Class<Object>>>>,
    }

    impl PublicMethods_Key {
        #[cfg_attr(any(), java_field(name = "reflectionFactory", descriptor = "Ljdk/internal/reflect/ReflectionFactory;", access = "private", modifiers = "static final", is_static = true))]
        // static field: reflectionFactory:Ljdk/internal/reflect/ReflectionFactory;
        pub fn reflectionFactory() -> ReflectionFactory {
            panic!("stub: java/lang/PublicMethods$Key.reflectionFactory:Ljdk/internal/reflect/ReflectionFactory;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/reflect/Method;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(method: Method) -> Result<Self> {
            panic!("stub: java/lang/PublicMethods$Key.<init>:(Ljava/lang/reflect/Method;)V")
        }

        #[java_method(name = "matches", descriptor = "(Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class;)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class<*>;)Z")]
        pub fn matches(mut method: Method, mut name: String, mut ptypes: Rc<RefCell<Vec<Object>>>) -> Result<bool> {
            let _t0 = method.getName()?;
            let _t1 = _t0.equals(Object::from_any(name.clone()))?;
            let mut _merged4: bool;
            if _t1 {
                let _t2 = PublicMethods_Key::reflectionFactory().getExecutableSharedParameterTypes(Clone::clone(&method).into())?;
                let _t3: bool = Arrays::equals_arr_obj_arr_obj(Clone::clone(&_t2), Clone::clone(&ptypes))?;
                _merged4 = !(!(_t3));
            } else {
                _merged4 = (0i32 != 0);
            }
            Ok(_merged4)
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, o: Object) -> Result<bool> {
            panic!("stub: java/lang/PublicMethods$Key.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }
    }
}
