//! `java/util/TimeZone` 的 native 方法（与生成的 time_zone.rs 共置）。
//!
//! 对标 HotSpot `TimeZone_md.c`（Linux）：时区 ID 取 `TZ` 环境变量，否则解析
//! `/etc/localtime` 符号链接中 `zoneinfo/` 之后的部分；都取不到时返回 null，
//! 由 Java 侧回落 `getSystemGMTOffsetID()`。ID 的有效性由 Java 侧
//! （ZoneInfoFile 查 tzdb.dat）判定，无效 ID 回落 GMT，与 JVM 行为一致。

use crate::prelude::*;
use super::time_zone::TimeZone;
use crate::java::lang::String;

fn zoneinfo_suffix(path: &str) -> Option<std::string::String> {
    path.find("zoneinfo/").map(|i| path[i + "zoneinfo/".len()..].to_owned())
}

impl TimeZone {
    /// `getSystemTimeZoneID(String javaHome)`：平台时区 ID（取不到 → null）。
    #[jvm_native]
    pub fn getSystemTimeZoneID(_java_home: String) -> Result<String> {
        if let Ok(tz) = std::env::var("TZ") {
            let tz = tz.trim_start_matches(':');
            if !tz.is_empty() {
                // TZ=/usr/share/zoneinfo/Asia/Shanghai 形态取 zoneinfo 之后的部分
                let id = if tz.starts_with('/') {
                    zoneinfo_suffix(tz).unwrap_or_else(|| tz.to_owned())
                } else {
                    tz.to_owned()
                };
                return Ok(String::from(id.as_str()));
            }
        }
        if let Ok(target) = std::fs::read_link("/etc/localtime") {
            if let Some(id) = zoneinfo_suffix(&target.to_string_lossy()) {
                return Ok(String::from(id.as_str()));
            }
        }
        Ok(Default::default())
    }

    /// `getSystemGMTOffsetID()`：按本地 UTC 偏移构造 `GMT±hh:mm`（偏移为 0 → `GMT`）。
    /// 原生二进制不依赖 libc 的 localtime：偏移未知时取 0，与 TZ 未设置的容器环境一致。
    #[jvm_native]
    pub fn getSystemGMTOffsetID() -> Result<String> {
        Ok(String::from("GMT"))
    }
}
