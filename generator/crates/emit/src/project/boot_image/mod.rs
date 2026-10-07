//! 构建期引导映像的物化（计划 2026-10-05-boot-image-evaluator §5.5.2 D1–D5）。
//!
//! 输入是闭包分析器导出的 [`ImageData`]（活对象集已由联合不动点求出），输出根门面 crate 的
//! `boot_image.rs`：
//!
//! - **映像区**：单个 `#[repr(C)]` 静态结构 `BOOT_IMAGE`，每个活对象一个字段（`__ImageObj<X__inner>` /
//!   `__ImageArr<T, S>`），对象头带常驻计数与构建期身份哈希，字段初值是常量求值的映像内引用；
//! - **启动序列** `__boot_image_start`：登记映像区 → VM 单元 → 链接（常量不可表达的引用：类镜像、接口
//!   视图、宿主相关值）→ 宿主值改写 → 驻留 → 静态字段初值 → 构建期初始化标记 → 按构建期次序重放
//!   重定位 / 重算 / 残差调用（档位随之切换）。
//!
//! 类名只来自映像数据与类文件，生成器中不出现 JDK 类名字面量。

pub(crate) mod layout;
mod start;
mod values;

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use closure::image::{IBody, IObj, IVal, ImageData};

use self::layout::{instance_layout, Slot};
use super::fs::Writer;
use crate::ctx::EmitCtx;
use crate::error::{EmitError, Result};
use crate::phase2::uses::use_path;
use crate::phase2::Emissions;

/// 映像模块名（根门面 crate 的 `pub mod`）
pub const MODULE: &str = "boot_image";
/// 启动入口（`main` 在创建 VM 之后调用）
pub const START_FN: &str = "__boot_image_start";

/// 物化计划：活对象的分类与各类的存储布局
pub(crate) struct Plan<'c, 'a> {
    pub ctx: &'c EmitCtx<'a>,
    pub ems: &'c Emissions,
    pub d: &'c ImageData,
    pub root: String,
    pub live: BTreeSet<u32>,
    /// 物化为映像区字段的对象（活、非类镜像、非占位）
    pub mat: BTreeSet<u32>,
    pub layouts: BTreeMap<String, Vec<Slot>>,
    /// 类 → 实现层模块路径（存储类型 `X__inner` 随存储层进实现 crate）
    pub homes: &'c BTreeMap<String, String>,
}

impl<'c, 'a> Plan<'c, 'a> {
    fn new(ctx: &'c EmitCtx<'a>, ems: &'c Emissions, d: &'c ImageData, homes: &'c BTreeMap<String, String>) -> Result<Self> {
        let live: BTreeSet<u32> = d.live.iter().copied().collect();
        let mat: BTreeSet<u32> =
            live.iter().copied().filter(|&i| d.objs.get(i as usize).is_some_and(|o| o.mirror.is_none() && !o.placeholder)).collect();
        let root = ctx.crates().root().to_string();
        let mut p = Plan { ctx, ems, d, root, live, mat, layouts: BTreeMap::new(), homes };
        let tys: BTreeSet<&str> = p.mat.iter().map(|&i| d.objs[i as usize].ty.as_str()).filter(|t| !t.starts_with('[')).collect();
        for t in tys {
            if t == ty::consts::OBJECT {
                p.layouts.insert(t.to_string(), Vec::new());
                continue;
            }
            let ci = p.generated(t).ok_or_else(|| EmitError::Input(format!("引导映像对象的类 {t} 没有生成的存储布局")))?;
            p.layouts.insert(t.to_string(), instance_layout(ctx, ci));
        }
        Ok(p)
    }

    pub fn obj(&self, i: u32) -> &'c IObj {
        &self.d.objs[i as usize]
    }

    /// 由字节码生成（宏展开有 `__inner` / 访问器）的根 crate 类
    pub fn generated(&self, cls: &str) -> Option<&'a ty::ClassInfo> {
        let ci = self.ctx.class(cls)?;
        let em = self.ems.get(cls)?;
        (!em.handwritten && !self.ctx.is_opaque(cls) && self.ctx.crate_of(cls) == self.root).then_some(ci)
    }

    /// 类在映像模块中的路径
    pub fn path(&self, cls: &str) -> String {
        use_path(self.ctx, cls, &self.root)
    }

    /// 类的存储类型 `X__inner` 的路径（实现层模块；未拆层的类在声明位置）
    pub fn inner(&self, cls: &str) -> String {
        match self.homes.get(cls) {
            Some(h) => format!("{h}::{}__inner", self.ctx.declared(cls)),
            None => format!("{}__inner", self.path(cls)),
        }
    }

    /// 泛型类的全 `Object` 实参（`<Object, ..>`；非泛型为空）
    pub fn targs(&self, cls: &str) -> String {
        let n = self.ctx.class(cls).map_or(0, |ci| self.ctx.ty.effective_class_type_params(ci).len());
        if n == 0 {
            String::new()
        } else {
            format!("<{}>", vec!["Object"; n].join(", "))
        }
    }

    /// 类的完整类型（类型位置）
    pub fn full_ty(&self, cls: &str) -> String {
        format!("{}{}", self.path(cls), self.targs(cls))
    }

    /// 类的表达式位置路径（泛型类带 turbofish）
    pub fn expr_path(&self, cls: &str) -> String {
        let t = self.targs(cls);
        if t.is_empty() {
            self.path(cls)
        } else {
            format!("{}::{t}", self.path(cls))
        }
    }

    /// 对象 `i` 在布局中的字段槽
    pub fn slot(&self, i: u32, decl: &str, name: &str) -> Result<&Slot> {
        let ty = &self.obj(i).ty;
        self.layouts
            .get(ty)
            .and_then(|l| l.iter().find(|s| s.decl == decl && s.java == name))
            .ok_or_else(|| EmitError::Input(format!("引导映像对象 #{i}（{ty}）的字段 {decl}.{name} 不在生成的存储布局中")))
    }

    /// 实例字段值（未列出的字段为缺省值）
    pub fn fields(&self, i: u32) -> &'c [(String, String, IVal)] {
        match &self.obj(i).body {
            IBody::Inst(fs) => fs,
            IBody::Arr(_) => &[],
        }
    }
}

/// 发射映像模块（无映像 → 不发射，返回 false）
pub fn write_boot_image(
    ctx: &EmitCtx<'_>, w: &mut Writer, out_dir: &Path, ems: &Emissions, homes: &BTreeMap<String, String>,
) -> Result<bool> {
    let Some(d) = ctx.input.boot_image.as_ref() else { return Ok(false) };
    let plan = Plan::new(ctx, ems, d, homes)?;
    let image = values::image_struct(&plan)?;
    let start = start::start_fn(&plan, &image.links)?;
    let text = format!(
        "//! 构建期引导映像（生成，见计划 2026-10-05-boot-image-evaluator §5.5.2）\n\
         #![allow(non_snake_case, unused_imports, unused_variables, unused_parens, clippy::all)]\n\
         use crate::prelude::*;\n\
         use crate::obj_ref::{{__ImageObj, __image_register}};\n\
         use crate::array::__ImageArr;\n\
         use crate::image_rt as rt;\n\
         use ::std::cell::UnsafeCell;\n\n{}\n{}",
        image.text, start
    );
    w.write(&out_dir.join(&plan.root).join("src").join(format!("{MODULE}.rs")), &text)?;
    Ok(true)
}
