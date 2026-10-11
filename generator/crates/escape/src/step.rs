//! 指令语义：方法内逐指令数据流（每条指令入口一帧，合流点取并），整型常量分支剪枝。

use classfile::op;
use classfile::{Const, Insn, Operand};
use input::NInsn;
use ty::registry::ClassInfo;

use crate::analysis::{reject, Analysis, NodeKind, ELEM, R};
use crate::val::{Frame, Prim, Refs, Val};
use crate::world::{field_kind, method_kinds, Kind};

/// 一条指令的后继
#[derive(Default)]
pub struct Out {
    pub fall: Option<Frame>,
    pub jumps: Vec<(u32, Frame)>,
    pub ret: Option<Val>,
}

pub fn pop(f: &mut Frame) -> R<Val> {
    match f.stack.pop() {
        Some(v) => Ok(v),
        None => reject("操作数栈下溢".into()),
    }
}

fn pop_refs(f: &mut Frame) -> R<Refs> {
    Ok(pop(f)?.refs())
}

fn pop_prim(f: &mut Frame) -> R<Prim> {
    Ok(pop(f)?.prim())
}

/// 规范化指令
pub fn step<'a>(a: &mut Analysis<'_, 'a>, ci: &'a ClassInfo, insn: &NInsn, mut f: Frame) -> R<Out> {
    match insn {
        NInsn::Op(i) => exec(a, ci, i, f),
        NInsn::FoldField { load, .. } => {
            pop(&mut f)?;
            exec(a, ci, load, f)
        }
        NInsn::FoldCall { call, load } => {
            let out = exec(a, ci, call, f)?;
            let Some(mut nf) = out.fall else { return Ok(Out::default()) };
            if returns_value(call) {
                pop(&mut nf)?;
            }
            exec(a, ci, load, nf)
        }
        NInsn::NoReturn { call } => {
            exec(a, ci, call, f)?;
            Ok(Out::default())
        }
        NInsn::NullRecv { .. } | NInsn::NoClassDef { .. } => Ok(Out::default()),
    }
}

fn returns_value(call: &Insn) -> bool {
    let desc = match &call.operand {
        Operand::Method(m, _) => &m.desc,
        Operand::InvokeDynamic { desc, .. } => desc,
        _ => return false,
    };
    method_kinds(desc).1.is_some()
}

fn local_index(insn: &Insn, base: u8) -> u16 {
    match insn.operand {
        Operand::Local(n) => n,
        _ => ((insn.opcode - base) % 4) as u16,
    }
}

fn get_local(f: &Frame, n: u16, k: Kind) -> Val {
    let v = f.locals.get(n as usize).cloned().unwrap_or(Val::Void);
    match (k, v) {
        (Kind::Ref, Val::Ref(r)) => Val::Ref(r),
        (Kind::Ref, Val::Live) => Val::old(),
        (Kind::Ref, _) => Val::null(),
        (Kind::Int, Val::Int(p)) => Val::Int(p),
        (Kind::Int, _) => Val::int_top(),
        (Kind::Wide, _) => Val::Wide,
    }
}

fn set_local(f: &mut Frame, n: u16, v: Val) {
    let n = n as usize;
    let wide = v.is_wide();
    let need = n + 1 + usize::from(wide);
    if f.locals.len() < need {
        f.locals.resize(need, Val::Void);
    }
    f.locals[n] = v;
    if wide {
        f.locals[n + 1] = Val::Void;
    }
}

fn fold_int(op: u8, x: i32, y: i32) -> Option<i32> {
    Some(match op {
        0x60 => x.wrapping_add(y),
        0x64 => x.wrapping_sub(y),
        0x68 => x.wrapping_mul(y),
        0x6c if y != 0 => x.wrapping_div(y),
        0x70 if y != 0 => x.wrapping_rem(y),
        0x78 => x.wrapping_shl(y as u32 & 31),
        0x7a => x.wrapping_shr(y as u32 & 31),
        0x7c => ((x as u32) >> (y as u32 & 31)) as i32,
        0x7e => x & y,
        0x80 => x | y,
        0x82 => x ^ y,
        _ => return None,
    })
}

fn cond(op: u8, x: i32, y: i32) -> bool {
    match op {
        0x99 | 0x9f => x == y,
        0x9a | 0xa0 => x != y,
        0x9b | 0xa1 => x < y,
        0x9c | 0xa2 => x >= y,
        0x9d | 0xa3 => x > y,
        _ => x <= y,
    }
}

/// 条件分支：`taken` 为 Some(b) 时只走一侧
fn branch(f: Frame, target: u32, taken: Option<bool>) -> Out {
    match taken {
        Some(true) => Out { jumps: vec![(target, f)], ..Out::default() },
        Some(false) => Out { fall: Some(f), ..Out::default() },
        None => Out { jumps: vec![(target, f.clone())], fall: Some(f), ..Out::default() },
    }
}

fn instance_of(a: &Analysis, r: &Refs, target: &str) -> Prim {
    let mut out = Prim::Bot;
    if r.is_empty() {
        return Prim::Const(0);
    }
    for n in r.iter() {
        let v = match &a.nodes[n as usize] {
            NodeKind::Alloc(c) | NodeKind::Made(c) if !c.starts_with('[') => a.w.is_subtype(c, target),
            NodeKind::Alloc(c) | NodeKind::Made(c) if c == target => true,
            NodeKind::Lambda(s) if a.w.is_subtype(&s.iface, target) => true,
            _ => return Prim::Top,
        };
        out = out.join(Prim::Const(v as i32));
    }
    out
}

fn array_desc(elem: &str) -> String {
    if elem.starts_with('[') {
        format!("[{elem}")
    } else {
        format!("[L{elem};")
    }
}

/// 单条 JVM 指令
pub fn exec<'a>(a: &mut Analysis<'_, 'a>, ci: &'a ClassInfo, insn: &Insn, mut f: Frame) -> R<Out> {
    let o = insn.opcode;
    let push = |f: &mut Frame, v: Val| f.stack.push(v);
    match o {
        0x00 => {}
        0x01 => push(&mut f, Val::null()),
        0x02..=0x08 => push(&mut f, Val::Int(Prim::Const(o as i32 - 3))),
        0x09 | 0x0a | 0x0e | 0x0f => push(&mut f, Val::Wide),
        0x0b..=0x0d => push(&mut f, Val::int_top()),
        0x10 | 0x11 => {
            let v = if let Operand::Int(v) = insn.operand { Prim::Const(v) } else { Prim::Top };
            push(&mut f, Val::Int(v));
        }
        op::LDC | op::LDC_W | op::LDC2_W => {
            let Operand::Ldc(c) = &insn.operand else { return reject("ldc 操作数".into()) };
            let v = match c {
                Const::Int(v) => Val::Int(Prim::Const(*v)),
                Const::Float(_) => Val::int_top(),
                Const::Long(_) | Const::Double(_) => Val::Wide,
                // 字面量驻留（挂起登记）与类镜像（全局缓存，挂起登记）都是区域外对象
                Const::String(_) | Const::StringUtf16(_) | Const::Class(_) => Val::old(),
                Const::MethodType(_) | Const::MethodHandle(_) | Const::Dynamic(..) => {
                    if a.shallow {
                        Val::old()
                    } else {
                        return reject(format!("ldc 方法类型 / 句柄 / 动态常量：{}", ci.name()));
                    }
                }
            };
            push(&mut f, v);
        }
        0x15..=0x19 => {
            let k = [Kind::Int, Kind::Wide, Kind::Int, Kind::Wide, Kind::Ref][(o - 0x15) as usize];
            let v = get_local(&f, local_index(insn, 0), k);
            push(&mut f, v);
        }
        0x1a..=0x2d => {
            let k = [Kind::Int, Kind::Wide, Kind::Int, Kind::Wide, Kind::Ref][((o - 0x1a) / 4) as usize];
            let v = get_local(&f, local_index(insn, 0x1a), k);
            push(&mut f, v);
        }
        0x2e..=0x35 => {
            pop(&mut f)?;
            let arr = pop_refs(&mut f)?;
            let v = match o {
                0x2f | 0x31 => Val::Wide,
                0x32 => Val::Ref(a.load(&arr, ELEM)),
                _ => Val::int_top(),
            };
            push(&mut f, v);
        }
        0x36..=0x3a => {
            let v = pop(&mut f)?;
            set_local(&mut f, local_index(insn, 0), v);
        }
        0x3b..=0x4e => {
            let v = pop(&mut f)?;
            set_local(&mut f, local_index(insn, 0x3b), v);
        }
        0x4f..=0x56 => {
            let v = pop(&mut f)?;
            pop(&mut f)?;
            let arr = pop_refs(&mut f)?;
            if o == op::AASTORE {
                a.store(&arr, ELEM, &v.refs(), &|| format!("aastore @{} {}", insn.offset, ci.name()))?;
            }
        }
        0x57 => {
            pop(&mut f)?;
        }
        0x58 => {
            if !pop(&mut f)?.is_wide() {
                pop(&mut f)?;
            }
        }
        0x59..=0x5f => stack_op(o, &mut f)?,
        0x60..=0x83 => arith(o, &mut f)?,
        op::IINC => {
            if let Operand::Iinc { index, delta } = insn.operand {
                let v = match get_local(&f, index, Kind::Int).prim() {
                    Prim::Const(x) => Prim::Const(x.wrapping_add(delta as i32)),
                    p => p.join(Prim::Top),
                };
                set_local(&mut f, index, Val::Int(v));
            }
        }
        0x85..=0x93 => {
            let v = pop(&mut f)?;
            let r = match o {
                0x85 | 0x87 | 0x8a | 0x8c | 0x8d | 0x8f => Val::Wide,
                0x91 => match v.prim() {
                    Prim::Const(x) => Val::Int(Prim::Const(x as i8 as i32)),
                    _ => Val::int_top(),
                },
                0x92 => match v.prim() {
                    Prim::Const(x) => Val::Int(Prim::Const(x as u16 as i32)),
                    _ => Val::int_top(),
                },
                0x93 => match v.prim() {
                    Prim::Const(x) => Val::Int(Prim::Const(x as i16 as i32)),
                    _ => Val::int_top(),
                },
                _ => Val::int_top(),
            };
            push(&mut f, r);
        }
        0x94..=0x98 => {
            pop(&mut f)?;
            pop(&mut f)?;
            push(&mut f, Val::int_top());
        }
        0x99..=0x9e | 0x9f..=0xa4 | 0xa5 | 0xa6 | op::IFNULL | op::IFNONNULL => {
            let Operand::Branch(t) = insn.operand else { return reject("分支操作数".into()) };
            let taken = match o {
                0x99..=0x9e => match pop_prim(&mut f)? {
                    Prim::Bot => return Ok(Out::default()),
                    Prim::Const(x) => Some(cond(o, x, 0)),
                    Prim::Top => None,
                },
                0x9f..=0xa4 => {
                    let y = pop_prim(&mut f)?;
                    let x = pop_prim(&mut f)?;
                    match (x, y) {
                        (Prim::Bot, _) | (_, Prim::Bot) => return Ok(Out::default()),
                        (Prim::Const(x), Prim::Const(y)) => Some(cond(o, x, y)),
                        _ => None,
                    }
                }
                0xa5 | 0xa6 => {
                    let y = pop_refs(&mut f)?;
                    let x = pop_refs(&mut f)?;
                    (x.is_empty() && y.is_empty()).then_some(o == 0xa5)
                }
                _ => {
                    let r = pop_refs(&mut f)?;
                    r.is_empty().then_some(o == op::IFNULL)
                }
            };
            return Ok(branch(f, t, taken));
        }
        op::GOTO | op::GOTO_W => {
            let Operand::Branch(t) = insn.operand else { return reject("分支操作数".into()) };
            return Ok(Out { jumps: vec![(t, f)], ..Out::default() });
        }
        op::JSR | op::JSR_W | op::RET => return reject("jsr / ret".into()),
        op::TABLESWITCH | op::LOOKUPSWITCH => {
            let k = pop_prim(&mut f)?;
            let targets: Vec<u32> = match (&insn.operand, k) {
                (_, Prim::Bot) => Vec::new(),
                (Operand::TableSwitch { default, low, targets, .. }, Prim::Const(x)) => {
                    let i = x.wrapping_sub(*low);
                    vec![if i >= 0 && (i as usize) < targets.len() { targets[i as usize] } else { *default }]
                }
                (Operand::LookupSwitch { default, pairs }, Prim::Const(x)) => {
                    vec![pairs.iter().find(|p| p.0 == x).map(|p| p.1).unwrap_or(*default)]
                }
                (op, _) => input::norm::switch_targets(op),
            };
            return Ok(Out { jumps: targets.into_iter().map(|t| (t, f.clone())).collect(), ..Out::default() });
        }
        0xac..=0xb0 => {
            let v = pop(&mut f)?;
            return Ok(Out { ret: Some(v), ..Out::default() });
        }
        op::RETURN => return Ok(Out { ret: Some(Val::Void), ..Out::default() }),
        op::GETSTATIC => {
            let Operand::Field(m) = &insn.operand else { return reject("字段操作数".into()) };
            let v = match field_kind(&m.desc) {
                Kind::Ref => Val::old(),
                Kind::Int => Val::int_top(),
                Kind::Wide => Val::Wide,
            };
            push(&mut f, v);
        }
        op::PUTSTATIC => {
            let Operand::Field(m) = &insn.operand else { return reject("字段操作数".into()) };
            let v = pop(&mut f)?;
            if v.refs().has_fresh() && !a.shallow {
                return reject(format!("区域对象存入静态字段：{m}"));
            }
        }
        op::GETFIELD => {
            let Operand::Field(m) = &insn.operand else { return reject("字段操作数".into()) };
            let r = pop_refs(&mut f)?;
            let v = match field_kind(&m.desc) {
                Kind::Ref => {
                    let k = a.slot(&m.name);
                    Val::Ref(a.load(&r, k))
                }
                Kind::Int => Val::Int(a.get_int_field(&r, &m.name, &m.desc)),
                Kind::Wide => Val::Wide,
            };
            push(&mut f, v);
        }
        op::PUTFIELD => {
            let Operand::Field(m) = &insn.operand else { return reject("字段操作数".into()) };
            let v = pop(&mut f)?;
            let r = pop_refs(&mut f)?;
            match field_kind(&m.desc) {
                Kind::Ref => {
                    let k = a.slot(&m.name);
                    a.store(&r, k, &v.refs(), &|| format!("{m}"))?
                }
                Kind::Int => a.put_int_field(&r, &m.name, &m.desc, v.prim()),
                Kind::Wide => {}
            }
        }
        op::INVOKEVIRTUAL..=op::INVOKEINTERFACE => {
            if !a.invoke(insn, &mut f)? {
                return Ok(Out::default());
            }
        }
        op::INVOKEDYNAMIC => {
            if !crate::indy::invoke_dynamic(a, ci, insn, &mut f)? {
                return Ok(Out::default());
            }
        }
        op::NEW => {
            let Operand::Class(c) = &insn.operand else { return reject("new 操作数".into()) };
            let n = a.node(NodeKind::Alloc(c.clone()));
            push(&mut f, Val::Ref(Refs::single(n)));
        }
        op::NEWARRAY | op::ANEWARRAY => {
            pop(&mut f)?;
            let desc = match &insn.operand {
                Operand::NewArray(t) => {
                    let c = b"ZCFDBSIJ".get((*t as usize).wrapping_sub(4)).copied().unwrap_or(b'I');
                    format!("[{}", c as char)
                }
                Operand::Class(c) => array_desc(c),
                _ => return reject("数组分配操作数".into()),
            };
            let n = a.node(NodeKind::Alloc(desc));
            push(&mut f, Val::Ref(Refs::single(n)));
        }
        op::MULTIANEWARRAY => {
            let Operand::MultiANewArray(desc, dims) = &insn.operand else { return reject("多维数组操作数".into()) };
            for _ in 0..*dims {
                pop(&mut f)?;
            }
            let mut d = desc.as_str();
            let top = a.node(NodeKind::Alloc(d.to_string()));
            let mut cur = top;
            for _ in 1..*dims {
                d = &d[1..];
                let inner = a.node(NodeKind::Alloc(d.to_string()));
                a.store(&Refs::single(cur), ELEM, &Refs::single(inner), &String::new)?;
                cur = inner;
            }
            push(&mut f, Val::Ref(Refs::single(top)));
        }
        op::ARRAYLENGTH => {
            pop(&mut f)?;
            push(&mut f, Val::int_top());
        }
        op::ATHROW => {
            let r = pop_refs(&mut f)?;
            a.throw(&r);
            return Ok(Out::default());
        }
        op::CHECKCAST => {}
        op::INSTANCEOF => {
            let Operand::Class(c) = &insn.operand else { return reject("instanceof 操作数".into()) };
            let r = pop_refs(&mut f)?;
            push(&mut f, Val::Int(instance_of(a, &r, c)));
        }
        op::MONITORENTER | op::MONITOREXIT => {
            pop(&mut f)?;
        }
        _ => return reject(format!("未知操作码 0x{o:02x}")),
    }
    Ok(Out { fall: Some(f), ..Out::default() })
}

/// dup 族 / swap（宽值占一个栈项、计算类型 2）
fn stack_op(o: u8, f: &mut Frame) -> R<()> {
    let v1 = pop(f)?;
    match o {
        0x59 => {
            f.stack.push(v1.clone());
            f.stack.push(v1);
        }
        0x5a => {
            let v2 = pop(f)?;
            f.stack.extend([v1.clone(), v2, v1]);
        }
        0x5b => {
            let v2 = pop(f)?;
            if v2.is_wide() {
                f.stack.extend([v1.clone(), v2, v1]);
            } else {
                let v3 = pop(f)?;
                f.stack.extend([v1.clone(), v3, v2, v1]);
            }
        }
        0x5c => {
            if v1.is_wide() {
                f.stack.extend([v1.clone(), v1]);
            } else {
                let v2 = pop(f)?;
                f.stack.extend([v2.clone(), v1.clone(), v2, v1]);
            }
        }
        0x5d => {
            let v2 = pop(f)?;
            if v1.is_wide() {
                f.stack.extend([v1.clone(), v2, v1]);
            } else {
                let v3 = pop(f)?;
                f.stack.extend([v2.clone(), v1.clone(), v3, v2, v1]);
            }
        }
        0x5e => {
            let v2 = pop(f)?;
            match (v1.is_wide(), v2.is_wide()) {
                (true, true) => f.stack.extend([v1.clone(), v2, v1]),
                (true, false) => {
                    let v3 = pop(f)?;
                    f.stack.extend([v1.clone(), v3, v2, v1]);
                }
                (false, _) => {
                    let v3 = pop(f)?;
                    if v3.is_wide() {
                        f.stack.extend([v2.clone(), v1.clone(), v3, v2, v1]);
                    } else {
                        let v4 = pop(f)?;
                        f.stack.extend([v2.clone(), v1.clone(), v4, v3, v2, v1]);
                    }
                }
            }
        }
        _ => {
            let v2 = pop(f)?;
            f.stack.extend([v1, v2]);
        }
    }
    Ok(())
}

/// 算术 / 位运算：int 常量折叠，其余结果为 ⊤ / 宽值
fn arith(o: u8, f: &mut Frame) -> R<()> {
    let unary = (0x74..=0x77).contains(&o);
    let y = pop(f)?;
    let x = if unary { None } else { Some(pop(f)?) };
    let int_family = match o {
        0x60..=0x77 => (o - 0x60) % 4 == 0,
        _ => (o - 0x78) % 2 == 0,
    };
    let float_family = matches!(o, 0x60..=0x77) && (o - 0x60) % 4 == 2;
    let v = if int_family {
        match (x.map(|v| v.prim()), y.prim()) {
            (None, Prim::Const(c)) => Prim::Const(c.wrapping_neg()),
            (Some(Prim::Const(a)), Prim::Const(b)) => fold_int(o, a, b).map(Prim::Const).unwrap_or(Prim::Top),
            _ => Prim::Top,
        }
    } else {
        Prim::Top
    };
    f.stack.push(if int_family || float_family { Val::Int(v) } else { Val::Wide });
    Ok(())
}
