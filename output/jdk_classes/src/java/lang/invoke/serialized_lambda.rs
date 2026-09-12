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
    pub fn new(&self, capturingClass: Object, functionalInterfaceClass: String, functionalInterfaceMethodName: String, functionalInterfaceMethodSignature: String, implMethodKind: i32, implClass: String, implMethodName: String, implMethodSignature: String, instantiatedMethodType: String, capturedArgs: Vec<Object>) -> Result<()> {
        todo!("abstract java/lang/invoke/SerializedLambda.<init>")
    }

    // java: getCapturingClass()Ljava/lang/String;
    pub fn getCapturingClass(&self) -> Result<String> {
        todo!("abstract java/lang/invoke/SerializedLambda.getCapturingClass")
    }

    // java: getFunctionalInterfaceClass()Ljava/lang/String;
    pub fn getFunctionalInterfaceClass(&self) -> Result<String> {
        todo!("abstract java/lang/invoke/SerializedLambda.getFunctionalInterfaceClass")
    }

    // java: getFunctionalInterfaceMethodName()Ljava/lang/String;
    pub fn getFunctionalInterfaceMethodName(&self) -> Result<String> {
        todo!("abstract java/lang/invoke/SerializedLambda.getFunctionalInterfaceMethodName")
    }

    // java: getFunctionalInterfaceMethodSignature()Ljava/lang/String;
    pub fn getFunctionalInterfaceMethodSignature(&self) -> Result<String> {
        todo!("abstract java/lang/invoke/SerializedLambda.getFunctionalInterfaceMethodSignature")
    }

    // java: getImplClass()Ljava/lang/String;
    pub fn getImplClass(&self) -> Result<String> {
        todo!("abstract java/lang/invoke/SerializedLambda.getImplClass")
    }

    // java: getImplMethodName()Ljava/lang/String;
    pub fn getImplMethodName(&self) -> Result<String> {
        todo!("abstract java/lang/invoke/SerializedLambda.getImplMethodName")
    }

    // java: getImplMethodSignature()Ljava/lang/String;
    pub fn getImplMethodSignature(&self) -> Result<String> {
        todo!("abstract java/lang/invoke/SerializedLambda.getImplMethodSignature")
    }

    // java: getImplMethodKind()I
    pub fn getImplMethodKind(&self) -> Result<i32> {
        todo!("abstract java/lang/invoke/SerializedLambda.getImplMethodKind")
    }

    // java: getInstantiatedMethodType()Ljava/lang/String;
    pub fn getInstantiatedMethodType(&self) -> Result<String> {
        todo!("abstract java/lang/invoke/SerializedLambda.getInstantiatedMethodType")
    }

    // java: getCapturedArgCount()I
    pub fn getCapturedArgCount(&self) -> Result<i32> {
        todo!("abstract java/lang/invoke/SerializedLambda.getCapturedArgCount")
    }

    // java: getCapturedArg(I)Ljava/lang/Object;
    pub fn getCapturedArg(&self, i: i32) -> Result<Object> {
        todo!("abstract java/lang/invoke/SerializedLambda.getCapturedArg")
    }

    // java: readResolve()Ljava/lang/Object;
    pub fn readResolve(&self) -> Result<Object> {
        todo!("abstract java/lang/invoke/SerializedLambda.readResolve")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/lang/invoke/SerializedLambda.toString")
    }
}
