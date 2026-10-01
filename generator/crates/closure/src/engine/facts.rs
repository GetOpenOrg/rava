//! 常量 / 事实查询：常量格 `PV`、分析上下文 `Ctx`、absint 的 `Oracle` 实现 `Facts`。

use super::*;

// ── 常量 / 事实查询（absint 的 Oracle）──────────────────────────────────────

/// 常量格上的值：缺席（⊥，尚无值）→ 单一常量 → Top
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
            V::Int(_) | V::Long(_) | V::Null | V::Str(_) => PV::Const(v.clone()),
            V::Ref { .. } if v.obj().is_some() => PV::Const(v.stripped()),
            _ => PV::Top,
        }
    }
    pub(super) fn join(a: Option<&PV>, b: &PV) -> PV {
        match (a, b) {
            (None, x) => x.clone(),
            (Some(PV::Const(x)), PV::Const(y)) if x == y => PV::Const(x.clone()),
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
    /// static final 字段常量缓存（None = 非常量）
    pub(super) consts: RefCell<HashMap<MemberRef, Option<V>>>,
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
    pub(super) svc_lines: std::cell::OnceCell<BTreeSet<String>>,
    /// 选择子形参缓存（见 `selector.rs`）
    pub(super) selectors: RefCell<HashMap<MemberRef, u64>>,
    /// 非 static final 字段的值集（初值 ∪ 可达写入；缺席 = 只有初值）
    pub(super) fvals: RefCell<HashMap<MemberRef, PV>>,
    /// 字节码方法的返回常量（缺席 = 尚无返回路径）
    pub(super) rvals: RefCell<HashMap<MemberRef, PV>>,
    /// 不折叠的字段：手写写入、反射按名写入
    pub(super) fopen: RefCell<HashSet<MemberRef>>,
    /// 不折叠的字段名（手写写入 / 反射写入推不出所属类）
    pub(super) fopen_names: RefCell<HashSet<String>>,
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
    pub(super) objs: RefCell<HashMap<String, Option<Rc<Obj>>>>,
    /// 字节码方法的属性读取摘要（None = 不是读取形态）
    pub(super) psums: RefCell<HashMap<MemberRef, Option<PropSum>>>,
    /// 运行期可能被改写（不折叠）的系统属性键
    pub(super) punstable: RefCell<PropUnstable>,
    /// 折叠过属性读取 / 对象字段读取的方法（不折叠集合增长时失效）
    pub(super) pdeps: RefCell<BTreeSet<usize>>,
    /// 分析进行中登记的依赖日志（摘要共享时向新上下文重放，见 `share.rs`）；None = 未在记录
    pub(super) dep_log: RefCell<Option<Vec<super::share::Dep>>>,
    /// 常量实参求值记忆：`目标|常量实参` → (结果, 读过的字段)
    pub(super) cevals: RefCell<HashMap<super::consteval::CKey, super::consteval::CEval>>,
    /// 进行中的常量实参求值的字段读集（栈）
    pub(super) ceval_reads: RefCell<Vec<Vec<MemberRef>>>,
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
}

/// 字段引用解析结果
pub(super) struct FieldInfo {
    /// 声明类上的字段键
    pub(super) key: MemberRef,
    pub(super) access: u16,
    pub(super) constant: Option<Const>,
    /// 写入来源超出字节码（边界类 / 手写字段）
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

/// fn 名是成员的伴生核心 `core_<Rust 名>`（mangle 名，或类内无重载时的裸名）
fn is_core_of(f: &str, rust: Option<&str>, mangled: &str) -> bool {
    f.strip_prefix("core_").is_some_and(|n| n == mangled || rust == Some(n))
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
            // 内部包边界：BFS 截断，整体手写（未手写的成员是 panic 存根，运行时不执行字节码）。
            // VM 耦合边界（公开包，`[vm_boundary]`）按方法划分：手写承载（native / VM 内建 /
            // 共置手写体按精确名提供）的取手写效果，其余被调用到的方法运行时执行的就是其字节码
            // （发射层同样翻译），按字节码建模——否则其体内的调用与写入（如经 native 手写体
            // 写入的字段）从分析中消失，成为漏报
            Domain::Boundary => {
                let hw = self.boundary_carried(cf, m, &member) || !self.man.is_vm_boundary(&cf.name);
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

    /// 边界方法由手写层承载（native / 无体 / `<clinit>` / VM 内建 / 共置手写体提供）
    fn boundary_carried(&self, cf: &ClassFile, m: &classfile::Method, member: &str) -> bool {
        m.is_native()
            || m.code.is_none()
            || m.name == "<clinit>"
            || self.man.is_intrinsic(member)
            || self.provided(cf, &m.name, &m.desc)
    }

    /// 边界截断方法：内部边界类上无手写承载的方法——发射层翻译其字节码，分析器不展开其体
    /// （体内的被调方只在另有路径时入闭包）。动态对照按此把运行期执行到的截断体当翻译体归因
    pub(super) fn boundary_cut(&self, cf: &ClassFile, m: &classfile::Method) -> bool {
        self.domain(&cf.name) == Domain::Boundary
            && !self.man.is_vm_boundary(&cf.name)
            && !self.boundary_carried(cf, m, &format!("{}.{}:{}", cf.name, m.name, m.desc))
            && !self.core_provided(cf, m)
    }

    /// 实例方法由伴生核心 `core_<Rust 名>` 承载（发射侧 `Verdict::Core` 同口径：适配转发到手写核心）
    fn core_provided(&self, cf: &ClassFile, m: &classfile::Method) -> bool {
        if m.is_static() || self.man.hw_dropped(&cf.name) {
            return false;
        }
        let (rust, mangled) = self.rust_names(cf, &m.name, &m.desc);
        self.hw.class(&cf.name).fns.keys().any(|f| is_core_of(f, rust.as_deref(), &mangled))
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
            // 纯数据资源束：数据不是实现细节，即便位于边界前缀内也按字节码翻译
            Domain::Boundary
                if !self.man.is_vm_boundary(cls) && self.cp.get(cls).is_some_and(|cf| self.man.seeds.carriers.is_pure_data_bundle(self.cp, &cf)) =>
            {
                Domain::Translate
            }
            // 服务 provider 执行线：运行期经 ServiceLoader 按普通构造实例化，执行的是字节码
            Domain::Boundary if !self.man.is_vm_boundary(cls) && self.on_provider_line(cls) => Domain::Translate,
            d => d,
        };
        self.domains.borrow_mut().insert(cls.to_string(), d);
        d
    }

    /// 字段的写入来源超出字节码（手写 / 边界类 / 反射 / 反序列化）：不折叠
    pub(super) fn field_open(&self, fi: &FieldInfo) -> bool {
        fi.open
            || self.fopen_all.get()
            || self.fopen.borrow().contains(&fi.key)
            || self.fopen_names.borrow().contains(&fi.key.name)
            || self.deser.get() && deser_writes(fi.access, fi.serializable)
    }

    pub(super) fn field_info(&self, f: &MemberRef) -> Option<Rc<FieldInfo>> {
        if let Some(fi) = self.fields.borrow().get(f) {
            return fi.clone();
        }
        let fi = self.h.resolve_field(&f.owner, &f.name, &f.desc).map(|site| {
            let fd = site.field();
            let key = MemberRef { owner: site.class.name.clone(), name: fd.name.clone(), desc: fd.desc.clone() };
            let open = matches!(self.domain(&key.owner), Domain::Boundary | Domain::Root) || !self.hw.member(&key.owner, &key.name).fns.is_empty();
            let markers = self.man.serializable_markers();
            let serializable = markers.is_empty() || markers.iter().any(|x| self.h.is_subtype(&key.owner, x));
            Rc::new(FieldInfo { key, access: fd.access, constant: fd.constant_value.clone(), open, serializable })
        });
        self.fields.borrow_mut().insert(f.clone(), fi.clone());
        fi
    }

    /// 字段读的常量值；方法 m 登记为该字段的读者
    pub(super) fn field_value(&self, m: Option<usize>, f: &MemberRef) -> Option<V> {
        let fi = self.field_info(f)?;
        match m {
            Some(m) => {
                self.dep(m, Dep::Field(fi.key.clone()));
            }
            None => self.note_aux_read(&fi.key),
        }
        if self.field_open(&fi) {
            return None;
        }
        if fi.access & acc::STATIC != 0 && fi.access & acc::FINAL != 0 {
            return self.static_const(&fi.key, fi.constant.as_ref());
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

    /// static final 字段：ConstantValue，或 `<clinit>` 唯一一次常量赋值
    pub(super) fn static_const(&self, key: &MemberRef, cv: Option<&Const>) -> Option<V> {
        if let Some(c) = cv {
            return const_value(c);
        }
        if let Some(v) = self.consts.borrow().get(key) {
            return v.clone();
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
        let clean = self.memo_leave(frame);
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
            consts.insert(MemberRef { owner: cls.name.clone(), name: fd.name.clone(), desc: fd.desc.clone() }, value_of(&fd.name, &fd.desc));
        }
        consts.get(key).cloned().flatten()
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
            Some(PV::Const(v)) => Ret::Value(v),
            Some(PV::Top) => eval(),
            None if self.ctx.noreturn.borrow().answer_never(t) => {
                self.ctx.dep(me, Dep::Never);
                Ret::Never
            }
            None => eval(),
        }
    }
    fn field(&self, opcode: u8, f: &MemberRef, recv: Option<&V>) -> Option<V> {
        if let Some(v) = self.ctx.object_field(self.m, opcode, f, recv) {
            return Some(v);
        }
        self.ctx.field_value(self.m, f)
    }
    fn construct(&self, init: &MemberRef, args: &[V]) -> Option<Rc<Obj>> {
        self.ctx.construct(init, args)
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

    #[test]
    fn core_name_matches_mangled_or_bare() {
        assert!(is_core_of("core_len", Some("len"), "len__I"));
        assert!(is_core_of("core_len__I", None, "len__I"));
        assert!(!is_core_of("core_len", None, "len__I"));
        assert!(!is_core_of("len", Some("len"), "len__I"));
    }
}
