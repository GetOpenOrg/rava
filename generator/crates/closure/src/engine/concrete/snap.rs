//! 求值结果的对象图快照：与解释器堆解耦，供物化（`apply.rs`）使用。
//!
//! 求值纪元内分配的对象按字段 / 元素展开；字符串、类镜像按值；映像对象（`<clinit>` 构造）只记类型——
//! 其抽象值来自抽象分析对 `<clinit>` 的建模，物化时只有非容器形态的类可按类型代表（见 `apply.rs`）；
//! lambda 对象按函数式接口、实现句柄与捕获值展开（捕获值记为元素）；映像数组不可物化，快照失败。

use super::vm::*;
use super::*;

#[derive(Clone, Debug)]
pub(in crate::engine) enum MV {
    /// 基本类型值与 null（常量格投影）
    Prim(Put),
    Str,
    Mirror(Rc<str>),
    /// 快照对象下标
    Obj(usize),
    /// 映像实例对象（类型）
    Image(Rc<str>),
}

#[derive(Debug)]
pub(in crate::engine) struct MObj {
    pub ty: Rc<str>,
    pub arr: bool,
    pub fields: Vec<(MemberRef, MV)>,
    /// 数组元素 / lambda 的捕获值
    pub elems: Vec<MV>,
    /// lambda 对象（`ty` 为函数式接口）
    pub lam: Option<Rc<Lam>>,
}

pub(super) struct Snap<'s, 'e, 'a> {
    vm: &'s Vm,
    env: &'s Env<'e, 'a>,
    objs: &'s mut Vec<MObj>,
    seen: HashMap<u32, usize>,
}

impl<'s, 'e, 'a> Snap<'s, 'e, 'a> {
    pub(super) fn new(vm: &'s Vm, env: &'s Env<'e, 'a>, objs: &'s mut Vec<MObj>) -> Self {
        Snap { vm, env, objs, seen: HashMap::default() }
    }

    pub(super) fn value(&mut self, v: CV) -> R<MV> {
        let o = match v {
            CV::R(o) => o,
            CV::I(x) => return Ok(MV::Prim(Put::Int(x))),
            CV::J(x) => return Ok(MV::Prim(Put::Long(x))),
            CV::N => return Ok(MV::Prim(Put::Null)),
            CV::F(_) | CV::D(_) => return Ok(MV::Prim(Put::Other)),
        };
        if let Some(&i) = self.seen.get(&o) {
            return Ok(MV::Obj(i));
        }
        let h = &self.vm.heap[o as usize];
        if &*h.ty == STRING {
            return Ok(MV::Str);
        }
        if let Some(t) = self.vm.mirror_of.get(&o) {
            return Ok(MV::Mirror(t.clone()));
        }
        let ty = h.ty.clone();
        match &h.body {
            Body::Lam(l) => {
                let l = l.clone();
                let i = self.push(o, ty, false);
                let mut out = Vec::with_capacity(l.captured.len());
                for &e in &l.captured {
                    out.push(self.value(e)?);
                }
                self.objs[i].elems = out;
                self.objs[i].lam = Some(l);
                Ok(MV::Obj(i))
            }
            Body::Arr(_) if h.epoch == 0 => fail(format!("结果引用映像数组 {ty}")),
            Body::Inst(_) if h.epoch == 0 => Ok(MV::Image(ty)),
            Body::Arr(es) => {
                let i = self.push(o, ty, true);
                let es = es.clone();
                let mut out = Vec::with_capacity(es.len());
                for e in es {
                    out.push(self.value(e)?);
                }
                self.objs[i].elems = out;
                Ok(MV::Obj(i))
            }
            Body::Inst(fs) => {
                let i = self.push(o, ty.clone(), false);
                let fs = fs.clone();
                let mut out = Vec::with_capacity(fs.len());
                for (k, v) in fs {
                    let (decl, name) = self.vm.fnames[k as usize].clone();
                    let desc = self.env.h().class(&decl).and_then(|c| c.fields.iter().find(|f| *f.name == *name).map(|f| f.desc.clone()));
                    let Some(desc) = desc else { return fail(format!("字段缺失 {decl}.{name}")) };
                    let mv = self.value(v)?;
                    out.push((MemberRef { owner: decl.to_string(), name: name.to_string(), desc }, mv));
                }
                // 未写的实例字段取缺省值：物化对象逐字段给出全部值（类型不经抽象分配，缺省值不另行并入）
                let mut cur = self.env.h().class(&ty);
                while let Some(cf) = cur {
                    for f in cf.fields.iter().filter(|f| !f.is_static()) {
                        if !out.iter().any(|(k, _)| k.owner == cf.name && k.name == f.name) {
                            let mv = self.value(CV::zero(&f.desc))?;
                            out.push((MemberRef { owner: cf.name.clone(), name: f.name.clone(), desc: f.desc.clone() }, mv));
                        }
                    }
                    cur = cf.super_name.as_deref().and_then(|s| self.env.h().class(s));
                }
                self.objs[i].fields = out;
                Ok(MV::Obj(i))
            }
        }
    }

    fn push(&mut self, o: u32, ty: Rc<str>, arr: bool) -> usize {
        let i = self.objs.len();
        self.objs.push(MObj { ty, arr, fields: Vec::new(), elems: Vec::new(), lam: None });
        self.seen.insert(o, i);
        i
    }
}
