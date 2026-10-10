//! Java 异常的 Rust 载体。
//!
//! 所有翻译方法返回 `Result<T>`；`Err` 侧只有一种形态：**被抛出的 Throwable 对象本身**。
//! `athrow` 把栈顶对象装入 `JvmError`，异常表匹配按对象的运行时类（含子类关系）进行，
//! VM 自身抛出的异常（空引用、数组越界、类初始化失败等）同样构造翻译后的 Java 异常对象，
//! 因此 catch、`getMessage()`、未捕获报告对用户异常与 VM 异常完全一致。
//!
//! 本文件构造 VM 异常对象的 fn 是 VM 规则的落地（闭包分析器 `engine/vmrules.rs`：规则 ID + JVMS 依据，
//! 由可达字节码中的触发指令激活），其中对已翻译 Java 方法的调用按手写体调用点推断。

use crate::java::lang::{Object, ObjectVTable, String, Throwable};

/// 被抛出的 Java 异常对象（运行时类任意，静态视图为 `Object`）。
#[derive(Clone)]
pub struct JvmError {
    thrown: Object,
}

pub type Result<T> = std::result::Result<T, JvmError>;

/// `throw obj`：任何可转为 `Object` 的引用都可以被抛出。
impl<T: Into<Object>> From<T> for JvmError {
    fn from(thrown: T) -> Self {
        let thrown: Object = thrown.into();
        if thrown.0.is_jvm_null() || crate::_is_jnull(&thrown) {
            // JVMS athrow：objectref 为 null 时抛 NullPointerException
            return JvmError::null_pointer();
        }
        JvmError { thrown }
    }
}

/// VM 构造异常对象：构造器自身失败时，传播构造过程中抛出的异常（与 JVM 行为一致）。
fn vm_throw<T: Into<Object>>(built: Result<T>) -> JvmError {
    // 诊断：RUST_BACKTRACE=full 时打印 VM 抛出点（NPE / 越界 / CCE …）的 Rust 回溯
    // （定位翻译体内的抛出位置）
    if std::env::var("RUST_BACKTRACE").map(|v| v == "full").unwrap_or(false) {
        eprintln!("[vm-throw]\n{}", std::backtrace::Backtrace::force_capture());
    }
    match built {
        Ok(exception) => JvmError { thrown: exception.into() },
        Err(nested) => nested,
    }
}

impl JvmError {
    /// 被抛出的异常对象。
    pub fn thrown(&self) -> &Object {
        &self.thrown
    }

    /// 运行时类的 binary name（如 `java/lang/NullPointerException`）。
    pub fn class_name(&self) -> &'static str {
        self.thrown.0.__class_name()
    }

    /// 异常表匹配：运行时类是否是 `binary_name` 或其子类。
    pub fn is_instance_of(&self, binary_name: &str) -> bool {
        self.thrown.0.is_instance_of(binary_name)
    }

    /// catch 绑定：按运行时类把异常对象还原为 catch 声明类型 `T`。仅在 `is_instance_of`
    /// （catch 类型名）成立后调用。
    ///
    /// 两条还原路径按序尝试：
    ///   1. 快路径：T 即 Object（类型擦除位置）
    ///   2. 擦除重建（A-1）：`From<Object> for X<A>` 任意 A 成立——运行时类是 T 或其子类
    ///      （描述符 display 判定），经擦除 vtable / 存储部件重建 T 视图：与运行时类同一对象、
    ///      vtable 经 supertrait 上转（祖先视图、以祖先 wrapper 形态流转后重抛均同）
    pub fn catch_as<T: std::any::Any + Clone + From<Object>>(&self) -> T {
        if let Some(same) = (&self.thrown as &dyn std::any::Any).downcast_ref::<T>() {
            return Clone::clone(same);
        }
        T::from(Clone::clone(&self.thrown))
    }

    /// catch-any 绑定（异常表 catch_type = 0）：athrow 操作数的静态类型即 Throwable。
    pub fn catch_any(&self) -> crate::java::lang::Throwable {
        self.catch_as::<crate::java::lang::Throwable>()
    }

    // ── VM 抛出的异常 ────────────────────────────────────────────────────────

    pub fn null_pointer() -> Self {
        vm_throw(crate::java::lang::NullPointerException::new())
    }

    pub fn array_index_out_of_bounds(index: i32, length: i32) -> Self {
        vm_throw(crate::java::lang::ArrayIndexOutOfBoundsException::new_str(String::from(
            format!("Index {} out of bounds for length {}", index, length))))
    }

    pub fn array_index_out_of_bounds_message(message: std::string::String) -> Self {
        vm_throw(crate::java::lang::ArrayIndexOutOfBoundsException::new_str(String::from(message)))
    }

    pub fn index_out_of_bounds(message: std::string::String) -> Self {
        vm_throw(crate::java::lang::IndexOutOfBoundsException::new_str(String::from(message)))
    }

    pub fn string_index_out_of_bounds(message: std::string::String) -> Self {
        vm_throw(crate::java::lang::StringIndexOutOfBoundsException::new_str(String::from(message)))
    }

    pub fn negative_array_size(size: i32) -> Self {
        vm_throw(crate::java::lang::NegativeArraySizeException::new_str(String::from(
            format!("{}", size))))
    }

    pub fn arithmetic(message: &str) -> Self {
        vm_throw(crate::java::lang::ArithmeticException::new_str(String::from(message)))
    }

    pub fn class_cast(message: std::string::String) -> Self {
        vm_throw(crate::java::lang::ClassCastException::new_str(String::from(message)))
    }

    /// aastore 存储检查失败（JLS §10.5 / S-4 数组协变）：值与数组元素类型不赋值兼容。
    /// 消息为值的运行时类全限定名（与 HotSpot 一致）。
    pub fn array_store(value_class: &str) -> Self {
        vm_throw(crate::java::lang::ArrayStoreException::new_str(String::from(
            value_class.replace('/', "."))))
    }

    /// Object.clone()：运行时类未实现 Cloneable。消息为类的全限定名（与 HotSpot 一致）。
    pub fn clone_not_supported(binary_name: &str) -> Self {
        vm_throw(crate::java::lang::CloneNotSupportedException::new_str(String::from(
            binary_name.replace('/', "."))))
    }

    pub fn illegal_monitor_state(message: &str) -> Self {
        vm_throw(crate::java::lang::IllegalMonitorStateException::new_str(String::from(message)))
    }

    /// `Object.wait` 参数校验（HotSpot JVM_MonitorWait 同序：先于持有检查）。
    pub fn illegal_argument(message: &str) -> Self {
        vm_throw(crate::java::lang::IllegalArgumentException::new_str(String::from(message)))
    }

    /// 阻塞原语被中断（`Thread.sleep` / `Object.wait`，HotSpot 同消息：sleep 带
    /// "sleep interrupted"，wait 无消息）。
    pub fn interrupted(message: Option<&str>) -> Self {
        match message {
            Some(m) => vm_throw(crate::java::lang::InterruptedException::new_str(String::from(m))),
            None => vm_throw(crate::java::lang::InterruptedException::new()),
        }
    }

    pub fn out_of_memory(message: &str) -> Self {
        vm_throw(crate::java::lang::OutOfMemoryError::new_str(String::from(message)))
    }

    /// 栈界检查判定耗尽（`__stack_check`）：在放开的余量区内构造 `StackOverflowError`（HotSpot 黄区同义）。
    /// 构造途中再次耗尽即余量也不够——与 HotSpot 红区同样按致命错误终止。
    pub fn stack_overflow() -> Self {
        let Some(_yellow) = rava_coro::YellowZone::enter() else {
            eprintln!("fatal error: stack overflow while constructing java.lang.StackOverflowError");
            std::process::abort();
        };
        vm_throw(crate::java::lang::StackOverflowError::new())
    }

    /// 类处于 erroneous 状态后的再次主动使用（JVMS §5.5 步骤 5）。
    pub fn no_class_def_found(binary_name: &str) -> Self {
        vm_throw(crate::java::lang::NoClassDefFoundError::new_str(String::from(
            format!("Could not initialize class {}", binary_name.replace('/', ".")))))
    }

    /// `<clinit>` 异常收尾（JVMS §5.5 步骤 11）：Error 及其子类原样传播，
    /// 其余包装为 ExceptionInInitializerError。
    pub fn in_initializer(cause: JvmError) -> Self {
        if cause.is_instance_of("java/lang/Error") {
            return cause;
        }
        let throwable: Throwable = cause.catch_as::<Throwable>();
        vm_throw(crate::java::lang::ExceptionInInitializerError::new_throwable(throwable))
    }

    // ── 未捕获异常报告 ───────────────────────────────────────────────────────

    /// Java 默认未捕获异常处理器的首行：`java.lang.Xxx: message`（与 Throwable.toString 同构）。
    pub fn describe(&self) -> std::string::String {
        let class_name = self.class_name().replace('/', ".");
        let message = if self.is_instance_of("java/lang/Throwable") {
            let throwable: Throwable = self.catch_as::<Throwable>();
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

    /// main 线程未捕获异常出口：按 Java 格式输出到 stderr，进程退出码 1。
    /// `RUST_BACKTRACE` 设为非 `0` 时附打印 Rust backtrace（定位抛出点）。
    pub fn report_uncaught(&self) -> ! {
        self.report_uncaught_in("main");
        std::process::exit(1)
    }

    /// 线程 `thread` 的未捕获异常报告（不退出进程：JVM 中只终结该线程）。
    pub fn report_uncaught_in(&self, thread: &str) {
        eprintln!("Exception in thread \"{}\" {}", thread, self.describe());
        // JVM printStackTrace 的 `Caused by:` 链（无栈帧行）：cause 经虚调用 `getCause()` 取得——子类覆盖
        // （如 javax.xml.transform.TransformerException 的 getCause 返回 containedException）与 JDK 同样生效；
        // Throwable.getCause 自身把 cause == this（未设置哨兵）答为 null。getCause 抛异常时停止
        if self.is_instance_of("java/lang/Throwable") {
            let mut cur: Throwable = self.catch_as::<Throwable>();
            for _ in 0..16 {
                let Ok(next) = cur.getCause() else { break };
                let next_obj = Object::from(Clone::clone(&next));
                if next_obj.0.is_jvm_null() || next_obj == Object::from(Clone::clone(&cur)) {
                    break;
                }
                eprintln!("Caused by: {}", JvmError::from(Clone::clone(&next)).describe());
                cur = next;
            }
        }
        if std::env::var("RUST_BACKTRACE").map(|v| v != "0").unwrap_or(false) {
            eprintln!("{}", std::backtrace::Backtrace::force_capture());
        }
    }
}

impl std::fmt::Debug for JvmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.describe())
    }
}

impl std::fmt::Display for JvmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.describe())
    }
}
