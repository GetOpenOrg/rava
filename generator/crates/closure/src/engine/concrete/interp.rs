//! 字节码解释（JVMS §6.5）：帧、操作数栈、异常表。
//!
//! 异常：athrow 抛出的对象与隐式异常（空指针、越界、类型转换、除零、数组存储、负数组长度，见 `init.rs`）
//! 都按异常表查找处理器；隐式异常的类型取自清单 `[concrete.implicit]`。

use classfile::{Insn, Operand};

use super::vm::*;
use super::*;

enum Next {
    Fall,
    Jump(u32),
    Ret(Option<CV>),
}

/// 描述符的形参个数（每个形参在操作数栈上占一项）
pub(super) fn nparams(desc: &str) -> usize {
    let b = desc.as_bytes();
    let (mut i, mut n) = (1, 0);
    while i < b.len() && b[i] != b')' {
        while b[i] == b'[' {
            i += 1;
        }
        if b[i] == b'L' {
            while b[i] != b';' {
                i += 1;
            }
        }
        i += 1;
        n += 1;
    }
    n
}

pub(super) fn returns_void(desc: &str) -> bool {
    desc.ends_with(")V")
}

/// 数组类型的元素描述符（`[I` → `I`，`[Ljava/lang/String;` → `Ljava/lang/String;`）
fn elem(ty: &str) -> &str {
    &ty[1..]
}

/// `anewarray` / `checkcast` 操作数（binary name 或数组描述符）对应的数组类型
pub(super) fn array_of(c: &str) -> String {
    if c.starts_with('[') || c.len() == 1 {
        format!("[{c}")
    } else {
        format!("[L{c};")
    }
}

const NEWARRAY_DESC: [&str; 8] = ["Z", "C", "F", "D", "B", "S", "I", "J"];

impl Vm {
    /// 执行已选定的方法（白名单 native 按操作名、字节码按解释）
    pub(super) fn call(&mut self, env: &Env, site: &MethodSite, args: Vec<CV>) -> R<Option<CV>> {
        let info = self.info(env, site);
        if let Some(op) = &info.op {
            return super::natives::call(self, env, op, &info, args);
        }
        if !info.bytecode {
            return fail(format!("无具体语义 {}", info.key));
        }
        self.run(env, &info, args)
    }

    pub(super) fn run(&mut self, env: &Env, info: &Rc<MInfo>, args: Vec<CV>) -> R<Option<CV>> {
        if self.frames.len() >= DEPTH_LIMIT {
            return fail("调用深度超限");
        }
        let Some(code) = info.site.method().code.as_ref() else { return fail(format!("无字节码 {}", info.key)) };
        let mut locals = vec![CV::N; code.max_locals as usize];
        let mut i = 0;
        for a in args {
            if i >= locals.len() {
                return fail("实参超出局部变量表");
            }
            locals[i] = a;
            i += if a.wide() { 2 } else { 1 };
        }
        let tracing = self.tracing();
        let mut hits = if tracing { vec![false; code.insns.len()] } else { Vec::new() };
        self.frames.push(info.key.clone());
        let r = self.exec(env, info, &mut locals, &mut hits);
        self.frames.pop();
        if tracing {
            let pcs = self.trace.pcs.entry(info.key.clone()).or_default();
            pcs.extend(hits.iter().zip(&code.insns).filter(|(h, _)| **h).map(|(_, x)| x.offset));
        }
        r
    }

    fn exec(&mut self, env: &Env, info: &Rc<MInfo>, locals: &mut [CV], hits: &mut [bool]) -> R<Option<CV>> {
        let code = info.code();
        let tracing = !hits.is_empty();
        let mut stack: Vec<CV> = Vec::with_capacity(code.max_stack as usize + 1);
        let mut ix = 0usize;
        loop {
            self.steps += 1;
            if self.steps > STEP_LIMIT {
                return fail("步数超限");
            }
            let Some(insn) = code.insns.get(ix) else { return fail("越过方法末尾") };
            if tracing {
                hits[ix] = true;
            }
            let r = match self.step(env, info, insn, locals, &mut stack) {
                Err(Flow::Implicit(k)) => self.implicit(env, k).and_then(|o| Err(Flow::Throw(o))),
                r => r,
            };
            match r {
                Ok(Next::Fall) => ix += 1,
                Ok(Next::Jump(off)) => ix = *info.index.get(&off).map_or_else(|| fail("分支目标非指令边界"), Ok)?,
                Ok(Next::Ret(v)) => return Ok(v),
                Err(Flow::Throw(o)) => {
                    let ty = self.ty(o);
                    let h = code.exception_table.iter().find(|e| {
                        e.start <= insn.offset
                            && insn.offset < e.end
                            && e.catch_type.as_deref().is_none_or(|c| self.instance_of(env, &ty, o, c))
                    });
                    let Some(h) = h else { return Err(Flow::Throw(o)) };
                    stack.clear();
                    stack.push(CV::R(o));
                    ix = *info.index.get(&h.handler).map_or_else(|| fail("处理器非指令边界"), Ok)?;
                }
                // 失败位置：最内层的方法与偏移（外层不再追加）
                Err(Flow::Fail(w)) if !w.contains(" @ ") => return fail(format!("{w} @ {}@{}", info.key, insn.offset)),
                Err(f) => return Err(f),
            }
        }
    }

    /// 运行期类型 `ty`（对象 `o`）是否 `c` 的实例
    pub(super) fn instance_of(&self, env: &Env, ty: &str, o: u32, c: &str) -> bool {
        if let Body::Lam(l) = &self.heap[o as usize].body {
            return c == OBJECT || [&l.iface].into_iter().chain(&l.markers).any(|i| env.h().is_subtype(i, c));
        }
        env.h().is_subtype(ty, c)
    }

    fn step(&mut self, env: &Env, info: &Rc<MInfo>, insn: &Insn, locals: &mut [CV], st: &mut Vec<CV>) -> R<Next> {
        macro_rules! pop {
            () => {
                st.pop().map_or_else(|| fail("操作数栈下溢"), Ok)?
            };
        }
        macro_rules! bin {
            ($get:ident, $mk:path, |$a:ident, $b:ident| $e:expr) => {{
                let $b = pop!().$get()?;
                let $a = pop!().$get()?;
                st.push($mk($e));
            }};
        }
        macro_rules! cmp_branch {
            (|$a:ident, $b:ident| $e:expr) => {{
                let $b = pop!().i()?;
                let $a = pop!().i()?;
                return Ok(if $e { Next::Jump(target(insn)) } else { Next::Fall });
            }};
        }
        let op = insn.opcode;
        match op {
            0x00 => {}
            0x01 => st.push(CV::N),
            0x02..=0x08 => st.push(CV::I(op as i32 - 3)),
            0x09 | 0x0a => st.push(CV::J((op - 0x09) as i64)),
            0x0b..=0x0d => st.push(CV::F((op - 0x0b) as f32)),
            0x0e | 0x0f => st.push(CV::D((op - 0x0e) as f64)),
            0x10 | 0x11 => match insn.operand {
                Operand::Int(v) => st.push(CV::I(v)),
                _ => return fail("bipush 操作数"),
            },
            0x12..=0x14 => {
                let Operand::Ldc(c) = &insn.operand else { return fail("ldc 操作数") };
                let v = self.ldc(env, c)?;
                st.push(v);
            }
            0x15..=0x19 => st.push(locals[local(insn)?]),
            0x1a..=0x2d => st.push(locals[((op - 0x1a) % 4) as usize]),
            0x2e..=0x35 => {
                let i = pop!().i()?;
                let a = pop!().obj()?;
                let arr = self.arr(a)?;
                let v = *arr.get(i as usize).filter(|_| i >= 0).map_or_else(|| implicit("index"), Ok)?;
                st.push(v);
            }
            0x36..=0x3a => {
                let v = pop!();
                store(locals, local(insn)?, v)?;
            }
            0x3b..=0x4e => {
                let v = pop!();
                store(locals, ((op - 0x3b) % 4) as usize, v)?;
            }
            0x4f..=0x56 => {
                let v = pop!();
                let i = pop!().i()?;
                let a = pop!().obj()?;
                let ty = self.ty(a);
                let v = match (op, elem(&ty)) {
                    (0x54, "Z") => CV::I(v.i()? & 1),
                    (0x54, _) => CV::I(v.i()? as i8 as i32),
                    (0x55, _) => CV::I(v.i()? as u16 as i32),
                    (0x56, _) => CV::I(v.i()? as i16 as i32),
                    (0x53, e) => {
                        if let Some(o) = v.r()? {
                            let ct = e.strip_prefix('L').and_then(|x| x.strip_suffix(';')).unwrap_or(e);
                            let vt = self.ty(o);
                            if !self.instance_of(env, &vt, o, ct) {
                                return implicit("store");
                            }
                        }
                        v
                    }
                    _ => v,
                };
                let arr = self.arr_mut(a)?;
                let slot = arr.get_mut(i as usize).filter(|_| i >= 0).map_or_else(|| implicit("index"), Ok)?;
                *slot = v;
            }
            0x57 => {
                pop!();
            }
            0x58 => {
                if !pop!().wide() {
                    pop!();
                }
            }
            0x59 => {
                let v = *st.last().map_or_else(|| fail("操作数栈下溢"), Ok)?;
                st.push(v);
            }
            0x5a..=0x5f => self.dup(op, st)?,
            0x60..=0x83 => self.arith(op, st)?,
            0x84 => {
                let Operand::Iinc { index, delta } = insn.operand else { return fail("iinc 操作数") };
                let v = locals[index as usize].i()?;
                locals[index as usize] = CV::I(v.wrapping_add(delta as i32));
            }
            0x85..=0x93 => {
                let v = pop!();
                st.push(match op {
                    0x85 => CV::J(v.i()? as i64),
                    0x86 => CV::F(v.i()? as f32),
                    0x87 => CV::D(v.i()? as f64),
                    0x88 => CV::I(v.j()? as i32),
                    0x89 => CV::F(v.j()? as f32),
                    0x8a => CV::D(v.j()? as f64),
                    0x8b => CV::I(v.f()? as i32),
                    0x8c => CV::J(v.f()? as i64),
                    0x8d => CV::D(v.f()? as f64),
                    0x8e => CV::I(v.d()? as i32),
                    0x8f => CV::J(v.d()? as i64),
                    0x90 => CV::F(v.d()? as f32),
                    0x91 => CV::I(v.i()? as i8 as i32),
                    0x92 => CV::I(v.i()? as u16 as i32),
                    _ => CV::I(v.i()? as i16 as i32),
                });
            }
            0x94 => bin!(j, CV::I, |a, b| (a > b) as i32 - (a < b) as i32),
            0x95..=0x98 => {
                let b = pop!();
                let a = pop!();
                let (a, b) = if op <= 0x96 { (a.f()? as f64, b.f()? as f64) } else { (a.d()?, b.d()?) };
                let nan = if op % 2 == 1 { -1 } else { 1 };
                st.push(CV::I(if a.is_nan() || b.is_nan() { nan } else { (a > b) as i32 - (a < b) as i32 }));
            }
            0x99..=0x9e => {
                let v = pop!().i()?;
                let t = [v == 0, v != 0, v < 0, v >= 0, v > 0, v <= 0][(op - 0x99) as usize];
                return Ok(if t { Next::Jump(target(insn)) } else { Next::Fall });
            }
            0x9f => cmp_branch!(|a, b| a == b),
            0xa0 => cmp_branch!(|a, b| a != b),
            0xa1 => cmp_branch!(|a, b| a < b),
            0xa2 => cmp_branch!(|a, b| a >= b),
            0xa3 => cmp_branch!(|a, b| a > b),
            0xa4 => cmp_branch!(|a, b| a <= b),
            0xa5 | 0xa6 => {
                let b = pop!().r()?;
                let a = pop!().r()?;
                return Ok(if (a == b) == (op == 0xa5) { Next::Jump(target(insn)) } else { Next::Fall });
            }
            0xa7 | 0xc8 => return Ok(Next::Jump(target(insn))),
            0xaa => {
                let Operand::TableSwitch { default, low, targets, .. } = &insn.operand else { return fail("tableswitch") };
                let v = pop!().i()?;
                let t = (v as i64 - *low as i64).try_into().ok().and_then(|i: usize| targets.get(i));
                return Ok(Next::Jump(*t.unwrap_or(default)));
            }
            0xab => {
                let Operand::LookupSwitch { default, pairs } = &insn.operand else { return fail("lookupswitch") };
                let v = pop!().i()?;
                return Ok(Next::Jump(pairs.iter().find(|(k, _)| *k == v).map_or(*default, |(_, t)| *t)));
            }
            0xac..=0xb0 => return Ok(Next::Ret(Some(pop!()))),
            0xb1 => return Ok(Next::Ret(None)),
            0xb2..=0xb5 => self.field_op(env, op, insn, st)?,
            0xb6..=0xb9 => self.invoke_op(env, info, op, insn, st)?,
            0xba => {
                let Operand::InvokeDynamic { bsm, name, desc, .. } = &insn.operand else { return fail("indy 操作数") };
                let n = nparams(desc);
                let args = st.split_off(st.len().checked_sub(n).map_or_else(|| fail("操作数栈下溢"), Ok)?);
                let v = self.indy(env, &info.site.class, *bsm, name, desc, args)?;
                st.push(v);
            }
            0xbb => {
                let Operand::Class(c) = &insn.operand else { return fail("new 操作数") };
                self.ensure_init(env, c)?;
                let cf = self.class(env, c)?;
                if cf.is_interface() || cf.is_abstract() {
                    return fail("实例化抽象类");
                }
                let o = self.alloc(c, Body::Inst(Vec::new()));
                st.push(CV::R(o));
            }
            0xbc => {
                let Operand::NewArray(t) = insn.operand else { return fail("newarray 操作数") };
                let d = NEWARRAY_DESC.get((t as usize).wrapping_sub(4)).map_or_else(|| fail("newarray 类型码"), Ok)?;
                let n = pop!().i()?;
                let a = self.new_array(&format!("[{d}"), n)?;
                st.push(CV::R(a));
            }
            0xbd => {
                let Operand::Class(c) = &insn.operand else { return fail("anewarray 操作数") };
                let n = pop!().i()?;
                let a = self.new_array(&array_of(c), n)?;
                st.push(CV::R(a));
            }
            0xbe => {
                // 数组长度不可变：映像数组同样可取
                let a = pop!().obj()?;
                let Body::Arr(v) = &self.heap[a as usize].body else { return fail("期望数组") };
                st.push(CV::I(v.len() as i32));
            }
            0xbf => return Err(Flow::Throw(pop!().obj()?)),
            0xc0 | 0xc1 => {
                let Operand::Class(c) = &insn.operand else { return fail("checkcast 操作数") };
                let v = pop!();
                let is = match v.r()? {
                    None => None,
                    Some(o) => Some(self.instance_of(env, &self.ty(o), o, c)),
                };
                if op == 0xc0 {
                    if is == Some(false) {
                        return implicit("cast");
                    }
                    st.push(v);
                } else {
                    st.push(CV::I(i32::from(is == Some(true))));
                }
            }
            0xc2 | 0xc3 => {
                pop!().obj()?;
            }
            0xc5 => {
                let Operand::MultiANewArray(c, dims) = &insn.operand else { return fail("multianewarray 操作数") };
                let n = *dims as usize;
                let lens = st.split_off(st.len().checked_sub(n).map_or_else(|| fail("操作数栈下溢"), Ok)?);
                let lens: Vec<i32> = lens.into_iter().map(CV::i).collect::<R<_>>()?;
                let a = self.multi(c, &lens)?;
                st.push(CV::R(a));
            }
            0xc6 | 0xc7 => {
                let v = pop!().r()?;
                return Ok(if v.is_none() == (op == 0xc6) { Next::Jump(target(insn)) } else { Next::Fall });
            }
            _ => return fail(format!("不支持的指令 {}", insn.name())),
        }
        Ok(Next::Fall)
    }

    fn multi(&mut self, ty: &str, lens: &[i32]) -> R<u32> {
        let a = self.new_array(ty, lens[0])?;
        if lens.len() > 1 {
            for i in 0..lens[0] as usize {
                let sub = self.multi(elem(ty), &lens[1..])?;
                self.arr_mut(a)?[i] = CV::R(sub);
            }
        }
        Ok(a)
    }

    fn dup(&mut self, op: u8, st: &mut Vec<CV>) -> R<()> {
        // 按「项」操作：long / double 一项即 category 2
        let n = st.len();
        let at = |k: usize| if k <= n { Ok(st[n - k]) } else { fail("操作数栈下溢") };
        let (v1, w1) = (at(1)?, at(1)?.wide());
        match op {
            0x5a => st.insert(n - 2, v1),
            0x5b => {
                let w2 = at(2)?.wide();
                st.insert(if w2 { n - 2 } else { n.checked_sub(3).map_or_else(|| fail("操作数栈下溢"), Ok)? }, v1);
            }
            0x5c => {
                if w1 {
                    st.push(v1);
                } else {
                    let v2 = at(2)?;
                    st.extend([v2, v1]);
                }
            }
            0x5d => {
                if w1 {
                    st.insert(n - 2, v1);
                } else {
                    let v2 = at(2)?;
                    let p = n.checked_sub(3).map_or_else(|| fail("操作数栈下溢"), Ok)?;
                    st.splice(p..p, [v2, v1]);
                }
            }
            0x5e => {
                let top: Vec<CV> = if w1 { vec![v1] } else { vec![at(2)?, v1] };
                let rest = n - top.len();
                let below = if rest >= 1 && st[rest - 1].wide() { 1 } else { 2 };
                let p = rest.checked_sub(below).map_or_else(|| fail("操作数栈下溢"), Ok)?;
                st.splice(p..p, top);
            }
            _ => {
                let v2 = at(2)?;
                st[n - 1] = v2;
                st[n - 2] = v1;
            }
        }
        Ok(())
    }

    fn arith(&mut self, op: u8, st: &mut Vec<CV>) -> R<()> {
        let pop = |st: &mut Vec<CV>| st.pop().map_or_else(|| fail("操作数栈下溢"), Ok);
        // 取负
        if (0x74..=0x77).contains(&op) {
            let v = pop(st)?;
            st.push(match v {
                CV::I(x) => CV::I(x.wrapping_neg()),
                CV::J(x) => CV::J(x.wrapping_neg()),
                CV::F(x) => CV::F(-x),
                CV::D(x) => CV::D(-x),
                _ => return fail("取负操作数"),
            });
            return Ok(());
        }
        let b = pop(st)?;
        let a = pop(st)?;
        let r = match (op, a, b) {
            (0x60, CV::I(x), CV::I(y)) => CV::I(x.wrapping_add(y)),
            (0x61, CV::J(x), CV::J(y)) => CV::J(x.wrapping_add(y)),
            (0x62, CV::F(x), CV::F(y)) => CV::F(x + y),
            (0x63, CV::D(x), CV::D(y)) => CV::D(x + y),
            (0x64, CV::I(x), CV::I(y)) => CV::I(x.wrapping_sub(y)),
            (0x65, CV::J(x), CV::J(y)) => CV::J(x.wrapping_sub(y)),
            (0x66, CV::F(x), CV::F(y)) => CV::F(x - y),
            (0x67, CV::D(x), CV::D(y)) => CV::D(x - y),
            (0x68, CV::I(x), CV::I(y)) => CV::I(x.wrapping_mul(y)),
            (0x69, CV::J(x), CV::J(y)) => CV::J(x.wrapping_mul(y)),
            (0x6a, CV::F(x), CV::F(y)) => CV::F(x * y),
            (0x6b, CV::D(x), CV::D(y)) => CV::D(x * y),
            (0x6c | 0x70, CV::I(_), CV::I(0)) | (0x6d | 0x71, CV::J(_), CV::J(0)) => return implicit("arith"),
            (0x6c, CV::I(x), CV::I(y)) => CV::I(x.wrapping_div(y)),
            (0x6d, CV::J(x), CV::J(y)) => CV::J(x.wrapping_div(y)),
            (0x6e, CV::F(x), CV::F(y)) => CV::F(x / y),
            (0x6f, CV::D(x), CV::D(y)) => CV::D(x / y),
            (0x70, CV::I(x), CV::I(y)) => CV::I(x.wrapping_rem(y)),
            (0x71, CV::J(x), CV::J(y)) => CV::J(x.wrapping_rem(y)),
            (0x72, CV::F(x), CV::F(y)) => CV::F(x % y),
            (0x73, CV::D(x), CV::D(y)) => CV::D(x % y),
            (0x78, CV::I(x), CV::I(y)) => CV::I(x.wrapping_shl(y as u32 & 31)),
            (0x79, CV::J(x), CV::I(y)) => CV::J(x.wrapping_shl(y as u32 & 63)),
            (0x7a, CV::I(x), CV::I(y)) => CV::I(x >> (y & 31)),
            (0x7b, CV::J(x), CV::I(y)) => CV::J(x >> (y & 63)),
            (0x7c, CV::I(x), CV::I(y)) => CV::I(((x as u32) >> (y & 31)) as i32),
            (0x7d, CV::J(x), CV::I(y)) => CV::J(((x as u64) >> (y & 63)) as i64),
            (0x7e, CV::I(x), CV::I(y)) => CV::I(x & y),
            (0x7f, CV::J(x), CV::J(y)) => CV::J(x & y),
            (0x80, CV::I(x), CV::I(y)) => CV::I(x | y),
            (0x81, CV::J(x), CV::J(y)) => CV::J(x | y),
            (0x82, CV::I(x), CV::I(y)) => CV::I(x ^ y),
            (0x83, CV::J(x), CV::J(y)) => CV::J(x ^ y),
            _ => return fail(format!("算术操作数类型不符 {op:#x}")),
        };
        st.push(r);
        Ok(())
    }

    pub(super) fn ldc(&mut self, env: &Env, c: &Const) -> R<CV> {
        Ok(match c {
            Const::Int(v) => CV::I(*v),
            Const::Float(b) => CV::F(f32::from_bits(*b)),
            Const::Long(v) => CV::J(*v),
            Const::Double(b) => CV::D(f64::from_bits(*b)),
            Const::String(s) => CV::R(self.string(env, &s.encode_utf16().collect::<Vec<_>>())?),
            Const::StringUtf16(u) => CV::R(self.string(env, u)?),
            Const::Class(n) => CV::R(self.mirror(env, n)?),
            _ => return fail("ldc 方法类型 / 句柄 / 动态常量"),
        })
    }
}

fn local(insn: &Insn) -> R<usize> {
    match insn.operand {
        Operand::Local(i) => Ok(i as usize),
        _ => fail("局部变量操作数"),
    }
}

fn store(locals: &mut [CV], i: usize, v: CV) -> R<()> {
    if i >= locals.len() || (v.wide() && i + 1 >= locals.len()) {
        return fail("局部变量越界");
    }
    locals[i] = v;
    if v.wide() {
        locals[i + 1] = CV::N;
    }
    Ok(())
}

fn target(insn: &Insn) -> u32 {
    match insn.operand {
        Operand::Branch(t) => t,
        _ => insn.offset,
    }
}
