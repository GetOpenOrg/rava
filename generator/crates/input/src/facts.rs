//! 闭包事实：发射层消费的 [`closure::Closure`] 投影。
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
    /// 接收者恒为 null 的活虚调用点：执行即 NullPointerException，调用不翻译
    pub null_recv: BTreeSet<u32>,
    /// 定论不返回的活调用点：调用照常翻译，其后控制流终止
    pub noreturn_calls: BTreeSet<u32>,
    /// 把 noreturn_calls 与 null_recv 当作控制流终点时另外不可达的区间（与 dead_pcs 不相交）
    pub noreturn_dead_pcs: Vec<(u32, u32)>,
}

/// 种子输出
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SeedFacts {
    pub annotation_enums: Vec<String>,
    /// 按类镜像强制初始化的目标类（Unsafe.ensureClassInitialized 等；需初始化钩子）
    pub mirror_inits: Vec<String>,
    pub reflect_names: BTreeMap<String, BTreeSet<String>>,
    pub reflect_all: BTreeSet<String>,
    /// 模块服务表：(服务, provider) 二元组，只含命名模块里的 provider（类路径 provider 经
    /// META-INF/services 发现，不入引导服务目录），事实序（服务名序 → provider 声明序）
    pub module_services: Vec<(String, String)>,
}

/// VM 初始系统属性表（分析器折叠属性读点所用的清单表 `[facts.system_properties]`）：
/// 运行时 System.registerNatives 只写入这两类键，与折叠结论同源
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SysPropFacts {
    /// 启动时的常量属性（键 → 值）
    pub values: BTreeMap<String, String>,
    /// 运行期取宿主值的键
    pub dynamic: BTreeSet<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ClosureFacts {
    /// 按类名排序（与引擎处理次序无关）
    pub classes: Vec<ClassFact>,
    pub methods: Vec<MethodFact>,
    /// 类初始化集合（按类名排序）
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
    /// 按名查字段点到的字段（声明类, 名字）
    pub reflect_fields: Vec<(String, String)>,
    /// 按名查字段目标类推不出时的字面量名
    pub reflect_field_names: Vec<String>,
    /// 经字段枚举 / 静态字段句柄常量可按名读写的静态字段（声明类, 名字）
    pub reflect_static_fields: Vec<(String, String)>,
    /// 不经构造器分配的类（序列化构造器的分配目标）：L3 分派闭包的 `<alloc>` 臂
    pub reflect_allocations: Vec<String>,
    /// 需要方法 / 构造器表的类（元数据裁剪口径，含超类型）
    pub reflect_meta_methods: Vec<String>,
    /// 需要字段表的类（元数据裁剪口径，含超类型）
    pub reflect_meta_fields: Vec<String>,
    pub seeds: SeedFacts,
    /// 经虚分派到达的实现（全部活虚调用点目标之并 + VM 反射虚调用选中的实现）
    pub dispatched: Vec<MemberRef>,
    /// 已实例化的类（lambda / 手写实现对象 / 数组除外）
    pub instantiated: Vec<String>,
    /// 手写体继承成员需求：`owner` 为接收者静态类型，方法声明在其超类型上（L1 编译期事实）
    pub hw_inherited: Vec<MemberRef>,
    /// lambda 站点的函数式接口（samtype，分析器 `sam_types`）：发射层合成 `I__Lambda` 的接口集
    pub sam_types: Vec<String>,
    pub system_properties: SysPropFacts,
    /// 构建期引导映像（物化数据与活对象集；求值失败时为 None）
    pub boot_image: Option<closure::image::ImageData>,
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
        V::Str(s, _) => FoldValue::Str(s.to_string()),
        _ => FoldValue::Null,
    }
}

impl ClosureFacts {
    /// 终态入口：进程内读取闭包引擎
    pub fn from_closure(c: &Closure<'_>) -> ClosureFacts {
        let e = &c.engine;
        let classes = e
            .class_entries()
            .into_iter()
            .map(|(n, c)| ClassFact {
                name: n.clone(),
                domain: c.domain,
                level: c.level,
            })
            .collect();
        let methods = e
            .method_entries()
            .into_iter()
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
                null_recv: f.null_recv.iter().copied().collect(),
                noreturn_calls: f.noreturn_calls.iter().copied().collect(),
                noreturn_dead_pcs: f.noreturn_dead_pcs.clone(),
            };
            folds.insert(f.method.clone(), mf);
        }
        let s = &e.seeds;
        ClosureFacts {
            classes,
            methods,
            clinit: e.clinit_list().into_iter().cloned().collect(),
            refs: e.refs.iter().filter_map(|r| parse_member_id(r).ok()).collect(),
            missing: e.missing.keys().cloned().collect(),
            unresolved: e.unresolved.iter().cloned().collect(),
            folds,
            reflect_members: e.reflect_members.iter().map(|(_, m)| m.clone()).collect(),
            reflect_gaps: e.reflect_gaps.iter().cloned().collect(),
            reflect_fields: e.reflect_fields.iter().cloned().collect(),
            reflect_field_names: e.reflect_field_names.iter().cloned().collect(),
            reflect_static_fields: e.static_field_handles().into_iter().collect(),
            reflect_allocations: e.serial_allocs.iter().cloned().collect(),
            reflect_meta_methods: e.meta_method_classes().into_iter().collect(),
            reflect_meta_fields: e.meta_field_classes().into_iter().collect(),
            seeds: SeedFacts {
                annotation_enums: s.annotation_enums.iter().cloned().collect(),
                mirror_inits: s.mirror_inits.iter().cloned().collect(),
                reflect_names: s.reflect_names.clone(),
                reflect_all: s.reflect_all.clone(),
                module_services: s
                    .services
                    .selected
                    .iter()
                    .flat_map(|(svc, ps)| ps.iter().filter(|p| p.module.is_some()).map(move |p| (svc.clone(), p.class.clone())))
                    .collect(),
            },
            dispatched: e.dispatched().iter().filter_map(|d| parse_member_id(d).ok()).collect(),
            instantiated: e.instantiated(),
            hw_inherited: e.hw_inherited_requests().into_iter().collect(),
            sam_types: e.sam_types().into_iter().collect(),
            system_properties: SysPropFacts {
                values: e.sysprops().values().clone(),
                dynamic: e.sysprops().dynamic().clone(),
            },
            boot_image: c.boot_image.as_ref().and_then(|b| b.data.clone()),
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
        if let Some(b) = v.get("boot_image_data").filter(|b| !b.is_null()) {
            out.boot_image = Some(closure::image::ImageData::from_json(b).map_err(InputError::Format)?);
        }
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
        for r in arr(reflect, "fields")? {
            out.reflect_fields.push((str_of(r, "owner")?.to_string(), str_of(r, "name")?.to_string()));
        }
        out.reflect_field_names = strings(reflect.get("field_names"))?;
        for r in arr(reflect, "static_fields")? {
            out.reflect_static_fields.push((str_of(r, "owner")?.to_string(), str_of(r, "name")?.to_string()));
        }
        out.reflect_allocations = strings(reflect.get("allocations"))?;
        out.reflect_meta_methods = strings(reflect.get("meta_methods"))?;
        out.reflect_meta_fields = strings(reflect.get("meta_fields"))?;
        if let Some(s) = v.get("seeds") {
            out.seeds = parse_seeds(s)?;
        }
        out.dispatched = strings(v.get("dispatched"))?.iter().map(|s| parse_member_id(s)).collect::<Result<_, _>>()?;
        out.instantiated = strings(v.get("instantiated"))?;
        out.sam_types = strings(v.get("sam_types"))?;
        out.hw_inherited = strings(v.get("hw_inherited"))?.iter().map(|s| parse_member_id(s)).collect::<Result<_, _>>()?;
        let sp = v.get("system_properties").ok_or_else(|| missing("system_properties"))?;
        for (k, val) in sp.get("values").and_then(Value::as_object).into_iter().flatten() {
            let val = val.as_str().ok_or_else(|| InputError::Format(format!("system_properties.values.{k} 应为字符串")))?;
            out.system_properties.values.insert(k.clone(), val.to_string());
        }
        out.system_properties.dynamic = strings(sp.get("dynamic"))?.into_iter().collect();
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
        "layout" => Level::Layout,
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

fn parse_ranges(f: &Value, key: &str) -> Result<Vec<(u32, u32)>, InputError> {
    let mut out = Vec::new();
    for r in f.get(key).and_then(Value::as_array).into_iter().flatten() {
        let pair = r.as_array().filter(|p| p.len() == 2).ok_or_else(|| InputError::Format(format!("{key} 项不合法：{r}")))?;
        out.push((u32_of(&pair[0])?, u32_of(&pair[1])?));
    }
    Ok(out)
}

fn parse_pcs(f: &Value, key: &str) -> Result<BTreeSet<u32>, InputError> {
    f.get(key).and_then(Value::as_array).into_iter().flatten().map(u32_of).collect()
}

pub(crate) fn parse_fold(f: &Value) -> Result<MethodFold, InputError> {
    let mut mf = MethodFold {
        dead_pcs: parse_ranges(f, "dead_pcs")?,
        null_recv: parse_pcs(f, "null_recv")?,
        noreturn_calls: parse_pcs(f, "noreturn_calls")?,
        noreturn_dead_pcs: parse_ranges(f, "noreturn_dead_pcs")?,
        ..MethodFold::default()
    };
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
    for svc in s.get("services").and_then(Value::as_array).into_iter().flatten() {
        let service = str_of(svc, "service")?;
        for p in arr(svc, "providers")? {
            if p.get("module").is_some_and(|m| !m.is_null()) {
                out.module_services.push((service.to_string(), str_of(p, "class")?.to_string()));
            }
        }
    }
    Ok(out)
}
