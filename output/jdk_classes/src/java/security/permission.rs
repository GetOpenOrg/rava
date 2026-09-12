#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/security/Permission",
    super_class = "java/lang/Object",
    interfaces  = "java/security/Guard,java/io/Serializable",
    access      = "public abstract",
    source      = "Permission.java",
))]
pub struct Permission {
    #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub name: Field<String>,
}

impl Permission {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn new(name: String) -> Result<Self> {
        let this = Self { name: Field::new(String::new()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.name.set(name);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "checkGuard", descriptor = "(Ljava/lang/Object;)V", access = "public"))]
    pub fn checkGuard(&self, object: Object) -> Result<()> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(this)?;
        Ok(())
    }

    #[cfg_attr(any(), java_native(name = "implies", descriptor = "(Ljava/security/Permission;)Z", access = "public abstract"))]
    pub fn implies(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/security/Permission.implies")
    }

    #[cfg_attr(any(), java_native(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public abstract"))]
    pub fn equals(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/security/Permission.equals")
    }

    #[cfg_attr(any(), java_native(name = "hashCode", descriptor = "()I", access = "public abstract"))]
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/security/Permission.hashCode")
    }

    #[cfg_attr(any(), java_method(name = "getName", descriptor = "()Ljava/lang/String;", access = "public final"))]
    pub fn getName(&self) -> Result<String> {
        let this = self;
        Ok(this.name.get())
    }

    #[cfg_attr(any(), java_native(name = "getActions", descriptor = "()Ljava/lang/String;", access = "public abstract"))]
    pub fn getActions(&self) -> Result<String> {
        todo!("abstract java/security/Permission.getActions")
    }

    #[cfg_attr(any(), java_method(name = "newPermissionCollection", descriptor = "()Ljava/security/PermissionCollection;", access = "public"))]
    pub fn newPermissionCollection(&self) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.getActions()?;
        let mut actions: String = _t0;
        let _t1 = actions.isEmpty()?;
        String::new().append(&String::from("(""))?;
        let _t2 = this.getClass()?;
        let _t3 = _t2.getName()?;
        String::new().append(&_t3)?;
        String::new().append(&String::from("" ""))?;
        String::new().append(&this.name.get())?;
        String::new().append(&String::from("")"))?;
        return Ok(String::new());
        String::new().append(&String::from("(""))?;
        let _t4 = this.getClass()?;
        let _t5 = _t4.getName()?;
        String::new().append(&_t5)?;
        String::new().append(&String::from("" ""))?;
        String::new().append(&this.name.get())?;
        String::new().append(&String::from("" ""))?;
        String::new().append(&actions)?;
        String::new().append(&String::from("")"))?;
        Ok(String::new())
    }
}
