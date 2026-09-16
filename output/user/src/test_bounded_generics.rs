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
    #[binary_name       = "TestBoundedGenerics"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestBoundedGenerics.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestBoundedGenerics;java/lang/Object"]

    pub struct TestBoundedGenerics;

    impl TestBoundedGenerics {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "sum", descriptor = "([Ljava/lang/Number;)D", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Number;>([TT;)D")]
        pub fn sum(mut arr: Rc<RefCell<Vec<Number>>>) -> Result<f64> {
            let mut total: f64 = 0f64;
            let mut local_3: Rc<RefCell<Vec<Number>>> = arr;
            let mut local_4 = (local_3.borrow().len() as i32);
            let mut local_5: i32 = 0i32;
            loop {
                if local_5 >= local_4 { break; }
                let mut x = Clone::clone(&local_3.borrow()[local_5 as usize]);
                let _t0: f64 = x.doubleValue()?;
                total = (total+_t0);
                local_5 = local_5.wrapping_add(1i32);
            }
            Ok(total)
        }

        #[java_method(name = "isSorted", descriptor = "([Ljava/lang/Comparable;)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T::Ljava/lang/Comparable<TT;>;>([TT;)Z")]
        pub fn isSorted(mut arr: Rc<RefCell<Vec<Object>>>) -> Result<bool> {
            let mut i: i32 = 1i32;
            loop {
                if i >= (arr.borrow().len() as i32) { break; }
                let _vdispatch0: i32 = if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<BuddhistCalendar>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<UTF_8>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<GregorianCalendar>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<JapaneseImperialCalendar>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<HeapCharBuffer>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<HeapByteBuffer>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Unicode>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<US_ASCII>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<ISO_8859_1>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Collector_Characteristics>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Comparators_NaturalOrderComparator>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Locale_Category>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Normalizer_Form>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<StreamShape>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<MatchOps_MatchKind>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Pattern_Qtype>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Formatter_BigDecimalLayoutForm>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<DayOfWeek>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<RoundingMode>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<ZoneOffsetTransitionRule_TimeDefinition>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Month>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<ChronoField>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Character_UnicodeScript>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<StreamOpFlag>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<TestSwitchEnum_Season>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<TestEnumMethods_Planet>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<TestEnumBasic_Day>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<LocalTime>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<ZoneOffsetTransition>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Instant>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<ZoneOffset>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<BigDecimal>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<BigInteger>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Short>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Byte>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Date>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Calendar>() { _d.compareTo_obj(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<CharBuffer>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<ByteBuffer>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Charset>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Float>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<i64>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<bool>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<f64>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<TestComparable_Version>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<TestComparable_Student>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(_d) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else if let Some(__f) = Clone::clone(&arr.borrow()[(i).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&arr.borrow()[i as usize])))? } else { Default::default() };
                if (_vdispatch0>0) {
                    return Ok((0i32 != 0i32));
                }
                i = i.wrapping_add(1i32);
            }
            Ok((1i32 != 0i32))
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![Default::default(); 5i32 as usize]));
            _arr0.borrow_mut()[0i32 as usize] = 1i32;
            _arr0.borrow_mut()[1i32 as usize] = 2i32;
            _arr0.borrow_mut()[2i32 as usize] = 3i32;
            _arr0.borrow_mut()[3i32 as usize] = 4i32;
            _arr0.borrow_mut()[4i32 as usize] = 5i32;
            let mut ints: Rc<RefCell<Vec<i32>>> = _arr0;
            let _t1: f64 = Self::sum(Default::default())?;
            System::out().println_v(_t1)?;
            let mut _arr2: Rc<RefCell<Vec<f64>>> = Rc::new(RefCell::new(vec![Default::default(); 3i32 as usize]));
            _arr2.borrow_mut()[0i32 as usize] = 1.5f64;
            _arr2.borrow_mut()[1i32 as usize] = 2.5f64;
            _arr2.borrow_mut()[2i32 as usize] = 3.0f64;
            let mut doubles: Rc<RefCell<Vec<f64>>> = _arr2;
            let _t3: f64 = Self::sum(Default::default())?;
            System::out().println_v(_t3)?;
            let mut _arr4: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 3i32 as usize]));
            _arr4.borrow_mut()[0i32 as usize] = Clone::clone(&String::from("a"));
            _arr4.borrow_mut()[1i32 as usize] = Clone::clone(&String::from("b"));
            _arr4.borrow_mut()[2i32 as usize] = Clone::clone(&String::from("c"));
            let mut sorted: Rc<RefCell<Vec<String>>> = _arr4;
            let _t5: bool = Self::isSorted(Default::default())?;
            System::out().println_v(_t5)?;
            let mut _arr6: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 3i32 as usize]));
            _arr6.borrow_mut()[0i32 as usize] = Clone::clone(&String::from("b"));
            _arr6.borrow_mut()[1i32 as usize] = Clone::clone(&String::from("a"));
            _arr6.borrow_mut()[2i32 as usize] = Clone::clone(&String::from("c"));
            let mut unsorted: Rc<RefCell<Vec<String>>> = _arr6;
            let _t7: bool = Self::isSorted(Default::default())?;
            System::out().println_v(_t7)?;
            Ok(())
        }
    }
}
