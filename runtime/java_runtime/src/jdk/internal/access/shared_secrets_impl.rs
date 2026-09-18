use crate::prelude::*;
use super::shared_secrets::SharedSecrets;
use std::cell::RefCell;

// SharedSecrets 的 static 槽位：各公开 API 类在自己的 <clinit> 里登记访问器实例。
// 按调用链按需实现，其余槽位保持 stub。
thread_local! {
    static JAVA_IO_FILE_DESCRIPTOR_ACCESS: RefCell<Option<Object>> = const { RefCell::new(None) };
    static JAVA_IO_PRINT_STREAM_ACCESS: RefCell<Option<Object>> = const { RefCell::new(None) };
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
}
