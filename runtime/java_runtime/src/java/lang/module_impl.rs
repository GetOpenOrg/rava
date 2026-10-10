use crate::prelude::*;
use super::Module;

// java.lang.Module 伴生：VM 模块表（HotSpot `Modules` / `ModuleEntryTable` 的可观测部分）。
//
// 模块系统按字节码运行：引导层（initPhase2）在构建期求值，其模块对象、读边与导出表都在映像中；
// 映像中 defineModule0 登记过的模块是本表的初值（首次访问时取自映像表），运行期新定义的层
// 经 `defineModule0` 追加。类镜像的模块（`Class.module`，VM 在建镜像时写入）由 `Class.__vm_module`
// 钩子按「定义加载器 + 包」查本表落地，未登记的包归其加载器的无名模块。

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
    /// defineModule0 的 location 实参（引导层模块为 `jrt:/<模块名>`）；null → None
    location: Option<std::string::String>,
    packages: Vec<std::string::String>,
}

fn _vm_modules<R>(f: impl FnOnce(&mut Vec<VmModule>) -> R) -> R {
    crate::__process_static! {
        static MODULES: crate::sync_model::__RefSlot<Vec<VmModule>> = crate::sync_model::__RefSlot::new(_image_modules());
    }
    MODULES.with(|t| f(&mut t.borrow_mut()))
}

/// 映像中 defineModule0 登记过的模块：VM 模块表的初值（首次访问模块表时取自映像表，启动时不逐项登记）
fn _image_modules() -> Vec<VmModule> {
    crate::image_rt::modules()
        .iter()
        .map(|m| {
            let module = Module::from(crate::image_rt::object(m.module));
            let name = format!("{}", module.__get_name());
            let loader = m.loader.map_or(0, |l| _identity(crate::image_rt::object(l)));
            let location = m.location.map(str::to_string);
            let packages = m.packages.iter().map(|p| p.to_string()).collect();
            VmModule { module, loader, name, open: m.open, location, packages }
        })
        .collect()
}

impl Module {
    /// 引导加载器定义的包 `pkg`（内部形式）所属模块的位置（HotSpot `ClassLoader::get_system_package`
    /// 取包所在模块的 location）；未登记 → None
    pub fn __vm_boot_package_location(pkg: &str) -> Option<std::string::String> {
        _vm_modules(|t| {
            t.iter()
                .find(|e| e.loader == 0 && e.packages.iter().any(|x| x == pkg))
                .and_then(|e| e.location.clone())
        })
    }

    /// 定义加载器为 `loader` 的包 `pkg`（内部形式）所属的已登记模块；`pkg` 为 None（基本类型）→ java.base
    pub fn __vm_package_module(loader: Object, pkg: Option<&str>) -> Option<Module> {
        let loader = _identity(loader);
        _vm_modules(|t| {
            t.iter()
                .find(|e| match pkg {
                    Some(p) => e.loader == loader && e.packages.iter().any(|x| x == p),
                    None => e.loader == 0 && e.name == "java.base",
                })
                .map(|e| Clone::clone(&e.module))
        })
    }
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
    pub fn defineModule0(module: Module, is_open: bool, _version: String, location: String,
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
            let location = if location.is_jvm_null() { None } else { Some(format!("{}", location)) };
            t.push(VmModule { module: Clone::clone(&module), loader, name: name.clone(), open: is_open, location, packages });
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
