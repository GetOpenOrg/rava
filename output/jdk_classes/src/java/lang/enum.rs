#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Enum",
    super_class = "java/lang/Object",
    interfaces  = "java/lang/constant/Constable,java/lang/Comparable,java/io/Serializable",
    access      = "public abstract",
    source      = "Enum.java",
))]
pub struct Enum<E> {
    #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub name: Field<String>,
    #[cfg_attr(any(), java_field(name = "ordinal", descriptor = "I", access = "private final"))]
    pub ordinal: Field<i32>,
    #[cfg_attr(any(), java_field(name = "hash", descriptor = "I", access = "private"))]
    pub hash: Field<i32>,
}

impl<E: Clone + 'static> Enum<E> {
    // java: name()Ljava/lang/String;
    pub fn name(&self) -> Result<String> {
        let this = self;
        Ok(this.name.get())
    }

    // java: ordinal()I
    pub fn ordinal(&self) -> Result<i32> {
        let this = self;
        Ok(this.ordinal.get())
    }

    // java: <init>(Ljava/lang/String;I)V
    pub fn new(name: String, ordinal: i32) -> Result<Self> {
        let this = Self { name: Field::new(String::new()), ordinal: Field::new(0), hash: Field::new(0) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.name.set(name);
        this.ordinal.set(ordinal);
        Ok(this)
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        Ok(this.name.get())
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, other: Object) -> Result<bool> {
        let this = self;
        Ok(/* if_acmpne */ true)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let mut hc: i32 = this.hash.get();
        let _t0: i32 = System::identityHashCode(this)?;
        this.hash.set(_t0);
        hc = _t0;
        Ok(hc)
    }

    // java: clone()Ljava/lang/Object;
    pub fn clone(&self) -> Result<Object> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    // java: compareTo(Ljava/lang/Enum;)I
    pub fn compareTo(&self, o: E) -> Result<i32> {
        let this = self;
        let mut other: E = o;
        let mut self_: Enum = this;
        let _t0 = self_.getClass()?;
        let _t1 = other.getClass()?;
        let _t2 = self_.getDeclaringClass()?;
        let _t3 = other.getDeclaringClass()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok((self_.ordinal.get()).wrapping_sub(other.ordinal.get()))
    }

    // java: getDeclaringClass()Ljava/lang/Class;
    pub fn getDeclaringClass(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.getClass()?;
        let mut clazz: Object = _t0;
        let _t1 = clazz.getSuperclass()?;
        let mut zuper: Object = _t1;
        Ok(zuper)
    }

    // java: describeConstable()Ljava/util/Optional;
    pub fn describeConstable(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.getDeclaringClass()?;
        let _t1 = _t0.describeConstable()?;
        /* TODO: invokedynamic 48 */
        let _t2 = _t1.map(this)?;
        Ok(_t2)
    }

    // java: valueOf(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;
    pub fn valueOf(enumClass: Object, name: String) -> Result<Object> {
        let _t0 = enumClass.enumConstantDirectory()?;
        let _t1 = _t0.get(name)?;
        let mut result: Object = _t1;
        return Ok(result);
        return Err(JvmError::Custom(String::from("athrow")));
        String::new().append(&String::from("No enum constant"))?;
        let _t2 = enumClass.getCanonicalName()?;
        String::new().append(&_t2)?;
        String::new().append(&String::from("."))?;
        String::new().append(&name)?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    // java: finalize()V
    pub fn finalize(&self) -> Result<()> {
        let this = self;
        Ok(())
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, in_: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    // java: readObjectNoData()V
    pub fn readObjectNoData(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }
}
