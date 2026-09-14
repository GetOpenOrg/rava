"""
Java 元数据注释生成：to_snake、pkg_from_java、访问标志字符串、
#[java_class] / #[java_field] / #[java_method] 属性块。
"""

import re
from ..types import ClassInfo, FieldInfo, ParsedMethod
from ..constants import RUST_KEYWORDS as _RUST_KEYWORDS

# Access flags
_ACC_PUBLIC       = 0x0001
_ACC_PRIVATE      = 0x0002
_ACC_PROTECTED    = 0x0004
_ACC_STATIC       = 0x0008
_ACC_FINAL        = 0x0010
_ACC_SYNCHRONIZED = 0x0020
_ACC_VOLATILE     = 0x0040  # 字段：volatile；方法：bridge
_ACC_TRANSIENT    = 0x0080  # 字段：transient；方法：varargs
_ACC_NATIVE       = 0x0100
_ACC_INTERFACE    = 0x0200
_ACC_ABSTRACT     = 0x0400
_ACC_SYNTHETIC    = 0x1000
_ACC_ANNOTATION   = 0x2000
_ACC_ENUM         = 0x4000


def to_snake(name: str) -> str:
    """PascalCase / camelCase → snake_case（Rust 模块文件名）。
    $ (内部类分隔符) → _ ，Rust 关键字加 _ 后缀。"""
    name = name.replace('$', '_')
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', name)
    s = re.sub(r'([a-z\d])([A-Z])', r'\1_\2', s)
    s = s.lower()
    if s in _RUST_KEYWORDS:
        s = s + '_'
    return s


def pkg_from_java(java_file: str) -> str:
    """从 .java 源文件读取 package 声明，无则返回空串"""
    try:
        with open(java_file, encoding='utf-8') as f:
            for line in f:
                line = line.strip()
                if line.startswith('package '):
                    return line[8:].rstrip(';').strip()
                if line and not line.startswith('//') and not line.startswith('/*'):
                    if any(line.startswith(k) for k in ('import ', 'public ', 'class ', '@')):
                        break
    except OSError:
        pass
    return ''


def _access_str(flags: int) -> str:
    """将 access_flags 整数转为访问权限字符串（public/protected/private/package）。"""
    if flags & _ACC_PUBLIC:    return 'public'
    if flags & _ACC_PRIVATE:   return 'private'
    if flags & _ACC_PROTECTED: return 'protected'
    return 'package'


def _class_modifiers_str(flags: int) -> str:
    """从类 access_flags 提取修饰符（排除访问权限）。"""
    parts = []
    if flags & _ACC_FINAL:      parts.append('final')
    if flags & _ACC_ABSTRACT:   parts.append('abstract')
    if flags & _ACC_INTERFACE:  parts.append('interface')
    if flags & _ACC_ENUM:       parts.append('enum')
    if flags & _ACC_ANNOTATION: parts.append('annotation')
    if flags & _ACC_SYNTHETIC:  parts.append('synthetic')
    return ' '.join(parts)


def _field_modifiers_str(flags: int) -> str:
    """从字段 access_flags 提取修饰符（static/final/volatile/transient/synthetic）。"""
    parts = []
    if flags & _ACC_STATIC:    parts.append('static')
    if flags & _ACC_FINAL:     parts.append('final')
    if flags & _ACC_VOLATILE:  parts.append('volatile')
    if flags & _ACC_TRANSIENT: parts.append('transient')
    if flags & _ACC_SYNTHETIC: parts.append('synthetic')
    return ' '.join(parts)


def _method_modifiers_str(flags: int) -> str:
    """从方法 access_flags 提取修饰符（static/final/synchronized/native/abstract/bridge/varargs）。"""
    parts = []
    if flags & _ACC_STATIC:       parts.append('static')
    if flags & _ACC_FINAL:        parts.append('final')
    if flags & _ACC_SYNCHRONIZED: parts.append('synchronized')
    if flags & _ACC_NATIVE:       parts.append('native')
    if flags & _ACC_ABSTRACT:     parts.append('abstract')
    if flags & _ACC_VOLATILE:     parts.append('bridge')    # ACC_BRIDGE 与 ACC_VOLATILE 同值
    if flags & _ACC_TRANSIENT:    parts.append('varargs')   # ACC_VARARGS 与 ACC_TRANSIENT 同值
    if flags & _ACC_SYNTHETIC:    parts.append('synthetic')
    return ' '.join(parts)


def _java_class_attr(ci: ClassInfo, compiled: bool = False) -> str:
    """生成 #[java_class(...)] 属性块。

    compiled=True：生成真实的 #[java_rta_macros::java_class(...)]，proc-macro 会自动派生
                   Into<Object>、From<Object>、Debug（Object 类除外）。
    compiled=False：原生属性格式，用于不参与编译的 JDK 元数据存根文件。
    """
    binary_name = ci.name
    super_class = ci.super_class or ""
    interfaces  = ','.join(ci.interfaces) if ci.interfaces else ""
    access      = _access_str(ci.access_flags) if ci.access_flags else ""
    modifiers   = _class_modifiers_str(ci.access_flags) if ci.access_flags else ""
    source      = ci.source_file or ""
    generic_sig = (ci.generic_signature or "").replace('"', '\\"')
    inner_lines = [
        f'    binary_name       = "{binary_name}",',
        f'    super_class       = "{super_class}",',
        f'    interfaces        = "{interfaces}",',
        f'    access            = "{access}",',
        f'    modifiers         = "{modifiers}",',
        f'    generic_signature = "{generic_sig}",',
        f'    is_interface      = {str(ci.is_interface).lower()},',
        f'    is_abstract       = {str(ci.is_abstract).lower()},',
        f'    is_enum           = {str(ci.is_enum).lower()},',
        f'    is_deprecated     = {str(ci.is_deprecated).lower()},',
        f'    source            = "{source}",',
    ]
    # inner_classes：以 "inner/Class:outer/Class:simple:flags" 形式编码，逗号分隔
    if ci.inner_classes:
        ic_strs = ';'.join(
            f'{ic.inner_class}:{ic.outer_class}:{ic.inner_name}:{ic.access_flags}'
            for ic in ci.inner_classes
        )
        inner_lines.append(f'    inner_classes     = "{ic_strs}",')
    inner = '\n'.join(inner_lines)
    if compiled:
        # Object 类自身不使用宏（from_any/downcast 定义在 Object 上，循环依赖）
        struct_name = ci.name.split('/')[-1]
        if struct_name == 'Object':
            return f'#[cfg_attr(any(), java_class(\n{inner}\n))]'
        return f'#[java_rta_macros::java_class(\n{inner}\n)]'
    else:
        return f'#[java_class(\n{inner}\n)]'


def _java_field_attr(f: FieldInfo) -> str:
    """生成 #[cfg_attr(any(), java_field(...))] 属性行（编译安全）。"""
    parts = [f'name = "{f.name}"', f'descriptor = "{f.descriptor}"']
    if f.access_flags:
        parts.append(f'access = "{_access_str(f.access_flags)}"')
        mods = _field_modifiers_str(f.access_flags)
        parts.append(f'modifiers = "{mods}"')
    parts.append(f'is_static = {str(f.is_static).lower()}')
    if f.generic_signature:
        sig = f.generic_signature.replace('"', '\\"')
        parts.append(f'generic_signature = "{sig}"')
    if f.constant_value:
        cv = f.constant_value.replace('"', '\\"')
        parts.append(f'constant_value = "{cv}"')
    if f.is_deprecated:
        parts.append('is_deprecated = true')
    return '#[cfg_attr(any(), java_field(' + ', '.join(parts) + '))]'


def _java_method_attr(m: ParsedMethod, compiled: bool = False) -> str:
    """生成方法元数据标注行。

    compiled=True（JDK 类生成）：
      - native 方法：#[cfg_attr(any(), java_native(...))]，供 build.rs 扫描
      - 其他方法：#[cfg_attr(any(), java_method(...))]，含完整元数据
    compiled=False（元数据存根）：原生属性格式 #[java_method(...)] / #[java_native(...)]。
    """
    tag = 'java_native' if m.is_native else 'java_method'
    desc = m.descriptor.replace('"', '\\"')
    name = m.name.replace('"', '\\"')
    parts = [f'name = "{name}"', f'descriptor = "{desc}"']
    if m.access_flags:
        parts.append(f'access = "{_access_str(m.access_flags)}"')
        mods = _method_modifiers_str(m.access_flags)
        parts.append(f'modifiers = "{mods}"')
    # 补全全部元数据字段
    parts.append(f'is_static    = {str(m.is_static).lower()}')
    parts.append(f'is_native    = {str(m.is_native).lower()}')
    parts.append(f'is_abstract  = {str(m.is_abstract).lower()}')
    parts.append(f'is_synthetic = {str(m.is_synthetic).lower()}')
    if m.exceptions:
        excs = ','.join(m.exceptions).replace('"', '\\"')
        parts.append(f'exceptions = "{excs}"')
    if m.generic_signature:
        sig = m.generic_signature.replace('"', '\\"')
        parts.append(f'generic_signature = "{sig}"')
    if m.is_deprecated:
        parts.append('is_deprecated = true')
    if m.method_parameters:
        mp_str = ';'.join(f'{n}:{a}' for n, a in m.method_parameters).replace('"', '\\"')
        parts.append(f'method_parameters = "{mp_str}"')
    if compiled:
        # 所有方法统一用 cfg_attr 包裹（编译安全，同时保留机器可读元数据）
        return f'#[cfg_attr(any(), {tag}(' + ', '.join(parts) + '))]'
    else:
        return f'#[{tag}(' + ', '.join(parts) + ')]'
