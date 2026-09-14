use java_runtime::prelude::*;
use super::*;

impl super::ArraysSupport {
    pub fn newLength(oldLength: i32, minGrowth: i32, prefGrowth: i32) -> Result<i32> {
        // Java 语义：新长度 = oldLength + max(minGrowth, prefGrowth)
        // 若结果溢出或超过 Integer.MAX_VALUE，退回到最小增长
        let pref_length = oldLength.saturating_add(minGrowth.max(prefGrowth));
        if pref_length > 0 {
            Ok(pref_length)
        } else {
            Ok(oldLength.saturating_add(minGrowth))
        }
    }
}
