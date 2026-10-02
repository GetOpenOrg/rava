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

    /// 类型序号 t 的类镜像（按类型序号记忆）
    fn mirror_id(&mut self, t: u32) -> u32 {
        if let Some(&k) = self.mirror_of.get(t as usize).filter(|&&k| k != u32::MAX) {
            return k;
        }
        let n = self.names[t as usize].clone();
        let k = self.mirror(&n);
        let ti = t as usize;
        if self.mirror_of.len() <= ti {
            self.mirror_of.resize(ti + 1, u32::MAX);
        }
        self.mirror_of[ti] = k;
        k
    }

    /// 值 x 的类镜像；类型推不出（lambda 合成类、手写实现对象）为所指未知的 Class
    fn value_mirror(&mut self, x: u32) -> u32 {
        if self.lambdas.contains_key(&x) || self.hwobjs.contains_key(&x) {
            self.id(CLASS)
        } else {
            let t = self.ty(x);
            self.mirror_id(t)
        }
    }

    /// open 值可取的对象：G 中 ⊂ o 的成员（同 `receivers` 的 open 展开口径；数组须已逃逸）
    fn open_members(&mut self, o: u32) -> Vec<u32> {
        let xs = self.g_of(o);
        xs.iter().copied().filter(|x| !self.arrays.contains_key(x) || self.escaped.contains(x)).collect()
    }

    /// 镜像流边的变换：op 作用于值集 s，结果并入 dst
    pub(super) fn mirror_op_into(&mut self, op: MirrorOp, s: &TypeSet, dst: Node) {
        match op {
            MirrorOp::Of => self.mirrors_into(s, dst),
            MirrorOp::Super => {
                let k = self.super_set(s);
                self.add_to(dst, &k);
            }
        }
    }

    /// Class 值集中各类镜像所指类的直接超类镜像（`getSuperclass`）：接口与根类无超类（null，不入结果），
    /// 数组的超类是根类；所指未知的 Class（非镜像值、类文件缺失）给所指未知的 Class，open 仍为 open。
    /// 镜像只由类字面量与 `getClass` 产生，不指向基本类型
    pub(super) fn super_set(&mut self, s: &TypeSet) -> TypeSet {
        let class = self.id(CLASS);
        let mut out = TypeSet::default();
        let xs: Vec<u32> = s.classes.iter().collect();
        for x in xs {
            let Some(&c) = self.mirrors.get(&x) else {
                out.classes.insert(class);
                continue;
            };
            let name = self.names[c as usize].clone();
            // None = 类文件缺失（所指未知）；Some(None) = 无超类（null）
            let sup = if name.starts_with('[') {
                Some(Some(OBJECT.to_string()))
            } else {
                self.h.class(&name).map(|cf| if cf.is_interface() { None } else { cf.super_name.clone() })
            };
            match sup {
                Some(Some(sc)) => {
                    let k = self.mirror(&sc);
                    out.classes.insert(k);
                }
                Some(None) => {}
                None => {
                    out.classes.insert(class);
                }
            }
        }
        if !s.open.is_empty() {
            out.open.insert(class);
        }
        out
    }

    /// 值集 s 中各值的类镜像并入 dst。open(o) 按 G 中 ⊂ o 的成员展开并登记，G 增长时由
    /// `reopen_mirrors` 补推新成员的镜像（闭世界：open 值只能是 G 中已分配的对象）
    pub(super) fn mirrors_into(&mut self, s: &TypeSet, dst: Node) {
        let mut out = TypeSet::default();
        for x in s.classes.iter() {
            let k = self.value_mirror(x);
            out.classes.insert(k);
        }
        for o in s.open.iter() {
            if !self.open_mirrors.entry(o).or_default().insert(dst) {
                continue;
            }
            for x in self.open_members(o) {
                let k = self.value_mirror(x);
                out.classes.insert(k);
            }
        }
        self.add_to(dst, &out);
    }

    /// G 新成员 / 新逃逸数组 x：登记过的 open 镜像展开补入 x 的镜像
    pub(super) fn reopen_mirrors(&mut self, x: u32) {
        if self.arrays.contains_key(&x) && !self.escaped.contains(&x) {
            return;
        }
        let os: Vec<u32> = self.open_mirrors.keys().copied().collect();
        let mut dsts: BTreeSet<Node> = BTreeSet::new();
        for o in os {
            if self.sub(x, o) {
                dsts.extend(self.open_mirrors[&o].iter().copied());
            }
        }
        if dsts.is_empty() {
            return;
        }
        let k = TypeSet::exact(self.value_mirror(x));
        for d in dsts {
            self.add_to(d, &k);
        }
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
        let xs: Vec<u32> = s.classes.iter().collect();
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
        for o in &s.open {
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
        // 按名取类解析到的类（常量名拼出的具体类）同样按枚举给出构造器
        let named = k == Members::Constructors && self.named_ctors.contains(&c);
        let user = (self.domain(&cls) == Domain::User || named) && self.enumerated.contains(&(k, c));
        let names = self.reflect_names.get(&c).cloned().unwrap_or_default();
        // 可被覆写的实例方法：反射调用按接收者虚分派（覆写可在子类，含未枚举的类）
        let overridable = |mm: &classfile::Method| reflect_virtual(mm.access, cf.access);
        let picked: Vec<(String, String, bool)> = cf
            .methods
            .iter()
            .filter(|mm| match k {
                Members::Methods => !mm.name.starts_with('<') && (user || names.contains(&mm.name)),
                Members::Constructors => mm.name == "<init>" && user,
                Members::RecordAccessors => comps.iter().any(|(n, d)| mm.name == *n && mm.desc == format!("(){d}")),
            })
            .map(|mm| (mm.name.clone(), mm.desc.clone(), k != Members::Constructors && overridable(mm)))
            .collect();
        let via = Via::class("reflect", &cls);
        if k == Members::Constructors && !picked.is_empty() {
            self.instantiate(&cls, via.clone());
        }
        if !picked.is_empty() {
            self.init(&cls, via.clone());
        }
        for (name, desc, virt) in picked {
            let key = MemberRef { owner: cls.clone(), name, desc };
            if !self.reflect_members.insert((k, key.clone())) {
                continue;
            }
            if virt {
                self.vm_dispatch(&key, cf.is_interface(), via.clone());
            }
            let t = self.method(key, via.clone());
            self.open_params(t);
        }
    }
}

/// 反射调用该方法按接收者虚分派（可被覆写的实例方法：非 static / private / final，所属类非 final）
fn reflect_virtual(method_access: u16, class_access: u16) -> bool {
    method_access & (acc::STATIC | acc::PRIVATE | acc::FINAL) == 0 && class_access & acc::FINAL == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reflect_virtual_rule() {
        assert!(reflect_virtual(acc::PUBLIC, acc::PUBLIC));
        assert!(reflect_virtual(0, acc::ABSTRACT));
        assert!(!reflect_virtual(acc::STATIC, 0));
        assert!(!reflect_virtual(acc::PRIVATE, 0));
        assert!(!reflect_virtual(acc::FINAL, 0));
        assert!(!reflect_virtual(0, acc::FINAL));
    }
}
