//! VM 钩子：类的共置手写文件里声明了回调边（`upcalls = "…"`）、却不对应该类任何 Java 成员的 pub fn。
//!
//! 它们不经 Java 调用点进入：生成代码 / rava_macros 在该类对象上直接调用（接口载体回落到代理调用等），
//! 运行期随该类对象一同存活。回调边只登记在钩子自身的属性上，成员匹配看不到它们——引擎在该类实例化时
//! 把钩子当作入口建模（见 `engine/vmhook.rs`）。

use super::{ClassHw, FnInfo, MemberHw};

impl ClassHw {
    /// 钩子 fn 名（排序）。`is_member` 判定 fn 名是否某个 Java 成员（含超类型继承成员）的 Rust 名
    pub fn vm_hooks(&self, is_member: impl Fn(&str) -> bool) -> Vec<String> {
        let mut v: Vec<String> =
            self.fns.iter().filter(|(n, f)| is_hook(f) && !is_member(n)).map(|(n, _)| n.clone()).collect();
        v.sort();
        v
    }

    /// 按 fn 名汇总手写体（钩子节点的手写体）
    pub fn fns_member(&self, fns: &[String]) -> MemberHw {
        let mut out = MemberHw::default();
        for n in fns {
            if let Some(f) = self.fns.get(n) {
                out.absorb(n, f);
            }
        }
        out
    }
}

fn is_hook(f: &FnInfo) -> bool {
    f.is_pub && !f.upcalls.is_empty()
}

#[cfg(test)]
mod tests {
    use super::super::scan::{close_transitive, scan_file};
    use super::super::{member_matches, ClassHw};
    use super::*;
    use std::collections::HashMap;

    /// 声明回调边的非成员 pub fn 才是钩子：成员 fn、无回调声明的 pub fn、私有 fn 都不是
    #[test]
    fn hooks_are_non_member_pub_fns_with_upcalls() {
        let src = r#"
            impl P {
                #[jvm_boundary(upcalls = "a/B.valueOf:(C)La/B;")]
                pub fn __vm_hook(&self, args: Vec<Object>) -> Result<Object> { helper() }
                #[jvm_native(upcalls = "a/B.valueOf:(C)La/B;")]
                pub fn dispatch_i(&self, x: i32) {}
                pub fn __vm_plain(&self) {}
                #[jvm_boundary(upcalls = "a/B.valueOf:(C)La/B;")]
                fn private_hook(&self) {}
            }
            fn helper() -> Result<Object> { todo() }
        "#;
        let file = syn::parse_file(src).expect("测试源码可解析");
        let mut raw = super::super::scan::FileFns::default();
        scan_file(&file, &HashMap::new(), &mut raw);
        close_transitive(&mut raw.fns, &raw.calls);
        let hw = ClassHw { fns: raw.fns, ..Default::default() };
        let members = ["dispatch", "<init>"];
        let hooks = hw.vm_hooks(|f| members.iter().any(|m| member_matches(f, m)));
        assert_eq!(hooks, vec!["__vm_hook".to_string()]);
        let mh = hw.fns_member(&hooks);
        assert_eq!(mh.upcalls.len(), 1);
        assert_eq!(mh.fns, hooks);
    }
}
