#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct TestP3;

impl TestP3 {
    pub fn new() -> Rc<RefCell<Self>> {
        let this: Rc<RefCell<Self>> = Rc::new(RefCell::new(Self {}));
        /* invokespecial Method java/lang/Object."<init>":()V */
        this
    }

    pub fn main() {
        let mut local_1: Vec<i32> = Vec::new();
        local_1.push(10i32);
        local_1.push(20i32);
        local_1.push(30i32);
        println!("{}", (local_1.len() as i32));
        println!("{}", local_1[1i32 as usize]);
        let mut local_2: HashMap<String,i32> = HashMap::new();
        local_2.insert("one".to_string(), 1i32);
        local_2.insert("two".to_string(), 2i32);
        local_2.insert("three".to_string(), 3i32);
        println!("{}", (local_2.len() as i32));
        println!("{}", local_2.get(&"two".to_string()).copied().unwrap_or(0));
        println!("{}", local_2.contains_key(&"one".to_string()));
        let mut local_3: HashSet<i32> = HashSet::new();
        local_3.insert(100i32);
        local_3.insert(200i32);
        local_3.insert(100i32);
        println!("{}", (local_3.len() as i32));
        println!("{}", local_3.contains(&200i32));
        return;
    }
}
