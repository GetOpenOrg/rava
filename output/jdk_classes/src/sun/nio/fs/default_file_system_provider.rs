#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "sun/nio/fs/DefaultFileSystemProvider",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public",
    source      = "DefaultFileSystemProvider.java",
))]
pub struct DefaultFileSystemProvider;

impl DefaultFileSystemProvider {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "instance", descriptor = "()Lsun/nio/fs/MacOSXFileSystemProvider;", access = "public static"))]
    pub fn instance() -> Result<Object> {
        Ok(DefaultFileSystemProvider::INSTANCE())
    }

    #[cfg_attr(any(), java_method(name = "theFileSystem", descriptor = "()Ljava/nio/file/FileSystem;", access = "public static"))]
    pub fn theFileSystem() -> Result<Object> {
        let _t0 = DefaultFileSystemProvider::INSTANCE().theFileSystem()?;
        Ok(_t0)
    }
}
