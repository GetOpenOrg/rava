//! 构建期引导映像：按 `[concrete.boot]` 执行 VM 预初始化与引导阶段，产出映像摘要与审计报告
//! （计划 2026-10-05-boot-image-evaluator §3、§6 第 1 步）。
//!
//! 摘要按规范形态计算，与堆下标、分析器内部表的遍历次序无关：根依次为静态字段（按声明类、字段名）、
//! VM 构造对象（按序）、类镜像（按类型名）、驻留字符串（按内容）、残差记录（按序）；对象按首次到达的
//! 广度优先次序编号，内容含类型、实例字段（按声明类、字段名，缺省值不计）、数组元素、身份哈希与延迟标记。

use std::collections::VecDeque;
use std::fmt::Write as _;

use serde_json::json;

use super::boot::PhaseEnd;
use super::journal::Rec;
use super::vm::*;
use super::*;

/// 映像求值结果
pub struct BootImage {
    pub ok: bool,
    /// 规范摘要（128 位十六进制）
    pub digest: String,
    /// closure.json `summary.boot_image`
    pub json: serde_json::Value,
    /// 审计报告（Markdown，`rava audit boot`）
    pub report: String,
}

/// FNV-1a 双通道
struct Fnv(u64, u64);

impl Fnv {
    fn new() -> Self {
        Fnv(0xcbf2_9ce4_8422_2325, 0x6c62_272e_07bb_0142)
    }
    fn bytes(&mut self, b: &[u8]) {
        for &x in b {
            self.0 = (self.0 ^ u64::from(x)).wrapping_mul(0x0100_0000_01b3);
            self.1 = (self.1 ^ u64::from(x)).wrapping_mul(0x0000_0100_0000_01b3 ^ 0x5bd1_e995);
        }
        self.0 = (self.0 ^ 0xff).wrapping_mul(0x0100_0000_01b3);
    }
    fn str(&mut self, s: &str) {
        self.bytes(&(s.len() as u64).to_le_bytes());
        self.bytes(s.as_bytes());
    }
    fn hex(&self) -> String {
        format!("{:016x}{:016x}", self.0, self.1)
    }
}

/// 规范编号器
struct Canon<'v> {
    vm: &'v Vm,
    ids: HashMap<u32, u32>,
    queue: VecDeque<u32>,
    h: Fnv,
}

impl Canon<'_> {
    fn value(&mut self, v: CV) {
        match v {
            CV::I(x) => self.h.str(&format!("I{x}")),
            CV::J(x) => self.h.str(&format!("J{x}")),
            CV::F(x) => self.h.str(&format!("F{:08x}", x.to_bits())),
            CV::D(x) => self.h.str(&format!("D{:016x}", x.to_bits())),
            CV::N => self.h.str("N"),
            CV::R(o) => {
                let n = self.ids.len() as u32;
                let id = *self.ids.entry(o).or_insert_with(|| {
                    self.queue.push_back(o);
                    n
                });
                self.h.str(&format!("R{id}"));
            }
        }
    }

    fn drain(&mut self) {
        while let Some(o) = self.queue.pop_front() {
            let vm = self.vm;
            let h = &vm.heap[o as usize];
            self.h.str(&h.ty);
            if let Some(x) = vm.ihash.get(&o) {
                self.h.str(&format!("#{x}"));
            }
            if let Some(w) = vm.deferred.get(&o) {
                self.h.str(&format!("deferred:{w}:{}", vm.bj.placeholders.contains(&o)));
            }
            match &h.body {
                Body::Inst(fs) => {
                    let mut fs: Vec<(&(Rc<str>, Rc<str>), CV)> = fs.iter().filter(|(_, v)| !is_zero(*v)).map(|(k, v)| (&vm.fnames[*k as usize], *v)).collect();
                    fs.sort_by(|a, b| a.0.cmp(b.0));
                    self.h.str(&format!("inst{}", fs.len()));
                    for ((d, n), v) in fs {
                        self.h.str(d);
                        self.h.str(n);
                        self.value(v);
                    }
                }
                Body::Arr(a) => {
                    self.h.str(&format!("arr{}", a.len()));
                    for &v in a {
                        self.value(v);
                    }
                }
                Body::Lam(l) => {
                    self.h.str(&format!("lam {} {}", l.iface, l.imp.member));
                    for &v in &l.captured {
                        self.value(v);
                    }
                }
            }
        }
    }
}

fn is_zero(v: CV) -> bool {
    match v {
        CV::I(0) | CV::J(0) | CV::N => true,
        CV::F(x) => x.to_bits() == 0,
        CV::D(x) => x.to_bits() == 0,
        _ => false,
    }
}

/// 映像规模（从根可达）
struct Size {
    objects: usize,
    slots: usize,
    types: BTreeSet<Rc<str>>,
}

fn reachable(vm: &Vm) -> Size {
    let mut stack: Vec<u32> = vm.statics.values().filter_map(|v| if let CV::R(o) = v { Some(*o) } else { None }).collect();
    stack.extend(vm.boot_objs.iter().copied());
    stack.extend(vm.strings.values().copied());
    stack.extend(vm.mirrors.values().copied());
    for r in &vm.bj.recs {
        match r {
            Rec::Call { args, ph, .. } | Rec::Native { args, ph, .. } => {
                stack.extend(args.iter().filter_map(|v| if let CV::R(o) = v { Some(*o) } else { None }));
                stack.extend(ph.iter().copied());
            }
            Rec::Region { locals, .. } => stack.extend(locals.iter().filter_map(|v| if let CV::R(o) = v { Some(*o) } else { None })),
            Rec::RuntimeInit { .. } => {}
        }
    }
    let mut seen = vec![false; vm.heap.len()];
    let mut s = Size { objects: 0, slots: 0, types: BTreeSet::new() };
    while let Some(o) = stack.pop() {
        let i = o as usize;
        if i >= seen.len() || seen[i] {
            continue;
        }
        seen[i] = true;
        s.objects += 1;
        let h = &vm.heap[i];
        if !h.ty.starts_with('[') {
            s.types.insert(h.ty.clone());
        }
        let push = |v: &CV, st: &mut Vec<u32>| {
            if let CV::R(r) = v {
                st.push(*r)
            }
        };
        match &h.body {
            Body::Inst(fs) => {
                s.slots += fs.len();
                fs.iter().for_each(|(_, v)| push(v, &mut stack));
            }
            Body::Arr(a) => {
                s.slots += a.len();
                a.iter().for_each(|v| push(v, &mut stack));
            }
            Body::Lam(l) => l.captured.iter().for_each(|v| push(v, &mut stack)),
        }
    }
    s
}

fn digest(vm: &Vm) -> String {
    let mut c = Canon { vm, ids: HashMap::default(), queue: VecDeque::new(), h: Fnv::new() };
    let mut statics: Vec<(&(Rc<str>, Rc<str>), CV)> = vm.statics.iter().map(|(k, v)| (&vm.fnames[*k as usize], *v)).collect();
    statics.sort_by(|a, b| a.0.cmp(b.0));
    c.h.str(&format!("statics{}", statics.len()));
    for ((d, n), v) in statics {
        c.h.str(d);
        c.h.str(n);
        c.value(v);
    }
    c.h.str("boot_objs");
    for &o in &vm.boot_objs {
        c.value(CV::R(o));
    }
    let mut mirrors: Vec<(&Rc<str>, u32)> = vm.mirrors.iter().map(|(t, o)| (t, *o)).collect();
    mirrors.sort();
    c.h.str(&format!("mirrors{}", mirrors.len()));
    for (t, o) in mirrors {
        c.h.str(t);
        c.value(CV::R(o));
    }
    let mut strings: Vec<(&Vec<u16>, u32)> = vm.strings.iter().map(|(s, o)| (s, *o)).collect();
    strings.sort();
    c.h.str(&format!("strings{}", strings.len()));
    for (s, o) in strings {
        c.h.str(&String::from_utf16_lossy(s));
        c.value(CV::R(o));
    }
    c.h.str(&format!("recs{}", vm.bj.recs.len()));
    for r in &vm.bj.recs {
        match r {
            Rec::RuntimeInit { class, .. } => c.h.str(&format!("init {class}")),
            Rec::Call { phase, off, callee, args, ph, .. } => {
                c.h.str(&format!("call {phase}@{off} {callee}"));
                args.iter().for_each(|&v| c.value(v));
                c.value(ph.map_or(CV::N, CV::R));
            }
            Rec::Native { callee, args, ph } => {
                c.h.str(&format!("native {callee}"));
                args.iter().for_each(|&v| c.value(v));
                c.value(ph.map_or(CV::N, CV::R));
            }
            Rec::Region { phase, start, end, locals, .. } => {
                c.h.str(&format!("region {phase}@{start}..{end:?}"));
                locals.iter().for_each(|&v| c.value(v));
            }
        }
    }
    c.drain();
    let mut inited: Vec<&Rc<str>> = vm.init.iter().filter(|(_, s)| matches!(s, Init::Done)).map(|(k, _)| k).collect();
    inited.sort();
    c.h.str(&format!("inited{}", inited.len()));
    for k in inited {
        c.h.str(k);
        c.h.str(if vm.opaque.contains(k) { "rt" } else { "bt" });
    }
    let mut cells: Vec<&(Rc<str>, i64)> = vm.cells.iter().collect();
    cells.sort();
    for (n, v) in cells {
        c.h.str(&format!("cell {n}={v}"));
    }
    for (k, v) in &vm.vm_tables {
        c.h.str(&format!("table {k}={v}"));
    }
    for (n, w) in &vm.bj.host_scalars {
        c.h.str(&format!("host {n} {w}"));
    }
    c.h.hex()
}

fn flow_text(vm: &Vm, f: &Flow) -> String {
    match f {
        Flow::Fail(w) => format!("失败：{w}"),
        Flow::Defer(w) => format!("延迟值未被残差化吸收：{w}"),
        Flow::Throw(o) => format!("抛出 {}", vm.ty(*o)),
        Flow::Implicit(k) => format!("隐式异常 {k}"),
    }
}

impl<'a> Engine<'a> {
    /// 构建期引导映像（`[concrete.boot] calls` 为空或首个阶段方法所在类不在类路径上时为 None）
    pub fn boot_image(&self) -> Option<BootImage> {
        let boot = &self.man.concrete.boot;
        let first = boot.calls.first().and_then(|(m, _)| super::super::seeds::parse_member(m))?;
        self.h.class(&first.owner)?;
        let t0 = std::time::Instant::now();
        let mut vm = Vm::new();
        vm.boot = true;
        vm.image = 1;
        vm.step_limit = 2_000_000_000;
        let env = Env { ctx: &self.ctx, cp: self.cp };
        let jdk = self.h.class(OBJECT).map_or(0, |c| i64::from(c.major) - 44);
        let mut phases: Vec<serde_json::Value> = Vec::new();
        let mut rows: Vec<String> = Vec::new();
        let mut error: Option<String> = None;
        let pre = boot_pre(&mut vm, &env);
        match pre {
            Ok(()) => rows.push(format!("| VM 预初始化（{} 类 + {} 个 VM 构造对象） | 完成 | {} | {} | {} | {} |", boot.init.len(), boot.objects.len(), vm.steps, vm.heap.len(), vm.done_log.len(), t0.elapsed().as_millis())),
            Err(f) => error = Some(format!("VM 预初始化：{}", flow_text(&vm, &f))),
        }
        for (m, args) in &boot.calls {
            if error.is_some() {
                break;
            }
            let r = (|| -> R<PhaseEnd> {
                let mr = super::super::seeds::parse_member(m).map_or_else(|| fail(format!("成员格式 {m}")), Ok)?;
                let md = parse_method(&mr.desc).map_or_else(|| fail("描述符"), Ok)?;
                let cargs: Vec<CV> = md.params.iter().zip(args).map(|(p, &x)| if matches!(p, FieldType::Prim(b'J')) { CV::J(x) } else { CV::I(x as i32) }).collect();
                vm.boot_phase(&env, &mr, cargs)
            })();
            let res = match &r {
                Ok(PhaseEnd::Value(Some(v))) => format!("完成 → {v:?}"),
                Ok(PhaseEnd::Value(None)) => "完成".to_string(),
                Ok(PhaseEnd::Residual) => "剩余部分运行期执行".to_string(),
                Err(f) => flow_text(&vm, f),
            };
            phases.push(json!({ "call": m, "result": res, "steps": vm.steps }));
            rows.push(format!("| `{m}` | {res} | {} | {} | {} | {} |", vm.steps, vm.heap.len(), vm.done_log.len(), t0.elapsed().as_millis()));
            if r.is_err() {
                error = Some(format!("{m}：{res}"));
            }
        }
        let elapsed = t0.elapsed().as_millis();
        let size = reachable(&vm);
        let dg = digest(&vm);
        let ok = error.is_none();
        let rt_init: Vec<(String, String)> = vm.bj.recs.iter().filter_map(|r| if let Rec::RuntimeInit { class, why } = r { Some((class.to_string(), why.clone())) } else { None }).collect();
        let calls: Vec<String> = vm.bj.recs.iter().filter_map(|r| if let Rec::Call { phase, off, callee, why, ph, .. } = r { Some(format!("`{phase}@{off}` → `{callee}`{}：{why}", if ph.is_some() { "（结果为占位对象）" } else { "" })) } else { None }).collect();
        let regions: Vec<String> = vm.bj.recs.iter().filter_map(|r| if let Rec::Region { phase, start, end, why, .. } = r { Some(format!("`{phase}` [{start}, {})：{why}", end.map_or("出口".to_string(), |e| e.to_string()))) } else { None }).collect();
        let natives: Vec<String> = vm.bj.recs.iter().filter_map(|r| if let Rec::Native { callee, ph, .. } = r { Some(format!("`{callee}`{}", if ph.is_some() { "（结果为占位对象）" } else { "" })) } else { None }).collect();
        let json = json!({
            "status": if ok { "ok" } else { "failed" },
            "digest": dg,
            "jdk": jdk,
            "phases": phases,
            "error": error,
            "steps": vm.steps,
            "heap": vm.heap.len(),
            "objects": size.objects,
            "slots": size.slots,
            "statics": vm.statics.len(),
            "types": size.types.len(),
            "inited": vm.done_log.len(),
            "runtime_init": rt_init.iter().map(|(c, _)| c.clone()).collect::<Vec<_>>(),
            "residual_calls": calls.len(),
            "regions": regions.len(),
            "replay_natives": natives.len(),
            "host_scalars": vm.bj.host_scalars.len(),
        });
        let mut r = String::new();
        let _ = writeln!(r, "# 构建期引导映像审计（JDK {jdk}，{}）\n", std::env::consts::OS);
        let _ = writeln!(r, "- 结论：**{}**{}", if ok { "通过" } else { "失败" }, error.as_ref().map_or(String::new(), |e| format!("——{e}")));
        let _ = writeln!(r, "- 摘要：`{dg}`");
        let _ = writeln!(r, "- 求值耗时 {elapsed} ms，指令步数 {}，堆对象 {}\n", vm.steps, vm.heap.len());
        let _ = writeln!(r, "## 阶段（累计值）\n\n| 阶段 | 结局 | 步数 | 堆对象 | 已初始化类 | 耗时 ms |\n|---|---|---:|---:|---:|---:|");
        for row in &rows {
            let _ = writeln!(r, "{row}");
        }
        let _ = writeln!(r, "\n## 映像规模\n\n- 可达对象 {}，槽位 {}，静态字段 {}，非数组类型 {}；驻留字符串 {}，镜像 {}，身份哈希 {}", size.objects, size.slots, vm.statics.len(), size.types.len(), vm.strings.len(), vm.mirrors.len(), vm.ihash.len());
        let _ = writeln!(r, "- 已初始化类 {}（其中运行期初始化 {}）", vm.done_log.len() + rt_init.len(), rt_init.len());
        let _ = writeln!(r, "- VM 表：{:?}；VM 单元：{:?}", vm.vm_tables, vm.cells);
        let _ = writeln!(r, "\n## 运行期初始化类（{}）\n", rt_init.len());
        for (c, w) in &rt_init {
            let _ = writeln!(r, "- `{c}`：{w}");
        }
        let _ = writeln!(r, "\n## 残差调用（{}）\n", calls.len());
        for c in &calls {
            let _ = writeln!(r, "- {c}");
        }
        let _ = writeln!(r, "\n## 残差区段（{}）\n", regions.len());
        for c in &regions {
            let _ = writeln!(r, "- {c}");
        }
        let _ = writeln!(r, "\n## 启动重放 native（{}）\n", natives.len());
        for c in &natives {
            let _ = writeln!(r, "- {c}");
        }
        let _ = writeln!(r, "\n## 宿主标量（构建期取零值，第 2 步污点与重算槽，{}）\n", vm.bj.host_scalars.len());
        for (n, c) in &vm.bj.host_scalars {
            let _ = writeln!(r, "- `{n}` ← `{c}`");
        }
        if let Some(fs) = vm.fail_frames.as_ref().or(vm.throw_frames.as_ref()).filter(|_| !ok) {
            let _ = writeln!(r, "\n## 失败栈（外→内）\n");
            for f in fs.iter().rev().take(30).rev() {
                let _ = writeln!(r, "    {f}");
            }
        }
        let _ = writeln!(r, "\n## 已初始化类（构建期，按完成次序）\n\n{}", vm.done_log.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" "));
        Some(BootImage { ok, digest: dg, json, report: r })
    }
}

/// VM 预初始化：`init` 的类与 `objects` 的 VM 构造对象（先分配登记，再执行构造器）
fn boot_pre(vm: &mut Vm, env: &Env) -> R<()> {
    let boot = &env.cfg().boot;
    for c in &boot.init {
        vm.ensure_init(env, c)?;
    }
    for spec in &boot.objects {
        let (c, d) = (&spec[0], &spec[1]);
        vm.ensure_init(env, c)?;
        let o = vm.alloc(c, Body::Inst(Vec::new()));
        vm.boot_objs.push(o);
        let mut args = vec![CV::R(o)];
        for a in &spec[2..] {
            args.push(if let Some(k) = a.strip_prefix('@') {
                let i = k.parse::<usize>().map_or_else(|_| fail("对象下标"), Ok)?;
                CV::R(*vm.boot_objs.get(i).map_or_else(|| fail("对象下标越界"), Ok)?)
            } else if let Some(t) = a.strip_prefix("str:") {
                CV::R(vm.string(env, &t.encode_utf16().collect::<Vec<_>>())?)
            } else {
                return fail(format!("引导对象实参 {a}"));
            });
        }
        let site = vm.resolve(env, &MemberRef { owner: c.clone(), name: "<init>".into(), desc: d.clone() }, false)?;
        vm.call(env, &site, args)?;
    }
    Ok(())
}
