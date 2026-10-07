//! 单条指令的抽象执行（[`Interp::step`]）与事件发出。

use super::*;

impl<'a, O: Oracle> Interp<'a, O> {
    pub(super) fn ev(&mut self, off: u32, e: Event) {
        if let Some(out) = self.emit.as_deref_mut() {
            out.push((off, e));
        }
    }

    /// 读取点折叠出的常量：登记折叠事件，原样返回
    /// 带对象标签的引用不是可导出的常量：来源换成本读取点，不登记折叠事件
    pub(super) fn folded(&mut self, opcode: u8, off: u32, v: Option<V>) -> Option<V> {
        let v = v?;
        if let V::Ref { .. } = v {
            return Some(v.rebased(Src::Site(off)));
        }
        self.ev(off, Event::Const { opcode, value: v.stripped() });
        Some(v.rebased(Src::Site(off)))
    }

    /// 构造器返回：新建对象（`Uninit` 标签）的各份拷贝换成构造完成的标签（final 字段常量，推不出则无标签）
    pub(super) fn constructed(&mut self, s: &mut State, init: &MemberRef, args: &[V]) {
        let Some(recv @ V::Ref { obj: Some(o), .. }) = args.first() else { return };
        if **o != Obj::Uninit {
            return;
        }
        let done = match self.oracle.str_kind(op::INVOKESPECIAL, init, false) {
            Some(StrKind::Init) => parse_method(&init.desc).and_then(|md| strs::init_tag(&md.params, args)),
            _ => self.oracle.construct(init, args),
        };
        let V::Ref { ty, nonnull, src, .. } = recv else { return };
        let v = V::Ref { ty: ty.clone(), nonnull: *nonnull, src: src.clone(), obj: done };
        for x in s.locals.iter_mut().chain(s.stack.iter_mut()) {
            if x == recv {
                *x = v.clone();
            }
        }
    }

    pub(super) fn step(&mut self, st: &mut State, ins: &Insn) -> Step {
        let opc = ins.opcode;
        let off = ins.offset;
        let s = &mut *st;
        match opc {
            0x00 => {}
            0x01 => s.stack.push(V::Null),
            0x02..=0x08 => s.stack.push(V::Int(opc as i32 - 3)),
            0x09 | 0x0a => {
                s.stack.push(V::Long(opc as i64 - 9));
                s.stack.push(V::Hi);
            }
            0x0b..=0x0d => s.stack.push(V::Top),
            0x0e | 0x0f => {
                s.stack.push(V::Top);
                s.stack.push(V::Hi);
            }
            0x10 | 0x11 => match ins.operand {
                Operand::Int(v) => s.stack.push(V::Int(v)),
                _ => return Err(()),
            },
            op::LDC | op::LDC_W | op::LDC2_W => {
                let Operand::Ldc(c) = &ins.operand else { return Err(()) };
                match c {
                    Const::Int(v) => s.stack.push(V::Int(*v)),
                    Const::Float(_) => s.stack.push(V::Top),
                    Const::Long(v) => {
                        s.stack.push(V::Long(*v));
                        s.stack.push(V::Hi);
                    }
                    Const::Double(_) => {
                        s.stack.push(V::Top);
                        s.stack.push(V::Hi);
                    }
                    Const::String(x) => s.stack.push(V::lit(Rc::from(x.as_str()))),
                    // 含孤立代理项：值不入常量格（格上字符串为 Rust 文本，无法无损表示），按非空 String 站点值
                    Const::StringUtf16(_) => s.stack.push(site_ref(STRING, true, off)),
                    Const::Class(x) => s.stack.push(V::Class(Rc::from(x.as_str()), off)),
                    Const::MethodType(_) => s.stack.push(site_ref("java/lang/invoke/MethodType", true, off)),
                    Const::MethodHandle(_) => s.stack.push(site_ref("java/lang/invoke/MethodHandle", true, off)),
                    Const::Dynamic(_, _, d) => {
                        let ft = parse_field(d).ok_or(())?;
                        push_typed(&mut s.stack, &ft, value_of(&ft, Src::Site(off)));
                    }
                }
                if matches!(c, Const::String(_) | Const::StringUtf16(_) | Const::Class(_) | Const::MethodType(_) | Const::MethodHandle(_) | Const::Dynamic(..)) {
                    self.ev(off, Event::Ldc(c.clone()));
                }
            }
            // xload
            0x15..=0x19 | 0x1a..=0x2d => {
                let (kind, idx) = if opc <= 0x19 {
                    let Operand::Local(i) = ins.operand else { return Err(()) };
                    (opc - 0x15, i as usize)
                } else {
                    ((opc - 0x1a) / 4, ((opc - 0x1a) % 4) as usize)
                };
                let v = s.locals.get(idx).cloned().ok_or(())?;
                s.stack.push(v);
                if kind == 1 || kind == 3 {
                    s.stack.push(V::Hi);
                }
            }
            0x2e | 0x30 | 0x33 | 0x34 | 0x35 => {
                popn(s, 2)?;
                s.stack.push(V::Top);
            }
            0x2f | 0x31 => {
                popn(s, 2)?;
                s.stack.push(V::Top);
                s.stack.push(V::Hi);
            }
            0x32 => {
                let index = pop(s)?;
                let arr = pop(s)?;
                let ty = arr.static_type().and_then(component);
                self.ev(off, Event::ArrayLoad { array: arr, index });
                s.stack.push(V::Ref { ty, nonnull: false, src: src1(Src::Site(off)), obj: None });
            }
            // xstore
            0x36..=0x3a | 0x3b..=0x4e => {
                let (kind, idx) = if opc <= 0x3a {
                    let Operand::Local(i) = ins.operand else { return Err(()) };
                    (opc - 0x36, i as usize)
                } else {
                    ((opc - 0x3b) / 4, ((opc - 0x3b) % 4) as usize)
                };
                let wide = kind == 1 || kind == 3;
                if wide {
                    pop(s)?;
                }
                let v = pop(s)?;
                let need = idx + if wide { 2 } else { 1 };
                if s.locals.len() < need {
                    return Err(());
                }
                s.locals[idx] = v;
                if wide {
                    s.locals[idx + 1] = V::Hi;
                }
                if !s.nnf.is_empty() {
                    s.nnf.retain(|(k, _)| *k != idx && !(wide && *k == idx + 1));
                }
            }
            0x4f | 0x51 | 0x54 | 0x55 | 0x56 => popn(s, 3)?,
            0x50 | 0x52 => popn(s, 4)?,
            0x53 => {
                let value = pop(s)?;
                let index = pop(s)?;
                let array = pop(s)?;
                strs::escape(s, &value);
                self.ev(off, Event::ArrayStore { array, index, value });
            }
            0x57 => popn(s, 1)?,
            0x58 => popn(s, 2)?,
            0x59..=0x5f => {
                let n = s.stack.len();
                let need = match opc {
                    0x59 => 1,
                    0x5a | 0x5c | 0x5f => 2,
                    0x5b | 0x5d => 3,
                    _ => 4,
                };
                if n < need {
                    return Err(());
                }
                let top: Vec<V> = s.stack[n - need..].to_vec();
                s.stack.truncate(n - need);
                let seq: Vec<usize> = match opc {
                    0x59 => vec![0, 0],             // dup: a → a a
                    0x5a => vec![1, 0, 1],          // dup_x1: b a → a b a
                    0x5b => vec![2, 0, 1, 2],       // dup_x2: c b a → a c b a
                    0x5c => vec![0, 1, 0, 1],       // dup2: b a → b a b a
                    0x5d => vec![1, 2, 0, 1, 2],    // dup2_x1: c b a → b a c b a
                    0x5e => vec![2, 3, 0, 1, 2, 3], // dup2_x2: d c b a → b a d c b a
                    _ => vec![1, 0],                // swap: b a → a b
                };
                s.stack.extend(seq.into_iter().map(|i| top[i].clone()));
            }
            // int 二元
            0x60 | 0x64 | 0x68 | 0x6c | 0x70 | 0x78 | 0x7a | 0x7c | 0x7e | 0x80 | 0x82 => {
                let b = pop(s)?;
                let a = pop(s)?;
                s.stack.push(match (a, b) {
                    (V::Int(a), V::Int(b)) => int_bin(opc, a, b).map_or(V::Top, V::Int),
                    (a, b) => match ints::map2(&a, &b, |x, y| int_bin(opc, x, y)) {
                        Some(v) => v,
                        None => int_bin_parity(opc, &a, &b).map_or(V::Top, V::Par),
                    },
                });
            }
            // long 二元（含移位：long, int）
            0x61 | 0x65 | 0x69 | 0x6d | 0x71 | 0x7f | 0x81 | 0x83 => {
                popn(s, 4)?;
                s.stack.push(V::Top);
                s.stack.push(V::Hi);
            }
            0x79 | 0x7b | 0x7d => {
                popn(s, 3)?;
                s.stack.push(V::Top);
                s.stack.push(V::Hi);
            }
            // float 二元
            0x62 | 0x66 | 0x6a | 0x6e | 0x72 => {
                popn(s, 2)?;
                s.stack.push(V::Top);
            }
            // double 二元
            0x63 | 0x67 | 0x6b | 0x6f | 0x73 => {
                popn(s, 4)?;
                s.stack.push(V::Top);
                s.stack.push(V::Hi);
            }
            0x74 => {
                let a = pop(s)?;
                s.stack.push(match a {
                    V::Int(a) => V::Int(a.wrapping_neg()),
                    V::Par(p) => V::Par(p),
                    _ => V::Top,
                });
            }
            0x75 | 0x77 => {
                popn(s, 2)?;
                s.stack.push(V::Top);
                s.stack.push(V::Hi);
            }
            0x76 => {
                pop(s)?;
                s.stack.push(V::Top);
            }
            op::IINC => {
                let Operand::Iinc { index, delta } = ins.operand else { return Err(()) };
                let slot = s.locals.get_mut(index as usize).ok_or(())?;
                *slot = match slot {
                    V::Int(v) => V::Int(v.wrapping_add(delta as i32)),
                    V::Par(p) => V::Par(*p ^ (delta & 1 != 0)),
                    _ => V::Top,
                };
            }
            // 类型转换：(弹出槽数, 压入槽数)
            0x85..=0x93 => {
                let (inn, out) = match opc {
                    0x85 | 0x87 | 0x8c | 0x8d => (1, 2),
                    0x86 | 0x8b | 0x91..=0x93 => (1, 1),
                    0x88 | 0x89 | 0x8e | 0x90 => (2, 1),
                    _ => (2, 2),
                };
                let top = s.stack.len().checked_sub(inn).ok_or(())?;
                let a = s.stack[top].clone();
                s.stack.truncate(top);
                let v = match (opc, a) {
                    (0x85, V::Int(x)) => V::Long(x as i64),
                    (0x88, V::Long(x)) => V::Int(x as i32),
                    (0x91, V::Int(x)) => V::Int(x as i8 as i32),
                    (0x92, V::Int(x)) => V::Int(x as u16 as i32),
                    (0x93, V::Int(x)) => V::Int(x as i16 as i32),
                    _ => V::Top,
                };
                s.stack.push(v);
                if out == 2 {
                    s.stack.push(V::Hi);
                }
            }
            0x94 => {
                pop(s)?;
                let b = pop(s)?;
                pop(s)?;
                let a = pop(s)?;
                s.stack.push(match (a, b) {
                    (V::Long(a), V::Long(b)) => V::Int(a.cmp(&b) as i32),
                    _ => V::Top,
                });
            }
            0x95 | 0x96 => {
                popn(s, 2)?;
                s.stack.push(V::Top);
            }
            0x97 | 0x98 => {
                popn(s, 4)?;
                s.stack.push(V::Top);
            }
            0x99..=0x9e => {
                let a = pop(s)?;
                self.select(&a);
                let Operand::Branch(t) = ins.operand else { return Err(()) };
                let k = ints::decide(&a, &V::Int(0), |x, y| cond(opc, x, y));
                return Ok(Flow::Cond(t, k));
            }
            0x9f..=0xa4 => {
                let b = pop(s)?;
                let a = pop(s)?;
                self.select(&a);
                self.select(&b);
                let Operand::Branch(t) = ins.operand else { return Err(()) };
                let k = ints::decide(&a, &b, |x, y| cond(opc, x, y));
                return Ok(Flow::Cond(t, k));
            }
            0xa5 | 0xa6 => {
                let b = pop(s)?;
                let a = pop(s)?;
                let Operand::Branch(t) = ins.operand else { return Err(()) };
                let eq = self.ref_eq(&a, &b);
                return Ok(Flow::Cond(t, eq.map(|e| if opc == 0xa5 { e } else { !e })));
            }
            op::GOTO | op::GOTO_W => {
                let Operand::Branch(t) = ins.operand else { return Err(()) };
                return Ok(Flow::Goto(t));
            }
            op::JSR | op::RET | op::JSR_W => return Err(()),
            op::TABLESWITCH | op::LOOKUPSWITCH => {
                let key = pop(s)?;
                self.select(&key);
                let (default, cases): (u32, Vec<(i32, u32)>) = match &ins.operand {
                    Operand::TableSwitch { default, low, targets, .. } => {
                        (*default, targets.iter().enumerate().map(|(i, t)| (low + i as i32, *t)).collect())
                    }
                    Operand::LookupSwitch { default, pairs } => (*default, pairs.clone()),
                    _ => return Err(()),
                };
                let mut all: Vec<u32> = cases.iter().map(|c| c.1).collect();
                all.push(default);
                all.sort();
                all.dedup();
                if let Some(ks) = ints::members(&key) {
                    let mut ts: Vec<u32> = ks.iter().map(|k| cases.iter().find(|c| c.0 == *k).map_or(default, |c| c.1)).collect();
                    ts.sort();
                    ts.dedup();
                    return Ok(Flow::Switch(ts));
                }
                return Ok(Flow::Switch(all));
            }
            0xb0 => {
                let v = pop(s)?;
                self.ev(off, Event::Return(v));
                return Ok(Flow::End);
            }
            0xac | 0xae => {
                let v = pop(s)?;
                self.ev(off, Event::Return(if opc == 0xac { v } else { V::Top }));
                return Ok(Flow::End);
            }
            0xad | 0xaf => {
                pop(s)?;
                let v = pop(s)?;
                self.ev(off, Event::Return(if opc == 0xad { v } else { V::Top }));
                return Ok(Flow::End);
            }
            op::RETURN => {
                self.ev(off, Event::Return(V::Top));
                return Ok(Flow::End);
            }
            op::GETSTATIC | op::PUTSTATIC | op::GETFIELD | op::PUTFIELD => {
                let Operand::Field(f) = &ins.operand else { return Err(()) };
                let ft = parse_field(&f.desc).ok_or(())?;
                let mut value = None;
                let mut recv = None;
                match opc {
                    op::GETSTATIC => {
                        let own = s.finals.iter().find(|(g, _)| g == f).map(|(_, v)| v.clone());
                        let v = own.or_else(|| self.folded(opc, off, self.oracle.field(opc, f, None)).map(|v| typed(v, &ft))).unwrap_or_else(|| value_of(&ft, Src::Site(off)));
                        push_typed(&mut s.stack, &ft, v);
                    }
                    op::PUTSTATIC => {
                        if ft.slots() == 2 {
                            pop(s)?;
                        }
                        let v = pop(s)?;
                        strs::escape(s, &v);
                        if self.oracle.final_static(f) {
                            s.finals.retain(|(g, _)| g != f);
                            s.finals.push((f.clone(), v.clone()));
                            strs::escape(s, &v);
                        }
                        value = Some(v);
                    }
                    op::GETFIELD => {
                        recv = Some(pop(s)?);
                        let known = match self.oracle.getfield(f, recv.as_ref()) {
                            Ret::Never => {
                                self.ev(off, Event::Field { opcode: opc, mref: f.clone(), recv, value: None });
                                return Ok(Flow::End);
                            }
                            Ret::Value(v) => Some(v),
                            Ret::Unknown => None,
                        };
                        let known = known.or_else(|| self.param_mirror_field(recv.as_ref(), f));
                        let v = self.folded(opc, off, known).map(|v| typed(v, &ft)).unwrap_or_else(|| value_of(&ft, Src::Site(off)));
                        push_typed(&mut s.stack, &ft, v);
                    }
                    _ => {
                        if ft.slots() == 2 {
                            pop(s)?;
                        }
                        value = Some(pop(s)?);
                        recv = Some(pop(s)?);
                        if let Some(v) = &value {
                            strs::escape(s, v);
                        }
                        s.nnf.retain(|(_, g)| g != f);
                    }
                }
                self.ev(off, Event::Field { opcode: opc, mref: f.clone(), recv, value });
            }
            op::INVOKEVIRTUAL | op::INVOKESPECIAL | op::INVOKESTATIC | op::INVOKEINTERFACE => {
                let Operand::Method(m, iface) = &ins.operand else { return Err(()) };
                let md = parse_method(&m.desc).ok_or(())?;
                let mut args = pop_args(s, &md.params)?;
                if opc != op::INVOKESTATIC {
                    let recv = pop(s)?;
                    args.insert(0, recv);
                }
                s.nnf.clear();
                let kind = self.oracle.str_kind(opc, m, *iface);
                let retag = strs::invoke(s, kind, &md.params, &args, opc == op::INVOKESTATIC);
                let r = match self.oracle.invoke_result(opc, m, *iface, &args) {
                    Ret::Unknown if opc == op::INVOKEVIRTUAL => self.param_mirror_call(args.first(), m).map_or(Ret::Unknown, Ret::Value),
                    r => r,
                };
                if opc == op::INVOKESPECIAL && m.name == "<init>" {
                    self.constructed(s, m, &args);
                }
                self.ev(off, Event::Invoke { opcode: opc, mref: m.clone(), iface: *iface, args });
                let v = match r {
                    Ret::Never => return Ok(Flow::End),
                    Ret::Value(v) => Some(v),
                    Ret::Unknown => None,
                };
                if let Some(ret) = &md.ret {
                    // 返回常量格的非空引用不带类型：补上声明返回类型
                    let v = match self.folded(opc, off, v) {
                        Some(V::Ref { ty: None, nonnull, src, obj }) => V::Ref { ty: Some(ft_name(ret)), nonnull, src, obj },
                        Some(v) => v,
                        None => value_of(ret, Src::Site(off)),
                    };
                    // 构建器追加 / 取结果：结果换成内容标签（常量结果保持常量）
                    let v = match (retag, &v) {
                        (Some(tag), V::Ref { ty, nonnull, src, obj }) if tag.is_some() || matches!(obj.as_deref(), Some(Obj::Builder { .. })) => {
                            let nonnull = *nonnull || tag.is_some();
                            V::Ref { ty: ty.clone(), nonnull, src: src.clone(), obj: tag }
                        }
                        _ => v,
                    };
                    push_typed(&mut s.stack, ret, v);
                }
            }
            op::INVOKEDYNAMIC => {
                let Operand::InvokeDynamic { bsm, name, desc, .. } = &ins.operand else { return Err(()) };
                let md = parse_method(desc).ok_or(())?;
                let args = pop_args(s, &md.params)?;
                s.nnf.clear();
                for a in &args {
                    strs::escape(s, a);
                }
                if let Some(ret) = &md.ret {
                    push_typed(&mut s.stack, ret, value_of(ret, Src::Site(off)));
                }
                self.ev(off, Event::Indy { bsm: *bsm, name: name.clone(), desc: desc.clone(), args });
            }
            op::NEW => {
                let Operand::Class(c) = &ins.operand else { return Err(()) };
                // 再次执行分配点：旧对象的构建器标签整组撤掉（新对象独占该组）
                strs::drop_group(s, off);
                s.stack.push(V::Ref { ty: Some(Rc::from(c.as_str())), nonnull: true, src: src1(Src::Site(off)), obj: Some(Rc::new(Obj::Uninit)) });
                self.ev(off, Event::New(c.clone()));
            }
            op::NEWARRAY | op::ANEWARRAY => {
                let len = pop(s)?;
                let empty = len == V::Int(0);
                let ty = match &ins.operand {
                    Operand::NewArray(t) => {
                        let c = b"ZCFDBSIJ".get((*t as usize).wrapping_sub(4)).ok_or(())?;
                        format!("[{}", *c as char)
                    }
                    Operand::Class(c) if c.starts_with('[') => format!("[{c}"),
                    Operand::Class(c) => format!("[L{c};"),
                    _ => return Err(()),
                };
                // 常量长度：数组带长度标签（长度不可变，`arraylength` 按标签折叠）
                let obj = match len {
                    V::Int(n) if n >= 0 => Some(Rc::new(Obj::Len(n))),
                    _ => None,
                };
                s.stack.push(V::Ref { ty: Some(Rc::from(ty.as_str())), nonnull: true, src: src1(Src::Site(off)), obj });
                self.ev(off, Event::NewArray(ty, empty));
            }
            // arraylength：常量长度标签的数组折叠为该长度
            0xbe => {
                let a = pop(s)?;
                s.stack.push(match a.obj().map(|o| &**o) {
                    Some(&Obj::Len(n)) => V::Int(n),
                    _ => V::Top,
                });
            }
            op::ATHROW => {
                let v = pop(s)?;
                self.ev(off, Event::Throw(v));
                return Ok(Flow::End);
            }
            op::CHECKCAST => {
                let Operand::Class(c) = &ins.operand else { return Err(()) };
                let v = pop(s)?;
                let (out, input) = match v {
                    V::Null => (V::Null, None),
                    V::Str(..) | V::Class(..) => (v, None),
                    // 数组目标：来源不变（数组类型不参与收窄）
                    V::Ref { nonnull, src, obj, .. } if c.starts_with('[') => (V::Ref { ty: Some(Rc::from(c.as_str())), nonnull, src, obj }, None),
                    // 类目标：结果以本偏移为来源，跨汇合点仍保留按来源的收窄
                    V::Ref { nonnull, ref obj, .. } => {
                        let obj = obj.clone();
                        (V::Ref { ty: Some(Rc::from(c.as_str())), nonnull, src: src1(Src::Site(off)), obj }, Some(v))
                    }
                    // 未知值（保守）：来源仍未知
                    other => (other, None),
                };
                s.stack.push(out);
                self.ev(off, Event::CheckCast(c.clone(), input));
            }
            op::INSTANCEOF => {
                let Operand::Class(c) = &ins.operand else { return Err(()) };
                let v = pop(s)?;
                // 目标类型（非数组）无已实例化子类型时恒为 false：值只可能是 null 或其它类型的对象
                let dead = matches!(v, V::Ref { .. }) && !c.starts_with('[') && !self.oracle.type_live(c);
                s.stack.push(if v == V::Null || dead { V::Int(0) } else { V::Top });
                let input = (matches!(v, V::Ref { .. }) && !c.starts_with('[')).then_some(v);
                self.ev(off, Event::InstanceOf(c.clone(), input));
            }
            0xc2 | 0xc3 => popn(s, 1)?,
            op::MULTIANEWARRAY => {
                let Operand::MultiANewArray(c, dims) = &ins.operand else { return Err(()) };
                popn(s, *dims as usize)?;
                s.stack.push(site_ref(c, true, off));
                self.ev(off, Event::NewArray(c.clone(), false));
            }
            op::IFNULL | op::IFNONNULL => {
                let a = pop(s)?;
                let Operand::Branch(t) = ins.operand else { return Err(()) };
                let k = a.nonnull().map(|nn| if opc == op::IFNULL { !nn } else { nn });
                if k.is_none() {
                    if let Some(i) = a.param_ref() {
                        self.selects |= 1 << i;
                    }
                }
                return Ok(Flow::Cond(t, k));
            }
            _ => return Err(()),
        }
        Ok(Flow::Next)
    }
}
