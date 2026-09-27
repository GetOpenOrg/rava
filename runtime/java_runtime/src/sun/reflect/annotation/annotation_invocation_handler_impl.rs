//! `sun/reflect/annotation/AnnotationInvocationHandler.memberValueToString`（内部边界类的手写
//! 方法，FS-R R4b）。
//!
//! JDK 体对基本类型数组经 DoubleStream / IntStream / LongStream 的 mapToObj + joining 拼接
//! （`{a, b}`）。三族基本类型流水线使 java_runtime 编译峰值内存越过 15G；此处以逐元素循环
//! 等价拼接，单元素格式化仍调用翻译体的 toSourceString 重载（字面量形态与 JDK 逐字相同：
//! `1.0f` / `'c'` / `10L` / `(byte)0x01` / `"s"` / `Foo.class` / 枚举常量名）。int / short /
//! boolean 元素与 JDK 同为 String.valueOf 的十进制 / true|false 文本。其余方法按字节码翻译。

use crate::prelude::*;
use super::annotation_invocation_handler::AnnotationInvocationHandler as H;
use crate::java::lang::{Class, Enum};

fn join(parts: Vec<std::string::String>) -> String {
    String::from(format!("{{{}}}", parts.join(", ")).as_str())
}

fn each<T: Clone + Default + From<Object> + Into<Object> + 'static + crate::sync_model::__ThreadSafe>(arr: &JArray<T>, mut f: impl FnMut(T) -> Result<std::string::String>)
    -> Result<Vec<std::string::String>> {
    let mut out = Vec::new();
    for i in 0..arr.len()? {
        out.push(f(arr.get(i)?)?);
    }
    Ok(out)
}

impl H {
    #[jvm_boundary(upcalls = "sun/reflect/annotation/AnnotationInvocationHandler.toSourceString:(Ljava/lang/Class;)Ljava/lang/String; sun/reflect/annotation/AnnotationInvocationHandler.toSourceString:(F)Ljava/lang/String; sun/reflect/annotation/AnnotationInvocationHandler.toSourceString:(D)Ljava/lang/String; sun/reflect/annotation/AnnotationInvocationHandler.toSourceString:(C)Ljava/lang/String; sun/reflect/annotation/AnnotationInvocationHandler.toSourceString:(B)Ljava/lang/String; sun/reflect/annotation/AnnotationInvocationHandler.toSourceString:(J)Ljava/lang/String; sun/reflect/annotation/AnnotationInvocationHandler.toSourceString:(Ljava/lang/Enum;)Ljava/lang/String; sun/reflect/annotation/AnnotationInvocationHandler.toSourceString:(Ljava/lang/String;)Ljava/lang/String;")]
    pub fn memberValueToString(value: Object) -> Result<String> {
        let s = |x: String| -> std::string::String { format!("{}", x) };
        let kind = value.0.__class_name();
        if !kind.starts_with('[') {
            return match kind {
                "java/lang/Class" => H::toSourceString_class(<Class as From<Object>>::from(value)),
                "java/lang/String" => H::toSourceString_str(<String as From<Object>>::from(value)),
                "java/lang/Character" => H::toSourceString_c(
                    crate::reflect_dispatch::unbox_char(&value).unwrap_or_default()),
                "java/lang/Double" => H::toSourceString_d(
                    crate::reflect_dispatch::unbox_f64(&value).unwrap_or_default()),
                "java/lang/Float" => H::toSourceString_f(
                    crate::reflect_dispatch::unbox_f32(&value).unwrap_or_default()),
                "java/lang/Long" => H::toSourceString_l(
                    crate::reflect_dispatch::unbox_i64(&value).unwrap_or_default()),
                "java/lang/Byte" => H::toSourceString_b(
                    crate::reflect_dispatch::unbox_i32(&value).unwrap_or_default() as i8),
                _ if value.is_instance_of("java/lang/Enum") =>
                    H::toSourceString_enum(<Enum<Object> as From<Object>>::from(value)),
                _ => value.toString(),
            };
        }
        let arr = |o: &Object| Clone::clone(o);
        let parts: Vec<std::string::String> = match kind {
            "[B" => each(&<JArray<i8> as From<Object>>::from(arr(&value)), |x| Ok(s(H::toSourceString_b(x)?)))?,
            "[C" => each(&<JArray<u16> as From<Object>>::from(arr(&value)), |x| Ok(s(H::toSourceString_c(x)?)))?,
            "[D" => each(&<JArray<f64> as From<Object>>::from(arr(&value)), |x| Ok(s(H::toSourceString_d(x)?)))?,
            "[F" => each(&<JArray<f32> as From<Object>>::from(arr(&value)), |x| Ok(s(H::toSourceString_f(x)?)))?,
            "[I" => each(&<JArray<i32> as From<Object>>::from(arr(&value)), |x| Ok(x.to_string()))?,
            "[J" => each(&<JArray<i64> as From<Object>>::from(arr(&value)), |x| Ok(s(H::toSourceString_l(x)?)))?,
            "[S" => each(&<JArray<i16> as From<Object>>::from(arr(&value)), |x| Ok(x.to_string()))?,
            "[Z" => each(&<JArray<bool> as From<Object>>::from(arr(&value)), |x| Ok(x.to_string()))?,
            _ => each(&<JArray<Object> as From<Object>>::from(arr(&value)), |x| {
                if x.0.is_jvm_null() {
                    return Ok("null".to_owned());
                }
                Ok(match x.0.__class_name() {
                    "java/lang/Class" => s(H::toSourceString_class(<Class as From<Object>>::from(x))?),
                    "java/lang/String" => s(H::toSourceString_str(<String as From<Object>>::from(x))?),
                    _ if x.is_instance_of("java/lang/Enum") =>
                        s(H::toSourceString_enum(<Enum<Object> as From<Object>>::from(x))?),
                    _ => s(x.toString()?),
                })
            })?,
        };
        Ok(join(parts))
    }
}
