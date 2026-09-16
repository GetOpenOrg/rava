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

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/Random"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/random/RandomGenerator,java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Random.java"]
    #[inner_classes     = "java/util/Random$RandomWrapper:java/util/Random:RandomWrapper:26;java/io/ObjectInputStream$GetField:java/io/ObjectInputStream:GetField:1033;java/io/ObjectOutputStream$PutField:java/io/ObjectOutputStream:PutField:1033;jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator:jdk/internal/util/random/RandomSupport:AbstractSpliteratorGenerator:1033;jdk/internal/util/random/RandomSupport$RandomGeneratorProperties:jdk/internal/util/random/RandomSupport:RandomGeneratorProperties:9737"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/util/Random;java/util/random/RandomGenerator"]

    pub struct Random {
        #[cfg_attr(any(), java_field(name = "seed", descriptor = "Ljava/util/concurrent/atomic/AtomicLong;", access = "private", modifiers = "final", is_static = false))]
        pub seed: AtomicLong,
        #[cfg_attr(any(), java_field(name = "nextNextGaussian", descriptor = "D", access = "private", modifiers = "", is_static = false))]
        pub nextNextGaussian: f64,
        #[cfg_attr(any(), java_field(name = "haveNextNextGaussian", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub haveNextNextGaussian: bool,
    }

    impl Random {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "3905348978240129619"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            3905348978240129619i64
        }

        #[cfg_attr(any(), java_field(name = "multiplier", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "25214903917"))]
        // static field: multiplier:J
        pub fn multiplier() -> i64 {
            25214903917i64
        }

        #[cfg_attr(any(), java_field(name = "addend", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "11"))]
        // static field: addend:J
        pub fn addend() -> i64 {
            11i64
        }

        #[cfg_attr(any(), java_field(name = "mask", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "281474976710655"))]
        // static field: mask:J
        pub fn mask() -> i64 {
            281474976710655i64
        }

        #[cfg_attr(any(), java_field(name = "DOUBLE_UNIT", descriptor = "D", access = "private", modifiers = "static final", is_static = true, constant_value = "1.1102230246251565e-16"))]
        // static field: DOUBLE_UNIT:D
        pub fn DOUBLE_UNIT() -> f64 {
            1.1102230246251565e-16f64
        }

        #[cfg_attr(any(), java_field(name = "FLOAT_UNIT", descriptor = "F", access = "private", modifiers = "static final", is_static = true, constant_value = "5.960464477539063e-08"))]
        // static field: FLOAT_UNIT:F
        pub fn FLOAT_UNIT() -> f32 {
            5.960464477539063e-08f32
        }

        #[cfg_attr(any(), java_field(name = "seedUniquifier", descriptor = "Ljava/util/concurrent/atomic/AtomicLong;", access = "private", modifiers = "static final", is_static = true))]
        // static field: seedUniquifier:Ljava/util/concurrent/atomic/AtomicLong;
        pub fn seedUniquifier_field() -> AtomicLong {
            panic!("stub: java/util/Random.seedUniquifier:Ljava/util/concurrent/atomic/AtomicLong;")
        }

        #[cfg_attr(any(), java_field(name = "serialPersistentFields", descriptor = "[Ljava/io/ObjectStreamField;", access = "private", modifiers = "static final", is_static = true))]
        // static field: serialPersistentFields:[Ljava/io/ObjectStreamField;
        pub fn serialPersistentFields() -> Rc<RefCell<Vec<Object>>> {
            panic!("stub: java/util/Random.serialPersistentFields:[Ljava/io/ObjectStreamField;")
        }

        #[cfg_attr(any(), java_field(name = "unsafe", descriptor = "Ljdk/internal/misc/Unsafe;", access = "private", modifiers = "static final", is_static = true))]
        // static field: unsafe:Ljdk/internal/misc/Unsafe;
        pub fn unsafe_() -> Unsafe {
            panic!("stub: java/util/Random.unsafe:Ljdk/internal/misc/Unsafe;")
        }

        #[cfg_attr(any(), java_field(name = "seedOffset", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: seedOffset:J
        pub fn seedOffset() -> i64 {
            panic!("stub: java/util/Random.seedOffset:J")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/Random.<init>:()V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Void;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_void(unused: Object) -> Result<Self> {
            panic!("stub: java/util/Random.<init>:(Ljava/lang/Void;)V")
        }

        #[java_method(name = "seedUniquifier", descriptor = "()J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn seedUniquifier() -> Result<i64> {
            panic!("stub: java/util/Random.seedUniquifier:()J")
        }

        #[java_method(name = "<init>", descriptor = "(J)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_l(seed: i64) -> Result<Self> {
            panic!("stub: java/util/Random.<init>:(J)V")
        }

        #[java_method(name = "initialScramble", descriptor = "(J)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn initialScramble(seed: i64) -> Result<i64> {
            panic!("stub: java/util/Random.initialScramble:(J)J")
        }

        #[java_method(name = "from", descriptor = "(Ljava/util/random/RandomGenerator;)Ljava/util/Random;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn from(generator: Object) -> Result<Random> {
            panic!("stub: java/util/Random.from:(Ljava/util/random/RandomGenerator;)Ljava/util/Random;")
        }

        #[java_method(name = "setSeed", descriptor = "(J)V", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setSeed(&self, seed: i64) -> Result<()> {
            panic!("stub: java/util/Random.setSeed:(J)V")
        }

        #[java_method(name = "next", descriptor = "(I)I", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn next(&self, bits: i32) -> Result<i32> {
            panic!("stub: java/util/Random.next:(I)I")
        }

        #[java_method(name = "nextBytes", descriptor = "([B)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextBytes(&self, bytes: Rc<RefCell<Vec<i8>>>) -> Result<()> {
            panic!("stub: java/util/Random.nextBytes:([B)V")
        }

        #[java_method(name = "nextInt", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextInt(&self) -> Result<i32> {
            panic!("stub: java/util/Random.nextInt:()I")
        }

        #[java_method(name = "nextInt", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextInt_i(&self, bound: i32) -> Result<i32> {
            panic!("stub: java/util/Random.nextInt:(I)I")
        }

        #[java_method(name = "nextLong", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextLong(&self) -> Result<i64> {
            panic!("stub: java/util/Random.nextLong:()J")
        }

        #[java_method(name = "nextBoolean", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextBoolean(&self) -> Result<bool> {
            panic!("stub: java/util/Random.nextBoolean:()Z")
        }

        #[java_method(name = "nextFloat", descriptor = "()F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextFloat(&self) -> Result<f32> {
            panic!("stub: java/util/Random.nextFloat:()F")
        }

        #[java_method(name = "nextDouble", descriptor = "()D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextDouble(&self) -> Result<f64> {
            panic!("stub: java/util/Random.nextDouble:()D")
        }

        #[java_method(name = "nextGaussian", descriptor = "()D", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextGaussian(&self) -> Result<f64> {
            panic!("stub: java/util/Random.nextGaussian:()D")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/Random.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/Random.writeObject:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "resetSeed", descriptor = "(J)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn resetSeed(&self, seedVal: i64) -> Result<()> {
            panic!("stub: java/util/Random.resetSeed:(J)V")
        }

        #[java_method(name = "ints", descriptor = "(J)Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ints_l(&self, streamSize: i64) -> Result<Object> {
            panic!("stub: java/util/Random.ints:(J)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "ints", descriptor = "()Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ints(&self) -> Result<Object> {
            panic!("stub: java/util/Random.ints:()Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "ints", descriptor = "(JII)Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ints_l_i_i(&self, streamSize: i64, arg1: i32, randomNumberOrigin: i32) -> Result<Object> {
            panic!("stub: java/util/Random.ints:(JII)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "ints", descriptor = "(II)Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ints_i_i(&self, randomNumberOrigin: i32, randomNumberBound: i32) -> Result<Object> {
            panic!("stub: java/util/Random.ints:(II)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "longs", descriptor = "(J)Ljava/util/stream/LongStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longs_l(&self, streamSize: i64) -> Result<Object> {
            panic!("stub: java/util/Random.longs:(J)Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "longs", descriptor = "()Ljava/util/stream/LongStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longs(&self) -> Result<Object> {
            panic!("stub: java/util/Random.longs:()Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "longs", descriptor = "(JJJ)Ljava/util/stream/LongStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longs_l_l_l(&self, streamSize: i64, arg1: i64, randomNumberOrigin: i64) -> Result<Object> {
            panic!("stub: java/util/Random.longs:(JJJ)Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "longs", descriptor = "(JJ)Ljava/util/stream/LongStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longs_l_l(&self, randomNumberOrigin: i64, arg1: i64) -> Result<Object> {
            panic!("stub: java/util/Random.longs:(JJ)Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "doubles", descriptor = "(J)Ljava/util/stream/DoubleStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubles_l(&self, streamSize: i64) -> Result<Object> {
            panic!("stub: java/util/Random.doubles:(J)Ljava/util/stream/DoubleStream;")
        }

        #[java_method(name = "doubles", descriptor = "()Ljava/util/stream/DoubleStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubles(&self) -> Result<Object> {
            panic!("stub: java/util/Random.doubles:()Ljava/util/stream/DoubleStream;")
        }

        #[java_method(name = "doubles", descriptor = "(JDD)Ljava/util/stream/DoubleStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubles_l_d_d(&self, streamSize: i64, arg1: f64, randomNumberOrigin: f64) -> Result<Object> {
            panic!("stub: java/util/Random.doubles:(JDD)Ljava/util/stream/DoubleStream;")
        }

        #[java_method(name = "doubles", descriptor = "(DD)Ljava/util/stream/DoubleStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubles_d_d(&self, randomNumberOrigin: f64, arg1: f64) -> Result<Object> {
            panic!("stub: java/util/Random.doubles:(DD)Ljava/util/stream/DoubleStream;")
        }
    }
}
