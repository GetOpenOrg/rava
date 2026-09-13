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
        panic!("stub: java/lang/Integer.toString:(II)Ljava/lang/String;")
    }

    // java: toStringUTF16(II)Ljava/lang/String;
    pub fn toStringUTF16(i: i32, radix: i32) -> Result<String> {
        panic!("stub: java/lang/Integer.toStringUTF16:(II)Ljava/lang/String;")
    }

    // java: toUnsignedString(II)Ljava/lang/String;
    pub fn toUnsignedString__i_i(i: i32, radix: i32) -> Result<String> {
        panic!("stub: java/lang/Integer.toUnsignedString:(II)Ljava/lang/String;")
    }

    // java: toHexString(I)Ljava/lang/String;
    pub fn toHexString(i: i32) -> Result<String> {
        panic!("stub: java/lang/Integer.toHexString:(I)Ljava/lang/String;")
    }

    // java: toOctalString(I)Ljava/lang/String;
    pub fn toOctalString(i: i32) -> Result<String> {
        panic!("stub: java/lang/Integer.toOctalString:(I)Ljava/lang/String;")
    }

    // java: toBinaryString(I)Ljava/lang/String;
    pub fn toBinaryString(i: i32) -> Result<String> {
        panic!("stub: java/lang/Integer.toBinaryString:(I)Ljava/lang/String;")
    }

    // java: toUnsignedString0(II)Ljava/lang/String;
    pub fn toUnsignedString0(val: i32, shift: i32) -> Result<String> {
        panic!("stub: java/lang/Integer.toUnsignedString0:(II)Ljava/lang/String;")
    }

    // java: formatUnsignedInt(II[BI)V
    pub fn formatUnsignedInt(val: i32, shift: i32, buf: Vec<i8>, len: i32) -> Result<()> {
        panic!("stub: java/lang/Integer.formatUnsignedInt:(II[BI)V")
    }

    // java: formatUnsignedIntUTF16(II[BI)V
    pub fn formatUnsignedIntUTF16(val: i32, shift: i32, buf: Vec<i8>, len: i32) -> Result<()> {
        panic!("stub: java/lang/Integer.formatUnsignedIntUTF16:(II[BI)V")
    }

    // java: toString(I)Ljava/lang/String;
    pub fn toString__i(i: i32) -> Result<String> {
        panic!("stub: java/lang/Integer.toString:(I)Ljava/lang/String;")
    }

    // java: toUnsignedString(I)Ljava/lang/String;
    pub fn toUnsignedString__i(i: i32) -> Result<String> {
        panic!("stub: java/lang/Integer.toUnsignedString:(I)Ljava/lang/String;")
    }

    // java: getChars(II[B)I
    pub fn getChars(i: i32, index: i32, buf: Vec<i8>) -> Result<i32> {
        panic!("stub: java/lang/Integer.getChars:(II[B)I")
    }

    // java: stringSize(I)I
    pub fn stringSize(x: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.stringSize:(I)I")
    }

    // java: parseInt(Ljava/lang/String;I)I
    pub fn parseInt__str_i(s: String, radix: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.parseInt:(Ljava/lang/String;I)I")
    }

    // java: parseInt(Ljava/lang/CharSequence;III)I
    pub fn parseInt__seq_i_i_i(s: Object, beginIndex: i32, endIndex: i32, radix: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.parseInt:(Ljava/lang/CharSequence;III)I")
    }

    // java: parseInt(Ljava/lang/String;)I
    pub fn parseInt__str(s: String) -> Result<i32> {
        panic!("stub: java/lang/Integer.parseInt:(Ljava/lang/String;)I")
    }

    // java: parseUnsignedInt(Ljava/lang/String;I)I
    pub fn parseUnsignedInt__str_i(s: String, radix: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.parseUnsignedInt:(Ljava/lang/String;I)I")
    }

    // java: parseUnsignedInt(Ljava/lang/CharSequence;III)I
    pub fn parseUnsignedInt__seq_i_i_i(s: Object, beginIndex: i32, endIndex: i32, radix: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.parseUnsignedInt:(Ljava/lang/CharSequence;III)I")
    }

    // java: parseUnsignedInt(Ljava/lang/String;)I
    pub fn parseUnsignedInt__str(s: String) -> Result<i32> {
        panic!("stub: java/lang/Integer.parseUnsignedInt:(Ljava/lang/String;)I")
    }

    // java: valueOf(Ljava/lang/String;I)Ljava/lang/Integer;
    pub fn valueOf__str_i(s: String, radix: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.valueOf:(Ljava/lang/String;I)Ljava/lang/Integer;")
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Integer;
    pub fn valueOf__str(s: String) -> Result<i32> {
        panic!("stub: java/lang/Integer.valueOf:(Ljava/lang/String;)Ljava/lang/Integer;")
    }

    // java: valueOf(I)Ljava/lang/Integer;
    pub fn valueOf__i(i: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.valueOf:(I)Ljava/lang/Integer;")
    }

    // java: <init>(I)V
    pub fn new__i(&self, value: i32) -> Result<()> {
        panic!("stub: java/lang/Integer.<init>:(I)V")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, s: String) -> Result<()> {
        panic!("stub: java/lang/Integer.<init>:(Ljava/lang/String;)V")
    }

    // java: byteValue()B
    pub fn byteValue(&self) -> Result<i8> {
        panic!("stub: java/lang/Integer.byteValue:()B")
    }

    // java: shortValue()S
    pub fn shortValue(&self) -> Result<i16> {
        panic!("stub: java/lang/Integer.shortValue:()S")
    }

    // java: intValue()I
    pub fn intValue(&self) -> Result<i32> {
        panic!("stub: java/lang/Integer.intValue:()I")
    }

    // java: longValue()J
    pub fn longValue(&self) -> Result<i64> {
        panic!("stub: java/lang/Integer.longValue:()J")
    }

    // java: floatValue()F
    pub fn floatValue(&self) -> Result<f32> {
        panic!("stub: java/lang/Integer.floatValue:()F")
    }

    // java: doubleValue()D
    pub fn doubleValue(&self) -> Result<f64> {
        panic!("stub: java/lang/Integer.doubleValue:()D")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/lang/Integer.toString:()Ljava/lang/String;")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/lang/Integer.hashCode:()I")
    }

    // java: hashCode(I)I
    pub fn hashCode__i(value: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.hashCode:(I)I")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        panic!("stub: java/lang/Integer.equals:(Ljava/lang/Object;)Z")
    }

    // java: getInteger(Ljava/lang/String;)Ljava/lang/Integer;
    pub fn getInteger__str(nm: String) -> Result<i32> {
        panic!("stub: java/lang/Integer.getInteger:(Ljava/lang/String;)Ljava/lang/Integer;")
    }

    // java: getInteger(Ljava/lang/String;I)Ljava/lang/Integer;
    pub fn getInteger__str_i(nm: String, val: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.getInteger:(Ljava/lang/String;I)Ljava/lang/Integer;")
    }

    // java: getInteger(Ljava/lang/String;Ljava/lang/Integer;)Ljava/lang/Integer;
    pub fn getInteger__str_int(nm: String, val: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.getInteger:(Ljava/lang/String;Ljava/lang/Integer;)Ljava/lang/Integer;")
    }

    // java: decode(Ljava/lang/String;)Ljava/lang/Integer;
    pub fn decode(nm: String) -> Result<i32> {
        panic!("stub: java/lang/Integer.decode:(Ljava/lang/String;)Ljava/lang/Integer;")
    }

    // java: compareTo(Ljava/lang/Integer;)I
    pub fn compareTo(&self, anotherInteger: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.compareTo:(Ljava/lang/Integer;)I")
    }

    // java: compare(II)I
    pub fn compare(x: i32, y: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.compare:(II)I")
    }

    // java: compareUnsigned(II)I
    pub fn compareUnsigned(x: i32, y: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.compareUnsigned:(II)I")
    }

    // java: toUnsignedLong(I)J
    pub fn toUnsignedLong(x: i32) -> Result<i64> {
        panic!("stub: java/lang/Integer.toUnsignedLong:(I)J")
    }

    // java: divideUnsigned(II)I
    pub fn divideUnsigned(dividend: i32, divisor: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.divideUnsigned:(II)I")
    }

    // java: remainderUnsigned(II)I
    pub fn remainderUnsigned(dividend: i32, divisor: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.remainderUnsigned:(II)I")
    }

    // java: highestOneBit(I)I
    pub fn highestOneBit(i: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.highestOneBit:(I)I")
    }

    // java: lowestOneBit(I)I
    pub fn lowestOneBit(i: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.lowestOneBit:(I)I")
    }

    // java: numberOfLeadingZeros(I)I
    pub fn numberOfLeadingZeros(i: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.numberOfLeadingZeros:(I)I")
    }

    // java: numberOfTrailingZeros(I)I
    pub fn numberOfTrailingZeros(i: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.numberOfTrailingZeros:(I)I")
    }

    // java: bitCount(I)I
    pub fn bitCount(i: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.bitCount:(I)I")
    }

    // java: rotateLeft(II)I
    pub fn rotateLeft(i: i32, distance: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.rotateLeft:(II)I")
    }

    // java: rotateRight(II)I
    pub fn rotateRight(i: i32, distance: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.rotateRight:(II)I")
    }

    // java: reverse(I)I
    pub fn reverse(i: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.reverse:(I)I")
    }

    // java: compress(II)I
    pub fn compress(i: i32, mask: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.compress:(II)I")
    }

    // java: expand(II)I
    pub fn expand(i: i32, mask: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.expand:(II)I")
    }

    // java: parallelSuffix(I)I
    pub fn parallelSuffix(maskCount: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.parallelSuffix:(I)I")
    }

    // java: signum(I)I
    pub fn signum(i: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.signum:(I)I")
    }

    // java: reverseBytes(I)I
    pub fn reverseBytes(i: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.reverseBytes:(I)I")
    }

    // java: sum(II)I
    pub fn sum(a: i32, b: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.sum:(II)I")
    }

    // java: max(II)I
    pub fn max(a: i32, b: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.max:(II)I")
    }

    // java: min(II)I
    pub fn min(a: i32, b: i32) -> Result<i32> {
        panic!("stub: java/lang/Integer.min:(II)I")
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        panic!("stub: java/lang/Integer.describeConstable:()Ljava/util/Optional;")
    }

    // java: resolveConstantDesc(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Integer;
    pub fn resolveConstantDesc(&self, lookup: Object) -> Result<i32> {
        panic!("stub: java/lang/Integer.resolveConstantDesc:(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Integer;")
    }
}
