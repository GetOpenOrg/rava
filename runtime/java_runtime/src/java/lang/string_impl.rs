use crate::prelude::*;
use super::string::String;

/// String 内容的驻留键：UTF-16 code units 序列（Java `String.intern` 按内容判等，
/// 与 coder 无关——同一内容可能是 LATIN1 或 UTF16 存储，键必须统一到 code units）。
///
/// 解码与写入侧字节序约定一致（`from_owned` / `StringUTF16.putChar` 的
/// HI/LO_BYTE_SHIFT：平台字节序）。
fn __intern_key(s: &String) -> Vec<u16> {
    let val = s.__get_value().to_vec();
    if s.__get_coder() == 0i8 {
        // LATIN1：每字节一 code unit
        val.iter().map(|b| *b as u8 as u16).collect()
    } else {
        // UTF16：每两字节一 code unit（平台字节序）
        (0..val.len() / 2)
            .map(|i| {
                let b0 = val[i * 2] as u8;
                let b1 = val[i * 2 + 1] as u8;
                if cfg!(target_endian = "big") {
                    u16::from_be_bytes([b0, b1])
                } else {
                    u16::from_le_bytes([b0, b1])
                }
            })
            .collect()
    }
}

/// 全局字符串驻留表（S-6）：内容（UTF-16 code units）→ 规范实例。
///
/// Java 语义（JLS §3.10.5 / JVMS §5.1 常量池解析）：相同内容的字符串字面量与
/// `intern()` 结果是同一对象（`==` 为 true）。字面量加载路径（`From<&str>`，
/// ldc 发射形态 `String::from("...")`）与本表同源；`from_owned`（拼接结果等
/// 非字面量构造）不入表——Java 中拼接产生新对象，不入常量池。
///
/// `String` 含 `Rc` 非 `Send`，且对象模型单线程协作调度（S-11），故 thread_local。
thread_local! {
    static __STRING_INTERN_TABLE: RefCell<std::collections::HashMap<Vec<u16>, String>> =
        RefCell::new(std::collections::HashMap::new());
}

impl String {
    /// 驻留表的内部访问形态（无 `Result` 包装，字面量构造路径共用）：返回该内容
    /// 的规范实例——表中已有则取同一实例（identity 相同），否则把 `self` 入表
    /// 成为规范实例。语义同 `intern()`。
    #[doc(hidden)]
    pub fn __interned(self) -> String {
        let key = __intern_key(&self);
        __STRING_INTERN_TABLE.with(|table| {
            let mut map = table.borrow_mut();
            if let Some(canon) = map.get(&key) {
                Clone::clone(canon)
            } else {
                map.insert(key, Clone::clone(&self));
                self
            }
        })
    }

    /// `java/lang/String.intern:()Ljava/lang/String;`（S-6）：按内容返回常量池
    /// 规范实例——与相同内容的字面量是同一对象。
    #[jvm_native]
    pub fn intern(&self) -> Result<String> {
        Ok(Clone::clone(self).__interned())
    }
}
