#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Collections",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public",
    source      = "Collections.java",
))]
pub struct Collections;

impl Collections {
    // java: <init>()V
    pub fn new(&self) -> Result<()> {
        todo!("abstract java/util/Collections.<init>")
    }

    // java: sort(Ljava/util/List;)V
    pub fn sort__list(list: Object) -> Result<()> {
        todo!("abstract java/util/Collections.sort")
    }

    // java: sort(Ljava/util/List;Ljava/util/Comparator;)V
    pub fn sort__list_compar(list: Object, c: Object) -> Result<()> {
        todo!("abstract java/util/Collections.sort")
    }

    // java: binarySearch(Ljava/util/List;Ljava/lang/Object;)I
    pub fn binarySearch__list_obj(list: Object, key: Object) -> Result<i32> {
        todo!("abstract java/util/Collections.binarySearch")
    }

    // java: indexedBinarySearch(Ljava/util/List;Ljava/lang/Object;)I
    pub fn indexedBinarySearch__list_obj(list: Object, key: Object) -> Result<i32> {
        todo!("abstract java/util/Collections.indexedBinarySearch")
    }

    // java: iteratorBinarySearch(Ljava/util/List;Ljava/lang/Object;)I
    pub fn iteratorBinarySearch__list_obj(list: Object, key: Object) -> Result<i32> {
        todo!("abstract java/util/Collections.iteratorBinarySearch")
    }

    // java: get(Ljava/util/ListIterator;I)Ljava/lang/Object;
    pub fn get(i: Object, index: i32) -> Result<Object> {
        todo!("abstract java/util/Collections.get")
    }

    // java: binarySearch(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn binarySearch__list_obj_compar(list: Object, key: Object, c: Object) -> Result<i32> {
        todo!("abstract java/util/Collections.binarySearch")
    }

    // java: indexedBinarySearch(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn indexedBinarySearch__list_obj_compar(l: Object, key: Object, c: Object) -> Result<i32> {
        todo!("abstract java/util/Collections.indexedBinarySearch")
    }

    // java: iteratorBinarySearch(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn iteratorBinarySearch__list_obj_compar(l: Object, key: Object, c: Object) -> Result<i32> {
        todo!("abstract java/util/Collections.iteratorBinarySearch")
    }

    // java: reverse(Ljava/util/List;)V
    pub fn reverse(list: Object) -> Result<()> {
        todo!("abstract java/util/Collections.reverse")
    }

    // java: shuffle(Ljava/util/List;)V
    pub fn shuffle__list(list: Object) -> Result<()> {
        todo!("abstract java/util/Collections.shuffle")
    }

    // java: shuffle(Ljava/util/List;Ljava/util/Random;)V
    pub fn shuffle__list_random(list: Object, rnd: Object) -> Result<()> {
        todo!("abstract java/util/Collections.shuffle")
    }

    // java: shuffle(Ljava/util/List;Ljava/util/random/RandomGenerator;)V
    pub fn shuffle__list_random_1(list: Object, rnd: Object) -> Result<()> {
        todo!("abstract java/util/Collections.shuffle")
    }

    // java: swap(Ljava/util/List;II)V
    pub fn swap__list_i_i(list: Object, i: i32, j: i32) -> Result<()> {
        todo!("abstract java/util/Collections.swap")
    }

    // java: swap([Ljava/lang/Object;II)V
    pub fn swap__arr_obj_i_i(arr: Vec<Object>, i: i32, j: i32) -> Result<()> {
        todo!("abstract java/util/Collections.swap")
    }

    // java: fill(Ljava/util/List;Ljava/lang/Object;)V
    pub fn fill(list: Object, obj: Object) -> Result<()> {
        todo!("abstract java/util/Collections.fill")
    }

    // java: copy(Ljava/util/List;Ljava/util/List;)V
    pub fn copy(dest: Object, src: Object) -> Result<()> {
        todo!("abstract java/util/Collections.copy")
    }

    // java: min(Ljava/util/Collection;)Ljava/lang/Object;
    pub fn min__coll(coll: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.min")
    }

    // java: min(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;
    pub fn min__coll_compar(coll: Object, comp: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.min")
    }

    // java: max(Ljava/util/Collection;)Ljava/lang/Object;
    pub fn max__coll(coll: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.max")
    }

    // java: max(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;
    pub fn max__coll_compar(coll: Object, comp: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.max")
    }

    // java: rotate(Ljava/util/List;I)V
    pub fn rotate(list: Object, distance: i32) -> Result<()> {
        todo!("abstract java/util/Collections.rotate")
    }

    // java: rotate1(Ljava/util/List;I)V
    pub fn rotate1(list: Object, distance: i32) -> Result<()> {
        todo!("abstract java/util/Collections.rotate1")
    }

    // java: rotate2(Ljava/util/List;I)V
    pub fn rotate2(list: Object, distance: i32) -> Result<()> {
        todo!("abstract java/util/Collections.rotate2")
    }

    // java: replaceAll(Ljava/util/List;Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn replaceAll(list: Object, oldVal: Object, newVal: Object) -> Result<bool> {
        todo!("abstract java/util/Collections.replaceAll")
    }

    // java: indexOfSubList(Ljava/util/List;Ljava/util/List;)I
    pub fn indexOfSubList(source: Object, target: Object) -> Result<i32> {
        todo!("abstract java/util/Collections.indexOfSubList")
    }

    // java: lastIndexOfSubList(Ljava/util/List;Ljava/util/List;)I
    pub fn lastIndexOfSubList(source: Object, target: Object) -> Result<i32> {
        todo!("abstract java/util/Collections.lastIndexOfSubList")
    }

    // java: unmodifiableCollection(Ljava/util/Collection;)Ljava/util/Collection;
    pub fn unmodifiableCollection(c: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.unmodifiableCollection")
    }

    // java: unmodifiableSequencedCollection(Ljava/util/SequencedCollection;)Ljava/util/SequencedCollection;
    pub fn unmodifiableSequencedCollection(c: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.unmodifiableSequencedCollection")
    }

    // java: unmodifiableSet(Ljava/util/Set;)Ljava/util/Set;
    pub fn unmodifiableSet(s: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.unmodifiableSet")
    }

    // java: unmodifiableSequencedSet(Ljava/util/SequencedSet;)Ljava/util/SequencedSet;
    pub fn unmodifiableSequencedSet(s: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.unmodifiableSequencedSet")
    }

    // java: unmodifiableSortedSet(Ljava/util/SortedSet;)Ljava/util/SortedSet;
    pub fn unmodifiableSortedSet(s: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.unmodifiableSortedSet")
    }

    // java: unmodifiableNavigableSet(Ljava/util/NavigableSet;)Ljava/util/NavigableSet;
    pub fn unmodifiableNavigableSet(s: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.unmodifiableNavigableSet")
    }

    // java: unmodifiableList(Ljava/util/List;)Ljava/util/List;
    pub fn unmodifiableList(list: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.unmodifiableList")
    }

    // java: unmodifiableMap(Ljava/util/Map;)Ljava/util/Map;
    pub fn unmodifiableMap(m: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.unmodifiableMap")
    }

    // java: unmodifiableSequencedMap(Ljava/util/SequencedMap;)Ljava/util/SequencedMap;
    pub fn unmodifiableSequencedMap(m: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.unmodifiableSequencedMap")
    }

    // java: unmodifiableSortedMap(Ljava/util/SortedMap;)Ljava/util/SortedMap;
    pub fn unmodifiableSortedMap(m: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.unmodifiableSortedMap")
    }

    // java: unmodifiableNavigableMap(Ljava/util/NavigableMap;)Ljava/util/NavigableMap;
    pub fn unmodifiableNavigableMap(m: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.unmodifiableNavigableMap")
    }

    // java: synchronizedCollection(Ljava/util/Collection;)Ljava/util/Collection;
    pub fn synchronizedCollection__coll(c: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.synchronizedCollection")
    }

    // java: synchronizedCollection(Ljava/util/Collection;Ljava/lang/Object;)Ljava/util/Collection;
    pub fn synchronizedCollection__coll_obj(c: Object, mutex: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.synchronizedCollection")
    }

    // java: synchronizedSet(Ljava/util/Set;)Ljava/util/Set;
    pub fn synchronizedSet__set(s: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.synchronizedSet")
    }

    // java: synchronizedSet(Ljava/util/Set;Ljava/lang/Object;)Ljava/util/Set;
    pub fn synchronizedSet__set_obj(s: Object, mutex: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.synchronizedSet")
    }

    // java: synchronizedSortedSet(Ljava/util/SortedSet;)Ljava/util/SortedSet;
    pub fn synchronizedSortedSet(s: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.synchronizedSortedSet")
    }

    // java: synchronizedNavigableSet(Ljava/util/NavigableSet;)Ljava/util/NavigableSet;
    pub fn synchronizedNavigableSet(s: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.synchronizedNavigableSet")
    }

    // java: synchronizedList(Ljava/util/List;)Ljava/util/List;
    pub fn synchronizedList__list(list: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.synchronizedList")
    }

    // java: synchronizedList(Ljava/util/List;Ljava/lang/Object;)Ljava/util/List;
    pub fn synchronizedList__list_obj(list: Object, mutex: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.synchronizedList")
    }

    // java: synchronizedMap(Ljava/util/Map;)Ljava/util/Map;
    pub fn synchronizedMap(m: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.synchronizedMap")
    }

    // java: synchronizedSortedMap(Ljava/util/SortedMap;)Ljava/util/SortedMap;
    pub fn synchronizedSortedMap(m: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.synchronizedSortedMap")
    }

    // java: synchronizedNavigableMap(Ljava/util/NavigableMap;)Ljava/util/NavigableMap;
    pub fn synchronizedNavigableMap(m: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.synchronizedNavigableMap")
    }

    // java: checkedCollection(Ljava/util/Collection;Ljava/lang/Class;)Ljava/util/Collection;
    pub fn checkedCollection(c: Object, type_: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.checkedCollection")
    }

    // java: zeroLengthArray(Ljava/lang/Class;)[Ljava/lang/Object;
    pub fn zeroLengthArray(type_: Object) -> Result<Vec<Object>> {
        todo!("abstract java/util/Collections.zeroLengthArray")
    }

    // java: checkedQueue(Ljava/util/Queue;Ljava/lang/Class;)Ljava/util/Queue;
    pub fn checkedQueue(queue: Object, type_: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.checkedQueue")
    }

    // java: checkedSet(Ljava/util/Set;Ljava/lang/Class;)Ljava/util/Set;
    pub fn checkedSet(s: Object, type_: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.checkedSet")
    }

    // java: checkedSortedSet(Ljava/util/SortedSet;Ljava/lang/Class;)Ljava/util/SortedSet;
    pub fn checkedSortedSet(s: Object, type_: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.checkedSortedSet")
    }

    // java: checkedNavigableSet(Ljava/util/NavigableSet;Ljava/lang/Class;)Ljava/util/NavigableSet;
    pub fn checkedNavigableSet(s: Object, type_: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.checkedNavigableSet")
    }

    // java: checkedList(Ljava/util/List;Ljava/lang/Class;)Ljava/util/List;
    pub fn checkedList(list: Object, type_: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.checkedList")
    }

    // java: checkedMap(Ljava/util/Map;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/Map;
    pub fn checkedMap(m: Object, keyType: Object, valueType: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.checkedMap")
    }

    // java: checkedSortedMap(Ljava/util/SortedMap;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/SortedMap;
    pub fn checkedSortedMap(m: Object, keyType: Object, valueType: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.checkedSortedMap")
    }

    // java: checkedNavigableMap(Ljava/util/NavigableMap;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/NavigableMap;
    pub fn checkedNavigableMap(m: Object, keyType: Object, valueType: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.checkedNavigableMap")
    }

    // java: emptyIterator()Ljava/util/Iterator;
    pub fn emptyIterator() -> Result<Object> {
        todo!("abstract java/util/Collections.emptyIterator")
    }

    // java: emptyListIterator()Ljava/util/ListIterator;
    pub fn emptyListIterator() -> Result<Object> {
        todo!("abstract java/util/Collections.emptyListIterator")
    }

    // java: emptyEnumeration()Ljava/util/Enumeration;
    pub fn emptyEnumeration() -> Result<Object> {
        todo!("abstract java/util/Collections.emptyEnumeration")
    }

    // java: emptySet()Ljava/util/Set;
    pub fn emptySet() -> Result<Object> {
        todo!("abstract java/util/Collections.emptySet")
    }

    // java: emptySortedSet()Ljava/util/SortedSet;
    pub fn emptySortedSet() -> Result<Object> {
        todo!("abstract java/util/Collections.emptySortedSet")
    }

    // java: emptyNavigableSet()Ljava/util/NavigableSet;
    pub fn emptyNavigableSet() -> Result<Object> {
        todo!("abstract java/util/Collections.emptyNavigableSet")
    }

    // java: emptyList()Ljava/util/List;
    pub fn emptyList() -> Result<Object> {
        todo!("abstract java/util/Collections.emptyList")
    }

    // java: emptyMap()Ljava/util/Map;
    pub fn emptyMap() -> Result<Object> {
        todo!("abstract java/util/Collections.emptyMap")
    }

    // java: emptySortedMap()Ljava/util/SortedMap;
    pub fn emptySortedMap() -> Result<Object> {
        todo!("abstract java/util/Collections.emptySortedMap")
    }

    // java: emptyNavigableMap()Ljava/util/NavigableMap;
    pub fn emptyNavigableMap() -> Result<Object> {
        todo!("abstract java/util/Collections.emptyNavigableMap")
    }

    // java: singleton(Ljava/lang/Object;)Ljava/util/Set;
    pub fn singleton(o: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.singleton")
    }

    // java: singletonIterator(Ljava/lang/Object;)Ljava/util/Iterator;
    pub fn singletonIterator(e: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.singletonIterator")
    }

    // java: singletonSpliterator(Ljava/lang/Object;)Ljava/util/Spliterator;
    pub fn singletonSpliterator(element: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.singletonSpliterator")
    }

    // java: singletonList(Ljava/lang/Object;)Ljava/util/List;
    pub fn singletonList(o: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.singletonList")
    }

    // java: singletonMap(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    pub fn singletonMap(key: Object, value: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.singletonMap")
    }

    // java: nCopies(ILjava/lang/Object;)Ljava/util/List;
    pub fn nCopies(n: i32, o: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.nCopies")
    }

    // java: reverseOrder()Ljava/util/Comparator;
    pub fn reverseOrder() -> Result<Object> {
        todo!("abstract java/util/Collections.reverseOrder")
    }

    // java: reverseOrder(Ljava/util/Comparator;)Ljava/util/Comparator;
    pub fn reverseOrder__compar(cmp: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.reverseOrder")
    }

    // java: enumeration(Ljava/util/Collection;)Ljava/util/Enumeration;
    pub fn enumeration(c: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.enumeration")
    }

    // java: list(Ljava/util/Enumeration;)Ljava/util/ArrayList;
    pub fn list(e: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.list")
    }

    // java: eq(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn eq(o1: Object, o2: Object) -> Result<bool> {
        todo!("abstract java/util/Collections.eq")
    }

    // java: frequency(Ljava/util/Collection;Ljava/lang/Object;)I
    pub fn frequency(c: Object, o: Object) -> Result<i32> {
        todo!("abstract java/util/Collections.frequency")
    }

    // java: disjoint(Ljava/util/Collection;Ljava/util/Collection;)Z
    pub fn disjoint(c1: Object, c2: Object) -> Result<bool> {
        todo!("abstract java/util/Collections.disjoint")
    }

    // java: addAll(Ljava/util/Collection;[Ljava/lang/Object;)Z
    pub fn addAll(c: Object, elements: Vec<Object>) -> Result<bool> {
        todo!("abstract java/util/Collections.addAll")
    }

    // java: newSetFromMap(Ljava/util/Map;)Ljava/util/Set;
    pub fn newSetFromMap(map: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.newSetFromMap")
    }

    // java: newSequencedSetFromMap(Ljava/util/SequencedMap;)Ljava/util/SequencedSet;
    pub fn newSequencedSetFromMap(map: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.newSequencedSetFromMap")
    }

    // java: asLifoQueue(Ljava/util/Deque;)Ljava/util/Queue;
    pub fn asLifoQueue(deque: Object) -> Result<Object> {
        todo!("abstract java/util/Collections.asLifoQueue")
    }
}
