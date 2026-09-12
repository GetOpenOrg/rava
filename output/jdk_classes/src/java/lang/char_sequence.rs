#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/CharSequence",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "CharSequence.java",
))]
pub struct CharSequence;

impl CharSequence {
    // java: length()I
    pub fn length(&self) -> Result<i32> {
        todo!("abstract java/lang/CharSequence.length")
    }

    // java: charAt(I)C
    pub fn charAt(&self, arg0: i32) -> Result<u16> {
        todo!("abstract java/lang/CharSequence.charAt")
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.length()?;
        Ok(_t0==0i32)
    }

    // java: subSequence(II)Ljava/lang/CharSequence;
    pub fn subSequence(&self, arg0: i32, arg1: i32) -> Result<Object> {
        todo!("abstract java/lang/CharSequence.subSequence")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/lang/CharSequence.toString")
    }

    // java: chars()Ljava/util/stream/IntStream;
    pub fn chars(&self) -> Result<Object> {
        let this = self;
        /* TODO: invokedynamic 7 */
        let _t0: Object = StreamSupport::intStream(this, 16464i32, 0i32)?;
        Ok(_t0)
    }

    // java: codePoints()Ljava/util/stream/IntStream;
    pub fn codePoints(&self) -> Result<Object> {
        let this = self;
        /* TODO: invokedynamic 19 */
        let _t0: Object = StreamSupport::intStream(this, 16i32, 0i32)?;
        Ok(_t0)
    }

    // java: compare(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)I
    pub fn compare(cs1: Object, cs2: Object) -> Result<i32> {
        let _t0: Object = Objects::requireNonNull__obj(cs1)?;
        let _t1: Object = Objects::requireNonNull__obj(cs2)?;
        return Ok(0i32);
        let _t2 = cs1.getClass()?;
        let _t3 = cs2.getClass()?;
        let _t4 = cs1.compareTo(cs2)?;
        return Ok(_t4);
        let mut i: i32 = 0i32;
        let _t5 = cs1.length()?;
        let _t6 = cs2.length()?;
        let _t7: i32 = (_t5).min(_t6);
        let mut len: i32 = _t7;
        loop {
            if i >= len { break; }
            let _t0 = cs1.charAt(i)?;
            let mut a: i32 = _t0;
            let _t1 = cs2.charAt(i)?;
            let mut b: i32 = _t1;
            return Ok((a).wrapping_sub(b));
            i = i.wrapping_add(1i32);
        }
        let _t8 = cs1.length()?;
        let _t9 = cs2.length()?;
        Ok((_t8).wrapping_sub(_t9))
    }
}
