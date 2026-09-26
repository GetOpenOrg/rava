//! `java/lang/invoke/MethodHandle` 的 ACC_NATIVE 调用内核（MH-native，
//! docs/plans/2026-09-26-mh-native.md §二-3/4）。文件名取 `_ext`：`method_handle_impl.rs`
//! 被生成类 `MethodHandleImpl` 占用。
//!
//! ## 模型
//!
//! 句柄的语义完整编码在其 `LambdaForm`（`names[]` 表达式图）里——组合子（bindTo /
//! insertArguments / filterReturnValue / asType …）全部是 JDK 字节码构造的 LambdaForm，
//! 本文件不手写任何组合子。HotSpot 把 LambdaForm 编译为字节码后经 `vmentry` 执行；原生
//! 二进制不能定义类（`InvokerBytecodeGenerator` 为边界，vmentry 恒 null），改为**解释执行**：
//!
//! ```text
//! values = [mh, a1 … a(arity-1), 空 …]
//! for i in arity..names.len():  values[i] = eval(names[i])
//! return values[form.result]      // VOID_RESULT(-1) → null
//! ```
//!
//! `eval(name)` 按 `name.function`：
//! - member 为 `MethodHandle.invokeBasic` → 递归解释目标句柄；
//! - member 为 `MethodHandle.linkTo*` → 末参 MemberName 按 refKind 调用成员；
//! - 其他 member（JDK 辅助函数 / Unsafe 访问器 / 目标方法）→ 按 refKind 调用成员；
//! - 无 member → 解释其 `resolvedHandle`。
//!
//! 实参与返回值一律以 `Object` 流动（基本类型为装箱值）；成员调用经反射分派注册表
//! （`reflect_dispatch::reflect_invoke`，与 `Method.invoke` 同一协议）按描述符拆装箱。

use crate::prelude::*;
use super::method_handle::implref::MethodHandle;
use super::{LambdaForm_Name, LambdaForm_NamedFunction, MemberName, MethodType};
use crate::java::lang::Class;

// MethodHandleNatives.Constants 的 reference kind（JVMS §5.4.3.5）
const REF_GET_FIELD: i32 = 1;
const REF_GET_STATIC: i32 = 2;
const REF_PUT_FIELD: i32 = 3;
const REF_PUT_STATIC: i32 = 4;
const REF_NEW_INVOKE_SPECIAL: i32 = 8;

const MH_CLASS: &str = "java/lang/invoke/MethodHandle";
const NAME_CLASS: &str = "java/lang/invoke/LambdaForm$Name";
const UNSAFE_CLASS: &str = "jdk/internal/misc/Unsafe";

fn slash(cls: &Class) -> std::string::String {
    format!("{}", cls.__get_name()).replace('.', "/")
}

/// MemberName 的方法描述符（MethodType → toMethodDescriptorString；字段为类型描述符）。
fn member_descriptor(m: &MemberName) -> Result<std::string::String> {
    let t = m.__get_type_();
    if _is_jnull(&t) {
        return Ok(std::string::String::from("()V"));
    }
    if t.0.is_instance_of("java/lang/invoke/MethodType") {
        let mt = Clone::clone(&t).try_cast::<MethodType>("java/lang/invoke/MethodType")?;
        return Ok(format!("{}", mt.toMethodDescriptorString()?));
    }
    // 字段成员：type 为 Class
    let c = Clone::clone(&t).try_cast::<Class>("java/lang/Class")?;
    Ok(format!("{}", c.descriptorString()?))
}

/// 调用点描述符 → MethodType（类型经系统类加载器解析，与 javac 调用点的静态类型一致）。
fn site_type(site: &str) -> Result<MethodType> {
    MethodType::fromMethodDescriptorString(String::from(site), Default::default())
}

/// 解释执行句柄 `mh` 的 LambdaForm，`args` 为不含句柄自身的实参。
pub(crate) fn interpret(mh: MethodHandle, args: Vec<Object>) -> Result<Object> {
    let form = mh.__get_form();
    let names = form.__get_names();
    let n = names.len()? as usize;
    let arity = form.__get_arity() as usize;
    let mut values: Vec<Object> = vec![Object::default(); n.max(arity)];
    values[0] = Object::from(Clone::clone(&mh));
    for (i, a) in args.into_iter().enumerate() {
        if i + 1 < values.len() {
            values[i + 1] = a;
        }
    }
    for i in arity..n {
        let name = names.get(i as i32)?;
        let raw = name.__get_arguments();
        let mut argv: Vec<Object> = Vec::with_capacity(raw.len()? as usize);
        for k in 0..raw.len()? {
            let a = raw.get(k)?;
            if !_is_jnull(&a) && a.0.is_instance_of(NAME_CLASS) {
                let an = Clone::clone(&a).try_cast::<LambdaForm_Name>(NAME_CLASS)?;
                argv.push(Clone::clone(&values[an.__get_index() as usize]));
            } else {
                argv.push(a);
            }
        }
        values[i] = eval_function(&name.__get_function(), argv)?;
    }
    let result = form.__get_result();
    if result < 0 {
        return Ok(Object::default());
    }
    Ok(Clone::clone(&values[result as usize]))
}

fn eval_function(f: &LambdaForm_NamedFunction, argv: Vec<Object>) -> Result<Object> {
    let member = f.__get_member();
    if !_is_jnull(&Object::from(Clone::clone(&member))) {
        let cls = slash(&member.__get_clazz());
        let name = format!("{}", member.__get_name());
        if cls == MH_CLASS {
            match name.as_str() {
                "invokeBasic" | "invokeExact" | "invoke" => {
                    let mut it = argv.into_iter();
                    let target = it.next().unwrap_or_default()
                        .try_cast::<MethodHandle>(MH_CLASS)?;
                    return interpret(target, it.collect());
                }
                "linkToStatic" | "linkToVirtual" | "linkToSpecial" | "linkToInterface" => {
                    return link_to(argv);
                }
                _ => {}
            }
        }
        return invoke_member(&member, argv);
    }
    let mut h = f.__get_resolvedHandle();
    if _is_jnull(&Object::from(Clone::clone(&h))) {
        h = f.resolvedHandle()?;
    }
    interpret(h, argv)
}

/// `linkTo*(a1 … an, MemberName)`：末参为目标成员。
fn link_to(mut argv: Vec<Object>) -> Result<Object> {
    let last = argv.pop().unwrap_or_default();
    let member = last.try_cast::<MemberName>("java/lang/invoke/MemberName")?;
    invoke_member(&member, argv)
}

/// 按 MemberName 的 refKind 调用成员：字段访问 / 静态 / 虚 / 特殊 / 构造。
pub(crate) fn invoke_member(m: &MemberName, argv: Vec<Object>) -> Result<Object> {
    let cls = slash(&m.__get_clazz());
    let name = format!("{}", m.__get_name());
    let ref_kind = (m.__get_flags() >> 24) & 15;
    if cls == UNSAFE_CLASS {
        return invoke_unsafe(&name, argv);
    }
    let descriptor = member_descriptor(m)?;
    match ref_kind {
        REF_GET_FIELD | REF_GET_STATIC | REF_PUT_FIELD | REF_PUT_STATIC => {
            invoke_field(m, ref_kind, argv)
        }
        REF_NEW_INVOKE_SPECIAL => {
            let arr = JArray::from(argv);
            crate::reflect_dispatch::reflect_invoke(&cls, "<init>", &descriptor, Object::default(), &arr)
        }
        _ => {
            // static（6）：实参全体；virtual / interface / special（5/9/7）：首参为接收者
            let is_static = ref_kind == 6;
            let (recv, rest) = if is_static {
                (Object::default(), argv)
            } else {
                let mut it = argv.into_iter();
                (it.next().unwrap_or_default(), it.collect())
            };
            let arr = JArray::from(rest);
            crate::reflect_dispatch::reflect_invoke(&cls, &name, &descriptor, recv, &arr)
        }
    }
}

/// 字段 refKind：按名字段协议（reflect_field，与 Field.get/set 同一闭包）。
fn invoke_field(m: &MemberName, ref_kind: i32, argv: Vec<Object>) -> Result<Object> {
    let cls = slash(&m.__get_clazz());
    let name = format!("{}", m.__get_name());
    let is_static = ref_kind == REF_GET_STATIC || ref_kind == REF_PUT_STATIC;
    let mut it = argv.into_iter();
    let recv = if is_static { Object::default() } else { it.next().unwrap_or_default() };
    if !is_static && _is_jnull(&recv) {
        return Err(JvmError::null_pointer());
    }
    let value = if ref_kind == REF_PUT_FIELD || ref_kind == REF_PUT_STATIC { Some(it.next().unwrap_or_default()) } else { None };
    match crate::reflect_dispatch::reflect_field(&cls, &name, recv, value) {
        Some(r) => r,
        None => panic!("stub: MH-native 字段句柄未覆盖 {}.{}（字段闭包缺席）", cls, name),
    }
}

/// LambdaForm 里的 Unsafe 访问器成员（DMH 字段访问形态：`UNSAFE.getInt(base, offset)` 等）。
/// Unsafe 为手写边界类，无生成分派臂——在此按名直连手写实现。
fn invoke_unsafe(name: &str, argv: Vec<Object>) -> Result<Object> {
    let u = crate::jdk::internal::misc::Unsafe::getUnsafe()?;
    let mut it = argv.into_iter();
    // 虚成员：首参是 Unsafe 实例自身
    let _self = it.next();
    let base = it.next().unwrap_or_default();
    let off = crate::reflect_dispatch::unbox_i64(&it.next().unwrap_or_default())
        .ok_or_else(crate::reflect_dispatch::bad_arg)?;
    let val = it.next();
    // 字段身份可还原（objectFieldOffset / staticFieldOffset 登记表）→ 按名字段协议
    if let Some((cls, fname)) = crate::jdk::internal::misc::Unsafe::__field_of_offset(off) {
        let is_static = !_is_jnull(&base) && base.0.is_instance_of("java/lang/Class");
        let recv = if is_static { Object::default() } else { Clone::clone(&base) };
        let value = if name.starts_with("put") { Some(Clone::clone(val.as_ref().unwrap_or(&Object::default()))) } else { None };
        if let Some(r) = crate::reflect_dispatch::reflect_field(&cls, &fname, recv, value) {
            return r;
        }
    }
    let get_int = |u: &crate::jdk::internal::misc::Unsafe| u.getInt_obj_l(Clone::clone(&base), off);
    match name {
        "getInt" | "getIntVolatile" | "getIntAcquire" | "getIntOpaque" => Ok(Object::from(get_int(&u)?)),
        "getBoolean" | "getBooleanVolatile" => Ok(Object::from(get_int(&u)? != 0)),
        "getShort" | "getShortVolatile" => Ok(Object::from(get_int(&u)? as i16)),
        "getByte" | "getByteVolatile" => Ok(Object::from(get_int(&u)? as i8)),
        "getChar" | "getCharVolatile" => Ok(Object::from(get_int(&u)? as u16)),
        "getLong" | "getLongVolatile" | "getLongAcquire" | "getLongOpaque" => {
            Ok(Object::from(u.getLong_obj_l(base, off)?))
        }
        "getReference" | "getReferenceVolatile" | "getReferenceAcquire" | "getReferenceOpaque" => {
            u.getReference(base, off)
        }
        "putInt" | "putIntVolatile" | "putIntRelease" | "putIntOpaque"
        | "putBoolean" | "putBooleanVolatile" | "putShort" | "putShortVolatile"
        | "putByte" | "putByteVolatile" | "putChar" | "putCharVolatile" => {
            let v = val.unwrap_or_default();
            let iv = crate::reflect_dispatch::unbox_i32(&v)
                .or_else(|| crate::reflect_dispatch::unbox_bool(&v).map(|b| b as i32))
                .ok_or_else(crate::reflect_dispatch::bad_arg)?;
            u.putInt_obj_l_i(base, off, iv)?;
            Ok(Object::default())
        }
        "putLong" | "putLongVolatile" | "putLongRelease" | "putLongOpaque" => {
            let lv = crate::reflect_dispatch::unbox_i64(&val.unwrap_or_default())
                .ok_or_else(crate::reflect_dispatch::bad_arg)?;
            u.putLong_obj_l_l(base, off, lv)?;
            Ok(Object::default())
        }
        "putReference" | "putReferenceVolatile" | "putReferenceRelease" | "putReferenceOpaque" => {
            u.putReference(base, off, val.unwrap_or_default())?;
            Ok(Object::default())
        }
        _ => panic!("stub: MH-native 解释器未承载的 Unsafe 成员 {}", name),
    }
}

impl MethodHandle {
    /// native `invokeBasic(Object...)`：解释执行本句柄的 LambdaForm（无类型检查——
    /// 调用方保证基本类型形态一致，JDK 语义）。
    #[jvm_native]
    pub fn invokeBasic(&self, args: JArray<Object>) -> Result<Object> {
        interpret(Clone::clone(self), args.to_vec())
    }

    /// native `invokeExact(Object...)`：无调用点类型的入口（翻译字节码内部调用）按 invokeBasic
    /// 执行。用户调用点经 codegen 发 `invokeExact__site`（清单 sigpoly_callsite.txt）。
    ///
    /// upcalls：调用点类型检查 / 适配用到的 JDK 方法（运行时 → Java 调用边，字节码不可见）。
    #[jvm_native(upcalls = "java/lang/invoke/MethodType.fromMethodDescriptorString:(Ljava/lang/String;Ljava/lang/ClassLoader;)Ljava/lang/invoke/MethodType; java/lang/invoke/MethodType.equals:(Ljava/lang/invoke/MethodType;)Z java/lang/invoke/MethodType.toString:()Ljava/lang/String; java/lang/invoke/MethodHandle.asType:(Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle; java/lang/invoke/WrongMethodTypeException.<init>:(Ljava/lang/String;)V java/lang/Integer.toString:()Ljava/lang/String; java/lang/Long.toString:()Ljava/lang/String; java/lang/Short.toString:()Ljava/lang/String; java/lang/Byte.toString:()Ljava/lang/String; java/lang/Character.toString:()Ljava/lang/String; java/lang/Boolean.toString:()Ljava/lang/String; java/lang/Float.toString:()Ljava/lang/String; java/lang/Double.toString:()Ljava/lang/String;")]
    pub fn invokeExact(&self, args: JArray<Object>) -> Result<Object> {
        interpret(Clone::clone(self), args.to_vec())
    }

    /// native `invoke(Object...)`：同上（用户调用点经 `invoke__site`）。
    #[jvm_native]
    pub fn invoke(&self, args: JArray<Object>) -> Result<Object> {
        interpret(Clone::clone(self), args.to_vec())
    }

    /// 调用点带类型的 `invokeExact`：句柄类型必须与调用点 MethodType 完全相同
    ///（JVMS §5.4.3.4 / MethodHandle.invokeExact 规范），否则 WrongMethodTypeException。
    pub fn invokeExact__site(&self, site: &str, args: JArray<Object>) -> Result<Object> {
        let call_type = site_type(site)?;
        let own = self.type_()?;
        if !own.equals_methodtype(Clone::clone(&call_type))? {
            return Err(JvmError::from(super::WrongMethodTypeException::new_str(String::from(
                format!("handle's method type {} but found {}", own.toString()?, call_type.toString()?).as_str()))?));
        }
        interpret(Clone::clone(self), args.to_vec())
    }

    /// 调用点带类型的 `invoke`：类型不同 → `asType(调用点类型)` 的适配句柄（装拆箱 / 拓宽 /
    /// 变参收集等全部由 JDK asType 字节码构造）。
    pub fn invoke__site(&self, site: &str, args: JArray<Object>) -> Result<Object> {
        let call_type = site_type(site)?;
        let own = self.type_()?;
        if own.equals_methodtype(Clone::clone(&call_type))? {
            return interpret(Clone::clone(self), args.to_vec());
        }
        let adapted = self.asType(call_type)?;
        interpret(adapted, args.to_vec())
    }

    #[jvm_native]
    pub fn linkToStatic(args: JArray<Object>) -> Result<Object> {
        link_to(args.to_vec())
    }

    #[jvm_native]
    pub fn linkToVirtual(args: JArray<Object>) -> Result<Object> {
        link_to(args.to_vec())
    }

    #[jvm_native]
    pub fn linkToSpecial(args: JArray<Object>) -> Result<Object> {
        link_to(args.to_vec())
    }

    #[jvm_native]
    pub fn linkToInterface(args: JArray<Object>) -> Result<Object> {
        link_to(args.to_vec())
    }
}
