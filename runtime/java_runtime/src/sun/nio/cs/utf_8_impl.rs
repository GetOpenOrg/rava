use crate::prelude::*;
use super::utf_8::UTF_8;

impl UTF_8 {
    /// static final INSTANCE：进程内唯一的 UTF-8 charset 实例。
    #[jvm_boundary]
    pub fn INSTANCE() -> Result<UTF_8> {
        thread_local! {
            static INSTANCE: UTF_8 = {
                let mut cs = UTF_8::default();
                cs._init_not_null();
                cs
            };
        }
        Ok(INSTANCE.with(Clone::clone))
    }
}
