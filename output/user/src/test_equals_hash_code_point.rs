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
use crate::test_equals_hash_code_person::TestEqualsHashCode_Person;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestEqualsHashCode$Point"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestEqualsHashCode.java"]
    #[inner_classes     = "TestEqualsHashCode$Point:TestEqualsHashCode:Point:8;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestEqualsHashCode$Point;java/lang/Object"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct TestEqualsHashCode_Point {
        #[cfg_attr(any(), java_field(name = "x", descriptor = "I", is_static = false))]
        pub x: i32,
        #[cfg_attr(any(), java_field(name = "y", descriptor = "I", is_static = false))]
        pub y: i32,
    }

    impl TestEqualsHashCode_Point {
        #[java_method(name = "<init>", descriptor = "(II)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut x: i32, mut y: i32) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_x(x);
            this.__set_y(y);
            Ok(this)
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, mut obj: Object) -> Result<bool> {
            let this = self;
            if Object::from_any(this.clone()) == obj {
                return Ok((1i32 != 0i32));
            }
            if !((obj.is_instance_of("TestEqualsHashCode$Point"))) {
                return Ok((0i32 != 0i32));
            }
            let mut p = (obj).downcast::<TestEqualsHashCode_Point>();
            Ok((if this.__get_x() == p.__get_x() { this.__get_y() == p.__get_y() } else { (0i32 != 0) }))
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            let this = self;
            let mut _arr0: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 2i32 as usize]));
            _arr0.borrow_mut()[0i32 as usize] = this.__get_x().into();
            _arr0.borrow_mut()[1i32 as usize] = this.__get_y().into();
            let _t1: i32 = Objects::hash(Clone::clone(&_arr0))?;
            Ok(_t1)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            let this = self;
            Ok(String::from_owned(format!("Point({}, {})", this.__get_x(), this.__get_y())))
        }
    }
}
