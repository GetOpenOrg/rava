//! sim golden 回放支撑：Python 类型文本 → [`RsType`]、按钩子日志回答查询的 [`ReplayEnv`]、
//! 记录 JSON → 配置 / 状态、以及两侧状态的规范化快照（逐行文本，便于逐项比对）。

use crate::ir_golden::convert::Conv;
use crate::ir_golden::parse::parse_expr;
use ir::{Expr, Raw, Renderer, ShortNames, Stmt};
use serde_json::Value;
use sim::{erased_base, exprs::is_trivial, type_text, Local, SimConfig, SimEnv, SimError, SimResult, SimState, SlotDecl, StackEntry};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use ty::{Prim, RsType};

pub type R<T> = Result<T, String>;

/// names.json：binary → 短名（及反查）
pub struct Names {
    pub short: HashMap<String, String>,
    by_short: HashMap<String, String>,
}

impl Names {
    pub fn load(v: &Value) -> R<Names> {
        let obj = v["short"].as_object().ok_or("names.json 缺 short")?;
        let mut short = HashMap::new();
        let mut by_short = HashMap::new();
        for (b, s) in obj {
            let s = s.as_str().ok_or("短名非字符串")?.to_string();
            by_short.entry(s.clone()).or_insert_with(|| b.clone());
            short.insert(b.clone(), s);
        }
        Ok(Names { short, by_short })
    }
}

impl ShortNames for Names {
    fn short_cls(&self, binary: &str) -> String {
        // 注册表外的名字按 Python `short_cls` 的缺省规则（末段、`$` → `_`）
        self.short.get(binary).cloned().unwrap_or_else(|| binary.rsplit('/').next().unwrap_or(binary).replace('$', "_"))
    }
}

fn split_top(s: &str) -> Vec<&str> {
    let (mut depth, mut start, mut out) = (0i32, 0usize, Vec::new());
    for (i, c) in s.char_indices() {
        match c {
            '<' => depth += 1,
            '>' => depth -= 1,
            ',' if depth == 0 => {
                out.push(s[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push(s[start..].trim());
    out
}

fn prim(s: &str) -> Option<Prim> {
    Some(match s {
        "i8" => Prim::I8,
        "i16" => Prim::I16,
        "i32" => Prim::I32,
        "i64" => Prim::I64,
        "f32" => Prim::F32,
        "f64" => Prim::F64,
        "bool" => Prim::Bool,
        "u16" => Prim::U16,
        _ => return None,
    })
}

impl Names {
    /// Python 类型文本 → RsType（注册表外无实参的头名 = 类型形参）
    fn parse_ty(&self, s: &str) -> R<RsType> {
        let s = s.trim();
        if s == "()" {
            return Ok(RsType::Unit);
        }
        if let Some(p) = prim(s) {
            return Ok(RsType::Prim(p));
        }
        if s == ir::anchors::OBJECT {
            return Ok(RsType::Object);
        }
        let (head, args) = match s.find('<') {
            Some(i) if s.ends_with('>') => (&s[..i], split_top(&s[i + 1..s.len() - 1])),
            _ => (s, Vec::new()),
        };
        let args = args.into_iter().map(|a| self.parse_ty(a)).collect::<R<Vec<_>>>()?;
        if head == ir::anchors::ARRAY && args.len() == 1 {
            return Ok(RsType::array(args.into_iter().next().unwrap_or(RsType::Object)));
        }
        match self.by_short.get(head) {
            Some(b) => Ok(RsType::class(b.clone(), args)),
            None if args.is_empty() => Ok(RsType::Param(head.to_string())),
            None => Err(format!("未知类型头 {head}（{s}）")),
        }
    }
}

/// 按钩子日志回答查询的环境；未命中记入 `misses`
pub struct ReplayEnv<'n> {
    pub names: &'n Names,
    hooks: Vec<(String, Vec<Value>, Value)>,
    pub misses: RefCell<Vec<String>>,
}

impl<'n> ReplayEnv<'n> {
    pub fn new(names: &'n Names, hooks: &Value) -> ReplayEnv<'n> {
        let hooks = hooks
            .as_array()
            .map(|a| {
                a.iter()
                    .map(|h| {
                        let name = h[0].as_str().unwrap_or("").to_string();
                        (name, h[1].as_array().cloned().unwrap_or_default(), h[2].clone())
                    })
                    .collect()
            })
            .unwrap_or_default();
        ReplayEnv { names, hooks, misses: RefCell::new(Vec::new()) }
    }

    fn lookup(&self, name: &str, args: &[&str]) -> Option<Value> {
        let hit = self.hooks.iter().find(|(n, a, _)| {
            n == name && a.len() == args.len() && a.iter().zip(args).all(|(x, y)| x.as_str() == Some(*y))
        });
        if hit.is_none() {
            self.misses.borrow_mut().push(format!("{name}{args:?}"));
        }
        hit.map(|h| h.2.clone())
    }

    pub fn ty(&self, s: &str) -> R<RsType> {
        let t = self.names.parse_ty(s)?;
        let back = type_text(&t, self);
        if back != s {
            return Err(format!("类型往返不一致：{s} → {back}"));
        }
        Ok(t)
    }
}

impl SimEnv for ReplayEnv<'_> {
    fn short_name(&self, binary: &str) -> String {
        self.names.short_cls(binary)
    }

    fn is_subtype(&self, sub: &RsType, sup: &RsType) -> bool {
        let (a, b) = (erased_base(sub, self), erased_base(sup, self));
        self.lookup("is_subtype", &[&a, &b]).and_then(|v| v.as_bool()).unwrap_or(false)
    }

    fn is_interface(&self, ty: &RsType) -> bool {
        let a = erased_base(ty, self);
        self.lookup("is_interface", &[&a]).and_then(|v| v.as_bool()).unwrap_or(false)
    }

    fn carrier_type(&self, ty: &RsType) -> Option<RsType> {
        let a = type_text(ty, self);
        let v = self.lookup("carrier", &[&a])?;
        v.as_str().and_then(|s| self.ty(s).ok())
    }

    fn box_object(&self, value: Expr, ty: &RsType) -> SimResult<Expr> {
        let src = Renderer::new(self.names).expr(&value);
        let t = type_text(ty, self);
        let out = self.lookup("box_object", &[&src, &t]).ok_or_else(|| SimError::Env(format!("box_object 未命中 {src}")))?;
        let text = out.as_str().ok_or_else(|| SimError::Env("box_object 返回非字符串".into()))?;
        let rd = Renderer::new(self.names);
        Ok(parse_expr(text).filter(|e| rd.expr(e) == text).unwrap_or_else(|| Expr::Raw(Raw(text.to_string()))))
    }

    fn infer_type_args(&self, ty: &RsType, declared_sig: &str) -> Option<Vec<RsType>> {
        let head = erased_base(ty, self);
        let v = self.lookup("infer_type_args", &[&head, declared_sig])?;
        v.as_array()?.iter().map(|x| x.as_str().and_then(|s| self.ty(s).ok())).collect()
    }
}

fn u(v: &Value) -> R<u64> {
    v.as_u64().ok_or_else(|| format!("非整数 {v}"))
}

fn slot_map<T>(v: &Value, mut f: impl FnMut(&Value) -> R<T>) -> R<BTreeMap<u16, T>> {
    let mut out = BTreeMap::new();
    if let Some(o) = v.as_object() {
        for (k, x) in o {
            out.insert(k.parse::<u16>().map_err(|e| e.to_string())?, f(x)?);
        }
    }
    Ok(out)
}

/// ty_json（`{j, text}` 或 null）→ RsType
pub fn ty_of(env: &ReplayEnv, v: &Value) -> R<Option<RsType>> {
    match v.get("text").and_then(Value::as_str) {
        Some(s) => env.ty(s).map(Some),
        None => Ok(None),
    }
}

pub fn expr_of(conv: &mut Conv, v: &Value) -> R<Expr> {
    conv.expr(&v["j"])
}

pub fn config(env: &ReplayEnv, cfg: &Value, params: &[RsType]) -> R<SimConfig> {
    let slot_decls = slot_map(&cfg["slot_decls"], |ds| {
        ds.as_array()
            .ok_or("slot_decls 非列表")?
            .iter()
            .map(|d| {
                Ok(SlotDecl {
                    start: u(&d[0])? as u32,
                    end: u(&d[1])? as u32,
                    name: d[2].as_str().unwrap_or("").to_string(),
                    ty: ty_of(env, &d[3])?,
                    from_sig: d[4].as_bool().unwrap_or(false),
                    raw_sig: d[5].as_str().unwrap_or("").to_string(),
                    desc: d[6].as_str().unwrap_or("").to_string(),
                })
            })
            .collect::<R<Vec<_>>>()
    })?;
    let local_names = slot_map(&cfg["loc_names"], |n| Ok(n.as_str().unwrap_or("").to_string()))?;
    let ret = cfg["return_type"].as_str().unwrap_or("Object");
    Ok(SimConfig {
        params: params.to_vec(),
        is_static: cfg["is_static"].as_bool().unwrap_or(false),
        class_name: cfg["class_name"].as_str().unwrap_or("").to_string(),
        class_type_params: cfg["class_type_params"]
            .as_array()
            .map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect())
            .unwrap_or_default(),
        local_names,
        slot_decls,
        return_type: env.ty(ret).unwrap_or(RsType::Object),
        is_constructor: cfg["is_constructor"].as_bool().unwrap_or(false),
        in_vtable_body: cfg["in_vtable_body"].as_bool().unwrap_or(false),
    })
}

pub fn entry(env: &ReplayEnv, conv: &mut Conv, e: &Value, t: &Value, id: &Value) -> R<StackEntry> {
    Ok(StackEntry {
        expr: expr_of(conv, e)?,
        ty: ty_of(env, t)?.ok_or("栈条目缺类型")?,
        id: id.as_u64().unwrap_or(u64::from(u32::MAX)) as u32,
    })
}

/// pre 状态 JSON → SimState（语句缓冲从空开始，只比对新增语句）
pub fn state(env: &ReplayEnv, conv: &mut Conv, cfg: &Value, pre: &Value) -> R<SimState> {
    let mut st = SimState::default();
    for e in pre["stack"].as_array().ok_or("pre 缺 stack")? {
        st.stack.push(entry(env, conv, &e[0], &e[1], &e[2])?);
    }
    st.locals = slot_map(&pre["locals"], |l| {
        Ok(Local {
            name: ir::Ident::new(l[0].as_str().unwrap_or("")).map_err(|e| e.to_string())?,
            ty: ty_of(env, &l[1])?.ok_or("局部变量缺类型")?,
            is_new: l[2].as_bool().unwrap_or(false),
        })
    })?;
    st.counter = u(&pre["ctr"])? as u32;
    st.synth_slot_kinds = slot_map(&pre["synth"], |v| {
        Ok(v.as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default())
    })?;
    st.current_offset = u(&pre["cur"])? as u32;
    st.next_offset = u(&pre["next"])? as u32;
    st.depth = u(&pre["depth"])? as u32;
    st.slot_decl_depth = slot_map(&pre["decl_depth"], |v| Ok(u(v)? as u32))?;
    st.slot_bind_pos = slot_map(&pre["bind_pos"], |v| Ok(u(v)? as u32))?;
    st.underflow = pre["underflow"].as_bool().unwrap_or(false);
    if let Some(o) = pre["bounds"].as_object() {
        for (k, v) in o {
            st.type_var_bounds.insert(k.clone(), env.ty(v.as_str().unwrap_or(""))?);
        }
    }
    st.param_slots = cfg["param_slots"].as_array().map(|a| a.iter().filter_map(|x| x.as_u64().map(|x| x as u16)).collect()).unwrap_or_default();
    st.next_id = st.stack.iter().map(|e| e.id + 1).max().unwrap_or(0).max(1 << 20);
    Ok(st)
}

/// 栈条目身份按首次出现序规范化；平凡条目（Python 按新对象重建）不参与
fn groups(trivial: &[bool], ids: &[Option<u64>]) -> Vec<String> {
    let mut seen: BTreeMap<u64, usize> = BTreeMap::new();
    ids.iter()
        .zip(trivial)
        .map(|(id, t)| match (id, t) {
            (Some(id), false) => {
                let n = seen.len();
                format!("#{}", seen.entry(*id).or_insert(n))
            }
            _ => "#-".to_string(),
        })
        .collect()
}

/// 状态快照（逐行）：Python 侧由 post JSON 生成，Rust 侧由 SimState 生成，格式一致
pub fn snapshot_py(post: &Value, trivial: &[bool]) -> Vec<String> {
    let stack = post["stack"].as_array().cloned().unwrap_or_default();
    let ids: Vec<Option<u64>> = stack.iter().map(|e| e[2].as_u64()).collect();
    let g = groups(trivial, &ids);
    let mut out: Vec<String> = stack
        .iter()
        .zip(g)
        .map(|(e, g)| format!("stack {} : {} {g}", e[0]["text"].as_str().unwrap_or(""), e[1]["text"].as_str().unwrap_or("")))
        .collect();
    if let Some(o) = post["locals"].as_object() {
        let mut ls: Vec<(u16, &Value)> = o.iter().map(|(k, v)| (k.parse().unwrap_or(0), v)).collect();
        ls.sort_by_key(|x| x.0);
        for (k, l) in ls {
            out.push(format!("local {k} {} : {} new={}", l[0].as_str().unwrap_or(""), l[1]["text"].as_str().unwrap_or(""), l[2]));
        }
    }
    let num_map = |v: &Value| {
        let mut m: Vec<(u16, String)> =
            v.as_object().map(|o| o.iter().map(|(k, x)| (k.parse().unwrap_or(0), x.to_string())).collect()).unwrap_or_default();
        m.sort();
        format!("{m:?}")
    };
    out.push(format!("ctr {}", post["ctr"]));
    out.push(format!("synth {}", num_map(&post["synth"]).replace(['"', '\\'], "")));
    out.push(format!("depth {} decl_depth {} bind_pos {}", post["depth"], num_map(&post["decl_depth"]), num_map(&post["bind_pos"])));
    out.push(format!("underflow {}", post["underflow"]));
    out
}

pub fn snapshot_rs(st: &SimState, env: &ReplayEnv) -> Vec<String> {
    let rd = Renderer::new(env.names);
    let trivial: Vec<bool> = st.stack.iter().map(|e| is_trivial(&e.expr)).collect();
    let ids: Vec<Option<u64>> = st.stack.iter().map(|e| Some(u64::from(e.id))).collect();
    let g = groups(&trivial, &ids);
    let mut out: Vec<String> = st
        .stack
        .iter()
        .zip(g)
        .map(|(e, g)| format!("stack {} : {} {g}", rd.expr(&e.expr), type_text(&e.ty, env)))
        .collect();
    for (k, l) in &st.locals {
        out.push(format!("local {k} {} : {} new={}", l.name.as_str(), type_text(&l.ty, env), l.is_new));
    }
    let num = |m: &BTreeMap<u16, u32>| format!("{:?}", m.iter().map(|(k, v)| (*k, v.to_string())).collect::<Vec<_>>());
    out.push(format!("ctr {}", st.counter));
    let synth: Vec<(u16, String)> = st.synth_slot_kinds.iter().map(|(k, v)| (*k, format!("[{}]", v.join(",")))).collect();
    out.push(format!("synth {}", format!("{synth:?}").replace('"', "")));
    out.push(format!("depth {} decl_depth {} bind_pos {}", st.depth, num(&st.slot_decl_depth), num(&st.slot_bind_pos)));
    out.push(format!("underflow {}", st.underflow));
    out
}

/// 新增语句的快照行：文本 | slot | bind_off | value_ty
pub fn stmt_line_py(s: &Value) -> String {
    format!(
        "stmt {} | {} | {} | {}",
        s["text"].as_str().unwrap_or(""),
        s["slot"],
        s["bind_off"],
        s["value_ty"].get("text").and_then(Value::as_str).unwrap_or("null")
    )
}

pub fn stmt_line_rs(s: &Stmt, env: &ReplayEnv) -> String {
    let rd = Renderer::new(env.names);
    let origin = match s {
        Stmt::Let(l) => Some(&l.origin),
        Stmt::Assign(a) => Some(&a.origin),
        _ => None,
    };
    let opt = |x: Option<String>| x.unwrap_or_else(|| "null".to_string());
    format!(
        "stmt {} | {} | {} | {}",
        rd.stmt(s, 0),
        opt(origin.and_then(|o| o.slot).map(|x| x.to_string())),
        opt(origin.and_then(|o| o.bind_off).map(|x| x.to_string())),
        opt(origin.and_then(|o| o.value_ty.as_ref()).map(|t| rd.ty(t)))
    )
}
