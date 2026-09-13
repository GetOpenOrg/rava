#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Objects",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public final",
    source      = "Objects.java",
))]
pub struct Objects;

impl Objects {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        panic!("stub: java/util/Objects.<init>:()V")
    }

    // java: equals(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn equals(a: Object, b: Object) -> Result<bool> {
        panic!("stub: java/util/Objects.equals:(Ljava/lang/Object;Ljava/lang/Object;)Z")
    }

    // java: deepEquals(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn deepEquals(a: Object, b: Object) -> Result<bool> {
        panic!("stub: java/util/Objects.deepEquals:(Ljava/lang/Object;Ljava/lang/Object;)Z")
    }

    // java: hashCode(Ljava/lang/Object;)I
    pub fn hashCode(o: Object) -> Result<i32> {
        panic!("stub: java/util/Objects.hashCode:(Ljava/lang/Object;)I")
    }

    // java: hash([Ljava/lang/Object;)I
    pub fn hash(values: Vec<Object>) -> Result<i32> {
        panic!("stub: java/util/Objects.hash:([Ljava/lang/Object;)I")
    }

    // java: toString(Ljava/lang/Object;)Ljava/lang/String;
    pub fn toString__obj(o: Object) -> Result<String> {
        panic!("stub: java/util/Objects.toString:(Ljava/lang/Object;)Ljava/lang/String;")
    }

    // java: toString(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/String;
    pub fn toString__obj_str(o: Object, nullDefault: String) -> Result<String> {
        panic!("stub: java/util/Objects.toString:(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/String;")
    }

    // java: toIdentityString(Ljava/lang/Object;)Ljava/lang/String;
    pub fn toIdentityString(o: Object) -> Result<String> {
        panic!("stub: java/util/Objects.toIdentityString:(Ljava/lang/Object;)Ljava/lang/String;")
    }

    // java: compare(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn compare(a: Object, b: Object, c: Object) -> Result<i32> {
        panic!("stub: java/util/Objects.compare:(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I")
    }

    // java: requireNonNull(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn requireNonNull__obj(obj: Object) -> Result<Object> {
        panic!("stub: java/util/Objects.requireNonNull:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: requireNonNull(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;
    pub fn requireNonNull__obj_str(obj: Object, message: String) -> Result<Object> {
        panic!("stub: java/util/Objects.requireNonNull:(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;")
    }

    // java: isNull(Ljava/lang/Object;)Z
    pub fn isNull(obj: Object) -> Result<bool> {
        panic!("stub: java/util/Objects.isNull:(Ljava/lang/Object;)Z")
    }

    // java: nonNull(Ljava/lang/Object;)Z
    pub fn nonNull(obj: Object) -> Result<bool> {
        panic!("stub: java/util/Objects.nonNull:(Ljava/lang/Object;)Z")
    }

    // java: requireNonNullElse(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn requireNonNullElse(obj: Object, defaultObj: Object) -> Result<Object> {
        panic!("stub: java/util/Objects.requireNonNullElse:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
    }

    // java: requireNonNullElseGet(Ljava/lang/Object;Ljava/util/function/Supplier;)Ljava/lang/Object;
    pub fn requireNonNullElseGet(obj: Object, supplier: Object) -> Result<Object> {
        panic!("stub: java/util/Objects.requireNonNullElseGet:(Ljava/lang/Object;Ljava/util/function/Supplier;)Ljava/lang/Object;")
    }

    // java: requireNonNull(Ljava/lang/Object;Ljava/util/function/Supplier;)Ljava/lang/Object;
    pub fn requireNonNull__obj_suppli(obj: Object, messageSupplier: Object) -> Result<Object> {
        panic!("stub: java/util/Objects.requireNonNull:(Ljava/lang/Object;Ljava/util/function/Supplier;)Ljava/lang/Object;")
    }

    // java: checkIndex(II)I
    pub fn checkIndex__i_i(index: i32, length: i32) -> Result<i32> {
        panic!("stub: java/util/Objects.checkIndex:(II)I")
    }

    // java: checkFromToIndex(III)I
    pub fn checkFromToIndex__i_i_i(fromIndex: i32, toIndex: i32, length: i32) -> Result<i32> {
        panic!("stub: java/util/Objects.checkFromToIndex:(III)I")
    }

    // java: checkFromIndexSize(III)I
    pub fn checkFromIndexSize__i_i_i(fromIndex: i32, size: i32, length: i32) -> Result<i32> {
        panic!("stub: java/util/Objects.checkFromIndexSize:(III)I")
    }

    // java: checkIndex(JJ)J
    pub fn checkIndex__l_l(index: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/util/Objects.checkIndex:(JJ)J")
    }

    // java: checkFromToIndex(JJJ)J
    pub fn checkFromToIndex__l_l_l(fromIndex: i64, arg1: i64, toIndex: i64) -> Result<i64> {
        panic!("stub: java/util/Objects.checkFromToIndex:(JJJ)J")
    }

    // java: checkFromIndexSize(JJJ)J
    pub fn checkFromIndexSize__l_l_l(fromIndex: i64, arg1: i64, size: i64) -> Result<i64> {
        panic!("stub: java/util/Objects.checkFromIndexSize:(JJJ)J")
    }
}
