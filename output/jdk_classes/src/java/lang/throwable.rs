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
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { backtrace: Field::new(Default::default()), detailMessage: Field::new(String::new()), cause: Field::new(Default::default()), stackTrace: Field::new(Default::default()), depth: Field::new(0), suppressedExceptions: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.cause.set(this);
        this.stackTrace.set(Throwable::UNASSIGNED_STACK());
        this.suppressedExceptions.set(Throwable::SUPPRESSED_SENTINEL());
        let _t0 = this.fillInStackTrace()?;
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(message: String) -> Result<Self> {
        let this = Self { backtrace: Field::new(Default::default()), detailMessage: Field::new(String::new()), cause: Field::new(Default::default()), stackTrace: Field::new(Default::default()), depth: Field::new(0), suppressedExceptions: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.cause.set(this);
        this.stackTrace.set(Throwable::UNASSIGNED_STACK());
        this.suppressedExceptions.set(Throwable::SUPPRESSED_SENTINEL());
        let _t0 = this.fillInStackTrace()?;
        this.detailMessage.set(message);
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;)V
    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;)V
    pub fn new__str_throwa(message: String, cause: Object) -> Result<Self> {
        let this = Self { backtrace: Field::new(Default::default()), detailMessage: Field::new(String::new()), cause: Field::new(Default::default()), stackTrace: Field::new(Default::default()), depth: Field::new(0), suppressedExceptions: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.cause.set(this);
        this.stackTrace.set(Throwable::UNASSIGNED_STACK());
        this.suppressedExceptions.set(Throwable::SUPPRESSED_SENTINEL());
        let _t0 = this.fillInStackTrace()?;
        this.detailMessage.set(message);
        this.cause.set(cause);
        Ok(this)
    }

    // java: <init>(Ljava/lang/Throwable;)V
    // java: <init>(Ljava/lang/Throwable;)V
    pub fn new__throwa(cause: Object) -> Result<Self> {
        let this = Self { backtrace: Field::new(Default::default()), detailMessage: Field::new(String::new()), cause: Field::new(Default::default()), stackTrace: Field::new(Default::default()), depth: Field::new(0), suppressedExceptions: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.cause.set(this);
        this.stackTrace.set(Throwable::UNASSIGNED_STACK());
        this.suppressedExceptions.set(Throwable::SUPPRESSED_SENTINEL());
        let _t0 = this.fillInStackTrace()?;
        /* TODO: aconst_null  */
        let _t1 = cause.toString()?;
        cause.detailMessage.set(_t1);
        this.cause.set(cause);
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;ZZ)V
    // java: <init>(Ljava/lang/String;Ljava/lang/Throwable;ZZ)V
    pub fn new__str_throwa_z_z(message: String, cause: Object, enableSuppression: bool, writableStackTrace: bool) -> Result<Self> {
        let this = Self { backtrace: Field::new(Default::default()), detailMessage: Field::new(String::new()), cause: Field::new(Default::default()), stackTrace: Field::new(Default::default()), depth: Field::new(0), suppressedExceptions: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.cause.set(this);
        this.stackTrace.set(Throwable::UNASSIGNED_STACK());
        this.suppressedExceptions.set(Throwable::SUPPRESSED_SENTINEL());
        let _t0 = this.fillInStackTrace()?;
        /* TODO: aconst_null  */
        writableStackTrace.stackTrace.set(this);
        this.detailMessage.set(message);
        this.cause.set(cause);
        /* TODO: aconst_null  */
        enableSuppression.suppressedExceptions.set(this);
        Ok(this)
    }

    // java: getMessage()Ljava/lang/String;
    pub fn getMessage(&self) -> Result<String> {
        let this = self;
        Ok(this.detailMessage.get())
    }

    // java: getLocalizedMessage()Ljava/lang/String;
    pub fn getLocalizedMessage(&self) -> Result<String> {
        let this = self;
        let _t0 = this.getMessage()?;
        Ok(_t0)
    }

    // java: getCause()Ljava/lang/Throwable;
    pub fn getCause(&self) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        Ok(this.cause.get())
    }

    // java: initCause(Ljava/lang/Throwable;)Ljava/lang/Throwable;
    pub fn initCause(&self, cause: Object) -> Result<Object> {
        let this = self;
        String::new().append(&String::from("Can't overwrite cause with"))?;
        let _t0: String = Objects::toString__obj_str(cause, String::from("a null"))?;
        String::new().append(&_t0)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        this.cause.set(cause);
        Ok(this)
    }

    // java: setCause(Ljava/lang/Throwable;)V
    pub fn setCause(&self, t: Object) -> Result<()> {
        let this = self;
        this.cause.set(t);
        Ok(())
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.getClass()?;
        let _t1 = _t0.getName()?;
        let mut s: String = _t1;
        let _t2 = this.getLocalizedMessage()?;
        let mut message: String = _t2;
        String::new().append(&s)?;
        String::new().append(&String::from(":"))?;
        String::new().append(&message)?;
        Ok(s)
    }

    // java: printStackTrace()V
    // java: printStackTrace()V
    pub fn printStackTrace(&self) -> Result<()> {
        let this = self;
        this.printStackTrace__prints(System::err())?;
        Ok(())
    }

    // java: printStackTrace(Ljava/io/PrintStream;)V
    // java: printStackTrace(Ljava/io/PrintStream;)V
    pub fn printStackTrace__prints(&self, s: Object) -> Result<()> {
        let this = self;
        this.printStackTrace__throwa(Throwable_WrappedPrintStream::new(s)?)?;
        Ok(())
    }

    // java: printStackTrace(Ljava/lang/Throwable$PrintStreamOrWriter;)V
    // java: printStackTrace(Ljava/lang/Throwable$PrintStreamOrWriter;)V
    pub fn printStackTrace__throwa(&self, s: Object) -> Result<()> {
        let this = self;
        let _t0 = s.lock()?;
        let mut lock: Object = _t0;
        let mut locker: Object = lock;
        locker.lock()?;
        this.lockedPrintStackTrace(s)?;
        locker.unlock()?;
        let mut local_4: bool = true;
        locker.unlock()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        local_4 = lock;
        /* TODO: monitorenter  */
        this.lockedPrintStackTrace(s)?;
        /* TODO: monitorexit  */
        let mut local_5: bool = local_4;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: lockedPrintStackTrace(Ljava/lang/Throwable$PrintStreamOrWriter;)V
    pub fn lockedPrintStackTrace(&self, s: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Collections::newSetFromMap(IdentityHashMap::new()?)?;
        let mut dejaVu: Object = _t0;
        let _t1 = dejaVu.add(this)?;
        s.println(this)?;
        let _t2 = this.getOurStackTrace()?;
        let mut trace: Vec<Object> = _t2;
        let mut ourCause: Vec<Object> = trace;
        let mut local_5: i32 = (ourCause.len() as i32);
        let mut local_6: i32 = 0i32;
        loop {
            if local_6 >= local_5 { break; }
            let mut traceElement: Object = ourCause[local_6 as usize].clone();
            String::new().append(&String::from("at"))?;
            String::new().append(&traceElement)?;
            s.println(String::new())?;
            local_6 = local_6.wrapping_add(1i32);
        }
        let _t3 = this.getSuppressed()?;
        ourCause = _t3;
        local_5 = (ourCause.len() as i32);
        local_6 = 0i32;
        loop {
            if local_6 >= local_5 { break; }
            traceElement = ourCause[local_6 as usize].clone();
            traceElement.printEnclosedStackTrace(s, trace, String::from("Suppressed:"), String::from(""), dejaVu)?;
            local_6 = local_6.wrapping_add(1i32);
        }
        let _t4 = this.getCause()?;
        ourCause = _t4;
        ourCause.printEnclosedStackTrace(s, trace, String::from("Caused by:"), String::from(""), dejaVu)?;
        Ok(())
    }

    // java: printEnclosedStackTrace(Ljava/lang/Throwable$PrintStreamOrWriter;[Ljava/lang/StackTraceElement;Ljava/lang/String;Ljava/lang/String;Ljava/util/Set;)V
    pub fn printEnclosedStackTrace(&self, s: Object, enclosingTrace: Vec<Object>, caption: String, prefix: String, dejaVu: Object) -> Result<()> {
        let this = self;
        let _t0 = s.isLockedByCurrentThread()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1 = dejaVu.contains(this)?;
        String::new().append(&prefix)?;
        String::new().append(&caption)?;
        String::new().append(&String::from("[CIRCULAR REFERENCE:"))?;
        String::new().append(&this)?;
        String::new().append(&String::from("]"))?;
        s.println(String::new())?;
        let _t2 = dejaVu.add(this)?;
        let _t3 = this.getOurStackTrace()?;
        let mut trace: Vec<Object> = _t3;
        let mut m: i32 = ((trace.len() as i32)).wrapping_sub(1i32);
        let mut n: i32 = ((enclosingTrace.len() as i32)).wrapping_sub(1i32);
        loop {
            if m<0i32 { break; }
            let _t0 = trace[m as usize].clone().equals(enclosingTrace[n as usize].clone())?;
            m = m.wrapping_sub(1i32);
            n = n.wrapping_sub(1i32);
        }
        let mut framesInCommon: i32 = (((trace.len() as i32)).wrapping_sub(1i32)).wrapping_sub(m);
        String::new().append(&prefix)?;
        String::new().append(&caption)?;
        String::new().append(&this)?;
        s.println(String::new())?;
        let mut i: i32 = 0i32;
        loop {
            if i > m { break; }
            String::new().append(&prefix)?;
            String::new().append(&String::from("at"))?;
            String::new().append(&trace[i as usize].clone())?;
            s.println(String::new())?;
            i = i.wrapping_add(1i32);
        }
        String::new().append(&prefix)?;
        String::new().append(&String::from("..."))?;
        String::new().append(&framesInCommon)?;
        String::new().append(&String::from("more"))?;
        s.println(String::new())?;
        let _t4 = this.getSuppressed()?;
        i = _t4;
        let mut local_11: i32 = (i.len() as i32);
        let mut local_12: i32 = 0i32;
        loop {
            if local_12 >= local_11 { break; }
            let mut se: Object = i[local_12 as usize].clone();
            String::new().append(&prefix)?;
            String::new().append(&String::from(""))?;
            se.printEnclosedStackTrace(s, trace, String::from("Suppressed:"), String::new(), dejaVu)?;
            local_12 = local_12.wrapping_add(1i32);
        }
        let _t5 = this.getCause()?;
        i = _t5;
        i.printEnclosedStackTrace(s, trace, String::from("Caused by:"), prefix, dejaVu)?;
        Ok(())
    }

    // java: printStackTrace(Ljava/io/PrintWriter;)V
    // java: printStackTrace(Ljava/io/PrintWriter;)V
    pub fn printStackTrace__printw(&self, s: Object) -> Result<()> {
        let this = self;
        this.printStackTrace__throwa(Throwable_WrappedPrintWriter::new(s)?)?;
        Ok(())
    }

    // java: fillInStackTrace()Ljava/lang/Throwable;
    // java: fillInStackTrace()Ljava/lang/Throwable;
    pub fn fillInStackTrace(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.fillInStackTrace__i(0i32)?;
        this.stackTrace.set(Throwable::UNASSIGNED_STACK());
        Ok(this)
    }

    // java: fillInStackTrace(I)Ljava/lang/Throwable;
    pub fn fillInStackTrace__i(&self, arg0: i32) -> Result<Object> {
        todo!("native java/lang/Throwable.fillInStackTrace")
    }

    // java: getStackTrace()[Ljava/lang/StackTraceElement;
    pub fn getStackTrace(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.getOurStackTrace()?;
        let _t1 = _t0.clone()?;
        Ok(_t1)
    }

    // java: getOurStackTrace()[Ljava/lang/StackTraceElement;
    pub fn getOurStackTrace(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Vec<Object> = StackTraceElement::of(this.backtrace.get(), this.depth.get())?;
        this.stackTrace.set(_t0);
        return Ok(Throwable::UNASSIGNED_STACK());
        Ok(this.stackTrace.get())
    }

    // java: setStackTrace([Ljava/lang/StackTraceElement;)V
    pub fn setStackTrace(&self, stackTrace: Vec<Object>) -> Result<()> {
        let this = self;
        let _t0 = stackTrace.clone()?;
        let mut defensiveCopy: Object = _t0;
        let mut i: i32 = 0i32;
        loop {
            if i >= (defensiveCopy.len() as i32) { break; }
            String::new().append(&String::from("stackTrace["))?;
            String::new().append(&i)?;
            String::new().append(&String::from("]"))?;
            return Err(JvmError::Custom("athrow".to_owned()));
            i = i.wrapping_add(1i32);
        }
        i = this;
        /* TODO: monitorenter  */
        /* TODO: monitorexit  */
        return Ok(());
        this.stackTrace.set(defensiveCopy);
        /* TODO: monitorexit  */
        let mut local_4: i32 = i;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, s: Object) -> Result<()> {
        let this = self;
        s.defaultReadObject()?;
        let mut candidateSuppressedExceptions: Object = this.suppressedExceptions.get();
        this.suppressedExceptions.set(Throwable::SUPPRESSED_SENTINEL());
        let mut candidateStackTrace: Vec<Object> = this.stackTrace.get();
        let _t0 = Throwable::UNASSIGNED_STACK().clone()?;
        this.stackTrace.set(_t0);
        let _t1 = this.validateSuppressedExceptionsList(candidateSuppressedExceptions)?;
        let mut suppressedSize: i32 = _t1;
        let _t2: i32 = (100i32).min(suppressedSize);
        let mut suppList: ArrayList<_> = ArrayList::<_>::new()?;
        let _t3 = candidateSuppressedExceptions.iterator()?;
        let mut local_6: Object = _t3;
        loop {
            let _t0 = local_6.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_6.next()?;
            let mut t: Object = _t0;
            let _t1: Object = Objects::requireNonNull__obj_str(t, String::from("Cannot suppress a null exception."))?;
            return Err(JvmError::Custom("athrow".to_owned()));
            let _t2 = suppList.add(t)?;
        }
        this.suppressedExceptions.set(suppList);
        /* TODO: aconst_null  */
        suppressedSize.suppressedExceptions.set(this);
        let _t4 = candidateStackTrace.clone()?;
        candidateStackTrace = _t4;
        let _t5 = Throwable_SentinelHolder::STACK_TRACE_ELEMENT_SENTINEL().equals(candidateStackTrace[0i32 as usize].clone())?;
        /* TODO: aconst_null  */
        _t5.stackTrace.set(this);
        suppressedSize = candidateStackTrace;
        suppList = (suppressedSize.len() as i32);
        local_6 = 0i32;
        loop {
            if local_6 >= suppList { break; }
            t = suppressedSize[local_6 as usize].clone();
            let _t0: Object = Objects::requireNonNull__obj_str(t, String::from("null StackTraceElement in serial stream."))?;
            local_6 = local_6.wrapping_add(1i32);
        }
        this.stackTrace.set(candidateStackTrace);
        Ok(())
    }

    // java: validateSuppressedExceptionsList(Ljava/util/List;)I
    pub fn validateSuppressedExceptionsList(&self, deserSuppressedExceptions: Object) -> Result<i32> {
        let this = self;
        let _t0 = 2i32.getModule()?;
        let _t1 = deserSuppressedExceptions.getClass()?;
        let _t2 = _t1.getModule()?;
        let _t3 = _t0.equals(_t2)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t4 = deserSuppressedExceptions.size()?;
        let mut size: i32 = _t4;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(size)
    }

    // java: writeObject(Ljava/io/ObjectOutputStream;)V
    pub fn writeObject(&self, s: Object) -> Result<()> {
        let this = self;
        let _t0 = this.getOurStackTrace()?;
        let mut oldStackTrace: Vec<Object> = this.stackTrace.get();
        this.stackTrace.set(Throwable_SentinelHolder::STACK_TRACE_SENTINEL());
        s.defaultWriteObject()?;
        this.stackTrace.set(oldStackTrace);
        let mut local_3: Vec<Object> = this.stackTrace.get();
        this.stackTrace.set(oldStackTrace);
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: addSuppressed(Ljava/lang/Throwable;)V
    pub fn addSuppressed(&self, exception: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: Object = Objects::requireNonNull__obj_str(exception, String::from("Cannot suppress a null exception."))?;
        return Ok(());
        this.suppressedExceptions.set(ArrayList::<_>::new()?);
        let _t1 = this.suppressedExceptions.get().add(exception)?;
        Ok(())
    }

    // java: getSuppressed()[Ljava/lang/Throwable;
    pub fn getSuppressed(&self) -> Result<Vec<Object>> {
        let this = self;
        return Ok(Throwable::EMPTY_THROWABLE_ARRAY());
        let _t0 = this.suppressedExceptions.get().toArray(Throwable::EMPTY_THROWABLE_ARRAY())?;
        Ok(_t0)
    }
}
