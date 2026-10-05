//! Unsafe 的 int / long / 引用槽访问：CAS、volatile / opaque / acquire / release、park、静态字段基址（宿主 unsafe__impl.rs 的私有辅助模块）

use super::*;

impl Unsafe {
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
        // HotSpot Parker 挂在 JavaThread（载体）上：虚拟线程被 pin 时经 parkOnCarrierThread 在载体上停泊，
        // VirtualThread.unpark 对应地 U.unpark(carrier)
        crate::monitor::park(crate::monitor::current_thread_identity()?, is_absolute, time);
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
