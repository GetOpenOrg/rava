#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ImmutableCollections$List12",
    super_class = "java/util/ImmutableCollections$AbstractImmutableList",
    interfaces  = "java/io/Serializable",
    access      = "final",
    source      = "ImmutableCollections.java",
))]
pub struct ImmutableCollections_List12<E> {
    #[cfg_attr(any(), java_field(name = "e0", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub e0: Field<Object>,
    #[cfg_attr(any(), java_field(name = "e1", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub e1: Field<Object>,
}

impl<E: Clone + 'static> ImmutableCollections_List12<E> {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Object;)V"))]
    // java: <init>(Ljava/lang/Object;)V
    pub fn new__obj(e0: E) -> Result<Self> {
        let this = Self { e0: Field::new(Default::default()), e1: Field::new(Default::default()) };
        /* invokespecial Method java/util/ImmutableCollections$AbstractImmutableList.<init>:()V */
        let _t0: Object = Objects::requireNonNull(e0)?;
        this.e0.set(_t0);
        this.e1.set(ImmutableCollections::EMPTY());
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)V"))]
    // java: <init>(Ljava/lang/Object;Ljava/lang/Object;)V
    pub fn new__obj_obj(e0: E, e1: E) -> Result<Self> {
        let this = Self { e0: Field::new(Default::default()), e1: Field::new(Default::default()) };
        /* invokespecial Method java/util/ImmutableCollections$AbstractImmutableList.<init>:()V */
        let _t0: Object = Objects::requireNonNull(e0)?;
        this.e0.set(_t0);
        let _t1: Object = Objects::requireNonNull(e1)?;
        this.e1.set(_t1);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "size", descriptor = "()I", access = "public"))]
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok(/* if_acmpeq */ true)
    }

    #[cfg_attr(any(), java_method(name = "isEmpty", descriptor = "()Z", access = "public"))]
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "(I)Ljava/lang/Object;", access = "public"))]
    pub fn get(&self, index: i32) -> Result<E> {
        let this = self;
        return Ok(this.e0.get());
        return Ok(this.e1.get());
        let _t0 = this.outOfBounds(index)?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "(Ljava/lang/Object;)I", access = "public"))]
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(o)?;
        let _t1 = o.equals(this.e0.get())?;
        return Ok(0i32);
        let _t2 = o.equals(this.e1.get())?;
        return Ok(1i32);
        Ok(-1i32)
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/Object;)I", access = "public"))]
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(o)?;
        let _t1 = o.equals(this.e1.get())?;
        return Ok(1i32);
        let _t2 = o.equals(this.e0.get())?;
        return Ok(0i32);
        Ok(-1i32)
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
        let mut _arr0: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr0[0i32 as usize] = this.e0.get();
        return Ok(CollSer::new(1i32, _arr0)?);
        let mut _arr1: Vec<Object> = Vec::with_capacity(2i32 as usize);
        _arr1[0i32 as usize] = this.e0.get();
        _arr1[1i32 as usize] = this.e1.get();
        Ok(CollSer::new(1i32, _arr1)?)
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public"))]
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        let this = self;
        let mut _arr0: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr0[0i32 as usize] = this.e0.get();
        return Ok(_arr0);
        let mut _arr1: Vec<Object> = Vec::with_capacity(2i32 as usize);
        _arr1[0i32 as usize] = this.e0.get();
        _arr1[1i32 as usize] = this.e1.get();
        Ok(_arr1)
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public"))]
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
        array[1i32 as usize] = this.e1.get();
        /* TODO: aconst_null  */
        size[array as usize] = size;
        Ok(array)
    }
}
