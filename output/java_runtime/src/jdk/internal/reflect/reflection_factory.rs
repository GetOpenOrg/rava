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
use crate::jdk::internal::reflect::MethodAccessor;
use crate::jdk::internal::reflect::Reflection;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "jdk/internal/reflect/ReflectionFactory"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ReflectionFactory.java"]
    #[inner_classes     = "java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25;jdk/internal/reflect/ReflectionFactory$Config:jdk/internal/reflect/ReflectionFactory:Config:26;jdk/internal/reflect/ReflectionFactory$GetReflectionFactoryAction:jdk/internal/reflect/ReflectionFactory:GetReflectionFactoryAction:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;jdk/internal/reflect/ReflectionFactory"]

    pub struct ReflectionFactory {
        #[cfg_attr(any(), java_field(name = "langReflectAccess", descriptor = "Ljdk/internal/access/JavaLangReflectAccess;", access = "private", modifiers = "final", is_static = false))]
        pub langReflectAccess: Object,
    }

    impl ReflectionFactory {
        #[cfg_attr(any(), java_field(name = "soleInstance", descriptor = "Ljdk/internal/reflect/ReflectionFactory;", access = "private", modifiers = "static final", is_static = true))]
        // static field: soleInstance:Ljdk/internal/reflect/ReflectionFactory;
        pub fn soleInstance() -> ReflectionFactory {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.soleInstance:Ljdk/internal/reflect/ReflectionFactory;")
        }

        #[cfg_attr(any(), java_field(name = "hasStaticInitializerMethod", descriptor = "Ljava/lang/reflect/Method;", access = "private", modifiers = "static volatile", is_static = true))]
        // static field: hasStaticInitializerMethod:Ljava/lang/reflect/Method;
        pub fn hasStaticInitializerMethod() -> Method {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.hasStaticInitializerMethod:Ljava/lang/reflect/Method;")
        }

        #[cfg_attr(any(), java_field(name = "METHOD_MH_ACCESSOR", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: METHOD_MH_ACCESSOR:I
        pub fn METHOD_MH_ACCESSOR() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "FIELD_MH_ACCESSOR", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: FIELD_MH_ACCESSOR:I
        pub fn FIELD_MH_ACCESSOR() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "ALL_MH_ACCESSORS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: ALL_MH_ACCESSORS:I
        pub fn ALL_MH_ACCESSORS() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "config", descriptor = "Ljdk/internal/reflect/ReflectionFactory$Config;", access = "private", modifiers = "static", is_static = true))]
        // static field: config:Ljdk/internal/reflect/ReflectionFactory$Config;
        pub fn config_field() -> Object {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.config:Ljdk/internal/reflect/ReflectionFactory$Config;")
        }

        #[cfg_attr(any(), java_field(name = "DEFAULT_CONFIG", descriptor = "Ljdk/internal/reflect/ReflectionFactory$Config;", access = "private", modifiers = "static final", is_static = true))]
        // static field: DEFAULT_CONFIG:Ljdk/internal/reflect/ReflectionFactory$Config;
        pub fn DEFAULT_CONFIG() -> Object {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.DEFAULT_CONFIG:Ljdk/internal/reflect/ReflectionFactory$Config;")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.<init>:()V")
        }

        #[java_method(name = "getReflectionFactory", descriptor = "()Ljdk/internal/reflect/ReflectionFactory;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getReflectionFactory() -> Result<ReflectionFactory> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.getReflectionFactory:()Ljdk/internal/reflect/ReflectionFactory;")
        }

        #[java_method(name = "newFieldAccessor", descriptor = "(Ljava/lang/reflect/Field;Z)Ljdk/internal/reflect/FieldAccessor;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newFieldAccessor(&self, field: Object, override_: bool) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.newFieldAccessor:(Ljava/lang/reflect/Field;Z)Ljdk/internal/reflect/FieldAccessor;")
        }

        #[java_method(name = "newMethodAccessor", descriptor = "(Ljava/lang/reflect/Method;Z)Ljdk/internal/reflect/MethodAccessor;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newMethodAccessor(&self, method: Method, callerSensitive: bool) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.newMethodAccessor:(Ljava/lang/reflect/Method;Z)Ljdk/internal/reflect/MethodAccessor;")
        }

        #[java_method(name = "generateMethodAccessor", descriptor = "(Ljava/lang/reflect/Method;)Ljdk/internal/reflect/MethodAccessorImpl;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn generateMethodAccessor(method: Method) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.generateMethodAccessor:(Ljava/lang/reflect/Method;)Ljdk/internal/reflect/MethodAccessorImpl;")
        }

        #[java_method(name = "newConstructorAccessor", descriptor = "(Ljava/lang/reflect/Constructor;)Ljdk/internal/reflect/ConstructorAccessor;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/reflect/Constructor<*>;)Ljdk/internal/reflect/ConstructorAccessor;")]
        pub fn newConstructorAccessor(&self, c: Object) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.newConstructorAccessor:(Ljava/lang/reflect/Constructor;)Ljdk/internal/reflect/ConstructorAccessor;")
        }

        #[java_method(name = "newConstructor", descriptor = "(Ljava/lang/Class;[Ljava/lang/Class;[Ljava/lang/Class;IILjava/lang/String;[B[B)Ljava/lang/reflect/Constructor;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;[Ljava/lang/Class<*>;[Ljava/lang/Class<*>;IILjava/lang/String;[B[B)Ljava/lang/reflect/Constructor<*>;")]
        pub fn newConstructor(&self, declaringClass: Object, parameterTypes: Rc<RefCell<Vec<Object>>>, checkedExceptions: Rc<RefCell<Vec<Object>>>, modifiers: i32, slot: i32, signature: String, annotations: Rc<RefCell<Vec<i8>>>, parameterAnnotations: Rc<RefCell<Vec<i8>>>) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.newConstructor:(Ljava/lang/Class;[Ljava/lang/Class;[Ljava/lang/Class;IILjava/lang/String;[B[B)Ljava/lang/reflect/Constructor;")
        }

        #[java_method(name = "getConstructorAccessor", descriptor = "(Ljava/lang/reflect/Constructor;)Ljdk/internal/reflect/ConstructorAccessor;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/reflect/Constructor<*>;)Ljdk/internal/reflect/ConstructorAccessor;")]
        pub fn getConstructorAccessor(&self, c: Object) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.getConstructorAccessor:(Ljava/lang/reflect/Constructor;)Ljdk/internal/reflect/ConstructorAccessor;")
        }

        #[java_method(name = "setConstructorAccessor", descriptor = "(Ljava/lang/reflect/Constructor;Ljdk/internal/reflect/ConstructorAccessor;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/reflect/Constructor<*>;Ljdk/internal/reflect/ConstructorAccessor;)V")]
        pub fn setConstructorAccessor(&self, c: Object, accessor: Object) -> Result<()> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.setConstructorAccessor:(Ljava/lang/reflect/Constructor;Ljdk/internal/reflect/ConstructorAccessor;)V")
        }

        #[java_method(name = "copyMethod", descriptor = "(Ljava/lang/reflect/Method;)Ljava/lang/reflect/Method;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyMethod(&self, arg: Method) -> Result<Method> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.copyMethod:(Ljava/lang/reflect/Method;)Ljava/lang/reflect/Method;")
        }

        #[java_method(name = "leafCopyMethod", descriptor = "(Ljava/lang/reflect/Method;)Ljava/lang/reflect/Method;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn leafCopyMethod(&self, arg: Method) -> Result<Method> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.leafCopyMethod:(Ljava/lang/reflect/Method;)Ljava/lang/reflect/Method;")
        }

        #[java_method(name = "copyField", descriptor = "(Ljava/lang/reflect/Field;)Ljava/lang/reflect/Field;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyField(&self, arg: Object) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.copyField:(Ljava/lang/reflect/Field;)Ljava/lang/reflect/Field;")
        }

        #[java_method(name = "copyConstructor", descriptor = "(Ljava/lang/reflect/Constructor;)Ljava/lang/reflect/Constructor;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/lang/reflect/Constructor<TT;>;)Ljava/lang/reflect/Constructor<TT;>;")]
        pub fn copyConstructor(&self, arg: Object) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.copyConstructor:(Ljava/lang/reflect/Constructor;)Ljava/lang/reflect/Constructor;")
        }

        #[java_method(name = "getExecutableTypeAnnotationBytes", descriptor = "(Ljava/lang/reflect/Executable;)[B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getExecutableTypeAnnotationBytes(&self, ex: Executable) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.getExecutableTypeAnnotationBytes:(Ljava/lang/reflect/Executable;)[B")
        }

        #[java_method(name = "getExecutableSharedParameterTypes", descriptor = "(Ljava/lang/reflect/Executable;)[Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/reflect/Executable;)[Ljava/lang/Class<*>;")]
        pub fn getExecutableSharedParameterTypes(&self, ex: Executable) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.getExecutableSharedParameterTypes:(Ljava/lang/reflect/Executable;)[Ljava/lang/Class;")
        }

        #[java_method(name = "newInstance", descriptor = "(Ljava/lang/reflect/Constructor;[Ljava/lang/Object;Ljava/lang/Class;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalAccessException,java/lang/InstantiationException,java/lang/reflect/InvocationTargetException", generic_signature = "<T:Ljava/lang/Object;>(Ljava/lang/reflect/Constructor<TT;>;[Ljava/lang/Object;Ljava/lang/Class<*>;)TT;")]
        pub fn newInstance(&self, ctor: Object, args: Rc<RefCell<Vec<Object>>>, caller: Object) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.newInstance:(Ljava/lang/reflect/Constructor;[Ljava/lang/Object;Ljava/lang/Class;)Ljava/lang/Object;")
        }

        #[java_method(name = "newConstructorForExternalization", descriptor = "(Ljava/lang/Class;)Ljava/lang/reflect/Constructor;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Ljava/lang/reflect/Constructor<*>;")]
        pub fn newConstructorForExternalization(&self, cl: Object) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.newConstructorForExternalization:(Ljava/lang/Class;)Ljava/lang/reflect/Constructor;")
        }

        #[java_method(name = "newConstructorForSerialization", descriptor = "(Ljava/lang/Class;Ljava/lang/reflect/Constructor;)Ljava/lang/reflect/Constructor;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/reflect/Constructor<*>;)Ljava/lang/reflect/Constructor<*>;")]
        pub fn newConstructorForSerialization_class_constr(&self, cl: Object, constructorToCall: Object) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.newConstructorForSerialization:(Ljava/lang/Class;Ljava/lang/reflect/Constructor;)Ljava/lang/reflect/Constructor;")
        }

        #[java_method(name = "superHasAccessibleConstructor", descriptor = "(Ljava/lang/Class;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z")]
        pub fn superHasAccessibleConstructor(&self, cl: Object) -> Result<bool> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.superHasAccessibleConstructor:(Ljava/lang/Class;)Z")
        }

        #[java_method(name = "newConstructorForSerialization", descriptor = "(Ljava/lang/Class;)Ljava/lang/reflect/Constructor;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Ljava/lang/reflect/Constructor<*>;")]
        pub fn newConstructorForSerialization_class(&self, cl: Object) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.newConstructorForSerialization:(Ljava/lang/Class;)Ljava/lang/reflect/Constructor;")
        }

        #[java_method(name = "generateConstructor", descriptor = "(Ljava/lang/Class;Ljava/lang/reflect/Constructor;)Ljava/lang/reflect/Constructor;", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/reflect/Constructor<*>;)Ljava/lang/reflect/Constructor<*>;")]
        pub fn generateConstructor(&self, cl: Object, constructorToCall: Object) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.generateConstructor:(Ljava/lang/Class;Ljava/lang/reflect/Constructor;)Ljava/lang/reflect/Constructor;")
        }

        #[java_method(name = "readObjectForSerialization", descriptor = "(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Ljava/lang/invoke/MethodHandle;")]
        pub fn readObjectForSerialization(&self, cl: Object) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.readObjectForSerialization:(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;")
        }

        #[java_method(name = "readObjectNoDataForSerialization", descriptor = "(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Ljava/lang/invoke/MethodHandle;")]
        pub fn readObjectNoDataForSerialization(&self, cl: Object) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.readObjectNoDataForSerialization:(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;")
        }

        #[java_method(name = "writeObjectForSerialization", descriptor = "(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Ljava/lang/invoke/MethodHandle;")]
        pub fn writeObjectForSerialization(&self, cl: Object) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.writeObjectForSerialization:(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;")
        }

        #[java_method(name = "findReadWriteObjectForSerialization", descriptor = "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/String;Ljava/lang/Class<*>;)Ljava/lang/invoke/MethodHandle;")]
        pub fn findReadWriteObjectForSerialization(&self, cl: Object, methodName: String, streamClass: Object) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.findReadWriteObjectForSerialization:(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;")
        }

        #[java_method(name = "writeReplaceForSerialization", descriptor = "(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Ljava/lang/invoke/MethodHandle;")]
        pub fn writeReplaceForSerialization(&self, cl: Object) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.writeReplaceForSerialization:(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;")
        }

        #[java_method(name = "readResolveForSerialization", descriptor = "(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Ljava/lang/invoke/MethodHandle;")]
        pub fn readResolveForSerialization(&self, cl: Object) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.readResolveForSerialization:(Ljava/lang/Class;)Ljava/lang/invoke/MethodHandle;")
        }

        #[java_method(name = "getReplaceResolveForSerialization", descriptor = "(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/invoke/MethodHandle;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/String;)Ljava/lang/invoke/MethodHandle;")]
        pub fn getReplaceResolveForSerialization(&self, cl: Object, methodName: String) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.getReplaceResolveForSerialization:(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/invoke/MethodHandle;")
        }

        #[java_method(name = "hasStaticInitializerForSerialization", descriptor = "(Ljava/lang/Class;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z")]
        pub fn hasStaticInitializerForSerialization(&self, cl: Object) -> Result<bool> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.hasStaticInitializerForSerialization:(Ljava/lang/Class;)Z")
        }

        #[java_method(name = "newOptionalDataExceptionForSerialization", descriptor = "()Ljava/lang/reflect/Constructor;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/reflect/Constructor<Ljava/io/OptionalDataException;>;")]
        pub fn newOptionalDataExceptionForSerialization(&self) -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.newOptionalDataExceptionForSerialization:()Ljava/lang/reflect/Constructor;")
        }

        #[java_method(name = "inflationThreshold", descriptor = "()I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn inflationThreshold() -> Result<i32> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.inflationThreshold:()I")
        }

        #[java_method(name = "noInflation", descriptor = "()Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn noInflation() -> Result<bool> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.noInflation:()Z")
        }

        #[java_method(name = "useMethodHandleAccessor", descriptor = "()Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn useMethodHandleAccessor() -> Result<bool> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.useMethodHandleAccessor:()Z")
        }

        #[java_method(name = "useFieldHandleAccessor", descriptor = "()Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn useFieldHandleAccessor() -> Result<bool> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.useFieldHandleAccessor:()Z")
        }

        #[java_method(name = "useNativeAccessorOnly", descriptor = "()Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn useNativeAccessorOnly() -> Result<bool> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.useNativeAccessorOnly:()Z")
        }

        #[java_method(name = "disableSerialConstructorChecks", descriptor = "()Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn disableSerialConstructorChecks() -> Result<bool> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.disableSerialConstructorChecks:()Z")
        }

        #[java_method(name = "config", descriptor = "()Ljdk/internal/reflect/ReflectionFactory$Config;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn config() -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.config:()Ljdk/internal/reflect/ReflectionFactory$Config;")
        }

        #[java_method(name = "loadConfig", descriptor = "()Ljdk/internal/reflect/ReflectionFactory$Config;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn loadConfig() -> Result<Object> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.loadConfig:()Ljdk/internal/reflect/ReflectionFactory$Config;")
        }

        #[java_method(name = "packageEquals", descriptor = "(Ljava/lang/Class;Ljava/lang/Class;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/Class<*>;)Z")]
        pub fn packageEquals(cl1: Object, cl2: Object) -> Result<bool> {
            panic!("stub: jdk/internal/reflect/ReflectionFactory.packageEquals:(Ljava/lang/Class;Ljava/lang/Class;)Z")
        }
    }
}
