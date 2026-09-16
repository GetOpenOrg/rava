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
use crate::jdk::internal::reflect::*;
use crate::java::text::Normalizer;
use crate::jdk::internal::misc::VM;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "jdk/internal/reflect/Reflection"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Reflection.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;jdk/internal/reflect/Reflection"]

    pub struct Reflection;

    impl Reflection {
        #[cfg_attr(any(), java_field(name = "fieldFilterMap", descriptor = "Ljava/util/Map;", access = "private", modifiers = "static volatile", is_static = true, generic_signature = "Ljava/util/Map<Ljava/lang/Class<*>;Ljava/util/Set<Ljava/lang/String;>;>;"))]
        // static field: fieldFilterMap:Ljava/util/Map;
        pub fn fieldFilterMap() -> Object {
            panic!("stub: jdk/internal/reflect/Reflection.fieldFilterMap:Ljava/util/Map;")
        }

        #[cfg_attr(any(), java_field(name = "methodFilterMap", descriptor = "Ljava/util/Map;", access = "private", modifiers = "static volatile", is_static = true, generic_signature = "Ljava/util/Map<Ljava/lang/Class<*>;Ljava/util/Set<Ljava/lang/String;>;>;"))]
        // static field: methodFilterMap:Ljava/util/Map;
        pub fn methodFilterMap() -> Object {
            panic!("stub: jdk/internal/reflect/Reflection.methodFilterMap:Ljava/util/Map;")
        }

        #[cfg_attr(any(), java_field(name = "WILDCARD", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "*"))]
        // static field: WILDCARD:Ljava/lang/String;
        pub fn WILDCARD() -> String {
            String::from("*")
        }

        #[cfg_attr(any(), java_field(name = "ALL_MEMBERS", descriptor = "Ljava/util/Set;", access = "public", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Set<Ljava/lang/String;>;"))]
        // static field: ALL_MEMBERS:Ljava/util/Set;
        pub fn ALL_MEMBERS() -> Object {
            panic!("stub: jdk/internal/reflect/Reflection.ALL_MEMBERS:Ljava/util/Set;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: jdk/internal/reflect/Reflection.<init>:()V")
        }

        #[native]
        #[java_native(name = "getCallerClass", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;")]
        pub fn getCallerClass() -> Result<Object> {
            panic!("native: jdk/internal/reflect/Reflection.getCallerClass:()Ljava/lang/Class;")
        }

        #[native]
        #[java_native(name = "getClassAccessFlags", descriptor = "(Ljava/lang/Class;)I", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)I")]
        pub fn getClassAccessFlags(arg0: Object) -> Result<i32> {
            panic!("native: jdk/internal/reflect/Reflection.getClassAccessFlags:(Ljava/lang/Class;)I")
        }

        #[java_method(name = "ensureMemberAccess", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalAccessException", generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class<*>;Ljava/lang/Class<*>;I)V")]
        pub fn ensureMemberAccess(currentClass: Object, memberClass: Object, targetClass: Object, modifiers: i32) -> Result<()> {
            panic!("stub: jdk/internal/reflect/Reflection.ensureMemberAccess:(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)V")
        }

        #[java_method(name = "ensureNativeAccess", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class<*>;Ljava/lang/String;)V")]
        pub fn ensureNativeAccess(currentClass: Object, owner: Object, methodName: String) -> Result<()> {
            panic!("stub: jdk/internal/reflect/Reflection.ensureNativeAccess:(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/String;)V")
        }

        #[java_method(name = "verifyMemberAccess", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class<*>;Ljava/lang/Class<*>;I)Z")]
        pub fn verifyMemberAccess(currentClass: Object, memberClass: Object, targetClass: Object, modifiers: i32) -> Result<bool> {
            panic!("stub: jdk/internal/reflect/Reflection.verifyMemberAccess:(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)Z")
        }

        #[java_method(name = "verifyPublicMemberAccess", descriptor = "(Ljava/lang/Class;I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;I)Z")]
        pub fn verifyPublicMemberAccess(memberClass: Object, modifiers: i32) -> Result<bool> {
            panic!("stub: jdk/internal/reflect/Reflection.verifyPublicMemberAccess:(Ljava/lang/Class;I)Z")
        }

        #[java_method(name = "verifyModuleAccess", descriptor = "(Ljava/lang/Module;Ljava/lang/Class;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Module;Ljava/lang/Class<*>;)Z")]
        pub fn verifyModuleAccess(currentModule: Object, memberClass: Object) -> Result<bool> {
            panic!("stub: jdk/internal/reflect/Reflection.verifyModuleAccess:(Ljava/lang/Module;Ljava/lang/Class;)Z")
        }

        #[java_method(name = "isSameClassPackage", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class<*>;)Z")]
        pub fn isSameClassPackage(c1: Object, c2: Object) -> Result<bool> {
            panic!("stub: jdk/internal/reflect/Reflection.isSameClassPackage:(Ljava/lang/Class;Ljava/lang/Class;)Z")
        }

        #[java_method(name = "isSubclassOf", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class<*>;)Z")]
        pub fn isSubclassOf(queryClass: Object, ofClass: Object) -> Result<bool> {
            panic!("stub: jdk/internal/reflect/Reflection.isSubclassOf:(Ljava/lang/Class;Ljava/lang/Class;)Z")
        }

        #[java_method(name = "registerFieldsToFilter", descriptor = "(Ljava/lang/Class;Ljava/util/Set;)V", access = "public", modifiers = "static synchronized", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/util/Set<Ljava/lang/String;>;)V")]
        pub fn registerFieldsToFilter(containingClass: Object, fieldNames: Object) -> Result<()> {
            panic!("stub: jdk/internal/reflect/Reflection.registerFieldsToFilter:(Ljava/lang/Class;Ljava/util/Set;)V")
        }

        #[java_method(name = "registerMethodsToFilter", descriptor = "(Ljava/lang/Class;Ljava/util/Set;)V", access = "public", modifiers = "static synchronized", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/util/Set<Ljava/lang/String;>;)V")]
        pub fn registerMethodsToFilter(containingClass: Object, methodNames: Object) -> Result<()> {
            panic!("stub: jdk/internal/reflect/Reflection.registerMethodsToFilter:(Ljava/lang/Class;Ljava/util/Set;)V")
        }

        #[java_method(name = "registerFilter", descriptor = "(Ljava/util/Map;Ljava/lang/Class;Ljava/util/Set;)Ljava/util/Map;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/lang/Class<*>;Ljava/util/Set<Ljava/lang/String;>;>;Ljava/lang/Class<*>;Ljava/util/Set<Ljava/lang/String;>;)Ljava/util/Map<Ljava/lang/Class<*>;Ljava/util/Set<Ljava/lang/String;>;>;")]
        pub fn registerFilter(map: Object, containingClass: Object, names: Object) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/Reflection.registerFilter:(Ljava/util/Map;Ljava/lang/Class;Ljava/util/Set;)Ljava/util/Map;")
        }

        #[java_method(name = "filterFields", descriptor = "(Ljava/lang/Class;[Ljava/lang/reflect/Field;)[Ljava/lang/reflect/Field;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;[Ljava/lang/reflect/Field;)[Ljava/lang/reflect/Field;")]
        pub fn filterFields(containingClass: Object, fields: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: jdk/internal/reflect/Reflection.filterFields:(Ljava/lang/Class;[Ljava/lang/reflect/Field;)[Ljava/lang/reflect/Field;")
        }

        #[java_method(name = "filterMethods", descriptor = "(Ljava/lang/Class;[Ljava/lang/reflect/Method;)[Ljava/lang/reflect/Method;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;[Ljava/lang/reflect/Method;)[Ljava/lang/reflect/Method;")]
        pub fn filterMethods(containingClass: Object, methods: Rc<RefCell<Vec<Method>>>) -> Result<Rc<RefCell<Vec<Method>>>> {
            panic!("stub: jdk/internal/reflect/Reflection.filterMethods:(Ljava/lang/Class;[Ljava/lang/reflect/Method;)[Ljava/lang/reflect/Method;")
        }

        #[java_method(name = "filter", descriptor = "([Ljava/lang/reflect/Member;Ljava/util/Set;)[Ljava/lang/reflect/Member;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([Ljava/lang/reflect/Member;Ljava/util/Set<Ljava/lang/String;>;)[Ljava/lang/reflect/Member;")]
        pub fn filter(members: Rc<RefCell<Vec<Object>>>, filteredNames: Object) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: jdk/internal/reflect/Reflection.filter:([Ljava/lang/reflect/Member;Ljava/util/Set;)[Ljava/lang/reflect/Member;")
        }

        #[java_method(name = "isCallerSensitive", descriptor = "(Ljava/lang/reflect/Method;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isCallerSensitive(m: Method) -> Result<bool> {
            panic!("stub: jdk/internal/reflect/Reflection.isCallerSensitive:(Ljava/lang/reflect/Method;)Z")
        }

        #[java_method(name = "isTrustedFinalField", descriptor = "(Ljava/lang/reflect/Field;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isTrustedFinalField(field: Object) -> Result<bool> {
            panic!("stub: jdk/internal/reflect/Reflection.isTrustedFinalField:(Ljava/lang/reflect/Field;)Z")
        }

        #[java_method(name = "newIllegalAccessException", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)Ljava/lang/IllegalAccessException;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class<*>;Ljava/lang/Class<*>;I)Ljava/lang/IllegalAccessException;")]
        pub fn newIllegalAccessException_class_class_class_i(currentClass: Object, memberClass: Object, targetClass: Object, modifiers: i32) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/Reflection.newIllegalAccessException:(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;I)Ljava/lang/IllegalAccessException;")
        }

        #[java_method(name = "newIllegalAccessException", descriptor = "(Ljava/lang/Class;I)Ljava/lang/IllegalAccessException;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;I)Ljava/lang/IllegalAccessException;")]
        pub fn newIllegalAccessException_class_i(memberClass: Object, modifiers: i32) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/Reflection.newIllegalAccessException:(Ljava/lang/Class;I)Ljava/lang/IllegalAccessException;")
        }

        #[java_method(name = "msgSuffix", descriptor = "(I)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn msgSuffix(modifiers: i32) -> Result<String> {
            panic!("stub: jdk/internal/reflect/Reflection.msgSuffix:(I)Ljava/lang/String;")
        }

        #[native]
        #[java_native(name = "areNestMates", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;)Z", access = "public", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class<*>;)Z")]
        pub fn areNestMates(arg0: Object, arg1: Object) -> Result<bool> {
            panic!("native: jdk/internal/reflect/Reflection.areNestMates:(Ljava/lang/Class;Ljava/lang/Class;)Z")
        }
    }
}
