#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;
use crate::jdk::internal::misc::InternalLock;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/Throwable",
    super_class       = "java/lang/Object",
    interfaces        = "java/io/Serializable",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Throwable.java",
    inner_classes     = "java/lang/Throwable$WrappedPrintStream:java/lang/Throwable:WrappedPrintStream:10;java/lang/Throwable$PrintStreamOrWriter:java/lang/Throwable:PrintStreamOrWriter:1034;java/lang/Throwable$WrappedPrintWriter:java/lang/Throwable:WrappedPrintWriter:10;java/lang/Throwable$SentinelHolder:java/lang/Throwable:SentinelHolder:10",
    all_supertypes    = "java/io/Serializable;java/lang/Object;java/lang/Throwable",
    has_to_string_method = true,
)]
#[derive(Clone, Default, PartialEq)]
pub struct Throwable {
    #[cfg_attr(any(), java_field(name = "backtrace", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "transient", is_static = false))]
    pub backtrace: JField<Object>,
    #[cfg_attr(any(), java_field(name = "detailMessage", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
    pub detailMessage: JField<String>,
    #[cfg_attr(any(), java_field(name = "cause", descriptor = "Ljava/lang/Throwable;", access = "private", modifiers = "", is_static = false))]
    pub cause: JField<Throwable>,
    #[cfg_attr(any(), java_field(name = "stackTrace", descriptor = "[Ljava/lang/StackTraceElement;", access = "private", modifiers = "", is_static = false))]
    pub stackTrace: JField<Rc<RefCell<Vec<Object>>>>,
    #[cfg_attr(any(), java_field(name = "depth", descriptor = "I", access = "private", modifiers = "transient", is_static = false))]
    pub depth: JField<i32>,
    #[cfg_attr(any(), java_field(name = "suppressedExceptions", descriptor = "Ljava/util/List;", access = "private", modifiers = "", is_static = false, generic_signature = "Ljava/util/List<Ljava/lang/Throwable;>;"))]
    pub suppressedExceptions: JField<List<Throwable>>,
}

impl Throwable {
    #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-3042686055658047285"))]
    // static field: serialVersionUID:J
    pub fn serialVersionUID() -> i64 {
        -3042686055658047285i64
    }

    #[cfg_attr(any(), java_field(name = "UNASSIGNED_STACK", descriptor = "[Ljava/lang/StackTraceElement;", access = "private", modifiers = "static final", is_static = true))]
    // static field: UNASSIGNED_STACK:[Ljava/lang/StackTraceElement;
    pub fn UNASSIGNED_STACK() -> Rc<RefCell<Vec<Object>>> {
        Rc::new(RefCell::new(Vec::new()))
    }

    #[cfg_attr(any(), java_field(name = "SUPPRESSED_SENTINEL", descriptor = "Ljava/util/List;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/List<Ljava/lang/Throwable;>;"))]
    // static field: SUPPRESSED_SENTINEL:Ljava/util/List;
    pub fn SUPPRESSED_SENTINEL() -> List<Throwable> {
        panic!("stub: java/lang/Throwable.SUPPRESSED_SENTINEL:Ljava/util/List;")
    }

    #[cfg_attr(any(), java_field(name = "NULL_CAUSE_MESSAGE", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "Cannot suppress a null exception."))]
    // static field: NULL_CAUSE_MESSAGE:Ljava/lang/String;
    pub fn NULL_CAUSE_MESSAGE() -> String {
        String::from("Cannot suppress a null exception.")
    }

    #[cfg_attr(any(), java_field(name = "SELF_SUPPRESSION_MESSAGE", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "Self-suppression not permitted"))]
    // static field: SELF_SUPPRESSION_MESSAGE:Ljava/lang/String;
    pub fn SELF_SUPPRESSION_MESSAGE() -> String {
        String::from("Self-suppression not permitted")
    }

    #[cfg_attr(any(), java_field(name = "CAUSE_CAPTION", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "Caused by: "))]
    // static field: CAUSE_CAPTION:Ljava/lang/String;
    pub fn CAUSE_CAPTION() -> String {
        String::from("Caused by: ")
    }

    #[cfg_attr(any(), java_field(name = "SUPPRESSED_CAPTION", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "Suppressed: "))]
    // static field: SUPPRESSED_CAPTION:Ljava/lang/String;
    pub fn SUPPRESSED_CAPTION() -> String {
        String::from("Suppressed: ")
    }

    #[cfg_attr(any(), java_field(name = "EMPTY_THROWABLE_ARRAY", descriptor = "[Ljava/lang/Throwable;", access = "private", modifiers = "static final", is_static = true))]
    // static field: EMPTY_THROWABLE_ARRAY:[Ljava/lang/Throwable;
    pub fn EMPTY_THROWABLE_ARRAY() -> Rc<RefCell<Vec<Throwable>>> {
        Rc::new(RefCell::new(Vec::new()))
    }

    #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
    // static field: $assertionsDisabled:Z
    pub fn _assertionsDisabled() -> bool {
        false
    }

    #[java_rta_macros::java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let mut this = Self { backtrace: JField::new(Default::default()), detailMessage: JField::new(String::default()), cause: JField::new(Default::default()), stackTrace: JField::new(Default::default()), depth: JField::new(0), suppressedExceptions: JField::new(Default::default()), ..Default::default() };
        /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
        this.cause.set(Clone::clone(&this));
        this.stackTrace.set(Clone::clone(&Throwable::UNASSIGNED_STACK()));
        this.suppressedExceptions.set(Clone::clone(&Throwable::SUPPRESSED_SENTINEL()));
        let _t0 = this.fillInStackTrace()?;
        Ok(this)
    }

    #[java_rta_macros::java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    // java: <init>(Ljava/lang/String;)V
    pub fn new_str(mut message: String) -> Result<Self> {
        let mut this = Self { backtrace: JField::new(Default::default()), detailMessage: JField::new(String::default()), cause: JField::new(Default::default()), stackTrace: JField::new(Default::default()), depth: JField::new(0), suppressedExceptions: JField::new(Default::default()), ..Default::default() };
        /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
        this.cause.set(Clone::clone(&this));
        this.stackTrace.set(Clone::clone(&Throwable::UNASSIGNED_STACK()));
        this.suppressedExceptions.set(Clone::clone(&Throwable::SUPPRESSED_SENTINEL()));
        let _t0 = this.fillInStackTrace()?;
        this.detailMessage.set(Clone::clone(&message));
        Ok(this)
    }

    #[java_rta_macros::java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/Throwable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn new_str_throwa(message: String, cause: Throwable) -> Result<Self> {
        panic!("stub: java/lang/Throwable.<init>:(Ljava/lang/String;Ljava/lang/Throwable;)V")
    }

    #[java_rta_macros::java_method(name = "<init>", descriptor = "(Ljava/lang/Throwable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn new_throwa(cause: Throwable) -> Result<Self> {
        panic!("stub: java/lang/Throwable.<init>:(Ljava/lang/Throwable;)V")
    }

    #[java_rta_macros::java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/Throwable;ZZ)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn new_str_throwa_z_z(message: String, cause: Throwable, enableSuppression: bool, writableStackTrace: bool) -> Result<Self> {
        panic!("stub: java/lang/Throwable.<init>:(Ljava/lang/String;Ljava/lang/Throwable;ZZ)V")
    }

    #[java_rta_macros::java_method(name = "getMessage", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getMessage(&self) -> Result<String> {
        panic!("stub: java/lang/Throwable.getMessage:()Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "getLocalizedMessage", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getLocalizedMessage(&self) -> Result<String> {
        panic!("stub: java/lang/Throwable.getLocalizedMessage:()Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "getCause", descriptor = "()Ljava/lang/Throwable;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getCause(&self) -> Result<Throwable> {
        panic!("stub: java/lang/Throwable.getCause:()Ljava/lang/Throwable;")
    }

    #[java_rta_macros::java_method(name = "initCause", descriptor = "(Ljava/lang/Throwable;)Ljava/lang/Throwable;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn initCause(&self, cause: Throwable) -> Result<Throwable> {
        panic!("stub: java/lang/Throwable.initCause:(Ljava/lang/Throwable;)Ljava/lang/Throwable;")
    }

    #[java_rta_macros::java_method(name = "setCause", descriptor = "(Ljava/lang/Throwable;)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn setCause(&self, t: Throwable) -> Result<()> {
        panic!("stub: java/lang/Throwable.setCause:(Ljava/lang/Throwable;)V")
    }

    #[java_rta_macros::java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn toString(&self) -> Result<String> {
        Ok(String::from(Self::BINARY_NAME))
    }

    #[java_rta_macros::java_method(name = "printStackTrace", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn printStackTrace(&self) -> Result<()> {
        panic!("stub: java/lang/Throwable.printStackTrace:()V")
    }

    #[java_rta_macros::java_method(name = "printStackTrace", descriptor = "(Ljava/io/PrintStream;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn printStackTrace_prints(&self, s: PrintStream) -> Result<()> {
        panic!("stub: java/lang/Throwable.printStackTrace:(Ljava/io/PrintStream;)V")
    }

    #[java_rta_macros::java_method(name = "printStackTrace", descriptor = "(Ljava/lang/Throwable$PrintStreamOrWriter;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn printStackTrace_throwa(&self, s: Object) -> Result<()> {
        panic!("stub: java/lang/Throwable.printStackTrace:(Ljava/lang/Throwable$PrintStreamOrWriter;)V")
    }

    #[java_rta_macros::java_method(name = "lockedPrintStackTrace", descriptor = "(Ljava/lang/Throwable$PrintStreamOrWriter;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn lockedPrintStackTrace(&self, s: Object) -> Result<()> {
        panic!("stub: java/lang/Throwable.lockedPrintStackTrace:(Ljava/lang/Throwable$PrintStreamOrWriter;)V")
    }

    #[java_rta_macros::java_method(name = "printEnclosedStackTrace", descriptor = "(Ljava/lang/Throwable$PrintStreamOrWriter;[Ljava/lang/StackTraceElement;Ljava/lang/String;Ljava/lang/String;Ljava/util/Set;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Throwable$PrintStreamOrWriter;[Ljava/lang/StackTraceElement;Ljava/lang/String;Ljava/lang/String;Ljava/util/Set<Ljava/lang/Throwable;>;)V")]
    pub fn printEnclosedStackTrace(&self, s: Object, enclosingTrace: Rc<RefCell<Vec<Object>>>, caption: String, prefix: String, dejaVu: Object) -> Result<()> {
        panic!("stub: java/lang/Throwable.printEnclosedStackTrace:(Ljava/lang/Throwable$PrintStreamOrWriter;[Ljava/lang/StackTraceElement;Ljava/lang/String;Ljava/lang/String;Ljava/util/Set;)V")
    }

    #[java_rta_macros::java_method(name = "printStackTrace", descriptor = "(Ljava/io/PrintWriter;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn printStackTrace_printw(&self, s: Object) -> Result<()> {
        panic!("stub: java/lang/Throwable.printStackTrace:(Ljava/io/PrintWriter;)V")
    }

    #[cfg_attr(any(), java_native(name = "fillInStackTrace", descriptor = "(I)Ljava/lang/Throwable;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn fillInStackTrace_i(&self, arg0: i32) -> Result<Throwable> {
        panic!("native: java/lang/Throwable.fillInStackTrace:(I)Ljava/lang/Throwable;")
    }

    #[java_rta_macros::java_method(name = "getStackTrace", descriptor = "()[Ljava/lang/StackTraceElement;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getStackTrace(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Throwable.getStackTrace:()[Ljava/lang/StackTraceElement;")
    }

    #[java_rta_macros::java_method(name = "getOurStackTrace", descriptor = "()[Ljava/lang/StackTraceElement;", access = "private", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getOurStackTrace(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Throwable.getOurStackTrace:()[Ljava/lang/StackTraceElement;")
    }

    #[java_rta_macros::java_method(name = "setStackTrace", descriptor = "([Ljava/lang/StackTraceElement;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn setStackTrace(&self, stackTrace: Rc<RefCell<Vec<Object>>>) -> Result<()> {
        panic!("stub: java/lang/Throwable.setStackTrace:([Ljava/lang/StackTraceElement;)V")
    }

    #[java_rta_macros::java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
    pub fn readObject(&self, s: Object) -> Result<()> {
        panic!("stub: java/lang/Throwable.readObject:(Ljava/io/ObjectInputStream;)V")
    }

    #[java_rta_macros::java_method(name = "validateSuppressedExceptionsList", descriptor = "(Ljava/util/List;)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException", generic_signature = "(Ljava/util/List<Ljava/lang/Throwable;>;)I")]
    pub fn validateSuppressedExceptionsList(&self, deserSuppressedExceptions: Object) -> Result<i32> {
        panic!("stub: java/lang/Throwable.validateSuppressedExceptionsList:(Ljava/util/List;)I")
    }

    #[java_rta_macros::java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
    pub fn writeObject(&self, s: Object) -> Result<()> {
        panic!("stub: java/lang/Throwable.writeObject:(Ljava/io/ObjectOutputStream;)V")
    }

    #[java_rta_macros::java_method(name = "addSuppressed", descriptor = "(Ljava/lang/Throwable;)V", access = "public", modifiers = "final synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn addSuppressed(&self, exception: Throwable) -> Result<()> {
        panic!("stub: java/lang/Throwable.addSuppressed:(Ljava/lang/Throwable;)V")
    }

    #[java_rta_macros::java_method(name = "getSuppressed", descriptor = "()[Ljava/lang/Throwable;", access = "public", modifiers = "final synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getSuppressed(&self) -> Result<Rc<RefCell<Vec<Throwable>>>> {
        panic!("stub: java/lang/Throwable.getSuppressed:()[Ljava/lang/Throwable;")
    }
}
