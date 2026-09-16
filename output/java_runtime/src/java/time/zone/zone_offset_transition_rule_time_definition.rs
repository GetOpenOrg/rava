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

impl From<ZoneOffsetTransitionRule_TimeDefinition> for Enum<Object> {
    fn from(v: ZoneOffsetTransitionRule_TimeDefinition) -> Enum<Object> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/time/zone/ZoneOffsetTransitionRule$TimeDefinition"]
    #[super_class       = "java/lang/Enum"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final enum"]
    #[generic_signature = "Ljava/lang/Enum<Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;>;"]
    #[is_abstract       = false]
    #[is_enum           = true]
    #[is_deprecated     = false]
    #[source            = "ZoneOffsetTransitionRule.java"]
    #[inner_classes     = "java/time/zone/ZoneOffsetTransitionRule$TimeDefinition:java/time/zone/ZoneOffsetTransitionRule:TimeDefinition:16409"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Enum<Object>"]
    #[superclass_fields(name: String, ordinal: i32, hash: i32)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Enum;java/lang/Object;java/lang/constant/Constable;java/time/zone/ZoneOffsetTransitionRule$TimeDefinition"]

    pub struct ZoneOffsetTransitionRule_TimeDefinition;

    impl ZoneOffsetTransitionRule_TimeDefinition {
        #[cfg_attr(any(), java_field(name = "UTC", descriptor = "Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;", access = "public", modifiers = "static final", is_static = true))]
        // static field: UTC:Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;
        pub fn UTC() -> ZoneOffsetTransitionRule_TimeDefinition {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule$TimeDefinition.UTC:Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;")
        }

        #[cfg_attr(any(), java_field(name = "WALL", descriptor = "Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;", access = "public", modifiers = "static final", is_static = true))]
        // static field: WALL:Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;
        pub fn WALL() -> ZoneOffsetTransitionRule_TimeDefinition {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule$TimeDefinition.WALL:Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;")
        }

        #[cfg_attr(any(), java_field(name = "STANDARD", descriptor = "Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;", access = "public", modifiers = "static final", is_static = true))]
        // static field: STANDARD:Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;
        pub fn STANDARD() -> ZoneOffsetTransitionRule_TimeDefinition {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule$TimeDefinition.STANDARD:Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;")
        }

        #[cfg_attr(any(), java_field(name = "$VALUES", descriptor = "[Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;", access = "private", modifiers = "static final synthetic", is_static = true))]
        // static field: $VALUES:[Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;
        pub fn _VALUES() -> Rc<RefCell<Vec<ZoneOffsetTransitionRule_TimeDefinition>>> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule$TimeDefinition.$VALUES:[Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;")
        }

        #[java_method(name = "values", descriptor = "()[Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn values() -> Result<Rc<RefCell<Vec<ZoneOffsetTransitionRule_TimeDefinition>>>> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule$TimeDefinition.values:()[Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;")
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":32768")]
        pub fn valueOf(name: String) -> Result<ZoneOffsetTransitionRule_TimeDefinition> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule$TimeDefinition.valueOf:(Ljava/lang/String;)Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()V", method_parameters = ":4096;:4096")]
        pub fn new(arg0: String, arg1: i32) -> Result<Self> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule$TimeDefinition.<init>:(Ljava/lang/String;I)V")
        }

        #[java_method(name = "createDateTime", descriptor = "(Ljava/time/LocalDateTime;Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createDateTime(&self, mut dateTime: LocalDateTime, mut standardOffset: ZoneOffset, mut wallOffset: ZoneOffset) -> Result<LocalDateTime> {
            let this = self;
            let _t0 = this.__super().ordinal()?;
            let _switch_key = _t0;
            let _t1 = wallOffset.getTotalSeconds()?;
            let _t2 = ZoneOffset::UTC().getTotalSeconds()?;
            let mut difference = (_t1).wrapping_sub(_t2);
            let _t3 = dateTime.plusSeconds((difference as i64))?;
            return Ok(_t3);
            let _t4 = wallOffset.getTotalSeconds()?;
            let _t5 = standardOffset.getTotalSeconds()?;
            difference = (_t4).wrapping_sub(_t5);
            let _t6 = dateTime.plusSeconds((difference as i64))?;
            return Ok(_t6);
            Ok(dateTime)
        }
    }
}
