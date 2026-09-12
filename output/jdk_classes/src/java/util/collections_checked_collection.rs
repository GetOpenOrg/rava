#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$CheckedCollection",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Collection,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_CheckedCollection<E> {
    #[cfg_attr(any(), java_field(name = "c", descriptor = "Ljava/util/Collection;", access = "final"))]
    pub c: Field<Object>,
    #[cfg_attr(any(), java_field(name = "type", descriptor = "Ljava/lang/Class;", access = "final"))]
    pub type_: Field<Object>,
    #[cfg_attr(any(), java_field(name = "zeroLengthElementArray", descriptor = "[Ljava/lang/Object;", access = "private"))]
    pub zeroLengthElementArray: Field<Vec<Object>>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_CheckedCollection<E> {
    // java: typeCheck(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn typeCheck(&self, o: Object) -> Result<E> {
        let this = self;
        let _t0 = this.type_.get().isInstance(o)?;
        let _t1 = this.badElementMsg(o)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(o)
    }

    // java: badElementMsg(Ljava/lang/Object;)Ljava/lang/String;
    pub fn badElementMsg(&self, o: Object) -> Result<String> {
        let this = self;
        String::new().append(&String::from("Attempt to insert"))?;
        let _t0 = o.getClass()?;
        String::new().append(&_t0)?;
        String::new().append(&String::from("element into collection with element type"))?;
        String::new().append(&this.type_.get())?;
        Ok(String::new())
    }

    // java: <init>(Ljava/util/Collection;Ljava/lang/Class;)V
    pub fn new(c: Object, type_: Object) -> Result<Self> {
        let this = Self { c: Field::new(Default::default()), type_: Field::new(Default::default()), zeroLengthElementArray: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        let _t0: Object = Objects::requireNonNull__obj_str(c, String::from("c"))?;
        this.c.set(_t0);
        let _t1: Object = Objects::requireNonNull__obj_str(type_, String::from("type"))?;
        this.type_.set(_t1);
        Ok(this)
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.c.get().size()?;
        Ok(_t0)
    }

    // java: isEmpty()Z
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.c.get().isEmpty()?;
        Ok(_t0)
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.c.get().contains(o)?;
        Ok(_t0)
    }

    // java: toArray()[Ljava/lang/Object;
    // java: toArray()[Ljava/lang/Object;
    pub fn toArray(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.c.get().toArray()?;
        Ok(_t0)
    }

    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
    pub fn toArray__arr_obj(&self, a: Vec<Object>) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.c.get().toArray(a)?;
        Ok(_t0)
    }

    // java: toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;
    // java: toArray(Ljava/util/function/IntFunction;)[Ljava/lang/Object;
    pub fn toArray__intfun(&self, f: Object) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.c.get().toArray(f)?;
        Ok(_t0)
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.c.get().toString()?;
        Ok(_t0)
    }

    // java: remove(Ljava/lang/Object;)Z
    pub fn remove(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.c.get().remove(o)?;
        Ok(_t0)
    }

    // java: clear()V
    pub fn clear(&self) -> Result<()> {
        let this = self;
        this.c.get().clear()?;
        Ok(())
    }

    // java: containsAll(Ljava/util/Collection;)Z
    pub fn containsAll(&self, coll: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.c.get().containsAll(coll)?;
        Ok(_t0)
    }

    // java: removeAll(Ljava/util/Collection;)Z
    pub fn removeAll(&self, coll: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.c.get().removeAll(coll)?;
        Ok(_t0)
    }

    // java: retainAll(Ljava/util/Collection;)Z
    pub fn retainAll(&self, coll: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.c.get().retainAll(coll)?;
        Ok(_t0)
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.c.get().iterator()?;
        let mut it: Object = _t0;
        Ok(Collections_CheckedCollection_1::new(this, it)?)
    }

    // java: add(Ljava/lang/Object;)Z
    pub fn add(&self, e: E) -> Result<bool> {
        let this = self;
        let _t0 = this.typeCheck(e)?;
        let _t1 = this.c.get().add(_t0)?;
        Ok(_t1)
    }

    // java: zeroLengthElementArray()[Ljava/lang/Object;
    pub fn zeroLengthElementArray(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Vec<Object> = Collections::zeroLengthArray(this.type_.get())?;
        this.zeroLengthElementArray.set(_t0);
        Ok(_t0)
    }

    // java: checkedCopyOf(Ljava/util/Collection;)Ljava/util/Collection;
    pub fn checkedCopyOf(&self, coll: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.zeroLengthElementArray()?;
        let mut z: Vec<Object> = _t0;
        let _t1 = coll.toArray(z)?;
        let mut a: Vec<Object> = _t1;
        let _t2 = a.getClass()?;
        let _t3 = z.getClass()?;
        let _t4 = z.getClass()?;
        let _t5: Vec<Object> = Arrays::copyOf__arr_obj_i_class(&a, (a.len() as i32), _t4)?;
        a = _t5;
        z = _t3;
        let _t6 = coll.toArray()?;
        let _t7 = _t6.clone()?;
        a = _t7;
        let mut local_4: Vec<Object> = a;
        let mut local_5: i32 = (local_4.len() as i32);
        let mut local_6: i32 = 0i32;
        loop {
            if local_6 >= local_5 { break; }
            let mut o: Object = local_4[local_6 as usize].clone();
            let _t0 = this.typeCheck(o)?;
            local_6 = local_6.wrapping_add(1i32);
        }
        let _t8: Object = Arrays::asList(&a)?;
        Ok(_t8)
    }

    // java: addAll(Ljava/util/Collection;)Z
    pub fn addAll(&self, coll: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.checkedCopyOf(coll)?;
        let _t1 = this.c.get().addAll(_t0)?;
        Ok(_t1)
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        let this = self;
        this.c.get().forEach(action)?;
        Ok(())
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.c.get().removeIf(filter)?;
        Ok(_t0)
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.c.get().spliterator()?;
        Ok(_t0)
    }

    // java: stream()Ljava/util/stream/Stream;
    pub fn stream(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.c.get().stream()?;
        Ok(_t0)
    }

    // java: parallelStream()Ljava/util/stream/Stream;
    pub fn parallelStream(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.c.get().parallelStream()?;
        Ok(_t0)
    }
}
