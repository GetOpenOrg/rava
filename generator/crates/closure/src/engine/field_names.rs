//! 引擎：按名取字段身份（`[facts.field_writes.name_resolvers]`）的名字实参求值。
//!
//! 名字 → 字段偏移 / setter / VarHandle / 更新器 / 字段句柄，是按名写字段的唯一入口。字段常量折叠要求
//! 每个活调用点的名字值集都被放开：
//! - 常量：点名字段不折叠（与字节码形状规则一致）；
//! - 取自本方法形参：取各调用点在该形参上的字符串常量逐个放开；形参槽出现过非常量实参（或方法无调用点
//!   记录即进入，如 VM / 手写入口）时名字不可知，走保守回退；常量集增长 / 槽被污染时本站点重跑；
//! - 合流的字面量（`Src::Str` 携带字面量编号）：逐个取回放开；
//! - 字段读：String 字段各写入处的字符串常量（字段不折叠或有非常量写入时不可知）；
//! - 调用返回：被调辅助方法的返回常量候选（返回非常量时不可知）；
//! - 类与名字都取自本方法形参：登记字段配对，各调用点按本点实参放开（`lookup_pair.rs`）；
//! - 其它（拼接、数组读等）：名字不可知，走保守回退。
//!
//! 清单即边界：按名取字段身份只经 `name_resolvers`，不按调用形状（Class + String 实参）兜底放开同名字段。
//!
//! 保守回退：字段所属类是类字面量时放开该类及其超类的全部字段，否则全部字段不折叠；返回字段句柄的入口
//! （`handle = true`）按字段枚举处理——句柄写入口（`handle_writers`）可达时才放开。

use super::class_lookup::{event_at, is_invoke};
use super::sealed::is_field;
use super::*;

impl Engine<'_> {
    /// VM 基础设施文件（非共置手写）写访问器点名的字段：同名字段不折叠（handwritten/vm_writes.rs）
    pub fn open_vm_field_write(&mut self, name: &str) {
        self.open_field_name(name);
    }

    pub(super) fn field_name_site(&mut self, m: usize, off: u32, k: &str, opcode: u8, args: &[V]) {
        let Some(r) = self.man.field_name_resolver(k) else { return };
        let base = usize::from(opcode != classfile::op::INVOKESTATIC);
        let cls_arg = r.class.map_or(args.first(), |i| args.get(base + i)).cloned();
        let cls = match &cls_arg {
            Some(V::Class(c, _)) => Some(c.to_string()),
            _ => None,
        };
        let Some(v) = args.get(base + r.name).cloned() else { return };
        if matches!(v, V::Null) {
            return;
        }
        // 引用种类实参是只读种类（getter）的常量：只取读取能力，不放开、不配对
        let kind = r.kind.and_then(|i| args.get(base + i)).and_then(|a| match a {
            V::Int(x) => Some(i64::from(*x)),
            _ => None,
        });
        if r.read_only(kind) {
            return;
        }
        // 类与名字都来自本方法形参（如 `resolveOrFail` 内的 MemberName 构造）：登记字段配对，形参上的名字由各调用点
        // 按本点的类值集 × 名字放开（`lookup_pair.rs`）；返回字段句柄的入口按句柄标记口径，不配对
        let paired = !r.handle && cls_arg.as_ref().is_some_and(|c| self.lookup_wrap_site(m, c, &v, &[], 0, true));
        let (names, known) = self.name_values(m, off, &v, paired);
        for name in &names {
            match cls.as_deref().and_then(|c| self.field_by_name(c, name)) {
                Some((decl, desc)) => {
                    if self.is_static_field(&decl, name) {
                        self.static_field_owner(&decl, Via::method("field-name", m, Some(off)));
                    }
                    self.open_field(MemberRef { owner: decl, name: name.to_string(), desc })
                }
                None => {
                    // 类不是字面量：按 Class 值集所指类解析静态字段的声明类（初始化），字段按名放开；
                    // 值集不齐全时按名兜底（闭包中声明该名静态字段的类都初始化），不记反射缺口
                    if let Some(a) = &cls_arg {
                        let (classes, complete) = self.mirror_classes_of(m, off, k, a, false);
                        if !complete {
                            self.static_owner_name_open(name);
                        }
                        for c in classes {
                            if let Some((decl, _)) = self.field_by_name(&c, name) {
                                if self.is_static_field(&decl, name) {
                                    self.static_field_owner(&decl, Via::method("field-name", m, Some(off)));
                                }
                            }
                        }
                    }
                    self.open_field_name(name)
                }
            }
        }
        if !known {
            // 名字不可知时的口径：类是字面量取该类；否则取 Class 值集所指的类（值集增长时本站点重跑），
            // 值集不齐全（含 open / 推不出的镜像）或无 Class 实参时推不出
            let scopes: Vec<Option<String>> = match (&cls, &cls_arg) {
                (Some(_), _) => vec![cls.clone()],
                (None, Some(a)) => match self.mirror_classes_of(m, off, k, a, false) {
                    (classes, true) => classes.into_iter().map(Some).collect(),
                    (_, false) => vec![None],
                },
                (None, None) => vec![None],
            };
            let statics = !self.man.is_instance_field_user(&self.methods[m].key.to_string());
            for scope in scopes {
                if statics {
                    self.fenum_static.insert(scope.clone());
                }
                if r.handle {
                    // 句柄带本口径的来源标记：流到句柄写入口时才放开（`field_handles.rs`）
                    self.enumerate_fields(scope.clone());
                    self.mark_handle(m, off, k.split_once(':').map_or("", |x| x.1), (false, scope));
                } else {
                    self.open_class_fields(scope);
                }
            }
        }
    }

    /// 名字值 v 的名字集与是否齐全（齐全 = 名字集即终态全集，其增长由读者登记驱动重跑）：
    /// - 字面量（含合流前的各字面量）；常量格给出的字符串常量（`V::derived_str`）按其来源取；
    /// - 形参：各调用点在该形参上的字符串常量，槽被污染时不齐全；`paired`（已登记字段配对）时形参名字由各调用点
    ///   配对放开，此处只登记读者——方法经字节码调用点以外的入口接入时配对看不到这些入口，仍按常量集与污染口径取；
    /// - 字段读：String 字段各写入处的字符串常量（字段不折叠或有非常量写入时不齐全）；
    /// - 调用返回：被调辅助方法的返回常量候选（`callee_consts`，返回非常量时不齐全）；
    /// - 其它来源（数组读、异常值等）不齐全
    pub(super) fn name_values(&mut self, m: usize, off: u32, v: &V, paired: bool) -> (Vec<Rc<str>>, bool) {
        let srcs = v.srcs();
        // 常量格给出的字符串常量（`V::derived_str`）按其来源取：中间态常量不当字面量（来源给出终态全集，
        // 结果与处理次序无关，D1）；无来源的常量即终态值
        let mut names = if srcs.is_empty() { v.lits() } else { v.site_lits() };
        if matches!(v, V::Str(..)) && (!v.derived_str() || srcs.is_empty()) {
            return (names, true);
        }
        let mut known = !srcs.is_empty();
        let a = self.site_analysis(m);
        let offsite = self.pstr_is_offsite(m);
        for s in srcs.iter() {
            match *s {
                Src::Str(_) => {}
                Src::Param(i) => {
                    let ns = self.pstr_read(m, i as usize, off);
                    if !paired || offsite {
                        names.extend(ns);
                        known &= !self.ptaint.contains(&(m, i as usize));
                    }
                }
                Src::Site(o) => match a.as_deref().and_then(|a| self.site_names(a, o)) {
                    Some(xs) => names.extend(xs),
                    None => known = false,
                },
                Src::Catch(_) => known = false,
            }
        }
        names.sort_unstable();
        names.dedup();
        (names, known)
    }

    /// 偏移 o 处产生的名字值的终态全集：String 字段读取该字段各写入处的字符串常量（尚无写入为空集），调用返回取
    /// 被调辅助方法的返回常量候选；推不出时 None
    fn site_names(&mut self, a: &Analysis, o: u32) -> Option<Vec<Rc<str>>> {
        if let Some(Event::Field { opcode, mref, .. }) = event_at(a, o, is_field) {
            if !matches!(*opcode, classfile::op::GETSTATIC | classfile::op::GETFIELD) {
                return None;
            }
            let fi = self.ctx.field_info(mref)?;
            if fi.key.desc != format!("L{};", absint::STRING) || self.ctx.field_open(&fi) {
                return None;
            }
            return match self.field_strs.get(&fi.key) {
                Some(Some(set)) => Some(set.iter().cloned().collect()),
                Some(None) => None,
                None => Some(vec![]),
            };
        }
        event_at(a, o, is_invoke)?;
        self.callee_consts(a, o, 0).map(|s| s.into_iter().collect())
    }

    fn is_static_field(&self, decl: &str, name: &str) -> bool {
        self.h.class(decl).is_some_and(|cf| cf.fields.iter().any(|f| f.name == name && f.access & acc::STATIC != 0))
    }

    /// 形参槽并入实参（vals 不含接收者；None = 实参值未知）：非字符串常量的槽记为污染，重跑读过它的站点
    pub(super) fn taint_params(&mut self, t: usize, base: usize, n: usize, vals: Option<&[PV]>) {
        for i in base..n {
            if !slot_clean(vals.and_then(|vs| vs.get(i - base))) {
                self.taint_slot(t, i);
            }
        }
    }

    /// 字节码调用点 m → t 的形参槽污染：实参是字符串常量 / null，或全部来源是字面量与调用方未污染的形参槽
    /// （透传：其常量集沿子集边到达，见 `pstrs.rs`）时保持干净；调用方槽日后被污染时沿子集边传来
    pub(super) fn taint_site(&mut self, m: usize, t: usize, base: usize, n: usize, vals: &[V]) {
        for i in base..n {
            let clean = vals.get(i - base).is_some_and(|v| {
                slot_clean(Some(&PV::of(v))) || names_known(&v.srcs(), |j| self.ptaint.contains(&(m, j)))
            });
            if !clean {
                self.taint_slot(t, i);
            }
        }
    }

    /// 槽 (t, i) 记为污染：读过它的站点入站点队列重跑，沿形参子集边传给透传的被调槽。
    /// 不在此处同步重跑：污染发生在接边中途（`edge` → `bind_params`），同步重跑会重入调用事件并清掉
    /// 外层调用点的实参值（`call_vals`），外层余下的接边随之丢失形参字符串集
    fn taint_slot(&mut self, t: usize, i: usize) {
        let mut work = vec![(t, i)];
        while let Some((t, i)) = work.pop() {
            if !self.ptaint.insert((t, i)) {
                continue;
            }
            for off in self.pstr_readers(t, i) {
                self.push_site((t, off), site_prof::TRIG_TAINT, None);
            }
            work.extend(self.pstr_succ_methods(t, i));
        }
    }
}

/// 名字值的全部来源都是可取回的字面量或未污染的形参槽（其常量集即名字全集）
pub(super) fn names_known(srcs: &[Src], tainted: impl Fn(usize) -> bool) -> bool {
    !srcs.is_empty()
        && srcs.iter().all(|s| match s {
            Src::Str(_) => true,
            Src::Param(i) => !tainted(*i as usize),
            _ => false,
        })
}

/// 形参槽上的实参不污染名字集：字符串常量（入常量集）或 null（取字段身份时抛异常，不指向任何字段）
fn slot_clean(v: Option<&PV>) -> bool {
    matches!(v, Some(PV::Const(V::Str(..) | V::Null)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_from_clean_params_are_known() {
        assert!(names_known(&[Src::Param(1)], |_| false));
        assert!(names_known(&[Src::Param(1), Src::Param(2)], |i| i == 3));
        // 合流前的字面量可取回
        assert!(names_known(&[Src::Param(1), Src::Str(0)], |_| false));
    }

    #[test]
    fn tainted_or_non_param_sources_are_unknown() {
        assert!(!names_known(&[Src::Param(1)], |i| i == 1));
        // 字段读 / 调用返回、异常值
        assert!(!names_known(&[Src::Site(7)], |_| false));
        assert!(!names_known(&[Src::Catch(3)], |_| false));
        assert!(!names_known(&[], |_| false));
    }

    #[test]
    fn only_string_or_null_arguments_keep_slot_clean() {
        assert!(slot_clean(Some(&PV::Const(V::lit(Rc::from("value"))))));
        assert!(slot_clean(Some(&PV::Const(V::Null))));
        assert!(!slot_clean(Some(&PV::Const(V::Int(1)))));
        assert!(!slot_clean(Some(&PV::Top)));
        // 实参值未知（非字节码调用点 / 无调用点记录的入口）
        assert!(!slot_clean(None));
    }
}
