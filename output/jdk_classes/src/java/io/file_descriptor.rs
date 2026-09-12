#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/FileDescriptor",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public final",
    source      = "FileDescriptor.java",
))]
pub struct FileDescriptor {
    #[cfg_attr(any(), java_field(name = "fd", descriptor = "I", access = "private"))]
    pub fd: Field<i32>,
    #[cfg_attr(any(), java_field(name = "handle", descriptor = "J", access = "private"))]
    pub handle: Field<i64>,
    #[cfg_attr(any(), java_field(name = "parent", descriptor = "Ljava/io/Closeable;", access = "private"))]
    pub parent: Field<Object>,
    #[cfg_attr(any(), java_field(name = "otherParents", descriptor = "Ljava/util/List;", access = "private"))]
    pub otherParents: Field<Object>,
    #[cfg_attr(any(), java_field(name = "closed", descriptor = "Z", access = "private"))]
    pub closed: Field<bool>,
    #[cfg_attr(any(), java_field(name = "append", descriptor = "Z", access = "private"))]
    pub append: Field<bool>,
    #[cfg_attr(any(), java_field(name = "cleanup", descriptor = "Ljdk/internal/ref/PhantomCleanable;", access = "private"))]
    pub cleanup: Field<Object>,
}

impl FileDescriptor {
    // java: <init>()V
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { fd: Field::new(0), handle: Field::new(0), parent: Field::new(Default::default()), otherParents: Field::new(Default::default()), closed: Field::new(false), append: Field::new(false), cleanup: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.fd.set(-1i32);
        this.handle.set(18446744073709551615i64);
        Ok(this)
    }

    // java: <init>(I)V
    // java: <init>(I)V
    pub fn new__i(fd: i32) -> Result<Self> {
        let this = Self { fd: Field::new(0), handle: Field::new(0), parent: Field::new(Default::default()), otherParents: Field::new(Default::default()), closed: Field::new(false), append: Field::new(false), cleanup: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.fd.set(fd);
        let _t0: i64 = FileDescriptor::getHandle(fd)?;
        this.handle.set(_t0);
        let _t1: bool = FileDescriptor::getAppend(fd)?;
        this.append.set(_t1);
        Ok(this)
    }

    // java: valid()Z
    pub fn valid(&self) -> Result<bool> {
        let this = self;
        /* TODO: lcmp  */
        Ok(this.fd.get() != -1i32)
    }

    // java: sync()V
    pub fn sync(&self) -> Result<()> {
        let this = self;
        let _t0: i64 = Blocker::begin()?;
        let mut comp: i64 = _t0;
        this.sync0()?;
        Blocker::end(comp)?;
        let mut local_3: i32 = todo!("stack underflow");
        Blocker::end(comp)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: sync0()V
    pub fn sync0(&self) -> Result<()> {
        todo!("native java/io/FileDescriptor.sync0")
    }

    // java: initIDs()V
    pub fn initIDs() -> Result<()> {
        todo!("native java/io/FileDescriptor.initIDs")
    }

    // java: getHandle(I)J
    pub fn getHandle(arg0: i32) -> Result<i64> {
        todo!("native java/io/FileDescriptor.getHandle")
    }

    // java: getAppend(I)Z
    pub fn getAppend(arg0: i32) -> Result<bool> {
        todo!("native java/io/FileDescriptor.getAppend")
    }

    // java: set(I)V
    pub fn set(&self, fd: i32) -> Result<()> {
        let this = self;
        this.cleanup.get().clear()?;
        /* TODO: aconst_null  */
        this.cleanup.get().cleanup.set(this);
        this.fd.set(fd);
        Ok(())
    }

    // java: setHandle(J)V
    pub fn setHandle(&self, handle: i64) -> Result<()> {
        let this = self;
        /* TODO: lcmp  */
        this.cleanup.get().clear()?;
        /* TODO: aconst_null  */
        this.cleanup.get().cleanup.set(this);
        this.handle.set(handle);
        Ok(())
    }

    // java: registerCleanup(Ljdk/internal/ref/PhantomCleanable;)V
    pub fn registerCleanup(&self, cleanable: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(cleanable, String::from("cleanable"))?;
        this.cleanup.get().clear()?;
        this.cleanup.set(cleanable);
        Ok(())
    }

    // java: unregisterCleanup()V
    pub fn unregisterCleanup(&self) -> Result<()> {
        let this = self;
        this.cleanup.get().clear()?;
        /* TODO: aconst_null  */
        this.cleanup.get().cleanup.set(this);
        Ok(())
    }

    // java: close()V
    pub fn close(&self) -> Result<()> {
        let this = self;
        this.unregisterCleanup()?;
        this.close0()?;
        Ok(())
    }

    // java: close0()V
    pub fn close0(&self) -> Result<()> {
        todo!("native java/io/FileDescriptor.close0")
    }

    // java: attach(Ljava/io/Closeable;)V
    pub fn attach(&self, c: Object) -> Result<()> {
        let this = self;
        this.parent.set(c);
        this.otherParents.set(ArrayList::<_>::new()?);
        let _t0 = this.otherParents.get().add(this.parent.get())?;
        let _t1 = this.otherParents.get().add(c)?;
        let _t2 = this.otherParents.get().add(c)?;
        Ok(())
    }

    // java: closeAll(Ljava/io/Closeable;)V
    pub fn closeAll(&self, releaser: Object) -> Result<()> {
        let this = self;
        this.closed.set(1i32);
        /* TODO: aconst_null  */
        let mut ioe: bool = this.closed.get();
        let mut ex: Object = releaser;
        let _t0 = this.otherParents.get().iterator()?;
        let mut local_4: Object = _t0;
        loop {
            let _t0 = local_4.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_4.next()?;
            let mut referent: Object = _t0;
            referent.close()?;
            let mut x: i32 = todo!("stack underflow");
            ioe = x;
            ioe.addSuppressed(x)?;
        }
        ex.close()?;
        local_4 = ex;
        ex.close()?;
        referent = ex;
        local_4.addSuppressed(referent)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        ex = ioe;
        ex.addSuppressed(ioe)?;
        ioe = ex;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut local_7: bool = ioe;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }
}
