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
use crate::jdk::internal::misc::PreviewFeatures;
use crate::jdk::internal::misc::Unsafe;
use crate::jdk::internal::reflect::Reflection;
use crate::jdk::internal::reflect::ReflectionFactory;
use crate::jdk::internal::reflect::ReflectionFactory_GetReflectionFactoryAction;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/Class"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Serializable,java/lang/reflect/GenericDeclaration,java/lang/reflect/Type,java/lang/reflect/AnnotatedElement,java/lang/invoke/TypeDescriptor$OfField,java/lang/constant/Constable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = "<T:Ljava/lang/Object;>Ljava/lang/Object;Ljava/io/Serializable;Ljava/lang/reflect/GenericDeclaration;Ljava/lang/reflect/Type;Ljava/lang/reflect/AnnotatedElement;Ljava/lang/invoke/TypeDescriptor$OfField<Ljava/lang/Class<*>;>;Ljava/lang/constant/Constable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Class.java"]
    #[inner_classes     = "java/lang/Class$1:::0;java/lang/Class$ReflectionData:java/lang/Class:ReflectionData:10;java/lang/reflect/AccessFlag$Location:java/lang/reflect/AccessFlag:Location:16409;java/lang/Class$EnclosingMethodInfo:java/lang/Class:EnclosingMethodInfo:26;java/lang/Class$2:::0;java/lang/Class$Holder:java/lang/Class:Holder:10;java/lang/Class$Atomic:java/lang/Class:Atomic:10;java/lang/PublicMethods$MethodList:java/lang/PublicMethods:MethodList:24;jdk/internal/reflect/ReflectionFactory$GetReflectionFactoryAction:jdk/internal/reflect/ReflectionFactory:GetReflectionFactoryAction:25;java/lang/Class$3:::0;java/lang/Class$AnnotationData:java/lang/Class:AnnotationData:10;java/util/Map$Entry:java/util/Map:Entry:1545;java/lang/invoke/TypeDescriptor$OfField:java/lang/invoke/TypeDescriptor:OfField:1545;java/lang/ClassValue$ClassValueMap:java/lang/ClassValue:ClassValueMap:8;jdk/internal/javac/PreviewFeature$Feature:jdk/internal/javac/PreviewFeature:Feature:16409;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Class;java/lang/Object;java/lang/constant/Constable;java/lang/invoke/TypeDescriptor$OfField;java/lang/reflect/AnnotatedElement;java/lang/reflect/GenericDeclaration;java/lang/reflect/Type"]
    #[has_to_string_method = true]

    pub struct Class<T: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "cachedConstructor", descriptor = "Ljava/lang/reflect/Constructor;", access = "private", modifiers = "volatile transient", is_static = false, generic_signature = "Ljava/lang/reflect/Constructor<TT;>;"))]
        pub cachedConstructor: Object,
        #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private", modifiers = "transient", is_static = false))]
        pub name: String,
        #[cfg_attr(any(), java_field(name = "module", descriptor = "Ljava/lang/Module;", access = "private", modifiers = "transient", is_static = false))]
        pub module: Object,
        #[cfg_attr(any(), java_field(name = "classLoader", descriptor = "Ljava/lang/ClassLoader;", access = "private", modifiers = "final", is_static = false))]
        pub classLoader: ClassLoader,
        #[cfg_attr(any(), java_field(name = "classData", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "transient", is_static = false))]
        pub classData: Object,
        #[cfg_attr(any(), java_field(name = "packageName", descriptor = "Ljava/lang/String;", access = "private", modifiers = "transient", is_static = false))]
        pub packageName: String,
        #[cfg_attr(any(), java_field(name = "componentType", descriptor = "Ljava/lang/Class;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/lang/Class<*>;"))]
        pub componentType: Class<Object>,
        #[cfg_attr(any(), java_field(name = "reflectionData", descriptor = "Ljava/lang/ref/SoftReference;", access = "private", modifiers = "volatile transient", is_static = false, generic_signature = "Ljava/lang/ref/SoftReference<Ljava/lang/Class$ReflectionData<TT;>;>;"))]
        pub reflectionData: SoftReference<Class_ReflectionData<T>>,
        #[cfg_attr(any(), java_field(name = "classRedefinedCount", descriptor = "I", access = "private", modifiers = "volatile transient", is_static = false))]
        pub classRedefinedCount: i32,
        #[cfg_attr(any(), java_field(name = "genericInfo", descriptor = "Lsun/reflect/generics/repository/ClassRepository;", access = "private", modifiers = "volatile transient", is_static = false))]
        pub genericInfo: ClassRepository,
        #[cfg_attr(any(), java_field(name = "enumConstants", descriptor = "[Ljava/lang/Object;", access = "private", modifiers = "volatile transient", is_static = false, generic_signature = "[TT;"))]
        pub enumConstants: Rc<RefCell<Vec<T>>>,
        #[cfg_attr(any(), java_field(name = "enumConstantDirectory", descriptor = "Ljava/util/Map;", access = "private", modifiers = "volatile transient", is_static = false, generic_signature = "Ljava/util/Map<Ljava/lang/String;TT;>;"))]
        pub enumConstantDirectory: Object,
        #[cfg_attr(any(), java_field(name = "annotationData", descriptor = "Ljava/lang/Class$AnnotationData;", access = "private", modifiers = "volatile transient", is_static = false))]
        pub annotationData: Object,
        #[cfg_attr(any(), java_field(name = "annotationType", descriptor = "Lsun/reflect/annotation/AnnotationType;", access = "private", modifiers = "volatile transient", is_static = false))]
        pub annotationType: Object,
        #[cfg_attr(any(), java_field(name = "classValueMap", descriptor = "Ljava/lang/ClassValue$ClassValueMap;", access = "package", modifiers = "transient", is_static = false))]
        pub classValueMap: Object,
    }

    impl<T> Class<T> {
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
        pub fn reflectionFactory() -> ReflectionFactory {
            panic!("stub: java/lang/Class.reflectionFactory:Ljdk/internal/reflect/ReflectionFactory;")
        }

        #[native]
        #[java_native(name = "registerNatives", descriptor = "()V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn registerNatives() -> Result<()> {
            panic!("native: java/lang/Class.registerNatives:()V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/Class;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ClassLoader;Ljava/lang/Class<*>;)V")]
        pub fn new(loader: ClassLoader, arrayComponentType: Class<Object>) -> Result<Self> {
            panic!("stub: java/lang/Class.<init>:(Ljava/lang/ClassLoader;Ljava/lang/Class;)V")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "toGenericString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toGenericString(&self) -> Result<String> {
            panic!("stub: java/lang/Class.toGenericString:()Ljava/lang/String;")
        }

        #[java_method(name = "typeVarBounds", descriptor = "(Ljava/lang/reflect/TypeVariable;)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/reflect/TypeVariable<*>;)Ljava/lang/String;")]
        pub fn typeVarBounds(typeVar: Object) -> Result<String> {
            panic!("stub: java/lang/Class.typeVarBounds:(Ljava/lang/reflect/TypeVariable;)Ljava/lang/String;")
        }

        #[java_method(name = "forName", descriptor = "(Ljava/lang/String;)Ljava/lang/Class;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/ClassNotFoundException", generic_signature = "(Ljava/lang/String;)Ljava/lang/Class<*>;")]
        pub fn forName_str(className: String) -> Result<Object> {
            panic!("stub: java/lang/Class.forName:(Ljava/lang/String;)Ljava/lang/Class;")
        }

        #[java_method(name = "forName", descriptor = "(Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Class;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/ClassNotFoundException", generic_signature = "(Ljava/lang/String;Ljava/lang/Class<*>;)Ljava/lang/Class<*>;")]
        pub fn forName_str_class(className: String, caller: Class<Object>) -> Result<Object> {
            panic!("stub: java/lang/Class.forName:(Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Class;")
        }

        #[java_method(name = "forName", descriptor = "(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/ClassNotFoundException", generic_signature = "(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class<*>;")]
        pub fn forName_str_z_classl(name: String, initialize: bool, loader: ClassLoader) -> Result<Object> {
            panic!("stub: java/lang/Class.forName:(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;")
        }

        #[java_method(name = "forName", descriptor = "(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/ClassNotFoundException", generic_signature = "(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class<*>;)Ljava/lang/Class<*>;")]
        pub fn forName_str_z_classl_class(name: String, initialize: bool, loader: ClassLoader, caller: Class<Object>) -> Result<Object> {
            panic!("stub: java/lang/Class.forName:(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;")
        }

        #[native]
        #[java_native(name = "forName0", descriptor = "(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/ClassNotFoundException", generic_signature = "(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class<*>;)Ljava/lang/Class<*>;")]
        pub fn forName0(arg0: String, arg1: bool, arg2: ClassLoader, arg3: Class<Object>) -> Result<Object> {
            panic!("native: java/lang/Class.forName0:(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;")
        }

        #[java_method(name = "forName", descriptor = "(Ljava/lang/Module;Ljava/lang/String;)Ljava/lang/Class;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Module;Ljava/lang/String;)Ljava/lang/Class<*>;")]
        pub fn forName_module_str(module: Object, name: String) -> Result<Object> {
            panic!("stub: java/lang/Class.forName:(Ljava/lang/Module;Ljava/lang/String;)Ljava/lang/Class;")
        }

        #[java_method(name = "forName", descriptor = "(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Class;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Class<*>;)Ljava/lang/Class<*>;")]
        pub fn forName_module_str_class(module: Object, name: String, caller: Class<Object>) -> Result<Object> {
            panic!("stub: java/lang/Class.forName:(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Class;")
        }

        #[java_method(name = "newInstance", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/InstantiationException,java/lang/IllegalAccessException", generic_signature = "()TT;", is_deprecated = true)]
        pub fn newInstance(&self) -> Result<Object> {
            panic!("stub: java/lang/Class.newInstance:()Ljava/lang/Object;")
        }

        #[native]
        #[java_native(name = "isInstance", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn isInstance(&self, arg0: Object) -> Result<bool> {
            panic!("native: java/lang/Class.isInstance:(Ljava/lang/Object;)Z")
        }

        #[native]
        #[java_native(name = "isAssignableFrom", descriptor = "(Ljava/lang/Class;)Z", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z")]
        pub fn isAssignableFrom(&self, arg0: Class<Object>) -> Result<bool> {
            panic!("native: java/lang/Class.isAssignableFrom:(Ljava/lang/Class;)Z")
        }

        #[native]
        #[java_native(name = "isInterface", descriptor = "()Z", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn isInterface(&self) -> Result<bool> {
            panic!("native: java/lang/Class.isInterface:()Z")
        }

        #[native]
        #[java_native(name = "isArray", descriptor = "()Z", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn isArray(&self) -> Result<bool> {
            panic!("native: java/lang/Class.isArray:()Z")
        }

        #[native]
        #[java_native(name = "isPrimitive", descriptor = "()Z", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn isPrimitive(&self) -> Result<bool> {
            panic!("native: java/lang/Class.isPrimitive:()Z")
        }

        #[java_method(name = "isAnnotation", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isAnnotation(&self) -> Result<bool> {
            panic!("stub: java/lang/Class.isAnnotation:()Z")
        }

        #[java_method(name = "isSynthetic", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSynthetic(&self) -> Result<bool> {
            let this = self;
            let _t0 = this.getModifiers()?;
            Ok(((_t0&4096i32)!=0))
        }

        #[java_method(name = "getName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getName(&self) -> Result<String> {
            let this = self;
            let mut name = this.__get_name();
            let mut _merged1: String;
            if !_is_jnull(&name) {
                _merged1 = name;
            } else {
                let _t0 = this.initClassName()?;
                _merged1 = _t0;
            }
            Ok(_merged1)
        }

        #[native]
        #[java_native(name = "initClassName", descriptor = "()Ljava/lang/String;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn initClassName(&self) -> Result<String> {
            panic!("native: java/lang/Class.initClassName:()Ljava/lang/String;")
        }

        #[java_method(name = "getClassLoader", descriptor = "()Ljava/lang/ClassLoader;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getClassLoader(&self) -> Result<ClassLoader> {
            let this = self;
            let mut cl = this.__get_classLoader();
            if _is_jnull(&cl) {
                return Ok(Default::default());
            }
            let _t0: SecurityManager = System::getSecurityManager()?;
            let mut sm: SecurityManager = _t0;
            if !_is_jnull(&sm) {
                let _t1: Object = Reflection::getCallerClass()?;
                ClassLoader::checkClassLoaderPermission(Clone::clone(&cl), Clone::clone(&_t1))?;
            }
            Ok(cl)
        }

        #[java_method(name = "getClassLoader0", descriptor = "()Ljava/lang/ClassLoader;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getClassLoader0(&self) -> Result<ClassLoader> {
            let this = self;
            Ok(this.__get_classLoader())
        }

        #[java_method(name = "getModule", descriptor = "()Ljava/lang/Module;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getModule(&self) -> Result<Object> {
            panic!("stub: java/lang/Class.getModule:()Ljava/lang/Module;")
        }

        #[java_method(name = "getClassData", descriptor = "()Ljava/lang/Object;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getClassData(&self) -> Result<Object> {
            panic!("stub: java/lang/Class.getClassData:()Ljava/lang/Object;")
        }

        #[java_method(name = "getTypeParameters", descriptor = "()[Ljava/lang/reflect/TypeVariable;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/reflect/TypeVariable<Ljava/lang/Class<TT;>;>;")]
        pub fn getTypeParameters(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.getTypeParameters:()[Ljava/lang/reflect/TypeVariable;")
        }

        #[native]
        #[java_native(name = "getSuperclass", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<-TT;>;")]
        pub fn getSuperclass(&self) -> Result<Object> {
            panic!("native: java/lang/Class.getSuperclass:()Ljava/lang/Class;")
        }

        #[java_method(name = "getGenericSuperclass", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getGenericSuperclass(&self) -> Result<Object> {
            panic!("stub: java/lang/Class.getGenericSuperclass:()Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "getPackage", descriptor = "()Ljava/lang/Package;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPackage(&self) -> Result<Object> {
            panic!("stub: java/lang/Class.getPackage:()Ljava/lang/Package;")
        }

        #[java_method(name = "getPackageName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPackageName(&self) -> Result<String> {
            let this = self;
            let mut pn = this.__get_packageName();
            let _t0 = this.isArray()?;
            let mut _merged2: Object;
            if _t0 {
                let _t1 = this.elementType()?;
                _merged2 = _t1;
            } else {
                _merged2 = Object::from_any(Clone::clone(&Clone::clone(this)));
            }
            let mut c = (_merged2).downcast::<Class<Object>>();
            let _t3 = c.isPrimitive()?;
            if _t3 {
                pn = String::from("java.lang");
            } else {
                let _t4 = c.getName()?;
                let mut cn: String = _t4;
                let _t5 = cn.lastIndexOf_i(46i32)?;
                let mut dot: i32 = _t5;
                let mut _merged8: String;
                if dot != -1i32 {
                    let _t6 = cn.substring_i_i(0i32, dot)?;
                    let _t7 = _t6.intern()?;
                    _merged8 = _t7;
                } else {
                    _merged8 = String::from("");
                }
                pn = _merged8;
            }
            this.__set_packageName(Clone::clone(&pn));
            Ok(pn)
        }

        #[java_method(name = "getInterfaces", descriptor = "()[Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
        // java: getInterfaces()[Ljava/lang/Class;
        pub fn getInterfaces(&self) -> Result<Rc<RefCell<Vec<Class<Object>>>>> {
            let this = self;
            let _t0 = this.getInterfaces_z((1i32 != 0i32))?;
            Ok(Default::default())
        }

        #[java_method(name = "getInterfaces", descriptor = "(Z)[Ljava/lang/Class;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Z)[Ljava/lang/Class<*>;")]
        // java: getInterfaces(Z)[Ljava/lang/Class;
        pub fn getInterfaces_z(&self, mut cloneArray: bool) -> Result<Rc<RefCell<Vec<Class<Object>>>>> {
            let this = self;
            let _t0 = this.reflectionData()?;
            let mut rd: Class_ReflectionData<Object> = _t0;
            if _is_jnull(&rd) {
                let _t1 = this.getInterfaces0()?;
                return Ok(Default::default());
            }
            let mut interfaces = rd.__get_interfaces();
            if _is_jnull(&interfaces) {
                let _t1 = this.getInterfaces0()?;
                interfaces = _t1;
                rd.__set_interfaces(Clone::clone(&interfaces));
            }
            let mut _merged2: Rc<RefCell<Vec<Object>>>;
            if cloneArray {
                let _t1: Object = Object::from_any(interfaces.clone());
                _merged2 = (_t1).downcast::<Rc<RefCell<Vec<Object>>>>();
            } else {
                _merged2 = interfaces;
            }
            Ok(Default::default())
        }

        #[native]
        #[java_native(name = "getInterfaces0", descriptor = "()[Ljava/lang/Class;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
        pub fn getInterfaces0(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("native: java/lang/Class.getInterfaces0:()[Ljava/lang/Class;")
        }

        #[java_method(name = "getGenericInterfaces", descriptor = "()[Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getGenericInterfaces(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            let this = self;
            let _t0 = this.getGenericInfo()?;
            let mut info: ClassRepository = _t0;
            let mut _merged2: Rc<RefCell<Vec<Object>>>;
            if _is_jnull(&info) {
                let _t1 = this.getInterfaces()?;
                _merged2 = _t1;
            } else {
                let _t1 = info.getSuperInterfaces()?;
                _merged2 = _t1;
            }
            Ok(_merged2)
        }

        #[java_method(name = "getComponentType", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;")]
        pub fn getComponentType(&self) -> Result<Class<Object>> {
            let this = self;
            let _t0 = this.isArray()?;
            if _t0 {
                return Ok(this.__get_componentType());
            }
            Ok(Default::default())
        }

        #[java_method(name = "elementType", descriptor = "()Ljava/lang/Class;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;")]
        pub fn elementType(&self) -> Result<Class<Object>> {
            let this = self;
            let _t0 = this.isArray()?;
            if !(_t0) {
                return Ok(Default::default());
            }
            let mut c = this.clone();
            let mut c = Default::default();
            loop {
                let _t1 = c.isArray()?;
                if !(_t1) { break; }
                let _t1 = c.getComponentType()?;
                c = (_t1).downcast::<Class<Object>>();
            }
            Ok(c)
        }

        #[native]
        #[java_native(name = "getModifiers", descriptor = "()I", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getModifiers(&self) -> Result<i32> {
            panic!("native: java/lang/Class.getModifiers:()I")
        }

        #[java_method(name = "accessFlags", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/lang/reflect/AccessFlag;>;")]
        pub fn accessFlags(&self) -> Result<Object> {
            panic!("stub: java/lang/Class.accessFlags:()Ljava/util/Set;")
        }

        #[native]
        #[java_native(name = "getSigners", descriptor = "()[Ljava/lang/Object;", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getSigners(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("native: java/lang/Class.getSigners:()[Ljava/lang/Object;")
        }

        #[native]
        #[java_native(name = "setSigners", descriptor = "([Ljava/lang/Object;)V", access = "package", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn setSigners(&self, arg0: Rc<RefCell<Vec<Object>>>) -> Result<()> {
            panic!("native: java/lang/Class.setSigners:([Ljava/lang/Object;)V")
        }

        #[java_method(name = "getEnclosingMethod", descriptor = "()Ljava/lang/reflect/Method;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException")]
        pub fn getEnclosingMethod(&self) -> Result<Method> {
            panic!("stub: java/lang/Class.getEnclosingMethod:()Ljava/lang/reflect/Method;")
        }

        #[native]
        #[java_native(name = "getEnclosingMethod0", descriptor = "()[Ljava/lang/Object;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getEnclosingMethod0(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("native: java/lang/Class.getEnclosingMethod0:()[Ljava/lang/Object;")
        }

        #[java_method(name = "getEnclosingMethodInfo", descriptor = "()Ljava/lang/Class$EnclosingMethodInfo;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getEnclosingMethodInfo(&self) -> Result<Class_EnclosingMethodInfo> {
            let this = self;
            let _t0 = this.getEnclosingMethod0()?;
            let mut enclosingInfo: Rc<RefCell<Vec<Object>>> = _t0;
            if _is_jnull(&enclosingInfo) {
                return Ok(Default::default());
            }
            Ok(Class_EnclosingMethodInfo::new(Clone::clone(&enclosingInfo))?)
        }

        #[java_method(name = "toClass", descriptor = "(Ljava/lang/reflect/Type;)Ljava/lang/Class;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/reflect/Type;)Ljava/lang/Class<*>;")]
        pub fn toClass(o: Object) -> Result<Object> {
            panic!("stub: java/lang/Class.toClass:(Ljava/lang/reflect/Type;)Ljava/lang/Class;")
        }

        #[java_method(name = "getEnclosingConstructor", descriptor = "()Ljava/lang/reflect/Constructor;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException", generic_signature = "()Ljava/lang/reflect/Constructor<*>;")]
        pub fn getEnclosingConstructor(&self) -> Result<Object> {
            panic!("stub: java/lang/Class.getEnclosingConstructor:()Ljava/lang/reflect/Constructor;")
        }

        #[java_method(name = "getDeclaringClass", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException", generic_signature = "()Ljava/lang/Class<*>;")]
        pub fn getDeclaringClass(&self) -> Result<Object> {
            panic!("stub: java/lang/Class.getDeclaringClass:()Ljava/lang/Class;")
        }

        #[native]
        #[java_native(name = "getDeclaringClass0", descriptor = "()Ljava/lang/Class;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;")]
        pub fn getDeclaringClass0(&self) -> Result<Object> {
            panic!("native: java/lang/Class.getDeclaringClass0:()Ljava/lang/Class;")
        }

        #[java_method(name = "getEnclosingClass", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException", generic_signature = "()Ljava/lang/Class<*>;")]
        pub fn getEnclosingClass(&self) -> Result<Class<Object>> {
            let this = self;
            let _t0 = this.getEnclosingMethodInfo()?;
            let mut enclosingInfo: Class_EnclosingMethodInfo = _t0;
        let mut enclosingCandidate = Default::default();
            if _is_jnull(&enclosingInfo) {
                let _t1 = this.getDeclaringClass0()?;
                enclosingCandidate = (_t1).downcast::<Class<Object>>();
            } else {
                let _t1 = enclosingInfo.getEnclosingClass()?;
                let mut enclosingClass = (_t1).downcast::<Class<Object>>();
                if _is_jnull(&enclosingClass) {
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                enclosingCandidate = enclosingClass;
            }
            let _t1: SecurityManager = System::getSecurityManager()?;
            let mut enclosingClass: SecurityManager = _t1;
            if !_is_jnull(&enclosingClass) {
                let _t2: Object = Reflection::getCallerClass()?;
                let _t3: ClassLoader = ClassLoader::getClassLoader(Clone::clone(&_t2))?;
                enclosingCandidate.checkPackageAccess(Clone::clone(&enclosingClass), Clone::clone(&_t3), (1i32 != 0i32))?;
            }
            Ok(enclosingCandidate)
        }

        #[java_method(name = "getSimpleName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSimpleName(&self) -> Result<String> {
            let this = self;
            let _t0 = this.isUnnamedClass()?;
            if _t0 {
                return Ok(String::from(""));
            }
            let _t1 = this.reflectionData()?;
            let mut rd: Class_ReflectionData<Object> = _t1;
            let mut simpleName = rd.__get_simpleName();
            if _is_jnull(&simpleName) {
                let _t2 = this.getSimpleName0()?;
                simpleName = _t2;
                rd.__set_simpleName(Clone::clone(&simpleName));
            }
            Ok(simpleName)
        }

        #[java_method(name = "getSimpleName0", descriptor = "()Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSimpleName0(&self) -> Result<String> {
            let this = self;
            let _t0 = this.isArray()?;
            if _t0 {
                let _t1 = this.getComponentType()?;
                let _vdispatch2: String = if let Some(__f) = _t1.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
                let _t3 = _vdispatch2.concat(Clone::clone(&String::from("[]")))?;
                return Ok(_t3);
            }
            let _t1 = this.getSimpleBinaryName()?;
            let mut simpleName: String = _t1;
            if _is_jnull(&simpleName) {
                let _t2 = this.getName()?;
                simpleName = _t2;
                let _t3 = simpleName.lastIndexOf_i(46i32)?;
                let _t4 = simpleName.substring_i((_t3).wrapping_add(1i32))?;
                simpleName = _t4;
            }
            Ok(simpleName)
        }

        #[java_method(name = "getTypeName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTypeName(&self) -> Result<String> {
            panic!("stub: java/lang/Class.getTypeName:()Ljava/lang/String;")
        }

        #[java_method(name = "getCanonicalName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCanonicalName(&self) -> Result<String> {
            let this = self;
            let _t0 = this.isUnnamedClass()?;
            if _t0 {
                return Ok(Default::default());
            }
            let _t1 = this.reflectionData()?;
            let mut rd: Class_ReflectionData<Object> = _t1;
            let mut canonicalName = rd.__get_canonicalName();
            if _is_jnull(&canonicalName) {
                let _t2 = this.getCanonicalName0()?;
                canonicalName = _t2;
                rd.__set_canonicalName(Clone::clone(&canonicalName));
            }
            Ok(Default::default())
        }

        #[java_method(name = "getCanonicalName0", descriptor = "()Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCanonicalName0(&self) -> Result<String> {
            let this = self;
            let _t0 = this.isArray()?;
            let _t1 = this.getComponentType()?;
            let _vdispatch2: String = if let Some(__f) = _t1.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
            let mut canonicalName: String = _vdispatch2;
            if !_is_jnull(&canonicalName) {
                let _t3 = canonicalName.concat(Clone::clone(&String::from("[]")))?;
                return Ok(_t3);
            }
            return Ok(Class_ReflectionData::<Object>::NULL_SENTINEL());
            let _t3 = this.isHidden()?;
            let _t4 = this.isLocalOrAnonymousClass()?;
            if _t4 {
                return Ok(Class_ReflectionData::<Object>::NULL_SENTINEL());
            }
            let _t5 = this.getEnclosingClass()?;
            let mut canonicalName: Object = _t5;
            if _is_jnull(&canonicalName) {
                let _t6 = this.getName()?;
                return Ok(_t6);
            }
            let _vdispatch6: String = if let Some(__f) = canonicalName.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
            let mut enclosingName: String = _vdispatch6;
            if _is_jnull(&enclosingName) {
                return Ok(Class_ReflectionData::<Object>::NULL_SENTINEL());
            }
            let _t7 = this.getSimpleName()?;
            let mut simpleName: String = _t7;
            let _t8 = enclosingName.length()?;
            let _t9 = simpleName.length()?;
            let _t10 = StringBuilder::new_i(((_t8).wrapping_add(_t9)).wrapping_add(1i32))?.append_str(Clone::clone(&enclosingName))?;
            let _t11 = _t10.append_c(((46i32) as u16))?;
            let _t12 = _t11.append_str(Clone::clone(&simpleName))?;
            let _t13 = _t12.toString()?;
            Ok(_t13)
        }

        #[java_method(name = "isUnnamedClass", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUnnamedClass(&self) -> Result<bool> {
            let this = self;
            let _t0: bool = PreviewFeatures::isEnabled()?;
            let mut _merged7: bool;
            if _t0 {
                let _t1 = this.isSynthetic()?;
                let mut _merged6: bool;
                if _t1 {
                    let _t2 = this.isTopLevelClass()?;
                    let mut _merged5: bool;
                    if _t2 {
                        let _t3 = this.getModifiers()?;
                        let _t4: bool = Modifier::isFinal(_t3)?;
                        _merged5 = !(!(_t4));
                    } else {
                        _merged5 = (0i32 != 0);
                    }
                    _merged6 = _merged5;
                } else {
                    _merged6 = (0i32 != 0);
                }
                _merged7 = _merged6;
            } else {
                _merged7 = (0i32 != 0);
            }
            Ok(_merged7)
        }

        #[java_method(name = "isAnonymousClass", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isAnonymousClass(&self) -> Result<bool> {
            panic!("stub: java/lang/Class.isAnonymousClass:()Z")
        }

        #[java_method(name = "isLocalClass", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLocalClass(&self) -> Result<bool> {
            panic!("stub: java/lang/Class.isLocalClass:()Z")
        }

        #[java_method(name = "isMemberClass", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isMemberClass(&self) -> Result<bool> {
            panic!("stub: java/lang/Class.isMemberClass:()Z")
        }

        #[java_method(name = "getSimpleBinaryName", descriptor = "()Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSimpleBinaryName(&self) -> Result<String> {
            let this = self;
            let _t0 = this.isTopLevelClass()?;
            if _t0 {
                return Ok(Default::default());
            }
            let _t1 = this.getSimpleBinaryName0()?;
            let mut name: String = _t1;
            if _is_jnull(&name) {
                return Ok(String::from(""));
            }
            Ok(name)
        }

        #[native]
        #[java_native(name = "getSimpleBinaryName0", descriptor = "()Ljava/lang/String;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getSimpleBinaryName0(&self) -> Result<String> {
            panic!("native: java/lang/Class.getSimpleBinaryName0:()Ljava/lang/String;")
        }

        #[java_method(name = "isTopLevelClass", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isTopLevelClass(&self) -> Result<bool> {
            let this = self;
            let _t0 = this.isLocalOrAnonymousClass()?;
            let mut _merged2: bool;
            if !(_t0) {
                let _t1 = this.getDeclaringClass0()?;
                _merged2 = _is_jnull(&_t1);
            } else {
                _merged2 = (0i32 != 0);
            }
            Ok(_merged2)
        }

        #[java_method(name = "isLocalOrAnonymousClass", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLocalOrAnonymousClass(&self) -> Result<bool> {
            let this = self;
            let _t0 = this.hasEnclosingMethodInfo()?;
            Ok(_t0)
        }

        #[java_method(name = "hasEnclosingMethodInfo", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasEnclosingMethodInfo(&self) -> Result<bool> {
            let this = self;
            let _t0 = this.getEnclosingMethod0()?;
            let mut enclosingInfo: Rc<RefCell<Vec<Object>>> = _t0;
            if !_is_jnull(&enclosingInfo) {
                Class_EnclosingMethodInfo::validate(Clone::clone(&enclosingInfo))?;
                return Ok((1i32 != 0i32));
            }
            Ok((0i32 != 0i32))
        }

        #[java_method(name = "getClasses", descriptor = "()[Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
        pub fn getClasses(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.getClasses:()[Ljava/lang/Class;")
        }

        #[java_method(name = "getFields", descriptor = "()[Ljava/lang/reflect/Field;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException")]
        pub fn getFields(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.getFields:()[Ljava/lang/reflect/Field;")
        }

        #[java_method(name = "getMethods", descriptor = "()[Ljava/lang/reflect/Method;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException")]
        pub fn getMethods(&self) -> Result<Rc<RefCell<Vec<Method>>>> {
            panic!("stub: java/lang/Class.getMethods:()[Ljava/lang/reflect/Method;")
        }

        #[java_method(name = "getConstructors", descriptor = "()[Ljava/lang/reflect/Constructor;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException", generic_signature = "()[Ljava/lang/reflect/Constructor<*>;")]
        pub fn getConstructors(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.getConstructors:()[Ljava/lang/reflect/Constructor;")
        }

        #[java_method(name = "getField", descriptor = "(Ljava/lang/String;)Ljava/lang/reflect/Field;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NoSuchFieldException,java/lang/SecurityException")]
        pub fn getField(&self, name: String) -> Result<Object> {
            panic!("stub: java/lang/Class.getField:(Ljava/lang/String;)Ljava/lang/reflect/Field;")
        }

        #[java_method(name = "getMethod", descriptor = "(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NoSuchMethodException,java/lang/SecurityException", generic_signature = "(Ljava/lang/String;[Ljava/lang/Class<*>;)Ljava/lang/reflect/Method;")]
        pub fn getMethod(&self, mut name: String, mut parameterTypes: Rc<RefCell<Vec<Class<Object>>>>) -> Result<Method> {
            let this = self;
            let _t0: Object = Objects::requireNonNull_obj(Object::from_any(name.clone()))?;
            let _t1: SecurityManager = System::getSecurityManager()?;
            let mut sm: SecurityManager = _t1;
            if !_is_jnull(&sm) {
                let _t2: Object = Reflection::getCallerClass()?;
                this.checkMemberAccess(Clone::clone(&sm), 0i32, Clone::clone(&_t2), (1i32 != 0i32))?;
            }
            let _t2 = this.getMethod0(Clone::clone(&name), Clone::clone(&parameterTypes))?;
            let mut method: Method = _t2;
            if _is_jnull(&method) {
                let _t3 = this.methodToString(Clone::clone(&name), Clone::clone(&parameterTypes))?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t3: ReflectionFactory = Class::<Object>::getReflectionFactory()?;
            let _t4 = _t3.copyMethod(Clone::clone(&method))?;
            Ok(_t4)
        }

        #[java_method(name = "getConstructor", descriptor = "([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NoSuchMethodException,java/lang/SecurityException", generic_signature = "([Ljava/lang/Class<*>;)Ljava/lang/reflect/Constructor<TT;>;")]
        pub fn getConstructor(&self, parameterTypes: Rc<RefCell<Vec<Class<Object>>>>) -> Result<Object> {
            panic!("stub: java/lang/Class.getConstructor:([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;")
        }

        #[java_method(name = "getDeclaredClasses", descriptor = "()[Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException", generic_signature = "()[Ljava/lang/Class<*>;")]
        pub fn getDeclaredClasses(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.getDeclaredClasses:()[Ljava/lang/Class;")
        }

        #[java_method(name = "getDeclaredFields", descriptor = "()[Ljava/lang/reflect/Field;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException")]
        pub fn getDeclaredFields(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.getDeclaredFields:()[Ljava/lang/reflect/Field;")
        }

        #[java_method(name = "getRecordComponents", descriptor = "()[Ljava/lang/reflect/RecordComponent;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getRecordComponents(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.getRecordComponents:()[Ljava/lang/reflect/RecordComponent;")
        }

        #[java_method(name = "getDeclaredMethods", descriptor = "()[Ljava/lang/reflect/Method;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException")]
        pub fn getDeclaredMethods(&self) -> Result<Rc<RefCell<Vec<Method>>>> {
            panic!("stub: java/lang/Class.getDeclaredMethods:()[Ljava/lang/reflect/Method;")
        }

        #[java_method(name = "getDeclaredConstructors", descriptor = "()[Ljava/lang/reflect/Constructor;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/SecurityException", generic_signature = "()[Ljava/lang/reflect/Constructor<*>;")]
        pub fn getDeclaredConstructors(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.getDeclaredConstructors:()[Ljava/lang/reflect/Constructor;")
        }

        #[java_method(name = "getDeclaredField", descriptor = "(Ljava/lang/String;)Ljava/lang/reflect/Field;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NoSuchFieldException,java/lang/SecurityException")]
        pub fn getDeclaredField(&self, name: String) -> Result<Object> {
            panic!("stub: java/lang/Class.getDeclaredField:(Ljava/lang/String;)Ljava/lang/reflect/Field;")
        }

        #[java_method(name = "getDeclaredMethod", descriptor = "(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NoSuchMethodException,java/lang/SecurityException", generic_signature = "(Ljava/lang/String;[Ljava/lang/Class<*>;)Ljava/lang/reflect/Method;")]
        pub fn getDeclaredMethod(&self, name: String, parameterTypes: Rc<RefCell<Vec<Class<Object>>>>) -> Result<Method> {
            panic!("stub: java/lang/Class.getDeclaredMethod:(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;")
        }

        #[java_method(name = "getDeclaredPublicMethods", descriptor = "(Ljava/lang/String;[Ljava/lang/Class;)Ljava/util/List;", access = "package", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;[Ljava/lang/Class<*>;)Ljava/util/List<Ljava/lang/reflect/Method;>;")]
        pub fn getDeclaredPublicMethods(&self, name: String, parameterTypes: Rc<RefCell<Vec<Class<Object>>>>) -> Result<Object> {
            panic!("stub: java/lang/Class.getDeclaredPublicMethods:(Ljava/lang/String;[Ljava/lang/Class;)Ljava/util/List;")
        }

        #[java_method(name = "getDeclaredConstructor", descriptor = "([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NoSuchMethodException,java/lang/SecurityException", generic_signature = "([Ljava/lang/Class<*>;)Ljava/lang/reflect/Constructor<TT;>;")]
        pub fn getDeclaredConstructor(&self, parameterTypes: Rc<RefCell<Vec<Class<Object>>>>) -> Result<Object> {
            panic!("stub: java/lang/Class.getDeclaredConstructor:([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;")
        }

        #[java_method(name = "getResourceAsStream", descriptor = "(Ljava/lang/String;)Ljava/io/InputStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getResourceAsStream(&self, name: String) -> Result<InputStream> {
            panic!("stub: java/lang/Class.getResourceAsStream:(Ljava/lang/String;)Ljava/io/InputStream;")
        }

        #[java_method(name = "getResource", descriptor = "(Ljava/lang/String;)Ljava/net/URL;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getResource(&self, name: String) -> Result<Object> {
            panic!("stub: java/lang/Class.getResource:(Ljava/lang/String;)Ljava/net/URL;")
        }

        #[java_method(name = "isOpenToCaller", descriptor = "(Ljava/lang/String;Ljava/lang/Class;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;Ljava/lang/Class<*>;)Z")]
        pub fn isOpenToCaller(&self, name: String, caller: Class<Object>) -> Result<bool> {
            panic!("stub: java/lang/Class.isOpenToCaller:(Ljava/lang/String;Ljava/lang/Class;)Z")
        }

        #[java_method(name = "getProtectionDomain", descriptor = "()Ljava/security/ProtectionDomain;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getProtectionDomain(&self) -> Result<Object> {
            panic!("stub: java/lang/Class.getProtectionDomain:()Ljava/security/ProtectionDomain;")
        }

        #[java_method(name = "protectionDomain", descriptor = "()Ljava/security/ProtectionDomain;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn protectionDomain(&self) -> Result<Object> {
            panic!("stub: java/lang/Class.protectionDomain:()Ljava/security/ProtectionDomain;")
        }

        #[native]
        #[java_native(name = "getProtectionDomain0", descriptor = "()Ljava/security/ProtectionDomain;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getProtectionDomain0(&self) -> Result<Object> {
            panic!("native: java/lang/Class.getProtectionDomain0:()Ljava/security/ProtectionDomain;")
        }

        #[native]
        #[java_native(name = "getPrimitiveClass", descriptor = "(Ljava/lang/String;)Ljava/lang/Class;", access = "package", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;)Ljava/lang/Class<*>;")]
        pub fn getPrimitiveClass(arg0: String) -> Result<Object> {
            panic!("native: java/lang/Class.getPrimitiveClass:(Ljava/lang/String;)Ljava/lang/Class;")
        }

        #[java_method(name = "checkMemberAccess", descriptor = "(Ljava/lang/SecurityManager;ILjava/lang/Class;Z)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/SecurityManager;ILjava/lang/Class<*>;Z)V")]
        pub fn checkMemberAccess(&self, mut sm: SecurityManager, mut which: i32, mut caller: Class<Object>, mut checkProxyInterfaces: bool) -> Result<()> {
            let this = self;
            let _t0: ClassLoader = ClassLoader::getClassLoader(Clone::clone(&caller))?;
            let mut ccl: ClassLoader = _t0;
            let mut cl = this.__get_classLoader();
            if Object::from_any(ccl.clone()) != Object::from_any(cl.clone()) {
                sm.checkPermission_permis(Clone::clone(&SecurityConstants::CHECK_MEMBER_ACCESS_PERMISSION()).into())?;
            }
            this.checkPackageAccess(Clone::clone(&sm), Clone::clone(&ccl), checkProxyInterfaces)?;
            Ok(())
        }

        #[java_method(name = "checkPackageAccess", descriptor = "(Ljava/lang/SecurityManager;Ljava/lang/ClassLoader;Z)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkPackageAccess(&self, mut sm: SecurityManager, mut ccl: ClassLoader, mut checkProxyInterfaces: bool) -> Result<()> {
            let this = self;
            let mut cl = this.__get_classLoader();
            let _t0: bool = ReflectUtil::needsPackageAccessCheck(Clone::clone(&ccl), Clone::clone(&cl))?;
            let _t1 = this.getPackageName()?;
            let mut pkg: String = _t1;
            let _t2 = pkg.isEmpty()?;
            let _t3: bool = Proxy::isProxyClass(Clone::clone(this))?;
            let _t4: bool = ReflectUtil::isNonPublicProxyClass(Clone::clone(this))?;
            if _t4 {
                sm.checkPackageAccess(Clone::clone(&pkg))?;
            }
            let _t5: bool = Proxy::isProxyClass(Clone::clone(this))?;
            if _t5 {
                let _t6 = this.getInterfaces_z((0i32 != 0i32))?;
                ReflectUtil::checkProxyPackageAccess_classl_arr_cla(Clone::clone(&ccl), Default::default())?;
            }
            Ok(())
        }

        #[java_method(name = "checkPackageAccessForPermittedSubclasses", descriptor = "(Ljava/lang/SecurityManager;Ljava/lang/ClassLoader;[Ljava/lang/Class;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/SecurityManager;Ljava/lang/ClassLoader;[Ljava/lang/Class<*>;)V")]
        pub fn checkPackageAccessForPermittedSubclasses(sm: SecurityManager, ccl: ClassLoader, subClasses: Rc<RefCell<Vec<Class<Object>>>>) -> Result<()> {
            panic!("stub: java/lang/Class.checkPackageAccessForPermittedSubclasses:(Ljava/lang/SecurityManager;Ljava/lang/ClassLoader;[Ljava/lang/Class;)V")
        }

        #[java_method(name = "resolveName", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn resolveName(&self, name: String) -> Result<String> {
            panic!("stub: java/lang/Class.resolveName:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "reflectionData", descriptor = "()Ljava/lang/Class$ReflectionData;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class$ReflectionData<TT;>;")]
        pub fn reflectionData(&self) -> Result<Class_ReflectionData<T>> {
            let this = self;
            let mut reflectionData = this.__get_reflectionData();
            let mut classRedefinedCount = this.__get_classRedefinedCount();
            let _t0 = reflectionData.get()?;
            let mut rd = (_t0).downcast::<Class_ReflectionData<Object>>();
            if rd.__get_redefinedCount() == classRedefinedCount {
                return Ok(Default::default());
            }
            let _t1 = this.newReflectionData(Clone::clone(&reflectionData), classRedefinedCount)?;
            Ok(Default::default())
        }

        #[java_method(name = "newReflectionData", descriptor = "(Ljava/lang/ref/SoftReference;I)Ljava/lang/Class$ReflectionData;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/ref/SoftReference<Ljava/lang/Class$ReflectionData<TT;>;>;I)Ljava/lang/Class$ReflectionData<TT;>;")]
        pub fn newReflectionData(&self, mut oldReflectionData: SoftReference<Class_ReflectionData<T>>, mut classRedefinedCount: i32) -> Result<Class_ReflectionData<T>> {
            let this = self;
            loop {
                let mut rd = Class_ReflectionData::<Object>::new(classRedefinedCount)?;
                let _t0: bool = Class_Atomic::casReflectionData(Clone::clone(this), Clone::clone(&oldReflectionData), Clone::clone(&SoftReference::<Object>::new_obj(Clone::clone(&rd))?))?;
                if _t0 {
                    return Ok(Default::default());
                }
                let mut oldReflectionData = this.__get_reflectionData();
                classRedefinedCount = this.__get_classRedefinedCount();
                let _t1 = oldReflectionData.get()?;
                rd = (_t1).downcast::<Class_ReflectionData<Object>>();
                if rd.__get_redefinedCount() == classRedefinedCount {
                    return Ok(Default::default());
                }
            }
        }

        #[native]
        #[java_native(name = "getGenericSignature0", descriptor = "()Ljava/lang/String;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getGenericSignature0(&self) -> Result<String> {
            panic!("native: java/lang/Class.getGenericSignature0:()Ljava/lang/String;")
        }

        #[java_method(name = "getFactory", descriptor = "()Lsun/reflect/generics/factory/GenericsFactory;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFactory(&self) -> Result<Object> {
            let this = self;
            let _t0: ClassScope = ClassScope::make(Clone::clone(this))?;
            let _t1: CoreReflectionFactory = CoreReflectionFactory::make(Object::from_any(Clone::clone(self)), Object::from_any(_t0.clone()))?;
            Ok(Object::from_any(_t1.clone()))
        }

        #[java_method(name = "getGenericInfo", descriptor = "()Lsun/reflect/generics/repository/ClassRepository;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getGenericInfo(&self) -> Result<ClassRepository> {
            let this = self;
            let mut genericInfo = this.__get_genericInfo();
            let _t0 = this.getGenericSignature0()?;
            let mut signature: String = _t0;
            if _is_jnull(&signature) {
                genericInfo = ClassRepository::NONE();
            } else {
                let _t1 = this.getFactory()?;
                let _t2: ClassRepository = ClassRepository::make(Clone::clone(&signature), Clone::clone(&_t1))?;
                genericInfo = _t2;
            }
            this.__set_genericInfo(Clone::clone(&genericInfo));
            Ok((if Object::from_any(genericInfo.clone()) != Object::from_any(ClassRepository::NONE().clone()) { genericInfo } else { Default::default() }))
        }

        #[native]
        #[java_native(name = "getRawAnnotations", descriptor = "()[B", access = "package", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getRawAnnotations(&self) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("native: java/lang/Class.getRawAnnotations:()[B")
        }

        #[native]
        #[java_native(name = "getRawTypeAnnotations", descriptor = "()[B", access = "package", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getRawTypeAnnotations(&self) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("native: java/lang/Class.getRawTypeAnnotations:()[B")
        }

        #[java_method(name = "getExecutableTypeAnnotationBytes", descriptor = "(Ljava/lang/reflect/Executable;)[B", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getExecutableTypeAnnotationBytes(ex: Executable) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/lang/Class.getExecutableTypeAnnotationBytes:(Ljava/lang/reflect/Executable;)[B")
        }

        #[native]
        #[java_native(name = "getConstantPool", descriptor = "()Ljdk/internal/reflect/ConstantPool;", access = "package", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getConstantPool(&self) -> Result<Object> {
            panic!("native: java/lang/Class.getConstantPool:()Ljdk/internal/reflect/ConstantPool;")
        }

        #[java_method(name = "privateGetDeclaredFields", descriptor = "(Z)[Ljava/lang/reflect/Field;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn privateGetDeclaredFields(&self, publicOnly: bool) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.privateGetDeclaredFields:(Z)[Ljava/lang/reflect/Field;")
        }

        #[java_method(name = "privateGetPublicFields", descriptor = "()[Ljava/lang/reflect/Field;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn privateGetPublicFields(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.privateGetPublicFields:()[Ljava/lang/reflect/Field;")
        }

        #[java_method(name = "addAll", descriptor = "(Ljava/util/Collection;[Ljava/lang/reflect/Field;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<Ljava/lang/reflect/Field;>;[Ljava/lang/reflect/Field;)V")]
        pub fn addAll(c: Object, o: Rc<RefCell<Vec<Object>>>) -> Result<()> {
            panic!("stub: java/lang/Class.addAll:(Ljava/util/Collection;[Ljava/lang/reflect/Field;)V")
        }

        #[java_method(name = "privateGetDeclaredConstructors", descriptor = "(Z)[Ljava/lang/reflect/Constructor;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Z)[Ljava/lang/reflect/Constructor<TT;>;")]
        pub fn privateGetDeclaredConstructors(&self, publicOnly: bool) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.privateGetDeclaredConstructors:(Z)[Ljava/lang/reflect/Constructor;")
        }

        #[java_method(name = "privateGetDeclaredMethods", descriptor = "(Z)[Ljava/lang/reflect/Method;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn privateGetDeclaredMethods(&self, mut publicOnly: bool) -> Result<Rc<RefCell<Vec<Method>>>> {
            let this = self;
            let _t0 = this.reflectionData()?;
            let mut rd: Class_ReflectionData<Object> = _t0;
            let mut res = (if publicOnly { rd.__get_declaredPublicMethods() } else { rd.__get_declaredMethods() });
            if !_is_jnull(&res) {
                return Ok(res);
            }
            let _t1 = this.getDeclaredMethods0(publicOnly)?;
            let _t2: Rc<RefCell<Vec<Method>>> = Reflection::filterMethods(Clone::clone(this), Clone::clone(&_t1))?;
            res = _t2;
            if publicOnly {
                rd.__set_declaredPublicMethods(Clone::clone(&res));
            } else {
                rd.__set_declaredMethods(Clone::clone(&res));
            }
            Ok(res)
        }

        #[java_method(name = "privateGetPublicMethods", descriptor = "()[Ljava/lang/reflect/Method;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn privateGetPublicMethods(&self) -> Result<Rc<RefCell<Vec<Method>>>> {
            panic!("stub: java/lang/Class.privateGetPublicMethods:()[Ljava/lang/reflect/Method;")
        }

        #[java_method(name = "searchFields", descriptor = "([Ljava/lang/reflect/Field;Ljava/lang/String;)Ljava/lang/reflect/Field;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn searchFields(fields: Rc<RefCell<Vec<Object>>>, name: String) -> Result<Object> {
            panic!("stub: java/lang/Class.searchFields:([Ljava/lang/reflect/Field;Ljava/lang/String;)Ljava/lang/reflect/Field;")
        }

        #[java_method(name = "getField0", descriptor = "(Ljava/lang/String;)Ljava/lang/reflect/Field;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getField0(&self, name: String) -> Result<Object> {
            panic!("stub: java/lang/Class.getField0:(Ljava/lang/String;)Ljava/lang/reflect/Field;")
        }

        #[java_method(name = "searchMethods", descriptor = "([Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class<*>;)Ljava/lang/reflect/Method;")]
        pub fn searchMethods(methods: Rc<RefCell<Vec<Method>>>, name: String, parameterTypes: Rc<RefCell<Vec<Class<Object>>>>) -> Result<Method> {
            panic!("stub: java/lang/Class.searchMethods:([Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;")
        }

        #[java_method(name = "getMethod0", descriptor = "(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;[Ljava/lang/Class<*>;)Ljava/lang/reflect/Method;")]
        pub fn getMethod0(&self, mut name: String, mut parameterTypes: Rc<RefCell<Vec<Class<Object>>>>) -> Result<Method> {
            let this = self;
            let _t0 = this.getMethodsRecursive(Clone::clone(&name), Default::default(), (1i32 != 0i32))?;
            let mut res: PublicMethods_MethodList = _t0;
            let mut _merged2: Object;
            if _is_jnull(&res) {
                _merged2 = Object::default();
            } else {
                let _t1 = res.getMostSpecific()?;
                _merged2 = Object::from_any(Clone::clone(&_t1));
            }
            Ok(Default::default())
        }

        #[java_method(name = "getMethodsRecursive", descriptor = "(Ljava/lang/String;[Ljava/lang/Class;Z)Ljava/lang/PublicMethods$MethodList;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;[Ljava/lang/Class<*>;Z)Ljava/lang/PublicMethods$MethodList;")]
        pub fn getMethodsRecursive(&self, mut name: String, mut parameterTypes: Rc<RefCell<Vec<Class<Object>>>>, mut includeStatic: bool) -> Result<PublicMethods_MethodList> {
            let this = self;
            let _t0 = this.privateGetDeclaredMethods((1i32 != 0i32))?;
            let mut methods: Rc<RefCell<Vec<Method>>> = _t0;
            let _t1: PublicMethods_MethodList = PublicMethods_MethodList::filter(Clone::clone(&methods), Clone::clone(&name), Clone::clone(&parameterTypes), includeStatic)?;
            let mut res: PublicMethods_MethodList = _t1;
            if !_is_jnull(&res) {
                return Ok(res);
            }
            let _t2 = this.getSuperclass()?;
            let mut sc = (_t2).downcast::<Class<Object>>();
            if !_is_jnull(&sc) {
                let _t3 = sc.getMethodsRecursive(Clone::clone(&name), Clone::clone(&parameterTypes), includeStatic)?;
                res = _t3;
            }
            let _t3 = this.getInterfaces_z((0i32 != 0i32))?;
            let mut local_7: Rc<RefCell<Vec<Object>>> = _t3;
            let mut local_8 = (local_7.borrow().len() as i32);
            let mut local_9: i32 = 0i32;
            loop {
                if local_9 >= local_8 { break; }
                let mut intf = Clone::clone(&local_7.borrow()[local_9 as usize]);
                let _t4 = intf.getMethodsRecursive(Clone::clone(&name), Clone::clone(&parameterTypes), (0i32 != 0i32))?;
                let _t5: PublicMethods_MethodList = PublicMethods_MethodList::merge_public_public(Clone::clone(&res), Clone::clone(&_t4))?;
                res = _t5;
                local_9 = local_9.wrapping_add(1i32);
            }
            Ok(res)
        }

        #[java_method(name = "getConstructor0", descriptor = "([Ljava/lang/Class;I)Ljava/lang/reflect/Constructor;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NoSuchMethodException", generic_signature = "([Ljava/lang/Class<*>;I)Ljava/lang/reflect/Constructor<TT;>;")]
        pub fn getConstructor0(&self, parameterTypes: Rc<RefCell<Vec<Class<Object>>>>, which: i32) -> Result<Object> {
            panic!("stub: java/lang/Class.getConstructor0:([Ljava/lang/Class;I)Ljava/lang/reflect/Constructor;")
        }

        #[java_method(name = "arrayContentsEq", descriptor = "([Ljava/lang/Object;[Ljava/lang/Object;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn arrayContentsEq(a1: Rc<RefCell<Vec<Object>>>, a2: Rc<RefCell<Vec<Object>>>) -> Result<bool> {
            panic!("stub: java/lang/Class.arrayContentsEq:([Ljava/lang/Object;[Ljava/lang/Object;)Z")
        }

        #[java_method(name = "copyFields", descriptor = "([Ljava/lang/reflect/Field;)[Ljava/lang/reflect/Field;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyFields(arg: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.copyFields:([Ljava/lang/reflect/Field;)[Ljava/lang/reflect/Field;")
        }

        #[java_method(name = "copyMethods", descriptor = "([Ljava/lang/reflect/Method;)[Ljava/lang/reflect/Method;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyMethods(arg: Rc<RefCell<Vec<Method>>>) -> Result<Rc<RefCell<Vec<Method>>>> {
            panic!("stub: java/lang/Class.copyMethods:([Ljava/lang/reflect/Method;)[Ljava/lang/reflect/Method;")
        }

        #[java_method(name = "copyConstructors", descriptor = "([Ljava/lang/reflect/Constructor;)[Ljava/lang/reflect/Constructor;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>([Ljava/lang/reflect/Constructor<TU;>;)[Ljava/lang/reflect/Constructor<TU;>;")]
        pub fn copyConstructors(arg: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.copyConstructors:([Ljava/lang/reflect/Constructor;)[Ljava/lang/reflect/Constructor;")
        }

        #[native]
        #[java_native(name = "getDeclaredFields0", descriptor = "(Z)[Ljava/lang/reflect/Field;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getDeclaredFields0(&self, arg0: bool) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("native: java/lang/Class.getDeclaredFields0:(Z)[Ljava/lang/reflect/Field;")
        }

        #[native]
        #[java_native(name = "getDeclaredMethods0", descriptor = "(Z)[Ljava/lang/reflect/Method;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getDeclaredMethods0(&self, arg0: bool) -> Result<Rc<RefCell<Vec<Method>>>> {
            panic!("native: java/lang/Class.getDeclaredMethods0:(Z)[Ljava/lang/reflect/Method;")
        }

        #[native]
        #[java_native(name = "getDeclaredConstructors0", descriptor = "(Z)[Ljava/lang/reflect/Constructor;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Z)[Ljava/lang/reflect/Constructor<TT;>;")]
        pub fn getDeclaredConstructors0(&self, arg0: bool) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("native: java/lang/Class.getDeclaredConstructors0:(Z)[Ljava/lang/reflect/Constructor;")
        }

        #[native]
        #[java_native(name = "getDeclaredClasses0", descriptor = "()[Ljava/lang/Class;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
        pub fn getDeclaredClasses0(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("native: java/lang/Class.getDeclaredClasses0:()[Ljava/lang/Class;")
        }

        #[native]
        #[java_native(name = "getRecordComponents0", descriptor = "()[Ljava/lang/reflect/RecordComponent;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getRecordComponents0(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("native: java/lang/Class.getRecordComponents0:()[Ljava/lang/reflect/RecordComponent;")
        }

        #[native]
        #[java_native(name = "isRecord0", descriptor = "()Z", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn isRecord0(&self) -> Result<bool> {
            panic!("native: java/lang/Class.isRecord0:()Z")
        }

        #[java_method(name = "methodToString", descriptor = "(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;[Ljava/lang/Class<*>;)Ljava/lang/String;")]
        pub fn methodToString(&self, mut name: String, mut argTypes: Rc<RefCell<Vec<Class<Object>>>>) -> Result<String> {
            let this = self;
            let _t0 = this.getName()?;
            let _t1 = StringBuilder::new()?.append_str(Clone::clone(&_t0))?;
            let _t2 = _t1.append_c(((46i32) as u16))?;
            let _t3 = _t2.append_str(Clone::clone(&name))?;
            let mut _merged8: String;
            if ((argTypes.borrow().len() as i32)==0) {
                _merged8 = String::from("()");
            } else {
                let _t4: Object = Arrays::stream_arr_obj(Default::default())?;
                let __lam_968: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { Class::lambda_methodToString_0(_la0) });
                let _vdispatch5: Object = if let Some(_d) = _t4.0.as_any().downcast_ref::<WhileOps_1Op>() { _d.map(Clone::clone(&Object::from_any(__lam_968)))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<SliceOps_1>() { _d.map(Clone::clone(&Object::from_any(__lam_968)))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<WhileOps_1>() { _d.map(Clone::clone(&Object::from_any(__lam_968)))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<SortedOps_OfRef<Object>>() { _d.map(Clone::clone(&Object::from_any(__lam_968)))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<DistinctOps_1>() { _d.map(Clone::clone(&Object::from_any(__lam_968)))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<IntPipeline_1>() { _d.map(Clone::clone(&Object::from_any(__lam_968)))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<ReferencePipeline_2>() { _d.map(Clone::clone(&Object::from_any(__lam_968)))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<ReferencePipeline_3>() { _d.map(Clone::clone(&Object::from_any(__lam_968)))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<ReferencePipeline_7>() { _d.map(Clone::clone(&Object::from_any(__lam_968)))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<ReferencePipeline_15>() { _d.map(Clone::clone(&Object::from_any(__lam_968)))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<ReferencePipeline_StatefulOp<Object, Object>>() { _d.map(Clone::clone(&Object::from_any(__lam_968)))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<ReferencePipeline_StatelessOp<Object, Object>>() { _d.map(Clone::clone(&Object::from_any(__lam_968)))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<ReferencePipeline_Head<Object, Object>>() { _d.map(Clone::clone(&Object::from_any(__lam_968)))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<ReferencePipeline<Object, Object>>() { _d.map(Clone::clone(&Object::from_any(__lam_968)))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<Object>() { _d.map(Clone::clone(&Object::from_any(__lam_968)))? } else if let Some(__f) = _t4.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&Object::from_any(__lam_968)))? } else { Default::default() };
                let _t6: Object = Collectors::joining_seq_seq_seq(Object::from_any(String::from(",").clone()), Object::from_any(String::from("(").clone()), Object::from_any(String::from(")").clone()))?;
                let _vdispatch7: Object = if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<WhileOps_1Op>() { _d.collect(Clone::clone(&_t6))? } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<SliceOps_1>() { _d.collect(Clone::clone(&_t6))? } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<WhileOps_1>() { _d.collect(Clone::clone(&_t6))? } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<SortedOps_OfRef<Object>>() { _d.collect(Clone::clone(&_t6))? } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<DistinctOps_1>() { _d.collect(Clone::clone(&_t6))? } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<IntPipeline_1>() { _d.collect(Clone::clone(&_t6))? } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<ReferencePipeline_2>() { _d.collect(Clone::clone(&_t6))? } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<ReferencePipeline_3>() { _d.collect(Clone::clone(&_t6))? } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<ReferencePipeline_7>() { _d.collect(Clone::clone(&_t6))? } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<ReferencePipeline_15>() { _d.collect(Clone::clone(&_t6))? } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<ReferencePipeline_StatefulOp<Object, Object>>() { _d.collect(Clone::clone(&_t6))? } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<ReferencePipeline_StatelessOp<Object, Object>>() { _d.collect(Clone::clone(&_t6))? } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<ReferencePipeline_Head<Object, Object>>() { _d.collect(Clone::clone(&_t6))? } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<ReferencePipeline<Object, Object>>() { _d.collect(Clone::clone(&_t6))? } else if let Some(_d) = _vdispatch5.0.as_any().downcast_ref::<Object>() { _d.collect(Clone::clone(&_t6))? } else if let Some(__f) = _vdispatch5.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&_t6))? } else { Default::default() };
                _merged8 = (_vdispatch7).downcast::<String>();
            }
            let _t9 = _t3.append_str(Clone::clone(&_merged8))?;
            let _t10 = _t9.toString()?;
            Ok(_t10)
        }

        #[java_method(name = "desiredAssertionStatus", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn desiredAssertionStatus(&self) -> Result<bool> {
            panic!("stub: java/lang/Class.desiredAssertionStatus:()Z")
        }

        #[native]
        #[java_native(name = "desiredAssertionStatus0", descriptor = "(Ljava/lang/Class;)Z", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z")]
        pub fn desiredAssertionStatus0(arg0: Class<Object>) -> Result<bool> {
            panic!("native: java/lang/Class.desiredAssertionStatus0:(Ljava/lang/Class;)Z")
        }

        #[java_method(name = "isEnum", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEnum(&self) -> Result<bool> {
            let this = self;
            let _t0 = this.getModifiers()?;
            let mut _merged2: bool;
            if ((_t0&16384i32)!=0) {
                let _t1 = this.getSuperclass()?;
                _merged2 = _t1 == Object::default();
            } else {
                _merged2 = (0i32 != 0);
            }
            Ok(_merged2)
        }

        #[java_method(name = "isRecord", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isRecord(&self) -> Result<bool> {
            panic!("stub: java/lang/Class.isRecord:()Z")
        }

        #[java_method(name = "getReflectionFactory", descriptor = "()Ljdk/internal/reflect/ReflectionFactory;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getReflectionFactory() -> Result<ReflectionFactory> {
            let mut factory: ReflectionFactory = Class::<Object>::reflectionFactory();
            if !_is_jnull(&factory) {
                return Ok(factory);
            }
            let _t0: Object = AccessController::doPrivileged_privil(Clone::clone(&ReflectionFactory_GetReflectionFactoryAction::new()?).into())?;
            Class::set_reflectionFactory((_t0).downcast::<ReflectionFactory>());
            Ok((_t0).downcast::<ReflectionFactory>())
        }

        #[java_method(name = "getEnumConstants", descriptor = "()[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[TT;")]
        pub fn getEnumConstants(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.getEnumConstants:()[Ljava/lang/Object;")
        }

        #[java_method(name = "getEnumConstantsShared", descriptor = "()[Ljava/lang/Object;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[TT;")]
        pub fn getEnumConstantsShared(&self) -> Result<Rc<RefCell<Vec<T>>>> {
            let this = self;
            let mut constants = this.__get_enumConstants();
            let _t0 = this.isEnum()?;
            if !(_t0) {
                return Ok(Default::default());
            }
            let mut _arr1: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 0i32 as usize]));
            let _t2 = this.getMethod(Clone::clone(&String::from("values")), Default::default())?;
            let mut values: Method = _t2;
            let _t3: Object = AccessController::doPrivileged_privil(Clone::clone(&Class_3::new(Object::from_any(Clone::clone(self)), Clone::clone(&values))?).into())?;
            let mut _arr4: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 0i32 as usize]));
            let _t5 = values.invoke_obj_arr_obj(Clone::clone(&Object::default()), Clone::clone(&_arr4))?;
            let mut temporaryConstants = (_t5).downcast::<Rc<RefCell<Vec<T>>>>();
            let mut constants: Rc<RefCell<Vec<T>>> = temporaryConstants;
            this.__set_enumConstants(Clone::clone(&constants));
            Ok(constants)
        }

        #[java_method(name = "enumConstantDirectory", descriptor = "()Ljava/util/Map;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Map<Ljava/lang/String;TT;>;")]
        pub fn enumConstantDirectory(&self) -> Result<Object> {
            let this = self;
            let mut directory = this.__get_enumConstantDirectory();
            let _t0 = this.getEnumConstantsShared()?;
            let mut universe: Rc<RefCell<Vec<Object>>> = _t0;
            if _is_jnull(&universe) {
                let _t1 = this.getName()?;
                let _t2 = StringBuilder::new()?.append_str(Clone::clone(&_t1))?;
                let _t3 = _t2.append_str(Clone::clone(&String::from(" is not an enum class")))?;
                let _t4 = _t3.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t1: HashMap<Object, Object> = HashMap::<Object, Object>::newHashMap((universe.borrow().len() as i32))?;
            let mut directory: HashMap<Object, Object> = _t1;
            let mut local_3: Rc<RefCell<Vec<Object>>> = universe;
            let mut local_4 = (local_3.borrow().len() as i32);
            let mut local_5: i32 = 0i32;
            loop {
                if local_5 >= local_4 { break; }
                let mut constant = Clone::clone(&local_3.borrow()[local_5 as usize]);
                let _t2 = Default::default().name()?;
                let _t3 = directory.put(Object::from_any(_t2.clone()), Clone::clone(&constant))?;
                local_5 = local_5.wrapping_add(1i32);
            }
            this.__set_enumConstantDirectory(Object::from_any(directory.clone()));
            Ok(Object::from_any(directory.clone()))
        }

        #[java_method(name = "cast", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TT;")]
        pub fn cast(&self, obj: Object) -> Result<Object> {
            panic!("stub: java/lang/Class.cast:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "cannotCastMsg", descriptor = "(Ljava/lang/Object;)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn cannotCastMsg(&self, obj: Object) -> Result<String> {
            panic!("stub: java/lang/Class.cannotCastMsg:(Ljava/lang/Object;)Ljava/lang/String;")
        }

        #[java_method(name = "asSubclass", descriptor = "(Ljava/lang/Class;)Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(Ljava/lang/Class<TU;>;)Ljava/lang/Class<+TU;>;")]
        pub fn asSubclass(&self, clazz: Class<Object>) -> Result<Object> {
            panic!("stub: java/lang/Class.asSubclass:(Ljava/lang/Class;)Ljava/lang/Class;")
        }

        #[java_method(name = "getAnnotation", descriptor = "(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<A::Ljava/lang/annotation/Annotation;>(Ljava/lang/Class<TA;>;)TA;")]
        pub fn getAnnotation(&self, annotationClass: Class<Object>) -> Result<Object> {
            panic!("stub: java/lang/Class.getAnnotation:(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "isAnnotationPresent", descriptor = "(Ljava/lang/Class;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<+Ljava/lang/annotation/Annotation;>;)Z")]
        pub fn isAnnotationPresent(&self, annotationClass: Object) -> Result<bool> {
            panic!("stub: java/lang/Class.isAnnotationPresent:(Ljava/lang/Class;)Z")
        }

        #[java_method(name = "getAnnotationsByType", descriptor = "(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<A::Ljava/lang/annotation/Annotation;>(Ljava/lang/Class<TA;>;)[TA;")]
        pub fn getAnnotationsByType(&self, annotationClass: Class<Object>) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.getAnnotationsByType:(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "getAnnotations", descriptor = "()[Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAnnotations(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.getAnnotations:()[Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "getDeclaredAnnotation", descriptor = "(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<A::Ljava/lang/annotation/Annotation;>(Ljava/lang/Class<TA;>;)TA;")]
        pub fn getDeclaredAnnotation(&self, annotationClass: Class<Object>) -> Result<Object> {
            panic!("stub: java/lang/Class.getDeclaredAnnotation:(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "getDeclaredAnnotationsByType", descriptor = "(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<A::Ljava/lang/annotation/Annotation;>(Ljava/lang/Class<TA;>;)[TA;")]
        pub fn getDeclaredAnnotationsByType(&self, annotationClass: Class<Object>) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.getDeclaredAnnotationsByType:(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "getDeclaredAnnotations", descriptor = "()[Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDeclaredAnnotations(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.getDeclaredAnnotations:()[Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "annotationData", descriptor = "()Ljava/lang/Class$AnnotationData;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn annotationData(&self) -> Result<Object> {
            panic!("stub: java/lang/Class.annotationData:()Ljava/lang/Class$AnnotationData;")
        }

        #[java_method(name = "createAnnotationData", descriptor = "(I)Ljava/lang/Class$AnnotationData;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createAnnotationData(&self, classRedefinedCount: i32) -> Result<Object> {
            panic!("stub: java/lang/Class.createAnnotationData:(I)Ljava/lang/Class$AnnotationData;")
        }

        #[java_method(name = "casAnnotationType", descriptor = "(Lsun/reflect/annotation/AnnotationType;Lsun/reflect/annotation/AnnotationType;)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn casAnnotationType(&self, oldType: Object, newType: Object) -> Result<bool> {
            panic!("stub: java/lang/Class.casAnnotationType:(Lsun/reflect/annotation/AnnotationType;Lsun/reflect/annotation/AnnotationType;)Z")
        }

        #[java_method(name = "getAnnotationType", descriptor = "()Lsun/reflect/annotation/AnnotationType;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAnnotationType(&self) -> Result<Object> {
            panic!("stub: java/lang/Class.getAnnotationType:()Lsun/reflect/annotation/AnnotationType;")
        }

        #[java_method(name = "getDeclaredAnnotationMap", descriptor = "()Ljava/util/Map;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Map<Ljava/lang/Class<+Ljava/lang/annotation/Annotation;>;Ljava/lang/annotation/Annotation;>;")]
        pub fn getDeclaredAnnotationMap(&self) -> Result<Object> {
            panic!("stub: java/lang/Class.getDeclaredAnnotationMap:()Ljava/util/Map;")
        }

        #[java_method(name = "getAnnotatedSuperclass", descriptor = "()Ljava/lang/reflect/AnnotatedType;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAnnotatedSuperclass(&self) -> Result<Object> {
            panic!("stub: java/lang/Class.getAnnotatedSuperclass:()Ljava/lang/reflect/AnnotatedType;")
        }

        #[java_method(name = "getAnnotatedInterfaces", descriptor = "()[Ljava/lang/reflect/AnnotatedType;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAnnotatedInterfaces(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.getAnnotatedInterfaces:()[Ljava/lang/reflect/AnnotatedType;")
        }

        #[native]
        #[java_native(name = "getNestHost0", descriptor = "()Ljava/lang/Class;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;")]
        pub fn getNestHost0(&self) -> Result<Object> {
            panic!("native: java/lang/Class.getNestHost0:()Ljava/lang/Class;")
        }

        #[java_method(name = "getNestHost", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;")]
        pub fn getNestHost(&self) -> Result<Object> {
            panic!("stub: java/lang/Class.getNestHost:()Ljava/lang/Class;")
        }

        #[java_method(name = "isNestmateOf", descriptor = "(Ljava/lang/Class;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z")]
        pub fn isNestmateOf(&self, c: Class<Object>) -> Result<bool> {
            panic!("stub: java/lang/Class.isNestmateOf:(Ljava/lang/Class;)Z")
        }

        #[native]
        #[java_native(name = "getNestMembers0", descriptor = "()[Ljava/lang/Class;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
        pub fn getNestMembers0(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("native: java/lang/Class.getNestMembers0:()[Ljava/lang/Class;")
        }

        #[java_method(name = "getNestMembers", descriptor = "()[Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
        pub fn getNestMembers(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.getNestMembers:()[Ljava/lang/Class;")
        }

        #[java_method(name = "descriptorString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn descriptorString(&self) -> Result<String> {
            panic!("stub: java/lang/Class.descriptorString:()Ljava/lang/String;")
        }

        #[java_method(name = "componentType", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;")]
        pub fn componentType(&self) -> Result<Object> {
            panic!("stub: java/lang/Class.componentType:()Ljava/lang/Class;")
        }

        #[java_method(name = "arrayType", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;")]
        pub fn arrayType(&self) -> Result<Object> {
            panic!("stub: java/lang/Class.arrayType:()Ljava/lang/Class;")
        }

        #[java_method(name = "describeConstable", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Optional<Ljava/lang/constant/ClassDesc;>;")]
        pub fn describeConstable(&self) -> Result<Optional<Object>> {
            panic!("stub: java/lang/Class.describeConstable:()Ljava/util/Optional;")
        }

        #[native]
        #[java_native(name = "isHidden", descriptor = "()Z", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn isHidden(&self) -> Result<bool> {
            panic!("native: java/lang/Class.isHidden:()Z")
        }

        #[java_method(name = "getPermittedSubclasses", descriptor = "()[Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
        pub fn getPermittedSubclasses(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/Class.getPermittedSubclasses:()[Ljava/lang/Class;")
        }

        #[java_method(name = "isDirectSubType", descriptor = "(Ljava/lang/Class;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Z")]
        pub fn isDirectSubType(&self, c: Class<Object>) -> Result<bool> {
            panic!("stub: java/lang/Class.isDirectSubType:(Ljava/lang/Class;)Z")
        }

        #[java_method(name = "isSealed", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSealed(&self) -> Result<bool> {
            panic!("stub: java/lang/Class.isSealed:()Z")
        }

        #[native]
        #[java_native(name = "getPermittedSubclasses0", descriptor = "()[Ljava/lang/Class;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
        pub fn getPermittedSubclasses0(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("native: java/lang/Class.getPermittedSubclasses0:()[Ljava/lang/Class;")
        }

        #[java_method(name = "getClassFileVersion", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getClassFileVersion(&self) -> Result<i32> {
            panic!("stub: java/lang/Class.getClassFileVersion:()I")
        }

        #[native]
        #[java_native(name = "getClassFileVersion0", descriptor = "()I", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getClassFileVersion0(&self) -> Result<i32> {
            panic!("native: java/lang/Class.getClassFileVersion0:()I")
        }

        #[java_method(name = "getClassAccessFlagsRaw", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getClassAccessFlagsRaw(&self) -> Result<i32> {
            panic!("stub: java/lang/Class.getClassAccessFlagsRaw:()I")
        }

        #[native]
        #[java_native(name = "getClassAccessFlagsRaw0", descriptor = "()I", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getClassAccessFlagsRaw0(&self) -> Result<i32> {
            panic!("native: java/lang/Class.getClassAccessFlagsRaw0:()I")
        }
    }
}
