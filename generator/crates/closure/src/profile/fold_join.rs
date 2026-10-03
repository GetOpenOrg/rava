//! 折叠的并（join）：同一方法在到达它的各入口上的折叠记录 → 对每个入口都成立的折叠。
//!
//! 每个入口 X 的记录（folds v2）给出：死区 D_X = dead_pcs ∪ noreturn_dead_pcs，以及活指令上的折叠
//! （null_recv / noreturn_calls / consts）。到达但无记录的入口按空折叠（全部活、无折叠）参与。
//!
//! - 死区：D = ∩ D_X（区间集求交，端点仍是各输入的端点，即指令起点或代码长度）；
//! - 活指令 p 的折叠：在 p 活着的入口上逐点取格的并——NullRecv ⊔ NullRecv = NullRecv，
//!   NullRecv / NoReturn 两两之并 = NoReturn（都不正常返回；照常调用，之后截断），Const(v) ⊔ Const(v) = Const(v)，
//!   其余组合与未折叠（Normal）之并都是 Normal；
//! - 不进入的处理器：在每个入口里都列入 dead_handlers 或位于 D_X 的处理器；另外 D 内的全部处理器
//!   （各入口里它们都不会被进入：列入 dead_handlers，或随 noreturn_dead_pcs 连同表项删除）；
//! - 死 catch 表项：在每个入口里列入 dead_catches、或其处理器不进入、或其保护区间整段在 D_X 内；
//!   处理器已列入 dead_handlers 的不再重复列出。
//!
//! 输出把 D 全部写入 dead_pcs（noreturn_dead_pcs 为空），按 `input::norm` 的规则逐条成立：
//! 活指令 p 顺序落入 D 时，p 活着的每个入口 X 里 p 都顺序落入 D_X，故 p 在 X 中是跳转 / 无后继指令或
//! 不返回的调用点，并后 p 仍是跳转 / 无后继或 NullRecv / NoReturn；活跳转的目标在 D 内则在每个 D_X 内，
//! 与 X 自身的合法性矛盾；D 内的处理器全部列入 dead_handlers。

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{json, Value};

use super::HandlerTable;

type Ranges = Vec<(u32, u32)>;

/// 排序并合并相交 / 相邻区间
fn normalize(mut r: Ranges) -> Ranges {
    r.retain(|(a, b)| a < b);
    r.sort_unstable();
    let mut out: Ranges = Vec::with_capacity(r.len());
    for (a, b) in r {
        match out.last_mut() {
            Some(last) if a <= last.1 => last.1 = last.1.max(b),
            _ => out.push((a, b)),
        }
    }
    out
}

fn intersect(x: &[(u32, u32)], y: &[(u32, u32)]) -> Ranges {
    let (mut i, mut j, mut out) = (0, 0, Vec::new());
    while i < x.len() && j < y.len() {
        let (a, b) = (x[i].0.max(y[j].0), x[i].1.min(y[j].1));
        if a < b {
            out.push((a, b));
        }
        if x[i].1 < y[j].1 {
            i += 1;
        } else {
            j += 1;
        }
    }
    out
}

fn in_ranges(r: &[(u32, u32)], pc: u32) -> bool {
    r.iter().any(|&(a, b)| a <= pc && pc < b)
}

/// [s, e) 整段在规范化区间集内
fn covered(r: &[(u32, u32)], s: u32, e: u32) -> bool {
    r.iter().any(|&(a, b)| a <= s && e <= b)
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Catch {
    start: u32,
    end: u32,
    handler: u32,
    catch_type: String,
}

#[derive(Default)]
struct In {
    dead: Ranges,
    handlers: BTreeSet<u32>,
    catches: BTreeSet<Catch>,
    /// pc → (规范串, 常量条目)
    consts: BTreeMap<u32, (String, Value)>,
    null_recv: BTreeSet<u32>,
    noreturn: BTreeSet<u32>,
}

fn pcs(f: &Value, key: &str) -> Result<BTreeSet<u32>, String> {
    let mut out = BTreeSet::new();
    for x in f.get(key).and_then(Value::as_array).into_iter().flatten() {
        out.insert(x.as_u64().ok_or_else(|| format!("folds.{key} 项应为偏移：{x}"))? as u32);
    }
    Ok(out)
}

fn ranges(f: &Value, key: &str) -> Result<Ranges, String> {
    let mut out = Vec::new();
    for r in f.get(key).and_then(Value::as_array).into_iter().flatten() {
        let p = |i: usize| r.get(i).and_then(Value::as_u64).map(|x| x as u32);
        out.push(p(0).zip(p(1)).ok_or_else(|| format!("folds.{key} 项应为 [起, 止)：{r}"))?);
    }
    Ok(out)
}

fn parse(f: Option<&Value>) -> Result<In, String> {
    let Some(f) = f else { return Ok(In::default()) };
    let mut dead = ranges(f, "dead_pcs")?;
    dead.extend(ranges(f, "noreturn_dead_pcs")?);
    let mut catches = BTreeSet::new();
    for c in f.get("dead_catches").and_then(Value::as_array).into_iter().flatten() {
        let n = |k: &str| c.get(k).and_then(Value::as_u64).map(|x| x as u32).ok_or_else(|| format!("dead_catches 缺 {k}：{c}"));
        let ct = c.get("catch_type").and_then(Value::as_str).ok_or_else(|| format!("dead_catches 缺 catch_type：{c}"))?;
        catches.insert(Catch { start: n("start")?, end: n("end")?, handler: n("handler")?, catch_type: ct.into() });
    }
    let mut consts = BTreeMap::new();
    for c in f.get("consts").and_then(Value::as_array).into_iter().flatten() {
        let pc = c.get("pc").and_then(Value::as_u64).ok_or_else(|| format!("consts 缺 pc：{c}"))? as u32;
        let mut body = c.clone();
        if let Some(o) = body.as_object_mut() {
            o.remove("pc");
        }
        consts.insert(pc, (body.to_string(), c.clone()));
    }
    Ok(In {
        dead: normalize(dead),
        handlers: pcs(f, "dead_handlers")?,
        catches,
        consts,
        null_recv: pcs(f, "null_recv")?,
        noreturn: pcs(f, "noreturn_calls")?,
    })
}

/// 活指令上的折叠格
#[derive(Clone, PartialEq, Eq)]
enum St {
    NullRecv,
    NoReturn,
    Const(String),
    Normal,
}

fn status(x: &In, pc: u32) -> St {
    if x.null_recv.contains(&pc) {
        St::NullRecv
    } else if x.noreturn.contains(&pc) {
        St::NoReturn
    } else if let Some((k, _)) = x.consts.get(&pc) {
        St::Const(k.clone())
    } else {
        St::Normal
    }
}

fn lub(a: St, b: St) -> St {
    match (a, b) {
        (St::NullRecv, St::NullRecv) => St::NullRecv,
        (St::NullRecv | St::NoReturn, St::NullRecv | St::NoReturn) => St::NoReturn,
        (St::Const(x), St::Const(y)) if x == y => St::Const(x),
        _ => St::Normal,
    }
}

/// 一个方法在各到达入口上的折叠之并；并后无折叠内容时 None
pub(super) fn join(method: &str, inputs: &[Option<Value>], handlers: &HandlerTable) -> Result<Option<Value>, String> {
    let xs: Vec<In> = inputs.iter().map(|f| parse(f.as_ref())).collect::<Result<_, _>>().map_err(|e| format!("{method}：{e}"))?;
    let Some(first) = xs.first() else { return Ok(None) };
    let dead = xs[1..].iter().fold(first.dead.clone(), |d, x| intersect(&d, &x.dead));
    // 活指令的折叠：候选点为任一入口的折叠点
    let cand: BTreeSet<u32> = xs.iter().flat_map(|x| x.null_recv.iter().chain(&x.noreturn).chain(x.consts.keys()).copied()).collect();
    let (mut null_recv, mut noreturn, mut consts) = (Vec::new(), Vec::new(), Vec::new());
    for pc in cand {
        if in_ranges(&dead, pc) {
            continue;
        }
        let st = xs.iter().filter(|x| !in_ranges(&x.dead, pc)).map(|x| status(x, pc)).reduce(lub);
        match st {
            Some(St::NullRecv) => null_recv.push(pc),
            Some(St::NoReturn) => noreturn.push(pc),
            Some(St::Const(k)) => {
                let entry = xs.iter().find_map(|x| x.consts.get(&pc).filter(|(kk, _)| *kk == k)).map(|(_, v)| v.clone());
                consts.extend(entry);
            }
            Some(St::Normal) | None => {}
        }
    }
    // 不进入的处理器
    let dead_everywhere = |h: u32| xs.iter().all(|x| x.handlers.contains(&h) || in_ranges(&x.dead, h));
    let mut dead_handlers: BTreeSet<u32> = xs.iter().flat_map(|x| x.handlers.iter().copied()).filter(|&h| dead_everywhere(h)).collect();
    if !dead.is_empty() {
        let table = handlers(method).ok_or_else(|| format!("{method}：取不到字节码的异常表"))?;
        dead_handlers.extend(table.into_iter().filter(|&h| in_ranges(&dead, h)));
    }
    let catch_dead = |x: &In, c: &Catch| {
        x.catches.contains(c) || x.handlers.contains(&c.handler) || in_ranges(&x.dead, c.handler) || covered(&x.dead, c.start, c.end)
    };
    let catches: BTreeSet<&Catch> = xs
        .iter()
        .flat_map(|x| &x.catches)
        .filter(|c| !dead_handlers.contains(&c.handler) && xs.iter().all(|x| catch_dead(x, c)))
        .collect();
    if dead.is_empty() && dead_handlers.is_empty() && catches.is_empty() && consts.is_empty() && null_recv.is_empty() && noreturn.is_empty() {
        return Ok(None);
    }
    Ok(Some(json!({
        "method": method,
        "dead_pcs": dead.iter().map(|(a, b)| json!([a, b])).collect::<Vec<_>>(),
        "dead_handlers": dead_handlers,
        "dead_catches": catches.iter().map(|c| json!({"start": c.start, "end": c.end, "handler": c.handler, "catch_type": c.catch_type})).collect::<Vec<_>>(),
        "consts": consts,
        "null_recv": null_recv,
        "noreturn_calls": noreturn,
        "noreturn_dead_pcs": [],
    })))
}

#[cfg(test)]
pub(super) mod unit {
    use super::*;

    #[test]
    fn interval_ops() {
        assert_eq!(normalize(vec![(5, 8), (0, 2), (2, 4), (7, 9)]), vec![(0, 4), (5, 9)]);
        assert_eq!(intersect(&[(0, 4), (6, 10)], &[(2, 8)]), vec![(2, 4), (6, 8)]);
        assert!(covered(&[(0, 4)], 1, 4));
        assert!(!covered(&[(0, 4)], 1, 5));
    }

    #[test]
    fn lattice() {
        assert!(lub(St::NullRecv, St::NullRecv) == St::NullRecv);
        assert!(lub(St::NullRecv, St::NoReturn) == St::NoReturn);
        assert!(lub(St::Const("1".into()), St::Const("1".into())) == St::Const("1".into()));
        assert!(lub(St::Const("1".into()), St::Const("2".into())) == St::Normal);
        assert!(lub(St::Const("1".into()), St::NoReturn) == St::Normal);
        assert!(lub(St::NullRecv, St::Normal) == St::Normal);
    }
}
