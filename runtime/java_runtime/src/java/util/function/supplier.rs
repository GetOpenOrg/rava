#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::lang::*;

#[derive(Clone, Default, PartialEq)]
pub struct Supplier<T: Clone + Default + 'static>(std::marker::PhantomData<T>);

impl<T: Clone + Default + 'static> Supplier<T> {
    pub fn get(&self) -> Result<T> {
        panic!("stub: java/util/function/Supplier.get:()Ljava/lang/Object;")
    }
}
