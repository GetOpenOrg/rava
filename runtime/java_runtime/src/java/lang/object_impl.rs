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
