#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;
use crate::jdk::internal::util::ArraysSupport;

#[java_rta_macros::java_class(
    binary_name       = "java/util/AbstractCollection",
    super_class       = "java/lang/Object",
    interfaces        = "java/util/Collection",
    access            = "public",
    modifiers         = "abstract",
    generic_signature = "<E:Ljava/lang/Object;>Ljava/lang/Object;Ljava/util/Collection<TE;>;",
    is_interface      = false,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "AbstractCollection.java",
)]
#[derive(Clone, Default, PartialEq)]
pub struct AbstractCollection<E: Clone + Default + 'static>(std::marker::PhantomData<E>);

impl<E: Clone + Default + 'static> AbstractCollection<E> {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        let mut this = Self(std::marker::PhantomData);
        /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TE;>;"))]
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/AbstractCollection.iterator:()Ljava/util/Iterator;")
    }

    #[cfg_attr(any(), java_method(name = "size", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/AbstractCollection.size:()I")
    }

    #[cfg_attr(any(), java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/AbstractCollection.isEmpty:()Z")
    }

    #[cfg_attr(any(), java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn contains(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractCollection.contains:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toArray(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/util/AbstractCollection.toArray:()[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;"))]
    pub fn toArray_arr_obj(&self, a: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/util/AbstractCollection.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "finishToArray", descriptor = "([Ljava/lang/Object;Ljava/util/Iterator;)[Ljava/lang/Object;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;Ljava/util/Iterator<*>;)[TT;"))]
    pub fn finishToArray(r: Rc<RefCell<Vec<Object>>>, it: Object) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/util/AbstractCollection.finishToArray:([Ljava/lang/Object;Ljava/util/Iterator;)[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)Z"))]
    pub fn add(&self, e: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractCollection.add:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn remove(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractCollection.remove:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "containsAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z"))]
    pub fn containsAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractCollection.containsAll:(Ljava/util/Collection;)Z")
    }

    #[cfg_attr(any(), java_method(name = "addAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<+TE;>;)Z"))]
    pub fn addAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractCollection.addAll:(Ljava/util/Collection;)Z")
    }

    #[cfg_attr(any(), java_method(name = "removeAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z"))]
    pub fn removeAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractCollection.removeAll:(Ljava/util/Collection;)Z")
    }

    #[cfg_attr(any(), java_method(name = "retainAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z"))]
    pub fn retainAll(&self, c: Object) -> Result<bool> {
        panic!("stub: java/util/AbstractCollection.retainAll:(Ljava/util/Collection;)Z")
    }

    #[cfg_attr(any(), java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/AbstractCollection.clear:()V")
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/util/AbstractCollection.toString:()Ljava/lang/String;")
    }
}
