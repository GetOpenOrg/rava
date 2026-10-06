//! 引导求值的污点标量（计划 2026-10-05-boot-image-evaluator §3.2「延迟值的传播」，第 2 步）。
//!
//! 宿主标量（清单 op `host_scalar:<下界>:<上界>`，如 `availableProcessors`、`maxMemory`、`getAppend`）不再
//! 构建期取零值，而是返回污点值 [`CV::T`]：表达式表中的一项，带闭区间（宿主取值的全部可能）。
//! - 算术、比较、类型转换传播污点（区间按整数语义计算，回绕即取全域）；区间退化为单点即为常量；
//! - 在污点上分支：区间可判定即按判定走；否则在两侧路径各自执行到分支的直接后支配点并合并
//!   （`taint_fork.rs`），不同值合并为条件选择表达式；
//! - 其余取具体值的用法（数组下标 / 长度、switch、传给 native 等）即延迟值参与求值，按第 1 步规则残差化；
//! - 写入映像的污点值物化为**启动重算槽**：启动时按表达式重新求值（宿主源为 native 调用）。

use std::fmt::Write as _;

use super::vm::*;
use super::*;

/// 污点表达式
#[derive(Clone, Debug)]
pub(super) enum TOp {
    /// 宿主源：native 调用（实参为构建期值）
    Src { native: Rc<str>, args: Vec<CV> },
    /// 一元：取负 / 类型转换（操作码）
    Un(u8, CV),
    /// 二元算术 / 移位 / 位运算 / lcmp（操作码）
    Bin(u8, CV, CV),
    /// 条件选择：`cmp(a, b) ? t : f`（cmp 为分支比较种类 0..6：eq ne lt ge gt le）
    Sel { cmp: u8, a: CV, b: CV, t: CV, f: CV },
}

#[derive(Clone, Debug)]
pub(super) struct TExpr {
    pub op: TOp,
    pub lo: i64,
    pub hi: i64,
}

/// 污点表与统计
#[derive(Default)]
pub(super) struct Taints {
    pub exprs: Vec<TExpr>,
    /// 路径细化：(表达式, 下界, 上界)，分支两侧路径执行期间压栈
    pub refine: Vec<(u32, i64, i64)>,
    /// 统计：按区间判定的分支 / 两侧合并的分支 / 合并出的选择表达式
    pub decided: u64,
    pub forks: u64,
    pub selects: u64,
    /// 分支两侧合并的嵌套深度
    pub depth: u32,
}

const CMP: [&str; 6] = ["==", "!=", "<", ">=", ">", "<="];

fn full(wide: bool) -> (i64, i64) {
    if wide {
        (i64::MIN, i64::MAX)
    } else {
        (i64::from(i32::MIN), i64::from(i32::MAX))
    }
}

/// i128 区间收进类型值域（越界即回绕：取全域）
fn fit(lo: i128, hi: i128, wide: bool) -> (i64, i64) {
    let (a, b) = full(wide);
    if lo < i128::from(a) || hi > i128::from(b) {
        (a, b)
    } else {
        (lo as i64, hi as i64)
    }
}

/// 比较种类 k 在区间上的判定
pub(super) fn decide(k: u8, (al, ah): (i64, i64), (bl, bh): (i64, i64)) -> Option<bool> {
    let lt = if ah < bl {
        Some(true)
    } else if al >= bh {
        Some(false)
    } else {
        None
    };
    let gt = if al > bh {
        Some(true)
    } else if ah <= bl {
        Some(false)
    } else {
        None
    };
    let eq = if al == ah && bl == bh && al == bl {
        Some(true)
    } else if ah < bl || bh < al {
        Some(false)
    } else {
        None
    };
    match k {
        0 => eq,
        1 => eq.map(|x| !x),
        2 => lt,
        3 => lt.map(|x| !x),
        4 => gt,
        _ => gt.map(|x| !x),
    }
}

/// 比较两侧交换后的种类（`a k b` ⇔ `b k' a`）
fn mirror(k: u8) -> u8 {
    [0, 1, 4, 5, 2, 3][k as usize]
}

/// 比较种类取反
pub(super) fn negate(k: u8) -> u8 {
    [1, 0, 3, 2, 5, 4][k as usize]
}

/// `x k c` 成立时 x 的区间（None = 不可能）
fn narrow(k: u8, (lo, hi): (i64, i64), c: i64) -> Option<(i64, i64)> {
    let (lo, hi) = match k {
        0 => (lo.max(c), hi.min(c)),
        1 if lo == c => (lo.saturating_add(1), hi),
        1 if hi == c => (lo, hi.saturating_sub(1)),
        1 => (lo, hi),
        2 => (lo, hi.min(c.saturating_sub(1))),
        3 => (lo.max(c), hi),
        4 => (lo.max(c.saturating_add(1)), hi),
        _ => (lo, hi.min(c)),
    };
    (lo <= hi).then_some((lo, hi))
}

impl Vm {
    /// 值的区间（具体整数为单点；非整数为 None）
    pub(super) fn trange(&self, v: CV) -> Option<(i64, i64)> {
        match v {
            CV::I(x) => Some((i64::from(x), i64::from(x))),
            CV::J(x) => Some((x, x)),
            CV::T(id, _) => {
                let r = self.bj.taint.refine.iter().rev().find(|(i, _, _)| *i == id).map(|&(_, lo, hi)| (lo, hi));
                r.or_else(|| self.bj.taint.exprs.get(id as usize).map(|e| (e.lo, e.hi)))
            }
            _ => None,
        }
    }

    /// 新建污点值；区间退化为单点即常量
    pub(super) fn tmk(&mut self, op: TOp, wide: bool, (lo, hi): (i64, i64)) -> CV {
        if lo == hi {
            return if wide { CV::J(lo) } else { CV::I(lo as i32) };
        }
        let id = self.bj.taint.exprs.len() as u32;
        self.bj.taint.exprs.push(TExpr { op, lo, hi });
        CV::T(id, if wide { b'J' } else { b'I' })
    }

    /// 宿主源（`host_scalar:<下界>:<上界>`，缺省为返回类型全域；boolean 为 0..1）
    pub(super) fn tsrc(&mut self, native: &str, args: Vec<CV>, ret: &str, spec: &str) -> R<CV> {
        let wide = ret == "J";
        let (mut lo, mut hi) = if ret == "Z" { (0, 1) } else { full(wide) };
        let mut it = spec.split(':').skip(1);
        let bound = |s: Option<&str>, d: i64| -> R<i64> {
            match s {
                None | Some("") => Ok(d),
                Some("max") => Ok(full(wide).1),
                Some("min") => Ok(full(wide).0),
                Some(x) => x.parse().map_or_else(|_| fail(format!("宿主标量区间 {spec}")), Ok),
            }
        };
        lo = bound(it.next(), lo)?;
        hi = bound(it.next(), hi)?;
        if !matches!(ret, "I" | "J" | "Z" | "B" | "C" | "S") || lo > hi {
            return fail(format!("宿主标量类型 / 区间 {native} {spec}"));
        }
        Ok(self.tmk(TOp::Src { native: Rc::from(native), args }, wide, (lo, hi)))
    }

    /// 二元算术（含 lcmp）：至少一侧为污点
    pub(super) fn tbin(&mut self, op: u8, a: CV, b: CV) -> R<CV> {
        let (Some(ra), Some(rb)) = (self.trange(a), self.trange(b)) else { return fail(format!("算术操作数类型不符 {op:#x}")) };
        let wide = matches!(op, 0x61 | 0x65 | 0x69 | 0x6d | 0x71 | 0x79 | 0x7b | 0x7d | 0x7f | 0x81 | 0x83);
        let (al, ah, bl, bh) = (i128::from(ra.0), i128::from(ra.1), i128::from(rb.0), i128::from(rb.1));
        let corners = |f: &dyn Fn(i128, i128) -> i128| {
            let c = [f(al, bl), f(al, bh), f(ah, bl), f(ah, bh)];
            (*c.iter().min().unwrap_or(&0), *c.iter().max().unwrap_or(&0))
        };
        let r = match op {
            0x94 => {
                let (lt, gt, eq) = (decide(2, ra, rb), decide(4, ra, rb), decide(0, ra, rb));
                match (lt, gt, eq) {
                    (Some(true), _, _) => (-1, -1),
                    (_, Some(true), _) => (1, 1),
                    (_, _, Some(true)) => (0, 0),
                    _ => (if lt == Some(false) { 0 } else { -1 }, if gt == Some(false) { 0 } else { 1 }),
                }
            }
            0x60 | 0x61 => fit(al + bl, ah + bh, wide),
            0x64 | 0x65 => fit(al - bh, ah - bl, wide),
            0x68 | 0x69 => {
                let (lo, hi) = corners(&|x, y| x * y);
                fit(lo, hi, wide)
            }
            0x6c | 0x6d => {
                if bl <= 0 && bh >= 0 {
                    // 除数可能为零：构建期不可判定是否抛出
                    return defer("延迟值参与求值：宿主标量（污点）作除数且区间含零");
                }
                let (lo, hi) = corners(&|x, y| x / y);
                fit(lo, hi, wide)
            }
            0x70 | 0x71 => {
                if bl <= 0 && bh >= 0 {
                    return defer("延迟值参与求值：宿主标量（污点）作除数且区间含零");
                }
                let m = bl.abs().max(bh.abs()) - 1;
                let lo = if al >= 0 { 0 } else { -m.min(-al) };
                let hi = if ah <= 0 { 0 } else { m.min(ah) };
                fit(lo, hi, wide)
            }
            0x78..=0x7d => {
                let bits = if wide { 63 } else { 31 };
                match (rb.0 == rb.1, op) {
                    (true, 0x7a | 0x7b) => {
                        let s = (rb.0 & bits) as u32;
                        fit(al >> s, ah >> s, wide)
                    }
                    (true, 0x7c | 0x7d) if al >= 0 => {
                        let s = (rb.0 & bits) as u32;
                        fit(al >> s, ah >> s, wide)
                    }
                    (true, _) if al >= 0 => {
                        let s = (rb.0 & bits) as u32;
                        fit(al << s, ah << s, wide)
                    }
                    _ => full(wide),
                }
            }
            0x7e | 0x7f if al >= 0 || bl >= 0 => {
                let m = if al >= 0 && bl >= 0 { ah.min(bh) } else if al >= 0 { ah } else { bh };
                (0, m as i64)
            }
            0x80..=0x83 if al >= 0 && bl >= 0 => {
                let m = ah.max(bh);
                let p = if m == 0 { 0 } else { (1i128 << (128 - m.leading_zeros())) - 1 };
                fit(0, p, wide)
            }
            0x60..=0x83 => full(wide),
            _ => return fail(format!("污点算术 {op:#x}")),
        };
        let wide_out = wide && op != 0x94;
        Ok(self.tmk(TOp::Bin(op, a, b), wide_out, r))
    }

    /// 一元（取负 0x74 / 0x75、整数类型转换 0x85 / 0x88 / 0x91–0x93）：操作数为污点
    pub(super) fn tun(&mut self, op: u8, v: CV) -> R<CV> {
        let Some((lo, hi)) = self.trange(v) else { return fail("污点一元操作数") };
        let (wide, r) = match op {
            0x74 => (false, fit(-i128::from(hi), -i128::from(lo), false)),
            0x75 => (true, fit(-i128::from(hi), -i128::from(lo), true)),
            0x85 => (true, (lo, hi)),
            0x88 => (false, fit(i128::from(lo), i128::from(hi), false)),
            0x91 => (false, if lo >= -128 && hi <= 127 { (lo, hi) } else { (-128, 127) }),
            0x92 => (false, if lo >= 0 && hi <= 0xffff { (lo, hi) } else { (0, 0xffff) }),
            0x93 => (false, if lo >= -32768 && hi <= 32767 { (lo, hi) } else { (-32768, 32767) }),
            // 浮点转换：构建期取具体值
            _ => return defer("延迟值参与求值：宿主标量（污点）转浮点"),
        };
        if (lo, hi) == r && matches!(op, 0x91..=0x93) {
            return Ok(v);
        }
        Ok(self.tmk(TOp::Un(op, v), wide, r))
    }

    /// 污点上的比较分支的判定（k：0..6 = eq ne lt ge gt le）；无法判定为 None
    pub(super) fn tdecide(&self, k: u8, a: CV, b: CV) -> Option<bool> {
        decide(k, self.trange(a)?, self.trange(b)?)
    }

    /// 分支 `a k b` 在 `holds` 一侧对污点操作数的细化：Some(None) = 无细化，None = 该侧不可能
    pub(super) fn tnarrow(&self, k: u8, a: CV, b: CV, holds: bool) -> Option<Option<(u32, i64, i64)>> {
        let k = if holds { k } else { negate(k) };
        let (t, k, c) = match (a, b) {
            (CV::T(id, _), c) => (id, k, c),
            (c, CV::T(id, _)) => (id, mirror(k), c),
            _ => return Some(None),
        };
        let Some((cl, ch)) = self.trange(c) else { return Some(None) };
        if cl != ch {
            return Some(None);
        }
        let r = self.trange(CV::T(t, 0))?;
        narrow(k, r, cl).map(|(lo, hi)| Some((t, lo, hi)))
    }

    /// 两路合并：同值保留，不同的整数值合并为选择表达式（`cmp(a, b) ? t : f`）
    pub(super) fn tselect(&mut self, k: u8, a: CV, b: CV, t: CV, f: CV) -> R<CV> {
        if t == f {
            return Ok(t);
        }
        let (Some(rt), Some(rf)) = (self.trange(t), self.trange(f)) else {
            return defer("延迟值参与求值：宿主标量分支两侧的引用 / 浮点值不同");
        };
        if t.wide() != f.wide() {
            return fail("分支合并点两侧类型不符");
        }
        self.bj.taint.selects += 1;
        Ok(self.tmk(TOp::Sel { cmp: k, a, b, t, f }, t.wide(), (rt.0.min(rf.0), rt.1.max(rf.1))))
    }

    /// 表达式文本（规范形态：摘要与报告共用；不含表达式表下标）
    pub(super) fn tfmt(&self, v: CV, out: &mut String) {
        let e = match v {
            CV::T(id, _) => &self.bj.taint.exprs[id as usize],
            CV::I(x) => {
                let _ = write!(out, "{x}");
                return;
            }
            CV::J(x) => {
                let _ = write!(out, "{x}L");
                return;
            }
            CV::R(o) => {
                let _ = write!(out, "<{}>", self.heap[o as usize].ty);
                return;
            }
            v => {
                let _ = write!(out, "{v:?}");
                return;
            }
        };
        match &e.op {
            TOp::Src { native, args } => {
                let _ = write!(out, "{native}(");
                for (i, &a) in args.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    self.tfmt(a, out);
                }
                out.push(')');
            }
            TOp::Un(op, x) => {
                let _ = write!(out, "{}(", match op {
                    0x74 | 0x75 => "neg",
                    0x85 => "i2l",
                    0x88 => "l2i",
                    0x91 => "i2b",
                    0x92 => "i2c",
                    _ => "i2s",
                });
                self.tfmt(*x, out);
                out.push(')');
            }
            TOp::Bin(op, a, b) => {
                let s = match op {
                    0x94 => "cmp",
                    0x60 | 0x61 => "+",
                    0x64 | 0x65 => "-",
                    0x68 | 0x69 => "*",
                    0x6c | 0x6d => "/",
                    0x70 | 0x71 => "%",
                    0x78 | 0x79 => "<<",
                    0x7a | 0x7b => ">>",
                    0x7c | 0x7d => ">>>",
                    0x7e | 0x7f => "&",
                    0x80 | 0x81 => "|",
                    _ => "^",
                };
                out.push('(');
                self.tfmt(*a, out);
                let _ = write!(out, " {s} ");
                self.tfmt(*b, out);
                out.push(')');
            }
            TOp::Sel { cmp, a, b, t, f } => {
                out.push('(');
                self.tfmt(*a, out);
                let _ = write!(out, " {} ", CMP[*cmp as usize]);
                self.tfmt(*b, out);
                out.push_str(" ? ");
                self.tfmt(*t, out);
                out.push_str(" : ");
                self.tfmt(*f, out);
                out.push(')');
            }
        }
    }

    pub(super) fn tstr(&self, v: CV) -> String {
        let mut s = String::new();
        self.tfmt(v, &mut s);
        if let Some((lo, hi)) = self.trange(v).filter(|_| matches!(v, CV::T(..))) {
            let _ = write!(s, " ∈ [{lo}, {hi}]");
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decide_and_narrow() {
        // NCPU ∈ [1, max]：NCPU > 1 不可判定；≤ 1 一侧细化为单点 1
        let n = (1, i64::from(i32::MAX));
        assert_eq!(decide(4, n, (1, 1)), None);
        assert_eq!(decide(3, n, (1, 1)), Some(true));
        assert_eq!(narrow(5, n, 1), Some((1, 1)));
        assert_eq!(narrow(4, n, 1), Some((2, i64::from(i32::MAX))));
        // 2 / NCPU ∈ [0, 2] < 16 恒真
        assert_eq!(decide(2, (0, 2), (16, 16)), Some(true));
        assert_eq!(narrow(2, (16, 16), 16), None);
        assert_eq!(fit(0, i128::from(i32::MAX) + 1, false), full(false));
    }
}
