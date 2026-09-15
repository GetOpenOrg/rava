use java_runtime::prelude::*;
use super::*;
use crate::java::io::print_stream::PrintStream;

impl System {
    pub fn currentTimeMillis() -> Result<i64> {
        use std::time::{SystemTime, UNIX_EPOCH};
        Ok(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as i64)
    }

    pub fn nanoTime() -> Result<i64> {
        use std::time::{SystemTime, UNIX_EPOCH};
        Ok(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos() as i64)
    }

    pub fn out() -> PrintStream {
        PrintStream::default()
    }

    pub fn err() -> PrintStream {
        PrintStream::default()
    }
}
