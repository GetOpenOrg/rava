#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/CollSer",
    super_class = "java/lang/Object",
    interfaces  = "java/io/Serializable",
    access      = "final",
    source      = "ImmutableCollections.java",
))]
pub struct CollSer {
    #[cfg_attr(any(), java_field(name = "tag", descriptor = "I", access = "private final"))]
    pub tag: Field<i32>,
    #[cfg_attr(any(), java_field(name = "array", descriptor = "[Ljava/lang/Object;", access = "private"))]
    pub array: Field<Vec<Object>>,
}

impl CollSer {
    // java: <init>(I[Ljava/lang/Object;)V
    pub fn new(&self, t: i32, a: Vec<Object>) -> Result<()> {
        todo!("abstract java/util/CollSer.<init>")
    }

    // java: readObject(Ljava/io/ObjectInputStream;)V
    pub fn readObject(&self, ois: Object) -> Result<()> {
        todo!("abstract java/util/CollSer.readObject")
    }

    // java: writeObject(Ljava/io/ObjectOutputStream;)V
    pub fn writeObject(&self, oos: Object) -> Result<()> {
        todo!("abstract java/util/CollSer.writeObject")
    }

    // java: readResolve()Ljava/lang/Object;
    pub fn readResolve(&self) -> Result<Object> {
        todo!("abstract java/util/CollSer.readResolve")
    }
}
