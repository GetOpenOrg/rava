#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/ObjectOutputStream$PutField",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "ObjectOutputStream.java",
))]
pub struct ObjectOutputStream_PutField;

impl ObjectOutputStream_PutField {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_native(name = "put", descriptor = "(Ljava/lang/String;Z)V", access = "public abstract"))]
    pub fn put__str_z(&self, arg0: String, arg1: bool) -> Result<()> {
        todo!("abstract java/io/ObjectOutputStream$PutField.put")
    }

    #[cfg_attr(any(), java_native(name = "put", descriptor = "(Ljava/lang/String;B)V", access = "public abstract"))]
    pub fn put__str_b(&self, arg0: String, arg1: i8) -> Result<()> {
        todo!("abstract java/io/ObjectOutputStream$PutField.put")
    }

    #[cfg_attr(any(), java_native(name = "put", descriptor = "(Ljava/lang/String;C)V", access = "public abstract"))]
    pub fn put__str_c(&self, arg0: String, arg1: u16) -> Result<()> {
        todo!("abstract java/io/ObjectOutputStream$PutField.put")
    }

    #[cfg_attr(any(), java_native(name = "put", descriptor = "(Ljava/lang/String;S)V", access = "public abstract"))]
    pub fn put__str_s(&self, arg0: String, arg1: i16) -> Result<()> {
        todo!("abstract java/io/ObjectOutputStream$PutField.put")
    }

    #[cfg_attr(any(), java_native(name = "put", descriptor = "(Ljava/lang/String;I)V", access = "public abstract"))]
    pub fn put__str_i(&self, arg0: String, arg1: i32) -> Result<()> {
        todo!("abstract java/io/ObjectOutputStream$PutField.put")
    }

    #[cfg_attr(any(), java_native(name = "put", descriptor = "(Ljava/lang/String;J)V", access = "public abstract"))]
    pub fn put__str_l(&self, arg0: String, arg1: i64) -> Result<()> {
        todo!("abstract java/io/ObjectOutputStream$PutField.put")
    }

    #[cfg_attr(any(), java_native(name = "put", descriptor = "(Ljava/lang/String;F)V", access = "public abstract"))]
    pub fn put__str_f(&self, arg0: String, arg1: f32) -> Result<()> {
        todo!("abstract java/io/ObjectOutputStream$PutField.put")
    }

    #[cfg_attr(any(), java_native(name = "put", descriptor = "(Ljava/lang/String;D)V", access = "public abstract"))]
    pub fn put__str_d(&self, arg0: String, arg1: f64) -> Result<()> {
        todo!("abstract java/io/ObjectOutputStream$PutField.put")
    }

    #[cfg_attr(any(), java_native(name = "put", descriptor = "(Ljava/lang/String;Ljava/lang/Object;)V", access = "public abstract"))]
    pub fn put__str_obj(&self, arg0: String, arg1: Object) -> Result<()> {
        todo!("abstract java/io/ObjectOutputStream$PutField.put")
    }

    #[cfg_attr(any(), java_native(name = "write", descriptor = "(Ljava/io/ObjectOutput;)V", access = "public abstract"))]
    pub fn write(&self, arg0: Object) -> Result<()> {
        todo!("abstract java/io/ObjectOutputStream$PutField.write")
    }
}
