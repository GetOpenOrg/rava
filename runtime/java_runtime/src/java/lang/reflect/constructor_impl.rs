//! `java/lang/reflect/Constructor` 手写伴生：newInstance（L3 反射分派的
//! 构造器路径）+ 注解元数据查询（反射 L3 段 1，与 method_impl.rs 同构）。
//!
//! Constructor 是 final 类，调用侧接收者静态类型恒为 Constructor wrapper，
//! 查询族以 wrapper 固有方法承载（impl_methods 协议）。挂载键：
//! (clazz, "<init>", 描述符)——描述符参数段从 parameterTypes 还原。

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
    /// `newInstance(Object[])`：L3 分派的构造器路径（`<init>` 臂 → Self::new
    /// 系列 typed 构造）。目标构造器抛出的异常包装为
    /// InvocationTargetException（JDK Constructor.newInstance 契约），返回值
    /// 经 `<T as From<Object>>` 还原为构造类的擦除视图。
    pub fn newInstance(&self, initargs: JArray<Object>) -> Result<T> {
        let Some((cls_key, desc)) = __member_key(self) else {
            panic!("stub: Constructor.newInstance 无声明键（非表构造形态）");
        };
        // 序列化构造器（ReflectionFactory 登记，N2）：分配目标类实例（不运行其构造器），
        // 再在该实例上运行本构造器（首个不可序列化超类 initCl 的无参构造体）
        let __self_id = Object::from(Clone::clone(self)).0.__identity() as usize;
        if let Some(target) = crate::reflect_dispatch::serialization_target(__self_id) {
            let empty: JArray<Object> = JArray::from(Vec::<Object>::new());
            let obj = crate::reflect_dispatch::reflect_invoke(
                &target, "<alloc>", "()V", Object::default(), &empty)?;
            crate::reflect_dispatch::reflect_invoke(
                &cls_key, "<init_on>", "()V", Clone::clone(&obj), &empty)?;
            return Ok(<T as From<Object>>::from(obj));
        }
        match crate::reflect_dispatch::reflect_invoke(
            &cls_key, "<init>", &desc, Object::default(), &initargs) {
            Ok(v) => Ok(<T as From<Object>>::from(v)),
            // 实参拆箱失败 → IllegalArgumentException 直接抛出（不包装，JDK 同）
            Err(e) if crate::reflect_dispatch::take_bad_arg() => Err(e),
            Err(e) => {
                let ite = crate::java::lang::reflect::InvocationTargetException::new_throwable(
                    <crate::java::lang::Throwable as From<Object>>::from(
                        Clone::clone(e.thrown())))?;
                Err(JvmError::from(ite))
            }
        }
    }

    /// 本构造器挂载点的注解条目。
    fn __anno_entries(&self) -> &'static [crate::annotation_meta::__anno_table::AnnotationEntry] {
        match __member_key(self) {
            Some((c, d)) => crate::annotation_meta::method_annotation_entries(&c, "<init>", &d),
            None => &[],
        }
    }

    /// `getAnnotation(Class)`（AccessibleObject 继承 API 的 final 类固有承载）。
    /// upcalls：同体 newInstance 的包装异常种子（method_impl.rs 同一模式）。
    #[jvm_native(upcalls = "java/lang/reflect/InvocationTargetException.<init>:(Ljava/lang/Throwable;)V java/lang/Integer.toString:()Ljava/lang/String; java/lang/Long.toString:()Ljava/lang/String; java/lang/Short.toString:()Ljava/lang/String; java/lang/Byte.toString:()Ljava/lang/String; java/lang/Character.toString:()Ljava/lang/String; java/lang/Boolean.toString:()Ljava/lang/String; java/lang/Float.toString:()Ljava/lang/String; java/lang/Double.toString:()Ljava/lang/String;")]
    pub fn getAnnotation(&self, annotationClass: Class) -> Result<Object> {
        let anno = format!("{}", annotationClass.__get_name()).replace('.', "/");
        let Some(hit) = crate::annotation_meta::find_annotation(self.__anno_entries(), &anno)
        else { return Ok(Object::default()) };
        crate::annotation_meta::annotation_instance(hit.anno, hit.elements)
    }

    /// `isAnnotationPresent(Class)`：纯名匹配。
    pub fn isAnnotationPresent(&self, annotationClass: Class) -> Result<bool> {
        let anno = format!("{}", annotationClass.__get_name()).replace('.', "/");
        Ok(crate::annotation_meta::has_annotation(self.__anno_entries(), &anno))
    }

    /// `getAnnotations()`。
    pub fn getAnnotations(&self) -> Result<JArray<Object>> {
        let mut out: Vec<Object> = Vec::new();
        for e in self.__anno_entries() {
            out.push(crate::annotation_meta::annotation_instance(e.anno, e.elements)?);
        }
        Ok(JArray::from(out))
    }
}
