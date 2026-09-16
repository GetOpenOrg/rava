#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::java::util::function::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/util/AbstractList$Itr",
    super_class       = "java/lang/Object",
    interfaces        = "java/util/Iterator",
    access            = "package",
    modifiers         = "",
    generic_signature = "Ljava/lang/Object;Ljava/util/Iterator<TE;>;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "AbstractList.java",
    inner_classes     = "java/util/AbstractList$Itr:java/util/AbstractList:Itr:2",
    all_supertypes    = "java/lang/Object;java/util/AbstractList$Itr;java/util/Iterator",
)]
#[derive(Clone, Default, PartialEq)]
pub struct AbstractList_Itr {
    #[cfg_attr(any(), java_field(name = "cursor", descriptor = "I", is_static = false))]
    pub cursor: JField<i32>,
    #[cfg_attr(any(), java_field(name = "lastRet", descriptor = "I", is_static = false))]
    pub lastRet: JField<i32>,
    #[cfg_attr(any(), java_field(name = "expectedModCount", descriptor = "I", is_static = false))]
    pub expectedModCount: JField<i32>,
    #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/util/AbstractList;", access = "package", modifiers = "final synthetic", is_static = false))]
    pub this_0: JField<AbstractList<Object>>,
}

impl AbstractList_Itr {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/AbstractList;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":4112"))]
    pub fn new(mut arg_0: AbstractList<Object>) -> Result<Self> {
        let mut this = Self { cursor: JField::new(0), lastRet: JField::new(0), expectedModCount: JField::new(0), this_0: JField::new(Default::default()), ..Default::default() };
        this.this_0.set(Clone::clone(&arg_0));
        /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
        this.cursor.set(0i32);
        this.lastRet.set(-1i32);
        this.expectedModCount.set(this.this_0.get().modCount.get());
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "hasNext", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hasNext(&self) -> Result<bool> {
        panic!("stub: java/util/AbstractList$Itr.hasNext:()Z")
    }

    #[cfg_attr(any(), java_method(name = "next", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;"))]
    pub fn next(&self) -> Result<Object> {
        panic!("stub: java/util/AbstractList$Itr.next:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn remove(&self) -> Result<()> {
        panic!("stub: java/util/AbstractList$Itr.remove:()V")
    }

    #[cfg_attr(any(), java_method(name = "checkForComodification", descriptor = "()V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn checkForComodification(&self) -> Result<()> {
        panic!("stub: java/util/AbstractList$Itr.checkForComodification:()V")
    }

    #[cfg_attr(any(), java_method(name = "forEachRemaining", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TE;>;)V"))]
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/AbstractList$Itr.forEachRemaining:(Ljava/util/function/Consumer;)V")
    }
}
