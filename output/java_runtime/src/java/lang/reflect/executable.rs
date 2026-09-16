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

impl From<Executable> for AccessibleObject {
    fn from(v: Executable) -> AccessibleObject { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/reflect/Executable"]
    #[super_class       = "java/lang/reflect/AccessibleObject"]
    #[interfaces        = "java/lang/reflect/Member,java/lang/reflect/GenericDeclaration"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Executable.java"]
    #[inner_classes     = "java/lang/reflect/AccessFlag$Location:java/lang/reflect/AccessFlag:Location:16409;java/lang/reflect/Executable$ParameterData:java/lang/reflect/Executable:ParameterData:24;sun/reflect/annotation/TypeAnnotation$TypeAnnotationTarget:sun/reflect/annotation/TypeAnnotation:TypeAnnotationTarget:16409;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AccessibleObject"]
    #[superclass_fields(override_: bool, accessCheckCache: Object)]
    #[all_supertypes    = "java/lang/Object;java/lang/reflect/AccessibleObject;java/lang/reflect/AnnotatedElement;java/lang/reflect/Executable;java/lang/reflect/GenericDeclaration;java/lang/reflect/Member"]

    pub struct Executable {
        #[cfg_attr(any(), java_field(name = "parameterData", descriptor = "Ljava/lang/reflect/Executable$ParameterData;", access = "private", modifiers = "transient", is_static = false))]
        pub parameterData: Object,
        #[cfg_attr(any(), java_field(name = "declaredAnnotations", descriptor = "Ljava/util/Map;", access = "private", modifiers = "volatile transient", is_static = false, generic_signature = "Ljava/util/Map<Ljava/lang/Class<+Ljava/lang/annotation/Annotation;>;Ljava/lang/annotation/Annotation;>;"))]
        pub declaredAnnotations: Object,
    }

    impl Executable {
        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/reflect/Executable.<init>:()V")
        }

        #[java_method(name = "getAnnotationBytes", descriptor = "()[B", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getAnnotationBytes(&self) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/lang/reflect/Executable.getAnnotationBytes:()[B")
        }

        #[java_method(name = "hasGenericInformation", descriptor = "()Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn hasGenericInformation(&self) -> Result<bool> {
            panic!("stub: java/lang/reflect/Executable.hasGenericInformation:()Z")
        }

        #[java_method(name = "getGenericInfo", descriptor = "()Lsun/reflect/generics/repository/ConstructorRepository;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getGenericInfo(&self) -> Result<Object> {
            panic!("stub: java/lang/reflect/Executable.getGenericInfo:()Lsun/reflect/generics/repository/ConstructorRepository;")
        }

        #[java_method(name = "equalParamTypes", descriptor = "([Ljava/lang/Class;[Ljava/lang/Class;)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([Ljava/lang/Class<*>;[Ljava/lang/Class<*>;)Z")]
        pub fn equalParamTypes(&self, params1: Rc<RefCell<Vec<Object>>>, params2: Rc<RefCell<Vec<Object>>>) -> Result<bool> {
            panic!("stub: java/lang/reflect/Executable.equalParamTypes:([Ljava/lang/Class;[Ljava/lang/Class;)Z")
        }

        #[java_method(name = "parseParameterAnnotations", descriptor = "([B)[[Ljava/lang/annotation/Annotation;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parseParameterAnnotations(&self, parameterAnnotations: Rc<RefCell<Vec<i8>>>) -> Result<Rc<RefCell<Vec<Rc<RefCell<Vec<Object>>>>>>> {
            panic!("stub: java/lang/reflect/Executable.parseParameterAnnotations:([B)[[Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "printModifiersIfNonzero", descriptor = "(Ljava/lang/StringBuilder;IZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn printModifiersIfNonzero(&self, sb: StringBuilder, mask: i32, isDefault: bool) -> Result<()> {
            panic!("stub: java/lang/reflect/Executable.printModifiersIfNonzero:(Ljava/lang/StringBuilder;IZ)V")
        }

        #[java_method(name = "sharedToString", descriptor = "(IZ[Ljava/lang/Class;[Ljava/lang/Class;)Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(IZ[Ljava/lang/Class<*>;[Ljava/lang/Class<*>;)Ljava/lang/String;")]
        pub fn sharedToString(&self, modifierMask: i32, isDefault: bool, parameterTypes: Rc<RefCell<Vec<Object>>>, exceptionTypes: Rc<RefCell<Vec<Object>>>) -> Result<String> {
            panic!("stub: java/lang/reflect/Executable.sharedToString:(IZ[Ljava/lang/Class;[Ljava/lang/Class;)Ljava/lang/String;")
        }

        #[java_method(name = "specificToStringHeader", descriptor = "(Ljava/lang/StringBuilder;)V", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn specificToStringHeader(&self, arg0: StringBuilder) -> Result<()> {
            panic!("stub: java/lang/reflect/Executable.specificToStringHeader:(Ljava/lang/StringBuilder;)V")
        }

        #[java_method(name = "typeVarBounds", descriptor = "(Ljava/lang/reflect/TypeVariable;)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/reflect/TypeVariable<*>;)Ljava/lang/String;")]
        pub fn typeVarBounds(typeVar: Object) -> Result<String> {
            panic!("stub: java/lang/reflect/Executable.typeVarBounds:(Ljava/lang/reflect/TypeVariable;)Ljava/lang/String;")
        }

        #[java_method(name = "sharedToGenericString", descriptor = "(IZ)Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sharedToGenericString(&self, modifierMask: i32, isDefault: bool) -> Result<String> {
            panic!("stub: java/lang/reflect/Executable.sharedToGenericString:(IZ)Ljava/lang/String;")
        }

        #[java_method(name = "specificToGenericStringHeader", descriptor = "(Ljava/lang/StringBuilder;)V", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn specificToGenericStringHeader(&self, arg0: StringBuilder) -> Result<()> {
            panic!("stub: java/lang/reflect/Executable.specificToGenericStringHeader:(Ljava/lang/StringBuilder;)V")
        }

        #[java_method(name = "getDeclaringClass", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;")]
        pub fn getDeclaringClass(&self) -> Result<Object> {
            panic!("stub: java/lang/reflect/Executable.getDeclaringClass:()Ljava/lang/Class;")
        }

        #[java_method(name = "getName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getName(&self) -> Result<String> {
            panic!("stub: java/lang/reflect/Executable.getName:()Ljava/lang/String;")
        }

        #[java_method(name = "getModifiers", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getModifiers(&self) -> Result<i32> {
            panic!("stub: java/lang/reflect/Executable.getModifiers:()I")
        }

        #[java_method(name = "accessFlags", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/lang/reflect/AccessFlag;>;")]
        pub fn accessFlags(&self) -> Result<Object> {
            panic!("stub: java/lang/reflect/Executable.accessFlags:()Ljava/util/Set;")
        }

        #[java_method(name = "getTypeParameters", descriptor = "()[Ljava/lang/reflect/TypeVariable;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()[Ljava/lang/reflect/TypeVariable<*>;")]
        pub fn getTypeParameters(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Executable.getTypeParameters:()[Ljava/lang/reflect/TypeVariable;")
        }

        #[java_method(name = "getSharedParameterTypes", descriptor = "()[Ljava/lang/Class;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
        pub fn getSharedParameterTypes(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Executable.getSharedParameterTypes:()[Ljava/lang/Class;")
        }

        #[java_method(name = "getSharedExceptionTypes", descriptor = "()[Ljava/lang/Class;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
        pub fn getSharedExceptionTypes(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Executable.getSharedExceptionTypes:()[Ljava/lang/Class;")
        }

        #[java_method(name = "getParameterTypes", descriptor = "()[Ljava/lang/Class;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
        pub fn getParameterTypes(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Executable.getParameterTypes:()[Ljava/lang/Class;")
        }

        #[java_method(name = "getParameterCount", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getParameterCount(&self) -> Result<i32> {
            panic!("stub: java/lang/reflect/Executable.getParameterCount:()I")
        }

        #[java_method(name = "getGenericParameterTypes", descriptor = "()[Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getGenericParameterTypes(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Executable.getGenericParameterTypes:()[Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "getAllGenericParameterTypes", descriptor = "()[Ljava/lang/reflect/Type;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAllGenericParameterTypes(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Executable.getAllGenericParameterTypes:()[Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "getParameters", descriptor = "()[Ljava/lang/reflect/Parameter;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getParameters(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Executable.getParameters:()[Ljava/lang/reflect/Parameter;")
        }

        #[java_method(name = "synthesizeAllParams", descriptor = "()[Ljava/lang/reflect/Parameter;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn synthesizeAllParams(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Executable.synthesizeAllParams:()[Ljava/lang/reflect/Parameter;")
        }

        #[java_method(name = "verifyParameters", descriptor = "([Ljava/lang/reflect/Parameter;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn verifyParameters(&self, parameters: Rc<RefCell<Vec<Object>>>) -> Result<()> {
            panic!("stub: java/lang/reflect/Executable.verifyParameters:([Ljava/lang/reflect/Parameter;)V")
        }

        #[java_method(name = "hasRealParameterData", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasRealParameterData(&self) -> Result<bool> {
            panic!("stub: java/lang/reflect/Executable.hasRealParameterData:()Z")
        }

        #[java_method(name = "parameterData", descriptor = "()Ljava/lang/reflect/Executable$ParameterData;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parameterData(&self) -> Result<Object> {
            panic!("stub: java/lang/reflect/Executable.parameterData:()Ljava/lang/reflect/Executable$ParameterData;")
        }

        #[native]
        #[java_native(name = "getParameters0", descriptor = "()[Ljava/lang/reflect/Parameter;", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getParameters0(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("native: java/lang/reflect/Executable.getParameters0:()[Ljava/lang/reflect/Parameter;")
        }

        #[native]
        #[java_native(name = "getTypeAnnotationBytes0", descriptor = "()[B", access = "package", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getTypeAnnotationBytes0(&self) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("native: java/lang/reflect/Executable.getTypeAnnotationBytes0:()[B")
        }

        #[java_method(name = "getTypeAnnotationBytes", descriptor = "()[B", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTypeAnnotationBytes(&self) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/lang/reflect/Executable.getTypeAnnotationBytes:()[B")
        }

        #[java_method(name = "getExceptionTypes", descriptor = "()[Ljava/lang/Class;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
        pub fn getExceptionTypes(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Executable.getExceptionTypes:()[Ljava/lang/Class;")
        }

        #[java_method(name = "getGenericExceptionTypes", descriptor = "()[Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getGenericExceptionTypes(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Executable.getGenericExceptionTypes:()[Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "toGenericString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn toGenericString(&self) -> Result<String> {
            panic!("stub: java/lang/reflect/Executable.toGenericString:()Ljava/lang/String;")
        }

        #[java_method(name = "isVarArgs", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isVarArgs(&self) -> Result<bool> {
            panic!("stub: java/lang/reflect/Executable.isVarArgs:()Z")
        }

        #[java_method(name = "isSynthetic", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSynthetic(&self) -> Result<bool> {
            panic!("stub: java/lang/reflect/Executable.isSynthetic:()Z")
        }

        #[java_method(name = "getParameterAnnotations", descriptor = "()[[Ljava/lang/annotation/Annotation;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getParameterAnnotations(&self) -> Result<Rc<RefCell<Vec<Rc<RefCell<Vec<Object>>>>>>> {
            panic!("stub: java/lang/reflect/Executable.getParameterAnnotations:()[[Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "sharedGetParameterAnnotations", descriptor = "([Ljava/lang/Class;[B)[[Ljava/lang/annotation/Annotation;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([Ljava/lang/Class<*>;[B)[[Ljava/lang/annotation/Annotation;")]
        pub fn sharedGetParameterAnnotations(&self, parameterTypes: Rc<RefCell<Vec<Object>>>, parameterAnnotations: Rc<RefCell<Vec<i8>>>) -> Result<Rc<RefCell<Vec<Rc<RefCell<Vec<Object>>>>>>> {
            panic!("stub: java/lang/reflect/Executable.sharedGetParameterAnnotations:([Ljava/lang/Class;[B)[[Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "handleParameterNumberMismatch", descriptor = "(I[Ljava/lang/Class;)Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(I[Ljava/lang/Class<*>;)Z")]
        pub fn handleParameterNumberMismatch(&self, arg0: i32, arg1: Rc<RefCell<Vec<Object>>>) -> Result<bool> {
            panic!("stub: java/lang/reflect/Executable.handleParameterNumberMismatch:(I[Ljava/lang/Class;)Z")
        }

        #[java_method(name = "getAnnotation", descriptor = "(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T::Ljava/lang/annotation/Annotation;>(Ljava/lang/Class<TT;>;)TT;")]
        pub fn getAnnotation(&self, annotationClass: Object) -> Result<Object> {
            panic!("stub: java/lang/reflect/Executable.getAnnotation:(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "getAnnotationsByType", descriptor = "(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T::Ljava/lang/annotation/Annotation;>(Ljava/lang/Class<TT;>;)[TT;")]
        pub fn getAnnotationsByType(&self, annotationClass: Object) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Executable.getAnnotationsByType:(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "getDeclaredAnnotations", descriptor = "()[Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDeclaredAnnotations(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Executable.getDeclaredAnnotations:()[Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "declaredAnnotations", descriptor = "()Ljava/util/Map;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Map<Ljava/lang/Class<+Ljava/lang/annotation/Annotation;>;Ljava/lang/annotation/Annotation;>;")]
        pub fn declaredAnnotations(&self) -> Result<Object> {
            panic!("stub: java/lang/reflect/Executable.declaredAnnotations:()Ljava/util/Map;")
        }

        #[java_method(name = "getAnnotatedReturnType", descriptor = "()Ljava/lang/reflect/AnnotatedType;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getAnnotatedReturnType(&self) -> Result<Object> {
            panic!("stub: java/lang/reflect/Executable.getAnnotatedReturnType:()Ljava/lang/reflect/AnnotatedType;")
        }

        #[java_method(name = "getAnnotatedReturnType0", descriptor = "(Ljava/lang/reflect/Type;)Ljava/lang/reflect/AnnotatedType;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAnnotatedReturnType0(&self, returnType: Object) -> Result<Object> {
            panic!("stub: java/lang/reflect/Executable.getAnnotatedReturnType0:(Ljava/lang/reflect/Type;)Ljava/lang/reflect/AnnotatedType;")
        }

        #[java_method(name = "getAnnotatedReceiverType", descriptor = "()Ljava/lang/reflect/AnnotatedType;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAnnotatedReceiverType(&self) -> Result<Object> {
            panic!("stub: java/lang/reflect/Executable.getAnnotatedReceiverType:()Ljava/lang/reflect/AnnotatedType;")
        }

        #[java_method(name = "parameterize", descriptor = "(Ljava/lang/Class;)Ljava/lang/reflect/Type;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)Ljava/lang/reflect/Type;")]
        pub fn parameterize(&self, c: Object) -> Result<Object> {
            panic!("stub: java/lang/reflect/Executable.parameterize:(Ljava/lang/Class;)Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "getAnnotatedParameterTypes", descriptor = "()[Ljava/lang/reflect/AnnotatedType;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAnnotatedParameterTypes(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Executable.getAnnotatedParameterTypes:()[Ljava/lang/reflect/AnnotatedType;")
        }

        #[java_method(name = "getAnnotatedExceptionTypes", descriptor = "()[Ljava/lang/reflect/AnnotatedType;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAnnotatedExceptionTypes(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Executable.getAnnotatedExceptionTypes:()[Ljava/lang/reflect/AnnotatedType;")
        }
    }
}
