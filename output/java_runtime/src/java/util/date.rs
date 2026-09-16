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
    #[binary_name       = "java/util/Date"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Serializable,java/lang/Cloneable,java/lang/Comparable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = "Ljava/lang/Object;Ljava/io/Serializable;Ljava/lang/Cloneable;Ljava/lang/Comparable<Ljava/util/Date;>;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Date.java"]
    #[inner_classes     = "sun/util/calendar/BaseCalendar$Date:sun/util/calendar/BaseCalendar:Date:1033"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Comparable;java/lang/Object;java/util/Date"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct Date {
        #[cfg_attr(any(), java_field(name = "fastTime", descriptor = "J", access = "private", modifiers = "transient", is_static = false))]
        pub fastTime: i64,
        #[cfg_attr(any(), java_field(name = "cdate", descriptor = "Lsun/util/calendar/BaseCalendar$Date;", access = "private", modifiers = "transient", is_static = false))]
        pub cdate: BaseCalendar_Date,
    }

    impl Date {
        #[cfg_attr(any(), java_field(name = "gcal", descriptor = "Lsun/util/calendar/BaseCalendar;", access = "private", modifiers = "static final", is_static = true))]
        // static field: gcal:Lsun/util/calendar/BaseCalendar;
        pub fn gcal() -> BaseCalendar {
            panic!("stub: java/util/Date.gcal:Lsun/util/calendar/BaseCalendar;")
        }

        #[cfg_attr(any(), java_field(name = "jcal", descriptor = "Lsun/util/calendar/BaseCalendar;", access = "private", modifiers = "static", is_static = true))]
        // static field: jcal:Lsun/util/calendar/BaseCalendar;
        pub fn jcal() -> BaseCalendar {
            panic!("stub: java/util/Date.jcal:Lsun/util/calendar/BaseCalendar;")
        }

        #[cfg_attr(any(), java_field(name = "defaultCenturyStart", descriptor = "I", access = "private", modifiers = "static", is_static = true))]
        // static field: defaultCenturyStart:I
        pub fn defaultCenturyStart() -> i32 {
            panic!("stub: java/util/Date.defaultCenturyStart:I")
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "7523967970034938905"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            7523967970034938905i64
        }

        #[cfg_attr(any(), java_field(name = "wtb", descriptor = "[Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: wtb:[Ljava/lang/String;
        pub fn wtb() -> Rc<RefCell<Vec<String>>> {
            panic!("stub: java/util/Date.wtb:[Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "ttb", descriptor = "[I", access = "private", modifiers = "static final", is_static = true))]
        // static field: ttb:[I
        pub fn ttb() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/util/Date.ttb:[I")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            let _t0: i64 = System::currentTimeMillis()?;
            this = Date::new_l(_t0)?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(J)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(J)V
        pub fn new_l(mut date: i64) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_fastTime(date);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(III)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn new_i_i_i(year: i32, month: i32, date: i32) -> Result<Self> {
            panic!("stub: java/util/Date.<init>:(III)V")
        }

        #[java_method(name = "<init>", descriptor = "(IIIII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn new_i_i_i_i_i(year: i32, month: i32, date: i32, hrs: i32, min: i32) -> Result<Self> {
            panic!("stub: java/util/Date.<init>:(IIIII)V")
        }

        #[java_method(name = "<init>", descriptor = "(IIIIII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn new_i_i_i_i_i_i(year: i32, month: i32, date: i32, hrs: i32, min: i32, sec: i32) -> Result<Self> {
            panic!("stub: java/util/Date.<init>:(IIIIII)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn new_str(s: String) -> Result<Self> {
            panic!("stub: java/util/Date.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/util/Date.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "UTC", descriptor = "(IIIIII)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn UTC(year: i32, month: i32, date: i32, hrs: i32, min: i32, sec: i32) -> Result<i64> {
            panic!("stub: java/util/Date.UTC:(IIIIII)J")
        }

        #[java_method(name = "parse", descriptor = "(Ljava/lang/String;)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn parse(s: String) -> Result<i64> {
            panic!("stub: java/util/Date.parse:(Ljava/lang/String;)J")
        }

        #[java_method(name = "getYear", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getYear(&self) -> Result<i32> {
            panic!("stub: java/util/Date.getYear:()I")
        }

        #[java_method(name = "setYear", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn setYear(&self, year: i32) -> Result<()> {
            panic!("stub: java/util/Date.setYear:(I)V")
        }

        #[java_method(name = "getMonth", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getMonth(&self) -> Result<i32> {
            panic!("stub: java/util/Date.getMonth:()I")
        }

        #[java_method(name = "setMonth", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn setMonth(&self, month: i32) -> Result<()> {
            panic!("stub: java/util/Date.setMonth:(I)V")
        }

        #[java_method(name = "getDate", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getDate(&self) -> Result<i32> {
            panic!("stub: java/util/Date.getDate:()I")
        }

        #[java_method(name = "setDate", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn setDate(&self, date: i32) -> Result<()> {
            panic!("stub: java/util/Date.setDate:(I)V")
        }

        #[java_method(name = "getDay", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getDay(&self) -> Result<i32> {
            panic!("stub: java/util/Date.getDay:()I")
        }

        #[java_method(name = "getHours", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getHours(&self) -> Result<i32> {
            panic!("stub: java/util/Date.getHours:()I")
        }

        #[java_method(name = "setHours", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn setHours(&self, hours: i32) -> Result<()> {
            panic!("stub: java/util/Date.setHours:(I)V")
        }

        #[java_method(name = "getMinutes", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getMinutes(&self) -> Result<i32> {
            panic!("stub: java/util/Date.getMinutes:()I")
        }

        #[java_method(name = "setMinutes", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn setMinutes(&self, minutes: i32) -> Result<()> {
            panic!("stub: java/util/Date.setMinutes:(I)V")
        }

        #[java_method(name = "getSeconds", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getSeconds(&self) -> Result<i32> {
            panic!("stub: java/util/Date.getSeconds:()I")
        }

        #[java_method(name = "setSeconds", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn setSeconds(&self, seconds: i32) -> Result<()> {
            panic!("stub: java/util/Date.setSeconds:(I)V")
        }

        #[java_method(name = "getTime", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTime(&self) -> Result<i64> {
            let this = self;
            let _t0 = this.getTimeImpl()?;
            Ok(_t0)
        }

        #[java_method(name = "getTimeImpl", descriptor = "()J", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTimeImpl(&self) -> Result<i64> {
            let this = self;
            let _t0 = this.__get_cdate().__super().isNormalized()?;
            if !(_t0) {
                let _t1 = this.normalize()?;
            }
            Ok(this.__get_fastTime())
        }

        #[java_method(name = "setTime", descriptor = "(J)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setTime(&self, time: i64) -> Result<()> {
            panic!("stub: java/util/Date.setTime:(J)V")
        }

        #[java_method(name = "before", descriptor = "(Ljava/util/Date;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn before(&self, when: Date) -> Result<bool> {
            panic!("stub: java/util/Date.before:(Ljava/util/Date;)Z")
        }

        #[java_method(name = "after", descriptor = "(Ljava/util/Date;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn after(&self, when: Date) -> Result<bool> {
            panic!("stub: java/util/Date.after:(Ljava/util/Date;)Z")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/util/Date.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "getMillisOf", descriptor = "(Ljava/util/Date;)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMillisOf(date: Date) -> Result<i64> {
            panic!("stub: java/util/Date.getMillisOf:(Ljava/util/Date;)J")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/util/Date;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, anotherDate: Date) -> Result<i32> {
            panic!("stub: java/util/Date.compareTo:(Ljava/util/Date;)I")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "convertToAbbr", descriptor = "(Ljava/lang/StringBuilder;Ljava/lang/String;)Ljava/lang/StringBuilder;", access = "private", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn convertToAbbr(sb: StringBuilder, name: String) -> Result<StringBuilder> {
            panic!("stub: java/util/Date.convertToAbbr:(Ljava/lang/StringBuilder;Ljava/lang/String;)Ljava/lang/StringBuilder;")
        }

        #[java_method(name = "toLocaleString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn toLocaleString(&self) -> Result<String> {
            panic!("stub: java/util/Date.toLocaleString:()Ljava/lang/String;")
        }

        #[java_method(name = "toGMTString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn toGMTString(&self) -> Result<String> {
            panic!("stub: java/util/Date.toGMTString:()Ljava/lang/String;")
        }

        #[java_method(name = "getTimezoneOffset", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getTimezoneOffset(&self) -> Result<i32> {
            panic!("stub: java/util/Date.getTimezoneOffset:()I")
        }

        #[java_method(name = "getCalendarDate", descriptor = "()Lsun/util/calendar/BaseCalendar$Date;", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarDate(&self) -> Result<BaseCalendar_Date> {
            panic!("stub: java/util/Date.getCalendarDate:()Lsun/util/calendar/BaseCalendar$Date;")
        }

        #[java_method(name = "normalize", descriptor = "()Lsun/util/calendar/BaseCalendar$Date;", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: normalize()Lsun/util/calendar/BaseCalendar$Date;
        pub fn normalize(&self) -> Result<BaseCalendar_Date> {
            let this = self;
            if _is_jnull(&this.__get_cdate()) {
                let _t0: BaseCalendar = Date::getCalendarSystem_l(this.__get_fastTime())?;
                let mut cal: BaseCalendar = _t0;
                let _t1: TimeZone = TimeZone::getDefaultRef()?;
                let _t2 = cal.__super().getCalendarDate(this.__get_fastTime(), Clone::clone(&_t1))?;
                this.__set_cdate(Default::default());
                return Ok(this.__get_cdate());
            }
            let _t0 = this.__get_cdate().__super().isNormalized()?;
            if !(_t0) {
                let _t1 = this.normalize_baseca(Clone::clone(&this.__get_cdate()))?;
                this.__set_cdate(Clone::clone(&_t1));
            }
            let _t1: TimeZone = TimeZone::getDefaultRef()?;
            let mut cal: TimeZone = _t1;
            let _t2 = this.__get_cdate().__super().getZone()?;
            if Object::from_any(cal.clone()) != Object::from_any(_t2.clone()) {
                let _t3 = this.__get_cdate().__super().setZone(Clone::clone(&cal))?;
                let _t4: BaseCalendar = Date::getCalendarSystem_baseca(Clone::clone(&this.__get_cdate()))?;
                let mut cal: BaseCalendar = _t4;
                let _t5 = cal.__super().getCalendarDate(this.__get_fastTime(), Clone::clone(&this.__get_cdate()).into())?;
            }
            Ok(this.__get_cdate())
        }

        #[java_method(name = "normalize", descriptor = "(Lsun/util/calendar/BaseCalendar$Date;)Lsun/util/calendar/BaseCalendar$Date;", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: normalize(Lsun/util/calendar/BaseCalendar$Date;)Lsun/util/calendar/BaseCalendar$Date;
        pub fn normalize_baseca(&self, mut date: BaseCalendar_Date) -> Result<BaseCalendar_Date> {
            let this = self;
            let _t0 = date.getNormalizedYear()?;
            let mut y: i32 = _t0;
            let _t1 = date.__super().getMonth()?;
            let mut m: i32 = _t1;
            let _t2 = date.__super().getDayOfMonth()?;
            let mut d: i32 = _t2;
            let _t3 = date.__super().getHours()?;
            let mut hh: i32 = _t3;
            let _t4 = date.__super().getMinutes()?;
            let mut mm: i32 = _t4;
            let _t5 = date.__super().getSeconds()?;
            let mut ss: i32 = _t5;
            let _t6 = date.__super().getMillis()?;
            let mut ms: i32 = _t6;
            let _t7 = date.__super().getZone()?;
            let mut tz: TimeZone = _t7;
            if _is_jnull(&tz) {
                let _t8: TimeZone = TimeZone::getTimeZone_str(Clone::clone(&String::from("GMT")))?;
                tz = _t8;
            }
            let mut gc = GregorianCalendar::new_timezo(Clone::clone(&tz))?;
            gc.__super().clear()?;
            gc.__super().set(14i32, ms)?;
            gc.__super().set(y, (m).wrapping_sub(1i32), d, hh, mm, ss)?;
            let _t8 = gc.__super().getTimeInMillis()?;
            this.__set_fastTime(_t8);
            let _t9: BaseCalendar = Date::getCalendarSystem_l(this.__get_fastTime())?;
            let mut cal: BaseCalendar = _t9;
            let _t10 = cal.__super().getCalendarDate(this.__get_fastTime(), Clone::clone(&tz))?;
            date = Default::default();
            return Ok(date);
            let _t11: BaseCalendar = Date::getCalendarSystem_i(y)?;
            let mut gc: BaseCalendar = _t11;
            let _t12: BaseCalendar = Date::getCalendarSystem_baseca(Clone::clone(&date))?;
            if Object::from_any(gc.clone()) != Object::from_any(_t12.clone()) {
                let _t13 = gc.__super().__super().newCalendarDate(Clone::clone(&tz))?;
                date = Default::default();
                let _t14 = date.setNormalizedDate(y, m, d)?;
                let _t15 = _t14.__super().setTimeOfDay(hh, mm, ss, ms)?;
            }
            let _t13 = gc.__super().getTime(Clone::clone(&date).into())?;
            this.__set_fastTime(_t13);
            let _t14: BaseCalendar = Date::getCalendarSystem_l(this.__get_fastTime())?;
            cal = _t14;
            if Object::from_any(cal.clone()) != Object::from_any(gc.clone()) {
                let _t15 = cal.__super().__super().newCalendarDate(Clone::clone(&tz))?;
                date = Default::default();
                let _t16 = date.setNormalizedDate(y, m, d)?;
                let _t17 = _t16.__super().setTimeOfDay(hh, mm, ss, ms)?;
                let _t18 = cal.__super().getTime(Clone::clone(&date).into())?;
                this.__set_fastTime(_t18);
            }
            Ok(date)
        }

        #[java_method(name = "getCalendarSystem", descriptor = "(I)Lsun/util/calendar/BaseCalendar;", access = "private", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getCalendarSystem(I)Lsun/util/calendar/BaseCalendar;
        pub fn getCalendarSystem_i(mut year: i32) -> Result<BaseCalendar> {
            if year >= 1582i32 {
                return Ok(Date::gcal());
            }
            let _t0: BaseCalendar = Date::getJulianCalendar()?;
            Ok(_t0)
        }

        #[java_method(name = "getCalendarSystem", descriptor = "(J)Lsun/util/calendar/BaseCalendar;", access = "private", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getCalendarSystem(J)Lsun/util/calendar/BaseCalendar;
        pub fn getCalendarSystem_l(mut utc: i64) -> Result<BaseCalendar> {
            let _t0: TimeZone = TimeZone::getDefaultRef()?;
            let _t1 = _t0.getOffset_l(utc)?;
            if (((utc>((-12219292800000i64).wrapping_sub((_t1 as i64)))) as i32-((utc)<((-12219292800000i64).wrapping_sub((_t1 as i64)))) as i32)>=0) {
                return Ok(Date::gcal());
            }
            let _t2: BaseCalendar = Date::getJulianCalendar()?;
            Ok(_t2)
        }

        #[java_method(name = "getCalendarSystem", descriptor = "(Lsun/util/calendar/BaseCalendar$Date;)Lsun/util/calendar/BaseCalendar;", access = "private", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getCalendarSystem(Lsun/util/calendar/BaseCalendar$Date;)Lsun/util/calendar/BaseCalendar;
        pub fn getCalendarSystem_baseca(mut cdate: BaseCalendar_Date) -> Result<BaseCalendar> {
            if _is_jnull(&Date::jcal()) {
                return Ok(Date::gcal());
            }
            let _t0 = cdate.__super().getEra()?;
            if !_is_jnull(&_t0) {
                return Ok(Date::jcal());
            }
            Ok(Date::gcal())
        }

        #[java_method(name = "getJulianCalendar", descriptor = "()Lsun/util/calendar/BaseCalendar;", access = "private", modifiers = "static final synchronized", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getJulianCalendar() -> Result<BaseCalendar> {
            if _is_jnull(&Date::jcal()) {
                let _t0: CalendarSystem = CalendarSystem::forName(Clone::clone(&String::from("julian")))?;
                Date::set_jcal(Default::default());
            }
            Ok(Date::jcal())
        }

        #[java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/Date.writeObject:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/util/Date.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "from", descriptor = "(Ljava/time/Instant;)Ljava/util/Date;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn from(instant: Instant) -> Result<Date> {
            panic!("stub: java/util/Date.from:(Ljava/time/Instant;)Ljava/util/Date;")
        }

        #[java_method(name = "toInstant", descriptor = "()Ljava/time/Instant;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toInstant(&self) -> Result<Instant> {
            panic!("stub: java/util/Date.toInstant:()Ljava/time/Instant;")
        }
    }
}
