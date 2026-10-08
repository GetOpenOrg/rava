//! 引擎：枚举常量身份标记——按常量取 final 字符串字段的名字（闭包构成报告 §7.4）。
//!
//! 枚举类 E 的 `<clinit>` 里每条 `new`（E 自身或常量体子类）分配一个枚举常量。这些分配各记一个**身份标记**
//! （类型为被分配的类，进入 `objs`，但不是堆抽象：不作克隆上下文，见 `ctxsel.rs::recv_ctx`），标记随常量值沿
//! 字节码数据流传播（`putstatic` 进常量字段、`values()` 数组、集合元素……）。按对象字段节点与逃逸接线照抽象对象
//! 口径处理（`flow.rs::obj_field` / `escape`），所以按对象读到的写入覆盖该常量上的全部写入。
//!
//! 名字求值：段是枚举 final 字符串字段的平凡取值（`method_lookup.rs::enum_field_values`）时，接收者值集若只由
//! 本枚举的身份标记组成，各标记的字段值取自 `<clinit>` 构造调用点的实参（构造器把该字段平凡写成某个形参或
//! 字符串常量，经 `super(..)` / `this(..)` 追溯），只给出流到接收者的那些常量的名字；值集含 open、非标记值
//! （映像物化的常量、反序列化分配等），或某标记的字段值推不出时，退回该枚举类全部 ldc 字符串（超集）。
//! 值集取自引擎流图节点并登记当前站点为读者（`value_set`），值集增长时站点重跑——结果只增不减。
//!
//! 健全性：枚举常量只能由本类 `<clinit>` 分配（JVMS 禁止反射构造枚举，反序列化按名取已有常量）；
//! 字段是 final 且构造器里只平凡写入（`ctor_writes_plain` 同口径），常量构造完成后字段值即构造实参。

use super::name_eval::Frame;
use super::*;

const CTOR: &str = "<init>";

/// 构造器链追溯层数上限
const MAX_CTOR_CHAIN: u8 = 4;

/// 构造器对字段的写入来源
#[derive(Clone, PartialEq)]
enum Slot {
    /// 本构造器链上没有写该字段
    None,
    /// 写入值是构造器第 k 个形参（不含接收者）
    Param(usize),
    /// 写入值是字符串常量
    Lit(Rc<str>),
    /// 推不出（多处来源不一致、写入值非平凡）
    Unknown,
}

impl<'a> Engine<'a> {
    /// 字节码 `new c` 位于枚举类 `<clinit>`（方法 m、偏移 off）且 c 是该枚举或其常量体子类时，该常量的身份标记
    pub(super) fn enum_const_mark(&mut self, m: usize, off: u32, c: &str) -> Option<u32> {
        let key = &self.methods[m].key;
        if key.name.as_str() != "<clinit>" || self.methods[m].ctx != NOCTX {
            return None;
        }
        let owner = key.owner.clone();
        let ecf = self.h.class(&owner)?;
        if ecf.access & acc::ENUM == 0 {
            return None;
        }
        if *c != *owner {
            let cf = self.h.class(c)?;
            if cf.super_name.as_deref() != Some(&*owner) {
                return None;
            }
        }
        let name = format!("{c}#<const@{off}>");
        if let Some(&id) = self.ids.get(name.as_str()) {
            return Some(id);
        }
        let tid = self.id(c);
        let id = self.id(&name);
        self.objs.insert(id, tid);
        self.enum_consts.insert(id, (m, off));
        Some(id)
    }

    /// 枚举类 decl 的 final 字符串字段 (fname, fdesc) 在接收者 recv（帧 f 中的值）所指常量上的名字；
    /// None = 按常量推不出（调用方退回全部 ldc 字符串）
    pub(super) fn enum_recv_names(&mut self, f: &Frame, recv: &V, decl: &str, fname: &str, fdesc: &str) -> Option<BTreeSet<Rc<str>>> {
        // 只在站点求值中按值集答复：值集增长时由站点重跑补齐
        if self.cur_site.is_none() && self.cur_call.is_none() {
            return None;
        }
        let (rf, rv) = f.resolve(recv);
        let m = rf.m?;
        if !matches!(rv, V::Ref { .. }) {
            return None;
        }
        let did = self.id(decl);
        let fs = self.feeds(m, &rv, did);
        let s = self.value_set(&fs);
        if !s.open.is_empty() {
            return None;
        }
        let mut out = BTreeSet::new();
        for x in s.classes.iter() {
            let t = *self.objs.get(&x)?;
            if !self.sub(t, did) {
                // 不是本枚举的值：运行期在取值方法上抛 ClassCastException / 不会到达这里，不贡献名字
                continue;
            }
            // null 写入：按名取类在 null 上抛 NullPointerException，不贡献名字
            if let Some(s) = self.enum_const_field(x, decl, fname, fdesc)? {
                out.insert(s);
            }
        }
        Some(out)
    }

    /// 身份标记 x 所指常量的字段值：Some(Some(串)) / Some(None) = null / None = 推不出（含 x 不是枚举常量标记）
    fn enum_const_field(&mut self, x: u32, decl: &str, fname: &str, fdesc: &str) -> Option<Option<Rc<str>>> {
        let ck = (x, Rc::<str>::from(fname));
        if let Some(r) = self.enum_vals.get(&ck) {
            return r.clone();
        }
        let r = self.enum_const_field_uncached(x, decl, fname, fdesc);
        self.enum_vals.insert(ck, r.clone());
        r
    }

    fn enum_const_field_uncached(&mut self, x: u32, decl: &str, fname: &str, fdesc: &str) -> Option<Option<Rc<str>>> {
        let &(m, off) = self.enum_consts.get(&x)?;
        let a = self.site_analysis(m)?;
        if a.conservative {
            return None;
        }
        // 该分配的构造调用：invokespecial <init>，接收者恰为本分配点的值
        let mut call: Option<(MemberRef, Vec<V>)> = None;
        for (_, e) in &a.events {
            let Event::Invoke { opcode: classfile::op::INVOKESPECIAL, mref, args, .. } = e else { continue };
            if mref.name != CTOR || !matches!(args.first(), Some(V::Ref { src, .. }) if src[..] == [Src::Site(off)]) {
                continue;
            }
            if call.is_some() {
                return None;
            }
            call = Some((mref.clone(), args.clone()));
        }
        let (mref, args) = call?;
        match self.ctor_field_slot(&mref.owner, &mref.desc, decl, fname, fdesc, 0) {
            Slot::Lit(s) => Some(Some(s)),
            Slot::Param(k) => match args.get(k + 1)? {
                V::Str(s, _) if !args[k + 1].derived_str() => Some(Some(s.clone())),
                V::Null => Some(None),
                _ => None,
            },
            Slot::None | Slot::Unknown => None,
        }
    }

    /// 类 cls 的构造器 desc 对字段 decl.fname 的写入来源（沿对 this 的 `super(..)` / `this(..)` 追溯）
    fn ctor_field_slot(&self, cls: &str, desc: &str, decl: &str, fname: &str, fdesc: &str, depth: u8) -> Slot {
        if depth > MAX_CTOR_CHAIN {
            return Slot::Unknown;
        }
        let Some(cf) = self.h.class(cls) else { return Slot::Unknown };
        let Some(code) = cf.method(CTOR, desc).and_then(|mm| mm.code.as_ref()) else { return Slot::Unknown };
        let a = absint::analyze(cls, desc, false, code, &super::sealed::Plain);
        if a.conservative {
            return Slot::Unknown;
        }
        let on_this = |v: Option<&V>| matches!(v, Some(V::Ref { src, .. }) if src[..] == [Src::Param(0)]);
        // 形参值 → 本构造器的来源
        let of_value = |v: Option<&V>| -> Slot {
            match v {
                Some(s @ V::Str(x, _)) if !s.derived_str() => Slot::Lit(x.clone()),
                Some(V::Ref { src, .. }) => match src[..] {
                    [Src::Param(k)] if k > 0 => Slot::Param(k as usize - 1),
                    _ => Slot::Unknown,
                },
                _ => Slot::Unknown,
            }
        };
        let mut out = Slot::None;
        let mut merge = |s: Slot| {
            out = match (&out, s) {
                (_, Slot::None) => out.clone(),
                (Slot::None, s) => s,
                (p, s) if *p == s => s,
                _ => Slot::Unknown,
            };
        };
        for (_, e) in &a.events {
            match e {
                Event::Field { opcode: classfile::op::PUTFIELD, mref, recv, value, .. } if mref.name == fname && mref.desc == fdesc => {
                    let owner_hit = mref.owner == decl || self.h.is_subtype(&mref.owner, decl);
                    if !owner_hit {
                        continue;
                    }
                    if !on_this(recv.as_ref()) {
                        // 写别的对象的同名字段：不影响本对象，但本口径不细分，按推不出
                        merge(Slot::Unknown);
                        continue;
                    }
                    merge(of_value(value.as_ref()));
                }
                Event::Invoke { opcode: classfile::op::INVOKESPECIAL, mref, args, .. } if mref.name == CTOR && on_this(args.first()) => {
                    let s = match self.ctor_field_slot(&mref.owner, &mref.desc, decl, fname, fdesc, depth + 1) {
                        Slot::Param(j) => of_value(args.get(j + 1)),
                        s => s,
                    };
                    merge(s);
                }
                _ => {}
            }
        }
        out
    }
}
