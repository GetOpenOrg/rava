#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "sun/nio/cs/ArrayDecoder",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "ArrayDecoder.java",
))]
pub struct ArrayDecoder;

impl ArrayDecoder {
    #[cfg_attr(any(), java_native(name = "decode", descriptor = "([BII[C)I", access = "public abstract"))]
    pub fn decode(&self, arg0: Vec<i8>, arg1: i32, arg2: i32, arg3: Vec<u16>) -> Result<i32> {
        todo!("abstract sun/nio/cs/ArrayDecoder.decode")
    }

    #[cfg_attr(any(), java_method(name = "isASCIICompatible", descriptor = "()Z", access = "public"))]
    pub fn isASCIICompatible(&self) -> Result<bool> {
        let this = self;
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "isLatin1Decodable", descriptor = "()Z", access = "public"))]
    pub fn isLatin1Decodable(&self) -> Result<bool> {
        let this = self;
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "decodeToLatin1", descriptor = "([BII[B)I", access = "public"))]
    pub fn decodeToLatin1(&self, src: Vec<i8>, sp: i32, len: i32, dst: Vec<i8>) -> Result<i32> {
        let this = self;
        Ok(0i32)
    }
}
