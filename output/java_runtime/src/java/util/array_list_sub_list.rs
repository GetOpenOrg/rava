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

impl<E: Clone + Default + 'static> From<ArrayList_SubList<E>> for AbstractList<E> {
    fn from(v: ArrayList_SubList<E>) -> AbstractList<E> { v.__into_super() }
}

impl<E: Clone + Default + 'static> From<ArrayList_SubList<E>> for AbstractCollection<E> {
    fn from(v: ArrayList_SubList<E>) -> AbstractCollection<E> { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/ArrayList$SubList"]
    #[super_class       = "java/util/AbstractList"]
    #[interfaces        = "java/util/RandomAccess"]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "<E:Ljava/lang/Object;>Ljava/util/AbstractList<TE;>;Ljava/util/RandomAccess;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ArrayList.java"]
    #[inner_classes     = "java/util/ArrayList$SubList:java/util/ArrayList:SubList:10;java/util/ArrayList$SubList$1:::0;java/util/ArrayList$SubList$2:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractList<E>"]
    #[superclass_fields(modCount: i32)]
    #[all_supertypes    = "java/lang/Iterable;java/lang/Object;java/util/AbstractCollection;java/util/AbstractList;java/util/ArrayList$SubList;java/util/Collection;java/util/List;java/util/RandomAccess;java/util/SequencedCollection"]
    #[has_hash_code_method = true]

    pub struct ArrayList_SubList<E: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "root", descriptor = "Ljava/util/ArrayList;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/ArrayList<TE;>;"))]
        pub root: ArrayList<E>,
        #[cfg_attr(any(), java_field(name = "parent", descriptor = "Ljava/util/ArrayList$SubList;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/ArrayList$SubList<TE;>;"))]
        pub parent: ArrayList_SubList<E>,
        #[cfg_attr(any(), java_field(name = "offset", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub offset: i32,
        #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub size: i32,
    }

    impl<E> ArrayList_SubList<E> {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/ArrayList;II)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/ArrayList<TE;>;II)V")]
        // java: <init>(Ljava/util/ArrayList;II)V
        pub fn new_arrayl_i_i(mut root: ArrayList<E>, mut fromIndex: i32, mut toIndex: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(AbstractList::new()?);
            this.__set_root(Clone::clone(&root));
            this.__set_parent(Default::default());
            this.__set_offset(fromIndex);
            this.__set_size((toIndex).wrapping_sub(fromIndex));
            this.__set_modCount(root.__get_modCount());
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/ArrayList$SubList;II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/ArrayList$SubList<TE;>;II)V")]
        pub fn new_arrayl_i_i_1(parent: ArrayList_SubList<E>, fromIndex: i32, toIndex: i32) -> Result<Self> {
            panic!("stub: java/util/ArrayList$SubList.<init>:(Ljava/util/ArrayList$SubList;II)V")
        }

        #[java_method(name = "set", descriptor = "(ILjava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITE;)TE;")]
        pub fn set(&self, index: i32, element: E) -> Result<Object> {
            panic!("stub: java/util/ArrayList$SubList.set:(ILjava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "get", descriptor = "(I)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)TE;")]
        pub fn get(&self, index: i32) -> Result<Object> {
            panic!("stub: java/util/ArrayList$SubList.get:(I)Ljava/lang/Object;")
        }

        #[java_method(name = "size", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn size(&self) -> Result<i32> {
            panic!("stub: java/util/ArrayList$SubList.size:()I")
        }

        #[java_method(name = "add", descriptor = "(ILjava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITE;)V")]
        pub fn add(&self, index: i32, element: E) -> Result<()> {
            panic!("stub: java/util/ArrayList$SubList.add:(ILjava/lang/Object;)V")
        }

        #[java_method(name = "remove", descriptor = "(I)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)TE;")]
        pub fn remove(&self, index: i32) -> Result<Object> {
            panic!("stub: java/util/ArrayList$SubList.remove:(I)Ljava/lang/Object;")
        }

        #[java_method(name = "removeRange", descriptor = "(II)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn removeRange(&self, fromIndex: i32, toIndex: i32) -> Result<()> {
            panic!("stub: java/util/ArrayList$SubList.removeRange:(II)V")
        }

        #[java_method(name = "addAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<+TE;>;)Z")]
        pub fn addAll_coll(&self, c: Object) -> Result<bool> {
            panic!("stub: java/util/ArrayList$SubList.addAll:(Ljava/util/Collection;)Z")
        }

        #[java_method(name = "addAll", descriptor = "(ILjava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/util/Collection<+TE;>;)Z")]
        pub fn addAll_i_coll(&self, index: i32, c: Object) -> Result<bool> {
            panic!("stub: java/util/ArrayList$SubList.addAll:(ILjava/util/Collection;)Z")
        }

        #[java_method(name = "replaceAll", descriptor = "(Ljava/util/function/UnaryOperator;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/UnaryOperator<TE;>;)V")]
        pub fn replaceAll(&self, operator: Object) -> Result<()> {
            panic!("stub: java/util/ArrayList$SubList.replaceAll:(Ljava/util/function/UnaryOperator;)V")
        }

        #[java_method(name = "removeAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z")]
        pub fn removeAll(&self, c: Object) -> Result<bool> {
            panic!("stub: java/util/ArrayList$SubList.removeAll:(Ljava/util/Collection;)Z")
        }

        #[java_method(name = "retainAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z")]
        pub fn retainAll(&self, c: Object) -> Result<bool> {
            panic!("stub: java/util/ArrayList$SubList.retainAll:(Ljava/util/Collection;)Z")
        }

        #[java_method(name = "batchRemove", descriptor = "(Ljava/util/Collection;Z)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;Z)Z")]
        pub fn batchRemove(&self, c: Object, complement: bool) -> Result<bool> {
            panic!("stub: java/util/ArrayList$SubList.batchRemove:(Ljava/util/Collection;Z)Z")
        }

        #[java_method(name = "removeIf", descriptor = "(Ljava/util/function/Predicate;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TE;>;)Z")]
        pub fn removeIf(&self, filter: Object) -> Result<bool> {
            panic!("stub: java/util/ArrayList$SubList.removeIf:(Ljava/util/function/Predicate;)Z")
        }

        #[java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toArray(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/util/ArrayList$SubList.toArray:()[Ljava/lang/Object;")
        }

        #[java_method(name = "toArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;")]
        pub fn toArray_arr_obj(&self, a: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/util/ArrayList$SubList.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/ArrayList$SubList.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "indexOf", descriptor = "(Ljava/lang/Object;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOf(&self, o: Object) -> Result<i32> {
            panic!("stub: java/util/ArrayList$SubList.indexOf:(Ljava/lang/Object;)I")
        }

        #[java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/Object;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lastIndexOf(&self, o: Object) -> Result<i32> {
            panic!("stub: java/util/ArrayList$SubList.lastIndexOf:(Ljava/lang/Object;)I")
        }

        #[java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn contains(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/ArrayList$SubList.contains:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TE;>;")]
        pub fn iterator(&self) -> Result<Object> {
            panic!("stub: java/util/ArrayList$SubList.iterator:()Ljava/util/Iterator;")
        }

        #[java_method(name = "listIterator", descriptor = "(I)Ljava/util/ListIterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)Ljava/util/ListIterator<TE;>;")]
        pub fn listIterator(&self, index: i32) -> Result<Object> {
            panic!("stub: java/util/ArrayList$SubList.listIterator:(I)Ljava/util/ListIterator;")
        }

        #[java_method(name = "subList", descriptor = "(II)Ljava/util/List;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(II)Ljava/util/List<TE;>;")]
        pub fn subList(&self, fromIndex: i32, toIndex: i32) -> Result<Object> {
            panic!("stub: java/util/ArrayList$SubList.subList:(II)Ljava/util/List;")
        }

        #[java_method(name = "rangeCheckForAdd", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn rangeCheckForAdd(&self, index: i32) -> Result<()> {
            panic!("stub: java/util/ArrayList$SubList.rangeCheckForAdd:(I)V")
        }

        #[java_method(name = "outOfBoundsMsg", descriptor = "(I)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn outOfBoundsMsg(&self, index: i32) -> Result<String> {
            panic!("stub: java/util/ArrayList$SubList.outOfBoundsMsg:(I)Ljava/lang/String;")
        }

        #[java_method(name = "checkForComodification", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkForComodification(&self) -> Result<()> {
            panic!("stub: java/util/ArrayList$SubList.checkForComodification:()V")
        }

        #[java_method(name = "updateSizeAndModCount", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn updateSizeAndModCount(&self, sizeChange: i32) -> Result<()> {
            panic!("stub: java/util/ArrayList$SubList.updateSizeAndModCount:(I)V")
        }

        #[java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TE;>;")]
        pub fn spliterator(&self) -> Result<Object> {
            panic!("stub: java/util/ArrayList$SubList.spliterator:()Ljava/util/Spliterator;")
        }
    }
}
