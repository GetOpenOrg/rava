//! 引擎：键载体类——实例字段经读取方法流入按键查找键的类（计划 c1d §31.3 终态能力 ④-1）。
//!
//! 按键查找（`[facts.keyed_lookups]`）的键实参若是某个对象上读取方法的返回值（`uri.toString()` 作 URL 串、其协议即
//! 查找键），键随该对象的字段而定。这类对象不按分配点分开时，读取方法读的是字段的全局值集，所有对象的键汇合成
//! 任意键，查找放开到全部服务类。键载体类因此按容器形态类处理（`classes.rs::container_shape`），对象按分配点分开，
//! 其读取方法按接收者对象克隆、返回值按对象归属（`obj_rets.rs`），守卫查找的条件随之可判。
//!
//! 判定只看字节码，不列类名：
//! - **键形参**：清单入口的键形参（`key`），协议键站点方法的 URL 串形参（`scheme_sites`）；
//! - **扫描范围**：键形参所属方法的声明类（按键查找的包装调用集中在这些类内）。范围由清单与字节码定出，与分析顺序无关；
//! - 扫描范围内的方法调用键形参方法时，键实参来自本方法形参 → 该形参也是键形参（范围内求不动点）；
//!   来自无实参实例调用 `x.g()` 的返回值、且 g（按 x 的静态类型沿超类解析到的实现）的字节码读取 g 声明类（或其超类）的
//!   实例字段 → x 的静态类型是键载体类。
//!
//! 判定在首次查询时一次求出、缓存，结果只取决于清单与类文件，与处理顺序无关。

use super::*;

/// 扫描用的空事实 Oracle：调用结果、字段值一律未知，类型一律可能有实例（全部分支可达，只取实参来源）
struct NoFacts;

impl Oracle for NoFacts {
    fn invoke_result(&self, _: u8, _: u32, _: &MemberRef, _: bool, _: &[V]) -> Ret {
        Ret::Unknown
    }
    fn field(&self, _: u8, _: &MemberRef, _: Option<&V>) -> Option<V> {
        None
    }
    fn type_live(&self, _: &str) -> bool {
        true
    }
}

impl Engine<'_> {
    /// 键载体类（内部名，有序）
    pub(super) fn key_carriers(&mut self) -> Rc<BTreeSet<String>> {
        if let Some(c) = &self.keyed.carriers {
            return c.clone();
        }
        let c = Rc::new(self.key_carriers_scan());
        self.keyed.carriers = Some(c.clone());
        c
    }

    fn key_carriers_scan(&self) -> BTreeSet<String> {
        let man = self.man;
        // 键形参：成员键（`类.方法:描述符`）→ 形参序号集（按描述符，0 起，不含接收者）
        let mut params: BTreeMap<String, BTreeSet<usize>> = BTreeMap::new();
        for member in man.keyed_lookups.members() {
            let Some(spec) = man.keyed_lookups.get(member) else { continue };
            params.entry(member.to_string()).or_default().insert(spec.key);
            for (site, &j) in &spec.scheme_sites {
                params.entry(site.clone()).or_default().insert(j);
            }
        }
        let owners: BTreeSet<String> = params.keys().filter_map(|k| k.split_once(':').and_then(|(h, _)| h.rsplit_once('.')).map(|(c, _)| c.to_string())).collect();
        // 扫描范围内各方法的调用事件（一次分析，不动点迭代复用）
        // (方法键, 是否静态, 声明类, 方法序号, 调用事件)
        let mut scans: Vec<(String, bool, std::sync::Arc<ClassFile>, usize, Vec<Event>)> = Vec::new();
        for o in &owners {
            let Some(cf) = self.h.class(o) else { continue };
            for (i, mm) in cf.methods.iter().enumerate() {
                let Some(code) = &mm.code else { continue };
                let a = absint::analyze(&cf.name, &mm.desc, mm.is_static(), code, &NoFacts);
                if a.conservative {
                    continue;
                }
                let evs: Vec<Event> = a.events.into_iter().map(|(_, e)| e).filter(|e| matches!(e, Event::Invoke { .. })).collect();
                let key = format!("{}.{}:{}", cf.name, mm.name, mm.desc);
                scans.push((key, mm.is_static(), cf.clone(), i, evs));
            }
        }
        let mut carriers: BTreeSet<String> = BTreeSet::new();
        loop {
            let mut grown = false;
            for (me, is_static, cf, mi, evs) in &scans {
                let Some(code) = &cf.methods[*mi].code else { continue };
                for e in evs {
                    let Event::Invoke { opcode, mref, args, .. } = e else { continue };
                    let Some(idx) = params.get(&mref.to_string()) else { continue };
                    let skip = usize::from(*opcode != classfile::op::INVOKESTATIC);
                    for &k in idx.clone().iter() {
                        let Some(v) = args.get(skip + k) else { continue };
                        for s in v.srcs().iter() {
                            match *s {
                                Src::Param(i) => {
                                    // 形参序号含接收者槽；换成按描述符的序号
                                    let Some(j) = (i as usize).checked_sub(usize::from(!*is_static)) else { continue };
                                    grown |= params.entry(me.clone()).or_default().insert(j);
                                }
                                Src::Site(off) => {
                                    if let Some(c) = self.getter_carrier(code, off) {
                                        carriers.insert(c);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            if !grown {
                break;
            }
        }
        carriers
    }

    /// 偏移 off 处是无实参实例调用 `x.g()`、g 的字节码读取其声明类（或超类）的实例字段时，返回 x 的静态类型
    fn getter_carrier(&self, code: &classfile::Code, off: u32) -> Option<String> {
        let ins = code.insns.iter().find(|x| x.offset == off)?;
        let classfile::Operand::Method(r, _) = &ins.operand else { return None };
        if ins.opcode != classfile::op::INVOKEVIRTUAL || !r.desc.starts_with("()L") {
            return None;
        }
        let impl_cf = self.h.superclasses(&r.owner).into_iter().find(|cf| cf.method(&r.name, &r.desc).is_some_and(|mm| mm.code.is_some()))?;
        let mm = impl_cf.method(&r.name, &r.desc)?;
        let mine: Vec<String> = self.h.superclasses(&impl_cf.name).iter().map(|cf| cf.name.clone()).collect();
        let reads = mm.code.as_ref()?.insns.iter().any(|x| {
            x.opcode == classfile::op::GETFIELD && matches!(&x.operand, classfile::Operand::Field(f) if mine.contains(&f.owner))
        });
        reads.then(|| r.owner.clone())
    }
}
