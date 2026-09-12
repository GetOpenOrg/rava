#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ArrayPrefixHelpers$LongCumulateTask",
    super_class = "java/util/concurrent/CountedCompleter",
    interfaces  = "",
    access      = "final",
    source      = "ArrayPrefixHelpers.java",
))]
pub struct ArrayPrefixHelpers_LongCumulateTask {
    #[cfg_attr(any(), java_field(name = "array", descriptor = "[J", access = "final"))]
    pub array: Field<Vec<i64>>,
    #[cfg_attr(any(), java_field(name = "function", descriptor = "Ljava/util/function/LongBinaryOperator;", access = "final"))]
    pub function: Field<Object>,
    #[cfg_attr(any(), java_field(name = "left", descriptor = "Ljava/util/ArrayPrefixHelpers$LongCumulateTask;"))]
    pub left: Field<Object>,
    #[cfg_attr(any(), java_field(name = "right", descriptor = "Ljava/util/ArrayPrefixHelpers$LongCumulateTask;"))]
    pub right: Field<Object>,
    #[cfg_attr(any(), java_field(name = "in", descriptor = "J"))]
    pub in_: Field<i64>,
    #[cfg_attr(any(), java_field(name = "out", descriptor = "J"))]
    pub out: Field<i64>,
    #[cfg_attr(any(), java_field(name = "lo", descriptor = "I", access = "final"))]
    pub lo: Field<i32>,
    #[cfg_attr(any(), java_field(name = "hi", descriptor = "I", access = "final"))]
    pub hi: Field<i32>,
    #[cfg_attr(any(), java_field(name = "origin", descriptor = "I", access = "final"))]
    pub origin: Field<i32>,
    #[cfg_attr(any(), java_field(name = "fence", descriptor = "I", access = "final"))]
    pub fence: Field<i32>,
    #[cfg_attr(any(), java_field(name = "threshold", descriptor = "I", access = "final"))]
    pub threshold: Field<i32>,
}

impl ArrayPrefixHelpers_LongCumulateTask {
    // java: <init>(Ljava/util/ArrayPrefixHelpers$LongCumulateTask;Ljava/util/function/LongBinaryOperator;[JII)V
    // java: <init>(Ljava/util/ArrayPrefixHelpers$LongCumulateTask;Ljava/util/function/LongBinaryOperator;[JII)V
    pub fn new__arrayp_longbi_arr_l_i_i(parent: Object, function: Object, array: Vec<i64>, lo: i32, hi: i32) -> Result<Self> {
        let this = Self { array: Field::new(Default::default()), function: Field::new(Default::default()), left: Field::new(Default::default()), right: Field::new(Default::default()), in_: Field::new(0), out: Field::new(0), lo: Field::new(0), hi: Field::new(0), origin: Field::new(0), fence: Field::new(0), threshold: Field::new(0) };
        /* invokespecial Method java/util/concurrent/CountedCompleter.<init>:(Ljava/util/concurrent/CountedCompleter;)V */
        this.function.set(function);
        this.array.set(array);
        this.origin.set(lo);
        this.lo.set(lo);
        this.fence.set(hi);
        this.hi.set(hi);
        let _t0: i32 = ForkJoinPool::getCommonPoolParallelism()?;
        let mut p: i32 = ((hi).wrapping_sub(lo)/(_t0<<(3i32&0x1f)));
        16i32.threshold.set(p);
        Ok(this)
    }

    // java: <init>(Ljava/util/ArrayPrefixHelpers$LongCumulateTask;Ljava/util/function/LongBinaryOperator;[JIIIII)V
    // java: <init>(Ljava/util/ArrayPrefixHelpers$LongCumulateTask;Ljava/util/function/LongBinaryOperator;[JIIIII)V
    pub fn new__arrayp_longbi_arr_l_i_i_i_i_i(parent: Object, function: Object, array: Vec<i64>, origin: i32, fence: i32, threshold: i32, lo: i32, hi: i32) -> Result<Self> {
        let this = Self { array: Field::new(Default::default()), function: Field::new(Default::default()), left: Field::new(Default::default()), right: Field::new(Default::default()), in_: Field::new(0), out: Field::new(0), lo: Field::new(0), hi: Field::new(0), origin: Field::new(0), fence: Field::new(0), threshold: Field::new(0) };
        /* invokespecial Method java/util/concurrent/CountedCompleter.<init>:(Ljava/util/concurrent/CountedCompleter;)V */
        this.function.set(function);
        this.array.set(array);
        this.origin.set(origin);
        this.fence.set(fence);
        this.threshold.set(threshold);
        this.lo.set(lo);
        this.hi.set(hi);
        Ok(this)
    }

    // java: compute()V
    pub fn compute(&self) -> Result<()> {
        let this = self;
        let mut fn_: Object = this.function.get();
        let mut a: Vec<i64> = this.array.get();
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut th: i32 = this.threshold.get();
        let mut org: i32 = this.origin.get();
        let mut fnc: i32 = this.fence.get();
        let mut t: ArrayPrefixHelpers_LongCumulateTask = this;
        loop {
            let mut l: i32 = t.lo.get();
            if t.lo.get()<0i32 { break; }
            let mut h: i32 = t.hi.get();
            let mut lt: Object = t.left.get();
            let mut rt: Object = t.right.get();
            let mut mid: i32 = (((l).wrapping_add(h) as u32>>(1i32&0x1f)) as i32);
            t.right.set(ArrayPrefixHelpers_LongCumulateTask::new(t, fn_, a, org, fnc, th, mid, h)?);
            rt = ArrayPrefixHelpers_LongCumulateTask::new(t, fn_, a, org, fnc, th, mid, h)?;
            let mut f: ArrayPrefixHelpers_LongCumulateTask = ArrayPrefixHelpers_LongCumulateTask::new(t, fn_, a, org, fnc, th, mid, h)?;
            t.left.set(ArrayPrefixHelpers_LongCumulateTask::new(t, fn_, a, org, fnc, th, l, mid)?);
            lt = ArrayPrefixHelpers_LongCumulateTask::new(t, fn_, a, org, fnc, th, l, mid)?;
            t = ArrayPrefixHelpers_LongCumulateTask::new(t, fn_, a, org, fnc, th, l, mid)?;
            mid = t.in_.get();
            lt.in_.set(mid);
            /* TODO: aconst_null  */
            t = lt;
            f = lt;
            let mut lout: i64 = lt.out.get();
            let _t0 = fn_.applyAsLong(mid, lout)?;
            lout.in_.set(_t0);
            let _t1 = rt.getPendingCount()?;
            let mut c: i32 = _t1;
            let _t2 = rt.compareAndSetPendingCount(c, (c|1i32))?;
            t = rt;
            let _t3 = lt.getPendingCount()?;
            lout = _t3;
            let _t4 = lt.compareAndSetPendingCount(lout, (lout|1i32))?;
            f = t;
            t = lt;
            let _t5 = f.fork()?;
        }
        let _t0 = t.getPendingCount()?;
        rt = _t0;
        lt = 6i32;
        let _t1 = t.compareAndSetPendingCount(rt, (rt|lt))?;
        rt = a[org as usize];
        mid = (org).wrapping_add(1i32);
        rt = t.in_.get();
        mid = l;
        let mut i: i32 = mid;
        loop {
            if i >= h { break; }
            let _t0 = fn_.applyAsLong(rt, a[i as usize])?;
            /* TODO: dup2  */
            rt = _t0;
            todo!("stack underflow")[a as usize] = i;
            i = i.wrapping_add(1i32);
        }
        rt = a[l as usize];
        mid = (l).wrapping_add(1i32);
        loop {
            if mid >= h { break; }
            let _t0 = fn_.applyAsLong(rt, a[mid as usize])?;
            rt = _t0;
            mid = mid.wrapping_add(1i32);
        }
        rt = t.in_.get();
        t.out.set(rt);
        loop {
            let _t0 = t.getCompleter()?;
            mid = _t0;
            if (lt&4i32)==0i32 { break; }
            t.quietlyComplete()?;
            let _t0 = mid.getPendingCount()?;
            i = _t0;
            t = mid;
            let mut lt: Object = mid.left.get();
            c = mid.right.get();
            let mut lout: i64 = lt.out.get();
            let _t1 = fn_.applyAsLong(lout, c.out.get())?;
            lout.out.set(_t1);
            lout = 0i32;
            lout = ((i|lt)|lout);
            let _t2 = mid.compareAndSetPendingCount(i, lout)?;
            lt = 2i32;
            t = mid;
            let _t3 = mid.fork()?;
            let _t4 = mid.compareAndSetPendingCount(i, (i|lt))?;
        }
        Ok(())
    }
}
