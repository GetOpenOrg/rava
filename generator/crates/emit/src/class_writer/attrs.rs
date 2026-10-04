//! 元数据属性（← `emitter/attrs.py`）：访问标志字符串、字段 / 方法属性行、
//! 常量值与注解常量池编码。类块头在 [`super::head`]。

use classfile::extras::{AnnoConst, FieldExtras, MethodExtras};
use classfile::{Const, Field, Method};
use ty::Names;

use crate::text::{hex, py_float_repr};

pub const ACC_PUBLIC: u16 = 0x0001;
pub const ACC_PRIVATE: u16 = 0x0002;
pub const ACC_PROTECTED: u16 = 0x0004;
pub const ACC_STATIC: u16 = 0x0008;
pub const ACC_FINAL: u16 = 0x0010;
pub const ACC_SYNCHRONIZED: u16 = 0x0020;
pub const ACC_VOLATILE: u16 = 0x0040;
pub const ACC_TRANSIENT: u16 = 0x0080;
pub const ACC_NATIVE: u16 = 0x0100;
pub const ACC_INTERFACE: u16 = 0x0200;
pub const ACC_ABSTRACT: u16 = 0x0400;
pub const ACC_SYNTHETIC: u16 = 0x1000;
pub const ACC_ANNOTATION: u16 = 0x2000;
pub const ACC_ENUM: u16 = 0x4000;

/// 访问权限字符串（public / private / protected / package）
pub fn access_str(flags: u16) -> &'static str {
    if flags & ACC_PUBLIC != 0 {
        "public"
    } else if flags & ACC_PRIVATE != 0 {
        "private"
    } else if flags & ACC_PROTECTED != 0 {
        "protected"
    } else {
        "package"
    }
}

fn join_flags(flags: u16, table: &[(u16, &str)]) -> String {
    table
        .iter()
        .filter(|(bit, _)| flags & bit != 0)
        .map(|(_, s)| *s)
        .collect::<Vec<_>>()
        .join(" ")
}

/// 类修饰符串。private / protected 只出现在成员类的 InnerClasses 条目（类文件顶层 access 无此两位，
/// protected 成员类顶层记为 public），故由本串承载；public 位由 `access` 属性承载
pub fn class_modifiers_str(flags: u16) -> String {
    join_flags(
        flags,
        &[
            (ACC_PRIVATE, "private"),
            (ACC_PROTECTED, "protected"),
            (ACC_FINAL, "final"),
            (ACC_ABSTRACT, "abstract"),
            (ACC_INTERFACE, "interface"),
            (ACC_ENUM, "enum"),
            (ACC_ANNOTATION, "annotation"),
            (ACC_SYNTHETIC, "synthetic"),
            (ACC_STATIC, "static"),
        ],
    )
}

pub fn field_modifiers_str(flags: u16) -> String {
    join_flags(
        flags,
        &[
            (ACC_STATIC, "static"),
            (ACC_FINAL, "final"),
            (ACC_VOLATILE, "volatile"),
            (ACC_TRANSIENT, "transient"),
            (ACC_SYNTHETIC, "synthetic"),
        ],
    )
}

pub fn method_modifiers_str(flags: u16) -> String {
    join_flags(
        flags,
        &[
            (ACC_STATIC, "static"),
            (ACC_FINAL, "final"),
            (ACC_SYNCHRONIZED, "synchronized"),
            (ACC_NATIVE, "native"),
            (ACC_ABSTRACT, "abstract"),
            (ACC_VOLATILE, "bridge"),
            (ACC_TRANSIENT, "varargs"),
            (ACC_SYNTHETIC, "synthetic"),
        ],
    )
}

/// `"` / `\` 转义（属性字符串值）
pub fn q(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// ConstantValue 的元数据文本（← `classfile._constant_value_str`；空串 = 非常量）
pub fn constant_value_str(c: &Const) -> String {
    match c {
        Const::Int(v) => v.to_string(),
        Const::Long(v) => v.to_string(),
        Const::Float(bits) => {
            let v = f32::from_bits(*bits);
            if v.is_nan() {
                "NaN".into()
            } else {
                py_float_repr(f64::from(v))
            }
        }
        Const::Double(bits) => {
            let v = f64::from_bits(*bits);
            if v.is_nan() {
                "NaN".into()
            } else {
                py_float_repr(v)
            }
        }
        Const::String(s) => ir::render::escape_str(s),
        // 元数据文本只作展示；常量值本身由 `utf16_const_literal` 无损发射
        Const::StringUtf16(u) => ir::render::escape_str(&String::from_utf16_lossy(u)),
        _ => String::new(),
    }
}

/// 稀疏注解常量池编码 `idx:K:值;…`（← `classfile.encode_anno_cpool`）；字符串 `U` = UTF-8 字节 hex，
/// 含孤立代理项时 `W` = UTF-16 码元 hex（每码元 4 位），不经有损文本
pub fn anno_cpool_str(pool: &std::collections::BTreeMap<u16, AnnoConst>) -> String {
    pool.iter()
        .map(|(idx, c)| match c {
            AnnoConst::Utf8(s) => format!("{idx}:U:{}", hex(s.as_bytes())),
            AnnoConst::Utf16(u) => format!("{idx}:W:{}", u.iter().map(|x| format!("{x:04x}")).collect::<String>()),
            AnnoConst::Int(v) => format!("{idx}:I:{v}"),
            AnnoConst::Long(v) => format!("{idx}:J:{v}"),
            AnnoConst::Float(b) => format!("{idx}:F:{b:08x}"),
            AnnoConst::Double(b) => format!("{idx}:D:{b:016x}"),
        })
        .collect::<Vec<_>>()
        .join(";")
}

/// 字段属性行 `#[cfg_attr(any(), java_field(...))]`；`reflect`：静态字段可按名反射（宏据此展开 `__STATICS` 项）
pub fn field_attr(f: &Field, fx: Option<&FieldExtras>, reflect: bool) -> String {
    let mut parts = vec![format!("name = \"{}\"", f.name), format!("descriptor = \"{}\"", f.desc)];
    if f.access != 0 {
        let a = access_str(f.access);
        if a != "package" {
            parts.push(format!("access = \"{a}\""));
        }
        let mods = field_modifiers_str(f.access);
        if !mods.is_empty() {
            parts.push(format!("modifiers = \"{mods}\""));
        }
    }
    if f.is_static() {
        parts.push("is_static = true".into());
    }
    if let Some(sig) = f.signature.as_deref().filter(|s| !s.is_empty()) {
        parts.push(format!("generic_signature = \"{}\"", sig.replace('"', "\\\"")));
    }
    let cv = f.constant_value.as_ref().map(constant_value_str).unwrap_or_default();
    if !cv.is_empty() {
        parts.push(format!("constant_value = \"{cv}\""));
    }
    if fx.is_some_and(|x| x.deprecated) {
        parts.push("is_deprecated = true".into());
    }
    if let Some(x) = fx.filter(|x| !x.raw_annotations.is_empty()) {
        parts.push(format!("raw_annotations = \"{}\"", hex(&x.raw_annotations)));
    }
    if reflect {
        parts.push("reflect = true".into());
    }
    format!("#[cfg_attr(any(), java_field({}))]", parts.join(", "))
}

/// 方法属性行的发射期附加信息（Python 在 ParsedMethod 副本上挂的动态属性）
#[derive(Debug, Clone, Default)]
pub struct MethodAttrExtra {
    /// 槽位归属类 binary（空 = 不占槽）
    pub virtual_in: String,
    pub vtable_name: String,
    pub vtable_erasure: Vec<String>,
    pub handwritten_body: bool,
    /// 覆盖方法未被分派到：槽条目发 `__stub` 存根（漏派发显式失败）
    pub slot_stub: bool,
    /// 复制进本类的方法体的声明类型 binary（接口 default 体 / 未覆盖的用户超类虚方法体；空 = 本类
    /// 声明）：类文件里该方法不属于本类，反射声明表与栈帧归属都以声明类型为准
    pub declared_by: String,
    /// 入口可省栈界检查（[`leaf_entry`]）
    pub leaf: bool,
}

/// 叶子方法字节码长度上限：省掉入口栈界检查后，叶子帧落在检查点之间的 `SHADOW` 余量里，
/// 限长保证该帧远小于余量（手写方法体不计叶子：手写代码的调用不可见）。
const LEAF_MAX_CODE_LEN: u32 = 512;

/// 字节码可判定的叶子方法体（无调用指令且足够短）
fn is_leaf_body(code: &classfile::Code) -> bool {
    code.code_len <= LEAF_MAX_CODE_LEN && code.is_leaf()
}

/// 方法入口可省栈界检查（a3-T1b-2，计划 §21.8.2）：经该入口执行的恰是本方法体且它是叶子。
/// 可覆盖的虚方法不计——其 wrapper 入口可能分派到子类的非叶子覆盖体，省掉检查会让
/// 「经叶子声明的虚调用」构成的递归环上没有检查点；入口唯一对应本体的只有 static / private /
/// 构造器 / 类初始化 / final 方法，以及承载类为 final 的方法。
pub fn leaf_entry(m: &Method, host_final: bool) -> bool {
    let exact = m.is_static() || m.is_private() || m.is_init() || m.is_clinit() || m.is_final() || host_final;
    exact && m.code.as_ref().is_some_and(is_leaf_body)
}

/// 方法元数据标注行（`#[java_method(...)]` / native 为 `#[native]\n#[java_native(...)]`）；
/// `virtual_in` 按 `names` 渲染为槽位类 Rust 名
pub fn method_attr(m: &Method, mx: Option<&MethodExtras>, extra: &MethodAttrExtra, names: &dyn Names) -> String {
    let esc = |s: &str| s.replace('"', "\\\"");
    let tag = if m.is_native() { "java_native" } else { "java_method" };
    let mut parts = vec![format!("name = \"{}\"", esc(&m.name)), format!("descriptor = \"{}\"", esc(&m.desc))];
    if m.access != 0 {
        let a = access_str(m.access);
        if a != "package" {
            parts.push(format!("access = \"{a}\""));
        }
        let mods = method_modifiers_str(m.access);
        if !mods.is_empty() {
            parts.push(format!("modifiers = \"{mods}\""));
        }
    }
    let flags = [
        (m.is_static(), "is_static    = true"),
        (m.is_native(), "is_native    = true"),
        (m.is_abstract(), "is_abstract  = true"),
        (m.is_synthetic(), "is_synthetic = true"),
    ];
    parts.extend(flags.iter().filter(|(on, _)| *on).map(|(_, s)| s.to_string()));
    if !m.exceptions.is_empty() {
        parts.push(format!("exceptions = \"{}\"", esc(&m.exceptions.join(","))));
    }
    if let Some(sig) = m.signature.as_deref().filter(|s| !s.is_empty()) {
        parts.push(format!("generic_signature = \"{}\"", esc(sig)));
    }
    if mx.is_some_and(|x| x.deprecated) {
        parts.push("is_deprecated = true".into());
    }
    if !extra.virtual_in.is_empty() {
        parts.push(format!("virtual_in = \"{}\"", esc(&names.short(&extra.virtual_in))));
        if !extra.vtable_name.is_empty() {
            parts.push(format!("vtable_name = \"{}\"", esc(&extra.vtable_name)));
        }
        if extra.slot_stub {
            parts.push("slot_stub = \"true\"".into());
        }
    }
    if !extra.vtable_erasure.is_empty() {
        parts.push(format!("vtable_erasure = \"{}\"", esc(&extra.vtable_erasure.join(";"))));
    }
    if extra.handwritten_body {
        parts.push("body = \"handwritten\"".into());
    }
    if extra.leaf && !extra.handwritten_body {
        parts.push("leaf = \"true\"".into());
    }
    if !extra.declared_by.is_empty() {
        parts.push(format!("declared_by = \"{}\"", esc(&extra.declared_by)));
    }
    if !m.parameters.is_empty() {
        let mp: Vec<String> = m
            .parameters
            .iter()
            .map(|(n, a)| format!("{}:{a}", n.as_deref().unwrap_or("")))
            .collect();
        parts.push(format!("method_parameters = \"{}\"", esc(&mp.join(";"))));
    }
    if let Some(x) = mx {
        for (bytes, key) in [
            (&x.raw_annotations, "raw_annotations"),
            (&x.raw_param_annotations, "raw_param_annotations"),
            (&x.raw_annotation_default, "raw_annotation_default"),
        ] {
            if !bytes.is_empty() {
                parts.push(format!("{key} = \"{}\"", hex(bytes)));
            }
        }
    }
    let body = format!("#[{tag}({})]", parts.join(", "));
    if m.is_native() {
        format!("#[native]\n{body}")
    } else {
        body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn code(ops: &[u8], code_len: u32) -> classfile::Code {
        let insns = ops
            .iter()
            .enumerate()
            .map(|(i, &opcode)| classfile::Insn { offset: i as u32, opcode, operand: classfile::Operand::None })
            .collect();
        classfile::Code { max_stack: 2, max_locals: 2, code_len, insns, exception_table: vec![] }
    }

    #[test]
    fn leaf_body_from_bytecode() {
        use classfile::op;
        // getfield + ireturn：叶子
        assert!(is_leaf_body(&code(&[0x2a, op::GETFIELD, 0xac], 5)));
        // 任一调用指令（含 invokedynamic）都不是叶子
        for inv in [op::INVOKEVIRTUAL, op::INVOKESPECIAL, op::INVOKESTATIC, op::INVOKEINTERFACE, op::INVOKEDYNAMIC] {
            assert!(!is_leaf_body(&code(&[0x2a, inv, 0xac], 5)), "opcode {inv:#x}");
        }
        // 超长的无调用方法体不计叶子（帧须落在 SHADOW 余量内）
        assert!(!is_leaf_body(&code(&[0xac], LEAF_MAX_CODE_LEN + 1)));
        assert!(is_leaf_body(&code(&[0xac], LEAF_MAX_CODE_LEN)));
    }

    #[test]
    fn leaf_entry_requires_exact_target() {
        use classfile::acc;
        let m = |access: u16, name: &str, ops: &[u8]| Method {
            access,
            name: name.into(),
            desc: "()I".into(),
            signature: None,
            code: Some(code(ops, 4)),
            exceptions: vec![],
            annotations: vec![],
            annotation_default: None,
            parameters: vec![],
            synthetic_attr: false,
        };
        let leaf_ops = [0x2a, classfile::op::GETFIELD, 0xac];
        // 可覆盖的虚方法：即使本体是叶子，入口也可能分派到非叶子覆盖体
        assert!(!leaf_entry(&m(acc::PUBLIC, "get", &leaf_ops), false));
        // 入口唯一对应本体
        assert!(leaf_entry(&m(acc::PUBLIC, "get", &leaf_ops), true));
        assert!(leaf_entry(&m(acc::PUBLIC | acc::FINAL, "get", &leaf_ops), false));
        assert!(leaf_entry(&m(acc::PRIVATE, "get", &leaf_ops), false));
        assert!(leaf_entry(&m(acc::STATIC, "get", &leaf_ops), false));
        // 非叶子体 / 无体
        assert!(!leaf_entry(&m(acc::STATIC, "get", &[classfile::op::INVOKESTATIC, 0xac]), false));
        let mut abs = m(acc::PUBLIC | acc::FINAL, "get", &leaf_ops);
        abs.code = None;
        assert!(!leaf_entry(&abs, false));
    }

    #[test]
    fn modifiers_and_access() {
        assert_eq!(access_str(0x0011), "public");
        assert_eq!(access_str(0x0010), "package");
        assert_eq!(class_modifiers_str(0x0411 | ACC_STATIC), "final abstract static");
        assert_eq!(class_modifiers_str(0x060A), "private abstract interface static");
        assert_eq!(method_modifiers_str(0x1041), "bridge synthetic");
    }

    #[test]
    fn constant_values() {
        assert_eq!(constant_value_str(&Const::Float(1.5f32.to_bits())), "1.5");
        assert_eq!(constant_value_str(&Const::Float(0.1f32.to_bits())), "0.10000000149011612");
        assert_eq!(constant_value_str(&Const::Double(f64::NAN.to_bits())), "NaN");
        assert_eq!(constant_value_str(&Const::Double(f64::INFINITY.to_bits())), "inf");
    }

    #[test]
    fn anno_cpool_strings() {
        let pool: std::collections::BTreeMap<u16, AnnoConst> =
            [(1, AnnoConst::Utf8("ab".into())), (4, AnnoConst::Utf16(vec![0xD800, 0x41])), (5, AnnoConst::Int(-2))]
                .into_iter()
                .collect();
        assert_eq!(anno_cpool_str(&pool), "1:U:6162;4:W:d8000041;5:I:-2");
        assert_eq!(constant_value_str(&Const::String("a\"\\\n\u{1}".into())), "a\\\"\\\\\\n\\u{0001}");
    }
}
