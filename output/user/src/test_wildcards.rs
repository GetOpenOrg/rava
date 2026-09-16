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
    #[binary_name       = "TestWildcards"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TestWildcards.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "TestWildcards;java/lang/Object"]

    pub struct TestWildcards;

    impl TestWildcards {
        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            Ok(this)
        }

        #[java_method(name = "sumList", descriptor = "(Ljava/util/List;)D", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<+Ljava/lang/Number;>;)D")]
        pub fn sumList(mut list: Object) -> Result<f64> {
            let mut total: f64 = 0f64;
            let _vdispatch0: Object = if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.iterator()? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.iterator()? } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.iterator()? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.iterator()? } else if let Some(_d) = list.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.iterator()? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.iterator()? } else if let Some(_d) = list.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.iterator()? } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.iterator()? } else if let Some(_d) = list.0.as_any().downcast_ref::<Object>() { _d.iterator()? } else if let Some(__f) = list.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut local_3: Object = _vdispatch0;
            loop {
                let _vdispatch1: bool = if let Some(_d) = local_3.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.hasNext()? } else if let Some(_d) = local_3.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.hasNext()? } else if let Some(_d) = local_3.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.hasNext()? } else if let Some(_d) = local_3.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(_d) = local_3.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.hasNext()? } else if let Some(_d) = local_3.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(__f) = local_3.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<bool>>>() { (__f)()? } else { Default::default() };
                if !(_vdispatch1) { break; }
                let _vdispatch1: Object = if let Some(_d) = local_3.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.next()? } else if let Some(_d) = local_3.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.next()? } else if let Some(_d) = local_3.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.next()? } else if let Some(_d) = local_3.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(_d) = local_3.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.next()? } else if let Some(_d) = local_3.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(__f) = local_3.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let mut n = (_vdispatch1).downcast::<Number>();
                let _t2: f64 = n.doubleValue()?;
                total = (total+_t2);
            }
            Ok(total)
        }

        #[java_method(name = "addNumbers", descriptor = "(Ljava/util/List;II)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<-Ljava/lang/Integer;>;II)V")]
        pub fn addNumbers(mut list: Object, mut from: i32, mut to: i32) -> Result<()> {
            let mut i: i32 = from;
            loop {
                if i > to { break; }
                let _vdispatch0: bool = if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.add(i.into())? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.add(i.into())? } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.add(i.into())? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.add(i.into())? } else if let Some(_d) = list.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.add(i.into())? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.add(i.into())? } else if let Some(_d) = list.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.add(i.into())? } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.add(i.into())? } else if let Some(_d) = list.0.as_any().downcast_ref::<Object>() { _d.add(i.into())? } else if let Some(__f) = list.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<bool>>>() { (__f)(i.into())? } else { Default::default() };
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "printAll", descriptor = "(Ljava/util/List;)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<*>;)V")]
        pub fn printAll(mut list: Object) -> Result<()> {
            let mut sb = StringBuilder::new()?;
            let mut i: i32 = 0i32;
            loop {
                let _vdispatch0: i32 = if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.size()? } else if let Some(_d) = list.0.as_any().downcast_ref::<Object>() { _d.size()? } else if let Some(__f) = list.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
                if i >= _vdispatch0 { break; }
                if (i>0) {
                    let _t0 = sb.append_str(Clone::clone(&String::from(" ")))?;
                }
                let _vdispatch0: Object = if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.get(i)? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.get(i)? } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.get(i)? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.get(i)? } else if let Some(_d) = list.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.get(i)? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.get(i)? } else if let Some(_d) = list.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.get(i)? } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.get(i)? } else if let Some(_d) = list.0.as_any().downcast_ref::<Object>() { _d.get(i)? } else if let Some(__f) = list.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<Object>>>() { (__f)(i)? } else { Default::default() };
                let _t1 = sb.append_obj(Clone::clone(&_vdispatch0))?;
                i = i.wrapping_add(1i32);
            }
            System::out().println_v(Object::from_any(sb.clone()))?;
            Ok(())
        }

        #[java_method(name = "findMax", descriptor = "(Ljava/util/List;)Ljava/lang/Comparable;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T::Ljava/lang/Comparable<TT;>;>(Ljava/util/List<TT;>;)TT;")]
        pub fn findMax(mut list: Object) -> Result<Object> {
            let _vdispatch0: Object = if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.get(0i32)? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.get(0i32)? } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.get(0i32)? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.get(0i32)? } else if let Some(_d) = list.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.get(0i32)? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.get(0i32)? } else if let Some(_d) = list.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.get(0i32)? } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.get(0i32)? } else if let Some(_d) = list.0.as_any().downcast_ref::<Object>() { _d.get(0i32)? } else if let Some(__f) = list.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<Object>>>() { (__f)(0i32)? } else { Default::default() };
            let mut max: Object = _vdispatch0;
            let _vdispatch1: Object = if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.iterator()? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.iterator()? } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.iterator()? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.iterator()? } else if let Some(_d) = list.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.iterator()? } else if let Some(_d) = list.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.iterator()? } else if let Some(_d) = list.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.iterator()? } else if let Some(_d) = list.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.iterator()? } else if let Some(_d) = list.0.as_any().downcast_ref::<Object>() { _d.iterator()? } else if let Some(__f) = list.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut local_2: Object = _vdispatch1;
            loop {
                let _vdispatch2: bool = if let Some(_d) = local_2.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.hasNext()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.hasNext()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.hasNext()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.hasNext()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(__f) = local_2.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<bool>>>() { (__f)()? } else { Default::default() };
                if !(_vdispatch2) { break; }
                let _vdispatch2: Object = if let Some(_d) = local_2.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.next()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.next()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.next()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.next()? } else if let Some(_d) = local_2.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(__f) = local_2.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let mut item: Object = _vdispatch2;
                let _vdispatch3: i32 = if let Some(_d) = item.0.as_any().downcast_ref::<BuddhistCalendar>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<UTF_8>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<GregorianCalendar>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<JapaneseImperialCalendar>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<HeapByteBuffer>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Unicode>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<US_ASCII>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<ISO_8859_1>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Collector_Characteristics>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Comparators_NaturalOrderComparator>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Locale_Category>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Normalizer_Form>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<StreamShape>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<MatchOps_MatchKind>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Pattern_Qtype>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Formatter_BigDecimalLayoutForm>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<DayOfWeek>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<RoundingMode>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<ZoneOffsetTransitionRule_TimeDefinition>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Month>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<ChronoField>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Character_UnicodeScript>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<StreamOpFlag>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<TestSwitchEnum_Season>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<TestEnumMethods_Planet>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<TestEnumBasic_Day>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<LocalTime>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<ZoneOffsetTransition>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Instant>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<ZoneOffset>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<BigDecimal>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<BigInteger>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Short>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Byte>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Date>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Calendar>() { _d.compareTo_obj(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<CharBuffer>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<ByteBuffer>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Charset>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Float>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<i64>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<bool>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<f64>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<TestComparable_Version>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<TestComparable_Student>() { _d.compareTo(Clone::clone(&max))? } else if let Some(_d) = item.0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&max))? } else if let Some(__f) = item.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&max))? } else { Default::default() };
                if (_vdispatch3>0) {
                    max = item;
                }
            }
            Ok(max)
        }

        #[java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn main() -> Result<()> {
            let mut ints = ArrayList::<Object>::new()?;
            let _t0 = ints.add_obj(1i32.into())?;
            let _t1 = ints.add_obj(2i32.into())?;
            let _t2 = ints.add_obj(3i32.into())?;
            let _t3: f64 = Self::sumList(Object::from_any(ints.clone()))?;
            System::out().println_v(_t3)?;
            let mut doubles = ArrayList::<Object>::new()?;
            let _t4 = doubles.add_obj(1.5f64.into())?;
            let _t5 = doubles.add_obj(2.5f64.into())?;
            let _t6 = doubles.add_obj(3.0f64.into())?;
            let _t7: f64 = Self::sumList(Object::from_any(doubles.clone()))?;
            System::out().println_v(_t7)?;
            let mut numbers = ArrayList::<Object>::new()?;
            Self::addNumbers(Object::from_any(numbers.clone()), 1i32, 5i32)?;
            Self::printAll(Object::from_any(numbers.clone()))?;
            let mut objects = ArrayList::<Object>::new()?;
            Self::addNumbers(Object::from_any(objects.clone()), 10i32, 13i32)?;
            Self::printAll(Object::from_any(objects.clone()))?;
            let mut strs = ArrayList::<Object>::new()?;
            let _t8 = strs.add_obj(Object::from_any(String::from("banana").clone()))?;
            let _t9 = strs.add_obj(Object::from_any(String::from("apple").clone()))?;
            let _t10 = strs.add_obj(Object::from_any(String::from("cherry").clone()))?;
            Self::printAll(Object::from_any(strs.clone()))?;
            let _t11: Object = Self::findMax(Object::from_any(ints.clone()))?;
            System::out().println_v(Clone::clone(&_t11))?;
            let _t12: Object = Self::findMax(Object::from_any(strs.clone()))?;
            System::out().println_v(Clone::clone(&(_t12).downcast::<String>()))?;
            let mut mixedInts = ArrayList::<Object>::new()?;
            let _t13 = mixedInts.add_obj(10i32.into())?;
            let _t14 = mixedInts.add_obj(20i32.into())?;
            let _t15 = mixedInts.add_obj(5i32.into())?;
            let _t16: f64 = Self::sumList(Object::from_any(mixedInts.clone()))?;
            System::out().println_v(_t16)?;
            Ok(())
        }
    }
}
