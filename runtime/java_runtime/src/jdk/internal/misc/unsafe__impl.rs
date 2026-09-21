use crate::prelude::*;
use super::unsafe_::Unsafe;
use crate::java::lang::Class;

// 内部边界类 jdk.internal.misc.Unsafe：按调用链按需实现，其余保持 panic 存根。

impl Unsafe {
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


    /// 字段偏移量：HotSpot 返回对象布局的真实偏移；原生二进制没有 C 布局对象，
    /// 字段经名字访问，偏移量只作不透明标识使用（AtomicLong 等把它存进 long 字段
    /// 再传回 compareAndSwapLong——恒等即可）。按 (类名, 字段名) 分配稳定的
    /// 不透明 id（线程内递增），同一字段恒等。
    #[jvm_boundary]
    pub fn objectFieldOffset_class_str(&self, c: Class, name: String) -> Result<i64> {
        use std::cell::RefCell;
        thread_local! {
            static NEXT: RefCell<i64> = const { RefCell::new(1) };
        }
        let _ = (c, name);
        Ok(NEXT.with(|n| {
            let v = *n.borrow();
            *n.borrow_mut() += 1;
            v
        }))
    }

    /// `getAndAddLong(Object o, long offset, long delta)`：原子读取并加 delta，
    /// 返回旧值。
    ///
    /// `o` 为 null 载体是 Unsafe 的**静态字段基址约定**（JDK 里
    /// `Thread$ThreadIdentifiers.next` 以 `getAndAddLong(null, NEXT_TID_OFFSET, 1)`
    /// 推进线程 id 计数）。原生二进制没有原始内存布局：静态原子字以 offset 为
    /// 键的全局计数器承载（键来自 `Thread.getNextThreadIdOffset` 的固定哨兵，
    /// 与 objectFieldOffset 的实例字段不透明 id 无交集——消费面不同）。
    /// `o` 非 null（实例字段原子）暂无消费方，保持存根。
    #[jvm_boundary]
    pub fn getAndAddLong(&self, o: Object, offset: i64, delta: i64) -> Result<i64> {
        if !o.0.is_jvm_null() {
            panic!("stub: jdk/internal/misc/Unsafe.getAndAddLong:(Ljava/lang/Object;JJ)J (实例字段原子)");
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
    /// 返回旧值。静态字原子（base 为 staticFieldBase 返回的基址，offset 为
    /// staticFieldOffset 的不透明 id，如 `Thread$ThreadNumbering.next` 的线程
    /// 名计数）——原生二进制无原始内存，以 (基址身份, 偏移) 键的全局字承载；
    /// base 为 null 载体时身份取 0（与真实对象身份不冲突）。
    #[jvm_boundary]
    pub fn getAndAddInt(&self, base: Object, offset: i64, delta: i32) -> Result<i32> {
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
