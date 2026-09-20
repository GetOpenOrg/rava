"""
codegen.emitter 子包：Java 类 → Rust 代码生成。

子模块：
  attrs         — to_snake、pkg_from_java、属性字符串生成
  method_gen    — _parse_synthetic_fn、_scan_impl_files、_gen_native_stub
  struct_gen    — struct 定义相关（扩展占位）
  field_gen     — 字段类型解析器（_resolve_field_rust 等，class_writer 嵌套函数提升）
  import_gen    — 引用收集（collect_referenced）+ 精确 cross_imports 生成
  clinit_extract — static 字段声明 + <clinit> 方法块生成
  class_writer  — _gen_class_rs 主流程（方法发射段为模块级函数）
  project_writer — write_cargo_project 及辅助函数
"""

from .attrs import to_snake, pkg_from_java
from .project_writer import write_cargo_project

__all__ = ['write_cargo_project', 'to_snake', 'pkg_from_java']
