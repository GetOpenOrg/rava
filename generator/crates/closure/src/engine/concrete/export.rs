//! 引导映像导出：解释器堆 → [`crate::image::ImageData`]（与堆下标无关的规范编号）。
//!
//! 对象编号取规范根次序的广度优先首次到达次序：静态字段（按声明类、字段名）→ VM 构造对象 → 类镜像（按类型名）
//! → 驻留字符串（按内容）→ 残差记录的实参 / 占位对象 / 区段局部变量（按序）。
//! 污点表达式按首次引用次序重编号；启动序列 = 重算槽（构建期写入次序）+ 残差记录。

use std::collections::VecDeque;

use super::journal::Rec;
use super::taint::TOp;
use super::vm::*;
use super::war::Loc;
use super::*;
use crate::image::{IBody, IExpr, ILoc, IObj, IStep, IVal, ImageData};

struct Ex<'v> {
    vm: &'v Vm,
    ids: HashMap<u32, u32>,
    order: Vec<u32>,
    q: VecDeque<u32>,
    texprs: HashMap<u32, u32>,
    exprs: Vec<IExpr>,
}

impl Ex<'_> {
    fn obj(&mut self, o: u32) -> u32 {
        if let Some(&i) = self.ids.get(&o) {
            return i;
        }
        let i = self.order.len() as u32;
        self.ids.insert(o, i);
        self.order.push(o);
        self.q.push_back(o);
        i
    }

    /// 构建期值：对象引用只登记（编号次序由广度优先保证），污点重编号
    fn val(&mut self, v: CV) -> IVal {
        match v {
            CV::I(x) => IVal::I(x),
            CV::J(x) => IVal::J(x),
            CV::F(x) => IVal::F(x.to_bits()),
            CV::D(x) => IVal::D(x.to_bits()),
            CV::N => IVal::N,
            CV::R(o) => IVal::R(self.obj(o)),
            CV::T(id, k) => IVal::T(self.expr(id), k),
        }
    }

    fn expr(&mut self, id: u32) -> u32 {
        if let Some(&i) = self.texprs.get(&id) {
            return i;
        }
        let e = match &self.vm.bj.taint.exprs[id as usize].op {
            TOp::Src { native, args } => {
                let args = args.iter().map(|&a| self.val(a)).collect();
                IExpr::Src { native: native.to_string(), args }
            }
            TOp::Un(op, x) => IExpr::Un(*op, self.val(*x)),
            TOp::Bin(op, a, b) => {
                let (a, b) = (self.val(*a), self.val(*b));
                IExpr::Bin(*op, a, b)
            }
            TOp::Sel { cmp, a, b, t, f } => {
                let (a, b, t, f) = (self.val(*a), self.val(*b), self.val(*t), self.val(*f));
                IExpr::Sel { cmp: *cmp, a, b, t, f }
            }
        };
        let i = self.exprs.len() as u32;
        self.texprs.insert(id, i);
        self.exprs.push(e);
        i
    }

    /// 广度优先：只登记引用，不展开内容（内容在第二遍按编号次序导出）
    fn drain(&mut self) -> Result<(), String> {
        while let Some(o) = self.q.pop_front() {
            let vm = self.vm;
            match &vm.heap[o as usize].body {
                Body::Inst(fs) => {
                    let mut fs: Vec<(&(Rc<str>, Rc<str>), CV)> = fs.iter().map(|(k, v)| (&vm.fnames[*k as usize], *v)).collect();
                    fs.sort_by(|a, b| a.0.cmp(b.0));
                    for (_, v) in fs {
                        if let CV::R(r) = v {
                            self.obj(r);
                        }
                    }
                }
                Body::Arr(a) => {
                    for v in a {
                        if let CV::R(r) = v {
                            self.obj(*r);
                        }
                    }
                }
                Body::Lam(l) => return Err(format!("映像含 lambda 对象（{} ← {}）", l.iface, l.imp.member)),
            }
        }
        Ok(())
    }
}

fn is_default(v: CV) -> bool {
    match v {
        CV::I(0) | CV::J(0) | CV::N => true,
        CV::F(x) => x.to_bits() == 0,
        CV::D(x) => x.to_bits() == 0,
        _ => false,
    }
}

/// 导出映像（含 lambda 对象时失败）；`current_thread` 为 VM 初始线程的堆下标
pub(super) fn export(vm: &Vm, current_thread: Option<u32>) -> Result<ImageData, String> {
    let mut x = Ex { vm, ids: HashMap::default(), order: Vec::new(), q: VecDeque::new(), texprs: HashMap::default(), exprs: Vec::new() };
    let mut statics: Vec<(&(Rc<str>, Rc<str>), u32, CV)> = vm.statics.iter().map(|(k, v)| (&vm.fnames[*k as usize], *k, *v)).collect();
    statics.sort_by(|a, b| a.0.cmp(b.0));
    for (_, _, v) in &statics {
        if let CV::R(o) = v {
            x.obj(*o);
        }
    }
    x.drain()?;
    for &o in &vm.boot_objs {
        x.obj(o);
    }
    x.drain()?;
    let mut mirrors: Vec<(&Rc<str>, u32)> = vm.mirrors.iter().map(|(t, o)| (t, *o)).collect();
    mirrors.sort();
    for (_, o) in &mirrors {
        x.obj(*o);
    }
    x.drain()?;
    let mut strings: Vec<(&Vec<u16>, u32)> = vm.strings.iter().map(|(s, o)| (s, *o)).collect();
    strings.sort();
    let strings: Vec<u32> = strings.into_iter().map(|(_, o)| x.obj(o)).collect();
    x.drain()?;
    for r in &vm.bj.recs {
        match r {
            Rec::Call { args, ph, .. } | Rec::Native { args, ph, .. } => {
                args.iter().for_each(|v| if let CV::R(o) = v { x.obj(*o); });
                ph.iter().for_each(|&o| { x.obj(o); });
            }
            Rec::Read { ph, .. } => {
                x.obj(*ph);
            }
            Rec::Region { locals, .. } => locals.iter().for_each(|v| if let CV::R(o) = v { x.obj(*o); }),
            Rec::RuntimeInit { .. } | Rec::Level(_) => {}
        }
        x.drain()?;
    }
    // 重算槽（构建期写入次序）
    let wt = |l: Loc| vm.bj.war.wtime.get(&l).copied().unwrap_or(0);
    let mut slots: Vec<(u64, ILoc, u32)> = Vec::new();
    let mut d = ImageData::default();
    // 重定位槽（位置次序：静态字段、对象字段、数组元素）：映像取零值
    let mut relocs: Vec<IStep> = Vec::new();
    for ((decl, name), k, v) in &statics {
        if let Some(reloc) = super::unsafe_ops::reloc_of(vm, *v) {
            relocs.push(IStep::Reloc { loc: ILoc::Static(decl.to_string(), name.to_string()), reloc });
            continue;
        }
        let v = x.val(*v);
        if let IVal::T(e, _) = v {
            slots.push((wt(Loc::S(*k)), ILoc::Static(decl.to_string(), name.to_string()), e));
        }
        if !matches!(v, IVal::T(..)) && !is_default(vm.statics[k]) {
            d.statics.push((decl.to_string(), name.to_string(), v));
        }
    }
    let order = x.order.clone();
    for (i, &o) in order.iter().enumerate() {
        let h = &vm.heap[o as usize];
        let body = match &h.body {
            Body::Inst(fs) => {
                let mut out: Vec<(String, String, IVal)> = Vec::new();
                for &(k, v) in fs {
                    let (dc, n) = &vm.fnames[k as usize];
                    if let Some(reloc) = super::unsafe_ops::reloc_of(vm, v) {
                        relocs.push(IStep::Reloc { loc: ILoc::Field(i as u32, dc.to_string(), n.to_string()), reloc });
                        continue;
                    }
                    let iv = x.val(v);
                    if let IVal::T(e, _) = iv {
                        slots.push((wt(Loc::F(o, k)), ILoc::Field(i as u32, dc.to_string(), n.to_string()), e));
                    } else if !is_default(v) {
                        out.push((dc.to_string(), n.to_string(), iv));
                    }
                }
                out.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));
                IBody::Inst(out)
            }
            Body::Arr(a) => {
                let mut es = Vec::with_capacity(a.len());
                for (j, &v) in a.iter().enumerate() {
                    if let Some(reloc) = super::unsafe_ops::reloc_of(vm, v) {
                        relocs.push(IStep::Reloc { loc: ILoc::Elem(i as u32, j as u32), reloc });
                        es.push(IVal::J(0));
                        continue;
                    }
                    let iv = x.val(v);
                    if let IVal::T(e, _) = iv {
                        slots.push((wt(Loc::A(o)), ILoc::Elem(i as u32, j as u32), e));
                    }
                    es.push(iv);
                }
                IBody::Arr(es)
            }
            Body::Lam(_) => unreachable!("drain 已拒绝 lambda"),
        };
        d.objs.push(IObj {
            ty: h.ty.to_string(),
            hash: vm.ihash.get(&o).copied(),
            mirror: vm.mirror_of.get(&o).map(|t| t.to_string()),
            deferred: vm.deferred.get(&o).map(|w| w.to_string()),
            host: vm.host_src.get(&o).map(|(n, i)| (n.to_string(), *i)),
            placeholder: vm.bj.placeholders.contains(&o),
            body,
        });
    }
    // 槽位次序：写入时刻，同刻按位置
    slots.sort_by(|a, b| (a.0, format!("{:?}", a.1)).cmp(&(b.0, format!("{:?}", b.1))));
    d.steps.extend(relocs);
    d.steps.extend(slots.into_iter().map(|(_, loc, expr)| IStep::Recompute { loc, expr }));
    for r in &vm.bj.recs {
        d.steps.push(match r {
            Rec::RuntimeInit { class, .. } => IStep::RuntimeInit { class: class.to_string() },
            Rec::Call { phase, off, callee, args, ph, .. } => {
                IStep::Call { phase: phase.to_string(), off: *off, callee: callee.to_string(), args: args.iter().map(|&v| x.val(v)).collect(), ph: ph.map(|o| x.ids[&o]) }
            }
            Rec::Native { callee, args, ph } => IStep::Native { callee: callee.to_string(), args: args.iter().map(|&v| x.val(v)).collect(), ph: ph.map(|o| x.ids[&o]) },
            Rec::Read { decl, name, ph } => IStep::Read { decl: decl.to_string(), name: name.to_string(), ph: x.ids[ph] },
            Rec::Region { phase, start, end, locals, .. } => IStep::Region { phase: phase.to_string(), start: *start, end: *end, locals: locals.iter().map(|&v| x.val(v)).collect() },
            Rec::Level(l) => IStep::Level(*l),
        });
    }
    d.strings = strings;
    let mut bt: Vec<String> = vm.done_log.iter().filter(|c| !vm.opaque.contains(*c)).map(|c| c.to_string()).collect();
    bt.sort();
    bt.dedup();
    d.build_time = bt;
    d.cells = vm.cells.iter().map(|(n, v)| (n.to_string(), *v)).collect();
    d.cells.sort();
    d.current_thread = current_thread.map(|o| x.ids[&o]);
    if x.order.len() != d.objs.len() {
        return Err(format!("污点表达式引用了映像根不可达的对象 {} 个", x.order.len() - d.objs.len()));
    }
    d.exprs = x.exprs;
    Ok(d)
}
