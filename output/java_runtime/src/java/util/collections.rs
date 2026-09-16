#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::math::*;
use crate::java::nio::*;
use crate::java::nio::charset::*;
use crate::java::security::*;
use crate::java::text::*;
use crate::java::text::spi::*;
use crate::java::time::*;
use crate::java::time::chrono::*;
use crate::java::time::temporal::*;
use crate::java::time::zone::*;
use crate::java::util::*;
use crate::java::util::concurrent::*;
use crate::java::util::concurrent::atomic::*;
use crate::java::util::concurrent::locks::*;
use crate::java::util::function::*;
use crate::java::util::regex::*;
use crate::java::util::spi::*;
use crate::java::util::stream::*;
use crate::java::util::zip::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::reflect::misc::*;
use crate::sun::security::action::*;
use crate::sun::security::util::*;
use crate::sun::text::*;
use crate::sun::util::*;
use crate::sun::util::calendar::*;
use crate::sun::util::locale::*;
use crate::sun::util::locale::provider::*;
use crate::sun::util::spi::*;
use crate::java::text::Normalizer;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/Collections"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Collections.java"]
    #[inner_classes     = "java/util/Collections$UnmodifiableCollection:java/util/Collections:UnmodifiableCollection:8;java/util/Collections$UnmodifiableSequencedCollection:java/util/Collections:UnmodifiableSequencedCollection:8;java/util/Collections$UnmodifiableSet:java/util/Collections:UnmodifiableSet:8;java/util/Collections$UnmodifiableSequencedSet:java/util/Collections:UnmodifiableSequencedSet:8;java/util/Collections$UnmodifiableSortedSet:java/util/Collections:UnmodifiableSortedSet:8;java/util/Collections$UnmodifiableNavigableSet:java/util/Collections:UnmodifiableNavigableSet:8;java/util/Collections$UnmodifiableList:java/util/Collections:UnmodifiableList:8;java/util/Collections$UnmodifiableRandomAccessList:java/util/Collections:UnmodifiableRandomAccessList:8;java/util/Collections$UnmodifiableMap:java/util/Collections:UnmodifiableMap:10;java/util/Collections$UnmodifiableSequencedMap:java/util/Collections:UnmodifiableSequencedMap:10;java/util/Collections$UnmodifiableSortedMap:java/util/Collections:UnmodifiableSortedMap:8;java/util/Collections$UnmodifiableNavigableMap:java/util/Collections:UnmodifiableNavigableMap:8;java/util/Collections$SynchronizedCollection:java/util/Collections:SynchronizedCollection:8;java/util/Collections$SynchronizedSet:java/util/Collections:SynchronizedSet:8;java/util/Collections$SynchronizedSortedSet:java/util/Collections:SynchronizedSortedSet:8;java/util/Collections$SynchronizedNavigableSet:java/util/Collections:SynchronizedNavigableSet:8;java/util/Collections$SynchronizedRandomAccessList:java/util/Collections:SynchronizedRandomAccessList:8;java/util/Collections$SynchronizedList:java/util/Collections:SynchronizedList:8;java/util/Collections$SynchronizedMap:java/util/Collections:SynchronizedMap:10;java/util/Collections$SynchronizedSortedMap:java/util/Collections:SynchronizedSortedMap:8;java/util/Collections$SynchronizedNavigableMap:java/util/Collections:SynchronizedNavigableMap:8;java/util/Collections$CheckedCollection:java/util/Collections:CheckedCollection:8;java/util/Collections$CheckedQueue:java/util/Collections:CheckedQueue:8;java/util/Collections$CheckedSet:java/util/Collections:CheckedSet:8;java/util/Collections$CheckedSortedSet:java/util/Collections:CheckedSortedSet:8;java/util/Collections$CheckedNavigableSet:java/util/Collections:CheckedNavigableSet:8;java/util/Collections$CheckedRandomAccessList:java/util/Collections:CheckedRandomAccessList:8;java/util/Collections$CheckedList:java/util/Collections:CheckedList:8;java/util/Collections$CheckedMap:java/util/Collections:CheckedMap:10;java/util/Collections$CheckedSortedMap:java/util/Collections:CheckedSortedMap:8;java/util/Collections$CheckedNavigableMap:java/util/Collections:CheckedNavigableMap:8;java/util/Collections$EmptyIterator:java/util/Collections:EmptyIterator:10;java/util/Collections$EmptyListIterator:java/util/Collections:EmptyListIterator:10;java/util/Collections$EmptyEnumeration:java/util/Collections:EmptyEnumeration:10;java/util/Collections$UnmodifiableNavigableMap$EmptyNavigableMap:java/util/Collections$UnmodifiableNavigableMap:EmptyNavigableMap:10;java/util/Collections$SingletonSet:java/util/Collections:SingletonSet:10;java/util/Collections$1:::0;java/util/Collections$2:::0;java/util/Collections$SingletonList:java/util/Collections:SingletonList:10;java/util/Collections$SingletonMap:java/util/Collections:SingletonMap:10;java/util/Collections$CopiesList:java/util/Collections:CopiesList:10;java/util/Collections$ReverseComparator:java/util/Collections:ReverseComparator:10;java/util/Comparators$NaturalOrderComparator:java/util/Comparators:NaturalOrderComparator:16408;java/util/Collections$ReverseComparator2:java/util/Collections:ReverseComparator2:10;java/util/Collections$3:::0;java/util/Collections$SetFromMap:java/util/Collections:SetFromMap:10;java/util/Collections$SequencedSetFromMap:java/util/Collections:SequencedSetFromMap:10;java/util/Collections$AsLIFOQueue:java/util/Collections:AsLIFOQueue:8;java/util/Collections$EmptySet:java/util/Collections:EmptySet:10;java/util/Collections$EmptyList:java/util/Collections:EmptyList:10;java/util/Collections$EmptyMap:java/util/Collections:EmptyMap:10;java/util/Collections$CheckedMap$CheckedEntrySet:java/util/Collections$CheckedMap:CheckedEntrySet:8;java/util/Collections$CheckedMap$CheckedEntrySet$CheckedEntry:java/util/Collections$CheckedMap$CheckedEntrySet:CheckedEntry:10;java/util/Collections$CheckedMap$CheckedEntrySet$1:::0;java/util/Collections$CheckedList$1:::0;java/util/Collections$CheckedCollection$1:::0;java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet:java/util/Collections$UnmodifiableMap:UnmodifiableEntrySet:8;java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry:java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet:UnmodifiableEntry:10;java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntrySetSpliterator:java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet:UnmodifiableEntrySetSpliterator:24;java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$1:::0;java/util/Collections$UnmodifiableList$1:::0;java/util/Collections$UnmodifiableNavigableSet$EmptyNavigableSet:java/util/Collections$UnmodifiableNavigableSet:EmptyNavigableSet:10;java/util/Collections$UnmodifiableCollection$1:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Collections"]

    pub struct Collections;

    impl Collections {
        #[cfg_attr(any(), java_field(name = "BINARYSEARCH_THRESHOLD", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "5000"))]
        // static field: BINARYSEARCH_THRESHOLD:I
        pub fn BINARYSEARCH_THRESHOLD() -> i32 {
            5000
        }

        #[cfg_attr(any(), java_field(name = "REVERSE_THRESHOLD", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "18"))]
        // static field: REVERSE_THRESHOLD:I
        pub fn REVERSE_THRESHOLD() -> i32 {
            18
        }

        #[cfg_attr(any(), java_field(name = "SHUFFLE_THRESHOLD", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "5"))]
        // static field: SHUFFLE_THRESHOLD:I
        pub fn SHUFFLE_THRESHOLD() -> i32 {
            5
        }

        #[cfg_attr(any(), java_field(name = "FILL_THRESHOLD", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "25"))]
        // static field: FILL_THRESHOLD:I
        pub fn FILL_THRESHOLD() -> i32 {
            25
        }

        #[cfg_attr(any(), java_field(name = "ROTATE_THRESHOLD", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "100"))]
        // static field: ROTATE_THRESHOLD:I
        pub fn ROTATE_THRESHOLD() -> i32 {
            100
        }

        #[cfg_attr(any(), java_field(name = "COPY_THRESHOLD", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "10"))]
        // static field: COPY_THRESHOLD:I
        pub fn COPY_THRESHOLD() -> i32 {
            10
        }

        #[cfg_attr(any(), java_field(name = "REPLACEALL_THRESHOLD", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "11"))]
        // static field: REPLACEALL_THRESHOLD:I
        pub fn REPLACEALL_THRESHOLD() -> i32 {
            11
        }

        #[cfg_attr(any(), java_field(name = "INDEXOFSUBLIST_THRESHOLD", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "35"))]
        // static field: INDEXOFSUBLIST_THRESHOLD:I
        pub fn INDEXOFSUBLIST_THRESHOLD() -> i32 {
            35
        }

        #[cfg_attr(any(), java_field(name = "r", descriptor = "Ljava/util/Random;", access = "private", modifiers = "static", is_static = true))]
        // static field: r:Ljava/util/Random;
        pub fn r() -> Random {
            panic!("stub: java/util/Collections.r:Ljava/util/Random;")
        }

        #[cfg_attr(any(), java_field(name = "EMPTY_SET", descriptor = "Ljava/util/Set;", access = "public", modifiers = "static final", is_static = true))]
        // static field: EMPTY_SET:Ljava/util/Set;
        pub fn EMPTY_SET() -> Object {
            panic!("stub: java/util/Collections.EMPTY_SET:Ljava/util/Set;")
        }

        #[cfg_attr(any(), java_field(name = "EMPTY_LIST", descriptor = "Ljava/util/List;", access = "public", modifiers = "static final", is_static = true))]
        // static field: EMPTY_LIST:Ljava/util/List;
        pub fn EMPTY_LIST() -> Object {
            panic!("stub: java/util/Collections.EMPTY_LIST:Ljava/util/List;")
        }

        #[cfg_attr(any(), java_field(name = "EMPTY_MAP", descriptor = "Ljava/util/Map;", access = "public", modifiers = "static final", is_static = true))]
        // static field: EMPTY_MAP:Ljava/util/Map;
        pub fn EMPTY_MAP() -> Object {
            panic!("stub: java/util/Collections.EMPTY_MAP:Ljava/util/Map;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/Collections.<init>:()V")
        }

        #[java_method(name = "sort", descriptor = "(Ljava/util/List;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T::Ljava/lang/Comparable<-TT;>;>(Ljava/util/List<TT;>;)V")]
        pub fn sort_list(list: Object) -> Result<()> {
            panic!("stub: java/util/Collections.sort:(Ljava/util/List;)V")
        }

        #[java_method(name = "sort", descriptor = "(Ljava/util/List;Ljava/util/Comparator;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/List<TT;>;Ljava/util/Comparator<-TT;>;)V")]
        // java: sort(Ljava/util/List;Ljava/util/Comparator;)V
        pub fn sort_list_compar(mut list: Object, mut c: Object) -> Result<()> {
            if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.sort(Clone::clone(&c))?; } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.sort(Clone::clone(&c))?; } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.sort(Clone::clone(&c))?; } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.sort(Clone::clone(&c))?; } else if let Some(_d) = list.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.sort(Clone::clone(&c))?; } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.sort(Clone::clone(&c))?; } else if let Some(_d) = list.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.sort(Clone::clone(&c))?; } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.sort(Clone::clone(&c))?; } else if let Some(_d) = list.0.as_any().downcast_ref::<Object>() { _d.sort(Clone::clone(&c))?; } else if let Some(__f) = list.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<()>>>() { (__f)(Clone::clone(&c))?; }
            Ok(())
        }

        #[java_method(name = "binarySearch", descriptor = "(Ljava/util/List;Ljava/lang/Object;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/List<+Ljava/lang/Comparable<-TT;>;>;TT;)I")]
        pub fn binarySearch_list_obj(list: Object, key: Object) -> Result<i32> {
            panic!("stub: java/util/Collections.binarySearch:(Ljava/util/List;Ljava/lang/Object;)I")
        }

        #[java_method(name = "indexedBinarySearch", descriptor = "(Ljava/util/List;Ljava/lang/Object;)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/List<+Ljava/lang/Comparable<-TT;>;>;TT;)I")]
        pub fn indexedBinarySearch_list_obj(list: Object, key: Object) -> Result<i32> {
            panic!("stub: java/util/Collections.indexedBinarySearch:(Ljava/util/List;Ljava/lang/Object;)I")
        }

        #[java_method(name = "iteratorBinarySearch", descriptor = "(Ljava/util/List;Ljava/lang/Object;)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/List<+Ljava/lang/Comparable<-TT;>;>;TT;)I")]
        pub fn iteratorBinarySearch_list_obj(list: Object, key: Object) -> Result<i32> {
            panic!("stub: java/util/Collections.iteratorBinarySearch:(Ljava/util/List;Ljava/lang/Object;)I")
        }

        #[java_method(name = "get", descriptor = "(Ljava/util/ListIterator;I)Ljava/lang/Object;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/ListIterator<+TT;>;I)TT;")]
        pub fn get(i: Object, index: i32) -> Result<Object> {
            panic!("stub: java/util/Collections.get:(Ljava/util/ListIterator;I)Ljava/lang/Object;")
        }

        #[java_method(name = "binarySearch", descriptor = "(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/List<+TT;>;TT;Ljava/util/Comparator<-TT;>;)I")]
        pub fn binarySearch_list_obj_compar(list: Object, key: Object, c: Object) -> Result<i32> {
            panic!("stub: java/util/Collections.binarySearch:(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I")
        }

        #[java_method(name = "indexedBinarySearch", descriptor = "(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/List<+TT;>;TT;Ljava/util/Comparator<-TT;>;)I")]
        pub fn indexedBinarySearch_list_obj_compar(l: Object, key: Object, c: Object) -> Result<i32> {
            panic!("stub: java/util/Collections.indexedBinarySearch:(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I")
        }

        #[java_method(name = "iteratorBinarySearch", descriptor = "(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/List<+TT;>;TT;Ljava/util/Comparator<-TT;>;)I")]
        pub fn iteratorBinarySearch_list_obj_compar(l: Object, key: Object, c: Object) -> Result<i32> {
            panic!("stub: java/util/Collections.iteratorBinarySearch:(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I")
        }

        #[java_method(name = "reverse", descriptor = "(Ljava/util/List;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<*>;)V")]
        pub fn reverse(list: Object) -> Result<()> {
            panic!("stub: java/util/Collections.reverse:(Ljava/util/List;)V")
        }

        #[java_method(name = "shuffle", descriptor = "(Ljava/util/List;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<*>;)V")]
        pub fn shuffle_list(list: Object) -> Result<()> {
            panic!("stub: java/util/Collections.shuffle:(Ljava/util/List;)V")
        }

        #[java_method(name = "shuffle", descriptor = "(Ljava/util/List;Ljava/util/Random;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<*>;Ljava/util/Random;)V")]
        pub fn shuffle_list_random(list: Object, rnd: Random) -> Result<()> {
            panic!("stub: java/util/Collections.shuffle:(Ljava/util/List;Ljava/util/Random;)V")
        }

        #[java_method(name = "shuffle", descriptor = "(Ljava/util/List;Ljava/util/random/RandomGenerator;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<*>;Ljava/util/random/RandomGenerator;)V")]
        pub fn shuffle_list_random_1(list: Object, rnd: Object) -> Result<()> {
            panic!("stub: java/util/Collections.shuffle:(Ljava/util/List;Ljava/util/random/RandomGenerator;)V")
        }

        #[java_method(name = "swap", descriptor = "(Ljava/util/List;II)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<*>;II)V")]
        pub fn swap_list_i_i(list: Object, i: i32, j: i32) -> Result<()> {
            panic!("stub: java/util/Collections.swap:(Ljava/util/List;II)V")
        }

        #[java_method(name = "swap", descriptor = "([Ljava/lang/Object;II)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn swap_arr_obj_i_i(arr: Rc<RefCell<Vec<Object>>>, i: i32, j: i32) -> Result<()> {
            panic!("stub: java/util/Collections.swap:([Ljava/lang/Object;II)V")
        }

        #[java_method(name = "fill", descriptor = "(Ljava/util/List;Ljava/lang/Object;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/List<-TT;>;TT;)V")]
        pub fn fill(list: Object, obj: Object) -> Result<()> {
            panic!("stub: java/util/Collections.fill:(Ljava/util/List;Ljava/lang/Object;)V")
        }

        #[java_method(name = "copy", descriptor = "(Ljava/util/List;Ljava/util/List;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/List<-TT;>;Ljava/util/List<+TT;>;)V")]
        pub fn copy(dest: Object, src: Object) -> Result<()> {
            panic!("stub: java/util/Collections.copy:(Ljava/util/List;Ljava/util/List;)V")
        }

        #[java_method(name = "min", descriptor = "(Ljava/util/Collection;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;:Ljava/lang/Comparable<-TT;>;>(Ljava/util/Collection<+TT;>;)TT;")]
        pub fn min_coll(coll: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.min:(Ljava/util/Collection;)Ljava/lang/Object;")
        }

        #[java_method(name = "min", descriptor = "(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Collection<+TT;>;Ljava/util/Comparator<-TT;>;)TT;")]
        pub fn min_coll_compar(coll: Object, comp: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.min:(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;")
        }

        #[java_method(name = "max", descriptor = "(Ljava/util/Collection;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;:Ljava/lang/Comparable<-TT;>;>(Ljava/util/Collection<+TT;>;)TT;")]
        pub fn max_coll(coll: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.max:(Ljava/util/Collection;)Ljava/lang/Object;")
        }

        #[java_method(name = "max", descriptor = "(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Collection<+TT;>;Ljava/util/Comparator<-TT;>;)TT;")]
        pub fn max_coll_compar(coll: Object, comp: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.max:(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;")
        }

        #[java_method(name = "rotate", descriptor = "(Ljava/util/List;I)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<*>;I)V")]
        pub fn rotate(list: Object, distance: i32) -> Result<()> {
            panic!("stub: java/util/Collections.rotate:(Ljava/util/List;I)V")
        }

        #[java_method(name = "rotate1", descriptor = "(Ljava/util/List;I)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/List<TT;>;I)V")]
        pub fn rotate1(list: Object, distance: i32) -> Result<()> {
            panic!("stub: java/util/Collections.rotate1:(Ljava/util/List;I)V")
        }

        #[java_method(name = "rotate2", descriptor = "(Ljava/util/List;I)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<*>;I)V")]
        pub fn rotate2(list: Object, distance: i32) -> Result<()> {
            panic!("stub: java/util/Collections.rotate2:(Ljava/util/List;I)V")
        }

        #[java_method(name = "replaceAll", descriptor = "(Ljava/util/List;Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/List<TT;>;TT;TT;)Z")]
        pub fn replaceAll(list: Object, oldVal: Object, newVal: Object) -> Result<bool> {
            panic!("stub: java/util/Collections.replaceAll:(Ljava/util/List;Ljava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "indexOfSubList", descriptor = "(Ljava/util/List;Ljava/util/List;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<*>;Ljava/util/List<*>;)I")]
        pub fn indexOfSubList(source: Object, target: Object) -> Result<i32> {
            panic!("stub: java/util/Collections.indexOfSubList:(Ljava/util/List;Ljava/util/List;)I")
        }

        #[java_method(name = "lastIndexOfSubList", descriptor = "(Ljava/util/List;Ljava/util/List;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<*>;Ljava/util/List<*>;)I")]
        pub fn lastIndexOfSubList(source: Object, target: Object) -> Result<i32> {
            panic!("stub: java/util/Collections.lastIndexOfSubList:(Ljava/util/List;Ljava/util/List;)I")
        }

        #[java_method(name = "unmodifiableCollection", descriptor = "(Ljava/util/Collection;)Ljava/util/Collection;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Collection<+TT;>;)Ljava/util/Collection<TT;>;")]
        pub fn unmodifiableCollection(c: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.unmodifiableCollection:(Ljava/util/Collection;)Ljava/util/Collection;")
        }

        #[java_method(name = "unmodifiableSequencedCollection", descriptor = "(Ljava/util/SequencedCollection;)Ljava/util/SequencedCollection;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/SequencedCollection<+TT;>;)Ljava/util/SequencedCollection<TT;>;")]
        pub fn unmodifiableSequencedCollection(c: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.unmodifiableSequencedCollection:(Ljava/util/SequencedCollection;)Ljava/util/SequencedCollection;")
        }

        #[java_method(name = "unmodifiableSet", descriptor = "(Ljava/util/Set;)Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Set<+TT;>;)Ljava/util/Set<TT;>;")]
        pub fn unmodifiableSet(s: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.unmodifiableSet:(Ljava/util/Set;)Ljava/util/Set;")
        }

        #[java_method(name = "unmodifiableSequencedSet", descriptor = "(Ljava/util/SequencedSet;)Ljava/util/SequencedSet;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/SequencedSet<+TT;>;)Ljava/util/SequencedSet<TT;>;")]
        pub fn unmodifiableSequencedSet(s: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.unmodifiableSequencedSet:(Ljava/util/SequencedSet;)Ljava/util/SequencedSet;")
        }

        #[java_method(name = "unmodifiableSortedSet", descriptor = "(Ljava/util/SortedSet;)Ljava/util/SortedSet;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/SortedSet<TT;>;)Ljava/util/SortedSet<TT;>;")]
        pub fn unmodifiableSortedSet(s: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.unmodifiableSortedSet:(Ljava/util/SortedSet;)Ljava/util/SortedSet;")
        }

        #[java_method(name = "unmodifiableNavigableSet", descriptor = "(Ljava/util/NavigableSet;)Ljava/util/NavigableSet;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/NavigableSet<TT;>;)Ljava/util/NavigableSet<TT;>;")]
        pub fn unmodifiableNavigableSet(s: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.unmodifiableNavigableSet:(Ljava/util/NavigableSet;)Ljava/util/NavigableSet;")
        }

        #[java_method(name = "unmodifiableList", descriptor = "(Ljava/util/List;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/List<+TT;>;)Ljava/util/List<TT;>;")]
        pub fn unmodifiableList(list: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.unmodifiableList:(Ljava/util/List;)Ljava/util/List;")
        }

        #[java_method(name = "unmodifiableMap", descriptor = "(Ljava/util/Map;)Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/Map<+TK;+TV;>;)Ljava/util/Map<TK;TV;>;")]
        pub fn unmodifiableMap(m: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.unmodifiableMap:(Ljava/util/Map;)Ljava/util/Map;")
        }

        #[java_method(name = "unmodifiableSequencedMap", descriptor = "(Ljava/util/SequencedMap;)Ljava/util/SequencedMap;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/SequencedMap<+TK;+TV;>;)Ljava/util/SequencedMap<TK;TV;>;")]
        pub fn unmodifiableSequencedMap(m: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.unmodifiableSequencedMap:(Ljava/util/SequencedMap;)Ljava/util/SequencedMap;")
        }

        #[java_method(name = "unmodifiableSortedMap", descriptor = "(Ljava/util/SortedMap;)Ljava/util/SortedMap;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/SortedMap<TK;+TV;>;)Ljava/util/SortedMap<TK;TV;>;")]
        pub fn unmodifiableSortedMap(m: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.unmodifiableSortedMap:(Ljava/util/SortedMap;)Ljava/util/SortedMap;")
        }

        #[java_method(name = "unmodifiableNavigableMap", descriptor = "(Ljava/util/NavigableMap;)Ljava/util/NavigableMap;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/NavigableMap<TK;+TV;>;)Ljava/util/NavigableMap<TK;TV;>;")]
        pub fn unmodifiableNavigableMap(m: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.unmodifiableNavigableMap:(Ljava/util/NavigableMap;)Ljava/util/NavigableMap;")
        }

        #[java_method(name = "synchronizedCollection", descriptor = "(Ljava/util/Collection;)Ljava/util/Collection;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Collection<TT;>;)Ljava/util/Collection<TT;>;")]
        pub fn synchronizedCollection_coll(c: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.synchronizedCollection:(Ljava/util/Collection;)Ljava/util/Collection;")
        }

        #[java_method(name = "synchronizedCollection", descriptor = "(Ljava/util/Collection;Ljava/lang/Object;)Ljava/util/Collection;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Collection<TT;>;Ljava/lang/Object;)Ljava/util/Collection<TT;>;")]
        pub fn synchronizedCollection_coll_obj(c: Object, mutex: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.synchronizedCollection:(Ljava/util/Collection;Ljava/lang/Object;)Ljava/util/Collection;")
        }

        #[java_method(name = "synchronizedSet", descriptor = "(Ljava/util/Set;)Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Set<TT;>;)Ljava/util/Set<TT;>;")]
        pub fn synchronizedSet_set(s: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.synchronizedSet:(Ljava/util/Set;)Ljava/util/Set;")
        }

        #[java_method(name = "synchronizedSet", descriptor = "(Ljava/util/Set;Ljava/lang/Object;)Ljava/util/Set;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Set<TT;>;Ljava/lang/Object;)Ljava/util/Set<TT;>;")]
        pub fn synchronizedSet_set_obj(s: Object, mutex: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.synchronizedSet:(Ljava/util/Set;Ljava/lang/Object;)Ljava/util/Set;")
        }

        #[java_method(name = "synchronizedSortedSet", descriptor = "(Ljava/util/SortedSet;)Ljava/util/SortedSet;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/SortedSet<TT;>;)Ljava/util/SortedSet<TT;>;")]
        pub fn synchronizedSortedSet(s: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.synchronizedSortedSet:(Ljava/util/SortedSet;)Ljava/util/SortedSet;")
        }

        #[java_method(name = "synchronizedNavigableSet", descriptor = "(Ljava/util/NavigableSet;)Ljava/util/NavigableSet;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/NavigableSet<TT;>;)Ljava/util/NavigableSet<TT;>;")]
        pub fn synchronizedNavigableSet(s: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.synchronizedNavigableSet:(Ljava/util/NavigableSet;)Ljava/util/NavigableSet;")
        }

        #[java_method(name = "synchronizedList", descriptor = "(Ljava/util/List;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/List<TT;>;)Ljava/util/List<TT;>;")]
        pub fn synchronizedList_list(list: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.synchronizedList:(Ljava/util/List;)Ljava/util/List;")
        }

        #[java_method(name = "synchronizedList", descriptor = "(Ljava/util/List;Ljava/lang/Object;)Ljava/util/List;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/List<TT;>;Ljava/lang/Object;)Ljava/util/List<TT;>;")]
        pub fn synchronizedList_list_obj(list: Object, mutex: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.synchronizedList:(Ljava/util/List;Ljava/lang/Object;)Ljava/util/List;")
        }

        #[java_method(name = "synchronizedMap", descriptor = "(Ljava/util/Map;)Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/Map<TK;TV;>;)Ljava/util/Map<TK;TV;>;")]
        pub fn synchronizedMap(m: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.synchronizedMap:(Ljava/util/Map;)Ljava/util/Map;")
        }

        #[java_method(name = "synchronizedSortedMap", descriptor = "(Ljava/util/SortedMap;)Ljava/util/SortedMap;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/SortedMap<TK;TV;>;)Ljava/util/SortedMap<TK;TV;>;")]
        pub fn synchronizedSortedMap(m: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.synchronizedSortedMap:(Ljava/util/SortedMap;)Ljava/util/SortedMap;")
        }

        #[java_method(name = "synchronizedNavigableMap", descriptor = "(Ljava/util/NavigableMap;)Ljava/util/NavigableMap;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/NavigableMap<TK;TV;>;)Ljava/util/NavigableMap<TK;TV;>;")]
        pub fn synchronizedNavigableMap(m: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.synchronizedNavigableMap:(Ljava/util/NavigableMap;)Ljava/util/NavigableMap;")
        }

        #[java_method(name = "checkedCollection", descriptor = "(Ljava/util/Collection;Ljava/lang/Class;)Ljava/util/Collection;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(Ljava/util/Collection<TE;>;Ljava/lang/Class<TE;>;)Ljava/util/Collection<TE;>;")]
        pub fn checkedCollection(c: Object, type_: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.checkedCollection:(Ljava/util/Collection;Ljava/lang/Class;)Ljava/util/Collection;")
        }

        #[java_method(name = "zeroLengthArray", descriptor = "(Ljava/lang/Class;)[Ljava/lang/Object;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/lang/Class<TT;>;)[TT;")]
        pub fn zeroLengthArray(type_: Object) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/util/Collections.zeroLengthArray:(Ljava/lang/Class;)[Ljava/lang/Object;")
        }

        #[java_method(name = "checkedQueue", descriptor = "(Ljava/util/Queue;Ljava/lang/Class;)Ljava/util/Queue;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(Ljava/util/Queue<TE;>;Ljava/lang/Class<TE;>;)Ljava/util/Queue<TE;>;")]
        pub fn checkedQueue(queue: Object, type_: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.checkedQueue:(Ljava/util/Queue;Ljava/lang/Class;)Ljava/util/Queue;")
        }

        #[java_method(name = "checkedSet", descriptor = "(Ljava/util/Set;Ljava/lang/Class;)Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(Ljava/util/Set<TE;>;Ljava/lang/Class<TE;>;)Ljava/util/Set<TE;>;")]
        pub fn checkedSet(s: Object, type_: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.checkedSet:(Ljava/util/Set;Ljava/lang/Class;)Ljava/util/Set;")
        }

        #[java_method(name = "checkedSortedSet", descriptor = "(Ljava/util/SortedSet;Ljava/lang/Class;)Ljava/util/SortedSet;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(Ljava/util/SortedSet<TE;>;Ljava/lang/Class<TE;>;)Ljava/util/SortedSet<TE;>;")]
        pub fn checkedSortedSet(s: Object, type_: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.checkedSortedSet:(Ljava/util/SortedSet;Ljava/lang/Class;)Ljava/util/SortedSet;")
        }

        #[java_method(name = "checkedNavigableSet", descriptor = "(Ljava/util/NavigableSet;Ljava/lang/Class;)Ljava/util/NavigableSet;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(Ljava/util/NavigableSet<TE;>;Ljava/lang/Class<TE;>;)Ljava/util/NavigableSet<TE;>;")]
        pub fn checkedNavigableSet(s: Object, type_: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.checkedNavigableSet:(Ljava/util/NavigableSet;Ljava/lang/Class;)Ljava/util/NavigableSet;")
        }

        #[java_method(name = "checkedList", descriptor = "(Ljava/util/List;Ljava/lang/Class;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(Ljava/util/List<TE;>;Ljava/lang/Class<TE;>;)Ljava/util/List<TE;>;")]
        pub fn checkedList(list: Object, type_: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.checkedList:(Ljava/util/List;Ljava/lang/Class;)Ljava/util/List;")
        }

        #[java_method(name = "checkedMap", descriptor = "(Ljava/util/Map;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/Map<TK;TV;>;Ljava/lang/Class<TK;>;Ljava/lang/Class<TV;>;)Ljava/util/Map<TK;TV;>;")]
        pub fn checkedMap(m: Object, keyType: Object, valueType: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.checkedMap:(Ljava/util/Map;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/Map;")
        }

        #[java_method(name = "checkedSortedMap", descriptor = "(Ljava/util/SortedMap;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/SortedMap;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/SortedMap<TK;TV;>;Ljava/lang/Class<TK;>;Ljava/lang/Class<TV;>;)Ljava/util/SortedMap<TK;TV;>;")]
        pub fn checkedSortedMap(m: Object, keyType: Object, valueType: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.checkedSortedMap:(Ljava/util/SortedMap;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/SortedMap;")
        }

        #[java_method(name = "checkedNavigableMap", descriptor = "(Ljava/util/NavigableMap;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/NavigableMap;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(Ljava/util/NavigableMap<TK;TV;>;Ljava/lang/Class<TK;>;Ljava/lang/Class<TV;>;)Ljava/util/NavigableMap<TK;TV;>;")]
        pub fn checkedNavigableMap(m: Object, keyType: Object, valueType: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.checkedNavigableMap:(Ljava/util/NavigableMap;Ljava/lang/Class;Ljava/lang/Class;)Ljava/util/NavigableMap;")
        }

        #[java_method(name = "emptyIterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/Iterator<TT;>;")]
        pub fn emptyIterator() -> Result<Object> {
            panic!("stub: java/util/Collections.emptyIterator:()Ljava/util/Iterator;")
        }

        #[java_method(name = "emptyListIterator", descriptor = "()Ljava/util/ListIterator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/ListIterator<TT;>;")]
        pub fn emptyListIterator() -> Result<Object> {
            panic!("stub: java/util/Collections.emptyListIterator:()Ljava/util/ListIterator;")
        }

        #[java_method(name = "emptyEnumeration", descriptor = "()Ljava/util/Enumeration;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/Enumeration<TT;>;")]
        pub fn emptyEnumeration() -> Result<Object> {
            panic!("stub: java/util/Collections.emptyEnumeration:()Ljava/util/Enumeration;")
        }

        #[java_method(name = "emptySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/Set<TT;>;")]
        pub fn emptySet() -> Result<Object> {
            panic!("stub: java/util/Collections.emptySet:()Ljava/util/Set;")
        }

        #[java_method(name = "emptySortedSet", descriptor = "()Ljava/util/SortedSet;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>()Ljava/util/SortedSet<TE;>;")]
        pub fn emptySortedSet() -> Result<Object> {
            panic!("stub: java/util/Collections.emptySortedSet:()Ljava/util/SortedSet;")
        }

        #[java_method(name = "emptyNavigableSet", descriptor = "()Ljava/util/NavigableSet;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>()Ljava/util/NavigableSet<TE;>;")]
        pub fn emptyNavigableSet() -> Result<Object> {
            panic!("stub: java/util/Collections.emptyNavigableSet:()Ljava/util/NavigableSet;")
        }

        #[java_method(name = "emptyList", descriptor = "()Ljava/util/List;", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/List<TT;>;")]
        pub fn emptyList() -> Result<Object> {
            panic!("stub: java/util/Collections.emptyList:()Ljava/util/List;")
        }

        #[java_method(name = "emptyMap", descriptor = "()Ljava/util/Map;", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>()Ljava/util/Map<TK;TV;>;")]
        pub fn emptyMap() -> Result<Object> {
            panic!("stub: java/util/Collections.emptyMap:()Ljava/util/Map;")
        }

        #[java_method(name = "emptySortedMap", descriptor = "()Ljava/util/SortedMap;", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>()Ljava/util/SortedMap<TK;TV;>;")]
        pub fn emptySortedMap() -> Result<Object> {
            panic!("stub: java/util/Collections.emptySortedMap:()Ljava/util/SortedMap;")
        }

        #[java_method(name = "emptyNavigableMap", descriptor = "()Ljava/util/NavigableMap;", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>()Ljava/util/NavigableMap<TK;TV;>;")]
        pub fn emptyNavigableMap() -> Result<Object> {
            panic!("stub: java/util/Collections.emptyNavigableMap:()Ljava/util/NavigableMap;")
        }

        #[java_method(name = "singleton", descriptor = "(Ljava/lang/Object;)Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;)Ljava/util/Set<TT;>;")]
        pub fn singleton(o: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.singleton:(Ljava/lang/Object;)Ljava/util/Set;")
        }

        #[java_method(name = "singletonIterator", descriptor = "(Ljava/lang/Object;)Ljava/util/Iterator;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(TE;)Ljava/util/Iterator<TE;>;")]
        pub fn singletonIterator(e: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.singletonIterator:(Ljava/lang/Object;)Ljava/util/Iterator;")
        }

        #[java_method(name = "singletonSpliterator", descriptor = "(Ljava/lang/Object;)Ljava/util/Spliterator;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;)Ljava/util/Spliterator<TT;>;")]
        pub fn singletonSpliterator(element: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.singletonSpliterator:(Ljava/lang/Object;)Ljava/util/Spliterator;")
        }

        #[java_method(name = "singletonList", descriptor = "(Ljava/lang/Object;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;)Ljava/util/List<TT;>;")]
        pub fn singletonList(o: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.singletonList:(Ljava/lang/Object;)Ljava/util/List;")
        }

        #[java_method(name = "singletonMap", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>(TK;TV;)Ljava/util/Map<TK;TV;>;")]
        pub fn singletonMap(key: Object, value: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.singletonMap:(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;")
        }

        #[java_method(name = "nCopies", descriptor = "(ILjava/lang/Object;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(ITT;)Ljava/util/List<TT;>;")]
        pub fn nCopies(n: i32, o: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.nCopies:(ILjava/lang/Object;)Ljava/util/List;")
        }

        #[java_method(name = "reverseOrder", descriptor = "()Ljava/util/Comparator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/Comparator<TT;>;")]
        // java: reverseOrder()Ljava/util/Comparator;
        pub fn reverseOrder() -> Result<Object> {
            Ok(Object::from_any(Collections_ReverseComparator::REVERSE_ORDER().clone()))
        }

        #[java_method(name = "reverseOrder", descriptor = "(Ljava/util/Comparator;)Ljava/util/Comparator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Comparator<TT;>;)Ljava/util/Comparator<TT;>;")]
        pub fn reverseOrder_compar(cmp: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.reverseOrder:(Ljava/util/Comparator;)Ljava/util/Comparator;")
        }

        #[java_method(name = "enumeration", descriptor = "(Ljava/util/Collection;)Ljava/util/Enumeration;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Collection<TT;>;)Ljava/util/Enumeration<TT;>;")]
        pub fn enumeration(c: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.enumeration:(Ljava/util/Collection;)Ljava/util/Enumeration;")
        }

        #[java_method(name = "list", descriptor = "(Ljava/util/Enumeration;)Ljava/util/ArrayList;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Enumeration<TT;>;)Ljava/util/ArrayList<TT;>;")]
        pub fn list(e: Object) -> Result<ArrayList<Object>> {
            panic!("stub: java/util/Collections.list:(Ljava/util/Enumeration;)Ljava/util/ArrayList;")
        }

        #[java_method(name = "eq", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn eq(o1: Object, o2: Object) -> Result<bool> {
            panic!("stub: java/util/Collections.eq:(Ljava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "frequency", descriptor = "(Ljava/util/Collection;Ljava/lang/Object;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;Ljava/lang/Object;)I")]
        pub fn frequency(c: Object, o: Object) -> Result<i32> {
            panic!("stub: java/util/Collections.frequency:(Ljava/util/Collection;Ljava/lang/Object;)I")
        }

        #[java_method(name = "disjoint", descriptor = "(Ljava/util/Collection;Ljava/util/Collection;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;Ljava/util/Collection<*>;)Z")]
        pub fn disjoint(c1: Object, c2: Object) -> Result<bool> {
            panic!("stub: java/util/Collections.disjoint:(Ljava/util/Collection;Ljava/util/Collection;)Z")
        }

        #[java_method(name = "addAll", descriptor = "(Ljava/util/Collection;[Ljava/lang/Object;)Z", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Collection<-TT;>;[TT;)Z")]
        pub fn addAll(c: Object, elements: Rc<RefCell<Vec<Object>>>) -> Result<bool> {
            panic!("stub: java/util/Collections.addAll:(Ljava/util/Collection;[Ljava/lang/Object;)Z")
        }

        #[java_method(name = "newSetFromMap", descriptor = "(Ljava/util/Map;)Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(Ljava/util/Map<TE;Ljava/lang/Boolean;>;)Ljava/util/Set<TE;>;")]
        pub fn newSetFromMap(map: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.newSetFromMap:(Ljava/util/Map;)Ljava/util/Set;")
        }

        #[java_method(name = "newSequencedSetFromMap", descriptor = "(Ljava/util/SequencedMap;)Ljava/util/SequencedSet;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<E:Ljava/lang/Object;>(Ljava/util/SequencedMap<TE;Ljava/lang/Boolean;>;)Ljava/util/SequencedSet<TE;>;")]
        pub fn newSequencedSetFromMap(map: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.newSequencedSetFromMap:(Ljava/util/SequencedMap;)Ljava/util/SequencedSet;")
        }

        #[java_method(name = "asLifoQueue", descriptor = "(Ljava/util/Deque;)Ljava/util/Queue;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/Deque<TT;>;)Ljava/util/Queue<TT;>;")]
        pub fn asLifoQueue(deque: Object) -> Result<Object> {
            panic!("stub: java/util/Collections.asLifoQueue:(Ljava/util/Deque;)Ljava/util/Queue;")
        }
    }
}
