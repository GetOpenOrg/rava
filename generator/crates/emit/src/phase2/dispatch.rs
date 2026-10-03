//! L3 反射分派 / 字段闭包（← `emitter/dispatch_gen.py`）。
//!
//! 为用户树类（及 JDK / 库类的常量反射引用面）在类文件尾部追加
//! `__reflect_dispatch(name, descriptor, recv, args)`（Method.invoke / Constructor.newInstance
//! 的按名协议）与 `__reflect_field(name, recv, value)`（Field.get/set 与 MH 字段句柄），
//! 并产出 main 启动时的登记行（[`DispatchReg`]）。协议见 `java_runtime::reflect_dispatch` 头注。
//!
//! 数据源：类发射文本的 `java_method` / `java_native` / `java_field` 属性行及其后的声明行
//! （与 build.rs 方法表扫描同一属性协议——零二次推导）。

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use regex::Regex;
use ty::type_map::parse_descriptor_params;

use super::sig::split_top_level_trimmed;
use super::uses::class_use_path;
use super::Emissions;
use crate::ctx::EmitCtx;
use crate::emission::ClassEmission;
use crate::project::entry::DispatchReg;

const JAVA_RUNTIME: &str = "java_runtime";

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

/// 序列化协议规定经反射读取的静态成员名（Java Object Serialization Specification §4.6）
const SERIAL_PROTOCOL_FIELDS: [&str; 2] = ["serialPersistentFields", "serialVersionUID"];

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

/// 字段声明行（可见性同 [`fn_re`]：lib crate 的私有 `serialVersionUID` 等为 `pub(crate)`）
fn field_decl_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    re(r"^\s*pub(?:\(crate\))?\s+(?:(static)\s+|(const)\s+)?(\w+)\s*:\s*([^=;,]+?)\s*(?:=[^;]*)?[;,]\s*$", &R)
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

fn runtime_prefix(em: &ClassEmission) -> &'static str {
    if em.crate_name == JAVA_RUNTIME {
        "crate"
    } else {
        JAVA_RUNTIME
    }
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
            if descriptor == "()V" && only.is_none() {
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

/// 类发射文本 → `__reflect_dispatch` 实现（无臂 → None）。`only`：只发射这些方法名的臂。
/// 泛型类 / 接口的闭包挂在 `X<Object..>` 上，类型形参擦为 Object、实参 / 返回按擦除接入：接口载体只是对象引用 +
/// 接口视图，臂经载体做接口调用，按接收者的实现选中（字节码类、lambda、手写实现对象同一路径）；泛型类的接收者经
/// `try_cast::<X<Object..>>` 取擦除视图（共享存储与对象标识），任意实例化同一分派（如序列化按名回调泛型容器的
/// `writeObject`）
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
    if only.is_none_or(|o| o.contains("<init>")) && !em.text.contains("is_abstract       = true") {
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
        "//    协议与上溯语义见 java_runtime::reflect_dispatch 头注）──".into(),
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

/// 单个字段臂（读 + 写 / 常量只读）
fn field_arms(class_bin: &str, em: &ClassEmission, attr: &str, decl: &regex::Captures<'_>, generic: bool, arms: &mut Vec<String>) {
    let Some(fname) = extract(attr, "name") else { return };
    let (is_const, rname, rty) = (decl.get(2).is_some(), &decl[3], decl[4].trim());
    let is_static = decl.get(1).is_some() || is_const || flag(attr, "is_static");
    if generic && !is_static {
        return;
    }
    let disp = format!("{}::reflect_dispatch", runtime_prefix(em));
    let this = format!("recv.try_cast::<Self>(\"{class_bin}\")?");
    let read = if is_static { format!("Object::from(Self::{rname}()?)") } else { format!("Object::from({this}.__get_{rname}())") };
    let unbox = match prim_unbox(rty) {
        Some((f, cast)) => format!("({disp}::{f}(&v).ok_or_else({disp}::bad_arg)?{cast})"),
        None if rty == "Object" => "v".into(),
        None => format!("<{rty} as ::std::convert::From<Object>>::from(v)"),
    };
    arms.push(format!("            (\"{fname}\", None) => Some((|| {{ Ok({read}) }})()),"));
    arms.push(if is_const {
        format!("            (\"{fname}\", Some(_)) => Some(Err({disp}::final_field(\"{fname}\"))),")
    } else if is_static {
        format!("            (\"{fname}\", Some(v)) => Some((|| {{ Self::set_{rname}({unbox})?; Ok(Object::default()) }})()),")
    } else {
        format!("            (\"{fname}\", Some(v)) => Some((|| {{ {this}.__set_{rname}({unbox}); Ok(Object::default()) }})()),")
    });
}

/// 类发射文本 → `__reflect_field` 实现。泛型类闭包挂在 `X<Object..>` 上，实例字段不承载
fn emit_fields_for(ctx: &EmitCtx<'_>, class_bin: &str, em: &ClassEmission, only: Option<&BTreeSet<String>>) -> Option<String> {
    let short = ctx.declared(class_bin);
    let tps = class_tparams(ctx, class_bin);
    let gparams: Option<Vec<String>> = (!tps.is_empty()).then_some(tps);
    let lines: Vec<&str> = em.text.split('\n').collect();
    let mut arms = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if !line.contains("java_field(") {
            continue;
        }
        let (Some(fname), Some(_)) = (extract(line, "name"), extract(line, "descriptor")) else { continue };
        if only.is_some_and(|o| !o.contains(fname)) {
            continue;
        }
        let decl_line = lines[i + 1..lines.len().min(i + 4)].iter().find(|l| !l.trim().starts_with("//"));
        let Some(decl) = decl_line.and_then(|l| field_decl_re().captures(l)) else { continue };
        field_arms(class_bin, em, line, &decl, gparams.is_some(), &mut arms);
    }
    if arms.is_empty() {
        return None;
    }
    let head = match &gparams {
        Some(g) => format!("impl {short}<{}> {{", vec!["Object"; g.len()].join(", ")),
        None => format!("impl {short} {{"),
    };
    let mut out = vec![
        String::new(),
        "// ── L3 反射字段闭包（Field.get/set 与 MH 字段句柄的按名协议）──".into(),
        "#[allow(unused_variables, unreachable_patterns, unused_mut)]".into(),
        head,
        "    pub fn __reflect_field(".into(),
        "        name: &str, recv: Object, value: Option<Object>,".into(),
        "    ) -> Option<Result<Object>> {".into(),
        "        match (name, value) {".into(),
    ];
    out.extend(arms);
    out.extend(["            _ => None,", "        }", "    }", "}"].map(String::from));
    Some(out.join("\n"))
}

/// 泛型类的登记路径实参：每个类型形参取 Object
fn object_turbofish(ctx: &EmitCtx<'_>, bin: &str) -> String {
    let n = class_tparams(ctx, bin).len();
    if n == 0 {
        return String::new();
    }
    format!("::<{}>", vec!["java_runtime::java::lang::Object"; n].join(", "))
}

fn appended(text: &str, tail: &str) -> String {
    format!("{}\n{tail}\n", text.trim_end_matches('\n'))
}

/// 字段闭包：(追加后的类文本, 登记行)；无臂 → None（不登记）。只读 `ems`
fn field_closure(ctx: &EmitCtx<'_>, ems: &Emissions, bin: &str, only: Option<&BTreeSet<String>>) -> Option<(String, String)> {
    let em = ems.get(bin)?;
    // 闭包文本落在该类文件：引用名在其作用域认领；登记行在 main（全路径）
    let text = emit_fields_for(&ctx.scoped(&em.scope), bin, em, only)?;
    let path = class_use_path(ctx, bin, JAVA_RUNTIME, Some(ems), "user");
    let new = appended(&em.text, &text);
    let tf = object_turbofish(ctx, bin);
    let line = format!("    (\"{bin}\", java_runtime::sync_model::__Shared::new(|n, r, v| {path}{tf}::__reflect_field(n, r, v))),");
    Some((new, line))
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

/// 用户树类的分派 / 字段闭包 + 序列化协议 / 按名反射的 JDK 字段闭包 + 常量反射引用面的
/// JDK 方法臂；返回 main 登记行（binary 序）。
///
/// 每类的闭包只由该类自己的文本推出，三轮各自按类并行、按原序回写：字段闭包（用户类）→
/// 字段闭包（其余类，不含上一轮已登记者）→ 方法分派（读取字段闭包追加后的文本）
pub fn synthesize(ctx: &EmitCtx<'_>, ems: &mut Emissions) -> DispatchReg {
    let reflect = &ctx.input.reflect;
    let user_bins: BTreeSet<&str> = ctx.input.user_classes.iter().map(String::as_str).collect();
    let mut fields: BTreeMap<String, String> = BTreeMap::new();
    let users: Vec<(&str, Option<&BTreeSet<String>>)> =
        user_bins.iter().filter(|b| emittable(ctx, ems, b)).map(|b| (*b, None)).collect();
    per_class(ctx, ems, &users, &mut fields, |ems, bin, only| field_closure(ctx, ems, bin, only));
    let mut all: Vec<String> = ems.keys().cloned().collect();
    all.sort();
    let mut onlys: Vec<(&str, Option<BTreeSet<String>>)> = Vec::new();
    for bin in &all {
        if user_bins.contains(bin.as_str()) || fields.contains_key(bin) || !emittable(ctx, ems, bin) {
            continue;
        }
        let ci = ctx.ty.reg.get(bin).expect("已校验存在");
        let only: Option<BTreeSet<String>> = (!reflect.all_members.contains(bin)).then(|| {
            let looked = reflect.fields.get(bin.as_str());
            let named = ci
                .fields()
                .iter()
                .filter(|f| f.is_static() && (looked.is_some_and(|s| s.contains(&f.name)) || reflect.field_names.contains(&f.name)))
                .map(|f| f.name.clone());
            SERIAL_PROTOCOL_FIELDS.iter().map(|s| s.to_string()).chain(named).collect()
        });
        onlys.push((bin, only));
    }
    let jdk: Vec<(&str, Option<&BTreeSet<String>>)> = onlys.iter().map(|(b, o)| (*b, o.as_ref())).collect();
    per_class(ctx, ems, &jdk, &mut fields, |ems, bin, only| {
        if let Some(o) = only {
            let text = &ems[bin].text;
            if !o.iter().any(|n| text.contains(&format!("name = \"{n}\""))) {
                return None;
            }
        }
        field_closure(ctx, ems, bin, only)
    });
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
        let path = class_use_path(ctx, bin, JAVA_RUNTIME, Some(ems), "user");
        let tf = object_turbofish(ctx, bin);
        let line = format!("    (\"{bin}\", java_runtime::sync_model::__Shared::new(|n, d, r, a| {path}{tf}::__reflect_dispatch(n, d, r, a))),");
        Some((appended(&em.text, &text), line))
    });
    DispatchReg { methods: methods.into_values().collect(), fields: fields.into_values().collect() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lib_crate_visibility_lines_match() {
        assert!(field_decl_re().is_match("        pub(crate) const serialVersionUID: i64 = 1i64;"));
        assert!(field_decl_re().is_match("    pub(crate) fRuns: i64,"));
        assert!(field_decl_re().is_match("    pub static X: i32 = 0;"));
        assert_eq!(&fn_re().captures("        pub(crate) fn writeObject(&self, mut s: ObjectOutputStream) -> Result<()> {").unwrap()[1], "writeObject");
        assert_eq!(&fn_re().captures("    pub fn run(&self) -> Result<()> {").unwrap()[1], "run");
    }
}
