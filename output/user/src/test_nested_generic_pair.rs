#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use java_runtime::prelude::*;
use java_runtime::java::io::*;
use java_runtime::java::lang::*;
use java_runtime::java::lang::r#ref::*;
use java_runtime::java::lang::reflect::*;
use java_runtime::java::math::*;
use java_runtime::java::nio::*;
use java_runtime::java::nio::charset::*;
use java_runtime::java::security::*;
use java_runtime::java::text::*;
use java_runtime::java::text::spi::*;
use java_runtime::java::time::*;
use java_runtime::java::time::chrono::*;
use java_runtime::java::time::temporal::*;
use java_runtime::java::time::zone::*;
use java_runtime::java::util::*;
use java_runtime::java::util::concurrent::*;
use java_runtime::java::util::concurrent::atomic::*;
use java_runtime::java::util::concurrent::locks::*;
use java_runtime::java::util::function::*;
use java_runtime::java::util::regex::*;
use java_runtime::java::util::spi::*;
use java_runtime::java::util::stream::*;
use java_runtime::java::util::zip::*;
use java_runtime::sun::nio::ch::*;
use java_runtime::sun::nio::cs::*;
use java_runtime::sun::reflect::generics::factory::*;
use java_runtime::sun::reflect::generics::repository::*;
use java_runtime::sun::reflect::generics::scope::*;
use java_runtime::sun::reflect::misc::*;
use java_runtime::sun::security::action::*;
use java_runtime::sun::security::util::*;
use java_runtime::sun::text::*;
use java_runtime::sun::util::*;
use java_runtime::sun::util::calendar::*;
use java_runtime::sun::util::locale::*;
use java_runtime::sun::util::locale::provider::*;
use java_runtime::sun::util::spi::*;
use java_runtime::java::text::Normalizer;
use crate::test_nested_generic::TestNestedGeneric;
use crate::test_nested_generic_box::TestNestedGeneric_Box;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestNestedGeneric$Pair"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = "<A:Ljava/lang/Object;B:Ljava/lang/Object;>Ljava/lang/Object;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestNestedGeneric.java"]
    #[inner_classes     = "TestNestedGeneric$Pair:TestNestedGeneric:Pair:8;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestNestedGeneric$Pair;java/lang/Object"]
    #[has_to_string_method = true]

    pub struct TestNestedGeneric_Pair<A: Clone + Default + 'static, B: Clone + Default + 'static> {
        #[cfg_attr(any(), java_field(name = "first", descriptor = "Ljava/lang/Object;", is_static = false, generic_signature = "TA;"))]
        pub first: A,
        #[cfg_attr(any(), java_field(name = "second", descriptor = "Ljava/lang/Object;", is_static = false, generic_signature = "TB;"))]
        pub second: B,
    }

    impl<A, B> TestNestedGeneric_Pair<A, B> {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(TA;TB;)V")]
        pub fn new(mut first: A, mut second: B) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_first(Clone::clone(&first));
            this.__set_second(Clone::clone(&second));
            Ok(this)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            let this = self;
            Ok(String::from_owned(format!("({}, {})", String::from_owned(format!("{}", this.__get_first())), String::from_owned(format!("{}", this.__get_second())))))
        }

        #[java_method(name = "swap", descriptor = "()LTestNestedGeneric$Pair;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()LTestNestedGeneric$Pair<TB;TA;>;")]
        pub fn swap(&self) -> Result<TestNestedGeneric_Pair<B, A>> {
            let this = self;
            Ok(Default::default())
        }
    }
}
