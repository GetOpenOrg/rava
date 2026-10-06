//! 字符串形状域：值的已知前缀 / 后缀与确定不含的 ASCII 字符（经典的前缀 · 后缀 · 字符包含三个串抽象域之积）。
//!
//! - `pre` / `suf`：串一定以 `pre` 开头、以 `suf` 结尾（两者可重叠）；`exact` 时串恰为 `pre`（= `suf`）；
//! - `no`：一定不出现的 ASCII 字符（位 c = 字符 c）；非 ASCII 字符不记；
//! - 格序：合流取公共前缀、公共后缀、不含字符的交集，格高有限（各分量只减不增），不动点必收敛；
//!   前后缀超过 [`CAP`] 个字符时截短（变弱，仍健全）。
//!
//! 与 Java 语义的对应：形状里的串都来自类文件的合法 Unicode 常量（含孤立代理项的常量不成形状），
//! 合法串之间按码点的前缀 / 后缀关系与按 UTF-16 码元的关系一致（合法串不以孤立代理项起止），
//! 所以按 Rust `str` 判定 `startsWith` / `endsWith` 与运行期结果相同；运行期串在前后缀之外的部分可含任意码元。
//!
//! 判定（`Some(b)` = 恒为 b；None = 推不出）：
//! - `startsWith(L)`：`pre` 以 L 开头 → 真；L 与 `pre` 互不为前缀，或 L 含不出现的字符 → 假
//!   （串同时以 `pre` 与 L 开头时两者必有一个是另一个的前缀）；`endsWith` 对称；
//! - `equals(L)`：L 不以 `pre` 开头 / 不以 `suf` 结尾 / 含不出现的字符 → 假；
//! - `indexOf` / `lastIndexOf` / `contains`：所找字符（或子串中任一字符）不出现 → 未找到。

use std::rc::Rc;

/// 前后缀保留的最大字符数
pub const CAP: usize = 256;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Shape {
    pub pre: Rc<str>,
    pub suf: Rc<str>,
    /// 确定不出现的 ASCII 字符（位 c）
    pub no: u128,
    /// 串恰为 `pre`
    pub exact: bool,
}

fn ascii_mask(s: &str) -> u128 {
    s.chars().filter(|c| c.is_ascii()).fold(0u128, |m, c| m | 1u128 << (c as u32))
}

fn head(s: &str, n: usize) -> &str {
    match s.char_indices().nth(n) {
        Some((i, _)) => &s[..i],
        None => s,
    }
}

fn tail(s: &str, n: usize) -> &str {
    let k = s.chars().count();
    if k <= n {
        return s;
    }
    match s.char_indices().nth(k - n) {
        Some((i, _)) => &s[i..],
        None => s,
    }
}

fn common_prefix<'s>(a: &'s str, b: &str) -> &'s str {
    let mut end = 0;
    for ((i, x), y) in a.char_indices().zip(b.chars()) {
        if x != y {
            return &a[..i];
        }
        end = i + x.len_utf8();
    }
    &a[..end]
}

fn common_suffix<'s>(a: &'s str, b: &str) -> &'s str {
    let mut start = a.len();
    for ((i, x), y) in a.char_indices().rev().zip(b.chars().rev()) {
        if x != y {
            break;
        }
        start = i;
    }
    &a[start..]
}

impl Shape {
    /// 恰为 s 的串
    pub fn lit(s: &str) -> Shape {
        if s.chars().nth(CAP).is_some() {
            return Shape { pre: Rc::from(head(s, CAP)), suf: Rc::from(tail(s, CAP)), no: !ascii_mask(s), exact: false };
        }
        let r: Rc<str> = Rc::from(s);
        Shape { pre: r.clone(), suf: r, no: !ascii_mask(s), exact: true }
    }

    /// 任意串
    pub fn top() -> Shape {
        Shape { pre: Rc::from(""), suf: Rc::from(""), no: 0, exact: false }
    }

    /// 只由 `allowed` 中的 ASCII 字符组成的串（如整数的十进制形式）
    pub fn of_chars(allowed: &str) -> Shape {
        Shape { pre: Rc::from(""), suf: Rc::from(""), no: !ascii_mask(allowed), exact: false }
    }

    /// 确定不含 `absent` 中各 ASCII 字符的任意串
    pub fn excluding(absent: &str) -> Shape {
        Shape { pre: Rc::from(""), suf: Rc::from(""), no: ascii_mask(absent), exact: false }
    }

    /// 推不出任何性质
    pub fn is_top(&self) -> bool {
        !self.exact && self.pre.is_empty() && self.suf.is_empty() && self.no == 0
    }

    pub fn join(&self, o: &Shape) -> Shape {
        if self == o {
            return self.clone();
        }
        Shape { pre: Rc::from(common_prefix(&self.pre, &o.pre)), suf: Rc::from(common_suffix(&self.suf, &o.suf)), no: self.no & o.no, exact: false }
    }

    /// 拼接 self + o
    pub fn concat(&self, o: &Shape) -> Shape {
        let no = self.no & o.no;
        match (self.exact, o.exact) {
            (true, true) => Shape::lit(&format!("{}{}", self.pre, o.pre)),
            (true, false) => Shape { pre: Rc::from(head(&format!("{}{}", self.pre, o.pre), CAP)), suf: o.suf.clone(), no, exact: false },
            (false, true) => Shape { pre: self.pre.clone(), suf: Rc::from(tail(&format!("{}{}", self.suf, o.pre), CAP)), no, exact: false },
            (false, false) => Shape { pre: self.pre.clone(), suf: o.suf.clone(), no, exact: false },
        }
    }

    /// 串不可能含 l 中的某个字符（l 含不出现的 ASCII 字符）
    fn rules_out(&self, l: &str) -> bool {
        ascii_mask(l) & self.no != 0
    }

    pub fn starts_with(&self, l: &str) -> Option<bool> {
        if self.exact {
            return Some(self.pre.starts_with(l));
        }
        if self.pre.starts_with(l) {
            return Some(true);
        }
        if !l.starts_with(&*self.pre) || self.rules_out(l) {
            return Some(false);
        }
        None
    }

    pub fn ends_with(&self, l: &str) -> Option<bool> {
        if self.exact {
            return Some(self.pre.ends_with(l));
        }
        if self.suf.ends_with(l) {
            return Some(true);
        }
        if !l.ends_with(&*self.suf) || self.rules_out(l) {
            return Some(false);
        }
        None
    }

    pub fn equals(&self, l: &str) -> Option<bool> {
        if self.exact {
            return Some(*self.pre == *l);
        }
        if !l.starts_with(&*self.pre) || !l.ends_with(&*self.suf) || self.rules_out(l) {
            return Some(false);
        }
        None
    }

    pub fn is_empty(&self) -> Option<bool> {
        if self.exact {
            return Some(self.pre.is_empty());
        }
        (!self.pre.is_empty() || !self.suf.is_empty()).then_some(false)
    }

    /// 含子串 l
    pub fn contains(&self, l: &str) -> Option<bool> {
        if self.exact {
            return Some(self.pre.contains(l));
        }
        if self.pre.contains(l) || self.suf.contains(l) {
            return Some(true);
        }
        self.rules_out(l).then_some(false)
    }

    /// 判定成立一侧：串以 l 开头
    pub fn meet_starts(&self, l: &str) -> Shape {
        if self.exact || self.pre.starts_with(l) || !l.starts_with(&*self.pre) {
            return self.clone();
        }
        Shape { pre: Rc::from(head(l, CAP)), ..self.clone() }
    }

    /// 判定成立一侧：串以 l 结尾
    pub fn meet_ends(&self, l: &str) -> Shape {
        if self.exact || self.suf.ends_with(l) || !l.ends_with(&*self.suf) {
            return self.clone();
        }
        Shape { suf: Rc::from(tail(l, CAP)), ..self.clone() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_keeps_common_affixes_and_absent_chars() {
        let j = Shape::lit("/a/b/").join(&Shape::lit("/c/"));
        assert_eq!((&*j.pre, &*j.suf, j.exact), ("/", "/", false));
        assert_eq!(j.no & (1 << b'#'), 1 << b'#');
        assert_eq!(j.no & (1 << b'a'), 0);
        assert_eq!(j.starts_with("/"), Some(true));
        assert_eq!(j.ends_with("/"), Some(true));
        assert_eq!(j.starts_with("jar:"), Some(false));
        assert_eq!(j.starts_with("/a"), None);
        // `x` 两侧都不出现
        assert_eq!(j.starts_with("/x"), Some(false));
        assert_eq!(j.equals("/"), None);
        assert_eq!(j.equals("x/"), Some(false));
        assert_eq!(j.is_empty(), Some(false));
        assert_eq!(j.contains("#"), Some(false));
    }

    #[test]
    fn concat_extends_prefix_and_suffix() {
        let any = Shape::top();
        let s = Shape::lit("/").concat(&any);
        assert_eq!((&*s.pre, &*s.suf), ("/", ""));
        let t = s.concat(&Shape::lit("/"));
        assert_eq!((&*t.pre, &*t.suf), ("/", "/"));
        assert_eq!(Shape::lit("ab").concat(&Shape::lit("c")), Shape::lit("abc"));
        // 不出现的字符取交集：任意串一侧什么都不排除
        assert_eq!(t.no, 0);
        let digits = Shape::of_chars("-0123456789");
        assert_eq!(Shape::lit("x").concat(&digits).contains("#"), Some(false));
    }

    #[test]
    fn meets_narrow_only_compatibly() {
        let s = Shape::top().meet_starts("/");
        assert_eq!(&*s.pre, "/");
        assert_eq!(s.meet_starts("/ab").pre.as_ref(), "/ab");
        // 不相容：判定恒假，该侧不可达，形状不变
        assert_eq!(s.meet_starts("x").pre.as_ref(), "/");
        assert_eq!(Shape::top().meet_ends("/").suf.as_ref(), "/");
    }

    #[test]
    fn non_ascii_affixes_split_on_char_boundaries() {
        let j = Shape::lit("é/x").join(&Shape::lit("é/y"));
        assert_eq!(&*j.pre, "é/");
        let k = Shape::lit("aé").join(&Shape::lit("bé"));
        assert_eq!(&*k.suf, "é");
        let long = "z".repeat(CAP + 10);
        let l = Shape::lit(&long);
        assert!(!l.exact);
        assert_eq!(l.pre.chars().count(), CAP);
        assert_eq!(l.starts_with("zz"), Some(true));
        assert_eq!(l.equals("z"), Some(false));
        assert_eq!(l.equals(&long), None);
    }
}
