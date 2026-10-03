//! 栈溢出两层：
//! - 软件栈界（所有平台）：带检查点的递归在栈界处判定耗尽（不触及 guard），放开 YELLOW 后还能继续用栈；
//!   栈界随执行流切换；
//! - 硬件 guard（仅在启用时，见 `stack::hardware_guard`）：子进程（串行）中协程无检查点无限递归，
//!   退出为 SIGABRT 且 stderr 含 overflowed。子进程即本测试二进制以 `--ignored --exact child_overflow` 重入。

use rava_coro::{stack, stack_exhausted, stack_limit, switch, Context, Stack, YellowZone, SHADOW, YELLOW};
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
    drop(Stack::new().unwrap());
    if !stack::hardware_guard() {
        eprintln!("[rava_coro] 本机未启用硬件 guard（Linux < 6.13 无 MADV_GUARD_INSTALL），跳过；溢出由软件栈界拦截");
        return;
    }
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

/// 带检查点的递归（模拟宏注入 `__stack_check()?` 的 Java 方法）：耗尽时记下栈地址，并在判定点
/// 放开 YELLOW 验证构造 StackOverflowError 的余量（out = 深度、地址、余量可用且 drop 后恢复）
#[inline(never)]
fn checked(n: u64, out: &mut (u64, usize, bool)) -> Result<u64, ()> {
    let mut pad = [n; 32];
    std::hint::black_box(&mut pad);
    if stack_exhausted() {
        let ok = {
            let _y = YellowZone::enter();
            let before = !stack_exhausted();
            std::hint::black_box(burn(YELLOW / 2));
            before && !stack_exhausted()
        };
        *out = (n, pad.as_ptr() as usize, ok && stack_exhausted());
        return Err(());
    }
    Ok(checked(n + 1, out)?.wrapping_add(pad[3]))
}

/// 不检查地用掉约 `bytes` 栈
#[inline(never)]
fn burn(bytes: usize) -> u64 {
    let mut pad = [bytes as u64; 64];
    std::hint::black_box(&mut pad);
    if bytes <= 512 {
        return pad[1];
    }
    burn(bytes - 512).wrapping_add(pad[5])
}

struct Soft {
    main: Context,
    co: Context,
    lo: usize,
    depth: u64,
    hit: usize,
    limit_inside: usize,
    yellow_ok: bool,
}

unsafe extern "C" fn soft_body(_: usize, data: *mut u8) -> ! {
    let s = data as *mut Soft;
    (*s).limit_inside = stack_limit();
    let mut out = (0, 0, false);
    let _ = checked(0, &mut out);
    (*s).depth = out.0;
    (*s).hit = out.1;
    (*s).yellow_ok = out.2;
    switch(&mut (*s).co, &mut (*s).main, 0);
    unreachable!();
}

#[test]
fn software_limit_detects_exhaustion() {
    let platform = rava_coro::init_platform_thread();
    assert_ne!(platform, 0, "平台线程应取得栈区间");
    assert_eq!(stack_limit(), platform);
    let stack = Stack::new().unwrap();
    let mut s = Box::new(Soft {
        main: Context::empty(),
        co: Context::empty(),
        lo: stack.limit(),
        depth: 0,
        hit: 0,
        limit_inside: 0,
        yellow_ok: false,
    });
    let p: *mut Soft = &mut *s;
    unsafe {
        (*p).co = Context::new(&stack, soft_body, p.cast());
        switch(&mut (*p).main, &mut (*p).co, 0);
    }
    // 栈界随执行流：协程内为本栈的栈界，切回后恢复平台线程的
    assert_eq!(s.limit_inside, rava_coro::limit_for(s.lo));
    assert_eq!(stack_limit(), platform);
    // 判定点落在栈界之下一帧以内，远在可用区底（及 guard）之上
    let limit = s.lo + SHADOW + YELLOW;
    assert!(s.hit.abs_diff(limit) < 4096, "hit {:#x} limit {:#x}", s.hit, limit);
    assert!(s.yellow_ok);
    eprintln!(
        "[rava_coro] 软件栈界：深度 {} 处判定耗尽，距可用区底 {} KiB；硬件 guard {}",
        s.depth,
        (s.hit - s.lo) >> 10,
        if stack::hardware_guard() { "启用" } else { "未启用" }
    );
}
