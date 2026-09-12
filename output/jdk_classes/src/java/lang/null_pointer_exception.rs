#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/NullPointerException",
    super_class = "java/lang/RuntimeException",
    interfaces  = "",
    access      = "public",
    source      = "NullPointerException.java",
))]
pub struct NullPointerException {
    #[cfg_attr(any(), java_field(name = "extendedMessageState", descriptor = "I", access = "private"))]
    pub extendedMessageState: Field<i32>,
    #[cfg_attr(any(), java_field(name = "extendedMessage", descriptor = "Ljava/lang/String;", access = "private"))]
    pub extendedMessage: Field<String>,
}

impl NullPointerException {
    // java: <init>()V
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { extendedMessageState: Field::new(0), extendedMessage: Field::new(String::new()) };
        /* invokespecial Method java/lang/RuntimeException.<init>:()V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(s: String) -> Result<Self> {
        let this = Self { extendedMessageState: Field::new(0), extendedMessage: Field::new(String::new()) };
        /* invokespecial Method java/lang/RuntimeException.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: fillInStackTrace()Ljava/lang/Throwable;
    pub fn fillInStackTrace(&self) -> Result<Object> {
        let this = self;
        this.extendedMessageState.set(1i32);
        let _t0 = this.getExtendedNPEMessage()?;
        this.extendedMessage.set(_t0);
        this.extendedMessageState.set(2i32);
        let _t1: Object = RuntimeException::fillInStackTrace()?;
        Ok(_t1)
    }

    // java: getMessage()Ljava/lang/String;
    pub fn getMessage(&self) -> Result<String> {
        let this = self;
        let _t0: String = RuntimeException::getMessage()?;
        let mut message: String = _t0;
        let mut local_2: NullPointerException = this;
        /* TODO: monitorenter  */
        let _t1 = this.getExtendedNPEMessage()?;
        this.extendedMessage.set(_t1);
        this.extendedMessageState.set(2i32);
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_3: String = this.extendedMessage.get();
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(message)
    }

    // java: getExtendedNPEMessage()Ljava/lang/String;
    pub fn getExtendedNPEMessage(&self) -> Result<String> {
        todo!("native java/lang/NullPointerException.getExtendedNPEMessage")
    }
}
