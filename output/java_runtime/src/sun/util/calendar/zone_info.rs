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

impl From<ZoneInfo> for TimeZone {
    fn from(v: ZoneInfo) -> TimeZone { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "sun/util/calendar/ZoneInfo"]
    #[super_class       = "java/util/TimeZone"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ZoneInfo.java"]
    #[inner_classes     = "sun/util/calendar/Gregorian$Date:sun/util/calendar/Gregorian:Date:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "TimeZone"]
    #[superclass_fields(ID: String, zoneId: ZoneId)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Object;java/util/TimeZone;sun/util/calendar/ZoneInfo"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct ZoneInfo {
        #[cfg_attr(any(), java_field(name = "rawOffset", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub rawOffset: i32,
        #[cfg_attr(any(), java_field(name = "rawOffsetDiff", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub rawOffsetDiff: i32,
        #[cfg_attr(any(), java_field(name = "checksum", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub checksum: i32,
        #[cfg_attr(any(), java_field(name = "dstSavings", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub dstSavings: i32,
        #[cfg_attr(any(), java_field(name = "transitions", descriptor = "[J", access = "private", modifiers = "", is_static = false))]
        pub transitions: Rc<RefCell<Vec<i64>>>,
        #[cfg_attr(any(), java_field(name = "offsets", descriptor = "[I", access = "private", modifiers = "", is_static = false))]
        pub offsets: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "simpleTimeZoneParams", descriptor = "[I", access = "private", modifiers = "", is_static = false))]
        pub simpleTimeZoneParams: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "willGMTOffsetChange", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub willGMTOffsetChange: bool,
        #[cfg_attr(any(), java_field(name = "dirty", descriptor = "Z", access = "private", modifiers = "transient", is_static = false))]
        pub dirty: bool,
        #[cfg_attr(any(), java_field(name = "lastRule", descriptor = "Ljava/util/SimpleTimeZone;", access = "private", modifiers = "transient", is_static = false))]
        pub lastRule: Object,
    }

    impl ZoneInfo {
        #[cfg_attr(any(), java_field(name = "UTC_TIME", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: UTC_TIME:I
        pub fn UTC_TIME() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "STANDARD_TIME", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: STANDARD_TIME:I
        pub fn STANDARD_TIME() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "WALL_TIME", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: WALL_TIME:I
        pub fn WALL_TIME() -> i32 {
            2
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

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "2653134537216586139"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            2653134537216586139i64
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: sun/util/calendar/ZoneInfo.<init>:()V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str_i(ID: String, rawOffset: i32) -> Result<Self> {
            panic!("stub: sun/util/calendar/ZoneInfo.<init>:(Ljava/lang/String;I)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;III[J[I[IZ)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str_i_i_i_arr_l_arr_i_arr_i_z(ID: String, rawOffset: i32, dstSavings: i32, checksum: i32, transitions: Rc<RefCell<Vec<i64>>>, offsets: Rc<RefCell<Vec<i32>>>, simpleTimeZoneParams: Rc<RefCell<Vec<i32>>>, willGMTOffsetChange: bool) -> Result<Self> {
            panic!("stub: sun/util/calendar/ZoneInfo.<init>:(Ljava/lang/String;III[J[I[IZ)V")
        }

        #[java_method(name = "getOffset", descriptor = "(J)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOffset_l(&self, date: i64) -> Result<i32> {
            panic!("stub: sun/util/calendar/ZoneInfo.getOffset:(J)I")
        }

        #[java_method(name = "getOffsets", descriptor = "(J[I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOffsets_l_arr_i(&self, utc: i64, arg1: Rc<RefCell<Vec<i32>>>) -> Result<i32> {
            panic!("stub: sun/util/calendar/ZoneInfo.getOffsets:(J[I)I")
        }

        #[java_method(name = "getOffsetsByStandard", descriptor = "(J[I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOffsetsByStandard(&self, standard: i64, arg1: Rc<RefCell<Vec<i32>>>) -> Result<i32> {
            panic!("stub: sun/util/calendar/ZoneInfo.getOffsetsByStandard:(J[I)I")
        }

        #[java_method(name = "getOffsetsByWall", descriptor = "(J[I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOffsetsByWall(&self, wall: i64, arg1: Rc<RefCell<Vec<i32>>>) -> Result<i32> {
            panic!("stub: sun/util/calendar/ZoneInfo.getOffsetsByWall:(J[I)I")
        }

        #[java_method(name = "getOffsets", descriptor = "(J[II)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOffsets_l_arr_i_i(&self, date: i64, arg1: Rc<RefCell<Vec<i32>>>, offsets: i32) -> Result<i32> {
            panic!("stub: sun/util/calendar/ZoneInfo.getOffsets:(J[II)I")
        }

        #[java_method(name = "getTransitionIndex", descriptor = "(JI)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTransitionIndex(&self, date: i64, arg1: i32) -> Result<i32> {
            panic!("stub: sun/util/calendar/ZoneInfo.getTransitionIndex:(JI)I")
        }

        #[java_method(name = "getOffset", descriptor = "(IIIIII)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOffset_i_i_i_i_i_i(&self, era: i32, year: i32, month: i32, day: i32, dayOfWeek: i32, milliseconds: i32) -> Result<i32> {
            panic!("stub: sun/util/calendar/ZoneInfo.getOffset:(IIIIII)I")
        }

        #[java_method(name = "setRawOffset", descriptor = "(I)V", access = "public", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setRawOffset(&self, offsetMillis: i32) -> Result<()> {
            panic!("stub: sun/util/calendar/ZoneInfo.setRawOffset:(I)V")
        }

        #[java_method(name = "getRawOffset", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getRawOffset(&self) -> Result<i32> {
            panic!("stub: sun/util/calendar/ZoneInfo.getRawOffset:()I")
        }

        #[java_method(name = "isDirty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDirty(&self) -> Result<bool> {
            panic!("stub: sun/util/calendar/ZoneInfo.isDirty:()Z")
        }

        #[java_method(name = "getLastRawOffset", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLastRawOffset(&self) -> Result<i32> {
            panic!("stub: sun/util/calendar/ZoneInfo.getLastRawOffset:()I")
        }

        #[java_method(name = "useDaylightTime", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn useDaylightTime(&self) -> Result<bool> {
            panic!("stub: sun/util/calendar/ZoneInfo.useDaylightTime:()Z")
        }

        #[java_method(name = "observesDaylightTime", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn observesDaylightTime(&self) -> Result<bool> {
            panic!("stub: sun/util/calendar/ZoneInfo.observesDaylightTime:()Z")
        }

        #[java_method(name = "inDaylightTime", descriptor = "(Ljava/util/Date;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn inDaylightTime(&self, date: Date) -> Result<bool> {
            panic!("stub: sun/util/calendar/ZoneInfo.inDaylightTime:(Ljava/util/Date;)Z")
        }

        #[java_method(name = "getDSTSavings", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDSTSavings(&self) -> Result<i32> {
            panic!("stub: sun/util/calendar/ZoneInfo.getDSTSavings:()I")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "getAvailableIDs", descriptor = "()[Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAvailableIDs() -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: sun/util/calendar/ZoneInfo.getAvailableIDs:()[Ljava/lang/String;")
        }

        #[java_method(name = "getAvailableIDs", descriptor = "(I)[Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAvailableIDs_i(rawOffset: i32) -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: sun/util/calendar/ZoneInfo.getAvailableIDs:(I)[Ljava/lang/String;")
        }

        #[java_method(name = "getTimeZone", descriptor = "(Ljava/lang/String;)Ljava/util/TimeZone;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTimeZone(ID: String) -> Result<TimeZone> {
            panic!("stub: sun/util/calendar/ZoneInfo.getTimeZone:(Ljava/lang/String;)Ljava/util/TimeZone;")
        }

        #[java_method(name = "getLastRule", descriptor = "()Ljava/util/SimpleTimeZone;", access = "private", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLastRule(&self) -> Result<Object> {
            panic!("stub: sun/util/calendar/ZoneInfo.getLastRule:()Ljava/util/SimpleTimeZone;")
        }

        #[java_method(name = "getLastRuleInstance", descriptor = "()Ljava/util/SimpleTimeZone;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLastRuleInstance(&self) -> Result<Object> {
            panic!("stub: sun/util/calendar/ZoneInfo.getLastRuleInstance:()Ljava/util/SimpleTimeZone;")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: sun/util/calendar/ZoneInfo.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: sun/util/calendar/ZoneInfo.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hasSameRules", descriptor = "(Ljava/util/TimeZone;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasSameRules(&self, other: TimeZone) -> Result<bool> {
            panic!("stub: sun/util/calendar/ZoneInfo.hasSameRules:(Ljava/util/TimeZone;)Z")
        }

        #[java_method(name = "getAliasTable", descriptor = "()Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Map<Ljava/lang/String;Ljava/lang/String;>;")]
        pub fn getAliasTable() -> Result<Object> {
            panic!("stub: sun/util/calendar/ZoneInfo.getAliasTable:()Ljava/util/Map;")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, stream: Object) -> Result<()> {
            panic!("stub: sun/util/calendar/ZoneInfo.readObject:(Ljava/io/ObjectInputStream;)V")
        }
    }
}
