//! 引擎：VM 钩子节点——类的手写文件里声明回调边、却不对应任何 Java 成员的 pub fn（见 `handwritten/hooks.rs`）。
//!
//! 钩子由生成代码 / 宏在该类对象上直接调用（不经 Java 调用点），故在该类实例化（进 G）时作为入口建模：
//! 每个钩子一个伪方法节点（接收者 = 该类对象，形参未知、经值池），手写体效果（回调 / 分配 / 字段）照常建模。
//! 钩子节点不是 Java 成员，不进输出。

use super::*;

/// 钩子节点的种类标签
pub(super) const VMHOOK_KIND: &str = "vm-hook";

impl<'a> Engine<'a> {
    /// 类 id 进 G：登记其 VM 钩子节点（伪类型与数组无手写文件）
    pub(super) fn vm_hooks_on_alloc(&mut self, id: u32) {
        if self.lambdas.contains_key(&id) || self.hwobjs.contains_key(&id) {
            return;
        }
        let cls = self.names[id as usize].to_string();
        if cls.starts_with('[') {
            return;
        }
        let hwc = self.hw.class(&cls);
        if hwc.fns.is_empty() {
            return;
        }
        let members = self.member_names(&cls);
        for f in hwc.vm_hooks(|f| members.iter().any(|n| member_matches(f, n))) {
            self.vm_hook_node(id, &cls, f);
        }
    }

    /// 类及其全部超类型声明的方法名（钩子判定：继承成员的手写体不是钩子）
    fn member_names(&self, cls: &str) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut stack = vec![cls.to_string()];
        while let Some(c) = stack.pop() {
            if !seen.insert(c.clone()) {
                continue;
            }
            let Some(cf) = self.h.class(&c) else { continue };
            out.extend(cf.methods.iter().map(|m| m.name.clone()));
            stack.extend(cf.super_name.iter().chain(cf.interfaces.iter()).cloned());
        }
        out
    }

    fn vm_hook_node(&mut self, id: u32, cls: &str, f: String) {
        let key = MemberRef { owner: cls.to_string(), name: f.clone(), desc: "()V".into() };
        if self.methods.contains_key(&(key.clone(), NOCTX)) {
            return;
        }
        let idx = self.methods.len();
        self.methods.insert(
            (key.clone(), NOCTX),
            MNode {
                key: key.clone(),
                kind: Kind::Handwritten(VMHOOK_KIND),
                via: Via::class("vm-hook", cls),
                is_static: false,
                ptypes: vec![Some(id)],
                rtype: None,
                analysis: None,
                applied: None,
                returned: None,
                hw_fns: vec![f],
                ctx: NOCTX,
                ret_model: RetModel::Plain,
            },
        );
        self.mbase.entry(key).or_insert(idx);
        self.push_m(idx);
    }

    /// 钩子节点：接收者（该类全部对象）与手写体产出汇入值池，手写体效果按宿主类文件建模
    pub(super) fn process_vm_hook(&mut self, m: usize) {
        let via = Via::method("handwritten", m, None);
        let Some(recv) = self.methods[m].ptypes[0] else { return };
        self.add_to(Node::P(m, 0), &TypeSet::open(recv));
        let obj = self.id(OBJECT);
        self.flow(Node::P(m, 0), Node::S(m, POOL), recv);
        self.flow(Node::S(m, PROD), Node::S(m, POOL), obj);
        let Some((host, mh)) = self.hw_body(m) else { return };
        for t in self.hw_exports(&host, &mh, None, false) {
            self.flow(Node::S(m, POOL), Node::Esc, t);
        }
        self.apply_hw(m, &host, &mh, &via);
    }

    /// 不进输出的伪方法节点：手写实现对象的方法与 VM 钩子
    pub(super) fn is_pseudo_method(&self, t: usize) -> bool {
        matches!(self.methods[t].kind, Kind::Handwritten(HWOBJ_KIND) | Kind::Handwritten(VMHOOK_KIND))
    }
}
