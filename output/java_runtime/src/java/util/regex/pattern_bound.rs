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

impl From<Pattern_Bound> for Pattern_Node {
    fn from(v: Pattern_Bound) -> Pattern_Node { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/regex/Pattern$Bound"]
    #[super_class       = "java/util/regex/Pattern$Node"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Pattern.java"]
    #[inner_classes     = "java/util/regex/Pattern$Node:java/util/regex/Pattern:Node:8;java/util/regex/Pattern$Bound:java/util/regex/Pattern:Bound:24;java/util/regex/Pattern$CharPredicate:java/util/regex/Pattern:CharPredicate:1544;java/util/regex/Pattern$BmpCharPredicate:java/util/regex/Pattern:BmpCharPredicate:1544"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Pattern_Node"]
    #[superclass_fields(next: Pattern_Node)]
    #[all_supertypes    = "java/lang/Object;java/util/regex/Pattern$Bound;java/util/regex/Pattern$Node"]

    pub struct Pattern_Bound {
        #[cfg_attr(any(), java_field(name = "type", descriptor = "I", is_static = false))]
        pub type_: i32,
        #[cfg_attr(any(), java_field(name = "useUWORD", descriptor = "Z", is_static = false))]
        pub useUWORD: bool,
    }

    impl Pattern_Bound {
        #[cfg_attr(any(), java_field(name = "LEFT", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: LEFT:I
        pub fn LEFT() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "RIGHT", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: RIGHT:I
        pub fn RIGHT() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "BOTH", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: BOTH:I
        pub fn BOTH() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "NONE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: NONE:I
        pub fn NONE() -> i32 {
            4
        }

        #[java_method(name = "<init>", descriptor = "(IZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut n: i32, mut useUWORD: bool) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Pattern_Node::new()?);
            this.__set_type_(n);
            this.__set_useUWORD(useUWORD);
            Ok(this)
        }

        #[java_method(name = "isWord", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isWord(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/util/regex/Pattern$Bound.isWord:(I)Z")
        }

        #[java_method(name = "check", descriptor = "(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn check(&self, matcher: Matcher, i: i32, seq: Object) -> Result<i32> {
            panic!("stub: java/util/regex/Pattern$Bound.check:(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)I")
        }

        #[java_method(name = "match", descriptor = "(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn match_(&self, matcher: Matcher, i: i32, seq: Object) -> Result<bool> {
            panic!("stub: java/util/regex/Pattern$Bound.match:(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)Z")
        }
    }
}
