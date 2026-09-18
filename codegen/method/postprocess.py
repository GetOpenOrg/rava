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


def _normalize_this_clone(lines: list[str]) -> list[str]:
    """实例方法中 `this` 是 `&Self`（let this = self;），对它取值拷贝的正确形式是
    `Clone::clone(this)`（得到 owned Self）。`Clone::clone(&this)` 拷贝的是引用本身
    （得到 `&Self`），装入 Object 时借用逃逸（E0521），且宏无法识别为 wrapper 语义。
    各指令路径按通用局部变量规则生成 `Clone::clone(&x)`，在此对 `this` 统一归一化。
    构造器中 `this` 是 owned 值，不适用本 pass。"""
    return [re.sub(r'Clone::clone\(&this\)', 'Clone::clone(this)', ln) for ln in lines]


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
