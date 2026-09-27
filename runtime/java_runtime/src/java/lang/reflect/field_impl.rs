use crate::prelude::*;
use super::Field;
use crate::java::lang::Class;
use crate::java::lang::Object;

// `java/lang/reflect/Field` 手写伴生：注解元数据查询（反射 L3 段 1，FS-R R4 前的过渡）。
// Field.get / set / getInt / getLong 已回到 JDK 字节码（FS-R R2a：ReflectionFactory →
// MethodHandleAccessorFactory → 字段句柄 → reflect_field，docs/plans/2026-09-27-reflection-metadata-table.md）。

impl Field {
    // ── 注解元数据查询族（反射 L3 段 1）────────────────────────────────────
    //
    // Field 是 final 类（调用侧接收者静态类型恒为 Field wrapper），查询族以
    // wrapper 固有方法承载（impl_methods 协议；继承成员登记命中固有名时不再
    // 生成转发，E0592 防撞）。挂载键 (clazz, 字段名)；数据面是 build.rs 注解
    // 表，实例面是注解工厂（method_impl.rs 同款）。

    /// 本字段挂载点的注解条目（空 = 无注解 / 非表构造形态）。
    fn __anno_entries(&self) -> &'static [crate::annotation_meta::__anno_table::AnnotationEntry] {
        let clazz = self.__get_clazz();
        if Object::from(Clone::clone(&clazz)).0.is_jvm_null() {
            return &[];
        }
        let cls_key = format!("{}", clazz.__get_name()).replace('.', "/");
        let name = format!("{}", self.__get_name());
        crate::annotation_meta::field_annotation_entries(&cls_key, &name)
    }

    /// `getAnnotation(Class)`：命中 → 注解代理实例（未命中 → null）。
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
