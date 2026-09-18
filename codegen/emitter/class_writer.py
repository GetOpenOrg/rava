"""
单个 Java 类 → Rust 文件内容生成：_gen_class_rs 主函数。
"""

import os
from collections import Counter
from ..types import ClassInfo, FieldInfo, ParsedMethod
from ..type_map import jvm_to_rust, mangle_name, short_cls, rust_default
from ..method import gen_method_body, _indent
from ..type_map import parse_class_type_params, parse_field_type
from ..constants import safe_ident, RUST_KEYWORDS as _RUST_KEYWORDS
from .attrs import (to_snake, _java_class_block_head,
                    _java_field_attr, _java_method_attr)
from .method_gen import _gen_native_stub
from .vtable_util import _bin_to_rust, _find_virtual_in
from .clinit_extract import _push_int_value, _extract_clinit_consts, _extract_clinit_arrays

_safe_field_name = safe_ident

_ACC_FINAL   = 0x0010
_ACC_STATIC  = 0x0008


def _gen_class_rs(ci: ClassInfo, registry: dict | None = None,
                  jdk_crate_pkg_paths: list[str] | None = None,
                  stub_bodies: bool = False,
                  call_chain: set | None = None,
                  new_format_map: dict | None = None,
                  workspace_root: str | None = None,
                  user_crate_prefix: str | None = None,
                  full_impl_classes: set | None = None,
                  conflict_map: dict | None = None,
                  skipped_classes: set | None = None,
                  user_sibling_imports: list[str] | None = None,
                  generated_classes: set | None = None) -> str:
    """生成单个 Java 类对应的完整 .rs 文件内容。

    生成规则：
    - 实例字段用 Field<T> 包装（提供 Java 字段语义的内部可变性）
    - 方法直接在 impl 块中，无 raw:: 子模块
    - 所有方法返回 Result<T>
    - 每个 struct / field / method 前加 // @java_* 注释供 build.rs 扫描
    - new_format_map: 若提供，为覆盖的类插入 #[path] mod _impl; 并跳过被覆盖方法
    - user_crate_prefix: 若提供（如 'jdk_classes'），cross_imports 用该 crate 前缀
    """

    # ── Step 1: 始终计算引用集合（精确 use 生成的基础）─────────────────────────
    import re as _re

    def _strip_generic(cls: str) -> str:
        """去掉 JVM 类名中的泛型参数（<...>），返回裸 binary name。
        例：java/util/Collection<*> → java/util/Collection"""
        idx = cls.find('<')
        return cls[:idx] if idx >= 0 else cls

    _referenced: set[str] = set()
    # 超类链：宏为每个祖先生成 From<Self> for Ancestor，需要全部祖先类型名在作用域内
    _sc_cur = ci.super_class
    while _sc_cur and _sc_cur != 'java/lang/Object':
        _referenced.add(_sc_cur)
        if registry and _sc_cur in registry:
            _sc_cur = registry[_sc_cur].super_class
        else:
            break
    for _iface in (ci.interfaces or []):
        if _iface != 'java/lang/Object':
            _referenced.add(_iface)
    # 扫描方法指令中的类型引用
    for _m in ci.methods:
        for _instr in (_m.instrs or []):
            _c = _instr.comment
            if not _c:
                continue
            if _c.startswith(('Method ', 'InterfaceMethod ')):
                _rest = _c.split(' ', 1)[1]
                _dot = _rest.find('.')
                if _dot > 0:
                    _referenced.add(_rest[:_dot])
            elif _c.startswith('Field '):
                _rest = _c[6:]
                _dot = _rest.find('.')
                if _dot > 0:
                    _referenced.add(_rest[:_dot])
    # 扫描字段描述符（含超类链继承字段）
    _all_fields_to_scan = list(ci.fields)
    if registry:
        _sc_scan = ci.super_class
        _seen_scan: set[str] = {f.name for f in ci.fields}
        while _sc_scan and _sc_scan != 'java/lang/Object' and _sc_scan in registry:
            _sci_scan = registry[_sc_scan]
            for _f2 in _sci_scan.fields:
                if not _f2.is_static and _f2.name not in _seen_scan:
                    _all_fields_to_scan.append(_f2)
                    _seen_scan.add(_f2.name)
            _sc_scan = _sci_scan.super_class
    # 泛型签名中嵌套类型需要用更宽松的 regex（不能用 [^;]+ 因为嵌套 <TT;> 会截断）
    _cls_re_narrow = _re.compile(r'L([^;]+);')          # 平铺 descriptor（如 (LFoo;)V）
    _cls_re_wide = _re.compile(r'L([^;<>\[()\s]+)')     # 含嵌套泛型的 generic_signature
    for _f in _all_fields_to_scan:
        for _m in _cls_re_narrow.finditer(_f.descriptor or ''):
            _c2 = _strip_generic(_m.group(1))
            if _c2: _referenced.add(_c2)
        for _m in _cls_re_wide.finditer(_f.generic_signature or ''):
            _c2 = _strip_generic(_m.group(1))
            if _c2: _referenced.add(_c2)
    # 扫描方法描述符（参数和返回值）
    for _method in ci.methods:
        for _m in _cls_re_narrow.finditer(_method.descriptor or ''):
            _c2 = _strip_generic(_m.group(1))
            if _c2: _referenced.add(_c2)
        for _m in _cls_re_wide.finditer(getattr(_method, 'generic_signature', '') or ''):
            _c2 = _strip_generic(_m.group(1))
            if _c2: _referenced.add(_c2)

    # 收集 invokeinterface/invokevirtual 调度分支中引用的子类型
    # （dispatch 链 downcast_ref::<SubType>() 需要 SubType 在作用域内）
    if registry:
        from ..instr.coerce import _get_all_subtypes_ordered as _gaso
        from ..type_map import is_jdk as _is_jdk
        _iface_refs: set[str] = set()
        for _m in ci.methods:
            for _instr in (_m.instrs or []):
                _c = _instr.comment
                if not _c:
                    continue
                if _c.startswith(('InterfaceMethod ', 'Method ')):
                    _rest = _c.split(' ', 1)[1]
                    _dot = _rest.find('.')
                    if _dot > 0:
                        _iface_refs.add(_rest[:_dot])
        for _iface_bin in _iface_refs:
            _iface_ci = registry.get(_iface_bin)
            if _iface_ci is None:
                continue
            for _sub_bin in _gaso(_iface_bin, registry):
                _sub_bin_clean = _strip_generic(_sub_bin)
                if _is_jdk(_iface_bin):
                    if _is_jdk(_sub_bin_clean):
                        _referenced.add(_sub_bin_clean)
                else:
                    _referenced.add(_sub_bin_clean)

    # 过滤：只保留实际会生成到 scratch 的类型引用，避免为 stub 方法签名里的类型
    # 生成 use 语句（那些类型不在 scratch 里，会导致 E0432）。
    if generated_classes is not None:
        _referenced = {c for c in _referenced if c in generated_classes}

    # ── Step 2: 精确 cross_imports（按需逐类型导入，不使用包级 glob）───────────
    cross_imports: list[str] = []
    _prefix = user_crate_prefix or 'crate'
    _self_simple = (ci.name.split('/')[-1] if '/' in ci.name else ci.name).replace('$', '_')
    _seen_imports: set[str] = set()  # 去重键："{rust_pkg}::{simple}"

    # prelude 里已有的泛型/newtype 名称，若 Java 类名与其重名，跳过 use 导入
    # 调用方通过全路径（crate::java::...::Class）引用，不用短名
    _PRELUDE_NEWTYPE_NAMES = {'JArray'}

    def _add_precise_import(full_cls: str) -> None:
        """按 JVM binary name 添加精确 use 语句，跳过自身类型和重复项。"""
        _parts = full_cls.split('/')
        if len(_parts) < 2:
            return
        _rust_pkg = '::'.join(f'r#{p}' if p in _RUST_KEYWORDS else p for p in _parts[:-1])
        _simple = _parts[-1].replace('$', '_')
        if _simple == _self_simple:
            return
        _key = f"{_rust_pkg}::{_simple}"
        if _key in _seen_imports:
            return
        _seen_imports.add(_key)
        if _simple in _PRELUDE_NEWTYPE_NAMES:
            # 名称与 prelude newtype 冲突，跳过 use 导入；调用方应使用全路径引用
            return
        cross_imports.append(f"use {_prefix}::{_rust_pkg}::{_simple};")

    # JDK 包（jdk_crate_pkg_paths 中的包）：按需精确导入
    # jdk_crate_pkg_paths 是 Rust 路径（java::lang），_referenced 是 JVM 路径（java/lang）
    # 转换为同一格式再比对
    if jdk_crate_pkg_paths:
        _pkg_set_slash = {
            p.replace('::', '/').replace('r#', '') for p in jdk_crate_pkg_paths
        }
        for _full_cls in sorted(_referenced):
            _parts = _full_cls.split('/')
            if len(_parts) >= 2 and '/'.join(_parts[:-1]) in _pkg_set_slash:
                _add_precise_import(_full_cls)

    # 同包兄弟类：按需精确导入（若自身包未在 jdk_crate_pkg_paths 中）
    if ci.name and '/' in ci.name:
        _own_pkg = '/'.join(ci.name.split('/')[:-1])
        _own_pkg_path = '::'.join(
            f'r#{p}' if p in _RUST_KEYWORDS else p for p in ci.name.split('/')[:-1]
        )
        _existing_pkgs = set(jdk_crate_pkg_paths) if jdk_crate_pkg_paths else set()
        if _own_pkg_path not in _existing_pkgs:
            for _full_cls in sorted(_referenced):
                _parts = _full_cls.split('/')
                if len(_parts) >= 2 and '/'.join(_parts[:-1]) == _own_pkg:
                    _add_precise_import(_full_cls)

    # 消歧：同一简名存在于多个包时，精确 use 覆盖（conflict_map 仍需处理）
    if conflict_map:
        _own_pkg_cm = '/'.join(ci.name.split('/')[:-1]) if '/' in ci.name else ''

        def _pkg_to_use(pkg_slash: str) -> str:
            return _prefix + '::' + '::'.join(
                f'r#{p}' if p in _RUST_KEYWORDS else p
                for p in pkg_slash.split('/')
            )

        for _sn, _pkgs in conflict_map.items():
            if _sn.replace('$', '_') == _self_simple:
                continue
            _matches = [p for p in _pkgs if f'{p}/{_sn}' in _referenced]
            if len(_matches) == 1:
                _chosen = _matches[0]
            elif _own_pkg_cm in _pkgs:
                _chosen = _own_pkg_cm
            elif _matches:
                _chosen = _matches[0]
            else:
                _java = [p for p in _pkgs if p.startswith('java/')]
                _chosen = _java[0] if _java else _pkgs[0]
            cross_imports.append(f"use {_pkg_to_use(_chosen)}::{_sn.replace('$', '_')};")

    # 被跳过包（如 jdk/）中的类型：按需精确导入
    if skipped_classes and _referenced:
        _added_skipped: set[str] = set()
        for _full_cls in sorted(_referenced):
            _cls_parts = _full_cls.split('/')
            if len(_cls_parts) < 2:
                continue
            _rust_pkg = '::'.join(
                f'r#{p}' if p in _RUST_KEYWORDS else p for p in _cls_parts[:-1]
            )
            _simple = _cls_parts[-1].replace('$', '_')
            if (f"{_rust_pkg}::{_simple}" in skipped_classes
                    and _simple != _self_simple
                    and _simple not in _added_skipped):
                cross_imports.append(f"use {_prefix}::{_rust_pkg}::{_simple};")
                _added_skipped.add(_simple)

    # VTable trait 导入：沿超类链为每个祖先类导入 Ancestor__VTable。
    # java_class! 宏生成 impl Ancestor__VTable for Self__inner，需要该 trait 在作用域内。
    # 接口不生成 VTable trait（接口展开为 pub type Iface = Object;），故只处理非接口超类链。
    if not ci.is_interface and registry:
        _vtable_cur = ci.super_class
        while _vtable_cur and _vtable_cur != 'java/lang/Object':
            if generated_classes is None or _vtable_cur in generated_classes or '/' not in _vtable_cur:
                _vp = _vtable_cur.split('/')
                if len(_vp) >= 2:
                    _vpkg = '::'.join(f'r#{p}' if p in _RUST_KEYWORDS else p for p in _vp[:-1])
                    _vsimple = _vp[-1].replace('$', '_')
                    _vkey = f"{_vpkg}::{_vsimple}__VTable"
                    if _vkey not in _seen_imports:
                        _seen_imports.add(_vkey)
                        cross_imports.append(f"use {_prefix}::{_vpkg}::{_vsimple}__VTable;")
                elif len(_vp) == 1:
                    # user class without package path (no '/') — vtable is in same user crate
                    _vsimple = _vp[0].replace('$', '_')
                    _mod_n = to_snake(_vp[0])
                    _vkey = f"crate::{_mod_n}::{_vsimple}__VTable"
                    if _vkey not in _seen_imports:
                        _seen_imports.add(_vkey)
                        cross_imports.append(f"use crate::{_mod_n}::{_vsimple}__VTable;")
            _vtable_cur = registry[_vtable_cur].super_class if _vtable_cur in registry else None

    # __base 函数导入：扫描方法字节码中的非 <init> invokespecial 指令
    # 生成的 Rust 代码会调用 ParentClass__method_base(this, ...) 自由函数，
    # 需导入该函数所在模块（包括 JDK 父类，如 AbstractStringBuilder）。
    if not ci.is_interface:
        import re as _re2
        from ..instr.coerce import parse_method_ref as _pmr
        for _m in ci.methods:
            # 只为在调用链上（有实际方法体）的方法生成 __base 函数导入
            # stub 方法的字节码中有 invokespecial 但不会实际调用，不需要 cross-import
            _m_in_chain = call_chain is None or (ci.name, _m.name, _m.descriptor) in call_chain
            if not _m_in_chain:
                continue
            for _ins in getattr(_m, 'instrs', []) or []:
                if getattr(_ins, 'opcode', '') != 'invokespecial':
                    continue
                _c = getattr(_ins, 'comment', '') or ''
                if not _c or '<init>' in _c:
                    continue
                # InterfaceMethod invokespecial → 接口默认方法，接口无 __base 自由函数
                if _c.strip().startswith('InterfaceMethod '):
                    continue
                _cls_s, _mname_s, _params_s, _ret_s = _pmr(_c)
                if not _cls_s:
                    continue
                _orig_cls = _c.replace('Method ', '').replace('InterfaceMethod ', '')
                _orig_cls = _orig_cls.split('.')[0] if '.' in _orig_cls else _orig_cls
                _is_jdk = '/' in _orig_cls
                if _is_jdk:
                    # JDK 类：仅在父类已生成（在 generated_classes 中）时才导入 __base 函数
                    if _orig_cls not in (generated_classes or set()):
                        continue
                    # 额外检查：_orig_cls 必须在 registry 中自己定义该方法（非继承来的），
                    # 否则 invokespecial 引用的是祖先方法，_orig_cls 不会生成 __base 函数
                    if registry and _orig_cls in registry:
                        _anc_ci = registry[_orig_cls]
                        if not any(am.name == _mname_s for am in _anc_ci.methods):
                            continue
                    # 从 binary name（java/lang/AbstractStringBuilder）构建完整模块路径
                    _binary_parts = _orig_cls.split('/')
                    *_pkg, _simple_cls = _binary_parts
                    _base_cls_simple = _simple_cls.replace('$', '_')
                    _snake_cls = to_snake(_simple_cls)
                    _base_mod = '::'.join(
                        f'r#{p}' if p in _RUST_KEYWORDS else p
                        for p in _pkg + [_snake_cls]
                    )
                else:
                    # user class parent method
                    _base_cls_simple = _orig_cls.replace('$', '_')
                    _base_mod = to_snake(_orig_cls)
                from ..instr.coerce import _mangle_if_overloaded as _mio
                _rust_mname_s = _mio(_cls_s, _mname_s, _c, registry)
                from ..instr.coerce import _safe_field as _sf
                _rust_mname_s = _sf(_rust_mname_s)
                _base_fn = f"{_base_cls_simple}__{_rust_mname_s}_base"
                _bkey = f"crate::{_base_mod}::{_base_fn}"
                if _bkey not in _seen_imports:
                    _seen_imports.add(_bkey)
                    cross_imports.append(f"use crate::{_base_mod}::{_base_fn};")

    # 用户内部类兄弟模块导入（crate::mod_name::TypeName）
    if user_sibling_imports:
        cross_imports.extend(user_sibling_imports)

    # 全量手写类（native_impl 文件含 pub struct）：codegen 跳过 struct 生成，改输出 pub use _impl::*
    _full_impl = ci.name in (full_impl_classes or set())

    # Arch-1：接口在 Rust 层是 `pub type Name = Object;` 类型别名（由 java_class 宏生成），
    # 不存在可承载实例/静态方法的 Rust 类型，因此不生成任何 impl 块。
    # 注：接口静态方法（如 List.of）在当前架构下无归宿，属已知缺口。
    _is_iface = bool(ci.is_interface)

    parts: list[str] = [
        "#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]",
        f"use {user_crate_prefix or 'crate'}::prelude::*;",
        *cross_imports,
        "",
    ]
    inst_fields = [f for f in ci.fields if not f.is_static]

    # Option C（方案 §16「_super 语义边界」）：`_super` 是父类状态的**唯一所有者**，
    # 子类通过宏生成的转发访问器获得「展平字段视图」。
    # 不把父类字段复制进子类 Inner——那会让同一字段存在两份状态并立即分叉。
    _has_super = bool(
        ci.super_class and ci.super_class != 'java/lang/Object'
    )

    # 用短名作为 Rust 标识符（JDK 类含 /，内部类含 $，均需转换为合法 Rust 名）
    struct_name = short_cls(ci.name) if ('/' in ci.name or '$' in ci.name) else ci.name

    # 解析类级泛型参数（如 ArrayList<E>、HashMap<K,V>）
    class_type_params = parse_class_type_params(ci.generic_signature) if ci.generic_signature else []

    # 构建泛型参数字符串（用于 struct 和 impl 头）
    if class_type_params:
        type_params_str = ', '.join(class_type_params)
        # 泛型参数需要 Clone + Default + 'static。
        # 这组 bound 是纯粹的 Rust 能力声明——故意不引入 `JavaType` 这类
        # 游离于 Java 命名空间之外的 trait 名（CLAUDE.md 命名原则，方案 §4）。
        # Default 是必须的：`_super: Default::default()` 与字段默认值都要求它。
        bounds_str = ', '.join(f"{p}: Clone + Default + 'static" for p in class_type_params)
        struct_generic = f"<{type_params_str}>"   # bounds 由 java_class! 宏展开时注入，不出现在生成代码
        ty_params_only = f"<{type_params_str}>"
        # impl 头只写裸参数：块级宏用 struct 上的 generics（含补齐的 bound 与 where 子句）
        # 重新生成 impl 头，这里的 impl generics 仅作读者提示。
        impl_header   = f"impl<{type_params_str}> {struct_name}<{type_params_str}>"
    else:
        struct_generic = ''
        ty_params_only = ''
        impl_header   = f"impl {struct_name}"

    # 构建注册表短名集合，用于校验字段类型中引用的类是否存在
    _registry_short_names: set[str] = (
        {_k.rsplit('/', 1)[-1].replace('$', '_') for _k in registry}
        if registry else set()
    )
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
        """从 Rust 类型字符串中提取所有类型名（包含嵌套泛型参数中的类型）。"""
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

    def _validate_field_type(rust_ty: str, type_params: list[str]) -> bool:
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

    # ── 父类 Rust 类型（含泛型实参）─────────────────────────────────────
    parent_rust = ''
    if _has_super:
        parent_rust = short_cls(ci.super_class)
        if registry and ci.super_class in registry:
            parent_ci = registry[ci.super_class]
            parent_params = parse_class_type_params(parent_ci.generic_signature) if parent_ci.generic_signature else []
            if parent_params:
                if class_type_params:
                    # 子类有泛型参数：传播给父类，不足的用 Object 填充
                    args = class_type_params[:len(parent_params)]
                    while len(args) < len(parent_params):
                        args.append('Object')
                    parent_rust += '<' + ', '.join(args) + '>'
                else:
                    # 子类无泛型参数但父类需要（如 CharacterUnicodeScript extends Enum<E>）：用 Object 后备
                    parent_rust += '<' + ', '.join('Object' for _ in parent_params) + '>'

    def _resolve_field_rust(f) -> str:
        """字段的 Rust 类型：优先字段级 generic_signature（TE; → E），回退裸描述符。
        generic_signature 解析为 Object，或引用了不存在的类型时，用描述符推断。"""
        gen_rust = (parse_field_type(f.generic_signature, class_type_params, registry)
                    if f.generic_signature else '')
        desc_rust = jvm_to_rust(f.descriptor, registry)
        return (gen_rust
                if gen_rust and gen_rust != 'Object'
                and _validate_field_type(gen_rust, class_type_params)
                else desc_rust)

    def _resolve_anc_field_rust(f, anc_params: list, anc_map: dict) -> str:
        """祖先字段的 Rust 类型：用祖先自己的 tparams 解析签名，再按位置映射
        （与 T55 From 链 / parent_rust 的 args 构造一致）替换为子类可见参数。
        否则父类字段变量（如 AbstractRepository<T> 的 tree: T）被子类 tparams
        （['S']）解析成 Object，转发访问器 __set_tree(v: Object) 与父类
        AbstractRepository<S> 的 __set_tree(v: S) E0308。"""
        gen_rust = (parse_field_type(f.generic_signature, anc_params, registry)
                    if f.generic_signature else '')
        if gen_rust and gen_rust != 'Object' and _validate_field_type(gen_rust, anc_params):
            if anc_map:
                import re as _re_am
                gen_rust = _re_am.sub(
                    r'\b[A-Za-z_]\w*\b',
                    lambda m: anc_map.get(m.group(0), m.group(0)),
                    gen_rust)
            return gen_rust
        return jvm_to_rust(f.descriptor, registry)

    # ── 继承链字段展平（方案 §6）────────────────────────────────────────
    # codegen 侧展平整条继承链，父类字段在前；宏侧零 registry 依赖。
    # 注意：展平结果只用于生成「转发访问器」，父类字段的实际存储在 `_super` 里
    # （见 §16：复制字段会造成同一字段两份状态）。
    superclass_fields: list[tuple[str, str]] = []
    if _has_super and registry and not _full_impl:
        _chain: list = []
        _seen_chain: set[str] = set()
        _cursor = ci.super_class
        while (_cursor and _cursor != 'java/lang/Object'
               and _cursor in registry and _cursor not in _seen_chain):
            _seen_chain.add(_cursor)
            _p_ci = registry[_cursor]
            _chain.append(_p_ci)
            _cursor = _p_ci.super_class
        _declared: set[str] = set()
        for _ancestor in reversed(_chain):
            # 祖先参数 → 子类参数的位置映射（T55 一致）：不足补 Object
            _anc_params = (parse_class_type_params(_ancestor.generic_signature)
                           if _ancestor.generic_signature else [])
            _sub_args = list(class_type_params[:len(_anc_params)])
            while len(_sub_args) < len(_anc_params):
                _sub_args.append('Object')
            _anc_map = dict(zip(_anc_params, _sub_args))
            for _f in _ancestor.fields:
                if _f.is_static:
                    continue
                _sf_name = _safe_field_name(_f.name)
                if _sf_name in _declared:
                    continue
                _declared.add(_sf_name)
                superclass_fields.append(
                    (_sf_name, _resolve_anc_field_rust(_f, _anc_params, _anc_map)))

    # ── struct 声明（裸类型，封装细节由宏收拢）──────────────────────────
    struct_lines: list[str] = []
    if not _full_impl:
        # 父类已有的字段名（继承展平），避免子类重复声明（如内部类 this$0 与父类同名）
        _super_field_names: set[str] = {name for name, _ in superclass_fields}
        for f in inst_fields:
            safe_fname = _safe_field_name(f.name)
            if safe_fname in _super_field_names:
                continue  # 父类已展平，不重复声明
            struct_lines.append("    " + _java_field_attr(f))
            struct_lines.append(f"    pub {safe_fname}: {_resolve_field_rust(f)},")
        # 未被字段引用的类型参数由宏补 PhantomData（block.rs），codegen 不再输出 _phantom


    # T76 的 as_xxx / into_xxx upcast 方法已由 java_class! 宏统一承接
    # （宏生成 `__super()` / `__into_super()`，upcast 与字段存储解耦，见方案 §16）。

    # T55 已删除（R-2）：
    # From<Child> for Parent 链由 invoke.py / sim.py 的显式 __into_super() 链替代，
    # 宏为有父类的类生成 Deref<Target=Parent> 覆盖引用层面的向上转型。
    # upcast 调用点：Clone::clone(&child).__into_super().__into_super()...（见 coerce._into_super_chain）

    # T55b 已删除（Arch-5）：
    # Arch-1 后接口 = Object 类型别名，From<ConcreteClass> for Interface 语义上等于
    # From<ConcreteClass> for Object，与 java_class 宏生成的 Into<Object> 冲突且
    # 会用 Default::default() 丢弃具体数据。
    # 正确路径：ConcreteClass.into() → Object，通过 java_class 宏生成的 Into<Object> 完成。
    # K-2: 共置 _impl.rs 文件由 project_writer 在生成阶段复制；class_writer 不再生成 #[path] 块。
    _nf_entry = (new_format_map or {}).get(ci.name)

    # 过滤 synthetic 方法（编译器合成桥接方法），再统计重载
    visible_methods = [m for m in ci.methods if not m.is_synthetic]
    # 预扫描接口 default 方法，合并到名字计数中，确保定义与调用侧 mangle 一致
    _pre_default_names: list[str] = []
    if ci.interfaces and registry and not ci.is_interface:
        _pre_sigs = {(m.name, m.descriptor) for m in visible_methods}
        _pre_q = list(ci.interfaces)
        _pre_vis: set[str] = set()
        while _pre_q:
            _pn = _pre_q.pop(0)
            if _pn in _pre_vis:
                continue
            _pre_vis.add(_pn)
            _pi = registry.get(_pn)
            if _pi:
                _pre_q.extend(_pi.interfaces or [])
                for _dm in _pi.methods:
                    if (not _dm.is_abstract and not _dm.is_static and not _dm.is_synthetic
                            and _dm.name not in ('<init>', '<clinit>')
                            and (_dm.name, _dm.descriptor) not in _pre_sigs):
                        _pre_default_names.append(_dm.name)
                        _pre_sigs.add((_dm.name, _dm.descriptor))
    name_counts = Counter(m.name for m in visible_methods if m.name != '<clinit>') + Counter(_pre_default_names)
    overloaded_names: set[str] = {name for name, count in name_counts.items() if count > 1}

    method_blocks: list[str] = []
    module_statics: list[str] = []  # 模块级 static 声明（OnceLock 等），插在 impl 块前

    # 是否是用户类（call_chain is None 表示用户类，所有方法都翻译）
    _is_user_class = call_chain is None
    # 用户类中有 <clinit> 静态初始化器时，main() 需调用 class_init()
    _has_clinit = any(m.name == '<clinit>' for m in ci.methods)

    # public static 字段的 getter 方法（用于 getstatic 访问，如 System::out()）
    # 生成静态字段 getter：有 _impl 覆盖的跳过，其余生成 panic stub 或 constant_value
    static_fields = [f for f in ci.fields if f.is_static]
    existing_method_names: set[str] = {m.name for m in visible_methods}
    _nf_covered_sf = (_nf_entry or {}).get('methods', set())
    # 从 <clinit> 提取简单常量赋值（补充 ConstantValue attribute 未覆盖的情况）
    _clinit_consts = _extract_clinit_consts(ci)
    # 从 <clinit> 提取常量数组初始化（newarray + dup/index/value/xastore 模式）
    _clinit_arrays = _extract_clinit_arrays(ci)
    # JVM 原始类型描述符集合，可安全包装在 OnceLock<Mutex<T>> 中（均实现 Send + Copy）
    # String（Java 自定义类型）不在此集合，因其含 Rc 字段，不实现 Send
    _MUTABLE_STATIC_DESCS = frozenset({'I', 'J', 'F', 'D', 'Z', 'B', 'C', 'S'})
    for sf in ([] if _is_iface else static_fields):
        safe_fname = _safe_field_name(sf.name)
        if safe_fname in existing_method_names:
            # 字段名与方法名冲突：改用 _field 后缀，让 getstatic 仍能访问该字段
            safe_fname = safe_fname + '_field'
        # 若 _impl 已覆盖此静态字段访问器，跳过
        if safe_fname in _nf_covered_sf:
            continue
        # 优先用 generic_signature 确定返回类型（包含泛型参数信息）
        if sf.generic_signature:
            _gs_ret = parse_field_type(sf.generic_signature, class_type_params, registry)
            # 校验引用的类型存在，否则回退到描述符
            if _gs_ret and _gs_ret != 'Object' and not _validate_field_type(_gs_ret, class_type_params):
                _gs_ret = ''
        else:
            _gs_ret = ''
        rust_ret = _gs_ret if _gs_ret else jvm_to_rust(sf.descriptor, registry=registry)
        # ConstantValue attribute 优先；其次尝试从 <clinit> 提取简单常量
        cv = sf.constant_value or _clinit_consts.get(sf.name, '')
        if cv:
            if cv == '__EMPTY_ARRAY__':
                # iconst_0 → anewarray → putstatic：static final T[] = new T[0]
                body = 'JArray::new(0)'
            elif rust_ret == 'String':
                body = f'String::from("{cv}")'
            elif rust_ret == 'f32':
                if cv == 'inf':      body = 'f32::INFINITY'
                elif cv == '-inf':   body = 'f32::NEG_INFINITY'
                elif cv == 'NaN':    body = 'f32::NAN'
                else:                body = f'{cv}f32'
            elif rust_ret == 'f64':
                if cv == 'inf':      body = 'f64::INFINITY'
                elif cv == '-inf':   body = 'f64::NEG_INFINITY'
                elif cv == 'NaN':    body = 'f64::NAN'
                else:                body = f'{cv}f64'
            elif rust_ret == 'i64':
                body = f'{cv}i64'
            elif rust_ret == 'bool':
                body = 'true' if cv == '1' else 'false'
            else:
                body = cv
            field_meta = _java_field_attr(sf)
            method_blocks.append(f'{field_meta}\n// static field: {sf.name}:{sf.descriptor}\npub fn {safe_fname}() -> {rust_ret} {{\n    {body}\n}}')
        elif _is_user_class and sf.descriptor in _MUTABLE_STATIC_DESCS:
            # 用户类可变静态字段（JVM 原始类型，Send + Copy）：OnceLock<Mutex<T>>
            _static_var = f"_{struct_name}_{safe_fname}_STATIC"
            _default = rust_default(rust_ret)
            module_statics.append(
                f"static {_static_var}: std::sync::OnceLock<std::sync::Mutex<{rust_ret}>> = std::sync::OnceLock::new();"
            )
            field_meta = _java_field_attr(sf)
            method_blocks.append(
                f'{field_meta}\n// static field: {sf.name}:{sf.descriptor}\n'
                f'pub fn {safe_fname}() -> {rust_ret} {{\n'
                f'    *{_static_var}.get_or_init(|| std::sync::Mutex::new({_default})).lock().unwrap()\n'
                f'}}'
            )
            method_blocks.append(
                f'// static field setter: {sf.name}\n'
                f'pub fn set_{safe_fname}(v: {rust_ret}) {{\n'
                f'    *{_static_var}.get_or_init(|| std::sync::Mutex::new({_default})).lock().unwrap() = v;\n'
                f'}}'
            )
        elif _is_user_class:
            # 用户类可变静态字段（非原始类型，不实现 Send）：unsafe static mut Option<T>
            _static_var = f"_{struct_name}_{safe_fname}_STATIC"
            _default = rust_default(rust_ret)
            module_statics.append(
                f"static mut {_static_var}: Option<{rust_ret}> = None;"
            )
            field_meta = _java_field_attr(sf)
            method_blocks.append(
                f'{field_meta}\n// static field: {sf.name}:{sf.descriptor}\n'
                f'pub fn {safe_fname}() -> {rust_ret} {{\n'
                f'    unsafe {{ {_static_var}.clone().unwrap_or_default() }}\n'
                f'}}'
            )
            method_blocks.append(
                f'// static field setter: {sf.name}\n'
                f'pub fn set_{safe_fname}(v: {rust_ret}) {{\n'
                f'    unsafe {{ {_static_var} = Some(v); }}\n'
                f'}}'
            )
        elif sf.name in _clinit_arrays:
            # <clinit> 中识别到 newarray + dup/index/value/xastore 模式：生成常量数组
            _arr_vals = _clinit_arrays[sf.name]
            _desc_inner = sf.descriptor[1:]   # [B→B, [C→C, [S→S, [I→I
            _elem_rust = {'B': 'i8', 'C': 'u16', 'S': 'i16', 'I': 'i32'}.get(_desc_inner, 'i8')
            _items = ', '.join(f'{v} as {_elem_rust}' for v in _arr_vals)
            body = f'JArray::from(vec![{_items}])'
            field_meta = _java_field_attr(sf)
            method_blocks.append(f'{field_meta}\n// static field: {sf.name}:{sf.descriptor}\npub fn {safe_fname}() -> {rust_ret} {{\n    {body}\n}}')
        else:
            # 生成 panic stub，确保 getstatic 对应的 ClassName::fieldName() 能编译
            body = f'panic!("stub: {ci.name}.{sf.name}:{sf.descriptor}")'
            field_meta = _java_field_attr(sf)
            method_blocks.append(f'{field_meta}\n// static field: {sf.name}:{sf.descriptor}\npub fn {safe_fname}() -> {rust_ret} {{\n    {body}\n}}')

    used_rust_names: dict[str, int] = {}  # 追踪已用名，防止 mangle 碰撞后重名
    for m in ([] if _is_iface else visible_methods):
        if m.name == '<clinit>':
            # 用户类：翻译 <clinit> 为 class_init() 函数
            if _is_user_class:
                attr_line = _java_method_attr(m)
                try:
                    clinit_body = gen_method_body(
                        m, ci, registry=registry,
                        class_type_params=class_type_params,
                        overloaded_names=overloaded_names,
                        rust_name='class_init',
                    )
                    method_blocks.append(attr_line + '\n' + clinit_body)
                except Exception:
                    pass  # 翻译失败则跳过，class_init 不存在也不影响编译
            continue
        # 确定最终 Rust 方法名（有重载则加描述符后缀）
        rust_name = mangle_name(m.name, m.descriptor) if m.name in overloaded_names else m.name
        # 构造器统一用 new / new_suffix
        if m.is_constructor:
            if '<init>' in overloaded_names:
                rust_name = mangle_name('new', m.descriptor)
            else:
                rust_name = 'new'
        # 碰撞去重：若 mangle 后仍重名，追加数字后缀
        if rust_name in used_rust_names:
            used_rust_names[rust_name] += 1
            rust_name = f'{rust_name}_{used_rust_names[rust_name]}'
        else:
            used_rust_names[rust_name] = 0

        # 若 new_format_map 覆盖了此方法，跳过（_impl 模块已提供实现）
        _nf_covered = (_nf_entry or {}).get('methods', set())
        fn_name_check = safe_ident(rust_name or m.name)
        if fn_name_check in _nf_covered:
            continue

        # 计算虚方法归属（vtable 架构）
        m.virtual_in = _find_virtual_in(m, ci, registry)

        attr_line = _java_method_attr(m)
        # 判断该方法是否需要翻译字节码：
        #   1. native / abstract → 永远生成 stub（panic!）
        #   2. call_chain 不为空 且 此方法不在调用链上 → panic!("stub: ...")
        #   3. stub_bodies=True（兜底/fallback）→ stub
        #   4. 其他 → 翻译字节码
        in_call_chain = (
            call_chain is None or
            (ci.name, m.name, m.descriptor) in call_chain
        )
        if m.is_native or m.is_abstract:
            stub = _gen_native_stub(m, ci, rust_name=rust_name, registry=registry, class_type_params=class_type_params)
            method_blocks.append(attr_line + '\n' + stub)
        elif not in_call_chain or stub_bodies:
            stub = _gen_native_stub(m, ci, rust_name=rust_name, registry=registry, class_type_params=class_type_params)
            method_blocks.append(attr_line + '\n' + stub)
        else:
            try:
                body = gen_method_body(
                    m, ci, registry=registry,
                    class_type_params=class_type_params,
                    overloaded_names=overloaded_names,
                    rust_name=rust_name,
                    in_vtable_body=bool(m.virtual_in),
                )
                # 用户类 main()：若有 <clinit>，在方法体开头插入 class_init() 调用
                if (_is_user_class and _has_clinit
                        and m.name == 'main'
                        and m.descriptor == '([Ljava/lang/String;)V'):
                    body = body.replace(
                        'pub fn main() -> Result<()> {\n',
                        'pub fn main() -> Result<()> {\n    Self::class_init()?;\n',
                        1,
                    )
                method_blocks.append(attr_line + '\n' + body)
            except Exception as e:
                # 翻译失败：退化为 stub，避免生成无效 Rust
                import os as _os
                if _os.environ.get('JAVA_RTA_DEBUG'):
                    import traceback as _tb
                    print(f"[DEBUG] stub fallback for {ci.name}.{m.name}{m.descriptor}: {e}", file=__import__('sys').stderr)
                    _tb.print_exc()
                stub = _gen_native_stub(m, ci, rust_name=rust_name, registry=registry, class_type_params=class_type_params)
                method_blocks.append(attr_line + '\n' + stub)

    # 接口 default 方法继承：当类实现接口但未覆盖其 default 方法时，自动生成继承实现
    if ci.interfaces and registry and not ci.is_interface:
        import copy as _copy
        existing_sigs: set[tuple] = {(m.name, m.descriptor) for m in visible_methods}
        # 参数签名集合（忽略返回类型），用于检测协变返回覆盖（如 LinkedList.reversed() 覆盖 Deque.reversed()）
        def _param_part(desc: str) -> str:
            idx = desc.find(')')
            return desc[:idx + 1] if idx >= 0 else desc
        existing_param_sigs: set[tuple] = {(m.name, _param_part(m.descriptor)) for m in visible_methods}
        # 已用的 Rust 方法名（用于检测 default 方法与类自身方法重名）
        used_rust_names: set[str] = {
            (mangle_name(m.name, m.descriptor) if m.name in overloaded_names else m.name)
            for m in visible_methods if m.name not in ('<init>', '<clinit>')
        }
        # 预扫描：统计所有待继承 default 方法的名字（用于 default 方法之间互相冲突判断）
        default_name_counts: dict[str, int] = {}
        _pre_iface_queue = list(ci.interfaces)
        _pre_visited: set[str] = set()
        while _pre_iface_queue:
            _iname = _pre_iface_queue.pop(0)
            if _iname in _pre_visited:
                continue
            _pre_visited.add(_iname)
            _ici = registry.get(_iname)
            if _ici is None:
                continue
            if _ici.interfaces:
                _pre_iface_queue.extend(_ici.interfaces)
            for _dm in _ici.methods:
                if not _dm.is_abstract and not _dm.is_static and not _dm.is_synthetic and _dm.name not in ('<init>', '<clinit>'):
                    _dm_pp = _param_part(_dm.descriptor)
                    if (_dm.name, _dm.descriptor) not in existing_sigs and (_dm.name, _dm_pp) not in existing_param_sigs:
                        default_name_counts[_dm.name] = default_name_counts.get(_dm.name, 0) + 1
        iface_queue: list[str] = list(ci.interfaces)
        visited_ifaces: set[str] = set()
        while iface_queue:
            iface_name = iface_queue.pop(0)
            if iface_name in visited_ifaces:
                continue
            visited_ifaces.add(iface_name)
            iface_ci = registry.get(iface_name)
            if iface_ci is None:
                continue
            if iface_ci.interfaces:
                iface_queue.extend(iface_ci.interfaces)
            for dm in iface_ci.methods:
                if dm.is_abstract or dm.is_static or dm.is_synthetic or dm.name in ('<init>', '<clinit>'):
                    continue
                if (dm.name, dm.descriptor) in existing_sigs:
                    continue
                # 协变返回覆盖：如果类已有同名同参方法（返回类型不同），也跳过注入
                _dm_pp = _param_part(dm.descriptor)
                if (dm.name, _dm_pp) in existing_param_sigs:
                    continue
                existing_sigs.add((dm.name, dm.descriptor))
                existing_param_sigs.add((dm.name, _dm_pp))
                # 若名字与类自身方法冲突，或 default 方法之间有重名，则 mangle
                needs_mangle = (dm.name in overloaded_names or
                                dm.name in used_rust_names or
                                default_name_counts.get(dm.name, 0) > 1)
                dm_rust = mangle_name(dm.name, dm.descriptor) if needs_mangle else dm.name
                used_rust_names.add(dm_rust)
                # 将 class_name 替换为实现类，使 gen_method_body 生成正确的 this 类型
                dm_adapted = _copy.copy(dm)
                dm_adapted.class_name = ci.name
                # 接口 default 方法注入实现类时：virtual_in 改为实现类名（VirtualDefine）
                # 原始 virtual_in 是接口名（如 Drawable），在宏中会生成不存在的 Drawable__VTable impl
                dm_adapted.virtual_in = _bin_to_rust(ci.name)
                dm_attr = _java_method_attr(dm_adapted)
                # 仅在调用链上时翻译字节码，否则生成 stub（避免复杂 JDK default 方法引入编译错误）
                dm_in_cc = (
                    call_chain is None or
                    (ci.name, dm.name, dm.descriptor) in call_chain or
                    (iface_name, dm.name, dm.descriptor) in call_chain
                )
                if dm_in_cc and not stub_bodies:
                    try:
                        dm_body = gen_method_body(
                            dm_adapted, ci, registry=registry,
                            class_type_params=class_type_params,
                            overloaded_names=overloaded_names,
                            rust_name=dm_rust,
                            in_vtable_body=True,
                        )
                        method_blocks.append(dm_attr + '\n' + dm_body)
                    except Exception:
                        dm_stub = _gen_native_stub(dm_adapted, ci, rust_name=dm_rust, registry=registry, class_type_params=class_type_params)
                        method_blocks.append(dm_attr + '\n' + dm_stub)
                else:
                    dm_stub = _gen_native_stub(dm_adapted, ci, rust_name=dm_rust, registry=registry, class_type_params=class_type_params)
                    method_blocks.append(dm_attr + '\n' + dm_stub)

    # 超类虚方法继承：若子类未覆盖祖先虚方法，生成继承实现使 vtable impl 包含实际函数体。
    # 这避免子类的 Ancestor__VTable impl 退化为 panic!("stub")。
    # 仅对用户类超类链（无 '/'）处理，JDK 类的继承由 BFS 方法级调用链保证。
    if not ci.is_interface and registry and ci.super_class and '/' not in ci.super_class:
        import copy as _copy3
        _vinh_existing: set[tuple] = {(m.name, m.descriptor) for m in visible_methods}
        _vinh_super = ci.super_class
        while _vinh_super and _vinh_super != 'java/lang/Object' and '/' not in _vinh_super:
            _vinh_sci = registry.get(_vinh_super)
            if _vinh_sci is None:
                break
            for _vm in _vinh_sci.methods:
                if (_vm.name, _vm.descriptor) in _vinh_existing:
                    continue
                if _vm.is_static or _vm.is_constructor or _vm.name in ('<init>', '<clinit>'):
                    continue
                _vm_virt_in = _find_virtual_in(_vm, _vinh_sci, registry)
                if not _vm_virt_in:
                    continue  # 非虚方法，不继承
                _vinh_existing.add((_vm.name, _vm.descriptor))
                _vm2 = _copy3.copy(_vm)
                _vm2.class_name = ci.name
                _vm2.virtual_in = _vm_virt_in
                _vm_attr = _java_method_attr(_vm2)
                _vm_in_cc = (
                    call_chain is None or
                    (ci.name, _vm.name, _vm.descriptor) in call_chain or
                    (_vinh_super, _vm.name, _vm.descriptor) in call_chain
                )
                if _vm.is_native or _vm.is_abstract or not _vm_in_cc or stub_bodies:
                    _vm_stub = _gen_native_stub(_vm2, ci, registry=registry, class_type_params=class_type_params)
                    method_blocks.append(_vm_attr + '\n' + _vm_stub)
                else:
                    try:
                        _vm_body = gen_method_body(
                            _vm2, ci, registry=registry,
                            class_type_params=class_type_params,
                            overloaded_names=overloaded_names,
                            in_vtable_body=True,
                        )
                        method_blocks.append(_vm_attr + '\n' + _vm_body)
                    except Exception:
                        _vm_stub = _gen_native_stub(_vm2, ci, registry=registry, class_type_params=class_type_params)
                        method_blocks.append(_vm_attr + '\n' + _vm_stub)
            _vinh_super = _vinh_sci.super_class if _vinh_sci.super_class else None

    # Record 类（super_class == java/lang/Record）：覆盖 invokedynamic 无法翻译的方法
    if ci.super_class == 'java/lang/Record' and not ci.is_interface:
        record_fields = [f for f in ci.fields if not f.is_static]
        simple_name = ci.name.split('$')[-1].split('/')[-1]
        fmt_parts = [f'{f.name}={{}}' for f in record_fields]
        fmt_str = f'{simple_name}[{", ".join(fmt_parts)}]'
        fmt_args = ', '.join(f'self.{safe_ident(f.name)}.get()' for f in record_fields)
        new_blocks = []
        for block in method_blocks:
            if '/* TODO: invokedynamic' in block and any(f'pub fn {n}(' in block for n in ('toString', 'hashCode', 'equals')):
                if 'pub fn toString(' in block:
                    attr = block[:block.index('pub fn toString(')]
                    fmtcall = (f'String::from(format!("{fmt_str}", {fmt_args}).as_str())' if fmt_args
                               else f'String::from("{fmt_str}")')
                    new_blocks.append(attr +
                        f'pub fn toString(&self) -> Result<String> {{\n        Ok({fmtcall})\n    }}')
                elif 'pub fn hashCode(' in block:
                    attr = block[:block.index('pub fn hashCode(')]
                    new_blocks.append(attr +
                        f'pub fn hashCode(&self) -> Result<i32> {{\n        Ok(0)\n    }}')
                elif 'pub fn equals(' in block:
                    attr = block[:block.index('pub fn equals(')]
                    field_cmps = []
                    for f in record_fields:
                        fname = safe_ident(f.name)
                        rust_fty = jvm_to_rust(f.descriptor, registry)
                        if rust_fty == 'String':
                            field_cmps.append(
                                f'self.__get_{fname}().to_string() == other.__get_{fname}().to_string()'
                            )
                        else:
                            field_cmps.append(f'self.__get_{fname}() == other.__get_{fname}()')
                    cmp_expr = ' && '.join(field_cmps) if field_cmps else 'true'
                    new_blocks.append(attr +
                        f'pub fn equals(&self, mut o: Object) -> Result<bool> {{\n'
                        f'        if let Some(other) = o.0.downcast_ref::<Self>() {{\n'
                        f'            Ok({cmp_expr})\n'
                        f'        }} else {{\n'
                        f'            Ok(false)\n'
                        f'        }}\n'
                        f'    }}')
                else:
                    new_blocks.append(block)
            else:
                new_blocks.append(block)
        method_blocks = new_blocks

    # ── 组装 java_class! { ... } 块（方案 §3 核心设计）────────────────────
    # 模块级 static 声明必须留在宏外（宏不接受 struct/impl 之外的项目）。
    if module_statics and not _is_iface:
        parts.append('\n'.join(module_statics))

    if not _full_impl:
        block: list[str] = []
        block.extend(_java_class_block_head(
            ci, registry=registry,
            superclass_rust=parent_rust,
            superclass_fields=superclass_fields,
        ))
        block.append('')
        # struct 声明：裸类型（封装细节收拢进宏），无 derive / 无 _super / 无 _phantom
        if struct_lines:
            block.append(f"pub struct {struct_name}{struct_generic} {{")
            block.extend(struct_lines)
            block.append("}")
        else:
            block.append(f"pub struct {struct_name}{struct_generic};")

        # 接口：宏把 struct 展开为 `pub type Name = Object;`（Arch-1），无 impl 块
        if not _is_iface:
            block.append('')
            impl_body = '\n\n'.join(_indent(b) for b in method_blocks)
            block.append(f"{impl_header} {{")
            block.append(impl_body)
            block.append("}")

        parts.append("java_rta_macros::java_class! {")
        for line in block:
            parts.append(_indent(line) if line else '')
        parts.append("}")
        parts.append('')

    # BINARY_NAME / ObjectVTable / Into<Object> / From<Object> / Debug 全部由
    # java_class! 宏在编译期展开（方案 §11 职责边界总表）。
    # （Object 类走手写路径 java_runtime/，不经过此函数）

    return '\n'.join(parts)
