#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/AbstractStringBuilder",
    super_class = "java/lang/Object",
    interfaces  = "java/lang/Appendable,java/lang/CharSequence",
    access      = "abstract",
    source      = "AbstractStringBuilder.java",
))]
pub struct AbstractStringBuilder {
    #[cfg_attr(any(), java_field(name = "value", descriptor = "[B"))]
    pub value: Field<Vec<i8>>,
    #[cfg_attr(any(), java_field(name = "coder", descriptor = "B"))]
    pub coder: Field<i8>,
    #[cfg_attr(any(), java_field(name = "maybeLatin1", descriptor = "Z"))]
    pub maybeLatin1: Field<bool>,
    #[cfg_attr(any(), java_field(name = "count", descriptor = "I"))]
    pub count: Field<i32>,
}

impl AbstractStringBuilder {
    // java: <init>()V
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), maybeLatin1: Field::new(false), count: Field::new(0) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.value.set(AbstractStringBuilder::EMPTYVALUE());
        Ok(this)
    }

    // java: <init>(I)V
    // java: <init>(I)V
    pub fn new__i(capacity: i32) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), maybeLatin1: Field::new(false), count: Field::new(0) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        let mut _arr0: Vec<i8> = vec![0i8; capacity as usize];
        this.value.set(_arr0);
        this.coder.set(0i32);
        let _t1: Vec<i8> = StringUTF16::newBytesFor(capacity)?;
        this.value.set(_t1);
        this.coder.set(1i32);
        Ok(this)
    }

    // java: <init>(Ljava/lang/String;)V
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(str: String) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), maybeLatin1: Field::new(false), count: Field::new(0) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        let _t0 = str.length()?;
        let mut length: i32 = _t0;
        let mut capacity: i32 = 2147483647i32;
        let _t1 = str.coder()?;
        let mut initCoder: i32 = _t1;
        this.coder.set(initCoder);
        let mut _arr2: Vec<i8> = vec![0i8; capacity as usize];
        let _t3: Vec<i8> = StringUTF16::newBytesFor(capacity)?;
        _arr2.value.set(_t3);
        let _t4 = this.append(str)?;
        Ok(this)
    }

    // java: <init>(Ljava/lang/CharSequence;)V
    // java: <init>(Ljava/lang/CharSequence;)V
    pub fn new__seq(seq: Object) -> Result<Self> {
        let this = Self { value: Field::new(Default::default()), coder: Field::new(Default::default()), maybeLatin1: Field::new(false), count: Field::new(0) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        let _t0 = seq.length()?;
        let mut length: i32 = _t0;
        String::new().append(&String::from("Negative length:"))?;
        String::new().append(&length)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut capacity: i32 = 2147483647i32;
        let mut asb: Object = seq;
        let _t1 = asb.getCoder()?;
        let mut initCoder: i32 = _t1;
        this.maybeLatin1.set((this.maybeLatin1.get()|asb.maybeLatin1.get()));
        let mut s: Object = seq;
        let _t2 = s.coder()?;
        initCoder = _t2;
        initCoder = 0i32;
        initCoder = 1i32;
        this.coder.set(initCoder);
        let mut _arr3: Vec<i8> = vec![0i8; capacity as usize];
        let _t4: Vec<i8> = StringUTF16::newBytesFor(capacity)?;
        _arr3.value.set(_t4);
        let _t5 = this.append(seq)?;
        Ok(this)
    }

    // java: compareTo(Ljava/lang/AbstractStringBuilder;)I
    pub fn compareTo(&self, another: Object) -> Result<i32> {
        let this = self;
        return Ok(0i32);
        let mut val1: Vec<i8> = this.value.get();
        let mut val2: Vec<i8> = another.value.get();
        let mut count1: i32 = this.count.get();
        let mut count2: i32 = another.count.get();
        let _t0 = this.isLatin1()?;
        let _t1: i32 = StringLatin1::compareTo(&val1, &val2, count1, count2)?;
        let _t2: i32 = StringUTF16::compareTo(&val1, &val2, count1, count2)?;
        return Ok(_t2);
        let _t3 = this.isLatin1()?;
        let _t4: i32 = StringLatin1::compareToUTF16(&val1, &val2, count1, count2)?;
        let _t5: i32 = StringUTF16::compareToLatin1(&val1, &val2, count1, count2)?;
        Ok(_t5)
    }

    // java: length()I
    pub fn length(&self) -> Result<i32> {
        let this = self;
        Ok(this.count.get())
    }

    // java: capacity()I
    pub fn capacity(&self) -> Result<i32> {
        let this = self;
        Ok(((this.value.get().len() as i32)>>((this.coder.get()&0x1f))))
    }

    // java: ensureCapacity(I)V
    pub fn ensureCapacity(&self, minimumCapacity: i32) -> Result<()> {
        let this = self;
        this.ensureCapacityInternal(minimumCapacity)?;
        Ok(())
    }

    // java: ensureCapacityInternal(I)V
    pub fn ensureCapacityInternal(&self, minimumCapacity: i32) -> Result<()> {
        let this = self;
        let mut oldCapacity: i32 = ((this.value.get().len() as i32)>>((this.coder.get()&0x1f)));
        let _t0 = this.newCapacity(minimumCapacity)?;
        let _t1: Vec<i8> = Arrays::copyOf(&this.value.get(), (_t0<<(this.coder.get()&0x1f)))?;
        this.value.set(_t1);
        Ok(())
    }

    // java: newCapacity(I)I
    pub fn newCapacity(&self, minCapacity: i32) -> Result<i32> {
        let this = self;
        let mut oldLength: i32 = (this.value.get().len() as i32);
        let mut newLength: i32 = (minCapacity<<(this.coder.get()&0x1f));
        let mut growth: i32 = (newLength).wrapping_sub(oldLength);
        let _t0: i32 = ArraysSupport::newLength(oldLength, growth, (oldLength).wrapping_add((2i32<<(this.coder.get()&0x1f))))?;
        let mut length: i32 = _t0;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok((length>>((this.coder.get()&0x1f))))
    }

    // java: inflate()V
    pub fn inflate(&self) -> Result<()> {
        let this = self;
        let _t0 = this.isLatin1()?;
        return Ok(());
        let _t1: Vec<i8> = StringUTF16::newBytesFor((this.value.get().len() as i32))?;
        let mut buf: Vec<i8> = _t1;
        StringLatin1::inflate(&this.value.get(), 0i32, &buf, 0i32, this.count.get())?;
        this.value.set(buf);
        this.coder.set(1i32);
        Ok(())
    }

    // java: trimToSize()V
    pub fn trimToSize(&self) -> Result<()> {
        let this = self;
        let mut length: i32 = (this.count.get()<<(this.coder.get()&0x1f));
        let _t0: Vec<i8> = Arrays::copyOf(&this.value.get(), length)?;
        this.value.set(_t0);
        Ok(())
    }

    // java: setLength(I)V
    pub fn setLength(&self, newLength: i32) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.ensureCapacityInternal(newLength)?;
        let _t0 = this.isLatin1()?;
        StringLatin1::fillNull(&this.value.get(), this.count.get(), newLength)?;
        StringUTF16::fillNull(&this.value.get(), this.count.get(), newLength)?;
        this.maybeLatin1.set(1i32);
        this.count.set(newLength);
        Ok(())
    }

    // java: charAt(I)C
    pub fn charAt(&self, index: i32) -> Result<u16> {
        let this = self;
        let mut coder: i32 = this.coder.get();
        let mut value: Vec<i8> = this.value.get();
        let _t0: i32 = (this.count.get()).min(((value.len() as i32)>>((coder&0x1f))));
        let mut count: i32 = _t0;
        String::checkIndex(index, count)?;
        /* TODO: i2c  */
        return Ok((value[index as usize]&255i32));
        let _t1: u16 = StringUTF16::getChar(&value, index)?;
        Ok(_t1)
    }

    // java: codePointAt(I)I
    pub fn codePointAt(&self, index: i32) -> Result<i32> {
        let this = self;
        let mut count: i32 = this.count.get();
        let mut value: Vec<i8> = this.value.get();
        String::checkIndex(index, count)?;
        let _t0 = this.isLatin1()?;
        return Ok((value[index as usize]&255i32));
        let _t1: i32 = StringUTF16::codePointAtSB(&value, index, count)?;
        Ok(_t1)
    }

    // java: codePointBefore(I)I
    pub fn codePointBefore(&self, index: i32) -> Result<i32> {
        let this = self;
        let mut value: Vec<i8> = this.value.get();
        let mut i: i32 = (index).wrapping_sub(1i32);
        String::checkIndex(i, this.count.get())?;
        let _t0 = this.isLatin1()?;
        return Ok((value[i as usize]&255i32));
        let _t1: i32 = StringUTF16::codePointBeforeSB(&value, index)?;
        Ok(_t1)
    }

    // java: codePointCount(II)I
    pub fn codePointCount(&self, beginIndex: i32, endIndex: i32) -> Result<i32> {
        let this = self;
        let _t0 = this.length()?;
        /* TODO: aconst_null  */
        let _t1: i32 = Preconditions::checkFromToIndex(todo!("stack underflow"), beginIndex, endIndex, _t0)?;
        let _t2 = this.isLatin1()?;
        return Ok((endIndex).wrapping_sub(beginIndex));
        let _t3: i32 = StringUTF16::codePointCountSB(&this.value.get(), beginIndex, endIndex)?;
        Ok(_t3)
    }

    // java: offsetByCodePoints(II)I
    pub fn offsetByCodePoints(&self, index: i32, codePointOffset: i32) -> Result<i32> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: i32 = Character::offsetByCodePoints(this, index, codePointOffset)?;
        Ok(_t0)
    }

    // java: getChars(II[CI)V
    pub fn getChars(&self, srcBegin: i32, srcEnd: i32, dst: Vec<u16>, dstBegin: i32) -> Result<()> {
        let this = self;
        let _t0: i32 = Preconditions::checkFromToIndex(srcBegin, srcEnd, this.count.get(), Preconditions::SIOOBE_FORMATTER())?;
        let mut n: i32 = (srcEnd).wrapping_sub(srcBegin);
        let _t1: i32 = Preconditions::checkFromToIndex(dstBegin, (dstBegin).wrapping_add(n), (dst.len() as i32), Preconditions::IOOBE_FORMATTER())?;
        let _t2 = this.isLatin1()?;
        StringLatin1::getChars(&this.value.get(), srcBegin, srcEnd, &dst, dstBegin)?;
        StringUTF16::getChars(&this.value.get(), srcBegin, srcEnd, &dst, dstBegin)?;
        Ok(())
    }

    // java: setCharAt(IC)V
    pub fn setCharAt(&self, index: i32, ch: u16) -> Result<()> {
        let this = self;
        String::checkIndex(index, this.count.get())?;
        let _t0 = this.isLatin1()?;
        let _t1: bool = StringLatin1::canEncode(ch)?;
        /* TODO: i2b  */
        this.value.get()[index as usize] = ch;
        let _t2 = this.isLatin1()?;
        this.inflate()?;
        StringUTF16::putCharSB(&this.value.get(), index, ch)?;
        this.maybeLatin1.set(1i32);
        Ok(())
    }

    // java: append(Ljava/lang/Object;)Ljava/lang/AbstractStringBuilder;
    // java: append(Ljava/lang/Object;)Ljava/lang/AbstractStringBuilder;
    pub fn append__obj(&self, obj: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.append(String::from_owned(format!("{}", obj)))?;
        Ok(_t0)
    }

    // java: append(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;
    // java: append(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;
    pub fn append__str(&self, str: String) -> Result<Object> {
        let this = self;
        let _t0 = this.appendNull()?;
        return Ok(_t0);
        let _t1 = str.length()?;
        let mut len: i32 = _t1;
        this.ensureCapacityInternal((this.count.get()).wrapping_add(len))?;
        this.putStringAt(this.count.get(), str)?;
        this.count.set((this.count.get()).wrapping_add(len));
        Ok(this)
    }

    // java: append(Ljava/lang/StringBuffer;)Ljava/lang/AbstractStringBuilder;
    // java: append(Ljava/lang/StringBuffer;)Ljava/lang/AbstractStringBuilder;
    pub fn append__string(&self, sb: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.append(sb)?;
        Ok(_t0)
    }

    // java: append(Ljava/lang/AbstractStringBuilder;)Ljava/lang/AbstractStringBuilder;
    // java: append(Ljava/lang/AbstractStringBuilder;)Ljava/lang/AbstractStringBuilder;
    pub fn append__abstra(&self, asb: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.appendNull()?;
        return Ok(_t0);
        let _t1 = asb.length()?;
        let mut len: i32 = _t1;
        this.ensureCapacityInternal((this.count.get()).wrapping_add(len))?;
        this.inflateIfNeededFor(asb)?;
        asb.getBytes(this.value.get(), this.count.get(), this.coder.get())?;
        this.count.set((this.count.get()).wrapping_add(len));
        this.maybeLatin1.set((this.maybeLatin1.get()|asb.maybeLatin1.get()));
        Ok(this)
    }

    // java: append(Ljava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;
    // java: append(Ljava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;
    pub fn append__seq(&self, s: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.appendNull()?;
        return Ok(_t0);
        let _t1 = this.append(s)?;
        return Ok(_t1);
        let _t2 = this.append(s)?;
        return Ok(_t2);
        let _t3 = s.length()?;
        let _t4 = this.append(s, 0i32, _t3)?;
        Ok(_t4)
    }

    // java: appendNull()Ljava/lang/AbstractStringBuilder;
    pub fn appendNull(&self) -> Result<Object> {
        let this = self;
        this.ensureCapacityInternal((this.count.get()).wrapping_add(4i32))?;
        let mut count: i32 = this.count.get();
        let mut val: Vec<i8> = this.value.get();
        let _t0 = this.isLatin1()?;
        count = count.wrapping_add(1i32);
        val[count as usize] = 110i32;
        count = count.wrapping_add(1i32);
        val[count as usize] = 117i32;
        count = count.wrapping_add(1i32);
        val[count as usize] = 108i32;
        count = count.wrapping_add(1i32);
        val[count as usize] = 108i32;
        let _t1: i32 = StringUTF16::putCharsAt(&val, count, 110i32, 117i32, 108i32, 108i32)?;
        count = _t1;
        this.count.set(count);
        Ok(this)
    }

    // java: append(Ljava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;
    // java: append(Ljava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;
    pub fn append__seq_i_i(&self, s: Object, start: i32, end: i32) -> Result<Object> {
        let this = self;
        s = String::from("null");
        let _t0 = s.length()?;
        let _t1: i32 = Preconditions::checkFromToIndex(start, end, _t0, Preconditions::IOOBE_FORMATTER())?;
        let mut len: i32 = (end).wrapping_sub(start);
        this.ensureCapacityInternal((this.count.get()).wrapping_add(len))?;
        this.appendChars(s, start, end)?;
        this.appendChars(s, start, end)?;
        Ok(this)
    }

    // java: append([C)Ljava/lang/AbstractStringBuilder;
    // java: append([C)Ljava/lang/AbstractStringBuilder;
    pub fn append__arr_c(&self, str: Vec<u16>) -> Result<Object> {
        let this = self;
        let mut len: i32 = (str.len() as i32);
        this.ensureCapacityInternal((this.count.get()).wrapping_add(len))?;
        this.appendChars(str, 0i32, len)?;
        Ok(this)
    }

    // java: append([CII)Ljava/lang/AbstractStringBuilder;
    // java: append([CII)Ljava/lang/AbstractStringBuilder;
    pub fn append__arr_c_i_i(&self, str: Vec<u16>, offset: i32, len: i32) -> Result<Object> {
        let this = self;
        let mut end: i32 = (offset).wrapping_add(len);
        let _t0: i32 = Preconditions::checkFromToIndex(offset, end, (str.len() as i32), Preconditions::IOOBE_FORMATTER())?;
        this.ensureCapacityInternal((this.count.get()).wrapping_add(len))?;
        this.appendChars(str, offset, end)?;
        Ok(this)
    }

    // java: append(Z)Ljava/lang/AbstractStringBuilder;
    // java: append(Z)Ljava/lang/AbstractStringBuilder;
    pub fn append__z(&self, b: bool) -> Result<Object> {
        let this = self;
        this.ensureCapacityInternal((this.count.get()).wrapping_add(b==0i32))?;
        let mut count: i32 = this.count.get();
        let mut val: Vec<i8> = this.value.get();
        let _t0 = this.isLatin1()?;
        count = count.wrapping_add(1i32);
        val[count as usize] = 116i32;
        count = count.wrapping_add(1i32);
        val[count as usize] = 114i32;
        count = count.wrapping_add(1i32);
        val[count as usize] = 117i32;
        count = count.wrapping_add(1i32);
        val[count as usize] = 101i32;
        count = count.wrapping_add(1i32);
        val[count as usize] = 102i32;
        count = count.wrapping_add(1i32);
        val[count as usize] = 97i32;
        count = count.wrapping_add(1i32);
        val[count as usize] = 108i32;
        count = count.wrapping_add(1i32);
        val[count as usize] = 115i32;
        count = count.wrapping_add(1i32);
        val[count as usize] = 101i32;
        let _t1: i32 = StringUTF16::putCharsAt(&val, count, 116i32, 114i32, 117i32, 101i32)?;
        count = _t1;
        let _t2: i32 = StringUTF16::putCharsAt(&val, count, 102i32, 97i32, 108i32, 115i32, 101i32)?;
        count = _t2;
        this.count.set(count);
        Ok(this)
    }

    // java: append(C)Ljava/lang/AbstractStringBuilder;
    // java: append(C)Ljava/lang/AbstractStringBuilder;
    pub fn append__c(&self, c: u16) -> Result<Object> {
        let this = self;
        this.ensureCapacityInternal((this.count.get()).wrapping_add(1i32))?;
        let _t0 = this.isLatin1()?;
        let _t1: bool = StringLatin1::canEncode(c)?;
        this.count.set((this.count.get()).wrapping_add(1i32));
        /* TODO: i2b  */
        this.value.get()[this.count.get() as usize] = c;
        let _t2 = this.isLatin1()?;
        this.inflate()?;
        this.count.set((this.count.get()).wrapping_add(1i32));
        StringUTF16::putCharSB(&this.value.get(), this.count.get(), c)?;
        Ok(this)
    }

    // java: append(I)Ljava/lang/AbstractStringBuilder;
    // java: append(I)Ljava/lang/AbstractStringBuilder;
    pub fn append__i(&self, i: i32) -> Result<Object> {
        let this = self;
        let mut count: i32 = this.count.get();
        let _t0: i32 = Integer::stringSize(i)?;
        let mut spaceNeeded: i32 = (count).wrapping_add(_t0);
        this.ensureCapacityInternal(spaceNeeded)?;
        let _t1 = this.isLatin1()?;
        let _t2: i32 = Integer::getChars(i, spaceNeeded, &this.value.get())?;
        let _t3: i32 = StringUTF16::getChars(i, count, spaceNeeded, &this.value.get())?;
        this.count.set(spaceNeeded);
        Ok(this)
    }

    // java: append(J)Ljava/lang/AbstractStringBuilder;
    // java: append(J)Ljava/lang/AbstractStringBuilder;
    pub fn append__l(&self, l: i64) -> Result<Object> {
        let this = self;
        let mut count: i32 = this.count.get();
        let _t0: i32 = Long::stringSize(l)?;
        let mut spaceNeeded: i32 = (count).wrapping_add(_t0);
        this.ensureCapacityInternal(spaceNeeded)?;
        let _t1 = this.isLatin1()?;
        let _t2: i32 = Long::getChars(l, spaceNeeded, &this.value.get())?;
        let _t3: i32 = StringUTF16::getChars(l, count, spaceNeeded, &this.value.get())?;
        this.count.set(spaceNeeded);
        Ok(this)
    }

    // java: append(F)Ljava/lang/AbstractStringBuilder;
    // java: append(F)Ljava/lang/AbstractStringBuilder;
    pub fn append__f(&self, f: f32) -> Result<Object> {
        let this = self;
        let _t0: Object = FloatToDecimal::appendTo(f, this)?;
        let mut e: i32 = todo!("stack underflow");
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(this)
    }

    // java: append(D)Ljava/lang/AbstractStringBuilder;
    // java: append(D)Ljava/lang/AbstractStringBuilder;
    pub fn append__d(&self, d: f64) -> Result<Object> {
        let this = self;
        let _t0: Object = DoubleToDecimal::appendTo(d, this)?;
        let mut e: i32 = todo!("stack underflow");
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(this)
    }

    // java: delete(II)Ljava/lang/AbstractStringBuilder;
    pub fn delete(&self, start: i32, end: i32) -> Result<Object> {
        let this = self;
        let mut count: i32 = this.count.get();
        end = count;
        let _t0: i32 = Preconditions::checkFromToIndex(start, end, count, Preconditions::SIOOBE_FORMATTER())?;
        let mut len: i32 = (end).wrapping_sub(start);
        this.shift(end, (len).wrapping_neg())?;
        this.count.set((count).wrapping_sub(len));
        this.maybeLatin1.set(1i32);
        Ok(this)
    }

    // java: appendCodePoint(I)Ljava/lang/AbstractStringBuilder;
    pub fn appendCodePoint(&self, codePoint: i32) -> Result<Object> {
        let this = self;
        let _t0: bool = Character::isBmpCodePoint(codePoint)?;
        /* TODO: i2c  */
        let _t1 = this.append(codePoint)?;
        return Ok(_t1);
        let _t2: Vec<u16> = Character::toChars(codePoint)?;
        let _t3 = this.append(_t2)?;
        Ok(_t3)
    }

    // java: deleteCharAt(I)Ljava/lang/AbstractStringBuilder;
    pub fn deleteCharAt(&self, index: i32) -> Result<Object> {
        let this = self;
        String::checkIndex(index, this.count.get())?;
        this.shift((index).wrapping_add(1i32), -1i32)?;
        this.count.set((this.count.get()).wrapping_sub(1i32));
        this.maybeLatin1.set(1i32);
        Ok(this)
    }

    // java: replace(IILjava/lang/String;)Ljava/lang/AbstractStringBuilder;
    pub fn replace(&self, start: i32, end: i32, str: String) -> Result<Object> {
        let this = self;
        let mut count: i32 = this.count.get();
        end = count;
        let _t0: i32 = Preconditions::checkFromToIndex(start, end, count, Preconditions::SIOOBE_FORMATTER())?;
        let _t1 = str.length()?;
        let mut len: i32 = _t1;
        let mut newCount: i32 = ((count).wrapping_add(len)).wrapping_sub((end).wrapping_sub(start));
        this.ensureCapacityInternal(newCount)?;
        this.shift(end, (newCount).wrapping_sub(count))?;
        this.count.set(newCount);
        this.putStringAt(start, str)?;
        this.maybeLatin1.set(1i32);
        Ok(this)
    }

    // java: substring(I)Ljava/lang/String;
    // java: substring(I)Ljava/lang/String;
    pub fn substring__i(&self, start: i32) -> Result<String> {
        let this = self;
        let _t0 = this.substring(start, this.count.get())?;
        Ok(_t0)
    }

    // java: subSequence(II)Ljava/lang/CharSequence;
    pub fn subSequence(&self, start: i32, end: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.substring(start, end)?;
        Ok(_t0)
    }

    // java: substring(II)Ljava/lang/String;
    // java: substring(II)Ljava/lang/String;
    pub fn substring__i_i(&self, start: i32, end: i32) -> Result<String> {
        let this = self;
        let _t0: i32 = Preconditions::checkFromToIndex(start, end, this.count.get(), Preconditions::SIOOBE_FORMATTER())?;
        let _t1 = this.isLatin1()?;
        let _t2: String = StringLatin1::newString(&this.value.get(), start, (end).wrapping_sub(start))?;
        return Ok(_t2);
        let _t3: String = StringUTF16::newString(&this.value.get(), start, (end).wrapping_sub(start))?;
        Ok(_t3)
    }

    // java: shift(II)V
    pub fn shift(&self, offset: i32, n: i32) -> Result<()> {
        let this = self;
        System::arraycopy(&this.value.get(), (offset<<(this.coder.get()&0x1f)), &this.value.get(), ((offset).wrapping_add(n)<<(this.coder.get()&0x1f)), ((this.count.get()).wrapping_sub(offset)<<(this.coder.get()&0x1f)))?;
        Ok(())
    }

    // java: insert(I[CII)Ljava/lang/AbstractStringBuilder;
    // java: insert(I[CII)Ljava/lang/AbstractStringBuilder;
    pub fn insert__i_arr_c_i_i(&self, index: i32, str: Vec<u16>, offset: i32, len: i32) -> Result<Object> {
        let this = self;
        String::checkOffset(index, this.count.get())?;
        let _t0: i32 = Preconditions::checkFromToIndex(offset, (offset).wrapping_add(len), (str.len() as i32), Preconditions::SIOOBE_FORMATTER())?;
        this.ensureCapacityInternal((this.count.get()).wrapping_add(len))?;
        this.shift(index, len)?;
        this.count.set((this.count.get()).wrapping_add(len));
        this.putCharsAt(index, str, offset, (offset).wrapping_add(len))?;
        Ok(this)
    }

    // java: insert(ILjava/lang/Object;)Ljava/lang/AbstractStringBuilder;
    // java: insert(ILjava/lang/Object;)Ljava/lang/AbstractStringBuilder;
    pub fn insert__i_obj(&self, offset: i32, obj: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.insert(offset, String::from_owned(format!("{}", obj)))?;
        Ok(_t0)
    }

    // java: insert(ILjava/lang/String;)Ljava/lang/AbstractStringBuilder;
    // java: insert(ILjava/lang/String;)Ljava/lang/AbstractStringBuilder;
    pub fn insert__i_str(&self, offset: i32, str: String) -> Result<Object> {
        let this = self;
        String::checkOffset(offset, this.count.get())?;
        str = String::from("null");
        let _t0 = str.length()?;
        let mut len: i32 = _t0;
        this.ensureCapacityInternal((this.count.get()).wrapping_add(len))?;
        this.shift(offset, len)?;
        this.count.set((this.count.get()).wrapping_add(len));
        this.putStringAt(offset, str)?;
        Ok(this)
    }

    // java: insert(I[C)Ljava/lang/AbstractStringBuilder;
    // java: insert(I[C)Ljava/lang/AbstractStringBuilder;
    pub fn insert__i_arr_c(&self, offset: i32, str: Vec<u16>) -> Result<Object> {
        let this = self;
        String::checkOffset(offset, this.count.get())?;
        let mut len: i32 = (str.len() as i32);
        this.ensureCapacityInternal((this.count.get()).wrapping_add(len))?;
        this.shift(offset, len)?;
        this.count.set((this.count.get()).wrapping_add(len));
        this.putCharsAt(offset, str, 0i32, len)?;
        Ok(this)
    }

    // java: insert(ILjava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;
    // java: insert(ILjava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;
    pub fn insert__i_seq(&self, dstOffset: i32, s: Object) -> Result<Object> {
        let this = self;
        s = String::from("null");
        let _t0 = s.length()?;
        let _t1 = this.insert(dstOffset, s, 0i32, _t0)?;
        Ok(_t1)
    }

    // java: insert(ILjava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;
    // java: insert(ILjava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;
    pub fn insert__i_seq_i_i(&self, dstOffset: i32, s: Object, start: i32, end: i32) -> Result<Object> {
        let this = self;
        s = String::from("null");
        String::checkOffset(dstOffset, this.count.get())?;
        let _t0 = s.length()?;
        let _t1: i32 = Preconditions::checkFromToIndex(start, end, _t0, Preconditions::IOOBE_FORMATTER())?;
        let mut len: i32 = (end).wrapping_sub(start);
        this.ensureCapacityInternal((this.count.get()).wrapping_add(len))?;
        this.shift(dstOffset, len)?;
        this.count.set((this.count.get()).wrapping_add(len));
        let mut str: Object = s;
        let _t2 = str.length()?;
        this.putStringAt(dstOffset, str)?;
        this.putCharsAt(dstOffset, s, start, end)?;
        Ok(this)
    }

    // java: insert(IZ)Ljava/lang/AbstractStringBuilder;
    // java: insert(IZ)Ljava/lang/AbstractStringBuilder;
    pub fn insert__i_z(&self, offset: i32, b: bool) -> Result<Object> {
        let this = self;
        let _t0 = this.insert(offset, String::from_owned(format!("{}", b)))?;
        Ok(_t0)
    }

    // java: insert(IC)Ljava/lang/AbstractStringBuilder;
    // java: insert(IC)Ljava/lang/AbstractStringBuilder;
    pub fn insert__i_c(&self, offset: i32, c: u16) -> Result<Object> {
        let this = self;
        String::checkOffset(offset, this.count.get())?;
        this.ensureCapacityInternal((this.count.get()).wrapping_add(1i32))?;
        this.shift(offset, 1i32)?;
        this.count.set((this.count.get()).wrapping_add(1i32));
        let _t0 = this.isLatin1()?;
        let _t1: bool = StringLatin1::canEncode(c)?;
        /* TODO: i2b  */
        this.value.get()[offset as usize] = c;
        let _t2 = this.isLatin1()?;
        this.inflate()?;
        StringUTF16::putCharSB(&this.value.get(), offset, c)?;
        Ok(this)
    }

    // java: insert(II)Ljava/lang/AbstractStringBuilder;
    // java: insert(II)Ljava/lang/AbstractStringBuilder;
    pub fn insert__i_i(&self, offset: i32, i: i32) -> Result<Object> {
        let this = self;
        let _t0 = this.insert(offset, String::from_owned(format!("{}", i)))?;
        Ok(_t0)
    }

    // java: insert(IJ)Ljava/lang/AbstractStringBuilder;
    // java: insert(IJ)Ljava/lang/AbstractStringBuilder;
    pub fn insert__i_l(&self, offset: i32, l: i64) -> Result<Object> {
        let this = self;
        let _t0 = this.insert(offset, String::from_owned(format!("{}", l)))?;
        Ok(_t0)
    }

    // java: insert(IF)Ljava/lang/AbstractStringBuilder;
    // java: insert(IF)Ljava/lang/AbstractStringBuilder;
    pub fn insert__i_f(&self, offset: i32, f: f32) -> Result<Object> {
        let this = self;
        let _t0 = this.insert(offset, String::from_owned(format!("{}", f)))?;
        Ok(_t0)
    }

    // java: insert(ID)Ljava/lang/AbstractStringBuilder;
    // java: insert(ID)Ljava/lang/AbstractStringBuilder;
    pub fn insert__i_d(&self, offset: i32, d: f64) -> Result<Object> {
        let this = self;
        let _t0 = this.insert(offset, String::from_owned(format!("{}", d)))?;
        Ok(_t0)
    }

    // java: indexOf(Ljava/lang/String;)I
    // java: indexOf(Ljava/lang/String;)I
    pub fn indexOf__str(&self, str: String) -> Result<i32> {
        let this = self;
        let _t0 = this.indexOf(str, 0i32)?;
        Ok(_t0)
    }

    // java: indexOf(Ljava/lang/String;I)I
    // java: indexOf(Ljava/lang/String;I)I
    pub fn indexOf__str_i(&self, str: String, fromIndex: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = String::indexOf(&this.value.get(), this.coder.get(), this.count.get(), str, fromIndex)?;
        Ok(_t0)
    }

    // java: lastIndexOf(Ljava/lang/String;)I
    // java: lastIndexOf(Ljava/lang/String;)I
    pub fn lastIndexOf__str(&self, str: String) -> Result<i32> {
        let this = self;
        let _t0 = this.lastIndexOf(str, this.count.get())?;
        Ok(_t0)
    }

    // java: lastIndexOf(Ljava/lang/String;I)I
    // java: lastIndexOf(Ljava/lang/String;I)I
    pub fn lastIndexOf__str_i(&self, str: String, fromIndex: i32) -> Result<i32> {
        let this = self;
        let _t0: i32 = String::lastIndexOf(&this.value.get(), this.coder.get(), this.count.get(), str, fromIndex)?;
        Ok(_t0)
    }

    // java: reverse()Ljava/lang/AbstractStringBuilder;
    pub fn reverse(&self) -> Result<Object> {
        let this = self;
        let mut val: Vec<i8> = this.value.get();
        let mut count: i32 = this.count.get();
        let mut n: i32 = (count).wrapping_sub(1i32);
        let _t0 = this.isLatin1()?;
        let mut j: i32 = ((n).wrapping_sub(1i32)>>((1i32&0x1f)));
        loop {
            if j<0i32 { break; }
            let mut k: i32 = (n).wrapping_sub(j);
            let mut cj: i32 = val[j as usize];
            val[j as usize] = val[k as usize];
            val[k as usize] = cj;
            j = j.wrapping_sub(1i32);
        }
        StringUTF16::reverse(&val, count)?;
        Ok(this)
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        todo!("abstract java/lang/AbstractStringBuilder.toString")
    }

    // java: chars()Ljava/util/stream/IntStream;
    pub fn chars(&self) -> Result<Object> {
        let this = self;
        /* TODO: invokedynamic 362 */
        let _t0: Object = StreamSupport::intStream(this, 16464i32, 0i32)?;
        Ok(_t0)
    }

    // java: codePoints()Ljava/util/stream/IntStream;
    pub fn codePoints(&self) -> Result<Object> {
        let this = self;
        /* TODO: invokedynamic 374 */
        let _t0: Object = StreamSupport::intStream(this, 16i32, 0i32)?;
        Ok(_t0)
    }

    // java: getValue()[B
    pub fn getValue(&self) -> Result<Vec<i8>> {
        let this = self;
        Ok(this.value.get())
    }

    // java: getBytes([BIB)V
    pub fn getBytes(&self, dst: Vec<i8>, dstBegin: i32, coder: i8) -> Result<()> {
        let this = self;
        System::arraycopy(&this.value.get(), 0i32, &dst, (dstBegin<<(coder&0x1f)), (this.count.get()<<(coder&0x1f)))?;
        StringLatin1::inflate(&this.value.get(), 0i32, &dst, dstBegin, this.count.get())?;
        Ok(())
    }

    // java: initBytes([CII)V
    pub fn initBytes(&self, value: Vec<u16>, off: i32, len: i32) -> Result<()> {
        let this = self;
        let _t0: Vec<i8> = StringUTF16::compress(&value, off, len)?;
        let mut val: Vec<i8> = _t0;
        let _t1: i8 = StringUTF16::coderFromArrayLen(&val, len)?;
        this.coder.set(_t1);
        this.value.set(val);
        return Ok(());
        this.coder.set(1i32);
        let _t2: Vec<i8> = StringUTF16::toBytes(&value, off, len)?;
        this.value.set(_t2);
        Ok(())
    }

    // java: getCoder()B
    pub fn getCoder(&self) -> Result<i8> {
        let this = self;
        Ok(1i32)
    }

    // java: isLatin1()Z
    pub fn isLatin1(&self) -> Result<bool> {
        let this = self;
        Ok(this.coder.get()==0i32)
    }

    // java: putCharsAt(I[CII)V
    // java: putCharsAt(I[CII)V
    pub fn putCharsAt__i_arr_c_i_i(&self, index: i32, s: Vec<u16>, off: i32, end: i32) -> Result<()> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let mut val: Vec<i8> = this.value.get();
        let mut i: i32 = off;
        let mut j: i32 = index;
        loop {
            if i >= end { break; }
            let mut c: i32 = s[i as usize];
            let _t0: bool = StringLatin1::canEncode(c)?;
            j = j.wrapping_add(1i32);
            /* TODO: i2b  */
            val[j as usize] = c;
            this.inflate()?;
            StringUTF16::putCharsSB(&this.value.get(), j, &s, i, end)?;
            return Ok(());
            i = i.wrapping_add(1i32);
        }
        StringUTF16::putCharsSB(&this.value.get(), index, &s, off, end)?;
        Ok(())
    }

    // java: putCharsAt(ILjava/lang/CharSequence;II)V
    // java: putCharsAt(ILjava/lang/CharSequence;II)V
    pub fn putCharsAt__i_seq_i_i(&self, index: i32, s: Object, off: i32, end: i32) -> Result<()> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let mut val: Vec<i8> = this.value.get();
        let mut i: i32 = off;
        let mut j: i32 = index;
        loop {
            if i >= end { break; }
            let _t0 = s.charAt(i)?;
            let mut c: i32 = _t0;
            let _t1: bool = StringLatin1::canEncode(c)?;
            j = j.wrapping_add(1i32);
            /* TODO: i2b  */
            val[j as usize] = c;
            this.inflate()?;
            j = j.wrapping_add(1i32);
            StringUTF16::putCharSB(&this.value.get(), j, c)?;
            i = i.wrapping_add(1i32);
            StringUTF16::putCharsSB(&this.value.get(), j, s, i, end)?;
            return Ok(());
            i = i.wrapping_add(1i32);
        }
        StringUTF16::putCharsSB(&this.value.get(), index, s, off, end)?;
        Ok(())
    }

    // java: inflateIfNeededFor(Ljava/lang/String;)V
    // java: inflateIfNeededFor(Ljava/lang/String;)V
    pub fn inflateIfNeededFor__str(&self, input: String) -> Result<()> {
        let this = self;
        let _t0 = input.coder()?;
        this.inflate()?;
        Ok(())
    }

    // java: inflateIfNeededFor(Ljava/lang/AbstractStringBuilder;)V
    // java: inflateIfNeededFor(Ljava/lang/AbstractStringBuilder;)V
    pub fn inflateIfNeededFor__abstra(&self, input: Object) -> Result<()> {
        let this = self;
        let _t0 = input.getCoder()?;
        this.inflate()?;
        Ok(())
    }

    // java: putStringAt(ILjava/lang/String;)V
    pub fn putStringAt(&self, index: i32, str: String) -> Result<()> {
        let this = self;
        this.inflateIfNeededFor(str)?;
        str.getBytes(this.value.get(), index, this.coder.get())?;
        Ok(())
    }

    // java: appendChars([CII)V
    // java: appendChars([CII)V
    pub fn appendChars__arr_c_i_i(&self, s: Vec<u16>, off: i32, end: i32) -> Result<()> {
        let this = self;
        let mut count: i32 = this.count.get();
        let _t0 = this.isLatin1()?;
        let mut val: Vec<i8> = this.value.get();
        let mut i: i32 = off;
        let mut j: i32 = count;
        loop {
            if i >= end { break; }
            let mut c: i32 = s[i as usize];
            let _t0: bool = StringLatin1::canEncode(c)?;
            j = j.wrapping_add(1i32);
            /* TODO: i2b  */
            val[j as usize] = c;
            count = j;
            this.count.set(j);
            this.inflate()?;
            StringUTF16::putCharsSB(&this.value.get(), j, &s, i, end)?;
            this.count.set(((count).wrapping_add(end)).wrapping_sub(i));
            return Ok(());
            i = i.wrapping_add(1i32);
        }
        StringUTF16::putCharsSB(&this.value.get(), count, &s, off, end)?;
        this.count.set(((count).wrapping_add(end)).wrapping_sub(off));
        Ok(())
    }

    // java: appendChars(Ljava/lang/String;II)V
    // java: appendChars(Ljava/lang/String;II)V
    pub fn appendChars__str_i_i(&self, s: String, off: i32, end: i32) -> Result<()> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let _t1 = s.isLatin1()?;
        let _t2 = s.value()?;
        System::arraycopy(&_t2, off, &this.value.get(), this.count.get(), (end).wrapping_sub(off))?;
        let mut val: Vec<i8> = this.value.get();
        let mut i: i32 = off;
        let mut j: i32 = this.count.get();
        loop {
            if i >= end { break; }
            let _t0 = s.charAt(i)?;
            let mut c: i32 = _t0;
            let _t1: bool = StringLatin1::canEncode(c)?;
            j = j.wrapping_add(1i32);
            /* TODO: i2b  */
            val[j as usize] = c;
            this.count.set(j);
            this.inflate()?;
            let _t2 = s.value()?;
            System::arraycopy(&_t2, (i<<(1i32&0x1f)), &this.value.get(), (j<<(1i32&0x1f)), ((end).wrapping_sub(i)<<(1i32&0x1f)))?;
            this.count.set((this.count.get()).wrapping_add((end).wrapping_sub(i)));
            return Ok(());
            i = i.wrapping_add(1i32);
        }
        let _t3 = s.isLatin1()?;
        StringUTF16::putCharsSB(&this.value.get(), this.count.get(), s, off, end)?;
        let _t4 = s.value()?;
        System::arraycopy(&_t4, (off<<(1i32&0x1f)), &this.value.get(), (this.count.get()<<(1i32&0x1f)), ((end).wrapping_sub(off)<<(1i32&0x1f)))?;
        this.count.set((this.count.get()).wrapping_add((end).wrapping_sub(off)));
        Ok(())
    }

    // java: appendChars(Ljava/lang/CharSequence;II)V
    // java: appendChars(Ljava/lang/CharSequence;II)V
    pub fn appendChars__seq_i_i(&self, s: Object, off: i32, end: i32) -> Result<()> {
        let this = self;
        let _t0 = this.isLatin1()?;
        let mut val: Vec<i8> = this.value.get();
        let mut i: i32 = off;
        let mut j: i32 = this.count.get();
        loop {
            if i >= end { break; }
            let _t0 = s.charAt(i)?;
            let mut c: i32 = _t0;
            let _t1: bool = StringLatin1::canEncode(c)?;
            j = j.wrapping_add(1i32);
            /* TODO: i2b  */
            val[j as usize] = c;
            this.count.set(j);
            this.inflate()?;
            j = j.wrapping_add(1i32);
            StringUTF16::putCharSB(&this.value.get(), j, c)?;
            this.count.set(j);
            i = i.wrapping_add(1i32);
            StringUTF16::putCharsSB(&this.value.get(), j, s, i, end)?;
            this.count.set((this.count.get()).wrapping_add((end).wrapping_sub(i)));
            return Ok(());
            i = i.wrapping_add(1i32);
        }
        StringUTF16::putCharsSB(&this.value.get(), this.count.get(), s, off, end)?;
        this.count.set((this.count.get()).wrapping_add((end).wrapping_sub(off)));
        Ok(())
    }

    // java: mix(J)J
    pub fn mix(&self, lengthCoder: i64) -> Result<i64> {
        let this = self;
        /* TODO: lshl  */
        /* TODO: lor  */
        Ok(32i32)
    }

    // java: prepend(J[B)J
    pub fn prepend(&self, lengthCoder: i64, arg_1: Vec<i8>) -> Result<i64> {
        let this = self;
        lengthCoder = (lengthCoder).wrapping_sub((this.count.get() as i64));
        /* TODO: lcmp  */
        System::arraycopy(&this.value.get(), 0i32, local_3, (lengthCoder as i32), this.count.get())?;
        StringUTF16::inflate(&this.value.get(), 0i32, local_3, (lengthCoder as i32), this.count.get())?;
        System::arraycopy(&this.value.get(), 0i32, local_3, ((lengthCoder as i32)<<(1i32&0x1f)), (this.count.get()<<(1i32&0x1f)))?;
        Ok(lengthCoder)
    }

    // java: repeat(CI)Ljava/lang/AbstractStringBuilder;
    // java: repeat(CI)Ljava/lang/AbstractStringBuilder;
    pub fn repeat__c_i(&self, c: u16, count: i32) -> Result<Object> {
        let this = self;
        let mut limit: i32 = (this.count.get()).wrapping_add(count);
        this.ensureCapacityInternal(limit)?;
        let _t0 = this.isLatin1()?;
        let mut isLatin1: i32 = _t0;
        let _t1: bool = StringLatin1::canEncode(c)?;
        /* TODO: i2b  */
        Arrays::fill(&this.value.get(), this.count.get(), limit, c)?;
        this.inflate()?;
        let mut index: i32 = this.count.get();
        loop {
            if index >= limit { break; }
            StringUTF16::putCharSB(&this.value.get(), index, c)?;
            index = index.wrapping_add(1i32);
        }
        this.count.set(limit);
        Ok(this)
    }

    // java: repeat(II)Ljava/lang/AbstractStringBuilder;
    // java: repeat(II)Ljava/lang/AbstractStringBuilder;
    pub fn repeat__i_i(&self, codePoint: i32, count: i32) -> Result<Object> {
        let this = self;
        String::new().append(&String::from("count is negative:"))?;
        String::new().append(&count)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Ok(this);
        let _t0: bool = Character::isBmpCodePoint(codePoint)?;
        /* TODO: i2c  */
        let _t1 = this.repeat(codePoint, count)?;
        let _t2: Vec<u16> = Character::toChars(codePoint)?;
        let _t3: Object = CharBuffer::wrap(&_t2)?;
        let _t4 = this.repeat(_t3, count)?;
        Ok(this)
    }

    // java: repeat(Ljava/lang/CharSequence;I)Ljava/lang/AbstractStringBuilder;
    // java: repeat(Ljava/lang/CharSequence;I)Ljava/lang/AbstractStringBuilder;
    pub fn repeat__seq_i(&self, cs: Object, count: i32) -> Result<Object> {
        let this = self;
        String::new().append(&String::from("count is negative:"))?;
        String::new().append(&count)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Ok(this);
        let _t0 = this.append(cs)?;
        return Ok(_t0);
        cs = String::from("null");
        let _t1 = cs.length()?;
        let mut length: i32 = _t1;
        return Ok(this);
        let _t2 = cs.charAt(0i32)?;
        let _t3 = this.repeat(_t2, count)?;
        return Ok(_t3);
        let mut offset: i32 = this.count.get();
        let mut valueLength: i32 = (length<<(this.coder.get()&0x1f));
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut total: i32 = (count).wrapping_mul(length);
        let mut limit: i32 = (offset).wrapping_add(total);
        this.ensureCapacityInternal(limit)?;
        let mut str: Object = cs;
        this.putStringAt(offset, str)?;
        let mut asb: Object = cs;
        let _t4 = this.append(asb)?;
        this.appendChars(cs, 0i32, length)?;
        String::repeatCopyRest(&this.value.get(), (offset<<(this.coder.get()&0x1f)), (total<<(this.coder.get()&0x1f)), (length<<(this.coder.get()&0x1f)))?;
        this.count.set(limit);
        Ok(this)
    }
}
