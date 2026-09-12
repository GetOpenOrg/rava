#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Long",
    super_class = "java/lang/Number",
    interfaces  = "java/lang/Comparable,java/lang/constant/Constable,java/lang/constant/ConstantDesc",
    access      = "public final",
    source      = "Long.java",
))]
pub struct Long {
    #[cfg_attr(any(), java_field(name = "value", descriptor = "J", access = "private final"))]
    pub value: Field<i64>,
}

impl Long {
    // java: toString(JI)Ljava/lang/String;
    pub fn toString__l_i(i: i64, arg1: i32) -> Result<String> {
        todo!("abstract java/lang/Long.toString")
    }

    // java: toStringUTF16(JI)Ljava/lang/String;
    pub fn toStringUTF16(i: i64, arg1: i32) -> Result<String> {
        todo!("abstract java/lang/Long.toStringUTF16")
    }

    // java: toUnsignedString(JI)Ljava/lang/String;
    pub fn toUnsignedString__l_i(i: i64, arg1: i32) -> Result<String> {
        todo!("abstract java/lang/Long.toUnsignedString")
    }

    // java: toUnsignedBigInteger(J)Ljava/math/BigInteger;
    pub fn toUnsignedBigInteger(i: i64) -> Result<Object> {
        todo!("abstract java/lang/Long.toUnsignedBigInteger")
    }

    // java: toHexString(J)Ljava/lang/String;
    pub fn toHexString(i: i64) -> Result<String> {
        todo!("abstract java/lang/Long.toHexString")
    }

    // java: toOctalString(J)Ljava/lang/String;
    pub fn toOctalString(i: i64) -> Result<String> {
        todo!("abstract java/lang/Long.toOctalString")
    }

    // java: toBinaryString(J)Ljava/lang/String;
    pub fn toBinaryString(i: i64) -> Result<String> {
        todo!("abstract java/lang/Long.toBinaryString")
    }

    // java: toUnsignedString0(JI)Ljava/lang/String;
    pub fn toUnsignedString0(val: i64, arg1: i32) -> Result<String> {
        todo!("abstract java/lang/Long.toUnsignedString0")
    }

    // java: formatUnsignedLong0(JI[BII)V
    pub fn formatUnsignedLong0(val: i64, arg1: i32, shift: Vec<i8>, buf: i32, offset: i32) -> Result<()> {
        todo!("abstract java/lang/Long.formatUnsignedLong0")
    }

    // java: formatUnsignedLong0UTF16(JI[BII)V
    pub fn formatUnsignedLong0UTF16(val: i64, arg1: i32, shift: Vec<i8>, buf: i32, offset: i32) -> Result<()> {
        todo!("abstract java/lang/Long.formatUnsignedLong0UTF16")
    }

    // java: fastUUID(JJ)Ljava/lang/String;
    pub fn fastUUID(lsb: i64, arg1: i64) -> Result<String> {
        todo!("abstract java/lang/Long.fastUUID")
    }

    // java: toString(J)Ljava/lang/String;
    pub fn toString__l(i: i64) -> Result<String> {
        todo!("abstract java/lang/Long.toString")
    }

    // java: toUnsignedString(J)Ljava/lang/String;
    pub fn toUnsignedString__l(i: i64) -> Result<String> {
        todo!("abstract java/lang/Long.toUnsignedString")
    }

    // java: getChars(JI[B)I
    pub fn getChars(i: i64, arg1: i32, index: Vec<i8>) -> Result<i32> {
        todo!("abstract java/lang/Long.getChars")
    }

    // java: stringSize(J)I
    pub fn stringSize(x: i64) -> Result<i32> {
        todo!("abstract java/lang/Long.stringSize")
    }

    // java: parseLong(Ljava/lang/String;I)J
    pub fn parseLong__str_i(s: String, radix: i32) -> Result<i64> {
        todo!("abstract java/lang/Long.parseLong")
    }

    // java: parseLong(Ljava/lang/CharSequence;III)J
    pub fn parseLong__seq_i_i_i(s: Object, beginIndex: i32, endIndex: i32, radix: i32) -> Result<i64> {
        todo!("abstract java/lang/Long.parseLong")
    }

    // java: parseLong(Ljava/lang/String;)J
    pub fn parseLong__str(s: String) -> Result<i64> {
        todo!("abstract java/lang/Long.parseLong")
    }

    // java: parseUnsignedLong(Ljava/lang/String;I)J
    pub fn parseUnsignedLong__str_i(s: String, radix: i32) -> Result<i64> {
        todo!("abstract java/lang/Long.parseUnsignedLong")
    }

    // java: parseUnsignedLong(Ljava/lang/CharSequence;III)J
    pub fn parseUnsignedLong__seq_i_i_i(s: Object, beginIndex: i32, endIndex: i32, radix: i32) -> Result<i64> {
        todo!("abstract java/lang/Long.parseUnsignedLong")
    }

    // java: parseUnsignedLong(Ljava/lang/String;)J
    pub fn parseUnsignedLong__str(s: String) -> Result<i64> {
        todo!("abstract java/lang/Long.parseUnsignedLong")
    }

    // java: valueOf(Ljava/lang/String;I)Ljava/lang/Long;
    pub fn valueOf__str_i(s: String, radix: i32) -> Result<i64> {
        todo!("abstract java/lang/Long.valueOf")
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Long;
    pub fn valueOf__str(s: String) -> Result<i64> {
        todo!("abstract java/lang/Long.valueOf")
    }

    // java: valueOf(J)Ljava/lang/Long;
    pub fn valueOf__l(l: i64) -> Result<i64> {
        todo!("abstract java/lang/Long.valueOf")
    }

    // java: decode(Ljava/lang/String;)Ljava/lang/Long;
    pub fn decode(nm: String) -> Result<i64> {
        todo!("abstract java/lang/Long.decode")
    }

    // java: <init>(J)V
    pub fn new__l(&self, value: i64) -> Result<()> {
        todo!("abstract java/lang/Long.<init>")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, s: String) -> Result<()> {
        todo!("abstract java/lang/Long.<init>")
    }

    // java: byteValue()B
    pub fn byteValue(&self) -> Result<i8> {
        todo!("abstract java/lang/Long.byteValue")
    }

    // java: shortValue()S
    pub fn shortValue(&self) -> Result<i16> {
        todo!("abstract java/lang/Long.shortValue")
    }

    // java: intValue()I
    pub fn intValue(&self) -> Result<i32> {
        todo!("abstract java/lang/Long.intValue")
    }

    // java: longValue()J
    pub fn longValue(&self) -> Result<i64> {
        todo!("abstract java/lang/Long.longValue")
    }

    // java: floatValue()F
    pub fn floatValue(&self) -> Result<f32> {
        todo!("abstract java/lang/Long.floatValue")
    }

    // java: doubleValue()D
    pub fn doubleValue(&self) -> Result<f64> {
        todo!("abstract java/lang/Long.doubleValue")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/lang/Long.toString")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/lang/Long.hashCode")
    }

    // java: hashCode(J)I
    pub fn hashCode__l(value: i64) -> Result<i32> {
        todo!("abstract java/lang/Long.hashCode")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        todo!("abstract java/lang/Long.equals")
    }

    // java: getLong(Ljava/lang/String;)Ljava/lang/Long;
    pub fn getLong__str(nm: String) -> Result<i64> {
        todo!("abstract java/lang/Long.getLong")
    }

    // java: getLong(Ljava/lang/String;J)Ljava/lang/Long;
    pub fn getLong__str_l(nm: String, val: i64) -> Result<i64> {
        todo!("abstract java/lang/Long.getLong")
    }

    // java: getLong(Ljava/lang/String;Ljava/lang/Long;)Ljava/lang/Long;
    pub fn getLong__str_lng(nm: String, val: i64) -> Result<i64> {
        todo!("abstract java/lang/Long.getLong")
    }

    // java: compareTo(Ljava/lang/Long;)I
    pub fn compareTo(&self, anotherLong: i64) -> Result<i32> {
        todo!("abstract java/lang/Long.compareTo")
    }

    // java: compare(JJ)I
    pub fn compare(x: i64, arg1: i64) -> Result<i32> {
        todo!("abstract java/lang/Long.compare")
    }

    // java: compareUnsigned(JJ)I
    pub fn compareUnsigned(x: i64, arg1: i64) -> Result<i32> {
        todo!("abstract java/lang/Long.compareUnsigned")
    }

    // java: divideUnsigned(JJ)J
    pub fn divideUnsigned(dividend: i64, arg1: i64) -> Result<i64> {
        todo!("abstract java/lang/Long.divideUnsigned")
    }

    // java: remainderUnsigned(JJ)J
    pub fn remainderUnsigned(dividend: i64, arg1: i64) -> Result<i64> {
        todo!("abstract java/lang/Long.remainderUnsigned")
    }

    // java: highestOneBit(J)J
    pub fn highestOneBit(i: i64) -> Result<i64> {
        todo!("abstract java/lang/Long.highestOneBit")
    }

    // java: lowestOneBit(J)J
    pub fn lowestOneBit(i: i64) -> Result<i64> {
        todo!("abstract java/lang/Long.lowestOneBit")
    }

    // java: numberOfLeadingZeros(J)I
    pub fn numberOfLeadingZeros(i: i64) -> Result<i32> {
        todo!("abstract java/lang/Long.numberOfLeadingZeros")
    }

    // java: numberOfTrailingZeros(J)I
    pub fn numberOfTrailingZeros(i: i64) -> Result<i32> {
        todo!("abstract java/lang/Long.numberOfTrailingZeros")
    }

    // java: bitCount(J)I
    pub fn bitCount(i: i64) -> Result<i32> {
        todo!("abstract java/lang/Long.bitCount")
    }

    // java: rotateLeft(JI)J
    pub fn rotateLeft(i: i64, arg1: i32) -> Result<i64> {
        todo!("abstract java/lang/Long.rotateLeft")
    }

    // java: rotateRight(JI)J
    pub fn rotateRight(i: i64, arg1: i32) -> Result<i64> {
        todo!("abstract java/lang/Long.rotateRight")
    }

    // java: reverse(J)J
    pub fn reverse(i: i64) -> Result<i64> {
        todo!("abstract java/lang/Long.reverse")
    }

    // java: compress(JJ)J
    pub fn compress(i: i64, arg1: i64) -> Result<i64> {
        todo!("abstract java/lang/Long.compress")
    }

    // java: expand(JJ)J
    pub fn expand(i: i64, arg1: i64) -> Result<i64> {
        todo!("abstract java/lang/Long.expand")
    }

    // java: parallelSuffix(J)J
    pub fn parallelSuffix(maskCount: i64) -> Result<i64> {
        todo!("abstract java/lang/Long.parallelSuffix")
    }

    // java: signum(J)I
    pub fn signum(i: i64) -> Result<i32> {
        todo!("abstract java/lang/Long.signum")
    }

    // java: reverseBytes(J)J
    pub fn reverseBytes(i: i64) -> Result<i64> {
        todo!("abstract java/lang/Long.reverseBytes")
    }

    // java: sum(JJ)J
    pub fn sum(a: i64, arg1: i64) -> Result<i64> {
        todo!("abstract java/lang/Long.sum")
    }

    // java: max(JJ)J
    pub fn max(a: i64, arg1: i64) -> Result<i64> {
        todo!("abstract java/lang/Long.max")
    }

    // java: min(JJ)J
    pub fn min(a: i64, arg1: i64) -> Result<i64> {
        todo!("abstract java/lang/Long.min")
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        todo!("abstract java/lang/Long.describeConstable")
    }

    // java: resolveConstantDesc(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Long;
    pub fn resolveConstantDesc(&self, lookup: Object) -> Result<i64> {
        todo!("abstract java/lang/Long.resolveConstantDesc")
    }
}
