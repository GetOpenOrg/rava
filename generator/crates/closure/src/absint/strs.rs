//! 字符串形状的方法内转移：清单构建器（`[facts.string_concat]`）的内容跟踪与前后缀判定收窄。
//!
//! 构建器标签（[`Obj::Builder`]）的健全性靠一条不变式：**同一状态里同组（同分配点）的各份带标签拷贝指向同一对象，
//! 且该对象不可能经无标签的引用被改写**。维持手段：
//! - 再次执行分配点（循环）：先撤掉该组全部标签，新对象独占该组；
//! - 追加：接收者带标签时内容变为「旧内容 + 段」，同组各份拷贝一起更新（它们就是接收者本身）；
//! - 其它任何把带标签值交出去的操作（作其它调用 / indy 的实参或接收者、写入字段 / 静态字段 / 数组）：撤掉全组；
//! - 合流：某槽位在任一入边带组 g 标签而合流结果不带 → 撤掉结果里全部组 g 标签（[`drop_lost_groups`]），
//!   于是不会出现「一份无标签的别名 + 一份过时的带标签拷贝」；
//! - 异常处理器入口撤掉全部构建器标签（追加可能在中途抛出）。
//!
//! 取结果（`toString`）不改写构建器：结果带 [`Obj::Str`] 标签（内容形状）。
//!
//! 前后缀收窄：`aload k; ldc L; invokevirtual <startsWith / endsWith>; ifeq/ifne`（后三条同一基本块）判定成立一侧
//! 局部变量 k 的形状与 L 相交（[`Shape::meet_starts`] / [`Shape::meet_ends`]），并确定非空；值的来源 / 类型不变。

use std::rc::Rc;

use classfile::descriptor::FieldType;
use classfile::{op, Const, Insn, Operand};

use super::shape::Shape;
use super::{Obj, State, Src, V};

/// 清单字符串操作的种类（构建器三类 + 前后缀判定）
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrKind {
    /// 构建器构造器（无参 = 空内容；一个实参 = 初始内容）
    Init,
    /// 追加（返回接收者本身）
    Append,
    /// 取结果
    Result,
    /// `startsWith(String)`
    StartsWith,
    /// `endsWith(String)`
    EndsWith,
}

fn retagged(v: &V, obj: Option<Rc<Obj>>) -> V {
    match v {
        V::Ref { ty, nonnull, src, .. } => V::Ref { ty: ty.clone(), nonnull: *nonnull, src: src.clone(), obj },
        other => other.clone(),
    }
}

/// 状态中的全部值（局部变量、栈、本帧 static final 写入值）
fn slots(s: &mut State) -> impl Iterator<Item = &mut V> {
    s.locals.iter_mut().chain(s.stack.iter_mut()).chain(s.finals.iter_mut().map(|(_, v)| v))
}

/// 状态里有构建器标签
pub(super) fn has_builders(s: &State) -> bool {
    s.locals.iter().chain(s.stack.iter()).chain(s.finals.iter().map(|(_, v)| v)).any(|v| v.builder().is_some())
}

/// 撤掉组 g 的全部构建器标签
pub(super) fn drop_group(s: &mut State, g: u32) {
    for v in slots(s) {
        if v.builder().is_some_and(|(h, _)| h == g) {
            *v = retagged(v, None);
        }
    }
}

/// 撤掉值 v 所带的构建器组（v 被交出去：实参 / 写入字段 / 数组）
pub(super) fn escape(s: &mut State, v: &V) {
    if let Some((g, _)) = v.builder() {
        drop_group(s, g);
    }
}

/// 撤掉全部构建器标签（异常处理器入口）
pub(super) fn strip_builders(vs: &mut [V]) {
    for v in vs.iter_mut() {
        if v.builder().is_some() {
            *v = retagged(v, None);
        }
    }
}

/// 合流后：入边带某组标签、结果不带的槽位所在的组整组撤掉（见模块文档的不变式）
pub(super) fn drop_lost_groups(r: &mut State, a: &State, b: &State) -> bool {
    let all = |s: &State| s.locals.iter().chain(s.stack.iter()).chain(s.finals.iter().map(|(_, v)| v)).cloned().collect::<Vec<V>>();
    let (ra, aa, bb) = (all(r), all(a), all(b));
    let mut lost: Vec<u32> = Vec::new();
    // finals 按字段对齐（合流只保留两侧都有的字段）：按位置对齐局部变量与栈，finals 另按字段键对齐
    let n = r.locals.len() + r.stack.len();
    for i in 0..n {
        let have = ra[i].builder().map(|(g, _)| g);
        for x in [aa.get(i), bb.get(i)].into_iter().flatten() {
            if let Some((g, _)) = x.builder() {
                if have != Some(g) {
                    lost.push(g);
                }
            }
        }
    }
    for (f, v) in &r.finals {
        let have = v.builder().map(|(g, _)| g);
        for s in [a, b] {
            if let Some((g, _)) = s.finals.iter().find(|(h, _)| h == f).and_then(|(_, x)| x.builder()) {
                if have != Some(g) {
                    lost.push(g);
                }
            }
        }
    }
    // 只在一侧有的 finals 字段被合流丢弃：其构建器组同样撤掉
    for s in [a, b] {
        for (f, x) in &s.finals {
            if let Some((g, _)) = x.builder() {
                if !r.finals.iter().any(|(h, _)| h == f) {
                    lost.push(g);
                }
            }
        }
    }
    lost.sort();
    lost.dedup();
    let mut changed = false;
    for g in lost {
        let any = slots(r).any(|v| v.builder().is_some_and(|(h, _)| h == g));
        if any {
            drop_group(r, g);
            changed = true;
        }
    }
    changed
}

/// 追加段的形状（按追加方法的形参类型；`String.valueOf` 语义）
fn segment(param: &FieldType, arg: &V) -> Shape {
    let digits = || Shape::of_chars("-0123456789");
    match param {
        FieldType::Prim(b'C') => match arg {
            V::Int(c) => u32::try_from(*c).ok().and_then(char::from_u32).map_or_else(Shape::top, |c| Shape::lit(c.encode_utf8(&mut [0; 4]))),
            _ => Shape::top(),
        },
        FieldType::Prim(b'I' | b'S' | b'B') => match arg {
            V::Int(x) => Shape::lit(&x.to_string()),
            _ => digits(),
        },
        FieldType::Prim(b'J') => match arg {
            V::Long(x) => Shape::lit(&x.to_string()),
            _ => digits(),
        },
        FieldType::Prim(b'Z') => match arg {
            V::Int(0) => Shape::lit("false"),
            V::Int(1) => Shape::lit("true"),
            _ => Shape::of_chars("truefals"),
        },
        _ if param.is_reference() => match arg {
            V::Null => Shape::lit("null"),
            _ if arg.builder().is_some() => Shape::top(),
            _ => match (arg.str_shape(), arg.nonnull()) {
                (Some(sh), Some(true)) => sh,
                (Some(sh), _) => sh.join(&Shape::lit("null")),
                _ => Shape::top(),
            },
        },
        _ => Shape::top(),
    }
}

/// 构建器构造器返回时的标签：接收者是本方法分配点 g 的新建对象
pub(super) fn init_tag(params: &[FieldType], args: &[V]) -> Option<Rc<Obj>> {
    let recv = args.first()?;
    let [Src::Site(g)] = &recv.srcs()[..] else { return None };
    let content = match (params, args.get(1)) {
        ([], _) => Shape::lit(""),
        // 初始内容实参为 null 时构造器抛出，返回即非空
        ([p], Some(a)) if p.is_reference() => a.str_shape().unwrap_or_else(Shape::top),
        _ => return None,
    };
    Some(Rc::new(Obj::Builder { group: *g, content }))
}

/// 构建器调用的状态转移（实参已弹出、结果尚未压入）。返回结果值的标签改写：
/// Some(tag) = 结果换成该标签；None = 结果不改。非构建器调用 / 无标签接收者：交出的带标签实参整组撤掉
pub(super) fn invoke(s: &mut State, kind: Option<StrKind>, params: &[FieldType], args: &[V], is_static: bool) -> Option<Option<Rc<Obj>>> {
    let recv = (!is_static).then(|| args.first()).flatten();
    let tagged = recv.and_then(|r| r.builder()).map(|(g, c)| (g, c.clone()));
    let rest = if is_static { args } else { args.get(1..).unwrap_or(&[]) };
    match (kind, tagged) {
        (Some(StrKind::Append), Some((g, content))) => {
            for a in rest {
                if a.builder().is_some_and(|(h, _)| h != g) {
                    escape(s, a);
                }
            }
            let seg = match (params, rest) {
                ([p], [a]) => segment(p, a),
                _ => Shape::top(),
            };
            let tag = Rc::new(Obj::Builder { group: g, content: content.concat(&seg) });
            for v in slots(s) {
                if v.builder().is_some_and(|(h, _)| h == g) {
                    *v = retagged(v, Some(tag.clone()));
                }
            }
            Some(Some(tag))
        }
        (Some(StrKind::Result), Some((_, content))) => {
            for a in rest {
                escape(s, a);
            }
            Some((!content.is_top()).then(|| Rc::new(Obj::Str(content))))
        }
        _ => {
            for a in args {
                escape(s, a);
            }
            None
        }
    }
}

/// 下标 i 处 `aload k; ldc L; invokevirtual <前后缀判定>; ifeq/ifne` 的成立一侧收窄：返回 (k, 跳转侧的值, 顺延侧的值)
pub(super) fn affix_narrow(insns: &[Insn], leader: &[bool], i: usize, st: &State, kind: impl Fn(&classfile::MemberRef) -> Option<StrKind>) -> Option<(usize, V, V)> {
    let yes_taken = match insns[i].opcode {
        0x9a => true,
        0x99 => false,
        _ => return None,
    };
    if i < 3 || leader[i] || leader[i - 1] || leader[i - 2] {
        return None;
    }
    let (op::INVOKEVIRTUAL, Operand::Method(m, _)) = (insns[i - 1].opcode, &insns[i - 1].operand) else { return None };
    let k = kind(m)?;
    let Operand::Ldc(Const::String(l)) = &insns[i - 2].operand else { return None };
    let slot = match (insns[i - 3].opcode, &insns[i - 3].operand) {
        (0x19, Operand::Local(k)) => *k as usize,
        (0x2a..=0x2d, _) => (insns[i - 3].opcode - 0x2a) as usize,
        _ => return None,
    };
    let input = st.locals.get(slot)?;
    let V::Ref { ty, src, obj, .. } = input else { return None };
    let shape = match obj.as_deref() {
        None => Shape::top(),
        Some(Obj::Str(sh)) => sh.clone(),
        Some(_) => return None,
    };
    let met = match k {
        StrKind::StartsWith => shape.meet_starts(l),
        StrKind::EndsWith => shape.meet_ends(l),
        _ => return None,
    };
    let yes = V::Ref { ty: ty.clone(), nonnull: true, src: src.clone(), obj: Some(Rc::new(Obj::Str(met))) };
    // 接收者已被解引用：两侧都非空
    let no = V::Ref { ty: ty.clone(), nonnull: true, src: src.clone(), obj: obj.clone() };
    Some(if yes_taken { (slot, yes, no) } else { (slot, no, yes) })
}
