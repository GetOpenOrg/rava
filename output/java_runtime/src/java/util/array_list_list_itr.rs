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
    binary_name       = "java/util/ArrayList$ListItr",
    super_class       = "java/util/ArrayList$Itr",
    interfaces        = "java/util/ListIterator",
    access            = "package",
    modifiers         = "",
    generic_signature = "Ljava/util/ArrayList<TE;>.Itr;Ljava/util/ListIterator<TE;>;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "ArrayList.java",
    inner_classes     = "java/util/ArrayList$ListItr:java/util/ArrayList:ListItr:2;java/util/ArrayList$Itr:java/util/ArrayList:Itr:2",
    all_supertypes    = "java/lang/Object;java/util/ArrayList$Itr;java/util/ArrayList$ListItr;java/util/Iterator;java/util/ListIterator",
)]
#[derive(Clone, Default, PartialEq)]
pub struct ArrayList_ListItr {
    pub _super: ArrayList_Itr,
    #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/util/ArrayList;", access = "package", modifiers = "final synthetic", is_static = false))]
    pub this_0: JField<ArrayList<Object>>,
}

impl ArrayList_ListItr {
    pub fn as_array_list_itr(&self) -> &ArrayList_Itr { &self._super }
    pub fn into_array_list_itr(self) -> ArrayList_Itr { self._super }
}

impl From<ArrayList_ListItr> for ArrayList_Itr {
    fn from(v: ArrayList_ListItr) -> ArrayList_Itr { v._super }
}

impl ArrayList_ListItr {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/ArrayList;I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":4112;:0"))]
    pub fn new(mut arg_0: ArrayList<Object>, mut index: i32) -> Result<Self> {
        let mut this = Self { _super: Default::default(), this_0: JField::new(Default::default()), ..Default::default() };
        this.this_0.set(Clone::clone(&arg_0));
        this._super = ArrayList_Itr::new(Clone::clone(&arg_0))?;
        this._super.cursor.set(index);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "hasPrevious", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hasPrevious(&self) -> Result<bool> {
        panic!("stub: java/util/ArrayList$ListItr.hasPrevious:()Z")
    }

    #[cfg_attr(any(), java_method(name = "nextIndex", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn nextIndex(&self) -> Result<i32> {
        panic!("stub: java/util/ArrayList$ListItr.nextIndex:()I")
    }

    #[cfg_attr(any(), java_method(name = "previousIndex", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn previousIndex(&self) -> Result<i32> {
        panic!("stub: java/util/ArrayList$ListItr.previousIndex:()I")
    }

    #[cfg_attr(any(), java_method(name = "previous", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;"))]
    pub fn previous(&self) -> Result<Object> {
        panic!("stub: java/util/ArrayList$ListItr.previous:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "set", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V"))]
    pub fn set(&self, e: Object) -> Result<()> {
        panic!("stub: java/util/ArrayList$ListItr.set:(Ljava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V"))]
    pub fn add(&self, e: Object) -> Result<()> {
        panic!("stub: java/util/ArrayList$ListItr.add:(Ljava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn remove(&self) -> Result<()> {
        panic!("stub: java/util/ArrayList$ListItr.remove:()V")
    }

    #[cfg_attr(any(), java_method(name = "forEachRemaining", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TE;>;)V"))]
    pub fn forEachRemaining(&self, action: Object) -> Result<()> {
        panic!("stub: java/util/ArrayList$ListItr.forEachRemaining:(Ljava/util/function/Consumer;)V")
    }
}
