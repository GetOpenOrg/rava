use crate::prelude::*;
use super::object::Object;
use super::string::String as JvmString;

/// `new Object()` 的实例体：无 Java 字段；占 1 字节使每个实例拥有独立堆地址（对象身份）。
struct Instance(#[allow(dead_code)] u8);

impl super::object::ObjectVTable for Instance {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn is_instance_of(&self, type_id: &str) -> bool { type_id == "java/lang/Object" }
    fn hashCode(&self) -> i32 { self as *const Instance as usize as i32 }
    fn __obj_str(&self) -> std::string::String {
        format!("java.lang.Object@{:x}", self as *const Instance as usize as i32)
    }
}

impl Object {
    /// java.lang.Object.<init>()V
    #[jvm_native]
    pub fn new() -> Result<Object> { Ok(Object(std::rc::Rc::new(Instance(0)))) }

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

    #[jvm_native]
    pub fn hashCode(&self) -> Result<i32> { Ok(self.0.hashCode()) }

    /// `finalize()`（protected，方法体为空）：静态祖先链未覆盖 finalize 的类上
    /// `this.finalize()` 经根路由落此（invoke_virtual 的 protected void 根方法分支）。
    /// GC 触发的终结调用不建模（无 GC）。
    pub fn finalize(&self) -> Result<()> { Ok(()) }

    #[jvm_native]
    pub fn equals(&self, other: Object) -> Result<bool> {
        if *self == other {
            return Ok(true);
        }
        // String 内容比较：通过 Display impl（string_ext.rs 中使用字节数组解码）
        let s1 = self.0.as_any().downcast_ref::<JvmString>();
        let s2 = other.0.as_any().downcast_ref::<JvmString>();
        if let (Some(a), Some(b)) = (s1, s2) {
            return Ok(format!("{}", a) == format!("{}", b));
        }
        self.0.equals(other)
    }

    #[jvm_native]
    pub fn toString(&self) -> Result<String> { Ok(String::from(self.0.__obj_str())) }

    // ── Object 监视器方法（S-20）：bare-Object 接收者的调用落点 ─────────────
    //
    // invokevirtual java/lang/Object.{wait,notify,notifyAll} 在接收者静态类型为
    // Object 时直调本层固有方法（与 getClass 同一形态）；具体类型接收者经
    // ObjectVTable 的默认方法（object.rs）分派。命名与生成侧同源（描述符后缀）。

    /// java.lang.Object.wait()V（等价 wait(0)）
    #[jvm_native]
    pub fn wait(&self) -> Result<()> {
        if self.0.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        crate::monitor::wait_timeout(self.0.__identity() as usize, false, 0, 0)
    }

    /// java.lang.Object.wait(J)V
    #[jvm_native]
    pub fn wait_l(&self, millis: i64) -> Result<()> {
        if self.0.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        crate::monitor::wait_timeout(self.0.__identity() as usize, false, millis, 0)
    }

    /// java.lang.Object.wait(JI)V
    #[jvm_native]
    pub fn wait_l_i(&self, millis: i64, nanos: i32) -> Result<()> {
        if self.0.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        crate::monitor::wait_timeout(self.0.__identity() as usize, false, millis, nanos)
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
