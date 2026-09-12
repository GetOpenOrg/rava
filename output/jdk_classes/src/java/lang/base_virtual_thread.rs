#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/BaseVirtualThread",
    super_class = "java/lang/Thread",
    interfaces  = "",
    access      = "abstract",
    source      = "BaseVirtualThread.java",
))]
pub struct BaseVirtualThread;

impl BaseVirtualThread {
    // java: <init>(Ljava/lang/String;IZ)V
    pub fn new(name: String, characteristics: i32, bound: bool) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Thread.<init>:(Ljava/lang/String;IZ)V */
        Ok(this)
    }

    // java: park()V
    pub fn park(&self) -> Result<()> {
        todo!("abstract java/lang/BaseVirtualThread.park")
    }

    // java: parkNanos(J)V
    pub fn parkNanos(&self, arg0: i64) -> Result<()> {
        todo!("abstract java/lang/BaseVirtualThread.parkNanos")
    }

    // java: unpark()V
    pub fn unpark(&self) -> Result<()> {
        todo!("abstract java/lang/BaseVirtualThread.unpark")
    }
}
