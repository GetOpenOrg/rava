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
    #[binary_name       = "TestOptional"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestOptional.java"]
    #[inner_classes     = "java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestOptional;java/lang/Object"]

    pub struct TestOptional;

    impl TestOptional {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "findByLength", descriptor = "([Ljava/lang/String;I)Ljava/util/Optional;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([Ljava/lang/String;I)Ljava/util/Optional<Ljava/lang/String;>;")]
        pub fn findByLength(mut arr: Rc<RefCell<Vec<String>>>, mut len: i32) -> Result<Optional<Object>> {
            let mut local_2: Rc<RefCell<Vec<String>>> = arr;
            let mut local_3 = (local_2.borrow().len() as i32);
            let mut local_4: i32 = 0i32;
            loop {
                if local_4 >= local_3 { break; }
                let mut s = Clone::clone(&local_2.borrow()[local_4 as usize]);
                let _t0 = s.length()?;
                if _t0 == len {
                    let _t1: Optional<Object> = Optional::<Object>::of(Object::from_any(s.clone()))?;
                    return Ok(_t1);
                }
                local_4 = local_4.wrapping_add(1i32);
            }
            let _t0: Optional<Object> = Optional::<Object>::empty()?;
            Ok(_t0)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut _arr0: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 4i32 as usize]));
            _arr0.borrow_mut()[0i32 as usize] = Clone::clone(&String::from("hi"));
            _arr0.borrow_mut()[1i32 as usize] = Clone::clone(&String::from("hello"));
            _arr0.borrow_mut()[2i32 as usize] = Clone::clone(&String::from("hey"));
            _arr0.borrow_mut()[3i32 as usize] = Clone::clone(&String::from("world"));
            let mut words: Rc<RefCell<Vec<String>>> = _arr0;
            let _t1: Optional<Object> = Self::findByLength(Clone::clone(&words), 5i32)?;
            let mut found: Optional<Object> = _t1;
            let _t2 = found.isPresent()?;
            System::out().println_v(_t2)?;
            let _t3 = found.get()?;
            System::out().println_v(Clone::clone(&(_t3).downcast::<String>()))?;
            let _t4: Optional<Object> = Self::findByLength(Clone::clone(&words), 10i32)?;
            let mut notFound: Optional<Object> = _t4;
            let _t5 = notFound.isPresent()?;
            System::out().println_v(_t5)?;
            let _t6 = notFound.orElse(Object::from_any(String::from("default").clone()))?;
            System::out().println_v(Clone::clone(&(_t6).downcast::<String>()))?;
            let __lam_66: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { String::length(_la0) });
            let _t7 = found.map(Clone::clone(&Object::from_any(__lam_66)))?;
            let mut len: Optional<Object> = _t7;
            let _t8 = len.get()?;
            System::out().println_v(Clone::clone(&_t8))?;
            Ok(())
        }
    }
}
