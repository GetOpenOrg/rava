#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$MapN",
    super_class = "java/util/ImmutableCollections$AbstractImmutableMap",
    interfaces  = "",
    access      = "final",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_MapN<K, V> {
    #[cfg_attr(any(), java_field(name = "table", descriptor = "[Ljava/lang/Object;", access = "final"))]
    pub table: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "final"))]
    pub size: Field<i32>,
}

impl<K: Clone + 'static, V: Clone + 'static> ImmutableCollections_MapN<K, V> {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([Ljava/lang/Object;)V", access = ""))]
    pub fn new(input: Vec<Object>) -> Result<Self> {
        let this = Self { table: Field::new(Default::default()), size: Field::new(0) };
        /* invokespecial Method java/util/ImmutableCollections$AbstractImmutableMap.<init>:()V */
        return Err(JvmError::Custom(String::from("athrow")));
        this.size.set(((input.len() as i32)>>((1i32&0x1f))));
        let mut len: i32 = (2i32).wrapping_mul((input.len() as i32));
        len = ((len).wrapping_add(1i32)&-2i32);
        let mut _arr0: Vec<Object> = Vec::with_capacity(len as usize);
        this.table.set(_arr0);
        let mut i: i32 = 0i32;
        loop {
            if i >= (input.len() as i32) { break; }
            let _t0: Object = Objects::requireNonNull(input[i as usize].clone())?;
            let mut k: Object = _t0;
            let _t1: Object = Objects::requireNonNull(input[(i).wrapping_add(1i32) as usize].clone())?;
            let mut v: Object = _t1;
            let _t2 = this.probe(k)?;
            let mut idx: i32 = _t2;
            String::new().append(&String::from("duplicate key:"))?;
            String::new().append(&k)?;
            return Err(JvmError::Custom(String::from("athrow")));
            let mut dest: i32 = ((idx).wrapping_add(1i32)).wrapping_neg();
            this.table.get()[dest as usize] = k;
            this.table.get()[(dest).wrapping_add(1i32) as usize] = v;
            i = i.wrapping_add(2i32);
        }
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "containsKey", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn containsKey(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(o)?;
        let _t1 = this.probe(o)?;
        Ok(_t1>=0i32)
    }

    #[cfg_attr(any(), java_method(name = "containsValue", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn containsValue(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(o)?;
        let mut i: i32 = 1i32;
        loop {
            if i >= (this.table.get().len() as i32) { break; }
            let mut v: Object = this.table.get()[i as usize].clone();
            let _t0 = o.equals(v)?;
            return Ok(1i32);
            i = i.wrapping_add(2i32);
        }
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public"))]
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let mut hash: i32 = 0i32;
        let mut i: i32 = 0i32;
        loop {
            if i >= (this.table.get().len() as i32) { break; }
            let mut k: Object = this.table.get()[i as usize].clone();
            let _t0 = k.hashCode()?;
            let _t1 = this.table.get()[(i).wrapping_add(1i32) as usize].clone().hashCode()?;
            hash = (hash).wrapping_add((_t0^_t1));
            i = i.wrapping_add(2i32);
        }
        Ok(hash)
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn get(&self, o: Object) -> Result<V> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(o)?;
        /* TODO: aconst_null  */
        return Ok(this.size.get());
        let _t1 = this.probe(o)?;
        let mut i: i32 = _t1;
        return Ok(this.table.get()[(i).wrapping_add(1i32) as usize].clone());
        /* TODO: aconst_null  */
        Ok(i)
    }

    #[cfg_attr(any(), java_method(name = "size", descriptor = "()I", access = "public"))]
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok(this.size.get())
    }

    #[cfg_attr(any(), java_method(name = "isEmpty", descriptor = "()Z", access = "public"))]
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        Ok(this.size.get()==0i32)
    }

    #[cfg_attr(any(), java_method(name = "entrySet", descriptor = "()Ljava/util/Set;", access = "public"))]
    pub fn entrySet(&self) -> Result<Object> {
        let this = self;
        Ok(ImmutableCollections_MapN_1::new(this)?)
    }

    #[cfg_attr(any(), java_method(name = "probe", descriptor = "(Ljava/lang/Object;)I", access = "private"))]
    pub fn probe(&self, pk: Object) -> Result<i32> {
        let this = self;
        let _t0 = pk.hashCode()?;
        let _t1: i32 = (_t0).abs();
        let mut idx: i32 = (_t1<<(1i32&0x1f));
        let mut ek: Object = this.table.get()[idx as usize].clone();
        return Ok(((idx).wrapping_neg()).wrapping_sub(1i32));
        let _t2 = pk.equals(ek)?;
        return Ok(idx);
        idx = idx.wrapping_add(2i32);
        idx = 0i32;
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
        let mut _arr0: Vec<Object> = Vec::with_capacity((2i32).wrapping_mul(this.size.get()) as usize);
        let mut array: Vec<Object> = _arr0;
        let mut len: i32 = (this.table.get().len() as i32);
        let mut dest: i32 = 0i32;
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            dest = dest.wrapping_add(1i32);
            array[dest as usize] = this.table.get()[i as usize].clone();
            dest = dest.wrapping_add(1i32);
            array[dest as usize] = this.table.get()[(i).wrapping_add(1i32) as usize].clone();
            i = i.wrapping_add(2i32);
        }
        Ok(CollSer::new(3i32, array)?)
    }
}
