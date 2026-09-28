use crate::prelude::*;
use super::GetBooleanAction;
use crate::java::lang::Boolean;

// 内部边界类 sun.security.action.GetBooleanAction：读一个 boolean 系统属性的
// PrivilegedAction。struct 由字节码生成；按调用链按需实现，其余方法保持 panic 存根。
// 取值与 JDK 同源：`Boolean.getBoolean(theProp)`（翻译字节码 → System.getProperty，
// 系统属性全集见 FS-P1）；安全管理器恒 null，doPrivileged 包装透明。

impl GetBooleanAction {
    /// `<init>(String theProp)`：记录待查询的属性名。
    #[jvm_boundary]
    pub fn new(theProp: String) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        this.__set_theProp(theProp);
        Ok(this)
    }

    /// `run()`（虚方法，经 wrapper 钩子 `__impl_run` 执行）：`Boolean.getBoolean(theProp)` 装箱。
    #[jvm_boundary(upcalls = "java/lang/Boolean.getBoolean:(Ljava/lang/String;)Z java/lang/Boolean.valueOf:(Z)Ljava/lang/Boolean;")]
    pub fn __impl_run(&self) -> Result<Boolean> {
        Boolean::valueOf_z(Boolean::getBoolean(self.__get_theProp())?)
    }

    /// `privilegedGetProperty(String)`：安全管理器为 null 时 JDK 直接 `Boolean.getBoolean(theProp)`。
    #[jvm_boundary(upcalls = "java/lang/Boolean.getBoolean:(Ljava/lang/String;)Z")]
    pub fn privilegedGetProperty(theProp: String) -> Result<bool> {
        Boolean::getBoolean(theProp)
    }
}
