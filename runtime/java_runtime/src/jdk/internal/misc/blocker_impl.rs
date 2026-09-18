use crate::prelude::*;
use super::blocker::Blocker;

impl Blocker {
    /// 当前线程不是虚拟线程：无需补偿，返回 0。
    #[jvm_boundary]
    pub fn begin() -> Result<i64> {
        Ok(0)
    }

    #[jvm_boundary]
    pub fn end(_compensate_return: i64) -> Result<()> {
        Ok(())
    }
}
