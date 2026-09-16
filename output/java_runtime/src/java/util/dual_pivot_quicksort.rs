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

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/DualPivotQuicksort"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "DualPivotQuicksort.java"]
    #[inner_classes     = "java/util/DualPivotQuicksort$Sorter:java/util/DualPivotQuicksort:Sorter:26;java/util/DualPivotQuicksort$RunMerger:java/util/DualPivotQuicksort:RunMerger:26;java/util/DualPivotQuicksort$Merger:java/util/DualPivotQuicksort:Merger:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/DualPivotQuicksort"]

    pub struct DualPivotQuicksort;

    impl DualPivotQuicksort {
        #[cfg_attr(any(), java_field(name = "MAX_MIXED_INSERTION_SORT_SIZE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "65"))]
        // static field: MAX_MIXED_INSERTION_SORT_SIZE:I
        pub fn MAX_MIXED_INSERTION_SORT_SIZE() -> i32 {
            65
        }

        #[cfg_attr(any(), java_field(name = "MAX_INSERTION_SORT_SIZE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "44"))]
        // static field: MAX_INSERTION_SORT_SIZE:I
        pub fn MAX_INSERTION_SORT_SIZE() -> i32 {
            44
        }

        #[cfg_attr(any(), java_field(name = "MIN_PARALLEL_SORT_SIZE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "4096"))]
        // static field: MIN_PARALLEL_SORT_SIZE:I
        pub fn MIN_PARALLEL_SORT_SIZE() -> i32 {
            4096
        }

        #[cfg_attr(any(), java_field(name = "MIN_TRY_MERGE_SIZE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "4096"))]
        // static field: MIN_TRY_MERGE_SIZE:I
        pub fn MIN_TRY_MERGE_SIZE() -> i32 {
            4096
        }

        #[cfg_attr(any(), java_field(name = "MIN_FIRST_RUN_SIZE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: MIN_FIRST_RUN_SIZE:I
        pub fn MIN_FIRST_RUN_SIZE() -> i32 {
            16
        }

        #[cfg_attr(any(), java_field(name = "MIN_FIRST_RUNS_FACTOR", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "7"))]
        // static field: MIN_FIRST_RUNS_FACTOR:I
        pub fn MIN_FIRST_RUNS_FACTOR() -> i32 {
            7
        }

        #[cfg_attr(any(), java_field(name = "MAX_RUN_CAPACITY", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "5120"))]
        // static field: MAX_RUN_CAPACITY:I
        pub fn MAX_RUN_CAPACITY() -> i32 {
            5120
        }

        #[cfg_attr(any(), java_field(name = "MIN_RUN_COUNT", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: MIN_RUN_COUNT:I
        pub fn MIN_RUN_COUNT() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "MIN_PARALLEL_MERGE_PARTS_SIZE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "4096"))]
        // static field: MIN_PARALLEL_MERGE_PARTS_SIZE:I
        pub fn MIN_PARALLEL_MERGE_PARTS_SIZE() -> i32 {
            4096
        }

        #[cfg_attr(any(), java_field(name = "MIN_BYTE_COUNTING_SORT_SIZE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "64"))]
        // static field: MIN_BYTE_COUNTING_SORT_SIZE:I
        pub fn MIN_BYTE_COUNTING_SORT_SIZE() -> i32 {
            64
        }

        #[cfg_attr(any(), java_field(name = "MIN_SHORT_OR_CHAR_COUNTING_SORT_SIZE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1750"))]
        // static field: MIN_SHORT_OR_CHAR_COUNTING_SORT_SIZE:I
        pub fn MIN_SHORT_OR_CHAR_COUNTING_SORT_SIZE() -> i32 {
            1750
        }

        #[cfg_attr(any(), java_field(name = "DELTA", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "6"))]
        // static field: DELTA:I
        pub fn DELTA() -> i32 {
            6
        }

        #[cfg_attr(any(), java_field(name = "MAX_RECURSION_DEPTH", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "384"))]
        // static field: MAX_RECURSION_DEPTH:I
        pub fn MAX_RECURSION_DEPTH() -> i32 {
            384
        }

        #[cfg_attr(any(), java_field(name = "NUM_BYTE_VALUES", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "256"))]
        // static field: NUM_BYTE_VALUES:I
        pub fn NUM_BYTE_VALUES() -> i32 {
            256
        }

        #[cfg_attr(any(), java_field(name = "MAX_BYTE_INDEX", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "384"))]
        // static field: MAX_BYTE_INDEX:I
        pub fn MAX_BYTE_INDEX() -> i32 {
            384
        }

        #[cfg_attr(any(), java_field(name = "NUM_CHAR_VALUES", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "65536"))]
        // static field: NUM_CHAR_VALUES:I
        pub fn NUM_CHAR_VALUES() -> i32 {
            65536
        }

        #[cfg_attr(any(), java_field(name = "NUM_SHORT_VALUES", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "65536"))]
        // static field: NUM_SHORT_VALUES:I
        pub fn NUM_SHORT_VALUES() -> i32 {
            65536
        }

        #[cfg_attr(any(), java_field(name = "MAX_SHORT_INDEX", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "98304"))]
        // static field: MAX_SHORT_INDEX:I
        pub fn MAX_SHORT_INDEX() -> i32 {
            98304
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/DualPivotQuicksort.<init>:()V")
        }

        #[java_method(name = "getDepth", descriptor = "(II)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDepth(mut parallelism: i32, mut size: i32) -> Result<i32> {
            let mut depth: i32 = 0i32;
            loop {
                parallelism = (parallelism>>((3i32&0x1f)));
                if ((parallelism>>((3i32&0x1f)))<=0) { break; }
                size = (size>>((2i32&0x1f)));
                if ((size>>((2i32&0x1f)))>0) {
                    depth = depth.wrapping_sub(2i32);
                    continue;
                }
                break;
            }
            Ok(depth)
        }

        #[java_method(name = "sort", descriptor = "([IIII)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: sort([IIII)V
        pub fn sort_arr_i_i_i_i(mut a: Rc<RefCell<Vec<i32>>>, mut parallelism: i32, mut low: i32, mut high: i32) -> Result<()> {
            let mut size = (high).wrapping_sub(low);
            if parallelism > 1i32 {
                if size > 4096i32 {
                    let _t0: i32 = DualPivotQuicksort::getDepth(parallelism, (size>>((12i32&0x1f))))?;
                    let mut depth: i32 = _t0;
                    let mut _merged2: Object;
                    if (depth==0) {
                        _merged2 = Object::default();
                    } else {
                        let mut _arr1: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; size as usize]));
                        _merged2 = Object::from_any(Clone::clone(&_arr1));
                    }
                    let mut b: Object = _merged2;
                    let _t3 = DualPivotQuicksort_Sorter::new(Default::default(), Object::from_any(a.clone()), Clone::clone(&b), low, size, low, depth)?.__super().__super().invoke()?;
                } else {
                    DualPivotQuicksort::sort_dualpi_arr_i_i_i_i(Default::default(), Clone::clone(&a), 0i32, low, high)?;
                }
            } else {
                DualPivotQuicksort::sort_dualpi_arr_i_i_i_i(Default::default(), Clone::clone(&a), 0i32, low, high)?;
            }
            Ok(())
        }

        #[java_method(name = "sort", descriptor = "(Ljava/util/DualPivotQuicksort$Sorter;[IIII)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: sort(Ljava/util/DualPivotQuicksort$Sorter;[IIII)V
        pub fn sort_dualpi_arr_i_i_i_i(mut sorter: DualPivotQuicksort_Sorter, mut a: Rc<RefCell<Vec<i32>>>, mut bits: i32, mut low: i32, mut high: i32) -> Result<()> {
            let mut t = Default::default();
            loop {
                let mut end = (high).wrapping_sub(1i32);
                let mut size = (high).wrapping_sub(low);
                if ((bits&1i32)>0) {
                    DualPivotQuicksort::mixedInsertionSort_arr_i_i_i_i(Clone::clone(&a), low, (high).wrapping_sub((3i32).wrapping_mul(((size>>((5i32&0x1f)))<<(3i32&0x1f)))), high)?;
                    return Ok(());
                }
                if size < 44i32 {
                    DualPivotQuicksort::insertionSort_arr_i_i_i(Clone::clone(&a), low, high)?;
                    return Ok(());
                }
                let _t0: bool = DualPivotQuicksort::tryMergeRuns_dualpi_arr_i_i_i(Clone::clone(&sorter), Clone::clone(&a), low, size)?;
                if _t0 {
                    return Ok(());
                }
                bits = bits.wrapping_add(6i32);
                if bits > 384i32 {
                    DualPivotQuicksort::heapSort_arr_i_i_i(Clone::clone(&a), low, high)?;
                    return Ok(());
                }
                let mut step = (((size>>((3i32&0x1f)))).wrapping_mul(3i32)).wrapping_add(3i32);
                let mut e1 = (low).wrapping_add(step);
                let mut e5 = (end).wrapping_sub(step);
                let mut e3 = (((e1).wrapping_add(e5) as u32>>(1i32&0x1f)) as i32);
                let mut e2 = (((e1).wrapping_add(e3) as u32>>(1i32&0x1f)) as i32);
                let mut e4 = (((e3).wrapping_add(e5) as u32>>(1i32&0x1f)) as i32);
                let mut a3 = a.borrow()[e3 as usize];
                if a.borrow()[e5 as usize] < a.borrow()[e2 as usize] {
                    t = a.borrow()[e5 as usize];
                    let _tmp_val1 = a.borrow()[e2 as usize];
                    a.borrow_mut()[e5 as usize] = _tmp_val1;
                    a.borrow_mut()[e2 as usize] = t;
                }
                if a.borrow()[e4 as usize] < a.borrow()[e1 as usize] {
                    t = a.borrow()[e4 as usize];
                    let _tmp_val1 = a.borrow()[e1 as usize];
                    a.borrow_mut()[e4 as usize] = _tmp_val1;
                    a.borrow_mut()[e1 as usize] = t;
                }
                if a.borrow()[e5 as usize] < a.borrow()[e4 as usize] {
                    t = a.borrow()[e5 as usize];
                    let _tmp_val1 = a.borrow()[e4 as usize];
                    a.borrow_mut()[e5 as usize] = _tmp_val1;
                    a.borrow_mut()[e4 as usize] = t;
                }
                if a.borrow()[e2 as usize] < a.borrow()[e1 as usize] {
                    t = a.borrow()[e2 as usize];
                    let _tmp_val1 = a.borrow()[e1 as usize];
                    a.borrow_mut()[e2 as usize] = _tmp_val1;
                    a.borrow_mut()[e1 as usize] = t;
                }
                if a.borrow()[e4 as usize] < a.borrow()[e2 as usize] {
                    t = a.borrow()[e4 as usize];
                    let _tmp_val1 = a.borrow()[e2 as usize];
                    a.borrow_mut()[e4 as usize] = _tmp_val1;
                    a.borrow_mut()[e2 as usize] = t;
                }
                if a3 < a.borrow()[e2 as usize] {
                    if a3 < a.borrow()[e1 as usize] {
                        let _tmp_val1 = a.borrow()[e2 as usize];
                        a.borrow_mut()[e3 as usize] = _tmp_val1;
                        let _tmp_val2 = a.borrow()[e1 as usize];
                        a.borrow_mut()[e2 as usize] = _tmp_val2;
                        a.borrow_mut()[e1 as usize] = a3;
                    } else {
                        let _tmp_val1 = a.borrow()[e2 as usize];
                        a.borrow_mut()[e3 as usize] = _tmp_val1;
                        a.borrow_mut()[e2 as usize] = a3;
                    }
                } else {
                    if a3 > a.borrow()[e5 as usize] {
                        let _tmp_val1 = a.borrow()[e4 as usize];
                        a.borrow_mut()[e3 as usize] = _tmp_val1;
                        let _tmp_val2 = a.borrow()[e5 as usize];
                        a.borrow_mut()[e4 as usize] = _tmp_val2;
                        a.borrow_mut()[e5 as usize] = a3;
                    } else {
                        let _tmp_val1 = a.borrow()[e4 as usize];
                        a.borrow_mut()[e3 as usize] = _tmp_val1;
                        a.borrow_mut()[e4 as usize] = a3;
                    }
                }
                t = low;
                let mut upper: i32 = end;
                if a.borrow()[e1 as usize] < a.borrow()[e2 as usize] {
                    if a.borrow()[e2 as usize] < a.borrow()[e3 as usize] {
                        if a.borrow()[e3 as usize] < a.borrow()[e4 as usize] {
                            if a.borrow()[e4 as usize] < a.borrow()[e5 as usize] {
                                let mut pivot1 = a.borrow()[e1 as usize];
                                let mut pivot2 = a.borrow()[e5 as usize];
                                let _tmp_val1 = a.borrow()[t as usize];
                                a.borrow_mut()[e1 as usize] = _tmp_val1;
                                let _tmp_val2 = a.borrow()[upper as usize];
                                a.borrow_mut()[e5 as usize] = _tmp_val2;
                                loop {
                                    t = t.wrapping_add(1i32);
                                    if a.borrow()[t as usize] >= pivot1 { break; }
                                }
                                loop {
                                    upper = upper.wrapping_sub(1i32);
                                    if a.borrow()[upper as usize] <= pivot2 { break; }
                                }
                                t = t.wrapping_sub(1i32);
                                let mut unused: i32 = t;
                                upper = upper.wrapping_add(1i32);
                                let mut k: i32 = upper;
                                loop {
                                    k = k.wrapping_sub(1i32);
                                    if k <= t { break; }
                                    let mut ak = a.borrow()[k as usize];
                                    if ak < pivot1 {
                                        loop {
                                            t = t.wrapping_add(1i32);
                                            if a.borrow()[t as usize] >= pivot1 { break; }
                                        }
                                        if a.borrow()[t as usize] > pivot2 {
                                            upper = upper.wrapping_sub(1i32);
                                            let _tmp_val3 = a.borrow()[upper as usize];
                                            a.borrow_mut()[k as usize] = _tmp_val3;
                                            let _tmp_val4 = a.borrow()[t as usize];
                                            a.borrow_mut()[upper as usize] = _tmp_val4;
                                        } else {
                                            let _tmp_val3 = a.borrow()[t as usize];
                                            a.borrow_mut()[k as usize] = _tmp_val3;
                                        }
                                        a.borrow_mut()[t as usize] = ak;
                                    } else {
                                        if ak > pivot2 {
                                            upper = upper.wrapping_sub(1i32);
                                            let _tmp_val3 = a.borrow()[upper as usize];
                                            a.borrow_mut()[k as usize] = _tmp_val3;
                                            a.borrow_mut()[upper as usize] = ak;
                                        }
                                    }
                                }
                                let _tmp_val3 = a.borrow()[t as usize];
                                a.borrow_mut()[low as usize] = _tmp_val3;
                                a.borrow_mut()[t as usize] = pivot1;
                                let _tmp_val4 = a.borrow()[upper as usize];
                                a.borrow_mut()[end as usize] = _tmp_val4;
                                a.borrow_mut()[upper as usize] = pivot2;
                                if size > 4096i32 {
                                    if !_is_jnull(&sorter) {
                                        sorter.forkSorter((bits|1i32), (t).wrapping_add(1i32), upper)?;
                                        sorter.forkSorter((bits|1i32), (upper).wrapping_add(1i32), high)?;
                                    } else {
                                        DualPivotQuicksort::sort_dualpi_arr_i_i_i_i(Clone::clone(&sorter), Clone::clone(&a), (bits|1i32), (t).wrapping_add(1i32), upper)?;
                                        DualPivotQuicksort::sort_dualpi_arr_i_i_i_i(Clone::clone(&sorter), Clone::clone(&a), (bits|1i32), (upper).wrapping_add(1i32), high)?;
                                    }
                                } else {
                                    DualPivotQuicksort::sort_dualpi_arr_i_i_i_i(Clone::clone(&sorter), Clone::clone(&a), (bits|1i32), (t).wrapping_add(1i32), upper)?;
                                    DualPivotQuicksort::sort_dualpi_arr_i_i_i_i(Clone::clone(&sorter), Clone::clone(&a), (bits|1i32), (upper).wrapping_add(1i32), high)?;
                                }
                            } else {
                                let mut pivot1 = a.borrow()[e3 as usize];
                                let _tmp_val1 = a.borrow()[t as usize];
                                a.borrow_mut()[e3 as usize] = _tmp_val1;
                                upper = upper.wrapping_add(1i32);
                                let mut pivot2: i32 = upper;
                                loop {
                                    pivot2 = pivot2.wrapping_sub(1i32);
                                    if pivot2 <= t { break; }
                                    let mut unused = a.borrow()[pivot2 as usize];
                                    a.borrow_mut()[pivot2 as usize] = pivot1;
                                    if unused < pivot1 {
                                        loop {
                                            t = t.wrapping_add(1i32);
                                            if a.borrow()[t as usize] >= pivot1 { break; }
                                        }
                                        if a.borrow()[t as usize] > pivot1 {
                                            upper = upper.wrapping_sub(1i32);
                                            let _tmp_val2 = a.borrow()[t as usize];
                                            a.borrow_mut()[upper as usize] = _tmp_val2;
                                        }
                                        a.borrow_mut()[t as usize] = unused;
                                    } else {
                                        upper = upper.wrapping_sub(1i32);
                                        a.borrow_mut()[upper as usize] = unused;
                                    }
                                }
                                let _tmp_val2 = a.borrow()[t as usize];
                                a.borrow_mut()[low as usize] = _tmp_val2;
                                a.borrow_mut()[t as usize] = pivot1;
                                if size > 4096i32 {
                                    if !_is_jnull(&sorter) {
                                        sorter.forkSorter((bits|1i32), upper, high)?;
                                    } else {
                                        DualPivotQuicksort::sort_dualpi_arr_i_i_i_i(Clone::clone(&sorter), Clone::clone(&a), (bits|1i32), upper, high)?;
                                    }
                                } else {
                                    DualPivotQuicksort::sort_dualpi_arr_i_i_i_i(Clone::clone(&sorter), Clone::clone(&a), (bits|1i32), upper, high)?;
                                }
                            }
                        } else {
                            let mut pivot1 = a.borrow()[e3 as usize];
                            let _tmp_val1 = a.borrow()[t as usize];
                            a.borrow_mut()[e3 as usize] = _tmp_val1;
                            upper = upper.wrapping_add(1i32);
                            let mut pivot2: i32 = upper;
                            loop {
                                pivot2 = pivot2.wrapping_sub(1i32);
                                if pivot2 <= t { break; }
                                let mut unused = a.borrow()[pivot2 as usize];
                                a.borrow_mut()[pivot2 as usize] = pivot1;
                                if unused < pivot1 {
                                    loop {
                                        t = t.wrapping_add(1i32);
                                        if a.borrow()[t as usize] >= pivot1 { break; }
                                    }
                                    if a.borrow()[t as usize] > pivot1 {
                                        upper = upper.wrapping_sub(1i32);
                                        let _tmp_val2 = a.borrow()[t as usize];
                                        a.borrow_mut()[upper as usize] = _tmp_val2;
                                    }
                                    a.borrow_mut()[t as usize] = unused;
                                } else {
                                    upper = upper.wrapping_sub(1i32);
                                    a.borrow_mut()[upper as usize] = unused;
                                }
                            }
                            let _tmp_val2 = a.borrow()[t as usize];
                            a.borrow_mut()[low as usize] = _tmp_val2;
                            a.borrow_mut()[t as usize] = pivot1;
                            if size > 4096i32 {
                                if !_is_jnull(&sorter) {
                                    sorter.forkSorter((bits|1i32), upper, high)?;
                                } else {
                                    DualPivotQuicksort::sort_dualpi_arr_i_i_i_i(Clone::clone(&sorter), Clone::clone(&a), (bits|1i32), upper, high)?;
                                }
                            } else {
                                DualPivotQuicksort::sort_dualpi_arr_i_i_i_i(Clone::clone(&sorter), Clone::clone(&a), (bits|1i32), upper, high)?;
                            }
                        }
                    } else {
                        let mut pivot1 = a.borrow()[e3 as usize];
                        let _tmp_val1 = a.borrow()[t as usize];
                        a.borrow_mut()[e3 as usize] = _tmp_val1;
                        upper = upper.wrapping_add(1i32);
                        let mut pivot2: i32 = upper;
                        loop {
                            pivot2 = pivot2.wrapping_sub(1i32);
                            if pivot2 <= t { break; }
                            let mut unused = a.borrow()[pivot2 as usize];
                            a.borrow_mut()[pivot2 as usize] = pivot1;
                            if unused < pivot1 {
                                loop {
                                    t = t.wrapping_add(1i32);
                                    if a.borrow()[t as usize] >= pivot1 { break; }
                                }
                                if a.borrow()[t as usize] > pivot1 {
                                    upper = upper.wrapping_sub(1i32);
                                    let _tmp_val2 = a.borrow()[t as usize];
                                    a.borrow_mut()[upper as usize] = _tmp_val2;
                                }
                                a.borrow_mut()[t as usize] = unused;
                            } else {
                                upper = upper.wrapping_sub(1i32);
                                a.borrow_mut()[upper as usize] = unused;
                            }
                        }
                        let _tmp_val2 = a.borrow()[t as usize];
                        a.borrow_mut()[low as usize] = _tmp_val2;
                        a.borrow_mut()[t as usize] = pivot1;
                        if size > 4096i32 {
                            if !_is_jnull(&sorter) {
                                sorter.forkSorter((bits|1i32), upper, high)?;
                            } else {
                                DualPivotQuicksort::sort_dualpi_arr_i_i_i_i(Clone::clone(&sorter), Clone::clone(&a), (bits|1i32), upper, high)?;
                            }
                        } else {
                            DualPivotQuicksort::sort_dualpi_arr_i_i_i_i(Clone::clone(&sorter), Clone::clone(&a), (bits|1i32), upper, high)?;
                        }
                    }
                } else {
                    let mut pivot1 = a.borrow()[e3 as usize];
                    let _tmp_val1 = a.borrow()[t as usize];
                    a.borrow_mut()[e3 as usize] = _tmp_val1;
                    upper = upper.wrapping_add(1i32);
                    let mut pivot2: i32 = upper;
                    loop {
                        pivot2 = pivot2.wrapping_sub(1i32);
                        if pivot2 <= t { break; }
                        let mut unused = a.borrow()[pivot2 as usize];
                        a.borrow_mut()[pivot2 as usize] = pivot1;
                        if unused < pivot1 {
                            loop {
                                t = t.wrapping_add(1i32);
                                if a.borrow()[t as usize] >= pivot1 { break; }
                            }
                            if a.borrow()[t as usize] > pivot1 {
                                upper = upper.wrapping_sub(1i32);
                                let _tmp_val2 = a.borrow()[t as usize];
                                a.borrow_mut()[upper as usize] = _tmp_val2;
                            }
                            a.borrow_mut()[t as usize] = unused;
                        } else {
                            upper = upper.wrapping_sub(1i32);
                            a.borrow_mut()[upper as usize] = unused;
                        }
                    }
                    let _tmp_val2 = a.borrow()[t as usize];
                    a.borrow_mut()[low as usize] = _tmp_val2;
                    a.borrow_mut()[t as usize] = pivot1;
                    if size > 4096i32 {
                        if !_is_jnull(&sorter) {
                            sorter.forkSorter((bits|1i32), upper, high)?;
                        } else {
                            DualPivotQuicksort::sort_dualpi_arr_i_i_i_i(Clone::clone(&sorter), Clone::clone(&a), (bits|1i32), upper, high)?;
                        }
                    } else {
                        DualPivotQuicksort::sort_dualpi_arr_i_i_i_i(Clone::clone(&sorter), Clone::clone(&a), (bits|1i32), upper, high)?;
                    }
                }
                high = t;
            }
            Ok(())
        }

        #[java_method(name = "mixedInsertionSort", descriptor = "([IIII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: mixedInsertionSort([IIII)V
        pub fn mixedInsertionSort_arr_i_i_i_i(mut a: Rc<RefCell<Vec<i32>>>, mut low: i32, mut end: i32, mut high: i32) -> Result<()> {
        let mut ai = Default::default();
            if end == high {
        ai = Default::default();
                loop {
                    low = low.wrapping_add(1i32);
                    if low >= end { break; }
                    let mut i: i32 = low;
                    ai = a.borrow()[i as usize];
                    loop {
                        i = i.wrapping_sub(1i32);
                        if ai >= a.borrow()[i as usize] { break; }
                        let _tmp_val0 = a.borrow()[i as usize];
                        a.borrow_mut()[(i).wrapping_add(1i32) as usize] = _tmp_val0;
                    }
                    a.borrow_mut()[(i).wrapping_add(1i32) as usize] = ai;
                }
            } else {
                let mut i = a.borrow()[end as usize];
                let mut p: i32 = high;
                loop {
                    low = low.wrapping_add(1i32);
                    if low >= end { break; }
                    ai = low;
                    ai = a.borrow()[ai as usize];
                    if ai < a.borrow()[(ai).wrapping_sub(1i32) as usize] {
                        ai = ai.wrapping_sub(1i32);
                        let _tmp_val0 = a.borrow()[ai as usize];
                        a.borrow_mut()[ai as usize] = _tmp_val0;
                        loop {
                            ai = ai.wrapping_sub(1i32);
                            if ai >= a.borrow()[ai as usize] { break; }
                            let _tmp_val1 = a.borrow()[ai as usize];
                            a.borrow_mut()[(ai).wrapping_add(1i32) as usize] = _tmp_val1;
                        }
                        a.borrow_mut()[(ai).wrapping_add(1i32) as usize] = ai;
                    } else {
                        loop {
                            p = p.wrapping_sub(1i32);
                            if a.borrow()[p as usize] <= i { break; }
                        }
                        if p > ai {
                            ai = a.borrow()[p as usize];
                            let _tmp_val0 = a.borrow()[ai as usize];
                            a.borrow_mut()[p as usize] = _tmp_val0;
                        }
                        loop {
                            ai = ai.wrapping_sub(1i32);
                            if ai >= a.borrow()[ai as usize] { break; }
                            let _tmp_val0 = a.borrow()[ai as usize];
                            a.borrow_mut()[(ai).wrapping_add(1i32) as usize] = _tmp_val0;
                        }
                        a.borrow_mut()[(ai).wrapping_add(1i32) as usize] = ai;
                    }
                }
                loop {
                    if low >= high { break; }
                    ai = low;
                    p = a.borrow()[ai as usize];
                    low = low.wrapping_add(1i32);
                    ai = a.borrow()[low as usize];
                    if p > ai {
                        loop {
                            ai = ai.wrapping_sub(1i32);
                            if p >= a.borrow()[ai as usize] { break; }
                            let _tmp_val0 = a.borrow()[ai as usize];
                            a.borrow_mut()[(ai).wrapping_add(2i32) as usize] = _tmp_val0;
                        }
                        ai = ai.wrapping_add(1i32);
                        a.borrow_mut()[(ai).wrapping_add(1i32) as usize] = p;
                        loop {
                            ai = ai.wrapping_sub(1i32);
                            if ai >= a.borrow()[ai as usize] { break; }
                            let _tmp_val0 = a.borrow()[ai as usize];
                            a.borrow_mut()[(ai).wrapping_add(1i32) as usize] = _tmp_val0;
                        }
                        a.borrow_mut()[(ai).wrapping_add(1i32) as usize] = ai;
                    } else {
                        loop {
                            ai = ai.wrapping_sub(1i32);
                            if ai >= a.borrow()[ai as usize] { break; }
                            let _tmp_val0 = a.borrow()[ai as usize];
                            a.borrow_mut()[(ai).wrapping_add(2i32) as usize] = _tmp_val0;
                        }
                        ai = ai.wrapping_add(1i32);
                        a.borrow_mut()[(ai).wrapping_add(1i32) as usize] = ai;
                        loop {
                            ai = ai.wrapping_sub(1i32);
                            if p >= a.borrow()[ai as usize] { break; }
                            let _tmp_val0 = a.borrow()[ai as usize];
                            a.borrow_mut()[(ai).wrapping_add(1i32) as usize] = _tmp_val0;
                        }
                        a.borrow_mut()[(ai).wrapping_add(1i32) as usize] = p;
                    }
                    low = low.wrapping_add(1i32);
                }
            }
            Ok(())
        }

        #[java_method(name = "insertionSort", descriptor = "([III)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: insertionSort([III)V
        pub fn insertionSort_arr_i_i_i(mut a: Rc<RefCell<Vec<i32>>>, mut low: i32, mut high: i32) -> Result<()> {
            let mut k: i32 = low;
            loop {
                k = k.wrapping_add(1i32);
                if k >= high { break; }
                let mut i: i32 = k;
                let mut ai = a.borrow()[i as usize];
                loop {
                    i = i.wrapping_sub(1i32);
                    if i < low { break; }
                    if ai < a.borrow()[i as usize] {
                        let _tmp_val0 = a.borrow()[i as usize];
                        a.borrow_mut()[(i).wrapping_add(1i32) as usize] = _tmp_val0;
                        continue;
                    }
                    break;
                }
                a.borrow_mut()[(i).wrapping_add(1i32) as usize] = ai;
            }
            Ok(())
        }

        #[java_method(name = "heapSort", descriptor = "([III)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: heapSort([III)V
        pub fn heapSort_arr_i_i_i(mut a: Rc<RefCell<Vec<i32>>>, mut low: i32, mut high: i32) -> Result<()> {
            let mut k = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
            loop {
                if k <= low { break; }
                k = k.wrapping_sub(1i32);
                DualPivotQuicksort::pushDown_arr_i_i_i_i_i(Clone::clone(&a), k, a.borrow()[k as usize], low, high)?;
            }
            loop {
                high = high.wrapping_sub(1i32);
                if high <= low { break; }
                k = a.borrow()[low as usize];
                DualPivotQuicksort::pushDown_arr_i_i_i_i_i(Clone::clone(&a), low, a.borrow()[high as usize], low, high)?;
                a.borrow_mut()[high as usize] = k;
            }
            Ok(())
        }

        #[java_method(name = "pushDown", descriptor = "([IIIII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: pushDown([IIIII)V
        pub fn pushDown_arr_i_i_i_i_i(mut a: Rc<RefCell<Vec<i32>>>, mut p: i32, mut value: i32, mut low: i32, mut high: i32) -> Result<()> {
            loop {
                let mut k = (((p<<(1i32&0x1f))).wrapping_sub(low)).wrapping_add(2i32);
                if k > high {
                } else {
                    if a.borrow()[k as usize] < a.borrow()[(k).wrapping_sub(1i32) as usize] {
                        k = k.wrapping_sub(1i32);
                    }
                    if a.borrow()[k as usize] <= value {
                    } else {
                        p = k;
                        let _tmp_val0 = a.borrow()[p as usize];
                        a.borrow_mut()[p as usize] = _tmp_val0;
                        continue;
                    }
                }
            }
            a.borrow_mut()[p as usize] = value;
            Ok(())
        }

        #[java_method(name = "tryMergeRuns", descriptor = "(Ljava/util/DualPivotQuicksort$Sorter;[III)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: tryMergeRuns(Ljava/util/DualPivotQuicksort$Sorter;[III)Z
        pub fn tryMergeRuns_dualpi_arr_i_i_i(mut sorter: DualPivotQuicksort_Sorter, mut a: Rc<RefCell<Vec<i32>>>, mut low: i32, mut size: i32) -> Result<bool> {
            let mut run: Object = Object::default();
            let mut high = (low).wrapping_add(size);
            let mut count: i32 = 1i32;
            let mut last: i32 = low;
            let mut k = (low).wrapping_add(1i32);
            loop {
                if k >= high { break; }
                loop {
                    k = k.wrapping_add(1i32);
                    if k >= high { break; }
                }
                if a.borrow()[(k).wrapping_sub(1i32) as usize] > a.borrow()[k as usize] {
                    loop {
                        k = k.wrapping_add(1i32);
                        if k >= high { break; }
                        if a.borrow()[(k).wrapping_sub(1i32) as usize] >= a.borrow()[k as usize] {
                            continue;
                        }
                        break;
                    }
                    let mut i = (last).wrapping_sub(1i32);
                    let mut j: i32 = k;
                    loop {
                        i = i.wrapping_add(1i32);
                        j = j.wrapping_sub(1i32);
                        if i >= j { break; }
                        if a.borrow()[i as usize] > a.borrow()[j as usize] {
                            let mut ai = a.borrow()[i as usize];
                            let _tmp_val0 = a.borrow()[j as usize];
                            a.borrow_mut()[i as usize] = _tmp_val0;
                            a.borrow_mut()[j as usize] = ai;
                            continue;
                        }
                        break;
                    }
                } else {
                    let mut i = a.borrow()[k as usize];
                    let mut run: Rc<RefCell<Vec<i32>>> = Default::default();
                    loop {
                        k = k.wrapping_add(1i32);
                        if k >= high { break; }
                        if i == a.borrow()[k as usize] {
                            continue;
                        }
                        break;
                    }
                    if k < high {
                        continue;
                    }
                }
                if _is_jnull(&run) {
                    if k == high {
                        return Ok((1i32 != 0i32));
                    }
                    if (k).wrapping_sub(low) < 16i32 {
                        return Ok((0i32 != 0i32));
                    }
                    let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; (((size>>((10i32&0x1f)))|127i32)&1023i32) as usize]));
                    let mut run = _arr0;
                    run.borrow_mut()[0i32 as usize] = low;
                } else {
                    if count > ((k).wrapping_sub(low)>>((7i32&0x1f))) {
                        return Ok((0i32 != 0i32));
                    }
                    count = count.wrapping_add(1i32);
                    if count == 5120i32 {
                        return Ok((0i32 != 0i32));
                    }
                    if count == (run.borrow().len() as i32) {
                        let _t0: Rc<RefCell<Vec<i32>>> = Arrays::copyOf_arr_i_i(Clone::clone(&run), (count<<(1i32&0x1f)))?;
                        let mut run: Rc<RefCell<Vec<i32>>> = _t0;
                    }
                }
                last = k;
                run.borrow_mut()[count as usize] = last;
            }
            let mut i: i32 = low;
            let mut k = (sorter.__get_b()).downcast::<Rc<RefCell<Vec<i32>>>>();
            if _is_jnull(&(sorter.__get_b()).downcast::<Rc<RefCell<Vec<i32>>>>()) {
                let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; size as usize]));
                k = _arr0;
            } else {
                i = sorter.__get_offset();
            }
            let _t0: Rc<RefCell<Vec<i32>>> = DualPivotQuicksort::mergeRuns_arr_i_arr_i_i_i_z_arr_i_i_i(Clone::clone(&a), Clone::clone(&k), i, 1i32, !_is_jnull(&sorter), Clone::clone(&run), 0i32, count)?;
            Ok((1i32 != 0i32))
        }

        #[java_method(name = "mergeRuns", descriptor = "([I[IIIZ[III)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: mergeRuns([I[IIIZ[III)[I
        pub fn mergeRuns_arr_i_arr_i_i_i_z_arr_i_i_i(mut a: Rc<RefCell<Vec<i32>>>, mut b: Rc<RefCell<Vec<i32>>>, mut offset: i32, mut aim: i32, mut parallel: bool, mut run: Rc<RefCell<Vec<i32>>>, mut lo: i32, mut hi: i32) -> Result<Rc<RefCell<Vec<i32>>>> {
            if (aim>=0) {
                return Ok(a);
            }
            let mut i = run.borrow()[hi as usize];
            let mut j = (i).wrapping_sub(offset);
            let mut low = run.borrow()[lo as usize];
            loop {
                if i <= low { break; }
                j = j.wrapping_sub(1i32);
                i = i.wrapping_sub(1i32);
                let _tmp_val0 = a.borrow()[i as usize];
                b.borrow_mut()[j as usize] = _tmp_val0;
            }
            return Ok(b);
            i = lo;
            j = (((run.borrow()[lo as usize]).wrapping_add(run.borrow()[hi as usize]) as u32>>(1i32&0x1f)) as i32);
            loop {
                i = i.wrapping_add(1i32);
                if run.borrow()[(i).wrapping_add(1i32) as usize] > j { break; }
            }
        let mut a2 = Default::default();
            if parallel {
                if (hi).wrapping_sub(lo) > 4i32 {
                    let _t0 = DualPivotQuicksort_RunMerger::new(Object::from_any(a.clone()), Object::from_any(b.clone()), offset, 0i32, Clone::clone(&run), i, hi)?.forkMe()?;
                    let mut merger: DualPivotQuicksort_RunMerger = _t0;
                    let _t1: Rc<RefCell<Vec<i32>>> = DualPivotQuicksort::mergeRuns_arr_i_arr_i_i_i_z_arr_i_i_i(Clone::clone(&a), Clone::clone(&b), offset, (aim).wrapping_neg(), (1i32 != 0i32), Clone::clone(&run), lo, i)?;
                    let mut low: Rc<RefCell<Vec<i32>>> = _t1;
                    let _t2 = merger.getDestination()?;
                    a2 = (_t2).downcast::<Rc<RefCell<Vec<i32>>>>();
                } else {
                    let _t0: Rc<RefCell<Vec<i32>>> = DualPivotQuicksort::mergeRuns_arr_i_arr_i_i_i_z_arr_i_i_i(Clone::clone(&a), Clone::clone(&b), offset, (aim).wrapping_neg(), (0i32 != 0i32), Clone::clone(&run), lo, i)?;
                    let mut low: Rc<RefCell<Vec<i32>>> = _t0;
                    let _t1: Rc<RefCell<Vec<i32>>> = DualPivotQuicksort::mergeRuns_arr_i_arr_i_i_i_z_arr_i_i_i(Clone::clone(&a), Clone::clone(&b), offset, 0i32, (0i32 != 0i32), Clone::clone(&run), i, hi)?;
                    a2 = _t1;
                }
            } else {
                let _t0: Rc<RefCell<Vec<i32>>> = DualPivotQuicksort::mergeRuns_arr_i_arr_i_i_i_z_arr_i_i_i(Clone::clone(&a), Clone::clone(&b), offset, (aim).wrapping_neg(), (0i32 != 0i32), Clone::clone(&run), lo, i)?;
                let mut low: Rc<RefCell<Vec<i32>>> = _t0;
                let _t1: Rc<RefCell<Vec<i32>>> = DualPivotQuicksort::mergeRuns_arr_i_arr_i_i_i_z_arr_i_i_i(Clone::clone(&a), Clone::clone(&b), offset, 0i32, (0i32 != 0i32), Clone::clone(&run), i, hi)?;
                a2 = _t1;
            }
            let mut merger = (if Object::from_any(low.clone()) == Object::from_any(a.clone()) { b } else { a });
            let mut k = (if Object::from_any(low.clone()) == Object::from_any(a.clone()) { (run.borrow()[lo as usize]).wrapping_sub(offset) } else { run.borrow()[lo as usize] });
            let mut lo1 = (if Object::from_any(low.clone()) == Object::from_any(b.clone()) { (run.borrow()[lo as usize]).wrapping_sub(offset) } else { run.borrow()[lo as usize] });
            let mut hi1 = (if Object::from_any(low.clone()) == Object::from_any(b.clone()) { (run.borrow()[i as usize]).wrapping_sub(offset) } else { run.borrow()[i as usize] });
            let mut lo2 = (if Object::from_any(a2.clone()) == Object::from_any(b.clone()) { (run.borrow()[i as usize]).wrapping_sub(offset) } else { run.borrow()[i as usize] });
            let mut hi2 = (if Object::from_any(a2.clone()) == Object::from_any(b.clone()) { (run.borrow()[hi as usize]).wrapping_sub(offset) } else { run.borrow()[hi as usize] });
            if parallel {
                let _t0 = DualPivotQuicksort_Merger::new(Default::default(), Object::from_any(merger.clone()), k, Object::from_any(low.clone()), lo1, hi1, Object::from_any(a2.clone()), lo2, hi2)?.__super().__super().invoke()?;
            } else {
                DualPivotQuicksort::mergeParts_dualpi_arr_i_i_arr_i_i_i_arr_i_i_i(Default::default(), Clone::clone(&merger), k, Clone::clone(&low), lo1, hi1, Clone::clone(&a2), lo2, hi2)?;
            }
            Ok(merger)
        }

        #[java_method(name = "mergeParts", descriptor = "(Ljava/util/DualPivotQuicksort$Merger;[II[III[III)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: mergeParts(Ljava/util/DualPivotQuicksort$Merger;[II[III[III)V
        pub fn mergeParts_dualpi_arr_i_i_arr_i_i_i_arr_i_i_i(mut merger: DualPivotQuicksort_Merger, mut dst: Rc<RefCell<Vec<i32>>>, mut k: i32, mut a1: Rc<RefCell<Vec<i32>>>, mut lo1: i32, mut hi1: i32, mut a2: Rc<RefCell<Vec<i32>>>, mut lo2: i32, mut hi2: i32) -> Result<()> {
            let mut hi: i32 = Default::default();
            let mut lo: i32 = Default::default();
            loop {
                if (hi1).wrapping_sub(lo1) < (hi2).wrapping_sub(lo2) {
                    lo = lo1;
                    lo1 = lo2;
                    lo2 = lo;
                    hi = hi1;
                    hi1 = hi2;
                    hi2 = hi;
                }
                if (hi1).wrapping_sub(lo1) < 4096i32 {
                } else {
                    lo = (((lo1).wrapping_add(hi1) as u32>>(1i32&0x1f)) as i32);
                    hi = a1.borrow()[lo as usize];
                    let mut mi2: i32 = hi2;
                    let mut loo: i32 = lo2;
                    loop {
                        if loo >= mi2 { break; }
                        let mut t = (((loo).wrapping_add(mi2) as u32>>(1i32&0x1f)) as i32);
                        if hi > a2.borrow()[t as usize] {
                            loo = (t).wrapping_add(1i32);
                        } else {
                            mi2 = t;
                        }
                    }
                    loo = (((mi2).wrapping_sub(lo2)).wrapping_add(lo)).wrapping_sub(lo1);
                    merger.forkMerger(Object::from_any(dst.clone()), (k).wrapping_add(loo), Object::from_any(a1.clone()), lo, hi1, Object::from_any(a2.clone()), mi2, hi2)?;
                    hi1 = lo;
                    hi2 = mi2;
                    continue;
                }
            }
            loop {
                if lo1 >= hi1 { break; }
                k = k.wrapping_add(1i32);
                let mut _merged0: i32;
                if a1.borrow()[lo1 as usize] < a2.borrow()[lo2 as usize] {
                    lo1 = lo1.wrapping_add(1i32);
                    _merged0 = a1.borrow()[lo1 as usize];
                } else {
                    lo2 = lo2.wrapping_add(1i32);
                    _merged0 = a2.borrow()[lo2 as usize];
                }
                dst.borrow_mut()[k as usize] = _merged0;
            }
            loop {
                if lo1 >= hi1 { break; }
                k = k.wrapping_add(1i32);
                lo1 = lo1.wrapping_add(1i32);
                dst.borrow_mut()[k as usize] = a1.borrow()[lo1 as usize];
            }
            loop {
                if lo2 >= hi2 { break; }
                k = k.wrapping_add(1i32);
                lo2 = lo2.wrapping_add(1i32);
                dst.borrow_mut()[k as usize] = a2.borrow()[lo2 as usize];
            }
            Ok(())
        }

        #[java_method(name = "sort", descriptor = "([JIII)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_l_i_i_i(a: Rc<RefCell<Vec<i64>>>, parallelism: i32, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.sort:([JIII)V")
        }

        #[java_method(name = "sort", descriptor = "(Ljava/util/DualPivotQuicksort$Sorter;[JIII)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_dualpi_arr_l_i_i_i(sorter: DualPivotQuicksort_Sorter, a: Rc<RefCell<Vec<i64>>>, bits: i32, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.sort:(Ljava/util/DualPivotQuicksort$Sorter;[JIII)V")
        }

        #[java_method(name = "mixedInsertionSort", descriptor = "([JIII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mixedInsertionSort_arr_l_i_i_i(a: Rc<RefCell<Vec<i64>>>, low: i32, end: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.mixedInsertionSort:([JIII)V")
        }

        #[java_method(name = "insertionSort", descriptor = "([JII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn insertionSort_arr_l_i_i(a: Rc<RefCell<Vec<i64>>>, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.insertionSort:([JII)V")
        }

        #[java_method(name = "heapSort", descriptor = "([JII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn heapSort_arr_l_i_i(a: Rc<RefCell<Vec<i64>>>, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.heapSort:([JII)V")
        }

        #[java_method(name = "pushDown", descriptor = "([JIJII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn pushDown_arr_l_i_l_i_i(a: Rc<RefCell<Vec<i64>>>, p: i32, value: i64, arg3: i32, low: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.pushDown:([JIJII)V")
        }

        #[java_method(name = "tryMergeRuns", descriptor = "(Ljava/util/DualPivotQuicksort$Sorter;[JII)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn tryMergeRuns_dualpi_arr_l_i_i(sorter: DualPivotQuicksort_Sorter, a: Rc<RefCell<Vec<i64>>>, low: i32, size: i32) -> Result<bool> {
            panic!("stub: java/util/DualPivotQuicksort.tryMergeRuns:(Ljava/util/DualPivotQuicksort$Sorter;[JII)Z")
        }

        #[java_method(name = "mergeRuns", descriptor = "([J[JIIZ[III)[J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mergeRuns_arr_l_arr_l_i_i_z_arr_i_i_i(a: Rc<RefCell<Vec<i64>>>, b: Rc<RefCell<Vec<i64>>>, offset: i32, aim: i32, parallel: bool, run: Rc<RefCell<Vec<i32>>>, lo: i32, hi: i32) -> Result<Rc<RefCell<Vec<i64>>>> {
            panic!("stub: java/util/DualPivotQuicksort.mergeRuns:([J[JIIZ[III)[J")
        }

        #[java_method(name = "mergeParts", descriptor = "(Ljava/util/DualPivotQuicksort$Merger;[JI[JII[JII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mergeParts_dualpi_arr_l_i_arr_l_i_i_arr_l_i_i(merger: DualPivotQuicksort_Merger, dst: Rc<RefCell<Vec<i64>>>, k: i32, a1: Rc<RefCell<Vec<i64>>>, lo1: i32, hi1: i32, a2: Rc<RefCell<Vec<i64>>>, lo2: i32, hi2: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.mergeParts:(Ljava/util/DualPivotQuicksort$Merger;[JI[JII[JII)V")
        }

        #[java_method(name = "sort", descriptor = "([BII)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_b_i_i(a: Rc<RefCell<Vec<i8>>>, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.sort:([BII)V")
        }

        #[java_method(name = "insertionSort", descriptor = "([BII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn insertionSort_arr_b_i_i(a: Rc<RefCell<Vec<i8>>>, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.insertionSort:([BII)V")
        }

        #[java_method(name = "countingSort", descriptor = "([BII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn countingSort_arr_b_i_i(a: Rc<RefCell<Vec<i8>>>, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.countingSort:([BII)V")
        }

        #[java_method(name = "sort", descriptor = "([CII)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_c_i_i(a: Rc<RefCell<Vec<u16>>>, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.sort:([CII)V")
        }

        #[java_method(name = "sort", descriptor = "([CIII)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_c_i_i_i(a: Rc<RefCell<Vec<u16>>>, bits: i32, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.sort:([CIII)V")
        }

        #[java_method(name = "insertionSort", descriptor = "([CII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn insertionSort_arr_c_i_i(a: Rc<RefCell<Vec<u16>>>, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.insertionSort:([CII)V")
        }

        #[java_method(name = "countingSort", descriptor = "([CII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn countingSort_arr_c_i_i(a: Rc<RefCell<Vec<u16>>>, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.countingSort:([CII)V")
        }

        #[java_method(name = "sort", descriptor = "([SII)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_s_i_i(a: Rc<RefCell<Vec<i16>>>, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.sort:([SII)V")
        }

        #[java_method(name = "sort", descriptor = "([SIII)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_s_i_i_i(a: Rc<RefCell<Vec<i16>>>, bits: i32, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.sort:([SIII)V")
        }

        #[java_method(name = "insertionSort", descriptor = "([SII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn insertionSort_arr_s_i_i(a: Rc<RefCell<Vec<i16>>>, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.insertionSort:([SII)V")
        }

        #[java_method(name = "countingSort", descriptor = "([SII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn countingSort_arr_s_i_i(a: Rc<RefCell<Vec<i16>>>, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.countingSort:([SII)V")
        }

        #[java_method(name = "sort", descriptor = "([FIII)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_f_i_i_i(a: Rc<RefCell<Vec<f32>>>, parallelism: i32, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.sort:([FIII)V")
        }

        #[java_method(name = "sort", descriptor = "(Ljava/util/DualPivotQuicksort$Sorter;[FIII)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_dualpi_arr_f_i_i_i(sorter: DualPivotQuicksort_Sorter, a: Rc<RefCell<Vec<f32>>>, bits: i32, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.sort:(Ljava/util/DualPivotQuicksort$Sorter;[FIII)V")
        }

        #[java_method(name = "mixedInsertionSort", descriptor = "([FIII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mixedInsertionSort_arr_f_i_i_i(a: Rc<RefCell<Vec<f32>>>, low: i32, end: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.mixedInsertionSort:([FIII)V")
        }

        #[java_method(name = "insertionSort", descriptor = "([FII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn insertionSort_arr_f_i_i(a: Rc<RefCell<Vec<f32>>>, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.insertionSort:([FII)V")
        }

        #[java_method(name = "heapSort", descriptor = "([FII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn heapSort_arr_f_i_i(a: Rc<RefCell<Vec<f32>>>, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.heapSort:([FII)V")
        }

        #[java_method(name = "pushDown", descriptor = "([FIFII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn pushDown_arr_f_i_f_i_i(a: Rc<RefCell<Vec<f32>>>, p: i32, value: f32, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.pushDown:([FIFII)V")
        }

        #[java_method(name = "tryMergeRuns", descriptor = "(Ljava/util/DualPivotQuicksort$Sorter;[FII)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn tryMergeRuns_dualpi_arr_f_i_i(sorter: DualPivotQuicksort_Sorter, a: Rc<RefCell<Vec<f32>>>, low: i32, size: i32) -> Result<bool> {
            panic!("stub: java/util/DualPivotQuicksort.tryMergeRuns:(Ljava/util/DualPivotQuicksort$Sorter;[FII)Z")
        }

        #[java_method(name = "mergeRuns", descriptor = "([F[FIIZ[III)[F", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mergeRuns_arr_f_arr_f_i_i_z_arr_i_i_i(a: Rc<RefCell<Vec<f32>>>, b: Rc<RefCell<Vec<f32>>>, offset: i32, aim: i32, parallel: bool, run: Rc<RefCell<Vec<i32>>>, lo: i32, hi: i32) -> Result<Rc<RefCell<Vec<f32>>>> {
            panic!("stub: java/util/DualPivotQuicksort.mergeRuns:([F[FIIZ[III)[F")
        }

        #[java_method(name = "mergeParts", descriptor = "(Ljava/util/DualPivotQuicksort$Merger;[FI[FII[FII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mergeParts_dualpi_arr_f_i_arr_f_i_i_arr_f_i_i(merger: DualPivotQuicksort_Merger, dst: Rc<RefCell<Vec<f32>>>, k: i32, a1: Rc<RefCell<Vec<f32>>>, lo1: i32, hi1: i32, a2: Rc<RefCell<Vec<f32>>>, lo2: i32, hi2: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.mergeParts:(Ljava/util/DualPivotQuicksort$Merger;[FI[FII[FII)V")
        }

        #[java_method(name = "sort", descriptor = "([DIII)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_arr_d_i_i_i(a: Rc<RefCell<Vec<f64>>>, parallelism: i32, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.sort:([DIII)V")
        }

        #[java_method(name = "sort", descriptor = "(Ljava/util/DualPivotQuicksort$Sorter;[DIII)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sort_dualpi_arr_d_i_i_i(sorter: DualPivotQuicksort_Sorter, a: Rc<RefCell<Vec<f64>>>, bits: i32, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.sort:(Ljava/util/DualPivotQuicksort$Sorter;[DIII)V")
        }

        #[java_method(name = "mixedInsertionSort", descriptor = "([DIII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mixedInsertionSort_arr_d_i_i_i(a: Rc<RefCell<Vec<f64>>>, low: i32, end: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.mixedInsertionSort:([DIII)V")
        }

        #[java_method(name = "insertionSort", descriptor = "([DII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn insertionSort_arr_d_i_i(a: Rc<RefCell<Vec<f64>>>, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.insertionSort:([DII)V")
        }

        #[java_method(name = "heapSort", descriptor = "([DII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn heapSort_arr_d_i_i(a: Rc<RefCell<Vec<f64>>>, low: i32, high: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.heapSort:([DII)V")
        }

        #[java_method(name = "pushDown", descriptor = "([DIDII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn pushDown_arr_d_i_d_i_i(a: Rc<RefCell<Vec<f64>>>, p: i32, value: f64, arg3: i32, low: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.pushDown:([DIDII)V")
        }

        #[java_method(name = "tryMergeRuns", descriptor = "(Ljava/util/DualPivotQuicksort$Sorter;[DII)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn tryMergeRuns_dualpi_arr_d_i_i(sorter: DualPivotQuicksort_Sorter, a: Rc<RefCell<Vec<f64>>>, low: i32, size: i32) -> Result<bool> {
            panic!("stub: java/util/DualPivotQuicksort.tryMergeRuns:(Ljava/util/DualPivotQuicksort$Sorter;[DII)Z")
        }

        #[java_method(name = "mergeRuns", descriptor = "([D[DIIZ[III)[D", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mergeRuns_arr_d_arr_d_i_i_z_arr_i_i_i(a: Rc<RefCell<Vec<f64>>>, b: Rc<RefCell<Vec<f64>>>, offset: i32, aim: i32, parallel: bool, run: Rc<RefCell<Vec<i32>>>, lo: i32, hi: i32) -> Result<Rc<RefCell<Vec<f64>>>> {
            panic!("stub: java/util/DualPivotQuicksort.mergeRuns:([D[DIIZ[III)[D")
        }

        #[java_method(name = "mergeParts", descriptor = "(Ljava/util/DualPivotQuicksort$Merger;[DI[DII[DII)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mergeParts_dualpi_arr_d_i_arr_d_i_i_arr_d_i_i(merger: DualPivotQuicksort_Merger, dst: Rc<RefCell<Vec<f64>>>, k: i32, a1: Rc<RefCell<Vec<f64>>>, lo1: i32, hi1: i32, a2: Rc<RefCell<Vec<f64>>>, lo2: i32, hi2: i32) -> Result<()> {
            panic!("stub: java/util/DualPivotQuicksort.mergeParts:(Ljava/util/DualPivotQuicksort$Merger;[DI[DII[DII)V")
        }
    }
}
