//! 引擎：类初始化触发与方法节点登记（按成员 + 克隆上下文）。

use super::*;

impl<'a> Engine<'a> {
    /// JVMS §5.5 初始化：超类链、声明非抽象实例方法的超接口、`<clinit>`
    pub fn init(&mut self, cls: &str, via: Via) {
        if cut::edges_on() && !cls.starts_with('[') {
            let from = self.via_node(&via);
            cut::edge(&from, &format!("I:{cls}"));
        }
        if cls.starts_with('[') || self.inited.contains_key(cls) {
            return;
        }
        let Some(cf) = self.touch(cls, Level::Init, via.clone()) else { return };
        self.inited.insert(cls.to_string(), via);
        if !cf.is_interface() {
            if let Some(s) = &cf.super_name {
                self.init(s, Via::class("super-init", cls));
            }
            for i in self.h.all_superinterfaces(&cf) {
                if i.methods.iter().any(|m| !m.is_static() && !m.is_abstract() && !m.is_private()) {
                    self.init(&i.name, Via::class("iface-init", cls));
                }
            }
        }
        if cf.method("<clinit>", "()V").is_some() {
            let k = MemberRef { owner: cls.to_string(), name: "<clinit>".into(), desc: "()V".into() };
            self.method(k, Via::class("clinit", cls));
        }
    }

    // ── 方法节点 ────────────────────────────────────────────────────────────

    pub(super) fn kind_of(&self, cf: &ClassFile, m: &classfile::Method) -> Kind {
        self.ctx.kind_of(cf, m)
    }

    /// (类内无重载时的裸名, mangle 名)
    pub(super) fn rust_names(&self, cf: &ClassFile, name: &str, desc: &str) -> (Option<String>, String) {
        self.ctx.rust_names(cf, name, desc)
    }

    pub(super) fn ptype(&mut self, t: &FieldType) -> Option<u32> {
        match t {
            FieldType::Object(c) => Some(self.id(c)),
            other if other.is_reference() => {
                let d = other.descriptor();
                Some(self.id(&d))
            }
            _ => None,
        }
    }

    /// 方法本体节点（无克隆上下文）
    pub(super) fn method(&mut self, key: MemberRef, via: Via) -> usize {
        self.method_ctx(key, NOCTX, via)
    }

    /// 方法节点（按声明类 + 名字 + 描述符 + 克隆上下文）；首次登记入队。只有字节码方法按上下文克隆
    pub(super) fn method_ctx(&mut self, key: MemberRef, ctx: u32, via: Via) -> usize {
        if !self.fwriter_live {
            self.handle_writer_edge(&key, &via);
        }
        self.sysprops_entry(&key, &via);
        self.linked_member(&key, &via);
        if cut::edges_on() {
            let from = self.via_node(&via);
            cut::edge(&from, &format!("M:{key}"));
        }
        let k = (key, ctx);
        if let Some(i) = self.methods.get_index_of(&k) {
            return i;
        }
        let (key, ctx) = k;
        if ctx != NOCTX && self.mbase.get(&key).is_some_and(|&b| self.methods[b].kind != Kind::Bytecode) {
            return self.method_ctx(key, NOCTX, via);
        }
        let (kind, cf, is_static) = match self.h.class(&key.owner) {
            Some(cf) => match cf.method(&key.name, &key.desc) {
                Some(m) => (self.kind_of(&cf, m), Some(cf.clone()), m.is_static()),
                None => (Kind::Missing, None, false),
            },
            None => (Kind::Missing, None, false),
        };
        if ctx != NOCTX && kind != Kind::Bytecode {
            return self.method_ctx(key, NOCTX, via);
        }
        let mut ptypes = Vec::new();
        if !is_static {
            ptypes.push(Some(self.id(&key.owner)));
        }
        let md = parse_method(&key.desc);
        let mut rtype = None;
        if let Some(md) = &md {
            for p in &md.params {
                let t = self.ptype(p);
                ptypes.push(t);
            }
            rtype = md.ret.as_ref().and_then(|r| self.ptype(r));
        }
        let ks = key.to_string();
        let ret_model = if self.man.returns_mirror(&ks) {
            RetModel::Mirror
        } else if self.man.returns_receiver(&ks) {
            RetModel::Receiver
        } else if let Some(src) = self.man.memory_read(&ks) {
            RetModel::Read(src)
        } else {
            RetModel::Plain
        };
        let idx = self.methods.len();
        self.methods.insert(
            (key.clone(), ctx),
            MNode { key: key.clone(), kind, via: via.clone(), is_static, ptypes, rtype, analysis: None, hw_fns: vec![], ctx, ret_model, returned: None, applied: None },
        );
        self.mbase.entry(key.clone()).or_insert(idx);
        if kind == Kind::Bytecode {
            self.nr_created(&key);
        }
        let lvl = if kind == Kind::Bytecode { Level::Code } else { Level::Type };
        self.touch(&key.owner, lvl, Via::method("member", idx, None));
        if cf.is_some() {
            let v = Via::method("signature", idx, None);
            self.touch_desc(&key.desc, &v);
        } else {
            self.unresolved.insert(key.to_string());
        }
        self.push_m(idx);
        if kind != Kind::Missing && (is_static && key.name != "<clinit>" || key.name == "<init>") {
            self.init(&key.owner, via.clone());
        }
        idx
    }

    pub(super) fn push_m(&mut self, m: usize) {
        if self.in_mwork.insert(m) {
            self.mwork.push_back(m);
        }
    }
}
