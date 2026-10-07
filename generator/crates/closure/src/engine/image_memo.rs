//! 引擎：具体求值的镜像缓存并入引导映像（U13，计划 2026-10-05-boot-image-evaluator §8.4）。
//!
//! 具体求值在类镜像上写入的内存缓存（`[concrete] image_memo_fields`）以片段（`concrete/persist.rs`）给出；
//! 片段的对象追加为映像对象，值写进映像镜像的该字段并登记 [`ImageData::mirror_memos`]（发射层在启动序列中
//! 写入运行期镜像）。镜像已是活对象时补传播该字段；之后与映像中其他对象同一口径（`image_start.rs`）。
//!
//! 追加对象的编号与求值次序有关；分析结束时按（镜像, 声明类, 字段名）次序重排（新建镜像在前，按类名），
//! 使映像数据与工作表次序无关。

use super::concrete::persist::{FBody, FVal, Frag};
use super::*;
use crate::image::{IBody, IObj, IVal, ImageData};

#[derive(Default)]
pub(super) struct MemoState {
    /// 映像原有对象数（其后为追加对象）
    pub(super) base: usize,
    /// 新建的类镜像对象（类名, 对象）
    pub(super) mirrors: Vec<(String, u32)>,
    /// 已写入的缓存（镜像, 声明类, 字段名）→ 片段对象（按片段内下标）
    pub(super) blocks: BTreeMap<(String, String, String), Vec<u32>>,
}

impl<'a> Engine<'a> {
    /// 片段全部可在映像中解析：由不变静态字段持有的对象须是构建期初始化类静态字段的映像对象
    pub(super) fn image_memo_ok(&self, frags: &[Frag]) -> bool {
        let Some(s) = self.img.as_ref() else { return false };
        let ok = |v: &FVal| match v {
            FVal::Static(d, n) => s.build_time.contains(d) && matches!(s.statics.get(&(d.clone(), n.clone())), Some(IVal::R(_))),
            _ => true,
        };
        frags.iter().all(|f| {
            ok(&f.val)
                && f.objs.iter().all(|o| match &o.body {
                    FBody::Inst(fs) => fs.iter().all(|(_, _, v)| ok(v)),
                    FBody::Arr(es) => es.iter().all(ok),
                })
        })
    }

    /// 映像中类 c 的镜像对象（没有即新建）
    fn image_mirror_obj(&mut self, c: &str) -> u32 {
        let s = self.img.as_mut().expect("映像");
        if let Some(&o) = s.mirror_obj.get(c) {
            return o;
        }
        let d = Rc::make_mut(&mut s.data);
        let o = d.objs.len() as u32;
        d.objs.push(IObj { ty: CLASS.to_string(), hash: None, mirror: Some(c.to_string()), deferred: None, host: None, placeholder: false, body: IBody::Inst(Vec::new()) });
        s.live.push(false);
        s.mirror_obj.insert(c.to_string(), o);
        s.memo.mirrors.push((c.to_string(), o));
        o
    }

    /// 片段并入映像（已写入的（镜像, 字段）不重复写：同一缓存的值与求值实参无关，取首个）
    pub(super) fn image_memo_apply(&mut self, frags: &[Frag]) {
        for f in frags {
            let key = (f.mirror.to_string(), f.decl.clone(), f.name.clone());
            if self.img.as_ref().is_none_or(|s| s.memo.blocks.contains_key(&key)) {
                continue;
            }
            let mut names: Vec<&str> = Vec::new();
            for v in std::iter::once(&f.val).chain(f.objs.iter().flat_map(|o| -> Box<dyn Iterator<Item = &FVal>> {
                match &o.body {
                    FBody::Inst(fs) => Box::new(fs.iter().map(|e| &e.2)),
                    FBody::Arr(es) => Box::new(es.iter()),
                }
            })) {
                if let FVal::Mirror(m) = v {
                    names.push(m);
                }
            }
            let mut mobj: HashMap<String, u32> = HashMap::default();
            for n in names.into_iter().chain([&*f.mirror]) {
                let o = self.image_mirror_obj(n);
                mobj.insert(n.to_string(), o);
            }
            let s = self.img.as_mut().expect("映像");
            let start = s.data.objs.len() as u32;
            let res = |v: &FVal| match v {
                FVal::V(x) => *x,
                FVal::Obj(i) => IVal::R(start + i),
                FVal::Mirror(m) => IVal::R(mobj[&**m]),
                FVal::Static(d, n) => s.statics[&(d.clone(), n.clone())],
            };
            let objs: Vec<IObj> = f
                .objs
                .iter()
                .map(|o| IObj {
                    ty: o.ty.clone(),
                    hash: None,
                    mirror: None,
                    deferred: None,
                    host: None,
                    placeholder: false,
                    body: match &o.body {
                        FBody::Inst(fs) => IBody::Inst(fs.iter().map(|(d, n, v)| (d.clone(), n.clone(), res(v))).collect()),
                        FBody::Arr(es) => IBody::Arr(es.iter().map(res).collect()),
                    },
                })
                .collect();
            let val = res(&f.val);
            let mo = mobj[&*f.mirror];
            let d = Rc::make_mut(&mut s.data);
            d.objs.extend(objs);
            if let IBody::Inst(fs) = &mut d.objs[mo as usize].body {
                fs.retain(|(x, y, _)| !(*x == f.decl && *y == f.name));
                fs.push((f.decl.clone(), f.name.clone(), val));
                fs.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));
            }
            d.mirror_memos.push((mo, f.decl.clone(), f.name.clone()));
            let n = d.objs.len();
            s.live.resize(n, false);
            s.memo.blocks.insert(key, (start..n as u32).collect());
            if s.live[mo as usize] {
                s.queue.push(mo);
            }
        }
        self.image_drain();
    }

    /// 分析结束：活对象集写入映像数据；有追加对象时按规范次序重排
    pub fn image_finish(&self, out: &mut ImageData) {
        let Some(s) = self.img.as_ref() else { return };
        let live = self.image_live();
        let m = &s.memo;
        if m.blocks.is_empty() {
            out.live = live;
            return;
        }
        let mut order: Vec<u32> = Vec::new();
        let mut ms = m.mirrors.clone();
        ms.sort();
        order.extend(ms.iter().map(|x| x.1));
        for b in m.blocks.values() {
            order.extend(b);
        }
        let mut perm: Vec<u32> = (0..s.data.objs.len() as u32).collect();
        for (i, &o) in order.iter().enumerate() {
            perm[o as usize] = (m.base + i) as u32;
        }
        let re = |v: &IVal| match *v {
            IVal::R(o) => IVal::R(perm[o as usize]),
            x => x,
        };
        let mut d = (*s.data).clone();
        let mut objs: Vec<IObj> = d.objs[..m.base].to_vec();
        objs.extend(order.iter().map(|&o| d.objs[o as usize].clone()));
        for o in &mut objs {
            match &mut o.body {
                IBody::Inst(fs) => fs.iter_mut().for_each(|e| e.2 = re(&e.2)),
                IBody::Arr(es) => es.iter_mut().for_each(|e| *e = re(e)),
            }
        }
        d.objs = objs;
        let mut mm: Vec<(u32, String, String)> = d.mirror_memos.iter().map(|(o, a, b)| (perm[*o as usize], a.clone(), b.clone())).collect();
        mm.sort_by(|a, b| (&d.objs[a.0 as usize].mirror, &a.1, &a.2).cmp(&(&d.objs[b.0 as usize].mirror, &b.1, &b.2)));
        d.mirror_memos = mm;
        let mut lv: Vec<u32> = live.iter().map(|&o| perm[o as usize]).collect();
        lv.sort_unstable();
        d.live = lv;
        *out = d;
    }
}
