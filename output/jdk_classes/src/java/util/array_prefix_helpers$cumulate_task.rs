#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ArrayPrefixHelpers$CumulateTask",
    super_class = "java/util/concurrent/CountedCompleter",
    interfaces  = "",
    access      = "final",
    source      = "ArrayPrefixHelpers.java",
))]
pub struct ArrayPrefixHelpers_CumulateTask<T> {
    #[cfg_attr(any(), java_field(name = "array", descriptor = "[Ljava/lang/Object;", access = "final"))]
    pub array: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "function", descriptor = "Ljava/util/function/BinaryOperator;", access = "final"))]
    pub function: Field<Object>,
    #[cfg_attr(any(), java_field(name = "left", descriptor = "Ljava/util/ArrayPrefixHelpers$CumulateTask;"))]
    pub left: Field<Object>,
    #[cfg_attr(any(), java_field(name = "right", descriptor = "Ljava/util/ArrayPrefixHelpers$CumulateTask;"))]
    pub right: Field<Object>,
    #[cfg_attr(any(), java_field(name = "in", descriptor = "Ljava/lang/Object;"))]
    pub in: Field<Object>,
    #[cfg_attr(any(), java_field(name = "out", descriptor = "Ljava/lang/Object;"))]
    pub out: Field<Object>,
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

impl<T: Clone + 'static> ArrayPrefixHelpers_CumulateTask<T> {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/ArrayPrefixHelpers$CumulateTask;Ljava/util/function/BinaryOperator;[Ljava/lang/Object;II)V", access = "public"))]
    // java: <init>(Ljava/util/ArrayPrefixHelpers$CumulateTask;Ljava/util/function/BinaryOperator;[Ljava/lang/Object;II)V
    pub fn new__arrayp_binary_arr_obj_i_i(parent: Object, function: Object, array: Vec<Object>, lo: i32, hi: i32) -> Result<Self> {
        let this = Self { array: Field::new(Default::default()), function: Field::new(Default::default()), left: Field::new(Default::default()), right: Field::new(Default::default()), in: Field::new(Default::default()), out: Field::new(Default::default()), lo: Field::new(0), hi: Field::new(0), origin: Field::new(0), fence: Field::new(0), threshold: Field::new(0) };
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

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/ArrayPrefixHelpers$CumulateTask;Ljava/util/function/BinaryOperator;[Ljava/lang/Object;IIIII)V"))]
    // java: <init>(Ljava/util/ArrayPrefixHelpers$CumulateTask;Ljava/util/function/BinaryOperator;[Ljava/lang/Object;IIIII)V
    pub fn new__arrayp_binary_arr_obj_i_i_i_i_i(parent: Object, function: Object, array: Vec<Object>, origin: i32, fence: i32, threshold: i32, lo: i32, hi: i32) -> Result<Self> {
        let this = Self { array: Field::new(Default::default()), function: Field::new(Default::default()), left: Field::new(Default::default()), right: Field::new(Default::default()), in: Field::new(Default::default()), out: Field::new(Default::default()), lo: Field::new(0), hi: Field::new(0), origin: Field::new(0), fence: Field::new(0), threshold: Field::new(0) };
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

    #[cfg_attr(any(), java_method(name = "compute", descriptor = "()V", access = "public final"))]
    pub fn compute(&self) -> Result<()> {
        let this = self;
        let mut fn_: Object = this.function.get();
        let mut a: Vec<Object> = this.array.get();
        return Err(JvmError::Custom(String::from("athrow")));
        let mut th: i32 = this.threshold.get();
        let mut org: i32 = this.origin.get();
        let mut fnc: i32 = this.fence.get();
        let mut t: java/util/ArrayPrefixHelpers$CumulateTask = this;
        loop {
            let mut l: i32 = t.lo.get();
            if t.lo.get()<0i32 { break; }
            let mut h: i32 = t.hi.get();
            let mut lt: Object = t.left.get();
            let mut rt: Object = t.right.get();
            let mut mid: i32 = (((l).wrapping_add(h) as u32>>(1i32&0x1f)) as i32);
            t.right.set(ArrayPrefixHelpers_CumulateTask::new(t, fn_, a, org, fnc, th, mid, h)?);
            rt = ArrayPrefixHelpers_CumulateTask::new(t, fn_, a, org, fnc, th, mid, h)?;
            let mut f: ArrayPrefixHelpers_CumulateTask = ArrayPrefixHelpers_CumulateTask::new(t, fn_, a, org, fnc, th, mid, h)?;
            t.left.set(ArrayPrefixHelpers_CumulateTask::new(t, fn_, a, org, fnc, th, l, mid)?);
            lt = ArrayPrefixHelpers_CumulateTask::new(t, fn_, a, org, fnc, th, l, mid)?;
            t = ArrayPrefixHelpers_CumulateTask::new(t, fn_, a, org, fnc, th, l, mid)?;
            mid = t.in.get();
            lt.in.set(mid);
            /* TODO: aconst_null  */
            t = lt;
            f = lt;
            let mut lout: Object = lt.out.get();
            let _t0 = fn_.apply(mid, lout)?;
            lout.in.set(_t0);
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
        rt = a[org as usize].clone();
        f = (org).wrapping_add(1i32);
        rt = t.in.get();
        f = l;
        mid = f;
        loop {
            if mid >= h { break; }
            let _t0 = fn_.apply(rt, a[mid as usize].clone())?;
            rt = _t0;
            a[mid as usize] = _t0;
            mid = mid.wrapping_add(1i32);
        }
        rt = a[l as usize].clone();
        f = (l).wrapping_add(1i32);
        loop {
            if f >= h { break; }
            let _t0 = fn_.apply(rt, a[f as usize].clone())?;
            rt = _t0;
            f = f.wrapping_add(1i32);
        }
        rt = t.in.get();
        t.out.set(rt);
        loop {
            let _t0 = t.getCompleter()?;
            mid = _t0;
            f = mid;
            if (lt&4i32)==0i32 { break; }
            t.quietlyComplete()?;
            let _t0 = f.getPendingCount()?;
            lout = _t0;
            t = f;
            let mut lt: Object = f.left.get();
            let mut rt: Object = f.right.get();
            let mut lout: Object = lt.out.get();
            let _t1 = fn_.apply(lout, rt.out.get())?;
            lout.out.set(_t1);
            lout = 0i32;
            c = ((lout|lt)|lout);
            let _t2 = f.compareAndSetPendingCount(lout, c)?;
            lt = 2i32;
            t = f;
            let _t3 = f.fork()?;
            let _t4 = f.compareAndSetPendingCount(lout, (lout|lt))?;
        }
        Ok(())
    }
}
