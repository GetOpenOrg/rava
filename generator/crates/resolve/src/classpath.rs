//! 类路径：有序档案列表，首个命中者胜出；解析结果缓存（含「不存在」）。
//!
//! JDK 语料的 jmod 顺序与 `codegen/jdk_resolver.py` 一致：优先级清单在前，其余按名排序。

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};

use classfile::archive::Archive;
use classfile::ClassFile;

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
}

/// 档案的模块视图：模块描述符（jmod / 模块化 jar）与 `META-INF/services` 配置
pub struct ArchiveView {
    pub origin: Origin,
    pub module: Option<classfile::module::ModuleDecl>,
    /// (服务二进制名, 配置文件字节)
    pub services: Vec<(String, Vec<u8>)>,
}

pub struct ClassPath {
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
}

impl ClassPath {
    pub fn new() -> Self {
        ClassPath {
            archives: Mutex::new(Vec::new()),
            origins: Vec::new(),
            index: HashMap::new(),
            cache: RwLock::new(HashMap::new()),
            failures: Mutex::new(Vec::new()),
            java_home: None,
        }
    }

    pub fn java_home(&self) -> Option<&Path> {
        self.java_home.as_deref()
    }

    /// 追加一个档案（jmod / jar / 类目录）；先加入者优先
    pub fn add(&mut self, origin: Origin, path: &Path) -> Result<(), classfile::Error> {
        let a = Archive::open(path)?;
        let idx = self.origins.len();
        for n in a.class_names() {
            self.index.entry(n).or_insert(idx);
        }
        self.archives.get_mut().unwrap_or_else(|e| e.into_inner()).push(a);
        self.origins.push(origin);
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
        self.java_home = Some(java_home.to_path_buf());
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

    /// 读取原始字节（golden 对照等）
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

    pub fn failures(&self) -> Vec<(String, String)> {
        lock(&self.failures).clone()
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

impl Default for ClassPath {
    fn default() -> Self {
        Self::new()
    }
}
