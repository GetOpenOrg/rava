//! 构建期初始化扩展的导出：新完成的扩展类追加为映像扩展组（计划 §5.8，组的合并与规范化见 `image_ext.rs`）。
//!
//! - 类组 `c:<类>`：该类静态字段可达、归属该类的对象（广度优先，静态字段按名、实例字段按声明类与名），
//!   该类的静态字段、构建期初始化登记与重定位槽；
//! - 共享组：扩展期新建的驻留字符串（`s:<内容>`，字符串与其内容数组）与类镜像（`m:<类型>`）。
//!
//! 组内对象引用引导对象、其它组对象一律按映像编号；组键与程序无关，同键组内容由尝试隔离保证相同。

use std::collections::VecDeque;

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

    /// 扩展类 `done[from..]` 追加为扩展组；返回追加的类。失败时映像不变
    pub(super) fn ext_append(&mut self, cp: &ClassPath, d: &mut ImageData, from: usize) -> Result<Vec<String>, String> {
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
        let mut groups: Vec<(String, Option<Rc<str>>, Vec<u32>)> = Vec::new();
        let mut keys_done: HashSet<Rc<str>> = HashSet::default();
        let known = |ids: &HashMap<u32, u32>, o: u32| x.ids.contains_key(&o) || ids.contains_key(&o);
        for c in &classes {
            let mut members: Vec<u32> = Vec::new();
            let mut q: VecDeque<u32> = VecDeque::new();
            let mut shared_q: VecDeque<Rc<str>> = VecDeque::new();
            let visit = |o: u32, ids: &mut HashMap<u32, u32>, members: &mut Vec<u32>, q: &mut VecDeque<u32>, shared_q: &mut VecDeque<Rc<str>>| -> Result<(), String> {
                if known(ids, o) {
                    return Ok(());
                }
                if let Some(k) = x.shared.get(&o) {
                    if !keys_done.contains(k) && !shared_q.contains(k) {
                        shared_q.push_back(k.clone());
                    }
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
                    visit(o, &mut ids, &mut members, &mut q, &mut shared_q)?;
                }
            }
            while let Some(o) = q.pop_front() {
                for r in self.ext_refs(o) {
                    visit(r, &mut ids, &mut members, &mut q, &mut shared_q)?;
                }
            }
            next += members.len() as u32;
            groups.push((format!("{CLASS_KEY}{c}"), Some(c.clone()), members));
            // 共享组（可能引用更多共享对象）
            while let Some(k) = shared_q.pop_front() {
                if !keys_done.insert(k.clone()) {
                    continue;
                }
                let mut ms = by_key.get(&k).cloned().unwrap_or_default();
                ms.sort_unstable();
                for (i, &o) in ms.iter().enumerate() {
                    ids.insert(o, next + i as u32);
                }
                next += ms.len() as u32;
                for &o in &ms {
                    for r in self.ext_refs(o) {
                        if known(&ids, r) {
                            continue;
                        }
                        match x.shared.get(&r) {
                            Some(k2) if !keys_done.contains(k2) && !shared_q.contains(k2) => shared_q.push_back(k2.clone()),
                            Some(_) => {}
                            None => return Err(format!("共享对象 {k} 引用映像之外的对象 {}", self.heap[r as usize].ty)),
                        }
                    }
                }
                groups.push((k.to_string(), None, ms));
            }
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
                objs.push(IObj { ty: h.ty.to_string(), hash: self.ihash.get(&o).copied(), mirror: self.mirror_of.get(&o).map(|t| t.to_string()), deferred: None, host: None, placeholder: false, body });
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
