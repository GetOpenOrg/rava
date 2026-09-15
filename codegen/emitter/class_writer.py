"""
单个 Java 类 → Rust 文件内容生成：_gen_class_rs 主函数。
"""

import os
from collections import Counter
from ..types import ClassInfo, FieldInfo, ParsedMethod
from ..type_map import jvm_to_rust, mangle_name, short_cls, rust_default
from ..method import gen_method_body, _indent
from ..sig_parser import parse_class_type_params, parse_field_type
from ..constants import safe_ident, RUST_KEYWORDS as _RUST_KEYWORDS
from .attrs import (to_snake, _java_class_attr, _java_field_attr, _java_method_attr)
from .method_gen import _gen_native_stub

_safe_field_name = safe_ident

# 简单 const_push 指令 → Python 字面量值映射（用于扫描 <clinit>）
_CONST_PUSH_OPCODES: dict[str, object] = {
    'iconst_m1': -1, 'iconst_0': 0, 'iconst_1': 1, 'iconst_2': 2,
    'iconst_3': 3,   'iconst_4': 4, 'iconst_5': 5,
    'lconst_0': 0,   'lconst_1': 1,
    'fconst_0': 0.0, 'fconst_1': 1.0, 'fconst_2': 2.0,
    'dconst_0': 0.0, 'dconst_1': 1.0,
}


def _extract_clinit_consts(ci: ClassInfo) -> dict[str, str]:
    """扫描 <clinit> 中 const_push → putstatic 的简单模式，
    返回 {field_name: constant_value_str}（格式与 ConstantValue attribute 一致）。
    只识别相邻两条指令构成的最简赋值，复杂初始化不处理。
    """
    clinit = next((m for m in ci.methods if m.name == '<clinit>'), None)
    if not clinit or not clinit.instrs:
        return {}

    result: dict[str, str] = {}
    instrs = clinit.instrs
    for i, instr in enumerate(instrs):
        if instr.opcode != 'putstatic':
            continue
        # comment 格式: "Field java/lang/String.COMPACT_STRINGS:Z"
        comment = instr.comment or ''
        if not comment.startswith('Field ') or '.' not in comment:
            continue
        rest = comment[len('Field '):]          # "java/lang/String.COMPACT_STRINGS:Z"
        if '.' not in rest:
            continue
        cls_part, field_desc = rest.split('.', 1)   # "java/lang/String", "COMPACT_STRINGS:Z"
        if cls_part != ci.name or ':' not in field_desc:
            continue
        fname = field_desc.split(':')[0]

        # 前一条指令（或前两条：iconst_0 → anewarray → putstatic）
        if i == 0:
            continue
        prev = instrs[i - 1]
        # 模式：iconst_0 → anewarray → putstatic（static final Object[] = new T[0]）
        if prev.opcode == 'anewarray' and i >= 2 and instrs[i - 2].opcode == 'iconst_0':
            result[fname] = '__EMPTY_ARRAY__'
        elif prev.opcode in _CONST_PUSH_OPCODES:
            val = _CONST_PUSH_OPCODES[prev.opcode]
            result[fname] = str(int(val)) if isinstance(val, float) and val == int(val) else str(val)
        elif prev.opcode in ('bipush', 'sipush') and prev.operand is not None:
            result[fname] = str(prev.operand)
        elif prev.opcode == 'ldc' and prev.comment:
            # ldc comment 可能是 "String ...", "int 42", "float 1.0" 等
            ldc = prev.comment.strip()
            for prefix in ('String ', 'int ', 'long ', 'float ', 'double '):
                if ldc.startswith(prefix):
                    result[fname] = ldc[len(prefix):]
                    break

    return result


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
                  user_sibling_imports: list[str] | None = None) -> str:
    """生成单个 Java 类对应的完整 .rs 文件内容。

    生成规则：
    - 实例字段用 Field<T> 包装（提供 Java 字段语义的内部可变性）
    - 方法直接在 impl 块中，无 raw:: 子模块
    - 所有方法返回 Result<T>
    - 每个 struct / field / method 前加 // @java_* 注释供 build.rs 扫描
    - new_format_map: 若提供，为覆盖的类插入 #[path] mod _impl; 并跳过被覆盖方法
    - user_crate_prefix: 若提供（如 'jdk_classes'），cross_imports 用该 crate 前缀
    """

    # 跨包 glob import：让生成代码能直接用 Objects、Integer 等翻译过的 JDK 类型
    cross_imports: list[str] = []
    if jdk_crate_pkg_paths:
        prefix = user_crate_prefix or 'crate'
        for pkg_path in jdk_crate_pkg_paths:
            cross_imports.append(f"use {prefix}::{pkg_path}::*;")

    # 若文件所在包未包含在全局 glob 导入中（如 jdk/ 前缀），则补充自身包的 glob 导入
    # 以确保同包兄弟类型（如内部接口）可直接引用
    if ci.name and '/' in ci.name:
        _own_pkg_parts = ci.name.split('/')[:-1]
        _own_pkg_path = '::'.join(
            f'r#{p}' if p in _RUST_KEYWORDS else p for p in _own_pkg_parts
        )
        _existing_pkgs = set(jdk_crate_pkg_paths) if jdk_crate_pkg_paths else set()
        if _own_pkg_path not in _existing_pkgs:
            _pfx = user_crate_prefix or 'crate'
            cross_imports.append(f"use {_pfx}::{_own_pkg_path}::*;")

    # 以下两个机制（冲突消歧 + jdk/ 显式导入）共享同一次指令扫描
    _need_refs = bool(conflict_map or skipped_classes)
    _referenced: set[str] = set()
    if _need_refs:
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
        # 扫描字段描述符（含超类链继承字段）中引用的类型（含 jdk/ 等跨包引用）
        import re as _re
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
        for _f in _all_fields_to_scan:
            for _m in _re.finditer(r'L([^;]+);', _f.descriptor or ''):
                _referenced.add(_m.group(1))
            for _m in _re.finditer(r'L([^;]+);', _f.generic_signature or ''):
                _referenced.add(_m.group(1))
        # 扫描方法描述符（参数和返回值）中引用的类型
        for _method in ci.methods:
            for _m in _re.finditer(r'L([^;]+);', _method.descriptor or ''):
                _referenced.add(_m.group(1))
            for _m in _re.finditer(r'L([^;]+);', getattr(_method, 'generic_signature', '') or ''):
                _referenced.add(_m.group(1))

    # 消歧：当同一简单名存在于多个包中时，追加显式 use 覆盖 glob 歧义
    if conflict_map:
        _own_pkg = '/'.join(ci.name.split('/')[:-1]) if '/' in ci.name else ''
        _prefix = user_crate_prefix or 'crate'

        def _pkg_to_use(pkg_slash: str) -> str:
            return _prefix + '::' + '::'.join(
                f'r#{p}' if p in _RUST_KEYWORDS else p
                for p in pkg_slash.split('/')
            )

        # 本文件自身定义的类型简名（跳过：不能 use 自己定义的名字）
        # Java 内部类用 $ 分隔，Rust struct 名用 _ 替代
        _self_simple = (ci.name.split('/')[-1] if '/' in ci.name else ci.name).replace('$', '_')

        for _sn, _pkgs in conflict_map.items():
            # 跳过：本文件就是该类型的定义文件（struct 名会与 use 重复导致 E0255）
            if _sn.replace('$', '_') == _self_simple:
                continue
            # 优先：指令中有直接引用的包
            _matches = [p for p in _pkgs if f'{p}/{_sn}' in _referenced]
            if len(_matches) == 1:
                _chosen = _matches[0]
            elif _own_pkg in _pkgs:
                # 次优：文件自身所在包（引用了与本包同名的类型，用本包版本）
                _chosen = _own_pkg
            elif _matches:
                _chosen = _matches[0]
            else:
                # 回退：优先 java/ 包，否则取第一个
                _java = [p for p in _pkgs if p.startswith('java/')]
                _chosen = _java[0] if _java else _pkgs[0]
            # Java 内部类 $ → Rust struct 名用 _
            cross_imports.append(f"use {_pkg_to_use(_chosen)}::{_sn.replace('$', '_')};")

    # 为被跳过全局导入的包（如 jdk/）中的类型，按需添加逐文件显式导入
    if skipped_classes and _referenced:
        _prefix2 = user_crate_prefix or 'crate'
        _self_simple2 = (ci.name.split('/')[-1] if '/' in ci.name else ci.name).replace('$', '_')
        _added_skipped: set[str] = set()
        for _full_cls in sorted(_referenced):
            _cls_parts = _full_cls.split('/')
            if len(_cls_parts) < 2:
                continue
            _rust_pkg = '::'.join(
                f'r#{p}' if p in _RUST_KEYWORDS else p for p in _cls_parts[:-1]
            )
            # Java 内部类 $ → Rust struct 名用 _
            _simple = _cls_parts[-1].replace('$', '_')
            if f"{_rust_pkg}::{_simple}" in skipped_classes and _simple != _self_simple2 and _simple not in _added_skipped:
                cross_imports.append(f"use {_prefix2}::{_rust_pkg}::{_simple};")
                _added_skipped.add(_simple)

    # 用户内部类兄弟模块导入（crate::mod_name::TypeName）
    if user_sibling_imports:
        cross_imports.extend(user_sibling_imports)

    # 全量手写类（native_impl 文件含 pub struct）：codegen 跳过 struct 生成，改输出 pub use _impl::*
    _full_impl = ci.name in (full_impl_classes or set())

    parts: list[str] = [
        "#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]",
        f"use {user_crate_prefix or 'crate'}::prelude::*;",
        *cross_imports,
        "",
    ]
    if not _full_impl:
        parts.append(_java_class_attr(ci, compiled=True))
    field_type_prefix = "JField"

    inst_fields = [f for f in ci.fields if not f.is_static]

    # T76：_super 嵌套字段替代字段展平
    # 若有父类（且不是 Object），在实例字段前插入 _super: ParentType
    # proc-macro 读 super_class 注解，自动生成 From<Self> for Parent
    # emitter 在 struct 定义后额外生成显式 upcast 方法（as_xxx / into_xxx）
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
        # 泛型参数需要 Clone + Default + 'static；Default 是必须的，因为 _super: Default::default() 要求父链所有类型参数实现 Default
        bounds_str = ', '.join(f"{p}: Clone + Default + 'static" for p in class_type_params)
        struct_generic = f"<{bounds_str}>"
        impl_header   = f"impl<{bounds_str}> {struct_name}<{type_params_str}>"
    else:
        struct_generic = ''
        impl_header   = f"impl {struct_name}"

    if not _full_impl and (inst_fields or _has_super):
        field_lines = []
        # _super 字段：嵌入直接父类（T76，替代字段展平）
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
            field_lines.append(f"    pub _super: {parent_rust},")
        for f in inst_fields:
            safe_fname = _safe_field_name(f.name)
            field_lines.append("    " + _java_field_attr(f))
            # 优先用字段级 generic_signature（如 TE; → E），回退到裸描述符
            # 若 generic_signature 解析结果是 Object（简化），用描述符推断更精确的类型
            gen_rust = (parse_field_type(f.generic_signature, class_type_params)
                        if f.generic_signature else '')
            desc_rust = jvm_to_rust(f.descriptor, registry)
            field_rust = gen_rust if (gen_rust and gen_rust != 'Object') else desc_rust
            field_lines.append(f"    pub {safe_fname}: {field_type_prefix}<{field_rust}>,")
        # 若有泛型参数但字段中未用到，加 PhantomData 防止 E0392
        if class_type_params:
            phantom_ty = ', '.join(f'std::marker::PhantomData<{p}>' for p in class_type_params)
            if len(class_type_params) > 1:
                phantom_ty = f'std::marker::PhantomData<({", ".join(class_type_params)},)>'
            field_lines.append(f"    pub _phantom: {phantom_ty},")
        decls = '\n'.join(field_lines)
        parts.append(f"#[derive(Clone, Default, PartialEq)]\npub struct {struct_name}{struct_generic} {{\n{decls}\n}}\n")
    elif not _full_impl:
        if class_type_params:
            # 无字段但有泛型参数：改用 tuple struct 包含 PhantomData
            if len(class_type_params) == 1:
                phantom_ty = f'std::marker::PhantomData<{class_type_params[0]}>'
            else:
                phantom_ty = f'std::marker::PhantomData<({", ".join(class_type_params)},)>'
            parts.append(f"#[derive(Clone, Default, PartialEq)]\npub struct {struct_name}{struct_generic}({phantom_ty});\n")
        else:
            parts.append(f"#[derive(Clone, Default, PartialEq)]\npub struct {struct_name}{struct_generic};\n")

    # T76：为有父类的类生成显式 upcast 方法（as_xxx / into_xxx）
    # 不使用 Deref（Rust 反模式），改用显式方法，语义清晰
    # 全量手写类的 upcast 由手写文件自行提供，codegen 跳过
    if _has_super and registry and not _full_impl:
        upcast_lines = [f'{impl_header} {{']
        access_path = '_super'
        cur_super = ci.super_class
        # upcast 方法只使用子类自身的类型参数（child_tparams 在 impl block 中始终可用）
        # 不传播祖先的参数名（祖先参数名在子类 impl block 中不可见）
        child_tparams: list[str] = list(class_type_params)
        while cur_super and cur_super != 'java/lang/Object':
            parent_rust_name = short_cls(cur_super)
            snake = to_snake(parent_rust_name.replace('$', '_'))
            # 计算该祖先级的完整类型表达式（含泛型参数），只用子类自身参数名
            parent_full_type = parent_rust_name
            ancestor_ci = registry.get(cur_super)
            if ancestor_ci:
                ancestor_params = parse_class_type_params(ancestor_ci.generic_signature) if ancestor_ci.generic_signature else []
                if ancestor_params:
                    if child_tparams:
                        args = child_tparams[:len(ancestor_params)]
                        while len(args) < len(ancestor_params):
                            args.append('Object')
                    else:
                        args = ['Object'] * len(ancestor_params)
                    parent_full_type += '<' + ', '.join(args) + '>'
            upcast_lines.append(f'    pub fn as_{snake}(&self) -> &{parent_full_type} {{ &self.{access_path} }}')
            upcast_lines.append(f'    pub fn into_{snake}(self) -> {parent_full_type} {{ self.{access_path} }}')
            cur_super = ancestor_ci.super_class if ancestor_ci else None
            access_path += '._super'
        upcast_lines.append('}')
        parts.append('\n'.join(upcast_lines) + '\n')

    # T55：为每个祖先生成 From<Self> for Ancestor（含直接父类 + 所有祖先链）
    # 这让 child.into() 在期望父类类型的位置自动工作，避免 E0308
    # 全量手写类的 From impl 由手写文件自行提供，codegen 跳过
    if _has_super and registry and not _full_impl:
        child_full = struct_name
        if class_type_params:
            child_full += '<' + ', '.join(class_type_params) + '>'
        impl_generics_for_from = f"<{bounds_str}>" if class_type_params else ''

        access_path = '_super'
        cur_super = ci.super_class
        child_tparams_t55: list[str] = list(class_type_params)
        while cur_super and cur_super != 'java/lang/Object':
            parent_rust_name = short_cls(cur_super)
            parent_full_type = parent_rust_name
            ancestor_ci = registry.get(cur_super)
            if ancestor_ci:
                ancestor_params = parse_class_type_params(ancestor_ci.generic_signature) if ancestor_ci.generic_signature else []
                if ancestor_params:
                    if child_tparams_t55:
                        args = child_tparams_t55[:len(ancestor_params)]
                        while len(args) < len(ancestor_params):
                            args.append('Object')
                    else:
                        args = ['Object'] * len(ancestor_params)
                    parent_full_type += '<' + ', '.join(args) + '>'
            parts.append(
                f"impl{impl_generics_for_from} From<{child_full}> for {parent_full_type} {{\n"
                f"    fn from(v: {child_full}) -> {parent_full_type} {{ v.{access_path} }}\n"
                f"}}\n"
            )
            cur_super = ancestor_ci.super_class if ancestor_ci else None
            access_path += '._super'

    # T55b：为实现的接口生成 From<Self> for Interface
    # 接口无实例字段，用 Default::default() 创建空接口实例（允许 .into() 类型转换编译通过）
    # 包括所有祖先类实现的接口（传递接口）
    if registry and not ci.is_interface:
        child_full = struct_name
        if class_type_params:
            child_full += '<' + ', '.join(class_type_params) + '>'
        impl_generics_for_from = f"<{bounds_str}>" if class_type_params else ''
        # 收集直接接口 + 所有祖先类的接口
        iface_q: list[str] = list(ci.interfaces or [])
        _anc = ci.super_class
        while _anc and _anc != 'java/lang/Object':
            _anc_ci = registry.get(_anc)
            if _anc_ci is None:
                break
            iface_q.extend(_anc_ci.interfaces or [])
            _anc = _anc_ci.super_class
        visited_ifaces_from: set[str] = set()
        while iface_q:
            iface_bin = iface_q.pop(0)
            if iface_bin in visited_ifaces_from:
                continue
            visited_ifaces_from.add(iface_bin)
            iface_ci = registry.get(iface_bin)
            if iface_ci is None:
                continue
            if iface_ci.interfaces:
                iface_q.extend(iface_ci.interfaces)
            iface_rust_name = short_cls(iface_bin)
            iface_params = parse_class_type_params(iface_ci.generic_signature) if iface_ci.generic_signature else []
            if iface_params:
                if class_type_params:
                    args = class_type_params[:len(iface_params)]
                    while len(args) < len(iface_params):
                        args.append('Object')
                else:
                    args = ['Object'] * len(iface_params)
                iface_full_type = f"{iface_rust_name}<{', '.join(args)}>"
            else:
                iface_full_type = iface_rust_name
            parts.append(
                f"impl{impl_generics_for_from} From<{child_full}> for {iface_full_type} {{\n"
                f"    fn from(v: {child_full}) -> {iface_full_type} {{ Default::default() }}\n"
                f"}}\n"
            )

    # K-2: 共置 _impl.rs 文件由 project_writer 在生成阶段复制；class_writer 不再生成 #[path] 块。
    # 占位：保留变量引用以防后续代码使用，实际不生成任何内容。
    _nf_entry = (new_format_map or {}).get(ci.name)
    if False:  # K-2: 已由 project_writer 的共置机制替代
        pass

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
    # JVM 原始类型描述符集合，可安全包装在 OnceLock<Mutex<T>> 中（均实现 Send + Copy）
    # String（Java 自定义类型）不在此集合，因其含 Rc 字段，不实现 Send
    _MUTABLE_STATIC_DESCS = frozenset({'I', 'J', 'F', 'D', 'Z', 'B', 'C', 'S'})
    for sf in static_fields:
        safe_fname = _safe_field_name(sf.name)
        if safe_fname in existing_method_names:
            # 字段名与方法名冲突：改用 _field 后缀，让 getstatic 仍能访问该字段
            safe_fname = safe_fname + '_field'
        # 若 _impl 已覆盖此静态字段访问器，跳过
        if safe_fname in _nf_covered_sf:
            continue
        rust_ret = jvm_to_rust(sf.descriptor, registry=registry)
        # ConstantValue attribute 优先；其次尝试从 <clinit> 提取简单常量
        cv = sf.constant_value or _clinit_consts.get(sf.name, '')
        if cv:
            if cv == '__EMPTY_ARRAY__':
                # iconst_0 → anewarray → putstatic：static final T[] = new T[0]
                body = 'Rc::new(RefCell::new(Vec::new()))'
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
        else:
            # 生成 panic stub，确保 getstatic 对应的 ClassName::fieldName() 能编译
            body = f'panic!("stub: {ci.name}.{sf.name}:{sf.descriptor}")'
            field_meta = _java_field_attr(sf)
            method_blocks.append(f'{field_meta}\n// static field: {sf.name}:{sf.descriptor}\npub fn {safe_fname}() -> {rust_ret} {{\n    {body}\n}}')

    used_rust_names: dict[str, int] = {}  # 追踪已用名，防止 mangle 碰撞后重名
    for m in visible_methods:
        if m.name == '<clinit>':
            # 用户类：翻译 <clinit> 为 class_init() 函数
            if _is_user_class:
                attr_line = _java_method_attr(m, compiled=True)
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

        attr_line = _java_method_attr(m, compiled=True)
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
            stub = _gen_native_stub(m, ci, rust_name=rust_name, registry=registry)
            method_blocks.append(attr_line + '\n' + stub)
        elif not in_call_chain or stub_bodies:
            stub = _gen_native_stub(m, ci, rust_name=rust_name, registry=registry)
            method_blocks.append(attr_line + '\n' + stub)
        else:
            try:
                body = gen_method_body(
                    m, ci, registry=registry,
                    class_type_params=class_type_params,
                    overloaded_names=overloaded_names,
                    rust_name=rust_name,
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
                stub = _gen_native_stub(m, ci, rust_name=rust_name, registry=registry)
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
                dm_attr = _java_method_attr(dm, compiled=True)
                # 将 class_name 替换为实现类，使 gen_method_body 生成正确的 this 类型
                dm_adapted = _copy.copy(dm)
                dm_adapted.class_name = ci.name
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
                        )
                        method_blocks.append(dm_attr + '\n' + dm_body)
                    except Exception:
                        dm_stub = _gen_native_stub(dm_adapted, ci, rust_name=dm_rust, registry=registry)
                        method_blocks.append(dm_attr + '\n' + dm_stub)
                else:
                    dm_stub = _gen_native_stub(dm_adapted, ci, rust_name=dm_rust, registry=registry)
                    method_blocks.append(dm_attr + '\n' + dm_stub)

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
                                f'self.{fname}.get().to_string() == other.{fname}.get().to_string()'
                            )
                        else:
                            field_cmps.append(f'self.{fname}.get() == other.{fname}.get()')
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

    # 插入模块级 static 声明（OnceLock 等），放在 impl 块之前
    if module_statics:
        parts.append('\n'.join(module_statics))

    impl_body = '\n\n'.join(_indent(b) for b in method_blocks)
    parts.append(f"{impl_header} {{\n{impl_body}\n}}\n")

    # Into<Object> / From<Object> / Debug 由 #[java_rta_macros::java_class] proc-macro 自动生成
    # （Object 类走手写路径 java_runtime/，不经过此函数）

    return '\n'.join(parts)
