#[derive(Debug)]
pub enum JvmError {
    NullPointerException,
    ArrayIndexOutOfBounds(i32),
    ArithmeticException(&'static str),
    ClassCastException,
    Custom(String),
}
