use java_runtime::prelude::*;
use super::*;

impl Thread {
    pub fn isTerminated(&self) -> Result<bool> {
        Ok(false)
    }

    pub fn interrupt(&self) -> Result<()> {
        self.interrupted.set(true);
        Ok(())
    }

    pub fn getThreadGroup(&self) -> Result<Object> {
        Ok(Object::default())
    }
}
