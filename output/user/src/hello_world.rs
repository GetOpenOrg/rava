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
    #[binary_name       = "HelloWorld"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "HelloWorld.java"]
    #[inner_classes     = "java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "HelloWorld;java/lang/Object"]

    pub struct HelloWorld {
        #[cfg_attr(any(), java_field(name = "message", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub message: String,
    }

    impl HelloWorld {
        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut message: String) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_message(Clone::clone(&message));
            Ok(this)
        }

        #[java_method(name = "greet", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn greet(&self) -> Result<()> {
            let this = self;
            System::out().println_v(Clone::clone(&String::from_owned(format!("Hello, {}", this.__get_message()))))?;
            Ok(())
        }

        #[java_method(name = "repeat", descriptor = "(Ljava/lang/String;I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn repeat(mut s: String, mut times: i32) -> Result<String> {
            let mut sb = StringBuilder::new()?;
            let mut i: i32 = 0i32;
            loop {
                if i >= times { break; }
                let _t0 = sb.append_str(Clone::clone(&s))?;
                i = i.wrapping_add(1i32);
            }
            let _t0 = sb.toString()?;
            Ok(_t0)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut hw = HelloWorld::new(Clone::clone(&String::from("World")))?;
            hw.greet()?;
            let _t0: String = Self::repeat(Clone::clone(&String::from("ha")), 3i32)?;
            let mut r: String = _t0;
            System::out().println_v(Clone::clone(&r))?;
            let mut items = ArrayList::<Object>::new()?;
            let _t1 = items.add_obj(Object::from_any(String::from("foo").clone()))?;
            let _t2 = items.add_obj(Object::from_any(String::from("bar").clone()))?;
            let _t3 = items.size()?;
            System::out().println_v(_t3)?;
            Ok(())
        }
    }
}
