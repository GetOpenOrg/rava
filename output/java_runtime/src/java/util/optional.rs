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
    #[binary_name       = "java/util/Optional"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = "<T:Ljava/lang/Object;>Ljava/lang/Object;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Optional.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Optional"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct Optional<T: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "value", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "final", is_static = false, generic_signature = "TT;"))]
        pub value: T,
    }

    impl<T> Optional<T> {
        #[cfg_attr(any(), java_field(name = "EMPTY", descriptor = "Ljava/util/Optional;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Optional<*>;"))]
        // static field: EMPTY:Ljava/util/Optional;
        pub fn EMPTY() -> Optional<Object> {
            panic!("stub: java/util/Optional.EMPTY:Ljava/util/Optional;")
        }

        #[java_method(name = "empty", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>()Ljava/util/Optional<TT;>;")]
        pub fn empty() -> Result<Optional<T>> {
            let mut t: Optional<Object> = Optional::<Object>::EMPTY();
            Ok(Default::default())
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Object;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TT;)V")]
        pub fn new(mut value: T) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_value(Clone::clone(&value));
            Ok(this)
        }

        #[java_method(name = "of", descriptor = "(Ljava/lang/Object;)Ljava/util/Optional;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;)Ljava/util/Optional<TT;>;")]
        pub fn of(mut value: T) -> Result<Optional<T>> {
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&value))?;
            Ok(Default::default())
        }

        #[java_method(name = "ofNullable", descriptor = "(Ljava/lang/Object;)Ljava/util/Optional;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;)Ljava/util/Optional<TT;>;")]
        pub fn ofNullable(mut value: T) -> Result<Optional<T>> {
            Ok(Default::default())
        }

        #[java_method(name = "get", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TT;")]
        pub fn get(&self) -> Result<T> {
            let this = self;
            if _is_jnull(&this.__get_value()) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(this.__get_value())
        }

        #[java_method(name = "isPresent", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isPresent(&self) -> Result<bool> {
            let this = self;
            Ok(!_is_jnull(&this.__get_value()))
        }

        #[java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmpty(&self) -> Result<bool> {
            let this = self;
            Ok(_is_jnull(&this.__get_value()))
        }

        #[java_method(name = "ifPresent", descriptor = "(Ljava/util/function/Consumer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TT;>;)V")]
        pub fn ifPresent(&self, action: Object) -> Result<()> {
            panic!("stub: java/util/Optional.ifPresent:(Ljava/util/function/Consumer;)V")
        }

        #[java_method(name = "ifPresentOrElse", descriptor = "(Ljava/util/function/Consumer;Ljava/lang/Runnable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Consumer<-TT;>;Ljava/lang/Runnable;)V")]
        pub fn ifPresentOrElse(&self, action: Object, emptyAction: Object) -> Result<()> {
            panic!("stub: java/util/Optional.ifPresentOrElse:(Ljava/util/function/Consumer;Ljava/lang/Runnable;)V")
        }

        #[java_method(name = "filter", descriptor = "(Ljava/util/function/Predicate;)Ljava/util/Optional;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Predicate<-TT;>;)Ljava/util/Optional<TT;>;")]
        pub fn filter(&self, predicate: Object) -> Result<Optional<Object>> {
            panic!("stub: java/util/Optional.filter:(Ljava/util/function/Predicate;)Ljava/util/Optional;")
        }

        #[java_method(name = "map", descriptor = "(Ljava/util/function/Function;)Ljava/util/Optional;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+TU;>;)Ljava/util/Optional<TU;>;")]
        pub fn map(&self, mut mapper: Object) -> Result<Optional<Object>> {
            let this = self;
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&mapper))?;
            let _t1 = this.isEmpty()?;
            if _t1 {
                let _t2: Optional<Object> = Optional::<Object>::empty()?;
                return Ok(_t2);
            }
            let _vdispatch2: Object = if let Some(__f) = mapper.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&this.__get_value()))? } else { Default::default() };
            let _t3: Optional<Object> = Optional::<Object>::ofNullable(Clone::clone(&_vdispatch2))?;
            Ok(_t3)
        }

        #[java_method(name = "flatMap", descriptor = "(Ljava/util/function/Function;)Ljava/util/Optional;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<U:Ljava/lang/Object;>(Ljava/util/function/Function<-TT;+Ljava/util/Optional<+TU;>;>;)Ljava/util/Optional<TU;>;")]
        pub fn flatMap(&self, mapper: Object) -> Result<Optional<Object>> {
            panic!("stub: java/util/Optional.flatMap:(Ljava/util/function/Function;)Ljava/util/Optional;")
        }

        #[java_method(name = "or", descriptor = "(Ljava/util/function/Supplier;)Ljava/util/Optional;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Supplier<+Ljava/util/Optional<+TT;>;>;)Ljava/util/Optional<TT;>;")]
        pub fn or(&self, supplier: Object) -> Result<Optional<Object>> {
            panic!("stub: java/util/Optional.or:(Ljava/util/function/Supplier;)Ljava/util/Optional;")
        }

        #[java_method(name = "stream", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<TT;>;")]
        pub fn stream(&self) -> Result<Object> {
            panic!("stub: java/util/Optional.stream:()Ljava/util/stream/Stream;")
        }

        #[java_method(name = "orElse", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TT;)TT;")]
        pub fn orElse(&self, mut other: T) -> Result<T> {
            let this = self;
            Ok((if !_is_jnull(&this.__get_value()) { this.__get_value() } else { other }))
        }

        #[java_method(name = "orElseGet", descriptor = "(Ljava/util/function/Supplier;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/Supplier<+TT;>;)TT;")]
        pub fn orElseGet(&self, supplier: Object) -> Result<Object> {
            panic!("stub: java/util/Optional.orElseGet:(Ljava/util/function/Supplier;)Ljava/lang/Object;")
        }

        #[java_method(name = "orElseThrow", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()TT;")]
        pub fn orElseThrow(&self) -> Result<Object> {
            panic!("stub: java/util/Optional.orElseThrow:()Ljava/lang/Object;")
        }

        #[java_method(name = "orElseThrow", descriptor = "(Ljava/util/function/Supplier;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/Throwable", generic_signature = "<X:Ljava/lang/Throwable;>(Ljava/util/function/Supplier<+TX;>;)TT;^TX;")]
        pub fn orElseThrow_suppli(&self, exceptionSupplier: Object) -> Result<Object> {
            panic!("stub: java/util/Optional.orElseThrow:(Ljava/util/function/Supplier;)Ljava/lang/Object;")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/util/Optional.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }
    }
}
