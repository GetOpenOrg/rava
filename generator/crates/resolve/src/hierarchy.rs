//! 类层次查询与 JVMS 解析 / 选择规则：
//! - 字段解析 §5.4.3.2
//! - 类方法解析 §5.4.3.3、接口方法解析 §5.4.3.4
//! - 虚方法选择 §5.4.6（invokevirtual / invokeinterface 的运行期目标）
//!
//! 全部查询是纯函数（只经 ClassPath 读缓存），不产生「加载即改变可见性」的副作用。

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;
use std::sync::Arc;

use classfile::{ClassFile, Field, Method};

use crate::classpath::ClassPath;

/// JVMS §4.10.1.2：数组类型的直接超类型（语言规范层面的固定集合，不是库知识）
const OBJECT: &str = "java/lang/Object";
const ARRAY_SUPERTYPES: [&str; 3] = [OBJECT, "java/lang/Cloneable", "java/io/Serializable"];

/// 已解析的方法：声明类 + 方法下标
#[derive(Clone)]
pub struct MethodSite {
    pub class: Arc<ClassFile>,
    pub index: usize,
}

impl MethodSite {
    pub fn method(&self) -> &Method {
        &self.class.methods[self.index]
    }
    pub fn key(&self) -> (String, String, String) {
        let m = self.method();
        (self.class.name.clone(), m.name.clone(), m.desc.clone())
    }
}

impl std::fmt::Debug for MethodSite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m = self.method();
        write!(f, "{}.{}:{}", self.class.name, m.name, m.desc)
    }
}

#[derive(Clone)]
pub struct FieldSite {
    pub class: Arc<ClassFile>,
    pub index: usize,
}

impl FieldSite {
    pub fn field(&self) -> &Field {
        &self.class.fields[self.index]
    }
}

pub struct Hierarchy<'a> {
    pub cp: &'a ClassPath,
    supertypes: RefCell<HashMap<String, Rc<BTreeSet<String>>>>,
    /// 虚方法选择的记忆：(已解析方法的声明类实例地址, 方法下标) → (声明类, 接收者类 → 选中方法)。
    /// 类文件经类路径缓存唯一驻留；值里持有声明类，地址在记忆存续期间不会复用
    selected: RefCell<SelectMemo>,
}

type SelectMemo = HashMap<(usize, usize), (Arc<ClassFile>, HashMap<String, Option<MethodSite>>)>;

pub fn package_of(name: &str) -> &str {
    name.rfind('/').map_or("", |i| &name[..i])
}

/// JVMS §2.9.3 签名多态方法：native + varargs + 唯一形参 Object[]（结构判定）
pub fn is_signature_polymorphic(m: &Method) -> bool {
    m.is_native() && m.access & classfile::acc::VARARGS != 0 && m.desc.starts_with("([Ljava/lang/Object;)")
}

impl<'a> Hierarchy<'a> {
    pub fn new(cp: &'a ClassPath) -> Self {
        Hierarchy { cp, supertypes: RefCell::new(HashMap::new()), selected: RefCell::new(HashMap::new()) }
    }

    pub fn class(&self, name: &str) -> Option<Arc<ClassFile>> {
        self.cp.get(name)
    }

    /// 自身 + 父类链（最近优先）
    pub fn superclasses(&self, name: &str) -> Vec<Arc<ClassFile>> {
        let mut out = Vec::new();
        let mut cur = self.class(name);
        while let Some(c) = cur {
            if out.iter().any(|x: &Arc<ClassFile>| x.name == c.name) {
                break;
            }
            cur = c.super_name.as_deref().and_then(|s| self.class(s));
            out.push(c);
        }
        out
    }

    /// 全部超类型（含自身、父类链、传递父接口）；数组按 JVMS 规则
    pub fn supertypes(&self, name: &str) -> Rc<BTreeSet<String>> {
        if let Some(s) = self.supertypes.borrow().get(name) {
            return s.clone();
        }
        let mut acc = BTreeSet::new();
        acc.insert(name.to_string());
        if let Some(elem) = name.strip_prefix('[') {
            acc.extend(ARRAY_SUPERTYPES.iter().map(|s| s.to_string()));
            // 引用元素数组协变：[LA; <: [LB; 当 A <: B
            if let Some(inner) = elem.strip_prefix('L').and_then(|e| e.strip_suffix(';')) {
                for s in self.supertypes(inner).iter() {
                    acc.insert(format!("[L{s};"));
                }
            } else if elem.starts_with('[') {
                for s in self.supertypes(elem).iter() {
                    acc.insert(format!("[{}", if s.starts_with('[') { s.clone() } else { format!("L{s};") }));
                }
            }
        } else {
            // 环保护：先占位
            self.supertypes.borrow_mut().insert(name.to_string(), Rc::new(acc.clone()));
            if let Some(c) = self.class(name) {
                for up in c.super_name.iter().chain(c.interfaces.iter()) {
                    acc.extend(self.supertypes(up).iter().cloned());
                }
            }
        }
        let rc = Rc::new(acc);
        self.supertypes.borrow_mut().insert(name.to_string(), rc.clone());
        rc
    }

    pub fn is_subtype(&self, sub: &str, sup: &str) -> bool {
        sub == sup || sup == OBJECT || self.supertypes(sub).contains(sup)
    }

    pub fn is_interface(&self, name: &str) -> bool {
        self.class(name).is_some_and(|c| c.is_interface())
    }

    // ── 字段解析 §5.4.3.2 ─────────────────────────────────────────────────────

    pub fn resolve_field(&self, owner: &str, name: &str, desc: &str) -> Option<FieldSite> {
        let c = self.class(owner)?;
        if let Some(i) = c.fields.iter().position(|f| f.name == name && f.desc == desc) {
            return Some(FieldSite { class: c, index: i });
        }
        for i in &c.interfaces {
            if let Some(s) = self.resolve_field(i, name, desc) {
                return Some(s);
            }
        }
        c.super_name.as_deref().and_then(|s| self.resolve_field(s, name, desc))
    }

    // ── 方法解析 §5.4.3.3 / §5.4.3.4 ─────────────────────────────────────────

    /// `interface_ref`：常量池项是 InterfaceMethodref
    pub fn resolve_method(&self, owner: &str, name: &str, desc: &str, interface_ref: bool) -> Option<MethodSite> {
        if owner.starts_with('[') {
            // 数组上的方法调用（clone 等）解析到 Object
            return self.resolve_method(OBJECT, name, desc, false);
        }
        let c = self.class(owner)?;
        if interface_ref || c.is_interface() {
            // §5.4.3.4：接口自身 → Object 的 public 实例方法 → 极大特化超接口方法 → 任一超接口方法
            if let Some(i) = c.methods.iter().position(|m| m.name == name && m.desc == desc) {
                return Some(MethodSite { class: c, index: i });
            }
            if let Some(o) = self.class(OBJECT) {
                if let Some(i) = o.methods.iter().position(|m| {
                    m.name == name && m.desc == desc && m.access & classfile::acc::PUBLIC != 0 && !m.is_static()
                }) {
                    return Some(MethodSite { class: o, index: i });
                }
            }
            return self.superinterface_method(&c, name, desc);
        }
        // §5.4.3.3 步骤 2：C 及其父类链（签名多态按名字匹配）
        for k in self.superclasses(owner) {
            if let Some(i) = k.methods.iter().position(|m| m.name == name && m.desc == desc) {
                return Some(MethodSite { class: k, index: i });
            }
            if let Some(i) = k.methods.iter().position(|m| m.name == name && is_signature_polymorphic(m)) {
                return Some(MethodSite { class: k, index: i });
            }
        }
        // 步骤 3：超接口
        self.superinterface_method(&c, name, desc)
    }

    /// 全部超接口（传递，含父类的接口）
    pub fn all_superinterfaces(&self, c: &ClassFile) -> Vec<Arc<ClassFile>> {
        let mut seen = BTreeSet::new();
        let mut out = Vec::new();
        let mut stack: Vec<String> = Vec::new();
        for k in self.superclasses(&c.name) {
            stack.extend(k.interfaces.iter().rev().cloned());
        }
        stack.reverse();
        let mut queue: std::collections::VecDeque<String> = stack.into();
        while let Some(n) = queue.pop_front() {
            if !seen.insert(n.clone()) {
                continue;
            }
            if let Some(ic) = self.class(&n) {
                queue.extend(ic.interfaces.iter().cloned());
                out.push(ic);
            }
        }
        out
    }

    /// 极大特化超接口方法（非 private、非 static）；无则任一超接口的非 private 非 static 声明
    fn superinterface_method(&self, c: &ClassFile, name: &str, desc: &str) -> Option<MethodSite> {
        let cands = self.maximally_specific(c, name, desc);
        if let Some(nonabs) = cands.iter().find(|s| !s.method().is_abstract()) {
            if cands.iter().filter(|s| !s.method().is_abstract()).count() == 1 {
                return Some(nonabs.clone());
            }
        }
        cands.into_iter().next()
    }

    /// JVMS §5.4.3.3：极大特化超接口方法集合
    pub fn maximally_specific(&self, c: &ClassFile, name: &str, desc: &str) -> Vec<MethodSite> {
        let ifaces = self.all_superinterfaces(c);
        let decl: Vec<MethodSite> = ifaces
            .iter()
            .filter_map(|ic| {
                ic.methods
                    .iter()
                    .position(|m| m.name == name && m.desc == desc && !m.is_private() && !m.is_static())
                    .map(|i| MethodSite { class: ic.clone(), index: i })
            })
            .collect();
        decl.iter()
            .filter(|s| {
                !decl.iter().any(|o| o.class.name != s.class.name && self.is_subtype(&o.class.name, &s.class.name))
            })
            .cloned()
            .collect()
    }

    // ── 虚方法选择 §5.4.6 ─────────────────────────────────────────────────────

    /// 接收者运行期类为 `receiver` 时，已解析方法 `resolved` 的实际执行者
    pub fn select(&self, receiver: &str, resolved: &MethodSite) -> Option<MethodSite> {
        let rm = resolved.method();
        if rm.is_private() || rm.is_static() || rm.is_init() {
            return Some(resolved.clone());
        }
        let key = (Arc::as_ptr(&resolved.class) as usize, resolved.index);
        if let Some(hit) = self.selected.borrow().get(&key).and_then(|(_, m)| m.get(receiver)) {
            return hit.clone();
        }
        let out = self.select_uncached(receiver, resolved);
        let mut memo = self.selected.borrow_mut();
        let e = memo.entry(key).or_insert_with(|| (resolved.class.clone(), HashMap::new()));
        e.1.insert(receiver.to_string(), out.clone());
        out
    }

    fn select_uncached(&self, receiver: &str, resolved: &MethodSite) -> Option<MethodSite> {
        let rm = resolved.method();
        let lookup_start = if receiver.starts_with('[') { OBJECT } else { receiver };
        for k in self.superclasses(lookup_start) {
            if let Some(i) = k.methods.iter().position(|m| m.name == rm.name && m.desc == rm.desc && !m.is_static()) {
                let site = MethodSite { class: k.clone(), index: i };
                if k.name == resolved.class.name || self.overrides(&site, resolved) {
                    return Some(site);
                }
            }
        }
        let rc = self.class(lookup_start)?;
        let cands: Vec<MethodSite> = self
            .maximally_specific(&rc, &rm.name, &rm.desc)
            .into_iter()
            .filter(|s| !s.method().is_abstract())
            .collect();
        if cands.len() == 1 {
            return cands.into_iter().next();
        }
        // 抽象 / 冲突：运行期 AbstractMethodError / IncompatibleClassChangeError
        None
    }

    /// JVMS §5.4.5：mc 覆盖 ma（mc 声明类是 ma 声明类的子类）
    fn overrides(&self, mc: &MethodSite, ma: &MethodSite) -> bool {
        let a = ma.method();
        if a.is_private() || mc.method().is_private() {
            return false;
        }
        if a.access & (classfile::acc::PUBLIC | classfile::acc::PROTECTED) != 0 || ma.class.is_interface() {
            return true;
        }
        // 包私有：同运行期包（同一类加载器假设下即同包）
        package_of(&mc.class.name) == package_of(&ma.class.name)
    }
}
