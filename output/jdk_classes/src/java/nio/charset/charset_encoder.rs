#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/nio/charset/CharsetEncoder",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public abstract",
    source      = "CharsetEncoder.java",
))]
pub struct CharsetEncoder {
    #[cfg_attr(any(), java_field(name = "charset", descriptor = "Ljava/nio/charset/Charset;", access = "private final"))]
    pub charset: Field<Object>,
    #[cfg_attr(any(), java_field(name = "averageBytesPerChar", descriptor = "F", access = "private final"))]
    pub averageBytesPerChar: Field<f32>,
    #[cfg_attr(any(), java_field(name = "maxBytesPerChar", descriptor = "F", access = "private final"))]
    pub maxBytesPerChar: Field<f32>,
    #[cfg_attr(any(), java_field(name = "replacement", descriptor = "[B", access = "private"))]
    pub replacement: Field<Vec<i8>>,
    #[cfg_attr(any(), java_field(name = "malformedInputAction", descriptor = "Ljava/nio/charset/CodingErrorAction;", access = "private"))]
    pub malformedInputAction: Field<Object>,
    #[cfg_attr(any(), java_field(name = "unmappableCharacterAction", descriptor = "Ljava/nio/charset/CodingErrorAction;", access = "private"))]
    pub unmappableCharacterAction: Field<Object>,
    #[cfg_attr(any(), java_field(name = "state", descriptor = "I", access = "private"))]
    pub state: Field<i32>,
    #[cfg_attr(any(), java_field(name = "cachedDecoder", descriptor = "Ljava/lang/ref/WeakReference;", access = "private"))]
    pub cachedDecoder: Field<Object>,
}

impl CharsetEncoder {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/nio/charset/Charset;FF[B)V", access = "protected"))]
    // java: <init>(Ljava/nio/charset/Charset;FF[B)V
    pub fn new__charse_f_f_arr_b(cs: Object, averageBytesPerChar: f32, maxBytesPerChar: f32, replacement: Vec<i8>) -> Result<Self> {
        let this = Self { charset: Field::new(Default::default()), averageBytesPerChar: Field::new(0.0), maxBytesPerChar: Field::new(0.0), replacement: Field::new(Default::default()), malformedInputAction: Field::new(Default::default()), unmappableCharacterAction: Field::new(Default::default()), state: Field::new(0), cachedDecoder: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.malformedInputAction.set(CodingErrorAction::REPORT());
        this.unmappableCharacterAction.set(CodingErrorAction::REPORT());
        this.state.set(0i32);
        /* TODO: aconst_null  */
        todo!("stack underflow").cachedDecoder.set(this);
        this.charset.set(cs);
        /* TODO: fcmpl  */
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: fcmpl  */
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: fcmpl  */
        return Err(JvmError::Custom(String::from("athrow")));
        this.replacement.set(replacement);
        this.averageBytesPerChar.set(averageBytesPerChar);
        this.maxBytesPerChar.set(maxBytesPerChar);
        let _t0 = this.replaceWith(replacement)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/nio/charset/Charset;FF)V", access = "protected"))]
    // java: <init>(Ljava/nio/charset/Charset;FF)V
    pub fn new__charse_f_f(cs: Object, averageBytesPerChar: f32, maxBytesPerChar: f32) -> Result<Self> {
        let this = Self { charset: Field::new(Default::default()), averageBytesPerChar: Field::new(0.0), maxBytesPerChar: Field::new(0.0), replacement: Field::new(Default::default()), malformedInputAction: Field::new(Default::default()), unmappableCharacterAction: Field::new(Default::default()), state: Field::new(0), cachedDecoder: Field::new(Default::default()) };
        let mut _arr0: Vec<i8> = vec![0i8; 1i32 as usize];
        _arr0[0i32 as usize] = 63i32;
        /* invokespecial Method java/nio/charset/CharsetEncoder.<init>:(Ljava/nio/charset/Charset;FF[B)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "charset", descriptor = "()Ljava/nio/charset/Charset;", access = "public final"))]
    pub fn charset(&self) -> Result<Object> {
        let this = self;
        Ok(this.charset.get())
    }

    #[cfg_attr(any(), java_method(name = "replacement", descriptor = "()[B", access = "public final"))]
    pub fn replacement(&self) -> Result<Vec<i8>> {
        let this = self;
        let _t0: Vec<i8> = Arrays::copyOf(&this.replacement.get(), (this.replacement.get().len() as i32))?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "replaceWith", descriptor = "([B)Ljava/nio/charset/CharsetEncoder;", access = "public final"))]
    pub fn replaceWith(&self, newReplacement: Vec<i8>) -> Result<Object> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut len: i32 = (newReplacement.len() as i32);
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: fcmpl  */
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0 = this.isLegalReplacement(newReplacement)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1: Vec<i8> = Arrays::copyOf(&newReplacement, (newReplacement.len() as i32))?;
        this.replacement.set(_t1);
        this.implReplaceWith(this.replacement.get())?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "implReplaceWith", descriptor = "([B)V", access = "protected"))]
    pub fn implReplaceWith(&self, newReplacement: Vec<i8>) -> Result<()> {
        let this = self;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "isLegalReplacement", descriptor = "([B)Z", access = "public"))]
    pub fn isLegalReplacement(&self, repl: Vec<i8>) -> Result<bool> {
        let this = self;
        let mut wr: Object = this.cachedDecoder.get();
        /* TODO: aconst_null  */
        let mut dec: i32 = todo!("stack underflow");
        let _t0 = wr.get()?;
        dec = _t0;
        let _t1 = this.charset()?;
        let _t2 = _t1.newDecoder()?;
        dec = _t2;
        let _t3 = dec.onMalformedInput(CodingErrorAction::REPORT())?;
        let _t4 = dec.onUnmappableCharacter(CodingErrorAction::REPORT())?;
        this.cachedDecoder.set(WeakReference::new(dec)?);
        let _t5 = dec.reset()?;
        let _t6: Object = ByteBuffer::wrap(&repl)?;
        let mut bb: Object = _t6;
        let _t7 = bb.remaining()?;
        let _t8 = dec.maxCharsPerByte()?;
        let _t9: Object = CharBuffer::allocate((((_t7 as f32)*_t8) as i32))?;
        let mut cb: Object = _t9;
        let _t10 = dec.decode(bb, cb, 1i32)?;
        let mut cr: Object = _t10;
        let _t11 = cr.isError()?;
        Ok(_t11==0i32)
    }

    #[cfg_attr(any(), java_method(name = "malformedInputAction", descriptor = "()Ljava/nio/charset/CodingErrorAction;", access = "public"))]
    pub fn malformedInputAction(&self) -> Result<Object> {
        let this = self;
        Ok(this.malformedInputAction.get())
    }

    #[cfg_attr(any(), java_method(name = "onMalformedInput", descriptor = "(Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetEncoder;", access = "public final"))]
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

    #[cfg_attr(any(), java_method(name = "onUnmappableCharacter", descriptor = "(Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetEncoder;", access = "public final"))]
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

    #[cfg_attr(any(), java_method(name = "averageBytesPerChar", descriptor = "()F", access = "public final"))]
    pub fn averageBytesPerChar(&self) -> Result<f32> {
        let this = self;
        Ok(this.averageBytesPerChar.get())
    }

    #[cfg_attr(any(), java_method(name = "maxBytesPerChar", descriptor = "()F", access = "public final"))]
    pub fn maxBytesPerChar(&self) -> Result<f32> {
        let this = self;
        Ok(this.maxBytesPerChar.get())
    }

    #[cfg_attr(any(), java_method(name = "encode", descriptor = "(Ljava/nio/CharBuffer;Ljava/nio/ByteBuffer;Z)Ljava/nio/charset/CoderResult;", access = "public final"))]
    // java: encode(Ljava/nio/CharBuffer;Ljava/nio/ByteBuffer;Z)Ljava/nio/charset/CoderResult;
    pub fn encode__charbu_bytebu_z(&self, in_: Object, out: Object, endOfInput: bool) -> Result<Object> {
        let this = self;
        let mut newState: i32 = endOfInput==0i32;
        this.throwIllegalStateException(this.state.get(), newState)?;
        this.state.set(newState);
        loop {
            let _t0 = this.encodeLoop(in_, out)?;
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
            return Ok(CoderResult::OVERFLOW());
            let _t10 = out.put(this.replacement.get())?;
            if /* if_acmpne */ true { break; }
            let _t0 = in_.position()?;
            let _t1 = cr.length()?;
            let _t2 = in_.position((_t0).wrapping_add(_t1))?;
        }
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "flush", descriptor = "(Ljava/nio/ByteBuffer;)Ljava/nio/charset/CoderResult;", access = "public final"))]
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

    #[cfg_attr(any(), java_method(name = "implFlush", descriptor = "(Ljava/nio/ByteBuffer;)Ljava/nio/charset/CoderResult;", access = "protected"))]
    pub fn implFlush(&self, out: Object) -> Result<Object> {
        let this = self;
        Ok(CoderResult::UNDERFLOW())
    }

    #[cfg_attr(any(), java_method(name = "reset", descriptor = "()Ljava/nio/charset/CharsetEncoder;", access = "public final"))]
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

    #[cfg_attr(any(), java_native(name = "encodeLoop", descriptor = "(Ljava/nio/CharBuffer;Ljava/nio/ByteBuffer;)Ljava/nio/charset/CoderResult;", access = "protected abstract"))]
    pub fn encodeLoop(&self, arg0: Object, arg1: Object) -> Result<Object> {
        todo!("abstract java/nio/charset/CharsetEncoder.encodeLoop")
    }

    #[cfg_attr(any(), java_method(name = "encode", descriptor = "(Ljava/nio/CharBuffer;)Ljava/nio/ByteBuffer;", access = "public final"))]
    // java: encode(Ljava/nio/CharBuffer;)Ljava/nio/ByteBuffer;
    pub fn encode__charbu(&self, in_: Object) -> Result<Object> {
        let this = self;
        let _t0 = in_.remaining()?;
        let _t1 = this.averageBytesPerChar()?;
        let _t2: i32 = ((((_t0 as f32)*_t1) as i32)).min(2147483639i32);
        let mut n: i32 = _t2;
        let _t3: Object = ByteBuffer::allocate(n)?;
        let mut out: Object = _t3;
        let _t4 = in_.remaining()?;
        return Ok(out);
        let _t5 = this.reset()?;
        loop {
            let _t0 = in_.hasRemaining()?;
            let _t1 = this.encode(in_, out, 1i32)?;
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
            let _t2: Object = ByteBuffer::allocate(n)?;
            let mut o: Object = _t2;
            let _t3 = out.flip()?;
            let _t4 = o.put(out)?;
            out = o;
        }
        cr.throwException()?;
        let _t6 = out.flip()?;
        Ok(out)
    }

    #[cfg_attr(any(), java_method(name = "canEncode", descriptor = "(Ljava/nio/CharBuffer;)Z", access = "private"))]
    // java: canEncode(Ljava/nio/CharBuffer;)Z
    pub fn canEncode__charbu(&self, cb: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.reset()?;
        this.throwIllegalStateException(this.state.get(), 1i32)?;
        let _t1 = this.malformedInputAction()?;
        let mut ma: Object = _t1;
        let _t2 = this.unmappableCharacterAction()?;
        let mut ua: Object = _t2;
        let _t3 = this.onMalformedInput(CodingErrorAction::REPORT())?;
        let _t4 = this.onUnmappableCharacter(CodingErrorAction::REPORT())?;
        let _t5 = this.encode(cb)?;
        let _t6 = this.onMalformedInput(ma)?;
        let _t7 = this.onUnmappableCharacter(ua)?;
        let _t8 = this.reset()?;
        let mut x: i32 = this.state.get();
        let mut local_5: i32 = 0i32;
        let _t9 = this.onMalformedInput(ma)?;
        let _t10 = this.onUnmappableCharacter(ua)?;
        let _t11 = this.reset()?;
        return Ok(local_5);
        let mut local_6: i32 = 3i32;
        let _t12 = this.onMalformedInput(ma)?;
        let _t13 = this.onUnmappableCharacter(ua)?;
        let _t14 = this.reset()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(1i32)
    }

    #[cfg_attr(any(), java_method(name = "canEncode", descriptor = "(C)Z", access = "public"))]
    // java: canEncode(C)Z
    pub fn canEncode__c(&self, c: u16) -> Result<bool> {
        let this = self;
        let _t0: Object = CharBuffer::allocate(1i32)?;
        let mut cb: Object = _t0;
        let _t1 = cb.put(c)?;
        let _t2 = cb.flip()?;
        let _t3 = this.canEncode(cb)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "canEncode", descriptor = "(Ljava/lang/CharSequence;)Z", access = "public"))]
    // java: canEncode(Ljava/lang/CharSequence;)Z
    pub fn canEncode__seq(&self, cs: Object) -> Result<bool> {
        let this = self;
        let _t0 = cs.duplicate()?;
        let mut cb: Object = _t0;
        let _t1 = cs.toString()?;
        let _t2: Object = CharBuffer::wrap(_t1)?;
        cb = _t2;
        let _t3 = this.canEncode(cb)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "throwIllegalStateException", descriptor = "(II)V", access = "private"))]
    pub fn throwIllegalStateException(&self, from: i32, to: i32) -> Result<()> {
        let this = self;
        String::new().append(&String::from("Current state ="))?;
        String::new().append(&CharsetEncoder::stateNames()[from as usize].clone())?;
        String::new().append(&String::from(", new state ="))?;
        String::new().append(&CharsetEncoder::stateNames()[to as usize].clone())?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }
}
