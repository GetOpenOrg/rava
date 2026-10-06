//! 字符串形状转移单测：构建器拼接链、前后缀判定收窄、合流丢标签整组撤掉。

use super::*;

/// 桩 Oracle：`p/SB` 是清单构建器，`p/S.startsWith` 是前缀判定
struct Strs;

impl Oracle for Strs {
    fn invoke_result(&self, _: u8, _: &MemberRef, _: bool, _: &[V]) -> Ret {
        Ret::Unknown
    }
    fn field(&self, _: u8, _: &MemberRef, _: Option<&V>) -> Option<V> {
        None
    }
    fn type_live(&self, _: &str) -> bool {
        true
    }
    fn str_kind(&self, _: u8, m: &MemberRef, _: bool) -> Option<StrKind> {
        match (m.owner.as_str(), m.name.as_str()) {
            ("p/SB", "<init>") => Some(StrKind::Init),
            ("p/SB", "append") => Some(StrKind::Append),
            ("p/SB", "toString") => Some(StrKind::Result),
            ("p/S", "startsWith") => Some(StrKind::StartsWith),
            _ => None,
        }
    }
}

fn mref(owner: &str, name: &str, desc: &str) -> MemberRef {
    MemberRef { owner: owner.into(), name: name.into(), desc: desc.into() }
}

fn code(insns: Vec<(u32, u8, Operand)>, len: u32, max_locals: u16) -> Code {
    let insns = insns.into_iter().map(|(offset, opcode, operand)| Insn { offset, opcode, operand }).collect();
    Code { max_stack: 4, max_locals, code_len: len, insns, exception_table: vec![] }
}

fn returned(a: &Analysis) -> Vec<V> {
    a.events.iter().filter_map(|(_, e)| if let Event::Return(v) = e { Some(v.clone()) } else { None }).collect()
}

/// `if (!p.startsWith("/")) p = "/" + p; return p;`：两条路径的返回值都以 "/" 开头
#[test]
fn prefix_after_guard_and_concat() {
    let init = mref("p/SB", "<init>", "()V");
    let app = mref("p/SB", "append", "(Ljava/lang/String;)Lp/SB;");
    let c = code(
        vec![
            (0, 0x2a, Operand::None),
            (1, op::LDC, Operand::Ldc(Const::String("/".into()))),
            (3, op::INVOKEVIRTUAL, Operand::Method(mref("p/S", "startsWith", "(Ljava/lang/String;)Z"), false)),
            (6, 0x9a, Operand::Branch(29)), // ifne
            (9, op::NEW, Operand::Class("p/SB".into())),
            (12, 0x59, Operand::None),
            (13, op::INVOKESPECIAL, Operand::Method(init, false)),
            (16, op::LDC, Operand::Ldc(Const::String("/".into()))),
            (18, op::INVOKEVIRTUAL, Operand::Method(app.clone(), false)),
            (21, 0x2a, Operand::None),
            (22, op::INVOKEVIRTUAL, Operand::Method(app, false)),
            (25, op::INVOKEVIRTUAL, Operand::Method(mref("p/SB", "toString", "()Ljava/lang/String;"), false)),
            (28, 0x4b, Operand::None), // astore_0
            (29, 0x2a, Operand::None),
            (30, 0xb0, Operand::None),
        ],
        31,
        1,
    );
    let a = analyze("p/A", "(Ljava/lang/String;)Ljava/lang/String;", true, &c, &Strs);
    assert!(!a.conservative);
    let r = returned(&a);
    assert_eq!(r.len(), 1);
    let sh = r[0].str_shape().expect("返回值带形状");
    assert_eq!(sh.starts_with("/"), Some(true));
    assert_eq!(r[0].nonnull(), Some(true));
    // 来源不变：形参与 toString 调用点（按来源上溯的求值照旧）
    assert_eq!(r[0].srcs().as_ref(), [Src::Param(0), Src::Site(25)]);
}

/// 构建器与另一引用合流后经合流值追加：原局部变量的标签整组撤掉，取结果不再是确定的空串
#[test]
fn join_losing_tag_drops_group() {
    let init = mref("p/SB", "<init>", "()V");
    let app = mref("p/SB", "append", "(Ljava/lang/String;)Lp/SB;");
    let c = code(
        vec![
            (0, op::NEW, Operand::Class("p/SB".into())),
            (3, 0x59, Operand::None),
            (4, op::INVOKESPECIAL, Operand::Method(init, false)),
            (7, 0x4d, Operand::None), // astore_2
            (8, 0x1a, Operand::None), // iload_0
            (9, 0x99, Operand::Branch(16)),
            (12, 0x2c, Operand::None), // aload_2
            (13, op::GOTO, Operand::Branch(17)),
            (16, 0x2b, Operand::None), // aload_1
            (17, op::LDC, Operand::Ldc(Const::String("a".into()))),
            (19, op::INVOKEVIRTUAL, Operand::Method(app, false)),
            (22, 0x57, Operand::None), // pop
            (23, 0x2c, Operand::None), // aload_2
            (24, op::INVOKEVIRTUAL, Operand::Method(mref("p/SB", "toString", "()Ljava/lang/String;"), false)),
            (27, 0xb0, Operand::None),
        ],
        28,
        3,
    );
    let a = analyze("p/A", "(ILp/SB;)Ljava/lang/String;", true, &c, &Strs);
    assert!(!a.conservative);
    let r = returned(&a);
    assert_eq!(r.len(), 1);
    assert!(r[0].str_shape().is_none_or(|s| s.equals("").is_none()), "{:?}", r[0]);
}

/// 语句式追加（`sb.append("x"); sb.append(p); return sb.toString();`）：同组各份拷贝一起更新
#[test]
fn statement_appends_update_all_copies() {
    let init = mref("p/SB", "<init>", "()V");
    let app = mref("p/SB", "append", "(Ljava/lang/String;)Lp/SB;");
    let c = code(
        vec![
            (0, op::NEW, Operand::Class("p/SB".into())),
            (3, 0x59, Operand::None),
            (4, op::INVOKESPECIAL, Operand::Method(init, false)),
            (7, 0x4c, Operand::None), // astore_1
            (8, 0x2b, Operand::None),
            (9, op::LDC, Operand::Ldc(Const::String("x".into()))),
            (11, op::INVOKEVIRTUAL, Operand::Method(app.clone(), false)),
            (14, 0x57, Operand::None),
            (15, 0x2b, Operand::None),
            (16, 0x2a, Operand::None),
            (17, op::INVOKEVIRTUAL, Operand::Method(app, false)),
            (20, 0x57, Operand::None),
            (21, 0x2b, Operand::None),
            (22, op::INVOKEVIRTUAL, Operand::Method(mref("p/SB", "toString", "()Ljava/lang/String;"), false)),
            (25, 0xb0, Operand::None),
        ],
        26,
        2,
    );
    let a = analyze("p/A", "(Ljava/lang/String;)Ljava/lang/String;", true, &c, &Strs);
    let r = returned(&a);
    let sh = r[0].str_shape().expect("返回值带形状");
    assert_eq!(sh.starts_with("x"), Some(true));
    assert_eq!(sh.equals("x"), None);
}
