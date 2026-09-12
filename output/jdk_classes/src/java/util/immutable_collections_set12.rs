#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$Set12",
    super_class = "java/util/ImmutableCollections$AbstractImmutableSet",
    interfaces  = "java/io/Serializable",
    access      = "final",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_Set12<E> {
    #[cfg_attr(any(), java_field(name = "e0", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub e0: Field<Object>,
    #[cfg_attr(any(), java_field(name = "e1", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub e1: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> ImmutableCollections_Set12<E> {
    // java: <init>(Ljava/lang/Object;)V
    // java: <init>(Ljava/lang/Object;)V
    pub fn new__obj(e0: E) -> Result<Self> {
        let this = Self { e0: Field::new(Default::default()), e1: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/ImmutableCollections$AbstractImmutableSet.<init>:()V */
        let _t0: Object = Objects::requireNonNull__obj(e0)?;
        this.e0.set(_t0);
        this.e1.set(ImmutableCollections::EMPTY());
        Ok(this)
    }

    // java: <init>(Ljava/lang/Object;Ljava/lang/Object;)V
    // java: <init>(Ljava/lang/Object;Ljava/lang/Object;)V
    pub fn new__obj_obj(e0: E, e1: E) -> Result<Self> {
        let this = Self { e0: Field::new(Default::default()), e1: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/ImmutableCollections$AbstractImmutableSet.<init>:()V */
        let _t0: Object = Objects::requireNonNull__obj(e1)?;
        let _t1 = e0.equals(_t0)?;
        String::new().append(&String::from("duplicate element:"))?;
        String::new().append(&e0)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.e0.set(e0);
        this.e1.set(e1);
        Ok(this)
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok(/* if_acmpne */ true)
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        Ok(0i32)
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = o.equals(this.e0.get())?;
        let _t1 = this.e1.get().equals(o)?;
        Ok(_t1!=0i32)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.e0.get().hashCode()?;
        let _t1 = this.e1.get().hashCode()?;
        Ok((0i32).wrapping_add(_t1))
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        let this = self;
        Ok(ImmutableCollections_Set12_1::new(this)?)
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
        let mut _arr0: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr0[0i32 as usize] = this.e0.get();
        return Ok(CollSer::new(2i32, _arr0)?);
        let mut _arr1: Vec<Object> = Vec::with_capacity(2i32 as usize);
        _arr1[0i32 as usize] = this.e0.get();
        _arr1[1i32 as usize] = this.e1.get();
        Ok(CollSer::new(2i32, _arr1)?)
    }

    // java: toArray()[Ljava/lang/Object;
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        let this = self;
        let mut _arr0: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr0[0i32 as usize] = this.e0.get();
        return Ok(_arr0);
        let mut _arr1: Vec<Object> = Vec::with_capacity(2i32 as usize);
        _arr1[0i32 as usize] = this.e1.get();
        _arr1[1i32 as usize] = this.e0.get();
        return Ok(_arr1);
        let mut _arr2: Vec<Object> = Vec::with_capacity(2i32 as usize);
        _arr2[0i32 as usize] = this.e0.get();
        _arr2[1i32 as usize] = this.e1.get();
        Ok(_arr2)
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.size()?;
        let mut size: i32 = _t0;
        let _t1 = a.getClass()?;
        let _t2 = _t1.getComponentType()?;
        let _t3: Object = Array::newInstance(_t2, size)?;
        let mut array: Object = _t3;
        array[0i32 as usize] = this.e0.get();
        array[0i32 as usize] = this.e1.get();
        array[1i32 as usize] = this.e0.get();
        array[0i32 as usize] = this.e0.get();
        array[1i32 as usize] = this.e1.get();
        /* TODO: aconst_null  */
        size[array as usize] = size;
        Ok(array)
    }
}
