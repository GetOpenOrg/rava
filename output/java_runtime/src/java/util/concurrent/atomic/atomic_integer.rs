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
use crate::jdk::internal::misc::Unsafe;

impl From<AtomicInteger> for Number {
    fn from(v: AtomicInteger) -> Number { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/concurrent/atomic/AtomicInteger"]
    #[super_class       = "java/lang/Number"]
    #[interfaces        = "java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AtomicInteger.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Number"]
    #[all_supertypes    = "java/io/Serializable;java/lang/Number;java/lang/Object;java/util/concurrent/atomic/AtomicInteger"]
    #[has_to_string_method = true]

    pub struct AtomicInteger {
        #[cfg_attr(any(), java_field(name = "value", descriptor = "I", access = "private", modifiers = "volatile", is_static = false))]
        pub value: i32,
    }

    impl AtomicInteger {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "6214790243416807050"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            6214790243416807050i64
        }

        #[cfg_attr(any(), java_field(name = "U", descriptor = "Ljdk/internal/misc/Unsafe;", access = "private", modifiers = "static final", is_static = true))]
        // static field: U:Ljdk/internal/misc/Unsafe;
        pub fn U() -> Unsafe {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.U:Ljdk/internal/misc/Unsafe;")
        }

        #[cfg_attr(any(), java_field(name = "VALUE", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: VALUE:J
        pub fn VALUE() -> i64 {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.VALUE:J")
        }

        #[java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i(initialValue: i32) -> Result<Self> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.<init>:(I)V")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.<init>:()V")
        }

        #[java_method(name = "get", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get(&self) -> Result<i32> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.get:()I")
        }

        #[java_method(name = "set", descriptor = "(I)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn set(&self, newValue: i32) -> Result<()> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.set:(I)V")
        }

        #[java_method(name = "lazySet", descriptor = "(I)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lazySet(&self, newValue: i32) -> Result<()> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.lazySet:(I)V")
        }

        #[java_method(name = "getAndSet", descriptor = "(I)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSet(&self, newValue: i32) -> Result<i32> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.getAndSet:(I)I")
        }

        #[java_method(name = "compareAndSet", descriptor = "(II)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSet(&self, expectedValue: i32, newValue: i32) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.compareAndSet:(II)Z")
        }

        #[java_method(name = "weakCompareAndSet", descriptor = "(II)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn weakCompareAndSet(&self, expectedValue: i32, newValue: i32) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.weakCompareAndSet:(II)Z")
        }

        #[java_method(name = "weakCompareAndSetPlain", descriptor = "(II)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetPlain(&self, expectedValue: i32, newValue: i32) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.weakCompareAndSetPlain:(II)Z")
        }

        #[java_method(name = "getAndIncrement", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndIncrement(&self) -> Result<i32> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.getAndIncrement:()I")
        }

        #[java_method(name = "getAndDecrement", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndDecrement(&self) -> Result<i32> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.getAndDecrement:()I")
        }

        #[java_method(name = "getAndAdd", descriptor = "(I)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAdd(&self, delta: i32) -> Result<i32> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.getAndAdd:(I)I")
        }

        #[java_method(name = "incrementAndGet", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn incrementAndGet(&self) -> Result<i32> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.incrementAndGet:()I")
        }

        #[java_method(name = "decrementAndGet", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn decrementAndGet(&self) -> Result<i32> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.decrementAndGet:()I")
        }

        #[java_method(name = "addAndGet", descriptor = "(I)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addAndGet(&self, mut delta: i32) -> Result<i32> {
            let this = self;
            let _t0 = AtomicInteger::U().getAndAddInt(Object::from_any(Clone::clone(self)), AtomicInteger::VALUE(), delta)?;
            Ok((_t0).wrapping_add(delta))
        }

        #[java_method(name = "getAndUpdate", descriptor = "(Ljava/util/function/IntUnaryOperator;)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndUpdate(&self, updateFunction: Object) -> Result<i32> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.getAndUpdate:(Ljava/util/function/IntUnaryOperator;)I")
        }

        #[java_method(name = "updateAndGet", descriptor = "(Ljava/util/function/IntUnaryOperator;)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn updateAndGet(&self, updateFunction: Object) -> Result<i32> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.updateAndGet:(Ljava/util/function/IntUnaryOperator;)I")
        }

        #[java_method(name = "getAndAccumulate", descriptor = "(ILjava/util/function/IntBinaryOperator;)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAccumulate(&self, x: i32, accumulatorFunction: Object) -> Result<i32> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.getAndAccumulate:(ILjava/util/function/IntBinaryOperator;)I")
        }

        #[java_method(name = "accumulateAndGet", descriptor = "(ILjava/util/function/IntBinaryOperator;)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn accumulateAndGet(&self, x: i32, accumulatorFunction: Object) -> Result<i32> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.accumulateAndGet:(ILjava/util/function/IntBinaryOperator;)I")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "intValue", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn intValue(&self) -> Result<i32> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.intValue:()I")
        }

        #[java_method(name = "longValue", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longValue(&self) -> Result<i64> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.longValue:()J")
        }

        #[java_method(name = "floatValue", descriptor = "()F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floatValue(&self) -> Result<f32> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.floatValue:()F")
        }

        #[java_method(name = "doubleValue", descriptor = "()D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubleValue(&self) -> Result<f64> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.doubleValue:()D")
        }

        #[java_method(name = "getPlain", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPlain(&self) -> Result<i32> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.getPlain:()I")
        }

        #[java_method(name = "setPlain", descriptor = "(I)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setPlain(&self, newValue: i32) -> Result<()> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.setPlain:(I)V")
        }

        #[java_method(name = "getOpaque", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOpaque(&self) -> Result<i32> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.getOpaque:()I")
        }

        #[java_method(name = "setOpaque", descriptor = "(I)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setOpaque(&self, newValue: i32) -> Result<()> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.setOpaque:(I)V")
        }

        #[java_method(name = "getAcquire", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAcquire(&self) -> Result<i32> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.getAcquire:()I")
        }

        #[java_method(name = "setRelease", descriptor = "(I)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setRelease(&self, newValue: i32) -> Result<()> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.setRelease:(I)V")
        }

        #[java_method(name = "compareAndExchange", descriptor = "(II)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchange(&self, expectedValue: i32, newValue: i32) -> Result<i32> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.compareAndExchange:(II)I")
        }

        #[java_method(name = "compareAndExchangeAcquire", descriptor = "(II)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeAcquire(&self, expectedValue: i32, newValue: i32) -> Result<i32> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.compareAndExchangeAcquire:(II)I")
        }

        #[java_method(name = "compareAndExchangeRelease", descriptor = "(II)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeRelease(&self, expectedValue: i32, newValue: i32) -> Result<i32> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.compareAndExchangeRelease:(II)I")
        }

        #[java_method(name = "weakCompareAndSetVolatile", descriptor = "(II)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetVolatile(&self, expectedValue: i32, newValue: i32) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.weakCompareAndSetVolatile:(II)Z")
        }

        #[java_method(name = "weakCompareAndSetAcquire", descriptor = "(II)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetAcquire(&self, expectedValue: i32, newValue: i32) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.weakCompareAndSetAcquire:(II)Z")
        }

        #[java_method(name = "weakCompareAndSetRelease", descriptor = "(II)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetRelease(&self, expectedValue: i32, newValue: i32) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicInteger.weakCompareAndSetRelease:(II)Z")
        }
    }
}
