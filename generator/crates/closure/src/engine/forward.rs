//! 引擎：静态分派转发方法的调用点上下文（计划 boundary-narrowing §6.2 G1）。
//!
//! 分派转发方法：静态方法的引用形参（经 checkcast、经下游非虚调用的转发槽、作为字符串拼接的动态实参）
//! 流到虚分派 / 接口分派的接收者。形参按全部调用方汇合时，各调用点的实参类型一起参与分派——
//! 特权块 `doPrivileged → executePrivileged → action.run()` 的动作形参跨调用点合并，任一调用点的动作
//! 都被全部调用点执行。
//!
//! 策略：调用点敏感，k = 1。上下文无关的调用方调用转发方法时，被调方按调用点克隆（上下文 = 调用点，
//! 与新鲜工厂同一键）；调用方已在某个上下文（容器对象 / 调用点）中时继承之——转发链
//! （`doPrivileged → executePrivileged`）整条随最外层调用点分开。判定只看字节码结构，不按类名；
//! 克隆只细分值集，不改变任何调用点的可达性判定。
//!
//! 实例方法（G2：`append(Object)` / `valueOf(Object)` / `Objects.equals` 等汇合点）不按调用点克隆：
//! 实测（§6.7）按同一规则扩展到实例方法后闭包类集与只做静态方法时逐一相同，分析耗时 ×6–10——
//! 汇合点上的 open(Object) 在调用点处已是 open（`Object[]` 元素读、手写返回等引入点），调用点上下文分不开。

use super::*;

/// 形参槽位掩码（按形参槽，实例方法 0 = this）；超过 64 槽的形参不参与判定
type SlotMask = u64;

fn bit(i: u16) -> SlotMask {
    if i < 64 {
        1 << i
    } else {
        0
    }
}

impl<'a> Engine<'a> {
    /// 分派转发方法：静态方法，且有引用形参流到分派接收者
    pub(super) fn forwarder(&mut self, key: &MemberRef) -> bool {
        let is_static = self.h.class(&key.owner).and_then(|cf| cf.method(&key.name, &key.desc).map(|m| m.is_static())).unwrap_or(false);
        is_static && self.dispatch_slots(key) != 0
    }

    /// 流到分派接收者的形参槽（按成员缓存；递归成环处取 0——只少克隆，不影响可达性）
    pub(super) fn dispatch_slots(&mut self, key: &MemberRef) -> SlotMask {
        if let Some(&r) = self.forwarders.get(key) {
            return r;
        }
        self.forwarders.insert(key.clone(), 0);
        let r = self.dispatch_slots_uncached(key);
        self.forwarders.insert(key.clone(), r);
        r
    }

    fn dispatch_slots_uncached(&mut self, key: &MemberRef) -> SlotMask {
        use classfile::op;
        let Some(cf) = self.h.class(&key.owner) else { return 0 };
        let Some(meth) = cf.method(&key.name, &key.desc) else { return 0 };
        if self.kind_of(&cf, meth) != Kind::Bytecode {
            return 0;
        }
        let Some(md) = parse_method(&key.desc) else { return 0 };
        if !md.params.iter().any(|p| p.is_reference()) && meth.is_static() {
            return 0;
        }
        let Some(code) = meth.code.as_ref() else { return 0 };
        let live = |_: &str| true;
        let a = self.ctx.aux_analyze(&key.owner, &key.desc, meth.is_static(), code, &Facts { ctx: &self.ctx, live: &live, m: None, params: vec![], mirrors: vec![] });
        if a.conservative {
            return 0;
        }
        // 站点 → 其值所来自的形参槽（checkcast 结果沿用输入值的形参来源，链式到不动点）
        let mut derived: HashMap<u32, SlotMask> = HashMap::default();
        loop {
            let mut changed = false;
            for (off, e) in &a.events {
                let (Event::CheckCast(_, Some(v)) | Event::InstanceOf(_, Some(v))) = e else { continue };
                let s = slots_of(v, &derived);
                if s != 0 {
                    let cur = derived.entry(*off).or_default();
                    if *cur | s != *cur {
                        *cur |= s;
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }
        let mut out: SlotMask = 0;
        for (_, e) in &a.events {
            match e {
                Event::Invoke { opcode, mref, iface, args } => {
                    let Some(site) = self.h.resolve_method(&mref.owner, &mref.name, &mref.desc, *iface) else { continue };
                    let rm = site.method();
                    let virt = matches!(*opcode, op::INVOKEVIRTUAL | op::INVOKEINTERFACE)
                        && !(rm.is_private() || rm.is_static() || rm.is_final() || site.class.access & acc::FINAL != 0 && !site.class.is_interface());
                    if virt {
                        if let Some(r) = args.first() {
                            out |= slots_of(r, &derived);
                        }
                        continue;
                    }
                    // 非虚调用：实参流入被调方的分派转发槽
                    let (o, n, d) = site.key();
                    let callee = MemberRef { owner: o, name: n, desc: d };
                    if callee == *key {
                        continue;
                    }
                    let mask = self.dispatch_slots(&callee);
                    for (j, v) in args.iter().enumerate() {
                        if mask & bit(j as u16) != 0 {
                            out |= slots_of(v, &derived);
                        }
                    }
                }
                Event::Indy { bsm, args, .. } => {
                    let Some(b) = cf.bootstrap_methods.get(*bsm as usize) else { continue };
                    let bkey = format!("{}.{}", b.handle.member.owner, b.handle.member.name);
                    if self.man.indy_kind(&bkey) == Some(IndyKind::Concat) {
                        for v in args {
                            out |= slots_of(v, &derived);
                        }
                    }
                }
                _ => {}
            }
        }
        out
    }
}

/// 值来自的形参槽
fn slots_of(v: &V, derived: &HashMap<u32, SlotMask>) -> SlotMask {
    let V::Ref { src, .. } = v else { return 0 };
    let mut s = 0;
    for x in src.iter() {
        match *x {
            Src::Param(i) => s |= bit(i),
            Src::Site(o) => s |= derived.get(&o).copied().unwrap_or(0),
            _ => {}
        }
    }
    s
}
