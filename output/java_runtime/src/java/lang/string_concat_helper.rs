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
use crate::jdk::internal::misc::Unsafe;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/StringConcatHelper"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "StringConcatHelper.java"]
    #[inner_classes     = "java/lang/StringConcatHelper$LateInit:java/lang/StringConcatHelper:LateInit:10;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25;jdk/internal/javac/PreviewFeature$Feature:jdk/internal/javac/PreviewFeature:Feature:16409"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/lang/StringConcatHelper"]

    pub struct StringConcatHelper;

    impl StringConcatHelper {
        #[cfg_attr(any(), java_field(name = "LATIN1", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: LATIN1:J
        pub fn LATIN1() -> i64 {
            0i64
        }

        #[cfg_attr(any(), java_field(name = "UTF16", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "4294967296"))]
        // static field: UTF16:J
        pub fn UTF16() -> i64 {
            4294967296i64
        }

        #[cfg_attr(any(), java_field(name = "UNSAFE", descriptor = "Ljdk/internal/misc/Unsafe;", access = "private", modifiers = "static final", is_static = true))]
        // static field: UNSAFE:Ljdk/internal/misc/Unsafe;
        pub fn UNSAFE() -> Unsafe {
            panic!("stub: java/lang/StringConcatHelper.UNSAFE:Ljdk/internal/misc/Unsafe;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/StringConcatHelper.<init>:()V")
        }

        #[java_method(name = "coder", descriptor = "(C)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn coder(value: u16) -> Result<i64> {
            panic!("stub: java/lang/StringConcatHelper.coder:(C)J")
        }

        #[java_method(name = "checkOverflow", descriptor = "(J)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkOverflow(mut lengthCoder: i64) -> Result<i64> {
            if ((lengthCoder as i32)>=0) {
                return Ok(lengthCoder);
            }
            return Err(JvmError::Custom("athrow".to_owned()));
        }

        #[java_method(name = "mix", descriptor = "(JZ)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mix_l_z(lengthCoder: i64, arg1: bool) -> Result<i64> {
            panic!("stub: java/lang/StringConcatHelper.mix:(JZ)J")
        }

        #[java_method(name = "mix", descriptor = "(JC)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mix_l_c(lengthCoder: i64, arg1: u16) -> Result<i64> {
            panic!("stub: java/lang/StringConcatHelper.mix:(JC)J")
        }

        #[java_method(name = "mix", descriptor = "(JI)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mix_l_i(lengthCoder: i64, arg1: i32) -> Result<i64> {
            panic!("stub: java/lang/StringConcatHelper.mix:(JI)J")
        }

        #[java_method(name = "mix", descriptor = "(JJ)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mix_l_l(lengthCoder: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/StringConcatHelper.mix:(JJ)J")
        }

        #[java_method(name = "mix", descriptor = "(JLjava/lang/String;)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: mix(JLjava/lang/String;)J
        pub fn mix_l_str(mut lengthCoder: i64, mut value: String) -> Result<i64> {
            let _t0 = value.length()?;
            lengthCoder = (lengthCoder).wrapping_add((_t0 as i64));
            let _t1 = value.coder()?;
            if (_t1 as i32) == 1i32 {
                lengthCoder = (lengthCoder|(4294967296i64));
            }
            let _t2: i64 = StringConcatHelper::checkOverflow(lengthCoder)?;
            Ok(_t2)
        }

        #[java_method(name = "mix", descriptor = "(JLjdk/internal/util/FormatConcatItem;)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mix_l_format(lengthCoder: i64, arg1: Object) -> Result<i64> {
            panic!("stub: java/lang/StringConcatHelper.mix:(JLjdk/internal/util/FormatConcatItem;)J")
        }

        #[java_method(name = "prepend", descriptor = "(J[BZ)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn prepend_l_arr_b_z(indexCoder: i64, arg1: Rc<RefCell<Vec<i8>>>, buf: bool) -> Result<i64> {
            panic!("stub: java/lang/StringConcatHelper.prepend:(J[BZ)J")
        }

        #[java_method(name = "prepend", descriptor = "(J[BZLjava/lang/String;)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn prepend_l_arr_b_z_str(indexCoder: i64, arg1: Rc<RefCell<Vec<i8>>>, buf: bool, value: String) -> Result<i64> {
            panic!("stub: java/lang/StringConcatHelper.prepend:(J[BZLjava/lang/String;)J")
        }

        #[java_method(name = "prepend", descriptor = "(J[BC)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn prepend_l_arr_b_c(indexCoder: i64, arg1: Rc<RefCell<Vec<i8>>>, buf: u16) -> Result<i64> {
            panic!("stub: java/lang/StringConcatHelper.prepend:(J[BC)J")
        }

        #[java_method(name = "prepend", descriptor = "(J[BCLjava/lang/String;)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn prepend_l_arr_b_c_str(indexCoder: i64, arg1: Rc<RefCell<Vec<i8>>>, buf: u16, value: String) -> Result<i64> {
            panic!("stub: java/lang/StringConcatHelper.prepend:(J[BCLjava/lang/String;)J")
        }

        #[java_method(name = "prepend", descriptor = "(J[BI)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn prepend_l_arr_b_i(indexCoder: i64, arg1: Rc<RefCell<Vec<i8>>>, buf: i32) -> Result<i64> {
            panic!("stub: java/lang/StringConcatHelper.prepend:(J[BI)J")
        }

        #[java_method(name = "prepend", descriptor = "(J[BILjava/lang/String;)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn prepend_l_arr_b_i_str(indexCoder: i64, arg1: Rc<RefCell<Vec<i8>>>, buf: i32, value: String) -> Result<i64> {
            panic!("stub: java/lang/StringConcatHelper.prepend:(J[BILjava/lang/String;)J")
        }

        #[java_method(name = "prepend", descriptor = "(J[BJ)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn prepend_l_arr_b_l(indexCoder: i64, arg1: Rc<RefCell<Vec<i8>>>, buf: i64) -> Result<i64> {
            panic!("stub: java/lang/StringConcatHelper.prepend:(J[BJ)J")
        }

        #[java_method(name = "prepend", descriptor = "(J[BJLjava/lang/String;)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn prepend_l_arr_b_l_str(indexCoder: i64, arg1: Rc<RefCell<Vec<i8>>>, buf: i64, value: String) -> Result<i64> {
            panic!("stub: java/lang/StringConcatHelper.prepend:(J[BJLjava/lang/String;)J")
        }

        #[java_method(name = "prepend", descriptor = "(J[BLjava/lang/String;)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: prepend(J[BLjava/lang/String;)J
        pub fn prepend_l_arr_b_str(mut indexCoder: i64, mut buf: Rc<RefCell<Vec<i8>>>, mut value: String) -> Result<i64> {
            let _t0 = value.length()?;
            indexCoder = (indexCoder).wrapping_sub((_t0 as i64));
            if (((indexCoder>(4294967296i64)) as i32-((indexCoder)<(4294967296i64)) as i32)<0) {
                value.getBytes_arr_b_i_b(Clone::clone(&buf), (indexCoder as i32), ((0i32) as i8))?;
            } else {
                value.getBytes_arr_b_i_b(Clone::clone(&buf), (indexCoder as i32), ((1i32) as i8))?;
            }
            Ok(indexCoder)
        }

        #[java_method(name = "prepend", descriptor = "(J[BLjava/lang/String;Ljava/lang/String;)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn prepend_l_arr_b_str_str(indexCoder: i64, arg1: Rc<RefCell<Vec<i8>>>, buf: String, value: String) -> Result<i64> {
            panic!("stub: java/lang/StringConcatHelper.prepend:(J[BLjava/lang/String;Ljava/lang/String;)J")
        }

        #[java_method(name = "prepend", descriptor = "(J[BLjdk/internal/util/FormatConcatItem;)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn prepend_l_arr_b_format(indexCoder: i64, arg1: Rc<RefCell<Vec<i8>>>, buf: Object) -> Result<i64> {
            panic!("stub: java/lang/StringConcatHelper.prepend:(J[BLjdk/internal/util/FormatConcatItem;)J")
        }

        #[java_method(name = "prepend", descriptor = "(J[BLjdk/internal/util/FormatConcatItem;Ljava/lang/String;)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn prepend_l_arr_b_format_str(indexCoder: i64, arg1: Rc<RefCell<Vec<i8>>>, buf: Object, value: String) -> Result<i64> {
            panic!("stub: java/lang/StringConcatHelper.prepend:(J[BLjdk/internal/util/FormatConcatItem;Ljava/lang/String;)J")
        }

        #[java_method(name = "newString", descriptor = "([BJ)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newString(mut buf: Rc<RefCell<Vec<i8>>>, mut indexCoder: i64) -> Result<String> {
            if (((indexCoder>(0i64)) as i32-((indexCoder)<(0i64)) as i32)==0) {
                return Ok(String::new_arr_b_b(Clone::clone(&buf), ((0i32) as i8))?);
            }
            if (((indexCoder>(4294967296i64)) as i32-((indexCoder)<(4294967296i64)) as i32)==0) {
                return Ok(String::new_arr_b_b(Clone::clone(&buf), ((1i32) as i8))?);
            }
            let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Storage is not completely initialized, ")))?;
            let _t1 = _t0.append_i((indexCoder as i32))?;
            let _t2 = _t1.append_str(Clone::clone(&String::from(" bytes left")))?;
            let _t3 = _t2.toString()?;
            return Err(JvmError::Custom("athrow".to_owned()));
        }

        #[java_method(name = "simpleConcat", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn simpleConcat(mut first: Object, mut second: Object) -> Result<String> {
            let _t0: String = StringConcatHelper::stringOf(Clone::clone(&first))?;
            let mut s1: String = _t0;
            let _t1: String = StringConcatHelper::stringOf(Clone::clone(&second))?;
            let mut s2: String = _t1;
            let _t2 = s1.isEmpty()?;
            if _t2 {
                return Ok(String::new_str(Clone::clone(&s2))?);
            }
            let _t3 = s2.isEmpty()?;
            if _t3 {
                return Ok(String::new_str(Clone::clone(&s1))?);
            }
            let _t4: i64 = StringConcatHelper::initialCoder()?;
            let _t5: i64 = StringConcatHelper::mix_l_str(_t4, Clone::clone(&s1))?;
            let mut indexCoder: i64 = _t5;
            let _t6: i64 = StringConcatHelper::mix_l_str(indexCoder, Clone::clone(&s2))?;
            indexCoder = _t6;
            let _t7: Rc<RefCell<Vec<i8>>> = StringConcatHelper::newArray(indexCoder)?;
            let mut buf: Rc<RefCell<Vec<i8>>> = _t7;
            let _t8: i64 = StringConcatHelper::prepend_l_arr_b_str(indexCoder, Clone::clone(&buf), Clone::clone(&s2))?;
            indexCoder = _t8;
            let _t9: i64 = StringConcatHelper::prepend_l_arr_b_str(indexCoder, Clone::clone(&buf), Clone::clone(&s1))?;
            indexCoder = _t9;
            let _t10: String = StringConcatHelper::newString(Clone::clone(&buf), indexCoder)?;
            Ok(_t10)
        }

        #[java_method(name = "newStringOf", descriptor = "(Ljava/lang/Object;)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newStringOf(arg: Object) -> Result<String> {
            panic!("stub: java/lang/StringConcatHelper.newStringOf:(Ljava/lang/Object;)Ljava/lang/String;")
        }

        #[java_method(name = "stringOf", descriptor = "(Ljava/lang/Object;)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stringOf(mut value: Object) -> Result<String> {
            let _vdispatch0: String = if let Some(__f) = value.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
            let mut s: String = _vdispatch0;
            Ok((if _is_jnull(&s) { String::from("null") } else { s }))
        }

        #[java_method(name = "newArrayWithSuffix", descriptor = "(Ljava/lang/String;J)[B", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newArrayWithSuffix(suffix: String, indexCoder: i64) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/lang/StringConcatHelper.newArrayWithSuffix:(Ljava/lang/String;J)[B")
        }

        #[java_method(name = "newArray", descriptor = "(J)[B", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newArray(mut indexCoder: i64) -> Result<Rc<RefCell<Vec<i8>>>> {
            let mut coder = ((((indexCoder).wrapping_shr((32i32&0x3f) as u32) as i32)) as i8 as i32);
            let mut index = ((indexCoder as i32)<<(coder&0x1f));
            if (index<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0 = StringConcatHelper::UNSAFE().allocateUninitializedArray(Clone::clone(&Byte::TYPE()), index)?;
            Ok((_t0).downcast::<Rc<RefCell<Vec<i8>>>>())
        }

        #[java_method(name = "initialCoder", descriptor = "()J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn initialCoder() -> Result<i64> {
            Ok((if String::COMPACT_STRINGS() { 0i64 } else { 4294967296i64 }))
        }

        #[java_method(name = "getCharLatin1", descriptor = "([BI)C", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCharLatin1(buffer: Rc<RefCell<Vec<i8>>>, index: i32) -> Result<u16> {
            panic!("stub: java/lang/StringConcatHelper.getCharLatin1:([BI)C")
        }

        #[java_method(name = "getCharUTF16", descriptor = "([BI)C", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCharUTF16(buffer: Rc<RefCell<Vec<i8>>>, index: i32) -> Result<u16> {
            panic!("stub: java/lang/StringConcatHelper.getCharUTF16:([BI)C")
        }

        #[java_method(name = "putCharLatin1", descriptor = "([BII)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putCharLatin1(buffer: Rc<RefCell<Vec<i8>>>, index: i32, ch: i32) -> Result<()> {
            panic!("stub: java/lang/StringConcatHelper.putCharLatin1:([BII)V")
        }

        #[java_method(name = "putCharUTF16", descriptor = "([BII)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putCharUTF16(buffer: Rc<RefCell<Vec<i8>>>, index: i32, ch: i32) -> Result<()> {
            panic!("stub: java/lang/StringConcatHelper.putCharUTF16:([BII)V")
        }

        #[java_method(name = "selectGetChar", descriptor = "(J)Ljava/lang/invoke/MethodHandle;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn selectGetChar(indexCoder: i64) -> Result<Object> {
            panic!("stub: java/lang/StringConcatHelper.selectGetChar:(J)Ljava/lang/invoke/MethodHandle;")
        }

        #[java_method(name = "selectPutChar", descriptor = "(J)Ljava/lang/invoke/MethodHandle;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn selectPutChar(indexCoder: i64) -> Result<Object> {
            panic!("stub: java/lang/StringConcatHelper.selectPutChar:(J)Ljava/lang/invoke/MethodHandle;")
        }

        #[java_method(name = "lookupStatic", descriptor = "(Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lookupStatic(name: String, methodType: Object) -> Result<Object> {
            panic!("stub: java/lang/StringConcatHelper.lookupStatic:(Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;")
        }
    }
}
