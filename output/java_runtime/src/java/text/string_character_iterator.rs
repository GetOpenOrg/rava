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
    #[binary_name       = "java/text/StringCharacterIterator"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/text/CharacterIterator"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "StringCharacterIterator.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/text/CharacterIterator;java/text/StringCharacterIterator"]
    #[has_hash_code_method = true]

    pub struct StringCharacterIterator {
        #[cfg_attr(any(), java_field(name = "text", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub text: String,
        #[cfg_attr(any(), java_field(name = "begin", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub begin: i32,
        #[cfg_attr(any(), java_field(name = "end", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub end: i32,
        #[cfg_attr(any(), java_field(name = "pos", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub pos: i32,
    }

    impl StringCharacterIterator {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/String;)V
        pub fn new_str(mut text: String) -> Result<Self> {
            let mut this = Self::default();
            this = StringCharacterIterator::new_str_i(Clone::clone(&text), 0i32)?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/String;I)V
        pub fn new_str_i(mut text: String, mut pos: i32) -> Result<Self> {
            let mut this = Self::default();
            let _t0 = text.length()?;
            this = StringCharacterIterator::new_str_i_i_i(Clone::clone(&text), 0i32, _t0, pos)?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;III)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/String;III)V
        pub fn new_str_i_i_i(mut text: String, mut begin: i32, mut end: i32, mut pos: i32) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            if _is_jnull(&text) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.__set_text(Clone::clone(&text));
            let _t0 = text.length()?;
            if end > _t0 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if pos > end {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.__set_begin(begin);
            this.__set_end(end);
            this.__set_pos(pos);
            Ok(this)
        }

        #[java_method(name = "setText", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setText(&self, text: String) -> Result<()> {
            panic!("stub: java/text/StringCharacterIterator.setText:(Ljava/lang/String;)V")
        }

        #[java_method(name = "first", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn first(&self) -> Result<u16> {
            panic!("stub: java/text/StringCharacterIterator.first:()C")
        }

        #[java_method(name = "last", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn last(&self) -> Result<u16> {
            panic!("stub: java/text/StringCharacterIterator.last:()C")
        }

        #[java_method(name = "setIndex", descriptor = "(I)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setIndex(&self, p: i32) -> Result<u16> {
            panic!("stub: java/text/StringCharacterIterator.setIndex:(I)C")
        }

        #[java_method(name = "current", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn current(&self) -> Result<u16> {
            panic!("stub: java/text/StringCharacterIterator.current:()C")
        }

        #[java_method(name = "next", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn next(&self) -> Result<u16> {
            panic!("stub: java/text/StringCharacterIterator.next:()C")
        }

        #[java_method(name = "previous", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn previous(&self) -> Result<u16> {
            panic!("stub: java/text/StringCharacterIterator.previous:()C")
        }

        #[java_method(name = "getBeginIndex", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBeginIndex(&self) -> Result<i32> {
            panic!("stub: java/text/StringCharacterIterator.getBeginIndex:()I")
        }

        #[java_method(name = "getEndIndex", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getEndIndex(&self) -> Result<i32> {
            panic!("stub: java/text/StringCharacterIterator.getEndIndex:()I")
        }

        #[java_method(name = "getIndex", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getIndex(&self) -> Result<i32> {
            panic!("stub: java/text/StringCharacterIterator.getIndex:()I")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/text/StringCharacterIterator.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/text/StringCharacterIterator.clone:()Ljava/lang/Object;")
        }
    }
}
