#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/AssertionError",
    super_class = "java/lang/Error",
    interfaces  = "",
    access      = "public",
    source      = "AssertionError.java",
))]
pub struct AssertionError;

impl AssertionError {
    // java: <init>()V
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Error.<init>:()V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(detailMessage: String) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Error.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/Object;)V
    // java: <init>(Ljava/lang/Object;)V
    pub fn new__obj(detailMessage: Object) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/AssertionError.<init>:(Ljava/lang/String;)V */
        let _t0 = this.initCause(detailMessage)?;
        Ok(this)
    }

    // java: <init>(Z)V
    // java: <init>(Z)V
    pub fn new__z(detailMessage: bool) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/AssertionError.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: <init>(C)V
    // java: <init>(C)V
    pub fn new__c(detailMessage: u16) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/AssertionError.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: <init>(I)V
    // java: <init>(I)V
    pub fn new__i(detailMessage: i32) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/AssertionError.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: <init>(J)V
    // java: <init>(J)V
    pub fn new__l(detailMessage: i64) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/AssertionError.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: <init>(F)V
    // java: <init>(F)V
    pub fn new__f(detailMessage: f32) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/AssertionError.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: <init>(D)V
    // java: <init>(D)V
    pub fn new__d(detailMessage: f64) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/AssertionError.<init>:(Ljava/lang/String;)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;)V
    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;)V
    pub fn new__str_throwa(message: String, cause: Object) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Error.<init>:(Ljava/lang/String;Ljava/lang/Throwable;)V */
        Ok(this)
    }
}
