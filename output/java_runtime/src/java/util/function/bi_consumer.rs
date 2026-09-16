#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::lang::*;

#[derive(Clone, Default, PartialEq)]
pub struct BiConsumer<T: Clone + Default + 'static, U: Clone + Default + 'static>(std::marker::PhantomData<(T, U)>);

impl<T: Clone + Default + 'static, U: Clone + Default + 'static> BiConsumer<T, U> {
    pub fn accept(&self, t: T, u: U) -> Result<()> {
        panic!("stub: java/util/function/BiConsumer.accept:(Ljava/lang/Object;Ljava/lang/Object;)V")
    }

    pub fn andThen(&self, after: BiConsumer<T, U>) -> Result<BiConsumer<T, U>> {
        panic!("stub: java/util/function/BiConsumer.andThen:(Ljava/util/function/BiConsumer;)Ljava/util/function/BiConsumer;")
    }
}
