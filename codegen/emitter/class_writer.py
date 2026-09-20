"""
单个 Java 类 → Rust 文件内容生成：_gen_class_rs 主函数。
"""

from ..type_map import short_cls as _short_cls_g
import os
import re as _re
from ..types import ClassInfo, FieldInfo, ParsedMethod
from ..type_map import jvm_to_rust, mangle_name, short_cls, rust_default, _PRIMITIVE_MAP as _JVM_PRIMITIVE_MAP
from ..method import gen_method_body, _indent
from ..cfg import CfgAuditError, STATS as _CFG_STATS
from ..type_map import parse_class_type_params, parse_field_type, hierarchy_overloaded_names, method_name_is_mangled, instance_field_rust_name
from ..type_map import (effective_class_type_params, ancestor_type_args, outer_ref_field_type,
                        rust_type_with_args as _rust_type_with_args)
from ..constants import (safe_ident, OBJECT_CLASS as _OBJECT_CLASS,
                         PRIMITIVE_RUST_TYPES as _PRIMITIVE_RUST_TYPES, STRING_CLASS)

from .attrs import _java_class_block_head, _java_field_attr, _java_method_attr
from .method_gen import _gen_native_stub
from .vtable_util import _bin_to_rust, _find_virtual_in
from .import_gen import (collect_referenced, gen_cross_imports,
                         scan_used_vtable_imports)
from .inherited_gen import (ClassEmission, IMPORTS_SLOT as _INHERITED_IMPORTS_SLOT,
                            MEMBERS_SLOT as _INHERITED_MEMBERS_SLOT)
from .interface_gen import IMPLS_SLOT as _INTERFACE_IMPLS_SLOT, UPCASTS_SLOT as _INTERFACE_UPCASTS_SLOT
from ..type_map import interface_signature_views as _interface_signature_views
from ..instr.coerce import _parse_field_ref
from ..instr.coerce import lambda_impl_rust_name, LAMBDA_NAME_LEDGER

_safe_field_name = safe_ident

# <clinit> 翻译函数在 java_class! 块内的固定名字（与宏 block/class_init.rs 的约定一致）
_CLINIT_FN = '__clinit'


_ACC_FINAL   = 0x0010
_ACC_STATIC  = 0x0008


def _adapt_interface_method(method, ci, iface_bin: str, views: dict):
    """接口方法体展开到实现类 ci：所属类换成 ci，泛型签名（方法 / 局部变量）里的接口类型变量
    换成 ci 视角下的类型实参。"""
    import copy as _copy_adapt
    from ..type_map import substitute_signature_type_vars as _subst
    adapted = _copy_adapt.copy(method)
    adapted.class_name = ci.name
    view = views.get(iface_bin)
    if view:
        adapted.generic_signature = _subst(method.generic_signature, view)
        if getattr(method, 'local_types', None):
            adapted.local_types = {slot: (_subst(sig, view), start)
                                   for slot, (sig, start) in method.local_types.items()}
        if getattr(method, 'local_vars', None):
            adapted.local_vars = [tuple(entry[:5]) + (_subst(entry[5], view),)
                                  for entry in method.local_vars]
    return adapted


def _override_vtable_erasure(m, ci, registry) -> list[str]:
    """覆盖方法在「声明祖先 vtable」上的擦除位置：接收者签名里对应位置的类型串。

    声明祖先按自身类型形参声明的参数 / 返回位置，在祖先 vtable（Object 化签名）上
    已是 Object；继承者（本类）的覆盖条目签名需按名单同步擦除。位置判定以
    「同名同描述符方法在祖先声明中的泛型签名」提及祖先形参为准。"""
    if not registry:
        return []
    from ..type_map import parse_method_param_types
    # 声明祖先：沿超类链找 virtual_in 对应的类
    owner_bin = None
    cur = ci.super_class
    seen: set[str] = set()
    while cur and cur not in seen and cur in registry:
        seen.add(cur)
        if short_cls(cur) == m.virtual_in:
            owner_bin = cur
            break
        cur = registry[cur].super_class
    if owner_bin is None:
        return []
    owner_ci = registry[owner_bin]
    owner_params = effective_class_type_params(owner_ci, registry)
    if not owner_params:
        return []
    owner_m = next((x for x in (owner_ci.methods or [])
                    if x.name == m.name and x.descriptor == m.descriptor
                    and not x.is_static), None)
    if owner_m is None:
        return []
    own_types, own_ret = parse_method_param_types(
        m.generic_signature, effective_class_type_params(ci, registry),
        registry, is_static=False)
    own_types, own_ret = list(own_types or []), own_ret
    anc_types, anc_ret = parse_method_param_types(
        owner_m.generic_signature, owner_params, registry, is_static=False)
    anc_types, anc_ret = list(anc_types or []), anc_ret
    out: list[str] = []
    for i, anc_ty in enumerate(anc_types):
        if any(_re.search(r'\b' + _re.escape(_p) + r'\b', anc_ty) for _p in owner_params):
            if i < len(own_types) and own_types[i] and own_types[i] != 'Object':
                out.append(own_types[i])
    def _result_inner(ty: str):
        mm = _re.match(r'^Result<(.*)>$', ty or '')
        return mm.group(1) if mm else None
    anc_inner = _result_inner(anc_ret)
    if anc_inner and any(
            _re.search(r'\b' + _re.escape(_p) + r'\b', anc_inner) for _p in owner_params):
        own_inner = _result_inner(own_ret)
        if own_inner and own_inner != 'Object':
            out.append(own_inner)
    elif anc_ret and anc_ret != 'Result<()>' and any(
            _re.search(r'\b' + _re.escape(_p) + r'\b', anc_ret) for _p in owner_params):
        if own_ret and own_ret != 'Object':
            out.append(own_ret)
    return list(dict.fromkeys(out))


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
                  generated_classes: set | None = None,
                  emission: 'ClassEmission | None' = None) -> str:
    """生成单个 Java 类对应的完整 .rs 文件内容。

    生成规则：
    - 实例字段用 Field<T> 包装（提供 Java 字段语义的内部可变性）
    - 方法直接在 impl 块中，无 raw:: 子模块
    - 所有方法返回 Result<T>
    - 每个 struct / field / method 前加 // @java_* 注释供 build.rs 扫描
    - new_format_map: 若提供，为覆盖的类插入 #[path] mod _impl; 并跳过被覆盖方法
    - user_crate_prefix: 若提供（如 'jdk_classes'），cross_imports 用该 crate 前缀
    - emission: 若提供，记录本类实际生成的方法声明，并在文本中留出继承成员声明的
      两个插入位（use 区 / impl 块尾），由 inherited_gen.resolve_inherited_members 统一填充
    """

    # ── Step 1+2: 引用收集与精确 cross_imports（import_gen）─────────────────
    _referenced = collect_referenced(ci, registry, generated_classes)
    cross_imports = gen_cross_imports(
        ci, registry, jdk_crate_pkg_paths, call_chain, generated_classes,
        conflict_map, skipped_classes, user_sibling_imports,
        user_crate_prefix, _referenced)

    # 全量手写类（native_impl 文件含 pub struct）：codegen 跳过 struct 生成，改输出 pub use _impl::*
    _full_impl = ci.name in (full_impl_classes or set())
    if not _full_impl:
        # G-10 账本：本类方法由本轮生成（手写全量类的方法不走 class_writer，不参与断言）
        LAMBDA_NAME_LEDGER.generated_classes.add(ci.name)

    # 接口在 Rust 层是与 Java 同名的载体类型（由 java_class 宏展开）：
    #   - 值：持有 Object 的接口引用，实例方法分派走 Object vtable（impl 块不含实例方法；
    #     default 方法由实现类继承展开，见下方「接口 default 方法继承」）
    #   - 命名空间：static 字段访问器 / static 方法（含 private static 与 static 合成方法）
    #     落在载体的 impl 块，调用点与 Java 同构（`Iface::staticMethod(args)`）
    _is_iface = bool(ci.is_interface)

    parts: list[str] = [
        "#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]",
        f"use {user_crate_prefix or 'crate'}::prelude::*;",
        *cross_imports,
        *([_INHERITED_IMPORTS_SLOT] if emission is not None else []),
        "",
    ]
    inst_fields = [f for f in ci.fields if not f.is_static]

    # Option C（方案 §16「_super 语义边界」）：`_super` 是父类状态的**唯一所有者**，
    # 子类通过宏生成的转发访问器获得「展平字段视图」。
    # 不把父类字段复制进子类 Inner——那会让同一字段存在两份状态并立即分叉。
    _has_super = bool(
        ci.super_class and ci.super_class != _OBJECT_CLASS
    )

    # 用短名作为 Rust 标识符（JDK 类含 /，内部类含 $，均需转换为合法 Rust 名）
    struct_name = short_cls(ci.name) if ('/' in ci.name or '$' in ci.name) else ci.name

    # 类的有效类型参数：自身 Signature 形参；内部类无自身形参时继承外部类形参
    # （Java 内部类经 this$N 隐式可见外部类类型变量，如 ArrayList$Itr → ArrayList_Itr<E>）。
    class_type_params = effective_class_type_params(ci, registry)

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
        {_short_cls_g(_k) for _k in registry}
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
    # 实参取自本类 Signature 的 SuperclassSignature，整条祖先链逐级代入
    # （见 type_map.ancestor_type_args），不做「子类形参按位置套给父类」的猜测。
    parent_rust = ''
    _ancestor_args: dict[str, list[str]] = (
        dict(ancestor_type_args(ci, registry)) if _has_super and registry else {})
    if _has_super:
        parent_rust = _rust_type_with_args(short_cls(ci.super_class),
                                           _ancestor_args.get(ci.super_class, []))

    def _outer_ref_field_rust(f, decl_params: list) -> str:
        return outer_ref_field_type(f, decl_params, registry)

    def _resolve_field_rust(f) -> str:
        """字段的 Rust 类型：优先字段级 generic_signature（TE; → E），回退裸描述符。
        generic_signature 解析为 Object，或引用了不存在的类型时，用描述符推断。"""
        _outer_rust = _outer_ref_field_rust(f, class_type_params)
        if _outer_rust:
            return _outer_rust
        gen_rust = (parse_field_type(f.generic_signature, class_type_params, registry)
                    if f.generic_signature else '')
        desc_rust = jvm_to_rust(f.descriptor, registry)
        return (gen_rust
                if gen_rust and gen_rust != 'Object'
                and _validate_field_type(gen_rust, class_type_params)
                else desc_rust)

    def _resolve_anc_field_rust(f, anc_params: list, anc_map: dict) -> str:
        """祖先字段的 Rust 类型：用祖先自己的 tparams 解析签名，再按 anc_map
        （祖先形参 → 本类视角实参，与 parent_rust / all_superclasses 同源）代入。
        否则父类字段变量（如 AbstractRepository<T> 的 tree: T）被子类 tparams
        （['S']）解析成 Object，转发访问器 __set_tree(v: Object) 与父类
        AbstractRepository<S> 的 __set_tree(v: S) E0308。"""
        # this$N：与祖先自身 struct 的字段类型同规则（见 _outer_ref_field_rust），再代入实参
        gen_rust = _outer_ref_field_rust(f, anc_params) or (
            parse_field_type(f.generic_signature, anc_params, registry)
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
    superclass_reference_fields: list[str] = []
    superclass_erased_fields: list[str] = []
    if _has_super and registry and not _full_impl:
        _chain: list = []
        _seen_chain: set[str] = set()
        _cursor = ci.super_class
        while (_cursor and _cursor != _OBJECT_CLASS
               and _cursor in registry and _cursor not in _seen_chain):
            _seen_chain.add(_cursor)
            _p_ci = registry[_cursor]
            _chain.append(_p_ci)
            _cursor = _p_ci.super_class
        _declared: set[str] = set()
        for _ancestor in reversed(_chain):
            # 祖先形参 → 本类视角实参（沿 SuperclassSignature 逐级代入的结果）
            _anc_params = effective_class_type_params(_ancestor, registry)
            _sub_args = list(_ancestor_args.get(_ancestor.name, []))
            while len(_sub_args) < len(_anc_params):
                _sub_args.append('Object')
            _anc_map = dict(zip(_anc_params, _sub_args))
            for _f in _ancestor.fields:
                if _f.is_static:
                    continue
                # 隐藏更上层祖先同名字段的声明取独立槽位名（Java 字段按声明类静态解析）
                _sf_name = instance_field_rust_name(
                    _ancestor.name, _safe_field_name(_f.name), registry)
                if _sf_name in _declared:
                    continue
                _declared.add(_sf_name)
                _sf_view_ty = _resolve_anc_field_rust(_f, _anc_params, _anc_map)
                superclass_fields.append((_sf_name, _sf_view_ty))
                # 祖先按类型变量声明（引用存储 + __borrow_mut 访问器）而本类视角代入成基本类型
                # （Box<U>.state 在 `extends Box<Long>` 下是 i64）：存储形态由声明方决定，
                # 宏须按引用字段实现祖先 VTable 的访问器
                if (_sf_view_ty in _PRIMITIVE_RUST_TYPES and _resolve_anc_field_rust(
                        _f, _anc_params, {_p: _p for _p in _anc_params}) not in _PRIMITIVE_RUST_TYPES):
                    superclass_reference_fields.append(_sf_name)
                # 声明方（祖先）按自身类型形参声明的字段 → 存储与访问器已被声明方的宏
                # Object 化（A-1 擦除按声明类判定）——继承者的宏按名单同步擦除
                _declared_ty = _resolve_anc_field_rust(
                    _f, _anc_params, {_p: _p for _p in _anc_params})
                if any(_re.search(r'\b' + _re.escape(_p) + r'\b', _declared_ty)
                       for _p in _anc_params):
                    superclass_erased_fields.append(_sf_name)

    # ── struct 声明（裸类型，封装细节由宏收拢）──────────────────────────
    struct_lines: list[str] = []
    if not _full_impl:
        # 父类已有的字段名（继承展平），避免子类重复声明（如内部类 this$0 与父类同名）
        _super_field_names: set[str] = {name for name, _ in superclass_fields}
        for f in inst_fields:
            safe_fname = instance_field_rust_name(ci.name, _safe_field_name(f.name), registry)
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
    # 重载判定在整条父类链上进行（与调用侧 _mangle_if_overloaded 共用同一函数），
    # 保证子类方法名不会按名字遮蔽父类的同名异参方法。
    overloaded_names: set[str] = hierarchy_overloaded_names(ci, registry)

    method_blocks: list[str] = []
    # G-10：接口私有实例 lambda body 落在 java_class! 块之外的擦除 impl 块
    # （不进接口 vtable / 不被实现类继承），见方法循环内的专门分支
    _iface_lambda_blocks: list[str] = []

    # 是否是用户类（call_chain is None 表示用户类，所有方法都翻译）
    _is_user_class = call_chain is None

    # ── static 字段声明（JVMS §5.5 类初始化的事实层）──────────────────────
    # codegen 只声明事实，存储 / 访问器 / 初始化触发全部由 java_class! 宏展开：
    #   ConstantValue 属性 → `pub const NAME: T = 值;`（编译期常量，访问不触发初始化）
    #   其余 static 字段   → `pub static NAME: T;`（初值由 <clinit> 字节码翻译写入）
    # 类型存根（没有任何方法在调用链上：内部边界类 / 仅签名引用的类）不会被初始化：
    # 未被共置手写文件覆盖的 static 字段保持 panic 存根，命中时精确报出字段。
    _type_only = stub_bodies or (
        call_chain is not None
        and not any((ci.name, _m.name, _m.descriptor) in call_chain for _m in ci.methods))
    static_fields = [f for f in ci.fields if f.is_static]
    existing_method_names: set[str] = {m.name for m in ci.methods}
    _nf_covered_sf = (_nf_entry or {}).get('methods', set())
    for sf in static_fields:
        safe_fname = _safe_field_name(sf.name)
        if sf.name in existing_method_names:
            # 字段名与方法名冲突：改用 _field 后缀（读写侧 fields.py 同规则）
            safe_fname = safe_fname + '_field'
        # 优先用 generic_signature 确定字段类型（包含泛型参数信息）
        if sf.generic_signature:
            _gs_ret = parse_field_type(sf.generic_signature, class_type_params, registry)
            # 校验引用的类型存在，否则回退到描述符
            if _gs_ret and _gs_ret != 'Object' and not _validate_field_type(_gs_ret, class_type_params):
                _gs_ret = ''
            # static 字段不在类型参数作用域内（Java 同样禁止），引用类型变量时回退描述符
            if _gs_ret and any(_tp in _re.findall(r'[A-Za-z_][A-Za-z0-9_]*', _gs_ret)
                               for _tp in class_type_params):
                _gs_ret = ''
        else:
            _gs_ret = ''
        rust_ret = _gs_ret if _gs_ret else jvm_to_rust(sf.descriptor, registry=registry)
        field_meta = _java_field_attr(sf)
        cv = sf.constant_value
        if cv:
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
            method_blocks.append(
                f'{field_meta}\n// static field: {sf.name}:{sf.descriptor}\n'
                f'pub const {safe_fname}: {rust_ret} = {body};')
        elif _type_only:
            if safe_fname in _nf_covered_sf:
                continue
            method_blocks.append(
                f'{field_meta}\n// static field: {sf.name}:{sf.descriptor}\n'
                f'pub fn {safe_fname}() -> Result<{rust_ret}> {{\n'
                f'    panic!("stub: {ci.name}.{sf.name}:{sf.descriptor}")\n}}')
        else:
            method_blocks.append(
                f'{field_meta}\n// static field: {sf.name}:{sf.descriptor}\n'
                f'pub static {safe_fname}: {rust_ret};')

    used_rust_names: dict[str, int] = {}  # 追踪已用名，防止 mangle 碰撞后重名
    # 非桥接的 synthetic 方法（lambda$xxx$N、access$NNN 等）是 invokedynamic 闭包 /
    # 内部类访问器的真实调用目标，必须生成定义；桥接方法（ACC_BRIDGE）与真实方法同名，继续过滤。
    # 注意：它们不参与 name_counts / overloaded_names 统计（编译器保证其名字唯一）。
    _ACC_BRIDGE = 0x0040
    emitted_methods = visible_methods + [
        m for m in ci.methods
        if m.is_synthetic and not (m.access_flags & _ACC_BRIDGE)
        and m.name not in ('<init>', '<clinit>')
    ]
    # 接口重声明的 Object 公开方法（如 Comparator.equals）经 Object vtable 分派，不进接口 vtable
    from ..instr.coerce import _root_virtual_methods
    _root_method_keys = _root_virtual_methods() if _is_iface else set()
    for m in emitted_methods:
        if m.name == '<clinit>':
            # <clinit> → `fn __clinit()`：宏生成的 __class_init() 状态机在首次主动使用时调用它。
            # 类型存根不会被初始化；不在调用链上的 <clinit> 与其它方法同规则生成 panic 存根。
            if _type_only:
                continue
            attr_line = _java_method_attr(m)
            _clinit_stub = (f'pub fn {_CLINIT_FN}() -> Result<()> {{\n'
                            f'    panic!("stub: {ci.name}.<clinit>:()V")\n}}')
            if call_chain is not None and (ci.name, m.name, m.descriptor) not in call_chain:
                method_blocks.append(attr_line + '\n' + _clinit_stub)
                continue
            try:
                clinit_body = gen_method_body(
                    m, ci, registry=registry,
                    class_type_params=class_type_params,
                    overloaded_names=overloaded_names,
                    rust_name=_CLINIT_FN,
                )
                method_blocks.append(attr_line + '\n' + clinit_body)
            except CfgAuditError:
                raise
            except Exception as e:
                _CFG_STATS.record_stub_fallback(f"{ci.name}.{m.name}:{m.descriptor}", repr(e))
                import os as _os
                if _os.environ.get('JAVA_RTA_DEBUG'):
                    import traceback as _tb
                    print(f"[DEBUG] stub fallback for {ci.name}.<clinit>: {e}", file=__import__('sys').stderr)
                    _tb.print_exc()
                method_blocks.append(attr_line + '\n' + _clinit_stub)
            continue
        if _is_iface and not m.is_static and m.is_synthetic and m.name.startswith('lambda$'):
            # G-10：接口的私有实例 lambda body（如 Comparator.lambda$thenComparing$...）。
            # 不是接口契约成员：不进 Iface__VTable、不被实现类继承展开，而是落在接口
            # 载体擦除实例化（Iface<Object>）的固有 impl 块 —— 调用点（sim/dynamic.py
            # 的接口实例分支）生成 Into::<Iface<Object>>::into(recv).<name>(args)，
            # 接收者即擦除载体，body 内对 this 的虚调用经载体 vtable 分派（JVM 语义：
            # 私有 lambda 体在运行时接收者上执行）。方法名取 lambda_impl_rust_name
            # 单一来源，与调用点引用名一起受 G-10 生成期断言保护。
            _lam_rust = lambda_impl_rust_name(ci.name, m.name, m.descriptor, registry)
            # this 按擦除实例化定型（Comparator<T> → Comparator<Object>），与调用点
            # 接收者的 Into::<Iface<Object>> 一致；body 内 Object 实参直接命中擦除签名
            _lam_ctparams = ['Object'] * len(class_type_params) if class_type_params else []
            _lam_in_chain = (call_chain is None or (ci.name, m.name, m.descriptor) in call_chain)
            try:
                _lam_block = gen_method_body(
                    m, ci, registry=registry,
                    class_type_params=_lam_ctparams,
                    overloaded_names=overloaded_names,
                    rust_name=_lam_rust,
                ) if (_lam_in_chain and not stub_bodies) else _gen_native_stub(
                    m, ci, rust_name=_lam_rust, registry=registry,
                    class_type_params=_lam_ctparams)
            except CfgAuditError:
                raise
            except Exception as e:
                _CFG_STATS.record_stub_fallback(f"{ci.name}.{m.name}:{m.descriptor}", repr(e))
                _lam_block = _gen_native_stub(m, ci, rust_name=_lam_rust, registry=registry,
                                              class_type_params=_lam_ctparams)
            _iface_lambda_blocks.append(_lam_block)
            LAMBDA_NAME_LEDGER.record_definition(ci.name, m.name, _lam_rust)
            continue
        if _is_iface and not m.is_static and (
                m.is_synthetic or (m.access_flags & 0x0002) or (m.name, m.descriptor[:m.descriptor.index(')') + 1]) in _root_method_keys):
            continue  # 其余私有 / 合成实例方法不是接口契约的一部分
        # 确定最终 Rust 方法名（有重载则加描述符后缀）
        rust_name = (mangle_name(m.name, m.descriptor)
                     if method_name_is_mangled(ci, m, registry) else m.name)
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
            # 手写共置文件按同一 mangle 规则提供实现 → 定义名仍记为计算名（G-10 账本）
            LAMBDA_NAME_LEDGER.record_definition(ci.name, m.name, fn_name_check)
            continue
        # G-10 账本：定义侧登记最终 Rust 名（stub / 翻译体 / 接口声明各路径统一在此登记）
        LAMBDA_NAME_LEDGER.record_definition(ci.name, m.name, fn_name_check)

        if _is_iface and not m.is_static:
            # 接口实例方法（abstract / default）：宏据此生成擦除签名的接口 vtable
            # （`Iface__VTable`）与载体上的同名分派方法。abstract 保持无体声明；
            # default 方法体由实现类继承展开（类覆盖优先），**同时**以载体为接收者
            # 翻译一份落到接口自身块内 —— 载体分派在 vtable 查询未命中（lambda /
            # 闭包接收者不实现 `Iface__VTable`）时执行它，对应 JVM 对函数式接口
            # 实例调用 default 方法的语义（A-5 的闭包接收者路径）。
            _dm_iface_body = None
            # 含 `Iface.super.m()`（invokespecial 常量池类为接口）的 default 体不落到
            # 接口载体：其展开成员（`Owner_super_m`）建模在实现类，载体上下文不存在。
            # 判定与调用侧（invoke.py invokespecial）同源：_resolve_interface_special_target。
            from ..instr.coerce import (
                _resolve_interface_special_target as _rist_dm,
                _method_ref_binary_class as _mrbc_dm,
                _method_ref_descriptor as _mrd_dm,
            )

            def _ref_mname(c: str) -> str:
                _rest = c.split(' ', 1)[1] if ' ' in c else c
                _dot = _rest.find('.')
                _colon = _rest.find(':', _dot)
                return _rest[_dot + 1:_colon] if 0 < _dot < _colon else ''

            _has_iface_super = False
            for _ins in (m.instrs or []):
                if _ins.opcode != 'invokespecial' or not _ins.comment:
                    continue
                if _rist_dm(_mrbc_dm(_ins.comment), _ref_mname(_ins.comment),
                            _mrd_dm(_ins.comment), registry):
                    _has_iface_super = True
                    break
            # 载体只声明本接口的方法：default 体调用本接口未声明的方法（典型：
            # `this.hasNext()` 声明在父接口 Iterator）时不落接口 —— 实现类侧的
            # 继承成员展开（inherited_from）在载体上下文不存在，仍走类 vtable 分派。
            _own_sigs = {(_mm.name, _mm.descriptor) for _mm in ci.methods}
            _calls_nonself = False
            for _ins in (m.instrs or []):
                if _ins.opcode not in ('invokevirtual', 'invokeinterface') or not _ins.comment:
                    continue
                if (_ref_mname(_ins.comment), _mrd_dm(_ins.comment)) not in _own_sigs:
                    _calls_nonself = True
                    break
            _dm_in_cc = (
                not m.is_abstract and not m.is_synthetic and not _has_iface_super
                and not _calls_nonself
                and (call_chain is None or (ci.name, m.name, m.descriptor) in call_chain)
            )
            if _dm_in_cc and not stub_bodies:
                try:
                    _dm_iface_body = gen_method_body(
                        m, ci, registry=registry,
                        class_type_params=class_type_params,
                        overloaded_names=overloaded_names,
                        rust_name=rust_name,
                        in_vtable_body=False,
                    )
                except CfgAuditError:
                    raise
                except Exception as e:
                    _CFG_STATS.record_stub_fallback(
                        f"{ci.name}.{m.name}:{m.descriptor}(iface-default)", repr(e))
                    _dm_iface_body = None
            if _dm_iface_body is not None:
                method_blocks.append(_java_method_attr(m) + '\n' + _dm_iface_body)
                continue
            _decl = _gen_native_stub(m, ci, rust_name=rust_name, registry=registry,
                                     class_type_params=class_type_params)
            _decl_sig = next(ln.strip() for ln in _decl.split('\n') if ln.lstrip().startswith('pub fn '))
            _decl_sig = _decl_sig[:-1].rstrip() if _decl_sig.endswith('{') else _decl_sig
            method_blocks.append(_java_method_attr(m) + '\n' + _decl_sig + ';')
            continue

        # 计算虚方法归属（vtable 架构）
        m.virtual_in = _find_virtual_in(m, ci, registry, new_format_map)
        # A-1 擦除按声明类判定：覆盖条目的签名里，「声明祖先按自身类型形参声明的位置」
        # 在祖先 vtable 上已是 Object → 记录接收者视角下这些位置的代入形态（类型串），
        # 宏据此擦除 vtable impl 条目 / 包装 base 调用 turbofish（vtable_erasure）
        if m.virtual_in and m.virtual_in != short_cls(ci.name):
            m.vtable_erasure = _override_vtable_erasure(m, ci, registry)

        # 虚方法的方法体由共置 `_impl.rs` 手写为 `__impl_<method>`：声明留在宏块内（进 vtable、
        # 参与覆盖与根类方法桥接），宏经 wrapper 钩子执行手写体
        if m.virtual_in and ('__impl_' + fn_name_check) in _nf_covered:
            m.handwritten_body = True
            _decl = _gen_native_stub(m, ci, rust_name=rust_name, registry=registry,
                                     class_type_params=class_type_params)
            _decl_sig = next(ln.strip() for ln in _decl.split('\n') if ln.lstrip().startswith('pub fn '))
            _decl_sig = _decl_sig[:-1].rstrip() if _decl_sig.endswith('{') else _decl_sig
            method_blocks.append(_java_method_attr(m) + '\n' + _decl_sig + ';')
            continue

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
                method_blocks.append(attr_line + '\n' + body)
            except CfgAuditError:
                raise
            except Exception as e:
                # 翻译失败：退化为 stub，避免生成无效 Rust
                _CFG_STATS.record_stub_fallback(f"{ci.name}.{m.name}:{m.descriptor}", repr(e))
                import os as _os
                if _os.environ.get('JAVA_RTA_DEBUG'):
                    import traceback as _tb
                    print(f"[DEBUG] stub fallback for {ci.name}.{m.name}{m.descriptor}: {e}", file=__import__('sys').stderr)
                    _tb.print_exc()
                stub = _gen_native_stub(m, ci, rust_name=rust_name, registry=registry, class_type_params=class_type_params)
                method_blocks.append(attr_line + '\n' + stub)

    # 已翻译方法体的接口 default 方法（展开到本类）：其字节码同样可能含 `Iface.super.m()`
    _translated_defaults: list = []
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
            (mangle_name(m.name, m.descriptor) if method_name_is_mangled(ci, m, registry) else m.name)
            for m in visible_methods if m.name not in ('<init>', '<clinit>')
        }
        # 祖先类已实现的接口：其 default 方法已注入到该祖先（VirtualDefine），
        # 本类经 VTable supertrait 链继承；重复注入会在 vtable 上产生同名二义（E0034）。
        _anc_ifaces: set[str] = set()
        _anc_cur = ci.super_class
        _anc_seen: set[str] = set()
        while _anc_cur and _anc_cur in registry and _anc_cur not in _anc_seen:
            _anc_seen.add(_anc_cur)
            _anc_ci = registry[_anc_cur]
            _aq = list(_anc_ci.interfaces or [])
            while _aq:
                _an = _aq.pop(0)
                if _an in _anc_ifaces:
                    continue
                _anc_ifaces.add(_an)
                _a_ci = registry.get(_an)
                if _a_ci is not None:
                    _aq.extend(_a_ci.interfaces or [])
            _anc_cur = _anc_ci.super_class
        # 预扫描：统计所有待继承 default 方法的名字（用于 default 方法之间互相冲突判断）
        default_name_counts: dict[str, int] = {}
        _pre_counted_sigs: set[tuple] = set()
        _pre_iface_queue = list(ci.interfaces)
        _pre_visited: set[str] = set(_anc_ifaces)
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
                    # 子接口覆盖父接口的同签名 default（如子接口重新声明 and(P)）只注入一次，
                    # 计数也必须按 (name, 参数签名) 去重，否则单一方法被误判为重载而 mangle
                    if ((_dm.name, _dm.descriptor) not in existing_sigs
                            and (_dm.name, _dm_pp) not in existing_param_sigs
                            and (_dm.name, _dm_pp) not in _pre_counted_sigs):
                        _pre_counted_sigs.add((_dm.name, _dm_pp))
                        default_name_counts[_dm.name] = default_name_counts.get(_dm.name, 0) + 1
        iface_queue: list[str] = list(ci.interfaces)
        visited_ifaces: set[str] = set(_anc_ifaces)
        _iface_sig_views = _interface_signature_views(ci, registry)
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
                dm_adapted = _adapt_interface_method(dm, ci, iface_name, _iface_sig_views)
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
                        _translated_defaults.append(dm)
                    except CfgAuditError:
                        raise
                    except Exception as e:
                        _CFG_STATS.record_stub_fallback(f"{ci.name}.{dm.name}:{dm.descriptor}", repr(e))
                        dm_stub = _gen_native_stub(dm_adapted, ci, rust_name=dm_rust, registry=registry, class_type_params=class_type_params)
                        method_blocks.append(dm_attr + '\n' + dm_stub)
                else:
                    dm_stub = _gen_native_stub(dm_adapted, ci, rust_name=dm_rust, registry=registry, class_type_params=class_type_params)
                    method_blocks.append(dm_attr + '\n' + dm_stub)

    # `Iface.super.m()` / 接口私有方法（invokespecial InterfaceMethod）的落点：
    # 接口方法体按「展开到实现类」建模，被本类覆盖的 default 方法体（及接口私有方法体）
    # 以非虚成员 `Iface_super_m` 展开到本类；展开出的方法体自身的同类调用递归处理。
    if registry and not ci.is_interface and not stub_bodies:
        import copy as _copy_sp
        from ..instr.coerce import (
            _resolve_interface_special_target as _rist,
            interface_special_member_name as _ismn,
            _method_ref_binary_class as _mrbc,
            _method_ref_descriptor as _mrd_sp,
            parse_method_ref as _pmr_sp,
        )
        _sp_sources: list = [
            _m for _m in visible_methods
            if call_chain is None or (ci.name, _m.name, _m.descriptor) in call_chain
        ] + _translated_defaults
        _sp_done: set[tuple] = set()
        while _sp_sources:
            _src = _sp_sources.pop(0)
            for _ins in getattr(_src, 'instrs', []) or []:
                if getattr(_ins, 'opcode', '') != 'invokespecial':
                    continue
                _c = (getattr(_ins, 'comment', '') or '').strip()
                if not _c or '<init>' in _c:
                    continue
                _, _sp_mname, _, _ = _pmr_sp(_c)
                _sp_desc = _mrd_sp(_c)
                _sp_owner = _rist(_mrbc(_c), _sp_mname, _sp_desc, registry)
                if not _sp_owner or (_sp_owner, _sp_mname, _sp_desc) in _sp_done:
                    continue
                _sp_done.add((_sp_owner, _sp_mname, _sp_desc))
                _sp_m = next(_im for _im in registry[_sp_owner].methods
                             if _im.name == _sp_mname and _im.descriptor == _sp_desc
                             and not _im.is_static and not _im.is_abstract)
                _sp_rust = safe_ident(_ismn(_sp_owner, _sp_mname, _sp_desc, registry))
                _sp_adapted = _adapt_interface_method(
                    _sp_m, ci, _sp_owner, _interface_signature_views(ci, registry))
                _sp_adapted.virtual_in = None  # 非虚：只经 `Iface.super.m()` 静态绑定到达
                _sp_attr = _java_method_attr(_sp_adapted)
                _sp_in_cc = (call_chain is None
                             or (_sp_owner, _sp_mname, _sp_desc) in call_chain)
                _sp_block = None
                if _sp_in_cc:
                    try:
                        _sp_block = gen_method_body(
                            _sp_adapted, ci, registry=registry,
                            class_type_params=class_type_params,
                            overloaded_names=overloaded_names,
                            rust_name=_sp_rust,
                            in_vtable_body=False,
                        )
                        _sp_sources.append(_sp_m)
                    except CfgAuditError:
                        raise
                    except Exception as e:
                        _CFG_STATS.record_stub_fallback(
                            f"{ci.name}.{_sp_mname}:{_sp_desc}", repr(e))
                if _sp_block is None:
                    _sp_block = _gen_native_stub(_sp_adapted, ci, rust_name=_sp_rust,
                                                 registry=registry,
                                                 class_type_params=class_type_params)
                method_blocks.append(_sp_attr + '\n' + _sp_block)

    # 超类虚方法继承：若子类未覆盖祖先虚方法，生成继承实现使 vtable impl 包含实际函数体。
    # 这避免子类的 Ancestor__VTable impl 退化为 panic!("stub")。
    # 仅对用户类超类链（无 '/'）处理，JDK 类的继承由 BFS 方法级调用链保证。
    if not ci.is_interface and registry and ci.super_class and '/' not in ci.super_class:
        import copy as _copy3
        _vinh_existing: set[tuple] = {(m.name, m.descriptor) for m in visible_methods}
        _vinh_super = ci.super_class
        while _vinh_super and _vinh_super != _OBJECT_CLASS and '/' not in _vinh_super:
            _vinh_sci = registry.get(_vinh_super)
            if _vinh_sci is None:
                break
            for _vm in _vinh_sci.methods:
                if (_vm.name, _vm.descriptor) in _vinh_existing:
                    continue
                if _vm.is_static or _vm.is_constructor or _vm.name in ('<init>', '<clinit>'):
                    continue
                _vm_virt_in = _find_virtual_in(_vm, _vinh_sci, registry, new_format_map)
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
                    except CfgAuditError:
                        raise
                    except Exception as e:
                        _CFG_STATS.record_stub_fallback(f"{ci.name}.{_vm.name}:{_vm.descriptor}", repr(e))
                        _vm_stub = _gen_native_stub(_vm2, ci, registry=registry, class_type_params=class_type_params)
                        method_blocks.append(_vm_attr + '\n' + _vm_stub)
            _vinh_super = _vinh_sci.super_class if _vinh_sci.super_class else None

    # Record 类（super_class == java/lang/Record）：覆盖 invokedynamic 无法翻译的方法
    if ci.super_class == 'java/lang/Record' and not ci.is_interface:
        record_fields = [f for f in ci.fields if not f.is_static]
        simple_name = ci.name.split('$')[-1].split('/')[-1]
        fmt_parts = [f'{f.name}={{}}' for f in record_fields]
        fmt_str = f'{simple_name}[{", ".join(fmt_parts)}]'
        # 字段一律经宏生成的访问器读取（struct 字段的存储形态是宏的实现细节）
        # 基本类型 / 字符串分量直接格式化；其余引用分量按 Java 语义经 Object.toString() 取文本
        def _record_component_text(_rf) -> str:
            _rf_ty = jvm_to_rust(_rf.descriptor, registry)
            _rf_get = f'this.__get_{safe_ident(_rf.name)}()'
            if _rf_ty in _PRIMITIVE_RUST_TYPES or _rf_ty == jvm_to_rust(f'L{STRING_CLASS};', registry):
                return _rf_get
            return f'Into::<Object>::into({_rf_get}).toString()?'
        # 方法体与字节码翻译的方法同一约定：`let this = self;` + 字段访问器
        # （宏把虚方法体搬进 `this: &__BT` 的自由函数，其中不存在 `self` / `Self`）
        fmt_args = ', '.join(_record_component_text(f) for f in record_fields)
        _record_ty = f'{struct_name}{struct_generic}'
        new_blocks = []
        for block in method_blocks:
            if '/* TODO: invokedynamic' in block and any(f'pub fn {n}(' in block for n in ('toString', 'hashCode', 'equals')):
                if 'pub fn toString(' in block:
                    attr = block[:block.index('pub fn toString(')]
                    fmtcall = (f'String::from(format!("{fmt_str}", {fmt_args}).as_str())' if fmt_args
                               else f'String::from("{fmt_str}")')
                    new_blocks.append(attr +
                        f'pub fn toString(&self) -> Result<String> {{\n    let this = self;\n    Ok({fmtcall})\n}}')
                elif 'pub fn hashCode(' in block:
                    attr = block[:block.index('pub fn hashCode(')]
                    new_blocks.append(attr +
                        f'pub fn hashCode(&self) -> Result<i32> {{\n    Ok(0)\n}}')
                elif 'pub fn equals(' in block:
                    attr = block[:block.index('pub fn equals(')]
                    field_cmps = []
                    for f in record_fields:
                        fname = safe_ident(f.name)
                        rust_fty = jvm_to_rust(f.descriptor, registry)
                        if rust_fty == 'String':
                            field_cmps.append(
                                f'this.__get_{fname}().to_string() == other.__get_{fname}().to_string()'
                            )
                        else:
                            field_cmps.append(f'this.__get_{fname}() == other.__get_{fname}()')
                    cmp_expr = ' && '.join(field_cmps) if field_cmps else 'true'
                    new_blocks.append(attr +
                        f'pub fn equals(&self, mut o: Object) -> Result<bool> {{\n'
                        f'    let this = self;\n'
                        f'    if !o.is_instance_of("{ci.name}") {{\n'
                        f'        return Ok(false);\n'
                        f'    }}\n'
                        f'    let other = Into::<{_record_ty}>::into(Clone::clone(&o));\n'
                        f'    Ok({cmp_expr})\n'
                        f'}}')
                else:
                    new_blocks.append(block)
            else:
                new_blocks.append(block)
        method_blocks = new_blocks

    # ── 组装 java_class! { ... } 块（方案 §3 核心设计）────────────────────
    if not _full_impl:
        block: list[str] = []
        block.extend(_java_class_block_head(
            ci, registry=registry,
            superclass_rust=parent_rust,
            superclass_fields=superclass_fields,
            superclass_reference_fields=superclass_reference_fields,
            superclass_erased_fields=superclass_erased_fields,
            impl_methods=set((_nf_entry or {}).get('methods', set())),
            handwritten_methods=new_format_map,
        ))
        block.append('')
        # struct 声明：裸类型（封装细节收拢进宏），无 derive / 无 _super / 无 _phantom
        if struct_lines:
            block.append(f"pub struct {struct_name}{struct_generic} {{")
            block.extend(struct_lines)
            block.append("}")
        else:
            block.append(f"pub struct {struct_name}{struct_generic};")

        # 接口无成员且不可能补继承成员（无 emission）时不写空 impl 块
        if method_blocks or not _is_iface or emission is not None:
            block.append('')
            impl_body = '\n\n'.join(_indent(b) for b in method_blocks)
            block.append(f"{impl_header} {{")
            if impl_body:
                block.append(impl_body)
            if emission is not None:
                block.append(_INHERITED_MEMBERS_SLOT)
            block.append("}")
        if emission is not None and not _is_iface:
            block.append(_INTERFACE_IMPLS_SLOT)

        if emission is not None:
            emission.record_methods(method_blocks)
        parts.append("java_rta_macros::java_class! {")
        for line in block:
            parts.append(_indent(line) if line else '')
        parts.append("}")
        if emission is not None:
            # A-4 协变 upcast 占位（interface_gen 填充：impl From<Class> for Iface<Object>，
            # 载体构造依赖其私有字段，必须经载体的 From<Object>，不属于宏输入）
            parts.append(_INTERFACE_UPCASTS_SLOT)
        parts.append('')

        # G-10：接口私有实例 lambda body 的擦除固有 impl 块。置于 java_class! 块之外，
        # 避免被宏归入接口 vtable / 载体分派（lambda 体不是接口契约，只被 invokedynamic
        # 调用点按名引用）。this 与调用点接收者同为 Iface<Object> 擦除实例化。
        if _iface_lambda_blocks:
            _lam_recv = (f"{struct_name}<{', '.join(['Object'] * len(class_type_params))}>"
                         if class_type_params else struct_name)
            parts.append('// G-10: 接口私有实例 lambda body —— 载体擦除实例化上的固有方法')
            parts.append(f'impl {_lam_recv} {{')
            for _lb in _iface_lambda_blocks:
                parts.append(_indent(_lb))
            parts.append('}')
            parts.append('')

    # 扫描方法体中使用的 VTable trait（UFCS 调用 XxxVTable::method(...)），
    # 为未导入的 VTable 类型补充 use 语句（避免 E0433）。不盲目为所有类添加
    # __VTable（手写类如 Object/String 不一定有），而是按实际生成代码中
    # 出现的名称按需导入（import_gen.scan_used_vtable_imports）。
    _prefix = user_crate_prefix or 'crate'
    parts.extend(scan_used_vtable_imports(
        method_blocks, _iface_lambda_blocks, cross_imports,
        struct_name, registry, _prefix))


    # BINARY_NAME / ObjectVTable / Into<Object> / From<Object> / Debug 全部由
    # java_class! 宏在编译期展开（方案 §11 职责边界总表）。
    # （Object 类走手写路径 java_runtime/，不经过此函数）

    return '\n'.join(parts)