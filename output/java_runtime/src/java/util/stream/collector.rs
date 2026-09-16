// PERMANENT minimal stub for Collector interface.
// No java_class annotation so _write() never overwrites this.
#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types)]
use crate::prelude::*;
use crate::java::util::*;

#[derive(Clone, Default, PartialEq)]
pub struct Collector<T: Clone + Default + 'static, A: Clone + Default + 'static, R: Clone + Default + 'static>(pub std::marker::PhantomData<(T, A, R)>);

impl<T: Clone + Default + 'static, A: Clone + Default + 'static, R: Clone + Default + 'static> Collector<T, A, R> {
    pub fn supplier(&self) -> Result<Object> {
        panic!("stub: java/util/stream/Collector.supplier")
    }
    pub fn accumulator(&self) -> Result<Object> {
        panic!("stub: java/util/stream/Collector.accumulator")
    }
    pub fn combiner(&self) -> Result<Object> {
        panic!("stub: java/util/stream/Collector.combiner")
    }
    pub fn finisher(&self) -> Result<Object> {
        panic!("stub: java/util/stream/Collector.finisher")
    }
    pub fn characteristics(&self) -> Result<Set<Object>> {
        panic!("stub: java/util/stream/Collector.characteristics")
    }
}
