#[derive(Debug)]
pub enum JvmError {
    NullPointerException,
    ArrayIndexOutOfBoundsException(i32),
    ArithmeticException(&'static str),
    ClassCastException,
    StackOverflowError,
    Custom(std::string::String),
}

impl std::fmt::Display for JvmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Result<T> = std::result::Result<T, JvmError>;
