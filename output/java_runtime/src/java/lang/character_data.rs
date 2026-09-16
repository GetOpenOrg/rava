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
    #[binary_name       = "java/lang/CharacterData"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "CharacterData.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/CharacterData;java/lang/Object"]

    pub struct CharacterData;

    impl CharacterData {
        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/CharacterData.<init>:()V")
        }

        #[java_method(name = "getProperties", descriptor = "(I)I", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getProperties(&self, arg0: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData.getProperties:(I)I")
        }

        #[java_method(name = "getType", descriptor = "(I)I", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getType(&self, arg0: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData.getType:(I)I")
        }

        #[java_method(name = "isDigit", descriptor = "(I)Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isDigit(&self, arg0: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData.isDigit:(I)Z")
        }

        #[java_method(name = "isLowerCase", descriptor = "(I)Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isLowerCase(&self, arg0: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData.isLowerCase:(I)Z")
        }

        #[java_method(name = "isUpperCase", descriptor = "(I)Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isUpperCase(&self, arg0: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData.isUpperCase:(I)Z")
        }

        #[java_method(name = "isWhitespace", descriptor = "(I)Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isWhitespace(&self, arg0: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData.isWhitespace:(I)Z")
        }

        #[java_method(name = "isMirrored", descriptor = "(I)Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isMirrored(&self, arg0: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData.isMirrored:(I)Z")
        }

        #[java_method(name = "isJavaIdentifierStart", descriptor = "(I)Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isJavaIdentifierStart(&self, arg0: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData.isJavaIdentifierStart:(I)Z")
        }

        #[java_method(name = "isJavaIdentifierPart", descriptor = "(I)Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isJavaIdentifierPart(&self, arg0: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData.isJavaIdentifierPart:(I)Z")
        }

        #[java_method(name = "isUnicodeIdentifierStart", descriptor = "(I)Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isUnicodeIdentifierStart(&self, arg0: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData.isUnicodeIdentifierStart:(I)Z")
        }

        #[java_method(name = "isUnicodeIdentifierPart", descriptor = "(I)Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isUnicodeIdentifierPart(&self, arg0: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData.isUnicodeIdentifierPart:(I)Z")
        }

        #[java_method(name = "isIdentifierIgnorable", descriptor = "(I)Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isIdentifierIgnorable(&self, arg0: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData.isIdentifierIgnorable:(I)Z")
        }

        #[java_method(name = "isEmoji", descriptor = "(I)Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isEmoji(&self, arg0: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData.isEmoji:(I)Z")
        }

        #[java_method(name = "isEmojiPresentation", descriptor = "(I)Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isEmojiPresentation(&self, arg0: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData.isEmojiPresentation:(I)Z")
        }

        #[java_method(name = "isEmojiModifier", descriptor = "(I)Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isEmojiModifier(&self, arg0: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData.isEmojiModifier:(I)Z")
        }

        #[java_method(name = "isEmojiModifierBase", descriptor = "(I)Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isEmojiModifierBase(&self, arg0: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData.isEmojiModifierBase:(I)Z")
        }

        #[java_method(name = "isEmojiComponent", descriptor = "(I)Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isEmojiComponent(&self, arg0: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData.isEmojiComponent:(I)Z")
        }

        #[java_method(name = "isExtendedPictographic", descriptor = "(I)Z", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isExtendedPictographic(&self, arg0: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData.isExtendedPictographic:(I)Z")
        }

        #[java_method(name = "toLowerCase", descriptor = "(I)I", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn toLowerCase(&self, arg0: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData.toLowerCase:(I)I")
        }

        #[java_method(name = "toUpperCase", descriptor = "(I)I", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn toUpperCase(&self, arg0: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData.toUpperCase:(I)I")
        }

        #[java_method(name = "toTitleCase", descriptor = "(I)I", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn toTitleCase(&self, arg0: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData.toTitleCase:(I)I")
        }

        #[java_method(name = "digit", descriptor = "(II)I", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn digit(&self, arg0: i32, arg1: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData.digit:(II)I")
        }

        #[java_method(name = "getNumericValue", descriptor = "(I)I", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getNumericValue(&self, arg0: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterData.getNumericValue:(I)I")
        }

        #[java_method(name = "getDirectionality", descriptor = "(I)B", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getDirectionality(&self, arg0: i32) -> Result<i8> {
            panic!("stub: java/lang/CharacterData.getDirectionality:(I)B")
        }

        #[java_method(name = "toUpperCaseEx", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperCaseEx(&self, mut ch: i32) -> Result<i32> {
            let this = self;
            let _t0 = this.toUpperCase(ch)?;
            Ok(_t0)
        }

        #[java_method(name = "toUpperCaseCharArray", descriptor = "(I)[C", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperCaseCharArray(&self, mut ch: i32) -> Result<Rc<RefCell<Vec<u16>>>> {
            let this = self;
            Ok(Default::default())
        }

        #[java_method(name = "isOtherAlphabetic", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isOtherAlphabetic(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData.isOtherAlphabetic:(I)Z")
        }

        #[java_method(name = "isIdeographic", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isIdeographic(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterData.isIdeographic:(I)Z")
        }

        #[java_method(name = "of", descriptor = "(I)Ljava/lang/CharacterData;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of(ch: i32) -> Result<CharacterData> {
            panic!("stub: java/lang/CharacterData.of:(I)Ljava/lang/CharacterData;")
        }
    }
}
