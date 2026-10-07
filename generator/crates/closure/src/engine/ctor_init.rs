//! 构造器确定初始化（计划 c1d §30.9 的 P2）：字节码 `new` 分配的抽象对象，经构造器完成后必然已写的实例字段，
//! 按对象读（`obj_fields.rs`）不再并入其初值。
//!
//! - **摘要**：构造器方法体经 `absint::analyze_init` 得出 [`InitSum`]（`absint/init.rs`），以辅助分析的事实
//!   （清单事实、常量求值，`Facts { m: None }`）在记忆化帧内分析，记入 `cinits`，输入记录与求值记忆同一套：
//!   读过的字段转为不折叠、或属性不折叠集合增长时作废（`consteval.rs::ceval_drop` / `sysprops.rs`），
//!   被委托 / 超类构造器摘要的输入并入外层。辅助事实答不出的调用取唯一字节码目标的返回常量 `rvals`
//!   （缺席 / Top 按未知），读过的目标记入摘要（嵌套摘要并入外层），该目标返回常量变化时作废（`cinit_ret_changed`）。作废后 `odef` 整表清空，按对象读过的全部方法按答复复核
//!   （`obj_defs_dropped`），答复有变才重分析——终态等于按最终不折叠集合计算的摘要，与处理顺序无关。
//! - **分配点**：`bytecode.rs` 的 `Event::New` 把抽象对象登记到 `osite`：该方法里全部 `invokespecial cls.<init>`。
//!   JVMS §4.10.1.9：`new cls` 的未初始化对象只能经 `cls` 自身的 `<init>` 初始化，初始化前不能被使用，
//!   所以分配方法里的这些调用覆盖了该对象实际执行的构造器。抽象对象名含（方法键, 偏移），字节码 `new`、
//!   手写分配（偏移 `u32::MAX - k`）、lambda 构造引用（indy 偏移）三者的偏移空间互不相交，手写 / lambda 分配的
//!   对象不在 `osite` 中，保留初值。
//! - **确定集**（`odef`）：各构造器确定集的交；类链（不含根类）上声明了非抽象 `finalize()V` 时为空——
//!   终结器可在构造器异常退出后读到半初始化对象。
//!
//! 健全性：读到对象字段须先持有引用。构造链帧内直接读（`getfield this`）在未写入时记入 `bad`；
//! 引用经交出点流出时，只有交出时已写的字段保留；构造器正常返回后的读取只看返回集。未初始化对象受校验器约束不能被使用。
//! 反序列化 / 物化快照分配的是类 id，不是抽象对象；`clone` 复制的是已写入的值。

use super::memo::Inputs;
use super::*;
use crate::absint::{InitSum, Obj, Oracle, Ret, StrKind, V};

/// 构造器摘要缓存项
#[derive(Clone)]
pub(super) struct CInit {
    pub(super) sum: Option<Rc<InitSum>>,
    pub(super) inp: Inputs,
    /// 读过返回常量的被调方法（含嵌套摘要的）
    pub(super) rets: Rc<BTreeSet<MemberRef>>,
}

/// 摘要分析的事实：辅助分析事实（`Facts { m: None }`）之外，调用的唯一字节码目标取其返回常量 `rvals`
/// （缺席 / 汇合为 Top 按未知，不取「不返回」），读过的目标记入摘要的返回常量依赖
struct InitFacts<'a, 'b>(Facts<'a, 'b>);

impl Oracle for InitFacts<'_, '_> {
    fn invoke_result(&self, opcode: u8, m: &MemberRef, iface: bool, args: &[V]) -> Ret {
        let r = self.0.invoke_result(opcode, m, iface, args);
        if !matches!(r, Ret::Unknown) {
            return r;
        }
        let ctx = self.0.ctx;
        let Some(t) = ctx.call_info(opcode, m, iface).target.clone() else { return r };
        let v = ctx.rvals.borrow().get(&t).cloned();
        if let Some(top) = ctx.cinit_rets.borrow_mut().last_mut() {
            top.insert(t);
        }
        match v {
            Some(PV::Const(v)) => Ret::Value(v),
            _ => Ret::Unknown,
        }
    }
    fn field(&self, opcode: u8, f: &MemberRef, recv: Option<&V>) -> Option<V> {
        self.0.field(opcode, f, recv)
    }
    fn construct(&self, init: &MemberRef, args: &[V]) -> Option<Rc<Obj>> {
        self.0.construct(init, args)
    }
    fn param(&self, i: u16) -> Option<V> {
        self.0.param(i)
    }
    fn type_live(&self, ty: &str) -> bool {
        self.0.type_live(ty)
    }
    fn final_static(&self, f: &MemberRef) -> bool {
        self.0.final_static(f)
    }
    fn param_mirror(&self, i: u16, cls: &str) -> Option<bool> {
        self.0.param_mirror(i, cls)
    }
    fn param_mirror_field(&self, i: u16, f: &MemberRef) -> Option<V> {
        self.0.param_mirror_field(i, f)
    }
    fn mirror_subtype_test(&self, m: &MemberRef) -> bool {
        self.0.mirror_subtype_test(m)
    }
    fn str_kind(&self, opcode: u8, m: &MemberRef, iface: bool) -> Option<StrKind> {
        self.0.str_kind(opcode, m, iface)
    }
    fn final_field(&self, f: &MemberRef) -> bool {
        self.0.final_field(f)
    }
    fn init_key(&self, f: &MemberRef) -> Option<MemberRef> {
        self.0.init_key(f)
    }
    fn init_sum(&self, init: &MemberRef) -> Option<Rc<InitSum>> {
        self.0.init_sum(init)
    }
}

impl Ctx<'_> {
    /// 实例字段的解析后声明键
    pub(super) fn init_key(&self, f: &MemberRef) -> Option<MemberRef> {
        self.field_info(f).filter(|fi| fi.access & acc::STATIC == 0).map(|fi| fi.key.clone())
    }

    /// 构造器 init 的确定初始化摘要（记忆化，输入同求值记忆）；None = 无法分析（手写 / 边界 / 保守 / 递归）
    pub(super) fn ctor_init(&self, init: &MemberRef) -> Option<Rc<InitSum>> {
        let hit = self.cinits.borrow().get(init).cloned();
        let (r, inp, rets) = match hit {
            Some(c) => (c.sum, c.inp, c.rets),
            None => {
                self.cinit_rets.borrow_mut().push(BTreeSet::new());
                let (r, clean, inp) = self.ctor_init_calc(init);
                let rets: Rc<BTreeSet<MemberRef>> = Rc::new(self.cinit_rets.borrow_mut().pop().unwrap_or_default());
                if clean {
                    let mut rd = self.cinit_rdeps.borrow_mut();
                    for t in rets.iter() {
                        rd.entry(t.clone()).or_default().push(init.clone());
                    }
                    self.cinits.borrow_mut().insert(init.clone(), CInit { sum: r.clone(), inp: inp.clone(), rets: rets.clone() });
                }
                (r, inp, rets)
            }
        };
        // 外层是另一构造器摘要：输入与返回常量依赖并入其记录（被调摘要作废时外层一并作废）
        self.memo_use(None, &inp);
        if let Some(top) = self.cinit_rets.borrow_mut().last_mut() {
            top.extend(rets.iter().cloned());
        }
        r
    }

    /// 方法 t 的返回常量有变：读过它的构造器摘要作废（随后的 `invalidate_all` 经 `obj_defs_dropped` 复核读者）
    pub(super) fn cinit_ret_changed(&self, t: &MemberRef) {
        let Some(cs) = self.cinit_rdeps.borrow_mut().remove(t) else { return };
        let mut ci = self.cinits.borrow_mut();
        for c in cs {
            if ci.remove(&c).is_some() {
                self.cinit_drop.set(true);
            }
        }
    }

    /// (摘要, 可记忆, 输入)
    fn ctor_init_calc(&self, init: &MemberRef) -> (Option<Rc<InitSum>>, bool, Inputs) {
        let none = || (None, true, Inputs::default());
        let Some(cf) = self.h.class(&init.owner) else { return none() };
        let Some(meth) = cf.method(&init.name, &init.desc) else { return none() };
        // 根类的构造器按其字节码（空体）：手写承载不改变「不写任何字段、不交出」
        if cf.super_name.is_some() && self.kind_of(&cf, meth) != Kind::Bytecode {
            return none();
        }
        let Some(code) = meth.code.as_ref() else { return none() };
        let Some(frame) = self.memo_enter(format!("<init>#{init}"), true) else { return (None, false, Inputs::default()) };
        let live = |_: &str| true;
        let facts = Facts { ctx: self, live: &live, m: None, params: vec![], mirrors: vec![], level: None, objs: Default::default() };
        let r = absint::analyze_init(&cf.name, &init.desc, code, &InitFacts(facts));
        let (clean, inp) = self.memo_leave(frame);
        (r.map(Rc::new), clean, inp)
    }

    /// 抽象对象 o 上确定初始化的字段（排序）；不是字节码 `new` 分配的对象为空
    pub(super) fn obj_definite(&self, o: u32) -> Rc<[MemberRef]> {
        if let Some(d) = self.odef.borrow().get(&o) {
            return d.clone();
        }
        let d: Rc<[MemberRef]> = match self.osite.borrow().get(&o).cloned() {
            Some((cls, ctors)) if !self.finalizable(&cls) => {
                let mut acc: Option<Vec<MemberRef>> = None;
                for c in ctors.iter() {
                    let d = self.ctor_init(c).map(|s| s.definite()).unwrap_or_default();
                    match &mut acc {
                        Some(a) => a.retain(|k| d.binary_search(k).is_ok()),
                        None => acc = Some(d),
                    }
                }
                acc.unwrap_or_default().into()
            }
            _ => Rc::from([]),
        };
        self.odef.borrow_mut().insert(o, d.clone());
        d
    }

    /// 类链（不含根类）上声明了非抽象 `finalize()V`
    fn finalizable(&self, cls: &str) -> bool {
        let mut cur = self.h.class(cls);
        while let Some(cf) = cur {
            let Some(sup) = cf.super_name.as_deref() else { return false };
            if cf.methods.iter().any(|m| m.name == "finalize" && m.desc == "()V" && m.access & acc::ABSTRACT == 0) {
                return true;
            }
            cur = self.h.class(sup);
        }
        // 类链不全：按可终结处理
        true
    }
}

impl Engine<'_> {
    /// 构造器摘要有作废：确定集整表清空，按对象读过的方法记入待复核（`obj_flush`），答复有变才重分析
    pub(super) fn obj_defs_dropped(&mut self) {
        if !self.ctx.cinit_drop.replace(false) {
            return;
        }
        self.ctx.stats.borrow_mut().init_drops += 1;
        self.ctx.odef.borrow_mut().clear();
        let ms: BTreeSet<usize> = self.obj_queries.keys().copied().collect();
        self.obj_readers_recheck(ms, super::obj_fields::ObjCause::All);
    }

    /// 字节码 `new cls`（方法 m，结果为抽象对象 o）：登记分配方法里的 `cls` 构造器调用
    pub(super) fn osite_note(&mut self, m: usize, o: u32, cls: &str, cf: &Option<std::sync::Arc<ClassFile>>) {
        if self.ctx.osite.borrow().contains_key(&o) {
            return;
        }
        let key = &self.methods[m].key;
        let Some(code) = cf.as_ref().and_then(|cf| cf.method(&key.name, &key.desc)).and_then(|mm| mm.code.as_ref()) else { return };
        let mut ctors: Vec<MemberRef> = code
            .insns
            .iter()
            .filter_map(|x| match &x.operand {
                classfile::Operand::Method(r, _) if x.opcode == classfile::op::INVOKESPECIAL && r.name == "<init>" && r.owner == cls => Some(r.clone()),
                _ => None,
            })
            .collect();
        ctors.sort();
        ctors.dedup();
        self.ctx.osite.borrow_mut().insert(o, (Rc::from(cls), ctors.into()));
    }
}
