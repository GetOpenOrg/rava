#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Throwable$WrappedPrintStream",
    super_class = "java/lang/Throwable$PrintStreamOrWriter",
    interfaces  = "",
    access      = "",
    source      = "Throwable.java",
))]
pub struct Throwable_WrappedPrintStream {
    #[cfg_attr(any(), java_field(name = "printStream", descriptor = "Ljava/io/PrintStream;", access = "private final"))]
    pub printStream: Field<Object>,
}

impl Throwable_WrappedPrintStream {
    // java: <init>(Ljava/io/PrintStream;)V
    pub fn new(printStream: Object) -> Result<Self> {
        let this = Self { printStream: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Throwable$PrintStreamOrWriter.<init>:()V */
        this.printStream.set(printStream);
        Ok(this)
    }

    // java: lock()Ljava/lang/Object;
    pub fn lock(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = SharedSecrets::getJavaIOPrintStreamAccess()?;
        let _t1 = _t0.lock(this.printStream.get())?;
        Ok(_t1)
    }

    // java: println(Ljava/lang/Object;)V
    pub fn println(&self, o: Object) -> Result<()> {
        let this = self;
        this.printStream.get().println(o)?;
        Ok(())
    }
}
