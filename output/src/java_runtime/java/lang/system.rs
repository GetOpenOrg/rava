// java.lang.System.out / System.err 输出存根
pub fn println_i32(v: i32)    { println!("{}", v); }
pub fn println_i64(v: i64)    { println!("{}", v); }
pub fn println_f64(v: f64)    { println!("{}", v); }
pub fn println_bool(v: bool)  { println!("{}", v); }
pub fn println_str(v: &str)   { println!("{}", v); }
pub fn println_empty()        { println!(); }

pub fn eprintln_i32(v: i32)   { eprintln!("{}", v); }
pub fn eprintln_str(v: &str)  { eprintln!("{}", v); }

pub fn exit(code: i32) -> ! { std::process::exit(code); }
pub fn current_time_millis() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64
}
