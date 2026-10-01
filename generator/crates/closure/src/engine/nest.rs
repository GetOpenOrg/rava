//! 引擎：嵌套伙伴访问检查（JVMS §5.4.4）加载的嵌套宿主。
//!
//! 字节码解析到**另一个类声明的 private 成员**（嵌套伙伴间的私有构造器 / 方法 / 字段）时，VM 的访问检查
//! 要比较双方的嵌套宿主：读取访问方与声明方的 `NestHost` 属性并加载所指类（不初始化）。宿主可能从未
//! 以其他方式被引用（如 `Outer$A` 调 `Outer$B` 的私有构造器，`Outer` 本身无可达成员），所以它是
//! 独立于成员属主的类引用边，按 `Level::Type` 登记。
//!
//! 同一类内的私有访问不做嵌套检查；无 `NestHost` 属性的类以自身为宿主。

use super::*;

/// 访问方 `caller`（宿主 `caller_host`）解析到声明于 `decl`（宿主 `decl_host`）、私有标志为 `private`
/// 的成员时，嵌套检查需要加载、且不是访问双方本身的宿主（去重，按访问方、声明方次序）
pub(super) fn nest_hosts<'s>(caller: &'s str, caller_host: Option<&'s str>, decl: &'s str, decl_host: Option<&'s str>, private: bool) -> Vec<&'s str> {
    if !private || caller == decl {
        return vec![];
    }
    let mut out: Vec<&str> = vec![];
    for h in [caller_host.unwrap_or(caller), decl_host.unwrap_or(decl)] {
        if h != caller && h != decl && !out.contains(&h) {
            out.push(h);
        }
    }
    out
}

impl<'a> Engine<'a> {
    /// 字节码方法 m 在 off 处解析到 `decl` 声明的成员（私有标志 `private`）：登记嵌套检查加载的宿主
    pub(super) fn nest_access(&mut self, m: usize, off: u32, decl: &str, private: bool) {
        if !private || self.methods[m].kind != Kind::Bytecode {
            return;
        }
        let caller = self.methods[m].key.owner.clone();
        if caller == decl {
            return;
        }
        let host_of = |c: &str| self.h.class(c).and_then(|cf| cf.nest_host.clone());
        let (ch, dh) = (host_of(&caller), host_of(decl));
        let hosts: Vec<String> = nest_hosts(&caller, ch.as_deref(), decl, dh.as_deref(), true).into_iter().map(str::to_string).collect();
        for h in hosts {
            self.touch(&h, Level::Type, Via::method("nest-host", m, Some(off)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::nest_hosts;

    #[test]
    fn private_access_between_nestmates_loads_host() {
        // 成员类互访私有构造器：宿主是第三个类
        assert_eq!(nest_hosts("O$A", Some("O"), "O$B", Some("O"), true), vec!["O"]);
        // 成员类访问宿主的私有成员（及反向）：宿主即访问一方，已登记
        assert!(nest_hosts("O$A", Some("O"), "O", None, true).is_empty());
        assert!(nest_hosts("O", None, "O$A", Some("O"), true).is_empty());
    }

    #[test]
    fn no_nest_check_for_non_private_or_same_class() {
        assert!(nest_hosts("O$A", Some("O"), "O$B", Some("O"), false).is_empty());
        assert!(nest_hosts("O$A", Some("O"), "O$A", Some("O"), true).is_empty());
    }

    #[test]
    fn distinct_hosts_are_both_loaded() {
        // 不同嵌套（访问检查会失败，但两边宿主都已加载）
        assert_eq!(nest_hosts("P$A", Some("P"), "Q$B", Some("Q"), true), vec!["P", "Q"]);
    }
}
