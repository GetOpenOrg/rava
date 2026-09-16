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
    #[binary_name       = "java/util/regex/ASCII"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ASCII.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/regex/ASCII"]

    pub struct ASCII;

    impl ASCII {
        #[cfg_attr(any(), java_field(name = "UPPER", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "256"))]
        // static field: UPPER:I
        pub fn UPPER() -> i32 {
            256
        }

        #[cfg_attr(any(), java_field(name = "LOWER", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "512"))]
        // static field: LOWER:I
        pub fn LOWER() -> i32 {
            512
        }

        #[cfg_attr(any(), java_field(name = "DIGIT", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1024"))]
        // static field: DIGIT:I
        pub fn DIGIT() -> i32 {
            1024
        }

        #[cfg_attr(any(), java_field(name = "SPACE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "2048"))]
        // static field: SPACE:I
        pub fn SPACE() -> i32 {
            2048
        }

        #[cfg_attr(any(), java_field(name = "PUNCT", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "4096"))]
        // static field: PUNCT:I
        pub fn PUNCT() -> i32 {
            4096
        }

        #[cfg_attr(any(), java_field(name = "CNTRL", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "8192"))]
        // static field: CNTRL:I
        pub fn CNTRL() -> i32 {
            8192
        }

        #[cfg_attr(any(), java_field(name = "BLANK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "16384"))]
        // static field: BLANK:I
        pub fn BLANK() -> i32 {
            16384
        }

        #[cfg_attr(any(), java_field(name = "HEX", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "32768"))]
        // static field: HEX:I
        pub fn HEX() -> i32 {
            32768
        }

        #[cfg_attr(any(), java_field(name = "UNDER", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "65536"))]
        // static field: UNDER:I
        pub fn UNDER() -> i32 {
            65536
        }

        #[cfg_attr(any(), java_field(name = "ASCII", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "65280"))]
        // static field: ASCII:I
        pub fn ASCII() -> i32 {
            65280
        }

        #[cfg_attr(any(), java_field(name = "ALPHA", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "768"))]
        // static field: ALPHA:I
        pub fn ALPHA() -> i32 {
            768
        }

        #[cfg_attr(any(), java_field(name = "ALNUM", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1792"))]
        // static field: ALNUM:I
        pub fn ALNUM() -> i32 {
            1792
        }

        #[cfg_attr(any(), java_field(name = "GRAPH", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "5888"))]
        // static field: GRAPH:I
        pub fn GRAPH() -> i32 {
            5888
        }

        #[cfg_attr(any(), java_field(name = "WORD", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "67328"))]
        // static field: WORD:I
        pub fn WORD() -> i32 {
            67328
        }

        #[cfg_attr(any(), java_field(name = "XDIGIT", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "32768"))]
        // static field: XDIGIT:I
        pub fn XDIGIT() -> i32 {
            32768
        }

        #[cfg_attr(any(), java_field(name = "ctype", descriptor = "[I", access = "private", modifiers = "static final", is_static = true))]
        // static field: ctype:[I
        pub fn ctype() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/util/regex/ASCII.ctype:[I")
        }

        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/regex/ASCII.<init>:()V")
        }

        #[java_method(name = "getType", descriptor = "(I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getType(mut ch: i32) -> Result<i32> {
            Ok((if ((ch&-128i32)==0) { ASCII::ctype().borrow()[ch as usize] } else { 0i32 }))
        }

        #[java_method(name = "isType", descriptor = "(II)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isType(mut ch: i32, mut type_: i32) -> Result<bool> {
            let _t0: i32 = ASCII::getType(ch)?;
            Ok(((_t0&type_)!=0))
        }

        #[java_method(name = "isAscii", descriptor = "(I)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isAscii(mut ch: i32) -> Result<bool> {
            Ok(((ch&-128i32)==0))
        }

        #[java_method(name = "isAlpha", descriptor = "(I)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isAlpha(mut ch: i32) -> Result<bool> {
            let _t0: bool = ASCII::isType(ch, 768i32)?;
            Ok(_t0)
        }

        #[java_method(name = "isDigit", descriptor = "(I)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDigit(mut ch: i32) -> Result<bool> {
            Ok((((ch).wrapping_sub(48i32)|(57i32).wrapping_sub(ch))>=0))
        }

        #[java_method(name = "isAlnum", descriptor = "(I)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isAlnum(mut ch: i32) -> Result<bool> {
            let _t0: bool = ASCII::isType(ch, 1792i32)?;
            Ok(_t0)
        }

        #[java_method(name = "isGraph", descriptor = "(I)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isGraph(ch: i32) -> Result<bool> {
            panic!("stub: java/util/regex/ASCII.isGraph:(I)Z")
        }

        #[java_method(name = "isPrint", descriptor = "(I)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isPrint(ch: i32) -> Result<bool> {
            panic!("stub: java/util/regex/ASCII.isPrint:(I)Z")
        }

        #[java_method(name = "isPunct", descriptor = "(I)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isPunct(ch: i32) -> Result<bool> {
            panic!("stub: java/util/regex/ASCII.isPunct:(I)Z")
        }

        #[java_method(name = "isSpace", descriptor = "(I)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSpace(mut ch: i32) -> Result<bool> {
            let _t0: bool = ASCII::isType(ch, 2048i32)?;
            Ok(_t0)
        }

        #[java_method(name = "isHexDigit", descriptor = "(I)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isHexDigit(mut ch: i32) -> Result<bool> {
            let _t0: bool = ASCII::isType(ch, 32768i32)?;
            Ok(_t0)
        }

        #[java_method(name = "isOctDigit", descriptor = "(I)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isOctDigit(ch: i32) -> Result<bool> {
            panic!("stub: java/util/regex/ASCII.isOctDigit:(I)Z")
        }

        #[java_method(name = "isCntrl", descriptor = "(I)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isCntrl(ch: i32) -> Result<bool> {
            panic!("stub: java/util/regex/ASCII.isCntrl:(I)Z")
        }

        #[java_method(name = "isLower", descriptor = "(I)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLower(mut ch: i32) -> Result<bool> {
            Ok((((ch).wrapping_sub(97i32)|(122i32).wrapping_sub(ch))>=0))
        }

        #[java_method(name = "isUpper", descriptor = "(I)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUpper(mut ch: i32) -> Result<bool> {
            Ok((((ch).wrapping_sub(65i32)|(90i32).wrapping_sub(ch))>=0))
        }

        #[java_method(name = "isWord", descriptor = "(I)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isWord(ch: i32) -> Result<bool> {
            panic!("stub: java/util/regex/ASCII.isWord:(I)Z")
        }

        #[java_method(name = "toDigit", descriptor = "(I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toDigit(mut ch: i32) -> Result<i32> {
            Ok((ASCII::ctype().borrow()[(ch&127i32) as usize]&63i32))
        }

        #[java_method(name = "toLower", descriptor = "(I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toLower(mut ch: i32) -> Result<i32> {
            let _t0: bool = ASCII::isUpper(ch)?;
            Ok((if _t0 { (ch).wrapping_add(32i32) } else { ch }))
        }

        #[java_method(name = "toUpper", descriptor = "(I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpper(mut ch: i32) -> Result<i32> {
            let _t0: bool = ASCII::isLower(ch)?;
            Ok((if _t0 { (ch).wrapping_sub(32i32) } else { ch }))
        }
    }
}
