#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::math::*;
use crate::java::nio::*;
use crate::java::nio::charset::*;
use crate::java::security::*;
use crate::java::text::*;
use crate::java::text::spi::*;
use crate::java::time::*;
use crate::java::time::chrono::*;
use crate::java::time::temporal::*;
use crate::java::time::zone::*;
use crate::java::util::*;
use crate::java::util::concurrent::*;
use crate::java::util::concurrent::atomic::*;
use crate::java::util::concurrent::locks::*;
use crate::java::util::function::*;
use crate::java::util::regex::*;
use crate::java::util::spi::*;
use crate::java::util::stream::*;
use crate::java::util::zip::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::reflect::misc::*;
use crate::sun::security::action::*;
use crate::sun::security::util::*;
use crate::sun::text::*;
use crate::sun::util::*;
use crate::sun::util::calendar::*;
use crate::sun::util::locale::*;
use crate::sun::util::locale::provider::*;
use crate::sun::util::spi::*;
use crate::java::text::Normalizer;
use crate::jdk::internal::util::ArraysSupport;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/Arrays"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Arrays.java"]
    #[inner_classes     = "java/util/Arrays$NaturalOrder:java/util/Arrays:NaturalOrder:24;java/util/ArraysParallelSortHelpers$FJObject:java/util/ArraysParallelSortHelpers:FJObject:24;java/util/ArraysParallelSortHelpers$FJObject$Sorter:java/util/ArraysParallelSortHelpers$FJObject:Sorter:24;java/util/Arrays$LegacyMergeSort:java/util/Arrays:LegacyMergeSort:24;java/util/ArrayPrefixHelpers$CumulateTask:java/util/ArrayPrefixHelpers:CumulateTask:24;java/util/ArrayPrefixHelpers$LongCumulateTask:java/util/ArrayPrefixHelpers:LongCumulateTask:24;java/util/ArrayPrefixHelpers$DoubleCumulateTask:java/util/ArrayPrefixHelpers:DoubleCumulateTask:24;java/util/ArrayPrefixHelpers$IntCumulateTask:java/util/ArrayPrefixHelpers:IntCumulateTask:24;java/util/Arrays$ArrayList:java/util/Arrays:ArrayList:10;java/util/Spliterator$OfInt:java/util/Spliterator:OfInt:1545;java/util/Spliterator$OfLong:java/util/Spliterator:OfLong:1545;java/util/Spliterator$OfDouble:java/util/Spliterator:OfDouble:1545;java/util/Arrays$ArrayItr:java/util/Arrays:ArrayItr:10;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Arrays"]

    pub struct Arrays;

    impl Arrays {
        #[cfg_attr(any(), java_field(name = "MIN_ARRAY_SORT_GRAN", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "8192"))]
        // static field: MIN_ARRAY_SORT_GRAN:I
        pub fn MIN_ARRAY_SORT_GRAN() -> i32 {
            8192
        }

        #[cfg_attr(any(), java_field(name = "INSERTIONSORT_THRESHOLD", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "7"))]
        // static field: INSERTIONSORT_THRESHOLD:I
        pub fn INSERTIONSORT_THRESHOLD() -> i32 {
            7
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/Arrays.<init>:()V")
        }

        #[java_method(name = "sort", descriptor = "([I)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: sort([I)V
        pub fn sort_arr_i(mut a: Rc<RefCell<Vec<i32>>>) -> Result<()> {
            DualPivotQuicksort::sort_arr_i_i_i_i(Clone::clone(&a), 0i32, 0i32, (a.borrow().len() as i32))?;
            Ok(())
        }

        #[java_method(name = "sort", descriptor = "([III)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_i_i_i(a: Rc<RefCell<Vec<i32>>>, fromIndex: i32, toIndex: i32) -> Result<()> {
            panic!("stub: java/util/Arrays.sort:([III)V")
        }

        #[java_method(name = "sort", descriptor = "([J)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_l(a: Rc<RefCell<Vec<i64>>>) -> Result<()> {
            panic!("stub: java/util/Arrays.sort:([J)V")
        }

        #[java_method(name = "sort", descriptor = "([JII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_l_i_i(a: Rc<RefCell<Vec<i64>>>, fromIndex: i32, toIndex: i32) -> Result<()> {
            panic!("stub: java/util/Arrays.sort:([JII)V")
        }

        #[java_method(name = "sort", descriptor = "([S)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_s(a: Rc<RefCell<Vec<i16>>>) -> Result<()> {
            panic!("stub: java/util/Arrays.sort:([S)V")
        }

        #[java_method(name = "sort", descriptor = "([SII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_s_i_i(a: Rc<RefCell<Vec<i16>>>, fromIndex: i32, toIndex: i32) -> Result<()> {
            panic!("stub: java/util/Arrays.sort:([SII)V")
        }

        #[java_method(name = "sort", descriptor = "([C)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_c(a: Rc<RefCell<Vec<u16>>>) -> Result<()> {
            panic!("stub: java/util/Arrays.sort:([C)V")
        }

        #[java_method(name = "sort", descriptor = "([CII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_c_i_i(a: Rc<RefCell<Vec<u16>>>, fromIndex: i32, toIndex: i32) -> Result<()> {
            panic!("stub: java/util/Arrays.sort:([CII)V")
        }

        #[java_method(name = "sort", descriptor = "([B)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_b(a: Rc<RefCell<Vec<i8>>>) -> Result<()> {
            panic!("stub: java/util/Arrays.sort:([B)V")
        }

        #[java_method(name = "sort", descriptor = "([BII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_b_i_i(a: Rc<RefCell<Vec<i8>>>, fromIndex: i32, toIndex: i32) -> Result<()> {
            panic!("stub: java/util/Arrays.sort:([BII)V")
        }

        #[java_method(name = "sort", descriptor = "([F)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_f(a: Rc<RefCell<Vec<f32>>>) -> Result<()> {
            panic!("stub: java/util/Arrays.sort:([F)V")
        }

        #[java_method(name = "sort", descriptor = "([FII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_f_i_i(a: Rc<RefCell<Vec<f32>>>, fromIndex: i32, toIndex: i32) -> Result<()> {
            panic!("stub: java/util/Arrays.sort:([FII)V")
        }

        #[java_method(name = "sort", descriptor = "([D)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_d(a: Rc<RefCell<Vec<f64>>>) -> Result<()> {
            panic!("stub: java/util/Arrays.sort:([D)V")
        }

        #[java_method(name = "sort", descriptor = "([DII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_d_i_i(a: Rc<RefCell<Vec<f64>>>, fromIndex: i32, toIndex: i32) -> Result<()> {
            panic!("stub: java/util/Arrays.sort:([DII)V")
        }

        #[java_method(name = "parallelSort", descriptor = "([B)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelSort_arr_b(a: Rc<RefCell<Vec<i8>>>) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSort:([B)V")
        }

        #[java_method(name = "parallelSort", descriptor = "([BII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelSort_arr_b_i_i(a: Rc<RefCell<Vec<i8>>>, fromIndex: i32, toIndex: i32) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSort:([BII)V")
        }

        #[java_method(name = "parallelSort", descriptor = "([C)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelSort_arr_c(a: Rc<RefCell<Vec<u16>>>) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSort:([C)V")
        }

        #[java_method(name = "parallelSort", descriptor = "([CII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelSort_arr_c_i_i(a: Rc<RefCell<Vec<u16>>>, fromIndex: i32, toIndex: i32) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSort:([CII)V")
        }

        #[java_method(name = "parallelSort", descriptor = "([S)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelSort_arr_s(a: Rc<RefCell<Vec<i16>>>) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSort:([S)V")
        }

        #[java_method(name = "parallelSort", descriptor = "([SII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelSort_arr_s_i_i(a: Rc<RefCell<Vec<i16>>>, fromIndex: i32, toIndex: i32) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSort:([SII)V")
        }

        #[java_method(name = "parallelSort", descriptor = "([I)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelSort_arr_i(a: Rc<RefCell<Vec<i32>>>) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSort:([I)V")
        }

        #[java_method(name = "parallelSort", descriptor = "([III)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelSort_arr_i_i_i(a: Rc<RefCell<Vec<i32>>>, fromIndex: i32, toIndex: i32) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSort:([III)V")
        }

        #[java_method(name = "parallelSort", descriptor = "([J)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelSort_arr_l(a: Rc<RefCell<Vec<i64>>>) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSort:([J)V")
        }

        #[java_method(name = "parallelSort", descriptor = "([JII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelSort_arr_l_i_i(a: Rc<RefCell<Vec<i64>>>, fromIndex: i32, toIndex: i32) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSort:([JII)V")
        }

        #[java_method(name = "parallelSort", descriptor = "([F)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelSort_arr_f(a: Rc<RefCell<Vec<f32>>>) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSort:([F)V")
        }

        #[java_method(name = "parallelSort", descriptor = "([FII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelSort_arr_f_i_i(a: Rc<RefCell<Vec<f32>>>, fromIndex: i32, toIndex: i32) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSort:([FII)V")
        }

        #[java_method(name = "parallelSort", descriptor = "([D)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelSort_arr_d(a: Rc<RefCell<Vec<f64>>>) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSort:([D)V")
        }

        #[java_method(name = "parallelSort", descriptor = "([DII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelSort_arr_d_i_i(a: Rc<RefCell<Vec<f64>>>, fromIndex: i32, toIndex: i32) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSort:([DII)V")
        }

        #[java_method(name = "rangeCheck", descriptor = "(III)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn rangeCheck(mut arrayLength: i32, mut fromIndex: i32, mut toIndex: i32) -> Result<()> {
            if fromIndex > toIndex {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("fromIndex(")))?;
                let _t1 = _t0.append_i(fromIndex)?;
                let _t2 = _t1.append_str(Clone::clone(&String::from(") > toIndex(")))?;
                let _t3 = _t2.append_i(toIndex)?;
                let _t4 = _t3.append_str(Clone::clone(&String::from(")")))?;
                let _t5 = _t4.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (fromIndex<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if toIndex > arrayLength {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "parallelSort", descriptor = "([Ljava/lang/Comparable;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T::Ljava/lang/Comparable<-TT;>;>([TT;)V")]
        pub fn parallelSort_arr_cmp(a: Rc<RefCell<Vec<Object>>>) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSort:([Ljava/lang/Comparable;)V")
        }

        #[java_method(name = "parallelSort", descriptor = "([Ljava/lang/Comparable;II)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T::Ljava/lang/Comparable<-TT;>;>([TT;II)V")]
        pub fn parallelSort_arr_cmp_i_i(a: Rc<RefCell<Vec<Object>>>, fromIndex: i32, toIndex: i32) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSort:([Ljava/lang/Comparable;II)V")
        }

        #[java_method(name = "parallelSort", descriptor = "([Ljava/lang/Object;Ljava/util/Comparator;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;Ljava/util/Comparator<-TT;>;)V")]
        pub fn parallelSort_arr_obj_compar(a: Rc<RefCell<Vec<Object>>>, cmp: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSort:([Ljava/lang/Object;Ljava/util/Comparator;)V")
        }

        #[java_method(name = "parallelSort", descriptor = "([Ljava/lang/Object;IILjava/util/Comparator;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;IILjava/util/Comparator<-TT;>;)V")]
        pub fn parallelSort_arr_obj_i_i_compar(a: Rc<RefCell<Vec<Object>>>, fromIndex: i32, toIndex: i32, cmp: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSort:([Ljava/lang/Object;IILjava/util/Comparator;)V")
        }

        #[java_method(name = "sort", descriptor = "([Ljava/lang/Object;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: sort([Ljava/lang/Object;)V
        pub fn sort_arr_obj(mut a: Rc<RefCell<Vec<Object>>>) -> Result<()> {
            if Arrays_LegacyMergeSort::userRequested() {
                Arrays::legacyMergeSort_arr_obj(Clone::clone(&a))?;
            } else {
                ComparableTimSort::sort(Clone::clone(&a), 0i32, (a.borrow().len() as i32), Default::default(), 0i32, 0i32)?;
            }
            Ok(())
        }

        #[java_method(name = "legacyMergeSort", descriptor = "([Ljava/lang/Object;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: legacyMergeSort([Ljava/lang/Object;)V
        pub fn legacyMergeSort_arr_obj(mut a: Rc<RefCell<Vec<Object>>>) -> Result<()> {
            let _t0: Object = Object::from_any(a.clone());
            let mut aux = (_t0).downcast::<Rc<RefCell<Vec<Object>>>>();
            Arrays::mergeSort_arr_obj_arr_obj_i_i_i(Clone::clone(&aux), Clone::clone(&a), 0i32, (a.borrow().len() as i32), 0i32)?;
            Ok(())
        }

        #[java_method(name = "sort", descriptor = "([Ljava/lang/Object;II)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: sort([Ljava/lang/Object;II)V
        pub fn sort_arr_obj_i_i(mut a: Rc<RefCell<Vec<Object>>>, mut fromIndex: i32, mut toIndex: i32) -> Result<()> {
            Arrays::rangeCheck((a.borrow().len() as i32), fromIndex, toIndex)?;
            if Arrays_LegacyMergeSort::userRequested() {
                Arrays::legacyMergeSort_arr_obj_i_i(Clone::clone(&a), fromIndex, toIndex)?;
            } else {
                ComparableTimSort::sort(Clone::clone(&a), fromIndex, toIndex, Default::default(), 0i32, 0i32)?;
            }
            Ok(())
        }

        #[java_method(name = "legacyMergeSort", descriptor = "([Ljava/lang/Object;II)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: legacyMergeSort([Ljava/lang/Object;II)V
        pub fn legacyMergeSort_arr_obj_i_i(mut a: Rc<RefCell<Vec<Object>>>, mut fromIndex: i32, mut toIndex: i32) -> Result<()> {
            let _t0: Rc<RefCell<Vec<Object>>> = Arrays::copyOfRange_arr_obj_i_i(Clone::clone(&a), fromIndex, toIndex)?;
            let mut aux: Rc<RefCell<Vec<Object>>> = _t0;
            Arrays::mergeSort_arr_obj_arr_obj_i_i_i(Clone::clone(&aux), Clone::clone(&a), fromIndex, toIndex, (fromIndex).wrapping_neg())?;
            Ok(())
        }

        #[java_method(name = "mergeSort", descriptor = "([Ljava/lang/Object;[Ljava/lang/Object;III)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: mergeSort([Ljava/lang/Object;[Ljava/lang/Object;III)V
        pub fn mergeSort_arr_obj_arr_obj_i_i_i(mut src: Rc<RefCell<Vec<Object>>>, mut dest: Rc<RefCell<Vec<Object>>>, mut low: i32, mut high: i32, mut off: i32) -> Result<()> {
            let mut length = (high).wrapping_sub(low);
            let mut i: i32 = low;
            loop {
                if i >= high { break; }
                let mut j: i32 = i;
                loop {
                    if j <= low { break; }
                    let _vdispatch0: i32 = if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<BuddhistCalendar>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<UTF_8>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<GregorianCalendar>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<JapaneseImperialCalendar>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<HeapCharBuffer>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<HeapByteBuffer>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Unicode>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<US_ASCII>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<ISO_8859_1>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Collector_Characteristics>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Comparators_NaturalOrderComparator>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Locale_Category>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Normalizer_Form>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<StreamShape>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<MatchOps_MatchKind>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Pattern_Qtype>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Formatter_BigDecimalLayoutForm>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<DayOfWeek>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<RoundingMode>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<ZoneOffsetTransitionRule_TimeDefinition>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Month>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<ChronoField>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Character_UnicodeScript>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<StreamOpFlag>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<TestSwitchEnum_Season>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<TestEnumMethods_Planet>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<TestEnumBasic_Day>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<LocalTime>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<ZoneOffsetTransition>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Instant>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<ZoneOffset>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<BigDecimal>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<BigInteger>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Short>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Byte>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Date>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Calendar>() { _d.compareTo_obj(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<CharBuffer>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<ByteBuffer>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Charset>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Float>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<i64>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<bool>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<f64>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<TestComparable_Version>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<TestComparable_Student>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(__f) = Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else { Default::default() };
                    if (_vdispatch0>0) {
                        Arrays::swap(Clone::clone(&dest), j, (j).wrapping_sub(1i32))?;
                        j = j.wrapping_sub(1i32);
                        continue;
                    }
                    break;
                }
                i = i.wrapping_add(1i32);
            }
            return Ok(());
            i = low;
            let mut j: i32 = high;
            low = (low).wrapping_add(off);
            high = (high).wrapping_add(off);
            let mut mid = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
            Arrays::mergeSort_arr_obj_arr_obj_i_i_i(Clone::clone(&dest), Clone::clone(&src), low, mid, (off).wrapping_neg())?;
            Arrays::mergeSort_arr_obj_arr_obj_i_i_i(Clone::clone(&dest), Clone::clone(&src), mid, high, (off).wrapping_neg())?;
            let _vdispatch0: i32 = if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<BuddhistCalendar>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<UTF_8>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<GregorianCalendar>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<JapaneseImperialCalendar>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<HeapCharBuffer>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<HeapByteBuffer>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Unicode>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<US_ASCII>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<ISO_8859_1>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Collector_Characteristics>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Comparators_NaturalOrderComparator>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Locale_Category>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Normalizer_Form>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<StreamShape>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<MatchOps_MatchKind>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Pattern_Qtype>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Formatter_BigDecimalLayoutForm>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<DayOfWeek>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<RoundingMode>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<ZoneOffsetTransitionRule_TimeDefinition>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Month>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<ChronoField>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Character_UnicodeScript>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<StreamOpFlag>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<TestSwitchEnum_Season>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<TestEnumMethods_Planet>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<TestEnumBasic_Day>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<LocalTime>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<ZoneOffsetTransition>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Instant>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<ZoneOffset>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<BigDecimal>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<BigInteger>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Short>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Byte>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Date>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Calendar>() { _d.compareTo_obj(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<CharBuffer>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<ByteBuffer>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Charset>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Float>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<i64>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<bool>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<f64>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<TestComparable_Version>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<TestComparable_Student>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(__f) = Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize]).0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else { Default::default() };
            if (_vdispatch0<=0) {
                System::arraycopy(Object::from_any(src.clone()), low, Object::from_any(dest.clone()), i, length)?;
                return Ok(());
            }
            let mut i: i32 = i;
            let mut p: i32 = low;
            let mut q: i32 = mid;
            loop {
                if i >= j { break; }
                if p < mid {
                    let _vdispatch1: i32 = if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<BuddhistCalendar>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<UTF_8>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<GregorianCalendar>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<JapaneseImperialCalendar>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<HeapCharBuffer>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<HeapByteBuffer>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Unicode>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<US_ASCII>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<ISO_8859_1>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Collector_Characteristics>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Comparators_NaturalOrderComparator>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Locale_Category>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Normalizer_Form>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<StreamShape>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<MatchOps_MatchKind>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Pattern_Qtype>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Thread_State>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Formatter_BigDecimalLayoutForm>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<DayOfWeek>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<RoundingMode>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<ZoneOffsetTransitionRule_TimeDefinition>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Month>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<ChronoField>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Character_UnicodeScript>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<StreamOpFlag>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<TestSwitchEnum_Season>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<TestEnumMethods_Planet>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<TestEnumBasic_Day>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<LocalTime>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<ZoneOffsetTransition>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Instant>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<ZoneOffset>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<BigDecimal>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<BigInteger>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Short>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Byte>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Date>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Calendar>() { _d.compareTo_obj(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<CharBuffer>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<ByteBuffer>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Charset>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Float>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Character>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<i64>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<bool>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<f64>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Enum<Object>>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<i32>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<String>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<StringBuilder>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<TestComparable_Version>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<TestComparable_Student>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<Object>() { _d.compareTo(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(__f) = Clone::clone(&src.borrow()[p as usize]).0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else { Default::default() };
                    if (_vdispatch1<=0) {
                        p = p.wrapping_add(1i32);
                        let _aastore_tmp2 = Clone::clone(&Clone::clone(&src.borrow()[p as usize]));
                        dest.borrow_mut()[i as usize] = _aastore_tmp2;
                    } else {
                        q = q.wrapping_add(1i32);
                        let _aastore_tmp2 = Clone::clone(&Clone::clone(&src.borrow()[q as usize]));
                        dest.borrow_mut()[i as usize] = _aastore_tmp2;
                    }
                } else {
                    q = q.wrapping_add(1i32);
                    let _aastore_tmp1 = Clone::clone(&Clone::clone(&src.borrow()[q as usize]));
                    dest.borrow_mut()[i as usize] = _aastore_tmp1;
                }
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "swap", descriptor = "([Ljava/lang/Object;II)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn swap(mut x: Rc<RefCell<Vec<Object>>>, mut a: i32, mut b: i32) -> Result<()> {
            let mut t = Clone::clone(&x.borrow()[a as usize]);
            let _aastore_tmp0 = Clone::clone(&Clone::clone(&x.borrow()[b as usize]));
            x.borrow_mut()[a as usize] = _aastore_tmp0;
            x.borrow_mut()[b as usize] = Clone::clone(&t);
            Ok(())
        }

        #[java_method(name = "sort", descriptor = "([Ljava/lang/Object;Ljava/util/Comparator;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;Ljava/util/Comparator<-TT;>;)V")]
        // java: sort([Ljava/lang/Object;Ljava/util/Comparator;)V
        pub fn sort_arr_obj_compar(mut a: Rc<RefCell<Vec<Object>>>, mut c: Object) -> Result<()> {
            if _is_jnull(&c) {
                Arrays::sort_arr_obj(Clone::clone(&a))?;
            } else {
                if Arrays_LegacyMergeSort::userRequested() {
                    Arrays::legacyMergeSort_arr_obj_compar(Clone::clone(&a), Clone::clone(&c))?;
                } else {
                    TimSort::<Object>::sort(Default::default(), 0i32, (a.borrow().len() as i32), Clone::clone(&c), Default::default(), 0i32, 0i32)?;
                }
            }
            Ok(())
        }

        #[java_method(name = "legacyMergeSort", descriptor = "([Ljava/lang/Object;Ljava/util/Comparator;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;Ljava/util/Comparator<-TT;>;)V")]
        // java: legacyMergeSort([Ljava/lang/Object;Ljava/util/Comparator;)V
        pub fn legacyMergeSort_arr_obj_compar(mut a: Rc<RefCell<Vec<Object>>>, mut c: Object) -> Result<()> {
            let _t0: Object = Object::from_any(a.clone());
            let mut aux = (_t0).downcast::<Rc<RefCell<Vec<Object>>>>();
            if _is_jnull(&c) {
                Arrays::mergeSort_arr_obj_arr_obj_i_i_i(Clone::clone(&aux), Clone::clone(&a), 0i32, (a.borrow().len() as i32), 0i32)?;
            } else {
                Arrays::mergeSort_arr_obj_arr_obj_i_i_i_compar(Clone::clone(&aux), Clone::clone(&a), 0i32, (a.borrow().len() as i32), 0i32, Clone::clone(&c))?;
            }
            Ok(())
        }

        #[java_method(name = "sort", descriptor = "([Ljava/lang/Object;IILjava/util/Comparator;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;IILjava/util/Comparator<-TT;>;)V")]
        // java: sort([Ljava/lang/Object;IILjava/util/Comparator;)V
        pub fn sort_arr_obj_i_i_compar(mut a: Rc<RefCell<Vec<Object>>>, mut fromIndex: i32, mut toIndex: i32, mut c: Object) -> Result<()> {
            if _is_jnull(&c) {
                Arrays::sort_arr_obj_i_i(Clone::clone(&a), fromIndex, toIndex)?;
            } else {
                Arrays::rangeCheck((a.borrow().len() as i32), fromIndex, toIndex)?;
                if Arrays_LegacyMergeSort::userRequested() {
                    Arrays::legacyMergeSort_arr_obj_i_i_compar(Clone::clone(&a), fromIndex, toIndex, Clone::clone(&c))?;
                } else {
                    TimSort::<Object>::sort(Default::default(), fromIndex, toIndex, Clone::clone(&c), Default::default(), 0i32, 0i32)?;
                }
            }
            Ok(())
        }

        #[java_method(name = "legacyMergeSort", descriptor = "([Ljava/lang/Object;IILjava/util/Comparator;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;IILjava/util/Comparator<-TT;>;)V")]
        // java: legacyMergeSort([Ljava/lang/Object;IILjava/util/Comparator;)V
        pub fn legacyMergeSort_arr_obj_i_i_compar(mut a: Rc<RefCell<Vec<Object>>>, mut fromIndex: i32, mut toIndex: i32, mut c: Object) -> Result<()> {
            let _t0: Rc<RefCell<Vec<Object>>> = Arrays::copyOfRange_arr_obj_i_i(Clone::clone(&a), fromIndex, toIndex)?;
            let mut aux: Rc<RefCell<Vec<Object>>> = _t0;
            if _is_jnull(&c) {
                Arrays::mergeSort_arr_obj_arr_obj_i_i_i(Clone::clone(&aux), Clone::clone(&a), fromIndex, toIndex, (fromIndex).wrapping_neg())?;
            } else {
                Arrays::mergeSort_arr_obj_arr_obj_i_i_i_compar(Clone::clone(&aux), Clone::clone(&a), fromIndex, toIndex, (fromIndex).wrapping_neg(), Clone::clone(&c))?;
            }
            Ok(())
        }

        #[java_method(name = "mergeSort", descriptor = "([Ljava/lang/Object;[Ljava/lang/Object;IIILjava/util/Comparator;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: mergeSort([Ljava/lang/Object;[Ljava/lang/Object;IIILjava/util/Comparator;)V
        pub fn mergeSort_arr_obj_arr_obj_i_i_i_compar(mut src: Rc<RefCell<Vec<Object>>>, mut dest: Rc<RefCell<Vec<Object>>>, mut low: i32, mut high: i32, mut off: i32, mut c: Object) -> Result<()> {
            let mut length = (high).wrapping_sub(low);
            let mut i: i32 = low;
            loop {
                if i >= high { break; }
                let mut j: i32 = i;
                loop {
                    if j <= low { break; }
                    let _vdispatch0: i32 = if let Some(_d) = c.0.as_any().downcast_ref::<Comparators_NaturalOrderComparator>() { _d.compare(Clone::clone(&Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize])), Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = c.0.as_any().downcast_ref::<Collections_ReverseComparator>() { _d.compare(Clone::clone(&Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize])), Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(_d) = c.0.as_any().downcast_ref::<Object>() { _d.compare(Clone::clone(&Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize])), Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else if let Some(__f) = c.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&dest.borrow()[(j).wrapping_sub(1i32) as usize])), Clone::clone(&Clone::clone(&dest.borrow()[j as usize])))? } else { Default::default() };
                    if (_vdispatch0>0) {
                        Arrays::swap(Clone::clone(&dest), j, (j).wrapping_sub(1i32))?;
                        j = j.wrapping_sub(1i32);
                        continue;
                    }
                    break;
                }
                i = i.wrapping_add(1i32);
            }
            return Ok(());
            i = low;
            let mut j: i32 = high;
            low = (low).wrapping_add(off);
            high = (high).wrapping_add(off);
            let mut mid = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
            Arrays::mergeSort_arr_obj_arr_obj_i_i_i_compar(Clone::clone(&dest), Clone::clone(&src), low, mid, (off).wrapping_neg(), Clone::clone(&c))?;
            Arrays::mergeSort_arr_obj_arr_obj_i_i_i_compar(Clone::clone(&dest), Clone::clone(&src), mid, high, (off).wrapping_neg(), Clone::clone(&c))?;
            let _vdispatch0: i32 = if let Some(_d) = c.0.as_any().downcast_ref::<Comparators_NaturalOrderComparator>() { _d.compare(Clone::clone(&Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize])), Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = c.0.as_any().downcast_ref::<Collections_ReverseComparator>() { _d.compare(Clone::clone(&Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize])), Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(_d) = c.0.as_any().downcast_ref::<Object>() { _d.compare(Clone::clone(&Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize])), Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else if let Some(__f) = c.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&src.borrow()[(mid).wrapping_sub(1i32) as usize])), Clone::clone(&Clone::clone(&src.borrow()[mid as usize])))? } else { Default::default() };
            if (_vdispatch0<=0) {
                System::arraycopy(Object::from_any(src.clone()), low, Object::from_any(dest.clone()), i, length)?;
                return Ok(());
            }
            let mut i: i32 = i;
            let mut p: i32 = low;
            let mut q: i32 = mid;
            loop {
                if i >= j { break; }
                if p < mid {
                    let _vdispatch1: i32 = if let Some(_d) = c.0.as_any().downcast_ref::<Comparators_NaturalOrderComparator>() { _d.compare(Clone::clone(&Clone::clone(&src.borrow()[p as usize])), Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = c.0.as_any().downcast_ref::<Collections_ReverseComparator>() { _d.compare(Clone::clone(&Clone::clone(&src.borrow()[p as usize])), Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(_d) = c.0.as_any().downcast_ref::<Object>() { _d.compare(Clone::clone(&Clone::clone(&src.borrow()[p as usize])), Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else if let Some(__f) = c.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&Clone::clone(&src.borrow()[p as usize])), Clone::clone(&Clone::clone(&src.borrow()[q as usize])))? } else { Default::default() };
                    if (_vdispatch1<=0) {
                        p = p.wrapping_add(1i32);
                        let _aastore_tmp2 = Clone::clone(&Clone::clone(&src.borrow()[p as usize]));
                        dest.borrow_mut()[i as usize] = _aastore_tmp2;
                    } else {
                        q = q.wrapping_add(1i32);
                        let _aastore_tmp2 = Clone::clone(&Clone::clone(&src.borrow()[q as usize]));
                        dest.borrow_mut()[i as usize] = _aastore_tmp2;
                    }
                } else {
                    q = q.wrapping_add(1i32);
                    let _aastore_tmp1 = Clone::clone(&Clone::clone(&src.borrow()[q as usize]));
                    dest.borrow_mut()[i as usize] = _aastore_tmp1;
                }
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "parallelPrefix", descriptor = "([Ljava/lang/Object;Ljava/util/function/BinaryOperator;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;Ljava/util/function/BinaryOperator<TT;>;)V")]
        pub fn parallelPrefix_arr_obj_binary(array: Rc<RefCell<Vec<Object>>>, op: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelPrefix:([Ljava/lang/Object;Ljava/util/function/BinaryOperator;)V")
        }

        #[java_method(name = "parallelPrefix", descriptor = "([Ljava/lang/Object;IILjava/util/function/BinaryOperator;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;IILjava/util/function/BinaryOperator<TT;>;)V")]
        pub fn parallelPrefix_arr_obj_i_i_binary(array: Rc<RefCell<Vec<Object>>>, fromIndex: i32, toIndex: i32, op: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelPrefix:([Ljava/lang/Object;IILjava/util/function/BinaryOperator;)V")
        }

        #[java_method(name = "parallelPrefix", descriptor = "([JLjava/util/function/LongBinaryOperator;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelPrefix_arr_l_longbi(array: Rc<RefCell<Vec<i64>>>, op: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelPrefix:([JLjava/util/function/LongBinaryOperator;)V")
        }

        #[java_method(name = "parallelPrefix", descriptor = "([JIILjava/util/function/LongBinaryOperator;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelPrefix_arr_l_i_i_longbi(array: Rc<RefCell<Vec<i64>>>, fromIndex: i32, toIndex: i32, op: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelPrefix:([JIILjava/util/function/LongBinaryOperator;)V")
        }

        #[java_method(name = "parallelPrefix", descriptor = "([DLjava/util/function/DoubleBinaryOperator;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelPrefix_arr_d_double(array: Rc<RefCell<Vec<f64>>>, op: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelPrefix:([DLjava/util/function/DoubleBinaryOperator;)V")
        }

        #[java_method(name = "parallelPrefix", descriptor = "([DIILjava/util/function/DoubleBinaryOperator;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelPrefix_arr_d_i_i_double(array: Rc<RefCell<Vec<f64>>>, fromIndex: i32, toIndex: i32, op: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelPrefix:([DIILjava/util/function/DoubleBinaryOperator;)V")
        }

        #[java_method(name = "parallelPrefix", descriptor = "([ILjava/util/function/IntBinaryOperator;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelPrefix_arr_i_intbin(array: Rc<RefCell<Vec<i32>>>, op: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelPrefix:([ILjava/util/function/IntBinaryOperator;)V")
        }

        #[java_method(name = "parallelPrefix", descriptor = "([IIILjava/util/function/IntBinaryOperator;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelPrefix_arr_i_i_i_intbin(array: Rc<RefCell<Vec<i32>>>, fromIndex: i32, toIndex: i32, op: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelPrefix:([IIILjava/util/function/IntBinaryOperator;)V")
        }

        #[java_method(name = "binarySearch", descriptor = "([JJ)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: binarySearch([JJ)I
        pub fn binarySearch_arr_l_l(mut a: Rc<RefCell<Vec<i64>>>, mut key: i64) -> Result<i32> {
            let _t0: i32 = Arrays::binarySearch0_arr_l_i_i_l(Clone::clone(&a), 0i32, (a.borrow().len() as i32), key)?;
            Ok(_t0)
        }

        #[java_method(name = "binarySearch", descriptor = "([JIIJ)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch_arr_l_i_i_l(a: Rc<RefCell<Vec<i64>>>, fromIndex: i32, toIndex: i32, key: i64) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch:([JIIJ)I")
        }

        #[java_method(name = "binarySearch0", descriptor = "([JIIJ)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: binarySearch0([JIIJ)I
        pub fn binarySearch0_arr_l_i_i_l(mut a: Rc<RefCell<Vec<i64>>>, mut fromIndex: i32, mut toIndex: i32, mut key: i64) -> Result<i32> {
            let mut low: i32 = fromIndex;
            let mut high = (toIndex).wrapping_sub(1i32);
            loop {
                if low > high { break; }
                let mut mid = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
                let mut midVal = a.borrow()[mid as usize];
                if (((midVal>(key)) as i32-((midVal)<(key)) as i32)<0) {
                    low = (mid).wrapping_add(1i32);
                } else {
                    if (((midVal>(key)) as i32-((midVal)<(key)) as i32)>0) {
                        high = (mid).wrapping_sub(1i32);
                    } else {
                        return Ok(mid);
                    }
                }
            }
            Ok(((low).wrapping_add(1i32)).wrapping_neg())
        }

        #[java_method(name = "binarySearch", descriptor = "([II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: binarySearch([II)I
        pub fn binarySearch_arr_i_i(mut a: Rc<RefCell<Vec<i32>>>, mut key: i32) -> Result<i32> {
            let _t0: i32 = Arrays::binarySearch0_arr_i_i_i_i(Clone::clone(&a), 0i32, (a.borrow().len() as i32), key)?;
            Ok(_t0)
        }

        #[java_method(name = "binarySearch", descriptor = "([IIII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch_arr_i_i_i_i(a: Rc<RefCell<Vec<i32>>>, fromIndex: i32, toIndex: i32, key: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch:([IIII)I")
        }

        #[java_method(name = "binarySearch0", descriptor = "([IIII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: binarySearch0([IIII)I
        pub fn binarySearch0_arr_i_i_i_i(mut a: Rc<RefCell<Vec<i32>>>, mut fromIndex: i32, mut toIndex: i32, mut key: i32) -> Result<i32> {
            let mut low: i32 = fromIndex;
            let mut high = (toIndex).wrapping_sub(1i32);
            loop {
                if low > high { break; }
                let mut mid = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
                let mut midVal = a.borrow()[mid as usize];
                if midVal < key {
                    low = (mid).wrapping_add(1i32);
                } else {
                    if midVal > key {
                        high = (mid).wrapping_sub(1i32);
                    } else {
                        return Ok(mid);
                    }
                }
            }
            Ok(((low).wrapping_add(1i32)).wrapping_neg())
        }

        #[java_method(name = "binarySearch", descriptor = "([SS)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch_arr_s_s(a: Rc<RefCell<Vec<i16>>>, key: i16) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch:([SS)I")
        }

        #[java_method(name = "binarySearch", descriptor = "([SIIS)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch_arr_s_i_i_s(a: Rc<RefCell<Vec<i16>>>, fromIndex: i32, toIndex: i32, key: i16) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch:([SIIS)I")
        }

        #[java_method(name = "binarySearch0", descriptor = "([SIIS)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch0_arr_s_i_i_s(a: Rc<RefCell<Vec<i16>>>, fromIndex: i32, toIndex: i32, key: i16) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch0:([SIIS)I")
        }

        #[java_method(name = "binarySearch", descriptor = "([CC)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch_arr_c_c(a: Rc<RefCell<Vec<u16>>>, key: u16) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch:([CC)I")
        }

        #[java_method(name = "binarySearch", descriptor = "([CIIC)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch_arr_c_i_i_c(a: Rc<RefCell<Vec<u16>>>, fromIndex: i32, toIndex: i32, key: u16) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch:([CIIC)I")
        }

        #[java_method(name = "binarySearch0", descriptor = "([CIIC)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch0_arr_c_i_i_c(a: Rc<RefCell<Vec<u16>>>, fromIndex: i32, toIndex: i32, key: u16) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch0:([CIIC)I")
        }

        #[java_method(name = "binarySearch", descriptor = "([BB)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch_arr_b_b(a: Rc<RefCell<Vec<i8>>>, key: i8) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch:([BB)I")
        }

        #[java_method(name = "binarySearch", descriptor = "([BIIB)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch_arr_b_i_i_b(a: Rc<RefCell<Vec<i8>>>, fromIndex: i32, toIndex: i32, key: i8) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch:([BIIB)I")
        }

        #[java_method(name = "binarySearch0", descriptor = "([BIIB)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch0_arr_b_i_i_b(a: Rc<RefCell<Vec<i8>>>, fromIndex: i32, toIndex: i32, key: i8) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch0:([BIIB)I")
        }

        #[java_method(name = "binarySearch", descriptor = "([DD)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch_arr_d_d(a: Rc<RefCell<Vec<f64>>>, key: f64) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch:([DD)I")
        }

        #[java_method(name = "binarySearch", descriptor = "([DIID)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch_arr_d_i_i_d(a: Rc<RefCell<Vec<f64>>>, fromIndex: i32, toIndex: i32, key: f64) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch:([DIID)I")
        }

        #[java_method(name = "binarySearch0", descriptor = "([DIID)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch0_arr_d_i_i_d(a: Rc<RefCell<Vec<f64>>>, fromIndex: i32, toIndex: i32, key: f64) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch0:([DIID)I")
        }

        #[java_method(name = "binarySearch", descriptor = "([FF)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch_arr_f_f(a: Rc<RefCell<Vec<f32>>>, key: f32) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch:([FF)I")
        }

        #[java_method(name = "binarySearch", descriptor = "([FIIF)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch_arr_f_i_i_f(a: Rc<RefCell<Vec<f32>>>, fromIndex: i32, toIndex: i32, key: f32) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch:([FIIF)I")
        }

        #[java_method(name = "binarySearch0", descriptor = "([FIIF)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch0_arr_f_i_i_f(a: Rc<RefCell<Vec<f32>>>, fromIndex: i32, toIndex: i32, key: f32) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch0:([FIIF)I")
        }

        #[java_method(name = "binarySearch", descriptor = "([Ljava/lang/Object;Ljava/lang/Object;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch_arr_obj_obj(a: Rc<RefCell<Vec<Object>>>, key: Object) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch:([Ljava/lang/Object;Ljava/lang/Object;)I")
        }

        #[java_method(name = "binarySearch", descriptor = "([Ljava/lang/Object;IILjava/lang/Object;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch_arr_obj_i_i_obj(a: Rc<RefCell<Vec<Object>>>, fromIndex: i32, toIndex: i32, key: Object) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch:([Ljava/lang/Object;IILjava/lang/Object;)I")
        }

        #[java_method(name = "binarySearch0", descriptor = "([Ljava/lang/Object;IILjava/lang/Object;)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binarySearch0_arr_obj_i_i_obj(a: Rc<RefCell<Vec<Object>>>, fromIndex: i32, toIndex: i32, key: Object) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch0:([Ljava/lang/Object;IILjava/lang/Object;)I")
        }

        #[java_method(name = "binarySearch", descriptor = "([Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;TT;Ljava/util/Comparator<-TT;>;)I")]
        pub fn binarySearch_arr_obj_obj_compar(a: Rc<RefCell<Vec<Object>>>, key: Object, c: Object) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch:([Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I")
        }

        #[java_method(name = "binarySearch", descriptor = "([Ljava/lang/Object;IILjava/lang/Object;Ljava/util/Comparator;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;IITT;Ljava/util/Comparator<-TT;>;)I")]
        pub fn binarySearch_arr_obj_i_i_obj_compar(a: Rc<RefCell<Vec<Object>>>, fromIndex: i32, toIndex: i32, key: Object, c: Object) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch:([Ljava/lang/Object;IILjava/lang/Object;Ljava/util/Comparator;)I")
        }

        #[java_method(name = "binarySearch0", descriptor = "([Ljava/lang/Object;IILjava/lang/Object;Ljava/util/Comparator;)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;IITT;Ljava/util/Comparator<-TT;>;)I")]
        pub fn binarySearch0_arr_obj_i_i_obj_compar(a: Rc<RefCell<Vec<Object>>>, fromIndex: i32, toIndex: i32, key: Object, c: Object) -> Result<i32> {
            panic!("stub: java/util/Arrays.binarySearch0:([Ljava/lang/Object;IILjava/lang/Object;Ljava/util/Comparator;)I")
        }

        #[java_method(name = "equals", descriptor = "([J[J)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals_arr_l_arr_l(a: Rc<RefCell<Vec<i64>>>, a2: Rc<RefCell<Vec<i64>>>) -> Result<bool> {
            panic!("stub: java/util/Arrays.equals:([J[J)Z")
        }

        #[java_method(name = "equals", descriptor = "([JII[JII)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals_arr_l_i_i_arr_l_i_i(a: Rc<RefCell<Vec<i64>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<i64>>>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
            panic!("stub: java/util/Arrays.equals:([JII[JII)Z")
        }

        #[java_method(name = "equals", descriptor = "([I[I)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: equals([I[I)Z
        pub fn equals_arr_i_arr_i(mut a: Rc<RefCell<Vec<i32>>>, mut a2: Rc<RefCell<Vec<i32>>>) -> Result<bool> {
            if Object::from_any(a.clone()) == Object::from_any(a2.clone()) {
                return Ok((1i32 != 0i32));
            }
            if _is_jnull(&a2) {
                return Ok((0i32 != 0i32));
            }
            let mut length = (a.borrow().len() as i32);
            if (a2.borrow().len() as i32) != length {
                return Ok((0i32 != 0i32));
            }
            let _t0: i32 = ArraysSupport::mismatch_arr_i_arr_i_i(Clone::clone(&a), Clone::clone(&a2), length)?;
            Ok((_t0<0))
        }

        #[java_method(name = "equals", descriptor = "([III[III)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals_arr_i_i_i_arr_i_i_i(a: Rc<RefCell<Vec<i32>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<i32>>>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
            panic!("stub: java/util/Arrays.equals:([III[III)Z")
        }

        #[java_method(name = "equals", descriptor = "([S[S)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals_arr_s_arr_s(a: Rc<RefCell<Vec<i16>>>, a2: Rc<RefCell<Vec<i16>>>) -> Result<bool> {
            panic!("stub: java/util/Arrays.equals:([S[S)Z")
        }

        #[java_method(name = "equals", descriptor = "([SII[SII)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals_arr_s_i_i_arr_s_i_i(a: Rc<RefCell<Vec<i16>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<i16>>>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
            panic!("stub: java/util/Arrays.equals:([SII[SII)Z")
        }

        #[java_method(name = "equals", descriptor = "([C[C)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals_arr_c_arr_c(a: Rc<RefCell<Vec<u16>>>, a2: Rc<RefCell<Vec<u16>>>) -> Result<bool> {
            panic!("stub: java/util/Arrays.equals:([C[C)Z")
        }

        #[java_method(name = "equals", descriptor = "([CII[CII)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals_arr_c_i_i_arr_c_i_i(a: Rc<RefCell<Vec<u16>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<u16>>>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
            panic!("stub: java/util/Arrays.equals:([CII[CII)Z")
        }

        #[java_method(name = "equals", descriptor = "([B[B)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals_arr_b_arr_b(a: Rc<RefCell<Vec<i8>>>, a2: Rc<RefCell<Vec<i8>>>) -> Result<bool> {
            panic!("stub: java/util/Arrays.equals:([B[B)Z")
        }

        #[java_method(name = "equals", descriptor = "([BII[BII)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals_arr_b_i_i_arr_b_i_i(a: Rc<RefCell<Vec<i8>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<i8>>>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
            panic!("stub: java/util/Arrays.equals:([BII[BII)Z")
        }

        #[java_method(name = "equals", descriptor = "([Z[Z)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals_arr_z_arr_z(a: Rc<RefCell<Vec<bool>>>, a2: Rc<RefCell<Vec<bool>>>) -> Result<bool> {
            panic!("stub: java/util/Arrays.equals:([Z[Z)Z")
        }

        #[java_method(name = "equals", descriptor = "([ZII[ZII)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals_arr_z_i_i_arr_z_i_i(a: Rc<RefCell<Vec<bool>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<bool>>>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
            panic!("stub: java/util/Arrays.equals:([ZII[ZII)Z")
        }

        #[java_method(name = "equals", descriptor = "([D[D)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals_arr_d_arr_d(a: Rc<RefCell<Vec<f64>>>, a2: Rc<RefCell<Vec<f64>>>) -> Result<bool> {
            panic!("stub: java/util/Arrays.equals:([D[D)Z")
        }

        #[java_method(name = "equals", descriptor = "([DII[DII)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals_arr_d_i_i_arr_d_i_i(a: Rc<RefCell<Vec<f64>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<f64>>>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
            panic!("stub: java/util/Arrays.equals:([DII[DII)Z")
        }

        #[java_method(name = "equals", descriptor = "([F[F)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals_arr_f_arr_f(a: Rc<RefCell<Vec<f32>>>, a2: Rc<RefCell<Vec<f32>>>) -> Result<bool> {
            panic!("stub: java/util/Arrays.equals:([F[F)Z")
        }

        #[java_method(name = "equals", descriptor = "([FII[FII)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals_arr_f_i_i_arr_f_i_i(a: Rc<RefCell<Vec<f32>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<f32>>>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
            panic!("stub: java/util/Arrays.equals:([FII[FII)Z")
        }

        #[java_method(name = "equals", descriptor = "([Ljava/lang/Object;[Ljava/lang/Object;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: equals([Ljava/lang/Object;[Ljava/lang/Object;)Z
        pub fn equals_arr_obj_arr_obj(mut a: Rc<RefCell<Vec<Object>>>, mut a2: Rc<RefCell<Vec<Object>>>) -> Result<bool> {
            if Object::from_any(a.clone()) == Object::from_any(a2.clone()) {
                return Ok((1i32 != 0i32));
            }
            if _is_jnull(&a2) {
                return Ok((0i32 != 0i32));
            }
            let mut length = (a.borrow().len() as i32);
            if (a2.borrow().len() as i32) != length {
                return Ok((0i32 != 0i32));
            }
            let mut i: i32 = 0i32;
            loop {
                if i >= length { break; }
                let _t0: bool = Objects::equals(Clone::clone(&Clone::clone(&a.borrow()[i as usize])), Clone::clone(&Clone::clone(&a2.borrow()[i as usize])))?;
                if !(_t0) {
                    return Ok((0i32 != 0i32));
                }
                i = i.wrapping_add(1i32);
            }
            Ok((1i32 != 0i32))
        }

        #[java_method(name = "equals", descriptor = "([Ljava/lang/Object;II[Ljava/lang/Object;II)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals_arr_obj_i_i_arr_obj_i_i(a: Rc<RefCell<Vec<Object>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<Object>>>, bFromIndex: i32, bToIndex: i32) -> Result<bool> {
            panic!("stub: java/util/Arrays.equals:([Ljava/lang/Object;II[Ljava/lang/Object;II)Z")
        }

        #[java_method(name = "equals", descriptor = "([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;[TT;Ljava/util/Comparator<-TT;>;)Z")]
        pub fn equals_arr_obj_arr_obj_compar(a: Rc<RefCell<Vec<Object>>>, a2: Rc<RefCell<Vec<Object>>>, cmp: Object) -> Result<bool> {
            panic!("stub: java/util/Arrays.equals:([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)Z")
        }

        #[java_method(name = "equals", descriptor = "([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;II[TT;IILjava/util/Comparator<-TT;>;)Z")]
        pub fn equals_arr_obj_i_i_arr_obj_i_i_compar(a: Rc<RefCell<Vec<Object>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<Object>>>, bFromIndex: i32, bToIndex: i32, cmp: Object) -> Result<bool> {
            panic!("stub: java/util/Arrays.equals:([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)Z")
        }

        #[java_method(name = "fill", descriptor = "([JJ)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fill_arr_l_l(a: Rc<RefCell<Vec<i64>>>, val: i64) -> Result<()> {
            panic!("stub: java/util/Arrays.fill:([JJ)V")
        }

        #[java_method(name = "fill", descriptor = "([JIIJ)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fill_arr_l_i_i_l(a: Rc<RefCell<Vec<i64>>>, fromIndex: i32, toIndex: i32, val: i64) -> Result<()> {
            panic!("stub: java/util/Arrays.fill:([JIIJ)V")
        }

        #[java_method(name = "fill", descriptor = "([II)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: fill([II)V
        pub fn fill_arr_i_i(mut a: Rc<RefCell<Vec<i32>>>, mut val: i32) -> Result<()> {
            let mut i: i32 = 0i32;
            let mut len = (a.borrow().len() as i32);
            loop {
                if i >= len { break; }
                a.borrow_mut()[i as usize] = val;
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "fill", descriptor = "([IIII)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: fill([IIII)V
        pub fn fill_arr_i_i_i_i(mut a: Rc<RefCell<Vec<i32>>>, mut fromIndex: i32, mut toIndex: i32, mut val: i32) -> Result<()> {
            Arrays::rangeCheck((a.borrow().len() as i32), fromIndex, toIndex)?;
            let mut i: i32 = fromIndex;
            loop {
                if i >= toIndex { break; }
                a.borrow_mut()[i as usize] = val;
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "fill", descriptor = "([SS)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fill_arr_s_s(a: Rc<RefCell<Vec<i16>>>, val: i16) -> Result<()> {
            panic!("stub: java/util/Arrays.fill:([SS)V")
        }

        #[java_method(name = "fill", descriptor = "([SIIS)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fill_arr_s_i_i_s(a: Rc<RefCell<Vec<i16>>>, fromIndex: i32, toIndex: i32, val: i16) -> Result<()> {
            panic!("stub: java/util/Arrays.fill:([SIIS)V")
        }

        #[java_method(name = "fill", descriptor = "([CC)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fill_arr_c_c(a: Rc<RefCell<Vec<u16>>>, val: u16) -> Result<()> {
            panic!("stub: java/util/Arrays.fill:([CC)V")
        }

        #[java_method(name = "fill", descriptor = "([CIIC)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fill_arr_c_i_i_c(a: Rc<RefCell<Vec<u16>>>, fromIndex: i32, toIndex: i32, val: u16) -> Result<()> {
            panic!("stub: java/util/Arrays.fill:([CIIC)V")
        }

        #[java_method(name = "fill", descriptor = "([BB)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: fill([BB)V
        pub fn fill_arr_b_b(mut a: Rc<RefCell<Vec<i8>>>, mut val: i8) -> Result<()> {
            let mut i: i32 = 0i32;
            let mut len = (a.borrow().len() as i32);
            loop {
                if i >= len { break; }
                a.borrow_mut()[i as usize] = (val) as i8;
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "fill", descriptor = "([BIIB)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: fill([BIIB)V
        pub fn fill_arr_b_i_i_b(mut a: Rc<RefCell<Vec<i8>>>, mut fromIndex: i32, mut toIndex: i32, mut val: i8) -> Result<()> {
            Arrays::rangeCheck((a.borrow().len() as i32), fromIndex, toIndex)?;
            let mut i: i32 = fromIndex;
            loop {
                if i >= toIndex { break; }
                a.borrow_mut()[i as usize] = (val) as i8;
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "fill", descriptor = "([ZZ)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fill_arr_z_z(a: Rc<RefCell<Vec<bool>>>, val: bool) -> Result<()> {
            panic!("stub: java/util/Arrays.fill:([ZZ)V")
        }

        #[java_method(name = "fill", descriptor = "([ZIIZ)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fill_arr_z_i_i_z(a: Rc<RefCell<Vec<bool>>>, fromIndex: i32, toIndex: i32, val: bool) -> Result<()> {
            panic!("stub: java/util/Arrays.fill:([ZIIZ)V")
        }

        #[java_method(name = "fill", descriptor = "([DD)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fill_arr_d_d(a: Rc<RefCell<Vec<f64>>>, val: f64) -> Result<()> {
            panic!("stub: java/util/Arrays.fill:([DD)V")
        }

        #[java_method(name = "fill", descriptor = "([DIID)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fill_arr_d_i_i_d(a: Rc<RefCell<Vec<f64>>>, fromIndex: i32, toIndex: i32, val: f64) -> Result<()> {
            panic!("stub: java/util/Arrays.fill:([DIID)V")
        }

        #[java_method(name = "fill", descriptor = "([FF)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fill_arr_f_f(a: Rc<RefCell<Vec<f32>>>, val: f32) -> Result<()> {
            panic!("stub: java/util/Arrays.fill:([FF)V")
        }

        #[java_method(name = "fill", descriptor = "([FIIF)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fill_arr_f_i_i_f(a: Rc<RefCell<Vec<f32>>>, fromIndex: i32, toIndex: i32, val: f32) -> Result<()> {
            panic!("stub: java/util/Arrays.fill:([FIIF)V")
        }

        #[java_method(name = "fill", descriptor = "([Ljava/lang/Object;Ljava/lang/Object;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fill_arr_obj_obj(a: Rc<RefCell<Vec<Object>>>, val: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.fill:([Ljava/lang/Object;Ljava/lang/Object;)V")
        }

        #[java_method(name = "fill", descriptor = "([Ljava/lang/Object;IILjava/lang/Object;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fill_arr_obj_i_i_obj(a: Rc<RefCell<Vec<Object>>>, fromIndex: i32, toIndex: i32, val: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.fill:([Ljava/lang/Object;IILjava/lang/Object;)V")
        }

        #[java_method(name = "copyOf", descriptor = "([Ljava/lang/Object;I)[Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;I)[TT;")]
        // java: copyOf([Ljava/lang/Object;I)[Ljava/lang/Object;
        pub fn copyOf_arr_obj_i(mut original: Rc<RefCell<Vec<Object>>>, mut newLength: i32) -> Result<Rc<RefCell<Vec<Object>>>> {
            let _t0: Object = Object::default();
            let _t1: Rc<RefCell<Vec<Object>>> = Arrays::copyOf_arr_obj_i_class(Clone::clone(&original), newLength, Clone::clone(&_t0))?;
            Ok(_t1)
        }

        #[java_method(name = "copyOf", descriptor = "([Ljava/lang/Object;ILjava/lang/Class;)[Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;U:Ljava/lang/Object;>([TU;ILjava/lang/Class<+[TT;>;)[TT;")]
        // java: copyOf([Ljava/lang/Object;ILjava/lang/Class;)[Ljava/lang/Object;
        pub fn copyOf_arr_obj_i_class(mut original: Rc<RefCell<Vec<Object>>>, mut newLength: i32, mut newType: Object) -> Result<Rc<RefCell<Vec<Object>>>> {
            let mut _merged2: Rc<RefCell<Vec<Object>>>;
            if newType == Object::default() {
                let mut _arr0: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); newLength as usize]));
                _merged2 = _arr0;
            } else {
                let _vdispatch0: Object = if let Some(__f) = newType.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let _t1: Object = Array::newInstance_class_i(Clone::clone(&_vdispatch0), newLength)?;
                _merged2 = (_t1).downcast::<Rc<RefCell<Vec<Object>>>>();
            }
            let mut copy: Rc<RefCell<Vec<Object>>> = _merged2;
            let _t3: i32 = Math::min_i_i((original.borrow().len() as i32), newLength)?;
            System::arraycopy(Object::from_any(original.clone()), 0i32, Object::from_any(copy.clone()), 0i32, _t3)?;
            Ok(copy)
        }

        #[java_method(name = "copyOf", descriptor = "([BI)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: copyOf([BI)[B
        pub fn copyOf_arr_b_i(mut original: Rc<RefCell<Vec<i8>>>, mut newLength: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
            if newLength == (original.borrow().len() as i32) {
                let _t0: Object = Object::from_any(original.clone());
                return Ok((_t0).downcast::<Rc<RefCell<Vec<i8>>>>());
            }
            let mut _arr0: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; newLength as usize]));
            let mut copy: Rc<RefCell<Vec<i8>>> = _arr0;
            let _t1: i32 = Math::min_i_i((original.borrow().len() as i32), newLength)?;
            System::arraycopy(Object::from_any(original.clone()), 0i32, Object::from_any(copy.clone()), 0i32, _t1)?;
            Ok(copy)
        }

        #[java_method(name = "copyOf", descriptor = "([SI)[S", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOf_arr_s_i(original: Rc<RefCell<Vec<i16>>>, newLength: i32) -> Result<Rc<RefCell<Vec<i16>>>> {
            panic!("stub: java/util/Arrays.copyOf:([SI)[S")
        }

        #[java_method(name = "copyOf", descriptor = "([II)[I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: copyOf([II)[I
        pub fn copyOf_arr_i_i(mut original: Rc<RefCell<Vec<i32>>>, mut newLength: i32) -> Result<Rc<RefCell<Vec<i32>>>> {
            if newLength == (original.borrow().len() as i32) {
                let _t0: Object = Object::from_any(original.clone());
                return Ok((_t0).downcast::<Rc<RefCell<Vec<i32>>>>());
            }
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; newLength as usize]));
            let mut copy: Rc<RefCell<Vec<i32>>> = _arr0;
            let _t1: i32 = Math::min_i_i((original.borrow().len() as i32), newLength)?;
            System::arraycopy(Object::from_any(original.clone()), 0i32, Object::from_any(copy.clone()), 0i32, _t1)?;
            Ok(copy)
        }

        #[java_method(name = "copyOf", descriptor = "([JI)[J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOf_arr_l_i(original: Rc<RefCell<Vec<i64>>>, newLength: i32) -> Result<Rc<RefCell<Vec<i64>>>> {
            panic!("stub: java/util/Arrays.copyOf:([JI)[J")
        }

        #[java_method(name = "copyOf", descriptor = "([CI)[C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOf_arr_c_i(original: Rc<RefCell<Vec<u16>>>, newLength: i32) -> Result<Rc<RefCell<Vec<u16>>>> {
            panic!("stub: java/util/Arrays.copyOf:([CI)[C")
        }

        #[java_method(name = "copyOf", descriptor = "([FI)[F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOf_arr_f_i(original: Rc<RefCell<Vec<f32>>>, newLength: i32) -> Result<Rc<RefCell<Vec<f32>>>> {
            panic!("stub: java/util/Arrays.copyOf:([FI)[F")
        }

        #[java_method(name = "copyOf", descriptor = "([DI)[D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOf_arr_d_i(original: Rc<RefCell<Vec<f64>>>, newLength: i32) -> Result<Rc<RefCell<Vec<f64>>>> {
            panic!("stub: java/util/Arrays.copyOf:([DI)[D")
        }

        #[java_method(name = "copyOf", descriptor = "([ZI)[Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOf_arr_z_i(original: Rc<RefCell<Vec<bool>>>, newLength: i32) -> Result<Rc<RefCell<Vec<bool>>>> {
            panic!("stub: java/util/Arrays.copyOf:([ZI)[Z")
        }

        #[java_method(name = "copyOfRange", descriptor = "([Ljava/lang/Object;II)[Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;II)[TT;")]
        // java: copyOfRange([Ljava/lang/Object;II)[Ljava/lang/Object;
        pub fn copyOfRange_arr_obj_i_i(mut original: Rc<RefCell<Vec<Object>>>, mut from: i32, mut to: i32) -> Result<Rc<RefCell<Vec<Object>>>> {
            let _t0: Object = Object::default();
            let _t1: Rc<RefCell<Vec<Object>>> = Arrays::copyOfRange_arr_obj_i_i_class(Clone::clone(&original), from, to, Clone::clone(&_t0))?;
            Ok(_t1)
        }

        #[java_method(name = "copyOfRange", descriptor = "([Ljava/lang/Object;IILjava/lang/Class;)[Ljava/lang/Object;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;U:Ljava/lang/Object;>([TU;IILjava/lang/Class<+[TT;>;)[TT;")]
        // java: copyOfRange([Ljava/lang/Object;IILjava/lang/Class;)[Ljava/lang/Object;
        pub fn copyOfRange_arr_obj_i_i_class(mut original: Rc<RefCell<Vec<Object>>>, mut from: i32, mut to: i32, mut newType: Object) -> Result<Rc<RefCell<Vec<Object>>>> {
            let mut newLength = (to).wrapping_sub(from);
            if (newLength<0) {
                let _t0 = StringBuilder::new()?.append_i(from)?;
                let _t1 = _t0.append_str(Clone::clone(&String::from(" > ")))?;
                let _t2 = _t1.append_i(to)?;
                let _t3 = _t2.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut _merged2: Rc<RefCell<Vec<Object>>>;
            if newType == Object::default() {
                let mut _arr0: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); newLength as usize]));
                _merged2 = _arr0;
            } else {
                let _vdispatch0: Object = if let Some(__f) = newType.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let _t1: Object = Array::newInstance_class_i(Clone::clone(&_vdispatch0), newLength)?;
                _merged2 = (_t1).downcast::<Rc<RefCell<Vec<Object>>>>();
            }
            let mut copy: Rc<RefCell<Vec<Object>>> = _merged2;
            let _t3: i32 = Math::min_i_i(((original.borrow().len() as i32)).wrapping_sub(from), newLength)?;
            System::arraycopy(Object::from_any(original.clone()), from, Object::from_any(copy.clone()), 0i32, _t3)?;
            Ok(copy)
        }

        #[java_method(name = "checkLength", descriptor = "(II)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkLength(mut from: i32, mut to: i32) -> Result<()> {
            if to < from {
                let _t0 = StringBuilder::new()?.append_i(from)?;
                let _t1 = _t0.append_str(Clone::clone(&String::from(" > ")))?;
                let _t2 = _t1.append_i(to)?;
                let _t3 = _t2.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "copyOfRange", descriptor = "([BII)[B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: copyOfRange([BII)[B
        pub fn copyOfRange_arr_b_i_i(mut original: Rc<RefCell<Vec<i8>>>, mut from: i32, mut to: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
            if to != (original.borrow().len() as i32) {
                let _t0: Rc<RefCell<Vec<i8>>> = Arrays::copyOfRangeByte(Clone::clone(&original), from, to)?;
                return Ok(_t0);
            }
            let _t0: Object = Object::from_any(original.clone());
            Ok((_t0).downcast::<Rc<RefCell<Vec<i8>>>>())
        }

        #[java_method(name = "copyOfRangeByte", descriptor = "([BII)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOfRangeByte(mut original: Rc<RefCell<Vec<i8>>>, mut from: i32, mut to: i32) -> Result<Rc<RefCell<Vec<i8>>>> {
            Arrays::checkLength(from, to)?;
            let mut newLength = (to).wrapping_sub(from);
            let mut _arr0: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; newLength as usize]));
            let mut copy: Rc<RefCell<Vec<i8>>> = _arr0;
            let _t1: i32 = Math::min_i_i(((original.borrow().len() as i32)).wrapping_sub(from), newLength)?;
            System::arraycopy(Object::from_any(original.clone()), from, Object::from_any(copy.clone()), 0i32, _t1)?;
            Ok(copy)
        }

        #[java_method(name = "copyOfRange", descriptor = "([SII)[S", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOfRange_arr_s_i_i(original: Rc<RefCell<Vec<i16>>>, from: i32, to: i32) -> Result<Rc<RefCell<Vec<i16>>>> {
            panic!("stub: java/util/Arrays.copyOfRange:([SII)[S")
        }

        #[java_method(name = "copyOfRangeShort", descriptor = "([SII)[S", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOfRangeShort(original: Rc<RefCell<Vec<i16>>>, from: i32, to: i32) -> Result<Rc<RefCell<Vec<i16>>>> {
            panic!("stub: java/util/Arrays.copyOfRangeShort:([SII)[S")
        }

        #[java_method(name = "copyOfRange", descriptor = "([III)[I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: copyOfRange([III)[I
        pub fn copyOfRange_arr_i_i_i(mut original: Rc<RefCell<Vec<i32>>>, mut from: i32, mut to: i32) -> Result<Rc<RefCell<Vec<i32>>>> {
            if to != (original.borrow().len() as i32) {
                let _t0: Rc<RefCell<Vec<i32>>> = Arrays::copyOfRangeInt(Clone::clone(&original), from, to)?;
                return Ok(_t0);
            }
            let _t0: Object = Object::from_any(original.clone());
            Ok((_t0).downcast::<Rc<RefCell<Vec<i32>>>>())
        }

        #[java_method(name = "copyOfRangeInt", descriptor = "([III)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOfRangeInt(mut original: Rc<RefCell<Vec<i32>>>, mut from: i32, mut to: i32) -> Result<Rc<RefCell<Vec<i32>>>> {
            Arrays::checkLength(from, to)?;
            let mut newLength = (to).wrapping_sub(from);
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; newLength as usize]));
            let mut copy: Rc<RefCell<Vec<i32>>> = _arr0;
            let _t1: i32 = Math::min_i_i(((original.borrow().len() as i32)).wrapping_sub(from), newLength)?;
            System::arraycopy(Object::from_any(original.clone()), from, Object::from_any(copy.clone()), 0i32, _t1)?;
            Ok(copy)
        }

        #[java_method(name = "copyOfRange", descriptor = "([JII)[J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOfRange_arr_l_i_i(original: Rc<RefCell<Vec<i64>>>, from: i32, to: i32) -> Result<Rc<RefCell<Vec<i64>>>> {
            panic!("stub: java/util/Arrays.copyOfRange:([JII)[J")
        }

        #[java_method(name = "copyOfRangeLong", descriptor = "([JII)[J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOfRangeLong(original: Rc<RefCell<Vec<i64>>>, from: i32, to: i32) -> Result<Rc<RefCell<Vec<i64>>>> {
            panic!("stub: java/util/Arrays.copyOfRangeLong:([JII)[J")
        }

        #[java_method(name = "copyOfRange", descriptor = "([CII)[C", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOfRange_arr_c_i_i(original: Rc<RefCell<Vec<u16>>>, from: i32, to: i32) -> Result<Rc<RefCell<Vec<u16>>>> {
            panic!("stub: java/util/Arrays.copyOfRange:([CII)[C")
        }

        #[java_method(name = "copyOfRangeChar", descriptor = "([CII)[C", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOfRangeChar(original: Rc<RefCell<Vec<u16>>>, from: i32, to: i32) -> Result<Rc<RefCell<Vec<u16>>>> {
            panic!("stub: java/util/Arrays.copyOfRangeChar:([CII)[C")
        }

        #[java_method(name = "copyOfRange", descriptor = "([FII)[F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOfRange_arr_f_i_i(original: Rc<RefCell<Vec<f32>>>, from: i32, to: i32) -> Result<Rc<RefCell<Vec<f32>>>> {
            panic!("stub: java/util/Arrays.copyOfRange:([FII)[F")
        }

        #[java_method(name = "copyOfRangeFloat", descriptor = "([FII)[F", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOfRangeFloat(original: Rc<RefCell<Vec<f32>>>, from: i32, to: i32) -> Result<Rc<RefCell<Vec<f32>>>> {
            panic!("stub: java/util/Arrays.copyOfRangeFloat:([FII)[F")
        }

        #[java_method(name = "copyOfRange", descriptor = "([DII)[D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOfRange_arr_d_i_i(original: Rc<RefCell<Vec<f64>>>, from: i32, to: i32) -> Result<Rc<RefCell<Vec<f64>>>> {
            panic!("stub: java/util/Arrays.copyOfRange:([DII)[D")
        }

        #[java_method(name = "copyOfRangeDouble", descriptor = "([DII)[D", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOfRangeDouble(original: Rc<RefCell<Vec<f64>>>, from: i32, to: i32) -> Result<Rc<RefCell<Vec<f64>>>> {
            panic!("stub: java/util/Arrays.copyOfRangeDouble:([DII)[D")
        }

        #[java_method(name = "copyOfRange", descriptor = "([ZII)[Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOfRange_arr_z_i_i(original: Rc<RefCell<Vec<bool>>>, from: i32, to: i32) -> Result<Rc<RefCell<Vec<bool>>>> {
            panic!("stub: java/util/Arrays.copyOfRange:([ZII)[Z")
        }

        #[java_method(name = "copyOfRangeBoolean", descriptor = "([ZII)[Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyOfRangeBoolean(original: Rc<RefCell<Vec<bool>>>, from: i32, to: i32) -> Result<Rc<RefCell<Vec<bool>>>> {
            panic!("stub: java/util/Arrays.copyOfRangeBoolean:([ZII)[Z")
        }

        #[java_method(name = "asList", descriptor = "([Ljava/lang/Object;)Ljava/util/List;", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)Ljava/util/List<TT;>;")]
        pub fn asList(mut a: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
            Ok(Object::from_any(Arrays_ArrayList::<Object>::new(Default::default())?.clone()))
        }

        #[java_method(name = "hashCode", descriptor = "([J)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode_arr_l(a: Rc<RefCell<Vec<i64>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.hashCode:([J)I")
        }

        #[java_method(name = "hashCode", descriptor = "([I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode_arr_i(a: Rc<RefCell<Vec<i32>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.hashCode:([I)I")
        }

        #[java_method(name = "hashCode", descriptor = "([S)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode_arr_s(a: Rc<RefCell<Vec<i16>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.hashCode:([S)I")
        }

        #[java_method(name = "hashCode", descriptor = "([C)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode_arr_c(a: Rc<RefCell<Vec<u16>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.hashCode:([C)I")
        }

        #[java_method(name = "hashCode", descriptor = "([B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode_arr_b(a: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.hashCode:([B)I")
        }

        #[java_method(name = "hashCode", descriptor = "([Z)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode_arr_z(a: Rc<RefCell<Vec<bool>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.hashCode:([Z)I")
        }

        #[java_method(name = "hashCode", descriptor = "([F)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode_arr_f(a: Rc<RefCell<Vec<f32>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.hashCode:([F)I")
        }

        #[java_method(name = "hashCode", descriptor = "([D)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode_arr_d(a: Rc<RefCell<Vec<f64>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.hashCode:([D)I")
        }

        #[java_method(name = "hashCode", descriptor = "([Ljava/lang/Object;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: hashCode([Ljava/lang/Object;)I
        pub fn hashCode_arr_obj(mut a: Rc<RefCell<Vec<Object>>>) -> Result<i32> {
            if _is_jnull(&a) {
                return Ok(0i32);
            }
            let mut result: i32 = 1i32;
            let mut local_2: Rc<RefCell<Vec<Object>>> = a;
            let mut local_3 = (local_2.borrow().len() as i32);
            let mut local_4: i32 = 0i32;
            loop {
                if local_4 >= local_3 { break; }
                let mut element = Clone::clone(&local_2.borrow()[local_4 as usize]);
                let mut _merged1: i32;
                if _is_jnull(&element) {
                    _merged1 = 0i32;
                } else {
                    let _vdispatch0: i32 = if let Some(__f) = element.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
                    _merged1 = _vdispatch0;
                }
                result = ((31i32).wrapping_mul(result)).wrapping_add(_merged1);
                local_4 = local_4.wrapping_add(1i32);
            }
            Ok(result)
        }

        #[java_method(name = "deepHashCode", descriptor = "([Ljava/lang/Object;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn deepHashCode(a: Rc<RefCell<Vec<Object>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.deepHashCode:([Ljava/lang/Object;)I")
        }

        #[java_method(name = "primitiveArrayHashCode", descriptor = "(Ljava/lang/Object;Ljava/lang/Class;)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Object;Ljava/lang/Class<*>;)I")]
        pub fn primitiveArrayHashCode(a: Object, cl: Object) -> Result<i32> {
            panic!("stub: java/util/Arrays.primitiveArrayHashCode:(Ljava/lang/Object;Ljava/lang/Class;)I")
        }

        #[java_method(name = "deepEquals", descriptor = "([Ljava/lang/Object;[Ljava/lang/Object;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn deepEquals(a1: Rc<RefCell<Vec<Object>>>, a2: Rc<RefCell<Vec<Object>>>) -> Result<bool> {
            panic!("stub: java/util/Arrays.deepEquals:([Ljava/lang/Object;[Ljava/lang/Object;)Z")
        }

        #[java_method(name = "deepEquals0", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn deepEquals0(e1: Object, e2: Object) -> Result<bool> {
            panic!("stub: java/util/Arrays.deepEquals0:(Ljava/lang/Object;Ljava/lang/Object;)Z")
        }

        #[java_method(name = "toString", descriptor = "([J)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString_arr_l(a: Rc<RefCell<Vec<i64>>>) -> Result<String> {
            panic!("stub: java/util/Arrays.toString:([J)Ljava/lang/String;")
        }

        #[java_method(name = "toString", descriptor = "([I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toString([I)Ljava/lang/String;
        pub fn toString_arr_i(mut a: Rc<RefCell<Vec<i32>>>) -> Result<String> {
            if _is_jnull(&a) {
                return Ok(String::from("null"));
            }
            let mut iMax = ((a.borrow().len() as i32)).wrapping_sub(1i32);
            if iMax == -1i32 {
                return Ok(String::from("[]"));
            }
            let mut b = StringBuilder::new()?;
            let _t0 = b.append_c(((91i32) as u16))?;
            let mut i: i32 = 0i32;
            loop {
                let _t1 = b.append_i(a.borrow()[i as usize])?;
                if i == iMax {
                    let _t2 = b.append_c(((93i32) as u16))?;
                    let _t3 = _t2.toString()?;
                    return Ok(_t3);
                }
                let _t2 = b.append_str(Clone::clone(&String::from(", ")))?;
                i = i.wrapping_add(1i32);
            }
        }

        #[java_method(name = "toString", descriptor = "([S)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString_arr_s(a: Rc<RefCell<Vec<i16>>>) -> Result<String> {
            panic!("stub: java/util/Arrays.toString:([S)Ljava/lang/String;")
        }

        #[java_method(name = "toString", descriptor = "([C)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString_arr_c(a: Rc<RefCell<Vec<u16>>>) -> Result<String> {
            panic!("stub: java/util/Arrays.toString:([C)Ljava/lang/String;")
        }

        #[java_method(name = "toString", descriptor = "([B)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString_arr_b(a: Rc<RefCell<Vec<i8>>>) -> Result<String> {
            panic!("stub: java/util/Arrays.toString:([B)Ljava/lang/String;")
        }

        #[java_method(name = "toString", descriptor = "([Z)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString_arr_z(a: Rc<RefCell<Vec<bool>>>) -> Result<String> {
            panic!("stub: java/util/Arrays.toString:([Z)Ljava/lang/String;")
        }

        #[java_method(name = "toString", descriptor = "([F)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString_arr_f(a: Rc<RefCell<Vec<f32>>>) -> Result<String> {
            panic!("stub: java/util/Arrays.toString:([F)Ljava/lang/String;")
        }

        #[java_method(name = "toString", descriptor = "([D)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString_arr_d(a: Rc<RefCell<Vec<f64>>>) -> Result<String> {
            panic!("stub: java/util/Arrays.toString:([D)Ljava/lang/String;")
        }

        #[java_method(name = "toString", descriptor = "([Ljava/lang/Object;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toString([Ljava/lang/Object;)Ljava/lang/String;
        pub fn toString_arr_obj(mut a: Rc<RefCell<Vec<Object>>>) -> Result<String> {
            if _is_jnull(&a) {
                return Ok(String::from("null"));
            }
            let mut iMax = ((a.borrow().len() as i32)).wrapping_sub(1i32);
            if iMax == -1i32 {
                return Ok(String::from("[]"));
            }
            let mut b = StringBuilder::new()?;
            let _t0 = b.append_c(((91i32) as u16))?;
            let mut i: i32 = 0i32;
            loop {
                let _t1 = b.append_str(Clone::clone(&String::from_owned(format!("{}", Clone::clone(&a.borrow()[i as usize])))))?;
                if i == iMax {
                    let _t2 = b.append_c(((93i32) as u16))?;
                    let _t3 = _t2.toString()?;
                    return Ok(_t3);
                }
                let _t2 = b.append_str(Clone::clone(&String::from(", ")))?;
                i = i.wrapping_add(1i32);
            }
        }

        #[java_method(name = "deepToString", descriptor = "([Ljava/lang/Object;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn deepToString_arr_obj(a: Rc<RefCell<Vec<Object>>>) -> Result<String> {
            panic!("stub: java/util/Arrays.deepToString:([Ljava/lang/Object;)Ljava/lang/String;")
        }

        #[java_method(name = "deepToString", descriptor = "([Ljava/lang/Object;Ljava/lang/StringBuilder;Ljava/util/Set;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "([Ljava/lang/Object;Ljava/lang/StringBuilder;Ljava/util/Set<[Ljava/lang/Object;>;)V")]
        pub fn deepToString_arr_obj_sb_set(a: Rc<RefCell<Vec<Object>>>, buf: StringBuilder, dejaVu: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.deepToString:([Ljava/lang/Object;Ljava/lang/StringBuilder;Ljava/util/Set;)V")
        }

        #[java_method(name = "setAll", descriptor = "([Ljava/lang/Object;Ljava/util/function/IntFunction;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;Ljava/util/function/IntFunction<+TT;>;)V")]
        pub fn setAll_arr_obj_intfun(array: Rc<RefCell<Vec<Object>>>, generator: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.setAll:([Ljava/lang/Object;Ljava/util/function/IntFunction;)V")
        }

        #[java_method(name = "parallelSetAll", descriptor = "([Ljava/lang/Object;Ljava/util/function/IntFunction;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;Ljava/util/function/IntFunction<+TT;>;)V")]
        pub fn parallelSetAll_arr_obj_intfun(array: Rc<RefCell<Vec<Object>>>, generator: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSetAll:([Ljava/lang/Object;Ljava/util/function/IntFunction;)V")
        }

        #[java_method(name = "setAll", descriptor = "([ILjava/util/function/IntUnaryOperator;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setAll_arr_i_intuna(array: Rc<RefCell<Vec<i32>>>, generator: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.setAll:([ILjava/util/function/IntUnaryOperator;)V")
        }

        #[java_method(name = "parallelSetAll", descriptor = "([ILjava/util/function/IntUnaryOperator;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelSetAll_arr_i_intuna(array: Rc<RefCell<Vec<i32>>>, generator: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSetAll:([ILjava/util/function/IntUnaryOperator;)V")
        }

        #[java_method(name = "setAll", descriptor = "([JLjava/util/function/IntToLongFunction;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setAll_arr_l_inttol(array: Rc<RefCell<Vec<i64>>>, generator: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.setAll:([JLjava/util/function/IntToLongFunction;)V")
        }

        #[java_method(name = "parallelSetAll", descriptor = "([JLjava/util/function/IntToLongFunction;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelSetAll_arr_l_inttol(array: Rc<RefCell<Vec<i64>>>, generator: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSetAll:([JLjava/util/function/IntToLongFunction;)V")
        }

        #[java_method(name = "setAll", descriptor = "([DLjava/util/function/IntToDoubleFunction;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setAll_arr_d_inttod(array: Rc<RefCell<Vec<f64>>>, generator: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.setAll:([DLjava/util/function/IntToDoubleFunction;)V")
        }

        #[java_method(name = "parallelSetAll", descriptor = "([DLjava/util/function/IntToDoubleFunction;)V", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parallelSetAll_arr_d_inttod(array: Rc<RefCell<Vec<f64>>>, generator: Object) -> Result<()> {
            panic!("stub: java/util/Arrays.parallelSetAll:([DLjava/util/function/IntToDoubleFunction;)V")
        }

        #[java_method(name = "spliterator", descriptor = "([Ljava/lang/Object;)Ljava/util/Spliterator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)Ljava/util/Spliterator<TT;>;")]
        pub fn spliterator_arr_obj(array: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
            panic!("stub: java/util/Arrays.spliterator:([Ljava/lang/Object;)Ljava/util/Spliterator;")
        }

        #[java_method(name = "spliterator", descriptor = "([Ljava/lang/Object;II)Ljava/util/Spliterator;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;II)Ljava/util/Spliterator<TT;>;")]
        // java: spliterator([Ljava/lang/Object;II)Ljava/util/Spliterator;
        pub fn spliterator_arr_obj_i_i(mut array: Rc<RefCell<Vec<Object>>>, mut startInclusive: i32, mut endExclusive: i32) -> Result<Object> {
            let _t0: Object = Spliterators::spliterator_arr_obj_i_i_i(Clone::clone(&array), startInclusive, endExclusive, 1040i32)?;
            Ok(_t0)
        }

        #[java_method(name = "spliterator", descriptor = "([I)Ljava/util/Spliterator$OfInt;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spliterator_arr_i(array: Rc<RefCell<Vec<i32>>>) -> Result<Object> {
            panic!("stub: java/util/Arrays.spliterator:([I)Ljava/util/Spliterator$OfInt;")
        }

        #[java_method(name = "spliterator", descriptor = "([III)Ljava/util/Spliterator$OfInt;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spliterator_arr_i_i_i(array: Rc<RefCell<Vec<i32>>>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
            panic!("stub: java/util/Arrays.spliterator:([III)Ljava/util/Spliterator$OfInt;")
        }

        #[java_method(name = "spliterator", descriptor = "([J)Ljava/util/Spliterator$OfLong;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spliterator_arr_l(array: Rc<RefCell<Vec<i64>>>) -> Result<Object> {
            panic!("stub: java/util/Arrays.spliterator:([J)Ljava/util/Spliterator$OfLong;")
        }

        #[java_method(name = "spliterator", descriptor = "([JII)Ljava/util/Spliterator$OfLong;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spliterator_arr_l_i_i(array: Rc<RefCell<Vec<i64>>>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
            panic!("stub: java/util/Arrays.spliterator:([JII)Ljava/util/Spliterator$OfLong;")
        }

        #[java_method(name = "spliterator", descriptor = "([D)Ljava/util/Spliterator$OfDouble;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spliterator_arr_d(array: Rc<RefCell<Vec<f64>>>) -> Result<Object> {
            panic!("stub: java/util/Arrays.spliterator:([D)Ljava/util/Spliterator$OfDouble;")
        }

        #[java_method(name = "spliterator", descriptor = "([DII)Ljava/util/Spliterator$OfDouble;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn spliterator_arr_d_i_i(array: Rc<RefCell<Vec<f64>>>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
            panic!("stub: java/util/Arrays.spliterator:([DII)Ljava/util/Spliterator$OfDouble;")
        }

        #[java_method(name = "stream", descriptor = "([Ljava/lang/Object;)Ljava/util/stream/Stream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;)Ljava/util/stream/Stream<TT;>;")]
        // java: stream([Ljava/lang/Object;)Ljava/util/stream/Stream;
        pub fn stream_arr_obj(mut array: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
            let _t0: Object = Arrays::stream_arr_obj_i_i(Clone::clone(&array), 0i32, (array.borrow().len() as i32))?;
            Ok(_t0)
        }

        #[java_method(name = "stream", descriptor = "([Ljava/lang/Object;II)Ljava/util/stream/Stream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;II)Ljava/util/stream/Stream<TT;>;")]
        // java: stream([Ljava/lang/Object;II)Ljava/util/stream/Stream;
        pub fn stream_arr_obj_i_i(mut array: Rc<RefCell<Vec<Object>>>, mut startInclusive: i32, mut endExclusive: i32) -> Result<Object> {
            let _t0: Object = Arrays::spliterator_arr_obj_i_i(Clone::clone(&array), startInclusive, endExclusive)?;
            let _t1: Object = StreamSupport::stream_splite_z(Clone::clone(&_t0), (0i32 != 0i32))?;
            Ok(_t1)
        }

        #[java_method(name = "stream", descriptor = "([I)Ljava/util/stream/IntStream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stream_arr_i(array: Rc<RefCell<Vec<i32>>>) -> Result<Object> {
            panic!("stub: java/util/Arrays.stream:([I)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "stream", descriptor = "([III)Ljava/util/stream/IntStream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stream_arr_i_i_i(array: Rc<RefCell<Vec<i32>>>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
            panic!("stub: java/util/Arrays.stream:([III)Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "stream", descriptor = "([J)Ljava/util/stream/LongStream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stream_arr_l(array: Rc<RefCell<Vec<i64>>>) -> Result<Object> {
            panic!("stub: java/util/Arrays.stream:([J)Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "stream", descriptor = "([JII)Ljava/util/stream/LongStream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stream_arr_l_i_i(array: Rc<RefCell<Vec<i64>>>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
            panic!("stub: java/util/Arrays.stream:([JII)Ljava/util/stream/LongStream;")
        }

        #[java_method(name = "stream", descriptor = "([D)Ljava/util/stream/DoubleStream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stream_arr_d(array: Rc<RefCell<Vec<f64>>>) -> Result<Object> {
            panic!("stub: java/util/Arrays.stream:([D)Ljava/util/stream/DoubleStream;")
        }

        #[java_method(name = "stream", descriptor = "([DII)Ljava/util/stream/DoubleStream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stream_arr_d_i_i(array: Rc<RefCell<Vec<f64>>>, startInclusive: i32, endExclusive: i32) -> Result<Object> {
            panic!("stub: java/util/Arrays.stream:([DII)Ljava/util/stream/DoubleStream;")
        }

        #[java_method(name = "compare", descriptor = "([Z[Z)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare_arr_z_arr_z(a: Rc<RefCell<Vec<bool>>>, b: Rc<RefCell<Vec<bool>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([Z[Z)I")
        }

        #[java_method(name = "compare", descriptor = "([ZII[ZII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare_arr_z_i_i_arr_z_i_i(a: Rc<RefCell<Vec<bool>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<bool>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([ZII[ZII)I")
        }

        #[java_method(name = "compare", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare_arr_b_arr_b(a: Rc<RefCell<Vec<i8>>>, b: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([B[B)I")
        }

        #[java_method(name = "compare", descriptor = "([BII[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare_arr_b_i_i_arr_b_i_i(a: Rc<RefCell<Vec<i8>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<i8>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([BII[BII)I")
        }

        #[java_method(name = "compareUnsigned", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareUnsigned_arr_b_arr_b(a: Rc<RefCell<Vec<i8>>>, b: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.compareUnsigned:([B[B)I")
        }

        #[java_method(name = "compareUnsigned", descriptor = "([BII[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareUnsigned_arr_b_i_i_arr_b_i_i(a: Rc<RefCell<Vec<i8>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<i8>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.compareUnsigned:([BII[BII)I")
        }

        #[java_method(name = "compare", descriptor = "([S[S)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare_arr_s_arr_s(a: Rc<RefCell<Vec<i16>>>, b: Rc<RefCell<Vec<i16>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([S[S)I")
        }

        #[java_method(name = "compare", descriptor = "([SII[SII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare_arr_s_i_i_arr_s_i_i(a: Rc<RefCell<Vec<i16>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<i16>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([SII[SII)I")
        }

        #[java_method(name = "compareUnsigned", descriptor = "([S[S)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareUnsigned_arr_s_arr_s(a: Rc<RefCell<Vec<i16>>>, b: Rc<RefCell<Vec<i16>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.compareUnsigned:([S[S)I")
        }

        #[java_method(name = "compareUnsigned", descriptor = "([SII[SII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareUnsigned_arr_s_i_i_arr_s_i_i(a: Rc<RefCell<Vec<i16>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<i16>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.compareUnsigned:([SII[SII)I")
        }

        #[java_method(name = "compare", descriptor = "([C[C)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare_arr_c_arr_c(a: Rc<RefCell<Vec<u16>>>, b: Rc<RefCell<Vec<u16>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([C[C)I")
        }

        #[java_method(name = "compare", descriptor = "([CII[CII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare_arr_c_i_i_arr_c_i_i(a: Rc<RefCell<Vec<u16>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<u16>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([CII[CII)I")
        }

        #[java_method(name = "compare", descriptor = "([I[I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare_arr_i_arr_i(a: Rc<RefCell<Vec<i32>>>, b: Rc<RefCell<Vec<i32>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([I[I)I")
        }

        #[java_method(name = "compare", descriptor = "([III[III)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare_arr_i_i_i_arr_i_i_i(a: Rc<RefCell<Vec<i32>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<i32>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([III[III)I")
        }

        #[java_method(name = "compareUnsigned", descriptor = "([I[I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareUnsigned_arr_i_arr_i(a: Rc<RefCell<Vec<i32>>>, b: Rc<RefCell<Vec<i32>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.compareUnsigned:([I[I)I")
        }

        #[java_method(name = "compareUnsigned", descriptor = "([III[III)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareUnsigned_arr_i_i_i_arr_i_i_i(a: Rc<RefCell<Vec<i32>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<i32>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.compareUnsigned:([III[III)I")
        }

        #[java_method(name = "compare", descriptor = "([J[J)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare_arr_l_arr_l(a: Rc<RefCell<Vec<i64>>>, b: Rc<RefCell<Vec<i64>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([J[J)I")
        }

        #[java_method(name = "compare", descriptor = "([JII[JII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare_arr_l_i_i_arr_l_i_i(a: Rc<RefCell<Vec<i64>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<i64>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([JII[JII)I")
        }

        #[java_method(name = "compareUnsigned", descriptor = "([J[J)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareUnsigned_arr_l_arr_l(a: Rc<RefCell<Vec<i64>>>, b: Rc<RefCell<Vec<i64>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.compareUnsigned:([J[J)I")
        }

        #[java_method(name = "compareUnsigned", descriptor = "([JII[JII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareUnsigned_arr_l_i_i_arr_l_i_i(a: Rc<RefCell<Vec<i64>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<i64>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.compareUnsigned:([JII[JII)I")
        }

        #[java_method(name = "compare", descriptor = "([F[F)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare_arr_f_arr_f(a: Rc<RefCell<Vec<f32>>>, b: Rc<RefCell<Vec<f32>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([F[F)I")
        }

        #[java_method(name = "compare", descriptor = "([FII[FII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare_arr_f_i_i_arr_f_i_i(a: Rc<RefCell<Vec<f32>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<f32>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([FII[FII)I")
        }

        #[java_method(name = "compare", descriptor = "([D[D)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare_arr_d_arr_d(a: Rc<RefCell<Vec<f64>>>, b: Rc<RefCell<Vec<f64>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([D[D)I")
        }

        #[java_method(name = "compare", descriptor = "([DII[DII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare_arr_d_i_i_arr_d_i_i(a: Rc<RefCell<Vec<f64>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<f64>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([DII[DII)I")
        }

        #[java_method(name = "compare", descriptor = "([Ljava/lang/Comparable;[Ljava/lang/Comparable;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T::Ljava/lang/Comparable<-TT;>;>([TT;[TT;)I")]
        pub fn compare_arr_cmp_arr_cmp(a: Rc<RefCell<Vec<Object>>>, b: Rc<RefCell<Vec<Object>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([Ljava/lang/Comparable;[Ljava/lang/Comparable;)I")
        }

        #[java_method(name = "compare", descriptor = "([Ljava/lang/Comparable;II[Ljava/lang/Comparable;II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T::Ljava/lang/Comparable<-TT;>;>([TT;II[TT;II)I")]
        pub fn compare_arr_cmp_i_i_arr_cmp_i_i(a: Rc<RefCell<Vec<Object>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<Object>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([Ljava/lang/Comparable;II[Ljava/lang/Comparable;II)I")
        }

        #[java_method(name = "compare", descriptor = "([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;[TT;Ljava/util/Comparator<-TT;>;)I")]
        pub fn compare_arr_obj_arr_obj_compar(a: Rc<RefCell<Vec<Object>>>, b: Rc<RefCell<Vec<Object>>>, cmp: Object) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)I")
        }

        #[java_method(name = "compare", descriptor = "([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;II[TT;IILjava/util/Comparator<-TT;>;)I")]
        pub fn compare_arr_obj_i_i_arr_obj_i_i_compar(a: Rc<RefCell<Vec<Object>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<Object>>>, bFromIndex: i32, bToIndex: i32, cmp: Object) -> Result<i32> {
            panic!("stub: java/util/Arrays.compare:([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)I")
        }

        #[java_method(name = "mismatch", descriptor = "([Z[Z)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_arr_z_arr_z(a: Rc<RefCell<Vec<bool>>>, b: Rc<RefCell<Vec<bool>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([Z[Z)I")
        }

        #[java_method(name = "mismatch", descriptor = "([ZII[ZII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_arr_z_i_i_arr_z_i_i(a: Rc<RefCell<Vec<bool>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<bool>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([ZII[ZII)I")
        }

        #[java_method(name = "mismatch", descriptor = "([B[B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_arr_b_arr_b(a: Rc<RefCell<Vec<i8>>>, b: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([B[B)I")
        }

        #[java_method(name = "mismatch", descriptor = "([BII[BII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_arr_b_i_i_arr_b_i_i(a: Rc<RefCell<Vec<i8>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<i8>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([BII[BII)I")
        }

        #[java_method(name = "mismatch", descriptor = "([C[C)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_arr_c_arr_c(a: Rc<RefCell<Vec<u16>>>, b: Rc<RefCell<Vec<u16>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([C[C)I")
        }

        #[java_method(name = "mismatch", descriptor = "([CII[CII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_arr_c_i_i_arr_c_i_i(a: Rc<RefCell<Vec<u16>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<u16>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([CII[CII)I")
        }

        #[java_method(name = "mismatch", descriptor = "([S[S)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_arr_s_arr_s(a: Rc<RefCell<Vec<i16>>>, b: Rc<RefCell<Vec<i16>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([S[S)I")
        }

        #[java_method(name = "mismatch", descriptor = "([SII[SII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_arr_s_i_i_arr_s_i_i(a: Rc<RefCell<Vec<i16>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<i16>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([SII[SII)I")
        }

        #[java_method(name = "mismatch", descriptor = "([I[I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_arr_i_arr_i(a: Rc<RefCell<Vec<i32>>>, b: Rc<RefCell<Vec<i32>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([I[I)I")
        }

        #[java_method(name = "mismatch", descriptor = "([III[III)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_arr_i_i_i_arr_i_i_i(a: Rc<RefCell<Vec<i32>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<i32>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([III[III)I")
        }

        #[java_method(name = "mismatch", descriptor = "([J[J)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_arr_l_arr_l(a: Rc<RefCell<Vec<i64>>>, b: Rc<RefCell<Vec<i64>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([J[J)I")
        }

        #[java_method(name = "mismatch", descriptor = "([JII[JII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_arr_l_i_i_arr_l_i_i(a: Rc<RefCell<Vec<i64>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<i64>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([JII[JII)I")
        }

        #[java_method(name = "mismatch", descriptor = "([F[F)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_arr_f_arr_f(a: Rc<RefCell<Vec<f32>>>, b: Rc<RefCell<Vec<f32>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([F[F)I")
        }

        #[java_method(name = "mismatch", descriptor = "([FII[FII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_arr_f_i_i_arr_f_i_i(a: Rc<RefCell<Vec<f32>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<f32>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([FII[FII)I")
        }

        #[java_method(name = "mismatch", descriptor = "([D[D)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_arr_d_arr_d(a: Rc<RefCell<Vec<f64>>>, b: Rc<RefCell<Vec<f64>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([D[D)I")
        }

        #[java_method(name = "mismatch", descriptor = "([DII[DII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_arr_d_i_i_arr_d_i_i(a: Rc<RefCell<Vec<f64>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<f64>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([DII[DII)I")
        }

        #[java_method(name = "mismatch", descriptor = "([Ljava/lang/Object;[Ljava/lang/Object;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_arr_obj_arr_obj(a: Rc<RefCell<Vec<Object>>>, b: Rc<RefCell<Vec<Object>>>) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([Ljava/lang/Object;[Ljava/lang/Object;)I")
        }

        #[java_method(name = "mismatch", descriptor = "([Ljava/lang/Object;II[Ljava/lang/Object;II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mismatch_arr_obj_i_i_arr_obj_i_i(a: Rc<RefCell<Vec<Object>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<Object>>>, bFromIndex: i32, bToIndex: i32) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([Ljava/lang/Object;II[Ljava/lang/Object;II)I")
        }

        #[java_method(name = "mismatch", descriptor = "([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;[TT;Ljava/util/Comparator<-TT;>;)I")]
        pub fn mismatch_arr_obj_arr_obj_compar(a: Rc<RefCell<Vec<Object>>>, b: Rc<RefCell<Vec<Object>>>, cmp: Object) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)I")
        }

        #[java_method(name = "mismatch", descriptor = "([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<T:Ljava/lang/Object;>([TT;II[TT;IILjava/util/Comparator<-TT;>;)I")]
        pub fn mismatch_arr_obj_i_i_arr_obj_i_i_compar(a: Rc<RefCell<Vec<Object>>>, aFromIndex: i32, aToIndex: i32, b: Rc<RefCell<Vec<Object>>>, bFromIndex: i32, bToIndex: i32, cmp: Object) -> Result<i32> {
            panic!("stub: java/util/Arrays.mismatch:([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)I")
        }
    }
}
