//! JCA 服务种子（seeds.toml `[jca]`）：`Cipher.getInstance("DES")` 经 provider 服务表按类名反射
//! 构造实现类（`Provider$Service.newInstance`），无静态调用边。
//!
//! - 服务表：provider 注册类字节码里 `ldc 类型; ldc 算法; ldc 实现类名` 三连字符串常量；
//! - 入选：服务类型的 engine 类（简单名 == 服务类型）有方法可达，且算法名（大小写不敏感、含同义名）
//!   出现在候选算法串中（用户字符串常量 ∪ 可达 JDK 方法里 `<engine>.getInstance` 前的字符串实参）；
//!   `defaults` 的缺省服务在触发成员可达时入选。

use std::collections::{BTreeSet, HashMap, HashSet};

use classfile::{ClassFile, Const, Insn, Operand};
use resolve::ClassPath;

const INVOKESTATIC: u8 = 0xb8;
const INVOKESPECIAL: u8 = 0xb7;
const PUTSTATIC: u8 = 0xb3;
/// engine 调用实参窗口：`ldc 算法; [ldc provider;] invokestatic <Engine>.getInstance`
const ENGINE_ARG_WINDOW: usize = 3;
const GET_INSTANCE: &str = "getInstance";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Service {
    pub ty: String,
    pub algorithm: String,
    pub imp: String,
    pub provider: String,
}

#[derive(Debug, Default)]
pub struct JcaCfg {
    /// (provider 名, 注册类, Provider 子类)
    pub providers: Vec<(String, String, String)>,
    pub triggers: Vec<String>,
    /// (触发成员 `类.成员`, 类型, 算法)
    pub defaults: Vec<(String, String, String)>,
    pub alias_sources: Vec<String>,
}

impl JcaCfg {
    pub fn from_toml(sec: Option<&toml::Value>) -> Self {
        let Some(sec) = sec else { return Self::default() };
        let arr = |k: &str| sec.get(k).and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let s = |v: &toml::Value, k: &str| v.get(k).and_then(|x| x.as_str()).unwrap_or_default().to_string();
        JcaCfg {
            providers: arr("providers").iter().map(|p| (s(p, "name"), s(p, "class"), s(p, "provider"))).collect(),
            triggers: arr("triggers").iter().filter_map(|x| x.as_str().map(String::from)).collect(),
            defaults: arr("defaults")
                .iter()
                .filter_map(|d| {
                    let svc = s(d, "service");
                    let (t, a) = svc.split_once('.')?;
                    Some((s(d, "trigger"), t.to_string(), a.to_string()))
                })
                .collect(),
            alias_sources: arr("alias_sources").iter().filter_map(|x| x.as_str().map(String::from)).collect(),
        }
    }

    pub fn provider_class(&self, name: &str) -> Option<&str> {
        self.providers.iter().find(|p| p.0 == name).map(|p| p.2.as_str())
    }
}

fn str_lit(i: &Insn) -> Option<&str> {
    match &i.operand {
        Operand::Ldc(Const::String(s)) => Some(s),
        _ => None,
    }
}

fn algorithm_key(s: &str) -> String {
    s.split('/').next().unwrap_or(s).trim().to_lowercase()
}

/// 全部 provider 注册类的服务三元组
pub fn extract_services(cfg: &JcaCfg, cp: &ClassPath) -> BTreeSet<Service> {
    let mut out = BTreeSet::new();
    for (prov, cls, _) in &cfg.providers {
        let Some(cf) = cp.get(cls) else { continue };
        for m in &cf.methods {
            let Some(code) = &m.code else { continue };
            for w in code.insns.windows(3) {
                let (Some(a), Some(b), Some(c)) = (str_lit(&w[0]), str_lit(&w[1]), str_lit(&w[2])) else { continue };
                if a.is_empty() || b.is_empty() || !c.contains('.') || c.contains(' ') {
                    continue;
                }
                let imp = c.replace('.', "/");
                if cp.contains(&imp) {
                    out.insert(Service { ty: a.into(), algorithm: b.into(), imp, provider: prov.clone() });
                }
            }
        }
    }
    out
}

/// 用户类全部字符串常量（算法键）
pub fn user_algorithms(users: &[std::rc::Rc<ClassFile>]) -> HashSet<String> {
    let mut out = HashSet::new();
    for cf in users {
        for m in &cf.methods {
            for i in m.code.iter().flat_map(|c| c.insns.iter()) {
                if let Some(s) = str_lit(i).filter(|s| !s.is_empty()) {
                    out.insert(algorithm_key(s));
                }
            }
        }
    }
    out
}

/// 算法同义名表：别名源类初始化器中每个构造调用前累积的字符串常量即一组同义名
pub fn alias_groups(cfg: &JcaCfg, cp: &ClassPath) -> HashMap<String, HashSet<String>> {
    let mut out: HashMap<String, HashSet<String>> = HashMap::new();
    for cls in &cfg.alias_sources {
        let Some(cf) = cp.get(cls) else { continue };
        for m in cf.methods.iter().filter(|m| m.is_clinit()) {
            let mut group: Vec<String> = Vec::new();
            for i in m.code.iter().flat_map(|c| c.insns.iter()) {
                if let Some(s) = str_lit(i).filter(|s| !s.is_empty()) {
                    group.push(s.to_lowercase());
                } else if i.opcode == INVOKESPECIAL && matches!(&i.operand, Operand::Method(r, _) if r.name == "<init>") {
                    let names: HashSet<String> = group.drain(..).collect();
                    for n in &names {
                        out.entry(n.clone()).or_default().extend(names.iter().cloned());
                    }
                } else if i.opcode == PUTSTATIC {
                    group.clear();
                }
            }
        }
    }
    out
}

/// 方法体中紧邻 `<engine>.getInstance` 静态调用之前的字符串常量
pub fn engine_call_strings(insns: &[Insn], types: &HashSet<&str>, out: &mut HashSet<String>) {
    for (k, i) in insns.iter().enumerate() {
        let Operand::Method(r, _) = &i.operand else { continue };
        if i.opcode != INVOKESTATIC || r.name != GET_INSTANCE {
            continue;
        }
        let simple = r.owner.rsplit('/').next().unwrap_or(&r.owner);
        if !types.contains(simple) {
            continue;
        }
        for p in &insns[k.saturating_sub(ENGINE_ARG_WINDOW)..k] {
            if let Some(s) = str_lit(p).filter(|s| !s.is_empty()) {
                out.insert(algorithm_key(s));
            }
        }
    }
}

/// 入选服务：engine 类可达 且 算法名（或同义名）命中；或属触发的缺省服务
pub fn select<'s>(
    services: &'s BTreeSet<Service>,
    algorithms: &HashSet<String>,
    live_types: &HashSet<String>,
    aliases: &HashMap<String, HashSet<String>>,
    forced: &HashSet<(String, String)>,
) -> Vec<&'s Service> {
    services
        .iter()
        .filter(|s| {
            if forced.contains(&(s.ty.clone(), s.algorithm.clone())) {
                return true;
            }
            let a = s.algorithm.to_lowercase();
            live_types.contains(&s.ty) && (algorithms.contains(&a) || aliases.get(&a).is_some_and(|g| g.iter().any(|x| algorithms.contains(x))))
        })
        .collect()
}
