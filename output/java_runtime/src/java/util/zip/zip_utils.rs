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
use crate::jdk::internal::misc::Unsafe;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/zip/ZipUtils"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ZipUtils.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/zip/ZipUtils"]

    pub struct ZipUtils;

    impl ZipUtils {
        #[cfg_attr(any(), java_field(name = "NIO_ACCESS", descriptor = "Ljdk/internal/access/JavaNioAccess;", access = "package", modifiers = "static final", is_static = true))]
        // static field: NIO_ACCESS:Ljdk/internal/access/JavaNioAccess;
        pub fn NIO_ACCESS() -> Object {
            panic!("stub: java/util/zip/ZipUtils.NIO_ACCESS:Ljdk/internal/access/JavaNioAccess;")
        }

        #[cfg_attr(any(), java_field(name = "WINDOWS_EPOCH_IN_MICROSECONDS", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-11644473600000000"))]
        // static field: WINDOWS_EPOCH_IN_MICROSECONDS:J
        pub fn WINDOWS_EPOCH_IN_MICROSECONDS() -> i64 {
            -11644473600000000i64
        }

        #[cfg_attr(any(), java_field(name = "WINDOWS_TIME_NOT_AVAILABLE", descriptor = "J", access = "public", modifiers = "static final", is_static = true, constant_value = "-9223372036854775808"))]
        // static field: WINDOWS_TIME_NOT_AVAILABLE:J
        pub fn WINDOWS_TIME_NOT_AVAILABLE() -> i64 {
            -9223372036854775808i64
        }

        #[cfg_attr(any(), java_field(name = "defaultBuf", descriptor = "Ljava/nio/ByteBuffer;", access = "package", modifiers = "static final", is_static = true))]
        // static field: defaultBuf:Ljava/nio/ByteBuffer;
        pub fn defaultBuf() -> ByteBuffer {
            panic!("stub: java/util/zip/ZipUtils.defaultBuf:Ljava/nio/ByteBuffer;")
        }

        #[cfg_attr(any(), java_field(name = "UPPER_UNIXTIME_BOUND", descriptor = "J", access = "public", modifiers = "static final", is_static = true, constant_value = "2147483647"))]
        // static field: UPPER_UNIXTIME_BOUND:J
        pub fn UPPER_UNIXTIME_BOUND() -> i64 {
            2147483647i64
        }

        #[cfg_attr(any(), java_field(name = "FILE_ATTRIBUTES_UNIX", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: FILE_ATTRIBUTES_UNIX:I
        pub fn FILE_ATTRIBUTES_UNIX() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "VERSION_MADE_BY_BASE_UNIX", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "768"))]
        // static field: VERSION_MADE_BY_BASE_UNIX:I
        pub fn VERSION_MADE_BY_BASE_UNIX() -> i32 {
            768
        }

        #[cfg_attr(any(), java_field(name = "END_MAXLEN", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "65557"))]
        // static field: END_MAXLEN:J
        pub fn END_MAXLEN() -> i64 {
            65557i64
        }

        #[cfg_attr(any(), java_field(name = "READBLOCKSZ", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "128"))]
        // static field: READBLOCKSZ:I
        pub fn READBLOCKSZ() -> i32 {
            128
        }

        #[cfg_attr(any(), java_field(name = "unsafe", descriptor = "Ljdk/internal/misc/Unsafe;", access = "private", modifiers = "static final", is_static = true))]
        // static field: unsafe:Ljdk/internal/misc/Unsafe;
        pub fn unsafe_() -> Unsafe {
            panic!("stub: java/util/zip/ZipUtils.unsafe:Ljdk/internal/misc/Unsafe;")
        }

        #[cfg_attr(any(), java_field(name = "byteBufferArrayOffset", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: byteBufferArrayOffset:J
        pub fn byteBufferArrayOffset() -> i64 {
            panic!("stub: java/util/zip/ZipUtils.byteBufferArrayOffset:J")
        }

        #[cfg_attr(any(), java_field(name = "byteBufferOffsetOffset", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: byteBufferOffsetOffset:J
        pub fn byteBufferOffsetOffset() -> i64 {
            panic!("stub: java/util/zip/ZipUtils.byteBufferOffsetOffset:J")
        }

        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/zip/ZipUtils.<init>:()V")
        }

        #[java_method(name = "winTimeToFileTime", descriptor = "(J)Ljava/nio/file/attribute/FileTime;", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn winTimeToFileTime(wtime: i64) -> Result<Object> {
            panic!("stub: java/util/zip/ZipUtils.winTimeToFileTime:(J)Ljava/nio/file/attribute/FileTime;")
        }

        #[java_method(name = "fileTimeToWinTime", descriptor = "(Ljava/nio/file/attribute/FileTime;)J", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fileTimeToWinTime(ftime: Object) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.fileTimeToWinTime:(Ljava/nio/file/attribute/FileTime;)J")
        }

        #[java_method(name = "unixTimeToFileTime", descriptor = "(J)Ljava/nio/file/attribute/FileTime;", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unixTimeToFileTime(utime: i64) -> Result<Object> {
            panic!("stub: java/util/zip/ZipUtils.unixTimeToFileTime:(J)Ljava/nio/file/attribute/FileTime;")
        }

        #[java_method(name = "fileTimeToUnixTime", descriptor = "(Ljava/nio/file/attribute/FileTime;)J", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fileTimeToUnixTime(ftime: Object) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.fileTimeToUnixTime:(Ljava/nio/file/attribute/FileTime;)J")
        }

        #[java_method(name = "dosToJavaTime", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn dosToJavaTime(dtime: i64) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.dosToJavaTime:(J)J")
        }

        #[java_method(name = "overflowDosToJavaTime", descriptor = "(IIIIII)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn overflowDosToJavaTime(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.overflowDosToJavaTime:(IIIIII)J")
        }

        #[java_method(name = "extendedDosToJavaTime", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn extendedDosToJavaTime(xdostime: i64) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.extendedDosToJavaTime:(J)J")
        }

        #[java_method(name = "javaToDosTime", descriptor = "(Ljava/time/LocalDateTime;)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn javaToDosTime(ldt: LocalDateTime) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.javaToDosTime:(Ljava/time/LocalDateTime;)J")
        }

        #[java_method(name = "javaToExtendedDosTime", descriptor = "(J)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn javaToExtendedDosTime(time: i64) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.javaToExtendedDosTime:(J)J")
        }

        #[java_method(name = "javaEpochToLocalDateTime", descriptor = "(J)Ljava/time/LocalDateTime;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn javaEpochToLocalDateTime(time: i64) -> Result<LocalDateTime> {
            panic!("stub: java/util/zip/ZipUtils.javaEpochToLocalDateTime:(J)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "get16", descriptor = "([BI)I", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get16(b: Rc<RefCell<Vec<i8>>>, off: i32) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.get16:([BI)I")
        }

        #[java_method(name = "get32", descriptor = "([BI)J", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get32(b: Rc<RefCell<Vec<i8>>>, off: i32) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.get32:([BI)J")
        }

        #[java_method(name = "get64", descriptor = "([BI)J", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get64(b: Rc<RefCell<Vec<i8>>>, off: i32) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.get64:([BI)J")
        }

        #[java_method(name = "get32S", descriptor = "([BI)I", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get32S(b: Rc<RefCell<Vec<i8>>>, off: i32) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.get32S:([BI)I")
        }

        #[java_method(name = "CH", descriptor = "([BI)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CH(b: Rc<RefCell<Vec<i8>>>, n: i32) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.CH:([BI)I")
        }

        #[java_method(name = "SH", descriptor = "([BI)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn SH(b: Rc<RefCell<Vec<i8>>>, n: i32) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.SH:([BI)I")
        }

        #[java_method(name = "LG", descriptor = "([BI)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn LG(b: Rc<RefCell<Vec<i8>>>, n: i32) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.LG:([BI)J")
        }

        #[java_method(name = "LL", descriptor = "([BI)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn LL(b: Rc<RefCell<Vec<i8>>>, n: i32) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.LL:([BI)J")
        }

        #[java_method(name = "GETSIG", descriptor = "([B)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn GETSIG(b: Rc<RefCell<Vec<i8>>>) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.GETSIG:([B)J")
        }

        #[java_method(name = "LOCSIG", descriptor = "([B)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn LOCSIG(b: Rc<RefCell<Vec<i8>>>) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.LOCSIG:([B)J")
        }

        #[java_method(name = "LOCVER", descriptor = "([B)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn LOCVER(b: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.LOCVER:([B)I")
        }

        #[java_method(name = "LOCFLG", descriptor = "([B)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn LOCFLG(b: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.LOCFLG:([B)I")
        }

        #[java_method(name = "LOCHOW", descriptor = "([B)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn LOCHOW(b: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.LOCHOW:([B)I")
        }

        #[java_method(name = "LOCTIM", descriptor = "([B)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn LOCTIM(b: Rc<RefCell<Vec<i8>>>) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.LOCTIM:([B)J")
        }

        #[java_method(name = "LOCCRC", descriptor = "([B)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn LOCCRC(b: Rc<RefCell<Vec<i8>>>) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.LOCCRC:([B)J")
        }

        #[java_method(name = "LOCSIZ", descriptor = "([B)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn LOCSIZ(b: Rc<RefCell<Vec<i8>>>) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.LOCSIZ:([B)J")
        }

        #[java_method(name = "LOCLEN", descriptor = "([B)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn LOCLEN(b: Rc<RefCell<Vec<i8>>>) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.LOCLEN:([B)J")
        }

        #[java_method(name = "LOCNAM", descriptor = "([B)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn LOCNAM(b: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.LOCNAM:([B)I")
        }

        #[java_method(name = "LOCEXT", descriptor = "([B)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn LOCEXT(b: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.LOCEXT:([B)I")
        }

        #[java_method(name = "EXTCRC", descriptor = "([B)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn EXTCRC(b: Rc<RefCell<Vec<i8>>>) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.EXTCRC:([B)J")
        }

        #[java_method(name = "EXTSIZ", descriptor = "([B)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn EXTSIZ(b: Rc<RefCell<Vec<i8>>>) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.EXTSIZ:([B)J")
        }

        #[java_method(name = "EXTLEN", descriptor = "([B)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn EXTLEN(b: Rc<RefCell<Vec<i8>>>) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.EXTLEN:([B)J")
        }

        #[java_method(name = "ENDSUB", descriptor = "([B)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ENDSUB(b: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.ENDSUB:([B)I")
        }

        #[java_method(name = "ENDTOT", descriptor = "([B)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ENDTOT(b: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.ENDTOT:([B)I")
        }

        #[java_method(name = "ENDSIZ", descriptor = "([B)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ENDSIZ(b: Rc<RefCell<Vec<i8>>>) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.ENDSIZ:([B)J")
        }

        #[java_method(name = "ENDOFF", descriptor = "([B)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ENDOFF(b: Rc<RefCell<Vec<i8>>>) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.ENDOFF:([B)J")
        }

        #[java_method(name = "ENDCOM", descriptor = "([B)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ENDCOM_arr_b(b: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.ENDCOM:([B)I")
        }

        #[java_method(name = "ENDCOM", descriptor = "([BI)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ENDCOM_arr_b_i(b: Rc<RefCell<Vec<i8>>>, off: i32) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.ENDCOM:([BI)I")
        }

        #[java_method(name = "ZIP64_ENDTOD", descriptor = "([B)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ZIP64_ENDTOD(b: Rc<RefCell<Vec<i8>>>) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.ZIP64_ENDTOD:([B)J")
        }

        #[java_method(name = "ZIP64_ENDTOT", descriptor = "([B)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ZIP64_ENDTOT(b: Rc<RefCell<Vec<i8>>>) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.ZIP64_ENDTOT:([B)J")
        }

        #[java_method(name = "ZIP64_ENDSIZ", descriptor = "([B)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ZIP64_ENDSIZ(b: Rc<RefCell<Vec<i8>>>) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.ZIP64_ENDSIZ:([B)J")
        }

        #[java_method(name = "ZIP64_ENDOFF", descriptor = "([B)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ZIP64_ENDOFF(b: Rc<RefCell<Vec<i8>>>) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.ZIP64_ENDOFF:([B)J")
        }

        #[java_method(name = "ZIP64_LOCOFF", descriptor = "([B)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ZIP64_LOCOFF(b: Rc<RefCell<Vec<i8>>>) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.ZIP64_LOCOFF:([B)J")
        }

        #[java_method(name = "CENSIG", descriptor = "([BI)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CENSIG(b: Rc<RefCell<Vec<i8>>>, pos: i32) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.CENSIG:([BI)J")
        }

        #[java_method(name = "CENVEM", descriptor = "([BI)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CENVEM(b: Rc<RefCell<Vec<i8>>>, pos: i32) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.CENVEM:([BI)I")
        }

        #[java_method(name = "CENVEM_FA", descriptor = "([BI)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CENVEM_FA(b: Rc<RefCell<Vec<i8>>>, pos: i32) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.CENVEM_FA:([BI)I")
        }

        #[java_method(name = "CENVER", descriptor = "([BI)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CENVER(b: Rc<RefCell<Vec<i8>>>, pos: i32) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.CENVER:([BI)I")
        }

        #[java_method(name = "CENFLG", descriptor = "([BI)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CENFLG(b: Rc<RefCell<Vec<i8>>>, pos: i32) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.CENFLG:([BI)I")
        }

        #[java_method(name = "CENHOW", descriptor = "([BI)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CENHOW(b: Rc<RefCell<Vec<i8>>>, pos: i32) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.CENHOW:([BI)I")
        }

        #[java_method(name = "CENTIM", descriptor = "([BI)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CENTIM(b: Rc<RefCell<Vec<i8>>>, pos: i32) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.CENTIM:([BI)J")
        }

        #[java_method(name = "CENCRC", descriptor = "([BI)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CENCRC(b: Rc<RefCell<Vec<i8>>>, pos: i32) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.CENCRC:([BI)J")
        }

        #[java_method(name = "CENSIZ", descriptor = "([BI)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CENSIZ(b: Rc<RefCell<Vec<i8>>>, pos: i32) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.CENSIZ:([BI)J")
        }

        #[java_method(name = "CENLEN", descriptor = "([BI)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CENLEN(b: Rc<RefCell<Vec<i8>>>, pos: i32) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.CENLEN:([BI)J")
        }

        #[java_method(name = "CENNAM", descriptor = "([BI)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CENNAM(b: Rc<RefCell<Vec<i8>>>, pos: i32) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.CENNAM:([BI)I")
        }

        #[java_method(name = "CENEXT", descriptor = "([BI)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CENEXT(b: Rc<RefCell<Vec<i8>>>, pos: i32) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.CENEXT:([BI)I")
        }

        #[java_method(name = "CENCOM", descriptor = "([BI)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CENCOM(b: Rc<RefCell<Vec<i8>>>, pos: i32) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.CENCOM:([BI)I")
        }

        #[java_method(name = "CENDSK", descriptor = "([BI)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CENDSK(b: Rc<RefCell<Vec<i8>>>, pos: i32) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.CENDSK:([BI)I")
        }

        #[java_method(name = "CENATT", descriptor = "([BI)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CENATT(b: Rc<RefCell<Vec<i8>>>, pos: i32) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.CENATT:([BI)I")
        }

        #[java_method(name = "CENATX", descriptor = "([BI)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CENATX(b: Rc<RefCell<Vec<i8>>>, pos: i32) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.CENATX:([BI)J")
        }

        #[java_method(name = "CENATX_PERMS", descriptor = "([BI)I", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CENATX_PERMS(b: Rc<RefCell<Vec<i8>>>, pos: i32) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.CENATX_PERMS:([BI)I")
        }

        #[java_method(name = "CENOFF", descriptor = "([BI)J", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn CENOFF(b: Rc<RefCell<Vec<i8>>>, pos: i32) -> Result<i64> {
            panic!("stub: java/util/zip/ZipUtils.CENOFF:([BI)J")
        }

        #[java_method(name = "loadLibrary", descriptor = "()V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn loadLibrary() -> Result<()> {
            panic!("stub: java/util/zip/ZipUtils.loadLibrary:()V")
        }

        #[java_method(name = "getBufferArray", descriptor = "(Ljava/nio/ByteBuffer;)[B", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBufferArray(byteBuffer: ByteBuffer) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/util/zip/ZipUtils.getBufferArray:(Ljava/nio/ByteBuffer;)[B")
        }

        #[java_method(name = "getBufferOffset", descriptor = "(Ljava/nio/ByteBuffer;)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBufferOffset(byteBuffer: ByteBuffer) -> Result<i32> {
            panic!("stub: java/util/zip/ZipUtils.getBufferOffset:(Ljava/nio/ByteBuffer;)I")
        }
    }
}
