#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/FileInputStream",
    super_class = "java/io/InputStream",
    interfaces  = "",
    access      = "public",
    source      = "FileInputStream.java",
))]
pub struct FileInputStream {
    #[cfg_attr(any(), java_field(name = "fd", descriptor = "Ljava/io/FileDescriptor;", access = "private final"))]
    pub fd: Field<Object>,
    #[cfg_attr(any(), java_field(name = "path", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub path: Field<String>,
    #[cfg_attr(any(), java_field(name = "channel", descriptor = "Ljava/nio/channels/FileChannel;", access = "private"))]
    pub channel: Field<Object>,
    #[cfg_attr(any(), java_field(name = "closeLock", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub closeLock: Field<Object>,
    #[cfg_attr(any(), java_field(name = "closed", descriptor = "Z", access = "private"))]
    pub closed: Field<bool>,
}

impl FileInputStream {
    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(name: String) -> Result<Self> {
        let this = Self { fd: Field::new(Default::default()), path: Field::new(String::new()), channel: Field::new(Default::default()), closeLock: Field::new(Default::default()), closed: Field::new(false) };
        /* TODO: aconst_null  */
        /* invokespecial Method java/io/FileInputStream.<init>:(Ljava/io/File;)V */
        Ok(this)
    }

    // java: <init>(Ljava/io/File;)V
    // java: <init>(Ljava/io/File;)V
    pub fn new__file(file: Object) -> Result<Self> {
        let this = Self { fd: Field::new(Default::default()), path: Field::new(String::new()), channel: Field::new(Default::default()), closeLock: Field::new(Default::default()), closed: Field::new(false) };
        /* invokespecial Method java/io/InputStream.<init>:()V */
        this.closeLock.set(Object::new()?);
        let _t0 = file.getPath()?;
        /* TODO: aconst_null  */
        let mut name: String = _t0;
        let _t1: Object = System::getSecurityManager()?;
        let mut security: Object = _t1;
        security.checkRead(name)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t2 = file.isInvalid()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.fd.set(FileDescriptor::new()?);
        this.fd.get().attach(this)?;
        this.path.set(name);
        this.open(name)?;
        FileCleanable::register(this.fd.get())?;
        Ok(this)
    }

    // java: <init>(Ljava/io/FileDescriptor;)V
    // java: <init>(Ljava/io/FileDescriptor;)V
    pub fn new__filede(fdObj: Object) -> Result<Self> {
        let this = Self { fd: Field::new(Default::default()), path: Field::new(String::new()), channel: Field::new(Default::default()), closeLock: Field::new(Default::default()), closed: Field::new(false) };
        /* invokespecial Method java/io/InputStream.<init>:()V */
        this.closeLock.set(Object::new()?);
        let _t0: Object = System::getSecurityManager()?;
        let mut security: Object = _t0;
        return Err(JvmError::Custom("athrow".to_owned()));
        security.checkRead(fdObj)?;
        this.fd.set(fdObj);
        /* TODO: aconst_null  */
        security.path.set(this);
        this.fd.get().attach(this)?;
        Ok(this)
    }

    // java: open0(Ljava/lang/String;)V
    pub fn open0(&self, arg0: String) -> Result<()> {
        todo!("native java/io/FileInputStream.open0")
    }

    // java: open(Ljava/lang/String;)V
    pub fn open(&self, name: String) -> Result<()> {
        let this = self;
        let _t0: i64 = Blocker::begin()?;
        let mut comp: i64 = _t0;
        this.open0(name)?;
        Blocker::end(comp)?;
        let mut local_4: i32 = todo!("stack underflow");
        Blocker::end(comp)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: read()I
    // java: read()I
    pub fn read(&self) -> Result<i32> {
        let this = self;
        let _t0: i64 = Blocker::begin()?;
        let mut comp: i64 = _t0;
        let _t1 = this.read0()?;
        let mut local_3: i32 = _t1;
        Blocker::end(comp)?;
        return Ok(local_3);
        let mut local_4: i32 = todo!("stack underflow");
        Blocker::end(comp)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: read0()I
    pub fn read0(&self) -> Result<i32> {
        todo!("native java/io/FileInputStream.read0")
    }

    // java: readBytes([BII)I
    pub fn readBytes(&self, arg0: Vec<i8>, arg1: i32, arg2: i32) -> Result<i32> {
        todo!("native java/io/FileInputStream.readBytes")
    }

    // java: read([B)I
    // java: read([B)I
    pub fn read__arr_b(&self, b: Vec<i8>) -> Result<i32> {
        let this = self;
        let _t0: i64 = Blocker::begin()?;
        let mut comp: i64 = _t0;
        let _t1 = this.readBytes(b, 0i32, (b.len() as i32))?;
        let mut local_4: i32 = _t1;
        Blocker::end(comp)?;
        return Ok(local_4);
        let mut local_5: i32 = todo!("stack underflow");
        Blocker::end(comp)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: read([BII)I
    // java: read([BII)I
    pub fn read__arr_b_i_i(&self, b: Vec<i8>, off: i32, len: i32) -> Result<i32> {
        let this = self;
        let _t0: i64 = Blocker::begin()?;
        let mut comp: i64 = _t0;
        let _t1 = this.readBytes(b, off, len)?;
        let mut local_6: i32 = _t1;
        Blocker::end(comp)?;
        return Ok(local_6);
        let mut local_7: i32 = todo!("stack underflow");
        Blocker::end(comp)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: readAllBytes()[B
    pub fn readAllBytes(&self) -> Result<Vec<i8>> {
        let this = self;
        let _t0 = this.length()?;
        let mut length: i64 = _t0;
        let _t1 = this.position()?;
        let mut position: i64 = _t1;
        let mut size: i64 = (length).wrapping_sub(position);
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let _t2: Vec<i8> = InputStream::readAllBytes()?;
        return Ok(_t2);
        /* TODO: lcmp  */
        let mut _arr3: Vec<Object> = Vec::with_capacity(4i32 as usize);
        _arr3[0i32 as usize] = this.path.get();
        _arr3[1i32 as usize] = size;
        _arr3[2i32 as usize] = length;
        _arr3[3i32 as usize] = position;
        let _t4: String = String::format(String::from("Required array size too large for %s: %d = %d - %d"), &_arr3)?;
        let mut msg: String = _t4;
        return Err(JvmError::Custom("athrow".to_owned()));
        msg = (size as i32);
        let mut _arr5: Vec<i8> = vec![0i8; msg as usize];
        let mut buf: Vec<i8> = _arr5;
        let mut nread: i32 = 0i32;
        loop {
            let _t0 = this.read(buf, nread, (msg).wrapping_sub(nread))?;
            let mut n: i32 = _t0;
            nread = (nread).wrapping_add(n);
            if n<0i32 { break; }
            let _t0 = this.read()?;
            n = _t0;
            let _t1: i32 = ArraysSupport::newLength(msg, 1i32, msg)?;
            let _t2: i32 = (_t1).max(8192i32);
            msg = _t2;
            let _t3: Vec<i8> = Arrays::copyOf(&buf, msg)?;
            buf = _t3;
            nread = nread.wrapping_add(1i32);
            /* TODO: i2b  */
            buf[nread as usize] = n;
        }
        let _t6: Vec<i8> = Arrays::copyOf(&buf, nread)?;
        Ok(_t6)
    }

    // java: readNBytes(I)[B
    pub fn readNBytes(&self, len: i32) -> Result<Vec<i8>> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut _arr0: Vec<i8> = vec![0i8; 0i32 as usize];
        return Ok(_arr0);
        let _t1 = this.length()?;
        let mut length: i64 = _t1;
        let _t2 = this.position()?;
        let mut position: i64 = _t2;
        let mut size: i64 = (length).wrapping_sub(position);
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let _t3: Vec<i8> = InputStream::readNBytes(len)?;
        return Ok(_t3);
        let _t4: i64 = ((len as i64)).min(size);
        let mut capacity: i32 = (_t4 as i32);
        let mut _arr5: Vec<i8> = vec![0i8; capacity as usize];
        let mut buf: Vec<i8> = _arr5;
        let mut remaining: i32 = capacity;
        let mut nread: i32 = 0i32;
        let _t6 = this.read(buf, nread, remaining)?;
        let mut n: i32 = _t6;
        nread = (nread).wrapping_add(n);
        remaining = (remaining).wrapping_sub(n);
        let _t7 = this.read()?;
        /* TODO: i2b  */
        let mut b: i32 = _t7;
        nread = nread.wrapping_add(1i32);
        buf[nread as usize] = b;
        remaining = remaining.wrapping_sub(1i32);
        let _t8: Vec<i8> = Arrays::copyOf(&buf, nread)?;
        Ok(_t8)
    }

    // java: transferTo(Ljava/io/OutputStream;)J
    pub fn transferTo(&self, out: Object) -> Result<i64> {
        let this = self;
        let mut transferred: i64 = 0i64;
        let mut fos: Object = out;
        let _t0 = this.getChannel()?;
        let mut fc: Object = _t0;
        let _t1 = fc.position()?;
        let mut pos: i64 = _t1;
        let _t2 = fos.getChannel()?;
        let _t3 = fc.transferTo(pos, 9223372036854775807i64, _t2)?;
        transferred = _t3;
        let mut newPos: i64 = (pos).wrapping_add(transferred);
        let _t4 = fc.position(newPos)?;
        let _t5 = fc.size()?;
        /* TODO: lcmp  */
        return Ok(transferred);
        let _t6: i64 = InputStream::transferTo(out)?;
        let _t7: i64 = (this).abs();
        return Ok(_t7);
        fos = transferred;
        Ok(9223372036854775807i64)
    }

    // java: length()J
    pub fn length(&self) -> Result<i64> {
        let this = self;
        let _t0: i64 = Blocker::begin()?;
        let mut comp: i64 = _t0;
        let _t1 = this.length0()?;
        let mut local_3: i64 = _t1;
        Blocker::end(comp)?;
        return Ok(local_3);
        let mut local_5: i32 = todo!("stack underflow");
        Blocker::end(comp)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: length0()J
    pub fn length0(&self) -> Result<i64> {
        todo!("native java/io/FileInputStream.length0")
    }

    // java: position()J
    pub fn position(&self) -> Result<i64> {
        let this = self;
        let _t0: i64 = Blocker::begin()?;
        let mut comp: i64 = _t0;
        let _t1 = this.position0()?;
        let mut local_3: i64 = _t1;
        Blocker::end(comp)?;
        return Ok(local_3);
        let mut local_5: i32 = todo!("stack underflow");
        Blocker::end(comp)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: position0()J
    pub fn position0(&self) -> Result<i64> {
        todo!("native java/io/FileInputStream.position0")
    }

    // java: skip(J)J
    pub fn skip(&self, n: i64) -> Result<i64> {
        let this = self;
        let _t0: i64 = Blocker::begin()?;
        let mut comp: i64 = _t0;
        let _t1 = this.skip0(n)?;
        let mut local_5: i64 = _t1;
        Blocker::end(comp)?;
        return Ok(local_5);
        let mut local_7: i32 = todo!("stack underflow");
        Blocker::end(comp)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: skip0(J)J
    pub fn skip0(&self, arg0: i64) -> Result<i64> {
        todo!("native java/io/FileInputStream.skip0")
    }

    // java: available()I
    pub fn available(&self) -> Result<i32> {
        let this = self;
        let _t0: i64 = Blocker::begin()?;
        let mut comp: i64 = _t0;
        let _t1 = this.available0()?;
        let mut local_3: i32 = _t1;
        Blocker::end(comp)?;
        return Ok(local_3);
        let mut local_4: i32 = todo!("stack underflow");
        Blocker::end(comp)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: available0()I
    pub fn available0(&self) -> Result<i32> {
        todo!("native java/io/FileInputStream.available0")
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
        this.fd.get().closeAll(FileInputStream_1::new(this)?)?;
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
        let mut local_2: FileInputStream = this;
        /* TODO: monitorenter  */
        fc = this.channel.get();
        let _t0: Object = FileChannelImpl::open(this.fd.get(), this.path.get(), 1i32, 0i32, 0i32, this)?;
        fc = _t0;
        this.channel.set(_t0);
        fc.close()?;
        let mut ioe: bool = this.closed.get();
        return Err(JvmError::Custom("athrow".to_owned()));
        /* TODO: monitorexit  */
        let mut local_4: FileInputStream = local_2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(fc)
    }

    // java: initIDs()V
    pub fn initIDs() -> Result<()> {
        todo!("native java/io/FileInputStream.initIDs")
    }
}
