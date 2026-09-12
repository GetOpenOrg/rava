#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/HashSet",
    super_class = "java/util/AbstractSet",
    interfaces  = "java/util/Set,java/lang/Cloneable,java/io/Serializable",
    access      = "public",
    source      = "HashSet.java",
))]
pub struct HashSet<E> {
    #[cfg_attr(any(), java_field(name = "map", descriptor = "Ljava/util/HashMap;", access = ""))]
    pub map: Field<Object>,
}

impl<E: Clone + 'static> HashSet<E> {
    // java: <init>()V
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { map: Field::new(Default::default()) };
        /* invokespecial Method java/util/AbstractSet.<init>:()V */
        this.map.set(HashMap::<_, _>::new()?);
        Ok(this)
    }

    // java: <init>(Ljava/util/Collection;)V
    // java: <init>(Ljava/util/Collection;)V
    pub fn new__coll(c: Object) -> Result<Self> {
        let this = Self { map: Field::new(Default::default()) };
        /* invokespecial Method java/util/AbstractSet.<init>:()V */
        let _t0 = c.size()?;
        let _t1: i32 = (_t0).max(12i32);
        let _t2: Object = HashMap::newHashMap(_t1)?;
        this.map.set(_t2);
        let _t3 = this.addAll(c)?;
        Ok(this)
    }

    // java: <init>(IF)V
    // java: <init>(IF)V
    pub fn new__i_f(initialCapacity: i32, loadFactor: f32) -> Result<Self> {
        let this = Self { map: Field::new(Default::default()) };
        /* invokespecial Method java/util/AbstractSet.<init>:()V */
        this.map.set(HashMap::<_, _>::new()?);
        Ok(this)
    }

    // java: <init>(I)V
    // java: <init>(I)V
    pub fn new__i(initialCapacity: i32) -> Result<Self> {
        let this = Self { map: Field::new(Default::default()) };
        /* invokespecial Method java/util/AbstractSet.<init>:()V */
        this.map.set(HashMap::<_, _>::new()?);
        Ok(this)
    }

    // java: <init>(IFZ)V
    // java: <init>(IFZ)V
    pub fn new__i_f_z(initialCapacity: i32, loadFactor: f32, dummy: bool) -> Result<Self> {
        let this = Self { map: Field::new(Default::default()) };
        /* invokespecial Method java/util/AbstractSet.<init>:()V */
        this.map.set(LinkedHashMap::new(initialCapacity, loadFactor)?);
        Ok(this)
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.map.get().keySet()?;
        let _t1 = _t0.iterator()?;
        Ok(_t1)
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.map.get().size()?;
        Ok(_t0)
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.map.get().isEmpty()?;
        Ok(_t0)
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.map.get().containsKey(o)?;
        Ok(_t0)
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, e: E) -> Result<bool> {
        let this = self;
        let _t0 = this.map.get().put(e, HashSet::PRESENT())?;
        Ok(_t0.is_none())
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.map.get().remove(o)?;
        Ok(/* if_acmpne */ true)
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        let this = self;
        this.map.get().clear()?;
        Ok(())
    }

    // java: clone()Ljava/lang/Object;
    pub fn clone(&self) -> Result<Object> {
        let this = self;
        let mut newSet: HashSet = this;
        let _t0 = this.map.get().clone()?;
        newSet.map.set(_t0);
        return Ok(newSet);
        newSet = todo!("stack underflow");
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: writeObject(Ljava/io/ObjectOutputStream;)V
    pub fn writeObject(&self, s: Object) -> Result<()> {
        let this = self;
        s.defaultWriteObject()?;
        let _t0 = this.map.get().capacity()?;
        s.writeInt(_t0)?;
        let _t1 = this.map.get().loadFactor()?;
        s.writeFloat(_t1)?;
        let _t2 = this.map.get().size()?;
        s.writeInt(_t2)?;
        let _t3 = this.map.get().keySet()?;
        let _t4 = _t3.iterator()?;
        let mut local_2: Object = _t4;
        loop {
            let _t0 = local_2.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_2.next()?;
            let mut e: Object = _t0;
            s.writeObject(e)?;
        }
        Ok(())
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, s: Object) -> Result<()> {
        let this = self;
        let _t0 = s.readFields()?;
        let _t1 = s.readInt()?;
        let mut capacity: i32 = _t1;
        String::new().append(&String::from("Illegal capacity:"))?;
        String::new().append(&capacity)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t2 = s.readFloat()?;
        let mut loadFactor: f32 = _t2;
        /* TODO: fcmpg  */
        let _t3: bool = Float::isNaN(loadFactor)?;
        String::new().append(&String::from("Illegal load factor:"))?;
        String::new().append(&loadFactor)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t4: f32 = (loadFactor).abs();
        loadFactor = _t4;
        let _t5 = s.readInt()?;
        let mut size: i32 = _t5;
        String::new().append(&String::from("Illegal size:"))?;
        String::new().append(&size)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t6: f32 = ((1f32/loadFactor)).min(4.0f32);
        let _t7: f32 = (((size as f32)*_t6)).min(1073741824.0f32);
        capacity = (_t7 as i32);
        let _t8: Object = SharedSecrets::getJavaObjectInputStreamAccess()?;
        let _t9: i32 = HashMap::tableSizeFor(capacity)?;
        _t8.checkArray(s, 187i32, _t9)?;
        LinkedHashMap::new(capacity, loadFactor)?.map.set(HashMap::<_, _>::new()?);
        let mut i: i32 = 0i32;
        loop {
            if i >= size { break; }
            let _t0 = s.readObject()?;
            let mut e: Object = _t0;
            let _t1 = this.map.get().put(e, HashSet::PRESENT())?;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        let this = self;
        Ok(HashMap_KeySpliterator::new(this.map.get(), 0i32, -1i32, 0i32, 0i32)?)
    }

    // java: toArray()[Ljava/lang/Object;
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.map.get().size()?;
        let mut _arr1: Vec<Object> = Vec::with_capacity(_t0 as usize);
        let _t2 = this.map.get().keysToArray(_arr1)?;
        Ok(_t2)
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.map.get().prepareArray(a)?;
        let _t1 = this.map.get().keysToArray(_t0)?;
        Ok(_t1)
    }

    // java: newHashSet(I)Ljava/util/HashSet;
    pub fn newHashSet(numElements: i32) -> Result<Object> {
        String::new().append(&String::from("Negative number of elements:"))?;
        String::new().append(&numElements)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: i32 = HashMap::calculateHashMapCapacity(numElements)?;
        Ok(HashSet::<_>::new()?)
    }
}
