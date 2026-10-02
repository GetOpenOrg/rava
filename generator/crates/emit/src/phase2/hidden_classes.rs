//! lambda 隐藏类声明（LambdaMetafactory 在调用点链接时定义的隐藏类的编译期同构物）。
//!
//! JVM 为每个 lambda 调用点定义一个隐藏类：以调用者类命名（`Caller$$Lambda/0x…`），超类
//! Object，直接超接口为 samtype（+ 标记接口 / Serializable），`ACC_FINAL | ACC_SYNTHETIC`，
//! 不可经 `Class.forName` 按名取得。原生镜像中站点的合成对象（`I__Lambda`）携带隐藏类名作为
//! 运行时类；本模块为全部登记站点生成属性声明块（`rava_macros::hidden_class!`，宏展开为空），
//! 由 java_meta 构建脚本与 `java_class!` 块同一协议扫描进反射元数据表：Class.getSuperclass /
//! getInterfaces / getModifiers / isHidden / isAssignableFrom 都从该类自己的元数据应答。
//!
//! 声明块追加在站点发射所在类的文件尾（声明层，拆层不进实现 crate）；同一隐藏类（继承展开 /
//! vtable 体中的同一调用点副本）只声明一次。

use std::collections::BTreeSet;

use indexmap::IndexMap;

use super::Emissions;
use crate::body::SamSite;
use crate::class_writer::attrs::{class_modifiers_str, q};
use crate::ctx::EmitCtx;
use crate::error::{EmitError, Result};
use crate::lang;
use crate::sam::iface_closure;

/// `ACC_FINAL | ACC_SYNTHETIC`：InnerClassLambdaMetafactory 生成类的修饰符（Class.getModifiers）
const LAMBDA_MODIFIERS: u16 = 0x0010 | 0x1000;
/// 类文件 access_flags 原值：修饰符 + `ACC_SUPER`（Class.getClassAccessFlagsRaw0）
const ACC_SUPER: u16 = 0x0020;

/// 隐藏类名中的调用者类（`Caller$$Lambda/0x…` 的前缀）
fn host_of(hidden: &str) -> &str {
    hidden.rsplit_once("$$Lambda/").map_or(hidden, |(h, _)| h)
}

/// 单个隐藏类的声明块文本
fn declaration(ctx: &EmitCtx<'_>, site: &SamSite) -> String {
    let mut supers: Vec<String> = vec![site.hidden.clone(), lang::OBJECT.to_string()];
    let mut seen: BTreeSet<String> = supers.iter().cloned().collect();
    for i in &site.interfaces {
        for s in iface_closure(ctx, i) {
            if seen.insert(s.clone()) {
                supers.push(s);
            }
        }
    }
    let mut l = vec![
        "rava_macros::hidden_class! {".to_string(),
        format!("    #[binary_name       = \"{}\"]", q(&site.hidden)),
        format!("    #[super_class       = \"{}\"]", lang::OBJECT),
        format!("    #[interfaces        = \"{}\"]", q(&site.interfaces.join(","))),
        format!("    #[modifiers         = \"{}\"]", class_modifiers_str(LAMBDA_MODIFIERS)),
        format!("    #[class_access_flags = \"{}\"]", LAMBDA_MODIFIERS | ACC_SUPER),
    ];
    if let Some(loader) = ctx.defining_loader(host_of(&site.hidden)) {
        l.push(format!("    #[defining_loader   = \"{loader}\"]"));
    }
    l.push(format!("    #[all_supertypes    = \"{}\"]", q(&supers.join(";"))));
    l.push("    #[is_hidden         = true]".to_string());
    l.push("}".to_string());
    l.join("\n")
}

/// 全部登记站点的隐藏类声明追加到站点所在类的发射文本尾（全部方法体生成之后）
pub fn declare(ctx: &EmitCtx<'_>, sites: &[SamSite], ems: &mut Emissions) -> Result<()> {
    let mut by_class: IndexMap<&str, Vec<&SamSite>> = IndexMap::new();
    let mut declared: BTreeSet<&str> = BTreeSet::new();
    for site in sites {
        if declared.insert(site.hidden.as_str()) {
            by_class.entry(site.class.as_str()).or_default().push(site);
        }
    }
    for (class, list) in by_class {
        let em = ems
            .get_mut(class)
            .ok_or_else(|| EmitError::Assert(format!("[hidden-classes] 站点所在类未发射: {class}")))?;
        let decls: Vec<String> = list.iter().map(|s| declaration(ctx, s)).collect();
        em.text = format!(
            "{}\n\n// ── lambda 隐藏类（调用点链接期定义的类，元数据由 java_meta 扫描）──\n{}\n",
            em.text.trim_end_matches('\n'),
            decls.join("\n")
        );
    }
    Ok(())
}
