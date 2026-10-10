//! 镜像缓存入映像（U13，计划 2026-10-05-boot-image-evaluator §8.4）：具体求值在类镜像上写入的内存缓存
//! （`[concrete] image_memo_fields`，如 `Class.genericInfo`）连同其对象图导出为与解释器堆无关的片段，
//! 由引擎并入引导映像（`engine/image_memo.rs`）；运行期镜像在启动序列中写入该缓存，命中后不再重算。
//!
//! 一组实参可物化的条件：求值期间对映像状态的全部写入（撤销日志）都是类镜像上 `image_memo_fields` 的字段，
//! 且其值的对象图只由求值纪元内新建的对象、类镜像与「由不变静态字段持有的映像对象」组成（后者按该静态字段在
//! 引导映像中的值解析，见引擎侧）。新建对象带身份哈希（取值依赖求值次序）或是 lambda 即不可物化。

use std::collections::VecDeque;

use super::vm::*;
use super::*;
use crate::image::IVal;

/// 片段内的值
#[derive(Clone, Debug, PartialEq)]
pub(in crate::engine) enum FVal {
    /// 基本类型 / null
    V(IVal),
    /// 片段内对象（下标）
    Obj(u32),
    /// 类镜像（binary name / 数组描述符 / 基本类型描述符字符）
    Mirror(Rc<str>),
    /// 由不变静态字段持有的映像对象（声明类, 字段名）
    Static(String, String),
    /// 映像中的驻留字符串（UTF-16 内容；按内容取映像的驻留对象，驻留串的身份即其内容）
    Str(Vec<u16>),
}

#[derive(Clone, Debug, PartialEq)]
pub(in crate::engine) enum FBody {
    Inst(Vec<(String, String, FVal)>),
    Arr(Vec<FVal>),
}

#[derive(Clone, Debug, PartialEq)]
pub(in crate::engine) struct FObj {
    pub ty: String,
    pub body: FBody,
}

/// 一个镜像缓存字段的值与其对象图（对象按自值起的广度优先次序编号，确定）
#[derive(Clone, Debug, PartialEq)]
pub(in crate::engine) struct Frag {
    pub mirror: Rc<str>,
    pub decl: String,
    pub name: String,
    pub val: FVal,
    pub objs: Vec<FObj>,
}

fn is_default(v: CV) -> bool {
    match v {
        CV::I(0) | CV::J(0) | CV::N => true,
        CV::F(x) => x.to_bits() == 0,
        CV::D(x) => x.to_bits() == 0,
        _ => false,
    }
}

struct Ex<'v> {
    vm: &'v Vm,
    /// 驻留字符串对象 → 内容（首次遇到映像中的非静态所持对象时建）
    interned: Option<HashMap<u32, &'v [u16]>>,
    ids: HashMap<u32, u32>,
    order: Vec<u32>,
    q: VecDeque<u32>,
}

impl Ex<'_> {
    fn val(&mut self, v: CV) -> Result<FVal, String> {
        Ok(match v {
            CV::I(x) => FVal::V(IVal::I(x)),
            CV::J(x) => FVal::V(IVal::J(x)),
            CV::F(x) => FVal::V(IVal::F(x.to_bits())),
            CV::D(x) => FVal::V(IVal::D(x.to_bits())),
            CV::N => FVal::V(IVal::N),
            CV::T(..) => return Err("污点值".into()),
            CV::R(o) => {
                if let Some(t) = self.vm.mirror_of.get(&o) {
                    return Ok(FVal::Mirror(t.clone()));
                }
                if self.vm.bj.ph_origin.contains_key(&o) {
                    return Err("占位对象（初始化不可建模类的静态字段）".into());
                }
                let h = &self.vm.heap[o as usize];
                if h.epoch == 0 {
                    if let Some(f) = self.vm.image_roots.get(&o) {
                        return Ok(FVal::Static(f.owner.clone(), f.name.clone()));
                    }
                    let vm = self.vm;
                    let rev = self.interned.get_or_insert_with(|| vm.strings.iter().map(|(u, &o)| (o, u.as_slice())).collect());
                    return match rev.get(&o) {
                        Some(u) => Ok(FVal::Str(u.to_vec())),
                        None => Err(format!("引用映像对象 {}（非不变静态字段所持、非驻留字符串）", h.ty)),
                    };
                }
                if self.vm.ihash.contains_key(&o) {
                    return Err(format!("{} 对象取过身份哈希", h.ty));
                }
                if matches!(h.body, Body::Lam(_)) {
                    return Err(format!("lambda 对象 {}", h.ty));
                }
                if let Some(&i) = self.ids.get(&o) {
                    return Ok(FVal::Obj(i));
                }
                let i = self.order.len() as u32;
                self.ids.insert(o, i);
                self.order.push(o);
                self.q.push_back(o);
                FVal::Obj(i)
            }
        })
    }

    fn frag(vm: &Vm, mirror: Rc<str>, decl: &str, name: &str, v: CV) -> Result<Frag, String> {
        let mut x = Ex { vm, interned: None, ids: HashMap::default(), order: Vec::new(), q: VecDeque::new() };
        let val = x.val(v)?;
        let mut objs = Vec::new();
        while let Some(o) = x.q.pop_front() {
            let h = &vm.heap[o as usize];
            let body = match &h.body {
                Body::Inst(fs) => {
                    let mut fs: Vec<(String, String, CV)> =
                        fs.iter().filter(|(_, v)| !is_default(*v)).map(|(k, v)| (vm.fnames[*k as usize].0.to_string(), vm.fnames[*k as usize].1.to_string(), *v)).collect();
                    fs.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));
                    let mut out = Vec::with_capacity(fs.len());
                    for (d, n, v) in fs {
                        out.push((d, n, x.val(v)?));
                    }
                    FBody::Inst(out)
                }
                Body::Arr(es) => FBody::Arr(es.iter().map(|&e| x.val(e)).collect::<Result<_, _>>()?),
                Body::Lam(_) => return Err(format!("lambda 对象 {}", h.ty)),
            };
            objs.push(FObj { ty: h.ty.to_string(), body });
        }
        Ok(Frag { mirror, decl: decl.to_string(), name: name.to_string(), val, objs })
    }
}

/// 撤销日志的一条写入是否为类镜像上 `image_memo_fields` 的字段（可物化进映像）；否则给出原因
fn image_write(vm: &Vm, env: &Env, o: u32, k: u32) -> Result<Rc<str>, String> {
    let (d, n) = &vm.fnames[k as usize];
    if o == u32::MAX {
        return Err(format!("写静态字段 {d}.{n}"));
    }
    let t = vm.mirror_of.get(&o).ok_or_else(|| format!("写映像对象 {} 的字段 {d}.{n}", vm.heap[o as usize].ty))?;
    if !env.cfg().image_memo_fields.contains(&format!("{d}.{n}")) {
        return Err(format!("写镜像 {t} 的非映像缓存字段 {d}.{n}"));
    }
    Ok(t.clone())
}

/// 冷求值后撤销日志里既有映像缓存字段的写入、又有其他缓存写入（需温求值）
pub(super) fn mixed(vm: &Vm, env: &Env) -> bool {
    let (mut img, mut other) = (false, false);
    for &(o, k, _) in &vm.undo {
        match image_write(vm, env, o, k) {
            Ok(_) => img = true,
            Err(_) => other = true,
        }
    }
    img && other
}

/// 只撤销非映像缓存字段的写入；映像缓存字段的写入原样留在撤销日志（求值结束时整体撤销）
pub(super) fn rollback_non_image(vm: &mut Vm, env: &Env) {
    let all = std::mem::take(&mut vm.undo);
    let (keep, drop): (Vec<_>, Vec<_>) = all.into_iter().partition(|&(o, k, _)| image_write(vm, env, o, k).is_ok());
    vm.undo = drop;
    vm.rollback();
    vm.undo = keep;
}

/// 求值结束、撤销之前：映像缓存字段的写入可物化时，按（镜像, 字段）次序给出各缓存片段，另附运行期仍重算的
/// 非映像缓存（partial = 已做温求值，非映像写入不阻止物化）；否则给出原因（该组实参按冷 / 热轨迹的并入闭包）
pub(super) fn collect(vm: &Vm, env: &Env, partial: bool) -> Result<(Vec<Frag>, Option<String>), String> {
    if vm.undo.is_empty() {
        return Err("无缓存写入".into());
    }
    let mut keys: BTreeMap<(Rc<str>, u32), u32> = BTreeMap::new();
    let mut other = None;
    for &(o, k, _) in &vm.undo {
        match image_write(vm, env, o, k) {
            Ok(t) => {
                keys.insert((t, k), o);
            }
            Err(w) => {
                other.get_or_insert(w);
            }
        }
    }
    if let Some(w) = &other {
        if !partial || keys.is_empty() {
            return Err(w.clone());
        }
    }
    let mut out = Vec::with_capacity(keys.len());
    for ((t, k), o) in keys {
        let (d, n) = &vm.fnames[k as usize];
        let Body::Inst(fs) = &vm.heap[o as usize].body else { return Err("镜像不是实例".into()) };
        let v = fs.iter().find(|(x, _)| *x == k).map_or(CV::N, |e| e.1);
        out.push(Ex::frag(vm, t, d, n, v)?);
    }
    Ok((out, other))
}
