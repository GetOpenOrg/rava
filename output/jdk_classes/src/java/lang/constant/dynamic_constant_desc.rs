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
        panic!("stub: java/lang/constant/DynamicConstantDesc.<init>:(Ljava/lang/constant/DirectMethodHandleDesc;Ljava/lang/String;Ljava/lang/constant/ClassDesc;[Ljava/lang/constant/ConstantDesc;)V")
    }

    // java: ofCanonical(Ljava/lang/constant/DirectMethodHandleDesc;Ljava/lang/String;Ljava/lang/constant/ClassDesc;[Ljava/lang/constant/ConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn ofCanonical(bootstrapMethod: Object, constantName: String, constantType: Object, bootstrapArgs: Vec<Object>) -> Result<Object> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.ofCanonical:(Ljava/lang/constant/DirectMethodHandleDesc;Ljava/lang/String;Ljava/lang/constant/ClassDesc;[Ljava/lang/constant/ConstantDesc;)Ljava/lang/constant/ConstantDesc;")
    }

    // java: ofNamed(Ljava/lang/constant/DirectMethodHandleDesc;Ljava/lang/String;Ljava/lang/constant/ClassDesc;[Ljava/lang/constant/ConstantDesc;)Ljava/lang/constant/DynamicConstantDesc;
    pub fn ofNamed(bootstrapMethod: Object, constantName: String, constantType: Object, bootstrapArgs: Vec<Object>) -> Result<Object> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.ofNamed:(Ljava/lang/constant/DirectMethodHandleDesc;Ljava/lang/String;Ljava/lang/constant/ClassDesc;[Ljava/lang/constant/ConstantDesc;)Ljava/lang/constant/DynamicConstantDesc;")
    }

    // java: of(Ljava/lang/constant/DirectMethodHandleDesc;[Ljava/lang/constant/ConstantDesc;)Ljava/lang/constant/DynamicConstantDesc;
    pub fn of__direct_arr_con(bootstrapMethod: Object, bootstrapArgs: Vec<Object>) -> Result<Object> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.of:(Ljava/lang/constant/DirectMethodHandleDesc;[Ljava/lang/constant/ConstantDesc;)Ljava/lang/constant/DynamicConstantDesc;")
    }

    // java: of(Ljava/lang/constant/DirectMethodHandleDesc;)Ljava/lang/constant/DynamicConstantDesc;
    pub fn of__direct(bootstrapMethod: Object) -> Result<Object> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.of:(Ljava/lang/constant/DirectMethodHandleDesc;)Ljava/lang/constant/DynamicConstantDesc;")
    }

    // java: constantName()Ljava/lang/String;
    pub fn constantName(&self) -> Result<String> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.constantName:()Ljava/lang/String;")
    }

    // java: constantType()Ljava/lang/constant/ClassDesc;
    pub fn constantType(&self) -> Result<Object> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.constantType:()Ljava/lang/constant/ClassDesc;")
    }

    // java: bootstrapMethod()Ljava/lang/constant/DirectMethodHandleDesc;
    pub fn bootstrapMethod(&self) -> Result<Object> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.bootstrapMethod:()Ljava/lang/constant/DirectMethodHandleDesc;")
    }

    // java: bootstrapArgs()[Ljava/lang/constant/ConstantDesc;
    pub fn bootstrapArgs(&self) -> Result<Vec<Object>> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.bootstrapArgs:()[Ljava/lang/constant/ConstantDesc;")
    }

    // java: bootstrapArgsList()Ljava/util/List;
    pub fn bootstrapArgsList(&self) -> Result<Object> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.bootstrapArgsList:()Ljava/util/List;")
    }

    // java: resolveConstantDesc(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Object;
    pub fn resolveConstantDesc(&self, lookup: Object) -> Result<Object> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.resolveConstantDesc:(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Object;")
    }

    // java: tryCanonicalize()Ljava/lang/constant/ConstantDesc;
    pub fn tryCanonicalize(&self) -> Result<Object> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.tryCanonicalize:()Ljava/lang/constant/ConstantDesc;")
    }

    // java: canonicalizeNull(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn canonicalizeNull(desc: Object) -> Result<Object> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.canonicalizeNull:(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;")
    }

    // java: canonicalizeEnum(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn canonicalizeEnum(desc: Object) -> Result<Object> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.canonicalizeEnum:(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;")
    }

    // java: canonicalizePrimitiveClass(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn canonicalizePrimitiveClass(desc: Object) -> Result<Object> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.canonicalizePrimitiveClass:(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;")
    }

    // java: canonicalizeStaticFieldVarHandle(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn canonicalizeStaticFieldVarHandle(desc: Object) -> Result<Object> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.canonicalizeStaticFieldVarHandle:(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;")
    }

    // java: canonicalizeFieldVarHandle(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn canonicalizeFieldVarHandle(desc: Object) -> Result<Object> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.canonicalizeFieldVarHandle:(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;")
    }

    // java: canonicalizeArrayVarHandle(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;
    pub fn canonicalizeArrayVarHandle(desc: Object) -> Result<Object> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.canonicalizeArrayVarHandle:(Ljava/lang/constant/DynamicConstantDesc;)Ljava/lang/constant/ConstantDesc;")
    }

    // java: equals(Ljava/lang/Object;)Z
    pub fn equals(&self, o: Object) -> Result<bool> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.equals:(Ljava/lang/Object;)Z")
    }

    // java: hashCode()I
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.hashCode:()I")
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/lang/constant/DynamicConstantDesc.toString:()Ljava/lang/String;")
    }
}
