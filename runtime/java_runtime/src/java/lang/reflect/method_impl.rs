//! `java/lang/reflect/Method` 手写伴生：L3 分派键（NativeAccessor.invoke0 按键经
//! reflect_invoke 调用，FS-R R2）。注解查询族已回到 JDK 字节码（FS-R R4b：
//! Method.annotations 字节 → AnnotationParser）。
//!
//! 挂载键 (clazz, name, 描述符) 从成员对象元数据字段还原（描述符的参数段由
//! parameterTypes 还原，返回类型由 returnType 还原）。

use crate::prelude::*;
use super::Method;
use crate::java::lang::Class;
use crate::java::lang::Object;

/// Class 名（点/斜线形态均可）→ 描述符形态（与 class_impl 的 getDeclaredMethod
/// 参数归一同规则；对返回类型同样适用——void 的 Class 缺席形态 → "V"）。
fn __type_desc(class_name: &str) -> std::string::String {
    let slashes = class_name.replace('.', "/");
    match slashes.as_str() {
        "boolean" => "Z".into(),
        "byte" => "B".into(),
        "char" => "C".into(),
        "short" => "S".into(),
        "int" => "I".into(),
        "long" => "J".into(),
        "float" => "F".into(),
        "double" => "D".into(),
        "void" => "V".into(),
        _ if slashes.starts_with('[') => slashes,
        _ => format!("L{};", slashes),
    }
}

/// 接收者的方法挂载键（声明类斜线名, 方法名, 完整描述符）——与 build.rs
/// 方法注解表的键协议一致（含返回类型的完整描述符；返回段经 returnType
/// 还原，void 的 Class 缺席形态 → V）。
/// parameterTypes 缺席（非表构造的 Method）→ None（注解面为空）。
fn __member_key(m: &Method) -> Option<(std::string::String, std::string::String, std::string::String)> {
    let clazz = m.__get_clazz();
    if Object::from(Clone::clone(&clazz)).0.is_jvm_null() {
        return None;
    }
    let cls_key = format!("{}", clazz.__get_name()).replace('.', "/");
    let name = format!("{}", m.__get_name());
    let params = m.__get_parameterTypes();
    let mut desc = std::string::String::from("(");
    for i in 0..params.len().ok()? {
        let p = params.get(i).ok()?;
        desc.push_str(&__type_desc(&format!("{}", p.__get_name())));
    }
    desc.push(')');
    let ret = m.__get_returnType();
    let ret_name = if Object::from(Clone::clone(&ret)).0.is_jvm_null() {
        std::string::String::from("void")
    } else {
        format!("{}", ret.__get_name())
    };
    desc.push_str(&__type_desc(&ret_name));
    Some((cls_key, name, desc))
}

impl Method {
    /// 反射族内部：本方法的 L3 分派键 (声明类斜线名, 方法名, 完整描述符)。
    pub(crate) fn __reflect_key(&self) -> Option<(std::string::String, std::string::String, std::string::String)> {
        __member_key(self)
    }
}
