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
use crate::jdk::internal::util::*;
use crate::java::text::Normalizer;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "jdk/internal/util/StaticProperty"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "StaticProperty.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;jdk/internal/util/StaticProperty"]

    pub struct StaticProperty;

    impl StaticProperty {
        #[cfg_attr(any(), java_field(name = "JAVA_HOME", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: JAVA_HOME:Ljava/lang/String;
        pub fn JAVA_HOME() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.JAVA_HOME:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "USER_HOME", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: USER_HOME:Ljava/lang/String;
        pub fn USER_HOME() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.USER_HOME:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "USER_DIR", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: USER_DIR:Ljava/lang/String;
        pub fn USER_DIR() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.USER_DIR:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "USER_NAME", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: USER_NAME:Ljava/lang/String;
        pub fn USER_NAME() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.USER_NAME:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "JAVA_LIBRARY_PATH", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: JAVA_LIBRARY_PATH:Ljava/lang/String;
        pub fn JAVA_LIBRARY_PATH() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.JAVA_LIBRARY_PATH:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "SUN_BOOT_LIBRARY_PATH", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: SUN_BOOT_LIBRARY_PATH:Ljava/lang/String;
        pub fn SUN_BOOT_LIBRARY_PATH() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.SUN_BOOT_LIBRARY_PATH:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "JDK_SERIAL_FILTER", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: JDK_SERIAL_FILTER:Ljava/lang/String;
        pub fn JDK_SERIAL_FILTER() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.JDK_SERIAL_FILTER:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "JDK_SERIAL_FILTER_FACTORY", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: JDK_SERIAL_FILTER_FACTORY:Ljava/lang/String;
        pub fn JDK_SERIAL_FILTER_FACTORY() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.JDK_SERIAL_FILTER_FACTORY:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "JAVA_IO_TMPDIR", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: JAVA_IO_TMPDIR:Ljava/lang/String;
        pub fn JAVA_IO_TMPDIR() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.JAVA_IO_TMPDIR:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "NATIVE_ENCODING", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: NATIVE_ENCODING:Ljava/lang/String;
        pub fn NATIVE_ENCODING() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.NATIVE_ENCODING:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "FILE_ENCODING", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: FILE_ENCODING:Ljava/lang/String;
        pub fn FILE_ENCODING() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.FILE_ENCODING:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "JAVA_PROPERTIES_DATE", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: JAVA_PROPERTIES_DATE:Ljava/lang/String;
        pub fn JAVA_PROPERTIES_DATE() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.JAVA_PROPERTIES_DATE:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "SUN_JNU_ENCODING", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: SUN_JNU_ENCODING:Ljava/lang/String;
        pub fn SUN_JNU_ENCODING() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.SUN_JNU_ENCODING:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "JAVA_LOCALE_USE_OLD_ISO_CODES", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: JAVA_LOCALE_USE_OLD_ISO_CODES:Ljava/lang/String;
        pub fn JAVA_LOCALE_USE_OLD_ISO_CODES() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.JAVA_LOCALE_USE_OLD_ISO_CODES:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "OS_NAME", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: OS_NAME:Ljava/lang/String;
        pub fn OS_NAME() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.OS_NAME:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "OS_ARCH", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: OS_ARCH:Ljava/lang/String;
        pub fn OS_ARCH() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.OS_ARCH:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "OS_VERSION", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: OS_VERSION:Ljava/lang/String;
        pub fn OS_VERSION() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.OS_VERSION:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "USER_LANGUAGE", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true))]
        // static field: USER_LANGUAGE:Ljava/lang/String;
        pub fn USER_LANGUAGE() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.USER_LANGUAGE:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "USER_LANGUAGE_DISPLAY", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true))]
        // static field: USER_LANGUAGE_DISPLAY:Ljava/lang/String;
        pub fn USER_LANGUAGE_DISPLAY() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.USER_LANGUAGE_DISPLAY:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "USER_LANGUAGE_FORMAT", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true))]
        // static field: USER_LANGUAGE_FORMAT:Ljava/lang/String;
        pub fn USER_LANGUAGE_FORMAT() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.USER_LANGUAGE_FORMAT:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "USER_SCRIPT", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true))]
        // static field: USER_SCRIPT:Ljava/lang/String;
        pub fn USER_SCRIPT() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.USER_SCRIPT:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "USER_SCRIPT_DISPLAY", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true))]
        // static field: USER_SCRIPT_DISPLAY:Ljava/lang/String;
        pub fn USER_SCRIPT_DISPLAY() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.USER_SCRIPT_DISPLAY:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "USER_SCRIPT_FORMAT", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true))]
        // static field: USER_SCRIPT_FORMAT:Ljava/lang/String;
        pub fn USER_SCRIPT_FORMAT() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.USER_SCRIPT_FORMAT:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "USER_COUNTRY", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true))]
        // static field: USER_COUNTRY:Ljava/lang/String;
        pub fn USER_COUNTRY() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.USER_COUNTRY:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "USER_COUNTRY_DISPLAY", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true))]
        // static field: USER_COUNTRY_DISPLAY:Ljava/lang/String;
        pub fn USER_COUNTRY_DISPLAY() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.USER_COUNTRY_DISPLAY:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "USER_COUNTRY_FORMAT", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true))]
        // static field: USER_COUNTRY_FORMAT:Ljava/lang/String;
        pub fn USER_COUNTRY_FORMAT() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.USER_COUNTRY_FORMAT:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "USER_VARIANT", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true))]
        // static field: USER_VARIANT:Ljava/lang/String;
        pub fn USER_VARIANT() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.USER_VARIANT:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "USER_VARIANT_DISPLAY", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true))]
        // static field: USER_VARIANT_DISPLAY:Ljava/lang/String;
        pub fn USER_VARIANT_DISPLAY() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.USER_VARIANT_DISPLAY:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "USER_VARIANT_FORMAT", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true))]
        // static field: USER_VARIANT_FORMAT:Ljava/lang/String;
        pub fn USER_VARIANT_FORMAT() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.USER_VARIANT_FORMAT:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "USER_EXTENSIONS", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true))]
        // static field: USER_EXTENSIONS:Ljava/lang/String;
        pub fn USER_EXTENSIONS() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.USER_EXTENSIONS:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "USER_EXTENSIONS_DISPLAY", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true))]
        // static field: USER_EXTENSIONS_DISPLAY:Ljava/lang/String;
        pub fn USER_EXTENSIONS_DISPLAY() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.USER_EXTENSIONS_DISPLAY:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "USER_EXTENSIONS_FORMAT", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true))]
        // static field: USER_EXTENSIONS_FORMAT:Ljava/lang/String;
        pub fn USER_EXTENSIONS_FORMAT() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.USER_EXTENSIONS_FORMAT:Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "USER_REGION", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true))]
        // static field: USER_REGION:Ljava/lang/String;
        pub fn USER_REGION() -> String {
            panic!("stub: jdk/internal/util/StaticProperty.USER_REGION:Ljava/lang/String;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: jdk/internal/util/StaticProperty.<init>:()V")
        }

        #[java_method(name = "getProperty", descriptor = "(Ljava/util/Properties;Ljava/lang/String;)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getProperty_proper_str(props: Properties, key: String) -> Result<String> {
            panic!("stub: jdk/internal/util/StaticProperty.getProperty:(Ljava/util/Properties;Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "getProperty", descriptor = "(Ljava/util/Properties;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getProperty_proper_str_str(props: Properties, key: String, defaultVal: String) -> Result<String> {
            panic!("stub: jdk/internal/util/StaticProperty.getProperty:(Ljava/util/Properties;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "javaHome", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn javaHome() -> Result<String> {
            panic!("stub: jdk/internal/util/StaticProperty.javaHome:()Ljava/lang/String;")
        }

        #[java_method(name = "userHome", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn userHome() -> Result<String> {
            panic!("stub: jdk/internal/util/StaticProperty.userHome:()Ljava/lang/String;")
        }

        #[java_method(name = "userDir", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn userDir() -> Result<String> {
            panic!("stub: jdk/internal/util/StaticProperty.userDir:()Ljava/lang/String;")
        }

        #[java_method(name = "userName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn userName() -> Result<String> {
            panic!("stub: jdk/internal/util/StaticProperty.userName:()Ljava/lang/String;")
        }

        #[java_method(name = "javaLibraryPath", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn javaLibraryPath() -> Result<String> {
            panic!("stub: jdk/internal/util/StaticProperty.javaLibraryPath:()Ljava/lang/String;")
        }

        #[java_method(name = "javaIoTmpDir", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn javaIoTmpDir() -> Result<String> {
            panic!("stub: jdk/internal/util/StaticProperty.javaIoTmpDir:()Ljava/lang/String;")
        }

        #[java_method(name = "sunBootLibraryPath", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sunBootLibraryPath() -> Result<String> {
            panic!("stub: jdk/internal/util/StaticProperty.sunBootLibraryPath:()Ljava/lang/String;")
        }

        #[java_method(name = "jdkSerialFilter", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn jdkSerialFilter() -> Result<String> {
            panic!("stub: jdk/internal/util/StaticProperty.jdkSerialFilter:()Ljava/lang/String;")
        }

        #[java_method(name = "jdkSerialFilterFactory", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn jdkSerialFilterFactory() -> Result<String> {
            panic!("stub: jdk/internal/util/StaticProperty.jdkSerialFilterFactory:()Ljava/lang/String;")
        }

        #[java_method(name = "nativeEncoding", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nativeEncoding() -> Result<String> {
            panic!("stub: jdk/internal/util/StaticProperty.nativeEncoding:()Ljava/lang/String;")
        }

        #[java_method(name = "fileEncoding", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fileEncoding() -> Result<String> {
            panic!("stub: jdk/internal/util/StaticProperty.fileEncoding:()Ljava/lang/String;")
        }

        #[java_method(name = "javaPropertiesDate", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn javaPropertiesDate() -> Result<String> {
            panic!("stub: jdk/internal/util/StaticProperty.javaPropertiesDate:()Ljava/lang/String;")
        }

        #[java_method(name = "jnuEncoding", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn jnuEncoding() -> Result<String> {
            panic!("stub: jdk/internal/util/StaticProperty.jnuEncoding:()Ljava/lang/String;")
        }

        #[java_method(name = "javaLocaleUseOldISOCodes", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn javaLocaleUseOldISOCodes() -> Result<String> {
            panic!("stub: jdk/internal/util/StaticProperty.javaLocaleUseOldISOCodes:()Ljava/lang/String;")
        }

        #[java_method(name = "osName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn osName() -> Result<String> {
            panic!("stub: jdk/internal/util/StaticProperty.osName:()Ljava/lang/String;")
        }

        #[java_method(name = "osArch", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn osArch() -> Result<String> {
            panic!("stub: jdk/internal/util/StaticProperty.osArch:()Ljava/lang/String;")
        }

        #[java_method(name = "osVersion", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn osVersion() -> Result<String> {
            panic!("stub: jdk/internal/util/StaticProperty.osVersion:()Ljava/lang/String;")
        }
    }
}
