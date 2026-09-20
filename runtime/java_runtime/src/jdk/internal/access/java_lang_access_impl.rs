//! `jdk/internal/access/JavaLangAccess` 的实现对象（仅当 `java_lang_access.rs`
//! 进入闭包生成时编译，见 K-2 规则）。
//!
//! JDK 中该接口由 `java/lang/System$JavaLangAccess`（System 的内部类）实现，
//! `System.<clinit>` 经 `setJavaLangAccess()` 登记到 SharedSecrets。该内部类实现
//! `jdk/internal/` 内部接口，属内部边界族 → 本文件整体手写（规则 3b）：只实现
//! 调用链触达的 `getEnumConstantsShared`，其余方法走接口 vtable trait 的默认
//! `panic!("stub: ...")` 存根（生成侧 java_lang_access.rs 自带）。

use crate::prelude::*;
use super::java_lang_access::JavaLangAccess__VTable;
use crate::java::lang::Class;
use crate::java::lang::Enum;

/// `java/lang/System$JavaLangAccess` 的手写实现对象。
///
/// 无实例状态（JDK 原型也无状态，全部方法转发静态语义）；唯一职责是经
/// `ObjectVTable::__interface` 把自身填入 `Option<Rc<dyn JavaLangAccess__VTable>>`
/// 槽位——等价 JVM itable 条目，接口载体（`Into::<JavaLangAccess>::into(obj)`）
/// 的分派由此命中。
pub(super) struct SystemJavaLangAccess;

impl JavaLangAccess__VTable for SystemJavaLangAccess {
    /// `getEnumConstantsShared(Class<E>)E[]`：枚举宇宙从运行时常量目录重建
    /// （`java_class!` 宏在类初始化后按「自身类型 static 字段」形态登记，枚举
    /// 常量即该形态，登记序 == 声明序 == ordinal 序）。入参 Class 的名字为点分
    /// binary name，与目录键一致。空 / 未命中返回 null（JDK 对非枚举类同此）。
    fn getEnumConstantsShared(&self, arg0: Class) -> Result<JArray<Enum<Object>>> {
        let cls_name = format!("{}", arg0.__get_name());
        match constant_directory_universe(&cls_name) {
            None => Ok(JArray::default()),
            Some(elems) => Ok(JArray::from(
                elems.into_iter().map(Enum::<Object>::from).collect::<Vec<Enum<Object>>>()
            )),
        }
    }
}

impl ObjectVTable for SystemJavaLangAccess {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn __class_name(&self) -> &'static str { "java/lang/System$JavaLangAccess" }
    fn __obj_str(&self) -> std::string::String {
        "java.lang.System$JavaLangAccess".to_owned()
    }
    /// 接口视图查询（invokeinterface 的运行时入口）：调用方 slot 是
    /// `Option<Rc<dyn JavaLangAccess__VTable>>` 时填入自身（与 java_class! 宏为
    /// `impl Iface for Class` 生成的形态一致）。
    fn __interface(self: Rc<Self>, slot: &mut dyn std::any::Any) {
        if let Some(s) = slot.downcast_mut::<Option<Rc<dyn JavaLangAccess__VTable>>>() {
            *s = Some(self);
        }
    }
}
