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
    #[binary_name       = "java/util/Formatter$FormatSpecifier$BigDecimalLayout"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Formatter.java"]
    #[inner_classes     = "java/util/Formatter$FormatSpecifier:java/util/Formatter:FormatSpecifier:8;java/util/Formatter$FormatSpecifier$BigDecimalLayout:java/util/Formatter$FormatSpecifier:BigDecimalLayout:2;java/util/Formatter$BigDecimalLayoutForm:java/util/Formatter:BigDecimalLayoutForm:16409"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Formatter$FormatSpecifier$BigDecimalLayout"]

    pub struct Formatter_FormatSpecifier_BigDecimalLayout {
        #[cfg_attr(any(), java_field(name = "mant", descriptor = "Ljava/lang/StringBuilder;", access = "private", modifiers = "", is_static = false))]
        pub mant: StringBuilder,
        #[cfg_attr(any(), java_field(name = "exp", descriptor = "Ljava/lang/StringBuilder;", access = "private", modifiers = "", is_static = false))]
        pub exp: StringBuilder,
        #[cfg_attr(any(), java_field(name = "dot", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub dot: bool,
        #[cfg_attr(any(), java_field(name = "scale", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub scale: i32,
        #[cfg_attr(any(), java_field(name = "this$0", descriptor = "Ljava/util/Formatter$FormatSpecifier;", access = "package", modifiers = "final synthetic", is_static = false))]
        pub this_0: Formatter_FormatSpecifier,
    }

    impl Formatter_FormatSpecifier_BigDecimalLayout {
        #[java_method(name = "<init>", descriptor = "(Ljava/util/Formatter$FormatSpecifier;Ljava/math/BigInteger;ILjava/util/Formatter$BigDecimalLayoutForm;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":4112;:0;:0;:0")]
        pub fn new(mut arg_0: Formatter_FormatSpecifier, mut intVal: BigInteger, mut scale: i32, mut form: Formatter_BigDecimalLayoutForm) -> Result<Self> {
            let mut this = Self::default();
            this.__set_this_0(Clone::clone(&arg_0));
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_dot((0i32 != 0i32));
            this.layout(Clone::clone(&intVal), scale, Clone::clone(&form))?;
            Ok(this)
        }

        #[java_method(name = "hasDot", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasDot(&self) -> Result<bool> {
            let this = self;
            Ok(this.__get_dot())
        }

        #[java_method(name = "scale", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn scale(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_scale())
        }

        #[java_method(name = "mantissa", descriptor = "()Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mantissa(&self) -> Result<StringBuilder> {
            let this = self;
            Ok(this.__get_mant())
        }

        #[java_method(name = "exponent", descriptor = "()Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn exponent(&self) -> Result<StringBuilder> {
            let this = self;
            Ok(this.__get_exp())
        }

        #[java_method(name = "layout", descriptor = "(Ljava/math/BigInteger;ILjava/util/Formatter$BigDecimalLayoutForm;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn layout(&self, mut intVal: BigInteger, mut scale: i32, mut form: Formatter_BigDecimalLayoutForm) -> Result<()> {
            let this = self;
            let _t0 = intVal.toString()?;
            let mut coeff: String = _t0;
            this.__set_scale(scale);
            let _t1 = coeff.length()?;
            let mut len: i32 = _t1;
            this.__set_mant(Clone::clone(&StringBuilder::new_i((len).wrapping_add(14i32))?));
            if (scale==0) {
                if len > 1i32 {
                    let _t2 = coeff.charAt(0i32)?;
                    let _t3 = this.__get_mant().append_c(_t2)?;
                    if Object::from_any(form.clone()) == Object::from_any(Formatter_BigDecimalLayoutForm::SCIENTIFIC().clone()) {
                        let _t4 = this.__get_mant().append_c(((46i32) as u16))?;
                        this.__set_dot((1i32 != 0i32));
                        let _t5 = this.__get_mant().append_seq_i_i(Object::from_any(coeff.clone()), 1i32, len)?;
                        this.__set_exp(StringBuilder::new_str(Clone::clone(&String::from("+")))?);
                        if len < 10i32 {
                            let _t6 = this.__get_exp().append_c(((48i32) as u16))?;
                            let _t7 = _t6.append_i((len).wrapping_sub(1i32))?;
                        } else {
                            let _t6 = this.__get_exp().append_i((len).wrapping_sub(1i32))?;
                        }
                    } else {
                        let _t4 = this.__get_mant().append_seq_i_i(Object::from_any(coeff.clone()), 1i32, len)?;
                    }
                } else {
                    let _t2 = this.__get_mant().append_str(Clone::clone(&coeff))?;
                    this.__set_exp(StringBuilder::new_str(Clone::clone(&String::from("+00")))?);
                }
            } else {
                if Object::from_any(form.clone()) == Object::from_any(Formatter_BigDecimalLayoutForm::DECIMAL_FLOAT().clone()) {
                    if scale >= len {
                        let _t2 = this.__get_mant().append_str(Clone::clone(&String::from("0.")))?;
                        this.__set_dot((1i32 != 0i32));
                        this.__get_this_0().trailingZeros(Clone::clone(&this.__get_mant()), (scale).wrapping_sub(len))?;
                        let _t3 = this.__get_mant().append_str(Clone::clone(&coeff))?;
                    } else {
                        if (scale>0) {
                            let mut pad = (len).wrapping_sub(scale);
                            let _t2 = this.__get_mant().append_seq_i_i(Object::from_any(coeff.clone()), 0i32, pad)?;
                            let _t3 = this.__get_mant().append_c(((46i32) as u16))?;
                            this.__set_dot((1i32 != 0i32));
                            let _t4 = this.__get_mant().append_seq_i_i(Object::from_any(coeff.clone()), pad, len)?;
                        } else {
                            let _t2 = this.__get_mant().append_seq_i_i(Object::from_any(coeff.clone()), 0i32, len)?;
                            let _t3 = intVal.signum()?;
                            if (_t3!=0) {
                                this.__get_this_0().trailingZeros(Clone::clone(&this.__get_mant()), (scale).wrapping_neg())?;
                            }
                            this.__set_scale(0i32);
                        }
                    }
                } else {
                    let _t2 = coeff.charAt(0i32)?;
                    let _t3 = this.__get_mant().append_c(_t2)?;
                    if len > 1i32 {
                        let _t4 = this.__get_mant().append_c(((46i32) as u16))?;
                        this.__set_dot((1i32 != 0i32));
                        let _t5 = this.__get_mant().append_seq_i_i(Object::from_any(coeff.clone()), 1i32, len)?;
                    }
                    this.__set_exp(Clone::clone(&StringBuilder::new()?));
                    let mut pad = (((scale as i64)).wrapping_neg()).wrapping_add(((len).wrapping_sub(1i32) as i64));
                    if (((pad>(0i64)) as i32-((pad)<(0i64)) as i32)!=0) {
                        let _t4: i64 = Math::abs_l(pad)?;
                        let mut abs: i64 = _t4;
                        let _t5 = this.__get_exp().append_c((((if (((pad>(0i64)) as i32-((pad)<(0i64)) as i32)<0) { 45i32 } else { 43i32 })) as u16))?;
                        if (((abs>(10i64)) as i32-((abs)<(10i64)) as i32)<0) {
                            let _t6 = this.__get_exp().append_c(((48i32) as u16))?;
                        }
                        let _t6 = this.__get_exp().append_l(abs)?;
                    } else {
                        let _t4 = this.__get_exp().append_str(Clone::clone(&String::from("+00")))?;
                    }
                }
            }
            Ok(())
        }
    }
}
