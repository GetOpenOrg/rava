#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Math$RandomNumberGeneratorHolder",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "final",
    source      = "Math.java",
))]
pub struct Math_RandomNumberGeneratorHolder;

impl Math_RandomNumberGeneratorHolder {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }
}
