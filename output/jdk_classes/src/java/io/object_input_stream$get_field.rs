#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/ObjectInputStream$GetField",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "ObjectInputStream.java",
))]
pub struct ObjectInputStream_GetField;

impl ObjectInputStream_GetField {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_native(name = "getObjectStreamClass", descriptor = "()Ljava/io/ObjectStreamClass;", access = "public abstract"))]
    pub fn getObjectStreamClass(&self) -> Result<Object> {
        todo!("abstract java/io/ObjectInputStream$GetField.getObjectStreamClass")
    }

    #[cfg_attr(any(), java_native(name = "defaulted", descriptor = "(Ljava/lang/String;)Z", access = "public abstract"))]
    pub fn defaulted(&self, arg0: String) -> Result<bool> {
        todo!("abstract java/io/ObjectInputStream$GetField.defaulted")
    }

    #[cfg_attr(any(), java_native(name = "get", descriptor = "(Ljava/lang/String;Z)Z", access = "public abstract"))]
    pub fn get__str_z(&self, arg0: String, arg1: bool) -> Result<bool> {
        todo!("abstract java/io/ObjectInputStream$GetField.get")
    }

    #[cfg_attr(any(), java_native(name = "get", descriptor = "(Ljava/lang/String;B)B", access = "public abstract"))]
    pub fn get__str_b(&self, arg0: String, arg1: i8) -> Result<i8> {
        todo!("abstract java/io/ObjectInputStream$GetField.get")
    }

    #[cfg_attr(any(), java_native(name = "get", descriptor = "(Ljava/lang/String;C)C", access = "public abstract"))]
    pub fn get__str_c(&self, arg0: String, arg1: u16) -> Result<u16> {
        todo!("abstract java/io/ObjectInputStream$GetField.get")
    }

    #[cfg_attr(any(), java_native(name = "get", descriptor = "(Ljava/lang/String;S)S", access = "public abstract"))]
    pub fn get__str_s(&self, arg0: String, arg1: i16) -> Result<i16> {
        todo!("abstract java/io/ObjectInputStream$GetField.get")
    }

    #[cfg_attr(any(), java_native(name = "get", descriptor = "(Ljava/lang/String;I)I", access = "public abstract"))]
    pub fn get__str_i(&self, arg0: String, arg1: i32) -> Result<i32> {
        todo!("abstract java/io/ObjectInputStream$GetField.get")
    }

    #[cfg_attr(any(), java_native(name = "get", descriptor = "(Ljava/lang/String;J)J", access = "public abstract"))]
    pub fn get__str_l(&self, arg0: String, arg1: i64) -> Result<i64> {
        todo!("abstract java/io/ObjectInputStream$GetField.get")
    }

    #[cfg_attr(any(), java_native(name = "get", descriptor = "(Ljava/lang/String;F)F", access = "public abstract"))]
    pub fn get__str_f(&self, arg0: String, arg1: f32) -> Result<f32> {
        todo!("abstract java/io/ObjectInputStream$GetField.get")
    }

    #[cfg_attr(any(), java_native(name = "get", descriptor = "(Ljava/lang/String;D)D", access = "public abstract"))]
    pub fn get__str_d(&self, arg0: String, arg1: f64) -> Result<f64> {
        todo!("abstract java/io/ObjectInputStream$GetField.get")
    }

    #[cfg_attr(any(), java_native(name = "get", descriptor = "(Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/Object;", access = "public abstract"))]
    pub fn get__str_obj(&self, arg0: String, arg1: Object) -> Result<Object> {
        todo!("abstract java/io/ObjectInputStream$GetField.get")
    }
}
