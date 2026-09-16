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
    binary_name       = "java/util/Collection",
    super_class       = "java/lang/Object",
    interfaces        = "java/lang/Iterable",
    access            = "public",
    modifiers         = "abstract interface",
    generic_signature = "<E:Ljava/lang/Object;>Ljava/lang/Object;Ljava/lang/Iterable<TE;>;",
    is_interface      = true,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Collection.java",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Collection<E: Clone + Default + 'static>(std::marker::PhantomData<E>);

impl<E: Clone + Default + 'static> Collection<E> {
    #[cfg_attr(any(), java_method(name = "size", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/Collection.size:()I")
    }

    #[cfg_attr(any(), java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/Collection.isEmpty:()Z")
    }

    #[cfg_attr(any(), java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn contains(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Collection.contains:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TE;>;"))]
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collection.iterator:()Ljava/util/Iterator;")
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn toArray(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/util/Collection.toArray:()[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;"))]
    pub fn toArray_arr_obj(&self, arg0: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/util/Collection.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "(Ljava/util/function/IntFunction;)[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/IntFunction<[TT;>;)[TT;"))]
    pub fn toArray_intfun(&self, generator: Object) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/util/Collection.toArray:(Ljava/util/function/IntFunction;)[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(TE;)Z"))]
    pub fn add(&self, arg0: E) -> Result<bool> {
        panic!("stub: java/util/Collection.add:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn remove(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Collection.remove:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "containsAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z"))]
    pub fn containsAll(&self, arg0: Collection<Object>) -> Result<bool> {
        panic!("stub: java/util/Collection.containsAll:(Ljava/util/Collection;)Z")
    }

    #[cfg_attr(any(), java_method(name = "addAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/Collection<+TE;>;)Z"))]
    pub fn addAll(&self, arg0: Collection<E>) -> Result<bool> {
        panic!("stub: java/util/Collection.addAll:(Ljava/util/Collection;)Z")
    }

    #[cfg_attr(any(), java_method(name = "removeAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z"))]
    pub fn removeAll(&self, arg0: Collection<Object>) -> Result<bool> {
        panic!("stub: java/util/Collection.removeAll:(Ljava/util/Collection;)Z")
    }

    #[cfg_attr(any(), java_method(name = "removeIf", descriptor = "(Ljava/util/function/Predicate;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TE;>;)Z"))]
    pub fn removeIf(&self, filter: Predicate<E>) -> Result<bool> {
        panic!("stub: java/util/Collection.removeIf:(Ljava/util/function/Predicate;)Z")
    }

    #[cfg_attr(any(), java_method(name = "retainAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z"))]
    pub fn retainAll(&self, arg0: Collection<Object>) -> Result<bool> {
        panic!("stub: java/util/Collection.retainAll:(Ljava/util/Collection;)Z")
    }

    #[cfg_attr(any(), java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/Collection.clear:()V")
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn equals(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Collection.equals:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/Collection.hashCode:()I")
    }

    #[cfg_attr(any(), java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TE;>;"))]
    pub fn spliterator(&self) -> Result<Object> {
        panic!("stub: java/util/Collection.spliterator:()Ljava/util/Spliterator;")
    }

    #[cfg_attr(any(), java_method(name = "stream", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<TE;>;"))]
    pub fn stream(&self) -> Result<Object> {
        panic!("stub: java/util/Collection.stream:()Ljava/util/stream/Stream;")
    }

    #[cfg_attr(any(), java_method(name = "parallelStream", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<TE;>;"))]
    pub fn parallelStream(&self) -> Result<Object> {
        panic!("stub: java/util/Collection.parallelStream:()Ljava/util/stream/Stream;")
    }
}
