"""
方法体后处理 pass：纯函数。
"""

import re


def _remove_trailing_return_ok(lines: list[str]) -> list[str]:
    """删除函数末尾多余的 return Ok(()); 语句（void 函数）。"""
    result = list(lines)
    for i in reversed(range(len(result))):
        stripped = result[i].strip()
        if stripped:
            if stripped in ('return;', 'return Ok(());'):
                result.pop(i)
            break
    return result


def _fix_bool_returns(lines: list[str]) -> list[str]:
    """将 bool 返回方法中的 Ok(1i32)/Ok(0i32) 转换为 Ok(true)/Ok(false)。
    JVM 中 boolean 用 int 0/1 表示，直接 ireturn 时会生成 Ok(1i32)。"""
    result = []
    for line in lines:
        line = line.replace('Ok(1i32)', 'Ok(true)')
        line = line.replace('Ok(0i32)', 'Ok(false)')
        # 也修正裸 return Ok(1i32)
        result.append(line)
    return result


def _normalize_this_clone(lines: list[str], this_is_owned: bool = False) -> list[str]:
    """对 `this` 取值拷贝的形式按 `this` 的绑定方式统一归一化。

    实例方法中 `this` 是 `&Self`（let this = self;），正确形式是 `Clone::clone(this)`
    （得到 owned Self）。`Clone::clone(&this)` 拷贝的是引用本身（得到 `&Self`），
    装入 Object 时借用逃逸（E0521），且宏无法识别为 wrapper 语义。
    构造器中 `this` 是 owned 值（let mut this = Self::default();），正确形式是
    `Clone::clone(&this)`；`Clone::clone(this)` 把值当引用传（E0308）。
    各指令路径生成的两种写法混杂，在此按方法种类统一。"""
    if this_is_owned:
        return [re.sub(r'Clone::clone\(this\)', 'Clone::clone(&this)', ln) for ln in lines]
    return [re.sub(r'Clone::clone\(&this\)', 'Clone::clone(this)', ln) for ln in lines]


_ERASED_NEW_RE = re.compile(
    r'((?:Object::from_any|Object::from|Into::<Object>::into)\((?:Clone::clone\(&)?[A-Za-z_]\w*::<)([^()]*?)(>::)')


def _erase_boxed_ctor_type_args(lines: list[str]) -> list[str]:
    """构造出的泛型对象被立即装入 Object（Java 侧赋给接口/Object 类型）时，
    turbofish 中的 `_` 没有任何上下文可供 Rust 推断（E0283）。
    此处类型信息已被擦除，按 Java 擦除语义把未确定的类型实参定为 Object；
    已由实参确定的类型实参（`<_, T>` 中的 T）保持不变。"""
    def _erase(m: re.Match) -> str:
        return m.group(1) + re.sub(r'(?<![\w<])_(?![\w>])|(?<=<)_(?=[,>])|(?<=, )_(?=>)',
                                   'Object', m.group(2)) + m.group(3)
    return [_ERASED_NEW_RE.sub(_erase, ln) for ln in lines]


def _add_ok_return(lines: list[str], rust_ret: str, always_returns: bool = False) -> list[str]:
    """在方法末尾添加正确的 Ok(?) 返回表达式。

    always_returns: 由 CFG 分析得出，若 True 表示函数所有路径都经过 return/throw，
    或函数是发散函数（loop {} 无 break）。此时末尾不添加 unreachable!()。
    """
    result = list(lines)
    if rust_ret == '()':
        # void 方法：末尾加 Ok(())
        # 先检查末尾是否已有 Ok(())
        for i in reversed(range(len(result))):
            stripped = result[i].strip()
            if stripped:
                if stripped not in ('Ok(())', 'return Ok(());'):
                    result.append('    Ok(())')
                break
        else:
            result.append('    Ok(())')
    else:
        # 有返回值：将末尾的 return Ok(e); 转换为 Ok(e)（尾表达式形式）
        for i in reversed(range(len(result))):
            stripped = result[i].strip()
            if stripped:
                m = re.match(r'(\s*)return Ok\((.+)\);', result[i])
                if m:
                    result[i] = f"{m.group(1)}Ok({m.group(2)})"
                elif not stripped.startswith('Ok('):
                    # 如果最后一行不是 Ok(...) 也不是 return Ok(...)，
                    # 说明函数在块尾自然结束。
                    # CFG 分析若确认函数总是 return/throw（含发散 loop {} 无 break），
                    # 则 Rust 类型检查自动满足，无需 unreachable!()。
                    if not always_returns:
                        result.append('    unreachable!()')
                break
    return result


def _indent(block: str, n: int = 4) -> str:
    pad = ' ' * n
    return '\n'.join(pad + ln if ln.strip() else '' for ln in block.split('\n'))


# ── 数组初始化器折叠（L-1 L1-d）────────────────────────────────────────────
# javac 的数组初始化器 `new T[]{a, b, c}` 编译为 anewarray + (dup; 下标; 值; aastore)×n，
# 逐元素翻译为 `let mut _arrK = JArray::try_new(n)?;` + n 条 `_arrK.set(i, v)?;`。
# 折叠为单条 `JArray::from(vec![a, b, c])`——与 Java 源的初始化器同形，且大幅缩减
# 资源束 getContents（数千元素字面量表）的语句数。
#
# 语义保持条件（不满足即原样保留）：
#   - 声明长度为字面量 n，其后恰有下标 0..n-1 依序的 n 条 set；
#   - 值均为纯表达式（字面量 / `Clone::clone(&字面量或变量)` / 其 `Object::from` 包装），
#     无副作用、不读数组元素——求值时刻后移不改变结果；
#   - 声明与最后一条 set 之间只允许其它 `_arr` 临时数组的声明 / set / 已折叠块
#     （javac 嵌套初始化器的交错形态），不出现对本数组的其它引用、也无普通赋值；
#   - 折叠结果落在最后一条 set 的位置（其引用的内层数组此时均已构造完毕）。
_ARR_TMP = r'_arr\d+'
_ARR_DECL_RE = re.compile(
    r'^(\s*)let mut (' + _ARR_TMP + r'): JArray<(.+)> = JArray::<(.+)>::try_new\((\d+)i32\)\?;$')
_ARR_SET_RE = re.compile(r'^(\s*)(' + _ARR_TMP + r')\.set\((\d+)i32, (.*)\)\?;$')
_STR_LIT = r'String::from\("(?:[^"\\]|\\.)*"\)'
_PURE_ATOM = (r'(?:' + _STR_LIT + r'|[A-Za-z_]\w*'
              r'|-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?(?:i8|i16|i32|i64|u16|f32|f64)'
              r'|true|false)')
_PURE_VALUE_RE = re.compile(
    r'^(?:' + _PURE_ATOM + r'|Clone::clone\(&' + _PURE_ATOM + r'\)'
    r'|Object::from\(Clone::clone\(&' + _PURE_ATOM + r'\)\))$')
_FOLDED_MARK = 'JArray::from(vec!['


# 元素全为字符串字面量时的紧凑形态（静态切片，见 runtime array.rs from_strs）
_STR_ELEM_RE = re.compile(r'^(?:Clone::clone\(&)?String::from\(("(?:[^"\\]|\\.)*")\)\)?$')
_OBJ_STR_ELEM_RE = re.compile(
    r'^Object::from\((?:Clone::clone\(&)?String::from\(("(?:[^"\\]|\\.)*")\)\)?\)$')
_STRS_CTORS = {'String': ('from_strs', _STR_ELEM_RE), 'Object': ('objects_from_strs', _OBJ_STR_ELEM_RE)}


def _folded_literal(ind: str, var: str, elem_t: str, values: list[str]) -> str:
    """折叠结果：字符串字面量表 → `JArray::from_strs(&[..])`；其余 → `JArray::from(vec![..])`。"""
    ctor = _STRS_CTORS.get(elem_t)
    if ctor is not None:
        lits = [ctor[1].match(v) for v in values]
        if all(lits):
            body = ', '.join(m.group(1) for m in lits)
            return f'{ind}let mut {var}: JArray<{elem_t}> = JArray::{ctor[0]}(&[{body}]);'
    return (f'{ind}let mut {var}: JArray<{elem_t}> = {_FOLDED_MARK}\n'
            + '\n'.join(f'{ind}    {v},' for v in values)
            + f'\n{ind}]);')


def _is_arr_intermediate(stmt: str) -> bool:
    if '\n' in stmt or 'JArray::from_strs(' in stmt or 'JArray::objects_from_strs(' in stmt:
        # 已折叠块（多行 vec! 形态或单行字符串切片形态）
        return bool(re.match(r'^\s*let mut ' + _ARR_TMP + r':', stmt)) and (
            _FOLDED_MARK in stmt or '_from_strs(' in stmt or 'from_strs(' in stmt)
    if _ARR_DECL_RE.match(stmt):
        return True
    m = _ARR_SET_RE.match(stmt)
    return bool(m) and bool(_PURE_VALUE_RE.match(m.group(4)))


def _fold_array_literals(lines: list[str]) -> list[str]:
    """数组初始化器折叠（条件见上）。输入输出均为语句列表（折叠块为含换行的单元素）。"""
    stmts = list(lines)
    decls = [i for i, s in enumerate(stmts) if _ARR_DECL_RE.match(s)]
    for i in reversed(decls):              # 内层（后声明）先折叠
        m = _ARR_DECL_RE.match(stmts[i])
        if not m or m.group(3) != m.group(4):
            continue
        ind, var, elem_t, n = m.group(1), m.group(2), m.group(3), int(m.group(5))
        if n == 0:
            continue
        ref_re = re.compile(r'\b' + re.escape(var) + r'\b')
        values: list[str] = []
        set_pos: list[int] = []
        j = i + 1
        ok = True
        while j < len(stmts) and len(values) < n:
            s = stmts[j]
            sm = _ARR_SET_RE.match(s)
            if sm and sm.group(2) == var:
                if int(sm.group(3)) != len(values) or not _PURE_VALUE_RE.match(sm.group(4)) \
                        or ref_re.search(sm.group(4)):
                    ok = False
                    break
                values.append(sm.group(4))
                set_pos.append(j)
            elif ref_re.search(s) or not _is_arr_intermediate(s):
                ok = False
                break
            j += 1
        if not ok or len(values) != n:
            continue
        folded = _folded_literal(ind, var, elem_t, values)
        last = set_pos[-1]
        drop = set(set_pos) | {i}
        stmts = [s for k, s in enumerate(stmts[:last + 1]) if k not in drop] + [folded] + stmts[last + 1:]
        # 下标已变动：后续（更外层 / 更早声明）的 decls 位置在 i 之前，不受影响
    return stmts
