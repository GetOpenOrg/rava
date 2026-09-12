#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/PrimitiveIterator$OfLong",
    super_class = "java/lang/Object",
    interfaces  = "java/util/PrimitiveIterator",
    access      = "public abstract",
    source      = "PrimitiveIterator.java",
))]
pub struct PrimitiveIterator_OfLong;

impl PrimitiveIterator_OfLong {
    // java: nextLong()J
    pub fn nextLong(&self) -> Result<i64> {
        todo!("abstract java/util/PrimitiveIterator$OfLong.nextLong")
    }

    // java: forEachRemaining(Ljava/util/function/LongConsumer;)V
    // java: forEachRemaining(Ljava/util/function/LongConsumer;)V
    pub fn forEachRemaining__longco(&self, action: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(action)?;
        loop {
            let _t0 = this.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = this.nextLong()?;
            action.accept(_t0)?;
        }
        Ok(())
    }

    // java: next()Ljava/lang/Long;
    pub fn next(&self) -> Result<i64> {
        let this = self;
        let _t0 = this.getClass()?;
        Tripwire::trip(_t0, String::from("{0} calling PrimitiveIterator.OfLong.nextLong()"))?;
        let _t1 = this.nextLong()?;
        Ok(_t1)
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining__consum(&self, action: Object) -> Result<()> {
        let this = self;
        this.forEachRemaining(action)?;
        let _t0: Object = Objects::requireNonNull__obj(action)?;
        let _t1 = this.getClass()?;
        Tripwire::trip(_t1, String::from("{0} calling PrimitiveIterator.OfLong.forEachRemainingLong(action::accept)"))?;
        let _t2: Object = Objects::requireNonNull__obj(action)?;
        /* TODO: invokedynamic 52 */
        this.forEachRemaining(action)?;
        Ok(())
    }
}
