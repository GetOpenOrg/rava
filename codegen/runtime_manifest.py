"""runtime/java_runtime/ 下的手写层清单文件读取（P-1：库知识表的唯一落点）。

CLAUDE.md 原则 4：生成器 Python 代码中不出现 JDK 类名常量。需要按类名枚举的
**库知识**（载体铺设名单、边界包前缀、重载后缀缩写、签名擦除接口等）以清单文件
维护在 runtime/（手写层真源，与 vm_boundary.txt 同一先例），生成器只负责读取。
JLS / JVMS 规定的语言层类（Object / String / Class / 包装类 / Record / 注解根接口
等）不属库知识，集中在 constants.py。

格式：每行一项，`#` 起始为注释行，空行忽略；映射清单每行 `键 值`（空白分隔）。
"""

from __future__ import annotations

import os

from .constants import RUNTIME_JAVA_RUNTIME


def read_list(name: str) -> list[str]:
    """清单文件的全部条目（保持文件顺序）；文件缺失 → 空表。"""
    path = os.path.join(RUNTIME_JAVA_RUNTIME, name)
    if not os.path.exists(path):
        return []
    with open(path, encoding='utf-8') as f:
        return [ln.strip() for ln in f if ln.strip() and not ln.lstrip().startswith('#')]


def read_map(name: str) -> dict[str, str]:
    """映射清单（每行 `键 值`）。"""
    out: dict[str, str] = {}
    for ln in read_list(name):
        k, v = ln.split(None, 1)
        out[k] = v.strip()
    return out
