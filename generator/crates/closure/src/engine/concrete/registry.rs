//! 闭包期求值 VM 的运行期前置状态：一次写入的登记表静态字段（`[concrete] registry_statics`）与
//! 取引导映像值的静态字段（`[concrete] image_statics`）。
//!
//! 二者在运行期首次使用前已定、此后不再改写：登记表字段由登记者类的 `<clinit>` 写一次（运行期登记者在
//! 构建期已初始化），映像字段是构建期初始化类里只在为 null 时惰性写入 / 单调置位的字段，映像中已是终值。
//! 求值读到的是运行期同一个值，执行的是运行期同一条已初始化快路径，结果与求值次序无关。


use crate::image::{IBody, IVal, ImageData};

use super::vm::*;
use super::*;

impl Vm {
    /// 闭包期求值 VM：导入映像静态字段的值，再按类名序初始化全部登记者类
    pub(super) fn closure_vm(env: &Env, img: Option<(&ImageData, &HashMap<(String, String), IVal>)>) -> Box<Vm> {
        let mut vm = Box::new(Vm::new());
        if let Some((data, statics)) = img {
            let mut seen = HashMap::default();
            for f in &env.cfg().image_statics {
                let Some((decl, name)) = f.rsplit_once('.') else { continue };
                let Some(v) = statics.get(&(decl.to_string(), name.to_string())) else { continue };
                // 导入失败（宿主相关 / 占位 / 重算槽）：该字段不登记，读取即失败回退抽象调用边
                let Ok(cv) = vm.import_ival(env, data, *v, &mut seen) else { continue };
                let key = vm.fkey(decl, name);
                if let CV::R(o) = cv {
                    let desc = env.h().class(decl).and_then(|c| c.fields.iter().find(|x| *x.name == *name).map(|x| x.desc.clone()));
                    if let (Some(desc), Body::Inst(_)) = (desc, &vm.heap[o as usize].body) {
                        vm.image_roots.entry(o).or_insert_with(|| MemberRef { owner: decl.to_string(), name: name.to_string(), desc });
                    }
                }
                vm.statics.insert(key, cv);
                vm.img_statics.insert(key);
            }
        }
        let mut regs: Vec<&String> = env.cfg().registry_statics.values().collect();
        regs.sort();
        regs.dedup();
        for r in regs {
            // 登记者初始化不可建模：其登记表字段读取即失败（见 `pinned_static`）
            let _ = vm.ensure_init(env, r);
        }
        vm
    }

    /// 映像值按值导入求值 VM 的映像纪元（对象图逐个复制，类镜像取本 VM 的镜像）
    fn import_ival(&mut self, env: &Env, data: &ImageData, v: IVal, seen: &mut HashMap<u32, u32>) -> R<CV> {
        let i = match v {
            IVal::I(x) => return Ok(CV::I(x)),
            IVal::J(x) => return Ok(CV::J(x)),
            IVal::F(b) => return Ok(CV::F(f32::from_bits(b))),
            IVal::D(b) => return Ok(CV::D(f64::from_bits(b))),
            IVal::N => return Ok(CV::N),
            IVal::T(..) => return fail("映像静态值含启动重算槽"),
            IVal::R(i) => i,
        };
        if let Some(&o) = seen.get(&i) {
            return Ok(CV::R(o));
        }
        let Some(io) = data.objs.get(i as usize) else { return fail(format!("映像对象下标越界 #{i}")) };
        if let Some(t) = &io.mirror {
            let o = self.mirror(env, t)?;
            seen.insert(i, o);
            return Ok(CV::R(o));
        }
        if io.placeholder || io.deferred.is_some() || io.host.is_some() {
            return fail(format!("映像对象 #{i} 的值运行期才定"));
        }
        let o = self.alloc(&io.ty, Body::Inst(Vec::new()));
        seen.insert(i, o);
        if let Some(h) = io.hash {
            self.ihash.insert(o, h);
        }
        let body = match &io.body {
            IBody::Inst(fs) => {
                let mut out = Vec::with_capacity(fs.len());
                for (d, n, fv) in fs {
                    let cv = self.import_ival(env, data, *fv, seen)?;
                    out.push((self.fkey(d, n), cv));
                }
                Body::Inst(out)
            }
            IBody::Arr(es) => Body::Arr(es.iter().map(|e| self.import_ival(env, data, *e, seen)).collect::<R<Vec<CV>>>()?),
        };
        self.heap[o as usize].body = body;
        Ok(CV::R(o))
    }

    /// 取映像值 / 登记表静态字段的读取
    pub(super) fn pinned_static(&mut self, env: &Env, fr: &FRes) -> R<CV> {
        if fr.imaged {
            return match self.statics.get(&fr.key) {
                Some(v) if self.img_statics.contains(&fr.key) => Ok(*v),
                _ => fail(format!("引导映像未给出静态字段 {}.{}", fr.decl, fr.name)),
            };
        }
        let Some(reg) = fr.registry.clone() else { return fail("非登记表静态字段") };
        let cur = |vm: &Self| vm.statics.get(&fr.key).copied().unwrap_or(CV::N);
        let mut v = cur(self);
        if matches!(v, CV::N) && !self.init.contains_key(&*reg) {
            self.ensure_init(env, &reg)?;
            v = cur(self);
        }
        if self.opaque.contains(&reg) || matches!(self.init.get(&*reg), Some(Init::Failed(_))) {
            return fail(format!("登记者类初始化不可建模 {reg}（登记表静态字段 {}.{}）", fr.decl, fr.name));
        }
        if matches!(v, CV::N) {
            return fail(format!("登记表静态字段未登记 {}.{}", fr.decl, fr.name));
        }
        Ok(v)
    }

    /// 取映像值 / 登记表静态字段的写入：前者不可写；后者只由登记者 `<clinit>` 写一次
    pub(super) fn pinned_put_check(&self, fr: &FRes) -> R<()> {
        let Some(reg) = &fr.registry else { return fail(format!("写入取映像值的静态字段 {}.{}", fr.decl, fr.name)) };
        if self.image == 0 || !matches!(self.init.get(&**reg), Some(Init::Running)) {
            return fail(format!("登记表静态字段 {}.{} 只由登记者 {reg} 的类初始化写入", fr.decl, fr.name));
        }
        if !matches!(self.statics.get(&fr.key), None | Some(CV::N)) {
            return fail(format!("登记表静态字段重复写入 {}.{}", fr.decl, fr.name));
        }
        Ok(())
    }
}
