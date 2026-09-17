use crate::prelude::*;
use super::string::String;

impl String {
    #[jvm_native]
    pub fn intern(&self) -> Result<String> {
        Ok(Clone::clone(self))
    }
}
