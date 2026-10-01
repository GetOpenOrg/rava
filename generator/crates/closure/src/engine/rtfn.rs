//! 引擎：手写层调用图——手写体跨文件调用的非 Java 成员 fn（类手写文件里的辅助 fn、模块单元的 fn）。
//!
//! 手写体按「路径类型 / 接收者静态类型 + Rust 名」调用的 fn 若不是 Java 成员，就是另一处手写文件里的
//! Rust fn：Java 类（及其超类型）共置手写文件里的辅助 fn，或运行时模块单元（`handwritten/units.rs`）的
//! fn。每个被调 fn 一个伪方法节点（宿主 + fn 名，无接收者），调用方值池流入被调方值池、被调方产出流回
//! 调用方产出，手写体效果按被调 fn 所在文件建模。同文件调用已在扫描期传递闭包，不再建边。
//! 伪节点不是 Java 成员，不进输出。

use super::*;

/// 运行时 fn 节点的种类标签
pub(super) const RTFN_KIND: &str = "rt-fn";
/// 路径调用上的构造 / 转换关联函数：Java 类型上的这些名字由构造建模处理（`ctors`）或保持身份
const CTOR_LIKE: &[&str] = &["new", "default", "from"];

impl<'a> Engine<'a> {
    /// 手写 fn 节点：宿主 `host`（类或模块单元）的 fn `f`。同键的 VM 钩子节点（带接收者）即同一 fn 体，直接复用
    pub(super) fn rt_fn_node(&mut self, host: &str, f: &str, via: Via) -> usize {
        let key = MemberRef { owner: host.to_string(), name: f.to_string(), desc: "()V".into() };
        if let Some(i) = self.methods.get_index_of(&(key.clone(), NOCTX)) {
            return i;
        }
        let idx = self.methods.len();
        self.methods.insert(
            (key.clone(), NOCTX),
            MNode {
                key: key.clone(),
                kind: Kind::Handwritten(RTFN_KIND),
                via,
                is_static: true,
                ptypes: vec![],
                rtype: None,
                analysis: None,
                applied: None,
                aseq: 0,
                applied_seq: 0,
                returned: None,
                hw_fns: vec![f.to_string()],
                ctx: NOCTX,
                ret_model: RetModel::Plain,
            },
        );
        self.mbase.entry(key).or_insert(idx);
        self.push_m(idx);
        idx
    }

    /// 运行时 fn 节点：产出汇入值池，手写体效果按宿主文件建模
    pub(super) fn process_rt_fn(&mut self, m: usize) {
        let via = Via::method("handwritten", m, None);
        let obj = self.id(OBJECT);
        self.flow(Node::S(m, PROD), Node::S(m, POOL), obj);
        let Some((host, mh)) = self.hw_body(m) else { return };
        for t in self.hw_exports(&host, &mh, None, true) {
            self.flow(Node::S(m, POOL), Node::Esc, t);
        }
        self.apply_hw(m, &host, &mh, &via);
    }

    /// 手写体（宿主 `host`）跨文件调用的手写 fn：建节点并接值池 / 产出
    pub(super) fn hw_fn_calls(&mut self, m: usize, host: &str, mh: &MemberHw) {
        let mut targets: BTreeSet<(String, String)> = BTreeSet::new();
        for c in &mh.calls {
            if let Some(t) = self.hw_fn_target(host, c) {
                targets.insert(t);
            }
        }
        let obj = self.id(OBJECT);
        for (h, f) in targets {
            if h == host {
                continue;
            }
            let t = self.rt_fn_node(&h, &f, Via::method("hw-call", m, None));
            self.flow(Node::S(m, POOL), Node::S(t, POOL), obj);
            self.flow(Node::S(t, PROD), Node::S(m, PROD), obj);
        }
    }

    /// 类自身模块文件是手写单元时（无生成标记，如根类），方法 `X.m` 的方法体是单元里的 base 自由函数
    /// `<T>__<m 的 Rust 名>_base`（`T` 为单元定义的类型）——与生成层 super 调用 / 继承转发的 base 函数同一约定。
    /// 共置手写文件里无对应 fn 的方法按此接入
    pub(super) fn hw_base_fn(&mut self, m: usize, cf: &ClassFile, name: &str, desc: &str) {
        let (pkg, simple) = cf.name.rsplit_once('/').unwrap_or(("", &cf.name));
        let host = if pkg.is_empty() { to_snake(simple) } else { format!("{pkg}/{}", to_snake(simple)) };
        let Some(u) = self.hw.units().get(&host).cloned() else { return };
        let (plain, mangled) = self.rust_names(cf, name, desc);
        let hits: Vec<String> = u
            .types
            .iter()
            .flat_map(|t| plain.iter().chain([&mangled]).map(move |r| format!("{t}__{r}_base")))
            .filter(|f| u.fns.contains_key(f))
            .collect();
        let obj = self.id(OBJECT);
        for f in hits {
            let t = self.rt_fn_node(&host, &f, Via::method("hw-base", m, None));
            self.flow(Node::S(m, POOL), Node::S(t, POOL), obj);
            self.flow(Node::S(t, PROD), Node::S(m, PROD), obj);
        }
    }

    /// 调用点的手写 fn 目标（宿主, fn 名）：Java 类型上不对应 Java 成员的 fn 取其类型层次上首个提供该 fn
    /// 的共置手写文件；非 Java 类型 / 模块路径取定义它的模块单元
    fn hw_fn_target(&self, host: &str, c: &TypedCall) -> Option<(String, String)> {
        let java = |cls: &str| self.hw_class_fn(cls, &c.name, c.args.len()).map(|h| (h, c.name.clone()));
        match (&c.recv, &c.path_ty) {
            (None, Some(t)) => {
                if let Some(cls) = self.resolve_tref(host, t) {
                    if CTOR_LIKE.contains(&c.name.as_str()) || c.name.starts_with("new_") {
                        return None;
                    }
                    return java(&cls);
                }
                self.hw.unit_fn(&self.abs_path(host, &t.0), &c.name).map(|h| (h, c.name.clone()))
            }
            (Some(_), _) => {
                let s = c.srecv.as_ref()?;
                if let Some(cls) = self.stype_class(host, s) {
                    return java(&cls);
                }
                let SType::Named(t) = s else { return None };
                self.hw.unit_fn(&self.abs_path(host, &t.0), &c.name).map(|h| (h, c.name.clone()))
            }
            _ => None,
        }
    }

    /// 类型层次上提供手写 fn `f` 的首个类（`f` 不是该层次上的 Java 方法）
    fn hw_class_fn(&self, cls: &str, f: &str, nargs: usize) -> Option<String> {
        if !self.methods_by_rust_name(cls, f, Some(nargs)).is_empty() || !self.methods_by_rust_name(cls, f, None).is_empty() {
            return None;
        }
        self.supertypes(cls).iter().map(|cf| cf.name.clone()).find(|c| self.hw.class(c).fns.contains_key(f))
    }

    /// 宿主内的类型 / 模块路径 → `crate::…` 绝对路径：已是绝对路径原样；模块单元里的相对路径在单元模块下、
    /// `super::` 逐级上溯；类的共置手写文件是包模块的子模块，首个 `super` 即包
    fn abs_path(&self, host: &str, segs: &[String]) -> Vec<String> {
        if segs.first().is_some_and(|s| s == "crate") {
            return segs.to_vec();
        }
        let k = segs.iter().take_while(|s| *s == "super").count();
        let base: Vec<String> = if self.hw.units().contains_key(host) {
            let module: Vec<String> = if host == CRATE_ROOT { vec![] } else { host.split('/').map(str::to_string).collect() };
            module[..module.len().saturating_sub(k)].to_vec()
        } else {
            if k == 0 {
                return segs.to_vec();
            }
            let pkg: Vec<String> = host.rsplit_once('/').map_or(vec![], |(p, _)| p.split('/').map(str::to_string).collect());
            pkg[..pkg.len().saturating_sub(k - 1)].to_vec()
        };
        ["crate".to_string()].into_iter().chain(base).chain(segs[k..].iter().cloned()).collect()
    }
}
