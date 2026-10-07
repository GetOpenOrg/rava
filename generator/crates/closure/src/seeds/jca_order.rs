//! JCA 提供者序求值（seeds.toml `[jca.order]`，计划 docs/plans/2026-10-06-c1d-jca-provider-order.md）。
//!
//! 运行期提供者表 = 嵌入的 `java.security` 生效属性里 `security.provider.N` 的有序名单。表上逐项装载 provider：
//! 内建名字（清单 `builtin`，按 JDK 特性版本）由字节码直接 `new`，其余经装载器（清单 `loader`，ServiceLoader 遍历
//! 全部 Provider 实现）。按序游走在命中处停止：游走深度小于首个非内建表项序号时装载器不会执行。
//!
//! 本模块只做与引擎无关的纯计算：清单解析、提供者表解析、内建 provider「必定注册」的服务（只取无条件执行的
//! 注册调用，作下近似）与按序游走的深度。引擎侧闸门与放行判定见 `engine/jca_order.rs`。

use std::collections::{BTreeMap, BTreeSet};

use classfile::{Const, ExceptionEntry, Insn, Operand};
use resolve::ClassPath;

const INVOKESPECIAL: u8 = 0xb7;
const PUTSTATIC: u8 = 0xb3;
const ATHROW: u8 = 0xbf;
const IRETURN: u8 = 0xac;
const RETURN: u8 = 0xb1;

/// 外部入口的游走形态（实参序号含接收者）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Walk {
    /// 按序取首个提供 (类型, 算法) 的 provider；`failover` = 首个服务构造失败时改走全表服务列表
    First { ty: usize, algo: usize, failover: bool },
    /// 按 provider 名定位（逐项装载直到名字相等）：名字取实参
    Name(usize),
    /// 按 provider 名定位：名字为常量
    Literal(String),
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub member: String,
    pub walk: Walk,
}

/// 视图类：其对象只经 `producers` 返回（方法属内部类时调用方取的是生产者的调用方）
#[derive(Debug, Clone)]
pub struct View {
    pub class: String,
    pub producers: Vec<String>,
}

/// 改写入口：可达即改写提供者表；`key` = 属性键实参序号（含接收者），键命中表键前缀 / 偏好键才算改写
#[derive(Debug, Clone)]
pub struct Mutator {
    pub member: String,
    pub key: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct OrderCfg {
    /// 生效属性文件（相对 runtime 目录；`{jdk}` 换成参考 JDK 特性版本）
    pub table: String,
    pub table_key: String,
    pub preferred_key: String,
    /// 指定附加属性文件的系统属性（运行期可改写时提供者表不定）
    pub override_prop: String,
    /// JDK 特性版本 → 内建 provider 名（ProviderConfig 按名直接构造）
    pub builtin: BTreeMap<u32, Vec<String>>,
    /// 装载器调用点的被调成员（`类.名字:描述符`）
    pub loader: String,
    /// 内部类前缀：游走在其内部的方法沿调用方上溯
    pub interior: Vec<String>,
    /// JDK 特性版本 → 视图类
    pub views: BTreeMap<u32, Vec<View>>,
    pub entries: Vec<Entry>,
    pub mutators: Vec<Mutator>,
    /// 首个服务构造不会失败的服务类型（`First` 的失败转移不发生）
    pub total_types: Vec<String>,
    /// (provider 名, 注册体成员)：provider 构造时必定执行的注册方法
    pub registrations: Vec<(String, String)>,
    /// 注册调用：只登记标准名 / 另登记同义名表里的同义名
    pub plain: Vec<String>,
    pub aliased: Vec<String>,
}

impl OrderCfg {
    pub fn from_toml(sec: Option<&toml::Value>) -> Option<Self> {
        let sec = sec?;
        let s = |v: &toml::Value, k: &str| v.get(k).and_then(|x| x.as_str()).unwrap_or_default().to_string();
        let strs = |v: Option<&toml::Value>| -> Vec<String> { v.and_then(|x| x.as_array()).into_iter().flatten().filter_map(|x| x.as_str().map(String::from)).collect() };
        let arr = |k: &str| sec.get(k).and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let idx = |v: &toml::Value, k: &str| v.get(k).and_then(|x| x.as_integer()).map(|i| i as usize);
        let by_jdk = |k: &str| -> BTreeMap<u32, toml::Value> {
            sec.get(k).and_then(|v| v.as_table()).into_iter().flatten().filter_map(|(j, v)| Some((j.parse().ok()?, v.clone()))).collect()
        };
        let entries = arr("entries")
            .iter()
            .filter_map(|e| {
                let walk = match e.get("walk").and_then(|x| x.as_str())? {
                    "first" => Walk::First { ty: idx(e, "type")?, algo: idx(e, "algorithm")?, failover: e.get("failover").and_then(|x| x.as_bool()).unwrap_or(true) },
                    "name" => match e.get("literal").and_then(|x| x.as_str()) {
                        Some(l) => Walk::Literal(l.to_string()),
                        None => Walk::Name(idx(e, "name")?),
                    },
                    _ => return None,
                };
                Some(Entry { member: s(e, "member"), walk })
            })
            .collect();
        Some(OrderCfg {
            table: s(sec, "table"),
            table_key: s(sec, "table_key"),
            preferred_key: s(sec, "preferred_key"),
            override_prop: s(sec, "override_property"),
            builtin: by_jdk("builtin").into_iter().map(|(j, v)| (j, strs(Some(&v)))).collect(),
            loader: s(sec, "loader"),
            interior: strs(sec.get("interior")),
            views: by_jdk("views")
                .into_iter()
                .map(|(j, v)| {
                    let vs = v.as_array().into_iter().flatten().map(|x| View { class: s(x, "class"), producers: strs(x.get("producers")) }).collect();
                    (j, vs)
                })
                .collect(),
            entries,
            mutators: arr("mutators").iter().map(|m| Mutator { member: s(m, "member"), key: idx(m, "key") }).collect(),
            total_types: strs(sec.get("total_types")),
            registrations: arr("registrations").iter().map(|r| (s(r, "provider"), s(r, "body"))).collect(),
            plain: strs(sec.get("plain")),
            aliased: strs(sec.get("aliased")),
        })
    }
}

/// 属性文本里 `<key>N=值` 按 N 升序的值（值取首个空白前的部分：provider 名或类名）
pub fn parse_table(text: &str, key: &str) -> Vec<String> {
    let mut rows: Vec<(u32, String)> = text
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            let (k, v) = l.split_once('=')?;
            let n: u32 = k.trim().strip_prefix(key)?.parse().ok()?;
            let v = v.split_whitespace().next()?.to_string();
            Some((n, v))
        })
        .collect();
    rows.sort();
    rows.into_iter().map(|(_, v)| v).collect()
}

/// 属性文本中是否出现键（`key=` 形态，忽略注释行）
pub fn has_key(text: &str, key: &str) -> bool {
    text.lines().map(str::trim).filter(|l| !l.starts_with('#')).any(|l| l.split_once('=').is_some_and(|(k, _)| k.trim() == key))
}

/// 首个非内建表项的序号（全是内建时为表长）
pub fn first_loaded(table: &[String], builtin: &[String]) -> usize {
    table.iter().position(|n| !builtin.iter().any(|b| b == n)).unwrap_or(table.len())
}

/// 名字在表中的序号
pub fn index_of(table: &[String], name: &str) -> Option<usize> {
    table.iter().position(|n| n == name)
}

/// 按序取首个必定提供 (类型, 算法) 的 provider 的序号；游走经过非内建表项（序号 ≥ bound）或走完全表时为 None。
/// `sure`：provider 名 → 必定注册的 (类型大写, 名字大写)
pub fn first_depth(table: &[String], bound: usize, sure: &BTreeMap<String, BTreeSet<(String, String)>>, ty: &str, algo: &str) -> Option<usize> {
    let key = (ty.to_uppercase(), algo.to_uppercase());
    for (i, p) in table.iter().enumerate() {
        if i >= bound {
            return None;
        }
        if sure.get(p).is_some_and(|s| s.contains(&key)) {
            return Some(i);
        }
    }
    None
}

fn str_lit(i: &Insn) -> Option<&str> {
    match &i.operand {
        Operand::Ldc(Const::String(s)) => Some(s),
        _ => None,
    }
}

/// 偏移 `off` 的指令在方法正常完成的每条路径上都执行：其前没有返回 / 抛出，没有越过它的前向跳转，
/// 没有从它之前跳到它之后的异常处理入口
pub fn unconditional(insns: &[Insn], exc: &[ExceptionEntry], off: u32) -> bool {
    for i in insns.iter().take_while(|i| i.offset < off) {
        if (IRETURN..=RETURN).contains(&i.opcode) || i.opcode == ATHROW {
            return false;
        }
        let skips = |t: &u32| *t > off;
        let over = match &i.operand {
            Operand::Branch(t) => skips(t),
            Operand::TableSwitch { default, targets, .. } => skips(default) || targets.iter().any(skips),
            Operand::LookupSwitch { default, pairs } => skips(default) || pairs.iter().any(|(_, t)| skips(t)),
            _ => false,
        };
        if over {
            return false;
        }
    }
    !exc.iter().any(|e| e.start < off && e.handler > off)
}

/// 同义名源类初始化器中的同义名组：每个构造调用前累积的字符串常量 = [枚举常量名, OID, 标准名, 同义名..]
/// （只有两串时枚举常量名即标准名）。返回 标准名大写 → 该名登记时附带的全部名字（大写，含 OID 与同义名）；
/// 同一标准名出现在多组时不取（登记取哪一组不定）
pub fn alias_registry(insns_of_clinits: &[&[Insn]]) -> BTreeMap<String, BTreeSet<String>> {
    let mut seen: BTreeMap<String, Option<BTreeSet<String>>> = BTreeMap::new();
    for insns in insns_of_clinits {
        let mut group: Vec<String> = Vec::new();
        for i in insns.iter() {
            if let Some(s) = str_lit(i).filter(|s| !s.is_empty()) {
                group.push(s.to_uppercase());
            } else if i.opcode == INVOKESPECIAL && matches!(&i.operand, Operand::Method(r, _) if r.name == "<init>") {
                let g = std::mem::take(&mut group);
                if g.len() < 2 {
                    continue;
                }
                let (std, names): (String, BTreeSet<String>) = if g.len() == 2 { (g[0].clone(), g.iter().cloned().collect()) } else { (g[2].clone(), g[1..].iter().cloned().collect()) };
                seen.entry(std).and_modify(|v| *v = None).or_insert(Some(names));
            } else if i.opcode == PUTSTATIC {
                group.clear();
            }
        }
    }
    seen.into_iter().filter_map(|(k, v)| Some((k, v?))).collect()
}

/// 注册体里必定执行的注册：`ldc 类型; ldc 算法; ldc 实现类名` 之后首个调用是登记的注册调用且无条件执行。
/// 返回 (类型大写, 名字大写)：同义名注册另含该标准名的同义名组
pub fn sure_registrations(
    insns: &[Insn],
    exc: &[ExceptionEntry],
    plain: &[String],
    aliased: &[String],
    aliases: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeSet<(String, String)> {
    let mut out = BTreeSet::new();
    for (k, w) in insns.windows(3).enumerate() {
        let (Some(t), Some(a), Some(c)) = (str_lit(&w[0]), str_lit(&w[1]), str_lit(&w[2])) else { continue };
        if t.is_empty() || a.is_empty() || !c.contains('.') || c.contains(' ') {
            continue;
        }
        let Some(call) = insns[k + 3..].iter().find(|i| matches!(i.operand, Operand::Method(..))) else { continue };
        let Operand::Method(r, _) = &call.operand else { continue };
        let key = format!("{}.{}:{}", r.owner, r.name, r.desc);
        let with_alias = aliased.contains(&key);
        if !with_alias && !plain.contains(&key) {
            continue;
        }
        if !unconditional(insns, exc, call.offset) {
            continue;
        }
        let (tu, au) = (t.to_uppercase(), a.to_uppercase());
        if with_alias {
            for n in aliases.get(&au).into_iter().flatten() {
                out.insert((tu.clone(), n.clone()));
            }
        }
        out.insert((tu, au));
    }
    out
}

/// 内建 provider 必定注册的服务：provider 名 → (类型大写, 名字大写)
pub fn sure_services(cfg: &OrderCfg, alias_sources: &[String], cp: &ClassPath) -> BTreeMap<String, BTreeSet<(String, String)>> {
    let clinits: Vec<Vec<Insn>> = alias_sources
        .iter()
        .filter_map(|c| cp.get(c))
        .flat_map(|cf| cf.methods.iter().filter(|m| m.is_clinit()).filter_map(|m| m.code.as_ref().map(|c| c.insns.clone())).collect::<Vec<_>>())
        .collect();
    let refs: Vec<&[Insn]> = clinits.iter().map(|v| v.as_slice()).collect();
    let aliases = alias_registry(&refs);
    let mut out: BTreeMap<String, BTreeSet<(String, String)>> = BTreeMap::new();
    for (prov, body) in &cfg.registrations {
        let Some((head, desc)) = body.split_once(':') else { continue };
        let Some((owner, name)) = head.rsplit_once('.') else { continue };
        let Some(cf) = cp.get(owner) else { continue };
        let Some(code) = cf.method(name, desc).and_then(|m| m.code.as_ref()) else { continue };
        out.entry(prov.clone()).or_default().extend(sure_registrations(&code.insns, &code.exception_table, &cfg.plain, &cfg.aliased, &aliases));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use classfile::MemberRef;

    fn ldc(off: u32, s: &str) -> Insn {
        Insn { offset: off, opcode: 0x12, operand: Operand::Ldc(Const::String(s.into())) }
    }
    fn call(off: u32, op: u8, owner: &str, name: &str, desc: &str) -> Insn {
        Insn { offset: off, opcode: op, operand: Operand::Method(MemberRef { owner: owner.into(), name: name.into(), desc: desc.into() }, false) }
    }
    fn branch(off: u32, to: u32) -> Insn {
        Insn { offset: off, opcode: 0x99, operand: Operand::Branch(to) }
    }

    #[test]
    fn table_is_ordered_by_index() {
        let t = "# c\nk.10=J\nk.2=B arg\nk.1=A\nother=1\nk.x=bad\n";
        assert_eq!(parse_table(t, "k."), ["A", "B", "J"]);
        assert!(has_key(t, "other"));
        assert!(!has_key("#other=1\n", "other"));
    }

    #[test]
    fn depth_stops_at_first_sure_provider_before_loaded_entry() {
        let table: Vec<String> = ["P0", "P1", "X2", "P3"].iter().map(|s| s.to_string()).collect();
        let builtin: Vec<String> = ["P0", "P1", "P3"].iter().map(|s| s.to_string()).collect();
        let k = first_loaded(&table, &builtin);
        assert_eq!(k, 2);
        let mut sure: BTreeMap<String, BTreeSet<(String, String)>> = BTreeMap::new();
        sure.entry("P1".into()).or_default().insert(("T".into(), "A".into()));
        sure.entry("P3".into()).or_default().insert(("T".into(), "B".into()));
        assert_eq!(first_depth(&table, k, &sure, "t", "a"), Some(1));
        // 只有表项 3 提供：游走经过非内建表项 2
        assert_eq!(first_depth(&table, k, &sure, "T", "B"), None);
        assert_eq!(first_depth(&table, k, &sure, "T", "C"), None);
        assert_eq!(index_of(&table, "X2"), Some(2));
    }

    #[test]
    fn unconditional_rejects_skipped_and_post_return_sites() {
        let insns = vec![branch(0, 10), ldc(3, "x"), call(5, 0xb6, "o/R", "add", "()V"), ldc(10, "y"), call(12, 0xb6, "o/R", "add", "()V"), Insn { offset: 15, opcode: RETURN, operand: Operand::None }, call(16, 0xb6, "o/R", "add", "()V")];
        assert!(!unconditional(&insns, &[], 5));
        assert!(unconditional(&insns, &[], 12));
        assert!(!unconditional(&insns, &[], 16));
        let exc = vec![ExceptionEntry { start: 0, end: 12, handler: 14, catch_type: None }];
        assert!(!unconditional(&insns, &exc, 12));
    }

    #[test]
    fn sure_registrations_need_declared_unconditional_call() {
        let add = "o/R.add:(Ljava/lang/String;)V";
        let addw = "o/R.addW:(Ljava/lang/String;)V";
        let insns = vec![
            ldc(0, "T"), ldc(2, "A-1"), ldc(4, "o.Impl"), call(6, 0xb6, "o/R", "addW", "(Ljava/lang/String;)V"),
            ldc(9, "T"), ldc(11, "B"), ldc(13, "o.Impl2"), call(15, 0xb6, "o/R", "other", "()V"),
            branch(18, 40),
            ldc(21, "T"), ldc(23, "C"), ldc(25, "o.Impl3"), call(27, 0xb6, "o/R", "add", "(Ljava/lang/String;)V"),
        ];
        let mut aliases = BTreeMap::new();
        aliases.insert("A-1".to_string(), ["1.2.3".to_string(), "A-1".to_string(), "A1".to_string()].into_iter().collect());
        let r = sure_registrations(&insns, &[], &[add.into()], &[addw.into()], &aliases);
        let names: Vec<String> = r.iter().map(|(_, n)| n.clone()).collect();
        // B 后的首个调用不是注册调用；C 被前向跳转越过
        assert_eq!(names, ["1.2.3", "A-1", "A1"]);
    }

    #[test]
    fn alias_registry_skips_enum_name_and_ambiguous_names() {
        let init = |off| call(off, INVOKESPECIAL, "o/K", "<init>", "()V");
        let put = |off| Insn { offset: off, opcode: PUTSTATIC, operand: Operand::Field(MemberRef { owner: "o/K".into(), name: "f".into(), desc: "Lo/K;".into() }) };
        let insns = vec![
            ldc(0, "E_1"), ldc(1, "1.1"), ldc(2, "Std"), ldc(3, "Al"), init(4), put(5),
            ldc(6, "Bare"), ldc(7, "2.2"), init(8), put(9),
            ldc(10, "E_3"), ldc(11, "3.3"), ldc(12, "Dup"), init(13), put(14),
            ldc(15, "E_4"), ldc(16, "4.4"), ldc(17, "Dup"), init(18), put(19),
        ];
        let r = alias_registry(&[&insns]);
        assert_eq!(r.get("STD").unwrap().iter().cloned().collect::<Vec<_>>(), ["1.1", "AL", "STD"]);
        assert_eq!(r.get("BARE").unwrap().iter().cloned().collect::<Vec<_>>(), ["2.2", "BARE"]);
        assert!(r.get("DUP").is_none());
        assert!(r.get("E_1").is_none());
    }
}
