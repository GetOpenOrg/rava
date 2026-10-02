//! 引擎：手写体的继承成员需求（L1 编译期事实）。
//!
//! 手写文件随其类（或作为模块单元）进入生成范围即整体编译。其中以「接收者.方法」写成、方法声明在接收者
//! 超类型上的调用点（`df.setParseIntegerOnly(…)`，`df: DecimalFormat`，方法声明于 `NumberFormat`）要求
//! 接收者类承载该继承成员——与生成方法体登记的继承成员需求是同一事实，且与该 fn 是否可达无关
//! （不可达的手写 fn 照样要通过类型检查）。按闭包内全部类的共置手写文件与全部模块单元逐 fn 收集。

use super::hw_stype::StypeBreak;
use super::*;
use std::collections::HashSet;

impl Engine<'_> {
    /// 手写体继承成员需求：`owner` 为接收者静态类型，`name` / `desc` 为声明在其超类型上的实例方法
    pub fn hw_inherited_requests(&self) -> BTreeSet<MemberRef> {
        let mut out = BTreeSet::new();
        for (host, _, c) in self.hw_recv_sites() {
            let Some(cls) = c.srecv.as_ref().and_then(|s| self.stype_class(&host, s)) else { continue };
            for (owner, name, desc, is_static) in self.methods_by_rust_name(&cls, &c.name, Some(c.args.len())) {
                if !is_static && owner != cls {
                    out.insert(MemberRef { owner: cls.clone(), name, desc });
                }
            }
        }
        out
    }

    /// 静态类型推不出的接收者调用点（审计）：`类别 宿主 方法名`（同宿主同名合并）。方法名须是闭包内某 Java
    /// 方法的名字，且接收者的静态类型与语法推断都解析不到 Java 类。类别：
    /// - `chain`：经字段 / 返回推导的链，基底与中途各级都是 Java 类、某级在类型层次上查不到或不唯一
    ///   （推断缺口，应归零），附断开的那一级；
    /// - `value`：链中途的值不是 Java 对象（数组 / 基本类型、手写 fn 返回的 Rust 类型），之后的同名调用
    ///   是 Rust 方法（`JArray::get`、`Iterator::map` …），不是 Java 回调；
    /// - `camel`：基底无类型、方法名为 Java 驼峰形（含大写；Rust 标准方法一律蛇形），附基底的类型路径（推不出为 `?`）；
    /// - `lower`：基底无类型、全小写单词名（get / map / set …），与 Rust 标准方法同名，语法上无法区分
    pub fn hw_untyped_sites(&self) -> BTreeSet<String> {
        let java_names: HashSet<String> =
            self.classes.keys().filter_map(|c| self.cp.get(c)).flat_map(|cf| cf.methods.iter().map(|m| m.name.clone()).collect::<Vec<_>>()).collect();
        let mut out = BTreeSet::new();
        for (host, _, c) in self.hw_recv_sites() {
            let plain = c.name.split('_').next().unwrap_or_default();
            if !java_names.contains(&c.name) && !java_names.contains(plain) {
                continue;
            }
            if c.recv.as_ref().and_then(|r| r.as_ref()).and_then(|t| self.resolve_tref(&host, t)).is_some() {
                continue;
            }
            let brk = match c.srecv.as_ref().map(|s| self.stype_desc(&host, s)) {
                Some(Ok(d)) if d.starts_with('L') => continue,
                Some(Ok(d)) => StypeBreak::Value(d),
                Some(Err(b)) => b,
                None => StypeBreak::Base(String::new()),
            };
            let line = match brk {
                StypeBreak::Gap(why) => format!("chain {host} {} ← {why}", c.name),
                StypeBreak::Value(_) => format!("value {host} {}", c.name),
                StypeBreak::Base(t) if c.name.chars().any(|ch| ch.is_ascii_uppercase()) => {
                    let t = if t.is_empty() { "?" } else { t.as_str() };
                    format!("camel {host} {} ← {t}", c.name)
                }
                StypeBreak::Base(_) => format!("lower {host} {}", c.name),
            };
            out.insert(line);
        }
        out
    }

    /// 闭包内全部共置手写文件与模块单元中写成「接收者.方法」的调用点：(宿主, fn 名, 调用点)；
    /// Rust trait 方法（clone 等）不计
    fn hw_recv_sites(&self) -> Vec<(String, String, TypedCall)> {
        let mut hosts: Vec<(String, Rc<ClassHw>)> = self.hw.units().iter().map(|(h, u)| (h.clone(), u.clone())).collect();
        for c in self.classes.keys() {
            let hw = self.hw.class(c);
            if !hw.files.is_empty() {
                hosts.push((c.clone(), hw));
            }
        }
        let mut out = Vec::new();
        for (host, hw) in &hosts {
            let fns = hw.fns.iter().chain(hw.objects.values().flat_map(|o| o.fns.iter()));
            for (name, f) in fns {
                for c in &f.calls {
                    if c.recv.is_some() && !hw_infer::RUST_TRAIT_METHODS.contains(&c.name.as_str()) {
                        out.push((host.clone(), name.clone(), c.clone()));
                    }
                }
            }
        }
        out
    }
}
