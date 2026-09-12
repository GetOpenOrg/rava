#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/StackTraceElement",
    super_class = "java/lang/Object",
    interfaces  = "java/io/Serializable",
    access      = "public final",
    source      = "StackTraceElement.java",
))]
pub struct StackTraceElement {
    #[cfg_attr(any(), java_field(name = "declaringClassObject", descriptor = "Ljava/lang/Class;", access = "private"))]
    pub declaringClassObject: Field<Object>,
    #[cfg_attr(any(), java_field(name = "classLoaderName", descriptor = "Ljava/lang/String;", access = "private"))]
    pub classLoaderName: Field<String>,
    #[cfg_attr(any(), java_field(name = "moduleName", descriptor = "Ljava/lang/String;", access = "private"))]
    pub moduleName: Field<String>,
    #[cfg_attr(any(), java_field(name = "moduleVersion", descriptor = "Ljava/lang/String;", access = "private"))]
    pub moduleVersion: Field<String>,
    #[cfg_attr(any(), java_field(name = "declaringClass", descriptor = "Ljava/lang/String;", access = "private"))]
    pub declaringClass: Field<String>,
    #[cfg_attr(any(), java_field(name = "methodName", descriptor = "Ljava/lang/String;", access = "private"))]
    pub methodName: Field<String>,
    #[cfg_attr(any(), java_field(name = "fileName", descriptor = "Ljava/lang/String;", access = "private"))]
    pub fileName: Field<String>,
    #[cfg_attr(any(), java_field(name = "lineNumber", descriptor = "I", access = "private"))]
    pub lineNumber: Field<i32>,
    #[cfg_attr(any(), java_field(name = "format", descriptor = "B", access = "private"))]
    pub format: Field<i8>,
}

impl StackTraceElement {
    // java: <init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;I)V
    // java: <init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;I)V
    pub fn new__str_str_str_i(declaringClass: String, methodName: String, fileName: String, lineNumber: i32) -> Result<Self> {
        let this = Self { declaringClassObject: Field::new(Default::default()), classLoaderName: Field::new(String::new()), moduleName: Field::new(String::new()), moduleVersion: Field::new(String::new()), declaringClass: Field::new(String::new()), methodName: Field::new(String::new()), fileName: Field::new(String::new()), lineNumber: Field::new(0), format: Field::new(Default::default()) };
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        /* TODO: aconst_null  */
        /* invokespecial Method java/lang/StackTraceElement.<init>:(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;I)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;I)V
    // java: <init>(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;I)V
    pub fn new__str_str_str_str_str_str_i(classLoaderName: String, moduleName: String, moduleVersion: String, declaringClass: String, methodName: String, fileName: String, lineNumber: i32) -> Result<Self> {
        let this = Self { declaringClassObject: Field::new(Default::default()), classLoaderName: Field::new(String::new()), moduleName: Field::new(String::new()), moduleVersion: Field::new(String::new()), declaringClass: Field::new(String::new()), methodName: Field::new(String::new()), fileName: Field::new(String::new()), lineNumber: Field::new(0), format: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.format.set(0i32);
        this.classLoaderName.set(classLoaderName);
        this.moduleName.set(moduleName);
        this.moduleVersion.set(moduleVersion);
        let _t0: Object = Objects::requireNonNull(declaringClass, String::from("Declaring class is null"))?;
        this.declaringClass.set(_t0);
        let _t1: Object = Objects::requireNonNull(methodName, String::from("Method name is null"))?;
        this.methodName.set(_t1);
        this.fileName.set(fileName);
        this.lineNumber.set(lineNumber);
        Ok(this)
    }

    // java: <init>()V
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { declaringClassObject: Field::new(Default::default()), classLoaderName: Field::new(String::new()), moduleName: Field::new(String::new()), moduleVersion: Field::new(String::new()), declaringClass: Field::new(String::new()), methodName: Field::new(String::new()), fileName: Field::new(String::new()), lineNumber: Field::new(0), format: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.format.set(0i32);
        Ok(this)
    }

    // java: getFileName()Ljava/lang/String;
    pub fn getFileName(&self) -> Result<String> {
        let this = self;
        Ok(this.fileName.get())
    }

    // java: getLineNumber()I
    pub fn getLineNumber(&self) -> Result<i32> {
        let this = self;
        Ok(this.lineNumber.get())
    }

    // java: getModuleName()Ljava/lang/String;
    pub fn getModuleName(&self) -> Result<String> {
        let this = self;
        Ok(this.moduleName.get())
    }

    // java: getModuleVersion()Ljava/lang/String;
    pub fn getModuleVersion(&self) -> Result<String> {
        let this = self;
        Ok(this.moduleVersion.get())
    }

    // java: getClassLoaderName()Ljava/lang/String;
    pub fn getClassLoaderName(&self) -> Result<String> {
        let this = self;
        Ok(this.classLoaderName.get())
    }

    // java: getClassName()Ljava/lang/String;
    pub fn getClassName(&self) -> Result<String> {
        let this = self;
        Ok(this.declaringClass.get())
    }

    // java: getMethodName()Ljava/lang/String;
    pub fn getMethodName(&self) -> Result<String> {
        let this = self;
        Ok(this.methodName.get())
    }

    // java: isNativeMethod()Z
    pub fn isNativeMethod(&self) -> Result<bool> {
        let this = self;
        Ok(this.lineNumber.get() == -2i32)
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0: i32 = StackTraceElement::length(this.classLoaderName.get())?;
        let _t1: i32 = StackTraceElement::length(this.moduleName.get())?;
        let _t2: i32 = StackTraceElement::length(this.moduleVersion.get())?;
        let _t3 = this.declaringClass.get().length()?;
        let _t4 = this.methodName.get().length()?;
        let _t5 = String::from("Unknown Source").length()?;
        let _t6: i32 = StackTraceElement::length(this.fileName.get())?;
        let _t7: i32 = (_t5).max(_t6);
        let mut estimatedLength: i32 = ((((((((((((_t0).wrapping_add(1i32)).wrapping_add(_t1)).wrapping_add(1i32)).wrapping_add(_t2)).wrapping_add(1i32)).wrapping_add(_t3)).wrapping_add(1i32)).wrapping_add(_t4)).wrapping_add(1i32)).wrapping_add(_t7)).wrapping_add(1i32)).wrapping_add(12i32);
        let mut sb: String = String::new();
        let _t8 = this.dropClassLoaderName()?;
        let _t9 = this.classLoaderName.get().isEmpty()?;
        sb.append(&this.classLoaderName.get())?;
        sb.append(&47i32)?;
        let _t10 = this.moduleName.get().isEmpty()?;
        sb.append(&this.moduleName.get())?;
        let _t11 = this.dropModuleVersion()?;
        let _t12 = this.moduleVersion.get().isEmpty()?;
        sb.append(&64i32)?;
        sb.append(&this.moduleVersion.get())?;
        let _t13 = sb.length()?;
        sb.append(&47i32)?;
        sb.append(&this.declaringClass.get())?;
        sb.append(&46i32)?;
        sb.append(&this.methodName.get())?;
        sb.append(&40i32)?;
        let _t14 = this.isNativeMethod()?;
        sb.append(&String::from("Native Method"))?;
        sb.append(&String::from("Unknown Source"))?;
        sb.append(&this.fileName.get())?;
        sb.append(&58i32)?;
        sb.append(&this.lineNumber.get())?;
        sb.append(&41i32)?;
        Ok(sb)
    }

    // java: length(Ljava/lang/String;)I
    pub fn length(s: String) -> Result<i32> {
        let _t0 = s.length()?;
        Ok(_t0)
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, obj: Object) -> Result<bool> {
        let this = self;
        return Ok(1i32);
        let mut e: Object = obj;
        let _t0 = e.declaringClass.get().equals(this.declaringClass.get())?;
        let _t1: bool = Objects::equals(this.classLoaderName.get(), e.classLoaderName.get())?;
        let _t2: bool = Objects::equals(this.moduleName.get(), e.moduleName.get())?;
        let _t3: bool = Objects::equals(this.moduleVersion.get(), e.moduleVersion.get())?;
        let _t4: bool = Objects::equals(this.methodName.get(), e.methodName.get())?;
        let _t5: bool = Objects::equals(this.fileName.get(), e.fileName.get())?;
        Ok(_t5!=0i32)
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.declaringClass.get().hashCode()?;
        let _t1 = this.methodName.get().hashCode()?;
        let mut result: i32 = ((31i32).wrapping_mul(_t0)).wrapping_add(_t1);
        let _t2: i32 = Objects::hashCode(this.classLoaderName.get())?;
        result = ((31i32).wrapping_mul(result)).wrapping_add(_t2);
        let _t3: i32 = Objects::hashCode(this.moduleName.get())?;
        result = ((31i32).wrapping_mul(result)).wrapping_add(_t3);
        let _t4: i32 = Objects::hashCode(this.moduleVersion.get())?;
        result = ((31i32).wrapping_mul(result)).wrapping_add(_t4);
        let _t5: i32 = Objects::hashCode(this.fileName.get())?;
        result = ((31i32).wrapping_mul(result)).wrapping_add(_t5);
        result = ((31i32).wrapping_mul(result)).wrapping_add(this.lineNumber.get());
        Ok(result)
    }

    // java: computeFormat()V
    pub fn computeFormat(&self) -> Result<()> {
        let this = self;
        let mut cls: Object = this.declaringClassObject.get();
        let _t0 = cls.getClassLoader0()?;
        let mut loader: Object = _t0;
        let _t1 = cls.getModule()?;
        let mut m: Object = _t1;
        let mut bits: i32 = 0i32;
        /* TODO: i2b  */
        bits = (bits|1i32);
        let _t2: bool = StackTraceElement::isHashedInJavaBase(m)?;
        /* TODO: i2b  */
        bits = (bits|2i32);
        this.format.set(bits);
        /* TODO: aconst_null  */
        _t2.declaringClassObject.set(this);
        let mut local_5: bool = true;
        /* TODO: aconst_null  */
        loader.declaringClassObject.set(this);
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: dropClassLoaderName()Z
    pub fn dropClassLoaderName(&self) -> Result<bool> {
        let this = self;
        Ok((this.format.get()&1i32) == 1i32)
    }

    // java: dropModuleVersion()Z
    pub fn dropModuleVersion(&self) -> Result<bool> {
        let this = self;
        Ok((this.format.get()&2i32) == 2i32)
    }

    // java: isHashedInJavaBase(Ljava/lang/Module;)Z
    pub fn isHashedInJavaBase(m: Object) -> Result<bool> {
        let _t0: bool = VM::isModuleSystemInited()?;
        return Ok(1i32);
        let _t1: Object = ModuleLayer::boot()?;
        let _t2 = m.getLayer()?;
        let _t3: bool = StackTraceElement_HashedModules::contains(m)?;
        Ok(_t3!=0i32)
    }

    // java: of(Ljava/lang/Object;I)[Ljava/lang/StackTraceElement;
    // java: of(Ljava/lang/Object;I)[Ljava/lang/StackTraceElement;
    pub fn of__obj_i(x: Object, depth: i32) -> Result<Vec<Object>> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(depth as usize);
        let mut stackTrace: Vec<Object> = _arr0;
        let mut i: i32 = 0i32;
        loop {
            if i >= depth { break; }
            stackTrace[i as usize] = StackTraceElement::new()?;
            i = i.wrapping_add(1i32);
        }
        StackTraceElement::initStackTraceElements(&stackTrace, x, depth)?;
        let _t1: Vec<Object> = StackTraceElement::of(&stackTrace)?;
        Ok(_t1)
    }

    // java: of(Ljava/lang/StackFrameInfo;)Ljava/lang/StackTraceElement;
    // java: of(Ljava/lang/StackFrameInfo;)Ljava/lang/StackTraceElement;
    pub fn of__stackf(sfi: Object) -> Result<Object> {
        let mut ste: StackTraceElement = StackTraceElement::new()?;
        StackTraceElement::initStackTraceElement(ste, sfi)?;
        ste.computeFormat()?;
        Ok(ste)
    }

    // java: of([Ljava/lang/StackTraceElement;)[Ljava/lang/StackTraceElement;
    // java: of([Ljava/lang/StackTraceElement;)[Ljava/lang/StackTraceElement;
    pub fn of__arr_sta(stackTrace: &[Object]) -> Result<Vec<Object>> {
        let mut local_1: Vec<Object> = stackTrace;
        let mut local_2: i32 = (local_1.len() as i32);
        let mut local_3: i32 = 0i32;
        loop {
            if local_3 >= local_2 { break; }
            let mut ste: Object = local_1[local_3 as usize].clone();
            ste.computeFormat()?;
            local_3 = local_3.wrapping_add(1i32);
        }
        Ok(stackTrace)
    }

    // java: initStackTraceElements([Ljava/lang/StackTraceElement;Ljava/lang/Object;I)V
    pub fn initStackTraceElements(arg0: Vec<Object>, arg1: Object, arg2: i32) -> Result<()> {
        todo!("native java/lang/StackTraceElement.initStackTraceElements")
    }

    // java: initStackTraceElement(Ljava/lang/StackTraceElement;Ljava/lang/StackFrameInfo;)V
    pub fn initStackTraceElement(arg0: Object, arg1: Object) -> Result<()> {
        todo!("native java/lang/StackTraceElement.initStackTraceElement")
    }
}
