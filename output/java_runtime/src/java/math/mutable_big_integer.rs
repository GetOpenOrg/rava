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
    #[binary_name       = "java/math/MutableBigInteger"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "MutableBigInteger.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/math/MutableBigInteger"]
    #[has_to_string_method = true]

    pub struct MutableBigInteger {
        #[cfg_attr(any(), java_field(name = "value", descriptor = "[I", is_static = false))]
        pub value: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "intLen", descriptor = "I", is_static = false))]
        pub intLen: i32,
        #[cfg_attr(any(), java_field(name = "offset", descriptor = "I", is_static = false))]
        pub offset: i32,
    }

    impl MutableBigInteger {
        #[cfg_attr(any(), java_field(name = "ONE", descriptor = "Ljava/math/MutableBigInteger;", access = "package", modifiers = "static final", is_static = true))]
        // static field: ONE:Ljava/math/MutableBigInteger;
        pub fn ONE() -> MutableBigInteger {
            panic!("stub: java/math/MutableBigInteger.ONE:Ljava/math/MutableBigInteger;")
        }

        #[cfg_attr(any(), java_field(name = "KNUTH_POW2_THRESH_LEN", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "6"))]
        // static field: KNUTH_POW2_THRESH_LEN:I
        pub fn KNUTH_POW2_THRESH_LEN() -> i32 {
            6
        }

        #[cfg_attr(any(), java_field(name = "KNUTH_POW2_THRESH_ZEROS", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: KNUTH_POW2_THRESH_ZEROS:I
        pub fn KNUTH_POW2_THRESH_ZEROS() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_offset(0i32);
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 1i32 as usize]));
            this.__set_value(Clone::clone(&_arr0));
            this.__set_intLen(0i32);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(I)V
        pub fn new_i(mut val: i32) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_offset(0i32);
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 1i32 as usize]));
            this.__set_value(Clone::clone(&_arr0));
            this.__set_intLen(1i32);
            this.__get_value().borrow_mut()[0i32 as usize] = val;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "([I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>([I)V
        pub fn new_arr_i(mut val: Rc<RefCell<Vec<i32>>>) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_offset(0i32);
            this.__set_value(Clone::clone(&val));
            this.__set_intLen((val.borrow().len() as i32));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/math/BigInteger;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/math/BigInteger;)V
        pub fn new_bigint(mut b: BigInteger) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_offset(0i32);
            this.__set_intLen((b.__get_mag().borrow().len() as i32));
            let _t0: Rc<RefCell<Vec<i32>>> = Arrays::copyOf_arr_i_i(Clone::clone(&b.__get_mag()), this.__get_intLen())?;
            this.__set_value(Clone::clone(&_t0));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/math/MutableBigInteger;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/math/MutableBigInteger;)V
        pub fn new_mutabl(mut val: MutableBigInteger) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_offset(0i32);
            this.__set_intLen(val.__get_intLen());
            let _t0: Rc<RefCell<Vec<i32>>> = Arrays::copyOfRange_arr_i_i_i(Clone::clone(&val.__get_value()), val.__get_offset(), (val.__get_offset()).wrapping_add(this.__get_intLen()))?;
            this.__set_value(Clone::clone(&_t0));
            Ok(this)
        }

        #[java_method(name = "ones", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ones(&self, mut n: i32) -> Result<()> {
            let this = self;
            if n > (this.__get_value().borrow().len() as i32) {
                let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; n as usize]));
                this.__set_value(Clone::clone(&_arr0));
            }
            Arrays::fill_arr_i_i(Clone::clone(&this.__get_value()), -1i32)?;
            this.__set_offset(0i32);
            this.__set_intLen(n);
            Ok(())
        }

        #[java_method(name = "getMagnitudeArray", descriptor = "()[I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMagnitudeArray(&self) -> Result<Rc<RefCell<Vec<i32>>>> {
            let this = self;
            if (this.__get_value().borrow().len() as i32) != this.__get_intLen() {
                let _t0: Rc<RefCell<Vec<i32>>> = Arrays::copyOfRange_arr_i_i_i(Clone::clone(&this.__get_value()), this.__get_offset(), (this.__get_offset()).wrapping_add(this.__get_intLen()))?;
                let mut tmp: Rc<RefCell<Vec<i32>>> = _t0;
                Arrays::fill_arr_i_i(Clone::clone(&this.__get_value()), 0i32)?;
                this.__set_offset(0i32);
                this.__set_intLen((tmp.borrow().len() as i32));
                this.__set_value(Clone::clone(&tmp));
            }
            Ok(this.__get_value())
        }

        #[java_method(name = "toLong", descriptor = "()J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toLong(&self) -> Result<i64> {
            let this = self;
            if this.__get_intLen() > 2i32 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (this.__get_intLen()==0) {
                return Ok(0i64);
            }
            let mut d = ((this.__get_value().borrow()[this.__get_offset() as usize] as i64)&(4294967295i64));
            Ok((if this.__get_intLen() == 2i32 { ((d).wrapping_shl((32i32&0x3f) as u32)|(((this.__get_value().borrow()[(this.__get_offset()).wrapping_add(1i32) as usize] as i64)&(4294967295i64)))) } else { d }))
        }

        #[java_method(name = "toBigInteger", descriptor = "(I)Ljava/math/BigInteger;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toBigInteger(I)Ljava/math/BigInteger;
        pub fn toBigInteger_i(&self, mut sign: i32) -> Result<BigInteger> {
            let this = self;
            if (sign==0) {
                return Ok(BigInteger::ZERO());
            }
            let _t0 = this.getMagnitudeArray()?;
            Ok(BigInteger::new_arr_i_i(Clone::clone(&_t0), sign)?)
        }

        #[java_method(name = "toBigInteger", descriptor = "()Ljava/math/BigInteger;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toBigInteger()Ljava/math/BigInteger;
        pub fn toBigInteger(&self) -> Result<BigInteger> {
            let this = self;
            this.normalize()?;
            let _t0 = this.isZero()?;
            let _t1 = this.toBigInteger_i((!(_t0) as i32))?;
            Ok(_t1)
        }

        #[java_method(name = "toBigDecimal", descriptor = "(II)Ljava/math/BigDecimal;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toBigDecimal(&self, mut sign: i32, mut scale: i32) -> Result<BigDecimal> {
            let this = self;
            if (sign==0) {
                let _t0: BigDecimal = BigDecimal::zeroValueOf(scale)?;
                return Ok(_t0);
            }
            let _t0 = this.getMagnitudeArray()?;
            let mut mag: Rc<RefCell<Vec<i32>>> = _t0;
            let mut len = (mag.borrow().len() as i32);
            let mut d = mag.borrow()[0i32 as usize];
            if len == 2i32 {
                return Ok(BigDecimal::new_bigint_l_i_i(Clone::clone(&BigInteger::new_arr_i_i(Clone::clone(&mag), sign)?), -9223372036854775808i64, scale, 0i32)?);
            }
            let mut v = (if len == 2i32 { (((mag.borrow()[1i32 as usize] as i64)&(4294967295i64))|((((d as i64)&(4294967295i64))).wrapping_shl((32i32&0x3f) as u32))) } else { ((d as i64)&(4294967295i64)) });
            let _t1: BigDecimal = BigDecimal::valueOf_l_i((if sign == -1i32 { (v).wrapping_neg() } else { v }), scale)?;
            Ok(_t1)
        }

        #[java_method(name = "toCompactValue", descriptor = "(I)J", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toCompactValue(&self, mut sign: i32) -> Result<i64> {
            let this = self;
            if (sign==0) {
                return Ok(0i64);
            }
            let _t0 = this.getMagnitudeArray()?;
            let mut mag: Rc<RefCell<Vec<i32>>> = _t0;
            let mut len = (mag.borrow().len() as i32);
            let mut d = mag.borrow()[0i32 as usize];
            if len == 2i32 {
                return Ok(-9223372036854775808i64);
            }
            let mut v = (if len == 2i32 { (((mag.borrow()[1i32 as usize] as i64)&(4294967295i64))|((((d as i64)&(4294967295i64))).wrapping_shl((32i32&0x3f) as u32))) } else { ((d as i64)&(4294967295i64)) });
            Ok((if sign == -1i32 { (v).wrapping_neg() } else { v }))
        }

        #[java_method(name = "clear", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<()> {
            let this = self;
            this.__set_intLen(0i32);
            this.__set_offset(0i32);
            let mut index: i32 = 0i32;
            let mut n = (this.__get_value().borrow().len() as i32);
            loop {
                if index >= n { break; }
                this.__get_value().borrow_mut()[index as usize] = 0i32;
                index = index.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "reset", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reset(&self) -> Result<()> {
            let this = self;
            this.__set_intLen(0i32);
            this.__set_offset(0i32);
            Ok(())
        }

        #[java_method(name = "compare", descriptor = "(Ljava/math/MutableBigInteger;)I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare(&self, mut b: MutableBigInteger) -> Result<i32> {
            let this = self;
            let mut blen = b.__get_intLen();
            if this.__get_intLen() < blen {
                return Ok(-1i32);
            }
            if this.__get_intLen() > blen {
                return Ok(1i32);
            }
            let mut bval = b.__get_value();
            let mut i = this.__get_offset();
            let mut j = b.__get_offset();
            loop {
                if i >= (this.__get_intLen()).wrapping_add(this.__get_offset()) { break; }
                let mut b1 = (this.__get_value().borrow()[i as usize]).wrapping_add(-2147483648i32);
                let mut b2 = (bval.borrow()[j as usize]).wrapping_add(-2147483648i32);
                if b1 < b2 {
                    return Ok(-1i32);
                }
                if b1 > b2 {
                    return Ok(1i32);
                }
                i = i.wrapping_add(1i32);
                j = j.wrapping_add(1i32);
            }
            Ok(0i32)
        }

        #[java_method(name = "compareShifted", descriptor = "(Ljava/math/MutableBigInteger;I)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareShifted(&self, mut b: MutableBigInteger, mut ints: i32) -> Result<i32> {
            let this = self;
            let mut blen = b.__get_intLen();
            let mut alen = (this.__get_intLen()).wrapping_sub(ints);
            if alen < blen {
                return Ok(-1i32);
            }
            if alen > blen {
                return Ok(1i32);
            }
            let mut bval = b.__get_value();
            let mut i = this.__get_offset();
            let mut j = b.__get_offset();
            loop {
                if i >= (alen).wrapping_add(this.__get_offset()) { break; }
                let mut b1 = (this.__get_value().borrow()[i as usize]).wrapping_add(-2147483648i32);
                let mut b2 = (bval.borrow()[j as usize]).wrapping_add(-2147483648i32);
                if b1 < b2 {
                    return Ok(-1i32);
                }
                if b1 > b2 {
                    return Ok(1i32);
                }
                i = i.wrapping_add(1i32);
                j = j.wrapping_add(1i32);
            }
            Ok(0i32)
        }

        #[java_method(name = "compareHalf", descriptor = "(Ljava/math/MutableBigInteger;)I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareHalf(&self, mut b: MutableBigInteger) -> Result<i32> {
            let this = self;
            let mut blen = b.__get_intLen();
            let mut len = this.__get_intLen();
            return Ok(((blen>0)) as i32);
            if len > blen {
                return Ok(1i32);
            }
            if len < (blen).wrapping_sub(1i32) {
                return Ok(-1i32);
            }
            let mut bval = b.__get_value();
            let mut bstart: i32 = 0i32;
            let mut carry: i32 = 0i32;
            if bval.borrow()[bstart as usize] == 1i32 {
                bstart = bstart.wrapping_add(1i32);
                carry = -2147483648i32;
            } else {
                return Ok(-1i32);
            }
            let mut val = this.__get_value();
            let mut i = this.__get_offset();
            let mut j: i32 = bstart;
            loop {
                if i >= (len).wrapping_add(this.__get_offset()) { break; }
                j = j.wrapping_add(1i32);
                let mut bv = bval.borrow()[j as usize];
                let mut hb = (((((bv as u32>>(1i32&0x1f)) as i32)).wrapping_add(carry) as i64)&(4294967295i64));
                i = i.wrapping_add(1i32);
                let mut v = ((val.borrow()[i as usize] as i64)&(4294967295i64));
                return Ok(((((v>(hb)) as i32-((v)<(hb)) as i32)>=0)) as i32);
                carry = ((bv&1i32)<<(31i32&0x1f));
            }
            Ok(((carry!=0)) as i32)
        }

        #[java_method(name = "getLowestSetBit", descriptor = "()I", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLowestSetBit(&self) -> Result<i32> {
            let this = self;
            if (this.__get_intLen()==0) {
                return Ok(-1i32);
            }
            let mut j = (this.__get_intLen()).wrapping_sub(1i32);
            loop {
                if (j<=0) { break; }
                if (this.__get_value().borrow()[(j).wrapping_add(this.__get_offset()) as usize]==0) {
                    j = j.wrapping_sub(1i32);
                    continue;
                }
                break;
            }
            let mut b = this.__get_value().borrow()[(j).wrapping_add(this.__get_offset()) as usize];
            if (b==0) {
                return Ok(-1i32);
            }
            let _t0: i32 = Integer::numberOfTrailingZeros(b)?;
            Ok(((((this.__get_intLen()).wrapping_sub(1i32)).wrapping_sub(j)<<(5i32&0x1f))).wrapping_add(_t0))
        }

        #[java_method(name = "getInt", descriptor = "(I)I", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInt(&self, index: i32) -> Result<i32> {
            panic!("stub: java/math/MutableBigInteger.getInt:(I)I")
        }

        #[java_method(name = "getLong", descriptor = "(I)J", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLong(&self, index: i32) -> Result<i64> {
            panic!("stub: java/math/MutableBigInteger.getLong:(I)J")
        }

        #[java_method(name = "normalize", descriptor = "()V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn normalize(&self) -> Result<()> {
            let this = self;
            if (this.__get_intLen()==0) {
                this.__set_offset(0i32);
                return Ok(());
            }
            let mut index = this.__get_offset();
            if (this.__get_value().borrow()[index as usize]!=0) {
                return Ok(());
            }
            let mut indexBound = (index).wrapping_add(this.__get_intLen());
            loop {
                index = index.wrapping_add(1i32);
                if (this.__get_value().borrow()[index as usize]!=0) { break; }
            }
            let mut numZeros = (index).wrapping_sub(this.__get_offset());
            this.__set_intLen((this.__get_intLen()).wrapping_sub(numZeros));
            this.__set_offset((if (this.__get_intLen()==0) { 0i32 } else { (this.__get_offset()).wrapping_add(numZeros) }));
            Ok(())
        }

        #[java_method(name = "ensureCapacity", descriptor = "(I)V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ensureCapacity(&self, len: i32) -> Result<()> {
            panic!("stub: java/math/MutableBigInteger.ensureCapacity:(I)V")
        }

        #[java_method(name = "toIntArray", descriptor = "()[I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toIntArray(&self) -> Result<Rc<RefCell<Vec<i32>>>> {
            panic!("stub: java/math/MutableBigInteger.toIntArray:()[I")
        }

        #[java_method(name = "setInt", descriptor = "(II)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setInt(&self, index: i32, val: i32) -> Result<()> {
            panic!("stub: java/math/MutableBigInteger.setInt:(II)V")
        }

        #[java_method(name = "setValue", descriptor = "([II)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setValue(&self, mut val: Rc<RefCell<Vec<i32>>>, mut length: i32) -> Result<()> {
            let this = self;
            this.__set_value(Clone::clone(&val));
            this.__set_intLen(length);
            this.__set_offset(0i32);
            Ok(())
        }

        #[java_method(name = "copyValue", descriptor = "(Ljava/math/MutableBigInteger;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyValue_mutabl(&self, src: MutableBigInteger) -> Result<()> {
            panic!("stub: java/math/MutableBigInteger.copyValue:(Ljava/math/MutableBigInteger;)V")
        }

        #[java_method(name = "copyValue", descriptor = "([I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyValue_arr_i(&self, val: Rc<RefCell<Vec<i32>>>) -> Result<()> {
            panic!("stub: java/math/MutableBigInteger.copyValue:([I)V")
        }

        #[java_method(name = "isOne", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isOne(&self) -> Result<bool> {
            panic!("stub: java/math/MutableBigInteger.isOne:()Z")
        }

        #[java_method(name = "isZero", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isZero(&self) -> Result<bool> {
            let this = self;
            Ok((this.__get_intLen()==0))
        }

        #[java_method(name = "isEven", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEven(&self) -> Result<bool> {
            panic!("stub: java/math/MutableBigInteger.isEven:()Z")
        }

        #[java_method(name = "isOdd", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isOdd(&self) -> Result<bool> {
            let this = self;
            let _t0 = this.isZero()?;
            Ok((if _t0 { (0i32 != 0) } else { (this.__get_value().borrow()[((this.__get_offset()).wrapping_add(this.__get_intLen())).wrapping_sub(1i32) as usize]&1i32) == 1i32 }))
        }

        #[java_method(name = "isNormal", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isNormal(&self) -> Result<bool> {
            panic!("stub: java/math/MutableBigInteger.isNormal:()Z")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "safeRightShift", descriptor = "(I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn safeRightShift(&self, mut n: i32) -> Result<()> {
            let this = self;
            if (n/32i32) >= this.__get_intLen() {
                this.reset()?;
            } else {
                this.rightShift(n)?;
            }
            Ok(())
        }

        #[java_method(name = "rightShift", descriptor = "(I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn rightShift(&self, mut n: i32) -> Result<()> {
            let this = self;
            if (this.__get_intLen()==0) {
                return Ok(());
            }
            let mut nInts = ((n as u32>>(5i32&0x1f)) as i32);
            let mut nBits = (n&31i32);
            this.__set_intLen((this.__get_intLen()).wrapping_sub(nInts));
            if (nBits==0) {
                return Ok(());
            }
            let _t0: i32 = BigInteger::bitLengthForInt(this.__get_value().borrow()[this.__get_offset() as usize])?;
            let mut bitsInHighWord: i32 = _t0;
            if nBits >= bitsInHighWord {
                this.primitiveLeftShift((32i32).wrapping_sub(nBits))?;
                this.__set_intLen((this.__get_intLen()).wrapping_sub(1i32));
            } else {
                this.primitiveRightShift(nBits)?;
            }
            Ok(())
        }

        #[java_method(name = "safeLeftShift", descriptor = "(I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn safeLeftShift(&self, mut n: i32) -> Result<()> {
            let this = self;
            if (n>0) {
                this.leftShift(n)?;
            }
            Ok(())
        }

        #[java_method(name = "leftShift", descriptor = "(I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn leftShift(&self, mut n: i32) -> Result<()> {
            let this = self;
            if (this.__get_intLen()==0) {
                return Ok(());
            }
            let mut nInts = ((n as u32>>(5i32&0x1f)) as i32);
            let mut nBits = (n&31i32);
            let _t0: i32 = BigInteger::bitLengthForInt(this.__get_value().borrow()[this.__get_offset() as usize])?;
            let mut bitsInHighWord: i32 = _t0;
            if n <= (32i32).wrapping_sub(bitsInHighWord) {
                this.primitiveLeftShift(nBits)?;
                return Ok(());
            }
            let mut newLen = ((this.__get_intLen()).wrapping_add(nInts)).wrapping_add(1i32);
            if nBits <= (32i32).wrapping_sub(bitsInHighWord) {
                newLen = newLen.wrapping_sub(1i32);
            }
            if (this.__get_value().borrow().len() as i32) < newLen {
                let mut _arr1: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; newLen as usize]));
                let mut result: Rc<RefCell<Vec<i32>>> = _arr1;
                let mut i: i32 = 0i32;
                loop {
                    if i >= this.__get_intLen() { break; }
                    result.borrow_mut()[i as usize] = this.__get_value().borrow()[(this.__get_offset()).wrapping_add(i) as usize];
                    i = i.wrapping_add(1i32);
                }
                this.setValue(Clone::clone(&result), newLen)?;
            } else {
                if ((this.__get_value().borrow().len() as i32)).wrapping_sub(this.__get_offset()) >= newLen {
                    let mut result: i32 = 0i32;
                    loop {
                        if result >= (newLen).wrapping_sub(this.__get_intLen()) { break; }
                        this.__get_value().borrow_mut()[((this.__get_offset()).wrapping_add(this.__get_intLen())).wrapping_add(result) as usize] = 0i32;
                        result = result.wrapping_add(1i32);
                    }
                } else {
                    let mut result: i32 = 0i32;
                    loop {
                        if result >= this.__get_intLen() { break; }
                        let _tmp_val1 = this.__get_value().borrow()[(this.__get_offset()).wrapping_add(result) as usize];
                        this.__get_value().borrow_mut()[result as usize] = _tmp_val1;
                        result = result.wrapping_add(1i32);
                    }
                    result = this.__get_intLen();
                    loop {
                        if result >= newLen { break; }
                        this.__get_value().borrow_mut()[result as usize] = 0i32;
                        result = result.wrapping_add(1i32);
                    }
                    this.__set_offset(0i32);
                }
            }
            this.__set_intLen(newLen);
            if (nBits==0) {
                return Ok(());
            }
            if nBits <= (32i32).wrapping_sub(bitsInHighWord) {
                this.primitiveLeftShift(nBits)?;
            } else {
                this.primitiveRightShift((32i32).wrapping_sub(nBits))?;
            }
            Ok(())
        }

        #[java_method(name = "divadd", descriptor = "([I[II)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divadd(&self, mut a: Rc<RefCell<Vec<i32>>>, mut result: Rc<RefCell<Vec<i32>>>, mut offset: i32) -> Result<i32> {
            let this = self;
            let mut carry: i64 = 0i64;
            let mut j = ((a.borrow().len() as i32)).wrapping_sub(1i32);
            loop {
                if (j<0) { break; }
                let mut sum = ((((a.borrow()[j as usize] as i64)&(4294967295i64))).wrapping_add(((result.borrow()[(j).wrapping_add(offset) as usize] as i64)&(4294967295i64)))).wrapping_add(carry);
                result.borrow_mut()[(j).wrapping_add(offset) as usize] = (sum as i32);
                carry = ((sum as u64).wrapping_shr((32i32&0x3f) as u32) as i64);
                j = j.wrapping_sub(1i32);
            }
            Ok((carry as i32))
        }

        #[java_method(name = "mulsub", descriptor = "([I[IIII)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mulsub(&self, mut q: Rc<RefCell<Vec<i32>>>, mut a: Rc<RefCell<Vec<i32>>>, mut x: i32, mut len: i32, mut offset: i32) -> Result<i32> {
            let this = self;
            let mut xLong = ((x as i64)&(4294967295i64));
            let mut carry: i64 = 0i64;
            offset = (offset).wrapping_add(len);
            let mut j = (len).wrapping_sub(1i32);
            loop {
                if (j<0) { break; }
                let mut product = ((((a.borrow()[j as usize] as i64)&(4294967295i64))).wrapping_mul(xLong)).wrapping_add(carry);
                let mut difference = ((q.borrow()[offset as usize] as i64)).wrapping_sub(product);
                offset = offset.wrapping_sub(1i32);
                q.borrow_mut()[offset as usize] = (difference as i32);
                carry = (((product as u64).wrapping_shr((32i32&0x3f) as u32) as i64)).wrapping_add((((((difference&(4294967295i64))>(((((product as i32)^-1i32) as i64)&(4294967295i64)))) as i32-(((difference&(4294967295i64)))<(((((product as i32)^-1i32) as i64)&(4294967295i64)))) as i32)>0) as i64));
                j = j.wrapping_sub(1i32);
            }
            Ok((carry as i32))
        }

        #[java_method(name = "mulsubBorrow", descriptor = "([I[IIII)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mulsubBorrow(&self, mut q: Rc<RefCell<Vec<i32>>>, mut a: Rc<RefCell<Vec<i32>>>, mut x: i32, mut len: i32, mut offset: i32) -> Result<i32> {
            let this = self;
            let mut xLong = ((x as i64)&(4294967295i64));
            let mut carry: i64 = 0i64;
            offset = (offset).wrapping_add(len);
            let mut j = (len).wrapping_sub(1i32);
            loop {
                if (j<0) { break; }
                let mut product = ((((a.borrow()[j as usize] as i64)&(4294967295i64))).wrapping_mul(xLong)).wrapping_add(carry);
                offset = offset.wrapping_sub(1i32);
                let mut difference = ((q.borrow()[offset as usize] as i64)).wrapping_sub(product);
                carry = (((product as u64).wrapping_shr((32i32&0x3f) as u32) as i64)).wrapping_add((((((difference&(4294967295i64))>(((((product as i32)^-1i32) as i64)&(4294967295i64)))) as i32-(((difference&(4294967295i64)))<(((((product as i32)^-1i32) as i64)&(4294967295i64)))) as i32)>0) as i64));
                j = j.wrapping_sub(1i32);
            }
            Ok((carry as i32))
        }

        #[java_method(name = "primitiveRightShift", descriptor = "(I)V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn primitiveRightShift(&self, mut n: i32) -> Result<()> {
            let this = self;
            let mut val = this.__get_value();
            let mut n2 = (32i32).wrapping_sub(n);
            let mut i = ((this.__get_offset()).wrapping_add(this.__get_intLen())).wrapping_sub(1i32);
            let mut c = val.borrow()[i as usize];
            loop {
                if i <= this.__get_offset() { break; }
                let mut b: i32 = c;
                c = val.borrow()[(i).wrapping_sub(1i32) as usize];
                val.borrow_mut()[i as usize] = ((c<<(n2&0x1f))|((b as u32>>(n&0x1f)) as i32));
                i = i.wrapping_sub(1i32);
            }
            let _tmp_val0 = ((val.borrow()[this.__get_offset() as usize] as u32>>(n&0x1f)) as i32);
            val.borrow_mut()[this.__get_offset() as usize] = _tmp_val0;
            Ok(())
        }

        #[java_method(name = "primitiveLeftShift", descriptor = "(I)V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn primitiveLeftShift(&self, mut n: i32) -> Result<()> {
            let this = self;
            let mut val = this.__get_value();
            let mut n2 = (32i32).wrapping_sub(n);
            let mut i = this.__get_offset();
            let mut c = val.borrow()[i as usize];
            let mut m = ((i).wrapping_add(this.__get_intLen())).wrapping_sub(1i32);
            loop {
                if i >= m { break; }
                let mut b: i32 = c;
                c = val.borrow()[(i).wrapping_add(1i32) as usize];
                val.borrow_mut()[i as usize] = ((b<<(n&0x1f))|((c as u32>>(n2&0x1f)) as i32));
                i = i.wrapping_add(1i32);
            }
            let _tmp_val0 = (val.borrow()[((this.__get_offset()).wrapping_add(this.__get_intLen())).wrapping_sub(1i32) as usize]<<(n&0x1f));
            val.borrow_mut()[((this.__get_offset()).wrapping_add(this.__get_intLen())).wrapping_sub(1i32) as usize] = _tmp_val0;
            Ok(())
        }

        #[java_method(name = "getLower", descriptor = "(I)Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLower(&self, mut n: i32) -> Result<BigInteger> {
            let this = self;
            let _t0 = this.isZero()?;
            if _t0 {
                return Ok(BigInteger::ZERO());
            }
            if this.__get_intLen() < n {
                let _t1 = this.toBigInteger_i(1i32)?;
                return Ok(_t1);
            }
            let mut len: i32 = n;
            loop {
                if (len<=0) { break; }
                if (this.__get_value().borrow()[((this.__get_offset()).wrapping_add(this.__get_intLen())).wrapping_sub(len) as usize]==0) {
                    len = len.wrapping_sub(1i32);
                    continue;
                }
                break;
            }
            let mut sign = ((len>0)) as i32;
            let _t1: Rc<RefCell<Vec<i32>>> = Arrays::copyOfRange_arr_i_i_i(Clone::clone(&this.__get_value()), ((this.__get_offset()).wrapping_add(this.__get_intLen())).wrapping_sub(len), (this.__get_offset()).wrapping_add(this.__get_intLen()))?;
            Ok(BigInteger::new_arr_i_i(Clone::clone(&_t1), sign)?)
        }

        #[java_method(name = "keepLower", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn keepLower(&self, mut n: i32) -> Result<()> {
            let this = self;
            if this.__get_intLen() >= n {
                this.__set_offset((this.__get_offset()).wrapping_add((this.__get_intLen()).wrapping_sub(n)));
                this.__set_intLen(n);
            }
            Ok(())
        }

        #[java_method(name = "add", descriptor = "(Ljava/math/MutableBigInteger;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn add(&self, mut addend: MutableBigInteger) -> Result<()> {
            let this = self;
            let mut x = this.__get_intLen();
            let mut y = addend.__get_intLen();
            let mut resultLen = (if this.__get_intLen() > addend.__get_intLen() { this.__get_intLen() } else { addend.__get_intLen() });
            let mut _merged1: Rc<RefCell<Vec<i32>>>;
            if (this.__get_value().borrow().len() as i32) < resultLen {
                let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; resultLen as usize]));
                _merged1 = _arr0;
            } else {
                _merged1 = this.__get_value();
            }
            let mut result: Rc<RefCell<Vec<i32>>> = _merged1;
            let mut rstart = ((result.borrow().len() as i32)).wrapping_sub(1i32);
            let mut carry: i64 = 0i64;
        let mut sum = Default::default();
            loop {
                if (x<=0) { break; }
        sum = Default::default();
                if (y>0) {
                    x = x.wrapping_sub(1i32);
                    y = y.wrapping_sub(1i32);
                    sum = ((((this.__get_value().borrow()[(x).wrapping_add(this.__get_offset()) as usize] as i64)&(4294967295i64))).wrapping_add(((addend.__get_value().borrow()[(y).wrapping_add(addend.__get_offset()) as usize] as i64)&(4294967295i64)))).wrapping_add(carry);
                    rstart = rstart.wrapping_sub(1i32);
                    result.borrow_mut()[rstart as usize] = (sum as i32);
                    carry = ((sum as u64).wrapping_shr((32i32&0x3f) as u32) as i64);
                    continue;
                }
                break;
            }
            loop {
                if (x<=0) { break; }
                x = x.wrapping_sub(1i32);
                if rstart == (x).wrapping_add(this.__get_offset()) {
                    return Ok(());
                }
                sum = (((this.__get_value().borrow()[(x).wrapping_add(this.__get_offset()) as usize] as i64)&(4294967295i64))).wrapping_add(carry);
                rstart = rstart.wrapping_sub(1i32);
                result.borrow_mut()[rstart as usize] = (sum as i32);
                carry = ((sum as u64).wrapping_shr((32i32&0x3f) as u32) as i64);
            }
            loop {
                if (y<=0) { break; }
                y = y.wrapping_sub(1i32);
                sum = (((addend.__get_value().borrow()[(y).wrapping_add(addend.__get_offset()) as usize] as i64)&(4294967295i64))).wrapping_add(carry);
                rstart = rstart.wrapping_sub(1i32);
                result.borrow_mut()[rstart as usize] = (sum as i32);
                carry = ((sum as u64).wrapping_shr((32i32&0x3f) as u32) as i64);
            }
            resultLen = resultLen.wrapping_add(1i32);
            if (result.borrow().len() as i32) < resultLen {
                let mut _arr2: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; resultLen as usize]));
                let mut temp: Rc<RefCell<Vec<i32>>> = _arr2;
                System::arraycopy(Object::from_any(result.clone()), 0i32, Object::from_any(temp.clone()), 1i32, (result.borrow().len() as i32))?;
                temp.borrow_mut()[0i32 as usize] = 1i32;
                result = temp;
            } else {
                rstart = rstart.wrapping_sub(1i32);
                result.borrow_mut()[rstart as usize] = 1i32;
            }
            this.__set_value(Clone::clone(&result));
            this.__set_intLen(resultLen);
            this.__set_offset(((result.borrow().len() as i32)).wrapping_sub(resultLen));
            Ok(())
        }

        #[java_method(name = "addShifted", descriptor = "(Ljava/math/MutableBigInteger;I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addShifted(&self, mut addend: MutableBigInteger, mut n: i32) -> Result<()> {
            let this = self;
            let _t0 = addend.isZero()?;
            if _t0 {
                return Ok(());
            }
            let mut x = this.__get_intLen();
            let mut y = (addend.__get_intLen()).wrapping_add(n);
            let mut resultLen = (if this.__get_intLen() > y { this.__get_intLen() } else { y });
            let mut _merged2: Rc<RefCell<Vec<i32>>>;
            if (this.__get_value().borrow().len() as i32) < resultLen {
                let mut _arr1: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; resultLen as usize]));
                _merged2 = _arr1;
            } else {
                _merged2 = this.__get_value();
            }
            let mut result: Rc<RefCell<Vec<i32>>> = _merged2;
            let mut rstart = ((result.borrow().len() as i32)).wrapping_sub(1i32);
            let mut carry: i64 = 0i64;
            let mut sum = Default::default();
            let mut bval = Default::default();
            loop {
                if (x<=0) { break; }
                x = x.wrapping_sub(1i32);
                y = y.wrapping_sub(1i32);
                bval = (if (y).wrapping_add(addend.__get_offset()) < (addend.__get_value().borrow().len() as i32) { addend.__get_value().borrow()[(y).wrapping_add(addend.__get_offset()) as usize] } else { 0i32 });
                sum = ((((this.__get_value().borrow()[(x).wrapping_add(this.__get_offset()) as usize] as i64)&(4294967295i64))).wrapping_add(((bval as i64)&(4294967295i64)))).wrapping_add(carry);
                rstart = rstart.wrapping_sub(1i32);
                result.borrow_mut()[rstart as usize] = (sum as i32);
                carry = ((sum as u64).wrapping_shr((32i32&0x3f) as u32) as i64);
            }
            loop {
                if (x<=0) { break; }
                x = x.wrapping_sub(1i32);
                if rstart == (x).wrapping_add(this.__get_offset()) {
                    return Ok(());
                }
                sum = (((this.__get_value().borrow()[(x).wrapping_add(this.__get_offset()) as usize] as i64)&(4294967295i64))).wrapping_add(carry);
                rstart = rstart.wrapping_sub(1i32);
                result.borrow_mut()[rstart as usize] = (sum as i32);
                carry = ((sum as u64).wrapping_shr((32i32&0x3f) as u32) as i64);
            }
            loop {
                if (y<=0) { break; }
                y = y.wrapping_sub(1i32);
                bval = (if (y).wrapping_add(addend.__get_offset()) < (addend.__get_value().borrow().len() as i32) { addend.__get_value().borrow()[(y).wrapping_add(addend.__get_offset()) as usize] } else { 0i32 });
                sum = (((bval as i64)&(4294967295i64))).wrapping_add(carry);
                rstart = rstart.wrapping_sub(1i32);
                result.borrow_mut()[rstart as usize] = (sum as i32);
                carry = ((sum as u64).wrapping_shr((32i32&0x3f) as u32) as i64);
            }
            resultLen = resultLen.wrapping_add(1i32);
            if (result.borrow().len() as i32) < resultLen {
                let mut _arr3: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; resultLen as usize]));
                let mut bval: Rc<RefCell<Vec<i32>>> = _arr3;
                System::arraycopy(Object::from_any(result.clone()), 0i32, Object::from_any(bval.clone()), 1i32, (result.borrow().len() as i32))?;
                bval.borrow_mut()[0i32 as usize] = 1i32;
                result = bval;
            } else {
                rstart = rstart.wrapping_sub(1i32);
                result.borrow_mut()[rstart as usize] = 1i32;
            }
            this.__set_value(Clone::clone(&result));
            this.__set_intLen(resultLen);
            this.__set_offset(((result.borrow().len() as i32)).wrapping_sub(resultLen));
            Ok(())
        }

        #[java_method(name = "addDisjoint", descriptor = "(Ljava/math/MutableBigInteger;I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addDisjoint(&self, mut addend: MutableBigInteger, mut n: i32) -> Result<()> {
            let this = self;
            let _t0 = addend.isZero()?;
            if _t0 {
                return Ok(());
            }
            let mut x = this.__get_intLen();
            let mut y = (addend.__get_intLen()).wrapping_add(n);
            let mut resultLen = (if this.__get_intLen() > y { this.__get_intLen() } else { y });
        let mut result: Rc<RefCell<Vec<i32>>> = Default::default();
            if (this.__get_value().borrow().len() as i32) < resultLen {
                let mut _arr1: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; resultLen as usize]));
                result = _arr1;
            } else {
                result = this.__get_value();
                Arrays::fill_arr_i_i_i_i(Clone::clone(&this.__get_value()), (this.__get_offset()).wrapping_add(this.__get_intLen()), (this.__get_value().borrow().len() as i32), 0i32)?;
            }
            let mut rstart = ((result.borrow().len() as i32)).wrapping_sub(1i32);
            System::arraycopy(Object::from_any(this.__get_value().clone()), this.__get_offset(), Object::from_any(result.clone()), ((rstart).wrapping_add(1i32)).wrapping_sub(x), x)?;
            y = (y).wrapping_sub(x);
            rstart = (rstart).wrapping_sub(x);
            let _t1: i32 = Math::min_i_i(y, ((addend.__get_value().borrow().len() as i32)).wrapping_sub(addend.__get_offset()))?;
            let mut len: i32 = _t1;
            System::arraycopy(Object::from_any(addend.__get_value().clone()), addend.__get_offset(), Object::from_any(result.clone()), ((rstart).wrapping_add(1i32)).wrapping_sub(y), len)?;
            let mut i = (((rstart).wrapping_add(1i32)).wrapping_sub(y)).wrapping_add(len);
            loop {
                if i >= (rstart).wrapping_add(1i32) { break; }
                result.borrow_mut()[i as usize] = 0i32;
                i = i.wrapping_add(1i32);
            }
            this.__set_value(Clone::clone(&result));
            this.__set_intLen(resultLen);
            this.__set_offset(((result.borrow().len() as i32)).wrapping_sub(resultLen));
            Ok(())
        }

        #[java_method(name = "addLower", descriptor = "(Ljava/math/MutableBigInteger;I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addLower(&self, mut addend: MutableBigInteger, mut n: i32) -> Result<()> {
            let this = self;
            let mut a = MutableBigInteger::new_mutabl(Clone::clone(&addend))?;
            if (a.__get_offset()).wrapping_add(a.__get_intLen()) >= n {
                a.__set_offset(((a.__get_offset()).wrapping_add(a.__get_intLen())).wrapping_sub(n));
                a.__set_intLen(n);
            }
            a.normalize()?;
            this.add(Clone::clone(&a))?;
            Ok(())
        }

        #[java_method(name = "subtract", descriptor = "(Ljava/math/MutableBigInteger;)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn subtract(&self, mut b: MutableBigInteger) -> Result<i32> {
            let this = self;
            let mut a = this.clone();
            let mut result = this.__get_value();
            let _t0 = a.compare(Clone::clone(&b))?;
            let mut sign: i32 = _t0;
            if (sign==0) {
                this.reset()?;
                return Ok(0i32);
            }
            if (sign<0) {
                let mut tmp: MutableBigInteger = a;
                a = b;
                b = tmp;
            }
            let mut tmp = a.__get_intLen();
            if (result.borrow().len() as i32) < tmp {
                let mut _arr1: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; tmp as usize]));
                result = _arr1;
            }
            let mut diff: i64 = 0i64;
            let mut x = a.__get_intLen();
            let mut y = b.__get_intLen();
            let mut rstart = ((result.borrow().len() as i32)).wrapping_sub(1i32);
            loop {
                if (y<=0) { break; }
                x = x.wrapping_sub(1i32);
                y = y.wrapping_sub(1i32);
                diff = ((((a.__get_value().borrow()[(x).wrapping_add(a.__get_offset()) as usize] as i64)&(4294967295i64))).wrapping_sub(((b.__get_value().borrow()[(y).wrapping_add(b.__get_offset()) as usize] as i64)&(4294967295i64)))).wrapping_add((diff).wrapping_shr((32i32&0x3f) as u32));
                rstart = rstart.wrapping_sub(1i32);
                result.borrow_mut()[rstart as usize] = (diff as i32);
            }
            loop {
                if (x<=0) { break; }
                x = x.wrapping_sub(1i32);
                diff = (((a.__get_value().borrow()[(x).wrapping_add(a.__get_offset()) as usize] as i64)&(4294967295i64))).wrapping_add((diff).wrapping_shr((32i32&0x3f) as u32));
                rstart = rstart.wrapping_sub(1i32);
                result.borrow_mut()[rstart as usize] = (diff as i32);
            }
            this.__set_value(Clone::clone(&result));
            this.__set_intLen(tmp);
            this.__set_offset(((this.__get_value().borrow().len() as i32)).wrapping_sub(tmp));
            this.normalize()?;
            Ok(sign)
        }

        #[java_method(name = "difference", descriptor = "(Ljava/math/MutableBigInteger;)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn difference(&self, b: MutableBigInteger) -> Result<i32> {
            panic!("stub: java/math/MutableBigInteger.difference:(Ljava/math/MutableBigInteger;)I")
        }

        #[java_method(name = "multiply", descriptor = "(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiply(&self, y: MutableBigInteger, z: MutableBigInteger) -> Result<()> {
            panic!("stub: java/math/MutableBigInteger.multiply:(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)V")
        }

        #[java_method(name = "mul", descriptor = "(ILjava/math/MutableBigInteger;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mul(&self, y: i32, z: MutableBigInteger) -> Result<()> {
            panic!("stub: java/math/MutableBigInteger.mul:(ILjava/math/MutableBigInteger;)V")
        }

        #[java_method(name = "divideOneWord", descriptor = "(ILjava/math/MutableBigInteger;)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideOneWord(&self, mut divisor: i32, mut quotient: MutableBigInteger) -> Result<i32> {
            let this = self;
            let mut divisorLong = ((divisor as i64)&(4294967295i64));
            let mut dividendValue = ((this.__get_value().borrow()[this.__get_offset() as usize] as i64)&(4294967295i64));
            let mut q: i32 = ((dividendValue/divisorLong) as i32);
            let mut r: i32 = ((dividendValue).wrapping_sub(((q as i64)).wrapping_mul(divisorLong)) as i32);
            quotient.__get_value().borrow_mut()[0i32 as usize] = q;
            quotient.__set_intLen(((q!=0)) as i32);
            quotient.__set_offset(0i32);
            return Ok(r);
            if (quotient.__get_value().borrow().len() as i32) < this.__get_intLen() {
                let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; this.__get_intLen() as usize]));
                quotient.__set_value(Clone::clone(&_arr0));
            }
            quotient.__set_offset(0i32);
            quotient.__set_intLen(this.__get_intLen());
            let _t0: i32 = Integer::numberOfLeadingZeros(divisor)?;
            let mut dividendValue: i32 = _t0;
            let mut rem = this.__get_value().borrow()[this.__get_offset() as usize];
            let mut q = ((rem as i64)&(4294967295i64));
            if (((q>(divisorLong)) as i32-((q)<(divisorLong)) as i32)<0) {
                quotient.__get_value().borrow_mut()[0i32 as usize] = 0i32;
            } else {
                quotient.__get_value().borrow_mut()[0i32 as usize] = ((q/divisorLong) as i32);
                rem = ((q).wrapping_sub(((quotient.__get_value().borrow()[0i32 as usize] as i64)).wrapping_mul(divisorLong)) as i32);
                q = ((rem as i64)&(4294967295i64));
            }
            let mut xlen = this.__get_intLen();
            let mut q: i32 = Default::default();
            loop {
                xlen = xlen.wrapping_sub(1i32);
                if (xlen<=0) { break; }
                let mut dividendEstimate = ((q).wrapping_shl((32i32&0x3f) as u32)|(((this.__get_value().borrow()[((this.__get_offset()).wrapping_add(this.__get_intLen())).wrapping_sub(xlen) as usize] as i64)&(4294967295i64))));
                if (((dividendEstimate>(0i64)) as i32-((dividendEstimate)<(0i64)) as i32)>=0) {
                    q = ((dividendEstimate/divisorLong) as i32);
                    rem = ((dividendEstimate).wrapping_sub(((q as i64)).wrapping_mul(divisorLong)) as i32);
                } else {
                    let _t1: i64 = MutableBigInteger::divWord(dividendEstimate, divisor)?;
                    let mut tmp: i64 = _t1;
                    let mut q: i32 = ((tmp&(4294967295i64)) as i32);
                    rem = (((tmp as u64).wrapping_shr((32i32&0x3f) as u32) as i64) as i32);
                }
                quotient.__get_value().borrow_mut()[(this.__get_intLen()).wrapping_sub(xlen) as usize] = q;
                let mut q = ((rem as i64)&(4294967295i64));
            }
            quotient.normalize()?;
            if (dividendValue>0) {
                return Ok((rem%divisor));
            }
            Ok(rem)
        }

        #[java_method(name = "divide", descriptor = "(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: divide(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;
        pub fn divide_mutabl_mutabl(&self, mut b: MutableBigInteger, mut quotient: MutableBigInteger) -> Result<MutableBigInteger> {
            let this = self;
            let _t0 = this.divide_mutabl_mutabl_z(Clone::clone(&b), Clone::clone(&quotient), (1i32 != 0i32))?;
            Ok(_t0)
        }

        #[java_method(name = "divide", descriptor = "(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;Z)Ljava/math/MutableBigInteger;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: divide(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;Z)Ljava/math/MutableBigInteger;
        pub fn divide_mutabl_mutabl_z(&self, mut b: MutableBigInteger, mut quotient: MutableBigInteger, mut needRemainder: bool) -> Result<MutableBigInteger> {
            let this = self;
            if (this.__get_intLen()).wrapping_sub(b.__get_intLen()) < 40i32 {
                let _t0 = this.divideKnuth_mutabl_mutabl_z(Clone::clone(&b), Clone::clone(&quotient), needRemainder)?;
                return Ok(_t0);
            }
            let _t0 = this.divideAndRemainderBurnikelZiegler(Clone::clone(&b), Clone::clone(&quotient))?;
            Ok(_t0)
        }

        #[java_method(name = "divideKnuth", descriptor = "(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: divideKnuth(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;
        pub fn divideKnuth_mutabl_mutabl(&self, mut b: MutableBigInteger, mut quotient: MutableBigInteger) -> Result<MutableBigInteger> {
            let this = self;
            let _t0 = this.divideKnuth_mutabl_mutabl_z(Clone::clone(&b), Clone::clone(&quotient), (1i32 != 0i32))?;
            Ok(_t0)
        }

        #[java_method(name = "divideKnuth", descriptor = "(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;Z)Ljava/math/MutableBigInteger;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: divideKnuth(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;Z)Ljava/math/MutableBigInteger;
        pub fn divideKnuth_mutabl_mutabl_z(&self, mut b: MutableBigInteger, mut quotient: MutableBigInteger, mut needRemainder: bool) -> Result<MutableBigInteger> {
            let this = self;
            if (b.__get_intLen()==0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            quotient.__set_offset(0i32);
            quotient.__set_intLen(0i32);
            return Ok((if needRemainder { MutableBigInteger::new()? } else { Default::default() }));
            let _t0 = this.compare(Clone::clone(&b))?;
            let mut cmp: i32 = _t0;
            quotient.__set_offset(0i32);
            quotient.__set_intLen(0i32);
            return Ok((if needRemainder { MutableBigInteger::new_mutabl(Clone::clone(this))? } else { Default::default() }));
            quotient.__set_intLen(1i32);
            quotient.__get_value().borrow_mut()[0i32 as usize] = 1i32;
            quotient.__set_offset(0i32);
            return Ok((if needRemainder { MutableBigInteger::new()? } else { Default::default() }));
            quotient.clear()?;
            let _t1 = this.divideOneWord(b.__get_value().borrow()[b.__get_offset() as usize], Clone::clone(&quotient))?;
            let mut r: i32 = _t1;
            if (r==0) {
                return Ok(MutableBigInteger::new()?);
            }
            return Ok(MutableBigInteger::new_i(r)?);
            return Ok(Default::default());
            let _t2 = this.getLowestSetBit()?;
            let _t3 = b.getLowestSetBit()?;
            let _t4: i32 = Math::min_i_i(_t2, _t3)?;
            r = _t4;
            if r >= 96i32 {
                let mut a = MutableBigInteger::new_mutabl(Clone::clone(this))?;
                b = MutableBigInteger::new_mutabl(Clone::clone(&b))?;
                a.rightShift(r)?;
                b.rightShift(r)?;
                let _t5 = a.divideKnuth_mutabl_mutabl(Clone::clone(&b), Clone::clone(&quotient))?;
                let mut r: MutableBigInteger = _t5;
                r.leftShift(r)?;
                return Ok(r);
            }
            let _t5 = this.divideMagnitude(Clone::clone(&b), Clone::clone(&quotient), needRemainder)?;
            Ok(_t5)
        }

        #[java_method(name = "divideAndRemainderBurnikelZiegler", descriptor = "(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideAndRemainderBurnikelZiegler(&self, mut b: MutableBigInteger, mut quotient: MutableBigInteger) -> Result<MutableBigInteger> {
            let this = self;
            let mut r = this.__get_intLen();
            let mut s = b.__get_intLen();
            quotient.__set_intLen(0i32);
            quotient.__set_offset(0i32);
            if r < s {
                return Ok(Clone::clone(this));
            }
            let _t0: i32 = Integer::numberOfLeadingZeros((s/80i32))?;
            let mut m = (1i32<<((32i32).wrapping_sub(_t0)&0x1f));
            let mut j = (((s).wrapping_add(m)).wrapping_sub(1i32)/m);
            let mut n = (j).wrapping_mul(m);
            let mut n32 = (32i64).wrapping_mul((n as i64));
            let _t1 = b.bitLength()?;
            let _t2: i64 = Math::max_l_l(0i64, (n32).wrapping_sub(_t1))?;
            let mut sigma: i32 = (_t2 as i32);
            let mut bShifted = MutableBigInteger::new_mutabl(Clone::clone(&b))?;
            bShifted.safeLeftShift(sigma)?;
            let mut aShifted = MutableBigInteger::new_mutabl(Clone::clone(this))?;
            aShifted.safeLeftShift(sigma)?;
            let _t3 = aShifted.bitLength()?;
            let mut t: i32 = (((_t3).wrapping_add(n32)/n32) as i32);
            if t < 2i32 {
                t = 2i32;
            }
            let _t4 = aShifted.getBlock((t).wrapping_sub(1i32), t, n)?;
            let mut a1: MutableBigInteger = _t4;
            let _t5 = aShifted.getBlock((t).wrapping_sub(2i32), t, n)?;
            let mut z: MutableBigInteger = _t5;
            z.addDisjoint(Clone::clone(&a1), n)?;
            let mut qi = MutableBigInteger::new()?;
            let mut i = (t).wrapping_sub(2i32);
            loop {
                if (i<=0) { break; }
                let _t6 = z.divide2n1n(Clone::clone(&bShifted), Clone::clone(&qi))?;
                let mut ri: MutableBigInteger = _t6;
                let _t7 = aShifted.getBlock((i).wrapping_sub(1i32), t, n)?;
                z = _t7;
                z.addDisjoint(Clone::clone(&ri), n)?;
                quotient.addShifted(Clone::clone(&qi), (i).wrapping_mul(n))?;
                i = i.wrapping_sub(1i32);
            }
            let _t6 = z.divide2n1n(Clone::clone(&bShifted), Clone::clone(&qi))?;
            let mut ri: MutableBigInteger = _t6;
            quotient.add(Clone::clone(&qi))?;
            ri.rightShift(sigma)?;
            Ok(ri)
        }

        #[java_method(name = "divide2n1n", descriptor = "(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divide2n1n(&self, mut b: MutableBigInteger, mut quotient: MutableBigInteger) -> Result<MutableBigInteger> {
            let this = self;
            let mut n = b.__get_intLen();
            if n < 80i32 {
                let _t0 = this.divideKnuth_mutabl_mutabl(Clone::clone(&b), Clone::clone(&quotient))?;
                return Ok(_t0);
            }
            let mut aUpper = MutableBigInteger::new_mutabl(Clone::clone(this))?;
            aUpper.safeRightShift((32i32).wrapping_mul((n/2i32)))?;
            this.keepLower((n/2i32))?;
            let mut q1 = MutableBigInteger::new()?;
            let _t0 = aUpper.divide3n2n(Clone::clone(&b), Clone::clone(&q1))?;
            let mut r1: MutableBigInteger = _t0;
            this.addDisjoint(Clone::clone(&r1), (n/2i32))?;
            let _t1 = this.divide3n2n(Clone::clone(&b), Clone::clone(&quotient))?;
            let mut r2: MutableBigInteger = _t1;
            quotient.addDisjoint(Clone::clone(&q1), (n/2i32))?;
            Ok(r2)
        }

        #[java_method(name = "divide3n2n", descriptor = "(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divide3n2n(&self, mut b: MutableBigInteger, mut quotient: MutableBigInteger) -> Result<MutableBigInteger> {
            let this = self;
            let mut n = (b.__get_intLen()/2i32);
            let mut a12 = MutableBigInteger::new_mutabl(Clone::clone(this))?;
            a12.safeRightShift((32i32).wrapping_mul(n))?;
            let mut b1 = MutableBigInteger::new_mutabl(Clone::clone(&b))?;
            b1.safeRightShift((n).wrapping_mul(32i32))?;
            let _t0 = b.getLower(n)?;
            let mut b2: BigInteger = _t0;
            let _t1 = this.compareShifted(Clone::clone(&b), n)?;
        let mut d = Default::default();
        let mut r: MutableBigInteger = Default::default();
            if (_t1<0) {
                let _t2 = a12.divide2n1n(Clone::clone(&b1), Clone::clone(&quotient))?;
                r = _t2;
                let _t3 = quotient.toBigInteger()?;
                let _t4 = _t3.multiply_bigint(Clone::clone(&b2))?;
                d = MutableBigInteger::new_bigint(Clone::clone(&_t4))?;
            } else {
                quotient.ones(n)?;
                a12.add(Clone::clone(&b1))?;
                b1.leftShift((32i32).wrapping_mul(n))?;
                let _t2 = a12.subtract(Clone::clone(&b1))?;
                r = a12;
                d = MutableBigInteger::new_bigint(Clone::clone(&b2))?;
                d.leftShift((32i32).wrapping_mul(n))?;
                let _t3 = d.subtract(Clone::clone(&MutableBigInteger::new_bigint(Clone::clone(&b2))?))?;
            }
            r.leftShift((32i32).wrapping_mul(n))?;
            r.addLower(Clone::clone(this), n)?;
            loop {
                let _t2 = r.compare(Clone::clone(&d))?;
                if (_t2>=0) { break; }
                r.add(Clone::clone(&b))?;
                let _t2 = quotient.subtract(Clone::clone(&MutableBigInteger::ONE()))?;
            }
            let _t2 = r.subtract(Clone::clone(&d))?;
            Ok(r)
        }

        #[java_method(name = "getBlock", descriptor = "(III)Ljava/math/MutableBigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBlock(&self, mut index: i32, mut numBlocks: i32, mut blockLength: i32) -> Result<MutableBigInteger> {
            let this = self;
            let mut blockStart = (index).wrapping_mul(blockLength);
            if blockStart >= this.__get_intLen() {
                return Ok(MutableBigInteger::new()?);
            }
        let mut blockEnd = Default::default();
            if index == (numBlocks).wrapping_sub(1i32) {
                blockEnd = this.__get_intLen();
            } else {
                blockEnd = ((index).wrapping_add(1i32)).wrapping_mul(blockLength);
            }
            if blockEnd > this.__get_intLen() {
                return Ok(MutableBigInteger::new()?);
            }
            let _t0: Rc<RefCell<Vec<i32>>> = Arrays::copyOfRange_arr_i_i_i(Clone::clone(&this.__get_value()), ((this.__get_offset()).wrapping_add(this.__get_intLen())).wrapping_sub(blockEnd), ((this.__get_offset()).wrapping_add(this.__get_intLen())).wrapping_sub(blockStart))?;
            let mut newVal: Rc<RefCell<Vec<i32>>> = _t0;
            Ok(MutableBigInteger::new_arr_i(Clone::clone(&newVal))?)
        }

        #[java_method(name = "bitLength", descriptor = "()J", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn bitLength(&self) -> Result<i64> {
            let this = self;
            if (this.__get_intLen()==0) {
                return Ok(0i64);
            }
            let _t0: i32 = Integer::numberOfLeadingZeros(this.__get_value().borrow()[this.__get_offset() as usize])?;
            Ok((((this.__get_intLen() as i64)).wrapping_mul(32i64)).wrapping_sub((_t0 as i64)))
        }

        #[java_method(name = "divide", descriptor = "(JLjava/math/MutableBigInteger;)J", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: divide(JLjava/math/MutableBigInteger;)J
        pub fn divide_l_mutabl(&self, mut v: i64, mut quotient: MutableBigInteger) -> Result<i64> {
            let this = self;
            if (((v>(0i64)) as i32-((v)<(0i64)) as i32)==0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (this.__get_intLen()==0) {
                quotient.__set_offset(0i32);
                quotient.__set_intLen(0i32);
                return Ok(0i64);
            }
            if (((v>(0i64)) as i32-((v)<(0i64)) as i32)<0) {
                v = (v).wrapping_neg();
            }
            let mut d: i32 = (((v as u64).wrapping_shr((32i32&0x3f) as u32) as i64) as i32);
            quotient.clear()?;
            if (d==0) {
                let _t0 = this.divideOneWord((v as i32), Clone::clone(&quotient))?;
                return Ok(((_t0 as i64)&(4294967295i64)));
            }
            let _t0 = this.divideLongMagnitude(v, Clone::clone(&quotient))?;
            let _t1 = _t0.toLong()?;
            Ok(_t1)
        }

        #[java_method(name = "copyAndShift", descriptor = "([III[III)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyAndShift(mut src: Rc<RefCell<Vec<i32>>>, mut srcFrom: i32, mut srcLen: i32, mut dst: Rc<RefCell<Vec<i32>>>, mut dstFrom: i32, mut shift: i32) -> Result<()> {
            let mut n2 = (32i32).wrapping_sub(shift);
            let mut c = src.borrow()[srcFrom as usize];
            let mut i: i32 = 0i32;
            loop {
                if i >= (srcLen).wrapping_sub(1i32) { break; }
                let mut b: i32 = c;
                srcFrom = srcFrom.wrapping_add(1i32);
                c = src.borrow()[srcFrom as usize];
                dst.borrow_mut()[(dstFrom).wrapping_add(i) as usize] = ((b<<(shift&0x1f))|((c as u32>>(n2&0x1f)) as i32));
                i = i.wrapping_add(1i32);
            }
            dst.borrow_mut()[((dstFrom).wrapping_add(srcLen)).wrapping_sub(1i32) as usize] = (c<<(shift&0x1f));
            Ok(())
        }

        #[java_method(name = "divideMagnitude", descriptor = "(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;Z)Ljava/math/MutableBigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideMagnitude(&self, mut div: MutableBigInteger, mut quotient: MutableBigInteger, mut needRemainder: bool) -> Result<MutableBigInteger> {
            let this = self;
            let _t0: i32 = Integer::numberOfLeadingZeros(div.__get_value().borrow()[div.__get_offset() as usize])?;
            let mut shift: i32 = _t0;
            let mut dlen = div.__get_intLen();
        let mut rem = Default::default();
        let mut divisor: Rc<RefCell<Vec<i32>>> = Default::default();
            if (shift>0) {
                let mut _arr1: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; dlen as usize]));
                divisor = _arr1;
                MutableBigInteger::copyAndShift(Clone::clone(&div.__get_value()), div.__get_offset(), dlen, Clone::clone(&divisor), 0i32, shift)?;
                let _t2: i32 = Integer::numberOfLeadingZeros(this.__get_value().borrow()[this.__get_offset() as usize])?;
                if _t2 >= shift {
                    let mut _arr3: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; (this.__get_intLen()).wrapping_add(1i32) as usize]));
                    let mut remarr: Rc<RefCell<Vec<i32>>> = _arr3;
                    rem = MutableBigInteger::new_arr_i(Clone::clone(&remarr))?;
                    rem.__set_intLen(this.__get_intLen());
                    rem.__set_offset(1i32);
                    MutableBigInteger::copyAndShift(Clone::clone(&this.__get_value()), this.__get_offset(), this.__get_intLen(), Clone::clone(&remarr), 1i32, shift)?;
                } else {
                    let mut _arr3: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; (this.__get_intLen()).wrapping_add(2i32) as usize]));
                    let mut remarr: Rc<RefCell<Vec<i32>>> = _arr3;
                    rem = MutableBigInteger::new_arr_i(Clone::clone(&remarr))?;
                    rem.__set_intLen((this.__get_intLen()).wrapping_add(1i32));
                    rem.__set_offset(1i32);
                    let mut rFrom = this.__get_offset();
                    let mut c: i32 = 0i32;
                    let mut n2 = (32i32).wrapping_sub(shift);
                    let mut i: i32 = 1i32;
                    loop {
                        if i >= (this.__get_intLen()).wrapping_add(1i32) { break; }
                        let mut b: i32 = c;
                        c = this.__get_value().borrow()[rFrom as usize];
                        remarr.borrow_mut()[i as usize] = ((b<<(shift&0x1f))|((c as u32>>(n2&0x1f)) as i32));
                        i = i.wrapping_add(1i32);
                        rFrom = rFrom.wrapping_add(1i32);
                    }
                    remarr.borrow_mut()[(this.__get_intLen()).wrapping_add(1i32) as usize] = (c<<(shift&0x1f));
                }
            } else {
                let _t1: Rc<RefCell<Vec<i32>>> = Arrays::copyOfRange_arr_i_i_i(Clone::clone(&div.__get_value()), div.__get_offset(), (div.__get_offset()).wrapping_add(div.__get_intLen()))?;
                divisor = _t1;
                let mut _arr2: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; (this.__get_intLen()).wrapping_add(1i32) as usize]));
                rem = MutableBigInteger::new_arr_i(Clone::clone(&_arr2))?;
                System::arraycopy(Object::from_any(this.__get_value().clone()), this.__get_offset(), Object::from_any(rem.__get_value().clone()), 1i32, this.__get_intLen())?;
                rem.__set_intLen(this.__get_intLen());
                rem.__set_offset(1i32);
            }
            let mut remarr = rem.__get_intLen();
            let mut rFrom = ((remarr).wrapping_sub(dlen)).wrapping_add(1i32);
            if (quotient.__get_value().borrow().len() as i32) < rFrom {
                let mut _arr1: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; rFrom as usize]));
                quotient.__set_value(Clone::clone(&_arr1));
                quotient.__set_offset(0i32);
            }
            quotient.__set_intLen(rFrom);
            let mut c = quotient.__get_value();
            rem.__set_offset(0i32);
            rem.__get_value().borrow_mut()[0i32 as usize] = 0i32;
            rem.__set_intLen((rem.__get_intLen()).wrapping_add(1i32));
            let mut n2 = divisor.borrow()[0i32 as usize];
            let mut i = ((n2 as i64)&(4294967295i64));
            let mut dl = divisor.borrow()[1i32 as usize];
            let mut j: i32 = 0i32;
            loop {
                if j >= (rFrom).wrapping_sub(1i32) { break; }
                let mut qhat: i32 = 0i32;
                let mut qrem: i32 = 0i32;
                let mut skipCorrection: i32 = 0i32;
                let mut nh = rem.__get_value().borrow()[(j).wrapping_add(rem.__get_offset()) as usize];
                let mut nh2 = (nh).wrapping_add(-2147483648i32);
                let mut nm = rem.__get_value().borrow()[((j).wrapping_add(1i32)).wrapping_add(rem.__get_offset()) as usize];
                if nh == n2 {
                    qhat = -1i32;
                    qrem = (nh).wrapping_add(nm);
                    skipCorrection = ((qrem).wrapping_add(-2147483648i32) < nh2) as i32;
                } else {
                    let mut nChunk = (((nh as i64)).wrapping_shl((32i32&0x3f) as u32)|(((nm as i64)&(4294967295i64))));
                    if (((nChunk>(0i64)) as i32-((nChunk)<(0i64)) as i32)>=0) {
                        qhat = ((nChunk/i) as i32);
                        qrem = ((nChunk).wrapping_sub(((qhat as i64)).wrapping_mul(i)) as i32);
                    } else {
                        let _t1: i64 = MutableBigInteger::divWord(nChunk, n2)?;
                        let mut tmp: i64 = _t1;
                        qhat = ((tmp&(4294967295i64)) as i32);
                        qrem = (((tmp as u64).wrapping_shr((32i32&0x3f) as u32) as i64) as i32);
                    }
                }
                if (qhat==0) {
                } else {
                    let mut nChunk = ((rem.__get_value().borrow()[((j).wrapping_add(2i32)).wrapping_add(rem.__get_offset()) as usize] as i64)&(4294967295i64));
                    let mut tmp = ((((qrem as i64)&(4294967295i64))).wrapping_shl((32i32&0x3f) as u32)|(nChunk));
                    let mut estProduct = (((dl as i64)&(4294967295i64))).wrapping_mul(((qhat as i64)&(4294967295i64)));
                    let _t1 = this.unsignedLongCompare(estProduct, tmp)?;
                    qhat = qhat.wrapping_sub(1i32);
                    qrem = ((((qrem as i64)&(4294967295i64))).wrapping_add(i) as i32);
                    estProduct = (estProduct).wrapping_sub(((dl as i64)&(4294967295i64)));
                    tmp = ((((qrem as i64)&(4294967295i64))).wrapping_shl((32i32&0x3f) as u32)|(nChunk));
                    let _t2 = this.unsignedLongCompare(estProduct, tmp)?;
                    if _t2 {
                        qhat = qhat.wrapping_sub(1i32);
                    }
                    rem.__get_value().borrow_mut()[(j).wrapping_add(rem.__get_offset()) as usize] = 0i32;
                    let _t3 = this.mulsub(Clone::clone(&rem.__get_value()), Clone::clone(&divisor), qhat, dlen, (j).wrapping_add(rem.__get_offset()))?;
                    let mut nChunk: i32 = _t3;
                    if (nChunk).wrapping_add(-2147483648i32) > nh2 {
                        let _t4 = this.divadd(Clone::clone(&divisor), Clone::clone(&rem.__get_value()), ((j).wrapping_add(1i32)).wrapping_add(rem.__get_offset()))?;
                        qhat = qhat.wrapping_sub(1i32);
                    }
                    c.borrow_mut()[j as usize] = qhat;
                }
                j = j.wrapping_add(1i32);
            }
            j = 0i32;
            let mut qhat: i32 = 0i32;
            let mut qrem: i32 = 0i32;
            let mut skipCorrection = rem.__get_value().borrow()[((rFrom).wrapping_sub(1i32)).wrapping_add(rem.__get_offset()) as usize];
            let mut nh = (skipCorrection).wrapping_add(-2147483648i32);
            let mut nh2 = rem.__get_value().borrow()[(rFrom).wrapping_add(rem.__get_offset()) as usize];
            if skipCorrection == n2 {
                j = -1i32;
                qhat = (skipCorrection).wrapping_add(nh2);
                qrem = ((qhat).wrapping_add(-2147483648i32) < nh) as i32;
            } else {
                let mut nm = (((skipCorrection as i64)).wrapping_shl((32i32&0x3f) as u32)|(((nh2 as i64)&(4294967295i64))));
                if (((nm>(0i64)) as i32-((nm)<(0i64)) as i32)>=0) {
                    j = ((nm/i) as i32);
                    qhat = ((nm).wrapping_sub(((j as i64)).wrapping_mul(i)) as i32);
                } else {
                    let _t1: i64 = MutableBigInteger::divWord(nm, n2)?;
                    let mut tmp: i64 = _t1;
                    j = ((tmp&(4294967295i64)) as i32);
                    qhat = (((tmp as u64).wrapping_shr((32i32&0x3f) as u32) as i64) as i32);
                }
            }
            let mut nm = ((rem.__get_value().borrow()[((rFrom).wrapping_add(1i32)).wrapping_add(rem.__get_offset()) as usize] as i64)&(4294967295i64));
            let mut tmp = ((((qhat as i64)&(4294967295i64))).wrapping_shl((32i32&0x3f) as u32)|(nm));
            let mut estProduct = (((dl as i64)&(4294967295i64))).wrapping_mul(((j as i64)&(4294967295i64)));
            let _t1 = this.unsignedLongCompare(estProduct, tmp)?;
            j = j.wrapping_sub(1i32);
            qhat = ((((qhat as i64)&(4294967295i64))).wrapping_add(i) as i32);
            estProduct = (estProduct).wrapping_sub(((dl as i64)&(4294967295i64)));
            tmp = ((((qhat as i64)&(4294967295i64))).wrapping_shl((32i32&0x3f) as u32)|(nm));
            let _t2 = this.unsignedLongCompare(estProduct, tmp)?;
            if _t2 {
                j = j.wrapping_sub(1i32);
            }
            rem.__get_value().borrow_mut()[((rFrom).wrapping_sub(1i32)).wrapping_add(rem.__get_offset()) as usize] = 0i32;
            if needRemainder {
                let _t3 = this.mulsub(Clone::clone(&rem.__get_value()), Clone::clone(&divisor), j, dlen, ((rFrom).wrapping_sub(1i32)).wrapping_add(rem.__get_offset()))?;
                let mut nm: i32 = _t3;
            } else {
                let _t3 = this.mulsubBorrow(Clone::clone(&rem.__get_value()), Clone::clone(&divisor), j, dlen, ((rFrom).wrapping_sub(1i32)).wrapping_add(rem.__get_offset()))?;
                let mut nm: i32 = _t3;
            }
            if needRemainder {
                let _t3 = this.divadd(Clone::clone(&divisor), Clone::clone(&rem.__get_value()), (((rFrom).wrapping_sub(1i32)).wrapping_add(1i32)).wrapping_add(rem.__get_offset()))?;
            }
            j = j.wrapping_sub(1i32);
            c.borrow_mut()[(rFrom).wrapping_sub(1i32) as usize] = j;
            if (shift>0) {
                rem.rightShift(shift)?;
            }
            rem.normalize()?;
            quotient.normalize()?;
            Ok((if needRemainder { rem } else { Default::default() }))
        }

        #[java_method(name = "divideLongMagnitude", descriptor = "(JLjava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideLongMagnitude(&self, mut ldivisor: i64, mut quotient: MutableBigInteger) -> Result<MutableBigInteger> {
            let this = self;
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; (this.__get_intLen()).wrapping_add(1i32) as usize]));
            let mut rem = MutableBigInteger::new_arr_i(Clone::clone(&_arr0))?;
            System::arraycopy(Object::from_any(this.__get_value().clone()), this.__get_offset(), Object::from_any(rem.__get_value().clone()), 1i32, this.__get_intLen())?;
            rem.__set_intLen(this.__get_intLen());
            rem.__set_offset(1i32);
            let mut nlen = rem.__get_intLen();
            let mut limit = ((nlen).wrapping_sub(2i32)).wrapping_add(1i32);
            if (quotient.__get_value().borrow().len() as i32) < limit {
                let mut _arr1: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; limit as usize]));
                quotient.__set_value(Clone::clone(&_arr1));
                quotient.__set_offset(0i32);
            }
            quotient.__set_intLen(limit);
            let mut q = quotient.__get_value();
            let _t1: i32 = Long::numberOfLeadingZeros(ldivisor)?;
            let mut shift: i32 = _t1;
            if (shift>0) {
                ldivisor = (ldivisor).wrapping_shl((shift&0x3f) as u32);
                rem.leftShift(shift)?;
            }
            if rem.__get_intLen() == nlen {
                rem.__set_offset(0i32);
                rem.__get_value().borrow_mut()[0i32 as usize] = 0i32;
                rem.__set_intLen((rem.__get_intLen()).wrapping_add(1i32));
            }
            let mut dh: i32 = (((ldivisor as u64).wrapping_shr((32i32&0x3f) as u32) as i64) as i32);
            let mut dhLong = ((dh as i64)&(4294967295i64));
            let mut dl: i32 = ((ldivisor&(4294967295i64)) as i32);
            let mut j: i32 = 0i32;
            loop {
                if j >= limit { break; }
                let mut qhat: i32 = 0i32;
                let mut qrem: i32 = 0i32;
                let mut skipCorrection: i32 = 0i32;
                let mut nh = rem.__get_value().borrow()[(j).wrapping_add(rem.__get_offset()) as usize];
                let mut nh2 = (nh).wrapping_add(-2147483648i32);
                let mut nm = rem.__get_value().borrow()[((j).wrapping_add(1i32)).wrapping_add(rem.__get_offset()) as usize];
                if nh == dh {
                    qhat = -1i32;
                    qrem = (nh).wrapping_add(nm);
                    skipCorrection = ((qrem).wrapping_add(-2147483648i32) < nh2) as i32;
                } else {
                    let mut nChunk = (((nh as i64)).wrapping_shl((32i32&0x3f) as u32)|(((nm as i64)&(4294967295i64))));
                    if (((nChunk>(0i64)) as i32-((nChunk)<(0i64)) as i32)>=0) {
                        qhat = ((nChunk/dhLong) as i32);
                        qrem = ((nChunk).wrapping_sub(((qhat as i64)).wrapping_mul(dhLong)) as i32);
                    } else {
                        let _t2: i64 = MutableBigInteger::divWord(nChunk, dh)?;
                        let mut tmp: i64 = _t2;
                        qhat = ((tmp&(4294967295i64)) as i32);
                        qrem = (((tmp as u64).wrapping_shr((32i32&0x3f) as u32) as i64) as i32);
                    }
                }
                if (qhat==0) {
                } else {
                    let mut nChunk = ((rem.__get_value().borrow()[((j).wrapping_add(2i32)).wrapping_add(rem.__get_offset()) as usize] as i64)&(4294967295i64));
                    let mut tmp = ((((qrem as i64)&(4294967295i64))).wrapping_shl((32i32&0x3f) as u32)|(nChunk));
                    let mut estProduct = (((dl as i64)&(4294967295i64))).wrapping_mul(((qhat as i64)&(4294967295i64)));
                    let _t2 = this.unsignedLongCompare(estProduct, tmp)?;
                    qhat = qhat.wrapping_sub(1i32);
                    qrem = ((((qrem as i64)&(4294967295i64))).wrapping_add(dhLong) as i32);
                    estProduct = (estProduct).wrapping_sub(((dl as i64)&(4294967295i64)));
                    tmp = ((((qrem as i64)&(4294967295i64))).wrapping_shl((32i32&0x3f) as u32)|(nChunk));
                    let _t3 = this.unsignedLongCompare(estProduct, tmp)?;
                    if _t3 {
                        qhat = qhat.wrapping_sub(1i32);
                    }
                    rem.__get_value().borrow_mut()[(j).wrapping_add(rem.__get_offset()) as usize] = 0i32;
                    let _t4 = this.mulsubLong(Clone::clone(&rem.__get_value()), dh, dl, qhat, (j).wrapping_add(rem.__get_offset()))?;
                    let mut nChunk: i32 = _t4;
                    if (nChunk).wrapping_add(-2147483648i32) > nh2 {
                        let _t5 = this.divaddLong(dh, dl, Clone::clone(&rem.__get_value()), ((j).wrapping_add(1i32)).wrapping_add(rem.__get_offset()))?;
                        qhat = qhat.wrapping_sub(1i32);
                    }
                    q.borrow_mut()[j as usize] = qhat;
                }
                j = j.wrapping_add(1i32);
            }
            if (shift>0) {
                rem.rightShift(shift)?;
            }
            quotient.normalize()?;
            rem.normalize()?;
            Ok(rem)
        }

        #[java_method(name = "divaddLong", descriptor = "(II[II)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divaddLong(&self, mut dh: i32, mut dl: i32, mut result: Rc<RefCell<Vec<i32>>>, mut offset: i32) -> Result<i32> {
            let this = self;
            let mut carry: i64 = 0i64;
            let mut sum = (((dl as i64)&(4294967295i64))).wrapping_add(((result.borrow()[(1i32).wrapping_add(offset) as usize] as i64)&(4294967295i64)));
            result.borrow_mut()[(1i32).wrapping_add(offset) as usize] = (sum as i32);
            sum = ((((dh as i64)&(4294967295i64))).wrapping_add(((result.borrow()[offset as usize] as i64)&(4294967295i64)))).wrapping_add(carry);
            result.borrow_mut()[offset as usize] = (sum as i32);
            carry = ((sum as u64).wrapping_shr((32i32&0x3f) as u32) as i64);
            Ok((carry as i32))
        }

        #[java_method(name = "mulsubLong", descriptor = "([IIIII)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mulsubLong(&self, mut q: Rc<RefCell<Vec<i32>>>, mut dh: i32, mut dl: i32, mut x: i32, mut offset: i32) -> Result<i32> {
            let this = self;
            let mut xLong = ((x as i64)&(4294967295i64));
            offset = offset.wrapping_add(2i32);
            let mut product = (((dl as i64)&(4294967295i64))).wrapping_mul(xLong);
            let mut difference = ((q.borrow()[offset as usize] as i64)).wrapping_sub(product);
            offset = offset.wrapping_sub(1i32);
            q.borrow_mut()[offset as usize] = (difference as i32);
            let mut carry = (((product as u64).wrapping_shr((32i32&0x3f) as u32) as i64)).wrapping_add((((((difference&(4294967295i64))>(((((product as i32)^-1i32) as i64)&(4294967295i64)))) as i32-(((difference&(4294967295i64)))<(((((product as i32)^-1i32) as i64)&(4294967295i64)))) as i32)>0) as i64));
            product = ((((dh as i64)&(4294967295i64))).wrapping_mul(xLong)).wrapping_add(carry);
            difference = ((q.borrow()[offset as usize] as i64)).wrapping_sub(product);
            offset = offset.wrapping_sub(1i32);
            q.borrow_mut()[offset as usize] = (difference as i32);
            carry = (((product as u64).wrapping_shr((32i32&0x3f) as u32) as i64)).wrapping_add((((((difference&(4294967295i64))>(((((product as i32)^-1i32) as i64)&(4294967295i64)))) as i32-(((difference&(4294967295i64)))<(((((product as i32)^-1i32) as i64)&(4294967295i64)))) as i32)>0) as i64));
            Ok((carry as i32))
        }

        #[java_method(name = "unsignedLongCompare", descriptor = "(JJ)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unsignedLongCompare(&self, mut one: i64, mut two: i64) -> Result<bool> {
            let this = self;
            Ok(((((one).wrapping_add(-9223372036854775808i64)>((two).wrapping_add(-9223372036854775808i64))) as i32-(((one).wrapping_add(-9223372036854775808i64))<((two).wrapping_add(-9223372036854775808i64))) as i32)>0))
        }

        #[java_method(name = "divWord", descriptor = "(JI)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divWord(mut n: i64, mut d: i32) -> Result<i64> {
            let mut dLong = ((d as i64)&(4294967295i64));
            if (((dLong>(1i64)) as i32-((dLong)<(1i64)) as i32)==0) {
                let mut q: i64 = ((n as i32) as i64);
                let mut r: i64 = 0i64;
                return Ok(((r).wrapping_shl((32i32&0x3f) as u32)|((q&(4294967295i64)))));
            }
            let mut q = (((n as u64).wrapping_shr((1i32&0x3f) as u32) as i64)/((dLong as u64).wrapping_shr((1i32&0x3f) as u32) as i64));
            let mut r = (n).wrapping_sub((q).wrapping_mul(dLong));
            loop {
                if (((r>(0i64)) as i32-((r)<(0i64)) as i32)>=0) { break; }
                r = (r).wrapping_add(dLong);
                q = (q).wrapping_sub(1i64);
            }
            loop {
                if (((r>(dLong)) as i32-((r)<(dLong)) as i32)<0) { break; }
                r = (r).wrapping_sub(dLong);
                q = (q).wrapping_add(1i64);
            }
            Ok(((r).wrapping_shl((32i32&0x3f) as u32)|((q&(4294967295i64)))))
        }

        #[java_method(name = "sqrt", descriptor = "()Ljava/math/MutableBigInteger;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sqrt(&self) -> Result<MutableBigInteger> {
            panic!("stub: java/math/MutableBigInteger.sqrt:()Ljava/math/MutableBigInteger;")
        }

        #[java_method(name = "hybridGCD", descriptor = "(Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hybridGCD(&self, b: MutableBigInteger) -> Result<MutableBigInteger> {
            panic!("stub: java/math/MutableBigInteger.hybridGCD:(Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;")
        }

        #[java_method(name = "binaryGCD", descriptor = "(Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binaryGCD(&self, v: MutableBigInteger) -> Result<MutableBigInteger> {
            panic!("stub: java/math/MutableBigInteger.binaryGCD:(Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;")
        }

        #[java_method(name = "binaryGcd", descriptor = "(II)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn binaryGcd(a: i32, b: i32) -> Result<i32> {
            panic!("stub: java/math/MutableBigInteger.binaryGcd:(II)I")
        }

        #[java_method(name = "mutableModInverse", descriptor = "(Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mutableModInverse(&self, p: MutableBigInteger) -> Result<MutableBigInteger> {
            panic!("stub: java/math/MutableBigInteger.mutableModInverse:(Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;")
        }

        #[java_method(name = "modInverseMP2", descriptor = "(I)Ljava/math/MutableBigInteger;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn modInverseMP2(&self, k: i32) -> Result<MutableBigInteger> {
            panic!("stub: java/math/MutableBigInteger.modInverseMP2:(I)Ljava/math/MutableBigInteger;")
        }

        #[java_method(name = "inverseMod32", descriptor = "(I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn inverseMod32(val: i32) -> Result<i32> {
            panic!("stub: java/math/MutableBigInteger.inverseMod32:(I)I")
        }

        #[java_method(name = "inverseMod64", descriptor = "(J)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn inverseMod64(val: i64) -> Result<i64> {
            panic!("stub: java/math/MutableBigInteger.inverseMod64:(J)J")
        }

        #[java_method(name = "modInverseBP2", descriptor = "(Ljava/math/MutableBigInteger;I)Ljava/math/MutableBigInteger;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn modInverseBP2(mod_: MutableBigInteger, k: i32) -> Result<MutableBigInteger> {
            panic!("stub: java/math/MutableBigInteger.modInverseBP2:(Ljava/math/MutableBigInteger;I)Ljava/math/MutableBigInteger;")
        }

        #[java_method(name = "modInverse", descriptor = "(Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn modInverse(&self, mod_: MutableBigInteger) -> Result<MutableBigInteger> {
            panic!("stub: java/math/MutableBigInteger.modInverse:(Ljava/math/MutableBigInteger;)Ljava/math/MutableBigInteger;")
        }

        #[java_method(name = "fixup", descriptor = "(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;I)Ljava/math/MutableBigInteger;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fixup(c: MutableBigInteger, p: MutableBigInteger, k: i32) -> Result<MutableBigInteger> {
            panic!("stub: java/math/MutableBigInteger.fixup:(Ljava/math/MutableBigInteger;Ljava/math/MutableBigInteger;I)Ljava/math/MutableBigInteger;")
        }

        #[java_method(name = "euclidModInverse", descriptor = "(I)Ljava/math/MutableBigInteger;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn euclidModInverse(&self, k: i32) -> Result<MutableBigInteger> {
            panic!("stub: java/math/MutableBigInteger.euclidModInverse:(I)Ljava/math/MutableBigInteger;")
        }
    }
}
