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
use crate::test_inheritance_animal::TestInheritance_Animal;
use crate::test_inheritance::TestInheritance;
use crate::test_inheritance_cat::TestInheritance_Cat;
use crate::test_inheritance_guide_dog::TestInheritance_GuideDog;

impl From<TestInheritance_Dog> for TestInheritance_Animal {
    fn from(v: TestInheritance_Dog) -> TestInheritance_Animal { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestInheritance$Dog"]
    #[super_class       = "TestInheritance$Animal"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestInheritance.java"]
    #[inner_classes     = "TestInheritance$Animal:TestInheritance:Animal:8;TestInheritance$Dog:TestInheritance:Dog:8;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "TestInheritance_Animal"]
    #[superclass_fields(name: String)]
    #[all_supertypes    = "TestInheritance$Animal;TestInheritance$Dog;java/lang/Object"]

    pub struct TestInheritance_Dog;

    impl TestInheritance_Dog {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut name: String) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(TestInheritance_Animal::new(Clone::clone(&name))?);
            Ok(this)
        }

        #[java_method(name = "speak", descriptor = "()Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn speak(&self) -> Result<String> {
            let this = self;
            Ok(String::from_owned(format!("{} barks", this.__get_name())))
        }
    }
}
