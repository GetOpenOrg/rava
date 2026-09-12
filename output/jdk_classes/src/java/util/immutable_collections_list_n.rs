#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$ListN",
    super_class = "java/util/ImmutableCollections$AbstractImmutableList",
    interfaces  = "java/io/Serializable",
    access      = "final",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_ListN<E> {
    #[cfg_attr(any(), java_field(name = "elements", descriptor = "[Ljava/lang/Object;", access = "private final"))]
    pub elements: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "allowNulls", descriptor = "Z", access = "private final"))]
    pub allowNulls: Field<bool>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> ImmutableCollections_ListN<E> {
    // java: <init>([Ljava/lang/Object;Z)V
    pub fn new(elements: Vec<Object>, allowNulls: bool) -> Result<Self> {
        let this = Self { elements: Field::new(Default::default()), allowNulls: Field::new(false), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/ImmutableCollections$AbstractImmutableList.<init>:()V */
        this.elements.set(elements);
        this.allowNulls.set(allowNulls);
        Ok(this)
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        Ok((this.elements.get().len() as i32)==0i32)
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok((this.elements.get().len() as i32))
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, index: i32) -> Result<E> {
        let this = self;
        Ok(this.elements.get()[index as usize].clone())
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
        Ok(CollSer::new(this.allowNulls.get()==0i32, this.elements.get())?)
    }

    // java: toArray()[Ljava/lang/Object;
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Vec<Object> = Arrays::copyOf__arr_obj_i(&this.elements.get(), (this.elements.get().len() as i32))?;
        Ok(_t0)
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        let this = self;
        let mut size: i32 = (this.elements.get().len() as i32);
        let _t0 = a.getClass()?;
        let _t1: Vec<Object> = Arrays::copyOf__arr_obj_i_class(&this.elements.get(), size, _t0)?;
        return Ok(_t1);
        System::arraycopy(&this.elements.get(), 0i32, &a, 0i32, size)?;
        /* TODO: aconst_null  */
        size[a as usize] = size;
        Ok(a)
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut es: Vec<Object> = this.elements.get();
        let mut i: i32 = 0i32;
        loop {
            if i >= (es.len() as i32) { break; }
            let _t0: bool = Objects::equals(o, es[i as usize].clone())?;
            return Ok(i);
            i = i.wrapping_add(1i32);
        }
        Ok(-1i32)
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut es: Vec<Object> = this.elements.get();
        let mut i: i32 = ((es.len() as i32)).wrapping_sub(1i32);
        loop {
            if i<0i32 { break; }
            let _t0: bool = Objects::equals(o, es[i as usize].clone())?;
            return Ok(i);
            i = i.wrapping_sub(1i32);
        }
        Ok(-1i32)
    }
}
