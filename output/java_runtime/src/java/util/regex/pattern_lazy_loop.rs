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

impl From<Pattern_LazyLoop> for Pattern_Loop {
    fn from(v: Pattern_LazyLoop) -> Pattern_Loop { v.__into_super() }
}

impl From<Pattern_LazyLoop> for Pattern_Node {
    fn from(v: Pattern_LazyLoop) -> Pattern_Node { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/regex/Pattern$LazyLoop"]
    #[super_class       = "java/util/regex/Pattern$Loop"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Pattern.java"]
    #[inner_classes     = "java/util/regex/Pattern$Loop:java/util/regex/Pattern:Loop:8;java/util/regex/Pattern$LazyLoop:java/util/regex/Pattern:LazyLoop:24;java/util/regex/Pattern$Node:java/util/regex/Pattern:Node:8;java/util/regex/Pattern$TreeInfo:java/util/regex/Pattern:TreeInfo:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Pattern_Loop"]
    #[superclass_fields(next: Pattern_Node, body: Pattern_Node, countIndex: i32, beginIndex: i32, cmin: i32, cmax: i32, posIndex: i32)]
    #[all_supertypes    = "java/lang/Object;java/util/regex/Pattern$LazyLoop;java/util/regex/Pattern$Loop;java/util/regex/Pattern$Node"]

    pub struct Pattern_LazyLoop;

    impl Pattern_LazyLoop {
        #[java_method(name = "<init>", descriptor = "(II)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut countIndex: i32, mut beginIndex: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Pattern_Loop::new(countIndex, beginIndex)?);
            Ok(this)
        }

        #[java_method(name = "match", descriptor = "(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn match_(&self, matcher: Matcher, i: i32, seq: Object) -> Result<bool> {
            panic!("stub: java/util/regex/Pattern$LazyLoop.match:(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)Z")
        }

        #[java_method(name = "matchInit", descriptor = "(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn matchInit(&self, matcher: Matcher, i: i32, seq: Object) -> Result<bool> {
            panic!("stub: java/util/regex/Pattern$LazyLoop.matchInit:(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)Z")
        }

        #[java_method(name = "study", descriptor = "(Ljava/util/regex/Pattern$TreeInfo;)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn study(&self, info: Pattern_TreeInfo) -> Result<bool> {
            panic!("stub: java/util/regex/Pattern$LazyLoop.study:(Ljava/util/regex/Pattern$TreeInfo;)Z")
        }
    }
}
