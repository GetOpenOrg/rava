#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$Map1",
    super_class = "java/util/ImmutableCollections$AbstractImmutableMap",
    interfaces  = "",
    access      = "final",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_Map1<K, V> {
    #[cfg_attr(any(), java_field(name = "k0", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub k0: Field<Object>,
    #[cfg_attr(any(), java_field(name = "v0", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub v0: Field<Object>,
}

impl<K: Clone + 'static, V: Clone + 'static> ImmutableCollections_Map1<K, V> {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)V"))]
    pub fn new(k0: K, v0: V) -> Result<Self> {
        let this = Self { k0: Field::new(Default::default()), v0: Field::new(Default::default()) };
        /* invokespecial Method java/util/ImmutableCollections$AbstractImmutableMap.<init>:()V */
        let _t0: Object = Objects::requireNonNull(k0)?;
        this.k0.set(_t0);
        let _t1: Object = Objects::requireNonNull(v0)?;
        this.v0.set(_t1);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "entrySet", descriptor = "()Ljava/util/Set;", access = "public"))]
    pub fn entrySet(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Set::of(KeyValueHolder::new(this.k0.get(), this.v0.get())?)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn get(&self, o: Object) -> Result<V> {
        let this = self;
        let _t0 = o.equals(this.k0.get())?;
        /* TODO: aconst_null  */
        Ok(this.v0.get())
    }

    #[cfg_attr(any(), java_method(name = "containsKey", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn containsKey(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = o.equals(this.k0.get())?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "containsValue", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn containsValue(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = o.equals(this.v0.get())?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "size", descriptor = "()I", access = "public"))]
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok(1i32)
    }

    #[cfg_attr(any(), java_method(name = "isEmpty", descriptor = "()Z", access = "public"))]
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private"))]
    pub fn readObject(&self, in_: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeReplace", descriptor = "()Ljava/lang/Object;", access = "private"))]
    pub fn writeReplace(&self) -> Result<Object> {
        let this = self;
        let mut _arr0: Vec<Object> = Vec::with_capacity(2i32 as usize);
        _arr0[0i32 as usize] = this.k0.get();
        _arr0[1i32 as usize] = this.v0.get();
        Ok(CollSer::new(3i32, _arr0)?)
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public"))]
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.k0.get().hashCode()?;
        let _t1 = this.v0.get().hashCode()?;
        Ok((_t0^_t1))
    }
}
