//! 反射 L3 分派协议（Method.invoke / Constructor.newInstance 的按名分派）。
//!
//! ## 协议形态（架构决策，2026-09-24）
//!
//! 反射调用边在编译期不可知：`Method.invoke` 拿到的 (声明类, 方法名, 描述符)
//! 运行时才能确定目标。协议 = **按类的分派闭包注册表**：
//!
//!   - codegen 为**用户树全部（非泛型）类**发射 per-class `__reflect_dispatch`
//!    （共置类文件尾部，match (name, descriptor) 臂调本类 typed fn：static
//!     直调 / 实例方法经 receiver 的 `try_cast::<Self>` 视图 / `<init>` 经
//!     `Self::new(..)`——视图保留运行时 vtable，虚覆盖自动生效）；
//!   - 生成项目 main 启动时登记（`register_method_dispatch`，与类初始化钩子
//!     / 注解工厂同一登记模式）；
//!   - `reflect_invoke` 按名代调：static/构造器在声明类上直查；实例方法从
//!     **receiver 运行时类**起沿直接父类表（CLASS_DIRECT_SUPER）上溯，首个
//!     处理该 (name, descriptor) 的闭包胜出——最派生覆盖优先，与 JVM 虚分派
//!     同序。
//!
//! ## 为什么不合流 ObjectVTable 臂（评估文档 2026-09-22 的候选 (a)）
//!
//! 候选 (a)（vtable 追加 `__method_invoke` 臂）对**实例方法**是自然的，但
//! static/构造器不走 vtable，仍需静态分发表——两套机制并存正是评估文档要
//! 避免的。本协议以「注册表 + typed 闭包」统一承载三类成员，发射面只在
//! codegen（用户类文件尾部），不触碰 java_rta_macros 的 vtable 生成核心
//!（禁改域）；闭包内的 `try_cast` 视图本身经既有 vtable 分派，虚方法语义
//! 仍然只走一套 vtable。JDK/lib 闭包类不发射分派闭包（无反射调用边的类不
//! 付代码税）——语料出现该缺口时按同一协议扩发射面，协议不变。
//!
//! 未命中（闭包缺席 / 方法形态未承载）→ panic stub（如实报出缺口，与反射
//! 族其他未覆盖面语义一致）。

use crate::error::Result;
use crate::java::lang::Object;
use crate::prelude::JArray;
use std::collections::HashMap;
use std::rc::Rc;

/// 按名分派闭包：(方法名, 描述符, 接收者, 实参) → 处理结果。
/// `None` = 本类未声明该方法（上溯继续）；`Some(r)` = 已处理（含错误传播）。
pub type ReflectDispatch =
    Rc<dyn Fn(&str, &str, Object, &JArray<Object>) -> Option<Result<Object>>>;

std::thread_local! {
    static DISPATCHERS: std::cell::RefCell<HashMap<String, ReflectDispatch>> =
        std::cell::RefCell::new(HashMap::new());
}

/// 生成项目 main 启动时登记分派闭包（binary name 斜线形态；重登记幂等）。
pub fn register_method_dispatch(dispatchers: &[(&str, ReflectDispatch)]) {
    DISPATCHERS.with(|d| {
        let mut d = d.borrow_mut();
        for (name, f) in dispatchers {
            d.insert((*name).to_owned(), Clone::clone(f));
        }
    });
}

fn lookup(class_slash: &str) -> Option<ReflectDispatch> {
    DISPATCHERS.with(|d| d.borrow().get(class_slash).map(Clone::clone))
}

/// 统一按名分派入口（Method.invoke / Constructor.newInstance 共用）。
///
/// `virtual`：true = 实例方法（从 receiver 运行时类上溯，receiver null →
/// NPE，与 JVM invoke0 的隐式 null 检查一致）；false = static / `<init>`
///（receiver 忽略，在声明类上直查）。
pub fn reflect_invoke(declaring_slash: &str, name: &str, descriptor: &str,
                      recv: Object, args: &JArray<Object>) -> Result<Object> {
    let is_ctor = name == "<init>";
    // 手写根类 Object 无 codegen 分派闭包：其唯一构造器 `<init>()V`（build.rs
    // 方法表补行，JLS §4.3.2）在此直接承载——新建一个独立身份的 Object 实例
    if is_ctor && declaring_slash == "java/lang/Object" && descriptor == "()V" {
        return Object::new();
    }
    let is_virtual = !is_ctor
        && !is_static_descriptor(declaring_slash, name, descriptor);
    // static / 构造器：声明类直查；实例方法：receiver 运行类起沿直接父类上溯
    //（隐式 null 检查：实例方法的 null receiver → NPE，与 JVM invoke0 一致）
    let mut cur = if is_virtual {
        if recv.0.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        recv.0.__class_name().to_owned()
    } else {
        declaring_slash.to_owned()
    };
    let mut hops = 0usize;
    loop {
        if let Some(f) = lookup(&cur) {
            if let Some(r) = f(name, descriptor, Clone::clone(&recv), args) {
                return r;
            }
        }
        // 上溯：直接父类表（Class.getSuperclass 的公共查询面，build.rs 生成）；
        // 无父类 / 表外（接口 / Object 上方）→ 终止
        let zuper = crate::java::lang::Class::for_class(
            crate::java::lang::String::from(cur.as_str()))
            .getSuperclass()
            .unwrap_or_default();
        if Object::from(Clone::clone(&zuper)).0.is_jvm_null() {
            break;
        }
        cur = format!("{}", zuper.__get_name()).replace('.', "/");
        hops += 1;
        if hops > 256 {
            break; // 防御环
        }
    }
    panic!("stub: L3 反射分派未覆盖 {}.{}:{}（分派闭包缺席 / 方法未发射）",
           declaring_slash, name, descriptor)
}

/// static 判定：经方法元数据表（Class.__declared_method_meta 的公共查询面）。
fn is_static_descriptor(class_slash: &str, name: &str, descriptor: &str) -> bool {
    let cls = crate::java::lang::Class::for_class(
        crate::java::lang::String::from(class_slash));
    cls.__declared_method_meta(name, descriptor)
        .map(|(_mods, is_static, _native, _abstract)| is_static)
        .unwrap_or(false)
}

// ── 实参/返回值的边界 marshalling（分派闭包发射侧共用）─────────────────────

/// 装箱 Object → i32（两形态：站点装箱原生盒 Rc<i32> / 翻译 Integer 包装——
/// 与 field_impl 的 __unbox_int 同一双路径策略，此处作为协议公共面）。
pub fn unbox_i32(v: &Object) -> Option<i32> {
    if v.0.is_jvm_null() {
        return None;
    }
    if let Some(b) = v.0.as_any().downcast_ref::<i32>() {
        return Some(*b);
    }
    if v.0.__class_name() == "java/lang/Integer" {
        return v.0.__obj_str().parse::<i32>().ok();
    }
    None
}

pub fn unbox_i64(v: &Object) -> Option<i64> {
    if v.0.is_jvm_null() {
        return None;
    }
    if let Some(b) = v.0.as_any().downcast_ref::<i64>() {
        return Some(*b);
    }
    if v.0.__class_name() == "java/lang/Long" {
        return v.0.__obj_str().parse::<i64>().ok();
    }
    None
}

pub fn unbox_bool(v: &Object) -> Option<bool> {
    if v.0.is_jvm_null() {
        return None;
    }
    if let Some(b) = v.0.as_any().downcast_ref::<bool>() {
        return Some(*b);
    }
    if v.0.__class_name() == "java/lang/Boolean" {
        return match v.0.__obj_str().as_str() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        };
    }
    None
}

pub fn unbox_f64(v: &Object) -> Option<f64> {
    if v.0.is_jvm_null() {
        return None;
    }
    if let Some(b) = v.0.as_any().downcast_ref::<f64>() {
        return Some(*b);
    }
    None
}

pub fn unbox_f32(v: &Object) -> Option<f32> {
    if v.0.is_jvm_null() {
        return None;
    }
    if let Some(b) = v.0.as_any().downcast_ref::<f32>() {
        return Some(*b);
    }
    None
}
