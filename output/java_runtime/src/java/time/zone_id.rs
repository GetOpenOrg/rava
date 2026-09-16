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
    #[binary_name       = "java/time/ZoneId"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ZoneId.java"]
    #[inner_classes     = "java/time/ZoneId$1:::0;java/util/Map$Entry:java/util/Map:Entry:1545"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/time/ZoneId"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct ZoneId;

    impl ZoneId {
        #[cfg_attr(any(), java_field(name = "SHORT_IDS", descriptor = "Ljava/util/Map;", access = "public", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Map<Ljava/lang/String;Ljava/lang/String;>;"))]
        // static field: SHORT_IDS:Ljava/util/Map;
        pub fn SHORT_IDS() -> Object {
            panic!("stub: java/time/ZoneId.SHORT_IDS:Ljava/util/Map;")
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "8352817235686"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            8352817235686i64
        }

        #[java_method(name = "systemDefault", descriptor = "()Ljava/time/ZoneId;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn systemDefault() -> Result<ZoneId> {
            panic!("stub: java/time/ZoneId.systemDefault:()Ljava/time/ZoneId;")
        }

        #[java_method(name = "getAvailableZoneIds", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/lang/String;>;")]
        pub fn getAvailableZoneIds() -> Result<Object> {
            panic!("stub: java/time/ZoneId.getAvailableZoneIds:()Ljava/util/Set;")
        }

        #[java_method(name = "of", descriptor = "(Ljava/lang/String;Ljava/util/Map;)Ljava/time/ZoneId;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;Ljava/util/Map<Ljava/lang/String;Ljava/lang/String;>;)Ljava/time/ZoneId;")]
        pub fn of_str_map(zoneId: String, aliasMap: Object) -> Result<ZoneId> {
            panic!("stub: java/time/ZoneId.of:(Ljava/lang/String;Ljava/util/Map;)Ljava/time/ZoneId;")
        }

        #[java_method(name = "of", descriptor = "(Ljava/lang/String;)Ljava/time/ZoneId;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_str(zoneId: String) -> Result<ZoneId> {
            panic!("stub: java/time/ZoneId.of:(Ljava/lang/String;)Ljava/time/ZoneId;")
        }

        #[java_method(name = "ofOffset", descriptor = "(Ljava/lang/String;Ljava/time/ZoneOffset;)Ljava/time/ZoneId;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ofOffset(prefix: String, offset: ZoneOffset) -> Result<ZoneId> {
            panic!("stub: java/time/ZoneId.ofOffset:(Ljava/lang/String;Ljava/time/ZoneOffset;)Ljava/time/ZoneId;")
        }

        #[java_method(name = "of", descriptor = "(Ljava/lang/String;Z)Ljava/time/ZoneId;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_str_z(zoneId: String, checkAvailable: bool) -> Result<ZoneId> {
            panic!("stub: java/time/ZoneId.of:(Ljava/lang/String;Z)Ljava/time/ZoneId;")
        }

        #[java_method(name = "ofWithPrefix", descriptor = "(Ljava/lang/String;IZ)Ljava/time/ZoneId;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ofWithPrefix(zoneId: String, prefixLength: i32, checkAvailable: bool) -> Result<ZoneId> {
            panic!("stub: java/time/ZoneId.ofWithPrefix:(Ljava/lang/String;IZ)Ljava/time/ZoneId;")
        }

        #[java_method(name = "from", descriptor = "(Ljava/time/temporal/TemporalAccessor;)Ljava/time/ZoneId;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn from(temporal: Object) -> Result<ZoneId> {
            panic!("stub: java/time/ZoneId.from:(Ljava/time/temporal/TemporalAccessor;)Ljava/time/ZoneId;")
        }

        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "getId", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getId(&self) -> Result<String> {
            panic!("stub: java/time/ZoneId.getId:()Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayName", descriptor = "(Ljava/time/format/TextStyle;Ljava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayName(&self, style: Object, locale: Locale) -> Result<String> {
            panic!("stub: java/time/ZoneId.getDisplayName:(Ljava/time/format/TextStyle;Ljava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "toTemporal", descriptor = "()Ljava/time/temporal/TemporalAccessor;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toTemporal(&self) -> Result<Object> {
            panic!("stub: java/time/ZoneId.toTemporal:()Ljava/time/temporal/TemporalAccessor;")
        }

        #[java_method(name = "getRules", descriptor = "()Ljava/time/zone/ZoneRules;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getRules(&self) -> Result<ZoneRules> {
            panic!("stub: java/time/ZoneId.getRules:()Ljava/time/zone/ZoneRules;")
        }

        #[java_method(name = "normalized", descriptor = "()Ljava/time/ZoneId;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn normalized(&self) -> Result<ZoneId> {
            panic!("stub: java/time/ZoneId.normalized:()Ljava/time/ZoneId;")
        }

        #[java_method(name = "getOffset", descriptor = "(J)Ljava/time/ZoneOffset;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getOffset(&self, arg0: i64) -> Result<ZoneOffset> {
            panic!("stub: java/time/ZoneId.getOffset:(J)Ljava/time/ZoneOffset;")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/time/ZoneId.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/InvalidObjectException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/time/ZoneId.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "writeReplace", descriptor = "()Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn writeReplace(&self) -> Result<Object> {
            panic!("stub: java/time/ZoneId.writeReplace:()Ljava/lang/Object;")
        }

        #[java_method(name = "write", descriptor = "(Ljava/io/DataOutput;)V", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn write(&self, arg0: Object) -> Result<()> {
            panic!("stub: java/time/ZoneId.write:(Ljava/io/DataOutput;)V")
        }
    }
}
