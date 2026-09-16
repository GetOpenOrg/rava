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
    binary_name       = "java/util/AbstractList$ListItr",
    super_class       = "java/util/AbstractList$Itr",
    interfaces        = "java/util/ListIterator",
    access            = "package",
    modifiers         = "",
    generic_signature = "Ljava/util/AbstractList<TE;>.Itr;Ljava/util/ListIterator<TE;>;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "AbstractList.java",
    inner_classes     = "java/util/AbstractList$ListItr:java/util/AbstractList:ListItr:2;java/util/AbstractList$Itr:java/util/AbstractList:Itr:2",
    all_supertypes    = "java/lang/Object;java/util/AbstractList$Itr;java/util/AbstractList$ListItr;java/util/Iterator;java/util/ListIterator",
)]
#[derive(Clone, Default, PartialEq)]
pub struct AbstractList_ListItr {
    pub _super: AbstractList_Itr,
    #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/util/AbstractList;", access = "package", modifiers = "final synthetic", is_static = false))]
    pub this_0: JField<AbstractList<Object>>,
}

impl AbstractList_ListItr {
    pub fn as_abstract_list_itr(&self) -> &AbstractList_Itr { &self._super }
    pub fn into_abstract_list_itr(self) -> AbstractList_Itr { self._super }
}

impl From<AbstractList_ListItr> for AbstractList_Itr {
    fn from(v: AbstractList_ListItr) -> AbstractList_Itr { v._super }
}

impl AbstractList_ListItr {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/AbstractList;I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":4112;:0"))]
    pub fn new(mut arg_0: AbstractList<Object>, mut index: i32) -> Result<Self> {
        let mut this = Self { _super: Default::default(), this_0: JField::new(Default::default()), ..Default::default() };
        this.this_0.set(Clone::clone(&arg_0));
        this._super = AbstractList_Itr::new(Clone::clone(&arg_0))?;
        this._super.cursor.set(index);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "hasPrevious", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hasPrevious(&self) -> Result<bool> {
        panic!("stub: java/util/AbstractList$ListItr.hasPrevious:()Z")
    }

    #[cfg_attr(any(), java_method(name = "previous", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;"))]
    pub fn previous(&self) -> Result<Object> {
        panic!("stub: java/util/AbstractList$ListItr.previous:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "nextIndex", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn nextIndex(&self) -> Result<i32> {
        panic!("stub: java/util/AbstractList$ListItr.nextIndex:()I")
    }

    #[cfg_attr(any(), java_method(name = "previousIndex", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn previousIndex(&self) -> Result<i32> {
        panic!("stub: java/util/AbstractList$ListItr.previousIndex:()I")
    }

    #[cfg_attr(any(), java_method(name = "set", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V"))]
    pub fn set(&self, e: Object) -> Result<()> {
        panic!("stub: java/util/AbstractList$ListItr.set:(Ljava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V"))]
    pub fn add(&self, e: Object) -> Result<()> {
        panic!("stub: java/util/AbstractList$ListItr.add:(Ljava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn remove(&self) -> Result<()> {
        panic!("stub: java/util/AbstractList$ListItr.remove:()V")
    }

    #[cfg_attr(any(), java_method(name = "forEachRemaining", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TE;>;)V"))]
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/AbstractList$ListItr.forEachRemaining:(Ljava/util/function/Consumer;)V")
    }
}
