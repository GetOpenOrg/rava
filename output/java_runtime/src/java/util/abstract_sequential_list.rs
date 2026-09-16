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

impl<E: Clone + Default + 'static> From<AbstractSequentialList<E>> for AbstractList<E> {
    fn from(v: AbstractSequentialList<E>) -> AbstractList<E> { v.__into_super() }
}

impl<E: Clone + Default + 'static> From<AbstractSequentialList<E>> for AbstractCollection<E> {
    fn from(v: AbstractSequentialList<E>) -> AbstractCollection<E> { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/AbstractSequentialList"]
    #[super_class       = "java/util/AbstractList"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<E:Ljava/lang/Object;>Ljava/util/AbstractList<TE;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AbstractSequentialList.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractList<E>"]
    #[superclass_fields(modCount: i32)]
    #[all_supertypes    = "java/lang/Iterable;java/lang/Object;java/util/AbstractCollection;java/util/AbstractList;java/util/AbstractSequentialList;java/util/Collection;java/util/List;java/util/SequencedCollection"]

    pub struct AbstractSequentialList<E: Clone + Default + 'static>;

    impl<E> AbstractSequentialList<E> {
        #[java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(AbstractList::new()?);
            Ok(this)
        }

        #[java_method(name = "get", descriptor = "(I)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)TE;")]
        pub fn get(&self, index: i32) -> Result<Object> {
            panic!("stub: java/util/AbstractSequentialList.get:(I)Ljava/lang/Object;")
        }

        #[java_method(name = "set", descriptor = "(ILjava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITE;)TE;")]
        pub fn set(&self, index: i32, element: E) -> Result<Object> {
            panic!("stub: java/util/AbstractSequentialList.set:(ILjava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "add", descriptor = "(ILjava/lang/Object;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ITE;)V")]
        pub fn add(&self, index: i32, element: E) -> Result<()> {
            panic!("stub: java/util/AbstractSequentialList.add:(ILjava/lang/Object;)V")
        }

        #[java_method(name = "remove", descriptor = "(I)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)TE;")]
        pub fn remove(&self, index: i32) -> Result<Object> {
            panic!("stub: java/util/AbstractSequentialList.remove:(I)Ljava/lang/Object;")
        }

        #[java_method(name = "addAll", descriptor = "(ILjava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(ILjava/util/Collection<+TE;>;)Z")]
        pub fn addAll(&self, index: i32, c: Object) -> Result<bool> {
            panic!("stub: java/util/AbstractSequentialList.addAll:(ILjava/util/Collection;)Z")
        }

        #[java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TE;>;")]
        pub fn iterator(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractSequentialList.iterator:()Ljava/util/Iterator;")
        }

        #[java_method(name = "listIterator", descriptor = "(I)Ljava/util/ListIterator;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "(I)Ljava/util/ListIterator<TE;>;")]
        pub fn listIterator(&self, arg0: i32) -> Result<Object> {
            panic!("stub: java/util/AbstractSequentialList.listIterator:(I)Ljava/util/ListIterator;")
        }
    }
}
