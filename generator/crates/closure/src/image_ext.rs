//! 构建期初始化扩展的合并与规范化（计划 2026-10-05-boot-image-evaluator §5.8）。
//!
//! 映像 = 引导部分（与程序无关，各程序逐字节相同）+ 扩展组（[`IGroup`]）。分析期追加进映像的对象全部按组表达：
//! 构建期初始化扩展（`c:` / `s:` / `m:`）与具体求值的镜像缓存（U13，`f:`，组外记录 [`IMemo`]）是同一套追加与
//! 规范化流程（§5.8.5）。扩展组的键与程序无关，同键内容相同；不同程序的扩展组集合不同（各自初始化到 / 求值到的
//! 类不同）。于是：
//! - 单个程序的映像在分析中按发现次序追加扩展组，分析结束后 [`ImageData::canonicalize`] 按键排序重编号；
//! - 档案按键求并（[`ImageData::absorb`]：同键复用，新键追加），全部入口并入后再规范化。
//!
//! 重编号只动扩展对象（下标 ≥ `ext_base`）：引导对象下标不变。

use std::collections::{BTreeSet, HashMap};

use crate::image::{IBody, IGroup, ILoc, IMemo, IObj, IStep, IVal, ImageData};

/// 扩展组键：类初始化结果
pub const CLASS_KEY: &str = "c:";
/// 扩展组键：类镜像（扩展期新建；镜像缓存所需的镜像同经构建期求值器新建，内容同一口径）
pub const MIRROR_KEY: &str = "m:";
/// 扩展组键：镜像缓存（U13）
pub const MEMO_KEY: &str = "f:";

pub fn class_of_key(key: &str) -> Option<&str> {
    key.strip_prefix(CLASS_KEY)
}

/// 镜像缓存的组键：镜像所代表的类型 + 字段
pub fn memo_key(mirror: &str, decl: &str, name: &str) -> String {
    format!("{MEMO_KEY}{mirror}#{decl}.{name}")
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

fn remap_memo(m: &IMemo, f: &dyn Fn(u32) -> u32) -> IMemo {
    IMemo { mirror: f(m.mirror), decl: m.decl.clone(), name: m.name.clone(), val: remap_val(m.val, f) }
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

    /// 镜像缓存的组键（镜像对象须是类镜像）
    pub fn memo_key_of(&self, m: &IMemo) -> Result<String, String> {
        let t = self.objs.get(m.mirror as usize).and_then(|o| o.mirror.as_deref()).ok_or_else(|| format!("镜像缓存 {}.{} 的目标 #{} 不是类镜像", m.decl, m.name, m.mirror))?;
        Ok(memo_key(t, &m.decl, &m.name))
    }

    /// 组键 → 镜像缓存记录
    fn memos_by_key(&self) -> Result<HashMap<String, &IMemo>, String> {
        self.mirror_memos.iter().map(|m| Ok((self.memo_key_of(m)?, m))).collect()
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
        let mine_memos: HashMap<String, IMemo> = self.memos_by_key()?.into_iter().map(|(k, m)| (k, m.clone())).collect();
        let their_memos = o.memos_by_key()?;
        for g in &o.ext {
            if let Some(&i) = have.get(&g.key) {
                let mine = self.ext[i].clone();
                for k in 0..g.len {
                    if remap_obj(&o.objs[(g.start + k) as usize], &f) != self.objs[(mine.start + k) as usize] {
                        return Err(format!("扩展组 {} 与其他入口不一致（对象内容）", g.key));
                    }
                }
                if g.key.starts_with(MEMO_KEY) && their_memos.get(&g.key).map(|m| remap_memo(m, &f)) != mine_memos.get(&g.key).cloned() {
                    return Err(format!("扩展组 {} 与其他入口不一致（镜像缓存值）", g.key));
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
            if g.key.starts_with(MEMO_KEY) {
                let m = their_memos.get(&g.key).ok_or_else(|| format!("扩展组 {} 缺镜像缓存记录", g.key))?;
                self.mirror_memos.push(remap_memo(m, &f));
            }
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
        if let Some(w) = order.windows(2).find(|w| self.ext[w[0]].key == self.ext[w[1]].key) {
            return Err(format!("扩展组键重复 {}", self.ext[w[0]].key));
        }
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
        // 镜像缓存：随组重编号，按组键排序（与组次序一致）
        let mut memos: Vec<(String, IMemo)> = Vec::with_capacity(self.mirror_memos.len());
        for m in &self.mirror_memos {
            let m = remap_memo(m, &f);
            memos.push((self.memo_key_of(&m)?, m));
        }
        memos.sort_by(|a, b| a.0.cmp(&b.0));
        // 镜像缓存与 `f:` 组一一对应
        let fk: Vec<&str> = self.ext.iter().map(|g| g.key.as_str()).filter(|k| k.starts_with(MEMO_KEY)).collect();
        if !memos.iter().map(|e| e.0.as_str()).eq(fk.iter().copied()) {
            return Err("镜像缓存记录与镜像缓存组不对应".into());
        }
        self.mirror_memos = memos.into_iter().map(|e| e.1).collect();
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

    /// 镜像缓存组：镜像为 `m:` 组对象，值为组内对象；按键求并、重编号后记录随之重定位
    fn memo_img(groups: &[(&str, &[IObj])], memo_val: Option<IVal>) -> ImageData {
        let mut d = img(groups, &[]);
        let mirror = d.ext.iter().find(|g| g.key == "m:p/T").map(|g| g.start).unwrap();
        let at = d.ext.iter().find(|g| g.key.starts_with(MEMO_KEY)).map(|g| g.start).unwrap();
        d.mirror_memos.push(IMemo { mirror, decl: "q/C".into(), name: "memo".into(), val: memo_val.unwrap_or(IVal::R(at)) });
        d
    }

    fn mirror_obj(t: &str) -> IObj {
        IObj { mirror: Some(t.into()), ..obj("q/C", None) }
    }

    #[test]
    fn memo_groups_absorb_and_canonicalize() {
        let fk = memo_key("p/T", "q/C", "memo");
        // 程序一：先有镜像缓存（镜像组在其后），再有类组
        let a = memo_img(&[(fk.as_str(), &[obj("q/R", None)]), ("m:p/T", &[mirror_obj("p/T")]), ("c:p/A", &[obj("p/A", Some(0))])], None);
        // 程序二：只有镜像缓存，镜像组在前
        let b = memo_img(&[("m:p/T", &[mirror_obj("p/T")]), (fk.as_str(), &[obj("q/R", None)])], None);
        let mut x = a.clone();
        x.absorb(&b).unwrap();
        x.canonicalize().unwrap();
        let mut y = b.clone();
        y.absorb(&a).unwrap();
        y.canonicalize().unwrap();
        assert_eq!(x, y);
        // 组次序 c: < f: < m:；记录的镜像与值随重编号
        assert_eq!(x.ext.iter().map(|g| g.key.as_str()).collect::<Vec<_>>(), ["c:p/A", fk.as_str(), "m:p/T"]);
        assert_eq!(x.mirror_memos, vec![IMemo { mirror: 3, decl: "q/C".into(), name: "memo".into(), val: IVal::R(2) }]);
        let mut z = a.clone();
        z.canonicalize().unwrap();
        assert_eq!(z.mirror_memos, x.mirror_memos);
    }

    #[test]
    fn absorb_rejects_divergent_memo() {
        let fk = memo_key("p/T", "q/C", "memo");
        let a = memo_img(&[("m:p/T", &[mirror_obj("p/T")]), (fk.as_str(), &[obj("q/R", None)])], None);
        let b = memo_img(&[("m:p/T", &[mirror_obj("p/T")]), (fk.as_str(), &[obj("q/R", None)])], Some(IVal::N));
        let mut x = a.clone();
        assert!(x.absorb(&b).unwrap_err().contains("镜像缓存值"));
        // 记录与组不对应即拒绝
        let mut c = a.clone();
        c.mirror_memos.clear();
        assert!(c.canonicalize().is_err());
    }

    #[test]
    fn absorb_rejects_divergent_group() {
        let a = img(&[("c:p/B", &[obj("p/B", Some(0))])], &[]);
        let b = img(&[("c:p/B", &[obj("p/C", Some(0))])], &[]);
        let mut x = a.clone();
        assert!(x.absorb(&b).is_err());
    }
}
