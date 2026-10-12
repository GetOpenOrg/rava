//! Java 异常的 Rust 载体——行为面（§9.9 K4 阶段 3：结构与签名安全的成员已入
//! `java_marker::error`，此处再导出并承载引用生成类的未捕获报告；名字保留为
//! `error` 模块，vmrules 的 fns 名单不受影响）。
//!
//! `describe` / `report_uncaught` / `dispatch_uncaught` 的签名引用 `Throwable` /
//! `Thread`（字节码翻译类），不能进标记层；`dispatch_uncaught` 由方法形态改为
//! 自由函数（三个手写调用点随迁），与 `report_uncaught` 同形。

use crate::java::lang::Throwable__Methods as _;
use crate::java::lang::Thread__Methods as _;
use crate::java::lang::{Object, Throwable, Thread};

pub use java_marker::error::{JvmError, Result};

/// VM 构造异常对象：构造器自身失败时，传播构造过程中抛出的异常（与 JVM 行为一致）。
pub(crate) fn vm_throw<T: Into<Object>>(built: Result<T>) -> JvmError {
    // 诊断：RUST_BACKTRACE=full 时打印 VM 抛出点（NPE / 越界 / CCE …）的 Rust 回溯
    // （定位翻译体内的抛出位置）
    // 诊断开关进程内只读一次（每次抛出都查环境变量是热路径上的分配与系统调用）
    static FULL_BACKTRACE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if *FULL_BACKTRACE.get_or_init(|| std::env::var("RUST_BACKTRACE").is_ok_and(|v| v == "full")) {
        eprintln!("[vm-throw]\n{}", std::backtrace::Backtrace::force_capture());
    }
    match built {
        Ok(exception) => JvmError::__wrap(exception.into()),
        Err(nested) => nested,
    }
}

/// Java 默认未捕获异常处理器的首行：`java.lang.Xxx: message`（与 Throwable.toString 同构）。
/// 行为面（`JvmError::describe` 的钩子定义同此）：is_instance_of 判 Throwable 后取 message。
pub fn describe(e: &JvmError) -> std::string::String {
    let class_name = e.class_name().replace('/', ".");
    let message = if e.is_instance_of("java/lang/Throwable") {
        let throwable: Throwable = e.catch_as::<Throwable>();
        match throwable.getMessage() {
            Ok(m) if !m.is_jvm_null() => Some(format!("{}", m)),
            _ => None,
        }
    } else {
        None
    };
    match message {
        Some(m) => format!("{}: {}", class_name, m),
        None => class_name,
    }
}

/// 引导序列（`image_rt::run`）失败出口：与 HotSpot `vm_exit_during_initialization` 同构——
/// 引导期 Java 层（System.err、printStackTrace）未必就绪，由 VM 原生打印：tty（标准输出）先打
/// `Error occurred during initialization of VM`，再按 `java_lang_Throwable::print_stack_trace`
/// 打异常描述、`\tat` 帧（取自 `backtrace` 载体）与 `Caused by:` 链，进程退出码 1。
pub fn report_uncaught(e: &JvmError) -> ! {
    use std::io::Write;
    let mut out = std::string::String::from("Error occurred during initialization of VM\n");
    let mut cur = Clone::clone(e);
    for depth in 0..16 {
        if depth > 0 {
            out.push_str("Caused by: ");
        }
        out.push_str(&describe(&cur));
        out.push('\n');
        if !cur.is_instance_of("java/lang/Throwable") {
            break;
        }
        let t: Throwable = cur.catch_as::<Throwable>();
        for frame in Throwable::__vm_frame_lines(&t) {
            out.push_str("\tat ");
            out.push_str(&frame);
            out.push('\n');
        }
        let Ok(next) = t.getCause() else { break };
        let next_obj = Object::from(Clone::clone(&next));
        if next_obj.0.is_jvm_null() || next_obj == Object::from(Clone::clone(&t)) {
            break;
        }
        cur = JvmError::from(next);
    }
    out.push('\n');
    let mut stdout = std::io::stdout().lock();
    let _ = stdout.write_all(out.as_bytes());
    let _ = stdout.flush();
    std::process::exit(1)
}

/// 线程未捕获异常（HotSpot `JavaThread::exit`）：虚调用 Java `Thread.dispatchUncaughtException(e)`——
/// 经线程的 UncaughtExceptionHandler（缺省为线程组 → 默认处理器 → `System.err` 上
/// `Exception in thread "名" ` + `printStackTrace`，带 `at` 帧与 `Caused by:` 链）。处理器自身抛出时
/// 与 HotSpot 同样由 VM 打印一行后清除。不退出进程：JVM 中只终结该线程。
/// （§9.9 K4 阶段 3：自 `JvmError` 的方法形态改为自由函数——签名引用 `Thread` 不能进标记层。）
pub fn dispatch_uncaught(e: &JvmError, thread: &Thread) {
    let throwable: Throwable = e.catch_as::<Throwable>();
    if let Err(e2) = thread.dispatchUncaughtException(throwable) {
        eprint!(
            "\nException: {} thrown from the UncaughtExceptionHandler in thread \"{}\"\n",
            e2.class_name().replace('/', "."),
            thread.__get_name()
        );
    }
}
