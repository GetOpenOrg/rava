//! 具体求值器的堆与字段访问：分配、数组、实例 / 静态 / VM 布局字段的读写与纪元检查、撤销日志、身份哈希。

use super::vm::*;
use super::*;

impl Vm {
    pub(super) fn alloc(&mut self, ty: &str, body: Body) -> u32 {
        let id = self.heap.len() as u32;
        self.heap.push(HObj { ty: Rc::from(ty), epoch: self.cur_epoch(), body });
        id
    }

    pub(super) fn new_array(&mut self, ty: &str, n: i32) -> R<u32> {
        if n < 0 {
            return implicit("size");
        }
        let comp = &ty[1..];
        Ok(self.alloc(ty, Body::Arr(vec![CV::zero(comp); n as usize])))
    }

    pub(super) fn ty(&self, o: u32) -> Rc<str> {
        self.heap[o as usize].ty.clone()
    }

    pub(super) fn arr(&self, o: u32) -> R<&Vec<CV>> {
        if self.boot {
            self.boot_arr_check(o)?;
            self.war_read(super::war::Loc::A(o));
        }
        match &self.heap[o as usize].body {
            Body::Arr(_) if self.heap[o as usize].epoch == 0 && self.image == 0 && !self.frozen.contains(&o) => {
                fail(format!("读取可变映像数组 {}", self.heap[o as usize].ty))
            }
            Body::Arr(v) => Ok(v),
            _ => fail("期望数组"),
        }
    }

    /// 类初始化中改写其开始前已有的映像对象（初始化失败时不能降级为静态不可读）
    pub(super) fn note_foreign(&mut self, o: u32) {
        if self.image > 0 && self.clinit_floor.last().is_some_and(|&f| (o as usize) < f) {
            self.foreign += 1;
        }
    }

    pub(super) fn arr_mut(&mut self, o: u32) -> R<&mut Vec<CV>> {
        if self.boot {
            self.boot_arr_write(o)?;
        }
        self.arr_store(o)
    }

    /// 数组写入（不记引导日志：调用方已按元素记）
    pub(super) fn arr_store(&mut self, o: u32) -> R<&mut Vec<CV>> {
        let ep = self.cur_epoch();
        self.note_foreign(o);
        let h = &mut self.heap[o as usize];
        if h.epoch != ep {
            return fail(format!("写入共享数组 {}", h.ty));
        }
        match &mut h.body {
            Body::Arr(v) => Ok(v),
            _ => fail("期望数组"),
        }
    }

    /// 登记残差步骤：引导档位（清单 `[concrete.boot] level`）与上一档位记录不同时先记档位
    pub(super) fn push_rec(&mut self, env: &Env, r: super::journal::Rec) {
        use super::journal::Rec;
        if let Some((d, n)) = env.cfg().boot.level.as_deref().and_then(|l| l.rsplit_once('.')) {
            let k = self.fkey(d, n);
            let cur = match self.statics.get(&k) {
                Some(CV::I(x)) => *x,
                _ => 0,
            };
            let last = self.bj.recs.iter().rev().find_map(|r| if let Rec::Level { level, .. } = r { Some(*level) } else { None });
            if last != Some(cur) {
                self.bj.recs.push(Rec::Level { key: k, level: cur });
            }
        }
        self.bj.recs.push(r);
    }

    pub(super) fn fkey(&mut self, decl: &str, name: &str) -> u32 {
        let k = format!("{decl}.{name}");
        if let Some(&n) = self.fkeys.get(&k) {
            return n;
        }
        let n = self.fkeys.len() as u32;
        self.fkeys.insert(k, n);
        self.fnames.push((Rc::from(decl), Rc::from(name)));
        n
    }

    /// 字段解析（JVMS §5.4.3.2，按引用缓存）
    pub(super) fn field_res(&mut self, env: &Env, f: &MemberRef) -> R<Rc<FRes>> {
        if let Some(r) = self.fres.get(f) {
            return r.clone().map_or_else(|| fail(format!("字段解析失败 {f}")), Ok);
        }
        let r = env.h().resolve_field(&f.owner, &f.name, &f.desc).map(|site| {
            let fd = site.field();
            let decl: Rc<str> = Rc::from(site.class.name.as_str());
            let key = self.fkey(&decl, &fd.name);
            let memo = env.cfg().memo_fields.contains(&format!("{decl}.{}", fd.name));
            let fin = fd.access & acc::FINAL != 0 || self.init_only(env, &site.class, fd);
            Rc::new(FRes {
                key,
                decl,
                name: fd.name.clone(),
                desc: fd.desc.clone(),
                fin,
                memo,
                constant: fd.constant_value.clone(),
            })
        });
        self.fres.insert(f.clone(), r.clone());
        r.map_or_else(|| fail(format!("字段解析失败 {f}")), Ok)
    }

    /// 实例字段读：映像对象（`<clinit>` 构造、程序其余部分可见）只许读 final 字段、内存缓存字段，以及
    /// 发布后不再改写的类型（`[concrete] stable_types`）的全部字段——经后者取到的映像数组同样冻结
    pub(super) fn get_field(&mut self, env: &Env, o: u32, fr: &FRes) -> R<CV> {
        if self.boot {
            self.boot_field_check(o, fr)?;
        }
        let image = self.image == 0 && self.heap[o as usize].epoch == 0;
        let stable = image && self.stable(env, o);
        if image && !fr.fin && !fr.memo && !stable {
            return fail(format!("读取映像对象可变字段 {}.{}", fr.decl, fr.name));
        }
        let v = match &self.heap[o as usize].body {
            Body::Inst(fs) => fs.iter().find(|(k, _)| *k == fr.key).map_or_else(|| CV::zero(&fr.desc), |(_, v)| *v),
            _ => return fail(format!("对非实例对象取字段 {}", fr.name)),
        };
        if let (true, CV::R(a)) = (stable, v) {
            if matches!(self.heap[a as usize].body, Body::Arr(_)) && self.heap[a as usize].epoch == 0 {
                self.frozen.insert(a);
            }
        }
        Ok(v)
    }

    /// 实例字段写入：映像对象只许写内存缓存字段（记撤销）
    pub(super) fn put_field(&mut self, o: u32, fr: &FRes, v: CV) -> R<()> {
        if self.boot {
            self.boot_field_write(o, fr)?;
        }
        let ep = self.cur_epoch();
        let shared = self.heap[o as usize].epoch != ep;
        if shared && !fr.memo {
            return fail(format!("写入共享对象字段 {}.{}", fr.decl, fr.name));
        }
        if !fr.memo {
            self.note_foreign(o);
        }
        let Body::Inst(fs) = &mut self.heap[o as usize].body else {
            return fail(format!("对非实例对象写字段 {}", fr.name));
        };
        let old = fs.iter().position(|(k, _)| *k == fr.key);
        if shared {
            self.undo.push((o, fr.key, old.map(|i| fs[i].1)));
        }
        match old {
            Some(i) => fs[i].1 = v,
            None => fs.push((fr.key, v)),
        }
        Ok(())
    }

    /// 按 VM 布局字段的语义名写（字符串字面量 / 镜像构造）
    pub(super) fn put_vm_field(&mut self, env: &Env, o: u32, what: &str, v: CV) -> R<()> {
        let Some(spec) = env.cfg().vm_fields.get(what) else { return fail(format!("清单缺 VM 布局字段 {what}")) };
        let (owner, name) = spec.rsplit_once('.').unwrap_or((spec, ""));
        let key = self.fkey(owner, name);
        if self.boot {
            self.jlog_field(o, key);
        }
        let Body::Inst(fs) = &mut self.heap[o as usize].body else { return fail("期望实例") };
        fs.retain(|(k, _)| *k != key);
        fs.push((key, v));
        Ok(())
    }

    pub(super) fn get_vm_field(&mut self, env: &Env, o: u32, what: &str) -> R<CV> {
        let Some(spec) = env.cfg().vm_fields.get(what) else { return fail(format!("清单缺 VM 布局字段 {what}")) };
        let (owner, name) = spec.rsplit_once('.').unwrap_or((spec, ""));
        let key = self.fkey(owner, name);
        match &self.heap[o as usize].body {
            Body::Inst(fs) => Ok(fs.iter().find(|(k, _)| *k == key).map_or(CV::N, |(_, v)| *v)),
            _ => fail("期望实例"),
        }
    }

    /// 静态字段写入：求值纪元内只许写内存缓存字段（记撤销）；`<clinit>` 映像构造期间照常写
    pub(super) fn put_static(&mut self, fr: &FRes, v: CV) -> R<()> {
        if self.image == 0 {
            if !fr.memo {
                return fail(format!("写入静态字段 {}.{}", fr.decl, fr.name));
            }
            let old = self.statics.get(&fr.key).copied();
            self.undo.push((u32::MAX, fr.key, old));
        }
        if self.boot {
            self.jlog_static(fr.key);
        }
        if let (true, true, CV::R(o)) = (self.image > 0, fr.fin, v) {
            if matches!(self.heap[o as usize].body, Body::Inst(_)) && self.heap[o as usize].epoch == 0 {
                self.image_roots.entry(o).or_insert_with(|| fr.mref());
            }
        }
        self.statics.insert(fr.key, v);
        Ok(())
    }

    /// 撤销本次求值对映像的内存缓存写入
    pub(super) fn rollback(&mut self) {
        while let Some((o, k, old)) = self.undo.pop() {
            if o == u32::MAX {
                match old {
                    Some(v) => self.statics.insert(k, v),
                    None => self.statics.remove(&k),
                };
                continue;
            }
            if let Body::Inst(fs) = &mut self.heap[o as usize].body {
                match old {
                    Some(v) => {
                        if let Some(e) = fs.iter_mut().find(|(x, _)| *x == k) {
                            e.1 = v;
                        }
                    }
                    None => fs.retain(|(x, _)| *x != k),
                }
            }
        }
    }

    /// 身份哈希（按首次查询顺序编号，确定）
    pub(super) fn identity_hash(&mut self, o: u32) -> i32 {
        let n = self.ihash.len() as i32;
        *self.ihash.entry(o).or_insert(0x1000 + n * 7919)
    }
}
