//! 引擎：确定性具体求值（`vm_intrinsics.toml [concrete]`）。
//!
//! 入口方法的调用点实参可枚举（字符串字面量 / 类镜像 / 整数 / null，且来源全部可取回）时，按实参逐组
//! 具体执行入口（解释器见 `concrete/interp.rs`）。全部组合成功：
//! - 轨迹上的方法以具体上下文入闭包，只按轨迹执行过的指令登记依赖，不做抽象分析（`concrete/apply.rs`）；
//! - 结果对象图物化为类型流（实例化类型、字段值集、数组元素、返回值集）；
//! - 不接抽象调用边。
//! 任一组合失败（不可建模的指令 / native / 共享状态改写）即整个调用点回退抽象调用边；回退后不再撤回。
//! 每组实参先冷后热各求值一次（内存缓存字段先写后读），轨迹取并；求值后撤销对映像的缓存写入。
//! 冷求值兼写映像缓存与非映像缓存时插一次温求值（只撤销非映像缓存），映像缓存物化后按温 / 热之并入闭包。

mod apply;
mod boot;
mod boot_cfg;
pub mod boot_image;
mod boot_slots;
mod export;
mod ext_export;
mod ext_init;
mod indy;
mod init;
mod interp;
mod journal;
mod members;
mod natives;
pub(super) mod persist;
mod reflect;
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

pub(super) use self::ext_init::ExtVm;

use self::snap::{MObj, MV};
use self::vm::*;
use super::*;

/// 具体上下文的类型名（方法克隆上下文取其类型序号）
const CONCRETE_CTX: &str = "@concrete";
/// 单个调用点的实参组合数上限（含形参的笛卡尔积）
const COMBO_LIMIT: usize = 64;
/// 只按接收者枚举（实例方法入口、无其他形参）时的接收者数上限：各接收者逐个独立求值、按 (入口, 镜像) 记忆，
/// 代价随接收者数线性增长、无乘积膨胀，上限只防病态值集。散列表树化桶的键比较类查询以全部可比较键类型的镜像为
/// 接收者（HelloWorld 实测 > 64 个），按 64 截断时整点回退抽象调用边，把泛型签名解析整棵树拉进闭包
const RECV_LIMIT: usize = 4096;
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
    /// 其中接收者不是本次求值新分配对象的写入（见 `Trace::shared_puts`）
    shared_puts: BTreeMap<MemberRef, Vec<Put>>,
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
    /// 不可物化的原因；可物化（alt 为 Some）时为运行期仍重算的非映像缓存（温求值，诊断）
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
    /// 回退后逐组应用时已处理过的 (调用方节点, 偏移, 实参)：单组的结论（应用 / 失败 / 引用容器）与次序无关，处理一次即可
    partial_tried: HashSet<(usize, u32, Vec<AK>)>,
    /// 结果按对象物化的类：`[concrete] object_results` 各入口字节码自身 `new` 的类（分析前按字节码确定，
    /// 按对象读的形参门据此放行，见 `obj_fields.rs::obj_param_set`）
    pobj_types: HashSet<Rc<str>>,
    /// 诊断：按对象物化出的抽象对象数 / 回退后应用的已知常量组合数（`summary.perf.pobj`；对象数每翻倍报一行进度）
    pub(super) pobj_stats: [u64; 2],
    /// 诊断：调用点 → 各方法上下文的结论（成功时列出实参组合，按上下文分别求值的调用点逐条记录）
    pub(super) diag: BTreeMap<String, BTreeSet<String>>,
}

/// 实测用（临时）：回退后逐组应用开关
const MEASURE_PARTIAL: bool = true;

impl<'a> Engine<'a> {
    pub(super) fn concrete_init(&mut self) {
        self.concrete.ctx = self.id(CONCRETE_CTX);
        let mut types: HashSet<Rc<str>> = HashSet::default();
        for k in &self.man.concrete.object_results {
            let Some((head, desc)) = k.split_once(':') else { continue };
            let Some((owner, name)) = head.rsplit_once('.') else { continue };
            let code = self.h.class(owner).and_then(|cf| cf.method(name, desc).and_then(|x| x.code.clone()));
            for x in code.iter().flat_map(|c| c.insns.iter()) {
                if let (classfile::op::NEW, classfile::Operand::Class(c)) = (x.opcode, &x.operand) {
                    types.insert(Rc::from(c.as_str()));
                }
            }
        }
        self.concrete.pobj_types = types;
    }

    /// 类 t 的结果实例按对象物化（见 `pobj_types`）
    pub(super) fn per_object_type(&self, t: &str) -> bool {
        self.concrete.pobj_types.contains(t)
    }

    pub(super) fn is_concrete(&self, m: usize) -> bool {
        self.methods[m].ctx == self.concrete.ctx
    }

    /// 方法在具体上下文中的执行轨迹（指令偏移）
    pub(super) fn concrete_pcs(&self, key: &MemberRef) -> Option<&BTreeSet<u32>> {
        self.concrete.traces.get(key).map(|t| &t.0)
    }

    /// 调用 m@off → resolved（静态调用，或接收者值集为 recv 的非虚调用）按具体求值处理；返回 true 即不再接抽象调用边。
    /// 接收者尚无取值时暂不接边（接收者增长时调用方重处理）。
    /// 结果按对象物化的入口（`[concrete] object_results`）在调用点回退之后仍逐组应用实参里的已知常量组合（`concrete_known`）：
    /// 回退前已应用的组合撤不回，回退后若不再应用，结果就取决于回退前到达了哪些组合（处理次序）。逐组应用后，应用集合
    /// 恒为终态的已知常量组合（求值成功者），回退与否只取决于终态的污染与上限，二者都与次序无关
    pub(super) fn concrete_call(&mut self, m: usize, off: u32, resolved: &MemberRef, md: &MethodDesc, recv: Option<&TypeSet>, args: &[V]) -> bool {
        if self.is_concrete(m) {
            return false;
        }
        let k = self.mref_key(resolved);
        if !self.man.concrete.entries.contains(&*k) {
            return false;
        }
        let site_name = format!("{}@{off}", self.methods[m].key);
        if !self.concrete.fallback.contains(&(m, off)) {
            if recv.is_some_and(|s| s.classes.is_empty() && s.open.is_empty()) {
                return true;
            }
            match self.combos(m, off, md, recv, args) {
                Ok(c) => {
                    if self.concrete_run(m, off, resolved, md, &c, site_name.clone(), false) {
                        return true;
                    }
                }
                Err(why) => {
                    self.concrete_fallback(m, off, site_name.clone(), why);
                }
            }
        }
        if MEASURE_PARTIAL && self.concrete.fallback.contains(&(m, off)) && self.man.concrete.object_results.contains(&*k) {
            if let Some(c) = self.concrete_known(m, off, md, recv, args) {
                let c: Vec<Vec<AK>> = c.into_iter().filter(|c| self.concrete.partial_tried.insert((m, off, c.clone()))).collect();
                if !c.is_empty() {
                    self.concrete_run(m, off, resolved, md, &c, site_name, true);
                }
            }
        }
        false
    }

    /// 逐组求值并应用。partial = false：任一组失败即整点回退（返回 false）；partial = true（已回退的按对象物化入口）：
    /// 跳过失败的组合，其余照常应用
    #[allow(clippy::too_many_arguments)]
    fn concrete_run(&mut self, m: usize, off: u32, resolved: &MemberRef, md: &MethodDesc, combos: &[Vec<AK>], site_name: String, partial: bool) -> bool {
        let Some(site) = self.h.resolve_method(&resolved.owner, &resolved.name, &resolved.desc, false) else { return false };
        let mut outs = Vec::new();
        for c in combos {
            let r = self.concrete_eval(&site, resolved, c);
            if let Err(w) = &*r {
                if partial {
                    continue;
                }
                return self.concrete_fallback(m, off, site_name, format!("{c:?}：{w}"));
            }
            outs.push((c.clone(), r));
        }
        // 镜像缓存可物化进引导映像的组合按热（有温求值时温 / 热之并）入闭包（运行期缓存已命中）
        let mut why: Vec<Option<String>> = Vec::new();
        let mut hot: Vec<bool> = Vec::new();
        for (_, r) in &outs {
            let Ok(o) = &**r else {
                hot.push(false);
                why.push(None);
                continue;
            };
            let w = match (&o.alt, &o.alt_why) {
                (Some(a), _) => self.image_memo_prepare(&a.0).err(),
                (None, w) => w.clone(),
            };
            let h = w.is_none() && o.alt.is_some();
            hot.push(h);
            why.push(if h { o.alt_why.clone() } else { w });
        }
        // 结果引用映像中的容器形态对象的组合不可应用：整点回退，或（partial）跳过该组
        let mut bad: Vec<Option<Rc<str>>> = Vec::new();
        for ((_, r), &h) in outs.iter().zip(&hot) {
            let types: Vec<Rc<str>> = r.as_ref().as_ref().ok().map(|o| apply::image_types(pick(o, h)).cloned().collect()).unwrap_or_default();
            bad.push(types.into_iter().find(|t| self.container(t)));
        }
        if !partial {
            if let Some(t) = bad.iter().flatten().next() {
                let t = t.clone();
                return self.concrete_fallback(m, off, site_name, format!("结果引用映像中的容器形态对象 {t}"));
            }
        }
        if partial && bad.iter().all(Option::is_some) {
            // 无可应用的组合：不接具体入口（是否有可应用组合与次序无关）
            return false;
        }
        let entry = self.method_ctx(resolved.clone(), self.concrete.ctx, Via::method("concrete", m, Some(off)));
        self.dispatch.entry((m, off)).or_default().insert(entry);
        self.callers.entry(entry).or_default().insert(m);
        for (((c, r), &h), b) in outs.into_iter().zip(&hot).zip(&bad) {
            if b.is_some() || !self.concrete.applied.insert((m, off, c.clone())) {
                continue;
            }
            if partial {
                self.concrete.pobj_stats[1] += 1;
                // 逐组记诊断：应用集合单调，诊断行集合也与次序无关
                self.concrete.diag.entry(site_name.clone()).or_default().insert(format!("回退后应用已知常量组合 {c:?}"));
            }
            let Ok(o) = &*r else { continue };
            match o.alt.as_ref().filter(|_| h) {
                Some(a) => {
                    self.image_memo_apply((m, off), &a.0);
                    self.concrete_apply(m, off, resolved, md, &a.1);
                }
                None => self.concrete_apply(m, off, resolved, md, o),
            }
        }
        if partial {
            return false;
        }
        // 诊断：缓存物化进映像的组合标「⇒映像」，未物化的附原因
        // 未物化（按冷 / 热之并入闭包）的组合全部列出，物化的只列前 DIAG_COMBOS 组
        let mut n_hot = 0;
        let shown: Vec<String> = combos
            .iter()
            .zip(why.iter().zip(&hot))
            .filter(|(_, (_, &h))| {
                n_hot += usize::from(h);
                !h || n_hot <= DIAG_COMBOS
            })
            .map(|(c, (w, &h))| match (h, w) {
                (true, Some(w)) => format!("{c:?}⇒映像（温：{w}）"),
                (true, None) => format!("{c:?}⇒映像"),
                (false, Some(w)) if w != "无缓存写入" => format!("{c:?}（并：{w}）"),
                _ => format!("{c:?}"),
            })
            .collect();
        let more = combos.len() - shown.len();
        let line = format!("具体求值 {} 组实参：{}{}", combos.len(), shown.join(" "), if more > 0 { format!(" …（另 {more} 组）") } else { String::new() });
        self.concrete.diag.entry(site_name).or_default().insert(line);
        true
    }

    /// 调用点是否已回退抽象调用边
    pub(super) fn concrete_fell_back(&self, w: (usize, u32)) -> bool {
        self.concrete.fallback.contains(&w)
    }

    fn concrete_fallback(&mut self, m: usize, off: u32, site: String, why: String) -> bool {
        self.concrete.fallback.insert((m, off));
        let at = self.ctx_label(m);
        self.concrete.diag.entry(site).or_default().insert(format!("回退（{at}）：{why}"));
        false
    }

    /// 调用点实参的全部组合（笛卡尔积，超上限即失败）
    fn combos(&mut self, m: usize, off: u32, md: &MethodDesc, recv: Option<&TypeSet>, args: &[V]) -> Result<Vec<Vec<AK>>, String> {
        let mut out: Vec<Vec<AK>> = vec![vec![]];
        if let Some(s) = recv {
            out = self.recv_keys(s)?.into_iter().map(|k| vec![k]).collect();
            let limit = if md.params.is_empty() { RECV_LIMIT } else { COMBO_LIMIT };
            if out.len() > limit {
                return Err(format!("接收者超过 {limit} 个"));
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

    /// 已回退调用点的已知常量组合：接收者只取所指已知的类镜像，引用实参只取字面量与形参常量（含 null），不看污染、
    /// 不设组合上限（污染与上限决定的是能否不接抽象边，已回退的点抽象边已接，这里只补按对象的结果）。
    /// 任一实参无常量取值（或原始类型实参非常量）即无组合
    fn concrete_known(&mut self, m: usize, off: u32, md: &MethodDesc, recv: Option<&TypeSet>, args: &[V]) -> Option<Vec<Vec<AK>>> {
        let mut out: Vec<Vec<AK>> = vec![vec![]];
        if let Some(s) = recv {
            out = s.classes.iter().filter_map(|x| self.mirrors.get(&x).map(|&c| vec![AK::Mirror(self.names[c as usize].clone())])).collect();
        }
        for (p, v) in md.params.iter().zip(args) {
            let ks = self.arg_keys_with(m, off, p, v, false)?;
            out = out.into_iter().flat_map(|c| ks.iter().map(move |k| [c.clone(), vec![k.clone()]].concat())).collect();
        }
        (!out.is_empty()).then_some(out)
    }

    fn arg_keys(&mut self, m: usize, off: u32, p: &FieldType, v: &V) -> Option<Vec<AK>> {
        self.arg_keys_with(m, off, p, v, true)
    }

    /// strict：引用实参要求名字全部已知（未被污染）；否则只取已知部分
    fn arg_keys_with(&mut self, m: usize, off: u32, p: &FieldType, v: &V, strict: bool) -> Option<Vec<AK>> {
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
                if strict && !field_names::names_known(&srcs, |i| self.ptaint.contains(&(m, i))) {
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

/// 冷 / 热两次求值，轨迹与结果取并；任一次失败即失败。热求值的结果另记一份：缓存写入可物化进映像时按它入闭包。
/// 冷求值同时写了映像缓存字段与非映像缓存（如别的镜像缓存字段）时，只撤销后者再求值一次（温）：运行期首次执行时
/// 映像缓存已在、非映像缓存为空，执行的即温路径，此后为热路径；映像片段只取映像缓存字段，按温 / 热之并入闭包
fn eval(vm: &mut Vm, env: &Env, site: &MethodSite, args: &[AK]) -> Result<Outcome, String> {
    let mut out = Outcome::default();
    let mut hot = Outcome::default();
    let img = !env.cfg().image_memo_fields.is_empty();
    let mut partial = false;
    let r = (|| {
        let mut pass = 0;
        while pass < 2 {
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
            let warm_next = pass == 0 && img && !partial && persist::mixed(vm, env);
            if (pass == 1 || partial) && img {
                let mut sn = snap::Snap::new(vm, env, &mut hot.objs);
                if let Some(v) = ret {
                    let x = sn.value(v)?;
                    hot.rets.push(x);
                }
                if let Some(v) = thrown {
                    let x = sn.value(v)?;
                    hot.thrown.push(x);
                }
                merge(&mut hot, trace.clone());
            }
            merge(&mut out, trace);
            if warm_next {
                // 温：只撤销非映像缓存的写入，映像缓存保留（撤销日志里留待求值结束整体撤销）
                persist::rollback_non_image(vm, env);
                partial = true;
                continue;
            }
            pass += 1;
        }
        Ok(())
    })();
    if r.is_ok() && img {
        match persist::collect(vm, env, partial) {
            Ok((frags, note)) => {
                out.alt = Some(Box::new((frags, hot)));
                out.alt_why = note;
            }
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
    for (k, ps) in t.shared_puts {
        out.shared_puts.entry(k).or_default().extend(ps);
    }
    // 按「请求初始化」（touched）登记，不按「本次求值触发了初始化」（inited）：后者取决于共享 VM 里此前哪次求值
    // 先初始化了该类（如先求值的组合按热路径入闭包、其冷路径完成的初始化不登记），闭包随求值次序变化。
    // 请求集合只取决于本组实参的执行路径
    out.inited.extend(t.inited);
    out.inited.extend(t.touched);
}
