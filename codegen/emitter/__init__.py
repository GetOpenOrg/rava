"""
codegen.emitter 子包：Java 类 → Rust 代码生成。

子模块：
  attrs         — to_snake、pkg_from_java、属性字符串生成
  method_gen    — _parse_synthetic_fn、_scan_impl_files、_gen_native_stub
  struct_gen    — struct 定义相关（扩展占位）
  field_gen     — 静态字段 getter 相关（扩展占位）
  class_writer  — _gen_class_rs 主函数
  project_writer — write_cargo_project 及辅助函数
"""

from .attrs import to_snake, pkg_from_java
from .project_writer import write_cargo_project

__all__ = ['write_cargo_project', 'to_snake', 'pkg_from_java']
