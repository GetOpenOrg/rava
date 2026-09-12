#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/security/ProtectionDomain",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public",
    source      = "ProtectionDomain.java",
))]
pub struct ProtectionDomain {
    #[cfg_attr(any(), java_field(name = "codesource", descriptor = "Ljava/security/CodeSource;", access = "private final"))]
    pub codesource: Field<Object>,
    #[cfg_attr(any(), java_field(name = "classloader", descriptor = "Ljava/lang/ClassLoader;", access = "private final"))]
    pub classloader: Field<Object>,
    #[cfg_attr(any(), java_field(name = "principals", descriptor = "[Ljava/security/Principal;", access = "private final"))]
    pub principals: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "permissions", descriptor = "Ljava/security/PermissionCollection;", access = "private"))]
    pub permissions: Field<Object>,
    #[cfg_attr(any(), java_field(name = "hasAllPerm", descriptor = "Z", access = "private"))]
    pub hasAllPerm: Field<bool>,
    #[cfg_attr(any(), java_field(name = "staticPermissions", descriptor = "Z", access = "private final"))]
    pub staticPermissions: Field<bool>,
    #[cfg_attr(any(), java_field(name = "key", descriptor = "Ljava/security/ProtectionDomain$Key;", access = "final"))]
    pub key: Field<Object>,
}

impl ProtectionDomain {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/security/CodeSource;Ljava/security/PermissionCollection;)V", access = "public"))]
    // java: <init>(Ljava/security/CodeSource;Ljava/security/PermissionCollection;)V
    pub fn new__codeso_permis(codesource: Object, permissions: Object) -> Result<Self> {
        let this = Self { codesource: Field::new(Default::default()), classloader: Field::new(Default::default()), principals: Field::new(Default::default()), permissions: Field::new(Default::default()), hasAllPerm: Field::new(false), staticPermissions: Field::new(false), key: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.hasAllPerm.set(0i32);
        this.key.set(ProtectionDomain_Key::new()?);
        this.codesource.set(codesource);
        this.permissions.set(permissions);
        this.permissions.get().setReadOnly()?;
        this.hasAllPerm.set(1i32);
        /* TODO: aconst_null  */
        permissions.allPermission.get().classloader.set(this);
        let mut _arr0: Vec<Object> = Vec::with_capacity(0i32 as usize);
        this.principals.set(_arr0);
        this.staticPermissions.set(1i32);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/security/CodeSource;Ljava/security/PermissionCollection;Ljava/lang/ClassLoader;[Ljava/security/Principal;)V", access = "public"))]
    // java: <init>(Ljava/security/CodeSource;Ljava/security/PermissionCollection;Ljava/lang/ClassLoader;[Ljava/security/Principal;)V
    pub fn new__codeso_permis_classl_arr_pri(codesource: Object, permissions: Object, classloader: Object, principals: Vec<Object>) -> Result<Self> {
        let this = Self { codesource: Field::new(Default::default()), classloader: Field::new(Default::default()), principals: Field::new(Default::default()), permissions: Field::new(Default::default()), hasAllPerm: Field::new(false), staticPermissions: Field::new(false), key: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.hasAllPerm.set(0i32);
        this.key.set(ProtectionDomain_Key::new()?);
        this.codesource.set(codesource);
        this.permissions.set(permissions);
        this.permissions.get().setReadOnly()?;
        this.hasAllPerm.set(1i32);
        this.classloader.set(classloader);
        let _t0 = principals.clone()?;
        let mut _arr1: Vec<Object> = Vec::with_capacity(0i32 as usize);
        _t0.principals.set(_arr1);
        this.staticPermissions.set(0i32);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "getCodeSource", descriptor = "()Ljava/security/CodeSource;", access = "public final"))]
    pub fn getCodeSource(&self) -> Result<Object> {
        let this = self;
        Ok(this.codesource.get())
    }

    #[cfg_attr(any(), java_method(name = "getClassLoader", descriptor = "()Ljava/lang/ClassLoader;", access = "public final"))]
    pub fn getClassLoader(&self) -> Result<Object> {
        let this = self;
        Ok(this.classloader.get())
    }

    #[cfg_attr(any(), java_method(name = "getPrincipals", descriptor = "()[Ljava/security/Principal;", access = "public final"))]
    pub fn getPrincipals(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.principals.get().clone()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getPermissions", descriptor = "()Ljava/security/PermissionCollection;", access = "public final"))]
    pub fn getPermissions(&self) -> Result<Object> {
        let this = self;
        Ok(this.permissions.get())
    }

    #[cfg_attr(any(), java_method(name = "staticPermissionsOnly", descriptor = "()Z", access = "public final"))]
    pub fn staticPermissionsOnly(&self) -> Result<bool> {
        let this = self;
        Ok(this.staticPermissions.get())
    }

    #[cfg_attr(any(), java_method(name = "implies", descriptor = "(Ljava/security/Permission;)Z", access = "public"))]
    pub fn implies(&self, perm: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        let _t0: Object = Policy::getPolicyNoCheck()?;
        let _t1 = _t0.implies(this, perm)?;
        return Ok(1i32);
        let _t2 = this.permissions.get().implies(perm)?;
        return Ok(_t2);
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "impliesWithAltFilePerm", descriptor = "(Ljava/security/Permission;)Z"))]
    pub fn impliesWithAltFilePerm(&self, perm: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.getClass()?;
        let _t1 = this.implies(perm)?;
        return Ok(_t1);
        return Ok(1i32);
        /* TODO: aconst_null  */
        let mut p2: bool = this.hasAllPerm.get();
        let mut p2Calculated: i32 = 0i32;
        let _t2: Object = Policy::getPolicyNoCheck()?;
        let mut policy: Object = _t2;
        let _t3 = policy.implies(this, perm)?;
        return Ok(_t3);
        let _t4 = policy.implies(this, perm)?;
        return Ok(1i32);
        let _t5: Object = FilePermCompat::newPermUsingAltPath(perm)?;
        p2 = _t5;
        p2Calculated = 1i32;
        let _t6 = policy.implies(this, p2)?;
        return Ok(1i32);
        let _t7 = this.permissions.get().implies(perm)?;
        return Ok(1i32);
        let _t8: Object = FilePermCompat::newPermUsingAltPath(perm)?;
        p2 = _t8;
        let _t9 = this.permissions.get().implies(p2)?;
        return Ok(_t9);
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let mut pals: String = String::from("<no principals>");
        let mut palBuf: String = String::new();
        let mut i: i32 = 0i32;
        loop {
            if i >= (this.principals.get().len() as i32) { break; }
            let _t0 = this.principals.get()[i as usize].clone().getClass()?;
            let _t1 = _t0.getName()?;
            String::new().append(&_t1)?;
            String::new().append(&String::from("""))?;
            let _t2 = this.principals.get()[i as usize].clone().getName()?;
            String::new().append(&_t2)?;
            String::new().append(&String::from("""))?;
            palBuf.append(&String::new())?;
            palBuf.append(&String::from(","))?;
            palBuf.append(&String::from(")"))?;
            i = i.wrapping_add(1i32);
        }
        pals = palBuf;
        let _t0: bool = Policy::isSet()?;
        let _t1: bool = ProtectionDomain::seeAllp()?;
        let _t2 = this.mergePermissions()?;
        let _t3 = this.getPermissions()?;
        palBuf = _t3;
        String::new().append(&String::from("ProtectionDomain"))?;
        String::new().append(&this.codesource.get())?;
        String::new().append(&String::from(""))?;
        String::new().append(&this.classloader.get())?;
        String::new().append(&String::from(""))?;
        String::new().append(&pals)?;
        String::new().append(&String::from(""))?;
        String::new().append(&palBuf)?;
        String::new().append(&String::from(""))?;
        Ok(String::new())
    }

    #[cfg_attr(any(), java_method(name = "seeAllp", descriptor = "()Z", access = "private static"))]
    pub fn seeAllp() -> Result<bool> {
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        return Ok(1i32);
        let _t1 = sm.getClass()?;
        let _t2 = _t1.getClassLoader()?;
        let _t3: Object = Policy::getPolicyNoCheck()?;
        let _t4 = _t3.getClass()?;
        let _t5 = _t4.getClassLoader()?;
        return Ok(_t5.is_none());
        sm.checkPermission(SecurityConstants::GET_POLICY_PERMISSION())?;
        return Ok(1i32);
        let mut se: Object = _t2;
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "mergePermissions", descriptor = "()Ljava/security/PermissionCollection;", access = "private"))]
    pub fn mergePermissions(&self) -> Result<Object> {
        let this = self;
        return Ok(this.permissions.get());
        /* TODO: invokedynamic 174 */
        let _t0: Object = AccessController::doPrivileged(this)?;
        let mut perms: Object = _t0;
        let mut mergedPerms: Permissions = Permissions::new()?;
        let mut swag: i32 = 32i32;
        let mut vcap: i32 = 8i32;
        let mut pdVector: ArrayList<_> = ArrayList::<_>::new()?;
        let mut plVector: ArrayList<_> = ArrayList::<_>::new()?;
        let mut i: Object = this.permissions.get();
        /* TODO: monitorenter  */
        let _t1 = this.permissions.get().elements()?;
        let mut e: Object = _t1;
        loop {
            let _t0 = e.hasMoreElements()?;
            if _t0==0i32 { break; }
            let _t0 = e.nextElement()?;
            let _t1 = pdVector.add(_t0)?;
        }
        /* TODO: monitorexit  */
        let mut pdp: Object = i;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        i = perms;
        /* TODO: monitorenter  */
        let _t2 = perms.elements()?;
        e = _t2;
        loop {
            let _t0 = e.hasMoreElements()?;
            if _t0==0i32 { break; }
            let _t0 = e.nextElement()?;
            let _t1 = plVector.add(_t0)?;
            vcap = vcap.wrapping_add(1i32);
        }
        /* TODO: monitorexit  */
        let mut pdpClass: Object = i;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        i = this.permissions.get();
        /* TODO: monitorenter  */
        let _t3 = this.permissions.get().elements()?;
        e = _t3;
        loop {
            let _t0 = e.hasMoreElements()?;
            if _t0==0i32 { break; }
            let _t0 = e.nextElement()?;
            pdp = _t0;
            let _t1 = pdp.getClass()?;
            pdpClass = _t1;
            let _t2 = pdp.getActions()?;
            let mut pdpActions: String = _t2;
            let _t3 = pdp.getName()?;
            let mut pdpName: String = _t3;
            let mut i: i32 = 0i32;
            let _t4 = plVector.size()?;
            let _t5 = plVector.get(i)?;
            let mut pp: Object = _t5;
            let _t6 = pdpClass.isInstance(pp)?;
            let _t7 = pp.getName()?;
            let _t8 = pdpName.equals(_t7)?;
            let _t9 = pp.getActions()?;
            let _t10: bool = Objects::equals(pdpActions, _t9)?;
            let _t11 = plVector.remove(i)?;
            i = i.wrapping_add(1i32);
        }
        /* TODO: monitorexit  */
        let mut local_15: Object = i;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        let _t4 = plVector.size()?;
        i = (_t4).wrapping_sub(1i32);
        loop {
            if i<0i32 { break; }
            let _t0 = plVector.get(i)?;
            mergedPerms.add(_t0)?;
            i = i.wrapping_sub(1i32);
        }
        let _t5 = pdVector.size()?;
        i = (_t5).wrapping_sub(1i32);
        loop {
            if i<0i32 { break; }
            let _t0 = pdVector.get(i)?;
            mergedPerms.add(_t0)?;
            i = i.wrapping_sub(1i32);
        }
        Ok(mergedPerms)
    }
}
