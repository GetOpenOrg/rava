//! 具体求值器的状态：值、堆、静态区、类初始化、字符串字面量与类镜像。
//!
//! 堆对象带纪元：0 = 静态映像（`<clinit>` 具体执行时分配，跨求值共享），k = 第 k 次求值的分配。
//! 求值只能改写本纪元的对象；映像对象只有清单声明的内存缓存字段可写（写入记撤销日志，求值后复原），
//! 其余写入映像即求值失败（共享状态被改写，轨迹不再只取决于实参）。

use std::sync::Arc;

use resolve::hierarchy::MethodSite;

use super::*;

/// 单次求值的指令步数上限
pub(super) const STEP_LIMIT: u64 = 4_000_000;
/// 调用深度上限
pub(super) const DEPTH_LIMIT: usize = 400;

/// 具体值（long / double 在操作数栈上占一项，局部变量表里占两槽，第二槽为 `N`）
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum CV {
    I(i32),
    J(i64),
    F(f32),
    D(f64),
    N,
    R(u32),
}

impl CV {
    pub(super) fn wide(self) -> bool {
        matches!(self, CV::J(_) | CV::D(_))
    }
    pub(super) fn i(self) -> R<i32> {
        match self {
            CV::I(v) => Ok(v),
            v => fail(format!("期望 int：{v:?}")),
        }
    }
    pub(super) fn j(self) -> R<i64> {
        match self {
            CV::J(v) => Ok(v),
            v => fail(format!("期望 long：{v:?}")),
        }
    }
    pub(super) fn f(self) -> R<f32> {
        match self {
            CV::F(v) => Ok(v),
            v => fail(format!("期望 float：{v:?}")),
        }
    }
    pub(super) fn d(self) -> R<f64> {
        match self {
            CV::D(v) => Ok(v),
            v => fail(format!("期望 double：{v:?}")),
        }
    }
    /// 引用（null = None）
    pub(super) fn r(self) -> R<Option<u32>> {
        match self {
            CV::N => Ok(None),
            CV::R(o) => Ok(Some(o)),
            v => fail(format!("期望引用：{v:?}")),
        }
    }
    /// 非空引用（null 即空指针隐式异常）
    pub(super) fn obj(self) -> R<u32> {
        self.r()?.map_or_else(|| implicit("null"), Ok)
    }
    /// 描述符的缺省值
    pub(super) fn zero(desc: &str) -> CV {
        match desc.as_bytes().first() {
            Some(b'J') => CV::J(0),
            Some(b'F') => CV::F(0.0),
            Some(b'D') => CV::D(0.0),
            Some(b'L' | b'[') => CV::N,
            _ => CV::I(0),
        }
    }
}

/// 非正常完成：抛出 Java 异常（对象）/ 求值失败（不可建模，整次回退抽象调用边）
#[derive(Debug)]
pub(super) enum Flow {
    Throw(u32),
    /// 隐式异常（种类见清单 `[concrete.implicit]`）：由解释循环分配异常对象后按 `Throw` 处理
    Implicit(&'static str),
    Fail(String),
}

pub(super) type R<T> = Result<T, Flow>;

pub(super) fn implicit<T>(kind: &'static str) -> R<T> {
    Err(Flow::Implicit(kind))
}

pub(super) fn fail<T>(why: impl Into<String>) -> R<T> {
    Err(Flow::Fail(why.into()))
}

/// lambda 对象（LambdaMetafactory 产物）
#[derive(Debug)]
pub(in crate::engine) struct Lam {
    pub iface: String,
    pub markers: Vec<String>,
    pub sam: String,
    pub imp: classfile::MethodHandle,
    /// LambdaMetafactory 静态实参（物化为抽象 lambda 时计算适配表）
    pub bargs: Rc<[Const]>,
    /// indy 调用点描述符（形参为捕获值类型）
    pub desc: Rc<str>,
    pub captured: Vec<CV>,
}

pub(super) enum Body {
    /// 实例字段（字段键 → 值；缺席 = 缺省值）
    Inst(Vec<(u32, CV)>),
    Arr(Vec<CV>),
    Lam(Rc<Lam>),
}

pub(super) struct HObj {
    /// binary name / 数组描述符（lambda 为函数式接口名）
    pub ty: Rc<str>,
    pub epoch: u32,
    pub body: Body,
}

/// 已解析字段
pub(super) struct FRes {
    pub key: u32,
    pub decl: Rc<str>,
    pub name: String,
    pub desc: String,
    pub fin: bool,
    pub memo: bool,
    pub constant: Option<classfile::Const>,
}

impl FRes {
    pub(super) fn mref(&self) -> MemberRef {
        MemberRef { owner: self.decl.to_string(), name: self.name.clone(), desc: self.desc.clone() }
    }
}

/// 可执行方法
pub(super) struct MInfo {
    pub key: MemberRef,
    pub site: MethodSite,
    /// 指令偏移 → 下标
    pub index: HashMap<u32, usize>,
    /// 白名单 native 操作（`[concrete.natives]`）
    pub op: Option<String>,
    /// 按字节码执行
    pub bytecode: bool,
}

impl MInfo {
    pub(super) fn code(&self) -> &classfile::Code {
        self.site.method().code.as_ref().expect("MInfo 只为有字节码的方法建立")
    }
}

#[derive(Clone)]
pub(super) enum Init {
    Running,
    Done,
    Failed(Rc<str>),
}

/// 实例字段写入值的常量格投影（物化时并入字段值集，见 concrete.rs）
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Put {
    Int(i32),
    Long(i64),
    Null,
    /// 非空引用 / 浮点
    Other,
}

/// 一次求值的轨迹（只记求值纪元内执行的字节码；`<clinit>` 由抽象分析的类初始化覆盖）
#[derive(Default)]
pub(super) struct Trace {
    /// 执行过的指令偏移（按方法）
    pub pcs: BTreeMap<MemberRef, BTreeSet<u32>>,
    /// 调用：(调用方, 偏移) → 实际执行的目标
    pub calls: BTreeMap<(MemberRef, u32), BTreeSet<MemberRef>>,
    /// 实例字段写入（声明字段, 值）
    pub puts: BTreeMap<MemberRef, Vec<Put>>,
    /// 求值中触发初始化的类
    pub inited: BTreeSet<String>,
    /// 按白名单操作执行的手写承载方法
    pub natives: BTreeSet<MemberRef>,
    /// 写入映像（内存缓存字段 / 静态字段）的值：撤销后对程序仍可见，物化进字段值集
    pub memo_vals: Vec<(MemberRef, CV)>,
}

pub(super) struct Vm {
    pub heap: Vec<HObj>,
    pub epoch: u32,
    /// 正在执行 `<clinit>` 的嵌套层数（> 0 时分配进映像、不记轨迹）
    pub image: u32,
    pub statics: HashMap<u32, CV>,
    pub init: HashMap<Rc<str>, Init>,
    /// 静态状态由 VM / 手写层承载的类（`<clinit>` 操作名 `opaque`）
    pub opaque: HashSet<Rc<str>>,
    /// 完成初始化的类（按完成次序）/ 正在执行的 `<clinit>` 开始时的堆大小 / 映像纪元内改写既有映像对象的次数
    pub done_log: Vec<Rc<str>>,
    pub clinit_floor: Vec<usize>,
    pub foreign: u64,
    fkeys: HashMap<String, u32>,
    /// 字段键 → (声明类, 字段名)
    pub fnames: Vec<(Rc<str>, Rc<str>)>,
    /// 不可变的映像数组（字符串内容）：求值纪元内可读
    pub frozen: HashSet<u32>,
    /// 类型 → 是否发布后不再改写（`[concrete] stable_types`）
    pub stable_ty: HashMap<Rc<str>, bool>,
    /// 包 → 包内类名（静态字段写入点扫描，concrete/stable.rs）
    pub pkgs: Option<HashMap<String, Vec<String>>>,
    pub fres: HashMap<MemberRef, Option<Rc<FRes>>>,
    pub minfo: HashMap<MemberRef, Rc<MInfo>>,
    pub mres: HashMap<(MemberRef, bool), Option<MethodSite>>,
    pub sel: HashMap<(Rc<str>, MemberRef), Option<MethodSite>>,
    pub strings: HashMap<Vec<u16>, u32>,
    pub mirrors: HashMap<Rc<str>, u32>,
    pub mirror_of: HashMap<u32, Rc<str>>,
    /// VM 持有的单例对象（操作 `vm_singleton`，按类型）
    pub singletons: HashMap<Rc<str>, u32>,
    ihash: HashMap<u32, i32>,
    pub steps: u64,
    /// 调用栈（调用方类查询）
    pub frames: Vec<MemberRef>,
    pub trace: Trace,
    /// 内存缓存字段写入的撤销日志：(对象 / u32::MAX = 静态, 字段键, 原值)
    pub undo: Vec<(u32, u32, Option<CV>)>,
}

impl Vm {
    pub(super) fn new() -> Self {
        Vm {
            heap: Vec::new(),
            epoch: 0,
            image: 0,
            statics: HashMap::default(),
            init: HashMap::default(),
            opaque: HashSet::default(),
            done_log: Vec::new(),
            clinit_floor: Vec::new(),
            foreign: 0,
            fkeys: HashMap::default(),
            fnames: Vec::new(),
            frozen: HashSet::default(),
            stable_ty: HashMap::default(),
            pkgs: None,
            fres: HashMap::default(),
            minfo: HashMap::default(),
            mres: HashMap::default(),
            sel: HashMap::default(),
            strings: HashMap::default(),
            mirrors: HashMap::default(),
            mirror_of: HashMap::default(),
            singletons: HashMap::default(),
            ihash: HashMap::default(),
            steps: 0,
            frames: Vec::new(),
            trace: Trace::default(),
            undo: Vec::new(),
        }
    }

    pub(super) fn tracing(&self) -> bool {
        self.image == 0 && self.epoch > 0
    }

    fn cur_epoch(&self) -> u32 {
        if self.image > 0 {
            0
        } else {
            self.epoch
        }
    }

    pub(super) fn alloc(&mut self, ty: &str, body: Body) -> u32 {
        let id = self.heap.len() as u32;
        self.heap.push(HObj { ty: Rc::from(ty), epoch: self.cur_epoch(), body });
        id
    }

    pub(super) fn new_array(&mut self, ty: &str, n: i32) -> R<u32> {
        if n < 0 {
            return implicit("size");
        }
        let comp = &ty[1..];
        Ok(self.alloc(ty, Body::Arr(vec![CV::zero(comp); n as usize])))
    }

    pub(super) fn ty(&self, o: u32) -> Rc<str> {
        self.heap[o as usize].ty.clone()
    }

    pub(super) fn arr(&self, o: u32) -> R<&Vec<CV>> {
        match &self.heap[o as usize].body {
            Body::Arr(_) if self.heap[o as usize].epoch == 0 && self.image == 0 && !self.frozen.contains(&o) => {
                fail(format!("读取可变映像数组 {}", self.heap[o as usize].ty))
            }
            Body::Arr(v) => Ok(v),
            _ => fail("期望数组"),
        }
    }

    /// 类初始化中改写其开始前已有的映像对象（初始化失败时不能降级为静态不可读）
    fn note_foreign(&mut self, o: u32) {
        if self.image > 0 && self.clinit_floor.last().is_some_and(|&f| (o as usize) < f) {
            self.foreign += 1;
        }
    }

    pub(super) fn arr_mut(&mut self, o: u32) -> R<&mut Vec<CV>> {
        let ep = self.cur_epoch();
        self.note_foreign(o);
        let h = &mut self.heap[o as usize];
        if h.epoch != ep {
            return fail(format!("写入共享数组 {}", h.ty));
        }
        match &mut h.body {
            Body::Arr(v) => Ok(v),
            _ => fail("期望数组"),
        }
    }

    pub(super) fn fkey(&mut self, decl: &str, name: &str) -> u32 {
        let k = format!("{decl}.{name}");
        if let Some(&n) = self.fkeys.get(&k) {
            return n;
        }
        let n = self.fkeys.len() as u32;
        self.fkeys.insert(k, n);
        self.fnames.push((Rc::from(decl), Rc::from(name)));
        n
    }

    /// 字段解析（JVMS §5.4.3.2，按引用缓存）
    pub(super) fn field_res(&mut self, env: &Env, f: &MemberRef) -> R<Rc<FRes>> {
        if let Some(r) = self.fres.get(f) {
            return r.clone().map_or_else(|| fail(format!("字段解析失败 {f}")), Ok);
        }
        let r = env.h().resolve_field(&f.owner, &f.name, &f.desc).map(|site| {
            let fd = site.field();
            let decl: Rc<str> = Rc::from(site.class.name.as_str());
            let key = self.fkey(&decl, &fd.name);
            let memo = env.cfg().memo_fields.contains(&format!("{decl}.{}", fd.name));
            let fin = fd.access & acc::FINAL != 0 || self.init_only(env, &site.class, fd);
            Rc::new(FRes {
                key,
                decl,
                name: fd.name.clone(),
                desc: fd.desc.clone(),
                fin,
                memo,
                constant: fd.constant_value.clone(),
            })
        });
        self.fres.insert(f.clone(), r.clone());
        r.map_or_else(|| fail(format!("字段解析失败 {f}")), Ok)
    }

    /// 实例字段读：映像对象（`<clinit>` 构造、程序其余部分可见）只许读 final 字段、内存缓存字段，以及
    /// 发布后不再改写的类型（`[concrete] stable_types`）的全部字段——经后者取到的映像数组同样冻结
    pub(super) fn get_field(&mut self, env: &Env, o: u32, fr: &FRes) -> R<CV> {
        let image = self.image == 0 && self.heap[o as usize].epoch == 0;
        let stable = image && self.stable(env, o);
        if image && !fr.fin && !fr.memo && !stable {
            return fail(format!("读取映像对象可变字段 {}.{}", fr.decl, fr.name));
        }
        let v = match &self.heap[o as usize].body {
            Body::Inst(fs) => fs.iter().find(|(k, _)| *k == fr.key).map_or_else(|| CV::zero(&fr.desc), |(_, v)| *v),
            _ => return fail(format!("对非实例对象取字段 {}", fr.name)),
        };
        if let (true, CV::R(a)) = (stable, v) {
            if matches!(self.heap[a as usize].body, Body::Arr(_)) && self.heap[a as usize].epoch == 0 {
                self.frozen.insert(a);
            }
        }
        Ok(v)
    }

    /// 实例字段写入：映像对象只许写内存缓存字段（记撤销）
    pub(super) fn put_field(&mut self, o: u32, fr: &FRes, v: CV) -> R<()> {
        let ep = self.cur_epoch();
        let shared = self.heap[o as usize].epoch != ep;
        if shared && !fr.memo {
            return fail(format!("写入共享对象字段 {}.{}", fr.decl, fr.name));
        }
        if !fr.memo {
            self.note_foreign(o);
        }
        let Body::Inst(fs) = &mut self.heap[o as usize].body else {
            return fail(format!("对非实例对象写字段 {}", fr.name));
        };
        let old = fs.iter().position(|(k, _)| *k == fr.key);
        if shared {
            self.undo.push((o, fr.key, old.map(|i| fs[i].1)));
        }
        match old {
            Some(i) => fs[i].1 = v,
            None => fs.push((fr.key, v)),
        }
        Ok(())
    }

    /// 按 VM 布局字段的语义名写（字符串字面量 / 镜像构造）
    pub(super) fn put_vm_field(&mut self, env: &Env, o: u32, what: &str, v: CV) -> R<()> {
        let Some(spec) = env.cfg().vm_fields.get(what) else { return fail(format!("清单缺 VM 布局字段 {what}")) };
        let (owner, name) = spec.rsplit_once('.').unwrap_or((spec, ""));
        let key = self.fkey(owner, name);
        let Body::Inst(fs) = &mut self.heap[o as usize].body else { return fail("期望实例") };
        fs.push((key, v));
        Ok(())
    }

    pub(super) fn get_vm_field(&mut self, env: &Env, o: u32, what: &str) -> R<CV> {
        let Some(spec) = env.cfg().vm_fields.get(what) else { return fail(format!("清单缺 VM 布局字段 {what}")) };
        let (owner, name) = spec.rsplit_once('.').unwrap_or((spec, ""));
        let key = self.fkey(owner, name);
        match &self.heap[o as usize].body {
            Body::Inst(fs) => Ok(fs.iter().find(|(k, _)| *k == key).map_or(CV::N, |(_, v)| *v)),
            _ => fail("期望实例"),
        }
    }

    /// 静态字段写入：求值纪元内只许写内存缓存字段（记撤销）；`<clinit>` 映像构造期间照常写
    pub(super) fn put_static(&mut self, fr: &FRes, v: CV) -> R<()> {
        if self.image == 0 {
            if !fr.memo {
                return fail(format!("写入静态字段 {}.{}", fr.decl, fr.name));
            }
            let old = self.statics.get(&fr.key).copied();
            self.undo.push((u32::MAX, fr.key, old));
        }
        self.statics.insert(fr.key, v);
        Ok(())
    }

    /// 撤销本次求值对映像的内存缓存写入
    pub(super) fn rollback(&mut self) {
        while let Some((o, k, old)) = self.undo.pop() {
            if o == u32::MAX {
                match old {
                    Some(v) => self.statics.insert(k, v),
                    None => self.statics.remove(&k),
                };
                continue;
            }
            if let Body::Inst(fs) = &mut self.heap[o as usize].body {
                match old {
                    Some(v) => {
                        if let Some(e) = fs.iter_mut().find(|(x, _)| *x == k) {
                            e.1 = v;
                        }
                    }
                    None => fs.retain(|(x, _)| *x != k),
                }
            }
        }
    }

    /// 身份哈希（按首次查询顺序编号，确定）
    pub(super) fn identity_hash(&mut self, o: u32) -> i32 {
        let n = self.ihash.len() as i32;
        *self.ihash.entry(o).or_insert(0x1000 + n * 7919)
    }

    // ── 字符串与类镜像 ──────────────────────────────────────────────────────

    /// 字符串字面量（按内容驻留进映像）：紧凑字符串布局——全部码元 ≤ 0xFF 时 LATIN1（coder 0），
    /// 否则 UTF16（coder 1，小端码元）
    pub(super) fn string(&mut self, env: &Env, units: &[u16]) -> R<u32> {
        if let Some(&o) = self.strings.get(units) {
            return Ok(o);
        }
        let image = self.image;
        self.image += 1;
        let r = self.make_string(env, units);
        self.image = image;
        let o = r?;
        self.strings.insert(units.to_vec(), o);
        Ok(o)
    }

    /// 求值纪元内新建的字符串（不驻留）
    pub(super) fn make_string(&mut self, env: &Env, units: &[u16]) -> R<u32> {
        let latin1 = units.iter().all(|&u| u <= 0xFF);
        let bytes: Vec<CV> = if latin1 {
            units.iter().map(|&u| CV::I(u as u8 as i8 as i32)).collect()
        } else {
            units.iter().flat_map(|&u| [CV::I((u & 0xFF) as u8 as i8 as i32), CV::I((u >> 8) as u8 as i8 as i32)]).collect()
        };
        let arr = self.alloc("[B", Body::Arr(bytes));
        self.frozen.insert(arr);
        let s = self.alloc(STRING, Body::Inst(Vec::new()));
        self.put_vm_field(env, s, "string_value", CV::R(arr))?;
        self.put_vm_field(env, s, "string_coder", CV::I(i32::from(!latin1)))?;
        Ok(s)
    }

    /// 字符串对象的 UTF-16 内容
    pub(super) fn units(&mut self, env: &Env, s: u32) -> R<Vec<u16>> {
        let arr = self.get_vm_field(env, s, "string_value")?.obj()?;
        let coder = self.get_vm_field(env, s, "string_coder")?;
        let bytes: Vec<u8> = self.arr(arr)?.iter().map(|v| v.i().map(|b| b as u8)).collect::<R<_>>()?;
        Ok(if coder == CV::I(0) {
            bytes.iter().map(|&b| b as u16).collect()
        } else {
            bytes.chunks(2).map(|c| c[0] as u16 | (c.get(1).copied().unwrap_or(0) as u16) << 8).collect()
        })
    }

    pub(super) fn rust_string(&mut self, env: &Env, s: u32) -> R<String> {
        let u = self.units(env, s)?;
        String::from_utf16(&u).map_or_else(|_| fail("字符串含孤立代理项"), Ok)
    }

    /// 类镜像（按所指类型驻留进映像；类型取 binary name / 数组描述符 / 基本类型描述符字符）
    pub(super) fn mirror(&mut self, env: &Env, t: &str) -> R<u32> {
        if let Some(&o) = self.mirrors.get(t) {
            return Ok(o);
        }
        let image = self.image;
        self.image += 1;
        let o = self.alloc(CLASS, Body::Inst(Vec::new()));
        self.image = image;
        let t: Rc<str> = Rc::from(t);
        self.mirrors.insert(t.clone(), o);
        self.mirror_of.insert(o, t.clone());
        if let Some(c) = t.strip_prefix('[') {
            let ct = c.strip_prefix('L').and_then(|x| x.strip_suffix(';')).unwrap_or(c);
            let cm = self.mirror(env, ct)?;
            self.put_vm_field(env, o, "component_type", CV::R(cm))?;
        }
        Ok(o)
    }

    // ── 方法解析与选择（按引用缓存）──────────────────────────────────────────

    pub(super) fn resolve(&mut self, env: &Env, m: &MemberRef, iface: bool) -> R<MethodSite> {
        let k = (m.clone(), iface);
        if let Some(r) = self.mres.get(&k) {
            return r.clone().map_or_else(|| fail(format!("方法解析失败 {m}")), Ok);
        }
        let r = env.h().resolve_method(&m.owner, &m.name, &m.desc, iface);
        self.mres.insert(k, r.clone());
        r.map_or_else(|| fail(format!("方法解析失败 {m}")), Ok)
    }

    pub(super) fn select(&mut self, env: &Env, recv: &Rc<str>, site: &MethodSite) -> R<MethodSite> {
        let (o, n, d) = site.key();
        let k = (recv.clone(), MemberRef { owner: o, name: n, desc: d });
        if let Some(r) = self.sel.get(&k) {
            return r.clone().map_or_else(|| fail(format!("虚方法选择失败 {} on {recv}", k.1)), Ok);
        }
        let r = env.h().select(recv, site);
        self.sel.insert(k.clone(), r.clone());
        r.map_or_else(|| fail(format!("虚方法选择失败 {} on {recv}", k.1)), Ok)
    }

    pub(super) fn info(&mut self, env: &Env, site: &MethodSite) -> Rc<MInfo> {
        let (o, n, d) = site.key();
        let key = MemberRef { owner: o, name: n, desc: d };
        if let Some(i) = self.minfo.get(&key) {
            return i.clone();
        }
        let index = site.method().code.as_ref().map(|c| c.insns.iter().enumerate().map(|(i, x)| (x.offset, i)).collect()).unwrap_or_default();
        // 显式操作优先；其次清单的返回值事实（`[facts.returns]` / `[vm_constants] null_returns`：原生二进制里恒定的返回值）
        let ks = key.to_string();
        let op = env.cfg().natives.get(&ks).cloned().or_else(|| {
            env.man().return_fact(&ks).map(|f| match f {
                crate::manifest::Fact::Null => "const:null".to_string(),
                crate::manifest::Fact::Int(x) => format!("const:{x}"),
            })
        });
        let bytecode = site.method().code.is_some() && matches!(env.ctx.kind_of(&site.class, site.method()), Kind::Bytecode);
        let i = Rc::new(MInfo { key: key.clone(), site: site.clone(), index, op, bytecode });
        self.minfo.insert(key, i.clone());
        i
    }

    pub(super) fn class(&self, env: &Env, name: &str) -> R<Arc<ClassFile>> {
        env.h().class(name).map_or_else(|| fail(format!("类缺失 {name}")), Ok)
    }
}

/// 求值环境：类层次、清单与方法承载判定
pub(super) struct Env<'e, 'a> {
    pub ctx: &'e Ctx<'a>,
    pub cp: &'a ClassPath,
}

impl<'e, 'a> Env<'e, 'a> {
    pub(super) fn h(&self) -> &'a Hierarchy<'a> {
        self.ctx.h
    }
    pub(super) fn man(&self) -> &'a Manifest {
        self.ctx.man
    }
    pub(super) fn cfg(&self) -> &'a crate::manifest::ConcreteCfg {
        &self.ctx.man.concrete
    }
}
