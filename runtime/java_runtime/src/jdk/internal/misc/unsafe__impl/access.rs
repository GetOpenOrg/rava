//! Unsafe 的 int / long / 引用槽访问：CAS、volatile / opaque / acquire / release、park、静态字段基址（宿主 unsafe__impl.rs 的私有辅助模块）

use super::*;

impl Unsafe {
    /// `compareAndSetLong(Object o, long offset, long expected, long x)`：long 槽的 CAS——
    /// 统一载体（`unsafe__ext`）在该槽的存储上原子完成读-比-写：实例字段双字视图 / 静态字段
    /// 写锁 / 基本类型数组与直接内存的字节视图。JDK compareAndSetDouble 以原始位经此。
    #[jvm_native]
    pub fn compareAndSetLong(&self, o: Object, offset: i64, expected: i64, x: i64) -> Result<bool> {
        Ok(_ext::cas(&o, offset, 8, "compareAndSetLong:(Ljava/lang/Object;JJJ)Z", expected as u64, x as u64)? == expected as u64)
    }

    /// `compareAndExchangeLong(o, offset, expected, x)`：CAS 并返回**见证值**（交换前的
    /// 当前值；等于 expected 即交换成功）。读-比-写经统一载体原子完成（与
    /// compareAndSetLong 同一存储单元）。消费方：JDK25 ForkJoinPool.compareAndExchangeCtl
    ///（signalWork 的 ctl 状态字）。native。
    #[jvm_native]
    pub fn compareAndExchangeLong(&self, o: Object, offset: i64, expected: i64, x: i64) -> Result<i64> {
        Ok(_ext::cas(&o, offset, 8, "compareAndExchangeLong:(Ljava/lang/Object;JJJ)J", expected as u64, x as u64)? as i64)
    }

    /// `getLongVolatile(Object o, long offset)`：volatile 读——存储单元与 plain 同一（实例原子单元 /
    /// 静态字段 / 原生内存），内存序见下方「基本类型 volatile 访问」节。
    #[jvm_native]
    pub fn getLongVolatile(&self, o: Object, offset: i64) -> Result<i64> {
        _volatile_load(|| self.getLong_obj_l(o, offset))
    }

    /// `putLongVolatile(Object o, long offset, long x)`：volatile 写（单元与 plain 同一）。
    /// `AtomicLong.set` 等经此路径——写入对 `__get_value` 直读可见。
    #[jvm_native]
    pub fn putLongVolatile(&self, o: Object, offset: i64, x: i64) -> Result<()> {
        _volatile_store(|| self.putLong_obj_l_l(o, offset, x))
    }

    /// `putLong(Object o, long offset, long x)`：实例字段 plain 写（与
    /// putLongVolatile 同一存储单元；原子单元，plain 写不弱于 volatile 写）。
    /// 消费方：`ThreadLocalRandom.localInit` 对 Thread.threadLocalRandomSeed。
    #[jvm_native]
    pub fn putLong_obj_l_l(&self, o: Object, offset: i64, x: i64) -> Result<()> {
        _ext::put(&o, offset, 8, "putLong:(Ljava/lang/Object;JJ)V", x as u64)
    }

    /// `getLong(Object o, long offset)`：实例字段 long 读（plain 形态，与
    /// getLongVolatile 同一存储单元）。
    /// 消费方：`ThreadLocalRandom.nextSeed` 对 Thread.threadLocalRandomSeed
    /// （读改写种子的读半边；localInit 的写半边是 putLong_obj_l_l）。
    #[jvm_native]
    pub fn getLong_obj_l(&self, o: Object, offset: i64) -> Result<i64> {
        Ok(_ext::get(&o, offset, 8, "getLong:(Ljava/lang/Object;J)J")? as i64)
    }

    /// `getInt(Object o, long offset)`：实例字段 int 读（plain 形态）。
    /// 消费方：`ThreadLocalRandom.current` 对 Thread.threadLocalRandomProbe。
    #[jvm_native]
    pub fn getInt_obj_l(&self, o: Object, offset: i64) -> Result<i32> {
        Ok(_ext::get(&o, offset, 4, "getInt:(Ljava/lang/Object;J)I")? as u32 as i32)
    }

    /// `putInt(Object o, long offset, int x)`：实例字段 int 写（plain 形态）。
    #[jvm_native]
    pub fn putInt_obj_l_i(&self, o: Object, offset: i64, x: i32) -> Result<()> {
        _ext::put(&o, offset, 4, "putInt:(Ljava/lang/Object;JI)V", x as u32 as u64)
    }

    /// `compareAndSetInt(Object o, long offset, int expected, int x)`：int 槽的 CAS（统一载体，
    /// compareAndSetLong 的 32 位镜像）。JDK compareAndSetFloat 以原始位、子字 CAS
    ///（compareAndExchangeByte / Short）以 `offset & ~3` 的字经此。
    #[jvm_native]
    pub fn compareAndSetInt(&self, o: Object, offset: i64, expected: i32, x: i32) -> Result<bool> {
        Ok(_ext::cas(&o, offset, 4, "compareAndSetInt:(Ljava/lang/Object;JII)Z", expected as u32 as u64, x as u32 as u64)? == expected as u32 as u64)
    }

    /// `compareAndExchangeInt(o, offset, expected, x)`：int 形态的见证值 CAS
    ///（compareAndExchangeLong 的同族对偶）。native。
    #[jvm_native]
    pub fn compareAndExchangeInt(&self, o: Object, offset: i64, expected: i32, x: i32) -> Result<i32> {
        Ok(_ext::cas(&o, offset, 4, "compareAndExchangeInt:(Ljava/lang/Object;JII)I", expected as u32 as u64, x as u32 as u64)? as u32 as i32)
    }

    /// `compareAndExchangeReference(o, offset, expected, x)`：引用见证值 CAS——
    /// 数组槽位 / 实例字段两臂与 compareAndSetReference 同一载体分派，比较按
    /// Java `==`（对象身份）。消费方：JDK25 ForkJoinTask 的 aux 等待链。native。
    #[jvm_native]
    pub fn compareAndExchangeReference(&self, o: Object, offset: i64, expected: Object, x: Object) -> Result<Object> {
        let mut x = Some(x);
        _ref_rmw(&o, offset,
            "compareAndExchangeReference:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &mut |cur| if cur == expected { x.take() } else { None })
    }

    /// `getIntVolatile(Object o, long offset)`：int volatile 读（单元与 plain 同一）。
    #[jvm_native]
    pub fn getIntVolatile(&self, o: Object, offset: i64) -> Result<i32> {
        _volatile_load(|| self.getInt_obj_l(o, offset))
    }

    /// `putIntVolatile(Object o, long offset, int x)`：int volatile 写（单元与 plain 同一）。
    #[jvm_native]
    pub fn putIntVolatile(&self, o: Object, offset: i64, x: i32) -> Result<()> {
        _volatile_store(|| self.putInt_obj_l_i(o, offset, x))
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
    #[jvm_native]
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
    #[jvm_native]
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

    /// `getReferenceVolatile(Object o, long offset)`：引用 volatile 读。单元与 plain 同一，
    /// 内存序同「基本类型 volatile 访问」节的栅栏包络：volatile 访问模式可作用于非 volatile
    /// 声明的位置（引用元素数组、普通字段），单元自身的序（普通族 Acquire / Release）不足以
    /// 进入 SeqCst 全序，由前导 SeqCst 栅栏补足（无 GC 文档第四节小步 B）。
    #[jvm_native]
    pub fn getReferenceVolatile(&self, o: Object, offset: i64) -> Result<Object> {
        _volatile_load(|| self.getReference(o, offset))
    }

    /// `putReferenceVolatile(Object o, long offset, Object x)`：引用 volatile 写（栅栏包络同上，
    /// 尾随 SeqCst 栅栏）。
    #[jvm_native]
    pub fn putReferenceVolatile(&self, o: Object, offset: i64, x: Object) -> Result<()> {
        _volatile_store(|| self.putReference(o, offset, x))
    }

    /// `park(boolean isAbsolute, long time)`：LockSupport.park 的 VM 底座（permit 语义的
    /// 阻塞驻留，`monitor::park`）。许可按当前线程对象身份登记。blocker 字段
    /// （parkBlocker）由上层 `putReferenceOpaque` 携带。
    #[jvm_native]
    pub fn park(&self, is_absolute: bool, time: i64) -> Result<()> {
        // HotSpot Parker 挂在 JavaThread（载体）上：虚拟线程被 pin 时经 parkOnCarrierThread 在载体上停泊，
        // VirtualThread.unpark 对应地 U.unpark(carrier)
        crate::monitor::park(crate::monitor::current_thread_identity()?, is_absolute, time);
        Ok(())
    }

    /// `unpark(Object thread)`：LockSupport.unpark 的 VM 底座——授予目标线程许可并唤醒。
    /// null 线程静默（HotSpot Unsafe_Unpark 同判定）。
    #[jvm_native]
    pub fn unpark(&self, thread: Object) -> Result<()> {
        if !thread.0.is_jvm_null() {
            crate::monitor::unpark(thread.0.__identity() as usize);
        }
        Ok(())
    }

    /// `compareAndSetReference(Object o, long offset, Object expected, Object x)`：
    /// 槽位/字段 CAS。载体驱动分派：引用元素数组（CHM `casTabAt`）按偏移
    /// 反解；实例字段（BufferedInputStream.close 的 buf 清空）走登记表反查
    /// + 引用原子协议。比较按 Java `==`（对象身份，`PartialEq for Object`）；
    /// 读-比-写在存储写锁内完成（`_ref_rmw`），并行后端下真正原子。
    #[jvm_native]
    pub fn compareAndSetReference(&self, o: Object, offset: i64, expected: Object, x: Object) -> Result<bool> {
        let mut x = Some(x);
        let old = _ref_rmw(&o, offset,
            "compareAndSetReference:(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z",
            &mut |cur| if cur == expected { x.take() } else { None })?;
        Ok(old == expected)
    }

    /// native `staticFieldOffset0(Field)`：静态字段偏移。无原始内存布局，偏移是按 (声明类, 字段名) 登记的
    /// 稳定不透明 id（同一字段恒同一 id，JDK 语义），取值区间与 objectFieldOffset 的实例字段 id 不相交
    ///（`reflect_dispatch::STATIC_FIELD_ID_BASE` 起）：引用访问器据此把 (staticFieldBase, 偏移) 路由到声明类的
    /// 静态存储（引用族 `_static_ref_get/set` / `_ref_rmw`，基本类型族 `unsafe__ext::prim`，均经字段闭包）。
    /// 公开包装 `staticFieldOffset(Field)`（判空）按 JDK 字节码翻译。
    #[jvm_native]
    pub fn staticFieldOffset0(&self, f: crate::java::lang::reflect::Field) -> Result<i64> {
        let decl = format!("{}", f.__get_clazz().__get_name()).replace('.', "/");
        let name = format!("{}", f.__get_name());
        Ok(_static_field_id(decl, name))
    }

    /// native `staticFieldBase0(Field)`：静态字段基址——声明类的类镜像（HotSpot 同为 mirror）。
    #[jvm_native]
    pub fn staticFieldBase0(&self, f: crate::java::lang::reflect::Field) -> Result<Object> {
        Ok(Object::from(f.__get_clazz()))
    }
}
