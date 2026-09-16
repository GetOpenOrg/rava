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
    #[binary_name       = "sun/util/locale/InternalLocaleBuilder"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "InternalLocaleBuilder.java"]
    #[inner_classes     = "sun/util/locale/InternalLocaleBuilder$CaseInsensitiveString:sun/util/locale/InternalLocaleBuilder:CaseInsensitiveString:24;sun/util/locale/InternalLocaleBuilder$CaseInsensitiveChar:sun/util/locale/InternalLocaleBuilder:CaseInsensitiveChar:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;sun/util/locale/InternalLocaleBuilder"]

    pub struct InternalLocaleBuilder {
        #[cfg_attr(any(), java_field(name = "language", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub language: String,
        #[cfg_attr(any(), java_field(name = "script", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub script: String,
        #[cfg_attr(any(), java_field(name = "region", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub region: String,
        #[cfg_attr(any(), java_field(name = "variant", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub variant: String,
        #[cfg_attr(any(), java_field(name = "extensions", descriptor = "Ljava/util/Map;", access = "private", modifiers = "", is_static = false, generic_signature = "Ljava/util/Map<Lsun/util/locale/InternalLocaleBuilder$CaseInsensitiveChar;Ljava/lang/String;>;"))]
        pub extensions: Object,
        #[cfg_attr(any(), java_field(name = "uattributes", descriptor = "Ljava/util/Set;", access = "private", modifiers = "", is_static = false, generic_signature = "Ljava/util/Set<Lsun/util/locale/InternalLocaleBuilder$CaseInsensitiveString;>;"))]
        pub uattributes: Object,
        #[cfg_attr(any(), java_field(name = "ukeywords", descriptor = "Ljava/util/Map;", access = "private", modifiers = "", is_static = false, generic_signature = "Ljava/util/Map<Lsun/util/locale/InternalLocaleBuilder$CaseInsensitiveString;Ljava/lang/String;>;"))]
        pub ukeywords: Object,
    }

    impl InternalLocaleBuilder {
        #[cfg_attr(any(), java_field(name = "PRIVATEUSE_KEY", descriptor = "Lsun/util/locale/InternalLocaleBuilder$CaseInsensitiveChar;", access = "private", modifiers = "static final", is_static = true))]
        // static field: PRIVATEUSE_KEY:Lsun/util/locale/InternalLocaleBuilder$CaseInsensitiveChar;
        pub fn PRIVATEUSE_KEY() -> Object {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.PRIVATEUSE_KEY:Lsun/util/locale/InternalLocaleBuilder$CaseInsensitiveChar;")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.<init>:()V")
        }

        #[java_method(name = "setLanguage", descriptor = "(Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "sun/util/locale/LocaleSyntaxException")]
        pub fn setLanguage(&self, language: String) -> Result<InternalLocaleBuilder> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.setLanguage:(Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;")
        }

        #[java_method(name = "setScript", descriptor = "(Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "sun/util/locale/LocaleSyntaxException")]
        pub fn setScript(&self, script: String) -> Result<InternalLocaleBuilder> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.setScript:(Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;")
        }

        #[java_method(name = "setRegion", descriptor = "(Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "sun/util/locale/LocaleSyntaxException")]
        pub fn setRegion(&self, region: String) -> Result<InternalLocaleBuilder> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.setRegion:(Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;")
        }

        #[java_method(name = "setVariant", descriptor = "(Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "sun/util/locale/LocaleSyntaxException")]
        pub fn setVariant(&self, variant: String) -> Result<InternalLocaleBuilder> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.setVariant:(Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;")
        }

        #[java_method(name = "addUnicodeLocaleAttribute", descriptor = "(Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "sun/util/locale/LocaleSyntaxException")]
        pub fn addUnicodeLocaleAttribute(&self, attribute: String) -> Result<InternalLocaleBuilder> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.addUnicodeLocaleAttribute:(Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;")
        }

        #[java_method(name = "removeUnicodeLocaleAttribute", descriptor = "(Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "sun/util/locale/LocaleSyntaxException")]
        pub fn removeUnicodeLocaleAttribute(&self, attribute: String) -> Result<InternalLocaleBuilder> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.removeUnicodeLocaleAttribute:(Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;")
        }

        #[java_method(name = "setUnicodeLocaleKeyword", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "sun/util/locale/LocaleSyntaxException")]
        pub fn setUnicodeLocaleKeyword(&self, key: String, type_: String) -> Result<InternalLocaleBuilder> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.setUnicodeLocaleKeyword:(Ljava/lang/String;Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;")
        }

        #[java_method(name = "setExtension", descriptor = "(CLjava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "sun/util/locale/LocaleSyntaxException")]
        pub fn setExtension(&self, singleton: u16, value: String) -> Result<InternalLocaleBuilder> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.setExtension:(CLjava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;")
        }

        #[java_method(name = "setExtensions", descriptor = "(Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "sun/util/locale/LocaleSyntaxException")]
        pub fn setExtensions_str(&self, subtags: String) -> Result<InternalLocaleBuilder> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.setExtensions:(Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;")
        }

        #[java_method(name = "setExtensions", descriptor = "(Ljava/util/List;Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<Ljava/lang/String;>;Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;")]
        pub fn setExtensions_list_str(&self, bcpExtensions: Object, privateuse: String) -> Result<InternalLocaleBuilder> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.setExtensions:(Ljava/util/List;Ljava/lang/String;)Lsun/util/locale/InternalLocaleBuilder;")
        }

        #[java_method(name = "setLanguageTag", descriptor = "(Lsun/util/locale/LanguageTag;)Lsun/util/locale/InternalLocaleBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setLanguageTag(&self, langtag: Object) -> Result<InternalLocaleBuilder> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.setLanguageTag:(Lsun/util/locale/LanguageTag;)Lsun/util/locale/InternalLocaleBuilder;")
        }

        #[java_method(name = "setLocale", descriptor = "(Lsun/util/locale/BaseLocale;Lsun/util/locale/LocaleExtensions;)Lsun/util/locale/InternalLocaleBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "sun/util/locale/LocaleSyntaxException")]
        pub fn setLocale(&self, base: BaseLocale, localeExtensions: LocaleExtensions) -> Result<InternalLocaleBuilder> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.setLocale:(Lsun/util/locale/BaseLocale;Lsun/util/locale/LocaleExtensions;)Lsun/util/locale/InternalLocaleBuilder;")
        }

        #[java_method(name = "clear", descriptor = "()Lsun/util/locale/InternalLocaleBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<InternalLocaleBuilder> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.clear:()Lsun/util/locale/InternalLocaleBuilder;")
        }

        #[java_method(name = "clearExtensions", descriptor = "()Lsun/util/locale/InternalLocaleBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clearExtensions(&self) -> Result<InternalLocaleBuilder> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.clearExtensions:()Lsun/util/locale/InternalLocaleBuilder;")
        }

        #[java_method(name = "getBaseLocale", descriptor = "()Lsun/util/locale/BaseLocale;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBaseLocale(&self) -> Result<BaseLocale> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.getBaseLocale:()Lsun/util/locale/BaseLocale;")
        }

        #[java_method(name = "getLocaleExtensions", descriptor = "()Lsun/util/locale/LocaleExtensions;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLocaleExtensions(&self) -> Result<LocaleExtensions> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.getLocaleExtensions:()Lsun/util/locale/LocaleExtensions;")
        }

        #[java_method(name = "removePrivateuseVariant", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn removePrivateuseVariant(privuseVal: String) -> Result<String> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.removePrivateuseVariant:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "checkVariants", descriptor = "(Ljava/lang/String;Ljava/lang/String;)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkVariants(&self, variants: String, sep: String) -> Result<i32> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.checkVariants:(Ljava/lang/String;Ljava/lang/String;)I")
        }

        #[java_method(name = "setUnicodeLocaleExtension", descriptor = "(Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setUnicodeLocaleExtension(&self, subtags: String) -> Result<()> {
            panic!("stub: sun/util/locale/InternalLocaleBuilder.setUnicodeLocaleExtension:(Ljava/lang/String;)V")
        }
    }
}
