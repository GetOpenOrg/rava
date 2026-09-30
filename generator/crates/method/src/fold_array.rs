//! 数组初始化器折叠（← `postprocess._fold_array_literals`）：javac 把 `new T[]{a, b, c}` 编译为
//! anewarray + (dup; 下标; 值; aastore)×n，逐元素翻译为 `let mut _arrK = JArray::try_new(n)?;` +
//! n 条 `_arrK.set(i, v)?;`；折叠为单条 `JArray::from(vec![a, b, c])`。
//!
//! 语义保持条件（不满足即原样保留）：
//! - 声明长度为字面量 n，其后恰有下标 0..n-1 依序的 n 条 set；
//! - 值均为纯表达式（字面量 / 变量 / 其 clone 与 `Object::from` 包装，或本类静态字段 getter）；
//! - 声明与最后一条 set 之间只允许其它 `_arr` 临时数组的声明 / set / 已折叠块，
//!   不出现对本数组的其它引用；
//! - 折叠结果落在最后一条 set 的位置。

use std::collections::{BTreeMap, BTreeSet};
use std::sync::LazyLock;

use ir::anchors::{OBJECT, STRING};
use regex::Regex;

const ARR_TMP: &str = r"_arr\d+";
const STR_LIT: &str = r#"String::from\("(?:[^"\\]|\\.)*"\)"#;
const FOLDED_MARK: &str = "JArray::from(vec![";
const DEDUP_MARK: &str = "const __IDX: [";
/// 去重折叠阈值：元素数 ≥ 32 且不同值不超过一半
const DEDUP_MIN_LEN: usize = 32;

fn re(s: &str) -> Regex {
    Regex::new(s).expect("fold_array 正则")
}

static ARR_DECL_RE: LazyLock<Regex> = LazyLock::new(|| {
    re(&format!(r"^(\s*)let mut ({ARR_TMP}): JArray<(.+)> = JArray::<(.+)>::try_new\((\d+)i32\)\?;$"))
});
static ARR_SET_RE: LazyLock<Regex> = LazyLock::new(|| re(&format!(r"^(\s*)({ARR_TMP})\.set\((\d+)i32, (.*)\)\?;$")));
static FOLDED_DECL_RE: LazyLock<Regex> = LazyLock::new(|| re(&format!(r"^\s*let mut {ARR_TMP}:")));
static PURE_VALUE_RE: LazyLock<Regex> = LazyLock::new(|| {
    let atom = format!(
        r"(?:{STR_LIT}|[A-Za-z_]\w*|-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?(?:i8|i16|i32|i64|u16|f32|f64)|true|false)"
    );
    re(&format!(r"^(?:{atom}|Clone::clone\(&{atom}\)|Object::from\(Clone::clone\(&{atom}\)\))$"))
});
/// 元素全为字符串字面量时的紧凑形态（runtime `JArray::from_strs`）
static STR_ELEM_RE: LazyLock<Regex> =
    LazyLock::new(|| re(r#"^(?:Clone::clone\(&)?String::from\(("(?:[^"\\]|\\.)*")\)\)?$"#));
static OBJ_STR_ELEM_RE: LazyLock<Regex> =
    LazyLock::new(|| re(r#"^Object::from\((?:Clone::clone\(&)?String::from\(("(?:[^"\\]|\\.)*")\)\)?\)$"#));

/// 本类静态字段 getter 值形态：`X::f()?` / `Clone::clone(&X::f()?)` / 其 `Object::from` 包装
/// （即 `^(?:{call}|Clone::clone\(&{call}\)|Object::from\(Clone::clone\(&{call}\)\))$`，
/// `call = (?:g1|g2|..)\(\)\?`；按集合查找，不逐方法编译多选正则）
struct GetterValue<'a>(&'a BTreeSet<String>);

impl GetterValue<'_> {
    fn is_match(&self, v: &str) -> bool {
        let call = |s: &str| s.strip_suffix("()?").is_some_and(|g| self.0.contains(g));
        let clone = |s: &str| s.strip_prefix("Clone::clone(&").and_then(|r| r.strip_suffix(')')).is_some_and(call);
        call(v) || clone(v) || v.strip_prefix("Object::from(").and_then(|r| r.strip_suffix(')')).is_some_and(clone)
    }
}

fn getter_value_re(static_getters: &BTreeSet<String>) -> Option<GetterValue<'_>> {
    (!static_getters.is_empty()).then_some(GetterValue(static_getters))
}

fn foldable_value(v: &str, getter: Option<&GetterValue<'_>>) -> bool {
    PURE_VALUE_RE.is_match(v) || getter.is_some_and(|g| g.is_match(v))
}

fn is_arr_intermediate(stmt: &str, getter: Option<&GetterValue<'_>>) -> bool {
    if stmt.contains('\n') || stmt.contains("JArray::from_strs(") || stmt.contains("JArray::objects_from_strs(") {
        // 已折叠块（多行 vec! / 去重表形态或单行字符串切片形态）
        return FOLDED_DECL_RE.is_match(stmt)
            && (stmt.contains(FOLDED_MARK) || stmt.contains(DEDUP_MARK) || stmt.contains("from_strs("));
    }
    if ARR_DECL_RE.is_match(stmt) {
        return true;
    }
    ARR_SET_RE.captures(stmt).is_some_and(|m| foldable_value(&m[4], getter))
}

/// 折叠结果：字符串字面量表 → `from_strs`；重复率高 → 去重值表 + 下标常量数组；其余 → `vec!`
fn folded_literal(ind: &str, var: &str, elem_t: &str, values: &[String]) -> String {
    let ctor = if elem_t == STRING {
        Some(("from_strs", &*STR_ELEM_RE))
    } else if elem_t == OBJECT {
        Some(("objects_from_strs", &*OBJ_STR_ELEM_RE))
    } else {
        None
    };
    if let Some((name, elem_re)) = ctor {
        let lits: Option<Vec<String>> = values.iter().map(|v| elem_re.captures(v).map(|c| c[1].to_string())).collect();
        if let Some(lits) = lits {
            return format!("{ind}let mut {var}: JArray<{elem_t}> = JArray::{name}(&[{}]);", lits.join(", "));
        }
    }
    let mut distinct: Vec<&str> = Vec::new();
    let mut index: BTreeMap<&str, usize> = BTreeMap::new();
    for v in values {
        if !index.contains_key(v.as_str()) {
            index.insert(v, distinct.len());
            distinct.push(v);
        }
    }
    if values.len() >= DEDUP_MIN_LEN && distinct.len() * 2 <= values.len() && distinct.len() <= 65535 {
        let idx: Vec<String> = values.iter().map(|v| index[v.as_str()].to_string()).collect();
        let vals: Vec<String> = distinct.iter().map(|v| format!("{ind}        {v},")).collect();
        return format!(
            "{ind}let mut {var}: JArray<{elem_t}> = {{\n{ind}    let __vals: [{elem_t}; {}] = [\n{}\n{ind}    ];\n\
             {ind}    {DEDUP_MARK}u16; {}] = [{}];\n\
             {ind}    JArray::from(__IDX.iter().map(|&k| Clone::clone(&__vals[k as usize])).collect::<Vec<{elem_t}>>())\n\
             {ind}}};",
            distinct.len(),
            vals.join("\n"),
            values.len(),
            idx.join(", ")
        );
    }
    let body: Vec<String> = values.iter().map(|v| format!("{ind}    {v},")).collect();
    format!("{ind}let mut {var}: JArray<{elem_t}> = {FOLDED_MARK}\n{}\n{ind}]);", body.join("\n"))
}

/// 数组初始化器折叠；static_getters 为本类静态字段 getter 路径 `X::f`（其读取视同纯值）
pub fn fold_array_literals(mut stmts: Vec<String>, static_getters: &BTreeSet<String>) -> Vec<String> {
    let getter = getter_value_re(static_getters);
    let getter = getter.as_ref();
    let decls: Vec<usize> = (0..stmts.len()).filter(|&i| ARR_DECL_RE.is_match(&stmts[i])).collect();
    // 内层（后声明）先折叠；折叠只改动 i 之后的位置，更早的声明不受影响
    for &i in decls.iter().rev() {
        let Some(m) = ARR_DECL_RE.captures(&stmts[i]) else { continue };
        if m[3] != m[4] {
            continue;
        }
        let (ind, var, elem_t) = (m[1].to_string(), m[2].to_string(), m[3].to_string());
        let Ok(n) = m[5].parse::<usize>() else { continue };
        if n == 0 {
            continue;
        }
        let ref_re = re(&format!(r"\b{}\b", regex::escape(&var)));
        let mut values: Vec<String> = Vec::new();
        let mut set_pos: Vec<usize> = Vec::new();
        let mut ok = true;
        let mut j = i + 1;
        while j < stmts.len() && values.len() < n {
            let s = &stmts[j];
            match ARR_SET_RE.captures(s).filter(|sm| sm[2] == *var) {
                Some(sm) => {
                    let v = &sm[4];
                    if sm[3].parse::<usize>().ok() != Some(values.len()) || !foldable_value(v, getter) || ref_re.is_match(v) {
                        ok = false;
                        break;
                    }
                    values.push(v.to_string());
                    set_pos.push(j);
                }
                None => {
                    if ref_re.is_match(s) || !is_arr_intermediate(s, getter) {
                        ok = false;
                        break;
                    }
                }
            }
            j += 1;
        }
        if !ok || values.len() != n {
            continue;
        }
        let folded = folded_literal(&ind, &var, &elem_t, &values);
        let last = *set_pos.last().expect("n > 0");
        let drop: BTreeSet<usize> = set_pos.iter().copied().chain([i]).collect();
        let tail = stmts.split_off(last + 1);
        let mut head: Vec<String> =
            std::mem::take(&mut stmts).into_iter().enumerate().filter(|(k, _)| !drop.contains(k)).map(|(_, s)| s).collect();
        head.push(folded);
        head.extend(tail);
        stmts = head;
    }
    stmts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getter_value_matches_alternation_regex() {
        let gs: BTreeSet<String> = ["X::a", "X::b_c", "Y::d"].map(String::from).into();
        let alts: Vec<String> = gs.iter().map(|g| regex::escape(g)).collect();
        let call = format!(r"(?:{})\(\)\?", alts.join("|"));
        let rx = re(&format!(r"^(?:{call}|Clone::clone\(&{call}\)|Object::from\(Clone::clone\(&{call}\)\))$"));
        let g = GetterValue(&gs);
        for v in [
            "X::a()?", "X::b_c()?", "X::a()", "Clone::clone(&X::a()?)", "Object::from(Clone::clone(&Y::d()?))",
            "Object::from(X::a()?)", "Clone::clone(&X::z()?)", "X::a()?x", "Object::from(Clone::clone(&X::a()?)",
        ] {
            assert_eq!(g.is_match(v), rx.is_match(v), "{v}");
        }
    }

    #[test]
    fn fold_simple() {
        let v: Vec<String> = [
            "    let mut _arr0: JArray<i32> = JArray::<i32>::try_new(2i32)?;",
            "    _arr0.set(0i32, 1i32)?;",
            "    _arr0.set(1i32, x)?;",
            "    foo(_arr0);",
        ]
        .map(String::from)
        .to_vec();
        let out = fold_array_literals(v, &BTreeSet::new());
        assert_eq!(out[0], "    let mut _arr0: JArray<i32> = JArray::from(vec![\n        1i32,\n        x,\n    ]);");
        assert_eq!(out.len(), 2);
    }
}
