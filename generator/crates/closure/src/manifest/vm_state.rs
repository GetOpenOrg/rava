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

use std::collections::BTreeMap;

/// 字段访问钩子
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldHook {
    /// 钩子宿主类（binary name）
    pub host: String,
    /// 钩子 fn 名
    pub func: String,
    /// 接收者方法（实例字段，宿主即字段属主）
    pub receiver: bool,
    /// 接收者钩子对引导加载器定义的类镜像是空操作（字段恒为缺省值，如镜像的定义加载器）：闭包分析器对只含
    /// 引导类镜像的接收者值集不接钩子、读结果折叠为 null。缺省 false（每个镜像都须钩子落地，如镜像的模块）
    pub boot_noop: bool,
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
}

impl VmState {
    /// `vm` = vm_intrinsics.toml 全表
    pub fn from_toml(vm: &toml::Table) -> Result<Self, String> {
        let sec = vm.get("vm_state");
        let mut field_hooks = BTreeMap::new();
        for (k, v) in sec.and_then(|s| s.get("field_hooks")).and_then(|v| v.as_table()).into_iter().flatten() {
            let bad = || format!("vm_intrinsics.toml [vm_state.field_hooks]：{k} 须为 {{ hook = \"类.fn\", receiver = 布尔, boot_noop = 布尔 }}");
            let (owner, _) = k.split_once('.').filter(|(_, r)| r.contains(':')).ok_or_else(bad)?;
            let t = v.as_table().ok_or_else(bad)?;
            let (host, func) = t.get("hook").and_then(|h| h.as_str()).and_then(|h| h.rsplit_once('.')).ok_or_else(bad)?;
            let receiver = t.get("receiver").map(|r| r.as_bool().ok_or_else(bad)).transpose()?.unwrap_or(false);
            let boot_noop = t.get("boot_noop").map(|r| r.as_bool().ok_or_else(bad)).transpose()?.unwrap_or(false);
            if boot_noop && !receiver {
                return Err(format!("vm_intrinsics.toml [vm_state.field_hooks]：{k} 的 boot_noop 只用于接收者钩子"));
            }
            if receiver && host != owner {
                return Err(format!("vm_intrinsics.toml [vm_state.field_hooks]：{k} 的接收者钩子须定义在字段属主 {owner} 上"));
            }
            field_hooks.insert(k.clone(), FieldHook { host: host.to_string(), func: func.to_string(), receiver, boot_noop });
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
        Ok(VmState { field_hooks, loader_map })
    }

    /// 字段 `owner.name:desc` 的访问钩子
    pub fn field_hook(&self, owner: &str, name: &str, desc: &str) -> Option<&FieldHook> {
        if self.field_hooks.is_empty() {
            return None;
        }
        self.field_hooks.get(&format!("{owner}.{name}:{desc}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hooks_and_loader_map() {
        let t: toml::Table = r#"
[vm_state.field_hooks]
"a/B.f:La/C;" = { hook = "a/B.__vm_f", receiver = true, boot_noop = true }
"a/B.g:La/C;" = { hook = "a/B.__vm_g", receiver = true }
"a/D.s:La/C;" = { hook = "a/E.__vm_boot" }
[vm_state.loader_map]
class = "a/M"
boot = "b"
platform = "p"
"#
        .parse()
        .unwrap();
        let s = VmState::from_toml(&t).unwrap();
        assert_eq!(s.field_hook("a/B", "f", "La/C;").map(|h| (h.receiver, h.boot_noop)), Some((true, true)));
        assert_eq!(s.field_hook("a/B", "g", "La/C;").map(|h| (h.receiver, h.boot_noop)), Some((true, false)));
        let h = s.field_hook("a/D", "s", "La/C;").unwrap();
        assert_eq!((h.host.as_str(), h.func.as_str(), h.receiver), ("a/E", "__vm_boot", false));
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
