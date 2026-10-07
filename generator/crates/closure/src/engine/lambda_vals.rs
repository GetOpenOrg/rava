//! 引擎：lambda 实现方法的形参值——捕获值取创建点帧、SAM 实参取调用点帧，按实现方法形参位置对齐。
//!
//! 实现方法的实参序列 = indy 捕获值 ++ SAM 实参，前 `skip` 个绑定为接收者（虚 / 接口 / 特殊实现）。
//! 两段的值分属不同方法帧：捕获值的形参来源指创建方法的形参槽，SAM 实参的指调用方法的形参槽，
//! 形参常量、污染与字符串槽各按所属帧接入。任一段的值未知（非字节码调用方 / 物化的 lambda）则形参整体未知。

use super::*;

/// 进行中的 lambda 接边（`lambda_connect` 设置，`edge` 读取）
#[derive(Clone)]
pub(super) struct LambdaCap {
    /// 创建点（方法, 偏移）
    pub(super) site: (usize, u32),
    /// 创建点 indy 的实参值（None = 未知）
    pub(super) vals: Option<Rc<[V]>>,
    /// 绑定为实现方法接收者的前导实参数
    pub(super) skip: usize,
}

impl<'a> Engine<'a> {
    /// 调用点 m@off 经 lambda 接到实现方法 t（形参类型 ptypes，非接收者形参自 base 起）
    pub(super) fn bind_lambda_params(&mut self, m: usize, off: u32, t: usize, base: usize, ptypes: &[Option<u32>], c: &LambdaCap) {
        let n = ptypes.len();
        let (Some(cv), Some(sv)) = (c.vals.clone(), self.call_vals.clone()) else {
            self.bind_pvs(t, base, n, None);
            return;
        };
        let (cm, coff) = c.site;
        let kc = c.skip.min(cv.len());
        let ks = c.skip.saturating_sub(cv.len()).min(sv.len());
        let mid = (base + cv.len() - kc).min(n);
        self.taint_site(cm, t, base, mid, &cv[kc..]);
        self.taint_site(m, t, mid, n, &sv[ks..]);
        let pvs: Vec<PV> = cv[kc..].iter().chain(sv[ks..].iter()).map(PV::of).collect();
        self.join_pvs(t, base, n, Some(&pvs));
        let string = self.id(STRING);
        let is_str = |p: usize| ptypes.get(p).copied().flatten() == Some(string);
        let cap_at = |j: usize| (j >= kc).then(|| base + j - kc).filter(|&p| p < n);
        self.pstr_site(cm, coff, &cv, |j| cap_at(j).map(|p| pstrs::PSlot::M(t, p)), |j| cap_at(j).is_some_and(is_str));
        let sam_at = |j: usize| (j >= ks).then(|| mid + j - ks).filter(|&p| p < n);
        self.pstr_site(m, off, &sv, |j| sam_at(j).map(|p| pstrs::PSlot::M(t, p)), |j| sam_at(j).is_some_and(is_str));
    }
}
