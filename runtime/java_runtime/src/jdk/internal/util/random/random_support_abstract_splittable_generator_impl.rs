//! `jdk/internal/util/random/RandomSupport$AbstractSplittableGenerator` 手写伴生：
//! 内部边界类，按调用链按需实现（K-2 规则），其余保持 panic 存根。

use crate::prelude::*;
use super::random_support_abstract_splittable_generator::RandomSupport_AbstractSplittableGenerator;

impl RandomSupport_AbstractSplittableGenerator {
    /// `<init>()V`：隐式无参构造器（字节码仅 `invokespecial` 超类构造后返回，
    /// 类自身无状态；超类 `AbstractSpliteratorGenerator.<init>` 同为无状态链，
    /// 运行时模型里 Object 初始化是 no-op——与翻译侧对 `Object.<init>` 的省略同源）。
    ///
    /// 消费方：`SplittableRandom$AbstractSplittableGeneratorProxy.<init>` 的
    /// invokespecial super()（proxy 经 jdk/ 边界继承进入调用链）。
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Ok(this)
    }

    /// 构造器双入口的 `this` 形态（invokespecial super() 侧的落点）：
    /// 无状态初始化，幂等返回。
    #[doc(hidden)]
    pub fn __init_on(this: Self) -> Result<Self> {
        let _ = &this;
        Ok(this)
    }
}
