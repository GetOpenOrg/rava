#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/nio/charset/Charset$2",
    super_class = "java/lang/Object",
    interfaces  = "java/security/PrivilegedAction",
    access      = "",
    source      = "Charset.java",
))]
pub struct Charset_2 {
    #[cfg_attr(any(), java_field(name = "val$charsetName", descriptor = "Ljava/lang/String;", access = "final"))]
    pub val$charsetName: Field<String>,
}

impl Charset_2 {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V"))]
    pub fn new(arg_0: String) -> Result<Self> {
        let this = Self { val$charsetName: Field::new(String::new()) };
        this.val$charsetName.set(arg_0);
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "run", descriptor = "()Ljava/nio/charset/Charset;", access = "public"))]
    pub fn run(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Charset::providers()?;
        let mut i: Object = _t0;
        loop {
            let _t0 = i.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = i.next()?;
            let mut cp: Object = _t0;
            let _t1 = cp.charsetForName(this.val$charsetName.get())?;
            let mut cs: Object = _t1;
            return Ok(cs);
        }
        /* TODO: aconst_null  */
        Ok(todo!("stack underflow"))
    }
}
