//! 生成器身份摘要（档案键 P 的「生成器源码树」一项，计划 2026-10-01-cross-test-compile-reuse.md §4.1）：
//! `generator/` 下各 crate 的 `src/**`、`build.rs`、`Cargo.toml`，加工作区 `Cargo.toml` / `Cargo.lock`，
//! 按相对路径排序后连同内容取 128 位摘要，经 `RAVA_GENERATOR_DIGEST` 编入 rava。
//! 与构建位置、编译器产物无关：同一源码树在任何机器上得到同一摘要。

use std::path::{Path, PathBuf};

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with('.')) {
            continue;
        }
        if p.is_dir() {
            walk(&p, out);
        } else {
            out.push(p);
        }
    }
}

/// 两条 64 位通道的 FNV-1a 变体（构建脚本不依赖工作区 crate）
struct H(u64, u64);

impl H {
    fn eat(&mut self, data: &[u8]) {
        for &b in (data.len() as u64).to_le_bytes().iter().chain(data) {
            self.0 = (self.0 ^ b as u64).wrapping_mul(0x0000_0100_0000_01B3);
            self.1 = (self.1 ^ b as u64).wrapping_mul(0x0000_0100_0000_01B3).rotate_left(5);
        }
    }
}

fn main() {
    let driver = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let gen = driver.join("../..").canonicalize().expect("generator 目录");
    let mut files = vec![gen.join("Cargo.toml"), gen.join("Cargo.lock")];
    let crates = gen.join("crates");
    let mut members: Vec<PathBuf> = std::fs::read_dir(&crates).expect("crates 目录").flatten().map(|e| e.path()).filter(|p| p.is_dir()).collect();
    members.sort();
    for c in &members {
        for f in ["Cargo.toml", "build.rs"] {
            if c.join(f).is_file() {
                files.push(c.join(f));
            }
        }
        walk(&c.join("src"), &mut files);
        println!("cargo:rerun-if-changed={}", c.display());
    }
    println!("cargo:rerun-if-changed={}", gen.join("Cargo.toml").display());
    println!("cargo:rerun-if-changed={}", gen.join("Cargo.lock").display());
    files.sort();
    let mut h = H(0xCBF2_9CE4_8422_2325, 0x8422_2325_CBF2_9CE4);
    for f in &files {
        let rel = f.strip_prefix(&gen).unwrap_or(f);
        h.eat(rel.to_string_lossy().as_bytes());
        h.eat(&std::fs::read(f).unwrap_or_default());
    }
    println!("cargo:rustc-env=RAVA_GENERATOR_DIGEST={:016x}{:016x}", h.0, h.1);
}
