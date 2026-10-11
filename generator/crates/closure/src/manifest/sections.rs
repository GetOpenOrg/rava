//! 三清单的严格读取原语与只由发射层消费的节。
//!
//! 清单只在本模块（与 `manifest.rs` 及其子模块）解析一次；分析器与发射层（`input::manifest::RuntimeManifest`
//! 由 [`super::Manifest`] 投影得到）共用同一份读取与校验，不存在第二套解析。

/// invokedynamic 引导方法分类（vm_intrinsics.toml `[indy]`）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IndyKind {
    Native,
    Lambda,
    Concat,
    /// native 的细分：模式匹配 switch（静态实参即 case 标签序列）
    TypeSwitch,
    /// native 的细分：枚举选择子的模式 switch（标签为枚举常量名 / Class）
    EnumSwitch,
    /// native 的细分：record 的 equals / hashCode / toString（各引用分量派发同名 Object 方法）
    ObjectMethods,
}

impl IndyKind {
    /// 读取顺序：细分类别在 native 之后，同一引导方法同时列于 native 与细分类别时取细分
    pub const ALL: [IndyKind; 6] = [
        IndyKind::Native,
        IndyKind::Lambda,
        IndyKind::Concat,
        IndyKind::TypeSwitch,
        IndyKind::EnumSwitch,
        IndyKind::ObjectMethods,
    ];

    /// 清单键名（亦为 closure.json `indy_models` 的类别名）
    pub fn as_str(self) -> &'static str {
        match self {
            IndyKind::Native => "native",
            IndyKind::Lambda => "lambda",
            IndyKind::Concat => "concat",
            IndyKind::TypeSwitch => "type_switch",
            IndyKind::EnumSwitch => "enum_switch",
            IndyKind::ObjectMethods => "object_methods",
        }
    }

    /// native 及其细分：引导方法产出的调用点由手写 / 生成器另行承载，分析器按 native 建模
    pub fn is_native(self) -> bool {
        matches!(self, IndyKind::Native | IndyKind::TypeSwitch | IndyKind::EnumSwitch | IndyKind::ObjectMethods)
    }
}

/// VM 注入静态字段的取值（`[vm_constants.injected_statics]`）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InjectedValue {
    /// crate 根下的取值表达式（如 `vm_constants::page_size()`）
    Expr(String),
    Int(i64),
    Bool(bool),
}

impl InjectedValue {
    /// 字面量值（分析器按值折叠）；取值表达式不是字面量
    pub fn literal(&self) -> Option<i64> {
        match self {
            InjectedValue::Expr(_) => None,
            InjectedValue::Int(n) => Some(*n),
            InjectedValue::Bool(b) => Some(i64::from(*b)),
        }
    }
}

/// 取 `t[sec][key]`（`key` 可为空串表示节本身）
pub(super) fn get<'a>(t: &'a toml::Table, sec: &str, key: &str) -> Option<&'a toml::Value> {
    let s = t.get(sec)?;
    if key.is_empty() { Some(s) } else { s.get(key) }
}

/// 字符串数组：缺省为空；存在但不是字符串数组时报错（`file` / `where_` 用于定位）
pub(super) fn str_list(v: Option<&toml::Value>, where_: &str) -> Result<Vec<String>, String> {
    let Some(v) = v else { return Ok(Vec::new()) };
    let arr = v.as_array().ok_or_else(|| format!("{where_}：应为字符串数组"))?;
    arr.iter()
        .map(|x| x.as_str().map(str::to_string).ok_or_else(|| format!("{where_}：条目应为字符串：{x}")))
        .collect()
}

/// 逐类条目列表：同 [`str_list`]，且条目不得以 `/` 结尾（清单只收逐类条目）
pub(super) fn class_list(v: Option<&toml::Value>, where_: &str) -> Result<Vec<String>, String> {
    let out = str_list(v, where_)?;
    if let Some(bad) = out.iter().find(|p| p.ends_with('/')) {
        return Err(format!("{where_}：类条目不得以 / 结尾（清单只收逐类条目）：{bad}"));
    }
    Ok(out)
}

/// `[vm_constants.injected_statics]`：`"类.字段" = "取值表达式"` / 整数 / 布尔
pub(super) fn injected_statics(v: Option<&toml::Value>) -> Result<Vec<(String, InjectedValue)>, String> {
    let Some(v) = v else { return Ok(Vec::new()) };
    let t = v.as_table().ok_or("vm_intrinsics.toml [vm_constants.injected_statics]：应为表")?;
    t.iter()
        .map(|(k, v)| {
            let val = match v {
                toml::Value::String(e) if !e.is_empty() => Some(InjectedValue::Expr(e.clone())),
                toml::Value::Integer(n) => Some(InjectedValue::Int(*n)),
                toml::Value::Boolean(b) => Some(InjectedValue::Bool(*b)),
                _ => None,
            };
            match (k.split_once('.'), val) {
                (Some((c, f)), Some(val)) if !c.is_empty() && !f.is_empty() => Ok((k.clone(), val)),
                _ => Err(format!(
                    "vm_intrinsics.toml [vm_constants.injected_statics]：条目须为 \"类.字段\" = \"取值表达式\" / 字面量：{k}"
                )),
            }
        })
        .collect()
}

/// `[indy]`：引导方法（`类.方法`）→ 分类（细分类别覆盖 native）
pub(super) fn indy_kinds(vm: &toml::Table) -> Result<Vec<(String, IndyKind)>, String> {
    let mut out = Vec::new();
    for kind in IndyKind::ALL {
        for m in str_list(get(vm, "indy", kind.as_str()), &format!("vm_intrinsics.toml [indy] {}", kind.as_str()))? {
            out.push((m, kind));
        }
    }
    Ok(out)
}

/// 逃逸分析对 native / 手写方法的效果登记（vm_intrinsics.toml `[escape]`，无 GC 文档 §4.5）；未登记即不透明
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeEffect {
    /// 不存放、不保留实参；返回基本类型，或与实参无关的既有对象 / native 期间的（不登记的）分配
    Pure,
    /// 同 `Pure`，但返回接收者本身
    ReturnsReceiver,
    /// 把第 1 个实参数组的元素复制进第 3 个实参数组（`arraycopy` 形态）
    ArrayCopy,
    /// 同 `Pure`，但返回 native 期间新建、内容为空的对象（不登记，可被区域对象引用）
    Fresh,
}

/// `[escape]`：`类.方法:描述符` → 效果
fn escape_effects(vm: &toml::Table) -> Result<std::collections::BTreeMap<String, EscapeEffect>, String> {
    let mut out = std::collections::BTreeMap::new();
    for (key, eff) in [
        ("pure", EscapeEffect::Pure),
        ("returns_receiver", EscapeEffect::ReturnsReceiver),
        ("array_copy", EscapeEffect::ArrayCopy),
        ("fresh", EscapeEffect::Fresh),
    ] {
        let where_ = format!("vm_intrinsics.toml [escape] {key}");
        for m in str_list(get(vm, "escape", key), &where_)? {
            let well_formed = m
                .split_once(':')
                .is_some_and(|(head, desc)| desc.starts_with('(') && head.rsplit_once('.').is_some());
            if !well_formed {
                return Err(format!("{where_}：应为 `类.方法:描述符`：{m}"));
            }
            out.insert(m, eff);
        }
    }
    Ok(out)
}

/// 只由发射层消费的节（分析器不读，随 [`super::Manifest`] 一并解析与校验）
#[derive(Debug, Clone, Default)]
pub struct EmitSections {
    /// seeds.toml `[module_resources] paths`：模块资源路径（jmod `classes/` 下相对路径）
    pub module_resource_paths: Vec<String>,
    /// vm_intrinsics.toml `[class_path] resource_readers`（`类.方法:描述符`）：其一在调用链上即嵌入类路径资源
    pub class_path_readers: Vec<String>,
    /// vm_intrinsics.toml `[sigpoly] callsite_typed`
    pub sigpoly_callsite_typed: Vec<String>,
    /// vm_intrinsics.toml `[vm_constants] null_returns`
    pub null_returns: Vec<String>,
    /// vm_intrinsics.toml `[escape]`：逃逸分析效果登记
    pub escape_effects: std::collections::BTreeMap<String, EscapeEffect>,
}

impl EmitSections {
    pub(super) fn parse(seeds: &toml::Table, vm: &toml::Table) -> Result<Self, String> {
        Ok(EmitSections {
            module_resource_paths: str_list(get(seeds, "module_resources", "paths"), "seeds.toml [module_resources] paths")?,
            class_path_readers: str_list(get(vm, "class_path", "resource_readers"), "vm_intrinsics.toml [class_path] resource_readers")?,
            sigpoly_callsite_typed: str_list(get(vm, "sigpoly", "callsite_typed"), "vm_intrinsics.toml [sigpoly] callsite_typed")?,
            null_returns: str_list(get(vm, "vm_constants", "null_returns"), "vm_intrinsics.toml [vm_constants] null_returns")?,
            escape_effects: escape_effects(vm)?,
        })
    }
}

/// 发射层投影所需的只读视图（字段在 `manifest.rs` 私有；子模块可见）
impl super::Manifest {
    /// `[vm_boundary] classes`
    pub fn vm_boundary_classes(&self) -> impl Iterator<Item = &str> {
        self.vm_boundary.iter().map(String::as_str)
    }

    /// `[vm_boundary] clinit_carried`
    pub fn vm_clinit_carried_classes(&self) -> impl Iterator<Item = &str> {
        self.vm_clinit_carried.iter().map(String::as_str)
    }

    /// 放行条目：`[vm_boundary] translate_nested` 与分析期追加的条目
    pub fn release_entries(&self) -> &[String] {
        &self.release
    }

    /// `[vm_constants.injected_statics]`：`类.字段` → 取值
    pub fn injected_statics(&self) -> impl Iterator<Item = (&str, &InjectedValue)> {
        self.injected_statics.iter().map(|(k, v)| (k.as_str(), v))
    }

    /// `[[intrinsic]] member`
    pub fn intrinsic_members(&self) -> impl Iterator<Item = &str> {
        self.intrinsics.iter().map(String::as_str)
    }

    /// `[caller_sensitive] annotations`
    pub fn caller_sensitive_annotations(&self) -> impl Iterator<Item = &str> {
        self.caller_sensitive.iter().map(String::as_str)
    }

    /// `[vm_constants] null_to_false`
    pub fn null_to_false_members(&self) -> impl Iterator<Item = &str> {
        self.null_to_false.iter().map(String::as_str)
    }

    /// `[indy]`：引导方法（`类.方法`）→ 分类
    pub fn indy_kinds(&self) -> impl Iterator<Item = (&str, IndyKind)> {
        self.indy.iter().map(|(k, v)| (k.as_str(), *v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(s: &str) -> toml::Table {
        s.parse().unwrap()
    }

    #[test]
    fn lists_are_strict() {
        let t = table("[a]\nok = [\"x\"]\nbad = [1]\nnot_arr = \"x\"\ncls = [\"p/\"]\n");
        assert_eq!(str_list(get(&t, "a", "ok"), "a.ok").unwrap(), ["x"]);
        assert!(str_list(get(&t, "a", "missing"), "a.missing").unwrap().is_empty());
        assert!(str_list(get(&t, "a", "bad"), "a.bad").is_err());
        assert!(str_list(get(&t, "a", "not_arr"), "a.not_arr").is_err());
        assert!(class_list(get(&t, "a", "cls"), "a.cls").is_err());
    }

    #[test]
    fn indy_subkinds_override_native() {
        let t = table("[indy]\nnative = [\"a/S.t\", \"a/N.b\"]\ntype_switch = [\"a/S.t\"]\n");
        let m: std::collections::BTreeMap<_, _> = indy_kinds(&t).unwrap().into_iter().collect();
        assert_eq!(m["a/S.t"], IndyKind::TypeSwitch);
        assert_eq!(m["a/N.b"], IndyKind::Native);
        assert!(IndyKind::TypeSwitch.is_native() && !IndyKind::Lambda.is_native());
    }

    #[test]
    fn injected_statics_validated() {
        let t = table("[s]\n\"a/B.F\" = \"vm::f()\"\n\"a/B.G\" = 3\n\"a/B.H\" = true\n");
        let v = injected_statics(get(&t, "s", "")).unwrap();
        assert_eq!(v[0].1, InjectedValue::Expr("vm::f()".into()));
        assert_eq!(v[1].1.literal(), Some(3));
        assert_eq!(v[2].1.literal(), Some(1));
        assert!(injected_statics(get(&table("[s]\nnodot = 1\n"), "s", "")).is_err());
        assert!(injected_statics(get(&table("[s]\n\"a.b\" = \"\"\n"), "s", "")).is_err());
    }
}
