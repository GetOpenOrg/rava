#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use crate::java_runtime::prelude::*;

// @java_class(name="TestP2", super="java/lang/Object", access="public", source="TestP2.java")
pub struct TestP2;

impl TestP2 {
    // @java_method(name="<init>", descriptor="()V", access="public")
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // @java_method(name="sumArray", descriptor="([I)I", access="public static")
    pub fn sumArray(arr: &[i32]) -> Result<i32> {
        let mut sum: i32 = 0i32;
        let mut i: i32 = 0i32;
        loop {
            if i >= (arr.len() as i32) { break; }
            sum = (sum).wrapping_add(arr[i as usize]);
            i += 1;
        }
        Ok(sum)
    }

    // @java_method(name="main", descriptor="([Ljava/lang/String;)V", access="public static")
    pub fn main() -> Result<()> {
        let mut arr: Vec<i32> = vec![0i32; 5i32 as usize];
        arr[0i32 as usize] = 10i32;
        arr[1i32 as usize] = 20i32;
        arr[2i32 as usize] = 30i32;
        arr[3i32 as usize] = 40i32;
        arr[4i32 as usize] = 50i32;
        System::out().println((arr.len() as i32))?;
        let _t1: i32 = Self::sumArray(&arr)?;
        System::out().println(_t1)?;
        System::out().println(arr[2i32 as usize])?;
        let mut _arr2: Vec<i32> = vec![0i32; 5i32 as usize];
        _arr2[0i32 as usize] = 1i32;
        _arr2[1i32 as usize] = 2i32;
        _arr2[2i32 as usize] = 3i32;
        _arr2[3i32 as usize] = 4i32;
        _arr2[4i32 as usize] = 5i32;
        let arr2: Vec<i32> = _arr2;
        let _t3: i32 = Self::sumArray(&arr2)?;
        System::out().println(_t3)?;
        Ok(())
    }
}
