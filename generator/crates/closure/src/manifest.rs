//! 手写层三清单（runtime/java_runtime/{closure,seeds,vm_intrinsics}.toml）的读取与域判定。
//!
//! 与 `codegen/runtime_manifest.py` 同一数据源、同一语义；库知识（类名）只出现在清单里。

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// 类所属的分析域
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Domain {
    /// 用户类：字节码翻译
    User,
    /// 公开 API（java/、javax/）与放行条目：字节码翻译
    Translate,
    /// 内部边界 / VM 耦合边界 / 翻译域外：整体手写，BFS 截断
    Boundary,
    /// 根类（java/lang/Object）：手写 ObjectVTable
    Root,
}

/// invokedynamic 引导方法分类（vm_intrinsics.toml [indy]）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndyKind {
    Lambda,
    Concat,
    Native,
}

/// 方法返回值事实（[vm_constants] / [facts]）
#[derive(Debug, Clone, PartialEq)]
pub enum Fact {
    Null,
    Int(i32),
}

pub struct Manifest {
    pub runtime_dir: PathBuf,
    boundary_pkgs: Vec<String>,
    vm_boundary: HashSet<String>,
    release: Vec<String>,
    intrinsics: HashSet<String>,
    null_to_false: HashSet<String>,
    returns: HashMap<String, Fact>,
    pub boot_init: Vec<String>,
    indy: HashMap<String, IndyKind>,
}

const OBJECT: &str = "java/lang/Object";
const PUBLIC_API: [&str; 2] = ["java/", "javax/"];

fn load(dir: &Path, name: &str) -> Result<toml::Table, String> {
    let p = dir.join(name);
    match std::fs::read_to_string(&p) {
        Ok(s) => s.parse::<toml::Table>().map_err(|e| format!("{}：{e}", p.display())),
        Err(_) => Ok(toml::Table::new()),
    }
}

fn strings(t: &toml::Table, sec: &str, key: &str) -> Vec<String> {
    t.get(sec)
        .and_then(|s| s.get(key))
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
        .unwrap_or_default()
}

impl Manifest {
    /// `runtime_dir` = runtime/java_runtime
    pub fn load(runtime_dir: &Path) -> Result<Self, String> {
        let closure = load(runtime_dir, "closure.toml")?;
        let seeds = load(runtime_dir, "seeds.toml")?;
        let vm = load(runtime_dir, "vm_intrinsics.toml")?;

        let mut release = strings(&closure, "release", "packages");
        release.extend(strings(&closure, "release", "classes"));
        release.extend(strings(&seeds, "jca", "release_packages"));
        release.extend(strings(&seeds, "jca", "release_classes"));

        let mut intrinsics = HashSet::new();
        if let Some(arr) = vm.get("intrinsic").and_then(|v| v.as_array()) {
            for e in arr {
                let member = e.get("member").and_then(|v| v.as_str()).unwrap_or_default();
                if e.get("kind").is_none() || e.get("reason").is_none() {
                    return Err(format!("vm_intrinsics.toml：内建条目须写明 kind 与 reason：{member}"));
                }
                intrinsics.insert(member.to_string());
            }
        }

        let mut returns = HashMap::new();
        for m in strings(&vm, "vm_constants", "null_returns") {
            returns.insert(m, Fact::Null);
        }
        if let Some(t) = vm.get("facts").and_then(|s| s.get("returns")).and_then(|v| v.as_table()) {
            for (k, v) in t {
                let f = match v {
                    toml::Value::Boolean(b) => Fact::Int(*b as i32),
                    toml::Value::Integer(i) => Fact::Int(*i as i32),
                    toml::Value::String(s) if s == "null" => Fact::Null,
                    _ => return Err(format!("vm_intrinsics.toml [facts] returns：{k} 的值须为 null / 整数 / 布尔")),
                };
                returns.insert(k.clone(), f);
            }
        }

        let mut indy = HashMap::new();
        for (key, kind) in [("lambda", IndyKind::Lambda), ("concat", IndyKind::Concat), ("native", IndyKind::Native)] {
            for m in strings(&vm, "indy", key) {
                indy.insert(m, kind);
            }
        }

        Ok(Manifest {
            runtime_dir: runtime_dir.to_path_buf(),
            boundary_pkgs: strings(&closure, "boundary", "packages"),
            vm_boundary: strings(&closure, "vm_boundary", "classes").into_iter().collect(),
            release,
            intrinsics,
            null_to_false: strings(&vm, "vm_constants", "null_to_false").into_iter().collect(),
            returns,
            boot_init: strings(&seeds, "boot_init", "classes"),
            indy,
        })
    }

    /// 放行条目：包前缀（`/` 结尾）或类（含 `$` 嵌套类）
    fn released(&self, cls: &str) -> bool {
        self.release.iter().any(|r| {
            if r.ends_with('/') {
                cls.starts_with(r.as_str())
            } else {
                cls == r || cls.strip_prefix(r.as_str()).is_some_and(|rest| rest.starts_with('$'))
            }
        })
    }

    /// 类的分析域（`user` = 类来自用户输入）
    pub fn domain(&self, cls: &str, user: bool) -> Domain {
        if user {
            return Domain::User;
        }
        if cls == OBJECT {
            return Domain::Root;
        }
        if self.released(cls) {
            return Domain::Translate;
        }
        if self.boundary_pkgs.iter().any(|p| cls.starts_with(p.as_str())) {
            return Domain::Boundary;
        }
        let outer = cls.split('$').next().unwrap_or(cls);
        if self.vm_boundary.contains(outer) {
            return Domain::Boundary;
        }
        if PUBLIC_API.iter().any(|p| cls.starts_with(p)) {
            Domain::Translate
        } else {
            Domain::Boundary
        }
    }

    /// VM 内建（手写承载、不分析 Java 体）
    pub fn is_intrinsic(&self, member: &str) -> bool {
        self.intrinsics.contains(member)
    }

    /// 方法返回值事实（`类.方法:描述符`）
    pub fn return_fact(&self, member: &str) -> Option<&Fact> {
        self.returns.get(member)
    }

    /// 纯函数：null 实参 → false
    pub fn is_null_to_false(&self, member: &str) -> bool {
        self.null_to_false.contains(member)
    }

    /// 引导方法（`类.方法`）的分类；未登记 = 按普通静态调用分析
    pub fn indy_kind(&self, bsm: &str) -> Option<IndyKind> {
        self.indy.get(bsm).copied()
    }
}
