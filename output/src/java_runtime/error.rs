// java_runtime/error.rs
#[derive(Debug)]
pub enum JvmError {
    NullPointerException,
    ArrayIndexOutOfBounds(i32),
    ArithmeticException(&'static str),
    ClassCastException,
    Custom(String),
}
