//! 往返切换正确性与耗时（验收：10⁶ 次往返正确，release 下单次切换 ≤50 ns）

use rava_coro::{switch, Context, Stack};
use std::time::Instant;

struct Shared {
    main: Context,
    co: Context,
    count: usize,
    done: bool,
}

unsafe extern "C" fn counter(arg: usize, data: *mut u8) -> ! {
    let s = data as *mut Shared;
    let mut v = arg;
    while v != usize::MAX {
        (*s).count += 1;
        // 局部变量跨切换保持
        let before = v;
        let local = std::hint::black_box(before.wrapping_mul(3));
        v = switch(&mut (*s).co, &mut (*s).main, v + 1);
        assert_eq!(std::hint::black_box(local), before.wrapping_mul(3));
    }
    (*s).done = true;
    switch(&mut (*s).co, &mut (*s).main, 0);
    unreachable!("已完成的协程被再次切入");
}

#[test]
fn million_roundtrips() {
    const N: usize = 1_000_000;
    let stack = Stack::new().expect("stack");
    let mut s = Box::new(Shared { main: Context::empty(), co: Context::empty(), count: 0, done: false });
    let sp: *mut Shared = &mut *s;
    unsafe {
        (*sp).co = Context::new(&stack, counter, sp.cast());
        let mut v = 0usize;
        // 预热
        v = switch(&mut (*sp).main, &mut (*sp).co, v);
        assert_eq!(v, 1);
        let t = Instant::now();
        for i in 0..N {
            v = switch(&mut (*sp).main, &mut (*sp).co, v + 1);
            assert_eq!(v, 2 * i + 3);
        }
        let el = t.elapsed();
        assert_eq!((*sp).count, N + 1);
        switch(&mut (*sp).main, &mut (*sp).co, usize::MAX);
        assert!((*sp).done);
        let per = el.as_nanos() as f64 / (2 * N) as f64;
        eprintln!("[rava_coro] {N} 次往返 {el:?}，单次切换 {per:.2} ns（{}）", std::env::consts::ARCH);
        if !cfg!(debug_assertions) {
            assert!(per <= 50.0, "单次切换 {per:.2} ns > 50 ns");
        }
    }
    drop(stack);
}

/// 协程在一个线程挂起、在另一线程恢复（虚拟线程换载体）
#[test]
fn resume_on_other_thread() {
    struct Hop {
        main: Context,
        co: Context,
        seen: Vec<std::thread::ThreadId>,
    }
    unsafe extern "C" fn hop(_: usize, data: *mut u8) -> ! {
        let h = data as *mut Hop;
        for _ in 0..4 {
            (*h).seen.push(std::thread::current().id());
            switch(&mut (*h).co, &mut (*h).main, 0);
        }
        switch(&mut (*h).co, &mut (*h).main, 1);
        unreachable!();
    }
    let stack = Stack::new().expect("stack");
    let mut h = Box::new(Hop { main: Context::empty(), co: Context::empty(), seen: Vec::new() });
    let hp = &mut *h as *mut Hop as usize;
    unsafe { (*(hp as *mut Hop)).co = Context::new(&stack, hop, hp as *mut u8) };
    let mut fin = 0;
    for _ in 0..5 {
        fin = std::thread::spawn(move || unsafe {
            let h = hp as *mut Hop;
            switch(&mut (*h).main, &mut (*h).co, 0)
        })
        .join()
        .unwrap();
    }
    assert_eq!(fin, 1);
    let ids = &h.seen;
    assert_eq!(ids.len(), 4);
    for w in ids.windows(2) {
        assert_ne!(w[0], w[1], "每次恢复都在新线程上");
    }
    drop(stack);
}
