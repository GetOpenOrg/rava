//! 构建期引导映像的物化数据（计划 2026-10-05-boot-image-evaluator §5.5.2）。
//!
//! 由引导求值（`engine/concrete/boot_image.rs`）从解释器堆导出，与堆下标无关：对象按规范根次序
//! （静态字段按声明类与字段名、VM 构造对象、类镜像按类名、驻留字符串按内容、残差记录按序）广度优先编号。
//! 抽象分析从映像出发（`engine/image_start.rs`），联合不动点求出活对象集 `live`；发射层只物化活对象，
//! 指向非活对象的引用（闭包不读取的字段）物化为 null。
//!
//! closure.json 键 `boot_image_data`：值编码见 [`IVal`]。

use serde_json::{json, Value};

/// 槽位值
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IVal {
    I(i32),
    J(i64),
    /// float / double 的位模式
    F(u32),
    D(u64),
    N,
    /// 映像对象下标
    R(u32),
    /// 启动重算槽（污点表达式下标、类型 `I` / `J`）：映像中取零值，启动序列按表达式重算后写入
    T(u32, u8),
}

#[derive(Clone, Debug, PartialEq)]
pub enum IBody {
    /// 实例字段（声明类, 字段名, 值）：只列非缺省值，按（声明类, 字段名）排序
    Inst(Vec<(String, String, IVal)>),
    Arr(Vec<IVal>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct IObj {
    /// binary name / 数组描述符
    pub ty: String,
    /// 构建期取过的身份哈希
    pub hash: Option<i32>,
    /// 类镜像所代表的类（binary name / 数组描述符 / 基本类型描述符字符）
    pub mirror: Option<String>,
    /// 宿主相关值（`@deferred`）的字符串内容数组：属性名
    pub deferred: Option<String>,
    /// 宿主相关字符串内容数组的运行期来源：（native 键，结果字符串数组的下标；`None` = native 直接返回该串）。
    /// 启动序列调用一次该 native，以宿主值改写引用此内容数组的字符串
    pub host: Option<(String, Option<u32>)>,
    /// 残差调用 / 重放 native / 运行期初始化类静态读取的结果占位对象：启动序列以运行期结果回填其引用位置
    pub placeholder: bool,
    pub body: IBody,
}

/// 启动重算槽的污点表达式
#[derive(Clone, Debug, PartialEq)]
pub enum IExpr {
    /// 宿主源 native 调用
    Src { native: String, args: Vec<IVal> },
    Un(u8, IVal),
    Bin(u8, IVal, IVal),
    Sel { cmp: u8, a: IVal, b: IVal, t: IVal, f: IVal },
}

/// 重算槽位置
#[derive(Clone, Debug, PartialEq)]
pub enum ILoc {
    Static(String, String),
    Field(u32, String, String),
    Elem(u32, u32),
}

/// 运行期重定位值：构建期求值所用的编码在运行期另行分配，映像中取零值，启动序列按运行期口径写入
#[derive(Clone, Debug, PartialEq)]
pub enum IReloc {
    /// Unsafe 实例字段偏移（声明类, 字段名）：运行期偏移是按字段身份登记的不透明 id
    FieldOffset(String, String),
    /// VM 原生单元的地址（单元名）
    Cell(String),
}

/// 启动序列（构建期次序）
#[derive(Clone, Debug, PartialEq)]
pub enum IStep {
    Recompute { loc: ILoc, expr: u32 },
    /// 重定位槽：按运行期口径取值后写入
    Reloc { loc: ILoc, reloc: IReloc },
    /// 类转为运行期初始化（首次主动使用时执行 `<clinit>`）
    RuntimeInit { class: String },
    /// 根帧调用 `phase@off` → `callee(args)`；结果回填占位对象 `ph` 的引用位置
    Call { phase: String, off: u32, callee: String, args: Vec<IVal>, ph: Option<u32> },
    /// 运行期副作用 / 结果依赖宿主的调用
    Native { callee: String, args: Vec<IVal>, ph: Option<u32> },
    /// 运行期初始化类的静态字段读取：回填占位对象 `ph` 的引用位置
    Read { decl: String, name: String, ph: u32 },
    /// 根帧区段 `[start, end)`（局部变量取区段入口的值）
    Region { phase: String, start: u32, end: Option<u32>, locals: Vec<IVal> },
    /// 引导档位变更：档位静态字段（`[concrete.boot] level`）写入 `level`，其后的步骤在该档位下重放；
    /// 序列末尾的档位步骤恢复映像值
    Level { decl: String, name: String, level: i32 },
}

/// VM 模块表项（`defineModule0` 登记，构建期次序）：启动序列以之为运行期 VM 模块表的初值，
/// 类镜像的模块（清单 `[concrete.vm_fields] class_module`）按「定义加载器 + 包」查本表
#[derive(Clone, Debug, PartialEq)]
pub struct IModule {
    /// 模块对象
    pub obj: u32,
    /// 定义加载器（清单 `[concrete.vm_fields] module_loader`；null = 引导加载器）
    pub loader: IVal,
    pub open: bool,
    /// 模块位置（defineModule0 的 location 实参，如 `jrt:/java.base`；null → None）
    pub location: Option<String>,
    /// 包（内部形式，斜线分隔）
    pub packages: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageData {
    pub objs: Vec<IObj>,
    /// 静态字段初值（声明类, 字段名, 值），按（声明类, 字段名）排序
    pub statics: Vec<(String, String, IVal)>,
    /// 驻留字符串对象（按内容排序）
    pub strings: Vec<u32>,
    /// 构建期完成初始化的类（排序；运行期初始化类不在内）
    pub build_time: Vec<String>,
    /// VM 原生单元（名字, 位模式）
    pub cells: Vec<(String, i64)>,
    pub exprs: Vec<IExpr>,
    pub steps: Vec<IStep>,
    /// 联合不动点的活对象（升序）；抽象分析之前为空
    pub live: Vec<u32>,
    /// VM 初始线程（HotSpot `create_initial_thread` 的 main 线程对象）：启动序列把它绑定为 OS 主线程的
    /// 当前线程（§5.5.1 S3）
    pub current_thread: Option<u32>,
    /// VM 模块表（§5.5.1 S5）
    pub modules: Vec<IModule>,
    /// 引导部分的对象数 / 启动步骤数：其后是追加的扩展组（构建期初始化扩展与镜像缓存，§5.8），按 [`IGroup`] 分组
    pub ext_base: u32,
    pub ext_steps: u32,
    pub ext: Vec<IGroup>,
    /// 类镜像上写入映像的内存缓存（U13，`[concrete] image_memo_fields`；每条对应扩展组 `f:<镜像>#<声明类>.<字段>`），
    /// 按组键排序。值不写进镜像对象本身（镜像可能是引导对象，引导部分须与程序无关），启动序列写入运行期镜像
    pub mirror_memos: Vec<IMemo>,
}

/// 映像追加部分的一组（计划 §5.8 / §5.8.5）：键与程序无关——`c:<类>` 类初始化结果 / `s:<内容>` 驻留字符串 /
/// `m:<类型>` 类镜像 / `f:<镜像>#<声明类>.<字段>` 镜像缓存（U13）。组内对象在 `objs[start..start + len]`，
/// 组的重定位步骤按组次序接在引导步骤之后（`nsteps` 条）。同一键在任何程序中内容相同，档案按键求并
#[derive(Clone, Debug, PartialEq)]
pub struct IGroup {
    pub key: String,
    pub start: u32,
    pub len: u32,
    pub nsteps: u32,
}

/// 镜像缓存（U13）：镜像对象 `mirror` 的字段（声明类, 字段名）在运行期取值 `val`（值的对象图在同键扩展组内）
#[derive(Clone, Debug, PartialEq)]
pub struct IMemo {
    pub mirror: u32,
    pub decl: String,
    pub name: String,
    pub val: IVal,
}

fn val(v: IVal) -> Value {
    match v {
        IVal::I(x) => json!(x),
        IVal::N => Value::Null,
        IVal::J(x) => json!(format!("J{x}")),
        IVal::F(x) => json!(format!("F{x:08x}")),
        IVal::D(x) => json!(format!("D{x:016x}")),
        IVal::R(o) => json!(format!("#{o}")),
        IVal::T(e, t) => json!(format!("T{e}:{}", t as char)),
    }
}

fn unval(v: &Value) -> Result<IVal, String> {
    let bad = || format!("映像值格式：{v}");
    Ok(match v {
        Value::Null => IVal::N,
        Value::Number(n) => IVal::I(n.as_i64().and_then(|x| i32::try_from(x).ok()).ok_or_else(bad)?),
        Value::String(s) => {
            let (k, rest) = s.split_at(s.chars().next().map_or(0, char::len_utf8));
            match k {
                "J" => IVal::J(rest.parse().map_err(|_| bad())?),
                "F" => IVal::F(u32::from_str_radix(rest, 16).map_err(|_| bad())?),
                "D" => IVal::D(u64::from_str_radix(rest, 16).map_err(|_| bad())?),
                "#" => IVal::R(rest.parse().map_err(|_| bad())?),
                "T" => {
                    let (e, t) = rest.split_once(':').ok_or_else(bad)?;
                    IVal::T(e.parse().map_err(|_| bad())?, *t.as_bytes().first().ok_or_else(bad)?)
                }
                _ => return Err(bad()),
            }
        }
        _ => return Err(bad()),
    })
}

fn vals(vs: &[IVal]) -> Value {
    Value::Array(vs.iter().map(|&v| val(v)).collect())
}

fn unvals(v: Option<&Value>) -> Result<Vec<IVal>, String> {
    v.and_then(Value::as_array).ok_or("映像值数组缺失")?.iter().map(unval).collect()
}

fn s(v: &Value, k: &str) -> Result<String, String> {
    v.get(k).and_then(Value::as_str).map(str::to_string).ok_or_else(|| format!("映像键 {k} 缺失"))
}

fn n(v: &Value, k: &str) -> Result<u32, String> {
    v.get(k).and_then(Value::as_u64).map(|x| x as u32).ok_or_else(|| format!("映像键 {k} 缺失"))
}

fn opt_n(v: &Value, k: &str) -> Option<u32> {
    v.get(k).and_then(Value::as_u64).map(|x| x as u32)
}

fn triple(t: &(String, String, IVal)) -> Value {
    json!([t.0, t.1, val(t.2)])
}

fn untriple(v: &Value) -> Result<(String, String, IVal), String> {
    let a = v.as_array().filter(|a| a.len() == 3).ok_or("映像三元组格式")?;
    Ok((a[0].as_str().ok_or("映像三元组格式")?.to_string(), a[1].as_str().ok_or("映像三元组格式")?.to_string(), unval(&a[2])?))
}

fn loc_json(loc: &ILoc) -> Value {
    match loc {
        ILoc::Static(d, n) => json!({ "static": [d, n] }),
        ILoc::Field(o, d, n) => json!({ "obj": o, "field": [d, n] }),
        ILoc::Elem(o, i) => json!({ "obj": o, "elem": i }),
    }
}

fn unloc(l: &Value) -> Result<ILoc, String> {
    let two = |k: &str| l.get(k).and_then(Value::as_array).filter(|a| a.len() == 2);
    let st = |v: &Value| v.as_str().map(str::to_string).ok_or("映像序列格式");
    Ok(if let Some(a) = two("static") {
        ILoc::Static(st(&a[0])?, st(&a[1])?)
    } else if let Some(f) = two("field") {
        ILoc::Field(n(l, "obj")?, st(&f[0])?, st(&f[1])?)
    } else {
        ILoc::Elem(n(l, "obj")?, n(l, "elem")?)
    })
}

impl ImageData {
    /// 启动序列写入类 `cls` 的静态字段：静态初值，或档位 / 重定位 / 重算步骤写静态位置。
    /// 发射层以此判定该类的静态字段须有真实存储（含免触发 setter），与启动序列的写入同一口径
    pub fn writes_statics_of(&self, cls: &str) -> bool {
        self.statics.iter().any(|(c, _, _)| c == cls)
            || self.steps.iter().any(|st| match st {
                IStep::Level { decl, .. } => decl == cls,
                IStep::Reloc { loc: ILoc::Static(c, _), .. } | IStep::Recompute { loc: ILoc::Static(c, _), .. } => c == cls,
                _ => false,
            })
    }

    pub fn to_json(&self) -> Value {
        let objs: Vec<Value> = self
            .objs
            .iter()
            .map(|o| {
                let mut v = json!({ "ty": o.ty });
                if let Some(h) = o.hash {
                    v["hash"] = json!(h);
                }
                if let Some(m) = &o.mirror {
                    v["mirror"] = json!(m);
                }
                if let Some(d) = &o.deferred {
                    v["deferred"] = json!(d);
                }
                if let Some((n, i)) = &o.host {
                    v["host"] = json!([n, i]);
                }
                if o.placeholder {
                    v["placeholder"] = json!(true);
                }
                match &o.body {
                    IBody::Inst(fs) => v["fields"] = Value::Array(fs.iter().map(triple).collect()),
                    IBody::Arr(es) => v["elems"] = vals(es),
                }
                v
            })
            .collect();
        let exprs: Vec<Value> = self
            .exprs
            .iter()
            .map(|e| match e {
                IExpr::Src { native, args } => json!({ "src": native, "args": vals(args) }),
                IExpr::Un(op, x) => json!({ "un": op, "x": val(*x) }),
                IExpr::Bin(op, a, b) => json!({ "bin": op, "a": val(*a), "b": val(*b) }),
                IExpr::Sel { cmp, a, b, t, f } => json!({ "sel": cmp, "a": val(*a), "b": val(*b), "t": val(*t), "f": val(*f) }),
            })
            .collect();
        let steps: Vec<Value> = self
            .steps
            .iter()
            .map(|st| match st {
                IStep::Recompute { loc, expr } => json!({ "recompute": loc_json(loc), "expr": expr }),
                IStep::Reloc { loc, reloc: IReloc::FieldOffset(d, n) } => json!({ "reloc": loc_json(loc), "field_offset": [d, n] }),
                IStep::Reloc { loc, reloc: IReloc::Cell(c) } => json!({ "reloc": loc_json(loc), "cell": c }),
                IStep::RuntimeInit { class } => json!({ "runtime_init": class }),
                IStep::Call { phase, off, callee, args, ph } => json!({ "call": callee, "phase": phase, "off": off, "args": vals(args), "ph": ph }),
                IStep::Native { callee, args, ph } => json!({ "native": callee, "args": vals(args), "ph": ph }),
                IStep::Read { decl, name, ph } => json!({ "read": [decl, name], "ph": ph }),
                IStep::Region { phase, start, end, locals } => json!({ "region": phase, "start": start, "end": end, "locals": vals(locals) }),
                IStep::Level { decl, name, level } => json!({ "level": level, "at": [decl, name] }),
            })
            .collect();
        json!({
            "objs": objs,
            "statics": self.statics.iter().map(triple).collect::<Vec<_>>(),
            "strings": self.strings,
            "build_time": self.build_time,
            "cells": self.cells.iter().map(|(k, v)| json!([k, v])).collect::<Vec<_>>(),
            "exprs": exprs,
            "steps": steps,
            "live": self.live,
            "current_thread": self.current_thread,
            "modules": self.modules.iter().map(|m| json!({ "obj": m.obj, "loader": val(m.loader), "open": m.open, "location": m.location, "packages": m.packages })).collect::<Vec<_>>(),
            "ext_base": self.ext_base,
            "ext_steps": self.ext_steps,
            "ext": self.ext.iter().map(|g| json!([g.key, g.start, g.len, g.nsteps])).collect::<Vec<_>>(),
            "mirror_memos": self.mirror_memos.iter().map(|m| json!([m.mirror, m.decl, m.name, val(m.val)])).collect::<Vec<_>>(),
        })
    }

    pub fn from_json(v: &Value) -> Result<ImageData, String> {
        let arr = |k: &str| v.get(k).and_then(Value::as_array).ok_or_else(|| format!("映像键 {k} 缺失"));
        let mut d = ImageData::default();
        for o in arr("objs")? {
            let body = match o.get("fields") {
                Some(fs) => IBody::Inst(fs.as_array().ok_or("映像字段格式")?.iter().map(untriple).collect::<Result<_, _>>()?),
                None => IBody::Arr(unvals(o.get("elems"))?),
            };
            d.objs.push(IObj {
                ty: s(o, "ty")?,
                hash: o.get("hash").and_then(Value::as_i64).map(|h| h as i32),
                mirror: o.get("mirror").and_then(Value::as_str).map(str::to_string),
                deferred: o.get("deferred").and_then(Value::as_str).map(str::to_string),
                host: match o.get("host").and_then(Value::as_array) {
                    Some(h) => Some((
                        h.first().and_then(Value::as_str).ok_or("映像宿主来源格式")?.to_string(),
                        h.get(1).and_then(Value::as_u64).map(|i| i as u32),
                    )),
                    None => None,
                },
                placeholder: o.get("placeholder").and_then(Value::as_bool).unwrap_or(false),
                body,
            });
        }
        d.current_thread = v.get("current_thread").and_then(Value::as_u64).map(|x| x as u32);
        d.statics = arr("statics")?.iter().map(untriple).collect::<Result<_, _>>()?;
        d.strings = arr("strings")?.iter().map(|x| x.as_u64().map(|x| x as u32).ok_or("映像驻留表格式")).collect::<Result<_, _>>()?;
        d.build_time = arr("build_time")?.iter().map(|x| x.as_str().map(str::to_string).ok_or("映像类表格式")).collect::<Result<_, _>>()?;
        for c in arr("cells")? {
            let a = c.as_array().filter(|a| a.len() == 2).ok_or("映像单元格式")?;
            d.cells.push((a[0].as_str().ok_or("映像单元格式")?.to_string(), a[1].as_i64().ok_or("映像单元格式")?));
        }
        for e in arr("exprs")? {
            let g = |k: &str| e.get(k).ok_or_else(|| format!("映像表达式键 {k} 缺失")).and_then(unval);
            d.exprs.push(if let Some(nat) = e.get("src").and_then(Value::as_str) {
                IExpr::Src { native: nat.to_string(), args: unvals(e.get("args"))? }
            } else if let Some(op) = e.get("un").and_then(Value::as_u64) {
                IExpr::Un(op as u8, g("x")?)
            } else if let Some(op) = e.get("bin").and_then(Value::as_u64) {
                IExpr::Bin(op as u8, g("a")?, g("b")?)
            } else {
                IExpr::Sel { cmp: e.get("sel").and_then(Value::as_u64).ok_or("映像表达式格式")? as u8, a: g("a")?, b: g("b")?, t: g("t")?, f: g("f")? }
            });
        }
        for st in arr("steps")? {
            let pair = |k: &str| -> Result<(String, String), String> {
                let a = st.get(k).and_then(Value::as_array).filter(|a| a.len() == 2).ok_or("映像序列格式")?;
                Ok((a[0].as_str().ok_or("映像序列格式")?.to_string(), a[1].as_str().ok_or("映像序列格式")?.to_string()))
            };
            d.steps.push(if let Some(l) = st.get("recompute") {
                IStep::Recompute { loc: unloc(l)?, expr: n(st, "expr")? }
            } else if let Some(l) = st.get("reloc") {
                let reloc = match st.get("cell").and_then(Value::as_str) {
                    Some(c) => IReloc::Cell(c.to_string()),
                    None => {
                        let (d, n) = pair("field_offset")?;
                        IReloc::FieldOffset(d, n)
                    }
                };
                IStep::Reloc { loc: unloc(l)?, reloc }
            } else if let Some(c) = st.get("runtime_init").and_then(Value::as_str) {
                IStep::RuntimeInit { class: c.to_string() }
            } else if st.get("call").is_some() {
                IStep::Call { phase: s(st, "phase")?, off: n(st, "off")?, callee: s(st, "call")?, args: unvals(st.get("args"))?, ph: opt_n(st, "ph") }
            } else if st.get("native").is_some() {
                IStep::Native { callee: s(st, "native")?, args: unvals(st.get("args"))?, ph: opt_n(st, "ph") }
            } else if let Some(l) = st.get("level").and_then(Value::as_i64) {
                let (decl, name) = pair("at")?;
                IStep::Level { decl, name, level: l as i32 }
            } else if st.get("read").is_some() {
                let (decl, name) = pair("read")?;
                IStep::Read { decl, name, ph: n(st, "ph")? }
            } else {
                IStep::Region { phase: s(st, "region")?, start: n(st, "start")?, end: opt_n(st, "end"), locals: unvals(st.get("locals"))? }
            });
        }
        for m in v.get("modules").and_then(Value::as_array).into_iter().flatten() {
            d.modules.push(IModule {
                obj: n(m, "obj")?,
                loader: unval(m.get("loader").ok_or("映像模块表格式")?)?,
                open: m.get("open").and_then(Value::as_bool).ok_or("映像模块表格式")?,
                location: m.get("location").and_then(Value::as_str).map(str::to_string),
                packages: m
                    .get("packages")
                    .and_then(Value::as_array)
                    .ok_or("映像模块表格式")?
                    .iter()
                    .map(|p| p.as_str().map(str::to_string).ok_or("映像模块表格式"))
                    .collect::<Result<_, _>>()?,
            });
        }
        d.ext_base = opt_n(v, "ext_base").unwrap_or(d.objs.len() as u32);
        d.ext_steps = opt_n(v, "ext_steps").unwrap_or(d.steps.len() as u32);
        for g in v.get("ext").and_then(Value::as_array).into_iter().flatten() {
            let a = g.as_array().filter(|a| a.len() == 4).ok_or("映像扩展组格式")?;
            let u = |i: usize| a[i].as_u64().map(|x| x as u32).ok_or("映像扩展组格式");
            d.ext.push(IGroup { key: a[0].as_str().ok_or("映像扩展组格式")?.to_string(), start: u(1)?, len: u(2)?, nsteps: u(3)? });
        }
        for m in arr("mirror_memos")? {
            let a = m.as_array().filter(|a| a.len() == 4).ok_or("映像镜像缓存格式")?;
            let o = a[0].as_u64().ok_or("映像镜像缓存格式")? as u32;
            let st = |v: &Value| v.as_str().map(str::to_string).ok_or("映像镜像缓存格式");
            d.mirror_memos.push(IMemo { mirror: o, decl: st(&a[1])?, name: st(&a[2])?, val: unval(&a[3])? });
        }
        d.live = arr("live")?.iter().map(|x| x.as_u64().map(|x| x as u32).ok_or("映像活对象格式")).collect::<Result<_, _>>()?;
        Ok(d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_roundtrip() {
        let d = ImageData {
            objs: vec![
                IObj { ty: "a/B".into(), hash: Some(-7), mirror: None, deferred: None, host: None, placeholder: false, body: IBody::Inst(vec![("a/B".into(), "x".into(), IVal::R(1)), ("a/B".into(), "y".into(), IVal::J(-3))]) },
                IObj { ty: "[B".into(), hash: None, mirror: None, deferred: Some("p.q".into()), host: Some(("a/R.p:()[La/S;".into(), Some(3))), placeholder: false, body: IBody::Arr(vec![IVal::I(1), IVal::N, IVal::F(0x3f80_0000), IVal::D(1), IVal::T(0, b'J')]) },
                IObj { ty: "a/M".into(), hash: None, mirror: Some("I".into()), deferred: None, host: Some(("a/R.q:()La/S;".into(), None)), placeholder: true, body: IBody::Inst(vec![]) },
            ],
            statics: vec![("a/B".into(), "s".into(), IVal::R(0))],
            strings: vec![1],
            build_time: vec!["a/B".into()],
            cells: vec![("next_thread_id".into(), 1)],
            exprs: vec![IExpr::Src { native: "n:()I".into(), args: vec![IVal::I(2)] }, IExpr::Bin(0x60, IVal::T(0, b'I'), IVal::I(1))],
            steps: vec![
                IStep::Recompute { loc: ILoc::Field(0, "a/B".into(), "y".into()), expr: 1 },
                IStep::Recompute { loc: ILoc::Static("a/B".into(), "s".into()), expr: 0 },
                IStep::Recompute { loc: ILoc::Elem(1, 4), expr: 0 },
                IStep::Reloc { loc: ILoc::Static("a/B".into(), "OFF".into()), reloc: IReloc::FieldOffset("a/B".into(), "y".into()) },
                IStep::Reloc { loc: ILoc::Elem(1, 3), reloc: IReloc::Cell("next_thread_id".into()) },
                IStep::RuntimeInit { class: "a/C".into() },
                IStep::Call { phase: "a/B.p:()V".into(), off: 3, callee: "a/B.q:()La/B;".into(), args: vec![IVal::R(0)], ph: Some(2) },
                IStep::Native { callee: "a/B.n:()V".into(), args: vec![], ph: None },
                IStep::Read { decl: "a/C".into(), name: "X".into(), ph: 2 },
                IStep::Level { decl: "a/V".into(), name: "lv".into(), level: 1 },
                IStep::Region { phase: "a/B.p:()V".into(), start: 1, end: None, locals: vec![IVal::N] },
            ],
            live: vec![0, 2],
            current_thread: Some(0),
            modules: vec![IModule { obj: 0, loader: IVal::N, open: false, location: Some("jrt:/a".into()), packages: vec!["a".into(), "a/b".into()] }],
            ext_base: 2,
            ext_steps: 10,
            ext: vec![IGroup { key: "c:a/B".into(), start: 2, len: 1, nsteps: 1 }, IGroup { key: "f:a/M#a/M.memo".into(), start: 3, len: 0, nsteps: 0 }],
            mirror_memos: vec![IMemo { mirror: 2, decl: "a/M".into(), name: "memo".into(), val: IVal::R(0) }],
        };
        let back = ImageData::from_json(&d.to_json()).unwrap();
        assert_eq!(back, d);
    }
}
