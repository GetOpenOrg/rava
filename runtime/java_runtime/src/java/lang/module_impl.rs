use crate::prelude::*;
use super::Module;

// java.lang.Module 伴生：单二进制运行时的模块语义。
//
// 产物无模块层（no JPMS runtime）——类全集编译期定死，全部归属**无名模块**
//（JDK 的 ALL-UNNAMED 语义）：isNamed 恒 false，ServiceLoader 的
// LazyClassPathLookupIterator 据此走 classpath 分支（资源枚举为空，
// 外部 provider 发现终止）。无名模块单例挂系统类加载器为 defining loader。

/// 全二进制唯一的无名模块（ALL-UNNAMED 对应物）。
pub fn unnamed_module() -> Module {
    crate::__process_static! {
        static UNNAMED: Module = {
            let mut m = Module::default();
            m._init_not_null();
            m.__set_loader(crate::java::lang::ClassLoader::getSystemClassLoader()
                .unwrap_or_default());
            m
        };
    }
    UNNAMED.with(Clone::clone)
}

impl Module {
    /// `getLayer()`：命名模块所在的层；无名模块不属于任何层 → null（JDK 语义；本运行时只有
    /// 无名模块）。消费方：`StackTraceElement.isHashedInJavaBase`（`ModuleLayer.boot() == m.getLayer()`）。
    #[jvm_boundary]
    pub fn getLayer(&self) -> Result<crate::java::lang::ModuleLayer> {
        Ok(Default::default())
    }

    /// 无名模块恒未命名（name 为 null → isNamed false，JDK 语义）。
    #[jvm_boundary]
    pub fn isNamed(&self) -> Result<bool> {
        Ok(!_is_jnull(&Object::from(self.__get_name())))
    }

    /// `Module.canUse(Class service)`：模块是否 uses 该服务。
    /// 无名模块对全部服务恒 uses（JDK Module.addUses/implAddUses 语义的
    /// 无名模块默认——ServiceLoader 据此放行 provider 消费）。
    #[jvm_boundary]
    pub fn canUse(&self, _service: crate::java::lang::Class) -> Result<bool> {
        Ok(true)
    }

    /// `isExported(String pn, Module other)`：无名模块向全部模块导出其全部包（JLS §7.7.5 /
    /// Module 规范：unnamed module exports all packages）。消费方：反射访问检查
    /// Reflection.verifyModuleAccess（FS-R R2a 字节码路径）。
    #[jvm_boundary]
    pub fn isExported_str_module(&self, _pn: String, _other: Module) -> Result<bool> {
        Ok(true)
    }

    /// `isExported(String pn)`：无条件导出（同上）。
    #[jvm_boundary]
    pub fn isExported_str(&self, _pn: String) -> Result<bool> {
        Ok(true)
    }

    /// `isOpen(String pn, Module other)`：无名模块向全部模块开放其全部包（深反射可达）。
    #[jvm_boundary]
    pub fn isOpen_str_module(&self, _pn: String, _other: Module) -> Result<bool> {
        Ok(true)
    }

    /// `isOpen(String pn)`：无条件开放（同上）。
    #[jvm_boundary]
    pub fn isOpen_str(&self, _pn: String) -> Result<bool> {
        Ok(true)
    }
}

// ── VM 模块表（HotSpot ModuleEntryTable / PackageEntryTable 的落地，类 3 VM 状态）──────────────
// 命名模块只经 `Module(ModuleLayer, ClassLoader, ModuleDescriptor, URI)` 构造器的 defineModule0
// 进入 VM（ModuleLayer.defineModules*）；未登记的 Module（含全部无名模块）在 HotSpot 中解析为其
// 加载器的无名模块（`java_lang_Module::module_entry` 的缺省分支），addReads0 / addExports*0 对其
// 为空操作。读边 / 导出边是 VM 链接期访问检查的状态：原生二进制的类全集由编译期定死、访问检查
// 由 javac 与闭包完成，这些边无运行期消费方——本表只承载可观测部分（模块与包的归属及其冲突检查），
// 与 HotSpot 的异常行为逐条对应。

struct VmModule {
    module: Module,
    loader: usize,
    name: std::string::String,
    open: bool,
    packages: Vec<std::string::String>,
}

fn _vm_modules<R>(f: impl FnOnce(&mut Vec<VmModule>) -> R) -> R {
    crate::__process_static! {
        static MODULES: crate::sync_model::__RefSlot<Vec<VmModule>> = crate::sync_model::__RefSlot::new(Vec::new());
    }
    MODULES.with(|t| f(&mut t.borrow_mut()))
}

fn _identity(o: Object) -> usize {
    if o.0.is_jvm_null() || _is_jnull(&o) { 0 } else { o.0.__identity() as usize }
}

fn _is_null<T: Into<Object>>(v: T) -> bool {
    let o: Object = v.into();
    o.0.is_jvm_null() || _is_jnull(&o)
}

fn _iae(msg: &str) -> JvmError {
    JvmError::illegal_argument(msg)
}

fn _npe(msg: &str) -> JvmError {
    match crate::java::lang::NullPointerException::new_str(String::from(msg)) {
        Ok(e) => JvmError::from(e),
        Err(e) => e,
    }
}

fn _ise(msg: &str) -> JvmError {
    match crate::java::lang::IllegalStateException::new_str(String::from(msg)) {
        Ok(e) => JvmError::from(e),
        Err(e) => e,
    }
}

/// HotSpot `verify_package_name`（内部形态，斜线分隔）：非空、首尾非 `/`、无空段、无 `;` / `[`。
fn _valid_package(p: &str) -> bool {
    !p.is_empty() && !p.starts_with('/') && !p.ends_with('/') && !p.contains("//")
        && !p.contains(|c| c == ';' || c == '[')
}

/// 已登记的命名模块的下标（未登记 → 无名模块语义）。
fn _entry_of(t: &[VmModule], m: &Module) -> Option<usize> {
    let id = _identity(Object::from(Clone::clone(m)));
    t.iter().position(|e| _identity(Object::from(Clone::clone(&e.module))) == id)
}

/// add_module_exports / add_module_exports_to_all_unnamed 的包归属检查（命名且非 open 的模块）。
fn _check_export(m: &Module, pn: &String, from_word: &str) -> Result<()> {
    let pkg = format!("{}", pn).replace('.', "/");
    _vm_modules(|t| {
        let Some(i) = _entry_of(t, m) else { return Ok(()) };
        let e = &t[i];
        if e.open || e.packages.contains(&pkg) {
            return Ok(());
        }
        let owner = t.iter().find(|o| o.loader == e.loader && o.packages.contains(&pkg));
        Err(match owner {
            Some(o) => _iae(&format!("Package: {} found in module {}, not in {}: {}", pkg, o.name, from_word, e.name)),
            None => _iae(&format!("Package {} not found in {} {}", pkg, from_word, e.name)),
        })
    })
}

impl Module {
    /// native `defineModule0`：HotSpot `Modules::define_module`——校验名与包，按加载器登记模块及其包；
    /// 重名模块 / 包已属他模块 → IllegalStateException；java.base 已由引导定义。
    #[jvm_native]
    pub fn defineModule0(module: Module, is_open: bool, _version: String, _location: String,
                         pns: JArray<Object>) -> Result<()> {
        if _is_null(Clone::clone(&module)) {
            return Err(_npe("Null module object"));
        }
        let name_s = module.__get_name();
        if _is_null(Clone::clone(&name_s)) {
            return Err(_iae("Module name cannot be null"));
        }
        let name = format!("{}", name_s);
        if name.is_empty() {
            return Err(_iae(&format!("Invalid module name: {}", name)));
        }
        let loader = _identity(Object::from(module.__get_loader()));
        if name == "java.base" {
            return Err(if loader != 0 {
                _iae("Class loader must be the boot class loader")
            } else {
                _ise("Module java.base is already defined")
            });
        }
        let platform = crate::jdk::internal::loader::ClassLoaders::platformClassLoader()
            .map(|l| _identity(Object::from(l))).unwrap_or(0);
        let mut packages = Vec::new();
        let raw: Vec<Object> = if pns.is_jvm_null() { Vec::new() } else { pns.to_vec() };
        for p in raw {
            if _is_null(Clone::clone(&p)) || !p.0.is_instance_of("java/lang/String") {
                return Err(_iae("Bad package name"));
            }
            let pkg = format!("{}", String::from(p)).replace('.', "/");
            if !_valid_package(&pkg) {
                return Err(_iae(&format!("Invalid package name: {} for module: {}", pkg, name)));
            }
            if loader != 0 && loader != platform && (pkg == "java" || pkg.starts_with("java/")) {
                // HotSpot `loader_name_and_id`：无名加载器取类名，id 为身份散列
                let lo = Object::from(module.__get_loader());
                let lname = format!("{} @{:x}", lo.0.__class_name().replace('/', "."),
                    super::object::__identity_hash(lo.0.__identity()));
                return Err(_iae(&format!("Class loader (instance of): {} tried to define prohibited package name: {}",
                    lname, pkg.replace('/', "."))));
            }
            packages.push(pkg);
        }
        _vm_modules(|t| {
            let same_loader = |e: &&VmModule| e.loader == loader;
            if t.iter().filter(same_loader).any(|e| e.name == name) {
                return Err(_ise(&format!("Module {} is already defined", name)));
            }
            if let Some((pkg, other)) = packages.iter()
                .find_map(|p| t.iter().filter(same_loader).find(|e| e.packages.contains(p)).map(|e| (p, e)))
            {
                return Err(_ise(&format!(
                    "Package {} for module {} is already in another module, {}, defined to the class loader",
                    pkg.replace('/', "."), name, other.name)));
            }
            t.push(VmModule { module: Clone::clone(&module), loader, name: name.clone(), open: is_open, packages });
            Ok(())
        })
    }

    /// native `addReads0(from, to)`：HotSpot `Modules::add_reads_module`——from 为 null → NPE；
    /// 读边只服务 VM 链接期访问检查（见本节说明），校验后无可观测状态。
    #[jvm_native]
    pub fn addReads0(from: Module, _to: Module) -> Result<()> {
        if _is_null(from) {
            return Err(_npe("from_module is null"));
        }
        Ok(())
    }

    /// native `addExports0(from, pn, to)`：HotSpot `Modules::add_module_exports`（限定导出）。
    #[jvm_native]
    pub fn addExports0(from: Module, pn: String, _to: Module) -> Result<()> {
        Self::__add_exports(from, pn)
    }

    /// native `addExportsToAll0(from, pn)`：HotSpot `add_module_exports`（to 为 null，无限定导出）。
    #[jvm_native]
    pub fn addExportsToAll0(from: Module, pn: String) -> Result<()> {
        Self::__add_exports(from, pn)
    }

    /// native `addExportsToAllUnnamed0(from, pn)`：HotSpot `add_module_exports_to_all_unnamed`——
    /// 检查次序为 module、package（与 add_module_exports 相反）。
    #[jvm_native]
    pub fn addExportsToAllUnnamed0(from: Module, pn: String) -> Result<()> {
        if _is_null(Clone::clone(&from)) {
            return Err(_npe("module is null"));
        }
        if _is_null(Clone::clone(&pn)) {
            return Err(_npe("package is null"));
        }
        _check_export(&from, &pn, "module")
    }

    fn __add_exports(from: Module, pn: String) -> Result<()> {
        if _is_null(Clone::clone(&pn)) {
            return Err(_npe("package is null"));
        }
        if _is_null(Clone::clone(&from)) {
            return Err(_npe("from_module is null"));
        }
        _check_export(&from, &pn, "from_module")
    }
}
