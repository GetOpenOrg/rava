//! 引擎：反射——类镜像与成员面。

use super::*;

impl<'a> Engine<'a> {
    // ── 反射：类镜像与成员面 ────────────────────────────────────────────────

    /// 值 id 的类型（数组 / 容器对象 / 类镜像 → 其类型）
    pub(super) fn ty(&mut self, x: u32) -> u32 {
        match self.arrays.get(&x).or_else(|| self.objs.get(&x)) {
            Some(&t) => t,
            None if self.mirrors.contains_key(&x) => self.id(CLASS),
            None => x,
        }
    }

    /// 类 cls 的类镜像（Class 对象）
    pub(super) fn mirror(&mut self, cls: &str) -> u32 {
        let name = format!("{CLASS}#{cls}");
        if let Some(&id) = self.ids.get(name.as_str()) {
            return id;
        }
        let c = self.id(cls);
        let id = self.id(&name);
        self.mirrors.insert(id, c);
        id
    }

    /// 值集中各值的类镜像；类型推不出（open、lambda 合成类、手写实现对象）为所指未知的 Class
    pub(super) fn mirror_set(&mut self, s: &TypeSet) -> TypeSet {
        let mut out = TypeSet::default();
        let xs: Vec<u32> = s.classes.iter().copied().collect();
        for x in xs {
            let k = if self.lambdas.contains_key(&x) || self.hwobjs.contains_key(&x) {
                self.id(CLASS)
            } else {
                let t = self.ty(x);
                let n = self.names[t as usize].clone();
                self.mirror(&n)
            };
            out.classes.insert(k);
        }
        if !s.open.is_empty() {
            out.classes.insert(self.id(CLASS));
        }
        out
    }

    /// 成员类别由哪类反射调用执行
    pub(super) fn invoked_by(k: Members) -> Members {
        match k {
            Members::RecordAccessors => Members::Methods,
            k => k,
        }
    }

    /// 成员枚举 e（手写方法）的接收者新增值 s：镜像所指类的该类成员进入反射面；推不出所指类记为缺口
    pub(super) fn enumerate(&mut self, k: Members, e: usize, s: &TypeSet) {
        let xs: Vec<u32> = s.classes.iter().copied().collect();
        for x in xs {
            match self.mirrors.get(&x).copied() {
                Some(c) => {
                    if self.enumerated.insert((k, c)) && self.invokable.contains(&Self::invoked_by(k)) {
                        self.expose(k, c);
                    }
                }
                None => {
                    let what = if self.lambdas.contains_key(&x) { "lambda".to_string() } else { self.names[x as usize].to_string() };
                    self.reflect_gaps.insert(format!("{} <- {what}", self.methods[e].key));
                }
            }
        }
        for &o in &s.open {
            self.reflect_gaps.insert(format!("{} <- open({})", self.methods[e].key, self.names[o as usize]));
        }
    }

    /// 类 cls 的方法被按名 name 查找：方法反射调用可达时补入该名的方法
    pub(super) fn reflect_name(&mut self, cls: &str, name: &str) {
        let c = self.id(cls);
        if !self.reflect_names.entry(c).or_default().insert(name.to_string()) {
            return;
        }
        if self.invokable.contains(&Members::Methods) {
            self.expose(Members::Methods, c);
        }
    }

    /// 类 c 的 k 类成员入链：VM 按反射对象调用，形参按声明类型 open（同 VM 入口）；构造器实例化其类
    pub(super) fn expose(&mut self, k: Members, c: u32) {
        let cls = self.names[c as usize].to_string();
        let Some(cf) = self.h.class(&cls) else { return };
        let comps: Vec<(String, String)> = cf.record_components.clone().unwrap_or_default();
        // 用户类被枚举即全部成员有分派臂；其余类只有按名查找点到的方法（运行时反射分派面同口径：
        // 构造器无按名形状，非用户类的反射构造只经清单补种，如 JCA 服务实现类）
        let user = self.domain(&cls) == Domain::User && self.enumerated.contains(&(k, c));
        let names = self.reflect_names.get(&c).cloned().unwrap_or_default();
        let picked: Vec<(String, String)> = cf
            .methods
            .iter()
            .filter(|mm| match k {
                Members::Methods => !mm.name.starts_with('<') && (user || names.contains(&mm.name)),
                Members::Constructors => mm.name == "<init>" && user,
                Members::RecordAccessors => comps.iter().any(|(n, d)| mm.name == *n && mm.desc == format!("(){d}")),
            })
            .map(|mm| (mm.name.clone(), mm.desc.clone()))
            .collect();
        let via = Via::class("reflect", &cls);
        if k == Members::Constructors && !picked.is_empty() {
            self.instantiate(&cls, via.clone());
        }
        if !picked.is_empty() {
            self.init(&cls, via.clone());
        }
        for (name, desc) in picked {
            let key = MemberRef { owner: cls.clone(), name, desc };
            if !self.reflect_members.insert((k, key.clone())) {
                continue;
            }
            let t = self.method(key, via.clone());
            self.open_params(t);
        }
    }
}
