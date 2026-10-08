//! 系统属性读取：键为拼接值（不是字符串常量）时按拼接段求键。
//!
//! 引擎处理读取点事件时把键值拆成拼接段（`name_parts`，口径同按名取类：推不出的段记为任意串，String 形参段取
//! 各调用点流入的名字），展开成候选模式登记在 `Ctx::pkeys`（方法节点, 键值来源——调用结果站点或形参）；
//! 登记变化时方法失效重算。键为形参时即「各调用点传入不同常量键」的包装方法：分析器的形参常量格只容一个常量，
//! 各键结果相同（如都不在表中）时照样折叠。
//! 重算时 absint 的读取结果按候选模式求值（`Ctx::prop_read_patterns`）：
//! - 全由字面量组成的模式即确定的键，按常量键规则取值；
//! - 含候选集 / 任意串的模式：与表中键（`values` / `dynamic`）及不折叠键都不匹配时，该模式下键启动时不存在、
//!   运行期也未被写入，读取结果为缺省值；匹配到任一键即不折叠；
//! - 各模式的结果相同才折叠。
//!
//! 读取点登记为跨偏移读者（键的拼接链在本方法其它偏移）：重分析时一并重跑，模式随值集增长只会变宽。

use super::class_lookup::{expand, Gap, Part};
use super::method_lookup::parts_match;
use super::name_eval::Frame;
use super::sysprops::{DefArg, PropSum};
use super::*;

/// 读取点键值的候选模式
pub(super) type KeyPats = Rc<[Vec<Part>]>;

/// 键值的唯一来源（调用结果站点或形参）
pub(super) fn key_src(v: &V) -> Option<Src> {
    match (v, &*v.srcs()) {
        (V::Ref { .. }, [s @ (Src::Site(_) | Src::Param(_))]) => Some(*s),
        _ => None,
    }
}

/// 由字面量组成的模式拼成的键
fn literal(p: &[Part]) -> Option<String> {
    p.iter()
        .map(|x| match x {
            Part::Lit(s) => Some(&**s),
            _ => None,
        })
        .collect::<Option<Vec<_>>>()
        .map(|v| v.concat())
}

impl Ctx<'_> {
    /// 键为拼接值的读取点（已登记候选模式）的折叠值；`me` = 外层被分析的方法节点
    pub(super) fn prop_read_patterns(&self, me: Option<usize>, spec: &PropSum, args: &[V]) -> Option<Ret> {
        let me = me?;
        let o = key_src(args.get(spec.key)?)?;
        let pats = self.pkeys.borrow().get(&(me, o)).cloned()?;
        let mut out: Option<V> = None;
        for p in pats.iter() {
            let v = match literal(p) {
                Some(k) => {
                    let mut a = args.to_vec();
                    a[spec.key] = V::lit(k.as_str());
                    let Some(Ret::Value(v)) = self.prop_read(Some(me), spec, &a) else { return None };
                    v
                }
                None => self.absent_read(me, spec, args, p)?,
            };
            if out.as_ref().is_some_and(|x| *x != v) {
                return None;
            }
            out = Some(v);
        }
        out.map(Ret::Value)
    }

    /// 模式 p 不匹配任何表中键 / 不折叠键时的读取结果（缺省值）
    fn absent_read(&self, me: usize, spec: &PropSum, args: &[V], p: &[Part]) -> Option<V> {
        if spec.receiver && !args.first().is_some_and(|v| v.obj().is_some_and(|o| **o == Obj::SysProps)) {
            return None;
        }
        if !spec.snapshot {
            self.note_props(Some(me));
            let u = self.punstable.borrow();
            if u.all || u.keys.iter().any(|k| parts_match(p, k)) {
                return None;
            }
        }
        let sp = &self.man.sysprops;
        if sp.values().keys().chain(sp.dynamic().iter()).any(|k| parts_match(p, k)) {
            return None;
        }
        match &spec.default {
            DefArg::None => Some(V::Null),
            DefArg::Const(v) => Some(v.clone()),
            DefArg::Param(i) => match args.get(*i)? {
                v @ (V::Str(..) | V::Null) => Some(v.clone()),
                _ => None,
            },
        }
    }
}

impl Engine<'_> {
    /// 读取点（方法 m、偏移 off）的键为拼接值：求候选模式并登记；登记变化时方法失效重算
    pub(super) fn prop_key_site(&mut self, m: usize, off: u32, opcode: u8, mref: &MemberRef, iface: bool, args: &[V]) {
        if self.man.sysprops.is_empty() || opcode == classfile::op::INVOKEDYNAMIC {
            return;
        }
        let Some(spec) = self.ctx.read_spec(None, opcode, mref, iface, None) else { return };
        let Some(key) = args.get(spec.key) else { return };
        let Some(o) = key_src(key) else { return };
        self.xreaders.entry(m).or_default().insert(off);
        let Some(a) = self.methods[m].analysis.clone() else { return };
        let pats: Option<KeyPats> = if a.conservative {
            None
        } else {
            let owner = self.methods[m].key.owner.clone();
            let f = Frame { m: Some(m), a: &a, owner: &owner, up: None };
            self.name_parts(&f, key, Gap::Class, 0).and_then(|p| expand(&p)).map(Rc::from)
        };
        // 首次登记同样触发重算：登记前该读取点答复 ⊥（`sysprops.rs` derived_result）
        let first = self.ctx.pkeys_seen.borrow_mut().insert((m, o));
        let changed = {
            let mut pk = self.ctx.pkeys.borrow_mut();
            match pats {
                Some(p) => pk.insert((m, o), p.clone()).is_none_or(|old| old != p),
                None => pk.remove(&(m, o)).is_some(),
            }
        };
        if first || changed {
            self.pkey_dirty.insert(m);
        }
    }

    /// 候选模式有变化的方法失效重算（不在处理该方法的中途失效：其余站点仍按当前分析求值）
    pub(super) fn pkey_flush(&mut self) {
        if self.pkey_dirty.is_empty() {
            return;
        }
        for m in std::mem::take(&mut self.pkey_dirty) {
            self.invalidate(m, Why::Sysprops);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_patterns_concat_and_others_do_not() {
        let lit = |s: &str| Part::Lit(Rc::from(s));
        assert_eq!(literal(&[lit("jdk.module.addmods."), lit("0")]).as_deref(), Some("jdk.module.addmods.0"));
        assert_eq!(literal(&[]).as_deref(), Some(""));
        assert_eq!(literal(&[Part::Wild, lit("0")]), None);
        assert_eq!(literal(&[lit("a"), Part::Any([Rc::from("b")].into_iter().collect())]), None);
        // 含任意串的模式：表中键匹配与否决定能否按「启动时不存在」折叠
        let p = [Part::Wild, lit("0")];
        assert!(!parts_match(&p, "jdk.module.path"));
        assert!(parts_match(&p, "jdk.module.addmods.0"));
    }
}
