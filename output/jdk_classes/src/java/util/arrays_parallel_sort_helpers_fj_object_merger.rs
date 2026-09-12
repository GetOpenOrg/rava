#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ArraysParallelSortHelpers$FJObject$Merger",
    super_class = "java/util/concurrent/CountedCompleter",
    interfaces  = "",
    access      = "final",
    source      = "ArraysParallelSortHelpers.java",
))]
pub struct ArraysParallelSortHelpers_FJObject_Merger<T> {
    #[cfg_attr(any(), java_field(name = "a", descriptor = "[Ljava/lang/Object;", access = "final"))]
    pub a: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "w", descriptor = "[Ljava/lang/Object;", access = "final"))]
    pub w: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "lbase", descriptor = "I", access = "final"))]
    pub lbase: Field<i32>,
    #[cfg_attr(any(), java_field(name = "lsize", descriptor = "I", access = "final"))]
    pub lsize: Field<i32>,
    #[cfg_attr(any(), java_field(name = "rbase", descriptor = "I", access = "final"))]
    pub rbase: Field<i32>,
    #[cfg_attr(any(), java_field(name = "rsize", descriptor = "I", access = "final"))]
    pub rsize: Field<i32>,
    #[cfg_attr(any(), java_field(name = "wbase", descriptor = "I", access = "final"))]
    pub wbase: Field<i32>,
    #[cfg_attr(any(), java_field(name = "gran", descriptor = "I", access = "final"))]
    pub gran: Field<i32>,
    #[cfg_attr(any(), java_field(name = "comparator", descriptor = "Ljava/util/Comparator;"))]
    pub comparator: Field<Object>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + 'static> ArraysParallelSortHelpers_FJObject_Merger<T> {
    // java: <init>(Ljava/util/concurrent/CountedCompleter;[Ljava/lang/Object;[Ljava/lang/Object;IIIIIILjava/util/Comparator;)V
    pub fn new(par: Object, a: Vec<Object>, w: Vec<Object>, lbase: i32, lsize: i32, rbase: i32, rsize: i32, wbase: i32, gran: i32, comparator: Object) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), w: Field::new(Default::default()), lbase: Field::new(0), lsize: Field::new(0), rbase: Field::new(0), rsize: Field::new(0), wbase: Field::new(0), gran: Field::new(0), comparator: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/concurrent/CountedCompleter.<init>:(Ljava/util/concurrent/CountedCompleter;)V */
        this.a.set(a);
        this.w.set(w);
        this.lbase.set(lbase);
        this.lsize.set(lsize);
        this.rbase.set(rbase);
        this.rsize.set(rsize);
        this.wbase.set(wbase);
        this.gran.set(gran);
        this.comparator.set(comparator);
        Ok(this)
    }

    // java: compute()V
    pub fn compute(&self) -> Result<()> {
        let this = self;
        let mut c: Object = this.comparator.get();
        let mut a: Vec<Object> = this.a.get();
        let mut w: Vec<Object> = this.w.get();
        let mut lb: i32 = this.lbase.get();
        let mut ln: i32 = this.lsize.get();
        let mut rb: i32 = this.rbase.get();
        let mut rn: i32 = this.rsize.get();
        let mut k: i32 = this.wbase.get();
        let mut g: i32 = this.gran.get();
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut rh: i32 = rn;
        let mut lh: i32 = ((ln as u32>>(1i32&0x1f)) as i32);
        let mut split: Object = a[(((ln as u32>>(1i32&0x1f)) as i32)).wrapping_add(lb) as usize].clone();
        let mut lo: i32 = 0i32;
        loop {
            if lo >= rh { break; }
            let mut rm: i32 = (((lo).wrapping_add(rh) as u32>>(1i32&0x1f)) as i32);
            let _t0 = c.compare(split, a[(rm).wrapping_add(rb) as usize].clone())?;
            rh = rm;
            lo = (rm).wrapping_add(1i32);
        }
        lh = ln;
        rh = ((rn as u32>>(1i32&0x1f)) as i32);
        split = a[(((rn as u32>>(1i32&0x1f)) as i32)).wrapping_add(rb) as usize].clone();
        lo = 0i32;
        loop {
            if lo >= lh { break; }
            rm = (((lo).wrapping_add(lh) as u32>>(1i32&0x1f)) as i32);
            let _t0 = c.compare(split, a[(rm).wrapping_add(lb) as usize].clone())?;
            lh = rm;
            lo = (rm).wrapping_add(1i32);
        }
        split = ArraysParallelSortHelpers_FJObject_Merger::new(this, a, w, (lb).wrapping_add(lh), (ln).wrapping_sub(lh), (rb).wrapping_add(rh), (rn).wrapping_sub(rh), ((k).wrapping_add(lh)).wrapping_add(rh), g, c)?;
        rn = rh;
        ln = lh;
        this.addToPendingCount(1i32)?;
        let _t0 = split.fork()?;
        lh = (lb).wrapping_add(ln);
        rh = (rb).wrapping_add(rn);
        loop {
            if lb >= lh { break; }
            lo = a[lb as usize].clone();
            rm = a[rb as usize].clone();
            let _t0 = c.compare(a[lb as usize].clone(), a[rb as usize].clone())?;
            lb = lb.wrapping_add(1i32);
            split = lo;
            rb = rb.wrapping_add(1i32);
            split = rm;
            k = k.wrapping_add(1i32);
            w[k as usize] = split;
        }
        System::arraycopy(&a, rb, &w, k, (rh).wrapping_sub(rb))?;
        System::arraycopy(&a, lb, &w, k, (lh).wrapping_sub(lb))?;
        this.tryComplete()?;
        Ok(())
    }
}
