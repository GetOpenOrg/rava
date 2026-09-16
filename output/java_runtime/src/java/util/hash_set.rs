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

impl<E: Clone + Default + 'static> From<HashSet<E>> for AbstractSet<E> {
    fn from(v: HashSet<E>) -> AbstractSet<E> { v.__into_super() }
}

impl<E: Clone + Default + 'static> From<HashSet<E>> for AbstractCollection<E> {
    fn from(v: HashSet<E>) -> AbstractCollection<E> { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/HashSet"]
    #[super_class       = "java/util/AbstractSet"]
    #[interfaces        = "java/util/Set,java/lang/Cloneable,java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = "<E:Ljava/lang/Object;>Ljava/util/AbstractSet<TE;>;Ljava/util/Set<TE;>;Ljava/lang/Cloneable;Ljava/io/Serializable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "HashSet.java"]
    #[inner_classes     = "java/io/ObjectInputStream$GetField:java/io/ObjectInputStream:GetField:1033;java/util/Map$Entry:java/util/Map:Entry:1545;java/util/HashMap$KeySpliterator:java/util/HashMap:KeySpliterator:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractSet<E>"]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Iterable;java/lang/Object;java/util/AbstractCollection;java/util/AbstractSet;java/util/Collection;java/util/HashSet;java/util/Set"]

    pub struct HashSet<E: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "map", descriptor = "Ljava/util/HashMap;", access = "package", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/HashMap<TE;Ljava/lang/Object;>;"))]
        pub map: HashMap<E, Object>,
    }

    impl<E> HashSet<E> {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "-5024744406713321676"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -5024744406713321676i64
        }

        #[cfg_attr(any(), java_field(name = "PRESENT", descriptor = "Ljava/lang/Object;", access = "package", modifiers = "static final", is_static = true))]
        // static field: PRESENT:Ljava/lang/Object;
        pub fn PRESENT() -> Object {
            panic!("stub: java/util/HashSet.PRESENT:Ljava/lang/Object;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(AbstractSet::new()?);
            this.__set_map(Clone::clone(&HashMap::<Object, Object>::new()?));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Collection;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<+TE;>;)V")]
        pub fn new_coll(c: Object) -> Result<Self> {
            panic!("stub: java/util/HashSet.<init>:(Ljava/util/Collection;)V")
        }

        #[java_method(name = "<init>", descriptor = "(IF)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_f(initialCapacity: i32, loadFactor: f32) -> Result<Self> {
            panic!("stub: java/util/HashSet.<init>:(IF)V")
        }

        #[java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i(initialCapacity: i32) -> Result<Self> {
            panic!("stub: java/util/HashSet.<init>:(I)V")
        }

        #[java_method(name = "<init>", descriptor = "(IFZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(IFZ)V
        pub fn new_i_f_z(mut initialCapacity: i32, mut loadFactor: f32, mut dummy: bool) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(AbstractSet::new()?);
            this.__set_map(Clone::clone(&<_ as Into<HashMap<Object, Object>>>::into(LinkedHashMap::<Object, Object>::new_i_f(initialCapacity, loadFactor)?)));
            Ok(this)
        }

        #[java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TE;>;")]
        pub fn iterator(&self) -> Result<Object> {
            let this = self;
            let _t0 = this.__get_map().keySet()?;
            let _vdispatch1: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<HashMap_EntrySet>() { _d.iterator()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<HashMap_KeySet>() { _d.iterator()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<TreeMap_EntrySet>() { _d.iterator()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<LinkedHashSet<Object>>() { _d.iterator()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<AbstractSet<Object>>() { _d.iterator()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<HashSet<Object>>() { _d.iterator()? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.iterator()? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            Ok(_vdispatch1)
        }

        #[java_method(name = "size", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn size(&self) -> Result<i32> {
            let this = self;
            let _t0 = this.__get_map().size()?;
            Ok(_t0)
        }

        #[java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmpty(&self) -> Result<bool> {
            panic!("stub: java/util/HashSet.isEmpty:()Z")
        }

        #[java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn contains(&self, mut o: Object) -> Result<bool> {
            let this = self;
            let _t0 = this.__get_map().containsKey(Clone::clone(&o))?;
            Ok(_t0)
        }

        #[java_method(name = "add", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)Z")]
        pub fn add(&self, mut e: E) -> Result<bool> {
            let this = self;
            let _t0 = this.__get_map().put(Clone::clone(&e), Clone::clone(&HashSet::<Object>::PRESENT()))?;
            Ok(_is_jnull(&_t0))
        }

        #[java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remove(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/HashSet.remove:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<()> {
            panic!("stub: java/util/HashSet.clear:()V")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/util/HashSet.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/HashSet.writeObject:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/HashSet.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TE;>;")]
        pub fn spliterator(&self) -> Result<Object> {
            panic!("stub: java/util/HashSet.spliterator:()Ljava/util/Spliterator;")
        }

        #[java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toArray(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/util/HashSet.toArray:()[Ljava/lang/Object;")
        }

        #[java_method(name = "toArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;")]
        // java: toArray([Ljava/lang/Object;)[Ljava/lang/Object;
        pub fn toArray_arr_obj(&self, mut a: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
            let this = self;
            let _t0 = this.__get_map().prepareArray(Clone::clone(&a))?;
            let _t1 = this.__get_map().keysToArray(Clone::clone(&_t0))?;
            Ok(_t1)
        }

        #[java_method(name = "newHashSet", descriptor = "(I)Ljava/util/HashSet;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(I)Ljava/util/HashSet<TT;>;")]
        pub fn newHashSet(numElements: i32) -> Result<HashSet<Object>> {
            panic!("stub: java/util/HashSet.newHashSet:(I)Ljava/util/HashSet;")
        }

        #[java_method(name = "toArray", descriptor = "(Ljava/util/function/IntFunction;)[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(Ljava/util/function/IntFunction<[TT;>;)[TT;")]
        pub fn toArray_intfun(&self, generator: Object) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/util/HashSet.toArray:(Ljava/util/function/IntFunction;)[Ljava/lang/Object;")
        }

        #[java_method(name = "removeIf", descriptor = "(Ljava/util/function/Predicate;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TE;>;)Z")]
        pub fn removeIf(&self, filter: Object) -> Result<bool> {
            panic!("stub: java/util/HashSet.removeIf:(Ljava/util/function/Predicate;)Z")
        }

        #[java_method(name = "stream", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<TE;>;")]
        pub fn stream(&self) -> Result<Object> {
            panic!("stub: java/util/HashSet.stream:()Ljava/util/stream/Stream;")
        }

        #[java_method(name = "parallelStream", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<TE;>;")]
        pub fn parallelStream(&self) -> Result<Object> {
            panic!("stub: java/util/HashSet.parallelStream:()Ljava/util/stream/Stream;")
        }

        #[java_method(name = "forEach", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TT;>;)V")]
        pub fn forEach(&self, mut action: Object) -> Result<()> {
            let this = self;
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&action))?;
            let _t1 = this.iterator()?;
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
