//! 局部变量命名（← `stack._safe_name`）。

use ty::ident::safe_ident;

/// 局部变量名安全化：在 `safe_ident` 基础上，大写开头的名（catch 变量 `IOException` 等）
/// camelCase 化，避免遮蔽 Rust unit struct（E0530）；全大写常量风格名（`ARG_BASE`）原样保留。
pub fn safe_name(name: &str) -> String {
    let s = safe_ident(name);
    let Some(first) = s.chars().next() else {
        return s;
    };
    if !first.is_uppercase() {
        return s;
    }
    if s == s.to_uppercase() && s.chars().any(char::is_alphabetic) {
        return s;
    }
    // 开头的连续大写字母段（IOException 中的 IOE）
    let run_len = s.chars().take_while(|c| c.is_uppercase()).count();
    let run: Vec<char> = s.chars().take(run_len).collect();
    let rest: String = s.chars().skip(run_len).collect();
    let mut out = String::with_capacity(s.len());
    if run_len > 1 && !rest.is_empty() {
        // 缩写词前缀：除最后一个大写字母外全部小写，最后一个保留作下一词首字母
        out.extend(run[..run_len - 1].iter().flat_map(|c| c.to_lowercase()));
        out.push(run[run_len - 1]);
    } else {
        out.extend(run.iter().flat_map(|c| c.to_lowercase()));
    }
    out.push_str(&rest);
    // 小写化撞上接收者名（ICU Trie2 的局部变量 `This`）：`this` 在生成代码中专指接收者
    if out == "this" {
        out.push('_');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::safe_name;

    #[test]
    fn camel_case_rules() {
        assert_eq!(safe_name("IOException"), "ioException");
        assert_eq!(safe_name("Exception"), "exception");
        assert_eq!(safe_name("ARG_BASE"), "ARG_BASE");
        assert_eq!(safe_name("value"), "value");
        assert_eq!(safe_name("type"), "type_");
        assert_eq!(safe_name("X"), "X");
        assert_eq!(safe_name("a$b"), "a_b");
    }
}
