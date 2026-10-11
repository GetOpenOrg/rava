//! 区域整组释放（无 GC 第一步，`docs/plans/2026-10-07-no-gc-memory-model.md` §4.5）。
//!
//! 生成器的逃逸分析证明一段用户代码（区域）内分配的对象在区域结束后不被组外引用，并以 `java_scope!`
//! 包住这段代码。区域内经 `__Obj` 分配路径产生的对象登记进区域日志（日志持有一个强引用）：
//!
//! - **正常出口**（[`__Region::exit`]）：整组释放本区域的日志段——成环的临时对象（stream 管线等）也一并释放；
//! - **异常出口**（`?` / 异常 / panic 穿出块，守卫析构）：只撤销日志的强引用，不整组释放（不会悬空）。
//!
//! 区域栈放在载体槽（`exec_context::region_slot`）：区域内不会让出（`park` / `yield` / `Continuation.run` 都是 native，逃逸分析
//! 命中即不插区域；并行后端没有抢占），一个区域总在同一载体、同一执行流上开始与结束。访问一律经不内联的
//! 函数（同 `exec_context` 的 TLS 地址缓存约定）。
//!
//! 不登记的分配：native 方法体与 `<clinit>` 期间（执行流的 native pin 深度高于进入区域时；驻留表、类镜像、
//! 静态初值等运行时缓存都在其中），以及 [`__Suspend`] 期间（字面量驻留）。

use crate::exec_context::region_slot;
use crate::obj_ref::group::{self, Ops};

/// 一层活动区域
pub(crate) struct Frame {
    /// 本区域日志段的起点
    pub(crate) start: usize,
    /// 进入区域时执行流的 native 帧数：只登记同一深度上的分配
    pub(crate) natives: u32,
}

/// 分配路径的登记钩子：有活动区域、未挂起、且 native 深度与进入区域时相同 → 登记并返回 true
/// （调用方随即为日志加一个强引用）。线程退出期（线程局部已销毁）不登记。
#[inline(never)]
pub(crate) fn __on_alloc(value: *mut u8, ops: &'static Ops) -> bool {
    region_slot(|s| {
        if s.depth.get() == 0 || s.suspended.get() != 0 {
            return false;
        }
        let natives = s.frames.borrow().last().map(|f| f.natives);
        if natives != Some(crate::exec_context::native_depth()) {
            return false;
        }
        s.log.borrow_mut().push((value, ops));
        true
    })
    .unwrap_or(false)
}

/// 区域守卫（`java_scope!` 展开）：正常出口调用 [`__Region::exit`]，其余出口由析构撤销登记
pub struct __Region {
    _not_send: std::marker::PhantomData<*const ()>,
}

impl __Region {
    #[inline(never)]
    pub fn enter() -> __Region {
        let natives = crate::exec_context::native_depth();
        region_slot(|s| {
            let start = s.log.borrow().len();
            s.frames.borrow_mut().push(Frame { start, natives });
            s.depth.set(s.depth.get() + 1);
        })
        .expect("线程退出期不进入区域");
        __Region { _not_send: std::marker::PhantomData }
    }

    /// 弹出本层区域，取走其日志段
    fn take() -> Vec<(*mut u8, &'static Ops)> {
        region_slot(|s| {
            let f = s.frames.borrow_mut().pop().expect("区域栈失衡");
            s.depth.set(s.depth.get() - 1);
            s.log.borrow_mut().split_off(f.start)
        })
        .expect("区域出口时载体槽存活")
    }

    /// 正常出口：整组释放本区域分配的对象
    #[inline(never)]
    pub fn exit(self) {
        std::mem::forget(self);
        let objs = Self::take();
        // SAFETY: 逃逸分析证明区域内分配的对象在区域结束后组外不再引用；块内局部已先于此析构
        unsafe { group::free(&objs) };
    }
}

impl Drop for __Region {
    /// 异常出口：只撤销日志持有的强引用（对象按普通引用计数路径释放，成环的照旧泄漏）
    fn drop(&mut self) {
        let objs = Self::take();
        for (p, ops) in objs {
            // SAFETY: 日志为每个登记对象持有一个强引用
            unsafe { (ops.release)(p) };
        }
    }
}

/// 挂起区域登记（字面量驻留等进入全局表的分配）
pub struct __Suspend(());

impl __Suspend {
    #[inline(never)]
    pub fn new() -> __Suspend {
        let _ = region_slot(|s| s.suspended.set(s.suspended.get() + 1));
        __Suspend(())
    }
}

impl Drop for __Suspend {
    #[inline(never)]
    fn drop(&mut self) {
        let _ = region_slot(|s| s.suspended.set(s.suspended.get() - 1));
    }
}

/// 区域块：`java_scope!({ 语句…; 值 })`。块内局部在区域出口前析构；`?` 穿出块即异常出口
#[macro_export]
macro_rules! java_scope {
    ($body:block) => {{
        let __region = $crate::region::__Region::enter();
        let __value = $body;
        __region.exit();
        __value
    }};
}
