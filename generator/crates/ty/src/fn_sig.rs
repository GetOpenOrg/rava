//! 实例方法声明签名的结构化形态：`pub fn name(&self, p: T, ..) -> Result<R>`。
//!
//! 定义侧（方法体 / 存根）产出、第二阶段在接收方视角重渲染；类型以 [`RsType`] 承载身份，
//! 文本只在 [`FnSig::render`] 出口按调用方给的命名产生。

use crate::rs_type::RsType;
use crate::short_names::ShortNames;

/// 实例方法签名（形参不含 `self` 接收者；返回类型为 `Result<..>` 的内层）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnSig {
    /// Rust 方法名
    pub name: String,
    /// (形参名, 类型)
    pub params: Vec<(String, RsType)>,
    /// `Result<..>` 的内层（void 为 `()`）
    pub ret: RsType,
}

impl Default for FnSig {
    fn default() -> FnSig {
        FnSig { name: String::new(), params: Vec::new(), ret: RsType::Unit }
    }
}

impl FnSig {
    /// 签名行 `pub fn name(&self, a: T) -> Result<R>`（无方法体、形参不带 mut）
    pub fn render(&self, names: &ShortNames) -> String {
        let mut s = format!("pub fn {}(&self", self.name);
        for (n, t) in &self.params {
            s.push_str(", ");
            s.push_str(n);
            s.push_str(": ");
            s.push_str(&t.render(names));
        }
        s.push_str(") -> Result<");
        s.push_str(&self.ret.render(names));
        s.push('>');
        s
    }

    /// 形参名表
    pub fn param_names(&self) -> Vec<String> {
        self.params.iter().map(|(n, _)| n.clone()).collect()
    }

    /// 类型形参代入（形参与返回类型逐项结构化代入）
    pub fn substitute(&self, mapping: &dyn Fn(&str) -> Option<RsType>) -> FnSig {
        FnSig {
            name: self.name.clone(),
            params: self.params.iter().map(|(n, t)| (n.clone(), t.substitute(mapping))).collect(),
            ret: self.ret.substitute(mapping),
        }
    }

    /// 改名副本
    pub fn renamed(&self, name: &str) -> FnSig {
        FnSig { name: name.to_string(), ..self.clone() }
    }

    /// 签名引用的全部类（binary，渲染序，含重复）
    pub fn classes(&self) -> Vec<&str> {
        let mut out = Vec::new();
        for (_, t) in &self.params {
            t.collect_classes(&mut out);
        }
        self.ret.collect_classes(&mut out);
        out
    }
}
