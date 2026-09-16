#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::lang::*;

#[derive(Clone, Default, PartialEq)]
pub struct BinaryOperator<T: Clone + Default + 'static>(std::marker::PhantomData<T>);

impl<T: Clone + Default + 'static> BinaryOperator<T> {
    pub fn apply(&self, t: T, u: T) -> Result<T> {
        panic!("stub: java/util/function/BinaryOperator.apply:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;")
    }

    pub fn andThen(&self, after: BinaryOperator<T>) -> Result<BinaryOperator<T>> {
        panic!("stub: java/util/function/BinaryOperator.andThen:(Ljava/util/function/BinaryOperator;)Ljava/util/function/BinaryOperator;")
    }

    pub fn minBy(comparator: Object) -> Result<BinaryOperator<T>> {
        panic!("stub: java/util/function/BinaryOperator.minBy:(Ljava/util/Comparator;)Ljava/util/function/BinaryOperator;")
    }

    pub fn maxBy(comparator: Object) -> Result<BinaryOperator<T>> {
        panic!("stub: java/util/function/BinaryOperator.maxBy:(Ljava/util/Comparator;)Ljava/util/function/BinaryOperator;")
    }
}
