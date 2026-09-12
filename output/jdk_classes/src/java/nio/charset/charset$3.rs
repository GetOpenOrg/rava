#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/nio/charset/Charset$3",
    super_class = "java/lang/Object",
    interfaces  = "java/security/PrivilegedAction",
    access      = "",
    source      = "Charset.java",
))]
pub struct Charset_3;

impl Charset_3 {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "run", descriptor = "()Ljava/util/SortedMap;", access = "public"))]
    pub fn run(&self) -> Result<Object> {
        let this = self;
        let mut m: TreeMap = TreeMap::new(String::CASE_INSENSITIVE_ORDER())?;
        let _t0 = Charset::standardProvider().charsets()?;
        Charset::put(_t0, m)?;
        let mut ecps: Vec<Object> = Charset$ExtendedProviderHolder::extendedProviders();
        let mut i: Vec<Object> = ecps;
        let mut cp: i32 = (i.len() as i32);
        let mut local_5: i32 = 0i32;
        loop {
            if local_5 >= cp { break; }
            let mut ecp: Object = i[local_5 as usize].clone();
            let _t0 = ecp.charsets()?;
            Charset::put(_t0, m)?;
            local_5 = local_5.wrapping_add(1i32);
        }
        let _t1: Object = Charset::providers()?;
        i = _t1;
        loop {
            let _t0 = i.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = i.next()?;
            cp = _t0;
            let _t1 = cp.charsets()?;
            Charset::put(_t1, m)?;
        }
        let _t2: Object = Collections::unmodifiableSortedMap(m)?;
        Ok(_t2)
    }
}
