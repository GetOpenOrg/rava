//! `java/security/Provider` 手写伴生（Provider 本体按字节码翻译，本文件只有过渡辅助）。
//!
//! Provider 按字节码翻译（seeds.toml [jca] release_classes）；JCA 服务查找的 provider 对象由
//! `crate::jca::provider` 经翻译的 Provider 子类构造器创建（docs/plans/2026-09-28-jca-faithful-provider.md）。
//!
//! 过渡：`__for_name` 只供 SecureRandom 手写伴生报告缺省 provider（SUN）身份，随 SecureRandom
//! 回到字节码（缺省 PRNG 经 provider 列表选取）删除。

use crate::prelude::*;
use super::provider::implref::Provider;
use std::collections::HashMap;

crate::__process_static! {
    static PROVIDERS: crate::sync_model::__RefSlot<HashMap<&'static str, Provider>> =
        crate::sync_model::__RefSlot::new(HashMap::new());
}

impl Provider {
    /// provider 名 → 进程内唯一的身份对象（首次取用时构造，仅设 name 字段）。
    pub fn __for_name(name: &'static str) -> Provider {
        PROVIDERS.with(|p| {
            let mut p = p.borrow_mut();
            let prov = p.entry(name).or_insert_with(|| {
                let mut prov = Provider::default();
                prov._init_not_null();
                prov.__set_name(String::from(name));
                prov
            });
            Clone::clone(&*prov)
        })
    }
}
