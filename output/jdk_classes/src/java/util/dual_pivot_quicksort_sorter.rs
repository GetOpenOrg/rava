#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/DualPivotQuicksort$Sorter",
    super_class = "java/util/concurrent/CountedCompleter",
    interfaces  = "",
    access      = "final",
    source      = "DualPivotQuicksort.java",
))]
pub struct DualPivotQuicksort_Sorter {
    #[cfg_attr(any(), java_field(name = "a", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub a: Field<Object>,
    #[cfg_attr(any(), java_field(name = "b", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub b: Field<Object>,
    #[cfg_attr(any(), java_field(name = "low", descriptor = "I", access = "private final"))]
    pub low: Field<i32>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "private final"))]
    pub size: Field<i32>,
    #[cfg_attr(any(), java_field(name = "offset", descriptor = "I", access = "private final"))]
    pub offset: Field<i32>,
    #[cfg_attr(any(), java_field(name = "depth", descriptor = "I", access = "private final"))]
    pub depth: Field<i32>,
}

impl DualPivotQuicksort_Sorter {
    // java: <init>(Ljava/util/concurrent/CountedCompleter;Ljava/lang/Object;Ljava/lang/Object;IIII)V
    pub fn new(parent: Object, a: Object, b: Object, low: i32, size: i32, offset: i32, depth: i32) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), b: Field::new(Default::default()), low: Field::new(0), size: Field::new(0), offset: Field::new(0), depth: Field::new(0) };
        /* invokespecial Method java/util/concurrent/CountedCompleter.<init>:(Ljava/util/concurrent/CountedCompleter;)V */
        this.a.set(a);
        this.b.set(b);
        this.low.set(low);
        this.size.set(size);
        this.offset.set(offset);
        this.depth.set(depth);
        Ok(this)
    }

    // java: compute()V
    pub fn compute(&self) -> Result<()> {
        let this = self;
        this.setPendingCount(2i32)?;
        let mut half: i32 = (this.size.get()>>((1i32&0x1f)));
        let _t0 = DualPivotQuicksort_Sorter::new(this, this.b.get(), this.a.get(), this.low.get(), half, this.offset.get(), (this.depth.get()).wrapping_add(1i32))?.fork()?;
        DualPivotQuicksort_Sorter::new(this, this.b.get(), this.a.get(), (this.low.get()).wrapping_add(half), (this.size.get()).wrapping_sub(half), this.offset.get(), (this.depth.get()).wrapping_add(1i32))?.compute()?;
        DualPivotQuicksort::sort__dualpi_arr_i_i_i_i(this, this.a.get(), this.depth.get(), this.low.get(), (this.low.get()).wrapping_add(this.size.get()))?;
        DualPivotQuicksort::sort__dualpi_arr_l_i_i_i(this, this.a.get(), this.depth.get(), this.low.get(), (this.low.get()).wrapping_add(this.size.get()))?;
        DualPivotQuicksort::sort__dualpi_arr_f_i_i_i(this, this.a.get(), this.depth.get(), this.low.get(), (this.low.get()).wrapping_add(this.size.get()))?;
        DualPivotQuicksort::sort__dualpi_arr_d_i_i_i(this, this.a.get(), this.depth.get(), this.low.get(), (this.low.get()).wrapping_add(this.size.get()))?;
        String::new().append(&String::from("Unknown type of array:"))?;
        let _t1 = this.a.get().getClass()?;
        let _t2 = _t1.getName()?;
        String::new().append(&_t2)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.tryComplete()?;
        Ok(())
    }

    // java: onCompletion(Ljava/util/concurrent/CountedCompleter;)V
    pub fn onCompletion(&self, caller: Object) -> Result<()> {
        let this = self;
        let mut mi: i32 = (this.low.get()).wrapping_add((this.size.get()>>((1i32&0x1f))));
        let mut src: i32 = (this.depth.get()&1i32)==0i32;
        /* TODO: aconst_null  */
        /* invokespecial Method java/util/DualPivotQuicksort$Merger.<init>:(Ljava/util/concurrent/CountedCompleter;Ljava/lang/Object;ILjava/lang/Object;IILjava/lang/Object;II)V */
        let _t0 = this.low.get().invoke()?;
        Ok(())
    }

    // java: forkSorter(III)V
    pub fn forkSorter(&self, depth: i32, low: i32, high: i32) -> Result<()> {
        let this = self;
        this.addToPendingCount(1i32)?;
        let mut a: Object = this.a.get();
        let _t0 = DualPivotQuicksort_Sorter::new(this, a, this.b.get(), low, (high).wrapping_sub(low), this.offset.get(), depth)?.fork()?;
        Ok(())
    }
}
