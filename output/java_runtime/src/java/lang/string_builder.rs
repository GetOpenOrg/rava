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

#[java_rta_macros::java_class(
    binary_name       = "java/lang/StringBuilder",
    super_class       = "java/lang/AbstractStringBuilder",
    interfaces        = "java/lang/Appendable,java/io/Serializable,java/lang/Comparable,java/lang/CharSequence",
    access            = "public",
    modifiers         = "final",
    generic_signature = "Ljava/lang/AbstractStringBuilder;Ljava/lang/Appendable;Ljava/io/Serializable;Ljava/lang/Comparable<Ljava/lang/StringBuilder;>;Ljava/lang/CharSequence;",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "StringBuilder.java",
    all_supertypes    = "java/io/Serializable;java/lang/AbstractStringBuilder;java/lang/Appendable;java/lang/CharSequence;java/lang/Comparable;java/lang/Object;java/lang/StringBuilder",
)]
#[derive(Clone, Default, PartialEq)]
pub struct StringBuilder {
    pub _super: AbstractStringBuilder,
}

impl StringBuilder {
    pub fn as_abstract_string_builder(&self) -> &AbstractStringBuilder { &self._super }
    pub fn into_abstract_string_builder(self) -> AbstractStringBuilder { self._super }
}

impl From<StringBuilder> for AbstractStringBuilder {
    fn from(v: StringBuilder) -> AbstractStringBuilder { v._super }
}

impl StringBuilder {
    #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "4383685877147921099"))]
    // static field: serialVersionUID:J
    pub fn serialVersionUID() -> i64 {
        4383685877147921099i64
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let mut this = Self { _super: Default::default(), ..Default::default() };
        this._super = AbstractStringBuilder::new_i(16i32)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_i(capacity: i32) -> Result<Self> {
        panic!("stub: java/lang/StringBuilder.<init>:(I)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_str(str: String) -> Result<Self> {
        panic!("stub: java/lang/StringBuilder.<init>:(Ljava/lang/String;)V")
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/CharSequence;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new_seq(seq: Object) -> Result<Self> {
        panic!("stub: java/lang/StringBuilder.<init>:(Ljava/lang/CharSequence;)V")
    }

    #[cfg_attr(any(), java_method(name = "compareTo", descriptor = "(Ljava/lang/StringBuilder;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn compareTo(&self, another: StringBuilder) -> Result<i32> {
        panic!("stub: java/lang/StringBuilder.compareTo:(Ljava/lang/StringBuilder;)I")
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/Object;)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn append_obj(&self, obj: Object) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.append:(Ljava/lang/Object;)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/String;)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: append(Ljava/lang/String;)Ljava/lang/StringBuilder;
    pub fn append_str(&self, mut str: String) -> Result<StringBuilder> {
        let this = self;
        let _t0 = this._super.append_str(Clone::clone(&str))?;
        Ok(Clone::clone(this))
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/StringBuffer;)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn append_string(&self, sb: Object) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.append:(Ljava/lang/StringBuffer;)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn append_seq(&self, s: Object) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.append:(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn append_seq_i_i(&self, s: Object, start: i32, end: i32) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.append:(Ljava/lang/CharSequence;II)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "([C)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn append_arr_c(&self, str: Rc<RefCell<Vec<u16>>>) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.append:([C)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "([CII)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn append_arr_c_i_i(&self, str: Rc<RefCell<Vec<u16>>>, offset: i32, len: i32) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.append:([CII)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Z)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn append_z(&self, b: bool) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.append:(Z)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(C)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn append_c(&self, c: u16) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.append:(C)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(I)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: append(I)Ljava/lang/StringBuilder;
    pub fn append_i(&self, mut i: i32) -> Result<StringBuilder> {
        let this = self;
        let _t0 = this._super.append_i(i)?;
        Ok(Clone::clone(this))
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(J)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn append_l(&self, lng: i64) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.append:(J)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(F)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn append_f(&self, f: f32) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.append:(F)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(D)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn append_d(&self, d: f64) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.append:(D)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "appendCodePoint", descriptor = "(I)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn appendCodePoint(&self, codePoint: i32) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.appendCodePoint:(I)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "delete", descriptor = "(II)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn delete(&self, start: i32, end: i32) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.delete:(II)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "deleteCharAt", descriptor = "(I)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn deleteCharAt(&self, index: i32) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.deleteCharAt:(I)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "replace", descriptor = "(IILjava/lang/String;)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn replace(&self, start: i32, end: i32, str: String) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.replace:(IILjava/lang/String;)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(I[CII)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn insert_i_arr_c_i_i(&self, index: i32, str: Rc<RefCell<Vec<u16>>>, offset: i32, len: i32) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.insert:(I[CII)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ILjava/lang/Object;)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn insert_i_obj(&self, offset: i32, obj: Object) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.insert:(ILjava/lang/Object;)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ILjava/lang/String;)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn insert_i_str(&self, offset: i32, str: String) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.insert:(ILjava/lang/String;)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(I[C)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn insert_i_arr_c(&self, offset: i32, str: Rc<RefCell<Vec<u16>>>) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.insert:(I[C)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ILjava/lang/CharSequence;)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn insert_i_seq(&self, dstOffset: i32, s: Object) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.insert:(ILjava/lang/CharSequence;)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ILjava/lang/CharSequence;II)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn insert_i_seq_i_i(&self, dstOffset: i32, s: Object, start: i32, end: i32) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.insert:(ILjava/lang/CharSequence;II)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(IZ)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn insert_i_z(&self, offset: i32, b: bool) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.insert:(IZ)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(IC)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn insert_i_c(&self, offset: i32, c: u16) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.insert:(IC)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(II)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn insert_i_i(&self, offset: i32, i: i32) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.insert:(II)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(IJ)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn insert_i_l(&self, offset: i32, l: i64) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.insert:(IJ)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(IF)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn insert_i_f(&self, offset: i32, f: f32) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.insert:(IF)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "insert", descriptor = "(ID)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn insert_i_d(&self, offset: i32, d: f64) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.insert:(ID)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn indexOf_str(&self, str: String) -> Result<i32> {
        panic!("stub: java/lang/StringBuilder.indexOf:(Ljava/lang/String;)I")
    }

    #[cfg_attr(any(), java_method(name = "indexOf", descriptor = "(Ljava/lang/String;I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn indexOf_str_i(&self, str: String, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/StringBuilder.indexOf:(Ljava/lang/String;I)I")
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/String;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn lastIndexOf_str(&self, str: String) -> Result<i32> {
        panic!("stub: java/lang/StringBuilder.lastIndexOf:(Ljava/lang/String;)I")
    }

    #[cfg_attr(any(), java_method(name = "lastIndexOf", descriptor = "(Ljava/lang/String;I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn lastIndexOf_str_i(&self, str: String, fromIndex: i32) -> Result<i32> {
        panic!("stub: java/lang/StringBuilder.lastIndexOf:(Ljava/lang/String;I)I")
    }

    #[cfg_attr(any(), java_method(name = "reverse", descriptor = "()Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn reverse(&self) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.reverse:()Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "repeat", descriptor = "(II)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn repeat_i_i(&self, codePoint: i32, count: i32) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.repeat:(II)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "repeat", descriptor = "(Ljava/lang/CharSequence;I)Ljava/lang/StringBuilder;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn repeat_seq_i(&self, cs: Object, count: i32) -> Result<StringBuilder> {
        panic!("stub: java/lang/StringBuilder.repeat:(Ljava/lang/CharSequence;I)Ljava/lang/StringBuilder;")
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        Ok(String::new_sb(Clone::clone(&this))?)
    }

    #[cfg_attr(any(), java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException"))]
    pub fn writeObject(&self, s: Object) -> Result<()> {
        panic!("stub: java/lang/StringBuilder.writeObject:(Ljava/io/ObjectOutputStream;)V")
    }

    #[cfg_attr(any(), java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException"))]
    pub fn readObject(&self, s: Object) -> Result<()> {
        panic!("stub: java/lang/StringBuilder.readObject:(Ljava/io/ObjectInputStream;)V")
    }
}
