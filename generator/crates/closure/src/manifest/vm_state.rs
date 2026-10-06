//! VM 注入状态的落地清单（vm_intrinsics.toml `[vm_state]`）。
//!
//! - `field_hooks`：字段 → 访问（读 / 写）前调用的手写落地函数。HotSpot 在引导或建镜像时写入、原生二进制
//!   改为首次访问时落地的字段（类镜像的定义加载器、initPhase3 写入的系统类加载器等）。
//!   - `receiver = true`：实例字段，钩子是字段属主上的接收者方法 `fn(&self) -> Result<&Self>`，
//!     发射为 `recv.__nn()?.hook()?.__get_x()`；
//!   - 否则钩子是宿主类的静态函数 `fn() -> Result<()>`，发射为访问前的一条语句。
//!   闭包分析器把每个访问点连到钩子的手写体。
//! - `loader_map`：JDK 模块 → 内建加载器的映射所在（类 `<clinit>` 里按静态字段分组的字符串常量集合），
//!   生成器据此给每个类定出定义加载器（`loaders.rs`）。
//! - `boot_singletons`：类镜像上的实例方法（`类.方法:描述符`），接收者所指类由引导加载器定义时运行时恒返回同一个
//!   进程内对象（手写承载的 VM 状态，如引导类共用的模块单例）。闭包分析器据此给结果带身份标签，两个这样的结果
//!   引用相等（`if_acmp`）按标签折叠（`absint::obj::Obj::BootSingleton`）。

use std::collections::{BTreeMap, BTreeSet};

/// 字段访问钩子
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldHook {
    /// 钩子宿主类（binary name）
    pub host: String,
    /// 钩子 fn 名
    pub func: String,
    /// 接收者方法（实例字段，宿主即字段属主）
    pub receiver: bool,
}

/// 模块 → 加载器映射的来源：`class` 的 `<clinit>` 写入 `boot` / `platform` 两个静态字段的字符串常量集合
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LoaderMapSrc {
    pub class: String,
    pub boot: String,
    pub platform: String,
}

#[derive(Debug, Clone, Default)]
pub struct VmState {
    /// `属主.字段名:描述符` → 钩子
    pub field_hooks: BTreeMap<String, FieldHook>,
    pub loader_map: Option<LoaderMapSrc>,
    /// `类.方法:描述符`：接收者为引导类镜像时恒返回同一对象
    pub boot_singletons: BTreeSet<String>,
}

impl VmState {
    /// `vm` = vm_intrinsics.toml 全表
    pub fn from_toml(vm: &toml::Table) -> Result<Self, String> {
        let sec = vm.get("vm_state");
        let mut field_hooks = BTreeMap::new();
        for (k, v) in sec.and_then(|s| s.get("field_hooks")).and_then(|v| v.as_table()).into_iter().flatten() {
            let bad = || format!("vm_intrinsics.toml [vm_state.field_hooks]：{k} 须为 {{ hook = \"类.fn\", receiver = 布尔 }}");
            let (owner, _) = k.split_once('.').filter(|(_, r)| r.contains(':')).ok_or_else(bad)?;
            let t = v.as_table().ok_or_else(bad)?;
            let (host, func) = t.get("hook").and_then(|h| h.as_str()).and_then(|h| h.rsplit_once('.')).ok_or_else(bad)?;
            let receiver = t.get("receiver").map(|r| r.as_bool().ok_or_else(bad)).transpose()?.unwrap_or(false);
            if receiver && host != owner {
                return Err(format!("vm_intrinsics.toml [vm_state.field_hooks]：{k} 的接收者钩子须定义在字段属主 {owner} 上"));
            }
            field_hooks.insert(k.clone(), FieldHook { host: host.to_string(), func: func.to_string(), receiver });
        }
        let loader_map = match sec.and_then(|s| s.get("loader_map")) {
            None => None,
            Some(v) => {
                let get = |key: &str| v.get(key).and_then(|x| x.as_str()).map(str::to_string);
                match (get("class"), get("boot"), get("platform")) {
                    (Some(class), Some(boot), Some(platform)) => Some(LoaderMapSrc { class, boot, platform }),
                    _ => return Err("vm_intrinsics.toml [vm_state.loader_map]：须写 class / boot / platform".into()),
                }
            }
        };
        let mut boot_singletons = BTreeSet::new();
        for v in sec.and_then(|s| s.get("boot_singletons")).into_iter() {
            let bad = || "vm_intrinsics.toml [vm_state] boot_singletons 须为 \"类.方法:描述符\" 数组".to_string();
            for x in v.as_array().ok_or_else(bad)? {
                let k = x.as_str().filter(|k| k.split_once('.').is_some_and(|(_, r)| r.contains(':'))).ok_or_else(bad)?;
                boot_singletons.insert(k.to_string());
            }
        }
        Ok(VmState { field_hooks, loader_map, boot_singletons })
    }

    /// 字段 `owner.name:desc` 的访问钩子
    pub fn field_hook(&self, owner: &str, name: &str, desc: &str) -> Option<&FieldHook> {
        if self.field_hooks.is_empty() {
            return None;
        }
        self.field_hooks.get(&format!("{owner}.{name}:{desc}"))
    }

    /// 方法（`类.方法:描述符`）是否在接收者为引导类镜像时恒返回同一对象
    pub fn is_boot_singleton(&self, member: &str) -> bool {
        self.boot_singletons.contains(member)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hooks_and_loader_map() {
        let t: toml::Table = r#"
[vm_state]
boot_singletons = ["a/K.m:()La/M;"]
[vm_state.field_hooks]
"a/B.f:La/C;" = { hook = "a/B.__vm_f", receiver = true }
"a/D.s:La/C;" = { hook = "a/E.__vm_boot" }
[vm_state.loader_map]
class = "a/M"
boot = "b"
platform = "p"
"#
        .parse()
        .unwrap();
        let s = VmState::from_toml(&t).unwrap();
        assert_eq!(s.field_hook("a/B", "f", "La/C;").unwrap().receiver, true);
        let h = s.field_hook("a/D", "s", "La/C;").unwrap();
        assert_eq!((h.host.as_str(), h.func.as_str(), h.receiver), ("a/E", "__vm_boot", false));
        assert!(s.is_boot_singleton("a/K.m:()La/M;") && !s.is_boot_singleton("a/K.n:()La/M;"));
        assert_eq!(s.loader_map.unwrap().platform, "p");
    }

    #[test]
    fn receiver_hook_must_live_on_owner() {
        let t: toml::Table = r#"
[vm_state.field_hooks]
"a/B.f:La/C;" = { hook = "a/X.__vm_f", receiver = true }
"#
        .parse()
        .unwrap();
        assert!(VmState::from_toml(&t).is_err());
    }
}
