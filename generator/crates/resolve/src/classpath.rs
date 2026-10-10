//! 类路径：有序档案列表，首个命中者胜出；解析结果缓存（含「不存在」）。
//!
//! JDK 语料的 jmod 顺序：优先级清单在前，其余按名排序。

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};

use classfile::archive::Archive;
use classfile::ClassFile;

use crate::hierarchy::package_of;

const JMOD_PRIORITY: [&str; 6] = [
    "java.base.jmod",
    "java.desktop.jmod",
    "java.logging.jmod",
    "java.xml.jmod",
    "java.sql.jmod",
    "java.net.http.jmod",
];

/// 档案的来源角色（闭包分析按角色区分用户类 / 库类 / JDK 类）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Origin {
    User,
    Lib,
    Jdk,
    /// 运行时镜像独有类（jlink 预生成）/ VM 支持类
    Image,
    /// 预定义类：程序运行期经类定义 native（`ClassLoader.defineClass0/1/2`）定义的类，构建期取得其类文件
    /// （训练运行记录或用户项目提供），按内容寻址进用户 crate（docs/plans/2026-10-10-xsltc-translet.md §4）。
    /// 属用户域（按字节码翻译、程序私有、不进档案），但不在应用类路径上：不是类路径资源、不由应用加载器
    /// 定义，只作为类定义 native 的返回值进入闭包
    Predefined,
}

impl Origin {
    /// 程序私有的类（用户类与预定义类）：用户域，不进档案
    pub fn is_program(self) -> bool {
        matches!(self, Origin::User | Origin::Predefined)
    }
}

/// 依赖锁条目喂入的库元数据（J2 由 `deps.lock.toml` 填充）：坐标与显式模块名兜底
#[derive(Debug, Clone, Default)]
pub struct LibMeta {
    /// Maven 坐标 `group:artifact:version`（可缺省）
    pub coordinate: Option<String>,
    /// 锁条目显式给出的模块名（命名兜底链的最后一环之前）
    pub module: Option<String>,
    /// jar 内容摘要（依赖锁给出；profile.json modules[].jars 用）
    pub sha256: Option<String>,
}

/// 档案的模块视图：模块描述符（jmod / 模块化 jar）与 `META-INF/services` 配置
pub struct ArchiveView {
    pub origin: Origin,
    pub module: Option<classfile::module::ModuleDecl>,
    /// (服务二进制名, 配置文件字节)
    pub services: Vec<(String, Vec<u8>)>,
}

/// 被遮蔽的类：类名、被遮蔽副本所在档案、拥有该包的 JDK 模块
pub struct Shadowed {
    pub class: String,
    pub archive: PathBuf,
    pub owner: String,
}

/// 跨档案重复类：类名、胜出档案（类路径序在前）、落选档案
pub struct Duplicate {
    pub class: String,
    pub winner: PathBuf,
    pub loser: PathBuf,
}

pub struct ClassPath {
    /// 构建单元目标 release（多版本 jar 视图）
    release: u32,
    /// 档案（读取需可变游标：并行发射下以互斥锁串行化）
    archives: Mutex<Vec<Archive>>,
    /// 档案下标 → 来源角色（与 `archives` 同序；只读，免锁）
    origins: Vec<Origin>,
    /// binary name → 档案下标（首个命中者）
    index: HashMap<String, usize>,
    cache: RwLock<HashMap<String, Option<Arc<ClassFile>>>>,
    /// 解析失败的类（名字, 错误）：丢类必须可观测
    failures: Mutex<Vec<(String, String)>>,
    java_home: Option<PathBuf>,
    /// 档案下标 → 模块名（`module_of` 惰性填充）
    module_names: std::sync::OnceLock<Vec<Option<String>>>,
    /// JDK 侧无 module-info 的类目录的档案下标 → 所属模块名：镜像改写类目录（覆盖档案，登记名即 jmod 模块）、
    /// 镜像独有类目录与 VM 支持类目录（[`crate::image`] 按模块给出，目录名即模块名）
    overlay_modules: HashMap<usize, String>,
    /// 依赖锁喂入的库档案路径 → 元数据
    lib_meta: HashMap<PathBuf, LibMeta>,
    /// JDK 具名模块包遮蔽的用户 / 库类（`shadow_jdk_owned_packages` 填充）
    shadowed: Vec<Shadowed>,
    /// 跨档案重复类（后加入者落选）
    duplicates: Vec<Duplicate>,
}

impl ClassPath {
    pub fn new(release: u32) -> Self {
        ClassPath {
            release,
            archives: Mutex::new(Vec::new()),
            origins: Vec::new(),
            index: HashMap::new(),
            cache: RwLock::new(HashMap::new()),
            failures: Mutex::new(Vec::new()),
            java_home: None,
            module_names: std::sync::OnceLock::new(),
            overlay_modules: HashMap::new(),
            lib_meta: HashMap::new(),
            shadowed: Vec::new(),
            duplicates: Vec::new(),
        }
    }

    pub fn release(&self) -> u32 {
        self.release
    }

    pub fn java_home(&self) -> Option<&Path> {
        self.java_home.as_deref()
    }

    /// 依赖锁条目元数据登记（档案加入前或后均可，按路径匹配）
    pub fn set_lib_meta(&mut self, jar: &Path, meta: LibMeta) {
        self.lib_meta.insert(jar.to_path_buf(), meta);
    }

    /// 档案路径 → 依赖锁元数据（命名兜底链消费）
    pub fn lib_meta(&self, jar: &Path) -> Option<&LibMeta> {
        self.lib_meta.get(jar)
    }

    /// 追加一个档案（jmod / jar / 类目录）；先加入者优先；同名重复类记录在案
    pub fn add(&mut self, origin: Origin, path: &Path) -> Result<(), classfile::Error> {
        let a = Archive::open(path, self.release)?;
        let idx = self.origins.len();
        for n in a.class_names() {
            match self.index.get(&n) {
                None => {
                    self.index.insert(n, idx);
                }
                Some(&w) => {
                    // 镜像覆盖（add_overlay）与 JDK 档案不算重复；用户 / 库跨档案重复可观测
                    if !matches!((self.origins[w], origin), (Origin::Jdk, _) | (_, Origin::Jdk)) {
                        let winner_path = lock(&self.archives).get(w).map(|a| a.path.clone()).unwrap_or_default();
                        self.duplicates.push(Duplicate { class: n, winner: winner_path, loser: path.to_path_buf() });
                    }
                }
            }
        }
        self.archives.get_mut().unwrap_or_else(|e| e.into_inner()).push(a);
        self.origins.push(origin);
        // 镜像独有类 / VM 支持类目录按模块给出（`<根>/<模块>/`，见 [`crate::image`]）：目录名即所属模块。
        // VM 支持类可以落在模块原本没有的包里（如按协议名命名的 URL 处理器包），包归属由此登记，不按包借用
        if origin == Origin::Image && path.is_dir() {
            if let Some(m) = path.file_name().and_then(|n| n.to_str()) {
                self.overlay_modules.insert(idx, m.to_string());
            }
        }
        self.module_names = std::sync::OnceLock::new();
        Ok(())
    }

    /// 追加 JDK 语料（`<java_home>/jmods/*.jmod`）
    pub fn add_jdk(&mut self, java_home: &Path) -> Result<(), classfile::Error> {
        let dir = java_home.join("jmods");
        let rd = std::fs::read_dir(&dir).map_err(|e| classfile::Error::Io(dir.display().to_string(), e.to_string()))?;
        let mut all: Vec<String> = rd
            .flatten()
            .filter_map(|e| e.file_name().into_string().ok())
            .filter(|n| n.ends_with(".jmod"))
            .collect();
        all.sort();
        let mut ordered: Vec<String> = JMOD_PRIORITY.iter().filter(|p| all.iter().any(|a| a == *p)).map(|s| s.to_string()).collect();
        ordered.extend(all.into_iter().filter(|a| !JMOD_PRIORITY.contains(&a.as_str())));
        for j in ordered {
            self.add(Origin::Jdk, &dir.join(j))?;
        }
        // 镜像改写类（jlink 插件改写、与 jmod 字节不同者）：JVM 执行镜像里的版本，覆盖 jmod 中的同名类
        for (module, d) in crate::image::rewritten_dirs(java_home) {
            self.add_overlay(Origin::Jdk, &d, module)?;
        }
        self.java_home = Some(java_home.to_path_buf());
        Ok(())
    }

    /// 追加覆盖档案：其中的类取代已加入档案中的同名类；`module_of` 报告给定模块名
    fn add_overlay(&mut self, origin: Origin, path: &Path, module: String) -> Result<(), classfile::Error> {
        let a = Archive::open(path, self.release)?;
        let idx = self.origins.len();
        for n in a.class_names() {
            self.index.insert(n, idx);
        }
        self.archives.get_mut().unwrap_or_else(|e| e.into_inner()).push(a);
        self.origins.push(origin);
        self.overlay_modules.insert(idx, module);
        self.module_names = std::sync::OnceLock::new();
        Ok(())
    }

    /// 档案清单（加入序）：(来源角色, 路径)。跨运行缓存按它对全部输入取指纹
    pub fn archives(&self) -> Vec<(Origin, PathBuf)> {
        let a = lock(&self.archives);
        self.origins.iter().zip(a.iter()).map(|(o, a)| (*o, a.path.clone())).collect()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.index.contains_key(name)
    }

    pub fn origin(&self, name: &str) -> Option<Origin> {
        self.index.get(name).map(|&i| self.origins[i])
    }

    /// 某角色档案里的全部类名（排序）
    pub fn names_of(&self, origin: Origin) -> Vec<String> {
        let mut v: Vec<String> = self
            .index
            .iter()
            .filter(|(_, &i)| self.origins[i] == origin)
            .map(|(n, _)| n.clone())
            .collect();
        v.sort();
        v
    }

    pub fn get(&self, name: &str) -> Option<Arc<ClassFile>> {
        if let Some(c) = read(&self.cache).get(name) {
            return c.clone();
        }
        // 未命中：持写锁复查后解析——每个类只解析一次（失败只登记一次），所有调用方拿到同一实例
        let mut cache = lock_w(&self.cache);
        if let Some(c) = cache.get(name) {
            return c.clone();
        }
        let loaded = self.load(name);
        cache.insert(name.to_string(), loaded.clone());
        loaded
    }

    fn load(&self, name: &str) -> Option<Arc<ClassFile>> {
        let &i = self.index.get(name)?;
        let read = lock(&self.archives)[i].read_class(name);
        let bytes = match read {
            Ok(Some(b)) => b,
            Ok(None) => return None,
            Err(e) => {
                lock(&self.failures).push((name.to_string(), e.to_string()));
                return None;
            }
        };
        match classfile::parse(&bytes) {
            Ok(cf) => Some(Arc::new(cf)),
            Err(e) => {
                lock(&self.failures).push((name.to_string(), e.to_string()));
                None
            }
        }
    }

    /// 读取原始字节
    pub fn bytes(&self, name: &str) -> Option<Vec<u8>> {
        let &i = self.index.get(name)?;
        lock(&self.archives)[i].read_class(name).ok().flatten()
    }

    /// 模块资源（非类文件）：按档案顺序首个命中
    pub fn resource(&self, path: &str) -> Option<Vec<u8>> {
        for a in lock(&self.archives).iter_mut() {
            if let Ok(Some(b)) = a.read_resource(path) {
                return Some(b);
            }
        }
        None
    }

    /// 各档案的模块描述符与类路径服务配置（按档案序）。描述符解析失败记入 failures
    pub fn module_views(&self) -> Vec<ArchiveView> {
        let mut out = Vec::new();
        let mut archives = lock(&self.archives);
        for (origin, a) in self.origins.iter().zip(archives.iter_mut()) {
            let module = match a.read_module_info() {
                Ok(Some(b)) => match classfile::module::parse_module_info(&b) {
                    Ok(m) => m,
                    Err(e) => {
                        lock(&self.failures).push((format!("{}!module-info", a.path.display()), e.to_string()));
                        None
                    }
                },
                _ => None,
            };
            let services = a.service_files();
            out.push(ArchiveView { origin: *origin, module, services });
        }
        out
    }

    /// 类所在档案的模块名（模块化档案：jmod / 模块化 jar；非模块档案与未知类 → None）。
    /// 各档案的 module-info 只解析一次
    pub fn module_of(&self, name: &str) -> Option<String> {
        let &i = self.index.get(name)?;
        self.archive_module(i)
    }

    /// 档案级模块名（jmod / 模块化 jar 的描述符名；overlay 登记名另查 `overlay_modules`）
    fn ensure_module_names(&self) -> &Vec<Option<String>> {
        self.module_names.get_or_init(|| {
            let mut archives = lock(&self.archives);
            archives
                .iter_mut()
                .map(|a| match a.read_module_info() {
                    Ok(Some(b)) => classfile::module::parse_module_info(&b).ok().flatten().map(|m| m.name),
                    _ => None,
                })
                .collect()
        })
    }

    /// 类所在档案的下标（首个命中者；[`Self::module_views`] / [`Self::archives`] 同序）
    pub fn archive_of(&self, name: &str) -> Option<usize> {
        self.index.get(name).copied()
    }

    /// 全部已索引类：(binary name, 档案下标)，无序
    pub fn indexed(&self) -> impl Iterator<Item = (&str, usize)> {
        self.index.iter().map(|(n, &i)| (n.as_str(), i))
    }

    /// JDK 侧无 module-info 的类目录（镜像改写 / 镜像独有 / VM 支持类目录）登记的模块名
    pub fn overlay_module(&self, idx: usize) -> Option<&str> {
        self.overlay_modules.get(&idx).map(String::as_str)
    }

    /// JDK 侧资源：只在 JDK / 镜像档案中查（模块资源；用户与库档案的文件属于应用类路径，由
    /// [`ClassPath::class_path_files`] 承载）。`<类名>.class` 取该 JDK 类的解析胜出字节（与类本身同源，
    /// 镜像改写类优先于 jmod）；其余按档案顺序首个命中。返回 (所属模块, 字节)：模块取命中档案的
    /// 模块名（jmod 描述符名或 overlay 登记名）；命中档案无模块名时为 None（模块资源必属某个具名模块）
    pub fn jdk_resource(&self, path: &str) -> Option<(String, Vec<u8>)> {
        if let Some(name) = path.strip_suffix(".class") {
            return match self.origin(name) {
                Some(Origin::Jdk | Origin::Image) => Some((self.module_of(name)?, self.bytes(name)?)),
                _ => None,
            };
        }
        // 先在档案锁内找首个命中，放锁后再取模块名（模块名惰性解析同样要取档案锁）
        let (i, b) = {
            let mut a = lock(&self.archives);
            a.iter_mut()
                .enumerate()
                .filter(|(i, _)| matches!(self.origins[*i], Origin::Jdk | Origin::Image))
                .find_map(|(i, arch)| arch.read_resource(path).ok().flatten().map(|b| (i, b)))?
        };
        Some((self.archive_module(i)?, b))
    }

    /// 档案的模块名：overlay 登记名优先，其次档案描述符名
    fn archive_module(&self, i: usize) -> Option<String> {
        if let Some(m) = self.overlay_modules.get(&i) {
            return Some(m.clone());
        }
        self.ensure_module_names().get(i).cloned().flatten()
    }

    /// JDK 具名模块 `module` 的内容是否含资源 `path`（参考 JDK 运行期映像的模块读取器语义：只查该模块所在的
    /// JDK / 镜像档案；`<类名>.class` 取该类的解析胜出档案）
    pub fn module_has_resource(&self, module: &str, path: &str) -> bool {
        if let Some(name) = path.strip_suffix(".class") {
            return matches!(self.origin(name), Some(Origin::Jdk | Origin::Image)) && self.module_of(name).as_deref() == Some(module);
        }
        let names = self.ensure_module_names();
        let mut a = lock(&self.archives);
        for (i, arch) in a.iter_mut().enumerate() {
            if !matches!(self.origins[i], Origin::Jdk | Origin::Image) {
                continue;
            }
            let m = self.overlay_modules.get(&i).map(String::as_str).or_else(|| names.get(i).and_then(|x| x.as_deref()));
            if m == Some(module) && matches!(arch.read_resource(path), Ok(Some(_))) {
                return true;
            }
        }
        false
    }

    /// 应用类路径（用户与库档案，加入序）的全部文件：(资源名, 字节)，按名稳定排序——同名按类路径序。
    /// 读取失败记入 failures
    pub fn class_path_files(&self) -> Vec<(String, Vec<u8>)> {
        let mut out = Vec::new();
        let mut a = lock(&self.archives);
        for (i, arch) in a.iter_mut().enumerate() {
            if !matches!(self.origins[i], Origin::User | Origin::Lib) {
                continue;
            }
            match arch.all_files() {
                Ok(files) => out.extend(files),
                Err(e) => lock(&self.failures).push((arch.path.display().to_string(), e.to_string())),
            }
        }
        out.sort_by(|x: &(String, Vec<u8>), y| x.0.cmp(&y.0));
        out
    }

    /// 某个档案内的资源（非类文件）
    pub fn resource_in(&self, idx: usize, path: &str) -> Option<Vec<u8>> {
        lock(&self.archives).get_mut(idx)?.read_resource(path).ok().flatten()
    }

    pub fn failures(&self) -> Vec<(String, String)> {
        lock(&self.failures).clone()
    }

    /// JDK 具名模块包遮蔽的用户 / 库类（`shadow_jdk_owned_packages` 之后可查）
    pub fn shadowed(&self) -> &[Shadowed] {
        &self.shadowed
    }

    /// 跨档案重复类（类路径序在前者胜出；JDK 档案与镜像覆盖不在此列）
    pub fn duplicates(&self) -> &[Duplicate] {
        &self.duplicates
    }

    /// JDK / 镜像具名模块拥有的包 → 模块名（JPMS：一个包只属于一个具名模块）
    fn jdk_package_owners(&self) -> HashMap<String, String> {
        let names = self.ensure_module_names();
        let mut out: HashMap<String, String> = HashMap::new();
        let archives = lock(&self.archives);
        for (class, &i) in self.index.iter() {
            if !matches!(self.origins[i], Origin::Jdk | Origin::Image) {
                continue;
            }
            if archives.get(i).is_none() {
                continue;
            }
            let named = self.overlay_modules.get(&i).cloned().or_else(|| names.get(i).cloned().flatten());
            let Some(m) = named else { continue };
            out.entry(package_of(class).to_string()).or_insert(m);
        }
        out
    }

    /// 全部档案加入后调用一次：JPMS 包归属遮蔽——用户 / 库档案中属于 JDK 具名模块
    /// 所拥有包的类不入索引（记入 [`Self::shadowed`]），JDK 侧同名类接管可见性；
    /// JDK 侧无同名类时该类整体不可见。此后的全部查询以遮蔽后的索引为准
    pub fn shadow_jdk_owned_packages(&mut self) {
        let owners = self.jdk_package_owners();
        if owners.is_empty() {
            return;
        }
        let candidates: Vec<(String, usize, String)> = self
            .index
            .iter()
            .filter(|(_, &i)| matches!(self.origins[i], Origin::User | Origin::Lib))
            .filter_map(|(class, &i)| {
                let owner = owners.get(package_of(class))?;
                Some((class.clone(), i, owner.clone()))
            })
            .collect();
        if candidates.is_empty() {
            return;
        }
        for (class, i, owner) in candidates {
            let archive = lock(&self.archives).get(i).map(|a| a.path.clone()).unwrap_or_default();
            // JDK / 镜像侧同名类（最早加入且含有该类者）接管；没有则从索引移除
            let takeover = {
                let archives = lock(&self.archives);
                self.origins
                    .iter()
                    .zip(archives.iter())
                    .enumerate()
                    .find(|(_, (o, a))| matches!(o, Origin::Jdk | Origin::Image) && a.contains(&class))
                    .map(|(j, _)| j)
            };
            match takeover {
                Some(j) => {
                    self.index.insert(class.clone(), j);
                }
                None => {
                    self.index.remove(&class);
                }
            }
            self.shadowed.push(Shadowed { class, archive, owner });
        }
        // 遮蔽改变了解析结果：清掉此前可能已缓存的解析（正常流程在本方法后才首次查询）
        lock_w(&self.cache).clear();
    }
}

/// 锁中毒只意味着别的线程在持锁时 panic（整个进程随之失败）；数据本身仍一致，照常取用
fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

fn read<T>(m: &RwLock<T>) -> std::sync::RwLockReadGuard<'_, T> {
    m.read().unwrap_or_else(|e| e.into_inner())
}

fn lock_w<T>(m: &RwLock<T>) -> std::sync::RwLockWriteGuard<'_, T> {
    m.write().unwrap_or_else(|e| e.into_inner())
}

