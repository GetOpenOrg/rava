#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/function/Consumer",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "Consumer.java",
))]
pub struct Consumer<T>;

impl<T: Clone + 'static> Consumer<T> {
    #[cfg_attr(any(), java_native(name = "accept", descriptor = "(Ljava/lang/Object;)V", access = "public abstract"))]
    pub fn accept(&self, arg0: Object) -> Result<()> {
        todo!("abstract java/util/function/Consumer.accept")
    }

    #[cfg_attr(any(), java_method(name = "andThen", descriptor = "(Ljava/util/function/Consumer;)Ljava/util/function/Consumer;", access = "public"))]
    pub fn andThen(&self, after: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(after)?;
        /* TODO: invokedynamic 7 */
        Ok(after)
    }
}
