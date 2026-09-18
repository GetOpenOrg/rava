use crate::prelude::*;
use super::object::Object;

impl Object {
    #[jvm_native]
    pub fn lock(&self) -> Result<()> { Ok(()) }

    #[jvm_native]
    pub fn unlock(&self) -> Result<()> { Ok(()) }

    #[jvm_native]
    /// 返回类型 `C` 由调用点按字节码描述符（java/lang/Class 的翻译类型）标注：
    /// 手写根类不能直接依赖生成的 Class 类型（并非每个 scratch 都生成它）。
    pub fn getClass<C: Clone + Default + 'static>(&self) -> Result<C> {
        let cls = self.0.getClass()?;
        Ok(cls.0.as_any().downcast_ref::<C>().cloned().unwrap_or_default())
    }

    #[jvm_native]
    pub fn hashCode(&self) -> Result<i32> { Ok(self.0.hashCode()) }

    #[jvm_native]
    pub fn equals(&self, _other: Object) -> Result<bool> { Ok(false) }

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
