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
use crate::test_static_nested_node::TestStaticNested_Node;
use crate::test_static_nested::TestStaticNested;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestStaticNested$LinkedList"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestStaticNested.java"]
    #[inner_classes     = "TestStaticNested$Node:TestStaticNested:Node:8;TestStaticNested$LinkedList:TestStaticNested:LinkedList:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestStaticNested$LinkedList;java/lang/Object"]

    pub struct TestStaticNested_LinkedList {
        #[cfg_attr(any(), java_field(name = "head", descriptor = "LTestStaticNested$Node;", is_static = false))]
        pub head: TestStaticNested_Node,
    }

    impl TestStaticNested_LinkedList {
        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "add", descriptor = "(I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn add(&self, mut v: i32) -> Result<()> {
            let this = self;
            let mut n = TestStaticNested_Node::new(v)?;
            n.__set_next(Clone::clone(&this.__get_head()));
            this.__set_head(Clone::clone(&n));
            Ok(())
        }

        #[java_method(name = "print", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn print(&self) -> Result<()> {
            let this = self;
            let mut cur = this.__get_head();
            loop {
                if _is_jnull(&cur) { break; }
                System::out().println_v(cur.__get_value())?;
                cur = cur.__get_next();
            }
            Ok(())
        }
    }
}
