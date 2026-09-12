#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/nio/charset/Charset$1",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Iterator",
    access      = "",
    source      = "Charset.java",
))]
pub struct Charset_1 {
    #[cfg_attr(any(), java_field(name = "cl", descriptor = "Ljava/lang/ClassLoader;"))]
    pub cl: Field<Object>,
    #[cfg_attr(any(), java_field(name = "sl", descriptor = "Ljava/util/ServiceLoader;"))]
    pub sl: Field<Object>,
    #[cfg_attr(any(), java_field(name = "i", descriptor = "Ljava/util/Iterator;"))]
    pub i: Field<Object>,
    #[cfg_attr(any(), java_field(name = "next", descriptor = "Ljava/nio/charset/spi/CharsetProvider;"))]
    pub next: Field<Object>,
}

impl Charset_1 {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V"))]
    pub fn new() -> Result<Self> {
        let this = Self { cl: Field::new(Default::default()), sl: Field::new(Default::default()), i: Field::new(Default::default()), next: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        let _t0: Object = ClassLoader::getSystemClassLoader()?;
        this.cl.set(_t0);
        let _t1: Object = ServiceLoader::load(19i32, this.cl.get())?;
        this.sl.set(_t1);
        let _t2 = this.sl.get().iterator()?;
        this.i.set(_t2);
        /* TODO: aconst_null  */
        todo!("stack underflow").next.set(this);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "getNext", descriptor = "()Z", access = "private"))]
    pub fn getNext(&self) -> Result<bool> {
        let this = self;
        loop {
            if !this.next.get().is_none() { break; }
            let _t0 = this.i.get().hasNext()?;
            return Ok(0i32);
            let _t1 = this.i.get().next()?;
            this.next.set(_t1);
            let mut sce: bool = _t0;
            let _t2 = sce.getCause()?;
        }
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(1i32)
    }

    #[cfg_attr(any(), java_method(name = "hasNext", descriptor = "()Z", access = "public"))]
    pub fn hasNext(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.getNext()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "next", descriptor = "()Ljava/nio/charset/spi/CharsetProvider;", access = "public"))]
    pub fn next(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.getNext()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut n: Object = this.next.get();
        /* TODO: aconst_null  */
        _t0.next.set(this);
        Ok(n)
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "()V", access = "public"))]
    pub fn remove(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }
}
