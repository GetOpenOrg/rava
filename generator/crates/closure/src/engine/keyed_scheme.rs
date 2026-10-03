//! 引擎：按键查找的协议键——调用点的键是某个 URL 串形参解析出的协议名（`[facts.keyed_lookups]` 的 `scheme_sites`）。
//!
//! 解析规则（`java.net.URL` 按串构造的协议识别，与 JDK 字节码同口径）：
//! - 去掉首部 `<= ' '` 的字符，再跳过不区分大小写的 `url:` 前缀；
//! - 随后首字符是 `#`：无协议；
//! - 向后扫描到首个 `/`（无协议）或首个 `:`：其前的串按 `lowerCaseProtocol`（`Locale.ROOT` 小写）后，首字符为字母、
//!   其余为字母 / 数字 / `.` / `+` / `-` 时即协议名，否则无协议；
//! - 无协议的串不给出键（该构造器只有在串带协议、且不沿用上下文处理器时才按协议查找处理器）。
//!
//! 串由拼接段候选模式给出：判定所需的前缀落在任意串段之前即可确定；落进任意串段、含非 ASCII 字符（字母判定与
//! 大小写折叠随 Unicode）时推不出，记为任意键。槽推不出时整体为任意键。

use super::class_lookup::Part;
use super::keyed::Keys;
use super::*;

/// 一个模式最多展开的候选集元素组合数：超出按推不出处理
const MAX_STRS: usize = 256;

/// 已知前缀串的协议判定
#[derive(Debug, PartialEq, Eq)]
enum Scheme {
    /// 无协议
    None,
    /// 推不出
    Any,
    Name(String),
}

/// 前缀 s（open = 其后接任意串段）解析出的协议
fn scheme_of(s: &str, open: bool) -> Scheme {
    let t = s.trim_start_matches(|c: char| c <= ' ');
    if t.is_empty() {
        return if open { Scheme::Any } else { Scheme::None };
    }
    // 字母判定与大小写折叠只在 ASCII 上与 Unicode 规则无关
    let lower = t.to_ascii_lowercase();
    let t = if lower.starts_with("url:") {
        &t[4..]
    } else if "url:".starts_with(lower.as_str()) && open {
        return Scheme::Any;
    } else {
        t
    };
    if t.starts_with('#') {
        return Scheme::None;
    }
    let Some(end) = t.find([':', '/']) else {
        return if open { Scheme::Any } else { Scheme::None };
    };
    if t.as_bytes()[end] == b'/' {
        return Scheme::None;
    }
    let cand = &t[..end];
    if !cand.is_ascii() {
        return Scheme::Any;
    }
    let p = cand.to_ascii_lowercase();
    let mut cs = p.chars();
    let valid = cs.next().is_some_and(|c| c.is_ascii_alphabetic()) && cs.all(|c| c.is_ascii_alphanumeric() || ".+-".contains(c));
    if valid {
        Scheme::Name(p)
    } else {
        Scheme::None
    }
}

/// 一个候选模式的已知前缀（各候选集元素逐个展开）与其后是否接任意串段；超出上限为 None
fn prefixes(pat: &[Part]) -> Option<Vec<(String, bool)>> {
    let mut cur: Vec<String> = vec![String::new()];
    for p in pat {
        match p {
            Part::Lit(l) => cur.iter_mut().for_each(|s| s.push_str(l)),
            Part::Any(set) => {
                if cur.len().saturating_mul(set.len()) > MAX_STRS {
                    return None;
                }
                cur = cur.iter().flat_map(|s| set.iter().map(move |x| format!("{s}{x}"))).collect();
            }
            Part::Wild | Part::Alt(_) => return Some(cur.into_iter().map(|s| (s, true)).collect()),
        }
    }
    Some(cur.into_iter().map(|s| (s, false)).collect())
}

/// 候选模式集解析出的协议键集
pub(super) fn scheme_keys_of(pats: &[Vec<Part>]) -> Keys {
    let mut out: BTreeSet<Rc<str>> = BTreeSet::new();
    for pat in pats {
        let Some(ps) = prefixes(pat) else { return Keys::Any };
        for (s, open) in ps {
            match scheme_of(&s, open) {
                Scheme::None => {}
                Scheme::Any => return Keys::Any,
                Scheme::Name(n) => {
                    out.insert(Rc::from(n.as_str()));
                }
            }
        }
    }
    Keys::Set(out)
}

impl<'a> Engine<'a> {
    /// 方法 m 中协议键站点的键：形参 j（按描述符，0 起，不含接收者）上 URL 串的协议名集合
    pub(super) fn scheme_keys(&mut self, m: usize, j: usize) -> Keys {
        let i = j + usize::from(!self.methods[m].is_static);
        match self.param_patterns(m, i) {
            Some(pats) => scheme_keys_of(&pats),
            None => Keys::Any,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lit(s: &str) -> Part {
        Part::Lit(Rc::from(s))
    }

    fn set(xs: &[&str]) -> Keys {
        Keys::Set(xs.iter().map(|x| Rc::from(*x)).collect())
    }

    #[test]
    fn constant_specs() {
        assert_eq!(scheme_of("jar:file:/a.jar!/x", false), Scheme::Name("jar".into()));
        assert_eq!(scheme_of("  URL:FILE:/tmp", false), Scheme::Name("file".into()));
        assert_eq!(scheme_of("HTTP://h/p", false), Scheme::Name("http".into()));
        assert_eq!(scheme_of("x-y.z+w:rest", false), Scheme::Name("x-y.z+w".into()));
        // 无协议：相对路径、`/` 在 `:` 前、`#` 开头、非法协议名、全空白
        assert_eq!(scheme_of("a/b:c", false), Scheme::None);
        assert_eq!(scheme_of("#frag:x", false), Scheme::None);
        assert_eq!(scheme_of("1ab:x", false), Scheme::None);
        assert_eq!(scheme_of("relative", false), Scheme::None);
        assert_eq!(scheme_of("   ", false), Scheme::None);
        // 非 ASCII 协议名：推不出
        assert_eq!(scheme_of("é:x", false), Scheme::Any);
    }

    #[test]
    fn prefixes_before_wild() {
        // 已知前缀已定出协议 / 无协议
        assert_eq!(scheme_of("file:", true), Scheme::Name("file".into()));
        assert_eq!(scheme_of("/abs/", true), Scheme::None);
        // 判定落进任意串段：推不出
        assert_eq!(scheme_of("", true), Scheme::Any);
        assert_eq!(scheme_of("fil", true), Scheme::Any);
        assert_eq!(scheme_of("ur", true), Scheme::Any);
        assert_eq!(scheme_of("url:", true), Scheme::Any);
    }

    #[test]
    fn pattern_sets() {
        let pats = vec![vec![lit("jar:"), Part::Wild], vec![Part::Any([Rc::from("file:/a"), Rc::from("ftp://h/")].into())], vec![lit("rel/x")]];
        assert_eq!(scheme_keys_of(&pats), set(&["file", "ftp", "jar"]));
        assert_eq!(scheme_keys_of(&[vec![Part::Wild, lit(":x")]]), Keys::Any);
        assert_eq!(scheme_keys_of(&[vec![lit("rel/"), Part::Wild]]), set(&[]));
        assert_eq!(scheme_keys_of(&[]), set(&[]));
    }
}
