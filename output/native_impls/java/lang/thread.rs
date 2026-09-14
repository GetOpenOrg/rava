// @jvm_class: java/lang/Thread
use java_runtime::prelude::*;
use super::*;

impl super::Thread {
    pub fn isTerminated(&self) -> Result<bool> {
        Ok(false)
    }
}
