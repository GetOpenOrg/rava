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

impl From<CharacterDataLatin1> for CharacterData {
    fn from(v: CharacterDataLatin1) -> CharacterData { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/CharacterDataLatin1"]
    #[super_class       = "java/lang/CharacterData"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "CharacterDataLatin1.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "CharacterData"]
    #[all_supertypes    = "java/lang/CharacterData;java/lang/CharacterDataLatin1;java/lang/Object"]

    pub struct CharacterDataLatin1;

    impl CharacterDataLatin1 {
        #[cfg_attr(any(), java_field(name = "DIGITS", descriptor = "[B", access = "private", modifiers = "static final", is_static = true))]
        // static field: DIGITS:[B
        pub fn DIGITS() -> Rc<RefCell<Vec<i8>>> {
            panic!("stub: java/lang/CharacterDataLatin1.DIGITS:[B")
        }

        #[cfg_attr(any(), java_field(name = "sharpsMap", descriptor = "[C", access = "package", modifiers = "static", is_static = true))]
        // static field: sharpsMap:[C
        pub fn sharpsMap() -> Rc<RefCell<Vec<u16>>> {
            panic!("stub: java/lang/CharacterDataLatin1.sharpsMap:[C")
        }

        #[cfg_attr(any(), java_field(name = "instance", descriptor = "Ljava/lang/CharacterDataLatin1;", access = "package", modifiers = "static final", is_static = true))]
        // static field: instance:Ljava/lang/CharacterDataLatin1;
        pub fn instance() -> CharacterDataLatin1 {
            panic!("stub: java/lang/CharacterDataLatin1.instance:Ljava/lang/CharacterDataLatin1;")
        }

        #[cfg_attr(any(), java_field(name = "A", descriptor = "[I", access = "package", modifiers = "static final", is_static = true))]
        // static field: A:[I
        pub fn A() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/lang/CharacterDataLatin1.A:[I")
        }

        #[cfg_attr(any(), java_field(name = "B", descriptor = "[C", access = "package", modifiers = "static final", is_static = true))]
        // static field: B:[C
        pub fn B() -> Rc<RefCell<Vec<u16>>> {
            panic!("stub: java/lang/CharacterDataLatin1.B:[C")
        }

        #[java_method(name = "getProperties", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getProperties(&self, mut ch: i32) -> Result<i32> {
            let this = self;
            let mut offset = ((ch) as u16 as i32);
            let mut props = CharacterDataLatin1::A().borrow()[offset as usize];
            Ok(props)
        }

        #[java_method(name = "getPropertiesEx", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPropertiesEx(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterDataLatin1.getPropertiesEx:(I)I")
        }

        #[java_method(name = "isDigit", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDigit(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataLatin1.isDigit:(I)Z")
        }

        #[java_method(name = "isLowerCase", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLowerCase(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataLatin1.isLowerCase:(I)Z")
        }

        #[java_method(name = "isUpperCase", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUpperCase(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataLatin1.isUpperCase:(I)Z")
        }

        #[java_method(name = "isOtherAlphabetic", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isOtherAlphabetic(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataLatin1.isOtherAlphabetic:(I)Z")
        }

        #[java_method(name = "isIdeographic", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isIdeographic(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataLatin1.isIdeographic:(I)Z")
        }

        #[java_method(name = "getType", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getType(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterDataLatin1.getType:(I)I")
        }

        #[java_method(name = "isJavaIdentifierStart", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isJavaIdentifierStart(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataLatin1.isJavaIdentifierStart:(I)Z")
        }

        #[java_method(name = "isJavaIdentifierPart", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isJavaIdentifierPart(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataLatin1.isJavaIdentifierPart:(I)Z")
        }

        #[java_method(name = "isUnicodeIdentifierStart", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUnicodeIdentifierStart(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataLatin1.isUnicodeIdentifierStart:(I)Z")
        }

        #[java_method(name = "isUnicodeIdentifierPart", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUnicodeIdentifierPart(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataLatin1.isUnicodeIdentifierPart:(I)Z")
        }

        #[java_method(name = "isIdentifierIgnorable", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isIdentifierIgnorable(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataLatin1.isIdentifierIgnorable:(I)Z")
        }

        #[java_method(name = "isEmoji", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmoji(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataLatin1.isEmoji:(I)Z")
        }

        #[java_method(name = "isEmojiPresentation", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiPresentation(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataLatin1.isEmojiPresentation:(I)Z")
        }

        #[java_method(name = "isEmojiModifier", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiModifier(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataLatin1.isEmojiModifier:(I)Z")
        }

        #[java_method(name = "isEmojiModifierBase", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiModifierBase(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataLatin1.isEmojiModifierBase:(I)Z")
        }

        #[java_method(name = "isEmojiComponent", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmojiComponent(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataLatin1.isEmojiComponent:(I)Z")
        }

        #[java_method(name = "isExtendedPictographic", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isExtendedPictographic(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataLatin1.isExtendedPictographic:(I)Z")
        }

        #[java_method(name = "toLowerCase", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toLowerCase(&self, mut ch: i32) -> Result<i32> {
            let this = self;
            if ch < 65i32 {
                return Ok(ch);
            }
            let mut lower = (ch|32i32);
            if lower != 247i32 {
                return Ok(lower);
            }
            Ok(ch)
        }

        #[java_method(name = "toUpperCase", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperCase(&self, mut ch: i32) -> Result<i32> {
            let this = self;
            if ch < 97i32 {
                return Ok(ch);
            }
            let mut upper = (ch&223i32);
            if upper != 215i32 {
                return Ok(upper);
            }
            if ch == 255i32 {
                return Ok(376i32);
            }
            if ch == 181i32 {
                return Ok(924i32);
            }
            Ok(ch)
        }

        #[java_method(name = "equalsIgnoreCase", descriptor = "(BB)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equalsIgnoreCase(mut b1: i8, mut b2: i8) -> Result<bool> {
            if (b1 as i32) == (b2 as i32) {
                return Ok((1i32 != 0i32));
            }
            let mut upper = ((b1 as i32)&223i32);
            if upper < 65i32 {
                return Ok((0i32 != 0i32));
            }
            Ok((if upper >= 192i32 { (if upper <= 222i32 { (if upper != 215i32 { upper == ((b2 as i32)&223i32) } else { (0i32 != 0) }) } else { (0i32 != 0) }) } else { (0i32 != 0) }))
        }

        #[java_method(name = "toTitleCase", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toTitleCase(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterDataLatin1.toTitleCase:(I)I")
        }

        #[java_method(name = "digit", descriptor = "(II)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn digit(&self, ch: i32, radix: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterDataLatin1.digit:(II)I")
        }

        #[java_method(name = "getNumericValue", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNumericValue(&self, ch: i32) -> Result<i32> {
            panic!("stub: java/lang/CharacterDataLatin1.getNumericValue:(I)I")
        }

        #[java_method(name = "isWhitespace", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isWhitespace(&self, mut ch: i32) -> Result<bool> {
            let this = self;
            let _t0 = this.getProperties(ch)?;
            let mut props: i32 = _t0;
            Ok((props&28672i32) == 16384i32)
        }

        #[java_method(name = "getDirectionality", descriptor = "(I)B", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDirectionality(&self, ch: i32) -> Result<i8> {
            panic!("stub: java/lang/CharacterDataLatin1.getDirectionality:(I)B")
        }

        #[java_method(name = "isMirrored", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isMirrored(&self, ch: i32) -> Result<bool> {
            panic!("stub: java/lang/CharacterDataLatin1.isMirrored:(I)Z")
        }

        #[java_method(name = "toUpperCaseEx", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperCaseEx(&self, mut ch: i32) -> Result<i32> {
            let this = self;
            let mut mapChar: i32 = ch;
            let _t0 = this.getProperties(ch)?;
            let mut val: i32 = _t0;
            if (val&133955584i32) != 133955584i32 {
                let mut offset = ((val<<(5i32&0x1f))>>((23i32&0x1f)));
                mapChar = (ch).wrapping_sub(offset);
            } else {
                match ch {
                    181 => {
                        mapChar = 924i32;
                    }
                    _ => {
                        mapChar = -1i32;
                    }
                }
            }
            Ok(mapChar)
        }

        #[java_method(name = "toUpperCaseCharArray", descriptor = "(I)[C", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperCaseCharArray(&self, mut ch: i32) -> Result<Rc<RefCell<Vec<u16>>>> {
            let this = self;
            let mut _arr0: Rc<RefCell<Vec<u16>>> = Rc::new(RefCell::new(vec![0u16; 1i32 as usize]));
            _arr0.borrow_mut()[0i32 as usize] = (((ch) as u16 as i32)) as u16;
            let mut upperMap: Rc<RefCell<Vec<u16>>> = _arr0;
            if ch == 223i32 {
                upperMap = CharacterDataLatin1::sharpsMap();
            }
            Ok(upperMap)
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/CharacterDataLatin1.<init>:()V")
        }
    }
}
