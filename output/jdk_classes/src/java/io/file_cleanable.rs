#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/FileCleanable",
    super_class = "jdk/internal/ref/PhantomCleanable",
    interfaces  = "",
    access      = "final",
    source      = "FileCleanable.java",
))]
pub struct FileCleanable {
    #[cfg_attr(any(), java_field(name = "fd", descriptor = "I", access = "private final"))]
    pub fd: Field<i32>,
    #[cfg_attr(any(), java_field(name = "handle", descriptor = "J", access = "private final"))]
    pub handle: Field<i64>,
}

impl FileCleanable {
    // java: cleanupClose0(IJ)V
    pub fn cleanupClose0(arg0: i32, arg1: i64) -> Result<()> {
        todo!("native java/io/FileCleanable.cleanupClose0")
    }

    // java: register(Ljava/io/FileDescriptor;)V
    pub fn register(fdo: Object) -> Result<()> {
        let _t0 = fdo.valid()?;
        let _t1 = FileCleanable::fdAccess().get(fdo)?;
        let mut fd: i32 = _t1;
        let _t2 = FileCleanable::fdAccess().getHandle(fdo)?;
        let mut handle: i64 = _t2;
        let _t3: Object = CleanerFactory::cleaner()?;
        fdo.registerCleanup(FileCleanable::new(fdo, _t3, fd, handle)?)?;
        Ok(())
    }

    // java: unregister(Ljava/io/FileDescriptor;)V
    pub fn unregister(fdo: Object) -> Result<()> {
        fdo.unregisterCleanup()?;
        Ok(())
    }

    // java: <init>(Ljava/io/FileDescriptor;Ljava/lang/ref/Cleaner;IJ)V
    pub fn new(obj: Object, cleaner: Object, fd: i32, handle: i64) -> Result<Self> {
        let this = Self { fd: Field::new(0), handle: Field::new(0) };
        /* invokespecial Method jdk/internal/ref/PhantomCleanable.<init>:(Ljava/lang/Object;Ljava/lang/ref/Cleaner;)V */
        this.fd.set(fd);
        this.handle.set(handle);
        Ok(this)
    }

    // java: performCleanup()V
    pub fn performCleanup(&self) -> Result<()> {
        let this = self;
        FileCleanable::cleanupClose0(this.fd.get(), this.handle.get())?;
        let mut ioe: i32 = todo!("stack underflow");
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }
}
