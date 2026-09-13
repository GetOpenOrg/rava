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
    pub fn new(&self) -> Result<()> {
        panic!("stub: java/lang/AssertionError.<init>:()V")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, detailMessage: String) -> Result<()> {
        panic!("stub: java/lang/AssertionError.<init>:(Ljava/lang/String;)V")
    }

    // java: <init>(Ljava/lang/Object;)V
    pub fn new__obj(&self, detailMessage: Object) -> Result<()> {
        panic!("stub: java/lang/AssertionError.<init>:(Ljava/lang/Object;)V")
    }

    // java: <init>(Z)V
    pub fn new__z(&self, detailMessage: bool) -> Result<()> {
        panic!("stub: java/lang/AssertionError.<init>:(Z)V")
    }

    // java: <init>(C)V
    pub fn new__c(&self, detailMessage: u16) -> Result<()> {
        panic!("stub: java/lang/AssertionError.<init>:(C)V")
    }

    // java: <init>(I)V
    pub fn new__i(&self, detailMessage: i32) -> Result<()> {
        panic!("stub: java/lang/AssertionError.<init>:(I)V")
    }

    // java: <init>(J)V
    pub fn new__l(&self, detailMessage: i64) -> Result<()> {
        panic!("stub: java/lang/AssertionError.<init>:(J)V")
    }

    // java: <init>(F)V
    pub fn new__f(&self, detailMessage: f32) -> Result<()> {
        panic!("stub: java/lang/AssertionError.<init>:(F)V")
    }

    // java: <init>(D)V
    pub fn new__d(&self, detailMessage: f64) -> Result<()> {
        panic!("stub: java/lang/AssertionError.<init>:(D)V")
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;)V
    pub fn new__str_throwa(&self, message: String, cause: Object) -> Result<()> {
        panic!("stub: java/lang/AssertionError.<init>:(Ljava/lang/String;Ljava/lang/Throwable;)V")
    }
}
