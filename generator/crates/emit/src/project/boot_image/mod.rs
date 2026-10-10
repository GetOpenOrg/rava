//! 构建期引导映像的物化（计划 2026-10-05-boot-image-evaluator §5.5.2 D1–D5）。
//!
//! 输入是闭包分析器导出的 [`ImageData`]（活对象集已由联合不动点求出），输出根门面 crate 的
//! `boot_image.rs`：
//!
//! - **映像区**：按规模分段的 `#[repr(C)]` 静态结构 `BOOT_IMAGE_{k}`，每个活对象一个字段
//!   （`__ImageObj<X__inner>` / `__ImageArr<T, S>`），对象头带常驻计数与构建期身份哈希，字段初值是常量求值的
//!   映像内引用（可跨段）。分段使每个静态初值体规模有界：rustc 对单个初值体的分析随体规模超线性增长，
//!   整个映像放进一个静态时编译峰值随映像规模失控；分段后峰值只随映像规模线性增长；
//!   类镜像、接口视图（`__IfaceRef::image`）都是常量；以别的元素静态类型引用的数组取映像中的擦除协变视图
//!   （`BOOT_VIEW_{i}`）；
//! - **静态字段初值**：带映像值的静态字段由本模块以常量初值定义其存储（外部符号 [`static_symbol`]，声明层
//!   `#[image_static]` 只声明外部静态）；构建期完成初始化的类以 `#[boot_initialized = true]` 让初始化状态
//!   单元的初值即「已完成」；
//! - **映像表** `IMAGE_TABLES`：类镜像、驻留串（按内容有序）、构建期初始化类、VM 模块表初值，以链接期符号
//!   导出，运行期按符号直接查表，启动时不登记；
//! - **启动序列** `__boot_image_start`：登记映像区 → VM 单元 → 初始线程 → 宿主值改写 → 按构建期
//!   次序重放重定位 / 重算 / 残差调用（档位随之切换），占位对象随步骤回填。
//!
//! 类名只来自映像数据与类文件，生成器中不出现 JDK 类名字面量。

pub(crate) mod layout;
mod start;
mod tables;
mod values;

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use std::cell::RefCell;

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

/// 映像表的外部符号名（根门面以此导出 `IMAGE_TABLES`，运行时 `image_rt` 以同名外部静态声明）
pub const IMAGE_TABLES_SYMBOL: &str = "__rava_image_tables";

/// 物化计划：活对象的分类与各类的存储布局
pub(crate) struct Plan<'c, 'a> {
    pub ctx: &'c EmitCtx<'a>,
    pub ems: &'c Emissions,
    pub d: &'c ImageData,
    pub root: String,
    pub live: BTreeSet<u32>,
    /// 物化为映像区字段的对象（活、非占位；含类镜像）
    pub mat: BTreeSet<u32>,
    /// 类镜像上的缓存字段（U13 `mirror_memos`，镜像与值都活）：并入镜像的字段初值
    pub memos: BTreeMap<u32, Vec<(String, String, IVal)>>,
    /// 以别的元素静态类型被引用的映像数组（各取一个擦除协变视图 `BOOT_VIEW_{i}`）
    pub views: RefCell<BTreeSet<u32>>,
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
        let mat: BTreeSet<u32> = live.iter().copied().filter(|&i| d.objs.get(i as usize).is_some_and(|o| !o.placeholder)).collect();
        let mut memos: BTreeMap<u32, Vec<(String, String, IVal)>> = BTreeMap::new();
        for m in &d.mirror_memos {
            if let IVal::R(t) = m.val {
                if mat.contains(&m.mirror) && live.contains(&t) {
                    memos.entry(m.mirror).or_default().push((m.decl.clone(), m.name.clone(), m.val));
                }
            }
        }
        let root = ctx.crates().root().to_string();
        let mut p = Plan {
            ctx,
            ems,
            d,
            root,
            live,
            mat,
            memos,
            views: RefCell::new(BTreeSet::new()),
            layouts: BTreeMap::new(),
            homes,
            seg: BTreeMap::new(),
            nseg: 0,
        };
        let tys: BTreeSet<&str> = p.mat.iter().map(|&i| d.objs[i as usize].ty.as_str()).filter(|t| !t.starts_with('[')).collect();
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

    /// 实例字段值（未列出的字段为缺省值；类镜像含其缓存字段，同名时缓存值在后、覆盖）
    pub fn fields(&self, i: u32) -> impl Iterator<Item = &(String, String, IVal)> + '_ {
        let own: &[(String, String, IVal)] = match &self.obj(i).body {
            IBody::Inst(fs) => fs,
            IBody::Arr(_) => &[],
        };
        own.iter().chain(self.memos.get(&i).into_iter().flatten())
    }
}

/// 静态字段映像初值的外部符号名（声明层 `#[image_static]` 与本模块的定义同一来源）：binary name 与字段名
/// 逐字符转义（字母数字原样；`_` `/` `$` `.` 各有转义，其余取码点），不同字段不同名
pub fn static_symbol(cls: &str, name: &str) -> String {
    use std::fmt::Write as _;
    let mut s = String::from("__rava_image_static_");
    for c in cls.chars().chain(std::iter::once('.')).chain(name.chars()) {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => s.push(c),
            '_' => s.push_str("_u"),
            '/' => s.push_str("_s"),
            '$' => s.push_str("_d"),
            '.' => s.push_str("_f"),
            c => {
                let _ = write!(s, "_x{:x}_", c as u32);
            }
        }
    }
    s
}

/// 静态字段经启动序列 / 映像常量写入时的访问器名；不由映像给值（非根 crate 的生成类、常量、VM 注入、
/// 手写访问器）→ None
pub fn image_static_accessor(ctx: &EmitCtx<'_>, cls: &str, name: &str) -> Option<String> {
    let ci = ctx.class(cls)?;
    if ctx.is_opaque(cls) || ctx.crate_of(cls) != ctx.crates().root() {
        return None;
    }
    let f = ci.fields().iter().find(|f| f.is_static() && f.name == name)?;
    if f.constant_value.is_some() || ctx.manifest.vm_injected_statics.contains_key(&format!("{cls}.{name}")) {
        return None;
    }
    let acc = ci.static_accessor(name);
    let hw = ctx.input.handwritten.get(cls);
    if hw.is_some_and(|h| h.methods.contains(&acc) || h.methods.contains(&format!("set_{acc}"))) {
        return None;
    }
    Some(acc)
}

/// 映像值即常量初值的静态字段：映像对象引用（非占位）或非零基本值（重算 / 占位由启动序列写入）
pub fn is_image_constant(d: &ImageData, v: IVal) -> bool {
    match v {
        IVal::R(t) => d.objs.get(t as usize).is_some_and(|o| !o.placeholder),
        IVal::N | IVal::T(..) => false,
        v => values::bits(v) != 0,
    }
}

/// 类 `cls` 中以映像常量定义存储的静态字段：Java 字段名 → 外部符号名
pub fn image_statics_of(ctx: &EmitCtx<'_>, cls: &str) -> BTreeMap<String, String> {
    let d = &ctx.input.boot_image;
    d.statics
        .iter()
        .filter(|(c, n, v)| c == cls && is_image_constant(d, *v) && image_static_accessor(ctx, c, n).is_some())
        .map(|(c, n, _)| (n.clone(), static_symbol(c, n)))
        .collect()
}

/// 类是否在构建期完成初始化且由本层生成（类头 `#[boot_initialized = true]`）
pub fn is_boot_initialized(ctx: &EmitCtx<'_>, cls: &str) -> bool {
    let d = &ctx.input.boot_image;
    !ctx.is_opaque(cls) && ctx.crate_of(cls) == ctx.crates().root() && d.build_time.iter().any(|c| c == cls)
}

/// 发射映像模块
pub fn write_boot_image(
    ctx: &EmitCtx<'_>, w: &mut Writer, out_dir: &Path, ems: &Emissions, homes: &BTreeMap<String, String>,
) -> Result<()> {
    let d = &ctx.input.boot_image;
    let plan = Plan::new(ctx, ems, d, homes)?;
    let image = values::image_struct(&plan)?;
    let tabs = tables::tables(&plan)?;
    let start = start::start_fn(&plan, &image.residual, &tabs)?;
    let text = format!(
        "//! 构建期引导映像（生成，见计划 2026-10-05-boot-image-evaluator §5.5.2）\n\
         #![allow(non_snake_case, unused_imports, unused_variables, unused_parens, clippy::all)]\n\
         use crate::prelude::*;\n\
         use crate::obj_ref::{{__ImageObj, __image_register}};\n\
         use crate::array::__ImageArr;\n\
         use crate::image_rt as rt;\n\
         use ::std::cell::UnsafeCell;\n\n{}\n{}\n{}\n{}",
        image.text, values::views_text(&plan), tabs.text, start
    );
    w.write(&out_dir.join(&plan.root).join("src").join(format!("{MODULE}.rs")), &text)?;
    Ok(())
}
