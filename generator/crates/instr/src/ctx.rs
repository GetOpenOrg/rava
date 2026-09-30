//! 指令翻译上下文：类型层查询 + runtime 清单 + 预计算事实 + 发射层回调。
//!
//! Python 侧这些信息散落在模块级缓存（`_ROOT_API_NAMES`、`_HANDWRITTEN_BOUNDARY_CACHE`、
//! `_closure_subclasses` 的 lru_cache、`_CS_CTX`）与发射层全局账（`sam_objects.SAM_LEDGER`）；
//! 这里一律显式传入，由调用方（方法体生成 P4c）持有。

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use classfile::ClassFile;
use input::RuntimeManifest;
use ty::TyCtx;

use crate::text::{scan_fn_names, to_snake};

const ACC_PUBLIC: u16 = 0x0001;
const ACC_PROTECTED: u16 = 0x0004;
const ACC_STATIC: u16 = 0x0008;
const ACC_SYNTHETIC: u16 = 0x1000;

/// 发射层回调（类文件 / crate 布局相关，instr 不掌握的信息）
pub trait InstrHooks {
    /// invokedynamic lambda 站点的 SAM 合成对象构造路径（`sam_objects.site_ctor_path`）：
    /// `iface` 为函数式接口 binary 名，`current_class` 为站点所在类。None → 该接口不可合成。
    fn sam_ctor_path(&self, iface: &str, current_class: &str) -> Option<ir::Path>;
}

/// 无合成对象的缺省回调（单元测试 / 不含 lambda 的调用方）
pub struct NoHooks;

impl InstrHooks for NoHooks {
    fn sam_ctor_path(&self, _iface: &str, _current_class: &str) -> Option<ir::Path> {
        None
    }
}

/// 一次构建、整个类集共享的预计算事实
pub struct InstrFacts {
    /// 根类 `java/lang/Object` 的 public 实例方法（非合成，非 `<init>`）：
    /// `(name, "(params)")`（`member_owner._root_virtual_methods`）
    pub root_virtual: BTreeSet<(String, String)>,
    /// 根类的 protected void 实例方法（`_root_protected_void_methods`）
    pub root_protected_void: BTreeSet<(String, String)>,
    /// 手写根类 API 名面（`object_impl.rs` / `object_ext.rs` / `object.rs` 的 `pub fn` 名）
    pub root_api: BTreeSet<String>,
    /// 类子类索引：祖先 binary → 闭包内全部子类（registry 插入序；`_closure_subclasses`）
    pub subclasses: BTreeMap<String, Vec<String>>,
    /// runtime 手写源码根（`runtime/java_runtime/src`）
    runtime_src: PathBuf,
    /// `_impl.rs` 伴生文件的 `fn` 名缓存（相对路径 → 名集合；文件缺失 → 空集）
    impl_fn_cache: RefCell<BTreeMap<String, BTreeSet<String>>>,
}

impl InstrFacts {
    /// `root_object`：JDK `java/lang/Object` 的类文件（缺失时根类方法集为空）；
    /// `runtime_src`：`runtime/java_runtime/src`。
    pub fn build(reg: &ty::Registry, root_object: Option<&ClassFile>, runtime_src: &Path) -> InstrFacts {
        let mut root_virtual = BTreeSet::new();
        let mut root_protected_void = BTreeSet::new();
        for m in root_object.map(|cf| cf.methods.as_slice()).unwrap_or(&[]) {
            if m.access & ACC_STATIC != 0 || m.access & ACC_SYNTHETIC != 0 || m.synthetic_attr || m.name.starts_with('<') {
                continue;
            }
            let params = m.desc.split(')').next().map(|p| format!("{p})")).unwrap_or_default();
            if m.access & ACC_PUBLIC != 0 {
                root_virtual.insert((m.name.clone(), params.clone()));
            }
            if m.access & ACC_PROTECTED != 0 && m.desc.ends_with(")V") {
                root_protected_void.insert((m.name.clone(), params));
            }
        }
        let mut root_api = BTreeSet::new();
        let lang = runtime_src.join("java").join("lang");
        for f in ["object_impl.rs", "object_ext.rs", "object.rs"] {
            if let Ok(text) = std::fs::read_to_string(lang.join(f)) {
                root_api.extend(scan_fn_names(&text, true));
            }
        }
        InstrFacts {
            root_virtual,
            root_protected_void,
            root_api,
            subclasses: closure_subclasses(reg),
            runtime_src: runtime_src.to_path_buf(),
            impl_fn_cache: RefCell::new(BTreeMap::new()),
        }
    }

    /// 类 `cls_bin` 的手写伴生文件（`<pkg>/<snake>_impl.rs` 或 `<snake>_t_impl.rs`）
    /// 是否含 `fn <rust_name>`（`invoke_sig._handwritten_boundary_method` 的文件探测部分）
    pub fn impl_has_fn(&self, cls_bin: &str, rust_name: &str) -> bool {
        // 无包名的类不探测（Python：`len(parts) >= 2`）
        let Some((pkg, simple)) = cls_bin.rsplit_once('/') else {
            return false;
        };
        let snake = to_snake(simple);
        [format!("{pkg}/{snake}_impl.rs"), format!("{pkg}/{snake}_t_impl.rs")]
            .iter()
            .any(|rel| self.impl_fns(rel).contains(rust_name))
    }

    fn impl_fns(&self, rel: &str) -> BTreeSet<String> {
        if let Some(hit) = self.impl_fn_cache.borrow().get(rel) {
            return hit.clone();
        }
        let names = std::fs::read_to_string(self.runtime_src.join(rel))
            .map(|t| scan_fn_names(&t, false))
            .unwrap_or_default();
        self.impl_fn_cache.borrow_mut().insert(rel.to_string(), names.clone());
        names
    }
}

/// `invoke_virtual._closure_subclasses`：非接口类沿超类链登记到每个祖先（插入序）
fn closure_subclasses(reg: &ty::Registry) -> BTreeMap<String, Vec<String>> {
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for ci in reg.iter_insertion() {
        if ci.is_interface() {
            continue;
        }
        let mut seen = BTreeSet::new();
        let mut cur = ci.super_class();
        while !cur.is_empty() && !seen.contains(cur) {
            let Some(parent) = reg.get(cur) else {
                break;
            };
            seen.insert(cur);
            out.entry(cur.to_string()).or_default().push(ci.name().to_string());
            cur = parent.super_class();
        }
    }
    out
}

/// 单个类的指令翻译上下文
#[derive(Clone, Copy)]
pub struct InstrCtx<'a> {
    pub ty: TyCtx<'a>,
    pub rt: &'a RuntimeManifest,
    pub facts: &'a InstrFacts,
    pub hooks: &'a dyn InstrHooks,
    /// 当前类 binary 名
    pub class_name: &'a str,
}

impl<'a> InstrCtx<'a> {
    pub fn new(
        ty: TyCtx<'a>,
        rt: &'a RuntimeManifest,
        facts: &'a InstrFacts,
        hooks: &'a dyn InstrHooks,
        class_name: &'a str,
    ) -> InstrCtx<'a> {
        InstrCtx { ty, rt, facts, hooks, class_name }
    }

    pub fn reg(&self) -> &'a ty::Registry {
        self.ty.reg
    }

    /// binary → Rust 短名
    pub fn short(&self, binary: &str) -> String {
        self.ty.names.short(binary).into_owned()
    }

    /// 当前类的 bootstrap 方法表（invokedynamic 用）
    pub fn class_file(&self) -> Option<&'a ClassFile> {
        self.ty.reg.get(self.class_name).map(|ci| ci.class_file())
    }
}
