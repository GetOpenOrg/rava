#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Arrays$ArrayItr",
    super_class = "java/lang/Object",
    interfaces  = "java/util/Iterator",
    access      = "",
    source      = "Arrays.java",
))]
pub struct Arrays_ArrayItr<E> {
    #[cfg_attr(any(), java_field(name = "cursor", descriptor = "I", access = "private"))]
    pub cursor: Field<i32>,
    #[cfg_attr(any(), java_field(name = "a", descriptor = "[Ljava/lang/Object;", access = "private final"))]
    pub a: Field<Vec<Object>>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + 'static> Arrays_ArrayItr<E> {
    // java: <init>([Ljava/lang/Object;)V
    pub fn new(a: Vec<Object>) -> Result<Self> {
        let this = Self { cursor: Field::new(0), a: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.a.set(a);
        Ok(this)
    }

    // java: hasNext()Z
    pub fn hasNext(&self) -> Result<bool> {
        let this = self;
        Ok(this.cursor.get() < (this.a.get().len() as i32))
    }

    // java: next()Ljava/lang/Object;
    pub fn next(&self) -> Result<E> {
        let this = self;
        let mut i: i32 = this.cursor.get();
        return Err(JvmError::Custom("athrow".to_owned()));
        this.cursor.set((i).wrapping_add(1i32));
        Ok(this.a.get()[i as usize].clone())
    }
}
