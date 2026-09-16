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

impl<E: Clone + Default + 'static> From<AbstractList_RandomAccessSubList<E>> for AbstractList_SubList<E> {
    fn from(v: AbstractList_RandomAccessSubList<E>) -> AbstractList_SubList<E> { v.__into_super() }
}

impl<E: Clone + Default + 'static> From<AbstractList_RandomAccessSubList<E>> for AbstractList<E> {
    fn from(v: AbstractList_RandomAccessSubList<E>) -> AbstractList<E> { v.__into_super().__into_super() }
}

impl<E: Clone + Default + 'static> From<AbstractList_RandomAccessSubList<E>> for AbstractCollection<E> {
    fn from(v: AbstractList_RandomAccessSubList<E>) -> AbstractCollection<E> { v.__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/AbstractList$RandomAccessSubList"]
    #[super_class       = "java/util/AbstractList$SubList"]
    #[interfaces        = "java/util/RandomAccess"]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "<E:Ljava/lang/Object;>Ljava/util/AbstractList$SubList<TE;>;Ljava/util/RandomAccess;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AbstractList.java"]
    #[inner_classes     = "java/util/AbstractList$SubList:java/util/AbstractList:SubList:10;java/util/AbstractList$RandomAccessSubList:java/util/AbstractList:RandomAccessSubList:10"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractList_SubList<E>"]
    #[superclass_fields(modCount: i32, root: AbstractList<E>, parent: AbstractList_SubList<E>, offset: i32, size: i32)]
    #[all_supertypes    = "java/lang/Iterable;java/lang/Object;java/util/AbstractCollection;java/util/AbstractList;java/util/AbstractList$RandomAccessSubList;java/util/AbstractList$SubList;java/util/Collection;java/util/List;java/util/RandomAccess;java/util/SequencedCollection"]

    pub struct AbstractList_RandomAccessSubList<E: Clone + Default + 'static>;

    impl<E> AbstractList_RandomAccessSubList<E> {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/AbstractList;II)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/AbstractList<TE;>;II)V")]
        // java: <init>(Ljava/util/AbstractList;II)V
        pub fn new_abstra_i_i(mut root: AbstractList<E>, mut fromIndex: i32, mut toIndex: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(AbstractList_SubList::new_abstra_i_i(Clone::clone(&root), fromIndex, toIndex)?);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/AbstractList$RandomAccessSubList;II)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/AbstractList$RandomAccessSubList<TE;>;II)V")]
        pub fn new_abstra_i_i_1(parent: AbstractList_RandomAccessSubList<E>, fromIndex: i32, toIndex: i32) -> Result<Self> {
            panic!("stub: java/util/AbstractList$RandomAccessSubList.<init>:(Ljava/util/AbstractList$RandomAccessSubList;II)V")
        }

        #[java_method(name = "subList", descriptor = "(II)Ljava/util/List;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(II)Ljava/util/List<TE;>;")]
        pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
            panic!("stub: java/util/AbstractList$RandomAccessSubList.subList:(II)Ljava/util/List;")
        }
    }
}
