#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use crate::java_runtime::prelude::*;

// @java_class(name="TestP0", super="java/lang/Object", access="public", source="TestP0.java")
pub struct TestP0;

impl TestP0 {
    // @java_method(name="<init>", descriptor="()V", access="public")
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // @java_method(name="add", descriptor="(II)I", access="public static")
    pub fn add(a: i32, b: i32) -> Result<i32> {
        Ok((a).wrapping_add(b))
    }

    // @java_method(name="multiply", descriptor="(II)I", access="public static")
    pub fn multiply(a: i32, b: i32) -> Result<i32> {
        Ok((a).wrapping_mul(b))
    }

    // @java_method(name="factorial", descriptor="(I)I", access="public static")
    pub fn factorial(n: i32) -> Result<i32> {
        let mut result: i32 = 1i32;
        let mut i: i32 = 2i32;
        loop {
            if i > n { break; }
            result = (result).wrapping_mul(i);
            i += 1;
        }
        Ok(result)
    }

    // @java_method(name="main", descriptor="([Ljava/lang/String;)V", access="public static")
    pub fn main() -> Result<()> {
        let x: i32 = 5i32;
        let y: i32 = 10i32;
        let sum: i32 = Self::add(x, y)?;
        System::out().println(sum)?;
        let prod: i32 = Self::multiply(3i32, 7i32)?;
        System::out().println(prod)?;
        let fact: i32 = Self::factorial(6i32)?;
        System::out().println(fact)?;
        let rem: i32 = 2i32;
        System::out().println(rem)?;
        Ok(())
    }
}
