//! java.lang.System 同构类型
use crate::java_runtime::error::Result;

pub struct System;
pub struct PrintStream { pub is_err: bool }

impl System {
    pub fn out() -> PrintStream { PrintStream { is_err: false } }
    pub fn err() -> PrintStream { PrintStream { is_err: true } }
    pub fn exit(code: i32) -> ! { std::process::exit(code); }
    pub fn current_time_millis() -> i64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as i64
    }
}

impl PrintStream {
    pub fn println<T: std::fmt::Display>(&self, v: T) -> Result<()> {
        if self.is_err { eprintln!("{}", v); } else { println!("{}", v); }
        Ok(())
    }
    pub fn println_empty(&self) -> Result<()> {
        if self.is_err { eprintln!(); } else { println!(); }
        Ok(())
    }
    pub fn print<T: std::fmt::Display>(&self, v: T) -> Result<()> {
        if self.is_err { eprint!("{}", v); } else { print!("{}", v); }
        Ok(())
    }
    pub fn flush(&self) -> Result<()> { Ok(()) }
}
