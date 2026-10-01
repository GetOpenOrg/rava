//! A-5 函数式接口合成对象文本（← `emitter/sam_objects.synthesize`）。
//!
//! 对账本（[`crate::sam::SamLedger`]）内每个可合成接口，把 `I__Lambda` 合成对象追加到接口发射
//! 尾部：`ObjectVTable`（instanceof 名单 / `__interface` 应答）、各闭包接口的 `__VTable`
//! （SAM 条目直调闭包、default 条目经载体 `__default_<m>` 执行）、`TryFrom<Object>`。
//! 条目签名与 default 体有无取自发射记录（与落盘内容同源，不二次推导）。

use std::collections::{BTreeMap, BTreeSet};

use ty::ClassInfo;

use super::iface_impls::erased_declaration;
use super::sig::{idents, param_part, split_top_level_trimmed};
use super::uses::class_use_path;
use super::{class_params, Emissions};
use crate::ctx::EmitCtx;
use crate::emission::{ClassEmission, EmittedMethod};
use crate::error::{EmitError, Result};
use crate::lang;
use crate::sam::{contract_methods, SamSpec};

/// 接口 J 在宿主文件（接口 I 的文件）中的全限定类型路径
fn quote_path(ctx: &EmitCtx<'_>, jbin: &str, host: &ClassEmission, ems: &Emissions) -> String {
    class_use_path(ctx, jbin, &host.crate_prefix, Some(ems), &host.crate_name)
}

fn objects(n: usize) -> String {
    if n == 0 {
        String::new()
    } else {
        format!("<{}>", vec!["Object"; n].join(", "))
    }
}

/// 发射记录的方法 → (vtable 擦除条目签名（不含 `fn `）, 形参名, 条目形参类型)
fn entry_sig_parts(ctx: &EmitCtx<'_>, m: &EmittedMethod, jci: &ClassInfo) -> Option<(String, Vec<String>, Vec<String>)> {
    let tparams: BTreeSet<String> = class_params(ctx, jci).into_iter().collect();
    let erased = erased_declaration(ctx.ty.names, m, &tparams)?;
    let head = erased["fn ".len()..].to_string();
    let rest = &erased[erased.find('(')? + 1..erased.rfind(')')?];
    let parts = split_top_level_trimmed(rest);
    let names = parts.iter().skip(1).map(|p| p.split(':').next().unwrap_or("").trim().to_string()).collect();
    let types = parts.iter().skip(1).map(|p| p.split_once(':').map_or("", |x| x.1).trim().to_string()).collect();
    Some((head, names, types))
}

/// 载体声明签名 → (形参类型（去 self）, 返回类型文本)
fn declared_sig_parts(ctx: &EmitCtx<'_>, m: &EmittedMethod) -> Option<(Vec<String>, String)> {
    let sig = &m.signature(ctx.ty.names);
    if !sig.starts_with("pub fn ") || !sig.contains('(') {
        return None;
    }
    let (open, close) = (sig.find('(')?, sig.rfind(')')?);
    let parts = split_top_level_trimmed(sig.get(open + 1..close)?);
    let tys = parts.iter().skip(1).map(|p| p.split_once(':').map_or("", |x| x.1).trim().to_string()).collect();
    let ret = sig[close + 1..].trim();
    Some((tys, ret.strip_prefix("->").unwrap_or(ret).trim().to_string()))
}

fn mentions(ty: &str, tparams: &BTreeSet<String>) -> bool {
    idents(ty).any(|i| tparams.contains(i))
}

/// 类型文本中的类型变量代入 Object（整词）
fn subst_type_vars(ty: &str, tparams: &BTreeSet<String>) -> String {
    let map: BTreeMap<String, String> = tparams.iter().map(|t| (t.clone(), "Object".to_string())).collect();
    super::sig::substitute_type_params(ty, &map)
}

/// 类型串是否为接口载体形态（首标识符命中注册表内接口，且与其擦除载体文本相同）
fn is_carrier_type(ctx: &EmitCtx<'_>, ty: &str) -> bool {
    let head: String = ty.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
    if head.is_empty() {
        return false;
    }
    let Some(bin) = ctx.ty.names.binary_of(&head) else { return false };
    ctx.ty.carrier_type(bin).is_some_and(|c| c.render(ctx.ty.names) == ty)
}

/// default 条目体：`<J<Object,..> as From<Object>>::from(..).__default_m(..)`
fn default_entry_body(ctx: &EmitCtx<'_>, m: &EmittedMethod, kci: &ClassInfo, k_ty: &str, args: &[String], entry_tys: &[String]) -> Option<String> {
    let (param_tys, ret_ty) = declared_sig_parts(ctx, m)?;
    let tparams: BTreeSet<String> = class_params(ctx, kci).into_iter().collect();
    if param_tys.len() != args.len() {
        return None;
    }
    let mut call_args = Vec::new();
    for (i, (a, ty)) in args.iter().zip(&param_tys).enumerate() {
        let entry_ty = entry_tys.get(i).map_or("Object", String::as_str);
        if mentions(ty, &tparams) {
            call_args.push(format!("<{} as From<Object>>::from({a})", subst_type_vars(ty, &tparams)));
        } else if is_carrier_type(ctx, ty) && ty != entry_ty {
            call_args.push(format!("<{ty} as From<Object>>::from({a})"));
        } else {
            call_args.push(a.clone());
        }
    }
    let call = format!(
        "<{k_ty} as From<Object>>::from(Object::from(Clone::clone(self))).__default_{}({})",
        m.rust_name,
        call_args.join(", ")
    );
    Some(if mentions(&ret_ty, &tparams) { format!("Ok(Into::<Object>::into({call}?))") } else { call })
}

fn emission<'e>(ems: &'e Emissions, bin: &str) -> Result<&'e ClassEmission> {
    ems.get(bin).ok_or_else(|| EmitError::Assert(format!("[sam-objects] 接口未发射: {bin}")))
}

/// 闭包接口的 vtable 条目
fn vtable_entries(
    ctx: &EmitCtx<'_>,
    spec: &SamSpec,
    host: &ClassEmission,
    ems: &Emissions,
    jbin: &str,
    default_bodies: &BTreeMap<(String, String), (&str, &EmittedMethod)>,
) -> Result<Vec<String>> {
    let reg = ctx.ty.reg;
    let jci = reg.get(jbin).expect("impl 目标在注册表");
    let jem = emission(ems, jbin)?;
    let sam_key = (spec.sam_name.clone(), param_part(&spec.sam_desc).to_string());
    let mut entries = Vec::new();
    for jm in contract_methods(ctx, jci) {
        let key = (jm.name.clone(), param_part(&jm.desc).to_string());
        let Some(em_m) = jem.find(&jm.name, &key.1) else { continue };
        let Some((head, args, entry_tys)) = entry_sig_parts(ctx, em_m, jci) else { continue };
        let body = if key == sam_key {
            format!("(self.0)({})", args.join(", "))
        } else {
            let target = if !jm.is_abstract() { em_m.has_body.then_some((jbin, em_m)) } else { default_bodies.get(&key).copied() };
            let Some((kbin, em_k)) = target else { continue };
            let kci = reg.get(kbin).expect("default 声明接口在注册表");
            let k_ty = format!("{}{}", quote_path(ctx, kbin, host, ems), objects(class_params(ctx, kci).len()));
            let Some(b) = default_entry_body(ctx, em_k, kci, &k_ty, &args, &entry_tys) else { continue };
            b
        };
        entries.push(format!("    fn {head} {{ {body} }}"));
    }
    Ok(entries)
}

/// 单接口的合成对象文本
fn lambda_text(ctx: &EmitCtx<'_>, spec: &SamSpec, host: &ClassEmission, ems: &Emissions) -> Result<String> {
    let reg = ctx.ty.reg;
    let iface = spec.iface_bin.as_str();
    let lam = format!("{}__Lambda", ctx.short(iface));
    let fn_ty = format!("__Shared<__DynFn!(({}) -> Result<{}>)>", spec.erased_params.join(", "), spec.erased_ret);
    let mut l: Vec<String> = vec![
        "// ── A-5 函数式接口合成对象（LambdaMetafactory 产物的同构物）──".into(),
        format!("// {iface} 的 lambda 实例：SAM 闭包为存储，实现本接口及超接口的"),
        "// __VTable（SAM 条目直调闭包、default 条目经载体 __default_<m> 体执行）；".into(),
        "// __interface 查询对本接口及超接口闭包应答。".into(),
        "#[derive(Clone)]".into(),
        format!("pub struct {lam}(pub {fn_ty});"),
        String::new(),
        format!("impl {lam} {{"),
        format!("    pub fn new(f: {fn_ty}) -> Self {{ Self(f) }}"),
        "}".into(),
        String::new(),
    ];
    let mut patterns: BTreeSet<&str> = spec.closure.iter().map(String::as_str).collect();
    patterns.extend([iface, lang::OBJECT, lang::SERIALIZABLE]);
    let pats: Vec<String> = patterns.iter().map(|p| format!("\"{p}\"")).collect();
    l.push(format!("impl ObjectVTable for {lam} {{"));
    l.push("    fn as_any(&self) -> &dyn std::any::Any { self }".into());
    l.push(format!("    fn __obj_str(&self) -> std::string::String {{ std::format!(\"{iface}::Lambda\") }}"));
    l.push(format!("    fn __class_name(&self) -> &'static str {{ \"{iface}\" }}"));
    l.push("    fn is_instance_of(&self, type_id: &str) -> bool {".into());
    l.push(format!("        matches!(type_id, {})", pats.join(" | ")));
    l.push("    }".into());
    l.push("    fn __interface(self: __Shared<Self>, slot: &mut dyn std::any::Any) {".into());
    let mut targets: Vec<(&str, String)> = Vec::new();
    for jbin in &spec.closure {
        let Some(jci) = reg.get(jbin) else { continue };
        if contract_methods(ctx, jci).is_empty() {
            continue;
        }
        let jpath = quote_path(ctx, jbin, host, ems);
        l.push(format!(
            "        if let Some(s) = slot.downcast_mut::<Option<__Shared<dyn {jpath}__VTable>>>() {{ *s = Some(self); return; }}"
        ));
        targets.push((jbin, jpath));
    }
    l.push("    }".into());
    l.push("}".into());
    l.push(String::new());
    // default 执行载体：闭包内首个带翻译体的 default 声明
    let mut default_bodies: BTreeMap<(String, String), (&str, &EmittedMethod)> = BTreeMap::new();
    for (jbin, _) in &targets {
        let jci = reg.get(jbin).expect("impl 目标在注册表");
        let jem = emission(ems, jbin)?;
        for jm in contract_methods(ctx, jci).into_iter().filter(|m| !m.is_abstract()) {
            let pd = param_part(&jm.desc).to_string();
            if let Some(em_d) = jem.find(&jm.name, &pd).filter(|e| e.has_body) {
                default_bodies.entry((jm.name.clone(), pd)).or_insert((jbin, em_d));
            }
        }
    }
    for (jbin, jpath) in &targets {
        l.push(format!("impl {jpath}__VTable for {lam} {{"));
        l.extend(vtable_entries(ctx, spec, host, ems, jbin, &default_bodies)?);
        l.push("}".into());
        l.push(String::new());
    }
    let dotted = iface.replace('/', ".");
    l.push(format!("impl TryFrom<Object> for {lam} {{"));
    l.push("    type Error = JvmError;".into());
    l.push("    fn try_from(obj: Object) -> Result<Self> {".into());
    l.push("        obj.try_checkcast::<Self>().ok_or_else(|| JvmError::class_cast(".into());
    l.push(format!("            std::format!(\"class {{}} cannot be cast to {dotted}\","));
    l.push("                ObjectVTable::__class_name(&*obj.0).replace('/', \".\"))))".into());
    l.push("    }".into());
    l.push("}".into());
    Ok(l.join("\n"))
}

/// 账本内每个可合成接口：合成对象文本追加到接口发射尾部（全部 resolve_* 之后、落盘之前）
pub fn synthesize(ctx: &EmitCtx<'_>, ems: &mut Emissions) -> Result<()> {
    for (iface, spec) in &ctx.sam().specs {
        let host = emission(ems, iface)?;
        if host.handwritten {
            continue;
        }
        let text = lambda_text(ctx, spec, host, ems)?;
        let em = ems.get_mut(iface).expect("已校验存在");
        em.text = format!("{}\n\n{text}\n", em.text.trim_end_matches('\n'));
    }
    Ok(())
}
