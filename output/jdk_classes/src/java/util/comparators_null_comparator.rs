#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Comparators$NullComparator",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Comparator,java/io/Serializable",
    access      = "final",
    source      = "Comparators.java",
))]
pub struct Comparators_NullComparator<T> {
    #[cfg_attr(any(), java_field(name = "nullFirst", descriptor = "Z", access = "private final"))]
    pub nullFirst: Field<bool>,
    #[cfg_attr(any(), java_field(name = "real", descriptor = "Ljava/util/Comparator;", access = "private final"))]
    pub real: Field<Object>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + 'static> Comparators_NullComparator<T> {
    // java: <init>(ZLjava/util/Comparator;)V
    pub fn new(nullFirst: bool, real: Object) -> Result<Self> {
        let this = Self { nullFirst: Field::new(false), real: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.nullFirst.set(nullFirst);
        this.real.set(real);
        Ok(this)
    }

    // java: compare(Ljava/lang/Object;Ljava/lang/Object;)I
    pub fn compare(&self, a: T, b: T) -> Result<i32> {
        let this = self;
        return Ok(this.nullFirst.get()==0i32);
        return Ok(this.nullFirst.get()==0i32);
        let _t0 = this.real.get().compare(a, b)?;
        Ok(_t0)
    }

    // java: thenComparing(Ljava/util/Comparator;)Ljava/util/Comparator;
    pub fn thenComparing(&self, other: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull__obj(other)?;
        let _t1 = this.real.get().thenComparing(other)?;
        /* invokespecial Method java/util/Comparators$NullComparator.<init>:(ZLjava/util/Comparator;)V */
        Ok(this.nullFirst.get())
    }

    // java: reversed()Ljava/util/Comparator;
    pub fn reversed(&self) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        let _t0 = this.real.get().reversed()?;
        /* invokespecial Method java/util/Comparators$NullComparator.<init>:(ZLjava/util/Comparator;)V */
        Ok(Comparators_NullComparator::new())
    }
}
