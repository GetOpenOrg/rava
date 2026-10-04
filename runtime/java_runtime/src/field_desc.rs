//! 实例字段描述与按描述符的字段协议（S7-3，docs/plans/2026-10-04-s7-object-handle-descriptor.md）。
//!
//! 每个类的描述符（`__ClassDesc`）列出本类自有实例字段（`fields`）与继承字段数（`field_base`）。
//! 存储 `X__inner` 是 `#[repr(C)]`：继承字段（最深祖先在前）、自有字段、标识单元依次排列，每个
//! 字段都是一个 `__Shared` 细指针，故本类第 i 个自有字段位于存储基址 + `(field_base + i)` 个
//! 指针宽（宏为每个存储发射编译期断言守护该布局）。
//!
//! 由此，按类展开的浅拷贝、Unsafe 按名单元 / 字视图 / 引用协议、Java 字段身份 → Rust 字段名
//! 映射都改为这里的非泛型逻辑：沿运行时类描述符的 `display` 查各类的 `fields`。

use crate::class_desc::__ClassDesc;
use crate::java::lang::{Object, ObjectVTable};
use crate::sync_model::{__AtomicRepr, __PrimCell, __RefSlot, __Shared};

/// 引用原子协议的操作（`__unsafe_ref_get` / `__unsafe_ref_set` / `__unsafe_ref_update` 的落点）。
#[doc(hidden)]
pub enum __RefAccess<'a> {
    /// 读：可重入读锁内取当前值；`None`（未写入）与 `Some(Box<null>)` 均以 jvm-null 应答。
    Get,
    /// 写：命中时取走值写入（值经 `<T as From<Object>>::from` 还原声明类型视图，在取写锁
    /// 之前完成——类型不符按 checkcast 语义处理）；未命中时值原样保留。
    Set(Option<Object>),
    /// 读-改-写：写锁内读出当前值 `cur`，`f(cur)` 返回 `Some(new)` 时写入，应答 `cur`。
    Update(&'a mut dyn FnMut(Object) -> Option<Object>),
    /// 浅拷贝：把本单元的值（引用本身）写入同类型的目标单元（目标存储中同一字段的地址）。
    CopyInto(*const ()),
}

/// 引用字段单元上的协议操作：按字段载体类型实例化的 `__ref_field::<T>`，入参是存储中该字段
/// （`__Shared<__RefSlot<Option<Box<T>>>>`）的地址。
pub type __RefFieldFn = for<'a, 'b> unsafe fn(*const (), &'a mut __RefAccess<'b>) -> Option<Object>;

/// 实例字段的存储形态。基本类型单元一律是 `__Shared<__PrimCell<T>>`（`AtomicU64` 位形，
/// `#[repr(transparent)]`）；按 Java 基本类型细分只为 Unsafe 的字 / 双字视图与按名单元协议。
#[derive(Clone, Copy)]
pub enum __FieldKind {
    Bool,
    Byte,
    Short,
    Char,
    Int,
    Float,
    Long,
    Double,
    /// 其余基本单元（手写类的 `usize` 等运行期字段）：只参与浅拷贝
    Prim,
    /// 引用 / 擦除字段（擦除字段载体为 `Object`）
    Ref(__RefFieldFn),
}

impl std::fmt::Debug for __FieldKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            __FieldKind::Bool => "Bool",
            __FieldKind::Byte => "Byte",
            __FieldKind::Short => "Short",
            __FieldKind::Char => "Char",
            __FieldKind::Int => "Int",
            __FieldKind::Float => "Float",
            __FieldKind::Long => "Long",
            __FieldKind::Double => "Double",
            __FieldKind::Prim => "Prim",
            __FieldKind::Ref(_) => "Ref",
        })
    }
}

/// 本类声明的一个实例字段。
#[derive(Debug)]
pub struct __FieldDesc {
    /// Java 字段名
    pub java: &'static str,
    /// 存储里的 Rust 字段名（关键字加后缀、遮蔽祖先同名字段的加声明类后缀；沿继承链唯一）
    pub rust: &'static str,
    pub kind: __FieldKind,
}

/// 引用单元的协议实现，按字段载体类型 `T` 实例化（跨类共享）。擦除字段 `T = Object`。
///
/// # Safety
/// `slot`（与 `CopyInto` 的目标）必须指向存活存储中载体类型为 `T` 的引用字段。
#[doc(hidden)]
#[inline(never)]
pub unsafe fn __ref_field<T>(slot: *const (), op: &mut __RefAccess<'_>) -> Option<Object>
where T: Clone + From<Object>, Object: From<T>
{
    type Cell<T> = __Shared<__RefSlot<Option<Box<T>>>>;
    // SAFETY: 调用方保证
    let slot: &Cell<T> = unsafe { &*(slot as *const Cell<T>) };
    match op {
        __RefAccess::Get => Some(Option::unwrap_or_default(
            slot.borrow().as_deref().map(|b| Object::from(Clone::clone(b))))),
        __RefAccess::Set(v) => {
            let v = Some(Box::new(<T as From<Object>>::from(v.take().unwrap_or_default())));
            *slot.borrow_mut() = v;
            Some(Object::default())
        }
        __RefAccess::Update(f) => {
            let mut g = slot.borrow_mut();
            let cur: Object = Option::unwrap_or_default(
                g.as_deref().map(|b| Object::from(Clone::clone(b))));
            if let Some(n) = f(Clone::clone(&cur)) {
                *g = Some(Box::new(<T as From<Object>>::from(n)));
            }
            Some(cur)
        }
        __RefAccess::CopyInto(dst) => {
            // SAFETY: 调用方保证目标是同一字段（同一载体类型）
            let dst: &Cell<T> = unsafe { &*(*dst as *const Cell<T>) };
            let v = slot.borrow().clone();
            *dst.borrow_mut() = v;
            Some(Object::default())
        }
    }
}

/// 存储基址起第 `idx` 个字段的地址。
#[inline]
fn slot_at(base: *const (), idx: usize) -> *const () {
    (base as *const usize).wrapping_add(idx) as *const ()
}

/// 基本单元按 `T` 解读。
///
/// # Safety
/// `slot` 指向存活存储中的基本字段；`__PrimCell<_>` 为 `#[repr(transparent)]` 的 `AtomicU64`，
/// 各实例化布局相同，按另一 `T` 解读只改变位形的读写方式。
#[inline]
pub(crate) unsafe fn prim_at<'a, T: __AtomicRepr>(slot: *const ()) -> &'a __Shared<__PrimCell<T>> {
    unsafe { &*(slot as *const __Shared<__PrimCell<T>>) }
}

use prim_at as prim;

/// `Object.clone` 的类对象浅拷贝：经描述符的 `alloc` 新建运行时类的默认存储（新标识），再沿
/// `display` 逐字段拷贝——基本单元按位，引用单元拷引用（经该字段载体类型的 `__ref_field`）。
///
/// # Safety
/// `src` 是 `desc` 所述类的存活存储（`X__inner`）的基址。
#[doc(hidden)]
#[inline(never)]
pub unsafe fn __clone_fields(src: *const (), desc: &'static __ClassDesc) -> Object {
    let copy = (desc.alloc)();
    let dst = &*copy.0 as *const dyn ObjectVTable as *const ();
    for class in desc.display {
        for (i, f) in class.fields.iter().enumerate() {
            let idx = class.field_base as usize + i;
            let (s, d) = (slot_at(src, idx), slot_at(dst, idx));
            match f.kind {
                // SAFETY: 两处存储同属 `desc`，同一下标是同一字段
                __FieldKind::Ref(op) => unsafe { op(s, &mut __RefAccess::CopyInto(d)); },
                _ => unsafe { prim::<u64>(d).set(prim::<u64>(s).get()) },
            }
        }
    }
    copy
}

impl dyn ObjectVTable {
    /// 按 Rust 字段名找运行时类存储里的字段：(描述, 字段地址)。非类对象 / null → None。
    fn __field_at(&self, rust: &str) -> Option<(&'static __FieldDesc, *const ())> {
        if self.is_jvm_null() {
            return None;
        }
        let desc = self.__desc()?;
        let base = self as *const dyn ObjectVTable as *const ();
        desc.display.iter().rev().find_map(|class| {
            class.fields.iter().position(|f| f.rust == rust)
                .map(|i| (&class.fields[i], slot_at(base, class.field_base as usize + i)))
        })
    }

    /// Java 字段身份（声明类, 字段名）→ (种类, 字段地址)：声明类不在运行时类的祖先链上 →
    /// `Some(Err(()))`；null / 非类对象 / 声明类无此字段 → None。
    pub(crate) fn __instance_field(&self, decl: &str, name: &str)
        -> Option<Result<(__FieldKind, *const ()), ()>> {
        if self.is_jvm_null() {
            return None;
        }
        let desc = self.__desc()?;
        let Some(class) = desc.display.iter().find(|c| c.binary_name == decl) else {
            return Some(Err(()));
        };
        let i = class.fields.iter().position(|f| f.java == name)?;
        let base = self as *const dyn ObjectVTable as *const ();
        Some(Ok((class.fields[i].kind, slot_at(base, class.field_base as usize + i))))
    }

    /// 按名单元协议：类型为 `T` 的基本字段的共享单元（与该对象全部视图的字段读写同一存储）。
    fn __prim_cell<T: __AtomicRepr>(&self, field: &str, want: fn(__FieldKind) -> bool)
        -> Option<__Shared<__PrimCell<T>>> {
        let (f, p) = self.__field_at(field)?;
        // SAFETY: 字段种类已核对为 T 对应的基本类型
        want(f.kind).then(|| unsafe { prim::<T>(p) }.clone())
    }

    /// Unsafe 实例字段 long 单元（`getLongVolatile` / `compareAndSetLong` 等实例字段形态，及
    /// VarHandle `fieldOffset` 直读）：非擦除 long 字段 → 共享单元；其余 → None。
    #[doc(hidden)]
    pub fn __unsafe_long_cell(&self, field: &str) -> Option<__Shared<__PrimCell<i64>>> {
        self.__prim_cell(field, |k| matches!(k, __FieldKind::Long))
    }

    /// `__unsafe_long_cell` 的 int 镜像。
    #[doc(hidden)]
    pub fn __unsafe_int_cell(&self, field: &str) -> Option<__Shared<__PrimCell<i32>>> {
        self.__prim_cell(field, |k| matches!(k, __FieldKind::Int))
    }

    /// `__unsafe_long_cell` 的 boolean 镜像（VarHandle 字节数组视图的字节序位）。
    #[doc(hidden)]
    pub fn __unsafe_bool_cell(&self, field: &str) -> Option<__Shared<__PrimCell<bool>>> {
        self.__prim_cell(field, |k| matches!(k, __FieldKind::Bool))
    }

    /// Unsafe 实例字段 int 字视图：int、float（原始位）及子字（boolean / byte / short / char）
    /// 字段的共享单元上执行字视图读-改-写（`__PrimCell::__word_update`），返回旧字。子字字段
    /// 独占 4 字节对齐槽，JDK 的子字 CAS（`offset & ~3` + `weakCompareAndSetInt`）落在该字段
    /// 自身。未命中 → None。
    #[doc(hidden)]
    pub fn __unsafe_word(&self, field: &str, op: &mut dyn FnMut(i32) -> Option<i32>) -> Option<i32> {
        let (f, p) = self.__field_at(field)?;
        // SAFETY: 按字段种类取其单元类型
        unsafe {
            Some(match f.kind {
                __FieldKind::Int => prim::<i32>(p).__word_update(op),
                __FieldKind::Float => prim::<f32>(p).__word_update(op),
                __FieldKind::Bool => prim::<bool>(p).__word_update(op),
                __FieldKind::Byte => prim::<i8>(p).__word_update(op),
                __FieldKind::Short => prim::<i16>(p).__word_update(op),
                __FieldKind::Char => prim::<u16>(p).__word_update(op),
                _ => return None,
            })
        }
    }

    /// `__unsafe_word` 的 64 位镜像：long / double（原始位）字段。
    #[doc(hidden)]
    pub fn __unsafe_dword(&self, field: &str, op: &mut dyn FnMut(i64) -> Option<i64>) -> Option<i64> {
        let (f, p) = self.__field_at(field)?;
        // SAFETY: 按字段种类取其单元类型
        unsafe {
            Some(match f.kind {
                __FieldKind::Long => prim::<i64>(p).__dword_update(op),
                __FieldKind::Double => prim::<f64>(p).__dword_update(op),
                _ => return None,
            })
        }
    }

    /// Unsafe / VarHandle 实例字段引用原子协议：引用（含擦除）字段 → 在其单元上执行 `op`；
    /// 命中 → `Some`（写形态的值为命中标记）；未命中 → None 且 `op` 原样保留。
    #[doc(hidden)]
    pub fn __unsafe_ref_access(&self, field: &str, op: &mut __RefAccess<'_>) -> Option<Object> {
        let (f, p) = self.__field_at(field)?;
        match f.kind {
            // SAFETY: 函数指针按该字段载体类型实例化，p 是该字段地址
            __FieldKind::Ref(g) => unsafe { g(p, op) },
            _ => None,
        }
    }

    /// 引用原子协议读形态：命中 → `Some(当前值)`；未命中（无该引用字段）→ None（调用方归 stub）。
    #[doc(hidden)]
    pub fn __unsafe_ref_get(&self, field: &str) -> Option<Object> {
        self.__unsafe_ref_access(field, &mut __RefAccess::Get)
    }

    /// 引用原子协议写形态：命中写入返回 true；未命中 → false。
    #[doc(hidden)]
    pub fn __unsafe_ref_set(&self, field: &str, v: Object) -> bool {
        self.__unsafe_ref_access(field, &mut __RefAccess::Set(Some(v))).is_some()
    }

    /// int 按名写形态（`__unsafe_int_cell` 取单元再写）：命中写入返回 true；未命中 → false。
    /// 手写层以字面量字段名调用时，闭包分析器据此得知该名字段被写入（不折叠其读取）。
    #[doc(hidden)]
    pub fn __unsafe_int_set(&self, field: &str, v: i32) -> bool {
        self.__unsafe_int_cell(field).map(|c| c.set(v)).is_some()
    }

    /// 引用原子协议读-改-写形态：命中 → `Some(旧值)`；未命中 → None。Unsafe / VarHandle 的
    /// compareAndSet / compareAndExchange / getAndSet 引用族经此真正原子。
    #[doc(hidden)]
    pub fn __unsafe_ref_update(&self, field: &str,
                               f: &mut dyn FnMut(Object) -> Option<Object>) -> Option<Object> {
        self.__unsafe_ref_access(field, &mut __RefAccess::Update(f))
    }

    /// Java 字段身份（声明类 binary name, 字段名）→ 按名协议的 Rust 字段名（Unsafe 实例字段偏移按
    /// Java 字段身份登记）。声明类不在运行时类的祖先链上或无此字段 → None。
    #[doc(hidden)]
    pub fn __field_slot(&self, decl: &str, name: &str) -> Option<&'static str> {
        let desc = self.__desc()?;
        let class = desc.display.iter().find(|c| c.binary_name == decl)?;
        class.fields.iter().find(|f| f.java == name).map(|f| f.rust)
    }
}
