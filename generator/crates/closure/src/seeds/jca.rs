//! JCA 服务种子（seeds.toml `[jca]`）：`Cipher.getInstance("DES")` 经 provider 服务表按类名反射
//! 构造实现类（`Provider$Service.newInstance`），无静态调用边。
//!
//! - 服务表：provider 注册类字节码里 `ldc 类型; ldc 算法; ldc 实现类名` 三连字符串常量；
//!   provider 的注册类有成员可达（provider 已构造并注册服务）时其服务才可能被取到；
//! - 请求点：可达方法（用户与 JDK）里 `invokestatic <Engine>.getInstance(String 算法, ..)`，Engine 简单名为服务类型；
//!   算法实参按值流求全部名字（字面量 / 形参与 String 字段槽，含 JDK 内部按形参间接请求，如 HMAC 构造器把
//!   摘要算法名传给 `MessageDigest.getInstance`），推不出时该服务类型的全部算法都可能被请求；
//! - 入选：服务类型的 engine 类有方法可达，且算法名（大小写不敏感、含同义名）被请求或该类型请求名推不出；
//!   `defaults` 的缺省服务在触发成员可达时入选。

use std::collections::{BTreeSet, HashMap, HashSet};

use classfile::{Const, Insn, Operand};
use resolve::ClassPath;

const INVOKESTATIC: u8 = 0xb8;
const INVOKESPECIAL: u8 = 0xb7;
const PUTSTATIC: u8 = 0xb3;
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

    /// 注册类（`/` 分隔）→ provider 名
    pub fn registrar_of(&self, name: &str) -> Option<&str> {
        self.providers.iter().find(|p| p.0 == name).map(|p| p.1.as_str())
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

/// 请求名的算法键：整名，另加首个 `/` 之前的部分（变换串 `AES/CBC/PKCS5Padding` 按算法 `AES` 取服务；
/// `HmacSHA512/256`、`SHA-512/224` 等名字本身含 `/` 的服务按整名命中）
pub fn algorithm_keys(s: &str) -> Vec<String> {
    let full = s.trim().to_lowercase();
    match full.split_once('/') {
        Some((head, _)) => vec![head.trim().to_string(), full.clone()],
        None => vec![full],
    }
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

/// 方法体中的服务请求点：`invokestatic <Engine>.getInstance(String 算法, ..)`（string_desc = String 的类型描述符），
/// Engine 简单名为服务类型 → (偏移, 服务类型)
pub fn request_sites(insns: &[Insn], types: &HashSet<&str>, string_desc: &str) -> Vec<(u32, String)> {
    let mut out = Vec::new();
    for i in insns {
        let Operand::Method(r, _) = &i.operand else { continue };
        if i.opcode != INVOKESTATIC || r.name != GET_INSTANCE || !r.desc.strip_prefix('(').is_some_and(|d| d.starts_with(string_desc)) {
            continue;
        }
        let simple = r.owner.rsplit('/').next().unwrap_or(&r.owner);
        if types.contains(simple) {
            out.push((i.offset, simple.to_string()));
        }
    }
    out
}

/// 入选服务：provider 已注册、engine 类可达 且 算法名（或同义名）命中或该类型请求名推不出；或属触发的缺省服务
pub fn select<'s>(
    services: &'s BTreeSet<Service>,
    algorithms: &HashSet<String>,
    any_types: &HashSet<String>,
    live_providers: &HashSet<String>,
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
            if !live_providers.contains(&s.provider) || !live_types.contains(&s.ty) {
                return false;
            }
            let a = s.algorithm.to_lowercase();
            any_types.contains(&s.ty) || (algorithms.contains(&a) || aliases.get(&a).is_some_and(|g| g.iter().any(|x| algorithms.contains(x))))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn svc(ty: &str, algo: &str, provider: &str) -> Service {
        Service { ty: ty.into(), algorithm: algo.into(), imp: format!("p/{algo}"), provider: provider.into() }
    }

    #[test]
    fn algorithm_keys_keep_full_name_and_head() {
        assert_eq!(algorithm_keys("AES/CBC/PKCS5Padding"), ["aes", "aes/cbc/pkcs5padding"]);
        assert_eq!(algorithm_keys("HmacSHA512/256"), ["hmacsha512", "hmacsha512/256"]);
        assert_eq!(algorithm_keys("SHA-256"), ["sha-256"]);
    }

    #[test]
    fn select_needs_registered_provider_and_requested_or_unknown_name() {
        let services: BTreeSet<Service> = [svc("D", "X", "P"), svc("D", "Y", "P"), svc("D", "Z", "Q"), svc("S", "W", "P")].into();
        let set = |xs: &[&str]| xs.iter().map(|x| x.to_string()).collect::<HashSet<String>>();
        let live = set(&["D", "S"]);
        let none = HashMap::new();
        let forced = HashSet::new();
        let names = |v: Vec<&Service>| v.iter().map(|s| s.algorithm.clone()).collect::<Vec<_>>();
        // 只请求 x：Q 未注册，Z 不入选
        let r = select(&services, &set(&["x"]), &set(&[]), &set(&["P"]), &live, &none, &forced);
        assert_eq!(names(r), ["X"]);
        // 类型 D 的请求名推不出：P 的 D 全部入选，S 不受影响
        let r = select(&services, &set(&[]), &set(&["D"]), &set(&["P"]), &live, &none, &forced);
        assert_eq!(names(r), ["X", "Y"]);
    }
}
