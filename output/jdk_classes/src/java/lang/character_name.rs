#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/CharacterName",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "",
    source      = "CharacterName.java",
))]
pub struct CharacterName {
    #[cfg_attr(any(), java_field(name = "strPool", descriptor = "[B", access = "private final"))]
    pub strPool: Field<Vec<i8>>,
    #[cfg_attr(any(), java_field(name = "lookup", descriptor = "[I", access = "private final"))]
    pub lookup: Field<Vec<i32>>,
    #[cfg_attr(any(), java_field(name = "bkIndices", descriptor = "[I", access = "private final"))]
    pub bkIndices: Field<Vec<i32>>,
    #[cfg_attr(any(), java_field(name = "cpEntries", descriptor = "[I", access = "private final"))]
    pub cpEntries: Field<Vec<i32>>,
    #[cfg_attr(any(), java_field(name = "hsIndices", descriptor = "[I", access = "private final"))]
    pub hsIndices: Field<Vec<i32>>,
}

impl CharacterName {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        panic!("stub: java/lang/CharacterName.<init>:()V")
    }

    // java: hashN([BII)I
    pub fn hashN(a: Vec<i8>, off: i32, len: i32) -> Result<i32> {
        panic!("stub: java/lang/CharacterName.hashN:([BII)I")
    }

    // java: addCp(IIII)I
    pub fn addCp(&self, idx: i32, hash: i32, next: i32, cp: i32) -> Result<i32> {
        panic!("stub: java/lang/CharacterName.addCp:(IIII)I")
    }

    // java: getCpHash(I)I
    pub fn getCpHash(&self, idx: i32) -> Result<i32> {
        panic!("stub: java/lang/CharacterName.getCpHash:(I)I")
    }

    // java: getCpNext(I)I
    pub fn getCpNext(&self, idx: i32) -> Result<i32> {
        panic!("stub: java/lang/CharacterName.getCpNext:(I)I")
    }

    // java: getCp(I)I
    pub fn getCp(&self, idx: i32) -> Result<i32> {
        panic!("stub: java/lang/CharacterName.getCp:(I)I")
    }

    // java: getInstance()Ljava/lang/CharacterName;
    pub fn getInstance() -> Result<Object> {
        panic!("stub: java/lang/CharacterName.getInstance:()Ljava/lang/CharacterName;")
    }

    // java: getName(I)Ljava/lang/String;
    pub fn getName(&self, cp: i32) -> Result<String> {
        panic!("stub: java/lang/CharacterName.getName:(I)Ljava/lang/String;")
    }

    // java: getCodePoint(Ljava/lang/String;)I
    pub fn getCodePoint(&self, name: String) -> Result<i32> {
        panic!("stub: java/lang/CharacterName.getCodePoint:(Ljava/lang/String;)I")
    }
}
