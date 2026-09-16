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

impl<E: Clone + Default + 'static> From<LinkedHashSet<E>> for HashSet<E> {
    fn from(v: LinkedHashSet<E>) -> HashSet<E> { v.__into_super() }
}

impl<E: Clone + Default + 'static> From<LinkedHashSet<E>> for AbstractSet<E> {
    fn from(v: LinkedHashSet<E>) -> AbstractSet<E> { v.__into_super().__into_super() }
}

impl<E: Clone + Default + 'static> From<LinkedHashSet<E>> for AbstractCollection<E> {
    fn from(v: LinkedHashSet<E>) -> AbstractCollection<E> { v.__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/LinkedHashSet"]
    #[super_class       = "java/util/HashSet"]
    #[interfaces        = "java/util/SequencedSet,java/lang/Cloneable,java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = "<E:Ljava/lang/Object;>Ljava/util/HashSet<TE;>;Ljava/util/SequencedSet<TE;>;Ljava/lang/Cloneable;Ljava/io/Serializable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "LinkedHashSet.java"]
    #[inner_classes     = "java/util/LinkedHashSet$1ReverseLinkedHashSetView::ReverseLinkedHashSetView:0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "HashSet<E>"]
    #[superclass_fields(map: HashMap<E, Object>)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Iterable;java/lang/Object;java/util/AbstractCollection;java/util/AbstractSet;java/util/Collection;java/util/HashSet;java/util/LinkedHashSet;java/util/SequencedSet;java/util/Set"]

    pub struct LinkedHashSet<E: Clone + Default + 'static>;

    impl<E> LinkedHashSet<E> {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-2851667679971038690"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -2851667679971038690i64
        }

        #[java_method(name = "<init>", descriptor = "(IF)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_f(initialCapacity: i32, loadFactor: f32) -> Result<Self> {
            panic!("stub: java/util/LinkedHashSet.<init>:(IF)V")
        }

        #[java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i(initialCapacity: i32) -> Result<Self> {
            panic!("stub: java/util/LinkedHashSet.<init>:(I)V")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(HashSet::new_i_f_z(16i32, 0.75f32, (1i32 != 0i32))?);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Collection;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<+TE;>;)V")]
        pub fn new_coll(c: Object) -> Result<Self> {
            panic!("stub: java/util/LinkedHashSet.<init>:(Ljava/util/Collection;)V")
        }

        #[java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TE;>;")]
        pub fn spliterator(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedHashSet.spliterator:()Ljava/util/Spliterator;")
        }

        #[java_method(name = "newLinkedHashSet", descriptor = "(I)Ljava/util/LinkedHashSet;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(I)Ljava/util/LinkedHashSet<TT;>;")]
        pub fn newLinkedHashSet(numElements: i32) -> Result<LinkedHashSet<Object>> {
            panic!("stub: java/util/LinkedHashSet.newLinkedHashSet:(I)Ljava/util/LinkedHashSet;")
        }

        #[java_method(name = "map", descriptor = "()Ljava/util/LinkedHashMap;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/LinkedHashMap<TE;Ljava/lang/Object;>;")]
        pub fn map(&self) -> Result<LinkedHashMap<Object, Object>> {
            panic!("stub: java/util/LinkedHashSet.map:()Ljava/util/LinkedHashMap;")
        }

        #[java_method(name = "addFirst", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V")]
        pub fn addFirst(&self, e: E) -> Result<()> {
            panic!("stub: java/util/LinkedHashSet.addFirst:(Ljava/lang/Object;)V")
        }

        #[java_method(name = "addLast", descriptor = "(Ljava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)V")]
        pub fn addLast(&self, e: E) -> Result<()> {
            panic!("stub: java/util/LinkedHashSet.addLast:(Ljava/lang/Object;)V")
        }

        #[java_method(name = "getFirst", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn getFirst(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedHashSet.getFirst:()Ljava/lang/Object;")
        }

        #[java_method(name = "getLast", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn getLast(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedHashSet.getLast:()Ljava/lang/Object;")
        }

        #[java_method(name = "removeFirst", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn removeFirst(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedHashSet.removeFirst:()Ljava/lang/Object;")
        }

        #[java_method(name = "removeLast", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TE;")]
        pub fn removeLast(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedHashSet.removeLast:()Ljava/lang/Object;")
        }

        #[java_method(name = "reversed", descriptor = "()Ljava/util/SequencedSet;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/SequencedSet<TE;>;")]
        pub fn reversed(&self) -> Result<Object> {
            panic!("stub: java/util/LinkedHashSet.reversed:()Ljava/util/SequencedSet;")
        }
    }
}
