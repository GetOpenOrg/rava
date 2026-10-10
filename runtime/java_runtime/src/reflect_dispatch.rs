//! 反射 L3 分派协议（Method.invoke / Constructor.newInstance 的按名分派）。
//!
//! ## 协议形态（架构决策，2026-09-24）
//!
//! 反射调用边在编译期不可知：`Method.invoke` 拿到的 (声明类, 方法名, 描述符)
//! 运行时才能确定目标。协议 = **按类的分派闭包注册表**：
//!
//!   - codegen 为**用户树全部类与接口**（泛型类 / 接口挂在 `X<Object..>` 上）及常量反射引用面的
//!     JDK 类 / 接口发射 per-class `__reflect_dispatch`
//!    （共置类文件尾部，match (name, descriptor) 臂调本类 typed fn：static
//!     直调 / 实例方法经 receiver 的 `try_cast::<Self>` 视图 / `<init>` 经
//!     `Self::new(..)`——视图保留运行时 vtable，虚覆盖自动生效）；
//!   - 生成项目 main 启动时登记（`register_method_dispatch`，与类初始化钩子
//!     / 注解工厂同一登记模式）；
//!   - `reflect_invoke` 按名代调：static/构造器在声明类上直查；实例方法从
//!     **receiver 运行时类**起沿直接父类表（CLASS_DIRECT_SUPER）上溯，首个
//!     处理该 (name, descriptor) 的闭包胜出——最派生覆盖优先，与 JVM 虚分派
//!     同序。运行时类链上无闭包承载时（lambda / 手写实现对象、实现类无本方法臂），接收者是
//!     声明类型的实例 → 经声明类型的闭包调用（臂经接收者视图做虚 / 接口调用，按超类型判定，
//!     不依赖运行时类名）。
//!
//! ## 为什么不合流 ObjectVTable 臂（评估文档 2026-09-22 的候选 (a)）
//!
//! 候选 (a)（vtable 追加 `__method_invoke` 臂）对**实例方法**是自然的，但
//! static/构造器不走 vtable，仍需静态分发表——两套机制并存正是评估文档要
//! 避免的。本协议以「注册表 + typed 闭包」统一承载三类成员，发射面只在
//! codegen（用户类文件尾部），不触碰 rava_macros 的 vtable 生成核心
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
use crate::sync_model::__Shared as Rc;

/// 按名分派闭包：(方法名, 描述符, 接收者, 实参) → 处理结果。
/// `None` = 本类未声明该方法（上溯继续）；`Some(r)` = 已处理（含错误传播）。
pub type ReflectDispatch =
    Rc<crate::__DynFn!((&str, &str, Object, &JArray<Object>) -> Option<Result<Object>>)>;

crate::__process_static! {
    static DISPATCHERS: crate::sync_model::__RefSlot<HashMap<String, ReflectDispatch>> =
        crate::sync_model::__RefSlot::new(HashMap::new());
}

/// 生成项目 main 启动时登记分派闭包（binary name 斜线形态；重登记幂等）。
pub fn register_method_dispatch(dispatchers: &[(&str, ReflectDispatch)]) {
    DISPATCHERS.with(|d| {
        for (name, f) in dispatchers {
            d.put((*name).to_owned(), Clone::clone(f));
        }
    });
}

/// 生成项目 main 启动时登记各声明类的静态字段表（`X::__STATICS`，见 `field_reflect`）。
pub fn register_field_dispatch(tables: &[(&str, &'static [crate::field_reflect::__StaticFieldDesc])]) {
    crate::field_reflect::register(tables);
}

/// 按名字段访问（Field.get/set、MH 字段句柄、Unsafe 静态字段共用）。字段不参与虚分派，声明类即
/// 存储归属：静态字段查声明类登记的静态字段表；实例字段沿接收者运行时类描述符查声明类的字段表
/// （`field_reflect`）。均未命中 → None，由调用方回落既有协议或如实报缺口。
pub fn reflect_field(declaring_slash: &str, name: &str, recv: Object,
                     value: Option<Object>) -> Option<Result<Object>> {
    // BoundMethodHandle 动态物种的 key 形态字段（arg<T><i>，N11）
    if let Some(r) = crate::species_dyn::field(declaring_slash, name, &recv, &value) {
        return Some(r);
    }
    if let Some(r) = crate::field_reflect::static_field(declaring_slash, name, value.clone()) {
        return Some(r);
    }
    crate::field_reflect::instance_field(declaring_slash, name, &recv, value)
}

/// 编译期常量字段的写入（静态字段表常量项的写入；Field.set 先按 final 修饰位拦截，
/// 本出口只在绕过该检查的路径上可达）。
pub fn final_field(name: &str) -> crate::error::JvmError {
    crate::error::JvmError::illegal_argument(&format!("Can not set final field {}", name))
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
    // BoundMethodHandle 动态物种的 key 形态工厂 make(MethodType, LambdaForm, T0..)（N11）
    if let Some(r) = crate::species_dyn::invoke(declaring_slash, name, descriptor, args) {
        return r;
    }
    // @CallerSensitive 注入调用器（隐藏类）→ VM 支持类 InjectedInvokerDyn，以注入类为调用者
    if let Some(r) = crate::injected_invoker::invoke(declaring_slash, name, descriptor, &recv, args) {
        return r;
    }
    let is_ctor = name == "<init>";
    // 手写根类 Object 无 codegen 分派闭包：其唯一构造器 `<init>()V`（build.rs
    // 方法表补行，JLS §4.3.2）在此直接承载——新建一个独立身份的 Object 实例
    if is_ctor && declaring_slash == "java/lang/Object" && descriptor == "()V" {
        return Object::new();
    }
    // 序列化构造器伪成员（N2，codegen 分派闭包发射）：`<alloc>` 无构造分配、`<init_on>` 在已
    // 分配实例上运行无参构造体。按声明类直查（不经虚分派），接收者原样传入。根类 Object 手写
    // 无闭包：`<alloc>` 即新建实例，`<init_on>` 构造体为空。
    let is_pseudo = name.starts_with('<') && !is_ctor;
    if is_pseudo && declaring_slash == "java/lang/Object" {
        return match name {
            "<alloc>" => Object::new(),
            _ => Ok(Object::default()),
        };
    }
    let is_virtual = !is_ctor && !is_pseudo
        && !is_static_descriptor(declaring_slash, name, descriptor);
    // static / 构造器：声明类直查；实例方法：receiver 运行类起沿直接父类上溯
    //（隐式 null 检查：实例方法的 null receiver → NPE，与 JVM invoke0 一致）
    let cur = if is_virtual {
        if recv.0.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        recv.0.__class_name().to_owned()
    } else {
        declaring_slash.to_owned()
    };
    // 目标方法体抛出（非分派臂实参 marshalling 失败）：登记为「目标抛出」，
    // 供 AccessorUtils.isIllegalArgument 的栈帧判定（见 mark_target_thrown）
    let finish = |r: Result<Object>| {
        if let Err(e) = &r {
            if !crate::exec_context::state().bad_arg.get() && !is_platform_member(declaring_slash) {
                mark_target_thrown(e.thrown());
            }
        }
        r
    };
    // 实例方法的接收者须是声明类型的实例（JVM invoke0：IllegalArgumentException
    // "object is not an instance of declaring class"，属实参不符、不包装为 InvocationTargetException）。
    // 实例判定以接收者视图为准，再按父类链复核（手写实现对象的视图可能不全）
    if is_virtual && declaring_slash != "java/lang/Object" && !recv.0.is_instance_of(declaring_slash)
        && !superclass_chain(&cur).iter().any(|c| c == declaring_slash) {
        crate::exec_context::state().bad_arg.set(true);
        return Err(crate::error::JvmError::illegal_argument("object is not an instance of declaring class"));
    }
    let mut cur = cur;
    let mut hops = 0usize;
    loop {
        if let Some(f) = lookup(&cur) {
            if let Some(r) = f(name, descriptor, Clone::clone(&recv), args) {
                return finish(r);
            }
        }
        match superclass(&cur) {
            Some(z) if hops < 256 => cur = z,
            _ => break, // 无父类 / 表外（接口 / Object 上方）/ 防御环
        }
        hops += 1;
    }
    // 运行时类链上无闭包承载（lambda / 手写实现对象不是字节码类；接口方法的实现类无本方法臂）：
    // 接收者是声明类型的实例时，经声明类型的闭包做虚 / 接口调用——臂经接收者视图调用，按其实现选中
    if is_virtual && recv.0.is_instance_of(declaring_slash) {
        if let Some(r) = lookup(declaring_slash).and_then(|f| f(name, descriptor, Clone::clone(&recv), args)) {
            return finish(r);
        }
    }
    panic!("stub: L3 反射分派未覆盖 {}.{}:{}（分派闭包缺席 / 方法未发射）",
           declaring_slash, name, descriptor)
}

/// native 反射调用入口（HotSpot `Reflection::invoke_method` / `invoke_constructor` 的对应物，供
/// NativeAccessor / Native*AccessorImpl 的 invoke0 / newInstance0 使用）：先做 VM 的实参校验再按名分派。
/// - 实例方法的 null 接收者 → NPE（先于实参校验，JVM 同序；VM 直抛，不包装）；
/// - 实参个数须等于形参个数（null 实参数组视同 0 个）；
/// - 引用形参的非 null 实参须是形参类型的实例。
/// 不符 → IllegalArgumentException（置实参失败标记，不包装为 InvocationTargetException）。
/// 基本类型形参的拆箱 / 拓宽由分派臂承担（失败同走 [`bad_arg`]）。
pub fn native_invoke(declaring_slash: &str, name: &str, descriptor: &str,
                     recv: Object, args: &JArray<Object>) -> Result<Object> {
    if name != "<init>" && recv.0.is_jvm_null() && !is_static_descriptor(declaring_slash, name, descriptor) {
        crate::exec_context::state().bad_arg.set(true);
        return Err(crate::error::JvmError::null_pointer());
    }
    let params = crate::java::lang::Class::__descriptor_params(descriptor);
    let argc = if args.is_jvm_null() { 0 } else { args.len()? as usize };
    if argc != params.len() {
        crate::exec_context::state().bad_arg.set(true);
        return Err(crate::error::JvmError::illegal_argument(
            &format!("wrong number of arguments: {argc} expected: {}", params.len())));
    }
    for (i, p) in params.iter().enumerate() {
        let target = match p.strip_prefix('L').and_then(|r| r.strip_suffix(';')) {
            Some(b) => b,
            None if p.starts_with('[') => p.as_str(),
            None => continue,
        };
        let a = args.get(i as i32)?;
        if a.0.is_jvm_null() || target == "java/lang/Object" || a.0.is_instance_of(target)
            || crate::java::lang::Class::__name_assignable(target, a.0.__class_name()) {
            continue;
        }
        return Err(bad_arg());
    }
    reflect_invoke(declaring_slash, name, descriptor, recv, args)
}

/// 直接父类（Class.getSuperclass 的公共查询面，java_meta 生成）；无父类 / 表外 → None
fn superclass(class_slash: &str) -> Option<String> {
    let zuper = crate::java::lang::Class::for_class(crate::java::lang::String::from(class_slash))
        .getSuperclass()
        .unwrap_or_default();
    if Object::from(Clone::clone(&zuper)).0.is_jvm_null() {
        return None;
    }
    Some(format!("{}", zuper.__get_name()).replace('.', "/"))
}

/// class_slash 起的父类链（含自身）
fn superclass_chain(class_slash: &str) -> Vec<String> {
    let mut chain = vec![class_slash.to_owned()];
    while chain.len() <= 256 {
        match superclass(&chain[chain.len() - 1]) {
            Some(z) => chain.push(z),
            None => break,
        }
    }
    chain
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

/// 实参拆箱失败：置标记并返回 IllegalArgumentException（分派闭包的 marshalling 失败出口）。
pub fn bad_arg() -> crate::error::JvmError {
    crate::exec_context::state().bad_arg.set(true);
    crate::error::JvmError::illegal_argument("argument type mismatch")
}

/// 取出并清除实参失败标记（Method.invoke 在分派返回错误时调用）。
pub fn take_bad_arg() -> bool {
    crate::exec_context::state().bad_arg.replace(false)
}

/// 装箱 Object → i32（两形态：站点装箱原生盒 Rc<i32> / 翻译 Integer 包装——
/// 与 field_impl 的 __unbox_int 同一双路径策略，此处作为协议公共面）。
///
/// JDK `Method.invoke` 的实参拆箱允许基本类型拓宽（JLS §5.1.2）：Byte / Short / Character
/// 实参可传给 int 形参——两形态（原生盒 / 翻译包装类）同样接受。
pub fn unbox_i32(v: &Object) -> Option<i32> {
    if v.0.is_jvm_null() {
        return None;
    }
    let any = v.0.as_any();
    if let Some(b) = any.downcast_ref::<i32>() {
        return Some(*b);
    }
    if let Some(b) = any.downcast_ref::<i16>() {
        return Some(*b as i32);
    }
    if let Some(b) = any.downcast_ref::<i8>() {
        return Some(*b as i32);
    }
    if let Some(b) = any.downcast_ref::<u16>() {
        return Some(*b as i32);
    }
    match v.0.__class_name() {
        "java/lang/Integer" | "java/lang/Short" | "java/lang/Byte" => v.0.__obj_str().parse::<i32>().ok(),
        "java/lang/Character" => unbox_char(v).map(|c| c as i32),
        _ => None,
    }
}

/// 装箱 Object → char（UTF-16 码元）：原生 u16 盒 / 翻译 Character 包装（toString 为该字符）。
/// char 形参只接受 Character 实参（JDK：int 不窄化为 char）。
pub fn unbox_char(v: &Object) -> Option<u16> {
    if v.0.is_jvm_null() {
        return None;
    }
    if let Some(b) = v.0.as_any().downcast_ref::<u16>() {
        return Some(*b);
    }
    if v.0.__class_name() == "java/lang/Character" {
        let mut buf = [0u16; 2];
        let s = v.0.__obj_str();
        let mut chars = s.chars();
        let c = chars.next()?;
        if chars.next().is_some() {
            return None;
        }
        return c.encode_utf16(&mut buf).first().copied();
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
    // JLS §5.1.2 拓宽：byte / short / char / int → long（Method.invoke / Field.set 的实参转换）
    unbox_i32(v).map(|x| x as i64)
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

/// 装箱 Object → double：原生盒 / 翻译 Double、Float 包装（toString 文本——Rust 浮点解析
/// 接受 Java 的 `NaN` / `Infinity` / `1.0E10` 形态）；JLS §5.1.2 拓宽接受全部数值类型。
pub fn unbox_f64(v: &Object) -> Option<f64> {
    if v.0.is_jvm_null() {
        return None;
    }
    if let Some(b) = v.0.as_any().downcast_ref::<f64>() {
        return Some(*b);
    }
    if v.0.__class_name() == "java/lang/Double" {
        return v.0.__obj_str().parse::<f64>().ok();
    }
    unbox_f32(v).map(|x| x as f64)
}

/// 装箱 Object → float：原生盒 / 翻译 Float 包装；拓宽接受 byte / short / char / int / long。
pub fn unbox_f32(v: &Object) -> Option<f32> {
    if v.0.is_jvm_null() {
        return None;
    }
    if let Some(b) = v.0.as_any().downcast_ref::<f32>() {
        return Some(*b);
    }
    if v.0.__class_name() == "java/lang/Float" {
        return v.0.__obj_str().parse::<f32>().ok();
    }
    unbox_i64(v).map(|x| x as f32)
}

// ── @CallerSensitive 的显式调用者（平台无关的 getCallerClass 数据面）────────────
//
// HotSpot 的 `Reflection.getCallerClass` 读取 Java 栈帧；原生二进制没有 Java 帧，Rust 符号
// 解析随平台符号化形态而变（macOS / Linux 不一致）。生成器在调用标注 @CallerSensitive
// 的方法处把「调用处所在类」显式压栈（`__caller_sensitive`），`getCallerClass` 优先读栈顶，
// 与 JVM 语义逐点一致；栈空（手写运行时直接调用 caller-sensitive 方法）才回退帧解析。

struct CallerFrameGuard;
impl Drop for CallerFrameGuard {
    fn drop(&mut self) {
        crate::exec_context::state().cs_callers.borrow_mut().pop();
    }
}

/// 以 `caller`（调用处所在类的 binary name）为 @CallerSensitive 调用者执行 `f`。
pub fn __caller_sensitive<R>(caller: &'static str, f: impl FnOnce() -> R) -> R {
    crate::exec_context::state().cs_callers.borrow_mut().push(caller);
    let _guard = CallerFrameGuard;
    f()
}

/// 当前最内层 @CallerSensitive 调用的调用者类（生成器显式传入）；无 → None。
pub fn current_caller_sensitive() -> Option<&'static str> {
    crate::exec_context::state().cs_callers.borrow().last().copied()
}

// ── 静态字段偏移登记（Unsafe.staticFieldOffset / MethodHandleNatives.staticFieldOffset 共用）─────────────────

/// 字段偏移槽宽：每个字段（实例 / 静态）独占一个 4 字节对齐槽，偏移 id 恒为 4 的倍数。
/// JDK 的子字 CAS（compareAndExchangeByte / Short 等，字节码翻译）按 `offset & ~3` 取所在 int 字、
/// 按小端 `shift = (offset & 3) << 3` 定位——槽对齐使字地址即字段自身偏移、shift = 0，int 字视图
/// （`ObjectVTable::__unsafe_word`）的低位就是该字段，相邻字段不共字，字段之间无别名。
pub const FIELD_SLOT: i64 = 4;

// ── 实例字段偏移登记（Unsafe.objectFieldOffset 两重载、引导映像重定位槽共用）─────────────────

crate::__process_static! {
    /// 实例字段偏移登记表（进程级）：正向 (声明类名, 字段名) → id（只在 `objectFieldOffset` 登记时查），
    /// 反向按 id 稠密排列的 (声明类 binary name, Java 字段名)——id = `FIELD_SLOT * (下标 + 1)`，访问器
    /// 热路径按下标直取、不哈希不克隆。登记项进程内常驻（字段身份个数有界），以 `&'static` 借出。
    /// `objectFieldOffset` 两重载共用；id 消费见基本类型统一载体（`unsafe__ext::prim` 的实例字段臂）
    /// 与引用访问器族。
    static FIELD_OFFSETS: crate::sync_model::__RefSlot<std::collections::HashMap<(std::string::String, std::string::String), i64>> =
        crate::sync_model::__RefSlot::new(std::collections::HashMap::new());
    static FIELD_OFFSET_BY_ID: crate::sync_model::__RefSlot<Vec<&'static (std::string::String, std::string::String)>> =
        crate::sync_model::__RefSlot::new(Vec::new());
}

/// 实例字段偏移的不透明 id：键 = (声明类 binary name, 字段名)，同一字段恒等。
///
/// 原生二进制没有 C 对象布局，字段经名字访问——偏移量只作不透明标识。
/// `objectFieldOffset(Field)` 与 `objectFieldOffset(Class, String)` 按 JDK 语义
/// 对同一字段返回同一值，共用本登记表（Field 经 getDeclaredField 每次构造
/// 新对象，对象身份不稳定，字段身份 = 声明类 + 字段名）。
/// 消费方：基本类型访问器族经 ObjectVTable 的字 / 双字视图（`__unsafe_word` /
/// `__unsafe_dword`）按字段名访问共享存储单元，引用访问器族经引用原子协议——写入对
/// 直接字段读取可见。id 按 `FIELD_SLOT` 对齐，具体值不进可观察输出。
pub fn instance_field_id(clazz_name: std::string::String, field_name: std::string::String) -> i64 {
    FIELD_OFFSETS.with(|offsets| {
        let mut offsets = offsets.borrow_mut();
        let decl = clazz_name.replace('.', "/");
        let key = (clazz_name, Clone::clone(&field_name));
        if let Some(&id) = offsets.get(&key) {
            return id;
        }
        let id = FIELD_OFFSET_BY_ID.with(|by_id| {
            let mut by_id = by_id.borrow_mut();
            by_id.push(Box::leak(Box::new((decl, field_name))));
            FIELD_SLOT * by_id.len() as i64
        });
        offsets.insert(key, id);
        id
    })
}

/// 偏移 id → 登记的 (声明类 binary name, 字段名)；非实例字段 id（静态 id、数组偏移、哨兵）→ None。
pub fn instance_field_by_id(offset: i64) -> Option<&'static (std::string::String, std::string::String)> {
    if offset <= 0 || offset % FIELD_SLOT != 0 {
        return None;
    }
    let idx = usize::try_from(offset / FIELD_SLOT - 1).ok()?;
    FIELD_OFFSET_BY_ID.with(|by_id| by_id.borrow().get(idx).copied())
}

pub const STATIC_FIELD_ID_BASE: i64 = 1 << 40;

crate::__process_static! {
    static STATIC_FIELD_IDS: crate::sync_model::__RefSlot<
        std::collections::HashMap<(std::string::String, std::string::String), i64>> =
        crate::sync_model::__RefSlot::new(std::collections::HashMap::new());
    static STATIC_FIELD_BY_ID: crate::sync_model::__RefSlot<
        std::collections::HashMap<i64, (std::string::String, std::string::String)>> =
        crate::sync_model::__RefSlot::new(std::collections::HashMap::new());
}

/// (声明类, 字段名) → 稳定静态偏移 id（首次登记分配）。
pub fn static_field_id(decl: std::string::String, name: std::string::String) -> i64 {
    let key = (decl, name);
    if let Some(id) = STATIC_FIELD_IDS.with(|m| m.borrow().get(&key).copied()) {
        return id;
    }
    let id = STATIC_FIELD_IDS.with(|m| {
        let mut m = m.borrow_mut();
        // 与实例字段同一槽宽（`FIELD_SLOT`）：子字 CAS 的 `offset & ~3` 落在字段自身
        let next = STATIC_FIELD_ID_BASE + FIELD_SLOT * m.len() as i64;
        *m.entry(key.clone()).or_insert(next)
    });
    STATIC_FIELD_BY_ID.with(|m| { m.borrow_mut().insert(id, key); });
    id
}

/// 静态偏移 id → (声明类, 字段名)；非静态登记 id → None。
pub fn static_field_of(offset: i64) -> Option<(std::string::String, std::string::String)> {
    if offset < STATIC_FIELD_ID_BASE {
        return None;
    }
    STATIC_FIELD_BY_ID.with(|m| m.borrow().get(&offset).cloned())
}

// ── 反射目标抛出登记（栈帧判定的 VM 等价物）───────────────────────────────────
// JDK 的 DirectMethodHandleAccessor / DirectConstructorHandleAccessor 捕获 ClassCastException /
// NullPointerException / WrongMethodTypeException 后，经 AccessorUtils.isIllegalArgument 翻看
// 异常栈帧：抛出点在访问器 / 句柄适配层（实参转换）→ IllegalArgumentException，在目标方法内 →
// InvocationTargetException。原生二进制无 Java 栈帧；L3 分派是「进入目标方法」的唯一入口，
// 在此登记从目标方法体逃逸的异常身份，判定按「是否经目标逃逸」等价应答。

/// 平台（java.base 等 JDK 模块）成员：AccessorUtils.isIllegalArgument 的栈帧规则自抛出点
/// 向下，途经 java.base 帧继续、抵达访问器类判实参不符（IAE），遇非 java.base 帧（用户代码）
/// 判目标抛出（ITE）。经平台成员逃逸的异常因此不登记——只有逃逸出用户成员才算目标抛出。
fn is_platform_member(declaring_slash: &str) -> bool {
    ["java/", "javax/", "jdk/", "sun/"].iter().any(|p| declaring_slash.starts_with(p))
}

fn mark_target_thrown(e: &Object) {
    {
        let mut v = crate::exec_context::state().target_thrown.borrow_mut();
        if v.iter().any(|x| x == e) {
            return;
        }
        if v.len() >= 8 {
            v.remove(0);
        }
        v.push(Clone::clone(e));
    }
}

/// 异常 `e` 是否从反射目标方法体逃逸（AccessorUtils.isIllegalArgument 的 VM 应答）。
pub fn thrown_by_target(e: &Object) -> bool {
    crate::exec_context::state().target_thrown.borrow().iter().any(|x| x == e)
}
