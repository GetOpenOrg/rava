#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/RuntimePermission",
    super_class = "java/security/BasicPermission",
    interfaces  = "",
    access      = "public final",
    source      = "RuntimePermission.java",
))]
pub struct RuntimePermission;

impl RuntimePermission {
    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(name: String) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/security/BasicPermission.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;Ljava/lang/String;)V
    pub fn new__str_str(name: String, actions: String) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/security/BasicPermission.<init>:(Ljava/lang/String;Ljava/lang/String;)V */
        Ok(this)
    }
}
