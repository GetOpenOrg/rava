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
use crate::test_switch_enum::TestSwitchEnum;

impl From<TestSwitchEnum_Season> for Enum<Object> {
    fn from(v: TestSwitchEnum_Season) -> Enum<Object> { v.__into_super() }
}

static mut _TestSwitchEnum_Season_SPRING_STATIC: Option<TestSwitchEnum_Season> = None;
static mut _TestSwitchEnum_Season_SUMMER_STATIC: Option<TestSwitchEnum_Season> = None;
static mut _TestSwitchEnum_Season_FALL_STATIC: Option<TestSwitchEnum_Season> = None;
static mut _TestSwitchEnum_Season_WINTER_STATIC: Option<TestSwitchEnum_Season> = None;
static mut _TestSwitchEnum_Season__VALUES_STATIC: Option<Rc<RefCell<Vec<TestSwitchEnum_Season>>>> = None;
java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestSwitchEnum$Season"]
    #[super_class       = "java/lang/Enum"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final enum"]
    #[generic_signature = "Ljava/lang/Enum<LTestSwitchEnum$Season;>;"]
    #[is_abstract       = false]
    #[is_enum           = true]
    #[is_deprecated     = false]
    #[source            = "TestSwitchEnum.java"]
    #[inner_classes     = "TestSwitchEnum$Season:TestSwitchEnum:Season:16408"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Enum<Object>"]
    #[superclass_fields(name: String, ordinal: i32, hash: i32)]
    #[all_supertypes    = "TestSwitchEnum$Season;java/io/Serializable;java/lang/Comparable;java/lang/Enum;java/lang/Object;java/lang/constant/Constable"]

    pub struct TestSwitchEnum_Season;

    impl TestSwitchEnum_Season {
        #[cfg_attr(any(), java_field(name = "SPRING", descriptor = "LTestSwitchEnum$Season;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SPRING:LTestSwitchEnum$Season;
        pub fn SPRING() -> TestSwitchEnum_Season {
            unsafe { _TestSwitchEnum_Season_SPRING_STATIC.clone().unwrap_or_default() }
        }

        // static field setter: SPRING
        pub fn set_SPRING(v: TestSwitchEnum_Season) {
            unsafe { _TestSwitchEnum_Season_SPRING_STATIC = Some(v); }
        }

        #[cfg_attr(any(), java_field(name = "SUMMER", descriptor = "LTestSwitchEnum$Season;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SUMMER:LTestSwitchEnum$Season;
        pub fn SUMMER() -> TestSwitchEnum_Season {
            unsafe { _TestSwitchEnum_Season_SUMMER_STATIC.clone().unwrap_or_default() }
        }

        // static field setter: SUMMER
        pub fn set_SUMMER(v: TestSwitchEnum_Season) {
            unsafe { _TestSwitchEnum_Season_SUMMER_STATIC = Some(v); }
        }

        #[cfg_attr(any(), java_field(name = "FALL", descriptor = "LTestSwitchEnum$Season;", access = "public", modifiers = "static final", is_static = true))]
        // static field: FALL:LTestSwitchEnum$Season;
        pub fn FALL() -> TestSwitchEnum_Season {
            unsafe { _TestSwitchEnum_Season_FALL_STATIC.clone().unwrap_or_default() }
        }

        // static field setter: FALL
        pub fn set_FALL(v: TestSwitchEnum_Season) {
            unsafe { _TestSwitchEnum_Season_FALL_STATIC = Some(v); }
        }

        #[cfg_attr(any(), java_field(name = "WINTER", descriptor = "LTestSwitchEnum$Season;", access = "public", modifiers = "static final", is_static = true))]
        // static field: WINTER:LTestSwitchEnum$Season;
        pub fn WINTER() -> TestSwitchEnum_Season {
            unsafe { _TestSwitchEnum_Season_WINTER_STATIC.clone().unwrap_or_default() }
        }

        // static field setter: WINTER
        pub fn set_WINTER(v: TestSwitchEnum_Season) {
            unsafe { _TestSwitchEnum_Season_WINTER_STATIC = Some(v); }
        }

        #[cfg_attr(any(), java_field(name = "$VALUES", descriptor = "[LTestSwitchEnum$Season;", access = "private", modifiers = "static final synthetic", is_static = true))]
        // static field: $VALUES:[LTestSwitchEnum$Season;
        pub fn _VALUES() -> Rc<RefCell<Vec<TestSwitchEnum_Season>>> {
            unsafe { _TestSwitchEnum_Season__VALUES_STATIC.clone().unwrap_or_default() }
        }

        // static field setter: $VALUES
        pub fn set__VALUES(v: Rc<RefCell<Vec<TestSwitchEnum_Season>>>) {
            unsafe { _TestSwitchEnum_Season__VALUES_STATIC = Some(v); }
        }

        #[java_method(name = "values", descriptor = "()[LTestSwitchEnum$Season;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn values() -> Result<Rc<RefCell<Vec<TestSwitchEnum_Season>>>> {
            let _t0: Object = Object::from_any(TestSwitchEnum_Season::_VALUES().clone());
            Ok((_t0).downcast::<Rc<RefCell<Vec<TestSwitchEnum_Season>>>>())
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)LTestSwitchEnum$Season;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":32768")]
        pub fn valueOf(mut name: String) -> Result<TestSwitchEnum_Season> {
            let _t0: Enum<Object> = Enum::<Object>::valueOf(Default::default(), Clone::clone(&name))?;
            Ok(Default::default())
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()V", method_parameters = ":4096;:4096")]
        pub fn new(mut arg_0: String, mut arg_1: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Enum::new(Clone::clone(&arg_0), arg_1)?);
            Ok(this)
        }
    }
}
