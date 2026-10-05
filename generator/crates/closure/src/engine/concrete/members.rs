//! 字段与方法调用指令（JVMS §6.5 getstatic / putstatic / getfield / putfield / invoke*）。

use classfile::{Insn, Operand};

use super::interp::{nparams, returns_void};
use super::vm::*;
use super::*;

impl Vm {
    pub(super) fn field_op(&mut self, env: &Env, op: u8, insn: &Insn, st: &mut Vec<CV>) -> R<()> {
        let Operand::Field(f) = &insn.operand else { return fail("字段操作数") };
        let fr = self.field_res(env, f)?;
        let pop = |st: &mut Vec<CV>| st.pop().map_or_else(|| fail("操作数栈下溢"), Ok);
        match op {
            0xb2 => {
                self.ensure_init(env, &fr.decl)?;
                // VM 注入的字面量值：字节码写入被 VM 值覆盖，读取恒为该值
                if let Some(x) = env.man().injected_literal(&fr.decl, &fr.name) {
                    st.push(if &*fr.desc == "J" { CV::J(x) } else { CV::I(x as i32) });
                    return Ok(());
                }
                // 非 final 静态字段只在其类初始化期间可读：初始化之后它可能被程序其它部分改写
                let running = matches!(self.init.get(&fr.decl), Some(Init::Running));
                if !self.boot && !fr.fin && !fr.memo && !running {
                    return fail(format!("读取可变静态字段 {}.{}", fr.decl, fr.name));
                }
                if self.boot {
                    if let Some(v) = env.cfg().boot.statics.get(&format!("{}.{}", fr.decl, fr.name)) {
                        if v == "@deferred" {
                            return defer(format!("延迟值参与求值：VM 注入的宿主相关静态 {}.{}", fr.decl, fr.name));
                        }
                        let x = v.parse::<i64>().map_or_else(|_| fail(format!("引导静态值非整数 {v}")), Ok)?;
                        st.push(match &*fr.desc { "J" => CV::J(x), "Z" | "B" | "C" | "S" | "I" => CV::I(x as i32), d => return fail(format!("引导静态类型 {d}")) });
                        return Ok(());
                    }
                    self.boot_static_check(&fr)?;
                }
                if self.opaque.contains(&fr.decl) || env.man().is_injected_static(&fr.decl, &fr.name) {
                    return fail(format!("读取 VM 承载的静态字段 {}.{}", fr.decl, fr.name));
                }
                let v = match self.statics.get(&fr.key) {
                    Some(v) => *v,
                    None => match &fr.constant {
                        Some(c) => self.ldc(env, c)?,
                        None => CV::zero(&fr.desc),
                    },
                };
                self.freeze_static(env, &fr.decl, v);
                st.push(v);
            }
            0xb3 => {
                self.ensure_init(env, &fr.decl)?;
                let v = pop(st)?;
                if !self.boot && self.image > 0 && !matches!(self.init.get(&fr.decl), Some(Init::Running)) && !fr.memo {
                    return fail(format!("类初始化写入它类静态字段 {}.{}", fr.decl, fr.name));
                }
                self.put_static(&fr, v)?;
                if self.tracing() {
                    self.trace.puts.entry(fr.mref()).or_default().push(put_of(v));
                    self.trace.memo_vals.push((fr.mref(), v));
                }
            }
            0xb4 => {
                let o = pop(st)?.obj()?;
                let v = self.get_field(env, o, &fr)?;
                st.push(v);
            }
            _ => {
                let v = pop(st)?;
                let o = pop(st)?.obj()?;
                self.traced_put_field(o, &fr, v)?;
            }
        }
        Ok(())
    }

    /// 实例字段写入并记入轨迹（字段写入值；映像对象上的写入另记内存缓存值）
    pub(super) fn traced_put_field(&mut self, o: u32, fr: &FRes, v: CV) -> R<()> {
        self.put_field(o, fr, v)?;
        if self.tracing() {
            self.trace.puts.entry(fr.mref()).or_default().push(put_of(v));
            if self.heap[o as usize].epoch == 0 {
                self.trace.memo_vals.push((fr.mref(), v));
            }
        }
        Ok(())
    }

    pub(super) fn invoke_op(&mut self, env: &Env, info: &Rc<MInfo>, op: u8, insn: &Insn, st: &mut Vec<CV>) -> R<()> {
        let Operand::Method(m, iface) = &insn.operand else { return fail("调用操作数") };
        let n = nparams(&m.desc) + usize::from(op != 0xb8);
        let args = st.split_off(st.len().checked_sub(n).map_or_else(|| fail("操作数栈下溢"), Ok)?);
        let resolved = self.resolve(env, m, *iface)?;
        let target = match op {
            0xb8 => {
                self.ensure_init(env, &resolved.class.name)?;
                resolved
            }
            _ => {
                if self.boot {
                    self.check_identity(args[0])?;
                }
                let recv = args[0].obj()?;
                if let Body::Lam(l) = &self.heap[recv as usize].body {
                    let l = l.clone();
                    // 轨迹记调用点 → 实现方法（SAM）；非 SAM 的接口缺省方法由 call_lambda 选定后记入
                    if self.tracing() && resolved.method().is_abstract() {
                        self.trace.calls.entry((info.key.clone(), insn.offset)).or_default().insert(l.imp.member.clone());
                    }
                    let v = self.call_lambda(env, &l, &resolved, args)?;
                    if let Some(v) = v {
                        st.push(v);
                    }
                    return Ok(());
                }
                if op == 0xb7 {
                    self.special(env, &info.site, resolved)?
                } else {
                    let ty = self.ty(recv);
                    self.select(env, &ty, &resolved)?
                }
            }
        };
        if self.tracing() {
            let (o, nm, d) = target.key();
            let t = MemberRef { owner: o, name: nm, desc: d };
            self.trace.calls.entry((info.key.clone(), insn.offset)).or_default().insert(t);
        }
        let ret = self.call(env, &target, args)?;
        match (ret, returns_void(&m.desc)) {
            (Some(v), false) => st.push(v),
            (None, true) => {}
            _ => return fail(format!("返回值与描述符不符 {m}")),
        }
        Ok(())
    }

    /// invokespecial 的方法选择（JVMS §6.5 invokespecial：超类方法调用从当前类的直接超类起查）
    fn special(&mut self, env: &Env, cur: &MethodSite, resolved: MethodSite) -> R<MethodSite> {
        let rm = resolved.method();
        if rm.is_init() || rm.is_private() || resolved.class.is_interface() || resolved.class.name == cur.class.name {
            return Ok(resolved);
        }
        match &cur.class.super_name {
            Some(s) => {
                let s: Rc<str> = Rc::from(s.as_str());
                self.select(env, &s, &resolved)
            }
            None => Ok(resolved),
        }
    }
}

/// 写入值的常量格投影
fn put_of(v: CV) -> Put {
    match v {
        CV::I(x) => Put::Int(x),
        CV::J(x) => Put::Long(x),
        CV::N => Put::Null,
        _ => Put::Other,
    }
}
