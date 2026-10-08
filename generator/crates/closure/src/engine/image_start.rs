//! 引擎：从构建期引导映像出发的抽象分析（计划 2026-10-05-boot-image-evaluator §5.5.2 D6）。
//!
//! - 构建期初始化类（[`ImageData::build_time`]）的初始化不再展开 `<clinit>`：其静态字段的值取映像值；
//! - 映像对象按需成为活对象：静态字段节点首次出现（程序读写该字段）时，其映像值所指对象成为活对象；
//!   活对象的引用字段在该字段节点出现时、数组元素在数组成为活对象时、类镜像在程序取该类镜像时传播；
//!   活对象按类型代表（实例按确切类型、数组按「类型@image」分配点、镜像按镜像 id），值经字段并集
//!   （`U` → `F`）被读者取到——与具体求值结果的物化同一口径（`concrete/apply.rs`）；
//! - 占位对象（运行期结果）不按映像内容传播，取其来源：残差调用 / 重放 native 的返回值、运行期初始化类的静态字段；
//! - 运行期部分作根：残差调用与重放 native 的被调方法（形参 open）、运行期初始化类（照常初始化）、
//!   残差区段（区段内指令按调用点解析目标作根）。
//!
//! 联合不动点终止时的活对象集即物化集合（`live`），发射层只物化活对象，指向非活对象的引用物化为 null
//! （该引用所在字段 / 元素没有读者，不可观察）。

use classfile::{op, Operand};

use super::*;
use crate::image::{IBody, IStep, IVal, ImageData};

pub(super) struct ImgState {
    data: Rc<ImageData>,
    build_time: HashSet<String>,
    /// 构建期初始化类中已登记的（初始化不展开 `<clinit>`）
    touched: HashSet<String>,
    statics: HashMap<(String, String), IVal>,
    mirror_obj: HashMap<String, u32>,
    live: Vec<bool>,
    queue: Vec<u32>,
    /// 活对象的引用字段：字段节点尚未出现，等待（声明类, 字段名）→（值）
    pending: HashMap<(String, String), Vec<IVal>>,
    /// 占位对象 → 来源
    ph_src: HashMap<u32, Feed>,
    /// 映像对象 → 抽象对象（数组与容器形态类的实例逐对象成为分配点，与字节码 `new` 的容器建模一致）
    sites: HashMap<u32, u32>,
    busy: bool,
    /// VM 模块表已汇入类镜像模块钩子的值池
    modules_fed: bool,
}

impl ImgState {
    pub(super) fn is_build_time(&self, cls: &str) -> bool {
        self.build_time.contains(cls)
    }
}

/// 映像对象的宿主值来源 native（宿主相关内容数组）
fn x_host(s: &ImgState, o: u32) -> Option<String> {
    s.data.objs[o as usize].host.as_ref().map(|(n, _)| n.clone())
}

fn is_ref_desc(d: &str) -> bool {
    d.starts_with('L') || d.starts_with('[')
}

impl<'a> Engine<'a> {
    /// 映像作分析起点：登记映像状态并对运行期部分作根（须在其他根之前调用）
    pub fn install_image(&mut self, d: ImageData) {
        let data = Rc::new(d);
        let mut statics: HashMap<(String, String), IVal> = data.statics.iter().map(|(c, n, v)| ((c.clone(), n.clone()), *v)).collect();
        for st in &data.steps {
            if let IStep::Recompute { loc: crate::image::ILoc::Static(c, n), expr } = st {
                statics.insert((c.clone(), n.clone()), IVal::T(*expr, b'I'));
            }
        }
        let mut img_modules: HashMap<String, Vec<(u32, bool)>> = HashMap::default();
        for m in &data.modules {
            for p in &m.packages {
                img_modules.entry(p.clone()).or_default().push((m.obj, m.loader == IVal::N));
            }
        }
        let _ = self.ctx.img_modules.set(img_modules);
        let mirror_obj = data.objs.iter().enumerate().filter_map(|(i, o)| o.mirror.clone().map(|m| (m, i as u32))).collect();
        self.img = Some(Box::new(ImgState {
            build_time: data.build_time.iter().cloned().collect(),
            touched: HashSet::default(),
            statics,
            mirror_obj,
            live: vec![false; data.objs.len()],
            queue: Vec::new(),
            pending: HashMap::default(),
            ph_src: HashMap::default(),
            sites: HashMap::default(),
            busy: false,
            modules_fed: false,
            data: data.clone(),
        }));
        let st = self.img.as_ref().expect("映像").statics.clone();
        self.image_statics_install(&st, &data.build_time);
        // 残差调用 / 区段在其构建期档位的上下文中分析（`levels_boot.rs`）；重放 native 与运行期初始化类在本体
        let mut lc = NOCTX;
        for st in &data.steps {
            match st {
                IStep::Level { level, .. } => lc = if self.level_needed(*level) { self.level_ctx(*level) } else { NOCTX },
                IStep::Recompute { expr, .. } => self.image_expr_roots(&data, *expr),
                IStep::RuntimeInit { class } => self.init(class, Via::root("boot_image", class)),
                // 重定位值是基本类型（偏移 / 地址），不引入方法与对象
                IStep::Reloc { .. } => {}
                IStep::Call { callee, ph, .. } | IStep::Native { callee, ph, .. } => {
                    let Some(key) = seeds::parse_member(callee) else {
                        self.unresolved.insert(callee.clone());
                        continue;
                    };
                    let (ctx, args) = match st {
                        IStep::Call { args, .. } => (lc, Some(args.as_slice())),
                        _ => (NOCTX, None),
                    };
                    let m = self.root_in(key, "boot_image", ctx, args);
                    // 重放 native 的形参 open；其实参是启动序列传入的映像对象，须物化（成为活对象）
                    if let IStep::Native { args, .. } = st {
                        self.image_roots(args);
                    }
                    if let Some(p) = ph {
                        self.img.as_mut().expect("映像").ph_src.insert(*p, Feed::N(Node::R(m)));
                    }
                }
                IStep::Read { decl, name, ph } => {
                    self.init(decl, Via::root("boot_image", decl));
                    let desc = self.h.class(decl).and_then(|c| c.fields.iter().find(|f| f.name == *name).map(|f| f.desc.clone()));
                    if let Some(desc) = desc {
                        let fi = self.field_node(MemberRef { owner: decl.clone(), name: name.clone(), desc });
                        self.img.as_mut().expect("映像").ph_src.insert(*ph, Feed::N(Node::F(fi)));
                    }
                }
                IStep::Region { phase, start, end, locals } => {
                    // 区段入口的局部变量由启动序列传入合成方法：所指映像对象须物化
                    self.image_roots(locals);
                    self.image_region(phase, *start, *end, lc);
                }
            }
        }
        // VM 初始线程：启动序列把它绑定为 OS 主线程的当前线程（运行期 `Thread.currentThread()` 的结果）
        if let Some(t) = data.current_thread {
            self.image_roots(&[IVal::R(t)]);
        }
        // 已出现的字段节点 / 镜像补传播
        let keys: Vec<MemberRef> = self.fields.keys().cloned().collect();
        for (fi, k) in keys.into_iter().enumerate() {
            self.image_field(&k, fi);
        }
        let ms: Vec<u32> = self.mirrors.values().copied().collect();
        for c in ms {
            let n = self.names[c as usize].to_string();
            self.image_mirror(&n);
        }
        self.image_drain();
    }

    /// 重算槽表达式的宿主源方法作根
    fn image_expr_roots(&mut self, d: &ImageData, e: u32) {
        let mut st = vec![e];
        while let Some(e) = st.pop() {
            let vs: Vec<IVal> = match &d.exprs[e as usize] {
                crate::image::IExpr::Src { native, args } => {
                    if let Some(k) = seeds::parse_member(native) {
                        self.root(k, "boot_image");
                    }
                    args.clone()
                }
                crate::image::IExpr::Un(_, x) => vec![*x],
                crate::image::IExpr::Bin(_, a, b) => vec![*a, *b],
                crate::image::IExpr::Sel { a, b, t, f, .. } => vec![*a, *b, *t, *f],
            };
            for v in vs {
                match v {
                    IVal::T(x, _) => st.push(x),
                    IVal::R(o) => {
                        self.image_ref(o);
                    }
                    _ => {}
                }
            }
        }
    }

    /// 启动序列直接传递的映像对象（重放 native 实参、区段局部变量）成为活对象
    fn image_roots(&mut self, vs: &[IVal]) {
        for v in vs {
            if let IVal::R(o) = *v {
                self.image_ref(o);
            }
        }
        self.image_drain();
    }

    /// 构建期初始化类：初始化已在映像中完成（不展开 `<clinit>`）。返回 true 表示已处理
    pub(super) fn image_init(&mut self, cls: &str, via: &Via) -> bool {
        let Some(s) = self.img.as_mut() else { return false };
        if !s.build_time.contains(cls) {
            return false;
        }
        if s.touched.insert(cls.to_string()) {
            self.touch(cls, Level::Init, via.clone());
        }
        true
    }

    /// 字段节点首次出现：静态字段取映像值，活对象的该字段值传播
    pub(super) fn image_field(&mut self, key: &MemberRef, fi: usize) {
        let Some(s) = self.img.as_mut() else { return };
        let k = (key.owner.clone(), key.name.clone());
        let mut vals = s.pending.remove(&k).unwrap_or_default();
        if s.build_time.contains(&key.owner) {
            if let Some(v) = s.statics.get(&k) {
                vals.push(*v);
            }
        }
        for v in vals {
            self.image_put(key, Some(fi), v);
        }
        self.image_drain();
    }

    /// 构建期值（映像 / 具体求值物化）里按所指类型命名的类镜像：名字为 binary name、数组描述符或基本类型描述符
    /// 字符（含 V）。基本类型对应分析侧九类合一的基本类型类镜像——不能按类名取镜像，否则 `B` 被当成引用类，
    /// `Array.newInstance` 给出 `[LB;`，数组维数逐轮增长不收敛
    pub(super) fn named_mirror(&mut self, c: &str, via: &Via) -> u32 {
        self.instantiate(CLASS, via.clone());
        if c.len() == 1 {
            return self.primitive_mirror();
        }
        if c.starts_with('[') {
            self.touch_desc(c, via);
        } else {
            self.touch(c, Level::Type, via.clone());
        }
        self.mirror(c)
    }

    /// 程序取类 cls 的镜像：映像中的该镜像成为活对象
    pub(super) fn image_mirror(&mut self, cls: &str) {
        let Some(&o) = self.img.as_ref().and_then(|s| s.mirror_obj.get(cls)) else { return };
        self.image_ref(o);
        self.image_drain();
    }

    /// 映像值的常量格：int / long / null / 字符串（映像字符串对象的内容）
    pub(super) fn image_pv(&self, v: IVal) -> PV {
        match v {
            IVal::I(x) => PV::Const(V::Int(x)),
            IVal::J(x) => PV::Const(V::Long(x)),
            IVal::N => PV::Const(V::Null),
            IVal::R(o) => self.image_obj_pv(o, true),
            _ => PV::Top,
        }
    }

    /// 映像对象引用的常量格：字符串 → 字符串常量；其他构建期对象 → 带标签的非空引用，标签为其 final 实例字段中的
    /// 标量 / 字符串常量（与构造器摘要同一口径，`construct.rs`）；占位对象（运行期结果）→ Top。
    /// 延迟值对象（内容在启动序列按宿主值写入）只知非空
    fn image_obj_pv(&self, o: u32, deep: bool) -> PV {
        let Some(s) = self.img.as_ref() else { return PV::Top };
        let x = &s.data.objs[o as usize];
        if x.placeholder {
            return PV::Top;
        }
        if let Some(t) = self.image_string(o) {
            return PV::Const(V::Str(Rc::from(t), Default::default()));
        }
        let mut finals: Vec<(MemberRef, V)> = Vec::new();
        // 内容延迟的字符串：启动序列按宿主值改写其内容字段（value / coder / hash），不取字段标签
        if let (IBody::Inst(fs), true, None, None, false) = (&x.body, deep, &x.deferred, &x.mirror, x.ty == STRING) {
            for (decl, name, v) in fs {
                let Some(f) = self.h.class(decl).and_then(|c| c.fields.iter().find(|f| f.name == *name && f.access & classfile::acc::FINAL != 0 && !f.is_static()).map(|f| f.desc.clone())) else { continue };
                let pv = match *v {
                    IVal::R(r) => self.image_obj_pv(r, false),
                    v => self.image_pv(v),
                };
                if let PV::Const(c @ (V::Int(_) | V::Long(_) | V::Null | V::Str(..))) = pv {
                    finals.push((MemberRef { owner: decl.clone(), name: name.clone(), desc: f }, c));
                }
            }
            finals.sort_by(|a, b| (&a.0.owner, &a.0.name).cmp(&(&b.0.owner, &b.0.name)));
        }
        PV::Const(V::Ref { ty: Some(Rc::from(x.ty.as_str())), nonnull: true, src: Default::default(), obj: Some(Rc::new(Obj::Fields(finals))) })
    }

    /// 映像字符串对象的内容（`value` 数组 + `coder`；LATIN1 / UTF16 小端）；对象或内容数组为延迟值 / 占位 → None
    fn image_string(&self, o: u32) -> Option<String> {
        let d = &self.img.as_ref()?.data;
        let x = &d.objs[o as usize];
        if x.ty != STRING || x.placeholder || x.deferred.is_some() {
            return None;
        }
        let IBody::Inst(fs) = &x.body else { return None };
        let field = |n: &str| fs.iter().find(|(c, f, _)| c == STRING && f == n).map(|e| e.2);
        let coder = match field("coder") {
            None => 0,
            Some(IVal::I(c)) => c,
            Some(_) => return None,
        };
        let IVal::R(a) = field("value")? else { return None };
        // 宿主相关的属性值：内容数组登记为延迟值（启动序列按宿主值写入），内容不是常量
        let arr = &d.objs[a as usize];
        if arr.deferred.is_some() || arr.placeholder {
            return None;
        }
        let IBody::Arr(es) = &arr.body else { return None };
        let bytes: Vec<u8> = es.iter().map(|e| if let IVal::I(b) = e { Some(*b as u8) } else { None }).collect::<Option<_>>()?;
        match coder {
            0 => Some(bytes.iter().map(|&b| char::from(b)).collect()),
            1 => String::from_utf16(&bytes.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect::<Vec<_>>()).ok(),
            _ => None,
        }
    }

    /// 残差调用的形参取构建期记录的实参（实参与被调方法的形参逐个对应，含接收者）：常量入常量格，引用经映像对象传播
    pub(super) fn image_args(&mut self, m: usize, args: &[IVal]) {
        let n = self.methods[m].ptypes.len();
        let pvs: Vec<PV> = args.iter().map(|&v| self.image_pv(v)).collect();
        self.bind_pvs(m, 0, n, Some(&pvs));
        let pts = self.methods[m].ptypes.clone();
        for (i, pt) in pts.iter().enumerate() {
            let Some(pt) = *pt else { continue };
            match args[i] {
                IVal::R(o) => {
                    if let Some(f) = self.image_ref(o) {
                        self.feed(&[f], Node::P(m, i as u16), pt);
                    }
                }
                IVal::N => {}
                _ => self.add_to(Node::P(m, i as u16), &TypeSet::open(pt)),
            }
        }
        self.image_drain();
    }

    fn image_put(&mut self, key: &MemberRef, fi: Option<usize>, v: IVal) {
        let pv = self.image_pv(v);
        self.field_put(key, pv);
        if let (IVal::R(o), Some(fi)) = (v, fi) {
            if let Some(f) = self.image_ref(o) {
                let obj = self.id(OBJECT);
                self.feed(&[f], Node::U(fi), obj);
            }
        }
    }

    /// 映像对象的抽象值（首次引用时成为活对象，内容入队）
    fn image_ref(&mut self, o: u32) -> Option<Feed> {
        let s = self.img.as_ref()?;
        let x = &s.data.objs[o as usize];
        let newly = !s.live[o as usize];
        if x.placeholder {
            let f = s.ph_src.get(&o).cloned();
            self.img.as_mut()?.live[o as usize] = true;
            return f;
        }
        let ty = x.ty.clone();
        let mirror = x.mirror.clone();
        let via = Via::root("boot_image", &ty);
        let set = if let Some(c) = mirror {
            TypeSet::exact(self.named_mirror(&c, &via))
        } else if ty.starts_with('[') {
            TypeSet::exact(self.image_array_site(o, &ty, &via))
        } else {
            self.instantiate(&ty, via);
            TypeSet::exact(if self.container(&ty) { self.image_obj_site(o, &ty) } else { self.id(&ty) })
        };
        if newly {
            // 宿主相关内容数组：启动序列调用其来源 native 取宿主值改写（U1），该 native 作根
            let host = x_host(self.img.as_ref()?, o);
            let s = self.img.as_mut()?;
            s.live[o as usize] = true;
            s.queue.push(o);
            if let Some(k) = host.as_deref().and_then(seeds::parse_member) {
                self.root(k, "boot_image");
            }
        }
        Some(Feed::S(set))
    }

    /// 映像数组对象 o 的分配点（逐对象：不同映像数组的元素互不混合）
    fn image_array_site(&mut self, o: u32, t: &str, via: &Via) -> u32 {
        if let Some(&id) = self.img.as_ref().and_then(|s| s.sites.get(&o)) {
            return id;
        }
        self.touch_desc(t, via);
        let tid = self.id(t);
        let id = self.id(&format!("{t}@image{o}"));
        self.arrays.insert(id, tid);
        if self.g.insert(id) {
            self.on_g_grow(id);
        }
        self.img.as_mut().expect("映像").sites.insert(o, id);
        id
    }

    /// 映像中容器形态类的实例 o 的抽象对象（堆上下文链为该映像对象本身）
    fn image_obj_site(&mut self, o: u32, t: &str) -> u32 {
        if let Some(&id) = self.img.as_ref().and_then(|s| s.sites.get(&o)) {
            return id;
        }
        let chain = format!("@image{o}");
        let tid = self.id(t);
        let id = self.id(&format!("{t}{chain}"));
        self.objs.insert(id, tid);
        self.obj_chain.insert(id, Rc::from(chain));
        self.img.as_mut().expect("映像").sites.insert(o, id);
        id
    }

    /// 活对象内容传播（工作表，避免深对象图递归）
    fn image_drain(&mut self) {
        let Some(s) = self.img.as_mut() else { return };
        if s.busy {
            return;
        }
        s.busy = true;
        let obj = self.id(OBJECT);
        while let Some(o) = self.img.as_mut().and_then(|s| s.queue.pop()) {
            let data = self.img.as_ref().expect("映像").data.clone();
            let x = &data.objs[o as usize];
            match &x.body {
                IBody::Arr(es) => {
                    let id = self.image_array_site(o, &x.ty, &Via::root("boot_image", &x.ty));
                    for (j, v) in es.iter().enumerate() {
                        if let IVal::R(r) = v {
                            if let Some(f) = self.image_ref(*r) {
                                self.feed(&[f], Node::E(id, (j % 2) as u8), obj);
                            }
                        }
                    }
                }
                IBody::Inst(fs) => {
                    let site = self.img.as_ref().and_then(|s| s.sites.get(&o).copied());
                    for (d, n, v) in fs {
                        let Some(desc) = self.h.class(d).and_then(|c| c.fields.iter().find(|f| f.name == *n && !f.is_static()).map(|f| f.desc.clone())) else { continue };
                        let key = MemberRef { owner: d.clone(), name: n.clone(), desc };
                        // 抽象对象（容器分配点）的字段值另记入按对象值表（`obj_fields.rs`）：映像对象不经字节码 `new` / `putfield`，
                        // 不记则按对象读只得初值（size 0 / table null），其迭代、查找被折成不可达
                        if let Some(xo) = site {
                            let pv = self.image_pv(*v);
                            self.obj_field_put(&key, &[xo], false, &pv);
                        }
                        if let (Some(xo), IVal::R(r)) = (site, v) {
                            // 抽象对象的字段：值只进该对象的字段节点（逃逸后才与未知接收者视图相连）
                            self.field_put(&key, PV::Top);
                            let fi = self.field_node(key);
                            if let (Some(tid), Some(f)) = (self.fields.get_index(fi).and_then(|e| *e.1), self.image_ref(*r)) {
                                let node = self.obj_field(xo, fi, tid);
                                self.feed(&[f], node, obj);
                            }
                        } else if matches!(v, IVal::R(_)) && is_ref_desc(&key.desc) {
                            match self.fields.get_index_of(&key) {
                                Some(fi) => self.image_put(&key, Some(fi), *v),
                                None => {
                                    self.field_put(&key, PV::Top);
                                    self.img.as_mut().expect("映像").pending.entry((d.clone(), n.clone())).or_default().push(*v);
                                }
                            }
                        } else {
                            self.image_put(&key, None, *v);
                        }
                    }
                }
            }
        }
        self.img.as_mut().expect("映像").busy = false;
    }

    /// 残差区段：区段内指令的依赖登记，调用点按解析目标作根（形参 open）
    fn image_region(&mut self, phase: &str, start: u32, end: Option<u32>, lc: u32) {
        let Some(key) = seeds::parse_member(phase) else {
            self.unresolved.insert(phase.to_string());
            return;
        };
        let Some(cf) = self.h.class(&key.owner) else { return };
        let Some(code) = cf.method(&key.name, &key.desc).and_then(|x| x.code.as_ref()) else { return };
        let end = end.unwrap_or(u32::MAX);
        let via = Via::root("boot_region", phase);
        for x in code.insns.iter().filter(|x| x.offset >= start && x.offset < end) {
            match (&x.operand, x.opcode) {
                (Operand::Class(c), op::NEW) => {
                    self.instantiate(c, via.clone());
                    self.init(c, via.clone());
                    if lc != NOCTX {
                        self.level_init(c, lc);
                    }
                }
                (Operand::Class(c), op::ANEWARRAY) => self.touch_desc(&if c.starts_with('[') { format!("[{c}") } else { format!("[L{c};") }, &via),
                (Operand::Class(c), _) => {
                    if c.starts_with('[') {
                        self.touch_desc(c, &via);
                    } else {
                        self.touch(c, Level::Type, via.clone());
                    }
                }
                (Operand::MultiANewArray(c, _), _) => self.touch_desc(c, &via),
                (Operand::Ldc(Const::String(_)), _) => self.instantiate(STRING, via.clone()),
                (Operand::Ldc(Const::Class(c)), _) => {
                    let c = c.clone();
                    if c.starts_with('[') {
                        self.touch_desc(&c, &via);
                    } else {
                        self.touch(&c, Level::Type, via.clone());
                    }
                    self.instantiate(CLASS, via.clone());
                    self.mirror(&c);
                }
                (Operand::Field(f), opc) => {
                    let Some(site) = self.h.resolve_field(&f.owner, &f.name, &f.desc) else { continue };
                    let decl = site.class.name.clone();
                    self.touch(&f.owner, Level::Layout, via.clone());
                    self.touch_desc(&f.desc, &via);
                    if opc == op::GETSTATIC || opc == op::PUTSTATIC {
                        self.init(&decl, via.clone());
                        if lc != NOCTX {
                            self.level_init(&decl, lc);
                        }
                    }
                    let k = MemberRef { owner: decl.clone(), name: f.name.clone(), desc: f.desc.clone() };
                    let fi = self.field_node(k.clone());
                    if opc == op::PUTSTATIC || opc == op::PUTFIELD {
                        self.field_put(&k, PV::Top);
                        if let Some(t) = parse_field(&f.desc).and_then(|t| self.ptype(&t)) {
                            self.add_to(Node::U(fi), &TypeSet::open(t));
                        }
                    }
                    self.field_handwritten(&decl, &f.name, &f.desc, &via, None);
                }
                (Operand::Method(mref, iface), opc) => {
                    let lvl = if opc == op::INVOKESTATIC || opc == op::INVOKESPECIAL { Level::Layout } else { Level::Type };
                    self.touch(&mref.owner, lvl, via.clone());
                    self.note_ref(mref);
                    let Some(site) = self.h.resolve_method(&mref.owner, &mref.name, &mref.desc, *iface) else { continue };
                    let (owner, name, desc) = site.key();
                    self.root_in(MemberRef { owner, name, desc }, "boot_region", lc, None);
                }
                _ => {}
            }
        }
    }

    /// 类镜像模块字段（清单 `[concrete.vm_fields] class_module`）的钩子值池：映像 VM 模块表的模块（§5.5.1 S5）。
    /// 运行期钩子按「定义加载器 + 包」查启动序列登记的 VM 模块表，未登记的归其加载器的无名模块（钩子手写体）；
    /// 表中模块与其定义加载器在钩子首次接入时成为活对象（启动序列以之登记 VM 模块表）
    pub(super) fn image_module_table(&mut self, decl: &str, name: &str, hook: usize, tid: u32) {
        let Some(s) = self.img.as_ref() else { return };
        if s.modules_fed || s.data.modules.is_empty() {
            return;
        }
        let is_module_field = self.man.concrete.vm_fields.get("class_module").and_then(|k| k.rsplit_once('.')).is_some_and(|(o, n)| o == decl && n == name);
        if !is_module_field {
            return;
        }
        let data = s.data.clone();
        self.img.as_mut().expect("映像").modules_fed = true;
        for m in &data.modules {
            if let Some(f) = self.image_ref(m.obj) {
                self.feed(&[f], Node::S(hook, POOL), tid);
            }
            if let IVal::R(l) = m.loader {
                self.image_ref(l);
            }
        }
        self.image_drain();
    }

    /// 活对象（映像对象下标，升序）
    pub fn image_live(&self) -> Vec<u32> {
        self.img.as_ref().map_or_else(Vec::new, |s| s.live.iter().enumerate().filter(|(_, &l)| l).map(|(i, _)| i as u32).collect())
    }
}
