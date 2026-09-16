#![allow(ambiguous_glob_reexports)]
pub mod abstract_collection;
pub use abstract_collection::*;
pub mod abstract_list;
pub use abstract_list::*;
pub mod array_list;
pub use array_list::*;
pub mod arrays;
pub use arrays::*;
pub mod function;
pub use function::*;
pub mod iterator;
pub use iterator::*;
pub mod list;
pub use list::*;
mod array_list_impl;
// list_impl 已删除 — Arch-1 后接口方法通过 downcast dispatch 调用
