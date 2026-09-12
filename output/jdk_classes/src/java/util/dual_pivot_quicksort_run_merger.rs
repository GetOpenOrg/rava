#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::constant::*;
use crate::java::lang::invoke::*;
use crate::java::util::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/DualPivotQuicksort$RunMerger",
    super_class = "java/util/concurrent/RecursiveTask",
    interfaces  = "",
    access      = "final",
    source      = "DualPivotQuicksort.java",
))]
pub struct DualPivotQuicksort_RunMerger {
    #[cfg_attr(any(), java_field(name = "a", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub a: Field<Object>,
    #[cfg_attr(any(), java_field(name = "b", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub b: Field<Object>,
    #[cfg_attr(any(), java_field(name = "run", descriptor = "[I", access = "private final"))]
    pub run: Field<Vec<i32>>,
    #[cfg_attr(any(), java_field(name = "offset", descriptor = "I", access = "private final"))]
    pub offset: Field<i32>,
    #[cfg_attr(any(), java_field(name = "aim", descriptor = "I", access = "private final"))]
    pub aim: Field<i32>,
    #[cfg_attr(any(), java_field(name = "lo", descriptor = "I", access = "private final"))]
    pub lo: Field<i32>,
    #[cfg_attr(any(), java_field(name = "hi", descriptor = "I", access = "private final"))]
    pub hi: Field<i32>,
}

impl DualPivotQuicksort_RunMerger {
    // java: <init>(Ljava/lang/Object;Ljava/lang/Object;II[III)V
    pub fn new(a: Object, b: Object, offset: i32, aim: i32, run: Vec<i32>, lo: i32, hi: i32) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), b: Field::new(Default::default()), run: Field::new(Default::default()), offset: Field::new(0), aim: Field::new(0), lo: Field::new(0), hi: Field::new(0) };
        /* invokespecial Method java/util/concurrent/RecursiveTask.<init>:()V */
        this.a.set(a);
        this.b.set(b);
        this.offset.set(offset);
        this.aim.set(aim);
        this.run.set(run);
        this.lo.set(lo);
        this.hi.set(hi);
        Ok(this)
    }

    // java: compute()Ljava/lang/Object;
    pub fn compute(&self) -> Result<Object> {
        let this = self;
        let _t0: Vec<i32> = DualPivotQuicksort::mergeRuns__arr_i_arr_i_i_i_z_arr_i_i_i(this.a.get(), this.b.get(), this.offset.get(), this.aim.get(), 1i32, &this.run.get(), this.lo.get(), this.hi.get())?;
        return Ok(_t0);
        let _t1: Vec<i64> = DualPivotQuicksort::mergeRuns__arr_l_arr_l_i_i_z_arr_i_i_i(this.a.get(), this.b.get(), this.offset.get(), this.aim.get(), 1i32, &this.run.get(), this.lo.get(), this.hi.get())?;
        return Ok(_t1);
        let _t2: Vec<f32> = DualPivotQuicksort::mergeRuns__arr_f_arr_f_i_i_z_arr_i_i_i(this.a.get(), this.b.get(), this.offset.get(), this.aim.get(), 1i32, &this.run.get(), this.lo.get(), this.hi.get())?;
        return Ok(_t2);
        let _t3: Vec<f64> = DualPivotQuicksort::mergeRuns__arr_d_arr_d_i_i_z_arr_i_i_i(this.a.get(), this.b.get(), this.offset.get(), this.aim.get(), 1i32, &this.run.get(), this.lo.get(), this.hi.get())?;
        return Ok(_t3);
        String::new().append(&String::from("Unknown type of array:"))?;
        let _t4 = this.a.get().getClass()?;
        let _t5 = _t4.getName()?;
        String::new().append(&_t5)?;
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: forkMe()Ljava/util/DualPivotQuicksort$RunMerger;
    pub fn forkMe(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.fork()?;
        Ok(this)
    }

    // java: getDestination()Ljava/lang/Object;
    pub fn getDestination(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.join()?;
        let _t1 = this.getRawResult()?;
        Ok(_t1)
    }
}
