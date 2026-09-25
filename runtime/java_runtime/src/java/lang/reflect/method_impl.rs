//! `java/lang/reflect/Method` 手写伴生：注解元数据查询（反射 L3 段 1）。
//!
//! Method 是 final 类（JDK 语义无子类覆盖面），调用侧接收者静态类型恒为
//! Method wrapper——注解查询族以 wrapper 固有方法承载（impl_methods 协议），
//! 挂载键 (clazz, name, 描述符) 从成员对象元数据字段还原（描述符的参数段
//! 由 parameterTypes 还原——返回类型不在身份键内，与 getDeclaredMethod 的
//! 配对同一归一规则）。
//!
//! 数据面：build.rs 注解元数据表（java_class! 块的 annotations 属性 →
//! OUT_DIR/annotation_table.rs）；实例面：翻译期合成的注解代理经注解工厂
//! 构造（annotation_meta 模块头注）。

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
    /// `invoke(Object, Object[])`：L3 反射分派（协议见 reflect_dispatch 模块
    /// 头注）。JDK 语义：目标方法抛出的任何 Throwable 一律包装为
    /// InvocationTargetException（cause = 原异常——JUnit 的 ReflectiveCallable
    /// 捕获后 getTargetException 解包）；访问检查近似（override_ 或 public
    /// 之外 → IllegalAccessException，与 Field.get 同一策略）。
    #[jvm_native(upcalls = "java/lang/reflect/InvocationTargetException.<init>:(Ljava/lang/Throwable;)V")]
    pub fn invoke_obj_arr_obj(&self, obj: Object, args: JArray<Object>) -> Result<Object> {
        let name = format!("{}", self.__get_name());
        let mods = self.__get_modifiers();
        if !self.__get_override_() && (mods & 0x0001) == 0 {
            return Err(JvmError::from(crate::java::lang::IllegalAccessException::new_str(
                String::from(format!("Class can not access a member with modifiers {}", mods)))?));
        }
        let Some((cls_key, _n, desc)) = __member_key(self) else {
            panic!("stub: Method.invoke 无声明键（非表构造的 Method）: {}", name);
        };
        let ret = crate::reflect_dispatch::reflect_invoke(
            &cls_key, &name, &desc, obj, &args);
        match ret {
            Ok(v) => Ok(v),
            // 实参拆箱失败 → IllegalArgumentException 直接抛出（不包装，JDK 同）
            Err(e) if crate::reflect_dispatch::take_bad_arg() => Err(e),
            // 目标异常 → InvocationTargetException 包装（JDK Method.invoke 契约）
            Err(e) => {
                let ite = crate::java::lang::reflect::InvocationTargetException::new_throwable(
                    <crate::java::lang::Throwable as From<Object>>::from(
                        Clone::clone(e.thrown())))?;
                Err(JvmError::from(ite))
            }
        }
    }

    /// 本方法挂载点的注解条目（空 = 无注解 / 非表构造形态）。
    fn __anno_entries(&self) -> &'static [crate::annotation_meta::__anno_table::AnnotationEntry] {
        match __member_key(self) {
            Some((c, n, d)) => crate::annotation_meta::method_annotation_entries(&c, &n, &d),
            None => &[],
        }
    }

    /// `getAnnotation(Class)`：命中 → 注解代理实例（未命中 → null）。
    ///
    /// upcalls 说明：本文件同体的 invoke_obj_arr_obj 需要
    /// InvocationTargetException 构造器在闭包内（JDK invoke 契约的包装异常）；
    /// 注解查询是本文件最早被 BFS 触达的成员，在此声明种子。
    #[jvm_native(upcalls = "java/lang/reflect/InvocationTargetException.<init>:(Ljava/lang/Throwable;)V")]
    pub fn getAnnotation(&self, annotationClass: Class) -> Result<Object> {
        let anno = format!("{}", annotationClass.__get_name()).replace('.', "/");
        let Some(hit) = crate::annotation_meta::find_annotation(self.__anno_entries(), &anno)
        else { return Ok(Object::default()) };
        crate::annotation_meta::annotation_instance(hit.anno, hit.elements)
    }

    /// `isAnnotationPresent(Class)`：纯名匹配（无需工厂）。
    pub fn isAnnotationPresent(&self, annotationClass: Class) -> Result<bool> {
        let anno = format!("{}", annotationClass.__get_name()).replace('.', "/");
        Ok(crate::annotation_meta::has_annotation(self.__anno_entries(), &anno))
    }

    /// `getAnnotations()`：全部注解实例（声明序）。
    pub fn getAnnotations(&self) -> Result<JArray<Object>> {
        let mut out: Vec<Object> = Vec::new();
        for e in self.__anno_entries() {
            out.push(crate::annotation_meta::annotation_instance(e.anno, e.elements)?);
        }
        Ok(JArray::from(out))
    }

    /// `getDeclaredAnnotations()`：RuntimeVisibleAnnotations 即声明面。
    pub fn getDeclaredAnnotations(&self) -> Result<JArray<Object>> {
        self.getAnnotations()
    }
}
