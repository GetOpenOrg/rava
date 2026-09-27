//! `java/lang/reflect/Constructor` 手写伴生：L3 分派键（NativeAccessor.newInstance0 按键经
//! reflect_invoke 构造，FS-R R2）。newInstance 与注解查询族已回到 JDK 字节码（FS-R R2 / R4b）。
//! 挂载键：(clazz, 描述符)——描述符参数段从 parameterTypes 还原。

use crate::prelude::*;
use super::Constructor;
use crate::java::lang::Class;
use crate::java::lang::Object;

/// Class 名（点/斜线形态均可）→ 描述符形态（method_impl 同一归一规则）。
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

/// 接收者的构造器挂载键（声明类斜线名, "<init>", 完整描述符——返回段 V）。
fn __member_key<T: Clone + Default + 'static + From<Object> + Into<Object> + crate::sync_model::__ThreadSafe>(
    c: &Constructor<T>) -> Option<(std::string::String, std::string::String)> {
    let clazz = c.__get_clazz();
    if Object::from(Clone::clone(&clazz)).0.is_jvm_null() {
        return None;
    }
    let cls_key = format!("{}", clazz.__get_name()).replace('.', "/");
    let params = c.__get_parameterTypes();
    let mut desc = std::string::String::from("(");
    for i in 0..params.len().ok()? {
        let p = params.get(i).ok()?;
        desc.push_str(&__type_desc(&format!("{}", p.__get_name())));
    }
    desc.push_str(")V");
    Some((cls_key, desc))
}

impl<T: Clone + Default + 'static + From<Object> + Into<Object> + crate::sync_model::__ThreadSafe> Constructor<T> {
    /// 反射族内部：本构造器的 L3 分派键 (声明类斜线名, 描述符)。
    pub(crate) fn __reflect_key(&self) -> Option<(std::string::String, std::string::String)> {
        __member_key(self)
    }
}
