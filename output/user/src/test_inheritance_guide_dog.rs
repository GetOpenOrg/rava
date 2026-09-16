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
use crate::test_inheritance_dog::TestInheritance_Dog;
use crate::test_inheritance::TestInheritance;
use crate::test_inheritance_animal::TestInheritance_Animal;
use crate::test_inheritance_cat::TestInheritance_Cat;

impl From<TestInheritance_GuideDog> for TestInheritance_Dog {
    fn from(v: TestInheritance_GuideDog) -> TestInheritance_Dog { v.__into_super() }
}

impl From<TestInheritance_GuideDog> for TestInheritance_Animal {
    fn from(v: TestInheritance_GuideDog) -> TestInheritance_Animal { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestInheritance$GuideDog"]
    #[super_class       = "TestInheritance$Dog"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestInheritance.java"]
    #[inner_classes     = "TestInheritance$Dog:TestInheritance:Dog:8;TestInheritance$GuideDog:TestInheritance:GuideDog:8;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "TestInheritance_Dog"]
    #[superclass_fields(name: String)]
    #[all_supertypes    = "TestInheritance$Animal;TestInheritance$Dog;TestInheritance$GuideDog;java/lang/Object"]

    pub struct TestInheritance_GuideDog;

    impl TestInheritance_GuideDog {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut name: String) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(TestInheritance_Dog::new(Clone::clone(&name))?);
            Ok(this)
        }

        #[java_method(name = "speak", descriptor = "()Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn speak(&self) -> Result<String> {
            let this = self;
            let _t0 = this.__super().speak()?;
            Ok(String::from_owned(format!("{} (guide)", _t0)))
        }
    }
}
