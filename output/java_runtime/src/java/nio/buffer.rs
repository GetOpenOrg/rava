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
use crate::jdk::internal::misc::ScopedMemoryAccess;
use crate::jdk::internal::misc::Unsafe;
use crate::jdk::internal::util::Preconditions;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/nio/Buffer"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Buffer.java"]
    #[inner_classes     = "java/nio/Buffer$1:::0;java/nio/Buffer$2:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/nio/Buffer"]

    pub struct Buffer {
        #[cfg_attr(any(), java_field(name = "mark", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub mark: i32,
        #[cfg_attr(any(), java_field(name = "position", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub position: i32,
        #[cfg_attr(any(), java_field(name = "limit", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub limit: i32,
        #[cfg_attr(any(), java_field(name = "capacity", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub capacity: i32,
        #[cfg_attr(any(), java_field(name = "address", descriptor = "J", is_static = false))]
        pub address: i64,
        #[cfg_attr(any(), java_field(name = "segment", descriptor = "Ljava/lang/foreign/MemorySegment;", access = "package", modifiers = "final", is_static = false))]
        pub segment: Object,
    }

    impl Buffer {
        #[cfg_attr(any(), java_field(name = "UNSAFE", descriptor = "Ljdk/internal/misc/Unsafe;", access = "package", modifiers = "static final", is_static = true))]
        // static field: UNSAFE:Ljdk/internal/misc/Unsafe;
        pub fn UNSAFE() -> Unsafe {
            panic!("stub: java/nio/Buffer.UNSAFE:Ljdk/internal/misc/Unsafe;")
        }

        #[cfg_attr(any(), java_field(name = "SCOPED_MEMORY_ACCESS", descriptor = "Ljdk/internal/misc/ScopedMemoryAccess;", access = "package", modifiers = "static final", is_static = true))]
        // static field: SCOPED_MEMORY_ACCESS:Ljdk/internal/misc/ScopedMemoryAccess;
        pub fn SCOPED_MEMORY_ACCESS() -> ScopedMemoryAccess {
            panic!("stub: java/nio/Buffer.SCOPED_MEMORY_ACCESS:Ljdk/internal/misc/ScopedMemoryAccess;")
        }

        #[cfg_attr(any(), java_field(name = "SPLITERATOR_CHARACTERISTICS", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "16464"))]
        // static field: SPLITERATOR_CHARACTERISTICS:I
        pub fn SPLITERATOR_CHARACTERISTICS() -> i32 {
            16464
        }

        #[cfg_attr(any(), java_field(name = "IOOBE_FORMATTER", descriptor = "Ljava/util/function/BiFunction;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/function/BiFunction<Ljava/lang/String;Ljava/util/List<Ljava/lang/Number;>;Ljava/lang/IndexOutOfBoundsException;>;"))]
        // static field: IOOBE_FORMATTER:Ljava/util/function/BiFunction;
        pub fn IOOBE_FORMATTER() -> Object {
            panic!("stub: java/nio/Buffer.IOOBE_FORMATTER:Ljava/util/function/BiFunction;")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "(JILjava/lang/foreign/MemorySegment;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_l_i_memory(addr: i64, arg1: i32, cap: Object) -> Result<Self> {
            panic!("stub: java/nio/Buffer.<init>:(JILjava/lang/foreign/MemorySegment;)V")
        }

        #[java_method(name = "<init>", descriptor = "(IIIILjava/lang/foreign/MemorySegment;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(IIIILjava/lang/foreign/MemorySegment;)V
        pub fn new_i_i_i_i_memory(mut mark: i32, mut pos: i32, mut lim: i32, mut cap: i32, mut segment: Object) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_mark(-1i32);
            this.__set_position(0i32);
            if (cap<0) {
                let _t0: IllegalArgumentException = Buffer::createCapacityException(cap)?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.__set_capacity(cap);
            this.__set_segment(Clone::clone(&segment));
            let _t0 = this.limit_i(lim)?;
            let _t1 = this.position_i(pos)?;
            if mark > pos {
                let _t2 = StringBuilder::new()?.append_str(Clone::clone(&String::from("mark > position: (")))?;
                let _t3 = _t2.append_i(mark)?;
                let _t4 = _t3.append_str(Clone::clone(&String::from(" > ")))?;
                let _t5 = _t4.append_i(pos)?;
                let _t6 = _t5.append_str(Clone::clone(&String::from(")")))?;
                let _t7 = _t6.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.__set_mark(mark);
            Ok(this)
        }

        #[java_method(name = "createSameBufferException", descriptor = "()Ljava/lang/IllegalArgumentException;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createSameBufferException() -> Result<IllegalArgumentException> {
            panic!("stub: java/nio/Buffer.createSameBufferException:()Ljava/lang/IllegalArgumentException;")
        }

        #[java_method(name = "createCapacityException", descriptor = "(I)Ljava/lang/IllegalArgumentException;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createCapacityException(mut capacity: i32) -> Result<IllegalArgumentException> {
            if (capacity>=0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("capacity < 0: (")))?;
            let _t1 = _t0.append_i(capacity)?;
            let _t2 = _t1.append_str(Clone::clone(&String::from(" < 0)")))?;
            let _t3 = _t2.toString()?;
            Ok(IllegalArgumentException::new_str(Clone::clone(&_t3))?)
        }

        #[java_method(name = "capacity", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn capacity(&self) -> Result<i32> {
            panic!("stub: java/nio/Buffer.capacity:()I")
        }

        #[java_method(name = "position", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn position(&self) -> Result<i32> {
            panic!("stub: java/nio/Buffer.position:()I")
        }

        #[java_method(name = "position", descriptor = "(I)Ljava/nio/Buffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: position(I)Ljava/nio/Buffer;
        pub fn position_i(&self, mut newPosition: i32) -> Result<Buffer> {
            let this = self;
            if (((newPosition > this.__get_limit() as i32)|((newPosition<0) as i32))!=0) {
                let _t0 = this.createPositionException(newPosition)?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if this.__get_mark() > newPosition {
                this.__set_mark(-1i32);
            }
            this.__set_position(newPosition);
            Ok(Clone::clone(this))
        }

        #[java_method(name = "createPositionException", descriptor = "(I)Ljava/lang/IllegalArgumentException;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createPositionException(&self, mut newPosition: i32) -> Result<IllegalArgumentException> {
            let this = self;
            let mut msg: Object = Object::default();
            if newPosition > this.__get_limit() {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("newPosition > limit: (")))?;
                let _t1 = _t0.append_i(newPosition)?;
                let _t2 = _t1.append_str(Clone::clone(&String::from(" > ")))?;
                let _t3 = _t2.append_i(this.__get_limit())?;
                let _t4 = _t3.append_str(Clone::clone(&String::from(")")))?;
                let _t5 = _t4.toString()?;
                let mut msg: String = _t5;
            } else {
                if (newPosition>=0) {
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("newPosition < 0: (")))?;
                let _t1 = _t0.append_i(newPosition)?;
                let _t2 = _t1.append_str(Clone::clone(&String::from(" < 0)")))?;
                let _t3 = _t2.toString()?;
                let mut msg: String = _t3;
            }
            Ok(IllegalArgumentException::new_str(Clone::clone(&msg))?)
        }

        #[java_method(name = "limit", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn limit(&self) -> Result<i32> {
            panic!("stub: java/nio/Buffer.limit:()I")
        }

        #[java_method(name = "limit", descriptor = "(I)Ljava/nio/Buffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: limit(I)Ljava/nio/Buffer;
        pub fn limit_i(&self, mut newLimit: i32) -> Result<Buffer> {
            let this = self;
            if (((newLimit > this.__get_capacity() as i32)|((newLimit<0) as i32))!=0) {
                let _t0 = this.createLimitException(newLimit)?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.__set_limit(newLimit);
            if this.__get_position() > newLimit {
                this.__set_position(newLimit);
            }
            if this.__get_mark() > newLimit {
                this.__set_mark(-1i32);
            }
            Ok(Clone::clone(this))
        }

        #[java_method(name = "createLimitException", descriptor = "(I)Ljava/lang/IllegalArgumentException;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createLimitException(&self, mut newLimit: i32) -> Result<IllegalArgumentException> {
            let this = self;
            let mut msg: Object = Object::default();
            if newLimit > this.__get_capacity() {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("newLimit > capacity: (")))?;
                let _t1 = _t0.append_i(newLimit)?;
                let _t2 = _t1.append_str(Clone::clone(&String::from(" > ")))?;
                let _t3 = _t2.append_i(this.__get_capacity())?;
                let _t4 = _t3.append_str(Clone::clone(&String::from(")")))?;
                let _t5 = _t4.toString()?;
                let mut msg: String = _t5;
            } else {
                if (newLimit>=0) {
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("newLimit < 0: (")))?;
                let _t1 = _t0.append_i(newLimit)?;
                let _t2 = _t1.append_str(Clone::clone(&String::from(" < 0)")))?;
                let _t3 = _t2.toString()?;
                let mut msg: String = _t3;
            }
            Ok(IllegalArgumentException::new_str(Clone::clone(&msg))?)
        }

        #[java_method(name = "mark", descriptor = "()Ljava/nio/Buffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn mark(&self) -> Result<Buffer> {
            panic!("stub: java/nio/Buffer.mark:()Ljava/nio/Buffer;")
        }

        #[java_method(name = "reset", descriptor = "()Ljava/nio/Buffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reset(&self) -> Result<Buffer> {
            panic!("stub: java/nio/Buffer.reset:()Ljava/nio/Buffer;")
        }

        #[java_method(name = "clear", descriptor = "()Ljava/nio/Buffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<Buffer> {
            panic!("stub: java/nio/Buffer.clear:()Ljava/nio/Buffer;")
        }

        #[java_method(name = "flip", descriptor = "()Ljava/nio/Buffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn flip(&self) -> Result<Buffer> {
            panic!("stub: java/nio/Buffer.flip:()Ljava/nio/Buffer;")
        }

        #[java_method(name = "rewind", descriptor = "()Ljava/nio/Buffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn rewind(&self) -> Result<Buffer> {
            panic!("stub: java/nio/Buffer.rewind:()Ljava/nio/Buffer;")
        }

        #[java_method(name = "remaining", descriptor = "()I", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn remaining(&self) -> Result<i32> {
            panic!("stub: java/nio/Buffer.remaining:()I")
        }

        #[java_method(name = "hasRemaining", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasRemaining(&self) -> Result<bool> {
            panic!("stub: java/nio/Buffer.hasRemaining:()Z")
        }

        #[java_method(name = "isReadOnly", descriptor = "()Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isReadOnly(&self) -> Result<bool> {
            panic!("stub: java/nio/Buffer.isReadOnly:()Z")
        }

        #[java_method(name = "hasArray", descriptor = "()Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn hasArray(&self) -> Result<bool> {
            panic!("stub: java/nio/Buffer.hasArray:()Z")
        }

        #[java_method(name = "array", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn array(&self) -> Result<Object> {
            panic!("stub: java/nio/Buffer.array:()Ljava/lang/Object;")
        }

        #[java_method(name = "arrayOffset", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn arrayOffset(&self) -> Result<i32> {
            panic!("stub: java/nio/Buffer.arrayOffset:()I")
        }

        #[java_method(name = "isDirect", descriptor = "()Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isDirect(&self) -> Result<bool> {
            panic!("stub: java/nio/Buffer.isDirect:()Z")
        }

        #[java_method(name = "slice", descriptor = "()Ljava/nio/Buffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn slice(&self) -> Result<Buffer> {
            panic!("stub: java/nio/Buffer.slice:()Ljava/nio/Buffer;")
        }

        #[java_method(name = "slice", descriptor = "(II)Ljava/nio/Buffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn slice_i_i(&self, arg0: i32, arg1: i32) -> Result<Buffer> {
            panic!("stub: java/nio/Buffer.slice:(II)Ljava/nio/Buffer;")
        }

        #[java_method(name = "duplicate", descriptor = "()Ljava/nio/Buffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn duplicate(&self) -> Result<Buffer> {
            panic!("stub: java/nio/Buffer.duplicate:()Ljava/nio/Buffer;")
        }

        #[java_method(name = "base", descriptor = "()Ljava/lang/Object;", access = "package", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn base(&self) -> Result<Object> {
            panic!("stub: java/nio/Buffer.base:()Ljava/lang/Object;")
        }

        #[java_method(name = "nextGetIndex", descriptor = "()I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextGetIndex(&self) -> Result<i32> {
            panic!("stub: java/nio/Buffer.nextGetIndex:()I")
        }

        #[java_method(name = "nextGetIndex", descriptor = "(I)I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextGetIndex_i(&self, nb: i32) -> Result<i32> {
            panic!("stub: java/nio/Buffer.nextGetIndex:(I)I")
        }

        #[java_method(name = "nextPutIndex", descriptor = "()I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextPutIndex(&self) -> Result<i32> {
            panic!("stub: java/nio/Buffer.nextPutIndex:()I")
        }

        #[java_method(name = "nextPutIndex", descriptor = "(I)I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextPutIndex_i(&self, nb: i32) -> Result<i32> {
            panic!("stub: java/nio/Buffer.nextPutIndex:(I)I")
        }

        #[java_method(name = "checkIndex", descriptor = "(I)I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkIndex_i(&self, i: i32) -> Result<i32> {
            panic!("stub: java/nio/Buffer.checkIndex:(I)I")
        }

        #[java_method(name = "checkIndex", descriptor = "(II)I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkIndex_i_i(&self, i: i32, nb: i32) -> Result<i32> {
            panic!("stub: java/nio/Buffer.checkIndex:(II)I")
        }

        #[java_method(name = "markValue", descriptor = "()I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn markValue(&self) -> Result<i32> {
            panic!("stub: java/nio/Buffer.markValue:()I")
        }

        #[java_method(name = "discardMark", descriptor = "()V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn discardMark(&self) -> Result<()> {
            panic!("stub: java/nio/Buffer.discardMark:()V")
        }

        #[java_method(name = "session", descriptor = "()Ljdk/internal/foreign/MemorySessionImpl;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn session(&self) -> Result<Object> {
            panic!("stub: java/nio/Buffer.session:()Ljdk/internal/foreign/MemorySessionImpl;")
        }

        #[java_method(name = "checkSession", descriptor = "()V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkSession(&self) -> Result<()> {
            panic!("stub: java/nio/Buffer.checkSession:()V")
        }
    }
}
