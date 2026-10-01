//! `<scratch>/build_status.json`：`rava build` 本轮停在哪一阶段、是否成功、失败现场、所用 JDK 与重型判定。
//! 批量驱动（run_tests）只读此文件判定构建结果，不解析 stdout。

use std::path::Path;

use resolve::jdk::JdkChoice;
use serde_json::json;

use crate::build_opts::Stage;
use crate::cargo::{Failure, Heavy};

pub const STATUS_FILE: &str = "build_status.json";

/// 本轮构建状态（各阶段推进时更新 `stage`）
#[derive(Debug, Default)]
pub struct BuildStatus {
    pub stage: Stage,
    pub jdk: Option<JdkChoice>,
    pub heavy: Option<Heavy>,
    /// 编译失败现场（仅 cargo 阶段）
    pub failure: Option<Failure>,
}

impl BuildStatus {
    pub fn to_json(&self, err: Option<&String>) -> serde_json::Value {
        let f = self.failure.clone().unwrap_or_default();
        let first_error = match (&self.failure, err) {
            (Some(f), _) => Some(f.first_error.clone()),
            (None, Some(e)) => Some(e.lines().next().unwrap_or_default().to_string()),
            (None, None) => None,
        };
        json!({
            "stage": self.stage.name(),
            "ok": err.is_none(),
            "exit": f.exit,
            "signal": f.signal,
            "timeout": f.timeout,
            "first_error": first_error,
            "log": f.log,
            "jdk": self.jdk.as_ref().map(|j| json!({
                "home": j.home, "major": j.major, "source": j.source.to_string(),
            })),
            "heavy": self.heavy.as_ref().map(Heavy::to_json),
        })
    }

    /// 写入 scratch（scratch 尚未创建时一并创建）
    pub fn write(&self, out: &Path, err: Option<&String>) -> Result<(), String> {
        std::fs::create_dir_all(out).map_err(|e| format!("{}：{e}", out.display()))?;
        let p = out.join(STATUS_FILE);
        let text = serde_json::to_string_pretty(&self.to_json(err)).unwrap_or_default();
        std::fs::write(&p, text).map_err(|e| format!("{}：{e}", p.display()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failure_fields_and_stage() {
        let mut st = BuildStatus { stage: Stage::Compile, ..BuildStatus::default() };
        st.failure = Some(Failure { exit: Some(101), first_error: "error[E0308]: x".into(), ..Failure::default() });
        let v = st.to_json(Some(&"cargo build 失败".to_string()));
        assert_eq!(v["stage"], "compile");
        assert_eq!(v["ok"], false);
        assert_eq!(v["exit"], 101);
        assert_eq!(v["first_error"], "error[E0308]: x");
        let ok = BuildStatus { stage: Stage::Run, ..BuildStatus::default() }.to_json(None);
        assert_eq!(ok["ok"], true);
        assert!(ok["first_error"].is_null());
    }
}
