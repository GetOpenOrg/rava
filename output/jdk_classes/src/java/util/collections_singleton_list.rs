#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$SingletonList",
    super_class = "java/util/AbstractList",
    interfaces  = "java/util/RandomAccess,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_SingletonList<E> {
    #[cfg_attr(any(), java_field(name = "element", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub element: Field<Object>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Collections_SingletonList<E> {
    // java: <init>(Ljava/lang/Object;)V
    pub fn new(obj: E) -> Result<Self> {
        let this = Self { element: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/AbstractList.<init>:()V */
        this.element.set(obj);
        Ok(this)
    }

    // java: iterator()Ljava/util/Iterator;
    pub fn iterator(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Collections::singletonIterator(this.element.get())?;
        Ok(_t0)
    }

    // java: size()I
    pub fn size(&self) -> Result<i32> {
        let this = self;
        Ok(1i32)
    }

    // java: contains(Ljava/lang/Object;)Z
    pub fn contains(&self, obj: Object) -> Result<bool> {
        let this = self;
        let _t0: bool = Collections::eq(obj, this.element.get())?;
        Ok(_t0)
    }

    // java: get(I)Ljava/lang/Object;
    pub fn get(&self, index: i32) -> Result<E> {
        let this = self;
        String::new().append(&String::from("Index:"))?;
        String::new().append(&index)?;
        String::new().append(&String::from(", Size: 1"))?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(this.element.get())
    }

    // java: forEach(Ljava/util/function/Consumer;)V
    pub fn forEach(&self, action: Object) -> Result<()> {
        let this = self;
        action.accept(this.element.get())?;
        Ok(())
    }

    // java: removeIf(Ljava/util/function/Predicate;)Z
    pub fn removeIf(&self, filter: Object) -> Result<bool> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: replaceAll(Ljava/util/function/UnaryOperator;)V
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: sort(Ljava/util/Comparator;)V
    pub fn sort(&self, c: Object) -> Result<()> {
        let this = self;
        Ok(())
    }

    // java: spliterator()Ljava/util/Spliterator;
    pub fn spliterator(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Collections::singletonSpliterator(this.element.get())?;
        Ok(_t0)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0: i32 = Objects::hashCode(this.element.get())?;
        Ok((31i32).wrapping_add(_t0))
    }
}
