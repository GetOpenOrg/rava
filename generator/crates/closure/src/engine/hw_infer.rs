//! 引擎：手写体回调目标推断。
//!
//! 手写层与生成层命名空间同构（Rust 类型路径 ↔ Java binary name；Rust 方法名 = Java 名，重载取
//! 描述符 mangle 名），手写体调用点按「接收者静态类型（方法调用）/ 路径类型（关联函数调用）+ Rust 名 +
//! 实参个数」反解为 Java 方法引用，与 `upcalls` 声明同等处理。声明只兜底语法上不可见的调用
//! （宏内调用、按名反射）。

use super::*;

/// Rust 内建 trait 方法名：同名调用是 Rust 语义（`Rc` 引用克隆），不是 Java 回调
const RUST_TRAIT_METHODS: &[&str] = &["clone"];

impl Engine<'_> {
    /// 类及其全部超类型（超类链与超接口，广度优先，自类在前）
    pub(super) fn supertypes(&self, c: &str) -> Vec<std::sync::Arc<ClassFile>> {
        let mut out = Vec::new();
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut q: VecDeque<String> = VecDeque::from([c.to_string()]);
        while let Some(x) = q.pop_front() {
            if !seen.insert(x.clone()) {
                continue;
            }
            let Some(cf) = self.h.class(&x) else { continue };
            q.extend(cf.super_name.iter().cloned());
            q.extend(cf.interfaces.iter().cloned());
            out.push(cf);
        }
        out
    }

    /// `c` 类型层次上 Rust 名为 `m` 的方法（`<init>` / `<clinit>` 除外）：取最先命中的超类型
    /// （最具体的声明）上的全部匹配，实参个数给定时按形参个数过滤。
    /// 精确名（无重载裸名 / 描述符 mangle 名）全层次不中时，按 Java 名前缀回退（手写层对重载成员
    /// 用裸名或缩写后缀调用，与 `hw_member` 同一回退规则）
    pub(super) fn methods_by_rust_name(&self, c: &str, m: &str, nargs: Option<usize>) -> Vec<(String, String, String, bool)> {
        let types = self.supertypes(c);
        for exact in [true, false] {
            for cf in &types {
                let hits: Vec<(String, String, String, bool)> = cf
                    .methods
                    .iter()
                    .filter(|x| !x.name.starts_with('<'))
                    .filter(|x| {
                        if !exact {
                            return member_matches(m, &x.name);
                        }
                        // Rust 名 = 方法名或「方法名_描述符后缀」：前缀不符即不命中，免算重载与后缀
                        if !m.strip_prefix(x.name.as_str()).is_some_and(|r| r.is_empty() || r.starts_with('_')) {
                            return false;
                        }
                        let (plain, mangled) = self.rust_names(cf, &x.name, &x.desc);
                        plain.as_deref() == Some(m) || mangled == m
                    })
                    .filter(|x| nargs.is_none_or(|n| parse_method(&x.desc).is_some_and(|d| d.params.len() == n)))
                    .map(|x| (cf.name.clone(), x.name.clone(), x.desc.clone(), x.is_static()))
                    .collect();
                if !hits.is_empty() {
                    return hits;
                }
            }
        }
        Vec::new()
    }

    /// 类型层次上 Rust 名为 `m` 的方法的返回类（返回类型须唯一且为引用类型）
    pub(super) fn rust_method_ret(&self, c: &str, m: &str) -> Option<String> {
        let rets: BTreeSet<String> = self
            .methods_by_rust_name(c, m, None)
            .into_iter()
            .filter_map(|(_, _, d, _)| parse_method(&d).and_then(|d| d.ret).map(|r| r.descriptor()))
            .collect();
        if rets.len() != 1 {
            return None;
        }
        match parse_field(rets.first()?)? {
            FieldType::Object(c) => Some(c),
            _ => None,
        }
    }

    /// 手写体调用点反解出的 Java 回调目标：方法调用按接收者静态类型取实例方法，
    /// 路径调用 `T::m(…)` 取 `T` 上的静态方法（构造器经 ctors 另行处理）
    pub(super) fn hw_inferred_upcalls(&self, host: &str, mh: &MemberHw) -> BTreeSet<Upcall> {
        let mut out = BTreeSet::new();
        for c in &mh.calls {
            if RUST_TRAIT_METHODS.contains(&c.name.as_str()) {
                continue;
            }
            let (cls, want_static) = match (&c.recv, &c.path_ty, &c.srecv) {
                (Some(_), _, Some(s)) => (self.stype_class(host, s), false),
                (None, Some(t), _) => (self.resolve_tref(host, t), true),
                _ => continue,
            };
            let Some(cls) = cls else { continue };
            let hits = self.methods_by_rust_name(&cls, &c.name, Some(c.args.len()));
            // 路径调用 `T::f()` 不是方法时是 static 字段读访问器（生成层 static 访问器名 = 字段名）：getstatic
            if hits.is_empty() && want_static && c.args.is_empty() {
                if let Some((_, desc)) = self.static_field(&cls, &c.name) {
                    out.insert(Upcall::Field(MemberRef { owner: cls.clone(), name: c.name.clone(), desc }));
                }
            }
            for (_, name, desc, is_static) in hits {
                if is_static == want_static {
                    out.insert(Upcall::Method(MemberRef { owner: cls.clone(), name, desc }));
                }
            }
        }
        out
    }

    /// 回调目标：声明 ∪ 推断
    pub(super) fn hw_upcalls(&self, host: &str, mh: &MemberHw) -> Vec<Upcall> {
        let mut all = self.hw_inferred_upcalls(host, mh);
        all.extend(mh.upcalls.iter().cloned());
        all.into_iter().collect()
    }
}
