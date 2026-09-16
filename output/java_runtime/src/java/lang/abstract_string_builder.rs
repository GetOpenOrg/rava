#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;
use crate::jdk::internal::util::ArraysSupport;
use crate::jdk::internal::util::Preconditions;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/AbstractStringBuilder",
    super_class       = "java/lang/Object",
    interfaces        = "java/lang/Appendable,java/lang/CharSequence",
    access            = "package",
    modifiers         = "abstract",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = true,
    is_enum           = false,
    is_deprecated     = false,
    source            = "AbstractStringBuilder.java",
    inner_classes     = "java/lang/StringLatin1$CharsSpliterator:java/lang/StringLatin1:CharsSpliterator:8;java/lang/StringUTF16$CodePointsSpliterator:java/lang/StringUTF16:CodePointsSpliterator:8;java/lang/StringUTF16$CharsSpliterator:java/lang/StringUTF16:CharsSpliterator:8;java/util/Spliterator$OfInt:java/util/Spliterator:OfInt:1545;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25",
    all_supertypes    = "java/lang/AbstractStringBuilder;java/lang/Appendable;java/lang/CharSequence;java/lang/Object",
    has_to_string_method = true,
)]
#[derive(Clone, Default, PartialEq)]
pub struct AbstractStringBuilder {
    #[cfg_attr(any(), java_field(name = "value", descriptor = "[B", is_static = false))]
    pub value: JField<Rc<RefCell<Vec<i8>>>>,
    #[cfg_attr(any(), java_field(name = "coder", descriptor = "B", is_static = false))]
    pub coder: JField<i8>,
    #[cfg_attr(any(), java_field(name = "maybeLatin1", descriptor = "Z", is_static = false))]
    pub maybeLatin1: JField<bool>,
    #[cfg_attr(any(), java_field(name = "count", descriptor = "I", is_static = false))]
    pub count: JField<i32>,
}

impl AbstractStringBuilder {
    #[cfg_attr(any(), java_field(name = "EMPTYVALUE", descriptor = "[B", access = "private", modifiers = "static final", is_static = true))]
    // static field: EMPTYVALUE:[B
    pub fn EMPTYVALUE() -> Rc<RefCell<Vec<i8>>> {
        panic!("stub: java/lang/AbstractStringBuilder.EMPTYVALUE:[B")
    }

    #[java_rta_macros::java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn new() -> Result<Self> {
        panic!("stub: java/lang/AbstractStringBuilder.<init>:()V")
    }

    #[java_rta_macros::java_method(name = "<init>", descriptor = "(I)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    // java: <init>(I)V
    pub fn new_i(mut capacity: i32) -> Result<Self> {
        let mut this = Self { value: JField::new(Default::default()), coder: JField::new(Default::default()), maybeLatin1: JField::new(false), count: JField::new(0), ..Default::default() };
        /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
        if String::COMPACT_STRINGS() {
            let mut _arr0: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; capacity as usize]));
            this.value.set(Clone::clone(&_arr0));
            this.coder.set(((0i32) as i8));
        } else {
            let _t0: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor(capacity)?;
            this.value.set(Clone::clone(&_t0));
            this.coder.set(((1i32) as i8));
        }
        Ok(this)
    }

    #[java_rta_macros::java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn new_str(str: String) -> Result<Self> {
        panic!("stub: java/lang/AbstractStringBuilder.<init>:(Ljava/lang/String;)V")
    }

    #[java_rta_macros::java_method(name = "<init>", descriptor = "(Ljava/lang/CharSequence;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn new_seq(seq: Object) -> Result<Self> {
        panic!("stub: java/lang/AbstractStringBuilder.<init>:(Ljava/lang/CharSequence;)V")
    }

    #[java_rta_macros::java_method(name = "compareTo", descriptor = "(Ljava/lang/AbstractStringBuilder;)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn compareTo(&self, another: AbstractStringBuilder) -> Result<i32> {
        panic!("stub: java/lang/AbstractStringBuilder.compareTo:(Ljava/lang/AbstractStringBuilder;)I")
    }

    #[java_rta_macros::java_method(name = "length", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn length(&self) -> Result<i32> {
        let this = self;
        Ok(this.count.get())
    }

    #[java_rta_macros::java_method(name = "capacity", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn capacity(&self) -> Result<i32> {
        panic!("stub: java/lang/AbstractStringBuilder.capacity:()I")
    }

    #[java_rta_macros::java_method(name = "ensureCapacity", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn ensureCapacity(&self, minimumCapacity: i32) -> Result<()> {
        panic!("stub: java/lang/AbstractStringBuilder.ensureCapacity:(I)V")
    }

    #[java_rta_macros::java_method(name = "ensureCapacityInternal", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn ensureCapacityInternal(&self, mut minimumCapacity: i32) -> Result<()> {
        let this = self;
        let mut oldCapacity = ((this.value.get().borrow().len() as i32)>>(((this.coder.get() as i32)&0x1f)));
        if ((minimumCapacity).wrapping_sub(oldCapacity)>0) {
            let _t0 = this.newCapacity(minimumCapacity)?;
            let _t1: Rc<RefCell<Vec<i8>>> = Arrays::copyOf_arr_b_i(Clone::clone(&this.value.get()), (_t0<<((this.coder.get() as i32)&0x1f)))?;
            this.value.set(Clone::clone(&_t1));
        }
        Ok(())
    }

    #[java_rta_macros::java_method(name = "newCapacity", descriptor = "(I)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn newCapacity(&self, mut minCapacity: i32) -> Result<i32> {
        let this = self;
        let mut oldLength = (this.value.get().borrow().len() as i32);
        let mut newLength = (minCapacity<<((this.coder.get() as i32)&0x1f));
        let mut growth = (newLength).wrapping_sub(oldLength);
        let _t0: i32 = ArraysSupport::newLength(oldLength, growth, (oldLength).wrapping_add((2i32<<((this.coder.get() as i32)&0x1f))))?;
        let mut length: i32 = _t0;
        if length == 2147483647i32 {
            return Err(JvmError::Custom("athrow".to_owned()));
        }
        Ok((length>>(((this.coder.get() as i32)&0x1f))))
    }

    #[java_rta_macros::java_method(name = "inflate", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn inflate(&self) -> Result<()> {
        let this = self;
        let _t0 = this.isLatin1()?;
        if !(_t0) {
            return Ok(());
        }
        let _t1: Rc<RefCell<Vec<i8>>> = StringUTF16::newBytesFor((this.value.get().borrow().len() as i32))?;
        let mut buf: Rc<RefCell<Vec<i8>>> = _t1;
        StringLatin1::inflate_arr_b_i_arr_b_i_i(Clone::clone(&this.value.get()), 0i32, Clone::clone(&buf), 0i32, this.count.get())?;
        this.value.set(Clone::clone(&buf));
        this.coder.set(((1i32) as i8));
        Ok(())
    }

    #[java_rta_macros::java_method(name = "trimToSize", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn trimToSize(&self) -> Result<()> {
        panic!("stub: java/lang/AbstractStringBuilder.trimToSize:()V")
    }

    #[java_rta_macros::java_method(name = "setLength", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn setLength(&self, newLength: i32) -> Result<()> {
        panic!("stub: java/lang/AbstractStringBuilder.setLength:(I)V")
    }

    #[java_rta_macros::java_method(name = "charAt", descriptor = "(I)C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn charAt(&self, index: i32) -> Result<u16> {
        panic!("stub: java/lang/AbstractStringBuilder.charAt:(I)C")
    }

    #[java_rta_macros::java_method(name = "codePointAt", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn codePointAt(&self, index: i32) -> Result<i32> {
        panic!("stub: java/lang/AbstractStringBuilder.codePointAt:(I)I")
    }

    #[java_rta_macros::java_method(name = "codePointBefore", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn codePointBefore(&self, index: i32) -> Result<i32> {
        panic!("stub: java/lang/AbstractStringBuilder.codePointBefore:(I)I")
    }

    #[java_rta_macros::java_method(name = "codePointCount", descriptor = "(II)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn codePointCount(&self, beginIndex: i32, endIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/AbstractStringBuilder.codePointCount:(II)I")
    }

    #[java_rta_macros::java_method(name = "offsetByCodePoints", descriptor = "(II)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn offsetByCodePoints(&self, index: i32, codePointOffset: i32) -> Result<i32> {
        panic!("stub: java/lang/AbstractStringBuilder.offsetByCodePoints:(II)I")
    }

    #[java_rta_macros::java_method(name = "getChars", descriptor = "(II[CI)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getChars(&self, srcBegin: i32, srcEnd: i32, dst: Rc<RefCell<Vec<u16>>>, dstBegin: i32) -> Result<()> {
        panic!("stub: java/lang/AbstractStringBuilder.getChars:(II[CI)V")
    }

    #[java_rta_macros::java_method(name = "setCharAt", descriptor = "(IC)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn setCharAt(&self, index: i32, ch: u16) -> Result<()> {
        panic!("stub: java/lang/AbstractStringBuilder.setCharAt:(IC)V")
    }

    #[java_rta_macros::java_method(name = "append", descriptor = "(Ljava/lang/Object;)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn append_obj(&self, obj: Object) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.append:(Ljava/lang/Object;)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "append", descriptor = "(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    // java: append(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;
    pub fn append_str(&self, mut str: String) -> Result<AbstractStringBuilder> {
        let this = self;
        if _is_jnull(&str) {
            let _t0 = this.appendNull()?;
            return Ok(_t0);
        }
        let _t0 = str.length()?;
        let mut len: i32 = _t0;
        this.ensureCapacityInternal((this.count.get()).wrapping_add(len))?;
        this.putStringAt(this.count.get(), Clone::clone(&str))?;
        this.count.set((this.count.get()).wrapping_add(len));
        Ok(Clone::clone(this))
    }

    #[java_rta_macros::java_method(name = "append", descriptor = "(Ljava/lang/StringBuffer;)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn append_string(&self, sb: Object) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.append:(Ljava/lang/StringBuffer;)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "append", descriptor = "(Ljava/lang/AbstractStringBuilder;)Ljava/lang/AbstractStringBuilder;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn append_abstra(&self, asb: AbstractStringBuilder) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.append:(Ljava/lang/AbstractStringBuilder;)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn append_seq(&self, s: Object) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.append:(Ljava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "appendNull", descriptor = "()Ljava/lang/AbstractStringBuilder;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn appendNull(&self) -> Result<AbstractStringBuilder> {
        let this = self;
        this.ensureCapacityInternal((this.count.get()).wrapping_add(4i32))?;
        let mut count = this.count.get();
        let mut val = this.value.get();
        let _t0 = this.isLatin1()?;
        if _t0 {
            count = count.wrapping_add(1i32);
            val.borrow_mut()[count as usize] = (110i32) as i8;
            count = count.wrapping_add(1i32);
            val.borrow_mut()[count as usize] = (117i32) as i8;
            count = count.wrapping_add(1i32);
            val.borrow_mut()[count as usize] = (108i32) as i8;
            count = count.wrapping_add(1i32);
            val.borrow_mut()[count as usize] = (108i32) as i8;
        } else {
            let _t1: i32 = StringUTF16::putCharsAt_arr_b_i_c_c_c_c(Clone::clone(&val), count, ((110i32) as u16), ((117i32) as u16), ((108i32) as u16), ((108i32) as u16))?;
            count = _t1;
        }
        this.count.set(count);
        Ok(Clone::clone(this))
    }

    #[java_rta_macros::java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn append_seq_i_i(&self, s: Object, start: i32, end: i32) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.append:(Ljava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "append", descriptor = "([C)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn append_arr_c(&self, str: Rc<RefCell<Vec<u16>>>) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.append:([C)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "append", descriptor = "([CII)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn append_arr_c_i_i(&self, str: Rc<RefCell<Vec<u16>>>, offset: i32, len: i32) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.append:([CII)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "append", descriptor = "(Z)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn append_z(&self, b: bool) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.append:(Z)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "append", descriptor = "(C)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn append_c(&self, c: u16) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.append:(C)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "append", descriptor = "(I)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    // java: append(I)Ljava/lang/AbstractStringBuilder;
    pub fn append_i(&self, mut i: i32) -> Result<AbstractStringBuilder> {
        let this = self;
        let mut count = this.count.get();
        let _t0: i32 = Integer::stringSize(i)?;
        let mut spaceNeeded = (count).wrapping_add(_t0);
        this.ensureCapacityInternal(spaceNeeded)?;
        let _t1 = this.isLatin1()?;
        if _t1 {
            let _t2: i32 = Integer::getChars(i, spaceNeeded, Clone::clone(&this.value.get()))?;
        } else {
            let _t2: i32 = StringUTF16::getChars_i_i_i_arr_b(i, count, spaceNeeded, Clone::clone(&this.value.get()))?;
        }
        this.count.set(spaceNeeded);
        Ok(Clone::clone(this))
    }

    #[java_rta_macros::java_method(name = "append", descriptor = "(J)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn append_l(&self, l: i64) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.append:(J)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "append", descriptor = "(F)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn append_f(&self, f: f32) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.append:(F)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "append", descriptor = "(D)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn append_d(&self, d: f64) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.append:(D)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "delete", descriptor = "(II)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn delete(&self, start: i32, end: i32) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.delete:(II)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "appendCodePoint", descriptor = "(I)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn appendCodePoint(&self, codePoint: i32) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.appendCodePoint:(I)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "deleteCharAt", descriptor = "(I)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn deleteCharAt(&self, index: i32) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.deleteCharAt:(I)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "replace", descriptor = "(IILjava/lang/String;)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn replace(&self, start: i32, end: i32, str: String) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.replace:(IILjava/lang/String;)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "substring", descriptor = "(I)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn substring_i(&self, start: i32) -> Result<String> {
        panic!("stub: java/lang/AbstractStringBuilder.substring:(I)Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "subSequence", descriptor = "(II)Ljava/lang/CharSequence;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn subSequence(&self, start: i32, end: i32) -> Result<Object> {
        panic!("stub: java/lang/AbstractStringBuilder.subSequence:(II)Ljava/lang/CharSequence;")
    }

    #[java_rta_macros::java_method(name = "substring", descriptor = "(II)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn substring_i_i(&self, start: i32, end: i32) -> Result<String> {
        panic!("stub: java/lang/AbstractStringBuilder.substring:(II)Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "shift", descriptor = "(II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn shift(&self, offset: i32, n: i32) -> Result<()> {
        panic!("stub: java/lang/AbstractStringBuilder.shift:(II)V")
    }

    #[java_rta_macros::java_method(name = "insert", descriptor = "(I[CII)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn insert_i_arr_c_i_i(&self, index: i32, str: Rc<RefCell<Vec<u16>>>, offset: i32, len: i32) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.insert:(I[CII)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "insert", descriptor = "(ILjava/lang/Object;)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn insert_i_obj(&self, offset: i32, obj: Object) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.insert:(ILjava/lang/Object;)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "insert", descriptor = "(ILjava/lang/String;)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn insert_i_str(&self, offset: i32, str: String) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.insert:(ILjava/lang/String;)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "insert", descriptor = "(I[C)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn insert_i_arr_c(&self, offset: i32, str: Rc<RefCell<Vec<u16>>>) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.insert:(I[C)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "insert", descriptor = "(ILjava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn insert_i_seq(&self, dstOffset: i32, s: Object) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.insert:(ILjava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "insert", descriptor = "(ILjava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn insert_i_seq_i_i(&self, dstOffset: i32, s: Object, start: i32, end: i32) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.insert:(ILjava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "insert", descriptor = "(IZ)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn insert_i_z(&self, offset: i32, b: bool) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.insert:(IZ)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "insert", descriptor = "(IC)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn insert_i_c(&self, offset: i32, c: u16) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.insert:(IC)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "insert", descriptor = "(II)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn insert_i_i(&self, offset: i32, i: i32) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.insert:(II)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "insert", descriptor = "(IJ)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn insert_i_l(&self, offset: i32, l: i64) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.insert:(IJ)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "insert", descriptor = "(IF)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn insert_i_f(&self, offset: i32, f: f32) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.insert:(IF)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "insert", descriptor = "(ID)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn insert_i_d(&self, offset: i32, d: f64) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.insert:(ID)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "indexOf", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn indexOf_str(&self, str: String) -> Result<i32> {
        panic!("stub: java/lang/AbstractStringBuilder.indexOf:(Ljava/lang/String;)I")
    }

    #[java_rta_macros::java_method(name = "indexOf", descriptor = "(Ljava/lang/String;I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn indexOf_str_i(&self, str: String, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/AbstractStringBuilder.indexOf:(Ljava/lang/String;I)I")
    }

    #[java_rta_macros::java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn lastIndexOf_str(&self, str: String) -> Result<i32> {
        panic!("stub: java/lang/AbstractStringBuilder.lastIndexOf:(Ljava/lang/String;)I")
    }

    #[java_rta_macros::java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/String;I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn lastIndexOf_str_i(&self, str: String, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/AbstractStringBuilder.lastIndexOf:(Ljava/lang/String;I)I")
    }

    #[java_rta_macros::java_method(name = "reverse", descriptor = "()Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn reverse(&self) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.reverse:()Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
    pub fn toString(&self) -> Result<String> {
        panic!("stub: java/lang/AbstractStringBuilder.toString:()Ljava/lang/String;")
    }

    #[java_rta_macros::java_method(name = "chars", descriptor = "()Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn chars(&self) -> Result<Object> {
        panic!("stub: java/lang/AbstractStringBuilder.chars:()Ljava/util/stream/IntStream;")
    }

    #[java_rta_macros::java_method(name = "codePoints", descriptor = "()Ljava/util/stream/IntStream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn codePoints(&self) -> Result<Object> {
        panic!("stub: java/lang/AbstractStringBuilder.codePoints:()Ljava/util/stream/IntStream;")
    }

    #[java_rta_macros::java_method(name = "getValue", descriptor = "()[B", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getValue(&self) -> Result<Rc<RefCell<Vec<i8>>>> {
        let this = self;
        Ok(this.value.get())
    }

    #[java_rta_macros::java_method(name = "getBytes", descriptor = "([BIB)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getBytes(&self, dst: Rc<RefCell<Vec<i8>>>, dstBegin: i32, coder: i8) -> Result<()> {
        panic!("stub: java/lang/AbstractStringBuilder.getBytes:([BIB)V")
    }

    #[java_rta_macros::java_method(name = "initBytes", descriptor = "([CII)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn initBytes(&self, value: Rc<RefCell<Vec<u16>>>, off: i32, len: i32) -> Result<()> {
        panic!("stub: java/lang/AbstractStringBuilder.initBytes:([CII)V")
    }

    #[java_rta_macros::java_method(name = "getCoder", descriptor = "()B", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn getCoder(&self) -> Result<i8> {
        panic!("stub: java/lang/AbstractStringBuilder.getCoder:()B")
    }

    #[java_rta_macros::java_method(name = "isLatin1", descriptor = "()Z", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn isLatin1(&self) -> Result<bool> {
        let this = self;
        Ok((if String::COMPACT_STRINGS() { (this.coder.get()==0) } else { (0i32 != 0) }))
    }

    #[java_rta_macros::java_method(name = "putCharsAt", descriptor = "(I[CII)V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn putCharsAt_i_arr_c_i_i(&self, index: i32, s: Rc<RefCell<Vec<u16>>>, off: i32, end: i32) -> Result<()> {
        panic!("stub: java/lang/AbstractStringBuilder.putCharsAt:(I[CII)V")
    }

    #[java_rta_macros::java_method(name = "putCharsAt", descriptor = "(ILjava/lang/CharSequence;II)V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn putCharsAt_i_seq_i_i(&self, index: i32, s: Object, off: i32, end: i32) -> Result<()> {
        panic!("stub: java/lang/AbstractStringBuilder.putCharsAt:(ILjava/lang/CharSequence;II)V")
    }

    #[java_rta_macros::java_method(name = "inflateIfNeededFor", descriptor = "(Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    // java: inflateIfNeededFor(Ljava/lang/String;)V
    pub fn inflateIfNeededFor_str(&self, mut input: String) -> Result<()> {
        let this = self;
        let _t0 = input.coder()?;
        if (this.coder.get() as i32) != (_t0 as i32) {
            this.inflate()?;
        }
        Ok(())
    }

    #[java_rta_macros::java_method(name = "inflateIfNeededFor", descriptor = "(Ljava/lang/AbstractStringBuilder;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn inflateIfNeededFor_abstra(&self, input: AbstractStringBuilder) -> Result<()> {
        panic!("stub: java/lang/AbstractStringBuilder.inflateIfNeededFor:(Ljava/lang/AbstractStringBuilder;)V")
    }

    #[java_rta_macros::java_method(name = "putStringAt", descriptor = "(ILjava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn putStringAt(&self, mut index: i32, mut str: String) -> Result<()> {
        let this = self;
        this.inflateIfNeededFor_str(Clone::clone(&str))?;
        str.getBytes_arr_b_i_b(Clone::clone(&this.value.get()), index, this.coder.get())?;
        Ok(())
    }

    #[java_rta_macros::java_method(name = "appendChars", descriptor = "([CII)V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn appendChars_arr_c_i_i(&self, s: Rc<RefCell<Vec<u16>>>, off: i32, end: i32) -> Result<()> {
        panic!("stub: java/lang/AbstractStringBuilder.appendChars:([CII)V")
    }

    #[java_rta_macros::java_method(name = "appendChars", descriptor = "(Ljava/lang/String;II)V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn appendChars_str_i_i(&self, s: String, off: i32, end: i32) -> Result<()> {
        panic!("stub: java/lang/AbstractStringBuilder.appendChars:(Ljava/lang/String;II)V")
    }

    #[java_rta_macros::java_method(name = "appendChars", descriptor = "(Ljava/lang/CharSequence;II)V", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn appendChars_seq_i_i(&self, s: Object, off: i32, end: i32) -> Result<()> {
        panic!("stub: java/lang/AbstractStringBuilder.appendChars:(Ljava/lang/CharSequence;II)V")
    }

    #[java_rta_macros::java_method(name = "mix", descriptor = "(J)J", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn mix(&self, lengthCoder: i64) -> Result<i64> {
        panic!("stub: java/lang/AbstractStringBuilder.mix:(J)J")
    }

    #[java_rta_macros::java_method(name = "prepend", descriptor = "(J[B)J", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn prepend(&self, lengthCoder: i64, arg1: Rc<RefCell<Vec<i8>>>) -> Result<i64> {
        panic!("stub: java/lang/AbstractStringBuilder.prepend:(J[B)J")
    }

    #[java_rta_macros::java_method(name = "repeat", descriptor = "(CI)Ljava/lang/AbstractStringBuilder;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn repeat_c_i(&self, c: u16, count: i32) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.repeat:(CI)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "repeat", descriptor = "(II)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn repeat_i_i(&self, codePoint: i32, count: i32) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.repeat:(II)Ljava/lang/AbstractStringBuilder;")
    }

    #[java_rta_macros::java_method(name = "repeat", descriptor = "(Ljava/lang/CharSequence;I)Ljava/lang/AbstractStringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
    pub fn repeat_seq_i(&self, cs: Object, count: i32) -> Result<AbstractStringBuilder> {
        panic!("stub: java/lang/AbstractStringBuilder.repeat:(Ljava/lang/CharSequence;I)Ljava/lang/AbstractStringBuilder;")
    }
}
