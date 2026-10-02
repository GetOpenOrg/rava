//! 引擎：反射——类镜像与成员面。

use super::reflect_call::{rc_bit, RcallMember, RC_HANDLE, RC_OBJ};
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

    /// 值集中各值的类镜像；类型推不出（open、lambda 合成类、手写实现对象）为所指未知的 Class
    pub(super) fn mirror_set(&mut self, s: &TypeSet) -> TypeSet {
        let mut out = TypeSet::default();
        let xs: Vec<u32> = s.classes.iter().collect();
        for x in xs {
            let k = if self.lambdas.contains_key(&x) || self.hwobjs.contains_key(&x) {
                self.id(CLASS)
            } else {
                let t = self.ty(x);
                self.mirror_id(t)
            };
            out.classes.insert(k);
        }
        if !s.open.is_empty() {
            out.classes.insert(self.id(CLASS));
        }
        out
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

    /// 类 cls 的方法被按名 name 查找（形参类型受 sig 约束）、结果经反射调用通道 ch 调用：方法反射调用可达时
    /// 补入该名且形参符合约束的方法
    pub(super) fn reflect_name(&mut self, cls: &str, name: &str, ch: u8, sig: Option<&ParamSig>) {
        let c = self.id(cls);
        let e = self.reflect_names.entry(c).or_default().entry(name.to_string()).or_default().entry(sig.cloned()).or_default();
        if *e & rc_bit(ch) != 0 {
            return;
        }
        *e |= rc_bit(ch);
        if self.invokable.contains(&Members::Methods) {
            self.expose(Members::Methods, c);
        }
    }

    /// 类 c 的 k 类成员入链：方法按查找通道的反射调用实参池接形参与接收者（枚举得到的为反射对象通道）；
    /// 其余形参按声明类型 open（同 VM 入口）；构造器实例化其类
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
                Members::Methods => !mm.name.starts_with('<') && (user || name_mask(&names, &mm.name, &mm.desc) != 0),
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
            // 方法经反射调用入口执行：形参与接收者取自查找通道的反射调用实参池（`reflect_call.rs`）；
            // 记录分量访问器与构造器另有调用面（记录对象方法的引导模型 / 反射构造），形参按声明类型 open
            if k == Members::Methods {
                self.reflect_members.insert((k, key.clone()));
                let mask = name_mask(&names, &key.name, &key.desc) | if user { rc_bit(RC_OBJ) } else { 0 };
                for ch in [RC_OBJ, RC_HANDLE] {
                    if mask & rc_bit(ch) == 0 || !self.rcall_exposed.insert((key.clone(), ch)) {
                        continue;
                    }
                    let t = self.method(key.clone(), via.clone());
                    if !self.methods[t].is_static {
                        self.rcall_recv(RcallMember { key: key.clone(), iface: cf.is_interface(), ch, virt });
                    }
                    self.rcall_target(t, ch);
                }
                continue;
            }
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

/// 按名查方法的形参类型约束：查找调用的 `Class[]` 实参里各数组元素的类镜像所指类型（数组类按描述符）。
/// 被查到的方法形参逐个属于 types；无形参的方法只在实参可为空数组 / null 时被查到（empty）
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct ParamSig {
    pub(super) types: BTreeSet<String>,
    pub(super) empty: bool,
}

impl ParamSig {
    fn admits(&self, desc: &str) -> bool {
        let Some(md) = parse_method(desc) else { return true };
        if md.params.is_empty() {
            return self.empty;
        }
        md.params.iter().all(|p| {
            let t = match p {
                FieldType::Object(c) => c.clone(),
                other => other.descriptor(),
            };
            self.types.contains(&t)
        })
    }
}

/// 方法（name, desc）在按名点名表里命中的反射调用通道位集（名字相同且形参符合该查找点的约束）
fn name_mask(names: &BTreeMap<String, BTreeMap<Option<ParamSig>, u8>>, name: &str, desc: &str) -> u8 {
    names.get(name).map_or(0, |sigs| sigs.iter().filter(|(s, _)| s.as_ref().is_none_or(|s| s.admits(desc))).fold(0, |a, (_, &b)| a | b))
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
