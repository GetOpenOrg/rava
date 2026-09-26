use crate::prelude::*;
use super::unsafe_::Unsafe;
use crate::java::lang::Class;
use std::cell::RefCell;
use std::collections::HashMap;

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
const ARRAY_BASE_OFFSET: i64 = 16;
const REF_INDEX_SCALE: i64 = 4;

/// 数组类的元素 stride（HotSpot arrayIndexScale0 语义）：按 Class 名的数组
/// 描述符给出。名字来自 `Class::for_class`（斜线转点后的形态），前导 `[` 后
/// 是元素描述符；引用元素（`L...;` / 嵌套 `[`）取压缩指针 4。
fn _array_index_scale_by_name(name: &str) -> Option<i64> {
    let elem = name.strip_prefix('[')?;
    Some(match elem {
        "Z" | "B" => 1,
        "C" | "S" => 2,
        "I" | "F" => 4,
        "J" | "D" => 8,
        _ => 4,
    })
}

/// 引用元素数组的擦除视图（S-4 协变视图通道）：经 `__view_into` 把任意引用
/// 元素数组还原为 `JArray<Object>`（get/set 委托源数组存储），供 Unsafe 的
/// 引用访问器族按下标读写。基本元素数组 / 非数组对象返回 None。
fn _erased_ref_array(o: &Object) -> Option<JArray<Object>> {
    let unused: Rc<dyn std::any::Any> = Rc::new(());
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

/// 实例字段偏移登记表（线程本地）：正向 (声明类 binary name, 字段名) → id，
/// 反向 id → 字段名。`objectFieldOffset` 两重载共用；id 消费见
/// `_instance_long_cell`（实例字段 long 原子）与 `getAndAddInt`（计数器键）。
thread_local! {
    static FIELD_OFFSETS: RefCell<HashMap<(std::string::String, std::string::String), i64>> =
        RefCell::new(HashMap::new());
    static FIELD_OFFSET_NEXT: RefCell<i64> = const { RefCell::new(1) };
    static FIELD_OFFSET_BY_ID: RefCell<HashMap<i64, std::string::String>> = RefCell::new(HashMap::new());
}

/// 实例字段偏移的不透明 id：键 = (声明类 binary name, 字段名)，同一字段恒等。
///
/// 原生二进制没有 C 对象布局，字段经名字访问——偏移量只作不透明标识。
/// `objectFieldOffset(Field)` 与 `objectFieldOffset(Class, String)` 按 JDK 语义
/// 对同一字段返回同一值，共用本登记表（Field 经 getDeclaredField 每次构造
/// 新对象，对象身份不稳定，字段身份 = 声明类 + 字段名）。
/// 消费形态一：实例字段 long 原子（compareAndSetLong 等经
/// `ObjectVTable::__unsafe_long_cell` 按字段名取共享存储单元，写入对直接
/// 字段读取可见）；消费形态二：原子计数器键（getAndAddInt 以 (基址身份,
/// offset) 寻址）。id 具体值不进可观察输出。
fn _object_field_offset_id(clazz_name: std::string::String, field_name: std::string::String) -> i64 {
    FIELD_OFFSETS.with(|offsets| {
        let next = FIELD_OFFSET_NEXT.with(|n| {
            let v = *n.borrow();
            *n.borrow_mut() += 1;
            v
        });
        let id = *offsets.borrow_mut()
            .entry((clazz_name, Clone::clone(&field_name)))
            .or_insert(next);
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
fn _offset_field_name(offset: i64) -> Option<std::string::String> {
    FIELD_OFFSET_BY_ID.with(|by_id| by_id.borrow().get(&offset).cloned())
}

/// 偏移 id → 实例字段的共享 long 存储单元（经 ObjectVTable 的字段名协议）。
/// 未登记的 id 或运行时类无该平铺 long 字段 → None。
fn _instance_long_cell(o: &Object, offset: i64) -> Option<Rc<std::cell::Cell<i64>>> {
    let field = _offset_field_name(offset)?;
    o.0.__unsafe_long_cell(&field)
}

/// 偏移 id → 实例字段的共享 int 存储单元（`_instance_long_cell` 的 int 镜像）。
fn _instance_int_cell(o: &Object, offset: i64) -> Option<Rc<std::cell::Cell<i32>>> {
    let field = _offset_field_name(offset)?;
    o.0.__unsafe_int_cell(&field)
}

/// 偏移 id → 实例引用字段读（VarHandle 引用族消费）：字段名经登记表反查后走
/// ObjectVTable 的引用原子协议（`__unsafe_ref_get`）。与数组引用访问器族
/// （`getReferenceAcquire` 等，偏移按数组布局反解下标）分立——VarHandle 的
/// Field 家族偏移恒出自 objectFieldOffset 登记表，两口径不混用。
/// 未登记的 id 或运行时类无该引用字段 → None。
fn _instance_ref_get(o: &Object, offset: i64) -> Option<Object> {
    let field = _offset_field_name(offset)?;
    o.0.__unsafe_ref_get(&field)
}

/// 偏移 id → 实例引用字段写（`_instance_ref_get` 的镜像）：命中写入返回 true，
/// 未登记 / 无臂 → false。
fn _instance_ref_set(o: &Object, offset: i64, v: Object) -> bool {
    match _offset_field_name(offset) {
        Some(field) => o.0.__unsafe_ref_set(&field, v),
        None => false,
    }
}

impl Unsafe {
    /// 偏移 id → (声明类, 字段名)（`field_of_offset` 的类型挂载入口：本伴生文件以私有 mod
    /// 挂入，自由函数对包外不可见）。MH-native 解释器的 Unsafe 字段访问形态消费。
    pub fn __field_of_offset(off: i64) -> Option<(std::string::String, std::string::String)> {
        field_of_offset(off)
    }

    /// `isBigEndian()Z`（final）：宿主平台字节序。JDK25 的 StringUTF16 / 字节序
    /// 敏感路径经本方法查询（JDK21 为 StringUTF16.isBigEndian native，同义）；
    /// 小端平台（x86-64 / aarch64 Linux 与 macOS）为 false。
    pub fn isBigEndian(&self) -> Result<bool> {
        Ok(cfg!(target_endian = "big"))
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

    /// `storeStoreFence()`：StoreStore 栅栏——Release 栅栏覆盖。
    pub fn storeStoreFence(&self) -> Result<()> {
        std::sync::atomic::fence(std::sync::atomic::Ordering::Release);
        Ok(())
    }

    /// 进程内唯一的 Unsafe 实例（对应静态字段 theUnsafe）。
    #[jvm_boundary]
    pub fn getUnsafe() -> Result<Unsafe> {
        thread_local! {
            static THE_UNSAFE: Unsafe = {
                let mut u = Unsafe::default();
                u._init_not_null();
                u
            };
        }
        Ok(THE_UNSAFE.with(Clone::clone))
    }

    /// `ensureClassInitialized(Class)`：确保类初始化完成（HotSpot 走 VM 类初始化）。
    /// 本运行的类初始化由翻译层的 `__class_init` 惰性协议承载（首次主动使用
    /// 即初始化）——无需（也无法）从手写层按 Class 对象强制触发，no-op 即
    /// 与惰性协议一致（初始化只是推迟到真实首次使用）。
    /// 消费链：VarHandle.<clinit>（VarHandleGuards 的预初始化）、
    /// VarHandles.makeFieldHandle 的静态字段分支。
    #[jvm_boundary]
    pub fn ensureClassInitialized(&self, _c: Class) -> Result<()> {
        Ok(())
    }

    /// `shouldBeInitialized(Class)`：类是否已初始化。惰性 `__class_init` 协议
    /// 下「未初始化」只在首次主动使用前可观察——对查询方恒「已初始化」
    /// （false）等价于把初始化时机推迟到真实首次使用，与 ensureClassInitialized
    /// 的 no-op 语义自洽。
    #[jvm_boundary]
    pub fn shouldBeInitialized(&self, _c: Class) -> Result<bool> {
        Ok(false)
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

    /// 偏移 id → 实例引用字段读（VarHandle 引用族 `get`/`getVolatile`/CAS 的
    /// 读侧消费，非 Unsafe 的 Java 公开面）：经 `_instance_ref_get` 的登记表
    /// 反查 + ObjectVTable 引用原子协议。
    pub(crate) fn __vh_ref_get(&self, o: &Object, offset: i64) -> Option<Object> {
        _instance_ref_get(o, offset)
    }

    /// 偏移 id → 实例引用字段写（VarHandle 引用族 `set`/`setVolatile`/CAS 的
    /// 写侧消费）：命中写入 true，未登记 / 运行时类无该引用字段 → false。
    pub(crate) fn __vh_ref_set(&self, o: &Object, offset: i64, v: Object) -> bool {
        _instance_ref_set(o, offset, v)
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

    // ── 数组布局静态常量（ARRAY_<T>_BASE_OFFSET / ARRAY_<T>_INDEX_SCALE）──────
    // Unsafe 为内部边界类，<clinit> 不翻译，常量值由此给出：与 arrayBaseOffset /
    // arrayIndexScale 同一组常量（偏移解码自洽）。字段类型随 JDK 演化（BASE_OFFSET
    // JDK21 `I` → JDK25 `J`），核心按 JDK25 形态书写，生成侧 clinit_extract 按当前
    // 模型类型发 getter 转发（静态字段 core_ 适配）。消费方：JDK25 ArraysSupport
    // 向量化 hashCode / mismatch（TestArraysUtil）。
    pub fn core_ARRAY_BOOLEAN_BASE_OFFSET() -> Result<i64> { Ok(ARRAY_BASE_OFFSET) }
    pub fn core_ARRAY_BOOLEAN_INDEX_SCALE() -> Result<i32> {
        Ok(_array_index_scale_by_name("[Z").unwrap_or(REF_INDEX_SCALE) as i32)
    }
    pub fn core_ARRAY_BYTE_BASE_OFFSET() -> Result<i64> { Ok(ARRAY_BASE_OFFSET) }
    pub fn core_ARRAY_BYTE_INDEX_SCALE() -> Result<i32> {
        Ok(_array_index_scale_by_name("[B").unwrap_or(REF_INDEX_SCALE) as i32)
    }
    pub fn core_ARRAY_SHORT_BASE_OFFSET() -> Result<i64> { Ok(ARRAY_BASE_OFFSET) }
    pub fn core_ARRAY_SHORT_INDEX_SCALE() -> Result<i32> {
        Ok(_array_index_scale_by_name("[S").unwrap_or(REF_INDEX_SCALE) as i32)
    }
    pub fn core_ARRAY_CHAR_BASE_OFFSET() -> Result<i64> { Ok(ARRAY_BASE_OFFSET) }
    pub fn core_ARRAY_CHAR_INDEX_SCALE() -> Result<i32> {
        Ok(_array_index_scale_by_name("[C").unwrap_or(REF_INDEX_SCALE) as i32)
    }
    pub fn core_ARRAY_INT_BASE_OFFSET() -> Result<i64> { Ok(ARRAY_BASE_OFFSET) }
    pub fn core_ARRAY_INT_INDEX_SCALE() -> Result<i32> {
        Ok(_array_index_scale_by_name("[I").unwrap_or(REF_INDEX_SCALE) as i32)
    }
    pub fn core_ARRAY_LONG_BASE_OFFSET() -> Result<i64> { Ok(ARRAY_BASE_OFFSET) }
    pub fn core_ARRAY_LONG_INDEX_SCALE() -> Result<i32> {
        Ok(_array_index_scale_by_name("[J").unwrap_or(REF_INDEX_SCALE) as i32)
    }
    pub fn core_ARRAY_FLOAT_BASE_OFFSET() -> Result<i64> { Ok(ARRAY_BASE_OFFSET) }
    pub fn core_ARRAY_FLOAT_INDEX_SCALE() -> Result<i32> {
        Ok(_array_index_scale_by_name("[F").unwrap_or(REF_INDEX_SCALE) as i32)
    }
    pub fn core_ARRAY_DOUBLE_BASE_OFFSET() -> Result<i64> { Ok(ARRAY_BASE_OFFSET) }
    pub fn core_ARRAY_DOUBLE_INDEX_SCALE() -> Result<i32> {
        Ok(_array_index_scale_by_name("[D").unwrap_or(REF_INDEX_SCALE) as i32)
    }
    pub fn core_ARRAY_OBJECT_BASE_OFFSET() -> Result<i64> { Ok(ARRAY_BASE_OFFSET) }
    pub fn core_ARRAY_OBJECT_INDEX_SCALE() -> Result<i32> {
        Ok(_array_index_scale_by_name("[Ljava/lang/Object;").unwrap_or(REF_INDEX_SCALE) as i32)
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

    /// `compareAndSetLong(Object o, long offset, long expected, long x)`：实例字段
    /// long 的 CAS——经 `__unsafe_long_cell` 取共享存储单元（与直接字段读取同一
    /// 存储，JVM 字段内存语义）。单 OS 线程协作调度下读-比-写不可分割。
    #[jvm_boundary]
    pub fn compareAndSetLong(&self, o: Object, offset: i64, expected: i64, x: i64) -> Result<bool> {
        let cell = _instance_long_cell(&o, offset).unwrap_or_else(|| {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndSetLong:(Ljava/lang/Object;JJJ)Z (实例字段 offset={} 无共享 long 单元)", offset)
        });
        let current = cell.get();
        if current == expected {
            cell.set(x);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// `compareAndExchangeLong(o, offset, expected, x)`：CAS 并返回**见证值**（交换前的
    /// 当前值；等于 expected 即交换成功）。协作档位下读-比-写不可分割（与
    /// compareAndSetLong 同一存储单元）。消费方：JDK25 ForkJoinPool.compareAndExchangeCtl
    ///（signalWork 的 ctl 状态字）。native。
    #[jvm_boundary]
    pub fn compareAndExchangeLong(&self, o: Object, offset: i64, expected: i64, x: i64) -> Result<i64> {
        let cell = _instance_long_cell(&o, offset).unwrap_or_else(|| {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeLong:(Ljava/lang/Object;JJJ)J (实例字段 offset={} 无共享 long 单元)", offset)
        });
        let current = cell.get();
        if current == expected {
            cell.set(x);
        }
        Ok(current)
    }

    /// `getAndBitwiseOrLong(o, offset, mask)`：long 字段按位或的读-改-写，返回旧值
    ///（ForkJoinPool.runState 置位）。
    #[jvm_boundary]
    pub fn getAndBitwiseOrLong(&self, o: Object, offset: i64, mask: i64) -> Result<i64> {
        let cell = _instance_long_cell(&o, offset).unwrap_or_else(|| {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrLong:(Ljava/lang/Object;JJ)J (实例字段 offset={} 无共享 long 单元)", offset)
        });
        let old = cell.get();
        cell.set(old | mask);
        Ok(old)
    }

    /// `getLongVolatile(Object o, long offset)`：实例字段 volatile 读。
    #[jvm_boundary]
    pub fn getLongVolatile(&self, o: Object, offset: i64) -> Result<i64> {
        let cell = _instance_long_cell(&o, offset).unwrap_or_else(|| {
            panic!("stub: jdk/internal/misc/Unsafe.getLongVolatile:(Ljava/lang/Object;J)J (实例字段 offset={} 无共享 long 单元)", offset)
        });
        Ok(cell.get())
    }

    /// `putLongVolatile(Object o, long offset, long x)`：实例字段 volatile 写。
    /// `AtomicLong.set` 等经此路径——写入对 `__get_value` 直读可见。
    #[jvm_boundary]
    pub fn putLongVolatile(&self, o: Object, offset: i64, x: i64) -> Result<()> {
        let cell = _instance_long_cell(&o, offset).unwrap_or_else(|| {
            panic!("stub: jdk/internal/misc/Unsafe.putLongVolatile:(Ljava/lang/Object;JJ)V (实例字段 offset={} 无共享 long 单元)", offset)
        });
        cell.set(x);
        Ok(())
    }

    /// `putLong(Object o, long offset, long x)`：实例字段 plain 写（与
    /// putLongVolatile 同一存储单元；单 OS 线程协作调度下无可见性差异）。
    /// 消费方：`ThreadLocalRandom.localInit` 对 Thread.threadLocalRandomSeed。
    #[jvm_boundary]
    pub fn putLong_obj_l_l(&self, o: Object, offset: i64, x: i64) -> Result<()> {
        let cell = _instance_long_cell(&o, offset).unwrap_or_else(|| {
            panic!("stub: jdk/internal/misc/Unsafe.putLong:(Ljava/lang/Object;JJ)V (实例字段 offset={} 无共享 long 单元)", offset)
        });
        cell.set(x);
        Ok(())
    }

    /// `getLong(Object o, long offset)`：实例字段 long 读（plain 形态，与
    /// getLongVolatile 同一存储单元）。
    /// 消费方：`ThreadLocalRandom.nextSeed` 对 Thread.threadLocalRandomSeed
    /// （读改写种子的读半边；localInit 的写半边是 putLong_obj_l_l）。
    #[jvm_boundary]
    pub fn getLong_obj_l(&self, o: Object, offset: i64) -> Result<i64> {
        let cell = _instance_long_cell(&o, offset).unwrap_or_else(|| {
            panic!("stub: jdk/internal/misc/Unsafe.getLong:(Ljava/lang/Object;J)J (实例字段 offset={} 无共享 long 单元)", offset)
        });
        Ok(cell.get())
    }

    /// `getInt(Object o, long offset)`：实例字段 int 读（plain 形态）。
    /// 消费方：`ThreadLocalRandom.current` 对 Thread.threadLocalRandomProbe。
    #[jvm_boundary]
    pub fn getInt_obj_l(&self, o: Object, offset: i64) -> Result<i32> {
        let cell = _instance_int_cell(&o, offset).unwrap_or_else(|| {
            panic!("stub: jdk/internal/misc/Unsafe.getInt:(Ljava/lang/Object;J)I (实例字段 offset={} 无共享 int 单元)", offset)
        });
        Ok(cell.get())
    }

    /// `putInt(Object o, long offset, int x)`：实例字段 int 写（plain 形态）。
    #[jvm_boundary]
    pub fn putInt_obj_l_i(&self, o: Object, offset: i64, x: i32) -> Result<()> {
        let cell = _instance_int_cell(&o, offset).unwrap_or_else(|| {
            panic!("stub: jdk/internal/misc/Unsafe.putInt:(Ljava/lang/Object;JI)V (实例字段 offset={} 无共享 int 单元)", offset)
        });
        cell.set(x);
        Ok(())
    }

    /// `compareAndSetInt(Object o, long offset, int expected, int x)`：实例字段
    /// int 的 CAS（`_instance_long_cell` 的 int 镜像路径）。
    #[jvm_boundary]
    pub fn compareAndSetInt(&self, o: Object, offset: i64, expected: i32, x: i32) -> Result<bool> {
        let cell = _instance_int_cell(&o, offset).unwrap_or_else(|| {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndSetInt:(Ljava/lang/Object;JII)Z (实例字段 offset={} 无共享 int 单元)", offset)
        });
        let current = cell.get();
        if current == expected {
            cell.set(x);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// `getAndBitwiseAndInt(Object o, long offset, int mask)`：实例字段 int 的
    /// 原子按位与，返回旧值。JDK 原型是 CAS 重试循环；协作档位下读-改-写不被
    /// 穿插即不可分割（与 getAndAddInt 同族）。消费链：AQS `Node.getAndUnsetStatus`
    ///（CountDownLatch.countDown → releaseShared → signalNext）。
    #[jvm_boundary]
    pub fn getAndBitwiseAndInt(&self, o: Object, offset: i64, mask: i32) -> Result<i32> {
        let cell = _instance_int_cell(&o, offset).unwrap_or_else(|| {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseAndInt:(Ljava/lang/Object;JI)I (实例字段 offset={} 无共享 int 单元)", offset)
        });
        let old = cell.get();
        cell.set(old & mask);
        Ok(old)
    }

    /// `getAndBitwiseOrInt(Object o, long offset, int mask)`：按位或的读-改-写，
    /// 返回旧值（AQS `Node.setStatus` 族的对偶面；同 getAndBitwiseAndInt 取舍）。
    #[jvm_boundary]
    pub fn getAndBitwiseOrInt(&self, o: Object, offset: i64, mask: i32) -> Result<i32> {
        let cell = _instance_int_cell(&o, offset).unwrap_or_else(|| {
            panic!("stub: jdk/internal/misc/Unsafe.getAndBitwiseOrInt:(Ljava/lang/Object;JI)I (实例字段 offset={} 无共享 int 单元)", offset)
        });
        let old = cell.get();
        cell.set(old | mask);
        Ok(old)
    }

    /// `getAndSetInt(Object o, long offset, int x)`：原子交换，返回旧值。
    #[jvm_boundary]
    pub fn getAndSetInt(&self, o: Object, offset: i64, x: i32) -> Result<i32> {
        let cell = _instance_int_cell(&o, offset).unwrap_or_else(|| {
            panic!("stub: jdk/internal/misc/Unsafe.getAndSetInt:(Ljava/lang/Object;JI)I (实例字段 offset={} 无共享 int 单元)", offset)
        });
        let old = cell.get();
        cell.set(x);
        Ok(old)
    }

    /// `putIntOpaque` / `putIntRelease`：访问序变体——单 OS 线程协作调度下与
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
        if let Some(arr) = _erased_ref_array(&o) {
            let i = _ref_array_index(offset);
            let old = arr.get(i)?;
            arr.set(i, x)?;
            return Ok(old);
        }
        match _instance_ref_get(&o, offset) {
            Some(old) => {
                _instance_ref_set(&o, offset, x);
                Ok(old)
            }
            None => panic!("jdk/internal/misc/Unsafe.getAndSetReference:(Ljava/lang/Object;JLjava/lang/Object;)Ljava/lang/Object; (offset={} 无实例引用字段臂且非引用元素数组)", offset),
        }
    }

    /// `compareAndExchangeInt(o, offset, expected, x)`：int 形态的见证值 CAS
    ///（compareAndExchangeLong 的同族对偶）。native。
    #[jvm_boundary]
    pub fn compareAndExchangeInt(&self, o: Object, offset: i64, expected: i32, x: i32) -> Result<i32> {
        let cell = _instance_int_cell(&o, offset).unwrap_or_else(|| {
            panic!("stub: jdk/internal/misc/Unsafe.compareAndExchangeInt:(Ljava/lang/Object;JII)I (实例字段 offset={} 无共享 int 单元)", offset)
        });
        let current = cell.get();
        if current == expected {
            cell.set(x);
        }
        Ok(current)
    }

    /// `getIntAcquire(o, offset)`：acquire 读——单 OS 线程协作调度下与 volatile /
    /// plain 读同一存储单元（ForkJoinPool.WorkQueue 的 top/base 读）。
    #[jvm_boundary]
    pub fn getIntAcquire(&self, o: Object, offset: i64) -> Result<i32> {
        let cell = _instance_int_cell(&o, offset).unwrap_or_else(|| {
            panic!("stub: jdk/internal/misc/Unsafe.getIntAcquire:(Ljava/lang/Object;J)I (实例字段 offset={} 无共享 int 单元)", offset)
        });
        Ok(cell.get())
    }

    /// `compareAndExchangeReference(o, offset, expected, x)`：引用见证值 CAS——
    /// 数组槽位 / 实例字段两臂与 compareAndSetReference 同一载体分派，比较按
    /// Java `==`（对象身份）。消费方：JDK25 ForkJoinTask 的 aux 等待链。native。
    #[jvm_boundary]
    pub fn compareAndExchangeReference(&self, o: Object, offset: i64, expected: Object, x: Object) -> Result<Object> {
        if let Some(arr) = _erased_ref_array(&o) {
            let i = _ref_array_index(offset);
            let current = arr.get(i)?;
            if current == expected {
                arr.set(i, x)?;
            }
            return Ok(current);
        }
        match _instance_ref_get(&o, offset) {
            Some(current) => {
                if current == expected {
                    _instance_ref_set(&o, offset, x);
                }
                Ok(current)
            }
            None => panic!("jdk/internal/misc/Unsafe.compareAndExchangeReference:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object; (offset={} 无实例引用字段臂且非引用元素数组)", offset),
        }
    }

    /// `getIntVolatile(Object o, long offset)`：实例字段 int volatile 读。
    #[jvm_boundary]
    pub fn getIntVolatile(&self, o: Object, offset: i64) -> Result<i32> {
        let cell = _instance_int_cell(&o, offset).unwrap_or_else(|| {
            panic!("stub: jdk/internal/misc/Unsafe.getIntVolatile:(Ljava/lang/Object;J)I (实例字段 offset={} 无共享 int 单元)", offset)
        });
        Ok(cell.get())
    }

    /// `getIntOpaque(Object o, long offset)`：实例字段 int opaque 读
    /// （JDK 9+ `Unsafe.getIntOpaque`，VarHandle getOpaque 的底层形态）。
    /// 单线程协作档位（S-11）无跨线程重排可见性差异——与 plain/volatile
    /// 读同一存储单元。消费方：ForkJoinPool.getParallelismOpaque
    /// （CompletableFuture 公共池并行度 → USE_COMMON_POOL 判定链）。
    #[jvm_boundary]
    pub fn getIntOpaque(&self, o: Object, offset: i64) -> Result<i32> {
        let cell = _instance_int_cell(&o, offset).unwrap_or_else(|| {
            panic!("stub: jdk/internal/misc/Unsafe.getIntOpaque:(Ljava/lang/Object;J)I (实例字段 offset={} 无共享 int 单元)", offset)
        });
        Ok(cell.get())
    }

    /// `putIntVolatile(Object o, long offset, int x)`：实例字段 int volatile 写。
    #[jvm_boundary]
    pub fn putIntVolatile(&self, o: Object, offset: i64, x: i32) -> Result<()> {
        let cell = _instance_int_cell(&o, offset).unwrap_or_else(|| {
            panic!("stub: jdk/internal/misc/Unsafe.putIntVolatile:(Ljava/lang/Object;JI)V (实例字段 offset={} 无共享 int 单元)", offset)
        });
        cell.set(x);
        Ok(())
    }

    /// `getReferenceAcquire(Object o, long offset)`：引用元素数组按偏移读
    ///（CHM `tabAt`）：数组经擦除协变视图还原 `JArray<Object>`，偏移按
    /// `i = (offset - ABASE) >> 2` 反解（引用元素 stride 4）；实例字段形态
    ///（登记表反查 + 引用原子协议）与 getReference 同一存储单元（S-11）。
    #[jvm_boundary]
    pub fn getReferenceAcquire(&self, o: Object, offset: i64) -> Result<Object> {
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
    // JVM 字段内存语义）。单 OS 线程协作调度下三种访问序无可见性区别
    // （同一单元，S-11）。消费面：LockSupport.setBlocker（Thread.parkBlocker）、
    // AQS Node.prev、ThreadLocalRandom 的 Thread.threadLocals 清理、
    // ClassSpecializer 的 speciesData 槽等。

    /// `getReference(Object o, long offset)`：引用读（plain）。
    #[jvm_boundary]
    pub fn getReference(&self, o: Object, offset: i64) -> Result<Object> {
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

    /// `park(boolean isAbsolute, long time)`：LockSupport.park 的 VM 底座
    /// （permit 语义的阻塞）。单线程协作档位（S-11）与 `Thread.sleep0` 同一
    /// 承载：被「阻塞」的当前模拟线程泵运行就绪模拟线程（CompletableFuture
    /// 的 ThreadPerTaskExecutor 异步任务在此推进——完成后 `waitingGet` 的
    /// 重查循环即返回），泵尽返回（JLS §17.3 允许的虚假唤醒形态）。permit
    /// 簿记不驻留：调用方（LockSupport.park/CF waitingGet）均为条件循环 +
    /// 重查消费面，虚假唤醒语义下观察面等价。blocker 字段（parkBlocker）由
    /// 上层 `putReferenceOpaque` 携带（栈轨迹消费面，golden 不可见）。
    #[jvm_boundary]
    pub fn park(&self, is_absolute: bool, time: i64) -> Result<()> {
        crate::monitor::cooperative_park(is_absolute, time)
    }

    /// `unpark(Object thread)`：LockSupport.unpark 的 VM 底座。协作档位下
    /// 唤醒动作发生在泵内（被 park 的线程不在 OS 等待上）——permit 授予后
    /// 的重新调度由泵的 FIFO 与调用方重查循环兑现 → no-op。
    #[jvm_boundary]
    pub fn unpark(&self, _thread: Object) -> Result<()> {
        Ok(())
    }

    /// `putReferenceRelease(Object o, long offset, Object x)`：引用写
    ///（release）。载体驱动分派：引用元素数组 → 偏移反解（CHM `setTabAt`，
    /// 协变视图 + aastore 存储检查）；否则实例字段形态（与 putReference
    /// 同一存储单元，S-11）。
    #[jvm_boundary]
    pub fn putReferenceRelease(&self, o: Object, offset: i64, x: Object) -> Result<()> {
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
    /// 单 OS 线程协作调度下读-比-写不可分割。
    #[jvm_boundary]
    pub fn compareAndSetReference(&self, o: Object, offset: i64, expected: Object, x: Object) -> Result<bool> {
        let (current, swap): (Object, Box<dyn FnOnce() -> Result<bool>>) =
            if let Some(arr) = _erased_ref_array(&o) {
                let i = _ref_array_index(offset);
                let cur = arr.get(i)?;
                let arr2 = arr;
                (cur, Box::new(move || arr2.set(i, x).map(|_| true)))
            } else {
                match _instance_ref_get(&o, offset) {
                    Some(cur) => {
                        let holder = o;
                        (cur, Box::new(move || Ok(_instance_ref_set(&holder, offset, x))))
                    }
                    None => panic!("jdk/internal/misc/Unsafe.compareAndSetReference:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z (offset={} 无实例引用字段臂且非引用元素数组)", offset),
                }
            };
        if current == expected {
            return swap();
        }
        Ok(false)
    }

    /// `getAndAddLong(Object o, long offset, long delta)`：原子读取并加 delta，
    /// 返回旧值。
    ///
    /// `o` 为 null 载体是 Unsafe 的**静态字段基址约定**（JDK 里
    /// `Thread$ThreadIdentifiers.next` 以 `getAndAddLong(null, NEXT_TID_OFFSET, 1)`
    /// 推进线程 id 计数）。原生二进制没有原始内存布局：静态原子字以 offset 为
    /// 键的全局计数器承载（键来自 `Thread.getNextThreadIdOffset` 的固定哨兵，
    /// 与 objectFieldOffset 的实例字段不透明 id 无交集——消费面不同）。
    /// `o` 非 null（实例字段原子，如 CHM `addCount` 的 baseCount）经
    /// `__unsafe_long_cell` 的共享存储单元承载。
    #[jvm_boundary]
    pub fn getAndAddLong(&self, o: Object, offset: i64, delta: i64) -> Result<i64> {
        if !o.0.is_jvm_null() {
            let cell = _instance_long_cell(&o, offset).unwrap_or_else(|| {
                panic!("stub: jdk/internal/misc/Unsafe.getAndAddLong:(Ljava/lang/Object;JJ)J (实例字段 offset={} 无共享 long 单元)", offset)
            });
            let old = cell.get();
            cell.set(old.wrapping_add(delta));
            return Ok(old);
        }
        use std::cell::RefCell;
        use std::collections::HashMap;
        thread_local! {
            static CELLS: RefCell<HashMap<i64, i64>> = RefCell::new(HashMap::new());
        }
        Ok(CELLS.with(|cells| {
            let mut cells = cells.borrow_mut();
            let cell = cells.entry(offset).or_insert(0);
            let old = *cell;
            *cell = old.wrapping_add(delta);
            old
        }))
    }

    /// `staticFieldBase(Field)`：静态字存储基址。JDK 返回镜像 Class 对应的
    /// 基址对象；此处返回声明类对象装箱（身份稳定——`for_class` 按名缓存），
    /// 供 (基址身份, 偏移) 键的原子字（getAndAddInt）使用。
    #[jvm_boundary]
    pub fn staticFieldBase(&self, f: crate::java::lang::reflect::Field) -> Result<Object> {
        Ok(Object::from(f.__get_clazz()))
    }

    /// `staticFieldOffset(Field)`：静态字偏移量。与 objectFieldOffset 同约定：
    /// 无原始内存布局，偏移只作不透明标识（线程内递增、同一字段经调用方
    /// 静态存储恒等复用）。
    #[jvm_boundary]
    pub fn staticFieldOffset(&self, f: crate::java::lang::reflect::Field) -> Result<i64> {
        let _ = f;
        use std::cell::RefCell;
        thread_local! {
            static NEXT: RefCell<i64> = const { RefCell::new(1) };
        }
        Ok(NEXT.with(|n| {
            let v = *n.borrow();
            *n.borrow_mut() += 1;
            v
        }))
    }

    /// `getAndAddInt(Object base, long offset, int delta)`：原子读取并加 delta，
    /// 返回旧值。两条消费路径：
    /// - 实例字段原子（offset 出自 objectFieldOffset 登记表，如
    ///   `AtomicInteger.incrementAndGet` 的 value 字段）——经 `__unsafe_int_cell`
    ///   的共享存储单元（写入对 `__get_value` 直读可见）；
    /// - 静态字原子（base 为 staticFieldBase 返回的基址，offset 为
    ///   staticFieldOffset 的不透明 id，如 `Thread$ThreadNumbering.next` 的线程
    ///   名计数）——原生二进制无原始内存，以 (基址身份, 偏移) 键的全局字承载；
    ///   base 为 null 载体时身份取 0（与真实对象身份不冲突）。
    #[jvm_boundary]
    pub fn getAndAddInt(&self, base: Object, offset: i64, delta: i32) -> Result<i32> {
        if !base.0.is_jvm_null() {
            if let Some(cell) = _instance_int_cell(&base, offset) {
                let old = cell.get();
                cell.set(old.wrapping_add(delta));
                return Ok(old);
            }
        }
        use std::cell::RefCell;
        use std::collections::HashMap;
        thread_local! {
            static CELLS: RefCell<HashMap<(usize, i64), i32>> = RefCell::new(HashMap::new());
        }
        let identity = if base.0.is_jvm_null() { 0 } else { base.0.__identity() as usize };
        Ok(CELLS.with(|cells| {
            let mut cells = cells.borrow_mut();
            let cell = cells.entry((identity, offset)).or_insert(0);
            let old = *cell;
            *cell = old.wrapping_add(delta);
            old
        }))
    }

    /// 分配基本类型数组。Rust 侧不存在未初始化内存的可观察差异，元素一律零值
    /// （JDK 规格允许实现返回已清零的数组）。
    #[jvm_boundary(upcalls = "java/lang/IllegalArgumentException.<init>:(Ljava/lang/String;)V")]
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
