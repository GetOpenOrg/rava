//! `jdk/internal/misc/PreviewFeatures` 的 ACC_NATIVE（类 1）。

use crate::prelude::*;
use super::preview_features::PreviewFeatures;

impl PreviewFeatures {
    /// native `isPreviewEnabled()`：HotSpot 返回 `--enable-preview` 开关；原生二进制按缺省启动 → false。
    #[jvm_native]
    pub fn isPreviewEnabled() -> Result<bool> {
        Ok(false)
    }
}
