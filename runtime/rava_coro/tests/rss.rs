//! 10⁵ 个协程同时挂起：映射数与存活数脱钩（Linux 缺省 `vm.max_map_count` 65530 下成立，映射增量 ≤ 256），
//! 全部完成后 RSS 回落（验收：回到起点 +16 MiB 以内）

use rava_coro::{stack, switch, Context, Stack};

// macOS：取内核记账的物理足迹（phys_footprint，含页表）。resident_size 把 MADV_FREE_REUSABLE 归还的页
// 计到被回收为止，不反映归还效果；Linux 的 MADV_DONTNEED 立即生效，直接取 RSS。
#[cfg(target_os = "macos")]
fn rss() -> usize {
    // SAFETY: proc_pid_rusage 查询本进程资源记账，缓冲区为 rusage_info_v2
    unsafe {
        let mut info: libc::rusage_info_v2 = std::mem::zeroed();
        let r = libc::proc_pid_rusage(
            libc::getpid(),
            libc::RUSAGE_INFO_V2,
            &mut info as *mut _ as *mut libc::rusage_info_t,
        );
        assert_eq!(r, 0);
        info.ri_phys_footprint as usize
    }
}

// Linux：RSS 加页表（VmPTE）。页表不计入 RSS，但 madvise 不回收页表，空块整块释放才回收；一并计入才能验证
#[cfg(not(target_os = "macos"))]
fn rss() -> usize {
    let s = std::fs::read_to_string("/proc/self/statm").unwrap();
    let pages: usize = s.split_whitespace().nth(1).unwrap().parse().unwrap();
    let status = std::fs::read_to_string("/proc/self/status").unwrap();
    let pte_kib: usize = status
        .lines()
        .find_map(|l| l.strip_prefix("VmPTE:"))
        .and_then(|v| v.trim().trim_end_matches("kB").trim().parse().ok())
        .unwrap_or(0);
    pages * stack::page_size() + (pte_kib << 10)
}

struct Co {
    ctx: Context,
    back: Context,
    sum: u64,
}

unsafe extern "C" fn body(arg: usize, data: *mut u8) -> ! {
    let c = data as *mut Co;
    // 触碰 8 KiB 栈：模拟一个有若干帧的虚拟线程
    let mut buf = [0u8; 8192];
    for (i, b) in buf.iter_mut().enumerate() {
        *b = (i ^ arg) as u8;
    }
    std::hint::black_box(&mut buf);
    switch(&mut (*c).ctx, &mut (*c).back, 0);
    (*c).sum = buf.iter().map(|&b| b as u64).sum();
    switch(&mut (*c).ctx, &mut (*c).back, 1);
    unreachable!();
}

/// 本进程映射数（Linux 读 /proc/self/maps；其它平台 None）
fn map_count() -> Option<usize> {
    std::fs::read_to_string("/proc/self/maps").ok().map(|s| s.lines().count())
}

#[test]
fn rss_returns_after_hundred_thousand() {
    const N: usize = 100_000;
    // 预热：首个栈、处理器安装、载体信息
    {
        let s = Stack::new().unwrap();
        drop(s);
    }
    // 测试自身的簿记（10⁵ 个 Co + 栈句柄）在测起点前一次分配并写满：起点 / 终点之差只反映栈
    let mut cos: Vec<Co> = (0..N).map(|_| Co { ctx: Context::empty(), back: Context::empty(), sum: 0 }).collect();
    let mut stacks: Vec<Option<Stack>> = (0..N).map(|_| None).collect();
    let maps_start = map_count();
    let start = rss();
    for (i, (co, slot)) in cos.iter_mut().zip(stacks.iter_mut()).enumerate() {
        let st = Stack::new().unwrap();
        let p: *mut Co = co;
        unsafe {
            (*p).ctx = Context::new(&st, body, p.cast());
            assert_eq!(switch(&mut (*p).back, &mut (*p).ctx, i), 0);
        }
        *slot = Some(st);
    }
    let peak = rss();
    let maps_peak = map_count();
    assert_eq!(stack::live_stacks(), N);
    // 10⁵ 栈约 100 块 slab
    assert!(stack::mapped_chunks() <= 256, "slab 块数 {}", stack::mapped_chunks());
    if let (Some(a), Some(b)) = (maps_start, maps_peak) {
        eprintln!("[rava_coro] 映射数 {a} → {b}（{N} 协程挂起，slab {} 块）", stack::mapped_chunks());
        assert!(b <= a + 256, "映射数随存活协程增长：{a} → {b}");
    }
    for co in cos.iter_mut() {
        let p: *mut Co = co;
        unsafe { assert_eq!(switch(&mut (*p).back, &mut (*p).ctx, 0), 1) };
        assert!(co.sum > 0);
    }
    for slot in stacks.iter_mut() {
        *slot = None;
    }
    let end = rss();
    let mib = |b: usize| b as f64 / (1 << 20) as f64;
    eprintln!(
        "[rava_coro] RSS 起点 {:.1} MiB，{N} 协程挂起峰值 {:.1} MiB，全部完成后 {:.1} MiB（池 {} 块）",
        mib(start),
        mib(peak),
        mib(end),
        stack::pooled_stacks()
    );
    assert_eq!(stack::live_stacks(), 0);
    assert!(stack::pooled_stacks() <= stack::pool_limit());
    // 全部空块只留一个作缓冲
    assert!(stack::mapped_chunks() <= 1, "空块未释放：{} 块", stack::mapped_chunks());
    assert!(end <= start + (16 << 20), "RSS 未回落：{:.1} → {:.1} MiB", mib(start), mib(end));
}
