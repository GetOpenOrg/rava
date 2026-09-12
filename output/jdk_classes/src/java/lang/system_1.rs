#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/System$1",
    super_class = "java/lang/Object",
    interfaces  = "java/security/PrivilegedAction",
    access      = "",
    source      = "System.java",
))]
pub struct System_1 {
    #[cfg_attr(any(), java_field(name = "val$s", descriptor = "Ljava/lang/SecurityManager;", access = "final"))]
    pub val_s: Field<Object>,
}

impl System_1 {
    // java: <init>(Ljava/lang/SecurityManager;)V
    pub fn new(arg_0: Object) -> Result<Self> {
        let this = Self { val_s: Field::new(Default::default()) };
        this.val_s.set(arg_0);
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // java: run()Ljava/lang/Object;
    pub fn run(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.val_s.get().getClass()?;
        let _t1 = _t0.getProtectionDomain()?;
        let _t2 = _t1.implies(SecurityConstants::ALL_PERMISSION())?;
        /* TODO: aconst_null  */
        Ok(todo!("stack underflow"))
    }
}
