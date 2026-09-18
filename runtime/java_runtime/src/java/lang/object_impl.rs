use crate::prelude::*;
use super::object::Object;
use super::string::String as JvmString;

impl Object {
    #[jvm_native]
    pub fn lock(&self) -> Result<()> { Ok(()) }

    #[jvm_native]
    pub fn unlock(&self) -> Result<()> { Ok(()) }

    #[jvm_native]
    pub fn getClass(&self) -> Result<crate::java::lang::Class> { self.0.getClass() }

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
        Ok(false)
    }

    #[jvm_native]
    pub fn toString(&self) -> Result<String> { Ok(String::from(self.0.__obj_str())) }

    #[jvm_native]
    pub fn flushBuffer(&self) -> Result<()> { Ok(()) }

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
