use crate::prelude::*;
use super::shared_secrets::SharedSecrets;
use super::java_lang_access_impl::SystemJavaLangAccess;
use std::cell::RefCell;

// SharedSecrets 的 static 槽位：各公开 API 类在自己的 <clinit> 里登记访问器实例。
// 按调用链按需实现，其余槽位保持 stub。
thread_local! {
    static JAVA_IO_FILE_DESCRIPTOR_ACCESS: RefCell<Option<Object>> = const { RefCell::new(None) };
    static JAVA_IO_PRINT_STREAM_ACCESS: RefCell<Option<Object>> = const { RefCell::new(None) };
    static JAVA_LANG_ACCESS: RefCell<Option<Object>> = const { RefCell::new(None) };
    static JAVA_LANG_REF_ACCESS: RefCell<Option<Object>> = const { RefCell::new(None) };
}

impl SharedSecrets {
    #[jvm_boundary]
    pub fn setJavaIOFileDescriptorAccess(jiofda: Object) -> Result<()> {
        JAVA_IO_FILE_DESCRIPTOR_ACCESS.with(|slot| *slot.borrow_mut() = Some(jiofda));
        Ok(())
    }

    /// 槽位为空时先触发 FileDescriptor 的类初始化（对应 ensureClassInitialized），由其 <clinit> 登记。
    #[jvm_boundary]
    pub fn getJavaIOFileDescriptorAccess() -> Result<Object> {
        if JAVA_IO_FILE_DESCRIPTOR_ACCESS.with(|slot| slot.borrow().is_none()) {
            crate::java::io::FileDescriptor::__class_init()?;
        }
        Ok(JAVA_IO_FILE_DESCRIPTOR_ACCESS.with(|slot| slot.borrow().clone()).unwrap_or_default())
    }

    #[jvm_boundary]
    pub fn setJavaIOCPrintStreamAccess(a: Object) -> Result<()> {
        JAVA_IO_PRINT_STREAM_ACCESS.with(|slot| *slot.borrow_mut() = Some(a));
        Ok(())
    }

    #[jvm_boundary]
    pub fn getJavaIOPrintStreamAccess() -> Result<Object> {
        Ok(JAVA_IO_PRINT_STREAM_ACCESS.with(|slot| slot.borrow().clone()).unwrap_or_default())
    }

    #[jvm_boundary]
    pub fn setJavaLangAccess(jla: Object) -> Result<()> {
        JAVA_LANG_ACCESS.with(|slot| *slot.borrow_mut() = Some(jla));
        Ok(())
    }

    /// `Reference.<clinit>` 登记的引用处理访问器（waitForReferenceProcessing 等）。
    /// 原生二进制无引用处理器线程 / GC，访问器无从消费——按 JDK 形态存储，
    /// 槽位无读取方。
    #[jvm_boundary]
    pub fn setJavaLangRefAccess(a: Object) -> Result<()> {
        JAVA_LANG_REF_ACCESS.with(|slot| *slot.borrow_mut() = Some(a));
        Ok(())
    }

    /// JDK 中由 `System.<clinit>` → `setJavaLangAccess()` 登记 `System$JavaLangAccess`
    /// 实例；`System` 的 `<clinit>` 翻译未覆盖该内部类构造（边界截断），此处槽位
    /// 为空时直接构造登记——首次取用即生效，与 JDK「初始化后必有实例」的语义一致。
    #[jvm_boundary]
    pub fn getJavaLangAccess() -> Result<Object> {
        if JAVA_LANG_ACCESS.with(|slot| slot.borrow().is_none()) {
            let obj = Object::from(SystemJavaLangAccess);
            JAVA_LANG_ACCESS.with(|slot| *slot.borrow_mut() = Some(obj));
        }
        Ok(JAVA_LANG_ACCESS.with(|slot| slot.borrow().clone()).unwrap_or_default())
    }
}
