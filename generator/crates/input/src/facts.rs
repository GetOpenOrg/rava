//! 闭包事实：发射层消费的 [`closure::Closure`] 投影（`codegen/closure_input.py` 读取的形状）。
//!
//! 两个入口，产出同一结构：
//! - [`ClosureFacts::from_closure`]：终态路径，`rava build` 进程内直接读引擎结构；
//! - [`ClosureFacts::from_json`]：调试 / 审计入口，读 `rava closure -o closure.json`（计划
//!   docs/plans/2026-09-29-rust-closure-analyzer.md §4.2 格式）。

use std::collections::{BTreeMap, BTreeSet};

use classfile::MemberRef;
use closure::absint::V;
use closure::engine::{Kind, Level};
use closure::manifest::Domain;
use closure::Closure;
use serde_json::Value;

use crate::InputError;

/// closure.json 折叠点格式版本（与 [`closure::FOLDS_VERSION`] 同一约定）
pub const FOLDS_VERSION: u64 = closure::FOLDS_VERSION as u64;

/// 闭包内的一个类
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassFact {
    pub name: String,
    pub domain: Domain,
    pub level: Level,
}

/// 方法节点种类（`handwritten:<来源>` 保留来源标签）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodKind {
    Bytecode,
    Handwritten(String),
    Abstract,
    Missing,
}

impl MethodKind {
    fn parse(s: &str) -> Result<MethodKind, InputError> {
        Ok(match s {
            "bytecode" => MethodKind::Bytecode,
            "abstract" => MethodKind::Abstract,
            "missing" => MethodKind::Missing,
            _ => match s.strip_prefix("handwritten:") {
                Some(w) => MethodKind::Handwritten(w.to_string()),
                None => return Err(InputError::Format(format!("未知方法种类：{s}"))),
            },
        })
    }

    fn of(k: Kind) -> MethodKind {
        match k {
            Kind::Bytecode => MethodKind::Bytecode,
            Kind::Handwritten(w) => MethodKind::Handwritten(w.to_string()),
            Kind::Abstract => MethodKind::Abstract,
            Kind::Missing => MethodKind::Missing,
        }
    }

    /// 边界成员（`handwritten:boundary`）
    pub fn is_boundary(&self) -> bool {
        matches!(self, MethodKind::Handwritten(w) if w == "boundary")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodFact {
    pub id: MemberRef,
    pub kind: MethodKind,
}

/// 常量读取点的指令类别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadKind {
    GetField,
    GetStatic,
    Invoke,
}

/// 折叠常量值（folds 编码的语义形态；类型解释在应用时按描述符进行）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoldValue {
    Null,
    Bool(bool),
    Int(i64),
    Long(i64),
    Str(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoldConst {
    pub pc: u32,
    pub kind: ReadKind,
    pub value: FoldValue,
    /// 类型描述符
    pub ty: String,
}

/// 死 catch 表项：按异常表原值（区间、处理器、catch 类型）逐字段匹配
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeadCatch {
    pub start: u32,
    pub end: u32,
    pub handler: u32,
    pub catch_type: String,
}

/// 一个方法的折叠点
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MethodFold {
    /// 半开区间 [start, end)
    pub dead_pcs: Vec<(u32, u32)>,
    pub dead_handlers: BTreeSet<u32>,
    /// 逐项删除的异常表项（处理器仍活，只是这些 catch 类型不在闭包内）
    pub dead_catches: BTreeSet<DeadCatch>,
    /// pc → 常量读取点
    pub consts: BTreeMap<u32, FoldConst>,
}

/// 种子输出
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SeedFacts {
    pub annotation_enums: Vec<String>,
    /// 按类镜像强制初始化的目标类（Unsafe.ensureClassInitialized 等；需初始化钩子）
    pub mirror_inits: Vec<String>,
    pub reflect_names: BTreeMap<String, BTreeSet<String>>,
    pub reflect_all: BTreeSet<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ClosureFacts {
    /// 引擎插入序
    pub classes: Vec<ClassFact>,
    pub methods: Vec<MethodFact>,
    /// 类初始化集合（插入序）
    pub clinit: Vec<String>,
    /// 活代码调用点符号键
    pub refs: Vec<MemberRef>,
    pub missing: Vec<String>,
    pub unresolved: Vec<String>,
    /// 方法键（`类.方法:描述符`）→ 折叠点；折叠版本不认识时为空
    pub folds: BTreeMap<String, MethodFold>,
    /// 反射成员
    pub reflect_members: Vec<MemberRef>,
    pub reflect_gaps: Vec<String>,
    pub seeds: SeedFacts,
}

/// `owner.name:desc` → MemberRef（owner 含 `/`、`$`，名字不含 `.`）
pub fn parse_member_id(id: &str) -> Result<MemberRef, InputError> {
    let bad = || InputError::Format(format!("成员标识不合法：{id}"));
    let (head, desc) = id.split_once(':').ok_or_else(bad)?;
    let (owner, name) = head.rsplit_once('.').ok_or_else(bad)?;
    Ok(MemberRef {
        owner: owner.to_string(),
        name: name.to_string(),
        desc: desc.to_string(),
    })
}

fn fold_value_of(v: &V, ty: &str) -> FoldValue {
    match v {
        V::Int(i) if ty == "Z" => FoldValue::Bool(*i != 0),
        V::Int(i) => FoldValue::Int(i64::from(*i)),
        V::Long(l) => FoldValue::Long(*l),
        V::Str(s) => FoldValue::Str(s.to_string()),
        _ => FoldValue::Null,
    }
}

impl ClosureFacts {
    /// 终态入口：进程内读取闭包引擎
    pub fn from_closure(c: &Closure<'_>) -> ClosureFacts {
        let e = &c.engine;
        let classes = e
            .classes
            .iter()
            .map(|(n, c)| ClassFact {
                name: n.clone(),
                domain: c.domain,
                level: c.level,
            })
            .collect();
        let methods = e
            .method_nodes()
            .map(|m| MethodFact {
                id: m.key.clone(),
                kind: MethodKind::of(m.kind),
            })
            .collect();
        let mut folds = BTreeMap::new();
        for f in e.folds() {
            let consts = f
                .consts
                .iter()
                .map(|(pc, op, v, ty)| {
                    let kind = match *op {
                        classfile::op::GETFIELD => ReadKind::GetField,
                        classfile::op::GETSTATIC => ReadKind::GetStatic,
                        _ => ReadKind::Invoke,
                    };
                    let value = fold_value_of(v, ty);
                    (*pc, FoldConst { pc: *pc, kind, value, ty: ty.clone() })
                })
                .collect();
            let mf = MethodFold {
                dead_pcs: f.dead_pcs.clone(),
                dead_handlers: f.dead_handlers.iter().copied().collect(),
                dead_catches: f
                    .dead_catches
                    .iter()
                    .map(|c| DeadCatch { start: c.start, end: c.end, handler: c.handler, catch_type: c.catch_type.clone() })
                    .collect(),
                consts,
            };
            folds.insert(f.method.clone(), mf);
        }
        let s = &e.seeds;
        ClosureFacts {
            classes,
            methods,
            clinit: e.inited.keys().cloned().collect(),
            refs: e.refs.iter().filter_map(|r| parse_member_id(r).ok()).collect(),
            missing: e.missing.keys().cloned().collect(),
            unresolved: e.unresolved.iter().cloned().collect(),
            folds,
            reflect_members: e.reflect_members.iter().map(|(_, m)| m.clone()).collect(),
            reflect_gaps: e.reflect_gaps.iter().cloned().collect(),
            seeds: SeedFacts {
                annotation_enums: s.annotation_enums.iter().cloned().collect(),
                mirror_inits: s.mirror_inits.iter().cloned().collect(),
                reflect_names: s.reflect_names.clone(),
                reflect_all: s.reflect_all.clone(),
            },
        }
    }

    /// 调试入口：closure.json
    pub fn from_json(v: &Value) -> Result<ClosureFacts, InputError> {
        let mut out = ClosureFacts::default();
        for c in arr(v, "classes")? {
            out.classes.push(ClassFact {
                name: str_of(c, "name")?.to_string(),
                domain: parse_domain(str_of(c, "domain")?)?,
                level: parse_level(str_of(c, "level")?)?,
            });
        }
        for m in arr(v, "methods")? {
            out.methods.push(MethodFact {
                id: parse_member_id(str_of(m, "id")?)?,
                kind: MethodKind::parse(str_of(m, "kind")?)?,
            });
        }
        out.clinit = strings(v.get("clinit"))?;
        out.refs = strings(v.get("refs"))?.iter().map(|s| parse_member_id(s)).collect::<Result<_, _>>()?;
        for m in v.get("missing").and_then(Value::as_array).into_iter().flatten() {
            out.missing.push(str_of(m, "name")?.to_string());
        }
        out.unresolved = strings(v.get("unresolved"))?;
        if v.get("folds_version").and_then(Value::as_u64) == Some(FOLDS_VERSION) {
            for f in v.get("folds").and_then(Value::as_array).into_iter().flatten() {
                out.folds.insert(str_of(f, "method")?.to_string(), parse_fold(f)?);
            }
        }
        let reflect = v.get("reflect").ok_or_else(|| missing("reflect"))?;
        for r in arr(reflect, "members")? {
            out.reflect_members.push(parse_member_id(str_of(r, "member")?)?);
        }
        out.reflect_gaps = strings(reflect.get("gaps"))?;
        if let Some(s) = v.get("seeds") {
            out.seeds = parse_seeds(s)?;
        }
        Ok(out)
    }
}

fn missing(key: &str) -> InputError {
    InputError::Format(format!("closure.json 缺字段：{key}"))
}

fn arr<'a>(v: &'a Value, key: &str) -> Result<&'a Vec<Value>, InputError> {
    v.get(key).and_then(Value::as_array).ok_or_else(|| missing(key))
}

fn str_of<'a>(v: &'a Value, key: &str) -> Result<&'a str, InputError> {
    v.get(key).and_then(Value::as_str).ok_or_else(|| missing(key))
}

fn strings(v: Option<&Value>) -> Result<Vec<String>, InputError> {
    let Some(v) = v else { return Ok(Vec::new()) };
    let a = v.as_array().ok_or_else(|| InputError::Format(format!("应为字符串数组：{v}")))?;
    a.iter()
        .map(|x| x.as_str().map(str::to_string).ok_or_else(|| InputError::Format(format!("应为字符串：{x}"))))
        .collect()
}

fn u32_of(v: &Value) -> Result<u32, InputError> {
    v.as_u64()
        .and_then(|x| u32::try_from(x).ok())
        .ok_or_else(|| InputError::Format(format!("应为 u32：{v}")))
}

fn parse_domain(s: &str) -> Result<Domain, InputError> {
    Ok(match s {
        "user" => Domain::User,
        "translate" => Domain::Translate,
        "boundary" => Domain::Boundary,
        "root" => Domain::Root,
        _ => return Err(InputError::Format(format!("未知域：{s}"))),
    })
}

fn parse_level(s: &str) -> Result<Level, InputError> {
    Ok(match s {
        "type" => Level::Type,
        "init" => Level::Init,
        "alloc" => Level::Alloc,
        "code" => Level::Code,
        _ => return Err(InputError::Format(format!("未知层级：{s}"))),
    })
}

fn parse_fold_value(v: &Value, ty: &str) -> Result<FoldValue, InputError> {
    Ok(match v {
        Value::Null => FoldValue::Null,
        Value::Bool(b) => FoldValue::Bool(*b),
        Value::Number(n) if ty == "J" => FoldValue::Long(n.as_i64().ok_or_else(|| InputError::Format(format!("long 常量越界：{n}")))?),
        Value::Number(n) => FoldValue::Int(n.as_i64().ok_or_else(|| InputError::Format(format!("整数常量越界：{n}")))?),
        Value::String(s) if ty == "J" => {
            FoldValue::Long(s.parse().map_err(|_| InputError::Format(format!("long 常量不合法：{s}")))?)
        }
        Value::String(s) => FoldValue::Str(s.clone()),
        _ => return Err(InputError::Format(format!("不支持的常量编码：{v}"))),
    })
}

pub(crate) fn parse_fold(f: &Value) -> Result<MethodFold, InputError> {
    let mut mf = MethodFold::default();
    for r in f.get("dead_pcs").and_then(Value::as_array).into_iter().flatten() {
        let pair = r.as_array().filter(|p| p.len() == 2).ok_or_else(|| InputError::Format(format!("dead_pcs 项不合法：{r}")))?;
        mf.dead_pcs.push((u32_of(&pair[0])?, u32_of(&pair[1])?));
    }
    for h in f.get("dead_handlers").and_then(Value::as_array).into_iter().flatten() {
        mf.dead_handlers.insert(u32_of(h)?);
    }
    for c in f.get("dead_catches").and_then(Value::as_array).into_iter().flatten() {
        let at = |k: &str| u32_of(c.get(k).ok_or_else(|| missing(k))?);
        mf.dead_catches.insert(DeadCatch { start: at("start")?, end: at("end")?, handler: at("handler")?, catch_type: str_of(c, "catch_type")?.to_string() });
    }
    for c in f.get("consts").and_then(Value::as_array).into_iter().flatten() {
        let pc = u32_of(c.get("pc").ok_or_else(|| missing("pc"))?)?;
        let kind = match str_of(c, "kind")? {
            "getfield" => ReadKind::GetField,
            "getstatic" => ReadKind::GetStatic,
            "invoke" => ReadKind::Invoke,
            k => return Err(InputError::Format(format!("未知常量读取类别：{k}"))),
        };
        let ty = c.get("type").and_then(Value::as_str).unwrap_or("").to_string();
        let value = parse_fold_value(c.get("value").unwrap_or(&Value::Null), &ty)?;
        mf.consts.insert(pc, FoldConst { pc, kind, value, ty });
    }
    Ok(mf)
}

fn parse_seeds(s: &Value) -> Result<SeedFacts, InputError> {
    let mut out = SeedFacts {
        annotation_enums: strings(s.get("annotation_enums"))?,
        mirror_inits: strings(s.get("mirror_inits"))?,
        ..SeedFacts::default()
    };
    if let Some(m) = s.get("reflect_names").and_then(Value::as_object) {
        for (owner, names) in m {
            out.reflect_names.insert(owner.clone(), strings(Some(names))?.into_iter().collect());
        }
    }
    out.reflect_all = strings(s.get("reflect_all"))?.into_iter().collect();
    Ok(out)
}
