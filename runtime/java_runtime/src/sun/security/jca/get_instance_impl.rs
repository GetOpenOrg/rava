//! `sun/security/jca/GetInstance` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! K-JCA（`docs/plans/2026-09-25-jca-service-registry.md`）：JDK 在此遍历 ProviderList、
//! 经 `Provider$Service.newInstance` 按类名反射构造实现类。原生侧改查生成注册表
//! （`crate::jca`，codegen 按「engine 类在链上 × 用户算法名」入选服务并翻译实现类字节码），
//! 构造即调用翻译出的无参构造器；实现类的全部算法逻辑走翻译字节码。

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

fn instance_of(type_: &str, algorithm: &str) -> Result<GetInstance_Instance> {
    let Some(entry) = crate::jca::find(type_, algorithm) else {
        return Err(not_available(type_, algorithm));
    };
    let impl_ = (entry.ctor)()?;
    let mut inst = GetInstance_Instance::default();
    inst._init_not_null();
    inst.__set_provider(Provider::__for_name(entry.provider));
    inst.__set_impl_(impl_);
    Ok(inst)
}

impl GetInstance {
    /// `getInstance(String type, Class<?> clazz, String algorithm)`：首个登记该 (类型, 算法)
    /// 的服务 → 实例。clazz（SPI 基类）的超类核对由类型系统保证（登记表只含该类型实现类）。
    #[jvm_boundary(upcalls = "java/security/NoSuchAlgorithmException.<init>:(Ljava/lang/String;)V")]
    pub fn getInstance_str_class_str(type_: String, _clazz: Class, algorithm: String) -> Result<GetInstance_Instance> {
        if algorithm.is_jvm_null() {
            return Err(not_available(&format!("{}", type_), "null"));
        }
        instance_of(&format!("{}", type_), &format!("{}", algorithm))
    }

    /// `getServices(List<ServiceId>)`：按候选序（transformation 由具体到一般）收集已登记服务；
    /// 无匹配 → 空表（Cipher 据此抛 `NoSuchAlgorithmException("Cannot find any provider
    /// supporting ..")`，走翻译字节码）。
    #[jvm_boundary(upcalls = "java/util/ArrayList.<init>:()V java/util/ArrayList.add:(Ljava/lang/Object;)Z java/util/List.size:()I java/util/List.get:(I)Ljava/lang/Object;")]
    pub fn getServices_list(ids: Object) -> Result<List<Object>> {
        // 调用侧按边界方法的接口形参擦除传 Object（载体策略），此处还原 List 视图
        let ids = <List<Object> as ::std::convert::From<Object>>::from(ids);
        let out = ArrayList::<Object>::new()?;
        let n = ids.size()?;
        for i in 0..n {
            let id = <ServiceId as ::std::convert::From<Object>>::from(ids.get(i)?);
            let type_ = format!("{}", id.__get_type_());
            let algo = format!("{}", id.__get_algorithm());
            if let Some(entry) = crate::jca::find(&type_, &algo) {
                out.add_obj(Object::from(Provider_Service::__from_entry(&entry)))?;
            }
        }
        Ok(<List<Object> as ::std::convert::From<_>>::from(Object::from(out)))
    }
}
