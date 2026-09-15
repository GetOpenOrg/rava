use java_runtime::prelude::*;
use super::*;

impl Thread {
    #[jvm_native]
    pub fn isTerminated(&self) -> Result<bool> {
        Ok(false)
    }

    #[jvm_native]
    pub fn interrupt(&self) -> Result<()> {
        self.interrupted.set(true);
        Ok(())
    }

    #[jvm_native]
    pub fn getThreadGroup(&self) -> Result<Object> {
        Ok(Object::default())
    }
}
