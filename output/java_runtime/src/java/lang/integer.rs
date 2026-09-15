#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/Integer",
    super_class       = "java/lang/Number",
    interfaces        = "java/lang/Comparable,java/lang/constant/Constable,java/lang/constant/ConstantDesc",
    access            = "public",
    modifiers         = "final",
    generic_signature = "Ljava/lang/Number;Ljava/lang/Comparable<Ljava/lang/Integer;>;Ljava/lang/constant/Constable;Ljava/lang/constant/ConstantDesc;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Integer.java",
    inner_classes     = "java/lang/Integer$IntegerCache:java/lang/Integer:IntegerCache:26;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Integer {
    pub _super: Number,
    #[cfg_attr(any(), java_field(name = "value", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
    pub value: JField<i32>,
}

impl Integer {
    pub fn as_number(&self) -> &Number { &self._super }
    pub fn into_number(self) -> Number { self._super }
}

impl From<Integer> for Number {
    fn from(v: Integer) -> Number { v._super }
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
    pub fn TYPE() -> Object {
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

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "(II)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toString_i_i(i: i32, radix: i32) -> Result<String> {
        panic!("stub: java/lang/Integer.toString:(II)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toStringUTF16", descriptor = "(II)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toStringUTF16(i: i32, radix: i32) -> Result<String> {
        panic!("stub: java/lang/Integer.toStringUTF16:(II)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toUnsignedString", descriptor = "(II)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toUnsignedString_i_i(i: i32, radix: i32) -> Result<String> {
        panic!("stub: java/lang/Integer.toUnsignedString:(II)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toHexString", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toHexString(i: i32) -> Result<String> {
        panic!("stub: java/lang/Integer.toHexString:(I)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toOctalString", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toOctalString(i: i32) -> Result<String> {
        panic!("stub: java/lang/Integer.toOctalString:(I)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toBinaryString", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toBinaryString(i: i32) -> Result<String> {
        panic!("stub: java/lang/Integer.toBinaryString:(I)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "toUnsignedString0", descriptor = "(II)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toUnsignedString0(val: i32, shift: i32) -> Result<String> {
        panic!("stub: java/lang/Integer.toUnsignedString0:(II)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "formatUnsignedInt", descriptor = "(II[BI)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn formatUnsignedInt(val: i32, shift: i32, buf: Rc<RefCell<Vec<i8>>>, len: i32) -> Result<()> {
        panic!("stub: java/lang/Integer.formatUnsignedInt:(II[BI)V")
    }

    #[cfg_attr(any(), java_method(name = "formatUnsignedIntUTF16", descriptor = "(II[BI)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn formatUnsignedIntUTF16(val: i32, shift: i32, buf: Rc<RefCell<Vec<i8>>>, len: i32) -> Result<()> {
        panic!("stub: java/lang/Integer.formatUnsignedIntUTF16:(II[BI)V")
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: toString(I)Ljava/lang/String;
    pub fn toString_i(mut i: i32) -> Result<String> {
        let _t0: i32 = Integer::stringSize(i)?;
        let mut size: i32 = _t0;
    let mut buf: Rc<RefCell<Vec<i8>>> = Default::default();
        if String::COMPACT_STRINGS() {
            let mut _arr1: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; size as usize]));
            buf = _arr1;
            let _t2: i32 = Integer::getChars(i, size, Clone::clone(&buf))?;
            return Ok(String::new_arr_b_b(Clone::clone(&buf), ((0i32) as i8))?);
        }
        let mut _arr1: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; (size).wrapping_mul(2i32) as usize]));
        let mut buf: Rc<RefCell<Vec<i8>>> = _arr1;
        let _t2: i32 = StringUTF16::getChars_i_i_arr_b(i, size, Clone::clone(&buf))?;
        Ok(String::new_arr_b_b(Clone::clone(&buf), ((1i32) as i8))?)
    }

    #[cfg_attr(any(), java_method(name = "toUnsignedString", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toUnsignedString_i(i: i32) -> Result<String> {
        panic!("stub: java/lang/Integer.toUnsignedString:(I)Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "getChars", descriptor = "(II[B)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
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

    #[cfg_attr(any(), java_method(name = "stringSize", descriptor = "(I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
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

    #[cfg_attr(any(), java_method(name = "parseInt", descriptor = "(Ljava/lang/String;I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException"))]
    pub fn parseInt_str_i(s: String, radix: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.parseInt:(Ljava/lang/String;I)I")
    }

    #[cfg_attr(any(), java_method(name = "parseInt", descriptor = "(Ljava/lang/CharSequence;III)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException"))]
    pub fn parseInt_seq_i_i_i(s: Object, beginIndex: i32, endIndex: i32, radix: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.parseInt:(Ljava/lang/CharSequence;III)I")
    }

    #[cfg_attr(any(), java_method(name = "parseInt", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException"))]
    pub fn parseInt_str(s: String) -> Result<i32> {
        panic!("stub: java/lang/Integer.parseInt:(Ljava/lang/String;)I")
    }

    #[cfg_attr(any(), java_method(name = "parseUnsignedInt", descriptor = "(Ljava/lang/String;I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException"))]
    pub fn parseUnsignedInt_str_i(s: String, radix: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.parseUnsignedInt:(Ljava/lang/String;I)I")
    }

    #[cfg_attr(any(), java_method(name = "parseUnsignedInt", descriptor = "(Ljava/lang/CharSequence;III)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException"))]
    pub fn parseUnsignedInt_seq_i_i_i(s: Object, beginIndex: i32, endIndex: i32, radix: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.parseUnsignedInt:(Ljava/lang/CharSequence;III)I")
    }

    #[cfg_attr(any(), java_method(name = "parseUnsignedInt", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException"))]
    pub fn parseUnsignedInt_str(s: String) -> Result<i32> {
        panic!("stub: java/lang/Integer.parseUnsignedInt:(Ljava/lang/String;)I")
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(Ljava/lang/String;I)Ljava/lang/Integer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException"))]
    pub fn valueOf_str_i(s: String, radix: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.valueOf:(Ljava/lang/String;I)Ljava/lang/Integer;")
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/lang/Integer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException"))]
    pub fn valueOf_str(s: String) -> Result<i32> {
        panic!("stub: java/lang/Integer.valueOf:(Ljava/lang/String;)Ljava/lang/Integer;")
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(I)Ljava/lang/Integer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn valueOf_i(i: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.valueOf:(I)Ljava/lang/Integer;")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true))]
    pub fn new_i(value: i32) -> Result<Self> {
        panic!("stub: java/lang/Integer.<init>:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException", is_deprecated = true))]
    pub fn new_str(s: String) -> Result<Self> {
        panic!("stub: java/lang/Integer.<init>:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "byteValue", descriptor = "()B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn byteValue(&self) -> Result<i8> {
        panic!("stub: java/lang/Integer.byteValue:()B")
    }

    #[cfg_attr(any(), java_method(name = "shortValue", descriptor = "()S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn shortValue(&self) -> Result<i16> {
        panic!("stub: java/lang/Integer.shortValue:()S")
    }

    #[cfg_attr(any(), java_method(name = "intValue", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn intValue(&self) -> Result<i32> {
        panic!("stub: java/lang/Integer.intValue:()I")
    }

    #[cfg_attr(any(), java_method(name = "longValue", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn longValue(&self) -> Result<i64> {
        panic!("stub: java/lang/Integer.longValue:()J")
    }

    #[cfg_attr(any(), java_method(name = "floatValue", descriptor = "()F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn floatValue(&self) -> Result<f32> {
        panic!("stub: java/lang/Integer.floatValue:()F")
    }

    #[cfg_attr(any(), java_method(name = "doubleValue", descriptor = "()D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn doubleValue(&self) -> Result<f64> {
        panic!("stub: java/lang/Integer.doubleValue:()D")
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/lang/Integer.toString:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/lang/Integer.hashCode:()I")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hashCode_i(value: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.hashCode:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn equals(&self, obj: Object) -> Result<bool> {
        panic!("stub: java/lang/Integer.equals:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "getInteger", descriptor = "(Ljava/lang/String;)Ljava/lang/Integer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getInteger_str(nm: String) -> Result<i32> {
        panic!("stub: java/lang/Integer.getInteger:(Ljava/lang/String;)Ljava/lang/Integer;")
    }

    #[cfg_attr(any(), java_method(name = "getInteger", descriptor = "(Ljava/lang/String;I)Ljava/lang/Integer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getInteger_str_i(nm: String, val: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.getInteger:(Ljava/lang/String;I)Ljava/lang/Integer;")
    }

    #[cfg_attr(any(), java_method(name = "getInteger", descriptor = "(Ljava/lang/String;Ljava/lang/Integer;)Ljava/lang/Integer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getInteger_str_int(nm: String, val: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.getInteger:(Ljava/lang/String;Ljava/lang/Integer;)Ljava/lang/Integer;")
    }

    #[cfg_attr(any(), java_method(name = "decode", descriptor = "(Ljava/lang/String;)Ljava/lang/Integer;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException"))]
    pub fn decode(nm: String) -> Result<i32> {
        panic!("stub: java/lang/Integer.decode:(Ljava/lang/String;)Ljava/lang/Integer;")
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "(Ljava/lang/Integer;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compareTo(&self, anotherInteger: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.compareTo:(Ljava/lang/Integer;)I")
    }

    #[cfg_attr(any(), java_method(name = "compare", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compare(x: i32, y: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.compare:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "compareUnsigned", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compareUnsigned(x: i32, y: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.compareUnsigned:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "toUnsignedLong", descriptor = "(I)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toUnsignedLong(x: i32) -> Result<i64> {
        panic!("stub: java/lang/Integer.toUnsignedLong:(I)J")
    }

    #[cfg_attr(any(), java_method(name = "divideUnsigned", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn divideUnsigned(dividend: i32, divisor: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.divideUnsigned:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "remainderUnsigned", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn remainderUnsigned(dividend: i32, divisor: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.remainderUnsigned:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "highestOneBit", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn highestOneBit(i: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.highestOneBit:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "lowestOneBit", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn lowestOneBit(i: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.lowestOneBit:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "numberOfLeadingZeros", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn numberOfLeadingZeros(i: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.numberOfLeadingZeros:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "numberOfTrailingZeros", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn numberOfTrailingZeros(i: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.numberOfTrailingZeros:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "bitCount", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn bitCount(i: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.bitCount:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "rotateLeft", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn rotateLeft(i: i32, distance: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.rotateLeft:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "rotateRight", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn rotateRight(i: i32, distance: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.rotateRight:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "reverse", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn reverse(i: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.reverse:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "compress", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compress(i: i32, mask: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.compress:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "expand", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn expand(i: i32, mask: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.expand:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "parallelSuffix", descriptor = "(I)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn parallelSuffix(maskCount: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.parallelSuffix:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "signum", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn signum(i: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.signum:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "reverseBytes", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn reverseBytes(i: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.reverseBytes:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "sum", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn sum(a: i32, b: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.sum:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "max", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn max(a: i32, b: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.max:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "min", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn min(a: i32, b: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.min:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "describeConstable", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Optional<Ljava/lang/Integer;>;"))]
    pub fn describeConstable(&self) -> Result<Object> {
        panic!("stub: java/lang/Integer.describeConstable:()Ljava/util/Optional;")
    }

    #[cfg_attr(any(), java_method(name = "resolveConstantDesc", descriptor = "(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Integer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn resolveConstantDesc(&self, lookup: Object) -> Result<i32> {
        panic!("stub: java/lang/Integer.resolveConstantDesc:(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Integer;")
    }
}
