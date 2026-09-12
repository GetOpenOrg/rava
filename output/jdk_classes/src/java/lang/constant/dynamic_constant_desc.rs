#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/constant/DynamicConstantDesc",
    super_class = "java/lang/Object",
    interfaces  = "java/lang/constant/ConstantDesc",
    access      = "public abstract",
    source      = "DynamicConstantDesc.java",
))]
pub struct DynamicConstantDesc<T> {
    #[cfg_attr(any(), java_field(name = "bootstrapMethod", descriptor = "Ljava/lang/constant/DirectMethodHandleDesc;", access = "private final"))]
    pub bootstrapMethod: Field<Object>,
    #[cfg_attr(any(), java_field(name = "bootstrapArgs", descriptor = "[Ljava/lang/constant/ConstantDesc;", access = "private final"))]
    pub bootstrapArgs: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "constantName", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub constantName: Field<String>,
    #[cfg_attr(any(), java_field(name = "constantType", descriptor = "Ljava/lang/constant/ClassDesc;", access = "private final"))]
    pub constantType: Field<Object>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + 'static> DynamicConstantDesc<T> {
    // java: <init>(Ljava/lang/constant/DirectMethodHandleDesc;Ljava/lang/String;Ljava/lang/constant/ClassDesc;[Ljava/lang/constant/ConstantDesc;)V
    pub fn new(&self, bootstrapMethod: Object, constantName: String, constantType: Object, bootstrapArgs: Vec<Object>) -> Result<()> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.<init>")
    }

    // java: ofCanonical(Ljava/lang/constant/DirectMethodHandleDesc;Ljava/lang/String;Ljava/lang/constant/ClassDesc;[Ljava/lang/constant/ConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn ofCanonical(bootstrapMethod: Object, constantName: String, constantType: Object, bootstrapArgs: Vec<Object>) -> Result<Object> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.ofCanonical")
    }

    // java: ofNamed(Ljava/lang/constant/DirectMethodHandleDesc;Ljava/lang/String;Ljava/lang/constant/ClassDesc;[Ljava/lang/constant/ConstantDesc;)Ljava/lang/constant/DynamicConstantDesc;
    pub fn ofNamed(bootstrapMethod: Object, constantName: String, constantType: Object, bootstrapArgs: Vec<Object>) -> Result<Object> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.ofNamed")
    }

    // java: of(Ljava/lang/constant/DirectMethodHandleDesc;[Ljava/lang/constant/ConstantDesc;)Ljava/lang/constant/DynamicConstantDesc;
    pub fn of__direct_arr_con(bootstrapMethod: Object, bootstrapArgs: Vec<Object>) -> Result<Object> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.of")
    }

    // java: of(Ljava/lang/constant/DirectMethodHandleDesc;)Ljava/lang/constant/DynamicConstantDesc;
    pub fn of__direct(bootstrapMethod: Object) -> Result<Object> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.of")
    }

    // java: constantName()Ljava/lang/String;
    pub fn constantName(&self) -> Result<String> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.constantName")
    }

    // java: constantType()Ljava/lang/constant/ClassDesc;
    pub fn constantType(&self) -> Result<Object> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.constantType")
    }

    // java: bootstrapMethod()Ljava/lang/constant/DirectMethodHandleDesc;
    pub fn bootstrapMethod(&self) -> Result<Object> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.bootstrapMethod")
    }

    // java: bootstrapArgs()[Ljava/lang/constant/ConstantDesc;
    pub fn bootstrapArgs(&self) -> Result<Vec<Object>> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.bootstrapArgs")
    }

    // java: bootstrapArgsList()Ljava/util/List;
    pub fn bootstrapArgsList(&self) -> Result<Object> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.bootstrapArgsList")
    }

    // java: resolveConstantDesc(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Object;
    pub fn resolveConstantDesc(&self, lookup: Object) -> Result<Object> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.resolveConstantDesc")
    }

    // java: tryCanonicalize()Ljava/lang/constant/ConstantDesc;
    pub fn tryCanonicalize(&self) -> Result<Object> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.tryCanonicalize")
    }

    // java: canonicalizeNull(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn canonicalizeNull(desc: Object) -> Result<Object> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.canonicalizeNull")
    }

    // java: canonicalizeEnum(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn canonicalizeEnum(desc: Object) -> Result<Object> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.canonicalizeEnum")
    }

    // java: canonicalizePrimitiveClass(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn canonicalizePrimitiveClass(desc: Object) -> Result<Object> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.canonicalizePrimitiveClass")
    }

    // java: canonicalizeStaticFieldVarHandle(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn canonicalizeStaticFieldVarHandle(desc: Object) -> Result<Object> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.canonicalizeStaticFieldVarHandle")
    }

    // java: canonicalizeFieldVarHandle(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn canonicalizeFieldVarHandle(desc: Object) -> Result<Object> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.canonicalizeFieldVarHandle")
    }

    // java: canonicalizeArrayVarHandle(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn canonicalizeArrayVarHandle(desc: Object) -> Result<Object> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.canonicalizeArrayVarHandle")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.equals")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.hashCode")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/lang/constant/DynamicConstantDesc.toString")
    }
}
