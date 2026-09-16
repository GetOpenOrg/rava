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
    #[binary_name       = "java/util/Formatter$FixedString"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/Formatter$FormatString"]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Formatter.java"]
    #[inner_classes     = "java/util/Formatter$FixedString:java/util/Formatter:FixedString:10;java/util/Formatter$FormatString:java/util/Formatter:FormatString:1544"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Formatter$FixedString;java/util/Formatter$FormatString"]
    #[has_to_string_method = true]

    pub struct Formatter_FixedString {
        #[cfg_attr(any(), java_field(name = "s", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub s: String,
        #[cfg_attr(any(), java_field(name = "start", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub start: i32,
        #[cfg_attr(any(), java_field(name = "end", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub end: i32,
    }

    impl Formatter_FixedString {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;II)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut s: String, mut start: i32, mut end: i32) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_s(Clone::clone(&s));
            this.__set_start(start);
            this.__set_end(end);
            Ok(this)
        }

        #[java_method(name = "index", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn index(&self) -> Result<i32> {
            let this = self;
            Ok(-2i32)
        }

        #[java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn print(&self, mut fmt: Formatter, mut arg: Object, mut l: Locale) -> Result<()> {
            let this = self;
            let _vdispatch0: Object = if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_seq_i_i(Object::from_any(this.__get_s().clone()), this.__get_start(), this.__get_end())? } else if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<StreamEncoder>() { _d.append(Object::from_any(this.__get_s().clone()), this.__get_start(), this.__get_end())? } else if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_seq_i_i(Object::from_any(this.__get_s().clone()), this.__get_start(), this.__get_end())? } else if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<BufferedWriter>() { _d.append(Object::from_any(this.__get_s().clone()), this.__get_start(), this.__get_end())? } else if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<CharBuffer>() { _d.append_seq_i_i(Object::from_any(this.__get_s().clone()), this.__get_start(), this.__get_end())? } else if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<Writer>() { _d.append_seq_i_i(Object::from_any(this.__get_s().clone()), this.__get_start(), this.__get_end())? } else if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_seq_i_i(Object::from_any(this.__get_s().clone()), this.__get_start(), this.__get_end())? } else if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<StringBuilder>() { _d.append_seq_i_i(Object::from_any(this.__get_s().clone()), this.__get_start(), this.__get_end())? } else if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<PrintStream>() { _d.append_seq_i_i(Object::from_any(this.__get_s().clone()), this.__get_start(), this.__get_end())? } else if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<Object>() { _d.append(Object::from_any(this.__get_s().clone()), this.__get_start(), this.__get_end())? } else if let Some(__f) = fmt.__get_a().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object, i32, i32) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(this.__get_s().clone()), this.__get_start(), this.__get_end())? } else { Default::default() };
            Ok(())
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            let this = self;
            let _t0 = this.__get_s().substring_i_i(this.__get_start(), this.__get_end())?;
            Ok(_t0)
        }
    }
}
