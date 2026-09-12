#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Class",
    super_class = "java/lang/Object",
    interfaces  = "java/io/Serializable,java/lang/reflect/GenericDeclaration,java/lang/reflect/Type,java/lang/reflect/AnnotatedElement,java/lang/invoke/TypeDescriptor$OfField,java/lang/constant/Constable",
    access      = "public final",
    source      = "Class.java",
))]
pub struct Class<T> {
    #[cfg_attr(any(), java_field(name = "cachedConstructor", descriptor = "Ljava/lang/reflect/Constructor;", access = "private"))]
    pub cachedConstructor: Field<Object>,
    #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private"))]
    pub name: Field<String>,
    #[cfg_attr(any(), java_field(name = "module", descriptor = "Ljava/lang/Module;", access = "private"))]
    pub module: Field<Object>,
    #[cfg_attr(any(), java_field(name = "classLoader", descriptor = "Ljava/lang/ClassLoader;", access = "private final"))]
    pub classLoader: Field<Object>,
    #[cfg_attr(any(), java_field(name = "classData", descriptor = "Ljava/lang/Object;", access = "private"))]
    pub classData: Field<Object>,
    #[cfg_attr(any(), java_field(name = "packageName", descriptor = "Ljava/lang/String;", access = "private"))]
    pub packageName: Field<String>,
    #[cfg_attr(any(), java_field(name = "componentType", descriptor = "Ljava/lang/Class;", access = "private final"))]
    pub componentType: Field<Object>,
    #[cfg_attr(any(), java_field(name = "reflectionData", descriptor = "Ljava/lang/ref/SoftReference;", access = "private"))]
    pub reflectionData: Field<Object>,
    #[cfg_attr(any(), java_field(name = "classRedefinedCount", descriptor = "I", access = "private"))]
    pub classRedefinedCount: Field<i32>,
    #[cfg_attr(any(), java_field(name = "genericInfo", descriptor = "Lsun/reflect/generics/repository/ClassRepository;", access = "private"))]
    pub genericInfo: Field<Object>,
    #[cfg_attr(any(), java_field(name = "enumConstants", descriptor = "[Ljava/lang/Object;", access = "private"))]
    pub enumConstants: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "enumConstantDirectory", descriptor = "Ljava/util/Map;", access = "private"))]
    pub enumConstantDirectory: Field<Object>,
    #[cfg_attr(any(), java_field(name = "annotationData", descriptor = "Ljava/lang/Class$AnnotationData;", access = "private"))]
    pub annotationData: Field<Object>,
    #[cfg_attr(any(), java_field(name = "annotationType", descriptor = "Lsun/reflect/annotation/AnnotationType;", access = "private"))]
    pub annotationType: Field<Object>,
    #[cfg_attr(any(), java_field(name = "classValueMap", descriptor = "Ljava/lang/ClassValue$ClassValueMap;", access = ""))]
    pub classValueMap: Field<Object>,
}

impl<T: Clone + 'static> Class<T> {
    #[cfg_attr(any(), java_native(name = "registerNatives", descriptor = "()V", access = "private static native"))]
    pub fn registerNatives() -> Result<()> {
        todo!("native java/lang/Class.registerNatives")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/ClassLoader;Ljava/lang/Class;)V", access = "private"))]
    pub fn new(loader: Object, arrayComponentType: Object) -> Result<Self> {
        let this = Self { cachedConstructor: Field::new(Default::default()), name: Field::new(String::new()), module: Field::new(Default::default()), classLoader: Field::new(Default::default()), classData: Field::new(Default::default()), packageName: Field::new(String::new()), componentType: Field::new(Default::default()), reflectionData: Field::new(Default::default()), classRedefinedCount: Field::new(0), genericInfo: Field::new(Default::default()), enumConstants: Field::new(Default::default()), enumConstantDirectory: Field::new(Default::default()), annotationData: Field::new(Default::default()), annotationType: Field::new(Default::default()), classValueMap: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.classLoader.set(loader);
        this.componentType.set(arrayComponentType);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.isInterface()?;
        let _t1 = this.isPrimitive()?;
        let mut kind: String = String::from("class");
        let _t2 = this.getName()?;
        let _t3 = kind.concat(_t2)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "toGenericString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toGenericString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.isPrimitive()?;
        let _t1 = this.toString()?;
        return Ok(_t1);
        let mut sb: String = String::new();
        let mut component: java/lang/Class = this;
        let mut arrayDepth: i32 = 0i32;
        let _t2 = this.isArray()?;
        arrayDepth = arrayDepth.wrapping_add(1i32);
        let _t3 = component.getComponentType()?;
        component = _t3;
        let _t4 = component.isArray()?;
        let _t5 = component.getName()?;
        sb.append(&_t5)?;
        let _t6 = this.getModifiers()?;
        let _t7: i32 = Modifier::classModifiers()?;
        let mut modifiers: i32 = (_t6&_t7);
        let _t8: String = Modifier::toString(modifiers)?;
        sb.append(&_t8)?;
        sb.append(&32i32)?;
        let _t9 = this.isAnnotation()?;
        sb.append(&64i32)?;
        let _t10 = this.isInterface()?;
        sb.append(&String::from("interface"))?;
        let _t11 = this.isEnum()?;
        sb.append(&String::from("enum"))?;
        let _t12 = this.isRecord()?;
        sb.append(&String::from("record"))?;
        sb.append(&String::from("class"))?;
        sb.append(&32i32)?;
        let _t13 = this.getName()?;
        sb.append(&_t13)?;
        let _t14 = component.getTypeParameters()?;
        modifiers = _t14;
        let _t15: Object = Arrays::stream(modifiers)?;
        /* TODO: invokedynamic 99 */
        let _t16 = sb.map(_t15)?;
        let _t17: Object = Collectors::joining(String::from(","), String::from("<"), String::from(">"))?;
        let _t18 = _t16.collect(_t17)?;
        (modifiers.len() as i32).append(&_t18)?;
        let _ = (modifiers.len() as i32);
        let _t19 = String::from("[]").repeat(arrayDepth)?;
        sb.append(&_t19)?;
        Ok(sb)
    }

    #[cfg_attr(any(), java_method(name = "typeVarBounds", descriptor = "(Ljava/lang/reflect/TypeVariable;)Ljava/lang/String;", access = "static"))]
    pub fn typeVarBounds(typeVar: Object) -> Result<String> {
        let _t0 = typeVar.getBounds()?;
        let mut bounds: Vec<Object> = _t0;
        let _t1 = bounds[0i32 as usize].clone().equals(2i32)?;
        let _t2 = typeVar.getName()?;
        return Ok(_t2);
        let _t3 = typeVar.getName()?;
        String::new().append(&_t3)?;
        String::new().append(&String::from("extends"))?;
        let _t4: Object = Arrays::stream(&bounds)?;
        /* TODO: invokedynamic 146 */
        let _t5 = String::new().map(_t4)?;
        let _t6: Object = Collectors::joining(String::from("&"))?;
        let _t7 = _t5.collect(_t6)?;
        _t1.append(&_t7)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "forName", descriptor = "(Ljava/lang/String;)Ljava/lang/Class;", access = "public static"))]
    // java: forName(Ljava/lang/String;)Ljava/lang/Class;
    pub fn forName__str(className: String) -> Result<Object> {
        let _t0: Object = Reflection::getCallerClass()?;
        let mut caller: Object = _t0;
        let _t1: Object = Class::forName(className, caller)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "forName", descriptor = "(Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Class;", access = "private static"))]
    // java: forName(Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Class;
    pub fn forName__str_class(className: String, caller: Object) -> Result<Object> {
        let _t0: Object = ClassLoader::getSystemClassLoader()?;
        let _t1: Object = ClassLoader::getClassLoader(caller)?;
        let mut loader: Object = _t1;
        let _t2: Object = Class::forName0(className, 1i32, loader, caller)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "forName", descriptor = "(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;", access = "public static"))]
    // java: forName(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;
    pub fn forName__str_z_classl(name: String, initialize: bool, loader: Object) -> Result<Object> {
        /* TODO: aconst_null  */
        let mut caller: i32 = todo!("stack underflow");
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        let _t1: Object = Reflection::getCallerClass()?;
        caller = _t1;
        let _t2: Object = Class::forName(name, initialize, loader, caller)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "forName", descriptor = "(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;", access = "private static"))]
    // java: forName(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;
    pub fn forName__str_z_classl_class(name: String, initialize: bool, loader: Object, caller: Object) -> Result<Object> {
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        let _t1: Object = ClassLoader::getClassLoader(caller)?;
        let mut ccl: Object = _t1;
        sm.checkPermission(SecurityConstants::GET_CLASSLOADER_PERMISSION())?;
        let _t2: Object = Class::forName0(name, initialize, loader, caller)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_native(name = "forName0", descriptor = "(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;", access = "private static native"))]
    pub fn forName0(arg0: String, arg1: bool, arg2: Object, arg3: Object) -> Result<Object> {
        todo!("native java/lang/Class.forName0")
    }

    #[cfg_attr(any(), java_method(name = "forName", descriptor = "(Ljava/lang/Module;Ljava/lang/String;)Ljava/lang/Class;", access = "public static"))]
    // java: forName(Ljava/lang/Module;Ljava/lang/String;)Ljava/lang/Class;
    pub fn forName__module_str(module: Object, name: String) -> Result<Object> {
        /* TODO: aconst_null  */
        let mut caller: i32 = todo!("stack underflow");
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        let _t1: Object = Reflection::getCallerClass()?;
        caller = _t1;
        let _t2: Object = Class::forName(module, name, caller)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "forName", descriptor = "(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Class;", access = "private static"))]
    // java: forName(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Class;
    pub fn forName__module_str_class(module: Object, name: String, caller: Object) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(module)?;
        let _t1: Object = Objects::requireNonNull(name)?;
        let _t2: Object = System::getSecurityManager()?;
        let mut sm: Object = _t2;
        let _t3 = caller.getModule()?;
        sm.checkPermission(SecurityConstants::GET_CLASSLOADER_PERMISSION())?;
        let _t4: Object = Objects::requireNonNull(module)?;
        /* TODO: invokedynamic 208 */
        let mut pa: Object = module;
        let _t5: Object = AccessController::doPrivileged(pa)?;
        let mut cl: Object = _t5;
        let _t6 = module.getClassLoader()?;
        cl = _t6;
        let _t7 = cl.loadClass(module, name)?;
        return Ok(_t7);
        let _t8: Object = BootLoader::loadClass(module, name)?;
        Ok(_t8)
    }

    #[cfg_attr(any(), java_method(name = "newInstance", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn newInstance(&self) -> Result<T> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        let _t1: Object = Reflection::getCallerClass()?;
        this.checkMemberAccess(sm, 0i32, _t1, 0i32)?;
        let mut tmpConstructor: Object = this.cachedConstructor.get();
        return Err(JvmError::Custom(String::from("athrow")));
        let mut _arr2: Vec<Object> = Vec::with_capacity(0i32 as usize);
        let mut empty: Vec<Object> = _arr2;
        let _t3: Object = Class::getReflectionFactory()?;
        let _t4 = this.getConstructor0(empty, 1i32)?;
        let _t5 = _t3.copyConstructor(_t4)?;
        let mut c: Object = _t5;
        let _t6: Object = AccessController::doPrivileged(Class_1::new(this, c)?)?;
        tmpConstructor = c;
        this.cachedConstructor.set(c);
        empty = 8i32;
        let _t7 = this.getName()?;
        let _t8 = InstantiationException::new(_t7)?.initCause(empty)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t9: Object = Reflection::getCallerClass()?;
        empty = _t9;
        let _t10: Object = Class::getReflectionFactory()?;
        /* TODO: aconst_null  */
        let _t11 = this.newInstance(_t10, tmpConstructor, empty)?;
        return Ok(_t11);
        empty = tmpConstructor;
        let _t12: Object = Unsafe::getUnsafe()?;
        let _t13 = empty.getTargetException()?;
        _t12.throwException(_t13)?;
        /* TODO: aconst_null  */
        Ok(sm)
    }

    #[cfg_attr(any(), java_native(name = "isInstance", descriptor = "(Ljava/lang/Object;)Z", access = "public native"))]
    pub fn isInstance(&self, arg0: Object) -> Result<bool> {
        todo!("native java/lang/Class.isInstance")
    }

    #[cfg_attr(any(), java_native(name = "isAssignableFrom", descriptor = "(Ljava/lang/Class;)Z", access = "public native"))]
    pub fn isAssignableFrom(&self, arg0: Object) -> Result<bool> {
        todo!("native java/lang/Class.isAssignableFrom")
    }

    #[cfg_attr(any(), java_native(name = "isInterface", descriptor = "()Z", access = "public native"))]
    pub fn isInterface(&self) -> Result<bool> {
        todo!("native java/lang/Class.isInterface")
    }

    #[cfg_attr(any(), java_native(name = "isArray", descriptor = "()Z", access = "public native"))]
    pub fn isArray(&self) -> Result<bool> {
        todo!("native java/lang/Class.isArray")
    }

    #[cfg_attr(any(), java_native(name = "isPrimitive", descriptor = "()Z", access = "public native"))]
    pub fn isPrimitive(&self) -> Result<bool> {
        todo!("native java/lang/Class.isPrimitive")
    }

    #[cfg_attr(any(), java_method(name = "isAnnotation", descriptor = "()Z", access = "public"))]
    pub fn isAnnotation(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.getModifiers()?;
        Ok((_t0&8192i32)!=0i32)
    }

    #[cfg_attr(any(), java_method(name = "isSynthetic", descriptor = "()Z", access = "public"))]
    pub fn isSynthetic(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.getModifiers()?;
        Ok((_t0&4096i32)!=0i32)
    }

    #[cfg_attr(any(), java_method(name = "getName", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getName(&self) -> Result<String> {
        let this = self;
        let mut name: String = this.name.get();
        let _t0 = this.initClassName()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_native(name = "initClassName", descriptor = "()Ljava/lang/String;", access = "private native"))]
    pub fn initClassName(&self) -> Result<String> {
        todo!("native java/lang/Class.initClassName")
    }

    #[cfg_attr(any(), java_method(name = "getClassLoader", descriptor = "()Ljava/lang/ClassLoader;", access = "public"))]
    pub fn getClassLoader(&self) -> Result<Object> {
        let this = self;
        let mut cl: Object = this.classLoader.get();
        /* TODO: aconst_null  */
        return Ok(cl);
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        let _t1: Object = Reflection::getCallerClass()?;
        ClassLoader::checkClassLoaderPermission(cl, _t1)?;
        Ok(cl)
    }

    #[cfg_attr(any(), java_method(name = "getClassLoader0", descriptor = "()Ljava/lang/ClassLoader;"))]
    pub fn getClassLoader0(&self) -> Result<Object> {
        let this = self;
        Ok(this.classLoader.get())
    }

    #[cfg_attr(any(), java_method(name = "getModule", descriptor = "()Ljava/lang/Module;", access = "public"))]
    pub fn getModule(&self) -> Result<Object> {
        let this = self;
        Ok(this.module.get())
    }

    #[cfg_attr(any(), java_method(name = "getClassData", descriptor = "()Ljava/lang/Object;"))]
    pub fn getClassData(&self) -> Result<Object> {
        let this = self;
        Ok(this.classData.get())
    }

    #[cfg_attr(any(), java_method(name = "getTypeParameters", descriptor = "()[Ljava/lang/reflect/TypeVariable;", access = "public"))]
    pub fn getTypeParameters(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.getGenericInfo()?;
        let mut info: Object = _t0;
        let _t1 = info.getTypeParameters()?;
        return Ok(_t1);
        let mut _arr2: Vec<Object> = Vec::with_capacity(0i32 as usize);
        Ok(_arr2)
    }

    #[cfg_attr(any(), java_native(name = "getSuperclass", descriptor = "()Ljava/lang/Class;", access = "public native"))]
    pub fn getSuperclass(&self) -> Result<Object> {
        todo!("native java/lang/Class.getSuperclass")
    }

    #[cfg_attr(any(), java_method(name = "getGenericSuperclass", descriptor = "()Ljava/lang/reflect/Type;", access = "public"))]
    pub fn getGenericSuperclass(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.getGenericInfo()?;
        let mut info: Object = _t0;
        let _t1 = this.getSuperclass()?;
        return Ok(_t1);
        let _t2 = this.isInterface()?;
        /* TODO: aconst_null  */
        return Ok(_t2);
        let _t3 = info.getSuperclass()?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "getPackage", descriptor = "()Ljava/lang/Package;", access = "public"))]
    pub fn getPackage(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.isPrimitive()?;
        let _t1 = this.isArray()?;
        /* TODO: aconst_null  */
        return Ok(_t1);
        let mut cl: Object = this.classLoader.get();
        let _t2 = cl.definePackage(this)?;
        let _t3: Object = BootLoader::definePackage(this)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "getPackageName", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getPackageName(&self) -> Result<String> {
        let this = self;
        let mut pn: String = this.packageName.get();
        let _t0 = this.isArray()?;
        let _t1 = this.elementType()?;
        let mut c: java/lang/Class = this;
        let _t2 = c.isPrimitive()?;
        pn = String::from("java.lang");
        let _t3 = c.getName()?;
        let mut cn: String = _t3;
        let _t4 = cn.lastIndexOf(46i32)?;
        let mut dot: i32 = _t4;
        let _t5 = cn.substring(0i32, dot)?;
        let _t6 = _t5.intern()?;
        pn = String::from("");
        this.packageName.set(pn);
        Ok(pn)
    }

    #[cfg_attr(any(), java_method(name = "getInterfaces", descriptor = "()[Ljava/lang/Class;", access = "public"))]
    // java: getInterfaces()[Ljava/lang/Class;
    pub fn getInterfaces(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.getInterfaces(1i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getInterfaces", descriptor = "(Z)[Ljava/lang/Class;", access = "private"))]
    // java: getInterfaces(Z)[Ljava/lang/Class;
    pub fn getInterfaces__z(&self, cloneArray: bool) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.reflectionData()?;
        let mut rd: Object = _t0;
        let _t1 = this.getInterfaces0()?;
        return Ok(_t1);
        let mut interfaces: Vec<Object> = rd.interfaces.get();
        let _t2 = this.getInterfaces0()?;
        interfaces = _t2;
        rd.interfaces.set(interfaces);
        let _t3 = interfaces.clone()?;
        Ok(interfaces)
    }

    #[cfg_attr(any(), java_native(name = "getInterfaces0", descriptor = "()[Ljava/lang/Class;", access = "private native"))]
    pub fn getInterfaces0(&self) -> Result<Vec<Object>> {
        todo!("native java/lang/Class.getInterfaces0")
    }

    #[cfg_attr(any(), java_method(name = "getGenericInterfaces", descriptor = "()[Ljava/lang/reflect/Type;", access = "public"))]
    pub fn getGenericInterfaces(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.getGenericInfo()?;
        let mut info: Object = _t0;
        let _t1 = this.getInterfaces()?;
        let _t2 = info.getSuperInterfaces()?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "getComponentType", descriptor = "()Ljava/lang/Class;", access = "public"))]
    pub fn getComponentType(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.isArray()?;
        return Ok(this.componentType.get());
        /* TODO: aconst_null  */
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "elementType", descriptor = "()Ljava/lang/Class;", access = "private"))]
    pub fn elementType(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.isArray()?;
        /* TODO: aconst_null  */
        return Ok(_t0);
        let mut c: java/lang/Class = this;
        loop {
            let _t0 = c.isArray()?;
            if _t0==0i32 { break; }
            let _t0 = c.getComponentType()?;
            c = _t0;
        }
        Ok(c)
    }

    #[cfg_attr(any(), java_native(name = "getModifiers", descriptor = "()I", access = "public native"))]
    pub fn getModifiers(&self) -> Result<i32> {
        todo!("native java/lang/Class.getModifiers")
    }

    #[cfg_attr(any(), java_method(name = "accessFlags", descriptor = "()Ljava/util/Set;", access = "public"))]
    pub fn accessFlags(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.isMemberClass()?;
        let _t1 = this.isLocalClass()?;
        let _t2 = this.isAnonymousClass()?;
        let _t3 = this.isArray()?;
        let mut location: Object = AccessFlag$Location::CLASS();
        let _t4 = this.getClassAccessFlagsRaw()?;
        let _t5 = this.getModifiers()?;
        let _t6: Object = AccessFlag::maskToAccessFlags(_t5, location)?;
        Ok(_t6)
    }

    #[cfg_attr(any(), java_native(name = "getSigners", descriptor = "()[Ljava/lang/Object;", access = "public native"))]
    pub fn getSigners(&self) -> Result<Vec<Object>> {
        todo!("native java/lang/Class.getSigners")
    }

    #[cfg_attr(any(), java_native(name = "setSigners", descriptor = "([Ljava/lang/Object;)V", access = "native"))]
    pub fn setSigners(&self, arg0: Vec<Object>) -> Result<()> {
        todo!("native java/lang/Class.setSigners")
    }

    #[cfg_attr(any(), java_method(name = "getEnclosingMethod", descriptor = "()Ljava/lang/reflect/Method;", access = "public"))]
    pub fn getEnclosingMethod(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.getEnclosingMethodInfo()?;
        let mut enclosingInfo: Object = _t0;
        /* TODO: aconst_null  */
        return Ok(enclosingInfo);
        let _t1 = enclosingInfo.isMethod()?;
        /* TODO: aconst_null  */
        return Ok(_t1);
        let _t2 = enclosingInfo.getDescriptor()?;
        let _t3 = this.getFactory()?;
        let _t4: Object = MethodRepository::make(_t2, _t3)?;
        let mut typeInfo: Object = _t4;
        let _t5 = typeInfo.getReturnType()?;
        let _t6: Object = Class::toClass(_t5)?;
        let mut returnType: Object = _t6;
        let _t7 = typeInfo.getParameterTypes()?;
        let mut parameterTypes: Vec<Object> = _t7;
        let mut _arr8: Vec<Object> = Vec::with_capacity((parameterTypes.len() as i32) as usize);
        let mut parameterClasses: Vec<Object> = _arr8;
        let mut i: i32 = 0i32;
        loop {
            if i >= (parameterClasses.len() as i32) { break; }
            let _t0: Object = Class::toClass(parameterTypes[i as usize].clone())?;
            parameterClasses[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        let _t9 = enclosingInfo.getEnclosingClass()?;
        i = _t9;
        let _t10: Object = System::getSecurityManager()?;
        let mut sm: Object = _t10;
        let _t11: Object = Reflection::getCallerClass()?;
        i.checkMemberAccess(sm, 1i32, _t11, 1i32)?;
        let _t12 = i.privateGetDeclaredMethods(0i32)?;
        let mut candidates: Vec<Object> = _t12;
        let _t13: Object = Class::getReflectionFactory()?;
        let mut fact: Object = _t13;
        let mut local_10: Vec<Object> = candidates;
        let mut local_11: i32 = (local_10.len() as i32);
        let mut local_12: i32 = 0i32;
        loop {
            if local_12 >= local_11 { break; }
            let mut m: Object = local_10[local_12 as usize].clone();
            let _t0 = m.getName()?;
            let _t1 = enclosingInfo.getName()?;
            let _t2 = _t0.equals(_t1)?;
            let _t3 = fact.getExecutableSharedParameterTypes(m)?;
            let _t4: bool = Class::arrayContentsEq(&parameterClasses, &_t3)?;
            let _t5 = m.getReturnType()?;
            let _t6 = _t5.equals(returnType)?;
            let _t7 = fact.copyMethod(m)?;
            return Ok(_t7);
            local_12 = local_12.wrapping_add(1i32);
        }
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_native(name = "getEnclosingMethod0", descriptor = "()[Ljava/lang/Object;", access = "private native"))]
    pub fn getEnclosingMethod0(&self) -> Result<Vec<Object>> {
        todo!("native java/lang/Class.getEnclosingMethod0")
    }

    #[cfg_attr(any(), java_method(name = "getEnclosingMethodInfo", descriptor = "()Ljava/lang/Class$EnclosingMethodInfo;", access = "private"))]
    pub fn getEnclosingMethodInfo(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.getEnclosingMethod0()?;
        let mut enclosingInfo: Vec<Object> = _t0;
        /* TODO: aconst_null  */
        return Ok(enclosingInfo);
        Ok(Class_EnclosingMethodInfo::new(enclosingInfo)?)
    }

    #[cfg_attr(any(), java_method(name = "toClass", descriptor = "(Ljava/lang/reflect/Type;)Ljava/lang/Class;", access = "private static"))]
    pub fn toClass(o: Object) -> Result<Object> {
        let _t0 = o.getGenericComponentType()?;
        let _t1: Object = Class::toClass(_t0)?;
        let _t2: Object = Array::newInstance(_t1, 0i32)?;
        let _t3 = _t2.getClass()?;
        return Ok(_t3);
        Ok(o)
    }

    #[cfg_attr(any(), java_method(name = "getEnclosingConstructor", descriptor = "()Ljava/lang/reflect/Constructor;", access = "public"))]
    pub fn getEnclosingConstructor(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.getEnclosingMethodInfo()?;
        let mut enclosingInfo: Object = _t0;
        /* TODO: aconst_null  */
        return Ok(enclosingInfo);
        let _t1 = enclosingInfo.isConstructor()?;
        /* TODO: aconst_null  */
        return Ok(_t1);
        let _t2 = enclosingInfo.getDescriptor()?;
        let _t3 = this.getFactory()?;
        let _t4: Object = ConstructorRepository::make(_t2, _t3)?;
        let mut typeInfo: Object = _t4;
        let _t5 = typeInfo.getParameterTypes()?;
        let mut parameterTypes: Vec<Object> = _t5;
        let mut _arr6: Vec<Object> = Vec::with_capacity((parameterTypes.len() as i32) as usize);
        let mut parameterClasses: Vec<Object> = _arr6;
        let mut i: i32 = 0i32;
        loop {
            if i >= (parameterClasses.len() as i32) { break; }
            let _t0: Object = Class::toClass(parameterTypes[i as usize].clone())?;
            parameterClasses[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        let _t7 = enclosingInfo.getEnclosingClass()?;
        i = _t7;
        let _t8: Object = System::getSecurityManager()?;
        let mut sm: Object = _t8;
        let _t9: Object = Reflection::getCallerClass()?;
        i.checkMemberAccess(sm, 1i32, _t9, 1i32)?;
        let _t10 = i.privateGetDeclaredConstructors(0i32)?;
        let mut candidates: Vec<Object> = _t10;
        let _t11: Object = Class::getReflectionFactory()?;
        let mut fact: Object = _t11;
        let mut local_9: Vec<Object> = candidates;
        let mut local_10: i32 = (local_9.len() as i32);
        let mut local_11: i32 = 0i32;
        loop {
            if local_11 >= local_10 { break; }
            let mut c: Object = local_9[local_11 as usize].clone();
            let _t0 = fact.getExecutableSharedParameterTypes(c)?;
            let _t1: bool = Class::arrayContentsEq(&parameterClasses, &_t0)?;
            let _t2 = fact.copyConstructor(c)?;
            return Ok(_t2);
            local_11 = local_11.wrapping_add(1i32);
        }
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "getDeclaringClass", descriptor = "()Ljava/lang/Class;", access = "public"))]
    pub fn getDeclaringClass(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.getDeclaringClass0()?;
        let mut candidate: Object = _t0;
        let _t1: Object = System::getSecurityManager()?;
        let mut sm: Object = _t1;
        let _t2: Object = Reflection::getCallerClass()?;
        let _t3: Object = ClassLoader::getClassLoader(_t2)?;
        candidate.checkPackageAccess(sm, _t3, 1i32)?;
        Ok(candidate)
    }

    #[cfg_attr(any(), java_native(name = "getDeclaringClass0", descriptor = "()Ljava/lang/Class;", access = "private native"))]
    pub fn getDeclaringClass0(&self) -> Result<Object> {
        todo!("native java/lang/Class.getDeclaringClass0")
    }

    #[cfg_attr(any(), java_method(name = "getEnclosingClass", descriptor = "()Ljava/lang/Class;", access = "public"))]
    pub fn getEnclosingClass(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.getEnclosingMethodInfo()?;
        let mut enclosingInfo: Object = _t0;
        let _t1 = this.getDeclaringClass0()?;
        let mut enclosingCandidate: Object = _t1;
        let _t2 = enclosingInfo.getEnclosingClass()?;
        let mut enclosingClass: Object = _t2;
        return Err(JvmError::Custom(String::from("athrow")));
        enclosingCandidate = enclosingClass;
        let _t3: Object = System::getSecurityManager()?;
        enclosingClass = _t3;
        let _t4: Object = Reflection::getCallerClass()?;
        let _t5: Object = ClassLoader::getClassLoader(_t4)?;
        enclosingCandidate.checkPackageAccess(enclosingClass, _t5, 1i32)?;
        Ok(enclosingCandidate)
    }

    #[cfg_attr(any(), java_method(name = "getSimpleName", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getSimpleName(&self) -> Result<String> {
        let this = self;
        let _t0 = this.isUnnamedClass()?;
        return Ok(String::from(""));
        let _t1 = this.reflectionData()?;
        let mut rd: Object = _t1;
        let mut simpleName: String = rd.simpleName.get();
        let _t2 = this.getSimpleName0()?;
        simpleName = _t2;
        rd.simpleName.set(_t2);
        Ok(simpleName)
    }

    #[cfg_attr(any(), java_method(name = "getSimpleName0", descriptor = "()Ljava/lang/String;", access = "private"))]
    pub fn getSimpleName0(&self) -> Result<String> {
        let this = self;
        let _t0 = this.isArray()?;
        let _t1 = this.getComponentType()?;
        let _t2 = _t1.getSimpleName()?;
        let _t3 = _t2.concat(String::from("[]"))?;
        return Ok(_t3);
        let _t4 = this.getSimpleBinaryName()?;
        let mut simpleName: String = _t4;
        let _t5 = this.getName()?;
        simpleName = _t5;
        let _t6 = simpleName.lastIndexOf(46i32)?;
        let _t7 = simpleName.substring((_t6).wrapping_add(1i32))?;
        simpleName = _t7;
        Ok(simpleName)
    }

    #[cfg_attr(any(), java_method(name = "getTypeName", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getTypeName(&self) -> Result<String> {
        let this = self;
        let _t0 = this.isArray()?;
        let mut cl: java/lang/Class = this;
        let mut dimensions: i32 = 0i32;
        dimensions = dimensions.wrapping_add(1i32);
        let _t1 = cl.getComponentType()?;
        cl = _t1;
        let _t2 = cl.isArray()?;
        let _t3 = cl.getName()?;
        let _t4 = String::from("[]").repeat(dimensions)?;
        let _t5 = _t3.concat(_t4)?;
        return Ok(_t5);
        cl = _t2;
        let _t6 = this.getName()?;
        Ok(_t6)
    }

    #[cfg_attr(any(), java_method(name = "getCanonicalName", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getCanonicalName(&self) -> Result<String> {
        let this = self;
        let _t0 = this.isUnnamedClass()?;
        /* TODO: aconst_null  */
        return Ok(_t0);
        let _t1 = this.reflectionData()?;
        let mut rd: Object = _t1;
        let mut canonicalName: String = rd.canonicalName.get();
        let _t2 = this.getCanonicalName0()?;
        canonicalName = _t2;
        rd.canonicalName.set(_t2);
        /* TODO: aconst_null  */
        Ok(canonicalName)
    }

    #[cfg_attr(any(), java_method(name = "getCanonicalName0", descriptor = "()Ljava/lang/String;", access = "private"))]
    pub fn getCanonicalName0(&self) -> Result<String> {
        let this = self;
        let _t0 = this.isArray()?;
        let _t1 = this.getComponentType()?;
        let _t2 = _t1.getCanonicalName()?;
        let mut canonicalName: String = _t2;
        let _t3 = canonicalName.concat(String::from("[]"))?;
        return Ok(_t3);
        return Ok(Class$ReflectionData::NULL_SENTINEL());
        let _t4 = this.isHidden()?;
        let _t5 = this.isLocalOrAnonymousClass()?;
        return Ok(Class$ReflectionData::NULL_SENTINEL());
        let _t6 = this.getEnclosingClass()?;
        canonicalName = _t6;
        let _t7 = this.getName()?;
        return Ok(_t7);
        let _t8 = canonicalName.getCanonicalName()?;
        let mut enclosingName: String = _t8;
        return Ok(Class$ReflectionData::NULL_SENTINEL());
        let _t9 = this.getSimpleName()?;
        let mut simpleName: String = _t9;
        let _t10 = enclosingName.length()?;
        let _t11 = simpleName.length()?;
        String::new().append(&enclosingName)?;
        String::new().append(&46i32)?;
        String::new().append(&simpleName)?;
        Ok(String::new())
    }

    #[cfg_attr(any(), java_method(name = "isUnnamedClass", descriptor = "()Z", access = "public"))]
    pub fn isUnnamedClass(&self) -> Result<bool> {
        let this = self;
        let _t0: bool = PreviewFeatures::isEnabled()?;
        let _t1 = this.isSynthetic()?;
        let _t2 = this.isTopLevelClass()?;
        let _t3 = this.getModifiers()?;
        let _t4: bool = Modifier::isFinal(_t3)?;
        Ok(_t4!=0i32)
    }

    #[cfg_attr(any(), java_method(name = "isAnonymousClass", descriptor = "()Z", access = "public"))]
    pub fn isAnonymousClass(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.isArray()?;
        let _t1 = this.isLocalOrAnonymousClass()?;
        let _t2 = this.getSimpleBinaryName0()?;
        Ok(_t2.is_none())
    }

    #[cfg_attr(any(), java_method(name = "isLocalClass", descriptor = "()Z", access = "public"))]
    pub fn isLocalClass(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.isLocalOrAnonymousClass()?;
        let _t1 = this.isArray()?;
        let _t2 = this.getSimpleBinaryName0()?;
        Ok(!_t2.is_none())
    }

    #[cfg_attr(any(), java_method(name = "isMemberClass", descriptor = "()Z", access = "public"))]
    pub fn isMemberClass(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.isLocalOrAnonymousClass()?;
        let _t1 = this.getDeclaringClass0()?;
        Ok(!_t1.is_none())
    }

    #[cfg_attr(any(), java_method(name = "getSimpleBinaryName", descriptor = "()Ljava/lang/String;", access = "private"))]
    pub fn getSimpleBinaryName(&self) -> Result<String> {
        let this = self;
        let _t0 = this.isTopLevelClass()?;
        /* TODO: aconst_null  */
        return Ok(_t0);
        let _t1 = this.getSimpleBinaryName0()?;
        let mut name: String = _t1;
        return Ok(String::from(""));
        Ok(name)
    }

    #[cfg_attr(any(), java_native(name = "getSimpleBinaryName0", descriptor = "()Ljava/lang/String;", access = "private native"))]
    pub fn getSimpleBinaryName0(&self) -> Result<String> {
        todo!("native java/lang/Class.getSimpleBinaryName0")
    }

    #[cfg_attr(any(), java_method(name = "isTopLevelClass", descriptor = "()Z", access = "private"))]
    pub fn isTopLevelClass(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.isLocalOrAnonymousClass()?;
        let _t1 = this.getDeclaringClass0()?;
        Ok(_t1.is_none())
    }

    #[cfg_attr(any(), java_method(name = "isLocalOrAnonymousClass", descriptor = "()Z", access = "private"))]
    pub fn isLocalOrAnonymousClass(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.hasEnclosingMethodInfo()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "hasEnclosingMethodInfo", descriptor = "()Z", access = "private"))]
    pub fn hasEnclosingMethodInfo(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.getEnclosingMethod0()?;
        let mut enclosingInfo: Vec<Object> = _t0;
        Class$EnclosingMethodInfo::validate(&enclosingInfo)?;
        return Ok(1i32);
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "getClasses", descriptor = "()[Ljava/lang/Class;", access = "public"))]
    pub fn getClasses(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        let _t1: Object = Reflection::getCallerClass()?;
        this.checkMemberAccess(sm, 0i32, _t1, 0i32)?;
        let _t2: Object = AccessController::doPrivileged(Class_2::new(this)?)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "getFields", descriptor = "()[Ljava/lang/reflect/Field;", access = "public"))]
    pub fn getFields(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        let _t1: Object = Reflection::getCallerClass()?;
        this.checkMemberAccess(sm, 0i32, _t1, 1i32)?;
        let _t2 = this.privateGetPublicFields()?;
        let _t3: Vec<Object> = Class::copyFields(&_t2)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "getMethods", descriptor = "()[Ljava/lang/reflect/Method;", access = "public"))]
    pub fn getMethods(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        let _t1: Object = Reflection::getCallerClass()?;
        this.checkMemberAccess(sm, 0i32, _t1, 1i32)?;
        let _t2 = this.privateGetPublicMethods()?;
        let _t3: Vec<Object> = Class::copyMethods(&_t2)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "getConstructors", descriptor = "()[Ljava/lang/reflect/Constructor;", access = "public"))]
    pub fn getConstructors(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        let _t1: Object = Reflection::getCallerClass()?;
        this.checkMemberAccess(sm, 0i32, _t1, 1i32)?;
        let _t2 = this.privateGetDeclaredConstructors(1i32)?;
        let _t3: Vec<Object> = Class::copyConstructors(&_t2)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "getField", descriptor = "(Ljava/lang/String;)Ljava/lang/reflect/Field;", access = "public"))]
    pub fn getField(&self, name: String) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(name)?;
        let _t1: Object = System::getSecurityManager()?;
        let mut sm: Object = _t1;
        let _t2: Object = Reflection::getCallerClass()?;
        this.checkMemberAccess(sm, 0i32, _t2, 1i32)?;
        let _t3 = this.getField0(name)?;
        let mut field: Object = _t3;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t4: Object = Class::getReflectionFactory()?;
        let _t5 = _t4.copyField(field)?;
        Ok(_t5)
    }

    #[cfg_attr(any(), java_method(name = "getMethod", descriptor = "(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;", access = "public"))]
    pub fn getMethod(&self, name: String, parameterTypes: Vec<Object>) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(name)?;
        let _t1: Object = System::getSecurityManager()?;
        let mut sm: Object = _t1;
        let _t2: Object = Reflection::getCallerClass()?;
        this.checkMemberAccess(sm, 0i32, _t2, 1i32)?;
        let _t3 = this.getMethod0(name, parameterTypes)?;
        let mut method: Object = _t3;
        let _t4 = this.methodToString(name, parameterTypes)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t5: Object = Class::getReflectionFactory()?;
        let _t6 = _t5.copyMethod(method)?;
        Ok(_t6)
    }

    #[cfg_attr(any(), java_method(name = "getConstructor", descriptor = "([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;", access = "public"))]
    pub fn getConstructor(&self, parameterTypes: Vec<Object>) -> Result<Object> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        let _t1: Object = Reflection::getCallerClass()?;
        this.checkMemberAccess(sm, 0i32, _t1, 1i32)?;
        let _t2: Object = Class::getReflectionFactory()?;
        let _t3 = this.getConstructor0(parameterTypes, 0i32)?;
        let _t4 = _t2.copyConstructor(_t3)?;
        Ok(_t4)
    }

    #[cfg_attr(any(), java_method(name = "getDeclaredClasses", descriptor = "()[Ljava/lang/Class;", access = "public"))]
    pub fn getDeclaredClasses(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        let _t1: Object = Reflection::getCallerClass()?;
        this.checkMemberAccess(sm, 1i32, _t1, 0i32)?;
        let _t2 = this.getDeclaredClasses0()?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "getDeclaredFields", descriptor = "()[Ljava/lang/reflect/Field;", access = "public"))]
    pub fn getDeclaredFields(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        let _t1: Object = Reflection::getCallerClass()?;
        this.checkMemberAccess(sm, 1i32, _t1, 1i32)?;
        let _t2 = this.privateGetDeclaredFields(0i32)?;
        let _t3: Vec<Object> = Class::copyFields(&_t2)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "getRecordComponents", descriptor = "()[Ljava/lang/reflect/RecordComponent;", access = "public"))]
    pub fn getRecordComponents(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        let _t1: Object = Reflection::getCallerClass()?;
        this.checkMemberAccess(sm, 1i32, _t1, 1i32)?;
        let _t2 = this.isRecord()?;
        /* TODO: aconst_null  */
        return Ok(_t2);
        let _t3 = this.getRecordComponents0()?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "getDeclaredMethods", descriptor = "()[Ljava/lang/reflect/Method;", access = "public"))]
    pub fn getDeclaredMethods(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        let _t1: Object = Reflection::getCallerClass()?;
        this.checkMemberAccess(sm, 1i32, _t1, 1i32)?;
        let _t2 = this.privateGetDeclaredMethods(0i32)?;
        let _t3: Vec<Object> = Class::copyMethods(&_t2)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "getDeclaredConstructors", descriptor = "()[Ljava/lang/reflect/Constructor;", access = "public"))]
    pub fn getDeclaredConstructors(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        let _t1: Object = Reflection::getCallerClass()?;
        this.checkMemberAccess(sm, 1i32, _t1, 1i32)?;
        let _t2 = this.privateGetDeclaredConstructors(0i32)?;
        let _t3: Vec<Object> = Class::copyConstructors(&_t2)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "getDeclaredField", descriptor = "(Ljava/lang/String;)Ljava/lang/reflect/Field;", access = "public"))]
    pub fn getDeclaredField(&self, name: String) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(name)?;
        let _t1: Object = System::getSecurityManager()?;
        let mut sm: Object = _t1;
        let _t2: Object = Reflection::getCallerClass()?;
        this.checkMemberAccess(sm, 1i32, _t2, 1i32)?;
        let _t3 = this.privateGetDeclaredFields(0i32)?;
        let _t4: Object = Class::searchFields(&_t3, name)?;
        let mut field: Object = _t4;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t5: Object = Class::getReflectionFactory()?;
        let _t6 = _t5.copyField(field)?;
        Ok(_t6)
    }

    #[cfg_attr(any(), java_method(name = "getDeclaredMethod", descriptor = "(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;", access = "public"))]
    pub fn getDeclaredMethod(&self, name: String, parameterTypes: Vec<Object>) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(name)?;
        let _t1: Object = System::getSecurityManager()?;
        let mut sm: Object = _t1;
        let _t2: Object = Reflection::getCallerClass()?;
        this.checkMemberAccess(sm, 1i32, _t2, 1i32)?;
        let _t3 = this.privateGetDeclaredMethods(0i32)?;
        let _t4: Object = Class::searchMethods(&_t3, name, &parameterTypes)?;
        let mut method: Object = _t4;
        let _t5 = this.methodToString(name, parameterTypes)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t6: Object = Class::getReflectionFactory()?;
        let _t7 = _t6.copyMethod(method)?;
        Ok(_t7)
    }

    #[cfg_attr(any(), java_method(name = "getDeclaredPublicMethods", descriptor = "(Ljava/lang/String;[Ljava/lang/Class;)Ljava/util/List;", access = ""))]
    pub fn getDeclaredPublicMethods(&self, name: String, parameterTypes: Vec<Object>) -> Result<Object> {
        let this = self;
        let _t0 = this.privateGetDeclaredMethods(1i32)?;
        let mut methods: Vec<Object> = _t0;
        let _t1: Object = Class::getReflectionFactory()?;
        let mut factory: Object = _t1;
        let mut result: ArrayList<_> = ArrayList::<_>::new()?;
        let mut local_6: Vec<Object> = methods;
        let mut local_7: i32 = (local_6.len() as i32);
        let mut local_8: i32 = 0i32;
        loop {
            if local_8 >= local_7 { break; }
            let mut method: Object = local_6[local_8 as usize].clone();
            let _t0 = method.getName()?;
            let _t1 = _t0.equals(name)?;
            let _t2 = factory.getExecutableSharedParameterTypes(method)?;
            let _t3: bool = Arrays::equals(&_t2, &parameterTypes)?;
            let _t4 = factory.copyMethod(method)?;
            let _t5 = result.add(_t4)?;
            local_8 = local_8.wrapping_add(1i32);
        }
        Ok(result)
    }

    #[cfg_attr(any(), java_method(name = "getDeclaredConstructor", descriptor = "([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;", access = "public"))]
    pub fn getDeclaredConstructor(&self, parameterTypes: Vec<Object>) -> Result<Object> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        let _t1: Object = Reflection::getCallerClass()?;
        this.checkMemberAccess(sm, 1i32, _t1, 1i32)?;
        let _t2: Object = Class::getReflectionFactory()?;
        let _t3 = this.getConstructor0(parameterTypes, 1i32)?;
        let _t4 = _t2.copyConstructor(_t3)?;
        Ok(_t4)
    }

    #[cfg_attr(any(), java_method(name = "getResourceAsStream", descriptor = "(Ljava/lang/String;)Ljava/io/InputStream;", access = "public"))]
    pub fn getResourceAsStream(&self, name: String) -> Result<Object> {
        let this = self;
        let _t0 = this.resolveName(name)?;
        name = _t0;
        let _t1 = this.getModule()?;
        let mut thisModule: Object = _t1;
        let _t2 = thisModule.isNamed()?;
        let _t3: bool = Resources::canEncapsulate(name)?;
        let _t4: Object = Reflection::getCallerClass()?;
        let _t5 = this.isOpenToCaller(name, _t4)?;
        /* TODO: aconst_null  */
        return Ok(_t5);
        let _t6 = thisModule.getName()?;
        let mut mn: String = _t6;
        let mut cl: Object = this.classLoader.get();
        let _t7: Object = BootLoader::findResourceAsStream(mn, name)?;
        return Ok(_t7);
        let _t8 = cl.findResourceAsStream(mn, name)?;
        return Ok(_t8);
        let _t9 = cl.findResource(mn, name)?;
        let mut url: Object = _t9;
        let _t10 = url.openStream()?;
        /* TODO: aconst_null  */
        return Ok(_t10);
        url = url;
        /* TODO: aconst_null  */
        return Ok(true);
        mn = this.classLoader.get();
        let _t11: Object = ClassLoader::getSystemResourceAsStream(name)?;
        return Ok(_t11);
        let _t12 = mn.getResourceAsStream(name)?;
        Ok(_t12)
    }

    #[cfg_attr(any(), java_method(name = "getResource", descriptor = "(Ljava/lang/String;)Ljava/net/URL;", access = "public"))]
    pub fn getResource(&self, name: String) -> Result<Object> {
        let this = self;
        let _t0 = this.resolveName(name)?;
        name = _t0;
        let _t1 = this.getModule()?;
        let mut thisModule: Object = _t1;
        let _t2 = thisModule.isNamed()?;
        let _t3: bool = Resources::canEncapsulate(name)?;
        let _t4: Object = Reflection::getCallerClass()?;
        let _t5 = this.isOpenToCaller(name, _t4)?;
        /* TODO: aconst_null  */
        return Ok(_t5);
        let _t6 = thisModule.getName()?;
        let mut mn: String = _t6;
        let mut cl: Object = this.classLoader.get();
        let _t7: Object = BootLoader::findResource(mn, name)?;
        return Ok(_t7);
        let _t8 = cl.findResource(mn, name)?;
        return Ok(_t8);
        let mut ioe: Object = cl;
        /* TODO: aconst_null  */
        return Ok(_t3);
        mn = this.classLoader.get();
        let _t9: Object = ClassLoader::getSystemResource(name)?;
        return Ok(_t9);
        let _t10 = mn.getResource(name)?;
        Ok(_t10)
    }

    #[cfg_attr(any(), java_method(name = "isOpenToCaller", descriptor = "(Ljava/lang/String;Ljava/lang/Class;)Z", access = "private"))]
    pub fn isOpenToCaller(&self, name: String, caller: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.getModule()?;
        let mut thisModule: Object = _t0;
        let _t1 = caller.getModule()?;
        /* TODO: aconst_null  */
        let mut callerModule: Object = _t1;
        let _t2: String = Resources::toPackageName(name)?;
        let mut pn: String = _t2;
        let _t3 = thisModule.getDescriptor()?;
        let _t4 = _t3.packages()?;
        let _t5 = _t4.contains(pn)?;
        let _t6 = thisModule.isOpen(pn)?;
        return Ok(_t6);
        let _t7 = thisModule.isOpen(pn, callerModule)?;
        return Ok(0i32);
        Ok(1i32)
    }

    #[cfg_attr(any(), java_method(name = "getProtectionDomain", descriptor = "()Ljava/security/ProtectionDomain;", access = "public"))]
    pub fn getProtectionDomain(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = System::getSecurityManager()?;
        let mut sm: Object = _t0;
        sm.checkPermission(SecurityConstants::GET_PD_PERMISSION())?;
        let _t1 = this.protectionDomain()?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "protectionDomain", descriptor = "()Ljava/security/ProtectionDomain;"))]
    pub fn protectionDomain(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.getProtectionDomain0()?;
        let mut pd: Object = _t0;
        return Ok(Class$Holder::allPermDomain());
        Ok(pd)
    }

    #[cfg_attr(any(), java_native(name = "getProtectionDomain0", descriptor = "()Ljava/security/ProtectionDomain;", access = "private native"))]
    pub fn getProtectionDomain0(&self) -> Result<Object> {
        todo!("native java/lang/Class.getProtectionDomain0")
    }

    #[cfg_attr(any(), java_native(name = "getPrimitiveClass", descriptor = "(Ljava/lang/String;)Ljava/lang/Class;", access = "static native"))]
    pub fn getPrimitiveClass(arg0: String) -> Result<Object> {
        todo!("native java/lang/Class.getPrimitiveClass")
    }

    #[cfg_attr(any(), java_method(name = "checkMemberAccess", descriptor = "(Ljava/lang/SecurityManager;ILjava/lang/Class;Z)V", access = "private"))]
    pub fn checkMemberAccess(&self, sm: Object, which: i32, caller: Object, checkProxyInterfaces: bool) -> Result<()> {
        let this = self;
        let _t0: Object = ClassLoader::getClassLoader(caller)?;
        let mut ccl: Object = _t0;
        let mut cl: Object = this.classLoader.get();
        sm.checkPermission(SecurityConstants::CHECK_MEMBER_ACCESS_PERMISSION())?;
        this.checkPackageAccess(sm, ccl, checkProxyInterfaces)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkPackageAccess", descriptor = "(Ljava/lang/SecurityManager;Ljava/lang/ClassLoader;Z)V", access = "private"))]
    pub fn checkPackageAccess(&self, sm: Object, ccl: Object, checkProxyInterfaces: bool) -> Result<()> {
        let this = self;
        let mut cl: Object = this.classLoader.get();
        let _t0: bool = ReflectUtil::needsPackageAccessCheck(ccl, cl)?;
        let _t1 = this.getPackageName()?;
        let mut pkg: String = _t1;
        let _t2 = pkg.isEmpty()?;
        let _t3: bool = Proxy::isProxyClass(this)?;
        let _t4: bool = ReflectUtil::isNonPublicProxyClass(this)?;
        sm.checkPackageAccess(pkg)?;
        let _t5: bool = Proxy::isProxyClass(this)?;
        let _t6 = this.getInterfaces(0i32)?;
        ReflectUtil::checkProxyPackageAccess(ccl, &_t6)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkPackageAccessForPermittedSubclasses", descriptor = "(Ljava/lang/SecurityManager;Ljava/lang/ClassLoader;[Ljava/lang/Class;)V", access = "private static"))]
    pub fn checkPackageAccessForPermittedSubclasses(sm: Object, ccl: Object, subClasses: &[Object]) -> Result<()> {
        let mut cl: Object = subClasses[0i32 as usize].clone().classLoader.get();
        let _t0: bool = ReflectUtil::needsPackageAccessCheck(ccl, cl)?;
        let mut packages: HashSet<_> = HashSet::<_>::new()?;
        let mut local_5: Vec<Object> = subClasses;
        let mut pkg: i32 = (local_5.len() as i32);
        let mut local_7: i32 = 0i32;
        loop {
            if local_7 >= pkg { break; }
            let mut c: Object = local_5[local_7 as usize].clone();
            let _t0: bool = Proxy::isProxyClass(c)?;
            String::new().append(&String::from("a permitted subclass should not be a proxy class:"))?;
            String::new().append(&c)?;
            return Err(JvmError::Custom(String::from("athrow")));
            let _t1 = c.getPackageName()?;
            let mut pkg: String = _t1;
            let _t2 = pkg.isEmpty()?;
            let _t3 = packages.add(pkg)?;
            local_7 = local_7.wrapping_add(1i32);
        }
        let _t1 = packages.iterator()?;
        local_5 = _t1;
        loop {
            let _t0 = local_5.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_5.next()?;
            pkg = _t0;
            sm.checkPackageAccess(pkg)?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "resolveName", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "private"))]
    pub fn resolveName(&self, name: String) -> Result<String> {
        let this = self;
        let _t0 = name.startsWith(String::from("/"))?;
        let _t1 = this.getPackageName()?;
        let mut baseName: String = _t1;
        let _t2 = baseName.isEmpty()?;
        let _t3 = baseName.length()?;
        let _t4 = name.length()?;
        let mut len: i32 = ((_t3).wrapping_add(1i32)).wrapping_add(_t4);
        let mut sb: String = String::new();
        let _t5 = baseName.replace(46i32, 47i32)?;
        sb.append(&_t5)?;
        sb.append(&47i32)?;
        sb.append(&name)?;
        name = sb;
        let _t6 = name.substring(1i32)?;
        name = _t6;
        Ok(name)
    }

    #[cfg_attr(any(), java_method(name = "reflectionData", descriptor = "()Ljava/lang/Class$ReflectionData;", access = "private"))]
    pub fn reflectionData(&self) -> Result<Object> {
        let this = self;
        let mut reflectionData: Object = this.reflectionData.get();
        let mut classRedefinedCount: i32 = this.classRedefinedCount.get();
        let _t0 = reflectionData.get()?;
        let mut rd: Object = _t0;
        return Ok(rd);
        let _t1 = this.newReflectionData(reflectionData, classRedefinedCount)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "newReflectionData", descriptor = "(Ljava/lang/ref/SoftReference;I)Ljava/lang/Class$ReflectionData;", access = "private"))]
    pub fn newReflectionData(&self, oldReflectionData: Object, classRedefinedCount: i32) -> Result<Object> {
        let this = self;
        let mut rd: Class_ReflectionData = Class_ReflectionData::new(classRedefinedCount)?;
        let _t0: bool = Class$Atomic::casReflectionData(this, oldReflectionData, SoftReference::new(rd)?)?;
        return Ok(rd);
        oldReflectionData = this.reflectionData.get();
        classRedefinedCount = this.classRedefinedCount.get();
        let _t1 = oldReflectionData.get()?;
        rd = _t1;
        Ok(rd)
    }

    #[cfg_attr(any(), java_native(name = "getGenericSignature0", descriptor = "()Ljava/lang/String;", access = "private native"))]
    pub fn getGenericSignature0(&self) -> Result<String> {
        todo!("native java/lang/Class.getGenericSignature0")
    }

    #[cfg_attr(any(), java_method(name = "getFactory", descriptor = "()Lsun/reflect/generics/factory/GenericsFactory;", access = "private"))]
    pub fn getFactory(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = ClassScope::make(this)?;
        let _t1: Object = CoreReflectionFactory::make(this, _t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getGenericInfo", descriptor = "()Lsun/reflect/generics/repository/ClassRepository;", access = "private"))]
    pub fn getGenericInfo(&self) -> Result<Object> {
        let this = self;
        let mut genericInfo: Object = this.genericInfo.get();
        let _t0 = this.getGenericSignature0()?;
        let mut signature: String = _t0;
        genericInfo = ClassRepository::NONE();
        let _t1 = this.getFactory()?;
        let _t2: Object = ClassRepository::make(signature, _t1)?;
        genericInfo = _t2;
        this.genericInfo.set(genericInfo);
        /* TODO: aconst_null  */
        Ok(genericInfo)
    }

    #[cfg_attr(any(), java_native(name = "getRawAnnotations", descriptor = "()[B", access = "native"))]
    pub fn getRawAnnotations(&self) -> Result<Vec<i8>> {
        todo!("native java/lang/Class.getRawAnnotations")
    }

    #[cfg_attr(any(), java_native(name = "getRawTypeAnnotations", descriptor = "()[B", access = "native"))]
    pub fn getRawTypeAnnotations(&self) -> Result<Vec<i8>> {
        todo!("native java/lang/Class.getRawTypeAnnotations")
    }

    #[cfg_attr(any(), java_method(name = "getExecutableTypeAnnotationBytes", descriptor = "(Ljava/lang/reflect/Executable;)[B", access = "static"))]
    pub fn getExecutableTypeAnnotationBytes(ex: Object) -> Result<Vec<i8>> {
        let _t0: Object = Class::getReflectionFactory()?;
        let _t1 = _t0.getExecutableTypeAnnotationBytes(ex)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_native(name = "getConstantPool", descriptor = "()Ljdk/internal/reflect/ConstantPool;", access = "native"))]
    pub fn getConstantPool(&self) -> Result<Object> {
        todo!("native java/lang/Class.getConstantPool")
    }

    #[cfg_attr(any(), java_method(name = "privateGetDeclaredFields", descriptor = "(Z)[Ljava/lang/reflect/Field;", access = "private"))]
    pub fn privateGetDeclaredFields(&self, publicOnly: bool) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.reflectionData()?;
        let mut rd: Object = _t0;
        let mut res: Vec<Object> = rd.declaredFields.get();
        return Ok(res);
        let _t1 = this.getDeclaredFields0(publicOnly)?;
        let _t2: Vec<Object> = Reflection::filterFields(this, &_t1)?;
        res = _t2;
        rd.declaredPublicFields.set(res);
        rd.declaredFields.set(res);
        Ok(res)
    }

    #[cfg_attr(any(), java_method(name = "privateGetPublicFields", descriptor = "()[Ljava/lang/reflect/Field;", access = "private"))]
    pub fn privateGetPublicFields(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.reflectionData()?;
        let mut rd: Object = _t0;
        let mut res: Vec<Object> = rd.publicFields.get();
        return Ok(res);
        let mut fields: LinkedHashSet = LinkedHashSet::new()?;
        let _t1 = this.privateGetDeclaredFields(1i32)?;
        Class::addAll(fields, &_t1)?;
        let _t2 = this.getInterfaces(0i32)?;
        let mut sc: Vec<Object> = _t2;
        let mut local_5: i32 = (sc.len() as i32);
        let mut local_6: i32 = 0i32;
        loop {
            if local_6 >= local_5 { break; }
            let mut si: Object = sc[local_6 as usize].clone();
            let _t0 = si.privateGetPublicFields()?;
            Class::addAll(fields, &_t0)?;
            local_6 = local_6.wrapping_add(1i32);
        }
        let _t3 = this.getSuperclass()?;
        sc = _t3;
        let _t4 = sc.privateGetPublicFields()?;
        Class::addAll(fields, &_t4)?;
        let mut _arr5: Vec<Object> = Vec::with_capacity(0i32 as usize);
        let _t6 = fields.toArray(_arr5)?;
        res = _t6;
        rd.publicFields.set(res);
        Ok(res)
    }

    #[cfg_attr(any(), java_method(name = "addAll", descriptor = "(Ljava/util/Collection;[Ljava/lang/reflect/Field;)V", access = "private static"))]
    pub fn addAll(c: Object, o: &[Object]) -> Result<()> {
        let mut local_2: Vec<Object> = o;
        let mut local_3: i32 = (local_2.len() as i32);
        let mut local_4: i32 = 0i32;
        loop {
            if local_4 >= local_3 { break; }
            let mut f: Object = local_2[local_4 as usize].clone();
            let _t0 = c.add(f)?;
            local_4 = local_4.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "privateGetDeclaredConstructors", descriptor = "(Z)[Ljava/lang/reflect/Constructor;", access = "private"))]
    pub fn privateGetDeclaredConstructors(&self, publicOnly: bool) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.reflectionData()?;
        let mut rd: Object = _t0;
        let mut res: Vec<Object> = rd.declaredConstructors.get();
        return Ok(res);
        let _t1 = this.isInterface()?;
        let mut _arr2: Vec<Object> = Vec::with_capacity(0i32 as usize);
        let mut temporaryRes: Vec<Object> = _arr2;
        res = temporaryRes;
        let _t3 = this.getDeclaredConstructors0(publicOnly)?;
        res = _t3;
        rd.publicConstructors.set(res);
        rd.declaredConstructors.set(res);
        Ok(res)
    }

    #[cfg_attr(any(), java_method(name = "privateGetDeclaredMethods", descriptor = "(Z)[Ljava/lang/reflect/Method;", access = "private"))]
    pub fn privateGetDeclaredMethods(&self, publicOnly: bool) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.reflectionData()?;
        let mut rd: Object = _t0;
        let mut res: Vec<Object> = rd.declaredMethods.get();
        return Ok(res);
        let _t1 = this.getDeclaredMethods0(publicOnly)?;
        let _t2: Vec<Object> = Reflection::filterMethods(this, &_t1)?;
        res = _t2;
        rd.declaredPublicMethods.set(res);
        rd.declaredMethods.set(res);
        Ok(res)
    }

    #[cfg_attr(any(), java_method(name = "privateGetPublicMethods", descriptor = "()[Ljava/lang/reflect/Method;", access = "private"))]
    pub fn privateGetPublicMethods(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.reflectionData()?;
        let mut rd: Object = _t0;
        let mut res: Vec<Object> = rd.publicMethods.get();
        return Ok(res);
        let mut pms: PublicMethods = PublicMethods::new()?;
        let _t1 = this.privateGetDeclaredMethods(1i32)?;
        let mut sc: Vec<Object> = _t1;
        let mut local_5: i32 = (sc.len() as i32);
        let mut local_6: i32 = 0i32;
        loop {
            if local_6 >= local_5 { break; }
            let mut m: Object = sc[local_6 as usize].clone();
            pms.merge(m)?;
            local_6 = local_6.wrapping_add(1i32);
        }
        let _t2 = this.getSuperclass()?;
        sc = _t2;
        let _t3 = sc.privateGetPublicMethods()?;
        local_5 = _t3;
        local_6 = (local_5.len() as i32);
        m = 0i32;
        loop {
            if m >= local_6 { break; }
            let mut m: Object = local_5[m as usize].clone();
            pms.merge(m)?;
            m = m.wrapping_add(1i32);
        }
        let _t4 = this.getInterfaces(0i32)?;
        local_5 = _t4;
        local_6 = (local_5.len() as i32);
        m = 0i32;
        loop {
            if m >= local_6 { break; }
            m = local_5[m as usize].clone();
            let _t0 = m.privateGetPublicMethods()?;
            let mut local_9: Vec<Object> = _t0;
            let mut local_10: i32 = (local_9.len() as i32);
            let mut local_11: i32 = 0i32;
            let mut m: Object = local_9[local_11 as usize].clone();
            let _t1 = m.getModifiers()?;
            let _t2: bool = Modifier::isStatic(_t1)?;
            pms.merge(m)?;
            local_11 = local_11.wrapping_add(1i32);
            m = m.wrapping_add(1i32);
        }
        let _t5 = pms.toArray()?;
        res = _t5;
        rd.publicMethods.set(res);
        Ok(res)
    }

    #[cfg_attr(any(), java_method(name = "searchFields", descriptor = "([Ljava/lang/reflect/Field;Ljava/lang/String;)Ljava/lang/reflect/Field;", access = "private static"))]
    pub fn searchFields(fields: &[Object], name: String) -> Result<Object> {
        let mut local_2: Vec<Object> = fields;
        let mut local_3: i32 = (local_2.len() as i32);
        let mut local_4: i32 = 0i32;
        loop {
            if local_4 >= local_3 { break; }
            let mut field: Object = local_2[local_4 as usize].clone();
            let _t0 = field.getName()?;
            let _t1 = _t0.equals(name)?;
            return Ok(field);
            local_4 = local_4.wrapping_add(1i32);
        }
        /* TODO: aconst_null  */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "getField0", descriptor = "(Ljava/lang/String;)Ljava/lang/reflect/Field;", access = "private"))]
    pub fn getField0(&self, name: String) -> Result<Object> {
        let this = self;
        let _t0 = this.privateGetDeclaredFields(1i32)?;
        let _t1: Object = Class::searchFields(&_t0, name)?;
        let mut res: Object = _t1;
        return Ok(res);
        let _t2 = this.getInterfaces(0i32)?;
        let mut interfaces: Vec<Object> = _t2;
        let mut c: Vec<Object> = interfaces;
        let mut local_5: i32 = (c.len() as i32);
        let mut local_6: i32 = 0i32;
        loop {
            if local_6 >= local_5 { break; }
            let mut c: Object = c[local_6 as usize].clone();
            let _t0 = c.getField0(name)?;
            res = _t0;
            return Ok(res);
            local_6 = local_6.wrapping_add(1i32);
        }
        let _t3 = this.isInterface()?;
        let _t4 = this.getSuperclass()?;
        c = _t4;
        let _t5 = c.getField0(name)?;
        res = _t5;
        return Ok(res);
        /* TODO: aconst_null  */
        Ok(_t5)
    }

    #[cfg_attr(any(), java_method(name = "searchMethods", descriptor = "([Ljava/lang/reflect/Method;Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;", access = "private static"))]
    pub fn searchMethods(methods: &[Object], name: String, parameterTypes: &[Object]) -> Result<Object> {
        let _t0: Object = Class::getReflectionFactory()?;
        let mut fact: Object = _t0;
        /* TODO: aconst_null  */
        let mut res: i32 = todo!("stack underflow");
        let mut local_5: Vec<Object> = methods;
        let mut local_6: i32 = (local_5.len() as i32);
        let mut local_7: i32 = 0i32;
        loop {
            if local_7 >= local_6 { break; }
            let mut m: Object = local_5[local_7 as usize].clone();
            let _t0 = m.getName()?;
            let _t1 = _t0.equals(name)?;
            let _t2 = fact.getExecutableSharedParameterTypes(m)?;
            let _t3: bool = Class::arrayContentsEq(&parameterTypes, &_t2)?;
            let _t4 = res.getReturnType()?;
            let _t5 = m.getReturnType()?;
            let _t6 = res.getReturnType()?;
            let _t7 = m.getReturnType()?;
            let _t8 = _t6.isAssignableFrom(_t7)?;
            res = m;
            local_7 = local_7.wrapping_add(1i32);
        }
        Ok(res)
    }

    #[cfg_attr(any(), java_method(name = "getMethod0", descriptor = "(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;", access = "private"))]
    pub fn getMethod0(&self, name: String, parameterTypes: Vec<Object>) -> Result<Object> {
        let this = self;
        let _t0 = parameterTypes.getMethodsRecursive(Class::EMPTY_CLASS_ARRAY(), parameterTypes, 1i32)?;
        let mut res: Object = _t0;
        /* TODO: aconst_null  */
        let _t1 = res.getMostSpecific()?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getMethodsRecursive", descriptor = "(Ljava/lang/String;[Ljava/lang/Class;Z)Ljava/lang/PublicMethods$MethodList;", access = "private"))]
    pub fn getMethodsRecursive(&self, name: String, parameterTypes: Vec<Object>, includeStatic: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.privateGetDeclaredMethods(1i32)?;
        let mut methods: Vec<Object> = _t0;
        let _t1: Object = PublicMethods$MethodList::filter(&methods, name, &parameterTypes, includeStatic)?;
        let mut res: Object = _t1;
        return Ok(res);
        let _t2 = this.getSuperclass()?;
        let mut sc: Object = _t2;
        let _t3 = sc.getMethodsRecursive(name, parameterTypes, includeStatic)?;
        res = _t3;
        let _t4 = this.getInterfaces(0i32)?;
        let mut local_7: Vec<Object> = _t4;
        let mut local_8: i32 = (local_7.len() as i32);
        let mut local_9: i32 = 0i32;
        loop {
            if local_9 >= local_8 { break; }
            let mut intf: Object = local_7[local_9 as usize].clone();
            let _t0 = intf.getMethodsRecursive(name, parameterTypes, 0i32)?;
            let _t1: Object = PublicMethods$MethodList::merge(res, _t0)?;
            res = _t1;
            local_9 = local_9.wrapping_add(1i32);
        }
        Ok(res)
    }

    #[cfg_attr(any(), java_method(name = "getConstructor0", descriptor = "([Ljava/lang/Class;I)Ljava/lang/reflect/Constructor;", access = "private"))]
    pub fn getConstructor0(&self, parameterTypes: Vec<Object>, which: i32) -> Result<Object> {
        let this = self;
        let _t0: Object = Class::getReflectionFactory()?;
        let mut fact: Object = _t0;
        let _t1 = this.privateGetDeclaredConstructors(which==0i32)?;
        let mut constructors: Vec<Object> = _t1;
        let mut local_5: Vec<Object> = constructors;
        let mut local_6: i32 = (local_5.len() as i32);
        let mut local_7: i32 = 0i32;
        loop {
            if local_7 >= local_6 { break; }
            let mut constructor: Object = local_5[local_7 as usize].clone();
            let _t0 = fact.getExecutableSharedParameterTypes(constructor)?;
            let _t1: bool = Class::arrayContentsEq(&parameterTypes, &_t0)?;
            return Ok(constructor);
            local_7 = local_7.wrapping_add(1i32);
        }
        let _t2 = this.methodToString(String::from("<init>"), parameterTypes)?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "arrayContentsEq", descriptor = "([Ljava/lang/Object;[Ljava/lang/Object;)Z", access = "private static"))]
    pub fn arrayContentsEq(a1: &[Object], a2: &[Object]) -> Result<bool> {
        return Ok((a2.len() as i32)==0i32);
        return Ok((a1.len() as i32)==0i32);
        return Ok(0i32);
        let mut i: i32 = 0i32;
        loop {
            if i >= (a1.len() as i32) { break; }
            return Ok(0i32);
            i = i.wrapping_add(1i32);
        }
        Ok(1i32)
    }

    #[cfg_attr(any(), java_method(name = "copyFields", descriptor = "([Ljava/lang/reflect/Field;)[Ljava/lang/reflect/Field;", access = "private static"))]
    pub fn copyFields(arg: &[Object]) -> Result<Vec<Object>> {
        let mut _arr0: Vec<Object> = Vec::with_capacity((arg.len() as i32) as usize);
        let mut out: Vec<Object> = _arr0;
        let _t1: Object = Class::getReflectionFactory()?;
        let mut fact: Object = _t1;
        let mut i: i32 = 0i32;
        loop {
            if i >= (arg.len() as i32) { break; }
            let _t0 = fact.copyField(arg[i as usize].clone())?;
            out[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        Ok(out)
    }

    #[cfg_attr(any(), java_method(name = "copyMethods", descriptor = "([Ljava/lang/reflect/Method;)[Ljava/lang/reflect/Method;", access = "private static"))]
    pub fn copyMethods(arg: &[Object]) -> Result<Vec<Object>> {
        let mut _arr0: Vec<Object> = Vec::with_capacity((arg.len() as i32) as usize);
        let mut out: Vec<Object> = _arr0;
        let _t1: Object = Class::getReflectionFactory()?;
        let mut fact: Object = _t1;
        let mut i: i32 = 0i32;
        loop {
            if i >= (arg.len() as i32) { break; }
            let _t0 = fact.copyMethod(arg[i as usize].clone())?;
            out[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        Ok(out)
    }

    #[cfg_attr(any(), java_method(name = "copyConstructors", descriptor = "([Ljava/lang/reflect/Constructor;)[Ljava/lang/reflect/Constructor;", access = "private static"))]
    pub fn copyConstructors(arg: &[Object]) -> Result<Vec<Object>> {
        let _t0 = arg.clone()?;
        let mut out: Object = _t0;
        let _t1: Object = Class::getReflectionFactory()?;
        let mut fact: Object = _t1;
        let mut i: i32 = 0i32;
        loop {
            if i >= (out.len() as i32) { break; }
            let _t0 = fact.copyConstructor(out[i as usize].clone())?;
            out[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        Ok(out)
    }

    #[cfg_attr(any(), java_native(name = "getDeclaredFields0", descriptor = "(Z)[Ljava/lang/reflect/Field;", access = "private native"))]
    pub fn getDeclaredFields0(&self, arg0: bool) -> Result<Vec<Object>> {
        todo!("native java/lang/Class.getDeclaredFields0")
    }

    #[cfg_attr(any(), java_native(name = "getDeclaredMethods0", descriptor = "(Z)[Ljava/lang/reflect/Method;", access = "private native"))]
    pub fn getDeclaredMethods0(&self, arg0: bool) -> Result<Vec<Object>> {
        todo!("native java/lang/Class.getDeclaredMethods0")
    }

    #[cfg_attr(any(), java_native(name = "getDeclaredConstructors0", descriptor = "(Z)[Ljava/lang/reflect/Constructor;", access = "private native"))]
    pub fn getDeclaredConstructors0(&self, arg0: bool) -> Result<Vec<Object>> {
        todo!("native java/lang/Class.getDeclaredConstructors0")
    }

    #[cfg_attr(any(), java_native(name = "getDeclaredClasses0", descriptor = "()[Ljava/lang/Class;", access = "private native"))]
    pub fn getDeclaredClasses0(&self) -> Result<Vec<Object>> {
        todo!("native java/lang/Class.getDeclaredClasses0")
    }

    #[cfg_attr(any(), java_native(name = "getRecordComponents0", descriptor = "()[Ljava/lang/reflect/RecordComponent;", access = "private native"))]
    pub fn getRecordComponents0(&self) -> Result<Vec<Object>> {
        todo!("native java/lang/Class.getRecordComponents0")
    }

    #[cfg_attr(any(), java_native(name = "isRecord0", descriptor = "()Z", access = "private native"))]
    pub fn isRecord0(&self) -> Result<bool> {
        todo!("native java/lang/Class.isRecord0")
    }

    #[cfg_attr(any(), java_method(name = "methodToString", descriptor = "(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/String;", access = "private"))]
    pub fn methodToString(&self, name: String, argTypes: Vec<Object>) -> Result<String> {
        let this = self;
        let _t0 = this.getName()?;
        String::new().append(&_t0)?;
        String::new().append(&46i32)?;
        String::new().append(&name)?;
        let _t1: Object = Arrays::stream(&argTypes)?;
        /* TODO: invokedynamic 968 */
        let _t2 = String::from("()").map(_t1)?;
        let _t3: Object = Collectors::joining(String::from(","), String::from("("), String::from(")"))?;
        let _t4 = _t2.collect(_t3)?;
        (argTypes.len() as i32).append(&_t4)?;
        Ok((argTypes.len() as i32))
    }

    #[cfg_attr(any(), java_method(name = "desiredAssertionStatus", descriptor = "()Z", access = "public"))]
    pub fn desiredAssertionStatus(&self) -> Result<bool> {
        let this = self;
        let mut loader: Object = this.classLoader.get();
        let _t0: bool = Class::desiredAssertionStatus0(this)?;
        return Ok(_t0);
        let mut local_2: Object = loader.assertionLock.get();
        /* TODO: monitorenter  */
        let _t1 = this.getName()?;
        let _t2 = loader.desiredAssertionStatus(_t1)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        /* TODO: monitorexit  */
        let mut local_3: Object = local_2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        let _t3: bool = Class::desiredAssertionStatus0(this)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_native(name = "desiredAssertionStatus0", descriptor = "(Ljava/lang/Class;)Z", access = "private static native"))]
    pub fn desiredAssertionStatus0(arg0: Object) -> Result<bool> {
        todo!("native java/lang/Class.desiredAssertionStatus0")
    }

    #[cfg_attr(any(), java_method(name = "isEnum", descriptor = "()Z", access = "public"))]
    pub fn isEnum(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.getModifiers()?;
        let _t1 = this.getSuperclass()?;
        Ok(/* if_acmpne */ true)
    }

    #[cfg_attr(any(), java_method(name = "isRecord", descriptor = "()Z", access = "public"))]
    pub fn isRecord(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.getSuperclass()?;
        let _t1 = this.getModifiers()?;
        let _t2 = this.isRecord0()?;
        Ok(_t2!=0i32)
    }

    #[cfg_attr(any(), java_method(name = "getReflectionFactory", descriptor = "()Ljdk/internal/reflect/ReflectionFactory;", access = "private static"))]
    pub fn getReflectionFactory() -> Result<Object> {
        let mut factory: Object = Class::reflectionFactory();
        return Ok(factory);
        let _t0: Object = AccessController::doPrivileged(ReflectionFactory_GetReflectionFactoryAction::new()?)?;
        Class::reflectionFactory(_t0);
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getEnumConstants", descriptor = "()[Ljava/lang/Object;", access = "public"))]
    pub fn getEnumConstants(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.getEnumConstantsShared()?;
        let mut values: Vec<Object> = _t0;
        let _t1 = values.clone()?;
        /* TODO: aconst_null  */
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getEnumConstantsShared", descriptor = "()[Ljava/lang/Object;"))]
    pub fn getEnumConstantsShared(&self) -> Result<Vec<Object>> {
        let this = self;
        let mut constants: Vec<Object> = this.enumConstants.get();
        let _t0 = this.isEnum()?;
        /* TODO: aconst_null  */
        return Ok(_t0);
        let mut _arr1: Vec<Object> = Vec::with_capacity(0i32 as usize);
        let _t2 = this.getMethod(String::from("values"), _arr1)?;
        let mut values: Object = _t2;
        let _t3: Object = AccessController::doPrivileged(Class_3::new(this, values)?)?;
        /* TODO: aconst_null  */
        let mut _arr4: Vec<Object> = Vec::with_capacity(0i32 as usize);
        let _t5 = constants.invoke(values, _arr4)?;
        let mut temporaryConstants: Object = _t5;
        constants = temporaryConstants;
        this.enumConstants.set(temporaryConstants);
        values = todo!("stack underflow");
        /* TODO: aconst_null  */
        return Ok(todo!("stack underflow"));
        Ok(constants)
    }

    #[cfg_attr(any(), java_method(name = "enumConstantDirectory", descriptor = "()Ljava/util/Map;"))]
    pub fn enumConstantDirectory(&self) -> Result<Object> {
        let this = self;
        let mut directory: Object = this.enumConstantDirectory.get();
        let _t0 = this.getEnumConstantsShared()?;
        let mut universe: Vec<Object> = _t0;
        let _t1 = this.getName()?;
        String::new().append(&_t1)?;
        String::new().append(&String::from("is not an enum class"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t2: Object = HashMap::newHashMap((universe.len() as i32))?;
        directory = _t2;
        let mut local_3: Vec<Object> = universe;
        let mut local_4: i32 = (local_3.len() as i32);
        let mut local_5: i32 = 0i32;
        loop {
            if local_5 >= local_4 { break; }
            let mut constant: Object = local_3[local_5 as usize].clone();
            let _t0 = constant.name()?;
            let _t1 = directory.put(_t0, constant)?;
            local_5 = local_5.wrapping_add(1i32);
        }
        this.enumConstantDirectory.set(directory);
        Ok(directory)
    }

    #[cfg_attr(any(), java_method(name = "cast", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn cast(&self, obj: Object) -> Result<T> {
        let this = self;
        let _t0 = this.isInstance(obj)?;
        let _t1 = this.cannotCastMsg(obj)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(obj)
    }

    #[cfg_attr(any(), java_method(name = "cannotCastMsg", descriptor = "(Ljava/lang/Object;)Ljava/lang/String;", access = "private"))]
    pub fn cannotCastMsg(&self, obj: Object) -> Result<String> {
        let this = self;
        String::new().append(&String::from("Cannot cast"))?;
        let _t0 = obj.getClass()?;
        let _t1 = _t0.getName()?;
        String::new().append(&_t1)?;
        String::new().append(&String::from("to"))?;
        let _t2 = this.getName()?;
        String::new().append(&_t2)?;
        Ok(String::new())
    }

    #[cfg_attr(any(), java_method(name = "asSubclass", descriptor = "(Ljava/lang/Class;)Ljava/lang/Class;", access = "public"))]
    pub fn asSubclass(&self, clazz: Object) -> Result<Object> {
        let this = self;
        let _t0 = clazz.isAssignableFrom(this)?;
        return Ok(this);
        let _t1 = this.toString()?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "getAnnotation", descriptor = "(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;", access = "public"))]
    pub fn getAnnotation(&self, annotationClass: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(annotationClass)?;
        let _t1 = this.annotationData()?;
        let _t2 = _t1.annotations.get().get(annotationClass)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "isAnnotationPresent", descriptor = "(Ljava/lang/Class;)Z", access = "public"))]
    pub fn isAnnotationPresent(&self, annotationClass: Object) -> Result<bool> {
        let this = self;
        let _t0: bool = GenericDeclaration::isAnnotationPresent(annotationClass)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getAnnotationsByType", descriptor = "(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;", access = "public"))]
    pub fn getAnnotationsByType(&self, annotationClass: Object) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(annotationClass)?;
        let _t1 = this.annotationData()?;
        let mut annotationData: Object = _t1;
        let _t2: Vec<Object> = AnnotationSupport::getAssociatedAnnotations(annotationData.declaredAnnotations.get(), this, annotationClass)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "getAnnotations", descriptor = "()[Ljava/lang/annotation/Annotation;", access = "public"))]
    pub fn getAnnotations(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.annotationData()?;
        let _t1: Vec<Object> = AnnotationParser::toArray(_t0.annotations.get())?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getDeclaredAnnotation", descriptor = "(Ljava/lang/Class;)Ljava/lang/annotation/Annotation;", access = "public"))]
    pub fn getDeclaredAnnotation(&self, annotationClass: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(annotationClass)?;
        let _t1 = this.annotationData()?;
        let _t2 = _t1.declaredAnnotations.get().get(annotationClass)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "getDeclaredAnnotationsByType", descriptor = "(Ljava/lang/Class;)[Ljava/lang/annotation/Annotation;", access = "public"))]
    pub fn getDeclaredAnnotationsByType(&self, annotationClass: Object) -> Result<Vec<Object>> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(annotationClass)?;
        let _t1 = this.annotationData()?;
        let _t2: Vec<Object> = AnnotationSupport::getDirectlyAndIndirectlyPresent(_t1.declaredAnnotations.get(), annotationClass)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "getDeclaredAnnotations", descriptor = "()[Ljava/lang/annotation/Annotation;", access = "public"))]
    pub fn getDeclaredAnnotations(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.annotationData()?;
        let _t1: Vec<Object> = AnnotationParser::toArray(_t0.declaredAnnotations.get())?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "annotationData", descriptor = "()Ljava/lang/Class$AnnotationData;", access = "private"))]
    pub fn annotationData(&self) -> Result<Object> {
        let this = self;
        let mut annotationData: Object = this.annotationData.get();
        let mut classRedefinedCount: i32 = this.classRedefinedCount.get();
        return Ok(annotationData);
        let _t0 = this.createAnnotationData(classRedefinedCount)?;
        let mut newAnnotationData: Object = _t0;
        let _t1: bool = Class$Atomic::casAnnotationData(this, annotationData, newAnnotationData)?;
        Ok(newAnnotationData)
    }

    #[cfg_attr(any(), java_method(name = "createAnnotationData", descriptor = "(I)Ljava/lang/Class$AnnotationData;", access = "private"))]
    pub fn createAnnotationData(&self, classRedefinedCount: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.getRawAnnotations()?;
        let _t1 = this.getConstantPool()?;
        let _t2: Object = AnnotationParser::parseAnnotations(&_t0, _t1, this)?;
        let mut declaredAnnotations: Object = _t2;
        let _t3 = this.getSuperclass()?;
        let mut superClass: Object = _t3;
        /* TODO: aconst_null  */
        let mut annotations: i32 = todo!("stack underflow");
        let _t4 = superClass.annotationData()?;
        let mut superAnnotations: Object = _t4.annotations.get();
        let _t5 = superAnnotations.entrySet()?;
        let _t6 = _t5.iterator()?;
        let mut local_6: Object = _t6;
        loop {
            let _t0 = local_6.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_6.next()?;
            let mut e: Object = _t0;
            let _t1 = e.getKey()?;
            let mut annotationClass: Object = _t1;
            let _t2: Object = AnnotationType::getInstance(annotationClass)?;
            let _t3 = _t2.isInherited()?;
            let _t4 = declaredAnnotations.size()?;
            let _t5 = declaredAnnotations.size()?;
            let _t6 = superAnnotations.size()?;
            let _t7: i32 = (12i32).min((_t5).wrapping_add(_t6));
            let _t8: i32 = (_t4).max(_t7);
            let _t9: Object = LinkedHashMap::newLinkedHashMap(_t8)?;
            annotations = _t9;
            let _t10 = e.getValue()?;
            let _t11 = annotations.put(annotationClass, _t10)?;
        }
        annotations = declaredAnnotations;
        annotations.putAll(declaredAnnotations)?;
        Ok(Class_AnnotationData::new(annotations, declaredAnnotations, classRedefinedCount)?)
    }

    #[cfg_attr(any(), java_method(name = "casAnnotationType", descriptor = "(Lsun/reflect/annotation/AnnotationType;Lsun/reflect/annotation/AnnotationType;)Z"))]
    pub fn casAnnotationType(&self, oldType: Object, newType: Object) -> Result<bool> {
        let this = self;
        let _t0: bool = Class$Atomic::casAnnotationType(this, oldType, newType)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getAnnotationType", descriptor = "()Lsun/reflect/annotation/AnnotationType;"))]
    pub fn getAnnotationType(&self) -> Result<Object> {
        let this = self;
        Ok(this.annotationType.get())
    }

    #[cfg_attr(any(), java_method(name = "getDeclaredAnnotationMap", descriptor = "()Ljava/util/Map;"))]
    pub fn getDeclaredAnnotationMap(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.annotationData()?;
        Ok(_t0.declaredAnnotations.get())
    }

    #[cfg_attr(any(), java_method(name = "getAnnotatedSuperclass", descriptor = "()Ljava/lang/reflect/AnnotatedType;", access = "public"))]
    pub fn getAnnotatedSuperclass(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.isInterface()?;
        let _t1 = this.isArray()?;
        let _t2 = this.isPrimitive()?;
        /* TODO: aconst_null  */
        return Ok(Void::TYPE());
        let _t3 = this.getRawTypeAnnotations()?;
        let _t4 = this.getConstantPool()?;
        let _t5: Object = TypeAnnotationParser::buildAnnotatedSuperclass(&_t3, _t4, this)?;
        Ok(_t5)
    }

    #[cfg_attr(any(), java_method(name = "getAnnotatedInterfaces", descriptor = "()[Ljava/lang/reflect/AnnotatedType;", access = "public"))]
    pub fn getAnnotatedInterfaces(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.getRawTypeAnnotations()?;
        let _t1 = this.getConstantPool()?;
        let _t2: Vec<Object> = TypeAnnotationParser::buildAnnotatedInterfaces(&_t0, _t1, this)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_native(name = "getNestHost0", descriptor = "()Ljava/lang/Class;", access = "private native"))]
    pub fn getNestHost0(&self) -> Result<Object> {
        todo!("native java/lang/Class.getNestHost0")
    }

    #[cfg_attr(any(), java_method(name = "getNestHost", descriptor = "()Ljava/lang/Class;", access = "public"))]
    pub fn getNestHost(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.isPrimitive()?;
        let _t1 = this.isArray()?;
        return Ok(this);
        let _t2 = this.getNestHost0()?;
        let mut host: Object = _t2;
        return Ok(this);
        let _t3: Object = System::getSecurityManager()?;
        let mut sm: Object = _t3;
        let _t4: Object = Reflection::getCallerClass()?;
        let _t5: Object = ClassLoader::getClassLoader(_t4)?;
        this.checkPackageAccess(sm, _t5, 1i32)?;
        Ok(host)
    }

    #[cfg_attr(any(), java_method(name = "isNestmateOf", descriptor = "(Ljava/lang/Class;)Z", access = "public"))]
    pub fn isNestmateOf(&self, c: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        let _t0 = this.isPrimitive()?;
        let _t1 = this.isArray()?;
        let _t2 = c.isPrimitive()?;
        let _t3 = c.isArray()?;
        return Ok(0i32);
        let _t4 = this.getNestHost()?;
        let _t5 = c.getNestHost()?;
        Ok(/* if_acmpne */ true)
    }

    #[cfg_attr(any(), java_native(name = "getNestMembers0", descriptor = "()[Ljava/lang/Class;", access = "private native"))]
    pub fn getNestMembers0(&self) -> Result<Vec<Object>> {
        todo!("native java/lang/Class.getNestMembers0")
    }

    #[cfg_attr(any(), java_method(name = "getNestMembers", descriptor = "()[Ljava/lang/Class;", access = "public"))]
    pub fn getNestMembers(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.isPrimitive()?;
        let _t1 = this.isArray()?;
        let mut _arr2: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr2[0i32 as usize] = this;
        return Ok(_arr2);
        let _t3 = this.getNestMembers0()?;
        let mut members: Vec<Object> = _t3;
        let _t4: Object = System::getSecurityManager()?;
        let mut sm: Object = _t4;
        let _t5: Object = Reflection::getCallerClass()?;
        let _t6: Object = ClassLoader::getClassLoader(_t5)?;
        this.checkPackageAccess(sm, _t6, 1i32)?;
        Ok(members)
    }

    #[cfg_attr(any(), java_method(name = "descriptorString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn descriptorString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.isPrimitive()?;
        let _t1: Object = Wrapper::forPrimitiveType(this)?;
        let _t2 = _t1.basicTypeString()?;
        return Ok(_t2);
        let _t3 = this.isArray()?;
        String::new().append(&String::from("["))?;
        let _t4 = this.componentType.get().descriptorString()?;
        String::new().append(&_t4)?;
        return Ok(String::new());
        let _t5 = this.isHidden()?;
        let _t6 = this.getName()?;
        let mut name: String = _t6;
        let _t7 = name.indexOf(47i32)?;
        let mut index: i32 = _t7;
        let _t8 = name.length()?;
        String::new().append(&76i32)?;
        let _t9 = name.substring(0i32, index)?;
        let _t10 = _t9.replace(46i32, 47i32)?;
        String::new().append(&_t10)?;
        String::new().append(&46i32)?;
        let _t11 = name.length()?;
        String::new().append(&name)?;
        String::new().append(&59i32)?;
        return Ok(String::new());
        let _t12 = this.getName()?;
        let _t13 = _t12.replace(46i32, 47i32)?;
        name = _t13;
        let _t14 = name.length()?;
        String::new().append(&76i32)?;
        String::new().append(&name)?;
        String::new().append(&59i32)?;
        Ok(String::new())
    }

    #[cfg_attr(any(), java_method(name = "componentType", descriptor = "()Ljava/lang/Class;", access = "public"))]
    pub fn componentType(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.isArray()?;
        /* TODO: aconst_null  */
        Ok(this.componentType.get())
    }

    #[cfg_attr(any(), java_method(name = "arrayType", descriptor = "()Ljava/lang/Class;", access = "public"))]
    pub fn arrayType(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Array::newInstance(this, 0i32)?;
        let _t1 = _t0.getClass()?;
        return Ok(_t1);
        let mut iae: i32 = todo!("stack underflow");
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "describeConstable", descriptor = "()Ljava/util/Optional;", access = "public"))]
    pub fn describeConstable(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.isArray()?;
        let _t1 = this.elementType()?;
        let mut c: java/lang/Class = this;
        let _t2 = c.isHidden()?;
        let _t3: Object = Optional::empty()?;
        let _t4 = this.descriptorString()?;
        let _t5: Object = ClassDesc::ofDescriptor(_t4)?;
        let _t6: Object = Optional::of(_t5)?;
        Ok(_t6)
    }

    #[cfg_attr(any(), java_native(name = "isHidden", descriptor = "()Z", access = "public native"))]
    pub fn isHidden(&self) -> Result<bool> {
        todo!("native java/lang/Class.isHidden")
    }

    #[cfg_attr(any(), java_method(name = "getPermittedSubclasses", descriptor = "()[Ljava/lang/Class;", access = "public"))]
    pub fn getPermittedSubclasses(&self) -> Result<Vec<Object>> {
        let this = self;
        let _t0 = this.isArray()?;
        let _t1 = this.isPrimitive()?;
        let _t2 = this.getPermittedSubclasses0()?;
        let mut subClasses: Vec<Object> = _t2;
        /* TODO: aconst_null  */
        return Ok(_t2);
        let _t3: Object = Arrays::stream(&subClasses)?;
        /* TODO: invokedynamic 1244 */
        let _t4 = _t3.anyMatch(this)?;
        let _t5: Object = Arrays::stream(&subClasses)?;
        /* TODO: invokedynamic 1252 */
        let _t6 = _t5.filter(this)?;
        /* TODO: invokedynamic 1256 */
        let _t7 = _t4.toArray(_t6)?;
        subClasses = _t7;
        let _t8: Object = System::getSecurityManager()?;
        let mut sm: Object = _t8;
        let _t9: Object = Reflection::getCallerClass()?;
        let _t10: Object = ClassLoader::getClassLoader(_t9)?;
        Class::checkPackageAccessForPermittedSubclasses(sm, _t10, &subClasses)?;
        Ok(subClasses)
    }

    #[cfg_attr(any(), java_method(name = "isDirectSubType", descriptor = "(Ljava/lang/Class;)Z", access = "private"))]
    pub fn isDirectSubType(&self, c: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.isInterface()?;
        let _t1 = c.getInterfaces(0i32)?;
        let mut local_2: Vec<Object> = _t1;
        let mut local_3: i32 = (local_2.len() as i32);
        let mut local_4: i32 = 0i32;
        loop {
            if local_4 >= local_3 { break; }
            let mut i: Object = local_2[local_4 as usize].clone();
            return Ok(1i32);
            local_4 = local_4.wrapping_add(1i32);
        }
        let _t2 = c.getSuperclass()?;
        return Ok(/* if_acmpne */ true);
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "isSealed", descriptor = "()Z", access = "public"))]
    pub fn isSealed(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.isArray()?;
        let _t1 = this.isPrimitive()?;
        return Ok(0i32);
        let _t2 = this.getPermittedSubclasses()?;
        Ok(!_t2.is_none())
    }

    #[cfg_attr(any(), java_native(name = "getPermittedSubclasses0", descriptor = "()[Ljava/lang/Class;", access = "private native"))]
    pub fn getPermittedSubclasses0(&self) -> Result<Vec<Object>> {
        todo!("native java/lang/Class.getPermittedSubclasses0")
    }

    #[cfg_attr(any(), java_method(name = "getClassFileVersion", descriptor = "()I", access = "private"))]
    pub fn getClassFileVersion(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.isArray()?;
        let _t1 = this.elementType()?;
        let mut c: java/lang/Class = this;
        let _t2 = c.getClassFileVersion0()?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_native(name = "getClassFileVersion0", descriptor = "()I", access = "private native"))]
    pub fn getClassFileVersion0(&self) -> Result<i32> {
        todo!("native java/lang/Class.getClassFileVersion0")
    }

    #[cfg_attr(any(), java_method(name = "getClassAccessFlagsRaw", descriptor = "()I", access = "private"))]
    pub fn getClassAccessFlagsRaw(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.isArray()?;
        let _t1 = this.elementType()?;
        let mut c: java/lang/Class = this;
        let _t2 = c.getClassAccessFlagsRaw0()?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_native(name = "getClassAccessFlagsRaw0", descriptor = "()I", access = "private native"))]
    pub fn getClassAccessFlagsRaw0(&self) -> Result<i32> {
        todo!("native java/lang/Class.getClassAccessFlagsRaw0")
    }
}
