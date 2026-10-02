//! 字段声明类型恢复与 static 字段解析（← `sim/fields.py` 的 `_restore_field_declared_type` /
//! `_resolve_static_field` / `_static_field_decl_class`）。

use sim::StackSim;
use ty::{ClassInfo, RsType};

use crate::build::ty_text;
use crate::env::InstrEnv;
use crate::hierarchy::{short_binary, type_binary};
use crate::owner::{field_generic_signature, resolve_static_field_owner};

/// 可见性校验的内建名（`_BUILTIN_G`）：Rust 内建容器与已知类型短名
const BUILTIN: [&str; 7] = [ir::anchors::OBJECT, ir::anchors::STRING, "Rc", "__Shared", "Vec", "RefCell", ir::anchors::ARRAY];
/// `PRIMITIVE_RUST_TYPES`（装箱类型实参映射为基本类型，同样视为可见）
const PRIMITIVE_NAMES: [&str; 12] = ["i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f32", "f64", "bool", "usize"];

/// 类型文本中的标识符（`[A-Za-z_][A-Za-z0-9_]*` 的逐个匹配）
fn idents(s: &str) -> Vec<&str> {
    let mut out = Vec::new();
    for run in s.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')) {
        let start = run.find(|c: char| c.is_ascii_alphabetic() || c == '_');
        if let Some(i) = start {
            out.push(&run[i..]);
        }
    }
    out
}

/// 解析结果中的类型名在调用方均可见（调用方类型形参 / 注册表短名 / 内建名）
fn all_visible(env: &InstrEnv, t: &RsType, caller_tps: &[String]) -> bool {
    let names = &env.ctx.ty;
    idents(&ty_text(env, t)).into_iter().all(|n| {
        caller_tps.iter().any(|p| p == n) || names.is_registry_short(n) || BUILTIN.contains(&n) || PRIMITIVE_NAMES.contains(&n)
    })
}

/// 类型中含菱形推断占位 `_`
fn contains_infer(t: &RsType) -> bool {
    match t {
        RsType::Param(n) => n == sim::INFER_PARAM,
        _ => t.type_args().iter().any(contains_infer),
    }
}

fn declares_instance_field(ci: &ClassInfo, fname: &str) -> bool {
    ci.fields().iter().any(|f| !f.is_static() && ty::ident::safe_ident(&f.name) == fname)
}

/// 按字段 generic_signature 恢复声明类型（getfield / putfield 共用）。
///
/// `ftype` 是描述符擦除形态；沿继承链查字段声明，在声明类形参上下文中解析为精确泛型形态；
/// 声明在接收者静态类型的祖先上时按接收者视角的超类实参代入。解析结果中的类型名在调用方
/// 不可见（跨类形参名不同）时保持擦除形态。
pub fn restore_field_declared_type(env: &InstrEnv, sim: &StackSim, f_owner: &str, fname: &str, ftype: RsType, recv_ty: Option<&RsType>) -> RsType {
    field_view(env, sim, f_owner, fname, ftype, recv_ty).0
}

/// 字段声明类型恢复 + 访问器擦除标记：第二项为真表示字段槽位是类型变量、接收者实例化把它
/// 代入为 Object（宏访问器按实例化返回 Object），而描述符擦除类型（上界）不是 Object——
/// 读取值须经 `From<Object>` 取回上界视图
pub fn field_view(env: &InstrEnv, sim: &StackSim, f_owner: &str, fname: &str, ftype: RsType, recv_ty: Option<&RsType>) -> (RsType, bool) {
    let ctx = &env.ctx;
    let reg = ctx.reg();
    if reg.is_empty() {
        return (ftype, false);
    }
    let g_owner = if f_owner.is_empty() { ctx.class_name } else { f_owner };
    if g_owner.is_empty() {
        return (ftype, false);
    }
    let ref_ci = reg.get(g_owner);
    // 字段解析（JVMS §5.4.3.2）：声明类可能是限定类的祖先，类型变量属于声明类
    let mut g_ci = ref_ci;
    while let Some(c) = g_ci {
        if declares_instance_field(c, fname) {
            break;
        }
        g_ci = if c.super_class().is_empty() { None } else { reg.get(c.super_class()) };
    }
    let g_ci = g_ci.or(ref_ci);
    // 内部类外部引用字段（this$N，无 generic_signature）：与 struct 字段定义同规则
    let mut parsed = g_ci.and_then(|g| {
        let f = g.fields().iter().find(|f| !f.is_static() && ty::ident::safe_ident(&f.name) == fname)?;
        ctx.ty.outer_ref_field_type(&f.name, &f.desc, &ctx.ty.effective_class_type_params(g))
    });
    if parsed.is_none() {
        let gsig = field_generic_signature(reg, g_owner, fname);
        let Some(g) = g_ci.filter(|_| !gsig.is_empty()) else {
            return (ftype, false);
        };
        parsed = ctx.ty.parse_field_type(&gsig, &ctx.ty.effective_class_type_params(g));
    }
    let (Some(mut p), Some(g)) = (parsed, g_ci) else {
        return (ftype, false);
    };
    let slot_is_var = matches!(p, RsType::Param(_));
    if let Some(rt) = recv_ty {
        p = recv_view(env, p, g, ref_ci, rt);
    }
    if !sim::types::is_object(&p) && ty_text(env, &p) != ty_text(env, &ftype) && all_visible(env, &p, &sim.cfg.class_type_params) {
        return (p, false);
    }
    let erased = slot_is_var && sim::types::is_object(&p) && !sim::types::is_object(&ftype) && !sim::types::is_scalar(&ftype);
    (ftype, erased)
}

/// 声明类类型变量按接收者视角代入：祖先上声明 → 祖先链实参；接收者自身类声明 → 接收者实参
fn recv_view(env: &InstrEnv, parsed: RsType, g: &ClassInfo, ref_ci: Option<&ClassInfo>, recv_ty: &RsType) -> RsType {
    let ctx = &env.ctx;
    let recv_ci = type_binary(ctx, recv_ty).and_then(|b| ctx.reg().get(&b));
    let recv_ci = recv_ci.or_else(|| ref_ci.filter(|r| r.name() != g.name()));
    let Some(recv_ci) = recv_ci else {
        return parsed;
    };
    let owner_params = ctx.ty.effective_class_type_params(g);
    let recv_args = recv_ty.type_args();
    if recv_ci.name() != g.name() {
        let self_args = if recv_args.is_empty() { None } else { Some(recv_args) };
        let anc = ctx.ty.ancestor_type_args(recv_ci, self_args);
        let Some((_, owner_args)) = anc.into_iter().find(|(b, _)| b == g.name()) else {
            return parsed;
        };
        return parsed.substitute(&|n| {
            let i = owner_params.iter().position(|p| p == n)?;
            Some(owner_args.get(i).cloned().unwrap_or(RsType::Object))
        });
    }
    // 字段就声明在接收者自己的类上：宏访问器按接收者实例化返回类型实参（A-3 形态 2）
    let same_as_params = recv_args.len() == owner_params.len()
        && recv_args.iter().zip(owner_params.iter()).all(|(a, p)| ty_text(env, a) == *p);
    if owner_params.is_empty() || recv_args.len() != owner_params.len() || same_as_params || recv_args.iter().any(contains_infer) {
        return parsed;
    }
    parsed.substitute(&|n| owner_params.iter().position(|p| p == n).map(|i| recv_args[i].clone()))
}

/// static 字段解析结果：(声明类, 访问器名, 声明类型, turbofish)
pub struct StaticField {
    pub class: String,
    pub accessor: String,
    pub ty: RsType,
    pub turbofish: Vec<RsType>,
}

/// getstatic / putstatic 共用：常量池类可以是子类，static 字段实际声明在祖先类 / 父接口，
/// 访问器生成在声明类上；泛型类静态字段访问带 turbofish（E0283）
pub fn resolve_static_field(env: &InstrEnv, cls: &str, raw_name: &str, desc: &str) -> StaticField {
    let ctx = &env.ctx;
    let reg = ctx.reg();
    let field_name = ty::ident::safe_ident(raw_name);
    let mut ty = if desc.is_empty() { RsType::Object } else { ctx.ty.jvm_to_rust(desc) };
    let cls = if cls.is_empty() { String::new() } else { resolve_static_field_owner(reg, cls, raw_name).unwrap_or_else(|| cls.to_string()) };
    let cls_ci = if reg.is_empty() || cls.is_empty() {
        None
    } else if reg.contains(&cls) {
        reg.get(&cls)
    } else {
        short_binary(ctx, &ctx.short(&cls)).and_then(|b| reg.get(&b))
    };
    let mut turbofish = Vec::new();
    let mut accessor = field_name.clone();
    if let Some(ci) = cls_ci {
        // 有效形参：含内部 / 局部类从外围作用域继承的类型变量（struct 的泛型形参同源）
        turbofish = RsType::objects(ctx.ty.effective_class_type_params(ci).len());
        // 静态字段声明类型恢复（与 getfield 同规则）：getter 按字段级签名生成返回类型
        let sgsig = ci.fields().iter().find(|f| f.is_static() && f.name == field_name).map(ty::registry::field_signature).unwrap_or_default();
        if !sgsig.is_empty() {
            let tps = ty::class_params::parse_class_type_params(ci.generic_signature());
            if let Some(p) = ctx.ty.parse_field_type(sgsig, &tps) {
                if !sim::types::is_object(&p) && ty_text(env, &p) != ty_text(env, &ty) && all_visible(env, &p, &[]) {
                    ty = p;
                }
            }
        }
        // 字段名与方法名冲突：emitter 生成 `name_field` 后缀
        // （比较用 Java 原名：转义后的名字与方法原名不可比）
        if ci.methods().iter().any(|m| m.name == raw_name) {
            accessor = ty::ident::safe_ident(&format!("{raw_name}_field"));
        }
    }
    StaticField { class: cls, accessor, ty, turbofish }
}

#[cfg(test)]
mod tests {
    use super::idents;

    #[test]
    fn idents_follow_regex() {
        assert_eq!(idents("Foo_Node<K, Bar<i32>>"), vec!["Foo_Node", "K", "Bar", "i32"]);
        assert_eq!(idents("<3abc>"), vec!["abc"]);
    }
}
