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
        todo!("abstract java/lang/AssertionError.<init>")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, detailMessage: String) -> Result<()> {
        todo!("abstract java/lang/AssertionError.<init>")
    }

    // java: <init>(Ljava/lang/Object;)V
    pub fn new__obj(&self, detailMessage: Object) -> Result<()> {
        todo!("abstract java/lang/AssertionError.<init>")
    }

    // java: <init>(Z)V
    pub fn new__z(&self, detailMessage: bool) -> Result<()> {
        todo!("abstract java/lang/AssertionError.<init>")
    }

    // java: <init>(C)V
    pub fn new__c(&self, detailMessage: u16) -> Result<()> {
        todo!("abstract java/lang/AssertionError.<init>")
    }

    // java: <init>(I)V
    pub fn new__i(&self, detailMessage: i32) -> Result<()> {
        todo!("abstract java/lang/AssertionError.<init>")
    }

    // java: <init>(J)V
    pub fn new__l(&self, detailMessage: i64) -> Result<()> {
        todo!("abstract java/lang/AssertionError.<init>")
    }

    // java: <init>(F)V
    pub fn new__f(&self, detailMessage: f32) -> Result<()> {
        todo!("abstract java/lang/AssertionError.<init>")
    }

    // java: <init>(D)V
    pub fn new__d(&self, detailMessage: f64) -> Result<()> {
        todo!("abstract java/lang/AssertionError.<init>")
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;)V
    pub fn new__str_throwa(&self, message: String, cause: Object) -> Result<()> {
        todo!("abstract java/lang/AssertionError.<init>")
    }
}
