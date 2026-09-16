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
use crate::jdk::internal::reflect::MethodAccessor;
use crate::jdk::internal::reflect::Reflection;
use crate::jdk::internal::reflect::ReflectionFactory;

impl From<Method> for Executable {
    fn from(v: Method) -> Executable { v.__into_super() }
}

impl From<Method> for AccessibleObject {
    fn from(v: Method) -> AccessibleObject { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/reflect/Method"]
    #[super_class       = "java/lang/reflect/Executable"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Method.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Executable"]
    #[superclass_fields(override_: bool, accessCheckCache: Object, parameterData: Object, declaredAnnotations: Object)]
    #[all_supertypes    = "java/lang/Object;java/lang/reflect/AccessibleObject;java/lang/reflect/AnnotatedElement;java/lang/reflect/Executable;java/lang/reflect/GenericDeclaration;java/lang/reflect/Member;java/lang/reflect/Method"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct Method {
        #[cfg_attr(any(), java_field(name = "clazz", descriptor = "Ljava/lang/Class;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/lang/Class<*>;"))]
        pub clazz: Class<Object>,
        #[cfg_attr(any(), java_field(name = "slot", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub slot: i32,
        #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub name: String,
        #[cfg_attr(any(), java_field(name = "returnType", descriptor = "Ljava/lang/Class;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/lang/Class<*>;"))]
        pub returnType: Class<Object>,
        #[cfg_attr(any(), java_field(name = "parameterTypes", descriptor = "[Ljava/lang/Class;", access = "private", modifiers = "final", is_static = false, generic_signature = "[Ljava/lang/Class<*>;"))]
        pub parameterTypes: Rc<RefCell<Vec<Class<Object>>>>,
        #[cfg_attr(any(), java_field(name = "exceptionTypes", descriptor = "[Ljava/lang/Class;", access = "private", modifiers = "final", is_static = false, generic_signature = "[Ljava/lang/Class<*>;"))]
        pub exceptionTypes: Rc<RefCell<Vec<Class<Object>>>>,
        #[cfg_attr(any(), java_field(name = "modifiers", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub modifiers: i32,
        #[cfg_attr(any(), java_field(name = "signature", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final transient", is_static = false))]
        pub signature: String,
        #[cfg_attr(any(), java_field(name = "genericInfo", descriptor = "Lsun/reflect/generics/repository/MethodRepository;", access = "private", modifiers = "volatile transient", is_static = false))]
        pub genericInfo: Object,
        #[cfg_attr(any(), java_field(name = "annotations", descriptor = "[B", access = "private", modifiers = "final", is_static = false))]
        pub annotations: Rc<RefCell<Vec<i8>>>,
        #[cfg_attr(any(), java_field(name = "parameterAnnotations", descriptor = "[B", access = "private", modifiers = "final", is_static = false))]
        pub parameterAnnotations: Rc<RefCell<Vec<i8>>>,
        #[cfg_attr(any(), java_field(name = "annotationDefault", descriptor = "[B", access = "private", modifiers = "final", is_static = false))]
        pub annotationDefault: Rc<RefCell<Vec<i8>>>,
        #[cfg_attr(any(), java_field(name = "methodAccessor", descriptor = "Ljdk/internal/reflect/MethodAccessor;", access = "private", modifiers = "", is_static = false))]
        pub methodAccessor: Object,
        #[cfg_attr(any(), java_field(name = "root", descriptor = "Ljava/lang/reflect/Method;", access = "private", modifiers = "", is_static = false))]
        pub root: Method,
        #[cfg_attr(any(), java_field(name = "callerSensitive", descriptor = "B", access = "private", modifiers = "", is_static = false))]
        pub callerSensitive: i8,
    }

    impl Method {
        #[java_method(name = "getGenericSignature", descriptor = "()Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getGenericSignature(&self) -> Result<String> {
            panic!("stub: java/lang/reflect/Method.getGenericSignature:()Ljava/lang/String;")
        }

        #[java_method(name = "getFactory", descriptor = "()Lsun/reflect/generics/factory/GenericsFactory;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFactory(&self) -> Result<Object> {
            panic!("stub: java/lang/reflect/Method.getFactory:()Lsun/reflect/generics/factory/GenericsFactory;")
        }

        #[java_method(name = "getGenericInfo", descriptor = "()Lsun/reflect/generics/repository/MethodRepository;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getGenericInfo(&self) -> Result<Object> {
            panic!("stub: java/lang/reflect/Method.getGenericInfo:()Lsun/reflect/generics/repository/MethodRepository;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Class;Ljava/lang/String;[Ljava/lang/Class;Ljava/lang/Class;[Ljava/lang/Class;IILjava/lang/String;[B[B[B)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;Ljava/lang/String;[Ljava/lang/Class<*>;Ljava/lang/Class<*>;[Ljava/lang/Class<*>;IILjava/lang/String;[B[B[B)V")]
        pub fn new(declaringClass: Object, name: String, parameterTypes: Rc<RefCell<Vec<Object>>>, returnType: Object, checkedExceptions: Rc<RefCell<Vec<Object>>>, modifiers: i32, slot: i32, signature: String, annotations: Rc<RefCell<Vec<i8>>>, parameterAnnotations: Rc<RefCell<Vec<i8>>>, annotationDefault: Rc<RefCell<Vec<i8>>>) -> Result<Self> {
            panic!("stub: java/lang/reflect/Method.<init>:(Ljava/lang/Class;Ljava/lang/String;[Ljava/lang/Class;Ljava/lang/Class;[Ljava/lang/Class;IILjava/lang/String;[B[B[B)V")
        }

        #[java_method(name = "copy", descriptor = "()Ljava/lang/reflect/Method;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copy(&self) -> Result<Method> {
            panic!("stub: java/lang/reflect/Method.copy:()Ljava/lang/reflect/Method;")
        }

        #[java_method(name = "leafCopy", descriptor = "()Ljava/lang/reflect/Method;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn leafCopy(&self) -> Result<Method> {
            panic!("stub: java/lang/reflect/Method.leafCopy:()Ljava/lang/reflect/Method;")
        }

        #[java_method(name = "setAccessible", descriptor = "(Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setAccessible(&self, flag: bool) -> Result<()> {
            panic!("stub: java/lang/reflect/Method.setAccessible:(Z)V")
        }

        #[java_method(name = "checkCanSetAccessible", descriptor = "(Ljava/lang/Class;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<*>;)V")]
        pub fn checkCanSetAccessible(&self, caller: Object) -> Result<()> {
            panic!("stub: java/lang/reflect/Method.checkCanSetAccessible:(Ljava/lang/Class;)V")
        }

        #[java_method(name = "getRoot", descriptor = "()Ljava/lang/reflect/Method;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getRoot(&self) -> Result<Method> {
            panic!("stub: java/lang/reflect/Method.getRoot:()Ljava/lang/reflect/Method;")
        }

        #[java_method(name = "hasGenericInformation", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasGenericInformation(&self) -> Result<bool> {
            panic!("stub: java/lang/reflect/Method.hasGenericInformation:()Z")
        }

        #[java_method(name = "getAnnotationBytes", descriptor = "()[B", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAnnotationBytes(&self) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/lang/reflect/Method.getAnnotationBytes:()[B")
        }

        #[java_method(name = "getDeclaringClass", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;")]
        pub fn getDeclaringClass(&self) -> Result<Object> {
            let this = self;
            Ok(Object::from_any(this.__get_clazz().clone()))
        }

        #[java_method(name = "getName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getName(&self) -> Result<String> {
            let this = self;
            Ok(this.__get_name())
        }

        #[java_method(name = "getModifiers", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getModifiers(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_modifiers())
        }

        #[java_method(name = "getTypeParameters", descriptor = "()[Ljava/lang/reflect/TypeVariable;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/reflect/TypeVariable<Ljava/lang/reflect/Method;>;")]
        pub fn getTypeParameters(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Method.getTypeParameters:()[Ljava/lang/reflect/TypeVariable;")
        }

        #[java_method(name = "getReturnType", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<*>;")]
        pub fn getReturnType(&self) -> Result<Object> {
            let this = self;
            Ok(Object::from_any(this.__get_returnType().clone()))
        }

        #[java_method(name = "getGenericReturnType", descriptor = "()Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getGenericReturnType(&self) -> Result<Object> {
            panic!("stub: java/lang/reflect/Method.getGenericReturnType:()Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "getSharedParameterTypes", descriptor = "()[Ljava/lang/Class;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
        pub fn getSharedParameterTypes(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Method.getSharedParameterTypes:()[Ljava/lang/Class;")
        }

        #[java_method(name = "getSharedExceptionTypes", descriptor = "()[Ljava/lang/Class;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
        pub fn getSharedExceptionTypes(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Method.getSharedExceptionTypes:()[Ljava/lang/Class;")
        }

        #[java_method(name = "getParameterTypes", descriptor = "()[Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
        pub fn getParameterTypes(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Method.getParameterTypes:()[Ljava/lang/Class;")
        }

        #[java_method(name = "getParameterCount", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getParameterCount(&self) -> Result<i32> {
            panic!("stub: java/lang/reflect/Method.getParameterCount:()I")
        }

        #[java_method(name = "getGenericParameterTypes", descriptor = "()[Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getGenericParameterTypes(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Method.getGenericParameterTypes:()[Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "getExceptionTypes", descriptor = "()[Ljava/lang/Class;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()[Ljava/lang/Class<*>;")]
        pub fn getExceptionTypes(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Method.getExceptionTypes:()[Ljava/lang/Class;")
        }

        #[java_method(name = "getGenericExceptionTypes", descriptor = "()[Ljava/lang/reflect/Type;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getGenericExceptionTypes(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Method.getGenericExceptionTypes:()[Ljava/lang/reflect/Type;")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/lang/reflect/Method.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "specificToStringHeader", descriptor = "(Ljava/lang/StringBuilder;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn specificToStringHeader(&self, sb: StringBuilder) -> Result<()> {
            panic!("stub: java/lang/reflect/Method.specificToStringHeader:(Ljava/lang/StringBuilder;)V")
        }

        #[java_method(name = "toShortString", descriptor = "()Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toShortString(&self) -> Result<String> {
            panic!("stub: java/lang/reflect/Method.toShortString:()Ljava/lang/String;")
        }

        #[java_method(name = "toShortSignature", descriptor = "()Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toShortSignature(&self) -> Result<String> {
            panic!("stub: java/lang/reflect/Method.toShortSignature:()Ljava/lang/String;")
        }

        #[java_method(name = "toGenericString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toGenericString(&self) -> Result<String> {
            panic!("stub: java/lang/reflect/Method.toGenericString:()Ljava/lang/String;")
        }

        #[java_method(name = "specificToGenericStringHeader", descriptor = "(Ljava/lang/StringBuilder;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn specificToGenericStringHeader(&self, sb: StringBuilder) -> Result<()> {
            panic!("stub: java/lang/reflect/Method.specificToGenericStringHeader:(Ljava/lang/StringBuilder;)V")
        }

        #[java_method(name = "invoke", descriptor = "(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalAccessException,java/lang/reflect/InvocationTargetException")]
        // java: invoke(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;
        pub fn invoke_obj_arr_obj(&self, mut obj: Object, mut args: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
            let this = self;
            let _t0 = this.isCallerSensitive()?;
            let mut callerSensitive = (_t0) as i32;
            let mut caller: Class<Object> = Default::default();
            if (callerSensitive!=0) {
                let _t1: Object = Reflection::getCallerClass()?;
                caller = _t1;
            }
            let _t1: bool = Modifier::isStatic(this.__get_modifiers())?;
            let mut _merged3: Object;
            if _t1 {
                _merged3 = Object::default();
            } else {
                let _vdispatch2: Object = if let Some(__f) = obj.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                _merged3 = _vdispatch2;
            }
            this.__super().__super().checkAccess(Object::from_any(caller.clone()), Object::from_any(this.__get_clazz().clone()), Clone::clone(&_merged3), this.__get_modifiers())?;
            let mut ma = this.__get_methodAccessor();
            if _is_jnull(&ma) {
                let _t4 = this.acquireMethodAccessor()?;
                ma = _t4;
            }
            let mut _merged5: Object;
            if (callerSensitive!=0) {
                let _vdispatch4: Object = if let Some(__f) = ma.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object, Rc<RefCell<Vec<Object>>>, Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&obj), Clone::clone(&args), Clone::clone(&caller))? } else { Default::default() };
                _merged5 = _vdispatch4;
            } else {
                let _vdispatch4: Object = if let Some(__f) = ma.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object, Rc<RefCell<Vec<Object>>>) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&obj), Clone::clone(&args))? } else { Default::default() };
                _merged5 = _vdispatch4;
            }
            Ok(_merged5)
        }

        #[java_method(name = "invoke", descriptor = "(Ljava/lang/Object;[Ljava/lang/Object;Ljava/lang/Class;)Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/IllegalAccessException,java/lang/reflect/InvocationTargetException", generic_signature = "(Ljava/lang/Object;[Ljava/lang/Object;Ljava/lang/Class<*>;)Ljava/lang/Object;")]
        pub fn invoke_obj_arr_obj_class(&self, obj: Object, args: Rc<RefCell<Vec<Object>>>, caller: Object) -> Result<Object> {
            panic!("stub: java/lang/reflect/Method.invoke:(Ljava/lang/Object;[Ljava/lang/Object;Ljava/lang/Class;)Ljava/lang/Object;")
        }

        #[java_method(name = "isCallerSensitive", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isCallerSensitive(&self) -> Result<bool> {
            let this = self;
            let mut cs = this.__get_callerSensitive();
            let _t0: bool = Reflection::isCallerSensitive(Clone::clone(this))?;
            let mut cs = ((!(_t0)) as i8 as i32);
            this.__set_callerSensitive(((((!(_t0)) as i8 as i32)) as i8));
            Ok((cs>0))
        }

        #[java_method(name = "isBridge", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isBridge(&self) -> Result<bool> {
            panic!("stub: java/lang/reflect/Method.isBridge:()Z")
        }

        #[java_method(name = "isVarArgs", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isVarArgs(&self) -> Result<bool> {
            panic!("stub: java/lang/reflect/Method.isVarArgs:()Z")
        }

        #[java_method(name = "isSynthetic", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSynthetic(&self) -> Result<bool> {
            panic!("stub: java/lang/reflect/Method.isSynthetic:()Z")
        }

        #[java_method(name = "isDefault", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDefault(&self) -> Result<bool> {
            panic!("stub: java/lang/reflect/Method.isDefault:()Z")
        }

        #[java_method(name = "acquireMethodAccessor", descriptor = "()Ljdk/internal/reflect/MethodAccessor;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn acquireMethodAccessor(&self) -> Result<Object> {
            let this = self;
            let mut root = this.__get_root();
            let mut _merged1: Object;
            if _is_jnull(&root) {
                _merged1 = Object::default();
            } else {
                let _t0 = root.getMethodAccessor()?;
                _merged1 = _t0;
            }
            let mut tmp: Object = _merged1;
            if !_is_jnull(&tmp) {
                this.__set_methodAccessor(Clone::clone(&tmp));
            } else {
                let _t2 = this.isCallerSensitive()?;
                let _t3 = Method::reflectionFactory().newMethodAccessor(Clone::clone(this), _t2)?;
                tmp = _t3;
                let _t4: bool = VM::isJavaLangInvokeInited()?;
                if _t4 {
                    this.setMethodAccessor(Clone::clone(&tmp))?;
                }
            }
            Ok(tmp)
        }

        #[java_method(name = "getMethodAccessor", descriptor = "()Ljdk/internal/reflect/MethodAccessor;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMethodAccessor(&self) -> Result<Object> {
            let this = self;
            Ok(this.__get_methodAccessor())
        }

        #[java_method(name = "setMethodAccessor", descriptor = "(Ljdk/internal/reflect/MethodAccessor;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMethodAccessor(&self, mut accessor: Object) -> Result<()> {
            let this = self;
            this.__set_methodAccessor(Clone::clone(&accessor));
            let mut root = this.__get_root();
            if !_is_jnull(&root) {
                root.setMethodAccessor(Clone::clone(&accessor))?;
            }
            Ok(())
        }

        #[java_method(name = "getDefaultValue", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDefaultValue(&self) -> Result<Object> {
            panic!("stub: java/lang/reflect/Method.getDefaultValue:()Ljava/lang/Object;")
        }

        #[java_method(name = "getAnnotation", descriptor = "(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T::Ljava/lang/annotation/Annotation;>(Ljava/lang/Class<TT;>;)TT;")]
        pub fn getAnnotation(&self, annotationClass: Object) -> Result<Object> {
            panic!("stub: java/lang/reflect/Method.getAnnotation:(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "getDeclaredAnnotations", descriptor = "()[Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDeclaredAnnotations(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/lang/reflect/Method.getDeclaredAnnotations:()[Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "getParameterAnnotations", descriptor = "()[[Ljava/lang/annotation/Annotation;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getParameterAnnotations(&self) -> Result<Rc<RefCell<Vec<Rc<RefCell<Vec<Object>>>>>>> {
            panic!("stub: java/lang/reflect/Method.getParameterAnnotations:()[[Ljava/lang/annotation/Annotation;")
        }

        #[java_method(name = "getAnnotatedReturnType", descriptor = "()Ljava/lang/reflect/AnnotatedType;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAnnotatedReturnType(&self) -> Result<Object> {
            panic!("stub: java/lang/reflect/Method.getAnnotatedReturnType:()Ljava/lang/reflect/AnnotatedType;")
        }

        #[java_method(name = "handleParameterNumberMismatch", descriptor = "(I[Ljava/lang/Class;)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I[Ljava/lang/Class<*>;)Z")]
        pub fn handleParameterNumberMismatch(&self, resultLength: i32, parameterTypes: Rc<RefCell<Vec<Object>>>) -> Result<bool> {
            panic!("stub: java/lang/reflect/Method.handleParameterNumberMismatch:(I[Ljava/lang/Class;)Z")
        }
    }
}
