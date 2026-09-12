#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/function/Function",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "Function.java",
))]
pub struct Function<T, R>;

impl<T: Clone + 'static, R: Clone + 'static> Function<T, R> {
    #[cfg_attr(any(), java_native(name = "apply", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public abstract"))]
    pub fn apply(&self, arg0: Object) -> Result<Object> {
        todo!("abstract java/util/function/Function.apply")
    }

    #[cfg_attr(any(), java_method(name = "compose", descriptor = "(Ljava/util/function/Function;)Ljava/util/function/Function;", access = "public"))]
    pub fn compose(&self, before: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(before)?;
        /* TODO: invokedynamic 7 */
        Ok(before)
    }

    #[cfg_attr(any(), java_method(name = "andThen", descriptor = "(Ljava/util/function/Function;)Ljava/util/function/Function;", access = "public"))]
    pub fn andThen(&self, after: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(after)?;
        /* TODO: invokedynamic 11 */
        Ok(after)
    }

    #[cfg_attr(any(), java_method(name = "identity", descriptor = "()Ljava/util/function/Function;", access = "public static"))]
    pub fn identity() -> Result<Object> {
        /* TODO: invokedynamic 12 */
        Ok(todo!("stack underflow"))
    }
}
