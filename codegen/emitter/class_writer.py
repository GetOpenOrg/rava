"""
单个 Java 类 → Rust 文件内容生成：_gen_class_rs 主函数。
"""

from collections import Counter
from ..types import ClassInfo, FieldInfo, ParsedMethod
from ..type_map import jvm_to_rust, mangle_name, short_cls, get_ergonomic_jvm_rename
from ..method import gen_method_body, _indent
from ..sig_parser import parse_class_type_params, parse_field_type
from ..constants import safe_ident, RUST_KEYWORDS as _RUST_KEYWORDS
from .attrs import (to_snake, _java_class_attr, _java_field_attr, _java_method_attr)
from .method_gen import _gen_native_stub

_safe_field_name = safe_ident


def _gen_class_rs(ci: ClassInfo, registry: dict | None = None,
                  jdk_crate_pkg_paths: list[str] | None = None,
                  stub_bodies: bool = False,
                  call_chain: set | None = None,
                  new_format_map: dict | None = None,
                  workspace_root: str | None = None,
                  user_crate_prefix: str | None = None,
                  extra_fields: dict | None = None,
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

    parts: list[str] = [
        "#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types)]",
        "use java_runtime::prelude::*;",
        *cross_imports,
        "",
        _java_class_attr(ci, compiled=True),
    ]
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

    cls_extra_fields = (extra_fields or {}).get(ci.name, [])
    if inst_fields or cls_extra_fields or _has_super:
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
        for ef_name, ef_type in cls_extra_fields:
            field_lines.append(f"    pub {ef_name}: {ef_type},")
        # 若有泛型参数但字段中未用到，加 PhantomData 防止 E0392
        if class_type_params:
            phantom_ty = ', '.join(f'std::marker::PhantomData<{p}>' for p in class_type_params)
            if len(class_type_params) > 1:
                phantom_ty = f'std::marker::PhantomData<({", ".join(class_type_params)},)>'
            field_lines.append(f"    pub _phantom: {phantom_ty},")
        decls = '\n'.join(field_lines)
        parts.append(f"#[derive(Clone, Default)]\npub struct {struct_name}{struct_generic} {{\n{decls}\n}}\n")
    else:
        if class_type_params:
            # 无字段但有泛型参数：改用 tuple struct 包含 PhantomData
            if len(class_type_params) == 1:
                phantom_ty = f'std::marker::PhantomData<{class_type_params[0]}>'
            else:
                phantom_ty = f'std::marker::PhantomData<({", ".join(class_type_params)},)>'
            parts.append(f"#[derive(Clone, Default)]\npub struct {struct_name}{struct_generic}({phantom_ty});\n")
        else:
            parts.append(f"#[derive(Clone, Default)]\npub struct {struct_name}{struct_generic};\n")

    # T76：为有父类的类生成显式 upcast 方法（as_xxx / into_xxx）
    # 不使用 Deref（Rust 反模式），改用显式方法，语义清晰
    if _has_super and registry:
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
    if _has_super and registry:
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

    # 若有 new_format_map 覆盖，插入 #[path = "..."] mod _impl; 块
    _nf_entry = (new_format_map or {}).get(ci.name)
    if _nf_entry and workspace_root:
        pkg_depth = len(ci.name.split('/')) - 1
        ups = '../' * (pkg_depth + 2)
        if _nf_entry.get('main'):
            main_rel = _nf_entry['main']
            parts.append(f'#[allow(unused_imports, dead_code, unused_variables, non_snake_case, non_camel_case_types)]\n#[path = "{ups}{main_rel}"]\nmod _impl;\n')

    # 过滤 synthetic 方法（编译器合成桥接方法），再统计重载
    visible_methods = [m for m in ci.methods if not m.is_synthetic]
    name_counts = Counter(m.name for m in visible_methods if m.name != '<clinit>')
    overloaded_names: set[str] = {name for name, count in name_counts.items() if count > 1}

    method_blocks: list[str] = []

    # public static 字段的 getter 方法（用于 getstatic 访问，如 System::out()）
    # 生成静态字段 getter：有 _impl 覆盖的跳过，其余生成 panic stub 或 constant_value
    static_fields = [f for f in ci.fields if f.is_static]
    existing_method_names: set[str] = {m.name for m in visible_methods}
    _nf_covered_sf = (_nf_entry or {}).get('methods', set())
    for sf in static_fields:
        safe_fname = _safe_field_name(sf.name)
        if safe_fname in existing_method_names:
            # 字段名与方法名冲突：改用 _field 后缀，让 getstatic 仍能访问该字段
            safe_fname = safe_fname + '_field'
        # 若 _impl 已覆盖此静态字段访问器，跳过
        if safe_fname in _nf_covered_sf:
            continue
        rust_ret = jvm_to_rust(sf.descriptor, registry=registry)
        if sf.constant_value:
            # ConstantValue attribute：static final 字段有确定字面量，直接返回
            cv = sf.constant_value
            if rust_ret == 'String':
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
        else:
            # 生成 panic stub，确保 getstatic 对应的 ClassName::fieldName() 能编译
            body = f'panic!("stub: {ci.name}.{sf.name}:{sf.descriptor}")'
        field_meta = _java_field_attr(sf)
        method_blocks.append(f'{field_meta}\n// static field: {sf.name}:{sf.descriptor}\npub fn {safe_fname}() -> {rust_ret} {{\n    {body}\n}}')

    used_rust_names: dict[str, int] = {}  # 追踪已用名，防止 mangle 碰撞后重名
    for m in visible_methods:
        if m.name == '<clinit>':
            continue
        # 确定最终 Rust 方法名（有重载则加描述符后缀）
        rust_name = mangle_name(m.name, m.descriptor) if m.name in overloaded_names else m.name
        # T39：若 _ergonomic.rs 有 @jvm_rename 指令，将非重载方法改名（腾出干净名称给 ergonomic 层）
        if not m.is_constructor and m.name not in overloaded_names:
            erg_rename = get_ergonomic_jvm_rename(ci.name, m.name)
            if erg_rename is not None:
                rust_name = erg_rename
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
        if fn_name_check == 'clone':
            fn_name_check = 'jvm_clone'
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
                method_blocks.append(attr_line + '\n' + body)
            except Exception as e:
                # 翻译失败：退化为 stub，避免生成无效 Rust
                stub = _gen_native_stub(m, ci, rust_name=rust_name, registry=registry)
                method_blocks.append(attr_line + '\n' + stub)

    impl_body = '\n\n'.join(_indent(b) for b in method_blocks)
    parts.append(f"{impl_header} {{\n{impl_body}\n}}\n")

    # Into<Object> / From<Object> / Debug 由 #[java_rta_macros::java_class] proc-macro 自动生成
    # （Object 类走手写路径 java_runtime/，不经过此函数）

    return '\n'.join(parts)
