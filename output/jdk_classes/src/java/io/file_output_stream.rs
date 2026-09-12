#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/FileOutputStream",
    super_class = "java/io/OutputStream",
    interfaces  = "",
    access      = "public",
    source      = "FileOutputStream.java",
))]
pub struct FileOutputStream {
    #[cfg_attr(any(), java_field(name = "fd", descriptor = "Ljava/io/FileDescriptor;", access = "private final"))]
    pub fd: Field<Object>,
    #[cfg_attr(any(), java_field(name = "channel", descriptor = "Ljava/nio/channels/FileChannel;", access = "private"))]
    pub channel: Field<Object>,
    #[cfg_attr(any(), java_field(name = "path", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub path: Field<String>,
    #[cfg_attr(any(), java_field(name = "closeLock", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub closeLock: Field<Object>,
    #[cfg_attr(any(), java_field(name = "closed", descriptor = "Z", access = "private"))]
    pub closed: Field<bool>,
}

impl FileOutputStream {
    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(name: String) -> Result<Self> {
        let this = Self { fd: Field::new(Default::default()), channel: Field::new(Default::default()), path: Field::new(String::new()), closeLock: Field::new(Default::default()), closed: Field::new(false) };
        /* TODO: aconst_null  */
        /* invokespecial Method java/io/FileOutputStream.<init>:(Ljava/io/File;Z)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;Z)V
    // java: <init>(Ljava/lang/String;Z)V
    pub fn new__str_z(name: String, append: bool) -> Result<Self> {
        let this = Self { fd: Field::new(Default::default()), channel: Field::new(Default::default()), path: Field::new(String::new()), closeLock: Field::new(Default::default()), closed: Field::new(false) };
        /* TODO: aconst_null  */
        /* invokespecial Method java/io/FileOutputStream.<init>:(Ljava/io/File;Z)V */
        Ok(this)
    }

    // java: <init>(Ljava/io/File;)V
    // java: <init>(Ljava/io/File;)V
    pub fn new__file(file: Object) -> Result<Self> {
        let this = Self { fd: Field::new(Default::default()), channel: Field::new(Default::default()), path: Field::new(String::new()), closeLock: Field::new(Default::default()), closed: Field::new(false) };
        /* invokespecial Method java/io/FileOutputStream.<init>:(Ljava/io/File;Z)V */
        Ok(this)
    }

    // java: <init>(Ljava/io/File;Z)V
    // java: <init>(Ljava/io/File;Z)V
    pub fn new__file_z(file: Object, append: bool) -> Result<Self> {
        let this = Self { fd: Field::new(Default::default()), channel: Field::new(Default::default()), path: Field::new(String::new()), closeLock: Field::new(Default::default()), closed: Field::new(false) };
        /* invokespecial Method java/io/OutputStream.<init>:()V */
        this.closeLock.set(Object::new()?);
        let _t0 = file.getPath()?;
        /* TODO: aconst_null  */
        let mut name: String = _t0;
        let _t1: Object = System::getSecurityManager()?;
        let mut security: Object = _t1;
        security.checkWrite(name)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t2 = file.isInvalid()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.fd.set(FileDescriptor::new()?);
        this.fd.get().attach(this)?;
        this.path.set(name);
        this.open(name, append)?;
        FileCleanable::register(this.fd.get())?;
        Ok(this)
    }

    // java: <init>(Ljava/io/FileDescriptor;)V
    // java: <init>(Ljava/io/FileDescriptor;)V
    pub fn new__filede(fdObj: Object) -> Result<Self> {
        let this = Self { fd: Field::new(Default::default()), channel: Field::new(Default::default()), path: Field::new(String::new()), closeLock: Field::new(Default::default()), closed: Field::new(false) };
        /* invokespecial Method java/io/OutputStream.<init>:()V */
        this.closeLock.set(Object::new()?);
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        return Err(JvmError::Custom("athrow".to_owned()));
        security.checkWrite(fdObj)?;
        this.fd.set(fdObj);
        /* TODO: aconst_null  */
        security.path.set(this);
        this.fd.get().attach(this)?;
        Ok(this)
    }

    // java: open0(Ljava/lang/String;Z)V
    pub fn open0(&self, arg0: String, arg1: bool) -> Result<()> {
        todo!("native java/io/FileOutputStream.open0")
    }

    // java: open(Ljava/lang/String;Z)V
    pub fn open(&self, name: String, append: bool) -> Result<()> {
        let this = self;
        let _t0: i64 = Blocker::begin()?;
        let mut comp: i64 = _t0;
        this.open0(name, append)?;
        Blocker::end(comp)?;
        let mut local_5: i32 = todo!("stack underflow");
        Blocker::end(comp)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: write(IZ)V
    pub fn write__i_z(&self, arg0: i32, arg1: bool) -> Result<()> {
        todo!("native java/io/FileOutputStream.write")
    }

    // java: write(I)V
    // java: write(I)V
    pub fn write__i(&self, b: i32) -> Result<()> {
        let this = self;
        let _t0 = FileOutputStream::FD_ACCESS().getAppend(this.fd.get())?;
        let mut append: i32 = _t0;
        let _t1: i64 = Blocker::begin()?;
        let mut comp: i64 = _t1;
        this.write(b, append)?;
        Blocker::end(comp)?;
        let mut local_5: i32 = todo!("stack underflow");
        Blocker::end(comp)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: writeBytes([BIIZ)V
    pub fn writeBytes(&self, arg0: Vec<i8>, arg1: i32, arg2: i32, arg3: bool) -> Result<()> {
        todo!("native java/io/FileOutputStream.writeBytes")
    }

    // java: write([B)V
    // java: write([B)V
    pub fn write__arr_b(&self, b: Vec<i8>) -> Result<()> {
        let this = self;
        let _t0 = FileOutputStream::FD_ACCESS().getAppend(this.fd.get())?;
        let mut append: i32 = _t0;
        let _t1: i64 = Blocker::begin()?;
        let mut comp: i64 = _t1;
        this.writeBytes(b, 0i32, (b.len() as i32), append)?;
        Blocker::end(comp)?;
        let mut local_5: i32 = todo!("stack underflow");
        Blocker::end(comp)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: write([BII)V
    // java: write([BII)V
    pub fn write__arr_b_i_i(&self, b: Vec<i8>, off: i32, len: i32) -> Result<()> {
        let this = self;
        let _t0 = FileOutputStream::FD_ACCESS().getAppend(this.fd.get())?;
        let mut append: i32 = _t0;
        let _t1: i64 = Blocker::begin()?;
        let mut comp: i64 = _t1;
        this.writeBytes(b, off, len, append)?;
        Blocker::end(comp)?;
        let mut local_7: i32 = todo!("stack underflow");
        Blocker::end(comp)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: close()V
    pub fn close(&self) -> Result<()> {
        let this = self;
        return Ok(());
        let mut fc: Object = this.closeLock.get();
        /* TODO: monitorenter  */
        /* TODO: monitorexit  */
        return Ok(());
        this.closed.set(1i32);
        /* TODO: monitorexit  */
        let mut local_2: Object = fc;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        fc = this.channel.get();
        fc.close()?;
        this.fd.get().closeAll(FileOutputStream_1::new(this)?)?;
        Ok(())
    }

    // java: getFD()Ljava/io/FileDescriptor;
    pub fn getFD(&self) -> Result<Object> {
        let this = self;
        return Ok(this.fd.get());
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: getChannel()Ljava/nio/channels/FileChannel;
    pub fn getChannel(&self) -> Result<Object> {
        let this = self;
        let mut fc: Object = this.channel.get();
        let mut local_2: FileOutputStream = this;
        /* TODO: monitorenter  */
        fc = this.channel.get();
        let _t0: Object = FileChannelImpl::open(this.fd.get(), this.path.get(), 0i32, 1i32, 0i32, this)?;
        fc = _t0;
        this.channel.set(_t0);
        fc.close()?;
        let mut ioe: bool = this.closed.get();
        return Err(JvmError::Custom("athrow".to_owned()));
        /* TODO: monitorexit  */
        let mut local_4: FileOutputStream = local_2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(fc)
    }

    // java: initIDs()V
    pub fn initIDs() -> Result<()> {
        todo!("native java/io/FileOutputStream.initIDs")
    }
}
