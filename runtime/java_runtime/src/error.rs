//! Java 异常的 Rust 载体。
//!
//! 所有翻译方法返回 `Result<T>`；`Err` 侧只有一种形态：**被抛出的 Throwable 对象本身**。
//! `athrow` 把栈顶对象装入 `JvmError`，异常表匹配按对象的运行时类（含子类关系）进行，
//! VM 自身抛出的异常（空引用、数组越界、类初始化失败等）同样构造翻译后的 Java 异常对象，
//! 因此 catch、`getMessage()`、未捕获报告对用户异常与 VM 异常完全一致。
//!
//! 本文件调用的已翻译构造器登记在 `runtime/java_runtime/vm_roots.txt`（VM 根方法清单）。

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

    /// catch 绑定：按运行时类把异常对象还原为 catch 声明类型 `T`（其 binary name 为 `binary_name`）。
    /// 仅在 `is_instance_of(binary_name)` 成立后调用。
    pub fn catch_as<T: std::any::Any + Clone>(&self, binary_name: &str) -> T {
        if let Some(same) = (&self.thrown as &dyn std::any::Any).downcast_ref::<T>() {
            return Clone::clone(same);
        }
        if let Some(same) = self.thrown.0.as_any().downcast_ref::<T>() {
            return Clone::clone(same);
        }
        let unused: std::rc::Rc<dyn std::any::Any> = std::rc::Rc::new(());
        let view = self.thrown.0.__view_as(unused, binary_name)
            .and_then(|boxed| boxed.downcast::<T>().ok());
        match view {
            Some(v) => *v,
            None => panic!(
                "catch 类型还原失败：{} 无法视为 {}", self.class_name(), binary_name),
        }
    }

    /// catch-any 绑定（异常表 catch_type = 0）：athrow 操作数的静态类型即 Throwable。
    pub fn catch_any(&self) -> crate::java::lang::Throwable {
        self.catch_as::<crate::java::lang::Throwable>(crate::java::lang::Throwable::BINARY_NAME)
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

    /// Object.clone()：运行时类未实现 Cloneable。消息为类的全限定名（与 HotSpot 一致）。
    pub fn clone_not_supported(binary_name: &str) -> Self {
        vm_throw(crate::java::lang::CloneNotSupportedException::new_str(String::from(
            binary_name.replace('/', "."))))
    }

    pub fn illegal_monitor_state(message: &str) -> Self {
        vm_throw(crate::java::lang::IllegalMonitorStateException::new_str(String::from(message)))
    }

    pub fn out_of_memory(message: &str) -> Self {
        vm_throw(crate::java::lang::OutOfMemoryError::new_str(String::from(message)))
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
        let throwable: Throwable = cause.catch_as::<Throwable>("java/lang/Throwable");
        vm_throw(crate::java::lang::ExceptionInInitializerError::new_throwable(throwable))
    }

    // ── 未捕获异常报告 ───────────────────────────────────────────────────────

    /// Java 默认未捕获异常处理器的首行：`java.lang.Xxx: message`（与 Throwable.toString 同构）。
    pub fn describe(&self) -> std::string::String {
        let class_name = self.class_name().replace('/', ".");
        let message = if self.is_instance_of("java/lang/Throwable") {
            let throwable: Throwable = self.catch_as::<Throwable>("java/lang/Throwable");
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
    /// `JAVA_RTA_UNCAUGHT_BT=1` 时附打印时 Rust backtrace（定位抛出点的诊断开关）。
    pub fn report_uncaught(&self) -> ! {
        eprintln!("Exception in thread \"main\" {}", self.describe());
        if std::env::var_os("JAVA_RTA_UNCAUGHT_BT").is_some() {
            eprintln!("{}", std::backtrace::Backtrace::force_capture());
        }
        std::process::exit(1)
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
