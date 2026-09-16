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

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestStaticMembers"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestStaticMembers.java"]
    #[inner_classes     = "java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestStaticMembers;java/lang/Object"]

    pub struct TestStaticMembers;

    impl TestStaticMembers {
        #[cfg_attr(any(), java_field(name = "counter", descriptor = "I", access = "package", modifiers = "static", is_static = true))]
        // static field: counter:I
        pub fn counter() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "PREFIX", descriptor = "Ljava/lang/String;", access = "package", modifiers = "static final", is_static = true, constant_value = "Item"))]
        // static field: PREFIX:Ljava/lang/String;
        pub fn PREFIX() -> String {
            String::from("Item")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "increment", descriptor = "()I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn increment() -> Result<i32> {
            TestStaticMembers::set_counter((TestStaticMembers::counter()).wrapping_add(1i32));
            Ok((TestStaticMembers::counter()).wrapping_add(1i32))
        }

        #[java_method(name = "format", descriptor = "(I)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn format(mut n: i32) -> Result<String> {
            Ok(String::from_owned(format!("Item-{}", n)))
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            Self::class_init()?;
            System::out().println_v(TestStaticMembers::counter())?;
            let _t0: i32 = Self::increment()?;
            let _t1: i32 = Self::increment()?;
            let _t2: i32 = Self::increment()?;
            System::out().println_v(TestStaticMembers::counter())?;
            let _t3: String = Self::format(42i32)?;
            System::out().println_v(Clone::clone(&_t3))?;
            let _t4: String = Self::format(TestStaticMembers::counter())?;
            System::out().println_v(Clone::clone(&_t4))?;
            Ok(())
        }

        #[java_method(name = "<clinit>", descriptor = "()V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn class_init() -> Result<()> {
            TestStaticMembers::set_counter(0i32);
            Ok(())
        }
    }
}
