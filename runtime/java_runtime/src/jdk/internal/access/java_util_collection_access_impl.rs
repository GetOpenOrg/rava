//! `jdk/internal/access/JavaUtilCollectionAccess` 的实现对象（仅当
//! `java_util_collection_access.rs` 进入闭包生成时编译，见 K-2 规则）。
//!
//! JDK 中该接口由 `java/util/ImmutableCollections` 的 `<clinit>` 以匿名类实例
//! 登记到 `SharedSecrets.setJavaUtilCollectionAccess`，两个方法各自转发
//! `ImmutableCollections` 的同名静态。匿名类实现 `jdk/internal/` 内部接口，
//! 属内部边界族 → 整体手写（规则 3b）：转发目标静态由
//! `shared_secrets_impl.rs::getJavaUtilCollectionAccess` 的 upcalls 声明拉入
//! 闭包（BFS 触达即翻译，签名引用保证任意闭包形态可编译）。

use crate::prelude::*;
use super::java_util_collection_access::JavaUtilCollectionAccess__VTable;
use crate::java::util::ImmutableCollections;
use std::rc::Rc;

/// `java/util/ImmutableCollections$1` 的手写实现对象（JDK 的匿名访问器）。
///
/// 无实例状态；唯一职责是把接口两方法转发到 `ImmutableCollections` 的同名
/// 静态（`Stream.toList` 的 trusted-array 不可变列表构造路径）。
pub(super) struct ImmutableCollectionsCollAccess;

impl JavaUtilCollectionAccess__VTable for ImmutableCollectionsCollAccess {
    fn listFromTrustedArray(&self, arg0: JArray<Object>) -> Result<Object> {
        ImmutableCollections::listFromTrustedArray(arg0)
    }

    fn listFromTrustedArrayNullsAllowed(&self, arg0: JArray<Object>) -> Result<Object> {
        ImmutableCollections::listFromTrustedArrayNullsAllowed(arg0)
    }
}

impl ObjectVTable for ImmutableCollectionsCollAccess {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn __class_name(&self) -> &'static str { "java/util/ImmutableCollections$1" }
    fn __obj_str(&self) -> std::string::String {
        "java.util.ImmutableCollections$1".to_owned()
    }
    /// 接口视图查询（invokeinterface 的运行时入口）：调用方 slot 是
    /// `Option<Rc<dyn JavaUtilCollectionAccess__VTable>>` 时填入自身
    /// （与 java_class! 宏为 `impl Iface for Class` 生成的形态一致）。
    fn __interface(self: Rc<Self>, slot: &mut dyn std::any::Any) {
        if let Some(s) = slot.downcast_mut::<Option<Rc<dyn JavaUtilCollectionAccess__VTable>>>() {
            *s = Some(self);
        }
    }
}
