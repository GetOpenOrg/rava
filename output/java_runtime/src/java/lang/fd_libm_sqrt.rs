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
    #[binary_name       = "java/lang/FdLibm$Sqrt"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "FdLibm.java"]
    #[inner_classes     = "java/lang/FdLibm$Sqrt:java/lang/FdLibm:Sqrt:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/FdLibm$Sqrt;java/lang/Object"]

    pub struct FdLibm_Sqrt;

    impl FdLibm_Sqrt {
        #[cfg_attr(any(), java_field(name = "tiny", descriptor = "D", access = "private", modifiers = "static final", is_static = true, constant_value = "1e-300"))]
        // static field: tiny:D
        pub fn tiny() -> f64 {
            1e-300f64
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/FdLibm$Sqrt.<init>:()V")
        }

        #[java_method(name = "compute", descriptor = "(D)D", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compute(mut x: f64) -> Result<f64> {
            let mut z: f64 = 0f64;
            let mut sign: i32 = -2147483648i32;
            let _t0: i32 = FdLibm::__HI_d(x)?;
            let mut ix0: i32 = _t0;
            let _t1: i32 = FdLibm::__LO_d(x)?;
            let mut ix1: i32 = _t1;
            if (ix0&2146435072i32) == 2146435072i32 {
                return Ok(((x*x)+x));
            }
            if (((ix0&(sign^-1i32))|ix1)==0) {
                return Ok(x);
            }
            if (ix0<0) {
                return Ok(((x-x)/(x-x)));
            }
            let mut m = (ix0>>((20i32&0x1f)));
            loop {
                if (ix0!=0) { break; }
                m = m.wrapping_sub(21i32);
                ix0 = (ix0|((ix1 as u32>>(11i32&0x1f)) as i32));
                ix1 = (ix1<<(21i32&0x1f));
            }
            let mut i: i32 = 0i32;
            loop {
                if ((ix0&1048576i32)!=0) { break; }
                ix0 = (ix0<<(1i32&0x1f));
                i = i.wrapping_add(1i32);
            }
            m = (m).wrapping_sub((i).wrapping_sub(1i32));
            ix0 = (ix0|((ix1 as u32>>((32i32).wrapping_sub(i)&0x1f)) as i32));
            ix1 = (ix1<<(i&0x1f));
            /* TODO: wide_iinc 13 -1023 */
            ix0 = ((ix0&1048575i32)|1048576i32);
            if ((m&1i32)!=0) {
                ix0 = (ix0).wrapping_add((ix0).wrapping_add((((ix1&sign) as u32>>(31i32&0x1f)) as i32)));
                ix1 = (ix1).wrapping_add(ix1);
            }
            m = (m>>((1i32&0x1f)));
            ix0 = (ix0).wrapping_add((ix0).wrapping_add((((ix1&sign) as u32>>(31i32&0x1f)) as i32)));
            ix1 = (ix1).wrapping_add(ix1);
            let mut s1: i32 = 0i32;
            let mut s0: i32 = 0i32;
            let mut q1: i32 = 0i32;
            let mut q: i32 = 0i32;
            let mut r: i32 = 2097152i32;
            let mut t = Default::default();
            loop {
                if (r==0) { break; }
                t = (s0).wrapping_add(r);
                if t <= ix0 {
                    s0 = (t).wrapping_add(r);
                    ix0 = (ix0).wrapping_sub(t);
                    q = (q).wrapping_add(r);
                }
                ix0 = (ix0).wrapping_add((ix0).wrapping_add((((ix1&sign) as u32>>(31i32&0x1f)) as i32)));
                ix1 = (ix1).wrapping_add(ix1);
                r = ((r as u32>>(1i32&0x1f)) as i32);
            }
            r = sign;
            loop {
                if (r==0) { break; }
                let mut t1 = (s1).wrapping_add(r);
                t = s0;
                let _t2: i32 = Integer::compareUnsigned(t1, ix1)?;
                s1 = (t1).wrapping_add(r);
                if ((s1&sign)==0) {
                    s0 = s0.wrapping_add(1i32);
                }
                ix0 = (ix0).wrapping_sub(t);
                let _t3: i32 = Integer::compareUnsigned(ix1, t1)?;
                if (_t3<0) {
                    ix0 = ix0.wrapping_sub(1i32);
                }
                ix1 = (ix1).wrapping_sub(t1);
                q1 = (q1).wrapping_add(r);
                ix0 = (ix0).wrapping_add((ix0).wrapping_add((((ix1&sign) as u32>>(31i32&0x1f)) as i32)));
                ix1 = (ix1).wrapping_add(ix1);
                r = ((r as u32>>(1i32&0x1f)) as i32);
            }
            z = 1f64;
            z = 1f64;
            if q1 == -1i32 {
                q1 = 0i32;
                q = q.wrapping_add(1i32);
            } else {
                if (((z>(1f64)) as i32-((z)<(1f64)) as i32)>0) {
                    if q1 == -2i32 {
                        q = q.wrapping_add(1i32);
                    }
                    q1 = q1.wrapping_add(2i32);
                } else {
                    q1 = (q1).wrapping_add((q1&1i32));
                }
            }
            ix0 = ((q>>((1i32&0x1f)))).wrapping_add(1071644672i32);
            ix1 = ((q1 as u32>>(1i32&0x1f)) as i32);
            if (q&1i32) == 1i32 {
                ix1 = (ix1|sign);
            }
            ix0 = (ix0).wrapping_add((m<<(20i32&0x1f)));
            let _t2: f64 = FdLibm::__HI_LO(ix0, ix1)?;
            Ok(_t2)
        }
    }
}
