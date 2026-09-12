#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use crate::java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "TestP0",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public",
    source      = "TestP0.java",
))]
pub struct TestP0;

impl TestP0 {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(II)I", access = "public static"))]
    pub fn add(a: i32, b: i32) -> Result<i32> {
        Ok((a).wrapping_add(b))
    }

    #[cfg_attr(any(), java_method(name = "multiply", descriptor = "(II)I", access = "public static"))]
    pub fn multiply(a: i32, b: i32) -> Result<i32> {
        Ok((a).wrapping_mul(b))
    }

    #[cfg_attr(any(), java_method(name = "factorial", descriptor = "(I)I", access = "public static"))]
    pub fn factorial(n: i32) -> Result<i32> {
        let mut result: i32 = 1i32;
        let mut i: i32 = 2i32;
        loop {
            if i > n { break; }
            result = (result).wrapping_mul(i);
            i = i.wrapping_add(1i32);
        }
        Ok(result)
    }

    #[cfg_attr(any(), java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public static"))]
    pub fn main() -> Result<()> {
        let mut x: i32 = 5i32;
        let mut y: i32 = 10i32;
        let _t0: i32 = Self::add(x, y)?;
        let mut sum: i32 = _t0;
        System::out().println(sum)?;
        let _t1: i32 = Self::multiply(3i32, 7i32)?;
        let mut prod: i32 = _t1;
        System::out().println(prod)?;
        let _t2: i32 = Self::factorial(6i32)?;
        let mut fact: i32 = _t2;
        System::out().println(fact)?;
        let mut rem: i32 = 2i32;
        System::out().println(rem)?;
        Ok(())
    }
}
