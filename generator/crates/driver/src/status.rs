//! `<scratch>/build_status.json`：`rava build` 本轮停在哪一阶段、是否成功、失败现场、所用 JDK 与重型判定。
//! 批量驱动（run_tests）只读此文件判定构建结果，不解析 stdout。发射完成后另记 `emit` 段（bin 名、声明层类数、
//! 预检全量明细），`rava compile <scratch>` 据此编译已发射的工作区（预检明细原样保留）；编译成功后记 `exe`
//! （可执行文件路径）。

use std::path::{Path, PathBuf};

use emit::precheck::Precheck;
use resolve::jdk::JdkChoice;
use serde_json::json;

use crate::build_opts::Stage;
use crate::cargo::{Failure, Heavy};

pub const STATUS_FILE: &str = "build_status.json";

/// 发射结果中编译阶段需要的部分
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmitSummary {
    pub bin: String,
    /// 声明层 `java_runtime` 类数（重型判定输入）
    pub jdk_classes: usize,
    /// 预检全量明细（可达却缺手写的 native / 可达的存根）：stdout 明细封顶，批量回传只带状态文件时据此看全量
    pub precheck: Precheck,
}

impl EmitSummary {
    /// 从既有 `build_status.json` 读回（须为发射已成功的工作区）
    pub fn read(out: &Path) -> Result<(EmitSummary, serde_json::Value), String> {
        let p = out.join(STATUS_FILE);
        let text = std::fs::read_to_string(&p).map_err(|e| format!("{}：{e}（先 rava build --stop-after emit）", p.display()))?;
        let v: serde_json::Value = serde_json::from_str(&text).map_err(|e| format!("{}：{e}", p.display()))?;
        let e = &v["emit"];
        match (e["bin"].as_str(), e["jdk_classes"].as_u64()) {
            (Some(bin), Some(n)) => {
                let list = |k: &str| -> Vec<String> {
                    e["precheck"][k].as_array().map_or_else(Vec::new, |a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
                };
                let precheck = Precheck { native_missing: list("native_missing"), boundary_stub: list("boundary_stub") };
                Ok((EmitSummary { bin: bin.to_string(), jdk_classes: n as usize, precheck }, v))
            }
            _ => Err(format!("{}：无发射结果（emit 段缺失，发射未完成）", p.display())),
        }
    }
}

/// 本轮构建状态（各阶段推进时更新 `stage`）
#[derive(Debug, Default)]
pub struct BuildStatus {
    pub stage: Stage,
    pub jdk: Option<JdkChoice>,
    pub heavy: Option<Heavy>,
    /// 内存感知作业数决定（编译段）
    pub mem: Option<crate::mem_budget::MemPlan>,
    pub emit: Option<EmitSummary>,
    /// 编译失败现场（仅 cargo 阶段）
    pub failure: Option<Failure>,
    /// 编译成功的可执行文件（编排直接执行它）
    pub exe: Option<PathBuf>,
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
            "mem": self.mem.as_ref().map(crate::mem_budget::MemPlan::to_json),
            "emit": self.emit.as_ref().map(|e| json!({
                "bin": e.bin,
                "jdk_classes": e.jdk_classes,
                "precheck": {
                    "native_missing_count": e.precheck.native_missing.len(),
                    "boundary_stub_count": e.precheck.boundary_stub.len(),
                    "native_missing": e.precheck.native_missing,
                    "boundary_stub": e.precheck.boundary_stub,
                },
            })),
            "exe": self.exe,
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
        assert!(v["exe"].is_null());
        let ok = BuildStatus { stage: Stage::Run, exe: Some(PathBuf::from("/t/debug/a")), ..BuildStatus::default() }.to_json(None);
        assert_eq!(ok["ok"], true);
        assert!(ok["first_error"].is_null());
        assert_eq!(ok["exe"], "/t/debug/a");
    }

    #[test]
    fn emit_summary_round_trip() {
        let d = std::env::temp_dir().join(format!("rava-status-{}", std::process::id()));
        let st = BuildStatus { stage: Stage::Emit, ..BuildStatus::default() };
        st.write(&d, None).unwrap();
        assert!(EmitSummary::read(&d).is_err(), "无 emit 段");
        let precheck = Precheck { native_missing: vec!["p/A.n:()V".into()], boundary_stub: vec!["p/A.s:()V".into(), "p/B.t:()V".into()] };
        let e = EmitSummary { bin: "hello_world".into(), jdk_classes: 260, precheck };
        BuildStatus { stage: Stage::Emit, emit: Some(e.clone()), ..BuildStatus::default() }.write(&d, None).unwrap();
        let (back, v) = EmitSummary::read(&d).unwrap();
        assert_eq!(back, e);
        assert_eq!(v["emit"]["precheck"]["native_missing_count"], 1);
        assert_eq!(v["emit"]["precheck"]["boundary_stub_count"], 2);
        std::fs::remove_dir_all(&d).unwrap();
    }
}
