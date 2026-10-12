//! 标记层基础的行为钩子定义（§9.9 K4）：`java_marker` 中 `unsafe extern "Rust"` 声明的
//! 对侧定义（`#[export_name]`）。签名与声明取自同一类型（`java_marker::marker_hooks`），
//! 漂移即链接失败。最终二进制链接两侧（与 `rava_layer` 拆层同一机制）。
//!
//! 阶段 1 起两枚（region 登记 / monitor 摘表）；阶段 3 增全量：VM 异常构造器（构造翻译后
//! 的 Java 异常对象）、释放计深与栈界检查、根类字节码方法体（object_body 的链接期声明在
//! 声明层 crate，本 crate 侧经 `super::` 同 crate 直调）、监视器五入口、类镜像与名字、
//! Java 字符串构造、元数据名字、安全点与字节序位形原语。

use crate::java::lang::ArithmeticException__Methods as _;
use crate::java::lang::ArrayIndexOutOfBoundsException__Methods as _;
use crate::java::lang::ArrayStoreException__Methods as _;
use crate::java::lang::ClassCastException__Methods as _;
use crate::java::lang::CloneNotSupportedException__Methods as _;
use crate::java::lang::Class__Methods as _;
use crate::java::lang\ExceptionInInitializerError__Methods as _;
use crate::java::lang::IndexOutOfBoundsException__Methods as _;
use crate::java::lang::IllegalArgumentException__Methods as _;
use crate::java::lang::IllegalMonitorStateException__Methods as _;
use crate::java::lang::InterruptedException__Methods as _;
use crate::java::lang::NegativeArraySizeException__Methods as _;
use crate::java::lang::NoClassDefFoundError__Methods as _;
use crate::java::lang::NullPointerException__Methods as _;
use crate::java::lang::OutOfMemoryError__Methods as _;
use crate::java::lang::StackOverflowError__Methods as _;
use crate::java::lang::StringIndexOutOfBoundsException__Methods as _;
use crate::java::lang::Throwable__Methods as _;
use crate::java::lang::{Class, Object, String, Throwable};

// ── 阶段 1（obj_ref 分配 / 释放路径）──────────────────────────────────────────

/// 区域整组释放的分配登记（`obj_ref` 分配路径调用）：见 `region::__on_alloc`。
#[export_name = "__rava_marker_region_on_alloc"]
pub fn __region_on_alloc(value: *mut u8, ops: &'static java_marker::obj_ref::group::Ops) -> bool {
    crate::region::__on_alloc(value, ops)
}

/// 对象曾进入监视器，释放最后一个强引用时摘身份侧表条目：见 `monitor::forget`。
#[export_name = "__rava_marker_monitor_forget"]
pub fn __monitor_forget(identity: usize) {
    crate::monitor::forget(identity)
}

// ── 阶段 3：VM 异常构造器（java_marker error.rs 标记面的体）────────────────────

use crate::error::vm_throw;
use java_marker::error::JvmError;

macro_rules! error_hook {
    ($export:literal, $local:ident, |$($arg:ident : $ty:ty),*| $body:expr) => {
        #[export_name = $export]
        pub fn $local($($arg: $ty),*) -> JvmError { $body }
    };
}

error_hook!("__rava_marker_error_null_pointer" __null_pointer, || {
    vm_throw(crate::java::lang::NullPointerException::new())
});
error_hook!("__rava_marker_error_array_index_out_of_bounds" __array_index_out_of_bounds, |index: i32, length: i32| {
    vm_throw(crate::java::lang::ArrayIndexOutOfBoundsException::new_str(String::from(
        format!("Index {} out of bounds for length {}", index, length))))
});
error_hook!("__rava_marker_error_array_index_out_of_bounds_message" __array_index_out_of_bounds_message, |message: std::string::String| {
    vm_throw(crate::java::lang::ArrayIndexOutOfBoundsException::new_str(String::from(message)))
});
error_hook!("__rava_marker_error_index_out_of_bounds" __index_out_of_bounds, |message: std::string::String| {
    vm_throw(crate::java::lang::IndexOutOfBoundsException::new_str(String::from(message)))
});
error_hook!("__rava_marker_error_string_index_out_of_bounds" __string_index_out_of_bounds, |message: std::string::String| {
    vm_throw(crate::java::lang::StringIndexOutOfBoundsException::new_str(String::from(message)))
});
error_hook!("__rava_marker_error_negative_array_size" __negative_array_size, |size: i32| {
    vm_throw(crate::java::lang::NegativeArraySizeException::new_str(String::from(
        format!("{}", size))))
});
error_hook!("__rava_marker_error_arithmetic" __arithmetic, |message: &str| {
    vm_throw(crate::java::lang::ArithmeticException::new_str(String::from(message)))
});
error_hook!("__rava_marker_error_class_cast" __class_cast, |message: std::string::String| {
    vm_throw(crate::java::lang::ClassCastException::new_str(String::from(message)))
});
error_hook!("__rava_marker_error_array_store" __array_store, |value_class: &str| {
    vm_throw(crate::java::lang::ArrayStoreException::new_str(String::from(
        value_class.replace('/', "."))))
});
error_hook!("__rava_marker_error_clone_not_supported" __clone_not_supported, |binary_name: &str| {
    vm_throw(crate::java::lang::CloneNotSupportedException::new_str(String::from(
        binary_name.replace('/', "."))))
});
error_hook!("__rava_marker_error_illegal_monitor_state" __illegal_monitor_state, |message: &str| {
    vm_throw(crate::java::lang::IllegalMonitorStateException::new_str(String::from(message)))
});
error_hook!("__rava_marker_error_illegal_argument" __illegal_argument, |message: &str| {
    vm_throw(crate::java::lang::IllegalArgumentException::new_str(String::from(message)))
});
error_hook!("__rava_marker_error_interrupted" __interrupted, |message: Option<&str>| {
    match message {
        Some(m) => vm_throw(crate::java::lang::InterruptedException::new_str(String::from(m))),
        None => vm_throw(crate::java::lang::InterruptedException::new()),
    }
});
error_hook!("__rava_marker_error_out_of_memory" __out_of_memory, |message: &str| {
    vm_throw(crate::java::lang::OutOfMemoryError::new_str(String::from(message)))
});
error_hook!("__rava_marker_error_stack_overflow" __stack_overflow, || {
    // 栈界检查判定耗尽（`__stack_check`）：在放开的余量区内构造 `StackOverflowError`
    // （HotSpot 黄区同义）。构造途中再次耗尽即余量也不够——与 HotSpot 红区同样按致命错误终止。
    let Some(_yellow) = rava_coro::YellowZone::enter() else {
        eprintln!("fatal error: stack overflow while constructing java.lang.StackOverflowError");
        std::process::abort();
    };
    vm_throw(crate::java::lang::StackOverflowError::new())
});
error_hook!("__rava_marker_error_no_class_def_found" __no_class_def_found, |binary_name: &str| {
    vm_throw(crate::java::lang::NoClassDefFoundError::new_str(String::from(
        format!("Could not initialize class {}", binary_name.replace('/', ".")))))
});
error_hook!("__rava_marker_error_class_not_found_at_resolution" __class_not_found_at_resolution, |internal_name: &str| {
    vm_throw(crate::java::lang::NoClassDefFoundError::new_str(String::from(internal_name.to_string())))
});
error_hook!("__rava_marker_error_in_initializer" __in_initializer, |cause: JvmError| {
    // `<clinit>` 异常收尾（JVMS §5.5 步骤 11）：Error 及其子类原样传播，
    // 其余包装为 ExceptionInInitializerError。
    if cause.is_instance_of("java/lang/Error") {
        return cause;
    }
    let throwable: Throwable = cause.catch_as::<Throwable>();
    vm_throw(crate::java::lang::ExceptionInInitializerError::new_throwable(throwable))
});

/// 未捕获异常报告首行（java_marker `JvmError` 的 Debug / Display 体）。
#[export_name = "__rava_marker_error_describe"]
pub fn __error_describe(e: &JvmError) -> std::string::String {
    crate::error::describe(e)
}

// ── 阶段 3：释放计深 / 栈界检查（handle.rs）──────────────────────────────────

/// 释放一个对象引用（`__Handle` / `Object` 的 Drop）：非最后一个强引用只减计数（调用方已判）；
/// 最后一个按深度就地释放或入队，最外层释放清空队列（原 `handle::release_last`）。
#[export_name = "__rava_marker_release"]
pub fn __release(rc: java_marker::obj_ref::__Obj<dyn java_marker::java::lang::ObjectVTable>) {
    /// 就地释放的最大嵌套深度（超出的对象入待释放队列）
    const RELEASE_MAX_DEPTH: u32 = 32;
    /// 计深释放（原 `handle::release_last`）：线程局部释放槽不可用（线程退出期）时就地释放。
    #[inline(never)]
    fn release_last(rc: java_marker::obj_ref::__Obj<dyn java_marker::java::lang::ObjectVTable>) {
        use crate::exec_context::release_slot;
        // 释放槽不可用（线程退出期）时 rc 随闭包丢弃，就地释放
        let Some(depth) = release_slot(|s| s.depth.get()) else { return };
        if depth >= RELEASE_MAX_DEPTH {
            release_slot(move |s| s.pending.borrow_mut().push(rc));
            return;
        }
        release_slot(|s| s.depth.set(depth + 1));
        drop(rc);
        if depth == 0 {
            // 最外层：逐个释放队列中的对象（其字段再次超深的入队，循环至空）
            while let Some(Some(next)) = release_slot(|s| s.pending.borrow_mut().pop()) {
                drop(next);
            }
        }
        release_slot(|s| s.depth.set(depth));
    }
    release_last(rc)
}

/// 方法入口的栈界检查（`__IfaceRef::enter` 用；生成侧的 `__stack_check` 仍走本 crate 同名函数）。
#[export_name = "__rava_marker_stack_check"]
pub fn __stack_check() -> crate::error::Result<()> {
    crate::__stack_check()
}

// ── 阶段 3：根类字节码方法体（object_body 的 extern 声明在声明层 crate，本 crate 直调）──

#[export_name = "__rava_marker_object_equals"]
pub fn __object_equals(this: &Object, other: Object) -> crate::error::Result<bool> {
    crate::java::lang::object_body::Object__equals_body(this, other)
}

#[export_name = "__rava_marker_object_to_string_std"]
pub fn __object_to_string_std(this: &Object) -> crate::error::Result<std::string::String> {
    crate::java::lang::object_body::Object__toString_body(this)
        .map(|s| std::format!("{}", s))
}

#[export_name = "__rava_marker_object_to_string_java"]
pub fn __object_to_string_java(this: &Object) -> crate::error::Result<String> {
    crate::java::lang::object_body::Object__toString_body(this)
}

#[export_name = "__rava_marker_object_wait"]
pub fn __object_wait(this: &Object, millis: i64, nanos: i32) -> crate::error::Result<()> {
    match (millis, nanos) {
        (0, 0) => crate::java::lang::object_body::Object__wait_body(this),
        (m, 0) => crate::java::lang::object_body::Object__wait_l_body(this, m),
        (m, n) => crate::java::lang::object_body::Object__wait_l_i_body(this, m, n),
    }
}

#[export_name = "__rava_marker_object_finalize"]
pub fn __object_finalize(this: &Object) -> crate::error::Result<()> {
    crate::java::lang::object_body::Object__finalize_body(this)
}

// ── 阶段 3：对象监视器五入口 ──────────────────────────────────────────────────

#[export_name = "__rava_marker_monitor_wait_timeout"]
pub fn __monitor_wait_timeout(identity: usize, millis: i64, nanos: i32) -> crate::error::Result<()> {
    crate::monitor::wait_timeout(identity, millis, nanos)
}
#[export_name = "__rava_marker_monitor_notify"]
pub fn __monitor_notify(identity: usize) -> crate::error::Result<()> {
    crate::monitor::notify(identity)
}
#[export_name = "__rava_marker_monitor_notify_all"]
pub fn __monitor_notify_all(identity: usize) -> crate::error::Result<()> {
    crate::monitor::notify_all(identity)
}
#[export_name = "__rava_marker_monitor_enter"]
pub fn __monitor_enter(identity: usize) -> crate::error::Result<()> {
    crate::monitor::enter(identity)
}
#[export_name = "__rava_marker_monitor_exit"]
pub fn __monitor_exit(identity: usize) -> crate::error::Result<()> {
    crate::monitor::exit(identity)
}

// ── 阶段 3：类镜像 / 类名 / 元数据名字 / 字符串构造 ────────────────────────────

#[export_name = "__rava_marker_class_for_class"]
pub fn __class_for_class(binary_name: &str) -> Class {
    Class::for_class(binary_name)
}
#[export_name = "__rava_marker_class_mirror_hash"]
pub fn __class_mirror_hash(id: *const ()) -> Option<i32> {
    Class::__mirror_hash(id)
}
#[export_name = "__rava_marker_class_name"]
pub fn __class_name(c: &Class) -> crate::error::Result<std::string::String> {
    Ok(std::format!("{}", c.__get_name()))
}
#[export_name = "__rava_marker_class_name_assignable"]
pub fn __class_name_assignable(target: &str, source: &str) -> bool {
    Class::__name_assignable(target, source)
}
#[export_name = "__rava_marker_string_from_str"]
pub fn __string_from_str(s: &str) -> String {
    String::from(s)
}
#[export_name = "__rava_marker_string_from_std"]
pub fn __string_from_std(s: std::string::String) -> String {
    String::from(s)
}
#[export_name = "__rava_marker_meta_java_name"]
pub fn __meta_java_name(binary_name: &str) -> std::string::String {
    crate::meta::java_name(binary_name)
}

// ── 阶段 3：安全点 / 字节序位形（数组系统）────────────────────────────────────

#[export_name = "__rava_marker_safepoint"]
pub fn __safepoint() {
    crate::gil::safepoint()
}
#[export_name = "__rava_marker_native_store_bits"]
pub fn __native_store_bits(dst: &mut [u8], bits: u64) {
    crate::native_memory::store_bits(dst, bits)
}
#[export_name = "__rava_marker_native_load_bits"]
pub fn __native_load_bits(src: &[u8]) -> u64 {
    crate::native_memory::load_bits(src)
}
