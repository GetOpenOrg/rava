#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/Enum",
    super_class       = "java/lang/Object",
    interfaces        = "java/lang/constant/Constable,java/lang/Comparable,java/io/Serializable",
    access            = "public",
    modifiers         = "abstract",
    generic_signature = "<E:Ljava/lang/Enum<TE;>;>Ljava/lang/Object;Ljava/lang/constant/Constable;Ljava/lang/Comparable<TE;>;Ljava/io/Serializable;",
    is_interface      = false,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Enum.java",
    inner_classes     = "java/lang/Enum$EnumDesc:java/lang/Enum:EnumDesc:25;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25",
    all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Enum;java/lang/Object;java/lang/constant/Constable",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Enum<E: Clone + Default + 'static> {
    #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
    pub name: JField<String>,
    #[cfg_attr(any(), java_field(name = "ordinal", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
    pub ordinal: JField<i32>,
    #[cfg_attr(any(), java_field(name = "hash", descriptor = "I", access = "private", modifiers = "", is_static = false))]
    pub hash: JField<i32>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + Default + 'static> From<Enum<E>> for Comparable<E> {
    fn from(v: Enum<E>) -> Comparable<E> { Default::default() }
}

impl<E: Clone + Default + 'static> Enum<E> {
    #[cfg_attr(any(), java_method(name = "name", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn name(&self) -> Result<String> {
        panic!("stub: java/lang/Enum.name:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "ordinal", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn ordinal(&self) -> Result<i32> {
        panic!("stub: java/lang/Enum.ordinal:()I")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;I)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new(name: String, ordinal: i32) -> Result<Self> {
        panic!("stub: java/lang/Enum.<init>:(Ljava/lang/String;I)V")
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/lang/Enum.toString:()Ljava/lang/String;")
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn equals(&self, other: Object) -> Result<bool> {
        panic!("stub: java/lang/Enum.equals:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/lang/Enum.hashCode:()I")
    }

    #[cfg_attr(any(), java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "protected", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/CloneNotSupportedException"))]
    pub fn clone(&self) -> Result<Object> {
        panic!("stub: java/lang/Enum.clone:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "(Ljava/lang/Enum;)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)I"))]
    pub fn compareTo(&self, o: Enum<Object>) -> Result<i32> {
        panic!("stub: java/lang/Enum.compareTo:(Ljava/lang/Enum;)I")
    }

    #[cfg_attr(any(), java_method(name = "getDeclaringClass", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<TE;>;"))]
    pub fn getDeclaringClass(&self) -> Result<Object> {
        panic!("stub: java/lang/Enum.getDeclaringClass:()Ljava/lang/Class;")
    }

    #[cfg_attr(any(), java_method(name = "describeConstable", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Optional<Ljava/lang/Enum$EnumDesc<TE;>;>;"))]
    pub fn describeConstable(&self) -> Result<Object> {
        panic!("stub: java/lang/Enum.describeConstable:()Ljava/util/Optional;")
    }

    #[cfg_attr(any(), java_method(name = "valueOf", descriptor = "(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Enum<TT;>;>(Ljava/lang/Class<TT;>;Ljava/lang/String;)TT;"))]
    pub fn valueOf(enumClass: Object, name: String) -> Result<Enum<Object>> {
        panic!("stub: java/lang/Enum.valueOf:(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;")
    }

    #[cfg_attr(any(), java_method(name = "finalize", descriptor = "()V", access = "protected", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true))]
    pub fn finalize(&self) -> Result<()> {
        panic!("stub: java/lang/Enum.finalize:()V")
    }

    #[cfg_attr(any(), java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException"))]
    pub fn readObject(&self, in_: Object) -> Result<()> {
        panic!("stub: java/lang/Enum.readObject:(Ljava/io/ObjectInputStream;)V")
    }

    #[cfg_attr(any(), java_method(name = "readObjectNoData", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/ObjectStreamException"))]
    pub fn readObjectNoData(&self) -> Result<()> {
        panic!("stub: java/lang/Enum.readObjectNoData:()V")
    }
}
