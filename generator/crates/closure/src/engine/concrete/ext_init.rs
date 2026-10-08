//! 构建期初始化扩展：引导之后的类初始化（计划 2026-10-05-boot-image-evaluator §5.8）。
//!
//! 引导映像导出后保留求值器，分析中首次初始化的类先尝试在构建期执行 `<clinit>`（与引导类同一规则），成功即
//! 结果进映像（扩展组 `c:<类>`），运行期不再执行初始化链；失败即运行期初始化并记录原因。
//!
//! 扩展组须与程序无关（档案按键求并，同键内容须相同）。程序之间只差「哪些类初始化过、以什么次序」，于是一次
//! 尝试必须与此前初始化过的其它扩展类、与程序在运行期可能改变的状态隔离：
//! - 读：它类的静态字段只许读不变的（final / 只在 `<clinit>` 写入）与内存缓存字段；它类对象（映像对象、
//!   此前扩展类的对象）只许读不变字段、内存缓存字段、类镜像与稳定类型的字段；它类数组只许读冻结的（字符串内容、
//!   经稳定类型取到的数组）；
//! - 写：只许写本类的静态字段与本次尝试新建的对象；内存缓存字段的写入在尝试结束后撤销（缓存可重算）；
//! - 初始化：依赖的类嵌套尝试（失败即本类失败）；依赖引导中转为运行期初始化的类、初始化环即失败；
//! - 身份哈希：本类对象按「类名 + 序号」、共享对象（字符串 / 类镜像）按键确定，与全局次序无关；
//! - 效果：残差、VM 侧登记、延迟值、污点、VM 单元等任何增长即失败；`<clinit>` 抛出异常即失败（运行期照常抛）；
//! - 导出：本类静态字段可达的对象不得含 lambda / 污点 / 占位，不得引用映像之外的对象。
//!
//! 尝试在日志标记内执行，失败即回滚（新建的类镜像的 VM 字段保留：镜像由 VM 缓存，与尝试无关）。

use super::vm::*;
use super::*;

/// 单个扩展类（含嵌套依赖）尝试的指令步数上限
const EXT_STEPS: u64 = 20_000_000;

pub(super) struct Attempt {
    pub class: Rc<str>,
    /// 尝试开始时的堆规模：此后新建且未被嵌套类认领、非共享的对象属本类
    pub floor: usize,
    /// 本类对象身份哈希序号
    pub hn: u32,
}

#[derive(Default)]
pub(super) struct Ext {
    pub stack: Vec<Attempt>,
    /// 扩展对象 → 所属扩展类（成功后认领）
    pub owner: HashMap<u32, Rc<str>>,
    /// 扩展期新建的共享对象 → 组键（字符串 `s:` 与其内容数组、类镜像 `m:`）
    pub shared: HashMap<u32, Rc<str>>,
    /// 运行期初始化的类 → 原因
    pub failed: BTreeMap<Rc<str>, String>,
    /// 构建期初始化成功的类（完成次序）
    pub done: Vec<Rc<str>>,
    /// 引导中转为运行期初始化的类
    pub rt: HashSet<Rc<str>>,
    /// 堆下标 → 映像编号（引导导出 + 已追加的扩展组）
    pub ids: HashMap<u32, u32>,
    /// 字符串内容字段（清单 `vm_fields.string_value`）的字段键：经它取到的数组冻结
    pub str_value: Option<u32>,
    /// 尝试中出现不可撤回的效果：此后不再尝试
    pub broken: Option<String>,
}

/// 尝试前的效果计数
#[derive(PartialEq, Debug)]
struct Snap {
    recs: usize,
    vm_effects: u64,
    placeholders: usize,
    deferred: usize,
    host_src: usize,
    boot_objs: usize,
    modules: usize,
    singletons: usize,
    exprs: usize,
    tables: usize,
    opaque: usize,
    cells: Vec<i64>,
}

/// 32 位 FNV-1a（身份哈希：正数、非零）
fn fnv32(s: &str) -> i32 {
    let mut h: u32 = 0x811c_9dc5;
    for b in s.bytes() {
        h = (h ^ u32::from(b)).wrapping_mul(0x0100_0193);
    }
    ((h & 0x7fff_ffff) | 1) as i32
}

impl Vm {
    /// 进入扩展期：`ids` 为引导导出的堆下标 → 映像编号
    pub(super) fn ext_enter(&mut self, env: &Env, ids: HashMap<u32, u32>) {
        let rt = self.bj.recs.iter().filter_map(|r| if let super::journal::Rec::RuntimeInit { class, .. } = r { Some(class.clone()) } else { None }).collect();
        let str_value = env.cfg().vm_fields.get("string_value").and_then(|s| s.rsplit_once('.')).map(|(o, n)| (o.to_string(), n.to_string()));
        let str_value = str_value.map(|(o, n)| self.fkey(&o, &n));
        self.ext = Some(Box::new(Ext { rt, ids, str_value, ..Default::default() }));
        // 扩展类在进入 main 之后初始化：返回值事实（如 VM 已引导完毕）对有字节码的方法同样成立
        self.minfo.clear();
    }

    fn ext_top(&self) -> Option<(&Ext, &Attempt)> {
        let x = self.ext.as_deref()?;
        Some((x, x.stack.last()?))
    }

    /// 对象属当前尝试的类（无尝试时恒真）
    pub(super) fn ext_own(&self, o: u32) -> bool {
        match self.ext_top() {
            None => true,
            Some((x, t)) => o as usize >= t.floor && !x.owner.contains_key(&o) && !x.shared.contains_key(&o),
        }
    }

    /// 已初始化 / 正在初始化的类被再次触发初始化
    pub(super) fn ext_inited(&self, c: &str) -> R<()> {
        let Some((x, t)) = self.ext_top() else { return Ok(()) };
        if x.rt.contains(c) {
            return fail(format!("依赖运行期初始化类 {c}"));
        }
        if matches!(self.init.get(c), Some(Init::Running)) && *t.class != *c {
            return fail(format!("初始化环：{c} 正在初始化"));
        }
        Ok(())
    }

    /// 静态字段读：它类只许读不变字段与内存缓存字段
    pub(super) fn ext_get_static(&self, fr: &FRes) -> R<()> {
        match self.ext_top() {
            Some((_, t)) if fr.decl != t.class && !fr.fin && !fr.memo => fail(format!("读取它类可变静态字段 {}.{}", fr.decl, fr.name)),
            _ => Ok(()),
        }
    }

    /// 静态字段写：只许写本类
    pub(super) fn ext_put_static(&self, decl: &str, name: &str) -> R<()> {
        match self.ext_top() {
            Some((_, t)) if decl != &*t.class => fail(format!("写入它类静态字段 {decl}.{name}")),
            _ => Ok(()),
        }
    }

    /// 实例字段读：它类对象只许读不变字段、内存缓存字段、类镜像与稳定类型的字段
    pub(super) fn ext_get_field(&mut self, env: &Env, o: u32, fr: &FRes) -> R<()> {
        if self.ext_own(o) || fr.fin || fr.memo || self.mirror_of.contains_key(&o) || self.stable(env, o) {
            return Ok(());
        }
        fail(format!("读取它类对象可变字段 {}.{}", fr.decl, fr.name))
    }

    /// 经它类对象取到的数组：取自稳定类型或字符串内容字段即冻结（此后可读）
    pub(super) fn ext_freeze(&mut self, env: &Env, o: u32, fr: &FRes, v: CV) {
        let Some(x) = self.ext.as_deref() else { return };
        let CV::R(a) = v else { return };
        if self.ext_own(o) || !matches!(self.heap[a as usize].body, Body::Arr(_)) {
            return;
        }
        if x.str_value == Some(fr.key) || self.stable(env, o) {
            self.frozen.insert(a);
        }
    }

    pub(super) fn ext_arr_read(&self, o: u32) -> R<()> {
        if self.ext_own(o) || self.frozen.contains(&o) {
            return Ok(());
        }
        fail(format!("读取它类数组 {}", self.heap[o as usize].ty))
    }

    pub(super) fn ext_write(&self, o: u32) -> R<()> {
        if self.ext_own(o) {
            return Ok(());
        }
        fail(format!("改写它类对象 {}", self.heap[o as usize].ty))
    }

    /// 内存缓存字段写入：扩展期一律记撤销（尝试结束后撤销，结果与初始化次序无关）
    pub(super) fn ext_memo(&self) -> bool {
        self.ext.as_ref().is_some_and(|x| !x.stack.is_empty())
    }

    /// 共享对象登记（扩展期新建的驻留字符串 / 类镜像）
    pub(super) fn ext_share(&mut self, objs: &[u32], key: String) {
        if let Some(x) = self.ext.as_deref_mut() {
            let k: Rc<str> = Rc::from(key);
            for &o in objs {
                x.shared.insert(o, k.clone());
            }
        }
    }

    /// 身份哈希（按首次查询顺序编号；扩展期按所属确定）
    pub(super) fn identity_hash(&mut self, o: u32) -> R<i32> {
        if let Some(&h) = self.ihash.get(&o) {
            return Ok(h);
        }
        let h = if !self.ext_memo() {
            0x1000 + self.ihash.len() as i32 * 7919
        } else {
            let ty = self.heap[o as usize].ty.clone();
            let x = self.ext.as_deref_mut().expect("扩展期");
            if let Some(k) = x.shared.get(&o) {
                fnv32(k)
            } else {
                let owned = x.owner.contains_key(&o);
                let t = x.stack.last_mut().expect("尝试");
                if o as usize >= t.floor && !owned {
                    t.hn += 1;
                    fnv32(&format!("{}#{}", t.class, t.hn))
                } else {
                    return fail(format!("取它类对象的身份哈希 {ty}"));
                }
            }
        };
        self.ihash.insert(o, h);
        Ok(h)
    }

    fn ext_snap(&self) -> Snap {
        Snap {
            recs: self.bj.recs.len(),
            vm_effects: self.bj.vm_effects,
            placeholders: self.bj.placeholders.len(),
            deferred: self.deferred.len(),
            host_src: self.host_src.len(),
            boot_objs: self.boot_objs.len(),
            modules: self.modules.len(),
            singletons: self.singletons.len(),
            exprs: self.bj.taint.exprs.len(),
            tables: self.vm_tables.values().sum(),
            opaque: self.opaque.len(),
            cells: self.cells.iter().map(|c| c.1).collect(),
        }
    }

    /// 扩展类初始化尝试（`ensure_init` 在扩展期对未初始化的类调用）
    pub(super) fn ext_init(&mut self, env: &Env, key: Rc<str>) -> R<()> {
        let x = self.ext.as_deref_mut().expect("扩展期");
        if let Some(w) = &x.broken {
            return fail(format!("扩展求值已中止：{w}"));
        }
        if let Some(w) = x.failed.get(&key) {
            return fail(format!("依赖类 {key} 运行期初始化：{w}"));
        }
        let outer = x.stack.is_empty();
        let done0 = x.done.len();
        x.stack.push(Attempt { class: key.clone(), floor: self.heap.len(), hn: 0 });
        if outer {
            self.step_limit = self.steps + EXT_STEPS;
        }
        let snap = self.ext_snap();
        let m = self.jmark();
        let r = self.boot_init(env, key.clone());
        let r = r.and_then(|()| self.ext_verify(env, &key, &snap));
        let att = self.ext.as_deref_mut().expect("扩展期").stack.pop().expect("尝试");
        let r = match r {
            Ok(()) => {
                self.jpop(m);
                let x = self.ext.as_deref_mut().expect("扩展期");
                for o in att.floor..self.heap.len() {
                    let o = o as u32;
                    if !x.shared.contains_key(&o) {
                        x.owner.entry(o).or_insert_with(|| key.clone());
                    }
                }
                x.done.push(key.clone());
                Ok(())
            }
            Err(f) => {
                let why = match &f {
                    Flow::Fail(w) | Flow::Defer(w) => w.split(" @ ").next().unwrap_or(w).to_string(),
                    Flow::Throw(o) => format!("抛出 {}", self.ty(*o)),
                    Flow::Implicit(k) => format!("隐式异常 {k}"),
                };
                if let Err(Flow::Fail(w)) = self.jrollback_for(m, None) {
                    self.ext.as_deref_mut().expect("扩展期").broken = Some(w);
                }
                self.init.remove(&key);
                let x = self.ext.as_deref_mut().expect("扩展期");
                // 嵌套成功的依赖随本次撤回回到未初始化
                x.done.truncate(done0);
                x.owner.retain(|&o, _| (o as usize) < att.floor);
                x.failed.insert(key.clone(), why.clone());
                fail(format!("类 {key} 运行期初始化：{why}"))
            }
        };
        if outer {
            // 内存缓存写入撤销；诊断栈清空（失败是正常结局）
            self.rollback();
            self.fail_frames = None;
            self.throw_frames = None;
        }
        r
    }

    /// 尝试成功后的检查：效果未增长、导出可行
    fn ext_verify(&mut self, env: &Env, key: &Rc<str>, snap: &Snap) -> R<()> {
        let now = self.ext_snap();
        if now != *snap {
            let what = if now.recs != snap.recs {
                match self.bj.recs.iter().rev().find_map(|r| if let super::journal::Rec::RuntimeInit { why, .. } = r { Some(why.clone()) } else { None }) {
                    Some(w) => format!("依赖宿主值：{w}"),
                    None => "产生运行期残差".to_string(),
                }
            } else if now.opaque != snap.opaque {
                "静态状态由 VM / 手写层承载".to_string()
            } else {
                "有构建期不可保留的效果（VM 登记 / 延迟值 / 污点 / VM 单元）".to_string()
            };
            return fail(what);
        }
        let _ = env;
        let mut stack: Vec<u32> = Vec::new();
        let mut seen: HashSet<u32> = HashSet::default();
        for (&k, &v) in &self.statics {
            if *self.fnames[k as usize].0 != **key {
                continue;
            }
            if matches!(v, CV::T(..)) {
                return fail(format!("静态字段 {} 取宿主标量", self.fnames[k as usize].1));
            }
            if let CV::R(o) = v {
                stack.push(o);
            }
        }
        let x = self.ext.as_deref().expect("扩展期");
        while let Some(o) = stack.pop() {
            if x.ids.contains_key(&o) || x.owner.contains_key(&o) || !seen.insert(o) {
                continue;
            }
            let shared = x.shared.contains_key(&o);
            if !shared && !self.ext_own(o) {
                return fail(format!("引用映像之外的对象 {}", self.heap[o as usize].ty));
            }
            if self.deferred.contains_key(&o) || self.bj.placeholders.contains(&o) || self.host_src.contains_key(&o) {
                return fail(format!("引用延迟值对象 {}", self.heap[o as usize].ty));
            }
            let vals: Vec<CV> = match &self.heap[o as usize].body {
                Body::Inst(fs) => fs.iter().map(|e| e.1).collect(),
                Body::Arr(a) => a.clone(),
                Body::Lam(l) => return fail(format!("映像含 lambda 对象（{} ← {}）", l.iface, l.imp.member)),
            };
            for v in vals {
                match v {
                    CV::T(..) => return fail(format!("对象 {} 含宿主标量", self.heap[o as usize].ty)),
                    CV::R(r) => stack.push(r),
                    v if shared && super::unsafe_ops::reloc_of(self, v).is_some() => return fail("共享对象含重定位值"),
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

/// 构建期初始化扩展的求值器（引导映像求值器导出后保留；引擎持有）
pub struct ExtVm {
    vm: Vm,
    /// 已追加进映像的扩展类数（`Ext::done` 前缀）
    from: usize,
    /// 已成功但导出失败的类 → 原因
    export_failed: BTreeMap<String, String>,
}

impl ExtVm {
    pub(super) fn new(vm: Vm) -> Self {
        ExtVm { vm, from: 0, export_failed: BTreeMap::new() }
    }

    /// 类首次初始化：尝试构建期初始化。成功即把新完成的扩展类（含嵌套依赖）追加进映像数据 `d`，返回这些类；
    /// 引导中已处理（运行期初始化 / 已初始化）或尝试失败返回 None
    pub(in crate::engine) fn attempt(&mut self, ctx: &Ctx, cp: &ClassPath, cls: &str, d: &mut crate::image::ImageData) -> Option<Vec<String>> {
        if cls.starts_with('[') || self.vm.init.contains_key(cls) || ctx.h.class(cls).is_none() {
            return None;
        }
        let env = Env { ctx, cp };
        self.vm.ensure_init(&env, cls).ok()?;
        let r = self.vm.ext_append(d, self.from);
        self.from = self.vm.ext.as_deref().map_or(0, |x| x.done.len());
        match r {
            Ok(cs) => Some(cs),
            Err(w) => {
                self.export_failed.insert(cls.to_string(), w);
                None
            }
        }
    }

    /// closure.json `summary.build_time_init`：构建期初始化成功的类数、运行期初始化的类与原因（按原因计数）
    pub(in crate::engine) fn report(&self) -> serde_json::Value {
        let Some(x) = self.vm.ext.as_deref() else { return serde_json::Value::Null };
        let mut reasons: BTreeMap<String, usize> = BTreeMap::new();
        for w in x.failed.values().chain(self.export_failed.values()) {
            // 原因按首段归并（去掉类名后的具体位置）
            let k: String = w.chars().take(60).collect();
            *reasons.entry(k).or_default() += 1;
        }
        let mut top: Vec<(String, usize)> = reasons.into_iter().collect();
        top.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        top.truncate(40);
        serde_json::json!({
            "classes": x.done.len(),
            "runtime": x.failed.len() + self.export_failed.len(),
            "failed": x.failed.iter().map(|(c, w)| (c.to_string(), serde_json::Value::from(w.as_str()))).chain(self.export_failed.iter().map(|(c, w)| (c.clone(), serde_json::Value::from(format!("导出：{w}"))))).collect::<serde_json::Map<String, serde_json::Value>>(),
            "reasons": top.into_iter().map(|(w, n)| serde_json::json!([w, n])).collect::<Vec<_>>(),
        })
    }
}
