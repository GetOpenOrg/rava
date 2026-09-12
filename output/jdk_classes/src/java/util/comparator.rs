#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Comparator",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "Comparator.java",
))]
pub struct Comparator<T>(std::marker::PhantomData<T>);

impl<T: Clone + 'static> Comparator<T> {
    // java: compare(Ljava/lang/Object;Ljava/lang/Object;)I
    pub fn compare(&self, arg0: Object, arg1: Object) -> Result<i32> {
        todo!("abstract java/util/Comparator.compare")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/Comparator.equals")
    }

    // java: reversed()Ljava/util/Comparator;
    pub fn reversed(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Collections::reverseOrder__compar(this)?;
        Ok(_t0)
    }

    // java: thenComparing(Ljava/util/Comparator;)Ljava/util/Comparator;
    // java: thenComparing(Ljava/util/Comparator;)Ljava/util/Comparator;
    pub fn thenComparing__compar(&self, other: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(other)?;
        /* TODO: invokedynamic 13 */
        Ok(other)
    }

    // java: thenComparing(Ljava/util/function/Function;Ljava/util/Comparator;)Ljava/util/Comparator;
    // java: thenComparing(Ljava/util/function/Function;Ljava/util/Comparator;)Ljava/util/Comparator;
    pub fn thenComparing__functi_compar(&self, keyExtractor: Object, keyComparator: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Comparator::comparing__functi_compar(keyExtractor, keyComparator)?;
        let _t1 = this.thenComparing__compar(_t0)?;
        Ok(_t1)
    }

    // java: thenComparing(Ljava/util/function/Function;)Ljava/util/Comparator;
    // java: thenComparing(Ljava/util/function/Function;)Ljava/util/Comparator;
    pub fn thenComparing__functi(&self, keyExtractor: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Comparator::comparing__functi(keyExtractor)?;
        let _t1 = this.thenComparing__compar(_t0)?;
        Ok(_t1)
    }

    // java: thenComparingInt(Ljava/util/function/ToIntFunction;)Ljava/util/Comparator;
    pub fn thenComparingInt(&self, keyExtractor: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Comparator::comparingInt(keyExtractor)?;
        let _t1 = this.thenComparing__compar(_t0)?;
        Ok(_t1)
    }

    // java: thenComparingLong(Ljava/util/function/ToLongFunction;)Ljava/util/Comparator;
    pub fn thenComparingLong(&self, keyExtractor: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Comparator::comparingLong(keyExtractor)?;
        let _t1 = this.thenComparing__compar(_t0)?;
        Ok(_t1)
    }

    // java: thenComparingDouble(Ljava/util/function/ToDoubleFunction;)Ljava/util/Comparator;
    pub fn thenComparingDouble(&self, keyExtractor: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Comparator::comparingDouble(keyExtractor)?;
        let _t1 = this.thenComparing__compar(_t0)?;
        Ok(_t1)
    }

    // java: reverseOrder()Ljava/util/Comparator;
    pub fn reverseOrder() -> Result<Object> {
        let _t0: Object = Collections::reverseOrder()?;
        Ok(_t0)
    }

    // java: naturalOrder()Ljava/util/Comparator;
    pub fn naturalOrder() -> Result<Object> {
        Ok(Comparators_NaturalOrderComparator::INSTANCE())
    }

    // java: nullsFirst(Ljava/util/Comparator;)Ljava/util/Comparator;
    pub fn nullsFirst(comparator: Object) -> Result<Object> {
        Ok(Comparators_NullComparator::new(1i32, comparator)?)
    }

    // java: nullsLast(Ljava/util/Comparator;)Ljava/util/Comparator;
    pub fn nullsLast(comparator: Object) -> Result<Object> {
        Ok(Comparators_NullComparator::new(0i32, comparator)?)
    }

    // java: comparing(Ljava/util/function/Function;Ljava/util/Comparator;)Ljava/util/Comparator;
    // java: comparing(Ljava/util/function/Function;Ljava/util/Comparator;)Ljava/util/Comparator;
    pub fn comparing__functi_compar(keyExtractor: Object, keyComparator: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(keyExtractor)?;
        let _t1: Object = Objects::requireNonNull__obj(keyComparator)?;
        /* TODO: invokedynamic 58 */
        Ok(keyExtractor)
    }

    // java: comparing(Ljava/util/function/Function;)Ljava/util/Comparator;
    // java: comparing(Ljava/util/function/Function;)Ljava/util/Comparator;
    pub fn comparing__functi(keyExtractor: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(keyExtractor)?;
        /* TODO: invokedynamic 61 */
        Ok(keyExtractor)
    }

    // java: comparingInt(Ljava/util/function/ToIntFunction;)Ljava/util/Comparator;
    pub fn comparingInt(keyExtractor: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(keyExtractor)?;
        /* TODO: invokedynamic 63 */
        Ok(keyExtractor)
    }

    // java: comparingLong(Ljava/util/function/ToLongFunction;)Ljava/util/Comparator;
    pub fn comparingLong(keyExtractor: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(keyExtractor)?;
        /* TODO: invokedynamic 65 */
        Ok(keyExtractor)
    }

    // java: comparingDouble(Ljava/util/function/ToDoubleFunction;)Ljava/util/Comparator;
    pub fn comparingDouble(keyExtractor: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull__obj(keyExtractor)?;
        /* TODO: invokedynamic 67 */
        Ok(keyExtractor)
    }
}
