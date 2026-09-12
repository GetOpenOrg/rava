#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Arrays$NaturalOrder",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Comparator",
    access      = "final",
    source      = "Arrays.java",
))]
pub struct Arrays_NaturalOrder;

impl Arrays_NaturalOrder {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "compare", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)I", access = "public"))]
    pub fn compare(&self, first: Object, second: Object) -> Result<i32> {
        let this = self;
        let _t0 = first.compareTo(second)?;
        Ok(_t0)
    }
}
