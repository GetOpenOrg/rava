#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

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
    pub _phantom: std::marker::PhantomData<(K, V,)>,
}

impl<K: Clone + 'static, V: Clone + 'static> ImmutableCollections_Map1<K, V> {
    // java: <init>(Ljava/lang/Object;Ljava/lang/Object;)V
    pub fn new(k0: K, v0: V) -> Result<Self> {
        let this = Self { k0: Field::new(Default::default()), v0: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/ImmutableCollections$AbstractImmutableMap.<init>:()V */
        let _t0: Object = Objects::requireNonNull__obj(k0)?;
        this.k0.set(_t0);
        let _t1: Object = Objects::requireNonNull__obj(v0)?;
        this.v0.set(_t1);
        Ok(this)
    }

    // java: entrySet()Ljava/util/Set;
    pub fn entrySet(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Set::of__obj(KeyValueHolder::new(this.k0.get(), this.v0.get())?)?;
        Ok(_t0)
    }

    // java: get(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn get(&self, o: Object) -> Result<V> {
        let this = self;
        let _t0 = o.equals(this.k0.get())?;
        /* TODO: aconst_null  */
        Ok(this.v0.get())
    }

    // java: containsKey(Ljava/lang/Object;)Z
    pub fn containsKey(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = o.equals(this.k0.get())?;
        Ok(_t0)
    }

    // java: containsValue(Ljava/lang/Object;)Z
    pub fn containsValue(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = o.equals(this.v0.get())?;
        Ok(_t0)
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok(1i32)
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        Ok(0i32)
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, in_: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: writeReplace()Ljava/lang/Object;
    pub fn writeReplace(&self) -> Result<Object> {
        let this = self;
        let mut _arr0: Vec<Object> = Vec::with_capacity(2i32 as usize);
        _arr0[0i32 as usize] = this.k0.get();
        _arr0[1i32 as usize] = this.v0.get();
        Ok(CollSer::new(3i32, _arr0)?)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.k0.get().hashCode()?;
        let _t1 = this.v0.get().hashCode()?;
        Ok((_t0^_t1))
    }
}
