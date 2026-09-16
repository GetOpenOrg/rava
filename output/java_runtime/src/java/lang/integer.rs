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

impl From<Integer> for Number {
    fn from(v: Integer) -> Number { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/Integer"]
    #[super_class       = "java/lang/Number"]
    #[interfaces        = "java/lang/Comparable,java/lang/constant/Constable,java/lang/constant/ConstantDesc"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = "Ljava/lang/Number;Ljava/lang/Comparable<Ljava/lang/Integer;>;Ljava/lang/constant/Constable;Ljava/lang/constant/ConstantDesc;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Integer.java"]
    #[inner_classes     = "java/lang/Integer$IntegerCache:java/lang/Integer:IntegerCache:26;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Number"]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Integer;java/lang/Number;java/lang/Object;java/lang/constant/Constable;java/lang/constant/ConstantDesc"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct Integer {
        #[cfg_attr(any(), java_field(name = "value", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub value: i32,
    }

    impl Integer {
        #[cfg_attr(any(), java_field(name = "MIN_VALUE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "-2147483648"))]
        // static field: MIN_VALUE:I
        pub fn MIN_VALUE() -> i32 {
            -2147483648
        }

        #[cfg_attr(any(), java_field(name = "MAX_VALUE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "2147483647"))]
        // static field: MAX_VALUE:I
        pub fn MAX_VALUE() -> i32 {
            2147483647
        }

        #[cfg_attr(any(), java_field(name = "TYPE", descriptor = "Ljava/lang/Class;", access = "public", modifiers = "static final", is_static = true, generic_signature = "Ljava/lang/Class<Ljava/lang/Integer;>;"))]
        // static field: TYPE:Ljava/lang/Class;
        pub fn TYPE() -> Class<i32> {
            panic!("stub: java/lang/Integer.TYPE:Ljava/lang/Class;")
        }

        #[cfg_attr(any(), java_field(name = "digits", descriptor = "[C", access = "package", modifiers = "static final", is_static = true))]
        // static field: digits:[C
        pub fn digits() -> Rc<RefCell<Vec<u16>>> {
            panic!("stub: java/lang/Integer.digits:[C")
        }

        #[cfg_attr(any(), java_field(name = "DigitTens", descriptor = "[B", access = "package", modifiers = "static final", is_static = true))]
        // static field: DigitTens:[B
        pub fn DigitTens() -> Rc<RefCell<Vec<i8>>> {
            panic!("stub: java/lang/Integer.DigitTens:[B")
        }

        #[cfg_attr(any(), java_field(name = "DigitOnes", descriptor = "[B", access = "package", modifiers = "static final", is_static = true))]
        // static field: DigitOnes:[B
        pub fn DigitOnes() -> Rc<RefCell<Vec<i8>>> {
            panic!("stub: java/lang/Integer.DigitOnes:[B")
        }

        #[cfg_attr(any(), java_field(name = "SIZE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "32"))]
        // static field: SIZE:I
        pub fn SIZE() -> i32 {
            32
        }

        #[cfg_attr(any(), java_field(name = "BYTES", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: BYTES:I
        pub fn BYTES() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "1360826667806852920"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            1360826667806852920i64
        }

        #[java_method(name = "toString", descriptor = "(II)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString_i_i(i: i32, radix: i32) -> Result<String> {
            panic!("stub: java/lang/Integer.toString:(II)Ljava/lang/String;")
        }

        #[java_method(name = "toStringUTF16", descriptor = "(II)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toStringUTF16(i: i32, radix: i32) -> Result<String> {
            panic!("stub: java/lang/Integer.toStringUTF16:(II)Ljava/lang/String;")
        }

        #[java_method(name = "toUnsignedString", descriptor = "(II)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUnsignedString_i_i(i: i32, radix: i32) -> Result<String> {
            panic!("stub: java/lang/Integer.toUnsignedString:(II)Ljava/lang/String;")
        }

        #[java_method(name = "toHexString", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toHexString(mut i: i32) -> Result<String> {
            let _t0: String = Integer::toUnsignedString0(i, 4i32)?;
            Ok(_t0)
        }

        #[java_method(name = "toOctalString", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toOctalString(i: i32) -> Result<String> {
            panic!("stub: java/lang/Integer.toOctalString:(I)Ljava/lang/String;")
        }

        #[java_method(name = "toBinaryString", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toBinaryString(mut i: i32) -> Result<String> {
            let _t0: String = Integer::toUnsignedString0(i, 1i32)?;
            Ok(_t0)
        }

        #[java_method(name = "toUnsignedString0", descriptor = "(II)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUnsignedString0(mut val: i32, mut shift: i32) -> Result<String> {
            let _t0: i32 = Integer::numberOfLeadingZeros(val)?;
            let mut mag = (32i32).wrapping_sub(_t0);
            let _t1: i32 = Math::max_i_i(((mag).wrapping_add((shift).wrapping_sub(1i32))/shift), 1i32)?;
            let mut chars: i32 = _t1;
            if String::COMPACT_STRINGS() {
                let mut _arr2: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; chars as usize]));
                let mut buf: Rc<RefCell<Vec<i8>>> = _arr2;
                Integer::formatUnsignedInt(val, shift, Clone::clone(&buf), chars)?;
                return Ok(String::new_arr_b_b(Clone::clone(&buf), ((0i32) as i8))?);
            }
            let mut _arr2: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; (chars).wrapping_mul(2i32) as usize]));
            let mut buf: Rc<RefCell<Vec<i8>>> = _arr2;
            Integer::formatUnsignedIntUTF16(val, shift, Clone::clone(&buf), chars)?;
            Ok(String::new_arr_b_b(Clone::clone(&buf), ((1i32) as i8))?)
        }

        #[java_method(name = "formatUnsignedInt", descriptor = "(II[BI)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn formatUnsignedInt(mut val: i32, mut shift: i32, mut buf: Rc<RefCell<Vec<i8>>>, mut len: i32) -> Result<()> {
            let mut charPos: i32 = len;
            let mut radix = (1i32<<(shift&0x1f));
            let mut mask = (radix).wrapping_sub(1i32);
            loop {
                charPos = charPos.wrapping_sub(1i32);
                buf.borrow_mut()[charPos as usize] = ((((Integer::digits().borrow()[(val&mask) as usize] as i32)) as i8 as i32)) as i8;
                val = ((val as u32>>(shift&0x1f)) as i32);
                if (charPos<=0) { break; }
            }
            Ok(())
        }

        #[java_method(name = "formatUnsignedIntUTF16", descriptor = "(II[BI)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn formatUnsignedIntUTF16(mut val: i32, mut shift: i32, mut buf: Rc<RefCell<Vec<i8>>>, mut len: i32) -> Result<()> {
            let mut charPos: i32 = len;
            let mut radix = (1i32<<(shift&0x1f));
            let mut mask = (radix).wrapping_sub(1i32);
            loop {
                charPos = charPos.wrapping_sub(1i32);
                StringUTF16::putChar(Clone::clone(&buf), charPos, (Integer::digits().borrow()[(val&mask) as usize] as i32))?;
                val = ((val as u32>>(shift&0x1f)) as i32);
                if (charPos<=0) { break; }
            }
            Ok(())
        }

        #[java_method(name = "toString", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toString(I)Ljava/lang/String;
        pub fn toString_i(mut i: i32) -> Result<String> {
            let _t0: i32 = Integer::stringSize(i)?;
            let mut size: i32 = _t0;
            if String::COMPACT_STRINGS() {
                let mut _arr1: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; size as usize]));
                let mut buf: Rc<RefCell<Vec<i8>>> = _arr1;
                let _t2: i32 = Integer::getChars(i, size, Clone::clone(&buf))?;
                return Ok(String::new_arr_b_b(Clone::clone(&buf), ((0i32) as i8))?);
            }
            let mut _arr1: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; (size).wrapping_mul(2i32) as usize]));
            let mut buf: Rc<RefCell<Vec<i8>>> = _arr1;
            let _t2: i32 = StringUTF16::getChars_i_i_arr_b(i, size, Clone::clone(&buf))?;
            Ok(String::new_arr_b_b(Clone::clone(&buf), ((1i32) as i8))?)
        }

        #[java_method(name = "toUnsignedString", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUnsignedString_i(i: i32) -> Result<String> {
            panic!("stub: java/lang/Integer.toUnsignedString:(I)Ljava/lang/String;")
        }

        #[java_method(name = "getChars", descriptor = "(II[B)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getChars(mut i: i32, mut index: i32, mut buf: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            let mut charPos: i32 = index;
            let mut negative = ((i<0)) as i32;
            if (negative==0) {
                i = (i).wrapping_neg();
            }
            loop {
                if i > -100i32 { break; }
                let mut q = (i/100i32);
                let mut r = ((q).wrapping_mul(100i32)).wrapping_sub(i);
                i = q;
                charPos = charPos.wrapping_sub(1i32);
                buf.borrow_mut()[charPos as usize] = ((Integer::DigitOnes().borrow()[r as usize] as i32)) as i8;
                charPos = charPos.wrapping_sub(1i32);
                buf.borrow_mut()[charPos as usize] = ((Integer::DigitTens().borrow()[r as usize] as i32)) as i8;
            }
            charPos = charPos.wrapping_sub(1i32);
            buf.borrow_mut()[charPos as usize] = ((Integer::DigitOnes().borrow()[(i).wrapping_neg() as usize] as i32)) as i8;
            if i < -9i32 {
                charPos = charPos.wrapping_sub(1i32);
                buf.borrow_mut()[charPos as usize] = ((Integer::DigitTens().borrow()[(i).wrapping_neg() as usize] as i32)) as i8;
            }
            if (negative!=0) {
                charPos = charPos.wrapping_sub(1i32);
                buf.borrow_mut()[charPos as usize] = (45i32) as i8;
            }
            Ok(charPos)
        }

        #[java_method(name = "stringSize", descriptor = "(I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stringSize(mut x: i32) -> Result<i32> {
            let mut d: i32 = 1i32;
            if (x>=0) {
                d = 0i32;
                x = (x).wrapping_neg();
            }
            let mut p: i32 = -10i32;
            let mut i: i32 = 1i32;
            loop {
                if i >= 10i32 { break; }
                if x > p {
                    return Ok((i).wrapping_add(d));
                }
                p = (10i32).wrapping_mul(p);
                i = i.wrapping_add(1i32);
            }
            Ok((10i32).wrapping_add(d))
        }

        #[java_method(name = "parseInt", descriptor = "(Ljava/lang/String;I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        // java: parseInt(Ljava/lang/String;I)I
        pub fn parseInt_str_i(mut s: String, mut radix: i32) -> Result<i32> {
            if _is_jnull(&s) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if radix < 2i32 {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("radix ")))?;
                let _t1 = _t0.append_i(radix)?;
                let _t2 = _t1.append_str(Clone::clone(&String::from(" less than Character.MIN_RADIX")))?;
                let _t3 = _t2.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if radix > 36i32 {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("radix ")))?;
                let _t1 = _t0.append_i(radix)?;
                let _t2 = _t1.append_str(Clone::clone(&String::from(" greater than Character.MAX_RADIX")))?;
                let _t3 = _t2.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut negative: i32 = 0i32;
            let mut i: i32 = 0i32;
            let _t0 = s.length()?;
            let mut len: i32 = _t0;
            let mut limit: i32 = -2147483647i32;
            let _t1 = s.charAt(0i32)?;
            let mut firstChar: u16 = _t1;
            if (firstChar as i32) == 45i32 {
                negative = 1i32;
                limit = -2147483648i32;
            } else {
                if (firstChar as i32) != 43i32 {
                    let _t2: NumberFormatException = NumberFormatException::forInputString(Clone::clone(&s), radix)?;
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
            }
            if len == 1i32 {
                let _t2: NumberFormatException = NumberFormatException::forInputString(Clone::clone(&s), radix)?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            i = i.wrapping_add(1i32);
            let mut multmin = (limit/radix);
            let mut result: i32 = 0i32;
            loop {
                if i >= len { break; }
                i = i.wrapping_add(1i32);
                let _t2 = s.charAt(i)?;
                let _t3: i32 = Character::digit_c_i(_t2, radix)?;
                let mut digit: i32 = _t3;
                if result < multmin {
                    let _t4: NumberFormatException = NumberFormatException::forInputString(Clone::clone(&s), radix)?;
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                result = (result).wrapping_mul(radix);
                if result < (limit).wrapping_add(digit) {
                    let _t4: NumberFormatException = NumberFormatException::forInputString(Clone::clone(&s), radix)?;
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                result = (result).wrapping_sub(digit);
            }
            return Ok((if (negative!=0) { result } else { (result).wrapping_neg() }));
            let _t2: NumberFormatException = NumberFormatException::forInputString(Clone::clone(&s), radix)?;
            return Err(JvmError::Custom("athrow".to_owned()));
        }

        #[java_method(name = "parseInt", descriptor = "(Ljava/lang/CharSequence;III)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        // java: parseInt(Ljava/lang/CharSequence;III)I
        pub fn parseInt_seq_i_i_i(mut s: Object, mut beginIndex: i32, mut endIndex: i32, mut radix: i32) -> Result<i32> {
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&s))?;
            let _vdispatch1: i32 = if let Some(_d) = s.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.length()? } else if let Some(_d) = s.0.as_any().downcast_ref::<CharBuffer>() { _d.length()? } else if let Some(_d) = s.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.length()? } else if let Some(_d) = s.0.as_any().downcast_ref::<String>() { _d.length()? } else if let Some(_d) = s.0.as_any().downcast_ref::<StringBuilder>() { _d.length()? } else if let Some(_d) = s.0.as_any().downcast_ref::<Object>() { _d.length()? } else if let Some(__f) = s.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            let _t2: i32 = Objects::checkFromToIndex_i_i_i(beginIndex, endIndex, _vdispatch1)?;
            if radix < 2i32 {
                let _t3 = StringBuilder::new()?.append_str(Clone::clone(&String::from("radix ")))?;
                let _t4 = _t3.append_i(radix)?;
                let _t5 = _t4.append_str(Clone::clone(&String::from(" less than Character.MIN_RADIX")))?;
                let _t6 = _t5.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if radix > 36i32 {
                let _t3 = StringBuilder::new()?.append_str(Clone::clone(&String::from("radix ")))?;
                let _t4 = _t3.append_i(radix)?;
                let _t5 = _t4.append_str(Clone::clone(&String::from(" greater than Character.MAX_RADIX")))?;
                let _t6 = _t5.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut negative: i32 = 0i32;
            let mut i: i32 = beginIndex;
            let mut limit: i32 = -2147483647i32;
            let _vdispatch3: u16 = if let Some(_d) = s.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<CharBuffer>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<String>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<StringBuilder>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<Object>() { _d.charAt(i)? } else if let Some(__f) = s.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<u16>>>() { (__f)(i)? } else { Default::default() };
            let mut firstChar: u16 = _vdispatch3;
            if (firstChar as i32) == 45i32 {
                negative = 1i32;
                limit = -2147483648i32;
            } else {
                if (firstChar as i32) != 43i32 {
                    let _t4: NumberFormatException = NumberFormatException::forCharSequence(Clone::clone(&s), beginIndex, endIndex, i)?;
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
            }
            i = i.wrapping_add(1i32);
            if i == endIndex {
                let _t4: NumberFormatException = NumberFormatException::forCharSequence(Clone::clone(&s), beginIndex, endIndex, i)?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut multmin = (limit/radix);
            let mut result: i32 = 0i32;
            loop {
                if i >= endIndex { break; }
                let _vdispatch4: u16 = if let Some(_d) = s.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<CharBuffer>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<String>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<StringBuilder>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<Object>() { _d.charAt(i)? } else if let Some(__f) = s.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<u16>>>() { (__f)(i)? } else { Default::default() };
                let _t5: i32 = Character::digit_c_i(_vdispatch4, radix)?;
                let mut digit: i32 = _t5;
                if result < multmin {
                    let _t6: NumberFormatException = NumberFormatException::forCharSequence(Clone::clone(&s), beginIndex, endIndex, i)?;
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                result = (result).wrapping_mul(radix);
                if result < (limit).wrapping_add(digit) {
                    let _t6: NumberFormatException = NumberFormatException::forCharSequence(Clone::clone(&s), beginIndex, endIndex, i)?;
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                i = i.wrapping_add(1i32);
                result = (result).wrapping_sub(digit);
            }
            return Ok((if (negative!=0) { result } else { (result).wrapping_neg() }));
            let _t4: NumberFormatException = NumberFormatException::forInputString(Clone::clone(&String::from("")), radix)?;
            return Err(JvmError::Custom("athrow".to_owned()));
        }

        #[java_method(name = "parseInt", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        // java: parseInt(Ljava/lang/String;)I
        pub fn parseInt_str(mut s: String) -> Result<i32> {
            let _t0: i32 = Integer::parseInt_str_i(Clone::clone(&s), 10i32)?;
            Ok(_t0)
        }

        #[java_method(name = "parseUnsignedInt", descriptor = "(Ljava/lang/String;I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn parseUnsignedInt_str_i(s: String, radix: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.parseUnsignedInt:(Ljava/lang/String;I)I")
        }

        #[java_method(name = "parseUnsignedInt", descriptor = "(Ljava/lang/CharSequence;III)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn parseUnsignedInt_seq_i_i_i(s: Object, beginIndex: i32, endIndex: i32, radix: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.parseUnsignedInt:(Ljava/lang/CharSequence;III)I")
        }

        #[java_method(name = "parseUnsignedInt", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn parseUnsignedInt_str(s: String) -> Result<i32> {
            panic!("stub: java/lang/Integer.parseUnsignedInt:(Ljava/lang/String;)I")
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;I)Ljava/lang/Integer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn valueOf_str_i(s: String, radix: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.valueOf:(Ljava/lang/String;I)Ljava/lang/Integer;")
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/lang/Integer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn valueOf_str(s: String) -> Result<i32> {
            panic!("stub: java/lang/Integer.valueOf:(Ljava/lang/String;)Ljava/lang/Integer;")
        }

        #[java_method(name = "valueOf", descriptor = "(I)Ljava/lang/Integer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: valueOf(I)Ljava/lang/Integer;
        pub fn valueOf_i(mut i: i32) -> Result<i32> {
            if i <= Integer_IntegerCache::high() {
                return Ok(Integer_IntegerCache::cache().borrow()[(i).wrapping_add(128i32) as usize]);
            }
            Ok(i)
        }

        #[java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        // java: <init>(I)V
        pub fn new_i(mut value: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Number::new()?);
            this.__set_value(value);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException", is_deprecated = true)]
        pub fn new_str(s: String) -> Result<Self> {
            panic!("stub: java/lang/Integer.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "byteValue", descriptor = "()B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn byteValue(&self) -> Result<i8> {
            panic!("stub: java/lang/Integer.byteValue:()B")
        }

        #[java_method(name = "shortValue", descriptor = "()S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn shortValue(&self) -> Result<i16> {
            panic!("stub: java/lang/Integer.shortValue:()S")
        }

        #[java_method(name = "intValue", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn intValue(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_value())
        }

        #[java_method(name = "longValue", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longValue(&self) -> Result<i64> {
            panic!("stub: java/lang/Integer.longValue:()J")
        }

        #[java_method(name = "floatValue", descriptor = "()F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floatValue(&self) -> Result<f32> {
            panic!("stub: java/lang/Integer.floatValue:()F")
        }

        #[java_method(name = "doubleValue", descriptor = "()D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubleValue(&self) -> Result<f64> {
            panic!("stub: java/lang/Integer.doubleValue:()D")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "hashCode", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode_i(value: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.hashCode:(I)I")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, mut obj: Object) -> Result<bool> {
            let this = self;
            return Ok(this.__get_value() == (obj).downcast::<i32>());
            Ok((0i32 != 0i32))
        }

        #[java_method(name = "getInteger", descriptor = "(Ljava/lang/String;)Ljava/lang/Integer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInteger_str(nm: String) -> Result<i32> {
            panic!("stub: java/lang/Integer.getInteger:(Ljava/lang/String;)Ljava/lang/Integer;")
        }

        #[java_method(name = "getInteger", descriptor = "(Ljava/lang/String;I)Ljava/lang/Integer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInteger_str_i(nm: String, val: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.getInteger:(Ljava/lang/String;I)Ljava/lang/Integer;")
        }

        #[java_method(name = "getInteger", descriptor = "(Ljava/lang/String;Ljava/lang/Integer;)Ljava/lang/Integer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInteger_str_int(nm: String, val: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.getInteger:(Ljava/lang/String;Ljava/lang/Integer;)Ljava/lang/Integer;")
        }

        #[java_method(name = "decode", descriptor = "(Ljava/lang/String;)Ljava/lang/Integer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn decode(nm: String) -> Result<i32> {
            panic!("stub: java/lang/Integer.decode:(Ljava/lang/String;)Ljava/lang/Integer;")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/lang/Integer;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, mut anotherInteger: i32) -> Result<i32> {
            let this = self;
            let _t0: i32 = Integer::compare(this.__get_value(), anotherInteger.__get_value())?;
            Ok(_t0)
        }

        #[java_method(name = "compare", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare(mut x: i32, mut y: i32) -> Result<i32> {
            Ok(((if x < y { (-1i32 != 0) } else { x != y })) as i32)
        }

        #[java_method(name = "compareUnsigned", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareUnsigned(mut x: i32, mut y: i32) -> Result<i32> {
            let _t0: i32 = Integer::compare((x).wrapping_add(-2147483648i32), (y).wrapping_add(-2147483648i32))?;
            Ok(_t0)
        }

        #[java_method(name = "toUnsignedLong", descriptor = "(I)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUnsignedLong(x: i32) -> Result<i64> {
            panic!("stub: java/lang/Integer.toUnsignedLong:(I)J")
        }

        #[java_method(name = "divideUnsigned", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideUnsigned(dividend: i32, divisor: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.divideUnsigned:(II)I")
        }

        #[java_method(name = "remainderUnsigned", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remainderUnsigned(dividend: i32, divisor: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.remainderUnsigned:(II)I")
        }

        #[java_method(name = "highestOneBit", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn highestOneBit(i: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.highestOneBit:(I)I")
        }

        #[java_method(name = "lowestOneBit", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lowestOneBit(i: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.lowestOneBit:(I)I")
        }

        #[java_method(name = "numberOfLeadingZeros", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn numberOfLeadingZeros(mut i: i32) -> Result<i32> {
            return Ok((if (i==0) { 32i32 } else { 0i32 }));
            let mut n: i32 = 31i32;
            if i >= 264i32 {
                n = n.wrapping_sub(16i32);
                i = ((i as u32>>(16i32&0x1f)) as i32);
            }
            if i >= 256i32 {
                n = n.wrapping_sub(8i32);
                i = ((i as u32>>(8i32&0x1f)) as i32);
            }
            if i >= 16i32 {
                n = n.wrapping_sub(4i32);
                i = ((i as u32>>(4i32&0x1f)) as i32);
            }
            if i >= 4i32 {
                n = n.wrapping_sub(2i32);
                i = ((i as u32>>(2i32&0x1f)) as i32);
            }
            Ok((n).wrapping_sub(((i as u32>>(1i32&0x1f)) as i32)))
        }

        #[java_method(name = "numberOfTrailingZeros", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn numberOfTrailingZeros(mut i: i32) -> Result<i32> {
            i = ((i^-1i32)&(i).wrapping_sub(1i32));
            if (i<=0) {
                return Ok((i&32i32));
            }
            let mut n: i32 = 1i32;
            if i > 264i32 {
                n = n.wrapping_add(16i32);
                i = ((i as u32>>(16i32&0x1f)) as i32);
            }
            if i > 256i32 {
                n = n.wrapping_add(8i32);
                i = ((i as u32>>(8i32&0x1f)) as i32);
            }
            if i > 16i32 {
                n = n.wrapping_add(4i32);
                i = ((i as u32>>(4i32&0x1f)) as i32);
            }
            if i > 4i32 {
                n = n.wrapping_add(2i32);
                i = ((i as u32>>(2i32&0x1f)) as i32);
            }
            Ok((n).wrapping_add(((i as u32>>(1i32&0x1f)) as i32)))
        }

        #[java_method(name = "bitCount", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn bitCount(mut i: i32) -> Result<i32> {
            i = (i).wrapping_sub((((i as u32>>(1i32&0x1f)) as i32)&265i32));
            i = ((i&266i32)).wrapping_add((((i as u32>>(2i32&0x1f)) as i32)&266i32));
            i = ((i).wrapping_add(((i as u32>>(4i32&0x1f)) as i32))&267i32);
            i = (i).wrapping_add(((i as u32>>(8i32&0x1f)) as i32));
            i = (i).wrapping_add(((i as u32>>(16i32&0x1f)) as i32));
            Ok((i&63i32))
        }

        #[java_method(name = "rotateLeft", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn rotateLeft(i: i32, distance: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.rotateLeft:(II)I")
        }

        #[java_method(name = "rotateRight", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn rotateRight(i: i32, distance: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.rotateRight:(II)I")
        }

        #[java_method(name = "reverse", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reverse(i: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.reverse:(I)I")
        }

        #[java_method(name = "compress", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compress(i: i32, mask: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.compress:(II)I")
        }

        #[java_method(name = "expand", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn expand(i: i32, mask: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.expand:(II)I")
        }

        #[java_method(name = "parallelSuffix", descriptor = "(I)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelSuffix(maskCount: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.parallelSuffix:(I)I")
        }

        #[java_method(name = "signum", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn signum(i: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.signum:(I)I")
        }

        #[java_method(name = "reverseBytes", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reverseBytes(i: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.reverseBytes:(I)I")
        }

        #[java_method(name = "sum", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sum(a: i32, b: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.sum:(II)I")
        }

        #[java_method(name = "max", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn max(a: i32, b: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.max:(II)I")
        }

        #[java_method(name = "min", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn min(a: i32, b: i32) -> Result<i32> {
            panic!("stub: java/lang/Integer.min:(II)I")
        }

        #[java_method(name = "describeConstable", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Optional<Ljava/lang/Integer;>;")]
        pub fn describeConstable(&self) -> Result<Optional<Object>> {
            panic!("stub: java/lang/Integer.describeConstable:()Ljava/util/Optional;")
        }

        #[java_method(name = "resolveConstantDesc", descriptor = "(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Integer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn resolveConstantDesc(&self, lookup: Object) -> Result<i32> {
            panic!("stub: java/lang/Integer.resolveConstantDesc:(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Integer;")
        }
    }
}
