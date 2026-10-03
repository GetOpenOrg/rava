//! 小整数集：int 族常量的有限并（≤ [`MAX`] 个，有序去重）。
//!
//! 只由跨方法的常量格合流产生（字段值集、形参 / 返回值常量格，见 `engine/facts.rs` 的 `PV::join`）；
//! 方法内合流仍是单一常量 → 奇偶 → Top，循环计数器不会逐值展开。条件跳转与 switch 按集合逐值判定：
//! 全部取值结论一致才定向。

use std::rc::Rc;

use super::V;

pub const MAX: usize = 16;

/// int 族常量值的全部取值
pub fn members(v: &V) -> Option<Vec<i32>> {
    match v {
        V::Int(x) => Some(vec![*x]),
        V::Ints(s) => Some(s.to_vec()),
        _ => None,
    }
}

/// 两个 int 族常量值的并；超出上限为 None
pub fn union(a: &V, b: &V) -> Option<V> {
    let mut x = members(a)?;
    x.extend(members(b)?);
    x.sort_unstable();
    x.dedup();
    match x.len() {
        1 => Some(V::Int(x[0])),
        n if n <= MAX => Some(V::Ints(Rc::from(x))),
        _ => None,
    }
}

/// 二元谓词在两侧全部取值组合上结论一致时的结果
pub fn decide(a: &V, b: &V, f: impl Fn(i32, i32) -> bool) -> Option<bool> {
    let (xs, ys) = (members(a)?, members(b)?);
    let mut r = None;
    for x in &xs {
        for y in &ys {
            let c = f(*x, *y);
            if r.is_some_and(|p| p != c) {
                return None;
            }
            r = Some(c);
        }
    }
    r
}

/// 二元算子在两侧全部取值组合上的结果集；任一组合无定义或结果超出上限为 None
pub fn map2(a: &V, b: &V, f: impl Fn(i32, i32) -> Option<i32>) -> Option<V> {
    let (xs, ys) = (members(a)?, members(b)?);
    if xs.len() * ys.len() > MAX * MAX {
        return None;
    }
    let mut out = Vec::with_capacity(xs.len() * ys.len());
    for x in &xs {
        for y in &ys {
            out.push(f(*x, *y)?);
        }
    }
    out.sort_unstable();
    out.dedup();
    match out.len() {
        1 => Some(V::Int(out[0])),
        n if n <= MAX => Some(V::Ints(Rc::from(out))),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map2_unions_pairs() {
        let s = V::Ints(Rc::from([0, 256].as_slice()));
        assert_eq!(map2(&s, &V::Int(2), |x, y| Some(x & y)), Some(V::Int(0)));
        assert_eq!(map2(&s, &V::Int(1), |x, y| Some(x + y)), Some(V::Ints(Rc::from([1, 257].as_slice()))));
        assert_eq!(map2(&s, &V::Int(0), |x, y| (y != 0).then(|| x / y)), None);
    }
}
