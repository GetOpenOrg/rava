//! 构建期初始化扩展的导出：新完成的扩展类追加为映像扩展组（计划 §5.8，组的合并与规范化见 `image_ext.rs`）。
//!
//! - 类组 `c:<类>`：该类静态字段可达、归属该类的对象（广度优先，静态字段按名、实例字段按声明类与名），
//!   该类的静态字段、构建期初始化登记与重定位槽；
//! - 共享组：扩展期新建的驻留字符串（`s:<内容>`，字符串与其内容数组）与类镜像（`m:<类型>`）。
//!
//! 组内对象引用引导对象、其它组对象一律按映像编号；组键与程序无关，同键组内容由尝试隔离保证相同。
//! 共享对象的身份哈希一律取键哈希（扩展期查询时的取值），与是否在尝试中查询过无关。
//!
//! 镜像缓存（U13，`engine/image_memo.rs`）所需而映像中没有的类镜像同经本求值器新建、按 `m:` 组导出
//! （[`Vm::ext_append`] 的 `roots`），与扩展期新建的镜像内容同一口径。

/// 一组待导出：（组键, 类组的类, 成员）
type Group = (String, Option<Rc<str>>, Vec<u32>);

/// 共享组导出（队列中的键依次成组；成员引用的其它共享对象入队）
struct Shared<'x> {
    x: &'x Ext,
    by_key: HashMap<Rc<str>, Vec<u32>>,
    keys_done: HashSet<Rc<str>>,
    q: VecDeque<Rc<str>>,
}

impl Shared<'_> {
    fn push(&mut self, k: &Rc<str>) {
        if !self.keys_done.contains(k) && !self.q.contains(k) {
            self.q.push_back(k.clone());
        }
    }

    fn drain(&mut self, vm: &Vm, ids: &mut HashMap<u32, u32>, next: &mut u32, groups: &mut Vec<Group>) -> Result<(), String> {
        while let Some(k) = self.q.pop_front() {
            if !self.keys_done.insert(k.clone()) {
                continue;
            }
            let mut ms = self.by_key.get(&k).cloned().unwrap_or_default();
            ms.sort_unstable();
            for (i, &o) in ms.iter().enumerate() {
                ids.insert(o, *next + i as u32);
            }
            *next += ms.len() as u32;
            for &o in &ms {
                for r in vm.ext_refs(o) {
                    if self.x.ids.contains_key(&r) || ids.contains_key(&r) {
                        continue;
                    }
                    match self.x.shared.get(&r) {
                        Some(k2) => {
                            let k2 = k2.clone();
                            self.push(&k2);
                        }
                        None => return Err(format!("共享对象 {k} 引用映像之外的对象 {}", vm.heap[r as usize].ty)),
                    }
                }
            }
            groups.push((k.to_string(), None, ms));
        }
        Ok(())
    }
}

use std::collections::VecDeque;

use super::ext_init::{fnv32, Ext};

use super::export::is_default;
use super::vm::*;
use super::*;
use crate::image::{IBody, IGroup, ILoc, IObj, IStep, IVal, ImageData};
use crate::image_ext::CLASS_KEY;

impl Vm {
    /// 本类的静态字段（按字段名）
    fn ext_statics(&self, c: &str) -> Vec<(Rc<str>, CV)> {
        let mut v: Vec<(Rc<str>, CV)> = self.statics.iter().filter(|(k, _)| &*self.fnames[**k as usize].0 == c).map(|(k, v)| (self.fnames[*k as usize].1.clone(), *v)).collect();
        v.sort_by(|a, b| a.0.cmp(&b.0));
        v
    }

    /// 对象的引用（字段按声明类与名、数组按序）
    fn ext_refs(&self, o: u32) -> Vec<u32> {
        match &self.heap[o as usize].body {
            Body::Inst(fs) => {
                let mut fs: Vec<(&(Rc<str>, Rc<str>), CV)> = fs.iter().map(|(k, v)| (&self.fnames[*k as usize], *v)).collect();
                fs.sort_by(|a, b| a.0.cmp(b.0));
                fs.into_iter().filter_map(|(_, v)| if let CV::R(r) = v { Some(r) } else { None }).collect()
            }
            Body::Arr(a) => a.iter().filter_map(|v| if let CV::R(r) = v { Some(*r) } else { None }).collect(),
            Body::Lam(_) => Vec::new(),
        }
    }

    /// 扩展类 `done[from..]` 与共享对象 `roots`（尚未导出的类镜像等）追加为扩展组；返回追加的类。失败时映像不变
    pub(super) fn ext_append(&mut self, cp: &ClassPath, d: &mut ImageData, from: usize, roots: &[u32]) -> Result<Vec<String>, String> {
        let x = self.ext.as_deref().ok_or("非扩展期")?;
        let classes: Vec<Rc<str>> = x.done.get(from..).unwrap_or_default().to_vec();
        let batch: HashSet<&str> = classes.iter().map(|c| &**c).collect();
        let mut by_key: HashMap<Rc<str>, Vec<u32>> = HashMap::default();
        for (&o, k) in &x.shared {
            by_key.entry(k.clone()).or_default().push(o);
        }
        // 第一遍：分组与编号
        let mut ids: HashMap<u32, u32> = HashMap::default();
        let mut next = d.objs.len() as u32;
        let mut groups: Vec<Group> = Vec::new();
        let mut sh = Shared { x, by_key, keys_done: HashSet::default(), q: VecDeque::new() };
        let known = |ids: &HashMap<u32, u32>, o: u32| x.ids.contains_key(&o) || ids.contains_key(&o);
        for c in &classes {
            let mut members: Vec<u32> = Vec::new();
            let mut q: VecDeque<u32> = VecDeque::new();
            let visit = |o: u32, ids: &mut HashMap<u32, u32>, members: &mut Vec<u32>, q: &mut VecDeque<u32>, sh: &mut Shared| -> Result<(), String> {
                if known(ids, o) {
                    return Ok(());
                }
                if let Some(k) = x.shared.get(&o) {
                    sh.push(k);
                    return Ok(());
                }
                match x.owner.get(&o) {
                    Some(oc) if oc == c => {
                        ids.insert(o, next + members.len() as u32);
                        members.push(o);
                        q.push_back(o);
                        Ok(())
                    }
                    Some(oc) if batch.contains(&**oc) => Ok(()),
                    _ => Err(format!("扩展类 {c} 引用映像之外的对象 {}", self.heap[o as usize].ty)),
                }
            };
            for (_, v) in self.ext_statics(c) {
                if let CV::R(o) = v {
                    visit(o, &mut ids, &mut members, &mut q, &mut sh)?;
                }
            }
            while let Some(o) = q.pop_front() {
                for r in self.ext_refs(o) {
                    visit(r, &mut ids, &mut members, &mut q, &mut sh)?;
                }
            }
            next += members.len() as u32;
            groups.push((format!("{CLASS_KEY}{c}"), Some(c.clone()), members));
            // 共享组（可能引用更多共享对象）
            sh.drain(self, &mut ids, &mut next, &mut groups)?;
        }
        for &o in roots {
            if known(&ids, o) {
                continue;
            }
            let k = x.shared.get(&o).ok_or_else(|| format!("追加的根对象 {} 不是共享对象", self.heap[o as usize].ty))?;
            sh.push(k);
            sh.drain(self, &mut ids, &mut next, &mut groups)?;
        }
        // 第二遍：转换
        let id = |o: u32| -> Result<u32, String> { x.ids.get(&o).or_else(|| ids.get(&o)).copied().ok_or_else(|| format!("扩展组引用未编号对象 {}", self.heap[o as usize].ty)) };
        let val = |v: CV| -> Result<IVal, String> {
            Ok(match v {
                CV::I(a) => IVal::I(a),
                CV::J(a) => IVal::J(a),
                CV::F(a) => IVal::F(a.to_bits()),
                CV::D(a) => IVal::D(a.to_bits()),
                CV::N => IVal::N,
                CV::R(o) => IVal::R(id(o)?),
                CV::T(..) => return Err("扩展组含宿主标量".into()),
            })
        };
        let mut objs: Vec<IObj> = Vec::new();
        let mut steps: Vec<IStep> = Vec::new();
        let mut ext: Vec<IGroup> = Vec::new();
        let mut statics: Vec<(String, String, IVal)> = Vec::new();
        let mut strings: Vec<u32> = Vec::new();
        let base = d.objs.len() as u32;
        for (key, class, ms) in &groups {
            let start = base + objs.len() as u32;
            let s0 = steps.len();
            if let Some(c) = class {
                for (name, v) in self.ext_statics(c) {
                    if let Some(reloc) = super::unsafe_ops::reloc_of(self, v) {
                        steps.push(IStep::Reloc { loc: ILoc::Static(c.to_string(), name.to_string()), reloc });
                    } else if !is_default(v) {
                        statics.push((c.to_string(), name.to_string(), val(v)?));
                    }
                }
            }
            for &o in ms {
                let i = id(o)?;
                let h = &self.heap[o as usize];
                if !x.at_home(cp, &h.ty) {
                    return Err(format!("扩展组含根模块之外的类型 {}", h.ty));
                }
                let body = match &h.body {
                    Body::Inst(fs) => {
                        let mut out = Vec::new();
                        for &(k, v) in fs {
                            let (dc, n) = &self.fnames[k as usize];
                            if let Some(reloc) = super::unsafe_ops::reloc_of(self, v) {
                                steps.push(IStep::Reloc { loc: ILoc::Field(i, dc.to_string(), n.to_string()), reloc });
                            } else if !is_default(v) {
                                out.push((dc.to_string(), n.to_string(), val(v)?));
                            }
                        }
                        out.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));
                        IBody::Inst(out)
                    }
                    Body::Arr(a) => {
                        let mut es = Vec::with_capacity(a.len());
                        for (j, &v) in a.iter().enumerate() {
                            if let Some(reloc) = super::unsafe_ops::reloc_of(self, v) {
                                steps.push(IStep::Reloc { loc: ILoc::Elem(i, j as u32), reloc });
                                es.push(IVal::J(0));
                            } else {
                                es.push(val(v)?);
                            }
                        }
                        IBody::Arr(es)
                    }
                    Body::Lam(l) => return Err(format!("扩展组含 lambda 对象（{} ← {}）", l.iface, l.imp.member)),
                };
                if class.is_none() && matches!(h.body, Body::Inst(_)) && self.mirror_of.get(&o).is_none() {
                    strings.push(i);
                }
                // 共享对象的身份哈希取键哈希（与扩展期查询同值）：同键组内容与是否查询过、何时查询无关
                let hash = if class.is_none() { Some(fnv32(key)) } else { self.ihash.get(&o).copied() };
                objs.push(IObj { ty: h.ty.to_string(), hash, mirror: self.mirror_of.get(&o).map(|t| t.to_string()), deferred: None, host: None, placeholder: false, body });
            }
            ext.push(IGroup { key: key.clone(), start, len: ms.len() as u32, nsteps: (steps.len() - s0) as u32 });
        }
        // 提交
        d.objs.extend(objs);
        d.steps.extend(steps);
        d.ext.extend(ext);
        d.statics.extend(statics);
        d.strings.extend(strings);
        d.build_time.extend(classes.iter().map(|c| c.to_string()));
        let x = self.ext.as_deref_mut().ok_or("非扩展期")?;
        x.ids.extend(ids);
        Ok(classes.iter().map(|c| c.to_string()).collect())
    }
}
