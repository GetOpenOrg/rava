//! 引擎：VM 引导阶段（seeds.toml `[[boot_init.phases]]`）按锚点作根。
//!
//! 阶段写入的 VM 状态只经锚点字段被读取；闭包内出现锚点字段的读取点，说明程序会观察到该阶段的结果，
//! 阶段即作根，入口形参取清单常量（与调用点常量实参同一通道 `bind_pvs`，VM 是唯一调用方）。
//! 作根单调、不撤回；没有锚点读取点的程序不付阶段的代价。
//!
//! 实例字段锚点按接收者值集判定：类镜像上的锚点字段（如 `Class.module`）只在接收者含命名模块中的类
//! （JDK 类、运行时镜像类）的镜像、所指未知的 Class 对象或 open 时作根——用户类与库类的模块是定义加载器的
//! 无名模块，与引导阶段无关（boot-layer.md 1.2）。非镜像接收者照常作根。

use super::*;

impl<'a> Engine<'a> {
    /// 字段读取点（声明类 `decl`）：命中锚点的阶段作根
    pub(super) fn phase_anchor_read(&mut self, decl: &str, f: &MemberRef) {
        let hit: Vec<usize> = (0..self.man.boot_phases.len())
            .filter(|i| !self.phases_rooted.contains(i))
            .filter(|&i| self.man.boot_phases[i].anchors.iter().any(|(o, n, d)| o == decl && *n == f.name && *d == f.desc))
            .collect();
        for i in hit {
            self.root_phase(i);
        }
    }

    /// 实例字段读取点（接收者值集 s 已按属主过滤）：值集需要阶段结果时按锚点作根
    pub(super) fn phase_anchor_recv(&mut self, decl: &str, f: &MemberRef, s: &TypeSet) {
        if self.phases_rooted.len() == self.man.boot_phases.len() {
            return;
        }
        let xs: Vec<u32> = s.classes.iter().collect();
        let needed = !s.open.is_empty()
            || xs.into_iter().any(|x| match self.mirrors.get(&x).copied() {
                Some(c) => !matches!(self.ctx.cp.origin(&self.names[c as usize]), Some(Origin::User | Origin::Lib)),
                None => true,
            });
        if needed {
            self.phase_anchor_read(decl, f);
        }
    }

    fn root_phase(&mut self, i: usize) {
        self.phases_rooted.insert(i);
        let man = self.man;
        let p = &man.boot_phases[i];
        let Some(key) = seeds::parse_member(&p.call) else {
            self.unresolved.insert(p.call.clone());
            return;
        };
        let via = Via::root("boot_phase", &p.call);
        self.init(&key.owner.clone(), via.clone());
        let m = self.method(key, via);
        let vals: Vec<PV> = p.args.iter().map(|a| PV::of(&V::Int(*a))).collect();
        self.bind_pvs(m, 0, vals.len(), Some(&vals));
        self.returns_to_vm(m);
    }
}
