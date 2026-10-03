//! guard page 命中：子进程（串行）中协程无限递归，退出为 SIGABRT 且 stderr 含 overflowed。
//! 子进程即本测试二进制以 `--ignored --exact child_overflow` 重入。

use rava_coro::{switch, Context, Stack};
use std::os::unix::process::ExitStatusExt;
use std::process::Command;

#[inline(never)]
fn recurse(n: u64) -> u64 {
    let mut pad = [n; 64];
    std::hint::black_box(&mut pad);
    if n == u64::MAX {
        return 0;
    }
    recurse(n + 1).wrapping_add(pad[7])
}

struct Shared {
    main: Context,
    co: Context,
}

unsafe extern "C" fn deep(_: usize, data: *mut u8) -> ! {
    let s = data as *mut Shared;
    let v = recurse(0);
    switch(&mut (*s).co, &mut (*s).main, v as usize);
    unreachable!();
}

#[test]
#[ignore = "仅作 guard_page_overflow_aborts 的子进程"]
fn child_overflow() {
    let stack = Stack::new().unwrap();
    let mut s = Box::new(Shared { main: Context::empty(), co: Context::empty() });
    let p: *mut Shared = &mut *s;
    unsafe {
        (*p).co = Context::new(&stack, deep, p.cast());
        switch(&mut (*p).main, &mut (*p).co, 0);
    }
    unreachable!("协程无限递归应在 guard page 处 abort");
}

#[test]
fn guard_page_overflow_aborts() {
    let exe = std::env::current_exe().unwrap();
    let out = Command::new(exe)
        .args(["--ignored", "--exact", "child_overflow", "--nocapture", "--test-threads=1"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.signal(), Some(libc::SIGABRT), "退出状态 {:?}，stderr：{stderr}", out.status);
    assert!(stderr.contains("has overflowed its stack"), "stderr：{stderr}");
    eprintln!("[rava_coro] 子进程 stderr：{}", stderr.trim());
}
