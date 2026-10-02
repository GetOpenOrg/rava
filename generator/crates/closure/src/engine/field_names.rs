//! 引擎：按名取字段身份（`[facts.field_writes.name_resolvers]`）的名字实参求值。
//!
//! 名字 → 字段偏移 / setter / VarHandle / 更新器 / 字段句柄，是按名写字段的唯一入口。字段常量折叠要求
//! 每个活调用点的名字值集都被放开：
//! - 常量：点名字段不折叠（与字节码形状规则一致）；
//! - 取自本方法形参：取各调用点在该形参上的字符串常量逐个放开；形参槽出现过非常量实参（或方法无调用点
//!   记录即进入，如 VM / 手写入口）时名字不可知，走保守回退；常量集增长 / 槽被污染时本站点重跑；
//! - 合流的字面量（`Src::Str` 携带字面量编号）：逐个取回放开；
//! - 其它（字段读 / 调用返回 / 拼接）：名字不可知，走保守回退。
//!
//! 保守回退：字段所属类是类字面量时放开该类及其超类的全部字段，否则全部字段不折叠；返回字段句柄的入口
//! （`handle = true`）按字段枚举处理——句柄写入口（`handle_writers`）可达时才放开。

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
        let Some(v) = args.get(base + r.name) else { return };
        let (names, known) = match v {
            V::Str(s) => (vec![s.clone()], true),
            V::Null => return,
            _ => {
                // 合流前的各字面量（Src::Str 可取回）与形参上流入的字符串常量
                let mut names = v.lits();
                names.extend(self.param_strs(m, off, v));
                (names, names_known(&v.srcs(), |i| self.ptaint.contains(&(m, i))))
            }
        };
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
            if r.handle {
                self.enumerate_fields(cls);
            } else {
                self.open_class_fields(cls);
            }
        }
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

    /// 槽 (t, i) 记为污染：重跑读过它的站点，沿形参子集边传给透传的被调槽
    fn taint_slot(&mut self, t: usize, i: usize) {
        let mut work = vec![(t, i)];
        while let Some((t, i)) = work.pop() {
            if !self.ptaint.insert((t, i)) {
                continue;
            }
            for off in self.pstr_readers(t, i) {
                self.rerun_site(t, off);
            }
            work.extend(self.pstr_succ_methods(t, i));
        }
    }
}

/// 名字值的全部来源都是可取回的字面量或未污染的形参槽（其常量集即名字全集）
fn names_known(srcs: &[Src], tainted: impl Fn(usize) -> bool) -> bool {
    !srcs.is_empty()
        && srcs.iter().all(|s| match s {
            Src::Str(_) => true,
            Src::Param(i) => !tainted(*i as usize),
            _ => false,
        })
}

/// 形参槽上的实参不污染名字集：字符串常量（入常量集）或 null（取字段身份时抛异常，不指向任何字段）
fn slot_clean(v: Option<&PV>) -> bool {
    matches!(v, Some(PV::Const(V::Str(_) | V::Null)))
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
        assert!(slot_clean(Some(&PV::Const(V::Str(Rc::from("value"))))));
        assert!(slot_clean(Some(&PV::Const(V::Null))));
        assert!(!slot_clean(Some(&PV::Const(V::Int(1)))));
        assert!(!slot_clean(Some(&PV::Top)));
        // 实参值未知（非字节码调用点 / 无调用点记录的入口）
        assert!(!slot_clean(None));
    }
}
