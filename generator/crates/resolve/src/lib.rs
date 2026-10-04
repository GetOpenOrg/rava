//! 类路径与类层次：按需加载、缓存，JVMS 字段 / 方法解析与虚方法选择。

pub mod classpath;
pub mod hierarchy;
pub mod image;
pub mod jdk;
pub mod modules;

pub use classpath::{ArchiveView, ClassPath, Origin};
pub use modules::{ModuleFacts, ModuleGraph, ModuleNode};
pub use hierarchy::{is_signature_polymorphic, package_of, FieldSite, Hierarchy, MethodSite};
