#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/nio/charset/CharsetDecoder",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "CharsetDecoder.java",
))]
pub struct CharsetDecoder {
    #[cfg_attr(any(), java_field(name = "charset", descriptor = "Ljava/nio/charset/Charset;", access = "private final"))]
    pub charset: Field<Object>,
    #[cfg_attr(any(), java_field(name = "averageCharsPerByte", descriptor = "F", access = "private final"))]
    pub averageCharsPerByte: Field<f32>,
    #[cfg_attr(any(), java_field(name = "maxCharsPerByte", descriptor = "F", access = "private final"))]
    pub maxCharsPerByte: Field<f32>,
    #[cfg_attr(any(), java_field(name = "replacement", descriptor = "Ljava/lang/String;", access = "private"))]
    pub replacement: Field<String>,
    #[cfg_attr(any(), java_field(name = "malformedInputAction", descriptor = "Ljava/nio/charset/CodingErrorAction;", access = "private"))]
    pub malformedInputAction: Field<Object>,
    #[cfg_attr(any(), java_field(name = "unmappableCharacterAction", descriptor = "Ljava/nio/charset/CodingErrorAction;", access = "private"))]
    pub unmappableCharacterAction: Field<Object>,
    #[cfg_attr(any(), java_field(name = "state", descriptor = "I", access = "private"))]
    pub state: Field<i32>,
}

impl CharsetDecoder {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/nio/charset/Charset;FFLjava/lang/String;)V", access = "private"))]
    // java: <init>(Ljava/nio/charset/Charset;FFLjava/lang/String;)V
    pub fn new__charse_f_f_str(cs: Object, averageCharsPerByte: f32, maxCharsPerByte: f32, replacement: String) -> Result<Self> {
        let this = Self { charset: Field::new(Default::default()), averageCharsPerByte: Field::new(0.0), maxCharsPerByte: Field::new(0.0), replacement: Field::new(String::new()), malformedInputAction: Field::new(Default::default()), unmappableCharacterAction: Field::new(Default::default()), state: Field::new(0) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.malformedInputAction.set(CodingErrorAction::REPORT());
        this.unmappableCharacterAction.set(CodingErrorAction::REPORT());
        this.state.set(0i32);
        this.charset.set(cs);
        /* TODO: fcmpl  */
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: fcmpl  */
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: fcmpl  */
        return Err(JvmError::Custom(String::from("athrow")));
        this.replacement.set(replacement);
        this.averageCharsPerByte.set(averageCharsPerByte);
        this.maxCharsPerByte.set(maxCharsPerByte);
        let _t0 = this.replaceWith(replacement)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/nio/charset/Charset;FF)V", access = "protected"))]
    // java: <init>(Ljava/nio/charset/Charset;FF)V
    pub fn new__charse_f_f(cs: Object, averageCharsPerByte: f32, maxCharsPerByte: f32) -> Result<Self> {
        let this = Self { charset: Field::new(Default::default()), averageCharsPerByte: Field::new(0.0), maxCharsPerByte: Field::new(0.0), replacement: Field::new(String::new()), malformedInputAction: Field::new(Default::default()), unmappableCharacterAction: Field::new(Default::default()), state: Field::new(0) };
        /* invokespecial Method java/nio/charset/CharsetDecoder.<init>:(Ljava/nio/charset/Charset;FFLjava/lang/String;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "charset", descriptor = "()Ljava/nio/charset/Charset;", access = "public final"))]
    pub fn charset(&self) -> Result<Object> {
        let this = self;
        Ok(this.charset.get())
    }

    #[cfg_attr(any(), java_method(name = "replacement", descriptor = "()Ljava/lang/String;", access = "public final"))]
    pub fn replacement(&self) -> Result<String> {
        let this = self;
        Ok(this.replacement.get())
    }

    #[cfg_attr(any(), java_method(name = "replaceWith", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/CharsetDecoder;", access = "public final"))]
    pub fn replaceWith(&self, newReplacement: String) -> Result<Object> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0 = newReplacement.length()?;
        let mut len: i32 = _t0;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: fcmpl  */
        return Err(JvmError::Custom(String::from("athrow")));
        this.replacement.set(newReplacement);
        this.implReplaceWith(this.replacement.get())?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "implReplaceWith", descriptor = "(Ljava/lang/String;)V", access = "protected"))]
    pub fn implReplaceWith(&self, newReplacement: String) -> Result<()> {
        let this = self;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "malformedInputAction", descriptor = "()Ljava/nio/charset/CodingErrorAction;", access = "public"))]
    pub fn malformedInputAction(&self) -> Result<Object> {
        let this = self;
        Ok(this.malformedInputAction.get())
    }

    #[cfg_attr(any(), java_method(name = "onMalformedInput", descriptor = "(Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetDecoder;", access = "public final"))]
    pub fn onMalformedInput(&self, newAction: Object) -> Result<Object> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        this.malformedInputAction.set(newAction);
        this.implOnMalformedInput(newAction)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "implOnMalformedInput", descriptor = "(Ljava/nio/charset/CodingErrorAction;)V", access = "protected"))]
    pub fn implOnMalformedInput(&self, newAction: Object) -> Result<()> {
        let this = self;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "unmappableCharacterAction", descriptor = "()Ljava/nio/charset/CodingErrorAction;", access = "public"))]
    pub fn unmappableCharacterAction(&self) -> Result<Object> {
        let this = self;
        Ok(this.unmappableCharacterAction.get())
    }

    #[cfg_attr(any(), java_method(name = "onUnmappableCharacter", descriptor = "(Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetDecoder;", access = "public final"))]
    pub fn onUnmappableCharacter(&self, newAction: Object) -> Result<Object> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        this.unmappableCharacterAction.set(newAction);
        this.implOnUnmappableCharacter(newAction)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "implOnUnmappableCharacter", descriptor = "(Ljava/nio/charset/CodingErrorAction;)V", access = "protected"))]
    pub fn implOnUnmappableCharacter(&self, newAction: Object) -> Result<()> {
        let this = self;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "averageCharsPerByte", descriptor = "()F", access = "public final"))]
    pub fn averageCharsPerByte(&self) -> Result<f32> {
        let this = self;
        Ok(this.averageCharsPerByte.get())
    }

    #[cfg_attr(any(), java_method(name = "maxCharsPerByte", descriptor = "()F", access = "public final"))]
    pub fn maxCharsPerByte(&self) -> Result<f32> {
        let this = self;
        Ok(this.maxCharsPerByte.get())
    }

    #[cfg_attr(any(), java_method(name = "decode", descriptor = "(Ljava/nio/ByteBuffer;Ljava/nio/CharBuffer;Z)Ljava/nio/charset/CoderResult;", access = "public final"))]
    // java: decode(Ljava/nio/ByteBuffer;Ljava/nio/CharBuffer;Z)Ljava/nio/charset/CoderResult;
    pub fn decode__bytebu_charbu_z(&self, in_: Object, out: Object, endOfInput: bool) -> Result<Object> {
        let this = self;
        let mut newState: i32 = endOfInput==0i32;
        this.throwIllegalStateException(this.state.get(), newState)?;
        this.state.set(newState);
        loop {
            let _t0 = this.decodeLoop(in_, out)?;
            let mut cr: Object = _t0;
            let mut x: i32 = todo!("stack underflow");
            return Err(JvmError::Custom(String::from("athrow")));
            let _t1 = cr.isOverflow()?;
            return Ok(cr);
            let _t2 = cr.isUnderflow()?;
            let _t3 = in_.hasRemaining()?;
            let _t4 = in_.remaining()?;
            let _t5: Object = CoderResult::malformedForLength(_t4)?;
            cr = _t5;
            return Ok(cr);
            /* TODO: aconst_null  */
            x = _t3;
            let _t6 = cr.isMalformed()?;
            x = this.malformedInputAction.get();
            let _t7 = cr.isUnmappable()?;
            x = this.unmappableCharacterAction.get();
            let _t8 = cr.toString()?;
            return Err(JvmError::Custom(String::from("athrow")));
            return Ok(cr);
            let _t9 = out.remaining()?;
            let _t10 = this.replacement.get().length()?;
            return Ok(CoderResult::OVERFLOW());
            let _t11 = out.put(this.replacement.get())?;
            if /* if_acmpne */ true { break; }
            let _t0 = in_.position()?;
            let _t1 = cr.length()?;
            let _t2 = in_.position((_t0).wrapping_add(_t1))?;
        }
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "flush", descriptor = "(Ljava/nio/CharBuffer;)Ljava/nio/charset/CoderResult;", access = "public final"))]
    pub fn flush(&self, out: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.implFlush(out)?;
        let mut cr: Object = _t0;
        let _t1 = cr.isUnderflow()?;
        this.state.set(3i32);
        return Ok(cr);
        this.throwIllegalStateException(this.state.get(), 3i32)?;
        Ok(CoderResult::UNDERFLOW())
    }

    #[cfg_attr(any(), java_method(name = "implFlush", descriptor = "(Ljava/nio/CharBuffer;)Ljava/nio/charset/CoderResult;", access = "protected"))]
    pub fn implFlush(&self, out: Object) -> Result<Object> {
        let this = self;
        Ok(CoderResult::UNDERFLOW())
    }

    #[cfg_attr(any(), java_method(name = "reset", descriptor = "()Ljava/nio/charset/CharsetDecoder;", access = "public final"))]
    pub fn reset(&self) -> Result<Object> {
        let this = self;
        this.implReset()?;
        this.state.set(0i32);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "implReset", descriptor = "()V", access = "protected"))]
    pub fn implReset(&self) -> Result<()> {
        let this = self;
        Ok(())
    }

    #[cfg_attr(any(), java_native(name = "decodeLoop", descriptor = "(Ljava/nio/ByteBuffer;Ljava/nio/CharBuffer;)Ljava/nio/charset/CoderResult;", access = "protected abstract"))]
    pub fn decodeLoop(&self, arg0: Object, arg1: Object) -> Result<Object> {
        todo!("abstract java/nio/charset/CharsetDecoder.decodeLoop")
    }

    #[cfg_attr(any(), java_method(name = "decode", descriptor = "(Ljava/nio/ByteBuffer;)Ljava/nio/CharBuffer;", access = "public final"))]
    // java: decode(Ljava/nio/ByteBuffer;)Ljava/nio/CharBuffer;
    pub fn decode__bytebu(&self, in_: Object) -> Result<Object> {
        let this = self;
        let _t0 = in_.remaining()?;
        let _t1 = this.averageCharsPerByte()?;
        let _t2: i32 = ((((_t0 as f32)*_t1) as i32)).min(2147483639i32);
        let mut n: i32 = _t2;
        let _t3: Object = CharBuffer::allocate(n)?;
        let mut out: Object = _t3;
        let _t4 = in_.remaining()?;
        return Ok(out);
        let _t5 = this.reset()?;
        loop {
            let _t0 = in_.hasRemaining()?;
            let _t1 = this.decode(in_, out, 1i32)?;
            let mut cr: Object = CoderResult::UNDERFLOW();
            let _t2 = cr.isUnderflow()?;
            let _t3 = this.flush(out)?;
            cr = _t3;
            let _t4 = cr.isUnderflow()?;
            let _t5 = cr.isOverflow()?;
            if _t5==0i32 { break; }
            let _t0: i32 = ((n).wrapping_add(1i32)).min(1024i32);
            let _t1: i32 = ArraysSupport::newLength(n, _t0, (n).wrapping_add(1i32))?;
            n = _t1;
            let _t2: Object = CharBuffer::allocate(n)?;
            let mut o: Object = _t2;
            let _t3 = out.flip()?;
            let _t4 = o.put(out)?;
            out = o;
        }
        cr.throwException()?;
        let _t6 = out.flip()?;
        Ok(out)
    }

    #[cfg_attr(any(), java_method(name = "isAutoDetecting", descriptor = "()Z", access = "public"))]
    pub fn isAutoDetecting(&self) -> Result<bool> {
        let this = self;
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "isCharsetDetected", descriptor = "()Z", access = "public"))]
    pub fn isCharsetDetected(&self) -> Result<bool> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "detectedCharset", descriptor = "()Ljava/nio/charset/Charset;", access = "public"))]
    pub fn detectedCharset(&self) -> Result<Object> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "throwIllegalStateException", descriptor = "(II)V", access = "private"))]
    pub fn throwIllegalStateException(&self, from: i32, to: i32) -> Result<()> {
        let this = self;
        String::new().append(&String::from("Current state ="))?;
        String::new().append(&CharsetDecoder::stateNames()[from as usize].clone())?;
        String::new().append(&String::from(", new state ="))?;
        String::new().append(&CharsetDecoder::stateNames()[to as usize].clone())?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }
}
