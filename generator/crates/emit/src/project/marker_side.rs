//! 标记层落盘（S7 计划 §9.9 K6）：已拆层根模块类的标记文件 `<stem>_m.rs`。
//!
//! - 同一 `java_class!` 块在标记文件以 `#[rava_layer = "marker"]` 只展开与签名无关的部分
//!   （wrapper struct、身份 std impl、`__from_parts` 族、`From` 三族转换、描述符、alloc /
//!   offsets 钩子外部声明）——展开文本不含 `__VTable` / `__Methods`（宏侧守护）。
//! - **声明文件文本不变**：`decl` 层展开时由宏剥去标记部分，文件头 `use` 与块文本原样保留，
//!   D8 分段输入（图与体量）与本步之前逐字节一致 → 切段逐类不变。
//! - 标记文件与类声明文件**同目录同段**（同 crate）：struct 与固有 impl 的同 crate 约束仍在，
//!   各包 mod.rs 获得 `pub mod <标记模块>; pub use <标记模块>::*;` 行——底段由 `mod_tree`
//!   按磁盘扫描自动声明，上段由 [`super::decl_side`] 按本计划的类→模块名映射补行。
//!   K4 把手写基础设施（`__Handle` / `Object` 等）分层后，标记文件整体挪入 `java_marker`
//!   crate（届时只剩文件搬移与路径前缀）。
//! - 接口与不透明（L1）类不拆（与 [`super::layers`] 同一判定）；未拆层（块无 `decl` 属性）的
//!   类不发标记文件（完整展开含全部标记部分，无重复定义）。
//! - 文件名避让：同包 `Foo` 与 `FooM`（snake 同为 `foo` / `foo_m`）时 `foo_m.rs` 会撞
//!   `FooM` 的类文件——退 `foo__m.rs`（确定性：按类文件路径序取首个不冲突名）。

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use indexmap::IndexMap;

use super::fs::walk;
use crate::ctx::EmitCtx;
use crate::emission::ClassEmission;

const BLOCK_OPEN: &str = "rava_macros::java_class! {\n";
const DECL_ATTR: &str = "    #[rava_layer = \"decl\"]\n";
const MARKER_ATTR: &str = "    #[rava_layer = \"marker\"]\n";

/// 目录下已占用的模块名（类 / 手写文件 stem 与子包目录名）。`decl_src`：底段 crate 源码树
/// （手写 overlay 落同一相对路径，其文件名同样占用模块名）
fn reserved_names(
    ctx: &EmitCtx<'_>, ems: &IndexMap<String, ClassEmission>, decl_src: &Path,
) -> BTreeMap<PathBuf, BTreeSet<String>> {
    let mut reserved: BTreeMap<PathBuf, BTreeSet<String>> = BTreeMap::new();
    let add_file = |reserved: &mut BTreeMap<PathBuf, BTreeSet<String>>, p: &Path| {
        let Some(stem) = p.file_stem().map(|s| s.to_string_lossy().into_owned()) else { return };
        if let Some(dir) = p.parent() {
            reserved.entry(dir.to_path_buf()).or_default().insert(stem);
        }
    };
    for em in ems.values() {
        add_file(&mut reserved, &em.path);
        // 祖先目录名也是模块名（文件与目录同名 → E0761）
        for a in em.path.ancestors().skip(1) {
            if let Some(n) = a.file_name() {
                if let Some(dir) = a.parent() {
                    reserved.entry(dir.to_path_buf()).or_default()
                        .insert(n.to_string_lossy().into_owned());
                }
            }
        }
    }
    // 手写 overlay 真源（落底段同相对路径，文件在场即占用名；lib.rs / mod.rs 非模块名）
    let hw_src = ctx.runtime_src();
    for (dir, _, files) in walk(&hw_src) {
        let Ok(rel) = dir.strip_prefix(&hw_src) else { continue };
        let target_dir = decl_src.join(rel);
        for f in files.iter().filter(|f| f.ends_with(".rs") && **f != "lib.rs" && **f != "mod.rs") {
            add_file(&mut reserved, &target_dir.join(f.as_str()));
        }
    }
    reserved
}

/// 同目录不冲突的标记模块名：`<stem>_m`，被占用则 `<stem>__m`（再占用报错——命名面已穷尽）
fn free_marker_stem(reserved: &BTreeMap<PathBuf, BTreeSet<String>>, class: &Path) -> String {
    let stem = class.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    let dir = class.parent().map(Path::to_path_buf).unwrap_or_default();
    let taken = reserved.get(&dir);
    let clash = |n: &str| taken.is_some_and(|s| s.contains(n));
    if !clash(&format!("{stem}_m")) {
        format!("{stem}_m")
    } else if !clash(&format!("{stem}__m")) {
        format!("{stem}__m")
    } else {
        panic!("标记文件命名面穷尽：{class:?}（{stem}_m 与 {stem}__m 均被占用）");
    }
}

/// 类声明文本 → 标记文件文本：块首层属性换 `marker`，块后尾巴（`iface_upcasts!` 等，声明层项）
/// 不进标记文件。未拆层（无 `decl` 属性）返回 None。
pub fn marker_text(text: &str) -> Option<String> {
    let open = text.find(BLOCK_OPEN)?;
    let start = open + BLOCK_OPEN.len();
    // 块以第 0 列的 `}` 行结束（块内各项至少缩进 4 列），与 `layers::split_text` 同一判定
    let end = text[start..].find("\n}\n")?;
    let close = start + end + 3;
    let block = text[start..close].strip_prefix(DECL_ATTR)?;
    Some(format!("{}{BLOCK_OPEN}{}{}", &text[..open], MARKER_ATTR, block))
}

/// 标记层计划：标记文件与其宿主类（绝对路径 → 标记模块名；上层段 mod 行按它补声明）
#[derive(Debug, Default)]
pub struct MarkerPlan {
    /// 标记文件 (落盘路径, 文本)
    pub files: Vec<(PathBuf, String)>,
    /// 有标记文件的类文件绝对路径 → 标记模块名（`<stem>_m`，避让后可带双下划线）
    pub hosts: BTreeMap<PathBuf, String>,
}

impl MarkerPlan {
    pub fn total_bytes(&self) -> usize {
        self.files.iter().map(|(_, t)| t.len()).sum()
    }
}

/// 已拆层根模块类的标记文件划分。须在 D8 分段（[`super::decl_side::segment`]）之后调用
/// （分段输入是未剥标记部分的声明文本，切段与之前一致）。`decl_src`：底段 crate 源码树。
///
/// 同时改写宿主类的声明层文本：文件头（块前）插入 `pub use super::<标记模块>::<类名>;`
/// ——标记部分剥去后类名不在本模块，声明层展开（`impl X` / `X__VTable` / `X__Methods` 实现）
/// 与实现层文件的 `use <声明层>::<包>::<类模块>::*;` 都要它。该行在分段之后插入，
/// 不进 D8 图与体量；包级两 glob（`<类模块>::*` 与 `<标记模块>::*`）重复导出同一 `X`，
/// 同一定义不歧义。
pub fn split(
    ctx: &EmitCtx<'_>, ems: &mut IndexMap<String, ClassEmission>, decl_src: &Path,
) -> MarkerPlan {
    let crates = ctx.crates();
    let cands: Vec<usize> = ems
        .values()
        .enumerate()
        .filter(|(_, em)| em.crate_name == crates.root() && !em.handwritten)
        // 接口与不透明（L1）类不拆：与 `layers::split` 同一判定（接口载体本轮留接口文件）
        .filter(|(_, em)| {
            ctx.class(&em.binary_name).is_none_or(|ci| !ci.is_interface()) && !ctx.is_opaque(&em.binary_name)
        })
        .filter(|(_, em)| marker_text(&em.text).is_some())
        .map(|(i, _)| i)
        .collect();
    // 先登记全部候选与既有名，再取名（同包 Foo / FooM 的避让要看到对方）
    let mut reserved = reserved_names(ctx, ems, decl_src);
    let mut plan = MarkerPlan::default();
    for i in cands {
        let (path, bin) = {
            let em = &ems[i];
            (em.path.clone(), em.binary_name.clone())
        };
        let marker_stem = free_marker_stem(&reserved, &path);
        let file = path.with_file_name(format!("{marker_stem}.rs"));
        let text = marker_text(&ems[i].text).expect("候选已过滤");
        // 标记名自身也占用（两个类的标记名不互相冲突——stem 不同即名不同，此行只作保底）
        reserved.entry(file.parent().map(Path::to_path_buf).unwrap_or_default())
            .or_default()
            .insert(marker_stem.clone());
        plan.files.push((file, text));
        // 宿主声明文件头补类型再导出（见函数头注）
        let name = ctx.declared(&bin);
        let head_end = ems[i].text.find(BLOCK_OPEN).expect("候选已过滤");
        let head = ems[i].text[..head_end].trim_end_matches('\n').to_string();
        ems[i].text =
            format!("{head}\npub use super::{marker_stem}::{name};\n\n{}", &ems[i].text[head_end..]);
        plan.hosts.insert(path, marker_stem);
    }
    plan
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marker_text_swaps_layer_and_drops_tail() {
        let decl = "#![allow(x)]\nuse crate::prelude::*;\n\nrava_macros::java_class! {\n    \
                    #[rava_layer = \"decl\"]\n    #[binary_name = \"a/B\"]\n    pub struct B {}\n}\n\
                    // tail\nrava_macros::iface_upcasts! { impl B => C }\n";
        let m = marker_text(decl).unwrap();
        assert_eq!(
            m,
            "#![allow(x)]\nuse crate::prelude::*;\n\nrava_macros::java_class! {\n    \
             #[rava_layer = \"marker\"]\n    #[binary_name = \"a/B\"]\n    pub struct B {}\n}\n"
        );
        // 块后尾巴不进标记文件
        assert!(!m.contains("iface_upcasts"), "{m}");
    }

    #[test]
    fn unlayered_block_has_no_marker_file() {
        let full = "rava_macros::java_class! {\n    #[binary_name = \"a/B\"]\n    pub struct B {}\n}\n";
        assert!(marker_text(full).is_none());
        assert!(marker_text("fn f() {}").is_none());
    }

    #[test]
    fn marker_stem_avoids_class_name_collision() {
        let mut reserved: BTreeMap<PathBuf, BTreeSet<String>> = BTreeMap::new();
        let dir = PathBuf::from("/w/java/lang");
        // 同包既有 `foo_m.rs`（FooM 类）：foo 的标记名退 `foo__m`
        reserved.insert(dir.clone(), ["foo_m".to_string(), "bar".to_string()].into_iter().collect());
        assert_eq!(free_marker_stem(&reserved, &dir.join("foo.rs")), "foo__m");
        assert_eq!(free_marker_stem(&reserved, &dir.join("bar.rs")), "bar_m");
        // 未登记目录无冲突
        assert_eq!(free_marker_stem(&reserved, &dir.join("other/x.rs")), "x_m");
    }
}
