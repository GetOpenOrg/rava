//! 系统属性表对象经 lambda 返回（SAM 调用点进入的实现方法返回属性表对象）的逃逸判定。
//!
//! 实现方法由 SAM 调用点进入，调用点所在方法的分析与创建点无关，拿不到带标签的结果；但若 lambda 对象
//! 「封闭」于创建它的方法 O 的一次调用，结果只沿这次调用原路返回 O，便可在 O 里按值的来源检查它的用途：
//!
//! - lambda 对象在 O 里（indy 结果，含 checkcast / instanceof 收窄出的别名）只作唯一字节码目标 X 的
//!   非接收者实参（目标为清单事实替换的不算）；
//! - X 里该形参只作 SAM 调用的接收者（名字为 SAM、解析到抽象方法），或再作下一层唯一字节码目标的实参
//!   （逐层同判定，层数有上限）；它的 SAM 调用结果与下层调用结果（称「携带值」）在 X 里只作返回值；
//! - O 里调用 X 的结果（携带值）只按只读规则使用：只读查询 / 读取入口的接收者，或唯一字节码目标上的
//!   只读形参（与 `sysprops.rs` 的形参只读判定同口径）；作返回值、写入字段 / 数组、作 indy 实参都算逃逸。
//!
//! 实现方法的每个 lambda 创建点都封闭时，它的 lambda 入口（SAM 调用点进入，及静态 / 私有实现在创建点的
//! 预先入链）按可跟踪处理；任一不封闭、实现句柄不是
//! 静态 / 私有（虚派发的实现方法另由派发入口判定）、或分析保守 → 仍按非字节码入口（全部不折叠）。
//! 判定以辅助分析做，输入（读过的字段 / 记忆）登记给实现方法节点：输入变化时实现方法重分析、重判。

use super::*;

/// SAM 实参经唯一目标逐层转交的层数上限（超出按不封闭）
const SAM_DEPTH: u32 = 4;

/// 值的来源与集合相交
fn hits(v: &V, set: &BTreeSet<Src>) -> bool {
    v.srcs().iter().any(|s| set.contains(s))
}

/// 别名闭包：转型 / 类型测试收窄出的值以其偏移为来源，输入在集合中的并入
fn alias_close(a: &Analysis, mut set: BTreeSet<Src>) -> BTreeSet<Src> {
    loop {
        let n = set.len();
        for (o, e) in &a.events {
            let inp = match e {
                Event::CheckCast(_, Some(v)) | Event::InstanceOf(_, Some(v)) | Event::NotInstance(_, v) | Event::MirrorSub(_, v) | Event::KeyTest { input: v, .. } => v,
                _ => continue,
            };
            if hits(inp, &set) {
                set.insert(Src::Site(*o));
            }
        }
        if set.len() == n {
            return set;
        }
    }
}

impl Ctx<'_> {
    /// 字节码方法 t 的辅助分析（形参值未知，类型一律按存活）；无字节码 / 保守 → None
    fn lambda_aux(&self, t: &MemberRef) -> Option<Analysis> {
        let cf = self.h.class(&t.owner)?;
        let meth = cf.method(&t.name, &t.desc)?;
        let code = meth.code.as_ref()?;
        let n = parse_method(&t.desc)?.params.len() + usize::from(!meth.is_static());
        let live = |_: &str| true;
        let a = self.aux_analyze(&cf.name, &t.desc, meth.is_static(), code, &Facts { ctx: self, live: &live, m: None, params: vec![None; n], mirrors: vec![], level: None, objs: Default::default(), callers: None, caller_sites: Default::default(), sites: Rc::from([]) });
        (!a.conservative).then_some(a)
    }

    /// lambda 对象（来源集合 lam）在分析 a 里的用途：只作 SAM 调用接收者或封闭转交的实参时，
    /// 返回携带值的来源（这些调用点的结果）；其余用途 → None
    fn lambda_uses(&self, a: &Analysis, lam: &BTreeSet<Src>, sam: &str, depth: u32) -> Option<BTreeSet<Src>> {
        let mut car = BTreeSet::new();
        for (o, e) in &a.events {
            match e {
                Event::Invoke { opcode, mref, iface, args } => {
                    let pos: Vec<usize> = args.iter().enumerate().filter(|(_, v)| hits(v, lam)).map(|(i, _)| i).collect();
                    let [j] = pos.as_slice() else {
                        if pos.is_empty() {
                            continue;
                        }
                        return None;
                    };
                    if *j == 0 && *opcode != classfile::op::INVOKESTATIC {
                        let abstract_sam = mref.name == sam && self.h.resolve_method(&mref.owner, &mref.name, &mref.desc, *iface).is_some_and(|s| s.method().is_abstract());
                        if !abstract_sam {
                            return None;
                        }
                    } else {
                        let c = self.call_info(*opcode, mref, *iface);
                        let t = c.target.as_ref().filter(|_| c.fact.is_none())?;
                        if !self.sam_confined(t, *j, sam, depth + 1) {
                            return None;
                        }
                    }
                    car.insert(Src::Site(*o));
                }
                Event::Indy { args, .. } if args.iter().any(|v| hits(v, lam)) => return None,
                Event::Field { recv, value, .. } if [recv, value].iter().any(|v| v.as_ref().is_some_and(|v| hits(v, lam))) => return None,
                Event::ArrayStore { value, .. } | Event::Return(value) | Event::Throw(value) if hits(value, lam) => return None,
                _ => {}
            }
        }
        Some(car)
    }

    /// 字节码方法 t 的第 j 个实参（含接收者序号）为 lambda 对象时封闭：只作 SAM 调用接收者或再封闭转交，
    /// 携带值只作返回值
    fn sam_confined(&self, t: &MemberRef, j: usize, sam: &str, depth: u32) -> bool {
        if depth > SAM_DEPTH {
            return false;
        }
        let Some(a) = self.lambda_aux(t) else { return false };
        let Ok(j) = u16::try_from(j) else { return false };
        let lam = alias_close(&a, BTreeSet::from([Src::Param(j)]));
        let Some(car) = self.lambda_uses(&a, &lam, sam, depth) else { return false };
        let car = alias_close(&a, car);
        a.events.iter().all(|(_, e)| match e {
            Event::Invoke { args, .. } | Event::Indy { args, .. } => !args.iter().any(|v| hits(v, &car)),
            Event::Field { recv, value, .. } => ![recv, value].iter().any(|v| v.as_ref().is_some_and(|v| hits(v, &car))),
            Event::ArrayStore { value, .. } | Event::Throw(value) => !hits(value, &car),
            _ => true,
        })
    }

    /// 创建点 o@off 的 lambda（SAM 名 sam）封闭于 O 的一次调用，且调用结果在 O 里只读使用
    fn lambda_site_confined(&self, o: &MemberRef, off: u32, sam: &str) -> bool {
        let Some(a) = self.lambda_aux(o) else { return false };
        let lam = alias_close(&a, BTreeSet::from([Src::Site(off)]));
        let Some(car) = self.lambda_uses(&a, &lam, sam, 0) else { return false };
        let car = alias_close(&a, car);
        a.events.iter().all(|(_, e)| match e {
            Event::Invoke { opcode, mref, iface, args } => {
                args.iter().enumerate().filter(|(_, v)| hits(v, &car)).all(|(i, _)| self.sysprops_arg_ok(None, *opcode, mref, *iface, i))
            }
            Event::Indy { args, .. } => !args.iter().any(|v| hits(v, &car)),
            Event::Field { value, .. } => !value.as_ref().is_some_and(|v| hits(v, &car)),
            Event::ArrayStore { value, .. } | Event::Return(value) | Event::Throw(value) => !hits(value, &car),
            _ => true,
        })
    }
}

impl Engine<'_> {
    /// 新登记的 lambda 创建点：实现方法已按 lambda 入口判定过的，重判
    pub(super) fn sysprops_lambda_new(&mut self, imh: &MethodHandle) {
        if self.man.sysprops.is_empty() || !matches!(imh.kind, 6 | 7) {
            return;
        }
        let k = &imh.member;
        let Some(site) = self.h.resolve_method(&k.owner, &k.name, &k.desc, imh.interface) else { return };
        let (owner, name, desc) = site.key();
        self.sysprops_lambda_recheck(&MemberRef { owner, name, desc });
    }

    /// 实现方法 key 的 lambda 入口可跟踪：各创建点的 lambda 都封闭（判定输入登记给实现方法节点 m）
    pub(super) fn sysprops_lambda_tracked(&mut self, m: usize, key: &MemberRef) -> bool {
        let mut sites: Vec<(MemberRef, u32, String)> = Vec::new();
        for l in self.lambdas.values() {
            if !matches!(l.imh.kind, 6 | 7) {
                continue;
            }
            let k = &l.imh.member;
            let Some(site) = self.h.resolve_method(&k.owner, &k.name, &k.desc, l.imh.interface) else { continue };
            let (o, n, d) = site.key();
            if o == key.owner && n == key.name && d == key.desc {
                let om = l.site.0;
                if self.methods[om].kind != Kind::Bytecode {
                    return false;
                }
                sites.push((self.methods[om].key.clone(), l.site.1, l.sam.clone()));
            }
        }
        if sites.is_empty() {
            return false;
        }
        sites.sort();
        sites.dedup();
        let Some(frame) = self.ctx.memo_enter(format!("plam:{key}"), true) else { return false };
        let ok = sites.iter().all(|(o, off, sam)| self.ctx.lambda_site_confined(o, *off, sam));
        let (_, inp) = self.ctx.memo_leave(frame);
        self.ctx.memo_use(Some(m), &inp);
        ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alias_close_follows_casts() {
        let r = |s: Src| V::Ref { ty: None, nonnull: true, src: Rc::from([s].as_slice()), obj: None };
        let a = Analysis {
            reachable: vec![],
            events: vec![(4, Event::CheckCast("C".into(), Some(r(Src::Site(1))))), (9, Event::InstanceOf("D".into(), Some(r(Src::Site(4))))), (12, Event::CheckCast("E".into(), Some(r(Src::Site(2)))))],
            pending_types: vec![],
            mirror_assumed: vec![],
            mirror_field_assumed: vec![],
            site_mirror_assumed: false,
            conservative: false,
            cfg: Rc::new(Default::default()),
            selector_params: 0,
        };
        let s = alias_close(&a, BTreeSet::from([Src::Site(1)]));
        assert_eq!(s, BTreeSet::from([Src::Site(1), Src::Site(4), Src::Site(9)]));
    }
}
