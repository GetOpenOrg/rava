use crate::prelude::*;
use super::GetBooleanAction;
use crate::java::lang::Boolean;

// 内部边界类 sun.security.action.GetBooleanAction：读一个 boolean 系统属性的
// PrivilegedAction。struct 由字节码生成；按 Arrays$LegacyMergeSort 调用链按需实现
// 构造器与 run()，其余方法（privilegedGetProperty 等）保持 panic 存根。

// 系统属性表：原生二进制未提供 -D 注入机制，默认为空 —— 任何属性都不存在，
// getBoolean 恒为 false（这正是 java.util.Arrays.useLegacyMergeSort 关闭、
// 排序走 TimSort 的默认语义）。后续 System.getProperty 手写时在此对接或迁移。
thread_local! {
    static SYSTEM_PROPERTIES: crate::sync_model::__RefSlot<
        std::collections::HashMap<std::string::String, std::string::String>
    > = crate::sync_model::__RefSlot::new(std::collections::HashMap::new());
}

/// Boolean.getBoolean(name) 的取值：属性存在且值 equalsIgnoreCase("true") 才为 true。
fn get_boolean_property(name: &str) -> bool {
    SYSTEM_PROPERTIES.with(|props| {
        props.borrow()
            .get(name)
            .map_or(false, |v| v.eq_ignore_ascii_case("true"))
    })
}

impl GetBooleanAction {
    /// `<init>(String theProp)`：记录待查询的属性名。
    #[jvm_boundary]
    pub fn new(theProp: String) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        this.__set_theProp(theProp);
        Ok(this)
    }

    /// `run()`（虚方法，经 wrapper 钩子 `__impl_run` 执行）：读属性表返回 Boolean。
    /// S-3.1 后 Boolean 是字节码翻译类（不再是原生 bool），经 valueOf 构造
    ///（upcalls 声明使 valueOf 进入调用链，得到字节码翻译体而非存根）。
    #[jvm_boundary(upcalls = "java/lang/Boolean.valueOf:(Z)Ljava/lang/Boolean;")]
    pub fn __impl_run(&self) -> Result<Boolean> {
        Boolean::valueOf_z(get_boolean_property(&format!("{}", self.__get_theProp())))
    }
}
