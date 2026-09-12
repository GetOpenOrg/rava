#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$SetN",
    super_class = "java/util/ImmutableCollections$AbstractImmutableSet",
    interfaces  = "java/io/Serializable",
    access      = "final",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_SetN<E> {
    #[cfg_attr(any(), java_field(name = "elements", descriptor = "[Ljava/lang/Object;", access = "final"))]
    pub elements: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "final"))]
    pub size: Field<i32>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> ImmutableCollections_SetN<E> {
    // java: <init>([Ljava/lang/Object;)V
    pub fn new(input: Vec<Object>) -> Result<Self> {
        let this = Self { elements: Field::new(Default::default()), size: Field::new(0), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/ImmutableCollections$AbstractImmutableSet.<init>:()V */
        this.size.set((input.len() as i32));
        let mut _arr0: Vec<Object> = Vec::with_capacity((2i32).wrapping_mul((input.len() as i32)) as usize);
        this.elements.set(_arr0);
        let mut i: i32 = 0i32;
        loop {
            if i >= (input.len() as i32) { break; }
            let mut e: Object = input[i as usize].clone();
            let _t0 = this.probe(e)?;
            let mut idx: i32 = _t0;
            String::new().append(&String::from("duplicate element:"))?;
            String::new().append(&e)?;
            return Err(JvmError::Custom("athrow".to_owned()));
            this.elements.get()[((idx).wrapping_add(1i32)).wrapping_neg() as usize] = e;
            i = i.wrapping_add(1i32);
        }
        Ok(this)
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok(this.size.get())
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        Ok(this.size.get()==0i32)
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(o)?;
        let _t1 = this.probe(o)?;
        Ok(_t1>=0i32)
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        let this = self;
        Ok(ImmutableCollections_SetN_SetNIterator::new(this)?)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let mut h: i32 = 0i32;
        let mut local_2: Vec<Object> = this.elements.get();
        let mut local_3: i32 = (local_2.len() as i32);
        let mut local_4: i32 = 0i32;
        loop {
            if local_4 >= local_3 { break; }
            let mut e: Object = local_2[local_4 as usize].clone();
            let _t0 = e.hashCode()?;
            h = (h).wrapping_add(_t0);
            local_4 = local_4.wrapping_add(1i32);
        }
        Ok(h)
    }

    // java: probe(Ljava/lang/Object;)I
    pub fn probe(&self, pe: Object) -> Result<i32> {
        let this = self;
        let _t0 = pe.hashCode()?;
        let _t1: i32 = (_t0).abs();
        let mut idx: i32 = _t1;
        let mut ee: Object = this.elements.get()[idx as usize].clone();
        return Ok(((idx).wrapping_neg()).wrapping_sub(1i32));
        let _t2 = pe.equals(ee)?;
        return Ok(idx);
        idx = idx.wrapping_add(1i32);
        idx = 0i32;
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
        let mut _arr0: Vec<Object> = Vec::with_capacity(this.size.get() as usize);
        let mut array: Vec<Object> = _arr0;
        let mut dest: i32 = 0i32;
        let mut local_3: Vec<Object> = this.elements.get();
        let mut local_4: i32 = (local_3.len() as i32);
        let mut local_5: i32 = 0i32;
        loop {
            if local_5 >= local_4 { break; }
            let mut o: Object = local_3[local_5 as usize].clone();
            dest = dest.wrapping_add(1i32);
            array[dest as usize] = o;
            local_5 = local_5.wrapping_add(1i32);
        }
        Ok(CollSer::new(2i32, array)?)
    }

    // java: toArray()[Ljava/lang/Object;
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        let this = self;
        let mut _arr0: Vec<Object> = Vec::with_capacity(this.size.get() as usize);
        let mut array: Vec<Object> = _arr0;
        let _t1 = this.iterator()?;
        let mut it: Object = _t1;
        let mut i: i32 = 0i32;
        loop {
            if i >= this.size.get() { break; }
            let _t0 = it.next()?;
            array[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        Ok(array)
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = a.getClass()?;
        let _t1 = _t0.getComponentType()?;
        let _t2: Object = Array::newInstance(_t1, this.size.get())?;
        let mut array: Object = _t2;
        let _t3 = this.iterator()?;
        let mut it: Object = _t3;
        let mut i: i32 = 0i32;
        loop {
            if i >= this.size.get() { break; }
            let _t0 = it.next()?;
            array[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        /* TODO: aconst_null  */
        this.size.get()[array as usize] = this.size.get();
        Ok(array)
    }
}
