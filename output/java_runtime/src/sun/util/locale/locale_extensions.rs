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
    #[binary_name       = "sun/util/locale/LocaleExtensions"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "LocaleExtensions.java"]
    #[inner_classes     = "java/util/Map$Entry:java/util/Map:Entry:1545;sun/util/locale/InternalLocaleBuilder$CaseInsensitiveChar:sun/util/locale/InternalLocaleBuilder:CaseInsensitiveChar:24;sun/util/locale/InternalLocaleBuilder$CaseInsensitiveString:sun/util/locale/InternalLocaleBuilder:CaseInsensitiveString:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;sun/util/locale/LocaleExtensions"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct LocaleExtensions {
        #[cfg_attr(any(), java_field(name = "extensionMap", descriptor = "Ljava/util/Map;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/Map<Ljava/lang/Character;Lsun/util/locale/Extension;>;"))]
        pub extensionMap: Object,
        #[cfg_attr(any(), java_field(name = "id", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub id: String,
    }

    impl LocaleExtensions {
        #[cfg_attr(any(), java_field(name = "CALENDAR_JAPANESE", descriptor = "Lsun/util/locale/LocaleExtensions;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CALENDAR_JAPANESE:Lsun/util/locale/LocaleExtensions;
        pub fn CALENDAR_JAPANESE() -> LocaleExtensions {
            panic!("stub: sun/util/locale/LocaleExtensions.CALENDAR_JAPANESE:Lsun/util/locale/LocaleExtensions;")
        }

        #[cfg_attr(any(), java_field(name = "NUMBER_THAI", descriptor = "Lsun/util/locale/LocaleExtensions;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NUMBER_THAI:Lsun/util/locale/LocaleExtensions;
        pub fn NUMBER_THAI() -> LocaleExtensions {
            panic!("stub: sun/util/locale/LocaleExtensions.NUMBER_THAI:Lsun/util/locale/LocaleExtensions;")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/Character;Lsun/util/locale/Extension;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str_charac_extens(id: String, key: Character, value: Object) -> Result<Self> {
            panic!("stub: sun/util/locale/LocaleExtensions.<init>:(Ljava/lang/String;Ljava/lang/Character;Lsun/util/locale/Extension;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Map;Ljava/util/Set;Ljava/util/Map;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Lsun/util/locale/InternalLocaleBuilder$CaseInsensitiveChar;Ljava/lang/String;>;Ljava/util/Set<Lsun/util/locale/InternalLocaleBuilder$CaseInsensitiveString;>;Ljava/util/Map<Lsun/util/locale/InternalLocaleBuilder$CaseInsensitiveString;Ljava/lang/String;>;)V")]
        pub fn new_map_set_map(extensions: Object, uattributes: Object, ukeywords: Object) -> Result<Self> {
            panic!("stub: sun/util/locale/LocaleExtensions.<init>:(Ljava/util/Map;Ljava/util/Set;Ljava/util/Map;)V")
        }

        #[java_method(name = "getKeys", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/lang/Character;>;")]
        pub fn getKeys(&self) -> Result<Object> {
            panic!("stub: sun/util/locale/LocaleExtensions.getKeys:()Ljava/util/Set;")
        }

        #[java_method(name = "getExtension", descriptor = "(Ljava/lang/Character;)Lsun/util/locale/Extension;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getExtension(&self, key: Character) -> Result<Object> {
            panic!("stub: sun/util/locale/LocaleExtensions.getExtension:(Ljava/lang/Character;)Lsun/util/locale/Extension;")
        }

        #[java_method(name = "getExtensionValue", descriptor = "(Ljava/lang/Character;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getExtensionValue(&self, key: Character) -> Result<String> {
            panic!("stub: sun/util/locale/LocaleExtensions.getExtensionValue:(Ljava/lang/Character;)Ljava/lang/String;")
        }

        #[java_method(name = "getUnicodeLocaleAttributes", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/lang/String;>;")]
        pub fn getUnicodeLocaleAttributes(&self) -> Result<Object> {
            panic!("stub: sun/util/locale/LocaleExtensions.getUnicodeLocaleAttributes:()Ljava/util/Set;")
        }

        #[java_method(name = "getUnicodeLocaleKeys", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/lang/String;>;")]
        pub fn getUnicodeLocaleKeys(&self) -> Result<Object> {
            panic!("stub: sun/util/locale/LocaleExtensions.getUnicodeLocaleKeys:()Ljava/util/Set;")
        }

        #[java_method(name = "getUnicodeLocaleType", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getUnicodeLocaleType(&self, unicodeLocaleKey: String) -> Result<String> {
            panic!("stub: sun/util/locale/LocaleExtensions.getUnicodeLocaleType:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmpty(&self) -> Result<bool> {
            panic!("stub: sun/util/locale/LocaleExtensions.isEmpty:()Z")
        }

        #[java_method(name = "isValidKey", descriptor = "(C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isValidKey(c: u16) -> Result<bool> {
            panic!("stub: sun/util/locale/LocaleExtensions.isValidKey:(C)Z")
        }

        #[java_method(name = "isValidUnicodeLocaleKey", descriptor = "(Ljava/lang/String;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isValidUnicodeLocaleKey(ukey: String) -> Result<bool> {
            panic!("stub: sun/util/locale/LocaleExtensions.isValidUnicodeLocaleKey:(Ljava/lang/String;)Z")
        }

        #[java_method(name = "toID", descriptor = "(Ljava/util/SortedMap;)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/SortedMap<Ljava/lang/Character;Lsun/util/locale/Extension;>;)Ljava/lang/String;")]
        pub fn toID(map: Object) -> Result<String> {
            panic!("stub: sun/util/locale/LocaleExtensions.toID:(Ljava/util/SortedMap;)Ljava/lang/String;")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "getID", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getID(&self) -> Result<String> {
            panic!("stub: sun/util/locale/LocaleExtensions.getID:()Ljava/lang/String;")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, other: Object) -> Result<bool> {
            panic!("stub: sun/util/locale/LocaleExtensions.equals:(Ljava/lang/Object;)Z")
        }
    }
}
