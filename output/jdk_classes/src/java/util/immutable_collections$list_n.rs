#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

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
}

impl<E: Clone + 'static> ImmutableCollections_ListN<E> {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([Ljava/lang/Object;Z)V", access = "private"))]
    pub fn new(elements: Vec<Object>, allowNulls: bool) -> Result<Self> {
        let this = Self { elements: Field::new(Default::default()), allowNulls: Field::new(false) };
        /* invokespecial Method java/util/ImmutableCollections$AbstractImmutableList.<init>:()V */
        this.elements.set(elements);
        this.allowNulls.set(allowNulls);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "isEmpty", descriptor = "()Z", access = "public"))]
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        Ok((this.elements.get().len() as i32)==0i32)
    }

    #[cfg_attr(any(), java_method(name = "size", descriptor = "()I", access = "public"))]
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok((this.elements.get().len() as i32))
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "(I)Ljava/lang/Object;", access = "public"))]
    pub fn get(&self, index: i32) -> Result<E> {
        let this = self;
        Ok(this.elements.get()[index as usize].clone())
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
        Ok(CollSer::new(this.allowNulls.get()==0i32, this.elements.get())?)
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public"))]
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Vec<Object> = Arrays::copyOf(&this.elements.get(), (this.elements.get().len() as i32))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public"))]
    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        let this = self;
        let mut size: i32 = (this.elements.get().len() as i32);
        let _t0 = a.getClass()?;
        let _t1: Vec<Object> = Arrays::copyOf(&this.elements.get(), size, _t0)?;
        return Ok(_t1);
        System::arraycopy(&this.elements.get(), 0i32, &a, 0i32, size)?;
        /* TODO: aconst_null  */
        size[a as usize] = size;
        Ok(a)
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "(Ljava/lang/Object;)I", access = "public"))]
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
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

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/Object;)I", access = "public"))]
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
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
