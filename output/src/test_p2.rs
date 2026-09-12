#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct TestP2;

impl TestP2 {
    pub fn new() -> Rc<RefCell<Self>> {
        let this: Rc<RefCell<Self>> = Rc::new(RefCell::new(Self {}));
        /* invokespecial Method java/lang/Object."<init>":()V */
        this
    }

    pub fn sumArray(arg_0: &[i32]) -> i32 {
        let mut local_1: i32 = 0i32;
        let mut local_2: i32 = 0i32;
        loop {
            if local_2 >= (arg_0.len() as i32) { break; }
        local_1 = (local_1).wrapping_add(arg_0[local_2 as usize]);
        local_2 = local_2.wrapping_add(1i32);
        }  // end loop
        return local_1;
    }

    pub fn main() {
        let mut _arr0: Vec<i32> = vec![0i32; 5i32 as usize];
        let mut local_1: Vec<i32> = _arr0;
        local_1[0i32 as usize] = 10i32;
        local_1[1i32 as usize] = 20i32;
        local_1[2i32 as usize] = 30i32;
        local_1[3i32 as usize] = 40i32;
        local_1[4i32 as usize] = 50i32;
        println!("{}", (local_1.len() as i32));
        let _t1: i32 = Self::sumArray(&local_1);
        println!("{}", _t1);
        println!("{}", local_1[2i32 as usize]);
        let mut _arr2: Vec<i32> = vec![0i32; 5i32 as usize];
        _arr2[0i32 as usize] = 1i32;
        _arr2[1i32 as usize] = 2i32;
        _arr2[2i32 as usize] = 3i32;
        _arr2[3i32 as usize] = 4i32;
        _arr2[4i32 as usize] = 5i32;
        let mut local_2: Vec<i32> = _arr2;
        let _t3: i32 = Self::sumArray(&local_2);
        println!("{}", _t3);
        return;
    }
}
