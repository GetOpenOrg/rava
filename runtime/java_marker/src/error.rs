//! Java 异常的 Rust 载体——标记面（§9.9 K4 阶段 3：自 java_runtime error.rs 迁入）。
//!
//! 所有翻译方法返回 `Result<T>`；`Err` 侧只有一种形态：**被抛出的 Throwable 对象本身**
//! （静态视图为 `Object`）。结构与签名安全的访问器 / catch 绑定 / VM 构造器都在本层；
//! 构造器体是 `unsafe extern "Rust"` 钩子（`marker_hooks::__error_*` → java_runtime
//! `marker_hooks.rs` 构造翻译后的 Java 异常对象），签名引用生成类（`Throwable` /
//! `Thread`）的 `describe` / `report_uncaught` / `dispatch_uncaught` 留行为层
//! （java_runtime `error.rs` 的自由函数，名字与 vmrules 的 fns 名单一致）。
//!
//! VM 构造器即 VM 规则的落地（闭包分析器 `engine/vmrules.rs`：规则 ID + JVMS 依据，
//! 由可达字节码中的触发指令激活）；构造器对已翻译 Java 方法的调用在行为侧钩子定义里。

use crate::java::lang::Object;
use crate::marker_hooks as hooks;

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
            return hooks::__error_null_pointer();
        }
        JvmError { thrown }
    }
}

impl JvmError {
    /// 以已判空的异常对象直接构造（行为层 `vm_throw` 的包装点；athrow 判空走 [`From`]）。
    #[doc(hidden)]
    pub fn __wrap(thrown: Object) -> Self {
        JvmError { thrown }
    }

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

    // ── VM 抛出的异常（体 = 行为层钩子，见模块头注）──────────────────────────

    pub fn null_pointer() -> Self {
        hooks::__error_null_pointer()
    }

    pub fn array_index_out_of_bounds(index: i32, length: i32) -> Self {
        hooks::__error_array_index_out_of_bounds(index, length)
    }

    pub fn array_index_out_of_bounds_message(message: std::string::String) -> Self {
        hooks::__error_array_index_out_of_bounds_message(message)
    }

    pub fn index_out_of_bounds(message: std::string::String) -> Self {
        hooks::__error_index_out_of_bounds(message)
    }

    pub fn string_index_out_of_bounds(message: std::string::String) -> Self {
        hooks::__error_string_index_out_of_bounds(message)
    }

    pub fn negative_array_size(size: i32) -> Self {
        hooks::__error_negative_array_size(size)
    }

    pub fn arithmetic(message: &str) -> Self {
        hooks::__error_arithmetic(message)
    }

    pub fn class_cast(message: std::string::String) -> Self {
        hooks::__error_class_cast(message)
    }

    /// aastore 存储检查失败（JLS §10.5 / S-4 数组协变）：值与数组元素类型不赋值兼容。
    /// 消息为值的运行时类全限定名（与 HotSpot 一致）。
    pub fn array_store(value_class: &str) -> Self {
        hooks::__error_array_store(value_class)
    }

    /// Object.clone()：运行时类未实现 Cloneable。消息为类的全限定名（与 HotSpot 一致）。
    pub fn clone_not_supported(binary_name: &str) -> Self {
        hooks::__error_clone_not_supported(binary_name)
    }

    pub fn illegal_monitor_state(message: &str) -> Self {
        hooks::__error_illegal_monitor_state(message)
    }

    /// `Object.wait` 参数校验（HotSpot JVM_MonitorWait 同序：先于持有检查）。
    pub fn illegal_argument(message: &str) -> Self {
        hooks::__error_illegal_argument(message)
    }

    /// 阻塞原语被中断（`Thread.sleep` / `Object.wait`，HotSpot 同消息：sleep 带
    /// "sleep interrupted"，wait 无消息）。
    pub fn interrupted(message: Option<&str>) -> Self {
        hooks::__error_interrupted(message)
    }

    pub fn out_of_memory(message: &str) -> Self {
        hooks::__error_out_of_memory(message)
    }

    /// 栈界检查判定耗尽（`__stack_check`）：在放开的余量区内构造 `StackOverflowError`
    /// （HotSpot 黄区同义；余量区逻辑在行为层钩子内，rava_coro 依赖不出标记层）。
    pub fn stack_overflow() -> Self {
        hooks::__error_stack_overflow()
    }

    /// 类处于 erroneous 状态后的再次主动使用（JVMS §5.5 步骤 5）。
    pub fn no_class_def_found(binary_name: &str) -> Self {
        hooks::__error_no_class_def_found(binary_name)
    }

    /// 类解析失败（JVMS §5.4.3）：引用的类不在类路径上。消息为类的内部名（斜线形态，与 HotSpot 一致）
    pub fn class_not_found_at_resolution(internal_name: &str) -> Self {
        hooks::__error_class_not_found_at_resolution(internal_name)
    }

    /// `<clinit>` 异常收尾（JVMS §5.5 步骤 11）：Error 及其子类原样传播，
    /// 其余包装为 ExceptionInInitializerError（判定与包装在行为层钩子内）。
    pub fn in_initializer(cause: JvmError) -> Self {
        hooks::__error_in_initializer(cause)
    }
}

/// 未捕获异常报告的首行（`Throwable.toString` 同构）。行为面（读 `getMessage`）经钩子转交。
fn describe(e: &JvmError) -> std::string::String {
    hooks::__error_describe(e)
}

impl std::fmt::Debug for JvmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", describe(self))
    }
}

impl std::fmt::Display for JvmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", describe(self))
    }
}
