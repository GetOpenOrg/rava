use crate::prelude::*;
use super::shared_secrets::SharedSecrets;
use super::java_lang_access_impl::SystemJavaLangAccess;
use super::java_util_collection_access_impl::ImmutableCollectionsCollAccess;
use crate::sync_model::__RefSlot as RefCell;

// SharedSecrets 的 static 槽位：各公开 API 类在自己的 <clinit> 里登记访问器实例。
// 存取签名对访问器接口类型泛型（T-2 7a）：接口载体化后调用方以载体类型（JavaLangAccess 等）存取，
// 载体未入闭包时仍以 Object 存取——槽位统一按 Object 身份存储，两种调用形态共用。
// 按调用链按需实现，其余槽位保持 stub。
crate::__process_static! {
    static JAVA_IO_FILE_DESCRIPTOR_ACCESS: RefCell<Option<Object>> = const { RefCell::new(None) };
    static JAVA_IO_PRINT_STREAM_ACCESS: RefCell<Option<Object>> = const { RefCell::new(None) };
    static JAVA_LANG_ACCESS: RefCell<Option<Object>> = const { RefCell::new(None) };
    static JAVA_LANG_INVOKE_ACCESS: RefCell<Option<Object>> = const { RefCell::new(None) };
    static JAVA_LANG_REF_ACCESS: RefCell<Option<Object>> = const { RefCell::new(None) };
    static JAVA_LANG_REFLECT_ACCESS: RefCell<Option<Object>> = const { RefCell::new(None) };
    static JAVA_UTIL_COLLECTION_ACCESS: RefCell<Option<Object>> = const { RefCell::new(None) };
    static JAVA_UTIL_CONCURRENT_FJP_ACCESS: RefCell<Option<Object>> = const { RefCell::new(None) };
}

impl SharedSecrets {
    #[jvm_boundary]
    pub fn setJavaIOFileDescriptorAccess(jiofda: impl Into<Object>) -> Result<()> {
        JAVA_IO_FILE_DESCRIPTOR_ACCESS.with(|slot| *slot.borrow_mut() = Some(jiofda.into()));
        Ok(())
    }

    /// 槽位为空时先触发 FileDescriptor 的类初始化（对应 ensureClassInitialized），由其 <clinit> 登记。
    #[jvm_boundary]
    pub fn getJavaIOFileDescriptorAccess<T: From<Object>>() -> Result<T> {
        if JAVA_IO_FILE_DESCRIPTOR_ACCESS.with(|slot| slot.borrow().is_none()) {
            crate::java::io::FileDescriptor::__class_init()?;
        }
        Ok(T::from(JAVA_IO_FILE_DESCRIPTOR_ACCESS.with(|slot| slot.borrow().clone()).unwrap_or_default()))
    }

    #[jvm_boundary]
    pub fn setJavaIOCPrintStreamAccess(a: impl Into<Object>) -> Result<()> {
        JAVA_IO_PRINT_STREAM_ACCESS.with(|slot| *slot.borrow_mut() = Some(a.into()));
        Ok(())
    }

    #[jvm_boundary]
    pub fn getJavaIOPrintStreamAccess<T: From<Object>>() -> Result<T> {
        Ok(T::from(JAVA_IO_PRINT_STREAM_ACCESS.with(|slot| slot.borrow().clone()).unwrap_or_default()))
    }

    #[jvm_boundary]
    pub fn setJavaLangAccess(jla: impl Into<Object>) -> Result<()> {
        JAVA_LANG_ACCESS.with(|slot| *slot.borrow_mut() = Some(jla.into()));
        Ok(())
    }

    /// `MethodHandleImpl.<clinit>` 登记的 `JavaLangInvokeAccess`（MethodHandleImpl$1）。
    #[jvm_boundary]
    pub fn setJavaLangInvokeAccess(a: impl Into<Object>) -> Result<()> {
        JAVA_LANG_INVOKE_ACCESS.with(|slot| *slot.borrow_mut() = Some(a.into()));
        Ok(())
    }

    /// JDK：槽位为空时 `Class.forName("java.lang.invoke.MethodHandleImpl", true, null)` 触发其
    /// <clinit> 登记。消费方 `MethodHandleAccessorFactory$LazyStaticHolder.<clinit>`（反射访问器族）。
    #[jvm_boundary(upcalls = "java/lang/invoke/MethodHandleImpl.<clinit>:()V")]
    pub fn getJavaLangInvokeAccess<T: From<Object>>() -> Result<T> {
        if JAVA_LANG_INVOKE_ACCESS.with(|slot| slot.borrow().is_none()) {
            crate::java::lang::invoke::MethodHandleImpl::__class_init()?;
        }
        Ok(T::from(JAVA_LANG_INVOKE_ACCESS.with(|slot| slot.borrow().clone()).unwrap_or_default()))
    }

    /// `AccessibleObject.<clinit>` 登记的 `ReflectAccess`（反射对象复制 / 访问器槽位，
    /// ReflectionFactory 构造时取用）。
    #[jvm_boundary]
    pub fn setJavaLangReflectAccess(a: impl Into<Object>) -> Result<()> {
        JAVA_LANG_REFLECT_ACCESS.with(|slot| *slot.borrow_mut() = Some(a.into()));
        Ok(())
    }

    /// HotSpot 在 VM 引导期即初始化 AccessibleObject（登记 ReflectAccess），JDK 的取用方
    ///（ReflectionFactory 构造器）依赖该时序；惰性类初始化下槽位为空时先触发其 <clinit>
    ///（与 getJavaIOFileDescriptorAccess 同一约定）。
    #[jvm_boundary(upcalls = "java/lang/reflect/ReflectAccess.<init>:()V")]
    pub fn getJavaLangReflectAccess<T: From<Object>>() -> Result<T> {
        if JAVA_LANG_REFLECT_ACCESS.with(|slot| slot.borrow().is_none()) {
            crate::java::lang::reflect::AccessibleObject::__class_init()?;
        }
        Ok(T::from(JAVA_LANG_REFLECT_ACCESS.with(|slot| slot.borrow().clone()).unwrap_or_default()))
    }

    /// `Reference.<clinit>` 登记的引用处理访问器（waitForReferenceProcessing 等）。
    /// 原生二进制无引用处理器线程 / GC，访问器无从消费——按 JDK 形态存储，
    /// 槽位无读取方。
    #[jvm_boundary]
    pub fn setJavaLangRefAccess(a: impl Into<Object>) -> Result<()> {
        JAVA_LANG_REF_ACCESS.with(|slot| *slot.borrow_mut() = Some(a.into()));
        Ok(())
    }

    /// `ForkJoinPool.<clinit>` 登记的 FJP 访问器（容器/配置查询，供
    /// serviceability 与虚拟线程层消费）。当前闭包内无读取方
    /// （getJavaUtilConcurrentFJPAccess 未被触达）——按 JDK 形态存储即可。
    #[jvm_boundary]
    pub fn setJavaUtilConcurrentFJPAccess(a: impl Into<Object>) -> Result<()> {
        JAVA_UTIL_CONCURRENT_FJP_ACCESS.with(|slot| *slot.borrow_mut() = Some(a.into()));
        Ok(())
    }

    /// JDK 中由 `System.<clinit>` → `setJavaLangAccess()` 登记 `System$JavaLangAccess`
    /// 实例；`System` 的 `<clinit>` 翻译未覆盖该内部类构造（边界截断），此处槽位
    /// 为空时直接构造登记——首次取用即生效，与 JDK「初始化后必有实例」的语义一致。
    #[jvm_boundary]
    pub fn getJavaLangAccess<T: From<Object>>() -> Result<T> {
        if JAVA_LANG_ACCESS.with(|slot| slot.borrow().is_none()) {
            let obj = Object::from(SystemJavaLangAccess);
            JAVA_LANG_ACCESS.with(|slot| *slot.borrow_mut() = Some(obj));
        }
        Ok(T::from(JAVA_LANG_ACCESS.with(|slot| slot.borrow().clone()).unwrap_or_default()))
    }

    /// JDK 中由 `ImmutableCollections.<clinit>` 以匿名类登记（转发该类同名静态，
    /// `Stream.toList` 的 trusted-array 路径消费）。与 getJavaLangAccess 同约定：
    /// 槽位为空时直接构造登记（无状态对象，首次取用即生效）。
    ///
    /// upcalls：匿名类的两个转发目标静态不在任何字节码调用边上（BFS 在本边界
    /// 截断），经此声明拉入闭包——触达即翻译，`listFromTrustedArrayNullsAllowed`
    /// （ReferencePipeline.toList 实际消费的变体）不再停留 panic 存根。
    #[jvm_boundary(upcalls = "java/util/ImmutableCollections.listFromTrustedArray:([Ljava/lang/Object;)Ljava/util/List; java/util/ImmutableCollections.listFromTrustedArrayNullsAllowed:([Ljava/lang/Object;)Ljava/util/List;")]
    pub fn getJavaUtilCollectionAccess<T: From<Object>>() -> Result<T> {
        if JAVA_UTIL_COLLECTION_ACCESS.with(|slot| slot.borrow().is_none()) {
            let obj = Object::from(ImmutableCollectionsCollAccess);
            JAVA_UTIL_COLLECTION_ACCESS.with(|slot| *slot.borrow_mut() = Some(obj));
        }
        Ok(T::from(JAVA_UTIL_COLLECTION_ACCESS.with(|slot| slot.borrow().clone()).unwrap_or_default()))
    }
}
