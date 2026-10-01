//! 静默兜底审计（record / summary 模式）。
//!
//! 口径：非存根的质量降级点（解析失败回退默认值、代换失败沿用原文、路径拼装失败放弃合成对象）。
//! 存根兜底（A 组）归 `[cfg-audit]` 的位点分解，不在本行。目标全 0：非零即极可能是真 bug。
//!
//! 签名 / 形参表解析器是全函数（残缺签名在类型层硬失败），`vars` 渲染与类型渲染不会失败
//! （IR 渲染是全函数），SAM 描述符映射同理——这些位置没有失败分支，无需计数。降级点见 [`FALLBACK_IDS`]。

use std::collections::BTreeMap;
use std::sync::Mutex;

/// 降级点 ID 全集（`scripts/run_tests.py` `FALLBACK_IDS` 同步收录）
///
/// | ID | 触发条件 → 降级形态 |
/// |---|---|
/// | `class-extras` | 类补充属性（LVT / 注解 / Deprecated）二次解析失败 → 视为无补充属性 |
/// | `lvt-substitute` | 接口方法展开时局部变量表签名的类型变量代换失败 → 沿用原签名 |
/// | `sam-ctor-path` | SAM 合成对象构造路径含非法标识符段 → 站点回落闭包装箱 |
pub const FALLBACK_IDS: [&str; 3] = ["class-extras", "lvt-substitute", "sam-ctor-path"];

/// 触发计数（逐类并行发射共享；经共享引用记录）
#[derive(Debug, Default)]
pub struct FallbackAudit {
    counts: Mutex<BTreeMap<&'static str, usize>>,
    debug: bool,
}

impl FallbackAudit {
    /// `debug`：逐触发向 stderr 打印明细（`[fallback-audit] {id} {detail}`）
    pub fn new(debug: bool) -> FallbackAudit {
        FallbackAudit { counts: Mutex::new(BTreeMap::new()), debug }
    }

    /// 某降级点触发一次（只计数与明细，不改变降级行为本身）
    pub fn record(&self, id: &'static str, detail: impl FnOnce() -> String) {
        debug_assert!(FALLBACK_IDS.contains(&id), "未登记的降级点 {id}");
        *self.counts.lock().unwrap_or_else(|e| e.into_inner()).entry(id).or_default() += 1;
        if self.debug {
            let d = detail();
            eprintln!("{}", format!("[fallback-audit] {id} {d}").trim_end());
        }
    }

    pub fn count(&self, id: &str) -> usize {
        self.counts.lock().unwrap_or_else(|e| e.into_inner()).get(id).copied().unwrap_or(0)
    }

    /// `[fallback-audit]` 行：只列非零项（按 [`FALLBACK_IDS`] 序）；全零输出 none
    pub fn summary(&self) -> String {
        let c = self.counts.lock().unwrap_or_else(|e| e.into_inner());
        let items: Vec<String> =
            FALLBACK_IDS.iter().filter_map(|id| c.get(id).filter(|n| **n > 0).map(|n| format!("{id}={n}"))).collect();
        format!("[fallback-audit] {}", if items.is_empty() { "none".to_string() } else { items.join(" ") })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_lists_nonzero_in_id_order() {
        let a = FallbackAudit::new(false);
        assert_eq!(a.summary(), "[fallback-audit] none");
        a.record("sam-ctor-path", String::new);
        a.record("class-extras", || "p/A".into());
        a.record("sam-ctor-path", String::new);
        assert_eq!(a.summary(), "[fallback-audit] class-extras=1 sam-ctor-path=2");
        assert_eq!(a.count("lvt-substitute"), 0);
    }
}
