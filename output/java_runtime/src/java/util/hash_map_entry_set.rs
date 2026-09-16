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

impl From<HashMap_EntrySet> for AbstractSet<Object> {
    fn from(v: HashMap_EntrySet) -> AbstractSet<Object> { v.__into_super() }
}

impl From<HashMap_EntrySet> for AbstractCollection<Object> {
    fn from(v: HashMap_EntrySet) -> AbstractCollection<Object> { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/HashMap$EntrySet"]
    #[super_class       = "java/util/AbstractSet"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = "Ljava/util/AbstractSet<Ljava/util/Map$Entry<TK;TV;>;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "HashMap.java"]
    #[inner_classes     = "java/util/HashMap$EntrySet:java/util/HashMap:EntrySet:16;java/util/HashMap$EntryIterator:java/util/HashMap:EntryIterator:16;java/util/Map$Entry:java/util/Map:Entry:1545;java/util/HashMap$Node:java/util/HashMap:Node:8;java/util/HashMap$EntrySpliterator:java/util/HashMap:EntrySpliterator:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractSet<Object>"]
    #[all_supertypes    = "java/lang/Iterable;java/lang/Object;java/util/AbstractCollection;java/util/AbstractSet;java/util/Collection;java/util/HashMap$EntrySet;java/util/Set"]

    pub struct HashMap_EntrySet {
        #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/util/HashMap;", access = "package", modifiers = "final synthetic", is_static = false))]
        pub this_0: HashMap<Object, Object>,
    }

    impl HashMap_EntrySet {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/HashMap;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":32784")]
        pub fn new(mut this_0: HashMap<Object, Object>) -> Result<Self> {
            let mut this = Self::default();
            this.__set_this_0(Clone::clone(&this_0));
            this = Self::__new_with_super(AbstractSet::new()?);
            Ok(this)
        }

        #[java_method(name = "size", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn size(&self) -> Result<i32> {
            panic!("stub: java/util/HashMap$EntrySet.size:()I")
        }

        #[java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<()> {
            panic!("stub: java/util/HashMap$EntrySet.clear:()V")
        }

        #[java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<Ljava/util/Map$Entry<TK;TV;>;>;")]
        pub fn iterator(&self) -> Result<Object> {
            panic!("stub: java/util/HashMap$EntrySet.iterator:()Ljava/util/Iterator;")
        }

        #[java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn contains(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/HashMap$EntrySet.contains:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remove(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/HashMap$EntrySet.remove:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<Ljava/util/Map$Entry<TK;TV;>;>;")]
        pub fn spliterator(&self) -> Result<Object> {
            panic!("stub: java/util/HashMap$EntrySet.spliterator:()Ljava/util/Spliterator;")
        }

        #[java_method(name = "forEach", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-Ljava/util/Map$Entry<TK;TV;>;>;)V")]
        pub fn forEach(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/HashMap$EntrySet.forEach:(Ljava/util/function/Consumer;)V")
        }
    }
}
