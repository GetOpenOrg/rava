#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Comparators$NaturalOrderComparator",
    super_class = "java/lang/Enum",
    interfaces  = "java/util/Comparator",
    access      = "final",
    source      = "Comparators.java",
))]
pub struct Comparators_NaturalOrderComparator;

impl Comparators_NaturalOrderComparator {
    // java: values()[Ljava/util/Comparators$NaturalOrderComparator;
    pub fn values() -> Result<Vec<Object>> {
        let _t0 = Comparators_NaturalOrderComparator::_VALUES().clone()?;
        Ok(_t0)
    }

    // java: valueOf(Ljava/lang/String;)Ljava/util/Comparators$NaturalOrderComparator;
    pub fn valueOf(name: String) -> Result<Object> {
        let _t0: Object = Enum::valueOf(1i32, name)?;
        Ok(_t0)
    }

    // java: <init>(Ljava/lang/String;I)V
    pub fn new(arg_0: String, arg_1: i32) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Enum.<init>:(Ljava/lang/String;I)V */
        Ok(this)
    }

    // java: compare(Ljava/lang/Comparable;Ljava/lang/Comparable;)I
    pub fn compare(&self, c1: Object, c2: Object) -> Result<i32> {
        let this = self;
        let _t0 = c1.compareTo(c2)?;
        Ok(_t0)
    }

    // java: reversed()Ljava/util/Comparator;
    pub fn reversed(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Comparator::reverseOrder()?;
        Ok(_t0)
    }
}
