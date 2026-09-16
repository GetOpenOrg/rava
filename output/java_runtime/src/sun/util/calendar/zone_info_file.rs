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
    #[binary_name       = "sun/util/calendar/ZoneInfoFile"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ZoneInfoFile.java"]
    #[inner_classes     = "sun/util/calendar/ZoneInfoFile$1:::0;sun/util/calendar/ZoneInfoFile$ZoneOffsetTransitionRule:sun/util/calendar/ZoneInfoFile:ZoneOffsetTransitionRule:10;sun/util/calendar/ZoneInfoFile$Checksum:sun/util/calendar/ZoneInfoFile:Checksum:10"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;sun/util/calendar/ZoneInfoFile"]

    pub struct ZoneInfoFile;

    impl ZoneInfoFile {
        #[cfg_attr(any(), java_field(name = "versionId", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static", is_static = true))]
        // static field: versionId:Ljava/lang/String;
        pub fn versionId() -> String {
            panic!("stub: sun/util/calendar/ZoneInfoFile.versionId:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "zones", descriptor = "Ljava/util/Map;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Map<Ljava/lang/String;Lsun/util/calendar/ZoneInfo;>;"))]
        // static field: zones:Ljava/util/Map;
        pub fn zones() -> Object {
            panic!("stub: sun/util/calendar/ZoneInfoFile.zones:Ljava/util/Map;")
        }

        #[cfg_attr(any(), java_field(name = "aliases", descriptor = "Ljava/util/Map;", access = "private", modifiers = "static", is_static = true, generic_signature = "Ljava/util/Map<Ljava/lang/String;Ljava/lang/String;>;"))]
        // static field: aliases:Ljava/util/Map;
        pub fn aliases() -> Object {
            panic!("stub: sun/util/calendar/ZoneInfoFile.aliases:Ljava/util/Map;")
        }

        #[cfg_attr(any(), java_field(name = "ruleArray", descriptor = "[[B", access = "private", modifiers = "static", is_static = true))]
        // static field: ruleArray:[[B
        pub fn ruleArray() -> Rc<RefCell<Vec<Rc<RefCell<Vec<i8>>>>>> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.ruleArray:[[B")
        }

        #[cfg_attr(any(), java_field(name = "regions", descriptor = "[Ljava/lang/String;", access = "private", modifiers = "static", is_static = true))]
        // static field: regions:[Ljava/lang/String;
        pub fn regions() -> Rc<RefCell<Vec<String>>> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.regions:[Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "indices", descriptor = "[I", access = "private", modifiers = "static", is_static = true))]
        // static field: indices:[I
        pub fn indices() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.indices:[I")
        }

        #[cfg_attr(any(), java_field(name = "USE_OLDMAPPING", descriptor = "Z", access = "private", modifiers = "static final", is_static = true))]
        // static field: USE_OLDMAPPING:Z
        pub fn USE_OLDMAPPING() -> bool {
            false
        }

        #[cfg_attr(any(), java_field(name = "oldMappings", descriptor = "[[Ljava/lang/String;", access = "private", modifiers = "static", is_static = true))]
        // static field: oldMappings:[[Ljava/lang/String;
        pub fn oldMappings() -> Rc<RefCell<Vec<Rc<RefCell<Vec<String>>>>>> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.oldMappings:[[Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "UTC1900", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-2208988800"))]
        // static field: UTC1900:J
        pub fn UTC1900() -> i64 {
            -2208988800i64
        }

        #[cfg_attr(any(), java_field(name = "UTC2100", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "4133980799"))]
        // static field: UTC2100:J
        pub fn UTC2100() -> i64 {
            4133980799i64
        }

        #[cfg_attr(any(), java_field(name = "LDT2100", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "4102444800"))]
        // static field: LDT2100:J
        pub fn LDT2100() -> i64 {
            4102444800i64
        }

        #[cfg_attr(any(), java_field(name = "CURRT", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: CURRT:J
        pub fn CURRT() -> i64 {
            panic!("stub: sun/util/calendar/ZoneInfoFile.CURRT:J")
        }

        #[cfg_attr(any(), java_field(name = "SECONDS_PER_DAY", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "86400"))]
        // static field: SECONDS_PER_DAY:I
        pub fn SECONDS_PER_DAY() -> i32 {
            86400
        }

        #[cfg_attr(any(), java_field(name = "DAYS_PER_CYCLE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "146097"))]
        // static field: DAYS_PER_CYCLE:I
        pub fn DAYS_PER_CYCLE() -> i32 {
            146097
        }

        #[cfg_attr(any(), java_field(name = "DAYS_0000_TO_1970", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "719528"))]
        // static field: DAYS_0000_TO_1970:J
        pub fn DAYS_0000_TO_1970() -> i64 {
            719528i64
        }

        #[cfg_attr(any(), java_field(name = "toCalendarDOW", descriptor = "[I", access = "private", modifiers = "static final", is_static = true))]
        // static field: toCalendarDOW:[I
        pub fn toCalendarDOW() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.toCalendarDOW:[I")
        }

        #[cfg_attr(any(), java_field(name = "toSTZTime", descriptor = "[I", access = "private", modifiers = "static final", is_static = true))]
        // static field: toSTZTime:[I
        pub fn toSTZTime() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.toSTZTime:[I")
        }

        #[cfg_attr(any(), java_field(name = "OFFSET_MASK", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "15"))]
        // static field: OFFSET_MASK:J
        pub fn OFFSET_MASK() -> i64 {
            15i64
        }

        #[cfg_attr(any(), java_field(name = "DST_MASK", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "240"))]
        // static field: DST_MASK:J
        pub fn DST_MASK() -> i64 {
            240i64
        }

        #[cfg_attr(any(), java_field(name = "DST_NSHIFT", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: DST_NSHIFT:I
        pub fn DST_NSHIFT() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "TRANSITION_NSHIFT", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "12"))]
        // static field: TRANSITION_NSHIFT:I
        pub fn TRANSITION_NSHIFT() -> i32 {
            12
        }

        #[cfg_attr(any(), java_field(name = "LASTYEAR", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2100"))]
        // static field: LASTYEAR:I
        pub fn LASTYEAR() -> i32 {
            2100
        }

        #[java_method(name = "getZoneIds", descriptor = "()[Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getZoneIds() -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.getZoneIds:()[Ljava/lang/String;")
        }

        #[java_method(name = "getZoneIds", descriptor = "(I)[Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getZoneIds_i(rawOffset: i32) -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.getZoneIds:(I)[Ljava/lang/String;")
        }

        #[java_method(name = "getZoneInfo", descriptor = "(Ljava/lang/String;)Lsun/util/calendar/ZoneInfo;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getZoneInfo_str(zoneId: String) -> Result<ZoneInfo> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.getZoneInfo:(Ljava/lang/String;)Lsun/util/calendar/ZoneInfo;")
        }

        #[java_method(name = "getZoneInfo0", descriptor = "(Ljava/lang/String;)Lsun/util/calendar/ZoneInfo;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getZoneInfo0(zoneId: String) -> Result<ZoneInfo> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.getZoneInfo0:(Ljava/lang/String;)Lsun/util/calendar/ZoneInfo;")
        }

        #[java_method(name = "getAliasMap", descriptor = "()Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Map<Ljava/lang/String;Ljava/lang/String;>;")]
        pub fn getAliasMap() -> Result<Object> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.getAliasMap:()Ljava/util/Map;")
        }

        #[java_method(name = "getVersion", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getVersion() -> Result<String> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.getVersion:()Ljava/lang/String;")
        }

        #[java_method(name = "getCustomTimeZone", descriptor = "(Ljava/lang/String;I)Lsun/util/calendar/ZoneInfo;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCustomTimeZone(originalId: String, gmtOffset: i32) -> Result<ZoneInfo> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.getCustomTimeZone:(Ljava/lang/String;I)Lsun/util/calendar/ZoneInfo;")
        }

        #[java_method(name = "toCustomID", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toCustomID(gmtOffset: i32) -> Result<String> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.toCustomID:(I)Ljava/lang/String;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.<init>:()V")
        }

        #[java_method(name = "loadTZDB", descriptor = "()V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn loadTZDB() -> Result<()> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.loadTZDB:()V")
        }

        #[java_method(name = "addOldMapping", descriptor = "()V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addOldMapping() -> Result<()> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.addOldMapping:()V")
        }

        #[java_method(name = "useOldMapping", descriptor = "()Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn useOldMapping() -> Result<bool> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.useOldMapping:()Z")
        }

        #[java_method(name = "load", descriptor = "(Ljava/io/DataInputStream;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn load(dis: DataInputStream) -> Result<()> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.load:(Ljava/io/DataInputStream;)V")
        }

        #[java_method(name = "getZoneInfo", descriptor = "(Ljava/io/DataInput;Ljava/lang/String;)Lsun/util/calendar/ZoneInfo;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/Exception")]
        pub fn getZoneInfo_datain_str(in_: Object, zoneId: String) -> Result<ZoneInfo> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.getZoneInfo:(Ljava/io/DataInput;Ljava/lang/String;)Lsun/util/calendar/ZoneInfo;")
        }

        #[java_method(name = "readOffset", descriptor = "(Ljava/io/DataInput;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readOffset(in_: Object) -> Result<i32> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.readOffset:(Ljava/io/DataInput;)I")
        }

        #[java_method(name = "readEpochSec", descriptor = "(Ljava/io/DataInput;)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readEpochSec(in_: Object) -> Result<i64> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.readEpochSec:(Ljava/io/DataInput;)J")
        }

        #[java_method(name = "getZoneInfo", descriptor = "(Ljava/lang/String;[J[I[J[I[Lsun/util/calendar/ZoneInfoFile$ZoneOffsetTransitionRule;)Lsun/util/calendar/ZoneInfo;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getZoneInfo_str_arr_l_arr_i_arr_l_arr_i_arr_zon(zoneId: String, standardTransitions: Rc<RefCell<Vec<i64>>>, standardOffsets: Rc<RefCell<Vec<i32>>>, savingsInstantTransitions: Rc<RefCell<Vec<i64>>>, wallOffsets: Rc<RefCell<Vec<i32>>>, lastRules: Rc<RefCell<Vec<Object>>>) -> Result<ZoneInfo> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.getZoneInfo:(Ljava/lang/String;[J[I[J[I[Lsun/util/calendar/ZoneInfoFile$ZoneOffsetTransitionRule;)Lsun/util/calendar/ZoneInfo;")
        }

        #[java_method(name = "getStandardOffset", descriptor = "([J[IJ)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getStandardOffset(standardTransitions: Rc<RefCell<Vec<i64>>>, standardOffsets: Rc<RefCell<Vec<i32>>>, epochSec: i64) -> Result<i32> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.getStandardOffset:([J[IJ)I")
        }

        #[java_method(name = "getYear", descriptor = "(JI)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getYear(epochSecond: i64, arg1: i32) -> Result<i32> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.getYear:(JI)I")
        }

        #[java_method(name = "indexOf", descriptor = "([IIII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOf(offsets: Rc<RefCell<Vec<i32>>>, from: i32, nOffsets: i32, offset: i32) -> Result<i32> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.indexOf:([IIII)I")
        }

        #[java_method(name = "addTrans", descriptor = "([JI[IIJII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addTrans(transitions: Rc<RefCell<Vec<i64>>>, nTrans: i32, offsets: Rc<RefCell<Vec<i32>>>, nOffsets: i32, trans: i64, arg5: i32, offset: i32) -> Result<i32> {
            panic!("stub: sun/util/calendar/ZoneInfoFile.addTrans:([JI[IIJII)I")
        }
    }
}
