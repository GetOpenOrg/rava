//! 类名合法性：binary name 的各段要用作生成树的路径分量（包目录 / 文件名），在读入档案时校验一次。
//!
//! 规则：按 `/` 分段，每段非空、不是 `.` / `..`，且是合法的 Java 标识符（ASCII 部分按
//! `[A-Za-z_$][A-Za-z0-9_$]*`；非 ASCII 字符只排除空白与控制字符）；末段另允许 JLS 规定的
//! `package-info` / `module-info`。不合法的名字不会是 Java 源码编译出的类，也不能安全地落盘。

/// JLS 7.4.1.1 / 7.7 规定的非标识符类名（包注解载体 / 模块描述）
const SPECIAL_LAST: [&str; 2] = ["package-info", "module-info"];

fn id_start(c: char) -> bool {
    if c.is_ascii() { c.is_ascii_alphabetic() || c == '_' || c == '$' } else { !c.is_whitespace() && !c.is_control() }
}

fn id_part(c: char) -> bool {
    id_start(c) || c.is_ascii_digit()
}

/// 合法 Java 标识符（见模块注释的近似规则）
pub fn is_java_identifier(s: &str) -> bool {
    let mut cs = s.chars();
    cs.next().is_some_and(id_start) && cs.all(id_part)
}

/// binary name（`/` 分隔）的每段都可安全用作路径分量
pub fn is_valid_binary_name(bin: &str) -> bool {
    let segs: Vec<&str> = bin.split('/').collect();
    let last = segs.len() - 1;
    segs.iter()
        .enumerate()
        .all(|(i, s)| is_java_identifier(s) || (i == last && SPECIAL_LAST.contains(s)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_names() {
        for ok in ["A", "p/q/B", "p/B$1", "p/B$$Inner", "_x/$y", "p/package-info", "中文/类"] {
            assert!(is_valid_binary_name(ok), "{ok}");
        }
        for bad in ["", "/", "p//B", "./B", "../B", "p/../B", "p/.", "a-b/C", "p/1B", "p/B C", "p\\B", "p/B:c", "package-info/A"] {
            assert!(!is_valid_binary_name(bad), "{bad}");
        }
    }
}
