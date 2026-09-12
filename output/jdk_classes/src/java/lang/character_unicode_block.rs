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
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(idName: String) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Character$Subset.<init>:(Ljava/lang/String;)V */
        let _t0 = Character_UnicodeBlock::map().put(idName, this)?;
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;Ljava/lang/String;)V
    pub fn new__str_str(idName: String, alias: String) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Character$UnicodeBlock.<init>:(Ljava/lang/String;)V */
        let _t0 = Character_UnicodeBlock::map().put(alias, this)?;
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;[Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;[Ljava/lang/String;)V
    pub fn new__str_arr_str(idName: String, aliases: Vec<String>) -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Character$UnicodeBlock.<init>:(Ljava/lang/String;)V */
        let mut local_3: Vec<String> = aliases;
        let mut local_4: i32 = (local_3.len() as i32);
        let mut local_5: i32 = 0i32;
        loop {
            if local_5 >= local_4 { break; }
            let mut alias: String = local_3[local_5 as usize].clone();
            let _t0 = Character_UnicodeBlock::map().put(alias, this)?;
            local_5 = local_5.wrapping_add(1i32);
        }
        Ok(this)
    }

    // java: of(C)Ljava/lang/Character$UnicodeBlock;
    // java: of(C)Ljava/lang/Character$UnicodeBlock;
    pub fn of__c(c: u16) -> Result<Object> {
        let _t0: Object = Character_UnicodeBlock::of(c)?;
        Ok(_t0)
    }

    // java: of(I)Ljava/lang/Character$UnicodeBlock;
    // java: of(I)Ljava/lang/Character$UnicodeBlock;
    pub fn of__i(codePoint: i32) -> Result<Object> {
        let _t0: bool = Character::isValidCodePoint(codePoint)?;
        let mut _arr1: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr1[0i32 as usize] = codePoint;
        let _t2: String = String::format(String::from("Not a valid Unicode code point: 0x%X"), &_arr1)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut bottom: i32 = 0i32;
        let mut top: i32 = (Character_UnicodeBlock::blockStarts().len() as i32);
        let mut current: i32 = (top/2i32);
        loop {
            if (top).wrapping_sub(bottom) <= 1i32 { break; }
            bottom = current;
            top = current;
            current = ((top).wrapping_add(bottom)/2i32);
        }
        Ok(Character_UnicodeBlock::blocks()[current as usize].clone())
    }

    // java: forName(Ljava/lang/String;)Ljava/lang/Character$UnicodeBlock;
    pub fn forName(blockName: String) -> Result<Object> {
        let _t0 = blockName.toUpperCase(Locale::US())?;
        let _t1 = Character_UnicodeBlock::map().get(_t0)?;
        let mut block: Object = _t1;
        String::new().append(&String::from("Not a valid block name:"))?;
        String::new().append(&blockName)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(block)
    }
}
