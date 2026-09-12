#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/DualPivotQuicksort$Merger",
    super_class = "java/util/concurrent/CountedCompleter",
    interfaces  = "",
    access      = "final",
    source      = "DualPivotQuicksort.java",
))]
pub struct DualPivotQuicksort_Merger {
    #[cfg_attr(any(), java_field(name = "dst", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub dst: Field<Object>,
    #[cfg_attr(any(), java_field(name = "a1", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub a1: Field<Object>,
    #[cfg_attr(any(), java_field(name = "a2", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub a2: Field<Object>,
    #[cfg_attr(any(), java_field(name = "k", descriptor = "I", access = "private final"))]
    pub k: Field<i32>,
    #[cfg_attr(any(), java_field(name = "lo1", descriptor = "I", access = "private final"))]
    pub lo1: Field<i32>,
    #[cfg_attr(any(), java_field(name = "hi1", descriptor = "I", access = "private final"))]
    pub hi1: Field<i32>,
    #[cfg_attr(any(), java_field(name = "lo2", descriptor = "I", access = "private final"))]
    pub lo2: Field<i32>,
    #[cfg_attr(any(), java_field(name = "hi2", descriptor = "I", access = "private final"))]
    pub hi2: Field<i32>,
}

impl DualPivotQuicksort_Merger {
    // java: <init>(Ljava/util/concurrent/CountedCompleter;Ljava/lang/Object;ILjava/lang/Object;IILjava/lang/Object;II)V
    pub fn new(parent: Object, dst: Object, k: i32, a1: Object, lo1: i32, hi1: i32, a2: Object, lo2: i32, hi2: i32) -> Result<Self> {
        let this = Self { dst: Field::new(Default::default()), a1: Field::new(Default::default()), a2: Field::new(Default::default()), k: Field::new(0), lo1: Field::new(0), hi1: Field::new(0), lo2: Field::new(0), hi2: Field::new(0) };
        /* invokespecial Method java/util/concurrent/CountedCompleter.<init>:(Ljava/util/concurrent/CountedCompleter;)V */
        this.dst.set(dst);
        this.k.set(k);
        this.a1.set(a1);
        this.lo1.set(lo1);
        this.hi1.set(hi1);
        this.a2.set(a2);
        this.lo2.set(lo2);
        this.hi2.set(hi2);
        Ok(this)
    }

    // java: compute()V
    pub fn compute(&self) -> Result<()> {
        let this = self;
        DualPivotQuicksort::mergeParts__dualpi_arr_i_i_arr_i_i_i_arr_i_i_i(this, this.dst.get(), this.k.get(), this.a1.get(), this.lo1.get(), this.hi1.get(), this.a2.get(), this.lo2.get(), this.hi2.get())?;
        DualPivotQuicksort::mergeParts__dualpi_arr_l_i_arr_l_i_i_arr_l_i_i(this, this.dst.get(), this.k.get(), this.a1.get(), this.lo1.get(), this.hi1.get(), this.a2.get(), this.lo2.get(), this.hi2.get())?;
        DualPivotQuicksort::mergeParts__dualpi_arr_f_i_arr_f_i_i_arr_f_i_i(this, this.dst.get(), this.k.get(), this.a1.get(), this.lo1.get(), this.hi1.get(), this.a2.get(), this.lo2.get(), this.hi2.get())?;
        DualPivotQuicksort::mergeParts__dualpi_arr_d_i_arr_d_i_i_arr_d_i_i(this, this.dst.get(), this.k.get(), this.a1.get(), this.lo1.get(), this.hi1.get(), this.a2.get(), this.lo2.get(), this.hi2.get())?;
        String::new().append(&String::from("Unknown type of array:"))?;
        let _t0 = this.dst.get().getClass()?;
        let _t1 = _t0.getName()?;
        String::new().append(&_t1)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.propagateCompletion()?;
        Ok(())
    }

    // java: forkMerger(Ljava/lang/Object;ILjava/lang/Object;IILjava/lang/Object;II)V
    pub fn forkMerger(&self, dst: Object, k: i32, a1: Object, lo1: i32, hi1: i32, a2: Object, lo2: i32, hi2: i32) -> Result<()> {
        let this = self;
        this.addToPendingCount(1i32)?;
        let _t0 = DualPivotQuicksort_Merger::new(this, dst, k, a1, lo1, hi1, a2, lo2, hi2)?.fork()?;
        Ok(())
    }
}
