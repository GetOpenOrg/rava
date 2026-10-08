//! 引擎：手写方法返回值的来源（计划 c1d §30 B3）。
//!
//! 手写体的返回点只有 null 与 Java 静态方法调用的返回值时（语法判定见 `handwritten/returns.rs`），
//! 返回值 = 这些被调方法返回值节点之并（null 不贡献对象），不再按 open(返回类型) 交出任意对象。
//! 被调方法的返回值节点汇合其全部返回值，是该调用点结果的上近似；只有结果不经返回值节点、按调用点
//! 建模的被调方（类镜像 / 浅拷贝 / 内存读取 / 新数组 / 调用者类）不能这样接，整体退回 open。

use super::*;

impl<'a> Engine<'a> {
    /// 手写成员返回值来源的被调方法（下标）；推不出（含命中 fn 为空、任一调用解析不出唯一的静态方法链）→ None，
    /// 由调用方退回 open(返回类型)。空表 = 恒返回 null
    pub(super) fn hw_ret_sources(&mut self, host: &str, mh: &MemberHw, via: &Via) -> Option<Vec<usize>> {
        if mh.fns.is_empty() {
            return None;
        }
        let calls = mh.ret.known()?.to_vec();
        let mut out = Vec::new();
        for c in &calls {
            let cls = self.resolve_tref(host, &c.path_ty)?;
            let hits: Vec<(String, String, String, bool)> = self.methods_by_rust_name(&cls, &c.name, Some(c.nargs));
            if hits.is_empty() || hits.iter().any(|h| !h.3) {
                return None;
            }
            for (_, name, desc, _) in hits {
                let site = self.h.resolve_method(&cls, &name, &desc, self.h.is_interface(&cls))?;
                let (o, nm, d) = site.key();
                let t = self.method(MemberRef { owner: o, name: nm, desc: d }, via.clone());
                if self.methods[t].ret_model != RetModel::Plain {
                    return None;
                }
                out.push(t);
            }
        }
        Some(out)
    }
}
