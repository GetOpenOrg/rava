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
    #[binary_name       = "java/util/Formatter$Flags"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Formatter.java"]
    #[inner_classes     = "java/util/Formatter$Flags:java/util/Formatter:Flags:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Formatter$Flags"]

    pub struct Formatter_Flags;

    impl Formatter_Flags {
        #[cfg_attr(any(), java_field(name = "NONE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: NONE:I
        pub fn NONE() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "LEFT_JUSTIFY", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: LEFT_JUSTIFY:I
        pub fn LEFT_JUSTIFY() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "UPPERCASE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: UPPERCASE:I
        pub fn UPPERCASE() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "ALTERNATE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: ALTERNATE:I
        pub fn ALTERNATE() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "PLUS", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "8"))]
        // static field: PLUS:I
        pub fn PLUS() -> i32 {
            8
        }

        #[cfg_attr(any(), java_field(name = "LEADING_SPACE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: LEADING_SPACE:I
        pub fn LEADING_SPACE() -> i32 {
            16
        }

        #[cfg_attr(any(), java_field(name = "ZERO_PAD", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "32"))]
        // static field: ZERO_PAD:I
        pub fn ZERO_PAD() -> i32 {
            32
        }

        #[cfg_attr(any(), java_field(name = "GROUP", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "64"))]
        // static field: GROUP:I
        pub fn GROUP() -> i32 {
            64
        }

        #[cfg_attr(any(), java_field(name = "PARENTHESES", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "128"))]
        // static field: PARENTHESES:I
        pub fn PARENTHESES() -> i32 {
            128
        }

        #[cfg_attr(any(), java_field(name = "PREVIOUS", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "256"))]
        // static field: PREVIOUS:I
        pub fn PREVIOUS() -> i32 {
            256
        }

        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/Formatter$Flags.<init>:()V")
        }

        #[java_method(name = "contains", descriptor = "(II)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn contains(mut flags: i32, mut f: i32) -> Result<bool> {
            Ok((flags&f) == f)
        }

        #[java_method(name = "containsAny", descriptor = "(II)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn containsAny(mut flags: i32, mut f: i32) -> Result<bool> {
            Ok(((flags&f)!=0))
        }

        #[java_method(name = "add", descriptor = "(II)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn add(mut flags: i32, mut f: i32) -> Result<i32> {
            Ok((flags|f))
        }

        #[java_method(name = "remove", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remove(mut flags: i32, mut f: i32) -> Result<i32> {
            Ok((flags&(f^-1i32)))
        }

        #[java_method(name = "parse", descriptor = "(Ljava/lang/String;II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: parse(Ljava/lang/String;II)I
        pub fn parse_str_i_i(mut s: String, mut start: i32, mut end: i32) -> Result<i32> {
            let mut f: i32 = 0i32;
            let mut i: i32 = start;
            loop {
                if i >= end { break; }
                let _t0 = s.charAt(i)?;
                let mut c: u16 = _t0;
                let _t1: i32 = Formatter_Flags::parse_c(c)?;
                let mut v: i32 = _t1;
                let _t2: bool = Formatter_Flags::contains(f, v)?;
                if _t2 {
                    let _t3: String = Formatter_Flags::toString(v)?;
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                let _t3: i32 = Formatter_Flags::add(f, v)?;
                f = _t3;
                i = i.wrapping_add(1i32);
            }
            Ok(f)
        }

        #[java_method(name = "parse", descriptor = "(C)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parse_c(c: u16) -> Result<i32> {
            panic!("stub: java/util/Formatter$Flags.parse:(C)I")
        }

        #[java_method(name = "toString", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(mut f: i32) -> Result<String> {
            let mut sb = StringBuilder::new()?;
            let _t0: bool = Formatter_Flags::contains(f, 1i32)?;
            if _t0 {
                let _t1 = sb.append_c(((45i32) as u16))?;
            }
            let _t1: bool = Formatter_Flags::contains(f, 2i32)?;
            if _t1 {
                let _t2 = sb.append_c(((94i32) as u16))?;
            }
            let _t2: bool = Formatter_Flags::contains(f, 4i32)?;
            if _t2 {
                let _t3 = sb.append_c(((35i32) as u16))?;
            }
            let _t3: bool = Formatter_Flags::contains(f, 8i32)?;
            if _t3 {
                let _t4 = sb.append_c(((43i32) as u16))?;
            }
            let _t4: bool = Formatter_Flags::contains(f, 16i32)?;
            if _t4 {
                let _t5 = sb.append_c(((32i32) as u16))?;
            }
            let _t5: bool = Formatter_Flags::contains(f, 32i32)?;
            if _t5 {
                let _t6 = sb.append_c(((48i32) as u16))?;
            }
            let _t6: bool = Formatter_Flags::contains(f, 64i32)?;
            if _t6 {
                let _t7 = sb.append_c(((44i32) as u16))?;
            }
            let _t7: bool = Formatter_Flags::contains(f, 128i32)?;
            if _t7 {
                let _t8 = sb.append_c(((40i32) as u16))?;
            }
            let _t8: bool = Formatter_Flags::contains(f, 256i32)?;
            if _t8 {
                let _t9 = sb.append_c(((60i32) as u16))?;
            }
            let _t9 = sb.toString()?;
            Ok(_t9)
        }
    }
}
