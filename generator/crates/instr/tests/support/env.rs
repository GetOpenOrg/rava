//! 按 meta 行重建 Python 转译时的真实环境：类路径（用户类 → JDK jmods → 镜像 / VM 支持类）、
//! 按 Python registry 插入序装入的注册表、短名表、ty / runtime 清单与类级指令事实。

use std::path::{Path, PathBuf};

use input::RuntimeManifest;
use instr::InstrFacts;
use resolve::classpath::{ClassPath, Origin};
use serde_json::{json, Map, Value};
use ty::{Manifest, Registry, ShortNames};

use crate::sim_support::{Names, R};

pub struct Env {
    pub reg: Registry,
    pub names: ShortNames,
    pub manifest: Manifest,
    pub rt: RuntimeManifest,
    pub facts: InstrFacts,
    /// 回放侧短名表（Python 类型文本 ↔ RsType；与 `names` 同一映射）
    pub rnames: Names,
}

/// dump_instr.py 按 `build/<Test>/closure_input/classes` 记录用户类目录，而 scratch 实际按
/// snake_case 命名（`build/test_xxx/`）：原路径不存在时换成 snake_case 目录
fn user_dir(recorded: &str) -> PathBuf {
    let p = PathBuf::from(recorded);
    if p.exists() {
        return p;
    }
    let parts: Vec<_> = p.components().map(|c| c.as_os_str().to_string_lossy().into_owned()).collect();
    let Some(i) = parts.iter().rposition(|c| c == "closure_input").and_then(|i| i.checked_sub(1)) else {
        return p;
    };
    let mut out = PathBuf::new();
    for (k, c) in parts.iter().enumerate() {
        out.push(if k == i { instr::text::to_snake(c) } else { c.clone() });
    }
    out
}

pub fn build_env(meta: &Value) -> R<Env> {
    let s = |k: &str| meta[k].as_str().unwrap_or_default().to_string();
    let mut cp = ClassPath::new();
    cp.add(Origin::User, &user_dir(&s("user_dir"))).map_err(|e| format!("用户类目录：{e}"))?;
    cp.add_jdk(Path::new(&s("java_home"))).map_err(|e| format!("JDK jmods：{e}"))?;
    for d in meta["image_dirs"].as_array().into_iter().flatten() {
        cp.add(Origin::Image, Path::new(d.as_str().unwrap_or_default())).map_err(|e| format!("镜像类目录：{e}"))?;
    }
    let mut reg = Registry::new();
    let mut missing = Vec::new();
    for n in meta["registry"].as_array().into_iter().flatten().filter_map(Value::as_str) {
        match cp.get(n) {
            Some(cf) => {
                reg.insert(cf);
            }
            None => missing.push(n.to_string()),
        }
    }
    if !missing.is_empty() {
        return Err(format!("类路径缺类：{missing:?}"));
    }
    let runtime = s("runtime");
    let manifest = Manifest::load(Path::new(&runtime)).map_err(|e| format!("ty 清单：{e}"))?;
    let rt = RuntimeManifest::load(Path::new(&runtime)).map_err(|e| format!("runtime 清单：{e:?}"))?;
    let names = ShortNames::build(&reg);
    let root = reg.get(ty::consts::OBJECT).map(|ci| ci.class_file());
    let facts = InstrFacts::build(&reg, root, &Path::new(&runtime).join("src"));
    let mut short = Map::new();
    for ci in reg.iter_insertion() {
        short.insert(ci.name().to_string(), Value::String(names.short(ci.name()).into_owned()));
    }
    let rnames = Names::load(&json!({ "short": short }))?;
    Ok(Env { reg, names, manifest, rt, facts, rnames })
}
