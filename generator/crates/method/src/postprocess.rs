//! 方法体行级后处理（← `method/postprocess.py` 的返回值 / this 拷贝 / 装箱构造实参擦除部分；
//! 数组初始化器折叠见 [`crate::fold_array`]）。输入输出均为语句行列表（折叠块为含换行的单元素）。

use std::sync::LazyLock;

use regex::{Captures, Regex};

/// Python `str.strip()`（空白口径同 Python）
fn strip(s: &str) -> &str {
    crate::text::py_strip(s)
}

/// 末尾最后一条内容行（跳过空行与 [`crate::lines`] 独立行标记）
fn last_content(lines: &[String]) -> Option<usize> {
    lines.iter().rposition(|l| !strip(l).is_empty() && !crate::lines::is_mark(l))
}

/// 删除末尾多余的 `return;` / `return Ok(());`（void 函数）
pub fn remove_trailing_return_ok(lines: &mut Vec<String>) {
    if let Some(i) = last_content(lines) {
        if matches!(strip(&lines[i]), "return;" | "return Ok(());") {
            lines.remove(i);
        }
    }
}

/// boolean 返回方法：`Ok(1i32)` / `Ok(0i32)` → `Ok(true)` / `Ok(false)`（JVM 以 int 0/1 表示 boolean）
pub fn fix_bool_returns(lines: &mut [String]) {
    for l in lines.iter_mut() {
        if l.contains("Ok(1i32)") || l.contains("Ok(0i32)") {
            *l = l.replace("Ok(1i32)", "Ok(true)").replace("Ok(0i32)", "Ok(false)");
        }
    }
}

/// `this` 取值拷贝按绑定方式归一：构造器（owned）→ `Clone::clone(&this)`；实例方法（`&Self`）→
/// `Clone::clone(this)`
pub fn normalize_this_clone(lines: &mut [String], this_is_owned: bool) {
    let (from, to) = if this_is_owned {
        ("Clone::clone(this)", "Clone::clone(&this)")
    } else {
        ("Clone::clone(&this)", "Clone::clone(this)")
    };
    for l in lines.iter_mut() {
        if l.contains(from) {
            *l = l.replace(from, to);
        }
    }
}

static ERASED_NEW_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"((?:Object::from|Into::<Object>::into)\((?:Clone::clone\(&)?[A-Za-z_]\w*::<)([^()]*?)(>::)",
    )
    .expect("ERASED_NEW_RE")
});

fn is_w(c: Option<char>) -> bool {
    c.is_some_and(|c| c.is_alphanumeric() || c == '_')
}

/// turbofish 实参里待推断的 `_` 定为根类
/// （`(?<![\w<])_(?![\w>])|(?<=<)_(?=[,>])|(?<=, )_(?=>)`）
fn erase_infer_args(args: &str) -> String {
    let cs: Vec<char> = args.chars().collect();
    let mut out = String::with_capacity(args.len());
    for (i, &c) in cs.iter().enumerate() {
        if c != '_' {
            out.push(c);
            continue;
        }
        let p = i.checked_sub(1).map(|j| cs[j]);
        let n = cs.get(i + 1).copied();
        let alt1 = !is_w(p) && p != Some('<') && !is_w(n) && n != Some('>');
        let alt2 = p == Some('<') && matches!(n, Some(',' | '>'));
        let alt3 = i >= 2 && cs[i - 2] == ',' && cs[i - 1] == ' ' && n == Some('>');
        if alt1 || alt2 || alt3 {
            out.push_str(ir::anchors::OBJECT);
        } else {
            out.push(c);
        }
    }
    out
}

/// 构造出的泛型对象立即装入根类时，turbofish 中无上下文可推断的 `_` 按擦除语义定为根类
pub fn erase_boxed_ctor_type_args(lines: &mut [String]) {
    for l in lines.iter_mut() {
        if !ERASED_NEW_RE.is_match(l) {
            continue;
        }
        *l = ERASED_NEW_RE
            .replace_all(l, |c: &Captures| format!("{}{}{}", &c[1], erase_infer_args(&c[2]), &c[3]))
            .into_owned();
    }
}

static TEMP_LET_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*let (?:mut )?(_t\d+)(?:: [^=]+)? = .*;(?:\s*// line \d+)?$").expect("TEMP_LET_RE"));

/// 单用临时值按值移交（R1）：`let _tN = e;` 之后整个方法体只在紧随的下一条语句里以
/// `Clone::clone(&_tN)` 出现一次时，改为直接移交 `_tN`，省一对引用计数增减。
/// 下一条语句含闭包（`|`）或为循环头时不改（多次求值不能移交）；同名临时值在后文
/// 重新绑定时出现次数大于 1，保守不改。
pub fn move_single_use_temps(lines: &mut [String]) {
    for i in 0..lines.len() {
        let Some(name) = TEMP_LET_RE.captures(&lines[i]).map(|c| c[1].to_string()) else { continue };
        let Some(j) = (i + 1..lines.len()).find(|&k| !crate::lines::is_mark(&lines[k]) && !strip(&lines[k]).is_empty())
        else {
            continue;
        };
        let head = strip(&lines[j]);
        if lines[j].contains('|') || ["loop", "while ", "for ", "'"].iter().any(|k| head.starts_with(k)) {
            continue;
        }
        let clone = format!("Clone::clone(&{name})");
        if lines[j].matches(&clone).count() != 1 {
            continue;
        }
        let uses: usize = lines[i + 1..].iter().map(|l| count_word(l, &name)).sum();
        if uses == 1 {
            lines[j] = lines[j].replacen(&clone, &name, 1);
        }
    }
}

/// 标识符 `w` 在 `l` 中以整词出现的次数
fn count_word(l: &str, w: &str) -> usize {
    l.match_indices(w)
        .filter(|(at, _)| {
            !is_w(l[..*at].chars().next_back()) && !is_w(l[at + w.len()..].chars().next())
        })
        .count()
}

static RETURN_OK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\s*)return Ok\((.+)\);").expect("RETURN_OK_RE"));

/// 末尾补返回表达式：void → `Ok(())`；有返回值 → 末尾 `return Ok(e);` 改尾表达式，块尾自然结束且
/// CFG 未证明总是返回时补 `unreachable!()`
pub fn add_ok_return(lines: &mut Vec<String>, rust_ret: &str, always_returns: bool) {
    let last = last_content(lines);
    if rust_ret == "()" {
        match last {
            Some(i) if matches!(strip(&lines[i]), "Ok(())" | "return Ok(());") => {}
            _ => lines.push("    Ok(())".to_string()),
        }
        return;
    }
    let Some(i) = last else { return };
    if let Some(c) = RETURN_OK_RE.captures(&lines[i]) {
        lines[i] = format!("{}Ok({})", &c[1], &c[2]);
    } else if !strip(&lines[i]).starts_with("Ok(") && !always_returns {
        lines.push("    unreachable!()".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn erase_args() {
        assert_eq!(erase_infer_args("_, _"), "Object, Object");
        assert_eq!(erase_infer_args("_, T"), "Object, T");
        assert_eq!(erase_infer_args("Foo<_>, _"), "Foo<Object>, Object");
        assert_eq!(erase_infer_args("a_b"), "a_b");
        let mut v = vec!["let x = Object::from(HashMap::<_, _>::new()?);".to_string()];
        erase_boxed_ctor_type_args(&mut v);
        assert_eq!(v[0], "let x = Object::from(HashMap::<Object, Object>::new()?);");
    }

    #[test]
    fn single_use_temps() {
        let mut v: Vec<String> = [
            "    let _t0: String = String::valueOf_i(i)?; // line 11",
            "    X::set_s(Clone::clone(&_t0))?;",
            "    let _t1 = a()?;",
            "    f(Clone::clone(&_t1))?;",
            "    g(Clone::clone(&_t1))?;",
            "    let _t2 = a()?;",
            "    while h(Clone::clone(&_t2))? {",
            "    let _t3 = a()?;",
            "    k(Clone::clone(&_t3), Clone::clone(&_t30))?;",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        move_single_use_temps(&mut v);
        assert_eq!(v[1], "    X::set_s(_t0)?;");
        assert_eq!(v[3], "    f(Clone::clone(&_t1))?;");
        assert_eq!(v[6], "    while h(Clone::clone(&_t2))? {");
        assert_eq!(v[8], "    k(_t3, Clone::clone(&_t30))?;");
    }

    #[test]
    fn returns() {
        let mut v = vec!["    let a = 1;".to_string(), "    return Ok(a);".to_string()];
        add_ok_return(&mut v, "i32", false);
        assert_eq!(v[1], "    Ok(a)");
        let mut v = vec!["    foo();".to_string(), "    return;".to_string(), "".to_string()];
        remove_trailing_return_ok(&mut v);
        add_ok_return(&mut v, "()", false);
        assert_eq!(v, vec!["    foo();", "", "    Ok(())"]);
    }
}
