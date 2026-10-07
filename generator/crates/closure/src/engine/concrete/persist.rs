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
    ids: HashMap<u32, u32>,
    order: Vec<u32>,
    q: VecDeque<u32>,
}

impl Ex<'_> {
    fn val(&mut self, v: CV) -> Option<FVal> {
        Some(match v {
            CV::I(x) => FVal::V(IVal::I(x)),
            CV::J(x) => FVal::V(IVal::J(x)),
            CV::F(x) => FVal::V(IVal::F(x.to_bits())),
            CV::D(x) => FVal::V(IVal::D(x.to_bits())),
            CV::N => FVal::V(IVal::N),
            CV::T(..) => return None,
            CV::R(o) => {
                if let Some(t) = self.vm.mirror_of.get(&o) {
                    return Some(FVal::Mirror(t.clone()));
                }
                let h = &self.vm.heap[o as usize];
                if h.epoch == 0 {
                    let f = self.vm.image_roots.get(&o)?;
                    return Some(FVal::Static(f.owner.clone(), f.name.clone()));
                }
                if self.vm.ihash.contains_key(&o) || matches!(h.body, Body::Lam(_)) {
                    return None;
                }
                if let Some(&i) = self.ids.get(&o) {
                    return Some(FVal::Obj(i));
                }
                let i = self.order.len() as u32;
                self.ids.insert(o, i);
                self.order.push(o);
                self.q.push_back(o);
                FVal::Obj(i)
            }
        })
    }

    fn frag(vm: &Vm, mirror: Rc<str>, decl: &str, name: &str, v: CV) -> Option<Frag> {
        let mut x = Ex { vm, ids: HashMap::default(), order: Vec::new(), q: VecDeque::new() };
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
                Body::Arr(es) => FBody::Arr(es.iter().map(|&e| x.val(e)).collect::<Option<_>>()?),
                Body::Lam(_) => return None,
            };
            objs.push(FObj { ty: h.ty.to_string(), body });
        }
        Some(Frag { mirror, decl: decl.to_string(), name: name.to_string(), val, objs })
    }
}

/// 求值（冷 / 热两次）结束、撤销之前：映像状态的写入全部可物化时，按（镜像, 字段）次序给出各缓存片段；
/// 否则 None（该组实参按冷 / 热轨迹的并入闭包）
pub(super) fn collect(vm: &Vm, env: &Env) -> Option<Vec<Frag>> {
    let img = &env.cfg().image_memo_fields;
    if img.is_empty() || vm.undo.is_empty() {
        return None;
    }
    let mut keys: BTreeMap<(Rc<str>, u32), u32> = BTreeMap::new();
    for &(o, k, _) in &vm.undo {
        if o == u32::MAX {
            return None;
        }
        let t = vm.mirror_of.get(&o)?;
        let (d, n) = &vm.fnames[k as usize];
        if !img.contains(&format!("{d}.{n}")) {
            return None;
        }
        keys.insert((t.clone(), k), o);
    }
    let mut out = Vec::with_capacity(keys.len());
    for ((t, k), o) in keys {
        let (d, n) = &vm.fnames[k as usize];
        let Body::Inst(fs) = &vm.heap[o as usize].body else { return None };
        let v = fs.iter().find(|(x, _)| *x == k).map_or(CV::N, |e| e.1);
        out.push(Ex::frag(vm, t, d, n, v)?);
    }
    Some(out)
}
