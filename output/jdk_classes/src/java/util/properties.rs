#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Properties",
    super_class = "java/util/Hashtable",
    interfaces  = "",
    access      = "public",
    source      = "Properties.java",
))]
pub struct Properties {
    #[cfg_attr(any(), java_field(name = "defaults", descriptor = "Ljava/util/Properties;", access = "protected"))]
    pub defaults: Field<Object>,
    #[cfg_attr(any(), java_field(name = "map", descriptor = "Ljava/util/concurrent/ConcurrentHashMap;", access = "private"))]
    pub map: Field<Object>,
}

impl Properties {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public"))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { defaults: Field::new(Default::default()), map: Field::new(Default::default()) };
        /* TODO: aconst_null  */
        /* invokespecial Method java/util/Properties.<init>:(Ljava/util/Properties;I)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(I)V", access = "public"))]
    // java: <init>(I)V
    pub fn new__i(initialCapacity: i32) -> Result<Self> {
        let this = Self { defaults: Field::new(Default::default()), map: Field::new(Default::default()) };
        /* TODO: aconst_null  */
        /* invokespecial Method java/util/Properties.<init>:(Ljava/util/Properties;I)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/Properties;)V", access = "public"))]
    // java: <init>(Ljava/util/Properties;)V
    pub fn new__proper(defaults: Object) -> Result<Self> {
        let this = Self { defaults: Field::new(Default::default()), map: Field::new(Default::default()) };
        /* invokespecial Method java/util/Properties.<init>:(Ljava/util/Properties;I)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/Properties;I)V", access = "private"))]
    // java: <init>(Ljava/util/Properties;I)V
    pub fn new__proper_i(defaults: Object, initialCapacity: i32) -> Result<Self> {
        let this = Self { defaults: Field::new(Default::default()), map: Field::new(Default::default()) };
        /* TODO: aconst_null  */
        /* invokespecial Method java/util/Hashtable.<init>:(Ljava/lang/Void;)V */
        this.map.set(ConcurrentHashMap::new(initialCapacity)?);
        this.defaults.set(defaults);
        Properties::UNSAFE().storeFence()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "setProperty", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;", access = "public"))]
    pub fn setProperty(&self, key: String, value: String) -> Result<Object> {
        let this = self;
        let _t0 = this.put(key, value)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "load", descriptor = "(Ljava/io/Reader;)V", access = "public"))]
    // java: load(Ljava/io/Reader;)V
    pub fn load__reader(&self, reader: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(reader, String::from("reader parameter is null"))?;
        this.load0(Properties_LineReader::new(reader)?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "load", descriptor = "(Ljava/io/InputStream;)V", access = "public"))]
    // java: load(Ljava/io/InputStream;)V
    pub fn load__inputs(&self, inStream: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(inStream, String::from("inStream parameter is null"))?;
        this.load0(Properties_LineReader::new(inStream)?)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "load0", descriptor = "(Ljava/util/Properties$LineReader;)V", access = "private"))]
    pub fn load0(&self, lr: Object) -> Result<()> {
        let this = self;
        let mut outBuffer: String = String::new();
        loop {
            let _t0 = lr.readLine()?;
            let mut limit: i32 = _t0;
            if _t0<0i32 { break; }
            let mut keyLen: i32 = 0i32;
            let mut valueStart: i32 = limit;
            let mut hasSep: i32 = 0i32;
            let mut precedingBackslash: i32 = 0i32;
            let mut c: i32 = lr.lineBuf.get()[keyLen as usize];
            valueStart = (keyLen).wrapping_add(1i32);
            hasSep = 1i32;
            valueStart = (keyLen).wrapping_add(1i32);
            precedingBackslash = 0i32;
            precedingBackslash = 0i32;
            keyLen = keyLen.wrapping_add(1i32);
            c = lr.lineBuf.get()[valueStart as usize];
            hasSep = 1i32;
            valueStart = valueStart.wrapping_add(1i32);
            let _t0 = this.loadConvert(lr.lineBuf.get(), 0i32, keyLen, outBuffer)?;
            c = _t0;
            let _t1 = this.loadConvert(lr.lineBuf.get(), valueStart, (limit).wrapping_sub(valueStart), outBuffer)?;
            let mut value: String = _t1;
            let _t2 = this.put(c, value)?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "loadConvert", descriptor = "([CIILjava/lang/StringBuilder;)Ljava/lang/String;", access = "private"))]
    pub fn loadConvert(&self, in_: Vec<u16>, off: i32, len: i32, out: Object) -> Result<String> {
        let this = self;
        let mut end: i32 = (off).wrapping_add(len);
        let mut start: i32 = off;
        off = off.wrapping_add(1i32);
        let mut aChar: i32 = in_[off as usize];
        return Ok(String::new(in_, start, len)?);
        out.setLength(0i32)?;
        off = off.wrapping_sub(1i32);
        out.append(&in_)?;
        loop {
            if off >= end { break; }
            off = off.wrapping_add(1i32);
            aChar = in_[off as usize];
            off = off.wrapping_add(1i32);
            aChar = in_[off as usize];
            return Err(JvmError::Custom(String::from("athrow")));
            let mut value: i32 = 0i32;
            let mut i: i32 = 0i32;
            off = off.wrapping_add(1i32);
            aChar = in_[off as usize];
            /* TODO: tableswitch default:429 low:48 high:102 */
            return Err(JvmError::Custom(String::from("athrow")));
            value = ((((value<<(4i32&0x1f))).wrapping_add(10i32)).wrapping_add(aChar)).wrapping_sub(65i32);
            i = i.wrapping_add(1i32);
            /* TODO: i2c  */
            out.append(&value)?;
            aChar = 9i32;
            aChar = 13i32;
            aChar = 10i32;
            aChar = 12i32;
            out.append(&aChar)?;
            out.append(&aChar)?;
        }
        Ok(out)
    }

    #[cfg_attr(any(), java_method(name = "saveConvert", descriptor = "(Ljava/lang/String;ZZ)Ljava/lang/String;", access = "private"))]
    pub fn saveConvert(&self, theString: String, escapeSpace: bool, escapeUnicode: bool) -> Result<String> {
        let this = self;
        let _t0 = theString.length()?;
        let mut len: i32 = _t0;
        let mut bufLen: i32 = (len).wrapping_mul(2i32);
        bufLen = 2147483647i32;
        let mut outBuffer: String = String::new();
        let _t1: Object = HexFormat::of()?;
        let _t2 = _t1.withUpperCase()?;
        let mut hex: Object = _t2;
        let mut x: i32 = 0i32;
        loop {
            if x >= len { break; }
            let _t0 = theString.charAt(x)?;
            let mut aChar: i32 = _t0;
            outBuffer.append(&92i32)?;
            outBuffer.append(&92i32)?;
            outBuffer.append(&aChar)?;
            /* TODO: lookupswitch default:315 9:220 10:239 12:277 13:258 32:192 33:296 35:296 58:296 61:296 */
            outBuffer.append(&92i32)?;
            outBuffer.append(&32i32)?;
            outBuffer.append(&92i32)?;
            outBuffer.append(&116i32)?;
            outBuffer.append(&92i32)?;
            outBuffer.append(&110i32)?;
            outBuffer.append(&92i32)?;
            outBuffer.append(&114i32)?;
            outBuffer.append(&92i32)?;
            outBuffer.append(&102i32)?;
            outBuffer.append(&92i32)?;
            outBuffer.append(&aChar)?;
            outBuffer.append(&String::from("\u"))?;
            let _t1 = hex.toHexDigits(aChar)?;
            outBuffer.append(&_t1)?;
            outBuffer.append(&aChar)?;
            x = x.wrapping_add(1i32);
        }
        Ok(outBuffer)
    }

    #[cfg_attr(any(), java_method(name = "writeComments", descriptor = "(Ljava/io/BufferedWriter;Ljava/lang/String;)V", access = "private static"))]
    pub fn writeComments(bw: Object, comments: String) -> Result<()> {
        let _t0: Object = HexFormat::of()?;
        let _t1 = _t0.withUpperCase()?;
        let mut hex: Object = _t1;
        bw.write(String::from("#"))?;
        let _t2 = comments.length()?;
        let mut len: i32 = _t2;
        let mut current: i32 = 0i32;
        let mut last: i32 = 0i32;
        loop {
            if current >= len { break; }
            let _t0 = comments.charAt(current)?;
            let mut c: i32 = _t0;
            let _t1 = comments.substring(last, current)?;
            bw.write(_t1)?;
            bw.write(String::from("\u"))?;
            let _t2 = hex.toHexDigits(c)?;
            bw.write(_t2)?;
            bw.newLine()?;
            let _t3 = comments.charAt((current).wrapping_add(1i32))?;
            current = current.wrapping_add(1i32);
            let _t4 = comments.charAt((current).wrapping_add(1i32))?;
            let _t5 = comments.charAt((current).wrapping_add(1i32))?;
            bw.write(String::from("#"))?;
            last = (current).wrapping_add(1i32);
            current = current.wrapping_add(1i32);
        }
        let _t3 = comments.substring(last, current)?;
        bw.write(_t3)?;
        bw.newLine()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "save", descriptor = "(Ljava/io/OutputStream;Ljava/lang/String;)V", access = "public"))]
    pub fn save(&self, out: Object, comments: String) -> Result<()> {
        let this = self;
        this.store(out, comments)?;
        let mut local_3: i32 = todo!("stack underflow");
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "store", descriptor = "(Ljava/io/Writer;Ljava/lang/String;)V", access = "public"))]
    // java: store(Ljava/io/Writer;Ljava/lang/String;)V
    pub fn store__writer_str(&self, writer: Object, comments: String) -> Result<()> {
        let this = self;
        writer.store0(BufferedWriter::new(writer)?, comments, 0i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "store", descriptor = "(Ljava/io/OutputStream;Ljava/lang/String;)V", access = "public"))]
    // java: store(Ljava/io/OutputStream;Ljava/lang/String;)V
    pub fn store__output_str(&self, out: Object, comments: String) -> Result<()> {
        let this = self;
        this.store0(BufferedWriter::new(OutputStreamWriter::new(out, ISO_8859_1::INSTANCE())?)?, comments, 1i32)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "store0", descriptor = "(Ljava/io/BufferedWriter;Ljava/lang/String;Z)V", access = "private"))]
    pub fn store0(&self, bw: Object, comments: String, escUnicode: bool) -> Result<()> {
        let this = self;
        Properties::writeComments(bw, comments)?;
        Properties::writeDateComment(bw)?;
        let mut local_4: java/util/Properties = this;
        /* TODO: monitorenter  */
        let _t0 = this.entrySet()?;
        let mut entries: Object = _t0;
        let mut ss: Object = entries;
        entries = ArrayList::<_>::new()?;
        let _t1: Object = Map$Entry::comparingByKey()?;
        entries.sort(_t1)?;
        let _t2 = entries.iterator()?;
        ss = _t2;
        loop {
            let _t0 = ss.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = ss.next()?;
            let mut e: Object = _t0;
            let _t1 = e.getKey()?;
            let mut key: Object = _t1;
            let _t2 = e.getValue()?;
            let mut val: Object = _t2;
            let _t3 = this.saveConvert(key, 1i32, escUnicode)?;
            key = _t3;
            let _t4 = this.saveConvert(val, 0i32, escUnicode)?;
            val = _t4;
            String::new().append(&key)?;
            String::new().append(&String::from("="))?;
            String::new().append(&val)?;
            bw.write(String::new())?;
            bw.newLine()?;
        }
        /* TODO: monitorexit  */
        let mut local_10: java/util/Properties = local_4;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        bw.flush()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeDateComment", descriptor = "(Ljava/io/BufferedWriter;)V", access = "private static"))]
    pub fn writeDateComment(bw: Object) -> Result<()> {
        let _t0: String = StaticProperty::javaPropertiesDate()?;
        let mut sysPropVal: String = _t0;
        let _t1 = sysPropVal.isEmpty()?;
        Properties::writeComments(bw, sysPropVal)?;
        String::new().append(&String::from("#"))?;
        String::new().append(&Date::new()?)?;
        bw.write(String::new())?;
        bw.newLine()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "loadFromXML", descriptor = "(Ljava/io/InputStream;)V", access = "public"))]
    pub fn loadFromXML(&self, in_: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(in_)?;
        let mut handler: PropertiesDefaultHandler = PropertiesDefaultHandler::new()?;
        handler.load(this, in_)?;
        in_.close()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "storeToXML", descriptor = "(Ljava/io/OutputStream;Ljava/lang/String;)V", access = "public"))]
    // java: storeToXML(Ljava/io/OutputStream;Ljava/lang/String;)V
    pub fn storeToXML__output_str(&self, os: Object, comment: String) -> Result<()> {
        let this = self;
        this.storeToXML(os, comment, UTF_8::INSTANCE())?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "storeToXML", descriptor = "(Ljava/io/OutputStream;Ljava/lang/String;Ljava/lang/String;)V", access = "public"))]
    // java: storeToXML(Ljava/io/OutputStream;Ljava/lang/String;Ljava/lang/String;)V
    pub fn storeToXML__output_str_str(&self, os: Object, comment: String, encoding: String) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(os)?;
        let _t1: Object = Objects::requireNonNull(encoding)?;
        let _t2: Object = Charset::forName(encoding)?;
        let mut charset: Object = _t2;
        this.storeToXML(os, comment, charset)?;
        charset = todo!("stack underflow");
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "storeToXML", descriptor = "(Ljava/io/OutputStream;Ljava/lang/String;Ljava/nio/charset/Charset;)V", access = "public"))]
    // java: storeToXML(Ljava/io/OutputStream;Ljava/lang/String;Ljava/nio/charset/Charset;)V
    pub fn storeToXML__output_str_charse(&self, os: Object, comment: String, charset: Object) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(os, String::from("OutputStream"))?;
        let _t1: Object = Objects::requireNonNull(charset, String::from("Charset"))?;
        let mut handler: PropertiesDefaultHandler = PropertiesDefaultHandler::new()?;
        handler.store(this, os, comment, charset)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getProperty", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public"))]
    // java: getProperty(Ljava/lang/String;)Ljava/lang/String;
    pub fn getProperty__str(&self, key: String) -> Result<String> {
        let this = self;
        let _t0 = this.map.get().get(key)?;
        let mut oval: Object = _t0;
        /* TODO: aconst_null  */
        let mut sval: Object = oval;
        let mut defaults: Object = this.defaults.get();
        let _t1 = defaults.getProperty(key)?;
        Ok(sval)
    }

    #[cfg_attr(any(), java_method(name = "getProperty", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", access = "public"))]
    // java: getProperty(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
    pub fn getProperty__str_str(&self, key: String, defaultValue: String) -> Result<String> {
        let this = self;
        let _t0 = this.getProperty(key)?;
        let mut val: String = _t0;
        Ok(val)
    }

    #[cfg_attr(any(), java_method(name = "propertyNames", descriptor = "()Ljava/util/Enumeration;", access = "public"))]
    pub fn propertyNames(&self) -> Result<Object> {
        let this = self;
        let mut h: Hashtable = Hashtable::new()?;
        this.enumerate(h)?;
        let _t0 = h.keys()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "stringPropertyNames", descriptor = "()Ljava/util/Set;", access = "public"))]
    pub fn stringPropertyNames(&self) -> Result<Object> {
        let this = self;
        let mut h: HashMap<_, _> = HashMap::<_, _>::new()?;
        this.enumerateStringProperties(h)?;
        let _t0 = h.keySet()?;
        let _t1: Object = Collections::unmodifiableSet(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "list", descriptor = "(Ljava/io/PrintStream;)V", access = "public"))]
    // java: list(Ljava/io/PrintStream;)V
    pub fn list__prints(&self, out: Object) -> Result<()> {
        let this = self;
        out.println(String::from("-- listing properties --"))?;
        let mut h: HashMap<_, _> = HashMap::<_, _>::new()?;
        this.enumerate(h)?;
        let _t0 = h.entrySet()?;
        let _t1 = _t0.iterator()?;
        let mut local_3: Object = _t1;
        loop {
            let _t0 = local_3.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_3.next()?;
            let mut e: Object = _t0;
            let _t1 = e.getKey()?;
            let mut key: Object = _t1;
            let _t2 = e.getValue()?;
            let mut val: Object = _t2;
            let _t3 = val.length()?;
            let _t4 = val.substring(0i32, 37i32)?;
            String::new().append(&_t4)?;
            String::new().append(&String::from("..."))?;
            val = String::new();
            String::new().append(&key)?;
            String::new().append(&String::from("="))?;
            String::new().append(&val)?;
            out.println(String::new())?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "list", descriptor = "(Ljava/io/PrintWriter;)V", access = "public"))]
    // java: list(Ljava/io/PrintWriter;)V
    pub fn list__printw(&self, out: Object) -> Result<()> {
        let this = self;
        out.println(String::from("-- listing properties --"))?;
        let mut h: HashMap<_, _> = HashMap::<_, _>::new()?;
        this.enumerate(h)?;
        let _t0 = h.entrySet()?;
        let _t1 = _t0.iterator()?;
        let mut local_3: Object = _t1;
        loop {
            let _t0 = local_3.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_3.next()?;
            let mut e: Object = _t0;
            let _t1 = e.getKey()?;
            let mut key: Object = _t1;
            let _t2 = e.getValue()?;
            let mut val: Object = _t2;
            let _t3 = val.length()?;
            let _t4 = val.substring(0i32, 37i32)?;
            String::new().append(&_t4)?;
            String::new().append(&String::from("..."))?;
            val = String::new();
            String::new().append(&key)?;
            String::new().append(&String::from("="))?;
            String::new().append(&val)?;
            out.println(String::new())?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "enumerate", descriptor = "(Ljava/util/Map;)V", access = "private"))]
    pub fn enumerate(&self, h: Object) -> Result<()> {
        let this = self;
        this.defaults.get().enumerate(h)?;
        let _t0 = this.entrySet()?;
        let _t1 = _t0.iterator()?;
        let mut local_2: Object = _t1;
        loop {
            let _t0 = local_2.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_2.next()?;
            let mut e: Object = _t0;
            let _t1 = e.getKey()?;
            let mut key: Object = _t1;
            let _t2 = e.getValue()?;
            let _t3 = h.put(key, _t2)?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "enumerateStringProperties", descriptor = "(Ljava/util/Map;)V", access = "private"))]
    pub fn enumerateStringProperties(&self, h: Object) -> Result<()> {
        let this = self;
        this.defaults.get().enumerateStringProperties(h)?;
        let _t0 = this.entrySet()?;
        let _t1 = _t0.iterator()?;
        let mut local_2: Object = _t1;
        loop {
            let _t0 = local_2.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_2.next()?;
            let mut e: Object = _t0;
            let _t1 = e.getKey()?;
            let mut k: Object = _t1;
            let _t2 = e.getValue()?;
            let mut v: Object = _t2;
            let _t3 = h.put(k, v)?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "size", descriptor = "()I", access = "public"))]
    pub fn size(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.map.get().size()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "isEmpty", descriptor = "()Z", access = "public"))]
    pub fn isEmpty(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.map.get().isEmpty()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "keys", descriptor = "()Ljava/util/Enumeration;", access = "public"))]
    pub fn keys(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.map.get().keySet()?;
        let _t1: Object = Collections::enumeration(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "elements", descriptor = "()Ljava/util/Enumeration;", access = "public"))]
    pub fn elements(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.map.get().values()?;
        let _t1: Object = Collections::enumeration(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "contains", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn contains(&self, value: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.map.get().contains(value)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "containsValue", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn containsValue(&self, value: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.map.get().containsValue(value)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "containsKey", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn containsKey(&self, key: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.map.get().containsKey(key)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "get", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn get(&self, key: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.map.get().get(key)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "put", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn put(&self, key: Object, value: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.map.get().put(key, value)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    // java: remove(Ljava/lang/Object;)Ljava/lang/Object;
    pub fn remove__obj(&self, key: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.map.get().remove(key)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "putAll", descriptor = "(Ljava/util/Map;)V", access = "public"))]
    pub fn putAll(&self, t: Object) -> Result<()> {
        let this = self;
        this.map.get().putAll(t)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "clear", descriptor = "()V", access = "public"))]
    pub fn clear(&self) -> Result<()> {
        let this = self;
        this.map.get().clear()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let _t0 = this.map.get().toString()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "keySet", descriptor = "()Ljava/util/Set;", access = "public"))]
    pub fn keySet(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.map.get().keySet()?;
        let _t1: Object = Collections::synchronizedSet(_t0, this)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "values", descriptor = "()Ljava/util/Collection;", access = "public"))]
    pub fn values(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.map.get().values()?;
        let _t1: Object = Collections::synchronizedCollection(_t0, this)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "entrySet", descriptor = "()Ljava/util/Set;", access = "public"))]
    pub fn entrySet(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.map.get().entrySet()?;
        let _t1: Object = Collections::synchronizedSet(Properties_EntrySet::new(_t0)?, this)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn equals(&self, o: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.map.get().equals(o)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public"))]
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.map.get().hashCode()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getOrDefault", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn getOrDefault(&self, key: Object, defaultValue: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.map.get().getOrDefault(key, defaultValue)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "forEach", descriptor = "(Ljava/util/function/BiConsumer;)V", access = "public"))]
    pub fn forEach(&self, action: Object) -> Result<()> {
        let this = self;
        this.map.get().forEach(action)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "replaceAll", descriptor = "(Ljava/util/function/BiFunction;)V", access = "public"))]
    pub fn replaceAll(&self, function: Object) -> Result<()> {
        let this = self;
        this.map.get().replaceAll(function)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "putIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    pub fn putIfAbsent(&self, key: Object, value: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.map.get().putIfAbsent(key, value)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "remove", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public"))]
    // java: remove(Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn remove__obj_obj(&self, key: Object, value: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.map.get().remove(key, value)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z", access = "public"))]
    // java: replace(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z
    pub fn replace__obj_obj_obj(&self, key: Object, oldValue: Object, newValue: Object) -> Result<bool> {
        let this = self;
        let _t0 = this.map.get().replace(key, oldValue, newValue)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", access = "public"))]
    // java: replace(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
    pub fn replace__obj_obj(&self, key: Object, value: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.map.get().replace(key, value)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "computeIfAbsent", descriptor = "(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;", access = "public"))]
    pub fn computeIfAbsent(&self, key: Object, mappingFunction: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.map.get().computeIfAbsent(key, mappingFunction)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "computeIfPresent", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public"))]
    pub fn computeIfPresent(&self, key: Object, remappingFunction: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.map.get().computeIfPresent(key, remappingFunction)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "compute", descriptor = "(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public"))]
    pub fn compute(&self, key: Object, remappingFunction: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.map.get().compute(key, remappingFunction)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "merge", descriptor = "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", access = "public"))]
    pub fn merge(&self, key: Object, value: Object, remappingFunction: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.map.get().merge(key, value, remappingFunction)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "rehash", descriptor = "()V", access = "protected"))]
    pub fn rehash(&self) -> Result<()> {
        let this = self;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn clone(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.cloneHashtable()?;
        let mut clone: Object = _t0;
        clone.map.set(ConcurrentHashMap::new(this.map.get())?);
        Ok(clone)
    }

    #[cfg_attr(any(), java_method(name = "writeHashtable", descriptor = "(Ljava/io/ObjectOutputStream;)V"))]
    pub fn writeHashtable(&self, s: Object) -> Result<()> {
        let this = self;
        let mut map: Object = this.map.get();
        let _t0 = map.size()?;
        let mut entryStack: ArrayList<_> = ArrayList::<_>::new()?;
        let _t1 = map.entrySet()?;
        let _t2 = _t1.iterator()?;
        let mut loadFactor: Object = _t2;
        loop {
            let _t0 = loadFactor.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = loadFactor.next()?;
            let mut entry: Object = _t0;
            let _t1 = entry.getValue()?;
            let _t2 = entryStack.add(_t1)?;
            let _t3 = entry.getKey()?;
            let _t4 = entryStack.add(_t3)?;
        }
        loadFactor = 450i32;
        let _t3 = entryStack.size()?;
        entry = (_t3/2i32);
        let mut length: i32 = (((((entry as f32)/loadFactor) as i32)).wrapping_add((entry/20i32))).wrapping_add(3i32);
        length = length.wrapping_sub(1i32);
        let mut i: Object = map;
        /* TODO: monitorenter  */
        this.defaultWriteHashtable(s, length, loadFactor)?;
        /* TODO: monitorexit  */
        let mut local_8: Object = i;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        s.writeInt(length)?;
        s.writeInt(entry)?;
        let _t4 = entryStack.size()?;
        i = (_t4).wrapping_sub(1i32);
        loop {
            if i<0i32 { break; }
            let _t0 = entryStack.get(i)?;
            s.writeObject(_t0)?;
            i = i.wrapping_sub(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "readHashtable", descriptor = "(Ljava/io/ObjectInputStream;)V"))]
    pub fn readHashtable(&self, s: Object) -> Result<()> {
        let this = self;
        s.defaultReadObject()?;
        let _t0 = s.readInt()?;
        let mut origlength: i32 = _t0;
        let _t1 = s.readInt()?;
        let mut elements: i32 = _t1;
        String::new().append(&String::from("Illegal # of Elements:"))?;
        String::new().append(&elements)?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t2: Object = SharedSecrets::getJavaObjectInputStreamAccess()?;
        let _t3: i32 = HashMap::tableSizeFor((((elements as f64)/0.75f64) as i32))?;
        _t2.checkArray(s, 490i32, _t3)?;
        let mut map: ConcurrentHashMap = ConcurrentHashMap::new(elements)?;
        loop {
            if elements<=0i32 { break; }
            let _t0 = s.readObject()?;
            let mut key: Object = _t0;
            let _t1 = s.readObject()?;
            let mut value: Object = _t1;
            let _t2 = map.put(key, value)?;
            elements = elements.wrapping_sub(1i32);
        }
        this.map.set(map);
        Ok(())
    }
}
