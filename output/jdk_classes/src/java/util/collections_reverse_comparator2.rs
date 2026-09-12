#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections$ReverseComparator2",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Comparator,java/io/Serializable",
    access      = "",
    source      = "Collections.java",
))]
pub struct Collections_ReverseComparator2<T> {
    #[cfg_attr(any(), java_field(name = "cmp", descriptor = "Ljava/util/Comparator;", access = "final"))]
    pub cmp: Field<Object>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + 'static> Collections_ReverseComparator2<T> {
    // java: <init>(Ljava/util/Comparator;)V
    pub fn new(cmp: Object) -> Result<Self> {
        let this = Self { cmp: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        return Err(JvmError::Custom("athrow".to_owned()));
        this.cmp.set(cmp);
        Ok(this)
    }

    // java: compare(Ljava/lang/Object;Ljava/lang/Object;)I
    pub fn compare(&self, t1: T, t2: T) -> Result<i32> {
        let this = self;
        let _t0 = this.cmp.get().compare(t2, t1)?;
        Ok(_t0)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        let this = self;
        let mut that: Object = o;
        let _t0 = this.cmp.get().equals(that.cmp.get())?;
        Ok(_t0!=0i32)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.cmp.get().hashCode()?;
        Ok((_t0^-2147483648i32))
    }

    // java: reversed()Ljava/util/Comparator;
    pub fn reversed(&self) -> Result<Object> {
        let this = self;
        Ok(this.cmp.get())
    }
}
