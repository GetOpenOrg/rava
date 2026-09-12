#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/DualPivotQuicksort",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "final",
    source      = "DualPivotQuicksort.java",
))]
pub struct DualPivotQuicksort;

impl DualPivotQuicksort {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // java: getDepth(II)I
    pub fn getDepth(parallelism: i32, size: i32) -> Result<i32> {
        let mut depth: i32 = 0i32;
        loop {
            parallelism = (parallelism>>((3i32&0x1f)));
            if (parallelism>>((3i32&0x1f)))<=0i32 { break; }
            size = (size>>((2i32&0x1f)));
            depth = depth.wrapping_sub(2i32);
        }
        Ok(depth)
    }

    // java: sort([IIII)V
    // java: sort([IIII)V
    pub fn sort__arr_i_i_i_i(a: &[i32], parallelism: i32, low: i32, high: i32) -> Result<()> {
        let mut size: i32 = (high).wrapping_sub(low);
        let _t0: i32 = DualPivotQuicksort::getDepth(parallelism, (size>>((12i32&0x1f))))?;
        let mut depth: i32 = _t0;
        /* TODO: aconst_null  */
        let mut _arr1: Vec<i32> = vec![0i32; size as usize];
        let mut b: Vec<i32> = _arr1;
        /* TODO: aconst_null  */
        let mut _obj2: DualPivotQuicksort_Sorter = DualPivotQuicksort_Sorter::new(DualPivotQuicksort_Sorter::new(), a, b, low, size, low, depth)?;
        let _t3 = _obj2.invoke()?;
        /* TODO: aconst_null  */
        DualPivotQuicksort::sort__dualpi_arr_i_i_i_i(depth, &a, 0i32, low, high)?;
        Ok(())
    }

    // java: sort(Ljava/util/DualPivotQuicksort$Sorter;[IIII)V
    // java: sort(Ljava/util/DualPivotQuicksort$Sorter;[IIII)V
    pub fn sort__dualpi_arr_i_i_i_i(sorter: Object, a: &[i32], bits: i32, low: i32, high: i32) -> Result<()> {
        let mut end: i32 = (high).wrapping_sub(1i32);
        let mut size: i32 = (high).wrapping_sub(low);
        DualPivotQuicksort::mixedInsertionSort__arr_i_i_i_i(&a, low, (high).wrapping_sub((3i32).wrapping_mul(((size>>((5i32&0x1f)))<<(3i32&0x1f)))), high)?;
        return Ok(());
        DualPivotQuicksort::insertionSort__arr_i_i_i(&a, low, high)?;
        return Ok(());
        let _t0: bool = DualPivotQuicksort::tryMergeRuns__dualpi_arr_i_i_i(sorter, &a, low, size)?;
        return Ok(());
        bits = bits.wrapping_add(6i32);
        DualPivotQuicksort::heapSort__arr_i_i_i(&a, low, high)?;
        return Ok(());
        let mut step: i32 = (((size>>((3i32&0x1f)))).wrapping_mul(3i32)).wrapping_add(3i32);
        let mut e1: i32 = (low).wrapping_add(step);
        let mut e5: i32 = (end).wrapping_sub(step);
        let mut e3: i32 = (((e1).wrapping_add(e5) as u32>>(1i32&0x1f)) as i32);
        let mut e2: i32 = (((e1).wrapping_add(e3) as u32>>(1i32&0x1f)) as i32);
        let mut e4: i32 = (((e3).wrapping_add(e5) as u32>>(1i32&0x1f)) as i32);
        let mut a3: i32 = a[e3 as usize];
        let mut t: i32 = a[e5 as usize];
        a[e5 as usize] = a[e2 as usize];
        a[e2 as usize] = t;
        t = a[e4 as usize];
        a[e4 as usize] = a[e1 as usize];
        a[e1 as usize] = t;
        t = a[e5 as usize];
        a[e5 as usize] = a[e4 as usize];
        a[e4 as usize] = t;
        t = a[e2 as usize];
        a[e2 as usize] = a[e1 as usize];
        a[e1 as usize] = t;
        t = a[e4 as usize];
        a[e4 as usize] = a[e2 as usize];
        a[e2 as usize] = t;
        a[e3 as usize] = a[e2 as usize];
        a[e2 as usize] = a[e1 as usize];
        a[e1 as usize] = a3;
        a[e3 as usize] = a[e2 as usize];
        a[e2 as usize] = a3;
        a[e3 as usize] = a[e4 as usize];
        a[e4 as usize] = a[e5 as usize];
        a[e5 as usize] = a3;
        a[e3 as usize] = a[e4 as usize];
        a[e4 as usize] = a3;
        t = low;
        let mut upper: i32 = end;
        let mut pivot1: i32 = a[e1 as usize];
        let mut pivot2: i32 = a[e5 as usize];
        a[e1 as usize] = a[t as usize];
        a[e5 as usize] = a[upper as usize];
        loop {
            t = t.wrapping_add(1i32);
            if a[t as usize] >= pivot1 { break; }
        }
        loop {
            upper = upper.wrapping_sub(1i32);
            if a[upper as usize] <= pivot2 { break; }
        }
        t = t.wrapping_sub(1i32);
        let mut unused: i32 = t;
        upper = upper.wrapping_add(1i32);
        let mut k: i32 = upper;
        loop {
            k = k.wrapping_sub(1i32);
            if k <= t { break; }
            let mut ak: i32 = a[k as usize];
            t = t.wrapping_add(1i32);
            upper = upper.wrapping_sub(1i32);
            a[k as usize] = a[upper as usize];
            a[upper as usize] = a[t as usize];
            a[k as usize] = a[t as usize];
            a[t as usize] = ak;
            upper = upper.wrapping_sub(1i32);
            a[k as usize] = a[upper as usize];
            a[upper as usize] = ak;
        }
        a[low as usize] = a[t as usize];
        a[t as usize] = pivot1;
        a[end as usize] = a[upper as usize];
        a[upper as usize] = pivot2;
        sorter.forkSorter((bits|1i32), (t).wrapping_add(1i32), upper)?;
        sorter.forkSorter((bits|1i32), (upper).wrapping_add(1i32), high)?;
        DualPivotQuicksort::sort__dualpi_arr_i_i_i_i(sorter, &a, (bits|1i32), (t).wrapping_add(1i32), upper)?;
        DualPivotQuicksort::sort__dualpi_arr_i_i_i_i(sorter, &a, (bits|1i32), (upper).wrapping_add(1i32), high)?;
        pivot1 = a[e3 as usize];
        a[e3 as usize] = a[t as usize];
        upper = upper.wrapping_add(1i32);
        pivot2 = upper;
        loop {
            pivot2 = pivot2.wrapping_sub(1i32);
            if pivot2 <= t { break; }
            unused = a[pivot2 as usize];
            a[pivot2 as usize] = pivot1;
            t = t.wrapping_add(1i32);
            upper = upper.wrapping_sub(1i32);
            a[upper as usize] = a[t as usize];
            a[t as usize] = unused;
            upper = upper.wrapping_sub(1i32);
            a[upper as usize] = unused;
        }
        a[low as usize] = a[t as usize];
        a[t as usize] = pivot1;
        sorter.forkSorter((bits|1i32), upper, high)?;
        DualPivotQuicksort::sort__dualpi_arr_i_i_i_i(sorter, &a, (bits|1i32), upper, high)?;
        high = t;
        Ok(())
    }

    // java: mixedInsertionSort([IIII)V
    // java: mixedInsertionSort([IIII)V
    pub fn mixedInsertionSort__arr_i_i_i_i(a: &[i32], low: i32, end: i32, high: i32) -> Result<()> {
        loop {
            low = low.wrapping_add(1i32);
            if low >= end { break; }
            let mut i: i32 = low;
            let mut ai: i32 = a[low as usize];
            i = i.wrapping_sub(1i32);
            a[(i).wrapping_add(1i32) as usize] = a[i as usize];
            a[(i).wrapping_add(1i32) as usize] = ai;
        }
        i = a[end as usize];
        let mut p: i32 = high;
        loop {
            low = low.wrapping_add(1i32);
            if low >= end { break; }
            ai = low;
            let mut ai: i32 = a[low as usize];
            ai = ai.wrapping_sub(1i32);
            a[ai as usize] = a[ai as usize];
            ai = ai.wrapping_sub(1i32);
            a[(ai).wrapping_add(1i32) as usize] = a[ai as usize];
            a[(ai).wrapping_add(1i32) as usize] = ai;
            p = p.wrapping_sub(1i32);
            ai = a[p as usize];
            a[p as usize] = a[ai as usize];
            ai = ai.wrapping_sub(1i32);
            a[(ai).wrapping_add(1i32) as usize] = a[ai as usize];
            a[(ai).wrapping_add(1i32) as usize] = ai;
        }
        loop {
            if low >= high { break; }
            ai = low;
            p = a[low as usize];
            low = low.wrapping_add(1i32);
            ai = a[low as usize];
            ai = ai.wrapping_sub(1i32);
            a[(ai).wrapping_add(2i32) as usize] = a[ai as usize];
            ai = ai.wrapping_add(1i32);
            a[(ai).wrapping_add(1i32) as usize] = p;
            ai = ai.wrapping_sub(1i32);
            a[(ai).wrapping_add(1i32) as usize] = a[ai as usize];
            a[(ai).wrapping_add(1i32) as usize] = ai;
            ai = ai.wrapping_sub(1i32);
            a[(ai).wrapping_add(2i32) as usize] = a[ai as usize];
            ai = ai.wrapping_add(1i32);
            a[(ai).wrapping_add(1i32) as usize] = ai;
            ai = ai.wrapping_sub(1i32);
            a[(ai).wrapping_add(1i32) as usize] = a[ai as usize];
            a[(ai).wrapping_add(1i32) as usize] = p;
            low = low.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: insertionSort([III)V
    // java: insertionSort([III)V
    pub fn insertionSort__arr_i_i_i(a: &[i32], low: i32, high: i32) -> Result<()> {
        let mut k: i32 = low;
        loop {
            k = k.wrapping_add(1i32);
            if k >= high { break; }
            let mut i: i32 = k;
            let mut ai: i32 = a[k as usize];
            i = i.wrapping_sub(1i32);
            a[(i).wrapping_add(1i32) as usize] = a[i as usize];
            a[(i).wrapping_add(1i32) as usize] = ai;
        }
        Ok(())
    }

    // java: heapSort([III)V
    // java: heapSort([III)V
    pub fn heapSort__arr_i_i_i(a: &[i32], low: i32, high: i32) -> Result<()> {
        let mut k: i32 = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
        loop {
            if k <= low { break; }
            k = k.wrapping_sub(1i32);
            DualPivotQuicksort::pushDown__arr_i_i_i_i_i(&a, k, a[k as usize], low, high)?;
        }
        loop {
            high = high.wrapping_sub(1i32);
            if high <= low { break; }
            k = a[low as usize];
            DualPivotQuicksort::pushDown__arr_i_i_i_i_i(&a, low, a[high as usize], low, high)?;
            a[high as usize] = k;
        }
        Ok(())
    }

    // java: pushDown([IIIII)V
    // java: pushDown([IIIII)V
    pub fn pushDown__arr_i_i_i_i_i(a: &[i32], p: i32, value: i32, low: i32, high: i32) -> Result<()> {
        let mut k: i32 = (((p<<(1i32&0x1f))).wrapping_sub(low)).wrapping_add(2i32);
        k = k.wrapping_sub(1i32);
        p = k;
        a[p as usize] = a[k as usize];
        a[p as usize] = value;
        Ok(())
    }

    // java: tryMergeRuns(Ljava/util/DualPivotQuicksort$Sorter;[III)Z
    // java: tryMergeRuns(Ljava/util/DualPivotQuicksort$Sorter;[III)Z
    pub fn tryMergeRuns__dualpi_arr_i_i_i(sorter: Object, a: &[i32], low: i32, size: i32) -> Result<bool> {
        /* TODO: aconst_null  */
        let mut run: i32 = todo!("stack underflow");
        let mut high: i32 = (low).wrapping_add(size);
        let mut count: i32 = 1i32;
        let mut last: i32 = low;
        let mut k: i32 = (low).wrapping_add(1i32);
        loop {
            if k >= high { break; }
            k = k.wrapping_add(1i32);
            k = k.wrapping_add(1i32);
            let mut i: i32 = (last).wrapping_sub(1i32);
            let mut j: i32 = k;
            i = i.wrapping_add(1i32);
            j = j.wrapping_sub(1i32);
            let mut ai: i32 = a[i as usize];
            a[i as usize] = a[j as usize];
            a[j as usize] = ai;
            i = a[k as usize];
            k = k.wrapping_add(1i32);
            return Ok(1i32);
            return Ok(0i32);
            let mut _arr0: Vec<i32> = vec![0i32; (((size>>((10i32&0x1f)))|127i32)&1023i32) as usize];
            run = _arr0;
            run[0i32 as usize] = low;
            return Ok(0i32);
            count = count.wrapping_add(1i32);
            return Ok(0i32);
            let _t1: Vec<i32> = Arrays::copyOf__arr_i_i(run, (count<<(1i32&0x1f)))?;
            run = _t1;
            last = k;
            run[count as usize] = k;
        }
        i = low;
        k = sorter.b.get();
        let mut _arr0: Vec<i32> = vec![0i32; size as usize];
        k = _arr0;
        i = sorter.offset.get();
        let _t1: Vec<i32> = DualPivotQuicksort::mergeRuns__arr_i_arr_i_i_i_z_arr_i_i_i(&a, k, i, 1i32, !sorter.is_none(), run, 0i32, count)?;
        Ok(1i32)
    }

    // java: mergeRuns([I[IIIZ[III)[I
    // java: mergeRuns([I[IIIZ[III)[I
    pub fn mergeRuns__arr_i_arr_i_i_i_z_arr_i_i_i(a: &[i32], b: &[i32], offset: i32, aim: i32, parallel: bool, run: &[i32], lo: i32, hi: i32) -> Result<Vec<i32>> {
        return Ok(a);
        let mut i: i32 = run[hi as usize];
        let mut j: i32 = (i).wrapping_sub(offset);
        let mut low: i32 = run[lo as usize];
        loop {
            if i <= low { break; }
            j = j.wrapping_sub(1i32);
            i = i.wrapping_sub(1i32);
            b[j as usize] = a[i as usize];
        }
        return Ok(b);
        i = lo;
        j = (((run[lo as usize]).wrapping_add(run[hi as usize]) as u32>>(1i32&0x1f)) as i32);
        loop {
            i = i.wrapping_add(1i32);
            if run[(i).wrapping_add(1i32) as usize] > j { break; }
        }
        let _t0 = DualPivotQuicksort_RunMerger::new(a, b, offset, 0i32, run, i, hi)?.forkMe()?;
        let mut merger: Object = _t0;
        let _t1: Vec<i32> = DualPivotQuicksort::mergeRuns__arr_i_arr_i_i_i_z_arr_i_i_i(&a, &b, offset, (aim).wrapping_neg(), 1i32, &run, lo, i)?;
        low = _t1;
        let _t2 = merger.getDestination()?;
        let mut a2: Object = _t2;
        let _t3: Vec<i32> = DualPivotQuicksort::mergeRuns__arr_i_arr_i_i_i_z_arr_i_i_i(&a, &b, offset, (aim).wrapping_neg(), 0i32, &run, lo, i)?;
        low = _t3;
        let _t4: Vec<i32> = DualPivotQuicksort::mergeRuns__arr_i_arr_i_i_i_z_arr_i_i_i(&a, &b, offset, 0i32, 0i32, &run, i, hi)?;
        a2 = _t4;
        merger = a;
        let mut k: i32 = run[lo as usize];
        let mut lo1: i32 = run[lo as usize];
        let mut hi1: i32 = run[i as usize];
        let mut lo2: i32 = run[i as usize];
        let mut hi2: i32 = run[hi as usize];
        /* TODO: aconst_null  */
        let mut _obj5: DualPivotQuicksort_Merger = DualPivotQuicksort_Merger::new(DualPivotQuicksort_Merger::new(), merger, k, low, lo1, hi1, a2, lo2, hi2)?;
        let _t6 = _obj5.invoke()?;
        /* TODO: aconst_null  */
        DualPivotQuicksort::mergeParts__dualpi_arr_i_i_arr_i_i_i_arr_i_i_i(parallel, merger, k, low, lo1, hi1, a2, lo2, hi2)?;
        Ok(merger)
    }

    // java: mergeParts(Ljava/util/DualPivotQuicksort$Merger;[II[III[III)V
    // java: mergeParts(Ljava/util/DualPivotQuicksort$Merger;[II[III[III)V
    pub fn mergeParts__dualpi_arr_i_i_arr_i_i_i_arr_i_i_i(merger: Object, dst: &[i32], k: i32, a1: &[i32], lo1: i32, hi1: i32, a2: &[i32], lo2: i32, hi2: i32) -> Result<()> {
        let mut lo: i32 = lo1;
        lo1 = lo2;
        lo2 = lo;
        let mut hi: i32 = hi1;
        hi1 = hi2;
        hi2 = hi;
        lo = (((lo1).wrapping_add(hi1) as u32>>(1i32&0x1f)) as i32);
        hi = a1[lo as usize];
        let mut mi2: i32 = hi2;
        let mut loo: i32 = lo2;
        loop {
            if loo >= mi2 { break; }
            let mut t: i32 = (((loo).wrapping_add(mi2) as u32>>(1i32&0x1f)) as i32);
            loo = (t).wrapping_add(1i32);
            mi2 = t;
        }
        loo = (((mi2).wrapping_sub(lo2)).wrapping_add(lo)).wrapping_sub(lo1);
        merger.forkMerger(dst, (k).wrapping_add(loo), a1, lo, hi1, a2, mi2, hi2)?;
        hi1 = lo;
        hi2 = mi2;
        loop {
            if lo1 >= hi1 { break; }
            k = k.wrapping_add(1i32);
            lo1 = lo1.wrapping_add(1i32);
            lo2 = lo2.wrapping_add(1i32);
            a2[lo2 as usize][a1[lo1 as usize] as usize] = a2[lo2 as usize];
        }
        loop {
            if lo1 >= hi1 { break; }
            k = k.wrapping_add(1i32);
            lo1 = lo1.wrapping_add(1i32);
            dst[k as usize] = a1[lo1 as usize];
        }
        loop {
            if lo2 >= hi2 { break; }
            k = k.wrapping_add(1i32);
            lo2 = lo2.wrapping_add(1i32);
            dst[k as usize] = a2[lo2 as usize];
        }
        Ok(())
    }

    // java: sort([JIII)V
    // java: sort([JIII)V
    pub fn sort__arr_l_i_i_i(a: &[i64], parallelism: i32, low: i32, high: i32) -> Result<()> {
        let mut size: i32 = (high).wrapping_sub(low);
        let _t0: i32 = DualPivotQuicksort::getDepth(parallelism, (size>>((12i32&0x1f))))?;
        let mut depth: i32 = _t0;
        /* TODO: aconst_null  */
        let mut _arr1: Vec<i64> = vec![0i64; size as usize];
        let mut b: Vec<i64> = _arr1;
        /* TODO: aconst_null  */
        let mut _obj2: DualPivotQuicksort_Sorter = DualPivotQuicksort_Sorter::new(DualPivotQuicksort_Sorter::new(), a, b, low, size, low, depth)?;
        let _t3 = _obj2.invoke()?;
        /* TODO: aconst_null  */
        DualPivotQuicksort::sort__dualpi_arr_l_i_i_i(depth, &a, 0i32, low, high)?;
        Ok(())
    }

    // java: sort(Ljava/util/DualPivotQuicksort$Sorter;[JIII)V
    // java: sort(Ljava/util/DualPivotQuicksort$Sorter;[JIII)V
    pub fn sort__dualpi_arr_l_i_i_i(sorter: Object, a: &[i64], bits: i32, low: i32, high: i32) -> Result<()> {
        let mut end: i32 = (high).wrapping_sub(1i32);
        let mut size: i32 = (high).wrapping_sub(low);
        DualPivotQuicksort::mixedInsertionSort__arr_l_i_i_i(&a, low, (high).wrapping_sub((3i32).wrapping_mul(((size>>((5i32&0x1f)))<<(3i32&0x1f)))), high)?;
        return Ok(());
        DualPivotQuicksort::insertionSort__arr_l_i_i(&a, low, high)?;
        return Ok(());
        let _t0: bool = DualPivotQuicksort::tryMergeRuns__dualpi_arr_l_i_i(sorter, &a, low, size)?;
        return Ok(());
        bits = bits.wrapping_add(6i32);
        DualPivotQuicksort::heapSort__arr_l_i_i(&a, low, high)?;
        return Ok(());
        let mut step: i32 = (((size>>((3i32&0x1f)))).wrapping_mul(3i32)).wrapping_add(3i32);
        let mut e1: i32 = (low).wrapping_add(step);
        let mut e5: i32 = (end).wrapping_sub(step);
        let mut e3: i32 = (((e1).wrapping_add(e5) as u32>>(1i32&0x1f)) as i32);
        let mut e2: i32 = (((e1).wrapping_add(e3) as u32>>(1i32&0x1f)) as i32);
        let mut e4: i32 = (((e3).wrapping_add(e5) as u32>>(1i32&0x1f)) as i32);
        let mut a3: i64 = a[e3 as usize];
        /* TODO: lcmp  */
        let mut t: i64 = a[e5 as usize];
        a[e5 as usize] = a[e2 as usize];
        a[e2 as usize] = t;
        /* TODO: lcmp  */
        t = a[e4 as usize];
        a[e4 as usize] = a[e1 as usize];
        a[e1 as usize] = t;
        /* TODO: lcmp  */
        t = a[e5 as usize];
        a[e5 as usize] = a[e4 as usize];
        a[e4 as usize] = t;
        /* TODO: lcmp  */
        t = a[e2 as usize];
        a[e2 as usize] = a[e1 as usize];
        a[e1 as usize] = t;
        /* TODO: lcmp  */
        t = a[e4 as usize];
        a[e4 as usize] = a[e2 as usize];
        a[e2 as usize] = t;
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        a[e3 as usize] = a[e2 as usize];
        a[e2 as usize] = a[e1 as usize];
        a[e1 as usize] = a3;
        a[e3 as usize] = a[e2 as usize];
        a[e2 as usize] = a3;
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        a[e3 as usize] = a[e4 as usize];
        a[e4 as usize] = a[e5 as usize];
        a[e5 as usize] = a3;
        a[e3 as usize] = a[e4 as usize];
        a[e4 as usize] = a3;
        t = low;
        let mut upper: i32 = end;
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        let mut pivot1: i64 = a[e1 as usize];
        let mut pivot2: i64 = a[e5 as usize];
        a[e1 as usize] = a[t as usize];
        a[e5 as usize] = a[upper as usize];
        loop {
            t = t.wrapping_add(1i32);
            /* TODO: lcmp  */
            if pivot1>=0i32 { break; }
        }
        loop {
            upper = upper.wrapping_sub(1i32);
            /* TODO: lcmp  */
            if pivot2<=0i32 { break; }
        }
        t = t.wrapping_sub(1i32);
        let mut unused: i32 = t;
        upper = upper.wrapping_add(1i32);
        let mut k: i32 = upper;
        loop {
            k = k.wrapping_sub(1i32);
            if k <= t { break; }
            let mut ak: i64 = a[k as usize];
            /* TODO: lcmp  */
            t = t.wrapping_add(1i32);
            /* TODO: lcmp  */
            /* TODO: lcmp  */
            upper = upper.wrapping_sub(1i32);
            a[k as usize] = a[upper as usize];
            a[upper as usize] = a[t as usize];
            a[k as usize] = a[t as usize];
            a[t as usize] = ak;
            /* TODO: lcmp  */
            upper = upper.wrapping_sub(1i32);
            a[k as usize] = a[upper as usize];
            a[upper as usize] = ak;
        }
        a[low as usize] = a[t as usize];
        a[t as usize] = pivot1;
        a[end as usize] = a[upper as usize];
        a[upper as usize] = pivot2;
        sorter.forkSorter((bits|1i32), (t).wrapping_add(1i32), upper)?;
        sorter.forkSorter((bits|1i32), (upper).wrapping_add(1i32), high)?;
        DualPivotQuicksort::sort__dualpi_arr_l_i_i_i(sorter, &a, (bits|1i32), (t).wrapping_add(1i32), upper)?;
        DualPivotQuicksort::sort__dualpi_arr_l_i_i_i(sorter, &a, (bits|1i32), (upper).wrapping_add(1i32), high)?;
        pivot1 = a[e3 as usize];
        a[e3 as usize] = a[t as usize];
        upper = upper.wrapping_add(1i32);
        pivot2 = upper;
        loop {
            pivot2 = pivot2.wrapping_sub(1i32);
            if pivot2 <= t { break; }
            let mut ak: i64 = a[pivot2 as usize];
            /* TODO: lcmp  */
            a[pivot2 as usize] = pivot1;
            /* TODO: lcmp  */
            t = t.wrapping_add(1i32);
            /* TODO: lcmp  */
            /* TODO: lcmp  */
            upper = upper.wrapping_sub(1i32);
            a[upper as usize] = a[t as usize];
            a[t as usize] = ak;
            upper = upper.wrapping_sub(1i32);
            a[upper as usize] = ak;
        }
        a[low as usize] = a[t as usize];
        a[t as usize] = pivot1;
        sorter.forkSorter((bits|1i32), upper, high)?;
        DualPivotQuicksort::sort__dualpi_arr_l_i_i_i(sorter, &a, (bits|1i32), upper, high)?;
        high = t;
        Ok(())
    }

    // java: mixedInsertionSort([JIII)V
    // java: mixedInsertionSort([JIII)V
    pub fn mixedInsertionSort__arr_l_i_i_i(a: &[i64], low: i32, end: i32, high: i32) -> Result<()> {
        loop {
            low = low.wrapping_add(1i32);
            if low >= end { break; }
            let mut i: i32 = low;
            let mut ai: i64 = a[low as usize];
            i = i.wrapping_sub(1i32);
            /* TODO: lcmp  */
            a[(i).wrapping_add(1i32) as usize] = a[i as usize];
            a[(i).wrapping_add(1i32) as usize] = ai;
        }
        i = a[end as usize];
        let mut p: i32 = high;
        loop {
            low = low.wrapping_add(1i32);
            if low >= end { break; }
            let mut i: i32 = low;
            let mut ai: i64 = a[low as usize];
            /* TODO: lcmp  */
            i = i.wrapping_sub(1i32);
            a[i as usize] = a[i as usize];
            i = i.wrapping_sub(1i32);
            /* TODO: lcmp  */
            a[(i).wrapping_add(1i32) as usize] = a[i as usize];
            a[(i).wrapping_add(1i32) as usize] = ai;
            /* TODO: lcmp  */
            p = p.wrapping_sub(1i32);
            /* TODO: lcmp  */
            ai = a[p as usize];
            a[p as usize] = a[i as usize];
            i = i.wrapping_sub(1i32);
            /* TODO: lcmp  */
            a[(i).wrapping_add(1i32) as usize] = a[i as usize];
            a[(i).wrapping_add(1i32) as usize] = ai;
        }
        loop {
            if low >= high { break; }
            i = low;
            p = a[low as usize];
            low = low.wrapping_add(1i32);
            let mut a2: i64 = a[low as usize];
            /* TODO: lcmp  */
            i = i.wrapping_sub(1i32);
            /* TODO: lcmp  */
            a[(i).wrapping_add(2i32) as usize] = a[i as usize];
            i = i.wrapping_add(1i32);
            a[(i).wrapping_add(1i32) as usize] = p;
            i = i.wrapping_sub(1i32);
            /* TODO: lcmp  */
            a[(i).wrapping_add(1i32) as usize] = a[i as usize];
            a[(i).wrapping_add(1i32) as usize] = a2;
            /* TODO: lcmp  */
            i = i.wrapping_sub(1i32);
            /* TODO: lcmp  */
            a[(i).wrapping_add(2i32) as usize] = a[i as usize];
            i = i.wrapping_add(1i32);
            a[(i).wrapping_add(1i32) as usize] = a2;
            i = i.wrapping_sub(1i32);
            /* TODO: lcmp  */
            a[(i).wrapping_add(1i32) as usize] = a[i as usize];
            a[(i).wrapping_add(1i32) as usize] = p;
            low = low.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: insertionSort([JII)V
    // java: insertionSort([JII)V
    pub fn insertionSort__arr_l_i_i(a: &[i64], low: i32, high: i32) -> Result<()> {
        let mut k: i32 = low;
        loop {
            k = k.wrapping_add(1i32);
            if k >= high { break; }
            let mut i: i32 = k;
            let mut ai: i64 = a[k as usize];
            /* TODO: lcmp  */
            i = i.wrapping_sub(1i32);
            /* TODO: lcmp  */
            a[(i).wrapping_add(1i32) as usize] = a[i as usize];
            a[(i).wrapping_add(1i32) as usize] = ai;
        }
        Ok(())
    }

    // java: heapSort([JII)V
    // java: heapSort([JII)V
    pub fn heapSort__arr_l_i_i(a: &[i64], low: i32, high: i32) -> Result<()> {
        let mut k: i32 = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
        loop {
            if k <= low { break; }
            k = k.wrapping_sub(1i32);
            DualPivotQuicksort::pushDown__arr_l_i_l_i_i(&a, k, a[k as usize], low, high)?;
        }
        loop {
            high = high.wrapping_sub(1i32);
            if high <= low { break; }
            k = a[low as usize];
            DualPivotQuicksort::pushDown__arr_l_i_l_i_i(&a, low, a[high as usize], low, high)?;
            a[high as usize] = k;
        }
        Ok(())
    }

    // java: pushDown([JIJII)V
    // java: pushDown([JIJII)V
    pub fn pushDown__arr_l_i_l_i_i(a: &[i64], p: i32, value: i64, arg_3: i32, low: i32) -> Result<()> {
        let mut k: i32 = (((p<<(1i32&0x1f))).wrapping_sub(low)).wrapping_add(2i32);
        /* TODO: lcmp  */
        k = k.wrapping_sub(1i32);
        /* TODO: lcmp  */
        p = k;
        a[p as usize] = a[k as usize];
        a[p as usize] = value;
        Ok(())
    }

    // java: tryMergeRuns(Ljava/util/DualPivotQuicksort$Sorter;[JII)Z
    // java: tryMergeRuns(Ljava/util/DualPivotQuicksort$Sorter;[JII)Z
    pub fn tryMergeRuns__dualpi_arr_l_i_i(sorter: Object, a: &[i64], low: i32, size: i32) -> Result<bool> {
        /* TODO: aconst_null  */
        let mut run: i32 = todo!("stack underflow");
        let mut high: i32 = (low).wrapping_add(size);
        let mut count: i32 = 1i32;
        let mut last: i32 = low;
        let mut k: i32 = (low).wrapping_add(1i32);
        loop {
            if k >= high { break; }
            /* TODO: lcmp  */
            k = k.wrapping_add(1i32);
            /* TODO: lcmp  */
            /* TODO: lcmp  */
            k = k.wrapping_add(1i32);
            /* TODO: lcmp  */
            let mut i: i32 = (last).wrapping_sub(1i32);
            let mut j: i32 = k;
            i = i.wrapping_add(1i32);
            j = j.wrapping_sub(1i32);
            /* TODO: lcmp  */
            let mut ai: i64 = a[i as usize];
            a[i as usize] = a[j as usize];
            a[j as usize] = ai;
            i = a[k as usize];
            k = k.wrapping_add(1i32);
            /* TODO: lcmp  */
            return Ok(1i32);
            return Ok(0i32);
            let mut _arr0: Vec<i32> = vec![0i32; (((size>>((10i32&0x1f)))|127i32)&1023i32) as usize];
            run = _arr0;
            run[0i32 as usize] = low;
            /* TODO: lcmp  */
            return Ok(0i32);
            count = count.wrapping_add(1i32);
            return Ok(0i32);
            let _t1: Vec<i32> = Arrays::copyOf__arr_i_i(run, (count<<(1i32&0x1f)))?;
            run = _t1;
            last = k;
            run[count as usize] = k;
        }
        i = low;
        k = sorter.b.get();
        let mut _arr0: Vec<i64> = vec![0i64; size as usize];
        k = _arr0;
        i = sorter.offset.get();
        let _t1: Vec<i64> = DualPivotQuicksort::mergeRuns__arr_l_arr_l_i_i_z_arr_i_i_i(&a, k, i, 1i32, !sorter.is_none(), run, 0i32, count)?;
        Ok(1i32)
    }

    // java: mergeRuns([J[JIIZ[III)[J
    // java: mergeRuns([J[JIIZ[III)[J
    pub fn mergeRuns__arr_l_arr_l_i_i_z_arr_i_i_i(a: &[i64], b: &[i64], offset: i32, aim: i32, parallel: bool, run: &[i32], lo: i32, hi: i32) -> Result<Vec<i64>> {
        return Ok(a);
        let mut i: i32 = run[hi as usize];
        let mut j: i32 = (i).wrapping_sub(offset);
        let mut low: i32 = run[lo as usize];
        loop {
            if i <= low { break; }
            j = j.wrapping_sub(1i32);
            i = i.wrapping_sub(1i32);
            b[j as usize] = a[i as usize];
        }
        return Ok(b);
        i = lo;
        j = (((run[lo as usize]).wrapping_add(run[hi as usize]) as u32>>(1i32&0x1f)) as i32);
        loop {
            i = i.wrapping_add(1i32);
            if run[(i).wrapping_add(1i32) as usize] > j { break; }
        }
        let _t0 = DualPivotQuicksort_RunMerger::new(a, b, offset, 0i32, run, i, hi)?.forkMe()?;
        let mut merger: Object = _t0;
        let _t1: Vec<i64> = DualPivotQuicksort::mergeRuns__arr_l_arr_l_i_i_z_arr_i_i_i(&a, &b, offset, (aim).wrapping_neg(), 1i32, &run, lo, i)?;
        low = _t1;
        let _t2 = merger.getDestination()?;
        let mut a2: Object = _t2;
        let _t3: Vec<i64> = DualPivotQuicksort::mergeRuns__arr_l_arr_l_i_i_z_arr_i_i_i(&a, &b, offset, (aim).wrapping_neg(), 0i32, &run, lo, i)?;
        low = _t3;
        let _t4: Vec<i64> = DualPivotQuicksort::mergeRuns__arr_l_arr_l_i_i_z_arr_i_i_i(&a, &b, offset, 0i32, 0i32, &run, i, hi)?;
        a2 = _t4;
        merger = a;
        let mut k: i32 = run[lo as usize];
        let mut lo1: i32 = run[lo as usize];
        let mut hi1: i32 = run[i as usize];
        let mut lo2: i32 = run[i as usize];
        let mut hi2: i32 = run[hi as usize];
        /* TODO: aconst_null  */
        let mut _obj5: DualPivotQuicksort_Merger = DualPivotQuicksort_Merger::new(DualPivotQuicksort_Merger::new(), merger, k, low, lo1, hi1, a2, lo2, hi2)?;
        let _t6 = _obj5.invoke()?;
        /* TODO: aconst_null  */
        DualPivotQuicksort::mergeParts__dualpi_arr_l_i_arr_l_i_i_arr_l_i_i(parallel, merger, k, low, lo1, hi1, a2, lo2, hi2)?;
        Ok(merger)
    }

    // java: mergeParts(Ljava/util/DualPivotQuicksort$Merger;[JI[JII[JII)V
    // java: mergeParts(Ljava/util/DualPivotQuicksort$Merger;[JI[JII[JII)V
    pub fn mergeParts__dualpi_arr_l_i_arr_l_i_i_arr_l_i_i(merger: Object, dst: &[i64], k: i32, a1: &[i64], lo1: i32, hi1: i32, a2: &[i64], lo2: i32, hi2: i32) -> Result<()> {
        let mut lo: i32 = lo1;
        lo1 = lo2;
        lo2 = lo;
        let mut hi: i32 = hi1;
        hi1 = hi2;
        hi2 = hi;
        lo = (((lo1).wrapping_add(hi1) as u32>>(1i32&0x1f)) as i32);
        hi = a1[lo as usize];
        let mut mi2: i32 = hi2;
        let mut loo: i32 = lo2;
        loop {
            if loo >= mi2 { break; }
            let mut t: i32 = (((loo).wrapping_add(mi2) as u32>>(1i32&0x1f)) as i32);
            /* TODO: lcmp  */
            loo = (t).wrapping_add(1i32);
            mi2 = t;
        }
        loo = (((mi2).wrapping_sub(lo2)).wrapping_add(lo)).wrapping_sub(lo1);
        merger.forkMerger(dst, (k).wrapping_add(loo), a1, lo, hi1, a2, mi2, hi2)?;
        hi1 = lo;
        hi2 = mi2;
        loop {
            if lo1 >= hi1 { break; }
            k = k.wrapping_add(1i32);
            /* TODO: lcmp  */
            lo1 = lo1.wrapping_add(1i32);
            lo2 = lo2.wrapping_add(1i32);
            a2[lo2 as usize][a1[lo1 as usize] as usize] = a2[lo2 as usize];
        }
        loop {
            if lo1 >= hi1 { break; }
            k = k.wrapping_add(1i32);
            lo1 = lo1.wrapping_add(1i32);
            dst[k as usize] = a1[lo1 as usize];
        }
        loop {
            if lo2 >= hi2 { break; }
            k = k.wrapping_add(1i32);
            lo2 = lo2.wrapping_add(1i32);
            dst[k as usize] = a2[lo2 as usize];
        }
        Ok(())
    }

    // java: sort([BII)V
    // java: sort([BII)V
    pub fn sort__arr_b_i_i(a: &[i8], low: i32, high: i32) -> Result<()> {
        DualPivotQuicksort::countingSort__arr_b_i_i(&a, low, high)?;
        DualPivotQuicksort::insertionSort__arr_b_i_i(&a, low, high)?;
        Ok(())
    }

    // java: insertionSort([BII)V
    // java: insertionSort([BII)V
    pub fn insertionSort__arr_b_i_i(a: &[i8], low: i32, high: i32) -> Result<()> {
        let mut k: i32 = low;
        loop {
            k = k.wrapping_add(1i32);
            if k >= high { break; }
            let mut i: i32 = k;
            let mut ai: i32 = a[k as usize];
            i = i.wrapping_sub(1i32);
            a[(i).wrapping_add(1i32) as usize] = a[i as usize];
            a[(i).wrapping_add(1i32) as usize] = ai;
        }
        Ok(())
    }

    // java: countingSort([BII)V
    // java: countingSort([BII)V
    pub fn countingSort__arr_b_i_i(a: &[i8], low: i32, high: i32) -> Result<()> {
        let mut _arr0: Vec<i32> = vec![0i32; 256i32 as usize];
        let mut count: Vec<i32> = _arr0;
        let mut i: i32 = high;
        loop {
            if i <= low { break; }
            i = i.wrapping_sub(1i32);
            /* TODO: dup2  */
            todo!("stack underflow")[todo!("stack underflow") as usize] = (count[(a[i as usize]&255i32) as usize]).wrapping_add(1i32);
        }
        i = 384i32;
        loop {
            i = i.wrapping_sub(1i32);
            if i <= 127i32 { break; }
            let mut value: i32 = (i&255i32);
            low = (high).wrapping_sub(count[value as usize]);
            high = high.wrapping_sub(1i32);
            /* TODO: i2b  */
            a[high as usize] = value;
        }
        i = 384i32;
        loop {
            if high <= low { break; }
            i = i.wrapping_sub(1i32);
            value = (i&255i32);
            let mut c: i32 = count[value as usize];
            high = high.wrapping_sub(1i32);
            /* TODO: i2b  */
            a[high as usize] = value;
            c = c.wrapping_sub(1i32);
        }
        Ok(())
    }

    // java: sort([CII)V
    // java: sort([CII)V
    pub fn sort__arr_c_i_i(a: &[u16], low: i32, high: i32) -> Result<()> {
        DualPivotQuicksort::countingSort__arr_c_i_i(&a, low, high)?;
        DualPivotQuicksort::sort__arr_c_i_i_i(&a, 0i32, low, high)?;
        Ok(())
    }

    // java: sort([CIII)V
    // java: sort([CIII)V
    pub fn sort__arr_c_i_i_i(a: &[u16], bits: i32, low: i32, high: i32) -> Result<()> {
        let mut end: i32 = (high).wrapping_sub(1i32);
        let mut size: i32 = (high).wrapping_sub(low);
        DualPivotQuicksort::insertionSort__arr_c_i_i(&a, low, high)?;
        return Ok(());
        bits = bits.wrapping_add(6i32);
        DualPivotQuicksort::countingSort__arr_c_i_i(&a, low, high)?;
        return Ok(());
        let mut step: i32 = (((size>>((3i32&0x1f)))).wrapping_mul(3i32)).wrapping_add(3i32);
        let mut e1: i32 = (low).wrapping_add(step);
        let mut e5: i32 = (end).wrapping_sub(step);
        let mut e3: i32 = (((e1).wrapping_add(e5) as u32>>(1i32&0x1f)) as i32);
        let mut e2: i32 = (((e1).wrapping_add(e3) as u32>>(1i32&0x1f)) as i32);
        let mut e4: i32 = (((e3).wrapping_add(e5) as u32>>(1i32&0x1f)) as i32);
        let mut a3: i32 = a[e3 as usize];
        let mut t: i32 = a[e5 as usize];
        a[e5 as usize] = a[e2 as usize];
        a[e2 as usize] = t;
        t = a[e4 as usize];
        a[e4 as usize] = a[e1 as usize];
        a[e1 as usize] = t;
        t = a[e5 as usize];
        a[e5 as usize] = a[e4 as usize];
        a[e4 as usize] = t;
        t = a[e2 as usize];
        a[e2 as usize] = a[e1 as usize];
        a[e1 as usize] = t;
        t = a[e4 as usize];
        a[e4 as usize] = a[e2 as usize];
        a[e2 as usize] = t;
        a[e3 as usize] = a[e2 as usize];
        a[e2 as usize] = a[e1 as usize];
        a[e1 as usize] = a3;
        a[e3 as usize] = a[e2 as usize];
        a[e2 as usize] = a3;
        a[e3 as usize] = a[e4 as usize];
        a[e4 as usize] = a[e5 as usize];
        a[e5 as usize] = a3;
        a[e3 as usize] = a[e4 as usize];
        a[e4 as usize] = a3;
        t = low;
        let mut upper: i32 = end;
        let mut pivot1: i32 = a[e1 as usize];
        let mut pivot2: i32 = a[e5 as usize];
        a[e1 as usize] = a[t as usize];
        a[e5 as usize] = a[upper as usize];
        loop {
            t = t.wrapping_add(1i32);
            if a[t as usize] >= pivot1 { break; }
        }
        loop {
            upper = upper.wrapping_sub(1i32);
            if a[upper as usize] <= pivot2 { break; }
        }
        t = t.wrapping_sub(1i32);
        let mut unused: i32 = t;
        upper = upper.wrapping_add(1i32);
        let mut k: i32 = upper;
        loop {
            k = k.wrapping_sub(1i32);
            if k <= t { break; }
            let mut ak: i32 = a[k as usize];
            t = t.wrapping_add(1i32);
            upper = upper.wrapping_sub(1i32);
            a[k as usize] = a[upper as usize];
            a[upper as usize] = a[t as usize];
            a[k as usize] = a[t as usize];
            a[t as usize] = ak;
            upper = upper.wrapping_sub(1i32);
            a[k as usize] = a[upper as usize];
            a[upper as usize] = ak;
        }
        a[low as usize] = a[t as usize];
        a[t as usize] = pivot1;
        a[end as usize] = a[upper as usize];
        a[upper as usize] = pivot2;
        DualPivotQuicksort::sort__arr_c_i_i_i(&a, (bits|1i32), (t).wrapping_add(1i32), upper)?;
        DualPivotQuicksort::sort__arr_c_i_i_i(&a, (bits|1i32), (upper).wrapping_add(1i32), high)?;
        pivot1 = a[e3 as usize];
        a[e3 as usize] = a[t as usize];
        upper = upper.wrapping_add(1i32);
        pivot2 = upper;
        loop {
            pivot2 = pivot2.wrapping_sub(1i32);
            if pivot2 <= t { break; }
            unused = a[pivot2 as usize];
            a[pivot2 as usize] = pivot1;
            t = t.wrapping_add(1i32);
            upper = upper.wrapping_sub(1i32);
            a[upper as usize] = a[t as usize];
            a[t as usize] = unused;
            upper = upper.wrapping_sub(1i32);
            a[upper as usize] = unused;
        }
        a[low as usize] = a[t as usize];
        a[t as usize] = pivot1;
        DualPivotQuicksort::sort__arr_c_i_i_i(&a, (bits|1i32), upper, high)?;
        high = t;
        Ok(())
    }

    // java: insertionSort([CII)V
    // java: insertionSort([CII)V
    pub fn insertionSort__arr_c_i_i(a: &[u16], low: i32, high: i32) -> Result<()> {
        let mut k: i32 = low;
        loop {
            k = k.wrapping_add(1i32);
            if k >= high { break; }
            let mut i: i32 = k;
            let mut ai: i32 = a[k as usize];
            i = i.wrapping_sub(1i32);
            a[(i).wrapping_add(1i32) as usize] = a[i as usize];
            a[(i).wrapping_add(1i32) as usize] = ai;
        }
        Ok(())
    }

    // java: countingSort([CII)V
    // java: countingSort([CII)V
    pub fn countingSort__arr_c_i_i(a: &[u16], low: i32, high: i32) -> Result<()> {
        let mut _arr0: Vec<i32> = vec![0i32; 65536i32 as usize];
        let mut count: Vec<i32> = _arr0;
        let mut i: i32 = high;
        loop {
            if i <= low { break; }
            i = i.wrapping_sub(1i32);
            /* TODO: dup2  */
            todo!("stack underflow")[todo!("stack underflow") as usize] = (count[a[i as usize] as usize]).wrapping_add(1i32);
        }
        i = 65536i32;
        i = i.wrapping_sub(1i32);
        low = (high).wrapping_sub(count[i as usize]);
        high = high.wrapping_sub(1i32);
        /* TODO: i2c  */
        a[high as usize] = i;
        i = 65536i32;
        loop {
            if high <= low { break; }
            i = i.wrapping_sub(1i32);
            let mut c: i32 = count[i as usize];
            high = high.wrapping_sub(1i32);
            /* TODO: i2c  */
            a[high as usize] = i;
            c = c.wrapping_sub(1i32);
        }
        Ok(())
    }

    // java: sort([SII)V
    // java: sort([SII)V
    pub fn sort__arr_s_i_i(a: &[i16], low: i32, high: i32) -> Result<()> {
        DualPivotQuicksort::countingSort__arr_s_i_i(&a, low, high)?;
        DualPivotQuicksort::sort__arr_s_i_i_i(&a, 0i32, low, high)?;
        Ok(())
    }

    // java: sort([SIII)V
    // java: sort([SIII)V
    pub fn sort__arr_s_i_i_i(a: &[i16], bits: i32, low: i32, high: i32) -> Result<()> {
        let mut end: i32 = (high).wrapping_sub(1i32);
        let mut size: i32 = (high).wrapping_sub(low);
        DualPivotQuicksort::insertionSort__arr_s_i_i(&a, low, high)?;
        return Ok(());
        bits = bits.wrapping_add(6i32);
        DualPivotQuicksort::countingSort__arr_s_i_i(&a, low, high)?;
        return Ok(());
        let mut step: i32 = (((size>>((3i32&0x1f)))).wrapping_mul(3i32)).wrapping_add(3i32);
        let mut e1: i32 = (low).wrapping_add(step);
        let mut e5: i32 = (end).wrapping_sub(step);
        let mut e3: i32 = (((e1).wrapping_add(e5) as u32>>(1i32&0x1f)) as i32);
        let mut e2: i32 = (((e1).wrapping_add(e3) as u32>>(1i32&0x1f)) as i32);
        let mut e4: i32 = (((e3).wrapping_add(e5) as u32>>(1i32&0x1f)) as i32);
        let mut a3: i32 = a[e3 as usize];
        let mut t: i32 = a[e5 as usize];
        a[e5 as usize] = a[e2 as usize];
        a[e2 as usize] = t;
        t = a[e4 as usize];
        a[e4 as usize] = a[e1 as usize];
        a[e1 as usize] = t;
        t = a[e5 as usize];
        a[e5 as usize] = a[e4 as usize];
        a[e4 as usize] = t;
        t = a[e2 as usize];
        a[e2 as usize] = a[e1 as usize];
        a[e1 as usize] = t;
        t = a[e4 as usize];
        a[e4 as usize] = a[e2 as usize];
        a[e2 as usize] = t;
        a[e3 as usize] = a[e2 as usize];
        a[e2 as usize] = a[e1 as usize];
        a[e1 as usize] = a3;
        a[e3 as usize] = a[e2 as usize];
        a[e2 as usize] = a3;
        a[e3 as usize] = a[e4 as usize];
        a[e4 as usize] = a[e5 as usize];
        a[e5 as usize] = a3;
        a[e3 as usize] = a[e4 as usize];
        a[e4 as usize] = a3;
        t = low;
        let mut upper: i32 = end;
        let mut pivot1: i32 = a[e1 as usize];
        let mut pivot2: i32 = a[e5 as usize];
        a[e1 as usize] = a[t as usize];
        a[e5 as usize] = a[upper as usize];
        loop {
            t = t.wrapping_add(1i32);
            if a[t as usize] >= pivot1 { break; }
        }
        loop {
            upper = upper.wrapping_sub(1i32);
            if a[upper as usize] <= pivot2 { break; }
        }
        t = t.wrapping_sub(1i32);
        let mut unused: i32 = t;
        upper = upper.wrapping_add(1i32);
        let mut k: i32 = upper;
        loop {
            k = k.wrapping_sub(1i32);
            if k <= t { break; }
            let mut ak: i32 = a[k as usize];
            t = t.wrapping_add(1i32);
            upper = upper.wrapping_sub(1i32);
            a[k as usize] = a[upper as usize];
            a[upper as usize] = a[t as usize];
            a[k as usize] = a[t as usize];
            a[t as usize] = ak;
            upper = upper.wrapping_sub(1i32);
            a[k as usize] = a[upper as usize];
            a[upper as usize] = ak;
        }
        a[low as usize] = a[t as usize];
        a[t as usize] = pivot1;
        a[end as usize] = a[upper as usize];
        a[upper as usize] = pivot2;
        DualPivotQuicksort::sort__arr_s_i_i_i(&a, (bits|1i32), (t).wrapping_add(1i32), upper)?;
        DualPivotQuicksort::sort__arr_s_i_i_i(&a, (bits|1i32), (upper).wrapping_add(1i32), high)?;
        pivot1 = a[e3 as usize];
        a[e3 as usize] = a[t as usize];
        upper = upper.wrapping_add(1i32);
        pivot2 = upper;
        loop {
            pivot2 = pivot2.wrapping_sub(1i32);
            if pivot2 <= t { break; }
            unused = a[pivot2 as usize];
            a[pivot2 as usize] = pivot1;
            t = t.wrapping_add(1i32);
            upper = upper.wrapping_sub(1i32);
            a[upper as usize] = a[t as usize];
            a[t as usize] = unused;
            upper = upper.wrapping_sub(1i32);
            a[upper as usize] = unused;
        }
        a[low as usize] = a[t as usize];
        a[t as usize] = pivot1;
        DualPivotQuicksort::sort__arr_s_i_i_i(&a, (bits|1i32), upper, high)?;
        high = t;
        Ok(())
    }

    // java: insertionSort([SII)V
    // java: insertionSort([SII)V
    pub fn insertionSort__arr_s_i_i(a: &[i16], low: i32, high: i32) -> Result<()> {
        let mut k: i32 = low;
        loop {
            k = k.wrapping_add(1i32);
            if k >= high { break; }
            let mut i: i32 = k;
            let mut ai: i32 = a[k as usize];
            i = i.wrapping_sub(1i32);
            a[(i).wrapping_add(1i32) as usize] = a[i as usize];
            a[(i).wrapping_add(1i32) as usize] = ai;
        }
        Ok(())
    }

    // java: countingSort([SII)V
    // java: countingSort([SII)V
    pub fn countingSort__arr_s_i_i(a: &[i16], low: i32, high: i32) -> Result<()> {
        let mut _arr0: Vec<i32> = vec![0i32; 65536i32 as usize];
        let mut count: Vec<i32> = _arr0;
        let mut i: i32 = high;
        loop {
            if i <= low { break; }
            i = i.wrapping_sub(1i32);
            /* TODO: dup2  */
            todo!("stack underflow")[todo!("stack underflow") as usize] = (count[(a[i as usize]&65535i32) as usize]).wrapping_add(1i32);
        }
        i = 98304i32;
        loop {
            i = i.wrapping_sub(1i32);
            if i <= 32767i32 { break; }
            let mut value: i32 = (i&65535i32);
            low = (high).wrapping_sub(count[value as usize]);
            high = high.wrapping_sub(1i32);
            /* TODO: i2s  */
            a[high as usize] = value;
        }
        i = 98304i32;
        loop {
            if high <= low { break; }
            i = i.wrapping_sub(1i32);
            value = (i&65535i32);
            let mut c: i32 = count[value as usize];
            high = high.wrapping_sub(1i32);
            /* TODO: i2s  */
            a[high as usize] = value;
            c = c.wrapping_sub(1i32);
        }
        Ok(())
    }

    // java: sort([FIII)V
    // java: sort([FIII)V
    pub fn sort__arr_f_i_i_i(a: &[f32], parallelism: i32, low: i32, high: i32) -> Result<()> {
        let mut numNegativeZero: i32 = 0i32;
        let mut k: i32 = high;
        loop {
            if k <= low { break; }
            k = k.wrapping_sub(1i32);
            let mut ak: f32 = a[k as usize];
            /* TODO: fcmpl  */
            let _t0: i32 = Float::floatToRawIntBits(ak)?;
            numNegativeZero = numNegativeZero.wrapping_add(1i32);
            a[k as usize] = 0f32;
            /* TODO: fcmpl  */
            high = high.wrapping_sub(1i32);
            a[k as usize] = a[high as usize];
            a[high as usize] = ak;
        }
        k = (high).wrapping_sub(low);
        let _t0: i32 = DualPivotQuicksort::getDepth(parallelism, (k>>((12i32&0x1f))))?;
        ak = _t0;
        /* TODO: aconst_null  */
        let mut _arr1: Vec<f32> = vec![0f32; k as usize];
        let mut b: Vec<f32> = _arr1;
        /* TODO: aconst_null  */
        let mut _obj2: DualPivotQuicksort_Sorter = DualPivotQuicksort_Sorter::new(DualPivotQuicksort_Sorter::new(), a, b, low, k, low, ak)?;
        let _t3 = _obj2.invoke()?;
        /* TODO: aconst_null  */
        DualPivotQuicksort::sort__dualpi_arr_f_i_i_i(ak, &a, 0i32, low, high)?;
        numNegativeZero = numNegativeZero.wrapping_add(1i32);
        return Ok(());
        loop {
            if low > high { break; }
            ak = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
            /* TODO: fcmpg  */
            low = (ak).wrapping_add(1i32);
            high = (ak).wrapping_sub(1i32);
        }
        loop {
            numNegativeZero = numNegativeZero.wrapping_sub(1i32);
            if numNegativeZero<=0i32 { break; }
            high = high.wrapping_add(1i32);
            a[high as usize] = -0.0f32;
        }
        Ok(())
    }

    // java: sort(Ljava/util/DualPivotQuicksort$Sorter;[FIII)V
    // java: sort(Ljava/util/DualPivotQuicksort$Sorter;[FIII)V
    pub fn sort__dualpi_arr_f_i_i_i(sorter: Object, a: &[f32], bits: i32, low: i32, high: i32) -> Result<()> {
        let mut end: i32 = (high).wrapping_sub(1i32);
        let mut size: i32 = (high).wrapping_sub(low);
        DualPivotQuicksort::mixedInsertionSort__arr_f_i_i_i(&a, low, (high).wrapping_sub((3i32).wrapping_mul(((size>>((5i32&0x1f)))<<(3i32&0x1f)))), high)?;
        return Ok(());
        DualPivotQuicksort::insertionSort__arr_f_i_i(&a, low, high)?;
        return Ok(());
        let _t0: bool = DualPivotQuicksort::tryMergeRuns__dualpi_arr_f_i_i(sorter, &a, low, size)?;
        return Ok(());
        bits = bits.wrapping_add(6i32);
        DualPivotQuicksort::heapSort__arr_f_i_i(&a, low, high)?;
        return Ok(());
        let mut step: i32 = (((size>>((3i32&0x1f)))).wrapping_mul(3i32)).wrapping_add(3i32);
        let mut e1: i32 = (low).wrapping_add(step);
        let mut e5: i32 = (end).wrapping_sub(step);
        let mut e3: i32 = (((e1).wrapping_add(e5) as u32>>(1i32&0x1f)) as i32);
        let mut e2: i32 = (((e1).wrapping_add(e3) as u32>>(1i32&0x1f)) as i32);
        let mut e4: i32 = (((e3).wrapping_add(e5) as u32>>(1i32&0x1f)) as i32);
        let mut a3: f32 = a[e3 as usize];
        /* TODO: fcmpg  */
        let mut t: f32 = a[e5 as usize];
        a[e5 as usize] = a[e2 as usize];
        a[e2 as usize] = t;
        /* TODO: fcmpg  */
        t = a[e4 as usize];
        a[e4 as usize] = a[e1 as usize];
        a[e1 as usize] = t;
        /* TODO: fcmpg  */
        t = a[e5 as usize];
        a[e5 as usize] = a[e4 as usize];
        a[e4 as usize] = t;
        /* TODO: fcmpg  */
        t = a[e2 as usize];
        a[e2 as usize] = a[e1 as usize];
        a[e1 as usize] = t;
        /* TODO: fcmpg  */
        t = a[e4 as usize];
        a[e4 as usize] = a[e2 as usize];
        a[e2 as usize] = t;
        /* TODO: fcmpg  */
        /* TODO: fcmpg  */
        a[e3 as usize] = a[e2 as usize];
        a[e2 as usize] = a[e1 as usize];
        a[e1 as usize] = a3;
        a[e3 as usize] = a[e2 as usize];
        a[e2 as usize] = a3;
        /* TODO: fcmpl  */
        /* TODO: fcmpl  */
        a[e3 as usize] = a[e4 as usize];
        a[e4 as usize] = a[e5 as usize];
        a[e5 as usize] = a3;
        a[e3 as usize] = a[e4 as usize];
        a[e4 as usize] = a3;
        t = low;
        let mut upper: i32 = end;
        /* TODO: fcmpg  */
        /* TODO: fcmpg  */
        /* TODO: fcmpg  */
        /* TODO: fcmpg  */
        let mut pivot1: f32 = a[e1 as usize];
        let mut pivot2: f32 = a[e5 as usize];
        a[e1 as usize] = a[t as usize];
        a[e5 as usize] = a[upper as usize];
        loop {
            t = t.wrapping_add(1i32);
            /* TODO: fcmpg  */
            if pivot1>=0i32 { break; }
        }
        loop {
            upper = upper.wrapping_sub(1i32);
            /* TODO: fcmpl  */
            if pivot2<=0i32 { break; }
        }
        t = t.wrapping_sub(1i32);
        let mut unused: i32 = t;
        upper = upper.wrapping_add(1i32);
        let mut k: i32 = upper;
        loop {
            k = k.wrapping_sub(1i32);
            if k <= t { break; }
            let mut ak: f32 = a[k as usize];
            /* TODO: fcmpg  */
            t = t.wrapping_add(1i32);
            /* TODO: fcmpl  */
            /* TODO: fcmpl  */
            upper = upper.wrapping_sub(1i32);
            a[k as usize] = a[upper as usize];
            a[upper as usize] = a[t as usize];
            a[k as usize] = a[t as usize];
            a[t as usize] = ak;
            /* TODO: fcmpl  */
            upper = upper.wrapping_sub(1i32);
            a[k as usize] = a[upper as usize];
            a[upper as usize] = ak;
        }
        a[low as usize] = a[t as usize];
        a[t as usize] = pivot1;
        a[end as usize] = a[upper as usize];
        a[upper as usize] = pivot2;
        sorter.forkSorter((bits|1i32), (t).wrapping_add(1i32), upper)?;
        sorter.forkSorter((bits|1i32), (upper).wrapping_add(1i32), high)?;
        DualPivotQuicksort::sort__dualpi_arr_f_i_i_i(sorter, &a, (bits|1i32), (t).wrapping_add(1i32), upper)?;
        DualPivotQuicksort::sort__dualpi_arr_f_i_i_i(sorter, &a, (bits|1i32), (upper).wrapping_add(1i32), high)?;
        pivot1 = a[e3 as usize];
        a[e3 as usize] = a[t as usize];
        upper = upper.wrapping_add(1i32);
        pivot2 = upper;
        loop {
            pivot2 = pivot2.wrapping_sub(1i32);
            if pivot2 <= t { break; }
            unused = a[pivot2 as usize];
            /* TODO: fcmpl  */
            a[pivot2 as usize] = pivot1;
            /* TODO: fcmpg  */
            t = t.wrapping_add(1i32);
            /* TODO: fcmpg  */
            /* TODO: fcmpl  */
            upper = upper.wrapping_sub(1i32);
            a[upper as usize] = a[t as usize];
            a[t as usize] = unused;
            upper = upper.wrapping_sub(1i32);
            a[upper as usize] = unused;
        }
        a[low as usize] = a[t as usize];
        a[t as usize] = pivot1;
        sorter.forkSorter((bits|1i32), upper, high)?;
        DualPivotQuicksort::sort__dualpi_arr_f_i_i_i(sorter, &a, (bits|1i32), upper, high)?;
        high = t;
        Ok(())
    }

    // java: mixedInsertionSort([FIII)V
    // java: mixedInsertionSort([FIII)V
    pub fn mixedInsertionSort__arr_f_i_i_i(a: &[f32], low: i32, end: i32, high: i32) -> Result<()> {
        loop {
            low = low.wrapping_add(1i32);
            if low >= end { break; }
            let mut i: i32 = low;
            let mut ai: f32 = a[low as usize];
            i = i.wrapping_sub(1i32);
            /* TODO: fcmpg  */
            a[(i).wrapping_add(1i32) as usize] = a[i as usize];
            a[(i).wrapping_add(1i32) as usize] = ai;
        }
        i = a[end as usize];
        let mut p: i32 = high;
        loop {
            low = low.wrapping_add(1i32);
            if low >= end { break; }
            ai = low;
            let mut ai: f32 = a[low as usize];
            /* TODO: fcmpg  */
            ai = ai.wrapping_sub(1i32);
            a[ai as usize] = a[ai as usize];
            ai = ai.wrapping_sub(1i32);
            /* TODO: fcmpg  */
            a[(ai).wrapping_add(1i32) as usize] = a[ai as usize];
            a[(ai).wrapping_add(1i32) as usize] = ai;
            /* TODO: fcmpl  */
            p = p.wrapping_sub(1i32);
            /* TODO: fcmpl  */
            ai = a[p as usize];
            a[p as usize] = a[ai as usize];
            ai = ai.wrapping_sub(1i32);
            /* TODO: fcmpg  */
            a[(ai).wrapping_add(1i32) as usize] = a[ai as usize];
            a[(ai).wrapping_add(1i32) as usize] = ai;
        }
        loop {
            if low >= high { break; }
            ai = low;
            p = a[low as usize];
            low = low.wrapping_add(1i32);
            ai = a[low as usize];
            /* TODO: fcmpl  */
            ai = ai.wrapping_sub(1i32);
            /* TODO: fcmpg  */
            a[(ai).wrapping_add(2i32) as usize] = a[ai as usize];
            ai = ai.wrapping_add(1i32);
            a[(ai).wrapping_add(1i32) as usize] = p;
            ai = ai.wrapping_sub(1i32);
            /* TODO: fcmpg  */
            a[(ai).wrapping_add(1i32) as usize] = a[ai as usize];
            a[(ai).wrapping_add(1i32) as usize] = ai;
            /* TODO: fcmpg  */
            ai = ai.wrapping_sub(1i32);
            /* TODO: fcmpg  */
            a[(ai).wrapping_add(2i32) as usize] = a[ai as usize];
            ai = ai.wrapping_add(1i32);
            a[(ai).wrapping_add(1i32) as usize] = ai;
            ai = ai.wrapping_sub(1i32);
            /* TODO: fcmpg  */
            a[(ai).wrapping_add(1i32) as usize] = a[ai as usize];
            a[(ai).wrapping_add(1i32) as usize] = p;
            low = low.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: insertionSort([FII)V
    // java: insertionSort([FII)V
    pub fn insertionSort__arr_f_i_i(a: &[f32], low: i32, high: i32) -> Result<()> {
        let mut k: i32 = low;
        loop {
            k = k.wrapping_add(1i32);
            if k >= high { break; }
            let mut i: i32 = k;
            let mut ai: f32 = a[k as usize];
            /* TODO: fcmpg  */
            i = i.wrapping_sub(1i32);
            /* TODO: fcmpg  */
            a[(i).wrapping_add(1i32) as usize] = a[i as usize];
            a[(i).wrapping_add(1i32) as usize] = ai;
        }
        Ok(())
    }

    // java: heapSort([FII)V
    // java: heapSort([FII)V
    pub fn heapSort__arr_f_i_i(a: &[f32], low: i32, high: i32) -> Result<()> {
        let mut k: i32 = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
        loop {
            if k <= low { break; }
            k = k.wrapping_sub(1i32);
            DualPivotQuicksort::pushDown__arr_f_i_f_i_i(&a, k, a[k as usize], low, high)?;
        }
        loop {
            high = high.wrapping_sub(1i32);
            if high <= low { break; }
            k = a[low as usize];
            DualPivotQuicksort::pushDown__arr_f_i_f_i_i(&a, low, a[high as usize], low, high)?;
            a[high as usize] = k;
        }
        Ok(())
    }

    // java: pushDown([FIFII)V
    // java: pushDown([FIFII)V
    pub fn pushDown__arr_f_i_f_i_i(a: &[f32], p: i32, value: f32, low: i32, high: i32) -> Result<()> {
        let mut k: i32 = (((p<<(1i32&0x1f))).wrapping_sub(low)).wrapping_add(2i32);
        /* TODO: fcmpg  */
        k = k.wrapping_sub(1i32);
        /* TODO: fcmpg  */
        p = k;
        a[p as usize] = a[k as usize];
        a[p as usize] = value;
        Ok(())
    }

    // java: tryMergeRuns(Ljava/util/DualPivotQuicksort$Sorter;[FII)Z
    // java: tryMergeRuns(Ljava/util/DualPivotQuicksort$Sorter;[FII)Z
    pub fn tryMergeRuns__dualpi_arr_f_i_i(sorter: Object, a: &[f32], low: i32, size: i32) -> Result<bool> {
        /* TODO: aconst_null  */
        let mut run: i32 = todo!("stack underflow");
        let mut high: i32 = (low).wrapping_add(size);
        let mut count: i32 = 1i32;
        let mut last: i32 = low;
        let mut k: i32 = (low).wrapping_add(1i32);
        loop {
            if k >= high { break; }
            /* TODO: fcmpg  */
            k = k.wrapping_add(1i32);
            /* TODO: fcmpg  */
            /* TODO: fcmpl  */
            k = k.wrapping_add(1i32);
            /* TODO: fcmpl  */
            let mut i: i32 = (last).wrapping_sub(1i32);
            let mut j: i32 = k;
            i = i.wrapping_add(1i32);
            j = j.wrapping_sub(1i32);
            /* TODO: fcmpl  */
            let mut ai: f32 = a[i as usize];
            a[i as usize] = a[j as usize];
            a[j as usize] = ai;
            i = a[k as usize];
            k = k.wrapping_add(1i32);
            /* TODO: fcmpl  */
            return Ok(1i32);
            return Ok(0i32);
            let mut _arr0: Vec<i32> = vec![0i32; (((size>>((10i32&0x1f)))|127i32)&1023i32) as usize];
            run = _arr0;
            run[0i32 as usize] = low;
            /* TODO: fcmpl  */
            return Ok(0i32);
            count = count.wrapping_add(1i32);
            return Ok(0i32);
            let _t1: Vec<i32> = Arrays::copyOf__arr_i_i(run, (count<<(1i32&0x1f)))?;
            run = _t1;
            last = k;
            run[count as usize] = k;
        }
        i = low;
        k = sorter.b.get();
        let mut _arr0: Vec<f32> = vec![0f32; size as usize];
        k = _arr0;
        i = sorter.offset.get();
        let _t1: Vec<f32> = DualPivotQuicksort::mergeRuns__arr_f_arr_f_i_i_z_arr_i_i_i(&a, k, i, 1i32, !sorter.is_none(), run, 0i32, count)?;
        Ok(1i32)
    }

    // java: mergeRuns([F[FIIZ[III)[F
    // java: mergeRuns([F[FIIZ[III)[F
    pub fn mergeRuns__arr_f_arr_f_i_i_z_arr_i_i_i(a: &[f32], b: &[f32], offset: i32, aim: i32, parallel: bool, run: &[i32], lo: i32, hi: i32) -> Result<Vec<f32>> {
        return Ok(a);
        let mut i: i32 = run[hi as usize];
        let mut j: i32 = (i).wrapping_sub(offset);
        let mut low: i32 = run[lo as usize];
        loop {
            if i <= low { break; }
            j = j.wrapping_sub(1i32);
            i = i.wrapping_sub(1i32);
            b[j as usize] = a[i as usize];
        }
        return Ok(b);
        i = lo;
        j = (((run[lo as usize]).wrapping_add(run[hi as usize]) as u32>>(1i32&0x1f)) as i32);
        loop {
            i = i.wrapping_add(1i32);
            if run[(i).wrapping_add(1i32) as usize] > j { break; }
        }
        let _t0 = DualPivotQuicksort_RunMerger::new(a, b, offset, 0i32, run, i, hi)?.forkMe()?;
        let mut merger: Object = _t0;
        let _t1: Vec<f32> = DualPivotQuicksort::mergeRuns__arr_f_arr_f_i_i_z_arr_i_i_i(&a, &b, offset, (aim).wrapping_neg(), 1i32, &run, lo, i)?;
        low = _t1;
        let _t2 = merger.getDestination()?;
        let mut a2: Object = _t2;
        let _t3: Vec<f32> = DualPivotQuicksort::mergeRuns__arr_f_arr_f_i_i_z_arr_i_i_i(&a, &b, offset, (aim).wrapping_neg(), 0i32, &run, lo, i)?;
        low = _t3;
        let _t4: Vec<f32> = DualPivotQuicksort::mergeRuns__arr_f_arr_f_i_i_z_arr_i_i_i(&a, &b, offset, 0i32, 0i32, &run, i, hi)?;
        a2 = _t4;
        merger = a;
        let mut k: i32 = run[lo as usize];
        let mut lo1: i32 = run[lo as usize];
        let mut hi1: i32 = run[i as usize];
        let mut lo2: i32 = run[i as usize];
        let mut hi2: i32 = run[hi as usize];
        /* TODO: aconst_null  */
        let mut _obj5: DualPivotQuicksort_Merger = DualPivotQuicksort_Merger::new(DualPivotQuicksort_Merger::new(), merger, k, low, lo1, hi1, a2, lo2, hi2)?;
        let _t6 = _obj5.invoke()?;
        /* TODO: aconst_null  */
        DualPivotQuicksort::mergeParts__dualpi_arr_f_i_arr_f_i_i_arr_f_i_i(parallel, merger, k, low, lo1, hi1, a2, lo2, hi2)?;
        Ok(merger)
    }

    // java: mergeParts(Ljava/util/DualPivotQuicksort$Merger;[FI[FII[FII)V
    // java: mergeParts(Ljava/util/DualPivotQuicksort$Merger;[FI[FII[FII)V
    pub fn mergeParts__dualpi_arr_f_i_arr_f_i_i_arr_f_i_i(merger: Object, dst: &[f32], k: i32, a1: &[f32], lo1: i32, hi1: i32, a2: &[f32], lo2: i32, hi2: i32) -> Result<()> {
        let mut lo: i32 = lo1;
        lo1 = lo2;
        lo2 = lo;
        let mut hi: i32 = hi1;
        hi1 = hi2;
        hi2 = hi;
        lo = (((lo1).wrapping_add(hi1) as u32>>(1i32&0x1f)) as i32);
        hi = a1[lo as usize];
        let mut mi2: i32 = hi2;
        let mut loo: i32 = lo2;
        loop {
            if loo >= mi2 { break; }
            let mut t: i32 = (((loo).wrapping_add(mi2) as u32>>(1i32&0x1f)) as i32);
            /* TODO: fcmpl  */
            loo = (t).wrapping_add(1i32);
            mi2 = t;
        }
        loo = (((mi2).wrapping_sub(lo2)).wrapping_add(lo)).wrapping_sub(lo1);
        merger.forkMerger(dst, (k).wrapping_add(loo), a1, lo, hi1, a2, mi2, hi2)?;
        hi1 = lo;
        hi2 = mi2;
        loop {
            if lo1 >= hi1 { break; }
            k = k.wrapping_add(1i32);
            /* TODO: fcmpg  */
            lo1 = lo1.wrapping_add(1i32);
            lo2 = lo2.wrapping_add(1i32);
            a2[lo2 as usize][a1[lo1 as usize] as usize] = a2[lo2 as usize];
        }
        loop {
            if lo1 >= hi1 { break; }
            k = k.wrapping_add(1i32);
            lo1 = lo1.wrapping_add(1i32);
            dst[k as usize] = a1[lo1 as usize];
        }
        loop {
            if lo2 >= hi2 { break; }
            k = k.wrapping_add(1i32);
            lo2 = lo2.wrapping_add(1i32);
            dst[k as usize] = a2[lo2 as usize];
        }
        Ok(())
    }

    // java: sort([DIII)V
    // java: sort([DIII)V
    pub fn sort__arr_d_i_i_i(a: &[f64], parallelism: i32, low: i32, high: i32) -> Result<()> {
        let mut numNegativeZero: i32 = 0i32;
        let mut k: i32 = high;
        loop {
            if k <= low { break; }
            k = k.wrapping_sub(1i32);
            let mut ak: f64 = a[k as usize];
            /* TODO: dcmpl  */
            let _t0: i64 = Double::doubleToRawLongBits(ak)?;
            /* TODO: lcmp  */
            numNegativeZero = numNegativeZero.wrapping_add(1i32);
            a[k as usize] = 0f64;
            /* TODO: dcmpl  */
            high = high.wrapping_sub(1i32);
            a[k as usize] = a[high as usize];
            a[high as usize] = ak;
        }
        k = (high).wrapping_sub(low);
        let _t0: i32 = DualPivotQuicksort::getDepth(parallelism, (k>>((12i32&0x1f))))?;
        ak = _t0;
        /* TODO: aconst_null  */
        let mut _arr1: Vec<f64> = vec![0f64; k as usize];
        let mut b: Vec<f64> = _arr1;
        /* TODO: aconst_null  */
        let mut _obj2: DualPivotQuicksort_Sorter = DualPivotQuicksort_Sorter::new(DualPivotQuicksort_Sorter::new(), a, b, low, k, low, ak)?;
        let _t3 = _obj2.invoke()?;
        /* TODO: aconst_null  */
        DualPivotQuicksort::sort__dualpi_arr_d_i_i_i(ak, &a, 0i32, low, high)?;
        numNegativeZero = numNegativeZero.wrapping_add(1i32);
        return Ok(());
        loop {
            if low > high { break; }
            ak = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
            /* TODO: dcmpg  */
            low = (ak).wrapping_add(1i32);
            high = (ak).wrapping_sub(1i32);
        }
        loop {
            numNegativeZero = numNegativeZero.wrapping_sub(1i32);
            if numNegativeZero<=0i32 { break; }
            high = high.wrapping_add(1i32);
            a[high as usize] = -0.0f64;
        }
        Ok(())
    }

    // java: sort(Ljava/util/DualPivotQuicksort$Sorter;[DIII)V
    // java: sort(Ljava/util/DualPivotQuicksort$Sorter;[DIII)V
    pub fn sort__dualpi_arr_d_i_i_i(sorter: Object, a: &[f64], bits: i32, low: i32, high: i32) -> Result<()> {
        let mut end: i32 = (high).wrapping_sub(1i32);
        let mut size: i32 = (high).wrapping_sub(low);
        DualPivotQuicksort::mixedInsertionSort__arr_d_i_i_i(&a, low, (high).wrapping_sub((3i32).wrapping_mul(((size>>((5i32&0x1f)))<<(3i32&0x1f)))), high)?;
        return Ok(());
        DualPivotQuicksort::insertionSort__arr_d_i_i(&a, low, high)?;
        return Ok(());
        let _t0: bool = DualPivotQuicksort::tryMergeRuns__dualpi_arr_d_i_i(sorter, &a, low, size)?;
        return Ok(());
        bits = bits.wrapping_add(6i32);
        DualPivotQuicksort::heapSort__arr_d_i_i(&a, low, high)?;
        return Ok(());
        let mut step: i32 = (((size>>((3i32&0x1f)))).wrapping_mul(3i32)).wrapping_add(3i32);
        let mut e1: i32 = (low).wrapping_add(step);
        let mut e5: i32 = (end).wrapping_sub(step);
        let mut e3: i32 = (((e1).wrapping_add(e5) as u32>>(1i32&0x1f)) as i32);
        let mut e2: i32 = (((e1).wrapping_add(e3) as u32>>(1i32&0x1f)) as i32);
        let mut e4: i32 = (((e3).wrapping_add(e5) as u32>>(1i32&0x1f)) as i32);
        let mut a3: f64 = a[e3 as usize];
        /* TODO: dcmpg  */
        let mut t: f64 = a[e5 as usize];
        a[e5 as usize] = a[e2 as usize];
        a[e2 as usize] = t;
        /* TODO: dcmpg  */
        t = a[e4 as usize];
        a[e4 as usize] = a[e1 as usize];
        a[e1 as usize] = t;
        /* TODO: dcmpg  */
        t = a[e5 as usize];
        a[e5 as usize] = a[e4 as usize];
        a[e4 as usize] = t;
        /* TODO: dcmpg  */
        t = a[e2 as usize];
        a[e2 as usize] = a[e1 as usize];
        a[e1 as usize] = t;
        /* TODO: dcmpg  */
        t = a[e4 as usize];
        a[e4 as usize] = a[e2 as usize];
        a[e2 as usize] = t;
        /* TODO: dcmpg  */
        /* TODO: dcmpg  */
        a[e3 as usize] = a[e2 as usize];
        a[e2 as usize] = a[e1 as usize];
        a[e1 as usize] = a3;
        a[e3 as usize] = a[e2 as usize];
        a[e2 as usize] = a3;
        /* TODO: dcmpl  */
        /* TODO: dcmpl  */
        a[e3 as usize] = a[e4 as usize];
        a[e4 as usize] = a[e5 as usize];
        a[e5 as usize] = a3;
        a[e3 as usize] = a[e4 as usize];
        a[e4 as usize] = a3;
        t = low;
        let mut upper: i32 = end;
        /* TODO: dcmpg  */
        /* TODO: dcmpg  */
        /* TODO: dcmpg  */
        /* TODO: dcmpg  */
        let mut pivot1: f64 = a[e1 as usize];
        let mut pivot2: f64 = a[e5 as usize];
        a[e1 as usize] = a[t as usize];
        a[e5 as usize] = a[upper as usize];
        loop {
            t = t.wrapping_add(1i32);
            /* TODO: dcmpg  */
            if pivot1>=0i32 { break; }
        }
        loop {
            upper = upper.wrapping_sub(1i32);
            /* TODO: dcmpl  */
            if pivot2<=0i32 { break; }
        }
        t = t.wrapping_sub(1i32);
        let mut unused: i32 = t;
        upper = upper.wrapping_add(1i32);
        let mut k: i32 = upper;
        loop {
            k = k.wrapping_sub(1i32);
            if k <= t { break; }
            let mut ak: f64 = a[k as usize];
            /* TODO: dcmpg  */
            t = t.wrapping_add(1i32);
            /* TODO: dcmpl  */
            /* TODO: dcmpl  */
            upper = upper.wrapping_sub(1i32);
            a[k as usize] = a[upper as usize];
            a[upper as usize] = a[t as usize];
            a[k as usize] = a[t as usize];
            a[t as usize] = ak;
            /* TODO: dcmpl  */
            upper = upper.wrapping_sub(1i32);
            a[k as usize] = a[upper as usize];
            a[upper as usize] = ak;
        }
        a[low as usize] = a[t as usize];
        a[t as usize] = pivot1;
        a[end as usize] = a[upper as usize];
        a[upper as usize] = pivot2;
        sorter.forkSorter((bits|1i32), (t).wrapping_add(1i32), upper)?;
        sorter.forkSorter((bits|1i32), (upper).wrapping_add(1i32), high)?;
        DualPivotQuicksort::sort__dualpi_arr_d_i_i_i(sorter, &a, (bits|1i32), (t).wrapping_add(1i32), upper)?;
        DualPivotQuicksort::sort__dualpi_arr_d_i_i_i(sorter, &a, (bits|1i32), (upper).wrapping_add(1i32), high)?;
        pivot1 = a[e3 as usize];
        a[e3 as usize] = a[t as usize];
        upper = upper.wrapping_add(1i32);
        pivot2 = upper;
        loop {
            pivot2 = pivot2.wrapping_sub(1i32);
            if pivot2 <= t { break; }
            let mut ak: f64 = a[pivot2 as usize];
            /* TODO: dcmpl  */
            a[pivot2 as usize] = pivot1;
            /* TODO: dcmpg  */
            t = t.wrapping_add(1i32);
            /* TODO: dcmpg  */
            /* TODO: dcmpl  */
            upper = upper.wrapping_sub(1i32);
            a[upper as usize] = a[t as usize];
            a[t as usize] = ak;
            upper = upper.wrapping_sub(1i32);
            a[upper as usize] = ak;
        }
        a[low as usize] = a[t as usize];
        a[t as usize] = pivot1;
        sorter.forkSorter((bits|1i32), upper, high)?;
        DualPivotQuicksort::sort__dualpi_arr_d_i_i_i(sorter, &a, (bits|1i32), upper, high)?;
        high = t;
        Ok(())
    }

    // java: mixedInsertionSort([DIII)V
    // java: mixedInsertionSort([DIII)V
    pub fn mixedInsertionSort__arr_d_i_i_i(a: &[f64], low: i32, end: i32, high: i32) -> Result<()> {
        loop {
            low = low.wrapping_add(1i32);
            if low >= end { break; }
            let mut i: i32 = low;
            let mut ai: f64 = a[low as usize];
            i = i.wrapping_sub(1i32);
            /* TODO: dcmpg  */
            a[(i).wrapping_add(1i32) as usize] = a[i as usize];
            a[(i).wrapping_add(1i32) as usize] = ai;
        }
        i = a[end as usize];
        let mut p: i32 = high;
        loop {
            low = low.wrapping_add(1i32);
            if low >= end { break; }
            let mut i: i32 = low;
            let mut ai: f64 = a[low as usize];
            /* TODO: dcmpg  */
            i = i.wrapping_sub(1i32);
            a[i as usize] = a[i as usize];
            i = i.wrapping_sub(1i32);
            /* TODO: dcmpg  */
            a[(i).wrapping_add(1i32) as usize] = a[i as usize];
            a[(i).wrapping_add(1i32) as usize] = ai;
            /* TODO: dcmpl  */
            p = p.wrapping_sub(1i32);
            /* TODO: dcmpl  */
            ai = a[p as usize];
            a[p as usize] = a[i as usize];
            i = i.wrapping_sub(1i32);
            /* TODO: dcmpg  */
            a[(i).wrapping_add(1i32) as usize] = a[i as usize];
            a[(i).wrapping_add(1i32) as usize] = ai;
        }
        loop {
            if low >= high { break; }
            i = low;
            p = a[low as usize];
            low = low.wrapping_add(1i32);
            let mut a2: f64 = a[low as usize];
            /* TODO: dcmpl  */
            i = i.wrapping_sub(1i32);
            /* TODO: dcmpg  */
            a[(i).wrapping_add(2i32) as usize] = a[i as usize];
            i = i.wrapping_add(1i32);
            a[(i).wrapping_add(1i32) as usize] = p;
            i = i.wrapping_sub(1i32);
            /* TODO: dcmpg  */
            a[(i).wrapping_add(1i32) as usize] = a[i as usize];
            a[(i).wrapping_add(1i32) as usize] = a2;
            /* TODO: dcmpg  */
            i = i.wrapping_sub(1i32);
            /* TODO: dcmpg  */
            a[(i).wrapping_add(2i32) as usize] = a[i as usize];
            i = i.wrapping_add(1i32);
            a[(i).wrapping_add(1i32) as usize] = a2;
            i = i.wrapping_sub(1i32);
            /* TODO: dcmpg  */
            a[(i).wrapping_add(1i32) as usize] = a[i as usize];
            a[(i).wrapping_add(1i32) as usize] = p;
            low = low.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: insertionSort([DII)V
    // java: insertionSort([DII)V
    pub fn insertionSort__arr_d_i_i(a: &[f64], low: i32, high: i32) -> Result<()> {
        let mut k: i32 = low;
        loop {
            k = k.wrapping_add(1i32);
            if k >= high { break; }
            let mut i: i32 = k;
            let mut ai: f64 = a[k as usize];
            /* TODO: dcmpg  */
            i = i.wrapping_sub(1i32);
            /* TODO: dcmpg  */
            a[(i).wrapping_add(1i32) as usize] = a[i as usize];
            a[(i).wrapping_add(1i32) as usize] = ai;
        }
        Ok(())
    }

    // java: heapSort([DII)V
    // java: heapSort([DII)V
    pub fn heapSort__arr_d_i_i(a: &[f64], low: i32, high: i32) -> Result<()> {
        let mut k: i32 = (((low).wrapping_add(high) as u32>>(1i32&0x1f)) as i32);
        loop {
            if k <= low { break; }
            k = k.wrapping_sub(1i32);
            DualPivotQuicksort::pushDown__arr_d_i_d_i_i(&a, k, a[k as usize], low, high)?;
        }
        loop {
            high = high.wrapping_sub(1i32);
            if high <= low { break; }
            k = a[low as usize];
            DualPivotQuicksort::pushDown__arr_d_i_d_i_i(&a, low, a[high as usize], low, high)?;
            a[high as usize] = k;
        }
        Ok(())
    }

    // java: pushDown([DIDII)V
    // java: pushDown([DIDII)V
    pub fn pushDown__arr_d_i_d_i_i(a: &[f64], p: i32, value: f64, arg_3: i32, low: i32) -> Result<()> {
        let mut k: i32 = (((p<<(1i32&0x1f))).wrapping_sub(low)).wrapping_add(2i32);
        /* TODO: dcmpg  */
        k = k.wrapping_sub(1i32);
        /* TODO: dcmpg  */
        p = k;
        a[p as usize] = a[k as usize];
        a[p as usize] = value;
        Ok(())
    }

    // java: tryMergeRuns(Ljava/util/DualPivotQuicksort$Sorter;[DII)Z
    // java: tryMergeRuns(Ljava/util/DualPivotQuicksort$Sorter;[DII)Z
    pub fn tryMergeRuns__dualpi_arr_d_i_i(sorter: Object, a: &[f64], low: i32, size: i32) -> Result<bool> {
        /* TODO: aconst_null  */
        let mut run: i32 = todo!("stack underflow");
        let mut high: i32 = (low).wrapping_add(size);
        let mut count: i32 = 1i32;
        let mut last: i32 = low;
        let mut k: i32 = (low).wrapping_add(1i32);
        loop {
            if k >= high { break; }
            /* TODO: dcmpg  */
            k = k.wrapping_add(1i32);
            /* TODO: dcmpg  */
            /* TODO: dcmpl  */
            k = k.wrapping_add(1i32);
            /* TODO: dcmpl  */
            let mut i: i32 = (last).wrapping_sub(1i32);
            let mut j: i32 = k;
            i = i.wrapping_add(1i32);
            j = j.wrapping_sub(1i32);
            /* TODO: dcmpl  */
            let mut ai: f64 = a[i as usize];
            a[i as usize] = a[j as usize];
            a[j as usize] = ai;
            i = a[k as usize];
            k = k.wrapping_add(1i32);
            /* TODO: dcmpl  */
            return Ok(1i32);
            return Ok(0i32);
            let mut _arr0: Vec<i32> = vec![0i32; (((size>>((10i32&0x1f)))|127i32)&1023i32) as usize];
            run = _arr0;
            run[0i32 as usize] = low;
            /* TODO: dcmpl  */
            return Ok(0i32);
            count = count.wrapping_add(1i32);
            return Ok(0i32);
            let _t1: Vec<i32> = Arrays::copyOf__arr_i_i(run, (count<<(1i32&0x1f)))?;
            run = _t1;
            last = k;
            run[count as usize] = k;
        }
        i = low;
        k = sorter.b.get();
        let mut _arr0: Vec<f64> = vec![0f64; size as usize];
        k = _arr0;
        i = sorter.offset.get();
        let _t1: Vec<f64> = DualPivotQuicksort::mergeRuns__arr_d_arr_d_i_i_z_arr_i_i_i(&a, k, i, 1i32, !sorter.is_none(), run, 0i32, count)?;
        Ok(1i32)
    }

    // java: mergeRuns([D[DIIZ[III)[D
    // java: mergeRuns([D[DIIZ[III)[D
    pub fn mergeRuns__arr_d_arr_d_i_i_z_arr_i_i_i(a: &[f64], b: &[f64], offset: i32, aim: i32, parallel: bool, run: &[i32], lo: i32, hi: i32) -> Result<Vec<f64>> {
        return Ok(a);
        let mut i: i32 = run[hi as usize];
        let mut j: i32 = (i).wrapping_sub(offset);
        let mut low: i32 = run[lo as usize];
        loop {
            if i <= low { break; }
            j = j.wrapping_sub(1i32);
            i = i.wrapping_sub(1i32);
            b[j as usize] = a[i as usize];
        }
        return Ok(b);
        i = lo;
        j = (((run[lo as usize]).wrapping_add(run[hi as usize]) as u32>>(1i32&0x1f)) as i32);
        loop {
            i = i.wrapping_add(1i32);
            if run[(i).wrapping_add(1i32) as usize] > j { break; }
        }
        let _t0 = DualPivotQuicksort_RunMerger::new(a, b, offset, 0i32, run, i, hi)?.forkMe()?;
        let mut merger: Object = _t0;
        let _t1: Vec<f64> = DualPivotQuicksort::mergeRuns__arr_d_arr_d_i_i_z_arr_i_i_i(&a, &b, offset, (aim).wrapping_neg(), 1i32, &run, lo, i)?;
        low = _t1;
        let _t2 = merger.getDestination()?;
        let mut a2: Object = _t2;
        let _t3: Vec<f64> = DualPivotQuicksort::mergeRuns__arr_d_arr_d_i_i_z_arr_i_i_i(&a, &b, offset, (aim).wrapping_neg(), 0i32, &run, lo, i)?;
        low = _t3;
        let _t4: Vec<f64> = DualPivotQuicksort::mergeRuns__arr_d_arr_d_i_i_z_arr_i_i_i(&a, &b, offset, 0i32, 0i32, &run, i, hi)?;
        a2 = _t4;
        merger = a;
        let mut k: i32 = run[lo as usize];
        let mut lo1: i32 = run[lo as usize];
        let mut hi1: i32 = run[i as usize];
        let mut lo2: i32 = run[i as usize];
        let mut hi2: i32 = run[hi as usize];
        /* TODO: aconst_null  */
        let mut _obj5: DualPivotQuicksort_Merger = DualPivotQuicksort_Merger::new(DualPivotQuicksort_Merger::new(), merger, k, low, lo1, hi1, a2, lo2, hi2)?;
        let _t6 = _obj5.invoke()?;
        /* TODO: aconst_null  */
        DualPivotQuicksort::mergeParts__dualpi_arr_d_i_arr_d_i_i_arr_d_i_i(parallel, merger, k, low, lo1, hi1, a2, lo2, hi2)?;
        Ok(merger)
    }

    // java: mergeParts(Ljava/util/DualPivotQuicksort$Merger;[DI[DII[DII)V
    // java: mergeParts(Ljava/util/DualPivotQuicksort$Merger;[DI[DII[DII)V
    pub fn mergeParts__dualpi_arr_d_i_arr_d_i_i_arr_d_i_i(merger: Object, dst: &[f64], k: i32, a1: &[f64], lo1: i32, hi1: i32, a2: &[f64], lo2: i32, hi2: i32) -> Result<()> {
        let mut lo: i32 = lo1;
        lo1 = lo2;
        lo2 = lo;
        let mut hi: i32 = hi1;
        hi1 = hi2;
        hi2 = hi;
        lo = (((lo1).wrapping_add(hi1) as u32>>(1i32&0x1f)) as i32);
        hi = a1[lo as usize];
        let mut mi2: i32 = hi2;
        let mut loo: i32 = lo2;
        loop {
            if loo >= mi2 { break; }
            let mut t: i32 = (((loo).wrapping_add(mi2) as u32>>(1i32&0x1f)) as i32);
            /* TODO: dcmpl  */
            loo = (t).wrapping_add(1i32);
            mi2 = t;
        }
        loo = (((mi2).wrapping_sub(lo2)).wrapping_add(lo)).wrapping_sub(lo1);
        merger.forkMerger(dst, (k).wrapping_add(loo), a1, lo, hi1, a2, mi2, hi2)?;
        hi1 = lo;
        hi2 = mi2;
        loop {
            if lo1 >= hi1 { break; }
            k = k.wrapping_add(1i32);
            /* TODO: dcmpg  */
            lo1 = lo1.wrapping_add(1i32);
            lo2 = lo2.wrapping_add(1i32);
            a2[lo2 as usize][a1[lo1 as usize] as usize] = a2[lo2 as usize];
        }
        loop {
            if lo1 >= hi1 { break; }
            k = k.wrapping_add(1i32);
            lo1 = lo1.wrapping_add(1i32);
            dst[k as usize] = a1[lo1 as usize];
        }
        loop {
            if lo2 >= hi2 { break; }
            k = k.wrapping_add(1i32);
            lo2 = lo2.wrapping_add(1i32);
            dst[k as usize] = a2[lo2 as usize];
        }
        Ok(())
    }
}
