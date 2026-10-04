//! L3 反射分派 / 字段闭包（← `emitter/dispatch_gen.py`）。
//!
//! 为用户树类（及 JDK / 库类的常量反射引用面）在类文件尾部追加
//! `__reflect_dispatch(name, descriptor, recv, args)`（Method.invoke / Constructor.newInstance
//! 的按名协议），并产出 main 启动时的登记行（[`DispatchReg`]）：方法分派闭包，及声明类的静态
//! 字段表 `X::__STATICS`（宏展开；Field.get/set 与 MH 字段句柄的按名协议由运行时 `field_reflect`
//! 按描述符承载，S7-3b）。协议见 `java_runtime::reflect_dispatch` 头注。
//!
//! 数据源：类发射文本的 `java_method` / `java_native` 属性行及其后的声明行
//! （与 build.rs 方法表扫描同一属性协议——零二次推导）。

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use regex::Regex;
use ty::type_map::parse_descriptor_params;

use super::sig::split_top_level_trimmed;
use super::uses::{class_use_path, use_path};
use super::Emissions;
use crate::ctx::{EmitCtx, USER_CRATE};
use crate::emission::ClassEmission;
use crate::project::entry::DispatchReg;
use input::build::ALLOC_MEMBER;

/// 基本类型 → (拆箱函数, 追加转换)
const PRIM_UNBOX: [(&str, &str, &str); 8] = [
    ("i64", "unbox_i64", ""),
    ("i32", "unbox_i32", ""),
    ("bool", "unbox_bool", ""),
    ("f64", "unbox_f64", ""),
    ("f32", "unbox_f32", ""),
    ("i16", "unbox_i32", " as i16"),
    ("i8", "unbox_i32", " as i8"),
    ("u16", "unbox_char", ""),
];

fn prim_unbox(ty: &str) -> Option<(&'static str, &'static str)> {
    PRIM_UNBOX.iter().find(|(t, _, _)| *t == ty).map(|(_, f, c)| (*f, *c))
}

fn re(pat: &'static str, cell: &'static OnceLock<Regex>) -> &'static Regex {
    cell.get_or_init(|| Regex::new(pat).expect("静态正则"))
}

fn attr_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    re(r"#\[java_(?:method|native)\(", &R)
}

/// 方法签名行；lib crate 按 Java 可见性把 package / private 成员发为 `pub(crate)`，同样承载反射臂
fn fn_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    re(r"^\s*pub(?:\(crate\))?\s+fn\s+(\w+)\s*\(([^)]*)\)\s*(?:->\s*(.+?))?\s*[;{]", &R)
}

/// 属性行里 `key = "value"`（`\bkey`）
fn extract<'t>(window: &'t str, key: &str) -> Option<&'t str> {
    crate::scan::attr_str(window, key)
}

fn flag(window: &str, key: &str) -> bool {
    crate::scan::attr_true(window, key)
}

/// 类 struct 的类型形参（与类文件声明同源：`effective_class_type_params`）；非泛型 → 空
fn class_tparams(ctx: &EmitCtx<'_>, bin: &str) -> Vec<String> {
    ctx.ty.reg.get(bin).map(|ci| ctx.ty.effective_class_type_params(ci).to_vec()).unwrap_or_default()
}

/// 类文件内引用运行时基础设施的路径首段（JDK crate 内 `crate`，其余 crate 根门面名）
fn runtime_prefix(em: &ClassEmission) -> &str {
    &em.crate_prefix
}

/// 描述符第 idx 个参数的二进制名（`L..;` / 数组原样；基本类型 None）
fn param_bin(descriptor: &str, idx: usize) -> Option<String> {
    let p = parse_descriptor_params(descriptor).into_iter().nth(idx)?;
    if let Some(b) = p.strip_prefix('L').and_then(|r| r.strip_suffix(';')) {
        return Some(b.to_string());
    }
    p.starts_with('[').then_some(p)
}

/// 实参 marshalling 表达式（None = 形态不承载）
fn arg_expr(disp: &str, descriptor: &str, idx: usize, ty: &str) -> Option<String> {
    if ty == "Object" {
        return Some(format!("args.get({idx})?"));
    }
    if ty == "String" {
        return Some(format!("String::valueOf_obj(args.get({idx})?)?"));
    }
    if let Some((f, cast)) = prim_unbox(ty) {
        return Some(format!("({disp}::{f}(&args.get({idx})?).ok_or_else({disp}::bad_arg)?{cast})"));
    }
    if ty.starts_with("JArray") || ty.contains('<') {
        return Some(format!("<{ty} as ::std::convert::From<Object>>::from(args.get({idx})?)"));
    }
    let pbin = param_bin(descriptor, idx).filter(|b| !b.starts_with('['))?;
    Some(format!("args.get({idx})?.try_cast::<{ty}>(\"{pbin}\")?"))
}

/// 返回值装箱（None = 形态不承载）
fn ret_box(inner: Option<&str>) -> Option<&'static str> {
    let inner = inner?;
    match inner {
        "()" => Some("Ok(Object::default())"),
        "Object" => Some("Ok(__v)"),
        _ if prim_unbox(inner).is_some() || inner == "String" => Some("Ok(Object::from(__v))"),
        _ if inner.starts_with(|c: char| c.is_uppercase()) && !inner.contains('&') => Some("Ok(Object::from(__v))"),
        _ if inner.starts_with("JArray") => Some("Ok(Object::from(__v))"),
        _ => None,
    }
}

/// 发射签名行的切分（Rust 名, 形参表文本, 返回类型）
struct FnSig {
    rust_name: String,
    params: String,
    ret: Option<String>,
}

/// 属性行之后 4 行内最近的 `pub fn` 签名（手写体方法的签名以 `// [meta] ` 注释行承载，
/// 删去首个该标记后匹配）；中途遇到下一属性行即放弃
fn following_fn(lines: &[&str], i: usize) -> Option<FnSig> {
    let mut j = i + 1;
    while j < lines.len() && j <= i + 4 {
        let line = lines[j].replacen("// [meta] ", "", 1);
        if let Some(c) = fn_re().captures(&line) {
            return Some(FnSig { rust_name: c[1].to_string(), params: c[2].to_string(), ret: c.get(3).map(|m| m.as_str().to_string()) });
        }
        if attr_re().is_match(lines[j]) {
            return None;
        }
        j += 1;
    }
    None
}

/// 类型文本里的类型形参（整词）擦除为 Object：泛型接口的分派闭包挂在 `I<Object..>` 上
fn erase_tparams(ty: &str, tps: &[String]) -> String {
    let mut out = ty.to_string();
    for tp in tps {
        let r = Regex::new(&format!(r"\b{}\b", regex::escape(tp))).expect("类型形参正则");
        out = r.replace_all(&out, "Object").into_owned();
    }
    out
}

/// 单个方法臂（unsupported → panic 存根臂；`<init>` 可附带 `<init_on>` 臂）。tps：擦除为 Object 的类型形参
fn method_arms(class_bin: &str, em: &ClassEmission, attr: &str, sig: &FnSig, tps: &[String], only: Option<&BTreeSet<String>>, arms: &mut Vec<String>) {
    let (Some(mname), Some(descriptor)) = (extract(attr, "name"), extract(attr, "descriptor")) else { return };
    if only.is_some_and(|o| !o.contains(mname)) {
        return;
    }
    let rt = runtime_prefix(em);
    let disp = format!("{rt}::reflect_dispatch");
    let is_static = flag(attr, "is_static");
    let rust_name = sig.rust_name.as_str();
    let ret = erase_tparams(sig.ret.as_deref().unwrap_or("()").trim(), tps);
    let ret = ret.as_str();
    let pdefs: Vec<(String, String)> = split_top_level_trimmed(&sig.params)
        .into_iter()
        .filter(|p| !matches!(p.as_str(), "&self" | "self" | "mut self" | "&mut self"))
        .map(|p| {
            let (n, t) = p.split_once(':').unwrap_or((p.as_str(), ""));
            let n = n.trim();
            (n.strip_prefix("mut ").unwrap_or(n).trim().to_string(), erase_tparams(t.trim(), tps))
        })
        .collect();
    let exprs: Vec<Option<String>> = pdefs.iter().enumerate().map(|(i, (_, t))| arg_expr(&disp, descriptor, i, t)).collect();
    let inner = ret.strip_prefix("Result<").and_then(|r| r.strip_suffix('>'));
    let boxed = ret_box(inner);
    let (Some(boxed), true) = (boxed, exprs.iter().all(Option::is_some)) else {
        let body = crate::precheck::stub_call("stub", &format!("L3 分派未支持的签名形态 {class_bin}.{mname}:{descriptor}"));
        arms.push(format!("        (\"{mname}\", \"{descriptor}\") => Some({body}),"));
        return;
    };
    let call = pdefs.iter().map(|(n, _)| format!("{{{n}}}")).collect::<Vec<_>>().join(", ");
    let mut inner_body = if mname == "<init>" {
        let call_expr = format!("Self::{rust_name}({call})?");
        let tail = "Ok(Object::from(__r))";
        let mut b = format!("let __r = {call_expr}; {tail}");
        let init_on_ctor = rust_name.strip_prefix("new").map(|r| format!("__init_on{r}"));
        if let Some(ctor) = &init_on_ctor {
            if crate::scan::has_fn(&em.text, ctor) {
                let on_args = format!("recv.try_cast::<Self>(\"{class_bin}\")?{}", if call.is_empty() { String::new() } else { format!(", {call}") });
                b = format!("let __r = if {rt}::_is_jnull(&recv) {{ {call_expr} }} else {{ Self::{ctor}({on_args})? }}; {tail}");
            }
            // 序列化构造器在已分配实例上运行本类无参构造体（反射构造成员即有 `<init_on>` 臂）
            if descriptor == "()V" {
                arms.push(format!(
                    "        (\"<init_on>\", \"()V\") => Some((|| {{ Self::{ctor}(recv.try_cast::<Self>(\"{class_bin}\")?)?; Ok(Object::default()) }})()),"
                ));
            }
        }
        b
    } else {
        let callee = if is_static { format!("Self::{rust_name}") } else { format!("recv.try_cast::<Self>(\"{class_bin}\")?.{rust_name}") };
        if inner == Some("()") {
            format!("{callee}({call})?; Ok(Object::default())")
        } else {
            format!("let __v = {callee}({call})?; {boxed}")
        }
    };
    for ((pn, _), e) in pdefs.iter().zip(&exprs) {
        inner_body = inner_body.replace(&format!("{{{pn}}}"), e.as_deref().unwrap_or_default());
    }
    arms.push(format!("        (\"{mname}\", \"{descriptor}\") => Some((|| {{ {inner_body} }})()),"));
}

/// 桥方法臂（类头 `#[bridge_method(.., bridge_to = ..)]`）：桥不发射为 Rust 方法，按桥描述符键入
/// 被桥接真实方法的臂（实参按真实形参类型接入，与桥体的 checkcast 同义）。同键已有臂（协变桥
/// wrapper 已发射为方法）不重复；真实方法在祖先时无 `bridge_to`，分派沿接收者类链上溯
fn bridge_arms(class_bin: &str, em: &ClassEmission, lines: &[&str], tps: &[String], only: Option<&BTreeSet<String>>, arms: &mut Vec<String>) {
    let key = |n: &str, d: &str| format!("        (\"{n}\", \"{d}\") =>");
    for line in lines.iter().filter(|l| l.trim_start().starts_with("#[bridge_method(")) {
        let (Some(name), Some(desc), Some(to)) = (extract(line, "name"), extract(line, "descriptor"), extract(line, "bridge_to")) else {
            continue;
        };
        if arms.iter().any(|a| a.starts_with(&key(name, desc))) {
            continue;
        }
        let target = lines.iter().enumerate().find(|(_, l)| {
            attr_re().is_match(l) && extract(l, "name") == Some(name) && extract(l, "descriptor") == Some(to)
        });
        let Some((i, attr)) = target else { continue };
        let Some(sig) = following_fn(lines, i) else { continue };
        let mut tmp = Vec::new();
        method_arms(class_bin, em, attr, &sig, tps, only, &mut tmp);
        if let Some(arm) = tmp.into_iter().find(|a| a.starts_with(&key(name, to))) {
            arms.push(arm.replacen(&key(name, to), &key(name, desc), 1));
        }
    }
}

/// 类发射文本 → `__reflect_dispatch` 实现（无臂 → None）。`only`：只发射这些方法名的臂。
/// 泛型类 / 泛型接口的闭包挂在 `X<Object..>` 上（与字段闭包、类初始化登记同一实例化）：
/// 载体是对象引用 + 类型视图，任意实例化同形，实参 / 返回按擦除接入；构造臂产出 `X<Object..>`，
/// 调用方按 checkcast 视图取用。接口臂经载体做接口调用，按接收者的实现选中
fn emit_for(ctx: &EmitCtx<'_>, class_bin: &str, em: &ClassEmission, only: Option<&BTreeSet<String>>) -> Option<String> {
    let tps = class_tparams(ctx, class_bin);
    let short = ctx.declared(class_bin);
    let lines: Vec<&str> = em.text.split('\n').collect();
    let mut arms = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if !attr_re().is_match(line) {
            continue;
        }
        if let Some(sig) = following_fn(&lines, i) {
            method_arms(class_bin, em, line, &sig, &tps, only, &mut arms);
        }
    }
    bridge_arms(class_bin, em, &lines, &tps, only, &mut arms);
    if only.is_none_or(|o| o.contains("<init>") || o.contains(ALLOC_MEMBER)) && !em.text.contains("is_abstract       = true") {
        arms.push(
            "        (\"<alloc>\", \"()V\") => Some((|| { let mut __o = Self::default(); __o._init_not_null(); Ok(Object::from(__o)) })()),"
                .into(),
        );
    }
    if arms.is_empty() {
        return None;
    }
    let mut out = vec![
        String::new(),
        "// ── L3 反射分派闭包（Method.invoke / Constructor.newInstance 的按名协议；".into(),
        "//    协议与上溯语义见运行时 reflect_dispatch 头注）──".into(),
        "#[allow(unused_variables, unreachable_patterns)]".into(),
        if tps.is_empty() { format!("impl {short} {{") } else { format!("impl {short}<{}> {{", vec!["Object"; tps.len()].join(", ")) },
        "    pub fn __reflect_dispatch(".into(),
        "        name: &str, descriptor: &str,".into(),
        "        recv: Object, args: &JArray<Object>,".into(),
        "    ) -> Option<Result<Object>> {".into(),
        "        match (name, descriptor) {".into(),
    ];
    out.extend(arms);
    out.extend(["            _ => None,", "        }", "    }", "}"].map(String::from));
    Some(out.join("\n"))
}

/// 泛型类的登记路径实参（反射分派 / 类初始化钩子 / 引导初始化的按名登记共用）：无发射（手写类）→ 空
pub fn registration_turbofish(ctx: &EmitCtx<'_>, ems: &Emissions, bin: &str) -> String {
    ems.get(bin).map(|_| object_turbofish(ctx, bin)).unwrap_or_default()
}

/// 泛型类的登记路径实参：每个类型形参取 Object
fn object_turbofish(ctx: &EmitCtx<'_>, bin: &str) -> String {
    let n = class_tparams(ctx, bin).len();
    if n == 0 {
        return String::new();
    }
    let object = use_path(ctx, ty::consts::OBJECT, USER_CRATE);
    format!("::<{}>", vec![object.as_str(); n].join(", "))
}

fn appended(text: &str, tail: &str) -> String {
    format!("{}\n{tail}\n", text.trim_end_matches('\n'))
}

/// 静态字段表登记行：`X::__STATICS`（宏展开的关联常量，见运行时 `field_reflect`）
fn statics_line(ctx: &EmitCtx<'_>, bin: &str) -> String {
    let path = class_use_path(ctx, bin, USER_CRATE);
    let tf = object_turbofish(ctx, bin);
    format!("    (\"{bin}\", {path}{tf}::__STATICS),")
}

/// 静态字段是否可在运行期按名访问（反射 `Field`、方法句柄 / VarHandle、Unsafe 静态偏移；档案口径）：用户树类与
/// 全成员反射类的全部静态字段；其余类取闭包分析的可达事实——按名查字段点到的字段（含目标类推不出时的字面量名，
/// 如序列化协议经 `getDeclaredField` 读取的成员）、字段枚举口径所指类的静态字段与静态字段句柄常量。为真的字段
/// 在发射文本的字段属性上带 `reflect = true`，宏只为带标记的字段展开 `__STATICS` 项；类有任一此类字段即登记。
pub fn static_reflected(ctx: &EmitCtx<'_>, bin: &str, name: &str) -> bool {
    let reflect = &ctx.input.reflect;
    ctx.input.user_classes.iter().any(|u| u == bin)
        || reflect.all_members.contains(bin)
        || reflect.fields.get(bin).is_some_and(|s| s.contains(name))
        || reflect.static_fields.get(bin).is_some_and(|s| s.contains(name))
        || reflect.field_names.contains(name)
}

fn emittable(ctx: &EmitCtx<'_>, ems: &Emissions, bin: &str) -> bool {
    ems.get(bin).is_some_and(|e| !e.handwritten) && ctx.ty.reg.contains(bin)
}

/// 按类并行求值 `f`（各类只读 `ems`、只改写自己的文本），再按 `bins` 序回写文本、收集登记行
fn per_class<'b>(
    ctx: &EmitCtx<'_>,
    ems: &mut Emissions,
    bins: &[(&'b str, Option<&'b BTreeSet<String>>)],
    ledger: &mut BTreeMap<String, String>,
    f: impl Fn(&Emissions, &str, Option<&BTreeSet<String>>) -> Option<(String, String)> + Sync,
) {
    let jobs = crate::par::resolve_jobs(ctx.opts.jobs);
    let shared: &Emissions = ems;
    let out = crate::par::par_map(jobs, bins, |(bin, only)| f(shared, bin, *only));
    for ((bin, _), r) in bins.iter().zip(out) {
        let Some((text, line)) = r else { continue };
        ems.get_mut(*bin).expect("已校验存在").text = text;
        ledger.insert(bin.to_string(), line);
    }
}

/// 静态字段表登记（`static_reflected` 为真的静态字段所在类）+ 用户树类与常量
/// 反射引用面 JDK 类的方法分派臂；返回 main 登记行（binary 序）。
///
/// 方法分派闭包只由该类自己的文本推出，按类并行、按原序回写
pub fn synthesize(ctx: &EmitCtx<'_>, ems: &mut Emissions) -> DispatchReg {
    let reflect = &ctx.input.reflect;
    let user_bins: BTreeSet<&str> = ctx.input.user_classes.iter().map(String::as_str).collect();
    let mut fields: Vec<String> = Vec::new();
    let mut all: Vec<&String> = ems.keys().collect();
    all.sort();
    for bin in all {
        if !emittable(ctx, ems, bin) {
            continue;
        }
        let ci = ctx.ty.reg.get(bin).expect("已校验存在");
        if ci.fields().iter().any(|f| f.is_static() && static_reflected(ctx, bin, &f.name)) {
            fields.push(statics_line(ctx, bin));
        }
    }
    let mut targets: BTreeMap<&str, Option<&BTreeSet<String>>> = user_bins.iter().map(|b| (*b, None)).collect();
    for (b, names) in &reflect.consts {
        targets.entry(b.as_str()).or_insert(Some(names));
    }
    let targets: Vec<(&str, Option<&BTreeSet<String>>)> = targets
        .into_iter()
        .filter(|(bin, _)| ems.get(*bin).is_some_and(|e| !e.handwritten) && ctx.ty.reg.contains(bin))
        .collect();
    let mut methods: BTreeMap<String, String> = BTreeMap::new();
    per_class(ctx, ems, &targets, &mut methods, |ems, bin, only| {
        let em = &ems[bin];
        let text = emit_for(&ctx.scoped(&em.scope), bin, em, only)?;
        let path = class_use_path(ctx, bin, USER_CRATE);
        let tf = object_turbofish(ctx, bin);
        let rt = ctx.crates().root();
        let line = format!("    (\"{bin}\", {rt}::sync_model::__Shared::new(|n, d, r, a| {path}{tf}::__reflect_dispatch(n, d, r, a))),");
        Some((appended(&em.text, &text), line))
    });
    DispatchReg { methods: methods.into_values().collect(), fields }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lib_crate_visibility_lines_match() {
        assert_eq!(&fn_re().captures("        pub(crate) fn writeObject(&self, mut s: ObjectOutputStream) -> Result<()> {").unwrap()[1], "writeObject");
        assert_eq!(&fn_re().captures("    pub fn run(&self) -> Result<()> {").unwrap()[1], "run");
    }
}
