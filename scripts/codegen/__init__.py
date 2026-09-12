"""
codegen — Java .class → Rust 转译器包

公开接口：
    transpile(java_files, out_dir) — 完整转译流水线
"""

from .transpile import transpile

__all__ = ['transpile']
