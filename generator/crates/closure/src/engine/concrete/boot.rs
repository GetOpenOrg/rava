//! 构建期引导求值的根帧驱动（计划 2026-10-05-boot-image-evaluator §3.1–§3.2）。
//!
//! 引导阶段方法（`[concrete.boot] calls`，即 `System.initPhase1/2/3`）在根帧逐条执行，其余帧照常解释。
//! 宿主相关值（延迟值）参与求值时按出现位置残差化：
//! - `<clinit>` 内：撤回，该类转为运行期初始化（`init.rs`）；
//! - 根帧的调用（返回引用或 void）：撤回该调用，运行期重放（[`Rec::Call`]）；实参对象的状态此后构建期不可读，
//!   返回引用时以占位对象代表结果；
//! - 根帧其余位置（基本类型结果、在延迟值上分支）：撤回到所在语句的起点 s（根帧操作数栈为空的点），
//!   语句起点到依赖分支的直接后支配点 m 之间的区段运行期执行（[`Rec::Region`]）。构建期仍沿区段内全部
//!   分支各走一遍，以登记区段可能改写的位置（撤回后记为脏位置）；区段终点要求各路径栈空、局部变量与
//!   起点相同，否则终点沿后支配树外移，直至方法出口。
//!
//! 已知边界：残差调用 / 区段在延迟点之后的执行构建期观测不到，其写入不进脏位置（报告中逐条列出残差记录）。

use classfile::Operand;

use super::boot_cfg::{fork_shape, PDom};
use super::interp::{nparams, returns_void};
use super::journal::Rec;
use super::vm::*;
use super::*;

/// 区段探索的路径数上限
const PATH_LIMIT: usize = 256;

pub(super) enum RNext {
    Goto(usize),
    Ret(Option<CV>),
}

/// 引导阶段的结局
pub(super) enum PhaseEnd {
    /// 构建期执行完毕（返回值）
    Value(Option<CV>),
    /// 残差区段延伸到方法出口：阶段的剩余部分运行期执行
    Residual,
}

fn why_of(w: &str) -> String {
    w.split(" @ ").next().unwrap_or(w).to_string()
}

impl Vm {
    /// 参考 JDK 的特性版本号（`java/lang/Object` 类文件主版本 − 44）
    pub(super) fn jdk_feature(&self, env: &Env) -> R<i64> {
        Ok(i64::from(self.class(env, OBJECT)?.major) - 44)
    }

    /// 执行引导阶段方法（根帧）
    pub(super) fn boot_phase(&mut self, env: &Env, mr: &MemberRef, args: Vec<CV>) -> R<PhaseEnd> {
        let site = self.resolve(env, mr, false)?;
        self.ensure_init(env, &site.class.name.clone())?;
        let info = self.info(env, &site);
        if info.op.is_some() || !info.bytecode {
            return fail(format!("引导阶段方法无字节码 {}", info.key));
        }
        let code = info.code();
        let mut locals = vec![CV::N; code.max_locals as usize];
        let mut i = 0;
        for a in args {
            if i >= locals.len() {
                return fail("实参超出局部变量表");
            }
            locals[i] = a;
            i += if a.wide() { 2 } else { 1 };
        }
        self.frames.push(info.key.clone());
        let r = self.root_loop(env, &info, &mut locals);
        self.frames.pop();
        r
    }

    fn root_loop(&mut self, env: &Env, info: &Rc<MInfo>, locals: &mut [CV]) -> R<PhaseEnd> {
        let code = info.code();
        let mut pdom: Option<PDom> = None;
        let mut st: Vec<CV> = Vec::new();
        let mut ix = 0usize;
        // 语句检查点：(日志标记, 起点下标, 起点局部变量)
        let mut stmt: Option<(super::journal::Mark, usize, Vec<CV>)> = None;
        loop {
            if st.is_empty() {
                if let Some((m, _, _)) = stmt.take() {
                    self.jpop(m);
                }
                stmt = Some((self.jmark(), ix, locals.to_vec()));
            }
            let pre = st.clone();
            let invoke = matches!(code.insns.get(ix).map(|x| x.opcode), Some(0xb6..=0xb9));
            let im = invoke.then(|| self.jmark());
            match self.root_one(env, info, ix, locals, &mut st) {
                Ok(next) => {
                    if let Some(m) = im {
                        self.jpop(m);
                    }
                    match next {
                        RNext::Goto(n) => ix = n,
                        RNext::Ret(v) => {
                            if let Some((m, _, _)) = stmt.take() {
                                self.jpop(m);
                            }
                            return Ok(PhaseEnd::Value(v));
                        }
                    }
                }
                Err(Flow::Defer(why)) => {
                    self.fail_frames = None;
                    if let Some(m) = im {
                        if self.residual_call(env, info, ix, m, &pre, &mut st, &why, true)?.is_some() {
                            ix += 1;
                            continue;
                        }
                    }
                    let Some((m, s, ck)) = stmt.take() else { return fail("根帧无语句检查点") };
                    self.jrollback(m)?;
                    locals.copy_from_slice(&ck);
                    st.clear();
                    if pdom.is_none() {
                        pdom = Some(PDom::new(code, &info.index)?);
                    }
                    let pd = pdom.as_ref().map_or_else(|| fail("后支配树"), Ok)?;
                    let end = self.region(env, info, pd, s, ix, &ck)?;
                    self.push_rec(env, Rec::Region {
                        phase: info.key.clone(),
                        start: code.insns[s].offset,
                        end: end.map(|e| code.insns[e].offset),
                        locals: ck,
                        why: why_of(&why),
                    });
                    self.war_capture(m.rl(), m.heap());
                    match end {
                        Some(e) => ix = e,
                        None => return Ok(PhaseEnd::Residual),
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// 根帧单步：执行一条指令，异常按根方法的异常表转处理器
    fn root_one(&mut self, env: &Env, info: &Rc<MInfo>, ix: usize, locals: &mut [CV], st: &mut Vec<CV>) -> R<RNext> {
        let code = info.code();
        self.steps += 1;
        if self.steps > self.step_limit {
            return fail("步数超限");
        }
        let Some(insn) = code.insns.get(ix) else { return fail("越过方法末尾") };
        let r = match self.step_t(env, info, ix, insn, locals, st) {
            Err(Flow::Implicit(k)) => {
                self.throw_frames = Some(self.frames.iter().map(|f| f.to_string()).chain([format!("隐式异常 {k} @ {}", insn.offset)]).collect());
                self.implicit(env, k).and_then(|o| Err(Flow::Throw(o)))
            }
            r => r,
        };
        match r {
            Ok(super::interp::Next::Fall) => Ok(RNext::Goto(ix + 1)),
            Ok(super::interp::Next::Jump(off)) => Ok(RNext::Goto(*info.index.get(&off).map_or_else(|| fail("分支目标非指令边界"), Ok)?)),
            Ok(super::interp::Next::Ret(v)) => Ok(RNext::Ret(v)),
            Err(Flow::Throw(o)) => {
                let ty = self.ty(o);
                let h = code.exception_table.iter().find(|e| {
                    e.start <= insn.offset && insn.offset < e.end && e.catch_type.as_deref().is_none_or(|c| self.instance_of(env, &ty, o, c))
                });
                let Some(h) = h else { return Err(Flow::Throw(o)) };
                st.clear();
                st.push(CV::R(o));
                Ok(RNext::Goto(*info.index.get(&h.handler).map_or_else(|| fail("处理器非指令边界"), Ok)?))
            }
            Err(Flow::Fail(w)) if !w.contains(" @ ") => {
                if self.fail_frames.is_none() {
                    self.fail_frames = Some(self.frames.iter().map(|f| f.to_string()).collect());
                }
                fail(format!("{w} @ {}@{}", info.key, insn.offset))
            }
            Err(Flow::Defer(w)) if !w.contains(" @ ") => defer(format!("{w} @ {}@{}", info.key, insn.offset)),
            Err(f) => Err(f),
        }
    }

    /// 根帧调用（返回引用或 void）的残差化：撤回到调用前，实参对象标为延迟，返回引用时压入占位对象。
    /// 不适用（返回基本类型）时返回 None（不撤回）
    #[allow(clippy::too_many_arguments)]
    fn residual_call(&mut self, env: &Env, info: &Rc<MInfo>, ix: usize, m: super::journal::Mark, pre: &[CV], st: &mut Vec<CV>, why: &str, record: bool) -> R<Option<Option<u32>>> {
        let insn = &info.code().insns[ix];
        let Operand::Method(callee, _) = &insn.operand else { return Ok(None) };
        let ret = callee.desc.rsplit(')').next().unwrap_or("V");
        if !returns_void(&callee.desc) && !ret.starts_with('L') && !ret.starts_with('[') {
            return Ok(None);
        }
        self.jrollback(m)?;
        let n = nparams(&callee.desc) + usize::from(insn.opcode != 0xb8);
        let Some(keep) = pre.len().checked_sub(n) else { return fail("操作数栈下溢") };
        st.clear();
        st.extend_from_slice(&pre[..keep]);
        let args = pre[keep..].to_vec();
        let tag = format!("{callee} 的实参");
        for &a in &args {
            if let CV::R(o) = a {
                if !self.mirror_of.contains_key(&o) && !matches!(self.heap[o as usize].body, Body::Lam(_)) && !self.stable(env, o) {
                    self.mark_deferred(o, &tag);
                }
            }
        }
        let ph = if returns_void(&callee.desc) {
            None
        } else {
            let ty = ret.strip_prefix('L').and_then(|t| t.strip_suffix(';')).unwrap_or(ret);
            let body = if ty.starts_with('[') { Body::Arr(Vec::new()) } else { Body::Inst(Vec::new()) };
            let o = self.alloc(ty, body);
            self.mark_placeholder(o, &format!("{callee} 的结果"));
            st.push(CV::R(o));
            Some(o)
        };
        if record {
            self.push_rec(env, Rec::Call { phase: info.key.clone(), off: insn.offset, callee: callee.clone(), args, ph, why: why_of(why) });
            self.war_capture(m.rl(), m.heap());
        }
        Ok(Some(ph))
    }

    /// 残差区段 `[s, ·)`：自延迟点 k 的依赖分支的直接后支配点起，探索全部路径并撤回（写入记为脏位置），
    /// 返回区段终点（None = 方法出口）
    fn region(&mut self, env: &Env, info: &Rc<MInfo>, pd: &PDom, s: usize, k: usize, ck: &[CV]) -> R<Option<usize>> {
        let code = info.code();
        let b = fork_shape(code, k, usize::MAX / 2).map_or(k, |(b, _)| b);
        let mut c = pd.ipdom(b);
        loop {
            let m = self.jmark();
            let r = self.explore(env, info, s, ck, c);
            self.jrollback(m)?;
            match (r?, c) {
                (true, _) | (_, None) => return Ok(c),
                (false, Some(e)) => c = pd.ipdom(e),
            }
        }
    }

    /// 从 s 起探索各路径到 c：延迟值上的分支两侧都走，其余延迟点截断该路径。各路径到达 c 时栈空且
    /// 局部变量与起点相同返回 true
    fn explore(&mut self, env: &Env, info: &Rc<MInfo>, s: usize, ck: &[CV], c: Option<usize>) -> R<bool> {
        let code = info.code();
        let mut work: Vec<(usize, Vec<CV>, Vec<CV>)> = vec![(s, ck.to_vec(), Vec::new())];
        let (mut ok, mut paths) = (true, 0usize);
        while let Some((mut ix, mut locals, mut st)) = work.pop() {
            paths += 1;
            if paths > PATH_LIMIT {
                return fail(format!("残差区段路径数超限 {}@{}", info.key, code.insns[s].offset));
            }
            loop {
                if Some(ix) == c {
                    if !(st.is_empty() && locals == ck) {
                        ok = false;
                        let d: Vec<usize> = (0..ck.len()).filter(|&i| locals.get(i) != ck.get(i)).collect();
                        self.bj.region_notes.push(format!("{}@{}→{:?}：到达终点时栈深 {}，局部变量变化 {d:?}", info.key, code.insns[s].offset, c.map(|c| code.insns[c].offset), st.len()));
                    }
                    break;
                }
                let pre = st.clone();
                let invoke = matches!(code.insns.get(ix).map(|x| x.opcode), Some(0xb6..=0xb9));
                let im = invoke.then(|| self.jmark());
                match self.root_one(env, info, ix, &mut locals, &mut st) {
                    Ok(RNext::Goto(n)) => {
                        if let Some(m) = im {
                            self.jpop(m);
                        }
                        ix = n;
                    }
                    Ok(RNext::Ret(_)) => {
                        if let Some(m) = im {
                            self.jpop(m);
                        }
                        break;
                    }
                    Err(Flow::Defer(why)) => {
                        self.fail_frames = None;
                        if let Some(m) = im {
                            if self.residual_call(env, info, ix, m, &pre, &mut st, &why, false)?.is_some() {
                                ix += 1;
                                continue;
                            }
                        }
                        // 区段内取到延迟的引用值：以占位对象代之继续（区段整体运行期执行，构建期只求写入集）
                        if let Some((pops, ty)) = super::boot_cfg::ref_load(&code.insns[ix]) {
                            if let Some(keep) = pre.len().checked_sub(pops) {
                                st.clear();
                                st.extend_from_slice(&pre[..keep]);
                                let body = if ty.starts_with('[') { Body::Arr(Vec::new()) } else { Body::Inst(Vec::new()) };
                                let o = self.alloc(&ty, body);
                                self.mark_placeholder(o, &why_of(&why));
                                st.push(CV::R(o));
                                ix += 1;
                                continue;
                            }
                        }
                        match fork_shape(code, ix, pre.len()) {
                            Some((b, keep)) => {
                                let succ = super::boot_cfg::succs(code, &info.index, b)?;
                                for t in succ.into_iter().filter(|&t| t < code.insns.len()) {
                                    work.push((t, locals.clone(), pre[..keep].to_vec()));
                                }
                            }
                            // 延迟值不在分支上：此后的栈与局部变量构建期不可知
                            None => {
                                ok = false;
                                self.bj.region_notes.push(format!("{}@{}→{:?}：{} 处延迟值不在分支上：{}", info.key, code.insns[s].offset, c.map(|c| code.insns[c].offset), code.insns[ix].offset, why_of(&why)));
                            }
                        }
                        break;
                    }
                    // 未捕获的异常离开根帧：路径终止
                    Err(Flow::Throw(_)) => break,
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(ok)
    }
}
