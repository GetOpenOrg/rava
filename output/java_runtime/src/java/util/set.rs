#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::java::util::stream::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/util/Set",
    super_class       = "java/lang/Object",
    interfaces        = "java/util/Collection",
    access            = "public",
    modifiers         = "abstract interface",
    generic_signature = "<E:Ljava/lang/Object;>Ljava/lang/Object;Ljava/util/Collection<TE;>;",
    is_interface      = true,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Set.java",
    inner_classes     = "java/util/ImmutableCollections$SetN:java/util/ImmutableCollections:SetN:24;java/util/ImmutableCollections$Set12:java/util/ImmutableCollections:Set12:24;java/util/ImmutableCollections$AbstractImmutableSet:java/util/ImmutableCollections:AbstractImmutableSet:1032",
)]
#[derive(Clone, Default, PartialEq)]
pub struct Set<E: Clone + Default + 'static>(std::marker::PhantomData<E>);

impl<E: Clone + Default + 'static> Set<E> {
    #[cfg_attr(any(), java_method(name = "size", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/Set.size:()I")
    }

    #[cfg_attr(any(), java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/Set.isEmpty:()Z")
    }

    #[cfg_attr(any(), java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn contains(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Set.contains:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TE;>;"))]
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/Set.iterator:()Ljava/util/Iterator;")
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn toArray(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/util/Set.toArray:()[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;"))]
    pub fn toArray_arr_obj(&self, arg0: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/util/Set.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(TE;)Z"))]
    pub fn add(&self, arg0: E) -> Result<bool> {
        panic!("stub: java/util/Set.add:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn remove(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Set.remove:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "containsAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z"))]
    pub fn containsAll(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Set.containsAll:(Ljava/util/Collection;)Z")
    }

    #[cfg_attr(any(), java_method(name = "addAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/Collection<+TE;>;)Z"))]
    pub fn addAll(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Set.addAll:(Ljava/util/Collection;)Z")
    }

    #[cfg_attr(any(), java_method(name = "retainAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z"))]
    pub fn retainAll(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Set.retainAll:(Ljava/util/Collection;)Z")
    }

    #[cfg_attr(any(), java_method(name = "removeAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z"))]
    pub fn removeAll(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Set.removeAll:(Ljava/util/Collection;)Z")
    }

    #[cfg_attr(any(), java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/Set.clear:()V")
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn equals(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/Set.equals:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/Set.hashCode:()I")
    }

    #[cfg_attr(any(), java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TE;>;"))]
    pub fn spliterator(&self) -> Result<Object> {
        panic!("stub: java/util/Set.spliterator:()Ljava/util/Spliterator;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>()Ljava/util/Set<TE;>;"))]
    pub fn of() -> Result<Set<Object>> {
        panic!("stub: java/util/Set.of:()Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;)Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;)Ljava/util/Set<TE;>;"))]
    pub fn of_obj(e1: E) -> Result<Set<Object>> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;)Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;TE;)Ljava/util/Set<TE;>;"))]
    pub fn of_obj_obj(e1: E, e2: E) -> Result<Set<Object>> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;TE;TE;)Ljava/util/Set<TE;>;"))]
    pub fn of_obj_obj_obj(e1: E, e2: E, e3: E) -> Result<Set<Object>> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;TE;TE;TE;)Ljava/util/Set<TE;>;"))]
    pub fn of_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E) -> Result<Set<Object>> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;TE;TE;TE;TE;)Ljava/util/Set<TE;>;"))]
    pub fn of_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E) -> Result<Set<Object>> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;TE;TE;TE;TE;TE;)Ljava/util/Set<TE;>;"))]
    pub fn of_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E) -> Result<Set<Object>> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;TE;TE;TE;TE;TE;TE;)Ljava/util/Set<TE;>;"))]
    pub fn of_obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E) -> Result<Set<Object>> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;TE;TE;TE;TE;TE;TE;TE;)Ljava/util/Set<TE;>;"))]
    pub fn of_obj_obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E, e8: E) -> Result<Set<Object>> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;TE;TE;TE;TE;TE;TE;TE;TE;)Ljava/util/Set<TE;>;"))]
    pub fn of_obj_obj_obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E, e8: E, e9: E) -> Result<Set<Object>> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;TE;TE;TE;TE;TE;TE;TE;TE;TE;)Ljava/util/Set<TE;>;"))]
    pub fn of_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E, e8: E, e9: E, e10: E) -> Result<Set<Object>> {
        panic!("stub: java/util/Set.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "([Ljava/lang/Object;)Ljava/util/Set;", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>([TE;)Ljava/util/Set<TE;>;"))]
    pub fn of_arr_obj(elements: Rc<RefCell<Vec<E>>>) -> Result<Set<Object>> {
        panic!("stub: java/util/Set.of:([Ljava/lang/Object;)Ljava/util/Set;")
    }

    #[cfg_attr(any(), java_method(name = "copyOf", descriptor = "(Ljava/util/Collection;)Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(Ljava/util/Collection<+TE;>;)Ljava/util/Set<TE;>;"))]
    pub fn copyOf(coll: Object) -> Result<Set<Object>> {
        panic!("stub: java/util/Set.copyOf:(Ljava/util/Collection;)Ljava/util/Set;")
    }
}
