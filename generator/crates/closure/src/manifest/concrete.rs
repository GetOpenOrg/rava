//! 具体求值（`vm_intrinsics.toml [concrete]`）：入口、native 白名单、内存缓存字段、VM 布局字段。
//!
//! 入口方法在调用点实参可枚举时按实参逐组具体执行（engine/concrete），轨迹上的方法入闭包、
//! 结果对象图物化为类型流；native 只认白名单（成员 → 操作名，操作的实现只按操作名分派）。

use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct ConcreteCfg {
    /// 入口方法（`类.方法:描述符`）
    pub entries: HashSet<String>,
    /// 手写承载方法的具体语义：成员 → 操作名（engine/concrete/natives.rs）
    pub natives: HashMap<String, String>,
    /// 内存缓存字段（`类.字段`）：首次求值写入、之后命中；轨迹按冷 / 热两次求值取并
    pub memo_fields: HashSet<String>,
    /// 发布后不再改写的类型（含子类型）：映像中这类对象的全部实例字段可读，经其字段取到的映像数组视为冻结。
    /// 依据是类的不可变契约（如正则模式及其节点图编译后只读），由清单逐类声明
    pub stable_types: Vec<String>,
    /// 隐式异常的类型：种类（`null` / `index` / `cast` / `arith` / `store` / `size`）→ 类
    pub implicit: HashMap<String, String>,
    /// VM 布局的字段（字符串字面量与类镜像由 VM 直接构造）：键为语义名（`string_value` / `string_coder` / `component_type`），值为 `类.字段`
    pub vm_fields: HashMap<String, String>,
    /// 构建期引导求值（`[concrete.boot]`，engine/concrete/boot.rs）
    pub boot: BootCfg,
}

/// 构建期引导求值：调用序列、引导期专用 native 操作、VM 注入静态值、VM / 平台属性。
/// 值 `@deferred` = 宿主相关（运行期取宿主值，构建期内容不可读），`@null` = null，
/// `@jdk_feature` = 参考 JDK 的特性版本号，其余为字面量
#[derive(Debug, Clone, Default)]
pub struct BootCfg {
    /// 调用序列：`[成员, 实参...]`（实参为整数字面量）
    pub calls: Vec<(String, Vec<i64>)>,
    /// 成员 → 操作名（优先于 `[concrete.natives]`）
    pub natives: HashMap<String, String>,
    /// VM 注入静态字段 `类.字段` → 整数字面量 / `@deferred`
    pub statics: HashMap<String, String>,
    /// VM 属性（`SystemProps$Raw.vmProperties` 的键值对，按序）
    pub vm_props: Vec<(String, String)>,
    /// 平台属性：`SystemProps$Raw` 下标常量 `_<名>_NDX` 的名 → 值（下标按参考 JDK 的类文件取）
    pub platform_props: BTreeMap<String, String>,
    /// VM 在调用序列前初始化的类（按序）
    pub init: Vec<String>,
    /// VM 构造的对象：`[类, 构造器描述符, 实参...]`，实参 `@k` = 第 k 个对象、`str:..` = 字符串；
    /// 先分配并登记，再执行构造器（初始线程在构造器执行前已是当前线程）
    pub objects: Vec<Vec<String>>,
    /// 当前线程取 objects 的下标
    pub current_thread: Option<usize>,
    /// 引导档位静态字段（`类.字段`，int）：残差步骤记录其构建期档位，运行期在该档位下重放
    pub level: Option<String>,
    /// 按档位定值的 VM 查询：成员 → 门限（档位 ≥ 门限为 true）。档位低于门限的残差步骤上下文中折叠为 false，
    /// 其余处沿用 `[facts.returns]`
    pub level_queries: HashMap<String, i64>,
}

fn strs(v: Option<&toml::Value>) -> Vec<String> {
    v.and_then(|v| v.as_array()).map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default()
}

fn table(v: Option<&toml::Value>, what: &str) -> Result<HashMap<String, String>, String> {
    let mut out = HashMap::new();
    for (k, v) in v.and_then(|v| v.as_table()).into_iter().flatten() {
        let Some(s) = v.as_str() else {
            return Err(format!("vm_intrinsics.toml [concrete.{what}]：{k} 的值须为字符串"));
        };
        out.insert(k.clone(), s.to_string());
    }
    Ok(out)
}

pub fn parse(t: Option<&toml::Value>) -> Result<ConcreteCfg, String> {
    let get = |k: &str| t.and_then(|t| t.get(k));
    Ok(ConcreteCfg {
        entries: strs(get("entries")).into_iter().collect(),
        natives: table(get("natives"), "natives")?,
        memo_fields: strs(get("memo_fields")).into_iter().collect(),
        stable_types: strs(get("stable_types")),
        vm_fields: table(get("vm_fields"), "vm_fields")?,
        implicit: table(get("implicit"), "implicit")?,
        boot: parse_boot(get("boot"))?,
    })
}

fn parse_boot(t: Option<&toml::Value>) -> Result<BootCfg, String> {
    let get = |k: &str| t.and_then(|t| t.get(k));
    let mut calls = Vec::new();
    for c in get("calls").and_then(|v| v.as_array()).into_iter().flatten() {
        let Some(a) = c.as_array() else { return Err("[concrete.boot] calls 的项须为数组".into()) };
        let m = a.first().and_then(|x| x.as_str()).ok_or("[concrete.boot] calls 项首元须为成员")?;
        let args = a[1..].iter().map(|x| x.as_integer().ok_or("[concrete.boot] calls 实参须为整数")).collect::<Result<Vec<_>, _>>()?;
        calls.push((m.to_string(), args));
    }
    let mut vm_props = Vec::new();
    for p in get("vm_props").and_then(|v| v.as_array()).into_iter().flatten() {
        let a = p.as_array().filter(|a| a.len() == 2).ok_or("[concrete.boot] vm_props 项须为 [键, 值]")?;
        vm_props.push((a[0].as_str().unwrap_or_default().to_string(), a[1].as_str().unwrap_or_default().to_string()));
    }
    Ok(BootCfg {
        calls,
        natives: table(get("natives"), "boot.natives")?,
        statics: table(get("statics"), "boot.statics")?,
        vm_props,
        platform_props: table(get("platform_props"), "boot.platform_props")?.into_iter().collect(),
        init: strs(get("init")),
        objects: get("objects").and_then(|v| v.as_array()).into_iter().flatten().map(|o| strs(Some(o))).collect(),
        current_thread: get("current_thread").and_then(|v| v.as_integer()).map(|x| x as usize),
        level: get("level").and_then(|v| v.as_str()).map(String::from),
        level_queries: get("level_queries")
            .and_then(|v| v.as_table())
            .into_iter()
            .flatten()
            .map(|(k, v)| v.as_integer().map(|x| (k.clone(), x)).ok_or_else(|| format!("[concrete.boot.level_queries] {k} 的值须为整数")))
            .collect::<Result<_, _>>()?,
    })
}
