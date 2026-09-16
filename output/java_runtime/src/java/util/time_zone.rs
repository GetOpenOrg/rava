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
use crate::jdk::internal::util::StaticProperty;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/TimeZone"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Serializable,java/lang/Cloneable"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TimeZone.java"]
    #[inner_classes     = "java/util/Locale$Category:java/util/Locale:Category:16409"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Object;java/util/TimeZone"]

    pub struct TimeZone {
        #[cfg_attr(any(), java_field(name = "ID", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub ID: String,
        #[cfg_attr(any(), java_field(name = "zoneId", descriptor = "Ljava/time/ZoneId;", access = "private", modifiers = "transient", is_static = false))]
        pub zoneId: ZoneId,
    }

    impl TimeZone {
        #[cfg_attr(any(), java_field(name = "SHORT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: SHORT:I
        pub fn SHORT() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "LONG", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: LONG:I
        pub fn LONG() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "ONE_MINUTE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "60000"))]
        // static field: ONE_MINUTE:I
        pub fn ONE_MINUTE() -> i32 {
            60000
        }

        #[cfg_attr(any(), java_field(name = "ONE_HOUR", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "3600000"))]
        // static field: ONE_HOUR:I
        pub fn ONE_HOUR() -> i32 {
            3600000
        }

        #[cfg_attr(any(), java_field(name = "ONE_DAY", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "86400000"))]
        // static field: ONE_DAY:I
        pub fn ONE_DAY() -> i32 {
            86400000
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "3581463369166924961"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            3581463369166924961i64
        }

        #[cfg_attr(any(), java_field(name = "NO_TIMEZONE", descriptor = "Ljava/util/TimeZone;", access = "package", modifiers = "static final", is_static = true))]
        // static field: NO_TIMEZONE:Ljava/util/TimeZone;
        pub fn NO_TIMEZONE() -> TimeZone {
            panic!("stub: java/util/TimeZone.NO_TIMEZONE:Ljava/util/TimeZone;")
        }

        #[cfg_attr(any(), java_field(name = "defaultTimeZone", descriptor = "Ljava/util/TimeZone;", access = "private", modifiers = "static volatile", is_static = true))]
        // static field: defaultTimeZone:Ljava/util/TimeZone;
        pub fn defaultTimeZone() -> TimeZone {
            panic!("stub: java/util/TimeZone.defaultTimeZone:Ljava/util/TimeZone;")
        }

        #[cfg_attr(any(), java_field(name = "GMT_ID", descriptor = "Ljava/lang/String;", access = "package", modifiers = "static final", is_static = true, constant_value = "GMT"))]
        // static field: GMT_ID:Ljava/lang/String;
        pub fn GMT_ID() -> String {
            String::from("GMT")
        }

        #[cfg_attr(any(), java_field(name = "GMT_ID_LENGTH", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: GMT_ID_LENGTH:I
        pub fn GMT_ID_LENGTH() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "getOffset", descriptor = "(IIIIII)I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getOffset_i_i_i_i_i_i(&self, arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: i32) -> Result<i32> {
            panic!("stub: java/util/TimeZone.getOffset:(IIIIII)I")
        }

        #[java_method(name = "getOffset", descriptor = "(J)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getOffset(J)I
        pub fn getOffset_l(&self, mut date: i64) -> Result<i32> {
            let this = self;
            let _t0 = this.inDaylightTime(Clone::clone(&Date::new_l(date)?))?;
            if _t0 {
                let _t1 = this.getRawOffset()?;
                let _t2 = this.getDSTSavings()?;
                return Ok((_t1).wrapping_add(_t2));
            }
            let _t1 = this.getRawOffset()?;
            Ok(_t1)
        }

        #[java_method(name = "getOffsets", descriptor = "(J[I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOffsets(&self, date: i64, arg1: Rc<RefCell<Vec<i32>>>) -> Result<i32> {
            panic!("stub: java/util/TimeZone.getOffsets:(J[I)I")
        }

        #[java_method(name = "setRawOffset", descriptor = "(I)V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn setRawOffset(&self, arg0: i32) -> Result<()> {
            panic!("stub: java/util/TimeZone.setRawOffset:(I)V")
        }

        #[java_method(name = "getRawOffset", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getRawOffset(&self) -> Result<i32> {
            panic!("stub: java/util/TimeZone.getRawOffset:()I")
        }

        #[java_method(name = "getID", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getID(&self) -> Result<String> {
            let this = self;
            Ok(this.__get_ID())
        }

        #[java_method(name = "setID", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setID(&self, ID: String) -> Result<()> {
            panic!("stub: java/util/TimeZone.setID:(Ljava/lang/String;)V")
        }

        #[java_method(name = "getDisplayName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayName(&self) -> Result<String> {
            panic!("stub: java/util/TimeZone.getDisplayName:()Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayName", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayName_locale(&self, locale: Locale) -> Result<String> {
            panic!("stub: java/util/TimeZone.getDisplayName:(Ljava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayName", descriptor = "(ZI)Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayName_z_i(&self, daylight: bool, style: i32) -> Result<String> {
            panic!("stub: java/util/TimeZone.getDisplayName:(ZI)Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayName", descriptor = "(ZILjava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getDisplayName(ZILjava/util/Locale;)Ljava/lang/String;
        pub fn getDisplayName_z_i_locale(&self, mut daylight: bool, mut style: i32, mut locale: Locale) -> Result<String> {
            let this = self;
            if style != 1i32 {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Illegal style: ")))?;
                let _t1 = _t0.append_i(style)?;
                let _t2 = _t1.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0 = this.getID()?;
            let mut id: String = _t0;
            let _t1: String = TimeZoneNameUtility::retrieveDisplayName(Clone::clone(&id), daylight, style, Clone::clone(&locale))?;
            let mut name: String = _t1;
            if !_is_jnull(&name) {
                return Ok(name);
            }
            let _t2 = id.startsWith_str(Clone::clone(&String::from("GMT")))?;
            let _t3 = id.length()?;
            let _t4 = id.charAt(3i32)?;
            let mut sign: u16 = _t4;
            if (sign as i32) == 45i32 {
                return Ok(id);
            }
            let _t5 = this.getRawOffset()?;
            let mut sign: i32 = _t5;
            if daylight {
                let _t6 = this.getDSTSavings()?;
                sign = (sign).wrapping_add(_t6);
            }
            let _t6: String = ZoneInfoFile::toCustomID(sign)?;
            Ok(_t6)
        }

        #[java_method(name = "getDisplayNames", descriptor = "(Ljava/lang/String;Ljava/util/Locale;)[Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayNames(id: String, locale: Locale) -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: java/util/TimeZone.getDisplayNames:(Ljava/lang/String;Ljava/util/Locale;)[Ljava/lang/String;")
        }

        #[java_method(name = "getDSTSavings", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDSTSavings(&self) -> Result<i32> {
            let this = self;
            let _t0 = this.useDaylightTime()?;
            if _t0 {
                return Ok(3600000i32);
            }
            Ok(0i32)
        }

        #[java_method(name = "useDaylightTime", descriptor = "()Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn useDaylightTime(&self) -> Result<bool> {
            panic!("stub: java/util/TimeZone.useDaylightTime:()Z")
        }

        #[java_method(name = "observesDaylightTime", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn observesDaylightTime(&self) -> Result<bool> {
            panic!("stub: java/util/TimeZone.observesDaylightTime:()Z")
        }

        #[java_method(name = "inDaylightTime", descriptor = "(Ljava/util/Date;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn inDaylightTime(&self, arg0: Date) -> Result<bool> {
            panic!("stub: java/util/TimeZone.inDaylightTime:(Ljava/util/Date;)Z")
        }

        #[java_method(name = "getTimeZone", descriptor = "(Ljava/lang/String;)Ljava/util/TimeZone;", access = "public", modifiers = "static synchronized", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getTimeZone(Ljava/lang/String;)Ljava/util/TimeZone;
        pub fn getTimeZone_str(mut ID: String) -> Result<TimeZone> {
            let _t0: TimeZone = TimeZone::getTimeZone_str_z(Clone::clone(&ID), (1i32 != 0i32))?;
            Ok(_t0)
        }

        #[java_method(name = "getTimeZone", descriptor = "(Ljava/time/ZoneId;)Ljava/util/TimeZone;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTimeZone_zoneid(zoneId: ZoneId) -> Result<TimeZone> {
            panic!("stub: java/util/TimeZone.getTimeZone:(Ljava/time/ZoneId;)Ljava/util/TimeZone;")
        }

        #[java_method(name = "toZoneId", descriptor = "()Ljava/time/ZoneId;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toZoneId(&self) -> Result<ZoneId> {
            panic!("stub: java/util/TimeZone.toZoneId:()Ljava/time/ZoneId;")
        }

        #[java_method(name = "toZoneId0", descriptor = "()Ljava/time/ZoneId;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toZoneId0(&self) -> Result<ZoneId> {
            panic!("stub: java/util/TimeZone.toZoneId0:()Ljava/time/ZoneId;")
        }

        #[java_method(name = "getTimeZone", descriptor = "(Ljava/lang/String;Z)Ljava/util/TimeZone;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getTimeZone(Ljava/lang/String;Z)Ljava/util/TimeZone;
        pub fn getTimeZone_str_z(mut ID: String, mut fallback: bool) -> Result<TimeZone> {
            let _t0: TimeZone = ZoneInfo::getTimeZone(Clone::clone(&ID))?;
            let mut tz: TimeZone = _t0;
            let _t1: TimeZone = TimeZone::parseCustomTimeZone(Clone::clone(&ID))?;
            tz = _t1;
            if fallback {
                let mut tz = ZoneInfo::new_str_i(Clone::clone(&String::from("GMT")), 0i32)?;
            }
            Ok(<_ as Into<TimeZone>>::into(tz))
        }

        #[java_method(name = "getAvailableIDs", descriptor = "(I)[Ljava/lang/String;", access = "public", modifiers = "static synchronized", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAvailableIDs_i(rawOffset: i32) -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: java/util/TimeZone.getAvailableIDs:(I)[Ljava/lang/String;")
        }

        #[java_method(name = "getAvailableIDs", descriptor = "()[Ljava/lang/String;", access = "public", modifiers = "static synchronized", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAvailableIDs() -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: java/util/TimeZone.getAvailableIDs:()[Ljava/lang/String;")
        }

        #[native]
        #[java_native(name = "getSystemTimeZoneID", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getSystemTimeZoneID(arg0: String) -> Result<String> {
            panic!("native: java/util/TimeZone.getSystemTimeZoneID:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[native]
        #[java_native(name = "getSystemGMTOffsetID", descriptor = "()Ljava/lang/String;", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getSystemGMTOffsetID() -> Result<String> {
            panic!("native: java/util/TimeZone.getSystemGMTOffsetID:()Ljava/lang/String;")
        }

        #[java_method(name = "getDefault", descriptor = "()Ljava/util/TimeZone;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDefault() -> Result<TimeZone> {
            let _t0: TimeZone = TimeZone::getDefaultRef()?;
            let _t1: Object = Object::from_any(_t0.clone());
            Ok((_t1).downcast::<TimeZone>())
        }

        #[java_method(name = "getDefaultRef", descriptor = "()Ljava/util/TimeZone;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDefaultRef() -> Result<TimeZone> {
            let mut defaultZone: TimeZone = TimeZone::defaultTimeZone();
            let _t0: TimeZone = TimeZone::setDefaultZone()?;
            defaultZone = _t0;
            if _is_jnull(&defaultZone) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(defaultZone)
        }

        #[java_method(name = "setDefaultZone", descriptor = "()Ljava/util/TimeZone;", access = "private", modifiers = "static synchronized", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDefaultZone() -> Result<TimeZone> {
            let _t0: Properties = GetPropertyAction::privilegedGetProperties()?;
            let mut props: Properties = _t0;
            let _t1 = props.getProperty_str(Clone::clone(&String::from("user.timezone")))?;
            let mut zoneID: String = _t1;
            let _t2 = zoneID.isEmpty()?;
            let _t3: String = StaticProperty::javaHome()?;
            let _t4: String = TimeZone::getSystemTimeZoneID(Clone::clone(&_t3))?;
            zoneID = _t4;
            if _is_jnull(&zoneID) {
                zoneID = String::from("GMT");
            }
            let _t5: TimeZone = TimeZone::getTimeZone_str_z(Clone::clone(&zoneID), (0i32 != 0i32))?;
            let mut tz: TimeZone = _t5;
            let _t6: String = TimeZone::getSystemGMTOffsetID()?;
            let mut gmtOffsetID: String = _t6;
            if !_is_jnull(&gmtOffsetID) {
                zoneID = gmtOffsetID;
            }
            let _t7: TimeZone = TimeZone::getTimeZone_str_z(Clone::clone(&zoneID), (1i32 != 0i32))?;
            tz = _t7;
            if _is_jnull(&tz) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            gmtOffsetID = zoneID;
            let _t8 = props.setProperty(Clone::clone(&String::from("user.timezone")), Clone::clone(&gmtOffsetID))?;
            TimeZone::set_defaultTimeZone(tz);
            Ok(tz)
        }

        #[java_method(name = "setDefault", descriptor = "(Ljava/util/TimeZone;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDefault(zone: TimeZone) -> Result<()> {
            panic!("stub: java/util/TimeZone.setDefault:(Ljava/util/TimeZone;)V")
        }

        #[java_method(name = "hasSameRules", descriptor = "(Ljava/util/TimeZone;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasSameRules(&self, other: TimeZone) -> Result<bool> {
            panic!("stub: java/util/TimeZone.hasSameRules:(Ljava/util/TimeZone;)Z")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/util/TimeZone.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "parseCustomTimeZone", descriptor = "(Ljava/lang/String;)Ljava/util/TimeZone;", access = "private", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parseCustomTimeZone(mut id: String) -> Result<TimeZone> {
            let _t0 = id.length()?;
            let mut length: i32 = _t0;
            let _t1 = id.indexOf_str(Clone::clone(&String::from("GMT")))?;
            if (_t1!=0) {
                return Ok(Default::default());
            }
            let _t2: ZoneInfo = ZoneInfoFile::getZoneInfo_str(Clone::clone(&id))?;
            let mut zi: ZoneInfo = _t2;
            if !_is_jnull(&zi) {
                return Ok(<_ as Into<TimeZone>>::into(zi));
            }
            let mut index: i32 = 3i32;
            let mut negative: i32 = 0i32;
            index = index.wrapping_add(1i32);
            let _t3 = id.charAt(index)?;
            let mut c: u16 = _t3;
            if (c as i32) == 45i32 {
                negative = 1i32;
            } else {
                if (c as i32) != 43i32 {
                    return Ok(Default::default());
                }
            }
            let mut hours: i32 = 0i32;
            let mut minutes: i32 = 0i32;
            let mut num: i32 = 0i32;
            let mut countDelim: i32 = 0i32;
            let mut len: i32 = 0i32;
            loop {
                if index >= length { break; }
                index = index.wrapping_add(1i32);
                let _t4 = id.charAt(index)?;
                c = _t4;
                if countDelim > 1i32 {
                    return Ok(Default::default());
                }
                if len > 2i32 {
                    return Ok(Default::default());
                }
                if (countDelim==0) {
                    hours = num;
                } else {
                    if countDelim == 1i32 {
                        minutes = num;
                    }
                }
                countDelim = countDelim.wrapping_add(1i32);
                num = 0i32;
                len = 0i32;
                continue;
                if (c as i32) > 57i32 {
                    return Ok(Default::default());
                }
                num = ((num).wrapping_mul(10i32)).wrapping_add(((c as i32)).wrapping_sub(48i32));
                len = len.wrapping_add(1i32);
            }
            if index != length {
                return Ok(Default::default());
            }
            if len <= 2i32 {
                hours = num;
                minutes = 0i32;
                num = 0i32;
            } else {
                if len <= 4i32 {
                    hours = (num/100i32);
                    minutes = (num%100i32);
                    num = 0i32;
                } else {
                    return Ok(Default::default());
                    if len == 2i32 {
                        minutes = num;
                        num = 0i32;
                    } else {
                        return Ok(Default::default());
                        if len != 2i32 {
                            return Ok(Default::default());
                        }
                    }
                }
            }
            if num > 59i32 {
                return Ok(Default::default());
            }
            let mut gmtOffset = ((((hours).wrapping_mul(3600i32)).wrapping_add((minutes).wrapping_mul(60i32))).wrapping_add(num)).wrapping_mul(1000i32);
            if (gmtOffset==0) {
                let _t4: ZoneInfo = ZoneInfoFile::getZoneInfo_str(Clone::clone(&String::from("GMT")))?;
                zi = _t4;
                if (negative!=0) {
                    zi.__super().setID(Clone::clone(&String::from("GMT-00:00")))?;
                } else {
                    zi.__super().setID(Clone::clone(&String::from("GMT+00:00")))?;
                }
            } else {
                let _t4: ZoneInfo = ZoneInfoFile::getCustomTimeZone(Clone::clone(&id), (if (negative!=0) { (gmtOffset).wrapping_neg() } else { gmtOffset }))?;
                zi = _t4;
            }
            Ok(<_ as Into<TimeZone>>::into(zi))
        }
    }
}
