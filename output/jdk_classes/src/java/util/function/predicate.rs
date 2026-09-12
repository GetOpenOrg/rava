#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/function/Predicate",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "Predicate.java",
))]
pub struct Predicate<T>;

impl<T: Clone + 'static> Predicate<T> {
    #[cfg_attr(any(), java_native(name = "test", descriptor = "(Ljava/lang/Object;)Z", access = "public abstract"))]
    pub fn test(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/util/function/Predicate.test")
    }

    #[cfg_attr(any(), java_method(name = "and", descriptor = "(Ljava/util/function/Predicate;)Ljava/util/function/Predicate;", access = "public"))]
    pub fn and(&self, other: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(other)?;
        /* TODO: invokedynamic 7 */
        Ok(other)
    }

    #[cfg_attr(any(), java_method(name = "negate", descriptor = "()Ljava/util/function/Predicate;", access = "public"))]
    pub fn negate(&self) -> Result<Object> {
        let this = self;
        /* TODO: invokedynamic 11 */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "or", descriptor = "(Ljava/util/function/Predicate;)Ljava/util/function/Predicate;", access = "public"))]
    pub fn or(&self, other: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(other)?;
        /* TODO: invokedynamic 14 */
        Ok(other)
    }

    #[cfg_attr(any(), java_method(name = "isEqual", descriptor = "(Ljava/lang/Object;)Ljava/util/function/Predicate;", access = "public static"))]
    pub fn isEqual(targetRef: Object) -> Result<Object> {
        /* TODO: aconst_null  */
        /* TODO: invokedynamic 15 */
        /* TODO: invokedynamic 18 */
        Ok(targetRef)
    }

    #[cfg_attr(any(), java_method(name = "not", descriptor = "(Ljava/util/function/Predicate;)Ljava/util/function/Predicate;", access = "public static"))]
    pub fn not(target: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(target)?;
        let _t1 = target.negate()?;
        Ok(_t1)
    }
}
