#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/PropertyPermission",
    super_class = "java/security/BasicPermission",
    interfaces  = "",
    access      = "public final",
    source      = "PropertyPermission.java",
))]
pub struct PropertyPermission {
    #[cfg_attr(any(), java_field(name = "mask", descriptor = "I", access = "private"))]
    pub mask: Field<i32>,
    #[cfg_attr(any(), java_field(name = "actions", descriptor = "Ljava/lang/String;", access = "private"))]
    pub actions: Field<String>,
}

impl PropertyPermission {
    // java: init(I)V
    pub fn init(&self, mask: i32) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = this.getName()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.mask.set(mask);
        Ok(())
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;Ljava/lang/String;)V
    pub fn new__str_str(name: String, actions: String) -> Result<Self> {
        let this = Self { mask: Field::new(0), actions: Field::new(String::new()) };
        /* invokespecial Method java/security/BasicPermission.<init>:(Ljava/lang/String;Ljava/lang/String;)V */
        let _t0: i32 = PropertyPermission::getMask(actions)?;
        this.init(_t0)?;
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;I)V
    // java: <init>(Ljava/lang/String;I)V
    pub fn new__str_i(name: String, mask: i32) -> Result<Self> {
        let this = Self { mask: Field::new(0), actions: Field::new(String::new()) };
        let _t0: String = PropertyPermission::getActions(mask)?;
        /* invokespecial Method java/security/BasicPermission.<init>:(Ljava/lang/String;Ljava/lang/String;)V */
        this.mask.set(mask);
        Ok(this)
    }

    // java: implies(Ljava/security/Permission;)Z
    pub fn implies(&self, p: Object) -> Result<bool> {
        let this = self;
        let mut that: Object = p;
        let _t0: bool = BasicPermission::implies(that)?;
        Ok(_t0!=0i32)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        let mut that: Object = obj;
        let _t0 = this.getName()?;
        let _t1 = that.getName()?;
        let _t2 = _t0.equals(_t1)?;
        Ok(_t2!=0i32)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.getName()?;
        let _t1 = _t0.hashCode()?;
        Ok(_t1)
    }

    // java: getMask(Ljava/lang/String;)I
    // java: getMask(Ljava/lang/String;)I
    pub fn getMask__str(actions: String) -> Result<i32> {
        let mut mask: i32 = 0i32;
        return Ok(mask);
        return Ok(1i32);
        return Ok(2i32);
        return Ok(3i32);
        let _t0 = actions.toCharArray()?;
        let mut a: Vec<u16> = _t0;
        let mut i: i32 = ((a.len() as i32)).wrapping_sub(1i32);
        return Ok(mask);
        loop {
            if i == -1i32 { break; }
            let mut c: i32 = a[i as usize];
            i = i.wrapping_sub(1i32);
            let mut matchlen: i32 = 4i32;
            mask = (mask|1i32);
            matchlen = 5i32;
            mask = (mask|2i32);
            String::new().append(&String::from("invalid permission:"))?;
            String::new().append(&actions)?;
            return Err(JvmError::Custom("athrow".to_owned()));
            let mut seencomma: i32 = 0i32;
            /* TODO: lookupswitch default:421 9:418 10:418 12:418 13:418 32:418 44:412 */
            seencomma = 1i32;
            String::new().append(&String::from("invalid permission:"))?;
            String::new().append(&actions)?;
            return Err(JvmError::Custom("athrow".to_owned()));
            i = i.wrapping_sub(1i32);
            i = (i).wrapping_sub(matchlen);
        }
        Ok(mask)
    }

    // java: getActions(I)Ljava/lang/String;
    // java: getActions(I)Ljava/lang/String;
    pub fn getActions__i(mask: i32) -> Result<String> {
        /* TODO: tableswitch default:43 low:1 high:3 */
        Ok(String::from(""))
    }

    // java: getActions()Ljava/lang/String;
    // java: getActions()Ljava/lang/String;
    pub fn getActions(&self) -> Result<String> {
        let this = self;
        let _t0: String = PropertyPermission::getActions(this.mask.get())?;
        this.actions.set(_t0);
        Ok(this.actions.get())
    }

    // java: getMask()I
    // java: getMask()I
    pub fn getMask(&self) -> Result<i32> {
        let this = self;
        Ok(this.mask.get())
    }

    // java: newPermissionCollection()Ljava/security/PermissionCollection;
    pub fn newPermissionCollection(&self) -> Result<Object> {
        let this = self;
        Ok(PropertyPermissionCollection::new()?)
    }

    // java: writeObject(Ljava/io/ObjectOutputStream;)V
    pub fn writeObject(&self, s: Object) -> Result<()> {
        let this = self;
        let _t0 = this.getActions()?;
        s.defaultWriteObject()?;
        Ok(())
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, s: Object) -> Result<()> {
        let this = self;
        s.defaultReadObject()?;
        let _t0: i32 = PropertyPermission::getMask(this.actions.get())?;
        this.init(_t0)?;
        Ok(())
    }
}
