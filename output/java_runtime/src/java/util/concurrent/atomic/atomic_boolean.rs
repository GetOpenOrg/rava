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
    #[binary_name       = "java/util/concurrent/atomic/AtomicBoolean"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AtomicBoolean.java"]
    #[inner_classes     = "java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/util/concurrent/atomic/AtomicBoolean"]
    #[has_to_string_method = true]

    pub struct AtomicBoolean {
        #[cfg_attr(any(), java_field(name = "value", descriptor = "I", access = "private", modifiers = "volatile", is_static = false))]
        pub value: i32,
    }

    impl AtomicBoolean {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "4654671469794556979"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            4654671469794556979i64
        }

        #[cfg_attr(any(), java_field(name = "VALUE", descriptor = "Ljava/lang/invoke/VarHandle;", access = "private", modifiers = "static final", is_static = true))]
        // static field: VALUE:Ljava/lang/invoke/VarHandle;
        pub fn VALUE() -> Object {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.VALUE:Ljava/lang/invoke/VarHandle;")
        }

        #[java_method(name = "<init>", descriptor = "(Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_z(initialValue: bool) -> Result<Self> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.<init>:(Z)V")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "get", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get(&self) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.get:()Z")
        }

        #[java_method(name = "compareAndSet", descriptor = "(ZZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSet(&self, expectedValue: bool, newValue: bool) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.compareAndSet:(ZZ)Z")
        }

        #[java_method(name = "weakCompareAndSet", descriptor = "(ZZ)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn weakCompareAndSet(&self, expectedValue: bool, newValue: bool) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.weakCompareAndSet:(ZZ)Z")
        }

        #[java_method(name = "weakCompareAndSetPlain", descriptor = "(ZZ)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetPlain(&self, expectedValue: bool, newValue: bool) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.weakCompareAndSetPlain:(ZZ)Z")
        }

        #[java_method(name = "set", descriptor = "(Z)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn set(&self, newValue: bool) -> Result<()> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.set:(Z)V")
        }

        #[java_method(name = "lazySet", descriptor = "(Z)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lazySet(&self, newValue: bool) -> Result<()> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.lazySet:(Z)V")
        }

        #[java_method(name = "getAndSet", descriptor = "(Z)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSet(&self, newValue: bool) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.getAndSet:(Z)Z")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "getPlain", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPlain(&self) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.getPlain:()Z")
        }

        #[java_method(name = "setPlain", descriptor = "(Z)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setPlain(&self, newValue: bool) -> Result<()> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.setPlain:(Z)V")
        }

        #[java_method(name = "getOpaque", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOpaque(&self) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.getOpaque:()Z")
        }

        #[java_method(name = "setOpaque", descriptor = "(Z)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setOpaque(&self, newValue: bool) -> Result<()> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.setOpaque:(Z)V")
        }

        #[java_method(name = "getAcquire", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAcquire(&self) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.getAcquire:()Z")
        }

        #[java_method(name = "setRelease", descriptor = "(Z)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setRelease(&self, newValue: bool) -> Result<()> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.setRelease:(Z)V")
        }

        #[java_method(name = "compareAndExchange", descriptor = "(ZZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchange(&self, expectedValue: bool, newValue: bool) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.compareAndExchange:(ZZ)Z")
        }

        #[java_method(name = "compareAndExchangeAcquire", descriptor = "(ZZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeAcquire(&self, expectedValue: bool, newValue: bool) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.compareAndExchangeAcquire:(ZZ)Z")
        }

        #[java_method(name = "compareAndExchangeRelease", descriptor = "(ZZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeRelease(&self, expectedValue: bool, newValue: bool) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.compareAndExchangeRelease:(ZZ)Z")
        }

        #[java_method(name = "weakCompareAndSetVolatile", descriptor = "(ZZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetVolatile(&self, expectedValue: bool, newValue: bool) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.weakCompareAndSetVolatile:(ZZ)Z")
        }

        #[java_method(name = "weakCompareAndSetAcquire", descriptor = "(ZZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetAcquire(&self, expectedValue: bool, newValue: bool) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.weakCompareAndSetAcquire:(ZZ)Z")
        }

        #[java_method(name = "weakCompareAndSetRelease", descriptor = "(ZZ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetRelease(&self, expectedValue: bool, newValue: bool) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicBoolean.weakCompareAndSetRelease:(ZZ)Z")
        }
    }
}
