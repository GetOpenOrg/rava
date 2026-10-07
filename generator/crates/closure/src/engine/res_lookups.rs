//! 引擎：按名读取的资源（seeds.toml `[resource_lookups]`，配置见 `seeds/res_lookups.rs`）。
//!
//! 调用点由资源束的新可达方法扫描一并登记（`bundles.rs::named_scan`）。每个补种轮在调用点所在方法的当前
//! 分析里按名求资源名实参（`Gap::Class`：推不出的段记为任意串），展开后全为已知段的候选名即资源名——
//! 字段 / 拼接得出的名字（如 `"javax" + sep + ... + "rowset.properties"` 存入静态字段，另一支取系统属性）
//! 只取已知的那一支；系统属性等外部给出的名字不在闭包内，原生程序里只能读到嵌入的资源。
//! 类路径上存在的资源进 `named_resources`。字面量资源名另由生成器按调用链上的 ldc 推导，此处不重复要求。
//! 集合只增不减（外层不动点）。

use super::class_lookup::{event_at, expand, is_invoke, Gap};
use super::name_eval::Frame;
use super::sealed::flatten;
use super::*;
use crate::seeds::res_lookups::{self, Lookup};

#[derive(Default)]
pub struct ResLookupState {
    /// 读取调用点：(方法, 偏移, 入口)
    pub(super) sites: Vec<(usize, u32, Lookup)>,
    /// 入选的资源数
    found: usize,
}

impl<'a> Engine<'a> {
    /// 一轮按名读取的资源补种（调用点在 `seed_bundles` 的扫描里登记）
    pub(super) fn seed_res_lookups(&mut self) {
        let n0 = self.seeds.res_lookups.found;
        for (m, off, l) in self.seeds.res_lookups.sites.clone() {
            for name in self.res_names(m, off, l.arg) {
                for p in res_lookups::candidates(&self.methods[m].key.owner, &name, l.relative) {
                    if !self.seeds.named_resources.contains(&p) && self.cp.resource(&p).is_some() {
                        let k = &self.methods[m].key;
                        eprintln!("[closure] 按名读取的资源：{p}（{}.{}{}@{off}）", k.owner, k.name, k.desc);
                        self.seeds.named_resources.insert(p);
                        self.seeds.res_lookups.found += 1;
                    }
                }
            }
        }
        let s = &self.seeds.res_lookups;
        if s.found > n0 {
            eprintln!("[closure] 按名读取：{} 个调用点 → 资源 +{}（累计 {}）", s.sites.len(), s.found - n0, s.found);
        }
    }

    /// 调用点 m@off 第 arg 个实参（不含接收者）的已知资源名
    fn res_names(&mut self, m: usize, off: u32, arg: usize) -> Vec<String> {
        let Some(a) = self.methods[m].analysis.clone() else { return vec![] };
        if a.conservative {
            return vec![];
        }
        let Some(Event::Invoke { opcode, args, .. }) = event_at(&a, off, is_invoke) else { return vec![] };
        let Some(v) = args.get(usize::from(*opcode != classfile::op::INVOKESTATIC) + arg).cloned() else { return vec![] };
        let owner = self.methods[m].key.owner.clone();
        let f = Frame { m: Some(m), a: &a, owner: &owner, up: None };
        let prev = self.cur_site.replace((m, off));
        let parts = self.name_parts(&f, &v, Gap::Class, 0);
        self.cur_site = prev;
        let Some(pats) = parts.as_deref().and_then(expand) else { return vec![] };
        let mut out = Vec::new();
        for p in &pats {
            if let Some(names) = flatten(p) {
                out.extend(names.iter().map(|s| s.to_string()));
            }
        }
        out
    }
}
