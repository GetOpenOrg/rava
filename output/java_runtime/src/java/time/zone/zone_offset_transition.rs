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
    #[binary_name       = "java/time/zone/ZoneOffsetTransition"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/lang/Comparable,java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = "Ljava/lang/Object;Ljava/lang/Comparable<Ljava/time/zone/ZoneOffsetTransition;>;Ljava/io/Serializable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ZoneOffsetTransition.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Object;java/time/zone/ZoneOffsetTransition"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct ZoneOffsetTransition {
        #[cfg_attr(any(), java_field(name = "epochSecond", descriptor = "J", access = "private", modifiers = "final", is_static = false))]
        pub epochSecond: i64,
        #[cfg_attr(any(), java_field(name = "transition", descriptor = "Ljava/time/LocalDateTime;", access = "private", modifiers = "final", is_static = false))]
        pub transition: LocalDateTime,
        #[cfg_attr(any(), java_field(name = "offsetBefore", descriptor = "Ljava/time/ZoneOffset;", access = "private", modifiers = "final", is_static = false))]
        pub offsetBefore: ZoneOffset,
        #[cfg_attr(any(), java_field(name = "offsetAfter", descriptor = "Ljava/time/ZoneOffset;", access = "private", modifiers = "final", is_static = false))]
        pub offsetAfter: ZoneOffset,
    }

    impl ZoneOffsetTransition {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-6946044323557704546"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -6946044323557704546i64
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "of", descriptor = "(Ljava/time/LocalDateTime;Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;)Ljava/time/zone/ZoneOffsetTransition;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of(transition: LocalDateTime, offsetBefore: ZoneOffset, offsetAfter: ZoneOffset) -> Result<ZoneOffsetTransition> {
            panic!("stub: java/time/zone/ZoneOffsetTransition.of:(Ljava/time/LocalDateTime;Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;)Ljava/time/zone/ZoneOffsetTransition;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/time/LocalDateTime;Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/time/LocalDateTime;Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;)V
        pub fn new_locald_zoneof_zoneof(mut transition: LocalDateTime, mut offsetBefore: ZoneOffset, mut offsetAfter: ZoneOffset) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            let _t0 = transition.getNano()?;
            if (_t0!=0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t1 = transition.toEpochSecond(Clone::clone(&offsetBefore))?;
            this.__set_epochSecond(_t1);
            this.__set_transition(Clone::clone(&transition));
            this.__set_offsetBefore(Clone::clone(&offsetBefore));
            this.__set_offsetAfter(Clone::clone(&offsetAfter));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(JLjava/time/ZoneOffset;Ljava/time/ZoneOffset;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_l_zoneof_zoneof(epochSecond: i64, arg1: ZoneOffset, offsetBefore: ZoneOffset) -> Result<Self> {
            panic!("stub: java/time/zone/ZoneOffsetTransition.<init>:(JLjava/time/ZoneOffset;Ljava/time/ZoneOffset;)V")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/InvalidObjectException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/time/zone/ZoneOffsetTransition.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "writeReplace", descriptor = "()Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn writeReplace(&self) -> Result<Object> {
            panic!("stub: java/time/zone/ZoneOffsetTransition.writeReplace:()Ljava/lang/Object;")
        }

        #[java_method(name = "writeExternal", descriptor = "(Ljava/io/DataOutput;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeExternal(&self, out: Object) -> Result<()> {
            panic!("stub: java/time/zone/ZoneOffsetTransition.writeExternal:(Ljava/io/DataOutput;)V")
        }

        #[java_method(name = "readExternal", descriptor = "(Ljava/io/DataInput;)Ljava/time/zone/ZoneOffsetTransition;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readExternal(in_: Object) -> Result<ZoneOffsetTransition> {
            panic!("stub: java/time/zone/ZoneOffsetTransition.readExternal:(Ljava/io/DataInput;)Ljava/time/zone/ZoneOffsetTransition;")
        }

        #[java_method(name = "getInstant", descriptor = "()Ljava/time/Instant;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInstant(&self) -> Result<Instant> {
            panic!("stub: java/time/zone/ZoneOffsetTransition.getInstant:()Ljava/time/Instant;")
        }

        #[java_method(name = "toEpochSecond", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toEpochSecond(&self) -> Result<i64> {
            let this = self;
            Ok(this.__get_epochSecond())
        }

        #[java_method(name = "getDateTimeBefore", descriptor = "()Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDateTimeBefore(&self) -> Result<LocalDateTime> {
            panic!("stub: java/time/zone/ZoneOffsetTransition.getDateTimeBefore:()Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "getDateTimeAfter", descriptor = "()Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDateTimeAfter(&self) -> Result<LocalDateTime> {
            panic!("stub: java/time/zone/ZoneOffsetTransition.getDateTimeAfter:()Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "getOffsetBefore", descriptor = "()Ljava/time/ZoneOffset;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOffsetBefore(&self) -> Result<ZoneOffset> {
            let this = self;
            Ok(this.__get_offsetBefore())
        }

        #[java_method(name = "getOffsetAfter", descriptor = "()Ljava/time/ZoneOffset;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOffsetAfter(&self) -> Result<ZoneOffset> {
            let this = self;
            Ok(this.__get_offsetAfter())
        }

        #[java_method(name = "getDuration", descriptor = "()Ljava/time/Duration;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDuration(&self) -> Result<Object> {
            panic!("stub: java/time/zone/ZoneOffsetTransition.getDuration:()Ljava/time/Duration;")
        }

        #[java_method(name = "getDurationSeconds", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDurationSeconds(&self) -> Result<i32> {
            panic!("stub: java/time/zone/ZoneOffsetTransition.getDurationSeconds:()I")
        }

        #[java_method(name = "isGap", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isGap(&self) -> Result<bool> {
            panic!("stub: java/time/zone/ZoneOffsetTransition.isGap:()Z")
        }

        #[java_method(name = "isOverlap", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isOverlap(&self) -> Result<bool> {
            panic!("stub: java/time/zone/ZoneOffsetTransition.isOverlap:()Z")
        }

        #[java_method(name = "isValidOffset", descriptor = "(Ljava/time/ZoneOffset;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isValidOffset(&self, offset: ZoneOffset) -> Result<bool> {
            panic!("stub: java/time/zone/ZoneOffsetTransition.isValidOffset:(Ljava/time/ZoneOffset;)Z")
        }

        #[java_method(name = "getValidOffsets", descriptor = "()Ljava/util/List;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/List<Ljava/time/ZoneOffset;>;")]
        pub fn getValidOffsets(&self) -> Result<Object> {
            panic!("stub: java/time/zone/ZoneOffsetTransition.getValidOffsets:()Ljava/util/List;")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/time/zone/ZoneOffsetTransition;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, otherTransition: ZoneOffsetTransition) -> Result<i32> {
            panic!("stub: java/time/zone/ZoneOffsetTransition.compareTo:(Ljava/time/zone/ZoneOffsetTransition;)I")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, other: Object) -> Result<bool> {
            panic!("stub: java/time/zone/ZoneOffsetTransition.equals:(Ljava/lang/Object;)Z")
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
