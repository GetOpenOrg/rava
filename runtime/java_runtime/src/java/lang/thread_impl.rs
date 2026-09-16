use crate::prelude::*;
use super::*;

impl Thread {
    #[jvm_native]
    pub fn isTerminated(&self) -> Result<bool> {
        Ok(false)
    }

    #[jvm_native]
    pub fn interrupt(&self) -> Result<()> {
        self.__set_interrupted(true);
        Ok(())
    }

    #[jvm_native]
    pub fn getThreadGroup(&self) -> Result<Object> {
        Ok(Object::default())
    }
}
