#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::math::*;
use crate::java::nio::*;
use crate::java::nio::charset::*;
use crate::java::security::*;
use crate::java::text::*;
use crate::java::text::spi::*;
use crate::java::time::*;
use crate::java::time::chrono::*;
use crate::java::time::temporal::*;
use crate::java::time::zone::*;
use crate::java::util::*;
use crate::java::util::concurrent::*;
use crate::java::util::concurrent::atomic::*;
use crate::java::util::concurrent::locks::*;
use crate::java::util::function::*;
use crate::java::util::regex::*;
use crate::java::util::spi::*;
use crate::java::util::stream::*;
use crate::java::util::zip::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::reflect::misc::*;
use crate::sun::security::action::*;
use crate::sun::security::util::*;
use crate::sun::text::*;
use crate::sun::util::*;
use crate::sun::util::calendar::*;
use crate::sun::util::locale::*;
use crate::sun::util::locale::provider::*;
use crate::sun::util::spi::*;
use crate::java::text::Normalizer;
use crate::jdk::internal::misc::InternalLock;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/Throwable"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Throwable.java"]
    #[inner_classes     = "java/lang/Throwable$WrappedPrintStream:java/lang/Throwable:WrappedPrintStream:10;java/lang/Throwable$PrintStreamOrWriter:java/lang/Throwable:PrintStreamOrWriter:1034;java/lang/Throwable$WrappedPrintWriter:java/lang/Throwable:WrappedPrintWriter:10;java/lang/Throwable$SentinelHolder:java/lang/Throwable:SentinelHolder:10"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/lang/Throwable"]
    #[has_to_string_method = true]

    pub struct Throwable {
        #[cfg_attr(any(), java_field(name = "backtrace", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "transient", is_static = false))]
        pub backtrace: Object,
        #[cfg_attr(any(), java_field(name = "detailMessage", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub detailMessage: String,
        #[cfg_attr(any(), java_field(name = "cause", descriptor = "Ljava/lang/Throwable;", access = "private", modifiers = "", is_static = false))]
        pub cause: Throwable,
        #[cfg_attr(any(), java_field(name = "stackTrace", descriptor = "[Ljava/lang/StackTraceElement;", access = "private", modifiers = "", is_static = false))]
        pub stackTrace: Rc<RefCell<Vec<Object>>>,
        #[cfg_attr(any(), java_field(name = "depth", descriptor = "I", access = "private", modifiers = "transient", is_static = false))]
        pub depth: i32,
        #[cfg_attr(any(), java_field(name = "suppressedExceptions", descriptor = "Ljava/util/List;", access = "private", modifiers = "", is_static = false, generic_signature = "Ljava/util/List<Ljava/lang/Throwable;>;"))]
        pub suppressedExceptions: Object,
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
        pub fn SUPPRESSED_SENTINEL() -> Object {
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

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_cause(Clone::clone(&this));
            this.__set_stackTrace(Clone::clone(&Throwable::UNASSIGNED_STACK()));
            this.__set_suppressedExceptions(Clone::clone(&Throwable::SUPPRESSED_SENTINEL()));
            let _t0 = this.fillInStackTrace()?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/String;)V
        pub fn new_str(mut message: String) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_cause(Clone::clone(&this));
            this.__set_stackTrace(Clone::clone(&Throwable::UNASSIGNED_STACK()));
            this.__set_suppressedExceptions(Clone::clone(&Throwable::SUPPRESSED_SENTINEL()));
            let _t0 = this.fillInStackTrace()?;
            this.__set_detailMessage(Clone::clone(&message));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/Throwable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;)V
        pub fn new_str_throwa(mut message: String, mut cause: Throwable) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_cause(Clone::clone(&this));
            this.__set_stackTrace(Clone::clone(&Throwable::UNASSIGNED_STACK()));
            this.__set_suppressedExceptions(Clone::clone(&Throwable::SUPPRESSED_SENTINEL()));
            let _t0 = this.fillInStackTrace()?;
            this.__set_detailMessage(Clone::clone(&message));
            this.__set_cause(Clone::clone(&cause));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Throwable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/Throwable;)V
        pub fn new_throwa(mut cause: Throwable) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_cause(Clone::clone(&this));
            this.__set_stackTrace(Clone::clone(&Throwable::UNASSIGNED_STACK()));
            this.__set_suppressedExceptions(Clone::clone(&Throwable::SUPPRESSED_SENTINEL()));
            let _t0 = this.fillInStackTrace()?;
            let mut _merged2: Object;
            if _is_jnull(&cause) {
                _merged2 = Object::default();
            } else {
                let _t1 = cause.toString()?;
                _merged2 = Object::from_any(Clone::clone(&_t1));
            }
            this.__set_detailMessage(Clone::clone(&_merged2));
            this.__set_cause(Clone::clone(&cause));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/Throwable;ZZ)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str_throwa_z_z(message: String, cause: Throwable, enableSuppression: bool, writableStackTrace: bool) -> Result<Self> {
            panic!("stub: java/lang/Throwable.<init>:(Ljava/lang/String;Ljava/lang/Throwable;ZZ)V")
        }

        #[java_method(name = "getMessage", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMessage(&self) -> Result<String> {
            let this = self;
            Ok(this.__get_detailMessage())
        }

        #[java_method(name = "getLocalizedMessage", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLocalizedMessage(&self) -> Result<String> {
            let this = self;
            let _t0 = this.getMessage()?;
            Ok(_t0)
        }

        #[java_method(name = "getCause", descriptor = "()Ljava/lang/Throwable;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCause(&self) -> Result<Throwable> {
            panic!("stub: java/lang/Throwable.getCause:()Ljava/lang/Throwable;")
        }

        #[java_method(name = "initCause", descriptor = "(Ljava/lang/Throwable;)Ljava/lang/Throwable;", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn initCause(&self, cause: Throwable) -> Result<Throwable> {
            panic!("stub: java/lang/Throwable.initCause:(Ljava/lang/Throwable;)Ljava/lang/Throwable;")
        }

        #[java_method(name = "setCause", descriptor = "(Ljava/lang/Throwable;)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setCause(&self, t: Throwable) -> Result<()> {
            panic!("stub: java/lang/Throwable.setCause:(Ljava/lang/Throwable;)V")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            let this = self;
            let _t0 = this.getClass()?;
            let _vdispatch1: String = if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
            let mut s: String = _vdispatch1;
            let _t2 = this.getLocalizedMessage()?;
            let mut message: String = _t2;
            let mut _merged7: String;
            if !_is_jnull(&message) {
                let _t3 = StringBuilder::new()?.append_str(Clone::clone(&s))?;
                let _t4 = _t3.append_str(Clone::clone(&String::from(": ")))?;
                let _t5 = _t4.append_str(Clone::clone(&message))?;
                let _t6 = _t5.toString()?;
                _merged7 = _t6;
            } else {
                _merged7 = s;
            }
            Ok(_merged7)
        }

        #[java_method(name = "printStackTrace", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn printStackTrace(&self) -> Result<()> {
            panic!("stub: java/lang/Throwable.printStackTrace:()V")
        }

        #[java_method(name = "printStackTrace", descriptor = "(Ljava/io/PrintStream;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn printStackTrace_prints(&self, s: PrintStream) -> Result<()> {
            panic!("stub: java/lang/Throwable.printStackTrace:(Ljava/io/PrintStream;)V")
        }

        #[java_method(name = "printStackTrace", descriptor = "(Ljava/lang/Throwable$PrintStreamOrWriter;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn printStackTrace_throwa(&self, s: Object) -> Result<()> {
            panic!("stub: java/lang/Throwable.printStackTrace:(Ljava/lang/Throwable$PrintStreamOrWriter;)V")
        }

        #[java_method(name = "lockedPrintStackTrace", descriptor = "(Ljava/lang/Throwable$PrintStreamOrWriter;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lockedPrintStackTrace(&self, s: Object) -> Result<()> {
            panic!("stub: java/lang/Throwable.lockedPrintStackTrace:(Ljava/lang/Throwable$PrintStreamOrWriter;)V")
        }

        #[java_method(name = "printEnclosedStackTrace", descriptor = "(Ljava/lang/Throwable$PrintStreamOrWriter;[Ljava/lang/StackTraceElement;Ljava/lang/String;Ljava/lang/String;Ljava/util/Set;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Throwable$PrintStreamOrWriter;[Ljava/lang/StackTraceElement;Ljava/lang/String;Ljava/lang/String;Ljava/util/Set<Ljava/lang/Throwable;>;)V")]
        pub fn printEnclosedStackTrace(&self, s: Object, enclosingTrace: Rc<RefCell<Vec<Object>>>, caption: String, prefix: String, dejaVu: Object) -> Result<()> {
            panic!("stub: java/lang/Throwable.printEnclosedStackTrace:(Ljava/lang/Throwable$PrintStreamOrWriter;[Ljava/lang/StackTraceElement;Ljava/lang/String;Ljava/lang/String;Ljava/util/Set;)V")
        }

        #[java_method(name = "printStackTrace", descriptor = "(Ljava/io/PrintWriter;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn printStackTrace_printw(&self, s: Object) -> Result<()> {
            panic!("stub: java/lang/Throwable.printStackTrace:(Ljava/io/PrintWriter;)V")
        }

        #[native]
        #[java_native(name = "fillInStackTrace", descriptor = "(I)Ljava/lang/Throwable;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn fillInStackTrace_i(&self, arg0: i32) -> Result<Throwable> {
            panic!("native: java/lang/Throwable.fillInStackTrace:(I)Ljava/lang/Throwable;")
        }

        #[java_method(name = "getStackTrace", descriptor = "()[Ljava/lang/StackTraceElement;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getStackTrace(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Throwable.getStackTrace:()[Ljava/lang/StackTraceElement;")
        }

        #[java_method(name = "getOurStackTrace", descriptor = "()[Ljava/lang/StackTraceElement;", access = "private", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOurStackTrace(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Throwable.getOurStackTrace:()[Ljava/lang/StackTraceElement;")
        }

        #[java_method(name = "setStackTrace", descriptor = "([Ljava/lang/StackTraceElement;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setStackTrace(&self, stackTrace: Rc<RefCell<Vec<Object>>>) -> Result<()> {
            panic!("stub: java/lang/Throwable.setStackTrace:([Ljava/lang/StackTraceElement;)V")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/lang/Throwable.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "validateSuppressedExceptionsList", descriptor = "(Ljava/util/List;)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException", generic_signature = "(Ljava/util/List<Ljava/lang/Throwable;>;)I")]
        pub fn validateSuppressedExceptionsList(&self, deserSuppressedExceptions: Object) -> Result<i32> {
            panic!("stub: java/lang/Throwable.validateSuppressedExceptionsList:(Ljava/util/List;)I")
        }

        #[java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/lang/Throwable.writeObject:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "addSuppressed", descriptor = "(Ljava/lang/Throwable;)V", access = "public", modifiers = "final synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addSuppressed(&self, mut exception: Throwable) -> Result<()> {
            let this = self;
            if Object::from_any(exception.clone()) == Object::from_any(this.clone()) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0: Object = Objects::requireNonNull_obj_str(Object::from_any(exception.clone()), Clone::clone(&String::from("Cannot suppress a null exception.")))?;
            if _is_jnull(&this.__get_suppressedExceptions()) {
                return Ok(());
            }
            if this.__get_suppressedExceptions() == Throwable::SUPPRESSED_SENTINEL() {
                this.__set_suppressedExceptions(Object::from_any(ArrayList::<Object>::new_i(1i32)?.clone()));
            }
            let _vdispatch1: bool = if let Some(_d) = this.__get_suppressedExceptions().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.add(Object::from_any(exception.clone()))? } else if let Some(_d) = this.__get_suppressedExceptions().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.add(Object::from_any(exception.clone()))? } else if let Some(_d) = this.__get_suppressedExceptions().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.add(Object::from_any(exception.clone()))? } else if let Some(_d) = this.__get_suppressedExceptions().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.add(Object::from_any(exception.clone()))? } else if let Some(_d) = this.__get_suppressedExceptions().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.add(Object::from_any(exception.clone()))? } else if let Some(_d) = this.__get_suppressedExceptions().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.add(Object::from_any(exception.clone()))? } else if let Some(_d) = this.__get_suppressedExceptions().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.add(Object::from_any(exception.clone()))? } else if let Some(_d) = this.__get_suppressedExceptions().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.add(Object::from_any(exception.clone()))? } else if let Some(_d) = this.__get_suppressedExceptions().0.as_any().downcast_ref::<Object>() { _d.add(Object::from_any(exception.clone()))? } else if let Some(__f) = this.__get_suppressedExceptions().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Object::from_any(exception.clone()))? } else { Default::default() };
            Ok(())
        }

        #[java_method(name = "getSuppressed", descriptor = "()[Ljava/lang/Throwable;", access = "public", modifiers = "final synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSuppressed(&self) -> Result<Rc<RefCell<Vec<Throwable>>>> {
            panic!("stub: java/lang/Throwable.getSuppressed:()[Ljava/lang/Throwable;")
        }
    }
}
