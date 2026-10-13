//! `jdk/internal/misc/PreviewFeatures` 的 ACC_NATIVE（类 1）。

use crate::jdk::internal::misc::PreviewFeatures__Methods as _;
use crate::prelude::*;
use super::preview_features::PreviewFeatures;
use super::__PreviewFeaturesHw;

impl __PreviewFeaturesHw for PreviewFeatures {
    /// native `isPreviewEnabled()`：HotSpot 返回 `--enable-preview` 开关；原生二进制按缺省启动 → false。
    #[jvm_native]
    #[cfg_attr(any(), rava_hw_pub)]
    fn isPreviewEnabled() -> Result<bool> {
        Ok(false)
    }
}
