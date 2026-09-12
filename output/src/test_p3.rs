#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use crate::java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "TestP3",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public",
    source      = "TestP3.java",
))]
pub struct TestP3;

impl TestP3 {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public static"))]
    pub fn main() -> Result<()> {
        let mut list: ArrayList<_> = ArrayList::<_>::new()?;
        let _t0 = list.add(10i32)?;
        let _t1 = list.add(20i32)?;
        let _t2 = list.add(30i32)?;
        let _t3 = list.size()?;
        System::out().println(_t3)?;
        let _t4 = list.get(1i32)?;
        System::out().println(_t4)?;
        let mut map: HashMap<_, _> = HashMap::<_, _>::new()?;
        let _t5 = map.put(String::from("one"), 1i32)?;
        let _t6 = map.put(String::from("two"), 2i32)?;
        let _t7 = map.put(String::from("three"), 3i32)?;
        let _t8 = map.size()?;
        System::out().println(_t8)?;
        let _t9 = map.get(String::from("two"))?;
        System::out().println(_t9)?;
        let _t10 = map.containsKey(String::from("one"))?;
        System::out().println(_t10)?;
        let mut set: HashSet<_> = HashSet::<_>::new()?;
        let _t11 = set.add(100i32)?;
        let _t12 = set.add(200i32)?;
        let _t13 = set.add(100i32)?;
        let _t14 = set.size()?;
        System::out().println(_t14)?;
        let _t15 = set.contains(200i32)?;
        System::out().println(_t15)?;
        Ok(())
    }
}
