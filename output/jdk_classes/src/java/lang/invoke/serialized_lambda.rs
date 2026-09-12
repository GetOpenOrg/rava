#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/invoke/SerializedLambda",
    super_class = "java/lang/Object",
    interfaces  = "java/io/Serializable",
    access      = "public final",
    source      = "SerializedLambda.java",
))]
pub struct SerializedLambda {
    #[cfg_attr(any(), java_field(name = "capturingClass", descriptor = "Ljava/lang/Class;", access = "private final"))]
    pub capturingClass: Field<Object>,
    #[cfg_attr(any(), java_field(name = "functionalInterfaceClass", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub functionalInterfaceClass: Field<String>,
    #[cfg_attr(any(), java_field(name = "functionalInterfaceMethodName", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub functionalInterfaceMethodName: Field<String>,
    #[cfg_attr(any(), java_field(name = "functionalInterfaceMethodSignature", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub functionalInterfaceMethodSignature: Field<String>,
    #[cfg_attr(any(), java_field(name = "implClass", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub implClass: Field<String>,
    #[cfg_attr(any(), java_field(name = "implMethodName", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub implMethodName: Field<String>,
    #[cfg_attr(any(), java_field(name = "implMethodSignature", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub implMethodSignature: Field<String>,
    #[cfg_attr(any(), java_field(name = "implMethodKind", descriptor = "I", access = "private final"))]
    pub implMethodKind: Field<i32>,
    #[cfg_attr(any(), java_field(name = "instantiatedMethodType", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub instantiatedMethodType: Field<String>,
    #[cfg_attr(any(), java_field(name = "capturedArgs", descriptor = "[Ljava/lang/Object;", access = "private final"))]
    pub capturedArgs: Field<Vec<Object>>,
}

impl SerializedLambda {
    // java: <init>(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V
    pub fn new(capturingClass: Object, functionalInterfaceClass: String, functionalInterfaceMethodName: String, functionalInterfaceMethodSignature: String, implMethodKind: i32, implClass: String, implMethodName: String, implMethodSignature: String, instantiatedMethodType: String, capturedArgs: Vec<Object>) -> Result<Self> {
        let this = Self { capturingClass: Field::new(Default::default()), functionalInterfaceClass: Field::new(String::new()), functionalInterfaceMethodName: Field::new(String::new()), functionalInterfaceMethodSignature: Field::new(String::new()), implClass: Field::new(String::new()), implMethodName: Field::new(String::new()), implMethodSignature: Field::new(String::new()), implMethodKind: Field::new(0), instantiatedMethodType: Field::new(String::new()), capturedArgs: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.capturingClass.set(capturingClass);
        this.functionalInterfaceClass.set(functionalInterfaceClass);
        this.functionalInterfaceMethodName.set(functionalInterfaceMethodName);
        this.functionalInterfaceMethodSignature.set(functionalInterfaceMethodSignature);
        this.implMethodKind.set(implMethodKind);
        this.implClass.set(implClass);
        this.implMethodName.set(implMethodName);
        this.implMethodSignature.set(implMethodSignature);
        this.instantiatedMethodType.set(instantiatedMethodType);
        let _t0: Object = Objects::requireNonNull__obj(&capturedArgs)?;
        let _t1 = _t0.clone()?;
        this.capturedArgs.set(_t1);
        Ok(this)
    }

    // java: getCapturingClass()Ljava/lang/String;
    pub fn getCapturingClass(&self) -> Result<String> {
        let this = self;
        let _t0 = this.capturingClass.get().getName()?;
        let _t1 = _t0.replace(46i32, 47i32)?;
        Ok(_t1)
    }

    // java: getFunctionalInterfaceClass()Ljava/lang/String;
    pub fn getFunctionalInterfaceClass(&self) -> Result<String> {
        let this = self;
        Ok(this.functionalInterfaceClass.get())
    }

    // java: getFunctionalInterfaceMethodName()Ljava/lang/String;
    pub fn getFunctionalInterfaceMethodName(&self) -> Result<String> {
        let this = self;
        Ok(this.functionalInterfaceMethodName.get())
    }

    // java: getFunctionalInterfaceMethodSignature()Ljava/lang/String;
    pub fn getFunctionalInterfaceMethodSignature(&self) -> Result<String> {
        let this = self;
        Ok(this.functionalInterfaceMethodSignature.get())
    }

    // java: getImplClass()Ljava/lang/String;
    pub fn getImplClass(&self) -> Result<String> {
        let this = self;
        Ok(this.implClass.get())
    }

    // java: getImplMethodName()Ljava/lang/String;
    pub fn getImplMethodName(&self) -> Result<String> {
        let this = self;
        Ok(this.implMethodName.get())
    }

    // java: getImplMethodSignature()Ljava/lang/String;
    pub fn getImplMethodSignature(&self) -> Result<String> {
        let this = self;
        Ok(this.implMethodSignature.get())
    }

    // java: getImplMethodKind()I
    pub fn getImplMethodKind(&self) -> Result<i32> {
        let this = self;
        Ok(this.implMethodKind.get())
    }

    // java: getInstantiatedMethodType()Ljava/lang/String;
    pub fn getInstantiatedMethodType(&self) -> Result<String> {
        let this = self;
        Ok(this.instantiatedMethodType.get())
    }

    // java: getCapturedArgCount()I
    pub fn getCapturedArgCount(&self) -> Result<i32> {
        let this = self;
        Ok((this.capturedArgs.get().len() as i32))
    }

    // java: getCapturedArg(I)Ljava/lang/Object;
    pub fn getCapturedArg(&self, i: i32) -> Result<Object> {
        let this = self;
        Ok(this.capturedArgs.get()[i as usize].clone())
    }

    // java: readResolve()Ljava/lang/Object;
    pub fn readResolve(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = AccessController::doPrivileged(SerializedLambda_1::new(this)?)?;
        let mut deserialize: Object = _t0;
        /* TODO: aconst_null  */
        let mut _arr1: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr1[0i32 as usize] = this;
        let _t2 = todo!("stack underflow").invoke(deserialize, _arr1)?;
        return Ok(_t2);
        deserialize = todo!("stack underflow");
        return Err(JvmError::Custom("athrow".to_owned()));
        deserialize = todo!("stack underflow");
        let _t3 = deserialize.getException()?;
        let mut cause: Object = _t3;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0: String = MethodHandleInfo::referenceKindToString(this.implMethodKind.get())?;
        let mut implKind: String = _t0;
        let mut _arr1: Vec<Object> = Vec::with_capacity(15i32 as usize);
        _arr1[0i32 as usize] = String::from("capturingClass");
        _arr1[1i32 as usize] = this.capturingClass.get();
        _arr1[2i32 as usize] = String::from("functionalInterfaceMethod");
        _arr1[3i32 as usize] = this.functionalInterfaceClass.get();
        _arr1[4i32 as usize] = this.functionalInterfaceMethodName.get();
        _arr1[5i32 as usize] = this.functionalInterfaceMethodSignature.get();
        _arr1[6i32 as usize] = String::from("implementation");
        _arr1[7i32 as usize] = implKind;
        _arr1[8i32 as usize] = this.implClass.get();
        _arr1[9i32 as usize] = this.implMethodName.get();
        _arr1[10i32 as usize] = this.implMethodSignature.get();
        _arr1[11i32 as usize] = String::from("instantiatedMethodType");
        _arr1[12i32 as usize] = this.instantiatedMethodType.get();
        _arr1[13i32 as usize] = String::from("numCaptured");
        _arr1[14i32 as usize] = (this.capturedArgs.get().len() as i32);
        let _t2: String = String::format(String::from("SerializedLambda[%s=%s, %s=%s.%s:%s, %s=%s %s.%s:%s, %s=%s, %s=%d]"), &_arr1)?;
        Ok(_t2)
    }
}
