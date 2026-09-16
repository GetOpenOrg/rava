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

impl From<NumberFormatException> for IllegalArgumentException {
    fn from(v: NumberFormatException) -> IllegalArgumentException { v.__into_super() }
}

impl From<NumberFormatException> for RuntimeException {
    fn from(v: NumberFormatException) -> RuntimeException { v.__into_super().__into_super() }
}

impl From<NumberFormatException> for Exception {
    fn from(v: NumberFormatException) -> Exception { v.__into_super().__into_super().__into_super() }
}

impl From<NumberFormatException> for Throwable {
    fn from(v: NumberFormatException) -> Throwable { v.__into_super().__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/NumberFormatException"]
    #[super_class       = "java/lang/IllegalArgumentException"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "NumberFormatException.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "IllegalArgumentException"]
    #[superclass_fields(backtrace: Object, detailMessage: String, cause: Throwable, stackTrace: Rc<RefCell<Vec<Object>>>, depth: i32, suppressedExceptions: Object)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Exception;java/lang/IllegalArgumentException;java/lang/NumberFormatException;java/lang/Object;java/lang/RuntimeException;java/lang/Throwable"]

    pub struct NumberFormatException;

    impl NumberFormatException {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "-2848938806368998894"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -2848938806368998894i64
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(IllegalArgumentException::new()?);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/String;)V
        pub fn new_str(mut s: String) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(IllegalArgumentException::new_str(Clone::clone(&s))?);
            Ok(this)
        }

        #[java_method(name = "forInputString", descriptor = "(Ljava/lang/String;I)Ljava/lang/NumberFormatException;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forInputString(mut s: String, mut radix: i32) -> Result<NumberFormatException> {
            let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("For input string: \"")))?;
            let _t1 = _t0.append_str(Clone::clone(&s))?;
            let _t2 = _t1.append_str(Clone::clone(&String::from("\"")))?;
            let mut _merged6: String;
            if radix == 10i32 {
                _merged6 = String::from("");
            } else {
                let _t3 = StringBuilder::new()?.append_str(Clone::clone(&String::from(" under radix ")))?;
                let _t4 = _t3.append_i(radix)?;
                let _t5 = _t4.toString()?;
                _merged6 = _t5;
            }
            let _t7 = _t2.append_str(Clone::clone(&_merged6))?;
            let _t8 = _t7.toString()?;
            Ok(NumberFormatException::new_str(Clone::clone(&_t8))?)
        }

        #[java_method(name = "forCharSequence", descriptor = "(Ljava/lang/CharSequence;III)Ljava/lang/NumberFormatException;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forCharSequence(mut s: Object, mut beginIndex: i32, mut endIndex: i32, mut errorIndex: i32) -> Result<NumberFormatException> {
            let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Error at index ")))?;
            let _t1 = _t0.append_i((errorIndex).wrapping_sub(beginIndex))?;
            let _t2 = _t1.append_str(Clone::clone(&String::from(" in: \"")))?;
            let _vdispatch3: Object = if let Some(_d) = s.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.subSequence(beginIndex, endIndex)? } else if let Some(_d) = s.0.as_any().downcast_ref::<CharBuffer>() { _d.subSequence(beginIndex, endIndex)? } else if let Some(_d) = s.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.subSequence(beginIndex, endIndex)? } else if let Some(_d) = s.0.as_any().downcast_ref::<String>() { _d.subSequence(beginIndex, endIndex)? } else if let Some(_d) = s.0.as_any().downcast_ref::<StringBuilder>() { _d.subSequence(beginIndex, endIndex)? } else if let Some(_d) = s.0.as_any().downcast_ref::<Object>() { _d.subSequence(beginIndex, endIndex)? } else if let Some(__f) = s.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32, i32) -> crate::error::Result<Object>>>() { (__f)(beginIndex, endIndex)? } else { Default::default() };
            let _t4 = _t2.append_obj(Clone::clone(&_vdispatch3))?;
            let _t5 = _t4.append_str(Clone::clone(&String::from("\"")))?;
            let _t6 = _t5.toString()?;
            Ok(NumberFormatException::new_str(Clone::clone(&_t6))?)
        }
    }
}
