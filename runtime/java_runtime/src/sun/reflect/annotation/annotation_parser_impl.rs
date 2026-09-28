//! `sun/reflect/annotation/AnnotationParser.parseSig`（内部边界类的手写方法，FS-R R4b）。
//!
//! JDK 体经 sun/reflect/generics（SignatureParser → Reifier → CoreReflectionFactory →
//! Class.forName）解析签名。注解属性里的签名（注解类型 type_index、class 元素 class_info_index）
//! 恒为描述符形态（JVMS §4.7.16.1：return_descriptor / field descriptor），不含泛型，
//! 结果即描述符 → Class；类型不在类宇宙内 → TypeNotPresentException（parseAnnotation2 捕获后
//! 跳过该注解，与 JDK 对缺失类型的处置一致）。其余方法按字节码翻译（closure.toml [release]）。

use crate::prelude::*;
use super::annotation_parser::AnnotationParser;
use crate::java::lang::Class;

impl AnnotationParser {
    #[jvm_boundary(upcalls = "java/lang/TypeNotPresentException.<init>:(Ljava/lang/String;Ljava/lang/Throwable;)V")]
    pub fn parseSig(sig: String, _container: Class) -> Result<Class> {
        Class::__from_descriptor_checked(&format!("{}", sig))
    }
}
