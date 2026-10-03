//! panic 跨栈传播（§21.8.3）：
//! - abort 形态（生成工作区 `panic = "abort"`，`create_java_vm` 装「默认钩子 + exit(101)」）：子进程（串行）中
//!   协程内 panic 与平台线程 panic 退出码同为 101、stderr 同形（`thread '<名>' panicked at …` + 消息）；
//!   RUST_BACKTRACE=1 时协程内回溯止于蹦床，不越过它走进载体栈帧；
//! - unwind 形态（本 crate 单测即 unwind 构建）：入口体经 `catch_entry` 捕获，切回载体后 `resume_unwind`，
//!   在载体栈上被 `catch_unwind` 捕获 1/1。
//! 子进程即本测试二进制以 `--ignored --exact child_*` 重入。

use rava_coro::{catch_entry, switch, Context, PanicPayload, Stack};
use std::process::Command;

const THREAD_NAME: &str = "vm-worker";
const MESSAGE: &str = "boom from java";

/// 与 `create_java_vm` 同形的钩子：默认钩子输出后以 101 退出（不 unwind）
fn install_exit_hook() {
    let default = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        default(info);
        std::process::exit(101);
    }));
}

struct Shared {
    main: Context,
    co: Context,
    panicked: Option<PanicPayload>,
    catch: bool,
}

#[inline(never)]
fn coroutine_marker_body() {
    std::hint::black_box(0);
    panic!("{MESSAGE}");
}

unsafe extern "C" fn entry(_: usize, data: *mut u8) -> ! {
    let s = data as *mut Shared;
    if (*s).catch {
        if let Err(p) = catch_entry(coroutine_marker_body) {
            (*s).panicked = Some(p);
        }
    } else {
        coroutine_marker_body();
    }
    switch(&mut (*s).co, &mut (*s).main, 0);
    unreachable!();
}

/// 载体侧标记帧：协程回溯不得出现它
#[inline(never)]
fn carrier_marker_mount(catch: bool) -> Option<PanicPayload> {
    let stack = Stack::new().unwrap();
    let mut s = Box::new(Shared { main: Context::empty(), co: Context::empty(), panicked: None, catch });
    let p: *mut Shared = &mut *s;
    unsafe {
        (*p).co = Context::new(&stack, entry, p.cast());
        switch(&mut (*p).main, &mut (*p).co, 0);
    }
    std::hint::black_box(&stack);
    s.panicked.take()
}

fn on_named_thread(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new().name(THREAD_NAME.into()).spawn(f).unwrap().join().ok();
    unreachable!("子进程应在钩子中以 101 退出");
}

#[test]
#[ignore = "仅作子进程"]
fn child_platform_panic() {
    install_exit_hook();
    on_named_thread(coroutine_marker_body);
}

#[test]
#[ignore = "仅作子进程"]
fn child_coroutine_panic() {
    install_exit_hook();
    on_named_thread(|| {
        carrier_marker_mount(false);
    });
}

fn run_child(name: &str, backtrace: bool) -> (Option<i32>, String) {
    let exe = std::env::current_exe().unwrap();
    let out = Command::new(exe)
        .args(["--ignored", "--exact", name, "--nocapture", "--test-threads=1"])
        .env("RUST_BACKTRACE", if backtrace { "1" } else { "0" })
        .output()
        .unwrap();
    (out.status.code(), String::from_utf8_lossy(&out.stderr).into_owned())
}

/// panic 报告的形状：`thread '<名>' panicked at <文件>:<行>:<列>:` 去掉行列与线程号，加消息行
fn shape(stderr: &str) -> Vec<String> {
    let mut lines = stderr.lines().filter(|l| !l.trim().is_empty());
    let head = lines.find(|l| l.starts_with("thread '")).unwrap_or_else(|| panic!("无 panic 报告：{stderr}"));
    let mut head = match head.rsplit_once(".rs:") {
        Some((file, _)) => format!("{file}.rs"),
        None => head.to_string(),
    };
    // 新版 std 在线程名后附 OS 线程号 `(<tid>)`，各进程不同
    if let (Some(l), Some(r)) = (head.find("' ("), head.find(") panicked")) {
        head.replace_range(l + 1..r + 1, "");
    }
    vec![head, lines.next().unwrap_or_default().to_string()]
}

#[test]
fn coroutine_panic_matches_platform_thread() {
    let (code_p, err_p) = run_child("child_platform_panic", false);
    let (code_c, err_c) = run_child("child_coroutine_panic", false);
    assert_eq!(code_p, Some(101), "平台线程 stderr：{err_p}");
    assert_eq!(code_c, Some(101), "协程 stderr：{err_c}");
    let (sp, sc) = (shape(&err_p), shape(&err_c));
    assert_eq!(sp, sc);
    assert!(sc[0].starts_with(&format!("thread '{THREAD_NAME}' panicked at ")), "{sc:?}");
    assert_eq!(sc[1], MESSAGE);
    eprintln!("[rava_coro] 协程 / 平台线程 panic 同形：{sc:?}");
}

#[test]
fn coroutine_backtrace_stops_at_trampoline() {
    let (code, err) = run_child("child_coroutine_panic", true);
    assert_eq!(code, Some(101), "stderr：{err}");
    assert!(err.contains("coroutine_marker_body"), "回溯应含协程内帧：{err}");
    assert!(!err.contains("carrier_marker_mount"), "回溯越过蹦床走进了载体栈：{err}");
}

#[test]
fn unwind_crosses_to_carrier() {
    let quiet = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let mut caught = 0;
    for _ in 0..1 {
        let r = std::panic::catch_unwind(|| {
            if let Some(p) = carrier_marker_mount(true) {
                std::panic::resume_unwind(p);
            }
        });
        let p = r.expect_err("协程内 panic 应在载体上继续 unwind");
        let msg = p.downcast_ref::<String>().map(String::as_str).unwrap_or_default();
        assert_eq!(msg, MESSAGE);
        caught += 1;
    }
    std::panic::set_hook(quiet);
    assert_eq!(caught, 1);
    eprintln!("[rava_coro] 协程内 panic 在载体上被 catch_unwind 捕获 {caught}/1");
}
