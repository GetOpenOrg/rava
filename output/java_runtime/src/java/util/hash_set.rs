#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/util/HashSet",
    super_class       = "java/util/AbstractSet",
    interfaces        = "java/util/Set,java/lang/Cloneable,java/io/Serializable",
    access            = "public",
    modifiers         = "",
    generic_signature = "<E:Ljava/lang/Object;>Ljava/util/AbstractSet<TE;>;Ljava/util/Set<TE;>;Ljava/lang/Cloneable;Ljava/io/Serializable;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "HashSet.java",
    inner_classes     = "java/io/ObjectInputStream$GetField:java/io/ObjectInputStream:GetField:1033;java/util/Map$Entry:java/util/Map:Entry:1545;java/util/HashMap$KeySpliterator:java/util/HashMap:KeySpliterator:24",
    all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Object;java/util/AbstractCollection;java/util/AbstractSet;java/util/Collection;java/util/HashSet;java/util/Set",
)]
#[derive(Clone, Default, PartialEq)]
pub struct HashSet<E: Clone + Default + 'static> {
    pub _super: AbstractSet<E>,
    #[cfg_attr(any(), java_field(name = "map", descriptor = "Ljava/util/HashMap;", access = "package", modifiers = "transient", is_static = false, generic_signature = "Ljava/util/HashMap<TE;Ljava/lang/Object;>;"))]
    pub map: JField<HashMap<E, Object>>,
    pub _phantom: std::marker::PhantomData<E>,
}

impl<E: Clone + Default + 'static> HashSet<E> {
    pub fn as_abstract_set(&self) -> &AbstractSet<E> { &self._super }
    pub fn into_abstract_set(self) -> AbstractSet<E> { self._super }
    pub fn as_abstract_collection(&self) -> &AbstractCollection<E> { &self._super._super }
    pub fn into_abstract_collection(self) -> AbstractCollection<E> { self._super._super }
}

impl<E: Clone + Default + 'static> From<HashSet<E>> for AbstractSet<E> {
    fn from(v: HashSet<E>) -> AbstractSet<E> { v._super }
}

impl<E: Clone + Default + 'static> From<HashSet<E>> for AbstractCollection<E> {
    fn from(v: HashSet<E>) -> AbstractCollection<E> { v._super._super }
}

impl<E: Clone + Default + 'static> HashSet<E> {
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

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let mut this = Self { _super: Default::default(), map: JField::new(Default::default()), _phantom: std::marker::PhantomData, ..Default::default() };
        this._super = AbstractSet::new()?;
        this.map.set(Clone::clone(&HashMap::<Object, Object>::new()?));
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/Collection;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Collection<+TE;>;)V"))]
    pub fn new_coll(c: Object) -> Result<Self> {
        panic!("stub: java/util/HashSet.<init>:(Ljava/util/Collection;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(IF)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_i_f(initialCapacity: i32, loadFactor: f32) -> Result<Self> {
        panic!("stub: java/util/HashSet.<init>:(IF)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_i(initialCapacity: i32) -> Result<Self> {
        panic!("stub: java/util/HashSet.<init>:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(IFZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_i_f_z(initialCapacity: i32, loadFactor: f32, dummy: bool) -> Result<Self> {
        panic!("stub: java/util/HashSet.<init>:(IFZ)V")
    }

    #[cfg_attr(any(), java_method(name = "iterator", descriptor = "()Ljava/util/Iterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Iterator<TE;>;"))]
    pub fn iterator(&self) -> Result<Object> {
        panic!("stub: java/util/HashSet.iterator:()Ljava/util/Iterator;")
    }

    #[cfg_attr(any(), java_method(name = "size", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn size(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.map.get().size()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn isEmpty(&self) -> Result<bool> {
        panic!("stub: java/util/HashSet.isEmpty:()Z")
    }

    #[cfg_attr(any(), java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn contains(&self, mut o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.map.get().containsKey(Clone::clone(&o))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "add", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)Z"))]
    pub fn add(&self, mut e: E) -> Result<bool> {
        let this = self;
        let _t0 = this.map.get().put(Clone::clone(&e), Clone::clone(&HashSet::<Object>::PRESENT()))?;
        Ok(_is_jnull(&_t0))
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn remove(&self, o: Object) -> Result<bool> {
        panic!("stub: java/util/HashSet.remove:(Ljava/lang/Object;)Z")
    }

    #[cfg_attr(any(), java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn clear(&self) -> Result<()> {
        panic!("stub: java/util/HashSet.clear:()V")
    }

    #[cfg_attr(any(), java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn clone(&self) -> Result<Object> {
        panic!("stub: java/util/HashSet.clone:()Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn writeObject(&self, s: Object) -> Result<()> {
        panic!("stub: java/util/HashSet.writeObject:(Ljava/io/ObjectOutputStream;)V")
    }

    #[cfg_attr(any(), java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException"))]
    pub fn readObject(&self, s: Object) -> Result<()> {
        panic!("stub: java/util/HashSet.readObject:(Ljava/io/ObjectInputStream;)V")
    }

    #[cfg_attr(any(), java_method(name = "spliterator", descriptor = "()Ljava/util/Spliterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Spliterator<TE;>;"))]
    pub fn spliterator(&self) -> Result<Object> {
        panic!("stub: java/util/HashSet.spliterator:()Ljava/util/Spliterator;")
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "()[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toArray(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/util/HashSet.toArray:()[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "toArray", descriptor = "([Ljava/lang/Object;)[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)[TT;"))]
    pub fn toArray_arr_obj(&self, a: Rc<RefCell<Vec<Object>>>) -> Result<Rc<RefCell<Vec<Object>>>> {
        panic!("stub: java/util/HashSet.toArray:([Ljava/lang/Object;)[Ljava/lang/Object;")
    }

    #[cfg_attr(any(), java_method(name = "newHashSet", descriptor = "(I)Ljava/util/HashSet;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(I)Ljava/util/HashSet<TT;>;"))]
    pub fn newHashSet(numElements: i32) -> Result<HashSet<Object>> {
        panic!("stub: java/util/HashSet.newHashSet:(I)Ljava/util/HashSet;")
    }
}
