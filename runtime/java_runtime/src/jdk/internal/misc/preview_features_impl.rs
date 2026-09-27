//! `jdk.internal.misc.PreviewFeatures`（内部边界类）：预览特性开关。
//!
//! 原生二进制不启用预览特性（等价 JVM 缺省未带 `--enable-preview`）。
//! 消费方：`Class.isUnnamedClass`（JEP 445，`getSimpleName` 链）。

use crate::prelude::*;
use super::preview_features::PreviewFeatures;

impl PreviewFeatures {
    /// static `isEnabled()`：预览特性恒关。
    #[jvm_boundary]
    pub fn isEnabled() -> Result<bool> {
        Ok(false)
    }
}
