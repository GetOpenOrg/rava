#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/PrimitiveIterator$OfInt",
    super_class = "java/lang/Object",
    interfaces  = "java/util/PrimitiveIterator",
    access      = "public abstract",
    source      = "PrimitiveIterator.java",
))]
pub struct PrimitiveIterator_OfInt;

impl PrimitiveIterator_OfInt {
    // java: nextInt()I
    pub fn nextInt(&self) -> Result<i32> {
        todo!("abstract java/util/PrimitiveIterator$OfInt.nextInt")
    }

    // java: forEachRemaining(Ljava/util/function/IntConsumer;)V
    // java: forEachRemaining(Ljava/util/function/IntConsumer;)V
    pub fn forEachRemaining__intcon(&self, action: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(action)?;
        loop {
            let _t0 = this.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = this.nextInt()?;
            action.accept(_t0)?;
        }
        Ok(())
    }

    // java: next()Ljava/lang/Integer;
    pub fn next(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.getClass()?;
        Tripwire::trip(_t0, String::from("{0} calling PrimitiveIterator.OfInt.nextInt()"))?;
        let _t1 = this.nextInt()?;
        Ok(_t1)
    }

    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    // java: forEachRemaining(Ljava/util/function/Consumer;)V
    pub fn forEachRemaining__consum(&self, action: Object) -> Result<()> {
        let this = self;
        this.forEachRemaining(action)?;
        let _t0: Object = Objects::requireNonNull__obj(action)?;
        let _t1 = this.getClass()?;
        Tripwire::trip(_t1, String::from("{0} calling PrimitiveIterator.OfInt.forEachRemainingInt(action::accept)"))?;
        let _t2: Object = Objects::requireNonNull__obj(action)?;
        /* TODO: invokedynamic 52 */
        this.forEachRemaining(action)?;
        Ok(())
    }
}
