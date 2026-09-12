#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Integer",
    super_class = "java/lang/Number",
    interfaces  = "java/lang/Comparable,java/lang/constant/Constable,java/lang/constant/ConstantDesc",
    access      = "public final",
    source      = "Integer.java",
))]
pub struct Integer {
    #[cfg_attr(any(), java_field(name = "value", descriptor = "I", access = "private final"))]
    pub value: Field<i32>,
}

impl Integer {
    // java: toString(II)Ljava/lang/String;
    pub fn toString__i_i(i: i32, radix: i32) -> Result<String> {
        todo!("abstract java/lang/Integer.toString")
    }

    // java: toStringUTF16(II)Ljava/lang/String;
    pub fn toStringUTF16(i: i32, radix: i32) -> Result<String> {
        todo!("abstract java/lang/Integer.toStringUTF16")
    }

    // java: toUnsignedString(II)Ljava/lang/String;
    pub fn toUnsignedString__i_i(i: i32, radix: i32) -> Result<String> {
        todo!("abstract java/lang/Integer.toUnsignedString")
    }

    // java: toHexString(I)Ljava/lang/String;
    pub fn toHexString(i: i32) -> Result<String> {
        todo!("abstract java/lang/Integer.toHexString")
    }

    // java: toOctalString(I)Ljava/lang/String;
    pub fn toOctalString(i: i32) -> Result<String> {
        todo!("abstract java/lang/Integer.toOctalString")
    }

    // java: toBinaryString(I)Ljava/lang/String;
    pub fn toBinaryString(i: i32) -> Result<String> {
        todo!("abstract java/lang/Integer.toBinaryString")
    }

    // java: toUnsignedString0(II)Ljava/lang/String;
    pub fn toUnsignedString0(val: i32, shift: i32) -> Result<String> {
        todo!("abstract java/lang/Integer.toUnsignedString0")
    }

    // java: formatUnsignedInt(II[BI)V
    pub fn formatUnsignedInt(val: i32, shift: i32, buf: Vec<i8>, len: i32) -> Result<()> {
        todo!("abstract java/lang/Integer.formatUnsignedInt")
    }

    // java: formatUnsignedIntUTF16(II[BI)V
    pub fn formatUnsignedIntUTF16(val: i32, shift: i32, buf: Vec<i8>, len: i32) -> Result<()> {
        todo!("abstract java/lang/Integer.formatUnsignedIntUTF16")
    }

    // java: toString(I)Ljava/lang/String;
    pub fn toString__i(i: i32) -> Result<String> {
        todo!("abstract java/lang/Integer.toString")
    }

    // java: toUnsignedString(I)Ljava/lang/String;
    pub fn toUnsignedString__i(i: i32) -> Result<String> {
        todo!("abstract java/lang/Integer.toUnsignedString")
    }

    // java: getChars(II[B)I
    pub fn getChars(i: i32, index: i32, buf: Vec<i8>) -> Result<i32> {
        todo!("abstract java/lang/Integer.getChars")
    }

    // java: stringSize(I)I
    pub fn stringSize(x: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.stringSize")
    }

    // java: parseInt(Ljava/lang/String;I)I
    pub fn parseInt__str_i(s: String, radix: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.parseInt")
    }

    // java: parseInt(Ljava/lang/CharSequence;III)I
    pub fn parseInt__seq_i_i_i(s: Object, beginIndex: i32, endIndex: i32, radix: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.parseInt")
    }

    // java: parseInt(Ljava/lang/String;)I
    pub fn parseInt__str(s: String) -> Result<i32> {
        todo!("abstract java/lang/Integer.parseInt")
    }

    // java: parseUnsignedInt(Ljava/lang/String;I)I
    pub fn parseUnsignedInt__str_i(s: String, radix: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.parseUnsignedInt")
    }

    // java: parseUnsignedInt(Ljava/lang/CharSequence;III)I
    pub fn parseUnsignedInt__seq_i_i_i(s: Object, beginIndex: i32, endIndex: i32, radix: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.parseUnsignedInt")
    }

    // java: parseUnsignedInt(Ljava/lang/String;)I
    pub fn parseUnsignedInt__str(s: String) -> Result<i32> {
        todo!("abstract java/lang/Integer.parseUnsignedInt")
    }

    // java: valueOf(Ljava/lang/String;I)Ljava/lang/Integer;
    pub fn valueOf__str_i(s: String, radix: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.valueOf")
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Integer;
    pub fn valueOf__str(s: String) -> Result<i32> {
        todo!("abstract java/lang/Integer.valueOf")
    }

    // java: valueOf(I)Ljava/lang/Integer;
    pub fn valueOf__i(i: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.valueOf")
    }

    // java: <init>(I)V
    pub fn new__i(&self, value: i32) -> Result<()> {
        todo!("abstract java/lang/Integer.<init>")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, s: String) -> Result<()> {
        todo!("abstract java/lang/Integer.<init>")
    }

    // java: byteValue()B
    pub fn byteValue(&self) -> Result<i8> {
        todo!("abstract java/lang/Integer.byteValue")
    }

    // java: shortValue()S
    pub fn shortValue(&self) -> Result<i16> {
        todo!("abstract java/lang/Integer.shortValue")
    }

    // java: intValue()I
    pub fn intValue(&self) -> Result<i32> {
        todo!("abstract java/lang/Integer.intValue")
    }

    // java: longValue()J
    pub fn longValue(&self) -> Result<i64> {
        todo!("abstract java/lang/Integer.longValue")
    }

    // java: floatValue()F
    pub fn floatValue(&self) -> Result<f32> {
        todo!("abstract java/lang/Integer.floatValue")
    }

    // java: doubleValue()D
    pub fn doubleValue(&self) -> Result<f64> {
        todo!("abstract java/lang/Integer.doubleValue")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/lang/Integer.toString")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/lang/Integer.hashCode")
    }

    // java: hashCode(I)I
    pub fn hashCode__i(value: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.hashCode")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        todo!("abstract java/lang/Integer.equals")
    }

    // java: getInteger(Ljava/lang/String;)Ljava/lang/Integer;
    pub fn getInteger__str(nm: String) -> Result<i32> {
        todo!("abstract java/lang/Integer.getInteger")
    }

    // java: getInteger(Ljava/lang/String;I)Ljava/lang/Integer;
    pub fn getInteger__str_i(nm: String, val: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.getInteger")
    }

    // java: getInteger(Ljava/lang/String;Ljava/lang/Integer;)Ljava/lang/Integer;
    pub fn getInteger__str_int(nm: String, val: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.getInteger")
    }

    // java: decode(Ljava/lang/String;)Ljava/lang/Integer;
    pub fn decode(nm: String) -> Result<i32> {
        todo!("abstract java/lang/Integer.decode")
    }

    // java: compareTo(Ljava/lang/Integer;)I
    pub fn compareTo(&self, anotherInteger: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.compareTo")
    }

    // java: compare(II)I
    pub fn compare(x: i32, y: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.compare")
    }

    // java: compareUnsigned(II)I
    pub fn compareUnsigned(x: i32, y: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.compareUnsigned")
    }

    // java: toUnsignedLong(I)J
    pub fn toUnsignedLong(x: i32) -> Result<i64> {
        todo!("abstract java/lang/Integer.toUnsignedLong")
    }

    // java: divideUnsigned(II)I
    pub fn divideUnsigned(dividend: i32, divisor: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.divideUnsigned")
    }

    // java: remainderUnsigned(II)I
    pub fn remainderUnsigned(dividend: i32, divisor: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.remainderUnsigned")
    }

    // java: highestOneBit(I)I
    pub fn highestOneBit(i: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.highestOneBit")
    }

    // java: lowestOneBit(I)I
    pub fn lowestOneBit(i: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.lowestOneBit")
    }

    // java: numberOfLeadingZeros(I)I
    pub fn numberOfLeadingZeros(i: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.numberOfLeadingZeros")
    }

    // java: numberOfTrailingZeros(I)I
    pub fn numberOfTrailingZeros(i: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.numberOfTrailingZeros")
    }

    // java: bitCount(I)I
    pub fn bitCount(i: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.bitCount")
    }

    // java: rotateLeft(II)I
    pub fn rotateLeft(i: i32, distance: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.rotateLeft")
    }

    // java: rotateRight(II)I
    pub fn rotateRight(i: i32, distance: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.rotateRight")
    }

    // java: reverse(I)I
    pub fn reverse(i: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.reverse")
    }

    // java: compress(II)I
    pub fn compress(i: i32, mask: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.compress")
    }

    // java: expand(II)I
    pub fn expand(i: i32, mask: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.expand")
    }

    // java: parallelSuffix(I)I
    pub fn parallelSuffix(maskCount: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.parallelSuffix")
    }

    // java: signum(I)I
    pub fn signum(i: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.signum")
    }

    // java: reverseBytes(I)I
    pub fn reverseBytes(i: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.reverseBytes")
    }

    // java: sum(II)I
    pub fn sum(a: i32, b: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.sum")
    }

    // java: max(II)I
    pub fn max(a: i32, b: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.max")
    }

    // java: min(II)I
    pub fn min(a: i32, b: i32) -> Result<i32> {
        todo!("abstract java/lang/Integer.min")
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        todo!("abstract java/lang/Integer.describeConstable")
    }

    // java: resolveConstantDesc(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Integer;
    pub fn resolveConstantDesc(&self, lookup: Object) -> Result<i32> {
        todo!("abstract java/lang/Integer.resolveConstantDesc")
    }
}
