//! 构建期初始化扩展的合并与规范化（计划 2026-10-05-boot-image-evaluator §5.8）。
//!
//! 映像 = 引导部分（与程序无关，各程序逐字节相同）+ 扩展组（[`IGroup`]）。扩展组的键与程序无关，同键内容相同；
//! 不同程序的扩展组集合不同（各自初始化到的类不同）。于是：
//! - 单个程序的映像在分析中按发现次序追加扩展组，分析结束后 [`ImageData::canonicalize`] 按键排序重编号；
//! - 档案按键求并（[`ImageData::absorb`]：同键复用，新键追加），全部入口并入后再规范化。
//!
//! 重编号只动扩展对象（下标 ≥ `ext_base`）：引导对象下标不变。

use std::collections::{BTreeSet, HashMap};

use crate::image::{IBody, IGroup, ILoc, IObj, IStep, IVal, ImageData};

/// 扩展组键：类初始化结果
pub const CLASS_KEY: &str = "c:";

pub fn class_of_key(key: &str) -> Option<&str> {
    key.strip_prefix(CLASS_KEY)
}

fn remap_val(v: IVal, f: &dyn Fn(u32) -> u32) -> IVal {
    match v {
        IVal::R(o) => IVal::R(f(o)),
        v => v,
    }
}

fn remap_obj(o: &IObj, f: &dyn Fn(u32) -> u32) -> IObj {
    let body = match &o.body {
        IBody::Inst(fs) => IBody::Inst(fs.iter().map(|(d, n, v)| (d.clone(), n.clone(), remap_val(*v, f))).collect()),
        IBody::Arr(es) => IBody::Arr(es.iter().map(|v| remap_val(*v, f)).collect()),
    };
    IObj { body, ..o.clone() }
}

fn remap_loc(l: &ILoc, f: &dyn Fn(u32) -> u32) -> ILoc {
    match l {
        ILoc::Static(d, n) => ILoc::Static(d.clone(), n.clone()),
        ILoc::Field(o, d, n) => ILoc::Field(f(*o), d.clone(), n.clone()),
        ILoc::Elem(o, i) => ILoc::Elem(f(*o), *i),
    }
}

/// 扩展组的步骤只有重定位槽
fn remap_step(s: &IStep, f: &dyn Fn(u32) -> u32) -> Result<IStep, String> {
    match s {
        IStep::Reloc { loc, reloc } => Ok(IStep::Reloc { loc: remap_loc(loc, f), reloc: reloc.clone() }),
        s => Err(format!("扩展组含非重定位步骤 {s:?}")),
    }
}

impl ImageData {
    /// 扩展类（扩展组 `c:<类>`）
    pub fn ext_classes(&self) -> BTreeSet<&str> {
        self.ext.iter().filter_map(|g| class_of_key(&g.key)).collect()
    }

    /// 各扩展组的步骤区间（按组次序接在引导步骤之后）
    fn group_steps(&self) -> Vec<(usize, usize)> {
        let mut at = self.ext_steps as usize;
        self.ext
            .iter()
            .map(|g| {
                let r = (at, at + g.nsteps as usize);
                at += g.nsteps as usize;
                r
            })
            .collect()
    }

    /// 引导部分是否相同（扩展组之外的全部内容；活对象不计）
    pub fn boot_eq(&self, o: &ImageData) -> bool {
        let (sc, oc) = (self.ext_classes(), o.ext_classes());
        let (sb, ob) = (self.ext_base as usize, o.ext_base as usize);
        sb == ob
            && self.objs[..sb] == o.objs[..ob]
            && self.steps[..self.ext_steps as usize] == o.steps[..o.ext_steps as usize]
            && self.statics.iter().filter(|s| !sc.contains(s.0.as_str())).eq(o.statics.iter().filter(|s| !oc.contains(s.0.as_str())))
            && self.strings.iter().filter(|&&x| (x as usize) < sb).eq(o.strings.iter().filter(|&&x| (x as usize) < ob))
            && self.build_time.iter().filter(|c| !sc.contains(c.as_str())).eq(o.build_time.iter().filter(|c| !oc.contains(c.as_str())))
            && self.cells == o.cells
            && self.exprs == o.exprs
            && self.current_thread == o.current_thread
            && self.modules == o.modules
    }

    /// 并入另一程序的映像（引导部分须相同）：同键扩展组复用（内容须相同），新键追加；活对象求并
    pub fn absorb(&mut self, o: &ImageData) -> Result<(), String> {
        if !self.boot_eq(o) {
            return Err("引导映像与其他入口不一致（参考 JDK 或运行时清单不同）".into());
        }
        let base = o.ext_base;
        let have: HashMap<String, usize> = self.ext.iter().enumerate().map(|(i, g)| (g.key.clone(), i)).collect();
        // o 的扩展下标 → 本映像下标
        let mut map = vec![u32::MAX; o.objs.len() - base as usize];
        let mut fresh: Vec<usize> = Vec::new();
        let mut next = self.objs.len() as u32;
        for (gi, g) in o.ext.iter().enumerate() {
            let at = match have.get(&g.key) {
                Some(&i) => {
                    let mine = &self.ext[i];
                    if mine.len != g.len || mine.nsteps != g.nsteps {
                        return Err(format!("扩展组 {} 与其他入口不一致（对象 {} / {}）", g.key, mine.len, g.len));
                    }
                    mine.start
                }
                None => {
                    fresh.push(gi);
                    let s = next;
                    next += g.len;
                    s
                }
            };
            for k in 0..g.len {
                map[(g.start - base + k) as usize] = at + k;
            }
        }
        let f = |x: u32| if x < base { x } else { map[(x - base) as usize] };
        // 同键复用的组：内容须相同（不同程序得到不同结果即判定依赖了程序）
        let osteps = o.group_steps();
        for g in &o.ext {
            if let Some(&i) = have.get(&g.key) {
                let mine = self.ext[i].clone();
                for k in 0..g.len {
                    if remap_obj(&o.objs[(g.start + k) as usize], &f) != self.objs[(mine.start + k) as usize] {
                        return Err(format!("扩展组 {} 与其他入口不一致（对象内容）", g.key));
                    }
                }
            }
        }
        for gi in fresh {
            let g = &o.ext[gi];
            let start = self.objs.len() as u32;
            for k in 0..g.len {
                self.objs.push(remap_obj(&o.objs[(g.start + k) as usize], &f));
            }
            let (a, b) = osteps[gi];
            for st in &o.steps[a..b] {
                self.steps.push(remap_step(st, &f)?);
            }
            if let Some(c) = class_of_key(&g.key) {
                self.statics.extend(o.statics.iter().filter(|s| s.0 == c).map(|(d, n, v)| (d.clone(), n.clone(), remap_val(*v, &f))));
                self.build_time.push(c.to_string());
            }
            self.strings.extend(o.strings.iter().filter(|&&x| x >= g.start && x < g.start + g.len).map(|&x| f(x)));
            self.ext.push(IGroup { key: g.key.clone(), start, len: g.len, nsteps: g.nsteps });
        }
        let mut live: BTreeSet<u32> = self.live.iter().copied().collect();
        live.extend(o.live.iter().map(|&x| f(x)));
        self.live = live.into_iter().collect();
        Ok(())
    }

    /// 扩展组按键排序重编号；静态字段、构建期初始化类、活对象随之有序
    pub fn canonicalize(&mut self) -> Result<(), String> {
        let base = self.ext_base;
        let mut order: Vec<usize> = (0..self.ext.len()).collect();
        order.sort_by(|&a, &b| self.ext[a].key.cmp(&self.ext[b].key));
        let gsteps = self.group_steps();
        let mut map = vec![u32::MAX; self.objs.len() - base as usize];
        let mut at = base;
        for &gi in &order {
            let g = &self.ext[gi];
            for k in 0..g.len {
                map[(g.start - base + k) as usize] = at + k;
            }
            at += g.len;
        }
        let f = |x: u32| if x < base { x } else { map[(x - base) as usize] };
        let mut objs: Vec<IObj> = self.objs[..base as usize].to_vec();
        let mut steps: Vec<IStep> = self.steps[..self.ext_steps as usize].to_vec();
        let mut ext = Vec::with_capacity(self.ext.len());
        for &gi in &order {
            let g = &self.ext[gi];
            let start = objs.len() as u32;
            for k in 0..g.len {
                objs.push(remap_obj(&self.objs[(g.start + k) as usize], &f));
            }
            let (a, b) = gsteps[gi];
            for st in &self.steps[a..b] {
                steps.push(remap_step(st, &f)?);
            }
            ext.push(IGroup { key: g.key.clone(), start, len: g.len, nsteps: g.nsteps });
        }
        self.objs = objs;
        self.steps = steps;
        self.ext = ext;
        let ec = self.ext_classes().into_iter().map(str::to_string).collect::<BTreeSet<String>>();
        for s in &mut self.statics {
            if ec.contains(&s.0) {
                s.2 = remap_val(s.2, &f);
            }
        }
        self.statics.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));
        // 驻留字符串：引导部分在前（按内容），扩展部分按组键次序
        let (mut bs, mut es): (Vec<u32>, Vec<u32>) = self.strings.iter().partition(|&&x| x < base);
        es.iter_mut().for_each(|x| *x = f(*x));
        es.sort_unstable();
        bs.extend(es);
        self.strings = bs;
        self.build_time.sort();
        self.build_time.dedup();
        let mut live: Vec<u32> = self.live.iter().map(|&x| f(x)).collect();
        live.sort_unstable();
        live.dedup();
        self.live = live;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obj(ty: &str, r: Option<u32>) -> IObj {
        IObj { ty: ty.into(), hash: None, mirror: None, deferred: None, host: None, placeholder: false, body: IBody::Inst(r.map(|r| vec![("a/X".into(), "f".into(), IVal::R(r))]).unwrap_or_default()) }
    }

    fn img(groups: &[(&str, &[IObj])], live: &[u32]) -> ImageData {
        let mut d = ImageData { objs: vec![obj("a/Boot", None)], ext_base: 1, ..Default::default() };
        for (k, os) in groups {
            let start = d.objs.len() as u32;
            d.objs.extend(os.iter().cloned());
            d.ext.push(IGroup { key: k.to_string(), start, len: os.len() as u32, nsteps: 0 });
            if let Some(c) = class_of_key(k) {
                d.build_time.push(c.to_string());
                d.statics.push((c.to_string(), "S".into(), IVal::R(start)));
            }
        }
        d.live = live.to_vec();
        d
    }

    #[test]
    fn absorb_then_canonicalize_is_order_free() {
        // 程序一：c:p/B（引用引导对象 0）、c:p/A（引用 B 的对象 1）
        let a = img(&[("c:p/B", &[obj("p/B", Some(0))]), ("c:p/A", &[obj("p/A", Some(1))])], &[1, 2]);
        // 程序二：只有 c:p/B
        let b = img(&[("c:p/B", &[obj("p/B", Some(0))])], &[1]);
        let mut x = b.clone();
        x.absorb(&a).unwrap();
        x.canonicalize().unwrap();
        let mut y = a.clone();
        y.absorb(&b).unwrap();
        y.canonicalize().unwrap();
        assert_eq!(x, y);
        assert_eq!(x.ext.iter().map(|g| g.key.as_str()).collect::<Vec<_>>(), ["c:p/A", "c:p/B"]);
        // A 的对象在前，指向 B 的对象（下标 2）
        assert_eq!(x.objs[1].body, IBody::Inst(vec![("a/X".into(), "f".into(), IVal::R(2))]));
        assert_eq!(x.statics, vec![("p/A".into(), "S".into(), IVal::R(1)), ("p/B".into(), "S".into(), IVal::R(2))]);
    }

    #[test]
    fn absorb_rejects_divergent_group() {
        let a = img(&[("c:p/B", &[obj("p/B", Some(0))])], &[]);
        let b = img(&[("c:p/B", &[obj("p/C", Some(0))])], &[]);
        let mut x = a.clone();
        assert!(x.absorb(&b).is_err());
    }
}
