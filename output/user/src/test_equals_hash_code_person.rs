#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use java_runtime::prelude::*;
use java_runtime::java::io::*;
use java_runtime::java::lang::*;
use java_runtime::java::lang::r#ref::*;
use java_runtime::java::lang::reflect::*;
use java_runtime::java::math::*;
use java_runtime::java::nio::*;
use java_runtime::java::nio::charset::*;
use java_runtime::java::security::*;
use java_runtime::java::text::*;
use java_runtime::java::text::spi::*;
use java_runtime::java::time::*;
use java_runtime::java::time::chrono::*;
use java_runtime::java::time::temporal::*;
use java_runtime::java::time::zone::*;
use java_runtime::java::util::*;
use java_runtime::java::util::concurrent::*;
use java_runtime::java::util::concurrent::atomic::*;
use java_runtime::java::util::concurrent::locks::*;
use java_runtime::java::util::function::*;
use java_runtime::java::util::regex::*;
use java_runtime::java::util::spi::*;
use java_runtime::java::util::stream::*;
use java_runtime::java::util::zip::*;
use java_runtime::sun::nio::ch::*;
use java_runtime::sun::nio::cs::*;
use java_runtime::sun::reflect::generics::factory::*;
use java_runtime::sun::reflect::generics::repository::*;
use java_runtime::sun::reflect::generics::scope::*;
use java_runtime::sun::reflect::misc::*;
use java_runtime::sun::security::action::*;
use java_runtime::sun::security::util::*;
use java_runtime::sun::text::*;
use java_runtime::sun::util::*;
use java_runtime::sun::util::calendar::*;
use java_runtime::sun::util::locale::*;
use java_runtime::sun::util::locale::provider::*;
use java_runtime::sun::util::spi::*;
use java_runtime::java::text::Normalizer;
use crate::test_equals_hash_code::TestEqualsHashCode;
use crate::test_equals_hash_code_point::TestEqualsHashCode_Point;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestEqualsHashCode$Person"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestEqualsHashCode.java"]
    #[inner_classes     = "TestEqualsHashCode$Person:TestEqualsHashCode:Person:8;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestEqualsHashCode$Person;java/lang/Object"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct TestEqualsHashCode_Person {
        #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", is_static = false))]
        pub name: String,
        #[cfg_attr(any(), java_field(name = "age", descriptor = "I", is_static = false))]
        pub age: i32,
    }

    impl TestEqualsHashCode_Person {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut name: String, mut age: i32) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_name(Clone::clone(&name));
            this.__set_age(age);
            Ok(this)
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, mut obj: Object) -> Result<bool> {
            let this = self;
            if Object::from_any(this.clone()) == obj {
                return Ok((1i32 != 0i32));
            }
            if !((obj.is_instance_of("TestEqualsHashCode$Person"))) {
                return Ok((0i32 != 0i32));
            }
            let mut p = (obj).downcast::<TestEqualsHashCode_Person>();
            let mut _merged1: bool;
            if this.__get_age() == p.__get_age() {
                let _t0: bool = Objects::equals(Object::from_any(this.__get_name().clone()), Object::from_any(p.__get_name().clone()))?;
                _merged1 = !(!(_t0));
            } else {
                _merged1 = (0i32 != 0);
            }
            Ok(_merged1)
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            let this = self;
            let mut _arr0: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 2i32 as usize]));
            _arr0.borrow_mut()[0i32 as usize] = Object::from_any(this.__get_name().clone());
            _arr0.borrow_mut()[1i32 as usize] = this.__get_age().into();
            let _t1: i32 = Objects::hash(Clone::clone(&_arr0))?;
            Ok(_t1)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            let this = self;
            Ok(String::from_owned(format!("{}({})", this.__get_name(), this.__get_age())))
        }
    }
}
