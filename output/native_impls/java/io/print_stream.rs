use java_runtime::prelude::*;
use super::*;
use crate::java::lang::*;

impl super::PrintStream {
    // ── 流状态 ──────────────────────────────────────────────────────────────
    // 原始翻译因异常表缺失而提前 athrow，直接 no-op 覆盖
    pub fn ensureOpen(&self) -> Result<()> {
        Ok(())
    }

    // ── 底层写入（synchronized 翻译 broken，用直写替换）────────────────────
    pub fn write_str(&self, s: String) -> Result<()> {
        print!("{}", s);
        Ok(())
    }

    pub fn writeln_str(&self, s: String) -> Result<()> {
        println!("{}", s);
        Ok(())
    }

    pub fn newLine(&self) -> Result<()> {
        println!();
        Ok(())
    }

    pub fn flush(&self) -> Result<()> {
        use std::io::Write;
        let _ = std::io::stdout().flush();
        Ok(())
    }

    // ── print（无换行）──────────────────────────────────────────────────────
    pub fn print_str(&self, x: String) -> Result<()> {
        print!("{}", x);
        Ok(())
    }

    pub fn print_i(&self, i: i32) -> Result<()> {
        print!("{}", i);
        Ok(())
    }

    // ── println（带换行）────────────────────────────────────────────────────
    pub fn println_str(&self, x: String) -> Result<()> {
        println!("{}", x);
        Ok(())
    }

    pub fn println_i(&self, x: i32) -> Result<()> {
        println!("{}", x);
        Ok(())
    }

    pub fn println_z(&self, x: bool) -> Result<()> {
        println!("{}", x);
        Ok(())
    }

    pub fn println_l(&self, x: i64) -> Result<()> {
        println!("{}", x);
        Ok(())
    }

    pub fn println_obj(&self, x: Object) -> Result<()> {
        if let Some(s) = x.0.downcast_ref::<String>() {
            println!("{}", s);
        } else {
            println!("{}", x);
        }
        Ok(())
    }

    pub fn println(&self) -> Result<()> {
        println!();
        Ok(())
    }

    pub fn println_v<T: Printable>(&self, v: T) -> Result<()> {
        println!("{}", v.to_print_string());
        Ok(())
    }
}

impl Printable for String {
    fn to_print_string(&self) -> std::string::String {
        format!("{}", self)
    }
}
