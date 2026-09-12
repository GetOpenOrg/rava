#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$CopiesList",
    super_class = "java/util/AbstractList",
    interfaces  = "java/util/RandomAccess,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_CopiesList<E> {
    #[cfg_attr(any(), java_field(name = "n", descriptor = "I", access = "final"))]
    pub n: Field<i32>,
    #[cfg_attr(any(), java_field(name = "element", descriptor = "Ljava/lang/Object;", access = "final"))]
    pub element: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_CopiesList<E> {
    // java: <init>(ILjava/lang/Object;)V
    pub fn new(n: i32, e: E) -> Result<Self> {
        let this = Self { n: Field::new(0), element: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/AbstractList.<init>:()V */
        return Err(JvmError::Custom("athrow".to_owned()));
        this.n.set(n);
        this.element.set(e);
        Ok(this)
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok(this.n.get())
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, obj: Object) -> Result<bool> {
        let this = self;
        let _t0: bool = Collections::eq(obj, this.element.get())?;
        Ok(_t0!=0i32)
    }

    // java: indexOf(Ljava/lang/Object;)I
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let _t0 = this.contains(o)?;
        Ok(_t0==0i32)
    }

    // java: lastIndexOf(Ljava/lang/Object;)I
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        let this = self;
        let _t0 = this.contains(o)?;
        Ok(-1i32)
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, index: i32) -> Result<E> {
        let this = self;
        let _t0: i32 = Objects::checkIndex__i_i(index, this.n.get())?;
        Ok(this.element.get())
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(action)?;
        let mut n: i32 = this.n.get();
        let mut element: Object = this.element.get();
        let mut i: i32 = 0i32;
        loop {
            if i >= n { break; }
            action.accept(element)?;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: toArray()[Ljava/lang/Object;
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        let this = self;
        let mut _arr0: Vec<Object> = Vec::with_capacity(this.n.get() as usize);
        let mut a: Vec<Object> = _arr0;
        Arrays::fill__arr_obj_i_i_obj(&a, 0i32, this.n.get(), this.element.get())?;
        Ok(a)
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        let this = self;
        let mut n: i32 = this.n.get();
        let _t0 = a.getClass()?;
        let _t1 = _t0.getComponentType()?;
        let _t2: Object = Array::newInstance(_t1, n)?;
        a = _t2;
        Arrays::fill__arr_obj_i_i_obj(&a, 0i32, n, this.element.get())?;
        Arrays::fill__arr_obj_i_i_obj(&a, 0i32, n, this.element.get())?;
        /* TODO: aconst_null  */
        n[a as usize] = n;
        Ok(a)
    }

    // java: subList(II)Ljava/util/List;
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        let this = self;
        String::new().append(&String::from("fromIndex ="))?;
        String::new().append(&fromIndex)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        String::new().append(&String::from("toIndex ="))?;
        String::new().append(&toIndex)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        String::new().append(&String::from("fromIndex("))?;
        String::new().append(&fromIndex)?;
        String::new().append(&String::from(") > toIndex("))?;
        String::new().append(&toIndex)?;
        String::new().append(&String::from(")"))?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(Collections_CopiesList::new((toIndex).wrapping_sub(fromIndex), this.element.get())?)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        return Ok(1i32);
        let mut pow: i32 = 31i32;
        let mut sum: i32 = 1i32;
        let _t0: i32 = Integer::numberOfLeadingZeros(this.n.get())?;
        let mut i: i32 = (_t0).wrapping_add(1i32);
        loop {
            if i >= 32i32 { break; }
            sum = (sum).wrapping_mul((pow).wrapping_add(1i32));
            pow = (pow).wrapping_mul(pow);
            pow = (pow).wrapping_mul(31i32);
            sum = ((sum).wrapping_mul(31i32)).wrapping_add(1i32);
            i = i.wrapping_add(1i32);
        }
        let _t1 = this.element.get().hashCode()?;
        Ok((this.element.get()).wrapping_add((0i32).wrapping_mul(_t1)))
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        let mut other: Object = o;
        let _t0: bool = Collections::eq(this.element.get(), other.element.get())?;
        return Ok(_t0!=0i32);
        return Ok(0i32);
        other = this.n.get();
        let mut e: Object = this.element.get();
        let _t1 = o.iterator()?;
        let mut itr: Object = _t1;
        let _t2 = itr.hasNext()?;
        other = other.wrapping_sub(1i32);
        let _t3 = itr.next()?;
        return Ok(0i32);
        let _t4 = itr.hasNext()?;
        other = other.wrapping_sub(1i32);
        let _t5 = itr.next()?;
        let _t6 = e.equals(_t5)?;
        return Ok(0i32);
        let _t7 = itr.hasNext()?;
        Ok(_t7==0i32)
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = IntStream::range(0i32, this.n.get())?;
        /* TODO: invokedynamic 145 */
        let _t1 = _t0.mapToObj(this)?;
        Ok(_t1)
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = IntStream::range(0i32, this.n.get())?;
        let _t1 = _t0.parallel()?;
        /* TODO: invokedynamic 157 */
        let _t2 = _t1.mapToObj(this)?;
        Ok(_t2)
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.stream()?;
        let _t1 = _t0.spliterator()?;
        Ok(_t1)
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, ois: Object) -> Result<()> {
        let this = self;
        ois.defaultReadObject()?;
        let _t0: Object = SharedSecrets::getJavaObjectInputStreamAccess()?;
        _t0.checkArray(ois, 73i32, this.n.get())?;
        Ok(())
    }
}
