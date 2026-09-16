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

impl<E: Clone + Default + 'static> From<AbstractSet<E>> for AbstractCollection<E> {
    fn from(v: AbstractSet<E>) -> AbstractCollection<E> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/AbstractSet"]
    #[super_class       = "java/util/AbstractCollection"]
    #[interfaces        = "java/util/Set"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<E:Ljava/lang/Object;>Ljava/util/AbstractCollection<TE;>;Ljava/util/Set<TE;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AbstractSet.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractCollection<E>"]
    #[all_supertypes    = "java/lang/Iterable;java/lang/Object;java/util/AbstractCollection;java/util/AbstractSet;java/util/Collection;java/util/Set"]
    #[has_hash_code_method = true]

    pub struct AbstractSet<E: Clone + Default + 'static>;

    impl<E> AbstractSet<E> {
        #[java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(AbstractCollection::new()?);
            Ok(this)
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/AbstractSet.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "removeAll", descriptor = "(Ljava/util/Collection;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<*>;)Z")]
        pub fn removeAll(&self, c: Object) -> Result<bool> {
            panic!("stub: java/util/AbstractSet.removeAll:(Ljava/util/Collection;)Z")
        }

        #[java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TE;>;")]
        pub fn spliterator(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractSet.spliterator:()Ljava/util/Spliterator;")
        }

        #[java_method(name = "toArray", descriptor = "(Ljava/util/function/IntFunction;)[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/IntFunction<[TT;>;)[TT;")]
        pub fn toArray(&self, generator: Object) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/util/AbstractSet.toArray:(Ljava/util/function/IntFunction;)[Ljava/lang/Object;")
        }

        #[java_method(name = "removeIf", descriptor = "(Ljava/util/function/Predicate;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TE;>;)Z")]
        pub fn removeIf(&self, filter: Object) -> Result<bool> {
            panic!("stub: java/util/AbstractSet.removeIf:(Ljava/util/function/Predicate;)Z")
        }

        #[java_method(name = "stream", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<TE;>;")]
        pub fn stream(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractSet.stream:()Ljava/util/stream/Stream;")
        }

        #[java_method(name = "parallelStream", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<TE;>;")]
        pub fn parallelStream(&self) -> Result<Object> {
            panic!("stub: java/util/AbstractSet.parallelStream:()Ljava/util/stream/Stream;")
        }

        #[java_method(name = "forEach", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TT;>;)V")]
        pub fn forEach(&self, mut action: Object) -> Result<()> {
            let this = self;
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&action))?;
            let _t1 = this.__super().iterator()?;
            let mut local_2: Object = _t1;
            loop {
                let _vdispatch2: bool = if let Some(_d) = local_2.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.hasNext()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.hasNext()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.hasNext()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.hasNext()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(__f) = local_2.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<bool>>>() { (__f)()? } else { Default::default() };
                if !(_vdispatch2) { break; }
                let _vdispatch2: Object = if let Some(_d) = local_2.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.next()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.next()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.next()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.next()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(__f) = local_2.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let mut t: Object = _vdispatch2;
                if let Some(_d) = action.0.as_any().downcast_ref::<WhileOps_UnorderedWhileSpliterator_OfRef_Dropping<Object>>() { _d.accept(Clone::clone(&t))?; } else if let Some(_d) = action.0.as_any().downcast_ref::<WhileOps_UnorderedWhileSpliterator_OfRef_Taking<Object>>() { _d.accept(Clone::clone(&t))?; } else if let Some(_d) = action.0.as_any().downcast_ref::<WhileOps_UnorderedWhileSpliterator_OfRef<Object>>() { _d.accept(Clone::clone(&t))?; } else if let Some(_d) = action.0.as_any().downcast_ref::<Object>() { _d.accept(Clone::clone(&t))?; } else if let Some(__f) = action.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<()>>>() { (__f)(Clone::clone(&t))?; }
            }
            Ok(())
        }
    }
}
