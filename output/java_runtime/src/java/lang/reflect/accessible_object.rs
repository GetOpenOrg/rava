#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::math::*;
use crate::java::nio::*;
use crate::java::nio::charset::*;
use crate::java::security::*;
use crate::java::text::*;
use crate::java::text::spi::*;
use crate::java::time::*;
use crate::java::time::chrono::*;
use crate::java::time::temporal::*;
use crate::java::time::zone::*;
use crate::java::util::*;
use crate::java::util::concurrent::*;
use crate::java::util::concurrent::atomic::*;
use crate::java::util::concurrent::locks::*;
use crate::java::util::function::*;
use crate::java::util::regex::*;
use crate::java::util::spi::*;
use crate::java::util::stream::*;
use crate::java::util::zip::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::reflect::misc::*;
use crate::sun::security::action::*;
use crate::sun::security::util::*;
use crate::sun::text::*;
use crate::sun::util::*;
use crate::sun::util::calendar::*;
use crate::sun::util::locale::*;
use crate::sun::util::locale::provider::*;
use crate::sun::util::spi::*;
use crate::java::text::Normalizer;
use crate::jdk::internal::misc::VM;
use crate::jdk::internal::reflect::Reflection;
use crate::jdk::internal::reflect::ReflectionFactory;
use crate::jdk::internal::reflect::ReflectionFactory_GetReflectionFactoryAction;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/reflect/AccessibleObject"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/lang/reflect/AnnotatedElement"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AccessibleObject.java"]
    #[inner_classes     = "java/lang/reflect/AccessibleObject$Cache:java/lang/reflect/AccessibleObject:Cache:10;jdk/internal/reflect/ReflectionFactory$GetReflectionFactoryAction:jdk/internal/reflect/ReflectionFactory:GetReflectionFactoryAction:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/lang/reflect/AccessibleObject;java/lang/reflect/AnnotatedElement"]

    pub struct AccessibleObject {
        #[cfg_attr(any(), java_field(name = "override", descriptor = "Z", is_static = false))]
        pub override_: bool,
        #[cfg_attr(any(), java_field(name = "accessCheckCache", descriptor = "Ljava/lang/Object;", access = "package", modifiers = "volatile", is_static = false))]
        pub accessCheckCache: Object,
    }

    impl AccessibleObject {
        #[cfg_attr(any(), java_field(name = "reflectionFactory", descriptor = "Ljdk/internal/reflect/ReflectionFactory;", access = "package", modifiers = "static final", is_static = true))]
        // static field: reflectionFactory:Ljdk/internal/reflect/ReflectionFactory;
        pub fn reflectionFactory() -> ReflectionFactory {
            panic!("stub: java/lang/reflect/AccessibleObject.reflectionFactory:Ljdk/internal/reflect/ReflectionFactory;")
        }

        #[cfg_attr(any(), java_field(name = "printStackWhenAccessFails", descriptor = "Z", access = "private", modifiers = "static volatile", is_static = true))]
        // static field: printStackWhenAccessFails:Z
        pub fn printStackWhenAccessFails() -> bool {
            panic!("stub: java/lang/reflect/AccessibleObject.printStackWhenAccessFails:Z")
        }

        #[cfg_attr(any(), java_field(name = "printStackPropertiesSet", descriptor = "Z", access = "private", modifiers = "static volatile", is_static = true))]
        // static field: printStackPropertiesSet:Z
        pub fn printStackPropertiesSet() -> bool {
            panic!("stub: java/lang/reflect/AccessibleObject.printStackPropertiesSet:Z")
        }

        #[java_method(name = "checkPermission", descriptor = "()V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkPermission() -> Result<()> {
            panic!("stub: java/lang/reflect/AccessibleObject.checkPermission:()V")
        }

        #[java_method(name = "setAccessible", descriptor = "([Ljava/lang/reflect/AccessibleObject;Z)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setAccessible_arr_acc_z(array: Rc<RefCell<Vec<AccessibleObject>>>, flag: bool) -> Result<()> {
            panic!("stub: java/lang/reflect/AccessibleObject.setAccessible:([Ljava/lang/reflect/AccessibleObject;Z)V")
        }

        #[java_method(name = "setAccessible", descriptor = "(Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setAccessible_z(&self, flag: bool) -> Result<()> {
            panic!("stub: java/lang/reflect/AccessibleObject.setAccessible:(Z)V")
        }

        #[java_method(name = "setAccessible0", descriptor = "(Z)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setAccessible0(&self, flag: bool) -> Result<bool> {
            panic!("stub: java/lang/reflect/AccessibleObject.setAccessible0:(Z)Z")
        }

        #[java_method(name = "trySetAccessible", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn trySetAccessible(&self) -> Result<bool> {
            panic!("stub: java/lang/reflect/AccessibleObject.trySetAccessible:()Z")
        }

        #[java_method(name = "checkCanSetAccessible", descriptor = "(Ljava/lang/Class;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)V")]
        pub fn checkCanSetAccessible_class(&self, caller: Object) -> Result<()> {
            panic!("stub: java/lang/reflect/AccessibleObject.checkCanSetAccessible:(Ljava/lang/Class;)V")
        }

        #[java_method(name = "checkCanSetAccessible", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class<*>;)V")]
        pub fn checkCanSetAccessible_class_class(&self, caller: Object, declaringClass: Object) -> Result<()> {
            panic!("stub: java/lang/reflect/AccessibleObject.checkCanSetAccessible:(Ljava/lang/Class;Ljava/lang/Class;)V")
        }

        #[java_method(name = "checkCanSetAccessible", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;Z)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class<*>;Z)Z")]
        pub fn checkCanSetAccessible_class_class_z(&self, caller: Object, declaringClass: Object, throwExceptionIfDenied: bool) -> Result<bool> {
            panic!("stub: java/lang/reflect/AccessibleObject.checkCanSetAccessible:(Ljava/lang/Class;Ljava/lang/Class;Z)Z")
        }

        #[java_method(name = "throwInaccessibleObjectException", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class<*>;)V")]
        pub fn throwInaccessibleObjectException(&self, caller: Object, declaringClass: Object) -> Result<()> {
            panic!("stub: java/lang/reflect/AccessibleObject.throwInaccessibleObjectException:(Ljava/lang/Class;Ljava/lang/Class;)V")
        }

        #[java_method(name = "isSubclassOf", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class<*>;)Z")]
        pub fn isSubclassOf(&self, queryClass: Object, ofClass: Object) -> Result<bool> {
            panic!("stub: java/lang/reflect/AccessibleObject.isSubclassOf:(Ljava/lang/Class;Ljava/lang/Class;)Z")
        }

        #[java_method(name = "toShortString", descriptor = "()Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toShortString(&self) -> Result<String> {
            panic!("stub: java/lang/reflect/AccessibleObject.toShortString:()Ljava/lang/String;")
        }

        #[java_method(name = "isAccessible", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn isAccessible(&self) -> Result<bool> {
            panic!("stub: java/lang/reflect/AccessibleObject.isAccessible:()Z")
        }

        #[java_method(name = "canAccess", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn canAccess(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/lang/reflect/AccessibleObject.canAccess:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/reflect/AccessibleObject.<init>:()V")
        }

        #[java_method(name = "getAnnotation", descriptor = "(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T::Ljava/lang/annotation/Annotation;>(Ljava/lang/Class<TT;>;)TT;")]
        pub fn getAnnotation(&self, annotationClass: Object) -> Result<Object> {
            panic!("stub: java/lang/reflect/AccessibleObject.getAnnotation:(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "isAnnotationPresent", descriptor = "(Ljava/lang/Class;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<+Ljava/lang/annotation/Annotation;>;)Z")]
        pub fn isAnnotationPresent(&self, annotationClass: Object) -> Result<bool> {
            panic!("stub: java/lang/reflect/AccessibleObject.isAnnotationPresent:(Ljava/lang/Class;)Z")
        }

        #[java_method(name = "getAnnotationsByType", descriptor = "(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T::Ljava/lang/annotation/Annotation;>(Ljava/lang/Class<TT;>;)[TT;")]
        pub fn getAnnotationsByType(&self, annotationClass: Object) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/AccessibleObject.getAnnotationsByType:(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "getAnnotations", descriptor = "()[Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAnnotations(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/AccessibleObject.getAnnotations:()[Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "getDeclaredAnnotation", descriptor = "(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T::Ljava/lang/annotation/Annotation;>(Ljava/lang/Class<TT;>;)TT;")]
        pub fn getDeclaredAnnotation(&self, annotationClass: Object) -> Result<Object> {
            panic!("stub: java/lang/reflect/AccessibleObject.getDeclaredAnnotation:(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "getDeclaredAnnotationsByType", descriptor = "(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T::Ljava/lang/annotation/Annotation;>(Ljava/lang/Class<TT;>;)[TT;")]
        pub fn getDeclaredAnnotationsByType(&self, annotationClass: Object) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/AccessibleObject.getDeclaredAnnotationsByType:(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "getDeclaredAnnotations", descriptor = "()[Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDeclaredAnnotations(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/AccessibleObject.getDeclaredAnnotations:()[Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "isAccessChecked", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class<*>;)Z")]
        pub fn isAccessChecked_class_class(&self, caller: Object, targetClass: Object) -> Result<bool> {
            panic!("stub: java/lang/reflect/AccessibleObject.isAccessChecked:(Ljava/lang/Class;Ljava/lang/Class;)Z")
        }

        #[java_method(name = "isAccessChecked", descriptor = "(Ljava/lang/Class;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z")]
        pub fn isAccessChecked_class(&self, caller: Object) -> Result<bool> {
            panic!("stub: java/lang/reflect/AccessibleObject.isAccessChecked:(Ljava/lang/Class;)Z")
        }

        #[java_method(name = "checkAccess", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalAccessException", generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class<*>;Ljava/lang/Class<*>;I)V")]
        pub fn checkAccess(&self, caller: Object, memberClass: Object, targetClass: Object, modifiers: i32) -> Result<()> {
            panic!("stub: java/lang/reflect/AccessibleObject.checkAccess:(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)V")
        }

        #[java_method(name = "verifyAccess", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)Z", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class<*>;Ljava/lang/Class<*>;I)Z")]
        pub fn verifyAccess(&self, caller: Object, memberClass: Object, targetClass: Object, modifiers: i32) -> Result<bool> {
            panic!("stub: java/lang/reflect/AccessibleObject.verifyAccess:(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)Z")
        }

        #[java_method(name = "slowVerifyAccess", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class<*>;Ljava/lang/Class<*>;I)Z")]
        pub fn slowVerifyAccess(&self, caller: Object, memberClass: Object, targetClass: Object, modifiers: i32) -> Result<bool> {
            panic!("stub: java/lang/reflect/AccessibleObject.slowVerifyAccess:(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)Z")
        }

        #[java_method(name = "printStackTraceWhenAccessFails", descriptor = "()Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn printStackTraceWhenAccessFails() -> Result<bool> {
            panic!("stub: java/lang/reflect/AccessibleObject.printStackTraceWhenAccessFails:()Z")
        }

        #[java_method(name = "getRoot", descriptor = "()Ljava/lang/reflect/AccessibleObject;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getRoot(&self) -> Result<AccessibleObject> {
            panic!("stub: java/lang/reflect/AccessibleObject.getRoot:()Ljava/lang/reflect/AccessibleObject;")
        }
    }
}
