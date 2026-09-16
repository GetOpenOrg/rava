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
use crate::jdk::internal::misc::VM;
use crate::jdk::internal::util::random::RandomSupport;

impl From<ThreadLocalRandom> for Random {
    fn from(v: ThreadLocalRandom) -> Random { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/concurrent/ThreadLocalRandom"]
    #[super_class       = "java/util/Random"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ThreadLocalRandom.java"]
    #[inner_classes     = "java/io/ObjectOutputStream$PutField:java/io/ObjectOutputStream:PutField:1033;java/util/concurrent/ThreadLocalRandom$ThreadLocalRandomProxy:java/util/concurrent/ThreadLocalRandom:ThreadLocalRandomProxy:26;jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator:jdk/internal/util/random/RandomSupport:AbstractSpliteratorGenerator:1033;java/util/concurrent/ThreadLocalRandom$Access:java/util/concurrent/ThreadLocalRandom:Access:10;jdk/internal/util/random/RandomSupport$RandomGeneratorProperties:jdk/internal/util/random/RandomSupport:RandomGeneratorProperties:9737;java/util/concurrent/ThreadLocalRandom$Access$1:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Random"]
    #[superclass_fields(seed: AtomicLong, nextNextGaussian: f64, haveNextNextGaussian: bool)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/util/Random;java/util/concurrent/ThreadLocalRandom;java/util/random/RandomGenerator"]

    pub struct ThreadLocalRandom {
        #[cfg_attr(any(), java_field(name = "initialized", descriptor = "Z", is_static = false))]
        pub initialized: bool,
    }

    impl ThreadLocalRandom {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-5851777807851030925"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -5851777807851030925i64
        }

        #[cfg_attr(any(), java_field(name = "serialPersistentFields", descriptor = "[Ljava/io/ObjectStreamField;", access = "private", modifiers = "static final", is_static = true))]
        // static field: serialPersistentFields:[Ljava/io/ObjectStreamField;
        pub fn serialPersistentFields() -> Rc<RefCell<Vec<Object>>> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.serialPersistentFields:[Ljava/io/ObjectStreamField;")
        }

        #[cfg_attr(any(), java_field(name = "GOLDEN_GAMMA", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-7046029254386353131"))]
        // static field: GOLDEN_GAMMA:J
        pub fn GOLDEN_GAMMA() -> i64 {
            -7046029254386353131i64
        }

        #[cfg_attr(any(), java_field(name = "PROBE_INCREMENT", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "-1640531527"))]
        // static field: PROBE_INCREMENT:I
        pub fn PROBE_INCREMENT() -> i32 {
            -1640531527
        }

        #[cfg_attr(any(), java_field(name = "SEEDER_INCREMENT", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-4942790177534073029"))]
        // static field: SEEDER_INCREMENT:J
        pub fn SEEDER_INCREMENT() -> i64 {
            -4942790177534073029i64
        }

        #[cfg_attr(any(), java_field(name = "BAD_BOUND", descriptor = "Ljava/lang/String;", access = "package", modifiers = "static final", is_static = true, constant_value = "bound must be positive"))]
        // static field: BAD_BOUND:Ljava/lang/String;
        pub fn BAD_BOUND() -> String {
            String::from("bound must be positive")
        }

        #[cfg_attr(any(), java_field(name = "BAD_RANGE", descriptor = "Ljava/lang/String;", access = "package", modifiers = "static final", is_static = true, constant_value = "bound must be greater than origin"))]
        // static field: BAD_RANGE:Ljava/lang/String;
        pub fn BAD_RANGE() -> String {
            String::from("bound must be greater than origin")
        }

        #[cfg_attr(any(), java_field(name = "BAD_SIZE", descriptor = "Ljava/lang/String;", access = "package", modifiers = "static final", is_static = true, constant_value = "size must be non-negative"))]
        // static field: BAD_SIZE:Ljava/lang/String;
        pub fn BAD_SIZE() -> String {
            String::from("size must be non-negative")
        }

        #[cfg_attr(any(), java_field(name = "U", descriptor = "Ljdk/internal/misc/Unsafe;", access = "private", modifiers = "static final", is_static = true))]
        // static field: U:Ljdk/internal/misc/Unsafe;
        pub fn U() -> Unsafe {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.U:Ljdk/internal/misc/Unsafe;")
        }

        #[cfg_attr(any(), java_field(name = "SEED", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: SEED:J
        pub fn SEED() -> i64 {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.SEED:J")
        }

        #[cfg_attr(any(), java_field(name = "PROBE", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: PROBE:J
        pub fn PROBE() -> i64 {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.PROBE:J")
        }

        #[cfg_attr(any(), java_field(name = "SECONDARY", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: SECONDARY:J
        pub fn SECONDARY() -> i64 {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.SECONDARY:J")
        }

        #[cfg_attr(any(), java_field(name = "THREADLOCALS", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: THREADLOCALS:J
        pub fn THREADLOCALS() -> i64 {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.THREADLOCALS:J")
        }

        #[cfg_attr(any(), java_field(name = "INHERITABLETHREADLOCALS", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: INHERITABLETHREADLOCALS:J
        pub fn INHERITABLETHREADLOCALS() -> i64 {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.INHERITABLETHREADLOCALS:J")
        }

        #[cfg_attr(any(), java_field(name = "INHERITEDACCESSCONTROLCONTEXT", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: INHERITEDACCESSCONTROLCONTEXT:J
        pub fn INHERITEDACCESSCONTROLCONTEXT() -> i64 {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.INHERITEDACCESSCONTROLCONTEXT:J")
        }

        #[cfg_attr(any(), java_field(name = "probeGenerator", descriptor = "Ljava/util/concurrent/atomic/AtomicInteger;", access = "private", modifiers = "static final", is_static = true))]
        // static field: probeGenerator:Ljava/util/concurrent/atomic/AtomicInteger;
        pub fn probeGenerator() -> AtomicInteger {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.probeGenerator:Ljava/util/concurrent/atomic/AtomicInteger;")
        }

        #[cfg_attr(any(), java_field(name = "instance", descriptor = "Ljava/util/concurrent/ThreadLocalRandom;", access = "private", modifiers = "static final", is_static = true))]
        // static field: instance:Ljava/util/concurrent/ThreadLocalRandom;
        pub fn instance() -> ThreadLocalRandom {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.instance:Ljava/util/concurrent/ThreadLocalRandom;")
        }

        #[cfg_attr(any(), java_field(name = "seeder", descriptor = "Ljava/util/concurrent/atomic/AtomicLong;", access = "private", modifiers = "static final", is_static = true))]
        // static field: seeder:Ljava/util/concurrent/atomic/AtomicLong;
        pub fn seeder() -> AtomicLong {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.seeder:Ljava/util/concurrent/atomic/AtomicLong;")
        }

        #[java_method(name = "mix32", descriptor = "(J)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mix32(z: i64) -> Result<i32> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.mix32:(J)I")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.<init>:()V")
        }

        #[java_method(name = "localInit", descriptor = "()V", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn localInit() -> Result<()> {
            let _t0 = ThreadLocalRandom::probeGenerator().addAndGet(-1640531527i32)?;
            let mut p: i32 = _t0;
            let mut probe = (if (p==0) { 1i32 } else { p });
            let _t1 = ThreadLocalRandom::seeder().getAndAdd(-4942790177534073029i64)?;
            let _t2: i64 = RandomSupport::mixMurmur64(_t1)?;
            let mut seed: i64 = _t2;
            let _t3: Thread = Thread::currentThread()?;
            let mut t: Thread = _t3;
            ThreadLocalRandom::U().putLong_obj_l_l(Object::from_any(t.clone()), ThreadLocalRandom::SEED(), seed)?;
            ThreadLocalRandom::U().putInt_obj_l_i(Object::from_any(t.clone()), ThreadLocalRandom::PROBE(), probe)?;
            Ok(())
        }

        #[java_method(name = "current", descriptor = "()Ljava/util/concurrent/ThreadLocalRandom;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn current() -> Result<ThreadLocalRandom> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.current:()Ljava/util/concurrent/ThreadLocalRandom;")
        }

        #[java_method(name = "setSeed", descriptor = "(J)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setSeed(&self, seed: i64) -> Result<()> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.setSeed:(J)V")
        }

        #[java_method(name = "nextSeed", descriptor = "()J", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextSeed(&self) -> Result<i64> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.nextSeed:()J")
        }

        #[java_method(name = "next", descriptor = "(I)I", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn next(&self, bits: i32) -> Result<i32> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.next:(I)I")
        }

        #[java_method(name = "getProbe", descriptor = "()I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getProbe() -> Result<i32> {
            let _t0: Thread = Thread::currentThread()?;
            let _t1 = ThreadLocalRandom::U().getInt_obj_l(Object::from_any(_t0.clone()), ThreadLocalRandom::PROBE())?;
            Ok(_t1)
        }

        #[java_method(name = "advanceProbe", descriptor = "(I)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn advanceProbe(mut probe: i32) -> Result<i32> {
            probe = (probe^(probe<<(13i32&0x1f)));
            probe = (probe^((probe as u32>>(17i32&0x1f)) as i32));
            probe = (probe^(probe<<(5i32&0x1f)));
            let _t0: Thread = Thread::currentThread()?;
            ThreadLocalRandom::U().putInt_obj_l_i(Object::from_any(_t0.clone()), ThreadLocalRandom::PROBE(), probe)?;
            Ok(probe)
        }

        #[java_method(name = "nextSecondarySeed", descriptor = "()I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextSecondarySeed() -> Result<i32> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.nextSecondarySeed:()I")
        }

        #[java_method(name = "eraseThreadLocals", descriptor = "(Ljava/lang/Thread;)V", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn eraseThreadLocals(thread: Thread) -> Result<()> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.eraseThreadLocals:(Ljava/lang/Thread;)V")
        }

        #[java_method(name = "setInheritedAccessControlContext", descriptor = "(Ljava/lang/Thread;Ljava/security/AccessControlContext;)V", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setInheritedAccessControlContext(thread: Thread, acc: Object) -> Result<()> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.setInheritedAccessControlContext:(Ljava/lang/Thread;Ljava/security/AccessControlContext;)V")
        }

        #[java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.writeObject:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "readResolve", descriptor = "()Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn readResolve(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.readResolve:()Ljava/lang/Object;")
        }

        #[java_method(name = "nextBoolean", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextBoolean(&self) -> Result<bool> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.nextBoolean:()Z")
        }

        #[java_method(name = "nextInt", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextInt(&self) -> Result<i32> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.nextInt:()I")
        }

        #[java_method(name = "nextInt", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextInt_i(&self, bound: i32) -> Result<i32> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.nextInt:(I)I")
        }

        #[java_method(name = "nextInt", descriptor = "(II)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextInt_i_i(&self, origin: i32, bound: i32) -> Result<i32> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.nextInt:(II)I")
        }

        #[java_method(name = "nextLong", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextLong(&self) -> Result<i64> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.nextLong:()J")
        }

        #[java_method(name = "nextLong", descriptor = "(J)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextLong_l(&self, bound: i64) -> Result<i64> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.nextLong:(J)J")
        }

        #[java_method(name = "nextLong", descriptor = "(JJ)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextLong_l_l(&self, origin: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.nextLong:(JJ)J")
        }

        #[java_method(name = "nextFloat", descriptor = "()F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextFloat(&self) -> Result<f32> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.nextFloat:()F")
        }

        #[java_method(name = "nextFloat", descriptor = "(F)F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextFloat_f(&self, bound: f32) -> Result<f32> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.nextFloat:(F)F")
        }

        #[java_method(name = "nextFloat", descriptor = "(FF)F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextFloat_f_f(&self, origin: f32, bound: f32) -> Result<f32> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.nextFloat:(FF)F")
        }

        #[java_method(name = "nextDouble", descriptor = "()D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextDouble(&self) -> Result<f64> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.nextDouble:()D")
        }

        #[java_method(name = "nextDouble", descriptor = "(D)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextDouble_d(&self, bound: f64) -> Result<f64> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.nextDouble:(D)D")
        }

        #[java_method(name = "nextDouble", descriptor = "(DD)D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextDouble_d_d(&self, origin: f64, arg1: f64) -> Result<f64> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.nextDouble:(DD)D")
        }

        #[java_method(name = "ints", descriptor = "(J)Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ints_l(&self, streamSize: i64) -> Result<Object> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.ints:(J)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "ints", descriptor = "()Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ints(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.ints:()Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "ints", descriptor = "(JII)Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ints_l_i_i(&self, streamSize: i64, arg1: i32, randomNumberOrigin: i32) -> Result<Object> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.ints:(JII)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "ints", descriptor = "(II)Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ints_i_i(&self, randomNumberOrigin: i32, randomNumberBound: i32) -> Result<Object> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.ints:(II)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "longs", descriptor = "(J)Ljava/util/stream/LongStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longs_l(&self, streamSize: i64) -> Result<Object> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.longs:(J)Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "longs", descriptor = "()Ljava/util/stream/LongStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longs(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.longs:()Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "longs", descriptor = "(JJJ)Ljava/util/stream/LongStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longs_l_l_l(&self, streamSize: i64, arg1: i64, randomNumberOrigin: i64) -> Result<Object> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.longs:(JJJ)Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "longs", descriptor = "(JJ)Ljava/util/stream/LongStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longs_l_l(&self, randomNumberOrigin: i64, arg1: i64) -> Result<Object> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.longs:(JJ)Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "doubles", descriptor = "(J)Ljava/util/stream/DoubleStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubles_l(&self, streamSize: i64) -> Result<Object> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.doubles:(J)Ljava/util/stream/DoubleStream;")
        }

        #[java_method(name = "doubles", descriptor = "()Ljava/util/stream/DoubleStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubles(&self) -> Result<Object> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.doubles:()Ljava/util/stream/DoubleStream;")
        }

        #[java_method(name = "doubles", descriptor = "(JDD)Ljava/util/stream/DoubleStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubles_l_d_d(&self, streamSize: i64, arg1: f64, randomNumberOrigin: f64) -> Result<Object> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.doubles:(JDD)Ljava/util/stream/DoubleStream;")
        }

        #[java_method(name = "doubles", descriptor = "(DD)Ljava/util/stream/DoubleStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubles_d_d(&self, randomNumberOrigin: f64, arg1: f64) -> Result<Object> {
            panic!("stub: java/util/concurrent/ThreadLocalRandom.doubles:(DD)Ljava/util/stream/DoubleStream;")
        }
    }
}
