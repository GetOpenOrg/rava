//! 按名字段访问（Field.get/set、MethodHandle 字段句柄、Unsafe 静态字段）的描述符驱动实现（S7-3b，
//! docs/plans/2026-10-04-s7-object-handle-descriptor.md）。
//!
//! - **实例字段**：接收者运行时类描述符的 `display` 里找声明类，再按 Java 名找其自有字段
//!   （`__FieldDesc`），在存储中该字段的单元上读写——基本单元按种类装箱 / 拆箱，引用单元经该字段
//!   载体类型的 `__ref_field` 协议。不需要任何按类生成的代码。
//! - **静态字段**：宏为每个类 / 接口在 wrapper 上展开固有常量 `X::__STATICS`（关联常量，只在被
//!   登记处引用时求值与代码生成），每项记 Java 名与该静态字段既有访问器（getter / setter，含
//!   类初始化触发与安全点；手写访问器同样适用）的擦除函数指针，以及按值类型实例化的读写协议
//!   （`__static_ref::<T>` / `__static_prim::<P>`，跨类共享）。编译期常量无 setter，写入 →
//!   `final_field`。生成项目 main 启动时按声明类登记（`register_field_dispatch`）。

use std::collections::HashMap;

use crate::error::{JvmError, Result};
use crate::field_desc::{__FieldKind, __RefAccess};
use crate::java::lang::{Object, ObjectVTable};
use crate::reflect_dispatch::{bad_arg, final_field};

/// 静态字段的读写协议：按值类型实例化，入参是本项（取其擦除的访问器）。
pub type __StaticFieldFn = unsafe fn(&__StaticFieldDesc, Option<Object>) -> Result<Object>;

/// 类 / 接口声明的一个静态字段（宏展开的 `X::__STATICS` 的元素）。
pub struct __StaticFieldDesc {
    /// Java 字段名
    pub java: &'static str,
    /// 擦除的 getter：`fn() -> Result<T>`
    pub get: fn(),
    /// 擦除的 setter：`fn(T) -> Result<()>`；编译期常量为 None
    pub set: Option<fn()>,
    pub op: __StaticFieldFn,
}

/// 引用（含接口 / 数组）类型静态字段的读写。
///
/// # Safety
/// `d.get` / `d.set` 是值类型为 `T` 的访问器经 `transmute` 擦除得到的函数指针。
#[doc(hidden)]
#[inline(never)]
pub unsafe fn __static_ref<T>(d: &__StaticFieldDesc, value: Option<Object>) -> Result<Object>
where T: From<Object>, Object: From<T>
{
    match value {
        None => {
            // SAFETY: 调用方保证
            let get: fn() -> Result<T> = unsafe { std::mem::transmute(d.get) };
            Ok(Object::from(get()?))
        }
        Some(v) => {
            let Some(set) = d.set else { return Err(final_field(d.java)) };
            // SAFETY: 调用方保证
            let set: fn(T) -> Result<()> = unsafe { std::mem::transmute(set) };
            set(<T as From<Object>>::from(v))?;
            Ok(Object::default())
        }
    }
}

/// 基本类型值的反射拆箱（`Method.invoke` 同一拆箱规则，含基本类型拓宽）。
pub trait __ReflectPrim: Sized {
    fn unbox(v: &Object) -> Option<Self>;
}

macro_rules! reflect_prim {
    ($($t:ty => $f:expr;)*) => {
        $(impl __ReflectPrim for $t {
            fn unbox(v: &Object) -> Option<Self> { ($f)(v) }
        })*
    };
}

reflect_prim! {
    bool => crate::reflect_dispatch::unbox_bool;
    i8 => |v| crate::reflect_dispatch::unbox_i32(v).map(|x| x as i8);
    i16 => |v| crate::reflect_dispatch::unbox_i32(v).map(|x| x as i16);
    u16 => crate::reflect_dispatch::unbox_char;
    i32 => crate::reflect_dispatch::unbox_i32;
    f32 => crate::reflect_dispatch::unbox_f32;
    i64 => crate::reflect_dispatch::unbox_i64;
    f64 => crate::reflect_dispatch::unbox_f64;
}

/// 基本类型静态字段的读写（写入值拆箱失败 → `bad_arg`）。
///
/// # Safety
/// 同 [`__static_ref`]，值类型为 `P`。
#[doc(hidden)]
#[inline(never)]
pub unsafe fn __static_prim<P>(d: &__StaticFieldDesc, value: Option<Object>) -> Result<Object>
where P: __ReflectPrim, Object: From<P>
{
    match value {
        None => {
            // SAFETY: 调用方保证
            let get: fn() -> Result<P> = unsafe { std::mem::transmute(d.get) };
            Ok(Object::from(get()?))
        }
        Some(v) => {
            let Some(set) = d.set else { return Err(final_field(d.java)) };
            let v = P::unbox(&v).ok_or_else(bad_arg)?;
            // SAFETY: 调用方保证
            let set: fn(P) -> Result<()> = unsafe { std::mem::transmute(set) };
            set(v)?;
            Ok(Object::default())
        }
    }
}

crate::__process_static! {
    static STATICS: crate::sync_model::__RefSlot<HashMap<String, &'static [__StaticFieldDesc]>> =
        crate::sync_model::__RefSlot::new(HashMap::new());
}

/// 登记各声明类的静态字段表（binary name 斜线形态；重登记幂等）。
pub fn register(tables: &[(&str, &'static [__StaticFieldDesc])]) {
    STATICS.with(|s| {
        let mut s = s.borrow_mut();
        for (name, t) in tables {
            s.insert((*name).to_owned(), *t);
        }
    });
}

/// 静态字段：声明类已登记且有该字段 → 经其访问器读写。
pub(crate) fn static_field(decl: &str, name: &str, value: Option<Object>) -> Option<Result<Object>> {
    let table = STATICS.with(|s| s.borrow().get(decl).copied())?;
    let d = table.iter().find(|d| d.java == name)?;
    // SAFETY: 表项由宏按访问器的真实签名擦除并配对实例化的协议
    Some(unsafe { (d.op)(d, value) })
}

/// 实例字段：接收者是类对象且声明类在其运行时类的祖先链上、并声明了该字段 → 在该字段单元上
/// 读写；声明类不在链上 → IllegalArgumentException（JDK 字段访问器的接收者类型检查）。
/// null / 非类对象 / 无此字段 → None。
pub(crate) fn instance_field(decl: &str, name: &str, recv: &Object, value: Option<Object>)
    -> Option<Result<Object>> {
    let (kind, p) = match (*recv.0).__instance_field(decl, name)? {
        Ok(at) => at,
        Err(()) => return Some(Err(JvmError::illegal_argument(&format!(
            "Can not access field {}.{} on {}",
            decl.replace('/', "."), name, crate::meta::java_name(recv.0.__class_name()))))),
    };
    // SAFETY: p 是存活存储（recv 持有）中种类为 kind 的字段地址
    Some(unsafe { access(kind, p, value) })
}

/// 存储单元上的反射读写。
///
/// # Safety
/// `p` 指向存活存储中种类为 `kind` 的字段单元。
unsafe fn access(kind: __FieldKind, p: *const (), value: Option<Object>) -> Result<Object> {
    use crate::field_desc::prim_at as prim;
    fn put<P: __ReflectPrim + crate::sync_model::__AtomicRepr>(p: *const (), v: &Object) -> Result<()> {
        let v = P::unbox(v).ok_or_else(bad_arg)?;
        // SAFETY: 外层调用方保证
        unsafe { prim::<P>(p) }.set(v);
        Ok(())
    }
    let Some(v) = value else {
        // SAFETY: 按字段种类取其单元类型
        return Ok(unsafe {
            match kind {
                __FieldKind::Bool => Object::from(prim::<bool>(p).get()),
                __FieldKind::Byte => Object::from(prim::<i8>(p).get()),
                __FieldKind::Short => Object::from(prim::<i16>(p).get()),
                __FieldKind::Char => Object::from(prim::<u16>(p).get()),
                __FieldKind::Int => Object::from(prim::<i32>(p).get()),
                __FieldKind::Float => Object::from(prim::<f32>(p).get()),
                __FieldKind::Long => Object::from(prim::<i64>(p).get()),
                __FieldKind::Double => Object::from(prim::<f64>(p).get()),
                __FieldKind::Prim => return Err(bad_arg()),
                __FieldKind::Ref(op) => op(p, &mut __RefAccess::Get).unwrap_or_default(),
            }
        });
    };
    match kind {
        __FieldKind::Bool => put::<bool>(p, &v)?,
        __FieldKind::Byte => put::<i8>(p, &v)?,
        __FieldKind::Short => put::<i16>(p, &v)?,
        __FieldKind::Char => put::<u16>(p, &v)?,
        __FieldKind::Int => put::<i32>(p, &v)?,
        __FieldKind::Float => put::<f32>(p, &v)?,
        __FieldKind::Long => put::<i64>(p, &v)?,
        __FieldKind::Double => put::<f64>(p, &v)?,
        __FieldKind::Prim => return Err(bad_arg()),
        // SAFETY: 函数指针按该字段载体类型实例化，p 是该字段地址
        __FieldKind::Ref(op) => { unsafe { op(p, &mut __RefAccess::Set(Some(v))) }; }
    }
    Ok(Object::default())
}
