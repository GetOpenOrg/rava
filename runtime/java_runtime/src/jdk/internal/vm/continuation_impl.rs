//! `jdk/internal/vm/Continuation` 的 ACC_NATIVE（类 1，计划 §21.8.1）。其余方法按字节码翻译。
//!
//! 有栈协程：每个已启动的 Continuation 一条协程（`rava_coro` 独立栈 + 上下文）与一块执行上下文
//! （`exec_context`，pin 计数），合为一个协程记录。native 只切换「最内层」一层；嵌套作用域、`yieldInfo`、
//! `parent` 链、`mount` / `unmount` 簿记全部是 `Continuation` 的字节码。
//!
//! **记录的归属**：记录按 Continuation 身份登记在分片侧表中，并持有该 Continuation 的一个引用——挂起的协程栈上
//! `enter` 帧本就持有它，登记期间对象不会释放，身份不会被复用。执行完毕时（在载体上）摘除记录、栈归池。
//!
//! **`tail` 簿记**：字节码以 `tail != null` 判定已启动（`isStarted`）、以 `tail` 链上 `sp == bottom` 判定空
//! （`isEmpty`，`finish` 与 `run` 断言「空 ⇔ 执行完毕」）。首次进入时挂一个空 `StackChunk`；挂起期间 `sp = 1`
//! （非空），运行与执行完毕时 `sp = bottom`。帧本身留在协程栈上，不拷入 chunk（§21.8.2「否决的方向」②）。

use crate::prelude::*;
use super::continuation::Continuation;
use super::{ContinuationScope, StackChunk};
use parking_lot::Mutex;
use crate::exec_context::{self, ExecContext};
use rava_coro::{Context, Stack};
use std::collections::HashMap;

/// 一条协程（repr(C)：执行上下文块为首字段，`doYield` 由当前块地址还原记录）
#[repr(C)]
struct Coroutine {
    exec: ExecContext,
    /// 本记录在侧表中的键（Continuation 身份）
    key: usize,
    cont: Continuation,
    /// 协程挂起时的上下文
    suspended: Context,
    /// 切入方（`enterSpecial` 调用点）的上下文：让出或执行完毕时切回
    carrier: Context,
    /// 执行完毕：`Continuation.enter` 已返回
    finished: bool,
    /// `Continuation.enter` 抛出的异常：切回载体后由 `enterSpecial` 继续抛出（HotSpot 中异常穿过 enterSpecial 帧）
    thrown: Option<JvmError>,
    /// unwind 形态下入口体逃逸的 panic：切回载体后由 `enterSpecial` 在载体栈上继续 unwind（不穿过蹦床）
    panicked: Option<rava_coro::PanicPayload>,
    stack: Option<Stack>,
}

/// 侧表一项（记录只在持有它的执行流或切入它的载体上访问，互斥由 Continuation 的 mounted CAS 保证）
struct Slot(Box<Coroutine>);
// SAFETY: 同一时刻至多一个载体访问一条记录（见上）
unsafe impl Send for Slot {}

const SHARDS: usize = 64;

fn shard(key: usize) -> &'static Mutex<HashMap<usize, Slot>> {
    static TABLE: std::sync::OnceLock<Vec<Mutex<HashMap<usize, Slot>>>> = std::sync::OnceLock::new();
    let table = TABLE.get_or_init(|| (0..SHARDS).map(|_| Mutex::new(HashMap::new())).collect());
    // 对象按 16 字节对齐：去掉低位再取模
    &table[(key >> 4) % SHARDS]
}

fn identity<T: Into<Object>>(obj: T) -> usize {
    obj.into().0.__identity() as usize
}

/// 平台线程耗尽时 JDK 的同一消息（`Thread.start0` 建线程失败）：建栈失败按此抛出，不降级
const STACK_EXHAUSTED: &str =
    "unable to create native thread: possibly out of memory or process/resource limits reached";

/// 协程入口：执行 `Continuation.enter(c, false)`（字节码翻译体：`enter0` → `target.run()`，`finally` 置 `done`），
/// 记下结果后切回载体，不再被切入。函数体经 `catch_entry` 包住：unwind 不得穿过蹦床（§21.8.3 panic 跨栈传播）
unsafe extern "C" fn coroutine_entry(_arg: usize, data: *mut u8) -> ! {
    let co = data as *mut Coroutine;
    match rava_coro::catch_entry(|| Continuation::enter(Clone::clone(&(*co).cont), false)) {
        Ok(Ok(())) => {}
        Ok(Err(e)) => (*co).thrown = Some(e),
        Err(payload) => (*co).panicked = Some(payload),
    }
    (*co).finished = true;
    let mut dead = Context::empty();
    rava_coro::switch(&mut dead, &raw mut (*co).carrier, 0);
    eprintln!("fatal error: 已执行完毕的 Continuation 被再次切入");
    std::process::abort();
}

/// 置 `tail` 的空 / 非空（`tail` 为 null 时无操作：`postYieldCleanup` 只在执行完毕后置 null）
fn set_tail_empty(c: &Continuation, empty: bool) {
    let tail = c.__get_tail();
    if !tail.is_jvm_null() {
        let bottom = tail.__get_bottom();
        tail.__set_sp(if empty { bottom } else { bottom + 1 });
    }
}

impl Continuation {
    /// native `registerNatives()`：HotSpot 登记 enterSpecial / doYield 等入口；原生二进制按名链接 → no-op。
    #[jvm_native]
    pub fn registerNatives() -> Result<()> {
        Ok(())
    }

    /// native `enterSpecial(c, isContinue, isVirtualThread)`：`isContinue = false` 时取一块栈、以
    /// `Continuation.enter(c, false)` 为入口切入；`true` 时切回 `c` 上次 `doYield` 保存的上下文。
    /// `c` 让出或执行完毕时返回；执行完毕时记录摘除、栈归池，`enter` 抛出的异常在此继续抛出，入口体逃逸的
    /// panic（unwind 形态）在此于载体栈上继续 unwind。
    /// `isVirtualThread` 只供 HotSpot 的 JVMTI 通知，这里不用。
    #[jvm_native(unpinned)]
    pub fn enterSpecial(c: Continuation, isContinue: bool, isVirtualThread: bool) -> Result<()> {
        let key = identity(Clone::clone(&c));
        let co: *mut Coroutine = if isContinue {
            match shard(key).lock().get_mut(&key) {
                Some(slot) => &mut *slot.0,
                None => panic!("rava: enterSpecial 继续一个未挂起的 Continuation"),
            }
        } else {
            let stack = Stack::new().map_err(|_| JvmError::out_of_memory(STACK_EXHAUSTED))?;
            let scope = identity(c.__get_scope());
            c.__set_tail(StackChunk::new()?);
            let mut boxed = Box::new(Coroutine {
                exec: ExecContext::with_scope(scope),
                key,
                cont: Clone::clone(&c),
                suspended: Context::empty(),
                carrier: Context::empty(),
                finished: false,
                thrown: None,
                panicked: None,
                stack: None,
            });
            let raw: *mut Coroutine = &mut *boxed;
            // SAFETY: 栈随记录存活至执行完毕；记录（Box）地址不变
            boxed.suspended = unsafe { Context::new(&stack, coroutine_entry, raw as *mut u8) };
            boxed.stack = Some(stack);
            shard(key).lock().insert(key, Slot(boxed));
            raw
        };
        set_tail_empty(&c, true);
        // SAFETY: 记录在侧表中存活至执行完毕；挂载 / 卸载都在本载体上，切回时本帧仍在本 OS 线程
        let finished = unsafe {
            let outer = exec_context::mount(&raw const (*co).exec);
            rava_coro::switch(&raw mut (*co).carrier, &raw mut (*co).suspended, 0);
            exec_context::unmount(&raw const (*co).exec, outer);
            (*co).finished
        };
        if !finished {
            set_tail_empty(&c, false);
            return Ok(());
        }
        let Some(Slot(mut record)) = shard(key).lock().remove(&key) else {
            panic!("rava: 执行完毕的 Continuation 不在侧表中");
        };
        let thrown = record.thrown.take();
        let panicked = record.panicked.take();
        drop(record);
        if let Some(payload) = panicked {
            std::panic::resume_unwind(payload);
        }
        match thrown {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }

    /// native `doYield()`：当前（最内层）Continuation 未被 pin 时保存上下文、切回其 `enterSpecial` 调用点，
    /// 日后被继续时返回 0（可能已在另一载体上）；被 pin 时不切换，返回原因码（2 / 3 / 4）
    #[jvm_native(unpinned)]
    pub fn doYield() -> Result<i32> {
        let ctx = exec_context::current();
        match exec_context::yield_pinned_reason() {
            None => panic!("rava: doYield 不在 Continuation 内"),
            Some(exec_context::PINNED_NONE) => {}
            Some(reason) => return Ok(reason),
        }
        // SAFETY: 非平台块必是某条协程记录的首字段（repr(C)）；切回后本帧不再访问线程局部
        unsafe {
            let co = ctx as *mut Coroutine;
            rava_coro::switch(&raw mut (*co).suspended, &raw mut (*co).carrier, 0);
        }
        Ok(0)
    }

    /// native `pin()`：当前 Continuation 的临界区计数 +1；不在 Continuation 内时无操作
    #[jvm_native(unpinned)]
    pub fn pin() -> Result<()> {
        if exec_context::pin_critical() {
            Ok(())
        } else {
            Err(JvmError::from(crate::java::lang::IllegalStateException::new_str(String::from("pin overflow"))?))
        }
    }

    /// native `unpin()`：临界区计数 −1；计数为 0 时抛 IllegalStateException（HotSpot 同消息）
    #[jvm_native(unpinned)]
    pub fn unpin() -> Result<()> {
        if exec_context::unpin_critical() {
            Ok(())
        } else {
            Err(JvmError::from(crate::java::lang::IllegalStateException::new_str(String::from("pin underflow"))?))
        }
    }

    /// native `isPinned0(scope)`：自最内层向外到第一个 `scope` 匹配的 Continuation，任一层被 pin 即返回原因码，否则 0
    #[jvm_native(unpinned)]
    pub fn isPinned0(scope: ContinuationScope) -> Result<i32> {
        Ok(exec_context::scope_pinned_reason(identity(scope)))
    }
}
