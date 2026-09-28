//! `jdk/internal/ref/CleanerFactory` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! JDK 在 `<clinit>` 以 InnocuousThread 工厂创建公共 Cleaner（启动守护清理线程）。本运行时无
//! GC / 幻象引用入队（见 phantom_cleanable_impl.rs）：清理线程恒空转，只承载 Cleaner 身份对象，
//! 不创建线程。

use crate::prelude::*;
use super::cleaner_factory::CleanerFactory;
use crate::java::lang::r#ref::Cleaner;

impl CleanerFactory {
    /// `cleaner()`：进程内唯一的公共 Cleaner 身份对象（无清理线程）。
    #[jvm_boundary]
    pub fn cleaner() -> Result<Cleaner> {
        crate::__process_static! {
            static COMMON: Cleaner = {
                let mut c = Cleaner::default();
                c._init_not_null();
                c
            };
        }
        Ok(COMMON.with(Clone::clone))
    }
}
