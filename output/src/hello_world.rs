#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use crate::java_runtime::prelude::*;

// @java_class(name="HelloWorld", super="java/lang/Object", access="public", source="HelloWorld.java")
pub struct HelloWorld {
// @java_field(name="message", descriptor="Ljava/lang/String;", access="private")
    pub message: Field<String>,
}

impl HelloWorld {
    // @java_method(name="<init>", descriptor="(Ljava/lang/String;)V", access="public")
    pub fn new(message: String) -> Result<Self> {
        let this = Self { message: Field::new(String::new()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.message.set(message);
        Ok(this)
    }

    // @java_method(name="greet", descriptor="()V", access="public")
    pub fn greet(&self) -> Result<()> {
        let this = self;
        System::out().println(String::from_owned(format!("Hello, {}", this.message.get())))?;
        Ok(())
    }

    // @java_method(name="repeat", descriptor="(Ljava/lang/String;I)Ljava/lang/String;", access="public static")
    pub fn repeat(s: String, times: i32) -> Result<String> {
        let mut sb: String = String::new();
        let mut i: i32 = 0i32;
        loop {
            if i >= times { break; }
            sb.append(&s);
            i += 1;
        }
        Ok(sb)
    }

    // @java_method(name="main", descriptor="([Ljava/lang/String;)V", access="public static")
    pub fn main() -> Result<()> {
        let hw: HelloWorld = HelloWorld::new(String::from("World"))?;
        hw.greet()?;
        let r: String = Self::repeat(String::from("ha"), 3i32)?;
        System::out().println(r)?;
        let items: ArrayList<String> = ArrayList::<String>::new()?;
        items.add(String::from("foo"))?;
        items.add(String::from("bar"))?;
        System::out().println(items.size())?;
        Ok(())
    }
}
