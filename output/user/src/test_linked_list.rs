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
    #[binary_name       = "TestLinkedList"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestLinkedList.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestLinkedList;java/lang/Object"]

    pub struct TestLinkedList;

    impl TestLinkedList {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut list = LinkedList::<Object>::new()?;
            let _t0 = list.add_obj(1i32.into())?;
            let _t1 = list.add_obj(2i32.into())?;
            let _t2 = list.add_obj(3i32.into())?;
            list.addFirst(0i32.into())?;
            list.addLast(4i32.into())?;
            let _t3 = list.size()?;
            System::out().println_v(_t3)?;
            let _t4 = list.getFirst()?;
            System::out().println_v(Clone::clone(&_t4))?;
            let _t5 = list.getLast()?;
            System::out().println_v(Clone::clone(&_t5))?;
            let _t6 = list.removeFirst()?;
            let _t7 = list.removeLast()?;
            System::out().println_v(Object::from_any(list.clone()))?;
            let mut stack = LinkedList::<Object>::new()?;
            stack.push(Object::from_any(String::from("a").clone()))?;
            stack.push(Object::from_any(String::from("b").clone()))?;
            stack.push(Object::from_any(String::from("c").clone()))?;
            let _t8 = stack.pop()?;
            System::out().println_v(Clone::clone(&(_t8).downcast::<String>()))?;
            let _t9 = stack.peek()?;
            System::out().println_v(Clone::clone(&(_t9).downcast::<String>()))?;
            Ok(())
        }
    }
}
