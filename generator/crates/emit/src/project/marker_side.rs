//! 标记层落盘（S7 计划 §9.9 K6 / K4 阶段 3）：已拆层根模块类的标记文件入 `java_marker` crate。
//!
//! - 同一 `java_class!` 块在标记文件以 `#[rava_layer = "marker"]` 只展开与签名无关的部分
//!   （wrapper struct、身份 std impl、`__from_parts` 族、`From` 三族转换、描述符、alloc /
//!   offsets 钩子外部声明）——展开文本不含 `__VTable` / `__Methods`（宏侧守护）。
//! - **声明文件块文本不变**：`decl` 层展开时由宏剥去标记部分，文件头 `use` 与块文本原样保留，
//!   D8 分段输入（图与体量）与本步之前逐字节一致 → 切段逐类不变。接口（§9.9 K4 阶段 3 起）
//!   同样拆层：宿主声明文件块首补 `#[rava_layer = "decl"]`（分段之后插入，不进 D8 输入），
//!   标记文件承载载体身份面（`expand_interface` 的 marker 段，宏侧随阶段 2 就绪）。
//! - 标记文件落 `<scratch>/java_marker/src/<包>/<stem>_m.rs`（K4 阶段 3 起独立 crate，
//!   此前与声明层同目录同段）：全部标记文件单 crate（§9.8.2 探针：语料量级约 1 GB，非约束）；
//!   java_marker 的包 mod.rs / lib.rs 由 `mod_tree` / `complete_lib_rs` 按磁盘生成（与声明
//!   层 crate 同口径，见 [`super::project::write_project`]）。
//! - 宿主声明文件（类与接口）文件头补 `pub use java_marker::<包>::<标记模块>::<类名>;`
//!   ——标记部分剥去后类名不在本模块，声明层展开（`impl X` / `X__VTable` / `X__Methods`
//!   实现）与实现层文件的 `use <声明层>::<包>::<类模块>::*;` 都要它。该行在分段之后插入，
//!   不进 D8 图与体量。
//! - 不透明（L1）类不拆层（与 [`super::layers`] 同一判定），其文件是纯标记面
//!   （`java_class_opaque!`），但标记文件按名引用它们（字段类型 / 祖先）——对每份不透明
//!   类文件在 java_marker 同相对路径**双发**一份逐字节拷贝：声明层 crate 的原文件不动
//!   （D8 输入不变 → 切段不漂移），两份是不同 crate 里的同名类型，各自只在本 crate 内
//!   消费（布局同源于同一宏展开，DESC 的单元协议按位搬运，见 S7 计划 §9.9 阶段 3 注记）。
//! - 未拆层（块无 `decl` 属性）的类不发标记文件（完整展开含全部标记部分，无重复定义）。
//! - 文件名避让：同包 `Foo` 与 `FooM`（snake 同为 `foo` / `foo_m`）时 `foo_m.rs` 会撞
//!   `FooM` 的标记文件 `foo_m_m.rs`？不会（stem 不同名不同）；真正要避让的是 java_marker
//!   手写真源的同名文件（`<stem>_m.rs` 形态的理论占位）——冲突退 `<stem>__m.rs`（确定性）。

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use indexmap::IndexMap;

use super::fs::walk;
use crate::ctx::EmitCtx;
use crate::emission::ClassEmission;

const BLOCK_OPEN: &str = "rava_macros::java_class! {\n";
const DECL_ATTR: &str = "    #[rava_layer = \"decl\"]\n";
const MARKER_ATTR: &str = "    #[rava_layer = \"marker\"]\n";

/// 目录下已占用的模块名（手写文件 stem 与子包目录名）：java_marker 真源（标记文件与
/// 不透明拷贝的落点，overlay 镜像落同一相对路径）
fn reserved_names(ctx: &EmitCtx<'_>, decl_src: &Path) -> BTreeMap<PathBuf, BTreeSet<String>> {
    let mut reserved: BTreeMap<PathBuf, BTreeSet<String>> = BTreeMap::new();
    let add_file = |reserved: &mut BTreeMap<PathBuf, BTreeSet<String>>, p: &Path| {
        let Some(stem) = p.file_stem().map(|s| s.to_string_lossy().into_owned()) else { return };
        if let Some(dir) = p.parent() {
            reserved.entry(dir.to_path_buf()).or_default().insert(stem);
        }
    };
    // java_marker 手写真源（文件在场即占用名；lib.rs / mod.rs 非模块名）
    let Some(marker_src) = marker_src(ctx) else { return reserved };
    for (dir, _, files) in walk(&marker_src) {
        let Ok(rel) = dir.strip_prefix(&marker_src) else { continue };
        let target_dir = decl_src.join(rel);
        for f in files.iter().filter(|f| f.ends_with(".rs") && **f != "lib.rs" && **f != "mod.rs") {
            add_file(&mut reserved, &target_dir.join(f.as_str()));
        }
    }
    // 声明层 crate 的包目录名（java_marker 内同名目录 = E0761；包名是 Java 段名，与
    // `<stem>_m` 形态理论上可撞，保底登记）
    let add_dirs = |root: &Path, reserved: &mut BTreeMap<PathBuf, BTreeSet<String>>| {
        for (dir, subs, _) in walk(root) {
            let Ok(rel) = dir.strip_prefix(root) else { continue };
            for n in subs {
                reserved.entry(PathBuf::from(rel)).or_default().insert(n);
            }
        }
    };
    add_dirs(decl_src, &mut reserved);
    add_dirs(&marker_src, &mut reserved);
    reserved
}

/// java_marker 真源源码树（`runtime/java_marker/src`）
pub fn marker_src(ctx: &EmitCtx<'_>) -> Option<PathBuf> {
    ctx.runtime_src()
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join(super::overlay::MARKER_CRATE).join("src"))
        .filter(|p| p.is_dir())
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
/// 不进标记文件；文件头按标记块的词剪枝（见 [`prune_marker_header`]）。
/// 未拆层（无 `decl` 属性）返回 None。
pub fn marker_text(text: &str) -> Option<String> {
    let open = text.find(BLOCK_OPEN)?;
    let start = open + BLOCK_OPEN.len();
    // 块以第 0 列的 `}` 行结束（块内各项至少缩进 4 列），与 `layers::split_text` 同一判定
    let end = text[start..].find("\n}\n")?;
    let close = start + end + 3;
    let block = text[start..close].strip_prefix(DECL_ATTR)?;
    let header = prune_marker_header(&text[..open], block);
    Some(format!("{header}{BLOCK_OPEN}{MARKER_ATTR}{block}"))
}

/// 接口声明文本 → 标记文件文本（§9.9 K4 阶段 3：接口文件未过 `layers::split`，块首无层属性
/// ——标记文件补 `marker` 属性，宿主文件补 `decl` 属性，两者都只在分段之后插入）。
/// 块后尾巴同样不进标记文件；文件头按标记块词剪枝。
fn iface_marker_text(text: &str) -> Option<String> {
    let open = text.find(BLOCK_OPEN)?;
    let start = open + BLOCK_OPEN.len();
    let end = text[start..].find("\n}\n")?;
    let close = start + end + 3;
    let block = &text[start..close];
    // 已带层属性（理论上不会——接口不走 layers::split）按类路径处理
    if block.starts_with(DECL_ATTR) {
        return marker_text(text);
    }
    let header = prune_marker_header(&text[..open], block);
    Some(format!("{header}{BLOCK_OPEN}{MARKER_ATTR}{block}"))
}

/// 标记层计划：标记文件与其宿主类（绝对路径 → 标记模块名；上层段 mod 行按它补声明）
#[derive(Debug, Default)]
pub struct MarkerPlan {
    /// 标记/不透明文件 (落盘路径, 文本)
    pub files: Vec<(PathBuf, String)>,
    /// 有标记文件的类文件绝对路径 → 标记模块名（`<stem>_m`，避让后可带双下划线）
    pub hosts: BTreeMap<PathBuf, String>,
}

impl MarkerPlan {
    pub fn total_bytes(&self) -> usize {
        self.files.iter().map(|(_, t)| t.len()).sum()
    }
}

/// 标记文件在 java_marker 源码树内的落盘路径（类文件相对路径换根）。
/// `decl_src`：声明层底段 crate 源码树（相对路径的计算基准）。
fn marker_path(class: &Path, stem: &str, decl_src: &Path, marker_root: &Path) -> PathBuf {
    let rel = class
        .strip_prefix(decl_src)
        .ok()
        .and_then(|r| r.parent())
        .map(Path::to_path_buf)
        .unwrap_or_default();
    marker_root.join(rel).join(format!("{stem}.rs"))
}

/// 已拆层根模块类与接口的标记文件划分（K4 阶段 3：落 java_marker crate）＋不透明类文件
/// 双发。须在 D8 分段（[`super::decl_side::segment`]）之后调用（分段输入是未剥标记部分的
/// 声明文本，切段与之前一致）。`decl_src`：底段 crate 源码树；`marker_root`：scratch 的
/// java_marker 源码树。
///
/// 同时改写宿主的声明层文本：文件头（块前）插入 `pub use java_marker::<包>::<标记模块>::<类名>;`
/// ——标记部分剥去后类名不在本模块，声明层展开（`impl X` / `X__VTable` / `X__Methods` 实现）
/// 与实现层文件的 `use <声明层>::<包>::<类模块>::*;` 都要它。该行在分段之后插入，
/// 不进 D8 图与体量；java_marker 侧包 mod.rs 的两 glob（类模块与标记模块）重复导出同一
/// `X`，同一定义不歧义。接口宿主另在块首补 `#[rava_layer = "decl"]`（同样分段后插入）。
pub fn split(
    ctx: &EmitCtx<'_>, ems: &mut IndexMap<String, ClassEmission>, decl_src: &Path, marker_root: &Path,
) -> MarkerPlan {
    let crates = ctx.crates();
    let cands: Vec<usize> = ems
        .values()
        .enumerate()
        .filter(|(_, em)| em.crate_name == crates.root() && !em.handwritten)
        // 不透明（L1）类不拆（与 `layers::split` 同一判定）：整类文件双发进 java_marker
        .filter(|(_, em)| !ctx.is_opaque(&em.binary_name))
        // 类（`decl` 属性块）与接口（无层属性块，`iface_marker_text` 补属性）都拆
        .filter(|(_, em)| {
            ctx.class(&em.binary_name).is_some_and(|ci| ci.is_interface())
                || marker_text(&em.text).is_some()
        })
        .map(|(i, _)| i)
        .collect();
    let opaque: Vec<usize> = ems
        .values()
        .enumerate()
        .filter(|(_, em)| em.crate_name == crates.root() && !em.handwritten)
        .filter(|(_, em)| ctx.is_opaque(&em.binary_name))
        .map(|(i, _)| i)
        .collect();
    // 先登记全部候选与既有名，再取名（同包 Foo / FooM 的避让要看到对方）
    let mut reserved = reserved_names(ctx, decl_src);
    let mut plan = MarkerPlan::default();
    for i in cands {
        let (path, bin, is_iface) = {
            let em = &ems[i];
            (em.path.clone(), em.binary_name.clone(), ctx.class(&em.binary_name).is_some_and(|ci| ci.is_interface()))
        };
        let marker_stem = free_marker_stem(&reserved, &path);
        let text = if is_iface { iface_marker_text(&ems[i].text) } else { marker_text(&ems[i].text) }
            .expect("候选已过滤");
        let file = marker_path(&path, &marker_stem, decl_src, marker_root);
        // 标记名自身也占用（两个类的标记名不互相冲突——stem 不同即名不同，此行只作保底）
        reserved.entry(file.parent().map(Path::to_path_buf).unwrap_or_default())
            .or_default()
            .insert(marker_stem.clone());
        plan.files.push((file, text));
        // 宿主声明文件头补类型再导出（见函数头注）；接口宿主另补 decl 层属性
        let name = ctx.declared(&bin);
        let head_end = ems[i].text.find(BLOCK_OPEN).expect("候选已过滤");
        let head = ems[i].text[..head_end].trim_end_matches('\n').to_string();
        let pkg = path
            .strip_prefix(decl_src)
            .ok()
            .and_then(|r| r.parent())
            .map(|p| {
                p.components()
                    .map(|c| super::decl_side::seg_ident(&c.as_os_str().to_string_lossy()))
                    .collect::<Vec<_>>()
                    .join("::")
            })
            .unwrap_or_default();
        let pkg_prefix = if pkg.is_empty() { String::new() } else { format!("{pkg}::") };
        let reexport = format!("pub use java_marker::{pkg_prefix}{marker_stem}::{name};");
        if is_iface {
            let (before, rest) = ems[i].text.split_at(head_end + BLOCK_OPEN.len());
            ems[i].text = format!("{head}\n{reexport}\n\n{before}{DECL_ATTR}{rest}");
        } else {
            ems[i].text =
                format!("{head}\n{reexport}\n\n{}", &ems[i].text[head_end..]);
        }
        plan.hosts.insert(path, marker_stem);
    }
    // 不透明（L1）类文件双发：java_marker 同相对路径逐字节拷贝（头部引用的超类型在
    // java_marker 侧按标记文件 / 其他不透明拷贝解析）
    for i in opaque {
        let em = &ems[i];
        let Ok(rel) = em.path.strip_prefix(decl_src) else { continue };
        plan.files.push((marker_root.join(rel), em.text.clone()));
    }
    plan
}

/// 标记文件头剪枝：只留标记展开引用的名字（按词剪枝，与声明层 [`super::layers::prune_uses`]
/// 同一规则，词表取标记块——标记部分是声明层块的子集，声明层剪枝保下的名字这里保下的是子集，
/// 只会多留不会误删）。方法 trait 匿名导入全部剪去（标记层无方法面）。
fn prune_marker_header(header: &str, block: &str) -> String {
    super::layers::prune_uses(
        header,
        &[block],
        &super::layers::AnonKeep::Set(BTreeSet::new()),
    )
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
    fn unlayered_class_block_has_no_marker_file() {
        let full = "rava_macros::java_class! {\n    #[binary_name = \"a/B\"]\n    pub struct B {}\n}\n";
        assert!(marker_text(full).is_none());
        assert!(marker_text("fn f() {}").is_none());
    }

    #[test]
    fn iface_marker_text_adds_layer_and_drops_tail() {
        let iface = "#![allow(x)]\nuse crate::prelude::*;\n\nrava_macros::java_class! {\n    \
                     #[is_interface]\n     #[binary_name = \"a/I\"]\n     pub struct I {}\n}\n\
                     rava_macros::iface_upcasts! { impl X => I }\n";
        let m = iface_marker_text(iface).unwrap();
        assert_eq!(
            m,
            "#![allow(x)]\nuse crate::prelude::*;\n\nrava_macros::java_class! {\n    \
             #[rava_layer = \"marker\"]\n    #[is_interface]\n     #[binary_name = \"a/I\"]\n     pub struct I {}\n}\n"
        );
        assert!(!m.contains("iface_upcasts"), "{m}");
    }

    #[test]
    fn marker_stem_avoids_handwritten_name_collision() {
        let mut reserved: BTreeMap<PathBuf, BTreeSet<String>> = BTreeMap::new();
        let dir = PathBuf::from("/w/java/lang");
        // java_marker 手写同名占位：foo 的标记名退 `foo__m`
        reserved.insert(dir.clone(), ["foo_m".to_string(), "bar".to_string()].into_iter().collect());
        assert_eq!(free_marker_stem(&reserved, &dir.join("foo.rs")), "foo__m");
        assert_eq!(free_marker_stem(&reserved, &dir.join("bar.rs")), "bar_m");
        // 未登记目录无冲突
        assert_eq!(free_marker_stem(&reserved, &dir.join("other/x.rs")), "x_m");
    }
}
