//! `sun/security/jca/GetInstance` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! K-JCA（`docs/plans/2026-09-28-jca-faithful-provider.md`）：JDK 在此遍历 ProviderList
//!（运行期按配置装载 provider）。原生侧的 provider 列表 = 生成服务表中登记了该 (类型, 算法)
//! 的 provider（`crate::jca`，登记序即优先序），provider 对象按需构造一次；其后与 JDK 逐步
//! 同构：`Provider.getService` 取服务描述、`Provider$Service.newInstance` 反射构造实现类、
//! `new Instance(provider, impl)`——全部走翻译字节码。

use crate::prelude::*;
use super::get_instance::implref::GetInstance;
use super::get_instance_instance::implref::GetInstance_Instance;
use super::service_id::implref::ServiceId;
use crate::java::security::Provider;
use crate::java::security::Provider_Service;
use crate::java::lang::Class;
use crate::java::util::ArrayList;
use crate::java::util::List;

/// JDK 未找到服务时的异常：`NoSuchAlgorithmException(algorithm + " " + type + " not available")`
///（GetInstance.getInstance / getService 同一消息形态）。
fn not_available(type_: &str, algorithm: &str) -> crate::error::JvmError {
    let msg = String::from(format!("{} {} not available", algorithm, type_).as_str());
    match crate::java::security::NoSuchAlgorithmException::new_str(msg) {
        Ok(ex) => ex.into(),
        Err(nested) => nested,
    }
}

/// `ProviderList.getServices(type, algorithm)` 的等价物：按 provider 优先序，各 provider 的
/// `getService(type, algorithm)`（翻译字节码）非 null 者。
fn services(type_: &str, algorithm: &str) -> Result<Vec<Provider_Service>> {
    let mut out = Vec::new();
    for name in crate::jca::providers_for(type_, algorithm) {
        let Some(p) = crate::jca::provider(name)? else { continue };
        let p = <Provider as ::std::convert::From<Object>>::from(p);
        let s = p.getService(String::from(type_), String::from(algorithm))?;
        if !s.is_jvm_null() {
            out.push(s);
        }
    }
    Ok(out)
}

/// `GetInstance.getInstance(Service, Class)`：`s.newInstance(null)` 构造实现类（翻译字节码的
/// 反射路径），包成 `Instance(provider, impl)`。SPI 超类核对（checkSuperClass）由类型系统
/// 保证（服务表只含该类型实现类）。
fn instance_of(s: &Provider_Service) -> Result<GetInstance_Instance> {
    let impl_ = s.newInstance(Object::default())?;
    let mut inst = GetInstance_Instance::default();
    inst._init_not_null();
    inst.__set_provider(s.getProvider()?);
    inst.__set_impl_(impl_);
    Ok(inst)
}

impl GetInstance {
    /// `getInstance(String type, Class<?> clazz, String algorithm)`：JDK 同序——首个服务构造
    /// 失败（NoSuchAlgorithmException）时依次尝试其余服务，全部失败抛最后一个失败。
    #[jvm_boundary(upcalls = "java/security/NoSuchAlgorithmException.<init>:(Ljava/lang/String;)V java/security/Provider.getService:(Ljava/lang/String;Ljava/lang/String;)Ljava/security/Provider$Service; java/security/Provider$Service.newInstance:(Ljava/lang/Object;)Ljava/lang/Object; java/security/Provider$Service.getProvider:()Ljava/security/Provider;")]
    pub fn getInstance_str_class_str(type_: String, _clazz: Class, algorithm: String) -> Result<GetInstance_Instance> {
        let type_ = format!("{}", type_);
        if algorithm.is_jvm_null() {
            return Err(not_available(&type_, "null"));
        }
        let algorithm = format!("{}", algorithm);
        let list = services(&type_, &algorithm)?;
        if list.is_empty() {
            return Err(not_available(&type_, &algorithm));
        }
        let mut failure = None;
        for s in &list {
            match instance_of(s) {
                Ok(inst) => return Ok(inst),
                Err(e) if e.is_instance_of("java/security/NoSuchAlgorithmException") => failure = Some(e),
                Err(e) => return Err(e),
            }
        }
        Err(failure.expect("非空服务列表"))
    }

    /// `getInstance(String type, Class<?> clazz, String algorithm, String provider)`：JDK
    /// `getService(type, algorithm, provider)` 同一校验与异常形态——provider 名空 →
    /// IllegalArgumentException("missing provider")；未登记 → NoSuchProviderException("no such
    /// provider: " + provider)；该 provider 无此服务 → NoSuchAlgorithmException("no such
    /// algorithm: " + algorithm + " for provider " + provider)。
    #[jvm_boundary(upcalls = "java/lang/IllegalArgumentException.<init>:(Ljava/lang/String;)V java/security/NoSuchProviderException.<init>:(Ljava/lang/String;)V java/security/NoSuchAlgorithmException.<init>:(Ljava/lang/String;)V java/security/Provider.getService:(Ljava/lang/String;Ljava/lang/String;)Ljava/security/Provider$Service; java/security/Provider$Service.newInstance:(Ljava/lang/Object;)Ljava/lang/Object; java/security/Provider$Service.getProvider:()Ljava/security/Provider;")]
    pub fn getInstance_str_class_str_str(type_: String, _clazz: Class, algorithm: String, provider: String) -> Result<GetInstance_Instance> {
        if provider.is_jvm_null() || provider.length()? == 0 {
            let ex = crate::java::lang::IllegalArgumentException::new_str(String::from("missing provider"))?;
            return Err(ex.into());
        }
        let pname = format!("{}", provider);
        let Some(p) = crate::jca::provider(&pname)? else {
            let ex = crate::java::security::NoSuchProviderException::new_str(
                String::from(format!("no such provider: {}", pname).as_str()))?;
            return Err(ex.into());
        };
        let p = <Provider as ::std::convert::From<Object>>::from(p);
        let s = p.getService(type_, Clone::clone(&algorithm))?;
        if s.is_jvm_null() {
            let ex = crate::java::security::NoSuchAlgorithmException::new_str(
                String::from(format!("no such algorithm: {} for provider {}", algorithm, pname).as_str()))?;
            return Err(ex.into());
        }
        instance_of(&s)
    }

    /// `getInstance(String type, Class<?> clazz, String algorithm, Provider provider)`：JDK
    /// `getService(type, algorithm, provider)` 同一校验与异常形态——provider 为 null →
    /// IllegalArgumentException("missing provider")；该 provider 无此服务 →
    /// NoSuchAlgorithmException("no such algorithm: " + algorithm + " for provider " + provider.getName())。
    /// 消费方：`Security.getImpl(String, String, Provider)`（AlgorithmParameters.getInstance(String, Provider)，
    /// CipherCore.getParameters 经此以 SunJCE 实例取参数对象）。
    #[jvm_boundary(upcalls = "java/lang/IllegalArgumentException.<init>:(Ljava/lang/String;)V java/security/NoSuchAlgorithmException.<init>:(Ljava/lang/String;)V java/security/Provider.getService:(Ljava/lang/String;Ljava/lang/String;)Ljava/security/Provider$Service; java/security/Provider.getName:()Ljava/lang/String; java/security/Provider$Service.newInstance:(Ljava/lang/Object;)Ljava/lang/Object; java/security/Provider$Service.getProvider:()Ljava/security/Provider;")]
    pub fn getInstance_str_class_str_provider(type_: String, _clazz: Class, algorithm: String, provider: Provider) -> Result<GetInstance_Instance> {
        if provider.is_jvm_null() {
            let ex = crate::java::lang::IllegalArgumentException::new_str(String::from("missing provider"))?;
            return Err(ex.into());
        }
        let s = provider.getService(type_, Clone::clone(&algorithm))?;
        if s.is_jvm_null() {
            let ex = crate::java::security::NoSuchAlgorithmException::new_str(String::from(
                format!("no such algorithm: {} for provider {}", algorithm, provider.getName()?).as_str()))?;
            return Err(ex.into());
        }
        instance_of(&s)
    }

    /// `getInstance(Provider.Service s, Class<?> clazz)`：`s.newInstance(null)` 包成 `Instance`。
    #[jvm_boundary(upcalls = "java/security/Provider$Service.newInstance:(Ljava/lang/Object;)Ljava/lang/Object; java/security/Provider$Service.getProvider:()Ljava/security/Provider;")]
    pub fn getInstance_provider_service_class(s: Provider_Service, _clazz: Class) -> Result<GetInstance_Instance> {
        instance_of(&s)
    }

    /// `getServices(List<ServiceId>)`：按候选序（transformation 由具体到一般）收集已登记服务；
    /// 无匹配 → 空表（Cipher 据此抛 `NoSuchAlgorithmException("Cannot find any provider
    /// supporting ..")`，走翻译字节码）。
    #[cfg(not(jdk_ge_25))]
    #[jvm_boundary(upcalls = "java/util/ArrayList.<init>:()V java/util/ArrayList.add:(Ljava/lang/Object;)Z java/util/List.size:()I java/util/List.get:(I)Ljava/lang/Object; java/security/Provider.getService:(Ljava/lang/String;Ljava/lang/String;)Ljava/security/Provider$Service;")]
    pub fn getServices_list(ids: Object) -> Result<List<Object>> {
        Self::services_for(ids)
    }

    /// JDK 25：`getServices(List<ServiceId>)` 返回类型改为 `Iterator<Service>`（同一候选序）。
    #[cfg(jdk_ge_25)]
    #[jvm_boundary(upcalls = "java/util/ArrayList.<init>:()V java/util/ArrayList.add:(Ljava/lang/Object;)Z java/util/List.size:()I java/util/List.get:(I)Ljava/lang/Object; java/util/List.iterator:()Ljava/util/Iterator; java/security/Provider.getService:(Ljava/lang/String;Ljava/lang/String;)Ljava/security/Provider$Service;")]
    pub fn getServices_list(ids: Object) -> Result<crate::java::util::Iterator<Object>> {
        Self::services_for(ids)?.iterator()
    }

    /// JDK `ProviderList$ServiceList` 同序：provider 在外层、候选 id 在内层（每个 provider
    /// 依次对全部 id 调 `getService`，非 null 者收入）。
    fn services_for(ids: Object) -> Result<List<Object>> {
        // 调用侧按边界方法的接口形参擦除传 Object（载体策略），此处还原 List 视图
        let ids = <List<Object> as ::std::convert::From<Object>>::from(ids);
        let n = ids.size()?;
        let mut keys: Vec<(std::string::String, std::string::String)> = Vec::new();
        for i in 0..n {
            let id = <ServiceId as ::std::convert::From<Object>>::from(ids.get(i)?);
            keys.push((format!("{}", id.__get_type_()), format!("{}", id.__get_algorithm())));
        }
        let mut names: Vec<&'static str> = Vec::new();
        for (t, a) in &keys {
            for name in crate::jca::providers_for(t, a) {
                if !names.contains(&name) {
                    names.push(name);
                }
            }
        }
        let out = ArrayList::<Object>::new()?;
        for name in names {
            let Some(p) = crate::jca::provider(name)? else { continue };
            let p = <Provider as ::std::convert::From<Object>>::from(p);
            for (t, a) in &keys {
                let s = p.getService(String::from(t.as_str()), String::from(a.as_str()))?;
                if !s.is_jvm_null() {
                    out.add_obj(Object::from(s))?;
                }
            }
        }
        Ok(<List<Object> as ::std::convert::From<_>>::from(Object::from(out)))
    }
}
