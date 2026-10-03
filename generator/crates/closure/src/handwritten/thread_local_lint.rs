//! 手写层线程局部存储的守卫（a3-T3）。
//!
//! 执行流（平台线程或虚拟线程）可能在一次调用之后换到另一载体 OS 线程上继续，OS 线程局部存储只能承载
//! 与载体绑定的槽；随执行流走的状态一律放执行上下文块（`exec_context.rs` 的 `ExecState`）。故手写层的
//! `thread_local!` 定义只允许出现在两处：执行上下文块本身（平台线程的缺省块与当前块指针），以及
//! `Thread` 的载体槽（`JavaThread` 的 `_threadObj` / `_vthread` / `_scopedValueCache` 对应物，经不内联的
//! 存取函数访问）。

#[cfg(test)]
mod tests {
    /// 允许定义线程局部存储的手写文件（相对 runtime/java_runtime/src）
    const ALLOWED: &[&str] = &["exec_context.rs", "java/lang/thread_impl.rs"];

    fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        for e in std::fs::read_dir(dir).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                walk(&p, out);
            } else if p.extension().is_some_and(|x| x == "rs") {
                out.push(p);
            }
        }
    }

    #[test]
    fn thread_locals_only_in_carrier_slots() {
        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../runtime/java_runtime/src");
        let mut files = Vec::new();
        walk(&src, &mut files);
        assert!(files.len() > 10, "未找到手写层源文件：{}", src.display());
        let mut bad = Vec::new();
        for f in &files {
            let rel = f.strip_prefix(&src).unwrap().to_string_lossy().replace('\\', "/");
            if ALLOWED.contains(&rel.as_str()) {
                continue;
            }
            for (i, line) in std::fs::read_to_string(f).unwrap().lines().enumerate() {
                let t = line.trim_start();
                if t.starts_with("//") {
                    continue;
                }
                if t.contains("thread_local!") || t.contains("#[thread_local]") {
                    bad.push(format!("{rel}:{}: {t}", i + 1));
                }
            }
        }
        assert!(bad.is_empty(), "线程局部存储只允许在执行上下文块与载体槽中定义，随执行流走的状态放 ExecState：\n{}", bad.join("\n"));
    }
}
