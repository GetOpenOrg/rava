use crate::prelude::*;
use super::object::Object;
use super::object_body::{Object__finalize_body, Object__wait_body, Object__wait_l_body, Object__wait_l_i_body};

/// `new Object()` 的实例体：无 Java 字段；占 1 字节使每个实例拥有独立堆地址（对象身份）。
pub struct Instance(#[allow(dead_code)] u8);

impl Instance {
    /// 引导映像中的 `new Object()` 实例（常量求值，映像模块的对象值）
    pub const IMAGE: Instance = Instance(0);
}

impl super::object::ObjectVTable for Instance {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn is_instance_of(&self, type_id: &str) -> bool { type_id == "java/lang/Object" }
    fn hashCode(&self) -> i32 { super::object::__identity_hash(self as *const Instance as *const ()) }
    // SAFETY: 实例只经 `Object::__alloc` 分配或作为映像对象（带头部）存在
    fn __object(&self) -> Option<Object> { Some(unsafe { Object::__from_storage(self) }) }
    /// Display / Debug：Object.toString 的翻译体文本
    fn __obj_str(&self) -> std::string::String {
        self.__to_string().unwrap_or_else(|_| "java.lang.Object".to_owned())
    }
}

impl Object {
    /// java.lang.Object.<init>()V（手写根类无 `__class_init`：Object 无 `<clinit>`）
    #[jvm_native(no_class_init)]
    pub fn new() -> Result<Object> { Ok(Object::__alloc(Instance(0))) }

    #[jvm_native]
    pub fn lock(&self) -> Result<()> { Ok(()) }

    #[jvm_native]
    pub fn unlock(&self) -> Result<()> { Ok(()) }

    /// JVM 语义：对 null 引用调 getClass 抛 NPE（invokevirtual 的隐式 null 检查）。
    #[jvm_native]
    pub fn getClass(&self) -> Result<crate::java::lang::Class> {
        if self.0.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        self.0.getClass()
    }

    /// JVM 语义：null 接收者抛 NPE（invokevirtual 的隐式 null 检查；类型化 null 不进入覆盖体）。
    #[jvm_native]
    pub fn hashCode(&self) -> Result<i32> {
        if self.0.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        Ok(self.0.hashCode())
    }

    /// `finalize()`（protected）：静态祖先链未覆盖 finalize 的类上 `this.finalize()` 经根路由落此，
    /// 执行 Object.finalize 的翻译体。GC 触发的终结调用不建模（无 GC）。
    pub fn finalize(&self) -> Result<()> {
        if self.0.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        Object__finalize_body(self)
    }

    // ── 字节码方法的 bare-Object 接收者入口 ─────────────────────────────────
    //
    // equals / toString / wait 三个重载是 Object 的字节码方法，语义在生成的根类方法体
    // （`object_body`，按 JDK 字节码翻译）。这里只是接收者静态类型为 Object 时的调用入口：
    // invokevirtual 的隐式 null 检查 + 分派——可覆盖的 equals / toString 经 vtable 落到运行时类
    // 的覆盖体或根类翻译体；final 的 wait 直接调用翻译体。命名与生成侧同源（描述符后缀）。

    pub fn equals(&self, other: Object) -> Result<bool> {
        if self.0.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        self.0.equals(other)
    }

    pub fn toString(&self) -> Result<String> {
        if self.0.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        Ok(String::from(self.0.__to_string()?.as_str()))
    }

    /// java.lang.Object.wait()V
    pub fn wait(&self) -> Result<()> {
        if self.0.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        Object__wait_body(self)
    }

    /// java.lang.Object.wait(J)V
    pub fn wait_l(&self, millis: i64) -> Result<()> {
        if self.0.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        Object__wait_l_body(self, millis)
    }

    /// java.lang.Object.wait(JI)V
    pub fn wait_l_i(&self, millis: i64, nanos: i32) -> Result<()> {
        if self.0.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        Object__wait_l_i_body(self, millis, nanos)
    }

    /// java.lang.Object.wait0(J)V（private native）：在本对象监视器上等待，millis 为 0 表示无限等待；
    /// 中断以 InterruptedException 返回（监视器侧表，monitor.rs）
    #[jvm_native]
    pub fn wait0(&self, millis: i64) -> Result<()> {
        crate::monitor::wait_timeout(self.0.__identity() as usize, false, millis, 0)
    }

    /// java.lang.Object.notify()V：无等待者时静默
    #[jvm_native]
    pub fn notify(&self) -> Result<()> {
        if self.0.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        crate::monitor::notify(self.0.__identity() as usize, false)
    }

    /// java.lang.Object.notifyAll()V：notifyAll 无重载，mangle_name 保持
    /// 原名——翻译侧 invokevirtual 呼叫这个名字（notify_all 是早期蛇形名，
    /// 手写内部消费方继续可用，双名同体）。
    #[jvm_native]
    pub fn notify_all(&self) -> Result<()> {
        if self.0.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        crate::monitor::notify_all(self.0.__identity() as usize, false)
    }

    /// notifyAll 的生成侧名（mangle 语义别名，见上）
    #[jvm_native]
    pub fn notifyAll(&self) -> Result<()> {
        self.notify_all()
    }

    /// monitorenter（指令侧，codegen 发射）：可重入获取监视器
    #[jvm_ext]
    pub fn monitor_enter(&self) -> Result<()> {
        crate::monitor::enter(self.0.__identity() as usize, self.0.is_jvm_null())
    }

    /// monitorexit（指令侧，codegen 发射）：释放一层重入计数
    #[jvm_ext]
    pub fn monitor_exit(&self) -> Result<()> {
        crate::monitor::exit(self.0.__identity() as usize)
    }

    #[jvm_native]
    pub fn getComponentType(&self) -> Result<Object> {
        panic!("stub: Class.getComponentType()")
    }

    #[jvm_native]
    pub fn getName(&self) -> Result<Object> {
        panic!("stub: Class.getName()")
    }

    #[jvm_native]
    pub fn isArray(&self) -> Result<bool> {
        panic!("stub: Class.isArray()")
    }
}
