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

impl From<BuddhistCalendar> for GregorianCalendar {
    fn from(v: BuddhistCalendar) -> GregorianCalendar { v.__into_super() }
}

impl From<BuddhistCalendar> for Calendar {
    fn from(v: BuddhistCalendar) -> Calendar { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "sun/util/BuddhistCalendar"]
    #[super_class       = "java/util/GregorianCalendar"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "BuddhistCalendar.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "GregorianCalendar"]
    #[superclass_fields(fields: Rc<RefCell<Vec<i32>>>, isSet: Rc<RefCell<Vec<bool>>>, stamp: Rc<RefCell<Vec<i32>>>, time: i64, isTimeSet: bool, areFieldsSet: bool, areAllFieldsSet: bool, lenient: bool, zone: TimeZone, sharedZone: bool, firstDayOfWeek: i32, minimalDaysInFirstWeek: i32, nextStamp: i32, serialVersionOnStream: i32, gregorianCutover: i64, gregorianCutoverDate: i64, gregorianCutoverYear: i32, gregorianCutoverYearJulian: i32, gdate: BaseCalendar_Date, cdate: BaseCalendar_Date, calsys: BaseCalendar, zoneOffsets: Rc<RefCell<Vec<i32>>>, originalFields: Rc<RefCell<Vec<i32>>>, cachedFixedDate: i64)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Comparable;java/lang/Object;java/util/Calendar;java/util/GregorianCalendar;sun/util/BuddhistCalendar"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct BuddhistCalendar {
        #[cfg_attr(any(), java_field(name = "yearOffset", descriptor = "I", access = "private", modifiers = "transient", is_static = false))]
        pub yearOffset: i32,
    }

    impl BuddhistCalendar {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-8527488697350388578"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -8527488697350388578i64
        }

        #[cfg_attr(any(), java_field(name = "BUDDHIST_YEAR_OFFSET", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "543"))]
        // static field: BUDDHIST_YEAR_OFFSET:I
        pub fn BUDDHIST_YEAR_OFFSET() -> i32 {
            543
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: sun/util/BuddhistCalendar.<init>:()V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/TimeZone;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_timezo(zone: TimeZone) -> Result<Self> {
            panic!("stub: sun/util/BuddhistCalendar.<init>:(Ljava/util/TimeZone;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Locale;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_locale(aLocale: Locale) -> Result<Self> {
            panic!("stub: sun/util/BuddhistCalendar.<init>:(Ljava/util/Locale;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/TimeZone;Ljava/util/Locale;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_timezo_locale(zone: TimeZone, aLocale: Locale) -> Result<Self> {
            panic!("stub: sun/util/BuddhistCalendar.<init>:(Ljava/util/TimeZone;Ljava/util/Locale;)V")
        }

        #[java_method(name = "getCalendarType", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarType(&self) -> Result<String> {
            panic!("stub: sun/util/BuddhistCalendar.getCalendarType:()Ljava/lang/String;")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: sun/util/BuddhistCalendar.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "get", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get(&self, field: i32) -> Result<i32> {
            panic!("stub: sun/util/BuddhistCalendar.get:(I)I")
        }

        #[java_method(name = "set", descriptor = "(II)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn set(&self, field: i32, value: i32) -> Result<()> {
            panic!("stub: sun/util/BuddhistCalendar.set:(II)V")
        }

        #[java_method(name = "add", descriptor = "(II)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn add(&self, field: i32, amount: i32) -> Result<()> {
            panic!("stub: sun/util/BuddhistCalendar.add:(II)V")
        }

        #[java_method(name = "roll", descriptor = "(II)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn roll(&self, field: i32, amount: i32) -> Result<()> {
            panic!("stub: sun/util/BuddhistCalendar.roll:(II)V")
        }

        #[java_method(name = "getDisplayName", descriptor = "(IILjava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayName(&self, field: i32, style: i32, locale: Locale) -> Result<String> {
            panic!("stub: sun/util/BuddhistCalendar.getDisplayName:(IILjava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayNames", descriptor = "(IILjava/util/Locale;)Ljava/util/Map;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(IILjava/util/Locale;)Ljava/util/Map<Ljava/lang/String;Ljava/lang/Integer;>;")]
        pub fn getDisplayNames(&self, field: i32, style: i32, locale: Locale) -> Result<Object> {
            panic!("stub: sun/util/BuddhistCalendar.getDisplayNames:(IILjava/util/Locale;)Ljava/util/Map;")
        }

        #[java_method(name = "getActualMaximum", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getActualMaximum(&self, field: i32) -> Result<i32> {
            panic!("stub: sun/util/BuddhistCalendar.getActualMaximum:(I)I")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, stream: Object) -> Result<()> {
            panic!("stub: sun/util/BuddhistCalendar.readObject:(Ljava/io/ObjectInputStream;)V")
        }
    }
}
