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
use crate::test_enum_methods::TestEnumMethods;

impl From<TestEnumMethods_Planet> for Enum<Object> {
    fn from(v: TestEnumMethods_Planet) -> Enum<Object> { v.__into_super() }
}

static mut _TestEnumMethods_Planet_MERCURY_STATIC: Option<TestEnumMethods_Planet> = None;
static mut _TestEnumMethods_Planet_VENUS_STATIC: Option<TestEnumMethods_Planet> = None;
static mut _TestEnumMethods_Planet_EARTH_STATIC: Option<TestEnumMethods_Planet> = None;
static mut _TestEnumMethods_Planet__VALUES_STATIC: Option<Rc<RefCell<Vec<TestEnumMethods_Planet>>>> = None;
java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestEnumMethods$Planet"]
    #[super_class       = "java/lang/Enum"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final enum"]
    #[generic_signature = "Ljava/lang/Enum<LTestEnumMethods$Planet;>;"]
    #[is_abstract       = false]
    #[is_enum           = true]
    #[is_deprecated     = false]
    #[source            = "TestEnumMethods.java"]
    #[inner_classes     = "TestEnumMethods$Planet:TestEnumMethods:Planet:16408"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Enum<Object>"]
    #[superclass_fields(name: String, ordinal: i32, hash: i32)]
    #[all_supertypes    = "TestEnumMethods$Planet;java/io/Serializable;java/lang/Comparable;java/lang/Enum;java/lang/Object;java/lang/constant/Constable"]

    pub struct TestEnumMethods_Planet {
        #[cfg_attr(any(), java_field(name = "mass", descriptor = "D", access = "private", modifiers = "final", is_static = false))]
        pub mass: f64,
        #[cfg_attr(any(), java_field(name = "radius", descriptor = "D", access = "private", modifiers = "final", is_static = false))]
        pub radius: f64,
    }

    impl TestEnumMethods_Planet {
        #[cfg_attr(any(), java_field(name = "MERCURY", descriptor = "LTestEnumMethods$Planet;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MERCURY:LTestEnumMethods$Planet;
        pub fn MERCURY() -> TestEnumMethods_Planet {
            unsafe { _TestEnumMethods_Planet_MERCURY_STATIC.clone().unwrap_or_default() }
        }

        // static field setter: MERCURY
        pub fn set_MERCURY(v: TestEnumMethods_Planet) {
            unsafe { _TestEnumMethods_Planet_MERCURY_STATIC = Some(v); }
        }

        #[cfg_attr(any(), java_field(name = "VENUS", descriptor = "LTestEnumMethods$Planet;", access = "public", modifiers = "static final", is_static = true))]
        // static field: VENUS:LTestEnumMethods$Planet;
        pub fn VENUS() -> TestEnumMethods_Planet {
            unsafe { _TestEnumMethods_Planet_VENUS_STATIC.clone().unwrap_or_default() }
        }

        // static field setter: VENUS
        pub fn set_VENUS(v: TestEnumMethods_Planet) {
            unsafe { _TestEnumMethods_Planet_VENUS_STATIC = Some(v); }
        }

        #[cfg_attr(any(), java_field(name = "EARTH", descriptor = "LTestEnumMethods$Planet;", access = "public", modifiers = "static final", is_static = true))]
        // static field: EARTH:LTestEnumMethods$Planet;
        pub fn EARTH() -> TestEnumMethods_Planet {
            unsafe { _TestEnumMethods_Planet_EARTH_STATIC.clone().unwrap_or_default() }
        }

        // static field setter: EARTH
        pub fn set_EARTH(v: TestEnumMethods_Planet) {
            unsafe { _TestEnumMethods_Planet_EARTH_STATIC = Some(v); }
        }

        #[cfg_attr(any(), java_field(name = "G", descriptor = "D", access = "package", modifiers = "static final", is_static = true, constant_value = "6.673e-11"))]
        // static field: G:D
        pub fn G() -> f64 {
            6.673e-11f64
        }

        #[cfg_attr(any(), java_field(name = "$VALUES", descriptor = "[LTestEnumMethods$Planet;", access = "private", modifiers = "static final synthetic", is_static = true))]
        // static field: $VALUES:[LTestEnumMethods$Planet;
        pub fn _VALUES() -> Rc<RefCell<Vec<TestEnumMethods_Planet>>> {
            unsafe { _TestEnumMethods_Planet__VALUES_STATIC.clone().unwrap_or_default() }
        }

        // static field setter: $VALUES
        pub fn set__VALUES(v: Rc<RefCell<Vec<TestEnumMethods_Planet>>>) {
            unsafe { _TestEnumMethods_Planet__VALUES_STATIC = Some(v); }
        }

        #[java_method(name = "values", descriptor = "()[LTestEnumMethods$Planet;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn values() -> Result<Rc<RefCell<Vec<TestEnumMethods_Planet>>>> {
            let _t0: Object = Object::from_any(TestEnumMethods_Planet::_VALUES().clone());
            Ok((_t0).downcast::<Rc<RefCell<Vec<TestEnumMethods_Planet>>>>())
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)LTestEnumMethods$Planet;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":32768")]
        pub fn valueOf(mut name: String) -> Result<TestEnumMethods_Planet> {
            let _t0: Enum<Object> = Enum::<Object>::valueOf(Default::default(), Clone::clone(&name))?;
            Ok(Default::default())
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;IDD)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(DD)V", method_parameters = ":4096;:4096;:0;:0")]
        pub fn new(mut arg_0: String, mut arg_1: i32, mut mass: f64, mut radius: f64) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Enum::new(Clone::clone(&arg_0), arg_1)?);
            this.__set_mass(mass);
            this.__set_radius(radius);
            Ok(this)
        }

        #[java_method(name = "surfaceGravity", descriptor = "()D", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn surfaceGravity(&self) -> Result<f64> {
            let this = self;
            Ok(((6.673e-11f64*this.__get_mass())/(this.__get_radius()*this.__get_radius())))
        }

        #[java_method(name = "surfaceWeight", descriptor = "(D)D", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn surfaceWeight(&self, mut otherMass: f64) -> Result<f64> {
            let this = self;
            let _t0 = this.surfaceGravity()?;
            Ok((otherMass*_t0))
        }
    }
}
