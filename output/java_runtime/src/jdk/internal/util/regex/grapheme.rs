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
use crate::jdk::internal::util::regex::*;
use crate::java::text::Normalizer;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "jdk/internal/util/regex/Grapheme"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Grapheme.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;jdk/internal/util/regex/Grapheme"]

    pub struct Grapheme;

    impl Grapheme {
        #[cfg_attr(any(), java_field(name = "OTHER", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: OTHER:I
        pub fn OTHER() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "CR", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: CR:I
        pub fn CR() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "LF", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: LF:I
        pub fn LF() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "CONTROL", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: CONTROL:I
        pub fn CONTROL() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "EXTEND", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: EXTEND:I
        pub fn EXTEND() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "ZWJ", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "5"))]
        // static field: ZWJ:I
        pub fn ZWJ() -> i32 {
            5
        }

        #[cfg_attr(any(), java_field(name = "RI", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "6"))]
        // static field: RI:I
        pub fn RI() -> i32 {
            6
        }

        #[cfg_attr(any(), java_field(name = "PREPEND", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "7"))]
        // static field: PREPEND:I
        pub fn PREPEND() -> i32 {
            7
        }

        #[cfg_attr(any(), java_field(name = "SPACINGMARK", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "8"))]
        // static field: SPACINGMARK:I
        pub fn SPACINGMARK() -> i32 {
            8
        }

        #[cfg_attr(any(), java_field(name = "L", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "9"))]
        // static field: L:I
        pub fn L() -> i32 {
            9
        }

        #[cfg_attr(any(), java_field(name = "V", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "10"))]
        // static field: V:I
        pub fn V() -> i32 {
            10
        }

        #[cfg_attr(any(), java_field(name = "T", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "11"))]
        // static field: T:I
        pub fn T() -> i32 {
            11
        }

        #[cfg_attr(any(), java_field(name = "LV", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "12"))]
        // static field: LV:I
        pub fn LV() -> i32 {
            12
        }

        #[cfg_attr(any(), java_field(name = "LVT", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "13"))]
        // static field: LVT:I
        pub fn LVT() -> i32 {
            13
        }

        #[cfg_attr(any(), java_field(name = "EXTENDED_PICTOGRAPHIC", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "14"))]
        // static field: EXTENDED_PICTOGRAPHIC:I
        pub fn EXTENDED_PICTOGRAPHIC() -> i32 {
            14
        }

        #[cfg_attr(any(), java_field(name = "FIRST_TYPE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: FIRST_TYPE:I
        pub fn FIRST_TYPE() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "LAST_TYPE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "14"))]
        // static field: LAST_TYPE:I
        pub fn LAST_TYPE() -> i32 {
            14
        }

        #[cfg_attr(any(), java_field(name = "rules", descriptor = "[[Z", access = "private", modifiers = "static final", is_static = true))]
        // static field: rules:[[Z
        pub fn rules() -> Rc<RefCell<Vec<Rc<RefCell<Vec<bool>>>>>> {
            panic!("stub: jdk/internal/util/regex/Grapheme.rules:[[Z")
        }

        #[cfg_attr(any(), java_field(name = "SYLLABLE_BASE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "44032"))]
        // static field: SYLLABLE_BASE:I
        pub fn SYLLABLE_BASE() -> i32 {
            44032
        }

        #[cfg_attr(any(), java_field(name = "LCOUNT", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "19"))]
        // static field: LCOUNT:I
        pub fn LCOUNT() -> i32 {
            19
        }

        #[cfg_attr(any(), java_field(name = "VCOUNT", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "21"))]
        // static field: VCOUNT:I
        pub fn VCOUNT() -> i32 {
            21
        }

        #[cfg_attr(any(), java_field(name = "TCOUNT", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "28"))]
        // static field: TCOUNT:I
        pub fn TCOUNT() -> i32 {
            28
        }

        #[cfg_attr(any(), java_field(name = "NCOUNT", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "588"))]
        // static field: NCOUNT:I
        pub fn NCOUNT() -> i32 {
            588
        }

        #[cfg_attr(any(), java_field(name = "SCOUNT", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "11172"))]
        // static field: SCOUNT:I
        pub fn SCOUNT() -> i32 {
            11172
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: jdk/internal/util/regex/Grapheme.<init>:()V")
        }

        #[java_method(name = "nextBoundary", descriptor = "(Ljava/lang/CharSequence;II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextBoundary(src: Object, off: i32, limit: i32) -> Result<i32> {
            panic!("stub: jdk/internal/util/regex/Grapheme.nextBoundary:(Ljava/lang/CharSequence;II)I")
        }

        #[java_method(name = "isExcludedSpacingMark", descriptor = "(I)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isExcludedSpacingMark(cp: i32) -> Result<bool> {
            panic!("stub: jdk/internal/util/regex/Grapheme.isExcludedSpacingMark:(I)Z")
        }

        #[java_method(name = "getType", descriptor = "(I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getType(cp: i32) -> Result<i32> {
            panic!("stub: jdk/internal/util/regex/Grapheme.getType:(I)I")
        }
    }
}
