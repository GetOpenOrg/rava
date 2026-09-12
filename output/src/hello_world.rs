#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use crate::java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "HelloWorld",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public",
    source      = "HelloWorld.java",
))]
pub struct HelloWorld {
    #[cfg_attr(any(), java_field(name = "message", descriptor = "Ljava/lang/String;", access = "private"))]
    pub message: Field<String>,
}

impl HelloWorld {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn new(message: String) -> Result<Self> {
        let this = Self { message: Field::new(String::new()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.message.set(message);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "greet", descriptor = "()V", access = "public"))]
    pub fn greet(&self) -> Result<()> {
        let this = self;
        System::out().println(String::from_owned(format!("Hello, {}", this.message.get())))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "repeat", descriptor = "(Ljava/lang/String;I)Ljava/lang/String;", access = "public static"))]
    pub fn repeat(s: String, times: i32) -> Result<String> {
        let mut sb: String = String::new();
        let mut i: i32 = 0i32;
        loop {
            if i >= times { break; }
            sb.append(&s)?;
            i = i.wrapping_add(1i32);
        }
        Ok(sb)
    }

    #[cfg_attr(any(), java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public static"))]
    pub fn main() -> Result<()> {
        let mut hw: HelloWorld = HelloWorld::new(String::from("World"))?;
        hw.greet()?;
        let _t0: String = Self::repeat(String::from("ha"), 3i32)?;
        let mut r: String = _t0;
        System::out().println(r)?;
        let mut items: ArrayList<_> = ArrayList::<_>::new()?;
        let _t1 = items.add(String::from("foo"))?;
        let _t2 = items.add(String::from("bar"))?;
        let _t3 = items.size()?;
        System::out().println(_t3)?;
        Ok(())
    }
}
