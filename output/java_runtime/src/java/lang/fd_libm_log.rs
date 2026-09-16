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
    #[binary_name       = "java/lang/FdLibm$Log"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "FdLibm.java"]
    #[inner_classes     = "java/lang/FdLibm$Log:java/lang/FdLibm:Log:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/FdLibm$Log;java/lang/Object"]

    pub struct FdLibm_Log;

    impl FdLibm_Log {
        #[cfg_attr(any(), java_field(name = "ln2_hi", descriptor = "D", access = "private", modifiers = "static final", is_static = true, constant_value = "0.6931471803691238"))]
        // static field: ln2_hi:D
        pub fn ln2_hi() -> f64 {
            0.6931471803691238f64
        }

        #[cfg_attr(any(), java_field(name = "ln2_lo", descriptor = "D", access = "private", modifiers = "static final", is_static = true, constant_value = "1.9082149292705877e-10"))]
        // static field: ln2_lo:D
        pub fn ln2_lo() -> f64 {
            1.9082149292705877e-10f64
        }

        #[cfg_attr(any(), java_field(name = "Lg1", descriptor = "D", access = "private", modifiers = "static final", is_static = true, constant_value = "0.6666666666666735"))]
        // static field: Lg1:D
        pub fn Lg1() -> f64 {
            0.6666666666666735f64
        }

        #[cfg_attr(any(), java_field(name = "Lg2", descriptor = "D", access = "private", modifiers = "static final", is_static = true, constant_value = "0.3999999999940942"))]
        // static field: Lg2:D
        pub fn Lg2() -> f64 {
            0.3999999999940942f64
        }

        #[cfg_attr(any(), java_field(name = "Lg3", descriptor = "D", access = "private", modifiers = "static final", is_static = true, constant_value = "0.2857142874366239"))]
        // static field: Lg3:D
        pub fn Lg3() -> f64 {
            0.2857142874366239f64
        }

        #[cfg_attr(any(), java_field(name = "Lg4", descriptor = "D", access = "private", modifiers = "static final", is_static = true, constant_value = "0.22222198432149784"))]
        // static field: Lg4:D
        pub fn Lg4() -> f64 {
            0.22222198432149784f64
        }

        #[cfg_attr(any(), java_field(name = "Lg5", descriptor = "D", access = "private", modifiers = "static final", is_static = true, constant_value = "0.1818357216161805"))]
        // static field: Lg5:D
        pub fn Lg5() -> f64 {
            0.1818357216161805f64
        }

        #[cfg_attr(any(), java_field(name = "Lg6", descriptor = "D", access = "private", modifiers = "static final", is_static = true, constant_value = "0.15313837699209373"))]
        // static field: Lg6:D
        pub fn Lg6() -> f64 {
            0.15313837699209373f64
        }

        #[cfg_attr(any(), java_field(name = "Lg7", descriptor = "D", access = "private", modifiers = "static final", is_static = true, constant_value = "0.14798198605116586"))]
        // static field: Lg7:D
        pub fn Lg7() -> f64 {
            0.14798198605116586f64
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/FdLibm$Log.<init>:()V")
        }

        #[java_method(name = "compute", descriptor = "(D)D", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compute(mut x: f64) -> Result<f64> {
            let _t0: i32 = FdLibm::__HI_d(x)?;
            let mut hx: i32 = _t0;
            let _t1: i32 = FdLibm::__LO_d(x)?;
            let mut lx: i32 = _t1;
            let mut k: i32 = 0i32;
            if (((hx&2147483647i32)|lx)==0) {
                return Ok(f64::NEG_INFINITY);
            }
            if (hx<0) {
                return Ok(((x-x)/0f64));
            }
            k = k.wrapping_sub(54i32);
            x = (x*1.8014398509481984e+16f64);
            let _t2: i32 = FdLibm::__HI_d(x)?;
            hx = _t2;
            if hx >= 2146435072i32 {
                return Ok((x+x));
            }
            k = (k).wrapping_add(((hx>>((20i32&0x1f)))).wrapping_sub(1023i32));
            hx = (hx&1048575i32);
            let mut i = ((hx).wrapping_add(614244i32)&1048576i32);
            let _t3: f64 = FdLibm::__HI_d_i(x, (hx|(i^1072693248i32)))?;
            x = _t3;
            k = (k).wrapping_add((i>>((20i32&0x1f))));
            let mut f = (x-1f64);
            if (k==0) {
                return Ok(0f64);
            }
            let mut dk: f64 = (k as f64);
            return Ok(((dk*0.6931471803691238f64)+(dk*1.9082149292705877e-10f64)));
            let mut R = ((f*f)*(0.5f64-(0.3333333333333333f64*f)));
            if (k==0) {
                return Ok((f-R));
            }
            dk = (k as f64);
            return Ok(((dk*0.6931471803691238f64)-((R-(dk*1.9082149292705877e-10f64))-f)));
            let mut s = (f/(2.0f64+f));
            dk = (k as f64);
            let mut z = (s*s);
            i = (hx).wrapping_sub(398458i32);
            let mut w = (z*z);
            let mut j = (440401i32).wrapping_sub(hx);
            let mut t1 = (w*(0.3999999999940942f64+(w*(0.22222198432149784f64+(w*0.15313837699209373f64)))));
            let mut t2 = (z*(0.6666666666666735f64+(w*(0.2857142874366239f64+(w*(0.1818357216161805f64+(w*0.14798198605116586f64)))))));
            i = (i|j);
            R = (t2+t1);
            let mut hfsq = ((0.5f64*f)*f);
            if (k==0) {
                return Ok((f-(hfsq-(s*(hfsq+R)))));
            }
            return Ok(((dk*0.6931471803691238f64)-((hfsq-((s*(hfsq+R))+(dk*1.9082149292705877e-10f64)))-f)));
            if (k==0) {
                return Ok((f-(s*(f-R))));
            }
            Ok(((dk*0.6931471803691238f64)-(((s*(f-R))-(dk*1.9082149292705877e-10f64))-f)))
        }
    }
}
