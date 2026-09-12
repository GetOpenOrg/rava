#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/StringJoiner",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public final",
    source      = "StringJoiner.java",
))]
pub struct StringJoiner {
    #[cfg_attr(any(), java_field(name = "prefix", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub prefix: Field<String>,
    #[cfg_attr(any(), java_field(name = "delimiter", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub delimiter: Field<String>,
    #[cfg_attr(any(), java_field(name = "suffix", descriptor = "Ljava/lang/String;", access = "private final"))]
    pub suffix: Field<String>,
    #[cfg_attr(any(), java_field(name = "elts", descriptor = "[Ljava/lang/String;", access = "private"))]
    pub elts: Field<Vec<String>>,
    #[cfg_attr(any(), java_field(name = "size", descriptor = "I", access = "private"))]
    pub size: Field<i32>,
    #[cfg_attr(any(), java_field(name = "len", descriptor = "I", access = "private"))]
    pub len: Field<i32>,
    #[cfg_attr(any(), java_field(name = "emptyValue", descriptor = "Ljava/lang/String;", access = "private"))]
    pub emptyValue: Field<String>,
}

impl StringJoiner {
    // java: <init>(Ljava/lang/CharSequence;)V
    // java: <init>(Ljava/lang/CharSequence;)V
    pub fn new__seq(delimiter: Object) -> Result<Self> {
        let this = Self { prefix: Field::new(String::new()), delimiter: Field::new(String::new()), suffix: Field::new(String::new()), elts: Field::new(Default::default()), size: Field::new(0), len: Field::new(0), emptyValue: Field::new(String::new()) };
        /* invokespecial Method java/util/StringJoiner.<init>:(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)V */
        Ok(this)
    }

    // java: <init>(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)V
    // java: <init>(Ljava/lang/CharSequence;Ljava/lang/CharSequence;Ljava/lang/CharSequence;)V
    pub fn new__seq_seq_seq(delimiter: Object, prefix: Object, suffix: Object) -> Result<Self> {
        let this = Self { prefix: Field::new(String::new()), delimiter: Field::new(String::new()), suffix: Field::new(String::new()), elts: Field::new(Default::default()), size: Field::new(0), len: Field::new(0), emptyValue: Field::new(String::new()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        let _t0: Object = Objects::requireNonNull(prefix, String::from("The prefix must not be null"))?;
        let _t1: Object = Objects::requireNonNull(delimiter, String::from("The delimiter must not be null"))?;
        let _t2: Object = Objects::requireNonNull(suffix, String::from("The suffix must not be null"))?;
        let _t3 = prefix.toString()?;
        this.prefix.set(_t3);
        let _t4 = delimiter.toString()?;
        this.delimiter.set(_t4);
        let _t5 = suffix.toString()?;
        this.suffix.set(_t5);
        let _t6 = this.checkAddLength(0i32, 0i32)?;
        Ok(this)
    }

    // java: setEmptyValue(Ljava/lang/CharSequence;)Ljava/util/StringJoiner;
    pub fn setEmptyValue(&self, emptyValue: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(emptyValue, String::from("The empty value must not be null"))?;
        let _t1 = _t0.toString()?;
        this.emptyValue.set(_t1);
        Ok(this)
    }

    // java: toString()Ljava/lang/String;
    pub fn toString(&self) -> Result<String> {
        let this = self;
        let mut size: i32 = this.size.get();
        let mut elts: Vec<String> = this.elts.get();
        return Ok(this.emptyValue.get());
        elts = StringJoiner::EMPTY_STRING_ARRAY();
        let _t0 = StringJoiner::JLA().join(this.prefix.get(), this.suffix.get(), this.delimiter.get(), elts, size)?;
        Ok(_t0)
    }

    // java: add(Ljava/lang/CharSequence;)Ljava/util/StringJoiner;
    pub fn add(&self, newElement: Object) -> Result<Object> {
        let this = self;
        let mut elt: String = String::from_owned(format!("{}", newElement));
        let mut _arr0: Vec<Object> = Vec::with_capacity(8i32 as usize);
        this.elts.set(_arr0);
        let _t1: Vec<Object> = Arrays::copyOf(&this.elts.get(), (2i32).wrapping_mul(this.size.get()))?;
        this.elts.set(_t1);
        let _t2 = this.delimiter.get().length()?;
        let _t3 = this.checkAddLength(this.len.get(), _t2)?;
        this.len.set(_t3);
        let _t4 = elt.length()?;
        let _t5 = this.checkAddLength(this.len.get(), _t4)?;
        this.len.set(_t5);
        this.size.set((this.size.get()).wrapping_add(1i32));
        this.elts.get()[this.size.get() as usize] = elt;
        Ok(this)
    }

    // java: checkAddLength(II)I
    pub fn checkAddLength(&self, oldLen: i32, inc: i32) -> Result<i32> {
        let this = self;
        let mut newLen: i64 = ((oldLen as i64)).wrapping_add((inc as i64));
        let _t0 = this.prefix.get().length()?;
        let _t1 = this.suffix.get().length()?;
        let mut tmpLen: i64 = ((newLen).wrapping_add((_t0 as i64))).wrapping_add((_t1 as i64));
        /* TODO: lcmp  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok((newLen as i32))
    }

    // java: merge(Ljava/util/StringJoiner;)Ljava/util/StringJoiner;
    pub fn merge(&self, other: Object) -> Result<Object> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(other)?;
        return Ok(this);
        other.compactElts()?;
        let _t1 = this.add(other.elts.get()[0i32 as usize].clone())?;
        Ok(_t1)
    }

    // java: compactElts()V
    pub fn compactElts(&self) -> Result<()> {
        let this = self;
        let mut sz: i32 = this.size.get();
        let _t0 = StringJoiner::JLA().join(String::from(""), String::from(""), this.delimiter.get(), this.elts.get(), sz)?;
        this.elts.get()[0i32 as usize] = _t0;
        /* TODO: aconst_null  */
        Arrays::fill(1i32, &this.elts.get(), 1i32, sz)?;
        this.size.set(1i32);
        Ok(())
    }

    // java: length()I
    pub fn length(&self) -> Result<i32> {
        let this = self;
        let _t0 = this.emptyValue.get().length()?;
        let _t1 = this.prefix.get().length()?;
        let _t2 = this.suffix.get().length()?;
        Ok(((this.len.get()).wrapping_add(_t1)).wrapping_add(_t2))
    }
}
