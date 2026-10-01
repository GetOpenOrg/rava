//! JCA 服务注册表（K-JCA，`docs/plans/2026-09-25-jca-service-registry.md`、
//! `docs/plans/2026-09-28-jca-faithful-provider.md`）。
//!
//! JDK 的 provider 列表（ProviderList / ProviderConfig）在运行期按配置装载 provider，
//! 服务实现类由 `Provider$Service.newInstance` 按类名反射构造，都无静态调用边。
//! 生成器从 provider 注册字节码抽取服务表，对入选服务（engine 类在调用链上 × 算法名是
//! 用户字符串常量，见 `generator/crates/closure/src/seeds/jca.rs`）翻译实现类字节码并登记反射构造面；生成
//! main 启动时经 [`register_services`] 登记 `(类型, 算法, 实现类, provider)`，经
//! [`register_providers`] 登记 provider 构造闭包（JDK ProviderConfig 对内建 provider 同样
//! 直接 `new`）。
//!
//! 手写边界 `sun/security/jca` 只用本表决定「问哪些 provider」：provider 对象按需构造一次
//!（ProviderList 同样缓存 provider 实例），服务描述、属性与实现类构造全部经翻译字节码
//!（`Provider.getService` / `Provider$Service.newInstance`）。

use crate::error::Result;
use crate::java::lang::Object;

/// provider 构造闭包（调用翻译出的 Provider 子类无参构造器）。本模块恒编译、Provider 类未必
/// 在闭包内，故以 Object 承载（手写边界侧还原 Provider 视图）。
pub type ProviderCtor = fn() -> Result<Object>;

/// 一条已登记的服务（服务表只作 provider 选择依据）。
#[derive(Clone, Copy, Debug)]
pub struct ServiceEntry {
    pub type_: &'static str,
    pub algorithm: &'static str,
    /// 实现类 binary name（斜线形态）
    pub class_name: &'static str,
    pub provider: &'static str,
}

crate::__process_static! {
    static SERVICES: crate::sync_model::__RefSlot<Vec<ServiceEntry>> = crate::sync_model::__RefSlot::new(Vec::new());
}

crate::__process_static! {
    static PROVIDER_CTORS: crate::sync_model::__RefSlot<Vec<(&'static str, ProviderCtor)>> =
        crate::sync_model::__RefSlot::new(Vec::new());
}

crate::__process_static! {
    static PROVIDERS: crate::sync_model::__RefSlot<Vec<(&'static str, Object)>> =
        crate::sync_model::__RefSlot::new(Vec::new());
}

/// 生成项目 main 启动时登记入选服务（同 (类型, 算法, provider) 重登记幂等）。
pub fn register_services(services: &[(&'static str, &'static str, &'static str, &'static str)]) {
    SERVICES.with(|s| {
        let mut s = s.borrow_mut();
        for &(type_, algorithm, class_name, provider) in services {
            s.retain(|e| !(e.type_ == type_ && e.algorithm == algorithm && e.provider == provider));
            s.push(ServiceEntry { type_, algorithm, class_name, provider });
        }
    });
}

/// 生成项目 main 启动时登记 provider 构造闭包（重登记幂等）。
pub fn register_providers(ctors: &[(&'static str, ProviderCtor)]) {
    PROVIDER_CTORS.with(|c| {
        let mut c = c.borrow_mut();
        for &(name, ctor) in ctors {
            c.retain(|(n, _)| *n != name);
            c.push((name, ctor));
        }
    });
}

/// 按 (类型, 算法) 登记了服务的 provider 名（算法名大小写不敏感，JDK `Provider.getService`
/// 同语义；登记序即 provider 优先序，去重）。
///
/// 按标准名无匹配时（算法名是别名，如 `sun.security.provider.SecureRandom.init` 的
/// `MessageDigest.getInstance("SHA")`，服务表登记为标准名 `SHA-1`）退回该类型的全部 provider：
/// 别名由各 provider 的 `getService`（翻译字节码，查 legacy `Alg.Alias.*` 映射）解析，与 JDK
/// ProviderList 逐个 provider 询问同语义；不提供该算法的 provider 返回 null，由调用方滤除。
pub fn providers_for(type_: &str, algorithm: &str) -> Vec<&'static str> {
    SERVICES.with(|s| {
        let s = s.borrow();
        let collect = |exact: bool| {
            let mut out: Vec<&'static str> = Vec::new();
            for e in s.iter() {
                if e.type_ == type_ && (!exact || e.algorithm.eq_ignore_ascii_case(algorithm)) && !out.contains(&e.provider) {
                    out.push(e.provider);
                }
            }
            out
        };
        let exact = collect(true);
        if exact.is_empty() { collect(false) } else { exact }
    })
}

/// provider 名 → provider 对象（首次取用时经登记闭包构造，其后复用同一实例——ProviderList
/// 中 provider 实例唯一，`==` 比较成立）。未登记 → None。
pub fn provider(name: &str) -> Result<Option<Object>> {
    if let Some(p) = PROVIDERS.with(|p| p.borrow().iter().find(|(n, _)| *n == name).map(|(_, p)| Clone::clone(p))) {
        return Ok(Some(p));
    }
    let Some((key, ctor)) = PROVIDER_CTORS.with(|c| c.borrow().iter().find(|(n, _)| *n == name).copied()) else {
        return Ok(None);
    };
    // 构造在锁外进行（provider 构造器执行翻译字节码，可能重入本表）
    let made = ctor()?;
    Ok(Some(PROVIDERS.with(|p| {
        let mut p = p.borrow_mut();
        if let Some((_, existing)) = p.iter().find(|(n, _)| *n == key) {
            return Clone::clone(existing);
        }
        p.push((key, Clone::clone(&made)));
        made
    })))
}

/// 全部已登记 provider（登记序 = 优先序，逐个按需构造）——`ProviderList.providers()` 的等价物。
pub fn all_providers() -> Result<Vec<Object>> {
    let names: Vec<&'static str> = PROVIDER_CTORS.with(|c| c.borrow().iter().map(|(n, _)| *n).collect());
    let mut out = Vec::new();
    for n in names {
        if let Some(p) = provider(n)? {
            out.push(p);
        }
    }
    Ok(out)
}
