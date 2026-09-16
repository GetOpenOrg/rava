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

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<ImmutableCollections_MapN<K, V>> for ImmutableCollections_AbstractImmutableMap<K, V> {
    fn from(v: ImmutableCollections_MapN<K, V>) -> ImmutableCollections_AbstractImmutableMap<K, V> { v.__into_super() }
}

impl<K: Clone + Default + 'static, V: Clone + Default + 'static> From<ImmutableCollections_MapN<K, V>> for AbstractMap<K, V> {
    fn from(v: ImmutableCollections_MapN<K, V>) -> AbstractMap<K, V> { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/ImmutableCollections$MapN"]
    #[super_class       = "java/util/ImmutableCollections$AbstractImmutableMap"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/util/ImmutableCollections$AbstractImmutableMap<TK;TV;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ImmutableCollections.java"]
    #[inner_classes     = "java/util/ImmutableCollections$AbstractImmutableMap:java/util/ImmutableCollections:AbstractImmutableMap:1032;java/util/ImmutableCollections$MapN:java/util/ImmutableCollections:MapN:24;java/util/ImmutableCollections$MapN$1:::0;java/util/ImmutableCollections$MapN$MapNIterator:java/util/ImmutableCollections$MapN:MapNIterator:0;java/util/Map$Entry:java/util/Map:Entry:1545"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "ImmutableCollections_AbstractImmutableMap<K, V>"]
    #[superclass_fields(keySet: Object, values: Object)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/util/AbstractMap;java/util/ImmutableCollections$AbstractImmutableMap;java/util/ImmutableCollections$MapN;java/util/Map"]
    #[has_hash_code_method = true]

    pub struct ImmutableCollections_MapN<K: Clone + Default + 'static, V: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "table", descriptor = "[Ljava/lang/Object;", access = "package", modifiers = "final", is_static = false))]
        pub table: Rc<RefCell<Vec<Object>>>,
        #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "package", modifiers = "final", is_static = false))]
        pub size: i32,
    }

    impl<K, V> ImmutableCollections_MapN<K, V> {
        #[java_method(name = "<init>", descriptor = "([Ljava/lang/Object;)V", access = "package", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut input: Rc<RefCell<Vec<Object>>>) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(ImmutableCollections_AbstractImmutableMap::new()?);
            if (((input.borrow().len() as i32)&1i32)!=0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.__set_size(((input.borrow().len() as i32)>>((1i32&0x1f))));
            let mut len = (2i32).wrapping_mul((input.borrow().len() as i32));
            len = ((len).wrapping_add(1i32)&-2i32);
            let mut _arr0: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); len as usize]));
            this.__set_table(Clone::clone(&_arr0));
            let mut i: i32 = 0i32;
            loop {
                if i >= (input.borrow().len() as i32) { break; }
                let _t1: Object = Objects::requireNonNull_obj(Clone::clone(&Clone::clone(&input.borrow()[i as usize])))?;
                let mut k: K = _t1;
                let _t2: Object = Objects::requireNonNull_obj(Clone::clone(&Clone::clone(&input.borrow()[(i).wrapping_add(1i32) as usize])))?;
                let mut v: V = _t2;
                let _t3 = this.probe(Clone::clone(&k))?;
                let mut idx: i32 = _t3;
                if (idx>=0) {
                    let _t4 = StringBuilder::new()?.append_str(Clone::clone(&String::from("duplicate key: ")))?;
                    let _t5 = _t4.append_obj(Clone::clone(&k))?;
                    let _t6 = _t5.toString()?;
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                let mut dest = ((idx).wrapping_add(1i32)).wrapping_neg();
                this.__get_table().borrow_mut()[dest as usize] = Object::from_any(k.clone());
                this.__get_table().borrow_mut()[(dest).wrapping_add(1i32) as usize] = Object::from_any(v.clone());
                i = i.wrapping_add(2i32);
            }
            Ok(this)
        }

        #[java_method(name = "containsKey", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn containsKey(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/ImmutableCollections$MapN.containsKey:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "containsValue", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn containsValue(&self, o: Object) -> Result<bool> {
            panic!("stub: java/util/ImmutableCollections$MapN.containsValue:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "get", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;)TV;")]
        pub fn get(&self, o: Object) -> Result<Object> {
            panic!("stub: java/util/ImmutableCollections$MapN.get:(Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "size", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn size(&self) -> Result<i32> {
            panic!("stub: java/util/ImmutableCollections$MapN.size:()I")
        }

        #[java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmpty(&self) -> Result<bool> {
            panic!("stub: java/util/ImmutableCollections$MapN.isEmpty:()Z")
        }

        #[java_method(name = "entrySet", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/util/Map$Entry<TK;TV;>;>;")]
        pub fn entrySet(&self) -> Result<Object> {
            panic!("stub: java/util/ImmutableCollections$MapN.entrySet:()Ljava/util/Set;")
        }

        #[java_method(name = "probe", descriptor = "(Ljava/lang/Object;)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn probe(&self, mut pk: Object) -> Result<i32> {
            let this = self;
            let _vdispatch0: i32 = if let Some(__f) = pk.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            let _t1: i32 = Math::floorMod_i_i(_vdispatch0, ((this.__get_table().borrow().len() as i32)>>((1i32&0x1f))))?;
            let mut idx = (_t1<<(1i32&0x1f));
            loop {
                let mut ek = Clone::clone(&this.__get_table().borrow()[idx as usize]);
                if _is_jnull(&ek) {
                    return Ok(((idx).wrapping_neg()).wrapping_sub(1i32));
                }
                let _vdispatch2: bool = if let Some(__f) = pk.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(Clone::clone(&ek))? } else { Default::default() };
                if _vdispatch2 {
                    return Ok(idx);
                }
                idx = idx.wrapping_add(2i32);
                if idx == (this.__get_table().borrow().len() as i32) {
                    idx = 0i32;
                }
            }
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, in_: Object) -> Result<()> {
            panic!("stub: java/util/ImmutableCollections$MapN.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "writeReplace", descriptor = "()Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn writeReplace(&self) -> Result<Object> {
            panic!("stub: java/util/ImmutableCollections$MapN.writeReplace:()Ljava/lang/Object;")
        }
    }
}
