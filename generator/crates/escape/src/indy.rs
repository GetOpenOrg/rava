//! invokedynamic：lambda 站点（区域内新建函数对象，捕获值为其内容）与字符串拼接
//! （引用实参经 `concat_stringify` 入口字符串化，结果为运行时新建的字符串）、record 的
//! equals / hashCode / toString（只读接收者字段，对字段值调用同名方法）。其余引导方法拒绝。

use classfile::{Insn, Operand};
use input::manifest::IndyKind;
use ty::registry::ClassInfo;

use crate::analysis::{reject, Analysis, NodeKind, ALL, CAP, R};
use crate::step::pop;
use crate::val::{Frame, Refs, Val};
use crate::world::method_kinds;

pub fn invoke_dynamic<'a>(a: &mut Analysis<'_, 'a>, ci: &'a ClassInfo, insn: &Insn, f: &mut Frame) -> R<bool> {
    let Operand::InvokeDynamic { bsm, name, desc, .. } = &insn.operand else {
        return reject("invokedynamic 操作数".into());
    };
    let (pk, rk) = method_kinds(desc);
    let mut args = Vec::with_capacity(pk.len());
    for _ in 0..pk.len() {
        args.push(pop(f)?);
    }
    args.reverse();
    if a.shallow {
        if rk.is_some() {
            f.stack.push(Val::old());
        }
        return Ok(true);
    }
    match a.w.indy_kind(ci, *bsm) {
        Some(IndyKind::Lambda) => {
            let Some(site) = a.w.lambda_site(ci, *bsm, name, desc) else {
                return reject(format!("lambda 站点无法解析：{}", ci.name()));
            };
            let n = a.node(NodeKind::Lambda(site));
            let mut cap = Refs::default();
            for v in &args {
                cap.union_with(&v.refs());
            }
            a.store(&Refs::single(n), CAP, &cap, &String::new)?;
            f.stack.push(Val::Ref(Refs::single(n)));
            Ok(true)
        }
        Some(IndyKind::Concat) => {
            let Some((hci, hidx, string_class)) = a.w.concat_helper() else {
                return reject("缺 concat_stringify 入口".into());
            };
            for v in args.iter().filter(|v| matches!(v, Val::Ref(_) | Val::Live)) {
                if a.call_target(hci, hidx, vec![Val::Ref(v.refs())])?.is_none() && !v.refs().is_empty() {
                    // 字符串化不正常返回（只经异常离开）
                    return Ok(false);
                }
            }
            let n = a.node(NodeKind::Made(string_class));
            f.stack.push(Val::Ref(Refs::single(n)));
            Ok(true)
        }
        Some(IndyKind::ObjectMethods) => object_methods(a, name, &args, rk.is_some(), f),
        k => reject(format!("不支持的引导方法类别 {:?}：{}", k.map(|k| k.as_str()), ci.name())),
    }
}

/// record 的 `ObjectMethods` 引导：生成的方法只读组件字段，对字段值做相应的虚调用 / 字符串化，
/// 不保存实参。字段内容取实参节点内容（字段不敏感）
fn object_methods(a: &mut Analysis<'_, '_>, name: &str, args: &[Val], has_ret: bool, f: &mut Frame) -> R<bool> {
    let Some(root) = a.w.root_name() else { return reject("缺根类".into()) };
    let fields: Vec<Refs> = args.iter().map(|v| a.load(&v.refs(), ALL)).collect();
    let recv = fields.first().cloned().unwrap_or_default();
    match name {
        "toString" => {
            let Some((hci, hidx, string_class)) = a.w.concat_helper() else {
                return reject("缺 concat_stringify 入口".into());
            };
            if !recv.is_empty() && a.call_target(hci, hidx, vec![Val::Ref(recv)])?.is_none() {
                return Ok(false);
            }
            let n = a.node(NodeKind::Made(string_class));
            f.stack.push(Val::Ref(Refs::single(n)));
        }
        "hashCode" => {
            if !recv.is_empty() {
                a.call_virtual(root, "hashCode", "()I", vec![Val::Ref(recv)])?;
            }
            f.stack.push(Val::int_top());
        }
        "equals" => {
            let other = fields.get(1).cloned().unwrap_or_default();
            if !recv.is_empty() {
                a.call_virtual(root, "equals", &format!("(L{root};)Z"), vec![Val::Ref(recv), Val::Ref(other)])?;
            }
            f.stack.push(Val::int_top());
        }
        _ => return reject(format!("ObjectMethods 未知方法：{name}")),
    }
    debug_assert!(has_ret);
    Ok(true)
}
