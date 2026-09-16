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
use crate::test_enum_basic::TestEnumBasic;

impl From<TestEnumBasic_Day> for Enum<Object> {
    fn from(v: TestEnumBasic_Day) -> Enum<Object> { v.__into_super() }
}

static mut _TestEnumBasic_Day_MONDAY_STATIC: Option<TestEnumBasic_Day> = None;
static mut _TestEnumBasic_Day_TUESDAY_STATIC: Option<TestEnumBasic_Day> = None;
static mut _TestEnumBasic_Day_WEDNESDAY_STATIC: Option<TestEnumBasic_Day> = None;
static mut _TestEnumBasic_Day_THURSDAY_STATIC: Option<TestEnumBasic_Day> = None;
static mut _TestEnumBasic_Day_FRIDAY_STATIC: Option<TestEnumBasic_Day> = None;
static mut _TestEnumBasic_Day_SATURDAY_STATIC: Option<TestEnumBasic_Day> = None;
static mut _TestEnumBasic_Day_SUNDAY_STATIC: Option<TestEnumBasic_Day> = None;
static mut _TestEnumBasic_Day__VALUES_STATIC: Option<Rc<RefCell<Vec<TestEnumBasic_Day>>>> = None;
java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestEnumBasic$Day"]
    #[super_class       = "java/lang/Enum"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final enum"]
    #[generic_signature = "Ljava/lang/Enum<LTestEnumBasic$Day;>;"]
    #[is_abstract       = false]
    #[is_enum           = true]
    #[is_deprecated     = false]
    #[source            = "TestEnumBasic.java"]
    #[inner_classes     = "TestEnumBasic$Day:TestEnumBasic:Day:16408"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Enum<Object>"]
    #[superclass_fields(name: String, ordinal: i32, hash: i32)]
    #[all_supertypes    = "TestEnumBasic$Day;java/io/Serializable;java/lang/Comparable;java/lang/Enum;java/lang/Object;java/lang/constant/Constable"]

    pub struct TestEnumBasic_Day;

    impl TestEnumBasic_Day {
        #[cfg_attr(any(), java_field(name = "MONDAY", descriptor = "LTestEnumBasic$Day;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MONDAY:LTestEnumBasic$Day;
        pub fn MONDAY() -> TestEnumBasic_Day {
            unsafe { _TestEnumBasic_Day_MONDAY_STATIC.clone().unwrap_or_default() }
        }

        // static field setter: MONDAY
        pub fn set_MONDAY(v: TestEnumBasic_Day) {
            unsafe { _TestEnumBasic_Day_MONDAY_STATIC = Some(v); }
        }

        #[cfg_attr(any(), java_field(name = "TUESDAY", descriptor = "LTestEnumBasic$Day;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TUESDAY:LTestEnumBasic$Day;
        pub fn TUESDAY() -> TestEnumBasic_Day {
            unsafe { _TestEnumBasic_Day_TUESDAY_STATIC.clone().unwrap_or_default() }
        }

        // static field setter: TUESDAY
        pub fn set_TUESDAY(v: TestEnumBasic_Day) {
            unsafe { _TestEnumBasic_Day_TUESDAY_STATIC = Some(v); }
        }

        #[cfg_attr(any(), java_field(name = "WEDNESDAY", descriptor = "LTestEnumBasic$Day;", access = "public", modifiers = "static final", is_static = true))]
        // static field: WEDNESDAY:LTestEnumBasic$Day;
        pub fn WEDNESDAY() -> TestEnumBasic_Day {
            unsafe { _TestEnumBasic_Day_WEDNESDAY_STATIC.clone().unwrap_or_default() }
        }

        // static field setter: WEDNESDAY
        pub fn set_WEDNESDAY(v: TestEnumBasic_Day) {
            unsafe { _TestEnumBasic_Day_WEDNESDAY_STATIC = Some(v); }
        }

        #[cfg_attr(any(), java_field(name = "THURSDAY", descriptor = "LTestEnumBasic$Day;", access = "public", modifiers = "static final", is_static = true))]
        // static field: THURSDAY:LTestEnumBasic$Day;
        pub fn THURSDAY() -> TestEnumBasic_Day {
            unsafe { _TestEnumBasic_Day_THURSDAY_STATIC.clone().unwrap_or_default() }
        }

        // static field setter: THURSDAY
        pub fn set_THURSDAY(v: TestEnumBasic_Day) {
            unsafe { _TestEnumBasic_Day_THURSDAY_STATIC = Some(v); }
        }

        #[cfg_attr(any(), java_field(name = "FRIDAY", descriptor = "LTestEnumBasic$Day;", access = "public", modifiers = "static final", is_static = true))]
        // static field: FRIDAY:LTestEnumBasic$Day;
        pub fn FRIDAY() -> TestEnumBasic_Day {
            unsafe { _TestEnumBasic_Day_FRIDAY_STATIC.clone().unwrap_or_default() }
        }

        // static field setter: FRIDAY
        pub fn set_FRIDAY(v: TestEnumBasic_Day) {
            unsafe { _TestEnumBasic_Day_FRIDAY_STATIC = Some(v); }
        }

        #[cfg_attr(any(), java_field(name = "SATURDAY", descriptor = "LTestEnumBasic$Day;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SATURDAY:LTestEnumBasic$Day;
        pub fn SATURDAY() -> TestEnumBasic_Day {
            unsafe { _TestEnumBasic_Day_SATURDAY_STATIC.clone().unwrap_or_default() }
        }

        // static field setter: SATURDAY
        pub fn set_SATURDAY(v: TestEnumBasic_Day) {
            unsafe { _TestEnumBasic_Day_SATURDAY_STATIC = Some(v); }
        }

        #[cfg_attr(any(), java_field(name = "SUNDAY", descriptor = "LTestEnumBasic$Day;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SUNDAY:LTestEnumBasic$Day;
        pub fn SUNDAY() -> TestEnumBasic_Day {
            unsafe { _TestEnumBasic_Day_SUNDAY_STATIC.clone().unwrap_or_default() }
        }

        // static field setter: SUNDAY
        pub fn set_SUNDAY(v: TestEnumBasic_Day) {
            unsafe { _TestEnumBasic_Day_SUNDAY_STATIC = Some(v); }
        }

        #[cfg_attr(any(), java_field(name = "$VALUES", descriptor = "[LTestEnumBasic$Day;", access = "private", modifiers = "static final synthetic", is_static = true))]
        // static field: $VALUES:[LTestEnumBasic$Day;
        pub fn _VALUES() -> Rc<RefCell<Vec<TestEnumBasic_Day>>> {
            unsafe { _TestEnumBasic_Day__VALUES_STATIC.clone().unwrap_or_default() }
        }

        // static field setter: $VALUES
        pub fn set__VALUES(v: Rc<RefCell<Vec<TestEnumBasic_Day>>>) {
            unsafe { _TestEnumBasic_Day__VALUES_STATIC = Some(v); }
        }

        #[java_method(name = "values", descriptor = "()[LTestEnumBasic$Day;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn values() -> Result<Rc<RefCell<Vec<TestEnumBasic_Day>>>> {
            let _t0: Object = Object::from_any(TestEnumBasic_Day::_VALUES().clone());
            Ok((_t0).downcast::<Rc<RefCell<Vec<TestEnumBasic_Day>>>>())
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)LTestEnumBasic$Day;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":32768")]
        pub fn valueOf(mut name: String) -> Result<TestEnumBasic_Day> {
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
