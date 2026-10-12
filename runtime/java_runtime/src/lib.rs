#![allow(unused_imports)]
pub mod anno_pool;
pub mod field_reflect;
#[doc(hidden)]
pub mod image_rt;
pub mod region;
pub mod marker_hooks;
// 标记层基础（§9.9 K4 阶段 3）：obj_ref / sync_model / class_desc / field_desc / handle /
// array / java_fmt 移入独立 java_marker crate（本 crate 依赖它），按原模块路径整体再导出
// ——`crate::obj_ref::*` / `crate::array::JArray` 等既有引用路径不变。两个宏
// （`__process_static!` / `__DynFn!`）与 `java_fmt_*` / `_is_jnull` 族同路径再导出。
// error 拆两半：结构与 VM 构造器在 java_marker（`crate::error::{JvmError, Result}` 经
// error.rs 再导出），行为面（describe / report_uncaught / dispatch_uncaught）留本 crate。
// java/lang/object{,_ext,_impl}.rs 也随标记层迁入（crate::java::lang::Object 等路径经
// 生成器对包 mod.rs 注入的 `pub use java_marker::java::lang::object::*;` 保持不变）。
pub use java_marker::{array, class_desc, field_desc, handle, java_fmt, obj_ref, sync_model};
pub use java_marker::java_fmt::{java_fmt_f64, java_fmt_f32, java_fmt_char};
pub use java_marker::{_is_jnull, _is_jnull_ref};
pub use java_marker::__DynFn;
pub use java_marker::__process_static;
pub mod gil;
pub mod reflect_dispatch;
pub mod error;
pub mod exec_context;
pub mod java;
pub mod jdk;
pub mod jdk_resources;
pub mod meta;
mod meta_codec;
pub mod monitor;
pub mod native_memory;
pub mod vm_stack;
pub mod pc_map;
pub mod posix;
pub mod predefined;
pub mod net_posix;
pub mod species_dyn;
pub mod injected_invoker;
pub mod zlib;
pub mod proxy_dyn;
pub mod sun;
pub mod vm_constants;

use crate::java::lang::Shutdown__Methods as _;
pub use array::JArray;
pub use error::{JvmError, Result};
pub use java::lang::Object;

/// `main(String[] args)` 的实参：进程命令行参数（不含程序名），与 `java Main a b` 的
/// `args` 同义。生成的 `pub fn main()` 省略形参（Rust 入口签名固定），方法体引用
/// `args` 时由 codegen 在入口以本函数取值。
pub fn main_args() -> crate::array::JArray<crate::java::lang::String> {
    crate::array::JArray::from(
        std::env::args().skip(1)
            .map(|a| crate::java::lang::String::from(a.as_str()))
            .collect::<Vec<_>>())
}

/// JVM CreateJavaVM：生成的 `main` 第一条语句，进程级终止约定就位。
///
/// Java 异常经 `Result` 传播，不走 Rust unwind；Rust panic 只来自未覆盖的存根与运行时
/// 内部缺陷，一律终止进程。生成工作区的 profile 为 `panic = "abort"`（免除全部 unwind
/// 清理路径），钩子在默认输出（panic 消息 + 回溯提示）之后以退出码 101 退出——与
/// unwind 形态下主线程 panic 的退出码、stderr 文本一致，任何线程 panic 同一出口；
/// `process::exit` 照常冲刷 Rust 标准输出。
pub fn create_java_vm() {
    // 主线程的软件栈界（StackOverflowError 判定，见 __stack_check）
    rava_coro::init_platform_thread();
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        default_hook(info);
        std::process::exit(101);
    }));
}

/// 主线程 `main` 返回后的 VM 收尾（java 启动器 `LEAVE` 同序）：DetachCurrentThread（主线程未捕获异常经
/// `Thread.dispatchUncaughtException` 交处理器）→ DestroyJavaVM（等待全部非守护平台线程终结 → 运行
/// shutdown hook `Shutdown.shutdown`）→ 退出（main 异常时码 1）。
pub fn destroy_java_vm(result: crate::error::Result<()>) {
    let failed = match result {
        Err(e) => {
            match crate::java::lang::Thread::currentThread() {
                Ok(main) => error::dispatch_uncaught(&e, &main),
                Err(_) => error::report_uncaught(&e),
            }
            true
        }
        Ok(()) => false,
    };
    gil::await_non_daemon_threads();
    // JVM DestroyJavaVM：非守护线程全部结束后运行 shutdown hook（Shutdown.shutdown 翻译体）；
    // 其异常与 HotSpot `JavaThread::invoke_shutdown_hooks` 同样静默清除
    let _ = crate::java::lang::Shutdown::shutdown();
    if failed {
        std::process::exit(1);
    }
}

mod jdk_feature_gen {
    include!(concat!(env!("OUT_DIR"), "/jdk_feature.rs"));
}

/// 语料 JDK 特性版本（build.rs 由生成侧的 jdk_feature.txt 生成，缺省 21）。
pub fn jdk_feature() -> u32 {
    jdk_feature_gen::JDK_FEATURE
}

/// 存根出口（不在调用链上的方法体、未覆盖的翻译路径）：panic 消息即入参（`stub: 类.方法:描述符`
/// 等），报告位置经 `#[track_caller]` 落在存根所在处。非泛型冷路径——panic 与格式化代码
/// 全程序一份，存根处只剩一次调用。
#[cold]
#[inline(never)]
#[track_caller]
pub fn __stub(msg: &'static str) -> ! {
    panic!("{msg}")
}

/// 接收者恒为 null 的虚调用点（闭包分析 folds `null_recv`：接收者值集为空，被调方不在调用链上）：
/// 生成器不翻译调用，发射 `return Err(__null_recv(&recv, "被调方"))`。接收者确为 null → 与 JVM 同样
/// 抛 NullPointerException；不为 null 说明分析器漏了写入来源（未建模的写入使值集偏小）——panic 报出
/// 违约点，不把非 null 接收者静默当作 NPE。
#[cold]
#[inline(never)]
#[track_caller]
pub fn __null_recv<T: Clone + Into<Object>>(recv: &T, callee: &'static str) -> error::JvmError {
    if _is_jnull_ref(recv) {
        return error::JvmError::null_pointer();
    }
    panic!("null_recv 违约：{callee} 的接收者非 null（闭包分析判定恒为 null）")
}

/// 解析失败点（闭包分析 folds `no_class`：指令解析的类不在类路径上）：生成器不翻译该指令，发射
/// `return Err(__no_class_def("类内部名"))`——与 JVM 在该指令处解析失败同样抛 NoClassDefFoundError，
/// 覆盖它的异常处理器照常捕获（`catch (Throwable)` / `catch (NoClassDefFoundError)` 的可选依赖探测）。
#[cold]
#[inline(never)]
pub fn __no_class_def(internal_name: &'static str) -> error::JvmError {
    error::JvmError::class_not_found_at_resolution(internal_name)
}

/// 方法入口的栈界检查（宏在每个返回 `Result` 的 Java 方法入口注入 `__stack_check()?;`）：当前执行流
/// （平台线程或虚拟线程的协程栈）的栈指针低于软件栈界即抛 `StackOverflowError`（§21.8.2「溢出检测」）。
/// 快路径是一次不内联的栈界比较；构造异常在冷路径。
#[inline(always)]
pub fn __stack_check() -> error::Result<()> {
    if rava_coro::stack_exhausted() {
        return Err(__stack_overflow());
    }
    Ok(())
}

/// 实例方法入口（宏注入 `__enter(self.__r.is_none())?;`）：空接收者抛 NullPointerException（JVMS §6.5
/// invokevirtual / invokespecial / invokeinterface，先于建帧），否则做栈界检查（同 `__stack_check`）。
#[inline(always)]
pub fn __enter(is_null: bool) -> error::Result<()> {
    if is_null {
        return Err(__null_pointer());
    }
    __stack_check()
}

#[cold]
#[inline(never)]
fn __null_pointer() -> error::JvmError {
    error::JvmError::null_pointer()
}

#[cold]
#[inline(never)]
fn __stack_overflow() -> error::JvmError {
    error::JvmError::stack_overflow()
}

/// 不返回的调用点之后（闭包分析 folds `noreturn_calls`：唯一目标没有任何返回路径）：
/// 被调方正常返回即违反分析结论，panic 报出违约点，不继续执行已删去的代码。
#[cold]
#[inline(never)]
#[track_caller]
pub fn __noreturn(callee: &'static str) -> ! {
    panic!("noreturn 违约：{callee} 正常返回（闭包分析判定不返回）")
}

/// 整数除法/取余（JVMS §6.5 idiv / irem / ldiv / lrem）：
/// 除数为 0 抛 `ArithmeticException("/ by zero")`；`MIN / -1` 按二进制补码回绕。
pub fn idiv(a: i32, b: i32) -> error::Result<i32> {
    if b == 0 { return Err(error::JvmError::arithmetic("/ by zero")); }
    Ok(a.wrapping_div(b))
}
pub fn irem(a: i32, b: i32) -> error::Result<i32> {
    if b == 0 { return Err(error::JvmError::arithmetic("/ by zero")); }
    Ok(a.wrapping_rem(b))
}
pub fn ldiv(a: i64, b: i64) -> error::Result<i64> {
    if b == 0 { return Err(error::JvmError::arithmetic("/ by zero")); }
    Ok(a.wrapping_div(b))
}
pub fn lrem(a: i64, b: i64) -> error::Result<i64> {
    if b == 0 { return Err(error::JvmError::arithmetic("/ by zero")); }
    Ok(a.wrapping_rem(b))
}

/// getfield / putfield 的接收者判空（JVMS §6.5：objectref 为 null 抛 NullPointerException，
/// FS-M7）。生成器对非 `this` 接收者发射 `recv.__nn()?.__get_x()`——null 接收者以可捕获的
/// `Err(NPE)` 完成，而不是读到 null 实例的默认字段值。实现对象为全部 `ObjectVTable`
/// 载体（宏生成的 wrapper、手写边界类、数组）；`Object` 另有同名固有方法。
pub trait __NonNull {
    fn __nn(&self) -> Result<&Self>;
}

impl<T: crate::java::lang::ObjectVTable> __NonNull for T {
    #[inline]
    fn __nn(&self) -> Result<&Self> {
        if self.is_jvm_null() { Err(JvmError::null_pointer()) } else { Ok(self) }
    }
}

impl Object {
    /// `Object` 接收者的判空（见 [`__NonNull`]）。
    #[inline]
    pub fn __nn(&self) -> Result<&Self> {
        if self.0.is_jvm_null() { Err(JvmError::null_pointer()) } else { Ok(self) }
    }
}

/// S-17: typeSwitch String 常量标签判定（SwitchBootstraps 语义 `label.equals(selector)`：
/// selector 非 String 恒 false）。经运行时类判定 + `__obj_str` 内容比较，不依赖闭包内
/// 是否生成 java/lang/String 类型定义（K-2 规则：运行时手写层不引用转译类）。
#[inline]
pub fn _ts_str_label_eq(label: &str, sel: &Object) -> bool {
    sel.is_instance_of("java/lang/String") && sel.0.__obj_str() == label
}

/// S-17: typeSwitch Integer 常量标签判定（selector instanceof Integer 且值相等；
/// Integer 的 `__obj_str` 为十进制表示，与 i32 的 Display 逐字符一致）。
#[inline]
pub fn _ts_int_label_eq(label: i32, sel: &Object) -> bool {
    sel.is_instance_of("java/lang/Integer") && sel.0.__obj_str() == std::format!("{}", label)
}


/// MutexHolder：包装 parking_lot::ReentrantMutex，为 InternalLock 等需要 PartialEq 的结构体使用
#[derive(Clone)]
pub struct MutexHolder(pub std::sync::Arc<parking_lot::ReentrantMutex<()>>);
impl Default for MutexHolder {
    fn default() -> Self { Self(std::sync::Arc::new(parking_lot::ReentrantMutex::new(()))) }
}
impl PartialEq for MutexHolder {
    fn eq(&self, other: &Self) -> bool { std::sync::Arc::ptr_eq(&self.0, &other.0) }
}

// ── 类初始化钩子（JVM 反射路径强制初始化的数据面）──────────────────────────
//
// JVM 语义：`Class` 字面量（ldc）不触发初始化（JVMS §5.5 无 passivity 例外），
// 但反射式消费方（`getEnumConstantsShared`、`Class.getEnumConstants`、
// `Enum.valueOf(Class, name)` 等）会强制目标类初始化后再读常量。名字 token
// `Class::for_class` 与用户类的 `__class_init` 之间没有通道——生成项目在 main
// 启动时按语料登记钩子（枚举形态类），运行时按名代调。

pub type ClassInitHook = crate::sync_model::__Shared<crate::__DynFn!(() -> Result<()>)>;

crate::__process_static! {
    static CLASS_INIT_HOOKS: crate::sync_model::__RefSlot<
        std::collections::HashMap<std::string::String, ClassInitHook>
    > = crate::sync_model::__RefSlot::new(std::collections::HashMap::new());
}

/// 生成项目 main 启动时登记类初始化钩子。`binary_name` 归一为点分形态（与 `Class` 对象承载的名字一致）。
pub fn register_class_init_hooks(hooks: &[(&str, ClassInitHook)]) {
    CLASS_INIT_HOOKS.with(|h| {
        for (name, hook) in hooks {
            h.put(name.replace('/', "."), Clone::clone(hook));
        }
    });
}

/// 按名强制类初始化（JVM 反射路径语义）。未登记（非语料类 / 手写边界类）→
/// no-op；已初始化 → `__class_init` 状态机立即返回；初始化抛错 → 原样传播
/// （erroneous 状态与异常包装由状态机承载）。
pub fn ensure_class_initialized(binary_name: &str) -> Result<()> {
    let hook = CLASS_INIT_HOOKS.with(|h| {
        let h = h.borrow();
        h.get(binary_name.replace('/', ".").as_str()).map(Clone::clone)
    });
    match hook {
        Some(f) => f(),
        None => Ok(()),
    }
}

/// 类是否尚待初始化（`Unsafe.shouldBeInitialized` 语义：未完成初始化——含初始化中、曾失败）。只对登记了
/// 初始化钩子的类作答：未登记的类不会经 `ensure_class_initialized` 初始化，答「无须初始化」与之一致
pub fn class_needs_initialization(binary_name: &str) -> bool {
    let dotted = binary_name.replace('/', ".");
    let hooked = CLASS_INIT_HOOKS.with(|h| h.borrow().contains_key(dotted.as_str()));
    hooked && !gil::clinit_done(&dotted.replace('.', "/"))
}

/// prelude：生成代码用 `use java_runtime::prelude::*;` 引入所有必要符号。
pub mod prelude {
    #![allow(unused_imports)]
    pub use super::array::JArray;
    pub use super::class_desc::__ClassDesc;
    pub use super::handle::{__Handle, __IfaceRef};
    pub use super::obj_ref::__Obj;
    pub use super::java::lang::__TypedNull;
    pub use super::error::{JvmError, Result};
    pub use super::java::lang::Object;
    pub use super::java::lang::ObjectVTable;
    pub use super::field_desc::{__FieldDesc, __FieldKind, __RefAccess};
    pub use super::field_reflect::__StaticFieldDesc;
    pub use super::java::lang::{__class_from_object, __erased_view, __iface_missing};
    pub use super::java::lang::Object__clone_base;
    pub use super::java::lang::String;
    pub use super::sync_model::{__AnyRef, __PrimCell, __RefField, __RefSlot, __Shared, __ThreadSafe};
    pub use crate::__DynFn;
    pub use crate::__process_static;
    pub use crate::reflect_dispatch::__caller_sensitive;
    pub use crate::proxy_dyn::__ProxyRet;
    pub use crate::meta::{is_subtype_of as __is_subtype_of, java_name as __java_name};
    pub use crate::gil::{safepoint as __safepoint, ClinitEnter as __ClinitEnter,
                         clinit_enter as __clinit_enter, clinit_exit as __clinit_exit,
                         class_init as __class_init_run};
    pub use super::_is_jnull;
    pub use super::_is_jnull_ref;
    pub use super::__NonNull;
    pub use super::_ts_str_label_eq;
    pub use super::_ts_int_label_eq;
    pub use super::{idiv, irem, ldiv, lrem};
    pub use super::{__stub, __null_recv, __noreturn, __no_class_def, __stack_check, __enter};

    pub use super::java_fmt_f64;
    pub use super::java_fmt_f32;
    pub use super::monitor::{MonitorGuard, class_monitor};
    pub use crate::sync_model::__Shared as Rc;
    pub use crate::sync_model::__RefSlot as RefCell;
    pub use super::MutexHolder;
    pub use rava_macros::{jvm_native, jvm_boundary, jvm_ext};
    pub use rava_macros::java_try;
    pub use crate::java_scope;
}
