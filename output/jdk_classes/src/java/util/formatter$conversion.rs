#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Formatter$Conversion",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "",
    source      = "Formatter.java",
))]
pub struct Formatter_Conversion;

impl Formatter_Conversion {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V"))]
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "isValid", descriptor = "(C)Z", access = "static"))]
    pub fn isValid(c: u16) -> Result<bool> {
        /* TODO: tableswitch default:244 low:65 high:120 */
        Ok(c == 37i32)
    }

    #[cfg_attr(any(), java_method(name = "isGeneral", descriptor = "(C)Z", access = "static"))]
    pub fn isGeneral(c: u16) -> Result<bool> {
        /* TODO: lookupswitch default:64 66:60 72:60 83:60 98:60 104:60 115:60 */
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "isCharacter", descriptor = "(C)Z", access = "static"))]
    pub fn isCharacter(c: u16) -> Result<bool> {
        /* TODO: lookupswitch default:32 67:28 99:28 */
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "isInteger", descriptor = "(C)Z", access = "static"))]
    pub fn isInteger(c: u16) -> Result<bool> {
        /* TODO: lookupswitch default:48 88:44 100:44 111:44 120:44 */
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "isFloat", descriptor = "(C)Z", access = "static"))]
    pub fn isFloat(c: u16) -> Result<bool> {
        /* TODO: lookupswitch default:72 65:68 69:68 71:68 97:68 101:68 102:68 103:68 */
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "isText", descriptor = "(C)Z", access = "static"))]
    pub fn isText(c: u16) -> Result<bool> {
        /* TODO: lookupswitch default:32 37:28 110:28 */
        Ok(0i32)
    }
}
