"""runtime/java_runtime/ 下的手写层清单文件读取（P-1：库知识表的唯一落点）。

CLAUDE.md 原则 4：生成器 Python 代码中不出现 JDK 类名常量。需要按类名枚举的
**库知识**（载体铺设名单、边界包前缀、重载后缀缩写、签名擦除接口等）以清单文件
维护在 runtime/（手写层真源，与 closure.toml 同一先例），生成器只负责读取。
JLS / JVMS 规定的语言层类（Object / String / Class / 包装类 / Record / 注解根接口
等）不属库知识，集中在 constants.py。

两种载体：
- 结构化清单（TOML，按职责合并）：closure.toml（闭包边界）、seeds.toml（入链种子 / 引导初始化）、
  vm_intrinsics.toml（VM 承载与调用点特判）——经 `closure()` / `seeds()` / `vm_intrinsics()` 等读取；
- 行式清单（txt）：每行一项，`#` 起始为注释行，空行忽略；映射清单每行 `键 值`（空白分隔）。
"""

from __future__ import annotations

import functools
import os
import tomllib

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


# ── 结构化清单（TOML）──────────────────────────────────────────────────────────────

@functools.lru_cache(maxsize=None)
def _toml(name: str) -> dict:
    path = os.path.join(RUNTIME_JAVA_RUNTIME, name)
    if not os.path.exists(path):
        return {}
    with open(path, 'rb') as f:
        return tomllib.load(f)


def _packages(sec: dict, key: str, where: str) -> list[str]:
    vals = list(sec.get(key, []))
    for v in vals:
        if not v.endswith('/'):
            raise ValueError(f'{where}.{key}：包条目须以 / 结尾：{v}')
    return vals


def _classes(sec: dict, key: str, where: str) -> list[str]:
    vals = list(sec.get(key, []))
    for v in vals:
        if v.endswith('/'):
            raise ValueError(f'{where}.{key}：类条目不得以 / 结尾（包请写入 packages）：{v}')
    return vals


def _member_ref(ref: str) -> tuple[str, str]:
    """`类.成员` → (类, 成员)。"""
    cls, _, member = ref.rpartition('.')
    return cls, member


def _method_ref(ref: str) -> tuple[str, str, str]:
    """`类.方法:描述符` → (类, 方法, 描述符)。"""
    owner, _, desc = ref.partition(':')
    cls, member = _member_ref(owner)
    return cls, member, desc


def boundary_packages() -> list[str]:
    """内部边界包前缀（closure.toml [boundary]）。"""
    return _packages(_toml('closure.toml').get('boundary', {}), 'packages', 'boundary')


def vm_boundary_classes() -> list[str]:
    """VM 耦合边界类（closure.toml [vm_boundary]）。"""
    return _classes(_toml('closure.toml').get('vm_boundary', {}), 'classes', 'vm_boundary')


def release_entries() -> list[str]:
    """边界放行条目（closure.toml [release]）：包前缀（`/` 结尾）在前、类（含 `$` 嵌套类）在后。"""
    sec = _toml('closure.toml').get('release', {})
    return _packages(sec, 'packages', 'release') + _classes(sec, 'classes', 'release')


def seed_section(name: str) -> dict:
    """seeds.toml 的一节（annotation / locale / jca / data_bundle / boot_init）。"""
    return _toml('seeds.toml').get(name, {})


def annotation_triggers() -> frozenset:
    return frozenset(_member_ref(r) for r in seed_section('annotation').get('triggers', []))


def annotation_seed_members() -> tuple:
    return tuple(_method_ref(r) for r in seed_section('annotation').get('seeds', []))


def jca_release_entries() -> list[str]:
    sec = seed_section('jca')
    return (_packages(sec, 'release_packages', 'jca')
            + _classes(sec, 'release_classes', 'jca'))


def module_resource_paths() -> list[str]:
    """模块资源路径（seeds.toml [module_resources]，jmod `classes/` 下的相对路径）。"""
    return list(seed_section('module_resources').get('paths', []))


def boot_init_classes() -> list[str]:
    return _classes(seed_section('boot_init'), 'classes', 'boot_init')


def data_bundle_carriers() -> list[tuple[str, str, str]]:
    """纯数据资源束载体 (类, 方法, 描述符)。"""
    return [_method_ref(r) for r in seed_section('data_bundle').get('carriers', [])]


def intrinsic_members() -> frozenset:
    """VM 内建成员（vm_intrinsics.toml [[intrinsic]] member）；缺 kind / reason 即清单错误。"""
    out = set()
    for ent in _toml('vm_intrinsics.toml').get('intrinsic', []):
        if not ent.get('kind') or not ent.get('reason'):
            raise ValueError(f'vm_intrinsics.toml：内建条目须写明 kind 与 reason：{ent.get("member")}')
        out.add(ent['member'])
    return frozenset(out)


def caller_sensitive_annotations() -> frozenset:
    return frozenset(_toml('vm_intrinsics.toml').get('caller_sensitive', {}).get('annotations', []))


def sigpoly_callsite_typed() -> frozenset:
    return frozenset(_toml('vm_intrinsics.toml').get('sigpoly', {}).get('callsite_typed', []))
