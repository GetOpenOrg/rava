//! 引擎：元素封存的常量数组字段（数组标签 `Obj::Elems`，见 `absint/obj.rs`）。
//!
//! 字段 f 的读取值带各元素常量，`aaload` 在常量下标上折叠为该元素（如日志级别映射表 `spi2platformLevelMapping`）。条件：
//! - f 是 private static final 的引用元素数组字段，不开放（反射 / Unsafe / 手写写入由 `field_open` 在读取时另判）；
//! - 嵌套内（宿主与全部成员，不读事实的 Oracle 分析，与处理次序无关）唯一一次写入是声明类 `<clinit>` 的 putstatic；
//! - 嵌套内每次读取（getstatic）的值的全部用途都是以它为数组的元素读取——数组引用不逃出读取点，元素无从改写
//!   （`arraylength` 不产生事件，也不构成逃逸）；
//! - `<clinit>`（引擎事实分析，`static_const`）写入的值是新建的常量长度数组，新建到 putstatic 之间是直线代码
//!   （无跳转 / 返回 / 抛出，无外部跳入与异常处理器入口），其全部用途是常量下标、常量元素的存储、元素读取与
//!   putstatic f；未存入的下标为 null。
//!
//! 元素常量：int / long / null / 字符串 / 构建期映像对象（身份与 final 字段常量）。`<clinit>` 自身调用链在 putstatic
//! 之前读取 f 得 null、`aaload` 抛空指针——折叠只影响必然抛出的路径。

use super::class_lookup::{event_at, site_of, uses};
use super::sealed::nest_accesses;
use super::*;
use classfile::Operand;

/// 数组元素数上限
const MAX_ELEMS: i32 = 256;

fn const_elem(v: &V) -> bool {
    matches!(v, V::Int(_) | V::Long(_) | V::Null | V::Str(..)) || matches!(v.obj().map(|o| &**o), Some(Obj::Image(..)))
}

/// 读取点 r 的值只作数组元素读取的数组
fn load_only(a: &Analysis, r: u32) -> bool {
    uses(a, r).iter().all(|(_, e, _)| matches!(e, Event::ArrayLoad { array, .. } if site_of(array) == Some(r)))
}

/// 偏移 [from, to] 之间是直线代码：区间内（不含 to）无跳转 / switch / 返回 / 抛出，区间外无跳入 (from, to]，
/// 异常处理器入口不在 (from, to]
fn straight(code: &classfile::Code, from: u32, to: u32) -> bool {
    let inside = |o: u32| o > from && o <= to;
    let ok = code.insns.iter().all(|i| {
        let targets: Vec<u32> = match &i.operand {
            Operand::Branch(t) => vec![*t],
            Operand::TableSwitch { default, targets, .. } => std::iter::once(*default).chain(targets.iter().copied()).collect(),
            Operand::LookupSwitch { default, pairs } => std::iter::once(*default).chain(pairs.iter().map(|p| p.1)).collect(),
            _ => vec![],
        };
        // 返回 / athrow / ret
        let ends = matches!(i.opcode, 0xac..=0xb1 | 0xbf | 0xa9);
        let in_range = i.offset >= from && i.offset < to;
        !(in_range && (ends || !targets.is_empty())) && !targets.iter().any(|&t| inside(t))
    });
    ok && code.exception_table.iter().all(|e| !inside(e.handler))
}

/// `<clinit>` 事实分析 a（代码 code）中 putstatic f 写入的值 v：新建常量长度数组的元素
fn const_elems(a: &Analysis, code: &classfile::Code, v: &V, f: &MemberRef) -> Option<Rc<[V]>> {
    let n = site_of(v)?;
    event_at(a, n, |e| matches!(e, Event::NewArray(..)))?;
    let len = match v.obj().map(|o| &**o) {
        Some(&Obj::Len(l)) if (0..=MAX_ELEMS).contains(&l) => l as usize,
        _ => return None,
    };
    let mut es: Vec<Option<V>> = vec![None; len];
    let (mut stores, mut put) = (Vec::new(), None);
    for (off, e, _) in uses(a, n) {
        match e {
            Event::ArrayStore { array, index: V::Int(i), value } if site_of(array) == Some(n) && const_elem(value) => {
                let slot = usize::try_from(*i).ok().and_then(|i| es.get_mut(i))?;
                let x = value.stripped();
                if slot.as_ref().is_some_and(|p| *p != x) {
                    return None;
                }
                *slot = Some(x);
                stores.push(off);
            }
            Event::ArrayLoad { array, .. } if site_of(array) == Some(n) => {}
            Event::Field { opcode: classfile::op::PUTSTATIC, mref, value: Some(x), .. } if mref == f && site_of(x) == Some(n) && put.is_none() => put = Some(off),
            _ => return None,
        }
    }
    let put = put?;
    if !stores.iter().all(|&o| o > n && o < put) || !straight(code, n, put) {
        return None;
    }
    Some(es.into_iter().map(|e| e.unwrap_or(V::Null)).collect())
}

impl Ctx<'_> {
    /// 静态字段 owner.name:desc 在 `<clinit>` 中唯一一次写入值 v 的元素封存标签（不满足条件为 None）
    pub(super) fn sealed_elems(&self, owner: &str, name: &str, desc: &str, a: &Analysis, code: &classfile::Code, v: &V) -> Option<V> {
        if !desc.starts_with("[L") && !desc.starts_with("[[") {
            return None;
        }
        let key = MemberRef { owner: owner.to_string(), name: name.to_string(), desc: desc.to_string() };
        let es = const_elems(a, code, v, &key)?;
        if !self.sealed_array(&key) {
            return None;
        }
        Some(V::Ref { ty: Some(Rc::from(desc)), nonnull: true, src: Rc::from([].as_slice()), obj: Some(Rc::new(Obj::Elems(es))) })
    }

    /// 字段 f 的数组引用封存（见模块文档；只取决于字节码，按字段缓存）
    fn sealed_array(&self, f: &MemberRef) -> bool {
        if let Some(&b) = self.sealed_arrs.borrow().get(f) {
            return b;
        }
        let b = self.sealed_array_fresh(f);
        self.sealed_arrs.borrow_mut().insert(f.clone(), b);
        b
    }

    fn sealed_array_fresh(&self, f: &MemberRef) -> bool {
        const NEED: u16 = acc::PRIVATE | acc::STATIC | acc::FINAL;
        let Some(fi) = self.field_info(f) else { return false };
        if fi.access & NEED != NEED || fi.key != *f {
            return false;
        }
        let Some(xs) = nest_accesses(self.h, f) else { return false };
        let mut puts = 0;
        for x in &xs {
            if !x.writes.is_empty() {
                if x.owner != f.owner || x.name != "<clinit>" {
                    return false;
                }
                puts += x.writes.len();
            }
            if !x.reads.iter().all(|&r| load_only(&x.a, r)) {
                return false;
            }
        }
        puts == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use classfile::{ExceptionEntry, Insn};

    fn code(insns: Vec<(u32, u8, Operand)>, handlers: &[u32]) -> classfile::Code {
        classfile::Code {
            max_stack: 4,
            max_locals: 1,
            code_len: insns.last().map_or(0, |i| i.0 + 3),
            insns: insns.into_iter().map(|(offset, opcode, operand)| Insn { offset, opcode, operand }).collect(),
            exception_table: handlers.iter().map(|&h| ExceptionEntry { start: 0, end: 1, handler: h, catch_type: None }).collect(),
        }
    }

    /// 新建到 putstatic 之间无跳转、无跳入、无处理器入口才算直线代码
    #[test]
    fn straight_line_only() {
        let f = || Operand::Field(MemberRef { owner: "p/A".into(), name: "f".into(), desc: "[Lp/E;".into() });
        let base = vec![(0, 0x10, Operand::Int(2)), (2, 0xbd, Operand::Class("p/E".into())), (5, 0x59, Operand::None), (6, 0x03, Operand::None), (7, 0x01, Operand::None), (8, 0x53, Operand::None), (9, 0xb3, f()), (12, 0xb1, Operand::None)];
        assert!(straight(&code(base.clone(), &[]), 2, 9));
        // 区间内的条件跳转
        let mut br = base.clone();
        br[3] = (6, 0x99, Operand::Branch(9));
        assert!(!straight(&code(br, &[]), 2, 9));
        // 区间外跳入区间内
        let mut into = base.clone();
        into.push((13, 0xa7, Operand::Branch(8)));
        assert!(!straight(&code(into, &[]), 2, 9));
        // 异常处理器入口在区间内
        assert!(!straight(&code(base, &[7]), 2, 9));
    }
}
