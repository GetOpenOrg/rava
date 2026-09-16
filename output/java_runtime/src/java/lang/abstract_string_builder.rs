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
use crate::jdk::internal::math::DoubleToDecimal;
use crate::jdk::internal::math::FloatToDecimal;
use crate::jdk::internal::util::ArraysSupport;
use crate::jdk::internal::util::Preconditions;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/AbstractStringBuilder"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/lang/Appendable,java/lang/CharSequence"]
    #[access            = "package"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AbstractStringBuilder.java"]
    #[inner_classes     = "java/lang/StringLatin1$CharsSpliterator:java/lang/StringLatin1:CharsSpliterator:8;java/lang/StringUTF16$CodePointsSpliterator:java/lang/StringUTF16:CodePointsSpliterator:8;java/lang/StringUTF16$CharsSpliterator:java/lang/StringUTF16:CharsSpliterator:8;java/util/Spliterator$OfInt:java/util/Spliterator:OfInt:1545;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/AbstractStringBuilder;java/lang/Appendable;java/lang/CharSequence;java/lang/Object"]
    #[has_to_string_method = true]

    pub struct AbstractStringBuilder {
        #[cfg_attr(any(), java_field(name = "value", descriptor = "[B", is_static = false))]
        pub value: Rc<RefCell<Vec<i8>>>,
        #[cfg_attr(any(), java_field(name = "coder", descriptor = "B", is_static = false))]
        pub coder: i8,
        #[cfg_attr(any(), java_field(name = "maybeLatin1", descriptor = "Z", is_static = false))]
        pub maybeLatin1: bool,
        #[cfg_attr(any(), java_field(name = "count", descriptor = "I", is_static = false))]
        pub count: i32,
    }

    impl AbstractStringBuilder {
        #[cfg_attr(any(), java_field(name = "EMPTYVALUE", descriptor = "[B", access = "private", modifiers = "static final", is_static = true))]
        // static field: EMPTYVALUE:[B
        pub fn EMPTYVALUE() -> Rc<RefCell<Vec<i8>>> {
            panic!("stub: java/lang/AbstractStringBuilder.EMPTYVALUE:[B")
        }

        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_value(Clone::clone(&AbstractStringBuilder::EMPTYVALUE()));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(I)V
        pub fn new_i(mut capacity: i32) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            if String::COMPACT_STRINGS() {
                let mut _arr0: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; capacity as usize]));
                this.__set_value(Clone::clone(&_arr0));
                this.__set_coder(((0i32) as i8));
            } else {
                let _t0: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor(capacity)?;
                this.__set_value(Clone::clone(&_t0));
                this.__set_coder(((1i32) as i8));
            }
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/String;)V
        pub fn new_str(mut str: String) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            let _t0 = str.length()?;
            let mut length: i32 = _t0;
            let mut capacity = (if length < 2147483631i32 { (length).wrapping_add(16i32) } else { 2147483647i32 });
            let _t1 = str.coder()?;
            let mut initCoder: i8 = _t1;
            this.__set_coder(initCoder);
            let mut _merged3: Rc<RefCell<Vec<i8>>>;
            if (initCoder==0) {
                let mut _arr2: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; capacity as usize]));
                _merged3 = _arr2;
            } else {
                let _t2: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor(capacity)?;
                _merged3 = _t2;
            }
            this.__set_value(Clone::clone(&_merged3));
            let _t4 = this.append_str(Clone::clone(&str))?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/CharSequence;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_seq(seq: Object) -> Result<Self> {
            panic!("stub: java/lang/AbstractStringBuilder.<init>:(Ljava/lang/CharSequence;)V")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/lang/AbstractStringBuilder;)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, mut another: AbstractStringBuilder) -> Result<i32> {
            let this = self;
            if Object::from_any(this.clone()) == Object::from_any(another.clone()) {
                return Ok(0i32);
            }
            let mut val1 = this.__get_value();
            let mut val2 = another.__get_value();
            let mut count1 = this.__get_count();
            let mut count2 = another.__get_count();
            let _t0 = this.isLatin1()?;
            let mut _merged2: i32;
            if _t0 {
                let _t1: i32 = StringLatin1::compareTo_arr_b_arr_b_i_i(Clone::clone(&val1), Clone::clone(&val2), count1, count2)?;
                _merged2 = _t1;
            } else {
                let _t1: i32 = StringUTF16::compareTo_arr_b_arr_b_i_i(Clone::clone(&val1), Clone::clone(&val2), count1, count2)?;
                _merged2 = _t1;
            }
            return Ok(_merged2);
            let _t3 = this.isLatin1()?;
            let mut _merged5: i32;
            if _t3 {
                let _t4: i32 = StringLatin1::compareToUTF16_arr_b_arr_b_i_i(Clone::clone(&val1), Clone::clone(&val2), count1, count2)?;
                _merged5 = _t4;
            } else {
                let _t4: i32 = StringUTF16::compareToLatin1_arr_b_arr_b_i_i(Clone::clone(&val1), Clone::clone(&val2), count1, count2)?;
                _merged5 = _t4;
            }
            Ok(_merged5)
        }

        #[java_method(name = "length", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn length(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_count())
        }

        #[java_method(name = "capacity", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn capacity(&self) -> Result<i32> {
            panic!("stub: java/lang/AbstractStringBuilder.capacity:()I")
        }

        #[java_method(name = "ensureCapacity", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ensureCapacity(&self, minimumCapacity: i32) -> Result<()> {
            panic!("stub: java/lang/AbstractStringBuilder.ensureCapacity:(I)V")
        }

        #[java_method(name = "ensureCapacityInternal", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ensureCapacityInternal(&self, mut minimumCapacity: i32) -> Result<()> {
            let this = self;
            let mut oldCapacity = ((this.__get_value().borrow().len() as i32)>>(((this.__get_coder() as i32)&0x1f)));
            if ((minimumCapacity).wrapping_sub(oldCapacity)>0) {
                let _t0 = this.newCapacity(minimumCapacity)?;
                let _t1: Rc<RefCell<Vec<i8>>> = Arrays::copyOf_arr_b_i(Clone::clone(&this.__get_value()), (_t0<<((this.__get_coder() as i32)&0x1f)))?;
                this.__set_value(Clone::clone(&_t1));
            }
            Ok(())
        }

        #[java_method(name = "newCapacity", descriptor = "(I)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newCapacity(&self, mut minCapacity: i32) -> Result<i32> {
            let this = self;
            let mut oldLength = (this.__get_value().borrow().len() as i32);
            let mut newLength = (minCapacity<<((this.__get_coder() as i32)&0x1f));
            let mut growth = (newLength).wrapping_sub(oldLength);
            let _t0: i32 = ArraysSupport::newLength(oldLength, growth, (oldLength).wrapping_add((2i32<<((this.__get_coder() as i32)&0x1f))))?;
            let mut length: i32 = _t0;
            if length == 2147483647i32 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok((length>>(((this.__get_coder() as i32)&0x1f))))
        }

        #[java_method(name = "inflate", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn inflate(&self) -> Result<()> {
            let this = self;
            let _t0 = this.isLatin1()?;
            if !(_t0) {
                return Ok(());
            }
            let _t1: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor((this.__get_value().borrow().len() as i32))?;
            let mut buf: Rc<RefCell<Vec<i8>>> = _t1;
            StringLatin1::inflate_arr_b_i_arr_b_i_i(Clone::clone(&this.__get_value()), 0i32, Clone::clone(&buf), 0i32, this.__get_count())?;
            this.__set_value(Clone::clone(&buf));
            this.__set_coder(((1i32) as i8));
            Ok(())
        }

        #[java_method(name = "trimToSize", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn trimToSize(&self) -> Result<()> {
            panic!("stub: java/lang/AbstractStringBuilder.trimToSize:()V")
        }

        #[java_method(name = "setLength", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setLength(&self, mut newLength: i32) -> Result<()> {
            let this = self;
            if (newLength<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.ensureCapacityInternal(newLength)?;
            if this.__get_count() < newLength {
                let _t0 = this.isLatin1()?;
                if _t0 {
                    StringLatin1::fillNull(Clone::clone(&this.__get_value()), this.__get_count(), newLength)?;
                } else {
                    StringUTF16::fillNull(Clone::clone(&this.__get_value()), this.__get_count(), newLength)?;
                }
            } else {
                if this.__get_count() > newLength {
                    this.__set_maybeLatin1((1i32 != 0i32));
                }
            }
            this.__set_count(newLength);
            Ok(())
        }

        #[java_method(name = "charAt", descriptor = "(I)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn charAt(&self, mut index: i32) -> Result<u16> {
            let this = self;
            let mut coder = this.__get_coder();
            let mut value = this.__get_value();
            let _t0: i32 = Math::min_i_i(this.__get_count(), ((value.borrow().len() as i32)>>(((coder as i32)&0x1f))))?;
            let mut count: i32 = _t0;
            String::checkIndex(index, count)?;
            if (coder==0) {
                return Ok(((((((value.borrow()[index as usize] as i32)&255i32)) as u16 as i32)) as u16));
            }
            let _t1: u16 = StringUTF16::getChar(Clone::clone(&value), index)?;
            Ok(_t1)
        }

        #[java_method(name = "codePointAt", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointAt(&self, index: i32) -> Result<i32> {
            panic!("stub: java/lang/AbstractStringBuilder.codePointAt:(I)I")
        }

        #[java_method(name = "codePointBefore", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointBefore(&self, index: i32) -> Result<i32> {
            panic!("stub: java/lang/AbstractStringBuilder.codePointBefore:(I)I")
        }

        #[java_method(name = "codePointCount", descriptor = "(II)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointCount(&self, beginIndex: i32, endIndex: i32) -> Result<i32> {
            panic!("stub: java/lang/AbstractStringBuilder.codePointCount:(II)I")
        }

        #[java_method(name = "offsetByCodePoints", descriptor = "(II)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn offsetByCodePoints(&self, index: i32, codePointOffset: i32) -> Result<i32> {
            panic!("stub: java/lang/AbstractStringBuilder.offsetByCodePoints:(II)I")
        }

        #[java_method(name = "getChars", descriptor = "(II[CI)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getChars(&self, srcBegin: i32, srcEnd: i32, dst: Rc<RefCell<Vec<u16>>>, dstBegin: i32) -> Result<()> {
            panic!("stub: java/lang/AbstractStringBuilder.getChars:(II[CI)V")
        }

        #[java_method(name = "setCharAt", descriptor = "(IC)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setCharAt(&self, index: i32, ch: u16) -> Result<()> {
            panic!("stub: java/lang/AbstractStringBuilder.setCharAt:(IC)V")
        }

        #[java_method(name = "append", descriptor = "(Ljava/lang/Object;)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn append_obj(&self, obj: Object) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.append:(Ljava/lang/Object;)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "append", descriptor = "(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: append(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;
        pub fn append_str(&self, mut str: String) -> Result<AbstractStringBuilder> {
            let this = self;
            if _is_jnull(&str) {
                let _t0 = this.appendNull()?;
                return Ok(_t0);
            }
            let _t0 = str.length()?;
            let mut len: i32 = _t0;
            this.ensureCapacityInternal((this.__get_count()).wrapping_add(len))?;
            this.putStringAt(this.__get_count(), Clone::clone(&str))?;
            this.__set_count((this.__get_count()).wrapping_add(len));
            Ok(Clone::clone(this))
        }

        #[java_method(name = "append", descriptor = "(Ljava/lang/StringBuffer;)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn append_string(&self, sb: Object) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.append:(Ljava/lang/StringBuffer;)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "append", descriptor = "(Ljava/lang/AbstractStringBuilder;)Ljava/lang/AbstractStringBuilder;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: append(Ljava/lang/AbstractStringBuilder;)Ljava/lang/AbstractStringBuilder;
        pub fn append_abstra(&self, mut asb: AbstractStringBuilder) -> Result<AbstractStringBuilder> {
            let this = self;
            if _is_jnull(&asb) {
                let _t0 = this.appendNull()?;
                return Ok(_t0);
            }
            let _t0 = asb.length()?;
            let mut len: i32 = _t0;
            this.ensureCapacityInternal((this.__get_count()).wrapping_add(len))?;
            this.inflateIfNeededFor_abstra(Clone::clone(&asb))?;
            asb.getBytes(Clone::clone(&this.__get_value()), this.__get_count(), this.__get_coder())?;
            this.__set_count((this.__get_count()).wrapping_add(len));
            this.__set_maybeLatin1((((this.__get_maybeLatin1() as i32)|(asb.__get_maybeLatin1() as i32)) != 0i32));
            Ok(Clone::clone(this))
        }

        #[java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: append(Ljava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;
        pub fn append_seq(&self, mut s: Object) -> Result<AbstractStringBuilder> {
            let this = self;
            if _is_jnull(&s) {
                let _t0 = this.appendNull()?;
                return Ok(_t0);
            }
            if (s.is_instance_of("java/lang/String")) {
                let _t0 = this.append_str(Clone::clone(&(s).downcast::<String>()))?;
                return Ok(_t0);
            }
            if (s.is_instance_of("java/lang/AbstractStringBuilder")) {
                let _t0 = this.append_abstra(Clone::clone(&(s).downcast::<AbstractStringBuilder>()))?;
                return Ok(_t0);
            }
            let _vdispatch0: i32 = if let Some(_d) = s.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.length()? } else if let Some(_d) = s.0.as_any().downcast_ref::<CharBuffer>() { _d.length()? } else if let Some(_d) = s.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.length()? } else if let Some(_d) = s.0.as_any().downcast_ref::<String>() { _d.length()? } else if let Some(_d) = s.0.as_any().downcast_ref::<StringBuilder>() { _d.length()? } else if let Some(_d) = s.0.as_any().downcast_ref::<Object>() { _d.length()? } else if let Some(__f) = s.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            let _t1 = this.append_seq_i_i(Clone::clone(&s), 0i32, _vdispatch0)?;
            Ok(_t1)
        }

        #[java_method(name = "appendNull", descriptor = "()Ljava/lang/AbstractStringBuilder;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn appendNull(&self) -> Result<AbstractStringBuilder> {
            let this = self;
            this.ensureCapacityInternal((this.__get_count()).wrapping_add(4i32))?;
            let mut count = this.__get_count();
            let mut val = this.__get_value();
            let _t0 = this.isLatin1()?;
            if _t0 {
                count = count.wrapping_add(1i32);
                val.borrow_mut()[count as usize] = (110i32) as i8;
                count = count.wrapping_add(1i32);
                val.borrow_mut()[count as usize] = (117i32) as i8;
                count = count.wrapping_add(1i32);
                val.borrow_mut()[count as usize] = (108i32) as i8;
                count = count.wrapping_add(1i32);
                val.borrow_mut()[count as usize] = (108i32) as i8;
            } else {
                let _t1: i32 = StringUTF16::putCharsAt_arr_b_i_c_c_c_c(Clone::clone(&val), count, ((110i32) as u16), ((117i32) as u16), ((108i32) as u16), ((108i32) as u16))?;
                count = _t1;
            }
            this.__set_count(count);
            Ok(Clone::clone(this))
        }

        #[java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: append(Ljava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;
        pub fn append_seq_i_i(&self, mut s: Object, mut start: i32, mut end: i32) -> Result<AbstractStringBuilder> {
            let this = self;
            if _is_jnull(&s) {
                let mut s: String = String::from("null");
            }
            let _t0 = s.length()?;
            let _t1: i32 = Preconditions::checkFromToIndex_i_i_i_bifunc(start, end, _t0, Clone::clone(&Preconditions::IOOBE_FORMATTER()))?;
            let mut len = (end).wrapping_sub(start);
            this.ensureCapacityInternal((this.__get_count()).wrapping_add(len))?;
            if true {
                this.appendChars_str_i_i(Clone::clone(&s), start, end)?;
            } else {
                this.appendChars_seq_i_i(Object::from_any(s.clone()), start, end)?;
            }
            Ok(Clone::clone(this))
        }

        #[java_method(name = "append", descriptor = "([C)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: append([C)Ljava/lang/AbstractStringBuilder;
        pub fn append_arr_c(&self, mut str: Rc<RefCell<Vec<u16>>>) -> Result<AbstractStringBuilder> {
            let this = self;
            let mut len = (str.borrow().len() as i32);
            this.ensureCapacityInternal((this.__get_count()).wrapping_add(len))?;
            this.appendChars_arr_c_i_i(Clone::clone(&str), 0i32, len)?;
            Ok(Clone::clone(this))
        }

        #[java_method(name = "append", descriptor = "([CII)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn append_arr_c_i_i(&self, str: Rc<RefCell<Vec<u16>>>, offset: i32, len: i32) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.append:([CII)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "append", descriptor = "(Z)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn append_z(&self, b: bool) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.append:(Z)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "append", descriptor = "(C)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: append(C)Ljava/lang/AbstractStringBuilder;
        pub fn append_c(&self, mut c: u16) -> Result<AbstractStringBuilder> {
            let this = self;
            this.ensureCapacityInternal((this.__get_count()).wrapping_add(1i32))?;
            let _t0 = this.isLatin1()?;
            if _t0 {
                let _t1: bool = StringLatin1::canEncode_c(c)?;
                if _t1 {
                    this.__set_count((this.__get_count()).wrapping_add(1i32));
                    this.__get_value().borrow_mut()[this.__get_count() as usize] = (((c) as i8 as i32)) as i8;
                } else {
                    let _t2 = this.isLatin1()?;
                    if _t2 {
                        this.inflate()?;
                    }
                    this.__set_count((this.__get_count()).wrapping_add(1i32));
                    StringUTF16::putCharSB(Clone::clone(&this.__get_value()), this.__get_count(), (c as i32))?;
                }
            } else {
                let _t1 = this.isLatin1()?;
                if _t1 {
                    this.inflate()?;
                }
                this.__set_count((this.__get_count()).wrapping_add(1i32));
                StringUTF16::putCharSB(Clone::clone(&this.__get_value()), this.__get_count(), (c as i32))?;
            }
            Ok(Clone::clone(this))
        }

        #[java_method(name = "append", descriptor = "(I)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: append(I)Ljava/lang/AbstractStringBuilder;
        pub fn append_i(&self, mut i: i32) -> Result<AbstractStringBuilder> {
            let this = self;
            let mut count = this.__get_count();
            let _t0: i32 = Integer::stringSize(i)?;
            let mut spaceNeeded = (count).wrapping_add(_t0);
            this.ensureCapacityInternal(spaceNeeded)?;
            let _t1 = this.isLatin1()?;
            if _t1 {
                let _t2: i32 = Integer::getChars(i, spaceNeeded, Clone::clone(&this.__get_value()))?;
            } else {
                let _t2: i32 = StringUTF16::getChars_i_i_i_arr_b(i, count, spaceNeeded, Clone::clone(&this.__get_value()))?;
            }
            this.__set_count(spaceNeeded);
            Ok(Clone::clone(this))
        }

        #[java_method(name = "append", descriptor = "(J)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: append(J)Ljava/lang/AbstractStringBuilder;
        pub fn append_l(&self, mut l: i64) -> Result<AbstractStringBuilder> {
            let this = self;
            let mut count = this.__get_count();
            let _t0: i32 = Long::stringSize(l)?;
            let mut spaceNeeded = (count).wrapping_add(_t0);
            this.ensureCapacityInternal(spaceNeeded)?;
            let _t1 = this.isLatin1()?;
            if _t1 {
                let _t2: i32 = Long::getChars(l, spaceNeeded, Clone::clone(&this.__get_value()))?;
            } else {
                let _t2: i32 = StringUTF16::getChars_l_i_i_arr_b(l, count, spaceNeeded, Clone::clone(&this.__get_value()))?;
            }
            this.__set_count(spaceNeeded);
            Ok(Clone::clone(this))
        }

        #[java_method(name = "append", descriptor = "(F)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: append(F)Ljava/lang/AbstractStringBuilder;
        pub fn append_f(&self, mut f: f32) -> Result<AbstractStringBuilder> {
            let this = self;
            let _t0: Object = FloatToDecimal::appendTo(f, Object::from_any(Clone::clone(self)))?;
            Ok(Clone::clone(this))
        }

        #[java_method(name = "append", descriptor = "(D)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn append_d(&self, d: f64) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.append:(D)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "delete", descriptor = "(II)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn delete(&self, mut start: i32, mut end: i32) -> Result<AbstractStringBuilder> {
            let this = self;
            let mut count = this.__get_count();
            if end > count {
                end = count;
            }
            let _t0: i32 = Preconditions::checkFromToIndex_i_i_i_bifunc(start, end, count, Clone::clone(&Preconditions::SIOOBE_FORMATTER()))?;
            let mut len = (end).wrapping_sub(start);
            if (len>0) {
                this.shift(end, (len).wrapping_neg())?;
                this.__set_count((count).wrapping_sub(len));
                this.__set_maybeLatin1((1i32 != 0i32));
            }
            Ok(Clone::clone(this))
        }

        #[java_method(name = "appendCodePoint", descriptor = "(I)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn appendCodePoint(&self, mut codePoint: i32) -> Result<AbstractStringBuilder> {
            let this = self;
            let _t0: bool = Character::isBmpCodePoint(codePoint)?;
            if _t0 {
                let _t1 = this.append_c(((((codePoint) as u16 as i32)) as u16))?;
                return Ok(_t1);
            }
            let _t1: Rc<RefCell<Vec<u16>>> = Character::toChars_i(codePoint)?;
            let _t2 = this.append_arr_c(Clone::clone(&_t1))?;
            Ok(_t2)
        }

        #[java_method(name = "deleteCharAt", descriptor = "(I)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn deleteCharAt(&self, index: i32) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.deleteCharAt:(I)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "replace", descriptor = "(IILjava/lang/String;)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn replace(&self, start: i32, end: i32, str: String) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.replace:(IILjava/lang/String;)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "substring", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn substring_i(&self, start: i32) -> Result<String> {
            panic!("stub: java/lang/AbstractStringBuilder.substring:(I)Ljava/lang/String;")
        }

        #[java_method(name = "subSequence", descriptor = "(II)Ljava/lang/CharSequence;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn subSequence(&self, mut start: i32, mut end: i32) -> Result<Object> {
            let this = self;
            let _t0 = this.substring_i_i(start, end)?;
            Ok(Object::from_any(_t0.clone()))
        }

        #[java_method(name = "substring", descriptor = "(II)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: substring(II)Ljava/lang/String;
        pub fn substring_i_i(&self, mut start: i32, mut end: i32) -> Result<String> {
            let this = self;
            let _t0: i32 = Preconditions::checkFromToIndex_i_i_i_bifunc(start, end, this.__get_count(), Clone::clone(&Preconditions::SIOOBE_FORMATTER()))?;
            let _t1 = this.isLatin1()?;
            if _t1 {
                let _t2: String = StringLatin1::newString(Clone::clone(&this.__get_value()), start, (end).wrapping_sub(start))?;
                return Ok(_t2);
            }
            let _t2: String = StringUTF16::newString(Clone::clone(&this.__get_value()), start, (end).wrapping_sub(start))?;
            Ok(_t2)
        }

        #[java_method(name = "shift", descriptor = "(II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn shift(&self, mut offset: i32, mut n: i32) -> Result<()> {
            let this = self;
            System::arraycopy(Object::from_any(this.__get_value().clone()), (offset<<((this.__get_coder() as i32)&0x1f)), Object::from_any(this.__get_value().clone()), ((offset).wrapping_add(n)<<((this.__get_coder() as i32)&0x1f)), ((this.__get_count()).wrapping_sub(offset)<<((this.__get_coder() as i32)&0x1f)))?;
            Ok(())
        }

        #[java_method(name = "insert", descriptor = "(I[CII)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn insert_i_arr_c_i_i(&self, index: i32, str: Rc<RefCell<Vec<u16>>>, offset: i32, len: i32) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.insert:(I[CII)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "insert", descriptor = "(ILjava/lang/Object;)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn insert_i_obj(&self, offset: i32, obj: Object) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.insert:(ILjava/lang/Object;)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "insert", descriptor = "(ILjava/lang/String;)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: insert(ILjava/lang/String;)Ljava/lang/AbstractStringBuilder;
        pub fn insert_i_str(&self, mut offset: i32, mut str: String) -> Result<AbstractStringBuilder> {
            let this = self;
            String::checkOffset(offset, this.__get_count())?;
            if _is_jnull(&str) {
                str = String::from("null");
            }
            let _t0 = str.length()?;
            let mut len: i32 = _t0;
            this.ensureCapacityInternal((this.__get_count()).wrapping_add(len))?;
            this.shift(offset, len)?;
            this.__set_count((this.__get_count()).wrapping_add(len));
            this.putStringAt(offset, Clone::clone(&str))?;
            Ok(Clone::clone(this))
        }

        #[java_method(name = "insert", descriptor = "(I[C)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn insert_i_arr_c(&self, offset: i32, str: Rc<RefCell<Vec<u16>>>) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.insert:(I[C)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "insert", descriptor = "(ILjava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn insert_i_seq(&self, dstOffset: i32, s: Object) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.insert:(ILjava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "insert", descriptor = "(ILjava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn insert_i_seq_i_i(&self, dstOffset: i32, s: Object, start: i32, end: i32) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.insert:(ILjava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "insert", descriptor = "(IZ)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn insert_i_z(&self, offset: i32, b: bool) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.insert:(IZ)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "insert", descriptor = "(IC)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn insert_i_c(&self, offset: i32, c: u16) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.insert:(IC)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "insert", descriptor = "(II)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn insert_i_i(&self, offset: i32, i: i32) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.insert:(II)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "insert", descriptor = "(IJ)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn insert_i_l(&self, offset: i32, l: i64) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.insert:(IJ)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "insert", descriptor = "(IF)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn insert_i_f(&self, offset: i32, f: f32) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.insert:(IF)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "insert", descriptor = "(ID)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn insert_i_d(&self, offset: i32, d: f64) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.insert:(ID)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "indexOf", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOf_str(&self, str: String) -> Result<i32> {
            panic!("stub: java/lang/AbstractStringBuilder.indexOf:(Ljava/lang/String;)I")
        }

        #[java_method(name = "indexOf", descriptor = "(Ljava/lang/String;I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOf_str_i(&self, str: String, fromIndex: i32) -> Result<i32> {
            panic!("stub: java/lang/AbstractStringBuilder.indexOf:(Ljava/lang/String;I)I")
        }

        #[java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lastIndexOf_str(&self, str: String) -> Result<i32> {
            panic!("stub: java/lang/AbstractStringBuilder.lastIndexOf:(Ljava/lang/String;)I")
        }

        #[java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/String;I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lastIndexOf_str_i(&self, str: String, fromIndex: i32) -> Result<i32> {
            panic!("stub: java/lang/AbstractStringBuilder.lastIndexOf:(Ljava/lang/String;I)I")
        }

        #[java_method(name = "reverse", descriptor = "()Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reverse(&self) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.reverse:()Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            panic!("stub: java/lang/AbstractStringBuilder.toString:()Ljava/lang/String;")
        }

        #[java_method(name = "chars", descriptor = "()Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn chars(&self) -> Result<Object> {
            panic!("stub: java/lang/AbstractStringBuilder.chars:()Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "codePoints", descriptor = "()Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePoints(&self) -> Result<Object> {
            panic!("stub: java/lang/AbstractStringBuilder.codePoints:()Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "getValue", descriptor = "()[B", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getValue(&self) -> Result<Rc<RefCell<Vec<i8>>>> {
            let this = self;
            Ok(this.__get_value())
        }

        #[java_method(name = "getBytes", descriptor = "([BIB)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBytes(&self, mut dst: Rc<RefCell<Vec<i8>>>, mut dstBegin: i32, mut coder: i8) -> Result<()> {
            let this = self;
            if (this.__get_coder() as i32) == (coder as i32) {
                System::arraycopy(Object::from_any(this.__get_value().clone()), 0i32, Object::from_any(dst.clone()), (dstBegin<<((coder as i32)&0x1f)), (this.__get_count()<<((coder as i32)&0x1f)))?;
            } else {
                StringLatin1::inflate_arr_b_i_arr_b_i_i(Clone::clone(&this.__get_value()), 0i32, Clone::clone(&dst), dstBegin, this.__get_count())?;
            }
            Ok(())
        }

        #[java_method(name = "initBytes", descriptor = "([CII)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn initBytes(&self, value: Rc<RefCell<Vec<u16>>>, off: i32, len: i32) -> Result<()> {
            panic!("stub: java/lang/AbstractStringBuilder.initBytes:([CII)V")
        }

        #[java_method(name = "getCoder", descriptor = "()B", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCoder(&self) -> Result<i8> {
            let this = self;
            Ok((if String::COMPACT_STRINGS() { this.__get_coder() } else { (1i32 as i8) }))
        }

        #[java_method(name = "isLatin1", descriptor = "()Z", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLatin1(&self) -> Result<bool> {
            let this = self;
            Ok((if String::COMPACT_STRINGS() { (this.__get_coder()==0) } else { (0i32 != 0) }))
        }

        #[java_method(name = "putCharsAt", descriptor = "(I[CII)V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putCharsAt_i_arr_c_i_i(&self, index: i32, s: Rc<RefCell<Vec<u16>>>, off: i32, end: i32) -> Result<()> {
            panic!("stub: java/lang/AbstractStringBuilder.putCharsAt:(I[CII)V")
        }

        #[java_method(name = "putCharsAt", descriptor = "(ILjava/lang/CharSequence;II)V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putCharsAt_i_seq_i_i(&self, index: i32, s: Object, off: i32, end: i32) -> Result<()> {
            panic!("stub: java/lang/AbstractStringBuilder.putCharsAt:(ILjava/lang/CharSequence;II)V")
        }

        #[java_method(name = "inflateIfNeededFor", descriptor = "(Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: inflateIfNeededFor(Ljava/lang/String;)V
        pub fn inflateIfNeededFor_str(&self, mut input: String) -> Result<()> {
            let this = self;
            let _t0 = input.coder()?;
            if (this.__get_coder() as i32) != (_t0 as i32) {
                this.inflate()?;
            }
            Ok(())
        }

        #[java_method(name = "inflateIfNeededFor", descriptor = "(Ljava/lang/AbstractStringBuilder;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: inflateIfNeededFor(Ljava/lang/AbstractStringBuilder;)V
        pub fn inflateIfNeededFor_abstra(&self, mut input: AbstractStringBuilder) -> Result<()> {
            let this = self;
            let _t0 = input.getCoder()?;
            if (this.__get_coder() as i32) != (_t0 as i32) {
                this.inflate()?;
            }
            Ok(())
        }

        #[java_method(name = "putStringAt", descriptor = "(ILjava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn putStringAt(&self, mut index: i32, mut str: String) -> Result<()> {
            let this = self;
            this.inflateIfNeededFor_str(Clone::clone(&str))?;
            str.getBytes_arr_b_i_b(Clone::clone(&this.__get_value()), index, this.__get_coder())?;
            Ok(())
        }

        #[java_method(name = "appendChars", descriptor = "([CII)V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: appendChars([CII)V
        pub fn appendChars_arr_c_i_i(&self, mut s: Rc<RefCell<Vec<u16>>>, mut off: i32, mut end: i32) -> Result<()> {
            let this = self;
            let mut count = this.__get_count();
            let _t0 = this.isLatin1()?;
            if _t0 {
                let mut val = this.__get_value();
                let mut i: i32 = off;
                let mut j: i32 = count;
                loop {
                    if i >= end { break; }
                    let mut c = (s.borrow()[i as usize] as i32);
                    let _t1: bool = StringLatin1::canEncode_c(((c) as u16))?;
                    if _t1 {
                        j = j.wrapping_add(1i32);
                        val.borrow_mut()[j as usize] = (((c) as i8 as i32)) as i8;
                    } else {
                        count = j;
                        this.__set_count(count);
                        this.inflate()?;
                        StringUTF16::putCharsSB_arr_b_i_arr_c_i_i(Clone::clone(&this.__get_value()), j, Clone::clone(&s), i, end)?;
                        this.__set_count(((count).wrapping_add(end)).wrapping_sub(i));
                        return Ok(());
                    }
                    i = i.wrapping_add(1i32);
                }
            } else {
                StringUTF16::putCharsSB_arr_b_i_arr_c_i_i(Clone::clone(&this.__get_value()), count, Clone::clone(&s), off, end)?;
            }
            this.__set_count(((count).wrapping_add(end)).wrapping_sub(off));
            Ok(())
        }

        #[java_method(name = "appendChars", descriptor = "(Ljava/lang/String;II)V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: appendChars(Ljava/lang/String;II)V
        pub fn appendChars_str_i_i(&self, mut s: String, mut off: i32, mut end: i32) -> Result<()> {
            let this = self;
            let _t0 = this.isLatin1()?;
            if _t0 {
                let _t1 = s.isLatin1()?;
                if _t1 {
                    let _t2 = s.value()?;
                    System::arraycopy(Object::from_any(_t2.clone()), off, Object::from_any(this.__get_value().clone()), this.__get_count(), (end).wrapping_sub(off))?;
                } else {
                    let mut val = this.__get_value();
                    let mut i: i32 = off;
                    let mut j = this.__get_count();
                    loop {
                        if i >= end { break; }
                        let _t2 = s.charAt(i)?;
                        let mut c: u16 = _t2;
                        let _t3: bool = StringLatin1::canEncode_c(c)?;
                        if _t3 {
                            j = j.wrapping_add(1i32);
                            val.borrow_mut()[j as usize] = (((c) as i8 as i32)) as i8;
                        } else {
                            this.__set_count(j);
                            this.inflate()?;
                            let _t4 = s.value()?;
                            System::arraycopy(Object::from_any(_t4.clone()), (i<<(1i32&0x1f)), Object::from_any(this.__get_value().clone()), (j<<(1i32&0x1f)), ((end).wrapping_sub(i)<<(1i32&0x1f)))?;
                            this.__set_count((this.__get_count()).wrapping_add((end).wrapping_sub(i)));
                            return Ok(());
                        }
                        i = i.wrapping_add(1i32);
                    }
                }
            } else {
                let _t1 = s.isLatin1()?;
                if _t1 {
                    StringUTF16::putCharsSB_arr_b_i_seq_i_i(Clone::clone(&this.__get_value()), this.__get_count(), Object::from_any(s.clone()), off, end)?;
                } else {
                    let _t2 = s.value()?;
                    System::arraycopy(Object::from_any(_t2.clone()), (off<<(1i32&0x1f)), Object::from_any(this.__get_value().clone()), (this.__get_count()<<(1i32&0x1f)), ((end).wrapping_sub(off)<<(1i32&0x1f)))?;
                }
            }
            this.__set_count((this.__get_count()).wrapping_add((end).wrapping_sub(off)));
            Ok(())
        }

        #[java_method(name = "appendChars", descriptor = "(Ljava/lang/CharSequence;II)V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: appendChars(Ljava/lang/CharSequence;II)V
        pub fn appendChars_seq_i_i(&self, mut s: Object, mut off: i32, mut end: i32) -> Result<()> {
            let this = self;
            let _t0 = this.isLatin1()?;
            if _t0 {
                let mut val = this.__get_value();
                let mut i: i32 = off;
                let mut j = this.__get_count();
                loop {
                    if i >= end { break; }
                    let _vdispatch1: u16 = if let Some(_d) = s.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<CharBuffer>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<String>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<StringBuilder>() { _d.charAt(i)? } else if let Some(_d) = s.0.as_any().downcast_ref::<Object>() { _d.charAt(i)? } else if let Some(__f) = s.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<u16>>>() { (__f)(i)? } else { Default::default() };
                    let mut c: u16 = _vdispatch1;
                    let _t2: bool = StringLatin1::canEncode_c(c)?;
                    if _t2 {
                        j = j.wrapping_add(1i32);
                        val.borrow_mut()[j as usize] = (((c) as i8 as i32)) as i8;
                    } else {
                        this.__set_count(j);
                        this.inflate()?;
                        j = j.wrapping_add(1i32);
                        StringUTF16::putCharSB(Clone::clone(&this.__get_value()), j, (c as i32))?;
                        this.__set_count(j);
                        i = i.wrapping_add(1i32);
                        StringUTF16::putCharsSB_arr_b_i_seq_i_i(Clone::clone(&this.__get_value()), j, Clone::clone(&s), i, end)?;
                        this.__set_count((this.__get_count()).wrapping_add((end).wrapping_sub(i)));
                        return Ok(());
                    }
                    i = i.wrapping_add(1i32);
                }
            } else {
                StringUTF16::putCharsSB_arr_b_i_seq_i_i(Clone::clone(&this.__get_value()), this.__get_count(), Clone::clone(&s), off, end)?;
            }
            this.__set_count((this.__get_count()).wrapping_add((end).wrapping_sub(off)));
            Ok(())
        }

        #[java_method(name = "mix", descriptor = "(J)J", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mix(&self, lengthCoder: i64) -> Result<i64> {
            panic!("stub: java/lang/AbstractStringBuilder.mix:(J)J")
        }

        #[java_method(name = "prepend", descriptor = "(J[B)J", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn prepend(&self, lengthCoder: i64, arg1: Rc<RefCell<Vec<i8>>>) -> Result<i64> {
            panic!("stub: java/lang/AbstractStringBuilder.prepend:(J[B)J")
        }

        #[java_method(name = "repeat", descriptor = "(CI)Ljava/lang/AbstractStringBuilder;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn repeat_c_i(&self, c: u16, count: i32) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.repeat:(CI)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "repeat", descriptor = "(II)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn repeat_i_i(&self, codePoint: i32, count: i32) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.repeat:(II)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "repeat", descriptor = "(Ljava/lang/CharSequence;I)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn repeat_seq_i(&self, cs: Object, count: i32) -> Result<AbstractStringBuilder> {
            panic!("stub: java/lang/AbstractStringBuilder.repeat:(Ljava/lang/CharSequence;I)Ljava/lang/AbstractStringBuilder;")
        }

        #[java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmpty(&self) -> Result<bool> {
            panic!("stub: java/lang/AbstractStringBuilder.isEmpty:()Z")
        }
    }
}
