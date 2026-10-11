//! 分析所需的程序事实：类查找、子类型、方法解析、lambda 站点索引、不透明判定。
//! 跨区域共享（每个程序一份），内部缓存加锁（发射按类并行）。

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, OnceLock};

use classfile::{Method, MethodHandle, Operand};
use input::manifest::IndyKind;
use input::{EmitInput, EscapeEffect, RuntimeManifest};
use ty::ident::safe_ident;
use ty::registry::ClassInfo;
use ty::type_map::mangle_name;

/// 描述符槽类别
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Ref,
    Int,
    Wide,
}

/// 单个字段描述符的类别
pub fn field_kind(desc: &str) -> Kind {
    match desc.as_bytes().first() {
        Some(b'L') | Some(b'[') => Kind::Ref,
        Some(b'J') | Some(b'D') => Kind::Wide,
        _ => Kind::Int,
    }
}

/// 方法描述符 → (形参类别, 返回类别；void 为 None)
pub fn method_kinds(desc: &str) -> (Vec<Kind>, Option<Kind>) {
    let b = desc.as_bytes();
    let mut i = 1;
    let mut params = Vec::new();
    while i < b.len() && b[i] != b')' {
        let start = i;
        while b[i] == b'[' {
            i += 1;
        }
        if b[i] == b'L' {
            while b[i] != b';' {
                i += 1;
            }
        }
        i += 1;
        params.push(field_kind(&desc[start..i]));
    }
    let ret = desc.get(i + 1..).filter(|r| *r != "V").map(field_kind);
    (params, ret)
}

/// 返回类型为引用时的类名（`L…;` 去壳；数组保留描述符）
pub fn ret_class(desc: &str) -> Option<&str> {
    let r = &desc[desc.find(')')? + 1..];
    if let Some(c) = r.strip_prefix('L') {
        return c.strip_suffix(';');
    }
    r.starts_with('[').then_some(r)
}

/// lambda 站点：函数式接口、SAM 名、实现句柄
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LambdaSite {
    pub iface: String,
    pub sam: String,
    pub kind: u8,
    pub owner: String,
    pub name: String,
    pub desc: String,
}

/// 被调方法的处置
pub enum Callee<'a> {
    /// 不在调用链上（存根，命中即 panic）
    Stub,
    /// 无方法体（抽象）
    Abstract,
    /// 清单登记的效果
    Effect(EscapeEffect),
    /// native：VM 承载（native 帧内分配不登记），实参不得含区域对象
    Native,
    /// 手写 / VM 内建：分析不可见
    Opaque,
    /// 按字节码分析
    Code(&'a ClassInfo, &'a Method),
}

/// 跨方法共享的分析缓存（不含借用：调用方可放进任意生命周期的上下文）
#[derive(Default)]
pub struct WorldCache {
    lambdas: OnceLock<Vec<LambdaSite>>,
    supers: Mutex<HashMap<String, Arc<HashSet<String>>>>,
    cha: Mutex<HashMap<(String, String, String), Arc<Vec<(String, usize)>>>>,
}

#[derive(Clone, Copy)]
pub struct World<'a> {
    pub input: &'a EmitInput,
    pub manifest: &'a RuntimeManifest,
    cache: &'a WorldCache,
}

impl<'a> World<'a> {
    pub fn new(input: &'a EmitInput, manifest: &'a RuntimeManifest, cache: &'a WorldCache) -> World<'a> {
        World { input, manifest, cache }
    }

    pub fn class(&self, name: &str) -> Option<&'a ClassInfo> {
        if let Some(c) = self.input.registry.get(name) {
            return Some(c);
        }
        self.input.root.as_ref().filter(|r| r.name() == name)
    }

    /// 根类（Object）名
    pub fn root_name(&self) -> Option<&'a str> {
        self.input.root.as_ref().map(|r| r.name())
    }

    /// 类的全部超类型（含自身；数组只含自身与根类）
    pub fn supertypes(&self, name: &str) -> Arc<HashSet<String>> {
        if let Some(s) = self.cache.supers.lock().unwrap().get(name) {
            return s.clone();
        }
        let mut out = HashSet::new();
        let mut stack = vec![name.to_string()];
        while let Some(c) = stack.pop() {
            if !out.insert(c.clone()) {
                continue;
            }
            if let Some(ci) = self.class(&c) {
                if !ci.super_class().is_empty() {
                    stack.push(ci.super_class().to_string());
                }
                stack.extend(ci.interfaces().iter().cloned());
            }
        }
        if let Some(r) = self.root_name() {
            out.insert(r.to_string());
        }
        let out = Arc::new(out);
        self.cache.supers.lock().unwrap().insert(name.to_string(), out.clone());
        out
    }

    pub fn is_subtype(&self, c: &str, t: &str) -> bool {
        self.supertypes(c).contains(t)
    }

    fn find(&self, cls: &str, name: &str, desc: &str) -> Option<(&'a ClassInfo, usize)> {
        let ci = self.class(cls)?;
        let i = ci.methods().iter().position(|m| m.name == name && m.desc == desc)?;
        Some((ci, i))
    }

    /// invokestatic / invokespecial：自属主沿超类链查找（找不到再查接口，供 `I.super.m()` 与接口静态方法）
    pub fn resolve_direct(&self, owner: &str, name: &str, desc: &str) -> Option<(&'a ClassInfo, usize)> {
        let mut c = owner.to_string();
        loop {
            if let Some(hit) = self.find(&c, name, desc) {
                return Some(hit);
            }
            match self.class(&c) {
                Some(ci) if !ci.super_class().is_empty() => c = ci.super_class().to_string(),
                _ => break,
            }
        }
        self.iface_defaults(owner, name, desc).into_iter().next()
    }

    /// 接收者运行期类为 `cls` 时的虚分派目标：类链上的首个声明；类链上没有时取超接口的
    /// 全部非抽象默认方法（多个时并取，保守）
    pub fn resolve_virtual(&self, cls: &str, name: &str, desc: &str) -> Vec<(&'a ClassInfo, usize)> {
        let mut c = cls.to_string();
        if !cls.starts_with('[') {
            loop {
                if let Some((ci, i)) = self.find(&c, name, desc) {
                    if !ci.methods()[i].is_static() {
                        return vec![(ci, i)];
                    }
                }
                match self.class(&c) {
                    Some(ci) if !ci.super_class().is_empty() => c = ci.super_class().to_string(),
                    _ => break,
                }
            }
        }
        if let Some(r) = self.root_name() {
            if let Some(hit) = self.find(r, name, desc) {
                return vec![hit];
            }
        }
        self.iface_defaults(cls, name, desc)
    }

    fn iface_defaults(&self, cls: &str, name: &str, desc: &str) -> Vec<(&'a ClassInfo, usize)> {
        let mut out = Vec::new();
        let mut sups: Vec<String> = self.supertypes(cls).iter().cloned().collect();
        sups.sort();
        for s in sups {
            let Some(ci) = self.class(&s) else { continue };
            if !ci.is_interface() {
                continue;
            }
            if let Some((ci, i)) = self.find(&s, name, desc) {
                let m = &ci.methods()[i];
                if !m.is_abstract() && !m.is_static() {
                    out.push((ci, i));
                }
            }
        }
        out
    }

    /// 接收者类型未知（区域前已有的对象）时的分派目标：已实例化的 `owner` 子类型各自解析
    pub fn cha(&self, owner: &str, name: &str, desc: &str) -> Arc<Vec<(String, usize)>> {
        let key = (owner.to_string(), name.to_string(), desc.to_string());
        if let Some(v) = self.cache.cha.lock().unwrap().get(&key) {
            return v.clone();
        }
        let mut seen = HashSet::new();
        let mut out = Vec::new();
        for c in &self.input.instantiated {
            if !self.is_subtype(c, owner) {
                continue;
            }
            for (ci, i) in self.resolve_virtual(c, name, desc) {
                if seen.insert((ci.name().to_string(), i)) {
                    out.push((ci.name().to_string(), i));
                }
            }
        }
        // 根类 / 数组接收者
        if let Some(r) = self.root_name() {
            if owner == r || owner.starts_with('[') {
                if let Some((ci, i)) = self.find(r, name, desc) {
                    if seen.insert((ci.name().to_string(), i)) {
                        out.push((ci.name().to_string(), i));
                    }
                }
            }
        }
        let out = Arc::new(out);
        self.cache.cha.lock().unwrap().insert(key, out.clone());
        out
    }

    /// 调用链上全部 lambda 站点（扫描已访问方法的 invokedynamic）
    pub fn lambdas(&self) -> &[LambdaSite] {
        self.cache.lambdas.get_or_init(|| {
            let mut set = HashSet::new();
            let mut out = Vec::new();
            for (cls, name, desc) in &self.input.visited {
                let Some((ci, i)) = self.find(cls, name, desc) else { continue };
                let Some(code) = ci.methods()[i].code.as_ref() else { continue };
                for insn in &code.insns {
                    if let Operand::InvokeDynamic { bsm, name, desc, .. } = &insn.operand {
                        if let Some(site) = self.lambda_site(ci, *bsm, name, desc) {
                            if set.insert(site.clone()) {
                                out.push(site);
                            }
                        }
                    }
                }
            }
            out
        })
    }

    /// invokedynamic 的引导方法分类
    pub fn indy_kind(&self, ci: &ClassInfo, bsm: u16) -> Option<IndyKind> {
        let b = ci.class_file().bootstrap_methods.get(bsm as usize)?;
        let m = &b.handle.member;
        self.manifest.indy_kind(&format!("{}.{}", m.owner, m.name))
    }

    /// lambda 站点解析：引导方法属 lambda 类，第 2 个静态实参是实现句柄
    pub fn lambda_site(&self, ci: &ClassInfo, bsm: u16, name: &str, desc: &str) -> Option<LambdaSite> {
        if self.indy_kind(ci, bsm) != Some(IndyKind::Lambda) {
            return None;
        }
        let b = ci.class_file().bootstrap_methods.get(bsm as usize)?;
        let MethodHandle { kind, member, .. } = match b.args.get(1)? {
            classfile::Const::MethodHandle(h) => h,
            _ => return None,
        };
        Some(LambdaSite {
            iface: ret_class(desc)?.to_string(),
            sam: name.to_string(),
            kind: *kind,
            owner: member.owner.clone(),
            name: member.name.clone(),
            desc: member.desc.clone(),
        })
    }

    /// 手写层是否承载该方法（与发射层 `hw_involved` 同口径）
    fn handwritten(&self, cls: &str, m: &Method) -> bool {
        let Some(h) = self.input.handwritten.get(cls) else { return false };
        let plain = safe_ident(&m.name);
        let mangled = safe_ident(&mangle_name(&self.manifest.ty, &m.name, &m.desc));
        [plain, mangled].iter().any(|n| {
            let imp = format!("__impl_{n}");
            h.methods.contains(n)
                || h.methods.contains(&imp)
                || h.method_cores.contains_key(n)
                || h.method_cores.contains_key(&imp)
                || h.iface_method_sigs.contains_key(n)
                || h.class_vtable_fns.contains(n)
        })
    }

    /// 被调方法的处置
    pub fn callee(&self, ci: &'a ClassInfo, i: usize) -> Callee<'a> {
        let m = &ci.methods()[i];
        let key = format!("{}.{}:{}", ci.name(), m.name, m.desc);
        if let Some(e) = self.manifest.escape_effects.get(&key) {
            return Callee::Effect(*e);
        }
        if !self.input.in_chain(ci.name(), &m.name, &m.desc) {
            return Callee::Stub;
        }
        if m.is_native() {
            return Callee::Native;
        }
        // 根类的手写同名方法只是静态类型为根类时的分派入口，语义在按字节码翻译的根类方法体
        let root_entry = self.root_name() == Some(ci.name()) && m.code.is_some();
        if self.manifest.intrinsic_members.contains(&key) || (!root_entry && self.handwritten(ci.name(), m)) {
            return Callee::Opaque;
        }
        if m.code.is_none() {
            return Callee::Abstract;
        }
        Callee::Code(ci, m)
    }

    /// `concat_stringify` 入口与拼接结果类
    pub fn concat_helper(&self) -> Option<(&'a ClassInfo, usize, String)> {
        let (owner, name, desc) = self.manifest.indy_helper("concat_stringify").ok()?;
        let (ci, i) = self.find(owner, name, desc)?;
        Some((ci, i, ret_class(desc)?.to_string()))
    }
}
