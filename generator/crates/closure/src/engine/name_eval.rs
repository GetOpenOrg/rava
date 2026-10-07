//! 引擎：按名取类的名字求值——求值帧、基本类型段、类镜像取名。
//!
//! - 求值帧：名字值所在的分析。引擎方法帧读值集（类镜像、常量表接收者）；被调辅助方法的独立分析帧不读值集，
//!   其形参（来源恰为形参 i 的值）换成调用方帧里该调用点的第 i 个实参（含接收者，与 `Src::Param` 对齐），逐层上溯；
//! - 基本类型段（`StringBuilder.append(C)`、indy 拼接的基本类型动态实参）：常量按 `String.valueOf` 语义成字面量；
//! - 类镜像取名（`[facts.reflect] name_of_receiver` / `simple_name_of_receiver`）：接收者是类字面量，或引擎帧里
//!   值集全是字节码类镜像时，取各所指类的 binary name / 简单名；接收者含 open、非镜像、基本类型 / 合成镜像时推不出；
//!   值集尚空时为空集（值集增长时站点重跑）。
//!
//! 引擎方法形参上的名字（各调用点流入的字符串）见 `pstrs.rs::param_names`。

use super::class_lookup::{event_at, is_invoke};
use super::*;

/// 名字值所在的求值帧
pub(super) struct Frame<'f> {
    /// 引擎方法；None = 被调方法的独立分析（不读值集与常量表）
    pub(super) m: Option<usize>,
    pub(super) a: &'f Analysis,
    /// 代码所在类（取引导方法表）
    pub(super) owner: &'f str,
    /// 调用方帧与该调用点的实参（含接收者）
    pub(super) up: Option<(&'f Frame<'f>, &'f [V])>,
}

impl<'f> Frame<'f> {
    /// 值恰为本帧形参时换成调用方帧里对应的实参（逐层上溯），返回值所在帧与值
    pub(super) fn resolve(&'f self, v: &V) -> (&'f Frame<'f>, V) {
        let mut f = self;
        let mut v = v.clone();
        loop {
            let Some((pf, args)) = f.up else { return (f, v) };
            let i = match &v {
                V::Ref { src, .. } => match src[..] {
                    [Src::Param(i)] => i as usize,
                    _ => return (f, v),
                },
                _ => return (f, v),
            };
            let Some(x) = args.get(i) else { return (f, v) };
            v = x.clone();
            f = pf;
        }
    }
}

/// 基本类型常量段（描述符类型首字母 k）按 `String.valueOf` 成字面量
pub(super) fn prim_lit(v: &V, k: u8) -> Option<Rc<str>> {
    let s = match (v, k) {
        (V::Int(i), b'C') => char::from_u32(*i as u16 as u32)?.to_string(),
        (V::Int(i), b'Z') => (if *i != 0 { "true" } else { "false" }).to_string(),
        (V::Int(i), b'I' | b'B' | b'S') => i.to_string(),
        (V::Long(l), b'J') => l.to_string(),
        _ => return None,
    };
    Some(Rc::from(s.as_str()))
}

/// 基本类型描述符字母的类型名（`Class.getName` / `getSimpleName` 对基本类型的结果）
fn prim_name(c: u8) -> Option<&'static str> {
    Some(match c {
        b'B' => "byte",
        b'C' => "char",
        b'D' => "double",
        b'F' => "float",
        b'I' => "int",
        b'J' => "long",
        b'S' => "short",
        b'Z' => "boolean",
        b'V' => "void",
        _ => return None,
    })
}

/// 类（内部名，数组为描述符形式）的简单名（`Class.getSimpleName` 语义）：数组为元素简单名加 `[]`；
/// 有本类 InnerClasses 项（嵌套 / 局部 / 匿名类）取其内部名（匿名为空串）；其余取最后一个 `/` 之后的部分。
/// inner = 类的 InnerClasses 里本类项的简单名（外层 None = 无本类项；类不在类路径上时整体 None）
pub(super) fn simple_name(cls: &str, inner: &dyn Fn(&str) -> Option<Option<Option<String>>>) -> Option<String> {
    if let Some(e) = cls.strip_prefix('[') {
        let comp = match e.as_bytes().first()? {
            b'L' => simple_name(e.strip_prefix('L')?.strip_suffix(';')?, inner)?,
            b'[' => simple_name(e, inner)?,
            &c => prim_name(c)?.to_string(),
        };
        return Some(comp + "[]");
    }
    match inner(cls)? {
        Some(simple) => Some(simple.unwrap_or_default()),
        None => Some(cls.rsplit('/').next().unwrap_or(cls).to_string()),
    }
}

impl<'a> Engine<'a> {
    /// 站点 o 是类镜像取名调用时的结果候选：外层 None = 不是取名调用；内层 None = 推不出
    pub(super) fn mirror_name(&mut self, f: &Frame, o: u32) -> Option<Option<BTreeSet<Rc<str>>>> {
        let Some(Event::Invoke { mref, args, .. }) = event_at(f.a, o, is_invoke) else { return None };
        let key = mref.to_string();
        let simple = if self.man.names.is_name_of(&key) {
            false
        } else if self.man.names.is_simple_name_of(&key) {
            true
        } else {
            return None;
        };
        let Some(recv) = args.first() else { return Some(None) };
        let Some((classes, prim)) = self.frame_mirror_classes(f, recv) else { return Some(None) };
        let mut out = BTreeSet::new();
        // 基本类型镜像（各基本类型共用一个抽象镜像）：名字与简单名同为类型关键字，取全部基本类型名（超集）
        if prim {
            out.extend(b"BCDFIJSZV".iter().filter_map(|&c| prim_name(c)).map(Rc::from));
        }
        for c in classes {
            let name = if simple {
                let h = &self.h;
                let inner = |c: &str| -> Option<Option<Option<String>>> {
                    let cf = h.class(c)?;
                    Some(cf.inner_classes.iter().find(|ic| ic.inner == c).map(|ic| ic.simple_name.clone()))
                };
                match simple_name(&c, &inner) {
                    Some(n) => n,
                    None => return Some(None),
                }
            } else {
                c.replace('/', ".")
            };
            out.insert(Rc::from(name.as_str()));
        }
        Some(Some(out))
    }

    /// Class 值所指的字节码类（内部名 / 数组描述符）与是否含基本类型镜像：类字面量，或引擎帧里值集全是字节码类镜像 /
    /// 基本类型镜像（值集增长时站点重跑）
    fn frame_mirror_classes(&mut self, f: &Frame, v: &V) -> Option<(Vec<String>, bool)> {
        let (f, v) = f.resolve(v);
        match &v {
            V::Class(c, _) => Some((vec![c.to_string()], false)),
            V::Ref { .. } => {
                let m = f.m?;
                let class = self.id(CLASS);
                let fs = self.feeds(m, &v, class);
                let s = self.value_set(&fs);
                if !s.open.is_empty() {
                    return None;
                }
                let mut out = vec![];
                let mut prim = false;
                for x in s.classes.iter() {
                    if Some(x) == self.prim_mirror {
                        prim = true;
                        continue;
                    }
                    let &c = self.mirrors.get(&x)?;
                    out.push(self.names[c as usize].to_string());
                }
                Some((out, prim))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prim_literals() {
        assert_eq!(prim_lit(&V::Int('$' as i32), b'C').as_deref(), Some("$"));
        assert_eq!(prim_lit(&V::Int(1), b'Z').as_deref(), Some("true"));
        assert_eq!(prim_lit(&V::Int(0), b'Z').as_deref(), Some("false"));
        assert_eq!(prim_lit(&V::Int(-7), b'I').as_deref(), Some("-7"));
        assert_eq!(prim_lit(&V::Long(1 << 40), b'J').as_deref(), Some("1099511627776"));
        assert_eq!(prim_lit(&V::Top, b'I'), None);
        assert_eq!(prim_lit(&V::Int(3), b'F'), None);
    }

    #[test]
    fn simple_names() {
        // a/O$N 是嵌套类（简单名 N），a/O$1 是匿名类，a/O 是顶层类
        let inner = |c: &str| -> Option<Option<Option<String>>> {
            match c {
                "a/O$N" => Some(Some(Some("N".into()))),
                "a/O$1" => Some(Some(None)),
                "a/O" | "a/Top$X" => Some(None),
                _ => None,
            }
        };
        assert_eq!(simple_name("a/O$N", &inner).as_deref(), Some("N"));
        assert_eq!(simple_name("a/O$1", &inner).as_deref(), Some(""));
        assert_eq!(simple_name("a/O", &inner).as_deref(), Some("O"));
        assert_eq!(simple_name("a/Top$X", &inner).as_deref(), Some("Top$X"));
        assert_eq!(simple_name("[[La/O$N;", &inner).as_deref(), Some("N[][]"));
        assert_eq!(simple_name("[I", &inner).as_deref(), Some("int[]"));
        assert_eq!(simple_name("a/Missing", &inner), None);
    }
}
