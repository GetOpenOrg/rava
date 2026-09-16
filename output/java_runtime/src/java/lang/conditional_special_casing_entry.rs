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
    #[binary_name       = "java/lang/ConditionalSpecialCasing$Entry"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ConditionalSpecialCasing.java"]
    #[inner_classes     = "java/lang/ConditionalSpecialCasing$Entry:java/lang/ConditionalSpecialCasing:Entry:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/ConditionalSpecialCasing$Entry;java/lang/Object"]

    pub struct ConditionalSpecialCasing_Entry {
        #[cfg_attr(any(), java_field(name = "ch", descriptor = "I", is_static = false))]
        pub ch: i32,
        #[cfg_attr(any(), java_field(name = "lower", descriptor = "[C", is_static = false))]
        pub lower: Rc<RefCell<Vec<u16>>>,
        #[cfg_attr(any(), java_field(name = "upper", descriptor = "[C", is_static = false))]
        pub upper: Rc<RefCell<Vec<u16>>>,
        #[cfg_attr(any(), java_field(name = "lang", descriptor = "Ljava/lang/String;", is_static = false))]
        pub lang: String,
        #[cfg_attr(any(), java_field(name = "condition", descriptor = "I", is_static = false))]
        pub condition: i32,
    }

    impl ConditionalSpecialCasing_Entry {
        #[java_method(name = "<init>", descriptor = "(I[C[CLjava/lang/String;I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(ch: i32, lower: Rc<RefCell<Vec<u16>>>, upper: Rc<RefCell<Vec<u16>>>, lang: String, condition: i32) -> Result<Self> {
            panic!("stub: java/lang/ConditionalSpecialCasing$Entry.<init>:(I[C[CLjava/lang/String;I)V")
        }

        #[java_method(name = "getCodePoint", descriptor = "()I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCodePoint(&self) -> Result<i32> {
            panic!("stub: java/lang/ConditionalSpecialCasing$Entry.getCodePoint:()I")
        }

        #[java_method(name = "getLowerCase", descriptor = "()[C", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLowerCase(&self) -> Result<Rc<RefCell<Vec<u16>>>> {
            let this = self;
            Ok(this.__get_lower())
        }

        #[java_method(name = "getUpperCase", descriptor = "()[C", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getUpperCase(&self) -> Result<Rc<RefCell<Vec<u16>>>> {
            let this = self;
            Ok(this.__get_upper())
        }

        #[java_method(name = "getLanguage", descriptor = "()Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLanguage(&self) -> Result<String> {
            let this = self;
            Ok(this.__get_lang())
        }

        #[java_method(name = "getCondition", descriptor = "()I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCondition(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_condition())
        }
    }
}
