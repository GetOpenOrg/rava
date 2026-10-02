//! 引擎：手写体接收者静态类型（`SType`）的逐级解析。
//!
//! 推导链每一级解析为字段描述符：具名类型 → `L类;`；字段访问器 → 字段描述符；方法 / 关联函数调用 →
//! 返回描述符（Java 方法取协变返回中最具体者；类型上无此 Java 方法时依次查 static 字段访问器 `T::F()`
//! 与超类型共置手写 impl 块的同名 fn）；数组上的元素访问器 `get` → 元素描述符。

use super::*;

/// 推导链断开的原因
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum StypeBreak {
    /// 基底（具名类型 / 关联函数的宿主类型）不是 Java 类：附该类型路径
    Base(String),
    /// 中途某级的值不是 Java 对象（数组 / 基本类型上的 Rust 方法，手写 fn 返回 Rust 类型）：之后的调用是 Rust 语义
    Value(String),
    /// 推断缺口：基底与上一级都是 Java 类，本级在类型层次上查不到或无法唯一确定
    Gap(String),
}

/// 引用类型描述符 `L类;` 的类名
fn obj_class(d: &str) -> Option<&str> {
    d.strip_prefix('L')?.strip_suffix(';')
}

impl Engine<'_> {
    /// 手写体接收者静态类型 → 类名（引用类型；推不出为 None）
    pub(super) fn stype_class(&self, host: &str, s: &SType) -> Option<String> {
        let d = self.stype_desc(host, s).ok()?;
        obj_class(&d).map(str::to_string)
    }

    /// 手写体接收者静态类型 → 字段描述符；推不出给出断开原因
    pub(super) fn stype_desc(&self, host: &str, s: &SType) -> Result<String, StypeBreak> {
        let (base, m, path) = match s {
            SType::Named(t) => return self.resolve_tref(host, t).map(|c| format!("L{c};")).ok_or_else(|| StypeBreak::Base(t.0.join("::"))),
            SType::Ret(t, m) => match self.resolve_tref(host, t) {
                Some(c) => (format!("L{c};"), m, true),
                None => return self.module_fn_desc(host, t, m),
            },
            SType::Field(b, f) => {
                let d = self.stype_desc(host, b)?;
                let Some(c) = obj_class(&d) else { return Err(StypeBreak::Value(format!("{d} 上的 {f}"))) };
                return self.field_by_name(c, f).map(|(_, fd)| fd).ok_or_else(|| StypeBreak::Gap(format!("{c}.{f} 无此字段")));
            }
            SType::Call(b, m) => (self.stype_desc(host, b)?, m, false),
        };
        if let Some(elem) = base.strip_prefix('[') {
            // 数组值（`JArray`）的元素访问器
            return if m == "get" { Ok(elem.to_string()) } else { Err(StypeBreak::Value(format!("{base} 上的 {m}"))) };
        }
        let Some(c) = obj_class(&base) else { return Err(StypeBreak::Value(format!("{base} 上的 {m}"))) };
        self.member_ret(c, m, path)
    }

    /// 类型 `c` 上 Rust 名为 `m` 的成员的返回描述符：Java 方法（协变返回取最具体者）→ static 字段
    /// 访问器（仅路径调用）→ 超类型共置手写 impl 块中的同名 fn
    fn member_ret(&self, c: &str, m: &str, path: bool) -> Result<String, StypeBreak> {
        let rets: BTreeSet<String> = self
            .methods_by_rust_name(c, m, None)
            .into_iter()
            .filter_map(|(_, _, d, _)| parse_method(&d).and_then(|d| d.ret).map(|r| r.descriptor()))
            .collect();
        if !rets.is_empty() {
            let most = rets.iter().find(|r| {
                rets.iter().all(|o| o == *r || obj_class(r).zip(obj_class(o)).is_some_and(|(a, b)| self.h.is_subtype(a, b)))
            });
            return most.cloned().ok_or_else(|| StypeBreak::Gap(format!("{c}.{m} 返回不唯一 {rets:?}")));
        }
        if path {
            if let Some((_, d)) = self.static_field(c, m) {
                return Ok(d);
            }
        }
        match self.hw_fn_ret(c, m) {
            Some(Some(r)) => Ok(format!("L{r};")),
            Some(None) => Err(StypeBreak::Value(format!("{c}.{m} 手写返回非 Java 类型"))),
            None => Err(StypeBreak::Gap(format!("{c}.{m} 无此方法"))),
        }
    }

    /// 模块路径上的自由 fn（`super::thread_impl::f`）的返回描述符：按目标文件中 fn 的声明返回类型解析；
    /// 路径不是手写模块 / 无此 fn → 基底断开；返回类型不是 Java 类 → 值断开
    fn module_fn_desc(&self, host: &str, t: &TypeRef, m: &str) -> Result<String, StypeBreak> {
        let Some((at, r)) = self.hw.module_fn_ret(host, &t.0, m) else { return Err(StypeBreak::Base(t.0.join("::"))) };
        self.resolve_tref(&at, &TypeRef(r)).map(|c| format!("L{c};")).ok_or_else(|| StypeBreak::Value(format!("{}::{m} 手写返回非 Java 类型", t.0.join("::"))))
    }

    /// 超类型（自类在前）共置手写文件与模块单元中 `impl 该类型` 块里名为 `m` 的 fn：
    /// 外层 None = 无此 fn；内层 None = 返回类型不是 Java 类
    fn hw_fn_ret(&self, c: &str, m: &str) -> Option<Option<String>> {
        let units = self.hw.units();
        for cf in self.supertypes(c) {
            let own = self.hw.class(&cf.name);
            let hosts = std::iter::once((cf.name.as_str(), &own)).chain(units.iter().map(|(h, u)| (h.as_str(), u)));
            for (host, hw) in hosts {
                for ((st, f), r) in &hw.rets {
                    if f == m && !st.is_empty() && self.resolve_tref(host, &TypeRef(st.clone())).as_deref() == Some(cf.name.as_str()) {
                        return Some((!r.is_empty()).then(|| self.resolve_tref(host, &TypeRef(r.clone()))).flatten());
                    }
                }
            }
        }
        None
    }
}
