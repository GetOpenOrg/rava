"""
泛型签名（Generic Signature，JVMS §4.7.9）递归下降解析器。

自 type_map.py 拆出（该文件此前内联了 sig_parser.py，本次恢复为独立模块）。
包含类名映射表、单类型/实参表/方法签名解析与签名内类型变量代入。

依赖方向（禁止回环）：本模块 → type_map；type_args → 本模块；sig_types → 本模块。
"""

from __future__ import annotations
import re

from .constants import OBJECT_CLASS as _OBJECT_CLASS, STRING_CLASS, CLASS_CLASS
from .runtime_manifest import read_list
from . import fallback_audit
from .type_map import (_PRIMITIVE_MAP, effective_class_type_params, parse_class_type_params,
                       short_cls, _iface_full_path)


# 已知类名 → Rust 类型映射
_CLASSNAME_MAP: dict[str, str] = {
    STRING_CLASS:              'String',
    _OBJECT_CLASS:             'Object',
    # 签名解析直接擦为 Object 的接口：库知识（P-1），见
    # runtime/java_runtime/signature_erased_interfaces.txt
    **{_b: 'Object' for _b in read_list('signature_erased_interfaces.txt')},
    # 特判：Class 非泛化（类型参数纯 phantom，类级签名已在 classfile.py
    # 置空）。mapped 分支忽略 type_args，使 Ljava/lang/Class<*>; → Class，
    # 与 jvm_to_rust 的 registry 分支（裸 Class）保持一致。
    CLASS_CLASS:               'Class',
    # S-3.1：装箱类型（Integer/Long/...）不再映射为原生值 —— 签名里的
    # Ljava/lang/Integer; 是引用类型，走 registry 分支得到翻译类
    # StringBuilder / StringBuffer 不在特判表（T29 时代的 'String' 别名已删）：
    # 翻译类真实存在（java/beans/FeatureDescriptor 的 appendTo 重载族实证——
    # 别名使 (SB,String,Reference) 重载的首参渲染成 String，E0308），
    # 走 registry 分支得短名 + cross_imports 导入，与 jvm_to_rust 描述符路径一致。
}


def _parse_type_args(sig: str, i: int, class_type_params: list[str], registry=None,
                     method_bounds: 'dict | None' = None) -> tuple[list[str], int]:
    """解析 <TypeArgument*>，i 指向 '<'。返回 (类型字符串列表, '>' 之后的位置)。"""
    i += 1  # 跳过 '<'
    args: list[str] = []
    while i < len(sig) and sig[i] != '>':
        if sig[i] == '*':
            args.append('Object')
            i += 1
        elif sig[i] in ('+', '-'):
            t, i = _parse_one_type(sig, i + 1, class_type_params, registry, method_bounds)
            args.append(t)
        else:
            t, i = _parse_one_type(sig, i, class_type_params, registry, method_bounds)
            args.append(t)
    if i < len(sig) and sig[i] == '>':
        i += 1  # 跳过 '>'
    return args, i


def _extract_method_tparam_bounds(sig: str, registry=None,
                                  type_params: 'list[str] | None' = None,
                                  bound_binaries: 'dict[str, str] | None' = None) -> 'dict[str, str]':
    """从泛型参数段 '<T:Ljava/lang/Number;...>' 提取类型变量 → 上界 Rust 类型的映射。
    传入 registry 时，接口上界经 _iface_full_path 返回 'Object' 被自动过滤，
    只保留具体类上界（如 Number → "Number"，Enum<T> → "Enum<Object>"）。
    - type_params：上界内可见的类型变量（类级形参段传入类形参表，
      `E:Enum<TE;>` → "Enum<E>"；方法级不传 → 擦除为 Object）
    - bound_binaries：若提供，回填 类型变量 → 上界类 binary name
    """
    _visible = list(type_params or [])
    if not sig or sig[0] != '<':
        return {}
    bounds: dict[str, str] = {}
    i = 1  # 跳过 '<'
    depth = 1
    while i < len(sig) and depth > 0:
        c = sig[i]
        if c == '<':
            depth += 1
            i += 1
        elif c == '>':
            depth -= 1
            i += 1
        elif c.isalpha() or c == '_':
            # 读取类型变量名（到 ':'）
            j = i
            while j < len(sig) and sig[j] not in (':', '<', '>'):
                j += 1
            name = sig[i:j]
            i = j
            # 解析上界列表：:classbound(:iface1)* 格式
            first_class_bound: 'str | None' = None
            while i < len(sig) and sig[i] == ':':
                i += 1  # 跳过 ':'
                if i < len(sig) and sig[i] in ('L', '['):
                    # 传入 registry：接口类型 → _iface_full_path → 'Object' → 被过滤
                    # 具体类（Number/Enum 等）→ 正确类型（"Number"、"Enum<Object>"）
                    # A-4 批次 3+：接口载体化后接口上界解析为载体串（不再是 'Object'），
                    # 上界是 Rust trait 位置而非值位置——载体上界同样按接口上界过滤，
                    # 与载体化前的语义逐字一致（where K: Into<Bound> 只对具体类上界成立）
                    _b_start = i
                    rust_t, i = _parse_one_type(sig, i, _visible, registry)
                    from .jvm_type import carrier_type_for_ident
                    _is_carrier_bound = carrier_type_for_ident(rust_t, registry) == rust_t
                    if (first_class_bound is None and rust_t and rust_t != 'Object'
                            and not _is_carrier_bound):
                        first_class_bound = rust_t
                        if bound_binaries is not None and sig[_b_start] == 'L':
                            bound_binaries[name] = re.split(r'[<;.]', sig[_b_start + 1:], maxsplit=1)[0]
                elif i < len(sig) and sig[i] == 'T':
                    # 以另一类型变量作为上界，直接跳过
                    _, i = _parse_one_type(sig, i, [], None)
                # else: 空类上界（'::'）直接继续下一轮循环
            if name and first_class_bound:
                bounds[name] = first_class_bound
        else:
            i += 1
    return bounds


def _parse_one_type(sig: str, i: int, class_type_params: list[str], registry=None,
                    method_bounds: 'dict | None' = None) -> tuple[str, int]:
    """从 sig[i] 起解析一个类型（Generic Signature 格式），返回 (rust_type, next_i)。"""
    if i >= len(sig):
        return 'Object', i

    c = sig[i]

    if c in _PRIMITIVE_MAP:
        return _PRIMITIVE_MAP[c], i + 1

    if c == 'T':
        # TypeVariable
        try:
            end = sig.index(';', i + 1)
        except ValueError:
            return 'Object', len(sig)
        name = sig[i + 1:end]
        if name in class_type_params:
            return name, end + 1
        if method_bounds and name in method_bounds:
            # 方法级类型变量 → 使用其上界（如 T extends Number → Number）
            return method_bounds[name], end + 1
        return 'Object', end + 1

    if c == '[':
        # 数组 → JArray<elem>
        elem_type, next_i = _parse_one_type(sig, i + 1, class_type_params, registry, method_bounds)
        return f'JArray<{elem_type}>', next_i

    if c == '+' or c == '-':
        # 上下界通配符 — 取内部类型
        return _parse_one_type(sig, i + 1, class_type_params, registry, method_bounds)

    if c == '*':
        # 无界通配符
        return 'Object', i + 1

    if c == 'L':
        # ClassTypeSig: L<classname>(<TypeArgs>)?;
        j = i + 1
        while j < len(sig) and sig[j] not in ('<', ';', '.'):
            j += 1
        class_name = sig[i + 1:j]

        type_args: list[str] = []
        has_type_args = j < len(sig) and sig[j] == '<'
        if has_type_args:
            type_args, j = _parse_type_args(sig, j, class_type_params, registry, method_bounds)

        # ClassTypeSigSuffix（`Outer<TE;>.Inner<TX;>`）：类型是内部类 Outer$Inner，
        # 不是外部类。实参按内部类的有效形参选取：自带形参 → 本段实参；
        # 形参继承自外部类（非静态内部类）→ 外层段实参。
        if j < len(sig) and sig[j] == '.':
            outer_ci = registry.get(class_name) if registry else None
            while j < len(sig) and sig[j] == '.':
                k = j + 1
                while k < len(sig) and sig[k] not in ('<', ';', '.'):
                    k += 1
                class_name = class_name + '$' + sig[j + 1:k]
                j = k
                seg_args: list[str] = []
                if j < len(sig) and sig[j] == '<':
                    seg_args, j = _parse_type_args(sig, j, class_type_params, registry, method_bounds)
                inner_ci = registry.get(class_name) if registry else None
                type_args = (_inner_class_type_args(outer_ci, type_args, inner_ci, seg_args,
                                                    class_type_params, registry)
                             if inner_ci is not None else seg_args)
                outer_ci = inner_ci
            has_type_args = bool(type_args)

        elif has_type_args and registry and class_name in registry:
            # 无外层段的带实参类型（局部类 `LOuter$1Local<TX;>;`）：继承自外围作用域的形参
            # 不出现在签名里，按当前作用域补齐
            type_args = _inner_class_type_args(None, [], registry[class_name], type_args,
                                               class_type_params, registry)

        # 签名里不带实参的泛型类：
        #   - 形参全部继承自外围作用域的内部 / 局部类（javac 对局部类只写 `LOuter$1Var;`），
        #     且当前上下文正处于同一外围作用域（这些类型变量可见）→ 实参就是这些类型变量
        #   - 其余为 raw type → 按擦除语义全部取 Object
        if not has_type_args and registry and class_name in registry \
                and not registry[class_name].is_interface and class_name not in _CLASSNAME_MAP:
            raw_ci = registry[class_name]
            raw_eff = list(effective_class_type_params(raw_ci, registry) or [])
            if raw_eff:
                raw_own = (parse_class_type_params(raw_ci.generic_signature)
                           if raw_ci.generic_signature else [])
                if not raw_own and all(p in (class_type_params or []) for p in raw_eff):
                    type_args = raw_eff
                else:
                    type_args = ['Object'] * len(raw_eff)
                has_type_args = True

        # 跳过结尾 ';'
        if j < len(sig) and sig[j] == ';':
            j += 1

        # 映射类名到 Rust 类型
        # A-4 批次 6：已铺设载体化的接口（本表仅 CharSequence 一员）优先发射载体
        # `I<Object, ..>`——判定单一来源 jvm_type.carrier_type（惰性 import 防回环），
        # 未启用 / 闭包外回落本表的 Object 擦除（批次 3-5 五擦除点同口径）。
        if registry is not None and registry.get(class_name) is not None \
                and getattr(registry[class_name], 'is_interface', False):
            from .jvm_type import carrier_type
            _carrier = carrier_type(class_name, registry)
            if _carrier is not None:
                return _carrier, j
        mapped = _CLASSNAME_MAP.get(class_name)
        if mapped is not None:
            rust_type = mapped
        elif registry is not None and class_name not in registry:
            # 闭包外类型：没有生成模块，其 use 会被 import 过滤器拒绝（防 E0432），
            # 渲染简名必然落成 E0425——或与同简名的其他类静默错配（JDK 25 语料：
            # java/lang/annotation/Annotation 撞上 java/lang/classfile/Annotation，
            # getAnnotationsByType 的 [TA; 上界替换渲染出无 import 的裸名）。
            # 与 jvm_to_rust 对 registry 外类型的 fallback 同一哲学：签名位置
            # 一律退化为 Object。这类类型只出现在从未被调用的 panic stub 签名里
            # （调用链上的方法其描述符类型经 T88 通道必然入闭包），无可观察影响。
            rust_type = 'Object'
        else:
            short = short_cls(class_name)
            # Arch-1：接口 = Object 类型别名，用全路径避免与 Rust prelude 冲突。
            # A-4 批次 3+：已铺设载体化的接口发射擦除载体 `I<Object, ..>`
            # （判定单一来源 jvm_type.carrier_type，惰性 import 防回环）
            if registry and class_name in registry and registry[class_name].is_interface:
                from .jvm_type import carrier_type
                _carrier = carrier_type(class_name, registry)
                if _carrier is not None:
                    rust_type = _carrier
                else:
                    rust_type = _iface_full_path(class_name)
            elif has_type_args:
                rust_type = f"{short}<{', '.join(type_args)}>"
            else:
                rust_type = short

        return rust_type, j

    # 未知 — 前进一步
    return 'Object', i + 1


def parse_field_type(sig: str, class_type_params: list[str], registry=None) -> str:
    """从字段级 Signature 解析 Rust 类型字符串。"""
    if not sig:
        return ''
    try:
        rust_type, _ = _parse_one_type(sig, 0, class_type_params, registry)
        return rust_type
    except (ValueError, IndexError):
        # B 组收窄（fallback-audit 方案 §4.1）：签名残缺是 ValueError/IndexError
        # 形态；AttributeError/NameError/TypeError 等代码 bug 不再吞、穿透硬失败
        fallback_audit.record('sig-parse-field')
        return ''


def _inner_class_type_args(outer_ci, outer_args: list[str], inner_ci, seg_args: list[str],
                           scope_type_params, registry) -> list[str]:
    """`Outer<A..>.Inner<B..>` 中 Inner 的有效类型实参：继承自外围作用域的形参取外层实参
    （外层无对应实参：当前作用域可见的同名类型变量原样传递，否则按擦除取 Object）+ 本段实参。"""
    own = parse_class_type_params(inner_ci.generic_signature) if inner_ci.generic_signature else []
    eff = effective_class_type_params(inner_ci, registry)
    inherited = eff[:len(eff) - len(own)] if own else eff
    own_args = seg_args if len(seg_args) == len(own) else ['Object'] * len(own)
    outer_eff = effective_class_type_params(outer_ci, registry) if outer_ci is not None else []
    outer_map = dict(zip(outer_eff, outer_args)) if len(outer_eff) == len(outer_args) else {}
    scope = scope_type_params or ()
    return [outer_map.get(p, p if p in scope else 'Object') for p in inherited] + own_args


def parse_method_param_types(
    sig: str,
    class_type_params: list[str],
    registry=None,
    is_static: bool = True,
) -> tuple[list[str], str]:
    """从方法 Signature 中解析参数类型和返回类型（Rust 类型字符串）。

    is_static：方法是否为 static。实例方法声明的方法级类型形参遮蔽同名的类级
      形参（Java 作用域规则：`class P<S> { <S extends Sink> S wrap(S s) }` 里的 S
      是方法级变量，与类级 S 无关）→ 解析时从类级形参表剔除，按方法级变量
      （上界 / Object）处理，与父类/接口侧同一方法的擦除形态一致。
      static 方法不能引用类级形参，其方法级 `<K,V>` 由所在 impl 块的同名
      形参承载（impl<K,V> X<K,V> { fn f(..: K) }），保留同名解析。
      定义侧与所有调用侧必须传入同一取值。

    class_type_params：类级类型参数名（如 ['E'] 或 ['K', 'V']）
    registry：类注册表，用于 Arch-1 接口擦除（接口类型参数 → Object）

    示例（class_type_params=['E']）：
      '(TE;)Z'    → (['E'], 'bool')
      '(I)TE;'    → (['i32'], 'E')
      '(TE;I)V'   → (['E', 'i32'], '()')

    遇到解析错误时返回 ([], '')。
    """
    if not sig:
        return [], ''

    try:
        i = 0

        # 解析方法级类型参数 <T:Ljava/lang/Number;...>，提取上界用于参数类型解析
        method_bounds: dict[str, str] = {}
        if i < len(sig) and sig[i] == '<':
            method_bounds = _extract_method_tparam_bounds(sig[i:], registry)
            if not is_static:
                _shadowed = set(parse_class_type_params(sig[i:]))
                if _shadowed & set(class_type_params):
                    class_type_params = [p for p in class_type_params if p not in _shadowed]
            depth = 1
            i += 1
            while i < len(sig) and depth > 0:
                if sig[i] == '<':
                    depth += 1
                elif sig[i] == '>':
                    depth -= 1
                i += 1

        # 期望 '('
        if i >= len(sig) or sig[i] != '(':
            return [], ''
        i += 1  # 跳过 '('

        # 解析参数类型（方法级类型变量使用上界替代 Object）
        param_types: list[str] = []
        while i < len(sig) and sig[i] != ')':
            rust_type, i = _parse_one_type(sig, i, class_type_params, registry, method_bounds or None)
            param_types.append(rust_type)

        if i < len(sig) and sig[i] == ')':
            i += 1  # 跳过 ')'

        # 解析返回类型（忽略 ThrowsSignature ^...）
        if i < len(sig) and sig[i] != '^':
            ret_type, _ = _parse_one_type(sig, i, class_type_params, registry, method_bounds or None)
        else:
            ret_type = '()'

        return param_types, ret_type

    except (ValueError, IndexError):
        # B 组收窄（fallback-audit 方案 §4.1）：同 parse_field_type，只兜签名
        # 残缺形态，代码 bug 穿透
        fallback_audit.record('sig-parse-method')
        return [], ''


def _substitute_type_sig(sig: str, i: int, out: list, mapping: dict) -> int:
    """把 sig[i:] 处的一个类型签名写入 out（类型变量按 mapping 代入），返回其后的位置。"""
    if i >= len(sig):
        return i
    c = sig[i]
    if c == '[' or c in ('+', '-'):
        out.append(c)
        return _substitute_type_sig(sig, i + 1, out, mapping)
    if c == 'T':
        end = sig.find(';', i)
        if end < 0:
            out.append(sig[i:])
            return len(sig)
        out.append(mapping.get(sig[i + 1:end], sig[i:end + 1]))
        return end + 1
    if c != 'L':
        out.append(c)
        return i + 1
    while i < len(sig):
        ch = sig[i]
        if ch == ';':
            out.append(ch)
            return i + 1
        if ch == '<':
            out.append(ch)
            i += 1
            while i < len(sig) and sig[i] != '>':
                i = _substitute_type_sig(sig, i, out, mapping)
            continue
        out.append(ch)
        i += 1
    return i


def substitute_signature_type_vars(sig: str, mapping: dict) -> str:
    """JVM 泛型签名（字段 / 方法 / 局部变量）里的类型变量按 mapping（名字 → 类型签名）代入。
    方法自身声明的类型形参遮蔽同名的外层类型变量。"""
    if not sig or not mapping:
        return sig
    out: list[str] = []
    i = 0
    if sig[0] == '<':
        mapping = {k: v for k, v in mapping.items() if k not in parse_class_type_params(sig)}
        if not mapping:
            return sig
        out.append('<')
        i = 1
        while i < len(sig) and sig[i] != '>':
            colon = sig.index(':', i)
            out.append(sig[i:colon])
            i = colon
            while i < len(sig) and sig[i] == ':':
                out.append(':')
                i += 1
                if i < len(sig) and sig[i] in 'LT[':
                    i = _substitute_type_sig(sig, i, out, mapping)
        out.append('>')
        i += 1
    while i < len(sig):
        if sig[i] in '()^':
            out.append(sig[i])
            i += 1
        else:
            i = _substitute_type_sig(sig, i, out, mapping)
    return ''.join(out)
