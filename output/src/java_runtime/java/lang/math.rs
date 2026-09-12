// java.lang.Math native stubs
pub fn abs_i32(a: i32) -> i32   { a.abs() }
pub fn abs_i64(a: i64) -> i64   { a.abs() }
pub fn abs_f64(a: f64) -> f64   { a.abs() }
pub fn max_i32(a: i32, b: i32) -> i32 { a.max(b) }
pub fn min_i32(a: i32, b: i32) -> i32 { a.min(b) }
pub fn max_f64(a: f64, b: f64) -> f64 { a.max(b) }
pub fn min_f64(a: f64, b: f64) -> f64 { a.min(b) }
pub fn sqrt(a: f64) -> f64      { a.sqrt() }
pub fn pow(a: f64, b: f64) -> f64 { a.powf(b) }
pub fn floor(a: f64) -> f64     { a.floor() }
pub fn ceil(a: f64) -> f64      { a.ceil() }
pub fn round(a: f64) -> i64     { a.round() as i64 }
pub fn log(a: f64) -> f64       { a.ln() }
pub fn log10(a: f64) -> f64     { a.log10() }
pub fn sin(a: f64) -> f64       { a.sin() }
pub fn cos(a: f64) -> f64       { a.cos() }
pub fn tan(a: f64) -> f64       { a.tan() }
