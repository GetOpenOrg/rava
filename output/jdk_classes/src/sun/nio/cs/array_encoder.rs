#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "sun/nio/cs/ArrayEncoder",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "ArrayEncoder.java",
))]
pub struct ArrayEncoder;

impl ArrayEncoder {
    #[cfg_attr(any(), java_native(name = "encode", descriptor = "([CII[B)I", access = "public abstract"))]
    pub fn encode(&self, arg0: Vec<u16>, arg1: i32, arg2: i32, arg3: Vec<i8>) -> Result<i32> {
        todo!("abstract sun/nio/cs/ArrayEncoder.encode")
    }

    #[cfg_attr(any(), java_method(name = "encodeFromLatin1", descriptor = "([BII[B)I", access = "public"))]
    pub fn encodeFromLatin1(&self, src: Vec<i8>, sp: i32, len: i32, dst: Vec<i8>) -> Result<i32> {
        let this = self;
        Ok(-1i32)
    }

    #[cfg_attr(any(), java_method(name = "encodeFromUTF16", descriptor = "([BII[B)I", access = "public"))]
    pub fn encodeFromUTF16(&self, src: Vec<i8>, sp: i32, len: i32, dst: Vec<i8>) -> Result<i32> {
        let this = self;
        Ok(-1i32)
    }

    #[cfg_attr(any(), java_method(name = "isASCIICompatible", descriptor = "()Z", access = "public"))]
    pub fn isASCIICompatible(&self) -> Result<bool> {
        let this = self;
        Ok(0i32)
    }
}
