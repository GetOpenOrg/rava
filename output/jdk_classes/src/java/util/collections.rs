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
        panic!("stub: java/util/Collections.<init>:()V")
    }

    // java: sort(Ljava/util/List;)V
    pub fn sort__list(list: Object) -> Result<()> {
        panic!("stub: java/util/Collections.sort:(Ljava/util/List;)V")
    }

    // java: sort(Ljava/util/List;Ljava/util/Comparator;)V
    pub fn sort__list_compar(list: Object, c: Object) -> Result<()> {
        panic!("stub: java/util/Collections.sort:(Ljava/util/List;Ljava/util/Comparator;)V")
    }

    // java: binarySearch(Ljava/util/List;Ljava/lang/Object;)I
    pub fn binarySearch__list_obj(list: Object, key: Object) -> Result<i32> {
        panic!("stub: java/util/Collections.binarySearch:(Ljava/util/List;Ljava/lang/Object;)I")
    }

    // java: indexedBinarySearch(Ljava/util/List;Ljava/lang/Object;)I
    pub fn indexedBinarySearch__list_obj(list: Object, key: Object) -> Result<i32> {
        panic!("stub: java/util/Collections.indexedBinarySearch:(Ljava/util/List;Ljava/lang/Object;)I")
    }

    // java: iteratorBinarySearch(Ljava/util/List;Ljava/lang/Object;)I
    pub fn iteratorBinarySearch__list_obj(list: Object, key: Object) -> Result<i32> {
        panic!("stub: java/util/Collections.iteratorBinarySearch:(Ljava/util/List;Ljava/lang/Object;)I")
    }

    // java: get(Ljava/util/ListIterator;I)Ljava/lang/Object;
    pub fn get(i: Object, index: i32) -> Result<Object> {
        panic!("stub: java/util/Collections.get:(Ljava/util/ListIterator;I)Ljava/lang/Object;")
    }

    // java: binarySearch(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn binarySearch__list_obj_compar(list: Object, key: Object, c: Object) -> Result<i32> {
        panic!("stub: java/util/Collections.binarySearch:(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I")
    }

    // java: indexedBinarySearch(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn indexedBinarySearch__list_obj_compar(l: Object, key: Object, c: Object) -> Result<i32> {
        panic!("stub: java/util/Collections.indexedBinarySearch:(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I")
    }

    // java: iteratorBinarySearch(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn iteratorBinarySearch__list_obj_compar(l: Object, key: Object, c: Object) -> Result<i32> {
        panic!("stub: java/util/Collections.iteratorBinarySearch:(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I")
    }

    // java: reverse(Ljava/util/List;)V
    pub fn reverse(list: Object) -> Result<()> {
        panic!("stub: java/util/Collections.reverse:(Ljava/util/List;)V")
    }

    // java: shuffle(Ljava/util/List;)V
    pub fn shuffle__list(list: Object) -> Result<()> {
        panic!("stub: java/util/Collections.shuffle:(Ljava/util/List;)V")
    }

    // java: shuffle(Ljava/util/List;Ljava/util/Random;)V
    pub fn shuffle__list_random(list: Object, rnd: Object) -> Result<()> {
        panic!("stub: java/util/Collections.shuffle:(Ljava/util/List;Ljava/util/Random;)V")
    }

    // java: shuffle(Ljava/util/List;Ljava/util/random/RandomGenerator;)V
    pub fn shuffle__list_random_1(list: Object, rnd: Object) -> Result<()> {
        panic!("stub: java/util/Collections.shuffle:(Ljava/util/List;Ljava/util/random/RandomGenerator;)V")
    }

    // java: swap(Ljava/util/List;II)V
    pub fn swap__list_i_i(list: Object, i: i32, j: i32) -> Result<()> {
        panic!("stub: java/util/Collections.swap:(Ljava/util/List;II)V")
    }

    // java: swap([Ljava/lang/Object;II)V
    pub fn swap__arr_obj_i_i(arr: Vec<Object>, i: i32, j: i32) -> Result<()> {
        panic!("stub: java/util/Collections.swap:([Ljava/lang/Object;II)V")
    }

    // java: fill(Ljava/util/List;Ljava/lang/Object;)V
    pub fn fill(list: Object, obj: Object) -> Result<()> {
        panic!("stub: java/util/Collections.fill:(Ljava/util/List;Ljava/lang/Object;)V")
    }

    // java: copy(Ljava/util/List;Ljava/util/List;)V
    pub fn copy(dest: Object, src: Object) -> Result<()> {
        panic!("stub: java/util/Collections.copy:(Ljava/util/List;Ljava/util/List;)V")
    }

    // java: min(Ljava/util/Collection;)Ljava/lang/Object;
    pub fn min__coll(coll: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.min:(Ljava/util/Collection;)Ljava/lang/Object;")
    }

    // java: min(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;
    pub fn min__coll_compar(coll: Object, comp: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.min:(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;")
    }

    // java: max(Ljava/util/Collection;)Ljava/lang/Object;
    pub fn max__coll(coll: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.max:(Ljava/util/Collection;)Ljava/lang/Object;")
    }

    // java: max(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;
    pub fn max__coll_compar(coll: Object, comp: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.max:(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;")
    }

    // java: rotate(Ljava/util/List;I)V
    pub fn rotate(list: Object, distance: i32) -> Result<()> {
        panic!("stub: java/util/Collections.rotate:(Ljava/util/List;I)V")
    }

    // java: rotate1(Ljava/util/List;I)V
    pub fn rotate1(list: Object, distance: i32) -> Result<()> {
        panic!("stub: java/util/Collections.rotate1:(Ljava/util/List;I)V")
    }

    // java: rotate2(Ljava/util/List;I)V
    pub fn rotate2(list: Object, distance: i32) -> Result<()> {
        panic!("stub: java/util/Collections.rotate2:(Ljava/util/List;I)V")
    }

    // java: replaceAll(Ljava/util/List;Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn replaceAll(list: Object, oldVal: Object, newVal: Object) -> Result<bool> {
        panic!("stub: java/util/Collections.replaceAll:(Ljava/util/List;Ljava/lang/Object;Ljava/lang/Object;)Z")
    }

    // java: indexOfSubList(Ljava/util/List;Ljava/util/List;)I
    pub fn indexOfSubList(source: Object, target: Object) -> Result<i32> {
        panic!("stub: java/util/Collections.indexOfSubList:(Ljava/util/List;Ljava/util/List;)I")
    }

    // java: lastIndexOfSubList(Ljava/util/List;Ljava/util/List;)I
    pub fn lastIndexOfSubList(source: Object, target: Object) -> Result<i32> {
        panic!("stub: java/util/Collections.lastIndexOfSubList:(Ljava/util/List;Ljava/util/List;)I")
    }

    // java: unmodifiableCollection(Ljava/util/Collection;)Ljava/util/Collection;
    pub fn unmodifiableCollection(c: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.unmodifiableCollection:(Ljava/util/Collection;)Ljava/util/Collection;")
    }

    // java: unmodifiableSequencedCollection(Ljava/util/SequencedCollection;)Ljava/util/SequencedCollection;
    pub fn unmodifiableSequencedCollection(c: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.unmodifiableSequencedCollection:(Ljava/util/SequencedCollection;)Ljava/util/SequencedCollection;")
    }

    // java: unmodifiableSet(Ljava/util/Set;)Ljava/util/Set;
    pub fn unmodifiableSet(s: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.unmodifiableSet:(Ljava/util/Set;)Ljava/util/Set;")
    }

    // java: unmodifiableSequencedSet(Ljava/util/SequencedSet;)Ljava/util/SequencedSet;
    pub fn unmodifiableSequencedSet(s: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.unmodifiableSequencedSet:(Ljava/util/SequencedSet;)Ljava/util/SequencedSet;")
    }

    // java: unmodifiableSortedSet(Ljava/util/SortedSet;)Ljava/util/SortedSet;
    pub fn unmodifiableSortedSet(s: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.unmodifiableSortedSet:(Ljava/util/SortedSet;)Ljava/util/SortedSet;")
    }

    // java: unmodifiableNavigableSet(Ljava/util/NavigableSet;)Ljava/util/NavigableSet;
    pub fn unmodifiableNavigableSet(s: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.unmodifiableNavigableSet:(Ljava/util/NavigableSet;)Ljava/util/NavigableSet;")
    }

    // java: unmodifiableList(Ljava/util/List;)Ljava/util/List;
    pub fn unmodifiableList(list: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.unmodifiableList:(Ljava/util/List;)Ljava/util/List;")
    }

    // java: unmodifiableMap(Ljava/util/Map;)Ljava/util/Map;
    pub fn unmodifiableMap(m: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.unmodifiableMap:(Ljava/util/Map;)Ljava/util/Map;")
    }

    // java: unmodifiableSequencedMap(Ljava/util/SequencedMap;)Ljava/util/SequencedMap;
    pub fn unmodifiableSequencedMap(m: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.unmodifiableSequencedMap:(Ljava/util/SequencedMap;)Ljava/util/SequencedMap;")
    }

    // java: unmodifiableSortedMap(Ljava/util/SortedMap;)Ljava/util/SortedMap;
    pub fn unmodifiableSortedMap(m: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.unmodifiableSortedMap:(Ljava/util/SortedMap;)Ljava/util/SortedMap;")
    }

    // java: unmodifiableNavigableMap(Ljava/util/NavigableMap;)Ljava/util/NavigableMap;
    pub fn unmodifiableNavigableMap(m: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.unmodifiableNavigableMap:(Ljava/util/NavigableMap;)Ljava/util/NavigableMap;")
    }

    // java: synchronizedCollection(Ljava/util/Collection;)Ljava/util/Collection;
    pub fn synchronizedCollection__coll(c: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.synchronizedCollection:(Ljava/util/Collection;)Ljava/util/Collection;")
    }

    // java: synchronizedCollection(Ljava/util/Collection;Ljava/lang/Object;)Ljava/util/Collection;
    pub fn synchronizedCollection__coll_obj(c: Object, mutex: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.synchronizedCollection:(Ljava/util/Collection;Ljava/lang/Object;)Ljava/util/Collection;")
    }

    // java: synchronizedSet(Ljava/util/Set;)Ljava/util/Set;
    pub fn synchronizedSet__set(s: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.synchronizedSet:(Ljava/util/Set;)Ljava/util/Set;")
    }

    // java: synchronizedSet(Ljava/util/Set;Ljava/lang/Object;)Ljava/util/Set;
    pub fn synchronizedSet__set_obj(s: Object, mutex: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.synchronizedSet:(Ljava/util/Set;Ljava/lang/Object;)Ljava/util/Set;")
    }

    // java: synchronizedSortedSet(Ljava/util/SortedSet;)Ljava/util/SortedSet;
    pub fn synchronizedSortedSet(s: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.synchronizedSortedSet:(Ljava/util/SortedSet;)Ljava/util/SortedSet;")
    }

    // java: synchronizedNavigableSet(Ljava/util/NavigableSet;)Ljava/util/NavigableSet;
    pub fn synchronizedNavigableSet(s: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.synchronizedNavigableSet:(Ljava/util/NavigableSet;)Ljava/util/NavigableSet;")
    }

    // java: synchronizedList(Ljava/util/List;)Ljava/util/List;
    pub fn synchronizedList__list(list: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.synchronizedList:(Ljava/util/List;)Ljava/util/List;")
    }

    // java: synchronizedList(Ljava/util/List;Ljava/lang/Object;)Ljava/util/List;
    pub fn synchronizedList__list_obj(list: Object, mutex: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.synchronizedList:(Ljava/util/List;Ljava/lang/Object;)Ljava/util/List;")
    }

    // java: synchronizedMap(Ljava/util/Map;)Ljava/util/Map;
    pub fn synchronizedMap(m: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.synchronizedMap:(Ljava/util/Map;)Ljava/util/Map;")
    }

    // java: synchronizedSortedMap(Ljava/util/SortedMap;)Ljava/util/SortedMap;
    pub fn synchronizedSortedMap(m: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.synchronizedSortedMap:(Ljava/util/SortedMap;)Ljava/util/SortedMap;")
    }

    // java: synchronizedNavigableMap(Ljava/util/NavigableMap;)Ljava/util/NavigableMap;
    pub fn synchronizedNavigableMap(m: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.synchronizedNavigableMap:(Ljava/util/NavigableMap;)Ljava/util/NavigableMap;")
    }

    // java: checkedCollection(Ljava/util/Collection;Ljava/lang/Class;)Ljava/util/Collection;
    pub fn checkedCollection(c: Object, type_: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.checkedCollection:(Ljava/util/Collection;Ljava/lang/Class;)Ljava/util/Collection;")
    }

    // java: zeroLengthArray(Ljava/lang/Class;)[Ljava/lang/Object;
    pub fn zeroLengthArray(type_: Object) -> Result<Vec<Object>> {
        panic!("stub: java/util/Collections.zeroLengthArray:(Ljava/lang/Class;)[Ljava/lang/Object;")
    }

    // java: checkedQueue(Ljava/util/Queue;Ljava/lang/Class;)Ljava/util/Queue;
    pub fn checkedQueue(queue: Object, type_: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.checkedQueue:(Ljava/util/Queue;Ljava/lang/Class;)Ljava/util/Queue;")
    }

    // java: checkedSet(Ljava/util/Set;Ljava/lang/Class;)Ljava/util/Set;
    pub fn checkedSet(s: Object, type_: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.checkedSet:(Ljava/util/Set;Ljava/lang/Class;)Ljava/util/Set;")
    }

    // java: checkedSortedSet(Ljava/util/SortedSet;Ljava/lang/Class;)Ljava/util/SortedSet;
    pub fn checkedSortedSet(s: Object, type_: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.checkedSortedSet:(Ljava/util/SortedSet;Ljava/lang/Class;)Ljava/util/SortedSet;")
    }

    // java: checkedNavigableSet(Ljava/util/NavigableSet;Ljava/lang/Class;)Ljava/util/NavigableSet;
    pub fn checkedNavigableSet(s: Object, type_: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.checkedNavigableSet:(Ljava/util/NavigableSet;Ljava/lang/Class;)Ljava/util/NavigableSet;")
    }

    // java: checkedList(Ljava/util/List;Ljava/lang/Class;)Ljava/util/List;
    pub fn checkedList(list: Object, type_: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.checkedList:(Ljava/util/List;Ljava/lang/Class;)Ljava/util/List;")
    }

    // java: checkedMap(Ljava/util/Map;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/Map;
    pub fn checkedMap(m: Object, keyType: Object, valueType: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.checkedMap:(Ljava/util/Map;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/Map;")
    }

    // java: checkedSortedMap(Ljava/util/SortedMap;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/SortedMap;
    pub fn checkedSortedMap(m: Object, keyType: Object, valueType: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.checkedSortedMap:(Ljava/util/SortedMap;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/SortedMap;")
    }

    // java: checkedNavigableMap(Ljava/util/NavigableMap;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/NavigableMap;
    pub fn checkedNavigableMap(m: Object, keyType: Object, valueType: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.checkedNavigableMap:(Ljava/util/NavigableMap;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/NavigableMap;")
    }

    // java: emptyIterator()Ljava/util/Iterator;
    pub fn emptyIterator() -> Result<Object> {
        panic!("stub: java/util/Collections.emptyIterator:()Ljava/util/Iterator;")
    }

    // java: emptyListIterator()Ljava/util/ListIterator;
    pub fn emptyListIterator() -> Result<Object> {
        panic!("stub: java/util/Collections.emptyListIterator:()Ljava/util/ListIterator;")
    }

    // java: emptyEnumeration()Ljava/util/Enumeration;
    pub fn emptyEnumeration() -> Result<Object> {
        panic!("stub: java/util/Collections.emptyEnumeration:()Ljava/util/Enumeration;")
    }

    // java: emptySet()Ljava/util/Set;
    pub fn emptySet() -> Result<Object> {
        panic!("stub: java/util/Collections.emptySet:()Ljava/util/Set;")
    }

    // java: emptySortedSet()Ljava/util/SortedSet;
    pub fn emptySortedSet() -> Result<Object> {
        panic!("stub: java/util/Collections.emptySortedSet:()Ljava/util/SortedSet;")
    }

    // java: emptyNavigableSet()Ljava/util/NavigableSet;
    pub fn emptyNavigableSet() -> Result<Object> {
        panic!("stub: java/util/Collections.emptyNavigableSet:()Ljava/util/NavigableSet;")
    }

    // java: emptyList()Ljava/util/List;
    pub fn emptyList() -> Result<Object> {
        panic!("stub: java/util/Collections.emptyList:()Ljava/util/List;")
    }

    // java: emptyMap()Ljava/util/Map;
    pub fn emptyMap() -> Result<Object> {
        panic!("stub: java/util/Collections.emptyMap:()Ljava/util/Map;")
    }

    // java: emptySortedMap()Ljava/util/SortedMap;
    pub fn emptySortedMap() -> Result<Object> {
        panic!("stub: java/util/Collections.emptySortedMap:()Ljava/util/SortedMap;")
    }

    // java: emptyNavigableMap()Ljava/util/NavigableMap;
    pub fn emptyNavigableMap() -> Result<Object> {
        panic!("stub: java/util/Collections.emptyNavigableMap:()Ljava/util/NavigableMap;")
    }

    // java: singleton(Ljava/lang/Object;)Ljava/util/Set;
    pub fn singleton(o: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.singleton:(Ljava/lang/Object;)Ljava/util/Set;")
    }

    // java: singletonIterator(Ljava/lang/Object;)Ljava/util/Iterator;
    pub fn singletonIterator(e: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.singletonIterator:(Ljava/lang/Object;)Ljava/util/Iterator;")
    }

    // java: singletonSpliterator(Ljava/lang/Object;)Ljava/util/Spliterator;
    pub fn singletonSpliterator(element: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.singletonSpliterator:(Ljava/lang/Object;)Ljava/util/Spliterator;")
    }

    // java: singletonList(Ljava/lang/Object;)Ljava/util/List;
    pub fn singletonList(o: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.singletonList:(Ljava/lang/Object;)Ljava/util/List;")
    }

    // java: singletonMap(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
    pub fn singletonMap(key: Object, value: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.singletonMap:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;")
    }

    // java: nCopies(ILjava/lang/Object;)Ljava/util/List;
    pub fn nCopies(n: i32, o: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.nCopies:(ILjava/lang/Object;)Ljava/util/List;")
    }

    // java: reverseOrder()Ljava/util/Comparator;
    pub fn reverseOrder() -> Result<Object> {
        panic!("stub: java/util/Collections.reverseOrder:()Ljava/util/Comparator;")
    }

    // java: reverseOrder(Ljava/util/Comparator;)Ljava/util/Comparator;
    pub fn reverseOrder__compar(cmp: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.reverseOrder:(Ljava/util/Comparator;)Ljava/util/Comparator;")
    }

    // java: enumeration(Ljava/util/Collection;)Ljava/util/Enumeration;
    pub fn enumeration(c: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.enumeration:(Ljava/util/Collection;)Ljava/util/Enumeration;")
    }

    // java: list(Ljava/util/Enumeration;)Ljava/util/ArrayList;
    pub fn list(e: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.list:(Ljava/util/Enumeration;)Ljava/util/ArrayList;")
    }

    // java: eq(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn eq(o1: Object, o2: Object) -> Result<bool> {
        panic!("stub: java/util/Collections.eq:(Ljava/lang/Object;Ljava/lang/Object;)Z")
    }

    // java: frequency(Ljava/util/Collection;Ljava/lang/Object;)I
    pub fn frequency(c: Object, o: Object) -> Result<i32> {
        panic!("stub: java/util/Collections.frequency:(Ljava/util/Collection;Ljava/lang/Object;)I")
    }

    // java: disjoint(Ljava/util/Collection;Ljava/util/Collection;)Z
    pub fn disjoint(c1: Object, c2: Object) -> Result<bool> {
        panic!("stub: java/util/Collections.disjoint:(Ljava/util/Collection;Ljava/util/Collection;)Z")
    }

    // java: addAll(Ljava/util/Collection;[Ljava/lang/Object;)Z
    pub fn addAll(c: Object, elements: Vec<Object>) -> Result<bool> {
        panic!("stub: java/util/Collections.addAll:(Ljava/util/Collection;[Ljava/lang/Object;)Z")
    }

    // java: newSetFromMap(Ljava/util/Map;)Ljava/util/Set;
    pub fn newSetFromMap(map: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.newSetFromMap:(Ljava/util/Map;)Ljava/util/Set;")
    }

    // java: newSequencedSetFromMap(Ljava/util/SequencedMap;)Ljava/util/SequencedSet;
    pub fn newSequencedSetFromMap(map: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.newSequencedSetFromMap:(Ljava/util/SequencedMap;)Ljava/util/SequencedSet;")
    }

    // java: asLifoQueue(Ljava/util/Deque;)Ljava/util/Queue;
    pub fn asLifoQueue(deque: Object) -> Result<Object> {
        panic!("stub: java/util/Collections.asLifoQueue:(Ljava/util/Deque;)Ljava/util/Queue;")
    }
}
