//! 档案发射（T1 档案化 1b，计划 `docs/plans/2026-10-01-cross-test-compile-reuse.md` §6.4）：
//! `rava build|emit --profile profile.json` 时，非用户侧事实取档案，用户侧取本程序单例闭包
//! （[`ClosureFacts::compose`]），合成一次发射。
//!
//! 档案须与当前生成器、runtime 树、JDK 主版本一致（档案键的这三项输入），否则报错：
//! 过期档案给出的 JDK 侧事实与本生成器的发射口径不符。

use std::path::Path;

use input::ClosureFacts;
use serde_json::Value;

/// 生成器源码树摘要（`build.rs`，与 `rava profile` 同源）
const GENERATOR_DIGEST: &str = env!("RAVA_GENERATOR_DIGEST");

/// 读档案并校验其键输入与当前环境一致（`rt` = `runtime/java_runtime`）
pub(crate) fn load(path: &Path, rt: &Path, home: &Path) -> Result<ClosureFacts, String> {
    let at = |e: String| format!("{}：{e}", path.display());
    let text = std::fs::read_to_string(path).map_err(|e| at(e.to_string()))?;
    let v: Value = serde_json::from_str(&text).map_err(|e| at(e.to_string()))?;
    let inputs = v.pointer("/profile/inputs").ok_or_else(|| at("缺 profile.inputs 段（不是 rava profile 产物）".into()))?;
    let field = |k: &str| inputs.get(k).cloned().unwrap_or(Value::Null);
    if field("generator").as_str() != Some(GENERATOR_DIGEST) {
        return Err(at("档案由另一版本的生成器产出（profile.inputs.generator 不符），须重新 rava profile".into()));
    }
    let major = resolve::jdk::release_major(home).ok_or_else(|| format!("读不出 JDK 主版本：{}/release", home.display()))?;
    if field("jdk_major").as_u64() != Some(u64::from(major)) {
        return Err(at(format!("档案的 JDK 主版本 {} 与本次 {major} 不符", field("jdk_major"))));
    }
    let runtime = closure::profile::runtime_digest(rt.parent().unwrap_or(rt))?;
    if field("runtime").as_str() != Some(runtime.as_str()) {
        return Err(at("档案产出后 runtime 树已改变（profile.inputs.runtime 不符），须重新 rava profile".into()));
    }
    ClosureFacts::from_json(&v).map_err(|e| at(e.to_string()))
}

/// 有档案时合成发射事实，否则原样使用单例事实
pub(crate) fn facts<'f>(profile: Option<&ClosureFacts>, single: &'f ClosureFacts, slot: &'f mut Option<ClosureFacts>) -> Result<&'f ClosureFacts, String> {
    let Some(p) = profile else { return Ok(single) };
    let composed = ClosureFacts::compose(p, single).map_err(|e| e.to_string())?;
    Ok(slot.insert(composed))
}
