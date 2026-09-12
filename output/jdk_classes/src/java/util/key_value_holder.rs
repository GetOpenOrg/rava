#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/KeyValueHolder",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Map$Entry",
    access      = "final",
    source      = "KeyValueHolder.java",
))]
pub struct KeyValueHolder<K, V> {
    #[cfg_attr(any(), java_field(name = "key", descriptor = "Ljava/lang/Object;", access = "final"))]
    pub key: Field<Object>,
    #[cfg_attr(any(), java_field(name = "value", descriptor = "Ljava/lang/Object;", access = "final"))]
    pub value: Field<Object>,
}

impl<K: Clone + 'static, V: Clone + 'static> KeyValueHolder<K, V> {
    // java: <init>(Ljava/lang/Object;Ljava/lang/Object;)V
    pub fn new(k: K, v: V) -> Result<Self> {
        let this = Self { key: Field::new(Default::default()), value: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        let _t0: Object = Objects::requireNonNull(k)?;
        this.key.set(_t0);
        let _t1: Object = Objects::requireNonNull(v)?;
        this.value.set(_t1);
        Ok(this)
    }

    // java: getKey()Ljava/lang/Object;
    pub fn getKey(&self) -> Result<K> {
        let this = self;
        Ok(this.key.get())
    }

    // java: getValue()Ljava/lang/Object;
    pub fn getValue(&self) -> Result<V> {
        let this = self;
        Ok(this.value.get())
    }

    // java: setValue(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn setValue(&self, value: V) -> Result<V> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        let this = self;
        let mut e: Object = o;
        let _t0 = e.getKey()?;
        let _t1 = this.key.get().equals(_t0)?;
        let _t2 = e.getValue()?;
        let _t3 = this.value.get().equals(_t2)?;
        Ok(_t3!=0i32)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.key.get().hashCode()?;
        let _t1 = this.value.get().hashCode()?;
        Ok((_t0^_t1))
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        String::new().append(&this.key.get())?;
        String::new().append(&String::from("="))?;
        String::new().append(&this.value.get())?;
        Ok(String::new())
    }
}
