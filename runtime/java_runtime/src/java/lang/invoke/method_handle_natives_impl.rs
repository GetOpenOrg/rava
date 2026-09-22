//! `java/lang/invoke/MethodHandleNatives` 手写伴生：公开 API 类的 ACC_NATIVE
//! 方法（K-3a 共置形态）。HotSpot 里本类是 method handle 机制与 VM 的 JNI
//! 边界；原生二进制没有 VM 元数据层——resolve 族按「Rust 侧静态注册表」
//! 承载（build.rs 方法/字段元数据表，与 Class.getDeclaredField 同源）。

use crate::prelude::*;
use super::method_handle_natives::MethodHandleNatives;
use super::member_name::MemberName;
use super::method_type::MethodType;
use crate::java::lang::Class;
use crate::java::lang::NoSuchFieldError;
use crate::java::lang::NoSuchMethodError;

/// MemberName flags 的 MN_ 常量（JVMS 之外的 HotSpot 私有布局，
/// MethodHandleNatives.java 与 member_name.rs 的 flagsMods 同一编码）：
/// 低 16 位 = java.lang.reflect.Modifier 位集；bits 16-19 = 成员类别；
/// bits 24-27 = reference kind（REF_getField=1 … REF_invokeInterface=9）。
const MN_IS_METHOD: i32 = 0x0001_0000;
const MN_IS_CONSTRUCTOR: i32 = 0x0002_0000;
const MN_IS_FIELD: i32 = 0x0004_0000;

impl MethodHandleNatives {
    /// native `registerNatives()`：HotSpot 绑定 JNI 入口；原生二进制无此需要。
    #[jvm_native]
    pub fn registerNatives() -> Result<()> {
        Ok(())
    }

    /// native `resolve(MemberName, Class lookupClass, int allowedModes, boolean
    /// speculativeResolve)`：MemberName 解析内核——把「符号引用」(声明类,
    /// 名字, 类型, refKind) 对到具体成员并回填修饰位。
    ///
    /// HotSpot 查 VM 元数据（方法/字段在类结构里的真实槽位）；原生二进制
    /// 的对应物是 build.rs 从 java_field / java_method / java_native 属性
    /// 生成的静态元数据表（字段/方法声明元数据的唯一表达，规则四）：
    ///   - 字段 kind（refKind 1-4）：按 (声明类, 字段名) 查字段表，类型按
    ///     描述符还原 Class 与 MemberName 携带的 Class 名等核对，static
    ///     性与 refKind（getStatic/putStatic ↔ static 字段）核对；
    ///   - 方法/构造器 kind（refKind 5-9）：按 (声明类, 名字, 描述符) 查
    ///     方法表（MethodType 经 toMethodDescriptorString 还原描述符——重载
    ///     语义下 name 不唯一）。
    /// 命中：flags 回填 = MN 类别位 | refKind<<24 | 修饰位（低 16 位），
    /// resolution 由调用方（MemberName$Factory.resolve）置空。
    /// 未命中：非 speculative → Err(NoSuchFieldError / NoSuchMethodError)
    /// ——Factory 的 catch (LinkageError) 捕获后挂到 resolution 再返回，
    /// resolveOrFail 经 makeAccessException 还原 NoSuchField(/Method)
    /// Exception（与 JDK 解析失败同型）；speculative → Ok(null MemberName)。
    #[jvm_native]
    pub fn resolve(m: MemberName, _lookupClass: Class, _allowedModes: i32, speculativeResolve: bool) -> Result<MemberName> {
        let flags = m.__get_flags();
        let ref_kind: i8 = (((flags >> 24) & 15) as i8);
        if ref_kind >= 1 && ref_kind <= 4 {
            return Self::resolve_field(m, ref_kind, speculativeResolve);
        }
        Self::resolve_method(m, ref_kind, speculativeResolve)
    }

    /// 字段 kind 解析。类型核对按名（Class 身份经 for_class / getPrimitiveClass
    /// 线程内缓存，同名即同实例；名字比较对两形态都成立）。
    fn resolve_field(m: MemberName, ref_kind: i8, speculative: bool) -> Result<MemberName> {
        // 未命中 / 类型不符 / static 性不符 → NoSuchFieldError（LinkageError 族，
        // Factory 的 catch 捕获后挂 resolution——makeAccessException 据此还原
        // NoSuchFieldException）
        let not_found = |m: &MemberName| -> Result<MemberName> {
            if speculative {
                return Ok(MemberName::default());
            }
            Err(JvmError::from(NoSuchFieldError::new_str(Clone::clone(&m.__get_name()))?))
        };
        let name = format!("{}", m.__get_name());
        let Some((descriptor, is_static, modifiers, _)) =
            m.__get_clazz().__declared_field_meta(&name)
        else {
            return not_found(&m);
        };
        // 类型核对：MemberName.type（Class）与表内描述符还原的 Class 名等
        let declared_type = Class::__class_for_descriptor(descriptor);
        let Ok(mtype) = Clone::clone(&m.__get_type_()).try_cast::<Class>("java/lang/Class") else {
            return not_found(&m);
        };
        if format!("{}", mtype.__get_name()) != format!("{}", declared_type.__get_name()) {
            return not_found(&m);
        }
        // static 性核对：getStatic/putStatic ↔ static 字段（JDK 不符即
        // IncompatibleClassChangeError，此处按 NoSuchFieldError 简并——
        // Factory catch 与 makeAccessException 的路径完全同型）
        let static_kind = ref_kind == 2 || ref_kind == 4;
        if static_kind != is_static {
            return not_found(&m);
        }
        m.__set_flags(MN_IS_FIELD | ((ref_kind as i32) << 24) | (modifiers & 0xFFFF));
        Ok(m)
    }

    /// 方法/构造器 kind 解析。MemberName.type 是 MethodType（invokeVirtual 等）
    /// 或 null（构造器重载里也可能是 MethodType——newInvokeSpecial 的描述符
    /// 是构造器描述符）；身份键 (name, descriptor) 配对。
    fn resolve_method(m: MemberName, ref_kind: i8, speculative: bool) -> Result<MemberName> {
        // 未命中 → NoSuchMethodError（同 resolve_field 的 Factory catch 协议）
        let not_found = |m: &MemberName| -> Result<MemberName> {
            if speculative {
                return Ok(MemberName::default());
            }
            Err(JvmError::from(NoSuchMethodError::new_str(Clone::clone(&m.__get_name()))?))
        };
        let name = format!("{}", m.__get_name());
        let type_ = m.__get_type_();
        let descriptor: std::string::String = if _is_jnull(&type_) {
            // 构造器只有一种形态来源：MethodType；null 视为 "()V" 兜底
            std::string::String::from("()V")
        } else {
            let Ok(mt) = Clone::clone(&type_).try_cast::<MethodType>("java/lang/invoke/MethodType") else {
                return not_found(&m);
            };
            format!("{}", mt.toMethodDescriptorString()?)
        };
        let Some((modifiers, _is_static, _is_native, _is_abstract)) =
            m.__get_clazz().__declared_method_meta(&name, &descriptor)
        else {
            return not_found(&m);
        };
        // 类别位保留构造时的 MN_IS_METHOD / MN_IS_CONSTRUCTOR
        let kind_bits = m.__get_flags() & (MN_IS_METHOD | MN_IS_CONSTRUCTOR);
        m.__set_flags(kind_bits | ((ref_kind as i32) << 24) | (modifiers & 0xFFFF));
        Ok(m)
    }

    /// native `objectFieldOffset(MemberName)`：实例字段的偏移量。HotSpot 返回
    /// 对象布局真实偏移；原生二进制字段经名字访问，偏移只作不透明标识——
    /// 与 Unsafe.objectFieldOffset(Class, String) / (Field) 共用同一登记表
    /// （JDK 三路径对同一字段同值；VarHandle 的访问器与 Unsafe 原子族因
    /// 此对同一存储单元达成一致）。
    #[jvm_native]
    pub fn objectFieldOffset(m: MemberName) -> Result<i64> {
        crate::jdk::internal::misc::Unsafe::getUnsafe()?
            .objectFieldOffset_class_str(Clone::clone(&m.__get_clazz()), Clone::clone(&m.__get_name()))
    }
}
