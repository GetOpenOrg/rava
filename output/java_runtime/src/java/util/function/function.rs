#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::lang::*;

#[derive(Clone, Default, PartialEq)]
pub struct Function<T: Clone + Default + 'static, R: Clone + Default + 'static>(std::marker::PhantomData<(T, R)>);

impl<T: Clone + Default + 'static, R: Clone + Default + 'static> Function<T, R> {
    pub fn apply(&self, t: T) -> Result<R> {
        panic!("stub: java/util/function/Function.apply:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    pub fn compose(&self, before: Function<R, T>) -> Result<Function<R, R>> {
        panic!("stub: java/util/function/Function.compose:(Ljava/util/function/Function;)Ljava/util/function/Function;")
    }

    pub fn andThen(&self, after: Function<R, R>) -> Result<Function<T, R>> {
        panic!("stub: java/util/function/Function.andThen:(Ljava/util/function/Function;)Ljava/util/function/Function;")
    }

    pub fn identity() -> Result<Function<Object, Object>> {
        panic!("stub: java/util/function/Function.identity:()Ljava/util/function/Function;")
    }
}
