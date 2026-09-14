use java_runtime::prelude::*;
use super::*;
use crate::java::lang::*;

impl super::PrintStream {
    pub fn println__str(&self, x: String) -> Result<()> {
        println!("{}", x);
        Ok(())
    }

    pub fn println__i(&self, v: i32) -> Result<()> {
        println!("{}", v);
        Ok(())
    }

    pub fn println(&self) -> Result<()> {
        println!();
        Ok(())
    }

    pub fn flush(&self) -> Result<()> {
        use std::io::Write;
        let _ = std::io::stdout().flush();
        Ok(())
    }

    pub fn println__z(&self, v: bool) -> Result<()> {
        println!("{}", v);
        Ok(())
    }

    pub fn println__j(&self, v: i64) -> Result<()> {
        println!("{}", v);
        Ok(())
    }

    pub fn println__obj(&self, x: Object) -> Result<()> {
        if let Some(s) = x.0.downcast_ref::<String>() {
            println!("{}", s);
        } else {
            println!("{}", x);
        }
        Ok(())
    }

    pub fn print__str(&self, x: String) -> Result<()> {
        print!("{}", x);
        Ok(())
    }

    /// Java: System.out.println(x) — 统一 Printable 派发
    pub fn println_v<T: Printable>(&self, v: T) -> Result<()> {
        println!("{}", v.to_print_string());
        Ok(())
    }
}

// String 实现 Printable（在 jdk_classes 上下文中定义，因为 String 类型在此）
impl Printable for String {
    fn to_print_string(&self) -> std::string::String {
        format!("{}", self)
    }
}
