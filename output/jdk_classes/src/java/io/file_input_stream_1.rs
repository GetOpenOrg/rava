#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/FileInputStream$1",
    super_class = "java/lang/Object",
    interfaces  = "java/io/Closeable",
    access      = "",
    source      = "FileInputStream.java",
))]
pub struct FileInputStream_1 {
    #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/io/FileInputStream;", access = "final"))]
    pub this_0: Field<Object>,
}

impl FileInputStream_1 {
    // java: <init>(Ljava/io/FileInputStream;)V
    pub fn new(this_0: Object) -> Result<Self> {
        let this = Self { this_0: Field::new(Default::default()) };
        this.this_0.set(this_0);
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // java: close()V
    pub fn close(&self) -> Result<()> {
        let this = self;
        this.this_0.get().fd.get().close()?;
        Ok(())
    }
}
