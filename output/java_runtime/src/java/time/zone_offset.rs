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

impl From<ZoneOffset> for ZoneId {
    fn from(v: ZoneOffset) -> ZoneId { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/time/ZoneOffset"]
    #[super_class       = "java/time/ZoneId"]
    #[interfaces        = "java/time/temporal/TemporalAccessor,java/time/temporal/TemporalAdjuster,java/lang/Comparable,java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = "Ljava/time/ZoneId;Ljava/time/temporal/TemporalAccessor;Ljava/time/temporal/TemporalAdjuster;Ljava/lang/Comparable<Ljava/time/ZoneOffset;>;Ljava/io/Serializable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ZoneOffset.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "ZoneId"]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Object;java/time/ZoneId;java/time/ZoneOffset;java/time/temporal/TemporalAccessor;java/time/temporal/TemporalAdjuster"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct ZoneOffset {
        #[cfg_attr(any(), java_field(name = "totalSeconds", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub totalSeconds: i32,
        #[cfg_attr(any(), java_field(name = "id", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final transient", is_static = false))]
        pub id: String,
        #[cfg_attr(any(), java_field(name = "rules", descriptor = "Ljava/time/zone/ZoneRules;", access = "private", modifiers = "transient", is_static = false))]
        pub rules: ZoneRules,
    }

    impl ZoneOffset {
        #[cfg_attr(any(), java_field(name = "SECONDS_CACHE", descriptor = "Ljava/util/concurrent/ConcurrentMap;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/concurrent/ConcurrentMap<Ljava/lang/Integer;Ljava/time/ZoneOffset;>;"))]
        // static field: SECONDS_CACHE:Ljava/util/concurrent/ConcurrentMap;
        pub fn SECONDS_CACHE() -> Object {
            panic!("stub: java/time/ZoneOffset.SECONDS_CACHE:Ljava/util/concurrent/ConcurrentMap;")
        }

        #[cfg_attr(any(), java_field(name = "ID_CACHE", descriptor = "Ljava/util/concurrent/ConcurrentMap;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/concurrent/ConcurrentMap<Ljava/lang/String;Ljava/time/ZoneOffset;>;"))]
        // static field: ID_CACHE:Ljava/util/concurrent/ConcurrentMap;
        pub fn ID_CACHE() -> Object {
            panic!("stub: java/time/ZoneOffset.ID_CACHE:Ljava/util/concurrent/ConcurrentMap;")
        }

        #[cfg_attr(any(), java_field(name = "MAX_SECONDS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "64800"))]
        // static field: MAX_SECONDS:I
        pub fn MAX_SECONDS() -> i32 {
            64800
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "2357656521762053153"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            2357656521762053153i64
        }

        #[cfg_attr(any(), java_field(name = "UTC", descriptor = "Ljava/time/ZoneOffset;", access = "public", modifiers = "static final", is_static = true))]
        // static field: UTC:Ljava/time/ZoneOffset;
        pub fn UTC() -> ZoneOffset {
            panic!("stub: java/time/ZoneOffset.UTC:Ljava/time/ZoneOffset;")
        }

        #[cfg_attr(any(), java_field(name = "MIN", descriptor = "Ljava/time/ZoneOffset;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MIN:Ljava/time/ZoneOffset;
        pub fn MIN() -> ZoneOffset {
            panic!("stub: java/time/ZoneOffset.MIN:Ljava/time/ZoneOffset;")
        }

        #[cfg_attr(any(), java_field(name = "MAX", descriptor = "Ljava/time/ZoneOffset;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MAX:Ljava/time/ZoneOffset;
        pub fn MAX() -> ZoneOffset {
            panic!("stub: java/time/ZoneOffset.MAX:Ljava/time/ZoneOffset;")
        }

        #[java_method(name = "of", descriptor = "(Ljava/lang/String;)Ljava/time/ZoneOffset;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of(offsetId: String) -> Result<ZoneOffset> {
            panic!("stub: java/time/ZoneOffset.of:(Ljava/lang/String;)Ljava/time/ZoneOffset;")
        }

        #[java_method(name = "parseNumber", descriptor = "(Ljava/lang/CharSequence;IZ)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parseNumber(offsetId: Object, pos: i32, precededByColon: bool) -> Result<i32> {
            panic!("stub: java/time/ZoneOffset.parseNumber:(Ljava/lang/CharSequence;IZ)I")
        }

        #[java_method(name = "ofHours", descriptor = "(I)Ljava/time/ZoneOffset;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ofHours(hours: i32) -> Result<ZoneOffset> {
            panic!("stub: java/time/ZoneOffset.ofHours:(I)Ljava/time/ZoneOffset;")
        }

        #[java_method(name = "ofHoursMinutes", descriptor = "(II)Ljava/time/ZoneOffset;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ofHoursMinutes(hours: i32, minutes: i32) -> Result<ZoneOffset> {
            panic!("stub: java/time/ZoneOffset.ofHoursMinutes:(II)Ljava/time/ZoneOffset;")
        }

        #[java_method(name = "ofHoursMinutesSeconds", descriptor = "(III)Ljava/time/ZoneOffset;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ofHoursMinutesSeconds(hours: i32, minutes: i32, seconds: i32) -> Result<ZoneOffset> {
            panic!("stub: java/time/ZoneOffset.ofHoursMinutesSeconds:(III)Ljava/time/ZoneOffset;")
        }

        #[java_method(name = "from", descriptor = "(Ljava/time/temporal/TemporalAccessor;)Ljava/time/ZoneOffset;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn from(temporal: Object) -> Result<ZoneOffset> {
            panic!("stub: java/time/ZoneOffset.from:(Ljava/time/temporal/TemporalAccessor;)Ljava/time/ZoneOffset;")
        }

        #[java_method(name = "validate", descriptor = "(III)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn validate(hours: i32, minutes: i32, seconds: i32) -> Result<()> {
            panic!("stub: java/time/ZoneOffset.validate:(III)V")
        }

        #[java_method(name = "totalSeconds", descriptor = "(III)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn totalSeconds(hours: i32, minutes: i32, seconds: i32) -> Result<i32> {
            panic!("stub: java/time/ZoneOffset.totalSeconds:(III)I")
        }

        #[java_method(name = "ofTotalSeconds", descriptor = "(I)Ljava/time/ZoneOffset;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ofTotalSeconds(totalSeconds: i32) -> Result<ZoneOffset> {
            panic!("stub: java/time/ZoneOffset.ofTotalSeconds:(I)Ljava/time/ZoneOffset;")
        }

        #[java_method(name = "<init>", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(totalSeconds: i32) -> Result<Self> {
            panic!("stub: java/time/ZoneOffset.<init>:(I)V")
        }

        #[java_method(name = "buildId", descriptor = "(I)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn buildId(totalSeconds: i32) -> Result<String> {
            panic!("stub: java/time/ZoneOffset.buildId:(I)Ljava/lang/String;")
        }

        #[java_method(name = "getTotalSeconds", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTotalSeconds(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_totalSeconds())
        }

        #[java_method(name = "getId", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getId(&self) -> Result<String> {
            panic!("stub: java/time/ZoneOffset.getId:()Ljava/lang/String;")
        }

        #[java_method(name = "getRules", descriptor = "()Ljava/time/zone/ZoneRules;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getRules(&self) -> Result<ZoneRules> {
            panic!("stub: java/time/ZoneOffset.getRules:()Ljava/time/zone/ZoneRules;")
        }

        #[java_method(name = "normalized", descriptor = "()Ljava/time/ZoneId;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn normalized(&self) -> Result<ZoneId> {
            panic!("stub: java/time/ZoneOffset.normalized:()Ljava/time/ZoneId;")
        }

        #[java_method(name = "getOffset", descriptor = "(J)Ljava/time/ZoneOffset;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOffset(&self, epochSecond: i64) -> Result<ZoneOffset> {
            panic!("stub: java/time/ZoneOffset.getOffset:(J)Ljava/time/ZoneOffset;")
        }

        #[java_method(name = "isSupported", descriptor = "(Ljava/time/temporal/TemporalField;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSupported(&self, field: Object) -> Result<bool> {
            panic!("stub: java/time/ZoneOffset.isSupported:(Ljava/time/temporal/TemporalField;)Z")
        }

        #[java_method(name = "range", descriptor = "(Ljava/time/temporal/TemporalField;)Ljava/time/temporal/ValueRange;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn range(&self, field: Object) -> Result<ValueRange> {
            panic!("stub: java/time/ZoneOffset.range:(Ljava/time/temporal/TemporalField;)Ljava/time/temporal/ValueRange;")
        }

        #[java_method(name = "get", descriptor = "(Ljava/time/temporal/TemporalField;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get(&self, field: Object) -> Result<i32> {
            panic!("stub: java/time/ZoneOffset.get:(Ljava/time/temporal/TemporalField;)I")
        }

        #[java_method(name = "getLong", descriptor = "(Ljava/time/temporal/TemporalField;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLong(&self, field: Object) -> Result<i64> {
            panic!("stub: java/time/ZoneOffset.getLong:(Ljava/time/temporal/TemporalField;)J")
        }

        #[java_method(name = "query", descriptor = "(Ljava/time/temporal/TemporalQuery;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/time/temporal/TemporalQuery<TR;>;)TR;")]
        pub fn query(&self, query: Object) -> Result<Object> {
            panic!("stub: java/time/ZoneOffset.query:(Ljava/time/temporal/TemporalQuery;)Ljava/lang/Object;")
        }

        #[java_method(name = "adjustInto", descriptor = "(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn adjustInto(&self, temporal: Object) -> Result<Object> {
            panic!("stub: java/time/ZoneOffset.adjustInto:(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/time/ZoneOffset;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, other: ZoneOffset) -> Result<i32> {
            panic!("stub: java/time/ZoneOffset.compareTo:(Ljava/time/ZoneOffset;)I")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, mut obj: Object) -> Result<bool> {
            let this = self;
            if Object::from_any(this.clone()) == obj {
                return Ok((1i32 != 0i32));
            }
            return Ok(this.__get_totalSeconds() == (obj).downcast::<ZoneOffset>().__get_totalSeconds());
            Ok((0i32 != 0i32))
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "writeReplace", descriptor = "()Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn writeReplace(&self) -> Result<Object> {
            panic!("stub: java/time/ZoneOffset.writeReplace:()Ljava/lang/Object;")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/InvalidObjectException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/time/ZoneOffset.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "write", descriptor = "(Ljava/io/DataOutput;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn write(&self, out: Object) -> Result<()> {
            panic!("stub: java/time/ZoneOffset.write:(Ljava/io/DataOutput;)V")
        }

        #[java_method(name = "writeExternal", descriptor = "(Ljava/io/DataOutput;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeExternal(&self, out: Object) -> Result<()> {
            panic!("stub: java/time/ZoneOffset.writeExternal:(Ljava/io/DataOutput;)V")
        }

        #[java_method(name = "readExternal", descriptor = "(Ljava/io/DataInput;)Ljava/time/ZoneOffset;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readExternal(in_: Object) -> Result<ZoneOffset> {
            panic!("stub: java/time/ZoneOffset.readExternal:(Ljava/io/DataInput;)Ljava/time/ZoneOffset;")
        }
    }
}
