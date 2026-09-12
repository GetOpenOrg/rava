// java_runtime/io.rs — System.out / System.err 存根
pub fn println_int(v: i32)    { println!("{}", v); }
pub fn println_str(v: &str)   { println!("{}", v); }
pub fn println_long(v: i64)   { println!("{}", v); }
pub fn println_double(v: f64) { println!("{}", v); }
pub fn println_bool(v: bool)  { println!("{}", v); }
pub fn println_empty()        { println!(); }
