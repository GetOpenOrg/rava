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
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // java: emptySpliterator()Ljava/util/Spliterator;
    pub fn emptySpliterator() -> Result<Object> {
        Ok(Spliterators::EMPTY_SPLITERATOR())
    }

    // java: emptyIntSpliterator()Ljava/util/Spliterator$OfInt;
    pub fn emptyIntSpliterator() -> Result<Object> {
        Ok(Spliterators::EMPTY_INT_SPLITERATOR())
    }

    // java: emptyLongSpliterator()Ljava/util/Spliterator$OfLong;
    pub fn emptyLongSpliterator() -> Result<Object> {
        Ok(Spliterators::EMPTY_LONG_SPLITERATOR())
    }

    // java: emptyDoubleSpliterator()Ljava/util/Spliterator$OfDouble;
    pub fn emptyDoubleSpliterator() -> Result<Object> {
        Ok(Spliterators::EMPTY_DOUBLE_SPLITERATOR())
    }

    // java: spliterator([Ljava/lang/Object;I)Ljava/util/Spliterator;
    // java: spliterator([Ljava/lang/Object;I)Ljava/util/Spliterator;
    pub fn spliterator__arr_obj_i(array: &[Object], additionalCharacteristics: i32) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(&array)?;
        Ok(Spliterators_ArraySpliterator::new(_t0, additionalCharacteristics)?)
    }

    // java: spliterator([Ljava/lang/Object;III)Ljava/util/Spliterator;
    // java: spliterator([Ljava/lang/Object;III)Ljava/util/Spliterator;
    pub fn spliterator__arr_obj_i_i_i(array: &[Object], fromIndex: i32, toIndex: i32, additionalCharacteristics: i32) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(&array)?;
        Spliterators::checkFromToBounds((_t0.len() as i32), fromIndex, toIndex)?;
        Ok(Spliterators_ArraySpliterator::new(array, fromIndex, toIndex, additionalCharacteristics)?)
    }

    // java: spliterator([II)Ljava/util/Spliterator$OfInt;
    // java: spliterator([II)Ljava/util/Spliterator$OfInt;
    pub fn spliterator__arr_i_i(array: &[i32], additionalCharacteristics: i32) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(&array)?;
        Ok(Spliterators_IntArraySpliterator::new(_t0, additionalCharacteristics)?)
    }

    // java: spliterator([IIII)Ljava/util/Spliterator$OfInt;
    // java: spliterator([IIII)Ljava/util/Spliterator$OfInt;
    pub fn spliterator__arr_i_i_i_i(array: &[i32], fromIndex: i32, toIndex: i32, additionalCharacteristics: i32) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(&array)?;
        Spliterators::checkFromToBounds((_t0.len() as i32), fromIndex, toIndex)?;
        Ok(Spliterators_IntArraySpliterator::new(array, fromIndex, toIndex, additionalCharacteristics)?)
    }

    // java: spliterator([JI)Ljava/util/Spliterator$OfLong;
    // java: spliterator([JI)Ljava/util/Spliterator$OfLong;
    pub fn spliterator__arr_l_i(array: &[i64], additionalCharacteristics: i32) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(&array)?;
        Ok(Spliterators_LongArraySpliterator::new(_t0, additionalCharacteristics)?)
    }

    // java: spliterator([JIII)Ljava/util/Spliterator$OfLong;
    // java: spliterator([JIII)Ljava/util/Spliterator$OfLong;
    pub fn spliterator__arr_l_i_i_i(array: &[i64], fromIndex: i32, toIndex: i32, additionalCharacteristics: i32) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(&array)?;
        Spliterators::checkFromToBounds((_t0.len() as i32), fromIndex, toIndex)?;
        Ok(Spliterators_LongArraySpliterator::new(array, fromIndex, toIndex, additionalCharacteristics)?)
    }

    // java: spliterator([DI)Ljava/util/Spliterator$OfDouble;
    // java: spliterator([DI)Ljava/util/Spliterator$OfDouble;
    pub fn spliterator__arr_d_i(array: &[f64], additionalCharacteristics: i32) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(&array)?;
        Ok(Spliterators_DoubleArraySpliterator::new(_t0, additionalCharacteristics)?)
    }

    // java: spliterator([DIII)Ljava/util/Spliterator$OfDouble;
    // java: spliterator([DIII)Ljava/util/Spliterator$OfDouble;
    pub fn spliterator__arr_d_i_i_i(array: &[f64], fromIndex: i32, toIndex: i32, additionalCharacteristics: i32) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(&array)?;
        Spliterators::checkFromToBounds((_t0.len() as i32), fromIndex, toIndex)?;
        Ok(Spliterators_DoubleArraySpliterator::new(array, fromIndex, toIndex, additionalCharacteristics)?)
    }

    // java: checkFromToBounds(III)V
    pub fn checkFromToBounds(arrayLength: i32, origin: i32, fence: i32) -> Result<()> {
        String::new().append(&String::from("origin("))?;
        String::new().append(&origin)?;
        String::new().append(&String::from(") > fence("))?;
        String::new().append(&fence)?;
        String::new().append(&String::from(")"))?;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: spliterator(Ljava/util/Collection;I)Ljava/util/Spliterator;
    // java: spliterator(Ljava/util/Collection;I)Ljava/util/Spliterator;
    pub fn spliterator__coll_i(c: Object, characteristics: i32) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(c)?;
        Ok(Spliterators_IteratorSpliterator::new(_t0, characteristics)?)
    }

    // java: spliterator(Ljava/util/Iterator;JI)Ljava/util/Spliterator;
    // java: spliterator(Ljava/util/Iterator;JI)Ljava/util/Spliterator;
    pub fn spliterator__iterat_l_i(iterator: Object, size: i64, arg_2: i32) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(iterator)?;
        Ok(Spliterators_IteratorSpliterator::new(_t0, size, local_3)?)
    }

    // java: spliteratorUnknownSize(Ljava/util/Iterator;I)Ljava/util/Spliterator;
    // java: spliteratorUnknownSize(Ljava/util/Iterator;I)Ljava/util/Spliterator;
    pub fn spliteratorUnknownSize__iterat_i(iterator: Object, characteristics: i32) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(iterator)?;
        Ok(Spliterators_IteratorSpliterator::new(_t0, characteristics)?)
    }

    // java: spliterator(Ljava/util/PrimitiveIterator$OfInt;JI)Ljava/util/Spliterator$OfInt;
    // java: spliterator(Ljava/util/PrimitiveIterator$OfInt;JI)Ljava/util/Spliterator$OfInt;
    pub fn spliterator__primit_l_i(iterator: Object, size: i64, arg_2: i32) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(iterator)?;
        Ok(Spliterators_IntIteratorSpliterator::new(_t0, size, local_3)?)
    }

    // java: spliteratorUnknownSize(Ljava/util/PrimitiveIterator$OfInt;I)Ljava/util/Spliterator$OfInt;
    // java: spliteratorUnknownSize(Ljava/util/PrimitiveIterator$OfInt;I)Ljava/util/Spliterator$OfInt;
    pub fn spliteratorUnknownSize__primit_i(iterator: Object, characteristics: i32) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(iterator)?;
        Ok(Spliterators_IntIteratorSpliterator::new(_t0, characteristics)?)
    }

    // java: spliterator(Ljava/util/PrimitiveIterator$OfLong;JI)Ljava/util/Spliterator$OfLong;
    // java: spliterator(Ljava/util/PrimitiveIterator$OfLong;JI)Ljava/util/Spliterator$OfLong;
    pub fn spliterator__primit_l_i(iterator: Object, size: i64, arg_2: i32) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(iterator)?;
        Ok(Spliterators_LongIteratorSpliterator::new(_t0, size, local_3)?)
    }

    // java: spliteratorUnknownSize(Ljava/util/PrimitiveIterator$OfLong;I)Ljava/util/Spliterator$OfLong;
    // java: spliteratorUnknownSize(Ljava/util/PrimitiveIterator$OfLong;I)Ljava/util/Spliterator$OfLong;
    pub fn spliteratorUnknownSize__primit_i(iterator: Object, characteristics: i32) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(iterator)?;
        Ok(Spliterators_LongIteratorSpliterator::new(_t0, characteristics)?)
    }

    // java: spliterator(Ljava/util/PrimitiveIterator$OfDouble;JI)Ljava/util/Spliterator$OfDouble;
    // java: spliterator(Ljava/util/PrimitiveIterator$OfDouble;JI)Ljava/util/Spliterator$OfDouble;
    pub fn spliterator__primit_l_i(iterator: Object, size: i64, arg_2: i32) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(iterator)?;
        Ok(Spliterators_DoubleIteratorSpliterator::new(_t0, size, local_3)?)
    }

    // java: spliteratorUnknownSize(Ljava/util/PrimitiveIterator$OfDouble;I)Ljava/util/Spliterator$OfDouble;
    // java: spliteratorUnknownSize(Ljava/util/PrimitiveIterator$OfDouble;I)Ljava/util/Spliterator$OfDouble;
    pub fn spliteratorUnknownSize__primit_i(iterator: Object, characteristics: i32) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(iterator)?;
        Ok(Spliterators_DoubleIteratorSpliterator::new(_t0, characteristics)?)
    }

    // java: iterator(Ljava/util/Spliterator;)Ljava/util/Iterator;
    // java: iterator(Ljava/util/Spliterator;)Ljava/util/Iterator;
    pub fn iterator__splite(spliterator: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(spliterator)?;
        Ok(Spliterators_1Adapter::new(spliterator)?)
    }

    // java: iterator(Ljava/util/Spliterator$OfInt;)Ljava/util/PrimitiveIterator$OfInt;
    // java: iterator(Ljava/util/Spliterator$OfInt;)Ljava/util/PrimitiveIterator$OfInt;
    pub fn iterator__splite(spliterator: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(spliterator)?;
        Ok(Spliterators_2Adapter::new(spliterator)?)
    }

    // java: iterator(Ljava/util/Spliterator$OfLong;)Ljava/util/PrimitiveIterator$OfLong;
    // java: iterator(Ljava/util/Spliterator$OfLong;)Ljava/util/PrimitiveIterator$OfLong;
    pub fn iterator__splite(spliterator: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(spliterator)?;
        Ok(Spliterators_3Adapter::new(spliterator)?)
    }

    // java: iterator(Ljava/util/Spliterator$OfDouble;)Ljava/util/PrimitiveIterator$OfDouble;
    // java: iterator(Ljava/util/Spliterator$OfDouble;)Ljava/util/PrimitiveIterator$OfDouble;
    pub fn iterator__splite(spliterator: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(spliterator)?;
        Ok(Spliterators_4Adapter::new(spliterator)?)
    }
}
