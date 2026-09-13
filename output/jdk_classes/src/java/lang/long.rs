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
        panic!("stub: java/lang/Long.toString:(JI)Ljava/lang/String;")
    }

    // java: toStringUTF16(JI)Ljava/lang/String;
    pub fn toStringUTF16(i: i64, arg1: i32) -> Result<String> {
        panic!("stub: java/lang/Long.toStringUTF16:(JI)Ljava/lang/String;")
    }

    // java: toUnsignedString(JI)Ljava/lang/String;
    pub fn toUnsignedString__l_i(i: i64, arg1: i32) -> Result<String> {
        panic!("stub: java/lang/Long.toUnsignedString:(JI)Ljava/lang/String;")
    }

    // java: toUnsignedBigInteger(J)Ljava/math/BigInteger;
    pub fn toUnsignedBigInteger(i: i64) -> Result<Object> {
        panic!("stub: java/lang/Long.toUnsignedBigInteger:(J)Ljava/math/BigInteger;")
    }

    // java: toHexString(J)Ljava/lang/String;
    pub fn toHexString(i: i64) -> Result<String> {
        panic!("stub: java/lang/Long.toHexString:(J)Ljava/lang/String;")
    }

    // java: toOctalString(J)Ljava/lang/String;
    pub fn toOctalString(i: i64) -> Result<String> {
        panic!("stub: java/lang/Long.toOctalString:(J)Ljava/lang/String;")
    }

    // java: toBinaryString(J)Ljava/lang/String;
    pub fn toBinaryString(i: i64) -> Result<String> {
        panic!("stub: java/lang/Long.toBinaryString:(J)Ljava/lang/String;")
    }

    // java: toUnsignedString0(JI)Ljava/lang/String;
    pub fn toUnsignedString0(val: i64, arg1: i32) -> Result<String> {
        panic!("stub: java/lang/Long.toUnsignedString0:(JI)Ljava/lang/String;")
    }

    // java: formatUnsignedLong0(JI[BII)V
    pub fn formatUnsignedLong0(val: i64, arg1: i32, shift: Vec<i8>, buf: i32, offset: i32) -> Result<()> {
        panic!("stub: java/lang/Long.formatUnsignedLong0:(JI[BII)V")
    }

    // java: formatUnsignedLong0UTF16(JI[BII)V
    pub fn formatUnsignedLong0UTF16(val: i64, arg1: i32, shift: Vec<i8>, buf: i32, offset: i32) -> Result<()> {
        panic!("stub: java/lang/Long.formatUnsignedLong0UTF16:(JI[BII)V")
    }

    // java: fastUUID(JJ)Ljava/lang/String;
    pub fn fastUUID(lsb: i64, arg1: i64) -> Result<String> {
        panic!("stub: java/lang/Long.fastUUID:(JJ)Ljava/lang/String;")
    }

    // java: toString(J)Ljava/lang/String;
    pub fn toString__l(i: i64) -> Result<String> {
        panic!("stub: java/lang/Long.toString:(J)Ljava/lang/String;")
    }

    // java: toUnsignedString(J)Ljava/lang/String;
    pub fn toUnsignedString__l(i: i64) -> Result<String> {
        panic!("stub: java/lang/Long.toUnsignedString:(J)Ljava/lang/String;")
    }

    // java: getChars(JI[B)I
    pub fn getChars(i: i64, arg1: i32, index: Vec<i8>) -> Result<i32> {
        panic!("stub: java/lang/Long.getChars:(JI[B)I")
    }

    // java: stringSize(J)I
    pub fn stringSize(x: i64) -> Result<i32> {
        panic!("stub: java/lang/Long.stringSize:(J)I")
    }

    // java: parseLong(Ljava/lang/String;I)J
    pub fn parseLong__str_i(s: String, radix: i32) -> Result<i64> {
        panic!("stub: java/lang/Long.parseLong:(Ljava/lang/String;I)J")
    }

    // java: parseLong(Ljava/lang/CharSequence;III)J
    pub fn parseLong__seq_i_i_i(s: Object, beginIndex: i32, endIndex: i32, radix: i32) -> Result<i64> {
        panic!("stub: java/lang/Long.parseLong:(Ljava/lang/CharSequence;III)J")
    }

    // java: parseLong(Ljava/lang/String;)J
    pub fn parseLong__str(s: String) -> Result<i64> {
        panic!("stub: java/lang/Long.parseLong:(Ljava/lang/String;)J")
    }

    // java: parseUnsignedLong(Ljava/lang/String;I)J
    pub fn parseUnsignedLong__str_i(s: String, radix: i32) -> Result<i64> {
        panic!("stub: java/lang/Long.parseUnsignedLong:(Ljava/lang/String;I)J")
    }

    // java: parseUnsignedLong(Ljava/lang/CharSequence;III)J
    pub fn parseUnsignedLong__seq_i_i_i(s: Object, beginIndex: i32, endIndex: i32, radix: i32) -> Result<i64> {
        panic!("stub: java/lang/Long.parseUnsignedLong:(Ljava/lang/CharSequence;III)J")
    }

    // java: parseUnsignedLong(Ljava/lang/String;)J
    pub fn parseUnsignedLong__str(s: String) -> Result<i64> {
        panic!("stub: java/lang/Long.parseUnsignedLong:(Ljava/lang/String;)J")
    }

    // java: valueOf(Ljava/lang/String;I)Ljava/lang/Long;
    pub fn valueOf__str_i(s: String, radix: i32) -> Result<i64> {
        panic!("stub: java/lang/Long.valueOf:(Ljava/lang/String;I)Ljava/lang/Long;")
    }

    // java: valueOf(Ljava/lang/String;)Ljava/lang/Long;
    pub fn valueOf__str(s: String) -> Result<i64> {
        panic!("stub: java/lang/Long.valueOf:(Ljava/lang/String;)Ljava/lang/Long;")
    }

    // java: valueOf(J)Ljava/lang/Long;
    pub fn valueOf__l(l: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.valueOf:(J)Ljava/lang/Long;")
    }

    // java: decode(Ljava/lang/String;)Ljava/lang/Long;
    pub fn decode(nm: String) -> Result<i64> {
        panic!("stub: java/lang/Long.decode:(Ljava/lang/String;)Ljava/lang/Long;")
    }

    // java: <init>(J)V
    pub fn new__l(&self, value: i64) -> Result<()> {
        panic!("stub: java/lang/Long.<init>:(J)V")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, s: String) -> Result<()> {
        panic!("stub: java/lang/Long.<init>:(Ljava/lang/String;)V")
    }

    // java: byteValue()B
    pub fn byteValue(&self) -> Result<i8> {
        panic!("stub: java/lang/Long.byteValue:()B")
    }

    // java: shortValue()S
    pub fn shortValue(&self) -> Result<i16> {
        panic!("stub: java/lang/Long.shortValue:()S")
    }

    // java: intValue()I
    pub fn intValue(&self) -> Result<i32> {
        panic!("stub: java/lang/Long.intValue:()I")
    }

    // java: longValue()J
    pub fn longValue(&self) -> Result<i64> {
        panic!("stub: java/lang/Long.longValue:()J")
    }

    // java: floatValue()F
    pub fn floatValue(&self) -> Result<f32> {
        panic!("stub: java/lang/Long.floatValue:()F")
    }

    // java: doubleValue()D
    pub fn doubleValue(&self) -> Result<f64> {
        panic!("stub: java/lang/Long.doubleValue:()D")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/lang/Long.toString:()Ljava/lang/String;")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/lang/Long.hashCode:()I")
    }

    // java: hashCode(J)I
    pub fn hashCode__l(value: i64) -> Result<i32> {
        panic!("stub: java/lang/Long.hashCode:(J)I")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        panic!("stub: java/lang/Long.equals:(Ljava/lang/Object;)Z")
    }

    // java: getLong(Ljava/lang/String;)Ljava/lang/Long;
    pub fn getLong__str(nm: String) -> Result<i64> {
        panic!("stub: java/lang/Long.getLong:(Ljava/lang/String;)Ljava/lang/Long;")
    }

    // java: getLong(Ljava/lang/String;J)Ljava/lang/Long;
    pub fn getLong__str_l(nm: String, val: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.getLong:(Ljava/lang/String;J)Ljava/lang/Long;")
    }

    // java: getLong(Ljava/lang/String;Ljava/lang/Long;)Ljava/lang/Long;
    pub fn getLong__str_lng(nm: String, val: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.getLong:(Ljava/lang/String;Ljava/lang/Long;)Ljava/lang/Long;")
    }

    // java: compareTo(Ljava/lang/Long;)I
    pub fn compareTo(&self, anotherLong: i64) -> Result<i32> {
        panic!("stub: java/lang/Long.compareTo:(Ljava/lang/Long;)I")
    }

    // java: compare(JJ)I
    pub fn compare(x: i64, arg1: i64) -> Result<i32> {
        panic!("stub: java/lang/Long.compare:(JJ)I")
    }

    // java: compareUnsigned(JJ)I
    pub fn compareUnsigned(x: i64, arg1: i64) -> Result<i32> {
        panic!("stub: java/lang/Long.compareUnsigned:(JJ)I")
    }

    // java: divideUnsigned(JJ)J
    pub fn divideUnsigned(dividend: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.divideUnsigned:(JJ)J")
    }

    // java: remainderUnsigned(JJ)J
    pub fn remainderUnsigned(dividend: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.remainderUnsigned:(JJ)J")
    }

    // java: highestOneBit(J)J
    pub fn highestOneBit(i: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.highestOneBit:(J)J")
    }

    // java: lowestOneBit(J)J
    pub fn lowestOneBit(i: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.lowestOneBit:(J)J")
    }

    // java: numberOfLeadingZeros(J)I
    pub fn numberOfLeadingZeros(i: i64) -> Result<i32> {
        panic!("stub: java/lang/Long.numberOfLeadingZeros:(J)I")
    }

    // java: numberOfTrailingZeros(J)I
    pub fn numberOfTrailingZeros(i: i64) -> Result<i32> {
        panic!("stub: java/lang/Long.numberOfTrailingZeros:(J)I")
    }

    // java: bitCount(J)I
    pub fn bitCount(i: i64) -> Result<i32> {
        panic!("stub: java/lang/Long.bitCount:(J)I")
    }

    // java: rotateLeft(JI)J
    pub fn rotateLeft(i: i64, arg1: i32) -> Result<i64> {
        panic!("stub: java/lang/Long.rotateLeft:(JI)J")
    }

    // java: rotateRight(JI)J
    pub fn rotateRight(i: i64, arg1: i32) -> Result<i64> {
        panic!("stub: java/lang/Long.rotateRight:(JI)J")
    }

    // java: reverse(J)J
    pub fn reverse(i: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.reverse:(J)J")
    }

    // java: compress(JJ)J
    pub fn compress(i: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.compress:(JJ)J")
    }

    // java: expand(JJ)J
    pub fn expand(i: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.expand:(JJ)J")
    }

    // java: parallelSuffix(J)J
    pub fn parallelSuffix(maskCount: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.parallelSuffix:(J)J")
    }

    // java: signum(J)I
    pub fn signum(i: i64) -> Result<i32> {
        panic!("stub: java/lang/Long.signum:(J)I")
    }

    // java: reverseBytes(J)J
    pub fn reverseBytes(i: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.reverseBytes:(J)J")
    }

    // java: sum(JJ)J
    pub fn sum(a: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.sum:(JJ)J")
    }

    // java: max(JJ)J
    pub fn max(a: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.max:(JJ)J")
    }

    // java: min(JJ)J
    pub fn min(a: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Long.min:(JJ)J")
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        panic!("stub: java/lang/Long.describeConstable:()Ljava/util/Optional;")
    }

    // java: resolveConstantDesc(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Long;
    pub fn resolveConstantDesc(&self, lookup: Object) -> Result<i64> {
        panic!("stub: java/lang/Long.resolveConstantDesc:(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Long;")
    }
}
