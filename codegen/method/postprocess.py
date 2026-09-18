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
