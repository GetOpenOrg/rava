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

impl From<AtomicLong> for Number {
    fn from(v: AtomicLong) -> Number { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/concurrent/atomic/AtomicLong"]
    #[super_class       = "java/lang/Number"]
    #[interfaces        = "java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AtomicLong.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Number"]
    #[all_supertypes    = "java/io/Serializable;java/lang/Number;java/lang/Object;java/util/concurrent/atomic/AtomicLong"]
    #[has_to_string_method = true]

    pub struct AtomicLong {
        #[cfg_attr(any(), java_field(name = "value", descriptor = "J", access = "private", modifiers = "volatile", is_static = false))]
        pub value: i64,
    }

    impl AtomicLong {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "1927816293512124184"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            1927816293512124184i64
        }

        #[cfg_attr(any(), java_field(name = "VM_SUPPORTS_LONG_CAS", descriptor = "Z", access = "package", modifiers = "static final", is_static = true))]
        // static field: VM_SUPPORTS_LONG_CAS:Z
        pub fn VM_SUPPORTS_LONG_CAS() -> bool {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.VM_SUPPORTS_LONG_CAS:Z")
        }

        #[cfg_attr(any(), java_field(name = "U", descriptor = "Ljdk/internal/misc/Unsafe;", access = "private", modifiers = "static final", is_static = true))]
        // static field: U:Ljdk/internal/misc/Unsafe;
        pub fn U() -> Unsafe {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.U:Ljdk/internal/misc/Unsafe;")
        }

        #[cfg_attr(any(), java_field(name = "VALUE", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: VALUE:J
        pub fn VALUE() -> i64 {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.VALUE:J")
        }

        #[native]
        #[java_native(name = "VMSupportsCS8", descriptor = "()Z", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn VMSupportsCS8() -> Result<bool> {
            panic!("native: java/util/concurrent/atomic/AtomicLong.VMSupportsCS8:()Z")
        }

        #[java_method(name = "<init>", descriptor = "(J)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_l(initialValue: i64) -> Result<Self> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.<init>:(J)V")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.<init>:()V")
        }

        #[java_method(name = "get", descriptor = "()J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get(&self) -> Result<i64> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.get:()J")
        }

        #[java_method(name = "set", descriptor = "(J)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn set(&self, newValue: i64) -> Result<()> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.set:(J)V")
        }

        #[java_method(name = "lazySet", descriptor = "(J)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lazySet(&self, newValue: i64) -> Result<()> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.lazySet:(J)V")
        }

        #[java_method(name = "getAndSet", descriptor = "(J)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndSet(&self, newValue: i64) -> Result<i64> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.getAndSet:(J)J")
        }

        #[java_method(name = "compareAndSet", descriptor = "(JJ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndSet(&self, expectedValue: i64, arg1: i64) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.compareAndSet:(JJ)Z")
        }

        #[java_method(name = "weakCompareAndSet", descriptor = "(JJ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn weakCompareAndSet(&self, expectedValue: i64, arg1: i64) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.weakCompareAndSet:(JJ)Z")
        }

        #[java_method(name = "weakCompareAndSetPlain", descriptor = "(JJ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetPlain(&self, expectedValue: i64, arg1: i64) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.weakCompareAndSetPlain:(JJ)Z")
        }

        #[java_method(name = "getAndIncrement", descriptor = "()J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndIncrement(&self) -> Result<i64> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.getAndIncrement:()J")
        }

        #[java_method(name = "getAndDecrement", descriptor = "()J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndDecrement(&self) -> Result<i64> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.getAndDecrement:()J")
        }

        #[java_method(name = "getAndAdd", descriptor = "(J)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAdd(&self, mut delta: i64) -> Result<i64> {
            let this = self;
            let _t0 = AtomicLong::U().getAndAddLong(Object::from_any(Clone::clone(self)), AtomicLong::VALUE(), delta)?;
            Ok(_t0)
        }

        #[java_method(name = "incrementAndGet", descriptor = "()J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn incrementAndGet(&self) -> Result<i64> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.incrementAndGet:()J")
        }

        #[java_method(name = "decrementAndGet", descriptor = "()J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn decrementAndGet(&self) -> Result<i64> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.decrementAndGet:()J")
        }

        #[java_method(name = "addAndGet", descriptor = "(J)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addAndGet(&self, delta: i64) -> Result<i64> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.addAndGet:(J)J")
        }

        #[java_method(name = "getAndUpdate", descriptor = "(Ljava/util/function/LongUnaryOperator;)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndUpdate(&self, updateFunction: Object) -> Result<i64> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.getAndUpdate:(Ljava/util/function/LongUnaryOperator;)J")
        }

        #[java_method(name = "updateAndGet", descriptor = "(Ljava/util/function/LongUnaryOperator;)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn updateAndGet(&self, updateFunction: Object) -> Result<i64> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.updateAndGet:(Ljava/util/function/LongUnaryOperator;)J")
        }

        #[java_method(name = "getAndAccumulate", descriptor = "(JLjava/util/function/LongBinaryOperator;)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAndAccumulate(&self, x: i64, arg1: Object) -> Result<i64> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.getAndAccumulate:(JLjava/util/function/LongBinaryOperator;)J")
        }

        #[java_method(name = "accumulateAndGet", descriptor = "(JLjava/util/function/LongBinaryOperator;)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn accumulateAndGet(&self, x: i64, arg1: Object) -> Result<i64> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.accumulateAndGet:(JLjava/util/function/LongBinaryOperator;)J")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "intValue", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn intValue(&self) -> Result<i32> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.intValue:()I")
        }

        #[java_method(name = "longValue", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longValue(&self) -> Result<i64> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.longValue:()J")
        }

        #[java_method(name = "floatValue", descriptor = "()F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floatValue(&self) -> Result<f32> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.floatValue:()F")
        }

        #[java_method(name = "doubleValue", descriptor = "()D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubleValue(&self) -> Result<f64> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.doubleValue:()D")
        }

        #[java_method(name = "getPlain", descriptor = "()J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPlain(&self) -> Result<i64> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.getPlain:()J")
        }

        #[java_method(name = "setPlain", descriptor = "(J)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setPlain(&self, newValue: i64) -> Result<()> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.setPlain:(J)V")
        }

        #[java_method(name = "getOpaque", descriptor = "()J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOpaque(&self) -> Result<i64> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.getOpaque:()J")
        }

        #[java_method(name = "setOpaque", descriptor = "(J)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setOpaque(&self, newValue: i64) -> Result<()> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.setOpaque:(J)V")
        }

        #[java_method(name = "getAcquire", descriptor = "()J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAcquire(&self) -> Result<i64> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.getAcquire:()J")
        }

        #[java_method(name = "setRelease", descriptor = "(J)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setRelease(&self, newValue: i64) -> Result<()> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.setRelease:(J)V")
        }

        #[java_method(name = "compareAndExchange", descriptor = "(JJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchange(&self, expectedValue: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.compareAndExchange:(JJ)J")
        }

        #[java_method(name = "compareAndExchangeAcquire", descriptor = "(JJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeAcquire(&self, expectedValue: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.compareAndExchangeAcquire:(JJ)J")
        }

        #[java_method(name = "compareAndExchangeRelease", descriptor = "(JJ)J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareAndExchangeRelease(&self, expectedValue: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.compareAndExchangeRelease:(JJ)J")
        }

        #[java_method(name = "weakCompareAndSetVolatile", descriptor = "(JJ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetVolatile(&self, expectedValue: i64, arg1: i64) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.weakCompareAndSetVolatile:(JJ)Z")
        }

        #[java_method(name = "weakCompareAndSetAcquire", descriptor = "(JJ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetAcquire(&self, expectedValue: i64, arg1: i64) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.weakCompareAndSetAcquire:(JJ)Z")
        }

        #[java_method(name = "weakCompareAndSetRelease", descriptor = "(JJ)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn weakCompareAndSetRelease(&self, expectedValue: i64, arg1: i64) -> Result<bool> {
            panic!("stub: java/util/concurrent/atomic/AtomicLong.weakCompareAndSetRelease:(JJ)Z")
        }
    }
}
