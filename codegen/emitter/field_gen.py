"""字段类型解析器（从 class_writer._gen_class_rs 嵌套函数提升为模块级）。

- _extract_type_names：从 Rust 类型字符串提取所有类型名（含嵌套泛型参数）。
- _validate_field_type：递归校验 rust_ty 中类型名可用性（内建 / 类型参数 /
  注册表短名）；闭包变量 _registry_short_names 改为显式参数。
- _outer_ref_field_rust：this$N 外部类引用字段类型；闭包变量 registry 改为
  显式参数。
- _resolve_field_rust：字段 Rust 类型（字段级 generic_signature 优先，回退
  裸描述符）；闭包变量 class_type_params / registry / _registry_short_names
  改为显式参数。
- _resolve_anc_field_rust：祖先字段 Rust 类型（用祖先自身 tparams 解析签名，
  再按 anc_map 代入本类视角实参）；闭包变量 registry / _registry_short_names
  改为显式参数。
函数体逻辑零改动（提升后统一去一层缩进）。
依赖方向 class_writer / clinit_extract → field_gen 单向。
"""

from ..type_map import parse_field_type, jvm_to_rust, outer_ref_field_type

# 基础内建类型，不需要注册表校验
# JArray 是 Rust 端数组包装类型，不对应 Java 类，须手动加入
_BUILTIN_TYPES = frozenset({
    'Object', 'String', 'i32', 'i64', 'f32', 'f64', 'bool', 'u16',
    'i8', 'i16', 'u32', 'u64', '()', 'Rc', 'Vec', 'RefCell',
    'usize', 'u8', 'JArray',
})
# Rust 结构符号，不是类型名，跳过校验
_RUST_TOKENS = frozenset({'', 'mut', 'dyn', 'static', 'impl'})


def _extract_type_names(rust_ty: str) -> list[str]:
    """从 Rust 类型字符串提取所有类型名（包含嵌套泛型参数中的类型）。"""
    names: list[str] = []
    current: list[str] = []
    for ch in rust_ty:
        if ch in ('<', '>', ',', ' ', '&', "'", '[', ']', ':'):
            word = ''.join(current).strip()
            if word:
                names.append(word)
            current = []
        else:
            current.append(ch)
    word = ''.join(current).strip()
    if word:
        names.append(word)
    return names


def _validate_field_type(rust_ty: str, type_params: list[str],
                         _registry_short_names: set[str]) -> bool:
    """递归检查 rust_ty 中所有类型名是否可用（内建/类型参数/注册表中存在）。
    若任何嵌套类型名未知，返回 False，调用方将回退到裸描述符类型。"""
    # crate:: 全路径（_iface_full_path 生成，如 crate::java::util::Iterator）：
    # 路径段 java/util/lang 不在内建集合里，但整体是有效引用，直接通过
    import re as _re_fp
    cleaned = _re_fp.sub(r'\bcrate(?:::\w+)+\b', 'Object', rust_ty)
    for name in _extract_type_names(cleaned):
        if name in _RUST_TOKENS:
            continue
        if name in _BUILTIN_TYPES or name in type_params or name in _registry_short_names:
            continue
        return False  # 有未知类型名，校验失败
    return True


def _outer_ref_field_rust(f, decl_params: list, registry) -> str:
    return outer_ref_field_type(f, decl_params, registry)


def _resolve_field_rust(f, class_type_params: list, registry,
                        _registry_short_names: set[str]) -> str:
    """字段的 Rust 类型：优先字段级 generic_signature（TE; → E），回退裸描述符。
    generic_signature 解析为 Object，或引用了不存在的类型时，用描述符推断。"""
    _outer_rust = _outer_ref_field_rust(f, class_type_params, registry)
    if _outer_rust:
        return _outer_rust
    gen_rust = (parse_field_type(f.generic_signature, class_type_params, registry)
                if f.generic_signature else '')
    desc_rust = jvm_to_rust(f.descriptor, registry)
    return (gen_rust
            if gen_rust and gen_rust != 'Object'
            and _validate_field_type(gen_rust, class_type_params, _registry_short_names)
            else desc_rust)


def _resolve_anc_field_rust(f, anc_params: list, anc_map: dict, registry,
                            _registry_short_names: set[str]) -> str:
    """祖先字段的 Rust 类型：用祖先自己的 tparams 解析签名，再按 anc_map
    （祖先形参 → 本类视角实参，与 parent_rust / all_superclasses 同源）代入。
    否则父类字段变量（如 AbstractRepository<T> 的 tree: T）被子类 tparams
    （['S']）解析成 Object，转发访问器 __set_tree(v: Object) 与父类
    AbstractRepository<S> 的 __set_tree(v: S) E0308。"""
    # this$N：与祖先自身 struct 的字段类型同规则（见 _outer_ref_field_rust），再代入实参
    gen_rust = _outer_ref_field_rust(f, anc_params, registry) or (
        parse_field_type(f.generic_signature, anc_params, registry)
        if f.generic_signature else '')
    if gen_rust and gen_rust != 'Object' and _validate_field_type(
            gen_rust, anc_params, _registry_short_names):
        if anc_map:
            import re as _re_am
            gen_rust = _re_am.sub(
                r'\b[A-Za-z_]\w*\b',
                lambda m: anc_map.get(m.group(0), m.group(0)),
                gen_rust)
        return gen_rust
    return jvm_to_rust(f.descriptor, registry)
