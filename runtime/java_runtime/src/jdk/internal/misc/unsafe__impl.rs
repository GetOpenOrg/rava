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
