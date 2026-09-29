//! 类路径：有序档案列表，首个命中者胜出；解析结果缓存（含「不存在」）。
//!
//! JDK 语料的 jmod 顺序与 `codegen/jdk_resolver.py` 一致：优先级清单在前，其余按名排序。

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

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

pub struct ClassPath {
    archives: RefCell<Vec<(Origin, Archive)>>,
    /// binary name → 档案下标（首个命中者）
    index: HashMap<String, usize>,
    cache: RefCell<HashMap<String, Option<Rc<ClassFile>>>>,
    /// 解析失败的类（名字, 错误）：丢类必须可观测
    failures: RefCell<Vec<(String, String)>>,
    java_home: Option<PathBuf>,
}

impl ClassPath {
    pub fn new() -> Self {
        ClassPath {
            archives: RefCell::new(Vec::new()),
            index: HashMap::new(),
            cache: RefCell::new(HashMap::new()),
            failures: RefCell::new(Vec::new()),
            java_home: None,
        }
    }

    pub fn java_home(&self) -> Option<&Path> {
        self.java_home.as_deref()
    }

    /// 追加一个档案（jmod / jar / 类目录）；先加入者优先
    pub fn add(&mut self, origin: Origin, path: &Path) -> Result<(), classfile::Error> {
        let a = Archive::open(path)?;
        let idx = self.archives.borrow().len();
        for n in a.class_names() {
            self.index.entry(n).or_insert(idx);
        }
        self.archives.borrow_mut().push((origin, a));
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

    pub fn contains(&self, name: &str) -> bool {
        self.index.contains_key(name)
    }

    pub fn origin(&self, name: &str) -> Option<Origin> {
        self.index.get(name).map(|&i| self.archives.borrow()[i].0)
    }

    /// 某角色档案里的全部类名（排序）
    pub fn names_of(&self, origin: Origin) -> Vec<String> {
        let archives = self.archives.borrow();
        let mut v: Vec<String> = self
            .index
            .iter()
            .filter(|(_, &i)| archives[i].0 == origin)
            .map(|(n, _)| n.clone())
            .collect();
        v.sort();
        v
    }

    pub fn get(&self, name: &str) -> Option<Rc<ClassFile>> {
        if let Some(c) = self.cache.borrow().get(name) {
            return c.clone();
        }
        let loaded = self.load(name);
        self.cache.borrow_mut().insert(name.to_string(), loaded.clone());
        loaded
    }

    fn load(&self, name: &str) -> Option<Rc<ClassFile>> {
        let &i = self.index.get(name)?;
        let bytes = match self.archives.borrow_mut()[i].1.read_class(name) {
            Ok(Some(b)) => b,
            Ok(None) => return None,
            Err(e) => {
                self.failures.borrow_mut().push((name.to_string(), e.to_string()));
                return None;
            }
        };
        match classfile::parse(&bytes) {
            Ok(cf) => Some(Rc::new(cf)),
            Err(e) => {
                self.failures.borrow_mut().push((name.to_string(), e.to_string()));
                None
            }
        }
    }

    /// 读取原始字节（golden 对照等）
    pub fn bytes(&self, name: &str) -> Option<Vec<u8>> {
        let &i = self.index.get(name)?;
        self.archives.borrow_mut()[i].1.read_class(name).ok().flatten()
    }

    /// 模块资源（非类文件）：按档案顺序首个命中
    pub fn resource(&self, path: &str) -> Option<Vec<u8>> {
        for (_, a) in self.archives.borrow_mut().iter_mut() {
            if let Ok(Some(b)) = a.read_resource(path) {
                return Some(b);
            }
        }
        None
    }

    pub fn failures(&self) -> Vec<(String, String)> {
        self.failures.borrow().clone()
    }
}

impl Default for ClassPath {
    fn default() -> Self {
        Self::new()
    }
}
