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
    #[binary_name       = "java/time/chrono/AbstractChronology"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/time/chrono/Chronology"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AbstractChronology.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/time/chrono/AbstractChronology;java/time/chrono/Chronology"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct AbstractChronology;

    impl AbstractChronology {
        #[cfg_attr(any(), java_field(name = "CHRONOS_BY_ID", descriptor = "Ljava/util/concurrent/ConcurrentHashMap;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/concurrent/ConcurrentHashMap<Ljava/lang/String;Ljava/time/chrono/Chronology;>;"))]
        // static field: CHRONOS_BY_ID:Ljava/util/concurrent/ConcurrentHashMap;
        pub fn CHRONOS_BY_ID() -> ConcurrentHashMap<Object, Object> {
            panic!("stub: java/time/chrono/AbstractChronology.CHRONOS_BY_ID:Ljava/util/concurrent/ConcurrentHashMap;")
        }

        #[cfg_attr(any(), java_field(name = "CHRONOS_BY_TYPE", descriptor = "Ljava/util/concurrent/ConcurrentHashMap;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/concurrent/ConcurrentHashMap<Ljava/lang/String;Ljava/time/chrono/Chronology;>;"))]
        // static field: CHRONOS_BY_TYPE:Ljava/util/concurrent/ConcurrentHashMap;
        pub fn CHRONOS_BY_TYPE() -> ConcurrentHashMap<Object, Object> {
            panic!("stub: java/time/chrono/AbstractChronology.CHRONOS_BY_TYPE:Ljava/util/concurrent/ConcurrentHashMap;")
        }

        #[java_method(name = "registerChrono", descriptor = "(Ljava/time/chrono/Chronology;)Ljava/time/chrono/Chronology;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn registerChrono_chrono(chrono: Object) -> Result<Object> {
            panic!("stub: java/time/chrono/AbstractChronology.registerChrono:(Ljava/time/chrono/Chronology;)Ljava/time/chrono/Chronology;")
        }

        #[java_method(name = "registerChrono", descriptor = "(Ljava/time/chrono/Chronology;Ljava/lang/String;)Ljava/time/chrono/Chronology;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn registerChrono_chrono_str(chrono: Object, id: String) -> Result<Object> {
            panic!("stub: java/time/chrono/AbstractChronology.registerChrono:(Ljava/time/chrono/Chronology;Ljava/lang/String;)Ljava/time/chrono/Chronology;")
        }

        #[java_method(name = "initCache", descriptor = "()Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn initCache() -> Result<bool> {
            panic!("stub: java/time/chrono/AbstractChronology.initCache:()Z")
        }

        #[java_method(name = "ofLocale", descriptor = "(Ljava/util/Locale;)Ljava/time/chrono/Chronology;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ofLocale(locale: Locale) -> Result<Object> {
            panic!("stub: java/time/chrono/AbstractChronology.ofLocale:(Ljava/util/Locale;)Ljava/time/chrono/Chronology;")
        }

        #[java_method(name = "of", descriptor = "(Ljava/lang/String;)Ljava/time/chrono/Chronology;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of(id: String) -> Result<Object> {
            panic!("stub: java/time/chrono/AbstractChronology.of:(Ljava/lang/String;)Ljava/time/chrono/Chronology;")
        }

        #[java_method(name = "of0", descriptor = "(Ljava/lang/String;)Ljava/time/chrono/Chronology;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of0(id: String) -> Result<Object> {
            panic!("stub: java/time/chrono/AbstractChronology.of0:(Ljava/lang/String;)Ljava/time/chrono/Chronology;")
        }

        #[java_method(name = "getAvailableChronologies", descriptor = "()Ljava/util/Set;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/time/chrono/Chronology;>;")]
        pub fn getAvailableChronologies() -> Result<Object> {
            panic!("stub: java/time/chrono/AbstractChronology.getAvailableChronologies:()Ljava/util/Set;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/time/chrono/AbstractChronology.<init>:()V")
        }

        #[java_method(name = "resolveDate", descriptor = "(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/time/temporal/TemporalField;Ljava/lang/Long;>;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;")]
        pub fn resolveDate(&self, fieldValues: Object, resolverStyle: Object) -> Result<Object> {
            panic!("stub: java/time/chrono/AbstractChronology.resolveDate:(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;")
        }

        #[java_method(name = "resolveProlepticMonth", descriptor = "(Ljava/util/Map;Ljava/time/format/ResolverStyle;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/time/temporal/TemporalField;Ljava/lang/Long;>;Ljava/time/format/ResolverStyle;)V")]
        pub fn resolveProlepticMonth(&self, fieldValues: Object, resolverStyle: Object) -> Result<()> {
            panic!("stub: java/time/chrono/AbstractChronology.resolveProlepticMonth:(Ljava/util/Map;Ljava/time/format/ResolverStyle;)V")
        }

        #[java_method(name = "resolveYearOfEra", descriptor = "(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/time/temporal/TemporalField;Ljava/lang/Long;>;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;")]
        pub fn resolveYearOfEra(&self, fieldValues: Object, resolverStyle: Object) -> Result<Object> {
            panic!("stub: java/time/chrono/AbstractChronology.resolveYearOfEra:(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;")
        }

        #[java_method(name = "resolveYMD", descriptor = "(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/time/temporal/TemporalField;Ljava/lang/Long;>;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;")]
        pub fn resolveYMD(&self, fieldValues: Object, resolverStyle: Object) -> Result<Object> {
            panic!("stub: java/time/chrono/AbstractChronology.resolveYMD:(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;")
        }

        #[java_method(name = "resolveYD", descriptor = "(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/time/temporal/TemporalField;Ljava/lang/Long;>;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;")]
        pub fn resolveYD(&self, fieldValues: Object, resolverStyle: Object) -> Result<Object> {
            panic!("stub: java/time/chrono/AbstractChronology.resolveYD:(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;")
        }

        #[java_method(name = "resolveYMAA", descriptor = "(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/time/temporal/TemporalField;Ljava/lang/Long;>;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;")]
        pub fn resolveYMAA(&self, fieldValues: Object, resolverStyle: Object) -> Result<Object> {
            panic!("stub: java/time/chrono/AbstractChronology.resolveYMAA:(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;")
        }

        #[java_method(name = "resolveYMAD", descriptor = "(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/time/temporal/TemporalField;Ljava/lang/Long;>;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;")]
        pub fn resolveYMAD(&self, fieldValues: Object, resolverStyle: Object) -> Result<Object> {
            panic!("stub: java/time/chrono/AbstractChronology.resolveYMAD:(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;")
        }

        #[java_method(name = "resolveYAA", descriptor = "(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/time/temporal/TemporalField;Ljava/lang/Long;>;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;")]
        pub fn resolveYAA(&self, fieldValues: Object, resolverStyle: Object) -> Result<Object> {
            panic!("stub: java/time/chrono/AbstractChronology.resolveYAA:(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;")
        }

        #[java_method(name = "resolveYAD", descriptor = "(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/time/temporal/TemporalField;Ljava/lang/Long;>;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;")]
        pub fn resolveYAD(&self, fieldValues: Object, resolverStyle: Object) -> Result<Object> {
            panic!("stub: java/time/chrono/AbstractChronology.resolveYAD:(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/chrono/ChronoLocalDate;")
        }

        #[java_method(name = "resolveAligned", descriptor = "(Ljava/time/chrono/ChronoLocalDate;JJJ)Ljava/time/chrono/ChronoLocalDate;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn resolveAligned(&self, base: Object, months: i64, arg2: i64, weeks: i64) -> Result<Object> {
            panic!("stub: java/time/chrono/AbstractChronology.resolveAligned:(Ljava/time/chrono/ChronoLocalDate;JJJ)Ljava/time/chrono/ChronoLocalDate;")
        }

        #[java_method(name = "addFieldValue", descriptor = "(Ljava/util/Map;Ljava/time/temporal/ChronoField;J)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/time/temporal/TemporalField;Ljava/lang/Long;>;Ljava/time/temporal/ChronoField;J)V")]
        pub fn addFieldValue(&self, fieldValues: Object, field: ChronoField, value: i64) -> Result<()> {
            panic!("stub: java/time/chrono/AbstractChronology.addFieldValue:(Ljava/util/Map;Ljava/time/temporal/ChronoField;J)V")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/time/chrono/Chronology;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, other: Object) -> Result<i32> {
            panic!("stub: java/time/chrono/AbstractChronology.compareTo:(Ljava/time/chrono/Chronology;)I")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/time/chrono/AbstractChronology.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "writeReplace", descriptor = "()Ljava/lang/Object;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn writeReplace(&self) -> Result<Object> {
            panic!("stub: java/time/chrono/AbstractChronology.writeReplace:()Ljava/lang/Object;")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/ObjectStreamException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/time/chrono/AbstractChronology.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "writeExternal", descriptor = "(Ljava/io/DataOutput;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeExternal(&self, out: Object) -> Result<()> {
            panic!("stub: java/time/chrono/AbstractChronology.writeExternal:(Ljava/io/DataOutput;)V")
        }

        #[java_method(name = "readExternal", descriptor = "(Ljava/io/DataInput;)Ljava/time/chrono/Chronology;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readExternal(in_: Object) -> Result<Object> {
            panic!("stub: java/time/chrono/AbstractChronology.readExternal:(Ljava/io/DataInput;)Ljava/time/chrono/Chronology;")
        }
    }
}
