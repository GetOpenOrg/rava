"""
Java 元数据注释生成：to_snake、pkg_from_java、访问标志字符串、
#[java_class] / #[java_field] / #[java_method] 属性块。
"""

import re
from ..types import ClassInfo, FieldInfo, ParsedMethod
from ..constants import RUST_KEYWORDS as _RUST_KEYWORDS, OBJECT_CLASS as _OBJECT_CLASS

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


def _bin_to_rust_short(binary_name: str) -> str:
    """JVM binary 名 → Rust 类型名（末段，$ → _）。"""
    return binary_name.rsplit('/', 1)[-1].replace('$', '_')


def _compute_all_superclasses(ci: ClassInfo, registry: dict | None) -> list[str]:
    """计算线性超类链（不含接口），从最深祖先到直接父类，排除 java.lang.Object。

    返回 Rust short names（已做 binary_to_rust 转换）。
    用于 vtable impl 生成：idx=0 是最深祖先（字段声明者），字段 accessor 放在此处。
    """
    chain: list[str] = []
    cur = ci.super_class
    visited: set[str] = set()
    while cur and cur not in visited and cur != _OBJECT_CLASS:
        chain.append(_bin_to_rust_short(cur))
        visited.add(cur)
        if registry and cur in registry:
            cur = registry[cur].super_class
        else:
            break
    chain.reverse()  # 最深祖先在前（idx=0），直接父类在后
    return chain


def _compute_ancestor_fields_layout(ci: ClassInfo, registry: dict | None) -> list[tuple[str, list[str]]]:
    """计算每个祖先类自己声明的非静态字段列表（不含接口，不含 Object）。

    返回 [(ancestor_rust_name, [field_names]), ...] — 从最深祖先到直接父类排列。
    仅包含有非静态字段的祖先，空字段祖先不输出。
    用于 vtable impl 生成：宏可将字段 accessor 精确分发到声明该字段的祖先 vtable impl。
    """
    if not registry:
        return []
    chain: list[ClassInfo] = []
    cur = ci.super_class
    visited: set[str] = set()
    while cur and cur not in visited and cur != _OBJECT_CLASS and cur in registry:
        visited.add(cur)
        chain.append(registry[cur])
        cur = registry[cur].super_class
    chain.reverse()  # 最深祖先在前
    declared: set[str] = set()
    result: list[tuple[str, list[str]]] = []
    for anc in chain:
        rust_name = _bin_to_rust_short(anc.name)
        own_fields: list[str] = []
        for f in (anc.fields or []):
            if f.is_static:
                continue
            safe_name = f.name.lstrip('$').replace('$', '_')
            # 同名字段只声明一次（子类可能 shadow，取第一次出现）
            if safe_name not in declared:
                declared.add(safe_name)
                own_fields.append(safe_name)
        if own_fields:
            result.append((rust_name, own_fields))
    return result


def _compute_all_supertypes(ci: ClassInfo, registry: dict | None) -> list[str]:
    """计算类的所有超类型（自身 + 传递闭合的父类 + 接口），用于 instanceof 检查。

    返回值：JVM 二进制名列表，已排序去重。
    """
    supertypes: set[str] = {ci.name}
    if not registry:
        # 无 registry，只能加直接父类和直接接口
        if ci.super_class:
            supertypes.add(ci.super_class)
        supertypes.update(ci.interfaces or [])
        return sorted(supertypes)

    queue: list[str] = []
    if ci.super_class:
        queue.append(ci.super_class)
    queue.extend(ci.interfaces or [])

    visited: set[str] = set()
    while queue:
        name = queue.pop(0)
        if name in visited:
            continue
        visited.add(name)
        supertypes.add(name)
        if name in registry:
            parent = registry[name]
            if parent.super_class:
                queue.append(parent.super_class)
            queue.extend(parent.interfaces or [])

    return sorted(supertypes)


def _java_class_block_head(ci: ClassInfo, registry: dict | None = None,
                           superclass_rust: str = "",
                           superclass_fields: list[tuple[str, str]] | None = None) -> list[str]:
    """生成 `java_class! { ... }` 块内的类级别属性行（方案 §4）。

    输出约定（与 Java 源码「缺省即默认」一致，看重生成物可读性）：
    空串 / false / package 可见性（Java 默认）的键整行不写；
    `binary_name` 是身份键，无条件输出。

    分两段，用注释分隔，让读者一眼区分「字节码元数据」与「宏展开输入」：

      - 字节码元数据：binary_name / super_class / interfaces / access / …，
        纯记录用途，宏只读其中少数几个键；
      - 宏展开输入：superclass（Rust 类型文本）/ superclass_fields（codegen 展平）/
        all_supertypes（instanceof 静态展开）/ is_interface /
        has_to_string_method / has_hash_code_method。

    superclass_fields 的键值对由调用方（class_writer）从 registry 展平整条继承链得到，
    顺序必须是父类字段在前（JVM 内存布局，方案 §6）。
    """
    lines: list[str] = []
    looks = '// '

    # ── 段 1：字节码元数据（缺省即默认）──────────────────────────────────
    lines.append(f'{looks}── 字节码元数据 ' + '─' * 46)
    def _q(s: str) -> str:
        return s.replace('\\', '\\\\').replace('"', '\\"')

    lines.append(f'#[binary_name       = "{_q(ci.name)}"]')
    if ci.super_class:
        lines.append(f'#[super_class       = "{_q(ci.super_class)}"]')
    if ci.interfaces:
        lines.append(f'#[interfaces        = "{_q(",".join(ci.interfaces))}"]')
    if ci.access_flags:
        _access = _access_str(ci.access_flags)
        if _access != 'package':
            lines.append(f'#[access            = "{_access}"]')
        _mods = _class_modifiers_str(ci.access_flags)
        if _mods:
            lines.append(f'#[modifiers         = "{_mods}"]')
    if ci.generic_signature:
        lines.append(f'#[generic_signature = "{_q(ci.generic_signature)}"]')
    if ci.is_abstract:
        lines.append('#[is_abstract       = true]')
    if ci.is_enum:
        lines.append('#[is_enum           = true]')
    if ci.is_deprecated:
        lines.append('#[is_deprecated     = true]')
    if ci.source_file:
        lines.append(f'#[source            = "{_q(ci.source_file)}"]')
    if ci.inner_classes:
        ic_strs = ';'.join(
            f'{ic.inner_class}:{ic.outer_class}:{ic.inner_name}:{ic.access_flags}'
            for ic in ci.inner_classes
        )
        lines.append(f'#[inner_classes     = "{_q(ic_strs)}"]')

    # ── 段 2：宏展开输入（缺省即 Default）─────────────────────────────────
    lines.append('')
    lines.append(f'{looks}── 宏展开输入 ' + '─' * 46)
    if ci.is_interface:
        lines.append('#[is_interface      = true]')
    if superclass_rust:
        lines.append(f'#[superclass        = "{superclass_rust}"]')
    if superclass_fields:
        items = ', '.join(f'{n}: {t}' for n, t in superclass_fields)
        lines.append(f'#[superclass_fields({items})]')
    if not ci.is_interface:
        superclasses = _compute_all_superclasses(ci, registry)
        if superclasses:
            lines.append(f'#[all_superclasses  = "{";".join(superclasses)}"]')
        ancestor_fields = _compute_ancestor_fields_layout(ci, registry)
        if ancestor_fields:
            # 格式：AncName:field1,field2;AncName2:field3
            parts = [f'{name}:{",".join(fields)}' for name, fields in ancestor_fields]
            lines.append(f'#[ancestor_fields_layout = "{";".join(parts)}"]')
        supertypes = _compute_all_supertypes(ci, registry)
        if supertypes:
            lines.append(f'#[all_supertypes    = "{";".join(supertypes)}"]')
        _method_sigs = {(m.name, m.descriptor) for m in (ci.methods or [])}
        if ('toString', '()Ljava/lang/String;') in _method_sigs:
            lines.append('#[has_to_string_method = true]')
        if ('hashCode', '()I') in _method_sigs:
            lines.append('#[has_hash_code_method = true]')
    return lines


def _java_field_attr(f: FieldInfo) -> str:
    """生成 #[cfg_attr(any(), java_field(...))] 属性行（编译安全）。

    缺省即默认：is_static = false、package 可见性、空 modifiers 不输出；
    name / descriptor 是身份键，无条件输出。
    """
    parts = [f'name = "{f.name}"', f'descriptor = "{f.descriptor}"']
    if f.access_flags:
        _access = _access_str(f.access_flags)
        if _access != 'package':
            parts.append(f'access = "{_access}"')
        _mods = _field_modifiers_str(f.access_flags)
        if _mods:
            parts.append(f'modifiers = "{_mods}"')
    if f.is_static:
        parts.append('is_static = true')
    if f.generic_signature:
        sig = f.generic_signature.replace('"', '\\"')
        parts.append(f'generic_signature = "{sig}"')
    if f.constant_value:
        # constant_value already escaped by _constant_value_str (in classfile.py)
        parts.append(f'constant_value = "{f.constant_value}"')
    if f.is_deprecated:
        parts.append('is_deprecated = true')
    return '#[cfg_attr(any(), java_field(' + ', '.join(parts) + '))]'


def _java_method_attr(m: ParsedMethod) -> str:
    """生成方法元数据标注行（`java_class! { impl ... }` 块内路径）。

    方法一律写在 `java_class! { impl ... }` 块内，必须用单段路径
    `#[java_method(...)]` / `#[java_native(...)]`——块级宏按 ident 匹配并剥离这些
    元数据属性，不会把它们透传给方法（两段路径 `java_rta_macros::java_method`
    匹配不上，会被当作真实属性宏重新施加在方法上，而同名 proc-macro 已删除）。

    注意：这两个标签是纯文本，同名 proc-macro 不存在；build.rs 按文本前缀
    `#[java_native(` 扫描维护 native_status.toml，依赖的是这里的文本输出。

    缺省即默认：is_static / is_native / is_abstract / is_synthetic 为 false、
    package 可见性、空 modifiers 时不输出；name / descriptor 是身份键
    （build.rs 依赖），无条件输出。block.rs 只从中读 descriptor。
    """
    tag = 'java_native' if m.is_native else 'java_method'
    desc = m.descriptor.replace('"', '\\"')
    name = m.name.replace('"', '\\"')
    parts = [f'name = "{name}"', f'descriptor = "{desc}"']
    if m.access_flags:
        _access = _access_str(m.access_flags)
        if _access != 'package':
            parts.append(f'access = "{_access}"')
        _mods = _method_modifiers_str(m.access_flags)
        if _mods:
            parts.append(f'modifiers = "{_mods}"')
    if m.is_static:
        parts.append('is_static    = true')
    if m.is_native:
        parts.append('is_native    = true')
    if m.is_abstract:
        parts.append('is_abstract  = true')
    if m.is_synthetic:
        parts.append('is_synthetic = true')
    if m.exceptions:
        excs = ','.join(m.exceptions).replace('"', '\\"')
        parts.append(f'exceptions = "{excs}"')
    if m.generic_signature:
        sig = m.generic_signature.replace('"', '\\"')
        parts.append(f'generic_signature = "{sig}"')
    if m.is_deprecated:
        parts.append('is_deprecated = true')
    if getattr(m, 'virtual_in', ''):
        vin = m.virtual_in.replace('"', '\\"')
        parts.append(f'virtual_in = "{vin}"')
    if m.method_parameters:
        mp_str = ';'.join(f'{n}:{a}' for n, a in m.method_parameters).replace('"', '\\"')
        parts.append(f'method_parameters = "{mp_str}"')
    # native 需要显式标记（宏靠「无方法体」也认，但显式标记让文件读者一眼看出
    # 这是 native 声明）
    if m.is_native:
        return f'#[native]\n#[{tag}(' + ', '.join(parts) + ')]'
    return f'#[{tag}(' + ', '.join(parts) + ')]'
