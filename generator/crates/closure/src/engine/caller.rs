//! 引擎：@CallerSensitive 调用者（`[caller_sensitive] annotations` 与 `[facts.reflect] caller_class`）。
//!
//! 生成器在字节码调用点的被调引用（沿超类链解析声明处）标注 @CallerSensitive 时把调用处所在类压栈
//! （`__caller_sensitive`），`getCallerClass` 返回栈顶——即调用 M 的那个调用点所在类（JVM 语义同：跳过 M
//! 自身的帧）。分析对 M 建一个调用者镜像节点 `S(M, CALLER)`：每条进入 M 的调用边（`edge`）把调用方所在类的
//! 镜像并入，M 体内的 `caller_class` 调用点结果取这个节点（逐方法，不经 `R(getCallerClass)` 汇合）。
//!
//! 压栈与否按生成器同一判据逐调用边判定：边出自字节码调用指令（[`Engine::invoke`] 期间），且该指令的被调引用
//! 解析到 @CallerSensitive 声明（[`Engine::ref_caller_sensitive`]）。其余进入 M 的边不压栈——手写体调用、
//! lambda / 方法引用经 SAM 调用转接、方法句柄调用、indy 辅助、虚调用汇点的后续补边（调用点不在当前指令内）：
//! 运行期 `getCallerClass` 取外层压栈的栈顶，栈空时按栈遍历取调用方帧所在类、帧不可得时取根类。外层栈顶
//! 只能来自某条压栈的 CS 调用边，于是这样的 M 的调用者节点取「全部 CS 调用边的调用方所在类镜像 ∪ 根类镜像」
//! （`CallerState::all`，随新调用边单调增长）。非 CS 方法体内的 `caller_class` 调用（JVM 抛 InternalError）
//! 照旧取返回值节点。
use super::*;

#[derive(Default)]
pub(super) struct CallerState {
    /// 方法序号 → 是否标注 @CallerSensitive（按声明处注解判定的记忆）
    sensitive: HashMap<usize, bool>,
    /// 全部 CS 调用边的调用方所在类镜像 ∪ 根类镜像
    all: TypeSet,
    /// 有不压栈调用边的 CS 方法本体：其调用者节点跟随 `all`
    unwrapped: BTreeSet<usize>,
    /// 当前字节码调用指令的被调引用是否解析到 @CallerSensitive 声明（生成器压栈）；不在调用指令内为 false
    pub(super) site_wrapped: bool,
    /// 被调引用（类, 名, 描述符）→ 是否解析到 @CallerSensitive 声明的记忆
    refs: HashMap<(String, String, String), bool>,
}

impl<'a> Engine<'a> {
    /// 方法 t 是否标注 @CallerSensitive（按声明处注解，与生成器包装口径一致）
    fn is_caller_sensitive(&mut self, t: usize) -> bool {
        if let Some(&b) = self.cs.sensitive.get(&t) {
            return b;
        }
        let key = &self.methods[t].key;
        let b = self.h.class(&key.owner).is_some_and(|cf| {
            cf.method(&key.name, &key.desc)
                .is_some_and(|x| x.annotations.iter().any(|a| self.man.is_caller_sensitive_annotation(&a.type_desc)))
        });
        self.cs.sensitive.insert(t, b);
        b
    }

    /// 被调引用是否解析到 @CallerSensitive 声明：沿超类链找同名同描述符的首个声明，看其注解（与生成器
    /// `caller_sensitive_decl` 同一判据）
    pub(super) fn ref_caller_sensitive(&mut self, mref: &MemberRef) -> bool {
        let key = (mref.owner.to_string(), mref.name.to_string(), mref.desc.to_string());
        if let Some(&b) = self.cs.refs.get(&key) {
            return b;
        }
        let mut cur = self.h.class(&mref.owner);
        let mut b = false;
        let mut hops = 0;
        while let Some(cf) = cur {
            if let Some(x) = cf.method(&mref.name, &mref.desc) {
                b = x.annotations.iter().any(|a| self.man.is_caller_sensitive_annotation(&a.type_desc));
                break;
            }
            hops += 1;
            if hops > 64 {
                break;
            }
            cur = cf.super_name.as_deref().and_then(|s| self.h.class(s));
        }
        self.cs.refs.insert(key, b);
        b
    }

    /// 方法本体序号（克隆上下文共用一个调用者节点）
    fn caller_base(&self, m: usize) -> usize {
        self.mbase.get(&self.methods[m].key).copied().unwrap_or(m)
    }

    /// 调用边 m → t：t 是 @CallerSensitive 方法（`caller_class` 自身除外——生成器不包装它）时，
    /// 压栈的边把调用方所在类的镜像并入 t 的调用者节点；不压栈的边使 t 的调用者节点跟随全集
    pub(super) fn caller_edge(&mut self, m: usize, t: usize) {
        if self.methods[t].ret_model == RetModel::Caller || !self.is_caller_sensitive(t) {
            return;
        }
        let tb = self.caller_base(t);
        let owner = self.methods[m].key.owner.clone();
        let s = TypeSet::exact(self.mirror(&owner));
        if self.cs.all.is_empty() {
            let root = self.mirror(OBJECT);
            self.cs.all.classes.insert(root);
        }
        if self.cs.all.add_all(&s) {
            let all = self.cs.all.clone();
            for b in self.cs.unwrapped.clone() {
                self.add_to(Node::S(b, CALLER), &all);
            }
        }
        if self.methods[m].kind == Kind::Bytecode && self.cs.site_wrapped {
            self.add_to(Node::S(tb, CALLER), &s);
        } else if self.cs.unwrapped.insert(tb) {
            let all = self.cs.all.clone();
            self.add_to(Node::S(tb, CALLER), &all);
        }
    }

    /// `caller_class` 调用点（方法 m 内）的结果：m 是 @CallerSensitive 方法时取 m 的调用者节点，否则取返回值节点
    pub(super) fn caller_ret(&mut self, m: usize, t: usize, res: Node, rt: u32) {
        if self.is_caller_sensitive(m) {
            let mb = self.caller_base(m);
            self.flow(Node::S(mb, CALLER), res, rt);
        } else {
            self.flow(Node::R(t), res, rt);
        }
    }
}
