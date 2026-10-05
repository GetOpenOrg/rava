//! 引导求值的写入日志、回滚与脏位置（残差化的基础设施，计划 §3.2）。
//!
//! 引导求值在永久映像纪元执行，全部写入直接进映像。宿主相关值（延迟值）参与求值时，求值器把一段执行
//! 撤回、改为运行期重放：根帧的一次调用（残差调用）、根帧的一条语句区段（残差区段）或一个类的
//! `<clinit>`（运行期初始化）。撤回靠写入日志：
//!
//! - 标记（[`Mark`]）压栈时记下日志长度与堆大小；之后对**标记前已有对象**的写入、全部静态写入、
//!   类初始化状态与延迟登记的变化记入日志（标记后新建的对象撤回后不可达，不必记）；
//! - 回滚把日志倒放到标记处，并把撤回的位置记为**脏位置**：运行期重放会重写它们，构建期此后读到
//!   即「延迟值参与求值」，按同一规则继续残差化或使读者所在的类转为运行期初始化；
//! - 撤回范围内发生过 VM 侧登记（模块表等）不可撤回，即构建失败。
//!
//! 残差记录（[`Rec`]）同样按标记截断：外层撤回时内层的残差记录一并撤回（被外层吸收）。
//! 撤回后回到未初始化状态的类，其静态字段不记脏：日后的 `<clinit>`（构建期或运行期）重新定义它们。

use super::vm::*;
use super::*;

pub(super) enum JEnt {
    Static(u32, Option<CV>),
    Field(u32, u32, Option<CV>),
    Elem(u32, usize, CV),
    Arr(u32, Vec<CV>),
    Init(Rc<str>, Option<Init>),
    Opaque(Rc<str>),
    Deferred(u32),
    Placeholder(u32),
    Cell(usize, i64),
}

#[derive(Clone, Copy)]
pub(super) struct Mark {
    depth: usize,
    j: usize,
    heap: usize,
    done: usize,
    recs: usize,
    vm: u64,
}

/// 引导映像的运行期部分：运行期初始化的类、残差调用、残差区段
#[derive(Clone, Debug)]
pub(super) enum Rec {
    /// 类转为运行期初始化（原因）
    RuntimeInit { class: Rc<str>, why: String },
    /// 根帧调用 `phase@off` → `callee(args)` 运行期重放；结果（引用）由占位对象 `ph` 代表
    Call { phase: MemberRef, off: u32, callee: MemberRef, args: Vec<CV>, ph: Option<u32>, why: String },
    /// 运行期副作用 native（线程启动、信号、OS 环境）/ 结果依赖宿主的调用（占位对象 `ph`）：运行期按序重放
    Native { callee: MemberRef, args: Vec<CV>, ph: Option<u32> },
    /// 根帧区段 `[start, end)` 运行期执行（局部变量取区段入口的值）
    Region { phase: MemberRef, start: u32, end: Option<u32>, locals: Vec<CV>, why: String },
}

#[derive(Default)]
pub(super) struct Journal {
    pub ents: Vec<JEnt>,
    marks: Vec<Mark>,
    pub dirty_static: HashSet<u32>,
    pub dirty_field: HashSet<(u32, u32)>,
    pub dirty_arr: HashSet<u32>,
    pub dirty_cells: HashSet<usize>,
    /// 残差调用结果的占位对象：可存放、可传递；判空、比较身份、分派、取类型即延迟值参与求值
    pub placeholders: HashSet<u32>,
    pub recs: Vec<Rec>,
    /// VM 侧登记次数（模块表、构建期输出等不可撤回的效果）
    pub vm_effects: u64,
    /// 宿主标量的（native, 调用方）：构建期取零值（操作 `host_scalar`），第 2 步改为污点值与重算槽
    pub host_scalars: BTreeSet<(String, String)>,
}

impl Vm {
    pub(super) fn jmark(&mut self) -> Mark {
        let m = Mark {
            depth: self.bj.marks.len(),
            j: self.bj.ents.len(),
            heap: self.heap.len(),
            done: self.done_log.len(),
            recs: self.bj.recs.len(),
            vm: self.bj.vm_effects,
        };
        self.bj.marks.push(m);
        m
    }

    /// 标记出栈（正常完成，日志保留给外层）；最外层出栈即清空日志
    pub(super) fn jpop(&mut self, m: Mark) {
        self.bj.marks.truncate(m.depth);
        if self.bj.marks.is_empty() {
            self.bj.ents.clear();
        }
    }

    /// 回滚到标记（含出栈）：撤回的位置记为脏位置
    pub(super) fn jrollback(&mut self, m: Mark) -> R<()> {
        if self.bj.vm_effects != m.vm {
            return fail("撤回范围内有 VM 侧登记（模块表 / 构建期输出），不可残差化");
        }
        // 撤回后回到未初始化的类：其静态字段由日后的 `<clinit>` 重新定义，不记脏
        let uninit: HashSet<Rc<str>> = self.bj.ents[m.j..]
            .iter()
            .filter_map(|e| match e {
                JEnt::Init(c, None) => Some(c.clone()),
                _ => None,
            })
            .collect();
        while self.bj.ents.len() > m.j {
            let Some(e) = self.bj.ents.pop() else { break };
            match e {
                JEnt::Static(k, old) => {
                    match old {
                        Some(v) => self.statics.insert(k, v),
                        None => self.statics.remove(&k),
                    };
                    if old.is_some() || !uninit.contains(&self.fnames[k as usize].0) {
                        self.bj.dirty_static.insert(k);
                    }
                }
                JEnt::Field(o, k, old) => {
                    if let Body::Inst(fs) = &mut self.heap[o as usize].body {
                        fs.retain(|(x, _)| *x != k);
                        if let Some(v) = old {
                            fs.push((k, v));
                        }
                    }
                    self.bj.dirty_field.insert((o, k));
                }
                JEnt::Elem(o, i, v) => {
                    if let Body::Arr(a) = &mut self.heap[o as usize].body {
                        a[i] = v;
                    }
                    self.bj.dirty_arr.insert(o);
                }
                JEnt::Arr(o, v) => {
                    self.heap[o as usize].body = Body::Arr(v);
                    self.bj.dirty_arr.insert(o);
                }
                JEnt::Init(c, old) => {
                    match old {
                        Some(s) => self.init.insert(c, s),
                        None => self.init.remove(&c),
                    };
                }
                JEnt::Opaque(c) => {
                    self.opaque.remove(&c);
                }
                JEnt::Deferred(o) => {
                    self.deferred.remove(&o);
                }
                JEnt::Placeholder(o) => {
                    self.bj.placeholders.remove(&o);
                }
                JEnt::Cell(i, v) => {
                    self.cells[i].1 = v;
                    self.bj.dirty_cells.insert(i);
                }
            }
        }
        self.done_log.truncate(m.done);
        self.bj.recs.truncate(m.recs);
        self.bj.marks.truncate(m.depth);
        if self.bj.marks.is_empty() {
            self.bj.ents.clear();
        }
        Ok(())
    }

    fn logging(&self, o: u32) -> bool {
        // 类镜像是 VM 缓存的对象（新建后即被缓存持有），按已有对象记
        self.bj.marks.last().is_some_and(|m| (o as usize) < m.heap || self.mirror_of.contains_key(&o))
    }

    pub(super) fn jlog(&mut self, e: JEnt) {
        if !self.bj.marks.is_empty() {
            self.bj.ents.push(e);
        }
    }

    pub(super) fn jlog_static(&mut self, k: u32) {
        if !self.bj.marks.is_empty() {
            let old = self.statics.get(&k).copied();
            self.bj.ents.push(JEnt::Static(k, old));
        }
    }

    pub(super) fn jlog_field(&mut self, o: u32, k: u32) {
        if self.logging(o) {
            let old = match &self.heap[o as usize].body {
                Body::Inst(fs) => fs.iter().find(|(x, _)| *x == k).map(|(_, v)| *v),
                _ => None,
            };
            self.bj.ents.push(JEnt::Field(o, k, old));
        }
    }

    pub(super) fn jlog_init(&mut self, c: &Rc<str>) {
        if !self.bj.marks.is_empty() {
            let old = self.init.get(c).cloned();
            self.bj.ents.push(JEnt::Init(c.clone(), old));
        }
    }

    /// 延迟值登记（字符串内容数组 / 占位对象 / 残差调用实参）
    pub(super) fn mark_deferred(&mut self, o: u32, why: &str) {
        if self.deferred.insert(o, Rc::from(why)).is_none() {
            self.jlog(JEnt::Deferred(o));
        }
    }

    pub(super) fn mark_placeholder(&mut self, o: u32, why: &str) {
        self.mark_deferred(o, why);
        if self.bj.placeholders.insert(o) {
            self.jlog(JEnt::Placeholder(o));
        }
    }

    pub(super) fn mark_opaque(&mut self, c: Rc<str>) {
        if self.opaque.insert(c.clone()) {
            self.jlog(JEnt::Opaque(c));
        }
    }

    pub(super) fn is_placeholder(&self, v: CV) -> bool {
        matches!(v, CV::R(o) if self.bj.placeholders.contains(&o))
    }

    /// 占位对象参与身份运算（判空、比较、分派、取类型）
    pub(super) fn check_identity(&self, v: CV) -> R<()> {
        match v {
            CV::R(o) if self.bj.placeholders.contains(&o) => defer(format!("延迟值参与求值：{} 的结果参与身份运算", self.deferred.get(&o).map_or("残差调用", |k| k))),
            _ => Ok(()),
        }
    }

    pub(super) fn boot_arr_check(&self, o: u32) -> R<()> {
        if let Some(k) = self.deferred.get(&o) {
            return defer(format!("延迟值参与求值：{k} 的内容被读取"));
        }
        if self.bj.dirty_arr.contains(&o) {
            return defer(format!("延迟值参与求值：运行期重放会改写的数组 {}", self.heap[o as usize].ty));
        }
        Ok(())
    }

    /// 数组整体写（批量操作）：已有数组整体存底
    pub(super) fn boot_arr_write(&mut self, o: u32) -> R<()> {
        self.boot_arr_check(o)?;
        if self.logging(o) {
            if let Body::Arr(v) = &self.heap[o as usize].body {
                let v = v.clone();
                self.bj.ents.push(JEnt::Arr(o, v));
            }
        }
        Ok(())
    }

    /// 数组单元素写（xastore）
    pub(super) fn boot_elem_write(&mut self, o: u32, i: usize) -> R<()> {
        self.boot_arr_check(o)?;
        if self.logging(o) {
            if let Body::Arr(v) = &self.heap[o as usize].body {
                if let Some(&old) = v.get(i) {
                    self.bj.ents.push(JEnt::Elem(o, i, old));
                }
            }
        }
        Ok(())
    }

    pub(super) fn boot_field_check(&self, o: u32, fr: &FRes) -> R<()> {
        if let Some(k) = self.deferred.get(&o) {
            return defer(format!("延迟值参与求值：{k} 的字段 {} 被读取", fr.name));
        }
        if self.bj.dirty_field.contains(&(o, fr.key)) {
            return defer(format!("延迟值参与求值：运行期重放会改写的字段 {}.{}", fr.decl, fr.name));
        }
        Ok(())
    }

    pub(super) fn boot_field_write(&mut self, o: u32, fr: &FRes) -> R<()> {
        if let Some(k) = self.deferred.get(&o) {
            return defer(format!("延迟值参与求值：{k} 的字段 {} 被改写", fr.name));
        }
        self.jlog_field(o, fr.key);
        Ok(())
    }

    pub(super) fn boot_static_check(&self, fr: &FRes) -> R<()> {
        if self.bj.dirty_static.contains(&fr.key) {
            return defer(format!("延迟值参与求值：运行期重放会改写的静态字段 {}.{}", fr.decl, fr.name));
        }
        if self.opaque.contains(&fr.decl) {
            return defer(format!("延迟值参与求值：运行期初始化类的静态字段 {}.{}", fr.decl, fr.name));
        }
        Ok(())
    }
}
