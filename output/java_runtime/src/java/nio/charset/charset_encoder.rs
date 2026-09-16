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

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/nio/charset/CharsetEncoder"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "CharsetEncoder.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/nio/charset/CharsetEncoder"]

    pub struct CharsetEncoder {
        #[cfg_attr(any(), java_field(name = "charset", descriptor = "Ljava/nio/charset/Charset;", access = "private", modifiers = "final", is_static = false))]
        pub charset: Charset,
        #[cfg_attr(any(), java_field(name = "averageBytesPerChar", descriptor = "F", access = "private", modifiers = "final", is_static = false))]
        pub averageBytesPerChar: f32,
        #[cfg_attr(any(), java_field(name = "maxBytesPerChar", descriptor = "F", access = "private", modifiers = "final", is_static = false))]
        pub maxBytesPerChar: f32,
        #[cfg_attr(any(), java_field(name = "replacement", descriptor = "[B", access = "private", modifiers = "", is_static = false))]
        pub replacement: Rc<RefCell<Vec<i8>>>,
        #[cfg_attr(any(), java_field(name = "malformedInputAction", descriptor = "Ljava/nio/charset/CodingErrorAction;", access = "private", modifiers = "", is_static = false))]
        pub malformedInputAction: CodingErrorAction,
        #[cfg_attr(any(), java_field(name = "unmappableCharacterAction", descriptor = "Ljava/nio/charset/CodingErrorAction;", access = "private", modifiers = "", is_static = false))]
        pub unmappableCharacterAction: CodingErrorAction,
        #[cfg_attr(any(), java_field(name = "state", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub state: i32,
        #[cfg_attr(any(), java_field(name = "cachedDecoder", descriptor = "Ljava/lang/ref/WeakReference;", access = "private", modifiers = "", is_static = false, generic_signature = "Ljava/lang/ref/WeakReference<Ljava/nio/charset/CharsetDecoder;>;"))]
        pub cachedDecoder: Object,
    }

    impl CharsetEncoder {
        #[cfg_attr(any(), java_field(name = "ST_RESET", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: ST_RESET:I
        pub fn ST_RESET() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "ST_CODING", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: ST_CODING:I
        pub fn ST_CODING() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "ST_END", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: ST_END:I
        pub fn ST_END() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "ST_FLUSHED", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: ST_FLUSHED:I
        pub fn ST_FLUSHED() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "stateNames", descriptor = "[Ljava/lang/String;", access = "private", modifiers = "static", is_static = true))]
        // static field: stateNames:[Ljava/lang/String;
        pub fn stateNames() -> Rc<RefCell<Vec<String>>> {
            panic!("stub: java/nio/charset/CharsetEncoder.stateNames:[Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/nio/charset/Charset;FF[B)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_charse_f_f_arr_b(cs: Charset, averageBytesPerChar: f32, maxBytesPerChar: f32, replacement: Rc<RefCell<Vec<i8>>>) -> Result<Self> {
            panic!("stub: java/nio/charset/CharsetEncoder.<init>:(Ljava/nio/charset/Charset;FF[B)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/nio/charset/Charset;FF)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_charse_f_f(cs: Charset, averageBytesPerChar: f32, maxBytesPerChar: f32) -> Result<Self> {
            panic!("stub: java/nio/charset/CharsetEncoder.<init>:(Ljava/nio/charset/Charset;FF)V")
        }

        #[java_method(name = "charset", descriptor = "()Ljava/nio/charset/Charset;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn charset(&self) -> Result<Charset> {
            panic!("stub: java/nio/charset/CharsetEncoder.charset:()Ljava/nio/charset/Charset;")
        }

        #[java_method(name = "replacement", descriptor = "()[B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn replacement(&self) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: java/nio/charset/CharsetEncoder.replacement:()[B")
        }

        #[java_method(name = "replaceWith", descriptor = "([B)Ljava/nio/charset/CharsetEncoder;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn replaceWith(&self, newReplacement: Rc<RefCell<Vec<i8>>>) -> Result<CharsetEncoder> {
            panic!("stub: java/nio/charset/CharsetEncoder.replaceWith:([B)Ljava/nio/charset/CharsetEncoder;")
        }

        #[java_method(name = "implReplaceWith", descriptor = "([B)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn implReplaceWith(&self, newReplacement: Rc<RefCell<Vec<i8>>>) -> Result<()> {
            panic!("stub: java/nio/charset/CharsetEncoder.implReplaceWith:([B)V")
        }

        #[java_method(name = "isLegalReplacement", descriptor = "([B)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLegalReplacement(&self, repl: Rc<RefCell<Vec<i8>>>) -> Result<bool> {
            panic!("stub: java/nio/charset/CharsetEncoder.isLegalReplacement:([B)Z")
        }

        #[java_method(name = "malformedInputAction", descriptor = "()Ljava/nio/charset/CodingErrorAction;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn malformedInputAction(&self) -> Result<CodingErrorAction> {
            panic!("stub: java/nio/charset/CharsetEncoder.malformedInputAction:()Ljava/nio/charset/CodingErrorAction;")
        }

        #[java_method(name = "onMalformedInput", descriptor = "(Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetEncoder;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn onMalformedInput(&self, mut newAction: CodingErrorAction) -> Result<CharsetEncoder> {
            let this = self;
            if _is_jnull(&newAction) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.__set_malformedInputAction(Clone::clone(&newAction));
            this.implOnMalformedInput(Clone::clone(&newAction))?;
            Ok(Clone::clone(this))
        }

        #[java_method(name = "implOnMalformedInput", descriptor = "(Ljava/nio/charset/CodingErrorAction;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn implOnMalformedInput(&self, mut newAction: CodingErrorAction) -> Result<()> {
            let this = self;
            Ok(())
        }

        #[java_method(name = "unmappableCharacterAction", descriptor = "()Ljava/nio/charset/CodingErrorAction;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unmappableCharacterAction(&self) -> Result<CodingErrorAction> {
            panic!("stub: java/nio/charset/CharsetEncoder.unmappableCharacterAction:()Ljava/nio/charset/CodingErrorAction;")
        }

        #[java_method(name = "onUnmappableCharacter", descriptor = "(Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetEncoder;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn onUnmappableCharacter(&self, mut newAction: CodingErrorAction) -> Result<CharsetEncoder> {
            let this = self;
            if _is_jnull(&newAction) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.__set_unmappableCharacterAction(Clone::clone(&newAction));
            this.implOnUnmappableCharacter(Clone::clone(&newAction))?;
            Ok(Clone::clone(this))
        }

        #[java_method(name = "implOnUnmappableCharacter", descriptor = "(Ljava/nio/charset/CodingErrorAction;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn implOnUnmappableCharacter(&self, mut newAction: CodingErrorAction) -> Result<()> {
            let this = self;
            Ok(())
        }

        #[java_method(name = "averageBytesPerChar", descriptor = "()F", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn averageBytesPerChar(&self) -> Result<f32> {
            panic!("stub: java/nio/charset/CharsetEncoder.averageBytesPerChar:()F")
        }

        #[java_method(name = "maxBytesPerChar", descriptor = "()F", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn maxBytesPerChar(&self) -> Result<f32> {
            let this = self;
            Ok(this.__get_maxBytesPerChar())
        }

        #[java_method(name = "encode", descriptor = "(Ljava/nio/CharBuffer;Ljava/nio/ByteBuffer;Z)Ljava/nio/charset/CoderResult;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: encode(Ljava/nio/CharBuffer;Ljava/nio/ByteBuffer;Z)Ljava/nio/charset/CoderResult;
        pub fn encode_charbu_bytebu_z(&self, mut in_: CharBuffer, mut out: ByteBuffer, mut endOfInput: bool) -> Result<CoderResult> {
            let this = self;
            let mut newState = (!(endOfInput)) as i32;
            if this.__get_state() != 2i32 {
                this.throwIllegalStateException(this.__get_state(), newState)?;
            }
            this.__set_state(newState);
        let mut x: Object = Default::default();
            loop {
                let _t0 = this.encodeLoop(Clone::clone(&in_), Clone::clone(&out))?;
                let mut cr: CoderResult = _t0;
                let _t1 = cr.isOverflow()?;
                if _t1 {
                    return Ok(cr);
                }
                let _t2 = cr.isUnderflow()?;
                if endOfInput {
                    let _t3 = in_.__super().hasRemaining()?;
                    if _t3 {
                        let _t4 = in_.__super().remaining()?;
                        let _t5: CoderResult = CoderResult::malformedForLength(_t4)?;
                        cr = _t5;
                    } else {
                        return Ok(cr);
                    }
                } else {
                    return Ok(cr);
                }
                x = Object::default();
                let _t3 = cr.isMalformed()?;
                if _t3 {
                    x = this.__get_malformedInputAction();
                } else {
                    let _t4 = cr.isUnmappable()?;
                    if _t4 {
                        x = this.__get_unmappableCharacterAction();
                    } else {
                        if !(CharsetEncoder::_assertionsDisabled()) {
                            let _t5 = cr.toString()?;
                            return Err(JvmError::Custom("athrow".to_owned()));
                        }
                    }
                }
                if Object::from_any(x.clone()) == Object::from_any(CodingErrorAction::REPORT().clone()) {
                    return Ok(cr);
                }
                let _t4 = out.__super().remaining()?;
                if _t4 < (this.__get_replacement().borrow().len() as i32) {
                    return Ok(CoderResult::OVERFLOW());
                }
                let _t5 = out.put_arr_b(Clone::clone(&this.__get_replacement()))?;
                let _t6 = in_.__super().position()?;
                let _t7 = cr.length()?;
                let _t8 = in_.position((_t6).wrapping_add(_t7))?;
                continue;
                if !(CharsetEncoder::_assertionsDisabled()) {
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
            }
        }

        #[java_method(name = "flush", descriptor = "(Ljava/nio/ByteBuffer;)Ljava/nio/charset/CoderResult;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn flush(&self, mut out: ByteBuffer) -> Result<CoderResult> {
            let this = self;
            let _t0 = this.implFlush(Clone::clone(&out))?;
            let mut cr: CoderResult = _t0;
            let _t1 = cr.isUnderflow()?;
            if _t1 {
                this.__set_state(3i32);
            }
            return Ok(cr);
            if this.__get_state() != 3i32 {
                this.throwIllegalStateException(this.__get_state(), 3i32)?;
            }
            Ok(CoderResult::UNDERFLOW())
        }

        #[java_method(name = "implFlush", descriptor = "(Ljava/nio/ByteBuffer;)Ljava/nio/charset/CoderResult;", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn implFlush(&self, mut out: ByteBuffer) -> Result<CoderResult> {
            let this = self;
            Ok(CoderResult::UNDERFLOW())
        }

        #[java_method(name = "reset", descriptor = "()Ljava/nio/charset/CharsetEncoder;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reset(&self) -> Result<CharsetEncoder> {
            panic!("stub: java/nio/charset/CharsetEncoder.reset:()Ljava/nio/charset/CharsetEncoder;")
        }

        #[java_method(name = "implReset", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn implReset(&self) -> Result<()> {
            panic!("stub: java/nio/charset/CharsetEncoder.implReset:()V")
        }

        #[java_method(name = "encodeLoop", descriptor = "(Ljava/nio/CharBuffer;Ljava/nio/ByteBuffer;)Ljava/nio/charset/CoderResult;", access = "protected", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn encodeLoop(&self, arg0: CharBuffer, arg1: ByteBuffer) -> Result<CoderResult> {
            panic!("stub: java/nio/charset/CharsetEncoder.encodeLoop:(Ljava/nio/CharBuffer;Ljava/nio/ByteBuffer;)Ljava/nio/charset/CoderResult;")
        }

        #[java_method(name = "encode", descriptor = "(Ljava/nio/CharBuffer;)Ljava/nio/ByteBuffer;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/nio/charset/CharacterCodingException")]
        pub fn encode_charbu(&self, in_: CharBuffer) -> Result<ByteBuffer> {
            panic!("stub: java/nio/charset/CharsetEncoder.encode:(Ljava/nio/CharBuffer;)Ljava/nio/ByteBuffer;")
        }

        #[java_method(name = "canEncode", descriptor = "(Ljava/nio/CharBuffer;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn canEncode_charbu(&self, cb: CharBuffer) -> Result<bool> {
            panic!("stub: java/nio/charset/CharsetEncoder.canEncode:(Ljava/nio/CharBuffer;)Z")
        }

        #[java_method(name = "canEncode", descriptor = "(C)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn canEncode_c(&self, c: u16) -> Result<bool> {
            panic!("stub: java/nio/charset/CharsetEncoder.canEncode:(C)Z")
        }

        #[java_method(name = "canEncode", descriptor = "(Ljava/lang/CharSequence;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn canEncode_seq(&self, cs: Object) -> Result<bool> {
            panic!("stub: java/nio/charset/CharsetEncoder.canEncode:(Ljava/lang/CharSequence;)Z")
        }

        #[java_method(name = "throwIllegalStateException", descriptor = "(II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn throwIllegalStateException(&self, mut from: i32, mut to: i32) -> Result<()> {
            let this = self;
            let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Current state = ")))?;
            let _t1 = _t0.append_str(Clone::clone(&Clone::clone(&CharsetEncoder::stateNames().borrow()[from as usize])))?;
            let _t2 = _t1.append_str(Clone::clone(&String::from(", new state = ")))?;
            let _t3 = _t2.append_str(Clone::clone(&Clone::clone(&CharsetEncoder::stateNames().borrow()[to as usize])))?;
            let _t4 = _t3.toString()?;
            return Err(JvmError::Custom("athrow".to_owned()));
            Ok(())
        }
    }
}
