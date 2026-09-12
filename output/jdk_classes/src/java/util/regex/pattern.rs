#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/regex/Pattern",
    super_class = "java/lang/Object",
    interfaces  = "java/io/Serializable",
    access      = "public final",
    source      = "Pattern.java",
))]
pub struct Pattern {
    #[cfg_attr(any(), java_field(name = "pattern", descriptor = "Ljava/lang/String;", access = "private"))]
    pub pattern: Field<String>,
    #[cfg_attr(any(), java_field(name = "flags", descriptor = "I", access = "private"))]
    pub flags: Field<i32>,
    #[cfg_attr(any(), java_field(name = "flags0", descriptor = "I", access = "private"))]
    pub flags0: Field<i32>,
    #[cfg_attr(any(), java_field(name = "compiled", descriptor = "Z", access = "private"))]
    pub compiled: Field<bool>,
    #[cfg_attr(any(), java_field(name = "normalizedPattern", descriptor = "Ljava/lang/String;", access = "private"))]
    pub normalizedPattern: Field<String>,
    #[cfg_attr(any(), java_field(name = "root", descriptor = "Ljava/util/regex/Pattern$Node;", access = ""))]
    pub root: Field<Object>,
    #[cfg_attr(any(), java_field(name = "matchRoot", descriptor = "Ljava/util/regex/Pattern$Node;", access = ""))]
    pub matchRoot: Field<Object>,
    #[cfg_attr(any(), java_field(name = "buffer", descriptor = "[I", access = ""))]
    pub buffer: Field<Vec<i32>>,
    #[cfg_attr(any(), java_field(name = "predicate", descriptor = "Ljava/util/regex/Pattern$CharPredicate;", access = ""))]
    pub predicate: Field<Object>,
    #[cfg_attr(any(), java_field(name = "namedGroups", descriptor = "Ljava/util/Map;", access = ""))]
    pub namedGroups: Field<Object>,
    #[cfg_attr(any(), java_field(name = "groupNodes", descriptor = "[Ljava/util/regex/Pattern$GroupHead;", access = ""))]
    pub groupNodes: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "topClosureNodes", descriptor = "Ljava/util/List;", access = ""))]
    pub topClosureNodes: Field<Object>,
    #[cfg_attr(any(), java_field(name = "localTCNCount", descriptor = "I", access = ""))]
    pub localTCNCount: Field<i32>,
    #[cfg_attr(any(), java_field(name = "hasGroupRef", descriptor = "Z", access = ""))]
    pub hasGroupRef: Field<bool>,
    #[cfg_attr(any(), java_field(name = "temp", descriptor = "[I", access = "private"))]
    pub temp: Field<Vec<i32>>,
    #[cfg_attr(any(), java_field(name = "capturingGroupCount", descriptor = "I", access = ""))]
    pub capturingGroupCount: Field<i32>,
    #[cfg_attr(any(), java_field(name = "localCount", descriptor = "I", access = ""))]
    pub localCount: Field<i32>,
    #[cfg_attr(any(), java_field(name = "cursor", descriptor = "I", access = "private"))]
    pub cursor: Field<i32>,
    #[cfg_attr(any(), java_field(name = "patternLength", descriptor = "I", access = "private"))]
    pub patternLength: Field<i32>,
    #[cfg_attr(any(), java_field(name = "hasSupplementary", descriptor = "Z", access = "private"))]
    pub hasSupplementary: Field<bool>,
}

impl Pattern {
    #[cfg_attr(any(), java_method(name = "compile", descriptor = "(Ljava/lang/String;)Ljava/util/regex/Pattern;", access = "public static"))]
    // java: compile(Ljava/lang/String;)Ljava/util/regex/Pattern;
    pub fn compile__str(regex: String) -> Result<Object> {
        Ok(Pattern::new(regex, 0i32)?)
    }

    #[cfg_attr(any(), java_method(name = "compile", descriptor = "(Ljava/lang/String;I)Ljava/util/regex/Pattern;", access = "public static"))]
    // java: compile(Ljava/lang/String;I)Ljava/util/regex/Pattern;
    pub fn compile__str_i(regex: String, flags: i32) -> Result<Object> {
        Ok(Pattern::new(regex, flags)?)
    }

    #[cfg_attr(any(), java_method(name = "pattern", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn pattern(&self) -> Result<String> {
        let this = self;
        Ok(this.pattern.get())
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        Ok(this.pattern.get())
    }

    #[cfg_attr(any(), java_method(name = "matcher", descriptor = "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;", access = "public"))]
    pub fn matcher(&self, input: Object) -> Result<Object> {
        let this = self;
        let mut m: java/util/regex/Pattern = this;
        /* TODO: monitorenter  */
        this.compile()?;
        /* TODO: monitorexit  */
        let mut local_3: java/util/regex/Pattern = m;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        m = Matcher::new(this, input)?;
        Ok(m)
    }

    #[cfg_attr(any(), java_method(name = "flags", descriptor = "()I", access = "public"))]
    pub fn flags(&self) -> Result<i32> {
        let this = self;
        Ok(this.flags0.get())
    }

    #[cfg_attr(any(), java_method(name = "matches", descriptor = "(Ljava/lang/String;Ljava/lang/CharSequence;)Z", access = "public static"))]
    pub fn matches(regex: String, input: Object) -> Result<bool> {
        let _t0: Object = Pattern::compile(regex)?;
        let mut p: Object = _t0;
        let _t1 = p.matcher(input)?;
        let mut m: Object = _t1;
        let _t2 = m.matches()?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "split", descriptor = "(Ljava/lang/CharSequence;I)[Ljava/lang/String;", access = "public"))]
    // java: split(Ljava/lang/CharSequence;I)[Ljava/lang/String;
    pub fn split__seq_i(&self, input: Object, limit: i32) -> Result<Vec<String>> {
        let this = self;
        let _t0 = this.split(input, limit, 0i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "splitWithDelimiters", descriptor = "(Ljava/lang/CharSequence;I)[Ljava/lang/String;", access = "public"))]
    pub fn splitWithDelimiters(&self, input: Object, limit: i32) -> Result<Vec<String>> {
        let this = self;
        let _t0 = this.split(input, limit, 1i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "split", descriptor = "(Ljava/lang/CharSequence;IZ)[Ljava/lang/String;", access = "private"))]
    // java: split(Ljava/lang/CharSequence;IZ)[Ljava/lang/String;
    pub fn split__seq_i_z(&self, input: Object, limit: i32, withDelimiters: bool) -> Result<Vec<String>> {
        let this = self;
        let mut matchCount: i32 = 0i32;
        let mut index: i32 = 0i32;
        let mut matchLimited: i32 = limit>0i32;
        let mut matchList: ArrayList<_> = ArrayList::<_>::new()?;
        let _t0 = this.matcher(input)?;
        let mut m: Object = _t0;
        loop {
            let _t0 = m.find()?;
            if _t0==0i32 { break; }
            let _t0 = m.start()?;
            let _t1 = m.start()?;
            let _t2 = m.end()?;
            let _t3 = m.start()?;
            let _t4 = input.subSequence(index, _t3)?;
            let _t5 = _t4.toString()?;
            let mut match_: String = _t5;
            let _t6 = matchList.add(match_)?;
            let _t7 = m.end()?;
            index = _t7;
            let _t8 = m.start()?;
            let _t9 = input.subSequence(_t8, index)?;
            let _t10 = _t9.toString()?;
            let _t11 = matchList.add(_t10)?;
            matchCount = matchCount.wrapping_add(1i32);
            let _t12 = input.length()?;
            let _t13 = input.subSequence(index, _t12)?;
            let _t14 = _t13.toString()?;
            match_ = _t14;
            let _t15 = matchList.add(match_)?;
            let _t16 = m.end()?;
            index = _t16;
            matchCount = matchCount.wrapping_add(1i32);
        }
        let mut _arr1: Vec<Object> = Vec::with_capacity(1i32 as usize);
        let _t2 = input.toString()?;
        _arr1[0i32 as usize] = _t2;
        return Ok(_arr1);
        let _t3 = input.length()?;
        let _t4 = input.subSequence(index, _t3)?;
        let _t5 = _t4.toString()?;
        let _t6 = matchList.add(_t5)?;
        let _t7 = matchList.size()?;
        match_ = _t7;
        loop {
            if match_<=0i32 { break; }
            let _t0 = matchList.get((match_).wrapping_sub(1i32))?;
            let _t1 = _t0.isEmpty()?;
            match_ = match_.wrapping_sub(1i32);
        }
        let mut _arr8: Vec<Object> = Vec::with_capacity(match_ as usize);
        let mut result: Vec<Object> = _arr8;
        let _t9 = matchList.subList(0i32, match_)?;
        let _t10 = _t9.toArray(result)?;
        Ok(_t10)
    }

    #[cfg_attr(any(), java_method(name = "split", descriptor = "(Ljava/lang/CharSequence;)[Ljava/lang/String;", access = "public"))]
    // java: split(Ljava/lang/CharSequence;)[Ljava/lang/String;
    pub fn split__seq(&self, input: Object) -> Result<Vec<String>> {
        let this = self;
        let _t0 = this.split(input, 0i32, 0i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "quote", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public static"))]
    pub fn quote(s: String) -> Result<String> {
        let _t0 = s.indexOf(String::from("\E"))?;
        let mut slashEIndex: i32 = _t0;
        String::new().append(&String::from("\Q"))?;
        String::new().append(&s)?;
        String::new().append(&String::from("\E"))?;
        return Ok(String::new());
        let _t1 = s.length()?;
        let mut lenHint: i32 = _t1;
        lenHint = 2147483639i32;
        let mut sb: String = String::new();
        sb.append(&String::from("\Q"))?;
        let mut current: i32 = 0i32;
        sb.append(&s)?;
        sb.append(&String::from("\E\\E\Q"))?;
        current = (slashEIndex).wrapping_add(2i32);
        let _t2 = s.indexOf(String::from("\E"), current)?;
        slashEIndex = _t2;
        let _t3 = s.length()?;
        sb.append(&s)?;
        sb.append(&String::from("\E"))?;
        Ok(sb)
    }

    #[cfg_attr(any(), java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private"))]
    pub fn readObject(&self, s: Object) -> Result<()> {
        let this = self;
        s.defaultReadObject()?;
        this.flags0.set(this.flags.get());
        this.capturingGroupCount.set(1i32);
        this.localCount.set(0i32);
        this.localTCNCount.set(0i32);
        let _t0 = this.pattern.get().isEmpty()?;
        this.root.set(Pattern_Start::new(Pattern::lastAccept())?);
        this.matchRoot.set(Pattern::lastAccept());
        this.compiled.set(1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;I)V", access = "private"))]
    pub fn new(p: String, f: i32) -> Result<Self> {
        let this = Self { pattern: Field::new(String::new()), flags: Field::new(0), flags0: Field::new(0), compiled: Field::new(false), normalizedPattern: Field::new(String::new()), root: Field::new(Default::default()), matchRoot: Field::new(Default::default()), buffer: Field::new(Default::default()), predicate: Field::new(Default::default()), namedGroups: Field::new(Default::default()), groupNodes: Field::new(Default::default()), topClosureNodes: Field::new(Default::default()), localTCNCount: Field::new(0), hasGroupRef: Field::new(false), temp: Field::new(Default::default()), capturingGroupCount: Field::new(0), localCount: Field::new(0), cursor: Field::new(0), patternLength: Field::new(0), hasSupplementary: Field::new(false) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        String::new().append(&String::from("Unknown flag 0x"))?;
        let _t0: String = Integer::toHexString(f)?;
        String::new().append(&_t0)?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.pattern.set(p);
        this.flags.set(f);
        this.flags.set((this.flags.get()|64i32));
        this.flags0.set(this.flags.get());
        this.capturingGroupCount.set(1i32);
        this.localCount.set(0i32);
        this.localTCNCount.set(0i32);
        let _t1 = this.pattern.get().isEmpty()?;
        this.compile()?;
        let mut soe: bool = _t1;
        let _t2 = this.error(String::from("Stack overflow during pattern compilation"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.root.set(Pattern_Start::new(Pattern::lastAccept())?);
        this.matchRoot.set(Pattern::lastAccept());
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "normalize", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "private static"))]
    pub fn normalize(pattern: String) -> Result<String> {
        let _t0 = pattern.length()?;
        let mut plen: i32 = _t0;
        let mut pbuf: String = String::new();
        let mut last: i32 = 0i32;
        let mut lastStart: i32 = 0i32;
        let mut cc: i32 = 0i32;
        let mut i: i32 = 0i32;
        loop {
            if i >= plen { break; }
            let _t0 = pattern.charAt(i)?;
            let mut c: i32 = _t0;
            let _t1 = pattern.charAt((i).wrapping_add(1i32))?;
            i = i.wrapping_add(2i32);
            last = 0i32;
            Pattern::normalizeSlice(pattern, lastStart, i, pbuf)?;
            lastStart = i;
            /* TODO: i2c  */
            cc = (cc).wrapping_add(1i32);
            /* TODO: i2c  */
            cc = (cc).wrapping_sub(1i32);
            Pattern::normalizeClazz(pattern, lastStart, (i).wrapping_add(1i32), pbuf)?;
            lastStart = (i).wrapping_add(1i32);
            last = c;
            i = i.wrapping_add(1i32);
        }
        return Err(JvmError::Custom(String::from("athrow")));
        Pattern::normalizeSlice(pattern, lastStart, plen, pbuf)?;
        Ok(pbuf)
    }

    #[cfg_attr(any(), java_method(name = "normalizeSlice", descriptor = "(Ljava/lang/String;IILjava/lang/StringBuilder;)V", access = "private static"))]
    pub fn normalizeSlice(src: String, off: i32, limit: i32, dst: Object) -> Result<()> {
        let _t0 = src.length()?;
        let mut len: i32 = _t0;
        let mut off0: i32 = off;
        loop {
            if off >= limit { break; }
            let _t0 = src.charAt(off)?;
            let _t1: bool = ASCII::isAscii(_t0)?;
            off = off.wrapping_add(1i32);
        }
        dst.append(&src)?;
        return Ok(());
        off = off.wrapping_sub(1i32);
        off = off0;
        dst.append(&src)?;
        loop {
            if off >= limit { break; }
            let _t0 = src.codePointAt(off)?;
            let mut ch0: i32 = _t0;
            let _t1 = String::from(".$|()[]{}^?*+\").indexOf(ch0)?;
            /* TODO: i2c  */
            dst.append(&ch0)?;
            off = off.wrapping_add(1i32);
            let _t2: i32 = Grapheme::nextBoundary(src, off, limit)?;
            let mut j: i32 = _t2;
            let _t3 = src.substring(off, j)?;
            let mut seq: String = _t3;
            let _t4: String = Normalizer::normalize(seq, Normalizer$Form::NFD())?;
            let mut nfd: String = _t4;
            off = j;
            let _t5 = nfd.length()?;
            let _t6 = nfd.codePointCount(0i32, _t5)?;
            let _t7 = nfd.codePointAt(0i32)?;
            ch0 = _t7;
            let _t8: i32 = Character::charCount(ch0)?;
            let _t9 = nfd.codePointAt(_t8)?;
            let mut ch1: i32 = _t9;
            let _t10: i32 = Character::getType(ch1)?;
            let mut altns: LinkedHashSet = LinkedHashSet::new()?;
            let _t11 = altns.add(seq)?;
            Pattern::produceEquivalentAlternation(nfd, altns)?;
            dst.append(&String::from("(?:"))?;
            /* TODO: invokedynamic 263 */
            altns.forEach(dst)?;
            let _t12 = dst.length()?;
            let _t13 = dst.length()?;
            let _t14 = dst.delete((_t12).wrapping_sub(1i32), _t13)?;
            dst.append(&String::from(")"))?;
            let _t15: String = Normalizer::normalize(seq, Normalizer$Form::NFC())?;
            altns = _t15;
            let _t16 = seq.equals(altns)?;
            let _t17 = nfd.equals(altns)?;
            String::new().append(&String::from("(?:"))?;
            String::new().append(&seq)?;
            String::new().append(&String::from("|"))?;
            String::new().append(&nfd)?;
            String::new().append(&String::from("|"))?;
            String::new().append(&altns)?;
            String::new().append(&String::from(")"))?;
            dst.append(&String::new())?;
            let _t18 = seq.equals(nfd)?;
            String::new().append(&String::from("(?:"))?;
            String::new().append(&seq)?;
            String::new().append(&String::from("|"))?;
            String::new().append(&nfd)?;
            String::new().append(&String::from(")"))?;
            dst.append(&String::new())?;
            dst.append(&seq)?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "normalizeClazz", descriptor = "(Ljava/lang/String;IILjava/lang/StringBuilder;)V", access = "private static"))]
    pub fn normalizeClazz(src: String, off: i32, limit: i32, dst: Object) -> Result<()> {
        let _t0 = src.substring(off, limit)?;
        let _t1: String = Normalizer::normalize(_t0, Normalizer$Form::NFC())?;
        dst.append(&_t1)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "produceEquivalentAlternation", descriptor = "(Ljava/lang/String;Ljava/util/Set;)V", access = "private static"))]
    pub fn produceEquivalentAlternation(src: String, dst: Object) -> Result<()> {
        let _t0: i32 = Pattern::countChars(src, 0i32, 1i32)?;
        let mut len: i32 = _t0;
        let _t1 = src.length()?;
        let _t2 = dst.add(src)?;
        return Ok(());
        let _t3 = src.substring(0i32, len)?;
        let mut base: String = _t3;
        let _t4 = src.substring(len)?;
        let mut combiningMarks: String = _t4;
        let _t5: Vec<String> = Pattern::producePermutations(combiningMarks)?;
        let mut perms: Vec<String> = _t5;
        let mut x: i32 = 0i32;
        loop {
            if x >= (perms.len() as i32) { break; }
            String::new().append(&base)?;
            String::new().append(&perms[x as usize].clone())?;
            let mut next: String = String::new();
            let _t0 = dst.add(next)?;
            let _t1: String = Pattern::composeOneStep(next)?;
            next = _t1;
            Pattern::produceEquivalentAlternation(next, dst)?;
            x = x.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "producePermutations", descriptor = "(Ljava/lang/String;)[Ljava/lang/String;", access = "private static"))]
    pub fn producePermutations(input: String) -> Result<Vec<String>> {
        let _t0 = input.length()?;
        let _t1: i32 = Pattern::countChars(input, 0i32, 1i32)?;
        let mut _arr2: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr2[0i32 as usize] = input;
        return Ok(_arr2);
        let _t3 = input.length()?;
        let _t4: i32 = Pattern::countChars(input, 0i32, 2i32)?;
        let _t5: i32 = Character::codePointAt(input, 0i32)?;
        let mut c0: i32 = _t5;
        let _t6: i32 = Character::charCount(c0)?;
        let _t7: i32 = Character::codePointAt(input, _t6)?;
        let mut c1: i32 = _t7;
        let _t8: i32 = Pattern::getClass(c1)?;
        let _t9: i32 = Pattern::getClass(c0)?;
        let mut _arr10: Vec<Object> = Vec::with_capacity(1i32 as usize);
        _arr10[0i32 as usize] = input;
        return Ok(_arr10);
        let mut _arr11: Vec<Object> = Vec::with_capacity(2i32 as usize);
        let mut result: Vec<Object> = _arr11;
        result[0i32 as usize] = input;
        let mut sb: String = String::new();
        let _t12 = sb.appendCodePoint(c1)?;
        let _t13 = sb.appendCodePoint(c0)?;
        result[1i32 as usize] = sb;
        return Ok(result);
        let _t14: i32 = Pattern::countCodePoints(input)?;
        c0 = _t14;
        return Err(JvmError::Custom(String::from("athrow")));
        c1 = 1i32;
        result = 2i32;
        loop {
            if result > c0 { break; }
            c1 = (c1).wrapping_mul(result);
            result = result.wrapping_add(1i32);
        }
        let mut _arr15: Vec<Object> = Vec::with_capacity(c1 as usize);
        result = _arr15;
        let mut _arr16: Vec<i32> = vec![0i32; c0 as usize];
        sb = _arr16;
        let mut x: i32 = 0i32;
        let mut i: i32 = 0i32;
        loop {
            if x >= c0 { break; }
            let _t0: i32 = Character::codePointAt(input, i)?;
            let mut c: i32 = _t0;
            let _t1: i32 = Pattern::getClass(c)?;
            sb[x as usize] = _t1;
            let _t2: i32 = Character::charCount(c)?;
            i = (i).wrapping_add(_t2);
            x = x.wrapping_add(1i32);
        }
        x = 0i32;
        c = 0i32;
        let mut offset: i32 = 0i32;
        loop {
            if c >= c0 { break; }
            let _t0: i32 = Pattern::countChars(input, offset, 1i32)?;
            i = _t0;
            let mut y: i32 = (c).wrapping_sub(1i32);
            y = y.wrapping_sub(1i32);
            y = String::new();
            let _t1 = y.delete(offset, (offset).wrapping_add(i))?;
            let mut otherChars: String = _t1;
            let _t2: Vec<String> = Pattern::producePermutations(otherChars)?;
            let mut subResult: Vec<String> = _t2;
            let _t3 = input.substring(offset, (offset).wrapping_add(i))?;
            let mut prefix: String = _t3;
            let mut local_13: Vec<String> = subResult;
            let mut local_14: i32 = (local_13.len() as i32);
            let mut local_15: i32 = 0i32;
            let mut sre: String = local_13[local_15 as usize].clone();
            x = x.wrapping_add(1i32);
            String::new().append(&prefix)?;
            String::new().append(&sre)?;
            result[x as usize] = String::new();
            local_15 = local_15.wrapping_add(1i32);
            c = c.wrapping_add(1i32);
            offset = (offset).wrapping_add(i);
        }
        let mut _arr17: Vec<Object> = Vec::with_capacity(x as usize);
        c = _arr17;
        System::arraycopy(&result, 0i32, c, 0i32, x)?;
        Ok(c)
    }

    #[cfg_attr(any(), java_method(name = "getClass", descriptor = "(I)I", access = "private static"))]
    pub fn getClass(c: i32) -> Result<i32> {
        let _t0: i32 = Normalizer::getCombiningClass(c)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "composeOneStep", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "private static"))]
    pub fn composeOneStep(input: String) -> Result<String> {
        let _t0: i32 = Pattern::countChars(input, 0i32, 2i32)?;
        let mut len: i32 = _t0;
        let _t1 = input.substring(0i32, len)?;
        let mut firstTwoCharacters: String = _t1;
        let _t2: String = Normalizer::normalize(firstTwoCharacters, Normalizer$Form::NFC())?;
        let mut result: String = _t2;
        let _t3 = result.equals(firstTwoCharacters)?;
        /* TODO: aconst_null  */
        return Ok(_t3);
        let _t4 = input.substring(len)?;
        let mut remainder: String = _t4;
        String::new().append(&result)?;
        String::new().append(&remainder)?;
        Ok(String::new())
    }

    #[cfg_attr(any(), java_method(name = "RemoveQEQuoting", descriptor = "()V", access = "private"))]
    pub fn RemoveQEQuoting(&self) -> Result<()> {
        let this = self;
        let mut pLen: i32 = this.patternLength.get();
        let mut i: i32 = 0i32;
        loop {
            if i >= (pLen).wrapping_sub(1i32) { break; }
            i = i.wrapping_add(1i32);
            i = i.wrapping_add(2i32);
        }
        return Ok(());
        let mut j: i32 = i;
        i = i.wrapping_add(2i32);
        let _t0: i32 = (3i32).abs();
        let _t1: i32 = ((j).wrapping_add(2i32)).abs();
        let mut newTempLen: i32 = _t1;
        let mut ae: i32 = (pLen).wrapping_sub(1i32);
        return Err(JvmError::Custom(String::from("athrow")));
        let mut _arr2: Vec<i32> = vec![0i32; newTempLen as usize];
        ae = _arr2;
        System::arraycopy(&this.temp.get(), 0i32, ae, 0i32, j)?;
        let mut inQuote: i32 = 1i32;
        let mut beginQuote: i32 = 1i32;
        loop {
            if i >= pLen { break; }
            i = i.wrapping_add(1i32);
            let mut c: i32 = this.temp.get()[i as usize];
            let _t0: bool = ASCII::isAscii(c)?;
            let _t1: bool = ASCII::isAlpha(c)?;
            j = j.wrapping_add(1i32);
            ae[j as usize] = c;
            let _t2: bool = ASCII::isDigit(c)?;
            j = j.wrapping_add(1i32);
            ae[j as usize] = 92i32;
            j = j.wrapping_add(1i32);
            ae[j as usize] = 120i32;
            j = j.wrapping_add(1i32);
            ae[j as usize] = 51i32;
            j = j.wrapping_add(1i32);
            ae[j as usize] = c;
            j = j.wrapping_add(1i32);
            ae[j as usize] = 92i32;
            j = j.wrapping_add(1i32);
            ae[j as usize] = c;
            i = i.wrapping_add(1i32);
            inQuote = 0i32;
            j = j.wrapping_add(1i32);
            ae[j as usize] = 92i32;
            j = j.wrapping_add(1i32);
            ae[j as usize] = 92i32;
            i = i.wrapping_add(1i32);
            inQuote = 1i32;
            beginQuote = 1i32;
            j = j.wrapping_add(1i32);
            ae[j as usize] = c;
            j = j.wrapping_add(1i32);
            i = i.wrapping_add(1i32);
            ae[j as usize] = this.temp.get()[i as usize];
            beginQuote = 0i32;
        }
        this.patternLength.set(j);
        let _t3: Vec<i32> = Arrays::copyOf(ae, (j).wrapping_add(2i32))?;
        this.temp.set(_t3);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "compile", descriptor = "()V", access = "private"))]
    // java: compile()V
    pub fn compile(&self) -> Result<()> {
        let this = self;
        let _t0 = this.has(128i32)?;
        let _t1 = this.has(16i32)?;
        let _t2: String = Pattern::normalize(this.pattern.get())?;
        this.normalizedPattern.set(_t2);
        this.normalizedPattern.set(this.pattern.get());
        let _t3 = this.normalizedPattern.get().length()?;
        this.patternLength.set(_t3);
        let mut _arr4: Vec<i32> = vec![0i32; (this.patternLength.get()).wrapping_add(2i32) as usize];
        this.temp.set(_arr4);
        this.hasSupplementary.set(0i32);
        let mut count: i32 = 0i32;
        let mut x: i32 = 0i32;
        loop {
            if x >= this.patternLength.get() { break; }
            let _t0 = this.normalizedPattern.get().codePointAt(x)?;
            let mut c: i32 = _t0;
            let _t1: bool = Pattern::isSupplementary(c)?;
            this.hasSupplementary.set(1i32);
            count = count.wrapping_add(1i32);
            this.temp.get()[count as usize] = c;
            let _t2: i32 = Character::charCount(c)?;
            x = (x).wrapping_add(_t2);
        }
        this.patternLength.set(count);
        let _t5 = this.has(16i32)?;
        this.RemoveQEQuoting()?;
        let mut _arr6: Vec<i32> = vec![0i32; 32i32 as usize];
        this.buffer.set(_arr6);
        let mut _arr7: Vec<Object> = Vec::with_capacity(10i32 as usize);
        this.groupNodes.set(_arr7);
        /* TODO: aconst_null  */
        _t5.namedGroups.set(this);
        this.topClosureNodes.set(ArrayList::<_>::new()?);
        let _t8 = this.has(16i32)?;
        let _t9 = this.newSlice(this.temp.get(), this.patternLength.get(), this.hasSupplementary.get())?;
        this.matchRoot.set(_t9);
        this.matchRoot.get().next.set(Pattern::lastAccept());
        let _t10 = this.expr(Pattern::lastAccept())?;
        this.matchRoot.set(_t10);
        let _t11 = this.peek()?;
        let _t12 = this.error(String::from("Unmatched closing ')'"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t13 = this.error(String::from("Unescaped trailing backslash"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t14 = this.error(String::from("Unexpected internal error"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t15: Object = Pattern$BnM::optimize(this.matchRoot.get())?;
        this.root.set(_t15);
        Pattern_StartS::new(this.matchRoot.get())?.root.set(Pattern_Start::new(this.matchRoot.get())?);
        this.root.set(this.matchRoot.get());
        Pattern_StartS::new(this.matchRoot.get())?.root.set(Pattern_Start::new(this.matchRoot.get())?);
        let _t16 = this.topClosureNodes.get().iterator()?;
        x = _t16;
        loop {
            let _t0 = x.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = x.next()?;
            let mut node: Object = _t0;
            this.localTCNCount.set((this.localTCNCount.get()).wrapping_add(1i32));
            node.posIndex.set(this.localTCNCount.get());
        }
        /* TODO: aconst_null  */
        this.hasGroupRef.get().temp.set(this);
        /* TODO: aconst_null  */
        this.hasSupplementary.get().buffer.set(this);
        /* TODO: aconst_null  */
        this.groupNodes.set(this);
        this.patternLength.set(0i32);
        this.compiled.set(1i32);
        /* TODO: aconst_null  */
        true.topClosureNodes.set(this);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "namedGroupsMap", descriptor = "()Ljava/util/Map;", access = "private"))]
    pub fn namedGroupsMap(&self) -> Result<Object> {
        let this = self;
        let mut groups: Object = this.namedGroups.get();
        groups = HashMap::<_, _>::new()?;
        this.namedGroups.set(HashMap::<_, _>::new()?);
        Ok(groups)
    }

    #[cfg_attr(any(), java_method(name = "namedGroups", descriptor = "()Ljava/util/Map;", access = "public"))]
    pub fn namedGroups(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.namedGroupsMap()?;
        let _t1: Object = Map::copyOf(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "has", descriptor = "(I)Z", access = "private"))]
    pub fn has(&self, f: i32) -> Result<bool> {
        let this = self;
        Ok((this.flags0.get()&f)!=0i32)
    }

    #[cfg_attr(any(), java_method(name = "accept", descriptor = "(ILjava/lang/String;)V", access = "private"))]
    pub fn accept(&self, ch: i32, s: String) -> Result<()> {
        let this = self;
        this.cursor.set((this.cursor.get()).wrapping_add(1i32));
        let mut testChar: i32 = this.temp.get()[this.cursor.get() as usize];
        let _t0 = this.has(4i32)?;
        let _t1 = this.parsePastWhitespace(testChar)?;
        testChar = _t1;
        let _t2 = this.error(s)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "mark", descriptor = "(I)V", access = "private"))]
    pub fn mark(&self, c: i32) -> Result<()> {
        let this = self;
        this.temp.get()[this.patternLength.get() as usize] = c;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "peek", descriptor = "()I", access = "private"))]
    pub fn peek(&self) -> Result<i32> {
        let this = self;
        let mut ch: i32 = this.temp.get()[this.cursor.get() as usize];
        let _t0 = this.has(4i32)?;
        let _t1 = this.peekPastWhitespace(ch)?;
        ch = _t1;
        Ok(ch)
    }

    #[cfg_attr(any(), java_method(name = "read", descriptor = "()I", access = "private"))]
    pub fn read(&self) -> Result<i32> {
        let this = self;
        this.cursor.set((this.cursor.get()).wrapping_add(1i32));
        let mut ch: i32 = this.temp.get()[this.cursor.get() as usize];
        let _t0 = this.has(4i32)?;
        let _t1 = this.parsePastWhitespace(ch)?;
        ch = _t1;
        Ok(ch)
    }

    #[cfg_attr(any(), java_method(name = "readEscaped", descriptor = "()I", access = "private"))]
    pub fn readEscaped(&self) -> Result<i32> {
        let this = self;
        this.cursor.set((this.cursor.get()).wrapping_add(1i32));
        let mut ch: i32 = this.temp.get()[this.cursor.get() as usize];
        Ok(ch)
    }

    #[cfg_attr(any(), java_method(name = "next", descriptor = "()I", access = "private"))]
    pub fn next(&self) -> Result<i32> {
        let this = self;
        this.cursor.set((this.cursor.get()).wrapping_add(1i32));
        let mut ch: i32 = this.temp.get()[(this.cursor.get()).wrapping_add(1i32) as usize];
        let _t0 = this.has(4i32)?;
        let _t1 = this.peekPastWhitespace(ch)?;
        ch = _t1;
        Ok(ch)
    }

    #[cfg_attr(any(), java_method(name = "nextEscaped", descriptor = "()I", access = "private"))]
    pub fn nextEscaped(&self) -> Result<i32> {
        let this = self;
        this.cursor.set((this.cursor.get()).wrapping_add(1i32));
        let mut ch: i32 = this.temp.get()[(this.cursor.get()).wrapping_add(1i32) as usize];
        Ok(ch)
    }

    #[cfg_attr(any(), java_method(name = "peekPastWhitespace", descriptor = "(I)I", access = "private"))]
    pub fn peekPastWhitespace(&self, ch: i32) -> Result<i32> {
        let this = self;
        loop {
            let _t0: bool = ASCII::isSpace(ch)?;
            if ch != 35i32 { break; }
            let _t0: bool = ASCII::isSpace(ch)?;
            this.cursor.set((this.cursor.get()).wrapping_add(1i32));
            ch = this.temp.get()[(this.cursor.get()).wrapping_add(1i32) as usize];
            let _t1 = this.peekPastLine()?;
            ch = _t1;
        }
        Ok(ch)
    }

    #[cfg_attr(any(), java_method(name = "parsePastWhitespace", descriptor = "(I)I", access = "private"))]
    pub fn parsePastWhitespace(&self, ch: i32) -> Result<i32> {
        let this = self;
        loop {
            let _t0: bool = ASCII::isSpace(ch)?;
            if ch != 35i32 { break; }
            let _t0: bool = ASCII::isSpace(ch)?;
            this.cursor.set((this.cursor.get()).wrapping_add(1i32));
            ch = this.temp.get()[this.cursor.get() as usize];
            let _t1 = this.parsePastLine()?;
            ch = _t1;
        }
        Ok(ch)
    }

    #[cfg_attr(any(), java_method(name = "parsePastLine", descriptor = "()I", access = "private"))]
    pub fn parsePastLine(&self) -> Result<i32> {
        let this = self;
        this.cursor.set((this.cursor.get()).wrapping_add(1i32));
        let mut ch: i32 = this.temp.get()[this.cursor.get() as usize];
        loop {
            if ch==0i32 { break; }
            let _t0 = this.isLineSeparator(ch)?;
            this.cursor.set((this.cursor.get()).wrapping_add(1i32));
            ch = this.temp.get()[this.cursor.get() as usize];
        }
        this.cursor.set(this.patternLength.get());
        this.cursor.set((this.cursor.get()).wrapping_add(1i32));
        ch = this.temp.get()[this.cursor.get() as usize];
        Ok(ch)
    }

    #[cfg_attr(any(), java_method(name = "peekPastLine", descriptor = "()I", access = "private"))]
    pub fn peekPastLine(&self) -> Result<i32> {
        let this = self;
        this.cursor.set((this.cursor.get()).wrapping_add(1i32));
        let mut ch: i32 = this.temp.get()[(this.cursor.get()).wrapping_add(1i32) as usize];
        loop {
            if ch==0i32 { break; }
            let _t0 = this.isLineSeparator(ch)?;
            this.cursor.set((this.cursor.get()).wrapping_add(1i32));
            ch = this.temp.get()[(this.cursor.get()).wrapping_add(1i32) as usize];
        }
        this.cursor.set(this.patternLength.get());
        ch = this.temp.get()[this.cursor.get() as usize];
        Ok(ch)
    }

    #[cfg_attr(any(), java_method(name = "isLineSeparator", descriptor = "(I)Z", access = "private"))]
    pub fn isLineSeparator(&self, ch: i32) -> Result<bool> {
        let this = self;
        let _t0 = this.has(1i32)?;
        return Ok(ch == 10i32);
        Ok(ch == 133i32)
    }

    #[cfg_attr(any(), java_method(name = "skip", descriptor = "()I", access = "private"))]
    pub fn skip(&self) -> Result<i32> {
        let this = self;
        let mut i: i32 = this.cursor.get();
        let mut ch: i32 = this.temp.get()[(i).wrapping_add(1i32) as usize];
        this.cursor.set((i).wrapping_add(2i32));
        Ok(ch)
    }

    #[cfg_attr(any(), java_method(name = "unread", descriptor = "()V", access = "private"))]
    pub fn unread(&self) -> Result<()> {
        let this = self;
        this.cursor.set((this.cursor.get()).wrapping_sub(1i32));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "error", descriptor = "(Ljava/lang/String;)Ljava/util/regex/PatternSyntaxException;", access = "private"))]
    pub fn error(&self, s: String) -> Result<Object> {
        let this = self;
        Ok(PatternSyntaxException::new(s, this.normalizedPattern.get(), (this.cursor.get()).wrapping_sub(1i32))?)
    }

    #[cfg_attr(any(), java_method(name = "findSupplementary", descriptor = "(II)Z", access = "private"))]
    pub fn findSupplementary(&self, start: i32, end: i32) -> Result<bool> {
        let this = self;
        let mut i: i32 = start;
        loop {
            if i >= end { break; }
            let _t0: bool = Pattern::isSupplementary(this.temp.get()[i as usize])?;
            return Ok(1i32);
            i = i.wrapping_add(1i32);
        }
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "isSupplementary", descriptor = "(I)Z", access = "private static final"))]
    pub fn isSupplementary(ch: i32) -> Result<bool> {
        /* TODO: i2c  */
        let _t0: bool = Character::isSurrogate(ch)?;
        Ok(_t0!=0i32)
    }

    #[cfg_attr(any(), java_method(name = "expr", descriptor = "(Ljava/util/regex/Pattern$Node;)Ljava/util/regex/Pattern$Node;", access = "private"))]
    pub fn expr(&self, end: Object) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        let mut prev: i32 = todo!("stack underflow");
        /* TODO: aconst_null  */
        let mut firstTail: i32 = todo!("stack underflow");
        /* TODO: aconst_null  */
        let mut branch: i32 = todo!("stack underflow");
        /* TODO: aconst_null  */
        let mut branchConn: i32 = todo!("stack underflow");
        let _t0 = this.sequence(end)?;
        let mut node: Object = _t0;
        let mut nodeTail: Object = this.root.get();
        prev = node;
        firstTail = nodeTail;
        branchConn = Pattern_BranchConn::new()?;
        branchConn.next.set(end);
        /* TODO: aconst_null  */
        node = end;
        nodeTail.next.set(branchConn);
        branch.add(node)?;
        /* TODO: aconst_null  */
        prev = end;
        firstTail.next.set(branchConn);
        branch = Pattern_Branch::new(prev, node, branchConn)?;
        prev = Pattern_Branch::new(prev, node, branchConn)?;
        let _t1 = this.peek()?;
        return Ok(prev);
        let _t2 = this.next()?;
    }

    #[cfg_attr(any(), java_method(name = "sequence", descriptor = "(Ljava/util/regex/Pattern$Node;)Ljava/util/regex/Pattern$Node;", access = "private"))]
    pub fn sequence(&self, end: Object) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        let mut head: i32 = todo!("stack underflow");
        /* TODO: aconst_null  */
        let mut tail: i32 = todo!("stack underflow");
        loop {
            let _t0 = this.peek()?;
            let mut ch: i32 = _t0;
            /* TODO: lookupswitch default:591 0:577 36:411 40:136 41:527 42:539 43:539 46:460 63:539 91:174 92:224 93:530 94:353 124:527 125:530 */
            let _t1 = this.group0()?;
            let mut node: Object = _t1;
            if !node.is_none() { break; }
        }
        head = node;
        tail.next.set(node);
        tail = this.root.get();
        let _t0 = this.has(128i32)?;
        let _t1 = this.has(16i32)?;
        let _t2 = this.clazz(1i32)?;
        node = Pattern_NFCCharProperty::new(_t2)?;
        let _t3 = this.clazz(1i32)?;
        let _t4 = this.newCharProperty(_t3)?;
        node = _t4;
        let _t5 = this.nextEscaped()?;
        ch = _t5;
        let mut oneLetter: i32 = 1i32;
        let mut comp: i32 = ch == 80i32;
        let _t6 = this.next()?;
        ch = _t6;
        this.unread()?;
        oneLetter = 0i32;
        let _t7 = this.has(128i32)?;
        let _t8 = this.has(16i32)?;
        let _t9 = this.family(oneLetter, comp)?;
        node = Pattern_NFCCharProperty::new(_t9)?;
        let _t10 = this.family(oneLetter, comp)?;
        let _t11 = this.newCharProperty(_t10)?;
        node = _t11;
        this.unread()?;
        let _t12 = this.atom()?;
        node = _t12;
        let _t13 = this.next()?;
        let _t14 = this.has(8i32)?;
        let _t15 = this.has(1i32)?;
        node = Pattern_UnixCaret::new()?;
        node = Pattern_Caret::new()?;
        node = Pattern_Begin::new()?;
        let _t16 = this.next()?;
        let _t17 = this.has(1i32)?;
        let _t18 = this.has(8i32)?;
        node = Pattern_UnixDollar::new(_t18)?;
        let _t19 = this.has(8i32)?;
        node = Pattern_Dollar::new(_t19)?;
        let _t20 = this.next()?;
        let _t21 = this.has(32i32)?;
        let _t22: Object = Pattern::ALL()?;
        node = Pattern_CharProperty::new(_t22)?;
        let _t23 = this.has(1i32)?;
        let _t24: Object = Pattern::UNIXDOT()?;
        node = Pattern_CharProperty::new(_t24)?;
        let _t25: Object = Pattern::DOT()?;
        node = Pattern_CharProperty::new(_t25)?;
        let _t26 = this.atom()?;
        node = _t26;
        let _t27 = this.next()?;
        String::new().append(&String::from("Dangling meta character '"))?;
        /* TODO: i2c  */
        String::new().append(&ch)?;
        String::new().append(&String::from("'"))?;
        let _t28 = this.error(String::new())?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t29 = this.atom()?;
        node = _t29;
        let _t30 = this.closure(node)?;
        node = _t30;
        tail = node;
        head = node;
        tail.next.set(node);
        tail = node;
        return Ok(end);
        tail.next.set(end);
        this.root.set(tail);
        Ok(head)
    }

    #[cfg_attr(any(), java_method(name = "atom", descriptor = "()Ljava/util/regex/Pattern$Node;", access = "private"))]
    pub fn atom(&self) -> Result<Object> {
        let this = self;
        let mut first: i32 = 0i32;
        let mut prev: i32 = -1i32;
        let mut hasSupplementary: i32 = 0i32;
        let _t0 = this.peek()?;
        let mut ch: i32 = _t0;
        loop {
            /* TODO: lookupswitch default:357 0:343 36:144 40:144 41:144 42:128 43:128 46:144 63:128 91:144 92:147 94:144 123:128 124:144 */
            if first <= 1i32 { break; }
            this.cursor.set(prev);
            first = first.wrapping_sub(1i32);
            let _t0 = this.nextEscaped()?;
            ch = _t0;
            this.unread()?;
            let mut comp: i32 = 0i32;
            let mut oneLetter: i32 = 1i32;
            let _t1 = this.next()?;
            ch = _t1;
            this.unread()?;
            oneLetter = 0i32;
            let _t2 = this.has(128i32)?;
            let _t3 = this.has(16i32)?;
            let _t4 = this.family(oneLetter, comp)?;
            return Ok(Pattern_NFCCharProperty::new(_t4)?);
            let _t5 = this.family(oneLetter, comp)?;
            let _t6 = this.newCharProperty(_t5)?;
            return Ok(_t6);
            this.unread()?;
            prev = this.cursor.get();
            let _t7 = first.escape(1i32, 0i32, 0i32)?;
            ch = _t7;
            this.append(ch, first)?;
            first = first.wrapping_add(1i32);
            let _t8: bool = Pattern::isSupplementary(ch)?;
            hasSupplementary = 1i32;
            let _t9 = this.peek()?;
            ch = _t9;
            return Ok(this.root.get());
            this.cursor.set(prev);
            prev = this.cursor.get();
            this.append(ch, first)?;
            first = first.wrapping_add(1i32);
            let _t10: bool = Pattern::isSupplementary(ch)?;
            hasSupplementary = 1i32;
            let _t11 = this.next()?;
            ch = _t11;
        }
        let _t1 = this.single(this.buffer.get()[0i32 as usize])?;
        let _t2 = this.newCharProperty(_t1)?;
        return Ok(_t2);
        let _t3 = this.newSlice(this.buffer.get(), first, hasSupplementary)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(II)V", access = "private"))]
    pub fn append(&self, ch: i32, index: i32) -> Result<()> {
        let this = self;
        let mut len: i32 = (this.buffer.get().len() as i32);
        let _t0: i32 = ArraysSupport::newLength(len, ((1i32).wrapping_add(index)).wrapping_sub(len), len)?;
        len = _t0;
        let _t1: Vec<i32> = Arrays::copyOf(&this.buffer.get(), len)?;
        this.buffer.set(_t1);
        this.buffer.get()[index as usize] = ch;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "ref", descriptor = "(I)Ljava/util/regex/Pattern$Node;", access = "private"))]
    pub fn ref(&self, refNum: i32) -> Result<Object> {
        let this = self;
        let mut done: i32 = 0i32;
        loop {
            if done!=0i32 { break; }
            let _t0 = this.peek()?;
            let mut ch: i32 = _t0;
            /* TODO: tableswitch default:106 low:48 high:57 */
            let mut newRefNum: i32 = ((refNum).wrapping_mul(10i32)).wrapping_add((ch).wrapping_sub(48i32));
            done = 1i32;
            refNum = newRefNum;
            let _t1 = this.read()?;
            done = 1i32;
        }
        this.hasGroupRef.set(1i32);
        let _t0 = this.has(2i32)?;
        let _t1 = this.has(64i32)?;
        return Ok(Pattern_CIBackRef::new(refNum, _t1)?);
        Ok(Pattern_BackRef::new(refNum)?)
    }

    #[cfg_attr(any(), java_method(name = "escape", descriptor = "(ZZZ)I", access = "private"))]
    pub fn escape(&self, inclass: bool, create: bool, isrange: bool) -> Result<i32> {
        let this = self;
        let _t0 = this.skip()?;
        let mut ch: i32 = _t0;
        /* TODO: tableswitch default:1295 low:48 high:122 */
        let _t1 = this.o()?;
        return Ok(_t1);
        let _t2 = this.ref((ch).wrapping_sub(48i32))?;
        this.root.set(_t2);
        return Ok(-1i32);
        this.root.set(Pattern_Begin::new()?);
        return Ok(-1i32);
        let _t3 = this.has(256i32)?;
        this.root.set(Pattern_Bound::new(4i32, _t3)?);
        return Ok(-1i32);
        let _t4 = this.has(256i32)?;
        let _t5: Object = CharPredicates::DIGIT()?;
        let _t6: Object = CharPredicates::ASCII_DIGIT()?;
        _t5.predicate.set(_t6);
        let _t7 = this.predicate.get().negate()?;
        this.predicate.set(_t7);
        let _t8 = this.newCharProperty(this.predicate.get())?;
        this.root.set(_t8);
        return Ok(-1i32);
        this.root.set(Pattern_LastMatch::new()?);
        return Ok(-1i32);
        let _t9: Object = Pattern::HorizWS()?;
        let _t10 = _t9.negate()?;
        this.predicate.set(_t10);
        let _t11 = this.newCharProperty(this.predicate.get())?;
        this.root.set(_t11);
        return Ok(-1i32);
        let _t12 = this.N()?;
        return Ok(_t12);
        this.root.set(Pattern_LineEnding::new()?);
        return Ok(-1i32);
        let _t13 = this.has(256i32)?;
        let _t14: Object = CharPredicates::WHITE_SPACE()?;
        let _t15: Object = CharPredicates::ASCII_SPACE()?;
        _t14.predicate.set(_t15);
        let _t16 = this.predicate.get().negate()?;
        this.predicate.set(_t16);
        let _t17 = this.newCharProperty(this.predicate.get())?;
        this.root.set(_t17);
        return Ok(-1i32);
        let _t18: Object = Pattern::VertWS()?;
        let _t19 = _t18.negate()?;
        this.predicate.set(_t19);
        let _t20 = this.newCharProperty(this.predicate.get())?;
        this.root.set(_t20);
        return Ok(-1i32);
        let _t21 = this.has(256i32)?;
        let _t22: Object = CharPredicates::WORD()?;
        let _t23: Object = CharPredicates::ASCII_WORD()?;
        _t22.predicate.set(_t23);
        let _t24 = this.predicate.get().negate()?;
        this.predicate.set(_t24);
        let _t25 = this.newCharProperty(this.predicate.get())?;
        this.root.set(_t25);
        return Ok(-1i32);
        this.root.set(Pattern_XGrapheme::new()?);
        return Ok(-1i32);
        let _t26 = this.has(1i32)?;
        this.root.set(Pattern_UnixDollar::new(0i32)?);
        this.root.set(Pattern_Dollar::new(0i32)?);
        return Ok(-1i32);
        return Ok(7i32);
        let _t27 = this.peek()?;
        let _t28 = this.skip()?;
        let _t29 = this.read()?;
        this.root.set(Pattern_GraphemeBound::new()?);
        return Ok(-1i32);
        this.unread()?;
        this.unread()?;
        let _t30 = this.has(256i32)?;
        this.root.set(Pattern_Bound::new(3i32, _t30)?);
        return Ok(-1i32);
        let _t31 = this.c()?;
        return Ok(_t31);
        let _t32 = this.has(256i32)?;
        let _t33: Object = CharPredicates::DIGIT()?;
        let _t34: Object = CharPredicates::ASCII_DIGIT()?;
        _t33.predicate.set(_t34);
        let _t35 = this.newCharProperty(this.predicate.get())?;
        this.root.set(_t35);
        return Ok(-1i32);
        return Ok(27i32);
        return Ok(12i32);
        let _t36: Object = Pattern::HorizWS()?;
        this.predicate.set(_t36);
        let _t37 = this.newCharProperty(this.predicate.get())?;
        this.root.set(_t37);
        return Ok(-1i32);
        let _t38 = this.read()?;
        let _t39 = this.error(String::from("\k is not followed by '<' for named capturing group"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t40 = this.read()?;
        let _t41 = this.groupname(_t40)?;
        let mut name: String = _t41;
        let _t42 = this.namedGroupsMap()?;
        let _t43 = _t42.get(name)?;
        let mut number: Object = _t43;
        String::new().append(&String::from("named capturing group <"))?;
        String::new().append(&name)?;
        String::new().append(&String::from("> does not exist"))?;
        let _t44 = this.error(String::new())?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.hasGroupRef.set(1i32);
        let _t45 = this.has(2i32)?;
        let _t46 = this.has(64i32)?;
        this.root.set(Pattern_CIBackRef::new(number, _t46)?);
        this.root.set(Pattern_BackRef::new(number)?);
        return Ok(-1i32);
        return Ok(10i32);
        return Ok(13i32);
        let _t47 = this.has(256i32)?;
        let _t48: Object = CharPredicates::WHITE_SPACE()?;
        let _t49: Object = CharPredicates::ASCII_SPACE()?;
        _t48.predicate.set(_t49);
        let _t50 = this.newCharProperty(this.predicate.get())?;
        this.root.set(_t50);
        return Ok(-1i32);
        return Ok(9i32);
        let _t51 = this.u()?;
        return Ok(_t51);
        return Ok(11i32);
        let _t52: Object = Pattern::VertWS()?;
        this.predicate.set(_t52);
        let _t53 = this.newCharProperty(this.predicate.get())?;
        this.root.set(_t53);
        return Ok(-1i32);
        let _t54 = this.has(256i32)?;
        let _t55: Object = CharPredicates::WORD()?;
        let _t56: Object = CharPredicates::ASCII_WORD()?;
        _t55.predicate.set(_t56);
        let _t57 = this.newCharProperty(this.predicate.get())?;
        this.root.set(_t57);
        return Ok(-1i32);
        let _t58 = this.x()?;
        return Ok(_t58);
        this.root.set(Pattern_End::new()?);
        return Ok(-1i32);
        return Ok(ch);
        let _t59 = this.error(String::from("Illegal/unsupported escape sequence"))?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "clazz", descriptor = "(Z)Ljava/util/regex/Pattern$CharPredicate;", access = "private"))]
    pub fn clazz(&self, consume: bool) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        let mut prev: i32 = todo!("stack underflow");
        /* TODO: aconst_null  */
        let mut curr: i32 = todo!("stack underflow");
        let mut bits: Pattern_BitClass = Pattern_BitClass::new()?;
        let mut isNeg: i32 = 0i32;
        let mut hasBits: i32 = 0i32;
        let _t0 = this.next()?;
        let mut ch: i32 = _t0;
        let _t1 = this.next()?;
        ch = _t1;
        isNeg = 1i32;
        loop {
            /* TODO: lookupswitch default:414 0:339 38:132 91:100 93:358 */
            let _t0 = this.clazz(1i32)?;
            curr = _t0;
            prev = curr;
            let _t1 = prev.union(curr)?;
            prev = _t1;
            let _t2 = this.peek()?;
            ch = _t2;
            let _t3 = this.next()?;
            ch = _t3;
            if ch != 38i32 { break; }
            let _t0 = this.next()?;
            ch = _t0;
            /* TODO: aconst_null  */
            let mut right: i32 = todo!("stack underflow");
            let _t1 = this.clazz(1i32)?;
            right = _t1;
            let _t2 = this.clazz(1i32)?;
            let _t3 = right.union(_t2)?;
            right = _t3;
            this.unread()?;
            let _t4 = this.clazz(0i32)?;
            right = _t4;
            let _t5 = this.clazz(0i32)?;
            let _t6 = right.union(_t5)?;
            right = _t6;
            let _t7 = this.peek()?;
            ch = _t7;
            curr = bits;
            prev = bits;
            let _t8 = prev.union(bits)?;
            prev = _t8;
            hasBits = 0i32;
            curr = right;
            let _t9 = this.error(String::from("Bad class syntax"))?;
            return Err(JvmError::Custom(String::from("athrow")));
            prev = right;
            let _t10 = this.error(String::from("Bad intersection syntax"))?;
            return Err(JvmError::Custom(String::from("athrow")));
            let _t11 = prev.and(curr)?;
            prev = _t11;
        }
        this.unread()?;
        let _t2 = this.error(String::from("Unclosed character class"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t3 = this.next()?;
        prev = bits;
        let _t4 = prev.union(bits)?;
        prev = _t4;
        let _t5 = prev.negate()?;
        return Ok(_t5);
        return Ok(prev);
        let _t6 = this.range(bits)?;
        curr = _t6;
        hasBits = 1i32;
        prev = curr;
        let _t7 = prev.union(curr)?;
        prev = _t7;
        let _t8 = this.peek()?;
        ch = _t8;
    }

    #[cfg_attr(any(), java_method(name = "bitsOrSingle", descriptor = "(Ljava/util/regex/Pattern$BitClass;I)Ljava/util/regex/Pattern$CharPredicate;", access = "private"))]
    pub fn bitsOrSingle(&self, bits: Object, ch: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.has(2i32)?;
        let _t1 = this.has(64i32)?;
        let _t2 = bits.add(ch, this.flags0.get())?;
        /* TODO: aconst_null  */
        return Ok(229i32);
        let _t3 = this.single(ch)?;
        Ok(_t3)
    }

    #[cfg_attr(any(), java_method(name = "single", descriptor = "(I)Ljava/util/regex/Pattern$CharPredicate;", access = "private"))]
    pub fn single(&self, ch: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.has(2i32)?;
        let _t1 = this.has(64i32)?;
        let _t2: i32 = Character::toUpperCase(ch)?;
        let mut upper: i32 = _t2;
        let _t3: i32 = Character::toLowerCase(upper)?;
        let mut lower: i32 = _t3;
        let _t4: Object = Pattern::SingleU(lower)?;
        return Ok(_t4);
        let _t5: bool = ASCII::isAscii(ch)?;
        let _t6: i32 = ASCII::toLower(ch)?;
        lower = _t6;
        let _t7: i32 = ASCII::toUpper(ch)?;
        upper = _t7;
        let _t8: Object = Pattern::SingleI(lower, upper)?;
        return Ok(_t8);
        let _t9: bool = Pattern::isSupplementary(ch)?;
        let _t10: Object = Pattern::SingleS(ch)?;
        return Ok(_t10);
        let _t11: Object = Pattern::Single(ch)?;
        Ok(_t11)
    }

    #[cfg_attr(any(), java_method(name = "range", descriptor = "(Ljava/util/regex/Pattern$BitClass;)Ljava/util/regex/Pattern$CharPredicate;", access = "private"))]
    pub fn range(&self, bits: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.peek()?;
        let mut ch: i32 = _t0;
        let _t1 = this.nextEscaped()?;
        ch = _t1;
        let mut comp: i32 = ch == 80i32;
        let mut oneLetter: i32 = 1i32;
        let _t2 = this.next()?;
        ch = _t2;
        this.unread()?;
        oneLetter = 0i32;
        let _t3 = this.family(oneLetter, comp)?;
        return Ok(_t3);
        comp = this.temp.get()[(this.cursor.get()).wrapping_add(1i32) as usize] == 45i32;
        this.unread()?;
        let _t4 = this.escape(1i32, 1i32, comp)?;
        ch = _t4;
        return Ok(this.predicate.get());
        let _t5 = this.next()?;
        let _t6 = this.peek()?;
        comp = this.temp.get()[(this.cursor.get()).wrapping_add(1i32) as usize];
        let _t7 = this.bitsOrSingle(bits, ch)?;
        return Ok(_t7);
        let _t8 = this.next()?;
        let _t9 = this.peek()?;
        oneLetter = _t9;
        let _t10 = this.escape(1i32, 0i32, 1i32)?;
        oneLetter = _t10;
        let _t11 = this.next()?;
        let _t12 = this.error(String::from("Illegal character range"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t13 = this.has(2i32)?;
        let _t14 = this.has(64i32)?;
        let _t15: Object = Pattern::CIRangeU(ch, oneLetter)?;
        return Ok(_t15);
        let _t16: Object = Pattern::CIRange(ch, oneLetter)?;
        return Ok(_t16);
        let _t17: Object = Pattern::Range(ch, oneLetter)?;
        return Ok(_t17);
        let _t18 = this.bitsOrSingle(bits, ch)?;
        return Ok(_t18);
        String::new().append(&String::from("Unexpected character '"))?;
        /* TODO: i2c  */
        String::new().append(&ch)?;
        String::new().append(&String::from("'"))?;
        let _t19 = this.error(String::new())?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "family", descriptor = "(ZZ)Ljava/util/regex/Pattern$CharPredicate;", access = "private"))]
    pub fn family(&self, singleLetter: bool, isComplement: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.next()?;
        /* TODO: aconst_null  */
        let mut p: i32 = todo!("stack underflow");
        let mut c: i32 = this.temp.get()[this.cursor.get() as usize];
        let _t1: bool = Character::isSupplementaryCodePoint(c)?;
        /* TODO: i2c  */
        let mut name: String = String::from_owned(format!("{}", c));
        name = String::new(this.temp.get(), this.cursor.get(), 1i32)?;
        let _t2 = this.read()?;
        c = this.cursor.get();
        this.mark(125i32)?;
        loop {
            let _t0 = this.read()?;
            if _t0 == 125i32 { break; }
        }
        this.mark(0i32)?;
        let mut j: i32 = this.cursor.get();
        let _t3 = this.error(String::from("Unclosed character family"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t4 = this.error(String::from("Empty character family"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        name = String::new(this.temp.get(), c, ((j).wrapping_sub(c)).wrapping_sub(1i32))?;
        let _t5 = name.indexOf(61i32)?;
        c = _t5;
        let _t6 = name.substring((c).wrapping_add(1i32))?;
        j = _t6;
        let _t7 = name.substring(0i32, c)?;
        let _t8 = _t7.toLowerCase(Locale::ENGLISH())?;
        name = _t8;
        let mut local_7: String = name;
        let mut local_8: i32 = -1i32;
        let _t9 = local_7.hashCode()?;
        /* TODO: lookupswitch default:363 -907685685:281 3292:332 3664:264 97633:298 93832333:315 1265003125:349 */
        let _t10 = local_7.equals(String::from("sc"))?;
        local_8 = 0i32;
        let _t11 = local_7.equals(String::from("script"))?;
        local_8 = 1i32;
        let _t12 = local_7.equals(String::from("blk"))?;
        local_8 = 2i32;
        let _t13 = local_7.equals(String::from("block"))?;
        local_8 = 3i32;
        let _t14 = local_7.equals(String::from("gc"))?;
        local_8 = 4i32;
        let _t15 = local_7.equals(String::from("general_category"))?;
        local_8 = 5i32;
        /* TODO: tableswitch default:439 low:0 high:5 */
        let _t16: Object = CharPredicates::forUnicodeScript(j)?;
        p = _t16;
        let _t17: Object = CharPredicates::forUnicodeBlock(j)?;
        p = _t17;
        let _t18 = this.has(2i32)?;
        let _t19: Object = CharPredicates::forProperty(j, _t18)?;
        p = _t19;
        String::new().append(&String::from("Unknown Unicode property {name=<"))?;
        String::new().append(&name)?;
        String::new().append(&String::from(">, value=<"))?;
        String::new().append(&j)?;
        String::new().append(&String::from(">}"))?;
        let _t20 = this.error(String::new())?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t21 = name.startsWith(String::from("In"))?;
        let _t22 = name.substring(2i32)?;
        let _t23: Object = CharPredicates::forUnicodeBlock(_t22)?;
        p = _t23;
        let _t24 = name.startsWith(String::from("Is"))?;
        let _t25 = name.substring(2i32)?;
        j = _t25;
        let _t26 = this.has(2i32)?;
        let _t27: Object = CharPredicates::forUnicodeProperty(j, _t26)?;
        p = _t27;
        let _t28 = this.has(2i32)?;
        let _t29: Object = CharPredicates::forProperty(j, _t28)?;
        p = _t29;
        let _t30: Object = CharPredicates::forUnicodeScript(j)?;
        p = _t30;
        let _t31 = this.has(256i32)?;
        let _t32 = this.has(2i32)?;
        let _t33: Object = CharPredicates::forPOSIXName(name, _t32)?;
        p = _t33;
        let _t34 = this.has(2i32)?;
        let _t35: Object = CharPredicates::forProperty(name, _t34)?;
        p = _t35;
        String::new().append(&String::from("Unknown character property name {"))?;
        String::new().append(&name)?;
        String::new().append(&String::from("}"))?;
        let _t36 = this.error(String::new())?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.hasSupplementary.set(1i32);
        let _t37 = p.negate()?;
        p = _t37;
        Ok(p)
    }

    #[cfg_attr(any(), java_method(name = "newCharProperty", descriptor = "(Ljava/util/regex/Pattern$CharPredicate;)Ljava/util/regex/Pattern$CharProperty;", access = "private"))]
    pub fn newCharProperty(&self, p: Object) -> Result<Object> {
        let this = self;
        /* TODO: aconst_null  */
        return Ok(p);
        return Ok(Pattern_BmpCharProperty::new(p)?);
        this.hasSupplementary.set(1i32);
        Ok(Pattern_CharProperty::new(p)?)
    }

    #[cfg_attr(any(), java_method(name = "groupname", descriptor = "(I)Ljava/lang/String;", access = "private"))]
    pub fn groupname(&self, ch: i32) -> Result<String> {
        let this = self;
        let mut sb: String = String::new();
        let _t0: bool = ASCII::isAlpha(ch)?;
        let _t1 = this.error(String::from("capturing group name does not start with a Latin letter"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: i2c  */
        sb.append(&ch)?;
        let _t2 = this.read()?;
        ch = _t2;
        let _t3: bool = ASCII::isAlnum(_t2)?;
        let _t4 = this.error(String::from("named capturing group is missing trailing '>'"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(sb)
    }

    #[cfg_attr(any(), java_method(name = "group0", descriptor = "()Ljava/util/regex/Pattern$Node;", access = "private"))]
    pub fn group0(&self) -> Result<Object> {
        let this = self;
        let mut capturingGroup: i32 = 0i32;
        let mut save: i32 = this.flags0.get();
        let _t0 = this.topClosureNodes.get().size()?;
        let mut saveTCNCount: i32 = _t0;
        /* TODO: aconst_null  */
        todo!("stack underflow").root.set(this);
        let _t1 = this.next()?;
        let mut ch: i32 = _t1;
        let _t2 = this.skip()?;
        ch = _t2;
        /* TODO: lookupswitch default:578 33:135 36:570 58:112 60:227 61:135 62:190 64:570 */
        let _t3 = this.createGroup(1i32)?;
        let mut head: Object = _t3;
        let mut tail: Object = this.root.get();
        let _t4 = this.expr(tail)?;
        head.next.set(_t4);
        let _t5 = this.createGroup(1i32)?;
        head = _t5;
        tail = this.root.get();
        let _t6 = this.expr(tail)?;
        head.next.set(_t6);
        tail = Pattern_Pos::new(head)?;
        head = Pattern_Pos::new(head)?;
        tail = Pattern_Neg::new(head)?;
        head = Pattern_Neg::new(head)?;
        let _t7 = this.createGroup(1i32)?;
        head = _t7;
        tail = this.root.get();
        let _t8 = this.expr(tail)?;
        head.next.set(_t8);
        tail = Pattern_Ques::new(head, Pattern$Qtype::INDEPENDENT())?;
        head = Pattern_Ques::new(head, Pattern$Qtype::INDEPENDENT())?;
        let _t9 = this.read()?;
        ch = _t9;
        let _t10 = this.groupname(ch)?;
        let mut name: String = _t10;
        let _t11 = this.namedGroupsMap()?;
        let _t12 = _t11.containsKey(name)?;
        String::new().append(&String::from("Named capturing group <"))?;
        String::new().append(&name)?;
        String::new().append(&String::from("> is already defined"))?;
        let _t13 = this.error(String::new())?;
        return Err(JvmError::Custom(String::from("athrow")));
        capturingGroup = 1i32;
        let _t14 = this.createGroup(0i32)?;
        head = _t14;
        tail = this.root.get();
        let _t15 = this.namedGroupsMap()?;
        let _t16 = _t15.put(name, (this.capturingGroupCount.get()).wrapping_sub(1i32))?;
        let _t17 = this.expr(tail)?;
        head.next.set(_t17);
        name = this.cursor.get();
        let _t18 = this.createGroup(1i32)?;
        head = _t18;
        tail = this.root.get();
        let _t19 = this.expr(tail)?;
        head.next.set(_t19);
        tail.next.set(Pattern$LookBehindEndNode::INSTANCE());
        let mut info: Pattern_TreeInfo = Pattern_TreeInfo::new()?;
        let _t20 = head.study(info)?;
        let _t21 = this.error(String::from("Look-behind group does not have an obvious maximum length"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t22 = this.findSupplementary(name, this.patternLength.get())?;
        let mut hasSupplementary: i32 = _t22;
        tail = Pattern_Behind::new(head, info.maxLength.get(), info.minLength.get())?;
        head = Pattern_Behind::new(head, info.maxLength.get(), info.minLength.get())?;
        tail = Pattern_NotBehind::new(head, info.maxLength.get(), info.minLength.get())?;
        head = Pattern_NotBehind::new(head, info.maxLength.get(), info.minLength.get())?;
        let _t23 = this.topClosureNodes.get().size()?;
        let _t24 = this.topClosureNodes.get().size()?;
        let _t25 = this.topClosureNodes.get().subList(saveTCNCount, _t24)?;
        _t25.clear()?;
        let _t26 = this.error(String::from("Unknown group type"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.unread()?;
        this.addFlag()?;
        let _t27 = this.read()?;
        ch = _t27;
        /* TODO: aconst_null  */
        return Ok(41i32);
        let _t28 = this.error(String::from("Unknown inline modifier"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t29 = this.createGroup(1i32)?;
        head = _t29;
        tail = this.root.get();
        let _t30 = this.expr(tail)?;
        head.next.set(_t30);
        capturingGroup = 1i32;
        let _t31 = this.createGroup(0i32)?;
        head = _t31;
        tail = this.root.get();
        let _t32 = this.expr(tail)?;
        head.next.set(_t32);
        this.accept(41i32, String::from("Unclosed group"))?;
        this.flags0.set(save);
        let _t33 = this.closure(head)?;
        name = _t33;
        this.root.set(tail);
        return Ok(name);
        this.root.set(name);
        return Ok(name);
        let _t34 = this.topClosureNodes.get().size()?;
        let _t35 = this.topClosureNodes.get().size()?;
        let _t36 = this.topClosureNodes.get().subList(saveTCNCount, _t35)?;
        _t36.clear()?;
        info = name;
        this.root.set(name);
        return Ok(name);
        tail.next.set(Pattern_BranchConn::new()?);
        tail = tail.next.get();
        /* TODO: aconst_null  */
        let mut _obj37: Pattern_Branch = Pattern_Branch::new(Pattern$Branch::new(), head, tail)?;
        head = _obj37;
        /* TODO: aconst_null  */
        let mut _obj38: Pattern_Branch = Pattern_Branch::new(Pattern$Branch::new(), head, tail)?;
        head = _obj38;
        this.root.set(tail);
        return Ok(head);
        hasSupplementary = name;
        this.root.set(name);
        return Ok(name);
        let mut info: Pattern_TreeInfo = Pattern_TreeInfo::new()?;
        let _t39 = head.study(info)?;
        let mut temp: Object = tail;
        this.root.set(Pattern_GroupCurly::new(head.next.get(), hasSupplementary.cmin.get(), hasSupplementary.cmax.get(), hasSupplementary.type.get(), tail.localIndex.get(), tail.groupIndex.get(), capturingGroup)?);
        head = Pattern_GroupCurly::new(head.next.get(), hasSupplementary.cmin.get(), hasSupplementary.cmax.get(), hasSupplementary.type.get(), tail.localIndex.get(), tail.groupIndex.get(), capturingGroup)?;
        return Ok(head);
        temp = head.localIndex.get();
        let mut loop_: Pattern_Loop = Pattern_Loop::new(this.localCount.get(), temp)?;
        let _t40 = this.topClosureNodes.get().add(loop_)?;
        loop_ = Pattern_LazyLoop::new(this.localCount.get(), temp)?;
        let mut prolog: Pattern_Prolog = Pattern_Prolog::new(loop_)?;
        this.localCount.set((this.localCount.get()).wrapping_add(1i32));
        loop_.cmin.set(hasSupplementary.cmin.get());
        loop_.cmax.set(hasSupplementary.cmax.get());
        loop_.body.set(head);
        tail.next.set(loop_);
        this.root.set(loop_);
        return Ok(prolog);
        let _t41 = this.error(String::from("Internal logic error"))?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "createGroup", descriptor = "(Z)Ljava/util/regex/Pattern$Node;", access = "private"))]
    pub fn createGroup(&self, anonymous: bool) -> Result<Object> {
        let this = self;
        this.localCount.set((this.localCount.get()).wrapping_add(1i32));
        let mut localIndex: i32 = this.localCount.get();
        let mut groupIndex: i32 = 0i32;
        this.capturingGroupCount.set((this.capturingGroupCount.get()).wrapping_add(1i32));
        groupIndex = this.capturingGroupCount.get();
        let mut head: Pattern_GroupHead = Pattern_GroupHead::new(localIndex)?;
        this.root.set(Pattern_GroupTail::new(localIndex, groupIndex)?);
        head.tail.set(this.root.get());
        this.groupNodes.get()[groupIndex as usize] = head;
        Ok(head)
    }

    #[cfg_attr(any(), java_method(name = "addFlag", descriptor = "()V", access = "private"))]
    pub fn addFlag(&self) -> Result<()> {
        let this = self;
        let _t0 = this.peek()?;
        let mut ch: i32 = _t0;
        /* TODO: lookupswitch default:208 45:199 85:184 99:156 100:129 105:88 109:101 115:115 117:142 120:171 */
        this.flags0.set((this.flags0.get()|2i32));
        this.flags0.set((this.flags0.get()|8i32));
        this.flags0.set((this.flags0.get()|32i32));
        this.flags0.set((this.flags0.get()|1i32));
        this.flags0.set((this.flags0.get()|64i32));
        this.flags0.set((this.flags0.get()|128i32));
        this.flags0.set((this.flags0.get()|4i32));
        this.flags0.set((this.flags0.get()|320i32));
        let _t1 = this.next()?;
        ch = _t1;
        this.subFlag()?;
        return Ok(());
        let _t2 = this.next()?;
        ch = _t2;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "subFlag", descriptor = "()V", access = "private"))]
    pub fn subFlag(&self) -> Result<()> {
        let this = self;
        let _t0 = this.peek()?;
        let mut ch: i32 = _t0;
        /* TODO: lookupswitch default:194 85:179 99:150 100:122 105:80 109:94 115:108 117:136 120:165 */
        this.flags0.set((this.flags0.get()&-3i32));
        this.flags0.set((this.flags0.get()&-9i32));
        this.flags0.set((this.flags0.get()&-33i32));
        this.flags0.set((this.flags0.get()&-2i32));
        this.flags0.set((this.flags0.get()&-65i32));
        this.flags0.set((this.flags0.get()&-129i32));
        this.flags0.set((this.flags0.get()&-5i32));
        this.flags0.set((this.flags0.get()&-321i32));
        return Ok(());
        let _t1 = this.next()?;
        ch = _t1;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "qtype", descriptor = "()Ljava/util/regex/Pattern$Qtype;", access = "private"))]
    pub fn qtype(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.next()?;
        let mut ch: i32 = _t0;
        let _t1 = this.next()?;
        return Ok(Pattern$Qtype::LAZY());
        let _t2 = this.next()?;
        return Ok(Pattern$Qtype::POSSESSIVE());
        Ok(Pattern$Qtype::GREEDY())
    }

    #[cfg_attr(any(), java_method(name = "curly", descriptor = "(Ljava/util/regex/Pattern$Node;I)Ljava/util/regex/Pattern$Node;", access = "private"))]
    pub fn curly(&self, prev: Object, cmin: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.qtype()?;
        let mut qtype: Object = _t0;
        return Ok(Pattern_BmpCharPropertyGreedy::new(prev, cmin)?);
        return Ok(Pattern_CharPropertyGreedy::new(prev, cmin)?);
        Ok(Pattern_Curly::new(prev, cmin, 982i32, qtype)?)
    }

    #[cfg_attr(any(), java_method(name = "closure", descriptor = "(Ljava/util/regex/Pattern$Node;)Ljava/util/regex/Pattern$Node;", access = "private"))]
    pub fn closure(&self, prev: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.peek()?;
        let mut ch: i32 = _t0;
        /* TODO: lookupswitch default:274 42:61 43:68 63:48 123:75 */
        let _t1 = this.qtype()?;
        return Ok(Pattern_Ques::new(prev, _t1)?);
        let _t2 = this.curly(prev, 0i32)?;
        return Ok(_t2);
        let _t3 = this.curly(prev, 1i32)?;
        return Ok(_t3);
        let _t4 = this.skip()?;
        ch = _t4;
        let _t5: bool = ASCII::isDigit(ch)?;
        let mut cmin: i32 = 0i32;
        let _t6: i32 = (cmin).abs();
        let _t7: i32 = (_t6).abs();
        cmin = _t7;
        let _t8 = this.read()?;
        ch = _t8;
        let _t9: bool = ASCII::isDigit(_t8)?;
        let _t10 = this.read()?;
        ch = _t10;
        this.unread()?;
        let _t11 = this.curly(prev, cmin)?;
        return Ok(_t11);
        let mut cmax: i32 = 0i32;
        loop {
            let _t0: bool = ASCII::isDigit(ch)?;
            if _t0==0i32 { break; }
            let _t0: i32 = (cmax).abs();
            let _t1: i32 = (_t0).abs();
            cmax = _t1;
            let _t2 = this.read()?;
            ch = _t2;
        }
        cmax = cmin;
        let mut ae: i32 = 125i32;
        let _t12 = this.error(String::from("Illegal repetition range"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t13 = this.error(String::from("Unclosed counted closure"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t14 = this.error(String::from("Illegal repetition range"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.unread()?;
        let _t15 = this.qtype()?;
        let _t16 = this.qtype()?;
        return Ok(Pattern_Curly::new(prev, cmin, cmax, _t16)?);
        let _t17 = this.error(String::from("Illegal repetition"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(prev)
    }

    #[cfg_attr(any(), java_method(name = "c", descriptor = "()I", access = "private"))]
    pub fn c(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.read()?;
        return Ok((_t0^64i32));
        let _t1 = this.error(String::from("Illegal control escape sequence"))?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "o", descriptor = "()I", access = "private"))]
    pub fn o(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.read()?;
        let mut n: i32 = _t0;
        let _t1 = this.read()?;
        let mut m: i32 = _t1;
        let _t2 = this.read()?;
        let mut o: i32 = _t2;
        return Ok(((((n).wrapping_sub(48i32)).wrapping_mul(64i32)).wrapping_add(((m).wrapping_sub(48i32)).wrapping_mul(8i32))).wrapping_add((o).wrapping_sub(48i32)));
        this.unread()?;
        return Ok((((n).wrapping_sub(48i32)).wrapping_mul(8i32)).wrapping_add((m).wrapping_sub(48i32)));
        this.unread()?;
        return Ok((n).wrapping_sub(48i32));
        let _t3 = this.error(String::from("Illegal octal escape sequence"))?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "x", descriptor = "()I", access = "private"))]
    pub fn x(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.read()?;
        let mut n: i32 = _t0;
        let _t1: bool = ASCII::isHexDigit(n)?;
        let _t2 = this.read()?;
        let mut m: i32 = _t2;
        let _t3: bool = ASCII::isHexDigit(m)?;
        let _t4: i32 = ASCII::toDigit(n)?;
        let _t5: i32 = ASCII::toDigit(m)?;
        return Ok(((_t4).wrapping_mul(16i32)).wrapping_add(_t5));
        let _t6 = this.peek()?;
        let _t7: bool = ASCII::isHexDigit(_t6)?;
        m = 0i32;
        let _t8 = this.read()?;
        n = _t8;
        let _t9: bool = ASCII::isHexDigit(_t8)?;
        let _t10: i32 = ASCII::toDigit(n)?;
        m = ((m<<(4i32&0x1f))).wrapping_add(_t10);
        let _t11 = this.error(String::from("Hexadecimal codepoint is too big"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t12 = this.error(String::from("Unclosed hexadecimal escape sequence"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(m);
        let _t13 = this.error(String::from("Illegal hexadecimal escape sequence"))?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "cursor", descriptor = "()I", access = "private"))]
    pub fn cursor(&self) -> Result<i32> {
        let this = self;
        Ok(this.cursor.get())
    }

    #[cfg_attr(any(), java_method(name = "setcursor", descriptor = "(I)V", access = "private"))]
    pub fn setcursor(&self, pos: i32) -> Result<()> {
        let this = self;
        this.cursor.set(pos);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "uxxxx", descriptor = "()I", access = "private"))]
    pub fn uxxxx(&self) -> Result<i32> {
        let this = self;
        let mut n: i32 = 0i32;
        let mut i: i32 = 0i32;
        loop {
            if i >= 4i32 { break; }
            let _t0 = this.read()?;
            let mut ch: i32 = _t0;
            let _t1: bool = ASCII::isHexDigit(ch)?;
            let _t2 = this.error(String::from("Illegal Unicode escape sequence"))?;
            return Err(JvmError::Custom(String::from("athrow")));
            let _t3: i32 = ASCII::toDigit(ch)?;
            n = ((n).wrapping_mul(16i32)).wrapping_add(_t3);
            i = i.wrapping_add(1i32);
        }
        Ok(n)
    }

    #[cfg_attr(any(), java_method(name = "u", descriptor = "()I", access = "private"))]
    pub fn u(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.uxxxx()?;
        let mut n: i32 = _t0;
        /* TODO: i2c  */
        let _t1: bool = Character::isHighSurrogate(n)?;
        let _t2 = this.cursor()?;
        let mut cur: i32 = _t2;
        let _t3 = this.read()?;
        let _t4 = this.read()?;
        let _t5 = this.uxxxx()?;
        let mut n2: i32 = _t5;
        /* TODO: i2c  */
        let _t6: bool = Character::isLowSurrogate(n2)?;
        /* TODO: i2c  */
        /* TODO: i2c  */
        let _t7: i32 = Character::toCodePoint(n, n2)?;
        return Ok(_t7);
        this.setcursor(cur)?;
        Ok(n)
    }

    #[cfg_attr(any(), java_method(name = "N", descriptor = "()I", access = "private"))]
    pub fn N(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.read()?;
        let mut i: i32 = this.cursor.get();
        let _t1 = this.read()?;
        let _t2 = this.error(String::from("Unclosed character name escape sequence"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut name: String = String::new(this.temp.get(), i, ((this.cursor.get()).wrapping_sub(i)).wrapping_sub(1i32))?;
        let _t3: i32 = Character::codePointOf(name)?;
        return Ok(_t3);
        let mut x: i32 = this.patternLength.get();
        String::new().append(&String::from("Unknown character name ["))?;
        String::new().append(&name)?;
        String::new().append(&String::from("]"))?;
        let _t4 = this.error(String::new())?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t5 = this.error(String::from("Illegal character name escape sequence"))?;
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "countChars", descriptor = "(Ljava/lang/CharSequence;II)I", access = "private static final"))]
    pub fn countChars(seq: Object, index: i32, lengthInCodePoints: i32) -> Result<i32> {
        let _t0 = seq.length()?;
        let _t1 = seq.charAt(index)?;
        let _t2: bool = Character::isHighSurrogate(_t1)?;
        return Ok(1i32);
        let _t3 = seq.length()?;
        let mut length: i32 = _t3;
        let mut x: i32 = index;
        return Err(JvmError::Custom(String::from("athrow")));
        let mut i: i32 = 0i32;
        loop {
            if x >= length { break; }
            x = x.wrapping_add(1i32);
            let _t0 = seq.charAt(x)?;
            let _t1: bool = Character::isHighSurrogate(_t0)?;
            let _t2 = seq.charAt(x)?;
            let _t3: bool = Character::isLowSurrogate(_t2)?;
            x = x.wrapping_add(1i32);
            i = i.wrapping_add(1i32);
        }
        return Ok((x).wrapping_sub(index));
        return Err(JvmError::Custom(String::from("athrow")));
        return Ok(0i32);
        i = (lengthInCodePoints).wrapping_neg();
        let mut i: i32 = 0i32;
        loop {
            if x<=0i32 { break; }
            x = x.wrapping_sub(1i32);
            let _t0 = seq.charAt(x)?;
            let _t1: bool = Character::isLowSurrogate(_t0)?;
            let _t2 = seq.charAt((x).wrapping_sub(1i32))?;
            let _t3: bool = Character::isHighSurrogate(_t2)?;
            x = x.wrapping_sub(1i32);
            i = i.wrapping_add(1i32);
        }
        Ok((index).wrapping_sub(x))
    }

    #[cfg_attr(any(), java_method(name = "countCodePoints", descriptor = "(Ljava/lang/CharSequence;)I", access = "private static final"))]
    pub fn countCodePoints(seq: Object) -> Result<i32> {
        let _t0 = seq.length()?;
        let mut length: i32 = _t0;
        let mut n: i32 = 0i32;
        let mut i: i32 = 0i32;
        loop {
            if i >= length { break; }
            n = n.wrapping_add(1i32);
            i = i.wrapping_add(1i32);
            let _t0 = seq.charAt(i)?;
            let _t1: bool = Character::isHighSurrogate(_t0)?;
            let _t2 = seq.charAt(i)?;
            let _t3: bool = Character::isLowSurrogate(_t2)?;
            i = i.wrapping_add(1i32);
        }
        Ok(n)
    }

    #[cfg_attr(any(), java_method(name = "newSlice", descriptor = "([IIZ)Ljava/util/regex/Pattern$Node;", access = "private"))]
    pub fn newSlice(&self, buf: Vec<i32>, count: i32, hasSupplementary: bool) -> Result<Object> {
        let this = self;
        let mut _arr0: Vec<i32> = vec![0i32; count as usize];
        let mut tmp: Vec<i32> = _arr0;
        let _t1 = this.has(2i32)?;
        let _t2 = this.has(64i32)?;
        let mut i: i32 = 0i32;
        loop {
            if i >= count { break; }
            let _t0: i32 = Character::toUpperCase(buf[i as usize])?;
            let _t1: i32 = Character::toLowerCase(_t0)?;
            tmp[i as usize] = _t1;
            i = i.wrapping_add(1i32);
        }
        return Ok(Pattern_SliceU::new(tmp)?);
        i = 0i32;
        loop {
            if i >= count { break; }
            let _t0: i32 = ASCII::toLower(buf[i as usize])?;
            tmp[i as usize] = _t0;
            i = i.wrapping_add(1i32);
        }
        return Ok(Pattern_SliceI::new(tmp)?);
        i = 0i32;
        loop {
            if i >= count { break; }
            tmp[i as usize] = buf[i as usize];
            i = i.wrapping_add(1i32);
        }
        Ok(Pattern_Slice::new(tmp)?)
    }

    #[cfg_attr(any(), java_method(name = "hasBaseCharacter", descriptor = "(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)Z", access = "private static"))]
    pub fn hasBaseCharacter(matcher: Object, i: i32, seq: Object) -> Result<bool> {
        let mut start: i32 = 0i32;
        let mut x: i32 = i;
        loop {
            if x < start { break; }
            let _t0: i32 = Character::codePointAt(seq, x)?;
            let mut ch: i32 = _t0;
            let _t1: bool = Character::isLetterOrDigit(ch)?;
            return Ok(1i32);
            let _t2: i32 = Character::getType(ch)?;
            return Ok(0i32);
            x = x.wrapping_sub(1i32);
        }
        Ok(0i32)
    }

    #[cfg_attr(any(), java_method(name = "and", descriptor = "(Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;Z)Ljava/util/regex/Pattern$CharPredicate;", access = "private static"))]
    pub fn and(p1: Object, p2: Object, bmpChar: bool) -> Result<Object> {
        /* TODO: invokedynamic 1114 */
        return Ok(p2);
        /* TODO: invokedynamic 1118 */
        Ok(p2)
    }

    #[cfg_attr(any(), java_method(name = "union", descriptor = "(Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;Z)Ljava/util/regex/Pattern$CharPredicate;", access = "private static"))]
    // java: union(Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;Z)Ljava/util/regex/Pattern$CharPredicate;
    pub fn union__patter_patter_z(p1: Object, p2: Object, bmpChar: bool) -> Result<Object> {
        /* TODO: invokedynamic 1121 */
        return Ok(p2);
        /* TODO: invokedynamic 1122 */
        Ok(p2)
    }

    #[cfg_attr(any(), java_method(name = "union", descriptor = "(Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;Z)Ljava/util/regex/Pattern$CharPredicate;", access = "private static"))]
    // java: union(Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;Ljava/util/regex/Pattern$CharPredicate;Z)Ljava/util/regex/Pattern$CharPredicate;
    pub fn union__patter_patter_patter_z(p1: Object, p2: Object, p3: Object, bmpChar: bool) -> Result<Object> {
        /* TODO: invokedynamic 1123 */
        return Ok(p3);
        /* TODO: invokedynamic 1126 */
        Ok(p3)
    }

    #[cfg_attr(any(), java_method(name = "negate", descriptor = "(Ljava/util/regex/Pattern$CharPredicate;)Ljava/util/regex/Pattern$CharPredicate;", access = "private static"))]
    pub fn negate(p1: Object) -> Result<Object> {
        /* TODO: invokedynamic 1129 */
        Ok(p1)
    }

    #[cfg_attr(any(), java_method(name = "VertWS", descriptor = "()Ljava/util/regex/Pattern$BmpCharPredicate;", access = "static"))]
    pub fn VertWS() -> Result<Object> {
        /* TODO: invokedynamic 1131 */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "HorizWS", descriptor = "()Ljava/util/regex/Pattern$BmpCharPredicate;", access = "static"))]
    pub fn HorizWS() -> Result<Object> {
        /* TODO: invokedynamic 1133 */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "ALL", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "static"))]
    pub fn ALL() -> Result<Object> {
        /* TODO: invokedynamic 1134 */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "DOT", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "static"))]
    pub fn DOT() -> Result<Object> {
        /* TODO: invokedynamic 1136 */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "UNIXDOT", descriptor = "()Ljava/util/regex/Pattern$CharPredicate;", access = "static"))]
    pub fn UNIXDOT() -> Result<Object> {
        /* TODO: invokedynamic 1137 */
        Ok(todo!("stack underflow"))
    }

    #[cfg_attr(any(), java_method(name = "SingleS", descriptor = "(I)Ljava/util/regex/Pattern$CharPredicate;", access = "static"))]
    pub fn SingleS(c: i32) -> Result<Object> {
        /* TODO: invokedynamic 1138 */
        Ok(c)
    }

    #[cfg_attr(any(), java_method(name = "Single", descriptor = "(I)Ljava/util/regex/Pattern$BmpCharPredicate;", access = "static"))]
    pub fn Single(c: i32) -> Result<Object> {
        /* TODO: invokedynamic 1140 */
        Ok(c)
    }

    #[cfg_attr(any(), java_method(name = "SingleI", descriptor = "(II)Ljava/util/regex/Pattern$BmpCharPredicate;", access = "static"))]
    pub fn SingleI(lower: i32, upper: i32) -> Result<Object> {
        /* TODO: invokedynamic 1142 */
        Ok(upper)
    }

    #[cfg_attr(any(), java_method(name = "SingleU", descriptor = "(I)Ljava/util/regex/Pattern$CharPredicate;", access = "static"))]
    pub fn SingleU(lower: i32) -> Result<Object> {
        /* TODO: invokedynamic 1144 */
        Ok(lower)
    }

    #[cfg_attr(any(), java_method(name = "inRange", descriptor = "(III)Z", access = "private static"))]
    pub fn inRange(lower: i32, ch: i32, upper: i32) -> Result<bool> {
        Ok(ch <= upper)
    }

    #[cfg_attr(any(), java_method(name = "Range", descriptor = "(II)Ljava/util/regex/Pattern$CharPredicate;", access = "static"))]
    pub fn Range(lower: i32, upper: i32) -> Result<Object> {
        /* TODO: invokedynamic 1147 */
        return Ok(upper);
        /* TODO: invokedynamic 1148 */
        Ok(upper)
    }

    #[cfg_attr(any(), java_method(name = "CIRange", descriptor = "(II)Ljava/util/regex/Pattern$CharPredicate;", access = "static"))]
    pub fn CIRange(lower: i32, upper: i32) -> Result<Object> {
        /* TODO: invokedynamic 1150 */
        Ok(upper)
    }

    #[cfg_attr(any(), java_method(name = "CIRangeU", descriptor = "(II)Ljava/util/regex/Pattern$CharPredicate;", access = "static"))]
    pub fn CIRangeU(lower: i32, upper: i32) -> Result<Object> {
        /* TODO: invokedynamic 1151 */
        Ok(upper)
    }

    #[cfg_attr(any(), java_method(name = "asPredicate", descriptor = "()Ljava/util/function/Predicate;", access = "public"))]
    pub fn asPredicate(&self) -> Result<Object> {
        let this = self;
        /* TODO: invokedynamic 1152 */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "asMatchPredicate", descriptor = "()Ljava/util/function/Predicate;", access = "public"))]
    pub fn asMatchPredicate(&self) -> Result<Object> {
        let this = self;
        /* TODO: invokedynamic 1156 */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "splitAsStream", descriptor = "(Ljava/lang/CharSequence;)Ljava/util/stream/Stream;", access = "public"))]
    pub fn splitAsStream(&self, input: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Spliterators::spliteratorUnknownSize(Pattern_1MatcherIterator::new(this, input)?, 272i32)?;
        let _t1: Object = StreamSupport::stream(_t0, 0i32)?;
        Ok(_t1)
    }
}
