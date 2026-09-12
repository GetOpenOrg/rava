#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/nio/charset/Charset",
    super_class = "java/lang/Object",
    interfaces  = "java/lang/Comparable",
    access      = "public abstract",
    source      = "Charset.java",
))]
pub struct Charset {
    #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub name: Field<String>,
    #[cfg_attr(any(), java_field(name = "aliases", descriptor = "[Ljava/lang/String;", access = "private final"))]
    pub aliases: Field<Vec<String>>,
    #[cfg_attr(any(), java_field(name = "aliasSet", descriptor = "Ljava/util/Set;", access = "private"))]
    pub aliasSet: Field<Object>,
}

impl Charset {
    #[cfg_attr(any(), java_method(name = "checkName", descriptor = "(Ljava/lang/String;)V", access = "private static"))]
    pub fn checkName(s: String) -> Result<()> {
        let _t0 = s.length()?;
        let mut n: i32 = _t0;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut i: i32 = 0i32;
        loop {
            if i >= n { break; }
            let _t0 = s.charAt(i)?;
            let mut c: i32 = _t0;
            return Err(JvmError::Custom(String::from("athrow")));
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "cache", descriptor = "(Ljava/lang/String;Ljava/nio/charset/Charset;)V", access = "private static"))]
    pub fn cache(charsetName: String, cs: Object) -> Result<()> {
        Charset::cache2(Charset::cache1());
        let mut _arr0: Vec<Object> = Vec::with_capacity(2i32 as usize);
        _arr0[0i32 as usize] = charsetName;
        _arr0[1i32 as usize] = cs;
        Charset::cache1(_arr0);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "providers", descriptor = "()Ljava/util/Iterator;", access = "private static"))]
    pub fn providers() -> Result<Object> {
        Ok(Charset_1::new()?)
    }

    #[cfg_attr(any(), java_method(name = "tryBeginLookup", descriptor = "()Ljava/lang/Object;", access = "private static"))]
    pub fn tryBeginLookup() -> Result<Object> {
        let _t0 = Charset$ThreadTrackHolder::TRACKER().tryBegin()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "endLookup", descriptor = "(Ljava/lang/Object;)V", access = "private static"))]
    pub fn endLookup(key: Object) -> Result<()> {
        Charset$ThreadTrackHolder::TRACKER().end(key)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "lookupViaProviders", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/Charset;", access = "private static"))]
    pub fn lookupViaProviders(charsetName: String) -> Result<Object> {
        let _t0: bool = VM::isBooted()?;
        /* TODO: aconst_null  */
        return Ok(_t0);
        let _t1: Object = Charset::tryBeginLookup()?;
        let mut key: Object = _t1;
        /* TODO: aconst_null  */
        return Ok(key);
        let _t2: Object = AccessController::doPrivileged(Charset_2::new(charsetName)?)?;
        let mut local_2: Object = _t2;
        Charset::endLookup(key)?;
        return Ok(local_2);
        let mut local_3: i32 = todo!("stack underflow");
        Charset::endLookup(key)?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "lookupExtendedCharset", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/Charset;", access = "private static"))]
    pub fn lookupExtendedCharset(charsetName: String) -> Result<Object> {
        let _t0: bool = VM::isBooted()?;
        /* TODO: aconst_null  */
        return Ok(_t0);
        let mut ecps: Vec<Object> = Charset$ExtendedProviderHolder::extendedProviders();
        let mut local_2: Vec<Object> = ecps;
        let mut local_3: i32 = (local_2.len() as i32);
        let mut local_4: i32 = 0i32;
        loop {
            if local_4 >= local_3 { break; }
            let mut cp: Object = local_2[local_4 as usize].clone();
            let _t0 = cp.charsetForName(charsetName)?;
            let mut cs: Object = _t0;
            return Ok(cs);
            local_4 = local_4.wrapping_add(1i32);
        }
        /* TODO: aconst_null  */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "lookup", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/Charset;", access = "private static"))]
    pub fn lookup(charsetName: String) -> Result<Object> {
        return Err(JvmError::Custom(String::from("athrow")));
        let mut a: Vec<Object> = Charset::cache1();
        let _t0 = charsetName.equals(a[0i32 as usize].clone())?;
        return Ok(a[1i32 as usize].clone());
        let _t1: Object = Charset::lookup2(charsetName)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "lookup2", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/Charset;", access = "private static"))]
    pub fn lookup2(charsetName: String) -> Result<Object> {
        let mut a: Vec<Object> = Charset::cache2();
        let _t0 = charsetName.equals(a[0i32 as usize].clone())?;
        Charset::cache2(Charset::cache1());
        Charset::cache1(a);
        return Ok(a[1i32 as usize].clone());
        let _t1 = Charset::standardProvider().charsetForName(charsetName)?;
        let mut cs: Object = _t1;
        let _t2: Object = Charset::lookupExtendedCharset(charsetName)?;
        cs = _t2;
        let _t3: Object = Charset::lookupViaProviders(charsetName)?;
        cs = _t3;
        Charset::cache(charsetName, cs)?;
        return Ok(cs);
        Charset::checkName(charsetName)?;
        /* TODO: aconst_null  */
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "isSupported", descriptor = "(Ljava/lang/String;)Z", access = "public static"))]
    pub fn isSupported(charsetName: String) -> Result<bool> {
        let _t0: Object = Charset::lookup(charsetName)?;
        Ok(!_t0.is_none())
    }

    #[cfg_attr(any(), java_method(name = "forName", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/Charset;", access = "public static"))]
    // java: forName(Ljava/lang/String;)Ljava/nio/charset/Charset;
    pub fn forName__str(charsetName: String) -> Result<Object> {
        let _t0: Object = Charset::lookup(charsetName)?;
        let mut cs: Object = _t0;
        return Ok(cs);
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "forName", descriptor = "(Ljava/lang/String;Ljava/nio/charset/Charset;)Ljava/nio/charset/Charset;", access = "public static"))]
    // java: forName(Ljava/lang/String;Ljava/nio/charset/Charset;)Ljava/nio/charset/Charset;
    pub fn forName__str_charse(charsetName: String, fallback: Object) -> Result<Object> {
        let _t0: Object = Charset::lookup(charsetName)?;
        let mut cs: Object = _t0;
        return Ok(fallback);
        cs = cs;
        Ok(fallback)
    }

    #[cfg_attr(any(), java_method(name = "put", descriptor = "(Ljava/util/Iterator;Ljava/util/Map;)V", access = "private static"))]
    pub fn put(i: Object, m: Object) -> Result<()> {
        loop {
            let _t0 = i.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = i.next()?;
            let mut cs: Object = _t0;
            let _t1 = cs.name()?;
            let _t2 = m.putIfAbsent(_t1, cs)?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "availableCharsets", descriptor = "()Ljava/util/SortedMap;", access = "public static"))]
    pub fn availableCharsets() -> Result<Object> {
        let _t0: Object = AccessController::doPrivileged(Charset_3::new()?)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "defaultCharset", descriptor = "()Ljava/nio/charset/Charset;", access = "public static"))]
    pub fn defaultCharset() -> Result<Object> {
        let mut local_0: i32 = 18i32;
        /* TODO: monitorenter  */
        let _t0: String = StaticProperty::fileEncoding()?;
        let _t1 = Charset::standardProvider().charsetForName(_t0)?;
        let mut cs: Object = _t1;
        Charset::defaultCharset(cs);
        Charset::defaultCharset(UTF_8::INSTANCE());
        /* TODO: monitorexit  */
        let mut local_2: i32 = local_0;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(Charset::defaultCharset())
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;[Ljava/lang/String;)V", access = "protected"))]
    pub fn new(canonicalName: String, aliases: Vec<String>) -> Result<Self> {
        let this = Self { name: Field::new(String::new()), aliases: Field::new(Default::default()), aliasSet: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        let _t0 = this.getClass()?;
        let _t1 = _t0.getClassLoader()?;
        let _t2: bool = VM::isSystemDomainLoader(_t1)?;
        let _t3: Vec<Object> = Arrays::copyOf(&aliases, (aliases.len() as i32))?;
        let mut as_: Vec<Object> = _t3;
        Charset::checkName(canonicalName)?;
        let mut i: i32 = 0i32;
        loop {
            if i >= (as_.len() as i32) { break; }
            Charset::checkName(as_[i as usize].clone())?;
            i = i.wrapping_add(1i32);
        }
        this.name.set(canonicalName);
        this.aliases.set(as_);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "name", descriptor = "()Ljava/lang/String;", access = "public final"))]
    pub fn name(&self) -> Result<String> {
        let this = self;
        Ok(this.name.get())
    }

    #[cfg_attr(any(), java_method(name = "aliases", descriptor = "()Ljava/util/Set;", access = "public final"))]
    pub fn aliases(&self) -> Result<Object> {
        let this = self;
        let mut set: Object = this.aliasSet.get();
        let _t0: Object = Set::of(&this.aliases.get())?;
        set = _t0;
        this.aliasSet.set(set);
        Ok(set)
    }

    #[cfg_attr(any(), java_method(name = "displayName", descriptor = "()Ljava/lang/String;", access = "public"))]
    // java: displayName()Ljava/lang/String;
    pub fn displayName(&self) -> Result<String> {
        let this = self;
        Ok(this.name.get())
    }

    #[cfg_attr(any(), java_method(name = "isRegistered", descriptor = "()Z", access = "public final"))]
    pub fn isRegistered(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.name.get().startsWith(String::from("X-"))?;
        let _t1 = this.name.get().startsWith(String::from("x-"))?;
        Ok(_t1==0i32)
    }

    #[cfg_attr(any(), java_method(name = "displayName", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public"))]
    // java: displayName(Ljava/util/Locale;)Ljava/lang/String;
    pub fn displayName__locale(&self, locale: Object) -> Result<String> {
        let this = self;
        Ok(this.name.get())
    }

    #[cfg_attr(any(), java_native(name = "contains", descriptor = "(Ljava/nio/charset/Charset;)Z", access = "public abstract"))]
    pub fn contains(&self, arg0: Object) -> Result<bool> {
        todo!("abstract java/nio/charset/Charset.contains")
    }

    #[cfg_attr(any(), java_native(name = "newDecoder", descriptor = "()Ljava/nio/charset/CharsetDecoder;", access = "public abstract"))]
    pub fn newDecoder(&self) -> Result<Object> {
        todo!("abstract java/nio/charset/Charset.newDecoder")
    }

    #[cfg_attr(any(), java_native(name = "newEncoder", descriptor = "()Ljava/nio/charset/CharsetEncoder;", access = "public abstract"))]
    pub fn newEncoder(&self) -> Result<Object> {
        todo!("abstract java/nio/charset/Charset.newEncoder")
    }

    #[cfg_attr(any(), java_method(name = "canEncode", descriptor = "()Z", access = "public"))]
    pub fn canEncode(&self) -> Result<bool> {
        let this = self;
        Ok(1i32)
    }

    #[cfg_attr(any(), java_method(name = "decode", descriptor = "(Ljava/nio/ByteBuffer;)Ljava/nio/CharBuffer;", access = "public final"))]
    pub fn decode(&self, bb: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = ThreadLocalCoders::decoderFor(this)?;
        let _t1 = _t0.onMalformedInput(CodingErrorAction::REPLACE())?;
        let _t2 = _t1.onUnmappableCharacter(CodingErrorAction::REPLACE())?;
        let _t3 = _t2.decode(bb)?;
        return Ok(_t3);
        let mut x: i32 = todo!("stack underflow");
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "encode", descriptor = "(Ljava/nio/CharBuffer;)Ljava/nio/ByteBuffer;", access = "public final"))]
    // java: encode(Ljava/nio/CharBuffer;)Ljava/nio/ByteBuffer;
    pub fn encode__charbu(&self, cb: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = ThreadLocalCoders::encoderFor(this)?;
        let _t1 = _t0.onMalformedInput(CodingErrorAction::REPLACE())?;
        let _t2 = _t1.onUnmappableCharacter(CodingErrorAction::REPLACE())?;
        let _t3 = _t2.encode(cb)?;
        return Ok(_t3);
        let mut x: i32 = todo!("stack underflow");
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "encode", descriptor = "(Ljava/lang/String;)Ljava/nio/ByteBuffer;", access = "public final"))]
    // java: encode(Ljava/lang/String;)Ljava/nio/ByteBuffer;
    pub fn encode__str(&self, str: String) -> Result<Object> {
        let this = self;
        let _t0: Object = CharBuffer::wrap(str)?;
        let _t1 = this.encode(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "(Ljava/nio/charset/Charset;)I", access = "public final"))]
    pub fn compareTo(&self, that: Object) -> Result<i32> {
        let this = self;
        let _t0 = this.name()?;
        let _t1 = that.name()?;
        let _t2 = _t0.compareToIgnoreCase(_t1)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public final"))]
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.name()?;
        let _t1 = _t0.hashCode()?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public final"))]
    pub fn equals(&self, ob: Object) -> Result<bool> {
        let this = self;
        return Ok(0i32);
        return Ok(1i32);
        let _t0 = ob.name()?;
        let _t1 = this.name.get().equals(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public final"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.name()?;
        Ok(_t0)
    }
}
