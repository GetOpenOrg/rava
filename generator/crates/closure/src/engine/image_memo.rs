//! 引擎：具体求值的镜像缓存并入引导映像（U13，计划 2026-10-05-boot-image-evaluator §5.6.9 / §5.8.5）。
//!
//! 具体求值在类镜像上写入的内存缓存（`[concrete] image_memo_fields`）以片段（`concrete/persist.rs`）给出。
//! 片段与构建期初始化扩展走同一套追加与规范化流程（`image_ext.rs`）：
//! - 片段对象追加为扩展组 `f:<镜像>#<声明类>.<字段>`（键与程序无关：缓存值只取决于镜像所代表的类），
//!   缓存值记为组外记录 [`IMemo`]（不写进镜像对象：镜像可能是引导对象，引导部分须与程序无关）；
//! - 映像中没有的类镜像由构建期求值器新建，按 `m:` 组追加（与扩展期新建的镜像同键同内容）；
//! - 分析结束时与扩展组一并按键排序重编号，档案按键求并（同键内容与缓存值须相同）。
//!
//! 每组记下贡献它的具体求值调用点。调用点在接收者集合增长后可能永久回退（`concrete.rs`），回退前已并入的组
//! 撤不回分析，却只取决于回退前调用点被处理时的中间集合（随处理次序变化）。分析结束时只由回退调用点贡献的组
//! 剔除（[`Engine::image_final`] → [`crate::image::ImageData::drop_memo_groups`]），映像的镜像缓存组 = 最终未回退
//! 调用点所贡献的组。
//!
//! 镜像是活对象时，缓存值随镜像传播（`image_drain` 并读 [`MemoState::of`]），之后与映像中其他对象同一口径。

use super::concrete::persist::{FBody, FVal, Frag};
use super::*;
use crate::image::{IBody, IGroup, IMemo, IObj, IVal};
use crate::image_ext::memo_key;

#[derive(Default)]
pub(super) struct MemoState {
    /// 已并入的缓存（组键）
    keys: HashSet<String>,
    /// 组键 → 贡献它的具体求值调用点（含该组已由其他调用点并入的情形）
    pub(super) sites: HashMap<String, BTreeSet<(usize, u32)>>,
    /// 镜像对象 → 其缓存（声明类, 字段名, 值）
    by_mirror: HashMap<u32, Vec<(String, String, IVal)>>,
}

impl MemoState {
    /// 镜像对象 o 上并入的缓存
    pub(super) fn of(&self, o: u32) -> &[(String, String, IVal)] {
        self.by_mirror.get(&o).map_or(&[], Vec::as_slice)
    }
}

/// 片段引用的类镜像（含被缓存的镜像本身）
fn frag_mirrors(f: &Frag) -> Vec<&str> {
    let mut names: Vec<&str> = vec![&*f.mirror];
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
    names
}

impl<'a> Engine<'a> {
    /// 片段全部可在映像中表达时备好其所需的类镜像（映像中没有的经构建期求值器新建）；否则给出原因：
    /// 由不变静态字段持有的对象须是构建期初始化类静态字段的映像对象，片段对象的类型须属映像根模块
    pub(super) fn image_memo_prepare(&mut self, frags: &[Frag]) -> Result<(), String> {
        let Some(s) = self.img.as_ref() else { return Err("无引导映像".into()) };
        let Some(x) = self.ext_vm.as_deref() else { return Err("无构建期求值器".into()) };
        let ok = |v: &FVal| match v {
            FVal::Static(d, n) if !s.build_time.contains(d) => Err(format!("引用运行期初始化类的静态字段 {d}.{n}")),
            FVal::Static(d, n) if !matches!(s.statics.get(&(d.clone(), n.clone())), Some(IVal::R(_))) => Err(format!("映像静态字段 {d}.{n} 不是对象")),
            _ => Ok(()),
        };
        for f in frags {
            ok(&f.val)?;
            for o in &f.objs {
                if !x.at_home(self.cp, &o.ty) {
                    return Err(format!("缓存对象类型 {} 不在映像根模块", o.ty));
                }
                match &o.body {
                    FBody::Inst(fs) => fs.iter().try_for_each(|(_, _, v)| ok(v))?,
                    FBody::Arr(es) => es.iter().try_for_each(ok)?,
                }
            }
        }
        for f in frags {
            for n in frag_mirrors(f) {
                self.image_mirror_obj(n)?;
            }
        }
        Ok(())
    }

    /// 映像中类 c 的镜像对象（没有即经构建期求值器新建，按 `m:` 组追加）
    fn image_mirror_obj(&mut self, c: &str) -> Result<u32, String> {
        let (Some(x), Some(s)) = (self.ext_vm.as_deref_mut(), self.img.as_deref_mut()) else { return Err("无构建期求值器".into()) };
        if let Some(&o) = s.mirror_obj.get(c) {
            return Ok(o);
        }
        let d = Rc::make_mut(&mut s.data);
        let n0 = d.objs.len();
        let o = x.mirror(&self.ctx, self.cp, c, d)?;
        self.image_appended(n0);
        Ok(o)
    }

    /// 片段并入映像（须先经 [`Self::image_memo_prepare`]）：每个（镜像, 字段）一组，已并入的不重复写——同一缓存的值
    /// 与求值实参无关，取首个；不同程序 / 实参得到不同值由档案合并的同键比对报出
    pub(super) fn image_memo_apply(&mut self, site: (usize, u32), frags: &[Frag]) {
        for f in frags {
            let key = memo_key(&f.mirror, &f.decl, &f.name);
            if let Some(s) = self.img.as_mut() {
                s.memo.sites.entry(key.clone()).or_default().insert(site);
            }
            if self.img.as_ref().is_none_or(|s| s.memo.keys.contains(&key)) {
                continue;
            }
            let mut mobj: HashMap<&str, u32> = HashMap::default();
            for n in frag_mirrors(f) {
                match self.image_mirror_obj(n) {
                    Ok(o) => {
                        mobj.insert(n, o);
                    }
                    Err(w) => unreachable!("镜像缓存的类镜像已备好：{n}：{w}"),
                }
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
            let len = objs.len() as u32;
            d.objs.extend(objs);
            d.ext.push(IGroup { key: key.clone(), start, len, nsteps: 0 });
            d.mirror_memos.push(IMemo { mirror: mo, decl: f.decl.clone(), name: f.name.clone(), val });
            let n = d.objs.len();
            s.live.resize(n, false);
            s.memo.keys.insert(key);
            s.memo.by_mirror.entry(mo).or_default().push((f.decl.clone(), f.name.clone(), val));
            if s.live[mo as usize] {
                s.queue.push(mo);
            }
        }
        self.image_drain();
    }
}
