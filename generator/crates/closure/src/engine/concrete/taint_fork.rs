//! 污点上的分支（第 2 步）：区间可判定即按判定走；不可判定时两侧路径各自执行到分支的直接后支配点，
//! 合并局部变量与操作数栈（不同的整数值合并为选择表达式）。
//!
//! 两侧路径只许无副作用的指令（局部变量、栈、算术、读字段 / 已初始化类的静态字段、数组读、分支），
//! 不许调用、写入、分配、抛出。不满足即延迟值参与求值，按第 1 步规则残差化（`<clinit>` 转运行期初始化、
//! 根帧残差区段）。每侧路径对分支上的污点操作数按比较结果细化区间（如 `NCPU <= 1` 一侧 `NCPU` 为 1）。

use classfile::{Const, Insn, Operand};

use super::boot_cfg::PDom;
use super::interp::Next;
use super::vm::*;
use super::*;

/// 两侧路径的指令数上限（每侧）
const PURE_STEPS: u32 = 4096;
/// 合并嵌套深度上限
const FORK_DEPTH: u32 = 8;

impl Vm {
    /// 单步（引导求值时污点分支走合并）
    pub(super) fn step_t(&mut self, env: &Env, info: &Rc<MInfo>, ix: usize, insn: &Insn, locals: &mut [CV], st: &mut Vec<CV>) -> R<Next> {
        if self.boot && (0x99..=0xa4).contains(&insn.opcode) {
            let n = st.len();
            let (a, b, pops) = if insn.opcode <= 0x9e {
                (st.last().copied().unwrap_or(CV::N), CV::I(0), 1)
            } else if n >= 2 {
                (st[n - 2], st[n - 1], 2)
            } else {
                (CV::N, CV::N, 0)
            };
            if pops > 0 && (matches!(a, CV::T(..)) || matches!(b, CV::T(..))) {
                st.truncate(n - pops);
                let to = self.tbranch(env, info, ix, insn, a, b, locals, st)?;
                return Ok(Next::Jump(info.code().insns[to].offset));
            }
        }
        self.step(env, info, insn, locals, st)
    }

    fn pdom_of(&mut self, info: &MInfo) -> R<Rc<PDom>> {
        if let Some(p) = self.bj.pdoms.get(&info.key) {
            return Ok(p.clone());
        }
        let p = Rc::new(PDom::new(info.code(), &info.index)?);
        self.bj.pdoms.insert(info.key.clone(), p.clone());
        Ok(p)
    }

    /// 污点分支（操作数已出栈）：返回后继指令下标（合并时为直接后支配点，局部变量与栈已合并）
    #[allow(clippy::too_many_arguments)]
    fn tbranch(&mut self, env: &Env, info: &Rc<MInfo>, ix: usize, insn: &Insn, a: CV, b: CV, locals: &mut [CV], st: &mut Vec<CV>) -> R<usize> {
        let k = (insn.opcode - 0x99) % 6;
        let Operand::Branch(t) = insn.operand else { return fail("分支操作数") };
        let tgt = *info.index.get(&t).map_or_else(|| fail("分支目标非指令边界"), Ok)?;
        if let Some(h) = self.tdecide(k, a, b) {
            self.bj.taint.decided += 1;
            return Ok(if h { tgt } else { ix + 1 });
        }
        if self.bj.taint.depth >= FORK_DEPTH {
            return defer("延迟值参与求值：宿主标量（污点）上的分支嵌套过深");
        }
        let pd = self.pdom_of(info)?;
        let Some(c) = pd.ipdom(ix) else { return defer("延迟值参与求值：宿主标量（污点）上的分支无后支配点") };
        self.bj.taint.forks += 1;
        let m = self.jmark();
        self.bj.taint.depth += 1;
        let mut outs: [Option<(Vec<CV>, Vec<CV>)>; 2] = [None, None];
        let mut err: Option<Flow> = None;
        for (i, holds) in [true, false].into_iter().enumerate() {
            // 该侧不可能（细化后区间为空）：只取另一侧
            let Some(rf) = self.tnarrow(k, a, b, holds) else { continue };
            let mut l = locals.to_vec();
            let mut s = st.clone();
            if let Some((id, lo, hi)) = rf {
                if lo == hi {
                    let w = matches!(a, CV::T(x, b'J') if x == id) || matches!(b, CV::T(x, b'J') if x == id);
                    let cv = if w { CV::J(lo) } else { CV::I(lo as i32) };
                    for v in l.iter_mut().chain(s.iter_mut()) {
                        if matches!(*v, CV::T(x, _) if x == id) {
                            *v = cv;
                        }
                    }
                }
                self.bj.taint.refine.push((id, lo, hi));
            }
            let r = self.run_pure(env, info, if holds { tgt } else { ix + 1 }, c, &mut l, &mut s);
            if rf.is_some() {
                self.bj.taint.refine.pop();
            }
            match r {
                Ok(()) => outs[i] = Some((l, s)),
                Err(e) => {
                    err = Some(e);
                    break;
                }
            }
        }
        self.bj.taint.depth -= 1;
        if !self.pure_since(m) {
            self.jrollback(m)?;
            return defer("延迟值参与求值：宿主标量（污点）分支的路径有副作用");
        }
        self.jpop(m);
        if let Some(e) = err {
            return Err(e);
        }
        let [t_out, f_out] = outs;
        let (l, s) = match (t_out, f_out) {
            (Some(x), None) | (None, Some(x)) => x,
            (Some((lt, st_t)), Some((lf, st_f))) => {
                if st_t.len() != st_f.len() {
                    return defer("延迟值参与求值：宿主标量（污点）分支两侧到合并点的栈深不同");
                }
                let mut l = Vec::with_capacity(lt.len());
                for (x, y) in lt.into_iter().zip(lf) {
                    l.push(self.tselect(k, a, b, x, y)?);
                }
                let mut s = Vec::with_capacity(st_t.len());
                for (x, y) in st_t.into_iter().zip(st_f) {
                    s.push(self.tselect(k, a, b, x, y)?);
                }
                (l, s)
            }
            (None, None) => return fail("污点分支两侧均不可能"),
        };
        locals.copy_from_slice(&l);
        *st = s;
        Ok(c)
    }

    /// 无副作用地执行 `[ix, end)`（到达 end 即止）
    fn run_pure(&mut self, env: &Env, info: &Rc<MInfo>, mut ix: usize, end: usize, locals: &mut [CV], st: &mut Vec<CV>) -> R<()> {
        let code = info.code();
        let mut n = 0u32;
        while ix != end {
            n += 1;
            if n > PURE_STEPS {
                return defer("延迟值参与求值：宿主标量（污点）分支的路径过长");
            }
            self.steps += 1;
            let Some(insn) = code.insns.get(ix) else { return fail("越过方法末尾") };
            let ok = match insn.opcode {
                0x12..=0x14 => matches!(&insn.operand, Operand::Ldc(Const::Int(_) | Const::Float(_) | Const::Long(_) | Const::Double(_))),
                0x00..=0x11 | 0x15..=0x4e | 0x57..=0xa7 | 0xaa | 0xab | 0xb4 | 0xbe | 0xc0 | 0xc1 | 0xc6..=0xc8 => true,
                0xb2 => {
                    let Operand::Field(f) = &insn.operand else { return fail("字段操作数") };
                    let fr = self.field_res(env, f)?;
                    matches!(self.init.get(&fr.decl), Some(Init::Done | Init::Running))
                }
                _ => false,
            };
            if !ok {
                return defer(format!("延迟值参与求值：宿主标量（污点）分支的路径含 {}", insn.name()));
            }
            match self.step_t(env, info, ix, insn, locals, st) {
                Ok(Next::Fall) => ix += 1,
                Ok(Next::Jump(off)) => ix = *info.index.get(&off).map_or_else(|| fail("分支目标非指令边界"), Ok)?,
                Ok(Next::Ret(_)) => return fail("污点分支路径返回"),
                Err(Flow::Implicit(_) | Flow::Throw(_)) => return defer("延迟值参与求值：宿主标量（污点）分支的路径抛出异常"),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}
