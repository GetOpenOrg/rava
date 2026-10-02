//! 类的定义加载器（HotSpot `java_lang_Class::create_mirror` 写入镜像的 `classLoader`）。
//!
//! - 用户类与库类：应用加载器；
//! - JDK 类：按所在模块查 JDK 自己的模块 → 加载器映射（清单 `[vm_state.loader_map]` 指向的类，其 `<clinit>`
//!   把字符串常量集合写入 boot / platform 两个静态字段）。在 boot 集合中的为引导加载器（null），在 platform
//!   集合中的为平台加载器，其余 JDK 模块为应用加载器；
//! - 运行时镜像独有类 / VM 支持类、无模块归属的 JDK 类：引导加载器。
//!
//! 生成器据此给类块写 `defining_loader` 属性，运行时镜像的读取钩子按它填充定义加载器。

use std::collections::HashSet;

use classfile::{op, Const, Operand};
use resolve::classpath::{ClassPath, Origin};

use crate::manifest::LoaderMapSrc;

/// 内建加载器
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Loader {
    Boot,
    Platform,
    App,
}

impl Loader {
    /// 类块属性值（引导加载器不写属性）
    pub fn attr(self) -> Option<&'static str> {
        match self {
            Loader::Boot => None,
            Loader::Platform => Some("platform"),
            Loader::App => Some("app"),
        }
    }
}

#[derive(Debug, Default)]
pub struct DefiningLoaders {
    boot: HashSet<String>,
    platform: HashSet<String>,
}

impl DefiningLoaders {
    /// 读映射类 `<clinit>`：两个静态字段写入点之前出现的字符串常量归入该字段的集合
    pub fn new(cp: &ClassPath, src: Option<&LoaderMapSrc>) -> Self {
        let mut out = DefiningLoaders::default();
        let Some(src) = src else { return out };
        let Some(cf) = cp.get(&src.class) else { return out };
        let Some(code) = cf.method("<clinit>", "()V").and_then(|m| m.code.as_ref()) else { return out };
        let mut pending: Vec<String> = Vec::new();
        for x in &code.insns {
            match &x.operand {
                Operand::Ldc(Const::String(s)) => pending.push(s.clone()),
                Operand::Field(f) if x.opcode == op::PUTSTATIC && f.owner == src.class => {
                    let set = std::mem::take(&mut pending);
                    if f.name == src.boot {
                        out.boot.extend(set);
                    } else if f.name == src.platform {
                        out.platform.extend(set);
                    }
                }
                _ => {}
            }
        }
        out
    }

    pub fn loader_of(&self, cp: &ClassPath, class: &str) -> Loader {
        match cp.origin(class) {
            Some(Origin::User | Origin::Lib) => Loader::App,
            Some(Origin::Jdk) => match cp.module_of(class) {
                Some(m) if self.boot.contains(&m) => Loader::Boot,
                Some(m) if self.platform.contains(&m) => Loader::Platform,
                Some(_) if !self.boot.is_empty() => Loader::App,
                _ => Loader::Boot,
            },
            Some(Origin::Image) | None => Loader::Boot,
        }
    }
}
