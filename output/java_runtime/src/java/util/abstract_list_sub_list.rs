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

impl<E: Clone + Default + 'static> From<AbstractList_SubList<E>> for AbstractList<E> {
    fn from(v: AbstractList_SubList<E>) -> AbstractList<E> { v.__into_super() }
}

impl<E: Clone + Default + 'static> From<AbstractList_SubList<E>> for AbstractCollection<E> {
    fn from(v: AbstractList_SubList<E>) -> AbstractCollection<E> { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/AbstractList$SubList"]
    #[super_class       = "java/util/AbstractList"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "<E:Ljava/lang/Object;>Ljava/util/AbstractList<TE;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AbstractList.java"]
    #[inner_classes     = "java/util/AbstractList$SubList:java/util/AbstractList:SubList:10;java/util/AbstractList$SubList$1:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractList<E>"]
    #[superclass_fields(modCount: i32)]
    #[all_supertypes    = "java/lang/Iterable;java/lang/Object;java/util/AbstractCollection;java/util/AbstractList;java/util/AbstractList$SubList;java/util/Collection;java/util/List;java/util/SequencedCollection"]

    pub struct AbstractList_SubList<E: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "root", descriptor = "Ljava/util/AbstractList;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/AbstractList<TE;>;"))]
        pub root: AbstractList<E>,
        #[cfg_attr(any(), java_field(name = "parent", descriptor = "Ljava/util/AbstractList$SubList;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/AbstractList$SubList<TE;>;"))]
        pub parent: AbstractList_SubList<E>,
        #[cfg_attr(any(), java_field(name = "offset", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub offset: i32,
        #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "protected", modifiers = "", is_static = false))]
        pub size: i32,
    }

    impl<E> AbstractList_SubList<E> {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/AbstractList;II)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/AbstractList<TE;>;II)V")]
        // java: <init>(Ljava/util/AbstractList;II)V
        pub fn new_abstra_i_i(mut root: AbstractList<E>, mut fromIndex: i32, mut toIndex: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(AbstractList::new()?);
            this.__set_root(Clone::clone(&root));
            this.__set_parent(Default::default());
            this.__set_offset(fromIndex);
            this.__set_size((toIndex).wrapping_sub(fromIndex));
            this.__set_modCount(root.__get_modCount());
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/AbstractList$SubList;II)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/AbstractList$SubList<TE;>;II)V")]
        pub fn new_abstra_i_i_1(parent: AbstractList_SubList<E>, fromIndex: i32, toIndex: i32) -> Result<Self> {
            panic!("stub: java/util/AbstractList$SubList.<init>:(Ljava/util/AbstractList$SubList;II)V")
        }

        #[java_method(name = "set", descriptor = "(ILjava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITE;)TE;")]
        pub fn set(&self, index: i32, element: E) -> Result<Object> {
            panic!("stub: java/util/AbstractList$SubList.set:(ILjava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "get", descriptor = "(I)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)TE;")]
        pub fn get(&self, index: i32) -> Result<Object> {
            panic!("stub: java/util/AbstractList$SubList.get:(I)Ljava/lang/Object;")
        }

        #[java_method(name = "size", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn size(&self) -> Result<i32> {
            panic!("stub: java/util/AbstractList$SubList.size:()I")
        }

        #[java_method(name = "add", descriptor = "(ILjava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITE;)V")]
        pub fn add(&self, index: i32, element: E) -> Result<()> {
            panic!("stub: java/util/AbstractList$SubList.add:(ILjava/lang/Object;)V")
        }

        #[java_method(name = "remove", descriptor = "(I)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)TE;")]
        pub fn remove(&self, index: i32) -> Result<Object> {
            panic!("stub: java/util/AbstractList$SubList.remove:(I)Ljava/lang/Object;")
        }

        #[java_method(name = "removeRange", descriptor = "(II)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn removeRange(&self, fromIndex: i32, toIndex: i32) -> Result<()> {
            panic!("stub: java/util/AbstractList$SubList.removeRange:(II)V")
        }

        #[java_method(name = "addAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<+TE;>;)Z")]
        pub fn addAll_coll(&self, c: Object) -> Result<bool> {
            panic!("stub: java/util/AbstractList$SubList.addAll:(Ljava/util/Collection;)Z")
        }

        #[java_method(name = "addAll", descriptor = "(ILjava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/util/Collection<+TE;>;)Z")]
        pub fn addAll_i_coll(&self, index: i32, c: Object) -> Result<bool> {
            panic!("stub: java/util/AbstractList$SubList.addAll:(ILjava/util/Collection;)Z")
        }

        #[java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TE;>;")]
        pub fn iterator(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractList$SubList.iterator:()Ljava/util/Iterator;")
        }

        #[java_method(name = "listIterator", descriptor = "(I)Ljava/util/ListIterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)Ljava/util/ListIterator<TE;>;")]
        pub fn listIterator(&self, index: i32) -> Result<Object> {
            panic!("stub: java/util/AbstractList$SubList.listIterator:(I)Ljava/util/ListIterator;")
        }

        #[java_method(name = "subList", descriptor = "(II)Ljava/util/List;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(II)Ljava/util/List<TE;>;")]
        pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
            panic!("stub: java/util/AbstractList$SubList.subList:(II)Ljava/util/List;")
        }

        #[java_method(name = "rangeCheckForAdd", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn rangeCheckForAdd(&self, index: i32) -> Result<()> {
            panic!("stub: java/util/AbstractList$SubList.rangeCheckForAdd:(I)V")
        }

        #[java_method(name = "outOfBoundsMsg", descriptor = "(I)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn outOfBoundsMsg(&self, index: i32) -> Result<String> {
            panic!("stub: java/util/AbstractList$SubList.outOfBoundsMsg:(I)Ljava/lang/String;")
        }

        #[java_method(name = "checkForComodification", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkForComodification(&self) -> Result<()> {
            panic!("stub: java/util/AbstractList$SubList.checkForComodification:()V")
        }

        #[java_method(name = "updateSizeAndModCount", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn updateSizeAndModCount(&self, sizeChange: i32) -> Result<()> {
            panic!("stub: java/util/AbstractList$SubList.updateSizeAndModCount:(I)V")
        }
    }
}
