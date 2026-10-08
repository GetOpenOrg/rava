//! `[facts.reflect.direct_invokers]` 直连反射调用：反射调用入口 → 非 @CallerSensitive 目标的特化入口（VM 支持类）。
//!
//! 键为反射调用入口（`类.方法:描述符`，实例方法，接收者是反射对象）；值：
//! - `helper`：特化入口（`类.方法:描述符`，静态方法，形参 = 反射对象 + 原入口形参，返回类型同原入口）。其语义是原入口
//!   在「目标不是 @CallerSensitive、访问器为本地访问器」时的逐句特化（访问检查照常执行），调用点改写为对它的
//!   invokestatic 后栈形不变；
//! - `lookups`：按名查找入口（`类.方法:描述符`）→ 查找口径。`public` = 公开成员、沿超类链（JLS / `Class.getMethod`
//!   的成员查找；超接口的静态方法不被继承）；`declared` = 只查声明类的全部访问级别（`getDeclaredMethod`）。
//!
//! 分析器（`engine/reflect_direct.rs`）在反射调用点的反射对象来自同方法内一个所列查找调用点、名字为常量、
//! 形参类型数组为空、查找类集已知且解析出的目标全是无参、返回引用或 void、非 @CallerSensitive 的静态方法时，
//! 把调用点接到特化入口与各目标，并经折叠（folds `direct_calls`）把调用指令改写为对特化入口的调用。

use std::collections::HashMap;

use classfile::MemberRef;

/// 查找口径
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LookupScope {
    /// 公开成员，沿超类链（数组类即根类的公开成员）
    Public,
    /// 只查声明类，全部访问级别（数组类无声明方法）
    Declared,
}

/// 一个直连反射调用入口
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectInvoker {
    pub helper: MemberRef,
    /// 查找入口（`类.方法:描述符`）→ 口径
    pub lookups: HashMap<String, LookupScope>,
}

/// 按反射调用入口（`类.方法:描述符`）登记
#[derive(Debug, Default, Clone)]
pub struct DirectInvokers(HashMap<String, DirectInvoker>);

/// `类.方法:描述符` → 成员引用
pub fn parse_member(s: &str) -> Option<MemberRef> {
    let (head, desc) = s.split_once(':')?;
    let (owner, name) = head.rsplit_once('.')?;
    (!owner.is_empty() && !name.is_empty() && desc.starts_with('(')).then(|| MemberRef { owner: owner.into(), name: name.into(), desc: desc.into() })
}

impl DirectInvokers {
    pub fn from_toml(sec: Option<&toml::Value>) -> Result<Self, String> {
        let mut out = HashMap::new();
        let Some(t) = sec.and_then(|v| v.as_table()) else { return Ok(DirectInvokers(out)) };
        for (k, v) in t {
            let err = |what: &str| format!("vm_intrinsics.toml [facts.reflect.direct_invokers] {k}：{what}");
            let entry = parse_member(k).ok_or_else(|| err("键须为 `类.方法:描述符`"))?;
            let e = v.as_table().ok_or_else(|| err("须为 { helper = ..., lookups = { ... } }"))?;
            let helper = e.get("helper").and_then(|x| x.as_str()).and_then(parse_member).ok_or_else(|| err("helper 须为 `类.方法:描述符`"))?;
            // 特化入口 = 静态方法，首形参为原入口的接收者，其余形参与返回类型同原入口
            let expect = format!("(L{};{}", entry.owner, &entry.desc[1..]);
            if helper.desc != expect {
                return Err(err(&format!("helper 描述符须为 {expect}")));
            }
            let mut lookups = HashMap::new();
            for (lk, sv) in e.get("lookups").and_then(|x| x.as_table()).ok_or_else(|| err("缺 lookups（查找入口 → 口径）"))? {
                parse_member(lk).ok_or_else(|| err(&format!("lookups 键 {lk} 须为 `类.方法:描述符`")))?;
                let scope = match sv.as_str() {
                    Some("public") => LookupScope::Public,
                    Some("declared") => LookupScope::Declared,
                    _ => return Err(err(&format!("lookups {lk} 的口径须为 \"public\" / \"declared\""))),
                };
                lookups.insert(lk.clone(), scope);
            }
            out.insert(k.clone(), DirectInvoker { helper, lookups });
        }
        Ok(DirectInvokers(out))
    }

    pub fn get(&self, member: &str) -> Option<&DirectInvoker> {
        self.0.get(member)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(src: &str) -> Result<DirectInvokers, String> {
        let v: toml::Value = toml::from_str(src).unwrap();
        DirectInvokers::from_toml(v.get("d"))
    }

    #[test]
    fn parses_entry() {
        let d = parse(
            r#"
[d."p/M.invoke:(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;"]
helper = "p/M$D.invoke:(Lp/M;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;"
lookups = { "p/C.get:(Ljava/lang/String;[Lp/C;)Lp/M;" = "public", "p/C.getD:(Ljava/lang/String;[Lp/C;)Lp/M;" = "declared" }
"#,
        )
        .unwrap();
        let e = d.get("p/M.invoke:(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;").unwrap();
        assert_eq!(e.helper.owner, "p/M$D");
        assert_eq!(e.helper.name, "invoke");
        assert_eq!(e.lookups.get("p/C.get:(Ljava/lang/String;[Lp/C;)Lp/M;"), Some(&LookupScope::Public));
        assert_eq!(e.lookups.get("p/C.getD:(Ljava/lang/String;[Lp/C;)Lp/M;"), Some(&LookupScope::Declared));
    }

    #[test]
    fn rejects_bad_helper_shape() {
        let r = parse(
            r#"
[d."p/M.invoke:(Ljava/lang/Object;)Ljava/lang/Object;"]
helper = "p/M$D.invoke:(Ljava/lang/Object;)Ljava/lang/Object;"
lookups = {}
"#,
        );
        assert!(r.is_err());
        let r = parse(
            r#"
[d."p/M.invoke:(Ljava/lang/Object;)Ljava/lang/Object;"]
helper = "p/M$D.invoke:(Lp/M;Ljava/lang/Object;)Ljava/lang/Object;"
lookups = { "p/C.get:(Ljava/lang/String;)Lp/M;" = "any" }
"#,
        );
        assert!(r.is_err());
    }

    #[test]
    fn absent_section_is_empty() {
        assert!(DirectInvokers::from_toml(None).unwrap().is_empty());
    }
}
