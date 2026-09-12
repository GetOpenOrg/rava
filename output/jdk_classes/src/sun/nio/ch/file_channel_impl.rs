#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "sun/nio/ch/FileChannelImpl",
    super_class = "java/nio/channels/FileChannel",
    interfaces  = "",
    access      = "public",
    source      = "FileChannelImpl.java",
))]
pub struct FileChannelImpl {
    #[cfg_attr(any(), java_field(name = "fd", descriptor = "Ljava/io/FileDescriptor;", access = "private final"))]
    pub fd: Field<Object>,
    #[cfg_attr(any(), java_field(name = "writable", descriptor = "Z", access = "private final"))]
    pub writable: Field<bool>,
    #[cfg_attr(any(), java_field(name = "readable", descriptor = "Z", access = "private final"))]
    pub readable: Field<bool>,
    #[cfg_attr(any(), java_field(name = "parent", descriptor = "Ljava/io/Closeable;", access = "private final"))]
    pub parent: Field<Object>,
    #[cfg_attr(any(), java_field(name = "path", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub path: Field<String>,
    #[cfg_attr(any(), java_field(name = "threads", descriptor = "Lsun/nio/ch/NativeThreadSet;", access = "private final"))]
    pub threads: Field<Object>,
    #[cfg_attr(any(), java_field(name = "positionLock", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub positionLock: Field<Object>,
    #[cfg_attr(any(), java_field(name = "uninterruptible", descriptor = "Z", access = "private"))]
    pub uninterruptible: Field<bool>,
    #[cfg_attr(any(), java_field(name = "direct", descriptor = "Z", access = "private final"))]
    pub direct: Field<bool>,
    #[cfg_attr(any(), java_field(name = "alignment", descriptor = "I", access = "private final"))]
    pub alignment: Field<i32>,
    #[cfg_attr(any(), java_field(name = "closer", descriptor = "Ljava/lang/ref/Cleaner$Cleanable;", access = "private final"))]
    pub closer: Field<Object>,
    #[cfg_attr(any(), java_field(name = "fileLockTable", descriptor = "Lsun/nio/ch/FileLockTable;", access = "private"))]
    pub fileLockTable: Field<Object>,
}

impl FileChannelImpl {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/FileDescriptor;Ljava/lang/String;ZZZLjava/io/Closeable;)V", access = "private"))]
    pub fn new(fd: Object, path: String, readable: bool, writable: bool, direct: bool, parent: Object) -> Result<Self> {
        let this = Self { fd: Field::new(Default::default()), writable: Field::new(false), readable: Field::new(false), parent: Field::new(Default::default()), path: Field::new(String::new()), threads: Field::new(Default::default()), positionLock: Field::new(Default::default()), uninterruptible: Field::new(false), direct: Field::new(false), alignment: Field::new(0), closer: Field::new(Default::default()), fileLockTable: Field::new(Default::default()) };
        /* invokespecial Method java/nio/channels/FileChannel.<init>:()V */
        this.threads.set(NativeThreadSet::new(2i32)?);
        this.positionLock.set(Object::new()?);
        this.fd.set(fd);
        this.path.set(path);
        this.readable.set(readable);
        this.writable.set(writable);
        this.direct.set(direct);
        this.parent.set(parent);
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0 = FileChannelImpl::nd().setDirectIO(fd, path)?;
        this.alignment.set(_t0);
        this.alignment.set(-1i32);
        /* TODO: aconst_null  */
        let _t1: Object = CleanerFactory::cleaner()?;
        let _t2 = _t1.register(this, FileChannelImpl_Closer::new(fd)?)?;
        parent.closer.set(_t2);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "open", descriptor = "(Ljava/io/FileDescriptor;Ljava/lang/String;ZZZLjava/io/Closeable;)Ljava/nio/channels/FileChannel;", access = "public static"))]
    pub fn open(fd: Object, path: String, readable: bool, writable: bool, direct: bool, parent: Object) -> Result<Object> {
        Ok(FileChannelImpl::new(fd, path, readable, writable, direct, parent)?)
    }

    #[cfg_attr(any(), java_method(name = "ensureOpen", descriptor = "()V", access = "private"))]
    pub fn ensureOpen(&self) -> Result<()> {
        let this = self;
        let _t0 = this.isOpen()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "setUninterruptible", descriptor = "()V", access = "public"))]
    pub fn setUninterruptible(&self) -> Result<()> {
        let this = self;
        this.uninterruptible.set(1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "beginBlocking", descriptor = "()V", access = "private"))]
    pub fn beginBlocking(&self) -> Result<()> {
        let this = self;
        this.begin()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "endBlocking", descriptor = "(Z)V", access = "private"))]
    pub fn endBlocking(&self, completed: bool) -> Result<()> {
        let this = self;
        this.end(completed)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implCloseChannel", descriptor = "()V", access = "protected"))]
    pub fn implCloseChannel(&self) -> Result<()> {
        let this = self;
        let _t0 = this.fd.get().valid()?;
        return Ok(());
        let _t1 = this.fileLockTable.get().removeAll()?;
        let _t2 = _t1.iterator()?;
        let mut uioe: Object = _t2;
        loop {
            let _t0 = uioe.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = uioe.next()?;
            let mut fl: Object = _t0;
            let mut local_3: Object = fl;
            /* TODO: monitorenter  */
            let _t1 = fl.isValid()?;
            let _t2 = fl.position()?;
            let _t3 = fl.size()?;
            FileChannelImpl::nd().release(this.fd.get(), _t2, _t3)?;
            fl.invalidate()?;
            /* TODO: monitorexit  */
            let mut local_4: Object = local_3;
            /* TODO: monitorexit  */
            return Err(JvmError::Custom(String::from("athrow")));
        }
        this.threads.get().signalAndWait()?;
        this.parent.get().close()?;
        this.closer.get().clean()?;
        uioe = this.parent.get();
        let _t3 = uioe.getCause()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "read", descriptor = "(Ljava/nio/ByteBuffer;)I", access = "public"))]
    // java: read(Ljava/nio/ByteBuffer;)I
    pub fn read__bytebu(&self, dst: Object) -> Result<i32> {
        let this = self;
        this.ensureOpen()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_2: Object = this.positionLock.get();
        /* TODO: monitorenter  */
        let _t0 = this.position()?;
        Util::checkChannelPositionAligned(_t0, this.alignment.get())?;
        let mut n: i32 = 0i32;
        let mut ti: i32 = -1i32;
        this.beginBlocking()?;
        let _t1 = this.threads.get().add()?;
        ti = _t1;
        let _t2 = this.isOpen()?;
        let mut comp: i32 = 0i32;
        this.threads.get().remove(ti)?;
        this.endBlocking(n>0i32)?;
        let _t3: bool = IOStatus::check(n)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        return Ok(comp);
        let _t4: i64 = Blocker::begin()?;
        comp = _t4;
        let _t5: i32 = IOUtil::read(this.fd.get(), dst, 18446744073709551615i64, this.direct.get(), this.alignment.get(), FileChannelImpl::nd())?;
        n = _t5;
        Blocker::end(comp)?;
        let mut local_7: Object = local_2;
        Blocker::end(comp)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t6 = this.isOpen()?;
        let _t7: i32 = IOStatus::normalize(n)?;
        comp = _t7;
        this.threads.get().remove(ti)?;
        this.endBlocking(n>0i32)?;
        let _t8: bool = IOStatus::check(n)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        return Ok(comp);
        let mut local_8: Object = local_2;
        this.threads.get().remove(ti)?;
        this.endBlocking(n>0i32)?;
        let _t9: bool = IOStatus::check(n)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_9: bool = _t9;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "read", descriptor = "([Ljava/nio/ByteBuffer;II)J", access = "public"))]
    // java: read([Ljava/nio/ByteBuffer;II)J
    pub fn read__arr_byt_i_i(&self, dsts: Vec<Object>, offset: i32, length: i32) -> Result<i64> {
        let this = self;
        let _t0: i32 = Objects::checkFromIndexSize(offset, length, (dsts.len() as i32))?;
        this.ensureOpen()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_4: Object = this.positionLock.get();
        /* TODO: monitorenter  */
        let _t1 = this.position()?;
        Util::checkChannelPositionAligned(_t1, this.alignment.get())?;
        let mut n: i64 = 0i64;
        let mut ti: i32 = -1i32;
        this.beginBlocking()?;
        let _t2 = this.threads.get().add()?;
        ti = _t2;
        let _t3 = this.isOpen()?;
        let mut comp: i64 = 0i64;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        n.endBlocking(0i64>0i32)?;
        let _t4: bool = IOStatus::check(n)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        return Ok(comp);
        let _t5: i64 = Blocker::begin()?;
        comp = _t5;
        let _t6: i64 = IOUtil::read(this.fd.get(), &dsts, offset, length, this.direct.get(), this.alignment.get(), FileChannelImpl::nd())?;
        n = _t6;
        Blocker::end(comp)?;
        let mut local_10: Object = local_4;
        Blocker::end(comp)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        let _t7 = this.isOpen()?;
        let _t8: i64 = IOStatus::normalize(n)?;
        comp = _t8;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        n.endBlocking(0i64>0i32)?;
        let _t9: bool = IOStatus::check(n)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        return Ok(comp);
        let mut local_11: Object = local_4;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        n.endBlocking(0i64>0i32)?;
        let _t10: bool = IOStatus::check(n)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_12: bool = _t10;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(Ljava/nio/ByteBuffer;)I", access = "public"))]
    // java: write(Ljava/nio/ByteBuffer;)I
    pub fn write__bytebu(&self, src: Object) -> Result<i32> {
        let this = self;
        this.ensureOpen()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_2: Object = this.positionLock.get();
        /* TODO: monitorenter  */
        let _t0 = this.position()?;
        Util::checkChannelPositionAligned(_t0, this.alignment.get())?;
        let mut n: i32 = 0i32;
        let mut ti: i32 = -1i32;
        this.beginBlocking()?;
        let _t1 = this.threads.get().add()?;
        ti = _t1;
        let _t2 = this.isOpen()?;
        let mut comp: i32 = 0i32;
        this.threads.get().remove(ti)?;
        this.endBlocking(n>0i32)?;
        let _t3: bool = IOStatus::check(n)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        return Ok(comp);
        let _t4: i64 = Blocker::begin()?;
        comp = _t4;
        let _t5: i32 = IOUtil::write(this.fd.get(), src, 18446744073709551615i64, this.direct.get(), this.alignment.get(), FileChannelImpl::nd())?;
        n = _t5;
        Blocker::end(comp)?;
        let mut local_7: Object = local_2;
        Blocker::end(comp)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t6 = this.isOpen()?;
        let _t7: i32 = IOStatus::normalize(n)?;
        comp = _t7;
        this.threads.get().remove(ti)?;
        this.endBlocking(n>0i32)?;
        let _t8: bool = IOStatus::check(n)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        return Ok(comp);
        let mut local_8: Object = local_2;
        this.threads.get().remove(ti)?;
        this.endBlocking(n>0i32)?;
        let _t9: bool = IOStatus::check(n)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_9: bool = _t9;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([Ljava/nio/ByteBuffer;II)J", access = "public"))]
    // java: write([Ljava/nio/ByteBuffer;II)J
    pub fn write__arr_byt_i_i(&self, srcs: Vec<Object>, offset: i32, length: i32) -> Result<i64> {
        let this = self;
        let _t0: i32 = Objects::checkFromIndexSize(offset, length, (srcs.len() as i32))?;
        this.ensureOpen()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_4: Object = this.positionLock.get();
        /* TODO: monitorenter  */
        let _t1 = this.position()?;
        Util::checkChannelPositionAligned(_t1, this.alignment.get())?;
        let mut n: i64 = 0i64;
        let mut ti: i32 = -1i32;
        this.beginBlocking()?;
        let _t2 = this.threads.get().add()?;
        ti = _t2;
        let _t3 = this.isOpen()?;
        let mut comp: i64 = 0i64;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        n.endBlocking(0i64>0i32)?;
        let _t4: bool = IOStatus::check(n)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        return Ok(comp);
        let _t5: i64 = Blocker::begin()?;
        comp = _t5;
        let _t6: i64 = IOUtil::write(this.fd.get(), &srcs, offset, length, this.direct.get(), this.alignment.get(), FileChannelImpl::nd())?;
        n = _t6;
        Blocker::end(comp)?;
        let mut local_10: Object = local_4;
        Blocker::end(comp)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        let _t7 = this.isOpen()?;
        let _t8: i64 = IOStatus::normalize(n)?;
        comp = _t8;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        n.endBlocking(0i64>0i32)?;
        let _t9: bool = IOStatus::check(n)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        return Ok(comp);
        let mut local_11: Object = local_4;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        n.endBlocking(0i64>0i32)?;
        let _t10: bool = IOStatus::check(n)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_12: bool = _t10;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "position", descriptor = "()J", access = "public"))]
    // java: position()J
    pub fn position(&self) -> Result<i64> {
        let this = self;
        this.ensureOpen()?;
        let mut local_1: Object = this.positionLock.get();
        /* TODO: monitorenter  */
        let mut p: i64 = 18446744073709551615i64;
        let mut ti: i32 = -1i32;
        this.beginBlocking()?;
        let _t0 = this.threads.get().add()?;
        ti = _t0;
        let _t1 = this.isOpen()?;
        let mut append: i64 = 0i64;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        p.endBlocking(18446744073709551615i64>0i32)?;
        let _t2: bool = IOStatus::check(p)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        return Ok(append);
        let _t3 = FileChannelImpl::fdAccess().getAppend(this.fd.get())?;
        append = _t3;
        let _t4: i64 = Blocker::begin()?;
        let mut comp: i64 = _t4;
        let _t5 = FileChannelImpl::nd().size(this.fd.get())?;
        let _t6 = FileChannelImpl::nd().seek(this.fd.get(), 18446744073709551615i64)?;
        p = _t6;
        Blocker::end(comp)?;
        let mut local_8: i64 = _t5;
        Blocker::end(comp)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        let _t7 = this.isOpen()?;
        let _t8: i64 = IOStatus::normalize(p)?;
        comp = _t8;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        p.endBlocking(18446744073709551615i64>0i32)?;
        let _t9: bool = IOStatus::check(p)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        return Ok(comp);
        let mut local_9: Object = local_1;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        p.endBlocking(18446744073709551615i64>0i32)?;
        let _t10: bool = IOStatus::check(p)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_10: bool = _t10;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "position", descriptor = "(J)Ljava/nio/channels/FileChannel;", access = "public"))]
    // java: position(J)Ljava/nio/channels/FileChannel;
    pub fn position__l(&self, newPosition: i64) -> Result<Object> {
        let this = self;
        this.ensureOpen()?;
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_3: Object = this.positionLock.get();
        /* TODO: monitorenter  */
        let mut p: i64 = 18446744073709551615i64;
        let mut ti: i32 = -1i32;
        this.beginBlocking()?;
        let _t0 = this.threads.get().add()?;
        ti = _t0;
        let _t1 = this.isOpen()?;
        /* TODO: aconst_null  */
        let mut comp: bool = _t1;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        p.endBlocking(18446744073709551615i64>0i32)?;
        let _t2: bool = IOStatus::check(p)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        return Ok(comp);
        let _t3: i64 = Blocker::begin()?;
        comp = _t3;
        let _t4 = FileChannelImpl::nd().seek(this.fd.get(), newPosition)?;
        p = _t4;
        Blocker::end(comp)?;
        let mut local_9: Object = local_3;
        Blocker::end(comp)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        let _t5 = this.isOpen()?;
        comp = this;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        p.endBlocking(18446744073709551615i64>0i32)?;
        let _t6: bool = IOStatus::check(p)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        return Ok(comp);
        let mut local_10: Object = local_3;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        p.endBlocking(18446744073709551615i64>0i32)?;
        let _t7: bool = IOStatus::check(p)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_11: bool = _t7;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "size", descriptor = "()J", access = "public"))]
    pub fn size(&self) -> Result<i64> {
        let this = self;
        this.ensureOpen()?;
        let mut local_1: Object = this.positionLock.get();
        /* TODO: monitorenter  */
        let mut s: i64 = 18446744073709551615i64;
        let mut ti: i32 = -1i32;
        this.beginBlocking()?;
        let _t0 = this.threads.get().add()?;
        ti = _t0;
        let _t1 = this.isOpen()?;
        let mut comp: i64 = 18446744073709551615i64;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        s.endBlocking(18446744073709551615i64>0i32)?;
        let _t2: bool = IOStatus::check(s)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        return Ok(comp);
        let _t3: i64 = Blocker::begin()?;
        comp = _t3;
        let _t4 = FileChannelImpl::nd().size(this.fd.get())?;
        s = _t4;
        Blocker::end(comp)?;
        let mut local_7: Object = local_1;
        Blocker::end(comp)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        let _t5 = this.isOpen()?;
        let _t6: i64 = IOStatus::normalize(s)?;
        comp = _t6;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        s.endBlocking(18446744073709551615i64>0i32)?;
        let _t7: bool = IOStatus::check(s)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        return Ok(comp);
        let mut local_8: Object = local_1;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        s.endBlocking(18446744073709551615i64>0i32)?;
        let _t8: bool = IOStatus::check(s)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_9: bool = _t8;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "truncate", descriptor = "(J)Ljava/nio/channels/FileChannel;", access = "public"))]
    pub fn truncate(&self, newSize: i64) -> Result<Object> {
        let this = self;
        this.ensureOpen()?;
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_3: Object = this.positionLock.get();
        /* TODO: monitorenter  */
        let mut rv: i32 = -1i32;
        let mut p: i64 = 18446744073709551615i64;
        let mut ti: i32 = -1i32;
        let mut rp: i64 = 18446744073709551615i64;
        this.beginBlocking()?;
        let _t0 = this.threads.get().add()?;
        ti = _t0;
        let _t1 = this.isOpen()?;
        /* TODO: aconst_null  */
        let mut size: bool = _t1;
        this.threads.get().remove(ti)?;
        this.endBlocking(rv > -1i32)?;
        let _t2: bool = IOStatus::check(rv)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        return Ok(size);
        let _t3: i64 = Blocker::begin()?;
        let mut comp: i64 = _t3;
        let _t4 = FileChannelImpl::nd().size(this.fd.get())?;
        size = _t4;
        Blocker::end(comp)?;
        let mut local_14: Object = local_3;
        Blocker::end(comp)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        let _t5 = this.isOpen()?;
        let _t6 = this.isOpen()?;
        /* TODO: aconst_null  */
        comp = _t6;
        this.threads.get().remove(ti)?;
        this.endBlocking(rv > -1i32)?;
        let _t7: bool = IOStatus::check(rv)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        return Ok(comp);
        let _t8: i64 = Blocker::begin()?;
        comp = _t8;
        let _t9 = FileChannelImpl::nd().seek(this.fd.get(), 18446744073709551615i64)?;
        p = _t9;
        Blocker::end(comp)?;
        let mut local_15: Object = local_3;
        Blocker::end(comp)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        let _t10 = this.isOpen()?;
        let _t11 = this.isOpen()?;
        /* TODO: aconst_null  */
        comp = _t11;
        this.threads.get().remove(ti)?;
        this.endBlocking(rv > -1i32)?;
        let _t12: bool = IOStatus::check(rv)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        return Ok(comp);
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        let _t13: i64 = Blocker::begin()?;
        comp = _t13;
        let _t14 = FileChannelImpl::nd().truncate(this.fd.get(), newSize)?;
        rv = _t14;
        Blocker::end(comp)?;
        let mut local_16: i64 = size;
        Blocker::end(comp)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t15 = this.isOpen()?;
        let _t16 = this.isOpen()?;
        /* TODO: aconst_null  */
        comp = _t16;
        this.threads.get().remove(ti)?;
        this.endBlocking(rv > -1i32)?;
        let _t17: bool = IOStatus::check(rv)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        return Ok(comp);
        /* TODO: lcmp  */
        p = newSize;
        let _t18: i64 = Blocker::begin()?;
        comp = _t18;
        let _t19 = FileChannelImpl::nd().seek(this.fd.get(), p)?;
        rp = _t19;
        Blocker::end(comp)?;
        let mut local_17: i64 = newSize;
        Blocker::end(comp)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        let _t20 = this.isOpen()?;
        comp = this;
        this.threads.get().remove(ti)?;
        this.endBlocking(rv > -1i32)?;
        let _t21: bool = IOStatus::check(rv)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        return Ok(comp);
        let mut local_18: Object = local_3;
        this.threads.get().remove(ti)?;
        this.endBlocking(rv > -1i32)?;
        let _t22: bool = IOStatus::check(rv)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_19: bool = _t22;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "force", descriptor = "(Z)V", access = "public"))]
    pub fn force(&self, metaData: bool) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        let mut rv: i32 = -1i32;
        let mut ti: i32 = -1i32;
        this.beginBlocking()?;
        let _t0 = this.threads.get().add()?;
        ti = _t0;
        let _t1 = this.isOpen()?;
        this.threads.get().remove(ti)?;
        this.endBlocking(rv > -1i32)?;
        let _t2: bool = IOStatus::check(rv)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(());
        let _t3: i64 = Blocker::begin()?;
        let mut comp: i64 = _t3;
        let _t4 = FileChannelImpl::nd().force(this.fd.get(), metaData)?;
        rv = _t4;
        Blocker::end(comp)?;
        let mut local_6: bool = _t2;
        Blocker::end(comp)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t5 = this.isOpen()?;
        this.threads.get().remove(ti)?;
        this.endBlocking(rv > -1i32)?;
        let _t6: bool = IOStatus::check(rv)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_7: bool = _t6;
        this.threads.get().remove(ti)?;
        this.endBlocking(rv > -1i32)?;
        let _t7: bool = IOStatus::check(rv)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "transferToDirectlyInternal", descriptor = "(JILjava/nio/channels/WritableByteChannel;Ljava/io/FileDescriptor;)J", access = "private"))]
    pub fn transferToDirectlyInternal(&self, position: i64, arg_1: i32, icount: Object, target: Object) -> Result<i64> {
        let this = self;
        let _t0 = FileChannelImpl::nd().transferToDirectlyNeedsPositionLock()?;
        let _t1: bool = Thread::holdsLock(this.positionLock.get())?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut n: i64 = 18446744073709551615i64;
        let mut ti: i32 = -1i32;
        this.beginBlocking()?;
        let _t2 = this.threads.get().add()?;
        ti = _t2;
        let _t3 = this.isOpen()?;
        let mut append: i64 = 18446744073709551615i64;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        n.end(18446744073709551615i64>0i32)?;
        return Ok(append);
        let _t4 = FileChannelImpl::fdAccess().getAppend(local_5)?;
        append = _t4;
        let _t5: i64 = Blocker::begin()?;
        let mut comp: i64 = _t5;
        let _t6 = FileChannelImpl::nd().transferTo(this.fd.get(), position, (icount as i64), local_5, append)?;
        n = _t6;
        Blocker::end(comp)?;
        let mut local_12: sun/nio/ch/FileChannelImpl = this;
        Blocker::end(comp)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        let _t7 = this.isOpen()?;
        /* TODO: lcmp  */
        FileChannelImpl::pipeSupported(0i32);
        FileChannelImpl::fileSupported(0i32);
        comp = 18446744073709551610i64;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        n.end(18446744073709551615i64>0i32)?;
        return Ok(comp);
        /* TODO: lcmp  */
        FileChannelImpl::transferToNotSupported(1i32);
        comp = 18446744073709551612i64;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        n.end(18446744073709551615i64>0i32)?;
        return Ok(comp);
        let _t8: i64 = IOStatus::normalize(n)?;
        comp = _t8;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        n.end(18446744073709551615i64>0i32)?;
        return Ok(comp);
        let mut local_13: sun/nio/ch/FileChannelImpl = this;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        n.end(18446744073709551615i64>0i32)?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "transferToDirectly", descriptor = "(JILjava/nio/channels/WritableByteChannel;)J", access = "private"))]
    pub fn transferToDirectly(&self, position: i64, arg_1: i32, icount: Object) -> Result<i64> {
        let this = self;
        return Ok(18446744073709551612i64);
        /* TODO: aconst_null  */
        let mut targetFD: bool = FileChannelImpl::transferToNotSupported();
        return Ok(18446744073709551610i64);
        targetFD = local_4.fd.get();
        return Ok(18446744073709551610i64);
        let mut sc: i32 = local_4;
        let _t0 = FileChannelImpl::nd().canTransferToDirectly(sc)?;
        return Ok(18446744073709551610i64);
        let _t1 = local_4.getFD()?;
        targetFD = _t1;
        return Ok(18446744073709551612i64);
        let _t2: i32 = IOUtil::fdVal(this.fd.get())?;
        sc = _t2;
        let _t3: i32 = IOUtil::fdVal(targetFD)?;
        let mut targetFDVal: i32 = _t3;
        return Ok(18446744073709551612i64);
        let _t4 = FileChannelImpl::nd().transferToDirectlyNeedsPositionLock()?;
        let mut local_8: Object = this.positionLock.get();
        /* TODO: monitorenter  */
        let _t5 = this.position()?;
        let mut pos: i64 = _t5;
        let _t6 = this.transferToDirectlyInternal(position, icount, local_4, targetFD)?;
        let mut local_11: i64 = _t6;
        let _t7 = this.position(pos)?;
        /* TODO: monitorexit  */
        return Ok(local_11);
        let mut local_13: Object = local_8;
        let _t8 = this.position(pos)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_14: Object = this.positionLock.get();
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        let _t9 = this.transferToDirectlyInternal(position, icount, local_4, targetFD)?;
        Ok(_t9)
    }

    #[cfg_attr(any(), java_method(name = "transferToTrustedChannel", descriptor = "(JJLjava/nio/channels/WritableByteChannel;)J", access = "private"))]
    pub fn transferToTrustedChannel(&self, position: i64, arg_1: i64, count: Object) -> Result<i64> {
        let this = self;
        /* TODO: lcmp  */
        return Ok(18446744073709551610i64);
        let mut isSelChImpl: i32 = true;
        return Ok(18446744073709551612i64);
        let _t0 = this.position()?;
        let mut posThis: i64 = _t0;
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let _t1 = FileChannelImpl::nd().canTransferToFromOverlappedMap()?;
        return Ok(18446744073709551610i64);
        posThis = count;
        loop {
            /* TODO: lcmp  */
            if 0i64<=0i32 { break; }
            let _t0: i64 = (posThis).min(8388608i64);
            let mut size: i64 = _t0;
            let _t1 = this.map(FileChannel$MapMode::READ_ONLY(), position, size)?;
            let mut dbb: Object = _t1;
            let _t2 = local_5.write(dbb)?;
            let mut n: i32 = _t2;
            return Err(JvmError::Custom(String::from("athrow")));
            posThis = (posThis).wrapping_sub((n as i64));
            FileChannelImpl::unmap(dbb)?;
            return Err(JvmError::Custom(String::from("athrow")));
            position = (position).wrapping_add((n as i64));
            FileChannelImpl::unmap(dbb)?;
            let mut local_13: i32 = n;
            FileChannelImpl::unmap(dbb)?;
            return Err(JvmError::Custom(String::from("athrow")));
            dbb = FileChannelImpl::$assertionsDisabled();
            let _t3 = local_5.isOpen()?;
            return Err(JvmError::Custom(String::from("athrow")));
            this.close()?;
            n = _t3;
            dbb.addSuppressed(n)?;
            return Err(JvmError::Custom(String::from("athrow")));
            dbb = FileChannelImpl::$assertionsDisabled();
            /* TODO: lcmp  */
            return Err(JvmError::Custom(String::from("athrow")));
        }
        Ok((count).wrapping_sub(posThis))
    }

    #[cfg_attr(any(), java_method(name = "transferToArbitraryChannel", descriptor = "(JJLjava/nio/channels/WritableByteChannel;)J", access = "private"))]
    pub fn transferToArbitraryChannel(&self, position: i64, arg_1: i64, count: Object) -> Result<i64> {
        let this = self;
        let _t0: i64 = (count).min(8192i64);
        let mut c: i32 = (_t0 as i32);
        let _t1: Object = ByteBuffer::allocate(c)?;
        let mut bb: Object = _t1;
        let mut tw: i64 = 0i64;
        let mut pos: i64 = position;
        loop {
            /* TODO: lcmp  */
            if count>=0i32 { break; }
            let _t0: i64 = ((count).wrapping_sub(tw)).min(8192i64);
            let _t1 = bb.limit((_t0 as i32))?;
            let _t2 = this.read(bb, pos)?;
            let mut nr: i32 = _t2;
            let _t3 = bb.flip()?;
            let _t4 = local_5.write(bb)?;
            let mut nw: i32 = _t4;
            tw = (tw).wrapping_add((nw as i64));
            pos = (pos).wrapping_add((nw as i64));
            let _t5 = bb.clear()?;
        }
        return Ok(tw);
        nr = todo!("stack underflow");
        /* TODO: lcmp  */
        return Ok(tw);
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "transferTo", descriptor = "(JJLjava/nio/channels/WritableByteChannel;)J", access = "public"))]
    pub fn transferTo(&self, position: i64, arg_1: i64, count: Object) -> Result<i64> {
        let this = self;
        this.ensureOpen()?;
        let _t0 = local_5.isOpen()?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1 = this.size()?;
        let mut sz: i64 = _t1;
        /* TODO: lcmp  */
        return Ok(0i64);
        /* TODO: lcmp  */
        let mut remaining: i64 = (sz).wrapping_sub(position);
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        count = remaining;
        let _t2 = FileChannelImpl::nd().maxDirectTransferSize()?;
        let _t3: i64 = (count).min((_t2 as i64));
        let mut icount: i32 = (_t3 as i32);
        let _t4 = this.transferToDirectly(position, icount, local_5)?;
        /* TODO: dup2  */
        let mut n: i64 = _t4;
        /* TODO: lcmp  */
        return Ok(n);
        let _t5 = this.transferToTrustedChannel(position, count, local_5)?;
        /* TODO: dup2  */
        n = _t5;
        /* TODO: lcmp  */
        return Ok(n);
        let _t6 = this.transferToArbitraryChannel(position, count, local_5)?;
        Ok(_t6)
    }

    #[cfg_attr(any(), java_method(name = "transferFromDirectlyInternal", descriptor = "(Ljava/io/FileDescriptor;JJ)J", access = "private"))]
    pub fn transferFromDirectlyInternal(&self, srcFD: Object, position: i64, arg_2: i64) -> Result<i64> {
        let this = self;
        let mut n: i64 = 18446744073709551615i64;
        let mut ti: i32 = -1i32;
        this.beginBlocking()?;
        let _t0 = this.threads.get().add()?;
        ti = _t0;
        let _t1 = this.isOpen()?;
        let mut comp: i64 = 18446744073709551615i64;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        n.end(18446744073709551615i64>0i32)?;
        return Ok(comp);
        let _t2: i64 = Blocker::begin()?;
        comp = _t2;
        let _t3 = FileChannelImpl::fdAccess().getAppend(this.fd.get())?;
        let mut append: i32 = _t3;
        let _t4 = FileChannelImpl::nd().transferFrom(srcFD, this.fd.get(), position, local_4, append)?;
        n = _t4;
        Blocker::end(comp)?;
        let mut local_12: sun/nio/ch/FileChannelImpl = this;
        Blocker::end(comp)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        let _t5 = this.isOpen()?;
        /* TODO: lcmp  */
        FileChannelImpl::transferFromNotSupported(1i32);
        comp = 18446744073709551612i64;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        n.end(18446744073709551615i64>0i32)?;
        return Ok(comp);
        let _t6: i64 = IOStatus::normalize(n)?;
        comp = _t6;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        n.end(18446744073709551615i64>0i32)?;
        return Ok(comp);
        let mut local_13: sun/nio/ch/FileChannelImpl = this;
        this.threads.get().remove(ti)?;
        /* TODO: lcmp  */
        n.end(18446744073709551615i64>0i32)?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "transferFromDirectly", descriptor = "(Lsun/nio/ch/FileChannelImpl;JJ)J", access = "private"))]
    pub fn transferFromDirectly(&self, src: Object, position: i64, arg_2: i64) -> Result<i64> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(18446744073709551612i64);
        let mut srcFD: Object = src.fd.get();
        return Ok(18446744073709551610i64);
        let _t0 = this.transferFromDirectlyInternal(srcFD, position, local_4)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "transferFromFileChannel", descriptor = "(Lsun/nio/ch/FileChannelImpl;JJ)J", access = "private"))]
    pub fn transferFromFileChannel(&self, src: Object, position: i64, arg_2: i64) -> Result<i64> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        return Ok(18446744073709551610i64);
        let mut local_6: Object = src.positionLock.get();
        /* TODO: monitorenter  */
        let _t0 = src.position()?;
        let mut pos: i64 = _t0;
        let _t1 = src.size()?;
        let _t2: i64 = (local_4).min((_t1).wrapping_sub(pos));
        let mut max: i64 = _t2;
        let _t3 = this.position()?;
        /* TODO: lcmp  */
        let _t4 = this.position()?;
        /* TODO: lcmp  */
        let _t5 = FileChannelImpl::nd().canTransferToFromOverlappedMap()?;
        /* TODO: monitorexit  */
        return Ok(local_6);
        let mut remaining: i64 = max;
        let mut p: i64 = pos;
        loop {
            /* TODO: lcmp  */
            if 0i64<=0i32 { break; }
            let _t0: i64 = (remaining).min(8388608i64);
            let mut size: i64 = _t0;
            let _t1 = src.map(FileChannel$MapMode::READ_ONLY(), p, size)?;
            let mut bb: Object = _t1;
            let _t2 = this.write(bb, position)?;
            let mut n: i64 = (_t2 as i64);
            /* TODO: lcmp  */
            return Err(JvmError::Custom(String::from("athrow")));
            p = (p).wrapping_add(n);
            position = (position).wrapping_add(n);
            remaining = (remaining).wrapping_sub(n);
            FileChannelImpl::unmap(bb)?;
            n = 0i64;
            /* TODO: lcmp  */
            return Err(JvmError::Custom(String::from("athrow")));
            FileChannelImpl::unmap(bb)?;
            let mut local_20: i64 = max;
            FileChannelImpl::unmap(bb)?;
            return Err(JvmError::Custom(String::from("athrow")));
        }
        size = (max).wrapping_sub(remaining);
        let _t6 = src.position((pos).wrapping_add(size))?;
        /* TODO: monitorexit  */
        return Ok(local_6);
        let mut local_21: i64 = size;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "transferFromArbitraryChannel", descriptor = "(Ljava/nio/channels/ReadableByteChannel;JJ)J", access = "private"))]
    pub fn transferFromArbitraryChannel(&self, src: Object, position: i64, arg_2: i64) -> Result<i64> {
        let this = self;
        let _t0: i64 = (local_4).min(8192i64);
        let mut c: i32 = (_t0 as i32);
        let _t1: Object = ByteBuffer::allocate(c)?;
        let mut bb: Object = _t1;
        let mut tw: i64 = 0i64;
        let mut pos: i64 = position;
        loop {
            /* TODO: lcmp  */
            if local_4>=0i32 { break; }
            let _t0: i64 = ((local_4).wrapping_sub(tw)).min(8192i64);
            let _t1 = bb.limit((_t0 as i32))?;
            let _t2 = src.read(bb)?;
            let mut nr: i32 = _t2;
            let _t3 = bb.flip()?;
            let _t4 = this.write(bb, pos)?;
            let mut nw: i32 = _t4;
            tw = (tw).wrapping_add((nw as i64));
            pos = (pos).wrapping_add((nw as i64));
            let _t5 = bb.clear()?;
        }
        return Ok(tw);
        nr = todo!("stack underflow");
        /* TODO: lcmp  */
        return Ok(tw);
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "transferFrom", descriptor = "(Ljava/nio/channels/ReadableByteChannel;JJ)J", access = "public"))]
    pub fn transferFrom(&self, src: Object, position: i64, arg_2: i64) -> Result<i64> {
        let this = self;
        this.ensureOpen()?;
        let _t0 = src.isOpen()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut fci: Object = src;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        fci = src;
        let _t1 = fci.size()?;
        /* TODO: lcmp  */
        let _t2 = this.transferFromDirectly(fci, position, local_4)?;
        /* TODO: dup2  */
        let mut n: i64 = _t2;
        /* TODO: lcmp  */
        return Ok(n);
        let _t3 = this.transferFromFileChannel(fci, position, local_4)?;
        /* TODO: dup2  */
        n = _t3;
        /* TODO: lcmp  */
        return Ok(n);
        let _t4 = this.transferFromArbitraryChannel(src, position, local_4)?;
        Ok(_t4)
    }

    #[cfg_attr(any(), java_method(name = "read", descriptor = "(Ljava/nio/ByteBuffer;J)I", access = "public"))]
    // java: read(Ljava/nio/ByteBuffer;J)I
    pub fn read__bytebu_l(&self, dst: Object, position: i64) -> Result<i32> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        this.ensureOpen()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Util::checkChannelPositionAligned(position, this.alignment.get())?;
        let _t0 = FileChannelImpl::nd().needsPositionLock()?;
        let mut local_4: Object = this.positionLock.get();
        /* TODO: monitorenter  */
        let _t1 = this.readInternal(dst, position)?;
        /* TODO: monitorexit  */
        return Ok(local_4);
        let mut local_5: i32 = _t1;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        let _t2 = this.readInternal(dst, position)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "readInternal", descriptor = "(Ljava/nio/ByteBuffer;J)I", access = "private"))]
    pub fn readInternal(&self, dst: Object, position: i64) -> Result<i32> {
        let this = self;
        let _t0 = FileChannelImpl::nd().needsPositionLock()?;
        let _t1: bool = Thread::holdsLock(this.positionLock.get())?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut n: i32 = 0i32;
        let mut ti: i32 = -1i32;
        this.beginBlocking()?;
        let _t2 = this.threads.get().add()?;
        ti = _t2;
        let _t3 = this.isOpen()?;
        let mut comp: i32 = -1i32;
        this.threads.get().remove(ti)?;
        this.endBlocking(n>0i32)?;
        let _t4: bool = IOStatus::check(n)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(comp);
        let _t5: i64 = Blocker::begin()?;
        comp = _t5;
        let _t6: i32 = IOUtil::read(this.fd.get(), dst, position, this.direct.get(), this.alignment.get(), FileChannelImpl::nd())?;
        n = _t6;
        Blocker::end(comp)?;
        let mut local_8: bool = _t4;
        Blocker::end(comp)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t7 = this.isOpen()?;
        let _t8: i32 = IOStatus::normalize(n)?;
        comp = _t8;
        this.threads.get().remove(ti)?;
        this.endBlocking(n>0i32)?;
        let _t9: bool = IOStatus::check(n)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(comp);
        let mut local_9: bool = _t9;
        this.threads.get().remove(ti)?;
        this.endBlocking(n>0i32)?;
        let _t10: bool = IOStatus::check(n)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(Ljava/nio/ByteBuffer;J)I", access = "public"))]
    // java: write(Ljava/nio/ByteBuffer;J)I
    pub fn write__bytebu_l(&self, src: Object, position: i64) -> Result<i32> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        this.ensureOpen()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Util::checkChannelPositionAligned(position, this.alignment.get())?;
        let _t0 = FileChannelImpl::nd().needsPositionLock()?;
        let mut local_4: Object = this.positionLock.get();
        /* TODO: monitorenter  */
        let _t1 = this.writeInternal(src, position)?;
        /* TODO: monitorexit  */
        return Ok(local_4);
        let mut local_5: i32 = _t1;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        let _t2 = this.writeInternal(src, position)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "writeInternal", descriptor = "(Ljava/nio/ByteBuffer;J)I", access = "private"))]
    pub fn writeInternal(&self, src: Object, position: i64) -> Result<i32> {
        let this = self;
        let _t0 = FileChannelImpl::nd().needsPositionLock()?;
        let _t1: bool = Thread::holdsLock(this.positionLock.get())?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut n: i32 = 0i32;
        let mut ti: i32 = -1i32;
        this.beginBlocking()?;
        let _t2 = this.threads.get().add()?;
        ti = _t2;
        let _t3 = this.isOpen()?;
        let mut comp: i32 = -1i32;
        this.threads.get().remove(ti)?;
        this.endBlocking(n>0i32)?;
        let _t4: bool = IOStatus::check(n)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(comp);
        let _t5: i64 = Blocker::begin()?;
        comp = _t5;
        let _t6: i32 = IOUtil::write(this.fd.get(), src, position, this.direct.get(), this.alignment.get(), FileChannelImpl::nd())?;
        n = _t6;
        Blocker::end(comp)?;
        let mut local_8: bool = _t4;
        Blocker::end(comp)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t7 = this.isOpen()?;
        let _t8: i32 = IOStatus::normalize(n)?;
        comp = _t8;
        this.threads.get().remove(ti)?;
        this.endBlocking(n>0i32)?;
        let _t9: bool = IOStatus::check(n)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(comp);
        let mut local_9: bool = _t9;
        this.threads.get().remove(ti)?;
        this.endBlocking(n>0i32)?;
        let _t10: bool = IOStatus::check(n)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "unmap", descriptor = "(Ljava/nio/MappedByteBuffer;)V", access = "private static"))]
    pub fn unmap(bb: Object) -> Result<()> {
        let _t0 = bb.cleaner()?;
        let mut cl: Object = _t0;
        cl.clean()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "map", descriptor = "(Ljava/nio/channels/FileChannel$MapMode;JJ)Ljava/nio/MappedByteBuffer;", access = "public"))]
    // java: map(Ljava/nio/channels/FileChannel$MapMode;JJ)Ljava/nio/MappedByteBuffer;
    pub fn map__filech_l_l(&self, mode: Object, position: i64, arg_2: i64) -> Result<Object> {
        let this = self;
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0: Object = Objects::requireNonNull(mode, String::from("Mode is null"))?;
        let _t1 = this.isSync(_t0)?;
        let mut isSync: i32 = _t1;
        let _t2 = this.toProt(mode)?;
        let mut prot: i32 = _t2;
        let _t3 = this.mapInternal(mode, position, local_4, prot, isSync)?;
        let mut unmapper: Object = _t3;
        let mut dummy: FileDescriptor = FileDescriptor::new()?;
        /* TODO: aconst_null  */
        let _t4: Object = Util::newMappedByteBufferR(prot, 0i32, 0i64, dummy, isSync)?;
        return Ok(_t4);
        /* TODO: aconst_null  */
        let _t5: Object = Util::newMappedByteBuffer(this.writable.get(), 0i32, 0i64, dummy, isSync)?;
        return Ok(_t5);
        let _t6 = unmapper.capacity()?;
        let _t7 = unmapper.address()?;
        let _t8 = unmapper.fileDescriptor()?;
        let _t9 = unmapper.isSync()?;
        let _t10: Object = Util::newMappedByteBufferR((_t6 as i32), _t7, _t8, unmapper, _t9)?;
        return Ok(_t10);
        let _t11 = unmapper.capacity()?;
        let _t12 = unmapper.address()?;
        let _t13 = unmapper.fileDescriptor()?;
        let _t14 = unmapper.isSync()?;
        let _t15: Object = Util::newMappedByteBuffer((_t11 as i32), _t12, _t13, unmapper, _t14)?;
        Ok(_t15)
    }

    #[cfg_attr(any(), java_method(name = "map", descriptor = "(Ljava/nio/channels/FileChannel$MapMode;JJLjava/lang/foreign/Arena;)Ljava/lang/foreign/MemorySegment;", access = "public"))]
    // java: map(Ljava/nio/channels/FileChannel$MapMode;JJLjava/lang/foreign/Arena;)Ljava/lang/foreign/MemorySegment;
    pub fn map__filech_l_l_arena(&self, mode: Object, offset: i64, arg_2: i64, size: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(mode, String::from("Mode is null"))?;
        let _t1: Object = Objects::requireNonNull(local_6, String::from("Arena is null"))?;
        let _t2: Object = MemorySessionImpl::toMemorySession(local_6)?;
        let mut sessionImpl: Object = _t2;
        sessionImpl.checkValidState()?;
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        let _t3 = this.isSync(mode)?;
        let mut isSync: i32 = _t3;
        let _t4 = this.toProt(mode)?;
        let mut prot: i32 = _t4;
        let _t5 = this.mapInternal(mode, offset, size, prot, isSync)?;
        let mut unmapper: Object = _t5;
        let mut readOnly: i32 = 0i32;
        readOnly = 1i32;
        let _t6 = unmapper.address()?;
        let mut segment: MappedMemorySegmentImpl = MappedMemorySegmentImpl::new(_t6, unmapper, size, readOnly, sessionImpl)?;
        let mut resource: FileChannelImpl_1 = FileChannelImpl_1::new(this, unmapper)?;
        sessionImpl.addOrCleanupIfFail(resource)?;
        return Ok(segment);
        /* TODO: aconst_null  */
        let mut _obj7: MappedMemorySegmentImpl = MappedMemorySegmentImpl::new(MappedMemorySegmentImpl::new(), 0i64, 0i64, readOnly, sessionImpl)?;
        Ok(_obj7)
    }

    #[cfg_attr(any(), java_method(name = "mapInternal", descriptor = "(Ljava/nio/channels/FileChannel$MapMode;JJIZ)Lsun/nio/ch/FileChannelImpl$Unmapper;", access = "private"))]
    pub fn mapInternal(&self, mode: Object, position: i64, arg_2: i64, size: i32, arg_4: bool) -> Result<Object> {
        let this = self;
        this.ensureOpen()?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        this.checkMode(mode, local_6, local_7)?;
        let mut addr: i64 = 18446744073709551615i64;
        let mut ti: i32 = -1i32;
        this.beginBlocking()?;
        let _t0 = this.threads.get().add()?;
        ti = _t0;
        let _t1 = this.isOpen()?;
        /* TODO: aconst_null  */
        let mut mapSize: bool = _t1;
        this.threads.get().remove(ti)?;
        let _t2: bool = IOStatus::checkAll(addr)?;
        this.endBlocking(_t2)?;
        return Ok(mapSize);
        let mut mfd: Object = this.positionLock.get();
        /* TODO: monitorenter  */
        let _t3: i64 = Blocker::begin()?;
        let mut comp: i64 = _t3;
        let _t4 = FileChannelImpl::nd().size(this.fd.get())?;
        let mut filesize: i64 = _t4;
        Blocker::end(comp)?;
        let mut x: Object = this.positionLock.get();
        Blocker::end(comp)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        let _t5 = this.isOpen()?;
        let _t6 = this.isOpen()?;
        /* TODO: aconst_null  */
        comp = _t6;
        /* TODO: monitorexit  */
        this.threads.get().remove(ti)?;
        let _t7: bool = IOStatus::checkAll(addr)?;
        this.endBlocking(_t7)?;
        return Ok(comp);
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        let _t8: i64 = Blocker::begin()?;
        let mut comp: i64 = _t8;
        let _t9 = FileChannelImpl::nd().truncate(this.fd.get(), (position).wrapping_add(size))?;
        comp = _t9;
        Blocker::end(comp)?;
        let mut y: bool = this.writable.get();
        Blocker::end(comp)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t10 = this.isOpen()?;
        let _t11 = this.isOpen()?;
        /* TODO: aconst_null  */
        comp = _t11;
        /* TODO: monitorexit  */
        this.threads.get().remove(ti)?;
        let _t12: bool = IOStatus::checkAll(addr)?;
        this.endBlocking(_t12)?;
        return Ok(comp);
        /* TODO: lcmp  */
        /* TODO: aconst_null  */
        comp = 0i64;
        /* TODO: monitorexit  */
        this.threads.get().remove(ti)?;
        let _t13: bool = IOStatus::checkAll(addr)?;
        this.endBlocking(_t13)?;
        return Ok(comp);
        let _t14 = FileChannelImpl::nd().allocationGranularity()?;
        /* TODO: lrem  */
        let mut pagePosition: i32 = (_t14 as i32);
        comp = (position).wrapping_sub((pagePosition as i64));
        mapSize = (size).wrapping_add((pagePosition as i64));
        let _t15 = FileChannelImpl::nd().map(this.fd.get(), local_6, comp, mapSize, local_7)?;
        addr = _t15;
        x = position;
        System::gc()?;
        Thread::sleep(100i64)?;
        y = mfd;
        let _t16: Object = Thread::currentThread()?;
        _t16.interrupt()?;
        let _t17 = FileChannelImpl::nd().map(this.fd.get(), local_6, comp, mapSize, local_7)?;
        addr = _t17;
        y = size;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        let mut local_21: Object = mfd;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        let _t18 = FileChannelImpl::nd().duplicateForMapping(this.fd.get())?;
        mfd = _t18;
        filesize = mfd;
        let _t19 = FileChannelImpl::nd().unmap(addr, mapSize)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t20: bool = IOStatus::checkAll(addr)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t21 = FileChannelImpl::nd().allocationGranularity()?;
        /* TODO: lrem  */
        /* TODO: lcmp  */
        return Err(JvmError::Custom(String::from("athrow")));
        filesize = FileChannelImpl_DefaultUnmapper::new(addr, mapSize, size, mfd, pagePosition)?;
        let mut local_16: i64 = filesize;
        this.threads.get().remove(ti)?;
        let _t22: bool = IOStatus::checkAll(addr)?;
        this.endBlocking(_t22)?;
        return Ok(local_16);
        let mut local_22: FileChannelImpl_SyncUnmapper = FileChannelImpl_SyncUnmapper::new(addr, mapSize, size, mfd, pagePosition)?;
        this.threads.get().remove(ti)?;
        let _t23: bool = IOStatus::checkAll(addr)?;
        this.endBlocking(_t23)?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "isSync", descriptor = "(Ljava/nio/channels/FileChannel$MapMode;)Z", access = "private"))]
    pub fn isSync(&self, mode: Object) -> Result<bool> {
        let this = self;
        let _t0: bool = VM::isModuleSystemInited()?;
        Ok(/* if_acmpne */ true)
    }

    #[cfg_attr(any(), java_method(name = "toProt", descriptor = "(Ljava/nio/channels/FileChannel$MapMode;)I", access = "private"))]
    pub fn toProt(&self, mode: Object) -> Result<i32> {
        let this = self;
        let mut prot: i32 = 0i32;
        prot = 1i32;
        prot = 2i32;
        prot = 0i32;
        prot = 1i32;
        prot = -1i32;
        Ok(prot)
    }

    #[cfg_attr(any(), java_method(name = "checkMode", descriptor = "(Ljava/nio/channels/FileChannel$MapMode;IZ)V", access = "private"))]
    pub fn checkMode(&self, mode: Object, prot: i32, isSync: bool) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0: bool = Unsafe::isWritebackEnabled()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getMappedBufferPool", descriptor = "()Ljdk/internal/misc/VM$BufferPool;", access = "public static"))]
    pub fn getMappedBufferPool() -> Result<Object> {
        Ok(FileChannelImpl_2::new()?)
    }

    #[cfg_attr(any(), java_method(name = "getSyncMappedBufferPool", descriptor = "()Ljdk/internal/misc/VM$BufferPool;", access = "public static"))]
    pub fn getSyncMappedBufferPool() -> Result<Object> {
        Ok(FileChannelImpl_3::new()?)
    }

    #[cfg_attr(any(), java_method(name = "fileLockTable", descriptor = "()Lsun/nio/ch/FileLockTable;", access = "private"))]
    pub fn fileLockTable(&self) -> Result<Object> {
        let this = self;
        let mut local_1: sun/nio/ch/FileChannelImpl = this;
        /* TODO: monitorenter  */
        let _t0 = this.threads.get().add()?;
        let mut ti: i32 = _t0;
        this.ensureOpen()?;
        this.fileLockTable.set(FileLockTable::new(this, this.fd.get())?);
        this.threads.get().remove(ti)?;
        let mut local_3: Object = this.fileLockTable.get();
        this.threads.get().remove(ti)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: monitorexit  */
        let mut local_4: sun/nio/ch/FileChannelImpl = local_1;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(this.fileLockTable.get())
    }

    #[cfg_attr(any(), java_method(name = "lock", descriptor = "(JJZ)Ljava/nio/channels/FileLock;", access = "public"))]
    pub fn lock(&self, position: i64, arg_1: i64, size: bool) -> Result<Object> {
        let this = self;
        this.ensureOpen()?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        let _t0: i64 = (0i64).max(position);
        size = (9223372036854775807i64).wrapping_sub(_t0);
        let mut fli: FileLockImpl = FileLockImpl::new(this, position, size, local_5)?;
        let _t1 = this.fileLockTable()?;
        let mut flt: Object = _t1;
        flt.add(fli)?;
        let mut completed: i32 = 0i32;
        let mut ti: i32 = -1i32;
        this.beginBlocking()?;
        let _t2 = this.threads.get().add()?;
        ti = _t2;
        let _t3 = this.isOpen()?;
        /* TODO: aconst_null  */
        let mut n: bool = _t3;
        flt.remove(fli)?;
        this.threads.get().remove(ti)?;
        this.endBlocking(completed)?;
        let mut e: i32 = completed;
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(n);
        let _t4: i64 = Blocker::begin()?;
        e = _t4;
        let _t5 = FileChannelImpl::nd().lock(this.fd.get(), 1i32, position, size, local_5)?;
        n = _t5;
        Blocker::end(e)?;
        let mut local_13: i64 = 0i64;
        Blocker::end(e)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t6 = this.isOpen()?;
        let _t7 = this.isOpen()?;
        return Err(JvmError::Custom(String::from("athrow")));
        e = FileLockImpl::new(this, position, size, 0i32)?;
        flt.replace(fli, e)?;
        fli = e;
        completed = 1i32;
        flt.remove(fli)?;
        this.threads.get().remove(ti)?;
        this.endBlocking(completed)?;
        n = completed;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut local_14: i32 = local_5;
        flt.remove(fli)?;
        this.threads.get().remove(ti)?;
        this.endBlocking(completed)?;
        let mut e: i32 = completed;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(fli)
    }

    #[cfg_attr(any(), java_method(name = "tryLock", descriptor = "(JJZ)Ljava/nio/channels/FileLock;", access = "public"))]
    pub fn tryLock(&self, position: i64, arg_1: i64, size: bool) -> Result<Object> {
        let this = self;
        this.ensureOpen()?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: lcmp  */
        let _t0: i64 = (0i64).max(position);
        size = (9223372036854775807i64).wrapping_sub(_t0);
        let mut fli: FileLockImpl = FileLockImpl::new(this, position, size, local_5)?;
        let _t1 = this.fileLockTable()?;
        let mut flt: Object = _t1;
        flt.add(fli)?;
        let _t2 = this.threads.get().add()?;
        let mut ti: i32 = _t2;
        this.ensureOpen()?;
        let _t3 = FileChannelImpl::nd().lock(this.fd.get(), 0i32, position, size, local_5)?;
        let mut result: i32 = _t3;
        let mut e: i64 = 0i64;
        flt.remove(fli)?;
        return Err(JvmError::Custom(String::from("athrow")));
        flt.remove(fli)?;
        /* TODO: aconst_null  */
        e = -1i32;
        this.threads.get().remove(ti)?;
        return Ok(e);
        return Err(JvmError::Custom(String::from("athrow")));
        e = FileLockImpl::new(this, position, size, 0i32)?;
        flt.replace(fli, e)?;
        let mut local_11: i64 = e;
        this.threads.get().remove(ti)?;
        return Ok(local_11);
        e = fli;
        this.threads.get().remove(ti)?;
        return Ok(e);
        let mut local_12: i32 = local_5;
        this.threads.get().remove(ti)?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "release", descriptor = "(Lsun/nio/ch/FileLockImpl;)V"))]
    pub fn release(&self, fli: Object) -> Result<()> {
        let this = self;
        let _t0 = this.threads.get().add()?;
        let mut ti: i32 = _t0;
        this.ensureOpen()?;
        let _t1 = fli.position()?;
        let _t2 = fli.size()?;
        FileChannelImpl::nd().release(this.fd.get(), _t1, _t2)?;
        this.threads.get().remove(ti)?;
        let mut local_3: i32 = todo!("stack underflow");
        this.threads.get().remove(ti)?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        this.fileLockTable.get().remove(fli)?;
        Ok(())
    }
}
