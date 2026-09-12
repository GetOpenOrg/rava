#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct TestP0;

impl TestP0 {
    pub fn new() -> Rc<RefCell<Self>> {
        let this: Rc<RefCell<Self>> = Rc::new(RefCell::new(Self {}));
        /* invokespecial Method java/lang/Object."<init>":()V */
        this
    }

    pub fn add(arg_0: i32, arg_1: i32) -> i32 {
        return (arg_0).wrapping_add(arg_1);
    }

    pub fn multiply(arg_0: i32, arg_1: i32) -> i32 {
        return (arg_0).wrapping_mul(arg_1);
    }

    pub fn factorial(arg_0: i32) -> i32 {
        let mut local_1: i32 = 1i32;
        let mut local_2: i32 = 2i32;
        loop {
            if local_2 > arg_0 { break; }
        local_1 = (local_1).wrapping_mul(local_2);
        local_2 = local_2.wrapping_add(1i32);
        }  // end loop
        return local_1;
    }

    pub fn main() {
        let mut local_1: i32 = 5i32;
        let mut local_2: i32 = 10i32;
        let _t0: i32 = Self::add(local_1, local_2);
        let mut local_3: i32 = _t0;
        println!("{}", local_3);
        let _t1: i32 = Self::multiply(3i32, 7i32);
        let mut local_4: i32 = _t1;
        println!("{}", local_4);
        let _t2: i32 = Self::factorial(6i32);
        let mut local_5: i32 = _t2;
        println!("{}", local_5);
        let mut local_6: i32 = 2i32;
        println!("{}", local_6);
        return;
    }
}
