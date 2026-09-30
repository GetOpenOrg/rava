//! lib crate 发射的 Java 可见性映射（← `class_writer._java_member_vis` /
//! `_downgrade_non_public_blocks`）。
//!
//! public / protected → `pub`；package-private / private → `pub(crate)`（Java 包可见性在 Rust
//! 无对应，crate 内可见是合法超集）。protected 必须跨 crate 可见：下游 crate 的子类构造链调父类
//! protected `<init>`、覆盖模板方法都经此可见性。宏侧以 syn Visibility 透传 `pub(crate)`。

use std::sync::LazyLock;

use regex::Regex;

const ACC_PUBLIC: u16 = 0x0001;
const ACC_PROTECTED: u16 = 0x0004;

static BLOCK_ACCESS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\baccess = "(public|protected|private|package)""#).expect("access 正则"));
static BLOCK_PUB_LINE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^pub (fn|static|const) ").expect("pub 行正则"));

/// 成员 / 类的 Rust 可见性
pub fn java_member_vis(access: u16) -> &'static str {
    if access & (ACC_PUBLIC | ACC_PROTECTED) != 0 {
        "pub"
    } else {
        "pub(crate)"
    }
}

/// 块首属性行 access 为 private / package（或缺省）的成员块：行首 `pub fn|static|const` 整体降级为
/// `pub(crate)`；public / protected 保持。就地改写，属性值源自 classfile
pub fn downgrade_non_public_blocks(blocks: &mut [String]) {
    for b in blocks.iter_mut() {
        let head = b.split('\n').next().unwrap_or_default();
        if !head.contains("#[java_method(") && !head.contains("#[java_native(") && !head.contains("java_field(") {
            continue;
        }
        if BLOCK_ACCESS.captures(head).is_some_and(|c| matches!(&c[1], "public" | "protected")) {
            continue;
        }
        *b = BLOCK_PUB_LINE.replace_all(b, "pub(crate) $1 ").into_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vis_and_downgrade() {
        assert_eq!(java_member_vis(0x0001), "pub");
        assert_eq!(java_member_vis(0x0004), "pub");
        assert_eq!(java_member_vis(0x0002), "pub(crate)");
        assert_eq!(java_member_vis(0), "pub(crate)");
        let mut blocks = vec![
            "#[java_method(access = \"public\")]\npub fn a() {}".to_string(),
            "#[java_method(access = \"private\")]\npub fn b() {}\npub fn c() {}".to_string(),
            "#[java_field(name = \"x\")]\npub static X: i32 = 0;".to_string(),
            "// 其它块\npub fn d() {}".to_string(),
        ];
        downgrade_non_public_blocks(&mut blocks);
        assert_eq!(blocks[0], "#[java_method(access = \"public\")]\npub fn a() {}");
        assert_eq!(blocks[1], "#[java_method(access = \"private\")]\npub(crate) fn b() {}\npub(crate) fn c() {}");
        assert_eq!(blocks[2], "#[java_field(name = \"x\")]\npub(crate) static X: i32 = 0;");
        assert_eq!(blocks[3], "// 其它块\npub fn d() {}");
    }
}
