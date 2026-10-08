//! 构建期引导映像的物化（计划 2026-10-05-boot-image-evaluator §5.5.2 D1–D5）。
//!
//! 输入是闭包分析器导出的 [`ImageData`]（活对象集已由联合不动点求出），输出根门面 crate 的
//! `boot_image.rs`：
//!
//! - **映像区**：按规模分段的 `#[repr(C)]` 静态结构 `BOOT_IMAGE_{k}`，每个活对象一个字段
//!   （`__ImageObj<X__inner>` / `__ImageArr<T, S>`），对象头带常驻计数与构建期身份哈希，字段初值是常量求值的
//!   映像内引用（可跨段）。分段使每个静态初值体规模有界：rustc 对单个初值体的分析随体规模超线性增长，
//!   整个映像放进一个静态时编译峰值随映像规模失控；分段后峰值只随映像规模线性增长；
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
    /// 物化对象 → 所在映像段
    pub seg: BTreeMap<u32, usize>,
    /// 映像段数
    pub nseg: usize,
}

/// 每个映像段的规模上限（估算的初值表达式节点数，见 [`Plan::weight`]）
const SEG_WEIGHT: usize = 4096;

impl<'c, 'a> Plan<'c, 'a> {
    fn new(ctx: &'c EmitCtx<'a>, ems: &'c Emissions, d: &'c ImageData, homes: &'c BTreeMap<String, String>) -> Result<Self> {
        let live: BTreeSet<u32> = d.live.iter().copied().collect();
        let mat: BTreeSet<u32> =
            live.iter().copied().filter(|&i| d.objs.get(i as usize).is_some_and(|o| o.mirror.is_none() && !o.placeholder)).collect();
        let root = ctx.crates().root().to_string();
        let mut p = Plan { ctx, ems, d, root, live, mat, layouts: BTreeMap::new(), homes, seg: BTreeMap::new(), nseg: 0 };
        // 映像镜像上写入的缓存字段（`mirror_memos`）按镜像类型的存储布局取设值器
        let memo_tys = d.mirror_memos.iter().map(|m| d.objs[m.mirror as usize].ty.as_str());
        let tys: BTreeSet<&str> =
            p.mat.iter().map(|&i| d.objs[i as usize].ty.as_str()).chain(memo_tys).filter(|t| !t.starts_with('[')).collect();
        for t in tys {
            if t == ty::consts::OBJECT {
                p.layouts.insert(t.to_string(), Vec::new());
                continue;
            }
            let ci = p.generated(t).ok_or_else(|| EmitError::Input(format!("引导映像对象的类 {t} 没有生成的存储布局")))?;
            p.layouts.insert(t.to_string(), instance_layout(ctx, ci));
        }
        // 按对象次序装段，每段估算规模不超过 SEG_WEIGHT（超限的单个对象独占一段）
        let (mut k, mut acc) = (0usize, 0usize);
        for &i in &p.mat {
            let w = p.weight(i);
            if acc > 0 && acc + w > SEG_WEIGHT {
                k += 1;
                acc = 0;
            }
            acc += w;
            p.seg.insert(i, k);
        }
        p.nseg = if p.mat.is_empty() { 0 } else { k + 1 };
        Ok(p)
    }

    /// 对象初值的估算规模：实例按布局槽数，引用数组按元素数，基本类型数组的元素是字面量（`u8` 为单个
    /// 字节串字面量）按较低权重计
    fn weight(&self, i: u32) -> usize {
        let o = self.obj(i);
        1 + match &o.body {
            IBody::Inst(_) => self.layouts.get(&o.ty).map_or(0, |l| l.len()),
            IBody::Arr(es) => match o.ty.as_bytes().get(1) {
                Some(b'L' | b'[') => es.len(),
                Some(b'Z' | b'B') => es.len() / 64,
                _ => es.len() / 8,
            },
        }
    }

    /// 物化对象 `i` 在映像中的位置（`BOOT_IMAGE_{k}.o{i}`）
    pub fn img(&self, i: u32) -> String {
        format!("BOOT_IMAGE_{}.o{i}", self.seg.get(&i).copied().unwrap_or(0))
    }

    /// 映像对象的 `Object` 句柄（常量求值可用）
    pub fn obj_ref(&self, i: u32) -> String {
        format!("Object(__Obj::image(&{}.value as &dyn ObjectVTable))", self.img(i))
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

/// 发射映像模块
pub fn write_boot_image(
    ctx: &EmitCtx<'_>, w: &mut Writer, out_dir: &Path, ems: &Emissions, homes: &BTreeMap<String, String>,
) -> Result<()> {
    let d = &ctx.input.boot_image;
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
    Ok(())
}
