//! 方法签名（← `gen_method_body` 的签名部分）：Rust 函数名、形参列表与各形态签名行。

use std::collections::BTreeMap;

use classfile::extras::LocalVar;
use classfile::Method;
use instr::InstrEnv;
use sim::names::safe_name;
use ty::ident::safe_ident;
use ty::type_map::{mangle_name, parse_descriptor_params};
use ty::RsType;

use crate::text;

/// `public static void main(String[])` 的描述符
pub fn main_desc() -> String {
    format!("([L{};)V", ty::consts::STRING)
}

/// 方法是否为 Java 程序入口 `main(String[])`
pub fn is_main(m: &Method) -> bool {
    m.name == "main" && m.desc == main_desc()
}

/// slot → 代表名：作用域最长的 LVT 条目（等长取先出现者）
pub fn local_names(local_vars: &[LocalVar]) -> BTreeMap<u16, String> {
    let mut best: BTreeMap<u16, (u16, &str)> = BTreeMap::new();
    for v in local_vars {
        match best.get(&v.slot) {
            Some((len, _)) if v.len <= *len => {}
            _ => {
                best.insert(v.slot, (v.len, &v.name));
            }
        }
    }
    best.into_iter().map(|(s, (_, n))| (s, n.to_string())).collect()
}

/// 最终 Rust 方法名：调用方给定名（已去重）> 构造器 `new`（重载加描述符后缀）> 重载加后缀 > 安全化原名
pub fn rust_fn_name(env: &InstrEnv, m: &Method, overloaded: bool, rust_name: Option<&str>) -> String {
    let manifest = env.ctx.ty.manifest;
    if let Some(n) = rust_name {
        safe_ident(n)
    } else if m.name == "<init>" {
        if overloaded {
            mangle_name(manifest, "new", &m.desc)
        } else {
            "new".to_string()
        }
    } else if overloaded {
        mangle_name(manifest, &m.name, &m.desc)
    } else {
        safe_ident(&m.name)
    }
}

/// 形参名：取 LVT 槽位名（缺省 `arg_k`），long / double 占两个槽位
pub fn param_slot_names(m: &Method, start_slot: u16, n: usize, names: &BTreeMap<u16, String>) -> Vec<String> {
    let raw = parse_descriptor_params(&m.desc);
    let mut slot = start_slot;
    let mut out = Vec::with_capacity(n);
    for k in 0..n {
        out.push(safe_name(names.get(&slot).map_or(&format!("arg_{k}"), |n| n)));
        let wide = matches!(raw.get(k).map(String::as_str), Some("J" | "D"));
        slot += if wide { 2 } else { 1 };
    }
    out
}

/// 形参列表 `mut name: Ty`（名字见 [`param_slot_names`]）
pub fn params_text(env: &InstrEnv, pnames: &[String], types: &[RsType]) -> Vec<String> {
    pnames.iter().zip(types).map(|(n, rt)| format!("mut {n}: {}", text::ty(env, rt))).collect()
}

/// 返回类型的 `Result<..>` 形态（void → `Result<()>`）
fn result_of(ret: &str) -> String {
    format!("Result<{ret}>")
}

/// 签名行（不含函数体）；params 为 [`params_with_slot`] 的结果（实例方法不含 `&self`）
pub fn signature_line(m: &Method, fn_name: &str, params: &[String], ret: &str) -> String {
    let is_static = m.access & classfile::acc::STATIC != 0;
    if m.name == "<init>" {
        format!("pub fn {fn_name}({}) -> Result<Self>", params.join(", "))
    } else if is_main(m) {
        "pub fn main() -> Result<()>".to_string()
    } else if is_static {
        format!("pub fn {fn_name}({}) -> {}", params.join(", "), result_of(ret))
    } else {
        let mut all = vec!["&self".to_string()];
        all.extend(params.iter().cloned());
        format!("pub fn {fn_name}({}) -> {}", all.join(", "), result_of(ret))
    }
}

/// 构造器双入口：`new`（建立对象身份并转发）+ `__init_on`（持有构造器体）
pub fn ctor_functions(prefix: &str, fn_name: &str, params: &[String], body: &str) -> String {
    let mut fwd_params = Vec::with_capacity(params.len());
    let mut fwd_args = Vec::with_capacity(params.len());
    for p in params {
        let (pn, pt) = p.split_once(':').unwrap_or((p, ""));
        let pn = pn.strip_prefix("mut ").unwrap_or(pn).trim();
        fwd_params.push(format!("{pn}: {}", pt.trim()));
        fwd_args.push(pn.to_string());
    }
    let init_on = match fn_name.strip_prefix("new") {
        Some(rest) => format!("__init_on{rest}"),
        None => format!("__init_on_{fn_name}"),
    };
    let new_sig = format!("pub fn {fn_name}({}) -> Result<Self>", fwd_params.join(", "));
    let sep = if params.is_empty() { "" } else { ", " };
    let init_sig = format!("pub fn {init_on}(mut this: Self{sep}{}) -> Result<Self>", params.join(", "));
    let fwd_tail = if fwd_args.is_empty() { String::new() } else { format!(", {}", fwd_args.join(", ")) };
    format!(
        "{prefix}{new_sig} {{\n    let mut this = Self::default();\n    this._init_not_null();\n    \
         Self::{init_on}(this{fwd_tail})\n}}\n\n#[doc(hidden)]\n{init_sig} {{\n{body}\n}}"
    )
}
