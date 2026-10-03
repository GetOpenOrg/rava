//! pin 计数与判定（a3-T2）：CRITICAL_SECTION 公开 API 不可达、无 e2e，正确性由本测试保证（计划 §21.8.4 T2 行）。
//! 末尾一项在真实协程上验证「被 pin 不切换、解除后让出并在载体上恢复」的完整往返。

use rava_coro::pins::*;
use rava_coro::{switch, Context, Stack};

const SCOPE_A: usize = 0x1000;
const SCOPE_B: usize = 0x2000;

#[test]
fn root_flow_is_never_pinned_and_pin_is_noop() {
    let root = Pins::platform();
    assert_eq!(root.yield_reason(), None);
    assert!(root.pin());
    assert!(root.unpin());
    assert!(root.unpin(), "根执行流 unpin 无操作，不报下溢");
    root.monitor_entered();
    root.native_entered();
    assert_eq!(root.scope_reason(SCOPE_A), PINNED_NONE);
}

#[test]
fn critical_section_counts_and_underflow() {
    let root = Pins::platform();
    let flow = Pins::with_scope(SCOPE_A);
    unsafe { flow.mount(&root) };
    assert_eq!(flow.yield_reason(), Some(PINNED_NONE));
    assert!(!flow.unpin(), "计数为 0 时 unpin 即下溢");
    assert!(flow.pin());
    assert!(flow.pin());
    assert_eq!(flow.yield_reason(), Some(PINNED_CRITICAL_SECTION));
    assert!(flow.unpin());
    assert_eq!(flow.yield_reason(), Some(PINNED_CRITICAL_SECTION));
    assert!(flow.unpin());
    assert_eq!(flow.yield_reason(), Some(PINNED_NONE));
    assert!(!flow.unpin());
    flow.unmount();
}

#[test]
fn reason_order_critical_then_monitor_then_native() {
    let root = Pins::platform();
    let flow = Pins::with_scope(SCOPE_A);
    unsafe { flow.mount(&root) };
    flow.native_entered();
    assert_eq!(flow.yield_reason(), Some(PINNED_NATIVE));
    flow.monitor_entered();
    assert_eq!(flow.yield_reason(), Some(PINNED_MONITOR));
    assert!(flow.pin());
    assert_eq!(flow.yield_reason(), Some(PINNED_CRITICAL_SECTION));
    assert!(flow.unpin());
    flow.monitor_exited();
    flow.native_exited();
    assert_eq!(flow.yield_reason(), Some(PINNED_NONE));
}

#[test]
fn monitors_held_by_outer_flows_pin_inner_yield() {
    // HotSpot held_monitor_count 按线程计：外层持锁时内层让出同样 MONITOR
    let root = Pins::platform();
    let outer = Pins::with_scope(SCOPE_A);
    let inner = Pins::with_scope(SCOPE_B);
    unsafe {
        outer.mount(&root);
        inner.mount(&outer);
    }
    root.monitor_entered();
    assert_eq!(inner.yield_reason(), Some(PINNED_MONITOR));
    root.monitor_exited();
    outer.monitor_entered();
    assert_eq!(inner.yield_reason(), Some(PINNED_MONITOR));
    outer.monitor_exited();
    // 外层的临界区与 native 帧不影响内层让出（只冻结最内层）
    assert!(outer.pin());
    outer.native_entered();
    assert_eq!(inner.yield_reason(), Some(PINNED_NONE));
}

#[test]
fn scope_walk_stops_at_matching_scope() {
    let root = Pins::platform();
    let outer = Pins::with_scope(SCOPE_A);
    let inner = Pins::with_scope(SCOPE_B);
    unsafe {
        outer.mount(&root);
        inner.mount(&outer);
    }
    // 外层被 pin：只查到内层作用域时不受影响，查到外层作用域时命中
    assert!(outer.pin());
    assert_eq!(inner.scope_reason(SCOPE_B), PINNED_NONE);
    assert_eq!(inner.scope_reason(SCOPE_A), PINNED_CRITICAL_SECTION);
    assert!(outer.unpin());
    outer.native_entered();
    assert_eq!(inner.scope_reason(SCOPE_B), PINNED_NONE);
    assert_eq!(inner.scope_reason(SCOPE_A), PINNED_NATIVE);
    outer.native_exited();
    // 外层之外（根）持锁：越过内层后按外层的「之外持锁数」命中 MONITOR
    root.monitor_entered();
    assert_eq!(inner.scope_reason(SCOPE_A), PINNED_MONITOR);
    root.monitor_exited();
    // 不存在的作用域：走到根为止，未被 pin
    assert_eq!(inner.scope_reason(0xdead0), PINNED_NONE);
    // 最内层自身的计数优先
    inner.native_entered();
    assert_eq!(inner.scope_reason(SCOPE_B), PINNED_NATIVE);
    assert!(inner.pin());
    assert_eq!(inner.scope_reason(SCOPE_B), PINNED_CRITICAL_SECTION);
}

#[test]
fn unmount_detaches_from_outer() {
    let root = Pins::platform();
    let flow = Pins::with_scope(SCOPE_A);
    unsafe { flow.mount(&root) };
    root.monitor_entered();
    assert_eq!(flow.yield_reason(), Some(PINNED_MONITOR));
    flow.unmount();
    assert!(flow.is_root());
    // 换载体重新挂载：外层不再持锁
    let other = Pins::platform();
    unsafe { flow.mount(&other) };
    assert_eq!(flow.yield_reason(), Some(PINNED_NONE));
}

// ── 真实协程往返：被 pin 不切换，解除后让出 ──────────────────────────────────

struct Flow {
    pins: Pins,
    suspended: Context,
    carrier: Context,
    log: Vec<&'static str>,
    done: bool,
}

/// doYield 的同构实现：被 pin 返回原因码不切换，否则切回载体
unsafe fn do_yield(f: *mut Flow) -> i32 {
    match (*f).pins.yield_reason().expect("在协程内") {
        PINNED_NONE => {
            switch(&raw mut (*f).suspended, &raw mut (*f).carrier, 0);
            0
        }
        r => r,
    }
}

unsafe extern "C" fn body(_arg: usize, data: *mut u8) -> ! {
    let f = data as *mut Flow;
    assert!((*f).pins.pin());
    assert_eq!(do_yield(f), PINNED_CRITICAL_SECTION);
    (*f).log.push("pinned-cs");
    assert!((*f).pins.unpin());
    (*f).pins.monitor_entered();
    assert_eq!(do_yield(f), PINNED_MONITOR);
    (*f).log.push("pinned-monitor");
    (*f).pins.monitor_exited();
    (*f).pins.native_entered();
    assert_eq!(do_yield(f), PINNED_NATIVE);
    (*f).log.push("pinned-native");
    (*f).pins.native_exited();
    assert_eq!(do_yield(f), PINNED_NONE);
    (*f).log.push("resumed");
    (*f).done = true;
    let mut dead = Context::empty();
    switch(&mut dead, &raw mut (*f).carrier, 0);
    unreachable!();
}

#[test]
fn pinned_yield_does_not_switch() {
    let root = Pins::platform();
    let stack = Stack::new().expect("栈");
    let mut flow = Box::new(Flow {
        pins: Pins::with_scope(SCOPE_A),
        suspended: Context::empty(),
        carrier: Context::empty(),
        log: Vec::new(),
        done: false,
    });
    let raw: *mut Flow = &mut *flow;
    unsafe {
        flow.suspended = Context::new(&stack, body, raw as *mut u8);
        (*raw).pins.mount(&root);
        switch(&raw mut (*raw).carrier, &raw mut (*raw).suspended, 0);
        (*raw).pins.unmount();
    }
    // 三次被 pin 都留在协程内继续执行，第四次才真正让出
    assert_eq!(flow.log, ["pinned-cs", "pinned-monitor", "pinned-native"]);
    assert!(!flow.done);
    unsafe {
        (*raw).pins.mount(&root);
        switch(&raw mut (*raw).carrier, &raw mut (*raw).suspended, 0);
        (*raw).pins.unmount();
    }
    assert_eq!(flow.log, ["pinned-cs", "pinned-monitor", "pinned-native", "resumed"]);
    assert!(flow.done);
    drop(flow);
    drop(stack);
}
