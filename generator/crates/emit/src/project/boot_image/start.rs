//! 启动序列 `__boot_image_start`（计划 §5.5.2 D4 / D5）：登记映像区 → VM 单元 → 链接 → 宿主值改写 →
//! 驻留 → 静态字段初值 → 构建期初始化标记 → 按构建期次序重放（档位、重定位、重算、残差调用 / 读取）。
//!
//! 占位对象（残差调用 / 重放 native / 运行期初始化类静态读取的结果）在其步骤处绑定为局部 `ph{i}`，随即
//! 回填映像中全部引用该占位的位置；之后的步骤实参直接使用该绑定。

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use closure::image::{IBody, IExpr, ILoc, IReloc, IStep, IVal};
use ty::type_map::{parse_descriptor_params, parse_descriptor_return};

use super::values::{bits, obj_ref, prim_rust, Link, LinkLoc};
use super::{Plan, START_FN};
use crate::error::{EmitError, Result};

/// 基本类型名 → 描述符字母（类镜像名）
const PRIM_NAMES: [(&str, &str); 9] = [
    ("boolean", "Z"),
    ("byte", "B"),
    ("char", "C"),
    ("short", "S"),
    ("int", "I"),
    ("long", "J"),
    ("float", "F"),
    ("double", "D"),
    ("void", "V"),
];

/// 类镜像名 → 描述符
fn mirror_desc(name: &str) -> String {
    if name.starts_with('[') {
        return name.to_string();
    }
    PRIM_NAMES.iter().find(|(n, _)| *n == name).map_or_else(|| format!("L{name};"), |(_, d)| d.to_string())
}

/// 被调成员键 `cls.name:desc`
fn split_member(key: &str) -> Result<(&str, &str, &str)> {
    let (cls, rest) = key.split_once('.').ok_or_else(|| EmitError::Input(format!("引导映像成员键格式：{key}")))?;
    let (name, desc) = rest.split_once(':').ok_or_else(|| EmitError::Input(format!("引导映像成员键格式：{key}")))?;
    Ok((cls, name, desc))
}

/// 引用位置（字段 / 数组元素）的写入语句
fn store_ref(p: &Plan<'_, '_>, loc: &LinkLoc, v: &str) -> String {
    match loc {
        LinkLoc::Field(o, cls, rust) => {
            format!("<{} as From<Object>>::from({}).__set_{rust}(From::from({v}));", p.full_ty(cls), obj_ref(*o))
        }
        LinkLoc::Elem(o, j) => {
            let t = p.elem_rust(&p.obj(*o).ty[1..]);
            format!("JArray::<{t}>::__image(&BOOT_IMAGE.o{o}.value).set({j}, From::from({v}))?;")
        }
    }
}

struct Gen<'p, 'c, 'a> {
    p: &'p Plan<'c, 'a>,
    out: String,
    /// 已绑定的占位对象
    bound: BTreeSet<u32>,
    /// 已求值的重算表达式
    evaluated: BTreeSet<u32>,
    /// 占位对象 → 映像中引用它的位置（物化对象的字段 / 元素、静态字段）
    ph_refs: BTreeMap<u32, Vec<Target>>,
}

/// 占位对象的回填位置
enum Target {
    Loc(LinkLoc),
    Static(String, String),
}

impl Gen<'_, '_, '_> {
    fn line(&mut self, s: &str) {
        let _ = writeln!(self.out, "    {s}");
    }

    /// 映像对象 `t` 作为 `Object` 的表达式（未绑定的占位 → None）
    fn obj(&self, t: u32) -> Option<String> {
        let o = self.p.obj(t);
        if let Some(m) = &o.mirror {
            return Some(format!("rt::mirror({:?})", mirror_desc(m)));
        }
        if o.placeholder {
            return self.bound.contains(&t).then(|| format!("ph{t}.clone()"));
        }
        self.p.mat.contains(&t).then(|| obj_ref(t))
    }

    /// 值按描述符 `desc` 的 Rust 表达式（引用经 `From::from` 转为目标类型）
    fn val(&mut self, v: IVal, desc: &str) -> Result<String> {
        let c = desc.as_bytes()[0];
        if c == b'L' || c == b'[' {
            let o = match v {
                IVal::R(t) => self.obj(t).ok_or_else(|| EmitError::Input(format!("引导映像启动序列引用了未就绪的对象 #{t}")))?,
                _ => "Object::__NULL".to_string(),
            };
            return Ok(format!("From::from({o})"));
        }
        let r = prim_rust(c);
        Ok(match v {
            IVal::T(e, _) => {
                self.expr(e)?;
                if c == b'Z' {
                    format!("(e{e} != 0)")
                } else {
                    format!("(e{e} as {r})")
                }
            }
            _ => prim_lit(c, bits(v)),
        })
    }

    /// 重算表达式 `e` 及其依赖绑定为 `let e{k}`（整数语义同 JVM：回绕算术、除零抛异常）
    fn expr(&mut self, e: u32) -> Result<()> {
        if self.evaluated.contains(&e) {
            return Ok(());
        }
        let x = self.p.d.exprs.get(e as usize).cloned().ok_or_else(|| EmitError::Input(format!("引导映像表达式 #{e} 缺失")))?;
        let (wide, text) = match x {
            IExpr::Src { native, args } => {
                let (_, _, desc) = split_member(&native)?;
                let ret = parse_descriptor_return(desc).as_bytes()[0];
                let call = self.call(&native, &args)?;
                let text = match ret {
                    b'Z' => format!("i32::from({call})"),
                    b'J' => call,
                    _ => format!("({call} as i32)"),
                };
                (ret == b'J', text)
            }
            IExpr::Un(op, a) => {
                let a = self.operand(a)?;
                match op {
                    0x74 | 0x75 => (op == 0x75, format!("{a}.wrapping_neg()")),
                    0x85 => (true, format!("({a} as i64)")),
                    0x88 => (false, format!("({a} as i32)")),
                    0x91 => (false, format!("({a} as i8 as i32)")),
                    0x92 => (false, format!("({a} as u16 as i32)")),
                    0x93 => (false, format!("({a} as i16 as i32)")),
                    _ => return Err(EmitError::Input(format!("引导映像一元操作 {op:#x}"))),
                }
            }
            IExpr::Bin(op, a, b) => {
                let (a, b) = (self.operand(a)?, self.operand(b)?);
                let wide = matches!(op, 0x61 | 0x65 | 0x69 | 0x6d | 0x71 | 0x79 | 0x7b | 0x7d | 0x7f | 0x81 | 0x83);
                let (ut, st) = if wide { ("u64", "i64") } else { ("u32", "i32") };
                let text = match op {
                    0x60 | 0x61 => format!("{a}.wrapping_add({b})"),
                    0x64 | 0x65 => format!("{a}.wrapping_sub({b})"),
                    0x68 | 0x69 => format!("{a}.wrapping_mul({b})"),
                    0x6c => format!("idiv({a}, {b})?"),
                    0x6d => format!("ldiv({a}, {b})?"),
                    0x70 => format!("irem({a}, {b})?"),
                    0x71 => format!("lrem({a}, {b})?"),
                    0x78 | 0x79 => format!("{a}.wrapping_shl({b} as u32)"),
                    0x7a | 0x7b => format!("{a}.wrapping_shr({b} as u32)"),
                    0x7c | 0x7d => format!("(({a} as {ut}).wrapping_shr({b} as u32) as {st})"),
                    0x7e | 0x7f => format!("({a} & {b})"),
                    0x80 | 0x81 => format!("({a} | {b})"),
                    0x82 | 0x83 => format!("({a} ^ {b})"),
                    0x94 => format!("({a}.cmp(&{b}) as i32)"),
                    _ => return Err(EmitError::Input(format!("引导映像二元操作 {op:#x}"))),
                };
                (wide, text)
            }
            IExpr::Sel { cmp, a, b, t, f } => {
                let ops = ["==", "!=", "<", ">=", ">", "<="];
                let wide = matches!(t, IVal::J(_) | IVal::T(_, b'J')) || matches!(f, IVal::J(_) | IVal::T(_, b'J'));
                let (a, b, t, f) = (self.operand(a)?, self.operand(b)?, self.operand(t)?, self.operand(f)?);
                let op = ops.get(cmp as usize).ok_or_else(|| EmitError::Input(format!("引导映像比较种类 {cmp}")))?;
                (wide, format!("if {a} {op} {b} {{ {t} }} else {{ {f} }}"))
            }
        };
        self.line(&format!("let e{e}: {} = {text};", if wide { "i64" } else { "i32" }));
        self.evaluated.insert(e);
        Ok(())
    }

    /// 重算表达式的操作数（整数）
    fn operand(&mut self, v: IVal) -> Result<String> {
        Ok(match v {
            IVal::I(x) => format!("{x}i32"),
            IVal::J(x) => format!("{x}i64"),
            IVal::T(e, _) => {
                self.expr(e)?;
                format!("e{e}")
            }
            other => return Err(EmitError::Input(format!("引导映像重算操作数不是整数：{other:?}"))),
        })
    }

    /// 调用 `cls.name:desc`（实例方法的首个实参是接收者）；返回带 `?` 的调用表达式
    fn call(&mut self, key: &str, args: &[IVal]) -> Result<String> {
        let (cls, name, desc) = split_member(key)?;
        let p = self.p;
        let ci = p.ctx.class(cls).ok_or_else(|| EmitError::Input(format!("引导映像被调方法的类 {cls} 不在闭包中")))?;
        let m = ci
            .methods()
            .iter()
            .find(|m| m.name == name && m.desc == desc)
            .ok_or_else(|| EmitError::Input(format!("引导映像被调方法 {key} 不存在")))?;
        let mname = p.ctx.ty.receiver_member_name(name, desc, ci);
        let params = parse_descriptor_params(desc);
        let (recv, rest) = if m.is_static() {
            (None, args)
        } else {
            let (r, rest) = args.split_first().ok_or_else(|| EmitError::Input(format!("引导映像实例调用 {key} 缺接收者")))?;
            (Some(*r), rest)
        };
        if rest.len() != params.len() {
            return Err(EmitError::Input(format!("引导映像调用 {key} 实参个数不符")));
        }
        let mut vals = Vec::with_capacity(rest.len());
        for (v, d) in rest.iter().zip(&params) {
            vals.push(self.val(*v, d)?);
        }
        let vals = vals.join(", ");
        Ok(match recv {
            None => format!("{}::{mname}({vals})?", p.expr_path(cls)),
            Some(r) => {
                let IVal::R(t) = r else { return Err(EmitError::Input(format!("引导映像实例调用 {key} 的接收者不是对象"))) };
                let o = self.obj(t).ok_or_else(|| EmitError::Input(format!("引导映像实例调用 {key} 的接收者 #{t} 未就绪")))?;
                format!("<{} as From<Object>>::from({o}).{mname}({vals})?", p.full_ty(cls))
            }
        })
    }

    /// 位置写入：静态字段经免触发 setter、实例字段经 wrapper setter、元素经数组视图
    fn store(&mut self, loc: &ILoc, v: &str) -> Result<()> {
        let p = self.p;
        let s = match loc {
            ILoc::Static(c, n) => {
                let Some(acc) = static_setter(p, c, n) else { return Ok(()) };
                format!("{}::__si_set_{acc}({v})?;", p.expr_path(c))
            }
            ILoc::Field(o, decl, n) => {
                let slot = p.slot(*o, decl, n)?;
                format!("<{} as From<Object>>::from({}).__set_{}({v});", p.full_ty(&p.obj(*o).ty), obj_ref(*o), slot.rust)
            }
            ILoc::Elem(o, j) => {
                let t = p.elem_rust(&p.obj(*o).ty[1..]);
                format!("JArray::<{t}>::__image(&BOOT_IMAGE.o{o}.value).set({j}, {v})?;")
            }
        };
        self.line(&s);
        Ok(())
    }

    /// 位置的描述符（基本类型写入的类型转换）
    fn loc_desc(&self, loc: &ILoc) -> Result<String> {
        let p = self.p;
        Ok(match loc {
            ILoc::Static(c, n) => p
                .ctx
                .class(c)
                .and_then(|ci| ci.fields().iter().find(|f| f.is_static() && f.name == *n))
                .map(|f| f.desc.clone())
                .ok_or_else(|| EmitError::Input(format!("引导映像静态字段 {c}.{n} 不存在")))?,
            ILoc::Field(o, decl, n) => p.slot(*o, decl, n)?.desc.clone(),
            ILoc::Elem(o, _) => p.obj(*o).ty[1..].to_string(),
        })
    }

    /// 占位对象 `ph` 绑定后回填其引用位置
    fn backfill(&mut self, ph: u32) {
        let v = format!("ph{ph}.clone()");
        for t in self.ph_refs.remove(&ph).unwrap_or_default() {
            let s = match t {
                Target::Loc(loc) => store_ref(self.p, &loc, &v),
                Target::Static(c, acc) => format!("{}::__si_set_{acc}(From::from({v}))?;", self.p.expr_path(&c)),
            };
            self.line(&s);
        }
    }

    fn bind(&mut self, ph: Option<u32>, call: String) {
        match ph.filter(|p| self.p.live.contains(p)) {
            Some(ph) => {
                self.line(&format!("let ph{ph}: Object = Into::<Object>::into({call});"));
                self.bound.insert(ph);
                self.backfill(ph);
            }
            None => self.line(&format!("let _ = {call};")),
        }
    }
}

/// 基本类型字面量（位模式 → 描述符类型）
fn prim_lit(c: u8, b: u64) -> String {
    match c {
        b'Z' => (b != 0).to_string(),
        b'B' => format!("{}i8", b as i8),
        b'C' => format!("{}u16", b as u16),
        b'S' => format!("{}i16", b as i16),
        b'I' => format!("{}i32", b as i32),
        b'J' => format!("{}i64", b as i64),
        b'F' => format!("f32::from_bits({:#x})", b as u32),
        _ => format!("f64::from_bits({b:#x})"),
    }
}

/// 静态字段经启动序列写入时的访问器名；不由启动序列写入（非生成类、常量、VM 注入、手写访问器）→ None
fn static_setter(p: &Plan<'_, '_>, cls: &str, name: &str) -> Option<String> {
    let ci = p.generated(cls)?;
    let f = ci.fields().iter().find(|f| f.is_static() && f.name == name)?;
    if f.constant_value.is_some() || p.ctx.manifest.vm_injected_statics.contains_key(&format!("{cls}.{name}")) {
        return None;
    }
    let acc = ci.static_accessor(name);
    let hw = p.ctx.input.handwritten.get(cls);
    if hw.is_some_and(|h| h.methods.contains(&acc) || h.methods.contains(&format!("set_{acc}"))) {
        return None;
    }
    Some(acc)
}

/// 启动函数文本
pub(crate) fn start_fn(p: &Plan<'_, '_>, links: &[Link]) -> Result<String> {
    let mut g = Gen { p, out: String::new(), bound: BTreeSet::new(), evaluated: BTreeSet::new(), ph_refs: BTreeMap::new() };
    let d = p.d;
    g.line("__image_register(&BOOT_IMAGE as *const __BootImage as *const u8, ::std::mem::size_of::<__BootImage>());");
    for (name, v) in &d.cells {
        g.line(&format!("rt::vm_cell({name:?}, {v}i64);"));
    }
    // VM 初始线程（§5.5.1 S3）：先于任何 Java 代码绑定为 OS 主线程的当前线程
    if let Some(t) = d.current_thread {
        if !p.mat.contains(&t) {
            return Err(EmitError::Input(format!("引导映像的初始线程 #{t} 未物化")));
        }
        g.line(&format!("rt::bind_initial_thread({});", obj_ref(t)));
    }
    // 链接：常量不可表达的映像内引用与类镜像；占位对象留到其步骤
    for l in links {
        if p.obj(l.target).placeholder {
            g.ph_refs.entry(l.target).or_default().push(Target::Loc(l.loc.clone()));
            continue;
        }
        let v = g.obj(l.target).expect("链接目标已物化");
        let s = store_ref(p, &l.loc, &v);
        g.line(&s);
    }
    // 占位对象在物化对象中的引用位置（常量形态取 null 的槽；`links` 只登记可链接目标）
    for &i in &p.mat {
        match &p.obj(i).body {
            IBody::Inst(fs) => {
                for (decl, name, v) in fs {
                    if let IVal::R(t) = v {
                        if p.obj(*t).placeholder && p.live.contains(t) {
                            let s = p.slot(i, decl, name)?;
                            g.ph_refs.entry(*t).or_default().push(Target::Loc(LinkLoc::Field(i, p.obj(i).ty.clone(), s.rust.clone())));
                        }
                    }
                }
            }
            IBody::Arr(es) => {
                for (j, v) in es.iter().enumerate() {
                    if let IVal::R(t) = v {
                        if p.obj(*t).placeholder && p.live.contains(t) {
                            g.ph_refs.entry(*t).or_default().push(Target::Loc(LinkLoc::Elem(i, j as u32)));
                        }
                    }
                }
            }
        }
    }
    host_rewrite(&mut g)?;
    for &s in &d.strings {
        if p.mat.contains(&s) {
            g.line(&format!("rt::intern({});", obj_ref(s)));
        }
    }
    // 静态字段初值（零值即存储缺省值；重算 / 重定位槽由步骤写入）
    for (cls, name, v) in &d.statics {
        let Some(acc) = static_setter(p, cls, name) else { continue };
        let desc = g.loc_desc(&ILoc::Static(cls.clone(), name.clone()))?;
        let c = desc.as_bytes()[0];
        let val = match *v {
            IVal::N | IVal::T(..) => continue,
            IVal::R(t) if p.obj(t).placeholder => {
                if p.live.contains(&t) {
                    g.ph_refs.entry(t).or_default().push(Target::Static(cls.clone(), acc));
                }
                continue;
            }
            IVal::R(t) => match g.obj(t) {
                Some(o) => format!("From::from({o})"),
                None => continue,
            },
            v if bits(v) == 0 => continue,
            v => prim_lit(c, bits(v)),
        };
        g.line(&format!("{}::__si_set_{acc}({val})?;", p.expr_path(cls)));
    }
    for cls in &d.build_time {
        if p.generated(cls).is_some() {
            g.line(&format!("{}::__boot_initialized();", p.expr_path(cls)));
        }
    }
    for st in &d.steps {
        step(&mut g, st)?;
    }
    g.line("rt::set_level(None);");
    g.line("Ok(())");
    Ok(format!(
        "/// 构建期引导映像的启动序列（`main` 在创建 VM 之后调用）\npub fn {START_FN}() {{\n    rt::run(__start)\n}}\n\nfn __start() -> Result<()> {{\n{}}}\n",
        g.out
    ))
}

/// 宿主值改写：每个来源 native 调用一次，以运行期结果的全部字段改写引用该内容数组的映像字符串
fn host_rewrite(g: &mut Gen<'_, '_, '_>) -> Result<()> {
    let p = g.p;
    // native → [(结果下标, 引用该内容数组的物化对象)]
    let mut by_native: BTreeMap<&str, Vec<(Option<u32>, u32)>> = BTreeMap::new();
    for &a in &p.mat {
        let Some((native, idx)) = &p.obj(a).host else { continue };
        for &s in &p.mat {
            if p.fields(s).iter().any(|(_, _, v)| *v == IVal::R(a)) {
                by_native.entry(native.as_str()).or_default().push((*idx, s));
            }
        }
    }
    for (k, (native, uses)) in by_native.into_iter().enumerate() {
        let (cls, name, desc) = split_member(native)?;
        let ci = p.ctx.class(cls).ok_or_else(|| EmitError::Input(format!("引导映像宿主来源 {native} 的类不在闭包中")))?;
        if !parse_descriptor_params(desc).is_empty() || !ci.methods().iter().any(|m| m.name == name && m.desc == desc && m.is_static()) {
            return Err(EmitError::Input(format!("引导映像宿主来源 {native} 须为无参 static 方法")));
        }
        g.line(&format!("let host{k} = {}::{}()?;", p.expr_path(cls), p.ctx.ty.receiver_member_name(name, desc, ci)));
        for (idx, s) in uses {
            let src = match idx {
                Some(i) => format!("Into::<Object>::into(host{k}.get({i})?)"),
                None => format!("Into::<Object>::into(host{k}.clone())"),
            };
            let ty = &p.obj(s).ty;
            let full = p.full_ty(ty);
            g.line(&format!("let h: Object = {src};"));
            g.line("if !h.is_jvm_null() {");
            g.line(&format!("    let (src, dst) = (<{full} as From<Object>>::from(h), <{full} as From<Object>>::from({}));", obj_ref(s)));
            for slot in &p.layouts[ty] {
                g.line(&format!("    dst.__set_{0}(src.__get_{0}());", slot.rust));
            }
            g.line("}");
        }
    }
    Ok(())
}

fn step(g: &mut Gen<'_, '_, '_>, st: &IStep) -> Result<()> {
    let p = g.p;
    match st {
        IStep::Level(l) => g.line(&format!("rt::set_level(Some({l}));")),
        IStep::Reloc { loc, reloc } => {
            let v = match reloc {
                IReloc::FieldOffset(c, n) => format!("rt::field_offset({c:?}, {n:?})"),
                IReloc::Cell(n) => format!("rt::vm_cell_addr({n:?})"),
            };
            g.store(loc, &v)?;
        }
        IStep::Recompute { loc, expr } => {
            g.expr(*expr)?;
            let desc = g.loc_desc(loc)?;
            let c = desc.as_bytes()[0];
            let v = if c == b'Z' { format!("(e{expr} != 0)") } else { format!("(e{expr} as {})", prim_rust(c)) };
            g.store(loc, &v)?;
        }
        IStep::RuntimeInit { class } => g.line(&format!("// {class}：运行期初始化（首次主动使用时执行 <clinit>）")),
        IStep::Call { callee, args, ph, .. } | IStep::Native { callee, args, ph } => {
            let call = g.call(callee, args)?;
            g.bind(*ph, call);
        }
        IStep::Read { decl, name, ph } => {
            let ci = p.ctx.class(decl).ok_or_else(|| EmitError::Input(format!("引导映像静态读取的类 {decl} 不在闭包中")))?;
            g.bind(Some(*ph), format!("{}::{}()?", p.expr_path(decl), ci.static_accessor(name)));
        }
        IStep::Region { phase, start, end, .. } => {
            g.line(&format!("// 残差区段 {phase} [{start}, {end:?})：未物化（计划 §5.5.5 待办）"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literals_and_names() {
        assert_eq!(mirror_desc("int"), "I");
        assert_eq!(mirror_desc("[I"), "[I");
        assert_eq!(mirror_desc("a/B"), "La/B;");
        assert_eq!(prim_lit(b'Z', 1), "true");
        assert_eq!(prim_lit(b'I', u64::MAX), "-1i32");
        assert_eq!(prim_lit(b'C', 0x41), "65u16");
        assert_eq!(split_member("a/B.m:(I)V").unwrap(), ("a/B", "m", "(I)V"));
    }
}
