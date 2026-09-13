#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Throwable",
    super_class = "java/lang/Object",
    interfaces  = "java/io/Serializable",
    access      = "public",
    source      = "Throwable.java",
))]
pub struct Throwable {
    #[cfg_attr(any(), java_field(name = "backtrace", descriptor = "Ljava/lang/Object;", access = "private"))]
    pub backtrace: Field<Object>,
    #[cfg_attr(any(), java_field(name = "detailMessage", descriptor = "Ljava/lang/String;", access = "private"))]
    pub detailMessage: Field<String>,
    #[cfg_attr(any(), java_field(name = "cause", descriptor = "Ljava/lang/Throwable;", access = "private"))]
    pub cause: Field<Object>,
    #[cfg_attr(any(), java_field(name = "stackTrace", descriptor = "[Ljava/lang/StackTraceElement;", access = "private"))]
    pub stackTrace: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "depth", descriptor = "I", access = "private"))]
    pub depth: Field<i32>,
    #[cfg_attr(any(), java_field(name = "suppressedExceptions", descriptor = "Ljava/util/List;", access = "private"))]
    pub suppressedExceptions: Field<Object>,
}

impl Throwable {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        panic!("stub: java/lang/Throwable.<init>:()V")
    }

    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, message: String) -> Result<()> {
        panic!("stub: java/lang/Throwable.<init>:(Ljava/lang/String;)V")
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;)V
    pub fn new__str_throwa(&self, message: String, cause: Object) -> Result<()> {
        panic!("stub: java/lang/Throwable.<init>:(Ljava/lang/String;Ljava/lang/Throwable;)V")
    }

    // java: <init>(Ljava/lang/Throwable;)V
    pub fn new__throwa(&self, cause: Object) -> Result<()> {
        panic!("stub: java/lang/Throwable.<init>:(Ljava/lang/Throwable;)V")
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;ZZ)V
    pub fn new__str_throwa_z_z(&self, message: String, cause: Object, enableSuppression: bool, writableStackTrace: bool) -> Result<()> {
        panic!("stub: java/lang/Throwable.<init>:(Ljava/lang/String;Ljava/lang/Throwable;ZZ)V")
    }

    // java: getMessage()Ljava/lang/String;
    pub fn getMessage(&self) -> Result<String> {
        panic!("stub: java/lang/Throwable.getMessage:()Ljava/lang/String;")
    }

    // java: getLocalizedMessage()Ljava/lang/String;
    pub fn getLocalizedMessage(&self) -> Result<String> {
        panic!("stub: java/lang/Throwable.getLocalizedMessage:()Ljava/lang/String;")
    }

    // java: getCause()Ljava/lang/Throwable;
    pub fn getCause(&self) -> Result<Object> {
        panic!("stub: java/lang/Throwable.getCause:()Ljava/lang/Throwable;")
    }

    // java: initCause(Ljava/lang/Throwable;)Ljava/lang/Throwable;
    pub fn initCause(&self, cause: Object) -> Result<Object> {
        panic!("stub: java/lang/Throwable.initCause:(Ljava/lang/Throwable;)Ljava/lang/Throwable;")
    }

    // java: setCause(Ljava/lang/Throwable;)V
    pub fn setCause(&self, t: Object) -> Result<()> {
        panic!("stub: java/lang/Throwable.setCause:(Ljava/lang/Throwable;)V")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/lang/Throwable.toString:()Ljava/lang/String;")
    }

    // java: printStackTrace()V
    pub fn printStackTrace(&self) -> Result<()> {
        panic!("stub: java/lang/Throwable.printStackTrace:()V")
    }

    // java: printStackTrace(Ljava/io/PrintStream;)V
    pub fn printStackTrace__prints(&self, s: Object) -> Result<()> {
        panic!("stub: java/lang/Throwable.printStackTrace:(Ljava/io/PrintStream;)V")
    }

    // java: printStackTrace(Ljava/lang/Throwable$PrintStreamOrWriter;)V
    pub fn printStackTrace__throwa(&self, s: Object) -> Result<()> {
        panic!("stub: java/lang/Throwable.printStackTrace:(Ljava/lang/Throwable$PrintStreamOrWriter;)V")
    }

    // java: lockedPrintStackTrace(Ljava/lang/Throwable$PrintStreamOrWriter;)V
    pub fn lockedPrintStackTrace(&self, s: Object) -> Result<()> {
        panic!("stub: java/lang/Throwable.lockedPrintStackTrace:(Ljava/lang/Throwable$PrintStreamOrWriter;)V")
    }

    // java: printEnclosedStackTrace(Ljava/lang/Throwable$PrintStreamOrWriter;[Ljava/lang/StackTraceElement;Ljava/lang/String;Ljava/lang/String;Ljava/util/Set;)V
    pub fn printEnclosedStackTrace(&self, s: Object, enclosingTrace: Vec<Object>, caption: String, prefix: String, dejaVu: Object) -> Result<()> {
        panic!("stub: java/lang/Throwable.printEnclosedStackTrace:(Ljava/lang/Throwable$PrintStreamOrWriter;[Ljava/lang/StackTraceElement;Ljava/lang/String;Ljava/lang/String;Ljava/util/Set;)V")
    }

    // java: printStackTrace(Ljava/io/PrintWriter;)V
    pub fn printStackTrace__printw(&self, s: Object) -> Result<()> {
        panic!("stub: java/lang/Throwable.printStackTrace:(Ljava/io/PrintWriter;)V")
    }

    // java: fillInStackTrace()Ljava/lang/Throwable;
    pub fn fillInStackTrace(&self) -> Result<Object> {
        panic!("stub: java/lang/Throwable.fillInStackTrace:()Ljava/lang/Throwable;")
    }

    // java: fillInStackTrace(I)Ljava/lang/Throwable;
    pub fn fillInStackTrace__i(&self, arg0: i32) -> Result<Object> {
        panic!("native: java/lang/Throwable.fillInStackTrace:(I)Ljava/lang/Throwable;")
    }

    // java: getStackTrace()[Ljava/lang/StackTraceElement;
    pub fn getStackTrace(&self) -> Result<Vec<Object>> {
        panic!("stub: java/lang/Throwable.getStackTrace:()[Ljava/lang/StackTraceElement;")
    }

    // java: getOurStackTrace()[Ljava/lang/StackTraceElement;
    pub fn getOurStackTrace(&self) -> Result<Vec<Object>> {
        panic!("stub: java/lang/Throwable.getOurStackTrace:()[Ljava/lang/StackTraceElement;")
    }

    // java: setStackTrace([Ljava/lang/StackTraceElement;)V
    pub fn setStackTrace(&self, stackTrace: Vec<Object>) -> Result<()> {
        panic!("stub: java/lang/Throwable.setStackTrace:([Ljava/lang/StackTraceElement;)V")
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, s: Object) -> Result<()> {
        panic!("stub: java/lang/Throwable.readObject:(Ljava/io/ObjectInputStream;)V")
    }

    // java: validateSuppressedExceptionsList(Ljava/util/List;)I
    pub fn validateSuppressedExceptionsList(&self, deserSuppressedExceptions: Object) -> Result<i32> {
        panic!("stub: java/lang/Throwable.validateSuppressedExceptionsList:(Ljava/util/List;)I")
    }

    // java: writeObject(Ljava/io/ObjectOutputStream;)V
    pub fn writeObject(&self, s: Object) -> Result<()> {
        panic!("stub: java/lang/Throwable.writeObject:(Ljava/io/ObjectOutputStream;)V")
    }

    // java: addSuppressed(Ljava/lang/Throwable;)V
    pub fn addSuppressed(&self, exception: Object) -> Result<()> {
        panic!("stub: java/lang/Throwable.addSuppressed:(Ljava/lang/Throwable;)V")
    }

    // java: getSuppressed()[Ljava/lang/Throwable;
    pub fn getSuppressed(&self) -> Result<Vec<Object>> {
        panic!("stub: java/lang/Throwable.getSuppressed:()[Ljava/lang/Throwable;")
    }
}
