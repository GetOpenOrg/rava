//! panic 存根与无体声明的签名（← `emitter/method_gen._gen_native_stub`）。
//!
//! 签名类型取 [`ty::TyCtx::emitted_method_sig_types`]（与方法体生成同源）；形参名取局部变量表
//! （按「第 i 个参数 → 槽 i(+1)」对位，与 Python 同口径，不计 long/double 双槽宽）。

use std::collections::BTreeMap;

use classfile::Method;
use ty::ident::safe_ident;
use ty::ClassInfo;

use crate::ctx::EmitCtx;
use crate::lang;
use crate::precheck::stub_call;

/// 存根：`sig` 为签名行（不含 ` {`，供无体声明 / `[meta]` 行复用），`text` 为完整函数文本
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stub {
    pub sig: String,
    pub text: String,
}

/// String 结构类（`value:[B` + `coder:B`）的 hashCode 存根体：Latin1 / UTF16 双路径散列。
/// 不以外层 `{}` 包裹（宏会把整个内层 block 作为单条语句剥离）；`let this = self;`
/// 兼容 vtable impl 与 base 自由函数两种上下文。
const STRING_HASH_BODY: &str = "let this = self;
    let val = this.__get_value();
    let len = val.len()?;
    let mut h: i32 = 0i32;
    if this.__get_coder() == 0i8 {
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            h = h.wrapping_mul(31i32).wrapping_add(val.get(i)? as u8 as i32);
            i += 1i32;
        }
    } else {
        let pairs: i32 = len / 2i32;
        let mut i: i32 = 0i32;
        loop {
            if i >= pairs { break; }
            let b1 = val.get(i * 2i32)? as u8;
            let b2 = val.get(i * 2i32 + 1i32)? as u8;
            h = h.wrapping_mul(31i32).wrapping_add(((b1 as u32) << 8 | b2 as u32) as i32);
            i += 1i32;
        }
    }
    Ok(h)";

fn has_field(ci: &ClassInfo, name: &str, desc: &str) -> bool {
    ci.fields().iter().any(|f| f.name == name && f.desc == desc)
}

/// 存根体：非 native / abstract 的 toString / hashCode 给 vtable 可安全调用的默认值，其余 panic
fn stub_body(ci: &ClassInfo, m: &Method) -> String {
    if m.is_native() || m.is_abstract() {
        let label = if m.is_native() { "native" } else { "stub" };
        return stub_call(label, &format!("{}.{}:{}", ci.name(), m.name, m.desc));
    }
    if m.name == "toString" && m.desc == lang::TO_STRING_DESC {
        return "Ok(String::from(Self::BINARY_NAME))".into();
    }
    if m.name == "hashCode" && m.desc == "()I" {
        return if has_field(ci, "value", "[B") && has_field(ci, "coder", "B") {
            STRING_HASH_BODY.into()
        } else {
            "Ok(0)".into()
        };
    }
    stub_call("stub", &format!("{}.{}:{}", ci.name(), m.name, m.desc))
}

/// 形参名：局部变量表名（缺失为 `arg{i}`）→ safe_ident → 重名加序号
fn param_names(m: &Method, n: usize, local_names: &BTreeMap<u16, String>) -> Vec<String> {
    let off = usize::from(!m.is_static());
    let mut seen: BTreeMap<String, u32> = BTreeMap::new();
    (0..n)
        .map(|i| {
            let raw = u16::try_from(i + off)
                .ok()
                .and_then(|s| local_names.get(&s))
                .cloned()
                .unwrap_or_else(|| format!("arg{i}"));
            let name = safe_ident(&raw);
            match seen.get_mut(&name) {
                Some(k) => {
                    *k += 1;
                    format!("{name}{k}")
                }
                None => {
                    seen.insert(name.clone(), 0);
                    name
                }
            }
        })
        .collect()
}

/// 生成方法 `m`（按 `ci` 发射）的 panic 存根。`class_type_params` 为空时取类的有效类型形参
pub fn native_stub(
    ctx: &EmitCtx<'_>,
    ci: &ClassInfo,
    m: &Method,
    rust_name: &str,
    class_type_params: &[String],
    local_names: &BTreeMap<u16, String>,
) -> Stub {
    let eff;
    let ctparams = if class_type_params.is_empty() {
        eff = ctx.ty.effective_class_type_params(ci).to_vec();
        eff.as_slice()
    } else {
        class_type_params
    };
    let sig = ctx.ty.emitted_method_sig_types(ci, m, ctparams);
    let names = param_names(m, sig.params.len(), local_names);
    let args: Vec<String> = names
        .iter()
        .zip(&sig.params)
        .map(|(n, t)| format!("{n}: {}", t.render(ctx.ty.names)))
        .collect();
    let args = args.join(", ");
    let ctor = ci.is_constructor(m);
    let sig_self = match (m.is_static() || ctor, args.is_empty()) {
        (true, _) => String::new(),
        (false, true) => "&self".into(),
        (false, false) => "&self, ".into(),
    };
    let ret_type = if ctor {
        "Result<Self>".to_string()
    } else {
        format!("Result<{}>", sig.ret.render(ctx.ty.names))
    };
    let fn_name = safe_ident(if rust_name.is_empty() { &m.name } else { rust_name });
    let body = stub_body(ci, m);
    if m.is_static() && m.name == "main" && m.desc == lang::MAIN_DESC {
        let sig = "pub fn main() -> Result<()>".to_string();
        let text = format!("{sig} {{\n    {body}\n}}");
        return Stub { sig, text };
    }
    let sig_line = format!("pub fn {fn_name}({sig_self}{args}) -> {ret_type}");
    let mut text = format!("{sig_line} {{\n    {body}\n}}");
    // 构造器双入口（K-5）：存根构造器同样提供 `__init_on` 孪生入口（子类 super(...) 调用点）
    if ctor {
        let twin = match fn_name.strip_prefix("new") {
            Some(rest) => format!("__init_on{rest}"),
            None => format!("__init_on_{fn_name}"),
        };
        let twin_args = if args.is_empty() { "this: Self".to_string() } else { format!("this: Self, {args}") };
        text.push_str(&format!(
            "\n\n#[doc(hidden)]\npub fn {twin}({twin_args}) -> {ret_type} {{\n    let _ = &this;\n    {body}\n}}"
        ));
    }
    Stub { sig: sig_line, text }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn param_names_dedup_and_default() {
        let m = Method {
            access: 0x0008,
            name: "f".into(),
            desc: "(II)V".into(),
            signature: None,
            code: None,
            exceptions: Vec::new(),
            annotations: Vec::new(),
            annotation_default: None,
            parameters: Vec::new(),
            synthetic_attr: false,
        };
        let mut ln = BTreeMap::new();
        ln.insert(0u16, "type".to_string());
        ln.insert(1u16, "type".to_string());
        assert_eq!(param_names(&m, 3, &ln), vec!["type_", "type_1", "arg2"]);
    }
}
