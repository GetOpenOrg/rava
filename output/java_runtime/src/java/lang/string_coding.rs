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
    #[binary_name       = "java/lang/StringCoding"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "StringCoding.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/lang/StringCoding"]

    pub struct StringCoding;

    impl StringCoding {
        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/StringCoding.<init>:()V")
        }

        #[java_method(name = "hasNegatives", descriptor = "([BII)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasNegatives(mut ba: Rc<RefCell<Vec<i8>>>, mut off: i32, mut len: i32) -> Result<bool> {
            let _t0: i32 = StringCoding::countPositives(Clone::clone(&ba), off, len)?;
            Ok(_t0 != len)
        }

        #[java_method(name = "countPositives", descriptor = "([BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn countPositives(mut ba: Rc<RefCell<Vec<i8>>>, mut off: i32, mut len: i32) -> Result<i32> {
            let mut limit = (off).wrapping_add(len);
            let mut i: i32 = off;
            loop {
                if i >= limit { break; }
                if ((ba.borrow()[i as usize] as i32)<0) {
                    return Ok((i).wrapping_sub(off));
                }
                i = i.wrapping_add(1i32);
            }
            Ok(len)
        }

        #[java_method(name = "implEncodeISOArray", descriptor = "([BI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn implEncodeISOArray(mut sa: Rc<RefCell<Vec<i8>>>, mut sp: i32, mut da: Rc<RefCell<Vec<i8>>>, mut dp: i32, mut len: i32) -> Result<i32> {
            let mut i: i32 = 0i32;
            loop {
                if i >= len { break; }
                sp = sp.wrapping_add(1i32);
                let _t0: u16 = StringUTF16::getChar(Clone::clone(&sa), sp)?;
                let mut c: u16 = _t0;
                if (c as i32) > 255i32 {
                    break;
                }
                dp = dp.wrapping_add(1i32);
                da.borrow_mut()[dp as usize] = (((c) as i8 as i32)) as i8;
                i = i.wrapping_add(1i32);
            }
            Ok(i)
        }

        #[java_method(name = "implEncodeAsciiArray", descriptor = "([CI[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn implEncodeAsciiArray(sa: Rc<RefCell<Vec<u16>>>, sp: i32, da: Rc<RefCell<Vec<i8>>>, dp: i32, len: i32) -> Result<i32> {
            panic!("stub: java/lang/StringCoding.implEncodeAsciiArray:([CI[BII)I")
        }
    }
}
