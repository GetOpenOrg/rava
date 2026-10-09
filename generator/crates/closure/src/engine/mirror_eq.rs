//! 引擎：引用比较的类镜像答复（`refc == X.class`）。
//!
//! 抽象解释遇到「类字面量 vs 只来自一个 Class 形参的值」的 if_acmp 时问 Oracle：形参值集里的类镜像是否可能是该类。
//! 值集只由镜像组成且不含该类的镜像 → 不等，分支折叠（乐观答复）。答复登记在形参节点上：值集此后新增该类的镜像、
//! 所指未知的 Class 对象（非镜像）或 open 时方法重分析——值集单调增长，不动点时答复与最终值集一致。
//!
//! 同一机制承载形参镜像上的 VM 注入字段读（`Oracle::param_mirror_field`，如 `this.classLoader` 在只含引导类镜像时
//! 为 null）：登记为哨兵类 [`HOOK_FIELD`]，值集新增钩子非空操作的镜像（应用 / 平台类）、所指未知的 Class 对象或 open 时重分析。

use super::*;

/// 值集是否可能含类 c 的镜像：open、所指未知的 Class 对象，或 c 的镜像
fn may_be_mirror(classes: impl IntoIterator<Item = u32>, open: bool, c: u32, mirror: impl Fn(u32) -> Option<u32>, is_class: impl Fn(u32) -> bool) -> bool {
    open || classes.into_iter().any(|x| match mirror(x) {
        Some(t) => t == c,
        None => is_class(x),
    })
}

/// 值集是否可能含钩子非空操作的 Class 对象：open、所指未知的 Class 对象，或非引导类的镜像
fn may_hook(classes: impl IntoIterator<Item = u32>, open: bool, mirror: impl Fn(u32) -> Option<u32>, is_class: impl Fn(u32) -> bool, boot: impl Fn(u32) -> bool) -> bool {
    open || classes.into_iter().any(|x| match mirror(x) {
        Some(t) => !boot(t),
        None => is_class(x),
    })
}

/// 值集里 Class 对象所指的类：含 open 或所指未知的 Class 对象时为 None
fn mirror_targets(classes: impl IntoIterator<Item = u32>, open: bool, mirror: impl Fn(u32) -> Option<u32>, is_class: impl Fn(u32) -> bool) -> Option<BTreeSet<u32>> {
    if open {
        return None;
    }
    let mut out = BTreeSet::new();
    for x in classes {
        match mirror(x) {
            Some(t) => {
                out.insert(t);
            }
            None if is_class(x) => return None,
            None => {}
        }
    }
    Some(out)
}

/// 镜像答复登记的哨兵类：答复是「形参镜像值集上的接收者钩子字段读结果」（见模块注释）
pub(super) const HOOK_FIELD: u32 = u32::MAX;

/// 镜像答复登记的哨兵类：答复取决于整个值集（调用点镜像值集上的实例调用，`Oracle::site_mirror_call`），值集任何增长都重分析
pub(super) const HOOK_ANY: u32 = u32::MAX - 1;

impl<'a> Engine<'a> {
    /// 值 x 是否是 Class 对象（非数组、实例类型为 Class）
    fn is_class_obj(&self, cls: u32, x: u32) -> bool {
        !self.arrays.contains_key(&x) && self.objs.get(&x).copied().unwrap_or(x) == cls
    }

    /// 方法 m 各 Class 形参当前值集所指的类（按形参序号；非 Class 形参 / 所指未知为 None）
    pub(super) fn param_mirror_sets(&mut self, m: usize) -> Vec<Option<BTreeSet<Rc<str>>>> {
        let cls = self.id(CLASS);
        let pts = self.methods[m].ptypes.clone();
        pts.iter()
            .enumerate()
            .map(|(i, pt)| {
                if *pt != Some(cls) {
                    return None;
                }
                self.node_mirror_set(Node::P(m, i as u16))
            })
            .collect()
    }

    /// 节点值集里 Class 对象所指的类（含所指未知的 Class 对象或 open 时为 None）
    pub(super) fn node_mirror_set(&mut self, n: Node) -> Option<BTreeSet<Rc<str>>> {
        let cls = self.id(CLASS);
        let s = self.set_of(n);
        let ts = mirror_targets(s.classes.iter(), !s.open.is_empty(), |x| self.mirrors.get(&x).copied(), |x| self.is_class_obj(cls, x))?;
        Some(ts.into_iter().map(|t| Rc::from(&*self.names[t as usize])).collect())
    }

    /// 节点 n 的值集新增 delta：依赖其「不含某类镜像」答复的方法在答复可能失效时重分析
    pub(super) fn mirror_grown(&mut self, n: Node, delta: &TypeSet) {
        let cls = self.id(CLASS);
        let Some(ws) = self.mirror_watch.get(&n) else { return };
        let hit: Vec<(usize, u32)> = ws
            .iter()
            .copied()
            .filter(|&(_, c)| {
                if c == HOOK_ANY {
                    return true;
                }
                if c == HOOK_FIELD {
                    let boot = |t| self.defining_loader(t) == crate::loaders::Loader::Boot;
                    return may_hook(delta.classes.iter(), !delta.open.is_empty(), |x| self.mirrors.get(&x).copied(), |x| self.is_class_obj(cls, x), boot);
                }
                may_be_mirror(delta.classes.iter(), !delta.open.is_empty(), c, |x| self.mirrors.get(&x).copied(), |x| self.is_class_obj(cls, x))
            })
            .collect();
        if hit.is_empty() {
            return;
        }
        let ws = self.mirror_watch.get_mut(&n).unwrap();
        for w in &hit {
            ws.remove(w);
        }
        for (m, _) in hit {
            self.invalidate(m, Why::Mirror);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 类序号：1 = Class，10 / 11 = 类 A / B 的镜像（所指 20 / 21），30 = 其他对象
    fn mirror(x: u32) -> Option<u32> {
        match x {
            10 => Some(20),
            11 => Some(21),
            _ => None,
        }
    }
    fn is_class(x: u32) -> bool {
        x == 1
    }

    #[test]
    fn mirror_sets_known_only_for_pure_mirrors() {
        assert_eq!(mirror_targets([10, 30], false, mirror, is_class), Some(BTreeSet::from([20])));
        assert_eq!(mirror_targets([10, 1], false, mirror, is_class), None);
        assert_eq!(mirror_targets([10], true, mirror, is_class), None);
        assert_eq!(mirror_targets([], false, mirror, is_class), Some(BTreeSet::new()));
    }

    #[test]
    fn growth_invalidates_only_when_answer_may_change() {
        assert!(!may_be_mirror([10, 30], false, 21, mirror, is_class));
        assert!(may_be_mirror([11], false, 21, mirror, is_class));
        assert!(may_be_mirror([1], false, 21, mirror, is_class));
        assert!(may_be_mirror([], true, 21, mirror, is_class));
    }

    // 类 20 由引导加载器定义，21 不是
    #[test]
    fn hook_answer_invalidates_on_non_boot_mirror() {
        let boot = |t| t == 20;
        assert!(!may_hook([10, 30], false, mirror, is_class, boot));
        assert!(may_hook([11], false, mirror, is_class, boot));
        assert!(may_hook([1], false, mirror, is_class, boot));
        assert!(may_hook([], true, mirror, is_class, boot));
    }
}
