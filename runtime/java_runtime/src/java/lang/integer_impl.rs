//! `java/lang/Integer` 手写实现（仅当 `integer.rs` 进入闭包生成时编译，见 K-2 规则）。

use crate::prelude::*;
use super::*;

impl Integer {
    /// `Integer.valueOf(int)`（P-3 扩清单路线的按需手写先例；终态 T-4 包装类
    /// 走生成后本件退役）。
    ///
    /// JDK 语义（`Integer.valueOf` 的 IntegerCache 路径）：[-128, 127] 返回缓存
    /// 实例——同值两次装箱身份相等（`TestAutoboxEdge` 的 `127eq=true` 与
    /// `128eq=false` / `valueOf128eq=false` 即此语义的两面）；范围外每次新实例。
    /// 原生侧以线程局部缓存等价承载：`Integer` 的 Clone 共享 `__identity`
    /// 标识单元，缓存取值即同身份。缓存上界的系统属性调参
    /// （`java.lang.Integer.IntegerCache.high`）不在运行时语义面内，取 JDK
    /// 默认 [-128, 127]。
    #[jvm_native]
    pub fn valueOf_i(i: i32) -> Result<Integer> {
        crate::__process_static! {
            static CACHE: RefCell<Vec<Integer>> = RefCell::new(
                (-128i32..=127i32).map(|v| {
                    let mut boxed = Integer::default();
                    boxed._init_not_null();
                    boxed.__set_value(v);
                    boxed
                }).collect());
        }
        if (-128i32..=127i32).contains(&i) {
            return Ok(CACHE.with(|cache| {
                Clone::clone(&cache.borrow()[(i + 128) as usize])
            }));
        }
        let mut boxed = Integer::default();
        boxed._init_not_null();
        boxed.__set_value(i);
        Ok(boxed)
    }

    /// `Integer.toString()`（实例版本，`valueOf` 装箱值的最直接消费面——
    /// `println(Object)` / 字符串拼接都经此）。JDK 语义：十进制表示，
    /// 与 Rust `i32` 的 Display 逐字符一致（含 `MIN_VALUE` 的 `-2147483648`）。
    #[jvm_native]
    pub fn __impl_toString(&self) -> Result<String> {
        Ok(String::from(std::format!("{}", self.__get_value()).as_str()))
    }
}
