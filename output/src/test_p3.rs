#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use crate::java_runtime::prelude::*;

// @java_class(name="TestP3", super="java/lang/Object", access="public", source="TestP3.java")
pub struct TestP3;

impl TestP3 {
    // @java_method(name="<init>", descriptor="()V", access="public")
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // @java_method(name="main", descriptor="([Ljava/lang/String;)V", access="public static")
    pub fn main() -> Result<()> {
        let list: ArrayList<i32> = ArrayList::<i32>::new()?;
        list.add(10i32)?;
        list.add(20i32)?;
        list.add(30i32)?;
        System::out().println(list.size())?;
        let _e0: i32 = list.get(1i32)?;
        System::out().println(_e0)?;
        let map: HashMap<String, i32> = HashMap::<String, i32>::new()?;
        map.put(String::from("one"), 1i32);
        map.put(String::from("two"), 2i32);
        map.put(String::from("three"), 3i32);
        System::out().println(map.size())?;
        let _v1 = map.get(&String::from("two")).unwrap_or_default();
        System::out().println(_v1)?;
        System::out().println(map.contains_key(&String::from("one")))?;
        let set: HashSet<i32> = HashSet::<i32>::new()?;
        set.add(100i32);
        set.add(200i32);
        set.add(100i32);
        System::out().println(set.size())?;
        System::out().println(set.contains(&200i32))?;
        Ok(())
    }
}
