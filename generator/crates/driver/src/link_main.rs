//! rava-link：生成程序的链接器包装（`rava compile` 经 cargo `target.<host>.linker` 指定，二进制体积 B2）。
//!
//! 1. 以原参数链接（ELF 去掉剥离参数：DWARF 要留到读表之后；剥离由第二次链接完成）；
//! 2. 产物无地址锚点 `__rava_pc_anchor`（proc-macro、构建脚本等）→ 原样结束（ELF 若去过剥离参数则按原参数重链）；
//! 3. 有锚点（Java 程序）→ 读 DWARF 与旁路行表（环境变量 `RAVA_FRAME_LINES`，缺失即报错）构建地址表，
//!    写表目标文件，原参数加该目标文件再链接一次；核对两次链接的锚点地址与代码节不变（表内地址有效）。
//!
//! 真实链接器为 `cc`（与 rustc 缺省一致）。统计行写 `<scratch>/logs/pcmap.log`；诊断：
//! `RAVA_PCMAP_DUMP=<Java 方法名>` 把映射到该方法的各段（符号、含闭包帧的内联链）写 `<scratch>/logs/pcmap_dump.txt`。
//! release 的 `strip = "symbols"`：macOS 由 rustc 在链接器返回后执行 strip，ELF 传给链接器（第 1 步去掉、第二次链接生效）。

mod pcmap;

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::time::Instant;

/// ELF 链接器的剥离参数（`-Wl,` 逗号列表中的项亦计）
const STRIP_FLAGS: [&str; 4] = ["--strip-all", "--strip-debug", "-s", "-S"];

fn main() -> ExitCode {
    let args: Vec<OsString> = std::env::args_os().skip(1).collect();
    match run(OsStr::new("cc"), &args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("rava-link: {e}");
            ExitCode::FAILURE
        }
    }
}

fn link(linker: &OsStr, args: &[OsString]) -> Result<bool, String> {
    let status = Command::new(linker).args(args).status().map_err(|e| format!("{}：{e}", linker.to_string_lossy()))?;
    Ok(status.success())
}

fn run(linker: &OsStr, args: &[OsString]) -> Result<ExitCode, String> {
    let expanded = expand(args)?;
    let output = expanded
        .iter()
        .position(|a| a == "-o")
        .and_then(|i| expanded.get(i + 1))
        .map(PathBuf::from)
        .ok_or("链接参数缺少 -o")?;
    let (first, filtered, temp) = without_strip(args)?;
    let ok = link(linker, &first);
    for t in &temp {
        let _ = std::fs::remove_file(t);
    }
    if !ok? {
        return Ok(ExitCode::FAILURE);
    }
    let data = std::fs::read(&output).map_err(|e| format!("{}：{e}", output.display()))?;
    let exe = object::File::parse(&*data).map_err(|e| format!("{}：{e}", output.display()))?;
    let Some(before) = pcmap::layout(&exe) else {
        // 非 Java 程序：去过剥离参数则按原参数重链
        if filtered && !link(linker, args)? {
            return Ok(ExitCode::FAILURE);
        }
        return Ok(ExitCode::SUCCESS);
    };
    let start = Instant::now();
    let sidecar = PathBuf::from(std::env::var_os("RAVA_FRAME_LINES").ok_or("Java 程序链接缺少 RAVA_FRAME_LINES（旁路行表路径，由 rava compile 设置）")?);
    let text = std::fs::read_to_string(&sidecar).map_err(|e| format!("{}：{e}", sidecar.display()))?;
    let lines: emit::project::line_tables::FrameLines =
        serde_json::from_str(&text).map_err(|e| format!("{}：{e}", sidecar.display()))?;
    let built = pcmap::build(&exe, &data, &lines, before.anchor)?;
    let obj = pcmap::object_file(&exe, &data, &built.blob)?;
    drop(exe);
    drop(data);
    let obj_path = PathBuf::from(format!("{}.rava_pcmap.o", output.display()));
    std::fs::write(&obj_path, obj).map_err(|e| format!("{}：{e}", obj_path.display()))?;
    let mut second = args.to_vec();
    second.push(obj_path.clone().into_os_string());
    let ok = link(linker, &second)?;
    let _ = std::fs::remove_file(&obj_path);
    if !ok {
        return Ok(ExitCode::FAILURE);
    }
    let data = std::fs::read(&output).map_err(|e| format!("{}：{e}", output.display()))?;
    let exe = object::File::parse(&*data).map_err(|e| format!("{}：{e}", output.display()))?;
    let after = pcmap::layout(&exe);
    // ELF 第二次链接已剥离符号：锚点不可再查，只核对代码节
    let same = match after {
        Some(a) => a == before,
        None => text_range(&exe) == Some(before.text),
    };
    if !same {
        return Err(format!("两次链接布局不一致（{before:?} → {after:?}），地址表失效"));
    }
    let line = format!("{} {} | {:.2}s\n", output.display(), built.stats, start.elapsed().as_secs_f64());
    log(&sidecar, "pcmap.log", &line);
    if !built.dump.is_empty() {
        log(&sidecar, "pcmap_dump.txt", &built.dump);
    }
    Ok(ExitCode::SUCCESS)
}

fn text_range(exe: &object::File<'_>) -> Option<(u64, u64)> {
    use object::{Object, ObjectSection, SectionKind};
    exe.sections()
        .find(|s| s.kind() == SectionKind::Text && matches!(s.name(), Ok(".text" | "__text")))
        .map(|s| (s.address(), s.size()))
}

/// 统计行 / 诊断转储追加到 `<scratch>/logs/<name>`（旁路行表在 `<scratch>/closure_input/`）
fn log(sidecar: &Path, name: &str, line: &str) {
    use std::io::Write;
    let Some(scratch) = sidecar.parent().and_then(Path::parent) else { return };
    let dir = scratch.join("logs");
    let _ = std::fs::create_dir_all(&dir);
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(dir.join(name)) {
        let _ = f.write_all(line.as_bytes());
    }
}

/// 展开 `@响应文件`（只用于取 `-o`）
fn expand(args: &[OsString]) -> Result<Vec<OsString>, String> {
    let mut out = Vec::new();
    for a in args {
        match a.to_str().and_then(|s| s.strip_prefix('@')) {
            Some(file) => out.extend(read_response(Path::new(file))?.into_iter().map(OsString::from)),
            None => out.push(a.clone()),
        }
    }
    Ok(out)
}

/// 响应文件：每行一个参数（gcc 形态，`\` 转义）
fn read_response(path: &Path) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}：{e}", path.display()))?;
    Ok(text
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut s = String::with_capacity(l.len());
            let mut chars = l.chars();
            while let Some(c) = chars.next() {
                match c {
                    '\\' => s.extend(chars.next()),
                    '"' => {}
                    c => s.push(c),
                }
            }
            s
        })
        .collect())
}

/// 去掉剥离参数后的参数表（ELF；Mach-O 不经链接器剥离，原样返回）、是否去过、改写出的临时响应文件
fn without_strip(args: &[OsString]) -> Result<(Vec<OsString>, bool, Vec<PathBuf>), String> {
    if cfg!(target_vendor = "apple") {
        return Ok((args.to_vec(), false, Vec::new()));
    }
    let mut filtered = false;
    let mut temp = Vec::new();
    let mut out = Vec::new();
    for a in args {
        let Some(s) = a.to_str() else {
            out.push(a.clone());
            continue;
        };
        if let Some(file) = s.strip_prefix('@') {
            let text = std::fs::read_to_string(file).map_err(|e| format!("{file}：{e}"))?;
            let kept: Vec<String> = text.lines().filter_map(|l| strip_arg(l, &mut filtered)).collect();
            let copy = PathBuf::from(format!("{file}.rava-link"));
            std::fs::write(&copy, kept.join("\n") + "\n").map_err(|e| format!("{}：{e}", copy.display()))?;
            out.push(format!("@{}", copy.display()).into());
            temp.push(copy);
        } else if let Some(kept) = strip_arg(s, &mut filtered) {
            out.push(kept.into());
        }
    }
    Ok((out, filtered, temp))
}

/// 单个参数去掉剥离项：`-s` 等整项删除，`-Wl,a,--strip-all,b` 删其中一项（删空则整项删除）
fn strip_arg(arg: &str, filtered: &mut bool) -> Option<String> {
    if STRIP_FLAGS.contains(&arg) {
        *filtered = true;
        return None;
    }
    let Some(list) = arg.strip_prefix("-Wl,") else { return Some(arg.to_owned()) };
    let items: Vec<&str> = list.split(',').collect();
    let kept: Vec<&str> = items.iter().copied().filter(|i| !STRIP_FLAGS.contains(i)).collect();
    if kept.len() == items.len() {
        return Some(arg.to_owned());
    }
    *filtered = true;
    (!kept.is_empty()).then(|| format!("-Wl,{}", kept.join(",")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_args_removed() {
        let mut f = false;
        assert_eq!(strip_arg("-Wl,--as-needed", &mut f).as_deref(), Some("-Wl,--as-needed"));
        assert!(!f);
        assert_eq!(strip_arg("-Wl,--strip-all", &mut f), None);
        assert!(f);
        assert_eq!(strip_arg("-Wl,-O1,--strip-debug", &mut f).as_deref(), Some("-Wl,-O1"));
        assert_eq!(strip_arg("-s", &mut f), None);
        assert_eq!(strip_arg("-o", &mut f).as_deref(), Some("-o"));
    }
}
