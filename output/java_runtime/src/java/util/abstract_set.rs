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
    binary_name       = "java/util/AbstractSet",
    super_class       = "java/util/AbstractCollection",
    interfaces        = "java/util/Set",
    access            = "public",
    modifiers         = "abstract",
    generic_signature = "<E:Ljava/lang/Object;>Ljava/util/AbstractCollection<TE;>;Ljava/util/Set<TE;>;",
    is_interface      = false,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "AbstractSet.java",
    all_supertypes    = "java/lang/Object;java/util/AbstractCollection;java/util/AbstractSet;java/util/Collection;java/util/Set",
)]
#[derive(Clone, Default, PartialEq)]
pub struct AbstractSet<E: Clone + Default + 'static> {
    pub _super: AbstractCollection<E>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + Default + 'static> AbstractSet<E> {
    pub fn as_abstract_collection(&self) -> &AbstractCollection<E> { &self._super }
    pub fn into_abstract_collection(self) -> AbstractCollection<E> { self._super }
}

impl<E: Clone + Default + 'static> From<AbstractSet<E>> for AbstractCollection<E> {
    fn from(v: AbstractSet<E>) -> AbstractCollection<E> { v._super }
}

impl<E: Clone + Default + 'static> AbstractSet<E> {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        let mut this = Self { _super: Default::default(), _phantom: std::marker::PhantomData, ..Default::default() };
        this._super = AbstractCollection::new()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn equals(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractSet.equals:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/AbstractSet.hashCode:()I")
    }

    #[cfg_attr(any(), java_method(name = "removeAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z"))]
    pub fn removeAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractSet.removeAll:(Ljava/util/Collection;)Z")
    }
}
