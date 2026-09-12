#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/PropertyPermissionCollection",
    super_class = "java/security/PermissionCollection",
    interfaces  = "java/io/Serializable",
    access      = "final",
    source      = "PropertyPermission.java",
))]
pub struct PropertyPermissionCollection {
    #[cfg_attr(any(), java_field(name = "perms", descriptor = "Ljava/util/concurrent/ConcurrentHashMap;", access = "private"))]
    pub perms: Field<Object>,
    #[cfg_attr(any(), java_field(name = "all_allowed", descriptor = "Z", access = "private"))]
    pub all_allowed: Field<bool>,
}

impl PropertyPermissionCollection {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { perms: Field::new(Default::default()), all_allowed: Field::new(false) };
        /* invokespecial Method java/security/PermissionCollection.<init>:()V */
        this.perms.set(ConcurrentHashMap::new(32i32)?);
        this.all_allowed.set(0i32);
        Ok(this)
    }

    // java: add(Ljava/security/Permission;)V
    pub fn add(&self, permission: Object) -> Result<()> {
        let this = self;
        let mut pp: Object = permission;
        String::new().append(&String::from("invalid permission:"))?;
        String::new().append(&permission)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = this.isReadOnly()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1 = pp.getName()?;
        let mut propName: String = _t1;
        /* TODO: invokedynamic 57 */
        let _t2 = this.perms.get().merge(propName, pp, propName)?;
        let _t3 = propName.equals(String::from("*"))?;
        this.all_allowed.set(1i32);
        Ok(())
    }

    // java: implies(Ljava/security/Permission;)Z
    pub fn implies(&self, permission: Object) -> Result<bool> {
        let this = self;
        let mut pp: Object = permission;
        return Ok(0i32);
        let _t0 = pp.getMask()?;
        let mut desired: i32 = _t0;
        let mut effective: i32 = 0i32;
        let _t1 = this.perms.get().get(String::from("*"))?;
        let mut x: Object = _t1;
        let _t2 = x.getMask()?;
        effective = (effective|_t2);
        return Ok(1i32);
        let _t3 = pp.getName()?;
        let mut name: String = _t3;
        let _t4 = this.perms.get().get(name)?;
        x = _t4;
        let _t5 = x.getMask()?;
        effective = (effective|_t5);
        return Ok(1i32);
        let _t6 = name.length()?;
        let mut offset: i32 = (_t6).wrapping_sub(1i32);
        loop {
            let _t0 = name.lastIndexOf(46i32, offset)?;
            let mut last: i32 = _t0;
            if _t0 == -1i32 { break; }
            let _t0 = name.substring(0i32, (last).wrapping_add(1i32))?;
            String::new().append(&_t0)?;
            String::new().append(&String::from("*"))?;
            name = String::new();
            let _t1 = this.perms.get().get(name)?;
            x = _t1;
            let _t2 = x.getMask()?;
            effective = (effective|_t2);
            return Ok(1i32);
            offset = (last).wrapping_sub(1i32);
        }
        Ok(0i32)
    }

    // java: elements()Ljava/util/Enumeration;
    pub fn elements(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.perms.get().elements()?;
        Ok(_t0)
    }

    // java: writeObject(Ljava/io/ObjectOutputStream;)V
    pub fn writeObject(&self, out: Object) -> Result<()> {
        let this = self;
        let _t0 = this.perms.get().size()?;
        let mut permissions: Hashtable = Hashtable::new((_t0).wrapping_mul(2i32))?;
        permissions.putAll(this.perms.get())?;
        let _t1 = out.putFields()?;
        let mut pfields: Object = _t1;
        pfields.put(String::from("all_allowed"), this.all_allowed.get())?;
        pfields.put(String::from("permissions"), permissions)?;
        out.writeFields()?;
        Ok(())
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, in_: Object) -> Result<()> {
        let this = self;
        let _t0 = in_.readFields()?;
        let mut gfields: Object = _t0;
        let _t1 = gfields.get(String::from("all_allowed"), 0i32)?;
        this.all_allowed.set(_t1);
        /* TODO: aconst_null  */
        let _t2 = todo!("stack underflow").get(gfields, String::from("permissions"))?;
        let mut permissions: Object = _t2;
        let _t3 = permissions.size()?;
        this.perms.set(ConcurrentHashMap::new((_t3).wrapping_mul(2i32))?);
        this.perms.get().putAll(permissions)?;
        Ok(())
    }
}
