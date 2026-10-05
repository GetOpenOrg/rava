use crate::prelude::*;
use super::unsafe_::Unsafe;
use crate::java::lang::Class;
use crate::sync_model::__RefSlot as RefCell;
use std::collections::HashMap;
use crate::reflect_dispatch::FIELD_SLOT;
use super::unsafe__ext as _ext;
mod access;
use access::*;
mod memory;
use memory::*;
mod primitive;
use primitive::*;

// 内部边界类 jdk.internal.misc.Unsafe：按调用链按需实现，其余保持 panic 存根。

// ── 数组对象布局常量（HotSpot 64 位 + CompressedOops 的真实值） ──────────
// 原生二进制没有 C 数组布局，元素经下标访问；base/scale 只需与访问器族的
// 偏移解码自洽（生产者 arrayBaseOffset/arrayIndexScale 与消费者
// getReferenceAcquire 族使用同一组常量），具体值不进可观察输出。
//
// ABASE 对全部数组类恒 16（64 位压缩 oops 的数组对象头）；引用元素 stride 4
// （压缩指针）。CHM 等消费方按 `offset = (i << ASHIFT) + ABASE`、
// `ASHIFT = 31 - numberOfLeadingZeros(scale)` 计算，反解
// `i = (offset - 16) >> 2`。
const ARRAY_BASE_OFFSET: i64 = crate::native_memory::ARRAY_BASE_OFFSET;
const REF_INDEX_SCALE: i64 = 4;

/// 数组类的元素 stride（HotSpot arrayIndexScale0 语义）：按 Class 名的数组
/// 描述符给出。名字来自 `Class::for_class`（斜线转点后的形态），前导 `[` 后
/// 是元素描述符；引用元素（`L...;` / 嵌套 `[`）取压缩指针 4。
fn _array_index_scale_by_name(name: &str) -> Option<i64> {
    let elem = name.strip_prefix('[')?;
    Some(crate::vm_constants::array_index_scale(elem.chars().next()?))
}

/// 引用元素数组的擦除视图（S-4 协变视图通道）：经 `__view_into` 把任意引用
/// 元素数组还原为 `JArray<Object>`（get/set 委托源数组存储），供 Unsafe 的
/// 引用访问器族按下标读写。基本元素数组 / 非数组对象返回 None。
fn _erased_ref_array(o: &Object) -> Option<JArray<Object>> {
    let unused = crate::sync_model::__unused_any();
    let mut slot: Option<JArray<Object>> = None;
    if o.0.__view_into(unused, &mut slot) {
        slot
    } else {
        None
    }
}

/// 引用访问器族的偏移解码：`offset = (i << ASHIFT) + ABASE` 的逆
/// （引用元素 scale=4 → ASHIFT=2）。
fn _ref_array_index(offset: i64) -> i32 {
    ((offset - ARRAY_BASE_OFFSET) / REF_INDEX_SCALE) as i32
}

crate::__process_static! {
    /// 实例字段偏移登记表（进程级）：正向 (声明类名, 字段名) → id（只在 `objectFieldOffset` 登记时查），
    /// 反向按 id 稠密排列的 (声明类 binary name, Java 字段名)——id = `FIELD_SLOT * (下标 + 1)`，访问器
    /// 热路径按下标直取、不哈希不克隆。登记项进程内常驻（字段身份个数有界），以 `&'static` 借出。
    /// `objectFieldOffset` 两重载共用；id 消费见基本类型统一载体（`unsafe__ext::prim` 的实例字段臂）
    /// 与引用访问器族。
    static FIELD_OFFSETS: RefCell<HashMap<(std::string::String, std::string::String), i64>> =
        RefCell::new(HashMap::new());
    static FIELD_OFFSET_BY_ID: RefCell<Vec<&'static (std::string::String, std::string::String)>> =
        RefCell::new(Vec::new());
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
fn _object_field_offset_id(clazz_name: std::string::String, field_name: std::string::String) -> i64 {
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
fn _field_by_id(offset: i64) -> Option<&'static (std::string::String, std::string::String)> {
    if offset <= 0 || offset % FIELD_SLOT != 0 {
        return None;
    }
    let idx = usize::try_from(offset / FIELD_SLOT - 1).ok()?;
    FIELD_OFFSET_BY_ID.with(|by_id| by_id.borrow().get(idx).copied())
}

/// 偏移 id → (声明类 binary name, 字段名)：MethodHandle 字段访问形态（DMH Accessor 的
/// `UNSAFE.getX(base, offset)`）经此还原字段身份，走按名字段协议（reflect_field）。
fn field_of_offset(offset: i64) -> Option<(std::string::String, std::string::String)> {
    _field_by_id(offset).cloned()
}

/// 偏移 id → 接收者 `o` 上按名协议的 Rust 字段名（实例字段登记表的反查；静态字段偏移 / 哨兵不在表内 → None）。
/// 登记的是 Java 字段身份 (声明类, 字段名)，经运行时类的 `__field_slot` 还原为 Rust 字段名（关键字 /
/// `$` / 遮蔽字段改名时二者不同，如 `Socket.in` → `in_`），未改名的字段两名相同。
/// 引用族（`__unsafe_ref_*`）与基本类型统一载体（`unsafe__ext` 的字 / 双字视图臂）共用。
pub(super) fn offset_slot(o: &Object, offset: i64) -> Option<&'static str> {
    let (decl, name) = _field_by_id(offset)?;
    Some(o.0.__field_slot(decl, name).unwrap_or(name.as_str()))
}

/// 偏移 id → 实例引用字段读（VarHandle 引用族消费）：字段名经登记表反查后走
/// ObjectVTable 的引用原子协议（`__unsafe_ref_get`）。与数组引用访问器族
/// （`getReferenceAcquire` 等，偏移按数组布局反解下标）分立——VarHandle 的
/// Field 家族偏移恒出自 objectFieldOffset 登记表，两口径不混用。
/// 未登记的 id 或运行时类无该引用字段 → None。
fn _instance_ref_get(o: &Object, offset: i64) -> Option<Object> {
    let field = offset_slot(o, offset)?;
    o.0.__unsafe_ref_get(field)
}

/// 偏移 id → 实例引用字段写（`_instance_ref_get` 的镜像）：命中写入返回 true，
/// 未登记 / 无臂 → false。
fn _instance_ref_set(o: &Object, offset: i64, v: Object) -> bool {
    match offset_slot(o, offset) {
        Some(field) => o.0.__unsafe_ref_set(field, v),
        None => false,
    }
}

// ── 静态字段偏移（staticFieldOffset ↔ 引用访问器的静态臂）：登记表在
//    reflect_dispatch（与 MethodHandleNatives.staticFieldOffset 共用同一 id 空间）──────

fn _static_field_id(decl: std::string::String, name: std::string::String) -> i64 {
    crate::reflect_dispatch::static_field_id(decl, name)
}

fn _static_field_of(offset: i64) -> Option<(std::string::String, std::string::String)> {
    crate::reflect_dispatch::static_field_of(offset)
}

/// 静态引用字段读：经声明类的字段闭包（与 Field.get 静态臂同一存储）。
/// 非静态 id → None；字段闭包缺席 → 如实报缺口。
fn _static_ref_get(offset: i64) -> Option<Result<Object>> {
    let (decl, name) = _static_field_of(offset)?;
    Some(crate::reflect_dispatch::reflect_field(&decl, &name, Object::default(), None)
        .unwrap_or_else(|| panic!("stub: Unsafe 静态引用读：{}.{} 无字段闭包（{}）", decl, name, crate::field_reflect::describe_static(&decl))))
}

/// 静态引用字段写（`_static_ref_get` 的镜像）。
fn _static_ref_set(offset: i64, v: Object) -> Option<Result<()>> {
    let (decl, name) = _static_field_of(offset)?;
    Some(crate::reflect_dispatch::reflect_field(&decl, &name, Object::default(), Some(v))
        .unwrap_or_else(|| panic!("stub: Unsafe 静态引用写：{}.{} 无字段闭包（{}）", decl, name, crate::field_reflect::describe_static(&decl)))
        .map(|_| ()))
}

/// 静态字段读-改-写的进程级互斥：静态存储经声明类字段闭包按名读写（无引用槽写锁 /
/// 原子单元协议），读-比-写在本锁内完成，经偏移的静态 CAS / 交换彼此原子。
static STATIC_RMW_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// 静态字段 id → 原子读-改-写（`f` 返回 Some 则写入新值），返回旧值（基本类型字段为装箱值）。
/// 非静态 id → None。取锁前先读一次：首次访问触发的声明类初始化不在锁内运行
/// （初始化体内的静态 CAS 不自锁）。引用族（`_ref_rmw`）与基本类型统一载体（`unsafe__ext::prim`
/// 的静态臂，含子字宽）共用。
pub(super) fn _static_rmw(offset: i64, f: &mut dyn FnMut(Object) -> Option<Object>) -> Option<Result<Object>> {
    let (decl, name) = _static_field_of(offset)?;
    let field = |v: Option<Object>| crate::reflect_dispatch::reflect_field(&decl, &name, Object::default(), v)
        .unwrap_or_else(|| panic!("stub: Unsafe 静态字段读-改-写：{}.{} 无字段闭包（{}）", decl, name, crate::field_reflect::describe_static(&decl)));
    if let Err(e) = field(None) {
        return Some(Err(e));
    }
    let _guard = STATIC_RMW_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    Some(field(None).and_then(|cur| {
        match f(Clone::clone(&cur)) {
            Some(nv) => field(Some(nv)).map(|_| cur),
            None => Ok(cur),
        }
    }))
}

/// 偏移 id → 实例引用字段的原子读-改-写（ObjectVTable::__unsafe_ref_update）：返回旧值；
/// 未登记 / 无臂 → None。
fn _instance_ref_update(o: &Object, offset: i64,
                        f: &mut dyn FnMut(Object) -> Option<Object>) -> Option<Object> {
    let field = offset_slot(o, offset)?;
    o.0.__unsafe_ref_update(field, f)
}

/// 引用 CAS 族的统一实现：引用元素数组按下标、实例字段按偏移 id、静态字段按静态 id，
/// 均在对应存储的写锁内完成「读出 → 比较（引用相等）→ 条件写入」，返回旧值。
fn _ref_rmw(o: &Object, offset: i64, what: &str,
            f: &mut dyn FnMut(Object) -> Option<Object>) -> Result<Object> {
    if let Some(arr) = _erased_ref_array(o) {
        return arr.__update(_ref_array_index(offset), f);
    }
    if let Some(r) = _static_rmw(offset, f) {
        return r;
    }
    match _instance_ref_update(o, offset, f) {
        Some(old) => Ok(old),
        None => panic!("jdk/internal/misc/Unsafe.{} (offset={} 无实例引用字段臂且非引用元素数组)", what, offset),
    }
}

impl Unsafe {
    /// 偏移 id → (声明类, 字段名)（`field_of_offset` 的类型挂载入口：本伴生文件以私有 mod
    /// 挂入，自由函数对包外不可见）。MH-native 解释器的 Unsafe 字段访问形态消费。
    pub fn __field_of_offset(off: i64) -> Option<(std::string::String, std::string::String)> {
        field_of_offset(off)
    }

    /// `loadFence()`：JVM 内存序（LoadLoad|LoadStore）——单线程原生二进制下
    /// 取 Acquire 栅栏即观测等价。
    pub fn loadFence(&self) -> Result<()> {
        std::sync::atomic::fence(std::sync::atomic::Ordering::Acquire);
        Ok(())
    }

    /// `storeFence()`：JVM 内存序（StoreStore|LoadStore）——Release 栅栏等价
    /// （ClassValue.initializeMap 等发布路径触达）。
    pub fn storeFence(&self) -> Result<()> {
        std::sync::atomic::fence(std::sync::atomic::Ordering::Release);
        Ok(())
    }

    /// native `fullFence()`：全栅栏（StoreLoad 在内）——SeqCst 栅栏（进程回收线程的
    /// ProcessHandleImpl 完成通知等 VarHandle.fullFence 路径触达）。
    #[jvm_native]
    pub fn fullFence(&self) -> Result<()> {
        std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }

    /// `storeStoreFence()`：StoreStore 栅栏——Release 栅栏覆盖。
    pub fn storeStoreFence(&self) -> Result<()> {
        std::sync::atomic::fence(std::sync::atomic::Ordering::Release);
        Ok(())
    }

    /// 进程内唯一的 Unsafe 实例（对应静态字段 theUnsafe）。
    #[jvm_boundary]
    pub fn getUnsafe() -> Result<Unsafe> {
        crate::__process_static! {
            static THE_UNSAFE: Unsafe = {
                let mut u = Unsafe::default();
                u._init_not_null();
                u
            };
        }
        Ok(THE_UNSAFE.with(Clone::clone))
    }

    /// `ensureClassInitialized(Class)`：确保类初始化完成（HotSpot 走 VM 类初始化）。
    /// JDK 以此运行目标类 `<clinit>` 的副作用（`SharedSecrets.javaUtilJarAccess()`：初始化 JarFile 以登记
    /// 访问器字段），惰性协议推迟到「首次主动使用」会丢失该副作用，故按名同步触发：闭包把按镜像初始化的
    /// 目标类导出为初始化钩子（closure.json `seeds.mirror_inits`），未登记的类（数组 / 基本类型 / 无
    /// `<clinit>`）no-op。
    #[jvm_boundary]
    pub fn ensureClassInitialized(&self, c: Class) -> Result<()> {
        if c.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let name = format!("{}", c.__get_name());
        crate::ensure_class_initialized(&name)
    }

    /// `shouldBeInitialized(Class)`：类尚未完成初始化（HotSpot `should_be_initialized`）。调用方据此决定是否
    /// 立即初始化：VarHandles.makeFieldHandle 的静态字段分支在创建句柄时初始化声明类，DirectMethodHandle
    /// 据此选带初始化屏障的形态。不能恒答「已初始化」——那会把创建句柄时的初始化推迟到首次访问，
    /// `<clinit>` 的副作用顺序与 JVM 不同。只对登记了初始化钩子的类作答（与 `ensureClassInitialized` 同一张表）。
    #[jvm_boundary]
    pub fn shouldBeInitialized(&self, c: Class) -> Result<bool> {
        if crate::_is_jnull_ref(&c) {
            return Err(crate::error::JvmError::null_pointer());
        }
        Ok(crate::class_needs_initialization(&format!("{}", c.__get_name())))
    }

    /// `allocateInstance(Class)`：分配实例、不运行构造器（MH `newInvokeSpecial` 的
    /// 分配步：DirectMethodHandle.allocateInstance → 随后 invokeSpecial `<init>`）。
    /// 与序列化构造器的无构造分配同一协议——L3 分派闭包的 `<alloc>` 伪成员
    /// （字段置默认值 + 非空初始化，等价 JVM 的零初始化对象）。
    /// 分配前按 HotSpot `Unsafe_AllocateInstance` → `check_valid_for_instantiation` 校验：基本类型 / 数组类抛无消息的
    /// InstantiationException，接口、抽象类抛 InstantiationException（消息为类名），`Class` 本身抛 IllegalAccessException——不可实例化的类
    /// 没有 `<alloc>` 臂，校验先于分派（如 `findConstructor(Number.class, ..)` 句柄调用）
    #[jvm_native]
    pub fn allocateInstance(&self, cls: Class) -> Result<Object> {
        if crate::_is_jnull_ref(&cls) {
            return Err(JvmError::null_pointer());
        }
        let dotted = format!("{}", cls.__get_name());
        const ACC_INTERFACE: i32 = 0x0200;
        const ACC_ABSTRACT: i32 = 0x0400;
        // 基本类型 / 数组类镜像没有 InstanceKlass：HotSpot `allocate_instance` 抛无消息的 InstantiationException
        if cls.isPrimitive()? || cls.isArray()? {
            return Err(JvmError::from(crate::java::lang::InstantiationException::new()?));
        }
        if cls.getModifiers()? & (ACC_INTERFACE | ACC_ABSTRACT) != 0 {
            return Err(JvmError::from(crate::java::lang::InstantiationException::new_str(String::from(dotted))?));
        }
        if dotted == "java.lang.Class" {
            return Err(JvmError::from(crate::java::lang::IllegalAccessException::new_str(String::from(dotted))?));
        }
        let binary = dotted.replace('.', "/");
        let empty: crate::JArray<Object> = crate::JArray::from(Vec::<Object>::new());
        crate::reflect_dispatch::reflect_invoke(&binary, "<alloc>", "()V", Object::default(), &empty)
    }


    /// 字段偏移量：HotSpot 返回对象布局的真实偏移；原生二进制没有 C 布局对象，
    /// 字段经名字访问，偏移量只作不透明标识使用（AtomicLong 等把它存进 long 字段
    /// 再传回 compareAndSwapLong——恒等即可）。按 (声明类名, 字段名) 分配稳定的
    /// 不透明 id（线程内递增），同一字段恒等——与 `objectFieldOffset(Field)`
    /// 共用同一登记表（JDK 两重载对同一字段同值）。
    #[jvm_boundary]
    pub fn objectFieldOffset_class_str(&self, c: Class, name: String) -> Result<i64> {
        Ok(_object_field_offset_id(format!("{}", c.__get_name()), format!("{}", name)))
    }

    /// `objectFieldOffset(Field)`：实例字段偏移。Field 按不透明身份协作协议处理
    /// （并行任务深化 Field 内部表示，此处只消费其 (声明类, 字段名) 身份），
    /// 与 (Class, String) 重载经同一登记表对同一字段返回同一不透明 id。
    #[jvm_boundary]
    pub fn objectFieldOffset_field(&self, f: crate::java::lang::reflect::Field) -> Result<i64> {
        Ok(_object_field_offset_id(
            format!("{}", f.__get_clazz().__get_name()),
            format!("{}", f.__get_name()),
        ))
    }

    /// 引用槽的原子读-改-写（VarHandle 引用族 CAS / 交换）：与 Unsafe 引用 CAS 族同一载体分派
    ///（引用元素数组 / 静态字段 / 实例字段），返回旧值。
    pub(crate) fn __vh_ref_update(&self, o: &Object, offset: i64,
                                  f: &mut dyn FnMut(Object) -> Option<Object>) -> Result<Object> {
        _ref_rmw(o, offset, "VarHandle 引用族读-改-写", f)
    }

    /// 基本类型槽 `width` 字节的原子读-改-写（VarHandle 基本类型族 CAS / 交换 / getAndAdd）：与
    /// Unsafe 基本类型访问器族同一载体（`unsafe__ext::prim`），返回旧位形。
    pub(crate) fn __vh_prim_update(&self, o: &Object, offset: i64, width: usize,
                                   f: &mut dyn FnMut(u64) -> Option<u64>) -> Result<u64> {
        _ext::prim(o, offset, width, "VarHandle 基本类型族读-改-写", f)
    }

    /// `arrayBaseOffset(Class)` 的实现核心（`core_` 约定）：数组存储里首个
    /// 元素前的头部长度。HotSpot 64 位（压缩 oops）对所有数组类返回 16；原生
    /// 二进制无 C 布局，该值与访问器族的偏移解码共用常量（自洽即可，不进可
    /// 观察输出）。null 类按 JDK 抛 NPE。
    ///
    /// 返回宽度按 JDK 25 形态书写（long）：该方法签名随 JDK 演化（javap：
    /// jdk.internal.misc.Unsafe.arrayBaseOffset JDK21 `()I` → JDK25 `()J`，
    /// 消费方 CHM.ABASE 字段同步 I→J），伴生不再以 Java 名直接暴露（避免与
    /// 生成侧模型签名同名相撞 E0592）；生成侧 class_writer 检出 `core_` 核
    /// 心后按**当前模型宽度**发适配声明转发本核心（转发体经 `this.` 调用
    /// ——宏据 NeedsWrapper 分类落到 wrapper 上下文，核心即在 wrapper 上；宽度差经显式
    /// `as` 还原），调用
    /// 面（含 putstatic 值侧）恒为模型类型——两版模型下编译面归零。
    #[jvm_boundary]
    pub fn core_arrayBaseOffset(&self, arrayClass: Class) -> Result<i64> {
        if Object::from(Clone::clone(&arrayClass)).0.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let name = format!("{}", arrayClass.__get_name());
        if _array_index_scale_by_name(&name).is_none() {
            // JDK 语义：非数组类的返回值未定义（HotSpot 走 assert/崩溃）
            panic!("stub: jdk/internal/misc/Unsafe.arrayBaseOffset:(Ljava/lang/Class;)J (非数组类 {})", name);
        }
        Ok(ARRAY_BASE_OFFSET)
    }

    /// `arrayIndexScale(Class)`：数组元素的寻址 stride（字节）。HotSpot 语义按
    /// 元素类型给出（引用元素为压缩指针 4）；消费方（CHM 的 ASHIFT 等）据此
    /// 构造偏移，访问器族用同一组常量反解下标。null 类按 JDK 抛 NPE。
    #[jvm_boundary]
    pub fn arrayIndexScale(&self, arrayClass: Class) -> Result<i32> {
        if Object::from(Clone::clone(&arrayClass)).0.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let name = format!("{}", arrayClass.__get_name());
        match _array_index_scale_by_name(&name) {
            Some(scale) => Ok(scale as i32),
            None => panic!("stub: jdk/internal/misc/Unsafe.arrayIndexScale:(Ljava/lang/Class;)I (非数组类 {})", name),
        }
    }
}

/// ACC_NATIVE（类 1）：翻译体 `arrayBaseOffset` 的 native 落点（`throwException` 见 volatile 族之后）。
impl Unsafe {
    /// native `arrayBaseOffset0(Class)`：与 `core_arrayBaseOffset` 同一常量（偏移解码自洽）。
    #[jvm_native]
    pub fn arrayBaseOffset0(&self, array_class: Class) -> Result<i32> {
        Ok(self.core_arrayBaseOffset(array_class)? as i32)
    }
}
