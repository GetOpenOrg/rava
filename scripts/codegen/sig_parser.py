"""
JVM Generic Signature 解析器（JVMS §4.7.9）。

实现两个公开函数：
  parse_class_type_params  — 从类级 Signature 提取类型参数名列表
  parse_method_param_types — 从方法 Signature 解析参数/返回 Rust 类型
"""

# 已知类名 → Rust 类型映射
_CLASSNAME_MAP: dict[str, str] = {
    'java/lang/String':        'String',
    'java/lang/Object':        'JvmObject',
    'java/lang/CharSequence':  'JvmObject',
    'java/lang/Integer':       'i32',
    'java/lang/Long':          'i64',
    'java/lang/Double':        'f64',
    'java/lang/Float':         'f32',
    'java/lang/Boolean':       'bool',
    'java/lang/StringBuilder': 'String',
    'java/lang/StringBuffer':  'String',
}

# 基本类型映射
_PRIMITIVE_MAP: dict[str, str] = {
    'V': '()', 'I': 'i32', 'J': 'i64', 'F': 'f32', 'D': 'f64',
    'Z': 'bool', 'B': 'i8', 'C': 'u16', 'S': 'i16',
}


# ── 内部辅助 ──────────────────────────────────────────────────────

def _skip_field_type_sig(sig: str, i: int) -> int:
    """跳过一个 FieldTypeSig，返回跳过后的位置。

    处理基本类型、类类型（含嵌套 TypeArguments）、TypeVar、数组、通配符。
    遇到未知字符则前进一步。
    """
    if i >= len(sig):
        return i
    c = sig[i]
    if c in _PRIMITIVE_MAP:
        return i + 1
    if c == 'T':
        # TypeVariable: T<name>;
        try:
            end = sig.index(';', i + 1)
            return end + 1
        except ValueError:
            return len(sig)
    if c == 'L':
        # ClassTypeSig: L…<…>…; — 追踪 <> 深度，直到 depth==0 的 ';'
        j = i + 1
        depth = 0
        while j < len(sig):
            ch = sig[j]
            if ch == '<':
                depth += 1
            elif ch == '>':
                depth -= 1
            elif ch == ';' and depth == 0:
                return j + 1
            j += 1
        return j
    if c == '[':
        return _skip_field_type_sig(sig, i + 1)
    if c in ('+', '-'):
        return _skip_field_type_sig(sig, i + 1)
    if c == '*':
        return i + 1
    return i + 1  # 未知，前进一步


def _parse_one_type(sig: str, i: int, class_type_params: list[str]) -> tuple[str, int]:
    """从 sig[i] 起解析一个类型（Generic Signature 格式），返回 (rust_type, next_i)。

    - 基本类型 → 对应 Rust 基本类型
    - TypeVariable T<name>; 且 name 在 class_type_params → 类型变量名
    - ClassTypeSig（不带泛型参数）→ 按 _CLASSNAME_MAP 映射；未知则取短类名
    - ClassTypeSig（带泛型参数）→ 'JvmObject'（暂时简化）
    - 数组 → 'JvmObject'（暂时简化）
    - 通配符 +/- → 取内部类型；* → 'JvmObject'
    """
    if i >= len(sig):
        return 'JvmObject', i

    c = sig[i]

    if c in _PRIMITIVE_MAP:
        return _PRIMITIVE_MAP[c], i + 1

    if c == 'T':
        # TypeVariable
        try:
            end = sig.index(';', i + 1)
        except ValueError:
            return 'JvmObject', len(sig)
        name = sig[i + 1:end]
        rust_type = name if name in class_type_params else 'JvmObject'
        return rust_type, end + 1

    if c == '[':
        # Array — 跳过整个 component，返回 JvmObject
        _, next_i = _parse_one_type(sig, i + 1, class_type_params)
        return 'JvmObject', next_i

    if c == '+' or c == '-':
        # 上下界通配符 — 取内部类型
        return _parse_one_type(sig, i + 1, class_type_params)

    if c == '*':
        # 无界通配符
        return 'JvmObject', i + 1

    if c == 'L':
        # ClassTypeSig: L<classname>(<TypeArgs>)?;
        j = i + 1
        # 找类名结束：'<', ';', '.' 都可能终结类名
        while j < len(sig) and sig[j] not in ('<', ';', '.'):
            j += 1
        class_name = sig[i + 1:j]

        has_type_args = j < len(sig) and sig[j] == '<'

        # 跳过 TypeArguments（可能嵌套）
        if has_type_args:
            depth = 1
            j += 1
            while j < len(sig) and depth > 0:
                if sig[j] == '<':
                    depth += 1
                elif sig[j] == '>':
                    depth -= 1
                j += 1

        # 跳过 ClassTypeSigSuffix（.InnerClass…）
        while j < len(sig) and sig[j] == '.':
            j += 1
            while j < len(sig) and sig[j] not in ('<', ';', '.'):
                j += 1
            if j < len(sig) and sig[j] == '<':
                depth = 1
                j += 1
                while j < len(sig) and depth > 0:
                    if sig[j] == '<':
                        depth += 1
                    elif sig[j] == '>':
                        depth -= 1
                    j += 1

        # 跳过结尾 ';'
        if j < len(sig) and sig[j] == ';':
            j += 1

        if has_type_args:
            # 带泛型参数的类类型 → 暂时简化为 JvmObject
            rust_type = 'JvmObject'
        else:
            rust_type = _CLASSNAME_MAP.get(
                class_name,
                class_name.rsplit('/', 1)[-1]  # 取短类名
            )
        return rust_type, j

    # 未知 — 前进一步
    return 'JvmObject', i + 1


# ── 公开 API ──────────────────────────────────────────────────────

def parse_class_type_params(sig: str) -> list[str]:
    """从类级 Signature 中提取类型参数名列表。

    示例：
      '<E:Ljava/lang/Object;>...'     → ['E']
      '<K:Ljava/lang/Object;V:...>'   → ['K', 'V']
      'Ljava/lang/Object;'            → []
      ''                              → []
    """
    if not sig or sig[0] != '<':
        return []

    params: list[str] = []
    i = 1  # 跳过开头的 '<'

    try:
        while i < len(sig) and sig[i] != '>':
            # 读取类型参数名（直到 ':' 或 '>'）
            j = i
            while j < len(sig) and sig[j] != ':' and sig[j] != '>':
                j += 1
            if j >= len(sig) or sig[j] == '>':
                break
            name = sig[i:j]
            if name:
                params.append(name)
            i = j + 1  # 跳过 ':'

            # 跳过 ClassBound（可为空：下一个字符是 ':' 或 '>'）
            if i < len(sig) and sig[i] not in (':', '>'):
                i = _skip_field_type_sig(sig, i)

            # 跳过所有 InterfaceBound（每个以 ':' 开头）
            while i < len(sig) and sig[i] == ':':
                i += 1  # 跳过 ':'
                if i < len(sig) and sig[i] not in (':', '>'):
                    i = _skip_field_type_sig(sig, i)
    except Exception:
        pass  # 解析失败时返回已收集的部分

    return params


def parse_method_param_types(
    sig: str,
    class_type_params: list[str],
) -> tuple[list[str], str]:
    """从方法 Signature 中解析参数类型和返回类型（Rust 类型字符串）。

    class_type_params：类级类型参数名（如 ['E'] 或 ['K', 'V']）

    示例（class_type_params=['E']）：
      '(TE;)Z'    → (['E'], 'bool')
      '(I)TE;'    → (['i32'], 'E')
      '(TE;I)V'   → (['E', 'i32'], '()')
      '(Ljava/lang/String;)Ljava/lang/Object;' → (['String'], 'JvmObject')

    遇到解析错误时返回 ([], '')。
    """
    if not sig:
        return [], ''

    try:
        i = 0

        # 跳过方法级类型参数 <T:...> —— 我们不处理方法级泛型
        if i < len(sig) and sig[i] == '<':
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

        # 解析参数类型
        param_types: list[str] = []
        while i < len(sig) and sig[i] != ')':
            rust_type, i = _parse_one_type(sig, i, class_type_params)
            param_types.append(rust_type)

        if i < len(sig) and sig[i] == ')':
            i += 1  # 跳过 ')'

        # 解析返回类型（忽略 ThrowsSignature ^...）
        if i < len(sig) and sig[i] != '^':
            ret_type, _ = _parse_one_type(sig, i, class_type_params)
        else:
            ret_type = '()'

        return param_types, ret_type

    except Exception:
        return [], ''
