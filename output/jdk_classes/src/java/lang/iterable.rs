#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Iterable",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "Iterable.java",
))]
pub struct Iterable<T>;

impl<T: Clone + 'static> Iterable<T> {
    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        todo!("abstract java/lang/Iterable.iterator")
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(action)?;
        let _t1 = this.iterator()?;
        let mut local_2: Object = _t1;
        loop {
            let _t0 = local_2.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_2.next()?;
            let mut t: Object = _t0;
            action.accept(t)?;
        }
        Ok(())
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.iterator()?;
        let _t1: Object = Spliterators::spliteratorUnknownSize(_t0, 0i32)?;
        Ok(_t1)
    }
}
