//! `rava build --lib`：jar 输入模式的 lib crate 装配。
//!
//! 每个 `--lib NAME=JAR[:seed=…]` 枚举 jar 类成 [`LibCrate`]，并给出闭包分析的种子类：
//! 整包模式 = jar 全部类（按名排序），子集模式 = 声明的种子类（须在 jar 内）。种子类统一经
//! `seed_roots` 展开为 public 方法（命令行入口 main 除外），不进用户类通道。

use std::path::PathBuf;

use input::LibCrate;

use crate::build_opts::LibSpec;

/// 装配结果（声明序）
#[derive(Debug, Default)]
pub struct Libs {
    pub crates: Vec<LibCrate>,
    /// 闭包种子类（斜线形态）
    pub seed_classes: Vec<String>,
    /// jar 绝对路径（javac `-cp` 与类路径 `Origin::Lib`）
    pub jars: Vec<PathBuf>,
}

pub fn load(specs: &[LibSpec]) -> Result<Libs, String> {
    let mut out = Libs::default();
    for s in specs {
        if !s.jar.is_file() {
            return Err(format!("--lib jar 不存在：{}", s.jar.display()));
        }
        let jar = std::path::absolute(&s.jar).unwrap_or_else(|_| s.jar.clone());
        let lc = LibCrate::from_jar(&s.name, &jar, s.seeds.is_none()).map_err(|e| e.to_string())?;
        if lc.jar_classes.is_empty() {
            return Err(format!("jar 中未找到类条目：{}", jar.display()));
        }
        let base = jar.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default();
        println!("[jar] {} ← {base}（{} 类）", s.name, lc.jar_classes.len());
        match &s.seeds {
            None => {
                let mut all = lc.jar_classes.clone();
                all.sort();
                out.seed_classes.extend(all);
            }
            Some(seeds) => {
                if let Some(miss) = seeds.iter().find(|f| !lc.jar_classes.contains(f)) {
                    return Err(format!("--lib seed 类 {miss} 不在 {} 中", jar.display()));
                }
                out.seed_classes.extend(seeds.iter().cloned());
            }
        }
        out.jars.push(jar);
        out.crates.push(lc);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_jar_is_error() {
        let spec = LibSpec { name: "x".into(), jar: PathBuf::from("/nonexistent/x.jar"), seeds: None };
        assert_eq!(load(&[spec]).unwrap_err(), "--lib jar 不存在：/nonexistent/x.jar");
        assert!(load(&[]).unwrap().crates.is_empty());
    }
}
