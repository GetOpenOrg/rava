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
    #[binary_name       = "java/text/Format"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Serializable,java/lang/Cloneable"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Format.java"]
    #[inner_classes     = "java/text/AttributedCharacterIterator$Attribute:java/text/AttributedCharacterIterator:Attribute:9;java/text/Format$FieldDelegate:java/text/Format:FieldDelegate:1544;java/text/Format$Field:java/text/Format:Field:9"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Object;java/text/Format"]

    pub struct Format;

    impl Format {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-299282585814624189"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -299282585814624189i64
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "format", descriptor = "(Ljava/lang/Object;)Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn format_obj(&self, obj: Object) -> Result<String> {
            panic!("stub: java/text/Format.format:(Ljava/lang/Object;)Ljava/lang/String;")
        }

        #[java_method(name = "format", descriptor = "(Ljava/lang/Object;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn format_obj_string_fieldp(&self, arg0: Object, arg1: Object, arg2: Object) -> Result<Object> {
            panic!("stub: java/text/Format.format:(Ljava/lang/Object;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;")
        }

        #[java_method(name = "formatToCharacterIterator", descriptor = "(Ljava/lang/Object;)Ljava/text/AttributedCharacterIterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn formatToCharacterIterator(&self, obj: Object) -> Result<Object> {
            panic!("stub: java/text/Format.formatToCharacterIterator:(Ljava/lang/Object;)Ljava/text/AttributedCharacterIterator;")
        }

        #[java_method(name = "parseObject", descriptor = "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn parseObject_str_parsep(&self, arg0: String, arg1: Object) -> Result<Object> {
            panic!("stub: java/text/Format.parseObject:(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Object;")
        }

        #[java_method(name = "parseObject", descriptor = "(Ljava/lang/String;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/text/ParseException")]
        pub fn parseObject_str(&self, source: String) -> Result<Object> {
            panic!("stub: java/text/Format.parseObject:(Ljava/lang/String;)Ljava/lang/Object;")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/text/Format.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "createAttributedCharacterIterator", descriptor = "(Ljava/lang/String;)Ljava/text/AttributedCharacterIterator;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createAttributedCharacterIterator_str(&self, s: String) -> Result<Object> {
            panic!("stub: java/text/Format.createAttributedCharacterIterator:(Ljava/lang/String;)Ljava/text/AttributedCharacterIterator;")
        }

        #[java_method(name = "createAttributedCharacterIterator", descriptor = "([Ljava/text/AttributedCharacterIterator;)Ljava/text/AttributedCharacterIterator;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createAttributedCharacterIterator_arr_att(&self, iterators: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
            panic!("stub: java/text/Format.createAttributedCharacterIterator:([Ljava/text/AttributedCharacterIterator;)Ljava/text/AttributedCharacterIterator;")
        }

        #[java_method(name = "createAttributedCharacterIterator", descriptor = "(Ljava/lang/String;Ljava/text/AttributedCharacterIterator$Attribute;Ljava/lang/Object;)Ljava/text/AttributedCharacterIterator;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createAttributedCharacterIterator_str_attrib_obj(&self, string: String, key: Object, value: Object) -> Result<Object> {
            panic!("stub: java/text/Format.createAttributedCharacterIterator:(Ljava/lang/String;Ljava/text/AttributedCharacterIterator$Attribute;Ljava/lang/Object;)Ljava/text/AttributedCharacterIterator;")
        }

        #[java_method(name = "createAttributedCharacterIterator", descriptor = "(Ljava/text/AttributedCharacterIterator;Ljava/text/AttributedCharacterIterator$Attribute;Ljava/lang/Object;)Ljava/text/AttributedCharacterIterator;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createAttributedCharacterIterator_attrib_attrib_obj(&self, iterator: Object, key: Object, value: Object) -> Result<Object> {
            panic!("stub: java/text/Format.createAttributedCharacterIterator:(Ljava/text/AttributedCharacterIterator;Ljava/text/AttributedCharacterIterator$Attribute;Ljava/lang/Object;)Ljava/text/AttributedCharacterIterator;")
        }
    }
}
