#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Arrays",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public final",
    source      = "Arrays.java",
))]
pub struct Arrays;

impl Arrays {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // java: sort([I)V
    // java: sort([I)V
    pub fn sort__arr_i(a: &[i32]) -> Result<()> {
        DualPivotQuicksort::sort__arr_i_i_i_i(&a, 0i32, 0i32, (a.len() as i32))?;
        Ok(())
    }

    // java: sort([III)V
    // java: sort([III)V
    pub fn sort__arr_i_i_i(a: &[i32], fromIndex: i32, toIndex: i32) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        DualPivotQuicksort::sort__arr_i_i_i_i(&a, 0i32, fromIndex, toIndex)?;
        Ok(())
    }

    // java: sort([J)V
    // java: sort([J)V
    pub fn sort__arr_l(a: &[i64]) -> Result<()> {
        DualPivotQuicksort::sort__arr_l_i_i_i(&a, 0i32, 0i32, (a.len() as i32))?;
        Ok(())
    }

    // java: sort([JII)V
    // java: sort([JII)V
    pub fn sort__arr_l_i_i(a: &[i64], fromIndex: i32, toIndex: i32) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        DualPivotQuicksort::sort__arr_l_i_i_i(&a, 0i32, fromIndex, toIndex)?;
        Ok(())
    }

    // java: sort([S)V
    // java: sort([S)V
    pub fn sort__arr_s(a: &[i16]) -> Result<()> {
        DualPivotQuicksort::sort__arr_s_i_i(&a, 0i32, (a.len() as i32))?;
        Ok(())
    }

    // java: sort([SII)V
    // java: sort([SII)V
    pub fn sort__arr_s_i_i(a: &[i16], fromIndex: i32, toIndex: i32) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        DualPivotQuicksort::sort__arr_s_i_i(&a, fromIndex, toIndex)?;
        Ok(())
    }

    // java: sort([C)V
    // java: sort([C)V
    pub fn sort__arr_c(a: &[u16]) -> Result<()> {
        DualPivotQuicksort::sort__arr_c_i_i(&a, 0i32, (a.len() as i32))?;
        Ok(())
    }

    // java: sort([CII)V
    // java: sort([CII)V
    pub fn sort__arr_c_i_i(a: &[u16], fromIndex: i32, toIndex: i32) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        DualPivotQuicksort::sort__arr_c_i_i(&a, fromIndex, toIndex)?;
        Ok(())
    }

    // java: sort([B)V
    // java: sort([B)V
    pub fn sort__arr_b(a: &[i8]) -> Result<()> {
        DualPivotQuicksort::sort__arr_b_i_i(&a, 0i32, (a.len() as i32))?;
        Ok(())
    }

    // java: sort([BII)V
    // java: sort([BII)V
    pub fn sort__arr_b_i_i(a: &[i8], fromIndex: i32, toIndex: i32) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        DualPivotQuicksort::sort__arr_b_i_i(&a, fromIndex, toIndex)?;
        Ok(())
    }

    // java: sort([F)V
    // java: sort([F)V
    pub fn sort__arr_f(a: &[f32]) -> Result<()> {
        DualPivotQuicksort::sort__arr_f_i_i_i(&a, 0i32, 0i32, (a.len() as i32))?;
        Ok(())
    }

    // java: sort([FII)V
    // java: sort([FII)V
    pub fn sort__arr_f_i_i(a: &[f32], fromIndex: i32, toIndex: i32) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        DualPivotQuicksort::sort__arr_f_i_i_i(&a, 0i32, fromIndex, toIndex)?;
        Ok(())
    }

    // java: sort([D)V
    // java: sort([D)V
    pub fn sort__arr_d(a: &[f64]) -> Result<()> {
        DualPivotQuicksort::sort__arr_d_i_i_i(&a, 0i32, 0i32, (a.len() as i32))?;
        Ok(())
    }

    // java: sort([DII)V
    // java: sort([DII)V
    pub fn sort__arr_d_i_i(a: &[f64], fromIndex: i32, toIndex: i32) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        DualPivotQuicksort::sort__arr_d_i_i_i(&a, 0i32, fromIndex, toIndex)?;
        Ok(())
    }

    // java: parallelSort([B)V
    // java: parallelSort([B)V
    pub fn parallelSort__arr_b(a: &[i8]) -> Result<()> {
        DualPivotQuicksort::sort__arr_b_i_i(&a, 0i32, (a.len() as i32))?;
        Ok(())
    }

    // java: parallelSort([BII)V
    // java: parallelSort([BII)V
    pub fn parallelSort__arr_b_i_i(a: &[i8], fromIndex: i32, toIndex: i32) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        DualPivotQuicksort::sort__arr_b_i_i(&a, fromIndex, toIndex)?;
        Ok(())
    }

    // java: parallelSort([C)V
    // java: parallelSort([C)V
    pub fn parallelSort__arr_c(a: &[u16]) -> Result<()> {
        DualPivotQuicksort::sort__arr_c_i_i(&a, 0i32, (a.len() as i32))?;
        Ok(())
    }

    // java: parallelSort([CII)V
    // java: parallelSort([CII)V
    pub fn parallelSort__arr_c_i_i(a: &[u16], fromIndex: i32, toIndex: i32) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        DualPivotQuicksort::sort__arr_c_i_i(&a, fromIndex, toIndex)?;
        Ok(())
    }

    // java: parallelSort([S)V
    // java: parallelSort([S)V
    pub fn parallelSort__arr_s(a: &[i16]) -> Result<()> {
        DualPivotQuicksort::sort__arr_s_i_i(&a, 0i32, (a.len() as i32))?;
        Ok(())
    }

    // java: parallelSort([SII)V
    // java: parallelSort([SII)V
    pub fn parallelSort__arr_s_i_i(a: &[i16], fromIndex: i32, toIndex: i32) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        DualPivotQuicksort::sort__arr_s_i_i(&a, fromIndex, toIndex)?;
        Ok(())
    }

    // java: parallelSort([I)V
    // java: parallelSort([I)V
    pub fn parallelSort__arr_i(a: &[i32]) -> Result<()> {
        let _t0: i32 = ForkJoinPool::getCommonPoolParallelism()?;
        DualPivotQuicksort::sort__arr_i_i_i_i(&a, _t0, 0i32, (a.len() as i32))?;
        Ok(())
    }

    // java: parallelSort([III)V
    // java: parallelSort([III)V
    pub fn parallelSort__arr_i_i_i(a: &[i32], fromIndex: i32, toIndex: i32) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let _t0: i32 = ForkJoinPool::getCommonPoolParallelism()?;
        DualPivotQuicksort::sort__arr_i_i_i_i(&a, _t0, fromIndex, toIndex)?;
        Ok(())
    }

    // java: parallelSort([J)V
    // java: parallelSort([J)V
    pub fn parallelSort__arr_l(a: &[i64]) -> Result<()> {
        let _t0: i32 = ForkJoinPool::getCommonPoolParallelism()?;
        DualPivotQuicksort::sort__arr_l_i_i_i(&a, _t0, 0i32, (a.len() as i32))?;
        Ok(())
    }

    // java: parallelSort([JII)V
    // java: parallelSort([JII)V
    pub fn parallelSort__arr_l_i_i(a: &[i64], fromIndex: i32, toIndex: i32) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let _t0: i32 = ForkJoinPool::getCommonPoolParallelism()?;
        DualPivotQuicksort::sort__arr_l_i_i_i(&a, _t0, fromIndex, toIndex)?;
        Ok(())
    }

    // java: parallelSort([F)V
    // java: parallelSort([F)V
    pub fn parallelSort__arr_f(a: &[f32]) -> Result<()> {
        let _t0: i32 = ForkJoinPool::getCommonPoolParallelism()?;
        DualPivotQuicksort::sort__arr_f_i_i_i(&a, _t0, 0i32, (a.len() as i32))?;
        Ok(())
    }

    // java: parallelSort([FII)V
    // java: parallelSort([FII)V
    pub fn parallelSort__arr_f_i_i(a: &[f32], fromIndex: i32, toIndex: i32) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let _t0: i32 = ForkJoinPool::getCommonPoolParallelism()?;
        DualPivotQuicksort::sort__arr_f_i_i_i(&a, _t0, fromIndex, toIndex)?;
        Ok(())
    }

    // java: parallelSort([D)V
    // java: parallelSort([D)V
    pub fn parallelSort__arr_d(a: &[f64]) -> Result<()> {
        let _t0: i32 = ForkJoinPool::getCommonPoolParallelism()?;
        DualPivotQuicksort::sort__arr_d_i_i_i(&a, _t0, 0i32, (a.len() as i32))?;
        Ok(())
    }

    // java: parallelSort([DII)V
    // java: parallelSort([DII)V
    pub fn parallelSort__arr_d_i_i(a: &[f64], fromIndex: i32, toIndex: i32) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let _t0: i32 = ForkJoinPool::getCommonPoolParallelism()?;
        DualPivotQuicksort::sort__arr_d_i_i_i(&a, _t0, fromIndex, toIndex)?;
        Ok(())
    }

    // java: rangeCheck(III)V
    pub fn rangeCheck(arrayLength: i32, fromIndex: i32, toIndex: i32) -> Result<()> {
        String::new().append(&String::from("fromIndex("))?;
        String::new().append(&fromIndex)?;
        String::new().append(&String::from(") > toIndex("))?;
        String::new().append(&toIndex)?;
        String::new().append(&String::from(")"))?;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: parallelSort([Ljava/lang/Comparable;)V
    // java: parallelSort([Ljava/lang/Comparable;)V
    pub fn parallelSort__arr_cmp(a: &[Object]) -> Result<()> {
        let mut n: i32 = (a.len() as i32);
        let _t0: i32 = ForkJoinPool::getCommonPoolParallelism()?;
        let mut p: i32 = _t0;
        /* TODO: aconst_null  */
        TimSort::sort(1i32, &a, 0i32, n, Arrays_NaturalOrder::INSTANCE(), 0i32, 0i32)?;
        /* TODO: aconst_null  */
        let _t1 = a.getClass()?;
        let _t2 = _t1.getComponentType()?;
        let _t3: Object = Array::newInstance(_t2, n)?;
        let mut g: i32 = (n/(p<<(2i32&0x1f)));
        /* invokespecial Method java/util/ArraysParallelSortHelpers$FJObject$Sorter.<init>:(Ljava/util/concurrent/CountedCompleter;[Ljava/lang/Object;[Ljava/lang/Object;IIIILjava/util/Comparator;)V */
        let _t4 = a.invoke()?;
        Ok(())
    }

    // java: parallelSort([Ljava/lang/Comparable;II)V
    // java: parallelSort([Ljava/lang/Comparable;II)V
    pub fn parallelSort__arr_cmp_i_i(a: &[Object], fromIndex: i32, toIndex: i32) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let mut n: i32 = (toIndex).wrapping_sub(fromIndex);
        let _t0: i32 = ForkJoinPool::getCommonPoolParallelism()?;
        let mut p: i32 = _t0;
        /* TODO: aconst_null  */
        TimSort::sort(1i32, &a, fromIndex, toIndex, Arrays_NaturalOrder::INSTANCE(), 0i32, 0i32)?;
        /* TODO: aconst_null  */
        let _t1 = a.getClass()?;
        let _t2 = _t1.getComponentType()?;
        let _t3: Object = Array::newInstance(_t2, n)?;
        let mut g: i32 = (n/(p<<(2i32&0x1f)));
        /* invokespecial Method java/util/ArraysParallelSortHelpers$FJObject$Sorter.<init>:(Ljava/util/concurrent/CountedCompleter;[Ljava/lang/Object;[Ljava/lang/Object;IIIILjava/util/Comparator;)V */
        let _t4 = a.invoke()?;
        Ok(())
    }

    // java: parallelSort([Ljava/lang/Object;Ljava/util/Comparator;)V
    // java: parallelSort([Ljava/lang/Object;Ljava/util/Comparator;)V
    pub fn parallelSort__arr_obj_compar(a: &[Object], cmp: Object) -> Result<()> {
        cmp = Arrays_NaturalOrder::INSTANCE();
        let mut n: i32 = (a.len() as i32);
        let _t0: i32 = ForkJoinPool::getCommonPoolParallelism()?;
        let mut p: i32 = _t0;
        /* TODO: aconst_null  */
        TimSort::sort(1i32, &a, 0i32, n, cmp, 0i32, 0i32)?;
        /* TODO: aconst_null  */
        let _t1 = a.getClass()?;
        let _t2 = _t1.getComponentType()?;
        let _t3: Object = Array::newInstance(_t2, n)?;
        let mut g: i32 = (n/(p<<(2i32&0x1f)));
        /* invokespecial Method java/util/ArraysParallelSortHelpers$FJObject$Sorter.<init>:(Ljava/util/concurrent/CountedCompleter;[Ljava/lang/Object;[Ljava/lang/Object;IIIILjava/util/Comparator;)V */
        let _t4 = a.invoke()?;
        Ok(())
    }

    // java: parallelSort([Ljava/lang/Object;IILjava/util/Comparator;)V
    // java: parallelSort([Ljava/lang/Object;IILjava/util/Comparator;)V
    pub fn parallelSort__arr_obj_i_i_compar(a: &[Object], fromIndex: i32, toIndex: i32, cmp: Object) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        cmp = Arrays_NaturalOrder::INSTANCE();
        let mut n: i32 = (toIndex).wrapping_sub(fromIndex);
        let _t0: i32 = ForkJoinPool::getCommonPoolParallelism()?;
        let mut p: i32 = _t0;
        /* TODO: aconst_null  */
        TimSort::sort(1i32, &a, fromIndex, toIndex, cmp, 0i32, 0i32)?;
        /* TODO: aconst_null  */
        let _t1 = a.getClass()?;
        let _t2 = _t1.getComponentType()?;
        let _t3: Object = Array::newInstance(_t2, n)?;
        let mut g: i32 = (n/(p<<(2i32&0x1f)));
        /* invokespecial Method java/util/ArraysParallelSortHelpers$FJObject$Sorter.<init>:(Ljava/util/concurrent/CountedCompleter;[Ljava/lang/Object;[Ljava/lang/Object;IIIILjava/util/Comparator;)V */
        let _t4 = a.invoke()?;
        Ok(())
    }

    // java: sort([Ljava/lang/Object;)V
    // java: sort([Ljava/lang/Object;)V
    pub fn sort__arr_obj(a: &[Object]) -> Result<()> {
        Arrays::legacyMergeSort__arr_obj(&a)?;
        /* TODO: aconst_null  */
        ComparableTimSort::sort(Arrays_LegacyMergeSort::userRequested(), &a, 0i32, (a.len() as i32), 0i32, 0i32)?;
        Ok(())
    }

    // java: legacyMergeSort([Ljava/lang/Object;)V
    // java: legacyMergeSort([Ljava/lang/Object;)V
    pub fn legacyMergeSort__arr_obj(a: &[Object]) -> Result<()> {
        let _t0 = a.clone()?;
        let mut aux: Object = _t0;
        Arrays::mergeSort__arr_obj_arr_obj_i_i_i(aux, &a, 0i32, (a.len() as i32), 0i32)?;
        Ok(())
    }

    // java: sort([Ljava/lang/Object;II)V
    // java: sort([Ljava/lang/Object;II)V
    pub fn sort__arr_obj_i_i(a: &[Object], fromIndex: i32, toIndex: i32) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        Arrays::legacyMergeSort__arr_obj_i_i(&a, fromIndex, toIndex)?;
        /* TODO: aconst_null  */
        ComparableTimSort::sort(Arrays_LegacyMergeSort::userRequested(), &a, fromIndex, toIndex, 0i32, 0i32)?;
        Ok(())
    }

    // java: legacyMergeSort([Ljava/lang/Object;II)V
    // java: legacyMergeSort([Ljava/lang/Object;II)V
    pub fn legacyMergeSort__arr_obj_i_i(a: &[Object], fromIndex: i32, toIndex: i32) -> Result<()> {
        let _t0: Vec<Object> = Arrays::copyOfRange__arr_obj_i_i(&a, fromIndex, toIndex)?;
        let mut aux: Vec<Object> = _t0;
        Arrays::mergeSort__arr_obj_arr_obj_i_i_i(&aux, &a, fromIndex, toIndex, (fromIndex).wrapping_neg())?;
        Ok(())
    }

    // java: mergeSort([Ljava/lang/Object;[Ljava/lang/Object;III)V
    // java: mergeSort([Ljava/lang/Object;[Ljava/lang/Object;III)V
    pub fn mergeSort__arr_obj_arr_obj_i_i_i(src: &[Object], dest: &[Object], low: i32, high: i32, off: i32) -> Result<()> {
        let mut length: i32 = (high).wrapping_sub(low);
        let mut i: i32 = low;
        loop {
            if i >= high { break; }
            let mut j: i32 = i;
            let _t0 = dest[(j).wrapping_sub(1i32) as usize].clone().compareTo(dest[j as usize].clone())?;
            Arrays::swap(&dest, j, (j).wrapping_sub(1i32))?;
            j = j.wrapping_sub(1i32);
            i = i.wrapping_add(1i32);
        }
        return Ok(());
        i = low;
        j = high;
        low = (low).wrapping_add(off);
        high = (high).wrapping_add(off);
        let mut mid: i32 = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
        Arrays::mergeSort__arr_obj_arr_obj_i_i_i(&dest, &src, low, mid, (off).wrapping_neg())?;
        Arrays::mergeSort__arr_obj_arr_obj_i_i_i(&dest, &src, mid, high, (off).wrapping_neg())?;
        let _t0 = src[(mid).wrapping_sub(1i32) as usize].clone().compareTo(src[mid as usize].clone())?;
        System::arraycopy(&src, low, &dest, i, length)?;
        return Ok(());
        let mut i: i32 = i;
        let mut p: i32 = low;
        let mut q: i32 = mid;
        loop {
            if i >= j { break; }
            let _t0 = src[p as usize].clone().compareTo(src[q as usize].clone())?;
            p = p.wrapping_add(1i32);
            dest[i as usize] = src[p as usize].clone();
            q = q.wrapping_add(1i32);
            dest[i as usize] = src[q as usize].clone();
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: swap([Ljava/lang/Object;II)V
    pub fn swap(x: &[Object], a: i32, b: i32) -> Result<()> {
        let mut t: Object = x[a as usize].clone();
        x[a as usize] = x[b as usize].clone();
        x[b as usize] = t;
        Ok(())
    }

    // java: sort([Ljava/lang/Object;Ljava/util/Comparator;)V
    // java: sort([Ljava/lang/Object;Ljava/util/Comparator;)V
    pub fn sort__arr_obj_compar(a: &[Object], c: Object) -> Result<()> {
        Arrays::sort__arr_obj(&a)?;
        Arrays::legacyMergeSort__arr_obj_compar(&a, c)?;
        /* TODO: aconst_null  */
        TimSort::sort(Arrays_LegacyMergeSort::userRequested(), &a, 0i32, (a.len() as i32), c, 0i32, 0i32)?;
        Ok(())
    }

    // java: legacyMergeSort([Ljava/lang/Object;Ljava/util/Comparator;)V
    // java: legacyMergeSort([Ljava/lang/Object;Ljava/util/Comparator;)V
    pub fn legacyMergeSort__arr_obj_compar(a: &[Object], c: Object) -> Result<()> {
        let _t0 = a.clone()?;
        let mut aux: Object = _t0;
        Arrays::mergeSort__arr_obj_arr_obj_i_i_i(aux, &a, 0i32, (a.len() as i32), 0i32)?;
        Arrays::mergeSort__arr_obj_arr_obj_i_i_i_compar(aux, &a, 0i32, (a.len() as i32), 0i32, c)?;
        Ok(())
    }

    // java: sort([Ljava/lang/Object;IILjava/util/Comparator;)V
    // java: sort([Ljava/lang/Object;IILjava/util/Comparator;)V
    pub fn sort__arr_obj_i_i_compar(a: &[Object], fromIndex: i32, toIndex: i32, c: Object) -> Result<()> {
        Arrays::sort__arr_obj_i_i(&a, fromIndex, toIndex)?;
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        Arrays::legacyMergeSort__arr_obj_i_i_compar(&a, fromIndex, toIndex, c)?;
        /* TODO: aconst_null  */
        TimSort::sort(Arrays_LegacyMergeSort::userRequested(), &a, fromIndex, toIndex, c, 0i32, 0i32)?;
        Ok(())
    }

    // java: legacyMergeSort([Ljava/lang/Object;IILjava/util/Comparator;)V
    // java: legacyMergeSort([Ljava/lang/Object;IILjava/util/Comparator;)V
    pub fn legacyMergeSort__arr_obj_i_i_compar(a: &[Object], fromIndex: i32, toIndex: i32, c: Object) -> Result<()> {
        let _t0: Vec<Object> = Arrays::copyOfRange__arr_obj_i_i(&a, fromIndex, toIndex)?;
        let mut aux: Vec<Object> = _t0;
        Arrays::mergeSort__arr_obj_arr_obj_i_i_i(&aux, &a, fromIndex, toIndex, (fromIndex).wrapping_neg())?;
        Arrays::mergeSort__arr_obj_arr_obj_i_i_i_compar(&aux, &a, fromIndex, toIndex, (fromIndex).wrapping_neg(), c)?;
        Ok(())
    }

    // java: mergeSort([Ljava/lang/Object;[Ljava/lang/Object;IIILjava/util/Comparator;)V
    // java: mergeSort([Ljava/lang/Object;[Ljava/lang/Object;IIILjava/util/Comparator;)V
    pub fn mergeSort__arr_obj_arr_obj_i_i_i_compar(src: &[Object], dest: &[Object], low: i32, high: i32, off: i32, c: Object) -> Result<()> {
        let mut length: i32 = (high).wrapping_sub(low);
        let mut i: i32 = low;
        loop {
            if i >= high { break; }
            let mut j: i32 = i;
            let _t0 = c.compare(dest[(j).wrapping_sub(1i32) as usize].clone(), dest[j as usize].clone())?;
            Arrays::swap(&dest, j, (j).wrapping_sub(1i32))?;
            j = j.wrapping_sub(1i32);
            i = i.wrapping_add(1i32);
        }
        return Ok(());
        i = low;
        j = high;
        low = (low).wrapping_add(off);
        high = (high).wrapping_add(off);
        let mut mid: i32 = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
        Arrays::mergeSort__arr_obj_arr_obj_i_i_i_compar(&dest, &src, low, mid, (off).wrapping_neg(), c)?;
        Arrays::mergeSort__arr_obj_arr_obj_i_i_i_compar(&dest, &src, mid, high, (off).wrapping_neg(), c)?;
        let _t0 = c.compare(src[(mid).wrapping_sub(1i32) as usize].clone(), src[mid as usize].clone())?;
        System::arraycopy(&src, low, &dest, i, length)?;
        return Ok(());
        let mut i: i32 = i;
        let mut p: i32 = low;
        let mut q: i32 = mid;
        loop {
            if i >= j { break; }
            let _t0 = c.compare(src[p as usize].clone(), src[q as usize].clone())?;
            p = p.wrapping_add(1i32);
            dest[i as usize] = src[p as usize].clone();
            q = q.wrapping_add(1i32);
            dest[i as usize] = src[q as usize].clone();
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: parallelPrefix([Ljava/lang/Object;Ljava/util/function/BinaryOperator;)V
    // java: parallelPrefix([Ljava/lang/Object;Ljava/util/function/BinaryOperator;)V
    pub fn parallelPrefix__arr_obj_binary(array: &[Object], op: Object) -> Result<()> {
        let _t0: Object = Objects::requireNonNull__obj(op)?;
        /* TODO: aconst_null  */
        let mut _obj1: ArrayPrefixHelpers_CumulateTask = ArrayPrefixHelpers_CumulateTask::new(ArrayPrefixHelpers_CumulateTask::new(), op, array, 0i32, (array.len() as i32))?;
        let _t2 = _obj1.invoke()?;
        Ok(())
    }

    // java: parallelPrefix([Ljava/lang/Object;IILjava/util/function/BinaryOperator;)V
    // java: parallelPrefix([Ljava/lang/Object;IILjava/util/function/BinaryOperator;)V
    pub fn parallelPrefix__arr_obj_i_i_binary(array: &[Object], fromIndex: i32, toIndex: i32, op: Object) -> Result<()> {
        let _t0: Object = Objects::requireNonNull__obj(op)?;
        Arrays::rangeCheck((array.len() as i32), fromIndex, toIndex)?;
        /* TODO: aconst_null  */
        let mut _obj1: ArrayPrefixHelpers_CumulateTask = ArrayPrefixHelpers_CumulateTask::new(ArrayPrefixHelpers_CumulateTask::new(), op, array, fromIndex, toIndex)?;
        let _t2 = _obj1.invoke()?;
        Ok(())
    }

    // java: parallelPrefix([JLjava/util/function/LongBinaryOperator;)V
    // java: parallelPrefix([JLjava/util/function/LongBinaryOperator;)V
    pub fn parallelPrefix__arr_l_longbi(array: &[i64], op: Object) -> Result<()> {
        let _t0: Object = Objects::requireNonNull__obj(op)?;
        /* TODO: aconst_null  */
        let mut _obj1: ArrayPrefixHelpers_LongCumulateTask = ArrayPrefixHelpers_LongCumulateTask::new(ArrayPrefixHelpers_LongCumulateTask::new(), op, array, 0i32, (array.len() as i32))?;
        let _t2 = _obj1.invoke()?;
        Ok(())
    }

    // java: parallelPrefix([JIILjava/util/function/LongBinaryOperator;)V
    // java: parallelPrefix([JIILjava/util/function/LongBinaryOperator;)V
    pub fn parallelPrefix__arr_l_i_i_longbi(array: &[i64], fromIndex: i32, toIndex: i32, op: Object) -> Result<()> {
        let _t0: Object = Objects::requireNonNull__obj(op)?;
        Arrays::rangeCheck((array.len() as i32), fromIndex, toIndex)?;
        /* TODO: aconst_null  */
        let mut _obj1: ArrayPrefixHelpers_LongCumulateTask = ArrayPrefixHelpers_LongCumulateTask::new(ArrayPrefixHelpers_LongCumulateTask::new(), op, array, fromIndex, toIndex)?;
        let _t2 = _obj1.invoke()?;
        Ok(())
    }

    // java: parallelPrefix([DLjava/util/function/DoubleBinaryOperator;)V
    // java: parallelPrefix([DLjava/util/function/DoubleBinaryOperator;)V
    pub fn parallelPrefix__arr_d_double(array: &[f64], op: Object) -> Result<()> {
        let _t0: Object = Objects::requireNonNull__obj(op)?;
        /* TODO: aconst_null  */
        let mut _obj1: ArrayPrefixHelpers_DoubleCumulateTask = ArrayPrefixHelpers_DoubleCumulateTask::new(ArrayPrefixHelpers_DoubleCumulateTask::new(), op, array, 0i32, (array.len() as i32))?;
        let _t2 = _obj1.invoke()?;
        Ok(())
    }

    // java: parallelPrefix([DIILjava/util/function/DoubleBinaryOperator;)V
    // java: parallelPrefix([DIILjava/util/function/DoubleBinaryOperator;)V
    pub fn parallelPrefix__arr_d_i_i_double(array: &[f64], fromIndex: i32, toIndex: i32, op: Object) -> Result<()> {
        let _t0: Object = Objects::requireNonNull__obj(op)?;
        Arrays::rangeCheck((array.len() as i32), fromIndex, toIndex)?;
        /* TODO: aconst_null  */
        let mut _obj1: ArrayPrefixHelpers_DoubleCumulateTask = ArrayPrefixHelpers_DoubleCumulateTask::new(ArrayPrefixHelpers_DoubleCumulateTask::new(), op, array, fromIndex, toIndex)?;
        let _t2 = _obj1.invoke()?;
        Ok(())
    }

    // java: parallelPrefix([ILjava/util/function/IntBinaryOperator;)V
    // java: parallelPrefix([ILjava/util/function/IntBinaryOperator;)V
    pub fn parallelPrefix__arr_i_intbin(array: &[i32], op: Object) -> Result<()> {
        let _t0: Object = Objects::requireNonNull__obj(op)?;
        /* TODO: aconst_null  */
        let mut _obj1: ArrayPrefixHelpers_IntCumulateTask = ArrayPrefixHelpers_IntCumulateTask::new(ArrayPrefixHelpers_IntCumulateTask::new(), op, array, 0i32, (array.len() as i32))?;
        let _t2 = _obj1.invoke()?;
        Ok(())
    }

    // java: parallelPrefix([IIILjava/util/function/IntBinaryOperator;)V
    // java: parallelPrefix([IIILjava/util/function/IntBinaryOperator;)V
    pub fn parallelPrefix__arr_i_i_i_intbin(array: &[i32], fromIndex: i32, toIndex: i32, op: Object) -> Result<()> {
        let _t0: Object = Objects::requireNonNull__obj(op)?;
        Arrays::rangeCheck((array.len() as i32), fromIndex, toIndex)?;
        /* TODO: aconst_null  */
        let mut _obj1: ArrayPrefixHelpers_IntCumulateTask = ArrayPrefixHelpers_IntCumulateTask::new(ArrayPrefixHelpers_IntCumulateTask::new(), op, array, fromIndex, toIndex)?;
        let _t2 = _obj1.invoke()?;
        Ok(())
    }

    // java: binarySearch([JJ)I
    // java: binarySearch([JJ)I
    pub fn binarySearch__arr_l_l(a: &[i64], key: i64) -> Result<i32> {
        let _t0: i32 = Arrays::binarySearch0__arr_l_i_i_l(&a, 0i32, (a.len() as i32), key)?;
        Ok(_t0)
    }

    // java: binarySearch([JIIJ)I
    // java: binarySearch([JIIJ)I
    pub fn binarySearch__arr_l_i_i_l(a: &[i64], fromIndex: i32, toIndex: i32, key: i64) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let _t0: i32 = Arrays::binarySearch0__arr_l_i_i_l(&a, fromIndex, toIndex, key)?;
        Ok(_t0)
    }

    // java: binarySearch0([JIIJ)I
    // java: binarySearch0([JIIJ)I
    pub fn binarySearch0__arr_l_i_i_l(a: &[i64], fromIndex: i32, toIndex: i32, key: i64) -> Result<i32> {
        let mut low: i32 = fromIndex;
        let mut high: i32 = (toIndex).wrapping_sub(1i32);
        loop {
            if low > high { break; }
            let mut mid: i32 = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
            let mut midVal: i64 = a[mid as usize];
            /* TODO: lcmp  */
            low = (mid).wrapping_add(1i32);
            /* TODO: lcmp  */
            high = (mid).wrapping_sub(1i32);
            return Ok(mid);
        }
        Ok(((low).wrapping_add(1i32)).wrapping_neg())
    }

    // java: binarySearch([II)I
    // java: binarySearch([II)I
    pub fn binarySearch__arr_i_i(a: &[i32], key: i32) -> Result<i32> {
        let _t0: i32 = Arrays::binarySearch0__arr_i_i_i_i(&a, 0i32, (a.len() as i32), key)?;
        Ok(_t0)
    }

    // java: binarySearch([IIII)I
    // java: binarySearch([IIII)I
    pub fn binarySearch__arr_i_i_i_i(a: &[i32], fromIndex: i32, toIndex: i32, key: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let _t0: i32 = Arrays::binarySearch0__arr_i_i_i_i(&a, fromIndex, toIndex, key)?;
        Ok(_t0)
    }

    // java: binarySearch0([IIII)I
    // java: binarySearch0([IIII)I
    pub fn binarySearch0__arr_i_i_i_i(a: &[i32], fromIndex: i32, toIndex: i32, key: i32) -> Result<i32> {
        let mut low: i32 = fromIndex;
        let mut high: i32 = (toIndex).wrapping_sub(1i32);
        loop {
            if low > high { break; }
            let mut mid: i32 = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
            let mut midVal: i32 = a[mid as usize];
            low = (mid).wrapping_add(1i32);
            high = (mid).wrapping_sub(1i32);
            return Ok(mid);
        }
        Ok(((low).wrapping_add(1i32)).wrapping_neg())
    }

    // java: binarySearch([SS)I
    // java: binarySearch([SS)I
    pub fn binarySearch__arr_s_s(a: &[i16], key: i16) -> Result<i32> {
        let _t0: i32 = Arrays::binarySearch0__arr_s_i_i_s(&a, 0i32, (a.len() as i32), key)?;
        Ok(_t0)
    }

    // java: binarySearch([SIIS)I
    // java: binarySearch([SIIS)I
    pub fn binarySearch__arr_s_i_i_s(a: &[i16], fromIndex: i32, toIndex: i32, key: i16) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let _t0: i32 = Arrays::binarySearch0__arr_s_i_i_s(&a, fromIndex, toIndex, key)?;
        Ok(_t0)
    }

    // java: binarySearch0([SIIS)I
    // java: binarySearch0([SIIS)I
    pub fn binarySearch0__arr_s_i_i_s(a: &[i16], fromIndex: i32, toIndex: i32, key: i16) -> Result<i32> {
        let mut low: i32 = fromIndex;
        let mut high: i32 = (toIndex).wrapping_sub(1i32);
        loop {
            if low > high { break; }
            let mut mid: i32 = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
            let mut midVal: i32 = a[mid as usize];
            low = (mid).wrapping_add(1i32);
            high = (mid).wrapping_sub(1i32);
            return Ok(mid);
        }
        Ok(((low).wrapping_add(1i32)).wrapping_neg())
    }

    // java: binarySearch([CC)I
    // java: binarySearch([CC)I
    pub fn binarySearch__arr_c_c(a: &[u16], key: u16) -> Result<i32> {
        let _t0: i32 = Arrays::binarySearch0__arr_c_i_i_c(&a, 0i32, (a.len() as i32), key)?;
        Ok(_t0)
    }

    // java: binarySearch([CIIC)I
    // java: binarySearch([CIIC)I
    pub fn binarySearch__arr_c_i_i_c(a: &[u16], fromIndex: i32, toIndex: i32, key: u16) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let _t0: i32 = Arrays::binarySearch0__arr_c_i_i_c(&a, fromIndex, toIndex, key)?;
        Ok(_t0)
    }

    // java: binarySearch0([CIIC)I
    // java: binarySearch0([CIIC)I
    pub fn binarySearch0__arr_c_i_i_c(a: &[u16], fromIndex: i32, toIndex: i32, key: u16) -> Result<i32> {
        let mut low: i32 = fromIndex;
        let mut high: i32 = (toIndex).wrapping_sub(1i32);
        loop {
            if low > high { break; }
            let mut mid: i32 = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
            let mut midVal: i32 = a[mid as usize];
            low = (mid).wrapping_add(1i32);
            high = (mid).wrapping_sub(1i32);
            return Ok(mid);
        }
        Ok(((low).wrapping_add(1i32)).wrapping_neg())
    }

    // java: binarySearch([BB)I
    // java: binarySearch([BB)I
    pub fn binarySearch__arr_b_b(a: &[i8], key: i8) -> Result<i32> {
        let _t0: i32 = Arrays::binarySearch0__arr_b_i_i_b(&a, 0i32, (a.len() as i32), key)?;
        Ok(_t0)
    }

    // java: binarySearch([BIIB)I
    // java: binarySearch([BIIB)I
    pub fn binarySearch__arr_b_i_i_b(a: &[i8], fromIndex: i32, toIndex: i32, key: i8) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let _t0: i32 = Arrays::binarySearch0__arr_b_i_i_b(&a, fromIndex, toIndex, key)?;
        Ok(_t0)
    }

    // java: binarySearch0([BIIB)I
    // java: binarySearch0([BIIB)I
    pub fn binarySearch0__arr_b_i_i_b(a: &[i8], fromIndex: i32, toIndex: i32, key: i8) -> Result<i32> {
        let mut low: i32 = fromIndex;
        let mut high: i32 = (toIndex).wrapping_sub(1i32);
        loop {
            if low > high { break; }
            let mut mid: i32 = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
            let mut midVal: i32 = a[mid as usize];
            low = (mid).wrapping_add(1i32);
            high = (mid).wrapping_sub(1i32);
            return Ok(mid);
        }
        Ok(((low).wrapping_add(1i32)).wrapping_neg())
    }

    // java: binarySearch([DD)I
    // java: binarySearch([DD)I
    pub fn binarySearch__arr_d_d(a: &[f64], key: f64) -> Result<i32> {
        let _t0: i32 = Arrays::binarySearch0__arr_d_i_i_d(&a, 0i32, (a.len() as i32), key)?;
        Ok(_t0)
    }

    // java: binarySearch([DIID)I
    // java: binarySearch([DIID)I
    pub fn binarySearch__arr_d_i_i_d(a: &[f64], fromIndex: i32, toIndex: i32, key: f64) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let _t0: i32 = Arrays::binarySearch0__arr_d_i_i_d(&a, fromIndex, toIndex, key)?;
        Ok(_t0)
    }

    // java: binarySearch0([DIID)I
    // java: binarySearch0([DIID)I
    pub fn binarySearch0__arr_d_i_i_d(a: &[f64], fromIndex: i32, toIndex: i32, key: f64) -> Result<i32> {
        let mut low: i32 = fromIndex;
        let mut high: i32 = (toIndex).wrapping_sub(1i32);
        loop {
            if low > high { break; }
            let mut mid: i32 = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
            let mut midVal: f64 = a[mid as usize];
            /* TODO: dcmpg  */
            low = (mid).wrapping_add(1i32);
            /* TODO: dcmpl  */
            high = (mid).wrapping_sub(1i32);
            let _t0: i64 = Double::doubleToLongBits(midVal)?;
            let mut midBits: i64 = _t0;
            let _t1: i64 = Double::doubleToLongBits(key)?;
            let mut keyBits: i64 = _t1;
            /* TODO: lcmp  */
            return Ok(mid);
            /* TODO: lcmp  */
            low = (mid).wrapping_add(1i32);
            high = (mid).wrapping_sub(1i32);
        }
        Ok(((low).wrapping_add(1i32)).wrapping_neg())
    }

    // java: binarySearch([FF)I
    // java: binarySearch([FF)I
    pub fn binarySearch__arr_f_f(a: &[f32], key: f32) -> Result<i32> {
        let _t0: i32 = Arrays::binarySearch0__arr_f_i_i_f(&a, 0i32, (a.len() as i32), key)?;
        Ok(_t0)
    }

    // java: binarySearch([FIIF)I
    // java: binarySearch([FIIF)I
    pub fn binarySearch__arr_f_i_i_f(a: &[f32], fromIndex: i32, toIndex: i32, key: f32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let _t0: i32 = Arrays::binarySearch0__arr_f_i_i_f(&a, fromIndex, toIndex, key)?;
        Ok(_t0)
    }

    // java: binarySearch0([FIIF)I
    // java: binarySearch0([FIIF)I
    pub fn binarySearch0__arr_f_i_i_f(a: &[f32], fromIndex: i32, toIndex: i32, key: f32) -> Result<i32> {
        let mut low: i32 = fromIndex;
        let mut high: i32 = (toIndex).wrapping_sub(1i32);
        loop {
            if low > high { break; }
            let mut mid: i32 = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
            let mut midVal: f32 = a[mid as usize];
            /* TODO: fcmpg  */
            low = (mid).wrapping_add(1i32);
            /* TODO: fcmpl  */
            high = (mid).wrapping_sub(1i32);
            let _t0: i32 = Float::floatToIntBits(midVal)?;
            let mut midBits: i32 = _t0;
            let _t1: i32 = Float::floatToIntBits(key)?;
            let mut keyBits: i32 = _t1;
            return Ok(mid);
            low = (mid).wrapping_add(1i32);
            high = (mid).wrapping_sub(1i32);
        }
        Ok(((low).wrapping_add(1i32)).wrapping_neg())
    }

    // java: binarySearch([Ljava/lang/Object;Ljava/lang/Object;)I
    // java: binarySearch([Ljava/lang/Object;Ljava/lang/Object;)I
    pub fn binarySearch__arr_obj_obj(a: &[Object], key: Object) -> Result<i32> {
        let _t0: i32 = Arrays::binarySearch0__arr_obj_i_i_obj(&a, 0i32, (a.len() as i32), key)?;
        Ok(_t0)
    }

    // java: binarySearch([Ljava/lang/Object;IILjava/lang/Object;)I
    // java: binarySearch([Ljava/lang/Object;IILjava/lang/Object;)I
    pub fn binarySearch__arr_obj_i_i_obj(a: &[Object], fromIndex: i32, toIndex: i32, key: Object) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let _t0: i32 = Arrays::binarySearch0__arr_obj_i_i_obj(&a, fromIndex, toIndex, key)?;
        Ok(_t0)
    }

    // java: binarySearch0([Ljava/lang/Object;IILjava/lang/Object;)I
    // java: binarySearch0([Ljava/lang/Object;IILjava/lang/Object;)I
    pub fn binarySearch0__arr_obj_i_i_obj(a: &[Object], fromIndex: i32, toIndex: i32, key: Object) -> Result<i32> {
        let mut low: i32 = fromIndex;
        let mut high: i32 = (toIndex).wrapping_sub(1i32);
        loop {
            if low > high { break; }
            let mut mid: i32 = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
            let mut midVal: Object = a[mid as usize].clone();
            let _t0 = midVal.compareTo(key)?;
            let mut cmp: i32 = _t0;
            low = (mid).wrapping_add(1i32);
            high = (mid).wrapping_sub(1i32);
            return Ok(mid);
        }
        Ok(((low).wrapping_add(1i32)).wrapping_neg())
    }

    // java: binarySearch([Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I
    // java: binarySearch([Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn binarySearch__arr_obj_obj_compar(a: &[Object], key: Object, c: Object) -> Result<i32> {
        let _t0: i32 = Arrays::binarySearch0__arr_obj_i_i_obj_compar(&a, 0i32, (a.len() as i32), key, c)?;
        Ok(_t0)
    }

    // java: binarySearch([Ljava/lang/Object;IILjava/lang/Object;Ljava/util/Comparator;)I
    // java: binarySearch([Ljava/lang/Object;IILjava/lang/Object;Ljava/util/Comparator;)I
    pub fn binarySearch__arr_obj_i_i_obj_compar(a: &[Object], fromIndex: i32, toIndex: i32, key: Object, c: Object) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let _t0: i32 = Arrays::binarySearch0__arr_obj_i_i_obj_compar(&a, fromIndex, toIndex, key, c)?;
        Ok(_t0)
    }

    // java: binarySearch0([Ljava/lang/Object;IILjava/lang/Object;Ljava/util/Comparator;)I
    // java: binarySearch0([Ljava/lang/Object;IILjava/lang/Object;Ljava/util/Comparator;)I
    pub fn binarySearch0__arr_obj_i_i_obj_compar(a: &[Object], fromIndex: i32, toIndex: i32, key: Object, c: Object) -> Result<i32> {
        let _t0: i32 = Arrays::binarySearch0__arr_obj_i_i_obj(&a, fromIndex, toIndex, key)?;
        return Ok(_t0);
        let mut low: i32 = fromIndex;
        let mut high: i32 = (toIndex).wrapping_sub(1i32);
        loop {
            if low > high { break; }
            let mut mid: i32 = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
            let mut midVal: Object = a[mid as usize].clone();
            let _t0 = c.compare(midVal, key)?;
            let mut cmp: i32 = _t0;
            low = (mid).wrapping_add(1i32);
            high = (mid).wrapping_sub(1i32);
            return Ok(mid);
        }
        Ok(((low).wrapping_add(1i32)).wrapping_neg())
    }

    // java: equals([J[J)Z
    // java: equals([J[J)Z
    pub fn equals__arr_l_arr_l(a: &[i64], a2: &[i64]) -> Result<bool> {
        return Ok(1i32);
        return Ok(0i32);
        let mut length: i32 = (a.len() as i32);
        return Ok(0i32);
        let _t0: i32 = ArraysSupport::mismatch(&a, &a2, length)?;
        Ok(_t0<0i32)
    }

    // java: equals([JII[JII)Z
    // java: equals([JII[JII)Z
    pub fn equals__arr_l_i_i_arr_l_i_i(a: &[i64], aFromIndex: i32, aToIndex: i32, b: &[i64], bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        return Ok(0i32);
        let _t0: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, aLength)?;
        Ok(_t0<0i32)
    }

    // java: equals([I[I)Z
    // java: equals([I[I)Z
    pub fn equals__arr_i_arr_i(a: &[i32], a2: &[i32]) -> Result<bool> {
        return Ok(1i32);
        return Ok(0i32);
        let mut length: i32 = (a.len() as i32);
        return Ok(0i32);
        let _t0: i32 = ArraysSupport::mismatch(&a, &a2, length)?;
        Ok(_t0<0i32)
    }

    // java: equals([III[III)Z
    // java: equals([III[III)Z
    pub fn equals__arr_i_i_i_arr_i_i_i(a: &[i32], aFromIndex: i32, aToIndex: i32, b: &[i32], bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        return Ok(0i32);
        let _t0: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, aLength)?;
        Ok(_t0<0i32)
    }

    // java: equals([S[S)Z
    // java: equals([S[S)Z
    pub fn equals__arr_s_arr_s(a: &[i16], a2: &[i16]) -> Result<bool> {
        return Ok(1i32);
        return Ok(0i32);
        let mut length: i32 = (a.len() as i32);
        return Ok(0i32);
        let _t0: i32 = ArraysSupport::mismatch(&a, &a2, length)?;
        Ok(_t0<0i32)
    }

    // java: equals([SII[SII)Z
    // java: equals([SII[SII)Z
    pub fn equals__arr_s_i_i_arr_s_i_i(a: &[i16], aFromIndex: i32, aToIndex: i32, b: &[i16], bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        return Ok(0i32);
        let _t0: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, aLength)?;
        Ok(_t0<0i32)
    }

    // java: equals([C[C)Z
    // java: equals([C[C)Z
    pub fn equals__arr_c_arr_c(a: &[u16], a2: &[u16]) -> Result<bool> {
        return Ok(1i32);
        return Ok(0i32);
        let mut length: i32 = (a.len() as i32);
        return Ok(0i32);
        let _t0: i32 = ArraysSupport::mismatch(&a, &a2, length)?;
        Ok(_t0<0i32)
    }

    // java: equals([CII[CII)Z
    // java: equals([CII[CII)Z
    pub fn equals__arr_c_i_i_arr_c_i_i(a: &[u16], aFromIndex: i32, aToIndex: i32, b: &[u16], bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        return Ok(0i32);
        let _t0: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, aLength)?;
        Ok(_t0<0i32)
    }

    // java: equals([B[B)Z
    // java: equals([B[B)Z
    pub fn equals__arr_b_arr_b(a: &[i8], a2: &[i8]) -> Result<bool> {
        return Ok(1i32);
        return Ok(0i32);
        let mut length: i32 = (a.len() as i32);
        return Ok(0i32);
        let _t0: i32 = ArraysSupport::mismatch(&a, &a2, length)?;
        Ok(_t0<0i32)
    }

    // java: equals([BII[BII)Z
    // java: equals([BII[BII)Z
    pub fn equals__arr_b_i_i_arr_b_i_i(a: &[i8], aFromIndex: i32, aToIndex: i32, b: &[i8], bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        return Ok(0i32);
        let _t0: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, aLength)?;
        Ok(_t0<0i32)
    }

    // java: equals([Z[Z)Z
    // java: equals([Z[Z)Z
    pub fn equals__arr_z_arr_z(a: &[bool], a2: &[bool]) -> Result<bool> {
        return Ok(1i32);
        return Ok(0i32);
        let mut length: i32 = (a.len() as i32);
        return Ok(0i32);
        let _t0: i32 = ArraysSupport::mismatch(&a, &a2, length)?;
        Ok(_t0<0i32)
    }

    // java: equals([ZII[ZII)Z
    // java: equals([ZII[ZII)Z
    pub fn equals__arr_z_i_i_arr_z_i_i(a: &[bool], aFromIndex: i32, aToIndex: i32, b: &[bool], bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        return Ok(0i32);
        let _t0: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, aLength)?;
        Ok(_t0<0i32)
    }

    // java: equals([D[D)Z
    // java: equals([D[D)Z
    pub fn equals__arr_d_arr_d(a: &[f64], a2: &[f64]) -> Result<bool> {
        return Ok(1i32);
        return Ok(0i32);
        let mut length: i32 = (a.len() as i32);
        return Ok(0i32);
        let _t0: i32 = ArraysSupport::mismatch(&a, &a2, length)?;
        Ok(_t0<0i32)
    }

    // java: equals([DII[DII)Z
    // java: equals([DII[DII)Z
    pub fn equals__arr_d_i_i_arr_d_i_i(a: &[f64], aFromIndex: i32, aToIndex: i32, b: &[f64], bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        return Ok(0i32);
        let _t0: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, aLength)?;
        Ok(_t0<0i32)
    }

    // java: equals([F[F)Z
    // java: equals([F[F)Z
    pub fn equals__arr_f_arr_f(a: &[f32], a2: &[f32]) -> Result<bool> {
        return Ok(1i32);
        return Ok(0i32);
        let mut length: i32 = (a.len() as i32);
        return Ok(0i32);
        let _t0: i32 = ArraysSupport::mismatch(&a, &a2, length)?;
        Ok(_t0<0i32)
    }

    // java: equals([FII[FII)Z
    // java: equals([FII[FII)Z
    pub fn equals__arr_f_i_i_arr_f_i_i(a: &[f32], aFromIndex: i32, aToIndex: i32, b: &[f32], bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        return Ok(0i32);
        let _t0: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, aLength)?;
        Ok(_t0<0i32)
    }

    // java: equals([Ljava/lang/Object;[Ljava/lang/Object;)Z
    // java: equals([Ljava/lang/Object;[Ljava/lang/Object;)Z
    pub fn equals__arr_obj_arr_obj(a: &[Object], a2: &[Object]) -> Result<bool> {
        return Ok(1i32);
        return Ok(0i32);
        let mut length: i32 = (a.len() as i32);
        return Ok(0i32);
        let mut i: i32 = 0i32;
        loop {
            if i >= length { break; }
            let _t0: bool = Objects::equals(a[i as usize].clone(), a2[i as usize].clone())?;
            return Ok(0i32);
            i = i.wrapping_add(1i32);
        }
        Ok(1i32)
    }

    // java: equals([Ljava/lang/Object;II[Ljava/lang/Object;II)Z
    // java: equals([Ljava/lang/Object;II[Ljava/lang/Object;II)Z
    pub fn equals__arr_obj_i_i_arr_obj_i_i(a: &[Object], aFromIndex: i32, aToIndex: i32, b: &[Object], bFromIndex: i32, bToIndex: i32) -> Result<bool> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        return Ok(0i32);
        let mut i: i32 = 0i32;
        loop {
            if i >= aLength { break; }
            aFromIndex = aFromIndex.wrapping_add(1i32);
            bFromIndex = bFromIndex.wrapping_add(1i32);
            let _t0: bool = Objects::equals(a[aFromIndex as usize].clone(), b[bFromIndex as usize].clone())?;
            return Ok(0i32);
            i = i.wrapping_add(1i32);
        }
        Ok(1i32)
    }

    // java: equals([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)Z
    // java: equals([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)Z
    pub fn equals__arr_obj_arr_obj_compar(a: &[Object], a2: &[Object], cmp: Object) -> Result<bool> {
        let _t0: Object = Objects::requireNonNull__obj(cmp)?;
        return Ok(1i32);
        return Ok(0i32);
        let mut length: i32 = (a.len() as i32);
        return Ok(0i32);
        let mut i: i32 = 0i32;
        loop {
            if i >= length { break; }
            let _t0 = cmp.compare(a[i as usize].clone(), a2[i as usize].clone())?;
            return Ok(0i32);
            i = i.wrapping_add(1i32);
        }
        Ok(1i32)
    }

    // java: equals([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)Z
    // java: equals([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)Z
    pub fn equals__arr_obj_i_i_arr_obj_i_i_compar(a: &[Object], aFromIndex: i32, aToIndex: i32, b: &[Object], bFromIndex: i32, bToIndex: i32, cmp: Object) -> Result<bool> {
        let _t0: Object = Objects::requireNonNull__obj(cmp)?;
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        return Ok(0i32);
        let mut i: i32 = 0i32;
        loop {
            if i >= aLength { break; }
            aFromIndex = aFromIndex.wrapping_add(1i32);
            bFromIndex = bFromIndex.wrapping_add(1i32);
            let _t0 = cmp.compare(a[aFromIndex as usize].clone(), b[bFromIndex as usize].clone())?;
            return Ok(0i32);
            i = i.wrapping_add(1i32);
        }
        Ok(1i32)
    }

    // java: fill([JJ)V
    // java: fill([JJ)V
    pub fn fill__arr_l_l(a: &[i64], val: i64) -> Result<()> {
        let mut i: i32 = 0i32;
        let mut len: i32 = (a.len() as i32);
        loop {
            if i >= len { break; }
            a[i as usize] = val;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: fill([JIIJ)V
    // java: fill([JIIJ)V
    pub fn fill__arr_l_i_i_l(a: &[i64], fromIndex: i32, toIndex: i32, val: i64) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let mut i: i32 = fromIndex;
        loop {
            if i >= toIndex { break; }
            a[i as usize] = val;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: fill([II)V
    // java: fill([II)V
    pub fn fill__arr_i_i(a: &[i32], val: i32) -> Result<()> {
        let mut i: i32 = 0i32;
        let mut len: i32 = (a.len() as i32);
        loop {
            if i >= len { break; }
            a[i as usize] = val;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: fill([IIII)V
    // java: fill([IIII)V
    pub fn fill__arr_i_i_i_i(a: &[i32], fromIndex: i32, toIndex: i32, val: i32) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let mut i: i32 = fromIndex;
        loop {
            if i >= toIndex { break; }
            a[i as usize] = val;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: fill([SS)V
    // java: fill([SS)V
    pub fn fill__arr_s_s(a: &[i16], val: i16) -> Result<()> {
        let mut i: i32 = 0i32;
        let mut len: i32 = (a.len() as i32);
        loop {
            if i >= len { break; }
            a[i as usize] = val;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: fill([SIIS)V
    // java: fill([SIIS)V
    pub fn fill__arr_s_i_i_s(a: &[i16], fromIndex: i32, toIndex: i32, val: i16) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let mut i: i32 = fromIndex;
        loop {
            if i >= toIndex { break; }
            a[i as usize] = val;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: fill([CC)V
    // java: fill([CC)V
    pub fn fill__arr_c_c(a: &[u16], val: u16) -> Result<()> {
        let mut i: i32 = 0i32;
        let mut len: i32 = (a.len() as i32);
        loop {
            if i >= len { break; }
            a[i as usize] = val;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: fill([CIIC)V
    // java: fill([CIIC)V
    pub fn fill__arr_c_i_i_c(a: &[u16], fromIndex: i32, toIndex: i32, val: u16) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let mut i: i32 = fromIndex;
        loop {
            if i >= toIndex { break; }
            a[i as usize] = val;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: fill([BB)V
    // java: fill([BB)V
    pub fn fill__arr_b_b(a: &[i8], val: i8) -> Result<()> {
        let mut i: i32 = 0i32;
        let mut len: i32 = (a.len() as i32);
        loop {
            if i >= len { break; }
            a[i as usize] = val;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: fill([BIIB)V
    // java: fill([BIIB)V
    pub fn fill__arr_b_i_i_b(a: &[i8], fromIndex: i32, toIndex: i32, val: i8) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let mut i: i32 = fromIndex;
        loop {
            if i >= toIndex { break; }
            a[i as usize] = val;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: fill([ZZ)V
    // java: fill([ZZ)V
    pub fn fill__arr_z_z(a: &[bool], val: bool) -> Result<()> {
        let mut i: i32 = 0i32;
        let mut len: i32 = (a.len() as i32);
        loop {
            if i >= len { break; }
            a[i as usize] = val;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: fill([ZIIZ)V
    // java: fill([ZIIZ)V
    pub fn fill__arr_z_i_i_z(a: &[bool], fromIndex: i32, toIndex: i32, val: bool) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let mut i: i32 = fromIndex;
        loop {
            if i >= toIndex { break; }
            a[i as usize] = val;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: fill([DD)V
    // java: fill([DD)V
    pub fn fill__arr_d_d(a: &[f64], val: f64) -> Result<()> {
        let mut i: i32 = 0i32;
        let mut len: i32 = (a.len() as i32);
        loop {
            if i >= len { break; }
            a[i as usize] = val;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: fill([DIID)V
    // java: fill([DIID)V
    pub fn fill__arr_d_i_i_d(a: &[f64], fromIndex: i32, toIndex: i32, val: f64) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let mut i: i32 = fromIndex;
        loop {
            if i >= toIndex { break; }
            a[i as usize] = val;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: fill([FF)V
    // java: fill([FF)V
    pub fn fill__arr_f_f(a: &[f32], val: f32) -> Result<()> {
        let mut i: i32 = 0i32;
        let mut len: i32 = (a.len() as i32);
        loop {
            if i >= len { break; }
            a[i as usize] = val;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: fill([FIIF)V
    // java: fill([FIIF)V
    pub fn fill__arr_f_i_i_f(a: &[f32], fromIndex: i32, toIndex: i32, val: f32) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let mut i: i32 = fromIndex;
        loop {
            if i >= toIndex { break; }
            a[i as usize] = val;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: fill([Ljava/lang/Object;Ljava/lang/Object;)V
    // java: fill([Ljava/lang/Object;Ljava/lang/Object;)V
    pub fn fill__arr_obj_obj(a: &[Object], val: Object) -> Result<()> {
        let mut i: i32 = 0i32;
        let mut len: i32 = (a.len() as i32);
        loop {
            if i >= len { break; }
            a[i as usize] = val;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: fill([Ljava/lang/Object;IILjava/lang/Object;)V
    // java: fill([Ljava/lang/Object;IILjava/lang/Object;)V
    pub fn fill__arr_obj_i_i_obj(a: &[Object], fromIndex: i32, toIndex: i32, val: Object) -> Result<()> {
        Arrays::rangeCheck((a.len() as i32), fromIndex, toIndex)?;
        let mut i: i32 = fromIndex;
        loop {
            if i >= toIndex { break; }
            a[i as usize] = val;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: copyOf([Ljava/lang/Object;I)[Ljava/lang/Object;
    // java: copyOf([Ljava/lang/Object;I)[Ljava/lang/Object;
    pub fn copyOf__arr_obj_i(original: &[Object], newLength: i32) -> Result<Vec<Object>> {
        let _t0 = original.getClass()?;
        let _t1: Vec<Object> = Arrays::copyOf__arr_obj_i_class(&original, newLength, _t0)?;
        Ok(_t1)
    }

    // java: copyOf([Ljava/lang/Object;ILjava/lang/Class;)[Ljava/lang/Object;
    // java: copyOf([Ljava/lang/Object;ILjava/lang/Class;)[Ljava/lang/Object;
    pub fn copyOf__arr_obj_i_class(original: &[Object], newLength: i32, newType: Object) -> Result<Vec<Object>> {
        let mut _arr0: Vec<Object> = Vec::with_capacity(newLength as usize);
        let _t1 = newType.getComponentType()?;
        let _t2: Object = Array::newInstance(_t1, newLength)?;
        let mut copy: Object = _t2;
        let _t3: i32 = ((original.len() as i32)).min(newLength);
        System::arraycopy(&original, 0i32, copy, 0i32, _t3)?;
        Ok(copy)
    }

    // java: copyOf([BI)[B
    // java: copyOf([BI)[B
    pub fn copyOf__arr_b_i(original: &[i8], newLength: i32) -> Result<Vec<i8>> {
        let _t0 = original.clone()?;
        return Ok(_t0);
        let mut _arr1: Vec<i8> = vec![0i8; newLength as usize];
        let mut copy: Vec<i8> = _arr1;
        let _t2: i32 = ((original.len() as i32)).min(newLength);
        System::arraycopy(&original, 0i32, &copy, 0i32, _t2)?;
        Ok(copy)
    }

    // java: copyOf([SI)[S
    // java: copyOf([SI)[S
    pub fn copyOf__arr_s_i(original: &[i16], newLength: i32) -> Result<Vec<i16>> {
        let _t0 = original.clone()?;
        return Ok(_t0);
        let mut _arr1: Vec<i16> = vec![0i16; newLength as usize];
        let mut copy: Vec<i16> = _arr1;
        let _t2: i32 = ((original.len() as i32)).min(newLength);
        System::arraycopy(&original, 0i32, &copy, 0i32, _t2)?;
        Ok(copy)
    }

    // java: copyOf([II)[I
    // java: copyOf([II)[I
    pub fn copyOf__arr_i_i(original: &[i32], newLength: i32) -> Result<Vec<i32>> {
        let _t0 = original.clone()?;
        return Ok(_t0);
        let mut _arr1: Vec<i32> = vec![0i32; newLength as usize];
        let mut copy: Vec<i32> = _arr1;
        let _t2: i32 = ((original.len() as i32)).min(newLength);
        System::arraycopy(&original, 0i32, &copy, 0i32, _t2)?;
        Ok(copy)
    }

    // java: copyOf([JI)[J
    // java: copyOf([JI)[J
    pub fn copyOf__arr_l_i(original: &[i64], newLength: i32) -> Result<Vec<i64>> {
        let _t0 = original.clone()?;
        return Ok(_t0);
        let mut _arr1: Vec<i64> = vec![0i64; newLength as usize];
        let mut copy: Vec<i64> = _arr1;
        let _t2: i32 = ((original.len() as i32)).min(newLength);
        System::arraycopy(&original, 0i32, &copy, 0i32, _t2)?;
        Ok(copy)
    }

    // java: copyOf([CI)[C
    // java: copyOf([CI)[C
    pub fn copyOf__arr_c_i(original: &[u16], newLength: i32) -> Result<Vec<u16>> {
        let _t0 = original.clone()?;
        return Ok(_t0);
        let mut _arr1: Vec<u16> = vec![0u16; newLength as usize];
        let mut copy: Vec<u16> = _arr1;
        let _t2: i32 = ((original.len() as i32)).min(newLength);
        System::arraycopy(&original, 0i32, &copy, 0i32, _t2)?;
        Ok(copy)
    }

    // java: copyOf([FI)[F
    // java: copyOf([FI)[F
    pub fn copyOf__arr_f_i(original: &[f32], newLength: i32) -> Result<Vec<f32>> {
        let _t0 = original.clone()?;
        return Ok(_t0);
        let mut _arr1: Vec<f32> = vec![0f32; newLength as usize];
        let mut copy: Vec<f32> = _arr1;
        let _t2: i32 = ((original.len() as i32)).min(newLength);
        System::arraycopy(&original, 0i32, &copy, 0i32, _t2)?;
        Ok(copy)
    }

    // java: copyOf([DI)[D
    // java: copyOf([DI)[D
    pub fn copyOf__arr_d_i(original: &[f64], newLength: i32) -> Result<Vec<f64>> {
        let _t0 = original.clone()?;
        return Ok(_t0);
        let mut _arr1: Vec<f64> = vec![0f64; newLength as usize];
        let mut copy: Vec<f64> = _arr1;
        let _t2: i32 = ((original.len() as i32)).min(newLength);
        System::arraycopy(&original, 0i32, &copy, 0i32, _t2)?;
        Ok(copy)
    }

    // java: copyOf([ZI)[Z
    // java: copyOf([ZI)[Z
    pub fn copyOf__arr_z_i(original: &[bool], newLength: i32) -> Result<Vec<bool>> {
        let _t0 = original.clone()?;
        return Ok(_t0);
        let mut _arr1: Vec<bool> = vec![false; newLength as usize];
        let mut copy: Vec<bool> = _arr1;
        let _t2: i32 = ((original.len() as i32)).min(newLength);
        System::arraycopy(&original, 0i32, &copy, 0i32, _t2)?;
        Ok(copy)
    }

    // java: copyOfRange([Ljava/lang/Object;II)[Ljava/lang/Object;
    // java: copyOfRange([Ljava/lang/Object;II)[Ljava/lang/Object;
    pub fn copyOfRange__arr_obj_i_i(original: &[Object], from: i32, to: i32) -> Result<Vec<Object>> {
        let _t0 = original.getClass()?;
        let _t1: Vec<Object> = Arrays::copyOfRange__arr_obj_i_i_class(&original, from, to, _t0)?;
        Ok(_t1)
    }

    // java: copyOfRange([Ljava/lang/Object;IILjava/lang/Class;)[Ljava/lang/Object;
    // java: copyOfRange([Ljava/lang/Object;IILjava/lang/Class;)[Ljava/lang/Object;
    pub fn copyOfRange__arr_obj_i_i_class(original: &[Object], from: i32, to: i32, newType: Object) -> Result<Vec<Object>> {
        let mut newLength: i32 = (to).wrapping_sub(from);
        String::new().append(&from)?;
        String::new().append(&String::from(">"))?;
        String::new().append(&to)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut _arr0: Vec<Object> = Vec::with_capacity(newLength as usize);
        let _t1 = newType.getComponentType()?;
        let _t2: Object = Array::newInstance(_t1, newLength)?;
        let mut copy: Object = _t2;
        let _t3: i32 = (((original.len() as i32)).wrapping_sub(from)).min(newLength);
        System::arraycopy(&original, from, copy, 0i32, _t3)?;
        Ok(copy)
    }

    // java: checkLength(II)V
    pub fn checkLength(from: i32, to: i32) -> Result<()> {
        String::new().append(&from)?;
        String::new().append(&String::from(">"))?;
        String::new().append(&to)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: copyOfRange([BII)[B
    // java: copyOfRange([BII)[B
    pub fn copyOfRange__arr_b_i_i(original: &[i8], from: i32, to: i32) -> Result<Vec<i8>> {
        let _t0: Vec<i8> = Arrays::copyOfRangeByte(&original, from, to)?;
        return Ok(_t0);
        let _t1 = original.clone()?;
        Ok(_t1)
    }

    // java: copyOfRangeByte([BII)[B
    pub fn copyOfRangeByte(original: &[i8], from: i32, to: i32) -> Result<Vec<i8>> {
        Arrays::checkLength(from, to)?;
        let mut newLength: i32 = (to).wrapping_sub(from);
        let mut _arr0: Vec<i8> = vec![0i8; newLength as usize];
        let mut copy: Vec<i8> = _arr0;
        let _t1: i32 = (((original.len() as i32)).wrapping_sub(from)).min(newLength);
        System::arraycopy(&original, from, &copy, 0i32, _t1)?;
        Ok(copy)
    }

    // java: copyOfRange([SII)[S
    // java: copyOfRange([SII)[S
    pub fn copyOfRange__arr_s_i_i(original: &[i16], from: i32, to: i32) -> Result<Vec<i16>> {
        let _t0: Vec<i16> = Arrays::copyOfRangeShort(&original, from, to)?;
        return Ok(_t0);
        let _t1 = original.clone()?;
        Ok(_t1)
    }

    // java: copyOfRangeShort([SII)[S
    pub fn copyOfRangeShort(original: &[i16], from: i32, to: i32) -> Result<Vec<i16>> {
        Arrays::checkLength(from, to)?;
        let mut newLength: i32 = (to).wrapping_sub(from);
        let mut _arr0: Vec<i16> = vec![0i16; newLength as usize];
        let mut copy: Vec<i16> = _arr0;
        let _t1: i32 = (((original.len() as i32)).wrapping_sub(from)).min(newLength);
        System::arraycopy(&original, from, &copy, 0i32, _t1)?;
        Ok(copy)
    }

    // java: copyOfRange([III)[I
    // java: copyOfRange([III)[I
    pub fn copyOfRange__arr_i_i_i(original: &[i32], from: i32, to: i32) -> Result<Vec<i32>> {
        let _t0: Vec<i32> = Arrays::copyOfRangeInt(&original, from, to)?;
        return Ok(_t0);
        let _t1 = original.clone()?;
        Ok(_t1)
    }

    // java: copyOfRangeInt([III)[I
    pub fn copyOfRangeInt(original: &[i32], from: i32, to: i32) -> Result<Vec<i32>> {
        Arrays::checkLength(from, to)?;
        let mut newLength: i32 = (to).wrapping_sub(from);
        let mut _arr0: Vec<i32> = vec![0i32; newLength as usize];
        let mut copy: Vec<i32> = _arr0;
        let _t1: i32 = (((original.len() as i32)).wrapping_sub(from)).min(newLength);
        System::arraycopy(&original, from, &copy, 0i32, _t1)?;
        Ok(copy)
    }

    // java: copyOfRange([JII)[J
    // java: copyOfRange([JII)[J
    pub fn copyOfRange__arr_l_i_i(original: &[i64], from: i32, to: i32) -> Result<Vec<i64>> {
        let _t0: Vec<i64> = Arrays::copyOfRangeLong(&original, from, to)?;
        return Ok(_t0);
        let _t1 = original.clone()?;
        Ok(_t1)
    }

    // java: copyOfRangeLong([JII)[J
    pub fn copyOfRangeLong(original: &[i64], from: i32, to: i32) -> Result<Vec<i64>> {
        Arrays::checkLength(from, to)?;
        let mut newLength: i32 = (to).wrapping_sub(from);
        let mut _arr0: Vec<i64> = vec![0i64; newLength as usize];
        let mut copy: Vec<i64> = _arr0;
        let _t1: i32 = (((original.len() as i32)).wrapping_sub(from)).min(newLength);
        System::arraycopy(&original, from, &copy, 0i32, _t1)?;
        Ok(copy)
    }

    // java: copyOfRange([CII)[C
    // java: copyOfRange([CII)[C
    pub fn copyOfRange__arr_c_i_i(original: &[u16], from: i32, to: i32) -> Result<Vec<u16>> {
        let _t0: Vec<u16> = Arrays::copyOfRangeChar(&original, from, to)?;
        return Ok(_t0);
        let _t1 = original.clone()?;
        Ok(_t1)
    }

    // java: copyOfRangeChar([CII)[C
    pub fn copyOfRangeChar(original: &[u16], from: i32, to: i32) -> Result<Vec<u16>> {
        Arrays::checkLength(from, to)?;
        let mut newLength: i32 = (to).wrapping_sub(from);
        let mut _arr0: Vec<u16> = vec![0u16; newLength as usize];
        let mut copy: Vec<u16> = _arr0;
        let _t1: i32 = (((original.len() as i32)).wrapping_sub(from)).min(newLength);
        System::arraycopy(&original, from, &copy, 0i32, _t1)?;
        Ok(copy)
    }

    // java: copyOfRange([FII)[F
    // java: copyOfRange([FII)[F
    pub fn copyOfRange__arr_f_i_i(original: &[f32], from: i32, to: i32) -> Result<Vec<f32>> {
        let _t0: Vec<f32> = Arrays::copyOfRangeFloat(&original, from, to)?;
        return Ok(_t0);
        let _t1 = original.clone()?;
        Ok(_t1)
    }

    // java: copyOfRangeFloat([FII)[F
    pub fn copyOfRangeFloat(original: &[f32], from: i32, to: i32) -> Result<Vec<f32>> {
        Arrays::checkLength(from, to)?;
        let mut newLength: i32 = (to).wrapping_sub(from);
        let mut _arr0: Vec<f32> = vec![0f32; newLength as usize];
        let mut copy: Vec<f32> = _arr0;
        let _t1: i32 = (((original.len() as i32)).wrapping_sub(from)).min(newLength);
        System::arraycopy(&original, from, &copy, 0i32, _t1)?;
        Ok(copy)
    }

    // java: copyOfRange([DII)[D
    // java: copyOfRange([DII)[D
    pub fn copyOfRange__arr_d_i_i(original: &[f64], from: i32, to: i32) -> Result<Vec<f64>> {
        let _t0: Vec<f64> = Arrays::copyOfRangeDouble(&original, from, to)?;
        return Ok(_t0);
        let _t1 = original.clone()?;
        Ok(_t1)
    }

    // java: copyOfRangeDouble([DII)[D
    pub fn copyOfRangeDouble(original: &[f64], from: i32, to: i32) -> Result<Vec<f64>> {
        Arrays::checkLength(from, to)?;
        let mut newLength: i32 = (to).wrapping_sub(from);
        let mut _arr0: Vec<f64> = vec![0f64; newLength as usize];
        let mut copy: Vec<f64> = _arr0;
        let _t1: i32 = (((original.len() as i32)).wrapping_sub(from)).min(newLength);
        System::arraycopy(&original, from, &copy, 0i32, _t1)?;
        Ok(copy)
    }

    // java: copyOfRange([ZII)[Z
    // java: copyOfRange([ZII)[Z
    pub fn copyOfRange__arr_z_i_i(original: &[bool], from: i32, to: i32) -> Result<Vec<bool>> {
        let _t0: Vec<bool> = Arrays::copyOfRangeBoolean(&original, from, to)?;
        return Ok(_t0);
        let _t1 = original.clone()?;
        Ok(_t1)
    }

    // java: copyOfRangeBoolean([ZII)[Z
    pub fn copyOfRangeBoolean(original: &[bool], from: i32, to: i32) -> Result<Vec<bool>> {
        Arrays::checkLength(from, to)?;
        let mut newLength: i32 = (to).wrapping_sub(from);
        let mut _arr0: Vec<bool> = vec![false; newLength as usize];
        let mut copy: Vec<bool> = _arr0;
        let _t1: i32 = (((original.len() as i32)).wrapping_sub(from)).min(newLength);
        System::arraycopy(&original, from, &copy, 0i32, _t1)?;
        Ok(copy)
    }

    // java: asList([Ljava/lang/Object;)Ljava/util/List;
    pub fn asList(a: &[Object]) -> Result<Object> {
        Ok(Arrays_ArrayList::new(a)?)
    }

    // java: hashCode([J)I
    // java: hashCode([J)I
    pub fn hashCode__arr_l(a: &[i64]) -> Result<i32> {
        return Ok(0i32);
        let mut result: i32 = 1i32;
        let mut local_2: Vec<i64> = a;
        let mut local_3: i32 = (local_2.len() as i32);
        let mut local_4: i32 = 0i32;
        loop {
            if local_4 >= local_3 { break; }
            let mut element: i64 = local_2[local_4 as usize];
            /* TODO: lushr  */
            /* TODO: lxor  */
            let mut elementHash: i32 = (32i32 as i32);
            result = ((31i32).wrapping_mul(result)).wrapping_add(elementHash);
            local_4 = local_4.wrapping_add(1i32);
        }
        Ok(result)
    }

    // java: hashCode([I)I
    // java: hashCode([I)I
    pub fn hashCode__arr_i(a: &[i32]) -> Result<i32> {
        return Ok(0i32);
        /* TODO: lookupswitch default:49 0:36 1:40 */
        let _t0: i32 = ArraysSupport::vectorizedHashCode(&a, 0i32, (a.len() as i32), 1i32, 10i32)?;
        Ok(_t0)
    }

    // java: hashCode([S)I
    // java: hashCode([S)I
    pub fn hashCode__arr_s(a: &[i16]) -> Result<i32> {
        return Ok(0i32);
        /* TODO: lookupswitch default:49 0:36 1:40 */
        let _t0: i32 = ArraysSupport::vectorizedHashCode(&a, 0i32, (a.len() as i32), 1i32, 9i32)?;
        Ok(_t0)
    }

    // java: hashCode([C)I
    // java: hashCode([C)I
    pub fn hashCode__arr_c(a: &[u16]) -> Result<i32> {
        return Ok(0i32);
        /* TODO: lookupswitch default:49 0:36 1:40 */
        let _t0: i32 = ArraysSupport::vectorizedHashCode(&a, 0i32, (a.len() as i32), 1i32, 5i32)?;
        Ok(_t0)
    }

    // java: hashCode([B)I
    // java: hashCode([B)I
    pub fn hashCode__arr_b(a: &[i8]) -> Result<i32> {
        return Ok(0i32);
        /* TODO: lookupswitch default:49 0:36 1:40 */
        let _t0: i32 = ArraysSupport::vectorizedHashCode(&a, 0i32, (a.len() as i32), 1i32, 8i32)?;
        Ok(_t0)
    }

    // java: hashCode([Z)I
    // java: hashCode([Z)I
    pub fn hashCode__arr_z(a: &[bool]) -> Result<i32> {
        return Ok(0i32);
        let mut result: i32 = 1i32;
        let mut local_2: Vec<bool> = a;
        let mut local_3: i32 = (local_2.len() as i32);
        let mut local_4: i32 = 0i32;
        loop {
            if local_4 >= local_3 { break; }
            let mut element: i32 = local_2[local_4 as usize];
            result = (1231i32).wrapping_add(1237i32);
            local_4 = local_4.wrapping_add(1i32);
        }
        Ok(result)
    }

    // java: hashCode([F)I
    // java: hashCode([F)I
    pub fn hashCode__arr_f(a: &[f32]) -> Result<i32> {
        return Ok(0i32);
        let mut result: i32 = 1i32;
        let mut local_2: Vec<f32> = a;
        let mut local_3: i32 = (local_2.len() as i32);
        let mut local_4: i32 = 0i32;
        loop {
            if local_4 >= local_3 { break; }
            let mut element: f32 = local_2[local_4 as usize];
            let _t0: i32 = Float::floatToIntBits(element)?;
            result = ((31i32).wrapping_mul(result)).wrapping_add(_t0);
            local_4 = local_4.wrapping_add(1i32);
        }
        Ok(result)
    }

    // java: hashCode([D)I
    // java: hashCode([D)I
    pub fn hashCode__arr_d(a: &[f64]) -> Result<i32> {
        return Ok(0i32);
        let mut result: i32 = 1i32;
        let mut local_2: Vec<f64> = a;
        let mut local_3: i32 = (local_2.len() as i32);
        let mut local_4: i32 = 0i32;
        loop {
            if local_4 >= local_3 { break; }
            let mut element: f64 = local_2[local_4 as usize];
            let _t0: i64 = Double::doubleToLongBits(element)?;
            let mut bits: i64 = _t0;
            /* TODO: lushr  */
            /* TODO: lxor  */
            result = (bits).wrapping_add((32i32 as i32));
            local_4 = local_4.wrapping_add(1i32);
        }
        Ok(result)
    }

    // java: hashCode([Ljava/lang/Object;)I
    // java: hashCode([Ljava/lang/Object;)I
    pub fn hashCode__arr_obj(a: &[Object]) -> Result<i32> {
        return Ok(0i32);
        let mut result: i32 = 1i32;
        let mut local_2: Vec<Object> = a;
        let mut local_3: i32 = (local_2.len() as i32);
        let mut local_4: i32 = 0i32;
        loop {
            if local_4 >= local_3 { break; }
            let mut element: Object = local_2[local_4 as usize].clone();
            let _t0 = element.hashCode()?;
            result = (0i32).wrapping_add(_t0);
            local_4 = local_4.wrapping_add(1i32);
        }
        Ok(result)
    }

    // java: deepHashCode([Ljava/lang/Object;)I
    pub fn deepHashCode(a: &[Object]) -> Result<i32> {
        return Ok(0i32);
        let mut result: i32 = 1i32;
        let mut local_2: Vec<Object> = a;
        let mut local_3: i32 = (local_2.len() as i32);
        let mut local_4: i32 = 0i32;
        loop {
            if local_4 >= local_3 { break; }
            let mut element: Object = local_2[local_4 as usize].clone();
            let mut elementHash: i32 = 0i32;
            let _t0 = element.getClass()?;
            let _t1 = _t0.getComponentType()?;
            let mut cl: Object = _t1;
            let _t2 = element.hashCode()?;
            elementHash = _t2;
            let _t3: i32 = Arrays::deepHashCode(element)?;
            elementHash = _t3;
            let _t4: i32 = Arrays::primitiveArrayHashCode(element, cl)?;
            elementHash = _t4;
            result = ((31i32).wrapping_mul(result)).wrapping_add(elementHash);
            local_4 = local_4.wrapping_add(1i32);
        }
        Ok(result)
    }

    // java: primitiveArrayHashCode(Ljava/lang/Object;Ljava/lang/Class;)I
    pub fn primitiveArrayHashCode(a: Object, cl: Object) -> Result<i32> {
        let _t0: i32 = Arrays::hashCode__arr_b(a)?;
        let _t1: i32 = Arrays::hashCode__arr_i(a)?;
        let _t2: i32 = Arrays::hashCode__arr_l(a)?;
        let _t3: i32 = Arrays::hashCode__arr_c(a)?;
        let _t4: i32 = Arrays::hashCode__arr_s(a)?;
        let _t5: i32 = Arrays::hashCode__arr_z(a)?;
        let _t6: i32 = Arrays::hashCode__arr_d(a)?;
        let _t7: i32 = Arrays::hashCode__arr_f(a)?;
        Ok(_t7)
    }

    // java: deepEquals([Ljava/lang/Object;[Ljava/lang/Object;)Z
    pub fn deepEquals(a1: &[Object], a2: &[Object]) -> Result<bool> {
        return Ok(1i32);
        return Ok(0i32);
        let mut length: i32 = (a1.len() as i32);
        return Ok(0i32);
        let mut i: i32 = 0i32;
        loop {
            if i >= length { break; }
            let mut e1: Object = a1[i as usize].clone();
            let mut e2: Object = a2[i as usize].clone();
            return Ok(0i32);
            let _t0: bool = Arrays::deepEquals0(e1, e2)?;
            let mut eq: i32 = _t0;
            return Ok(0i32);
            i = i.wrapping_add(1i32);
        }
        Ok(1i32)
    }

    // java: deepEquals0(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn deepEquals0(e1: Object, e2: Object) -> Result<bool> {
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: bool = Arrays::deepEquals(e1, e2)?;
        let mut eq: i32 = _t0;
        let _t1: bool = Arrays::equals__arr_b_arr_b(e1, e2)?;
        eq = _t1;
        let _t2: bool = Arrays::equals__arr_s_arr_s(e1, e2)?;
        eq = _t2;
        let _t3: bool = Arrays::equals__arr_i_arr_i(e1, e2)?;
        eq = _t3;
        let _t4: bool = Arrays::equals__arr_l_arr_l(e1, e2)?;
        eq = _t4;
        let _t5: bool = Arrays::equals__arr_c_arr_c(e1, e2)?;
        eq = _t5;
        let _t6: bool = Arrays::equals__arr_f_arr_f(e1, e2)?;
        eq = _t6;
        let _t7: bool = Arrays::equals__arr_d_arr_d(e1, e2)?;
        eq = _t7;
        let _t8: bool = Arrays::equals__arr_z_arr_z(e1, e2)?;
        eq = _t8;
        let _t9 = e1.equals(e2)?;
        eq = _t9;
        Ok(eq)
    }

    // java: toString([J)Ljava/lang/String;
    // java: toString([J)Ljava/lang/String;
    pub fn toString__arr_l(a: &[i64]) -> Result<String> {
        return Ok(String::from("null"));
        let mut iMax: i32 = ((a.len() as i32)).wrapping_sub(1i32);
        return Ok(String::from("[]"));
        let mut b: String = String::new();
        b.append(&91i32)?;
        let mut i: i32 = 0i32;
        b.append(&a[i as usize])?;
        b.append(&93i32)?;
        return Ok(b);
        b.append(&String::from(","))?;
        i = i.wrapping_add(1i32);
    }

    // java: toString([I)Ljava/lang/String;
    // java: toString([I)Ljava/lang/String;
    pub fn toString__arr_i(a: &[i32]) -> Result<String> {
        return Ok(String::from("null"));
        let mut iMax: i32 = ((a.len() as i32)).wrapping_sub(1i32);
        return Ok(String::from("[]"));
        let mut b: String = String::new();
        b.append(&91i32)?;
        let mut i: i32 = 0i32;
        b.append(&a[i as usize])?;
        b.append(&93i32)?;
        return Ok(b);
        b.append(&String::from(","))?;
        i = i.wrapping_add(1i32);
    }

    // java: toString([S)Ljava/lang/String;
    // java: toString([S)Ljava/lang/String;
    pub fn toString__arr_s(a: &[i16]) -> Result<String> {
        return Ok(String::from("null"));
        let mut iMax: i32 = ((a.len() as i32)).wrapping_sub(1i32);
        return Ok(String::from("[]"));
        let mut b: String = String::new();
        b.append(&91i32)?;
        let mut i: i32 = 0i32;
        b.append(&a[i as usize])?;
        b.append(&93i32)?;
        return Ok(b);
        b.append(&String::from(","))?;
        i = i.wrapping_add(1i32);
    }

    // java: toString([C)Ljava/lang/String;
    // java: toString([C)Ljava/lang/String;
    pub fn toString__arr_c(a: &[u16]) -> Result<String> {
        return Ok(String::from("null"));
        let mut iMax: i32 = ((a.len() as i32)).wrapping_sub(1i32);
        return Ok(String::from("[]"));
        let mut b: String = String::new();
        b.append(&91i32)?;
        let mut i: i32 = 0i32;
        b.append(&a[i as usize])?;
        b.append(&93i32)?;
        return Ok(b);
        b.append(&String::from(","))?;
        i = i.wrapping_add(1i32);
    }

    // java: toString([B)Ljava/lang/String;
    // java: toString([B)Ljava/lang/String;
    pub fn toString__arr_b(a: &[i8]) -> Result<String> {
        return Ok(String::from("null"));
        let mut iMax: i32 = ((a.len() as i32)).wrapping_sub(1i32);
        return Ok(String::from("[]"));
        let mut b: String = String::new();
        b.append(&91i32)?;
        let mut i: i32 = 0i32;
        b.append(&a[i as usize])?;
        b.append(&93i32)?;
        return Ok(b);
        b.append(&String::from(","))?;
        i = i.wrapping_add(1i32);
    }

    // java: toString([Z)Ljava/lang/String;
    // java: toString([Z)Ljava/lang/String;
    pub fn toString__arr_z(a: &[bool]) -> Result<String> {
        return Ok(String::from("null"));
        let mut iMax: i32 = ((a.len() as i32)).wrapping_sub(1i32);
        return Ok(String::from("[]"));
        let mut b: String = String::new();
        b.append(&91i32)?;
        let mut i: i32 = 0i32;
        b.append(&a[i as usize])?;
        b.append(&93i32)?;
        return Ok(b);
        b.append(&String::from(","))?;
        i = i.wrapping_add(1i32);
    }

    // java: toString([F)Ljava/lang/String;
    // java: toString([F)Ljava/lang/String;
    pub fn toString__arr_f(a: &[f32]) -> Result<String> {
        return Ok(String::from("null"));
        let mut iMax: i32 = ((a.len() as i32)).wrapping_sub(1i32);
        return Ok(String::from("[]"));
        let mut b: String = String::new();
        b.append(&91i32)?;
        let mut i: i32 = 0i32;
        b.append(&a[i as usize])?;
        b.append(&93i32)?;
        return Ok(b);
        b.append(&String::from(","))?;
        i = i.wrapping_add(1i32);
    }

    // java: toString([D)Ljava/lang/String;
    // java: toString([D)Ljava/lang/String;
    pub fn toString__arr_d(a: &[f64]) -> Result<String> {
        return Ok(String::from("null"));
        let mut iMax: i32 = ((a.len() as i32)).wrapping_sub(1i32);
        return Ok(String::from("[]"));
        let mut b: String = String::new();
        b.append(&91i32)?;
        let mut i: i32 = 0i32;
        b.append(&a[i as usize])?;
        b.append(&93i32)?;
        return Ok(b);
        b.append(&String::from(","))?;
        i = i.wrapping_add(1i32);
    }

    // java: toString([Ljava/lang/Object;)Ljava/lang/String;
    // java: toString([Ljava/lang/Object;)Ljava/lang/String;
    pub fn toString__arr_obj(a: &[Object]) -> Result<String> {
        return Ok(String::from("null"));
        let mut iMax: i32 = ((a.len() as i32)).wrapping_sub(1i32);
        return Ok(String::from("[]"));
        let mut b: String = String::new();
        b.append(&91i32)?;
        let mut i: i32 = 0i32;
        b.append(&String::from_owned(format!("{}", a[i as usize].clone())))?;
        b.append(&93i32)?;
        return Ok(b);
        b.append(&String::from(","))?;
        i = i.wrapping_add(1i32);
    }

    // java: deepToString([Ljava/lang/Object;)Ljava/lang/String;
    // java: deepToString([Ljava/lang/Object;)Ljava/lang/String;
    pub fn deepToString__arr_obj(a: &[Object]) -> Result<String> {
        return Ok(String::from("null"));
        let mut bufLen: i32 = (20i32).wrapping_mul((a.len() as i32));
        bufLen = 507i32;
        let mut buf: String = String::new();
        Arrays::deepToString__arr_obj_sb_set(&a, buf, HashSet::<_>::new()?)?;
        Ok(buf)
    }

    // java: deepToString([Ljava/lang/Object;Ljava/lang/StringBuilder;Ljava/util/Set;)V
    // java: deepToString([Ljava/lang/Object;Ljava/lang/StringBuilder;Ljava/util/Set;)V
    pub fn deepToString__arr_obj_sb_set(a: &[Object], buf: Object, dejaVu: Object) -> Result<()> {
        buf.append(&String::from("null"))?;
        return Ok(());
        let mut iMax: i32 = ((a.len() as i32)).wrapping_sub(1i32);
        buf.append(&String::from("[]"))?;
        return Ok(());
        let _t0 = dejaVu.add(a)?;
        buf.append(&91i32)?;
        let mut i: i32 = 0i32;
        let mut element: Object = a[i as usize].clone();
        buf.append(&String::from("null"))?;
        let _t1 = element.getClass()?;
        let mut eClass: Object = _t1;
        let _t2 = eClass.isArray()?;
        let _t3: String = Arrays::toString__arr_b(element)?;
        buf.append(&_t3)?;
        let _t4: String = Arrays::toString__arr_s(element)?;
        buf.append(&_t4)?;
        let _t5: String = Arrays::toString__arr_i(element)?;
        buf.append(&_t5)?;
        let _t6: String = Arrays::toString__arr_l(element)?;
        buf.append(&_t6)?;
        let _t7: String = Arrays::toString__arr_c(element)?;
        buf.append(&_t7)?;
        let _t8: String = Arrays::toString__arr_f(element)?;
        buf.append(&_t8)?;
        let _t9: String = Arrays::toString__arr_d(element)?;
        buf.append(&_t9)?;
        let _t10: String = Arrays::toString__arr_z(element)?;
        buf.append(&_t10)?;
        let _t11 = dejaVu.contains(element)?;
        buf.append(&String::from("[...]"))?;
        Arrays::deepToString__arr_obj_sb_set(element, buf, dejaVu)?;
        let _t12 = element.toString()?;
        buf.append(&_t12)?;
        buf.append(&String::from(","))?;
        i = i.wrapping_add(1i32);
        buf.append(&93i32)?;
        let _t13 = dejaVu.remove(a)?;
        Ok(())
    }

    // java: setAll([Ljava/lang/Object;Ljava/util/function/IntFunction;)V
    // java: setAll([Ljava/lang/Object;Ljava/util/function/IntFunction;)V
    pub fn setAll__arr_obj_intfun(array: &[Object], generator: Object) -> Result<()> {
        let _t0: Object = Objects::requireNonNull__obj(generator)?;
        let mut i: i32 = 0i32;
        loop {
            if i >= (array.len() as i32) { break; }
            let _t0 = generator.apply(i)?;
            array[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: parallelSetAll([Ljava/lang/Object;Ljava/util/function/IntFunction;)V
    // java: parallelSetAll([Ljava/lang/Object;Ljava/util/function/IntFunction;)V
    pub fn parallelSetAll__arr_obj_intfun(array: &[Object], generator: Object) -> Result<()> {
        let _t0: Object = Objects::requireNonNull__obj(generator)?;
        let _t1: Object = IntStream::range(0i32, (array.len() as i32))?;
        let _t2 = _t1.parallel()?;
        /* TODO: invokedynamic 574 */
        array.forEach(generator)?;
        Ok(())
    }

    // java: setAll([ILjava/util/function/IntUnaryOperator;)V
    // java: setAll([ILjava/util/function/IntUnaryOperator;)V
    pub fn setAll__arr_i_intuna(array: &[i32], generator: Object) -> Result<()> {
        let _t0: Object = Objects::requireNonNull__obj(generator)?;
        let mut i: i32 = 0i32;
        loop {
            if i >= (array.len() as i32) { break; }
            let _t0 = generator.applyAsInt(i)?;
            array[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: parallelSetAll([ILjava/util/function/IntUnaryOperator;)V
    // java: parallelSetAll([ILjava/util/function/IntUnaryOperator;)V
    pub fn parallelSetAll__arr_i_intuna(array: &[i32], generator: Object) -> Result<()> {
        let _t0: Object = Objects::requireNonNull__obj(generator)?;
        let _t1: Object = IntStream::range(0i32, (array.len() as i32))?;
        let _t2 = _t1.parallel()?;
        /* TODO: invokedynamic 588 */
        array.forEach(generator)?;
        Ok(())
    }

    // java: setAll([JLjava/util/function/IntToLongFunction;)V
    // java: setAll([JLjava/util/function/IntToLongFunction;)V
    pub fn setAll__arr_l_inttol(array: &[i64], generator: Object) -> Result<()> {
        let _t0: Object = Objects::requireNonNull__obj(generator)?;
        let mut i: i32 = 0i32;
        loop {
            if i >= (array.len() as i32) { break; }
            let _t0 = generator.applyAsLong(i)?;
            array[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: parallelSetAll([JLjava/util/function/IntToLongFunction;)V
    // java: parallelSetAll([JLjava/util/function/IntToLongFunction;)V
    pub fn parallelSetAll__arr_l_inttol(array: &[i64], generator: Object) -> Result<()> {
        let _t0: Object = Objects::requireNonNull__obj(generator)?;
        let _t1: Object = IntStream::range(0i32, (array.len() as i32))?;
        let _t2 = _t1.parallel()?;
        /* TODO: invokedynamic 597 */
        array.forEach(generator)?;
        Ok(())
    }

    // java: setAll([DLjava/util/function/IntToDoubleFunction;)V
    // java: setAll([DLjava/util/function/IntToDoubleFunction;)V
    pub fn setAll__arr_d_inttod(array: &[f64], generator: Object) -> Result<()> {
        let _t0: Object = Objects::requireNonNull__obj(generator)?;
        let mut i: i32 = 0i32;
        loop {
            if i >= (array.len() as i32) { break; }
            let _t0 = generator.applyAsDouble(i)?;
            array[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: parallelSetAll([DLjava/util/function/IntToDoubleFunction;)V
    // java: parallelSetAll([DLjava/util/function/IntToDoubleFunction;)V
    pub fn parallelSetAll__arr_d_inttod(array: &[f64], generator: Object) -> Result<()> {
        let _t0: Object = Objects::requireNonNull__obj(generator)?;
        let _t1: Object = IntStream::range(0i32, (array.len() as i32))?;
        let _t2 = _t1.parallel()?;
        /* TODO: invokedynamic 606 */
        array.forEach(generator)?;
        Ok(())
    }

    // java: spliterator([Ljava/lang/Object;)Ljava/util/Spliterator;
    // java: spliterator([Ljava/lang/Object;)Ljava/util/Spliterator;
    pub fn spliterator__arr_obj(array: &[Object]) -> Result<Object> {
        let _t0: Object = Spliterators::spliterator__arr_obj_i(&array, 1040i32)?;
        Ok(_t0)
    }

    // java: spliterator([Ljava/lang/Object;II)Ljava/util/Spliterator;
    // java: spliterator([Ljava/lang/Object;II)Ljava/util/Spliterator;
    pub fn spliterator__arr_obj_i_i(array: &[Object], startInclusive: i32, endExclusive: i32) -> Result<Object> {
        let _t0: Object = Spliterators::spliterator__arr_obj_i_i_i(&array, startInclusive, endExclusive, 1040i32)?;
        Ok(_t0)
    }

    // java: spliterator([I)Ljava/util/Spliterator$OfInt;
    // java: spliterator([I)Ljava/util/Spliterator$OfInt;
    pub fn spliterator__arr_i(array: &[i32]) -> Result<Object> {
        let _t0: Object = Spliterators::spliterator__arr_i_i(&array, 1040i32)?;
        Ok(_t0)
    }

    // java: spliterator([III)Ljava/util/Spliterator$OfInt;
    // java: spliterator([III)Ljava/util/Spliterator$OfInt;
    pub fn spliterator__arr_i_i_i(array: &[i32], startInclusive: i32, endExclusive: i32) -> Result<Object> {
        let _t0: Object = Spliterators::spliterator__arr_i_i_i_i(&array, startInclusive, endExclusive, 1040i32)?;
        Ok(_t0)
    }

    // java: spliterator([J)Ljava/util/Spliterator$OfLong;
    // java: spliterator([J)Ljava/util/Spliterator$OfLong;
    pub fn spliterator__arr_l(array: &[i64]) -> Result<Object> {
        let _t0: Object = Spliterators::spliterator__arr_l_i(&array, 1040i32)?;
        Ok(_t0)
    }

    // java: spliterator([JII)Ljava/util/Spliterator$OfLong;
    // java: spliterator([JII)Ljava/util/Spliterator$OfLong;
    pub fn spliterator__arr_l_i_i(array: &[i64], startInclusive: i32, endExclusive: i32) -> Result<Object> {
        let _t0: Object = Spliterators::spliterator__arr_l_i_i_i(&array, startInclusive, endExclusive, 1040i32)?;
        Ok(_t0)
    }

    // java: spliterator([D)Ljava/util/Spliterator$OfDouble;
    // java: spliterator([D)Ljava/util/Spliterator$OfDouble;
    pub fn spliterator__arr_d(array: &[f64]) -> Result<Object> {
        let _t0: Object = Spliterators::spliterator__arr_d_i(&array, 1040i32)?;
        Ok(_t0)
    }

    // java: spliterator([DII)Ljava/util/Spliterator$OfDouble;
    // java: spliterator([DII)Ljava/util/Spliterator$OfDouble;
    pub fn spliterator__arr_d_i_i(array: &[f64], startInclusive: i32, endExclusive: i32) -> Result<Object> {
        let _t0: Object = Spliterators::spliterator__arr_d_i_i_i(&array, startInclusive, endExclusive, 1040i32)?;
        Ok(_t0)
    }

    // java: stream([Ljava/lang/Object;)Ljava/util/stream/Stream;
    // java: stream([Ljava/lang/Object;)Ljava/util/stream/Stream;
    pub fn stream__arr_obj(array: &[Object]) -> Result<Object> {
        let _t0: Object = Arrays::stream__arr_obj_i_i(&array, 0i32, (array.len() as i32))?;
        Ok(_t0)
    }

    // java: stream([Ljava/lang/Object;II)Ljava/util/stream/Stream;
    // java: stream([Ljava/lang/Object;II)Ljava/util/stream/Stream;
    pub fn stream__arr_obj_i_i(array: &[Object], startInclusive: i32, endExclusive: i32) -> Result<Object> {
        let _t0: Object = Arrays::spliterator__arr_obj_i_i(&array, startInclusive, endExclusive)?;
        let _t1: Object = StreamSupport::stream(_t0, 0i32)?;
        Ok(_t1)
    }

    // java: stream([I)Ljava/util/stream/IntStream;
    // java: stream([I)Ljava/util/stream/IntStream;
    pub fn stream__arr_i(array: &[i32]) -> Result<Object> {
        let _t0: Object = Arrays::stream__arr_i_i_i(&array, 0i32, (array.len() as i32))?;
        Ok(_t0)
    }

    // java: stream([III)Ljava/util/stream/IntStream;
    // java: stream([III)Ljava/util/stream/IntStream;
    pub fn stream__arr_i_i_i(array: &[i32], startInclusive: i32, endExclusive: i32) -> Result<Object> {
        let _t0: Object = Arrays::spliterator__arr_i_i_i(&array, startInclusive, endExclusive)?;
        let _t1: Object = StreamSupport::intStream(_t0, 0i32)?;
        Ok(_t1)
    }

    // java: stream([J)Ljava/util/stream/LongStream;
    // java: stream([J)Ljava/util/stream/LongStream;
    pub fn stream__arr_l(array: &[i64]) -> Result<Object> {
        let _t0: Object = Arrays::stream__arr_l_i_i(&array, 0i32, (array.len() as i32))?;
        Ok(_t0)
    }

    // java: stream([JII)Ljava/util/stream/LongStream;
    // java: stream([JII)Ljava/util/stream/LongStream;
    pub fn stream__arr_l_i_i(array: &[i64], startInclusive: i32, endExclusive: i32) -> Result<Object> {
        let _t0: Object = Arrays::spliterator__arr_l_i_i(&array, startInclusive, endExclusive)?;
        let _t1: Object = StreamSupport::longStream(_t0, 0i32)?;
        Ok(_t1)
    }

    // java: stream([D)Ljava/util/stream/DoubleStream;
    // java: stream([D)Ljava/util/stream/DoubleStream;
    pub fn stream__arr_d(array: &[f64]) -> Result<Object> {
        let _t0: Object = Arrays::stream__arr_d_i_i(&array, 0i32, (array.len() as i32))?;
        Ok(_t0)
    }

    // java: stream([DII)Ljava/util/stream/DoubleStream;
    // java: stream([DII)Ljava/util/stream/DoubleStream;
    pub fn stream__arr_d_i_i(array: &[f64], startInclusive: i32, endExclusive: i32) -> Result<Object> {
        let _t0: Object = Arrays::spliterator__arr_d_i_i(&array, startInclusive, endExclusive)?;
        let _t1: Object = StreamSupport::doubleStream(_t0, 0i32)?;
        Ok(_t1)
    }

    // java: compare([Z[Z)I
    // java: compare([Z[Z)I
    pub fn compare__arr_z_arr_z(a: &[bool], b: &[bool]) -> Result<i32> {
        return Ok(0i32);
        return Ok(!a.is_none());
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Boolean::compare(a[i as usize], b[i as usize])?;
        return Ok(_t2);
        Ok(((a.len() as i32)).wrapping_sub((b.len() as i32)))
    }

    // java: compare([ZII[ZII)I
    // java: compare([ZII[ZII)I
    pub fn compare__arr_z_i_i_arr_z_i_i(a: &[bool], aFromIndex: i32, aToIndex: i32, b: &[bool], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Boolean::compare(a[(aFromIndex).wrapping_add(i) as usize], b[(bFromIndex).wrapping_add(i) as usize])?;
        return Ok(_t2);
        Ok((aLength).wrapping_sub(bLength))
    }

    // java: compare([B[B)I
    // java: compare([B[B)I
    pub fn compare__arr_b_arr_b(a: &[i8], b: &[i8]) -> Result<i32> {
        return Ok(0i32);
        return Ok(!a.is_none());
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Byte::compare(a[i as usize], b[i as usize])?;
        return Ok(_t2);
        Ok(((a.len() as i32)).wrapping_sub((b.len() as i32)))
    }

    // java: compare([BII[BII)I
    // java: compare([BII[BII)I
    pub fn compare__arr_b_i_i_arr_b_i_i(a: &[i8], aFromIndex: i32, aToIndex: i32, b: &[i8], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Byte::compare(a[(aFromIndex).wrapping_add(i) as usize], b[(bFromIndex).wrapping_add(i) as usize])?;
        return Ok(_t2);
        Ok((aLength).wrapping_sub(bLength))
    }

    // java: compareUnsigned([B[B)I
    // java: compareUnsigned([B[B)I
    pub fn compareUnsigned__arr_b_arr_b(a: &[i8], b: &[i8]) -> Result<i32> {
        return Ok(0i32);
        return Ok(!a.is_none());
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Byte::compareUnsigned(a[i as usize], b[i as usize])?;
        return Ok(_t2);
        Ok(((a.len() as i32)).wrapping_sub((b.len() as i32)))
    }

    // java: compareUnsigned([BII[BII)I
    // java: compareUnsigned([BII[BII)I
    pub fn compareUnsigned__arr_b_i_i_arr_b_i_i(a: &[i8], aFromIndex: i32, aToIndex: i32, b: &[i8], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Byte::compareUnsigned(a[(aFromIndex).wrapping_add(i) as usize], b[(bFromIndex).wrapping_add(i) as usize])?;
        return Ok(_t2);
        Ok((aLength).wrapping_sub(bLength))
    }

    // java: compare([S[S)I
    // java: compare([S[S)I
    pub fn compare__arr_s_arr_s(a: &[i16], b: &[i16]) -> Result<i32> {
        return Ok(0i32);
        return Ok(!a.is_none());
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Short::compare(a[i as usize], b[i as usize])?;
        return Ok(_t2);
        Ok(((a.len() as i32)).wrapping_sub((b.len() as i32)))
    }

    // java: compare([SII[SII)I
    // java: compare([SII[SII)I
    pub fn compare__arr_s_i_i_arr_s_i_i(a: &[i16], aFromIndex: i32, aToIndex: i32, b: &[i16], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Short::compare(a[(aFromIndex).wrapping_add(i) as usize], b[(bFromIndex).wrapping_add(i) as usize])?;
        return Ok(_t2);
        Ok((aLength).wrapping_sub(bLength))
    }

    // java: compareUnsigned([S[S)I
    // java: compareUnsigned([S[S)I
    pub fn compareUnsigned__arr_s_arr_s(a: &[i16], b: &[i16]) -> Result<i32> {
        return Ok(0i32);
        return Ok(!a.is_none());
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Short::compareUnsigned(a[i as usize], b[i as usize])?;
        return Ok(_t2);
        Ok(((a.len() as i32)).wrapping_sub((b.len() as i32)))
    }

    // java: compareUnsigned([SII[SII)I
    // java: compareUnsigned([SII[SII)I
    pub fn compareUnsigned__arr_s_i_i_arr_s_i_i(a: &[i16], aFromIndex: i32, aToIndex: i32, b: &[i16], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Short::compareUnsigned(a[(aFromIndex).wrapping_add(i) as usize], b[(bFromIndex).wrapping_add(i) as usize])?;
        return Ok(_t2);
        Ok((aLength).wrapping_sub(bLength))
    }

    // java: compare([C[C)I
    // java: compare([C[C)I
    pub fn compare__arr_c_arr_c(a: &[u16], b: &[u16]) -> Result<i32> {
        return Ok(0i32);
        return Ok(!a.is_none());
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Character::compare(a[i as usize], b[i as usize])?;
        return Ok(_t2);
        Ok(((a.len() as i32)).wrapping_sub((b.len() as i32)))
    }

    // java: compare([CII[CII)I
    // java: compare([CII[CII)I
    pub fn compare__arr_c_i_i_arr_c_i_i(a: &[u16], aFromIndex: i32, aToIndex: i32, b: &[u16], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Character::compare(a[(aFromIndex).wrapping_add(i) as usize], b[(bFromIndex).wrapping_add(i) as usize])?;
        return Ok(_t2);
        Ok((aLength).wrapping_sub(bLength))
    }

    // java: compare([I[I)I
    // java: compare([I[I)I
    pub fn compare__arr_i_arr_i(a: &[i32], b: &[i32]) -> Result<i32> {
        return Ok(0i32);
        return Ok(!a.is_none());
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Integer::compare(a[i as usize], b[i as usize])?;
        return Ok(_t2);
        Ok(((a.len() as i32)).wrapping_sub((b.len() as i32)))
    }

    // java: compare([III[III)I
    // java: compare([III[III)I
    pub fn compare__arr_i_i_i_arr_i_i_i(a: &[i32], aFromIndex: i32, aToIndex: i32, b: &[i32], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Integer::compare(a[(aFromIndex).wrapping_add(i) as usize], b[(bFromIndex).wrapping_add(i) as usize])?;
        return Ok(_t2);
        Ok((aLength).wrapping_sub(bLength))
    }

    // java: compareUnsigned([I[I)I
    // java: compareUnsigned([I[I)I
    pub fn compareUnsigned__arr_i_arr_i(a: &[i32], b: &[i32]) -> Result<i32> {
        return Ok(0i32);
        return Ok(!a.is_none());
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Integer::compareUnsigned(a[i as usize], b[i as usize])?;
        return Ok(_t2);
        Ok(((a.len() as i32)).wrapping_sub((b.len() as i32)))
    }

    // java: compareUnsigned([III[III)I
    // java: compareUnsigned([III[III)I
    pub fn compareUnsigned__arr_i_i_i_arr_i_i_i(a: &[i32], aFromIndex: i32, aToIndex: i32, b: &[i32], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Integer::compareUnsigned(a[(aFromIndex).wrapping_add(i) as usize], b[(bFromIndex).wrapping_add(i) as usize])?;
        return Ok(_t2);
        Ok((aLength).wrapping_sub(bLength))
    }

    // java: compare([J[J)I
    // java: compare([J[J)I
    pub fn compare__arr_l_arr_l(a: &[i64], b: &[i64]) -> Result<i32> {
        return Ok(0i32);
        return Ok(!a.is_none());
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Long::compare(a[i as usize], b[i as usize])?;
        return Ok(_t2);
        Ok(((a.len() as i32)).wrapping_sub((b.len() as i32)))
    }

    // java: compare([JII[JII)I
    // java: compare([JII[JII)I
    pub fn compare__arr_l_i_i_arr_l_i_i(a: &[i64], aFromIndex: i32, aToIndex: i32, b: &[i64], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Long::compare(a[(aFromIndex).wrapping_add(i) as usize], b[(bFromIndex).wrapping_add(i) as usize])?;
        return Ok(_t2);
        Ok((aLength).wrapping_sub(bLength))
    }

    // java: compareUnsigned([J[J)I
    // java: compareUnsigned([J[J)I
    pub fn compareUnsigned__arr_l_arr_l(a: &[i64], b: &[i64]) -> Result<i32> {
        return Ok(0i32);
        return Ok(!a.is_none());
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Long::compareUnsigned(a[i as usize], b[i as usize])?;
        return Ok(_t2);
        Ok(((a.len() as i32)).wrapping_sub((b.len() as i32)))
    }

    // java: compareUnsigned([JII[JII)I
    // java: compareUnsigned([JII[JII)I
    pub fn compareUnsigned__arr_l_i_i_arr_l_i_i(a: &[i64], aFromIndex: i32, aToIndex: i32, b: &[i64], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Long::compareUnsigned(a[(aFromIndex).wrapping_add(i) as usize], b[(bFromIndex).wrapping_add(i) as usize])?;
        return Ok(_t2);
        Ok((aLength).wrapping_sub(bLength))
    }

    // java: compare([F[F)I
    // java: compare([F[F)I
    pub fn compare__arr_f_arr_f(a: &[f32], b: &[f32]) -> Result<i32> {
        return Ok(0i32);
        return Ok(!a.is_none());
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Float::compare(a[i as usize], b[i as usize])?;
        return Ok(_t2);
        Ok(((a.len() as i32)).wrapping_sub((b.len() as i32)))
    }

    // java: compare([FII[FII)I
    // java: compare([FII[FII)I
    pub fn compare__arr_f_i_i_arr_f_i_i(a: &[f32], aFromIndex: i32, aToIndex: i32, b: &[f32], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Float::compare(a[(aFromIndex).wrapping_add(i) as usize], b[(bFromIndex).wrapping_add(i) as usize])?;
        return Ok(_t2);
        Ok((aLength).wrapping_sub(bLength))
    }

    // java: compare([D[D)I
    // java: compare([D[D)I
    pub fn compare__arr_d_arr_d(a: &[f64], b: &[f64]) -> Result<i32> {
        return Ok(0i32);
        return Ok(!a.is_none());
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Double::compare(a[i as usize], b[i as usize])?;
        return Ok(_t2);
        Ok(((a.len() as i32)).wrapping_sub((b.len() as i32)))
    }

    // java: compare([DII[DII)I
    // java: compare([DII[DII)I
    pub fn compare__arr_d_i_i_arr_d_i_i(a: &[f64], aFromIndex: i32, aToIndex: i32, b: &[f64], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, _t0)?;
        let mut i: i32 = _t1;
        let _t2: i32 = Double::compare(a[(aFromIndex).wrapping_add(i) as usize], b[(bFromIndex).wrapping_add(i) as usize])?;
        return Ok(_t2);
        Ok((aLength).wrapping_sub(bLength))
    }

    // java: compare([Ljava/lang/Comparable;[Ljava/lang/Comparable;)I
    // java: compare([Ljava/lang/Comparable;[Ljava/lang/Comparable;)I
    pub fn compare__arr_cmp_arr_cmp(a: &[Object], b: &[Object]) -> Result<i32> {
        return Ok(0i32);
        return Ok(!a.is_none());
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let mut length: i32 = _t0;
        let mut i: i32 = 0i32;
        loop {
            if i >= length { break; }
            let mut oa: Object = a[i as usize].clone();
            let mut ob: Object = b[i as usize].clone();
            return Ok(1i32);
            let _t0 = oa.compareTo(ob)?;
            let mut v: i32 = _t0;
            return Ok(v);
            i = i.wrapping_add(1i32);
        }
        Ok(((a.len() as i32)).wrapping_sub((b.len() as i32)))
    }

    // java: compare([Ljava/lang/Comparable;II[Ljava/lang/Comparable;II)I
    // java: compare([Ljava/lang/Comparable;II[Ljava/lang/Comparable;II)I
    pub fn compare__arr_cmp_i_i_arr_cmp_i_i(a: &[Object], aFromIndex: i32, aToIndex: i32, b: &[Object], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let mut length: i32 = _t0;
        let mut i: i32 = 0i32;
        loop {
            if i >= length { break; }
            aFromIndex = aFromIndex.wrapping_add(1i32);
            let mut oa: Object = a[aFromIndex as usize].clone();
            bFromIndex = bFromIndex.wrapping_add(1i32);
            let mut ob: Object = b[bFromIndex as usize].clone();
            return Ok(1i32);
            let _t0 = oa.compareTo(ob)?;
            let mut v: i32 = _t0;
            return Ok(v);
            i = i.wrapping_add(1i32);
        }
        Ok((aLength).wrapping_sub(bLength))
    }

    // java: compare([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)I
    // java: compare([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn compare__arr_obj_arr_obj_compar(a: &[Object], b: &[Object], cmp: Object) -> Result<i32> {
        let _t0: Object = Objects::requireNonNull__obj(cmp)?;
        return Ok(0i32);
        return Ok(!a.is_none());
        let _t1: i32 = ((a.len() as i32)).min((b.len() as i32));
        let mut length: i32 = _t1;
        let mut i: i32 = 0i32;
        loop {
            if i >= length { break; }
            let mut oa: Object = a[i as usize].clone();
            let mut ob: Object = b[i as usize].clone();
            let _t0 = cmp.compare(oa, ob)?;
            let mut v: i32 = _t0;
            return Ok(v);
            i = i.wrapping_add(1i32);
        }
        Ok(((a.len() as i32)).wrapping_sub((b.len() as i32)))
    }

    // java: compare([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)I
    // java: compare([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)I
    pub fn compare__arr_obj_i_i_arr_obj_i_i_compar(a: &[Object], aFromIndex: i32, aToIndex: i32, b: &[Object], bFromIndex: i32, bToIndex: i32, cmp: Object) -> Result<i32> {
        let _t0: Object = Objects::requireNonNull__obj(cmp)?;
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t1: i32 = (aLength).min(bLength);
        let mut length: i32 = _t1;
        let mut i: i32 = 0i32;
        loop {
            if i >= length { break; }
            aFromIndex = aFromIndex.wrapping_add(1i32);
            let mut oa: Object = a[aFromIndex as usize].clone();
            bFromIndex = bFromIndex.wrapping_add(1i32);
            let mut ob: Object = b[bFromIndex as usize].clone();
            let _t0 = cmp.compare(oa, ob)?;
            let mut v: i32 = _t0;
            return Ok(v);
            i = i.wrapping_add(1i32);
        }
        Ok((aLength).wrapping_sub(bLength))
    }

    // java: mismatch([Z[Z)I
    // java: mismatch([Z[Z)I
    pub fn mismatch__arr_z_arr_z(a: &[bool], b: &[bool]) -> Result<i32> {
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let mut length: i32 = _t0;
        return Ok(-1i32);
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, length)?;
        let mut i: i32 = _t1;
        Ok(i)
    }

    // java: mismatch([ZII[ZII)I
    // java: mismatch([ZII[ZII)I
    pub fn mismatch__arr_z_i_i_arr_z_i_i(a: &[bool], aFromIndex: i32, aToIndex: i32, b: &[bool], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let mut length: i32 = _t0;
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, length)?;
        let mut i: i32 = _t1;
        Ok(i)
    }

    // java: mismatch([B[B)I
    // java: mismatch([B[B)I
    pub fn mismatch__arr_b_arr_b(a: &[i8], b: &[i8]) -> Result<i32> {
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let mut length: i32 = _t0;
        return Ok(-1i32);
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, length)?;
        let mut i: i32 = _t1;
        Ok(i)
    }

    // java: mismatch([BII[BII)I
    // java: mismatch([BII[BII)I
    pub fn mismatch__arr_b_i_i_arr_b_i_i(a: &[i8], aFromIndex: i32, aToIndex: i32, b: &[i8], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let mut length: i32 = _t0;
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, length)?;
        let mut i: i32 = _t1;
        Ok(i)
    }

    // java: mismatch([C[C)I
    // java: mismatch([C[C)I
    pub fn mismatch__arr_c_arr_c(a: &[u16], b: &[u16]) -> Result<i32> {
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let mut length: i32 = _t0;
        return Ok(-1i32);
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, length)?;
        let mut i: i32 = _t1;
        Ok(i)
    }

    // java: mismatch([CII[CII)I
    // java: mismatch([CII[CII)I
    pub fn mismatch__arr_c_i_i_arr_c_i_i(a: &[u16], aFromIndex: i32, aToIndex: i32, b: &[u16], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let mut length: i32 = _t0;
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, length)?;
        let mut i: i32 = _t1;
        Ok(i)
    }

    // java: mismatch([S[S)I
    // java: mismatch([S[S)I
    pub fn mismatch__arr_s_arr_s(a: &[i16], b: &[i16]) -> Result<i32> {
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let mut length: i32 = _t0;
        return Ok(-1i32);
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, length)?;
        let mut i: i32 = _t1;
        Ok(i)
    }

    // java: mismatch([SII[SII)I
    // java: mismatch([SII[SII)I
    pub fn mismatch__arr_s_i_i_arr_s_i_i(a: &[i16], aFromIndex: i32, aToIndex: i32, b: &[i16], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let mut length: i32 = _t0;
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, length)?;
        let mut i: i32 = _t1;
        Ok(i)
    }

    // java: mismatch([I[I)I
    // java: mismatch([I[I)I
    pub fn mismatch__arr_i_arr_i(a: &[i32], b: &[i32]) -> Result<i32> {
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let mut length: i32 = _t0;
        return Ok(-1i32);
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, length)?;
        let mut i: i32 = _t1;
        Ok(i)
    }

    // java: mismatch([III[III)I
    // java: mismatch([III[III)I
    pub fn mismatch__arr_i_i_i_arr_i_i_i(a: &[i32], aFromIndex: i32, aToIndex: i32, b: &[i32], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let mut length: i32 = _t0;
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, length)?;
        let mut i: i32 = _t1;
        Ok(i)
    }

    // java: mismatch([J[J)I
    // java: mismatch([J[J)I
    pub fn mismatch__arr_l_arr_l(a: &[i64], b: &[i64]) -> Result<i32> {
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let mut length: i32 = _t0;
        return Ok(-1i32);
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, length)?;
        let mut i: i32 = _t1;
        Ok(i)
    }

    // java: mismatch([JII[JII)I
    // java: mismatch([JII[JII)I
    pub fn mismatch__arr_l_i_i_arr_l_i_i(a: &[i64], aFromIndex: i32, aToIndex: i32, b: &[i64], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let mut length: i32 = _t0;
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, length)?;
        let mut i: i32 = _t1;
        Ok(i)
    }

    // java: mismatch([F[F)I
    // java: mismatch([F[F)I
    pub fn mismatch__arr_f_arr_f(a: &[f32], b: &[f32]) -> Result<i32> {
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let mut length: i32 = _t0;
        return Ok(-1i32);
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, length)?;
        let mut i: i32 = _t1;
        Ok(i)
    }

    // java: mismatch([FII[FII)I
    // java: mismatch([FII[FII)I
    pub fn mismatch__arr_f_i_i_arr_f_i_i(a: &[f32], aFromIndex: i32, aToIndex: i32, b: &[f32], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let mut length: i32 = _t0;
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, length)?;
        let mut i: i32 = _t1;
        Ok(i)
    }

    // java: mismatch([D[D)I
    // java: mismatch([D[D)I
    pub fn mismatch__arr_d_arr_d(a: &[f64], b: &[f64]) -> Result<i32> {
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let mut length: i32 = _t0;
        return Ok(-1i32);
        let _t1: i32 = ArraysSupport::mismatch(&a, &b, length)?;
        let mut i: i32 = _t1;
        Ok(i)
    }

    // java: mismatch([DII[DII)I
    // java: mismatch([DII[DII)I
    pub fn mismatch__arr_d_i_i_arr_d_i_i(a: &[f64], aFromIndex: i32, aToIndex: i32, b: &[f64], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let mut length: i32 = _t0;
        let _t1: i32 = ArraysSupport::mismatch(&a, aFromIndex, &b, bFromIndex, length)?;
        let mut i: i32 = _t1;
        Ok(i)
    }

    // java: mismatch([Ljava/lang/Object;[Ljava/lang/Object;)I
    // java: mismatch([Ljava/lang/Object;[Ljava/lang/Object;)I
    pub fn mismatch__arr_obj_arr_obj(a: &[Object], b: &[Object]) -> Result<i32> {
        let _t0: i32 = ((a.len() as i32)).min((b.len() as i32));
        let mut length: i32 = _t0;
        return Ok(-1i32);
        let mut i: i32 = 0i32;
        loop {
            if i >= length { break; }
            let _t0: bool = Objects::equals(a[i as usize].clone(), b[i as usize].clone())?;
            return Ok(i);
            i = i.wrapping_add(1i32);
        }
        Ok(-1i32)
    }

    // java: mismatch([Ljava/lang/Object;II[Ljava/lang/Object;II)I
    // java: mismatch([Ljava/lang/Object;II[Ljava/lang/Object;II)I
    pub fn mismatch__arr_obj_i_i_arr_obj_i_i(a: &[Object], aFromIndex: i32, aToIndex: i32, b: &[Object], bFromIndex: i32, bToIndex: i32) -> Result<i32> {
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t0: i32 = (aLength).min(bLength);
        let mut length: i32 = _t0;
        let mut i: i32 = 0i32;
        loop {
            if i >= length { break; }
            aFromIndex = aFromIndex.wrapping_add(1i32);
            bFromIndex = bFromIndex.wrapping_add(1i32);
            let _t0: bool = Objects::equals(a[aFromIndex as usize].clone(), b[bFromIndex as usize].clone())?;
            return Ok(i);
            i = i.wrapping_add(1i32);
        }
        Ok(-1i32)
    }

    // java: mismatch([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)I
    // java: mismatch([Ljava/lang/Object;[Ljava/lang/Object;Ljava/util/Comparator;)I
    pub fn mismatch__arr_obj_arr_obj_compar(a: &[Object], b: &[Object], cmp: Object) -> Result<i32> {
        let _t0: Object = Objects::requireNonNull__obj(cmp)?;
        let _t1: i32 = ((a.len() as i32)).min((b.len() as i32));
        let mut length: i32 = _t1;
        return Ok(-1i32);
        let mut i: i32 = 0i32;
        loop {
            if i >= length { break; }
            let mut oa: Object = a[i as usize].clone();
            let mut ob: Object = b[i as usize].clone();
            let _t0 = cmp.compare(oa, ob)?;
            let mut v: i32 = _t0;
            return Ok(i);
            i = i.wrapping_add(1i32);
        }
        Ok(-1i32)
    }

    // java: mismatch([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)I
    // java: mismatch([Ljava/lang/Object;II[Ljava/lang/Object;IILjava/util/Comparator;)I
    pub fn mismatch__arr_obj_i_i_arr_obj_i_i_compar(a: &[Object], aFromIndex: i32, aToIndex: i32, b: &[Object], bFromIndex: i32, bToIndex: i32, cmp: Object) -> Result<i32> {
        let _t0: Object = Objects::requireNonNull__obj(cmp)?;
        Arrays::rangeCheck((a.len() as i32), aFromIndex, aToIndex)?;
        Arrays::rangeCheck((b.len() as i32), bFromIndex, bToIndex)?;
        let mut aLength: i32 = (aToIndex).wrapping_sub(aFromIndex);
        let mut bLength: i32 = (bToIndex).wrapping_sub(bFromIndex);
        let _t1: i32 = (aLength).min(bLength);
        let mut length: i32 = _t1;
        let mut i: i32 = 0i32;
        loop {
            if i >= length { break; }
            aFromIndex = aFromIndex.wrapping_add(1i32);
            let mut oa: Object = a[aFromIndex as usize].clone();
            bFromIndex = bFromIndex.wrapping_add(1i32);
            let mut ob: Object = b[bFromIndex as usize].clone();
            let _t0 = cmp.compare(oa, ob)?;
            let mut v: i32 = _t0;
            return Ok(i);
            i = i.wrapping_add(1i32);
        }
        Ok(-1i32)
    }
}
