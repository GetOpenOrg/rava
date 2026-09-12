#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/regex/Matcher",
    super_class = "java/lang/Object",
    interfaces  = "java/util/regex/MatchResult",
    access      = "public final",
    source      = "Matcher.java",
))]
pub struct Matcher {
    #[cfg_attr(any(), java_field(name = "parentPattern", descriptor = "Ljava/util/regex/Pattern;"))]
    pub parentPattern: Field<Object>,
    #[cfg_attr(any(), java_field(name = "groups", descriptor = "[I"))]
    pub groups: Field<Vec<i32>>,
    #[cfg_attr(any(), java_field(name = "from", descriptor = "I"))]
    pub from: Field<i32>,
    #[cfg_attr(any(), java_field(name = "to", descriptor = "I"))]
    pub to: Field<i32>,
    #[cfg_attr(any(), java_field(name = "lookbehindTo", descriptor = "I"))]
    pub lookbehindTo: Field<i32>,
    #[cfg_attr(any(), java_field(name = "text", descriptor = "Ljava/lang/CharSequence;"))]
    pub text: Field<Object>,
    #[cfg_attr(any(), java_field(name = "acceptMode", descriptor = "I"))]
    pub acceptMode: Field<i32>,
    #[cfg_attr(any(), java_field(name = "first", descriptor = "I"))]
    pub first: Field<i32>,
    #[cfg_attr(any(), java_field(name = "last", descriptor = "I"))]
    pub last: Field<i32>,
    #[cfg_attr(any(), java_field(name = "oldLast", descriptor = "I"))]
    pub oldLast: Field<i32>,
    #[cfg_attr(any(), java_field(name = "lastAppendPosition", descriptor = "I"))]
    pub lastAppendPosition: Field<i32>,
    #[cfg_attr(any(), java_field(name = "locals", descriptor = "[I"))]
    pub locals: Field<Vec<i32>>,
    #[cfg_attr(any(), java_field(name = "localsPos", descriptor = "[Ljava/util/regex/IntHashSet;"))]
    pub localsPos: Field<Vec<Object>>,
    #[cfg_attr(any(), java_field(name = "hitEnd", descriptor = "Z"))]
    pub hitEnd: Field<bool>,
    #[cfg_attr(any(), java_field(name = "requireEnd", descriptor = "Z"))]
    pub requireEnd: Field<bool>,
    #[cfg_attr(any(), java_field(name = "transparentBounds", descriptor = "Z"))]
    pub transparentBounds: Field<bool>,
    #[cfg_attr(any(), java_field(name = "anchoringBounds", descriptor = "Z"))]
    pub anchoringBounds: Field<bool>,
    #[cfg_attr(any(), java_field(name = "modCount", descriptor = "I"))]
    pub modCount: Field<i32>,
    #[cfg_attr(any(), java_field(name = "namedGroups", descriptor = "Ljava/util/Map;", access = "private"))]
    pub namedGroups: Field<Object>,
}

impl Matcher {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V"))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { parentPattern: Field::new(Default::default()), groups: Field::new(Default::default()), from: Field::new(0), to: Field::new(0), lookbehindTo: Field::new(0), text: Field::new(Default::default()), acceptMode: Field::new(0), first: Field::new(0), last: Field::new(0), oldLast: Field::new(0), lastAppendPosition: Field::new(0), locals: Field::new(Default::default()), localsPos: Field::new(Default::default()), hitEnd: Field::new(false), requireEnd: Field::new(false), transparentBounds: Field::new(false), anchoringBounds: Field::new(false), modCount: Field::new(0), namedGroups: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.acceptMode.set(0i32);
        this.first.set(-1i32);
        this.last.set(0i32);
        this.oldLast.set(-1i32);
        this.lastAppendPosition.set(0i32);
        this.transparentBounds.set(0i32);
        this.anchoringBounds.set(1i32);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/regex/Pattern;Ljava/lang/CharSequence;)V"))]
    // java: <init>(Ljava/util/regex/Pattern;Ljava/lang/CharSequence;)V
    pub fn new__patter_seq(parent: Object, text: Object) -> Result<Self> {
        let this = Self { parentPattern: Field::new(Default::default()), groups: Field::new(Default::default()), from: Field::new(0), to: Field::new(0), lookbehindTo: Field::new(0), text: Field::new(Default::default()), acceptMode: Field::new(0), first: Field::new(0), last: Field::new(0), oldLast: Field::new(0), lastAppendPosition: Field::new(0), locals: Field::new(Default::default()), localsPos: Field::new(Default::default()), hitEnd: Field::new(false), requireEnd: Field::new(false), transparentBounds: Field::new(false), anchoringBounds: Field::new(false), modCount: Field::new(0), namedGroups: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.acceptMode.set(0i32);
        this.first.set(-1i32);
        this.last.set(0i32);
        this.oldLast.set(-1i32);
        this.lastAppendPosition.set(0i32);
        this.transparentBounds.set(0i32);
        this.anchoringBounds.set(1i32);
        this.parentPattern.set(parent);
        this.text.set(text);
        let _t0: i32 = (parent.capturingGroupCount.get()).max(10i32);
        let mut parentGroupCount: i32 = _t0;
        let mut _arr1: Vec<i32> = vec![0i32; (parentGroupCount).wrapping_mul(2i32) as usize];
        this.groups.set(_arr1);
        let mut _arr2: Vec<i32> = vec![0i32; parent.localCount.get() as usize];
        this.locals.set(_arr2);
        let mut _arr3: Vec<Object> = Vec::with_capacity(parent.localTCNCount.get() as usize);
        this.localsPos.set(_arr3);
        let _t4 = this.reset()?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "pattern", descriptor = "()Ljava/util/regex/Pattern;", access = "public"))]
    pub fn pattern(&self) -> Result<Object> {
        let this = self;
        Ok(this.parentPattern.get())
    }

    #[cfg_attr(any(), java_method(name = "toMatchResult", descriptor = "()Ljava/util/regex/MatchResult;", access = "public"))]
    pub fn toMatchResult(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.hasMatch()?;
        let _t1 = this.minStart()?;
        let mut minStart: i32 = _t1;
        let _t2 = this.maxEnd()?;
        let _t3 = this.text.get().subSequence(minStart, _t2)?;
        let _t4 = _t3.toString()?;
        let mut capturedText: String = _t4;
        minStart = -1i32;
        /* TODO: aconst_null  */
        capturedText = _t0;
        let _t5 = this.groupCount()?;
        let _t6 = this.groups.get().clone()?;
        let _t7 = this.namedGroups()?;
        Ok(Matcher_ImmutableMatchResult::new(this.first.get(), this.last.get(), _t5, _t6, capturedText, _t7, minStart)?)
    }

    #[cfg_attr(any(), java_method(name = "minStart", descriptor = "()I", access = "private"))]
    pub fn minStart(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.text.get().length()?;
        let mut r: i32 = _t0;
        let mut group: i32 = 0i32;
        loop {
            let _t0 = this.groupCount()?;
            if group > _t0 { break; }
            let mut start: i32 = this.groups.get()[(group).wrapping_mul(2i32) as usize];
            let _t0: i32 = (r).min(start);
            r = _t0;
            group = group.wrapping_add(1i32);
        }
        Ok(r)
    }

    #[cfg_attr(any(), java_method(name = "maxEnd", descriptor = "()I", access = "private"))]
    pub fn maxEnd(&self) -> Result<i32> {
        let this = self;
        let mut r: i32 = 0i32;
        let mut group: i32 = 0i32;
        loop {
            let _t0 = this.groupCount()?;
            if group > _t0 { break; }
            let mut end: i32 = this.groups.get()[((group).wrapping_mul(2i32)).wrapping_add(1i32) as usize];
            let _t0: i32 = (r).max(end);
            r = _t0;
            group = group.wrapping_add(1i32);
        }
        Ok(r)
    }

    #[cfg_attr(any(), java_method(name = "usePattern", descriptor = "(Ljava/util/regex/Pattern;)Ljava/util/regex/Matcher;", access = "public"))]
    pub fn usePattern(&self, newPattern: Object) -> Result<Object> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        this.parentPattern.set(newPattern);
        /* TODO: aconst_null  */
        newPattern.namedGroups.set(this);
        let _t0: i32 = (newPattern.capturingGroupCount.get()).max(10i32);
        let mut parentGroupCount: i32 = _t0;
        let mut _arr1: Vec<i32> = vec![0i32; (parentGroupCount).wrapping_mul(2i32) as usize];
        this.groups.set(_arr1);
        let mut _arr2: Vec<i32> = vec![0i32; newPattern.localCount.get() as usize];
        this.locals.set(_arr2);
        let mut i: i32 = 0i32;
        loop {
            if i >= (this.groups.get().len() as i32) { break; }
            this.groups.get()[i as usize] = -1i32;
            i = i.wrapping_add(1i32);
        }
        i = 0i32;
        loop {
            if i >= (this.locals.get().len() as i32) { break; }
            this.locals.get()[i as usize] = -1i32;
            i = i.wrapping_add(1i32);
        }
        let mut _arr3: Vec<Object> = Vec::with_capacity(this.parentPattern.get().localTCNCount.get() as usize);
        this.localsPos.set(_arr3);
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "reset", descriptor = "()Ljava/util/regex/Matcher;", access = "public"))]
    // java: reset()Ljava/util/regex/Matcher;
    pub fn reset(&self) -> Result<Object> {
        let this = self;
        this.first.set(-1i32);
        this.last.set(0i32);
        this.oldLast.set(-1i32);
        let mut i: i32 = 0i32;
        loop {
            if i >= (this.groups.get().len() as i32) { break; }
            this.groups.get()[i as usize] = -1i32;
            i = i.wrapping_add(1i32);
        }
        i = 0i32;
        loop {
            if i >= (this.locals.get().len() as i32) { break; }
            this.locals.get()[i as usize] = -1i32;
            i = i.wrapping_add(1i32);
        }
        i = 0i32;
        loop {
            if i >= (this.localsPos.get().len() as i32) { break; }
            this.localsPos.get()[i as usize].clone().clear()?;
            i = i.wrapping_add(1i32);
        }
        this.lastAppendPosition.set(0i32);
        this.from.set(0i32);
        let _t0 = this.getTextLength()?;
        this.to.set(_t0);
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "reset", descriptor = "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;", access = "public"))]
    // java: reset(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;
    pub fn reset__seq(&self, input: Object) -> Result<Object> {
        let this = self;
        this.text.set(input);
        let _t0 = this.reset()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "start", descriptor = "()I", access = "public"))]
    // java: start()I
    pub fn start(&self) -> Result<i32> {
        let this = self;
        this.checkMatch()?;
        Ok(this.first.get())
    }

    #[cfg_attr(any(), java_method(name = "start", descriptor = "(I)I", access = "public"))]
    // java: start(I)I
    pub fn start__i(&self, group: i32) -> Result<i32> {
        let this = self;
        this.checkMatch()?;
        this.checkGroup(group)?;
        Ok(this.groups.get()[(group).wrapping_mul(2i32) as usize])
    }

    #[cfg_attr(any(), java_method(name = "start", descriptor = "(Ljava/lang/String;)I", access = "public"))]
    // java: start(Ljava/lang/String;)I
    pub fn start__str(&self, name: String) -> Result<i32> {
        let this = self;
        let _t0 = this.getMatchedGroupIndex(name)?;
        Ok(this.groups.get()[(_t0).wrapping_mul(2i32) as usize])
    }

    #[cfg_attr(any(), java_method(name = "end", descriptor = "()I", access = "public"))]
    // java: end()I
    pub fn end(&self) -> Result<i32> {
        let this = self;
        this.checkMatch()?;
        Ok(this.last.get())
    }

    #[cfg_attr(any(), java_method(name = "end", descriptor = "(I)I", access = "public"))]
    // java: end(I)I
    pub fn end__i(&self, group: i32) -> Result<i32> {
        let this = self;
        this.checkMatch()?;
        this.checkGroup(group)?;
        Ok(this.groups.get()[((group).wrapping_mul(2i32)).wrapping_add(1i32) as usize])
    }

    #[cfg_attr(any(), java_method(name = "end", descriptor = "(Ljava/lang/String;)I", access = "public"))]
    // java: end(Ljava/lang/String;)I
    pub fn end__str(&self, name: String) -> Result<i32> {
        let this = self;
        let _t0 = this.getMatchedGroupIndex(name)?;
        Ok(this.groups.get()[((_t0).wrapping_mul(2i32)).wrapping_add(1i32) as usize])
    }

    #[cfg_attr(any(), java_method(name = "group", descriptor = "()Ljava/lang/String;", access = "public"))]
    // java: group()Ljava/lang/String;
    pub fn group(&self) -> Result<String> {
        let this = self;
        let _t0 = this.group(0i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "group", descriptor = "(I)Ljava/lang/String;", access = "public"))]
    // java: group(I)Ljava/lang/String;
    pub fn group__i(&self, group: i32) -> Result<String> {
        let this = self;
        this.checkMatch()?;
        this.checkGroup(group)?;
        /* TODO: aconst_null  */
        return Ok(-1i32);
        let _t0 = this.getSubSequence(this.groups.get()[(group).wrapping_mul(2i32) as usize], this.groups.get()[((group).wrapping_mul(2i32)).wrapping_add(1i32) as usize])?;
        let _t1 = _t0.toString()?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "group", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public"))]
    // java: group(Ljava/lang/String;)Ljava/lang/String;
    pub fn group__str(&self, name: String) -> Result<String> {
        let this = self;
        let _t0 = this.getMatchedGroupIndex(name)?;
        let mut group: i32 = _t0;
        /* TODO: aconst_null  */
        return Ok(-1i32);
        let _t1 = this.getSubSequence(this.groups.get()[(group).wrapping_mul(2i32) as usize], this.groups.get()[((group).wrapping_mul(2i32)).wrapping_add(1i32) as usize])?;
        let _t2 = _t1.toString()?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "groupCount", descriptor = "()I", access = "public"))]
    pub fn groupCount(&self) -> Result<i32> {
        let this = self;
        Ok((this.parentPattern.get().capturingGroupCount.get()).wrapping_sub(1i32))
    }

    #[cfg_attr(any(), java_method(name = "matches", descriptor = "()Z", access = "public"))]
    pub fn matches(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.match(this.from.get(), 1i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "find", descriptor = "()Z", access = "public"))]
    // java: find()Z
    pub fn find(&self) -> Result<bool> {
        let this = self;
        let mut nextSearchIndex: i32 = this.last.get();
        nextSearchIndex = nextSearchIndex.wrapping_add(1i32);
        nextSearchIndex = this.from.get();
        let mut i: i32 = 0i32;
        loop {
            if i >= (this.groups.get().len() as i32) { break; }
            this.groups.get()[i as usize] = -1i32;
            i = i.wrapping_add(1i32);
        }
        return Ok(0i32);
        let _t0 = this.search(nextSearchIndex)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "find", descriptor = "(I)Z", access = "public"))]
    // java: find(I)Z
    pub fn find__i(&self, start: i32) -> Result<bool> {
        let this = self;
        let _t0 = this.getTextLength()?;
        let mut limit: i32 = _t0;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1 = this.reset()?;
        let _t2 = this.search(start)?;
        Ok(_t2)
    }

    #[cfg_attr(any(), java_method(name = "lookingAt", descriptor = "()Z", access = "public"))]
    pub fn lookingAt(&self) -> Result<bool> {
        let this = self;
        let _t0 = this.match(this.from.get(), 0i32)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "quoteReplacement", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public static"))]
    pub fn quoteReplacement(s: String) -> Result<String> {
        let _t0 = s.indexOf(92i32)?;
        let _t1 = s.indexOf(36i32)?;
        return Ok(s);
        let mut sb: String = String::new();
        let mut i: i32 = 0i32;
        loop {
            let _t0 = s.length()?;
            if i >= _t0 { break; }
            let _t0 = s.charAt(i)?;
            let mut c: i32 = _t0;
            sb.append(&92i32)?;
            sb.append(&c)?;
            i = i.wrapping_add(1i32);
        }
        Ok(sb)
    }

    #[cfg_attr(any(), java_method(name = "appendReplacement", descriptor = "(Ljava/lang/StringBuffer;Ljava/lang/String;)Ljava/util/regex/Matcher;", access = "public"))]
    // java: appendReplacement(Ljava/lang/StringBuffer;Ljava/lang/String;)Ljava/util/regex/Matcher;
    pub fn appendReplacement__string_str(&self, sb: Object, replacement: String) -> Result<Object> {
        let this = self;
        this.checkMatch()?;
        let _t0 = sb.length()?;
        let mut curLen: i32 = _t0;
        sb.append(&this.text.get())?;
        this.appendExpandedReplacement(sb, replacement)?;
        let mut e: i32 = todo!("stack underflow");
        sb.setLength(curLen)?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.lastAppendPosition.set(this.last.get());
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "appendReplacement", descriptor = "(Ljava/lang/StringBuilder;Ljava/lang/String;)Ljava/util/regex/Matcher;", access = "public"))]
    // java: appendReplacement(Ljava/lang/StringBuilder;Ljava/lang/String;)Ljava/util/regex/Matcher;
    pub fn appendReplacement__sb_str(&self, sb: Object, replacement: String) -> Result<Object> {
        let this = self;
        this.checkMatch()?;
        let _t0 = sb.length()?;
        let mut curLen: i32 = _t0;
        sb.append(&this.text.get())?;
        this.appendExpandedReplacement(sb, replacement)?;
        let mut e: i32 = todo!("stack underflow");
        sb.setLength(curLen)?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.lastAppendPosition.set(this.last.get());
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "appendExpandedReplacement", descriptor = "(Ljava/lang/Appendable;Ljava/lang/String;)V", access = "private"))]
    pub fn appendExpandedReplacement(&self, app: Object, replacement: String) -> Result<()> {
        let this = self;
        let mut cursor: i32 = 0i32;
        loop {
            let _t0 = replacement.length()?;
            if cursor >= _t0 { break; }
            let _t0 = replacement.charAt(cursor)?;
            let mut nextChar: i32 = _t0;
            cursor = cursor.wrapping_add(1i32);
            let _t1 = replacement.length()?;
            return Err(JvmError::Custom(String::from("athrow")));
            let _t2 = replacement.charAt(cursor)?;
            nextChar = _t2;
            let _t3 = app.append(nextChar)?;
            cursor = cursor.wrapping_add(1i32);
            cursor = cursor.wrapping_add(1i32);
            let _t4 = replacement.length()?;
            return Err(JvmError::Custom(String::from("athrow")));
            let _t5 = replacement.charAt(cursor)?;
            nextChar = _t5;
            let mut refNum: i32 = -1i32;
            cursor = cursor.wrapping_add(1i32);
            let mut begin: i32 = cursor;
            let _t6 = replacement.length()?;
            let _t7 = replacement.charAt(cursor)?;
            nextChar = _t7;
            let _t8: bool = ASCII::isLower(nextChar)?;
            let _t9: bool = ASCII::isUpper(nextChar)?;
            let _t10: bool = ASCII::isDigit(nextChar)?;
            cursor = cursor.wrapping_add(1i32);
            return Err(JvmError::Custom(String::from("athrow")));
            return Err(JvmError::Custom(String::from("athrow")));
            let _t11 = replacement.substring(begin, cursor)?;
            let mut gname: String = _t11;
            let _t12 = gname.charAt(0i32)?;
            let _t13: bool = ASCII::isDigit(_t12)?;
            String::new().append(&String::from("capturing group name {"))?;
            String::new().append(&gname)?;
            String::new().append(&String::from("} starts with digit character"))?;
            return Err(JvmError::Custom(String::from("athrow")));
            let _t14 = this.namedGroups()?;
            let _t15 = _t14.get(gname)?;
            let mut number: Object = _t15;
            String::new().append(&String::from("No group with name {"))?;
            String::new().append(&gname)?;
            String::new().append(&String::from("}"))?;
            return Err(JvmError::Custom(String::from("athrow")));
            refNum = number;
            cursor = cursor.wrapping_add(1i32);
            refNum = (nextChar).wrapping_sub(48i32);
            return Err(JvmError::Custom(String::from("athrow")));
            cursor = cursor.wrapping_add(1i32);
            begin = 0i32;
            let _t16 = replacement.length()?;
            let _t17 = replacement.charAt(cursor)?;
            gname = (_t17).wrapping_sub(48i32);
            number = ((refNum).wrapping_mul(10i32)).wrapping_add(gname);
            let _t18 = this.groupCount()?;
            begin = 1i32;
            refNum = number;
            cursor = cursor.wrapping_add(1i32);
            let _t19 = this.start(refNum)?;
            let _t20 = this.end(refNum)?;
            let _t21 = this.start(refNum)?;
            let _t22 = this.end(refNum)?;
            let _t23 = app.append(this.text.get(), _t21, _t22)?;
            let _t24 = app.append(nextChar)?;
            cursor = cursor.wrapping_add(1i32);
        }
        cursor = todo!("stack underflow");
        let _t0 = cursor.getMessage()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "appendTail", descriptor = "(Ljava/lang/StringBuffer;)Ljava/lang/StringBuffer;", access = "public"))]
    // java: appendTail(Ljava/lang/StringBuffer;)Ljava/lang/StringBuffer;
    pub fn appendTail__string(&self, sb: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.getTextLength()?;
        sb.append(&this.text.get())?;
        Ok(sb)
    }

    #[cfg_attr(any(), java_method(name = "appendTail", descriptor = "(Ljava/lang/StringBuilder;)Ljava/lang/StringBuilder;", access = "public"))]
    // java: appendTail(Ljava/lang/StringBuilder;)Ljava/lang/StringBuilder;
    pub fn appendTail__sb(&self, sb: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.getTextLength()?;
        sb.append(&this.text.get())?;
        Ok(sb)
    }

    #[cfg_attr(any(), java_method(name = "replaceAll", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public"))]
    // java: replaceAll(Ljava/lang/String;)Ljava/lang/String;
    pub fn replaceAll__str(&self, replacement: String) -> Result<String> {
        let this = self;
        let _t0 = this.reset()?;
        let _t1 = this.find()?;
        let mut result: i32 = _t1;
        let mut sb: String = String::new();
        let _t2 = this.appendReplacement(sb, replacement)?;
        let _t3 = this.find()?;
        result = _t3;
        let _t4 = this.appendTail(sb)?;
        return Ok(sb);
        let _t5 = this.text.get().toString()?;
        Ok(_t5)
    }

    #[cfg_attr(any(), java_method(name = "replaceAll", descriptor = "(Ljava/util/function/Function;)Ljava/lang/String;", access = "public"))]
    // java: replaceAll(Ljava/util/function/Function;)Ljava/lang/String;
    pub fn replaceAll__functi(&self, replacer: Object) -> Result<String> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(replacer)?;
        let _t1 = this.reset()?;
        let _t2 = this.find()?;
        let mut result: i32 = _t2;
        let mut sb: String = String::new();
        let mut ec: i32 = this.modCount.get();
        let _t3 = replacer.apply(this)?;
        let mut replacement: Object = _t3;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t4 = this.appendReplacement(sb, replacement)?;
        let _t5 = this.find()?;
        result = _t5;
        let _t6 = this.appendTail(sb)?;
        return Ok(sb);
        let _t7 = this.text.get().toString()?;
        Ok(_t7)
    }

    #[cfg_attr(any(), java_method(name = "results", descriptor = "()Ljava/util/stream/Stream;", access = "public"))]
    pub fn results(&self) -> Result<Object> {
        let this = self;
        let _t0: Object = Spliterators::spliteratorUnknownSize(Matcher_1MatchResultIterator::new(this)?, 272i32)?;
        let _t1: Object = StreamSupport::stream(_t0, 0i32)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "replaceFirst", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public"))]
    // java: replaceFirst(Ljava/lang/String;)Ljava/lang/String;
    pub fn replaceFirst__str(&self, replacement: String) -> Result<String> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0 = this.reset()?;
        let _t1 = this.find()?;
        let _t2 = this.text.get().toString()?;
        return Ok(_t2);
        let mut sb: String = String::new();
        let _t3 = this.appendReplacement(sb, replacement)?;
        let _t4 = this.appendTail(sb)?;
        Ok(sb)
    }

    #[cfg_attr(any(), java_method(name = "replaceFirst", descriptor = "(Ljava/util/function/Function;)Ljava/lang/String;", access = "public"))]
    // java: replaceFirst(Ljava/util/function/Function;)Ljava/lang/String;
    pub fn replaceFirst__functi(&self, replacer: Object) -> Result<String> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(replacer)?;
        let _t1 = this.reset()?;
        let _t2 = this.find()?;
        let _t3 = this.text.get().toString()?;
        return Ok(_t3);
        let mut sb: String = String::new();
        let mut ec: i32 = this.modCount.get();
        let _t4 = replacer.apply(this)?;
        let mut replacement: Object = _t4;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t5 = this.appendReplacement(sb, replacement)?;
        let _t6 = this.appendTail(sb)?;
        Ok(sb)
    }

    #[cfg_attr(any(), java_method(name = "region", descriptor = "(II)Ljava/util/regex/Matcher;", access = "public"))]
    pub fn region(&self, start: i32, end: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.getTextLength()?;
        return Err(JvmError::Custom(String::from("athrow")));
        let _t1 = this.getTextLength()?;
        return Err(JvmError::Custom(String::from("athrow")));
        return Err(JvmError::Custom(String::from("athrow")));
        let _t2 = this.reset()?;
        this.from.set(start);
        this.to.set(end);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "regionStart", descriptor = "()I", access = "public"))]
    pub fn regionStart(&self) -> Result<i32> {
        let this = self;
        Ok(this.from.get())
    }

    #[cfg_attr(any(), java_method(name = "regionEnd", descriptor = "()I", access = "public"))]
    pub fn regionEnd(&self) -> Result<i32> {
        let this = self;
        Ok(this.to.get())
    }

    #[cfg_attr(any(), java_method(name = "hasTransparentBounds", descriptor = "()Z", access = "public"))]
    pub fn hasTransparentBounds(&self) -> Result<bool> {
        let this = self;
        Ok(this.transparentBounds.get())
    }

    #[cfg_attr(any(), java_method(name = "useTransparentBounds", descriptor = "(Z)Ljava/util/regex/Matcher;", access = "public"))]
    pub fn useTransparentBounds(&self, b: bool) -> Result<Object> {
        let this = self;
        this.transparentBounds.set(b);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "hasAnchoringBounds", descriptor = "()Z", access = "public"))]
    pub fn hasAnchoringBounds(&self) -> Result<bool> {
        let this = self;
        Ok(this.anchoringBounds.get())
    }

    #[cfg_attr(any(), java_method(name = "useAnchoringBounds", descriptor = "(Z)Ljava/util/regex/Matcher;", access = "public"))]
    pub fn useAnchoringBounds(&self, b: bool) -> Result<Object> {
        let this = self;
        this.anchoringBounds.set(b);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let mut sb: String = String::new();
        sb.append(&String::from("java.util.regex.Matcher"))?;
        sb.append(&String::from("[pattern="))?;
        let _t0 = this.pattern()?;
        sb.append(&_t0)?;
        sb.append(&String::from("region="))?;
        let _t1 = this.regionStart()?;
        sb.append(&_t1)?;
        sb.append(&44i32)?;
        let _t2 = this.regionEnd()?;
        sb.append(&_t2)?;
        sb.append(&String::from("lastmatch="))?;
        let _t3 = this.group()?;
        let _t4 = this.group()?;
        sb.append(&_t4)?;
        sb.append(&93i32)?;
        Ok(sb)
    }

    #[cfg_attr(any(), java_method(name = "hitEnd", descriptor = "()Z", access = "public"))]
    pub fn hitEnd(&self) -> Result<bool> {
        let this = self;
        Ok(this.hitEnd.get())
    }

    #[cfg_attr(any(), java_method(name = "requireEnd", descriptor = "()Z", access = "public"))]
    pub fn requireEnd(&self) -> Result<bool> {
        let this = self;
        Ok(this.requireEnd.get())
    }

    #[cfg_attr(any(), java_method(name = "search", descriptor = "(I)Z"))]
    pub fn search(&self, from: i32) -> Result<bool> {
        let this = self;
        this.hitEnd.set(0i32);
        this.requireEnd.set(0i32);
        from = from;
        this.first.set(from);
        from.oldLast.set(this.oldLast.get());
        let mut i: i32 = 0i32;
        loop {
            if i >= (this.groups.get().len() as i32) { break; }
            this.groups.get()[i as usize] = -1i32;
            i = i.wrapping_add(1i32);
        }
        i = 0i32;
        loop {
            if i >= (this.localsPos.get().len() as i32) { break; }
            this.localsPos.get()[i as usize].clone().clear()?;
            i = i.wrapping_add(1i32);
        }
        this.acceptMode.set(0i32);
        let _t0 = this.parentPattern.get().root.get().match(this, from, this.text.get())?;
        i = _t0;
        this.first.set(-1i32);
        this.oldLast.set(this.last.get());
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        Ok(i)
    }

    #[cfg_attr(any(), java_method(name = "match", descriptor = "(II)Z"))]
    pub fn match(&self, from: i32, anchor: i32) -> Result<bool> {
        let this = self;
        this.hitEnd.set(0i32);
        this.requireEnd.set(0i32);
        from = from;
        this.first.set(from);
        from.oldLast.set(this.oldLast.get());
        let mut i: i32 = 0i32;
        loop {
            if i >= (this.groups.get().len() as i32) { break; }
            this.groups.get()[i as usize] = -1i32;
            i = i.wrapping_add(1i32);
        }
        i = 0i32;
        loop {
            if i >= (this.localsPos.get().len() as i32) { break; }
            this.localsPos.get()[i as usize].clone().clear()?;
            i = i.wrapping_add(1i32);
        }
        this.acceptMode.set(anchor);
        let _t0 = this.parentPattern.get().matchRoot.get().match(this, from, this.text.get())?;
        i = _t0;
        this.first.set(-1i32);
        this.oldLast.set(this.last.get());
        this.modCount.set((this.modCount.get()).wrapping_add(1i32));
        Ok(i)
    }

    #[cfg_attr(any(), java_method(name = "getTextLength", descriptor = "()I"))]
    pub fn getTextLength(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.text.get().length()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getSubSequence", descriptor = "(II)Ljava/lang/CharSequence;"))]
    pub fn getSubSequence(&self, beginIndex: i32, endIndex: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.text.get().subSequence(beginIndex, endIndex)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "charAt", descriptor = "(I)C"))]
    pub fn charAt(&self, i: i32) -> Result<u16> {
        let this = self;
        let _t0 = this.text.get().charAt(i)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "getMatchedGroupIndex", descriptor = "(Ljava/lang/String;)I"))]
    pub fn getMatchedGroupIndex(&self, name: String) -> Result<i32> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(name, String::from("Group name"))?;
        this.checkMatch()?;
        let _t1 = this.namedGroups()?;
        let _t2 = _t1.get(name)?;
        let mut number: Object = _t2;
        String::new().append(&String::from("No group with name <"))?;
        String::new().append(&name)?;
        String::new().append(&String::from(">"))?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(number)
    }

    #[cfg_attr(any(), java_method(name = "checkGroup", descriptor = "(I)V", access = "private"))]
    pub fn checkGroup(&self, group: i32) -> Result<()> {
        let this = self;
        let _t0 = this.groupCount()?;
        String::new().append(&String::from("No group"))?;
        String::new().append(&group)?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkMatch", descriptor = "()V", access = "private"))]
    pub fn checkMatch(&self) -> Result<()> {
        let this = self;
        let _t0 = this.hasMatch()?;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "namedGroups", descriptor = "()Ljava/util/Map;", access = "public"))]
    pub fn namedGroups(&self) -> Result<Object> {
        let this = self;
        let _t0 = this.parentPattern.get().namedGroups()?;
        this.namedGroups.set(_t0);
        return Ok(_t0);
        Ok(this.namedGroups.get())
    }

    #[cfg_attr(any(), java_method(name = "hasMatch", descriptor = "()Z", access = "public"))]
    pub fn hasMatch(&self) -> Result<bool> {
        let this = self;
        Ok(this.first.get()>=0i32)
    }
}
