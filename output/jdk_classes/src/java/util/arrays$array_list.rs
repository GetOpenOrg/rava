#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Arrays$ArrayList",
    super_class = "java/util/AbstractList",
    interfaces  = "java/util/RandomAccess,java/io/Serializable",
    access      = "",
    source      = "Arrays.java",
))]
pub struct Arrays_ArrayList<E> {
    #[cfg_attr(any(), java_field(name = "a", descriptor = "[Ljava/lang/Object;", access = "private final"))]
    pub a: Field<Vec<Object>>,
}

impl<E: Clone + 'static> Arrays_ArrayList<E> {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "([Ljava/lang/Object;)V"))]
    pub fn new(array: Vec<Object>) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()) };
        /* invokespecial Method java/util/AbstractList.<init>:()V */
        let _t0: Object = Objects::requireNonNull(&array)?;
        this.a.set(_t0);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "size", descriptor = "()I", access = "public"))]
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok((this.a.get().len() as i32))
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public"))]
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Vec<Object> = Arrays::copyOf(&this.a.get(), (this.a.get().len() as i32), 13i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public"))]
    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.size()?;
        let mut size: i32 = _t0;
        let _t1 = a.getClass()?;
        let _t2: Vec<Object> = Arrays::copyOf(&this.a.get(), size, _t1)?;
        return Ok(_t2);
        System::arraycopy(&this.a.get(), 0i32, &a, 0i32, size)?;
        /* TODO: aconst_null  */
        size[a as usize] = size;
        Ok(a)
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "(I)Ljava/lang/Object;", access = "public"))]
    pub fn get(&self, index: i32) -> Result<E> {
        let this = self;
        Ok(this.a.get()[index as usize].clone())
    }

    #[cfg_attr(any(), java_method(name = "set", descriptor = "(ILjava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn set(&self, index: i32, element: E) -> Result<E> {
        let this = self;
        let mut oldValue: Object = this.a.get()[index as usize].clone();
        this.a.get()[index as usize] = element;
        Ok(oldValue)
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "(Ljava/lang/Object;)I", access = "public"))]
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let mut a: Vec<Object> = this.a.get();
        let mut i: i32 = 0i32;
        loop {
            if i >= (a.len() as i32) { break; }
            return Ok(i);
            i = i.wrapping_add(1i32);
        }
        i = 0i32;
        loop {
            if i >= (a.len() as i32) { break; }
            let _t0 = o.equals(a[i as usize].clone())?;
            return Ok(i);
            i = i.wrapping_add(1i32);
        }
        Ok(-1i32)
    }

    #[cfg_attr(any(), java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn contains(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.indexOf(o)?;
        Ok(_t0>=0i32)
    }

    #[cfg_attr(any(), java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public"))]
    pub fn spliterator(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Spliterators::spliterator(&this.a.get(), 16i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "forEach", descriptor = "(Ljava/util/function/Consumer;)V", access = "public"))]
    pub fn forEach(&self, action: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(action)?;
        let mut local_2: Vec<Object> = this.a.get();
        let mut local_3: i32 = (local_2.len() as i32);
        let mut local_4: i32 = 0i32;
        loop {
            if local_4 >= local_3 { break; }
            let mut e: Object = local_2[local_4 as usize].clone();
            action.accept(e)?;
            local_4 = local_4.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "replaceAll", descriptor = "(Ljava/util/function/UnaryOperator;)V", access = "public"))]
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(operator)?;
        let mut a: Vec<Object> = this.a.get();
        let mut i: i32 = 0i32;
        loop {
            if i >= (a.len() as i32) { break; }
            let _t0 = operator.apply(a[i as usize].clone())?;
            a[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "sort", descriptor = "(Ljava/util/Comparator;)V", access = "public"))]
    pub fn sort(&self, c: Object) -> Result<()> {
        let this = self;
        Arrays::sort(&this.a.get(), c)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public"))]
    pub fn iterator(&self) -> Result<Object> {
        let this = self;
        Ok(Arrays_ArrayItr::new(this.a.get())?)
    }
}
