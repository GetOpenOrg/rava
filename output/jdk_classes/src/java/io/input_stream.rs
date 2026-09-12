#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/InputStream",
    super_class = "java/lang/Object",
    interfaces  = "java/io/Closeable",
    access      = "public abstract",
    source      = "InputStream.java",
))]
pub struct InputStream;

impl InputStream {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // java: nullInputStream()Ljava/io/InputStream;
    pub fn nullInputStream() -> Result<Object> {
        Ok(InputStream_1::new()?)
    }

    // java: read()I
    pub fn read(&self) -> Result<i32> {
        todo!("abstract java/io/InputStream.read")
    }

    // java: read([B)I
    // java: read([B)I
    pub fn read__arr_b(&self, b: Vec<i8>) -> Result<i32> {
        let this = self;
        let _t0 = this.read(b, 0i32, (b.len() as i32))?;
        Ok(_t0)
    }

    // java: read([BII)I
    // java: read([BII)I
    pub fn read__arr_b_i_i(&self, b: Vec<i8>, off: i32, len: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = Objects::checkFromIndexSize(off, len, (b.len() as i32))?;
        return Ok(0i32);
        let _t1 = this.read()?;
        let mut c: i32 = _t1;
        return Ok(-1i32);
        /* TODO: i2b  */
        b[off as usize] = c;
        let mut i: i32 = 1i32;
        loop {
            if i >= len { break; }
            let _t0 = this.read()?;
            c = _t0;
            /* TODO: i2b  */
            b[(off).wrapping_add(i) as usize] = c;
            i = i.wrapping_add(1i32);
        }
        let mut local_6: i32 = -1i32;
        Ok(i)
    }

    // java: readAllBytes()[B
    pub fn readAllBytes(&self) -> Result<Vec<i8>> {
        let this = self;
        let _t0 = this.readNBytes(2147483647i32)?;
        Ok(_t0)
    }

    // java: readNBytes(I)[B
    // java: readNBytes(I)[B
    pub fn readNBytes__i(&self, len: i32) -> Result<Vec<i8>> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        /* TODO: aconst_null  */
        let mut bufs: i32 = len;
        /* TODO: aconst_null  */
        let mut result: i32 = todo!("stack underflow");
        let mut total: i32 = 0i32;
        let mut remaining: i32 = len;
        let _t0: i32 = (remaining).min(16384i32);
        let mut _arr1: Vec<i8> = vec![0i8; _t0 as usize];
        let mut buf: Vec<i8> = _arr1;
        let mut nread: i32 = 0i32;
        loop {
            let _t0: i32 = (((buf.len() as i32)).wrapping_sub(nread)).min(remaining);
            let _t1 = this.read(buf, nread, _t0)?;
            let mut n: i32 = _t1;
            if _t1<=0i32 { break; }
            nread = (nread).wrapping_add(n);
            remaining = (remaining).wrapping_sub(n);
        }
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t2: Vec<i8> = Arrays::copyOfRange(&buf, 0i32, nread)?;
        buf = _t2;
        total = (total).wrapping_add(nread);
        result = buf;
        bufs = ArrayList::<_>::new()?;
        let _t3 = bufs.add(result)?;
        let _t4 = bufs.add(buf)?;
        let mut _arr5: Vec<i8> = vec![0i8; 0i32 as usize];
        return Ok(_arr5);
        let _t6: Vec<i8> = Arrays::copyOf(result, total)?;
        return Ok(_t6);
        let mut _arr7: Vec<i8> = vec![0i8; total as usize];
        result = _arr7;
        buf = 0i32;
        remaining = total;
        let _t8 = bufs.iterator()?;
        nread = _t8;
        loop {
            let _t0 = nread.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = nread.next()?;
            let mut b: Object = _t0;
            let _t1: i32 = ((b.len() as i32)).min(remaining);
            let mut count: i32 = _t1;
            System::arraycopy(b, 0i32, result, &buf, count)?;
            buf = (buf).wrapping_add(count);
            remaining = (remaining).wrapping_sub(count);
        }
        Ok(result)
    }

    // java: readNBytes([BII)I
    // java: readNBytes([BII)I
    pub fn readNBytes__arr_b_i_i(&self, b: Vec<i8>, off: i32, len: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = Objects::checkFromIndexSize(off, len, (b.len() as i32))?;
        let mut n: i32 = 0i32;
        loop {
            if n >= len { break; }
            let _t0 = this.read(b, (off).wrapping_add(n), (len).wrapping_sub(n))?;
            let mut count: i32 = _t0;
            n = (n).wrapping_add(count);
        }
        Ok(n)
    }

    // java: skip(J)J
    pub fn skip(&self, n: i64) -> Result<i64> {
        let this = self;
        let mut remaining: i64 = n;
        /* TODO: lcmp  */
        return Ok(0i64);
        let _t0: i64 = (2048i64).min(remaining);
        let mut size: i32 = (_t0 as i32);
        let mut _arr1: Vec<i8> = vec![0i8; size as usize];
        let mut skipBuffer: Vec<i8> = _arr1;
        loop {
            /* TODO: lcmp  */
            if 0i64<=0i32 { break; }
            let _t0: i64 = ((size as i64)).min(remaining);
            let _t1 = this.read(skipBuffer, 0i32, (_t0 as i32))?;
            let mut nr: i32 = _t1;
            remaining = (remaining).wrapping_sub((nr as i64));
        }
        Ok((n).wrapping_sub(remaining))
    }

    // java: skipNBytes(J)V
    pub fn skipNBytes(&self, n: i64) -> Result<()> {
        let this = self;
        loop {
            /* TODO: lcmp  */
            if 0i64<=0i32 { break; }
            let _t0 = this.skip(n)?;
            let mut ns: i64 = _t0;
            /* TODO: lcmp  */
            /* TODO: lcmp  */
            n = (n).wrapping_sub(ns);
            /* TODO: lcmp  */
            let _t1 = this.read()?;
            return Err(JvmError::Custom("athrow".to_owned()));
            n = (n).wrapping_sub(1i64);
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        Ok(())
    }

    // java: available()I
    pub fn available(&self) -> Result<i32> {
        let this = self;
        Ok(0i32)
    }

    // java: close()V
    pub fn close(&self) -> Result<()> {
        let this = self;
        Ok(())
    }

    // java: mark(I)V
    pub fn mark(&self, readlimit: i32) -> Result<()> {
        let this = self;
        Ok(())
    }

    // java: reset()V
    pub fn reset(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: markSupported()Z
    pub fn markSupported(&self) -> Result<bool> {
        let this = self;
        Ok(0i32)
    }

    // java: transferTo(Ljava/io/OutputStream;)J
    pub fn transferTo(&self, out: Object) -> Result<i64> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(out, String::from("out"))?;
        let mut transferred: i64 = 0i64;
        let mut _arr1: Vec<i8> = vec![0i8; 16384i32 as usize];
        let mut buffer: Vec<i8> = _arr1;
        loop {
            let _t0 = this.read(buffer, 0i32, 16384i32)?;
            let mut read: i32 = _t0;
            if _t0<0i32 { break; }
            out.write(buffer, 0i32, read)?;
            /* TODO: lcmp  */
            let _t0: i64 = (transferred).abs();
            transferred = _t0;
            let mut ignore: i64 = 9223372036854775807i64;
            transferred = 9223372036854775807i64;
        }
        Ok(transferred)
    }
}
