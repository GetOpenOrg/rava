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
use crate::jdk::internal::r#ref::CleanerFactory;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/zip/Inflater$InflaterZStreamRef"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/lang/Runnable"]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Inflater.java"]
    #[inner_classes     = "java/lang/ref/Cleaner$Cleanable:java/lang/ref/Cleaner:Cleanable:1545;java/util/zip/Inflater$InflaterZStreamRef:java/util/zip/Inflater:InflaterZStreamRef:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/lang/Runnable;java/util/zip/Inflater$InflaterZStreamRef"]

    pub struct Inflater_InflaterZStreamRef {
        #[cfg_attr(any(), java_field(name = "address", descriptor = "J", access = "private", modifiers = "", is_static = false))]
        pub address: i64,
        #[cfg_attr(any(), java_field(name = "cleanable", descriptor = "Ljava/lang/ref/Cleaner$Cleanable;", access = "private", modifiers = "final", is_static = false))]
        pub cleanable: Object,
    }

    impl Inflater_InflaterZStreamRef {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/zip/Inflater;J)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut owner: Inflater, mut addr: i64) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            let mut _merged2: Object;
            if !_is_jnull(&owner) {
                let _t0: Cleaner = CleanerFactory::cleaner()?;
                let _t1 = _t0.register(Object::from_any(owner.clone()), Object::from_any(Clone::clone(self)))?;
                _merged2 = _t1;
            } else {
                _merged2 = Object::default();
            }
            this.__set_cleanable(Clone::clone(&_merged2));
            this.__set_address(addr);
            Ok(this)
        }

        #[java_method(name = "address", descriptor = "()J", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn address(&self) -> Result<i64> {
            panic!("stub: java/util/zip/Inflater$InflaterZStreamRef.address:()J")
        }

        #[java_method(name = "clean", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clean(&self) -> Result<()> {
            panic!("stub: java/util/zip/Inflater$InflaterZStreamRef.clean:()V")
        }

        #[java_method(name = "run", descriptor = "()V", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn run(&self) -> Result<()> {
            panic!("stub: java/util/zip/Inflater$InflaterZStreamRef.run:()V")
        }
    }
}
