// java.io.PrintStream 存根（对应 System.out / System.err）
pub struct PrintStream { pub is_err: bool }

impl PrintStream {
    pub fn println_i32(&self, v: i32)   { if self.is_err { eprintln!("{}", v); } else { println!("{}", v); } }
    pub fn println_i64(&self, v: i64)   { if self.is_err { eprintln!("{}", v); } else { println!("{}", v); } }
    pub fn println_f64(&self, v: f64)   { if self.is_err { eprintln!("{}", v); } else { println!("{}", v); } }
    pub fn println_bool(&self, v: bool) { if self.is_err { eprintln!("{}", v); } else { println!("{}", v); } }
    pub fn println_str(&self, v: &str)  { if self.is_err { eprintln!("{}", v); } else { println!("{}", v); } }
    pub fn println(&self)               { if self.is_err { eprintln!(); }        else { println!(); } }
}
