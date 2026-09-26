//! JCA 服务注册表（K-JCA，`docs/plans/2026-09-25-jca-service-registry.md`）。
//!
//! JDK 的 `Provider$Service.newInstance` 按类名反射构造服务实现类（`DESCipher`、`MD5` 等，
//! 无静态调用边）。原生侧在「构造」这一跳截断：codegen 从 provider 注册字节码抽取服务表，
//! 对入选服务（engine 类在调用链上 × 算法名是用户字符串常量，见 `codegen/jca_services.py`）
//! 的实现类照常翻译字节码，并在生成 main 启动时经 [`register_services`] 登记
//! `(类型, 算法, 实现类, provider, 构造闭包)`——与 data_bundles / reflect_dispatch 同一登记模式。
//! 服务查找（`sun/security/jca` 手写边界）按 (类型, 算法) 大小写不敏感匹配本表。

use crate::error::Result;
use crate::java::lang::Object;

/// 服务实现类构造闭包（调用翻译出的无参构造器）。
pub type ServiceCtor = fn() -> Result<Object>;

/// 一条已登记的服务。
#[derive(Clone, Copy, Debug)]
pub struct ServiceEntry {
    pub type_: &'static str,
    pub algorithm: &'static str,
    /// 实现类 binary name（斜线形态）
    pub class_name: &'static str,
    pub provider: &'static str,
    pub ctor: ServiceCtor,
}

crate::__process_static! {
    static SERVICES: crate::sync_model::__RefSlot<Vec<ServiceEntry>> = crate::sync_model::__RefSlot::new(Vec::new());
}

/// 生成项目 main 启动时登记入选服务（同 (类型, 算法, provider) 重登记幂等）。
pub fn register_services(services: &[(&'static str, &'static str, &'static str, &'static str, ServiceCtor)]) {
    SERVICES.with(|s| {
        let mut s = s.borrow_mut();
        for &(type_, algorithm, class_name, provider, ctor) in services {
            s.retain(|e| !(e.type_ == type_ && e.algorithm == algorithm && e.provider == provider));
            s.push(ServiceEntry { type_, algorithm, class_name, provider, ctor });
        }
    });
}

/// 按 (类型, 算法) 查服务（算法名大小写不敏感，JDK `Provider.getService` 同语义）；
/// 登记序即 provider 优先序。
pub fn find(type_: &str, algorithm: &str) -> Option<ServiceEntry> {
    SERVICES.with(|s| {
        s.borrow().iter()
            .find(|e| e.type_ == type_ && e.algorithm.eq_ignore_ascii_case(algorithm))
            .copied()
    })
}

/// 某 provider 名下按 (类型, 算法) 查服务。
pub fn find_in(provider: &str, type_: &str, algorithm: &str) -> Option<ServiceEntry> {
    SERVICES.with(|s| {
        s.borrow().iter()
            .find(|e| e.provider == provider && e.type_ == type_
                  && e.algorithm.eq_ignore_ascii_case(algorithm))
            .copied()
    })
}
