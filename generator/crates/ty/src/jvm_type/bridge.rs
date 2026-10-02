//! 发射侧 [`RsType`] 与查询侧 [`JvmType`] 的桥（`from_rs_type` / `rust_head_name`）。
//!
//! Python 侧的 `from_rust_type` 解析 Rust 类型**文本**；Rust 侧类型本就是结构化节点，
//! 这里按结构直接转换（名字语义与文本解析逐点一致：类型形参名先于短名反查，
//! 反查不到的名字成为 binary = 名字本身的占位类引用）。

use std::collections::BTreeSet;

use super::{JvmType, PrimKind};
use crate::rs_type::{Prim, RsType};
use crate::TyCtx;

fn prim_kind(p: Prim) -> PrimKind {
    match p {
        Prim::I8 => PrimKind::Byte,
        Prim::I16 => PrimKind::Short,
        Prim::I32 => PrimKind::Int,
        Prim::I64 => PrimKind::Long,
        Prim::F32 => PrimKind::Float,
        Prim::F64 => PrimKind::Double,
        Prim::Bool => PrimKind::Boolean,
        Prim::U16 => PrimKind::Char,
    }
}

impl TyCtx<'_> {
    /// 名字（Rust 短名）→ 类引用：注册表短名反查（补 is_interface），否则占位
    fn class_by_short(&self, name: &str, args: Vec<JvmType>) -> JvmType {
        match self.binary_of(name).and_then(|b| self.reg.get(&b)) {
            Some(ci) => JvmType::class_with(ci.name(), args, ci.is_interface()),
            None => JvmType::class_with(name, args, false),
        }
    }

    /// RsType → JvmType
    pub fn from_rs_type(&self, t: &RsType, tparams: &BTreeSet<String>) -> JvmType {
        match t {
            RsType::Prim(p) => JvmType::Primitive(prim_kind(*p)),
            RsType::Unit => JvmType::Primitive(PrimKind::Void),
            RsType::Object => JvmType::object_type(),
            RsType::Class { binary, args }
                if args.is_empty() && tparams.contains(self.short(binary).as_str()) =>
            {
                // 文本口径：无实参的头名是作用域内类型形参 → 类型变量（形参名遮蔽同名短类名）
                JvmType::type_var(self.short(binary), None)
            }
            RsType::Class { binary, args } => {
                let args = args.iter().map(|a| self.from_rs_type(a, tparams)).collect();
                JvmType::class_with(
                    binary,
                    args,
                    self.reg.get(binary).is_some_and(|c| c.is_interface()),
                )
            }
            RsType::Bare { .. } => {
                let head = t.head_name(self).unwrap_or_default();
                if tparams.contains(&head) {
                    JvmType::type_var(head, None)
                } else {
                    self.class_by_short(&head, Vec::new())
                }
            }
            RsType::Array(e) => JvmType::array(self.from_rs_type(e, tparams)),
            RsType::Param(n) if tparams.contains(n) => JvmType::type_var(n, None),
            RsType::Param(n) => self.class_by_short(n, Vec::new()),
        }
    }

    /// 类型的 Rust 头名（去实参）：类 → 短名；数组 → `JArray`；基本类型 → Rust 拼写；
    /// 类型变量 → 名字；通配符 → `?`；null → 空串
    pub fn rust_head_name(&self, t: &JvmType) -> String {
        match t {
            JvmType::Class { binary, .. } => self.short(binary),
            JvmType::Array(_) => "JArray".to_string(),
            JvmType::Primitive(k) => k.rust_name().to_string(),
            JvmType::TypeVar { name, .. } => name.clone(),
            JvmType::HostPrim(h) => h.rust_name().to_string(),
            JvmType::Wildcard { .. } => "?".to_string(),
            JvmType::Null => String::new(),
        }
    }
}
