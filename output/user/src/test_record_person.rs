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
use crate::test_record::TestRecord;
use crate::test_record_point::TestRecord_Point;

impl From<TestRecord_Person> for Record {
    fn from(v: TestRecord_Person) -> Record { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "TestRecord$Person"]
    #[super_class       = "java/lang/Record"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestRecord.java"]
    #[inner_classes     = "TestRecord$Person:TestRecord:Person:24;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Record"]
    #[all_supertypes    = "TestRecord$Person;java/lang/Object;java/lang/Record"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct TestRecord_Person {
        #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub name: String,
        #[cfg_attr(any(), java_field(name = "age", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub age: i32,
    }

    impl TestRecord_Person {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = "name:32768;age:32768")]
        pub fn new(mut name: String, mut age: i32) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Record::new()?);
            if (age<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.__set_name(Clone::clone(&name));
            this.__set_age(age);
            Ok(this)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
                Ok(String::from(format!("Person[name={}, age={}]", self.name.get(), self.age.get()).as_str()))
            }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
                Ok(0)
            }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, mut o: Object) -> Result<bool> {
                if let Some(other) = o.0.downcast_ref::<Self>() {
                    Ok(self.__get_name().to_string() == other.__get_name().to_string() && self.__get_age() == other.__get_age())
                } else {
                    Ok(false)
                }
            }

        #[java_method(name = "name", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn name(&self) -> Result<String> {
            let this = self;
            Ok(this.__get_name())
        }

        #[java_method(name = "age", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn age(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_age())
        }
    }
}
