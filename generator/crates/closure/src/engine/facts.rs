//! 常量 / 事实查询：常量格 `PV`、分析上下文 `Ctx`、absint 的 `Oracle` 实现 `Facts`。


mod calls;
mod fields;
mod kinds;
mod oracle;

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
            V::Int(_) | V::Ints(_) | V::Long(_) | V::Null | V::Offset(_) => PV::Const(v.clone()),
            V::Str(..) => PV::Const(v.stripped()),
            V::Ref { .. } if v.obj().is_some() || v.shape_tagged() => PV::Const(v.stripped()),
            _ => PV::Top,
        }
    }
    /// 返回值 / 实参 → 返回常量格与形参常量格：在 [`PV::of`] 之上，确定非空、无标签的引用记为「非空引用」
    /// （[`nonnull_ref`]）。调用点 / 被调方法体据此判定 `ifnull` / `ifnonnull`（如恒返回新建对象的工厂方法、
    /// 实参恒为调用者类镜像的形参）；形参入口按形参序号换来源、按描述符补类型（`absint::entry_state`）。
    /// 字段常量格只有静态字段取它（`static_init::write_pv`；读点按描述符补类型），实例字段不取
    pub(super) fn of_ret(v: &V) -> PV {
        match v {
            V::Ref { nonnull: true, obj: None, .. } => PV::Const(nonnull_ref()),
            // 构建器标签只在方法内有效
            V::Ref { nonnull: true, .. } if v.builder().is_some() => PV::Const(nonnull_ref()),
            _ => PV::of(v),
        }
    }
    /// 形参 / 返回 / 静态字段常量格的合流。两侧都确定非空而值 / 标签 / 形状无法合流时取「非空引用」
    /// （如一条路径返回常量串、另一条返回带字段标签的新建对象）
    pub(super) fn join(a: Option<&PV>, b: &PV) -> PV {
        let both_nonnull = |x: &V, y: &V| x.nonnull() == Some(true) && y.nonnull() == Some(true);
        match (a, b) {
            (None, x) => x.clone(),
            (Some(PV::Const(x)), PV::Const(y)) if x == y => PV::Const(x.clone()),
            // 非空引用与确定非空的引用（含带标签的）合流：仍是非空引用
            (Some(PV::Const(x)), PV::Const(y)) if (is_nonnull_ref(x) || is_nonnull_ref(y)) && both_nonnull(x, y) => {
                PV::Const(nonnull_ref())
            }
            // int 族常量：取有限并
            (Some(PV::Const(x)), PV::Const(y)) if crate::absint::ints::members(x).is_some() && crate::absint::ints::members(y).is_some() => {
                crate::absint::ints::union(x, y).map_or(PV::Top, PV::Const)
            }
            // 字符串形状（常量 / 形状标签 / null 之间）：合流取形状的并
            (Some(PV::Const(x)), PV::Const(y)) if x.shape_tagged() || y.shape_tagged() || matches!((x, y), (V::Str(..), V::Str(..))) => match x.join(y) {
                j @ V::Ref { .. } if j.shape_tagged() => PV::Const(j.stripped()),
                _ if both_nonnull(x, y) => PV::Const(nonnull_ref()),
                _ => PV::Top,
            },
            // 同一对象标签（或 null 与标签对象）：合流保留标签，可空性取并
            (Some(PV::Const(x)), PV::Const(y)) if (x.obj().is_some() || y.obj().is_some()) && matches!(x.join(y), V::Ref { obj: Some(_), .. }) => {
                PV::Const(x.join(y).stripped())
            }
            // 其余两侧都确定非空的引用（不同标签、不同字符串常量）：非空引用
            (Some(PV::Const(x)), PV::Const(y)) if both_nonnull(x, y) => {
                PV::Const(nonnull_ref())
            }
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
    /// 映像 VM 模块表：包 → [(模块对象, 定义加载器为引导)]（装入映像时建立；类镜像模块读折叠用）
    pub(super) img_modules: std::cell::OnceCell<HashMap<String, Vec<(u32, bool)>>>,
    /// 映像 VM 模块对象 → 其映像标签（带 final 实例字段常量，如定义加载器；装入映像时建立）
    pub(super) img_module_tags: std::cell::OnceCell<HashMap<u32, Rc<crate::absint::Obj>>>,
    /// 映像中构建期初始化类的静态字段初值（装入映像时建立，见 `static_init.rs`）
    pub(super) img_statics: std::cell::RefCell<Option<super::static_init::ImgStatics>>,
    /// 选择子形参缓存（见 `selector.rs`）
    pub(super) selectors: RefCell<HashMap<MemberRef, u64>>,
    /// 调用方 → 调用点偏移 → 字节码字面常量实参掩码（见 `selector.rs` `site_literals`）
    pub(super) site_lits: RefCell<HashMap<MemberRef, Rc<HashMap<u32, super::selector::SiteLits>>>>,
    /// 非 static final 字段的值集（初值 ∪ 可达写入；缺席 = 只有初值）
    pub(super) fvals: RefCell<HashMap<MemberRef, PV>>,
    /// 按抽象对象的实例字段写入值：字段 → 抽象对象 → 值（见 `obj_fields.rs`）
    pub(super) ovals: RefCell<HashMap<MemberRef, HashMap<u32, PV>>>,
    /// 不按抽象对象分开的实例字段写入值（接收者含非抽象对象 / 物化快照）
    pub(super) owild: RefCell<HashMap<MemberRef, PV>>,
    /// 构造器的确定初始化摘要（永久缓存，见 `ctor_init.rs`）
    pub(super) cinits: RefCell<HashMap<MemberRef, super::ctor_init::CInit>>,
    /// 被调方法 → 读过其返回常量的构造器摘要（`rvals` 该项变化时作废，见 `ctor_init.rs`）
    pub(super) cinit_rdeps: RefCell<HashMap<MemberRef, Vec<MemberRef>>>,
    /// 正在计算的构造器摘要各自读过返回常量的被调方法（栈，嵌套摘要并入外层）
    pub(super) cinit_rets: RefCell<Vec<BTreeSet<MemberRef>>>,
    /// `cinits` 有条目作废（`odef` 待清空、按对象读者待复核，见 `ctor_init.rs`）
    pub(super) cinit_drop: Cell<bool>,
    /// 字节码 `new` 分配的抽象对象 → (类, 分配方法里该类的构造器调用)
    pub(super) osite: RefCell<HashMap<u32, (Rc<str>, Rc<[MemberRef]>)>>,
    /// 全部实例字段由构建期内容给出的抽象对象，按对象读不并入初值：映像抽象对象（`image_start.rs`，未列出的字段即缺省值，
    /// 已显式记入 `ovals`），以及具体求值结果按对象物化的对象（`concrete/apply.rs`，快照含全部实例字段）
    pub(super) ofull: RefCell<HashSet<u32>>,
    /// 抽象对象上确定初始化的字段（惰性，见 `ctor_init.rs::obj_definite`）
    pub(super) odef: RefCell<HashMap<u32, Rc<[MemberRef]>>>,
    /// 字段 → 抽象对象 → 按对象读过它的方法（`ovals` 该项变化时失效；开放判定变化走 `fdeps`）
    pub(super) odeps: RefCell<HashMap<MemberRef, HashMap<u32, BTreeSet<usize>>>>,
    /// 字段 → 按对象读过它的方法（`owild` 变化时失效）
    pub(super) owdeps: RefCell<HashMap<MemberRef, BTreeSet<usize>>>,
    /// 实例方法 → 抽象对象 → 接收者含该对象的方法节点返回值之并（见 `obj_rets.rs`）
    pub(super) orvals: RefCell<HashMap<MemberRef, HashMap<u32, PV>>>,
    /// 实例方法 → 接收者不按对象归属的返回值（具体求值结果）
    pub(super) orwild: RefCell<HashMap<MemberRef, PV>>,
    /// 实例方法 → 抽象对象 → 按对象读过其返回值的方法
    pub(super) ordeps: RefCell<HashMap<MemberRef, HashMap<u32, BTreeSet<usize>>>>,
    /// 实例方法 → 按对象读过其返回值的方法（`orwild` 变化时复核）
    pub(super) orwdeps: RefCell<HashMap<MemberRef, BTreeSet<usize>>>,
    /// 按对象查询过的抽象对象 → 其类（`obj_rets.rs` 按对象选择调用目标）
    pub(super) oclass: RefCell<HashMap<u32, Rc<str>>>,
    /// 按对象选择的调用目标缓存：调用点 (指令, 符号引用, 接口调用) → 抽象对象 → 目标（选择只看对象的类与类层次，不随分析变化）
    pub(super) oret_sel: RefCell<HashMap<(u8, bool, MemberRef), HashMap<u32, Option<Rc<MemberRef>>>>>,
    /// 字节码方法的返回常量（缺席 = 尚无返回路径）
    pub(super) rvals: RefCell<HashMap<MemberRef, PV>>,
    /// 偏移可得、不折叠的字段：反射 / VarHandle / Unsafe 按名取得的字段
    pub(super) fopen: RefCell<HashSet<MemberRef>>,
    /// 求值中折叠出符号偏移、尚待放开的字段（上下文只读，由工作循环排空经 `open_field` 放开并使读者失效）
    pub(super) fopen_pending: RefCell<Vec<MemberRef>>,
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
    /// 键为拼接值的属性读取点的候选模式：(方法节点, 键值来源站点)（`sysprops_key.rs`）
    pub(super) pkeys: RefCell<HashMap<(usize, Src), super::sysprops_key::KeyPats>>,
    /// 已登记过候选模式（含求不出模式）的键为拼接值的读取点：(方法节点, 键值来源站点)；未登记前读取答复 ⊥
    pub(super) pkeys_seen: RefCell<HashSet<(usize, Src)>>,
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
    /// 常量实参求值的嵌套位置（`consteval.rs`）
    pub(super) ceval_depth: Cell<super::consteval::EvalDepth>,
    /// 分派转发槽判定缓存（按成员）：流到分派接收者的形参槽；静态方法非空即按调用点区分上下文（`forward.rs`），
    /// 常量实参求值穿过转发方法不计深度（`consteval.rs`）
    pub(super) forwarders: RefCell<HashMap<MemberRef, u64>>,
    /// 元素封存的数组字段判定缓存（`sealed_elems.rs`）
    pub(super) sealed_arrs: RefCell<HashMap<MemberRef, bool>>,
    /// 调用点派发集（按调用方成员与偏移汇合各上下文，`deval.rs`）与查询过它的方法
    pub(super) vdisp: RefCell<HashMap<(MemberRef, u32), super::deval::VDisp>>,
    pub(super) vwatch: RefCell<HashMap<(MemberRef, u32), BTreeSet<usize>>>,
    /// 进行中的分派求值：发起的方法节点、嵌套深度、剩余求值次数（`deval.rs`）
    pub(super) dv_top: Cell<Option<usize>>,
    pub(super) dv_depth: Cell<u32>,
    pub(super) dv_budget: Cell<u32>,
    /// 分派求值诊断轨迹（`--flows @vals:`；None = 不记录）
    pub(super) dv_trace: RefCell<Option<Vec<String>>>,
    /// 一次分派求值内的结果缓存（`deval.rs`；键为目标与绑定实参，值 None = 未知、Some(None) = 不返回）
    pub(super) dv_cache: RefCell<HashMap<String, Option<Option<V>>>>,
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
    /// 清单确定非空的调用结果：空的不可修改集合工厂（带 `Obj::Empty` 标签，`[facts.empty_collections] factories`）、
    /// 调用者类镜像（`caller_class`，运行期恒有调用者或回退根类，见 `vm_intrinsics.toml`）
    pub(super) nonnull_ret: Option<V>,
    /// 接收者为空集合时的查询结果（`[facts.empty_collections] queries`）
    pub(super) empty_query: Option<V>,
    /// 清单字符串操作种类（构建器 / 前后缀判定，absint `strs.rs`）
    pub(super) str_kind: Option<crate::absint::StrKind>,
    /// 返回串的形状事实（`[facts.string_shapes]`，带形状标签的可空 String 引用）
    pub(super) shape: Option<V>,
    /// 取调用者类（清单 `caller_class`）
    pub(super) caller_class: bool,
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
    /// 引导档位上下文（残差步骤的运行期档位）：清单 `level_queries` 的调用按档位折叠
    pub(super) level: Option<i32>,
    /// 形参的抽象对象集与按对象读过的形参（见 `obj_fields.rs`）
    pub(super) objs: super::obj_fields::ObjParams,
    /// 被分析方法是 @CallerSensitive 时其调用者镜像值集所指的类（`caller.rs`；None = 非 CS 方法或值集含所指未知的 Class）
    pub(super) callers: Option<BTreeSet<Rc<str>>>,
    /// 取调用者类的调用点偏移（其结果的类镜像值集即 `callers`，见 [`Oracle::site_mirror_call`]）
    pub(super) caller_sites: RefCell<BTreeSet<u32>>,
    /// 静态调用点按克隆节点的返回值答复（`site_rets.rs`；空 = 全部走按成员的返回常量格）
    pub(super) sites: super::site_rets::SiteTable,
    /// 被分析方法的成员（方法体分析与分派求值时给出：按调用点派发集求值，`deval.rs`）
    pub(super) key: Option<MemberRef>,
    /// 分派求值中的嵌套分析（`deval.rs`）
    pub(super) dv: bool,
}

pub(super) fn const_value(c: &Const) -> Option<V> {
    match c {
        Const::Int(v) => Some(V::Int(*v)),
        Const::Long(v) => Some(V::Long(*v)),
        Const::String(s) => Some(V::lit(Rc::from(s.as_str()))),
        _ => None,
    }
}
