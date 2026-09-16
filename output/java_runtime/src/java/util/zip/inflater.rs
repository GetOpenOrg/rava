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
use crate::jdk::internal::util::Preconditions;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/zip/Inflater"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Inflater.java"]
    #[inner_classes     = "java/util/zip/Inflater$InflaterZStreamRef:java/util/zip/Inflater:InflaterZStreamRef:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/zip/Inflater"]

    pub struct Inflater {
        #[cfg_attr(any(), java_field(name = "zsRef", descriptor = "Ljava/util/zip/Inflater$InflaterZStreamRef;", access = "private", modifiers = "final", is_static = false))]
        pub zsRef: Inflater_InflaterZStreamRef,
        #[cfg_attr(any(), java_field(name = "input", descriptor = "Ljava/nio/ByteBuffer;", access = "private", modifiers = "", is_static = false))]
        pub input: ByteBuffer,
        #[cfg_attr(any(), java_field(name = "inputArray", descriptor = "[B", access = "private", modifiers = "", is_static = false))]
        pub inputArray: Rc<RefCell<Vec<i8>>>,
        #[cfg_attr(any(), java_field(name = "inputPos", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub inputPos: i32,
        #[cfg_attr(any(), java_field(name = "inputLim", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub inputLim: i32,
        #[cfg_attr(any(), java_field(name = "finished", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub finished: bool,
        #[cfg_attr(any(), java_field(name = "pendingOutput", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub pendingOutput: bool,
        #[cfg_attr(any(), java_field(name = "needDict", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub needDict: bool,
        #[cfg_attr(any(), java_field(name = "bytesRead", descriptor = "J", access = "private", modifiers = "", is_static = false))]
        pub bytesRead: i64,
        #[cfg_attr(any(), java_field(name = "bytesWritten", descriptor = "J", access = "private", modifiers = "", is_static = false))]
        pub bytesWritten: i64,
        #[cfg_attr(any(), java_field(name = "inputConsumed", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub inputConsumed: i32,
        #[cfg_attr(any(), java_field(name = "outputConsumed", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub outputConsumed: i32,
    }

    impl Inflater {
        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "(Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Z)V
        pub fn new_z(mut nowrap: bool) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_input(Clone::clone(&ZipUtils::defaultBuf()));
            let _t0: i64 = Inflater::init(nowrap)?;
            this.__set_zsRef(Inflater_InflaterZStreamRef::new(Clone::clone(this), _t0)?);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this = Inflater::new_z((0i32 != 0i32))?;
            Ok(this)
        }

        #[java_method(name = "setInput", descriptor = "([BII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setInput_arr_b_i_i(&self, input: Rc<RefCell<Vec<i8>>>, off: i32, len: i32) -> Result<()> {
            panic!("stub: java/util/zip/Inflater.setInput:([BII)V")
        }

        #[java_method(name = "setInput", descriptor = "([B)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setInput_arr_b(&self, input: Rc<RefCell<Vec<i8>>>) -> Result<()> {
            panic!("stub: java/util/zip/Inflater.setInput:([B)V")
        }

        #[java_method(name = "setInput", descriptor = "(Ljava/nio/ByteBuffer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setInput_bytebu(&self, input: ByteBuffer) -> Result<()> {
            panic!("stub: java/util/zip/Inflater.setInput:(Ljava/nio/ByteBuffer;)V")
        }

        #[java_method(name = "setDictionary", descriptor = "([BII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDictionary_arr_b_i_i(&self, dictionary: Rc<RefCell<Vec<i8>>>, off: i32, len: i32) -> Result<()> {
            panic!("stub: java/util/zip/Inflater.setDictionary:([BII)V")
        }

        #[java_method(name = "setDictionary", descriptor = "([B)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDictionary_arr_b(&self, dictionary: Rc<RefCell<Vec<i8>>>) -> Result<()> {
            panic!("stub: java/util/zip/Inflater.setDictionary:([B)V")
        }

        #[java_method(name = "setDictionary", descriptor = "(Ljava/nio/ByteBuffer;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDictionary_bytebu(&self, dictionary: ByteBuffer) -> Result<()> {
            panic!("stub: java/util/zip/Inflater.setDictionary:(Ljava/nio/ByteBuffer;)V")
        }

        #[java_method(name = "getRemaining", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getRemaining(&self) -> Result<i32> {
            panic!("stub: java/util/zip/Inflater.getRemaining:()I")
        }

        #[java_method(name = "needsInput", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn needsInput(&self) -> Result<bool> {
            panic!("stub: java/util/zip/Inflater.needsInput:()Z")
        }

        #[java_method(name = "needsDictionary", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn needsDictionary(&self) -> Result<bool> {
            panic!("stub: java/util/zip/Inflater.needsDictionary:()Z")
        }

        #[java_method(name = "finished", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn finished(&self) -> Result<bool> {
            panic!("stub: java/util/zip/Inflater.finished:()Z")
        }

        #[java_method(name = "inflate", descriptor = "([BII)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/util/zip/DataFormatException")]
        pub fn inflate_arr_b_i_i(&self, output: Rc<RefCell<Vec<i8>>>, off: i32, len: i32) -> Result<i32> {
            panic!("stub: java/util/zip/Inflater.inflate:([BII)I")
        }

        #[java_method(name = "inflate", descriptor = "([B)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/util/zip/DataFormatException")]
        pub fn inflate_arr_b(&self, output: Rc<RefCell<Vec<i8>>>) -> Result<i32> {
            panic!("stub: java/util/zip/Inflater.inflate:([B)I")
        }

        #[java_method(name = "inflate", descriptor = "(Ljava/nio/ByteBuffer;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/util/zip/DataFormatException")]
        pub fn inflate_bytebu(&self, output: ByteBuffer) -> Result<i32> {
            panic!("stub: java/util/zip/Inflater.inflate:(Ljava/nio/ByteBuffer;)I")
        }

        #[java_method(name = "getAdler", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAdler(&self) -> Result<i32> {
            panic!("stub: java/util/zip/Inflater.getAdler:()I")
        }

        #[java_method(name = "getTotalIn", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTotalIn(&self) -> Result<i32> {
            panic!("stub: java/util/zip/Inflater.getTotalIn:()I")
        }

        #[java_method(name = "getBytesRead", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBytesRead(&self) -> Result<i64> {
            panic!("stub: java/util/zip/Inflater.getBytesRead:()J")
        }

        #[java_method(name = "getTotalOut", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTotalOut(&self) -> Result<i32> {
            panic!("stub: java/util/zip/Inflater.getTotalOut:()I")
        }

        #[java_method(name = "getBytesWritten", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBytesWritten(&self) -> Result<i64> {
            panic!("stub: java/util/zip/Inflater.getBytesWritten:()J")
        }

        #[java_method(name = "reset", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reset(&self) -> Result<()> {
            panic!("stub: java/util/zip/Inflater.reset:()V")
        }

        #[java_method(name = "end", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn end(&self) -> Result<()> {
            panic!("stub: java/util/zip/Inflater.end:()V")
        }

        #[java_method(name = "ensureOpen", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ensureOpen(&self) -> Result<()> {
            panic!("stub: java/util/zip/Inflater.ensureOpen:()V")
        }

        #[java_method(name = "hasPendingOutput", descriptor = "()Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasPendingOutput(&self) -> Result<bool> {
            panic!("stub: java/util/zip/Inflater.hasPendingOutput:()Z")
        }

        #[native]
        #[java_native(name = "initIDs", descriptor = "()V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn initIDs() -> Result<()> {
            panic!("native: java/util/zip/Inflater.initIDs:()V")
        }

        #[native]
        #[java_native(name = "init", descriptor = "(Z)J", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn init(arg0: bool) -> Result<i64> {
            panic!("native: java/util/zip/Inflater.init:(Z)J")
        }

        #[native]
        #[java_native(name = "setDictionary", descriptor = "(J[BII)V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn setDictionary_l_arr_b_i_i(arg0: i64, arg1: Rc<RefCell<Vec<i8>>>, arg2: i32, arg3: i32) -> Result<()> {
            panic!("native: java/util/zip/Inflater.setDictionary:(J[BII)V")
        }

        #[native]
        #[java_native(name = "setDictionaryBuffer", descriptor = "(JJI)V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn setDictionaryBuffer(arg0: i64, arg1: i64, arg2: i32) -> Result<()> {
            panic!("native: java/util/zip/Inflater.setDictionaryBuffer:(JJI)V")
        }

        #[native]
        #[java_native(name = "inflateBytesBytes", descriptor = "(J[BII[BII)J", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/util/zip/DataFormatException")]
        pub fn inflateBytesBytes(&self, arg0: i64, arg1: Rc<RefCell<Vec<i8>>>, arg2: i32, arg3: i32, arg4: Rc<RefCell<Vec<i8>>>, arg5: i32, arg6: i32) -> Result<i64> {
            panic!("native: java/util/zip/Inflater.inflateBytesBytes:(J[BII[BII)J")
        }

        #[native]
        #[java_native(name = "inflateBytesBuffer", descriptor = "(J[BIIJI)J", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/util/zip/DataFormatException")]
        pub fn inflateBytesBuffer(&self, arg0: i64, arg1: Rc<RefCell<Vec<i8>>>, arg2: i32, arg3: i32, arg4: i64, arg5: i32) -> Result<i64> {
            panic!("native: java/util/zip/Inflater.inflateBytesBuffer:(J[BIIJI)J")
        }

        #[native]
        #[java_native(name = "inflateBufferBytes", descriptor = "(JJI[BII)J", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/util/zip/DataFormatException")]
        pub fn inflateBufferBytes(&self, arg0: i64, arg1: i64, arg2: i32, arg3: Rc<RefCell<Vec<i8>>>, arg4: i32, arg5: i32) -> Result<i64> {
            panic!("native: java/util/zip/Inflater.inflateBufferBytes:(JJI[BII)J")
        }

        #[native]
        #[java_native(name = "inflateBufferBuffer", descriptor = "(JJIJI)J", access = "private", modifiers = "native", is_static    = false, is_native    = true, is_abstract  = false, is_synthetic = false, exceptions = "java/util/zip/DataFormatException")]
        pub fn inflateBufferBuffer(&self, arg0: i64, arg1: i64, arg2: i32, arg3: i64, arg4: i32) -> Result<i64> {
            panic!("native: java/util/zip/Inflater.inflateBufferBuffer:(JJIJI)J")
        }

        #[native]
        #[java_native(name = "getAdler", descriptor = "(J)I", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn getAdler_l(arg0: i64) -> Result<i32> {
            panic!("native: java/util/zip/Inflater.getAdler:(J)I")
        }

        #[native]
        #[java_native(name = "reset", descriptor = "(J)V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn reset_l(arg0: i64) -> Result<()> {
            panic!("native: java/util/zip/Inflater.reset:(J)V")
        }

        #[native]
        #[java_native(name = "end", descriptor = "(J)V", access = "private", modifiers = "static native", is_static    = true, is_native    = true, is_abstract  = false, is_synthetic = false)]
        pub fn end_l(arg0: i64) -> Result<()> {
            panic!("native: java/util/zip/Inflater.end:(J)V")
        }
    }
}
