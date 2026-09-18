use crate::prelude::*;
use super::*;
use crate::java::io::print_stream::PrintStream;

impl System {
    #[jvm_native]
    pub fn arraycopy(src: Object, src_pos: i32, dest: Object, dest_pos: i32, length: i32) -> Result<()> {
        macro_rules! try_copy {
            ($t:ty) => {
                if let Some(s) = src.0.as_any().downcast_ref::<JArray<$t>>() {
                    let d = dest.downcast::<JArray<$t>>();
                    for i in 0..length {
                        d.set(dest_pos + i, s.get(src_pos + i));
                    }
                    return Ok(());
                }
            };
        }
        try_copy!(i8);
        try_copy!(i32);
        try_copy!(i64);
        try_copy!(u16);
        try_copy!(f32);
        try_copy!(f64);
        try_copy!(bool);
        try_copy!(i16);
        try_copy!(Object);
        panic!("stub: System.arraycopy: unsupported array element type")
    }

    #[jvm_native]
    pub fn currentTimeMillis() -> Result<i64> {
        use std::time::{SystemTime, UNIX_EPOCH};
        Ok(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as i64)
    }

    #[jvm_native]
    pub fn nanoTime() -> Result<i64> {
        use std::time::{SystemTime, UNIX_EPOCH};
        Ok(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos() as i64)
    }

    #[jvm_native]
    pub fn out() -> PrintStream {
        PrintStream::default()
    }

    #[jvm_native]
    pub fn err() -> PrintStream {
        PrintStream::default()
    }
}
