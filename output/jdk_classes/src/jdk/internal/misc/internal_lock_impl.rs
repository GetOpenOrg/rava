use java_runtime::prelude::*;
use super::*;

use std::cell::RefCell;

thread_local! {
    // 可重入锁深度计数：同线程多次 lock/unlock 配对，与 JDK ReentrantLock 语义一致
    static DEPTH: RefCell<usize> = RefCell::new(0);
}

impl InternalLock {
    pub fn lock(&self) -> Result<()> {
        DEPTH.with(|d| *d.borrow_mut() += 1);
        Ok(())
    }

    pub fn unlock(&self) -> Result<()> {
        DEPTH.with(|d| {
            let mut depth = d.borrow_mut();
            if *depth > 0 {
                *depth -= 1;
            }
        });
        Ok(())
    }
}
