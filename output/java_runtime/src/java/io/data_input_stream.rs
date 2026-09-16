#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::math::*;
use crate::java::nio::*;
use crate::java::nio::charset::*;
use crate::java::security::*;
use crate::java::text::*;
use crate::java::text::spi::*;
use crate::java::time::*;
use crate::java::time::chrono::*;
use crate::java::time::temporal::*;
use crate::java::time::zone::*;
use crate::java::util::*;
use crate::java::util::concurrent::*;
use crate::java::util::concurrent::atomic::*;
use crate::java::util::concurrent::locks::*;
use crate::java::util::function::*;
use crate::java::util::regex::*;
use crate::java::util::spi::*;
use crate::java::util::stream::*;
use crate::java::util::zip::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::reflect::misc::*;
use crate::sun::security::action::*;
use crate::sun::security::util::*;
use crate::sun::text::*;
use crate::sun::util::*;
use crate::sun::util::calendar::*;
use crate::sun::util::locale::*;
use crate::sun::util::locale::provider::*;
use crate::sun::util::spi::*;
use crate::java::text::Normalizer;
use crate::jdk::internal::util::ByteArray;

impl From<DataInputStream> for FilterInputStream {
    fn from(v: DataInputStream) -> FilterInputStream { v.__into_super() }
}

impl From<DataInputStream> for InputStream {
    fn from(v: DataInputStream) -> InputStream { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/io/DataInputStream"]
    #[super_class       = "java/io/FilterInputStream"]
    #[interfaces        = "java/io/DataInput"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "DataInputStream.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "FilterInputStream"]
    #[superclass_fields(in_: InputStream)]
    #[all_supertypes    = "java/io/Closeable;java/io/DataInput;java/io/DataInputStream;java/io/FilterInputStream;java/io/InputStream;java/lang/Object"]

    pub struct DataInputStream {
        #[cfg_attr(any(), java_field(name = "readBuffer", descriptor = "[B", access = "private", modifiers = "final", is_static = false))]
        pub readBuffer: Rc<RefCell<Vec<i8>>>,
        #[cfg_attr(any(), java_field(name = "bytearr", descriptor = "[B", access = "private", modifiers = "", is_static = false))]
        pub bytearr: Rc<RefCell<Vec<i8>>>,
        #[cfg_attr(any(), java_field(name = "chararr", descriptor = "[C", access = "private", modifiers = "", is_static = false))]
        pub chararr: Rc<RefCell<Vec<u16>>>,
        #[cfg_attr(any(), java_field(name = "lineBuffer", descriptor = "[C", access = "private", modifiers = "", is_static = false))]
        pub lineBuffer: Rc<RefCell<Vec<u16>>>,
    }

    impl DataInputStream {
        #[cfg_attr(any(), java_field(name = "EMPTY_BYTE_ARRAY", descriptor = "[B", access = "private", modifiers = "static final", is_static = true))]
        // static field: EMPTY_BYTE_ARRAY:[B
        pub fn EMPTY_BYTE_ARRAY() -> Rc<RefCell<Vec<i8>>> {
            panic!("stub: java/io/DataInputStream.EMPTY_BYTE_ARRAY:[B")
        }

        #[cfg_attr(any(), java_field(name = "EMPTY_CHAR_ARRAY", descriptor = "[C", access = "private", modifiers = "static final", is_static = true))]
        // static field: EMPTY_CHAR_ARRAY:[C
        pub fn EMPTY_CHAR_ARRAY() -> Rc<RefCell<Vec<u16>>> {
            panic!("stub: java/io/DataInputStream.EMPTY_CHAR_ARRAY:[C")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/InputStream;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut in_: InputStream) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(FilterInputStream::new(Clone::clone(&in_))?);
            let mut _arr0: Rc<RefCell<Vec<i8>>> = Rc::new(RefCell::new(vec![0i8; 8i32 as usize]));
            this.__set_readBuffer(Clone::clone(&_arr0));
            this.__set_bytearr(Clone::clone(&DataInputStream::EMPTY_BYTE_ARRAY()));
            this.__set_chararr(Clone::clone(&DataInputStream::EMPTY_CHAR_ARRAY()));
            Ok(this)
        }

        #[java_method(name = "read", descriptor = "([B)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn read_arr_b(&self, b: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/io/DataInputStream.read:([B)I")
        }

        #[java_method(name = "read", descriptor = "([BII)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn read_arr_b_i_i(&self, b: Rc<RefCell<Vec<i8>>>, off: i32, len: i32) -> Result<i32> {
            panic!("stub: java/io/DataInputStream.read:([BII)I")
        }

        #[java_method(name = "readFully", descriptor = "([B)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: readFully([B)V
        pub fn readFully_arr_b(&self, mut b: Rc<RefCell<Vec<i8>>>) -> Result<()> {
            let this = self;
            this.readFully_arr_b_i_i(Clone::clone(&b), 0i32, (b.borrow().len() as i32))?;
            Ok(())
        }

        #[java_method(name = "readFully", descriptor = "([BII)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: readFully([BII)V
        pub fn readFully_arr_b_i_i(&self, mut b: Rc<RefCell<Vec<i8>>>, mut off: i32, mut len: i32) -> Result<()> {
            let this = self;
            let _t0: i32 = Objects::checkFromIndexSize_i_i_i(off, len, (b.borrow().len() as i32))?;
            let mut n: i32 = 0i32;
            loop {
                if n >= len { break; }
                let _t1 = this.__get_in_().read_arr_b_i_i(Clone::clone(&b), (off).wrapping_add(n), (len).wrapping_sub(n))?;
                let mut count: i32 = _t1;
                if (count<0) {
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                n = (n).wrapping_add(count);
            }
            Ok(())
        }

        #[java_method(name = "skipBytes", descriptor = "(I)I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn skipBytes(&self, n: i32) -> Result<i32> {
            panic!("stub: java/io/DataInputStream.skipBytes:(I)I")
        }

        #[java_method(name = "readBoolean", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readBoolean(&self) -> Result<bool> {
            panic!("stub: java/io/DataInputStream.readBoolean:()Z")
        }

        #[java_method(name = "readByte", descriptor = "()B", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readByte(&self) -> Result<i8> {
            panic!("stub: java/io/DataInputStream.readByte:()B")
        }

        #[java_method(name = "readUnsignedByte", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readUnsignedByte(&self) -> Result<i32> {
            panic!("stub: java/io/DataInputStream.readUnsignedByte:()I")
        }

        #[java_method(name = "readShort", descriptor = "()S", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readShort(&self) -> Result<i16> {
            panic!("stub: java/io/DataInputStream.readShort:()S")
        }

        #[java_method(name = "readUnsignedShort", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readUnsignedShort(&self) -> Result<i32> {
            panic!("stub: java/io/DataInputStream.readUnsignedShort:()I")
        }

        #[java_method(name = "readChar", descriptor = "()C", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readChar(&self) -> Result<u16> {
            panic!("stub: java/io/DataInputStream.readChar:()C")
        }

        #[java_method(name = "readInt", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readInt(&self) -> Result<i32> {
            let this = self;
            this.readFully_arr_b_i_i(Clone::clone(&this.__get_readBuffer()), 0i32, 4i32)?;
            let _t0: i32 = ByteArray::getInt(Clone::clone(&this.__get_readBuffer()), 0i32)?;
            Ok(_t0)
        }

        #[java_method(name = "readLong", descriptor = "()J", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readLong(&self) -> Result<i64> {
            panic!("stub: java/io/DataInputStream.readLong:()J")
        }

        #[java_method(name = "readFloat", descriptor = "()F", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readFloat(&self) -> Result<f32> {
            panic!("stub: java/io/DataInputStream.readFloat:()F")
        }

        #[java_method(name = "readDouble", descriptor = "()D", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readDouble(&self) -> Result<f64> {
            panic!("stub: java/io/DataInputStream.readDouble:()D")
        }

        #[java_method(name = "readLine", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException", is_deprecated = true)]
        pub fn readLine(&self) -> Result<String> {
            panic!("stub: java/io/DataInputStream.readLine:()Ljava/lang/String;")
        }

        #[java_method(name = "readUTF", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readUTF(&self) -> Result<String> {
            panic!("stub: java/io/DataInputStream.readUTF:()Ljava/lang/String;")
        }

        #[java_method(name = "readUTF", descriptor = "(Ljava/io/DataInput;)Ljava/lang/String;", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readUTF_datain(in_: Object) -> Result<String> {
            panic!("stub: java/io/DataInputStream.readUTF:(Ljava/io/DataInput;)Ljava/lang/String;")
        }
    }
}
