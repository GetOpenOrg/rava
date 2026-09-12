#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Spliterators",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public final",
    source      = "Spliterators.java",
))]
pub struct Spliterators;

impl Spliterators {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        todo!("abstract java/util/Spliterators.<init>")
    }

    // java: emptySpliterator()Ljava/util/Spliterator;
    pub fn emptySpliterator() -> Result<Object> {
        todo!("abstract java/util/Spliterators.emptySpliterator")
    }

    // java: emptyIntSpliterator()Ljava/util/Spliterator$OfInt;
    pub fn emptyIntSpliterator() -> Result<Object> {
        todo!("abstract java/util/Spliterators.emptyIntSpliterator")
    }

    // java: emptyLongSpliterator()Ljava/util/Spliterator$OfLong;
    pub fn emptyLongSpliterator() -> Result<Object> {
        todo!("abstract java/util/Spliterators.emptyLongSpliterator")
    }

    // java: emptyDoubleSpliterator()Ljava/util/Spliterator$OfDouble;
    pub fn emptyDoubleSpliterator() -> Result<Object> {
        todo!("abstract java/util/Spliterators.emptyDoubleSpliterator")
    }

    // java: spliterator([Ljava/lang/Object;I)Ljava/util/Spliterator;
    pub fn spliterator__arr_obj_i(array: Vec<Object>, additionalCharacteristics: i32) -> Result<Object> {
        todo!("abstract java/util/Spliterators.spliterator")
    }

    // java: spliterator([Ljava/lang/Object;III)Ljava/util/Spliterator;
    pub fn spliterator__arr_obj_i_i_i(array: Vec<Object>, fromIndex: i32, toIndex: i32, additionalCharacteristics: i32) -> Result<Object> {
        todo!("abstract java/util/Spliterators.spliterator")
    }

    // java: spliterator([II)Ljava/util/Spliterator$OfInt;
    pub fn spliterator__arr_i_i(array: Vec<i32>, additionalCharacteristics: i32) -> Result<Object> {
        todo!("abstract java/util/Spliterators.spliterator")
    }

    // java: spliterator([IIII)Ljava/util/Spliterator$OfInt;
    pub fn spliterator__arr_i_i_i_i(array: Vec<i32>, fromIndex: i32, toIndex: i32, additionalCharacteristics: i32) -> Result<Object> {
        todo!("abstract java/util/Spliterators.spliterator")
    }

    // java: spliterator([JI)Ljava/util/Spliterator$OfLong;
    pub fn spliterator__arr_l_i(array: Vec<i64>, additionalCharacteristics: i32) -> Result<Object> {
        todo!("abstract java/util/Spliterators.spliterator")
    }

    // java: spliterator([JIII)Ljava/util/Spliterator$OfLong;
    pub fn spliterator__arr_l_i_i_i(array: Vec<i64>, fromIndex: i32, toIndex: i32, additionalCharacteristics: i32) -> Result<Object> {
        todo!("abstract java/util/Spliterators.spliterator")
    }

    // java: spliterator([DI)Ljava/util/Spliterator$OfDouble;
    pub fn spliterator__arr_d_i(array: Vec<f64>, additionalCharacteristics: i32) -> Result<Object> {
        todo!("abstract java/util/Spliterators.spliterator")
    }

    // java: spliterator([DIII)Ljava/util/Spliterator$OfDouble;
    pub fn spliterator__arr_d_i_i_i(array: Vec<f64>, fromIndex: i32, toIndex: i32, additionalCharacteristics: i32) -> Result<Object> {
        todo!("abstract java/util/Spliterators.spliterator")
    }

    // java: checkFromToBounds(III)V
    pub fn checkFromToBounds(arrayLength: i32, origin: i32, fence: i32) -> Result<()> {
        todo!("abstract java/util/Spliterators.checkFromToBounds")
    }

    // java: spliterator(Ljava/util/Collection;I)Ljava/util/Spliterator;
    pub fn spliterator__coll_i(c: Object, characteristics: i32) -> Result<Object> {
        todo!("abstract java/util/Spliterators.spliterator")
    }

    // java: spliterator(Ljava/util/Iterator;JI)Ljava/util/Spliterator;
    pub fn spliterator__iterat_l_i(iterator: Object, size: i64, arg2: i32) -> Result<Object> {
        todo!("abstract java/util/Spliterators.spliterator")
    }

    // java: spliteratorUnknownSize(Ljava/util/Iterator;I)Ljava/util/Spliterator;
    pub fn spliteratorUnknownSize__iterat_i(iterator: Object, characteristics: i32) -> Result<Object> {
        todo!("abstract java/util/Spliterators.spliteratorUnknownSize")
    }

    // java: spliterator(Ljava/util/PrimitiveIterator$OfInt;JI)Ljava/util/Spliterator$OfInt;
    pub fn spliterator__primit_l_i(iterator: Object, size: i64, arg2: i32) -> Result<Object> {
        todo!("abstract java/util/Spliterators.spliterator")
    }

    // java: spliteratorUnknownSize(Ljava/util/PrimitiveIterator$OfInt;I)Ljava/util/Spliterator$OfInt;
    pub fn spliteratorUnknownSize__primit_i(iterator: Object, characteristics: i32) -> Result<Object> {
        todo!("abstract java/util/Spliterators.spliteratorUnknownSize")
    }

    // java: spliterator(Ljava/util/PrimitiveIterator$OfLong;JI)Ljava/util/Spliterator$OfLong;
    pub fn spliterator__primit_l_i_1(iterator: Object, size: i64, arg2: i32) -> Result<Object> {
        todo!("abstract java/util/Spliterators.spliterator")
    }

    // java: spliteratorUnknownSize(Ljava/util/PrimitiveIterator$OfLong;I)Ljava/util/Spliterator$OfLong;
    pub fn spliteratorUnknownSize__primit_i_1(iterator: Object, characteristics: i32) -> Result<Object> {
        todo!("abstract java/util/Spliterators.spliteratorUnknownSize")
    }

    // java: spliterator(Ljava/util/PrimitiveIterator$OfDouble;JI)Ljava/util/Spliterator$OfDouble;
    pub fn spliterator__primit_l_i_2(iterator: Object, size: i64, arg2: i32) -> Result<Object> {
        todo!("abstract java/util/Spliterators.spliterator")
    }

    // java: spliteratorUnknownSize(Ljava/util/PrimitiveIterator$OfDouble;I)Ljava/util/Spliterator$OfDouble;
    pub fn spliteratorUnknownSize__primit_i_2(iterator: Object, characteristics: i32) -> Result<Object> {
        todo!("abstract java/util/Spliterators.spliteratorUnknownSize")
    }

    // java: iterator(Ljava/util/Spliterator;)Ljava/util/Iterator;
    pub fn iterator__splite(spliterator: Object) -> Result<Object> {
        todo!("abstract java/util/Spliterators.iterator")
    }

    // java: iterator(Ljava/util/Spliterator$OfInt;)Ljava/util/PrimitiveIterator$OfInt;
    pub fn iterator__splite_1(spliterator: Object) -> Result<Object> {
        todo!("abstract java/util/Spliterators.iterator")
    }

    // java: iterator(Ljava/util/Spliterator$OfLong;)Ljava/util/PrimitiveIterator$OfLong;
    pub fn iterator__splite_2(spliterator: Object) -> Result<Object> {
        todo!("abstract java/util/Spliterators.iterator")
    }

    // java: iterator(Ljava/util/Spliterator$OfDouble;)Ljava/util/PrimitiveIterator$OfDouble;
    pub fn iterator__splite_3(spliterator: Object) -> Result<Object> {
        todo!("abstract java/util/Spliterators.iterator")
    }
}
