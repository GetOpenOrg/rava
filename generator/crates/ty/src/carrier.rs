//! 接口擦除载体（`jvm_type.carrier_type` / `carrier_type_for_ident`）。

use crate::rs_type::RsType;
use crate::TyCtx;

impl TyCtx<'_> {
    /// 接口 → 擦除载体 `Short<Object, ..>`（形参数取有效类型形参）；
    /// 非接口 / 注册表外 → None
    pub fn carrier_type(&self, binary: &str) -> Option<RsType> {
        let ci = self.reg.get(binary)?;
        if !ci.is_interface() {
            return None;
        }
        let n = self.effective_class_type_params(ci).len();
        Some(RsType::class(binary, RsType::objects(n)))
    }

    /// 类型头名命中注册表内接口（短名反查）→ 载体；否则 None
    pub fn carrier_type_for_ident(&self, t: &RsType) -> Option<RsType> {
        let head = t.head_name(self.names)?;
        let binary = self.names.binary_of(&head)?;
        self.carrier_type(binary)
    }

    /// `carrier_type_for_ident(t) == t`（按渲染文本比较，与 Python 串比较同口径）
    pub fn is_carrier(&self, t: &RsType) -> bool {
        self.carrier_type_for_ident(t)
            .is_some_and(|c| c.render(self.names) == t.render(self.names))
    }
}
