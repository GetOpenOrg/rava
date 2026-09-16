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
    #[binary_name       = "TestGenericMethod"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestGenericMethod.java"]
    #[inner_classes     = "java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestGenericMethod;java/lang/Object"]

    pub struct TestGenericMethod;

    impl TestGenericMethod {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "max", descriptor = "(Ljava/lang/Comparable;Ljava/lang/Comparable;)Ljava/lang/Comparable;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T::Ljava/lang/Comparable<TT;>;>(TT;TT;)TT;")]
        pub fn max(mut a: Object, mut b: Object) -> Result<Object> {
            let _vdispatch0: i32 = if let Some(_d) = a.0.as_any().downcast_ref::<BuddhistCalendar>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<UTF_8>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<GregorianCalendar>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<JapaneseImperialCalendar>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<HeapByteBuffer>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Unicode>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<US_ASCII>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<ISO_8859_1>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Collector_Characteristics>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Comparators_NaturalOrderComparator>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Locale_Category>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Normalizer_Form>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<StreamShape>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<MatchOps_MatchKind>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Pattern_Qtype>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Formatter_BigDecimalLayoutForm>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<DayOfWeek>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<RoundingMode>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<ZoneOffsetTransitionRule_TimeDefinition>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Month>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<ChronoField>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Character_UnicodeScript>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<StreamOpFlag>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<TestSwitchEnum_Season>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<TestEnumMethods_Planet>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<TestEnumBasic_Day>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<LocalTime>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<ZoneOffsetTransition>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Instant>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<ZoneOffset>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<BigDecimal>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<BigInteger>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Short>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Byte>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Date>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Calendar>() { _d.compareTo_obj(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<CharBuffer>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<ByteBuffer>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Charset>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Float>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<i64>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<bool>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<f64>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<TestComparable_Version>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<TestComparable_Student>() { _d.compareTo(Clone::clone(&b))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&b))? } else if let Some(__f) = a.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&b))? } else { Default::default() };
            Ok((if (_vdispatch0>=0) { a } else { b }))
        }

        #[java_method(name = "swap", descriptor = "([Ljava/lang/Object;II)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;II)V")]
        pub fn swap(mut arr: Rc<RefCell<Vec<Object>>>, mut i: i32, mut j: i32) -> Result<()> {
            let mut tmp = Clone::clone(&arr.borrow()[i as usize]);
            let _aastore_tmp0 = Clone::clone(&Clone::clone(&arr.borrow()[j as usize]));
            arr.borrow_mut()[i as usize] = _aastore_tmp0;
            arr.borrow_mut()[j as usize] = Clone::clone(&tmp);
            Ok(())
        }

        #[java_method(name = "identity", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>(TT;)TT;")]
        pub fn identity(mut value: Object) -> Result<Object> {
            Ok(value)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let _t0: Object = Self::max(3i32.into(), 7i32.into())?;
            System::out().println_v(Clone::clone(&_t0))?;
            let _t1: Object = Self::max(Object::from_any(String::from("apple").clone()), Object::from_any(String::from("banana").clone()))?;
            System::out().println_v(Clone::clone(&(_t1).downcast::<String>()))?;
            let mut _arr2: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![Default::default(); 5i32 as usize]));
            _arr2.borrow_mut()[0i32 as usize] = 1i32;
            _arr2.borrow_mut()[1i32 as usize] = 2i32;
            _arr2.borrow_mut()[2i32 as usize] = 3i32;
            _arr2.borrow_mut()[3i32 as usize] = 4i32;
            _arr2.borrow_mut()[4i32 as usize] = 5i32;
            let mut arr: Rc<RefCell<Vec<i32>>> = _arr2;
            Self::swap(Default::default(), 0i32, 4i32)?;
            let mut local_2: Rc<RefCell<Vec<i32>>> = arr;
            let mut local_3 = (local_2.borrow().len() as i32);
            let mut local_4: i32 = 0i32;
            loop {
                if local_4 >= local_3 { break; }
                let mut x = local_2.borrow()[local_4 as usize];
                System::out().print_str(Clone::clone(&String::from_owned(format!("{} ", x))))?;
                local_4 = local_4.wrapping_add(1i32);
            }
            System::out().println()?;
            let _t3: Object = Self::identity(Object::from_any(String::from("hello").clone()))?;
            System::out().println_v(Clone::clone(&(_t3).downcast::<String>()))?;
            let _t4: Object = Self::identity(42i32.into())?;
            System::out().println_v(Clone::clone(&_t4))?;
            Ok(())
        }
    }
}
