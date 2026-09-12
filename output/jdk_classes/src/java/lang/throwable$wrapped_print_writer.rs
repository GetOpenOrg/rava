#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Throwable$WrappedPrintWriter",
    super_class = "java/lang/Throwable$PrintStreamOrWriter",
    interfaces  = "",
    access      = "",
    source      = "Throwable.java",
))]
pub struct Throwable_WrappedPrintWriter {
    #[cfg_attr(any(), java_field(name = "printWriter", descriptor = "Ljava/io/PrintWriter;", access = "private final"))]
    pub printWriter: Field<Object>,
}

impl Throwable_WrappedPrintWriter {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/PrintWriter;)V"))]
    pub fn new(printWriter: Object) -> Result<Self> {
        let this = Self { printWriter: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Throwable$PrintStreamOrWriter.<init>:()V */
        this.printWriter.set(printWriter);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "lock", descriptor = "()Ljava/lang/Object;"))]
    pub fn lock(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = SharedSecrets::getJavaIOPrintWriterAccess()?;
        let _t1 = _t0.lock(this.printWriter.get())?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "(Ljava/lang/Object;)V"))]
    pub fn println(&self, o: Object) -> Result<()> {
        let this = self;
        this.printWriter.get().println(o)?;
        Ok(())
    }
}
