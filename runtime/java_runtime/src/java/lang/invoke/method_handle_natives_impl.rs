//! `java/lang/invoke/MethodHandleNatives` 手写伴生：公开 API 类的 ACC_NATIVE
//! 方法（K-3a 共置形态）。HotSpot 里本类是 method handle 机制与 VM 的 JNI
//! 边界；原生二进制没有 VM 元数据层——resolve 族按「Rust 侧静态注册表」
//! 承载（java_meta 方法/字段元数据表，与 Class.getDeclaredField 同源）。

use crate::prelude::*;
use super::method_handle_natives::MethodHandleNatives;
use super::member_name::MemberName;
use super::method_type::MethodType;
use crate::java::lang::Class;
use crate::java::lang::NoSuchFieldError;
use crate::java::lang::NoSuchMethodError;

fn _internal_error(msg: &str) -> JvmError {
    match crate::java::lang::InternalError::new_str(String::from(msg)) {
        Ok(e) => JvmError::from(e),
        Err(e) => e,
    }
}

/// MemberName flags 的 MN_ 常量（JVMS 之外的 HotSpot 私有布局，
/// MethodHandleNatives.java 与 member_name.rs 的 flagsMods 同一编码）：
/// 低 16 位 = java.lang.reflect.Modifier 位集；bits 16-19 = 成员类别；
/// bits 24-27 = reference kind（REF_getField=1 … REF_invokeInterface=9）。
const MN_IS_METHOD: i32 = 0x0001_0000;
const MN_IS_CONSTRUCTOR: i32 = 0x0002_0000;
const MN_IS_FIELD: i32 = 0x0004_0000;
/// `@jdk.internal.reflect.CallerSensitive` 方法（HotSpot `init_method_MemberName` 与 resolve 同置）：
/// `Lookup.findBoundCallerLookup` 据此拒绝受限 lookup，`maybeBindCaller` 据此经
/// `MethodHandleImpl.bindCaller` 以 lookup 类为调用者（适配器或注入调用器）。
const MN_CALLER_SENSITIVE: i32 = 0x0010_0000;

/// 声明方法 (类, 名, 描述符) 带 @CallerSensitive → MN_CALLER_SENSITIVE，否则 0。
fn caller_sensitive_bit(class_slash: &str, name: &str, descriptor: &str) -> i32 {
    crate::meta::class_methods().iter()
        .find(|(n, _)| *n == class_slash)
        .and_then(|(_, ms)| ms.iter().find(|m| m.name == name && m.descriptor == descriptor))
        .filter(|m| crate::anno_pool::has_annotation(class_slash, m.annotations, "Ljdk/internal/reflect/CallerSensitive;"))
        .map_or(0, |_| MN_CALLER_SENSITIVE)
}

impl MethodHandleNatives {
    /// native `registerNatives()`：HotSpot 绑定 JNI 入口；原生二进制无此需要。
    #[jvm_native]
    pub fn registerNatives() -> Result<()> {
        Ok(())
    }

    /// native `init(MemberName self, Object ref)`：以反射对象（Constructor / Method / Field）
    /// 初始化 MemberName（`MemberName(Constructor|Method|Field)` 构造器的 VM 底座，
    /// `Lookup.unreflect*` 消费——record 序列化的 canonicalRecordCtr 即此路径）。
    ///
    /// HotSpot 填 clazz / flags / vmtarget，name 与 type 由调用方或 expand 惰性补齐；此处
    /// 直接读反射对象字段（与 JDK 反射对象同源的声明元数据）：
    ///   - 构造器：clazz、flags = IS_CONSTRUCTOR | REF_newInvokeSpecial | 修饰位（name / type
    ///     由 MemberName 构造器在 init 之后按 `<init>` 与参数类型补齐）；
    ///   - 方法：clazz、name、type = `Object[]{返回类型, 参数类型[]}`（getMethodType 认可的
    ///     惰性形态）、flags = IS_METHOD | refKind（static → invokeStatic；接口 → invokeInterface；
    ///     private → invokeSpecial；其余 invokeVirtual）| 修饰位；
    ///   - 字段：clazz、name、type = 字段类型、flags = IS_FIELD | refKind（getStatic / getField）| 修饰位。
    #[jvm_native]
    pub fn init(m: MemberName, reference: Object) -> Result<()> {
        // 经 ObjectVTable 的按名字段协议读反射对象（不静态引用反射类型：本 impl 在
        // MethodHandleNatives 生成时即参与编译，反射类未必在闭包内）
        const ACC_STATIC: i32 = 0x0008;
        const ACC_PRIVATE: i32 = 0x0002;
        let stub = |what: &str| -> ! {
            panic!("stub: java/lang/invoke/MethodHandleNatives.init:(Ljava/lang/invoke/MemberName;Ljava/lang/Object;)V（{}）", what)
        };
        let ref_field = |name: &str| -> Object {
            reference.0.__unsafe_ref_get(name).unwrap_or_else(|| stub(name))
        };
        let mods = reference.0.__unsafe_int_cell("modifiers").map(|c| c.get()).unwrap_or_else(|| stub("modifiers"));
        // 声明类按名读字面量字段名：分析器据此把写入 MemberName.clazz 的值接为反射对象 clazz 字段的内容
        let clazz: Class = reference.0.__unsafe_ref_get("clazz").unwrap_or_else(|| stub("clazz")).try_cast::<Class>("java/lang/Class")?;
        match reference.0.__class_name() {
            "java/lang/reflect/Constructor" => {
                m.__set_clazz(clazz);
                m.__set_flags(MN_IS_CONSTRUCTOR | (8 << 24) | (mods & 0xFFFF));
            }
            "java/lang/reflect/Method" => {
                let ref_kind = if mods & ACC_STATIC != 0 {
                    6
                } else if clazz.isInterface()? {
                    9
                } else if mods & ACC_PRIVATE != 0 {
                    7
                } else {
                    5
                };
                let type_info: JArray<Object> =
                    JArray::from(vec![ref_field("returnType"), ref_field("parameterTypes")]);
                // @CallerSensitive：反射对象的原始注解字节（Method.annotations，与 isCallerSensitive 同源）
                let owner = format!("{}", clazz.__get_name()).replace('.', "/");
                let cs = reference.0.__unsafe_ref_get("annotations")
                    .filter(|a| !a.0.is_jvm_null())
                    .map(|a| JArray::<i8>::from(a).to_vec().into_iter().map(|b| b as u8).collect::<Vec<u8>>())
                    .filter(|raw| crate::anno_pool::has_annotation(&owner, raw, "Ljdk/internal/reflect/CallerSensitive;"))
                    .map_or(0, |_| MN_CALLER_SENSITIVE);
                m.__set_clazz(clazz);
                m.__set_name(ref_field("name").try_cast::<String>("java/lang/String")?);
                m.__set_type_(Object::from(type_info));
                m.__set_flags(MN_IS_METHOD | cs | (ref_kind << 24) | (mods & 0xFFFF));
            }
            "java/lang/reflect/Field" => {
                let ref_kind = if mods & ACC_STATIC != 0 { 2 } else { 1 };
                m.__set_clazz(clazz);
                m.__set_name(ref_field("name").try_cast::<String>("java/lang/String")?);
                // Java 字段名 `type` 在 Rust 侧为关键字转义 `type_`（按名协议的臂取 Rust 标识符）
                let ty = reference.0.__unsafe_ref_get("type_")
                    .or_else(|| reference.0.__unsafe_ref_get("type"))
                    .unwrap_or_else(|| stub("type"));
                m.__set_type_(ty);
                m.__set_flags(MN_IS_FIELD | (ref_kind << 24) | (mods & 0xFFFF));
            }
            other => stub(other),
        }
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
        let owner = format!("{}", m.__get_clazz().__get_name()).replace('.', "/");
        // BoundMethodHandle 动态物种的 key 形态字段 arg<T><i>（N11，species_dyn）
        let dyn_meta = crate::species_dyn::field_meta(&owner, &name)
            .map(|(d, st, mods)| (d, st, mods, None::<i64>));
        let Some((descriptor, is_static, modifiers, _)) =
            dyn_meta.or_else(|| m.__get_clazz().__declared_field_meta(&name))
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
        let clazz = m.__get_clazz();
        let meta = clazz.__declared_method_meta(&name, &descriptor).or_else(|| {
            // 签名多态方法（JVMS §2.9.3）：声明于 MethodHandle / VarHandle、唯一形参 Object[]、
            // native（+ varargs）——invokeBasic / linkToStatic / linkToSpecial / VarHandle.get 等
            // 按任意调用描述符解析（HotSpot 的 resolve 同样对其放行，调用经内建 linker 执行，
            // 见 method_handle_ext.rs）。返回位按声明形态逐一匹配（Object / boolean / void）。
            let owner = format!("{}", clazz.__get_name()).replace('.', "/");
            // BoundMethodHandle 动态物种的 key 形态工厂 make(MethodType, LambdaForm, T0..)（N11）
            if let Some(meta) = crate::species_dyn::method_meta(&owner, &name, &descriptor) {
                return Some(meta);
            }
            // @CallerSensitive 注入调用器（运行期登记的隐藏类）→ VM 支持类的模板方法声明
            if let Some(meta) = crate::injected_invoker::method_meta(&owner, &name, &descriptor) {
                return Some(meta);
            }
            if owner != "java/lang/invoke/MethodHandle" && owner != "java/lang/invoke/VarHandle" {
                return None;
            }
            ["Ljava/lang/Object;", "Z", "V"].iter().find_map(|ret| {
                clazz.__declared_method_meta(&name, &format!("([Ljava/lang/Object;){}", ret))
                    .filter(|(_, _, is_native, _)| *is_native)
            })
        });
        let Some((modifiers, _is_static, _is_native, _is_abstract)) = meta else {
            return not_found(&m);
        };
        // 类别位保留构造时的 MN_IS_METHOD / MN_IS_CONSTRUCTOR
        let kind_bits = m.__get_flags() & (MN_IS_METHOD | MN_IS_CONSTRUCTOR);
        let owner = format!("{}", clazz.__get_name()).replace('.', "/");
        let cs = caller_sensitive_bit(&owner, &name, &descriptor);
        m.__set_flags(kind_bits | cs | ((ref_kind as i32) << 24) | (modifiers & 0xFFFF));
        Ok(m)
    }

    /// native `objectFieldOffset(MemberName)`：实例字段的偏移量。HotSpot 返回
    /// 对象布局真实偏移；原生二进制字段经名字访问，偏移只作不透明标识——
    /// 与 Unsafe.objectFieldOffset(Class, String) / (Field) 共用同一登记表
    /// （JDK 三路径对同一字段同值；VarHandle 的访问器与 Unsafe 原子族因
    /// 此对同一存储单元达成一致）。
    /// native `staticFieldBase(MemberName)`：静态字段基址——声明类 Class 对象（与
    /// Unsafe.staticFieldBase 同约定；MH 解释器据 Class 基址识别静态访问）。
    #[jvm_native]
    pub fn staticFieldBase(m: MemberName) -> Result<Object> {
        Ok(Object::from(m.__get_clazz()))
    }

    /// native `staticFieldOffset(MemberName)`：静态字段偏移——与 Unsafe.staticFieldOffset
    /// 共用 (声明类, 字段名) 静态登记表（reflect_dispatch::static_field_id，id 区间与实例
    /// 字段不相交）：Unsafe 引用访问器据此路由到声明类的静态存储（ClassSpecializer$Factory
    /// .linkCodeToSpeciesData 写 species 类 BMH_SPECIES 的消费路径）。
    #[jvm_native]
    pub fn staticFieldOffset(m: MemberName) -> Result<i64> {
        let decl = format!("{}", m.__get_clazz().__get_name()).replace('.', "/");
        Ok(crate::reflect_dispatch::static_field_id(decl, format!("{}", m.__get_name())))
    }

    /// native `expand(MemberName)`：HotSpot `MethodHandles::expand_MemberName`（suppress = 0）。
    /// clazz / name / type 齐全 → 无事可做；方法 / 构造器的补全来源是 vmtarget（Method*），本模型的
    /// MemberName 由 init / resolve / 栈遍历一次填齐 name 与 type，不留 vmtarget 形态 → 与 HotSpot
    /// vmtarget 为空时同：IAE "nothing to expand"。字段：clazz 为空 → IAE "nothing to expand (as field)"；
    /// HotSpot 按 vmindex（偏移）回查字段，本模型偏移是不透明标识，按已知名字回查字段表补 type，名字也缺
    /// 则回查不到 → 与 HotSpot 回查失败同落到 "unrecognized MemberName format"。
    #[jvm_native]
    pub fn expand(m: MemberName) -> Result<()> {
        if _is_jnull_ref(&m) {
            return Err(_internal_error("mname is null"));
        }
        let have_defc = !_is_jnull_ref(&m.__get_clazz());
        let have_name = !_is_jnull_ref(&m.__get_name());
        let have_type = !_is_jnull_ref(&m.__get_type_());
        if have_defc && have_name && have_type {
            return Ok(());
        }
        let flags = m.__get_flags();
        if flags & (MN_IS_METHOD | MN_IS_CONSTRUCTOR) != 0 && flags & MN_IS_FIELD == 0 {
            return Err(JvmError::illegal_argument("nothing to expand"));
        }
        if flags & MN_IS_FIELD != 0 && flags & (MN_IS_METHOD | MN_IS_CONSTRUCTOR) == 0 {
            if !have_defc {
                return Err(JvmError::illegal_argument("nothing to expand (as field)"));
            }
            if have_name {
                let name = format!("{}", m.__get_name());
                if let Some((descriptor, _, _, _)) = m.__get_clazz().__declared_field_meta(&name) {
                    m.__set_type_(Object::from(Class::__class_for_descriptor(descriptor)));
                    return Ok(());
                }
            }
        }
        Err(JvmError::illegal_argument("unrecognized MemberName format"))
    }

    #[jvm_native]
    pub fn objectFieldOffset(m: MemberName) -> Result<i64> {
        crate::jdk::internal::misc::Unsafe::getUnsafe()?
            .objectFieldOffset1(Clone::clone(&m.__get_clazz()), Clone::clone(&m.__get_name()))
    }
}
