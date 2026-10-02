//! VM 钩子：类的共置手写文件里不对应该类任何 Java 成员的 pub fn。
//!
//! 它们不经 Java 调用点进入：生成代码 / rava_macros / 其他手写体在该类对象上直接调用（接口载体回落到
//! 代理调用等），运行期随该类对象一同存活。成员匹配看不到它们——引擎在该类实例化时把钩子当作入口建模
//!（见 `engine/vmhook.rs`），手写体调用点照常推断回调边。

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
    f.is_pub
}

#[cfg(test)]
mod tests {
    use super::super::scan::{close_transitive, scan_file};
    use super::super::{member_matches, ClassHw};
    use std::collections::HashMap;

    /// 非成员 pub fn 才是钩子：成员 fn、私有 fn 都不是
    #[test]
    fn hooks_are_non_member_pub_fns() {
        let src = r#"
            impl P {
                #[jvm_boundary]
                pub fn __vm_hook(&self, args: Vec<Object>) -> Result<Object> { helper() }
                #[jvm_native]
                pub fn dispatch_i(&self, x: i32) {}
                pub fn __vm_plain(&self) {}
                #[jvm_boundary]
                fn private_hook(&self) {}
            }
            fn helper() -> Result<Object> { todo() }
        "#;
        let file = syn::parse_file(src).expect("测试源码可解析");
        let mut raw = super::super::scan::FileFns::default();
        scan_file(&file, &HashMap::new(), &mut raw);
        close_transitive(&mut raw.fns, &raw.calls, &raw.nonself);
        let hw = ClassHw { fns: raw.fns, ..Default::default() };
        let members = ["dispatch", "<init>"];
        let hooks = hw.vm_hooks(|f| members.iter().any(|m| member_matches(f, m)));
        assert_eq!(hooks, vec!["__vm_hook".to_string(), "__vm_plain".to_string()]);
        let mh = hw.fns_member(&hooks);
        assert_eq!(mh.fns, hooks);
    }
}
