//! S5 剥体对照（§7.5.2 验收）：`decl_check <S4 树> <S5 树>`（两树为同一用例的生成树根）。
//!
//! 对 S5 树 `java_runtime/src` 下每个声明层类文件（块首 `#[rava_layer = "decl"]`）：
//! 1. 文本：对 S4 同名文件的块按剥体计划改写，须与 S5 文件逐字节一致（块外文本也一致）；
//! 2. 展开：S4 块（带体）与 S5 块（剥体）的声明模式展开，摘要常量值归一后须逐记号一致。
//! 实现层与其余文件的逐字节对照用 `diff -r`（本工具不做）。

use std::path::{Path, PathBuf};

use rava_macros_core::plan::{decl_elisions, decl_expansion_normalized, elide};

const BLOCK_OPEN: &str = "rava_macros::java_class! {\n";
const DECL_ATTR: &str = "    #[rava_layer = \"decl\"]\n";

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

/// (块前文本, 块内文本, 块后文本)
fn parts(text: &str) -> Option<(&str, &str, &str)> {
    let open = text.find(BLOCK_OPEN)?;
    let start = open + BLOCK_OPEN.len();
    let end = start + text[start..].find("\n}\n")? + 1;
    Some((&text[..start], &text[start..end], &text[end..]))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (t4, t5) = (PathBuf::from(&args[1]), PathBuf::from(&args[2]));
    let src5 = t5.join("java_runtime/src");
    let mut files = Vec::new();
    walk(&src5, &mut files);
    files.sort();
    let (mut n, mut moved, mut bad) = (0usize, 0usize, 0usize);
    let (mut bytes4, mut bytes5) = (0usize, 0usize);
    for p5 in files {
        let s5 = std::fs::read_to_string(&p5).unwrap();
        let Some((pre5, inner5, post5)) = parts(&s5) else { continue };
        if !inner5.starts_with(DECL_ATTR) {
            continue;
        }
        n += 1;
        let rel = p5.strip_prefix(&t5).unwrap();
        let p4 = t4.join(rel);
        let Ok(s4) = std::fs::read_to_string(&p4) else {
            println!("MISSING-S4 {}", rel.display());
            bad += 1;
            continue;
        };
        let (pre4, inner4, post4) = parts(&s4).unwrap();
        bytes4 += s4.len();
        bytes5 += s5.len();
        let plan = match decl_elisions(inner4) {
            Ok(p) => p,
            Err(e) => {
                println!("PLAN-FAIL {}：{e}", rel.display());
                bad += 1;
                continue;
            }
        };
        moved += plan.len();
        if pre4 != pre5 || post4 != post5 || elide(inner4, &plan) != inner5 {
            println!("TEXT-DIFF {}", rel.display());
            bad += 1;
            continue;
        }
        match (decl_expansion_normalized(inner4), decl_expansion_normalized(inner5)) {
            (Ok(a), Ok(b)) if a == b && !a.contains("compile_error") => {}
            (Ok(a), Ok(b)) => {
                println!("EXPAND-DIFF {} ({} / {} 字节)", rel.display(), a.len(), b.len());
                // 设 DECL_CHECK_DUMP=<目录> 时写出两侧展开（逐记号换行），便于 diff 定位
                if let Ok(dir) = std::env::var("DECL_CHECK_DUMP") {
                    let stem = rel.to_string_lossy().replace('/', "__");
                    let one = |s: &str| s.split(' ').collect::<Vec<_>>().join("\n");
                    let _ = std::fs::create_dir_all(&dir);
                    let _ = std::fs::write(format!("{dir}/{stem}.s4"), one(&a));
                    let _ = std::fs::write(format!("{dir}/{stem}.s5"), one(&b));
                }
                bad += 1;
            }
            (a, b) => {
                println!("EXPAND-FAIL {}：{:?} / {:?}", rel.display(), a.err(), b.err());
                bad += 1;
            }
        }
    }
    println!("decl-check classes={n} moved_bodies={moved} bad={bad} decl_bytes {bytes4} -> {bytes5}");
    std::process::exit(if bad == 0 { 0 } else { 1 });
}
