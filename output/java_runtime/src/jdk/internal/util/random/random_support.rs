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
use crate::jdk::internal::util::random::*;
use crate::java::text::Normalizer;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "jdk/internal/util/random/RandomSupport"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "RandomSupport.java"]
    #[inner_classes     = "jdk/internal/util/random/RandomSupport$DoubleZigguratTables:jdk/internal/util/random/RandomSupport:DoubleZigguratTables:24;jdk/internal/util/random/RandomSupport$AbstractSplittableWithBrineGenerator:jdk/internal/util/random/RandomSupport:AbstractSplittableWithBrineGenerator:1033;jdk/internal/util/random/RandomSupport$AbstractSplittableGenerator:jdk/internal/util/random/RandomSupport:AbstractSplittableGenerator:1033;jdk/internal/util/random/RandomSupport$AbstractArbitrarilyJumpableGenerator:jdk/internal/util/random/RandomSupport:AbstractArbitrarilyJumpableGenerator:1033;jdk/internal/util/random/RandomSupport$AbstractSpliteratorGenerator:jdk/internal/util/random/RandomSupport:AbstractSpliteratorGenerator:1033;jdk/internal/util/random/RandomSupport$RandomDoublesSpliterator:jdk/internal/util/random/RandomSupport:RandomDoublesSpliterator:9;jdk/internal/util/random/RandomSupport$RandomLongsSpliterator:jdk/internal/util/random/RandomSupport:RandomLongsSpliterator:9;jdk/internal/util/random/RandomSupport$RandomIntsSpliterator:jdk/internal/util/random/RandomSupport:RandomIntsSpliterator:9;jdk/internal/util/random/RandomSupport$RandomSpliterator:jdk/internal/util/random/RandomSupport:RandomSpliterator:1033;jdk/internal/util/random/RandomSupport$RandomGeneratorProperties:jdk/internal/util/random/RandomSupport:RandomGeneratorProperties:9737;jdk/internal/util/random/RandomSupport$AbstractSplittableWithBrineGenerator$RandomSplitsSpliteratorWithSalt:jdk/internal/util/random/RandomSupport$AbstractSplittableWithBrineGenerator:RandomSplitsSpliteratorWithSalt:8;jdk/internal/util/random/RandomSupport$AbstractSplittableGenerator$RandomSplitsSpliterator:jdk/internal/util/random/RandomSupport$AbstractSplittableGenerator:RandomSplitsSpliterator:8;jdk/internal/util/random/RandomSupport$AbstractSplittableGenerator$RandomDoublesSpliterator:jdk/internal/util/random/RandomSupport$AbstractSplittableGenerator:RandomDoublesSpliterator:8;jdk/internal/util/random/RandomSupport$AbstractSplittableGenerator$RandomLongsSpliterator:jdk/internal/util/random/RandomSupport$AbstractSplittableGenerator:RandomLongsSpliterator:8;jdk/internal/util/random/RandomSupport$AbstractSplittableGenerator$RandomIntsSpliterator:jdk/internal/util/random/RandomSupport$AbstractSplittableGenerator:RandomIntsSpliterator:8;jdk/internal/util/random/RandomSupport$AbstractArbitrarilyJumpableGenerator$RandomArbitraryJumpsSpliterator:jdk/internal/util/random/RandomSupport$AbstractArbitrarilyJumpableGenerator:RandomArbitraryJumpsSpliterator:8;jdk/internal/util/random/RandomSupport$AbstractArbitrarilyJumpableGenerator$RandomLeapsSpliterator:jdk/internal/util/random/RandomSupport$AbstractArbitrarilyJumpableGenerator:RandomLeapsSpliterator:8;jdk/internal/util/random/RandomSupport$AbstractArbitrarilyJumpableGenerator$RandomJumpsSpliterator:jdk/internal/util/random/RandomSupport$AbstractArbitrarilyJumpableGenerator:RandomJumpsSpliterator:8;jdk/internal/util/random/RandomSupport$AbstractArbitrarilyJumpableGenerator$RandomDoublesSpliterator:jdk/internal/util/random/RandomSupport$AbstractArbitrarilyJumpableGenerator:RandomDoublesSpliterator:8;jdk/internal/util/random/RandomSupport$AbstractArbitrarilyJumpableGenerator$RandomLongsSpliterator:jdk/internal/util/random/RandomSupport$AbstractArbitrarilyJumpableGenerator:RandomLongsSpliterator:8;jdk/internal/util/random/RandomSupport$AbstractArbitrarilyJumpableGenerator$RandomIntsSpliterator:jdk/internal/util/random/RandomSupport$AbstractArbitrarilyJumpableGenerator:RandomIntsSpliterator:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;jdk/internal/util/random/RandomSupport"]

    pub struct RandomSupport;

    impl RandomSupport {
        #[cfg_attr(any(), java_field(name = "BAD_SIZE", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "size must be non-negative"))]
        // static field: BAD_SIZE:Ljava/lang/String;
        pub fn BAD_SIZE() -> String {
            String::from("size must be non-negative")
        }

        #[cfg_attr(any(), java_field(name = "BAD_DISTANCE", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "jump distance must be finite, positive, and an exact integer"))]
        // static field: BAD_DISTANCE:Ljava/lang/String;
        pub fn BAD_DISTANCE() -> String {
            String::from("jump distance must be finite, positive, and an exact integer")
        }

        #[cfg_attr(any(), java_field(name = "BAD_BOUND", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "bound must be positive"))]
        // static field: BAD_BOUND:Ljava/lang/String;
        pub fn BAD_BOUND() -> String {
            String::from("bound must be positive")
        }

        #[cfg_attr(any(), java_field(name = "BAD_FLOATING_BOUND", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "bound must be finite and positive"))]
        // static field: BAD_FLOATING_BOUND:Ljava/lang/String;
        pub fn BAD_FLOATING_BOUND() -> String {
            String::from("bound must be finite and positive")
        }

        #[cfg_attr(any(), java_field(name = "BAD_RANGE", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "bound must be greater than origin"))]
        // static field: BAD_RANGE:Ljava/lang/String;
        pub fn BAD_RANGE() -> String {
            String::from("bound must be greater than origin")
        }

        #[cfg_attr(any(), java_field(name = "useSecureRandomSeed", descriptor = "Z", access = "private", modifiers = "static final", is_static = true))]
        // static field: useSecureRandomSeed:Z
        pub fn useSecureRandomSeed() -> bool {
            panic!("stub: jdk/internal/util/random/RandomSupport.useSecureRandomSeed:Z")
        }

        #[cfg_attr(any(), java_field(name = "GOLDEN_RATIO_32", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "-1640531527"))]
        // static field: GOLDEN_RATIO_32:I
        pub fn GOLDEN_RATIO_32() -> i32 {
            -1640531527
        }

        #[cfg_attr(any(), java_field(name = "GOLDEN_RATIO_64", descriptor = "J", access = "public", modifiers = "static final", is_static = true, constant_value = "-7046029254386353131"))]
        // static field: GOLDEN_RATIO_64:J
        pub fn GOLDEN_RATIO_64() -> i64 {
            -7046029254386353131i64
        }

        #[cfg_attr(any(), java_field(name = "SILVER_RATIO_32", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1779033703"))]
        // static field: SILVER_RATIO_32:I
        pub fn SILVER_RATIO_32() -> i32 {
            1779033703
        }

        #[cfg_attr(any(), java_field(name = "SILVER_RATIO_64", descriptor = "J", access = "public", modifiers = "static final", is_static = true, constant_value = "7640891576956012809"))]
        // static field: SILVER_RATIO_64:J
        pub fn SILVER_RATIO_64() -> i64 {
            7640891576956012809i64
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: jdk/internal/util/random/RandomSupport.<init>:()V")
        }

        #[java_method(name = "checkStreamSize", descriptor = "(J)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkStreamSize(streamSize: i64) -> Result<()> {
            panic!("stub: jdk/internal/util/random/RandomSupport.checkStreamSize:(J)V")
        }

        #[java_method(name = "checkBound", descriptor = "(F)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkBound_f(bound: f32) -> Result<()> {
            panic!("stub: jdk/internal/util/random/RandomSupport.checkBound:(F)V")
        }

        #[java_method(name = "checkBound", descriptor = "(D)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkBound_d(bound: f64) -> Result<()> {
            panic!("stub: jdk/internal/util/random/RandomSupport.checkBound:(D)V")
        }

        #[java_method(name = "checkBound", descriptor = "(I)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkBound_i(bound: i32) -> Result<()> {
            panic!("stub: jdk/internal/util/random/RandomSupport.checkBound:(I)V")
        }

        #[java_method(name = "checkBound", descriptor = "(J)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkBound_l(bound: i64) -> Result<()> {
            panic!("stub: jdk/internal/util/random/RandomSupport.checkBound:(J)V")
        }

        #[java_method(name = "checkRange", descriptor = "(FF)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkRange_f_f(origin: f32, bound: f32) -> Result<()> {
            panic!("stub: jdk/internal/util/random/RandomSupport.checkRange:(FF)V")
        }

        #[java_method(name = "checkRange", descriptor = "(DD)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkRange_d_d(origin: f64, arg1: f64) -> Result<()> {
            panic!("stub: jdk/internal/util/random/RandomSupport.checkRange:(DD)V")
        }

        #[java_method(name = "checkRange", descriptor = "(II)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkRange_i_i(origin: i32, bound: i32) -> Result<()> {
            panic!("stub: jdk/internal/util/random/RandomSupport.checkRange:(II)V")
        }

        #[java_method(name = "checkRange", descriptor = "(JJ)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkRange_l_l(origin: i64, arg1: i64) -> Result<()> {
            panic!("stub: jdk/internal/util/random/RandomSupport.checkRange:(JJ)V")
        }

        #[java_method(name = "convertSeedBytesToLongs", descriptor = "([BII)[J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn convertSeedBytesToLongs(seed: Rc<RefCell<Vec<i8>>>, n: i32, z: i32) -> Result<Rc<RefCell<Vec<i64>>>> {
            panic!("stub: jdk/internal/util/random/RandomSupport.convertSeedBytesToLongs:([BII)[J")
        }

        #[java_method(name = "convertSeedBytesToInts", descriptor = "([BII)[I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn convertSeedBytesToInts(seed: Rc<RefCell<Vec<i8>>>, n: i32, z: i32) -> Result<Rc<RefCell<Vec<i32>>>> {
            panic!("stub: jdk/internal/util/random/RandomSupport.convertSeedBytesToInts:([BII)[I")
        }

        #[java_method(name = "boundedNextLong", descriptor = "(Ljava/util/random/RandomGenerator;JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn boundedNextLong_random_l_l(rng: Object, origin: i64, arg2: i64) -> Result<i64> {
            panic!("stub: jdk/internal/util/random/RandomSupport.boundedNextLong:(Ljava/util/random/RandomGenerator;JJ)J")
        }

        #[java_method(name = "boundedNextLong", descriptor = "(Ljava/util/random/RandomGenerator;J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn boundedNextLong_random_l(rng: Object, bound: i64) -> Result<i64> {
            panic!("stub: jdk/internal/util/random/RandomSupport.boundedNextLong:(Ljava/util/random/RandomGenerator;J)J")
        }

        #[java_method(name = "boundedNextInt", descriptor = "(Ljava/util/random/RandomGenerator;II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn boundedNextInt_random_i_i(rng: Object, origin: i32, bound: i32) -> Result<i32> {
            panic!("stub: jdk/internal/util/random/RandomSupport.boundedNextInt:(Ljava/util/random/RandomGenerator;II)I")
        }

        #[java_method(name = "boundedNextInt", descriptor = "(Ljava/util/random/RandomGenerator;I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn boundedNextInt_random_i(rng: Object, bound: i32) -> Result<i32> {
            panic!("stub: jdk/internal/util/random/RandomSupport.boundedNextInt:(Ljava/util/random/RandomGenerator;I)I")
        }

        #[java_method(name = "boundedNextDouble", descriptor = "(Ljava/util/random/RandomGenerator;DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn boundedNextDouble_random_d_d(rng: Object, origin: f64, arg2: f64) -> Result<f64> {
            panic!("stub: jdk/internal/util/random/RandomSupport.boundedNextDouble:(Ljava/util/random/RandomGenerator;DD)D")
        }

        #[java_method(name = "boundedNextDouble", descriptor = "(Ljava/util/random/RandomGenerator;D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn boundedNextDouble_random_d(rng: Object, bound: f64) -> Result<f64> {
            panic!("stub: jdk/internal/util/random/RandomSupport.boundedNextDouble:(Ljava/util/random/RandomGenerator;D)D")
        }

        #[java_method(name = "boundedNextFloat", descriptor = "(Ljava/util/random/RandomGenerator;FF)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn boundedNextFloat_random_f_f(rng: Object, origin: f32, bound: f32) -> Result<f32> {
            panic!("stub: jdk/internal/util/random/RandomSupport.boundedNextFloat:(Ljava/util/random/RandomGenerator;FF)F")
        }

        #[java_method(name = "boundedNextFloat", descriptor = "(Ljava/util/random/RandomGenerator;F)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn boundedNextFloat_random_f(rng: Object, bound: f32) -> Result<f32> {
            panic!("stub: jdk/internal/util/random/RandomSupport.boundedNextFloat:(Ljava/util/random/RandomGenerator;F)F")
        }

        #[java_method(name = "secureRandomSeedRequested", descriptor = "()Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn secureRandomSeedRequested() -> Result<bool> {
            panic!("stub: jdk/internal/util/random/RandomSupport.secureRandomSeedRequested:()Z")
        }

        #[java_method(name = "initialSeed", descriptor = "()J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn initialSeed() -> Result<i64> {
            panic!("stub: jdk/internal/util/random/RandomSupport.initialSeed:()J")
        }

        #[java_method(name = "mixMurmur64", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mixMurmur64(z: i64) -> Result<i64> {
            panic!("stub: jdk/internal/util/random/RandomSupport.mixMurmur64:(J)J")
        }

        #[java_method(name = "mixStafford13", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mixStafford13(z: i64) -> Result<i64> {
            panic!("stub: jdk/internal/util/random/RandomSupport.mixStafford13:(J)J")
        }

        #[java_method(name = "mixLea64", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mixLea64(z: i64) -> Result<i64> {
            panic!("stub: jdk/internal/util/random/RandomSupport.mixLea64:(J)J")
        }

        #[java_method(name = "mixMurmur32", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mixMurmur32(z: i32) -> Result<i32> {
            panic!("stub: jdk/internal/util/random/RandomSupport.mixMurmur32:(I)I")
        }

        #[java_method(name = "mixLea32", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mixLea32(z: i32) -> Result<i32> {
            panic!("stub: jdk/internal/util/random/RandomSupport.mixLea32:(I)I")
        }

        #[java_method(name = "computeNextExponential", descriptor = "(Ljava/util/random/RandomGenerator;)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn computeNextExponential(rng: Object) -> Result<f64> {
            panic!("stub: jdk/internal/util/random/RandomSupport.computeNextExponential:(Ljava/util/random/RandomGenerator;)D")
        }

        #[java_method(name = "computeNextGaussian", descriptor = "(Ljava/util/random/RandomGenerator;)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn computeNextGaussian(rng: Object) -> Result<f64> {
            panic!("stub: jdk/internal/util/random/RandomSupport.computeNextGaussian:(Ljava/util/random/RandomGenerator;)D")
        }
    }
}
