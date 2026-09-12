#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use crate::java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/HashSet",
    super_class = "java/util/AbstractSet",
    interfaces  = "java/util/Set,java/lang/Cloneable,java/io/Serializable",
    access      = "public",
    source      = "HashSet.java",
))]
pub struct HashSet<E> {
    #[cfg_attr(any(), java_field(name = "map", descriptor = "Ljava/util/HashMap;", access = ""))]
    pub map: Field<JvmObject>,
}

impl<E: Clone + 'static> HashSet<E> {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public"))]
    pub fn new() -> Result<Self> {
        let this = Self { map: Field::new(Default::default()) };
        /* invokespecial Method java/util/AbstractSet.<init>:()V */
        this.map.set(HashMap::<_, _>::new()?);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/Collection;)V", access = "public"))]
    pub fn new(c: JvmObject) -> Result<Self> {
        let this = Self { map: Field::new(Default::default()) };
        /* invokespecial Method java/util/AbstractSet.<init>:()V */
        let _t0 = c.size()?;
        let _t1: i32 = (_t0).max(12i32);
        let _t2: JvmObject = HashMap::newHashMap(_t1)?;
        this.map.set(_t2);
        let _t3 = this.addAll(c)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(IF)V", access = "public"))]
    pub fn new(initialCapacity: i32, loadFactor: f32) -> Result<Self> {
        let this = Self { map: Field::new(Default::default()) };
        /* invokespecial Method java/util/AbstractSet.<init>:()V */
        this.map.set(HashMap::<_, _>::new()?);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(I)V", access = "public"))]
    pub fn new(initialCapacity: i32) -> Result<Self> {
        let this = Self { map: Field::new(Default::default()) };
        /* invokespecial Method java/util/AbstractSet.<init>:()V */
        this.map.set(HashMap::<_, _>::new()?);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(IFZ)V"))]
    pub fn new(initialCapacity: i32, loadFactor: f32, dummy: bool) -> Result<Self> {
        let this = Self { map: Field::new(Default::default()) };
        /* invokespecial Method java/util/AbstractSet.<init>:()V */
        this.map.set(LinkedHashMap::new(initialCapacity, loadFactor)?);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public"))]
    pub fn iterator(&self) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.map.get().keySet()?;
        let _t1 = _t0.iterator()?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "size", descriptor = "()I", access = "public"))]
    pub fn size(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.map.get().size()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "isEmpty", descriptor = "()Z", access = "public"))]
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.map.get().isEmpty()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn contains(&self, o: JvmObject) -> Result<bool> {
        let this = self;
        let _t0 = this.map.get().containsKey(o)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn add(&self, e: E) -> Result<bool> {
        let this = self;
        let _t0 = this.map.get().put(e, HashSet::PRESENT())?;
        Ok(_t0.is_none())
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn remove(&self, o: JvmObject) -> Result<bool> {
        let this = self;
        let _t0 = this.map.get().remove(o)?;
        Ok(/* if_acmpne */ true)
    }

    #[cfg_attr(any(), java_method(name = "clear", descriptor = "()V", access = "public"))]
    pub fn clear(&self) -> Result<()> {
        let this = self;
        this.map.get().clear()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn clone(&self) -> Result<JvmObject> {
        let this = self;
        let mut newSet: java/util/HashSet = this;
        let _t0 = this.map.get().clone()?;
        newSet.map.set(_t0);
        return Ok(newSet);
        newSet = /* UNDERFLOW */;
        panic!("{}", /* InternalError::new(newSet)? */);
    }

    #[cfg_attr(any(), java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private"))]
    pub fn writeObject(&self, s: JvmObject) -> Result<()> {
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
        let mut local_2: JvmObject = _t4;
        loop {
            let _t0 = local_2.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_2.next()?;
            let mut e: JvmObject = _t0;
            s.writeObject(e)?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private"))]
    pub fn readObject(&self, s: JvmObject) -> Result<()> {
        let this = self;
        let _t0 = s.readFields()?;
        let _t1 = s.readInt()?;
        let mut capacity: i32 = _t1;
        String::new().append(&String::from("Illegal capacity:"))?;
        String::new().append(&capacity)?;
        panic!("{}", /* InvalidObjectException::new(String::new())? */);
        let _t2 = s.readFloat()?;
        let mut loadFactor: f32 = _t2;
        /* TODO: fcmpg  */
        let _t3: bool = Float::isNaN(loadFactor)?;
        String::new().append(&String::from("Illegal load factor:"))?;
        String::new().append(&loadFactor)?;
        panic!("{}", /* InvalidObjectException::new(String::new())? */);
        let _t4: f32 = (loadFactor).abs();
        loadFactor = _t4;
        let _t5 = s.readInt()?;
        let mut size: i32 = _t5;
        String::new().append(&String::from("Illegal size:"))?;
        String::new().append(&size)?;
        panic!("{}", /* InvalidObjectException::new(String::new())? */);
        let _t6: f32 = ((1f32/loadFactor)).min(4.0f32);
        let _t7: f32 = (((size as f32)*_t6)).min(1073741824.0f32);
        capacity = (_t7 as i32);
        let _t8: JvmObject = SharedSecrets::getJavaObjectInputStreamAccess()?;
        let _t9: i32 = HashMap::tableSizeFor(capacity)?;
        _t8.checkArray(s, 187i32, _t9)?;
        LinkedHashMap::new(capacity, loadFactor)?.map.set(HashMap::<_, _>::new()?);
        let mut i: i32 = 0i32;
        loop {
            if i >= size { break; }
            let _t0 = s.readObject()?;
            let mut e: JvmObject = _t0;
            let _t1 = this.map.get().put(e, HashSet::PRESENT())?;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public"))]
    pub fn spliterator(&self) -> Result<JvmObject> {
        let this = self;
        Ok(HashMap$KeySpliterator::new(this.map.get(), 0i32, -1i32, 0i32, 0i32)?)
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public"))]
    pub fn toArray(&self) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.map.get().size()?;
        let mut _arr1: Vec<JvmObject> = Vec::with_capacity(_t0 as usize);
        let _t2 = this.map.get().keysToArray(_arr1)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public"))]
    pub fn toArray(&self, a: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.map.get().prepareArray(a)?;
        let _t1 = this.map.get().keysToArray(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "newHashSet", descriptor = "(I)Ljava/util/HashSet;", access = "public static"))]
    pub fn newHashSet(numElements: i32) -> Result<JvmObject> {
        String::new().append(&String::from("Negative number of elements:"))?;
        String::new().append(&numElements)?;
        panic!("{}", /* IllegalArgumentException::new(String::new())? */);
        let _t0: i32 = HashMap::calculateHashMapCapacity(numElements)?;
        Ok(HashSet::<_>::new()?)
    }
}
