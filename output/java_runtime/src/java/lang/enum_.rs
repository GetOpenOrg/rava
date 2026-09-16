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

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/Enum"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/lang/constant/Constable,java/lang/Comparable,java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = "<E:Ljava/lang/Enum<TE;>;>Ljava/lang/Object;Ljava/lang/constant/Constable;Ljava/lang/Comparable<TE;>;Ljava/io/Serializable;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Enum.java"]
    #[inner_classes     = "java/lang/Enum$EnumDesc:java/lang/Enum:EnumDesc:25;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Enum;java/lang/Object;java/lang/constant/Constable"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct Enum<E: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub name: String,
        #[cfg_attr(any(), java_field(name = "ordinal", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub ordinal: i32,
        #[cfg_attr(any(), java_field(name = "hash", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub hash: i32,
    }

    impl<E> Enum<E> {
        #[java_method(name = "name", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn name(&self) -> Result<String> {
            let this = self;
            Ok(this.__get_name())
        }

        #[java_method(name = "ordinal", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ordinal(&self) -> Result<i32> {
            panic!("stub: java/lang/Enum.ordinal:()I")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;I)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut name: String, mut ordinal: i32) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_name(Clone::clone(&name));
            this.__set_ordinal(ordinal);
            Ok(this)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, other: Object) -> Result<bool> {
            panic!("stub: java/lang/Enum.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "protected", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/CloneNotSupportedException")]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/lang/Enum.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/lang/Enum;)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TE;)I")]
        pub fn compareTo(&self, mut o: E) -> Result<i32> {
            let this = self;
            let mut other: E = o;
            let mut self_ = this.clone();
            let _t0 = self_.getClass()?;
            let _t1 = other.getClass()?;
            let _t2 = self_.getDeclaringClass()?;
            let _t3 = other.getDeclaringClass()?;
            if _t2 != _t3 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok((self_.__get_ordinal()).wrapping_sub(other.__get_ordinal()))
        }

        #[java_method(name = "getDeclaringClass", descriptor = "()Ljava/lang/Class;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/lang/Class<TE;>;")]
        pub fn getDeclaringClass(&self) -> Result<Class<E>> {
            let this = self;
            let _t0 = this.getClass()?;
            let mut clazz = (_t0).downcast::<Class<Object>>();
            let _t1 = clazz.getSuperclass()?;
            let mut zuper = (_t1).downcast::<Class<Object>>();
            Ok(Default::default())
        }

        #[java_method(name = "describeConstable", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Optional<Ljava/lang/Enum$EnumDesc<TE;>;>;")]
        pub fn describeConstable(&self) -> Result<Optional<Object>> {
            panic!("stub: java/lang/Enum.describeConstable:()Ljava/util/Optional;")
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Enum<TT;>;>(Ljava/lang/Class<TT;>;Ljava/lang/String;)TT;")]
        pub fn valueOf(mut enumClass: Class<Object>, mut name: String) -> Result<Object> {
            let _t0 = enumClass.enumConstantDirectory()?;
            let _vdispatch1: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<ImmutableCollections_MapN<Object, Object>>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<ImmutableCollections_Map1<Object, Object>>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Properties>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<ConcurrentHashMap<Object, Object>>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<ImmutableCollections_AbstractImmutableMap<Object, Object>>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<TreeMap<Object, Object>>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<LinkedHashMap<Object, Object>>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Hashtable<Object, Object>>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<AbstractMap<Object, Object>>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<HashMap<Object, Object>>() { _d.get(Object::from_any(name.clone()))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(name.clone()))? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(name.clone()))? } else { Default::default() };
            let mut result = (_vdispatch1).downcast::<Enum<Object>>();
            if !_is_jnull(&result) {
                return Ok(Object::from_any(result.clone()));
            }
            if _is_jnull(&name) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t2 = StringBuilder::new()?.append_str(Clone::clone(&String::from("No enum constant ")))?;
            let _t3 = enumClass.getCanonicalName()?;
            let _t4 = _t2.append_str(Clone::clone(&_t3))?;
            let _t5 = _t4.append_str(Clone::clone(&String::from(".")))?;
            let _t6 = _t5.append_str(Clone::clone(&name))?;
            let _t7 = _t6.toString()?;
            return Err(JvmError::Custom("athrow".to_owned()));
        }

        #[java_method(name = "finalize", descriptor = "()V", access = "protected", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn finalize(&self) -> Result<()> {
            panic!("stub: java/lang/Enum.finalize:()V")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, in_: Object) -> Result<()> {
            panic!("stub: java/lang/Enum.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "readObjectNoData", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/ObjectStreamException")]
        pub fn readObjectNoData(&self) -> Result<()> {
            panic!("stub: java/lang/Enum.readObjectNoData:()V")
        }
    }
}
