#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct TestP1 {
    pub x: i32,
    pub y: i32,
}

impl TestP1 {
    pub fn new(arg_0: i32, arg_1: i32) -> Rc<RefCell<Self>> {
        let this: Rc<RefCell<Self>> = Rc::new(RefCell::new(Self { x: 0, y: 0 }));
        /* invokespecial Method java/lang/Object."<init>":()V */
        this.borrow_mut().x = arg_0;
        this.borrow_mut().y = arg_1;
        this
    }

    pub fn sum(this: &Rc<RefCell<Self>>) -> i32 {
        return (this.borrow().x).wrapping_add(this.borrow().y);
    }

    pub fn setX(this: &Rc<RefCell<Self>>, arg_0: i32) {
        this.borrow_mut().x = arg_0;
        return;
    }

    pub fn main() {
        let mut local_1: Rc<RefCell<TestP1>> = TestP1::new(3i32, 4i32);
        let _t0: i32 = TestP1::sum(&local_1);
        println!("{}", _t0);
        TestP1::setX(&local_1, 10i32);
        let _t1: i32 = TestP1::sum(&local_1);
        println!("{}", _t1);
        println!("{}", local_1.borrow().x);
        return;
    }
}
