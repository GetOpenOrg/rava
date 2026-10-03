//! 常量 / 事实查询：常量格 `PV`、分析上下文 `Ctx`、absint 的 `Oracle` 实现 `Facts`。

use super::*;

// ── 常量 / 事实查询（absint 的 Oracle）──────────────────────────────────────

/// 常量格上的值：缺席（⊥，尚无值）→ 常量（int 族可为小集合，见 `absint::ints`）→ Top
#[derive(Clone, Debug, PartialEq)]
pub(super) enum PV {
    Const(V),
    Top,
}

impl PV {
    /// 抽象值 → 常量格：int / long / null / 字符串常量，及带对象标签的引用（去来源存储；
    /// 标签引用只在分析内部传递，不作为折叠常量导出）
    pub(super) fn of(v: &V) -> PV {
        match v {
            V::Int(_) | V::Ints(_) | V::Long(_) | V::Null | V::Str(_) | V::Offset(_) => PV::Const(v.clone()),
            V::Ref { .. } if v.obj().is_some() => PV::Const(v.stripped()),
            _ => PV::Top,
        }
    }
    /// 返回值 → 返回常量格：在 [`PV::of`] 之上，确定非空、无标签的引用记为「非空引用」（[`nonnull_ref`]）。
    /// 只用于返回常量格：调用点据此判定 `ifnull` / `ifnonnull`（如恒返回新建对象的工厂方法），
    /// 形参 / 字段常量格不取它（那里的引用须保留来源）
    pub(super) fn of_ret(v: &V) -> PV {
        match v {
            V::Ref { nonnull: true, obj: None, .. } => PV::Const(nonnull_ref()),
            _ => PV::of(v),
        }
    }
    pub(super) fn join(a: Option<&PV>, b: &PV) -> PV {
        match (a, b) {
            (None, x) => x.clone(),
            (Some(PV::Const(x)), PV::Const(y)) if x == y => PV::Const(x.clone()),
            // 非空引用与确定非空的引用（含带标签的）合流：仍是非空引用
            (Some(PV::Const(x)), PV::Const(y)) if (is_nonnull_ref(x) || is_nonnull_ref(y)) && x.nonnull() == Some(true) && y.nonnull() == Some(true) => {
                PV::Const(nonnull_ref())
            }
            // int 族常量：取有限并
            (Some(PV::Const(x)), PV::Const(y)) if crate::absint::ints::members(x).is_some() && crate::absint::ints::members(y).is_some() => {
                crate::absint::ints::union(x, y).map_or(PV::Top, PV::Const)
            }
            // 同一对象标签（或 null 与标签对象）：合流保留标签，可空性取并
            (Some(PV::Const(x)), PV::Const(y)) if x.obj().is_some() || y.obj().is_some() => match x.join(y) {
                j @ V::Ref { .. } if j.obj().is_some() => PV::Const(j.stripped()),
                _ => PV::Top,
            },
            _ => PV::Top,
        }
    }
    pub(super) fn value(&self) -> Option<V> {
        match self {
            PV::Const(v) => Some(v.clone()),
            PV::Top => None,
        }
    }
}

/// 返回常量格的「非空引用」：值未知、类型未知（调用点按声明返回类型补上）、恒非 null
pub(super) fn nonnull_ref() -> V {
    V::Ref { ty: None, nonnull: true, src: Rc::from([].as_slice()), obj: None }
}

pub(super) fn is_nonnull_ref(v: &V) -> bool {
    matches!(v, V::Ref { ty: None, nonnull: true, src, obj: None } if src.is_empty())
}

/// 字段初值（默认值）；float / double 不折叠
pub(super) fn default_pv(desc: &str) -> PV {
    match desc.as_bytes().first() {
        Some(b'B' | b'C' | b'I' | b'S' | b'Z') => PV::Const(V::Int(0)),
        Some(b'J') => PV::Const(V::Long(0)),
        Some(b'L' | b'[') => PV::Const(V::Null),
        _ => PV::Top,
    }
}

pub(super) struct Ctx<'a> {
    pub(super) h: &'a Hierarchy<'a>,
    pub(super) cp: &'a ClassPath,
    pub(super) man: &'a Manifest,
    pub(super) hw: &'a Handwritten,
    /// static final 字段常量缓存（None = 非常量）与其 `<clinit>` 求值的输入
    pub(super) consts: RefCell<HashMap<MemberRef, (Option<V>, super::memo::Inputs)>>,
    /// 类的分析域缓存
    pub(super) domains: RefCell<HashMap<String, Domain>>,
    /// 调用点的静态摘要缓存：成员引用 → [(opcode, iface, 摘要)]
    pub(super) calls: RefCell<HashMap<MemberRef, Vec<(u8, bool, Rc<CallInfo>)>>>,
    /// 字段引用的解析缓存（None = 解析失败）
    pub(super) fields: RefCell<HashMap<MemberRef, Option<Rc<FieldInfo>>>>,
    /// 进行中的记忆化计算（递归保护与截断记录，见 `memo.rs`）
    pub(super) guards: RefCell<super::memo::Guards>,
    /// 服务目录与 provider 执行线（见 `services.rs`）
    pub(super) catalog: std::cell::OnceCell<Rc<crate::seeds::services::Catalog>>,
    /// 类的定义加载器表（字段钩子的接收者判定与镜像读取折叠，惰性建立）
    pub(super) loaders: std::cell::OnceCell<crate::loaders::DefiningLoaders>,
    /// 选择子形参缓存（见 `selector.rs`）
    pub(super) selectors: RefCell<HashMap<MemberRef, u64>>,
    /// 非 static final 字段的值集（初值 ∪ 可达写入；缺席 = 只有初值）
    pub(super) fvals: RefCell<HashMap<MemberRef, PV>>,
    /// 字节码方法的返回常量（缺席 = 尚无返回路径）
    pub(super) rvals: RefCell<HashMap<MemberRef, PV>>,
    /// 偏移可得、不折叠的字段：反射 / VarHandle / Unsafe 按名取得的字段
    pub(super) fopen: RefCell<HashSet<MemberRef>>,
    /// 偏移可得、不折叠的字段名（按名取得推不出所属类）
    pub(super) fopen_names: RefCell<HashSet<String>>,
    /// 手写体写入的字段：只不折叠，偏移不因此可得（手写体按 Rust 字段直接写，不经偏移）
    pub(super) fhw: RefCell<HashSet<MemberRef>>,
    /// 手写体写入、推不出所属类的字段名：只不折叠
    pub(super) fhw_names: RefCell<HashSet<String>>,
    /// 反射枚举式写入推不出所属类：全部字段不折叠
    pub(super) fopen_all: Cell<bool>,
    /// 反序列化可达：非 static、非 transient 字段不折叠
    pub(super) deser: Cell<bool>,
    /// 字段 → 读取过它的方法（值集变化时失效重算）
    pub(super) fdeps: RefCell<HashMap<MemberRef, BTreeSet<usize>>>,
    /// 被调方法 → 查询过其返回常量的方法
    pub(super) rdeps: RefCell<HashMap<MemberRef, BTreeSet<usize>>>,
    /// 「尚无返回」答复的阶段与定论判定（见 `noreturn.rs`）
    pub(super) noreturn: RefCell<super::noreturn::NoReturn>,
    /// 当前分析得到过「不返回」答复的方法节点（排空时按值未知重算）
    pub(super) never: RefCell<BTreeSet<usize>>,
    /// 构造器摘要缓存：`构造器|实参` → 构造完成的对象标签
    pub(super) objs: RefCell<HashMap<String, (Option<Rc<Obj>>, super::memo::Inputs)>>,
    /// 字节码方法的属性读取摘要（None = 不是读取形态）
    pub(super) psums: RefCell<HashMap<MemberRef, (Option<PropSum>, super::memo::Inputs)>>,
    /// 只读形参判定缓存：(方法, 形参序号) → 属性表对象经该形参传入时不逃逸
    pub(super) preadonly: RefCell<HashMap<(MemberRef, usize), (bool, super::memo::Inputs)>>,
    /// 删除包装方法里作删除键的形参序号（`sysprops_write.rs`）
    pub(super) pwsums: RefCell<HashMap<MemberRef, Rc<[usize]>>>,
    /// 运行期可能被改写（不折叠）的系统属性键
    pub(super) punstable: RefCell<PropUnstable>,
    /// 折叠过属性读取 / 对象字段读取的方法（不折叠集合增长时失效）
    pub(super) pdeps: RefCell<BTreeSet<usize>>,
    /// 分析进行中登记的依赖日志（摘要共享时向新上下文重放，见 `share.rs`）；None = 未在记录
    pub(super) dep_log: RefCell<Option<Vec<super::share::Dep>>>,
    /// 常量实参求值记忆：(目标, 常量实参, 起始深度) → (结果, 输入)
    pub(super) cevals: RefCell<HashMap<super::consteval::CKey, super::consteval::CEval>>,
    /// 进行中的记忆化计算的输入记录（栈，见 `memo.rs`）
    pub(super) mrecs: RefCell<Vec<super::memo::MemoRec>>,
    /// 记忆条目编号 → 取用过它的方法；下一个编号
    pub(super) mdeps: RefCell<HashMap<u32, BTreeSet<usize>>>,
    pub(super) memo_next: Cell<u32>,
    pub(super) ceval_depth: Cell<u32>,
    /// 性能观测（`summary.perf`）
    pub(super) stats: RefCell<super::stats::Stats>,
}

/// 调用点只依赖类文件与清单的摘要（`Oracle::invoke_result` 用）
pub(super) struct CallInfo {
    /// 清单返回事实
    pub(super) fact: Option<V>,
    pub(super) null_to_false: bool,
    /// 唯一目标且为字节码方法
    pub(super) target: Option<MemberRef>,
    /// 清单 value_equals（调用名或唯一目标名）
    pub(super) value_eq: bool,
    /// 清单字符串纯函数（调用名优先，其次唯一目标名）
    pub(super) str_op: Option<crate::manifest::StrOp>,
    /// 清单属性表持有方法
    pub(super) holder: bool,
    /// 清单属性读取锚点的读取形态
    pub(super) reader: Option<super::sysprops::PropSum>,
    /// 按名取字段偏移的入口（`name_resolvers` 里 offset = true）
    pub(super) offset: Option<crate::manifest::NameResolver>,
}

/// 字段引用解析结果
pub(super) struct FieldInfo {
    /// 声明类上的字段键
    pub(super) key: MemberRef,
    pub(super) access: u16,
    pub(super) constant: Option<Const>,
    /// 写入来源超出字节码（边界类 / 根类 / 手写字段 / VM 状态字段钩子）：不折叠。这些写入都按 Rust 字段
    /// 直接落地，不产出偏移，与偏移可得（[`Ctx::field_offset_under`]）无关
    pub(super) open: bool,
    /// 声明类实现可序列化标记接口（反序列化可写该字段）
    pub(super) serializable: bool,
}

pub(super) struct Facts<'c, 'a> {
    pub(super) ctx: &'c Ctx<'a>,
    pub(super) live: &'c dyn Fn(&str) -> bool,
    /// 被分析的方法（None = 静态常量求值用的 `<clinit>` 分析：只用清单事实与 static final 常量）
    pub(super) m: Option<usize>,
    /// 形参常量（按形参槽序号）
    pub(super) params: Vec<Option<V>>,
    /// Class 形参值集所指的类镜像（按形参序号；None = 非 Class 形参或值集含所指未知的 Class）
    pub(super) mirrors: Vec<Option<BTreeSet<Rc<str>>>>,
}

pub(super) fn const_value(c: &Const) -> Option<V> {
    match c {
        Const::Int(v) => Some(V::Int(*v)),
        Const::Long(v) => Some(V::Long(*v)),
        Const::String(s) => Some(V::Str(Rc::from(s.as_str()))),
        _ => None,
    }
}

impl Ctx<'_> {
    pub(super) fn kind_of(&self, cf: &ClassFile, m: &classfile::Method) -> Kind {
        let member = format!("{}.{}:{}", cf.name, m.name, m.desc);
        match self.domain(&cf.name) {
            // VM 契约边界（`[vm_boundary]`）按方法划分：手写承载（native / VM 内建 / 共置手写体
            // 按精确名提供 / 类初始化器）的取手写效果，其余被调用到的方法运行时执行的就是其字节码
            // （发射层同样翻译），按字节码建模——否则其体内的调用与写入（如经 native 手写体
            // 写入的字段）从分析中消失，成为漏报。类初始化器由清单逐类决定（`translate_clinit`）
            Domain::Boundary => {
                let hw = self.boundary_carried(cf, m, &member);
                return if hw { Kind::Handwritten("boundary") } else { Kind::Bytecode };
            }
            Domain::Root => return Kind::Handwritten("root"),
            _ => {}
        }
        if m.is_native() {
            return Kind::Handwritten("native");
        }
        if self.man.is_intrinsic(&member) {
            return Kind::Handwritten("intrinsic");
        }
        if m.name != "<clinit>" && self.provided(cf, &m.name, &m.desc) {
            return Kind::Handwritten("provides");
        }
        if m.code.is_none() {
            return Kind::Abstract;
        }
        Kind::Bytecode
    }

    /// 边界方法由手写层承载（native / 无体 / 内部边界类与 `clinit_carried` 所列 VM 边界类的 `<clinit>` /
    /// VM 内建 / 共置手写体提供）。其余 VM 边界类的 `<clinit>` 是纯 Java 静态状态，按字节码翻译
    fn boundary_carried(&self, cf: &ClassFile, m: &classfile::Method, member: &str) -> bool {
        m.is_native()
            || m.code.is_none()
            || (m.name == "<clinit>" && (!self.man.is_vm_boundary(&cf.name) || self.man.is_vm_clinit_carried(&cf.name)))
            || self.man.is_intrinsic(member)
            || self.provided(cf, &m.name, &m.desc)
    }

    /// 共置手写体按精确 Rust 名提供该成员（与发射侧 `_nf_covered` 同口径：mangle 名，或类内无重载时的裸名）
    pub(super) fn provided(&self, cf: &ClassFile, name: &str, desc: &str) -> bool {
        let hw = self.hw.class(&cf.name);
        if hw.fns.is_empty() || self.man.hw_dropped(&cf.name) {
            return false;
        }
        let (rust, mangled) = self.rust_names(cf, name, desc);
        hw.fns.iter().any(|(f, i)| {
            let f = f.strip_prefix("__impl_").unwrap_or(f);
            i.is_pub && (f == mangled || rust.as_deref() == Some(f))
        })
    }

    /// (类内无重载时的裸名, mangle 名)
    pub(super) fn rust_names(&self, cf: &ClassFile, name: &str, desc: &str) -> (Option<String>, String) {
        let base = if name == "<init>" { "new" } else { name };
        let suffix = self.hw.descriptor_suffix(desc);
        let mangled = if suffix.is_empty() { base.to_string() } else { format!("{base}_{suffix}") };
        let overloaded = cf.methods.iter().filter(|m| m.name == name).count() > 1;
        (if overloaded { None } else { Some(base.to_string()) }, mangled)
    }

    pub(super) fn domain(&self, cls: &str) -> Domain {
        if let Some(&d) = self.domains.borrow().get(cls) {
            return d;
        }
        let origin = self.cp.origin(cls);
        let d = match self.man.domain(cls, origin == Some(Origin::User)) {
            // 依赖库类（`--lib`）：库自身不属 JDK 边界，一律按字节码翻译
            Domain::Boundary if origin == Some(Origin::Lib) => Domain::Translate,
            d => d,
        };
        self.domains.borrow_mut().insert(cls.to_string(), d);
        d
    }

    /// 字段的写入来源超出字节码（手写 / 边界类 / 反射 / 反序列化）：不折叠
    pub(super) fn field_open(&self, fi: &FieldInfo) -> bool {
        self.field_open_under(fi, self.fopen_all.get(), self.deser.get())
    }

    /// 按给定的全局开关（全部放开 / 反序列化可达）判定字段是否不折叠：偏移可得的字段（[`Self::field_offset_under`]）
    /// 加上手写体写入的字段
    pub(super) fn field_open_under(&self, fi: &FieldInfo, all: bool, deser: bool) -> bool {
        fi.open || self.field_offset_under(fi, all, deser) || self.hw_written(fi)
    }

    /// 可序列化字段：所属类可序列化、非 static、非 transient（默认序列化与反序列化按偏移读写的字段面）
    pub(super) fn serial_field(fi: &FieldInfo) -> bool {
        deser_writes(fi.access, fi.serializable)
    }

    /// 字段被手写体写入（只不折叠，不使偏移可得）
    pub(super) fn hw_written(&self, fi: &FieldInfo) -> bool {
        self.fhw.borrow().contains(&fi.key) || self.fhw_names.borrow().contains(&fi.key.name)
    }

    /// 按给定的全局开关判定字段偏移可经字节码外途径取得（按名取得 / 字段枚举 / 反序列化），从而可按偏移读写。
    /// 手写体写入与 [`FieldInfo::open`] 不算：手写 / VM 落地按 Rust 字段直接写入，不产出偏移
    pub(super) fn field_offset_under(&self, fi: &FieldInfo, all: bool, deser: bool) -> bool {
        all
            || self.fopen.borrow().contains(&fi.key)
            || self.fopen_names.borrow().contains(&fi.key.name)
            || deser && deser_writes(fi.access, fi.serializable)
    }

    pub(super) fn field_info(&self, f: &MemberRef) -> Option<Rc<FieldInfo>> {
        if let Some(fi) = self.fields.borrow().get(f) {
            return fi.clone();
        }
        let fi = self.h.resolve_field(&f.owner, &f.name, &f.desc).map(|site| {
            let fd = site.field();
            let key = MemberRef { owner: site.class.name.clone(), name: fd.name.clone(), desc: fd.desc.clone() };
            // VM 注入的静态字段：运行期值由 VM 写入，字节码初值 / ConstantValue 均不代表运行期值
            let injected = self.man.is_injected_static(&key.owner, &key.name);
            // VM 状态字段（清单字段钩子）由钩子落地写入，同属字节码外的写入来源
            let open = injected
                || matches!(self.domain(&key.owner), Domain::Boundary | Domain::Root)
                || !self.hw.member(&key.owner, &key.name).fns.is_empty()
                || self.man.vm_state.field_hook(&key.owner, &key.name, &key.desc).is_some();
            let constant = if injected { None } else { fd.constant_value.clone() };
            let markers = self.man.serializable_markers();
            let serializable = markers.is_empty() || markers.iter().any(|x| self.h.is_subtype(&key.owner, x));
            Rc::new(FieldInfo { key, access: fd.access, constant, open, serializable })
        });
        self.fields.borrow_mut().insert(f.clone(), fi.clone());
        fi
    }

    /// 字段读的常量值；方法 m 登记为该字段的读者
    pub(super) fn field_value(&self, m: Option<usize>, f: &MemberRef) -> Option<V> {
        let fi = self.field_info(f)?;
        if let Some(m) = m {
            self.dep(m, Dep::Field(fi.key.clone()));
        }
        // VM 注入的字面量值：字节码写入被 VM 值覆盖，读取恒为该值
        if let Some(x) = self.man.injected_literal(&fi.key.owner, &fi.key.name) {
            return Some(if fi.key.desc == "J" { V::Long(x) } else { V::Int(x as i32) });
        }
        if self.field_open(&fi) {
            return None;
        }
        if m.is_none() {
            self.note_aux_read(&fi.key);
        }
        if fi.access & acc::STATIC != 0 && fi.access & acc::FINAL != 0 {
            return self.static_const(m, &fi.key, fi.constant.as_ref());
        }
        m?;
        self.fvals.borrow().get(&fi.key).cloned().unwrap_or_else(|| default_pv(&fi.key.desc)).value()
    }

    /// 调用点的静态摘要（清单事实 + 唯一字节码目标）
    pub(super) fn call_info(&self, opcode: u8, m: &MemberRef, iface: bool) -> Rc<CallInfo> {
        if let Some(c) = self.calls.borrow().get(m).and_then(|v| v.iter().find(|x| x.0 == opcode && x.1 == iface)) {
            return c.2.clone();
        }
        let k = m.to_string();
        let fact = self.man.return_fact(&k).map(|f| match f {
            Fact::Null => V::Null,
            Fact::Int(i) => V::Int(*i),
        });
        let target = self
            .exact_target(opcode, m, iface)
            .filter(|(cf, t)| cf.method(&t.name, &t.desc).is_some_and(|tm| self.kind_of(cf, tm) == Kind::Bytecode))
            .map(|(_, t)| t);
        let tk = target.as_ref().map(|t| t.to_string());
        let value_eq = self.man.is_value_equals(&k) || tk.as_ref().is_some_and(|t| self.man.is_value_equals(t));
        let str_op = self.man.string_op(&k).or_else(|| tk.as_ref().and_then(|t| self.man.string_op(t)));
        let c = Rc::new(CallInfo {
            fact,
            null_to_false: self.man.is_null_to_false(&k),
            target,
            value_eq,
            str_op,
            holder: self.man.sysprops.is_holder(&k),
            reader: self.reader_spec(&k),
            offset: self.man.field_name_resolver(&k).filter(|r| r.offset),
        });
        self.calls.borrow_mut().entry(m.clone()).or_default().push((opcode, iface, c.clone()));
        c
    }

    /// 调用的唯一目标（静态 / 构造 / 私有 / final 方法 / final 类）
    pub(super) fn exact_target(&self, opcode: u8, m: &MemberRef, iface: bool) -> Option<(std::sync::Arc<ClassFile>, MemberRef)> {
        use classfile::op;
        let site = self.h.resolve_method(&m.owner, &m.name, &m.desc, iface)?;
        let rm = site.method();
        let exact = matches!(opcode, op::INVOKESTATIC | op::INVOKESPECIAL)
            || rm.is_private()
            || rm.is_static()
            || rm.is_final()
            || site.class.access & acc::FINAL != 0 && !site.class.is_interface();
        let (o, n, d) = site.key();
        exact.then(|| (site.class.clone(), MemberRef { owner: o, name: n, desc: d }))
    }

    /// 按名取字段偏移的调用折叠为符号偏移：Class 实参是类字面量、名字是字符串常量，且该类自身声明了
    /// 同名实例字段（VM 只查声明类本身，查不到即抛出）
    pub(super) fn field_offset(&self, opcode: u8, r: crate::manifest::NameResolver, args: &[V]) -> Option<V> {
        let base = usize::from(opcode != classfile::op::INVOKESTATIC);
        let cls = match r.class {
            Some(i) => args.get(i + base)?,
            None => args.first()?,
        };
        let (V::Class(c, _), Some(V::Str(name))) = (cls, args.get(r.name + base)) else { return None };
        let cf = self.h.class(c)?;
        let fd = cf.fields.iter().find(|f| f.name == **name && !f.is_static())?;
        Some(V::Offset(Rc::new(MemberRef { owner: cf.name.clone(), name: fd.name.clone(), desc: fd.desc.clone() })))
    }

    /// static final 字段：ConstantValue，或 `<clinit>` 唯一一次常量赋值
    pub(super) fn static_const(&self, me: Option<usize>, key: &MemberRef, cv: Option<&Const>) -> Option<V> {
        if let Some(c) = cv {
            return const_value(c);
        }
        let hit = self.consts.borrow().get(key).cloned();
        if let Some((v, inp)) = hit {
            self.memo_use(me, &inp);
            return v;
        }
        // `<clinit>` 唯一一次常量赋值（递归保护：分析中的类不再展开）。
        // 一次分析得出本类全部 static final 字段的答复，与外层无关时逐字段缓存
        let cls = self.h.class(&key.owner)?;
        let frame = self.memo_enter(format!("clinit:{}", cls.name), true)?;
        let mut puts: HashMap<(&str, &str), Vec<Option<V>>> = HashMap::default();
        let a = cls.method("<clinit>", "()V").and_then(|m| m.code.as_ref()).map(|code| {
            let live = |_: &str| true;
            self.aux_analyze(&cls.name, "()V", true, code, &Facts { ctx: self, live: &live, m: None, params: vec![], mirrors: vec![] })
        });
        for (_, e) in a.iter().flat_map(|a| &a.events) {
            if let Event::Field { opcode: classfile::op::PUTSTATIC, mref, value, .. } = e {
                if mref.owner == cls.name {
                    puts.entry((&mref.name, &mref.desc)).or_default().push(value.clone());
                }
            }
        }
        let (clean, inp) = self.memo_leave(frame);
        self.memo_use(me, &inp);
        let value_of = |name: &str, desc: &str| match puts.get(&(name, desc)).map(Vec::as_slice) {
            Some([Some(v)]) => PV::of(v).value(),
            _ => None,
        };
        if !clean {
            return value_of(&key.name, &key.desc);
        }
        let mut consts = self.consts.borrow_mut();
        for fd in &cls.fields {
            if fd.access & acc::STATIC == 0 || fd.access & acc::FINAL == 0 || fd.constant_value.is_some() {
                continue;
            }
            let k = MemberRef { owner: cls.name.clone(), name: fd.name.clone(), desc: fd.desc.clone() };
            consts.insert(k, (value_of(&fd.name, &fd.desc), inp.clone()));
        }
        consts.get(key).and_then(|e| e.0.clone())
    }
}

impl Oracle for Facts<'_, '_> {
    fn invoke_result(&self, opcode: u8, m: &MemberRef, iface: bool, args: &[V]) -> Ret {
        let c = self.ctx.call_info(opcode, m, iface);
        if let Some(v) = &c.fact {
            return Ret::Value(v.clone());
        }
        if c.null_to_false && args.contains(&V::Null) {
            return Ret::Value(V::Int(0));
        }
        if let Some(v) = c.offset.and_then(|r| self.ctx.field_offset(opcode, r, args)) {
            return Ret::Value(v);
        }
        if let Some(r) = self.ctx.derived_result(self.m, opcode, m, iface, args, &c) {
            return r;
        }
        // 唯一目标且为字节码方法：取其返回常量（被调方法返回常量变化时本方法失效重算）；
        // 返回常量在各调用点上汇合为 Top 时按本调用点的常量实参求值
        let Some(t) = &c.target else { return Ret::Unknown };
        let eval = || self.ctx.const_eval(self.m, t, args).map_or(Ret::Unknown, Ret::Value);
        let Some(me) = self.m else { return eval() };
        let r = self.ctx.rvals.borrow().get(t).cloned();
        self.ctx.dep(me, Dep::Ret(t.clone()));
        match r {
            // 小集合：先按本调用点的常量实参求值，求不出时取集合
            Some(PV::Const(v @ V::Ints(_))) => match eval() {
                Ret::Unknown => Ret::Value(v),
                x => x,
            },
            // 非空引用：先按本调用点的常量实参求值（可能得出字符串常量），求不出时取非空引用
            Some(PV::Const(v)) if is_nonnull_ref(&v) => match eval() {
                Ret::Unknown => Ret::Value(v),
                x => x,
            },
            Some(PV::Const(v)) => Ret::Value(v),
            Some(PV::Top) => eval(),
            None if self.ctx.noreturn.borrow().answer_never(t) => {
                self.ctx.dep(me, Dep::Never);
                Ret::Never
            }
            None => eval(),
        }
    }
    fn final_static(&self, f: &MemberRef) -> bool {
        self.ctx.field_info(f).is_some_and(|fi| fi.access & acc::STATIC != 0 && fi.access & acc::FINAL != 0 && !self.ctx.field_open(&fi))
    }
    fn field(&self, opcode: u8, f: &MemberRef, recv: Option<&V>) -> Option<V> {
        if let Some(v) = self.ctx.mirror_hook_field(opcode, f, recv) {
            return Some(v);
        }
        if let Some(v) = self.ctx.object_field(self.m, opcode, f, recv) {
            return Some(v);
        }
        self.ctx.field_value(self.m, f)
    }
    fn construct(&self, init: &MemberRef, args: &[V]) -> Option<Rc<Obj>> {
        self.ctx.construct(self.m, init, args)
    }
    fn param(&self, i: u16) -> Option<V> {
        self.params.get(i as usize).cloned().flatten()
    }
    fn param_mirror(&self, i: u16, cls: &str) -> Option<bool> {
        self.mirrors.get(i as usize)?.as_ref().map(|s| s.contains(cls))
    }
    fn type_live(&self, ty: &str) -> bool {
        (self.live)(ty)
    }
}

/// 反序列化可写的字段：非 static、非 transient，且声明类可序列化（非可序列化超类的字段由其无参构造器初始化，走字节码）
fn deser_writes(access: u16, serializable: bool) -> bool {
    serializable && access & (acc::STATIC | acc::TRANSIENT) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deser_writes_rule() {
        assert!(deser_writes(acc::PRIVATE, true));
        assert!(!deser_writes(acc::PRIVATE, false));
        assert!(!deser_writes(acc::STATIC, true));
        assert!(!deser_writes(acc::TRANSIENT, true));
    }
}
