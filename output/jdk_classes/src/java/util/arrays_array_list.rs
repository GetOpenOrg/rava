#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

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
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Arrays_ArrayList<E> {
    // java: <init>([Ljava/lang/Object;)V
    pub fn new(array: Vec<Object>) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/AbstractList.<init>:()V */
        let _t0: Object = Objects::requireNonNull__obj(&array)?;
        this.a.set(_t0);
        Ok(this)
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok((this.a.get().len() as i32))
    }

    // java: toArray()[Ljava/lang/Object;
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Vec<Object> = Arrays::copyOf__arr_obj_i_class(&this.a.get(), (this.a.get().len() as i32), 13i32)?;
        Ok(_t0)
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.size()?;
        let mut size: i32 = _t0;
        let _t1 = a.getClass()?;
        let _t2: Vec<Object> = Arrays::copyOf__arr_obj_i_class(&this.a.get(), size, _t1)?;
        return Ok(_t2);
        System::arraycopy(&this.a.get(), 0i32, &a, 0i32, size)?;
        /* TODO: aconst_null  */
        size[a as usize] = size;
        Ok(a)
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, index: i32) -> Result<E> {
        let this = self;
        Ok(this.a.get()[index as usize].clone())
    }

    // java: set(ILjava/lang/Object;)Ljava/lang/Object;
    pub fn set(&self, index: i32, element: E) -> Result<E> {
        let this = self;
        let mut oldValue: Object = this.a.get()[index as usize].clone();
        this.a.get()[index as usize] = element;
        Ok(oldValue)
    }

    // java: indexOf(Ljava/lang/Object;)I
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

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.indexOf(o)?;
        Ok(_t0>=0i32)
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Spliterators::spliterator__arr_obj_i(&this.a.get(), 16i32)?;
        Ok(_t0)
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(action)?;
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

    // java: replaceAll(Ljava/util/function/UnaryOperator;)V
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(operator)?;
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

    // java: sort(Ljava/util/Comparator;)V
    pub fn sort(&self, c: Object) -> Result<()> {
        let this = self;
        Arrays::sort__arr_obj_compar(&this.a.get(), c)?;
        Ok(())
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        let this = self;
        Ok(Arrays_ArrayItr::new(this.a.get())?)
    }
}
