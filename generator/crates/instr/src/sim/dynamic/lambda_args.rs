//! lambda 闭包内调用实现方法的实参表：捕获值 + SAM 实参，按函数式接口擦除签名（samtype）
//! 与实现方法签名之间的差异逐个适配（载体包装 / 拆箱 / checkcast / 类型变量视图 / 装箱 /
//! 跨实例化重建），接收者以 `&recv` 传入。

use ty::RsType;

use super::boxing::{is_prim_text, obj_text, unbox_object_arg};
use super::lambda::{rust_text, Lam};
use super::wrapper_of;
use crate::build::{id, ir_ty, text, ty_text};
use crate::coerce::{cast_node, same_generic_family};
use crate::env::InstrEnv;
use crate::error::InstrResult;

/// 闭包内调用实现方法的实参文本
pub(super) struct CallArgs {
    /// 捕获值部分（绑定接收者时首项为接收者）
    pub cap: Vec<String>,
    /// SAM 实参部分（未绑定接收者的方法引用时首项为接收者）
    pub sam: Vec<String>,
}

/// 载体类型文本（`carrier_type_for_ident`：已铺设接口载体 → 载体名，否则 None）
fn carrier_text(env: &InstrEnv, t: &RsType) -> Option<String> {
    env.ctx.ty.carrier_type_for_ident(t).map(|c| ty_text(env, &c))
}

/// SAM 实参 → 实现方法形参的适配（`pi` 为对应的实现方法形参下标）
fn adapt_sam_arg(env: &InstrEnv, lam: &Lam, si: usize, pi: usize) -> InstrResult<Option<String>> {
    let (sd, pd) = (lam.sam_params[si].as_str(), lam.impl_params[pi].as_str());
    let name = lam.sam_names[si].as_str();
    let pd_t = env.ctx.ty.jvm_to_rust(pd);
    let pd_rust = ty_text(env, &pd_t);
    let erased_sd = lam.is_erased_ref(env, sd);
    if let Some(carrier) = carrier_text(env, &pd_t).filter(|c| *c == pd_rust && pd != sd && erased_sd) {
        // 实现方法形参是已铺设接口载体：SAM 擦除实参经载体的 From<Object> 非受检包装
        // （UFCS 不被接口自带静态 from 工厂遮蔽）
        return Ok(Some(format!("<{carrier} as ::std::convert::From<{}>>::from({name})", ir::anchors::OBJECT)));
    }
    if pd != sd && erased_sd && !lam.is_erased_ref(env, pd) {
        if wrapper_of(pd).is_some() {
            // SAM 擦除实参是装箱对象、实现方法形参是基本类型（`Integer::sum` 适配
            // BinaryOperator）：cast 到装箱类后调拆箱方法
            return Ok(Some(unbox_object_arg(env, name, pd)?.unwrap_or_else(|| name.to_string())));
        }
        // SAM 擦除实参 → 实现方法具体形参：checkcast 语义（try_cast 失败返回 Err 可被
        // java_try 捕获，S-1；目标类型由形参推断，turbofish 不写）
        let bin = if pd.starts_with('[') { pd } else { pd.get(1..pd.len().saturating_sub(1)).unwrap_or("") };
        return Ok(Some(format!("{name}.try_cast(\"{bin}\")?")));
    }
    if erased_sd {
        // 实现类不在注册表（手写根类）：无声明类类型变量可取，跳过本判定
        if lam.ci.is_some_and(|ci| !ci.is_interface())
            && lam.sig_types.len() == lam.impl_params.len()
            && lam.tparams.contains(&ty_text(env, &lam.sig_types[pi]))
        {
            // 实现方法形参是声明类的类型变量（`this::addLast`）：SAM 擦除实参经宏补的
            // From<Object> bound 取回类型变量视图
            return Ok(Some(format!("From::from({name})")));
        }
    }
    if sd.len() == 1 && pd.len() == 1 && sd != pd && sd != "Z" && pd != "Z" {
        // 基本类型拓宽（metafactory 允许的 widening：`AtomicLong::new` 适配 IntFunction，I → J）；
        // Rust `as` 与 Java 拓宽同义（整数符号扩展、char 零扩展、整数 → 浮点就近舍入）
        return Ok(Some(format!("({name} as {pd_rust})")));
    }
    if sd.len() == 1 && pd_rust == ir::anchors::OBJECT {
        // SAM 实参是基本类型、实现方法形参是引用（metafactory 的装箱适配）
        return Ok(Some(format!("{name}.into()")));
    }
    Ok(None)
}

/// 捕获值 → 实现方法形参（`cp` 为形参下标）的适配：形参是载体 / 擦除引用而捕获值是具体
/// 类实例 → 隐式上转（保持对象标识）；形参是另一实例化 → 经 Object 边界重建
fn adapt_cap_arg(env: &InstrEnv, lam: &Lam, ci_idx: usize, cp: usize) -> InstrResult<Option<String>> {
    let Some(pd) = lam.impl_params.get(cp) else {
        return Ok(None);
    };
    let cap = lam.cap_names[ci_idx].as_str();
    let cap_t = &lam.cap_tys[ci_idx];
    let cap_ty = ty_text(env, cap_t);
    let obj = ir::anchors::OBJECT;
    let cp_t = env.ctx.ty.jvm_to_rust(pd);
    let cp_rust = ty_text(env, &cp_t);
    if let Some(carrier) = carrier_text(env, &cp_t).filter(|c| *c == cp_rust) {
        // 实现方法形参是已铺设接口载体：捕获值已是同载体 → 保持 Clone；是 Object / 具体类 →
        // 经 Object 边界的非受检包装（Fn 闭包可多次调用：Clone 取值，不 move 捕获变量）
        if cap_ty == obj {
            return Ok(Some(format!("<{carrier} as ::std::convert::From<_>>::from(Clone::clone(&{cap}))")));
        }
        if cap_ty != "()" && cap_ty != "_" && cap_ty != carrier {
            return Ok(Some(format!("<{carrier} as ::std::convert::From<_>>::from({})", obj_text(env, cap, cap_t)?)));
        }
        return Ok(None);
    }
    let array_prefix = format!("{}<", ir::anchors::ARRAY);
    if lam.is_erased_ref(env, pd)
        && ![obj, "()", "_"].contains(&cap_ty.as_str())
        && !is_prim_text(&cap_ty)
        && !cap_ty.starts_with(array_prefix.as_str())
        && !cap_ty.starts_with("Vec<")
        && !cap_ty.starts_with('&')
    {
        return Ok(Some(obj_text(env, cap, cap_t)?));
    }
    // 合成 lambda 方法的形参是擦除实例化（X<Object, Object>），捕获值是精确实例化：经 Object
    // 边界重新实例化。形参类型与定义侧同源：有泛型签名取签名，否则取描述符擦除形态
    let expected =
        if lam.sig_types.len() == lam.impl_params.len() { lam.sig_types[cp].clone() } else { cp_t };
    if same_generic_family(env, cap_t, &expected) {
        let node = cast_node(ir::Expr::Var(id(cap)?), ir_ty(env, &expected)?, "", false, true);
        return Ok(Some(text(env, &node)));
    }
    Ok(None)
}

/// 实参表：SAM 实参与捕获值逐个适配后，接收者改为 `&recv` 形态
pub(super) fn call_args(env: &InstrEnv, lam: &Lam) -> InstrResult<CallArgs> {
    let mut cap: Vec<String> = lam.cap_names.iter().map(|v| format!("Clone::clone(&{v})")).collect();
    let mut sam: Vec<String> = lam.sam_names.clone();
    let has_caps = !cap.is_empty();
    // 实现方法形参表对应 (捕获值 + SAM 实参) 去掉接收者之后的部分
    let recv_from_sam = lam.is_instance && !has_caps;
    let offset = cap.len() as i64 - i64::from(lam.is_instance && has_caps);
    for (si, slot) in sam.iter_mut().enumerate() {
        if recv_from_sam && si == 0 {
            continue;
        }
        let pi = offset + si as i64 - i64::from(recv_from_sam);
        let Ok(pi) = usize::try_from(pi) else {
            continue;
        };
        if pi < lam.impl_params.len() {
            if let Some(a) = adapt_sam_arg(env, lam, si, pi)? {
                *slot = a;
            }
        }
    }
    let cap_recv = usize::from(lam.is_instance && has_caps);
    for (ci_idx, slot) in cap.iter_mut().enumerate().skip(cap_recv) {
        if let Some(a) = adapt_cap_arg(env, lam, ci_idx, ci_idx - cap_recv)? {
            *slot = a;
        }
    }
    if lam.is_instance && has_caps {
        // 捕获 this 的 lambda / 绑定接收者的方法引用：第一个捕获值是接收者
        cap[0] = if lam.ci.is_none() && !lam.is_ctor {
            // 实现类在运行时手写层：UFCS 第一实参是 &self（实现类形态）——捕获接收者是调用点
            // 子类实例，先经 Object 边界上转（保持对象身份，虚分派在 vtable 上进行）
            format!("&{}::from(Clone::clone(&{}))", ir::anchors::OBJECT, lam.cap_names[0])
        } else {
            format!("&{}", lam.cap_names[0])
        };
    } else if recv_from_sam && !sam.is_empty() {
        // 未绑定接收者的方法引用（X::method）：第一个 SAM 实参是接收者
        let recv_desc = format!("L{};", lam.impl_cls);
        sam[0] = if lam.is_erased_ref(env, &lam.sam_params[0]) && !lam.is_erased_ref(env, &recv_desc) {
            // 擦除的 SAM 接收者 → 实现类具体接收者：checkcast 语义（S-1）
            format!("&{}.try_cast::<{}>(\"{}\")?", lam.sam_names[0], rust_text(env, &recv_desc), lam.impl_cls)
        } else {
            format!("&{}", lam.sam_names[0])
        };
    }
    Ok(CallArgs { cap, sam })
}
