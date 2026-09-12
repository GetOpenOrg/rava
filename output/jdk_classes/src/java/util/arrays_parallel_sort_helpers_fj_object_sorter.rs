#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/ArraysParallelSortHelpers$FJObject$Sorter",
    super_class = "java/util/concurrent/CountedCompleter",
    interfaces  = "",
    access      = "final",
    source      = "ArraysParallelSortHelpers.java",
))]
pub struct ArraysParallelSortHelpers_FJObject_Sorter<T> {
    #[cfg_attr(any(), java_field(name = "a", descriptor = "[Ljava/lang/Object;", access = "final"))]
    pub a: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "w", descriptor = "[Ljava/lang/Object;", access = "final"))]
    pub w: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "base", descriptor = "I", access = "final"))]
    pub base: Field<i32>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "final"))]
    pub size: Field<i32>,
    #[cfg_attr(any(), java_field(name = "wbase", descriptor = "I", access = "final"))]
    pub wbase: Field<i32>,
    #[cfg_attr(any(), java_field(name = "gran", descriptor = "I", access = "final"))]
    pub gran: Field<i32>,
    #[cfg_attr(any(), java_field(name = "comparator", descriptor = "Ljava/util/Comparator;"))]
    pub comparator: Field<Object>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + 'static> ArraysParallelSortHelpers_FJObject_Sorter<T> {
    // java: <init>(Ljava/util/concurrent/CountedCompleter;[Ljava/lang/Object;[Ljava/lang/Object;IIIILjava/util/Comparator;)V
    pub fn new(par: Object, a: Vec<Object>, w: Vec<Object>, base: i32, size: i32, wbase: i32, gran: i32, comparator: Object) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), w: Field::new(Default::default()), base: Field::new(0), size: Field::new(0), wbase: Field::new(0), gran: Field::new(0), comparator: Field::new(Default::default()), _phantom: std::marker::PhantomData };
        /* invokespecial Method java/util/concurrent/CountedCompleter.<init>:(Ljava/util/concurrent/CountedCompleter;)V */
        this.a.set(a);
        this.w.set(w);
        this.base.set(base);
        this.size.set(size);
        this.wbase.set(wbase);
        this.gran.set(gran);
        this.comparator.set(comparator);
        Ok(this)
    }

    // java: compute()V
    pub fn compute(&self) -> Result<()> {
        let this = self;
        let mut s: ArraysParallelSortHelpers_FJObject_Sorter = this;
        let mut c: Object = this.comparator.get();
        let mut a: Vec<Object> = this.a.get();
        let mut w: Vec<Object> = this.w.get();
        let mut b: i32 = this.base.get();
        let mut n: i32 = this.size.get();
        let mut wb: i32 = this.wbase.get();
        let mut g: i32 = this.gran.get();
        loop {
            if n <= g { break; }
            let mut h: i32 = ((n as u32>>(1i32&0x1f)) as i32);
            let mut q: i32 = ((h as u32>>(1i32&0x1f)) as i32);
            let mut u: i32 = (h).wrapping_add(q);
            let mut fc: ArraysParallelSortHelpers_Relay = ArraysParallelSortHelpers_Relay::new(ArraysParallelSortHelpers_FJObject_Merger::new(s, w, a, wb, h, (wb).wrapping_add(h), (n).wrapping_sub(h), b, g, c)?)?;
            let mut rc: ArraysParallelSortHelpers_Relay = ArraysParallelSortHelpers_Relay::new(ArraysParallelSortHelpers_FJObject_Merger::new(fc, a, w, (b).wrapping_add(h), q, (b).wrapping_add(u), (n).wrapping_sub(u), (wb).wrapping_add(h), g, c)?)?;
            let _t0 = ArraysParallelSortHelpers_FJObject_Sorter::new(rc, a, w, (b).wrapping_add(u), (n).wrapping_sub(u), (wb).wrapping_add(u), g, c)?.fork()?;
            let _t1 = ArraysParallelSortHelpers_FJObject_Sorter::new(rc, a, w, (b).wrapping_add(h), q, (wb).wrapping_add(h), g, c)?.fork()?;
            let mut bc: ArraysParallelSortHelpers_Relay = ArraysParallelSortHelpers_Relay::new(ArraysParallelSortHelpers_FJObject_Merger::new(fc, a, w, b, q, (b).wrapping_add(q), (h).wrapping_sub(q), wb, g, c)?)?;
            let _t2 = ArraysParallelSortHelpers_FJObject_Sorter::new(bc, a, w, (b).wrapping_add(q), (h).wrapping_sub(q), (wb).wrapping_add(q), g, c)?.fork()?;
            s = ArraysParallelSortHelpers_EmptyCompleter::new(bc)?;
            n = q;
        }
        TimSort::sort(&a, b, (b).wrapping_add(n), c, &w, wb, n)?;
        s.tryComplete()?;
        Ok(())
    }
}
