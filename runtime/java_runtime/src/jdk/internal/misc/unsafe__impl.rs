use crate::prelude::*;
use super::unsafe_::Unsafe;
use crate::java::lang::Class;
use crate::sync_model::__RefSlot as RefCell;
use std::collections::HashMap;
use crate::reflect_dispatch::FIELD_SLOT;
use super::unsafe__ext as _ext;

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
    let unused: crate::sync_model::__AnyRef = Rc::new(());
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
    /// 实例字段偏移登记表（线程本地）：正向 (声明类 binary name, 字段名) → id，
    /// 反向 id → 字段名。`objectFieldOffset` 两重载共用；id 消费见基本类型统一载体
    ///（`unsafe__ext::prim` 的实例字段臂）与引用访问器族。
    static FIELD_OFFSETS: RefCell<HashMap<(std::string::String, std::string::String), i64>> =
        RefCell::new(HashMap::new());
    static FIELD_OFFSET_NEXT: RefCell<i64> = const { RefCell::new(FIELD_SLOT) };
    static FIELD_OFFSET_BY_ID: RefCell<HashMap<i64, std::string::String>> = RefCell::new(HashMap::new());
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
        let key = (clazz_name, Clone::clone(&field_name));
        if let Some(&id) = offsets.get(&key) {
            return id;
        }
        let id = FIELD_OFFSET_NEXT.with(|n| {
            let v = *n.borrow();
            *n.borrow_mut() += FIELD_SLOT;
            v
        });
        offsets.insert(key, id);
        FIELD_OFFSET_BY_ID.with(|by_id| {
            by_id.borrow_mut().insert(id, field_name);
        });
        id
    })
}

/// 偏移 id → (声明类 binary name, 字段名)：MethodHandle 字段访问形态（DMH Accessor 的
/// `UNSAFE.getX(base, offset)`）经此还原字段身份，走按名字段协议（reflect_field）。
fn field_of_offset(offset: i64) -> Option<(std::string::String, std::string::String)> {
    FIELD_OFFSETS.with(|offsets| {
        offsets.borrow().iter()
            .find(|(_, id)| **id == offset)
            .map(|((c, f), _)| (c.replace('.', "/"), Clone::clone(f)))
    })
}

/// 偏移 id → 字段名（实例字段登记表的反查；静态字偏移 / 哨兵不在表内 → None）。
/// 基本类型统一载体（`unsafe__ext`）的实例字段臂消费。
pub(super) fn offset_field_name(offset: i64) -> Option<std::string::String> {
    FIELD_OFFSET_BY_ID.with(|by_id| by_id.borrow().get(&offset).cloned())
}

/// 偏移 id → 实例引用字段读（VarHandle 引用族消费）：字段名经登记表反查后走
/// ObjectVTable 的引用原子协议（`__unsafe_ref_get`）。与数组引用访问器族
/// （`getReferenceAcquire` 等，偏移按数组布局反解下标）分立——VarHandle 的
/// Field 家族偏移恒出自 objectFieldOffset 登记表，两口径不混用。
/// 未登记的 id 或运行时类无该引用字段 → None。
fn _instance_ref_get(o: &Object, offset: i64) -> Option<Object> {
    let field = offset_field_name(offset)?;
    o.0.__unsafe_ref_get(&field)
}

/// 偏移 id → 实例引用字段写（`_instance_ref_get` 的镜像）：命中写入返回 true，
/// 未登记 / 无臂 → false。
fn _instance_ref_set(o: &Object, offset: i64, v: Object) -> bool {
    match offset_field_name(offset) {
        Some(field) => o.0.__unsafe_ref_set(&field, v),
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
        .unwrap_or_else(|| panic!("stub: Unsafe 静态引用读：{}.{} 无字段闭包", decl, name)))
}

/// 静态引用字段写（`_static_ref_get` 的镜像）。
fn _static_ref_set(offset: i64, v: Object) -> Option<Result<()>> {
    let (decl, name) = _static_field_of(offset)?;
    Some(crate::reflect_dispatch::reflect_field(&decl, &name, Object::default(), Some(v))
        .unwrap_or_else(|| panic!("stub: Unsafe 静态引用写：{}.{} 无字段闭包", decl, name))
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
        .unwrap_or_else(|| panic!("stub: Unsafe 静态字段读-改-写：{}.{} 无字段闭包", decl, name));
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
    let field = offset_field_name(offset)?;
    o.0.__unsafe_ref_update(&field, f)
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
    #[jvm_boundary]
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

    /// `compareAndSetLong(Object o, long offset, long expected, long x)`：long 槽的 CAS——
    /// 统一载体（`unsafe__ext`）在该槽的存储上原子完成读-比-写：实例字段双字视图 / 静态字段
    /// 写锁 / 基本类型数组与直接内存的字节视图。JDK compareAndSetDouble 以原始位经此。
    #[jvm_boundary]
    pub fn compareAndSetLong(&self, o: Object, offset: i64, expected: i64, x: i64) -> Result<bool> {
        Ok(_ext::cas(&o, offset, 8, "compareAndSetLong:(Ljava/lang/Object;JJJ)Z", expected as u64, x as u64)? == expected as u64)
    }

    /// `compareAndExchangeLong(o, offset, expected, x)`：CAS 并返回**见证值**（交换前的
    /// 当前值；等于 expected 即交换成功）。读-比-写经统一载体原子完成（与
    /// compareAndSetLong 同一存储单元）。消费方：JDK25 ForkJoinPool.compareAndExchangeCtl
    ///（signalWork 的 ctl 状态字）。native。
    #[jvm_boundary]
    pub fn compareAndExchangeLong(&self, o: Object, offset: i64, expected: i64, x: i64) -> Result<i64> {
        Ok(_ext::cas(&o, offset, 8, "compareAndExchangeLong:(Ljava/lang/Object;JJJ)J", expected as u64, x as u64)? as i64)
    }

    /// `getAndBitwiseOrLong(o, offset, mask)`：long 字段按位或的读-改-写，返回旧值
    ///（ForkJoinPool.runState 置位）。
    #[jvm_boundary]
    pub fn getAndBitwiseOrLong(&self, o: Object, offset: i64, mask: i64) -> Result<i64> {
        Ok(_ext::prim(&o, offset, 8, "getAndBitwiseOrLong:(Ljava/lang/Object;JJ)J", &mut |c| Some(c | mask as u64))? as i64)
    }

    /// `getLongVolatile(Object o, long offset)`：volatile 读——存储单元与 plain 同一（实例原子单元 /
    /// 静态字段 / 原生内存），内存序见下方「基本类型 volatile 访问」节。
    #[jvm_boundary]
    pub fn getLongVolatile(&self, o: Object, offset: i64) -> Result<i64> {
        _volatile_load(|| self.getLong_obj_l(o, offset))
    }

    /// `putLongVolatile(Object o, long offset, long x)`：volatile 写（单元与 plain 同一）。
    /// `AtomicLong.set` 等经此路径——写入对 `__get_value` 直读可见。
    #[jvm_boundary]
    pub fn putLongVolatile(&self, o: Object, offset: i64, x: i64) -> Result<()> {
        _volatile_store(|| self.putLong_obj_l_l(o, offset, x))
    }

    /// `putLong(Object o, long offset, long x)`：实例字段 plain 写（与
    /// putLongVolatile 同一存储单元；原子单元，plain 写不弱于 volatile 写）。
    /// 消费方：`ThreadLocalRandom.localInit` 对 Thread.threadLocalRandomSeed。
    #[jvm_boundary]
    pub fn putLong_obj_l_l(&self, o: Object, offset: i64, x: i64) -> Result<()> {
        _ext::put(&o, offset, 8, "putLong:(Ljava/lang/Object;JJ)V", x as u64)
    }

    /// `getLong(Object o, long offset)`：实例字段 long 读（plain 形态，与
    /// getLongVolatile 同一存储单元）。
    /// 消费方：`ThreadLocalRandom.nextSeed` 对 Thread.threadLocalRandomSeed
    /// （读改写种子的读半边；localInit 的写半边是 putLong_obj_l_l）。
    #[jvm_boundary]
    pub fn getLong_obj_l(&self, o: Object, offset: i64) -> Result<i64> {
        Ok(_ext::get(&o, offset, 8, "getLong:(Ljava/lang/Object;J)J")? as i64)
    }

    /// `getInt(Object o, long offset)`：实例字段 int 读（plain 形态）。
    /// 消费方：`ThreadLocalRandom.current` 对 Thread.threadLocalRandomProbe。
    #[jvm_boundary]
    pub fn getInt_obj_l(&self, o: Object, offset: i64) -> Result<i32> {
        Ok(_ext::get(&o, offset, 4, "getInt:(Ljava/lang/Object;J)I")? as u32 as i32)
    }

    /// `putInt(Object o, long offset, int x)`：实例字段 int 写（plain 形态）。
    #[jvm_boundary]
    pub fn putInt_obj_l_i(&self, o: Object, offset: i64, x: i32) -> Result<()> {
        _ext::put(&o, offset, 4, "putInt:(Ljava/lang/Object;JI)V", x as u32 as u64)
    }

    /// `compareAndSetInt(Object o, long offset, int expected, int x)`：int 槽的 CAS（统一载体，
    /// compareAndSetLong 的 32 位镜像）。JDK compareAndSetFloat 以原始位、子字 CAS
    ///（compareAndExchangeByte / Short）以 `offset & ~3` 的字经此。
    #[jvm_boundary]
    pub fn compareAndSetInt(&self, o: Object, offset: i64, expected: i32, x: i32) -> Result<bool> {
        Ok(_ext::cas(&o, offset, 4, "compareAndSetInt:(Ljava/lang/Object;JII)Z", expected as u32 as u64, x as u32 as u64)? == expected as u32 as u64)
    }

    /// `getAndBitwiseAndInt(Object o, long offset, int mask)`：实例字段 int 的
    /// 原子按位与，返回旧值。JDK 原型是 CAS 重试循环；此处经统一载体一次完成
    ///（与 getAndAddInt 同族）。消费链：AQS `Node.getAndUnsetStatus`
    ///（CountDownLatch.countDown → releaseShared → signalNext）。
    #[jvm_boundary]
    pub fn getAndBitwiseAndInt(&self, o: Object, offset: i64, mask: i32) -> Result<i32> {
        Ok(_ext::prim(&o, offset, 4, "getAndBitwiseAndInt:(Ljava/lang/Object;JI)I", &mut |c| Some(c & mask as u32 as u64))? as u32 as i32)
    }

    /// `getAndBitwiseOrInt(Object o, long offset, int mask)`：按位或的读-改-写，
    /// 返回旧值（AQS `Node.setStatus` 族的对偶面；同 getAndBitwiseAndInt 取舍）。
    #[jvm_boundary]
    pub fn getAndBitwiseOrInt(&self, o: Object, offset: i64, mask: i32) -> Result<i32> {
        Ok(_ext::prim(&o, offset, 4, "getAndBitwiseOrInt:(Ljava/lang/Object;JI)I", &mut |c| Some(c | mask as u32 as u64))? as u32 as i32)
    }

    /// `getAndSetInt(Object o, long offset, int x)`：原子交换，返回旧值。
    #[jvm_boundary]
    pub fn getAndSetInt(&self, o: Object, offset: i64, x: i32) -> Result<i32> {
        Ok(_ext::prim(&o, offset, 4, "getAndSetInt:(Ljava/lang/Object;JI)I", &mut |_| Some(x as u32 as u64))? as u32 as i32)
    }

    /// `putIntOpaque` / `putIntRelease`：访问序变体——原子单元 SeqCst 存取（#42）不弱于
    /// plain 写同一存储单元（S-11 档位等价，见 VarHandle 伴生模块注释）。
    #[jvm_boundary]
    pub fn putIntOpaque(&self, o: Object, offset: i64, x: i32) -> Result<()> {
        self.putInt_obj_l_i(o, offset, x)
    }

    #[jvm_boundary]
    pub fn putIntRelease(&self, o: Object, offset: i64, x: i32) -> Result<()> {
        self.putInt_obj_l_i(o, offset, x)
    }

    /// `weakCompareAndSetInt(o, offset, expected, x)`：无竞争下 weak 与强 CAS 同义
    ///（无伪失败）。
    #[jvm_boundary]
    pub fn weakCompareAndSetInt(&self, o: Object, offset: i64, expected: i32, x: i32) -> Result<bool> {
        self.compareAndSetInt(o, offset, expected, x)
    }

    /// `weakCompareAndSetReference(o, offset, expected, x)`：同上，引用形态。
    /// 消费链：AQS 等待队列入队（`casTail` / `casNext`）。
    #[jvm_boundary]
    pub fn weakCompareAndSetReference(&self, o: Object, offset: i64, expected: Object, x: Object) -> Result<bool> {
        self.compareAndSetReference(o, offset, expected, x)
    }

    /// `getAndSetReference(o, offset, x)`：引用原子交换，返回旧值（数组槽位 /
    /// 实例字段两臂，与 compareAndSetReference 同一载体分派）。
    #[jvm_boundary]
    pub fn getAndSetReference(&self, o: Object, offset: i64, x: Object) -> Result<Object> {
        let mut x = Some(x);
        _ref_rmw(&o, offset,
            "getAndSetReference:(Ljava/lang/Object;JLjava/lang/Object;)Ljava/lang/Object;",
            &mut |_| x.take())
    }

    /// `compareAndExchangeInt(o, offset, expected, x)`：int 形态的见证值 CAS
    ///（compareAndExchangeLong 的同族对偶）。native。
    #[jvm_boundary]
    pub fn compareAndExchangeInt(&self, o: Object, offset: i64, expected: i32, x: i32) -> Result<i32> {
        Ok(_ext::cas(&o, offset, 4, "compareAndExchangeInt:(Ljava/lang/Object;JII)I", expected as u32 as u64, x as u32 as u64)? as u32 as i32)
    }

    /// `getIntAcquire(o, offset)`：acquire 读——原子单元 SeqCst 读（#42）不弱于 volatile /
    /// plain 读同一存储单元（ForkJoinPool.WorkQueue 的 top/base 读）。
    #[jvm_boundary]
    pub fn getIntAcquire(&self, o: Object, offset: i64) -> Result<i32> {
        self.getInt_obj_l(o, offset)
    }

    /// `compareAndExchangeReference(o, offset, expected, x)`：引用见证值 CAS——
    /// 数组槽位 / 实例字段两臂与 compareAndSetReference 同一载体分派，比较按
    /// Java `==`（对象身份）。消费方：JDK25 ForkJoinTask 的 aux 等待链。native。
    #[jvm_boundary]
    pub fn compareAndExchangeReference(&self, o: Object, offset: i64, expected: Object, x: Object) -> Result<Object> {
        let mut x = Some(x);
        _ref_rmw(&o, offset,
            "compareAndExchangeReference:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &mut |cur| if cur == expected { x.take() } else { None })
    }

    /// `getIntVolatile(Object o, long offset)`：int volatile 读（单元与 plain 同一）。
    #[jvm_boundary]
    pub fn getIntVolatile(&self, o: Object, offset: i64) -> Result<i32> {
        _volatile_load(|| self.getInt_obj_l(o, offset))
    }

    /// `getIntOpaque(Object o, long offset)`：实例字段 int opaque 读
    /// （JDK 9+ `Unsafe.getIntOpaque`，VarHandle getOpaque 的底层形态）。
    /// 原子单元 SeqCst 存取（#42）不弱于各访问序——与 plain/volatile
    /// 读同一存储单元。消费方：ForkJoinPool.getParallelismOpaque
    /// （CompletableFuture 公共池并行度 → USE_COMMON_POOL 判定链）。
    #[jvm_boundary]
    pub fn getIntOpaque(&self, o: Object, offset: i64) -> Result<i32> {
        self.getInt_obj_l(o, offset)
    }

    /// `putIntVolatile(Object o, long offset, int x)`：int volatile 写（单元与 plain 同一）。
    #[jvm_boundary]
    pub fn putIntVolatile(&self, o: Object, offset: i64, x: i32) -> Result<()> {
        _volatile_store(|| self.putInt_obj_l_i(o, offset, x))
    }

    /// `getReferenceAcquire(Object o, long offset)`：引用元素数组按偏移读
    ///（CHM `tabAt`）：数组经擦除协变视图还原 `JArray<Object>`，偏移按
    /// `i = (offset - ABASE) >> 2` 反解（引用元素 stride 4）；实例字段形态
    ///（登记表反查 + 引用原子协议）与 getReference 同一存储单元（S-11）。
    #[jvm_boundary]
    pub fn getReferenceAcquire(&self, o: Object, offset: i64) -> Result<Object> {
        if let Some(r) = _static_ref_get(offset) {
            return r;
        }
        if let Some(arr) = _erased_ref_array(&o) {
            return arr.get(_ref_array_index(offset));
        }
        match _instance_ref_get(&o, offset) {
            Some(v) => Ok(v),
            None => panic!("jdk/internal/misc/Unsafe.getReferenceAcquire:(Ljava/lang/Object;J)Ljava/lang/Object; (offset={} 无实例引用字段臂且非引用元素数组)", offset),
        }
    }

    // ── 引用访问器的通用形态（plain / volatile / opaque 同一族）───────────────
    //
    // 载体驱动分派：holder 是引用元素数组 → 数组形态（协变视图 + 偏移反解，
    // 与 acquire/release 形态同一套常量）；否则实例字段形态（偏移经登记表
    // 反查字段名 + ObjectVTable 引用原子协议——与直接字段读取同一存储单元，
    // JVM 字段内存语义）。原子单元 SeqCst 存取（#42）不弱于三种访问序
    // （同一单元，S-11）。消费面：LockSupport.setBlocker（Thread.parkBlocker）、
    // AQS Node.prev、ThreadLocalRandom 的 Thread.threadLocals 清理、
    // ClassSpecializer 的 speciesData 槽等。

    /// `getReference(Object o, long offset)`：引用读（plain）。
    #[jvm_boundary]
    pub fn getReference(&self, o: Object, offset: i64) -> Result<Object> {
        if let Some(r) = _static_ref_get(offset) {
            return r;
        }
        if let Some(arr) = _erased_ref_array(&o) {
            return arr.get(_ref_array_index(offset));
        }
        match _instance_ref_get(&o, offset) {
            Some(v) => Ok(v),
            None => panic!("stub: jdk/internal/misc/Unsafe.getReference:(Ljava/lang/Object;J)Ljava/lang/Object; (offset={} 无实例引用字段臂且非引用元素数组)", offset),
        }
    }

    /// `putReference(Object o, long offset, Object x)`：引用写（plain）。
    #[jvm_boundary]
    pub fn putReference(&self, o: Object, offset: i64, x: Object) -> Result<()> {
        if let Some(r) = _static_ref_set(offset, Clone::clone(&x)) {
            return r;
        }
        if let Some(arr) = _erased_ref_array(&o) {
            return arr.set(_ref_array_index(offset), x);
        }
        if _instance_ref_set(&o, offset, x) {
            return Ok(());
        }
        panic!("stub: jdk/internal/misc/Unsafe.putReference:(Ljava/lang/Object;JLjava/lang/Object;)V (offset={} 无实例引用字段臂且非引用元素数组)", offset)
    }

    /// `getReferenceVolatile(Object o, long offset)`：引用 volatile 读。
    #[jvm_boundary]
    pub fn getReferenceVolatile(&self, o: Object, offset: i64) -> Result<Object> {
        self.getReference(o, offset)
    }

    /// `putReferenceVolatile(Object o, long offset, Object x)`：引用 volatile 写。
    #[jvm_boundary]
    pub fn putReferenceVolatile(&self, o: Object, offset: i64, x: Object) -> Result<()> {
        self.putReference(o, offset, x)
    }

    /// `getReferenceOpaque(Object o, long offset)`：引用 opaque 读
    /// （JDK 9+ `Unsafe.getReferenceOpaque`）。
    #[jvm_boundary]
    pub fn getReferenceOpaque(&self, o: Object, offset: i64) -> Result<Object> {
        self.getReference(o, offset)
    }

    /// `putReferenceOpaque(Object o, long offset, Object x)`：引用 opaque 写
    /// （LockSupport.setBlocker 的写路径）。
    #[jvm_boundary]
    pub fn putReferenceOpaque(&self, o: Object, offset: i64, x: Object) -> Result<()> {
        self.putReference(o, offset, x)
    }

    /// `park(boolean isAbsolute, long time)`：LockSupport.park 的 VM 底座（permit 语义的
    /// 阻塞驻留，`monitor::park`）。许可按当前线程对象身份登记。blocker 字段
    /// （parkBlocker）由上层 `putReferenceOpaque` 携带。
    #[jvm_boundary]
    pub fn park(&self, is_absolute: bool, time: i64) -> Result<()> {
        let me = Object::from(crate::java::lang::Thread::currentThread()?);
        crate::monitor::park(me.0.__identity() as usize, is_absolute, time);
        Ok(())
    }

    /// `unpark(Object thread)`：LockSupport.unpark 的 VM 底座——授予目标线程许可并唤醒。
    /// null 线程静默（HotSpot Unsafe_Unpark 同判定）。
    #[jvm_boundary]
    pub fn unpark(&self, thread: Object) -> Result<()> {
        if !thread.0.is_jvm_null() {
            crate::monitor::unpark(thread.0.__identity() as usize);
        }
        Ok(())
    }

    /// `putReferenceRelease(Object o, long offset, Object x)`：引用写
    ///（release）。载体驱动分派：引用元素数组 → 偏移反解（CHM `setTabAt`，
    /// 协变视图 + aastore 存储检查）；否则实例字段形态（与 putReference
    /// 同一存储单元，S-11）。
    #[jvm_boundary]
    pub fn putReferenceRelease(&self, o: Object, offset: i64, x: Object) -> Result<()> {
        if let Some(r) = _static_ref_set(offset, Clone::clone(&x)) {
            return r;
        }
        if let Some(arr) = _erased_ref_array(&o) {
            return arr.set(_ref_array_index(offset), x);
        }
        if _instance_ref_set(&o, offset, x) {
            return Ok(());
        }
        panic!("jdk/internal/misc/Unsafe.putReferenceRelease:(Ljava/lang/Object;JLjava/lang/Object;)V (offset={} 无实例引用字段臂且非引用元素数组)", offset)
    }

    /// `compareAndSetReference(Object o, long offset, Object expected, Object x)`：
    /// 槽位/字段 CAS。载体驱动分派：引用元素数组（CHM `casTabAt`）按偏移
    /// 反解；实例字段（BufferedInputStream.close 的 buf 清空）走登记表反查
    /// + 引用原子协议。比较按 Java `==`（对象身份，`PartialEq for Object`）；
    /// 读-比-写在存储写锁内完成（`_ref_rmw`），并行后端下真正原子。
    #[jvm_boundary]
    pub fn compareAndSetReference(&self, o: Object, offset: i64, expected: Object, x: Object) -> Result<bool> {
        let mut x = Some(x);
        let old = _ref_rmw(&o, offset,
            "compareAndSetReference:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
            &mut |cur| if cur == expected { x.take() } else { None })?;
        Ok(old == expected)
    }

    /// `getAndAddLong(Object o, long offset, long delta)`：原子读取并加 delta，
    /// 返回旧值。
    ///
    /// 载体同全部基本类型访问器（统一载体 `unsafe__ext::prim`）：`o` 为 null 是绝对地址
    ///（JDK `Thread$ThreadIdentifiers.next` 以 `getAndAddLong(null, NEXT_TID_OFFSET, 1)` 推进
    /// VM 侧的线程 id 计数字，地址由 `Thread.getNextThreadIdOffset` 给出），经该地址上的原子
    /// 指令；实例字段（CHM `addCount` 的 baseCount）经双字视图；静态字段经字段闭包写锁。
    #[jvm_boundary]
    pub fn getAndAddLong(&self, o: Object, offset: i64, delta: i64) -> Result<i64> {
        Ok(_ext::prim(&o, offset, 8, "getAndAddLong:(Ljava/lang/Object;JJ)J", &mut |c| Some((c as i64).wrapping_add(delta) as u64))? as i64)
    }

    /// `staticFieldBase(Field)`：静态字存储基址。JDK 返回镜像 Class 对应的
    /// 基址对象；此处返回声明类对象装箱（身份稳定——`for_class` 按名缓存）。访问器按
    /// 偏移（静态字段 id）路由到声明类的静态存储，基址只作非 null 载体。
    #[jvm_boundary]
    pub fn staticFieldBase(&self, f: crate::java::lang::reflect::Field) -> Result<Object> {
        Ok(Object::from(f.__get_clazz()))
    }

    /// `staticFieldOffset(Field)`：静态字偏移量。无原始内存布局，偏移是按
    /// (声明类, 字段名) 登记的稳定不透明 id（同一字段恒同一 id，JDK 语义），取值区间
    /// 与 objectFieldOffset 的实例字段 id 不相交（`reflect_dispatch::STATIC_FIELD_ID_BASE` 起）：引用访问器
    /// 据此把 (staticFieldBase, 偏移) 路由到声明类的静态存储（引用族 `_static_ref_get/set` /
    /// `_ref_rmw`，基本类型族 `unsafe__ext::prim`，均经字段闭包）。
    #[jvm_boundary]
    pub fn staticFieldOffset(&self, f: crate::java::lang::reflect::Field) -> Result<i64> {
        let decl = format!("{}", f.__get_clazz().__get_name()).replace('.', "/");
        let name = format!("{}", f.__get_name());
        Ok(_static_field_id(decl, name))
    }

    /// `getAndAddInt(Object base, long offset, int delta)`：原子读取并加 delta，
    /// 返回旧值。统一载体：实例字段（`AtomicInteger.incrementAndGet` 的 value）经字视图，
    /// 静态字段（`Thread$ThreadNumbering.next` 的线程名计数：staticFieldBase +
    /// staticFieldOffset）经声明类字段闭包在静态存储写锁内完成——与 getstatic 读同一存储。
    #[jvm_boundary]
    pub fn getAndAddInt(&self, base: Object, offset: i64, delta: i32) -> Result<i32> {
        Ok(_ext::prim(&base, offset, 4, "getAndAddInt:(Ljava/lang/Object;JI)I", &mut |c| Some((c as u32 as i32).wrapping_add(delta) as u32 as u64))? as u32 as i32)
    }

    /// 分配基本类型数组。Rust 侧不存在未初始化内存的可观察差异，元素一律零值
    /// （JDK 规格允许实现返回已清零的数组）。
    #[jvm_boundary]
    pub fn __impl_allocateUninitializedArray(&self, componentType: Class, length: i32) -> Result<Object> {
        if length < 0 {
            return Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(String::from("Negative length"))?));
        }
        let n = length;
        let name = format!("{}", componentType.__get_name());
        Ok(match name.as_str() {
            "byte" => Object::from(JArray::<i8>::new(n)),
            "boolean" => Object::from(JArray::<bool>::new(n)),
            "short" => Object::from(JArray::<i16>::new(n)),
            "char" => Object::from(JArray::<u16>::new(n)),
            "int" => Object::from(JArray::<i32>::new(n)),
            "long" => Object::from(JArray::<i64>::new(n)),
            "float" => Object::from(JArray::<f32>::new(n)),
            "double" => Object::from(JArray::<f64>::new(n)),
            _ => return Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(String::from("Component type is not primitive"))?)),
        })
    }
}

// ── 基本类型读写（boolean / byte / short / char / float / double 的对象偏移形态）──────────
//
// 与 int / long 访问器同经统一载体 `unsafe__ext::prim`：基本类型数组 / 直接内存走字节视图，
// 静态字段走字段闭包 Update 臂，实例字段（如 ObjectStreamClass.FieldReflector 以 objectFieldOffset
// 读写的序列化字段）走 ObjectVTable 字 / 双字视图。

impl Unsafe {
    #[jvm_boundary]
    pub fn getBoolean(&self, o: Object, offset: i64) -> Result<bool> {
        Ok(_ext::get(&o, offset, 1, "getBoolean:(Ljava/lang/Object;J)Z")? != 0)
    }

    #[jvm_boundary]
    pub fn putBoolean(&self, o: Object, offset: i64, x: bool) -> Result<()> {
        _ext::put(&o, offset, 1, "putBoolean:(Ljava/lang/Object;JZ)V", x as u64)
    }

    #[jvm_boundary]
    pub fn getByte_obj_l(&self, o: Object, offset: i64) -> Result<i8> {
        Ok(_ext::get(&o, offset, 1, "getByte:(Ljava/lang/Object;J)B")? as u8 as i8)
    }

    #[jvm_boundary]
    pub fn putByte_obj_l_b(&self, o: Object, offset: i64, x: i8) -> Result<()> {
        _ext::put(&o, offset, 1, "putByte:(Ljava/lang/Object;JB)V", x as u8 as u64)
    }

    #[jvm_boundary]
    pub fn getShort_obj_l(&self, o: Object, offset: i64) -> Result<i16> {
        Ok(_ext::get(&o, offset, 2, "getShort:(Ljava/lang/Object;J)S")? as u16 as i16)
    }

    #[jvm_boundary]
    pub fn putShort_obj_l_s(&self, o: Object, offset: i64, x: i16) -> Result<()> {
        _ext::put(&o, offset, 2, "putShort:(Ljava/lang/Object;JS)V", x as u16 as u64)
    }

    #[jvm_boundary]
    pub fn getChar_obj_l(&self, o: Object, offset: i64) -> Result<u16> {
        Ok(_ext::get(&o, offset, 2, "getChar:(Ljava/lang/Object;J)C")? as u16)
    }

    #[jvm_boundary]
    pub fn putChar_obj_l_c(&self, o: Object, offset: i64, x: u16) -> Result<()> {
        _ext::put(&o, offset, 2, "putChar:(Ljava/lang/Object;JC)V", x as u64)
    }

    #[jvm_boundary]
    pub fn getFloat_obj_l(&self, o: Object, offset: i64) -> Result<f32> {
        Ok(f32::from_bits(_ext::get(&o, offset, 4, "getFloat:(Ljava/lang/Object;J)F")? as u32))
    }

    #[jvm_boundary]
    pub fn putFloat_obj_l_f(&self, o: Object, offset: i64, x: f32) -> Result<()> {
        _ext::put(&o, offset, 4, "putFloat:(Ljava/lang/Object;JF)V", x.to_bits() as u64)
    }

    #[jvm_boundary]
    pub fn getDouble_obj_l(&self, o: Object, offset: i64) -> Result<f64> {
        Ok(f64::from_bits(_ext::get(&o, offset, 8, "getDouble:(Ljava/lang/Object;J)D")?))
    }

    #[jvm_boundary]
    pub fn putDouble_obj_l_d(&self, o: Object, offset: i64, x: f64) -> Result<()> {
        _ext::put(&o, offset, 8, "putDouble:(Ljava/lang/Object;JD)V", x.to_bits())
    }
}

// ── 基本类型 volatile 访问（native get/put{Boolean,Byte,Short,Char,Float,Double}Volatile）──────
// HotSpot `MemoryAccess::get_volatile` / `put_volatile`（unsafe.cpp）：
//   读：[IRIW 平台先 fence] load；acquire
//   写：release；store；fence
// 存储单元与 plain 形态同一（字段闭包 / 原生内存），内存序按 HotSpot 原样以栅栏落地：
// 读前 SeqCst 栅栏（IRIW 保守取法）+ 读后 Acquire，写前 Release + 写后 SeqCst——
// 与 Java volatile 的顺序一致性同解（全部 volatile 访问之间存在单一全序）。

fn _volatile_load<T>(load: impl FnOnce() -> Result<T>) -> Result<T> {
    use std::sync::atomic::{fence, Ordering};
    fence(Ordering::SeqCst);
    let v = load();
    fence(Ordering::Acquire);
    v
}

fn _volatile_store(store: impl FnOnce() -> Result<()>) -> Result<()> {
    use std::sync::atomic::{fence, Ordering};
    fence(Ordering::Release);
    let r = store();
    fence(Ordering::SeqCst);
    r
}

impl Unsafe {
    #[jvm_native]
    pub fn getBooleanVolatile(&self, o: Object, offset: i64) -> Result<bool> {
        _volatile_load(|| self.getBoolean(o, offset))
    }

    #[jvm_native]
    pub fn putBooleanVolatile(&self, o: Object, offset: i64, x: bool) -> Result<()> {
        _volatile_store(|| self.putBoolean(o, offset, x))
    }

    #[jvm_native]
    pub fn getByteVolatile(&self, o: Object, offset: i64) -> Result<i8> {
        _volatile_load(|| self.getByte_obj_l(o, offset))
    }

    #[jvm_native]
    pub fn putByteVolatile(&self, o: Object, offset: i64, x: i8) -> Result<()> {
        _volatile_store(|| self.putByte_obj_l_b(o, offset, x))
    }

    #[jvm_native]
    pub fn getShortVolatile(&self, o: Object, offset: i64) -> Result<i16> {
        _volatile_load(|| self.getShort_obj_l(o, offset))
    }

    #[jvm_native]
    pub fn putShortVolatile(&self, o: Object, offset: i64, x: i16) -> Result<()> {
        _volatile_store(|| self.putShort_obj_l_s(o, offset, x))
    }

    #[jvm_native]
    pub fn getCharVolatile(&self, o: Object, offset: i64) -> Result<u16> {
        _volatile_load(|| self.getChar_obj_l(o, offset))
    }

    #[jvm_native]
    pub fn putCharVolatile(&self, o: Object, offset: i64, x: u16) -> Result<()> {
        _volatile_store(|| self.putChar_obj_l_c(o, offset, x))
    }

    #[jvm_native]
    pub fn getFloatVolatile(&self, o: Object, offset: i64) -> Result<f32> {
        _volatile_load(|| self.getFloat_obj_l(o, offset))
    }

    #[jvm_native]
    pub fn putFloatVolatile(&self, o: Object, offset: i64, x: f32) -> Result<()> {
        _volatile_store(|| self.putFloat_obj_l_f(o, offset, x))
    }

    #[jvm_native]
    pub fn getDoubleVolatile(&self, o: Object, offset: i64) -> Result<f64> {
        _volatile_load(|| self.getDouble_obj_l(o, offset))
    }

    #[jvm_native]
    pub fn putDoubleVolatile(&self, o: Object, offset: i64, x: f64) -> Result<()> {
        _volatile_store(|| self.putDouble_obj_l_d(o, offset, x))
    }

    /// native `throwException(Throwable ee)`：原样抛出 ee（HotSpot `Unsafe_ThrowException`：
    /// `THROW_OOP(JNIHandles::resolve(thr))`，不包装、不重填栈）；null → NullPointerException
    /// （与 athrow 对 null 的 JVMS 语义同一载体 `JvmError::from`）。
    #[jvm_native]
    pub fn throwException(&self, ee: crate::java::lang::Throwable) -> Result<()> {
        Err(crate::error::JvmError::from(ee))
    }
}

// ── 直接内存（malloc 承载；寻址约定见 crate::native_memory）─────────────────
// native（allocateMemory0 族）为 VM 契约；公开包装（allocateMemory / setMemory / copyMemory …）
// 的参数检查与零长度短路照 JDK 字节码语义手写（jdk/internal/misc 前缀截断期的过渡实现）。

/// JDK `alignToHeapWordSize`：按 8 字节向上取整。
fn _align_to_heap_word(bytes: i64) -> i64 {
    if bytes >= 0 { bytes.wrapping_add(7) & !7 } else { bytes }
}

fn _check_size(bytes: i64) -> Result<()> {
    if bytes < 0 {
        return Err(JvmError::illegal_argument("negative size"));
    }
    Ok(())
}

impl Unsafe {
    #[jvm_native]
    pub fn allocateMemory0(&self, bytes: i64) -> Result<i64> {
        Ok(crate::native_memory::allocate(bytes))
    }

    #[jvm_native]
    pub fn reallocateMemory0(&self, address: i64, bytes: i64) -> Result<i64> {
        Ok(crate::native_memory::reallocate(address, bytes))
    }

    #[jvm_native]
    pub fn freeMemory0(&self, address: i64) -> Result<()> {
        crate::native_memory::free(address);
        Ok(())
    }

    #[jvm_native]
    pub fn setMemory0(&self, o: Object, offset: i64, bytes: i64, value: i8) -> Result<()> {
        crate::native_memory::fill(&o, offset, bytes, value)
    }

    #[jvm_native]
    pub fn copyMemory0(&self, src_base: Object, src_offset: i64, dst_base: Object, dst_offset: i64, bytes: i64) -> Result<()> {
        crate::native_memory::copy(&src_base, src_offset, &dst_base, dst_offset, bytes, 1)
    }

    #[jvm_native]
    pub fn copySwapMemory0(&self, src_base: Object, src_offset: i64, dst_base: Object, dst_offset: i64,
                           bytes: i64, elem_size: i64) -> Result<()> {
        crate::native_memory::copy(&src_base, src_offset, &dst_base, dst_offset, bytes, elem_size as usize)
    }

    /// `allocateMemory(long)`：按堆字宽对齐；0 字节返回 0；分配失败抛 OutOfMemoryError。
    #[jvm_boundary]
    pub fn allocateMemory(&self, bytes: i64) -> Result<i64> {
        let bytes = _align_to_heap_word(bytes);
        _check_size(bytes)?;
        if bytes == 0 {
            return Ok(0);
        }
        let p = self.allocateMemory0(bytes)?;
        if p == 0 {
            return Err(JvmError::out_of_memory(&format!("Unable to allocate {} bytes", bytes)));
        }
        Ok(p)
    }

    /// `reallocateMemory(long, long)`：0 字节时释放并返回 0。
    #[jvm_boundary]
    pub fn reallocateMemory(&self, address: i64, bytes: i64) -> Result<i64> {
        let bytes = _align_to_heap_word(bytes);
        _check_size(bytes)?;
        if bytes == 0 {
            self.freeMemory(address)?;
            return Ok(0);
        }
        let p = if address == 0 { self.allocateMemory0(bytes)? } else { self.reallocateMemory0(address, bytes)? };
        if p == 0 {
            return Err(JvmError::out_of_memory(&format!("Unable to allocate {} bytes", bytes)));
        }
        Ok(p)
    }

    #[jvm_boundary]
    pub fn freeMemory(&self, address: i64) -> Result<()> {
        if address == 0 {
            return Ok(());
        }
        self.freeMemory0(address)
    }

    #[jvm_boundary]
    pub fn setMemory_obj_l_l_b(&self, o: Object, offset: i64, bytes: i64, value: i8) -> Result<()> {
        _check_size(bytes)?;
        if bytes == 0 {
            return Ok(());
        }
        self.setMemory0(o, offset, bytes, value)
    }

    #[jvm_boundary]
    pub fn setMemory_l_l_b(&self, address: i64, bytes: i64, value: i8) -> Result<()> {
        self.setMemory_obj_l_l_b(Object::default(), address, bytes, value)
    }

    #[jvm_boundary]
    pub fn copyMemory_obj_l_obj_l_l(&self, src_base: Object, src_offset: i64, dst_base: Object, dst_offset: i64, bytes: i64) -> Result<()> {
        _check_size(bytes)?;
        if bytes == 0 {
            return Ok(());
        }
        self.copyMemory0(src_base, src_offset, dst_base, dst_offset, bytes)
    }

    #[jvm_boundary]
    pub fn copyMemory_l_l_l(&self, src_address: i64, dst_address: i64, bytes: i64) -> Result<()> {
        self.copyMemory_obj_l_obj_l_l(Object::default(), src_address, Object::default(), dst_address, bytes)
    }

    /// `copySwapMemory(Object, long, Object, long, long, long elemSize)`：elemSize ∈ {2, 4, 8}，
    /// bytes 须为其整数倍（JDK copySwapMemoryChecks）。
    #[jvm_boundary]
    pub fn copySwapMemory_obj_l_obj_l_l_l(&self, src_base: Object, src_offset: i64, dst_base: Object, dst_offset: i64,
                                          bytes: i64, elem_size: i64) -> Result<()> {
        _check_size(bytes)?;
        if !matches!(elem_size, 2 | 4 | 8) {
            return Err(JvmError::illegal_argument("Illegal element size"));
        }
        if bytes % elem_size != 0 {
            return Err(JvmError::illegal_argument("Size not a multiple of element size"));
        }
        if bytes == 0 {
            return Ok(());
        }
        self.copySwapMemory0(src_base, src_offset, dst_base, dst_offset, bytes, elem_size)
    }

    #[jvm_boundary]
    pub fn copySwapMemory_l_l_l_l(&self, src_address: i64, dst_address: i64, bytes: i64, elem_size: i64) -> Result<()> {
        self.copySwapMemory_obj_l_obj_l_l_l(Object::default(), src_address, Object::default(), dst_address, bytes, elem_size)
    }

    #[jvm_boundary]
    pub fn getByte_l(&self, address: i64) -> Result<i8> {
        self.getByte_obj_l(Object::default(), address)
    }

    #[jvm_boundary]
    pub fn putByte_l_b(&self, address: i64, x: i8) -> Result<()> {
        self.putByte_obj_l_b(Object::default(), address, x)
    }

    #[jvm_boundary]
    pub fn getInt_l(&self, address: i64) -> Result<i32> {
        self.getInt_obj_l(Object::default(), address)
    }

    #[jvm_boundary]
    pub fn putInt_l_i(&self, address: i64, x: i32) -> Result<()> {
        self.putInt_obj_l_i(Object::default(), address, x)
    }

    #[jvm_boundary]
    pub fn getLong_l(&self, address: i64) -> Result<i64> {
        self.getLong_obj_l(Object::default(), address)
    }

    #[jvm_boundary]
    pub fn putLong_l_l(&self, address: i64, x: i64) -> Result<()> {
        self.putLong_obj_l_l(Object::default(), address, x)
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
