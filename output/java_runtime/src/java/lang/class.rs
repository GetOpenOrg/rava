#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/Class",
    super_class       = "java/lang/Object",
    interfaces        = "java/io/Serializable,java/lang/reflect/GenericDeclaration,java/lang/reflect/Type,java/lang/reflect/AnnotatedElement,java/lang/invoke/TypeDescriptor$OfField,java/lang/constant/Constable",
    access            = "public",
    modifiers         = "final",
    generic_signature = "<T:Ljava/lang/Object;>Ljava/lang/Object;Ljava/io/Serializable;Ljava/lang/reflect/GenericDeclaration;Ljava/lang/reflect/Type;Ljava/lang/reflect/AnnotatedElement;Ljava/lang/invoke/TypeDescriptor$OfField<Ljava/lang/Class<*>;>;Ljava/lang/constant/Constable;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Class.java",
    inner_classes     = "java/lang/Class$1:::0;java/lang/Class$ReflectionData:java/lang/Class:ReflectionData:10;java/lang/reflect/AccessFlag$Location:java/lang/reflect/AccessFlag:Location:16409;java/lang/Class$EnclosingMethodInfo:java/lang/Class:EnclosingMethodInfo:26;java/lang/Class$2:::0;java/lang/Class$Holder:java/lang/Class:Holder:10;java/lang/Class$Atomic:java/lang/Class:Atomic:10;java/lang/PublicMethods$MethodList:java/lang/PublicMethods:MethodList:24;jdk/internal/reflect/ReflectionFactory$GetReflectionFactoryAction:jdk/internal/reflect/ReflectionFactory:GetReflectionFactoryAction:25;java/lang/Class$3:::0;java/lang/Class$AnnotationData:java/lang/Class:AnnotationData:10;java/util/Map$Entry:java/util/Map:Entry:1545;java/lang/invoke/TypeDescriptor$OfField:java/lang/invoke/TypeDescriptor:OfField:1545;java/lang/ClassValue$ClassValueMap:java/lang/ClassValue:ClassValueMap:8;jdk/internal/javac/PreviewFeature$Feature:jdk/internal/javac/PreviewFeature:Feature:16409;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25",
    all_supertypes    = "java/io/Serializable;java/lang/Class;java/lang/Object;java/lang/constant/Constable;java/lang/invoke/TypeDescriptor$OfField;java/lang/reflect/AnnotatedElement;java/lang/reflect/GenericDeclaration;java/lang/reflect/Type",
    has_to_string_method = true,
)]
#[derive(Clone, Default, PartialEq)]
pub struct Class<T: Clone + Default + 'static> {
    #[cfg_attr(any(), java_field(name = "cachedConstructor", descriptor = "Ljava/lang/reflect/Constructor;", access = "private", modifiers = "volatile transient", is_static = false, generic_signature = "Ljava/lang/reflect/Constructor<TT;>;"))]
    pub cachedConstructor: JField<Object>,
    #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private", modifiers = "transient", is_static = false))]
    pub name: JField<String>,
    #[cfg_attr(any(), java_field(name = "module", descriptor = "Ljava/lang/Module;", access = "private", modifiers = "transient", is_static = false))]
    pub module: JField<Object>,
    #[cfg_attr(any(), java_field(name = "classLoader", descriptor = "Ljava/lang/ClassLoader;", access = "private", modifiers = "final", is_static = false))]
    pub classLoader: JField<Object>,
    #[cfg_attr(any(), java_field(name = "classData", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "transient", is_static = false))]
    pub classData: JField<Object>,
    #[cfg_attr(any(), java_field(name = "packageName", descriptor = "Ljava/lang/String;", access = "private", modifiers = "transient", is_static = false))]
    pub packageName: JField<String>,
    #[cfg_attr(any(), java_field(name = "componentType", descriptor = "Ljava/lang/Class;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/lang/Class<*>;"))]
    pub componentType: JField<Class<Object>>,
    #[cfg_attr(any(), java_field(name = "reflectionData", descriptor = "Ljava/lang/ref/SoftReference;", access = "private", modifiers = "volatile transient", is_static = false, generic_signature = "Ljava/lang/ref/SoftReference<Ljava/lang/Class$ReflectionData<TT;>;>;"))]
    pub reflectionData: JField<Object>,
    #[cfg_attr(any(), java_field(name = "classRedefinedCount", descriptor = "I", access = "private", modifiers = "volatile transient", is_static = false))]
    pub classRedefinedCount: JField<i32>,
    #[cfg_attr(any(), java_field(name = "genericInfo", descriptor = "Lsun/reflect/generics/repository/ClassRepository;", access = "private", modifiers = "volatile transient", is_static = false))]
    pub genericInfo: JField<Object>,
    #[cfg_attr(any(), java_field(name = "enumConstants", descriptor = "[Ljava/lang/Object;", access = "private", modifiers = "volatile transient", is_static = false, generic_signature = "[TT;"))]
    pub enumConstants: JField<Rc<RefCell<Vec<T>>>>,
    #[cfg_attr(any(), java_field(name = "enumConstantDirectory", descriptor = "Ljava/util/Map;", access = "private", modifiers = "volatile transient", is_static = false, generic_signature = "Ljava/util/Map<Ljava/lang/String;TT;>;"))]
    pub enumConstantDirectory: JField<Object>,
    #[cfg_attr(any(), java_field(name = "annotationData", descriptor = "Ljava/lang/Class$AnnotationData;", access = "private", modifiers = "volatile transient", is_static = false))]
    pub annotationData: JField<Object>,
    #[cfg_attr(any(), java_field(name = "annotationType", descriptor = "Lsun/reflect/annotation/AnnotationType;", access = "private", modifiers = "volatile transient", is_static = false))]
    pub annotationType: JField<Object>,
    #[cfg_attr(any(), java_field(name = "classValueMap", descriptor = "Ljava/lang/ClassValue$ClassValueMap;", access = "package", modifiers = "transient", is_static = false))]
    pub classValueMap: JField<Object>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + Default + 'static> Class<T> {
    #[cfg_attr(any(), java_field(name = "ANNOTATION", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "8192"))]
    // static field: ANNOTATION:I
    pub fn ANNOTATION() -> i32 {
        8192
    }

    #[cfg_attr(any(), java_field(name = "ENUM", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "16384"))]
    // static field: ENUM:I
    pub fn ENUM() -> i32 {
        16384
    }

    #[cfg_attr(any(), java_field(name = "SYNTHETIC", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "4096"))]
    // static field: SYNTHETIC:I
    pub fn SYNTHETIC() -> i32 {
        4096
    }

    #[cfg_attr(any(), java_field(name = "EMPTY_CLASS_ARRAY", descriptor = "[Ljava/lang/Class;", access = "private", modifiers = "static final", is_static = true, generic_signature = "[Ljava/lang/Class<*>;"))]
    // static field: EMPTY_CLASS_ARRAY:[Ljava/lang/Class;
    pub fn EMPTY_CLASS_ARRAY() -> Rc<RefCell<Vec<Class<Object>>>> {
        Rc::new(RefCell::new(Vec::new()))
    }

    #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "3206093459760846163"))]
    // static field: serialVersionUID:J
    pub fn serialVersionUID() -> i64 {
        3206093459760846163i64
    }

    #[cfg_attr(any(), java_field(name = "serialPersistentFields", descriptor = "[Ljava/io/ObjectStreamField;", access = "private", modifiers = "static final", is_static = true))]
    // static field: serialPersistentFields:[Ljava/io/ObjectStreamField;
    pub fn serialPersistentFields() -> Rc<RefCell<Vec<Object>>> {
        Rc::new(RefCell::new(Vec::new()))
    }

    #[cfg_attr(any(), java_field(name = "reflectionFactory", descriptor = "Ljdk/internal/reflect/ReflectionFactory;", access = "private", modifiers = "static", is_static = true))]
    // static field: reflectionFactory:Ljdk/internal/reflect/ReflectionFactory;
    pub fn reflectionFactory() -> Object {
        panic!("stub: java/lang/Class.reflectionFactory:Ljdk/internal/reflect/ReflectionFactory;")
    }

    #[cfg_attr(any(), java_native(name = "registerNatives", descriptor = "()V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn registerNatives() -> Result<()> {
        panic!("native: java/lang/Class.registerNatives:()V")
    }

    #[java_rta_macros::java_method(name = "<init>", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/Class;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ClassLoader;Ljava/lang/Class<*>;)V")]
    pub fn new(loader: Object, arrayComponentType: Class<Object>) -> Result<Self> {
        panic!("stub: java/lang/Class.<init>:(Ljava/lang/ClassLoader;Ljava/lang/Class;)V")
    }

    #[java_rta_macros::java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn toString(&self) -> Result<String> {
        Ok(String::from(Self::BINARY_NAME))
    }

    #[java_rta_macros::java_method(name = "toGenericString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn toGenericString(&self) -> Result<String> {
        panic!("stub: java/lang/Class.toGenericString:()Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "typeVarBounds", descriptor = "(Ljava/lang/reflect/TypeVariable;)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/reflect/TypeVariable<*>;)Ljava/lang/String;")]
    pub fn typeVarBounds(typeVar: Object) -> Result<String> {
        panic!("stub: java/lang/Class.typeVarBounds:(Ljava/lang/reflect/TypeVariable;)Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "forName", descriptor = "(Ljava/lang/String;)Ljava/lang/Class;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/ClassNotFoundException", generic_signature = "(Ljava/lang/String;)Ljava/lang/Class<*>;")]
    pub fn forName_str(className: String) -> Result<Object> {
        panic!("stub: java/lang/Class.forName:(Ljava/lang/String;)Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "forName", descriptor = "(Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Class;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/ClassNotFoundException", generic_signature = "(Ljava/lang/String;Ljava/lang/Class<*>;)Ljava/lang/Class<*>;")]
    pub fn forName_str_class(className: String, caller: Class<Object>) -> Result<Object> {
        panic!("stub: java/lang/Class.forName:(Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "forName", descriptor = "(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/ClassNotFoundException", generic_signature = "(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class<*>;")]
    pub fn forName_str_z_classl(name: String, initialize: bool, loader: Object) -> Result<Object> {
        panic!("stub: java/lang/Class.forName:(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "forName", descriptor = "(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/ClassNotFoundException", generic_signature = "(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class<*>;)Ljava/lang/Class<*>;")]
    pub fn forName_str_z_classl_class(name: String, initialize: bool, loader: Object, caller: Class<Object>) -> Result<Object> {
        panic!("stub: java/lang/Class.forName:(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;")
    }

    #[cfg_attr(any(), java_native(name = "forName0", descriptor = "(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/ClassNotFoundException", generic_signature = "(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class<*>;)Ljava/lang/Class<*>;"))]
    pub fn forName0(arg0: String, arg1: bool, arg2: Object, arg3: Class<Object>) -> Result<Object> {
        panic!("native: java/lang/Class.forName0:(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "forName", descriptor = "(Ljava/lang/Module;Ljava/lang/String;)Ljava/lang/Class;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Module;Ljava/lang/String;)Ljava/lang/Class<*>;")]
    pub fn forName_module_str(module: Object, name: String) -> Result<Object> {
        panic!("stub: java/lang/Class.forName:(Ljava/lang/Module;Ljava/lang/String;)Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "forName", descriptor = "(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Class;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Class<*>;)Ljava/lang/Class<*>;")]
    pub fn forName_module_str_class(module: Object, name: String, caller: Class<Object>) -> Result<Object> {
        panic!("stub: java/lang/Class.forName:(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "newInstance", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InstantiationException,java/lang/IllegalAccessException", generic_signature = "()TT;", is_deprecated = true)]
    pub fn newInstance(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.newInstance:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_native(name = "isInstance", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn isInstance(&self, arg0: Object) -> Result<bool> {
        panic!("native: java/lang/Class.isInstance:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_native(name = "isAssignableFrom", descriptor = "(Ljava/lang/Class;)Z", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z"))]
    pub fn isAssignableFrom(&self, arg0: Class<Object>) -> Result<bool> {
        panic!("native: java/lang/Class.isAssignableFrom:(Ljava/lang/Class;)Z")
    }

    #[cfg_attr(any(), java_native(name = "isInterface", descriptor = "()Z", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn isInterface(&self) -> Result<bool> {
        panic!("native: java/lang/Class.isInterface:()Z")
    }

    #[cfg_attr(any(), java_native(name = "isArray", descriptor = "()Z", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn isArray(&self) -> Result<bool> {
        panic!("native: java/lang/Class.isArray:()Z")
    }

    #[cfg_attr(any(), java_native(name = "isPrimitive", descriptor = "()Z", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn isPrimitive(&self) -> Result<bool> {
        panic!("native: java/lang/Class.isPrimitive:()Z")
    }

    #[java_rta_macros::java_method(name = "isAnnotation", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn isAnnotation(&self) -> Result<bool> {
        panic!("stub: java/lang/Class.isAnnotation:()Z")
    }

    #[java_rta_macros::java_method(name = "isSynthetic", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn isSynthetic(&self) -> Result<bool> {
        panic!("stub: java/lang/Class.isSynthetic:()Z")
    }

    #[java_rta_macros::java_method(name = "getName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getName(&self) -> Result<String> {
        panic!("stub: java/lang/Class.getName:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_native(name = "initClassName", descriptor = "()Ljava/lang/String;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn initClassName(&self) -> Result<String> {
        panic!("native: java/lang/Class.initClassName:()Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "getClassLoader", descriptor = "()Ljava/lang/ClassLoader;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getClassLoader(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.getClassLoader:()Ljava/lang/ClassLoader;")
    }

    #[java_rta_macros::java_method(name = "getClassLoader0", descriptor = "()Ljava/lang/ClassLoader;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getClassLoader0(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.getClassLoader0:()Ljava/lang/ClassLoader;")
    }

    #[java_rta_macros::java_method(name = "getModule", descriptor = "()Ljava/lang/Module;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getModule(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.getModule:()Ljava/lang/Module;")
    }

    #[java_rta_macros::java_method(name = "getClassData", descriptor = "()Ljava/lang/Object;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getClassData(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.getClassData:()Ljava/lang/Object;")
    }

    #[java_rta_macros::java_method(name = "getTypeParameters", descriptor = "()[Ljava/lang/reflect/TypeVariable;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/reflect/TypeVariable<Ljava/lang/Class<TT;>;>;")]
    pub fn getTypeParameters(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getTypeParameters:()[Ljava/lang/reflect/TypeVariable;")
    }

    #[cfg_attr(any(), java_native(name = "getSuperclass", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<-TT;>;"))]
    pub fn getSuperclass(&self) -> Result<Object> {
        panic!("native: java/lang/Class.getSuperclass:()Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "getGenericSuperclass", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getGenericSuperclass(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.getGenericSuperclass:()Ljava/lang/reflect/Type;")
    }

    #[java_rta_macros::java_method(name = "getPackage", descriptor = "()Ljava/lang/Package;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getPackage(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.getPackage:()Ljava/lang/Package;")
    }

    #[java_rta_macros::java_method(name = "getPackageName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getPackageName(&self) -> Result<String> {
        panic!("stub: java/lang/Class.getPackageName:()Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "getInterfaces", descriptor = "()[Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
    pub fn getInterfaces(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getInterfaces:()[Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "getInterfaces", descriptor = "(Z)[Ljava/lang/Class;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Z)[Ljava/lang/Class<*>;")]
    pub fn getInterfaces_z(&self, cloneArray: bool) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getInterfaces:(Z)[Ljava/lang/Class;")
    }

    #[cfg_attr(any(), java_native(name = "getInterfaces0", descriptor = "()[Ljava/lang/Class;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;"))]
    pub fn getInterfaces0(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("native: java/lang/Class.getInterfaces0:()[Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "getGenericInterfaces", descriptor = "()[Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getGenericInterfaces(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getGenericInterfaces:()[Ljava/lang/reflect/Type;")
    }

    #[java_rta_macros::java_method(name = "getComponentType", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;")]
    pub fn getComponentType(&self) -> Result<Class<Object>> {
        let this = self;
        let _t0 = this.isArray()?;
        if _t0 {
            return Ok(this.componentType.get());
        }
        Ok(Default::default())
    }

    #[java_rta_macros::java_method(name = "elementType", descriptor = "()Ljava/lang/Class;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;")]
    pub fn elementType(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.elementType:()Ljava/lang/Class;")
    }

    #[cfg_attr(any(), java_native(name = "getModifiers", descriptor = "()I", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getModifiers(&self) -> Result<i32> {
        panic!("native: java/lang/Class.getModifiers:()I")
    }

    #[java_rta_macros::java_method(name = "accessFlags", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/lang/reflect/AccessFlag;>;")]
    pub fn accessFlags(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.accessFlags:()Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_native(name = "getSigners", descriptor = "()[Ljava/lang/Object;", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getSigners(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("native: java/lang/Class.getSigners:()[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_native(name = "setSigners", descriptor = "([Ljava/lang/Object;)V", access = "package", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn setSigners(&self, arg0: Rc<RefCell<Vec<Object>>>) -> Result<()> {
        panic!("native: java/lang/Class.setSigners:([Ljava/lang/Object;)V")
    }

    #[java_rta_macros::java_method(name = "getEnclosingMethod", descriptor = "()Ljava/lang/reflect/Method;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException")]
    pub fn getEnclosingMethod(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.getEnclosingMethod:()Ljava/lang/reflect/Method;")
    }

    #[cfg_attr(any(), java_native(name = "getEnclosingMethod0", descriptor = "()[Ljava/lang/Object;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getEnclosingMethod0(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("native: java/lang/Class.getEnclosingMethod0:()[Ljava/lang/Object;")
    }

    #[java_rta_macros::java_method(name = "getEnclosingMethodInfo", descriptor = "()Ljava/lang/Class$EnclosingMethodInfo;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getEnclosingMethodInfo(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.getEnclosingMethodInfo:()Ljava/lang/Class$EnclosingMethodInfo;")
    }

    #[java_rta_macros::java_method(name = "toClass", descriptor = "(Ljava/lang/reflect/Type;)Ljava/lang/Class;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/reflect/Type;)Ljava/lang/Class<*>;")]
    pub fn toClass(o: Object) -> Result<Object> {
        panic!("stub: java/lang/Class.toClass:(Ljava/lang/reflect/Type;)Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "getEnclosingConstructor", descriptor = "()Ljava/lang/reflect/Constructor;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException", generic_signature = "()Ljava/lang/reflect/Constructor<*>;")]
    pub fn getEnclosingConstructor(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.getEnclosingConstructor:()Ljava/lang/reflect/Constructor;")
    }

    #[java_rta_macros::java_method(name = "getDeclaringClass", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException", generic_signature = "()Ljava/lang/Class<*>;")]
    pub fn getDeclaringClass(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.getDeclaringClass:()Ljava/lang/Class;")
    }

    #[cfg_attr(any(), java_native(name = "getDeclaringClass0", descriptor = "()Ljava/lang/Class;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;"))]
    pub fn getDeclaringClass0(&self) -> Result<Object> {
        panic!("native: java/lang/Class.getDeclaringClass0:()Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "getEnclosingClass", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException", generic_signature = "()Ljava/lang/Class<*>;")]
    pub fn getEnclosingClass(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.getEnclosingClass:()Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "getSimpleName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getSimpleName(&self) -> Result<String> {
        panic!("stub: java/lang/Class.getSimpleName:()Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "getSimpleName0", descriptor = "()Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getSimpleName0(&self) -> Result<String> {
        panic!("stub: java/lang/Class.getSimpleName0:()Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "getTypeName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getTypeName(&self) -> Result<String> {
        panic!("stub: java/lang/Class.getTypeName:()Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "getCanonicalName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getCanonicalName(&self) -> Result<String> {
        panic!("stub: java/lang/Class.getCanonicalName:()Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "getCanonicalName0", descriptor = "()Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getCanonicalName0(&self) -> Result<String> {
        panic!("stub: java/lang/Class.getCanonicalName0:()Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "isUnnamedClass", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn isUnnamedClass(&self) -> Result<bool> {
        panic!("stub: java/lang/Class.isUnnamedClass:()Z")
    }

    #[java_rta_macros::java_method(name = "isAnonymousClass", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn isAnonymousClass(&self) -> Result<bool> {
        panic!("stub: java/lang/Class.isAnonymousClass:()Z")
    }

    #[java_rta_macros::java_method(name = "isLocalClass", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn isLocalClass(&self) -> Result<bool> {
        panic!("stub: java/lang/Class.isLocalClass:()Z")
    }

    #[java_rta_macros::java_method(name = "isMemberClass", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn isMemberClass(&self) -> Result<bool> {
        panic!("stub: java/lang/Class.isMemberClass:()Z")
    }

    #[java_rta_macros::java_method(name = "getSimpleBinaryName", descriptor = "()Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getSimpleBinaryName(&self) -> Result<String> {
        panic!("stub: java/lang/Class.getSimpleBinaryName:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_native(name = "getSimpleBinaryName0", descriptor = "()Ljava/lang/String;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getSimpleBinaryName0(&self) -> Result<String> {
        panic!("native: java/lang/Class.getSimpleBinaryName0:()Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "isTopLevelClass", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn isTopLevelClass(&self) -> Result<bool> {
        panic!("stub: java/lang/Class.isTopLevelClass:()Z")
    }

    #[java_rta_macros::java_method(name = "isLocalOrAnonymousClass", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn isLocalOrAnonymousClass(&self) -> Result<bool> {
        panic!("stub: java/lang/Class.isLocalOrAnonymousClass:()Z")
    }

    #[java_rta_macros::java_method(name = "hasEnclosingMethodInfo", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn hasEnclosingMethodInfo(&self) -> Result<bool> {
        panic!("stub: java/lang/Class.hasEnclosingMethodInfo:()Z")
    }

    #[java_rta_macros::java_method(name = "getClasses", descriptor = "()[Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
    pub fn getClasses(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getClasses:()[Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "getFields", descriptor = "()[Ljava/lang/reflect/Field;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException")]
    pub fn getFields(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getFields:()[Ljava/lang/reflect/Field;")
    }

    #[java_rta_macros::java_method(name = "getMethods", descriptor = "()[Ljava/lang/reflect/Method;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException")]
    pub fn getMethods(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getMethods:()[Ljava/lang/reflect/Method;")
    }

    #[java_rta_macros::java_method(name = "getConstructors", descriptor = "()[Ljava/lang/reflect/Constructor;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException", generic_signature = "()[Ljava/lang/reflect/Constructor<*>;")]
    pub fn getConstructors(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getConstructors:()[Ljava/lang/reflect/Constructor;")
    }

    #[java_rta_macros::java_method(name = "getField", descriptor = "(Ljava/lang/String;)Ljava/lang/reflect/Field;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NoSuchFieldException,java/lang/SecurityException")]
    pub fn getField(&self, name: String) -> Result<Object> {
        panic!("stub: java/lang/Class.getField:(Ljava/lang/String;)Ljava/lang/reflect/Field;")
    }

    #[java_rta_macros::java_method(name = "getMethod", descriptor = "(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NoSuchMethodException,java/lang/SecurityException", generic_signature = "(Ljava/lang/String;[Ljava/lang/Class<*>;)Ljava/lang/reflect/Method;")]
    pub fn getMethod(&self, name: String, parameterTypes: Rc<RefCell<Vec<Class<Object>>>>) -> Result<Object> {
        panic!("stub: java/lang/Class.getMethod:(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;")
    }

    #[java_rta_macros::java_method(name = "getConstructor", descriptor = "([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NoSuchMethodException,java/lang/SecurityException", generic_signature = "([Ljava/lang/Class<*>;)Ljava/lang/reflect/Constructor<TT;>;")]
    pub fn getConstructor(&self, parameterTypes: Rc<RefCell<Vec<Class<Object>>>>) -> Result<Object> {
        panic!("stub: java/lang/Class.getConstructor:([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;")
    }

    #[java_rta_macros::java_method(name = "getDeclaredClasses", descriptor = "()[Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException", generic_signature = "()[Ljava/lang/Class<*>;")]
    pub fn getDeclaredClasses(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getDeclaredClasses:()[Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "getDeclaredFields", descriptor = "()[Ljava/lang/reflect/Field;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException")]
    pub fn getDeclaredFields(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getDeclaredFields:()[Ljava/lang/reflect/Field;")
    }

    #[java_rta_macros::java_method(name = "getRecordComponents", descriptor = "()[Ljava/lang/reflect/RecordComponent;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getRecordComponents(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getRecordComponents:()[Ljava/lang/reflect/RecordComponent;")
    }

    #[java_rta_macros::java_method(name = "getDeclaredMethods", descriptor = "()[Ljava/lang/reflect/Method;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException")]
    pub fn getDeclaredMethods(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getDeclaredMethods:()[Ljava/lang/reflect/Method;")
    }

    #[java_rta_macros::java_method(name = "getDeclaredConstructors", descriptor = "()[Ljava/lang/reflect/Constructor;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException", generic_signature = "()[Ljava/lang/reflect/Constructor<*>;")]
    pub fn getDeclaredConstructors(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getDeclaredConstructors:()[Ljava/lang/reflect/Constructor;")
    }

    #[java_rta_macros::java_method(name = "getDeclaredField", descriptor = "(Ljava/lang/String;)Ljava/lang/reflect/Field;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NoSuchFieldException,java/lang/SecurityException")]
    pub fn getDeclaredField(&self, name: String) -> Result<Object> {
        panic!("stub: java/lang/Class.getDeclaredField:(Ljava/lang/String;)Ljava/lang/reflect/Field;")
    }

    #[java_rta_macros::java_method(name = "getDeclaredMethod", descriptor = "(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NoSuchMethodException,java/lang/SecurityException", generic_signature = "(Ljava/lang/String;[Ljava/lang/Class<*>;)Ljava/lang/reflect/Method;")]
    pub fn getDeclaredMethod(&self, name: String, parameterTypes: Rc<RefCell<Vec<Class<Object>>>>) -> Result<Object> {
        panic!("stub: java/lang/Class.getDeclaredMethod:(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;")
    }

    #[java_rta_macros::java_method(name = "getDeclaredPublicMethods", descriptor = "(Ljava/lang/String;[Ljava/lang/Class;)Ljava/util/List;", access = "package", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;[Ljava/lang/Class<*>;)Ljava/util/List<Ljava/lang/reflect/Method;>;")]
    pub fn getDeclaredPublicMethods(&self, name: String, parameterTypes: Rc<RefCell<Vec<Class<Object>>>>) -> Result<Object> {
        panic!("stub: java/lang/Class.getDeclaredPublicMethods:(Ljava/lang/String;[Ljava/lang/Class;)Ljava/util/List;")
    }

    #[java_rta_macros::java_method(name = "getDeclaredConstructor", descriptor = "([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NoSuchMethodException,java/lang/SecurityException", generic_signature = "([Ljava/lang/Class<*>;)Ljava/lang/reflect/Constructor<TT;>;")]
    pub fn getDeclaredConstructor(&self, parameterTypes: Rc<RefCell<Vec<Class<Object>>>>) -> Result<Object> {
        panic!("stub: java/lang/Class.getDeclaredConstructor:([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;")
    }

    #[java_rta_macros::java_method(name = "getResourceAsStream", descriptor = "(Ljava/lang/String;)Ljava/io/InputStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getResourceAsStream(&self, name: String) -> Result<Object> {
        panic!("stub: java/lang/Class.getResourceAsStream:(Ljava/lang/String;)Ljava/io/InputStream;")
    }

    #[java_rta_macros::java_method(name = "getResource", descriptor = "(Ljava/lang/String;)Ljava/net/URL;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getResource(&self, name: String) -> Result<Object> {
        panic!("stub: java/lang/Class.getResource:(Ljava/lang/String;)Ljava/net/URL;")
    }

    #[java_rta_macros::java_method(name = "isOpenToCaller", descriptor = "(Ljava/lang/String;Ljava/lang/Class;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;Ljava/lang/Class<*>;)Z")]
    pub fn isOpenToCaller(&self, name: String, caller: Class<Object>) -> Result<bool> {
        panic!("stub: java/lang/Class.isOpenToCaller:(Ljava/lang/String;Ljava/lang/Class;)Z")
    }

    #[java_rta_macros::java_method(name = "getProtectionDomain", descriptor = "()Ljava/security/ProtectionDomain;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getProtectionDomain(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.getProtectionDomain:()Ljava/security/ProtectionDomain;")
    }

    #[java_rta_macros::java_method(name = "protectionDomain", descriptor = "()Ljava/security/ProtectionDomain;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn protectionDomain(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.protectionDomain:()Ljava/security/ProtectionDomain;")
    }

    #[cfg_attr(any(), java_native(name = "getProtectionDomain0", descriptor = "()Ljava/security/ProtectionDomain;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getProtectionDomain0(&self) -> Result<Object> {
        panic!("native: java/lang/Class.getProtectionDomain0:()Ljava/security/ProtectionDomain;")
    }

    #[cfg_attr(any(), java_native(name = "getPrimitiveClass", descriptor = "(Ljava/lang/String;)Ljava/lang/Class;", access = "package", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;)Ljava/lang/Class<*>;"))]
    pub fn getPrimitiveClass(arg0: String) -> Result<Object> {
        panic!("native: java/lang/Class.getPrimitiveClass:(Ljava/lang/String;)Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "checkMemberAccess", descriptor = "(Ljava/lang/SecurityManager;ILjava/lang/Class;Z)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/SecurityManager;ILjava/lang/Class<*>;Z)V")]
    pub fn checkMemberAccess(&self, sm: SecurityManager, which: i32, caller: Class<Object>, checkProxyInterfaces: bool) -> Result<()> {
        panic!("stub: java/lang/Class.checkMemberAccess:(Ljava/lang/SecurityManager;ILjava/lang/Class;Z)V")
    }

    #[java_rta_macros::java_method(name = "checkPackageAccess", descriptor = "(Ljava/lang/SecurityManager;Ljava/lang/ClassLoader;Z)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn checkPackageAccess(&self, sm: SecurityManager, ccl: Object, checkProxyInterfaces: bool) -> Result<()> {
        panic!("stub: java/lang/Class.checkPackageAccess:(Ljava/lang/SecurityManager;Ljava/lang/ClassLoader;Z)V")
    }

    #[java_rta_macros::java_method(name = "checkPackageAccessForPermittedSubclasses", descriptor = "(Ljava/lang/SecurityManager;Ljava/lang/ClassLoader;[Ljava/lang/Class;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/SecurityManager;Ljava/lang/ClassLoader;[Ljava/lang/Class<*>;)V")]
    pub fn checkPackageAccessForPermittedSubclasses(sm: SecurityManager, ccl: Object, subClasses: Rc<RefCell<Vec<Class<Object>>>>) -> Result<()> {
        panic!("stub: java/lang/Class.checkPackageAccessForPermittedSubclasses:(Ljava/lang/SecurityManager;Ljava/lang/ClassLoader;[Ljava/lang/Class;)V")
    }

    #[java_rta_macros::java_method(name = "resolveName", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn resolveName(&self, name: String) -> Result<String> {
        panic!("stub: java/lang/Class.resolveName:(Ljava/lang/String;)Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "reflectionData", descriptor = "()Ljava/lang/Class$ReflectionData;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class$ReflectionData<TT;>;")]
    pub fn reflectionData(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.reflectionData:()Ljava/lang/Class$ReflectionData;")
    }

    #[java_rta_macros::java_method(name = "newReflectionData", descriptor = "(Ljava/lang/ref/SoftReference;I)Ljava/lang/Class$ReflectionData;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ref/SoftReference<Ljava/lang/Class$ReflectionData<TT;>;>;I)Ljava/lang/Class$ReflectionData<TT;>;")]
    pub fn newReflectionData(&self, oldReflectionData: Object, classRedefinedCount: i32) -> Result<Object> {
        panic!("stub: java/lang/Class.newReflectionData:(Ljava/lang/ref/SoftReference;I)Ljava/lang/Class$ReflectionData;")
    }

    #[cfg_attr(any(), java_native(name = "getGenericSignature0", descriptor = "()Ljava/lang/String;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getGenericSignature0(&self) -> Result<String> {
        panic!("native: java/lang/Class.getGenericSignature0:()Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "getFactory", descriptor = "()Lsun/reflect/generics/factory/GenericsFactory;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getFactory(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.getFactory:()Lsun/reflect/generics/factory/GenericsFactory;")
    }

    #[java_rta_macros::java_method(name = "getGenericInfo", descriptor = "()Lsun/reflect/generics/repository/ClassRepository;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getGenericInfo(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.getGenericInfo:()Lsun/reflect/generics/repository/ClassRepository;")
    }

    #[cfg_attr(any(), java_native(name = "getRawAnnotations", descriptor = "()[B", access = "package", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getRawAnnotations(&self) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("native: java/lang/Class.getRawAnnotations:()[B")
    }

    #[cfg_attr(any(), java_native(name = "getRawTypeAnnotations", descriptor = "()[B", access = "package", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getRawTypeAnnotations(&self) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("native: java/lang/Class.getRawTypeAnnotations:()[B")
    }

    #[java_rta_macros::java_method(name = "getExecutableTypeAnnotationBytes", descriptor = "(Ljava/lang/reflect/Executable;)[B", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getExecutableTypeAnnotationBytes(ex: Object) -> Result<Rc<RefCell<Vec<i8>>>> {
        panic!("stub: java/lang/Class.getExecutableTypeAnnotationBytes:(Ljava/lang/reflect/Executable;)[B")
    }

    #[cfg_attr(any(), java_native(name = "getConstantPool", descriptor = "()Ljdk/internal/reflect/ConstantPool;", access = "package", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getConstantPool(&self) -> Result<Object> {
        panic!("native: java/lang/Class.getConstantPool:()Ljdk/internal/reflect/ConstantPool;")
    }

    #[java_rta_macros::java_method(name = "privateGetDeclaredFields", descriptor = "(Z)[Ljava/lang/reflect/Field;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn privateGetDeclaredFields(&self, publicOnly: bool) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.privateGetDeclaredFields:(Z)[Ljava/lang/reflect/Field;")
    }

    #[java_rta_macros::java_method(name = "privateGetPublicFields", descriptor = "()[Ljava/lang/reflect/Field;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn privateGetPublicFields(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.privateGetPublicFields:()[Ljava/lang/reflect/Field;")
    }

    #[java_rta_macros::java_method(name = "addAll", descriptor = "(Ljava/util/Collection;[Ljava/lang/reflect/Field;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<Ljava/lang/reflect/Field;>;[Ljava/lang/reflect/Field;)V")]
    pub fn addAll(c: Object, o: Rc<RefCell<Vec<Object>>>) -> Result<()> {
        panic!("stub: java/lang/Class.addAll:(Ljava/util/Collection;[Ljava/lang/reflect/Field;)V")
    }

    #[java_rta_macros::java_method(name = "privateGetDeclaredConstructors", descriptor = "(Z)[Ljava/lang/reflect/Constructor;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Z)[Ljava/lang/reflect/Constructor<TT;>;")]
    pub fn privateGetDeclaredConstructors(&self, publicOnly: bool) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.privateGetDeclaredConstructors:(Z)[Ljava/lang/reflect/Constructor;")
    }

    #[java_rta_macros::java_method(name = "privateGetDeclaredMethods", descriptor = "(Z)[Ljava/lang/reflect/Method;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn privateGetDeclaredMethods(&self, publicOnly: bool) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.privateGetDeclaredMethods:(Z)[Ljava/lang/reflect/Method;")
    }

    #[java_rta_macros::java_method(name = "privateGetPublicMethods", descriptor = "()[Ljava/lang/reflect/Method;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn privateGetPublicMethods(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.privateGetPublicMethods:()[Ljava/lang/reflect/Method;")
    }

    #[java_rta_macros::java_method(name = "searchFields", descriptor = "([Ljava/lang/reflect/Field;Ljava/lang/String;)Ljava/lang/reflect/Field;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn searchFields(fields: Rc<RefCell<Vec<Object>>>, name: String) -> Result<Object> {
        panic!("stub: java/lang/Class.searchFields:([Ljava/lang/reflect/Field;Ljava/lang/String;)Ljava/lang/reflect/Field;")
    }

    #[java_rta_macros::java_method(name = "getField0", descriptor = "(Ljava/lang/String;)Ljava/lang/reflect/Field;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getField0(&self, name: String) -> Result<Object> {
        panic!("stub: java/lang/Class.getField0:(Ljava/lang/String;)Ljava/lang/reflect/Field;")
    }

    #[java_rta_macros::java_method(name = "searchMethods", descriptor = "([Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class<*>;)Ljava/lang/reflect/Method;")]
    pub fn searchMethods(methods: Rc<RefCell<Vec<Object>>>, name: String, parameterTypes: Rc<RefCell<Vec<Class<Object>>>>) -> Result<Object> {
        panic!("stub: java/lang/Class.searchMethods:([Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;")
    }

    #[java_rta_macros::java_method(name = "getMethod0", descriptor = "(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;[Ljava/lang/Class<*>;)Ljava/lang/reflect/Method;")]
    pub fn getMethod0(&self, name: String, parameterTypes: Rc<RefCell<Vec<Class<Object>>>>) -> Result<Object> {
        panic!("stub: java/lang/Class.getMethod0:(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;")
    }

    #[java_rta_macros::java_method(name = "getMethodsRecursive", descriptor = "(Ljava/lang/String;[Ljava/lang/Class;Z)Ljava/lang/PublicMethods$MethodList;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;[Ljava/lang/Class<*>;Z)Ljava/lang/PublicMethods$MethodList;")]
    pub fn getMethodsRecursive(&self, name: String, parameterTypes: Rc<RefCell<Vec<Class<Object>>>>, includeStatic: bool) -> Result<Object> {
        panic!("stub: java/lang/Class.getMethodsRecursive:(Ljava/lang/String;[Ljava/lang/Class;Z)Ljava/lang/PublicMethods$MethodList;")
    }

    #[java_rta_macros::java_method(name = "getConstructor0", descriptor = "([Ljava/lang/Class;I)Ljava/lang/reflect/Constructor;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NoSuchMethodException", generic_signature = "([Ljava/lang/Class<*>;I)Ljava/lang/reflect/Constructor<TT;>;")]
    pub fn getConstructor0(&self, parameterTypes: Rc<RefCell<Vec<Class<Object>>>>, which: i32) -> Result<Object> {
        panic!("stub: java/lang/Class.getConstructor0:([Ljava/lang/Class;I)Ljava/lang/reflect/Constructor;")
    }

    #[java_rta_macros::java_method(name = "arrayContentsEq", descriptor = "([Ljava/lang/Object;[Ljava/lang/Object;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn arrayContentsEq(a1: Rc<RefCell<Vec<Object>>>, a2: Rc<RefCell<Vec<Object>>>) -> Result<bool> {
        panic!("stub: java/lang/Class.arrayContentsEq:([Ljava/lang/Object;[Ljava/lang/Object;)Z")
    }

    #[java_rta_macros::java_method(name = "copyFields", descriptor = "([Ljava/lang/reflect/Field;)[Ljava/lang/reflect/Field;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn copyFields(arg: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.copyFields:([Ljava/lang/reflect/Field;)[Ljava/lang/reflect/Field;")
    }

    #[java_rta_macros::java_method(name = "copyMethods", descriptor = "([Ljava/lang/reflect/Method;)[Ljava/lang/reflect/Method;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn copyMethods(arg: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.copyMethods:([Ljava/lang/reflect/Method;)[Ljava/lang/reflect/Method;")
    }

    #[java_rta_macros::java_method(name = "copyConstructors", descriptor = "([Ljava/lang/reflect/Constructor;)[Ljava/lang/reflect/Constructor;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>([Ljava/lang/reflect/Constructor<TU;>;)[Ljava/lang/reflect/Constructor<TU;>;")]
    pub fn copyConstructors(arg: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.copyConstructors:([Ljava/lang/reflect/Constructor;)[Ljava/lang/reflect/Constructor;")
    }

    #[cfg_attr(any(), java_native(name = "getDeclaredFields0", descriptor = "(Z)[Ljava/lang/reflect/Field;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getDeclaredFields0(&self, arg0: bool) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("native: java/lang/Class.getDeclaredFields0:(Z)[Ljava/lang/reflect/Field;")
    }

    #[cfg_attr(any(), java_native(name = "getDeclaredMethods0", descriptor = "(Z)[Ljava/lang/reflect/Method;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getDeclaredMethods0(&self, arg0: bool) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("native: java/lang/Class.getDeclaredMethods0:(Z)[Ljava/lang/reflect/Method;")
    }

    #[cfg_attr(any(), java_native(name = "getDeclaredConstructors0", descriptor = "(Z)[Ljava/lang/reflect/Constructor;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Z)[Ljava/lang/reflect/Constructor<TT;>;"))]
    pub fn getDeclaredConstructors0(&self, arg0: bool) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("native: java/lang/Class.getDeclaredConstructors0:(Z)[Ljava/lang/reflect/Constructor;")
    }

    #[cfg_attr(any(), java_native(name = "getDeclaredClasses0", descriptor = "()[Ljava/lang/Class;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;"))]
    pub fn getDeclaredClasses0(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("native: java/lang/Class.getDeclaredClasses0:()[Ljava/lang/Class;")
    }

    #[cfg_attr(any(), java_native(name = "getRecordComponents0", descriptor = "()[Ljava/lang/reflect/RecordComponent;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getRecordComponents0(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("native: java/lang/Class.getRecordComponents0:()[Ljava/lang/reflect/RecordComponent;")
    }

    #[cfg_attr(any(), java_native(name = "isRecord0", descriptor = "()Z", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn isRecord0(&self) -> Result<bool> {
        panic!("native: java/lang/Class.isRecord0:()Z")
    }

    #[java_rta_macros::java_method(name = "methodToString", descriptor = "(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;[Ljava/lang/Class<*>;)Ljava/lang/String;")]
    pub fn methodToString(&self, name: String, argTypes: Rc<RefCell<Vec<Class<Object>>>>) -> Result<String> {
        panic!("stub: java/lang/Class.methodToString:(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "desiredAssertionStatus", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn desiredAssertionStatus(&self) -> Result<bool> {
        panic!("stub: java/lang/Class.desiredAssertionStatus:()Z")
    }

    #[cfg_attr(any(), java_native(name = "desiredAssertionStatus0", descriptor = "(Ljava/lang/Class;)Z", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z"))]
    pub fn desiredAssertionStatus0(arg0: Class<Object>) -> Result<bool> {
        panic!("native: java/lang/Class.desiredAssertionStatus0:(Ljava/lang/Class;)Z")
    }

    #[java_rta_macros::java_method(name = "isEnum", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn isEnum(&self) -> Result<bool> {
        panic!("stub: java/lang/Class.isEnum:()Z")
    }

    #[java_rta_macros::java_method(name = "isRecord", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn isRecord(&self) -> Result<bool> {
        panic!("stub: java/lang/Class.isRecord:()Z")
    }

    #[java_rta_macros::java_method(name = "getReflectionFactory", descriptor = "()Ljdk/internal/reflect/ReflectionFactory;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getReflectionFactory() -> Result<Object> {
        panic!("stub: java/lang/Class.getReflectionFactory:()Ljdk/internal/reflect/ReflectionFactory;")
    }

    #[java_rta_macros::java_method(name = "getEnumConstants", descriptor = "()[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[TT;")]
    pub fn getEnumConstants(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getEnumConstants:()[Ljava/lang/Object;")
    }

    #[java_rta_macros::java_method(name = "getEnumConstantsShared", descriptor = "()[Ljava/lang/Object;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[TT;")]
    pub fn getEnumConstantsShared(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getEnumConstantsShared:()[Ljava/lang/Object;")
    }

    #[java_rta_macros::java_method(name = "enumConstantDirectory", descriptor = "()Ljava/util/Map;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Map<Ljava/lang/String;TT;>;")]
    pub fn enumConstantDirectory(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.enumConstantDirectory:()Ljava/util/Map;")
    }

    #[java_rta_macros::java_method(name = "cast", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TT;")]
    pub fn cast(&self, obj: Object) -> Result<Object> {
        panic!("stub: java/lang/Class.cast:(Ljava/lang/Object;)Ljava/lang/Object;")
    }

    #[java_rta_macros::java_method(name = "cannotCastMsg", descriptor = "(Ljava/lang/Object;)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn cannotCastMsg(&self, obj: Object) -> Result<String> {
        panic!("stub: java/lang/Class.cannotCastMsg:(Ljava/lang/Object;)Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "asSubclass", descriptor = "(Ljava/lang/Class;)Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(Ljava/lang/Class<TU;>;)Ljava/lang/Class<+TU;>;")]
    pub fn asSubclass(&self, clazz: Class<Object>) -> Result<Object> {
        panic!("stub: java/lang/Class.asSubclass:(Ljava/lang/Class;)Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "getAnnotation", descriptor = "(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<A::Ljava/lang/annotation/Annotation;>(Ljava/lang/Class<TA;>;)TA;")]
    pub fn getAnnotation(&self, annotationClass: Class<Object>) -> Result<Object> {
        panic!("stub: java/lang/Class.getAnnotation:(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;")
    }

    #[java_rta_macros::java_method(name = "isAnnotationPresent", descriptor = "(Ljava/lang/Class;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<+Ljava/lang/annotation/Annotation;>;)Z")]
    pub fn isAnnotationPresent(&self, annotationClass: Object) -> Result<bool> {
        panic!("stub: java/lang/Class.isAnnotationPresent:(Ljava/lang/Class;)Z")
    }

    #[java_rta_macros::java_method(name = "getAnnotationsByType", descriptor = "(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<A::Ljava/lang/annotation/Annotation;>(Ljava/lang/Class<TA;>;)[TA;")]
    pub fn getAnnotationsByType(&self, annotationClass: Class<Object>) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getAnnotationsByType:(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;")
    }

    #[java_rta_macros::java_method(name = "getAnnotations", descriptor = "()[Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getAnnotations(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getAnnotations:()[Ljava/lang/annotation/Annotation;")
    }

    #[java_rta_macros::java_method(name = "getDeclaredAnnotation", descriptor = "(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<A::Ljava/lang/annotation/Annotation;>(Ljava/lang/Class<TA;>;)TA;")]
    pub fn getDeclaredAnnotation(&self, annotationClass: Class<Object>) -> Result<Object> {
        panic!("stub: java/lang/Class.getDeclaredAnnotation:(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;")
    }

    #[java_rta_macros::java_method(name = "getDeclaredAnnotationsByType", descriptor = "(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<A::Ljava/lang/annotation/Annotation;>(Ljava/lang/Class<TA;>;)[TA;")]
    pub fn getDeclaredAnnotationsByType(&self, annotationClass: Class<Object>) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getDeclaredAnnotationsByType:(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;")
    }

    #[java_rta_macros::java_method(name = "getDeclaredAnnotations", descriptor = "()[Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getDeclaredAnnotations(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getDeclaredAnnotations:()[Ljava/lang/annotation/Annotation;")
    }

    #[java_rta_macros::java_method(name = "annotationData", descriptor = "()Ljava/lang/Class$AnnotationData;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn annotationData(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.annotationData:()Ljava/lang/Class$AnnotationData;")
    }

    #[java_rta_macros::java_method(name = "createAnnotationData", descriptor = "(I)Ljava/lang/Class$AnnotationData;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn createAnnotationData(&self, classRedefinedCount: i32) -> Result<Object> {
        panic!("stub: java/lang/Class.createAnnotationData:(I)Ljava/lang/Class$AnnotationData;")
    }

    #[java_rta_macros::java_method(name = "casAnnotationType", descriptor = "(Lsun/reflect/annotation/AnnotationType;Lsun/reflect/annotation/AnnotationType;)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn casAnnotationType(&self, oldType: Object, newType: Object) -> Result<bool> {
        panic!("stub: java/lang/Class.casAnnotationType:(Lsun/reflect/annotation/AnnotationType;Lsun/reflect/annotation/AnnotationType;)Z")
    }

    #[java_rta_macros::java_method(name = "getAnnotationType", descriptor = "()Lsun/reflect/annotation/AnnotationType;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getAnnotationType(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.getAnnotationType:()Lsun/reflect/annotation/AnnotationType;")
    }

    #[java_rta_macros::java_method(name = "getDeclaredAnnotationMap", descriptor = "()Ljava/util/Map;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Map<Ljava/lang/Class<+Ljava/lang/annotation/Annotation;>;Ljava/lang/annotation/Annotation;>;")]
    pub fn getDeclaredAnnotationMap(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.getDeclaredAnnotationMap:()Ljava/util/Map;")
    }

    #[java_rta_macros::java_method(name = "getAnnotatedSuperclass", descriptor = "()Ljava/lang/reflect/AnnotatedType;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getAnnotatedSuperclass(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.getAnnotatedSuperclass:()Ljava/lang/reflect/AnnotatedType;")
    }

    #[java_rta_macros::java_method(name = "getAnnotatedInterfaces", descriptor = "()[Ljava/lang/reflect/AnnotatedType;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getAnnotatedInterfaces(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getAnnotatedInterfaces:()[Ljava/lang/reflect/AnnotatedType;")
    }

    #[cfg_attr(any(), java_native(name = "getNestHost0", descriptor = "()Ljava/lang/Class;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;"))]
    pub fn getNestHost0(&self) -> Result<Object> {
        panic!("native: java/lang/Class.getNestHost0:()Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "getNestHost", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;")]
    pub fn getNestHost(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.getNestHost:()Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "isNestmateOf", descriptor = "(Ljava/lang/Class;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z")]
    pub fn isNestmateOf(&self, c: Class<Object>) -> Result<bool> {
        panic!("stub: java/lang/Class.isNestmateOf:(Ljava/lang/Class;)Z")
    }

    #[cfg_attr(any(), java_native(name = "getNestMembers0", descriptor = "()[Ljava/lang/Class;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;"))]
    pub fn getNestMembers0(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("native: java/lang/Class.getNestMembers0:()[Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "getNestMembers", descriptor = "()[Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
    pub fn getNestMembers(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getNestMembers:()[Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "descriptorString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn descriptorString(&self) -> Result<String> {
        panic!("stub: java/lang/Class.descriptorString:()Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "componentType", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;")]
    pub fn componentType(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.componentType:()Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "arrayType", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;")]
    pub fn arrayType(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.arrayType:()Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "describeConstable", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Optional<Ljava/lang/constant/ClassDesc;>;")]
    pub fn describeConstable(&self) -> Result<Object> {
        panic!("stub: java/lang/Class.describeConstable:()Ljava/util/Optional;")
    }

    #[cfg_attr(any(), java_native(name = "isHidden", descriptor = "()Z", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn isHidden(&self) -> Result<bool> {
        panic!("native: java/lang/Class.isHidden:()Z")
    }

    #[java_rta_macros::java_method(name = "getPermittedSubclasses", descriptor = "()[Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
    pub fn getPermittedSubclasses(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/lang/Class.getPermittedSubclasses:()[Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "isDirectSubType", descriptor = "(Ljava/lang/Class;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z")]
    pub fn isDirectSubType(&self, c: Class<Object>) -> Result<bool> {
        panic!("stub: java/lang/Class.isDirectSubType:(Ljava/lang/Class;)Z")
    }

    #[java_rta_macros::java_method(name = "isSealed", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn isSealed(&self) -> Result<bool> {
        panic!("stub: java/lang/Class.isSealed:()Z")
    }

    #[cfg_attr(any(), java_native(name = "getPermittedSubclasses0", descriptor = "()[Ljava/lang/Class;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;"))]
    pub fn getPermittedSubclasses0(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("native: java/lang/Class.getPermittedSubclasses0:()[Ljava/lang/Class;")
    }

    #[java_rta_macros::java_method(name = "getClassFileVersion", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getClassFileVersion(&self) -> Result<i32> {
        panic!("stub: java/lang/Class.getClassFileVersion:()I")
    }

    #[cfg_attr(any(), java_native(name = "getClassFileVersion0", descriptor = "()I", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getClassFileVersion0(&self) -> Result<i32> {
        panic!("native: java/lang/Class.getClassFileVersion0:()I")
    }

    #[java_rta_macros::java_method(name = "getClassAccessFlagsRaw", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getClassAccessFlagsRaw(&self) -> Result<i32> {
        panic!("stub: java/lang/Class.getClassAccessFlagsRaw:()I")
    }

    #[cfg_attr(any(), java_native(name = "getClassAccessFlagsRaw0", descriptor = "()I", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false))]
    pub fn getClassAccessFlagsRaw0(&self) -> Result<i32> {
        panic!("native: java/lang/Class.getClassAccessFlagsRaw0:()I")
    }
}
