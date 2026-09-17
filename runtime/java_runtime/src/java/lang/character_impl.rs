use crate::prelude::*;
use super::*;

impl Character {
    // Character.digit(int, int) 的 ASCII 实现，绕过尚未翻译的 CharacterData 层次
    #[jvm_native]
    pub fn digit_i_i(codePoint: i32, radix: i32) -> Result<i32> {
        if radix < 2 || radix > 36 {
            return Ok(-1i32);
        }
        let d = if codePoint >= 0x30 && codePoint <= 0x39 {
            codePoint - 0x30
        } else if codePoint >= 0x61 && codePoint <= 0x7A {
            codePoint - 0x61 + 10
        } else if codePoint >= 0x41 && codePoint <= 0x5A {
            codePoint - 0x41 + 10
        } else {
            -1
        };
        Ok(if d >= 0 && d < radix { d } else { -1i32 })
    }
}
