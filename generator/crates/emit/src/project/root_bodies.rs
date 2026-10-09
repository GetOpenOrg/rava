//! 根类（`Domain::Root`，不入注册表、无 `java_class!` 块）非 native 方法的字节码翻译体。
//!
//! 根类的 struct 与 vtable 是运行时契约（`object.rs`），但它的方法语义以自身字节码为准
//! （手写边界规范 §一）：`equals` / `toString` / `wait` 三个重载 / `finalize` 由方法体生成器按根类
//! 字节码翻译成自由函数 `<根类型>__<fn>_body(this: &<根类型>, ..)`，手写层只留 native 方法与调用入口
//! （`object_impl.rs` 的固有方法、`ObjectVTable` 默认方法、`<根类型>__<fn>_base` 均转交这些自由函数）。
//!
//! 落盘形态与生成类的拆层同构：方法体放首个实现 crate 的 `body/<根类路径>_body.rs`（`#[export_name]`），
//! 声明层同路径文件放外部声明块（`#[link_name]`，链接符号与宏拆层同一规则）——方法体引用的类
//! （`StringBuilder` 等）可能在声明层上层段，不能直接落在声明层底段。无实现 crate 时整体落声明层。
//! 不在档案调用链上的方法同样生成同签名的 panic 存根（运行时契约总要链接到这组符号）。

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use ty::{ClassInfo, NameScope};

use crate::body::{BodyError, BodyRequest, MethodBodyEmitter};
use crate::ctx::{EmitCtx, HwAudit, ProjectState};
use crate::error::{EmitError, Result};
use crate::project::layers::BodyPlan;
use crate::project::line_tables::ROOT_BODIES_OPEN;

/// 生成标记（`fs::GEN_MARKER`）：复用 scratch 时识别为生成文件（未写入即清扫）
const GEN_NOTE: &str = "// 生成文件（rava_macros::java_class 同类生成标记）：根类方法体，勿手改";

/// 一个根类方法体（自由函数文本，不含属性行）
pub struct RootFn {
    pub name: String,
    pub desc: String,
    pub text: String,
}

/// 根类方法体集合（发射后、落盘前）
pub struct RootBodies {
    class: String,
    source: String,
    /// 文件作用域（导入块由调用方按根类所在 crate 求出）
    pub scope: Arc<NameScope>,
    pub fns: Vec<RootFn>,
}

/// 根类方法的 Rust 名：根类同名重载取描述符后缀名（前提是该名在手写 API 名面中），与调用侧
/// （`invoke_virtual` 的根类命名）、行表登记同源
pub fn rust_name(ctx: &EmitCtx<'_>, name: &str, desc: &str) -> String {
    let mangled = ty::type_map::mangle_name(&ctx.manifest.ty, name, desc);
    let chosen = if mangled != name && ctx.root_api().contains(&mangled) { mangled } else { name.to_string() };
    ty::ident::safe_ident(&chosen)
}

/// 根类类型名（Rust 侧短名）
fn root_ty(ctx: &EmitCtx<'_>, root: &ClassInfo) -> String {
    ctx.short(root.name())
}

/// 自由函数名：`<根类型>__<fn>_body`
pub fn body_fn_name(root_ty: &str, rust: &str) -> String {
    format!("{root_ty}__{rust}_body")
}

/// 翻译根类全部非 native 字节码方法（构造器 / 类初始化除外）；根类不可装载时 None。
/// 档案链外方法生成 panic 存根；生成期事实并入 `state`；非 native 方法未翻译者计入手写审计
pub fn emit(ctx: &EmitCtx<'_>, state: &mut ProjectState, bodies: &dyn MethodBodyEmitter) -> Result<Option<RootBodies>> {
    let Some(root) = ctx.input.root.as_ref() else { return Ok(None) };
    let class = root.name().to_string();
    let source = ctx.cp.get(&class).and_then(|cf| cf.source_file.clone()).unwrap_or_default();
    let scope = Arc::new(NameScope::new(&class, ctx.ty.global_names()));
    let sctx = ctx.scoped(&scope);
    let rty = root_ty(ctx, root);
    let mut counts: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
    for m in root.methods() {
        *counts.entry(m.name.as_str()).or_default() += 1;
    }
    let overloaded: BTreeSet<String> = counts.iter().filter(|(_, n)| **n > 1).map(|(k, _)| (*k).to_string()).collect();
    let mut fns = Vec::new();
    for m in root.methods() {
        if m.is_native() || m.is_abstract() || m.is_static() {
            continue;
        }
        if m.name.starts_with('<') {
            // 构造器：根类构造器体为单条 return（对象身份由分配建立），等价于无体；否则即手写近似
            if m.name == "<init>" && m.code.as_ref().is_some_and(|c| c.code_len > 1) {
                state.hw_audit.push((HwAudit::Override, format!("{class}.{}:{}", m.name, m.desc)));
            }
            continue;
        }
        if m.code.is_none() {
            continue;
        }
        let rust = rust_name(ctx, &m.name, &m.desc);
        let key = format!("{class}.{}:{}", m.name, m.desc);
        let translated = if ctx.in_chain(&class, &m.name, &m.desc) {
            let req = BodyRequest {
                class: root,
                method: m,
                declaring_class: &class,
                type_var_view: None,
                class_type_params: &[],
                overloaded_names: &overloaded,
                rust_name: Some(&rust),
                in_vtable_body: false,
                site: "root",
            };
            match bodies.emit_body(&sctx, &req, &mut state.body_log) {
                Ok(out) => {
                    state.absorb(&out.effects);
                    Some(out.text)
                }
                Err(BodyError::Fallback(_)) => None,
                Err(BodyError::Fatal(s)) => return Err(EmitError::Body(s)),
            }
        } else {
            None
        };
        let method_text = match translated {
            Some(t) => t,
            None => {
                let ex = ctx.extras(&class);
                let index = root.methods().iter().position(|x| x.name == m.name && x.desc == m.desc);
                let names = index.and_then(|i| ex.methods.get(i)).map(|x| x.local_names()).unwrap_or_default();
                let stub = crate::class_writer::stub::native_stub(&sctx, root, m, &rust, &[], &names);
                format!("{} {{\n    {}\n}}", stub.sig, crate::precheck::stub_call("stub", &key))
            }
        };
        let text = free_fn(&method_text, &rust, &rty).map_err(|e| EmitError::Assert(format!("{key}：{e}")))?;
        fns.push(RootFn { name: m.name.clone(), desc: m.desc.clone(), text });
    }
    Ok(Some(RootBodies { class, source, scope, fns }))
}

/// 实例方法文本（`pub fn <rust>(&self, ..) -> R {` + `let this = self;` 序言）→ 根类型自由函数
/// `pub fn <根类型>__<rust>_body(this: &<根类型>, ..) -> R`（行布局不变，行标记照旧）
fn free_fn(text: &str, rust: &str, rty: &str) -> std::result::Result<String, String> {
    let head = format!("pub fn {rust}(&self");
    let lines: Vec<&str> = text.lines().skip_while(|l| l.starts_with("// java: ")).collect();
    let first = lines.first().ok_or("空方法体")?;
    let rest = first.strip_prefix(&head).ok_or_else(|| format!("签名不是实例方法形态：{first}"))?;
    let mut out = format!("pub fn {}(this: &{rty}{rest}\n", body_fn_name(rty, rust));
    for l in &lines[1..] {
        if l.trim() == "let this = self;" {
            continue;
        }
        out.push_str(&l.replace("Self::", &format!("{rty}::")));
        out.push('\n');
    }
    Ok(out)
}

impl RootBodies {
    /// 行表方法属性行（注释形态：自由函数不带 `#[java_method]` 宏属性）
    fn method_attr(f: &RootFn) -> String {
        format!("// #[java_method(name = \"{}\", descriptor = \"{}\")]\n", f.name, f.desc)
    }

    /// 文件头：lint 放行 + 生成标记 + 预导入 + 导入块 + 行表头
    fn header(&self, imports: &[String]) -> String {
        let mut h = format!("{}\n{GEN_NOTE}\nuse crate::prelude::*;\n", crate::class_writer::FILE_ALLOW);
        for l in imports {
            h.push_str(l);
            h.push('\n');
        }
        h.push_str(&format!("\n{ROOT_BODIES_OPEN}{} {}\n", self.class, self.source));
        h
    }

    /// 落盘文本：方法体进首个实现 crate（`#[export_name]`），返回声明层文件 (路径, 文本)（外部声明块）；
    /// 无实现 crate 时声明层文件即完整方法体
    pub fn place(&self, imports: &[String], body_plan: &mut BodyPlan, decl_src: &Path) -> Result<(PathBuf, String)> {
        let rel = PathBuf::from(class_rel(&self.class));
        let decl_path = decl_src.join(&rel);
        let header = self.header(imports);
        let Some(first) = body_plan.crates.first_mut() else {
            let mut full = header;
            for f in &self.fns {
                full.push_str(&Self::method_attr(f));
                full.push_str(&f.text);
                full.push('\n');
            }
            return Ok((decl_path, full));
        };
        let mut body = header;
        let mut decl = format!("{}\n{GEN_NOTE}\nuse crate::prelude::*;\n", crate::class_writer::FILE_ALLOW);
        for l in imports {
            decl.push_str(l);
            decl.push('\n');
        }
        for f in &self.fns {
            let (sym, block) = rava_macros_core::plan::free_fn_link(&self.class, &f.text)
                .map_err(|e| EmitError::Assert(format!("{}.{}:{}：链接拆分失败：{e}", self.class, f.name, f.desc)))?;
            body.push_str(&Self::method_attr(f));
            body.push_str(&format!("#[export_name = \"{sym}\"]\n"));
            body.push_str(&f.text);
            body.push('\n');
            decl.push_str(&block);
            decl.push('\n');
        }
        first.files.insert(rel, body);
        Ok((decl_path, decl))
    }
}

/// 根类方法体文件的相对路径：类文件路径加 `_body` 后缀（`java/lang/Object` → `java/lang/object_body.rs`）
fn class_rel(binary: &str) -> String {
    let (pkg, simple) = binary.rsplit_once('/').unwrap_or(("", binary));
    let file = format!("{}_body.rs", crate::text::to_snake(simple));
    if pkg.is_empty() { file } else { format!("{pkg}/{file}") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn free_fn_rewrites_receiver() {
        let text = "// java: m(J)V\npub fn m_l(&self, x: i64) -> Result<()> {\n    let this = self;\n    Self::k(this, x)?; // line 3\n    Ok(())\n}";
        let out = free_fn(text, "m_l", "R").expect("实例方法形态");
        assert_eq!(out, "pub fn R__m_l_body(this: &R, x: i64) -> Result<()> {\n    R::k(this, x)?; // line 3\n    Ok(())\n}\n");
        assert!(free_fn("pub fn s() -> Result<()> {\n}", "s", "R").is_err());
        assert_eq!(class_rel("p/q/Root"), "p/q/root_body.rs");
    }
}
