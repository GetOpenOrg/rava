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
use crate::jdk::internal::util::ArraysSupport;
use crate::jdk::internal::util::Preconditions;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/String"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Serializable,java/lang/Comparable,java/lang/CharSequence,java/lang/constant/Constable,java/lang/constant/ConstantDesc"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = "Ljava/lang/Object;Ljava/io/Serializable;Ljava/lang/Comparable<Ljava/lang/String;>;Ljava/lang/CharSequence;Ljava/lang/constant/Constable;Ljava/lang/constant/ConstantDesc;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "String.java"]
    #[inner_classes     = "java/lang/StringLatin1$CharsSpliterator:java/lang/StringLatin1:CharsSpliterator:8;java/lang/StringUTF16$CharsSpliterator:java/lang/StringUTF16:CharsSpliterator:8;java/util/Spliterator$OfInt:java/util/Spliterator:OfInt:1545;java/lang/StringUTF16$CodePointsSpliterator:java/lang/StringUTF16:CodePointsSpliterator:8;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25;java/lang/String$CaseInsensitiveComparator:java/lang/String:CaseInsensitiveComparator:10"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/CharSequence;java/lang/Comparable;java/lang/Object;java/lang/String;java/lang/constant/Constable;java/lang/constant/ConstantDesc"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct String {
        #[cfg_attr(any(), java_field(name = "value", descriptor = "[B", access = "private", modifiers = "final", is_static = false))]
        pub value: Rc<RefCell<Vec<i8>>>,
        #[cfg_attr(any(), java_field(name = "coder", descriptor = "B", access = "private", modifiers = "final", is_static = false))]
        pub coder: i8,
        #[cfg_attr(any(), java_field(name = "hash", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub hash: i32,
        #[cfg_attr(any(), java_field(name = "hashIsZero", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub hashIsZero: bool,
    }

    impl String {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-6849794470754667710"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -6849794470754667710i64
        }

        #[cfg_attr(any(), java_field(name = "COMPACT_STRINGS", descriptor = "Z", access = "package", modifiers = "static final", is_static = true))]
        // static field: COMPACT_STRINGS:Z
        pub fn COMPACT_STRINGS() -> bool {
            true
        }

        #[cfg_attr(any(), java_field(name = "serialPersistentFields", descriptor = "[Ljava/io/ObjectStreamField;", access = "private", modifiers = "static final", is_static = true))]
        // static field: serialPersistentFields:[Ljava/io/ObjectStreamField;
        pub fn serialPersistentFields() -> Rc<RefCell<Vec<Object>>> {
            Rc::new(RefCell::new(Vec::new()))
        }

        #[cfg_attr(any(), java_field(name = "REPL", descriptor = "C", access = "private", modifiers = "static final", is_static = true, constant_value = "65533"))]
        // static field: REPL:C
        pub fn REPL() -> u16 {
            65533
        }

        #[cfg_attr(any(), java_field(name = "CASE_INSENSITIVE_ORDER", descriptor = "Ljava/util/Comparator;", access = "public", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Comparator<Ljava/lang/String;>;"))]
        // static field: CASE_INSENSITIVE_ORDER:Ljava/util/Comparator;
        pub fn CASE_INSENSITIVE_ORDER() -> Object {
            panic!("stub: java/lang/String.CASE_INSENSITIVE_ORDER:Ljava/util/Comparator;")
        }

        #[cfg_attr(any(), java_field(name = "LATIN1", descriptor = "B", access = "package", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: LATIN1:B
        pub fn LATIN1() -> i8 {
            0
        }

        #[cfg_attr(any(), java_field(name = "UTF16", descriptor = "B", access = "package", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: UTF16:B
        pub fn UTF16() -> i8 {
            1
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_value(Clone::clone(&String::from("").__get_value()));
            this.__set_coder(String::from("").__get_coder());
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/String;)V
        pub fn new_str(mut original: String) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_value(Clone::clone(&original.__get_value()));
            this.__set_coder(original.__get_coder());
            this.__set_hash(original.__get_hash());
            this.__set_hashIsZero(original.__get_hashIsZero());
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "([C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>([C)V
        pub fn new_arr_c(mut value: Rc<RefCell<Vec<u16>>>) -> Result<Self> {
            let mut this = Self::default();
            this = String::new_arr_c_i_i_void(Clone::clone(&value), 0i32, (value.borrow().len() as i32), Clone::clone(&Object::default()))?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "([CII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_arr_c_i_i(value: Rc<RefCell<Vec<u16>>>, offset: i32, count: i32) -> Result<Self> {
            panic!("stub: java/lang/String.<init>:([CII)V")
        }

        #[java_method(name = "rangeCheck", descriptor = "([CII)Ljava/lang/Void;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn rangeCheck(value: Rc<RefCell<Vec<u16>>>, offset: i32, count: i32) -> Result<Object> {
            panic!("stub: java/lang/String.rangeCheck:([CII)Ljava/lang/Void;")
        }

        #[java_method(name = "<init>", descriptor = "([III)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>([III)V
        pub fn new_arr_i_i_i(mut codePoints: Rc<RefCell<Vec<i32>>>, mut offset: i32, mut count: i32) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            let _t0: i32 = String::checkBoundsOffCount(offset, count, (codePoints.borrow().len() as i32))?;
            if (count==0) {
                this.__set_value(Clone::clone(&String::from("").__get_value()));
                this.__set_coder(String::from("").__get_coder());
                return Ok(this);
            }
            if String::COMPACT_STRINGS() {
                let _t1: Rc<RefCell<Vec<i8>>> = StringUTF16::compress_arr_i_i_i(Clone::clone(&codePoints), offset, count)?;
                let mut val: Rc<RefCell<Vec<i8>>> = _t1;
                let _t2: i8 = StringUTF16::coderFromArrayLen(Clone::clone(&val), count)?;
                this.__set_coder(_t2);
                this.__set_value(Clone::clone(&val));
                return Ok(this);
            }
            this.__set_coder(((1i32) as i8));
            let _t1: Rc<RefCell<Vec<i8>>> = StringUTF16::toBytes_arr_i_i_i(Clone::clone(&codePoints), offset, count)?;
            this.__set_value(Clone::clone(&_t1));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "([BIII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        // java: <init>([BIII)V
        pub fn new_arr_b_i_i_i(mut ascii: Rc<RefCell<Vec<i8>>>, mut hibyte: i32, mut offset: i32, mut count: i32) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            let _t0: i32 = String::checkBoundsOffCount(offset, count, (ascii.borrow().len() as i32))?;
            if (count==0) {
                this.__set_value(Clone::clone(&String::from("").__get_value()));
                this.__set_coder(String::from("").__get_coder());
                return Ok(this);
            }
            if String::COMPACT_STRINGS() {
                if (((hibyte) as i8 as i32)==0) {
                    let _t1: Rc<RefCell<Vec<i8>>> = Arrays::copyOfRange_arr_b_i_i(Clone::clone(&ascii), offset, (offset).wrapping_add(count))?;
                    this.__set_value(Clone::clone(&_t1));
                    this.__set_coder(((0i32) as i8));
                } else {
                    hibyte = (hibyte<<(8i32&0x1f));
                    let _t1: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor(count)?;
                    let mut val: Rc<RefCell<Vec<i8>>> = _t1;
                    let mut i: i32 = 0i32;
                    loop {
                        if i >= count { break; }
                        offset = offset.wrapping_add(1i32);
                        StringUTF16::putChar(Clone::clone(&val), i, (hibyte|((ascii.borrow()[offset as usize] as i32)&255i32)))?;
                        i = i.wrapping_add(1i32);
                    }
                    this.__set_value(Clone::clone(&val));
                    this.__set_coder(((1i32) as i8));
                }
            } else {
                hibyte = (hibyte<<(8i32&0x1f));
                let _t1: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor(count)?;
                let mut val: Rc<RefCell<Vec<i8>>> = _t1;
                let mut i: i32 = 0i32;
                loop {
                    if i >= count { break; }
                    offset = offset.wrapping_add(1i32);
                    StringUTF16::putChar(Clone::clone(&val), i, (hibyte|((ascii.borrow()[offset as usize] as i32)&255i32)))?;
                    i = i.wrapping_add(1i32);
                }
                this.__set_value(Clone::clone(&val));
                this.__set_coder(((1i32) as i8));
            }
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "([BI)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn new_arr_b_i(ascii: Rc<RefCell<Vec<i8>>>, hibyte: i32) -> Result<Self> {
            panic!("stub: java/lang/String.<init>:([BI)V")
        }

        #[java_method(name = "<init>", descriptor = "([BIILjava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/UnsupportedEncodingException")]
        pub fn new_arr_b_i_i_str(bytes: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32, charsetName: String) -> Result<Self> {
            panic!("stub: java/lang/String.<init>:([BIILjava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "([BIILjava/nio/charset/Charset;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_arr_b_i_i_charse(bytes: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32, charset: Charset) -> Result<Self> {
            panic!("stub: java/lang/String.<init>:([BIILjava/nio/charset/Charset;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/nio/charset/Charset;[BII)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_charse_arr_b_i_i(charset: Charset, bytes: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32) -> Result<Self> {
            panic!("stub: java/lang/String.<init>:(Ljava/nio/charset/Charset;[BII)V")
        }

        #[java_method(name = "newStringUTF8NoRepl", descriptor = "([BIIZ)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newStringUTF8NoRepl(bytes: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32, noShare: bool) -> Result<String> {
            panic!("stub: java/lang/String.newStringUTF8NoRepl:([BIIZ)Ljava/lang/String;")
        }

        #[java_method(name = "newStringNoRepl", descriptor = "([BLjava/nio/charset/Charset;)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/nio/charset/CharacterCodingException")]
        pub fn newStringNoRepl(src: Rc<RefCell<Vec<i8>>>, cs: Charset) -> Result<String> {
            panic!("stub: java/lang/String.newStringNoRepl:([BLjava/nio/charset/Charset;)Ljava/lang/String;")
        }

        #[java_method(name = "newStringNoRepl1", descriptor = "([BLjava/nio/charset/Charset;)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newStringNoRepl1(src: Rc<RefCell<Vec<i8>>>, cs: Charset) -> Result<String> {
            panic!("stub: java/lang/String.newStringNoRepl1:([BLjava/nio/charset/Charset;)Ljava/lang/String;")
        }

        #[java_method(name = "safeTrim", descriptor = "([BIZ)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn safeTrim(mut ba: Rc<RefCell<Vec<i8>>>, mut len: i32, mut isTrusted: bool) -> Result<Rc<RefCell<Vec<i8>>>> {
            let _t0: SecurityManager = System::getSecurityManager()?;
            if _is_jnull(&_t0) {
                return Ok(ba);
            }
            let _t1: Rc<RefCell<Vec<i8>>> = Arrays::copyOf_arr_b_i(Clone::clone(&ba), len)?;
            Ok(_t1)
        }

        #[java_method(name = "scale", descriptor = "(IF)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn scale(mut len: i32, mut expansionFactor: f32) -> Result<i32> {
            Ok((((len as f64)*(expansionFactor as f64)) as i32))
        }

        #[java_method(name = "lookupCharset", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/Charset;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/UnsupportedEncodingException")]
        pub fn lookupCharset(csn: String) -> Result<Charset> {
            panic!("stub: java/lang/String.lookupCharset:(Ljava/lang/String;)Ljava/nio/charset/Charset;")
        }

        #[java_method(name = "encode", descriptor = "(Ljava/nio/charset/Charset;B[B)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn encode(mut cs: Charset, mut coder: i8, mut val: Rc<RefCell<Vec<i8>>>) -> Result<Rc<RefCell<Vec<i8>>>> {
            if Object::from_any(cs.clone()) == Object::from_any(UTF_8::INSTANCE().clone()) {
                let _t0: Rc<RefCell<Vec<i8>>> = String::encodeUTF8(coder, Clone::clone(&val), (1i32 != 0i32))?;
                return Ok(_t0);
            }
            if Object::from_any(cs.clone()) == Object::from_any(ISO_8859_1::INSTANCE().clone()) {
                let _t0: Rc<RefCell<Vec<i8>>> = String::encode8859_1_b_arr_b(coder, Clone::clone(&val))?;
                return Ok(_t0);
            }
            if Object::from_any(cs.clone()) == Object::from_any(US_ASCII::INSTANCE().clone()) {
                let _t0: Rc<RefCell<Vec<i8>>> = String::encodeASCII(coder, Clone::clone(&val))?;
                return Ok(_t0);
            }
            let _t0: Rc<RefCell<Vec<i8>>> = String::encodeWithEncoder(Clone::clone(&cs), coder, Clone::clone(&val), (1i32 != 0i32))?;
            Ok(_t0)
        }

        #[java_method(name = "encodeWithEncoder", descriptor = "(Ljava/nio/charset/Charset;B[BZ)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn encodeWithEncoder(mut cs: Charset, mut coder: i8, mut val: Rc<RefCell<Vec<i8>>>, mut doReplace: bool) -> Result<Rc<RefCell<Vec<i8>>>> {
            let _t0 = cs.newEncoder()?;
            let mut ce: CharsetEncoder = _t0;
            let mut len = ((val.borrow().len() as i32)>>(((coder as i32)&0x1f)));
            let _t1 = ce.maxBytesPerChar()?;
            let _t2: i32 = String::scale(len, _t1)?;
            let mut en: i32 = _t2;
            let mut ae: Object = ce;
            let _vdispatch3: bool = if let Some(__f) = ae.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<bool>>>() { (__f)()? } else { Default::default() };
            let _t4: bool = StringCoding::hasNegatives(Clone::clone(&val), 0i32, (val.borrow().len() as i32))?;
            if !(_t4) {
                let _t5: Object = Object::from_any(val.clone());
                return Ok((_t5).downcast::<Rc<RefCell<Vec<i8>>>>());
            }
            let mut _arr5: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; en as usize]));
            let mut ba: Rc<RefCell<Vec<i8>>> = _arr5;
            if (len==0) {
                return Ok(ba);
            }
            let mut _merged7: i32;
            if (coder==0) {
                let _vdispatch6: i32 = if let Some(__f) = ae.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Rc<RefCell<Vec<i8>>>, i32, i32, Rc<RefCell<Vec<i8>>>) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&val), 0i32, len, Clone::clone(&ba))? } else { Default::default() };
                _merged7 = _vdispatch6;
            } else {
                let _vdispatch6: i32 = if let Some(__f) = ae.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Rc<RefCell<Vec<i8>>>, i32, i32, Rc<RefCell<Vec<i8>>>) -> crate::error::Result<i32>>>() { (__f)(Clone::clone(&val), 0i32, len, Clone::clone(&ba))? } else { Default::default() };
                _merged7 = _vdispatch6;
            }
            let mut blen: i32 = _merged7;
            if blen != -1i32 {
                let _t8: Rc<RefCell<Vec<i8>>> = String::safeTrim(Clone::clone(&ba), blen, (1i32 != 0i32))?;
                return Ok(_t8);
            }
            let mut _arr8: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; en as usize]));
            let mut ae: Rc<RefCell<Vec<i8>>> = _arr8;
            if (len==0) {
                return Ok(ae);
            }
            if doReplace {
                let _t9 = ce.onMalformedInput(Clone::clone(&CodingErrorAction::REPLACE()))?;
                let _t10 = _t9.onUnmappableCharacter(Clone::clone(&CodingErrorAction::REPLACE()))?;
            }
            let mut _merged10: Rc<RefCell<Vec<u16>>>;
            if (coder==0) {
                let _t9: Rc<RefCell<Vec<u16>>> = StringLatin1::toChars(Clone::clone(&val))?;
                _merged10 = _t9;
            } else {
                let _t9: Rc<RefCell<Vec<u16>>> = StringUTF16::toChars(Clone::clone(&val))?;
                _merged10 = _t9;
            }
            let mut ba: Rc<RefCell<Vec<u16>>> = _merged10;
            let _t11: ByteBuffer = ByteBuffer::wrap_arr_b(Clone::clone(&ae))?;
            let mut blen: ByteBuffer = _t11;
            let _t12: CharBuffer = CharBuffer::wrap_arr_c_i_i(Clone::clone(&ba), 0i32, len)?;
            let mut cb: CharBuffer = _t12;
            let _t13 = ce.encode_charbu_bytebu_z(Clone::clone(&cb), Clone::clone(&blen), (1i32 != 0i32))?;
            let mut cr: CoderResult = _t13;
            let _t14 = cr.isUnderflow()?;
            if !(_t14) {
                cr.throwException()?;
            }
            let _t15 = ce.flush(Clone::clone(&blen))?;
            cr = _t15;
            let _t16 = cr.isUnderflow()?;
            if !(_t16) {
                cr.throwException()?;
            }
            let _t17 = blen.__super().position()?;
            let _t18 = cs.getClass()?;
            let _vdispatch19: ClassLoader = if let Some(__f) = _t18.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<ClassLoader>>>() { (__f)()? } else { Default::default() };
            let _t20: Rc<RefCell<Vec<i8>>> = String::safeTrim(Clone::clone(&ae), _t17, _is_jnull(&_vdispatch19))?;
            Ok(_t20)
        }

        #[java_method(name = "getBytesUTF8NoRepl", descriptor = "(Ljava/lang/String;)[B", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBytesUTF8NoRepl(s: String) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/lang/String.getBytesUTF8NoRepl:(Ljava/lang/String;)[B")
        }

        #[java_method(name = "isASCII", descriptor = "([B)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isASCII(src: Rc<RefCell<Vec<i8>>>) -> Result<bool> {
            panic!("stub: java/lang/String.isASCII:([B)Z")
        }

        #[java_method(name = "getBytesNoRepl", descriptor = "(Ljava/lang/String;Ljava/nio/charset/Charset;)[B", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/nio/charset/CharacterCodingException")]
        pub fn getBytesNoRepl(s: String, cs: Charset) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/lang/String.getBytesNoRepl:(Ljava/lang/String;Ljava/nio/charset/Charset;)[B")
        }

        #[java_method(name = "getBytesNoRepl1", descriptor = "(Ljava/lang/String;Ljava/nio/charset/Charset;)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBytesNoRepl1(s: String, cs: Charset) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/lang/String.getBytesNoRepl1:(Ljava/lang/String;Ljava/nio/charset/Charset;)[B")
        }

        #[java_method(name = "encodeASCII", descriptor = "(B[B)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn encodeASCII(mut coder: i8, mut val: Rc<RefCell<Vec<i8>>>) -> Result<Rc<RefCell<Vec<i8>>>> {
            let _t0: i32 = StringCoding::countPositives(Clone::clone(&val), 0i32, (val.borrow().len() as i32))?;
            let mut positives: i32 = _t0;
            let _t1: Object = Object::from_any(val.clone());
            let mut dst = (_t1).downcast::<Rc<RefCell<Vec<i8>>>>();
            if positives < (dst.borrow().len() as i32) {
                String::replaceNegatives(Clone::clone(&dst), positives)?;
            }
            return Ok(dst);
            positives = ((val.borrow().len() as i32)>>((1i32&0x1f)));
            let mut _arr2: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; positives as usize]));
            dst = _arr2;
            let mut dp: i32 = 0i32;
            let mut i: i32 = 0i32;
            loop {
                if i >= positives { break; }
                let _t3: u16 = StringUTF16::getChar(Clone::clone(&val), i)?;
                let mut c: u16 = _t3;
                if (c as i32) < 128i32 {
                    dp = dp.wrapping_add(1i32);
                    dst.borrow_mut()[dp as usize] = (((c) as i8 as i32)) as i8;
                } else {
                    let _t4: bool = Character::isHighSurrogate(c)?;
                    let _t5: u16 = StringUTF16::getChar(Clone::clone(&val), (i).wrapping_add(1i32))?;
                    let _t6: bool = Character::isLowSurrogate(_t5)?;
                    if _t6 {
                        i = i.wrapping_add(1i32);
                    }
                    dp = dp.wrapping_add(1i32);
                    dst.borrow_mut()[dp as usize] = (63i32) as i8;
                }
                i = i.wrapping_add(1i32);
            }
            if positives == dp {
                return Ok(dst);
            }
            let _t3: Rc<RefCell<Vec<i8>>> = Arrays::copyOf_arr_b_i(Clone::clone(&dst), dp)?;
            Ok(_t3)
        }

        #[java_method(name = "replaceNegatives", descriptor = "([BI)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn replaceNegatives(mut val: Rc<RefCell<Vec<i8>>>, mut fromIndex: i32) -> Result<()> {
            let mut i: i32 = fromIndex;
            loop {
                if i >= (val.borrow().len() as i32) { break; }
                if ((val.borrow()[i as usize] as i32)<0) {
                    val.borrow_mut()[i as usize] = (63i32) as i8;
                }
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "encode8859_1", descriptor = "(B[B)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: encode8859_1(B[B)[B
        pub fn encode8859_1_b_arr_b(mut coder: i8, mut val: Rc<RefCell<Vec<i8>>>) -> Result<Rc<RefCell<Vec<i8>>>> {
            let _t0: Rc<RefCell<Vec<i8>>> = String::encode8859_1_b_arr_b_z(coder, Clone::clone(&val), (1i32 != 0i32))?;
            Ok(_t0)
        }

        #[java_method(name = "encode8859_1", descriptor = "(B[BZ)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: encode8859_1(B[BZ)[B
        pub fn encode8859_1_b_arr_b_z(mut coder: i8, mut val: Rc<RefCell<Vec<i8>>>, mut doReplace: bool) -> Result<Rc<RefCell<Vec<i8>>>> {
            if (coder==0) {
                let _t0: Object = Object::from_any(val.clone());
                return Ok((_t0).downcast::<Rc<RefCell<Vec<i8>>>>());
            }
            let mut len = ((val.borrow().len() as i32)>>((1i32&0x1f)));
            let mut _arr0: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; len as usize]));
            let mut dst: Rc<RefCell<Vec<i8>>> = _arr0;
            let mut dp: i32 = 0i32;
            let mut sp: i32 = 0i32;
            let mut sl: i32 = len;
            loop {
                if sp >= sl { break; }
                let _t1: i32 = StringCoding::implEncodeISOArray(Clone::clone(&val), sp, Clone::clone(&dst), dp, len)?;
                let mut ret: i32 = _t1;
                sp = (sp).wrapping_add(ret);
                dp = (dp).wrapping_add(ret);
                if !(doReplace) {
                    String::throwUnmappable_i(sp)?;
                }
                sp = sp.wrapping_add(1i32);
                let _t2: u16 = StringUTF16::getChar(Clone::clone(&val), sp)?;
                let mut c: u16 = _t2;
                let _t3: bool = Character::isHighSurrogate(c)?;
                let _t4: u16 = StringUTF16::getChar(Clone::clone(&val), sp)?;
                let _t5: bool = Character::isLowSurrogate(_t4)?;
                if _t5 {
                    sp = sp.wrapping_add(1i32);
                }
                dp = dp.wrapping_add(1i32);
                dst.borrow_mut()[dp as usize] = (63i32) as i8;
                len = (sl).wrapping_sub(sp);
            }
            if dp == (dst.borrow().len() as i32) {
                return Ok(dst);
            }
            let _t1: Rc<RefCell<Vec<i8>>> = Arrays::copyOf_arr_b_i(Clone::clone(&dst), dp)?;
            Ok(_t1)
        }

        #[java_method(name = "decodeASCII", descriptor = "([BI[CII)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn decodeASCII(sa: Rc<RefCell<Vec<i8>>>, sp: i32, da: Rc<RefCell<Vec<u16>>>, dp: i32, len: i32) -> Result<i32> {
            panic!("stub: java/lang/String.decodeASCII:([BI[CII)I")
        }

        #[java_method(name = "isNotContinuation", descriptor = "(I)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isNotContinuation(b: i32) -> Result<bool> {
            panic!("stub: java/lang/String.isNotContinuation:(I)Z")
        }

        #[java_method(name = "isMalformed3", descriptor = "(III)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isMalformed3(b1: i32, b2: i32, b3: i32) -> Result<bool> {
            panic!("stub: java/lang/String.isMalformed3:(III)Z")
        }

        #[java_method(name = "isMalformed3_2", descriptor = "(II)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isMalformed3_2(b1: i32, b2: i32) -> Result<bool> {
            panic!("stub: java/lang/String.isMalformed3_2:(II)Z")
        }

        #[java_method(name = "isMalformed4", descriptor = "(III)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isMalformed4(b2: i32, b3: i32, b4: i32) -> Result<bool> {
            panic!("stub: java/lang/String.isMalformed4:(III)Z")
        }

        #[java_method(name = "isMalformed4_2", descriptor = "(II)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isMalformed4_2(b1: i32, b2: i32) -> Result<bool> {
            panic!("stub: java/lang/String.isMalformed4_2:(II)Z")
        }

        #[java_method(name = "isMalformed4_3", descriptor = "(I)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isMalformed4_3(b3: i32) -> Result<bool> {
            panic!("stub: java/lang/String.isMalformed4_3:(I)Z")
        }

        #[java_method(name = "decode2", descriptor = "(II)C", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn decode2(b1: i32, b2: i32) -> Result<u16> {
            panic!("stub: java/lang/String.decode2:(II)C")
        }

        #[java_method(name = "decode3", descriptor = "(III)C", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn decode3(b1: i32, b2: i32, b3: i32) -> Result<u16> {
            panic!("stub: java/lang/String.decode3:(III)C")
        }

        #[java_method(name = "decode4", descriptor = "(IIII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn decode4(b1: i32, b2: i32, b3: i32, b4: i32) -> Result<i32> {
            panic!("stub: java/lang/String.decode4:(IIII)I")
        }

        #[java_method(name = "decodeUTF8_UTF16", descriptor = "([BII[BIZ)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn decodeUTF8_UTF16(src: Rc<RefCell<Vec<i8>>>, sp: i32, sl: i32, dst: Rc<RefCell<Vec<i8>>>, dp: i32, doReplace: bool) -> Result<i32> {
            panic!("stub: java/lang/String.decodeUTF8_UTF16:([BII[BIZ)I")
        }

        #[java_method(name = "decodeWithDecoder", descriptor = "(Ljava/nio/charset/CharsetDecoder;[C[BII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/nio/charset/CharacterCodingException")]
        pub fn decodeWithDecoder(cd: Object, dst: Rc<RefCell<Vec<u16>>>, src: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32) -> Result<i32> {
            panic!("stub: java/lang/String.decodeWithDecoder:(Ljava/nio/charset/CharsetDecoder;[C[BII)I")
        }

        #[java_method(name = "malformed3", descriptor = "([BI)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn malformed3(src: Rc<RefCell<Vec<i8>>>, sp: i32) -> Result<i32> {
            panic!("stub: java/lang/String.malformed3:([BI)I")
        }

        #[java_method(name = "malformed4", descriptor = "([BI)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn malformed4(src: Rc<RefCell<Vec<i8>>>, sp: i32) -> Result<i32> {
            panic!("stub: java/lang/String.malformed4:([BI)I")
        }

        #[java_method(name = "throwMalformed", descriptor = "(II)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn throwMalformed_i_i(off: i32, nb: i32) -> Result<()> {
            panic!("stub: java/lang/String.throwMalformed:(II)V")
        }

        #[java_method(name = "throwMalformed", descriptor = "([B)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn throwMalformed_arr_b(val: Rc<RefCell<Vec<i8>>>) -> Result<()> {
            panic!("stub: java/lang/String.throwMalformed:([B)V")
        }

        #[java_method(name = "throwUnmappable", descriptor = "(I)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: throwUnmappable(I)V
        pub fn throwUnmappable_i(mut off: i32) -> Result<()> {
            let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("malformed input off : ")))?;
            let _t1 = _t0.append_i(off)?;
            let _t2 = _t1.append_str(Clone::clone(&String::from(", length : 1")))?;
            let _t3 = _t2.toString()?;
            let mut msg: String = _t3;
            return Err(JvmError::Custom("athrow".to_owned()));
            Ok(())
        }

        #[java_method(name = "throwUnmappable", descriptor = "([B)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn throwUnmappable_arr_b(val: Rc<RefCell<Vec<i8>>>) -> Result<()> {
            panic!("stub: java/lang/String.throwUnmappable:([B)V")
        }

        #[java_method(name = "encodeUTF8", descriptor = "(B[BZ)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn encodeUTF8(mut coder: i8, mut val: Rc<RefCell<Vec<i8>>>, mut doReplace: bool) -> Result<Rc<RefCell<Vec<i8>>>> {
            if (coder as i32) == 1i32 {
                let _t0: Rc<RefCell<Vec<i8>>> = String::encodeUTF8_UTF16(Clone::clone(&val), doReplace)?;
                return Ok(_t0);
            }
            let _t0: bool = StringCoding::hasNegatives(Clone::clone(&val), 0i32, (val.borrow().len() as i32))?;
            if !(_t0) {
                let _t1: Object = Object::from_any(val.clone());
                return Ok((_t1).downcast::<Rc<RefCell<Vec<i8>>>>());
            }
            let mut dp: i32 = 0i32;
            let _t1: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor((val.borrow().len() as i32))?;
            let mut dst: Rc<RefCell<Vec<i8>>> = _t1;
            let mut local_5: Rc<RefCell<Vec<i8>>> = val;
            let mut local_6 = (local_5.borrow().len() as i32);
            let mut local_7: i32 = 0i32;
            loop {
                if local_7 >= local_6 { break; }
                let mut c = (local_5.borrow()[local_7 as usize] as i32);
                if (c<0) {
                    dp = dp.wrapping_add(1i32);
                    dst.borrow_mut()[dp as usize] = ((((192i32|((c&255i32)>>((6i32&0x1f))))) as i8 as i32)) as i8;
                    dp = dp.wrapping_add(1i32);
                    dst.borrow_mut()[dp as usize] = ((((128i32|(c&63i32))) as i8 as i32)) as i8;
                } else {
                    dp = dp.wrapping_add(1i32);
                    dst.borrow_mut()[dp as usize] = (c) as i8;
                }
                local_7 = local_7.wrapping_add(1i32);
            }
            if dp == (dst.borrow().len() as i32) {
                return Ok(dst);
            }
            let _t2: Rc<RefCell<Vec<i8>>> = Arrays::copyOf_arr_b_i(Clone::clone(&dst), dp)?;
            Ok(_t2)
        }

        #[java_method(name = "encodeUTF8_UTF16", descriptor = "([BZ)[B", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn encodeUTF8_UTF16(mut val: Rc<RefCell<Vec<i8>>>, mut doReplace: bool) -> Result<Rc<RefCell<Vec<i8>>>> {
            let mut dp: i32 = 0i32;
            let mut sp: i32 = 0i32;
            let mut sl = ((val.borrow().len() as i32)>>((1i32&0x1f)));
            let mut _arr0: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; (sl).wrapping_mul(3i32) as usize]));
            let mut dst: Rc<RefCell<Vec<i8>>> = _arr0;
            let mut c: u16 = Default::default();
            loop {
                if sp >= sl { break; }
                let _t1: u16 = StringUTF16::getChar(Clone::clone(&val), sp)?;
                c = _t1;
                if (c as i32) >= 128i32 {
                    break;
                }
                dp = dp.wrapping_add(1i32);
                dst.borrow_mut()[dp as usize] = (((c) as i8 as i32)) as i8;
                sp = sp.wrapping_add(1i32);
            }
            loop {
                if sp >= sl { break; }
                sp = sp.wrapping_add(1i32);
                let _t1: u16 = StringUTF16::getChar(Clone::clone(&val), sp)?;
                c = _t1;
                if (c as i32) < 128i32 {
                    dp = dp.wrapping_add(1i32);
                    dst.borrow_mut()[dp as usize] = (((c) as i8 as i32)) as i8;
                } else {
                    if (c as i32) < 2048i32 {
                        dp = dp.wrapping_add(1i32);
                        dst.borrow_mut()[dp as usize] = ((((192i32|((c as i32)>>((6i32&0x1f))))) as i8 as i32)) as i8;
                        dp = dp.wrapping_add(1i32);
                        dst.borrow_mut()[dp as usize] = ((((128i32|((c as i32)&63i32))) as i8 as i32)) as i8;
                    } else {
                        let _t2: bool = Character::isSurrogate(c)?;
                        if _t2 {
                            let mut uc: i32 = -1i32;
                            let _t3: bool = Character::isHighSurrogate(c)?;
                            let _t4: u16 = StringUTF16::getChar(Clone::clone(&val), sp)?;
                            let mut c2: u16 = _t4;
                            let _t5: bool = Character::isLowSurrogate(c2)?;
                            if _t5 {
                                let _t6: i32 = Character::toCodePoint(c, c2)?;
                                uc = _t6;
                            }
                            if (uc<0) {
                                if doReplace {
                                    dp = dp.wrapping_add(1i32);
                                    dst.borrow_mut()[dp as usize] = (63i32) as i8;
                                } else {
                                    String::throwUnmappable_i((sp).wrapping_sub(1i32))?;
                                }
                            } else {
                                dp = dp.wrapping_add(1i32);
                                dst.borrow_mut()[dp as usize] = ((((240i32|(uc>>((18i32&0x1f))))) as i8 as i32)) as i8;
                                dp = dp.wrapping_add(1i32);
                                dst.borrow_mut()[dp as usize] = ((((128i32|((uc>>((12i32&0x1f)))&63i32))) as i8 as i32)) as i8;
                                dp = dp.wrapping_add(1i32);
                                dst.borrow_mut()[dp as usize] = ((((128i32|((uc>>((6i32&0x1f)))&63i32))) as i8 as i32)) as i8;
                                dp = dp.wrapping_add(1i32);
                                dst.borrow_mut()[dp as usize] = ((((128i32|(uc&63i32))) as i8 as i32)) as i8;
                                sp = sp.wrapping_add(1i32);
                            }
                        } else {
                            dp = dp.wrapping_add(1i32);
                            dst.borrow_mut()[dp as usize] = ((((224i32|((c as i32)>>((12i32&0x1f))))) as i8 as i32)) as i8;
                            dp = dp.wrapping_add(1i32);
                            dst.borrow_mut()[dp as usize] = ((((128i32|(((c as i32)>>((6i32&0x1f)))&63i32))) as i8 as i32)) as i8;
                            dp = dp.wrapping_add(1i32);
                            dst.borrow_mut()[dp as usize] = ((((128i32|((c as i32)&63i32))) as i8 as i32)) as i8;
                        }
                    }
                }
            }
            if dp == (dst.borrow().len() as i32) {
                return Ok(dst);
            }
            let _t1: Rc<RefCell<Vec<i8>>> = Arrays::copyOf_arr_b_i(Clone::clone(&dst), dp)?;
            Ok(_t1)
        }

        #[java_method(name = "<init>", descriptor = "([BLjava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/UnsupportedEncodingException")]
        pub fn new_arr_b_str(bytes: Rc<RefCell<Vec<i8>>>, charsetName: String) -> Result<Self> {
            panic!("stub: java/lang/String.<init>:([BLjava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "([BLjava/nio/charset/Charset;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_arr_b_charse(bytes: Rc<RefCell<Vec<i8>>>, charset: Charset) -> Result<Self> {
            panic!("stub: java/lang/String.<init>:([BLjava/nio/charset/Charset;)V")
        }

        #[java_method(name = "<init>", descriptor = "([BII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_arr_b_i_i(bytes: Rc<RefCell<Vec<i8>>>, offset: i32, length: i32) -> Result<Self> {
            panic!("stub: java/lang/String.<init>:([BII)V")
        }

        #[java_method(name = "<init>", descriptor = "([B)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_arr_b(bytes: Rc<RefCell<Vec<i8>>>) -> Result<Self> {
            panic!("stub: java/lang/String.<init>:([B)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/StringBuffer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_string(buffer: Object) -> Result<Self> {
            panic!("stub: java/lang/String.<init>:(Ljava/lang/StringBuffer;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/StringBuilder;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/StringBuilder;)V
        pub fn new_sb(mut builder: StringBuilder) -> Result<Self> {
            let mut this = Self::default();
            this = String::new_abstra_void(Clone::clone(&builder).into(), Clone::clone(&Object::default()))?;
            Ok(this)
        }

        #[java_method(name = "length", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn length(&self) -> Result<i32> {
            let this = self;
            let _t0 = this.coder()?;
            Ok(((this.__get_value().borrow().len() as i32)>>(((_t0 as i32)&0x1f))))
        }

        #[java_method(name = "isEmpty", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmpty(&self) -> Result<bool> {
            let this = self;
            Ok(((this.__get_value().borrow().len() as i32)==0))
        }

        #[java_method(name = "charAt", descriptor = "(I)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn charAt(&self, mut index: i32) -> Result<u16> {
            let this = self;
            let _t0 = this.isLatin1()?;
            if _t0 {
                let _t1: u16 = StringLatin1::charAt(Clone::clone(&this.__get_value()), index)?;
                return Ok(_t1);
            }
            let _t1: u16 = StringUTF16::charAt(Clone::clone(&this.__get_value()), index)?;
            Ok(_t1)
        }

        #[java_method(name = "codePointAt", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointAt(&self, mut index: i32) -> Result<i32> {
            let this = self;
            let _t0 = this.isLatin1()?;
            if _t0 {
                String::checkIndex(index, (this.__get_value().borrow().len() as i32))?;
                return Ok(((this.__get_value().borrow()[index as usize] as i32)&255i32));
            }
            let mut length = ((this.__get_value().borrow().len() as i32)>>((1i32&0x1f)));
            String::checkIndex(index, length)?;
            let _t1: i32 = StringUTF16::codePointAt_arr_b_i_i(Clone::clone(&this.__get_value()), index, length)?;
            Ok(_t1)
        }

        #[java_method(name = "codePointBefore", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointBefore(&self, mut index: i32) -> Result<i32> {
            let this = self;
            let mut i = (index).wrapping_sub(1i32);
            let _t0 = this.length()?;
            String::checkIndex(i, _t0)?;
            let _t1 = this.isLatin1()?;
            if _t1 {
                return Ok(((this.__get_value().borrow()[i as usize] as i32)&255i32));
            }
            let _t2: i32 = StringUTF16::codePointBefore_arr_b_i(Clone::clone(&this.__get_value()), index)?;
            Ok(_t2)
        }

        #[java_method(name = "codePointCount", descriptor = "(II)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePointCount(&self, mut beginIndex: i32, mut endIndex: i32) -> Result<i32> {
            let this = self;
            let _t0 = this.length()?;
            let _t1: i32 = Objects::checkFromToIndex_i_i_i(beginIndex, endIndex, _t0)?;
            let _t2 = this.isLatin1()?;
            if _t2 {
                return Ok((endIndex).wrapping_sub(beginIndex));
            }
            let _t3: i32 = StringUTF16::codePointCount_arr_b_i_i(Clone::clone(&this.__get_value()), beginIndex, endIndex)?;
            Ok(_t3)
        }

        #[java_method(name = "offsetByCodePoints", descriptor = "(II)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn offsetByCodePoints(&self, index: i32, codePointOffset: i32) -> Result<i32> {
            panic!("stub: java/lang/String.offsetByCodePoints:(II)I")
        }

        #[java_method(name = "getChars", descriptor = "(II[CI)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getChars(&self, mut srcBegin: i32, mut srcEnd: i32, mut dst: Rc<RefCell<Vec<u16>>>, mut dstBegin: i32) -> Result<()> {
            let this = self;
            let _t0 = this.length()?;
            String::checkBoundsBeginEnd(srcBegin, srcEnd, _t0)?;
            let _t1: i32 = String::checkBoundsOffCount(dstBegin, (srcEnd).wrapping_sub(srcBegin), (dst.borrow().len() as i32))?;
            let _t2 = this.isLatin1()?;
            if _t2 {
                StringLatin1::getChars(Clone::clone(&this.__get_value()), srcBegin, srcEnd, Clone::clone(&dst), dstBegin)?;
            } else {
                StringUTF16::getChars_arr_b_i_i_arr_c_i(Clone::clone(&this.__get_value()), srcBegin, srcEnd, Clone::clone(&dst), dstBegin)?;
            }
            Ok(())
        }

        #[java_method(name = "getBytes", descriptor = "(II[BI)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn getBytes_i_i_arr_b_i(&self, srcBegin: i32, srcEnd: i32, dst: Rc<RefCell<Vec<i8>>>, dstBegin: i32) -> Result<()> {
            panic!("stub: java/lang/String.getBytes:(II[BI)V")
        }

        #[java_method(name = "getBytes", descriptor = "(Ljava/lang/String;)[B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/UnsupportedEncodingException")]
        pub fn getBytes_str(&self, charsetName: String) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/lang/String.getBytes:(Ljava/lang/String;)[B")
        }

        #[java_method(name = "getBytes", descriptor = "(Ljava/nio/charset/Charset;)[B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getBytes(Ljava/nio/charset/Charset;)[B
        pub fn getBytes_charse(&self, mut charset: Charset) -> Result<Rc<RefCell<Vec<i8>>>> {
            let this = self;
            if _is_jnull(&charset) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0 = this.coder()?;
            let _t1: Rc<RefCell<Vec<i8>>> = String::encode(Clone::clone(&charset), _t0, Clone::clone(&this.__get_value()))?;
            Ok(_t1)
        }

        #[java_method(name = "getBytes", descriptor = "()[B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBytes(&self) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/lang/String.getBytes:()[B")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, mut anObject: Object) -> Result<bool> {
            let this = self;
            if Object::from_any(this.clone()) == anObject {
                return Ok((1i32 != 0i32));
            }
            let mut _merged2: bool;
            if (anObject.is_instance_of("java/lang/String")) {
                let mut aString = (anObject).downcast::<String>();
                let mut _merged1: bool;
                if (this.__get_coder() as i32) == (aString.__get_coder() as i32) {
                    let _t0: bool = StringLatin1::equals(Clone::clone(&this.__get_value()), Clone::clone(&aString.__get_value()))?;
                    _merged1 = !(!(_t0));
                } else {
                    _merged1 = (0i32 != 0);
                }
                _merged2 = _merged1;
            } else {
                _merged2 = (0i32 != 0);
            }
            Ok(_merged2)
        }

        #[java_method(name = "contentEquals", descriptor = "(Ljava/lang/StringBuffer;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn contentEquals_string(&self, sb: Object) -> Result<bool> {
            panic!("stub: java/lang/String.contentEquals:(Ljava/lang/StringBuffer;)Z")
        }

        #[java_method(name = "nonSyncContentEquals", descriptor = "(Ljava/lang/AbstractStringBuilder;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nonSyncContentEquals(&self, sb: AbstractStringBuilder) -> Result<bool> {
            panic!("stub: java/lang/String.nonSyncContentEquals:(Ljava/lang/AbstractStringBuilder;)Z")
        }

        #[java_method(name = "contentEquals", descriptor = "(Ljava/lang/CharSequence;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn contentEquals_seq(&self, cs: Object) -> Result<bool> {
            panic!("stub: java/lang/String.contentEquals:(Ljava/lang/CharSequence;)Z")
        }

        #[java_method(name = "equalsIgnoreCase", descriptor = "(Ljava/lang/String;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equalsIgnoreCase(&self, mut anotherString: String) -> Result<bool> {
            let this = self;
            let mut _merged6: bool;
            if Object::from_any(this.clone()) == Object::from_any(anotherString.clone()) {
                _merged6 = (1i32 != 0);
            } else {
                let mut _merged5: bool;
                if !_is_jnull(&anotherString) {
                    let _t0 = anotherString.length()?;
                    let _t1 = this.length()?;
                    let mut _merged4: bool;
                    if _t0 == _t1 {
                        let _t2 = this.length()?;
                        let _t3 = this.regionMatches_z_i_str_i_i((1i32 != 0i32), 0i32, Clone::clone(&anotherString), 0i32, _t2)?;
                        _merged4 = !(!(_t3));
                    } else {
                        _merged4 = (0i32 != 0);
                    }
                    _merged5 = _merged4;
                } else {
                    _merged5 = (0i32 != 0);
                }
                _merged6 = _merged5;
            }
            Ok(_merged6)
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, mut anotherString: String) -> Result<i32> {
            let this = self;
            let mut v1 = this.__get_value();
            let mut v2 = anotherString.__get_value();
            let _t0 = this.coder()?;
            let mut coder: i8 = _t0;
            let _t1 = anotherString.coder()?;
            let mut _merged3: i32;
            if (coder==0) {
                let _t2: i32 = StringLatin1::compareTo_arr_b_arr_b(Clone::clone(&v1), Clone::clone(&v2))?;
                _merged3 = _t2;
            } else {
                let _t2: i32 = StringUTF16::compareTo_arr_b_arr_b(Clone::clone(&v1), Clone::clone(&v2))?;
                _merged3 = _t2;
            }
            return Ok(_merged3);
            let mut _merged5: i32;
            if (coder==0) {
                let _t4: i32 = StringLatin1::compareToUTF16_arr_b_arr_b(Clone::clone(&v1), Clone::clone(&v2))?;
                _merged5 = _t4;
            } else {
                let _t4: i32 = StringUTF16::compareToLatin1_arr_b_arr_b(Clone::clone(&v1), Clone::clone(&v2))?;
                _merged5 = _t4;
            }
            Ok(_merged5)
        }

        #[java_method(name = "compareToIgnoreCase", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareToIgnoreCase(&self, mut str: String) -> Result<i32> {
            let this = self;
            let _vdispatch0: i32 = if let Some(_d) = String::CASE_INSENSITIVE_ORDER().0.as_any().downcast_ref::<Comparators_NaturalOrderComparator>() { _d.compare(Object::from_any(Clone::clone(self)), Object::from_any(str.clone()))? } else if let Some(_d) = String::CASE_INSENSITIVE_ORDER().0.as_any().downcast_ref::<Collections_ReverseComparator>() { _d.compare(Object::from_any(Clone::clone(self)), Object::from_any(str.clone()))? } else if let Some(_d) = String::CASE_INSENSITIVE_ORDER().0.as_any().downcast_ref::<Object>() { _d.compare(Object::from_any(Clone::clone(self)), Object::from_any(str.clone()))? } else if let Some(__f) = String::CASE_INSENSITIVE_ORDER().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(Clone::clone(self)), Object::from_any(str.clone()))? } else { Default::default() };
            Ok(_vdispatch0)
        }

        #[java_method(name = "regionMatches", descriptor = "(ILjava/lang/String;II)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: regionMatches(ILjava/lang/String;II)Z
        pub fn regionMatches_i_str_i_i(&self, mut toffset: i32, mut other: String, mut ooffset: i32, mut len: i32) -> Result<bool> {
            let this = self;
            let _t0 = this.length()?;
            let _t1 = other.length()?;
            if ((((ooffset as i64)>(((_t1 as i64)).wrapping_sub((len as i64)))) as i32-(((ooffset as i64))<(((_t1 as i64)).wrapping_sub((len as i64)))) as i32)>0) {
                return Ok((0i32 != 0i32));
            }
            if (len<=0) {
                return Ok((1i32 != 0i32));
            }
            let mut tv = this.__get_value();
            let mut ov = other.__get_value();
            let _t2 = this.coder()?;
            let mut coder: i8 = _t2;
            let _t3 = other.coder()?;
            if (coder as i32) == 1i32 {
                toffset = (toffset<<(1i32&0x1f));
                ooffset = (ooffset<<(1i32&0x1f));
                len = (len<<(1i32&0x1f));
            }
            let _t4: i32 = ArraysSupport::mismatch_arr_b_i_arr_b_i_i(Clone::clone(&tv), toffset, Clone::clone(&ov), ooffset, len)?;
            return Ok((_t4<0));
            loop {
                len = len.wrapping_sub(1i32);
                toffset = toffset.wrapping_add(1i32);
                let _t5: u16 = StringLatin1::getChar(Clone::clone(&tv), toffset)?;
                ooffset = ooffset.wrapping_add(1i32);
                let _t6: u16 = StringUTF16::getChar(Clone::clone(&ov), ooffset)?;
                if (_t5 as i32) != (_t6 as i32) { break; }
            }
            return Ok((0i32 != 0i32));
            loop {
                len = len.wrapping_sub(1i32);
                toffset = toffset.wrapping_add(1i32);
                let _t5: u16 = StringUTF16::getChar(Clone::clone(&tv), toffset)?;
                ooffset = ooffset.wrapping_add(1i32);
                let _t6: u16 = StringLatin1::getChar(Clone::clone(&ov), ooffset)?;
                if (_t5 as i32) != (_t6 as i32) { break; }
            }
            return Ok((0i32 != 0i32));
            Ok((1i32 != 0i32))
        }

        #[java_method(name = "regionMatches", descriptor = "(ZILjava/lang/String;II)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: regionMatches(ZILjava/lang/String;II)Z
        pub fn regionMatches_z_i_str_i_i(&self, mut ignoreCase: bool, mut toffset: i32, mut other: String, mut ooffset: i32, mut len: i32) -> Result<bool> {
            let this = self;
            if !(ignoreCase) {
                let _t0 = this.regionMatches_i_str_i_i(toffset, Clone::clone(&other), ooffset, len)?;
                return Ok(_t0);
            }
            let _t0 = this.length()?;
            let _t1 = other.length()?;
            if ((((ooffset as i64)>(((_t1 as i64)).wrapping_sub((len as i64)))) as i32-(((ooffset as i64))<(((_t1 as i64)).wrapping_sub((len as i64)))) as i32)>0) {
                return Ok((0i32 != 0i32));
            }
            let mut tv = this.__get_value();
            let mut ov = other.__get_value();
            let _t2 = this.coder()?;
            let mut coder: i8 = _t2;
            let _t3 = other.coder()?;
            let mut _merged5: bool;
            if (coder==0) {
                let _t4: bool = StringLatin1::regionMatchesCI(Clone::clone(&tv), toffset, Clone::clone(&ov), ooffset, len)?;
                _merged5 = _t4;
            } else {
                let _t4: bool = StringUTF16::regionMatchesCI(Clone::clone(&tv), toffset, Clone::clone(&ov), ooffset, len)?;
                _merged5 = _t4;
            }
            return Ok(_merged5);
            let mut _merged7: bool;
            if (coder==0) {
                let _t6: bool = StringLatin1::regionMatchesCI_UTF16(Clone::clone(&tv), toffset, Clone::clone(&ov), ooffset, len)?;
                _merged7 = _t6;
            } else {
                let _t6: bool = StringUTF16::regionMatchesCI_Latin1(Clone::clone(&tv), toffset, Clone::clone(&ov), ooffset, len)?;
                _merged7 = _t6;
            }
            Ok(_merged7)
        }

        #[java_method(name = "startsWith", descriptor = "(Ljava/lang/String;I)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: startsWith(Ljava/lang/String;I)Z
        pub fn startsWith_str_i(&self, mut prefix: String, mut toffset: i32) -> Result<bool> {
            let this = self;
            let _t0 = this.length()?;
            let _t1 = prefix.length()?;
            if toffset > (_t0).wrapping_sub(_t1) {
                return Ok((0i32 != 0i32));
            }
            let mut ta = this.__get_value();
            let mut pa = prefix.__get_value();
            let mut po: i32 = 0i32;
            let mut pc = (pa.borrow().len() as i32);
            let _t2 = this.coder()?;
            let mut coder: i8 = _t2;
            let _t3 = prefix.coder()?;
            if (coder as i32) == 1i32 {
                toffset = (toffset<<(1i32&0x1f));
            }
            let _t4: i32 = ArraysSupport::mismatch_arr_b_i_arr_b_i_i(Clone::clone(&ta), toffset, Clone::clone(&pa), 0i32, pc)?;
            return Ok((_t4<0));
            if (coder==0) {
                return Ok((0i32 != 0i32));
            }
            loop {
                toffset = toffset.wrapping_add(1i32);
                let _t5: u16 = StringUTF16::getChar(Clone::clone(&ta), toffset)?;
                po = po.wrapping_add(1i32);
                if (_t5 as i32) != ((pa.borrow()[po as usize] as i32)&255i32) { break; }
            }
            return Ok((0i32 != 0i32));
            Ok((1i32 != 0i32))
        }

        #[java_method(name = "startsWith", descriptor = "(Ljava/lang/String;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: startsWith(Ljava/lang/String;)Z
        pub fn startsWith_str(&self, mut prefix: String) -> Result<bool> {
            let this = self;
            let _t0 = this.startsWith_str_i(Clone::clone(&prefix), 0i32)?;
            Ok(_t0)
        }

        #[java_method(name = "endsWith", descriptor = "(Ljava/lang/String;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn endsWith(&self, mut suffix: String) -> Result<bool> {
            let this = self;
            let _t0 = this.length()?;
            let _t1 = suffix.length()?;
            let _t2 = this.startsWith_str_i(Clone::clone(&suffix), (_t0).wrapping_sub(_t1))?;
            Ok(_t2)
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            let this = self;
            let mut h = this.__get_hash();
            let _t0 = this.isLatin1()?;
            let mut _merged2: i32;
            if _t0 {
                let _t1: i32 = StringLatin1::hashCode(Clone::clone(&this.__get_value()))?;
                _merged2 = _t1;
            } else {
                let _t1: i32 = StringUTF16::hashCode(Clone::clone(&this.__get_value()))?;
                _merged2 = _t1;
            }
            h = _merged2;
            if (h==0) {
                this.__set_hashIsZero((1i32 != 0i32));
            } else {
                this.__set_hash(h);
            }
            Ok(h)
        }

        #[java_method(name = "indexOf", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: indexOf(I)I
        pub fn indexOf_i(&self, mut ch: i32) -> Result<i32> {
            let this = self;
            let _t0 = this.indexOf_i_i(ch, 0i32)?;
            Ok(_t0)
        }

        #[java_method(name = "indexOf", descriptor = "(II)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: indexOf(II)I
        pub fn indexOf_i_i(&self, mut ch: i32, mut fromIndex: i32) -> Result<i32> {
            let this = self;
            let _t0 = this.isLatin1()?;
            let mut _merged3: i32;
            if _t0 {
                let _t1 = this.length()?;
                let _t2: i32 = StringLatin1::indexOf_arr_b_i_i_i(Clone::clone(&this.__get_value()), ch, fromIndex, _t1)?;
                _merged3 = _t2;
            } else {
                let _t1 = this.length()?;
                let _t2: i32 = StringUTF16::indexOf_arr_b_i_i_i(Clone::clone(&this.__get_value()), ch, fromIndex, _t1)?;
                _merged3 = _t2;
            }
            Ok(_merged3)
        }

        #[java_method(name = "indexOf", descriptor = "(III)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOf_i_i_i(&self, ch: i32, beginIndex: i32, endIndex: i32) -> Result<i32> {
            panic!("stub: java/lang/String.indexOf:(III)I")
        }

        #[java_method(name = "lastIndexOf", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: lastIndexOf(I)I
        pub fn lastIndexOf_i(&self, mut ch: i32) -> Result<i32> {
            let this = self;
            let _t0 = this.length()?;
            let _t1 = this.lastIndexOf_i_i(ch, (_t0).wrapping_sub(1i32))?;
            Ok(_t1)
        }

        #[java_method(name = "lastIndexOf", descriptor = "(II)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: lastIndexOf(II)I
        pub fn lastIndexOf_i_i(&self, mut ch: i32, mut fromIndex: i32) -> Result<i32> {
            let this = self;
            let _t0 = this.isLatin1()?;
            let mut _merged2: i32;
            if _t0 {
                let _t1: i32 = StringLatin1::lastIndexOf_arr_b_i_i(Clone::clone(&this.__get_value()), ch, fromIndex)?;
                _merged2 = _t1;
            } else {
                let _t1: i32 = StringUTF16::lastIndexOf_arr_b_i_i(Clone::clone(&this.__get_value()), ch, fromIndex)?;
                _merged2 = _t1;
            }
            Ok(_merged2)
        }

        #[java_method(name = "indexOf", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: indexOf(Ljava/lang/String;)I
        pub fn indexOf_str(&self, mut str: String) -> Result<i32> {
            let this = self;
            let _t0 = this.coder()?;
            let mut coder: i8 = _t0;
            let _t1 = str.coder()?;
            let _t2 = this.isLatin1()?;
            let mut _merged4: i32;
            if _t2 {
                let _t3: i32 = StringLatin1::indexOf_arr_b_arr_b(Clone::clone(&this.__get_value()), Clone::clone(&str.__get_value()))?;
                _merged4 = _t3;
            } else {
                let _t3: i32 = StringUTF16::indexOf_arr_b_arr_b(Clone::clone(&this.__get_value()), Clone::clone(&str.__get_value()))?;
                _merged4 = _t3;
            }
            return Ok(_merged4);
            if (coder==0) {
                return Ok(-1i32);
            }
            let _t5: i32 = StringUTF16::indexOfLatin1_arr_b_arr_b(Clone::clone(&this.__get_value()), Clone::clone(&str.__get_value()))?;
            Ok(_t5)
        }

        #[java_method(name = "indexOf", descriptor = "(Ljava/lang/String;I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOf_str_i(&self, str: String, fromIndex: i32) -> Result<i32> {
            panic!("stub: java/lang/String.indexOf:(Ljava/lang/String;I)I")
        }

        #[java_method(name = "indexOf", descriptor = "(Ljava/lang/String;II)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOf_str_i_i(&self, str: String, beginIndex: i32, endIndex: i32) -> Result<i32> {
            panic!("stub: java/lang/String.indexOf:(Ljava/lang/String;II)I")
        }

        #[java_method(name = "indexOf", descriptor = "([BBILjava/lang/String;I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOf_arr_b_b_i_str_i(src: Rc<RefCell<Vec<i8>>>, srcCoder: i8, srcCount: i32, tgtStr: String, fromIndex: i32) -> Result<i32> {
            panic!("stub: java/lang/String.indexOf:([BBILjava/lang/String;I)I")
        }

        #[java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lastIndexOf_str(&self, str: String) -> Result<i32> {
            panic!("stub: java/lang/String.lastIndexOf:(Ljava/lang/String;)I")
        }

        #[java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/String;I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lastIndexOf_str_i(&self, str: String, fromIndex: i32) -> Result<i32> {
            panic!("stub: java/lang/String.lastIndexOf:(Ljava/lang/String;I)I")
        }

        #[java_method(name = "lastIndexOf", descriptor = "([BBILjava/lang/String;I)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lastIndexOf_arr_b_b_i_str_i(src: Rc<RefCell<Vec<i8>>>, srcCoder: i8, srcCount: i32, tgtStr: String, fromIndex: i32) -> Result<i32> {
            panic!("stub: java/lang/String.lastIndexOf:([BBILjava/lang/String;I)I")
        }

        #[java_method(name = "substring", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: substring(I)Ljava/lang/String;
        pub fn substring_i(&self, mut beginIndex: i32) -> Result<String> {
            let this = self;
            let _t0 = this.length()?;
            let _t1 = this.substring_i_i(beginIndex, _t0)?;
            Ok(_t1)
        }

        #[java_method(name = "substring", descriptor = "(II)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: substring(II)Ljava/lang/String;
        pub fn substring_i_i(&self, mut beginIndex: i32, mut endIndex: i32) -> Result<String> {
            let this = self;
            let _t0 = this.length()?;
            let mut length: i32 = _t0;
            String::checkBoundsBeginEnd(beginIndex, endIndex, length)?;
            if endIndex == length {
                return Ok(Clone::clone(this));
            }
            let mut subLen = (endIndex).wrapping_sub(beginIndex);
            let _t1 = this.isLatin1()?;
            let mut _merged3: String;
            if _t1 {
                let _t2: String = StringLatin1::newString(Clone::clone(&this.__get_value()), beginIndex, subLen)?;
                _merged3 = _t2;
            } else {
                let _t2: String = StringUTF16::newString(Clone::clone(&this.__get_value()), beginIndex, subLen)?;
                _merged3 = _t2;
            }
            Ok(_merged3)
        }

        #[java_method(name = "subSequence", descriptor = "(II)Ljava/lang/CharSequence;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn subSequence(&self, mut beginIndex: i32, mut endIndex: i32) -> Result<Object> {
            let this = self;
            let _t0 = this.substring_i_i(beginIndex, endIndex)?;
            Ok(Object::from_any(_t0.clone()))
        }

        #[java_method(name = "concat", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn concat(&self, mut str: String) -> Result<String> {
            let this = self;
            let _t0 = str.isEmpty()?;
            if _t0 {
                return Ok(Clone::clone(this));
            }
            let _t1: String = StringConcatHelper::simpleConcat(Object::from_any(Clone::clone(self)), Object::from_any(str.clone()))?;
            Ok(_t1)
        }

        #[java_method(name = "replace", descriptor = "(CC)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: replace(CC)Ljava/lang/String;
        pub fn replace_c_c(&self, mut oldChar: u16, mut newChar: u16) -> Result<String> {
            let this = self;
            let _t0 = this.isLatin1()?;
            let mut _merged2: String;
            if _t0 {
                let _t1: String = StringLatin1::replace_arr_b_c_c(Clone::clone(&this.__get_value()), oldChar, newChar)?;
                _merged2 = _t1;
            } else {
                let _t1: String = StringUTF16::replace_arr_b_c_c(Clone::clone(&this.__get_value()), oldChar, newChar)?;
                _merged2 = _t1;
            }
            let mut ret: String = _merged2;
            if !_is_jnull(&ret) {
                return Ok(ret);
            }
            Ok(Clone::clone(this))
        }

        #[java_method(name = "matches", descriptor = "(Ljava/lang/String;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn matches(&self, mut regex: String) -> Result<bool> {
            let this = self;
            let _t0: bool = Pattern::matches(Clone::clone(&regex), Object::from_any(Clone::clone(self)))?;
            Ok(_t0)
        }

        #[java_method(name = "contains", descriptor = "(Ljava/lang/CharSequence;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn contains(&self, mut s: Object) -> Result<bool> {
            let this = self;
            let _vdispatch0: String = if let Some(_d) = s.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.toString()? } else if let Some(_d) = s.0.as_any().downcast_ref::<CharBuffer>() { _d.toString()? } else if let Some(_d) = s.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.toString()? } else if let Some(_d) = s.0.as_any().downcast_ref::<String>() { _d.toString()? } else if let Some(_d) = s.0.as_any().downcast_ref::<StringBuilder>() { _d.toString()? } else if let Some(_d) = s.0.as_any().downcast_ref::<Object>() { _d.toString()? } else if let Some(__f) = s.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
            let _t1 = this.indexOf_str(Clone::clone(&_vdispatch0))?;
            Ok((_t1>=0))
        }

        #[java_method(name = "replaceFirst", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn replaceFirst(&self, mut regex: String, mut replacement: String) -> Result<String> {
            let this = self;
            let _t0: Pattern = Pattern::compile_str(Clone::clone(&regex))?;
            let _t1 = _t0.matcher(Object::from_any(Clone::clone(self)))?;
            let _t2 = _t1.replaceFirst_str(Clone::clone(&replacement))?;
            Ok(_t2)
        }

        #[java_method(name = "replaceAll", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn replaceAll(&self, mut regex: String, mut replacement: String) -> Result<String> {
            let this = self;
            let _t0: Pattern = Pattern::compile_str(Clone::clone(&regex))?;
            let _t1 = _t0.matcher(Object::from_any(Clone::clone(self)))?;
            let _t2 = _t1.replaceAll_str(Clone::clone(&replacement))?;
            Ok(_t2)
        }

        #[java_method(name = "replace", descriptor = "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: replace(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/lang/String;
        pub fn replace_seq_seq(&self, mut target: Object, mut replacement: Object) -> Result<String> {
            let this = self;
            let _vdispatch0: String = if let Some(_d) = target.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.toString()? } else if let Some(_d) = target.0.as_any().downcast_ref::<CharBuffer>() { _d.toString()? } else if let Some(_d) = target.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.toString()? } else if let Some(_d) = target.0.as_any().downcast_ref::<String>() { _d.toString()? } else if let Some(_d) = target.0.as_any().downcast_ref::<StringBuilder>() { _d.toString()? } else if let Some(_d) = target.0.as_any().downcast_ref::<Object>() { _d.toString()? } else if let Some(__f) = target.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
            let mut trgtStr: String = _vdispatch0;
            let _vdispatch1: String = if let Some(_d) = replacement.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.toString()? } else if let Some(_d) = replacement.0.as_any().downcast_ref::<CharBuffer>() { _d.toString()? } else if let Some(_d) = replacement.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.toString()? } else if let Some(_d) = replacement.0.as_any().downcast_ref::<String>() { _d.toString()? } else if let Some(_d) = replacement.0.as_any().downcast_ref::<StringBuilder>() { _d.toString()? } else if let Some(_d) = replacement.0.as_any().downcast_ref::<Object>() { _d.toString()? } else if let Some(__f) = replacement.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
            let mut replStr: String = _vdispatch1;
            let _t2 = this.length()?;
            let mut thisLen: i32 = _t2;
            let _t3 = trgtStr.length()?;
            let mut trgtLen: i32 = _t3;
            let _t4 = replStr.length()?;
            let mut replLen: i32 = _t4;
            if replLen == 1i32 {
                let _t5 = trgtStr.charAt(0i32)?;
                let _t6 = replStr.charAt(0i32)?;
                let _t7 = this.replace_c_c(_t5, _t6)?;
                return Ok(_t7);
            }
            let _t5 = this.isLatin1()?;
            let mut thisIsLatin1 = (_t5) as i32;
            let _t6 = trgtStr.isLatin1()?;
            let mut trgtIsLatin1 = (_t6) as i32;
            let _t7 = replStr.isLatin1()?;
            let mut replIsLatin1 = (_t7) as i32;
            let mut _merged11: String;
            if (thisIsLatin1!=0) {
                let mut _merged10: String;
                if (trgtIsLatin1!=0) {
                    let mut _merged9: String;
                    if (replIsLatin1!=0) {
                        let _t8: String = StringLatin1::replace_arr_b_i_arr_b_i_arr_b_i(Clone::clone(&this.__get_value()), thisLen, Clone::clone(&trgtStr.__get_value()), trgtLen, Clone::clone(&replStr.__get_value()), replLen)?;
                        _merged9 = _t8;
                    } else {
                        let _t8: String = StringUTF16::replace_arr_b_i_z_arr_b_i_z_arr_b_i_z(Clone::clone(&this.__get_value()), thisLen, (thisIsLatin1 != 0i32), Clone::clone(&trgtStr.__get_value()), trgtLen, (trgtIsLatin1 != 0i32), Clone::clone(&replStr.__get_value()), replLen, (replIsLatin1 != 0i32))?;
                        _merged9 = _t8;
                    }
                    _merged10 = _merged9;
                } else {
                    let _t8: String = StringUTF16::replace_arr_b_i_z_arr_b_i_z_arr_b_i_z(Clone::clone(&this.__get_value()), thisLen, (thisIsLatin1 != 0i32), Clone::clone(&trgtStr.__get_value()), trgtLen, (trgtIsLatin1 != 0i32), Clone::clone(&replStr.__get_value()), replLen, (replIsLatin1 != 0i32))?;
                    _merged10 = _t8;
                }
                _merged11 = _merged10;
            } else {
                let _t8: String = StringUTF16::replace_arr_b_i_z_arr_b_i_z_arr_b_i_z(Clone::clone(&this.__get_value()), thisLen, (thisIsLatin1 != 0i32), Clone::clone(&trgtStr.__get_value()), trgtLen, (trgtIsLatin1 != 0i32), Clone::clone(&replStr.__get_value()), replLen, (replIsLatin1 != 0i32))?;
                _merged11 = _t8;
            }
            let mut ret: String = _merged11;
            if !_is_jnull(&ret) {
                return Ok(ret);
            }
            return Ok(Clone::clone(this));
            let _t12: i32 = Math::addExact_i_i(thisLen, 1i32)?;
            let _t13: i32 = Math::multiplyExact_i_i(_t12, replLen)?;
            let _t14: i32 = Math::addExact_i_i(thisLen, _t13)?;
            thisIsLatin1 = _t14;
            let mut trgtIsLatin1 = StringBuilder::new_i(thisIsLatin1)?;
            let _t15 = trgtIsLatin1.append_str(Clone::clone(&replStr))?;
            replIsLatin1 = 0i32;
            loop {
                if replIsLatin1 >= thisLen { break; }
                let _t16 = this.charAt(replIsLatin1)?;
                let _t17 = trgtIsLatin1.append_c(_t16)?;
                let _t18 = _t17.append_str(Clone::clone(&replStr))?;
                replIsLatin1 = replIsLatin1.wrapping_add(1i32);
            }
            let _t16 = trgtIsLatin1.toString()?;
            Ok(_t16)
        }

        #[java_method(name = "split", descriptor = "(Ljava/lang/String;I)[Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: split(Ljava/lang/String;I)[Ljava/lang/String;
        pub fn split_str_i(&self, mut regex: String, mut limit: i32) -> Result<Rc<RefCell<Vec<String>>>> {
            let this = self;
            let _t0 = this.split_str_i_z(Clone::clone(&regex), limit, (0i32 != 0i32))?;
            Ok(_t0)
        }

        #[java_method(name = "splitWithDelimiters", descriptor = "(Ljava/lang/String;I)[Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn splitWithDelimiters(&self, regex: String, limit: i32) -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: java/lang/String.splitWithDelimiters:(Ljava/lang/String;I)[Ljava/lang/String;")
        }

        #[java_method(name = "split", descriptor = "(Ljava/lang/String;IZ)[Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: split(Ljava/lang/String;IZ)[Ljava/lang/String;
        pub fn split_str_i_z(&self, mut regex: String, mut limit: i32, mut withDelimiters: bool) -> Result<Rc<RefCell<Vec<String>>>> {
            let this = self;
            let mut ch: i32 = 0i32;
            let _t0 = regex.length()?;
            let _t1 = regex.charAt(0i32)?;
            let mut ch: u16 = _t1;
            let _t2 = String::from(".$|()[{^?*+\\").indexOf_i((ch as i32))?;
            let _t3 = regex.length()?;
            let _t4 = regex.charAt(0i32)?;
            let _t5 = regex.charAt(1i32)?;
            ch = _t5;
            if (ch as i32) > 767i32 {
                let _t6 = this.split_c_i_z(ch, limit, withDelimiters)?;
                return Ok(_t6);
            }
            let _t6: Pattern = Pattern::compile_str(Clone::clone(&regex))?;
            let mut pattern: Pattern = _t6;
            let mut _merged8: Rc<RefCell<Vec<String>>>;
            if withDelimiters {
                let _t7 = pattern.splitWithDelimiters(Object::from_any(Clone::clone(self)), limit)?;
                _merged8 = _t7;
            } else {
                let _t7 = pattern.split_seq_i(Object::from_any(Clone::clone(self)), limit)?;
                _merged8 = _t7;
            }
            Ok(_merged8)
        }

        #[java_method(name = "split", descriptor = "(CIZ)[Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: split(CIZ)[Ljava/lang/String;
        pub fn split_c_i_z(&self, mut ch: u16, mut limit: i32, mut withDelimiters: bool) -> Result<Rc<RefCell<Vec<String>>>> {
            let this = self;
            let mut matchCount: i32 = 0i32;
            let mut off: i32 = 0i32;
            let mut limited = ((limit>0)) as i32;
            let mut list = ArrayList::<Object>::new()?;
            let mut del = (if withDelimiters { String::from_owned(format!("{}", ch)) } else { Default::default() });
            loop {
                let _t0 = this.indexOf_i_i((ch as i32), off)?;
                let mut next: i32 = _t0;
                if next == -1i32 { break; }
                let _t0 = this.substring_i_i(off, next)?;
                let _t1 = list.add_obj(Object::from_any(_t0.clone()))?;
                if withDelimiters {
                    let _t2 = list.add_obj(Object::from_any(del.clone()))?;
                }
                off = (next).wrapping_add(1i32);
                matchCount = matchCount.wrapping_add(1i32);
            }
            let _t0 = this.length()?;
            let mut last: i32 = _t0;
            let _t1 = this.substring_i_i(off, last)?;
            let _t2 = list.add_obj(Object::from_any(_t1.clone()))?;
            off = last;
            matchCount = matchCount.wrapping_add(1i32);
            if (off==0) {
                let mut _arr3: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
                _arr3.borrow_mut()[0i32 as usize] = Clone::clone(&this);
                return Ok(_arr3);
            }
            if matchCount < limit {
                let _t3 = this.length()?;
                let _t4 = this.substring_i_i(off, _t3)?;
                let _t5 = list.add_obj(Object::from_any(_t4.clone()))?;
            }
            let _t3 = list.size()?;
            last = _t3;
            loop {
                if (last<=0) { break; }
                let _t4 = list.get((last).wrapping_sub(1i32))?;
                let _t5 = (_t4).downcast::<String>().isEmpty()?;
                if _t5 {
                    last = last.wrapping_sub(1i32);
                    continue;
                }
                break;
            }
            let mut _arr4: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); last as usize]));
            let mut result: Rc<RefCell<Vec<String>>> = _arr4;
            let _t5 = list.subList(0i32, last)?;
            let _vdispatch6: Rc<RefCell<Vec<Object>>> = if let Some(_d) = _t5.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.toArray(Default::default())? } else if let Some(_d) = _t5.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.toArray(Default::default())? } else if let Some(_d) = _t5.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.toArray(Default::default())? } else if let Some(_d) = _t5.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.toArray(Default::default())? } else if let Some(_d) = _t5.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.toArray(Default::default())? } else if let Some(_d) = _t5.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.toArray(Default::default())? } else if let Some(_d) = _t5.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.toArray(Default::default())? } else if let Some(_d) = _t5.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.toArray(Default::default())? } else if let Some(_d) = _t5.0.as_any().downcast_ref::<Object>() { _d.toArray(Default::default())? } else if let Some(__f) = _t5.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Rc<RefCell<Vec<Object>>>) -> crate::error::Result<Rc<RefCell<Vec<Object>>>>>>() { (__f)(Default::default())? } else { Default::default() };
            Ok(Default::default())
        }

        #[java_method(name = "split", descriptor = "(Ljava/lang/String;)[Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: split(Ljava/lang/String;)[Ljava/lang/String;
        pub fn split_str(&self, mut regex: String) -> Result<Rc<RefCell<Vec<String>>>> {
            let this = self;
            let _t0 = this.split_str_i_z(Clone::clone(&regex), 0i32, (0i32 != 0i32))?;
            Ok(_t0)
        }

        #[java_method(name = "join", descriptor = "(Ljava/lang/CharSequence;[Ljava/lang/CharSequence;)Ljava/lang/String;", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: join(Ljava/lang/CharSequence;[Ljava/lang/CharSequence;)Ljava/lang/String;
        pub fn join_seq_arr_seq(mut delimiter: Object, mut elements: Rc<RefCell<Vec<Object>>>) -> Result<String> {
            let _vdispatch0: String = if let Some(_d) = delimiter.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.toString()? } else if let Some(_d) = delimiter.0.as_any().downcast_ref::<CharBuffer>() { _d.toString()? } else if let Some(_d) = delimiter.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.toString()? } else if let Some(_d) = delimiter.0.as_any().downcast_ref::<String>() { _d.toString()? } else if let Some(_d) = delimiter.0.as_any().downcast_ref::<StringBuilder>() { _d.toString()? } else if let Some(_d) = delimiter.0.as_any().downcast_ref::<Object>() { _d.toString()? } else if let Some(__f) = delimiter.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
            let mut delim: String = _vdispatch0;
            let mut _arr1: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); (elements.borrow().len() as i32) as usize]));
            let mut elems: Rc<RefCell<Vec<String>>> = _arr1;
            let mut i: i32 = 0i32;
            loop {
                if i >= (elements.borrow().len() as i32) { break; }
                let _aastore_tmp2 = Clone::clone(&String::from_owned(format!("{}", Clone::clone(&elements.borrow()[i as usize]))));
                elems.borrow_mut()[i as usize] = _aastore_tmp2;
                i = i.wrapping_add(1i32);
            }
            let _t2: String = String::join_str_str_str_arr_str_i(Clone::clone(&String::from("")), Clone::clone(&String::from("")), Clone::clone(&delim), Clone::clone(&elems), (elems.borrow().len() as i32))?;
            Ok(_t2)
        }

        #[java_method(name = "join", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;I)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: join(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;I)Ljava/lang/String;
        pub fn join_str_str_str_arr_str_i(mut prefix: String, mut suffix: String, mut delimiter: String, mut elements: Rc<RefCell<Vec<String>>>, mut size: i32) -> Result<String> {
            let _t0 = prefix.coder()?;
            let _t1 = suffix.coder()?;
            let mut icoder = ((_t0 as i32)|(_t1 as i32));
            let _t2 = prefix.length()?;
            let _t3 = suffix.length()?;
            let mut len = ((_t2 as i64)).wrapping_add((_t3 as i64));
            if size > 1i32 {
                let _t4 = delimiter.length()?;
                len = (len).wrapping_add((((size).wrapping_sub(1i32) as i64)).wrapping_mul((_t4 as i64)));
                let _t5 = delimiter.coder()?;
                icoder = (icoder|(_t5 as i32));
            }
            let mut i: i32 = 0i32;
            loop {
                if i >= size { break; }
                let mut el = Clone::clone(&elements.borrow()[i as usize]);
                let _t4 = el.length()?;
                len = (len).wrapping_add((_t4 as i64));
                let _t5 = el.coder()?;
                icoder = (icoder|(_t5 as i32));
                i = i.wrapping_add(1i32);
            }
            i = ((icoder) as i8 as i32);
            len = (len).wrapping_shl((i&0x3f) as u32);
            if ((((len).wrapping_shl((i&0x3f) as u32)>(((len as i32) as i64))) as i32-(((len).wrapping_shl((i&0x3f) as u32))<(((len as i32) as i64))) as i32)!=0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t4: Rc<RefCell<Vec<i8>>> = StringConcatHelper::newArray(len)?;
            let mut el: Rc<RefCell<Vec<i8>>> = _t4;
            let mut off: i32 = 0i32;
            prefix.getBytes_arr_b_i_b(Clone::clone(&el), off, ((i) as i8))?;
            let _t5 = prefix.length()?;
            off = (off).wrapping_add(_t5);
            let mut el = Clone::clone(&elements.borrow()[0i32 as usize]);
            el.getBytes_arr_b_i_b(Clone::clone(&el), off, ((i) as i8))?;
            let _t6 = el.length()?;
            off = (off).wrapping_add(_t6);
            let mut i: i32 = 1i32;
            loop {
                if i >= size { break; }
                delimiter.getBytes_arr_b_i_b(Clone::clone(&el), off, ((i) as i8))?;
                let _t7 = delimiter.length()?;
                off = (off).wrapping_add(_t7);
                el = Clone::clone(&elements.borrow()[i as usize]);
                el.getBytes_arr_b_i_b(Clone::clone(&el), off, ((i) as i8))?;
                let _t8 = el.length()?;
                off = (off).wrapping_add(_t8);
                i = i.wrapping_add(1i32);
            }
            suffix.getBytes_arr_b_i_b(Clone::clone(&el), off, ((i) as i8))?;
            Ok(String::new_arr_b_b(Clone::clone(&el), ((i) as i8))?)
        }

        #[java_method(name = "join", descriptor = "(Ljava/lang/CharSequence;Ljava/lang/Iterable;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/CharSequence;Ljava/lang/Iterable<+Ljava/lang/CharSequence;>;)Ljava/lang/String;")]
        // java: join(Ljava/lang/CharSequence;Ljava/lang/Iterable;)Ljava/lang/String;
        pub fn join_seq_iter(mut delimiter: Object, mut elements: Object) -> Result<String> {
            let _t0: Object = Objects::requireNonNull_obj(Clone::clone(&delimiter))?;
            let _t1: Object = Objects::requireNonNull_obj(Clone::clone(&elements))?;
            let _vdispatch2: String = if let Some(_d) = delimiter.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.toString()? } else if let Some(_d) = delimiter.0.as_any().downcast_ref::<CharBuffer>() { _d.toString()? } else if let Some(_d) = delimiter.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.toString()? } else if let Some(_d) = delimiter.0.as_any().downcast_ref::<String>() { _d.toString()? } else if let Some(_d) = delimiter.0.as_any().downcast_ref::<StringBuilder>() { _d.toString()? } else if let Some(_d) = delimiter.0.as_any().downcast_ref::<Object>() { _d.toString()? } else if let Some(__f) = delimiter.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
            let mut delim: String = _vdispatch2;
            let mut _arr3: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 8i32 as usize]));
            let mut elems: Rc<RefCell<Vec<String>>> = _arr3;
            let mut size: i32 = 0i32;
            let _vdispatch4: Object = if let Some(_d) = elements.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.iterator()? } else if let Some(_d) = elements.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.iterator()? } else if let Some(_d) = elements.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.iterator()? } else if let Some(_d) = elements.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.iterator()? } else if let Some(_d) = elements.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.iterator()? } else if let Some(_d) = elements.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.iterator()? } else if let Some(_d) = elements.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.iterator()? } else if let Some(_d) = elements.0.as_any().downcast_ref::<HashMap_EntrySet>() { _d.iterator()? } else if let Some(_d) = elements.0.as_any().downcast_ref::<HashMap_KeySet>() { _d.iterator()? } else if let Some(_d) = elements.0.as_any().downcast_ref::<TreeMap_EntrySet>() { _d.iterator()? } else if let Some(_d) = elements.0.as_any().downcast_ref::<LinkedHashSet<Object>>() { _d.iterator()? } else if let Some(_d) = elements.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.iterator()? } else if let Some(_d) = elements.0.as_any().downcast_ref::<ArrayDeque<Object>>() { _d.iterator()? } else if let Some(_d) = elements.0.as_any().downcast_ref::<AbstractSet<Object>>() { _d.iterator()? } else if let Some(_d) = elements.0.as_any().downcast_ref::<HashSet<Object>>() { _d.iterator()? } else if let Some(_d) = elements.0.as_any().downcast_ref::<AbstractCollection<Object>>() { _d.iterator()? } else if let Some(_d) = elements.0.as_any().downcast_ref::<Object>() { _d.iterator()? } else if let Some(_d) = elements.0.as_any().downcast_ref::<Object>() { _d.iterator()? } else if let Some(_d) = elements.0.as_any().downcast_ref::<Object>() { _d.iterator()? } else if let Some(__f) = elements.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut local_5: Object = _vdispatch4;
            loop {
                let _vdispatch5: bool = if let Some(_d) = local_5.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.hasNext()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.hasNext()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.hasNext()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.hasNext()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(__f) = local_5.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<bool>>>() { (__f)()? } else { Default::default() };
                if !(_vdispatch5) { break; }
                let _vdispatch5: Object = if let Some(_d) = local_5.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.next()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.next()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.next()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.next()? } else if let Some(_d) = local_5.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(__f) = local_5.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let mut cs: Object = _vdispatch5;
                if size >= (elems.borrow().len() as i32) {
                    let _t6: Rc<RefCell<Vec<Object>>> = Arrays::copyOf_arr_obj_i(Default::default(), ((elems.borrow().len() as i32)<<(1i32&0x1f)))?;
                    elems = Default::default();
                }
                size = size.wrapping_add(1i32);
                elems.borrow_mut()[size as usize] = Clone::clone(&String::from_owned(format!("{}", cs)));
            }
            let _t5: String = String::join_str_str_str_arr_str_i(Clone::clone(&String::from("")), Clone::clone(&String::from("")), Clone::clone(&delim), Clone::clone(&elems), size)?;
            Ok(_t5)
        }

        #[java_method(name = "toLowerCase", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toLowerCase(Ljava/util/Locale;)Ljava/lang/String;
        pub fn toLowerCase_locale(&self, mut locale: Locale) -> Result<String> {
            let this = self;
            let _t0 = this.isLatin1()?;
            let mut _merged2: String;
            if _t0 {
                let _t1: String = StringLatin1::toLowerCase(Clone::clone(this), Clone::clone(&this.__get_value()), Clone::clone(&locale))?;
                _merged2 = _t1;
            } else {
                let _t1: String = StringUTF16::toLowerCase(Clone::clone(this), Clone::clone(&this.__get_value()), Clone::clone(&locale))?;
                _merged2 = _t1;
            }
            Ok(_merged2)
        }

        #[java_method(name = "toLowerCase", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toLowerCase()Ljava/lang/String;
        pub fn toLowerCase(&self) -> Result<String> {
            let this = self;
            let _t0: Locale = Locale::getDefault()?;
            let _t1 = this.toLowerCase_locale(Clone::clone(&_t0))?;
            Ok(_t1)
        }

        #[java_method(name = "toUpperCase", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toUpperCase(Ljava/util/Locale;)Ljava/lang/String;
        pub fn toUpperCase_locale(&self, mut locale: Locale) -> Result<String> {
            let this = self;
            let _t0 = this.isLatin1()?;
            let mut _merged2: String;
            if _t0 {
                let _t1: String = StringLatin1::toUpperCase(Clone::clone(this), Clone::clone(&this.__get_value()), Clone::clone(&locale))?;
                _merged2 = _t1;
            } else {
                let _t1: String = StringUTF16::toUpperCase(Clone::clone(this), Clone::clone(&this.__get_value()), Clone::clone(&locale))?;
                _merged2 = _t1;
            }
            Ok(_merged2)
        }

        #[java_method(name = "toUpperCase", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toUpperCase()Ljava/lang/String;
        pub fn toUpperCase(&self) -> Result<String> {
            let this = self;
            let _t0: Locale = Locale::getDefault()?;
            let _t1 = this.toUpperCase_locale(Clone::clone(&_t0))?;
            Ok(_t1)
        }

        #[java_method(name = "trim", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn trim(&self) -> Result<String> {
            let this = self;
            let _t0 = this.isLatin1()?;
            let mut _merged2: String;
            if _t0 {
                let _t1: String = StringLatin1::trim(Clone::clone(&this.__get_value()))?;
                _merged2 = _t1;
            } else {
                let _t1: String = StringUTF16::trim(Clone::clone(&this.__get_value()))?;
                _merged2 = _t1;
            }
            let mut ret: String = _merged2;
            Ok((if _is_jnull(&ret) { Clone::clone(this) } else { ret }))
        }

        #[java_method(name = "strip", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn strip(&self) -> Result<String> {
            let this = self;
            let _t0 = this.isLatin1()?;
            let mut _merged2: String;
            if _t0 {
                let _t1: String = StringLatin1::strip(Clone::clone(&this.__get_value()))?;
                _merged2 = _t1;
            } else {
                let _t1: String = StringUTF16::strip(Clone::clone(&this.__get_value()))?;
                _merged2 = _t1;
            }
            let mut ret: String = _merged2;
            Ok((if _is_jnull(&ret) { Clone::clone(this) } else { ret }))
        }

        #[java_method(name = "stripLeading", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stripLeading(&self) -> Result<String> {
            panic!("stub: java/lang/String.stripLeading:()Ljava/lang/String;")
        }

        #[java_method(name = "stripTrailing", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stripTrailing(&self) -> Result<String> {
            panic!("stub: java/lang/String.stripTrailing:()Ljava/lang/String;")
        }

        #[java_method(name = "isBlank", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isBlank(&self) -> Result<bool> {
            let this = self;
            let _t0 = this.indexOfNonWhitespace()?;
            let _t1 = this.length()?;
            Ok(_t0 == _t1)
        }

        #[java_method(name = "lines", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<Ljava/lang/String;>;")]
        pub fn lines(&self) -> Result<Object> {
            panic!("stub: java/lang/String.lines:()Ljava/util/stream/Stream;")
        }

        #[java_method(name = "indent", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indent(&self, n: i32) -> Result<String> {
            panic!("stub: java/lang/String.indent:(I)Ljava/lang/String;")
        }

        #[java_method(name = "indexOfNonWhitespace", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn indexOfNonWhitespace(&self) -> Result<i32> {
            let this = self;
            let _t0 = this.isLatin1()?;
            let mut _merged2: i32;
            if _t0 {
                let _t1: i32 = StringLatin1::indexOfNonWhitespace(Clone::clone(&this.__get_value()))?;
                _merged2 = _t1;
            } else {
                let _t1: i32 = StringUTF16::indexOfNonWhitespace(Clone::clone(&this.__get_value()))?;
                _merged2 = _t1;
            }
            Ok(_merged2)
        }

        #[java_method(name = "lastIndexOfNonWhitespace", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lastIndexOfNonWhitespace(&self) -> Result<i32> {
            panic!("stub: java/lang/String.lastIndexOfNonWhitespace:()I")
        }

        #[java_method(name = "stripIndent", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stripIndent(&self) -> Result<String> {
            panic!("stub: java/lang/String.stripIndent:()Ljava/lang/String;")
        }

        #[java_method(name = "outdent", descriptor = "(Ljava/util/List;)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<Ljava/lang/String;>;)I")]
        pub fn outdent(lines: Object) -> Result<i32> {
            panic!("stub: java/lang/String.outdent:(Ljava/util/List;)I")
        }

        #[java_method(name = "translateEscapes", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn translateEscapes(&self) -> Result<String> {
            panic!("stub: java/lang/String.translateEscapes:()Ljava/lang/String;")
        }

        #[java_method(name = "transform", descriptor = "(Ljava/util/function/Function;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/util/function/Function<-Ljava/lang/String;+TR;>;)TR;")]
        pub fn transform(&self, f: Object) -> Result<Object> {
            panic!("stub: java/lang/String.transform:(Ljava/util/function/Function;)Ljava/lang/Object;")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            let this = self;
            Ok(Clone::clone(this))
        }

        #[java_method(name = "chars", descriptor = "()Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn chars(&self) -> Result<Object> {
            panic!("stub: java/lang/String.chars:()Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "codePoints", descriptor = "()Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn codePoints(&self) -> Result<Object> {
            panic!("stub: java/lang/String.codePoints:()Ljava/util/stream/IntStream;")
        }

        #[java_method(name = "toCharArray", descriptor = "()[C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toCharArray(&self) -> Result<Rc<RefCell<Vec<u16>>>> {
            panic!("stub: java/lang/String.toCharArray:()[C")
        }

        #[java_method(name = "format", descriptor = "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
        pub fn format_str_arr_obj(mut format: String, mut args: Rc<RefCell<Vec<Object>>>) -> Result<String> {
            let _t0 = Formatter::new()?.format_str_arr_obj(Clone::clone(&format), Clone::clone(&args))?;
            let _t1 = _t0.toString()?;
            Ok(_t1)
        }

        #[java_method(name = "format", descriptor = "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn format_locale_str_arr_obj(l: Locale, format: String, args: Rc<RefCell<Vec<Object>>>) -> Result<String> {
            panic!("stub: java/lang/String.format:(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;")
        }

        #[java_method(name = "formatted", descriptor = "([Ljava/lang/Object;)Ljava/lang/String;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn formatted(&self, mut args: Rc<RefCell<Vec<Object>>>) -> Result<String> {
            let this = self;
            let _t0 = Formatter::new()?.format_str_arr_obj(Clone::clone(this), Clone::clone(&args))?;
            let _t1 = _t0.toString()?;
            Ok(_t1)
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/Object;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: valueOf(Ljava/lang/Object;)Ljava/lang/String;
        pub fn valueOf_obj(mut obj: Object) -> Result<String> {
            let mut _merged1: String;
            if _is_jnull(&obj) {
                _merged1 = String::from("null");
            } else {
                let _vdispatch0: String = if let Some(__f) = obj.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
                _merged1 = _vdispatch0;
            }
            Ok(_merged1)
        }

        #[java_method(name = "valueOf", descriptor = "([C)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn valueOf_arr_c(data: Rc<RefCell<Vec<u16>>>) -> Result<String> {
            panic!("stub: java/lang/String.valueOf:([C)Ljava/lang/String;")
        }

        #[java_method(name = "valueOf", descriptor = "([CII)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn valueOf_arr_c_i_i(data: Rc<RefCell<Vec<u16>>>, offset: i32, count: i32) -> Result<String> {
            panic!("stub: java/lang/String.valueOf:([CII)Ljava/lang/String;")
        }

        #[java_method(name = "copyValueOf", descriptor = "([CII)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyValueOf_arr_c_i_i(data: Rc<RefCell<Vec<u16>>>, offset: i32, count: i32) -> Result<String> {
            panic!("stub: java/lang/String.copyValueOf:([CII)Ljava/lang/String;")
        }

        #[java_method(name = "copyValueOf", descriptor = "([C)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyValueOf_arr_c(data: Rc<RefCell<Vec<u16>>>) -> Result<String> {
            panic!("stub: java/lang/String.copyValueOf:([C)Ljava/lang/String;")
        }

        #[java_method(name = "valueOf", descriptor = "(Z)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: valueOf(Z)Ljava/lang/String;
        pub fn valueOf_z(mut b: bool) -> Result<String> {
            Ok((if b { String::from("true") } else { String::from("false") }))
        }

        #[java_method(name = "valueOf", descriptor = "(C)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: valueOf(C)Ljava/lang/String;
        pub fn valueOf_c(mut c: u16) -> Result<String> {
            let _t0: bool = StringLatin1::canEncode_c(c)?;
            if _t0 {
                let _t1: Rc<RefCell<Vec<i8>>> = StringLatin1::toBytes_c(c)?;
                return Ok(String::new_arr_b_b(Clone::clone(&_t1), ((0i32) as i8))?);
            }
            let _t1: Rc<RefCell<Vec<i8>>> = StringUTF16::toBytes_c(c)?;
            Ok(String::new_arr_b_b(Clone::clone(&_t1), ((1i32) as i8))?)
        }

        #[java_method(name = "valueOf", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: valueOf(I)Ljava/lang/String;
        pub fn valueOf_i(mut i: i32) -> Result<String> {
            let _t0: String = Integer::toString_i(i)?;
            Ok(_t0)
        }

        #[java_method(name = "valueOf", descriptor = "(J)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: valueOf(J)Ljava/lang/String;
        pub fn valueOf_l(mut l: i64) -> Result<String> {
            let _t0: String = Long::toString_l(l)?;
            Ok(_t0)
        }

        #[java_method(name = "valueOf", descriptor = "(F)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn valueOf_f(f: f32) -> Result<String> {
            panic!("stub: java/lang/String.valueOf:(F)Ljava/lang/String;")
        }

        #[java_method(name = "valueOf", descriptor = "(D)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: valueOf(D)Ljava/lang/String;
        pub fn valueOf_d(mut d: f64) -> Result<String> {
            let _t0: String = Double::toString_d(d)?;
            Ok(_t0)
        }

        #[native]
        #[java_native(name = "intern", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn intern(&self) -> Result<String> {
            panic!("native: java/lang/String.intern:()Ljava/lang/String;")
        }

        #[java_method(name = "repeat", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn repeat(&self, mut count: i32) -> Result<String> {
            let this = self;
            if (count<0) {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("count is negative: ")))?;
                let _t1 = _t0.append_i(count)?;
                let _t2 = _t1.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if count == 1i32 {
                return Ok(Clone::clone(this));
            }
            let mut len = (this.__get_value().borrow().len() as i32);
            if (count==0) {
                return Ok(String::from(""));
            }
            if (943i32/count) < len {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if len == 1i32 {
                let mut _arr0: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; count as usize]));
                let mut single: Rc<RefCell<Vec<i8>>> = _arr0;
                Arrays::fill_arr_b_b(Clone::clone(&single), (((this.__get_value().borrow()[0i32 as usize] as i32)) as i8))?;
                return Ok(String::new_arr_b_b(Clone::clone(&single), this.__get_coder())?);
            }
            let mut single = (len).wrapping_mul(count);
            let mut _arr0: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; single as usize]));
            let mut multiple: Rc<RefCell<Vec<i8>>> = _arr0;
            System::arraycopy(Object::from_any(this.__get_value().clone()), 0i32, Object::from_any(multiple.clone()), 0i32, len)?;
            String::repeatCopyRest(Clone::clone(&multiple), 0i32, single, len)?;
            Ok(String::new_arr_b_b(Clone::clone(&multiple), this.__get_coder())?)
        }

        #[java_method(name = "repeatCopyRest", descriptor = "([BIII)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn repeatCopyRest(mut buffer: Rc<RefCell<Vec<i8>>>, mut offset: i32, mut limit: i32, mut copied: i32) -> Result<()> {
            loop {
                if copied >= (limit).wrapping_sub(copied) { break; }
                System::arraycopy(Object::from_any(buffer.clone()), offset, Object::from_any(buffer.clone()), (offset).wrapping_add(copied), copied)?;
                copied = (copied<<(1i32&0x1f));
            }
            System::arraycopy(Object::from_any(buffer.clone()), offset, Object::from_any(buffer.clone()), (offset).wrapping_add(copied), (limit).wrapping_sub(copied))?;
            Ok(())
        }

        #[java_method(name = "getBytes", descriptor = "([BIB)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getBytes([BIB)V
        pub fn getBytes_arr_b_i_b(&self, mut dst: Rc<RefCell<Vec<i8>>>, mut dstBegin: i32, mut coder: i8) -> Result<()> {
            let this = self;
            let _t0 = this.coder()?;
            if (_t0 as i32) == (coder as i32) {
                System::arraycopy(Object::from_any(this.__get_value().clone()), 0i32, Object::from_any(dst.clone()), (dstBegin<<((coder as i32)&0x1f)), (this.__get_value().borrow().len() as i32))?;
            } else {
                StringLatin1::inflate_arr_b_i_arr_b_i_i(Clone::clone(&this.__get_value()), 0i32, Clone::clone(&dst), dstBegin, (this.__get_value().borrow().len() as i32))?;
            }
            Ok(())
        }

        #[java_method(name = "getBytes", descriptor = "([BIIBI)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBytes_arr_b_i_i_b_i(&self, dst: Rc<RefCell<Vec<i8>>>, srcPos: i32, dstBegin: i32, coder: i8, length: i32) -> Result<()> {
            panic!("stub: java/lang/String.getBytes:([BIIBI)V")
        }

        #[java_method(name = "<init>", descriptor = "([CIILjava/lang/Void;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>([CIILjava/lang/Void;)V
        pub fn new_arr_c_i_i_void(mut value: Rc<RefCell<Vec<u16>>>, mut off: i32, mut len: i32, mut sig: Object) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            if (len==0) {
                this.__set_value(Clone::clone(&String::from("").__get_value()));
                this.__set_coder(String::from("").__get_coder());
                return Ok(this);
            }
            if String::COMPACT_STRINGS() {
                let _t0: Rc<RefCell<Vec<i8>>> = StringUTF16::compress_arr_c_i_i(Clone::clone(&value), off, len)?;
                let mut val: Rc<RefCell<Vec<i8>>> = _t0;
                let _t1: i8 = StringUTF16::coderFromArrayLen(Clone::clone(&val), len)?;
                this.__set_coder(_t1);
                this.__set_value(Clone::clone(&val));
                return Ok(this);
            }
            this.__set_coder(((1i32) as i8));
            let _t0: Rc<RefCell<Vec<i8>>> = StringUTF16::toBytes_arr_c_i_i(Clone::clone(&value), off, len)?;
            this.__set_value(Clone::clone(&_t0));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/AbstractStringBuilder;Ljava/lang/Void;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/AbstractStringBuilder;Ljava/lang/Void;)V
        pub fn new_abstra_void(mut asb: AbstractStringBuilder, mut sig: Object) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            let _t0 = asb.getValue()?;
            let mut val: Rc<RefCell<Vec<i8>>> = _t0;
            let _t1 = asb.length()?;
            let mut length: i32 = _t1;
            let _t2 = asb.isLatin1()?;
            if _t2 {
                this.__set_coder(((0i32) as i8));
                let _t3: Rc<RefCell<Vec<i8>>> = Arrays::copyOfRange_arr_b_i_i(Clone::clone(&val), 0i32, length)?;
                this.__set_value(Clone::clone(&_t3));
            } else {
                if asb.__get_maybeLatin1() {
                    let _t3: Rc<RefCell<Vec<i8>>> = StringUTF16::compress_arr_b_i_i(Clone::clone(&val), 0i32, length)?;
                    this.__set_value(Clone::clone(&_t3));
                    let _t4: i8 = StringUTF16::coderFromArrayLen(Clone::clone(&this.__get_value()), length)?;
                    this.__set_coder(_t4);
                    return Ok(this);
                }
                this.__set_coder(((1i32) as i8));
                let _t3: Rc<RefCell<Vec<i8>>> = Arrays::copyOfRange_arr_b_i_i(Clone::clone(&val), 0i32, (length<<(1i32&0x1f)))?;
                this.__set_value(Clone::clone(&_t3));
            }
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "([BB)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>([BB)V
        pub fn new_arr_b_b(mut value: Rc<RefCell<Vec<i8>>>, mut coder: i8) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_value(Clone::clone(&value));
            this.__set_coder(coder);
            Ok(this)
        }

        #[java_method(name = "coder", descriptor = "()B", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn coder(&self) -> Result<i8> {
            let this = self;
            Ok((if String::COMPACT_STRINGS() { this.__get_coder() } else { (1i32 as i8) }))
        }

        #[java_method(name = "value", descriptor = "()[B", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn value(&self) -> Result<Rc<RefCell<Vec<i8>>>> {
            let this = self;
            Ok(this.__get_value())
        }

        #[java_method(name = "isLatin1", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLatin1(&self) -> Result<bool> {
            let this = self;
            Ok((if String::COMPACT_STRINGS() { (this.__get_coder()==0) } else { (0i32 != 0) }))
        }

        #[java_method(name = "checkIndex", descriptor = "(II)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkIndex(mut index: i32, mut length: i32) -> Result<()> {
            let _t0: i32 = Preconditions::checkIndex_i_i_bifunc(index, length, Clone::clone(&Preconditions::SIOOBE_FORMATTER()))?;
            Ok(())
        }

        #[java_method(name = "checkOffset", descriptor = "(II)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkOffset(mut offset: i32, mut length: i32) -> Result<()> {
            let _t0: i32 = Preconditions::checkFromToIndex_i_i_i_bifunc(offset, length, length, Clone::clone(&Preconditions::SIOOBE_FORMATTER()))?;
            Ok(())
        }

        #[java_method(name = "checkBoundsOffCount", descriptor = "(III)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkBoundsOffCount(mut offset: i32, mut count: i32, mut length: i32) -> Result<i32> {
            let _t0: i32 = Preconditions::checkFromIndexSize_i_i_i_bifunc(offset, count, length, Clone::clone(&Preconditions::SIOOBE_FORMATTER()))?;
            Ok(_t0)
        }

        #[java_method(name = "checkBoundsBeginEnd", descriptor = "(III)V", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkBoundsBeginEnd(mut begin: i32, mut end: i32, mut length: i32) -> Result<()> {
            let _t0: i32 = Preconditions::checkFromToIndex_i_i_i_bifunc(begin, end, length, Clone::clone(&Preconditions::SIOOBE_FORMATTER()))?;
            Ok(())
        }

        #[java_method(name = "valueOfCodePoint", descriptor = "(I)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn valueOfCodePoint(codePoint: i32) -> Result<String> {
            panic!("stub: java/lang/String.valueOfCodePoint:(I)Ljava/lang/String;")
        }

        #[java_method(name = "describeConstable", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Optional<Ljava/lang/String;>;")]
        pub fn describeConstable(&self) -> Result<Optional<Object>> {
            panic!("stub: java/lang/String.describeConstable:()Ljava/util/Optional;")
        }

        #[java_method(name = "resolveConstantDesc", descriptor = "(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn resolveConstantDesc(&self, lookup: Object) -> Result<String> {
            panic!("stub: java/lang/String.resolveConstantDesc:(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/String;")
        }
    }
}
