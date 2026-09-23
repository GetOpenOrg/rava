"""引用收集与 cross_imports 生成（从 class_writer._gen_class_rs 迁出）。

- collect_referenced（原 Step 1）：计算本类文件需引用的类型集合——超类链 /
  接口 / 类型变量上界 / 方法指令（Method/Field 引用、invokedynamic、
  new/checkcast 等、类字面量）/ 局部变量与异常表 / multi-catch 公共祖先 /
  字段与方法描述符 / 虚分派子类型。
- gen_cross_imports（原 Step 2）：精确 use 逐类型导入——JDK 包 / 同包兄弟类 /
  conflict_map 消歧 / skipped_classes / VTable trait 导入 / __base 函数导入 /
  用户内部类兄弟模块。
- scan_used_vtable_imports：方法体 UFCS 调用使用的 __VTable 按需补导。
- 嵌套函数 _strip_generic / _add_desc_refs / _add_precise_import 提升为模块级，
  闭包变量（_referenced / _prefix / _self_simple / _seen_imports / _seen_simples /
  cross_imports）改为显式参数传递，函数体逻辑不变。
依赖方向 class_writer → import_gen 单向。
"""

import re as _re

from ..type_map import short_cls as _short_cls_g
from ..type_args import class_type_param_bounds
from ..constants import (OBJECT_CLASS as _OBJECT_CLASS,
                         CLASS_CLASS as _CLASS_CLASS,
                         RUST_KEYWORDS as _RUST_KEYWORDS)
from .attrs import to_snake


# 模块级 regex，避免在每次调用时重复编译
_CLS_RE_NARROW = _re.compile(r'L([^;]+);')          # 平铺 descriptor（如 (LFoo;)V）
_CLS_RE_WIDE   = _re.compile(r'L([^;<>\[()\s]+)')   # 含嵌套泛型的 generic_signature
# 操作数为常量池 Class 引用的指令（comment = binary name 或数组描述符）
_CLASS_OPERAND_OPCODES = frozenset({'new', 'anewarray', 'checkcast', 'instanceof', 'multianewarray'})


def _strip_generic(cls: str) -> str:
    """去掉 JVM 类名中的泛型参数（<...>），返回裸 binary name。
    例：java/util/Collection<*> → java/util/Collection"""
    idx = cls.find('<')
    return cls[:idx] if idx >= 0 else cls


def _add_desc_refs(text: str, _referenced: set[str]) -> None:
    """把描述符 / 泛型签名文本中出现的全部类引用加入 _referenced。"""
    for _dm2 in _CLS_RE_WIDE.finditer(text or ''):
        _dc = _strip_generic(_dm2.group(1))
        if _dc:
            _referenced.add(_dc)


def _superclass_arg_refs(ci, registry) -> set[str]:
    """类操作数（new / checkcast 等）超类链实例化里引用的类（import 收账）。

    方法体对子类值上转时按其实例化渲染祖先视图（invoke_sig 的
    `_exact_ancestor_type` → `Into::<Ancestor<Arg>>`，如
    `new ForkJoinTask$AdaptedRunnableAction` 上转出 `ForkJoinTask<Void>`）。
    Arg 里的类来自操作数类超类链各节点 generic_signature 的**超类实参部分**
    （AdaptedRunnableAction 的 `LForkJoinTask<Ljava/lang/Void;>;` → Void）——
    不在本类自身签名 / LVT / 描述符的任何扫描面里，无此收账则 E0425。

    只取超类部分顶层实参（不含超类自身名字与接口段）：实参才是发射面新出现的
    名字，超类名总是与期望类型同名（已由期望类型路径导入），接口不经
    ancestor_type_args 渲染——收窄保证既有文件引用集零扰动。"""
    out: set[str] = set()
    cur = ci
    seen: set[str] = set()
    while (cur is not None and cur.name not in seen and cur.name != _OBJECT_CLASS
           and cur.super_class and cur.super_class != _OBJECT_CLASS
           and cur.super_class in registry):
        seen.add(cur.name)
        sig = getattr(cur, 'generic_signature', '') or ''
        i = 0
        if sig.startswith('<'):
            depth = 0
            while i < len(sig):
                if sig[i] == '<':
                    depth += 1
                elif sig[i] == '>':
                    depth -= 1
                    if depth == 0:
                        i += 1
                        break
                i += 1
        # 超类类型 `Lpkg/Parent<Args>;` 到首个深度 0 的 `;` 结束（其后是接口段）；
        # 有顶层 `<...>` 时其实参段为渲染面（首个 `<` 到配对 `>`，深度平衡）
        if i < len(sig) and sig[i] == 'L':
            depth, j = 0, i
            sup_end = -1
            while j < len(sig):
                if sig[j] == '<':
                    depth += 1
                elif sig[j] == '>':
                    depth -= 1
                elif sig[j] == ';' and depth == 0:
                    sup_end = j
                    break
                j += 1
            if sup_end > 0:
                lt = sig.find('<', i, sup_end)
                if lt >= 0:
                    # 从顶层开括号之后起扫：配对闭括号使深度转负（嵌套括号先开后闭）
                    depth, j = 0, lt + 1
                    while j < sup_end:
                        if sig[j] == '<':
                            depth += 1
                        elif sig[j] == '>':
                            depth -= 1
                            if depth < 0:
                                _add_desc_refs(sig[lt + 1:j], out)
                                break
                        j += 1
        cur = registry[cur.super_class]
    return out

def collect_referenced(ci, registry, generated_classes) -> set[str]:
    """Step 1：计算本类文件需引用的类型集合（精确 use 生成的基础）。"""
    _referenced: set[str] = set()
    # 超类链：宏为每个祖先生成 From<Self> for Ancestor，需要全部祖先类型名在作用域内
    _sc_cur = ci.super_class
    while _sc_cur and _sc_cur != _OBJECT_CLASS:
        _referenced.add(_sc_cur)
        if registry and _sc_cur in registry:
            _sc_cur = registry[_sc_cur].super_class
        else:
            break
    for _iface in (ci.interfaces or []):
        if _iface != _OBJECT_CLASS:
            _referenced.add(_iface)
    # 类级类型变量的类上界（`E extends B<E>`）：方法体把类型变量值转换为上界类型、
    # 方法签名声明 `where E: Into<B<E>>`，上界类型名须在作用域内
    for _tv_bound in class_type_param_bounds(ci, registry).values():
        _referenced.add(_tv_bound[1])
    # 参与引用扫描的方法集合：自身方法 + 会被注入本类的继承方法体
    # （接口 default 方法、用户类超类链的虚方法——见下方「接口 default 方法继承」
    #   与「超类虚方法继承」两段）。注入的方法体/签名同样出现在本文件中，
    #   其引用的类型必须一并导入，否则产生 E0425/E0433。
    _scan_methods: list = list(ci.methods)
    if registry and not ci.is_interface:
        _scan_iface_queue = list(ci.interfaces or [])
        _scan_iface_seen: set[str] = set()
        while _scan_iface_queue:
            _scan_iname = _scan_iface_queue.pop(0)
            if _scan_iname in _scan_iface_seen:
                continue
            _scan_iface_seen.add(_scan_iname)
            _scan_ici = registry.get(_scan_iname)
            if _scan_ici is None:
                continue
            _scan_iface_queue.extend(_scan_ici.interfaces or [])
            _scan_methods.extend(
                _dm for _dm in _scan_ici.methods
                if not _dm.is_abstract and not _dm.is_static)
        _scan_super = ci.super_class
        while (_scan_super and _scan_super != _OBJECT_CLASS
               and '/' not in _scan_super and _scan_super in registry):
            _scan_methods.extend(registry[_scan_super].methods)
            _scan_super = registry[_scan_super].super_class


    # 类级泛型签名（本类 + 超类链）：超类/接口的类型实参（`Base<Arg>`）出现在宏属性
    # superclass / all_superclasses 与宏生成的祖先转换 impl 中，类型实参名须在作用域内
    _sig_cur = ci
    _sig_seen: set[str] = set()
    while _sig_cur is not None and _sig_cur.name not in _sig_seen:
        _sig_seen.add(_sig_cur.name)
        _add_desc_refs(getattr(_sig_cur, 'generic_signature', '') or '', _referenced)
        _sig_cur = registry.get(_sig_cur.super_class) if registry and _sig_cur.super_class else None

    # 扫描方法指令中的类型引用
    for _m in _scan_methods:
        for _instr in (_m.instrs or []):
            _c = _instr.comment
            if not _c:
                continue
            if _c.startswith(('Method ', 'InterfaceMethod ', 'Field ')):
                _rest = _c.split(' ', 1)[1]
                _dot = _rest.find('.')
                if _dot > 0:
                    _referenced.add(_rest[:_dot])
                # 被调方法的参数/返回类型、被访问字段的类型：
                # 方法体中以 `let _tN: RetType = ...` / downcast::<ParamType>() 形式出现
                _colon = _rest.find(':')
                if _colon > 0:
                    _add_desc_refs(_rest[_colon + 1:], _referenced)
            elif _instr.opcode == 'invokedynamic':
                # 方法引用 / lambda：闭包体以 `Owner::m(...)` / `Owner::new(...)` 形式调用
                # 实现方法，其声明类与签名类型须在作用域内
                _impl_at = _c.find(' impl:')
                if _impl_at >= 0:
                    _impl_ref = _c[_impl_at + len(' impl:'):].split(' ', 1)[0]
                    _impl_dot = _impl_ref.find('.')
                    if _impl_dot > 0:
                        _referenced.add(_impl_ref[:_impl_dot])
                    _impl_colon = _impl_ref.find(':')
                    if _impl_colon > 0:
                        _add_desc_refs(_impl_ref[_impl_colon + 1:], _referenced)
            elif _instr.opcode in _CLASS_OPERAND_OPCODES:
                # new / anewarray / checkcast / instanceof / multianewarray：
                # comment 为裸 binary name 或数组描述符（[Lpkg/Cls;）
                if _c.startswith('['):
                    _add_desc_refs(_c, _referenced)
                else:
                    _referenced.add(_strip_generic(_c))
                    # 类操作数的超类链实例化收账（见 _superclass_arg_refs）：
                    # 上转渲染的祖先实参类（ForkJoinTask<Void> 的 Void）
                    if registry:
                        _op_ci = registry.get(_strip_generic(_c))
                        if _op_ci is not None:
                            _referenced |= _superclass_arg_refs(_op_ci, registry)
            elif _c.startswith('class '):
                # ldc / ldc_w 类字面量（`X.class`）：方法体发射 `Class::for_class(..)`，
                # 结果类型 Class 须在本文件作用域内（E0433 的来源）
                _referenced.add(_CLASS_CLASS)
        # 局部变量声明类型（LocalVariableTable / LocalVariableTypeTable）：
        # 方法体按声明类型生成 `let x: T`
        for _lv in (getattr(_m, 'local_vars', None) or []):
            _add_desc_refs(_lv[4], _referenced)
            _add_desc_refs(_lv[5], _referenced)
        # 异常表 catch_type：方法体以 `catch (e: T)` 形式引用
        _catch_by_handler: dict[int, list[str]] = {}
        for _exc in (getattr(_m, 'exception_table', None) or []):
            if _exc[3]:
                _referenced.add(_exc[3])
                _handler_types = _catch_by_handler.setdefault(_exc[2], [])
                if _exc[3] not in _handler_types:
                    _handler_types.append(_exc[3])
            else:
                # catch_type 为空 = catch-any 处理器（try/finally 合成的 rethrow 臂）：
                # 绑定类型按 try_catch._throwable_root 恒为 Throwable，方法体以
                # `let e: Throwable = _caughtN;` / `catch (e: Throwable)` 引用。
                # 合成处理器无 LVT 条目，用户类（源码无 catch 变量）不会经
                # 局部变量声明路径兜底引入 → 在此显式收账，否则用户 bin E0425
                from ..constants import THROWABLE_CLASS as _THROWABLE_CLASS
                _referenced.add(_THROWABLE_CLASS)
        # multi-catch（`catch (e: A | B as LUB)`）：绑定类型是各 catch 类型的最近公共祖先类
        for _handler_types in _catch_by_handler.values():
            if len(_handler_types) < 2 or not registry:
                continue
            _common: list[str] | None = None
            for _ct in _handler_types:
                _chain: list[str] = []
                _cc = _ct
                while _cc and _cc != _OBJECT_CLASS and _cc not in _chain:
                    _chain.append(_cc)
                    _cc = registry[_cc].super_class if _cc in registry else None
                _common = _chain if _common is None else [c for c in _common if c in _chain]
            if _common:
                _referenced.add(_common[0])
    # 扫描字段描述符（含超类链继承字段）
    _all_fields_to_scan = list(ci.fields)
    if registry:
        _sc_scan = ci.super_class
        _seen_scan: set[str] = {f.name for f in ci.fields}
        while _sc_scan and _sc_scan != _OBJECT_CLASS and _sc_scan in registry:
            _sci_scan = registry[_sc_scan]
            for _f2 in _sci_scan.fields:
                if not _f2.is_static and _f2.name not in _seen_scan:
                    _all_fields_to_scan.append(_f2)
                    _seen_scan.add(_f2.name)
            _sc_scan = _sci_scan.super_class
    # 泛型签名中嵌套类型需要用更宽松的 regex（不能用 [^;]+ 因为嵌套 <TT;> 会截断）
    # _CLS_RE_NARROW / _CLS_RE_WIDE 为模块级常量（文件顶部编译，避免每次重复编译）
    for _f in _all_fields_to_scan:
        for _m in _CLS_RE_NARROW.finditer(_f.descriptor or ''):
            _c2 = _strip_generic(_m.group(1))
            if _c2: _referenced.add(_c2)
        for _m in _CLS_RE_WIDE.finditer(_f.generic_signature or ''):
            _c2 = _strip_generic(_m.group(1))
            if _c2: _referenced.add(_c2)
    # 扫描方法描述符（参数和返回值）
    for _method in _scan_methods:
        for _m in _CLS_RE_NARROW.finditer(_method.descriptor or ''):
            _c2 = _strip_generic(_m.group(1))
            if _c2: _referenced.add(_c2)
        for _m in _CLS_RE_WIDE.finditer(getattr(_method, 'generic_signature', '') or ''):
            _c2 = _strip_generic(_m.group(1))
            if _c2: _referenced.add(_c2)

    # 收集 invokeinterface/invokevirtual 调度分支中引用的子类型
    # （dispatch 链 downcast_ref::<SubType>() 需要 SubType 在作用域内）
    if registry:
        from ..instr.hierarchy import _get_all_subtypes_ordered as _gaso
        # JDK 命名空间判定（真前缀）：is_jdk 的 '/' 存在性检查在 jar 输入模式下把
        # lib crate 类也算作 JDK（registry 跨 crate 合并后，AssertionError 的
        # junit 子类漏进 hamcrest 文件的引用集 → 幽灵 use junit4::，E0433）。
        # 既有 .java 路径等价（旧 registry 只含 JDK 命名空间 + 无包用户类）。
        _JDK_NS = ('java/', 'javax/', 'jdk/', 'sun/', 'com/sun/', 'com/oracle/',
                   'org/xml/', 'org/w3c/', 'org/ietf/')
        _iface_refs: set[str] = set()
        for _m in _scan_methods:
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
                if _iface_bin.startswith(_JDK_NS):
                    if _sub_bin_clean.startswith(_JDK_NS):
                        _referenced.add(_sub_bin_clean)
                else:
                    _referenced.add(_sub_bin_clean)

    # 过滤：只保留实际会生成到 scratch 的类型引用，避免为 stub 方法签名里的类型
    # 生成 use 语句（那些类型不在 scratch 里，会导致 E0432）。
    if generated_classes is not None:
        _referenced = {c for c in _referenced if c in generated_classes}
    return _referenced

# prelude 里已有的泛型/newtype 名称，若 Java 类名与其重名，跳过 use 导入
# 调用方通过全路径（crate::java::...::Class）引用，不用短名
_PRELUDE_NEWTYPE_NAMES = {'JArray'}

# 简名 → 第一个导入路径 key，用于检测跨包同名冲突（E0252）
_seen_simples: dict[str, str] = {}


def _add_precise_import(full_cls: str, _prefix: str, _self_simple: str,
                        _seen_imports: set[str], _seen_simples: dict[str, str],
                        cross_imports: list[str]) -> None:
    """按 JVM binary name 添加精确 use 语句，跳过自身类型和重复项。"""
    _parts = full_cls.split('/')
    if len(_parts) < 2:
        return
    _rust_pkg = '::'.join(f'r#{p}' if p in _RUST_KEYWORDS else p for p in _parts[:-1])
    _simple = _short_cls_g(full_cls)
    if _simple == _self_simple:
        return
    _key = f"{_rust_pkg}::{_simple}"
    if _key in _seen_imports:
        return
    # 同名已被不同路径导入（跨包同名冲突），跳过以避免 E0252
    if _simple in _seen_simples and _seen_simples[_simple] != _key:
        return
    _seen_imports.add(_key)
    _seen_simples[_simple] = _key
    if _simple in _PRELUDE_NEWTYPE_NAMES:
        # 名称与 prelude newtype 冲突，跳过 use 导入；调用方应使用全路径引用
        return
    cross_imports.append(f"use {_prefix}::{_rust_pkg}::{_simple};")

def gen_cross_imports(ci, registry, jdk_crate_pkg_paths, call_chain,
                      generated_classes, conflict_map, skipped_classes,
                      user_sibling_imports, user_crate_prefix,
                      _referenced: set[str],
                      crate_prefix_resolver=None) -> list[str]:
    """Step 2：精确 cross_imports（按需逐类型导入，不使用包级 glob）。

    crate_prefix_resolver（lib crate 发射模式）：binary name → 目标 crate 名
    （'crate' / 'java_runtime' / 'hamcrest' / ...）。提供时每个引用类按其归属
    crate 定向导入——多 lib crate workspace（junit4 引 hamcrest 又引 JDK 闭包）
    的唯一正确形态；此时跳过包集合过滤分支（jdk_crate_pkg_paths / 同包 /
    消歧 / skipped），VTable 与 __base 导入同样按目标 crate 定向。
    """
    cross_imports: list[str] = []
    _prefix = user_crate_prefix or 'crate'
    _self_simple = _short_cls_g(ci.name)
    _seen_imports: set[str] = set()  # 去重键："{rust_pkg}::{simple}"

    if crate_prefix_resolver is not None:
        # lib 模式：生成集内的引用类逐个定向（各 crate 包 mod.rs 均 `pub use <mod>::*`
        # 再导出，统一 prefix::pkg::Simple 形态）。无包用户类不在此列（同 crate，
        # 由 user_sibling_imports 承载）。
        _generated = (generated_classes or set())
        for _full_cls in sorted(_referenced):
            if _full_cls == ci.name or _full_cls not in _generated:
                continue
            if len(_full_cls.split('/')) < 2:
                continue
            _add_precise_import(_full_cls, crate_prefix_resolver(_full_cls),
                                _self_simple, _seen_imports, _seen_simples,
                                cross_imports)
    elif jdk_crate_pkg_paths:
        # JDK 包（jdk_crate_pkg_paths 中的包）：按需精确导入
        # jdk_crate_pkg_paths 是 Rust 路径（java::lang），_referenced 是 JVM 路径（java/lang）
        # 转换为同一格式再比对
        _pkg_set_slash = {
            p.replace('::', '/').replace('r#', '') for p in jdk_crate_pkg_paths
        }
        for _full_cls in sorted(_referenced):
            _parts = _full_cls.split('/')
            if len(_parts) >= 2 and '/'.join(_parts[:-1]) in _pkg_set_slash:
                _add_precise_import(_full_cls, _prefix, _self_simple, _seen_imports, _seen_simples, cross_imports)

    # 同包兄弟类：按需精确导入（若自身包未在 jdk_crate_pkg_paths 中）
    if crate_prefix_resolver is None and ci.name and '/' in ci.name:
        _own_pkg = '/'.join(ci.name.split('/')[:-1])
        _own_pkg_path = '::'.join(
            f'r#{p}' if p in _RUST_KEYWORDS else p for p in ci.name.split('/')[:-1]
        )
        _existing_pkgs = set(jdk_crate_pkg_paths) if jdk_crate_pkg_paths else set()
        if _own_pkg_path not in _existing_pkgs:
            for _full_cls in sorted(_referenced):
                _parts = _full_cls.split('/')
                if len(_parts) >= 2 and '/'.join(_parts[:-1]) == _own_pkg:
                    _add_precise_import(_full_cls, _prefix, _self_simple, _seen_imports, _seen_simples, cross_imports)

    # 消歧：同一简名存在于多个包时，精确 use 覆盖（conflict_map 仍需处理）
    if crate_prefix_resolver is None and conflict_map:
        # 同简单名的类各有唯一的 Rust 类型名（short_cls 的包限定消歧），被引用者逐个导入
        for _sn, _pkgs in conflict_map.items():
            for _p in sorted(_pkgs):
                if f'{_p}/{_sn}' in _referenced:
                    _add_precise_import(f'{_p}/{_sn}', _prefix, _self_simple, _seen_imports, _seen_simples, cross_imports)

    # 被跳过包（如 jdk/）中的类型：按需精确导入
    if crate_prefix_resolver is None and skipped_classes and _referenced:
        for _full_cls in sorted(_referenced):
            _cls_parts = _full_cls.split('/')
            if len(_cls_parts) < 2:
                continue
            _rust_pkg = '::'.join(
                f'r#{p}' if p in _RUST_KEYWORDS else p for p in _cls_parts[:-1]
            )
            _simple = _short_cls_g(_full_cls)
            _sk_key = f"{_rust_pkg}::{_simple}"
            if (f"{_rust_pkg}::{_simple}" in skipped_classes
                    and _simple != _self_simple
                    and _sk_key not in _seen_imports
                    and (_simple not in _seen_simples or _seen_simples[_simple] == _sk_key)):
                cross_imports.append(f"use {_prefix}::{_rust_pkg}::{_simple};")
                _seen_imports.add(_sk_key)
                _seen_simples[_simple] = _sk_key

    # VTable trait 导入：沿超类链为每个祖先类导入 Ancestor__VTable。
    # java_class! 宏生成 impl Ancestor__VTable for Self__inner，需要该 trait 在作用域内。
    # 接口不生成 VTable trait（接口展开为持有 Object 的同名载体类型），故只处理非接口超类链。
    if not ci.is_interface and registry:
        _vtable_cur = ci.super_class
        while _vtable_cur and _vtable_cur != _OBJECT_CLASS:
            if generated_classes is None or _vtable_cur in generated_classes or '/' not in _vtable_cur:
                _vp = _vtable_cur.split('/')
                if len(_vp) >= 2:
                    _vpkg = '::'.join(f'r#{p}' if p in _RUST_KEYWORDS else p for p in _vp[:-1])
                    _vsimple = _short_cls_g(_vtable_cur)
                    _vkey = f"{_vpkg}::{_vsimple}__VTable"
                    if _vkey not in _seen_imports:
                        _seen_imports.add(_vkey)
                        _vtp = (crate_prefix_resolver(_vtable_cur)
                                if crate_prefix_resolver is not None else _prefix)
                        cross_imports.append(f"use {_vtp}::{_vpkg}::{_vsimple}__VTable;")
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
        from ..instr.member_owner import parse_method_ref as _pmr
        from ..instr.member_naming import _method_ref_binary_class as _mrbc_imp
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
                # 常量池类是接口（`Iface.super.m()` / 接口私有方法）→ 接口无 __base 自由函数，
                # 落点是展开到本类的 `Iface_super_m` 成员（见下方方法块生成）
                _cp_ci = registry.get(_mrbc_imp(_c)) if registry else None
                if _cp_ci is not None and _cp_ci.is_interface:
                    continue
                _cls_s, _mname_s, _params_s, _ret_s = _pmr(_c)
                if not _cls_s:
                    continue
                _orig_cls = _c.replace('Method ', '').replace('InterfaceMethod ', '')
                _orig_cls = _orig_cls.split('.')[0] if '.' in _orig_cls else _orig_cls
                # JVM 方法解析：常量池类未声明时，__base 函数属于最近的祖先声明者
                # （与 invoke.py 的 invokespecial 调用点同源）
                from ..instr.member_owner import _resolve_special_method_owner as _rsmo
                from ..instr.member_naming import _method_ref_descriptor as _mrd
                _orig_cls = _rsmo(_orig_cls, _mname_s, _mrd(_c), registry)
                if _orig_cls == ci.name:
                    continue   # 自模块的 base 函数由 java_class! 宏在本模块定义，
                    # 调用点直接用裸名；自导入与宏定义同名即 E0255（同类重载的
                    # invokespecial，如 BaseDescription.toJavaSyntax(C) 私有体）
                _cls_s = _short_cls_g(_orig_cls)
                _is_jdk = '/' in _orig_cls
                if _is_jdk:
                    # JDK 类：仅在父类已生成（在 generated_classes 中）时才导入 __base 函数
                    # 层次根类（无父类）整体手写，其 __base 函数由手写模块提供，同样导入
                    _chain_top = ci.name
                    while registry and _chain_top in registry and registry[_chain_top].super_class:
                        _chain_top = registry[_chain_top].super_class
                    _is_hierarchy_root = (_orig_cls == _chain_top and _orig_cls != ci.name)
                    if _orig_cls not in (generated_classes or set()) and not _is_hierarchy_root:
                        continue
                    # 解析后的声明者仍未声明该方法（祖先链超出 registry）→ 无 __base 函数可导入
                    if registry and _orig_cls in registry:
                        _anc_ci = registry[_orig_cls]
                        from ..instr.member_owner import class_inherits_default_method as _cidm
                        if (not any(am.name == _mname_s for am in _anc_ci.methods)
                                and not _cidm(_orig_cls, _mname_s, _mrd(_c), registry)):
                            continue
                    # 从 binary name（java/lang/AbstractStringBuilder）构建完整模块路径
                    _binary_parts = _orig_cls.split('/')
                    *_pkg, _simple_cls = _binary_parts
                    _base_cls_simple = _short_cls_g(_orig_cls)
                    _snake_cls = to_snake(_simple_cls)
                    _base_mod = '::'.join(
                        f'r#{p}' if p in _RUST_KEYWORDS else p
                        for p in _pkg + [_snake_cls]
                    )
                else:
                    # user class parent method
                    _base_cls_simple = _orig_cls.replace('$', '_')
                    _base_mod = to_snake(_orig_cls)
                from ..instr.member_naming import _mangle_if_overloaded as _mio
                _rust_mname_s = _mio(_cls_s, _mname_s, _c, registry)
                from ..constants import safe_ident as _sf
                _rust_mname_s = _sf(_rust_mname_s)
                _base_fn = f"{_base_cls_simple}__{_rust_mname_s}_base"
                _bkey = f"crate::{_base_mod}::{_base_fn}"
                if _bkey not in _seen_imports:
                    _seen_imports.add(_bkey)
                    # __base 自由函数随声明者 crate 定向（lib 模式：junit4 的
                    # ComparisonFailure 转发 java/lang/AssertionError 的 base 函数）
                    _bprefix = 'crate'
                    if _is_jdk and crate_prefix_resolver is not None:
                        _bprefix = crate_prefix_resolver(_orig_cls)
                    cross_imports.append(f"use {_bprefix}::{_base_mod}::{_base_fn};")

    # 用户内部类兄弟模块导入（crate::mod_name::TypeName）
    if user_sibling_imports:
        cross_imports.extend(user_sibling_imports)
    return cross_imports

def scan_used_vtable_imports(method_blocks: list[str],
                             _iface_lambda_blocks: list[str],
                             cross_imports: list[str], struct_name: str,
                             registry: dict | None, _prefix: str,
                             crate_prefix_resolver=None) -> list[str]:
# 扫描方法体中使用的 VTable trait（UFCS 调用 XxxVTable::method(...)），
# 为未导入的 VTable 类型补充 use 语句（避免 E0433）。
# 不盲目为所有类添加 __VTable（手写类如 Object/String 不一定有），
# 而是按实际生成代码中出现的名称按需导入。
    _extra_vt_imports: list[str] = []
    import re as _re_vt2
    _all_body_text2 = '\n'.join(method_blocks + _iface_lambda_blocks) if (method_blocks or _iface_lambda_blocks) else ''
    _used_vtables = set(_re_vt2.findall(r'\b(\w+__VTable)\b', _all_body_text2))
    if _used_vtables:
        _vt_simple_to_pkg: dict[str, str] = {}
        for _vt_ci_line in cross_imports:
            _vt_m = _re_vt2.match(r'use (.+)::(\w+);$', _vt_ci_line.strip())
            if _vt_m:
                _vt_simple_to_pkg[_vt_m.group(2)] = _vt_m.group(1)
        _extra_vt_imports: list[str] = []
        for _vt_name in sorted(_used_vtables):
            _already = any(_vt_name + ';' in _ci or _vt_name + '::' in _ci
                           for _ci in cross_imports)
            if _already:
                continue
            _base_name = _vt_name[:-len('__VTable')]
            if _base_name == struct_name:
                continue  # 本类的 VTable trait 由 java_class! 宏在本模块内定义，再 use 即 E0255
            _vt_pkg = _vt_simple_to_pkg.get(_base_name)
            # 若 base 类未在 cross_imports 中（如 Writer 在 StreamEncoder 的继承链里但未直接引用），
            # 从 registry 查路径
            if _vt_pkg is None and registry:
                _norm = _base_name.replace('_', '$')
                for _rk in registry:
                    _rshort = _rk.rsplit('/', 1)[-1]
                    if _short_cls_g(_rk) == _base_name or _rshort == _norm:
                        _rparts = _rk.split('/')
                        if len(_rparts) >= 2:
                            _rpkg = '::'.join(f'r#{p}' if p in _RUST_KEYWORDS else p for p in _rparts[:-1])
                            _rprefix = (crate_prefix_resolver(_rk)
                                        if crate_prefix_resolver is not None else _prefix)
                            _vt_pkg = f"{_rprefix}::{_rpkg}"
                        break
            if _vt_pkg is not None:
                _extra_vt_imports.append(f"use {_vt_pkg}::{_vt_name};")
    return _extra_vt_imports
