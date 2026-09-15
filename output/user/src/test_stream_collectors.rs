#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use java_runtime::prelude::*;
use java_runtime::java::io::*;
use java_runtime::java::lang::*;
use java_runtime::java::security::*;
use java_runtime::java::util::*;
use java_runtime::java::util::stream::*;
use java_runtime::sun::nio::ch::*;
use java_runtime::sun::nio::cs::*;
use java_runtime::sun::security::util::*;
use crate::test_stream_collectors_person::TestStreamCollectors_Person;

#[java_rta_macros::java_class(
    binary_name       = "TestStreamCollectors",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "TestStreamCollectors.java",
    inner_classes     = "TestStreamCollectors$Person:TestStreamCollectors:Person:8;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25",
    all_supertypes    = "TestStreamCollectors;java/lang/Object",
)]
#[derive(Clone, Default, PartialEq)]
pub struct TestStreamCollectors;

impl TestStreamCollectors {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "main", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn main() -> Result<()> {
        let mut _arr0: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 5i32 as usize]));
        _arr0.borrow_mut()[0i32 as usize] = Clone::clone(&String::from("hello"));
        _arr0.borrow_mut()[1i32 as usize] = Clone::clone(&String::from("world"));
        _arr0.borrow_mut()[2i32 as usize] = Clone::clone(&String::from("java"));
        _arr0.borrow_mut()[3i32 as usize] = Clone::clone(&String::from("streams"));
        _arr0.borrow_mut()[4i32 as usize] = Clone::clone(&String::from("rocks"));
        let _t1: List<Object> = Arrays::asList(Default::default())?;
        let mut words: List<Object> = _t1;
        let _t2 = words.stream()?;
        let _t3: Collector<Object, Object, Object> = Collectors::joining_seq(Object::from_any(String::from(", ").clone()))?;
        let _t4 = _t2.collect_collec(Clone::clone(&_t3))?;
        let mut joined = (_t4).downcast::<String>();
        System::out().println_v(Clone::clone(&joined))?;
        let _t5 = words.stream()?;
        let _t6: Collector<Object, Object, Object> = Collectors::joining_seq_seq_seq(Object::from_any(String::from(", ").clone()), Object::from_any(String::from("[").clone()), Object::from_any(String::from("]").clone()))?;
        let _t7 = _t5.collect_collec(Clone::clone(&_t6))?;
        let mut withBrackets = (_t7).downcast::<String>();
        System::out().println_v(Clone::clone(&withBrackets))?;
        let _t8 = words.stream()?;
        /* TODO: invokedynamic 64 */
        let _t9 = _t8.map(Clone::clone(&Object::default()))?;
        let _t10: Collector<Object, Object, Object> = Collectors::toList()?;
        let _t11 = _t9.collect_collec(Clone::clone(&_t10))?;
        let mut upper = (_t11).downcast::<List<Object>>();
        System::out().println_v(Object::from_any(upper.clone()))?;
        let _t12 = words.stream()?;
        /* TODO: invokedynamic 79 */
        let _t13 = _t12.filter(Clone::clone(&Object::default()))?;
        let _t14: Collector<Object, Object, Object> = Collectors::counting()?;
        let _t15 = _t13.collect_collec(Clone::clone(&_t14))?;
        let mut count = (_t15).downcast::<i64>();
        System::out().println_v(count)?;
        let mut _arr16: Rc<RefCell<Vec<TestStreamCollectors_Person>>> = Rc::new(RefCell::new(vec![Default::default(); 5i32 as usize]));
        _arr16.borrow_mut()[0i32 as usize] = Clone::clone(&TestStreamCollectors_Person::new(Clone::clone(&String::from("Alice")), 30i32, Clone::clone(&String::from("Engineering")))?);
        _arr16.borrow_mut()[1i32 as usize] = Clone::clone(&TestStreamCollectors_Person::new(Clone::clone(&String::from("Bob")), 25i32, Clone::clone(&String::from("Marketing")))?);
        _arr16.borrow_mut()[2i32 as usize] = Clone::clone(&TestStreamCollectors_Person::new(Clone::clone(&String::from("Charlie")), 28i32, Clone::clone(&String::from("Engineering")))?);
        _arr16.borrow_mut()[3i32 as usize] = Clone::clone(&TestStreamCollectors_Person::new(Clone::clone(&String::from("Dave")), 35i32, Clone::clone(&String::from("Marketing")))?);
        _arr16.borrow_mut()[4i32 as usize] = Clone::clone(&TestStreamCollectors_Person::new(Clone::clone(&String::from("Eve")), 27i32, Clone::clone(&String::from("Engineering")))?);
        let _t17: List<Object> = Arrays::asList(Default::default())?;
        let mut people: List<Object> = _t17;
        let _t18 = people.stream()?;
        /* TODO: invokedynamic 118 */
        let _t19: Collector<Object, Object, Object> = Collectors::groupingBy_functi(Clone::clone(&Object::default()))?;
        let _t20 = _t18.collect_collec(Clone::clone(&_t19))?;
        let mut byDept = (_t20).downcast::<Map<Object, Object>>();
        let _t21 = byDept.get(Object::from_any(String::from("Engineering").clone()))?;
        let _t22 = (_t21).downcast::<List<Object>>().stream()?;
        /* TODO: invokedynamic 129 */
        let _t23 = _t22.map(Clone::clone(&Object::default()))?;
        let _t24 = _t23.sorted()?;
        let _t25: Collector<Object, Object, Object> = Collectors::toList()?;
        let _t26 = _t24.collect_collec(Clone::clone(&_t25))?;
        let mut engNames = (_t26).downcast::<List<Object>>();
        System::out().println_v(Object::from_any(engNames.clone()))?;
        let _t27 = byDept.get(Object::from_any(String::from("Marketing").clone()))?;
        let _t28 = (_t27).downcast::<List<Object>>().stream()?;
        /* TODO: invokedynamic 133 */
        let _t29 = _t28.map(Clone::clone(&Object::default()))?;
        let _t30 = _t29.sorted()?;
        let _t31: Collector<Object, Object, Object> = Collectors::toList()?;
        let _t32 = _t30.collect_collec(Clone::clone(&_t31))?;
        let mut mktNames = (_t32).downcast::<List<Object>>();
        System::out().println_v(Object::from_any(mktNames.clone()))?;
        let _t33 = words.stream()?;
        /* TODO: invokedynamic 134 */
        let _t34: Collector<Object, Object, Object> = Collectors::partitioningBy_predic(Clone::clone(&Object::default()))?;
        let _t35 = _t33.collect_collec(Clone::clone(&_t34))?;
        let mut byLength = (_t35).downcast::<Map<Object, Object>>();
        let _t36 = byLength.get(1i32.into())?;
        let _t37 = (_t36).downcast::<List<Object>>().stream()?;
        let _t38 = _t37.sorted()?;
        let _t39: Collector<Object, Object, Object> = Collectors::toList()?;
        let _t40 = _t38.collect_collec(Clone::clone(&_t39))?;
        let mut longWords = (_t40).downcast::<List<Object>>();
        let _t41 = byLength.get(0i32.into())?;
        let _t42 = (_t41).downcast::<List<Object>>().stream()?;
        let _t43 = _t42.sorted()?;
        let _t44: Collector<Object, Object, Object> = Collectors::toList()?;
        let _t45 = _t43.collect_collec(Clone::clone(&_t44))?;
        let mut shortWords = (_t45).downcast::<List<Object>>();
        System::out().println_v(Object::from_any(longWords.clone()))?;
        System::out().println_v(Object::from_any(shortWords.clone()))?;
        let mut _arr46: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![Default::default(); 5i32 as usize]));
        _arr46.borrow_mut()[0i32 as usize] = 1i32;
        _arr46.borrow_mut()[1i32 as usize] = 2i32;
        _arr46.borrow_mut()[2i32 as usize] = 3i32;
        _arr46.borrow_mut()[3i32 as usize] = 4i32;
        _arr46.borrow_mut()[4i32 as usize] = 5i32;
        let _t47: List<Object> = Arrays::asList(Default::default())?;
        let mut nums: List<Object> = _t47;
        let _t48 = nums.stream()?;
        /* TODO: invokedynamic 150 */
        let _t49: Collector<Object, Object, Object> = Collectors::summingInt(Clone::clone(&Object::default()))?;
        let _t50 = _t48.collect_collec(Clone::clone(&_t49))?;
        let mut total = (_t50).downcast::<i32>();
        System::out().println_v(total)?;
        Ok(())
    }
}
