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

impl From<CharacterDataPrivateUse> for CharacterData {
    fn from(v: CharacterDataPrivateUse) -> CharacterData { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/CharacterDataPrivateUse"]
    #[super_class       = "java/lang/CharacterData"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "CharacterDataPrivateUse.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "CharacterData"]
    #[all_supertypes    = "java/lang/CharacterData;java/lang/CharacterDataPrivateUse;java/lang/Object"]

    pub struct CharacterDataPrivateUse;

    impl CharacterDataPrivateUse {
        #[cfg_attr(any(), java_field(name = "instance", descriptor = "Ljava/lang/CharacterData;", access = "package", modifiers = "static final", is_static = true))]
        // static field: instance:Ljava/lang/CharacterData;
        pub fn instance() -> CharacterData {
            panic!("stub: java/lang/CharacterDataPrivateUse.instance:Ljava/lang/CharacterData;")
        }

        #[java_method(name = "getProperties", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getProperties(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterDataPrivateUse.getProperties:(I)I")
        }

        #[java_method(name = "getType", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getType(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterDataPrivateUse.getType:(I)I")
        }

        #[java_method(name = "isJavaIdentifierStart", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isJavaIdentifierStart(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataPrivateUse.isJavaIdentifierStart:(I)Z")
        }

        #[java_method(name = "isJavaIdentifierPart", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isJavaIdentifierPart(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataPrivateUse.isJavaIdentifierPart:(I)Z")
        }

        #[java_method(name = "isUnicodeIdentifierStart", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUnicodeIdentifierStart(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataPrivateUse.isUnicodeIdentifierStart:(I)Z")
        }

        #[java_method(name = "isUnicodeIdentifierPart", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUnicodeIdentifierPart(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataPrivateUse.isUnicodeIdentifierPart:(I)Z")
        }

        #[java_method(name = "isIdentifierIgnorable", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isIdentifierIgnorable(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataPrivateUse.isIdentifierIgnorable:(I)Z")
        }

        #[java_method(name = "isEmoji", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmoji(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataPrivateUse.isEmoji:(I)Z")
        }

        #[java_method(name = "isEmojiPresentation", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiPresentation(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataPrivateUse.isEmojiPresentation:(I)Z")
        }

        #[java_method(name = "isEmojiModifier", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiModifier(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataPrivateUse.isEmojiModifier:(I)Z")
        }

        #[java_method(name = "isEmojiModifierBase", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiModifierBase(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataPrivateUse.isEmojiModifierBase:(I)Z")
        }

        #[java_method(name = "isEmojiComponent", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiComponent(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataPrivateUse.isEmojiComponent:(I)Z")
        }

        #[java_method(name = "isExtendedPictographic", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isExtendedPictographic(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataPrivateUse.isExtendedPictographic:(I)Z")
        }

        #[java_method(name = "toLowerCase", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toLowerCase(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterDataPrivateUse.toLowerCase:(I)I")
        }

        #[java_method(name = "toUpperCase", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperCase(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterDataPrivateUse.toUpperCase:(I)I")
        }

        #[java_method(name = "toTitleCase", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toTitleCase(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterDataPrivateUse.toTitleCase:(I)I")
        }

        #[java_method(name = "digit", descriptor = "(II)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn digit(&self, ch: i32, radix: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterDataPrivateUse.digit:(II)I")
        }

        #[java_method(name = "getNumericValue", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNumericValue(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterDataPrivateUse.getNumericValue:(I)I")
        }

        #[java_method(name = "isDigit", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDigit(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataPrivateUse.isDigit:(I)Z")
        }

        #[java_method(name = "isLowerCase", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLowerCase(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataPrivateUse.isLowerCase:(I)Z")
        }

        #[java_method(name = "isUpperCase", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUpperCase(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataPrivateUse.isUpperCase:(I)Z")
        }

        #[java_method(name = "isWhitespace", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isWhitespace(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataPrivateUse.isWhitespace:(I)Z")
        }

        #[java_method(name = "getDirectionality", descriptor = "(I)B", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDirectionality(&self, ch: i32) -> Result<i8> {
            panic!("stub: java/lang/CharacterDataPrivateUse.getDirectionality:(I)B")
        }

        #[java_method(name = "isMirrored", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isMirrored(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataPrivateUse.isMirrored:(I)Z")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/CharacterDataPrivateUse.<init>:()V")
        }
    }
}
