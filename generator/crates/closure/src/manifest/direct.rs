//! `[facts.reflect.direct_invokers]` 直连反射调用：反射调用入口 → 非 @CallerSensitive 目标的特化入口（VM 支持类）。
//!
//! 键为反射调用入口（`类.方法:描述符`，实例方法，接收者是反射对象，其类型下称「反射类型」）；值：
//! - `helper`：特化入口（`类.方法:描述符`，静态方法，形参 = 反射对象 + 原入口形参，返回类型同原入口）。其语义是原入口
//!   在「目标不是 @CallerSensitive、访问器为本地访问器」时的逐句特化（访问检查照常执行），调用点改写为对它的
//!   invokestatic 后栈形不变；
//! - `lookups`：查找入口（`类.方法:描述符`）→ 查找口径。`public` = 公开成员、沿超类链与超接口（`Class.getMethod`；
//!   超接口的静态方法不被继承）；`declared` = 只查声明类的全部访问级别（`getDeclaredMethod(s)`）；`declared_public` =
//!   只查声明类的公开方法（`JavaLangAccess.getDeclaredPublicMethods`）。查找入口的形参只能是名字（String）、查找类
//!   （Class，缺省取 Class 接收者）、形参类型（Class[]）；无名字形参即枚举全部方法。结果形态由返回类型决定：
//!   反射类型 = 单个对象；反射类型数组 = 对象数组；`list` 的返回类型 = 列表（经 `list` 建模）；
//! - `list`（可选）：列表形结果的分析模型（`类.方法:描述符`，静态方法，形参为反射类型数组，返回列表：按数组顺序装入新建
//!   列表，同 `Class.getDeclaredPublicMethods` 的「新建 ArrayList 逐个 add」）。分析器把列表形查找的结果建模为它对
//!   标记数组的返回值，运行期不调用；
//! - `native`（可选）：helper 体内执行目标调用的 native（`类.方法:描述符`，描述符同 helper）。它的执行即「按反射对象的
//!   声明键调用目标」，直连调用点已逐目标接边（接收者 / 目标形参取实参数组元素 / 返回值流入调用点结果），故分析器不把它的
//!   返回值当作 open 交出、不把形参值池按返回类型交给逃逸汇点（否则全部直连点的接收者与实参数组按 Object 逃逸）；
//! - `copies`（可选）：复制入口（`类.方法:描述符`，返回反射类型；输入为反射类型的形参，无则为接收者）。复制保持所指
//!   方法不变（`ReflectionFactory.copyMethod` / `Method.copy`），分析器让输入的标记原样成为结果。
//!
//! 分析器（`engine/method_marks.rs`、`engine/reflect_direct.rs`）把查找结果建模为携带所指方法的标记对象，反射调用点的
//! 接收者全是标记、所指方法全非 @CallerSensitive 时把调用点接到特化入口与各目标，并经折叠（folds `direct_calls`）
//! 把调用指令改写为对特化入口的调用。

use std::collections::{HashMap, HashSet};

use classfile::MemberRef;

/// 查找口径
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LookupScope {
    /// 公开成员，沿超类链与超接口（数组类即根类的公开成员）
    Public,
    /// 只查声明类，全部访问级别（数组类无声明方法）
    Declared,
    /// 只查声明类的公开方法
    DeclaredPublic,
}

/// 查找结果的形态（由查找入口的返回类型决定）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LookupShape {
    /// 单个反射对象
    One,
    /// 反射对象数组
    Array,
    /// 列表（经 `list` 建模）
    List,
}

/// 一个查找入口
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lookup {
    pub scope: LookupScope,
    pub shape: LookupShape,
}

/// 一个直连反射调用入口
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectInvoker {
    pub helper: MemberRef,
    /// 反射类型（入口的接收者类型）
    pub recv: String,
    /// 查找入口（`类.方法:描述符`）→ 口径与形态
    pub lookups: HashMap<String, Lookup>,
    /// 列表形结果的分析模型
    pub list: Option<MemberRef>,
    /// 复制入口（`类.方法:描述符`）
    pub copies: HashSet<String>,
    /// helper 体内执行目标调用的 native
    pub native: Option<MemberRef>,
}

/// 按反射调用入口（`类.方法:描述符`）登记
#[derive(Debug, Default, Clone)]
pub struct DirectInvokers {
    by_entry: HashMap<String, DirectInvoker>,
    /// 查找入口 → 反射调用入口
    lookup_of: HashMap<String, String>,
    /// 复制入口 → 反射调用入口
    copy_of: HashMap<String, String>,
    /// 各入口 helper 体内执行目标调用的 native（`类.方法:描述符`）
    natives: HashSet<String>,
}

/// `类.方法:描述符` → 成员引用
pub fn parse_member(s: &str) -> Option<MemberRef> {
    let (head, desc) = s.split_once(':')?;
    let (owner, name) = head.rsplit_once('.')?;
    (!owner.is_empty() && !name.is_empty() && desc.starts_with('(')).then(|| MemberRef { owner: owner.into(), name: name.into(), desc: desc.into() })
}

fn ret_of(desc: &str) -> &str {
    desc.rsplit_once(')').map_or("", |(_, r)| r)
}

impl DirectInvokers {
    pub fn from_toml(sec: Option<&toml::Value>) -> Result<Self, String> {
        let mut out = DirectInvokers::default();
        let Some(t) = sec.and_then(|v| v.as_table()) else { return Ok(out) };
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
            let recv = entry.owner.clone();
            let one = format!("L{recv};");
            let arr = format!("[L{recv};");
            let list = match e.get("list") {
                None => None,
                Some(x) => {
                    let l = x.as_str().and_then(parse_member).ok_or_else(|| err("list 须为 `类.方法:描述符`"))?;
                    if !l.desc.starts_with(&format!("({arr})L")) {
                        return Err(err(&format!("list 须只有一个 {arr} 形参、返回引用类型")));
                    }
                    Some(l)
                }
            };
            let mut lookups = HashMap::new();
            for (lk, sv) in e.get("lookups").and_then(|x| x.as_table()).ok_or_else(|| err("缺 lookups（查找入口 → 口径）"))? {
                let m = parse_member(lk).ok_or_else(|| err(&format!("lookups 键 {lk} 须为 `类.方法:描述符`")))?;
                let scope = match sv.as_str() {
                    Some("public") => LookupScope::Public,
                    Some("declared") => LookupScope::Declared,
                    Some("declared_public") => LookupScope::DeclaredPublic,
                    _ => return Err(err(&format!("lookups {lk} 的口径须为 \"public\" / \"declared\" / \"declared_public\""))),
                };
                let r = ret_of(&m.desc);
                let shape = if r == one {
                    LookupShape::One
                } else if r == arr {
                    LookupShape::Array
                } else if list.as_ref().is_some_and(|l| ret_of(&l.desc) == r) {
                    LookupShape::List
                } else {
                    return Err(err(&format!("lookups {lk} 的返回类型须为 {one}、{arr} 或 list 的返回类型")));
                };
                lookups.insert(lk.clone(), Lookup { scope, shape });
            }
            let native = match e.get("native") {
                None => None,
                Some(x) => {
                    let n = x.as_str().and_then(parse_member).ok_or_else(|| err("native 须为 `类.方法:描述符`"))?;
                    if n.desc != helper.desc {
                        return Err(err(&format!("native 描述符须同 helper（{}）", helper.desc)));
                    }
                    out.natives.insert(n.to_string());
                    Some(n)
                }
            };
            let mut copies = HashSet::new();
            for c in e.get("copies").and_then(|x| x.as_array()).map(|a| a.as_slice()).unwrap_or_default() {
                let s = c.as_str().ok_or_else(|| err("copies 须为字符串数组"))?;
                let m = parse_member(s).ok_or_else(|| err(&format!("copies {s} 须为 `类.方法:描述符`")))?;
                if ret_of(&m.desc) != one {
                    return Err(err(&format!("copies {s} 须返回 {one}")));
                }
                copies.insert(s.to_string());
            }
            for lk in lookups.keys() {
                out.lookup_of.insert(lk.clone(), k.clone());
            }
            for c in &copies {
                out.copy_of.insert(c.clone(), k.clone());
            }
            out.by_entry.insert(k.clone(), DirectInvoker { helper, recv, lookups, list, copies, native });
        }
        Ok(out)
    }

    pub fn get(&self, member: &str) -> Option<&DirectInvoker> {
        self.by_entry.get(member)
    }

    /// 查找入口 member 所属的直连入口与其口径 / 形态
    pub fn lookup(&self, member: &str) -> Option<(&DirectInvoker, Lookup)> {
        let d = self.by_entry.get(self.lookup_of.get(member)?)?;
        Some((d, *d.lookups.get(member)?))
    }

    /// 复制入口 member 所属的直连入口
    pub fn copy(&self, member: &str) -> Option<&DirectInvoker> {
        self.by_entry.get(self.copy_of.get(member)?)
    }

    /// member 是某入口 helper 体内执行目标调用的 native（返回值与实参由直连调用点逐目标建模）
    pub fn is_native(&self, member: &str) -> bool {
        self.natives.contains(member)
    }

    pub fn is_empty(&self) -> bool {
        self.by_entry.is_empty()
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
list = "p/M$D.list:([Lp/M;)Lp/L;"
lookups = { "p/C.get:(Ljava/lang/String;[Lp/C;)Lp/M;" = "public", "p/C.getD:(Ljava/lang/String;[Lp/C;)Lp/M;" = "declared", "p/C.all:()[Lp/M;" = "declared", "p/A.pub:(Lp/C;Ljava/lang/String;[Lp/C;)Lp/L;" = "declared_public" }
copies = ["p/F.copy:(Lp/M;)Lp/M;", "p/M.copy:()Lp/M;"]
native = "p/M$D.invoke0:(Lp/M;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;"
"#,
        )
        .unwrap();
        assert!(d.is_native("p/M$D.invoke0:(Lp/M;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;"));
        assert!(!d.is_native("p/M$D.invoke:(Lp/M;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;"));
        let e = d.get("p/M.invoke:(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;").unwrap();
        assert_eq!(e.helper.owner, "p/M$D");
        assert_eq!(e.helper.name, "invoke");
        assert_eq!(e.recv, "p/M");
        let one = |s| Lookup { scope: s, shape: LookupShape::One };
        assert_eq!(e.lookups.get("p/C.get:(Ljava/lang/String;[Lp/C;)Lp/M;"), Some(&one(LookupScope::Public)));
        assert_eq!(e.lookups.get("p/C.getD:(Ljava/lang/String;[Lp/C;)Lp/M;"), Some(&one(LookupScope::Declared)));
        let (_, all) = d.lookup("p/C.all:()[Lp/M;").unwrap();
        assert_eq!(all, Lookup { scope: LookupScope::Declared, shape: LookupShape::Array });
        let (inv, l) = d.lookup("p/A.pub:(Lp/C;Ljava/lang/String;[Lp/C;)Lp/L;").unwrap();
        assert_eq!(l, Lookup { scope: LookupScope::DeclaredPublic, shape: LookupShape::List });
        assert_eq!(inv.list.as_ref().unwrap().name, "list");
        assert!(d.copy("p/F.copy:(Lp/M;)Lp/M;").is_some());
        assert!(d.copy("p/M.copy:()Lp/M;").is_some());
        assert!(d.copy("p/M.other:()Lp/M;").is_none());
    }

    #[test]
    fn rejects_bad_shapes() {
        let bad = [
            // helper 形参不以反射类型开头
            r#"
[d."p/M.invoke:(Ljava/lang/Object;)Ljava/lang/Object;"]
helper = "p/M$D.invoke:(Ljava/lang/Object;)Ljava/lang/Object;"
lookups = {}
"#,
            // 未知口径
            r#"
[d."p/M.invoke:(Ljava/lang/Object;)Ljava/lang/Object;"]
helper = "p/M$D.invoke:(Lp/M;Ljava/lang/Object;)Ljava/lang/Object;"
lookups = { "p/C.get:(Ljava/lang/String;)Lp/M;" = "any" }
"#,
            // 列表形查找缺 list
            r#"
[d."p/M.invoke:(Ljava/lang/Object;)Ljava/lang/Object;"]
helper = "p/M$D.invoke:(Lp/M;Ljava/lang/Object;)Ljava/lang/Object;"
lookups = { "p/C.get:(Ljava/lang/String;)Lp/L;" = "declared" }
"#,
            // 复制入口返回类型不符
            r#"
[d."p/M.invoke:(Ljava/lang/Object;)Ljava/lang/Object;"]
helper = "p/M$D.invoke:(Lp/M;Ljava/lang/Object;)Ljava/lang/Object;"
lookups = {}
copies = ["p/F.copy:(Lp/M;)Ljava/lang/Object;"]
"#,
        ];
        for src in bad {
            assert!(parse(src).is_err(), "{src}");
        }
    }

    #[test]
    fn absent_section_is_empty() {
        assert!(DirectInvokers::from_toml(None).unwrap().is_empty());
    }
}
