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
    binary_name       = "java/util/List",
    super_class       = "java/lang/Object",
    interfaces        = "java/util/SequencedCollection",
    access            = "public",
    modifiers         = "abstract interface",
    generic_signature = "<E:Ljava/lang/Object;>Ljava/lang/Object;Ljava/util/SequencedCollection<TE;>;",
    is_interface      = true,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "List.java",
    inner_classes     = "java/util/AbstractList$RandomAccessSpliterator:java/util/AbstractList:RandomAccessSpliterator:24;java/util/ImmutableCollections$ListN:java/util/ImmutableCollections:ListN:24;java/util/ImmutableCollections$List12:java/util/ImmutableCollections:List12:24",
)]
#[derive(Clone, Default, PartialEq)]
pub struct List<E: Clone + Default + 'static>(std::marker::PhantomData<E>);

impl<E: Clone + Default + 'static> List<E> {
    #[cfg_attr(any(), java_method(name = "size", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn size(&self) -> Result<i32> {
        panic!("stub: java/util/List.size:()I")
    }

    #[cfg_attr(any(), java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/List.isEmpty:()Z")
    }

    #[cfg_attr(any(), java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn contains(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/List.contains:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TE;>;"))]
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/List.iterator:()Ljava/util/Iterator;")
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn toArray(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/util/List.toArray:()[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;"))]
    pub fn toArray_arr_obj(&self, arg0: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/util/List.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(TE;)Z"))]
    pub fn add_obj(&self, arg0: E) -> Result<bool> {
        panic!("stub: java/util/List.add:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn remove_obj(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/List.remove:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "containsAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z"))]
    pub fn containsAll(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/List.containsAll:(Ljava/util/Collection;)Z")
    }

    #[cfg_attr(any(), java_method(name = "addAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/Collection<+TE;>;)Z"))]
    pub fn addAll_coll(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/List.addAll:(Ljava/util/Collection;)Z")
    }

    #[cfg_attr(any(), java_method(name = "addAll", descriptor = "(ILjava/util/Collection;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(ILjava/util/Collection<+TE;>;)Z"))]
    pub fn addAll_i_coll(&self, arg0: i32, arg1: Object) -> Result<bool> {
        panic!("stub: java/util/List.addAll:(ILjava/util/Collection;)Z")
    }

    #[cfg_attr(any(), java_method(name = "removeAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z"))]
    pub fn removeAll(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/List.removeAll:(Ljava/util/Collection;)Z")
    }

    #[cfg_attr(any(), java_method(name = "retainAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z"))]
    pub fn retainAll(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/List.retainAll:(Ljava/util/Collection;)Z")
    }

    #[cfg_attr(any(), java_method(name = "replaceAll", descriptor = "(Ljava/util/function/UnaryOperator;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/UnaryOperator<TE;>;)V"))]
    pub fn replaceAll(&self, operator: Object) -> Result<()> {
        panic!("stub: java/util/List.replaceAll:(Ljava/util/function/UnaryOperator;)V")
    }

    #[cfg_attr(any(), java_method(name = "sort", descriptor = "(Ljava/util/Comparator;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Comparator<-TE;>;)V"))]
    pub fn sort(&self, c: Object) -> Result<()> {
        panic!("stub: java/util/List.sort:(Ljava/util/Comparator;)V")
    }

    #[cfg_attr(any(), java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/List.clear:()V")
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn equals(&self, arg0: Object) -> Result<bool> {
        panic!("stub: java/util/List.equals:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn hashCode(&self) -> Result<i32> {
        panic!("stub: java/util/List.hashCode:()I")
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "(I)Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(I)TE;"))]
    pub fn get(&self, arg0: i32) -> Result<Object> {
        panic!("stub: java/util/List.get:(I)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "set", descriptor = "(ILjava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(ITE;)TE;"))]
    pub fn set(&self, arg0: i32, arg1: E) -> Result<Object> {
        panic!("stub: java/util/List.set:(ILjava/lang/Object;)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(ILjava/lang/Object;)V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(ITE;)V"))]
    pub fn add_i_obj(&self, arg0: i32, arg1: E) -> Result<()> {
        panic!("stub: java/util/List.add:(ILjava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(I)Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(I)TE;"))]
    pub fn remove_i(&self, arg0: i32) -> Result<Object> {
        panic!("stub: java/util/List.remove:(I)Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "(Ljava/lang/Object;)I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn indexOf(&self, arg0: Object) -> Result<i32> {
        panic!("stub: java/util/List.indexOf:(Ljava/lang/Object;)I")
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/Object;)I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false))]
    pub fn lastIndexOf(&self, arg0: Object) -> Result<i32> {
        panic!("stub: java/util/List.lastIndexOf:(Ljava/lang/Object;)I")
    }

    #[cfg_attr(any(), java_method(name = "listIterator", descriptor = "()Ljava/util/ListIterator;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "()Ljava/util/ListIterator<TE;>;"))]
    pub fn listIterator(&self) -> Result<Object> {
        panic!("stub: java/util/List.listIterator:()Ljava/util/ListIterator;")
    }

    #[cfg_attr(any(), java_method(name = "listIterator", descriptor = "(I)Ljava/util/ListIterator;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(I)Ljava/util/ListIterator<TE;>;"))]
    pub fn listIterator_i(&self, arg0: i32) -> Result<Object> {
        panic!("stub: java/util/List.listIterator:(I)Ljava/util/ListIterator;")
    }

    #[cfg_attr(any(), java_method(name = "subList", descriptor = "(II)Ljava/util/List;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(II)Ljava/util/List<TE;>;"))]
    pub fn subList(&self, arg0: i32, arg1: i32) -> Result<List<Object>> {
        panic!("stub: java/util/List.subList:(II)Ljava/util/List;")
    }

    #[cfg_attr(any(), java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TE;>;"))]
    pub fn spliterator(&self) -> Result<Object> {
        panic!("stub: java/util/List.spliterator:()Ljava/util/Spliterator;")
    }

    #[cfg_attr(any(), java_method(name = "addFirst", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V"))]
    pub fn addFirst(&self, e: E) -> Result<()> {
        panic!("stub: java/util/List.addFirst:(Ljava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_method(name = "addLast", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V"))]
    pub fn addLast(&self, e: E) -> Result<()> {
        panic!("stub: java/util/List.addLast:(Ljava/lang/Object;)V")
    }

    #[cfg_attr(any(), java_method(name = "getFirst", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;"))]
    pub fn getFirst(&self) -> Result<Object> {
        panic!("stub: java/util/List.getFirst:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "getLast", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;"))]
    pub fn getLast(&self) -> Result<Object> {
        panic!("stub: java/util/List.getLast:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "removeFirst", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;"))]
    pub fn removeFirst(&self) -> Result<Object> {
        panic!("stub: java/util/List.removeFirst:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "removeLast", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;"))]
    pub fn removeLast(&self) -> Result<Object> {
        panic!("stub: java/util/List.removeLast:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "reversed", descriptor = "()Ljava/util/List;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/List<TE;>;"))]
    pub fn reversed(&self) -> Result<List<Object>> {
        panic!("stub: java/util/List.reversed:()Ljava/util/List;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "()Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>()Ljava/util/List<TE;>;"))]
    pub fn of() -> Result<List<Object>> {
        panic!("stub: java/util/List.of:()Ljava/util/List;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;)Ljava/util/List<TE;>;"))]
    pub fn of_obj(e1: E) -> Result<List<Object>> {
        panic!("stub: java/util/List.of:(Ljava/lang/Object;)Ljava/util/List;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;TE;)Ljava/util/List<TE;>;"))]
    pub fn of_obj_obj(e1: E, e2: E) -> Result<List<Object>> {
        panic!("stub: java/util/List.of:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;TE;TE;)Ljava/util/List<TE;>;"))]
    pub fn of_obj_obj_obj(e1: E, e2: E, e3: E) -> Result<List<Object>> {
        panic!("stub: java/util/List.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;TE;TE;TE;)Ljava/util/List<TE;>;"))]
    pub fn of_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E) -> Result<List<Object>> {
        panic!("stub: java/util/List.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;TE;TE;TE;TE;)Ljava/util/List<TE;>;"))]
    pub fn of_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E) -> Result<List<Object>> {
        panic!("stub: java/util/List.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;TE;TE;TE;TE;TE;)Ljava/util/List<TE;>;"))]
    pub fn of_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E) -> Result<List<Object>> {
        panic!("stub: java/util/List.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;TE;TE;TE;TE;TE;TE;)Ljava/util/List<TE;>;"))]
    pub fn of_obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E) -> Result<List<Object>> {
        panic!("stub: java/util/List.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;TE;TE;TE;TE;TE;TE;TE;)Ljava/util/List<TE;>;"))]
    pub fn of_obj_obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E, e8: E) -> Result<List<Object>> {
        panic!("stub: java/util/List.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;TE;TE;TE;TE;TE;TE;TE;TE;)Ljava/util/List<TE;>;"))]
    pub fn of_obj_obj_obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E, e8: E, e9: E) -> Result<List<Object>> {
        panic!("stub: java/util/List.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;TE;TE;TE;TE;TE;TE;TE;TE;TE;)Ljava/util/List<TE;>;"))]
    pub fn of_obj_obj_obj_obj_obj_obj_obj_obj_obj_obj(e1: E, e2: E, e3: E, e4: E, e5: E, e6: E, e7: E, e8: E, e9: E, e10: E) -> Result<List<Object>> {
        panic!("stub: java/util/List.of:(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/List;")
    }

    #[cfg_attr(any(), java_method(name = "of", descriptor = "([Ljava/lang/Object;)Ljava/util/List;", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>([TE;)Ljava/util/List<TE;>;"))]
    pub fn of_arr_obj(elements: Rc<RefCell<Vec<E>>>) -> Result<List<Object>> {
        panic!("stub: java/util/List.of:([Ljava/lang/Object;)Ljava/util/List;")
    }

    #[cfg_attr(any(), java_method(name = "copyOf", descriptor = "(Ljava/util/Collection;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(Ljava/util/Collection<+TE;>;)Ljava/util/List<TE;>;"))]
    pub fn copyOf(coll: Object) -> Result<List<Object>> {
        panic!("stub: java/util/List.copyOf:(Ljava/util/Collection;)Ljava/util/List;")
    }
}
