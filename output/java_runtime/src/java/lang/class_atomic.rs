#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::security::util::*;
use crate::jdk::internal::misc::Unsafe;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/Class$Atomic",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "package",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Class.java",
    inner_classes     = "java/lang/Class$Atomic:java/lang/Class:Atomic:10;java/lang/Class$ReflectionData:java/lang/Class:ReflectionData:10;java/lang/Class$AnnotationData:java/lang/Class:AnnotationData:10",
    all_supertypes    = "java/lang/Class$Atomic;java/lang/Object",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Class_Atomic;

impl Class_Atomic {
    #[cfg_attr(any(), java_field(name = "unsafe", descriptor = "Ljdk/internal/misc/Unsafe;", access = "private", modifiers = "static final", is_static = true))]
    // static field: unsafe:Ljdk/internal/misc/Unsafe;
    pub fn unsafe_() -> Unsafe {
        panic!("stub: java/lang/Class$Atomic.unsafe:Ljdk/internal/misc/Unsafe;")
    }

    #[cfg_attr(any(), java_field(name = "reflectionDataOffset", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
    // static field: reflectionDataOffset:J
    pub fn reflectionDataOffset() -> i64 {
        panic!("stub: java/lang/Class$Atomic.reflectionDataOffset:J")
    }

    #[cfg_attr(any(), java_field(name = "annotationTypeOffset", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
    // static field: annotationTypeOffset:J
    pub fn annotationTypeOffset() -> i64 {
        panic!("stub: java/lang/Class$Atomic.annotationTypeOffset:J")
    }

    #[cfg_attr(any(), java_field(name = "annotationDataOffset", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
    // static field: annotationDataOffset:J
    pub fn annotationDataOffset() -> i64 {
        panic!("stub: java/lang/Class$Atomic.annotationDataOffset:J")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        panic!("stub: java/lang/Class$Atomic.<init>:()V")
    }

    #[cfg_attr(any(), java_method(name = "casReflectionData", descriptor = "(Ljava/lang/Class;Ljava/lang/ref/SoftReference;Ljava/lang/ref/SoftReference;)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/lang/Class<*>;Ljava/lang/ref/SoftReference<Ljava/lang/Class$ReflectionData<TT;>;>;Ljava/lang/ref/SoftReference<Ljava/lang/Class$ReflectionData<TT;>;>;)Z"))]
    pub fn casReflectionData(mut clazz: Object, mut oldData: SoftReference<Object>, mut newData: SoftReference<Object>) -> Result<bool> {
        let _t0 = Class_Atomic::unsafe_().compareAndSetReference(Clone::clone(&clazz), Class_Atomic::reflectionDataOffset(), Object::from_any(oldData.clone()), Object::from_any(newData.clone()))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "casAnnotationType", descriptor = "(Ljava/lang/Class;Lsun/reflect/annotation/AnnotationType;Lsun/reflect/annotation/AnnotationType;)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Lsun/reflect/annotation/AnnotationType;Lsun/reflect/annotation/AnnotationType;)Z"))]
    pub fn casAnnotationType(clazz: Object, oldType: Object, newType: Object) -> Result<bool> {
        panic!("stub: java/lang/Class$Atomic.casAnnotationType:(Ljava/lang/Class;Lsun/reflect/annotation/AnnotationType;Lsun/reflect/annotation/AnnotationType;)Z")
    }

    #[cfg_attr(any(), java_method(name = "casAnnotationData", descriptor = "(Ljava/lang/Class;Ljava/lang/Class$AnnotationData;Ljava/lang/Class$AnnotationData;)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class$AnnotationData;Ljava/lang/Class$AnnotationData;)Z"))]
    pub fn casAnnotationData(clazz: Object, oldData: Object, newData: Object) -> Result<bool> {
        panic!("stub: java/lang/Class$Atomic.casAnnotationData:(Ljava/lang/Class;Ljava/lang/Class$AnnotationData;Ljava/lang/Class$AnnotationData;)Z")
    }
}
