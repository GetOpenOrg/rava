//! 层次查询：registry 继承链 / 接口链上的严格子类型判定、
//! 公共祖先、`_super` 路径、具体子类枚举。
//!
//! Python 以 Rust 短名串为实参（`_rust_type_to_binary` 反查短名索引）；这里以结构化
//! [`RsType`] / binary 名为实参，短名只在「类型形参名 / 注册表外名」的占位匹配时使用。

use ty::{JvmType, RsType};

use crate::ctx::InstrCtx;

/// 类型 → 注册表内的 binary 名（`_rust_type_to_binary`：按擦除头名反查；查不到 → None）
pub fn type_binary(ctx: &InstrCtx, t: &RsType) -> Option<String> {
    match t {
        RsType::Class { binary, .. } | RsType::Bare { binary } if ctx.reg().contains(binary) => Some(binary.clone()),
        RsType::Class { binary, .. } | RsType::Bare { binary } => {
            // 注册表外 binary：按短名反查（Python 短名索引口径）
            let g = ctx.ty.global_names();
            g.binary_of(&g.short(binary)).filter(|b| ctx.reg().contains(b)).map(str::to_string)
        }
        RsType::Param(n) => ctx.ty.binary_of(n).filter(|b| ctx.reg().contains(b)),
        _ => None,
    }
}

/// 短名 → 注册表内 binary（`_rust_type_to_binary` 的原形，供仍以短名定位的调用点）
pub fn short_binary(ctx: &InstrCtx, short: &str) -> Option<String> {
    ctx.ty.binary_of(short).filter(|b| ctx.reg().contains(b))
}

/// 类型的比较键：Rust 擦除头名（Python 以短名比较）
fn head(ctx: &InstrCtx, t: &RsType) -> String {
    t.head_name(&ctx.ty).unwrap_or_else(|| "()".to_string())
}

/// `child` 是否为 `parent` 的**严格**子类型（`_is_subtype`）：
/// - 同一类型 / 解析到同一 binary → false（不自反）；
/// - 根类 `java/lang/Object` 恒不作为成立目标；
/// - child 须能解析到注册表内 binary；parent 解析不到时以其头名占位匹配闭包。
pub fn is_subtype(ctx: &InstrCtx, child: &RsType, parent: &RsType) -> bool {
    if ctx.reg().is_empty() {
        return false;
    }
    let (ch, ph) = (head(ctx, child), head(ctx, parent));
    if ch == ph {
        return false;
    }
    let Some(child_bin) = type_binary(ctx, child) else {
        return false;
    };
    let parent_bin = type_binary(ctx, parent);
    if matches!(parent, RsType::Object)
        || parent_bin.as_deref() == Some(ty::consts::OBJECT)
        || (parent_bin.is_none() && ph == ctx.short(ty::consts::OBJECT))
    {
        return false;
    }
    let target_name = parent_bin.unwrap_or(ph);
    let child_t = JvmType::class_of(&child_bin, ctx.reg());
    let target_t = JvmType::class_of(&target_name, ctx.reg());
    if child_bin == target_name {
        return false;
    }
    child_t.is_subtype_of(&target_t, ctx.reg())
}

/// 擦除类型是否为注册表内接口（`_is_interface`；未知类型按非接口）
pub fn is_interface(ctx: &InstrCtx, t: &RsType) -> bool {
    type_binary(ctx, t).and_then(|b| ctx.reg().get(&b)).is_some_and(|ci| ci.is_interface())
}

/// `class_binary` 的全部具体子类（含传递），叶节点优先（`_get_all_subtypes_ordered`；
/// 兄弟节点按 binary 名排序）
pub fn all_subtypes_ordered(ctx: &InstrCtx, class_binary: &str) -> Vec<String> {
    let mut all = Vec::new();
    let mut queue = std::collections::VecDeque::from([class_binary.to_string()]);
    let mut visited = std::collections::BTreeSet::from([class_binary.to_string()]);
    while let Some(cur) = queue.pop_front() {
        for ci in ctx.reg().iter() {
            if visited.contains(ci.name()) {
                continue;
            }
            if ci.super_class() == cur || ci.interfaces().contains(&cur) {
                visited.insert(ci.name().to_string());
                all.push(ci.name().to_string());
                queue.push_back(ci.name().to_string());
            }
        }
    }
    all.reverse();
    all
}

/// 两个非泛型引用类型的公共类型（`_common_ref_type`）：一方是另一方子类型 → 取父类型；
/// 否则沿 `a` 的超类链找第一个同为 `b` 祖先的类。含泛型实参 / 找不到 → None
pub fn common_ref_type(ctx: &InstrCtx, a: &RsType, b: &RsType) -> Option<RsType> {
    if ctx.reg().is_empty() || !a.type_args().is_empty() || !b.type_args().is_empty() || head(ctx, a) == head(ctx, b) {
        return None;
    }
    if is_subtype(ctx, a, b) {
        return Some(erased_generic(ctx, b.clone()));
    }
    if is_subtype(ctx, b, a) {
        return Some(erased_generic(ctx, a.clone()));
    }
    let mut cur = type_binary(ctx, a)?;
    let mut seen = std::collections::BTreeSet::new();
    while seen.insert(cur.clone()) {
        let ci = ctx.reg().get(&cur)?;
        let sc = ci.super_class();
        if sc.is_empty() || sc == ty::consts::OBJECT {
            return None;
        }
        cur = sc.to_string();
        let sc_t = RsType::class(cur.clone(), Vec::new());
        if is_subtype(ctx, b, &sc_t) {
            return Some(erased_generic(ctx, sc_t));
        }
    }
    None
}

/// 公共祖先是泛型类时补擦除实例化（`Enum` → `Enum<Object>`，`_erased_generic`）：两个非泛型子类
/// 的合并点只能取擦除视图，宏生成的 `From<Child> for Ancestor<任意实参>` 保证上转成立
fn erased_generic(ctx: &InstrCtx, t: RsType) -> RsType {
    let RsType::Class { binary, args } = &t else { return t };
    if !args.is_empty() {
        return t;
    }
    let Some(ci) = ctx.reg().get(binary.as_str()) else { return t };
    let n = ctx.ty.effective_class_type_params(ci).len();
    if n == 0 {
        return t;
    }
    RsType::class(binary.to_string(), vec![RsType::Object; n])
}

/// 槽位 widening 的公共类祖先（`_common_ref_type_widening`）：基名走 [`common_ref_type`]；
/// 接口 / 根类 / 无公共祖先 → None；双方实参（渲染文本）一致时保留实参，否则 None
pub fn common_ref_type_widening(ctx: &InstrCtx, a: &RsType, b: &RsType) -> Option<RsType> {
    let common = common_ref_type(ctx, &sim::erase(a), &sim::erase(b))?;
    if matches!(common, RsType::Object) || is_interface(ctx, &common) {
        return None;
    }
    let (aa, ba) = (a.type_args(), b.type_args());
    if aa.is_empty() && ba.is_empty() {
        return Some(common);
    }
    let text = |xs: &[RsType]| ty::rs_type::render_arg_list(xs, &ctx.ty);
    if !aa.is_empty() && text(aa) == text(ba) {
        let RsType::Class { binary, .. } = common else {
            return None;
        };
        return Some(RsType::class(binary, aa.to_vec()));
    }
    None
}

/// invokespecial `super.m()` 的 `_super` 链段数（`_find_super_chain_to_class`）：
/// `target` 为 binary 名或 Rust 短名。目标即当前类 → 0；沿超类链找到 → 段数；
/// 注册信息不全 → 1（至少一级 `_super`）
pub fn super_chain_to_class(ctx: &InstrCtx, current: &str, target: &str) -> usize {
    if ctx.reg().is_empty() || current.is_empty() || target.is_empty() {
        return 1;
    }
    let same = |b: &str| b == target || ctx.short(b) == target;
    if same(current) {
        return 0;
    }
    let Some(ci) = ctx.reg().get(current) else {
        return 1;
    };
    let mut depth = 0;
    let mut sc = ci.super_class().to_string();
    while !sc.is_empty() && sc != ty::consts::OBJECT {
        depth += 1;
        if same(&sc) {
            return depth;
        }
        match ctx.reg().get(&sc) {
            Some(p) => sc = p.super_class().to_string(),
            None => break,
        }
    }
    1
}
