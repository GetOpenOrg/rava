#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Character$UnicodeBlock",
    super_class = "java/lang/Character$Subset",
    interfaces  = "",
    access      = "public final",
    source      = "Character.java",
))]
pub struct Character_UnicodeBlock;

impl Character_UnicodeBlock {
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(&self, idName: String) -> Result<()> {
        panic!("stub: java/lang/Character$UnicodeBlock.<init>:(Ljava/lang/String;)V")
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/String;)V
    pub fn new__str_str(&self, idName: String, alias: String) -> Result<()> {
        panic!("stub: java/lang/Character$UnicodeBlock.<init>:(Ljava/lang/String;Ljava/lang/String;)V")
    }

    // java: <init>(Ljava/lang/String;[Ljava/lang/String;)V
    pub fn new__str_arr_str(&self, idName: String, aliases: Vec<String>) -> Result<()> {
        panic!("stub: java/lang/Character$UnicodeBlock.<init>:(Ljava/lang/String;[Ljava/lang/String;)V")
    }

    // java: of(C)Ljava/lang/Character$UnicodeBlock;
    pub fn of__c(c: u16) -> Result<Object> {
        panic!("stub: java/lang/Character$UnicodeBlock.of:(C)Ljava/lang/Character$UnicodeBlock;")
    }

    // java: of(I)Ljava/lang/Character$UnicodeBlock;
    pub fn of__i(codePoint: i32) -> Result<Object> {
        panic!("stub: java/lang/Character$UnicodeBlock.of:(I)Ljava/lang/Character$UnicodeBlock;")
    }

    // java: forName(Ljava/lang/String;)Ljava/lang/Character$UnicodeBlock;
    pub fn forName(blockName: String) -> Result<Object> {
        panic!("stub: java/lang/Character$UnicodeBlock.forName:(Ljava/lang/String;)Ljava/lang/Character$UnicodeBlock;")
    }
}
