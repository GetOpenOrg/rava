//! `sun/reflect/generics/repository/ClassRepository` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! 泛型签名解析器（sun/reflect/generics）尚未放行（随 C1d 边界收窄处理），
//! `Class.getGenericSignature0` 暂返回 null。本伴生只提供 `NONE` 哨兵：
//! `Class.getGenericInfo` 以 `genericInfo != NONE` 判断「有泛型信息」，哨兵须为非 null 的唯一实例。

use crate::prelude::*;
use super::class_repository::ClassRepository;

impl ClassRepository {
    /// static `NONE`：「无泛型签名」哨兵（JDK：`ClassRepository.make("Ljava/lang/Object;", null)`）。
    /// 进程内唯一实例，只用于身份比较。
    #[jvm_boundary]
    pub fn NONE() -> Result<ClassRepository> {
        crate::__process_static! {
            static SENTINEL: ClassRepository = {
                let mut r = ClassRepository::default();
                r._init_not_null();
                r
            };
        }
        Ok(SENTINEL.with(Clone::clone))
    }
}
