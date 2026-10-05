//! 引擎：反射——类镜像与成员面。

use super::reflect_call::{rc_bit, VmBind, RC_OBJ};
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

    /// 镜像流边的变换：op 作用于值集 s
    fn mirror_op(&mut self, op: MirrorOp, s: &TypeSet) -> TypeSet {
        match op {
            MirrorOp::Of => self.mirror_set(s),
            MirrorOp::Super => self.super_set(s),
            MirrorOp::Component => self.component_set(s),
            MirrorOp::Declaring => self.declaring_set(s),
            MirrorOp::Sub(k) => self.sub_mirrors(s, k),
            MirrorOp::ArrayOf(m, off) => self.array_of_set(m as usize, off, s),
        }
    }

    /// Class 值集中各类镜像所指类型的新数组（`Array.newInstance(c, n)`，元素为缺省值 null）：调用点 (m, off) 上
    /// 按数组类型区分的分配点，元素只来自其后的写入。基本类型类镜像给各基本类型数组（void 抛异常，无结果）；
    /// 维数已达上限的数组类镜像抛异常（JVMS §4.4.1 数组至多 255 维；Array.newInstance 同），无结果。
    /// 所指未知的 Class（open、非镜像值、非字节码类镜像）给 open(Object)，该调用点此后不再逐类型建分配点
    fn array_of_set(&mut self, m: usize, off: u32, s: &TypeSet) -> TypeSet {
        let obj = self.id(OBJECT);
        let unknown = !s.open.is_empty()
            || s.classes.iter().any(|x| Some(x) != self.prim_mirror && (Some(x) == self.synth_mirror || !self.mirrors.contains_key(&x)));
        if unknown {
            self.array_of_open.insert((m, off));
        }
        if self.array_of_open.contains(&(m, off)) {
            return TypeSet::open(obj);
        }
        let mut out = TypeSet::default();
        let xs: Vec<u32> = s.classes.iter().collect();
        for x in xs {
            let ts: Vec<String> = if Some(x) == self.prim_mirror {
                b"ZCFDBSIJ".iter().map(|&c| format!("[{}", c as char)).collect()
            } else {
                let name = &self.names[self.mirrors[&x] as usize];
                if name.bytes().take_while(|&b| b == b'[').count() >= MAX_ARRAY_DIMS {
                    continue;
                }
                vec![if name.starts_with('[') { format!("[{name}") } else { format!("[L{name};") }]
            };
            for t in ts {
                let id = self.array_site(m, off, &t, false, Via::method("reflect-newarray", m, Some(off)));
                out.classes.insert(id);
            }
        }
        out
    }

    /// Class 值集中所指类 ⊂ k 的类镜像（`K.class.isAssignableFrom(x)` 成立一侧）：所指类已知者按子类型判定取舍；
    /// 基本类型类镜像所指不是引用类型（判定恒假）去掉；非字节码类镜像（lambda 合成类可实现任意接口）、所指未知的
    /// Class 与 open 保留
    fn sub_mirrors(&mut self, s: &TypeSet, k: u32) -> TypeSet {
        let mut out = TypeSet { classes: IdSet::default(), open: s.open.clone() };
        let xs: Vec<u32> = s.classes.iter().collect();
        let kept: Vec<u32> = xs
            .into_iter()
            .filter(|&x| match self.mirrors.get(&x) {
                Some(&c) => self.sub(c, k),
                None => Some(x) != self.prim_mirror,
            })
            .collect();
        out.classes = IdSet::from_sorted(kept);
        out
    }

    /// Class 值集中各类镜像所指类的直接超类镜像（`getSuperclass`）：接口与根类无超类（null，不入结果），
    /// 数组与非字节码类镜像的超类是根类，基本类型类镜像的超类为 null；所指未知的 Class（非镜像值、类文件缺失）给所指
    /// 未知的 Class，open 仍为 open
    pub(super) fn super_set(&mut self, s: &TypeSet) -> TypeSet {
        let class = self.id(CLASS);
        let mut out = TypeSet::default();
        let xs: Vec<u32> = s.classes.iter().collect();
        for x in xs {
            if Some(x) == self.synth_mirror {
                let k = self.mirror(OBJECT);
                out.classes.insert(k);
                continue;
            }
            // 基本类型类的超类为 null（JLS §15.8.2 / Class.getSuperclass 规范）
            if Some(x) == self.prim_mirror {
                continue;
            }
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

    /// Class 值集中各类镜像所指类的声明类镜像（`getDeclaringClass0`，JVMS §4.7.6：本类 InnerClasses 中以本类为
    /// inner 的条目的 outer；顶层 / 局部 / 匿名类、数组、基本类型与非字节码类镜像为 null，不入结果）；所指未知的
    /// Class（非镜像值、类文件缺失）给所指未知的 Class，open 仍为 open
    fn declaring_set(&mut self, s: &TypeSet) -> TypeSet {
        let class = self.id(CLASS);
        let mut out = TypeSet::default();
        let xs: Vec<u32> = s.classes.iter().collect();
        for x in xs {
            if Some(x) == self.synth_mirror || Some(x) == self.prim_mirror {
                continue;
            }
            let Some(&c) = self.mirrors.get(&x) else {
                out.classes.insert(class);
                continue;
            };
            let name = self.names[c as usize].clone();
            if name.starts_with('[') {
                continue;
            }
            let Some(cf) = self.h.class(&name) else {
                out.classes.insert(class);
                continue;
            };
            let outer = cf.inner_classes.iter().find(|ic| ic.inner == *name).and_then(|ic| ic.outer.clone());
            if let Some(o) = outer {
                let k = self.mirror(&o);
                out.classes.insert(k);
            }
        }
        if !s.open.is_empty() {
            out.open.insert(class);
        }
        out
    }

    /// Class 值集中各数组类镜像的元素类型镜像（`getComponentType`，JVMS §4.3.2 描述符去一维）：引用 / 数组元素给其
    /// 类镜像，基本类型元素给基本类型类镜像；非数组类、非字节码类与基本类型类镜像为 null（不入结果）；所指未知的
    /// Class 给所指未知的 Class，open 仍为 open
    fn component_set(&mut self, s: &TypeSet) -> TypeSet {
        let class = self.id(CLASS);
        let mut out = TypeSet::default();
        let xs: Vec<u32> = s.classes.iter().collect();
        for x in xs {
            if Some(x) == self.synth_mirror || Some(x) == self.prim_mirror {
                continue;
            }
            let Some(&c) = self.mirrors.get(&x) else {
                out.classes.insert(class);
                continue;
            };
            let name = self.names[c as usize].clone();
            let Some(rest) = name.strip_prefix('[') else { continue };
            let k = if rest.starts_with('[') {
                self.mirror(rest)
            } else if let Some(e) = rest.strip_prefix('L').and_then(|r| r.strip_suffix(';')) {
                self.mirror(e)
            } else {
                self.primitive_mirror()
            };
            out.classes.insert(k);
        }
        if !s.open.is_empty() {
            out.open.insert(class);
        }
        out
    }

    /// 值集中各值的类镜像（`getClass`）。open(T) 是「任意已实例化的 T 子类型」，其类镜像是 G 中 T 的子类型
    /// （数组分配点须已逃逸，同虚调用接收者的 open 展开）各自的镜像；G 增长时由 [`Self::mirror_into`] 登记的
    /// 结果节点补入（[`Self::mirror_reopen`]）。lambda 合成类、手写实现对象给非字节码类镜像（[`Self::synthetic_mirror`]）
    pub(super) fn mirror_set(&mut self, s: &TypeSet) -> TypeSet {
        let mut out = TypeSet::default();
        let xs: Vec<u32> = s.classes.iter().collect();
        for x in xs {
            let k = self.value_mirror(x);
            out.classes.insert(k);
        }
        for o in s.open.iter() {
            for &x in self.g_of(o).iter() {
                if self.arrays.contains_key(&x) && !self.escaped.contains(&x) {
                    continue;
                }
                let k = self.value_mirror(x);
                out.classes.insert(k);
            }
        }
        out
    }

    /// 值 x 的类镜像：lambda / 手写实现对象的运行期类不是字节码类，为共用的非字节码类镜像
    fn value_mirror(&mut self, x: u32) -> u32 {
        if self.lambdas.contains_key(&x) || self.hwobjs.contains_key(&x) {
            return self.synthetic_mirror();
        }
        let t = self.ty(x);
        self.mirror_id(t)
    }

    /// 非字节码类（lambda 合成类、手写实现对象）的类镜像：一个 Class 类型的抽象对象（Class 实例字段按对象建模）。
    /// 不登记为镜像——它不指向任何字节码类：成员查找与枚举照所指未知处理；类初始化跳过（无 `<clinit>`）；
    /// 字段查找与字段枚举视为所指已知且无 Java 字段（合成类的捕获字段不经反射写入）；超类是根类
    pub(super) fn synthetic_mirror(&mut self) -> u32 {
        if let Some(k) = self.synth_mirror {
            return k;
        }
        let class = self.id(CLASS);
        let k = self.id(&format!("{CLASS}#<synthetic>"));
        self.objs.insert(k, class);
        self.synth_mirror = Some(k);
        k
    }

    /// 镜像流边推送：s 经变换 op 并入 dst。`getClass` 作用于 open(T) 时登记 dst，
    /// T 的已实例化子类型此后进入 G（或数组逃逸）时补入其镜像
    pub(super) fn mirror_into(&mut self, op: MirrorOp, s: &TypeSet, dst: Node) {
        if op == MirrorOp::Of {
            for o in s.open.iter() {
                if self.mirror_open_seen.insert((o, dst)) {
                    self.mirror_open.entry(o).or_default().push(dst);
                }
            }
        }
        let k = self.mirror_op(op, s);
        self.add_to(dst, &k);
    }

    /// open 展开的取值面扩大（x 进入 G / 数组 x 逃逸）：`getClass(open T)`（x ⊂ T）的结果节点补入 x 的类镜像
    pub(super) fn mirror_reopen(&mut self, x: u32) {
        if self.mirror_open.is_empty() || (self.arrays.contains_key(&x) && !self.escaped.contains(&x)) {
            return;
        }
        let os: Vec<u32> = self.mirror_open.keys().copied().collect();
        let mut dsts: Vec<Node> = vec![];
        for o in os {
            if self.sub(x, o) {
                dsts.extend(self.mirror_open[&o].iter().copied());
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

    /// 类 cls 的方法被按名 name 查找、查找结果经反射调用通道 ch 调用：方法反射调用可达时补入该名的方法
    pub(super) fn reflect_name(&mut self, cls: &str, name: &str, ch: u8) {
        let c = self.id(cls);
        let e = self.reflect_names.entry(c).or_default().entry(name.to_string()).or_default();
        if *e & rc_bit(ch) != 0 {
            return;
        }
        *e |= rc_bit(ch);
        if self.invokable.contains(&Members::Methods) {
            self.expose(Members::Methods, c);
        }
    }

    /// 构造器查找（`[facts.reflect] constructor_lookups`）：Class 实参与 Class 接收者值集里类镜像所指的类，其构造器
    /// 成为反射构造目标（构造器没有按名形状，查找即点名该类的构造器；同按名取类解析到的类），反射构造可达时入链。
    /// 只有值集齐全（全是推得出的镜像、无 open）时查找目标才确定、才点名；值集不齐全时运行期目标可以是值集之外的
    /// 任意类，已知部分只是流不敏感合流带进来的镜像（如 `Objects.requireNonNull` 返回值），不比其余类更可信——
    /// 此时同构造器枚举：所指类记为已枚举（用户类照常有分派臂），JDK 类不点名，查找点记为反射缺口。
    /// 「齐全」随值集增长可以变为不齐全（非单调）：所指类即时记为已枚举（单调），点名登记查找点、留到工作队列
    /// 排空时按当时的值集判定（[`Self::seed_ctor_lookups`]），结果与处理顺序无关。数组类没有构造器
    pub(super) fn constructor_lookup(&mut self, m: usize, off: u32, k: &str, mref: &MemberRef, opcode: u8, args: &[V]) {
        let Some(md) = parse_method(&mref.desc) else { return };
        let skip = usize::from(opcode != classfile::op::INVOKESTATIC);
        let mut vals: Vec<V> = vec![];
        if skip == 1 && mref.owner == CLASS {
            vals.extend(args.first().cloned());
        }
        for (p, a) in md.params.iter().zip(args.iter().skip(skip)) {
            if matches!(p, FieldType::Object(c) if c == CLASS) {
                vals.push(a.clone());
            }
        }
        let mut classes: BTreeSet<String> = BTreeSet::new();
        for v in &vals {
            classes.extend(self.mirror_classes_of(m, off, k, v, true).0);
        }
        self.ctor_lookups.insert((m, off), (k.to_string(), vals));
        for cls in classes.into_iter().filter(|c| !c.starts_with('[')) {
            let c = self.id(&cls);
            if self.enumerated.insert((Members::Constructors, c)) && self.invokable.contains(&Members::Constructors) {
                self.expose(Members::Constructors, c);
            }
        }
    }

    /// 补种：工作队列排空时，构造器查找点上值集齐全的 Class 值点名其所指类（先按当前值集求出全部判定再施加，
    /// 施加带来的增长留给下一轮）；有新点名返回 true
    pub(super) fn seed_ctor_lookups(&mut self) -> bool {
        let sites: Vec<((usize, u32), (String, Vec<V>))> = self.ctor_lookups.iter().map(|(s, x)| (*s, x.clone())).collect();
        let mut picked: BTreeSet<String> = BTreeSet::new();
        for ((m, off), (k, vals)) in sites {
            for v in &vals {
                let (cs, complete) = self.mirror_classes_of(m, off, &k, v, false);
                if complete {
                    picked.extend(cs.into_iter().filter(|c| !c.starts_with('[')));
                }
            }
        }
        let mut grew = false;
        for cls in picked {
            let c = self.id(&cls);
            if self.named_ctors.insert(c) {
                grew = true;
                if self.invokable.contains(&Members::Constructors) {
                    self.expose(Members::Constructors, c);
                }
            }
        }
        grew
    }

    /// 类 c 的 k 类成员入链：方法经反射调用通道执行，形参与接收者取自通道的实参池（`reflect_call.rs`；
    /// 枚举得到的方法为反射对象通道，按名查找到的按查找结果的类型定通道）；记录分量访问器与构造器另有调用面
    /// （记录对象方法的引导模型 / 反射构造），VM 按反射对象调用，形参按声明类型 open（同 VM 入口）；构造器实例化其类
    pub(super) fn expose(&mut self, k: Members, c: u32) {
        let cls = self.names[c as usize].to_string();
        let Some(cf) = self.h.class(&cls) else { return };
        let comps: Vec<(String, String)> = cf.record_components.clone().unwrap_or_default();
        // 用户类被枚举即全部成员有分派臂；其余类只有按名查找点到的方法（运行时反射分派面同口径）。
        // 构造器无按名形状：非用户类的构造器经按名取类解析到的类（常量名拼出的具体类）、构造器查找
        // （`constructor_lookup`）点名的类给出，其余经清单补种（如 JCA 服务实现类）
        let named = k == Members::Constructors && self.named_ctors.contains(&c);
        let user = (self.domain(&cls) == Domain::User || named) && self.enumerated.contains(&(k, c));
        let names = self.reflect_names.get(&c).cloned().unwrap_or_default();
        // 可被覆写的实例方法：反射调用按接收者虚分派（覆写可在子类，含未枚举的类）
        let overridable = |mm: &classfile::Method| reflect_virtual(mm.access, cf.access);
        let picked: Vec<(String, String, bool)> = cf
            .methods
            .iter()
            .filter(|mm| match k {
                Members::Methods => !mm.name.starts_with('<') && (user || names.contains_key(&mm.name)),
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
            if k == Members::Methods {
                let mask = names.get(&key.name).copied().unwrap_or(0) | if user { rc_bit(RC_OBJ) } else { 0 };
                self.reflect_members.insert((k, key.clone()));
                self.rcall_member(key, cf.is_interface(), virt, mask);
                continue;
            }
            if !self.reflect_members.insert((k, key.clone())) {
                continue;
            }
            if virt {
                self.vm_dispatch(&key, cf.is_interface(), via.clone(), VmBind::Open);
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
