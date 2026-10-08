//! 引擎：确定性具体求值（`vm_intrinsics.toml [concrete]`）。
//!
//! 入口方法的调用点实参可枚举（字符串字面量 / 类镜像 / 整数 / null，且来源全部可取回）时，按实参逐组
//! 具体执行入口（解释器见 `concrete/interp.rs`）。全部组合成功：
//! - 轨迹上的方法以具体上下文入闭包，只按轨迹执行过的指令登记依赖，不做抽象分析（`concrete/apply.rs`）；
//! - 结果对象图物化为类型流（实例化类型、字段值集、数组元素、返回值集）；
//! - 不接抽象调用边。
//! 任一组合失败（不可建模的指令 / native / 共享状态改写）即整个调用点回退抽象调用边；回退后不再撤回。
//! 每组实参先冷后热各求值一次（内存缓存字段先写后读），轨迹取并；求值后撤销对映像的缓存写入。

mod apply;
mod boot;
mod boot_cfg;
pub mod boot_image;
mod boot_slots;
mod export;
mod indy;
mod init;
mod interp;
mod journal;
mod members;
mod natives;
pub(super) mod persist;
mod snap;
mod stable;
mod taint;
mod taint_fork;
mod unsafe_ops;
mod vm;
mod vm_heap;
mod vm_link;
mod war;

use resolve::MethodSite;

use self::snap::{MObj, MV};
use self::vm::*;
use super::*;

/// 具体上下文的类型名（方法克隆上下文取其类型序号）
const CONCRETE_CTX: &str = "@concrete";
/// 单个调用点的实参组合数上限
const COMBO_LIMIT: usize = 64;
/// 诊断（`--flows @concrete`）列出的实参组合数上限
const DIAG_COMBOS: usize = 64;

/// 枚举出的实参
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(super) enum AK {
    Str(Rc<str>),
    Mirror(Rc<str>),
    Int(i32),
    Long(i64),
    Null,
}

/// 一组实参的求值结果（冷 / 热两次取并）
#[derive(Default)]
pub(super) struct Outcome {
    pcs: BTreeMap<MemberRef, BTreeSet<u32>>,
    calls: BTreeMap<(MemberRef, u32), BTreeSet<MemberRef>>,
    puts: BTreeMap<MemberRef, Vec<Put>>,
    inited: BTreeSet<String>,
    objs: Vec<MObj>,
    /// 返回值（void 为 None）/ 抛出的异常对象
    rets: Vec<MV>,
    thrown: Vec<MV>,
    /// 写入映像的字段值（撤销后对程序仍可见）
    memo: Vec<(MemberRef, MV)>,
    /// 映像状态的写入全部可物化进引导映像时（`[concrete] image_memo_fields`，见 `concrete/persist.rs`）：
    /// 各缓存片段与只按热求值的结果（运行期缓存已在映像中，执行的即热路径）
    alt: Option<Box<(Vec<persist::Frag>, Outcome)>>,
    /// 不可物化的原因（诊断）
    alt_why: Option<String>,
}

#[derive(Default)]
pub(super) struct Concrete {
    /// 具体上下文（类型序号）
    pub(super) ctx: u32,
    vm: Option<Box<Vm>>,
    /// 轨迹并集：方法 → (执行过的指令偏移, 调用偏移 → 实际目标)
    traces: HashMap<MemberRef, (BTreeSet<u32>, BTreeMap<u32, BTreeSet<MemberRef>>)>,
    /// (入口, 实参) → 求值结果（Err = 失败原因）
    memo: HashMap<(MemberRef, Vec<AK>), Rc<Result<Outcome, String>>>,
    /// 已应用的 (调用方节点, 偏移, 实参)
    applied: HashSet<(usize, u32, Vec<AK>)>,
    /// 已回退抽象调用边的调用点
    fallback: HashSet<(usize, u32)>,
    /// 诊断：调用点 → 各方法上下文的结论（成功时列出实参组合，按上下文分别求值的调用点逐条记录）
    pub(super) diag: BTreeMap<String, BTreeSet<String>>,
}

impl<'a> Engine<'a> {
    pub(super) fn concrete_init(&mut self) {
        self.concrete.ctx = self.id(CONCRETE_CTX);
    }

    pub(super) fn is_concrete(&self, m: usize) -> bool {
        self.methods[m].ctx == self.concrete.ctx
    }

    /// 方法在具体上下文中的执行轨迹（指令偏移）
    pub(super) fn concrete_pcs(&self, key: &MemberRef) -> Option<&BTreeSet<u32>> {
        self.concrete.traces.get(key).map(|t| &t.0)
    }

    /// 调用 m@off → resolved（静态调用，或接收者值集为 recv 的非虚调用）按具体求值处理；返回 true 即不再接抽象调用边。
    /// 接收者尚无取值时暂不接边（接收者增长时调用方重处理）
    pub(super) fn concrete_call(&mut self, m: usize, off: u32, resolved: &MemberRef, md: &MethodDesc, recv: Option<&TypeSet>, args: &[V]) -> bool {
        if self.concrete.fallback.contains(&(m, off)) || self.is_concrete(m) {
            return false;
        }
        let k = self.mref_key(resolved);
        if !self.man.concrete.entries.contains(&*k) {
            return false;
        }
        let site_name = format!("{}@{off}", self.methods[m].key);
        if recv.is_some_and(|s| s.classes.is_empty() && s.open.is_empty()) {
            return true;
        }
        let combos = match self.combos(m, off, md, recv, args) {
            Ok(c) => c,
            Err(why) => return self.concrete_fallback(m, off, site_name, why),
        };
        let Some(site) = self.h.resolve_method(&resolved.owner, &resolved.name, &resolved.desc, false) else { return false };
        let mut outs = Vec::new();
        for c in &combos {
            let r = self.concrete_eval(&site, resolved, c);
            if let Err(w) = &*r {
                return self.concrete_fallback(m, off, site_name, format!("{c:?}：{w}"));
            }
            outs.push((c.clone(), r));
        }
        // 镜像缓存可物化进引导映像的组合按热求值入闭包（运行期缓存已命中）
        let mut why: Vec<Option<String>> = Vec::new();
        let mut hot: Vec<bool> = Vec::new();
        for (_, r) in &outs {
            let Ok(o) = &**r else {
                hot.push(false);
                why.push(None);
                continue;
            };
            let w = match (&o.alt, &o.alt_why) {
                (Some(a), _) => self.image_memo_ok(&a.0).err(),
                (None, w) => w.clone(),
            };
            hot.push(w.is_none() && o.alt.is_some());
            why.push(w);
        }
        let image: Vec<Rc<str>> = outs
            .iter()
            .zip(&hot)
            .filter_map(|((_, r), &h)| r.as_ref().as_ref().ok().map(|o| pick(o, h)))
            .flat_map(|o| apply::image_types(o).cloned().collect::<Vec<_>>())
            .collect();
        if let Some(t) = image.iter().find(|t| self.container(t)) {
            return self.concrete_fallback(m, off, site_name, format!("结果引用映像中的容器形态对象 {t}"));
        }
        let entry = self.method_ctx(resolved.clone(), self.concrete.ctx, Via::method("concrete", m, Some(off)));
        self.dispatch.entry((m, off)).or_default().insert(entry);
        self.callers.entry(entry).or_default().insert(m);
        for ((c, r), &h) in outs.into_iter().zip(&hot) {
            if !self.concrete.applied.insert((m, off, c)) {
                continue;
            }
            let Ok(o) = &*r else { continue };
            match o.alt.as_ref().filter(|_| h) {
                Some(a) => {
                    self.image_memo_apply(&a.0);
                    self.concrete_apply(m, off, resolved, md, &a.1);
                }
                None => self.concrete_apply(m, off, resolved, md, o),
            }
        }
        // 诊断：缓存物化进映像的组合标「⇒映像」，未物化的附原因
        let shown: Vec<String> = combos
            .iter()
            .zip(why.iter().zip(&hot))
            .take(DIAG_COMBOS)
            .map(|(c, (w, &h))| match (h, w) {
                (true, _) => format!("{c:?}⇒映像"),
                (false, Some(w)) if w != "无缓存写入" => format!("{c:?}（并：{w}）"),
                _ => format!("{c:?}"),
            })
            .collect();
        let more = combos.len().saturating_sub(DIAG_COMBOS);
        let line = format!("具体求值 {} 组实参：{}{}", combos.len(), shown.join(" "), if more > 0 { format!(" …（另 {more} 组）") } else { String::new() });
        self.concrete.diag.entry(site_name).or_default().insert(line);
        true
    }

    fn concrete_fallback(&mut self, m: usize, off: u32, site: String, why: String) -> bool {
        self.concrete.fallback.insert((m, off));
        self.concrete.diag.entry(site).or_default().insert(format!("回退：{why}"));
        false
    }

    /// 调用点实参的全部组合（笛卡尔积，超上限即失败）
    fn combos(&mut self, m: usize, off: u32, md: &MethodDesc, recv: Option<&TypeSet>, args: &[V]) -> Result<Vec<Vec<AK>>, String> {
        let mut out: Vec<Vec<AK>> = vec![vec![]];
        if let Some(s) = recv {
            out = self.recv_keys(s)?.into_iter().map(|k| vec![k]).collect();
            if out.len() > COMBO_LIMIT {
                return Err(format!("接收者超过 {COMBO_LIMIT} 个"));
            }
        }
        for (i, (p, v)) in md.params.iter().zip(args).enumerate() {
            let ks = self.arg_keys(m, off, p, v).ok_or_else(|| format!("实参 {i} 不可枚举：{v:?}"))?;
            if ks.is_empty() {
                return Err(format!("实参 {i} 尚无取值"));
            }
            if out.len() * ks.len() > COMBO_LIMIT {
                return Err(format!("实参组合超过 {COMBO_LIMIT}"));
            }
            out = out.into_iter().flat_map(|c| ks.iter().map(move |k| [c.clone(), vec![k.clone()]].concat())).collect();
        }
        Ok(out)
    }

    /// 接收者值集的枚举：只接受所指已知的类镜像
    fn recv_keys(&mut self, s: &TypeSet) -> Result<Vec<AK>, String> {
        let mut out = Vec::new();
        let mut bad: Vec<String> = s.open.iter().map(|o| format!("open({})", self.names[o as usize])).collect();
        for x in s.classes.iter() {
            match self.mirrors.get(&x) {
                Some(&c) => out.push(AK::Mirror(self.names[c as usize].clone())),
                None => bad.push(self.names[x as usize].to_string()),
            }
        }
        if !bad.is_empty() {
            return Err(format!("接收者 {} 不是所指已知的类镜像", bad.join(" / ")));
        }
        Ok(out)
    }

    fn arg_keys(&mut self, m: usize, off: u32, p: &FieldType, v: &V) -> Option<Vec<AK>> {
        if !p.is_reference() {
            return match (p, v) {
                (FieldType::Prim(b'J'), V::Long(x)) => Some(vec![AK::Long(*x)]),
                (FieldType::Prim(b'B' | b'C' | b'I' | b'S' | b'Z'), V::Int(x)) => Some(vec![AK::Int(*x)]),
                _ => None,
            };
        }
        match v {
            V::Null => Some(vec![AK::Null]),
            V::Str(s, _) => Some(vec![AK::Str(s.clone())]),
            V::Class(c, _) => Some(vec![AK::Mirror(c.clone())]),
            V::Ref { nonnull, .. } => {
                let srcs = v.srcs();
                if !field_names::names_known(&srcs, |i| self.ptaint.contains(&(m, i))) {
                    return None;
                }
                let mut ks: BTreeSet<Rc<str>> = v.lits().into_iter().collect();
                ks.extend(self.param_strs(m, off, v));
                let mut out: Vec<AK> = ks.into_iter().map(AK::Str).collect();
                if !nonnull {
                    out.push(AK::Null);
                }
                Some(out)
            }
            _ => None,
        }
    }

    /// 一组实参的求值（按 (入口, 实参) 记忆）
    fn concrete_eval(&mut self, site: &MethodSite, entry: &MemberRef, args: &[AK]) -> Rc<Result<Outcome, String>> {
        let key = (entry.clone(), args.to_vec());
        if let Some(r) = self.concrete.memo.get(&key) {
            return r.clone();
        }
        let mut vm = self.concrete.vm.take().unwrap_or_else(|| Box::new(Vm::new()));
        let env = Env { ctx: &self.ctx, cp: self.cp };
        let r = eval(&mut vm, &env, site, args);
        self.concrete.vm = Some(vm);
        let r = Rc::new(r);
        self.concrete.memo.insert(key, r.clone());
        r
    }
}

/// 冷 / 热两次求值，轨迹与结果取并；任一次失败即失败。热求值的结果另记一份：缓存写入可物化进映像时按它入闭包
fn eval(vm: &mut Vm, env: &Env, site: &MethodSite, args: &[AK]) -> Result<Outcome, String> {
    let mut out = Outcome::default();
    let mut hot = Outcome::default();
    let r = (|| {
        for pass in 0..2 {
            vm.epoch += 1;
            vm.trace = Trace::default();
            vm.steps = 0;
            vm.frames.clear();
            let cargs = args.iter().map(|a| arg_value(vm, env, a)).collect::<R<Vec<CV>>>()?;
            let r = vm.call(env, site, cargs);
            let trace = std::mem::take(&mut vm.trace);
            let (ret, thrown) = match r {
                Ok(v) => (v, None),
                Err(Flow::Throw(o)) => (None, Some(CV::R(o))),
                Err(Flow::Implicit(k)) => (None, Some(CV::R(vm.implicit(env, k)?))),
                Err(f) => return Err(f),
            };
            let mut sn = snap::Snap::new(vm, env, &mut out.objs);
            if let Some(v) = ret {
                let x = sn.value(v)?;
                out.rets.push(x);
            }
            if let Some(v) = thrown {
                let x = sn.value(v)?;
                out.thrown.push(x);
            }
            for (f, v) in &trace.memo_vals {
                let x = sn.value(*v)?;
                out.memo.push((f.clone(), x));
            }
            if pass == 1 && !env.cfg().image_memo_fields.is_empty() {
                let mut sn = snap::Snap::new(vm, env, &mut hot.objs);
                if let Some(v) = ret {
                    let x = sn.value(v)?;
                    hot.rets.push(x);
                }
                if let Some(v) = thrown {
                    let x = sn.value(v)?;
                    hot.thrown.push(x);
                }
                let mut t = trace.clone();
                t.inited.extend(t.touched.iter().cloned());
                merge(&mut hot, t);
            }
            merge(&mut out, trace);
        }
        Ok(())
    })();
    if r.is_ok() && !env.cfg().image_memo_fields.is_empty() {
        match persist::collect(vm, env) {
            Ok(frags) => out.alt = Some(Box::new((frags, hot))),
            Err(w) => out.alt_why = Some(w),
        }
    }
    vm.rollback();
    vm.frames.clear();
    match r {
        Ok(()) => Ok(out),
        Err(Flow::Fail(w)) => Err(w),
        Err(Flow::Throw(_) | Flow::Implicit(_)) => Err("实参构造抛出异常".into()),
        Err(Flow::Defer(w)) => Err(w),
    }
}

/// 一组实参入闭包的结果：热求值（缓存物化进映像）或冷 / 热之并
fn pick(o: &Outcome, hot: bool) -> &Outcome {
    match &o.alt {
        Some(a) if hot => &a.1,
        _ => o,
    }
}

fn arg_value(vm: &mut Vm, env: &Env, a: &AK) -> R<CV> {
    Ok(match a {
        AK::Str(s) => CV::R(vm.string(env, &s.encode_utf16().collect::<Vec<_>>())?),
        AK::Mirror(c) => CV::R(vm.mirror(env, c)?),
        AK::Int(x) => CV::I(*x),
        AK::Long(x) => CV::J(*x),
        AK::Null => CV::N,
    })
}

fn merge(out: &mut Outcome, t: Trace) {
    for (k, pcs) in t.pcs {
        out.pcs.entry(k).or_default().extend(pcs);
    }
    for (k, ts) in t.calls {
        out.calls.entry(k).or_default().extend(ts);
    }
    for (k, ps) in t.puts {
        out.puts.entry(k).or_default().extend(ps);
    }
    out.inited.extend(t.inited);
}
