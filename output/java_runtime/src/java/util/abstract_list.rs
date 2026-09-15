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
    binary_name       = "java/util/AbstractList",
    super_class       = "java/util/AbstractCollection",
    interfaces        = "java/util/List",
    access            = "public",
    modifiers         = "abstract",
    generic_signature = "<E:Ljava/lang/Object;>Ljava/util/AbstractCollection<TE;>;Ljava/util/List<TE;>;",
    is_interface      = false,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "AbstractList.java",
    inner_classes     = "java/util/AbstractList$Itr:java/util/AbstractList:Itr:2;java/util/AbstractList$ListItr:java/util/AbstractList:ListItr:2;java/util/AbstractList$RandomAccessSubList:java/util/AbstractList:RandomAccessSubList:10;java/util/AbstractList$SubList:java/util/AbstractList:SubList:10;java/util/AbstractList$RandomAccessSpliterator:java/util/AbstractList:RandomAccessSpliterator:24;java/util/AbstractList$SubList$1:::0",
    all_supertypes    = "java/lang/Object;java/util/AbstractCollection;java/util/AbstractList;java/util/Collection;java/util/List",
)]
#[derive(Clone, Default, PartialEq)]
pub struct AbstractList<E: Clone + Default + 'static> {
    pub _super: AbstractCollection<E>,
    #[cfg_attr(any(), java_field(name = "modCount", descriptor = "I", access = "protected", modifiers = "transient", is_static = false))]
    pub modCount: JField<i32>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + Default + 'static> AbstractList<E> {
    pub fn as_abstract_collection(&self) -> &AbstractCollection<E> { &self._super }
    pub fn into_abstract_collection(self) -> AbstractCollection<E> { self._super }
}

impl<E: Clone + Default + 'static> From<AbstractList<E>> for AbstractCollection<E> {
    fn from(v: AbstractList<E>) -> AbstractCollection<E> { v._super }
}

impl<E: Clone + Default + 'static> AbstractList<E> {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        let mut this = Self { _super: Default::default(), modCount: JField::new(0), _phantom: std::marker::PhantomData, ..Default::default() };
        this._super = AbstractCollection::new()?;
        this.modCount.set(0i32);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)Z"))]
    pub fn add_obj(&self, e: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractList.add:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "(I)Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(I)TE;"))]
    pub fn get(&self, arg0: i32) -> Result<Object> {
        panic!("stub: java/util/AbstractList.get:(I)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "set", descriptor = "(ILjava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITE;)TE;"))]
    pub fn set(&self, index: i32, element: Object) -> Result<Object> {
        panic!("stub: java/util/AbstractList.set:(ILjava/lang/Object;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(ILjava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITE;)V"))]
    pub fn add_i_obj(&self, index: i32, element: Object) -> Result<()> {
        panic!("stub: java/util/AbstractList.add:(ILjava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(I)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)TE;"))]
    pub fn remove(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/AbstractList.remove:(I)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "(Ljava/lang/Object;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn indexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/AbstractList.indexOf:(Ljava/lang/Object;)I")
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/Object;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
        panic!("stub: java/util/AbstractList.lastIndexOf:(Ljava/lang/Object;)I")
    }

    #[cfg_attr(any(), java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/AbstractList.clear:()V")
    }

    #[cfg_attr(any(), java_method(name = "addAll", descriptor = "(ILjava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/util/Collection<+TE;>;)Z"))]
    pub fn addAll(&self, index: i32, c: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractList.addAll:(ILjava/util/Collection;)Z")
    }

    #[cfg_attr(any(), java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TE;>;"))]
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/AbstractList.iterator:()Ljava/util/Iterator;")
    }

    #[cfg_attr(any(), java_method(name = "listIterator", descriptor = "()Ljava/util/ListIterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/ListIterator<TE;>;"))]
    pub fn listIterator(&self) -> Result<Object> {
        panic!("stub: java/util/AbstractList.listIterator:()Ljava/util/ListIterator;")
    }

    #[cfg_attr(any(), java_method(name = "listIterator", descriptor = "(I)Ljava/util/ListIterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)Ljava/util/ListIterator<TE;>;"))]
    pub fn listIterator_i(&self, index: i32) -> Result<Object> {
        panic!("stub: java/util/AbstractList.listIterator:(I)Ljava/util/ListIterator;")
    }

    #[cfg_attr(any(), java_method(name = "subList", descriptor = "(II)Ljava/util/List;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(II)Ljava/util/List<TE;>;"))]
    pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
        panic!("stub: java/util/AbstractList.subList:(II)Ljava/util/List;")
    }

    #[cfg_attr(any(), java_method(name = "subListRangeCheck", descriptor = "(III)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn subListRangeCheck(fromIndex: i32, toIndex: i32, size: i32) -> Result<()> {
        panic!("stub: java/util/AbstractList.subListRangeCheck:(III)V")
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn equals(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractList.equals:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/AbstractList.hashCode:()I")
    }

    #[cfg_attr(any(), java_method(name = "removeRange", descriptor = "(II)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn removeRange(&self, fromIndex: i32, toIndex: i32) -> Result<()> {
        panic!("stub: java/util/AbstractList.removeRange:(II)V")
    }

    #[cfg_attr(any(), java_method(name = "rangeCheckForAdd", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn rangeCheckForAdd(&self, index: i32) -> Result<()> {
        panic!("stub: java/util/AbstractList.rangeCheckForAdd:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "outOfBoundsMsg", descriptor = "(I)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn outOfBoundsMsg(&self, index: i32) -> Result<String> {
        panic!("stub: java/util/AbstractList.outOfBoundsMsg:(I)Ljava/lang/String;")
    }
}
